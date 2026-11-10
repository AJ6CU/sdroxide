// Screen-facing billboards: body glows, the star field, the QTH ring and point
// markers. One instanced quad, `params.x` picks the shape.
//
// The glow is what guarantees a body is never invisible: at 2 AU the Earth is a
// fraction of a pixel across, and a minimum-size glow decouples "can I see it"
// from the radius-exaggeration slider entirely.

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

const KIND_GLOW = 0.0;
const KIND_STAR = 1.0;
const KIND_RING = 2.0;
const KIND_DOT  = 3.0;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) kind: f32,
};

@vertex
fn vs(
    @location(0) corner: vec2<f32>,
    @location(1) center: vec3<f32>,
    @location(2) size_px: f32,
    @location(3) color: vec4<f32>,
    @location(4) params: vec4<f32>,
) -> VsOut {
    var o: VsOut;
    o.uv = corner;
    o.color = color;
    o.kind = params.x;

    let p = g.view_proj * vec4(center, 1.0);
    if (p.w <= 0.0) {
        o.clip = vec4(0.0, 0.0, -1.0, 1.0);
        return o;
    }
    let n = p.xy / p.w;
    let off = corner * size_px * g.viewport.zw;
    o.clip = vec4((n + off) * p.w, p.z, p.w);
    return o;
}

@fragment
fn fs(in: VsOut) -> @location(0) vec4<f32> {
    let r = length(in.uv);
    var a = 0.0;
    var additive = true;

    if (in.kind < 0.5) {
        // Soft falloff, deliberately wide so a distant body still catches the eye.
        a = exp(-r * r * 4.2) * (1.0 - smoothstep(0.85, 1.0, r));
    } else if (in.kind < 1.5) {
        a = (1.0 - smoothstep(0.15, 1.0, r));
        a = a * a;
    } else if (in.kind < 2.5) {
        // Annulus: a locator ring that stays legible over any terrain.
        a = 1.0 - smoothstep(0.10, 0.28, abs(r - 0.66));
        additive = false;
    } else {
        a = 1.0 - smoothstep(0.55, 0.80, r);
        additive = false;
    }

    a = a * in.color.a;
    if (a <= 0.001) {
        discard;
    }
    // Premultiplied output. Additive shapes emit zero alpha so they add light
    // without occluding; markers use real alpha so they sit on top.
    return vec4(in.color.rgb * a, select(a, 0.0, additive));
}
