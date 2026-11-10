// Comet tails.
//
// A tail is a hundred million kilometres of gas thin enough to see stars
// through, so it is drawn as emission: additive, no depth write, but depth
// *tested*, which is what puts the far half of a tail behind the planet it is
// passing.
//
// The mesh is parametric — `pos` is `[s, t, 0]`, across and along, not a
// position — and the ribbon is expanded perpendicular to both the tail's axis
// and the eye. That matters: a tube would vanish when looked at end-on, which
// is exactly the view you get from the comet's own orbit plane, and a fixed
// billboard would shear as the camera swung round it.
//
// The two tails a comet has are genuinely different objects and the shader
// treats them so:
//
//   * The **ion tail** is CO+ fluorescing at 420 nm — blue, not reflected
//     sunlight. The plasma is tied to the solar wind's magnetic field, so it
//     runs dead straight, stays narrow, and breaks into rays and travelling
//     knots as the field it is frozen into varies.
//   * The **dust tail** is grains reflecting sunlight — warm and white. Too
//     heavy for the wind, they drift out under radiation pressure while keeping
//     the orbital speed they left with, so the tail bows away from the
//     anti-solar line, broadens, and is smooth where the ion tail is streaky.

struct Globals {
    view_proj: mat4x4<f32>,
    camera_pos: vec4<f32>,
    sun_pos: vec4<f32>,
    sun_to_earth: vec4<f32>,
    solar_north: vec4<f32>,
    viewport: vec4<f32>,
    misc: vec4<f32>,
};

struct DrawData {
    // Columns: the direction the tail curves towards, its binormal, the axis it
    // runs along, and the nucleus. No scale — the shader works in gigametres.
    model: mat4x4<f32>,
    basis: mat4x4<f32>,
    // Near colour, and the colour it fades towards down its length.
    tint: vec4<f32>,
    tint2: vec4<f32>,
    // x = KIND_*, y = length (Gm), z = width at the head (Gm), w = how far the
    // tip bows off the axis, as a fraction of the length.
    params: vec4<f32>,
    // x = activity, 0..1. y..w spare.
    style: vec4<f32>,
};

@group(0) @binding(0) var<uniform> g: Globals;
@group(0) @binding(1) var land_tex: texture_2d<f32>;
@group(0) @binding(2) var sun_tex: texture_2d<f32>;
@group(0) @binding(3) var samp: sampler;
@group(1) @binding(0) var<uniform> d: DrawData;

const KIND_ION  = 0.0;
const KIND_DUST = 1.0;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    /// −1 to 1 across the ribbon.
    @location(0) across: f32,
    /// 0 at the nucleus, 1 at the tip.
    @location(1) along: f32,
    @location(2) world: vec3<f32>,
};

/// How wide the tail is at `t`, in units of the head width.
///
/// A comet tail is not a cone. It leaves the coma already narrowed — the wind
/// collimates the ions into a neck a few thousand km across — and then opens
/// out slowly, so the profile is a root rather than a line. The dust tail,
/// which is not collimated by anything, opens faster.
fn flare(t: f32, dust: bool) -> f32 {
    let k = select(1.9, 3.4, dust);
    let p = select(0.55, 0.75, dust);
    return 0.30 + k * pow(t, p);
}

@vertex
fn vs(@location(0) p: vec3<f32>, @location(1) uv: vec2<f32>) -> VsOut {
    let s = p.x;
    let t = p.y;
    let dust = d.params.x > 0.5;
    let tail_len = d.params.y;
    let width = d.params.z;
    let curve = d.params.w;

    // Down the axis, bowing towards +x as the square of the distance out: dust
    // released further back has been pushed out for longer, which is what puts
    // the classic curve in a dust tail and leaves an ion tail straight.
    let centre_local = vec3(curve * tail_len * t * t, 0.0, tail_len * t);
    let centre = (d.model * vec4(centre_local, 1.0)).xyz;

    // The tangent, in world space, so the ribbon can be expanded across it. The
    // curved tail's tangent turns along its length, so it is differentiated
    // rather than taken to be the axis.
    let axis = normalize((d.basis * vec4(0.0, 0.0, 1.0, 0.0)).xyz);
    let bend = normalize((d.basis * vec4(1.0, 0.0, 0.0, 0.0)).xyz);
    let tangent = normalize(axis + bend * (2.0 * curve * t));

    // Perpendicular to the tail and to the line of sight: the ribbon turns to
    // face the eye, so it never edges out of existence.
    let to_eye = normalize(g.camera_pos.xyz - centre);
    var side = cross(tangent, to_eye);
    let len = length(side);
    // Looking straight down the tail there is no such direction; any
    // perpendicular will do, and the fragment shader has faded it to nothing by
    // then anyway.
    side = select(normalize(cross(tangent, vec3(0.0, 0.0, 1.0))), side / max(len, 1e-9),
                  len > 1e-6);

    let world = centre + side * (s * width * flare(t, dust));

    var o: VsOut;
    o.clip = g.view_proj * vec4(world, 1.0);
    o.across = uv.x;
    o.along = uv.y;
    o.world = world;
    return o;
}

/// Cheap hash noise, for the ion tail's rays and knots.
fn hash(n: f32) -> f32 {
    return fract(sin(n * 12.9898) * 43758.5453);
}

fn noise(x: f32) -> f32 {
    let i = floor(x);
    let f = fract(x);
    let u = f * f * (3.0 - 2.0 * f);
    return mix(hash(i), hash(i + 1.0), u);
}

@fragment
fn fs(in: VsOut) -> @location(0) vec4<f32> {
    let dust = d.params.x > 0.5;
    let activity = d.style.x;
    let t = clamp(in.along, 0.0, 1.0);
    let r = abs(in.across);

    // Across the tail: a bright core in a broad skirt. The ion tail's core is
    // much tighter, which is what makes it read as a beam next to the dust.
    let core_w = select(0.30, 0.62, dust);
    var across = exp(-(r * r) / (core_w * core_w));
    across = across + 0.35 * exp(-(r * r) / 0.85);
    // Hard zero at the mesh edge, so the ribbon has no visible boundary.
    across = across * (1.0 - smoothstep(0.72, 1.0, r));

    // Along the tail: brightest just behind the head, where the gas has been
    // swept out of the coma but not yet spread, then falling away. Not a
    // straight exponential — the very base is dimmer, because there the coma is
    // in front of it. The ion tail holds its brightness further out than the
    // dust does, which is why it is the one that reaches across a photograph.
    let rise = smoothstep(0.0, 0.05, t);
    var along = rise * exp(-t * select(1.9, 3.4, dust));
    // ...and gone by the tip rather than cut off at it.
    along = along * (1.0 - smoothstep(0.55, 1.0, t));

    var v = across * along;

    if (!dust) {
        // Rays. An ion tail is not a smooth plume: the plasma is frozen into
        // the wind's magnetic field, and the streamers that makes converge back
        // towards the nucleus. Sampling the noise in `across / (t + k)` is what
        // makes them converge — the same feature is narrower nearer the head.
        let ray = noise(in.across / (t * 0.9 + 0.06) * 3.1 + 11.0);
        v = v * (0.62 + 0.72 * ray);
        // Knots, travelling outward. Disconnection events and field-line
        // draping send condensations down a real ion tail over hours; this is
        // the same motion at a speed the eye can follow.
        let knot = noise(t * 9.0 - g.misc.x * 0.06);
        v = v * (0.80 + 0.42 * knot);
    } else {
        // Dust is smooth, but not featureless: striae from discrete outbursts
        // lie along the tail rather than across it.
        v = v * (0.88 + 0.18 * noise(t * 4.0 + in.across * 1.7));
    }

    // Colour cools and dims down the length as the gas thins out.
    let colour = mix(d.tint.rgb, d.tint2.rgb, smoothstep(0.05, 0.85, t));
    // The ion tail is driven hard on purpose. `activity` is an honest inverse
    // square, so a comet with a 1 AU perihelion sits around a quarter of full —
    // physically right, and far too dim to see against a black sky once it has
    // been through the width and length falloffs as well. Saturating the core
    // near the head is also what a bright comet actually looks like: a hard
    // white-blue spine that only resolves into structure further out. The dust
    // stays subdued so the two never compete.
    let gain = select(3.2, 1.05, dust);
    let a = clamp(v * (0.22 + 0.78 * activity) * gain, 0.0, 1.0);
    if (a <= 0.002) {
        discard;
    }
    // Premultiplied, and emitting zero alpha: a tail adds light to whatever is
    // behind it and occludes nothing, which is what looking through one is.
    return vec4(colour * a, 0.0);
}
