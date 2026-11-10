//! Draw the spectrum trace with and without the colour under it, so the fill
//! can be looked at rather than only asserted.
//!
//! The fill goes through the *real* `spectrum_fill::fill_mesh`, rasterised
//! here triangle by triangle with its own vertex colours, over the panadapter's
//! default background gradient and under the cyan line. Panels, top to bottom:
//! no fill, then the default cyan fill **Fixed**, **Gradient** and **Mixed**.
//!
//! ```sh
//! cargo run -p sdroxide-ui --release --example spectrum_fill_probe
//! # writes target/spectrum_fill_probe.png
//! ```
//!
//! The band is synthetic: it shows what the fill does to a trace shaped like
//! a busy band, not what this radio hears.

use eframe::egui::epaint::Mesh;
use eframe::egui::{Color32, Pos2};
use image::{Rgb, RgbImage};
use sdroxide_types::SpectrumFillStyle;
use sdroxide_ui::spectrum_fill::fill_mesh;

const W: u32 = 1000;
const H: u32 = 240;
const GAP: u32 = 12;
const LINE: [f32; 3] = [120.0, 220.0, 255.0];
const CYAN: [u8; 3] = [120, 220, 255];

fn hash(a: u32) -> f32 {
    let mut h = a.wrapping_mul(0x9E37_79B9);
    h ^= h >> 15;
    h = h.wrapping_mul(0xC2B2_AE35);
    h ^= h >> 13;
    (h & 0xFFFF) as f32 / 65535.0
}

/// A busy band: a floor with speckle, a few broadcast humps and narrow carriers.
fn trace() -> Vec<Pos2> {
    let peaks: [(f32, f32, f32); 9] = [
        (0.06, 0.010, 0.45),
        (0.24, 0.006, 0.55),
        (0.41, 0.012, 0.30),
        (0.49, 0.004, 0.35),
        (0.56, 0.010, 0.25),
        (0.69, 0.008, 0.40),
        (0.92, 0.0015, 0.60),
        (0.96, 0.009, 0.32),
        (0.995, 0.006, 0.42),
    ];
    (0..W)
        .map(|x| {
            let f = x as f32 / W as f32;
            // A ragged floor, as a real band has: the edge the first fill
            // turned into stripes.
            let mut v = 0.12 + 0.07 * hash(x) + 0.04 * hash(x * 7 + 3);
            for (c, w, a) in peaks {
                let d = (f - c) / w;
                v += a * (-0.5 * d * d).exp() * (0.8 + 0.2 * hash(x * 13));
            }
            Pos2::new(x as f32, H as f32 - v.min(0.95) * H as f32)
        })
        .collect()
}

fn background(img: &mut RgbImage, y0: u32) {
    for y in 0..H {
        let t = y as f32 / (H - 1) as f32;
        let r = (64.0 * (1.0 - t)) as u8;
        for x in 0..W {
            // The grid: a line every 100 px and every 50 px down.
            let grid = x % 100 == 0 || y % 50 == 0;
            let px = if grid { Rgb([r.saturating_add(18), 18, 18]) } else { Rgb([r, 0, 0]) };
            img.put_pixel(x, y0 + y, px);
        }
    }
}

/// Blend a premultiplied colour over a pixel.
fn over(px: &mut Rgb<u8>, c: [f32; 4]) {
    let a = c[3] / 255.0;
    for i in 0..3 {
        px.0[i] = (c[i] + px.0[i] as f32 * (1.0 - a)).round().clamp(0.0, 255.0) as u8;
    }
}

fn premul(c: Color32) -> [f32; 4] {
    [c.r() as f32, c.g() as f32, c.b() as f32, c.a() as f32]
}

/// Rasterise a mesh: every pixel centre inside a triangle takes the
/// barycentric mix of its vertex colours, as the GPU interpolates them.
fn draw_mesh(img: &mut RgbImage, y0: u32, mesh: &Mesh) {
    for tri in mesh.indices.chunks_exact(3) {
        let v = [
            &mesh.vertices[tri[0] as usize],
            &mesh.vertices[tri[1] as usize],
            &mesh.vertices[tri[2] as usize],
        ];
        let (p0, p1, p2) = (v[0].pos, v[1].pos, v[2].pos);
        let area = (p1.x - p0.x) * (p2.y - p0.y) - (p2.x - p0.x) * (p1.y - p0.y);
        if area.abs() < 1e-6 {
            continue;
        }
        let xmin = p0.x.min(p1.x).min(p2.x).floor().max(0.0) as u32;
        let xmax = p0.x.max(p1.x).max(p2.x).ceil().min(W as f32 - 1.0) as u32;
        let ymin = p0.y.min(p1.y).min(p2.y).floor().max(0.0) as u32;
        let ymax = p0.y.max(p1.y).max(p2.y).ceil().min(H as f32 - 1.0) as u32;
        let c = [premul(v[0].color), premul(v[1].color), premul(v[2].color)];
        for y in ymin..=ymax {
            for x in xmin..=xmax {
                let (px, py) = (x as f32 + 0.5, y as f32 + 0.5);
                let w0 = ((p1.x - px) * (p2.y - py) - (p2.x - px) * (p1.y - py)) / area;
                let w1 = ((p2.x - px) * (p0.y - py) - (p0.x - px) * (p2.y - py)) / area;
                let w2 = 1.0 - w0 - w1;
                if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                    continue;
                }
                let mut mix = [0.0; 4];
                for i in 0..4 {
                    mix[i] = w0 * c[0][i] + w1 * c[1][i] + w2 * c[2][i];
                }
                over(img.get_pixel_mut(x, y0 + y), mix);
            }
        }
    }
}

fn draw_line(img: &mut RgbImage, y0: u32, pts: &[Pos2]) {
    for pair in pts.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        let steps = ((b.y - a.y).abs().ceil() as u32).max(1);
        for s in 0..=steps {
            let t = s as f32 / steps as f32;
            let x = (a.x + (b.x - a.x) * t).round() as u32;
            let y = (a.y + (b.y - a.y) * t).round().clamp(0.0, H as f32 - 1.0) as u32;
            if x < W {
                over(img.get_pixel_mut(x, y0 + y), [LINE[0], LINE[1], LINE[2], 255.0]);
            }
        }
    }
}

fn main() {
    let pts = trace();
    let panels: [Option<SpectrumFillStyle>; 4] = [
        None,
        Some(SpectrumFillStyle::Fixed),
        Some(SpectrumFillStyle::Gradient),
        Some(SpectrumFillStyle::Mixed),
    ];
    let mut img = RgbImage::from_pixel(W, panels.len() as u32 * (H + GAP), Rgb([40, 40, 40]));
    for (i, panel) in panels.iter().enumerate() {
        let y0 = i as u32 * (H + GAP);
        background(&mut img, y0);
        if let Some(style) = panel {
            draw_mesh(&mut img, y0, &fill_mesh(&pts, 0.0, H as f32, CYAN, *style));
        }
        draw_line(&mut img, y0, &pts);
    }
    let out = concat!(env!("CARGO_MANIFEST_DIR"), "/../../target/spectrum_fill_probe.png");
    img.save(out).expect("write the probe image");
    println!("wrote {out}");
}
