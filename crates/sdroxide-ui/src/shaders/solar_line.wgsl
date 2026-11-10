// Orbit paths, the solar axis and the heliographic graticule.
//
// wgpu's line topology is always exactly one pixel wide, so every stroke here
// is an instanced quad expanded to screen space in the vertex shader. That also
// makes the width resolution-independent, which a 3D-space tube would not be.

struct Globals {
    view_proj: mat4x4<f32>,
    camera_pos: vec4<f32>,
    sun_pos: vec4<f32>,
    sun_to_earth: vec4<f32>,
    solar_north: vec4<f32>,
    viewport: vec4<f32>,
    misc: vec4<f32>,
};

@group(0) @binding(0) var<uniform> g: Globals;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) color: vec4<f32>,
    /// −1..1 across the stroke, for the soft edge.
    @location(1) edge: f32,
};

@vertex
fn vs(
    @location(0) corner: vec2<f32>,
    @location(1) a: vec3<f32>,
    @location(2) width_px: f32,
    @location(3) b: vec3<f32>,
    @location(4) color: vec4<f32>,
) -> VsOut {
    var o: VsOut;
    o.color = color;
    o.edge = corner.y;

    let pa = g.view_proj * vec4(a, 1.0);
    let pb = g.view_proj * vec4(b, 1.0);
    // A segment with an endpoint behind the eye cannot be expanded in screen
    // space. Collapse it outside the clip volume instead: orbit rings are
    // hundreds of segments, so dropping the one or two that straddle the eye is
    // invisible, and near-plane clipping the strip properly is not worth it.
    if (pa.w <= 0.0 || pb.w <= 0.0) {
        o.clip = vec4(0.0, 0.0, -1.0, 1.0);
        return o;
    }

    let na = pa.xy / pa.w;
    let nb = pb.xy / pb.w;
    let delta = (nb - na) * g.viewport.xy;
    var dir = vec2(1.0, 0.0);
    if (length(delta) > 1e-9) {
        dir = normalize(delta);
    }
    let perp = vec2(-dir.y, dir.x);

    let at_b = corner.x > 0.0;
    let p = select(pa, pb, at_b);
    let n = select(na, nb, at_b);
    // Half-width in pixels → NDC (which spans 2 units across the viewport).
    let off = perp * (width_px * 0.5 * corner.y) * g.viewport.zw * 2.0;
    o.clip = vec4((n + off) * p.w, p.z, p.w);
    return o;
}

@fragment
fn fs(in: VsOut) -> @location(0) vec4<f32> {
    let a = in.color.a * (1.0 - smoothstep(0.35, 1.0, abs(in.edge)));
    // Premultiplied, with zero output alpha: the strokes add light rather than
    // occluding, which is what makes overlapping orbit rings read.
    return vec4(in.color.rgb * a, 0.0);
}
