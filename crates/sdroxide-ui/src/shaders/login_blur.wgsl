// Softens the globe behind the sign-in screen, on its way into egui's pass.
//
// The scene is rendered in `CallbackTrait::prepare` into a private target at a
// fraction of the screen's resolution, and this is what runs inside egui's own
// pass. Two things fall out of that arrangement, both wanted:
//
// * the downscale *is* most of the blur — a 3× reduction resampled back up is
//   a wider kernel than anything affordable at full resolution, and it costs a
//   ninth of the fragments rather than more of them;
// * the nine taps below run on the small image, so their reach in screen
//   pixels is three times what their reach in texels suggests.
//
// The result is a backdrop the eye reads as depth of field rather than as a
// picture it is being asked to look at.

struct Blur {
    /// xy = the fraction of the allocated target actually rendered this frame.
    /// The target is over-allocated in steps so a window drag does not
    /// reallocate every frame; the UVs take up the slack.
    /// zw = one source texel in uv units.
    uv_scale: vec4<f32>,
    /// x = tap spacing in source texels, y = master alpha, z = centre dim,
    /// w = spare.
    params: vec4<f32>,
};

@group(0) @binding(0) var src: texture_2d<f32>;
@group(0) @binding(1) var samp: sampler;
@group(0) @binding(2) var<uniform> b: Blur;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
    /// Position within the widget, −1…1, for the centre dim.
    @location(1) ndc: vec2<f32>,
};

@vertex
fn vs(@builtin(vertex_index) i: u32) -> VsOut {
    let xy = vec2(f32((i << 1u) & 2u) * 2.0 - 1.0, f32(i & 2u) * 2.0 - 1.0);
    var o: VsOut;
    o.clip = vec4(xy, 0.0, 1.0);
    o.uv = vec2(xy.x * 0.5 + 0.5, 0.5 - xy.y * 0.5) * b.uv_scale.xy;
    o.ndc = xy;
    return o;
}

@fragment
fn fs(in: VsOut) -> @location(0) vec4<f32> {
    let step = b.uv_scale.zw * b.params.x;
    // A 3×3 tent, 1-2-1 by 1-2-1. Separating it would cost a second pass and a
    // second target to save four taps on an image this small.
    var sum = vec4(0.0);
    sum += textureSample(src, samp, in.uv + vec2(-step.x, -step.y)) * 1.0;
    sum += textureSample(src, samp, in.uv + vec2(0.0, -step.y)) * 2.0;
    sum += textureSample(src, samp, in.uv + vec2(step.x, -step.y)) * 1.0;
    sum += textureSample(src, samp, in.uv + vec2(-step.x, 0.0)) * 2.0;
    sum += textureSample(src, samp, in.uv) * 4.0;
    sum += textureSample(src, samp, in.uv + vec2(step.x, 0.0)) * 2.0;
    sum += textureSample(src, samp, in.uv + vec2(-step.x, step.y)) * 1.0;
    sum += textureSample(src, samp, in.uv + vec2(0.0, step.y)) * 2.0;
    sum += textureSample(src, samp, in.uv + vec2(step.x, step.y)) * 1.0;
    var col = sum / 16.0;

    // Held back where the card sits, full strength out at the corners. A
    // backdrop that competes with the two boxes the operator has to type into
    // is a backdrop that has failed at its one job.
    let r = length(in.ndc);
    let dim = mix(b.params.z, 1.0, smoothstep(0.10, 1.15, r));
    return vec4(col.rgb * dim, col.a * dim * b.params.y);
}
