//! Draw the waterfall the way the panadapter draws it, with and without
//! **FLATTEN**, so the correction can be looked at rather than only asserted.
//!
//! Unit tests can say the lift is one-sided and stops at the ceiling. Neither
//! is the question an operator has, which is what a quiet band looks like
//! before and after. So this drives the *real* `flatten_offsets`, the real
//! `apply_offsets` and the *real* colour map over a synthetic band whose floor
//! falls away under a carrier — and writes both pictures as one PNG.
//!
//! ```sh
//! cargo run -p sdroxide-ui --release --example flatten_probe
//! # writes target/flatten_probe.png
//! ```
//!
//! The band is synthetic, and the picture is only as good as that: it says
//! whether the correction does the thing it claims on the shapes a real band
//! has, not whether those shapes are the ones this radio hears.

use image::{Rgb, RgbImage};
use sdroxide_ui::waterfall_gpu::{FLATTEN_ALPHA, apply_offsets, flatten_offsets};

const COLS: usize = 1024;
const ROWS: usize = 260;
/// The level window the engine hands over, in dB, and the ramp it maps onto.
const FLOOR_DB: f32 = -125.0;
const CEIL_DB: f32 = -45.0;
/// The colour map to colourise through — the one FLATTEN is judged under.
const PALETTE: usize = 0; // Classic

/// A cheap deterministic hash, so the probe draws the same noise every run.
fn hash2(a: u32, b: u32) -> u32 {
    let mut h = a.wrapping_mul(0x9E37_79B9) ^ b.wrapping_mul(0x85EB_CA6B);
    h ^= h >> 15;
    h = h.wrapping_mul(0xC2B2_AE35);
    h ^= h >> 13;
    h
}

fn db_to_u8(db: f32) -> u8 {
    (((db - FLOOR_DB) / (CEIL_DB - FLOOR_DB)) * 255.0).round().clamp(0.0, 255.0) as u8
}

/// One row of spectrum, in `u8` ramp steps, before flattening.
///
/// A floor that *falls* across the band under a carrier at mid-span — the shape
/// this exists to fix. dB returns per column, so the drop reads as a floor and
/// not as a ramp painted on.
fn band_row(i: usize) -> Vec<u8> {
    let t = i as f32;
    let mut row = Vec::with_capacity(COLS);
    for x in 0..COLS {
        let f = x as f32 / COLS as f32;
        // Floor from -118 dB on the left, down to -104 by the carrier, back to
        // -100 on the far side: about 18 dB of spread across the band.
        let floor =
            if f < 0.5 { -118.0 + 14.0 * (f / 0.5) } else { -104.0 + 4.0 * ((f - 0.5) / 0.5) };
        // Receiver noise, not a travelling wave: a hash of (column, row) gives
        // the speckle a real floor has, and the slow `drift` term is the common
        // -mode wander every receiver shows. A structured pattern here would
        // put a picture of this probe's generator into the PNG instead of a
        // picture of the correction.
        let drift = 0.9 * (t * 0.11).sin();
        let speckle = 2.2 * (hash2(x as u32, i as u32) as f32 / u32::MAX as f32 - 0.5);
        let mut db = floor + drift + speckle;
        // The carrier that took the floor down with it, a weaker one, and one so
        // faint it must stay invisible.
        for (at, width, amp) in
            [(0.50f32, 0.010f32, 55.0f32), (0.18, 0.006, 22.0), (0.82, 0.005, 9.0)]
        {
            let d = ((f - at) / width).abs();
            if d < 1.0 {
                db += amp * (-3.0 * d * d).exp();
            }
        }
        row.push(db_to_u8(db));
    }
    row
}

/// The same band, drawn twice: as the panadapter draws it now, and with
/// FLATTEN on. Both come out of running the real code, so the pair is what an
/// operator would actually see rather than an illustration of it.
fn waterfalls() -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
    let mut off: Vec<f32> = Vec::new();
    let mut plain = Vec::with_capacity(ROWS);
    let mut lifted = Vec::with_capacity(ROWS);
    for i in 0..ROWS {
        let plain_row = band_row(i);
        let mut lifted_row = plain_row.clone();
        off = flatten_offsets(&plain_row, &off, FLOOR_DB, CEIL_DB, FLATTEN_ALPHA);
        apply_offsets(&mut lifted_row, &off);
        plain.push(plain_row);
        lifted.push(lifted_row);
    }
    (plain, lifted)
}

fn paint(img: &mut RgbImage, row: &[u8], y: usize) {
    let lut = sdroxide_ui::colormap::lut(PALETTE);
    for (x, &v) in row.iter().enumerate() {
        let i = v as usize * 4;
        img.put_pixel(x as u32, y as u32, Rgb([lut[i], lut[i + 1], lut[i + 2]]));
    }
}

fn main() {
    let (plain, lifted) = waterfalls();

    const GAP: usize = 4;
    let mut img = RgbImage::from_pixel(COLS as u32, (ROWS * 2 + GAP) as u32, Rgb([16, 16, 16]));
    for (i, row) in plain.iter().enumerate() {
        paint(&mut img, row, ROWS - 1 - i);
    }
    for (i, row) in lifted.iter().enumerate() {
        paint(&mut img, row, ROWS + GAP + ROWS - 1 - i);
    }
    // A gutter between them, so it is obvious which is which in a viewer.
    for x in 0..COLS as u32 {
        for dy in 0..GAP as u32 {
            img.put_pixel(x, ROWS as u32 + dy, Rgb([220, 220, 90]));
        }
    }

    let path = std::path::Path::new("target/flatten_probe.png");
    if let Some(dir) = path.parent() {
        let _ = std::fs::create_dir_all(dir);
    }
    img.save(path).expect("the probe writes a PNG a person can open");
    eprintln!(
        "wrote {} — top: the waterfall as drawn now, bottom: the same band with FLATTEN on",
        path.display()
    );
}
