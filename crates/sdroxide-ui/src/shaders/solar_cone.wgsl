// CME trajectory cones.
//
// The mesh is parametric — `pos` is `[azimuth, t, kind]`, not a position — so
// one static mesh serves every event and the half-angle arrives as a uniform.
// The leading edge is a spherical cap rather than a flat disc because a CME
// expands radially: every point of the front is the same distance from the Sun,
// which is what makes the arrival geometry readable.

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
    model: mat4x4<f32>,
    basis: mat4x4<f32>,
    tint: vec4<f32>,
    tint2: vec4<f32>,
    // x mode, y half-angle (rad), z alpha, w inner radius as a fraction of the
    // front distance (the cone is truncated there — see the vertex shader).
    params: vec4<f32>,
    // Unused here; the block is shared with `solar_body.wgsl`.
    style: vec4<f32>,
};

@group(0) @binding(0) var<uniform> g: Globals;
@group(0) @binding(1) var land_tex: texture_2d<f32>;
@group(0) @binding(2) var sun_tex: texture_2d<f32>;
@group(0) @binding(3) var samp: sampler;
@group(1) @binding(0) var<uniform> d: DrawData;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    /// 0 at the apex, 1 at the leading edge.
    @location(0) along: f32,
    @location(1) nrm: vec3<f32>,
    @location(2) world: vec3<f32>,
};

@vertex
fn vs(@location(0) p: vec3<f32>, @location(1) uv: vec2<f32>) -> VsOut {
    let phi = p.x;
    let t = p.y;
    let is_cap = p.z > 0.5;
    let half_angle = d.params.y;
    let inner = d.params.w;

    // Polar angle from the axis: the full half-angle on the lateral surface,
    // sweeping in from the nose across the cap.
    let a = select(half_angle, t * half_angle, is_cap);
    let dir = vec3(sin(a) * cos(phi), sin(a) * sin(phi), cos(a));
    // The cone is a frustum, not a full cone: it begins at the 21.5 R☉ height
    // that DONKI quotes the speed from, which is both more truthful and what
    // stops a close-up of the solar disk from being swallowed by the inside
    // surface of every cone in the display.
    let radius = select(mix(inner, 1.0, t), 1.0, is_cap);
    let local = dir * radius;

    var o: VsOut;
    let world = d.model * vec4(local, 1.0);
    o.clip = g.view_proj * world;
    o.world = world.xyz;
    o.along = uv.y;
    // Outward normal: the cap's is radial, the lateral surface's is
    // perpendicular to the slant.
    let lateral = vec3(cos(a) * cos(phi), cos(a) * sin(phi), -sin(a));
    let local_n = select(lateral, dir, is_cap);
    o.nrm = normalize((d.basis * vec4(local_n, 0.0)).xyz);
    return o;
}

@fragment
fn fs(in: VsOut) -> @location(0) vec4<f32> {
    let to_eye = normalize(g.camera_pos.xyz - in.world);
    // Edge-on faces glow: a shell reads as a volume that way, and the front
    // stays visible when looked at straight on.
    let facing = abs(dot(normalize(in.nrm), to_eye));
    let rim = pow(1.0 - facing, 2.0);

    // Faint at the apex, bright at the leading edge — the front is where the
    // plasma actually is.
    let ramp = 0.10 + 0.90 * pow(in.along, 2.2);
    let a = clamp(d.params.z * ramp * (0.20 + 0.80 * rim), 0.0, 1.0);
    return vec4(d.tint.rgb * a, a * 0.75);
}
