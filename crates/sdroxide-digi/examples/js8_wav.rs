//! JS8 ↔ WAV bridge, for interoperability testing against JS8Call itself.
//!
//! This drives the *real* encoder the engine transmits with, so what it writes
//! is what sdroxide puts on the air. Open the result in JS8Call (or feed it to
//! `jt9`) and the message should appear exactly as given.
//!
//! ```text
//! cargo run -p sdroxide-digi --example js8_wav -- \
//!     encode 0123456789AB out.wav --speed normal [--type 0] [--hz 1500] [--shaping gfsk|cpfsk]
//! cargo run -p sdroxide-digi --example js8_wav -- tones 0123456789AB --speed normal
//! ```
//!
//! The message is twelve characters of JS8's six-bit transport alphabet
//! (`0-9A-Za-z-+`), which is the layer directly under the varicode — one frame
//! exactly, no reassembly. Once `js8::varicode` lands this grows a mode that
//! takes ordinary text.
//!
//! The WAV is 16-bit mono at 12 kHz, padded to a full slot with the submode's
//! start delay ahead of the frame, so the file is slot-aligned the way a
//! decoder expects.

use sdroxide_digi::js8::decode::{self, Js8Depth};
use sdroxide_digi::js8::message::Js8Payload;
use sdroxide_digi::js8::modem::{self, DECODE_RATE};
use sdroxide_types::Js8Speed;

fn speed_from(name: &str) -> Option<Js8Speed> {
    match name.to_ascii_lowercase().as_str() {
        "normal" | "a" | "js8a" => Some(Js8Speed::Normal),
        "fast" | "b" | "js8b" => Some(Js8Speed::Fast),
        "turbo" | "c" | "js8c" => Some(Js8Speed::Turbo),
        "slow" | "e" | "js8e" => Some(Js8Speed::Slow),
        _ => None,
    }
}

struct Opts {
    speed: Js8Speed,
    frame_type: u8,
    hz: f32,
    cpfsk: bool,
    /// Peak signal amplitude before noise is added.
    amp: f32,
    /// Uniform noise amplitude, for sensitivity comparisons against JS8Call.
    noise: f32,
    seed: u64,
    osd: bool,
}

fn parse_opts(args: &[String]) -> Result<Opts, String> {
    let mut o = Opts {
        speed: Js8Speed::Normal,
        frame_type: 0,
        hz: 1500.0,
        cpfsk: false,
        amp: 0.5,
        noise: 0.0,
        seed: 1,
        osd: false,
    };
    let mut i = 0;
    while i < args.len() {
        let value = || args.get(i + 1).ok_or_else(|| format!("{} needs a value", args[i]));
        match args[i].as_str() {
            "--speed" => {
                o.speed = speed_from(value()?).ok_or("speed: normal|fast|turbo|slow")?;
                i += 2;
            }
            "--type" => {
                o.frame_type = value()?.parse::<u8>().map_err(|e| e.to_string())? & 0b111;
                i += 2;
            }
            "--hz" => {
                o.hz = value()?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?;
                i += 2;
            }
            "--shaping" => {
                o.cpfsk = value()? == "cpfsk";
                i += 2;
            }
            "--amp" => {
                o.amp = value()?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?;
                i += 2;
            }
            "--noise" => {
                o.noise = value()?.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?;
                i += 2;
            }
            "--seed" => {
                o.seed = value()?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
                i += 2;
            }
            "--osd" => {
                o.osd = true;
                i += 1;
            }
            other => return Err(format!("unknown option {other}")),
        }
    }
    Ok(o)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("encode") => {
            let text = args.get(2).ok_or("usage: encode <12 chars> <out.wav> [options]")?;
            let path = args.get(3).ok_or("usage: encode <12 chars> <out.wav> [options]")?;
            let o = parse_opts(&args[4..])?;
            let payload = Js8Payload::from_chars(text, o.frame_type)
                .ok_or("message must be exactly 12 characters of 0-9A-Za-z-+")?;

            let tones = modem::frame_tones_for(o.speed, payload);
            let frame = if o.cpfsk {
                modem::synth_cpfsk_for(o.speed, &tones, o.hz, o.amp)
            } else {
                modem::synth_gfsk_for(o.speed, &tones, o.hz, o.amp)
            };

            // Lay the frame into a full slot at its submode's start delay, so
            // the file is slot-aligned rather than merely containing a burst.
            let slot = (o.speed.slot_s() * DECODE_RATE) as usize;
            let start = modem::start_offset_samples(o.speed);
            let mut audio = vec![0.0f32; slot.max(start + frame.len())];
            audio[start..start + frame.len()].copy_from_slice(&frame);

            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: DECODE_RATE as u32,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };
            // Deterministic noise, so a sensitivity sweep is reproducible.
            let mut state = o.seed | 1;
            let mut next = || {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                (state % 20_000) as f32 / 10_000.0 - 1.0
            };

            let mut w = hound::WavWriter::create(path, spec)?;
            for s in &audio {
                let v = s + next() * o.noise;
                w.write_sample((v.clamp(-1.0, 1.0) * 28_000.0) as i16)?;
            }
            w.finalize()?;

            eprintln!(
                "wrote {path}: {} {} Hz, frame type {}, {:.2} s frame in a {:.0} s slot ({})",
                o.speed.label(),
                o.hz,
                o.frame_type,
                o.speed.burst_s(),
                o.speed.slot_s(),
                if o.cpfsk { "cpfsk, as JS8Call transmits" } else { "gfsk" },
            );
            eprintln!("open it in JS8Call; it should read {text:?}");
        }
        Some("tones") => {
            let text = args.get(2).ok_or("usage: tones <12 chars> [options]")?;
            let o = parse_opts(&args[3..])?;
            let payload = Js8Payload::from_chars(text, o.frame_type)
                .ok_or("message must be exactly 12 characters of 0-9A-Za-z-+")?;
            let tones = modem::frame_tones_for(o.speed, payload);
            let rendered: Vec<String> = tones.iter().map(|t| t.to_string()).collect();
            println!("{}", rendered.join(","));
        }
        Some("mix") => {
            // `mix out.wav TEXT@HZ TEXT@HZ ...` — a crowded slot, for busy-band
            // comparisons against JS8Call.
            let path = args.get(2).ok_or("usage: mix <out.wav> <TEXT@HZ>...")?;
            let mut specs = Vec::new();
            let mut rest = Vec::new();
            for a in &args[3..] {
                if a.contains('@') { specs.push(a.clone()) } else { rest.push(a.clone()) }
            }
            let o = parse_opts(&rest)?;
            let slot = (o.speed.slot_s() * DECODE_RATE) as usize;
            let start = modem::start_offset_samples(o.speed);
            let mut audio = vec![0.0f32; slot];
            for spec in &specs {
                let (text, hz) = spec.split_once('@').ok_or("expected TEXT@HZ")?;
                let hz: f32 = hz.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?;
                let payload = Js8Payload::from_chars(text, o.frame_type)
                    .ok_or("message must be exactly 12 characters of 0-9A-Za-z-+")?;
                let tones = modem::frame_tones_for(o.speed, payload);
                let frame = modem::synth_gfsk_for(o.speed, &tones, hz, o.amp);
                for (i, &v) in frame.iter().enumerate() {
                    if start + i < slot {
                        audio[start + i] += v;
                    }
                }
            }
            let mut state = o.seed | 1;
            let mut next = || {
                state ^= state << 13;
                state ^= state >> 7;
                state ^= state << 17;
                (state % 20_000) as f32 / 10_000.0 - 1.0
            };
            let spec = hound::WavSpec {
                channels: 1,
                sample_rate: DECODE_RATE as u32,
                bits_per_sample: 16,
                sample_format: hound::SampleFormat::Int,
            };
            let mut w = hound::WavWriter::create(path, spec)?;
            for v in &audio {
                w.write_sample(((v + next() * o.noise).clamp(-1.0, 1.0) * 28_000.0) as i16)?;
            }
            w.finalize()?;
            eprintln!("wrote {path}: {} signals", specs.len());
        }
        Some("decode") => {
            let path = args.get(2).ok_or("usage: decode <in.wav> [--speed ...] [--osd]")?;
            let o = parse_opts(&args[3..])?;
            let mut r = hound::WavReader::open(path)?;
            if r.spec().sample_rate != DECODE_RATE as u32 {
                return Err(
                    format!("expected {} Hz, got {}", DECODE_RATE, r.spec().sample_rate).into()
                );
            }
            let pcm: Vec<i16> = r.samples::<i16>().collect::<Result<_, _>>()?;
            let depth = if o.osd { Js8Depth::BpOsd } else { Js8Depth::Bp };
            let decodes = decode::decode_slot_for(o.speed, &pcm, depth);
            for d in &decodes {
                println!(
                    "{:>4.0} {:+5.1} {:>6.1} {}  {}  type {}",
                    d.snr_db,
                    d.dt_sec,
                    d.audio_hz,
                    o.speed.label().chars().next().unwrap(),
                    d.payload.to_chars(),
                    d.payload.frame_type,
                );
            }
            if decodes.is_empty() {
                std::process::exit(1);
            }
        }
        _ => {
            eprintln!("usage:");
            eprintln!("  js8_wav encode <12 chars> <out.wav> [--speed normal|fast|turbo|slow]");
            eprintln!("                 [--type 0..7] [--hz 1500] [--shaping gfsk|cpfsk]");
            eprintln!("                 [--amp 0.5] [--noise 0.0] [--seed 1]");
            eprintln!("  js8_wav decode <in.wav> [--speed ...] [--osd]");
            eprintln!("  js8_wav tones  <12 chars> [--speed ...] [--type ...]");
            std::process::exit(2);
        }
    }
    Ok(())
}
