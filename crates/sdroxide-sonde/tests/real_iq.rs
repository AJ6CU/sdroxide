//! Real-recording check. Ignored by default: set `SONDE_IQ_WAV` to a 16-bit
//! stereo (I/Q) WAV and run `cargo test --release -p sdroxide-sonde --test real_iq -- --ignored --nocapture`.
//! Sweeps the carrier offset and prints what decodes.

use sdroxide_dsp::Complex32 as Complex;
use sdroxide_sonde::{SondeModel, decoder_for};

fn read_wav(path: &str, max_secs: f64) -> (f64, Vec<Complex>) {
    let b = std::fs::read(path).expect("read wav");
    let rate = u32::from_le_bytes(b[24..28].try_into().unwrap()) as f64;
    let mut p = 12;
    while &b[p..p + 4] != b"data" {
        p += 8 + u32::from_le_bytes(b[p + 4..p + 8].try_into().unwrap()) as usize;
    }
    let len = u32::from_le_bytes(b[p + 4..p + 8].try_into().unwrap()) as usize;
    let data = &b[p + 8..(p + 8 + len).min(b.len())];
    let n = ((max_secs * rate) as usize).min(data.len() / 4);
    let iq = (0..n)
        .map(|i| {
            let re = i16::from_le_bytes([data[4 * i], data[4 * i + 1]]) as f32 / 32768.0;
            let im = i16::from_le_bytes([data[4 * i + 2], data[4 * i + 3]]) as f32 / 32768.0;
            Complex::new(re, im)
        })
        .collect();
    (rate, iq)
}

#[test]
#[ignore]
fn real_recording_sweep() {
    let path = std::env::var("SONDE_IQ_WAV").expect("SONDE_IQ_WAV");
    let (rate0, iq0) = read_wav(&path, 60.0);
    // The decoder wants at least 48 kS/s: upsample x3 with linear interpolation.
    let rate = rate0 * 3.0;
    let mut iq = Vec::with_capacity(iq0.len() * 3);
    for w in iq0.windows(2) {
        for k in 0..3 {
            let t = k as f32 / 3.0;
            iq.push(w[0] * (1.0 - t) + w[1] * t);
        }
    }
    println!("{} samples at {rate} Hz", iq.len());
    let mut best = (0usize, 0.0f64);
    let mut off = -14_000.0;
    while off <= 14_000.0 {
        let mut dec = decoder_for(SondeModel::Rs41, rate, off).unwrap();
        let mut n = 0;
        let mut first = None;
        for chunk in iq.chunks(4096) {
            for d in dec.process(chunk) {
                n += 1;
                if first.is_none() {
                    first = Some(format!("{} #{} {:?} pos={:?}", d.serial, d.frame_num, d.battery_v, d.pos));
                }
            }
        }
        if n > 0 {
            println!("offset {off:+.0} Hz: {n} frames, first {first:?}");
        }
        if n > best.0 {
            best = (n, off);
        }
        off += 500.0;
    }
    println!("best: {} frames at {:+.0} Hz", best.0, best.1);
}
