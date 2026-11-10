//! Is a recording carrying STANAG 4285 with Annex E coding, or without it?
//!
//!     cargo run --release -p sdroxide-dsp --example s4285_fec_check -- \
//!         ~/s4285-captures/6348.000kHz-20261022-045545Z.wav [carrier_hz]
//!
//! The question a constellation cannot answer. Every uncoded rate shares its
//! modulation with a coded one — 1200 bps BPSK is also 600 at rate 1/2, 300 at
//! 1/4, 150 at 1/8 and 75 at 1/16 — so the only evidence is whether the stream
//! satisfies the code's trellis after deinterleaving.
//!
//! The test is the decoder's own metric against **the same values shuffled**.
//! Shuffling destroys the trellis and nothing else, so it is the null for exactly
//! this question: a stream that carries the code scores far above it, and one that
//! does not scores the same. Both are printed, beside each other, because a
//! statistic without its own null is how this mode's earlier work lost a day.

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
    // A capture can be stereo, and its samples are then interleaved: read as
    // mono, the rate is effectively halved and two channels are spliced into one
    // stream. The real coded capture we were sent was stereo. See `s4285_probe`.
    let ch = spec.channels.max(1) as usize;
    let audio = if ch == 1 {
        raw
    } else {
        raw.chunks(ch).map(|f| f.iter().sum::<f32>() / ch as f32).collect()
    };
    (audio, spec.sample_rate as f64)
}

/// A xorshift, so the null is reproducible and is not the statistics of `rand`.
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

/// Every branch's metric against its own shuffled null, as
/// `(rate label, style, depth, metric, null, ratio)`, plus the best of them.
struct Verdict {
    best: Option<(f32, String)>,
    rows: Vec<String>,
}

fn judge(soft: &[f32]) -> Verdict {
    // The stream has to be deinterleaved before the decode means anything, so
    // every rate's `j` is tried — and both signs, since the frame phase is only
    // known up to pi.
    let mut v = Verdict { best: None, rows: Vec::new() };
    for rate in fec::CodedRate::BPSK {
        for sign in [1.0f32, -1.0] {
            for style in fec::Interleave::ALL {
                // **The depth is the other half of the convention, and the annex has a
                // short and a long column.** fldigi and MultiPSK both offer "long
                // interleaving", and our transpose uses the long column's figures —
                // 24 at 1200 and 12 below it. A wrong `j` looks exactly like a wrong
                // commutator: every branch at its null. So the few depths a 600 bps
                // transmission could plausibly use are swept too.
                // **The depths a real transmission may use are the *long*
                // interleave's, and they are an order of magnitude past what this
                // receiver shipped with.** Independent descriptions of the mode —
                // Sigmira's manual and the WAVECOM summary — put the long
                // interleave at about a **ten-second** delay and say virtually all
                // real traffic uses it, while the short one is about a second. With
                // 31 rows, a one-second delay is `j` ≈ 40 and ten seconds is `j` ≈
                // 400, so the shipped 12 and 24 are the short column and a sweep
                // that stopped at 48 never left the short interleave's neighbourhood.
                let depths: Vec<usize> = vec![rate.j(), 60, 120, 240, 480, 960];
                for &j in &depths {
                    let mut di = fec::ConvInterleaver::deinterleaver_with(j, style);
                    // **Two bits per symbol belong to the QPSK rate and one to the
                    // others.** The traced stream carries both, so a BPSK branch
                    // takes the in-phase half of it.
                    let src: Vec<f32> = if rate == fec::CodedRate::Bps1200 {
                        soft.to_vec()
                    } else {
                        soft.iter().step_by(2).copied().collect()
                    };
                    let stream: Vec<f32> = src.iter().map(|v| di.push(v * sign)).collect();
                    let pairs = fec::average_pairs(&stream, rate.repeat());
                    let (_, metric) = fec::viterbi_decode_with_metric(&pairs);
                    // The null: the same values, shuffled. Two shuffles, and the larger
                    // is reported, because the null is what the answer has to beat.
                    let null = [shuffled(&pairs, 0x1234_5678), shuffled(&pairs, 0x9E37_79B9)]
                        .iter()
                        .map(|s| fec::viterbi_decode_with_metric(s).1)
                        .fold(f32::NEG_INFINITY, f32::max);
                    let ratio = if null > 0.0 { metric / null } else { 0.0 };
                    let label = format!("{} j{j} sign {sign:+.0} {}", rate.label(), style.label());
                    if v.best.as_ref().is_none_or(|(b, _)| ratio > *b) {
                        v.best = Some((ratio, label.clone()));
                    }
                    v.rows.push(format!(
                        "  {label:>44}  {metric:>8.0}  {null:>8.0}  {ratio:>5.2}x{}",
                        if ratio > 1.3 { "   <-- stands up" } else { "" }
                    ));
                }
            }
        }
    }
    v
}

/// The closest the framer's output comes to the SOM, for a given branch.
///
/// `best_som` is quoted in the handover as evidence of a working chain, so it has
/// to survive the same question as every other number here: **what does it read
/// on noise?** The same values shuffled are noise, and the answer is printed
/// beside the real one.
fn som_for(
    traced: &[f32],
    rate: fec::CodedRate,
    style: fec::Interleave,
    j: usize,
    sign: f32,
) -> u8 {
    let soft: Vec<f32> = if rate == fec::CodedRate::Bps1200 {
        traced.to_vec()
    } else {
        traced.iter().step_by(2).copied().collect()
    };
    let soft = &soft[..];
    // **The receiver's own chunking, replicated**, because that is where the
    // quoted 31/32 came from: `CodedChain` decodes 256 pairs at a time from a
    // 64-pair overlap thrown away in front, so every chunk starts its Viterbi on
    // state zero. Decoding the whole stream in one pass reads the same bits
    // differently, and a statistic has to be compared with its null through the
    // *same* machinery or the difference is the machinery.
    const CHUNK: usize = 256;
    const OVERLAP: usize = 64;
    let times = rate.repeat();
    let mut di = fec::ConvInterleaver::deinterleaver_with(j, style);
    let mut stream: Vec<f32> = Vec::with_capacity(soft.len());
    let mut consumed = 0;
    let mut fr = fec::MessageFramer::new(rate);
    let need = (CHUNK + OVERLAP) * 2 * times;
    for v in soft {
        stream.push(di.push(v * sign));
        while stream.len() - consumed >= need {
            let pairs = fec::average_pairs(&stream[consumed..consumed + need], times);
            let bits = fec::viterbi_decode(&pairs);
            for b in bits.iter().skip(OVERLAP).take(CHUNK) {
                fr.push(*b);
            }
            consumed += CHUNK * 2 * times;
        }
    }
    fr.best_som
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.is_empty() {
        eprintln!("usage: s4285_fec_check <wav> [carrier_hz]");
        std::process::exit(2);
    }
    let (audio, rate) = read_wav(&a[0]);
    let carrier: f64 = a.get(1).and_then(|v| v.parse().ok()).unwrap_or(1800.0);

    // **The channel path is swept first**, because it decides every number after
    // it: the equaliser is chosen per frame by whether the reference blocks
    // collapse harder, and that is a question about nine symbols, not about the
    // data's eye. On a recording the trellis has never been seen, the right way to
    // ask is the metric itself, so all three paths are read and judged.
    let mut best: Option<(EqMode, Verdict)> = None;
    for mode in [EqMode::Auto, EqMode::Off, EqMode::On] {
        let mut rx = S4285Receiver::new(rate, carrier);
        rx.set_eq_mode(mode);
        rx.trace_coded_stream(true);
        let mut frames = 0;
        for chunk in audio.chunks(4096) {
            for ev in rx.process(chunk) {
                if let S4285Event::Frame(_) = ev {
                    frames += 1;
                }
            }
        }
        // **The stream the chains were fed**, not the frames' own. A frame too
        // poor to report is still fed as 128 zeros so the interleaver stays
        // aligned through a fade; those values never reach a frame event, so a
        // judge reading the events alone sees holes the decoder did not have — and
        // one hole shifts every soft decision after it by a whole frame, which is
        // exactly enough to turn real code into a null.
        let soft: Vec<f32> = rx.coded_stream().to_vec();
        println!(
            "{}: {frames} frames, {} soft decisions at {carrier}, equaliser {mode:?}",
            a[0],
            soft.len()
        );
        if soft.len() < 5_000 {
            println!("  too little to judge");
            continue;
        }
        let v = judge(&soft);
        if let Some((ratio, label)) = &v.best {
            println!("  best {ratio:.2}x on {label}");
        }
        if best.as_ref().is_none_or(|(_, b)| {
            v.best.as_ref().zip(b.best.as_ref()).is_some_and(|(x, y)| x.0 > y.0)
        }) {
            best = Some((mode, v));
        }
    }

    // **The SOM statistic checked against its own null**, on the 600 bps branch
    // that produces it: 31/32 is either the decoder finding real framing or a
    // statistic that also fires on noise.
    if let Some((_, v)) = best.as_ref() {
        let _ = v;
        let mut rx = S4285Receiver::new(rate, carrier);
        rx.trace_coded_stream(true);
        for chunk in audio.chunks(4096) {
            rx.process(chunk);
        }
        let soft: Vec<f32> = rx.coded_stream().to_vec();
        let null: Vec<f32> = shuffled(&soft, 0x0BAD_F00D);
        let mut rows = Vec::new();
        let mut best = (0u8, 0u8, String::new());
        for rate in fec::CodedRate::BPSK {
            for sign in [1.0f32, -1.0] {
                for style in fec::Interleave::ALL {
                    let j = rate.j();
                    let real = som_for(&soft, rate, style, j, sign);
                    let noise = som_for(&null, rate, style, j, sign);
                    let label = format!("{} j{j} sign {sign:+.0} {}", rate.label(), style.label());
                    if real as i16 - noise as i16 > best.0 as i16 - best.1 as i16 {
                        best = (real, noise, label.clone());
                    }
                    rows.push(format!("  {label:>44}  real {real:>2}/32  shuffled {noise:>2}/32"));
                }
            }
        }
        println!("  closest SOM to the known pattern, every branch:");
        for r in &rows {
            println!("{r}");
        }
        println!("  best margin: real {}/32 against shuffled {}/32 on {}", best.0, best.1, best.2);
    }

    if let Some((mode, v)) = best {
        println!();
        println!("  every branch, equaliser {mode:?}:");
        println!("  {:>44}  {:>8}  {:>8}  {:>6}", "branch", "metric", "null", "ratio");
        for row in &v.rows {
            println!("{row}");
        }
        println!(
            "  (a ratio near 1 is no coding; the code is there when a branch stands well above its null)"
        );
    }
}
