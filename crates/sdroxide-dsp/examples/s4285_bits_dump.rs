//! Decode a soft-decision stream with this crate's Viterbi, in the independent
//! implementation's chunking, and print the bits.
//!
//!     cargo run --release -p sdroxide-dsp --example s4285_bits_dump -- \
//!         ours.soft 256 0 1
//!
//! Arguments: `<soft> [chunk_symbols] [swapped] [repeat]`. `repeat` is the coded
//! rate's pair repetition (1, 2, 4 or 8 for 600/300/150/75 bps), averaged on the
//! deinterleaved values exactly as the receiver does it. `chunk_symbols` is how many soft
//! decisions one Viterbi run consumes (the reference decoder uses 256 = 128 pairs);
//! `swapped` feeds each pair as (T2, T1) instead of (T1, T2), which is the one
//! convention a loopback can never falsify — our encoder and decoder share it.
//!
//! **Why this exists.** With the deinterleaver proved identical to the independent
//! implementation (`s4285_deint_dump`), the remaining question is whether the
//! trellis consumes the deinterleaved pairs the way the standard means. Comparing
//! our bits with theirs, pair for pair, answers it as a fact rather than as a
//! sweep over guesses.
use sdroxide_dsp::stanag4285_fec::{self as fec, ConvInterleaver};

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.is_empty() {
        eprintln!("usage: s4285_bits_dump <soft.txt> [chunk] [swapped]");
        std::process::exit(2);
    }
    let chunk: usize = a.get(1).and_then(|v| v.parse().ok()).unwrap_or(256);
    let swapped = a.get(2).is_some_and(|v| v == "1");
    let times: usize = a.get(3).and_then(|v| v.parse().ok()).unwrap_or(1);
    let text = std::fs::read_to_string(&a[0]).expect("read the soft stream");
    // **The trace is two values per symbol** — the in-phase and the quadrature
    // parts, because that is what the QPSK chain is fed — so a BPSK rate's stream
    // is every second value of it. Taking the trace whole feeds re,im,re,im… to a
    // one-bit-per-symbol deinterleaver, which is how the first version of this tool
    // produced 50 % nonsense on the crate's own clean signal.
    let all: Vec<f32> = text.lines().filter_map(|l| l.trim().parse::<f32>().ok()).collect();
    let soft: Vec<f32> = all.iter().step_by(2).copied().collect();
    let mut de = ConvInterleaver::deinterleaver(12);
    let deint: Vec<f32> = soft.iter().map(|v| de.push(*v)).collect();
    // The lower rates average the repeated pairs on the deinterleaved stream.
    let deint: Vec<f32> = if times > 1 { fec::average_pairs(&deint, times) } else { deint };
    let mut bits = Vec::new();
    for c in deint.chunks(chunk) {
        if c.len() < chunk {
            break;
        }
        let paired: Vec<f32> = if swapped {
            c.chunks_exact(2).flat_map(|p| [p[1], p[0]]).collect()
        } else {
            c.to_vec()
        };
        bits.extend(fec::viterbi_decode(&paired));
    }
    let mut line = String::with_capacity(bits.len());
    for b in &bits {
        line.push(if *b == 1 { '1' } else { '0' });
    }
    println!("{line}");
    eprintln!("{} soft -> {} deinterleaved -> {} bits", soft.len(), deint.len(), bits.len());
}
