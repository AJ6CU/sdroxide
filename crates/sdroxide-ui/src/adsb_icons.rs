//! The map's three symbols: an aircraft, an airport and a VOR.
//!
//! Each is a plain silhouette with nothing behind it: no disc, no tile, no
//! halo. The outlines were traced from the source pictures into triangles
//! (see `trace.rs`, which writes `data.rs`), so they stay sharp at any size
//! and ship no picture. The fill is a mesh; a hairline along the outline, drawn
//! by egui's anti-aliased path, takes the stair-steps off the edge.
//!
//! Every symbol is described in units (±1 about its centre, nose or top up)
//! and scaled by the caller's size; an aircraft is then turned to its track.

mod data;
#[cfg(test)]
mod trace;

use eframe::egui::{Color32, Mesh, Painter, Pos2, Shape, Stroke, pos2};

/// One symbol: its corner points, the triangles filling it and how many corner
/// points each closed outline has.
struct Icon {
    verts: &'static [[f32; 2]],
    tris: &'static [u32],
    loops: &'static [u32],
}

const AIRCRAFT: Icon =
    Icon { verts: data::AIRCRAFT_VERTS, tris: data::AIRCRAFT_TRIS, loops: data::AIRCRAFT_LOOPS };
const AIRPORT: Icon =
    Icon { verts: data::AIRPORT_VERTS, tris: data::AIRPORT_TRIS, loops: data::AIRPORT_LOOPS };
const VOR: Icon = Icon { verts: data::VOR_VERTS, tris: data::VOR_TRIS, loops: data::VOR_LOOPS };

/// Draw `icon` centred on `c`, `s` pixels from its centre to its widest edge,
/// turned `turn` radians clockwise.
fn draw(p: &Painter, icon: &Icon, c: Pos2, s: f32, turn: f32, ink: Color32) {
    let (sin, cos) = turn.sin_cos();
    let pts: Vec<Pos2> = icon
        .verts
        .iter()
        .map(|&[x, y]| pos2(c.x + (x * cos - y * sin) * s, c.y + (x * sin + y * cos) * s))
        .collect();
    let mut mesh = Mesh::default();
    for &pt in &pts {
        mesh.colored_vertex(pt, ink);
    }
    for t in icon.tris.chunks_exact(3) {
        mesh.add_triangle(t[0], t[1], t[2]);
    }
    p.add(Shape::mesh(mesh));
    let hairline = Stroke::new(0.8, ink);
    let mut at = 0;
    for &n in icon.loops {
        let end = at + n as usize;
        p.add(Shape::closed_line(pts[at..end].to_vec(), hairline));
        at = end;
    }
}

/// An airport: an aeroplane taking off over a runway. `s` is half the symbol's
/// width in pixels; a medium airport is drawn smaller and fainter by the caller.
pub fn airport(p: &Painter, c: Pos2, s: f32, ink: Color32) {
    draw(p, &AIRPORT, c, s, 0.0, ink);
}

/// An aircraft: an airliner seen from above, turned to its track (nose up when
/// the track is not known). On the ground it is drawn darker, which is what
/// tells a taxiing aircraft from one flying over the same runway.
pub fn aircraft(
    p: &Painter,
    c: Pos2,
    s: f32,
    track_deg: Option<f32>,
    ink: Color32,
    on_ground: bool,
) {
    let ink = if on_ground { dimmed(ink) } else { ink };
    draw(p, &AIRCRAFT, c, s, track_deg.unwrap_or(0.0).to_radians(), ink);
}

/// A VOR: a mast with radio waves on each side.
pub fn vor(p: &Painter, c: Pos2, s: f32, ink: Color32) {
    draw(p, &VOR, c, s, 0.0, ink);
}

/// The same colour, a little darker and fainter.
fn dimmed(c: Color32) -> Color32 {
    let [r, g, b, a] = c.to_srgba_unmultiplied();
    let k = |v: u8| (f32::from(v) * 0.6) as u8;
    Color32::from_rgba_unmultiplied(k(r), k(g), k(b), a)
}
