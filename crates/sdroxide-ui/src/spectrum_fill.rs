//! The colour under the spectrum trace: the area between the line and the
//! bottom of the spectrum, shaded fixed, fading or half of each.
//!
//! The shading is a function of **height in the spectrum area**, not of each
//! column. A first version faded every column from the line down to the
//! bottom, which made the fill follow the noise floor's ragged edge and
//! painted it as vertical stripes; measured on the area, neighbouring columns
//! carry the same colour at the same height and the fill is smooth.
//!
//! Kept apart from the panadapter's draw so the colour rule is one function the
//! panadapter and `examples/spectrum_fill_probe.rs` both call — the probe draws
//! the fill through this code, not through a copy of it.

use eframe::egui::epaint::{Mesh, Vertex, WHITE_UV};
use eframe::egui::{Color32, Pos2};
use sdroxide_types::SpectrumFillStyle;

/// Opacity where the fill is at full strength.
const FULL_ALPHA: f32 = 170.0;
/// Opacity of a fixed fill: enough to give the band its mass, little enough
/// that the grid still shows through it.
const FIXED_ALPHA: f32 = 110.0;

/// Opacity at height `y` in a spectrum area running from `top` to `bottom`.
///
/// Linear in `y` wherever it fades, which is what lets the mesh carry it
/// exactly: a triangle interpolates its vertex colours linearly, so as long as
/// no triangle straddles the point where a fade starts, the GPU draws exactly
/// this function.
pub fn alpha_at(style: SpectrumFillStyle, y: f32, top: f32, bottom: f32) -> f32 {
    let h = (bottom - top).max(1.0);
    // 0 at the bottom of the area, 1 at its top.
    let up = ((bottom - y) / h).clamp(0.0, 1.0);
    match style {
        SpectrumFillStyle::Fixed => FIXED_ALPHA,
        SpectrumFillStyle::Gradient => FULL_ALPHA * (1.0 - up),
        SpectrumFillStyle::Mixed => {
            if up <= 0.5 {
                FULL_ALPHA
            } else {
                FULL_ALPHA * (1.0 - up) * 2.0
            }
        }
    }
}

/// Where the fade starts: a column crossing this height gets a vertex on it,
/// so no triangle spans the bend in [`alpha_at`].
fn knee(style: SpectrumFillStyle, top: f32, bottom: f32) -> f32 {
    match style {
        SpectrumFillStyle::Mixed => (top + bottom) / 2.0,
        // Fixed is flat and Gradient a single slope: no bend to cut at.
        SpectrumFillStyle::Fixed | SpectrumFillStyle::Gradient => bottom,
    }
}

fn colour(rgb: [u8; 3], alpha: f32) -> Color32 {
    let [r, g, b] = rgb;
    Color32::from_rgba_unmultiplied(r, g, b, alpha.round().clamp(0.0, 255.0) as u8)
}

/// The fill under a trace in a spectrum area from `top` to `bottom`: per trace
/// point, a vertex on the line, one on the knee and one on the bottom, joined
/// to the next point's by two quads.
pub fn fill_mesh(
    points: &[Pos2],
    top: f32,
    bottom: f32,
    rgb: [u8; 3],
    style: SpectrumFillStyle,
) -> Mesh {
    let mut mesh = Mesh::default();
    if points.len() < 2 {
        return mesh;
    }
    let knee = knee(style, top, bottom);
    mesh.vertices.reserve(points.len() * 3);
    mesh.indices.reserve((points.len() - 1) * 12);
    for p in points {
        let line = p.y.clamp(top, bottom);
        // A trace below the knee has nothing above it to fade: the knee
        // vertex sits on the line and its upper quad is empty.
        let mid = line.max(knee);
        for y in [line, mid, bottom] {
            mesh.vertices.push(Vertex {
                pos: Pos2::new(p.x, y),
                uv: WHITE_UV,
                color: colour(rgb, alpha_at(style, y, top, bottom)),
            });
        }
    }
    for i in 0..points.len() as u32 - 1 {
        let (a, b) = (3 * i, 3 * i + 3);
        for k in 0..2 {
            let (a0, a1, b0, b1) = (a + k, a + k + 1, b + k, b + k + 1);
            mesh.indices.extend_from_slice(&[a0, a1, b0, b0, a1, b1]);
        }
    }
    mesh
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: [SpectrumFillStyle; 3] = SpectrumFillStyle::ALL;

    #[test]
    fn the_fill_runs_from_the_line_to_the_bottom() {
        let pts = [Pos2::new(0.0, 10.0), Pos2::new(1.0, 80.0), Pos2::new(2.0, 25.0)];
        for style in ALL {
            let mesh = fill_mesh(&pts, 0.0, 100.0, [120, 220, 255], style);
            assert_eq!(mesh.vertices.len(), 9);
            assert_eq!(mesh.indices.len(), 24);
            for (i, p) in pts.iter().enumerate() {
                assert_eq!(mesh.vertices[3 * i].pos, *p, "{style:?}: the fill left the line");
                assert_eq!(
                    mesh.vertices[3 * i + 2].pos.y,
                    100.0,
                    "{style:?}: the fill stopped short of the bottom"
                );
            }
        }
    }

    /// The whole point of measuring on the area: two columns whose lines sit
    /// at different heights carry the same colour at the same height, so the
    /// noise floor's ragged edge does not become stripes.
    #[test]
    fn neighbouring_columns_match_at_the_same_height() {
        let pts = [Pos2::new(0.0, 60.0), Pos2::new(1.0, 70.0)];
        for style in ALL {
            let mesh = fill_mesh(&pts, 0.0, 100.0, [0, 255, 0], style);
            // The bottom vertices of the two columns.
            assert_eq!(
                mesh.vertices[2].color, mesh.vertices[5].color,
                "{style:?}: two columns differ at the bottom — the stripes are back"
            );
        }
    }

    #[test]
    fn each_style_shades_as_named() {
        let (top, bottom) = (0.0, 100.0);
        let a = |s, y| alpha_at(s, y, top, bottom);
        use SpectrumFillStyle::*;
        assert_eq!(a(Fixed, 5.0), a(Fixed, 95.0), "fixed changed with height");
        assert!(a(Gradient, 100.0) > a(Gradient, 50.0), "gradient is not strongest at the bottom");
        assert!(a(Gradient, 50.0) > a(Gradient, 10.0));
        assert_eq!(a(Gradient, 0.0), 0.0, "gradient is not clear at the top");
        assert_eq!(a(Mixed, 100.0), a(Mixed, 50.0), "mixed fades in its lower half");
        assert!(a(Mixed, 25.0) < a(Mixed, 50.0), "mixed does not fade in its upper half");
        assert_eq!(a(Mixed, 0.0), 0.0, "mixed is not clear at the top");
    }

    /// Mixed bends at half height; a column crossing it must get a vertex
    /// there, or the GPU would interpolate straight across the bend.
    #[test]
    fn a_mixed_column_is_cut_at_the_knee() {
        let mesh = fill_mesh(
            &[Pos2::new(0.0, 10.0), Pos2::new(1.0, 10.0)],
            0.0,
            100.0,
            [1, 2, 3],
            SpectrumFillStyle::Mixed,
        );
        assert_eq!(mesh.vertices[1].pos.y, 50.0, "no vertex on the knee");
    }

    #[test]
    fn a_trace_too_short_to_span_anything_draws_nothing() {
        for style in ALL {
            assert!(fill_mesh(&[Pos2::new(0.0, 1.0)], 0.0, 5.0, [1, 2, 3], style).is_empty());
        }
    }
}
