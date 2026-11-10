//! Watch the IQ image balancer converge — with no hardware required.
//!
//! ```text
//! cargo run --release -p sdroxide-airspyhf --example balance_check
//! cargo run --release -p sdroxide-airspyhf --example balance_check -- capture.cs16
//! ```
//!
//! The estimator ported from libairspyhf's `iqbalancer.c` is the one part of
//! this driver that can be judged without owning the receiver, and this is how:
//! it synthesises a tone with a known gain and phase imbalance, runs it through
//! the balancer, and prints image rejection in dB against block number. A
//! working port climbs and settles; a transcription slip either goes nowhere or
//! makes the image *worse*.
//!
//! Given a file argument it reads raw interleaved `i16` samples instead —
//! exactly what the bulk endpoint delivers, so a capture from a real receiver
//! can be replayed offline. **Q first, then I**, matching the wire.

use std::f32::consts::TAU;

use rustfft::FftPlanner;
use sdroxide_airspyhf::IqBalancer;
use sdroxide_airspyhf::iqbalancer::{FFT_BINS, WORKING_BUFFER_LEN};
use sdroxide_dsp::Complex32;

/// Power in the strongest bin against power in its mirror, in dB.
fn image_rejection_db(
    x: &[Complex32],
    window: &[f32],
    fft: &dyn rustfft::Fft<f32>,
) -> (f32, usize) {
    let mut buf: Vec<Complex32> = x[x.len() - FFT_BINS..].to_vec();
    for (v, w) in buf.iter_mut().zip(window) {
        *v *= *w;
    }
    fft.process(&mut buf);
    // Ignore DC and the very edges, the same regions the estimator excludes.
    let edge = FFT_BINS / 22;
    let (mut best, mut best_bin) = (0.0f32, edge);
    for (i, v) in buf.iter().enumerate() {
        if i < edge || i > FFT_BINS - edge {
            continue;
        }
        if v.norm_sqr() > best {
            best = v.norm_sqr();
            best_bin = i;
        }
    }
    let image = buf[FFT_BINS - best_bin].norm_sqr();
    (10.0 * (best.max(1e-30) / image.max(1e-30)).log10(), best_bin)
}

/// A complex tone with a known gain/phase imbalance, in the same functional
/// form the corrector undoes.
fn synth(n: usize, bin: f32, phase0: f32, p: f32, a: f32) -> Vec<Complex32> {
    (0..n)
        .map(|i| {
            let t = TAU * bin * i as f32 / FFT_BINS as f32 + phase0;
            let (re, im) = (t.cos(), t.sin());
            Complex32::new((re + p * im) * (1.0 + a), (im + p * re) * (1.0 - a))
        })
        .collect()
}

/// Raw interleaved `i16`, Q before I, as the bulk endpoint delivers them.
fn read_capture(path: &str) -> Vec<Complex32> {
    let bytes = std::fs::read(path).unwrap_or_else(|e| {
        eprintln!("cannot read {path}: {e}");
        std::process::exit(1);
    });
    bytes
        .chunks_exact(4)
        .map(|b| {
            let im = i16::from_le_bytes([b[0], b[1]]) as f32 / 32768.0;
            let re = i16::from_le_bytes([b[2], b[3]]) as f32 / 32768.0;
            Complex32::new(re, im)
        })
        .collect()
}

fn main() {
    let arg = std::env::args().nth(1);
    let fft = FftPlanner::<f32>::new().plan_fft_forward(FFT_BINS);
    let window = sdroxide_dsp::blackman_harris(FFT_BINS);

    let mut b = IqBalancer::new();
    // The shipping settings need seconds of signal per update, which makes for a
    // dull chart. These converge in about a second of samples and exercise the
    // identical code path.
    b.configure(0, 4, 2, 1);

    let blocks: Vec<Vec<Complex32>> = match &arg {
        Some(path) => {
            println!("Replaying {path} (interleaved i16, Q first)");
            read_capture(path).chunks(WORKING_BUFFER_LEN).map(<[Complex32]>::to_vec).collect()
        }
        None => {
            let (p, a) = (0.02f32, 0.01f32);
            println!("Synthetic tone with an injected imbalance of phase {p}, amplitude {a}");
            println!("(the balancer should converge to roughly the negatives of those)");
            (0..60)
                .map(|blk| {
                    let phase0 = TAU * 700.0 * (blk * WORKING_BUFFER_LEN) as f32 / FFT_BINS as f32;
                    synth(WORKING_BUFFER_LEN, 700.0, phase0, p, a)
                })
                .collect()
        }
    };

    if blocks.is_empty() || blocks[0].len() < FFT_BINS {
        eprintln!("not enough samples: need at least {FFT_BINS} in the first block");
        std::process::exit(1);
    }

    println!();
    println!(
        "{:>5}  {:>10}  {:>10}  {:>9}  {:>9}  updates",
        "block", "rejection", "bin", "phase", "amp"
    );
    let mut first = None;
    let mut last = 0.0;
    for (i, blk) in blocks.iter().enumerate() {
        if blk.len() < FFT_BINS {
            break;
        }
        let mut x = blk.clone();
        b.process(&mut x);
        let (db, bin) = image_rejection_db(&x, &window, fft.as_ref());
        first.get_or_insert(db);
        last = db;
        let (p, a) = b.state();
        // A line per block is unreadable over a long capture.
        if blocks.len() <= 60 || i % (blocks.len() / 60).max(1) == 0 {
            println!("{i:>5}  {db:>9.1} dB  {bin:>10}  {p:>+9.6}  {a:>+9.6}  {:>7}", b.estimates());
        }
    }

    let first = first.unwrap_or(0.0);
    println!();
    println!("image rejection: {first:.1} dB → {last:.1} dB ({:+.1} dB)", last - first);
    let (p, a) = b.state();
    println!("converged correction: phase {p:+.6}, amplitude {a:+.6}, {} updates", b.estimates());
    if b.estimates() == 0 {
        println!();
        println!("The estimator never updated. On a real capture that means the band was");
        println!("too quiet, or too short — it needs a few seconds of signal.");
    }
}
