//! Replay a recorded IQ capture through the ISM decoder and report every burst.
//!
//! This is the tool that answers the only question that matters about a decoder
//! like this — *does it work on your band* — and the one before it: what is
//! actually on your band. Every transmission the gate opens on is printed with
//! what was measured from it and whether anything could read it, including the
//! ones nothing could. A burst with a plausible symbol rate, a sane deviation and
//! a few candidate frames is a device worth writing a protocol module for.
//!
//! ```text
//! cargo run --release -p sdroxide-ism --example ism_replay -- capture.iq 868880000 2025000
//! cargo run --release -p sdroxide-ism --example ism_replay -- capture.iq 868880000 2025000 --threshold 8
//! cargo run --release -p sdroxide-ism --example ism_replay -- capture.iq 866900000 2025000 --survey
//! ```
//!
//! `--survey` stops listening to the channel plan and tiles the **whole window**
//! instead, reporting every burst anywhere in it and finishing with a histogram of
//! where they were. That is the mode to reach for first: a neighbourhood whose
//! traffic actually sits at 866.5 MHz — the RFID part of the band, which no sensor
//! protocol uses — looks completely silent to the channel decoders and looks busy
//! here. Find out where the transmissions are before asking what they are.
//!
//! The file is interleaved little-endian `complex64` — two `f32`s per sample —
//! which is what sdroxide's own file source reads and writes, so a capture made
//! with the radio replays here unchanged.
//!
//! # Making a capture on an RX-888
//!
//! Its VHF path reaches 868 MHz through the tuner and hands the engine the
//! wideband downconverter's 2.025 Msps output, so those are the numbers above.
//! Tune to 868.880 MHz — the centre that fits the whole channel plan inside the
//! flat part of that window — and record.

use std::fs::File;
use std::io::{BufReader, Read};

use sdroxide_dsp::Complex32;
use sdroxide_ism::{BurstReport, Probe, Survey};

/// Either view of a capture: the channel plan, or the whole window.
enum Ears {
    Channels(Box<Probe>),
    Survey(Box<Survey>),
}

impl Ears {
    fn push(&mut self, iq: &[Complex32], out: &mut Vec<BurstReport>) {
        match self {
            Ears::Channels(p) => p.push(iq, out),
            Ears::Survey(s) => s.push(iq, out),
        }
    }
}

/// Samples per read. Large enough that the per-block overhead disappears, small
/// enough that a long capture does not have to fit in memory.
const CHUNK: usize = 1 << 16;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let positional: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    if positional.len() < 3 {
        eprintln!(
            "usage: ism_replay <capture.iq> <window-centre-hz> <sample-rate-hz> [--threshold dB]"
        );
        eprintln!("       the file is interleaved little-endian f32 I/Q pairs");
        std::process::exit(2);
    }
    let path = positional[0];
    let center: f64 = positional[1].parse().expect("window centre must be a number in Hz");
    let rate: f64 = positional[2].parse().expect("sample rate must be a number in Hz");
    let threshold: f32 = args
        .iter()
        .position(|a| a == "--threshold")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(12.0);

    let survey = args.iter().any(|a| a == "--survey");
    // Narrow tiles for hunting a narrow signal; see `TILE_RATE_DEFAULT_HZ`.
    let tile: f64 = args
        .iter()
        .position(|a| a == "--tile")
        .and_then(|i| args.get(i + 1))
        .and_then(|v| v.parse().ok())
        .unwrap_or(sdroxide_ism::probe::TILE_RATE_DEFAULT_HZ);

    println!("window {:.4} MHz at {:.3} Msps, threshold {threshold} dB", center / 1e6, rate / 1e6);

    let mut ears = if survey {
        let s = Survey::with_tile(center, rate, threshold, tile);
        let tiles = s.tiles();
        match tiles.first().zip(tiles.last()) {
            Some((lo, hi)) => {
                println!(
                    "survey: {} tiles of {:.0} kHz across {:.3}–{:.3} MHz",
                    tiles.len(),
                    tile / 1e3,
                    lo / 1e6,
                    hi / 1e6
                )
            }
            None => {
                println!("the window is too narrow to survey");
                return;
            }
        }
        Ears::Survey(Box::new(s))
    } else {
        let probe = Probe::new(center, rate, threshold);
        let channels = probe.channels();
        if channels.is_empty() {
            // The honest failure, and the common one: a capture made where the
            // channels are not.
            println!(
                "no ISM channel is inside this window — the plan needs {:.3} MHz centred near \
                 {:.4} MHz. Try --survey to see what is here instead.",
                sdroxide_ism::span_hz() / 1e6,
                sdroxide_ism::ideal_center_hz() / 1e6
            );
            return;
        }
        print!("listening on");
        for c in &channels {
            print!(" {:.3}", c / 1e6);
        }
        println!(" MHz");
        Ears::Channels(Box::new(probe))
    };
    println!();

    let f = File::open(path).unwrap_or_else(|e| panic!("cannot open {path}: {e}"));
    let mut r = BufReader::new(f);
    let mut raw = vec![0u8; CHUNK * 8];
    let mut iq: Vec<Complex32> = Vec::with_capacity(CHUNK);
    let mut reports = Vec::new();

    let mut samples: u64 = 0;
    let mut bursts: u64 = 0;
    let mut decodes: u64 = 0;
    // Measured frequency of every burst, for the closing histogram.
    let mut busy: Vec<f64> = Vec::new();

    loop {
        // A partial read is normal on a pipe, and a capture may not end on a
        // sample boundary; keep whole samples and drop any trailing odd bytes.
        let mut got = 0usize;
        while got < raw.len() {
            match r.read(&mut raw[got..]) {
                Ok(0) => break,
                Ok(n) => got += n,
                Err(e) => panic!("reading {path}: {e}"),
            }
        }
        if got < 8 {
            break;
        }
        iq.clear();
        for c in raw[..got - got % 8].chunks_exact(8) {
            let re = f32::from_le_bytes([c[0], c[1], c[2], c[3]]);
            let im = f32::from_le_bytes([c[4], c[5], c[6], c[7]]);
            iq.push(Complex32::new(re, im));
        }
        samples += iq.len() as u64;

        reports.clear();
        ears.push(&iq, &mut reports);
        for b in &reports {
            bursts += 1;
            decodes += b.decoded.len() as u64;
            busy.push(b.freq_hz);
            // Where in the capture, so a burst can be found again in a viewer.
            let at = (samples - iq.len() as u64) as f64 / rate;
            println!("{at:8.3}s  {}", b.line());
        }

        if got < raw.len() {
            break;
        }
    }

    println!();
    println!("{:.1} s of capture, {bursts} bursts, {decodes} decoded", samples as f64 / rate);
    if let Ears::Channels(p) = &ears {
        for (hz, floor) in p.floors_dbfs() {
            println!("  {:.3} MHz  noise floor {floor:6.1} dBFS", hz / 1e6);
        }
    }

    // Where the traffic actually was, which is the answer a survey is run for.
    if survey && bursts > 0 {
        let mut hist: Vec<(f64, u64)> = Vec::new();
        for f in &busy {
            // Quarter-megahertz bins: fine enough to separate the parts of the
            // band that carry different things, coarse enough to read down a page.
            let bin = (f / 250_000.0).floor() * 250_000.0;
            match hist.iter_mut().find(|(b, _)| *b == bin) {
                Some((_, n)) => *n += 1,
                None => hist.push((bin, 1)),
            }
        }
        hist.sort_by(|a, b| a.0.total_cmp(&b.0));
        let peak = hist.iter().map(|(_, n)| *n).max().unwrap_or(1).max(1);
        println!();
        println!("where the traffic was:");
        for (bin, n) in hist {
            let bar = "#".repeat(((n * 50 / peak) as usize).max(1));
            println!("  {:.3}–{:.3} MHz  {n:5}  {bar}", bin / 1e6, (bin + 250_000.0) / 1e6);
        }
    }

    if bursts > 0 && decodes == 0 {
        println!();
        println!(
            "Bursts but no decodes. The `cand` column is the tell — a burst with candidate \
             frames is the right modulation and a frame format this build does not know, one \
             with none is a modulation this chain does not handle at all (LoRa chirps look \
             like that). The `dev` and `baud` columns say what to write next."
        );
    }
}
