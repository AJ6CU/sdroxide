//! Scratch: decode the known-good capture, but at 2.000 Msps input so the
//! receiver's resampler (2.0 -> 2.048) runs, exactly as the live RSP1 lane
//! does. Reports frames and FIBs, so a resampler/window problem shows apart
//! from an RF one.

use num_complex::Complex32;
use sdroxide_dab::DabReceiver;

fn main() {
    let raw = std::fs::read("/tmp/opencode/dab-lab/nancy.cs16").unwrap();
    let native: Vec<Complex32> = raw
        .chunks_exact(4)
        .map(|b| {
            Complex32::new(
                i16::from_le_bytes([b[0], b[1]]) as f32 / 32768.0,
                i16::from_le_bytes([b[2], b[3]]) as f32 / 32768.0,
            )
        })
        .collect();
    for in_rate in [2_500_000.0_f64, 2_048_000.0, 2_000_000.0] {
        // Resample the capture to the rate under test by nearest sample; crude,
        // but it is the *receiver's* resampler we are exercising, so the input
        // shape only has to be plausible.
        let ratio = in_rate / 2_500_000.0;
        let n = (native.len() as f64 * ratio) as usize;
        let iq: Vec<Complex32> =
            (0..n).map(|i| native[((i as f64) / ratio) as usize % native.len()]).collect();
        let mut rx = DabReceiver::new(in_rate).unwrap();
        for b in iq.chunks(32768) {
            rx.feed(b);
        }
        let e = rx.ensemble();
        let (frames, fibs) = rx.counters();
        println!(
            "in_rate {:>10.0}: frames {frames}, fibs {fibs}, services {}, ensemble {:?}",
            in_rate,
            e.services.len(),
            e.label
        );
    }
}
