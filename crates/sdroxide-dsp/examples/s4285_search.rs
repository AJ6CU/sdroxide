//! Search the conventions a loopback cannot falsify, against a known SOM.
//!
//!     cargo run --release -p sdroxide-dsp --example s4285_search -- tape.soft [j] [times] [phase]
//!
//! The input is the **BPSK soft stream** (one value per data symbol, 128 per frame
//! — `S4285_CODED_OUT` writes two per symbol, so take every second one).
//!
//! Every convention the chain has is shared between the encoder and the decoder,
//! so a loopback proves nothing about it. This sweeps the ones a real signal can
//! tell apart, and judges by the only test that has ever been validated here: the
//! 32-bit start-of-message pattern appearing in the decoded bits, in any of the
//! four bit orders, at any alignment. A permutation of the frame's data blocks
//! would leave the constellation intact — it is rotation- and order-invariant —
//! and destroy the code, which is exactly the shape of what the tape does.
use sdroxide_dsp::stanag4285_fec::{self as fec, ConvInterleaver, Interleave};

const SOM: u32 = 0x0387_3C3C;
const EOM: u32 = 0x4B65_A5B2;

fn find(bits: &[u8], pat: u32) -> Option<usize> {
    let forms = [pat, pat.reverse_bits(), !pat, (!pat).reverse_bits()];
    let mut v = 0u32;
    for (i, b) in bits.iter().enumerate() {
        v = (v << 1) | *b as u32;
        if i >= 31 && forms.contains(&v) {
            return Some(i - 31);
        }
    }
    None
}

fn permutations() -> Vec<[usize; 4]> {
    let mut out = Vec::new();
    for a in 0..4 {
        for b in 0..4 {
            for c in 0..4 {
                for d in 0..4 {
                    if a != b && a != c && a != d && b != c && b != d && c != d {
                        out.push([a, b, c, d]);
                    }
                }
            }
        }
    }
    out
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.is_empty() {
        eprintln!("usage: s4285_search <bpsk-soft.txt> [j] [times] [phase]");
        std::process::exit(2);
    }
    let j: usize = a.get(1).and_then(|v| v.parse().ok()).unwrap_or(12);
    let times: usize = a.get(2).and_then(|v| v.parse().ok()).unwrap_or(1);
    let phase: usize = a.get(3).and_then(|v| v.parse().ok()).unwrap_or(0);
    let style = Interleave::ALL
        [a.get(4).and_then(|v| v.parse::<usize>().ok()).unwrap_or(0) % Interleave::ALL.len()];
    let text = std::fs::read_to_string(&a[0]).expect("read the soft stream");
    let all: Vec<f32> = text.lines().filter_map(|l| l.trim().parse::<f32>().ok()).collect();
    let soft: Vec<f32> = all.iter().step_by(2).copied().collect();
    let frames: Vec<&[f32]> = soft.chunks_exact(128).collect();
    eprintln!(
        "{} frames of 128, j={j} times={times} phase={phase} style={}",
        frames.len(),
        style.label()
    );

    let mut hits = 0usize;
    let mut tried = 0usize;
    for perm in permutations() {
        for rev in [false, true] {
            for swap in [false, true] {
                for diff in [false, true] {
                    let mut flat: Vec<f32> = Vec::with_capacity(soft.len());
                    for f in &frames {
                        for p in perm {
                            let mut v = f[p * 32..p * 32 + 32].to_vec();
                            if rev {
                                v.reverse();
                            }
                            flat.extend(v);
                        }
                    }
                    // **Differentially encoded?** A PSK link that needs no absolute
                    // phase reference sends the *product* of adjacent symbols, and then
                    // every one of our measurements still reads correctly — reference
                    // blocks collapse, the eyes sits on the lattice, the constellation
                    // is unchanged — while the bits we hand the trellis are the running
                    // integral of the real ones.
                    if diff {
                        let mut prev = 1.0f32;
                        for v in flat.iter_mut() {
                            let d = if *v * prev >= 0.0 { 1.0 } else { -1.0 };
                            prev = *v;
                            *v = d;
                        }
                    }
                    let rot: Vec<f32> = if phase == 0 {
                        flat.clone()
                    } else {
                        flat[phase..].iter().chain(flat[..phase].iter()).copied().collect()
                    };
                    let mut de = ConvInterleaver::deinterleaver_with(j, style);
                    let deint: Vec<f32> = rot.iter().map(|v| de.push(*v)).collect();
                    let paired = fec::average_pairs(&deint, times);
                    let paired: Vec<f32> = if swap {
                        paired.chunks_exact(2).flat_map(|p| [p[1], p[0]]).collect()
                    } else {
                        paired
                    };
                    tried += 1;
                    // Chunked like the receiver: the first 64 bits of each run are the
                    // part the forced start state distorts, so they are dropped.
                    let mut bits = Vec::new();
                    let chunk = 512;
                    for c in paired.chunks(chunk) {
                        if c.len() < chunk {
                            break;
                        }
                        bits.extend(fec::viterbi_decode(c).into_iter().skip(64));
                    }
                    if let Some(i) = find(&bits, SOM) {
                        hits += 1;
                        println!(
                            "SOM j={j} times={times} phase={phase} perm={perm:?} rev={rev} swap={swap} diff={diff} at bit {i} ({} bits)",
                            bits.len()
                        );
                    }
                    if let Some(i) = find(&bits, EOM) {
                        hits += 1;
                        println!(
                            "EOM j={j} times={times} phase={phase} perm={perm:?} rev={rev} swap={swap} diff={diff} at bit {i} ({} bits)",
                            bits.len()
                        );
                    }
                }
            }
        }
    }
    println!("{tried} combinations at j={j} times={times} phase={phase}, {hits} hits");
}
