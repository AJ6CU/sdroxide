// The cloud deck again, marched instead of sliced.
//
// `solar_cloud.wgsl` samples the troposphere at a set of fixed altitudes, which
// is cheap and gets the shape right. What it cannot get right is light *inside*
// the volume: a shell knows how much cloud is above it, but a flash three
// kilometres away in the same storm either lights the shell or does not, and
// there is nothing between them to glow. Here the ray is walked instead, so the
// light from a stroke is attenuated by every metre of cloud it crosses, and a
// thunderhead lights up from within and dims towards its edges the way one
// really does.
//
// One draw covers the whole deck: a single sphere at the top of the slab, with
// the entry and exit points found analytically per fragment. It costs several
// times what the stack does, which is why the stack is the default and this is
// a switch.
//
// Everything about *what* is drawn — the height field, the optical thickness,
// where the storms are — is shared with the other path and comes from the same
// infrared mosaic. Only the integration differs.

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
    // x top of the slab (km), y its depth (km), z screen-size fade,
    // w how many steps to spend.
    params: vec4<f32>,
    // x vertical exaggeration, y deck floor (km), z deck ceiling (km),
    // w the Earth's rendered radius, in world units.
    style: vec4<f32>,
};

struct Flashes {
    items: array<vec4<f32>, 8>,
    reach: vec4<f32>,
};

@group(0) @binding(0) var<uniform> g: Globals;
@group(0) @binding(3) var samp: sampler;
@group(0) @binding(7) var cloud_tex: texture_2d<f32>;
@group(0) @binding(8) var<uniform> fl: Flashes;
@group(1) @binding(0) var<uniform> d: DrawData;

const TOP_MAX_KM = 18.0;
const EARTH_R_KM = 6371.0;
const ALBEDO = vec3<f32>(0.93, 0.95, 0.99);
const FLASH_TINT = vec3<f32>(0.80, 0.87, 1.00);
const NIGHT_FLOOR = 0.035;
/// The lit face's diffuse return. Purely diffuse and matched to the sliced path
/// in `solar_cloud.wgsl`, so switching between the two does not change how bright
/// the weather is. Same reasoning for the value: the deeper shadow below only
/// takes light away, and this holds the mean up against it.
const DAY_GAIN = 0.82;
/// Hard ceiling on the loop. The step budget in `params.w` follows the globe's
/// size on screen; this is what stops a driver having to unroll the worst case.
const MAX_STEPS = 40;
/// Once this little light is still getting through, whatever is behind cannot
/// be seen and the rest of the march is wasted.
const MIN_TRANSMITTANCE = 0.012;

/// True radii, gigametres — `ephem::{SUN_R, EARTH_R, MOON_R}`.
const TRUE_SUN_R = 0.6957;
const TRUE_EARTH_R = 0.006371;
const TRUE_MOON_R = 0.0017374;

/// The Moon's shadow — the same TRUE-scale eclipse test the ground and the
/// sliced deck run (see `solar_body.wgsl` for the full argument): 1 in the
/// open, 0 in the umbra. `n` is the sample's radial direction and `moon_off`
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

fn billows(n: vec3<f32>, alt: f32, t: f32) -> f32 {
    let drift = t * 0.004;
    let p = n * 190.0 + vec3(0.0, 0.0, alt * 0.55) + vec3(drift, drift * 0.3, 0.0);
    return clamp(0.26 + 1.28 * vnoise(p * 0.35) + 0.46 * vnoise(p * 1.4), 0.0, 2.0);
}

fn deck_depth(opacity: f32, top_km: f32) -> f32 {
    return clamp(1.0 + opacity * top_km * 0.85, 0.8, top_km);
}

/// Where a ray meets a sphere of radius `r` about the origin, in ray parameter.
/// `x` is the near root and `y` the far one; `z` is zero when it misses.
fn sphere_hit(ro: vec3<f32>, rd: vec3<f32>, r: f32) -> vec3<f32> {
    let b = dot(ro, rd);
    let c = dot(ro, ro) - r * r;
    let disc = b * b - c;
    if (disc <= 0.0) {
        return vec3(0.0, 0.0, 0.0);
    }
    let s = sqrt(disc);
    return vec3(-b - s, -b + s, 1.0);
}

/// The cloud in the column under `p`, and how tall it stands.
///
/// Returns density at this point in x, and the column's convective fraction in
/// y — the second only so the lightning lights the storms that are making it.
fn sample_cloud(p: vec3<f32>, earth_r: f32, lift: f32, floor_km: f32, t: f32) -> vec2<f32> {
    let r = length(p);
    let world_n = p / r;
    // Into the Earth's own frame first. `basis` is rotation only, so its inverse
    // is its transpose and this is three dot products. Skipping it would leave
    // the weather fixed in space while the planet turned under it.
    let n = vec3(
        dot(world_n, d.basis[0].xyz),
        dot(world_n, d.basis[1].xyz),
        dot(world_n, d.basis[2].xyz),
    );
    // The sphere mesh's own convention: +X at (0°N, 0°E), +Z at the north pole,
    // u from 180° west eastward, v from the north pole down. Matching it is what
    // lets one texture serve the globe, the aurora and this.
    let uv = vec2(
        atan2(n.y, n.x) / (2.0 * 3.14159265) + 0.5,
        acos(clamp(n.z, -1.0, 1.0)) / 3.14159265,
    );
    let c = textureSampleLevel(cloud_tex, samp, uv, 0.0);
    let opacity = c.r;
    if (opacity < 0.02) {
        return vec2(0.0, 0.0);
    }
    // Back out the altitude this sample sits at, undoing the same exaggeration
    // the geometry was built with.
    let alt = (r / earth_r - 1.0) * EARTH_R_KM / max(lift, 1e-3);
    let top_km = c.g * TOP_MAX_KM;
    let base_km = max(floor_km, top_km - deck_depth(opacity, top_km));
    let inside = smoothstep(base_km - 0.5, base_km + 0.6, alt)
               * (1.0 - smoothstep(top_km - 0.7, top_km + 0.3, alt));
    return vec2(opacity * inside * billows(n, alt, t), smoothstep(0.55, 0.80, c.g));
}

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) world: vec3<f32>,
};

@vertex
fn vs(@location(0) pos: vec3<f32>, @location(1) uv: vec2<f32>) -> VsOut {
    var o: VsOut;
    let world = d.model * vec4(pos, 1.0);
    o.clip = g.view_proj * world;
    o.world = world.xyz;
    return o;
}

@fragment
fn fs(in: VsOut) -> @location(0) vec4<f32> {
    // The Earth's centre is the origin of everything below: the model matrix
    // put this sphere there, so its translation is the planet.
    let centre = d.model[3].xyz;
    let earth_r = d.style.w;
    let lift = d.style.x;
    let r_top = earth_r * (1.0 + d.style.z * lift / EARTH_R_KM);
    let r_base = earth_r * (1.0 + d.style.y * lift / EARTH_R_KM);

    let eye = g.camera_pos.xyz - centre;
    let rd = normalize(in.world - g.camera_pos.xyz);

    let outer = sphere_hit(eye, rd, r_top);
    if (outer.z == 0.0) {
        discard;
    }
    var t0 = max(outer.x, 0.0);
    var t1 = outer.y;

    // The solid planet. Its front surface is where the march has to stop, and
    // it is found here rather than read out of the depth buffer: the buffer
    // holds a reversed-Z globe and comparing against it by hand is fiddly
    // enough to get wrong quietly. Without this, the far wall of the slab shows
    // straight through the Earth.
    let solid = sphere_hit(eye, rd, earth_r);
    if (solid.z != 0.0 && solid.x > 0.0) {
        t1 = min(t1, solid.x);
    }
    // Under the deck's floor there is nothing but clear air, so a ray that
    // misses the planet can still skip the hole in the middle of the shell.
    let inner = sphere_hit(eye, rd, r_base);
    if (inner.z != 0.0 && inner.x > 0.0 && (solid.z == 0.0 || solid.x <= 0.0)) {
        t1 = min(t1, inner.x);
    }
    if (t1 <= t0) {
        discard;
    }

    // A constant step in world space, not "the span divided by N". A ray at the
    // limb crosses hundreds of kilometres of slab where one looking straight
    // down crosses eighteen, and dividing by N would sample the limb — the part
    // worth looking at — thirty times too coarsely.
    let steps = i32(clamp(d.params.w, 4.0, f32(MAX_STEPS)));
    let span = t1 - t0;
    let ideal = (r_top - r_base) / max(f32(steps), 1.0) * 1.6;
    let dt = max(ideal, span / f32(MAX_STEPS));
    // Break up the banding a fixed step leaves, with a dither that is stable
    // for a given direction so it does not crawl between frames.
    let jitter = hash3(rd * 977.0) * dt;

    let to_sun_dir = normalize(g.sun_pos.xyz - centre);
    var transmittance = 1.0;
    var lit = vec3(0.0, 0.0, 0.0);
    var t = t0 + jitter;

    for (var i = 0; i < MAX_STEPS; i++) {
        if (t >= t1 || transmittance < MIN_TRANSMITTANCE) {
            break;
        }
        let p = eye + rd * t;
        let s = sample_cloud(p, earth_r, lift, d.style.y, g.misc.x);
        if (s.x > 0.001) {
            let n = normalize(p);
            // Terminator and eclipse shadow, exactly as the sliced path takes
            // them, so switching decks never moves either line.
            let day = smoothstep(-0.06, 0.16, dot(n, to_sun_dir))
                * eclipse_light(centre, d.tint2.xyz, n);

            // Two shadow taps towards the Sun. A full secondary march would be
            // the textbook answer and many times the cost; two samples up-sun
            // capture what actually reads — the near one that a tower shades its
            // own flank, the far one that it throws shadow across the deck
            // beside it. One tap alone gave a single soft falloff, which is the
            // shading equivalent of a wash, and the deck came out flat.
            let slab = r_top - r_base;
            let near = sample_cloud(p + to_sun_dir * slab * 0.3, earth_r, lift, d.style.y, g.misc.x).x;
            let far = sample_cloud(p + to_sun_dir * slab * 1.1, earth_r, lift, d.style.y, g.misc.x).x;
            let shade = exp(-(near * 2.6 + far * 1.6));

            // Light from the storms, attenuated by the cloud it has already
            // crossed. This is the whole reason for marching: a flash inside a
            // thunderhead glows *through* it rather than merely brightening the
            // outside of it.
            var flash = 0.0;
            for (var k = 0u; k < 8u; k++) {
                let f = fl.items[k];
                let dist = length(p + centre - f.xyz);
                flash += f.w * exp(-dist / max(fl.reach.x, 1e-6));
            }

            let sun_col = srgb_to_linear(ALBEDO) * (NIGHT_FLOOR + day * shade * DAY_GAIN);
            let flash_col = srgb_to_linear(FLASH_TINT) * flash * s.y * 1.1;
            // Beer–Lambert over one step, integrated front to back.
            let absorbed = 1.0 - exp(-s.x * dt / earth_r * 900.0);
            lit += (sun_col + flash_col) * absorbed * transmittance;
            transmittance *= 1.0 - absorbed;
        }
        t += dt;
    }

    let alpha = (1.0 - transmittance) * d.params.z;
    if (alpha < 0.002) {
        discard;
    }
    // Premultiplied already: `lit` was accumulated weighted by transmittance.
    return vec4(lit * d.params.z, alpha);
}
