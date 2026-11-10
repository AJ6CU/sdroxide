//! What is this burst? — cross-burst forensics for the traffic nothing decodes.
//!
//! ```text
//! cargo run --release -p sdroxide-ism --example ism_forensics -- \
//!     capture.iq 868880000 2025000 --tile 150000
//! ```
//!
//! # Why this exists rather than another sync-word guess
//!
//! `ism_replay --survey` says where the bursts are and what rate they run at. It
//! cannot say *what they are*, and the obvious next move — try the sync word of
//! every protocol you can think of — is a poor one: it only ever confirms a guess
//! you already had, and on a 16-bit pattern with an error budget it will
//! eventually confirm a wrong one.
//!
//! The strong signal is repetition. These are unattended transmitters that say
//! nearly the same thing over and over, so a handful of bursts from *one* of them
//! gives up its structure without any prior idea of what it is:
//!
//! - **The repeat interval** is a fingerprint on its own. A weather sensor goes
//!   every 45–60 s, a meter every few minutes, a door contact only when a door
//!   moves.
//! - **Bits that never change** across frames are the preamble, the sync word and
//!   the device identity. Bits that change every time are payload and checksum.
//!   Printing which is which is most of a protocol's layout, derived rather than
//!   assumed.
//! - **The longest run of equal bits** says whether the line code is Manchester:
//!   Manchester cannot produce a run of three, so a stream whose longest run is
//!   two is Manchester at half the measured rate, and a decoder written for NRZ
//!   would never have found it.
//!
//! None of that requires knowing the protocol, which is the point — it is what
//! you have when the protocol is exactly what you are missing.
//!
//! # Reading the output
//!
//! Bursts are grouped into emitters by frequency and symbol rate. For each one
//! the tool prints the inventory, the repeat intervals, and then the aligned
//! frames with a stability map underneath: `=` where every frame agrees, `·`
//! where they differ. A long run of `=` at the front is a sync word and an id; a
//! field of `·` after it is the reading.

use std::fs::File;
use std::io::{BufReader, Read};

use sdroxide_dsp::Complex32;
use sdroxide_ism::{BurstReport, Survey};

const CHUNK: usize = 1 << 16;

/// How far apart two bursts may be and still be the same emitter, Hz.
///
/// These transmitters are crystal-referenced and drift by a few kilohertz with
/// temperature, but two *different* devices on the same nominal channel are
/// usually further apart than that.
const SAME_FREQ_HZ: f64 = 30_000.0;

/// Fractional symbol-rate agreement for two bursts to be the same emitter.
const SAME_BAUD: f64 = 0.06;

/// Frames printed per emitter. Enough to see which fields move; more is a wall.
const SHOW_FRAMES: usize = 8;

/// Payload bits printed per frame.
const SHOW_BITS: usize = 48 * 8;

struct Seen {
    at_s: f64,
    freq_hz: f64,
    dev_hz: f32,
    len_ms: f64,
    baud: f64,
    fit: f32,
    /// Sliced at the burst's own measured rate, MSB first from the burst start.
    bits: Vec<u8>,
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let positional: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();
    if positional.len() < 3 {
        eprintln!("usage: ism_forensics <capture.iq> <window-centre-hz> <sample-rate-hz>");
        eprintln!("       [--tile hz] [--threshold dB]");
        std::process::exit(2);
    }
    let path = positional[0].as_str();
    let center: f64 = positional[1].parse().expect("centre frequency");
    let rate: f64 = positional[2].parse().expect("sample rate");
    let flag = |name: &str, default: f64| -> f64 {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1))
            .and_then(|v| v.parse().ok())
            .unwrap_or(default)
    };
    let tile = flag("--tile", sdroxide_ism::probe::TILE_RATE_DEFAULT_HZ);
    let threshold = flag("--threshold", 8.0) as f32;

    let mut survey = Survey::with_tile(center, rate, threshold, tile);
    println!(
        "window {:.4} MHz at {:.3} Msps, {} tiles of {:.0} kHz, threshold {threshold} dB\n",
        center / 1e6,
        rate / 1e6,
        survey.tiles().len(),
        tile / 1e3
    );

    // ── Collect ──
    let f = File::open(path).unwrap_or_else(|e| panic!("cannot open {path}: {e}"));
    let mut r = BufReader::new(f);
    let mut raw = vec![0u8; CHUNK * 8];
    let mut iq: Vec<Complex32> = Vec::with_capacity(CHUNK);
    let mut reports = Vec::new();
    let mut seen: Vec<Seen> = Vec::new();
    let mut samples: u64 = 0;

    loop {
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
            iq.push(Complex32::new(
                f32::from_le_bytes([c[0], c[1], c[2], c[3]]),
                f32::from_le_bytes([c[4], c[5], c[6], c[7]]),
            ));
        }
        samples += iq.len() as u64;
        let at = (samples - iq.len() as u64) as f64 / rate;

        reports.clear();
        survey.push(&iq, &mut reports);
        for b in &reports {
            if let Some(s) = collect(b, at) {
                seen.push(s);
            }
        }
        if got < raw.len() {
            break;
        }
    }

    println!("{:.1} s of capture, {} measurable bursts\n", samples as f64 / rate, seen.len());
    if seen.is_empty() {
        println!("nothing held still long enough to measure a symbol rate.");
        println!("try a wider --tile, or --threshold lower.");
        return;
    }

    // ── Group ──
    let mut groups: Vec<Vec<usize>> = Vec::new();
    for (i, s) in seen.iter().enumerate() {
        let hit = groups.iter_mut().find(|g| {
            let h = &seen[g[0]];
            (h.freq_hz - s.freq_hz).abs() < SAME_FREQ_HZ
                && (h.baud - s.baud).abs() < h.baud * SAME_BAUD
        });
        match hit {
            Some(g) => g.push(i),
            None => groups.push(vec![i]),
        }
    }
    groups.sort_by_key(|g| std::cmp::Reverse(g.len()));

    for (n, g) in groups.iter().enumerate() {
        report_emitter(n + 1, g, &seen);
    }
}

/// Fraction of aligned bits that may differ and still be the same message shape.
///
/// Two frames from one sensor differ only in the fields it is varying — a
/// counter, a reading, the checksum that follows them — which is a small part of
/// a frame. Two *different* devices sharing a sync word differ in their identity
/// as well, and that shows up as a much larger fraction.
const SAME_SHAPE: f64 = 0.22;

/// Split one emitter's frames into message shapes.
///
/// Grouping by frequency and symbol rate finds a *protocol*, not a device: three
/// sensors of the same make in one house are one group, and averaging their
/// frames together produces a stability map where nothing is stable and a repeat
/// interval that is the interleaving of three unrelated clocks. Splitting on how
/// much the frames actually differ separates them without needing to know where
/// the protocol keeps its device id.
fn shapes(aligned: &[&[u8]], width: usize) -> Vec<Vec<usize>> {
    let mut out: Vec<Vec<usize>> = Vec::new();
    for (i, a) in aligned.iter().enumerate() {
        let hit = out.iter_mut().find(|s| {
            let h = aligned[s[0]];
            let diff = (0..width).filter(|&k| h[k] != a[k]).count();
            (diff as f64) < width as f64 * SAME_SHAPE
        });
        match hit {
            Some(s) => s.push(i),
            None => out.push(vec![i]),
        }
    }
    out
}

/// Pull one burst's measurements and its bits out of a survey report.
///
/// Bursts with no measurable symbol rate are dropped rather than grouped: a
/// chirp has no symbol rate to measure, and averaging a meaningless figure into
/// an emitter's profile would only corrupt it. The survey line already reports
/// them.
fn collect(b: &BurstReport, at: f64) -> Option<Seen> {
    let (baud, fit) = b.measured_baud?;
    Some(Seen {
        at_s: at,
        freq_hz: b.freq_hz,
        dev_hz: b.dev_hz,
        len_ms: b.len_ms,
        baud,
        fit,
        bits: to_bits(&parse_bits_hex(&b.bits_hex)),
    })
}

/// The normal-sense byte dump out of [`BurstReport::bits_hex`].
///
/// Only that sense: the inverted dump is its bitwise complement, so it has the
/// same run lengths and the same stability map, and everything this tool
/// measures about structure is identical in both. The complement is printed
/// beside each frame instead, since which one is "right" is a fact about the
/// device that nothing here can settle.
fn parse_bits_hex(s: &str) -> Vec<u8> {
    let body = s.strip_prefix("bits ").unwrap_or(s);
    let body = body.split("inv").next().unwrap_or(body);
    body.split_whitespace().filter_map(|t| u8::from_str_radix(t, 16).ok()).collect()
}

fn to_bits(bytes: &[u8]) -> Vec<u8> {
    let mut v = Vec::with_capacity(bytes.len() * 8);
    for &b in bytes {
        for k in (0..8).rev() {
            v.push((b >> k) & 1);
        }
    }
    v
}

fn pack(bits: &[u8]) -> Vec<u8> {
    bits.chunks(8)
        .map(|c| c.iter().enumerate().fold(0u8, |a, (i, &b)| a | (b << (7 - i))))
        .collect()
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ")
}

/// Where the leading run of alternating bits stops.
///
/// Every protocol in this band opens with one so its receiver can find the
/// symbol clock. It is only a *starting* guess at the alignment, though — a sync
/// word often begins with alternating bits of its own, so this lands a bit or
/// two late as often as not, and a bit or two is the difference between two
/// frames lining up and looking like different protocols. See [`align`].
fn preamble_end(bits: &[u8]) -> usize {
    let mut breaks = 0;
    let mut i = 1;
    while i < bits.len() {
        if bits[i] == bits[i - 1] {
            breaks += 1;
            if breaks > 1 {
                // Back up to the start of this second run of equal bits: that
                // pair is the first thing which is not preamble.
                return i - 1;
            }
        }
        i += 1;
    }
    bits.len()
}

/// Bits of derived sync word to align the frames of one emitter on.
const ANCHOR_BITS: usize = 32;
/// Bit errors tolerated when matching it. These bursts arrive at 10–20 dB and
/// slip bits; an exact match would drop the weak frames, which are most of them.
const ANCHOR_ERRORS: u32 = 4;

/// Line every frame of one emitter up on a sync word *derived from the frames
/// themselves*.
///
/// This is the part that makes the stability map mean anything. The preamble
/// heuristic lands within a bit or two, and a bit or two of misalignment makes
/// two copies of the same frame disagree everywhere — which reads as "nothing is
/// stable" and hides exactly the structure the tool is looking for.
///
/// So: take one frame's first [`ANCHOR_BITS`] after its preamble as a candidate
/// sync word, find the best-matching offset for that pattern in every frame, then
/// rebuild the pattern from a majority vote of what actually lined up and do it
/// once more. The second pass matters because the first frame may itself have
/// been a bit out, and the vote pulls the anchor onto whatever the emitter really
/// transmits. Nothing here assumes any particular protocol — the sync word is an
/// output, not an input.
fn align(frames: &[&[u8]]) -> Vec<usize> {
    let seed = frames
        .iter()
        .find(|f| preamble_end(f) + ANCHOR_BITS < f.len())
        .map(|f| {
            let s = preamble_end(f);
            f[s..s + ANCHOR_BITS].to_vec()
        })
        .unwrap_or_default();
    if seed.is_empty() {
        return frames.iter().map(|f| preamble_end(f)).collect();
    }

    let mut anchor = seed;
    let mut offsets: Vec<usize> = Vec::new();
    for _ in 0..2 {
        offsets = frames.iter().map(|f| best_offset(f, &anchor)).collect();
        // Majority vote at the aligned position, so the anchor converges on what
        // the emitter sends rather than on whichever frame happened to be first.
        let mut voted = vec![0u8; ANCHOR_BITS];
        for k in 0..ANCHOR_BITS {
            let ones = frames
                .iter()
                .zip(&offsets)
                .filter(|&(f, &o)| o + k < f.len() && f[o + k] == 1)
                .count();
            voted[k] = u8::from(ones * 2 >= frames.len());
        }
        anchor = voted;
    }
    offsets
}

/// Where `anchor` best matches in `bits`, preferring the earliest good match.
fn best_offset(bits: &[u8], anchor: &[u8]) -> usize {
    let fallback = preamble_end(bits);
    if bits.len() < anchor.len() {
        return fallback;
    }
    let mut best = (u32::MAX, fallback);
    for o in 0..=bits.len() - anchor.len() {
        let errs = anchor.iter().zip(&bits[o..]).filter(|(a, b)| a != b).count() as u32;
        if errs < best.0 {
            best = (errs, o);
            if errs == 0 {
                break;
            }
        }
    }
    if best.0 <= ANCHOR_ERRORS { best.1 } else { fallback }
}

/// Longest run of equal bits, which is what says whether the line code could be
/// Manchester.
fn max_run(bits: &[u8]) -> usize {
    let (mut best, mut run) = (0, 0);
    for i in 0..bits.len() {
        if i > 0 && bits[i] == bits[i - 1] {
            run += 1;
        } else {
            run = 1;
        }
        best = best.max(run);
    }
    best
}

fn report_emitter(n: usize, g: &[usize], seen: &[Seen]) {
    let m = &seen[g[0]];
    let bauds: Vec<f64> = g.iter().map(|&i| seen[i].baud).collect();
    let baud = bauds.iter().sum::<f64>() / bauds.len() as f64;
    let devs: Vec<f32> = g.iter().map(|&i| seen[i].dev_hz).collect();
    let dev = devs.iter().sum::<f32>() / devs.len() as f32;
    let lens: Vec<f64> = g.iter().map(|&i| seen[i].len_ms).collect();
    let fmin = g.iter().map(|&i| seen[i].freq_hz).fold(f64::MAX, f64::min);
    let fmax = g.iter().map(|&i| seen[i].freq_hz).fold(f64::MIN, f64::max);

    println!("══ emitter {n} ── {} bursts ─────────────────────────────", g.len());
    println!(
        "   {:.4}–{:.4} MHz   {:.0} baud   dev {:.1} kHz   {:.1}–{:.1} ms   fit {:.2}",
        fmin / 1e6,
        fmax / 1e6,
        baud,
        dev / 1e3,
        lens.iter().cloned().fold(f64::MAX, f64::min),
        lens.iter().cloned().fold(f64::MIN, f64::max),
        m.fit,
    );

    // Repeat interval: the single most identifying thing about an unattended
    // transmitter, and free.
    if g.len() > 1 {
        let mut gaps: Vec<f64> = g.windows(2).map(|w| seen[w[1]].at_s - seen[w[0]].at_s).collect();
        gaps.sort_by(f64::total_cmp);
        let shown: Vec<String> = gaps.iter().map(|s| format!("{s:.1}")).collect();
        println!("   repeats after: {} s", shown.join(", "));
    }

    // The burst as it came off the slicer, before anything was stripped. This is
    // where a preamble's own shape is visible, and it is what to paste into a
    // search when the derived sync word means nothing.
    println!("\n   as sliced, from the start of the signal:");
    for &i in g.iter().take(2) {
        println!("     {}", hex(&pack(&seen[i].bits)));
    }

    // Aligned frames, and which bits hold still.
    let raw: Vec<&[u8]> = g.iter().map(|&i| seen[i].bits.as_slice()).collect();
    let offsets = align(&raw);
    let aligned: Vec<&[u8]> =
        raw.iter().zip(&offsets).map(|(f, &o)| &f[o.min(f.len())..]).collect();
    let width = aligned.iter().map(|a| a.len()).min().unwrap_or(0).min(SHOW_BITS);
    if width < 16 {
        println!("   nothing after the preamble to compare\n");
        return;
    }

    // Line code, measured over the aligned frame only. Measuring it over the whole
    // sliced stream would be measuring the tail: the dump runs past the end of the
    // transmission, and over noise the discriminator wanders and produces runs of
    // any length — which reported a run of eleven for a stream that is Manchester
    // throughout its actual burst.
    let run = aligned.iter().map(|a| max_run(&a[..width])).max().unwrap_or(0);
    println!(
        "   longest run of equal bits in the frame: {run}{}",
        if run <= 2 {
            format!("  ← Manchester; the data rate is {:.0} baud", baud / 2.0)
        } else {
            String::new()
        }
    );

    // Split into message shapes, so three devices behind one sync word are not
    // averaged into a map where nothing holds still.
    let by_shape = shapes(&aligned, width);
    println!(
        "\n   {} message shape{} behind this sync word",
        by_shape.len(),
        if by_shape.len() == 1 { "" } else { "s" }
    );

    for (k, s) in by_shape.iter().enumerate() {
        println!("\n   ── shape {} of {} ── {} frames", k + 1, by_shape.len(), s.len());

        // This shape's own repeat interval, which is the one that means anything.
        if s.len() > 1 {
            let times: Vec<f64> = s.iter().map(|&i| seen[g[i]].at_s).collect();
            let mut gaps: Vec<f64> = times.windows(2).map(|w| w[1] - w[0]).collect();
            gaps.sort_by(f64::total_cmp);
            // A gap under a second is the same transmission sent again for
            // reliability, not the device's own clock; they are counted rather
            // than listed so the period is readable.
            let bursts = gaps.iter().filter(|g| **g < 1.0).count();
            let period: Vec<String> =
                gaps.iter().filter(|g| **g >= 1.0).map(|s| format!("{s:.1}")).collect();
            if bursts > 0 {
                println!("      {bursts} immediate repeats (under 1 s — one event sent twice)");
            }
            if !period.is_empty() {
                println!("      then every: {} s", period.join(", "));
            }
        }

        // Trimmed to the common width for the comparison above, then in full —
        // the trailing bytes are where a checksum lives, and the common width is
        // set by the shortest frame in the emitter, which may not be this shape's.
        println!("      frames (normal sense | inverted):");
        for &i in s.iter().take(SHOW_FRAMES) {
            let b = pack(&aligned[i][..width]);
            let inv: Vec<u8> = b.iter().map(|x| !*x).collect();
            println!("        {}   |   {}", hex(&b), hex(&inv));
        }
        println!("      the same frames in full, to the end of the burst:");
        for &i in s.iter().take(SHOW_FRAMES) {
            println!("        {}", hex(&pack(aligned[i])));
        }
        if s.len() > SHOW_FRAMES {
            println!("        … {} more", s.len() - SHOW_FRAMES);
        }

        if s.len() < 2 {
            continue;
        }
        let mut map = String::new();
        for bit in 0..width {
            let ones = s.iter().filter(|&&i| aligned[i][bit] == 1).count();
            map.push(if ones == 0 || ones == s.len() { '=' } else { '·' });
            if bit % 8 == 7 {
                map.push(' ');
            }
        }
        let constant = map.chars().filter(|c| *c == '=').count();
        println!("      what holds still (= same in every frame, · differs):");
        for (line, chunk) in map.as_bytes().chunks(9 * 8).enumerate() {
            println!("        bit {:3}  {}", line * 64, String::from_utf8_lossy(chunk));
        }
        println!("      {constant} of {width} bits never change");
    }
    println!();
}
