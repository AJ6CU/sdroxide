//! Run the STANAG 4285 receiver over a WAV and print what it found.
//!
//!     cargo run --release -p sdroxide-dsp --example s4285_probe -- \
//!         ~/s4285-captures/6348.000kHz-20261022-045545Z.wav 1800
//!
//! Arguments: `<wav> [<carrier_hz>]`. The carrier is the audio frequency of the
//! suppressed carrier (1800 Hz when the dial is on it, which is the standard's
//! own figure). A fixed-frequency log entry can be a kilohertz off, so it is a
//! parameter rather than an assumption.
//!
//! The point of this probe is the **report**, not the text: a lock, the folded
//! sync correlation and its null, the frame ACF, the three reference
//! concentrations and a rate verdict are the difference between "a 4285 is on
//! this frequency" and a guess. Text is printed when it comes, and the raw
//! payload bits are always available.
//!
//! `S4285_EQ=auto|off|on|dfe` picks the channel path, `S4285_SYMBOLS_OUT` and
//! `S4285_SOFT_OUT` dump a frame's symbols and soft decisions, and
//! `S4285_FRAME_LOG` prints one line per frame — which is how the scrambler
//! polarity was found to be consistent across a real tape (285 direct against 16
//! reversed in 301 frames, so it is not what stops the code being read).

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
    // **A capture can be stereo**, and its samples are then interleaved: read as
    // mono, the effective rate is halved and two channels are spliced into one
    // stream, so nothing locks and every number reads zero. The first real coded
    // capture we were sent — a Windows machine recording its own output — was
    // stereo for exactly that reason, and it looked like "the receiver cannot
    // read a coded tape" rather than like a file the probe could not open.
    let ch = spec.channels.max(1) as usize;
    let audio = if ch == 1 {
        raw
    } else {
        raw.chunks(ch).map(|f| f.iter().sum::<f32>() / ch as f32).collect()
    };
    (audio, spec.sample_rate as f64)
}

fn hex(bits: &[u8], n: usize) -> String {
    let take = bits.len().min(n * 8);
    let mut s = String::new();
    for byte in bits[..take].chunks(8) {
        let mut v = 0u8;
        for (i, b) in byte.iter().enumerate() {
            v |= b << (7 - i);
        }
        s.push_str(&format!("{v:02x}"));
    }
    if bits.len() > take {
        s.push('…');
    }
    s
}

fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    if a.is_empty() {
        eprintln!("usage: s4285_probe <wav> [carrier_hz]");
        std::process::exit(2);
    }
    let (audio, rate) = read_wav(&a[0]);
    let carrier: f64 = a.get(1).and_then(|v| v.parse().ok()).unwrap_or(1800.0);
    println!(
        "{}: {:.1} s at {:.0} Hz, carrier {:.0} Hz",
        a[0],
        audio.len() as f64 / rate,
        rate,
        carrier
    );

    let mut rx = sdroxide_dsp::stanag4285::S4285Receiver::new(rate, carrier);
    // `S4285_EQ=auto|off|on|dfe`, so a diagnostic run can ask for one channel path
    // without a rebuild — the equaliser question is asked by comparing them.
    match std::env::var("S4285_EQ").as_deref() {
        Ok("off") => rx.set_eq_mode(sdroxide_dsp::stanag4285::EqMode::Off),
        Ok("on") => rx.set_eq_mode(sdroxide_dsp::stanag4285::EqMode::On),
        Ok("dfe") => rx.set_eq_mode(sdroxide_dsp::stanag4285::EqMode::Dfe),
        _ => {}
    }
    // The stream the Annex E chains actually saw, erasure-filled frames included
    // — the one `coded_stream()` reports and the only one a deinterleaver can be
    // judged on, because a hole in it shifts everything after it by a frame.
    rx.trace_coded_stream(true);
    let mut frames = Vec::new();
    let mut text = String::new();
    let mut messages = 0usize;
    for chunk in audio.chunks(4096) {
        for ev in rx.process(chunk) {
            match ev {
                sdroxide_dsp::stanag4285::S4285Event::Frame(f) => {
                    // The frame's own symbols, for asking *where* an assumed layout
                    // went wrong: the reference blocks are runs of constant phase
                    // and their positions are a property of the transmission.
                    if let Ok(path) = std::env::var("S4285_SYMBOLS_OUT") {
                        use std::io::Write;
                        let mut fh = std::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&path)
                            .expect("open the symbol dump");
                        for c in &f.symbols {
                            write!(fh, "{:.5},{:.5} ", c.re, c.im).expect("write");
                        }
                        writeln!(fh).expect("newline");
                    }
                    if let Ok(path) = std::env::var("S4285_SOFT_OUT") {
                        use std::io::Write;
                        let mut fh = std::fs::OpenOptions::new()
                            .create(true)
                            .append(true)
                            .open(&path)
                            .expect("open the soft dump");
                        for v in &f.soft {
                            write!(fh, "{v:.4} ").expect("write");
                        }
                    }
                    if std::env::var("S4285_FRAME_LOG").is_ok() {
                        println!(
                            "FRAME {} rev={} refs={:?} const={:?}",
                            frames.len(),
                            f.reversed,
                            f.reference,
                            f.constellation
                        );
                    }
                    frames.push(f)
                }
                sdroxide_dsp::stanag4285::S4285Event::Text(t) => {
                    text.push_str(&t);
                    for ch in t.chars() {
                        print!("{ch}");
                    }
                }
                sdroxide_dsp::stanag4285::S4285Event::Message(m) => {
                    messages += 1;
                    if messages <= 6 {
                        println!(
                            "\nmessage via {} ({} bits, {}): {}",
                            m.rate.label(),
                            m.bits.len(),
                            if m.closed { "SOM..EOM" } else { "no EOM" },
                            hex(&m.bits, 24)
                        );
                    }
                }
            }
        }
    }
    if !text.is_empty() {
        println!();
    }

    // **Can this recording carry a message at all?** See
    // `S4285Receiver::coded_quality`: a message needs a *continuous* run of usable
    // frames at least as long as the interleaver, and a recording with good frames
    // scattered through a fade cannot decode whatever the decoder does.
    {
        let quality = rx.coded_quality();
        // Usable is a mean reference concentration above 0.9 — about 26° of phase
        // scatter, a channel-bit error rate near 1e-3 — which is the line the
        // sixteen identical reference symbols can draw on every frame.
        const USABLE_REFS: f32 = 0.9;
        if !quality.is_empty() {
            let (mut best, mut run) = (0usize, 0usize);
            for q in quality {
                run = if *q > USABLE_REFS { run + 1 } else { 0 };
                best = best.max(run);
            }
            let good = quality.iter().filter(|q| **q > USABLE_REFS).count();
            // The long interleave's deepest row is 31·j 32-symbol segments, and a
            // frame carries four of them.
            let need_long = (31 * 12) / 4;
            let need_short = (31 * 3) / 4;
            println!(
                "coded readiness: {} frames, {} usable (refs > {USABLE_REFS}), longest run {} \
                 — a message needs {} with the long interleave, {} with the short",
                quality.len(),
                good,
                best,
                need_long,
                need_short
            );
            if best < need_short {
                println!(
                    "  -> no coded message can come out of this recording: not enough \
                     continuous signal for even the short interleave"
                );
            } else if best < need_long {
                println!(
                    "  -> the long interleave is out of reach; a short-interleaved \
                     transmission could still read"
                );
            }
        }
    }
    if let Ok(path) = std::env::var("S4285_CODED_OUT") {
        use std::io::Write;
        let mut fh = std::fs::File::create(&path).expect("create the coded dump");
        for v in rx.coded_stream() {
            writeln!(fh, "{v:.6}").expect("write");
        }
    }
    let r = rx.report();
    println!(
        "locked {} | sync {:.3} (null {:.3}) | frame ACF {:.3} | frames {} | rate {}",
        r.locked,
        r.sync_quality,
        r.correlation_floor,
        r.frame_acf,
        r.frames,
        r.rate.label()
    );
    println!(
        "last reference blocks: {:.3} {:.3} {:.3} (1.0 collapses, 0.25 is sixteen symbols of noise)",
        r.reference[0], r.reference[1], r.reference[2]
    );
    println!(
        "constellation 2/4/8-fold: {:.3} {:.3} {:.3} | offset {:+.1} Hz | timing {:.2}",
        r.constellation[0], r.constellation[1], r.constellation[2], r.carrier_offset_hz, r.timing
    );
    if !frames.is_empty() {
        let mean = |i: usize| {
            frames
                .iter()
                .map(|f: &sdroxide_dsp::stanag4285::S4285Frame| f.reference[i])
                .sum::<f32>()
                / frames.len() as f32
        };
        println!(
            "mean reference over {} frames: {:.3} {:.3} {:.3}",
            frames.len(),
            mean(0),
            mean(1),
            mean(2)
        );
        let f = frames.last().unwrap();
        println!(
            "last frame: {} phases used {}, scrambler {}, {} bits",
            f.rate.label(),
            f.phases_used,
            if f.reversed { "reversed" } else { "direct" },
            f.bits.len()
        );
    }
    println!(
        "coded: rate {} | {} messages | best SOM {}/32",
        r.coded.map(|c| c.label()).unwrap_or("none seen"),
        r.messages,
        r.best_som
    );
    println!("payload bits so far: {}", rx.bits().len());
    if let Some(path) = std::env::var("S4285_BITS_OUT").ok() {
        let s: String = rx.bits().iter().map(|b| if *b == 1 { '1' } else { '0' }).collect();
        std::fs::write(path, s).expect("write bits");
    }
    if let Some(f) = frames.last() {
        // The whole-frame constellation against each data block's own: a phase
        // that walks *between* blocks leaves these high and the frame low, and
        // one that moves *inside* a block of 32 symbols flattens both.
        let b = f.block;
        println!(
            "data blocks (best of 2/4/8-fold per block): {:.3} {:.3} {:.3} {:.3}",
            b[0], b[1], b[2], b[3]
        );
        let head: String =
            f.bits.iter().take(64).map(|b| if *b == 1 { '1' } else { '0' }).collect();
        println!("first 64 bits of the last frame: {head}");
    }
}
