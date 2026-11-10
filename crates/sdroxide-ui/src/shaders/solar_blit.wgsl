// Copies the offscreen 3D render into egui's render pass.
//
// The whole scene is drawn in `CallbackTrait::prepare` into private colour and
// depth targets, and this is all that runs inside egui's own pass. That keeps
// the depth buffer, MSAA and vertex buffers out of the shared pass entirely, so
// nothing about the waterfall pipelines or the browser build has to change.

@group(0) @binding(0) var src: texture_2d<f32>;
@group(0) @binding(1) var samp: sampler;

struct Blit {
    /// xy = the fraction of the allocated target actually rendered this frame.
    /// The target is over-allocated in 128 px steps so a window drag does not
    /// reallocate every frame; the UVs take up the slack.
    uv_scale: vec4<f32>,
};
@group(0) @binding(2) var<uniform> b: Blit;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs(@builtin(vertex_index) i: u32) -> VsOut {
    // One oversized triangle covering the viewport — no vertex buffer, and no
    // seam down the diagonal of a two-triangle quad.
    let xy = vec2(f32((i << 1u) & 2u) * 2.0 - 1.0, f32(i & 2u) * 2.0 - 1.0);
    var o: VsOut;
    o.clip = vec4(xy, 0.0, 1.0);
    o.uv = vec2(xy.x * 0.5 + 0.5, 0.5 - xy.y * 0.5) * b.uv_scale.xy;
    return o;
}

@fragment
fn fs(in: VsOut) -> @location(0) vec4<f32> {
    return textureSample(src, samp, in.uv);
}
