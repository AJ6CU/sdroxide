// A planet's ring system: a flat annulus in the body's equatorial plane.
//
// Like the CME cone, the geometry is parametric — the mesh carries (azimuth, t)
// and the vertex shader builds the ring between the inner and outer radii the
// draw asks for, so one static mesh serves Saturn and Uranus both.

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
    // x inner radius as a fraction of the outer, y opacity, z the planet's
    // radius in the same units, w 0 = broad sheet, 1 = narrow ringlets.
    params: vec4<f32>,
    style: vec4<f32>,
};

@group(0) @binding(0) var<uniform> g: Globals;
@group(1) @binding(0) var<uniform> d: DrawData;

const TAU = 6.28318531;

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    // Position across the ring, 0 at the inner edge and 1 at the outer.
    @location(0) t: f32,
    @location(1) world: vec3<f32>,
    // Offset from the planet's centre in the same units the model is scaled
    // in, which is what the shadow test needs.
    @location(2) local: vec3<f32>,
};

@vertex
fn vs(@location(0) p: vec3<f32>, @location(1) uv: vec2<f32>) -> VsOut {
    var o: VsOut;
    let phi = p.x;
    let t = p.y;
    let r = mix(d.params.x, 1.0, t);
    let local = vec3(cos(phi) * r, sin(phi) * r, 0.0);
    let world = d.model * vec4(local, 1.0);
    o.clip = g.view_proj * world;
    o.world = world.xyz;
    o.local = (d.basis * vec4(local, 0.0)).xyz;
    o.t = t;
    return o;
}

/// Saturn's rings, by fraction of the way across: the dim C ring, the bright B
/// ring, the Cassini division, then the A ring with the Encke gap near its
/// outer edge. The numbers are where those features fall between the 74 660 km
/// inner edge and the 136 780 km outer one.
fn saturn_profile(t: f32) -> f32 {
    var v = 0.0;
    v = v + 0.30 * (smoothstep(-0.02, 0.03, t) - smoothstep(0.20, 0.24, t));      // C
    v = v + 0.95 * (smoothstep(0.21, 0.26, t) - smoothstep(0.60, 0.628, t));      // B
    v = v + 0.10 * (smoothstep(0.62, 0.64, t) - smoothstep(0.68, 0.70, t));       // Cassini
    v = v + 0.62 * (smoothstep(0.69, 0.71, t) - smoothstep(0.985, 1.0, t));       // A
    v = v * (1.0 - 0.75 * (smoothstep(0.865, 0.874, t) - smoothstep(0.874, 0.883, t))); // Encke
    // Fine structure: the ringlets that make the sheet read as thousands of
    // separate orbits rather than as a painted disc.
    return v * (0.85 + 0.15 * sin(t * 210.0) * sin(t * 37.0));
}

/// Uranus: nine narrow rings, of which ε is much the widest.
fn ringlets(t: f32) -> f32 {
    var v = 0.0;
    var pos = array<vec2<f32>, 5>(
        vec2(0.02, 0.5), vec2(0.24, 0.4), vec2(0.46, 0.45), vec2(0.71, 0.5), vec2(0.985, 1.0),
    );
    for (var i = 0; i < 5; i = i + 1) {
        v = v + pos[i].y * smoothstep(0.022, 0.0, abs(t - pos[i].x));
    }
    return v;
}

@fragment
fn fs(in: VsOut) -> @location(0) vec4<f32> {
    let profile = select(saturn_profile(in.t), ringlets(in.t), d.params.w > 0.5);
    if (profile <= 0.002) {
        discard;
    }

    let to_sun = normalize(g.sun_pos.xyz - in.world);
    let normal = normalize((d.basis * vec4(0.0, 0.0, 1.0, 0.0)).xyz);
    // Rings are a flat sheet, so they do dim as the Sun approaches their plane
    // — that is what makes Saturn's rings vanish at equinox. But a plain
    // cosine is far too dark for the 27° they sit at most of the time: these
    // are metre-scale ice boulders that backscatter hard, and in every
    // photograph ever taken they are the brightest thing in the frame.
    let illum = 0.25 + 0.85 * pow(abs(dot(normal, to_sun)), 0.35);

    // The planet's shadow. The perpendicular distance from the planet's centre
    // to the ray a ring particle sends towards the Sun; inside the planet's
    // radius, and behind it, is night.
    let along = dot(in.local, to_sun);
    let perp = length(in.local - to_sun * along);
    let shadow = select(
        1.0,
        smoothstep(d.params.z * 0.92, d.params.z * 1.06, perp),
        along < 0.0,
    );

    // Colour across the ring: ice at the bright B ring, dustier towards both
    // edges, which is roughly what the composition does.
    let col = mix(d.tint2.rgb, d.tint.rgb, clamp(profile * 1.2, 0.0, 1.0));
    // Edge-on, a ray crosses far more of the sheet, so the thin parts thicken
    // up — the same reason the rings brighten into a hard line near equinox.
    let to_eye = normalize(g.camera_pos.xyz - in.world);
    let grazing = 1.0 - abs(dot(normal, to_eye));
    let alpha = clamp(profile * d.params.y * (0.92 + 0.5 * grazing), 0.0, 1.0);

    let lit = col * illum * max(shadow, 0.08);
    // Premultiplied, like every other blended pass here.
    return vec4(lit * alpha, alpha);
}
