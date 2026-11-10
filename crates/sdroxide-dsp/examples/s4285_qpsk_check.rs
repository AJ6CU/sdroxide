//! Is a recording carrying STANAG 4285 with Annex E coding, judged on **two**
//! bits per symbol?
//!
//!     cargo run --release -p sdroxide-dsp --example s4285_qpsk_check -- \
//!         ~/s4285-captures/keepers/….wav [carrier_hz]
//!
//! `s4285_fec_check` judges one soft decision per symbol, which is right for the
//! BPSK rates and wrong for the QPSK one: a QPSK symbol carries two bits, and a
//! judge that keeps only the in-phase part is asking whether half a trellis is
//! there. This one takes both, and sweeps the three things the annex does not pin
//! down in the texts this fork was built from:
//!
//! * **which axis pair the bits live on** — the four phases sit at 0/90/180/270,
//!   so the bits are on the ±45° diagonals, `(I+Q, I−Q)`, not on I and Q;
//! * **which of the two enters the code first**;
//! * the interleaver's commutator wiring and depth, as the other check does.
//!
//! The verdict is the decoder's own metric against **the same values shuffled**,
//! which is the only null that answers this question and nothing else.

use sdroxide_dsp::stanag4285::{EqMode, S4285Receiver};
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

fn shuffled(input: &[f32], seed: u32) -> Vec<f32> {
    let mut out = input.to_vec();
    let mut s = seed;
    for i in (1..out.len()).rev() {
        s ^= s << 13;
        s ^= s >> 17;
        s ^= s << 5;
        let j = (s as usize) % (i + 1);
        out.swap(i, j);
    }
    out
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.is_empty() {
        eprintln!("usage: s4285_qpsk_check <wav> [carrier_hz]");
        std::process::exit(2);
    }
    let (audio, rate) = read_wav(&a[0]);
    let carrier: f64 = a.get(1).and_then(|v| v.parse().ok()).unwrap_or(1800.0);

    let mut rx = S4285Receiver::new(rate, carrier);
    rx.set_eq_mode(EqMode::Auto);
    rx.trace_coded_stream(true);
    for chunk in audio.chunks(4096) {
        rx.process(chunk);
    }
    let traced: Vec<f32> = rx.coded_stream().to_vec();
    // The trace carries two values per data symbol, so the number of symbols is
    // half of it and both bits are present for the QPSK hypothesis.
    println!("{}: {} values = {} symbols at {}", a[0], traced.len(), traced.len() / 2, carrier);
    if traced.len() < 20_000 {
        println!("  too little to judge");
        return;
    }

    // The two axis conventions: I/Q as they arrive, and the ±45° diagonals the
    // 0/90/180/270 phases actually put the bits on.
    let mut axes: Vec<(&str, Vec<f32>)> = Vec::new();
    axes.push(("diagonal", {
        let mut v = Vec::with_capacity(traced.len());
        for p in traced.chunks_exact(2) {
            v.push((p[0] + p[1]) * std::f32::consts::FRAC_1_SQRT_2);
            v.push((p[0] - p[1]) * std::f32::consts::FRAC_1_SQRT_2);
        }
        v
    }));
    axes.push(("IQ", traced.clone()));

    let mut best: Vec<(f32, String)> = Vec::new();
    for (axis, src) in &axes {
        for swap in [false, true] {
            let ordered: Vec<f32> = if swap {
                let mut v = Vec::with_capacity(src.len());
                for p in src.chunks_exact(2) {
                    v.push(p[1]);
                    v.push(p[0]);
                }
                v
            } else {
                src.clone()
            };
            for sign in [1.0f32, -1.0] {
                for style in fec::Interleave::ALL {
                    for j in [12, 24, 48, 60, 120, 240, 480] {
                        let mut di = fec::ConvInterleaver::deinterleaver_with(j, style);
                        let stream: Vec<f32> = ordered.iter().map(|v| di.push(v * sign)).collect();
                        // One coded pair per QPSK symbol: no repetition to average.
                        let pairs = fec::average_pairs(&stream, 1);
                        let (_, metric) = fec::viterbi_decode_with_metric(&pairs);
                        let null = [shuffled(&pairs, 0x1234_5678), shuffled(&pairs, 0x9E37_79B9)]
                            .iter()
                            .map(|s| fec::viterbi_decode_with_metric(s).1)
                            .fold(f32::NEG_INFINITY, f32::max);
                        let ratio = if null > 0.0 { metric / null } else { 0.0 };
                        best.push((
                            ratio,
                            format!("{axis} swap={swap} sign {sign:+.0} {} j{j}", style.label()),
                        ));
                    }
                }
            }
        }
    }
    best.sort_by(|x, y| y.0.partial_cmp(&x.0).unwrap_or(std::cmp::Ordering::Equal));
    println!("  best twelve of {} branches (a ratio near 1 is no coding):", best.len());
    for (ratio, label) in best.iter().take(12) {
        println!("  {ratio:5.2}x  {label}");
    }
}
