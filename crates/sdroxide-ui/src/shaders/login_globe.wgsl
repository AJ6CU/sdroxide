// The slowly turning globe behind the sign-in screen.
//
// One fullscreen triangle and one ray-sphere intersection: there is no mesh, no
// depth buffer and no vertex data at all, which is what keeps this inside the
// WebGL2 downlevel budget the rest of the crate's shared rendering is written
// to. The limb is analytic, so it stays perfectly round however close the
// camera gets — a tessellated sphere would show its facets at this distance.
//
// The maps are the same two the 3D view draws the Earth from
// (`assets/earth/{land,borders}.png`, Natural Earth), sampled in the same ECEF
// convention `solar3d::mesh::sphere` builds: +X at (0°N, 0°E), +Z at the north
// pole, u = (lon+180)/360, v = (90−lat)/180.
//
// **Scale.** Everything here is in globe radii, with the sphere at the origin
// and radius exactly 1. The 3D view works in gigametres, where the Earth's
// radius is 0.006371 and a surface point carries barely three significant
// figures of f32 — fine for a body a few dozen pixels across, but the rotation
// visibly steps when the globe fills the screen. At unit scale every quantity
// in this file sits in the range f32 resolves best, and the spin is smooth.

struct Globe {
    // Rows of the 3×3 that takes a world direction into globe-local space:
    // the spin and the axial tilt, inverted. Rotating the *ray* rather than the
    // globe is what keeps the map and the arcs turning as one, with no
    // per-arc transform on the way in.
    rot0: vec4<f32>,
    rot1: vec4<f32>,
    rot2: vec4<f32>,
    /// xyz eye position in globe radii, w = focal length (1/tan(½ fov)).
    camera: vec4<f32>,
    /// xyz unit direction to the sun, in world space. w = arc colour mix.
    sun: vec4<f32>,
    /// x aspect (w/h), y arc half-width in radii, z station dot radius, w spare.
    misc: vec4<f32>,
    /// Two per arc: (a.xyz, drawn fraction) and (b.xyz, alpha).
    arcs: array<vec4<f32>, 32>,
};

@group(0) @binding(0) var<uniform> g: Globe;
@group(0) @binding(1) var land_tex: texture_2d<f32>;
@group(0) @binding(2) var border_tex: texture_2d<f32>;
@group(0) @binding(3) var samp: sampler;

const PI = 3.14159265;
const MAX_ARCS = 16u;

// The FT8 map's palette, so this reads as the same Earth the 3D view draws
// (see widgets/worldmap.rs and shaders/solar_body.wgsl).
const LAND_DAY  = vec3<f32>(0.109804, 0.266667, 0.345098); // #1c4458
const OCEAN_DAY = vec3<f32>(0.039216, 0.094118, 0.149020); // #0a1826
const COAST     = vec3<f32>(0.113725, 0.611765, 0.745098); // #1d9cbe  theme::CYAN_DIM
const ATMO      = vec3<f32>(0.000000, 0.815686, 0.956863); // #00d0f4  theme::CYAN
const ARC_HOT   = vec3<f32>(1.000000, 0.823529, 0.247059); // #ffd23f  theme::YELLOW

struct VsOut {
    @builtin(position) clip: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs(@builtin(vertex_index) i: u32) -> VsOut {
    // One oversized triangle covering the viewport — no vertex buffer, and no
    // seam down the diagonal a two-triangle quad would have.
    let xy = vec2(f32((i << 1u) & 2u) * 2.0 - 1.0, f32(i & 2u) * 2.0 - 1.0);
    var o: VsOut;
    o.clip = vec4(xy, 0.0, 1.0);
    o.uv = vec2(xy.x * 0.5 + 0.5, 0.5 - xy.y * 0.5);
    return o;
}

fn to_local(v: vec3<f32>) -> vec3<f32> {
    return vec3(dot(g.rot0.xyz, v), dot(g.rot1.xyz, v), dot(g.rot2.xyz, v));
}

/// Equirectangular texture coordinates for a unit surface normal.
fn equirect(n: vec3<f32>) -> vec2<f32> {
    let lon = atan2(n.y, n.x);
    let lat = asin(clamp(n.z, -1.0, 1.0));
    return vec2(lon / (2.0 * PI) + 0.5, 0.5 - lat / PI);
}

/// How fast the texture coordinates move per unit of normal, differentiated by
/// hand.
///
/// The screen-space derivative of `equirect` itself cannot be used: `atan2`
/// wraps at the date line, so `dpdx` there is a whole texture wide and the
/// hardware picks the coarsest mip — a bright seam down the Pacific that no
/// amount of sampler configuration fixes. The normal is continuous across the
/// seam, so differentiating *it* and converting analytically is not an
/// optimisation but the only correct way round.
fn equirect_grad(n: vec3<f32>, dn: vec3<f32>) -> vec2<f32> {
    let horiz = max(n.x * n.x + n.y * n.y, 1e-6);
    let du = (n.x * dn.y - n.y * dn.x) / horiz / (2.0 * PI);
    let dv = -dn.z / (PI * sqrt(max(1.0 - n.z * n.z, 1e-6)));
    return vec2(du, dv);
}

/// Glow from one great-circle arc, as seen along `rd` from `ro`.
///
/// The arc bows off the surface, so it is not a curve on the sphere and cannot
/// be tested against the surface point. What makes it cheap anyway is that the
/// whole arc lies in one plane through the origin: intersect the ray with that
/// plane, and the problem collapses to a distance in two dimensions, where the
/// arc is just a radius as a function of angle.
fn arc_glow(i: u32, ro: vec3<f32>, rd: vec3<f32>, t_hit: f32, hit: bool) -> f32 {
    let a4 = g.arcs[i * 2u];
    let b4 = g.arcs[i * 2u + 1u];
    let alpha = b4.w;
    if alpha <= 0.002 {
        return 0.0;
    }
    let a = a4.xyz;
    let b = b4.xyz;
    let drawn = a4.w;

    let axis = cross(a, b);
    let axis_len = length(axis);
    if axis_len < 1e-4 {
        return 0.0; // the two stations are antipodal or coincident
    }
    let nrm = axis / axis_len;

    // Grazing the plane edge-on makes `t` explode and the distance below
    // meaningless; those pixels are exactly the ones where the arc is thinner
    // than a pixel anyway.
    let denom = dot(rd, nrm);
    if abs(denom) < 0.02 {
        return 0.0;
    }
    let t = -dot(ro, nrm) / denom;
    if t <= 0.0 || (hit && t > t_hit) {
        return 0.0; // behind the camera, or behind the globe
    }

    let q = ro + rd * t;
    let e2 = normalize(cross(nrm, a));
    let ang = atan2(dot(q, e2), dot(q, a));
    let omega = acos(clamp(dot(a, b), -1.0, 1.0));
    let s = ang / max(omega, 1e-4);
    if s < 0.0 || s > drawn {
        return 0.0;
    }

    // Longer paths bow higher, exactly as the 3D view's QSO arcs do: a short
    // hop that stood as tall as a transatlantic one would read as a tower.
    let bulge = 0.14 * omega;
    let radius = 1.0 + bulge * sin(PI * s);
    // In-plane distance, converted to the ray's true distance of closest
    // approach — without this an arc seen nearly edge-on paints as a wide
    // smear instead of thinning away.
    let d = abs(length(q) - radius) * abs(denom);

    let w = g.misc.y;
    var glow = exp(-(d * d) / (w * w));
    // A brighter head while the arc is still drawing in, so the eye follows
    // the contact being made rather than watching a line appear.
    glow = glow * (1.0 + 1.6 * exp(-pow((drawn - s) / 0.045, 2.0)));
    // Ends taper instead of stopping dead at the station.
    let taper = smoothstep(0.0, 0.05, s) * smoothstep(0.0, 0.05, drawn - s);
    return glow * alpha * taper;
}

/// A station's own marker, drawn on the surface at both ends of every live arc.
fn station_glow(i: u32, n: vec3<f32>) -> f32 {
    let a4 = g.arcs[i * 2u];
    let b4 = g.arcs[i * 2u + 1u];
    let alpha = b4.w;
    if alpha <= 0.002 {
        return 0.0;
    }
    let r = g.misc.z;
    // The far end only lights up once the arc has reached it.
    let far = select(0.0, 1.0, a4.w > 0.98);
    let da = 1.0 - smoothstep(0.0, r, acos(clamp(dot(n, a4.xyz), -1.0, 1.0)));
    let db = 1.0 - smoothstep(0.0, r, acos(clamp(dot(n, b4.xyz), -1.0, 1.0)));
    return (da + db * far) * alpha;
}

@fragment
fn fs(in: VsOut) -> @location(0) vec4<f32> {
    let aspect = g.misc.x;
    let focal = g.camera.w;
    let ndc = vec2((in.uv.x * 2.0 - 1.0) * aspect, 1.0 - in.uv.y * 2.0);
    let rd_world = normalize(vec3(ndc.x / focal, ndc.y / focal, -1.0));

    let ro = to_local(g.camera.xyz);
    let rd = to_local(rd_world);
    let sun = to_local(g.sun.xyz);

    // Unit sphere at the origin. `disc` is negative for rays that miss, and the
    // branch is deferred so the derivatives below stay defined everywhere —
    // a normal computed only where the sphere is hit gives the limb pixels a
    // garbage mip level, which shows up as a bright fringe all the way round.
    let b = dot(ro, rd);
    let c = dot(ro, ro) - 1.0;
    let disc = b * b - c;
    let hit = disc > 0.0;
    let t_hit = -b - sqrt(max(disc, 0.0));
    let n = normalize(ro + rd * max(t_hit, 0.0));

    let uv = equirect(n);
    let duvdx = equirect_grad(n, dpdx(n));
    let duvdy = equirect_grad(n, dpdy(n));
    let land = textureSampleGrad(land_tex, samp, uv, duvdx, duvdy).r;
    let border = textureSampleGrad(border_tex, samp, uv, duvdx, duvdy).r;

    // Coastline, as the difference between the mask here and a slightly
    // blurrier copy of it. Cheaper than a gradient of four taps and, at the
    // softness this whole scene is drawn to, indistinguishable from one.
    let land_soft = textureSampleGrad(land_tex, samp, uv, duvdx * 3.0, duvdy * 3.0).r;
    let coast = clamp(abs(land - land_soft) * 3.0, 0.0, 1.0);

    // A deliberately wide terminator. A sharp one is correct and looks wrong
    // here: this is a backdrop, and the eye reads a hard shadow edge as a
    // second object lying across the globe.
    let day = smoothstep(-0.35, 0.45, dot(n, sun));

    var surface = mix(OCEAN_DAY, LAND_DAY, land);
    surface = mix(surface * 0.10, surface, day);
    // Coast and borders carry their own light on the night side, which is what
    // keeps the dark half from being a featureless hole.
    surface += COAST * coast * (0.30 + 0.55 * (1.0 - day));
    surface += ATMO * border * 0.10 * (0.35 + 0.65 * (1.0 - day));

    // Stations and their arcs.
    var arcs = 0.0;
    var dots = 0.0;
    for (var i = 0u; i < MAX_ARCS; i = i + 1u) {
        arcs += arc_glow(i, ro, rd, t_hit, hit);
        dots += station_glow(i, n);
    }
    if hit {
        surface += ATMO * clamp(dots, 0.0, 2.0) * 0.9;
    }

    // Atmosphere: a rim on the disc, and a halo outside it that falls off with
    // the ray's closest approach to the sphere.
    let facing = clamp(dot(n, -rd), 0.0, 1.0);
    let rim = pow(1.0 - facing, 3.5);
    let miss_dist = max(sqrt(max(dot(ro, ro) - b * b, 0.0)) - 1.0, 0.0);
    let halo = exp(-miss_dist * 16.0);

    var colour = vec3(0.0);
    var alpha = 0.0;
    if hit {
        colour = surface + ATMO * rim * 0.55 * (0.35 + 0.65 * day);
        alpha = 1.0;
    } else {
        colour = ATMO * halo * 0.30;
        alpha = halo * 0.55;
    }
    // The arcs ride over both, so a contact crossing the limb stays one line.
    let arc_col = mix(ATMO, ARC_HOT, g.sun.w);
    let arc_i = clamp(arcs, 0.0, 2.2);
    colour += arc_col * arc_i;
    alpha = clamp(alpha + arc_i * 0.8, 0.0, 1.0);

    return vec4(colour, alpha);
}
