//! A small software rasterizer for tests: turn one egui frame into a PNG a
//! person can open, with no GPU and no window.
//!
//! It draws exactly what egui hands a renderer — the tessellated meshes and
//! the textures they sample — so the picture is the real layout and the real
//! paint, not a mock-up of it. Blending is egui's own: premultiplied colour in
//! gamma space, `src + dst · (1 − src.a)`. Only for looking at things; nothing
//! ships through it.

use std::collections::HashMap;

use eframe::egui::{self, ClippedPrimitive, Color32, ColorImage, TextureId, epaint::Primitive};

/// The textures egui has handed out so far. A frame's `textures_delta` only
/// carries what changed, so they are kept across every frame that is run.
#[derive(Default)]
pub struct Textures(HashMap<TextureId, ColorImage>);

impl Textures {
    pub fn apply(&mut self, delta: &egui::TexturesDelta) {
        for (id, deltas) in &delta.set {
            for delta in deltas.iter() {
                self.apply_one(*id, delta);
            }
        }
    }

    fn apply_one(&mut self, id: TextureId, delta: &egui::epaint::ImageDelta) {
        {
            let egui::ImageData::Color(img) = &delta.image;
            match delta.pos {
                None => {
                    self.0.insert(id, (**img).clone());
                }
                Some([x0, y0]) => {
                    let t = self.0.get_mut(&id).expect("a partial update follows a full one");
                    for y in 0..img.size[1] {
                        for x in 0..img.size[0] {
                            t.pixels[(y0 + y) * t.size[0] + x0 + x] =
                                img.pixels[y * img.size[0] + x];
                        }
                    }
                }
            }
        }
    }
}

/// Draw `full`'s shapes into an RGBA image `size` points big, on the given
/// background. `textures` must already hold this frame's delta.
pub fn rasterize(
    ctx: &egui::Context,
    full: egui::FullOutput,
    textures: &Textures,
    size: egui::Vec2,
    background: Color32,
) -> image::RgbaImage {
    let ppp = full.pixels_per_point;
    let textures = &textures.0;
    let prims: Vec<ClippedPrimitive> = ctx.tessellate(full.shapes, ppp);

    let w = (size.x * ppp).round() as usize;
    let h = (size.y * ppp).round() as usize;
    let bg = background.to_array().map(|c| c as f32 / 255.0);
    let mut px: Vec<[f32; 4]> = vec![bg; w * h];

    for cp in prims {
        let Primitive::Mesh(mesh) = cp.primitive else { continue };
        let Some(tex) = textures.get(&mesh.texture_id) else { continue };
        let clip = cp.clip_rect;
        let (cx0, cy0) = ((clip.min.x * ppp).max(0.0), (clip.min.y * ppp).max(0.0));
        let (cx1, cy1) = ((clip.max.x * ppp).min(w as f32), (clip.max.y * ppp).min(h as f32));
        for tri in mesh.indices.chunks_exact(3) {
            let v = [
                &mesh.vertices[tri[0] as usize],
                &mesh.vertices[tri[1] as usize],
                &mesh.vertices[tri[2] as usize],
            ];
            let p = v.map(|v| (v.pos.x * ppp, v.pos.y * ppp));
            let area =
                (p[1].0 - p[0].0) * (p[2].1 - p[0].1) - (p[2].0 - p[0].0) * (p[1].1 - p[0].1);
            if area.abs() < 1e-6 {
                continue;
            }
            let minx = p.iter().map(|q| q.0).fold(f32::MAX, f32::min).max(cx0).floor() as i32;
            let maxx = p.iter().map(|q| q.0).fold(f32::MIN, f32::max).min(cx1).ceil() as i32;
            let miny = p.iter().map(|q| q.1).fold(f32::MAX, f32::min).max(cy0).floor() as i32;
            let maxy = p.iter().map(|q| q.1).fold(f32::MIN, f32::max).min(cy1).ceil() as i32;
            for y in miny.max(0)..maxy.min(h as i32) {
                for x in minx.max(0)..maxx.min(w as i32) {
                    let (sx, sy) = (x as f32 + 0.5, y as f32 + 0.5);
                    if sx < cx0 || sx >= cx1 || sy < cy0 || sy >= cy1 {
                        continue;
                    }
                    let edge = |a: (f32, f32), b: (f32, f32)| {
                        (b.0 - a.0) * (sy - a.1) - (b.1 - a.1) * (sx - a.0)
                    };
                    let w0 = edge(p[1], p[2]) / area;
                    let w1 = edge(p[2], p[0]) / area;
                    let w2 = edge(p[0], p[1]) / area;
                    if w0 < 0.0 || w1 < 0.0 || w2 < 0.0 {
                        continue;
                    }
                    let col = |i: usize| v[i].color.to_array().map(|c| c as f32 / 255.0);
                    let (c0, c1, c2) = (col(0), col(1), col(2));
                    let u = w0 * v[0].uv.x + w1 * v[1].uv.x + w2 * v[2].uv.x;
                    let vv = w0 * v[0].uv.y + w1 * v[1].uv.y + w2 * v[2].uv.y;
                    let t = sample(tex, u, vv);
                    let mut src = [0.0f32; 4];
                    for k in 0..4 {
                        src[k] = (w0 * c0[k] + w1 * c1[k] + w2 * c2[k]) * t[k];
                    }
                    let d = &mut px[y as usize * w + x as usize];
                    for k in 0..4 {
                        d[k] = src[k] + d[k] * (1.0 - src[3]);
                    }
                }
            }
        }
    }

    let mut out = image::RgbaImage::new(w as u32, h as u32);
    for (i, p) in px.iter().enumerate() {
        let a = p[3].clamp(1e-6, 1.0);
        let rgba = [
            (p[0] / a * 255.0).round().clamp(0.0, 255.0) as u8,
            (p[1] / a * 255.0).round().clamp(0.0, 255.0) as u8,
            (p[2] / a * 255.0).round().clamp(0.0, 255.0) as u8,
            255,
        ];
        out.put_pixel((i % w) as u32, (i / w) as u32, image::Rgba(rgba));
    }
    out
}

/// Bilinear sample of a premultiplied texture at `(u, v)` in 0..1.
fn sample(t: &ColorImage, u: f32, v: f32) -> [f32; 4] {
    let (tw, th) = (t.size[0], t.size[1]);
    let x = (u * tw as f32 - 0.5).clamp(0.0, (tw - 1) as f32);
    let y = (v * th as f32 - 0.5).clamp(0.0, (th - 1) as f32);
    let (x0, y0) = (x.floor() as usize, y.floor() as usize);
    let (x1, y1) = ((x0 + 1).min(tw - 1), (y0 + 1).min(th - 1));
    let (fx, fy) = (x - x0 as f32, y - y0 as f32);
    let at = |x: usize, y: usize| t.pixels[y * tw + x].to_array().map(|c| c as f32 / 255.0);
    let (a, b, c, d) = (at(x0, y0), at(x1, y0), at(x0, y1), at(x1, y1));
    let mut o = [0.0; 4];
    for k in 0..4 {
        o[k] = (a[k] * (1.0 - fx) + b[k] * fx) * (1.0 - fy) + (c[k] * (1.0 - fx) + d[k] * fx) * fy;
    }
    o
}
