// The cloud deck, as a stack of slices through the troposphere.
//
// Weather is a depth of air, not a picture stuck on a sphere, and the argument
// here is the one `solar_aurora.wgsl` makes one atmosphere higher up: hand the
// shader concentric spheres at real altitudes and let each contribute its own
// slice. What is different is that cloud *occludes*. The aurora emits zero
// alpha and only ever adds light; a deck hides the coastline under it, so these
// slices composite, and the CPU hands them over bottom-up so the blend runs
// back to front.
//
// Nothing about the vertical structure is invented. `cloud_tex` carries, per
// column, how thick the cloud is and how *high its top stands* — the second
// straight out of the infrared mosaic, because a cloud top's temperature is its
// altitude. So a thunderhead towers over the stratus beside it for the same
// reason it does in the sky, and a shell only contributes where it is inside
// the cloud that column actually has.
//
// The lightning is the one invention, and it is confined to the timing. Where
// the storms are, how large, how tall and how often each flashes all come from
// the same mosaic; which millisecond a given stroke fires does not, because no
// free worldwide feed of real strikes exists. The flashes light the cloud from
// inside rather than being drawn as marks on it, which is why an anvil goes
// bright from below.

struct Globals {
    view_proj: mat4x4<f32>,
    camera_pos: vec4<f32>,   // xyz eye, w near
    sun_pos: vec4<f32>,      // xyz centre, w rendered radius
    sun_to_earth: vec4<f32>,
    solar_north: vec4<f32>,
    viewport: vec4<f32>,
    misc: vec4<f32>,         // x seconds, y photo blend, zw spare
};

struct DrawData {
    model: mat4x4<f32>,
    basis: mat4x4<f32>,
    tint: vec4<f32>,
    tint2: vec4<f32>,
    // x this shell's altitude (km), y the slab of air it stands for (km),
    // z intensity (screen-size fade), w unused on this path.
    params: vec4<f32>,
    // x vertical exaggeration, y deck floor (km), z deck ceiling (km),
    // w the Earth's rendered radius, in world units.
    style: vec4<f32>,
};

/// Up to eight flashes alight at once — see `MAX_FLASHES` in `scene.rs`.
struct Flashes {
    // xyz world position inside the tower, w brightness. Zero is an unused
    // slot, which contributes nothing, so the loop below never branches.
    items: array<vec4<f32>, 8>,
    // x how far the light reaches, in world units.
    reach: vec4<f32>,
};

@group(0) @binding(0) var<uniform> g: Globals;
@group(0) @binding(3) var samp: sampler;
@group(0) @binding(7) var cloud_tex: texture_2d<f32>;
@group(0) @binding(8) var<uniform> fl: Flashes;
@group(1) @binding(0) var<uniform> d: DrawData;

/// Cloud-top height at the top of the stored range, kilometres. Must match
/// `clouds::TOP_MAX_KM`.
const TOP_MAX_KM = 18.0;
const EARTH_R_KM = 6371.0;
const PI = 3.14159265;

/// True radii, gigametres — `ephem::{SUN_R, EARTH_R, MOON_R}`.
const TRUE_SUN_R = 0.6957;
const TRUE_EARTH_R = 0.006371;
const TRUE_MOON_R = 0.0017374;

/// The Moon's shadow — the same TRUE-scale eclipse test the ground runs (see
/// `solar_body.wgsl` for the full argument): 1 in the open, 0 in the umbra.
/// The deck has to darken with the surface under it or the shadow would read
/// as a hole in the weather rather than as a shadow across it. `moon_off` is
/// the Moon's true geocentric offset, carried in `DrawData::tint2`.
fn eclipse_light(centre: vec3<f32>, moon_off: vec3<f32>, n: vec3<f32>) -> f32 {
    if (dot(moon_off, moon_off) < 1e-6) {
        return 1.0;
    }
    let p = n * TRUE_EARTH_R;
    let to_sun = (g.sun_pos.xyz - centre) - p;
    let to_moon = moon_off - p;
    let su = normalize(to_sun);
    let mu = normalize(to_moon);
    if (dot(su, mu) <= 0.0) {
        return 1.0;
    }
    let rs = TRUE_SUN_R / length(to_sun);
    let rm = TRUE_MOON_R / length(to_moon);
    let sep = length(cross(su, mu));
    let f = clamp((rs + rm - sep) / (2.0 * rs), 0.0, 1.0);
    return 1.0 - f * f * (3.0 - 2.0 * f);
}

/// A sunlit cloud top reflects about seven tenths of what falls on it, against
/// the ocean's six hundredths. If the deck is not markedly brighter than the
/// sea it reads as smoke rather than as cloud.
const ALBEDO = vec3<f32>(0.93, 0.95, 0.99);
/// A flash is a spark gap, so its light is blue-white.
const FLASH_TINT = vec3<f32>(0.80, 0.87, 1.00);
/// What the night side keeps. Not zero: an unlit deck still has to occlude the
/// coastline glow under it, or the land shows straight through the weather.
const NIGHT_FLOOR = 0.035;
/// How much of the sunlight the lit face returns towards the eye. The deck is
/// lit purely diffusely — no view-dependent term — so a column is exactly as
/// bright from over the subsolar point as it is from the side, and the middle of
/// the daylit disc no longer blows out into a hotspot as the globe is turned.
///
/// A little above the flat-lit value it would take on its own: the relief terms
/// below only ever remove light on balance, so the deck would drift darker
/// overall as they were added. This holds the *mean* where it was and spends the
/// difference on contrast.
const DAY_GAIN = 0.82;
/// How hard the deck's own relief is allowed to shade it — see `top_slope`.
///
/// Asymmetric, and that is the physical way round rather than a taste knob. A
/// tilted cloud top that turns to face the Sun gathers a little more light than
/// a flat one; a tilted top that turns *away* falls into its own shadow, and
/// there is no floor on how dark that gets short of the skylight bouncing around
/// it. So relief mostly carves shadow, and only slightly adds highlight.
const RELIEF_LIGHT = 0.32;
const RELIEF_DARK = 0.62;
/// The drop below its neighbours, in exaggerated kilometres, at which a column
/// counts as a fully enclosed hollow — and how much light such a hollow loses.
/// Cut the depth to zero and the deck goes back to reading as a painted sheet
/// wherever the Sun is high.
const CAVITY_KM = 4.0;
const CAVITY_DEPTH = 0.38;

fn srgb_to_linear(c: vec3<f32>) -> vec3<f32> {
    let lo = c / 12.92;
    let hi = pow((c + vec3(0.055)) / 1.055, vec3(2.4));
    return select(hi, lo, c <= vec3(0.04045));
}

fn hash3(p: vec3<f32>) -> f32 {
    return fract(sin(dot(p, vec3(12.9898, 78.233, 37.719))) * 43758.5453);
}

fn vnoise(p: vec3<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);
    let c000 = hash3(i + vec3(0.0, 0.0, 0.0));
    let c100 = hash3(i + vec3(1.0, 0.0, 0.0));
    let c010 = hash3(i + vec3(0.0, 1.0, 0.0));
    let c110 = hash3(i + vec3(1.0, 1.0, 0.0));
    let c001 = hash3(i + vec3(0.0, 0.0, 1.0));
    let c101 = hash3(i + vec3(1.0, 0.0, 1.0));
    let c011 = hash3(i + vec3(0.0, 1.0, 1.0));
    let c111 = hash3(i + vec3(1.0, 1.0, 1.0));
    let x00 = mix(c000, c100, u.x);
    let x10 = mix(c010, c110, u.x);
    let x01 = mix(c001, c101, u.x);
    let x11 = mix(c011, c111, u.x);
    return mix(mix(x00, x10, u.y), mix(x01, x11, u.y), u.z);
}

/// Billows. Sampled from the direction *and* the altitude, so two shells agree
/// about the same lump of cloud and the stack reads as one body of air instead
/// of as a pile of spheres. The 1° mosaic cannot resolve an individual cell, so
/// this is the texture between its pixels — multiplicative and centred on one,
/// so it can shape what the picture says but never put cloud in a clear sky.
fn billows(n: vec3<f32>, alt: f32, t: f32) -> f32 {
    let drift = t * 0.004;
    let p = n * 190.0 + vec3(0.0, 0.0, alt * 0.55) + vec3(drift, drift * 0.3, 0.0);
    let coarse = vnoise(p * 0.35);
    let fine = vnoise(p * 1.4);
    // Same mean as an unweighted deck, wider spread: the pedestal is lower and
    // the swing above it larger, so the thin edges of a sheet break up into
    // ragged cloud instead of fading out as an even wash. Keeping the mean put
    // is what stops this from being a global opacity change in disguise.
    return clamp(0.26 + 1.28 * coarse + 0.46 * fine, 0.0, 2.0);
}

/// The shape of the cloud top over one column: which way it tilts, and whether
/// it stands proud of what surrounds it or sits down in a hollow.
///
/// The mosaic is a height field, and a height field has a shape. Every column at
/// the same opacity returns exactly the same light without this, which is why an
/// unshaded deck reads as a sheet of paint however much structure is in the data
/// — the structure is all in the *coverage* and none of it in the *light*. Four
/// taps of the neighbourhood buy both halves of the fix, and they are the same
/// four either way.
struct TopRelief {
    /// (east, south) tilt: kilometres of rise per kilometre of run, carrying the
    /// same vertical exaggeration the geometry was built with. The shells really
    /// are drawn `lift` times too tall, so shading them any flatter would light a
    /// shape that is not the one on screen.
    slope: vec2<f32>,
    /// How far this column sits below its neighbours' mean, in exaggerated
    /// kilometres. Positive in a hollow, negative on a peak. This is the part
    /// that does not care where the Sun is: a dell in the deck sees less of the
    /// sky than a dome does whatever the hour, so it is darker at noon too — and
    /// noon is exactly where the tilt term has nothing left to say, because a
    /// Sun straight overhead lights every slope alike.
    cavity: f32,
};

fn top_relief(uv: vec2<f32>, centre: f32, coslat: f32, lift: f32) -> TopRelief {
    let dims = vec2<f32>(textureDimensions(cloud_tex, 0));
    let du = 1.0 / dims.x;
    let dv = 1.0 / dims.y;
    // Central differences, at level zero: the mosaic has no mips, and a filtered
    // tap would flatten the very gradient this is here to find.
    let e = textureSampleLevel(cloud_tex, samp, vec2(uv.x + du, uv.y), 0.0).g;
    let w = textureSampleLevel(cloud_tex, samp, vec2(uv.x - du, uv.y), 0.0).g;
    let s = textureSampleLevel(cloud_tex, samp, vec2(uv.x, uv.y + dv), 0.0).g;
    let n = textureSampleLevel(cloud_tex, samp, vec2(uv.x, uv.y - dv), 0.0).g;
    // How far apart those taps are on the ground, kilometres. Meridians
    // converge, so a texel of longitude is shorter the further from the equator
    // it lies; the clamp is what stops the last rows before the pole turning a
    // one-texel step into a cliff. Nothing is drawn poleward of about 73° anyway
    // — no geostationary satellite looks there — so it never bites in practice.
    let run_e = 2.0 * EARTH_R_KM * (2.0 * PI * du) * max(coslat, 0.10);
    let run_s = 2.0 * EARTH_R_KM * (PI * dv);

    var r: TopRelief;
    r.slope = vec2((e - w) * TOP_MAX_KM / run_e, (s - n) * TOP_MAX_KM / run_s) * lift;
    r.cavity = ((e + w + s + n) * 0.25 - centre) * TOP_MAX_KM * lift;
    return r;
}

/// How deep the cloud in this column is, kilometres.
///
/// A thin cirrus shield is a sheet a kilometre thick with its top at eleven; a
/// storm reaches from near the ground to near the tropopause. Optical thickness
/// is what separates them, and it is the other thing the mosaic measured.
fn deck_depth(opacity: f32, top_km: f32) -> f32 {
    return clamp(1.0 + opacity * top_km * 0.85, 0.8, top_km);
}

/// Light from the storms, at a point in the deck.
///
/// Fixed cost and uniform control flow: an unused slot has zero brightness and
/// adds nothing, so there is nothing to branch on.
fn flash_light(world: vec3<f32>, convective: f32) -> f32 {
    var acc = 0.0;
    for (var i = 0u; i < 8u; i++) {
        let f = fl.items[i];
        let dist = length(world - f.xyz);
        acc += f.w * exp(-dist / max(fl.reach.x, 1e-6));
    }
    // Only cloud that is deep enough to be making the lightning lights up with
    // it. A flash that lit the cirrus half a continent away would be a lamp in
    // the sky rather than a storm.
    return acc * convective;
}

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) nrm: vec3<f32>,
    @location(2) world: vec3<f32>,
    /// The same direction in the Earth's own frame. The billows are sampled from
    /// this rather than from the world-space normal, so the fine structure turns
    /// with the planet instead of crawling across its surface all day.
    @location(3) body: vec3<f32>,
};

@vertex
fn vs(@location(0) pos: vec3<f32>, @location(1) uv: vec2<f32>) -> VsOut {
    var o: VsOut;
    let world = d.model * vec4(pos, 1.0);
    o.clip = g.view_proj * world;
    o.world = world.xyz;
    o.nrm = normalize((d.basis * vec4(pos, 0.0)).xyz);
    o.body = normalize(pos);
    o.uv = uv;
    return o;
}

@fragment
fn fs(in: VsOut) -> @location(0) vec4<f32> {
    // The grid is exactly the picture the mosaic was requested as, laid out on
    // the same equirectangular convention as the sphere's own UVs, so this is a
    // straight tap with no correction.
    let c = textureSample(cloud_tex, samp, in.uv);
    let opacity = c.r;
    let top_km = c.g * TOP_MAX_KM;

    // Most columns are clear or as good as, and leaving early there is what
    // makes a stack of eighteen full spheres affordable. It is also what draws
    // *nothing* poleward of about 73°, where no geostationary satellite looks:
    // blank is the honest answer there, and a clear sky would be a claim.
    if (opacity < 0.02) {
        discard;
    }

    let alt = d.params.x;
    let base_km = max(d.style.y, top_km - deck_depth(opacity, top_km));
    // This shell only contributes where it is inside the cloud this column
    // actually has. That single line is the whole trick: a two-dimensional
    // height field, read at a series of altitudes, is a volume.
    let inside = smoothstep(base_km - 0.5, base_km + 0.6, alt)
               * (1.0 - smoothstep(top_km - 0.7, top_km + 0.3, alt));
    if (inside < 0.004) {
        discard;
    }

    let n = normalize(in.nrm);
    let to_eye = normalize(g.camera_pos.xyz - in.world);
    let to_sun = normalize(g.sun_pos.xyz - in.world);

    // Density here, and from it how much of this slab is opaque. `slab` makes
    // it a Riemann sum, so the deck looks the same however many shells were
    // spent on it.
    // Path length through this slab: 1/cos of the incidence angle, exactly as
    // the aurora computes it. Looking along the deck instead of across it is
    // what turns the limb into a visible band of weather standing off the
    // surface — the whole reason for drawing this in the round.
    let grazing = min(1.0 / max(abs(dot(n, to_eye)), 0.20), 3.0);

    let dens = opacity * inside * billows(normalize(in.body), alt, g.misc.x);
    let a = clamp(1.0 - exp(-dens * d.params.y * grazing * 0.55), 0.0, 1.0) * d.params.z;
    if (a < 0.002) {
        discard;
    }

    // The same soft terminator the ground uses (`solar_body.wgsl`) — and the
    // same eclipse shadow. If the cloud's day/night line does not sit exactly
    // on the planet's, the deck reads as floating above it.
    let day = smoothstep(-0.06, 0.16, dot(n, to_sun))
        * eclipse_light(d.model[3].xyz, d.tint2.xyz, n);

    // Self-shadow: how much cloud stands between this sample and the top of its
    // own column. It is why a deck looks three-dimensional instead of like fog
    // — the underside of a tower is dark and its anvil is white.
    let above = max(top_km - alt, 0.0) / max(top_km - base_km, 0.1);
    let shade = mix(1.0, 0.22, clamp(above * opacity, 0.0, 1.0));

    // Relief, from the tilt of the cloud top over this column.
    //
    // Taken as the *difference* the slope makes against the smooth sphere under
    // it, not as a replacement for the terminator above. That is what lets the
    // deck be sculpted and still share the ground's day/night line exactly: the
    // large-scale distribution of light is untouched, and only the local tilt —
    // the part that was missing — is added.
    //
    // The tangent frame is the sphere mesh's own: +Z at the north pole and u
    // running east, so `east` is the horizontal perpendicular to the column and
    // `north` closes the triple. The slope tilts the normal away from uphill,
    // which is a height field's normal written out.
    let nb = normalize(in.body);
    let horiz = length(nb.xy);
    let east = select(vec3(0.0, 1.0, 0.0), vec3(-nb.y, nb.x, 0.0) / max(horiz, 1e-5), horiz > 1e-5);
    let north = cross(nb, east);
    let rel = top_relief(in.uv, c.g, horiz, d.style.x);
    let tilted = normalize(nb - east * rel.slope.x + north * rel.slope.y);
    let sun_b = normalize(vec3(dot(to_sun, d.basis[0].xyz), dot(to_sun, d.basis[1].xyz), dot(to_sun, d.basis[2].xyz)));
    // Thin cloud has no relief to speak of — it is a sheet, and carving shadow
    // into a cirrus veil would invent a structure the data never claimed.
    let relief = clamp((dot(tilted, sun_b) - dot(nb, sun_b)) * opacity, -RELIEF_DARK, RELIEF_LIGHT);
    // The hollows, over a few kilometres of exaggerated depth. Unlike the tilt
    // this survives a Sun overhead, which is what keeps the middle of the daylit
    // disc from going back to being a wash.
    let cavity = clamp(rel.cavity / CAVITY_KM, -0.5, 1.0) * opacity;

    var col = srgb_to_linear(ALBEDO)
        * (NIGHT_FLOOR + day * DAY_GAIN * shade * (1.0 + relief) * (1.0 - CAVITY_DEPTH * cavity));
    // Lightning, added as light rather than as more cloud: it brightens the
    // storm without making it thicker. `convective` is read off the height
    // field, so only the towers that are making the flashes light up with them.
    let convective = smoothstep(0.55, 0.80, c.g);
    col += srgb_to_linear(FLASH_TINT) * flash_light(in.world, convective) * 0.9;

    // Premultiplied: the colour is already scaled by the coverage it stands for.
    return vec4(col * a, a);
}
