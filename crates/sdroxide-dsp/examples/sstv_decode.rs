//! Decode an SSTV WAV with sdroxide's own receiver and write the picture out
//! as a binary PPM, for cross-checking against an independent encoder.
//!
//! Usage: `sstv_decode input.wav output.ppm`
//!
//! Built for the mode-interop check: encode a known image with PySSTV (a
//! separate implementation), decode the WAV here, and compare the two. That is
//! the check the round-trip tests in `sstv.rs` cannot make — they feed the
//! decoder its own encoder's output.

use std::io::Write;

use sdroxide_dsp::{SstvEvent, SstvRx};

fn main() {
    let mut args = std::env::args().skip(1);
    let input = args.next().expect("usage: sstv_decode input.wav output.ppm");
    let output = args.next().expect("usage: sstv_decode input.wav output.ppm");

    let mut r = hound::WavReader::open(&input).expect("open wav");
    let spec = r.spec();
    assert_eq!(spec.channels, 1, "SSTV fixture must be mono");
    let rate = spec.sample_rate as f64;
    let audio: Vec<f32> = r.samples::<i16>().map(|s| s.expect("sample") as f32 / 32768.0).collect();

    let mut rx = SstvRx::new(rate);
    let mut events = Vec::new();
    let mut mode = None;
    let mut image: Vec<u8> = Vec::new();
    let mut dims = (0u16, 0u16);
    let mut lines = 0usize;
    let mut complete = false;

    for chunk in audio.chunks(4800) {
        events.clear();
        rx.process(chunk, &mut events);
        for e in &events {
            match e {
                SstvEvent::ModeDetected(m) => {
                    mode = Some(*m);
                    dims = m.dimensions();
                    image = vec![0u8; dims.0 as usize * dims.1 as usize * 3];
                }
                SstvEvent::Line { y, rgb } => {
                    let w = dims.0 as usize;
                    let row = *y as usize * w * 3;
                    if row + rgb.len() <= image.len() {
                        image[row..row + rgb.len()].copy_from_slice(rgb);
                    }
                    lines += 1;
                }
                SstvEvent::ImageComplete { .. } => complete = true,
                SstvEvent::GaveUp => {}
                SstvEvent::FskId(id) => eprintln!("fsk id: {id}"),
                SstvEvent::UnsupportedMode { code, name } => {
                    eprintln!("unsupported VIS ${code:02X} ({name:?})");
                }
            }
        }
    }

    let (w, h) = dims;
    println!("detected {:?}, {w}x{h}, {lines} lines, complete {complete}", mode);
    if mode.is_none() || lines == 0 {
        std::process::exit(1);
    }
    let mut f = std::fs::File::create(&output).expect("create ppm");
    write!(f, "P6\n{w} {h}\n255\n").unwrap();
    f.write_all(&image).unwrap();
}
