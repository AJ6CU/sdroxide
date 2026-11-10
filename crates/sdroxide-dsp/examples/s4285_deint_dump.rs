//! Dump this crate's deinterleaver output for a soft-decision stream, so it can be
//! compared with an independent implementation's.
//!
//!     S4285_CODED_OUT=ours.soft cargo run --release -p sdroxide-dsp \
//!         --example s4285_probe -- <wav> 1800
//!     cargo run --release -p sdroxide-dsp --example s4285_deint_dump -- \
//!         ours.soft 12 out.txt
//!
//! Arguments: `<soft> [j] [out]`. The soft values are one per line, positive for a
//! zero — what `S4285_CODED_OUT` writes and what the Annex E chains are fed.
//!
//! **Why this exists.** Annex E's interleaver is the one part of the chain a
//! loopback cannot check: the encoder and the decoder share whatever convention
//! they use, so a pair that round-trips perfectly through itself can still be
//! wiring the rows the other way round from a real transmitter. The only honest
//! test is the same soft decisions through a second implementation, and then the
//! streams side by side. The independent decoder stops at the decoded bit stream
//! and has no framing, so its *bits* cannot say whether a message was framed; its
//! deinterleaver output can say whether ours is the same machine.
use std::io::Write;

use sdroxide_dsp::stanag4285_fec::ConvInterleaver;

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.is_empty() {
        eprintln!("usage: s4285_deint_dump <soft.txt> [j] [out.txt]");
        std::process::exit(2);
    }
    let j: usize = a.get(1).and_then(|v| v.parse().ok()).unwrap_or(12);
    let text = std::fs::read_to_string(&a[0]).expect("read the soft stream");
    let soft: Vec<f32> = text.lines().filter_map(|l| l.trim().parse::<f32>().ok()).collect();
    let mut de = ConvInterleaver::deinterleaver(j);
    let out: Vec<f32> = soft.iter().map(|v| de.push(*v)).collect();
    match a.get(2) {
        Some(path) => {
            let mut f = std::fs::File::create(path).expect("create the dump");
            for v in &out {
                writeln!(f, "{v:.6}").expect("write");
            }
            println!("{} values from {} j={} -> {}", out.len(), soft.len(), j, path);
        }
        None => {
            for v in &out {
                println!("{v:.6}");
            }
        }
    }
}
