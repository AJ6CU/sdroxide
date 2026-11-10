//! Where in the interleaver does a recording start?
//!
//!     cargo run --release -p sdroxide-dsp --example s4285_align_check -- \
//!         ~/s4285-captures/keepers/….wav [carrier_hz]
//!
//! A convolutional deinterleaver has a **commutator position**, and a recording
//! that begins mid-transmission begins with that position unknown — one of 32
//! rows. Every check this fork has run so far started it at row zero, which is
//! the transmitter's own start and the case a loopback can test; on an off-air
//! capture it is right one time in thirty-two, and when it is wrong the
//! deinterleaved stream is a cyclic permutation of the right one and the trellis
//! is invisible however good the signal is.
//!
//! So this one sweeps it: for each branch, the commutator is advanced by `p·32`
//! values before the recording is fed, and the first `31·j·32` values of output
//! are discarded so the deinterleaver's zero fill has flushed. The judge is the
//! decoder's own metric against the same values shuffled, as everywhere else.

use sdroxide_dsp::stanag4285::{EqMode, S4285Event, S4285Receiver};
use sdroxide_dsp::stanag4285_fec as fec;

fn read_wav(path: &str) -> (Vec<f32>, f64) {
    let mut reader = hound::WavReader::open(path).expect("open the wav");
    let spec = reader.spec();
    let raw: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.expect("s")).collect(),
        hound::SampleFormat::Int => match spec.bits_per_sample {
            16 => reader.samples::<i16>().map(|s| s.expect("s") as f32 / 32_768.0).collect(),
            _ => reader.samples::<i32>().map(|s| s.expect("s") as f32 / 2_147_483_648.0).collect(),
        },
    };
    let ch = spec.channels.max(1) as usize;
    let audio = if ch == 1 {
        raw
    } else {
        raw.chunks(ch).map(|f| f.iter().sum::<f32>() / ch as f32).collect()
    };
    (audio, spec.sample_rate as f64)
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.is_empty() {
        eprintln!("usage: s4285_align_check <wav> [carrier_hz]");
        std::process::exit(2);
    }
    let (audio, rate) = read_wav(&a[0]);
    let carrier: f64 = a.get(1).and_then(|v| v.parse().ok()).unwrap_or(1800.0);

    let mut rx = S4285Receiver::new(rate, carrier);
    rx.set_eq_mode(EqMode::Auto);
    rx.trace_coded_stream(true);
    let mut frames = 0;
    for chunk in audio.chunks(4096) {
        for ev in rx.process(chunk) {
            if let S4285Event::Frame(_) = ev {
                frames += 1;
            }
        }
    }
    let traced: Vec<f32> = rx.coded_stream().to_vec();
    println!("{}: {frames} frames, {} traced values at {carrier}", a[0], traced.len());
    if traced.len() < 20_000 {
        println!("  too little to judge");
        return;
    }

    let mut best: Vec<(f32, String)> = Vec::new();
    for rate in fec::CodedRate::BPSK {
        // One bit per symbol for the BPSK rates: the trace carries two.
        let soft: Vec<f32> = traced.iter().step_by(2).copied().collect();
        // **The depth is 32x what the receiver ships with.** The standard's long
        // interleave is a ten-second delay: measured in 32-symbol blocks — which is
        // how the independent implementation reads it — 31 rows of `j` blocks gives
        // 31*12*32 = 11 904 symbols, 9.9 s at 1200 symbols per second. Our
        // per-symbol commutator with the same `j` gives 372 symbols, 0.31 s, so the
        // equivalent per-symbol depth is j = 384. Both are tried.
        let j = rate.j();
        for sign in [1.0f32, -1.0] {
            for style in fec::Interleave::ALL {
                for jj in [j, j * 32] {
                    let flush = 31 * jj * 32;
                    for phase in [0usize, 1, 2, 3, 4, 5, 6, 7] {
                        let mut di = fec::ConvInterleaver::deinterleaver_with(jj, style);
                        for _ in 0..phase * 32 {
                            di.push(0.0);
                        }
                        let mut out: Vec<f32> = Vec::with_capacity(soft.len());
                        for v in &soft {
                            out.push(di.push(v * sign));
                        }
                        if out.len() <= flush + 5_000 {
                            continue;
                        }
                        let body = &out[flush..];
                        let pairs = fec::average_pairs(body, rate.repeat());
                        // **The judge is a framed message**, not the metric ratio:
                        // that ratio reads 1.02x on a signal the receiver decodes, so
                        // it cannot separate signal from noise. A message either
                        // closes or it does not.
                        let bits = fec::viterbi_decode(&pairs);
                        let mut fr = fec::MessageFramer::new(rate);
                        let mut found = 0usize;
                        let mut first_len = 0usize;
                        for b in bits {
                            if let Some(m) = fr.push(b) {
                                found += 1;
                                first_len = m.bits.len();
                                if m.closed {
                                    break;
                                }
                            }
                        }
                        if found > 0 {
                            best.push((
                                found as f32,
                                format!(
                                    "{} j{jj} sign {sign:+.0} {} phase {phase} -> {found} message(s), first {first_len} bits",
                                    rate.label(),
                                    style.label()
                                ),
                            ));
                        }
                    }
                }
            }
        }
    }
    best.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap_or(std::cmp::Ordering::Equal));
    println!("  {} branches framed a message:", best.len());
    for (ratio, label) in best.iter().take(12) {
        println!("  {ratio:5.2}x  {label}");
    }
}
