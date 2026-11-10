// The auroral oval, as a stack of additive emission shells around the Earth.
//
// Aurora is a volume, not a surface: a curtain is a hundred kilometres of thin
// glowing air, and what makes it read as light rather than as paint is that you
// see *through* it. So the CPU hands this shader a series of concentric
// spheres at real atmospheric altitudes and each one contributes its own slice
// of the emission integral, additively. Three things fall out of that for free:
//
//   * The colour changes with height on its own, because the emission lines do
//     — green oxygen low down, crimson oxygen far above it, a violet nitrogen
//     fringe underneath when the precipitation is hard. Nothing here picks a
//     colour; it picks an altitude and the spectrum follows.
//   * The limb is brighter than the disk, because a grazing ray crosses far
//     more of every shell. That bright ribbon on the horizon is the single
//     most recognisable thing about aurora seen from orbit.
//   * The curtains are field-aligned, because the structure noise is a
//     function of *direction only* — it does not vary along the radius, so it
//     draws itself out into rays through the whole stack.
//
// The oval's shape and strength are not invented here: `aurora_tex` is NOAA
// SWPC's OVATION grid, one cell per degree, sampled with the sphere's own UVs.

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
    // x shell altitude (km), y the slab of atmosphere it stands for (km),
    // z intensity (screen-size fade × layer opacity), w spare.
    params: vec4<f32>,
    // Unused here; the block is shared with `solar_body.wgsl`.
    style: vec4<f32>,
};

@group(0) @binding(0) var<uniform> g: Globals;
@group(0) @binding(1) var land_tex: texture_2d<f32>;
@group(0) @binding(2) var sun_tex: texture_2d<f32>;
@group(0) @binding(3) var samp: sampler;
@group(0) @binding(4) var aurora_tex: texture_2d<f32>;
@group(1) @binding(0) var<uniform> d: DrawData;

// The auroral emission lines, as the colours they photograph as.
// 557.7 nm atomic oxygen — the green everyone has seen.
const GREEN_557  = vec3<f32>(0.180392, 1.000000, 0.482353); // #2eff7b
// 630.0 nm atomic oxygen — the crimson top of a big storm.
const RED_630    = vec3<f32>(1.000000, 0.145098, 0.235294); // #ff253c
// 427.8 nm ionised nitrogen — the violet fringe under a hard curtain.
const VIOLET_428 = vec3<f32>(0.435294, 0.360784, 1.000000); // #6f5cff

/// The grid's own dimensions, so the half-texel offset below is exact.
const GRID_W = 360.0;
const GRID_H = 181.0;

/// Relative strength of each line, at the point where they are summed over the
/// whole stack. Green is the reference: at ordinary activity 557.7 nm is far
/// the brightest thing in the sky, and only in a real storm does the red top
/// catch up with it — which is why the red and violet terms scale with the
/// activity and the green one does not.
const RED_WEIGHT = 0.10;
const VIOLET_WEIGHT = 0.45;
/// Overall gain. Set so a quiet oval is a clear but unobtrusive arc and a storm
/// saturates, which is the right way round for something that is trying to say
/// how big the event is.
const GAIN = 0.17;

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

/// A normalised bell, for an emission layer's vertical profile.
fn bell(x: f32, mu: f32, sigma: f32) -> f32 {
    let t = (x - mu) / sigma;
    return exp(-t * t);
}

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) nrm: vec3<f32>,
    @location(2) world: vec3<f32>,
};

@vertex
fn vs(@location(0) pos: vec3<f32>, @location(1) uv: vec2<f32>) -> VsOut {
    var o: VsOut;
    let world = d.model * vec4(pos, 1.0);
    o.clip = g.view_proj * world;
    o.world = world.xyz;
    o.nrm = normalize((d.basis * vec4(pos, 0.0)).xyz);
    o.uv = uv;
    return o;
}

@fragment
fn fs(in: VsOut) -> @location(0) vec4<f32> {
    // The grid holds one sample per whole degree and the sampler puts a texel's
    // centre at (i + 0.5)/N, so both axes need half a texel to line the cells
    // up with the degrees they were published for.
    let auv = vec2(
        in.uv.x + 0.5 / GRID_W,
        (in.uv.y * (GRID_H - 1.0) + 0.5) / GRID_H,
    );
    let p = textureSample(aurora_tex, samp, auv).r;
    // Two more taps north and south of it, for the gradient across the oval.
    // Taken here, before the early-out, so every texture read stays in uniform
    // control flow whatever the discard below does.
    let step_v = 2.5 / GRID_H;
    let p_n = textureSample(aurora_tex, samp, auv - vec2(0.0, step_v)).r;
    let p_s = textureSample(aurora_tex, samp, auv + vec2(0.0, step_v)).r;

    // Most of the planet has no aurora over it, and leaving early there is what
    // makes a stack of twenty full spheres affordable.
    if (p < 0.006) {
        discard;
    }

    // OVATION publishes a probability of *visibility*, so the low end is mostly
    // its own noise floor; the curve lifts the faint oval into view without
    // flattening a real storm against the top of the range.
    //
    // The `smoothstep` is not cosmetic. The grid is whole percentages on whole
    // degrees, and a lifting curve alone has infinite slope at zero, which
    // turns the interpolation between a cell reading 1 and one reading 2 into a
    // hard staircase along the edge of the oval. Fading the last percent out
    // smoothly is what stops the boundary looking like a cut-out.
    let strength =
        pow(max(p - 0.004, 0.0) / 0.996, 0.62) * smoothstep(0.004, 0.045, p);

    let alt = d.params.x;
    let slab = d.params.y;
    let n = normalize(in.nrm);

    // Which lines are radiating at this height. The 630 nm red line is
    // forbidden — it takes over a minute to decay, so it only survives where
    // collisions are rare, which is why red sits above green rather than mixing
    // with it. Both red and violet strengthen with the hardness of the
    // precipitation, which is what turns a green arc crimson in a real storm.
    let green  = bell(alt, 112.0, 32.0);
    let red    = bell(alt, 250.0, 105.0) * RED_WEIGHT * (0.15 + 0.85 * strength);
    let violet = bell(alt, 97.0, 11.0) * VIOLET_WEIGHT * strength * strength;
    let emission = srgb_to_linear(GREEN_557) * green
                 + srgb_to_linear(RED_630) * red
                 + srgb_to_linear(VIOLET_428) * violet;

    // Structure. Sampled from the *direction* alone, so it is identical at
    // every altitude and therefore draws itself out into vertical rays through
    // the whole stack — which is precisely what field-aligned precipitation
    // looks like. The slow translation is the drift of the curtains.
    //
    // Squashing the polar axis stretches the noise out along circles of
    // latitude, so the fine structure comes out as arcs lying along the oval
    // rather than as isotropic blobs — the banding in every photograph of the
    // aurora from above. It is multiplicative and centred on one, so it can
    // shape what the data says but never invent aurora where there is none.
    let drift = g.misc.x * 0.013;
    let banded = vec3(n.x, n.y, n.z * 6.0);
    let folds = vnoise(n * 7.0 + vec3(drift, drift * 0.35, 0.0));
    let rays = vnoise(banded * 30.0 + vec3(drift * 1.6, 0.0, drift));
    let fine = vnoise(banded * 96.0 + vec3(0.0, drift * 2.4, 0.0));
    let structure =
        (0.30 + 1.05 * folds) * (0.34 + 1.25 * pow(rays, 1.5)) * (0.62 + 0.70 * fine) * 1.3;

    // A discrete arc rides the boundary of the oval, where the precipitation
    // cuts off sharpest — the hard bright line along the equatorward edge that
    // every photograph of an aurora has, and the diffuse glow behind it. This
    // comes from the gradient of the grid, so it marks the edge the data
    // actually has rather than one drawn on for effect.
    let edge = 1.0 + 1.4 * smoothstep(0.015, 0.10, abs(p_n - p_s));

    // Path length through the shell: exactly 1/cos of the incidence angle.
    // Edge-on you look along a curtain instead of across it, which — together
    // with the far side of each shell clearing the planet's silhouette and so
    // being drawn as well — is why the oval is a thin bright ribbon on the limb
    // and a faint wash on the disk.
    let to_eye = normalize(g.camera_pos.xyz - in.world);
    let grazing = min(1.0 / max(abs(dot(n, to_eye)), 0.22), 3.2);

    // Daylight. The emission does not stop on the sunlit side, it is simply
    // drowned out — but this is a data display as well as a picture, so the
    // daylit half of the oval fades to a floor rather than to nothing.
    let to_sun = normalize(g.sun_pos.xyz - in.world);
    let night = 1.0 - smoothstep(-0.22, 0.06, dot(n, to_sun));
    let visible = mix(0.16, 1.0, night);

    // `slab` makes this a Riemann sum: shells stand for different thicknesses
    // of atmosphere, so brightness must not depend on how many were drawn.
    let i = GAIN * strength * structure * edge * grazing * visible * d.params.z * (slab / 24.0);
    // Zero alpha: light adds to what is behind it, it does not hide it.
    return vec4(emission * i, 0.0);
}
