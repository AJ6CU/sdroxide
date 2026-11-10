//! Drawing the picture composer's document, and the hand-off of that document
//! to the engine.
//!
//! The **document itself** is a wire type ([`sdroxide_types::Composer`]): each
//! transmit preset carries its own, so the console, a browser tab attached to
//! the same radio and the next session all compose the same picture. This module
//! is the half that only a client can do — turning layers into pixels — and the
//! little state machine that keeps one operator's edits from fighting the
//! engine's echo of them.
//!
//! Rendering is pure Rust (`ab_glyph` + `image`) so the native app and the wasm
//! browser client compose identically.

use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use sdroxide_types::{Command, Composer, Face, ImagePresets, TextLayer};

use crate::sstv::{Ink, TextSink, crop_scale, draw_text_into, text_width};

/// A box in picture pixels: `(x0, y0, x1, y1)`, top-left first.
pub(crate) type Box2 = (i32, i32, i32, i32);

/// The bundled font a face is printed with.
///
/// Bundled rather than taken from the system on purpose: the browser client has
/// no access to the operator's fonts, so a system face would make the same
/// document compose into two different pictures depending on which screen
/// pressed SEND. All three ship in `assets/fonts` under the OFL.
pub(crate) fn font(face: Face) -> Option<FontRef<'static>> {
    const BOLD: &[u8] = include_bytes!("../assets/fonts/ChakraPetch-SemiBold.ttf");
    const PLAIN: &[u8] = include_bytes!("../assets/fonts/ChakraPetch-Regular.ttf");
    const MONO: &[u8] = include_bytes!("../assets/fonts/ShareTechMono-Regular.ttf");
    let raw = match face {
        Face::ChakraBold => BOLD,
        Face::Chakra => PLAIN,
        Face::Mono => MONO,
    };
    debug_assert_eq!(face.file(), match face {
        Face::ChakraBold => "ChakraPetch-SemiBold.ttf",
        Face::Chakra => "ChakraPetch-Regular.ttf",
        Face::Mono => "ShareTechMono-Regular.ttf",
    });
    FontRef::try_from_slice(raw).ok()
}

/// Compose the frame: `src` is the base picture in interleaved RGB, or `None`
/// for a flat dark frame so a layout can be tried before a picture is chosen.
pub(crate) fn render(
    doc: &Composer,
    w: u16,
    h: u16,
    src: Option<(&[u8], u16, u16)>,
) -> (Vec<u8>, u16, u16) {
    let mut img = match src {
        Some((rgb, sw, sh)) => crop_scale(rgb, sw, sh, w, h),
        None => vec![24u8; usize::from(w) * usize::from(h) * 3],
    };
    draw_onto(doc, &mut img, w, h);
    (img, w, h)
}

/// Draw every visible layer over an already sized frame.
pub(crate) fn draw_onto(doc: &Composer, img: &mut [u8], w: u16, h: u16) {
    let (uw, uh) = (usize::from(w), usize::from(h));
    for l in &doc.layers {
        if l.hidden || l.text.trim().is_empty() {
            continue;
        }
        if let Some(band) = &l.band {
            draw_band(img, uw, uh, l, band);
        }
        draw_layer(img, uw, uh, l);
    }
}

/// How much larger than the transmit frame a mosaic is composed.
///
/// The transmit path crops and scales to the mode's size anyway, so composing at
/// the frame's own size throws away the detail the four pictures still had at the
/// moment they were laid out. Twice the frame keeps it and costs four times the
/// pixels of one 320×256 picture — nothing against the engine's 1024 bound.
pub(crate) const MOSAIC_SCALE: u16 = 2;

/// The most pictures one mosaic holds — a 3 × 3 grid. Past that a cell is too
/// small to carry a chart on a 320-pixel frame, which is the smallest SSTV line
/// format.
pub(crate) const MOSAIC_MAX: usize = 9;

/// The grid a mosaic of `n` pictures is laid out in: `(columns, rows)`.
///
/// One fills the frame, two sit side by side, and the rest take the smallest
/// square-ish grid that holds them — 3 and 4 as 2 × 2 (3 leaves a cell empty),
/// 5 and 6 as 3 × 2, 7 to 9 as 3 × 3.
pub(crate) fn grid_for(n: usize) -> (u16, u16) {
    match n {
        0 | 1 => (1, 1),
        2 => (2, 1),
        3 | 4 => (2, 2),
        5 | 6 => (3, 2),
        _ => (3, 3),
    }
}

/// **A mosaic of received pictures**, composed to `(w, h)`.
///
/// The layout is a grid decided by how many pictures there are: one fills the
/// frame, two sit side by side, three and four make a 2×2 — the arrangement
/// issue #237 asked for, since the operator's case is "four pictures the other
/// station missed".
///
/// Each cell is **letterboxed, never stretched**: the picture is scaled until it
/// fits inside its cell and the difference is black, so a portrait shot is not
/// squeezed into a landscape half. The cells are centred, and the originals are
/// only read — nothing in the received store is touched.
pub(crate) fn mosaic(
    w: u16,
    h: u16,
    cells: &[(&[u8], u16, u16)],
) -> (Vec<u8>, u16, u16) {
    let mut img = vec![0u8; usize::from(w) * usize::from(h) * 3];
    let n = cells.len().min(MOSAIC_MAX);
    if n == 0 {
        return (img, w, h);
    }
    // **The grid follows the count**, so a picture fills the frame, two sit in a
    // row, four make a square, and five to nine use the smallest grid that holds
    // them. The cells are cut from the frame by integer arithmetic, so a row of
    // three is three equal cells and the next row starts where it ends.
    let (cols, rows) = grid_for(n);
    let (w, h) = (w, h);
    for (i, (rgb, sw, sh)) in cells.iter().take(n).enumerate() {
        let (i, sw, sh) = (i as u16, *sw, *sh);
        if sw == 0 || sh == 0 {
            continue;
        }
        let (cx, cy) = (i % cols, i / cols);
        let (x0, y0) = (w * cx / cols, h * cy / rows);
        let (x1, y1) = (w * (cx + 1) / cols, h * (cy + 1) / rows);
        let (cw, ch) = (x1.saturating_sub(x0), y1.saturating_sub(y0));
        if cw == 0 || ch == 0 {
            continue;
        }
        // Fit inside the cell, keeping the aspect ratio; the leftover is black.
        let scale = (cw as f32 / sw as f32).min(ch as f32 / sh as f32);
        let dw = ((sw as f32 * scale).round() as u16).max(1).min(cw);
        let dh = ((sh as f32 * scale).round() as u16).max(1).min(ch);
        let scaled = crop_scale_fit(rgb, sw, sh, dw, dh);
        let ox = x0 + (cw - dw) / 2;
        let oy = y0 + (ch - dh) / 2;
        blit(&mut img, w, h, &scaled, dw, dh, ox, oy);
    }
    (img, w, h)
}

/// Scale a picture to exactly `(dw, dh)` **without cropping it** — the whole
/// frame, letterboxed by the caller.
fn crop_scale_fit(src: &[u8], sw: u16, sh: u16, dw: u16, dh: u16) -> Vec<u8> {
    let Some(img) = image::RgbImage::from_raw(u32::from(sw), u32::from(sh), src.to_vec()) else {
        return vec![0u8; usize::from(dw) * usize::from(dh) * 3];
    };
    image::DynamicImage::ImageRgb8(img)
        .resize_exact(u32::from(dw), u32::from(dh), image::imageops::FilterType::Triangle)
        .to_rgb8()
        .into_raw()
}

/// Copy a `dw × dh` picture into the frame at `(ox, oy)`, clipping at the edges.
fn blit(dst: &mut [u8], w: u16, h: u16, src: &[u8], dw: u16, dh: u16, ox: u16, oy: u16) {
    let (w, h) = (usize::from(w), usize::from(h));
    for y in 0..usize::from(dh) {
        let ty = usize::from(oy) + y;
        if ty >= h {
            break;
        }
        for x in 0..usize::from(dw) {
            let tx = usize::from(ox) + x;
            if tx >= w {
                break;
            }
            let si = (y * usize::from(dw) + x) * 3;
            let di = (ty * w + tx) * 3;
            dst[di..di + 3].copy_from_slice(&src[si..si + 3]);
        }
    }
}

/// The box `doc.layers[i]` covers at frame size `(w, h)`, for hit-testing a drag
/// and for drawing the selection outline.
pub(crate) fn layer_box(doc: &Composer, i: usize, w: u16, h: u16) -> Option<Box2> {
    let l = doc.layers.get(i)?;
    let font = font(l.face)?;
    let (w, h) = (f32::from(w), f32::from(h));
    let scale = PxScale::from(f32::from(l.px.max(4)));
    let half = (text_width(&l.text, &font, scale) / 2.0).max(6.0);
    let half_h = (f32::from(l.px.max(4)) * 0.75).max(6.0);
    let (ax, ay) = (l.x * w, l.y * h);
    // A turned line's box is the same width and height, swapped.
    let (hx, hy) = if l.vertical { (half_h, half) } else { (half, half_h) };
    Some((
        (ax - hx).round() as i32,
        (ay - hy).round() as i32,
        (ax + hx).round() as i32,
        (ay + hy).round() as i32,
    ))
}

/// The layer under `(px, py)` — topmost first, so the line the operator sees on
/// top is the one a click picks up.
pub(crate) fn layer_at(doc: &Composer, px: f32, py: f32, w: u16, h: u16) -> Option<usize> {
    for i in (0..doc.layers.len()).rev() {
        if doc.layers[i].hidden {
            continue;
        }
        if let Some((x0, y0, x1, y1)) = layer_box(doc, i, w, h)
            && px >= x0 as f32
            && px <= x1 as f32
            && py >= y0 as f32
            && py <= y1 as f32
        {
            return Some(i);
        }
    }
    None
}

/// Fill the band behind a layer: rows across the frame for a horizontal line,
/// columns down it for a turned one. It is centred on the layer, so dragging the
/// text drags its strip with it.
fn draw_band(img: &mut [u8], w: usize, h: usize, layer: &TextLayer, band: &sdroxide_types::Strip) {
    let t = usize::from(band.thickness.max(1));
    let [fr, fg, fb] = band.fill;
    if layer.vertical {
        let cx = (layer.x * w as f32).round() as i32;
        let x0 = cx - t as i32 / 2;
        for x in x0..x0 + t as i32 {
            if x < 0 || x >= w as i32 {
                continue;
            }
            let u = (x - x0) as f32 / t as f32;
            let (r, g, b) = band_colour(fr, fg, fb, band.fill2, u);
            for y in 0..h {
                let i = (y * w + x as usize) * 3;
                img[i] = r;
                img[i + 1] = g;
                img[i + 2] = b;
            }
        }
    } else {
        let cy = (layer.y * h as f32).round() as i32;
        let y0 = cy - t as i32 / 2;
        for y in y0..y0 + t as i32 {
            if y < 0 || y >= h as i32 {
                continue;
            }
            let u = (y - y0) as f32 / t as f32;
            let (r, g, b) = band_colour(fr, fg, fb, band.fill2, u);
            let row = y as usize * w;
            for x in 0..w {
                let i = (row + x) * 3;
                img[i] = r;
                img[i + 1] = g;
                img[i + 2] = b;
            }
        }
    }
}

/// The band's colour at `u` (0 at the first edge, 1 at the last): a straight
/// two-stop gradient, or the original fade to black.
fn band_colour(fr: u8, fg: u8, fb: u8, fill2: Option<[u8; 3]>, u: f32) -> (u8, u8, u8) {
    match fill2 {
        Some([r2, g2, b2]) => {
            let lerp = |a: u8, c: u8| (f32::from(a) + (f32::from(c) - f32::from(a)) * u) as u8;
            (lerp(fr, r2), lerp(fg, g2), lerp(fb, b2))
        }
        None => {
            let shade = 1.0 - u;
            (
                (f32::from(fr) * shade) as u8,
                (f32::from(fg) * shade) as u8,
                (f32::from(fb) * shade) as u8,
            )
        }
    }
}

/// Draw one line, centred on its anchor — across the frame, or turned a quarter
/// turn clockwise about that same anchor.
fn draw_layer(img: &mut [u8], w: usize, h: usize, layer: &TextLayer) {
    let Some(font) = font(layer.face) else { return };
    let px = f32::from(layer.px.max(4));
    let scale = PxScale::from(px);
    let scaled = font.as_scaled(scale);
    let tw = text_width(&layer.text, &font, scale);
    let ax = layer.x * w as f32;
    let ay = layer.y * h as f32;
    // Baseline from the font's own ascent and descent, so a face with a deep
    // descender still sits in the middle of its band.
    let baseline = ay + (scaled.ascent() - scaled.descent()) / 2.0 - scaled.descent() / 2.0;
    let left = ax - tw / 2.0;
    let ink = Ink::Solid(layer.ink);
    let span = w as f32;

    let mut plain = crate::sstv::RgbSink { buf: img, w, h };
    if layer.vertical {
        // Painted flat and then turned about the anchor: one lattice rotation,
        // which is lossless, unlike scaling a rendered line.
        let mut turned = Turned { inner: &mut plain, cx: ax.round(), cy: ay.round() };
        paint_line(&mut turned, left, baseline, layer, &font, scale, ink, span);
    } else {
        paint_line(&mut plain, left, baseline, layer, &font, scale, ink, span);
    }
}

/// Print the line, with its outline first when it has one.
fn paint_line<S: TextSink>(
    sink: &mut S,
    left: f32,
    baseline: f32,
    layer: &TextLayer,
    font: &FontRef<'static>,
    scale: PxScale,
    ink: Ink,
    span: f32,
) {
    if let Some(oc) = layer.outline {
        // Eight directions, scaled to the text: a 12-pixel line needs one pixel
        // of edge, a 60-pixel headline needs more.
        let off = (f32::from(layer.px) / 14.0).max(1.0);
        for (ox, oy) in [
            (-off, 0.0),
            (off, 0.0),
            (0.0, -off),
            (0.0, off),
            (-off, -off),
            (off, -off),
            (-off, off),
            (off, off),
        ] {
            draw_text_into(
                sink,
                left + ox,
                baseline + oy,
                &layer.text,
                font,
                scale,
                Ink::Solid(oc),
                span,
                1.0,
            );
        }
    }
    draw_text_into(sink, left, baseline, &layer.text, font, scale, ink, span, 1.0);
}

/// A sink that turns everything a quarter turn clockwise about a point.
struct Turned<'a, S: TextSink> {
    inner: &'a mut S,
    cx: f32,
    cy: f32,
}

impl<S: TextSink> TextSink for Turned<'_, S> {
    fn paint(&mut self, x: i32, y: i32, rgb: (u8, u8, u8), alpha: f32) {
        let dx = x as f32 - self.cx;
        let dy = y as f32 - self.cy;
        // (dx, dy) → (-dy, dx): a point to the right of the anchor moves below
        // it, which is what makes the line read downwards.
        self.inner.paint((self.cx - dy).round() as i32, (self.cy + dx).round() as i32, rgb, alpha);
    }
}

// ── Whose copy of the document is the truth ──
//
// The document lives in the engine's preset store, and comes back on every
// presets announcement. One slot at a time is client-owned — whichever the
// operator is editing — so an echo of our own write, or an edit made on another
// screen, can update the other four without fighting the drag in hand. The same
// arrangement the preset messages use, for the same reason.

/// The slot claimed for editing, with the document being edited.
pub(crate) type Edit = Option<(usize, Composer)>;

/// The document to draw for `slot`: what is being edited if it is this slot,
/// otherwise what the engine says.
pub(crate) fn shown<'a>(
    edit: &'a Edit,
    presets: &'a ImagePresets,
    slot: usize,
) -> &'a Composer {
    match edit {
        Some((i, doc)) if *i == slot => doc,
        _ => presets.slots.get(slot).map(|s| &s.composer).unwrap_or(&EMPTY),
    }
}

static EMPTY: Composer = Composer { layers: Vec::new() };

/// Claim `slot` for editing, taking the engine's document as the starting
/// point. Returns whether the claim changed, so an unchanged claim does not
/// recompose the preview every frame.
pub(crate) fn claim(edit: &mut Edit, presets: &ImagePresets, slot: usize) -> bool {
    if matches!(edit, Some((i, _)) if *i == slot) {
        return false;
    }
    let doc = presets.slots.get(slot).map(|s| s.composer.clone()).unwrap_or_default();
    *edit = Some((slot, doc));
    true
}

/// Give the slot back to the engine.
pub(crate) fn release(edit: &mut Edit, presets: &ImagePresets, sent: &Edit) -> Option<Command> {
    let cmd = pending(edit, presets, sent);
    *edit = None;
    cmd
}

/// The command that would bring the engine up to date, if any.
///
/// `None` when nothing is claimed, when the engine already has this document, or
/// when this very document has already been sent and is only waiting for the
/// echo — which is what stops an un-echoed write being repeated on every frame.
pub(crate) fn pending(edit: &Edit, presets: &ImagePresets, sent: &Edit) -> Option<Command> {
    let (slot, doc) = edit.as_ref()?;
    let engine = presets.slots.get(*slot).map(|s| &s.composer);
    if engine == Some(doc) {
        return None;
    }
    if matches!(sent, Some((i, d)) if i == slot && d == doc) {
        return None;
    }
    Some(Command::ImageSetComposer { slot: *slot as u8, composer: doc.clone() })
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdroxide_types::{ImageSlotInfo, Strip};

    fn presets(docs: &[Composer]) -> ImagePresets {
        ImagePresets {
            templates: Vec::new(),
            slots: docs
                .iter()
                .map(|c| ImageSlotInfo { composer: c.clone(), ..ImageSlotInfo::default() })
                .collect(),
        }
    }

    fn doc(text: &str) -> Composer {
        let mut c = Composer::default();
        c.layers.push(TextLayer { text: text.into(), ..TextLayer::default() });
        c
    }

    fn frame(w: u16, h: u16) -> Vec<u8> {
        // Mid grey, so ink and band are both findable.
        vec![128u8; usize::from(w) * usize::from(h) * 3]
    }

    fn ink_rows(img: &[u8], w: u16, h: u16, want: [u8; 3]) -> Vec<usize> {
        let w = usize::from(w);
        let mut rows = Vec::new();
        for y in 0..usize::from(h) {
            let hit = (0..w).any(|x| {
                let i = (y * w + x) * 3;
                img[i] == want[0] && img[i + 1] == want[1] && img[i + 2] == want[2]
            });
            if hit {
                rows.push(y);
            }
        }
        rows
    }

    /// **A layer goes where its fractions say.** The document is written once
    /// and rendered at the mode's size, so this is the property everything else
    /// rests on — and it is why the document can ride the wire at all.
    #[test]
    fn a_layer_lands_at_its_fraction_of_the_frame() {
        let mut c = Composer::default();
        c.layers.push(TextLayer {
            text: "TEST".into(),
            x: 0.5,
            y: 0.25,
            px: 24,
            ink: [255, 0, 0],
            outline: None,
            ..Default::default()
        });
        let (img, w, h) = render(&c, 320, 256, None);
        let rows = ink_rows(&img, w, h, [255, 0, 0]);
        assert!(!rows.is_empty(), "nothing was printed");
        let mid = rows[rows.len() / 2];
        assert!(
            (mid as i32 - 64).abs() < 10,
            "a line at 0.25 of a 256-pixel frame printed at row {mid}"
        );
        // And the same document at another frame size lands at the same place.
        let (img2, w2, h2) = render(&c, 640, 512, None);
        let rows2 = ink_rows(&img2, w2, h2, [255, 0, 0]);
        let mid2 = rows2[rows2.len() / 2];
        assert!((mid2 as i32 - 128).abs() < 14, "row {mid2} at double size");
    }

    /// **A vertical layer reads downwards and its band runs the other way.**
    /// This is the "banner down the side" the operator asked for, and the one
    /// case that has to turn glyphs rather than lay out a line.
    #[test]
    fn a_vertical_layer_turns_the_text_and_its_band_with_it() {
        let mut c = Composer::default();
        c.layers.push(TextLayer {
            text: "F4NRM".into(),
            x: 0.15,
            y: 0.5,
            px: 22,
            ink: [255, 255, 0],
            outline: None,
            vertical: true,
            band: Some(Strip { thickness: 30, fill: [0, 0, 200], fill2: Some([0, 0, 200]) }),
            ..Default::default()
        });
        let (img, w, h) = render(&c, 320, 256, None);
        let rows = ink_rows(&img, w, h, [255, 255, 0]);
        // A turned line is taller than it is wide: it must cover far more rows
        // than a horizontal one of the same text would.
        assert!(rows.len() > 30, "a turned line covered only {} rows", rows.len());
        assert!(rows.len() < usize::from(h), "a turned line covered the whole frame");
        // The band is a column, not a row: at the layer's x, every row is blue
        // except where the letters are.
        let x = (0.15 * f32::from(w)) as usize;
        for y in 0..usize::from(h) {
            let i = (y * usize::from(w) + x) * 3;
            if img[i] == 255 && img[i + 1] == 255 {
                continue;
            }
            assert_eq!((img[i], img[i + 1], img[i + 2]), (0, 0, 200), "row {y} misses the band");
        }
    }

    /// The topmost layer is the one a click picks up, and a hidden one is not
    /// there at all.
    #[test]
    fn a_click_picks_the_top_layer_and_never_a_hidden_one() {
        let mut c = Composer::default();
        c.layers.push(TextLayer { text: "UNDER".into(), y: 0.5, px: 40, ..Default::default() });
        c.layers.push(TextLayer { text: "OVER".into(), y: 0.5, px: 40, ..Default::default() });
        assert_eq!(layer_at(&c, 160.0, 128.0, 320, 256), Some(1));
        c.layers[1].hidden = true;
        assert_eq!(layer_at(&c, 160.0, 128.0, 320, 256), Some(0));
        c.layers[0].hidden = true;
        assert_eq!(layer_at(&c, 160.0, 128.0, 320, 256), None);
    }

    /// Moving a layer's fractions moves its box, which is what a drag does.
    #[test]
    fn dragging_moves_the_box() {
        let mut c = Composer::default();
        c.layers.push(TextLayer { text: "CQ".into(), x: 0.2, y: 0.2, ..Default::default() });
        let before = layer_box(&c, 0, 320, 256).unwrap();
        c.layers[0].x = 0.8;
        let after = layer_box(&c, 0, 320, 256).unwrap();
        assert!(after.0 > before.0, "the box did not move right");
    }

    // ── the mosaic ──

    /// A solid colour rectangle, so the cells can be told apart.
    fn solid(w: u16, h: u16, rgb: [u8; 3]) -> Vec<u8> {
        let mut v = Vec::with_capacity(usize::from(w) * usize::from(h) * 3);
        for _ in 0..usize::from(w) * usize::from(h) {
            v.extend_from_slice(&rgb);
        }
        v
    }

    fn pixel(img: &[u8], w: u16, x: u16, y: u16) -> [u8; 3] {
        let i = (usize::from(y) * usize::from(w) + usize::from(x)) * 3;
        [img[i], img[i + 1], img[i + 2]]
    }

    /// **Four pictures make a 2×2**, each in its own quarter, and the originals
    /// are only read.
    #[test]
    fn four_pictures_make_a_two_by_two_grid() {
        let red = solid(64, 48, [220, 0, 0]);
        let green = solid(64, 48, [0, 220, 0]);
        let blue = solid(64, 48, [0, 0, 220]);
        let white = solid(64, 48, [220, 220, 220]);
        let cells: Vec<(&[u8], u16, u16)> =
            vec![(&red, 64, 48), (&green, 64, 48), (&blue, 64, 48), (&white, 64, 48)];
        let (img, w, h) = mosaic(320, 256, &cells);
        assert_eq!((w, h), (320, 256));
        // One probe inside each quarter.
        assert_eq!(pixel(&img, w, 80, 64), [220, 0, 0], "top left");
        assert_eq!(pixel(&img, w, 240, 64), [0, 220, 0], "top right");
        assert_eq!(pixel(&img, w, 80, 192), [0, 0, 220], "bottom left");
        assert_eq!(pixel(&img, w, 240, 192), [220, 220, 220], "bottom right");
        // And the originals are untouched: the source buffers still hold what
        // they held.
        assert_eq!(pixel(&red, 64, 0, 0), [220, 0, 0]);
    }

    /// **A cell is letterboxed, never stretched.** A wide picture in a square
    /// quarter keeps its shape, and the leftover is black rather than smeared.
    #[test]
    fn a_picture_keeps_its_shape_inside_its_cell() {
        // A 4:1 strip, pure red.
        let strip = solid(160, 40, [200, 0, 0]);
        let (img, w, _) = mosaic(320, 256, &[(&strip, 160, 40)]);
        // It fills the full width of the frame (160/40 is wider than 320/256),
        // so rows above and below it are black.
        assert_eq!(pixel(&img, w, 160, 4), [0, 0, 0], "the top is not black: 4:1 was stretched");
        assert_eq!(pixel(&img, w, 160, 128), [200, 0, 0], "the strip is not where it should be");
        assert_eq!(pixel(&img, w, 160, 251), [0, 0, 0], "the bottom is not black");
    }

    /// **A mosaic composed larger keeps the detail** the transmit path would
    /// otherwise throw away: the same layout at twice the frame has twice the
    /// pixels per cell, and the received originals are still untouched.
    #[test]
    fn a_mosaic_at_twice_the_frame_keeps_more_of_the_pictures() {
        let src = solid(64, 48, [200, 0, 0]);
        let cells: Vec<(&[u8], u16, u16)> = vec![(&src, 64, 48)];
        let (small, sw, _) = mosaic(320, 256, &cells);
        let (big, bw, bh) = mosaic(320 * MOSAIC_SCALE, 256 * MOSAIC_SCALE, &cells);
        assert_eq!((bw, bh), (640, 512));
        assert_eq!(big.len(), small.len() * 4, "twice each side is four times the pixels");
        // The picture is in the middle of both, and the big one has no black
        // gap where the small one's rounding left one.
        assert_eq!(pixel(&big, bw, 320, 256), [200, 0, 0]);
        assert_eq!(pixel(&small, sw, 160, 128), [200, 0, 0]);
    }

    /// One picture fills the frame; two sit side by side; three leave the fourth
    /// quarter black rather than stretching one of them into it.
    #[test]
    fn one_two_and_three_pictures_have_their_own_arrangements() {
        let red = solid(32, 32, [200, 0, 0]);
        let green = solid(32, 32, [0, 200, 0]);
        let (img, w, _) = mosaic(320, 256, &[(&red, 32, 32)]);
        // A square picture in a 5:4 frame is letterboxed left and right, not
        // stretched across it: the middle is the picture, the edges are black.
        assert_eq!(pixel(&img, w, 160, 128), [200, 0, 0], "the picture is not in the middle");
        assert_eq!(pixel(&img, w, 4, 128), [0, 0, 0], "a square was stretched across the frame");

        let (img, w, _) = mosaic(320, 256, &[(&red, 32, 32), (&green, 32, 32)]);
        assert_eq!(pixel(&img, w, 40, 128), [200, 0, 0]);
        assert_eq!(pixel(&img, w, 280, 128), [0, 200, 0]);

        let (img, w, _) = mosaic(320, 256, &[(&red, 32, 32), (&green, 32, 32), (&red, 32, 32)]);
        assert_eq!(pixel(&img, w, 240, 192), [0, 0, 0], "the empty quarter is not black");
    }

    /// **The grid follows the count**, up to nine cells, and every cell keeps
    /// its own quarter of the frame.
    #[test]
    fn the_grid_follows_how_many_pictures_there_are() {
        assert_eq!(grid_for(1), (1, 1));
        assert_eq!(grid_for(2), (2, 1));
        assert_eq!(grid_for(3), (2, 2));
        assert_eq!(grid_for(4), (2, 2));
        assert_eq!(grid_for(5), (3, 2));
        assert_eq!(grid_for(6), (3, 2));
        assert_eq!(grid_for(9), (3, 3));

        // Nine pictures, one colour each, and each lands in its own cell.
        let colours = [
            [200, 0, 0],
            [0, 200, 0],
            [0, 0, 200],
            [200, 200, 0],
            [0, 200, 200],
            [200, 0, 200],
            [120, 120, 120],
            [240, 240, 240],
            [40, 80, 40],
        ];
        let cells: Vec<(Vec<u8>, u16, u16)> =
            colours.iter().map(|c| (solid(32, 32, *c), 32u16, 32u16)).collect();
        let refs: Vec<(&[u8], u16, u16)> =
            cells.iter().map(|(v, w, h)| (v.as_slice(), *w, *h)).collect();
        let (img, w, h) = mosaic(320, 256, &refs);
        for (i, want) in colours.iter().enumerate() {
            let (cx, cy) = (i % 3, i / 3);
            let x = (320 / 3) * (cx as u16) + 40;
            let y = (256 / 3) * (cy as u16) + 30;
            assert_eq!(pixel(&img, w, x.min(w - 1), y.min(h - 1)), *want, "cell {i}");
        }
        // A tenth picture is not drawn anywhere: the cap is nine.
        assert_eq!(grid_for(12), (3, 3));
    }

    /// No pictures is an empty frame, not a panic, and a zero-sized source is
    /// skipped rather than dividing by zero.
    #[test]
    fn an_empty_or_broken_mosaic_is_a_blank_frame() {
        let (img, w, h) = mosaic(64, 48, &[]);
        assert_eq!((w, h), (64, 48));
        assert!(img.iter().all(|b| *b == 0));
        let empty: Vec<u8> = Vec::new();
        let (img, ..) = mosaic(64, 48, &[(&empty, 0, 0)]);
        assert!(img.iter().all(|b| *b == 0));
    }

    // ── the hand-off ──

    /// **The engine's document is the truth for every slot that is not being
    /// edited**, and the edit is the truth for the one that is — which is what
    /// lets an echo arrive without yanking the line out of the operator's hand.
    #[test]
    fn the_slot_being_edited_follows_the_hand_and_the_others_follow_the_engine() {
        let p = presets(&[doc("a"), doc("b"), doc("c")]);
        let mut edit: Edit = None;
        assert!(claim(&mut edit, &p, 1));
        assert_eq!(shown(&edit, &p, 1).layers[0].text, "b");
        // The operator types: the edit is what the preview draws...
        if let Some((_, d)) = edit.as_mut() {
            d.layers[0].text = "typed".into();
        }
        assert_eq!(shown(&edit, &p, 1).layers[0].text, "typed");
        // ...and an edit made elsewhere still reaches the other slots.
        assert_eq!(shown(&edit, &p, 0).layers[0].text, "a");
        assert_eq!(shown(&edit, &p, 2).layers[0].text, "c");
        // A slot the engine has never mentioned reads empty rather than panicking.
        assert!(!shown(&edit, &p, 99).is_active());
    }

    /// A write goes out once, and is not repeated while its echo is in flight.
    #[test]
    fn a_document_is_sent_once_and_not_again_until_it_changes() {
        let p = presets(&[doc("old")]);
        let mut edit: Edit = None;
        let mut sent: Edit = None;
        claim(&mut edit, &p, 0);
        assert_eq!(pending(&edit, &p, &sent), None, "an unchanged document is not written");

        if let Some((_, d)) = edit.as_mut() {
            d.layers[0].text = "new".into();
        }
        let cmd = pending(&edit, &p, &sent).expect("the change is written");
        assert!(matches!(&cmd, Command::ImageSetComposer { slot: 0, composer } if composer.layers[0].text == "new"));
        sent = edit.clone();
        assert_eq!(pending(&edit, &p, &sent), None, "not repeated before the echo");

        // The echo lands: the engine now has it, so nothing is pending.
        let echoed = presets(&[doc("new")]);
        assert_eq!(pending(&edit, &echoed, &sent), None);

        // Releasing an edited slot hands it back and writes the difference.
        if let Some((_, d)) = edit.as_mut() {
            d.layers[0].text = "final".into();
        }
        let cmd = release(&mut edit, &echoed, &sent).expect("the last edit is written");
        assert!(matches!(&cmd, Command::ImageSetComposer { slot: 0, composer } if composer.layers[0].text == "final"));
        assert!(edit.is_none(), "the claim is released");
    }

    /// An empty document is not active, so a preset nobody has composed still
    /// sends the picture it always did, banner and caption included.
    #[test]
    fn an_empty_document_falls_back_to_the_banner() {
        assert!(!Composer::default().is_active());
        let mut c = Composer::default();
        c.layers.push(TextLayer::default());
        assert!(!c.is_active(), "a layer with no text draws nothing");
        c.layers[0].text = "HI".into();
        assert!(c.is_active());
        c.layers[0].hidden = true;
        assert!(!c.is_active());
    }

    /// A grey frame of the right size is what a document with no picture under
    /// it composes onto.
    #[test]
    fn an_empty_slot_composes_onto_a_flat_frame() {
        let c = doc("HI");
        let (img, w, h) = render(&c, 64, 48, None);
        assert_eq!(img.len(), 64 * 48 * 3);
        assert_eq!((w, h), (64, 48));
    }

    /// For the eye, and only when asked: **a four-picture mosaic with a
    /// retransmission label, the station overlay on and off** — the example
    /// F4NRM asked to see on #240, produced by the same functions the editor
    /// calls rather than drawn by hand.
    #[test]
    fn a_mosaic_with_a_retransmission_label_can_be_dumped_to_look_at() {
        if std::env::var("SDROXIDE_MOSAIC_DUMP").is_err() {
            return;
        }
        // Four "received" pictures, each a different colour with a band so the
        // grid and the letterboxing are both visible.
        let cell = |rgb: [u8; 3], w: u16, h: u16| {
            let mut v = vec![0u8; usize::from(w) * usize::from(h) * 3];
            for y in 0..usize::from(h) {
                for x in 0..usize::from(w) {
                    let i = (y * usize::from(w) + x) * 3;
                    let edge = y < usize::from(h) / 6;
                    let c = if edge { [255u8, 255, 255] } else { rgb };
                    v[i..i + 3].copy_from_slice(&c);
                }
            }
            v
        };
        let a = cell([40, 90, 160], 320, 240);
        let b = cell([160, 70, 40], 240, 320);
        let c = cell([50, 140, 70], 300, 200);
        let d = cell([120, 60, 140], 200, 200);
        let cells: Vec<(&[u8], u16, u16)> =
            vec![(&a, 320, 240), (&b, 240, 320), (&c, 300, 200), (&d, 200, 200)];
        let (mosaic_rgb, w, h) = mosaic(320, 256, &cells);

        // The station overlay: the callsign on a strip at the top.
        let mut overlay = Composer::default();
        overlay.layers.push(TextLayer {
            text: "19DCG373".into(),
            face: Face::Mono,
            x: 0.5,
            y: 0.07,
            px: 24,
            band: Some(Strip { thickness: 34, fill: [150, 20, 20], fill2: Some([40, 0, 0]) }),
            ..Default::default()
        });
        // The retransmission label: this picture's own document.
        let mut own = Composer::default();
        own.layers.push(TextLayer {
            text: "RELAY OF SKEDS 20/10".into(),
            face: Face::ChakraBold,
            x: 0.5,
            y: 0.86,
            px: 20,
            ink: [255, 230, 60],
            outline: Some([0, 0, 0]),
            ..Default::default()
        });

        for (name, on) in [("overlay-on", true), ("overlay-off", false)] {
            let doc = if on { own.over(&overlay) } else { own.clone() };
            let mut img = mosaic_rgb.clone();
            draw_onto(&doc, &mut img, w, h);
            let png = crate::sstv::encode_png(&img, w, h).unwrap();
            let path = format!("/tmp/opencode/sstv-mosaic-{name}.png");
            std::fs::write(&path, png).unwrap();
            eprintln!("wrote {path}");
        }
    }

    /// For the eye, and only when asked: a sample composition to look at.
    /// A layout is judged by looking, not by counting pixels.
    #[test]
    fn a_sample_layout_can_be_dumped_to_look_at() {
        if std::env::var("SDROXIDE_COMPOSER_DUMP").is_err() {
            return;
        }
        let mut c = Composer::default();
        c.layers.push(TextLayer {
            text: "19DCG373".into(),
            face: Face::Mono,
            x: 0.5,
            y: 0.08,
            px: 30,
            band: Some(Strip { thickness: 44, fill: [150, 20, 20], fill2: Some([20, 0, 0]) }),
            ..Default::default()
        });
        c.layers.push(TextLayer {
            text: "CQ CQ DE 19DCG373".into(),
            face: Face::Chakra,
            x: 0.5,
            y: 0.55,
            px: 26,
            ink: [255, 230, 60],
            ..Default::default()
        });
        c.layers.push(TextLayer {
            text: "PSE K".into(),
            face: Face::ChakraBold,
            x: 0.5,
            y: 0.75,
            px: 22,
            ..Default::default()
        });
        c.layers.push(TextLayer {
            text: "JO22".into(),
            face: Face::Mono,
            x: 0.07,
            y: 0.5,
            px: 20,
            vertical: true,
            band: Some(Strip { thickness: 26, fill: [0, 40, 120], fill2: Some([0, 0, 30]) }),
            ..Default::default()
        });
        let (img, w, h) = render(&c, 320, 256, None);
        let png = crate::sstv::encode_png(&img, w, h).unwrap();
        std::fs::write("/tmp/opencode/composer-sample.png", png).unwrap();
        eprintln!("wrote /tmp/opencode/composer-sample.png");
        let _ = frame(w, h);
    }
}
