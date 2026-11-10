//! CPU mesh builders. All three static meshes are built once at init and live
//! in one vertex buffer format so the pipelines stay simple.

/// Shared vertex layout. `pos` is a position for the sphere and a parameter
/// triple for the cone (see [`cone`]); `uv` is texture/shading coordinates.
#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
}

pub const SPHERE_LON: u32 = 64;
pub const SPHERE_LAT: u32 = 32;

/// Unit sphere in the ECEF convention the land mask uses: +X at (0°N, 0°E),
/// +Z at the north pole, `u = (lon+180)/360`, `v = (90−lat)/180`.
///
/// The seam column is duplicated so `u` never wraps within a triangle — without
/// that, the texture derivative at the antimeridian spans the whole map and the
/// mip/anisotropic selection produces a visible stripe.
pub fn sphere() -> (Vec<Vertex>, Vec<u32>) {
    let mut verts = Vec::with_capacity(((SPHERE_LON + 1) * (SPHERE_LAT + 1)) as usize);
    for j in 0..=SPHERE_LAT {
        let v = j as f32 / SPHERE_LAT as f32;
        let lat = (90.0 - 180.0 * v as f64).to_radians();
        for i in 0..=SPHERE_LON {
            let u = i as f32 / SPHERE_LON as f32;
            let lon = (-180.0 + 360.0 * u as f64).to_radians();
            verts.push(Vertex {
                pos: [
                    (lat.cos() * lon.cos()) as f32,
                    (lat.cos() * lon.sin()) as f32,
                    lat.sin() as f32,
                ],
                uv: [u, v],
            });
        }
    }

    let row = SPHERE_LON + 1;
    let mut idx = Vec::with_capacity((SPHERE_LON * SPHERE_LAT * 6) as usize);
    for j in 0..SPHERE_LAT {
        for i in 0..SPHERE_LON {
            let a = j * row + i;
            let (b, c, d) = (a + 1, a + row, a + row + 1);
            idx.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }
    (verts, idx)
}

pub const CONE_RADIAL: u32 = 48;
/// Steps along the cone's slant, for a smooth length-wise alpha ramp.
const CONE_AXIAL: u32 = 10;
/// Steps across the spherical leading-edge cap.
const CONE_CAP: u32 = 6;

/// A CME cone in *parametric* form: the half-angle is a shader uniform, not
/// baked into the geometry, so one static mesh serves every event.
///
/// `pos` carries `[azimuth, t, kind]`. With apex at the origin and axis +Z the
/// vertex shader reconstructs, for half-angle θ:
///
/// * `kind = 0` — the ruled lateral surface, `t · dir(θ, φ)` for `t ∈ [0, 1]`
/// * `kind = 1` — the leading edge, `dir(t·θ, φ)`, a spherical cap
///
/// The cap is spherical rather than flat because a CME expands radially: every
/// point of the front is the same distance from the Sun, which is exactly what
/// makes the arrival-time estimate readable off the picture.
pub fn cone() -> (Vec<Vertex>, Vec<u32>) {
    let mut verts = Vec::new();
    let mut idx = Vec::new();
    let row = CONE_RADIAL + 1;

    let strip = |steps: u32, kind: f32, verts: &mut Vec<Vertex>, idx: &mut Vec<u32>| {
        let base = verts.len() as u32;
        for j in 0..=steps {
            let t = j as f32 / steps as f32;
            for i in 0..=CONE_RADIAL {
                let phi = i as f32 / CONE_RADIAL as f32 * std::f32::consts::TAU;
                verts.push(Vertex {
                    pos: [phi, t, kind],
                    // v runs 0 at the apex to 1 at the leading edge, for the
                    // fade; the cap is all at the front, so it pins to 1.
                    uv: [i as f32 / CONE_RADIAL as f32, if kind < 0.5 { t } else { 1.0 }],
                });
            }
        }
        for j in 0..steps {
            for i in 0..CONE_RADIAL {
                let a = base + j * row + i;
                let (b, c, d) = (a + 1, a + row, a + row + 1);
                idx.extend_from_slice(&[a, c, b, b, c, d]);
            }
        }
    };

    strip(CONE_AXIAL, 0.0, &mut verts, &mut idx);
    strip(CONE_CAP, 1.0, &mut verts, &mut idx);
    (verts, idx)
}

pub const RING_RADIAL: u32 = 128;
/// Steps across the ring. More than a flat sheet needs, but the radial
/// structure (the Cassini division, the ringlets) is shaded per fragment, so
/// this only has to be smooth enough that the inner and outer edges are round.
const RING_ACROSS: u32 = 6;

/// A planet's ring system, parametric like [`cone`]: `pos` carries
/// `[azimuth, t, 0]` and the vertex shader places `t = 0` at the inner edge and
/// `t = 1` at the outer one, using the radii the draw supplies. One mesh
/// therefore serves Saturn's sheet and Uranus's threads both.
pub fn ring() -> (Vec<Vertex>, Vec<u32>) {
    let row = RING_RADIAL + 1;
    let mut verts = Vec::with_capacity(((RING_ACROSS + 1) * row) as usize);
    for j in 0..=RING_ACROSS {
        let t = j as f32 / RING_ACROSS as f32;
        for i in 0..=RING_RADIAL {
            let u = i as f32 / RING_RADIAL as f32;
            verts.push(Vertex { pos: [u * std::f32::consts::TAU, t, 0.0], uv: [t, u] });
        }
    }
    let mut idx = Vec::with_capacity((RING_RADIAL * RING_ACROSS * 6) as usize);
    for j in 0..RING_ACROSS {
        for i in 0..RING_RADIAL {
            let a = j * row + i;
            let (b, c, d) = (a + 1, a + row, a + row + 1);
            idx.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }
    (verts, idx)
}

/// Steps along a comet tail. Generous, because the dust tail is a curve and a
/// coarse one would read as a bent stick.
pub const PLUME_AXIAL: u32 = 72;
/// Steps across it. The brightness profile across a tail is shaded per
/// fragment, so this only has to be fine enough that the silhouette is smooth.
const PLUME_ACROSS: u32 = 8;

/// A comet tail, parametric like [`cone`]: `pos` carries `[s, t, 0]` with
/// `s ∈ [−1, 1]` across the ribbon and `t ∈ [0, 1]` from the nucleus to the tip.
///
/// The vertex shader turns that into a screen-facing ribbon — expanded
/// perpendicular to both the tail's axis and the eye, so a tail seen end-on is
/// still a tail rather than an invisible edge. That is also what makes one mesh
/// serve a 40 Gm ion tail and a 4 Gm dust one: length, width and curvature all
/// arrive as uniforms.
pub fn plume() -> (Vec<Vertex>, Vec<u32>) {
    let row = PLUME_ACROSS + 1;
    let mut verts = Vec::with_capacity(((PLUME_AXIAL + 1) * row) as usize);
    for j in 0..=PLUME_AXIAL {
        let t = j as f32 / PLUME_AXIAL as f32;
        for i in 0..=PLUME_ACROSS {
            let s = i as f32 / PLUME_ACROSS as f32 * 2.0 - 1.0;
            verts.push(Vertex { pos: [s, t, 0.0], uv: [s, t] });
        }
    }
    let mut idx = Vec::with_capacity((PLUME_AXIAL * PLUME_ACROSS * 6) as usize);
    for j in 0..PLUME_AXIAL {
        for i in 0..PLUME_ACROSS {
            let a = j * row + i;
            let (b, c, d) = (a + 1, a + row, a + row + 1);
            idx.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }
    (verts, idx)
}

/// Corner offsets of a quad, `[-1, 1]` in both axes, as two triangles.
///
/// Instanced pipelines (lines, sprites) expand this in the vertex shader, which
/// is not optional for lines: wgpu's line topology is always exactly 1 px wide,
/// so screen-space quads are the only way to get the app's glowing strokes.
pub fn quad() -> Vec<[f32; 2]> {
    vec![[-1.0, -1.0], [1.0, -1.0], [-1.0, 1.0], [-1.0, 1.0], [1.0, -1.0], [1.0, 1.0]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sphere_vertices_are_unit_length_and_uv_matches_the_land_mask() {
        let (v, idx) = sphere();
        assert_eq!(v.len(), ((SPHERE_LON + 1) * (SPHERE_LAT + 1)) as usize);
        assert_eq!(idx.len(), (SPHERE_LON * SPHERE_LAT * 6) as usize);
        assert!(idx.iter().all(|i| (*i as usize) < v.len()));
        for vert in &v {
            let l = (vert.pos[0].powi(2) + vert.pos[1].powi(2) + vert.pos[2].powi(2)).sqrt();
            assert!((l - 1.0).abs() < 1e-5, "not on the unit sphere: {l}");
        }
        // v = 0 is the north pole, v = 1 the south — the mask's row order.
        assert!((v[0].pos[2] - 1.0).abs() < 1e-5 && v[0].uv[1] == 0.0);
        assert!((v[v.len() - 1].pos[2] + 1.0).abs() < 1e-5 && v[v.len() - 1].uv[1] == 1.0);
        // u = 0.5 is 0° longitude, i.e. the +X axis.
        let mid = &v[(SPHERE_LON / 2) as usize];
        assert!((mid.uv[0] - 0.5).abs() < 1e-6);
    }

    #[test]
    fn the_ring_spans_the_annulus_exactly_once() {
        let (v, idx) = ring();
        assert!(idx.iter().all(|i| (*i as usize) < v.len()));
        for vert in &v {
            assert!((0.0..=std::f32::consts::TAU + 1e-4).contains(&vert.pos[0]));
            assert!((0.0..=1.0).contains(&vert.pos[1]));
        }
        // The seam is duplicated (0 and τ both present), so the annulus closes
        // without a triangle spanning the whole ring.
        assert_eq!(v[0].pos[0], 0.0);
        assert!((v[RING_RADIAL as usize].pos[0] - std::f32::consts::TAU).abs() < 1e-5);
        // t = 0 is the inner edge everywhere in the first row.
        assert!(v[..=RING_RADIAL as usize].iter().all(|x| x.pos[1] == 0.0));
        assert_eq!(v[v.len() - 1].pos[1], 1.0);
    }

    #[test]
    fn cone_parameters_are_in_range() {
        let (v, idx) = cone();
        assert!(idx.iter().all(|i| (*i as usize) < v.len()));
        for vert in &v {
            assert!((0.0..=std::f32::consts::TAU + 1e-4).contains(&vert.pos[0]));
            assert!((0.0..=1.0).contains(&vert.pos[1]));
            assert!(vert.pos[2] == 0.0 || vert.pos[2] == 1.0);
        }
        // The lateral strip starts at the apex (t = 0) and the cap is pinned to
        // the leading edge for shading.
        assert_eq!(v[0].pos[1], 0.0);
        assert_eq!(v[v.len() - 1].uv[1], 1.0);
    }
}
