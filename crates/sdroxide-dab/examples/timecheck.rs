//! Measure how long the capture takes to name an ensemble and its services —
//! what a scan dwell has to cover. Scratch; not shipped.

use num_complex::Complex32;
use sdroxide_dab::DabReceiver;

fn main() {
    let raw = std::fs::read("/tmp/opencode/dab-lab/nancy.cs16").unwrap();
    let iq: Vec<Complex32> = raw
        .chunks_exact(4)
        .map(|b| {
            Complex32::new(
                i16::from_le_bytes([b[0], b[1]]) as f32 / 32768.0,
                i16::from_le_bytes([b[2], b[3]]) as f32 / 32768.0,
            )
        })
        .collect();
    let rate = 2_500_000.0;
    let mut rx = DabReceiver::new(rate).unwrap();
    let mut fed = 0usize;
    let (mut named_at, mut svc_at) = (None, None);
    for block in iq.chunks(32768) {
        rx.feed(block);
        fed += block.len();
        let secs = fed as f64 / rate;
        if named_at.is_none() && rx.ensemble().label.is_some() {
            named_at = Some(secs);
        }
        if svc_at.is_none() && !rx.ensemble().services.is_empty() {
            svc_at = Some(secs);
            break;
        }
    }
    println!("ensemble named after {:?}s, services after {:?}s", named_at, svc_at);
}
