// The propagation heat map, as one shell of paint just above the surface.
//
// Unlike every other data layer in this window, this shader picks no colours.
// The whole field — the ramp, the per-band hues, the mixing where two bands
// overlap, the alpha — is resolved on the CPU in `prop_map::PropHeat::rgba` and
// arrives here as finished RGBA. The flat map in the operating panel uploads
// exactly the same bytes as an egui texture. That is deliberate: the two views
// are not merely consistent by convention, they are the same pixels, and there
// is no second copy of the colour rules to drift.
//
// What is left here is what only the GPU can do: put it on a sphere, filter
// 2.5° cells into smooth shapes, and light it like paint rather than like
// emission.
//
// Paint, not light: the aurora adds to what is behind it because a curtain is
// glowing air you see through. Heat is a property *of* the ground, so it
// composites over the surface and its alpha is capped well below opaque so the
// coastline stays readable underneath. Getting that backwards would wash the
// whole daylit hemisphere out.

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
    // x: overall opacity (the layer's screen-size fade). yzw spare.
    params: vec4<f32>,
    style: vec4<f32>,
};

@group(0) @binding(0) var<uniform> g: Globals;
@group(0) @binding(1) var land_tex: texture_2d<f32>;
@group(0) @binding(2) var sun_tex: texture_2d<f32>;
@group(0) @binding(3) var samp: sampler;
@group(0) @binding(9) var prop_tex: texture_2d<f32>;
@group(1) @binding(0) var<uniform> d: DrawData;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
    @location(1) nrm: vec3<f32>,
    @location(2) world: vec3<f32>,
};

// The propagation grid: 2.5° cells, row-major from the north pole. Must match
// `PROP_GRID_W`/`PROP_GRID_H`, which a test in `tests/shaders.rs` checks.
const GRID_W = 144.0;
const GRID_H = 72.0;

// Cubic B-spline weights for the four texels around a fractional position.
//
// Chosen over Catmull-Rom, the other cubic reached for here, because these
// weights are all non-negative: an interpolating filter overshoots at an edge
// and would ring a dark halo around every hot patch. This one does not pass
// through its samples — it smooths them — which is the right trade for a field
// whose cells are already a 400 km smear of something known no better.
fn cubic(v: f32) -> vec4<f32> {
    let n = 1.0 - v;
    return vec4(
        n * n * n,
        4.0 + 3.0 * v * v * v - 6.0 * v * v,
        4.0 + 3.0 * n * n * n - 6.0 * n * n,
        v * v * v,
    ) / 6.0;
}

// The heat at `uv`, filtered so the grid does not show through it.
//
// Hardware bilinear filtering of a 2.5° grid is not enough on a globe: a cell
// is ten pixels across, and a bilinear patch has a kink in its gradient at
// every cell boundary, so a smooth field reads as a quilt of quadrilaterals.
// This is a 4×4 cubic instead, which is continuous in its curvature and shows
// no seam at all.
//
// Four samples rather than sixteen: each neighbouring pair of texels is folded
// into one bilinear tap placed off-centre so the hardware's own lerp produces
// the pair's weighted sum. The taps land within two texels of the fragment, and
// nothing is done to keep them in range on purpose — the sampler repeats in `u`
// so the antimeridian wraps, and clamps in `v` so a tap past the pole reads the
// polar row, which is what a plain sample does there too.
//
// The tap coordinates jump between texels, so the derivatives the hardware
// infers from them are meaningless. That is safe only because this texture has
// a single mip level: there is no other level for a bad derivative to select.
fn sample_heat(uv: vec2<f32>) -> vec4<f32> {
    let tc = uv * vec2(GRID_W, GRID_H) - 0.5;
    let base = floor(tc);
    let f = tc - base;
    let wx = cubic(f.x);
    let wy = cubic(f.y);
    // Weight of each folded pair, and where inside the pair the tap must sit.
    let gx = vec2(wx.x + wx.y, wx.z + wx.w);
    let gy = vec2(wy.x + wy.y, wy.z + wy.w);
    let px = (vec2(base.x - 1.0 + wx.y / gx.x, base.x + 1.0 + wx.w / gx.y) + 0.5) / GRID_W;
    let py = (vec2(base.y - 1.0 + wy.y / gy.x, base.y + 1.0 + wy.w / gy.y) + 0.5) / GRID_H;

    let top = gx.x * textureSample(prop_tex, samp, vec2(px.x, py.x))
            + gx.y * textureSample(prop_tex, samp, vec2(px.y, py.x));
    let bot = gx.x * textureSample(prop_tex, samp, vec2(px.x, py.y))
            + gx.y * textureSample(prop_tex, samp, vec2(px.y, py.y));
    // The sixteen weights sum to one, so the pairs need no renormalising.
    return gy.x * top + gy.y * bot;
}

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
    // The grid is row-major from the north pole and starts at 180° W, which is
    // the sphere's own UV convention, so no remapping is needed — the reason
    // `PropField` is laid out that way. A cell holds the value at its own
    // centre, so the sampler's half-texel offset is already right and there is
    // nothing to shift, unlike the aurora grid next door.
    let c = sample_heat(in.uv);
    var a = c.a * d.params.x;

    // Fade the shell out where it is edge-on. A sphere's silhouette is where a
    // surface texture is crossed at a grazing angle, and without this the rim
    // gathers into a hard bright ring that reads as a feature of the data
    // rather than of the geometry.
    let n = normalize(in.nrm);
    let to_eye = normalize(g.camera_pos.xyz - in.world);
    a = a * smoothstep(0.03, 0.34, abs(dot(n, to_eye)));

    if (a < 0.004) {
        discard;
    }
    // Premultiplied, matching the blend state this pipeline is built with.
    return vec4(c.rgb * a, a);
}
