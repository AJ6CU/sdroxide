//! Replay a raw RX-888 capture through the wideband downconverter.
//!
//! ```text
//! cargo run --release -p sdroxide-rx888 --example ddc_replay -- capture.i16 17.740
//! ```
//!
//! The file is the raw bulk-endpoint bytes as written by `probe`'s `PROBE_DUMP`,
//! i.e. 16-bit little-endian real samples straight off the ADC, still
//! randomized. The second argument is the frequency to tune to, in MHz.
//!
//! # Why this exists
//!
//! Synthetic tones prove the converter is self-consistent, which is not the same
//! as proving it is right. This replays actual off-air HF and reports the power
//! in the tuned channel against the power well outside it, so a real broadcast
//! carrier has to show up where it physically is. Point it at a frequency with a
//! known station and then at a quiet part of the band; if those two come back
//! the same, something is wrong no matter how green the unit tests are.

use std::f64::consts::TAU;

use sdroxide_dsp::{Complex32, WbDdc};
use sdroxide_rx888::convert;

const FS: f64 = 64.8e6;
const N: usize = 8192;
const M: usize = 256;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() < 2 {
        eprintln!("usage: ddc_replay <capture.i16> <MHz> [more MHz...]");
        std::process::exit(2);
    }
    let bytes = std::fs::read(&args[0]).expect("cannot read the capture");
    println!(
        "{} : {:.1} MB = {:.2} s at {:.1} Msps",
        args[0],
        bytes.len() as f64 / 1e6,
        bytes.len() as f64 / 2.0 / FS,
        FS / 1e6
    );

    // Skip the first megabyte: the stream start is not representative.
    let bytes = &bytes[1_000_000.min(bytes.len())..];
    let mut real = Vec::new();
    convert::to_f32(bytes, true, &mut None, &mut real);
    println!("{} real samples\n", real.len());

    println!("{:>12} {:>12} {:>12} {:>10}", "tune MHz", "out Msps", "rms dBFS", "peak dBFS");
    for a in &args[1..] {
        let mhz: f64 = a.parse().expect("frequency must be a number in MHz");
        let mut ddc = WbDdc::new(FS, N, M);
        ddc.set_center_hz(mhz * 1e6);

        let mut out: Vec<Complex32> = Vec::new();
        // One pass is enough: a couple of hundred blocks is plenty of averaging.
        let take = real.len().min(N * 400);
        ddc.process(&real[..take], &mut out);

        // Drop the first block, which is still filling the overlap.
        let out = &out[M.min(out.len())..];
        if out.is_empty() {
            println!("{mhz:>12.4} (no output)");
            continue;
        }
        let n = out.len() as f64;
        let rms = (out.iter().map(|v| v.norm_sqr() as f64).sum::<f64>() / n).sqrt();
        let peak = out.iter().map(|v| v.norm()).fold(0.0f32, f32::max);
        println!(
            "{mhz:>12.4} {:>12.4} {:>12.1} {:>10.1}",
            ddc.out_rate() / 1e6,
            20.0 * rms.max(1e-12).log10(),
            20.0 * peak.max(1e-12).log10(),
        );

        // Where inside the channel the energy actually sits, by direct DFT at a
        // few offsets — enough to see a carrier without pulling in an FFT.
        let mut best = (0.0f64, 0.0f32);
        let steps = 240;
        for s in 0..=steps {
            let f = -ddc.usable_bw_hz() / 2.0 + ddc.usable_bw_hz() * s as f64 / steps as f64;
            let mut acc = Complex32::default();
            for (i, v) in out.iter().enumerate().take(65536) {
                let ph = -TAU * f * i as f64 / ddc.out_rate();
                acc += *v * Complex32::new(ph.cos() as f32, ph.sin() as f32);
            }
            let mag = acc.norm() / out.len().min(65536) as f32;
            if mag > best.1 {
                best = (f, mag);
            }
        }
        println!(
            "{:>12} strongest component {:+.1} kHz from centre = {:.4} MHz, {:.1} dBFS",
            "",
            best.0 / 1e3,
            (mhz * 1e6 + best.0) / 1e6,
            20.0 * best.1.max(1e-12).log10()
        );
    }
}
