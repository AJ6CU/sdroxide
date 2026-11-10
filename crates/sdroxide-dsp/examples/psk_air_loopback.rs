//! A PSK loopback across the air, for the mode table's "loses a word" rows.
//!
//! Two subcommands, because the two halves happen either side of a real
//! transmission:
//!
//! ```sh
//! # 1. Audio a transceiver can be keyed with: PSK31 or PSK63, known text.
//! cargo run --release -p sdroxide-dsp --example psk_air_loopback -- \
//!     tx 31 /tmp/opencode/psk31.wav
//!
//! # 2. Decode what the receiver heard, from a CF32 capture.
//! cargo run --release -p sdroxide-dsp --example psk_air_loopback -- \
//!     decode /tmp/opencode/psk31.raw 31
//! ```
//!
//! The point of it is that the text is known and the channel is ours: a word the
//! decoder drops here is the decoder's, not a recording's. The capture is CF32 at
//! the radio's own rate; the carrier is found by scanning for the strongest tone,
//! which is what makes it work with two radios whose dials disagree by a few tens
//! of hertz.
//!
//! **What it has shown so far (2026-10-26).** `selftest` decodes the generated
//! audio 10/10 words, so the modulator, the receiver and this harness are sound.
//! Across the air on 27.415 MHz USB at 2 W the capture carries a clean BPSK31
//! spectrum (peaks at the carrier with sidebands at ±9 dB, 60 Hz wide) but `split`
//! and `decode` read only fragments: the surviving suspects are the transmitter's
//! audio path — a CB rig's ALC pumping on a signal whose envelope must stay flat —
//! and the extraction's sideband. That is a bench question for the operator
//! (drive and ALC), not something to keep guessing at.

use sdroxide_dsp::{PskRx, PskTx};

/// The text, long enough to be three "words" in the table's sense and short
/// enough to keep a transmission brief.
const TEXT: &str = "the quick brown fox jumps over the lazy dog 73";

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let Some(cmd) = args.get(1).map(String::as_str) else {
        eprintln!("usage: tx <baud> <out.wav> | decode <capture.raw> <baud>");
        std::process::exit(2);
    };
    match cmd {
        "tx" => {
            let baud: f64 = args[2].parse().expect("baud");
            let out = &args[3];
            tx(baud, out);
        }
        "selftest" => {
            // The same receiver on the audio we generated, with no air in
            // between: if this fails, the harness is wrong and the air is
            // innocent.
            let wav = &args[2];
            let baud: f64 = args[3].parse().expect("baud");
            let carrier: f64 = args.get(4).and_then(|v| v.parse().ok()).unwrap_or(1500.0);
            selftest(wav, baud, carrier);
        }
        "split" => {
            let raw = &args[2];
            let out = &args[3];
            split(raw, out);
        }
        "decode" => {
            let raw = &args[2];
            let baud: f64 = args[3].parse().expect("baud");
            decode(raw, baud);
        }
        other => {
            eprintln!("unknown command {other}");
            std::process::exit(2);
        }
    }
}

fn tx(baud: f64, out: &str) {
    const RATE: f64 = 48_000.0;
    const CARRIER: f64 = 1500.0;
    let mut tx = PskTx::new_baud(RATE, CARRIER, baud);
    tx.push_text(TEXT);
    // One second of idle first, so the receiver has something to lock onto, then
    // the text, then a second of idle: the last character's word only completes
    // when the next one starts.
    let text_s = TEXT.len() as f64 / baud * 12.0 + 2.0;
    let total = ((1.0 + text_s) * RATE) as usize;
    let mut audio = vec![0f32; total];
    let mut block = vec![0f32; 1024];
    let mut at = RATE as usize; // one second of silence at the head
    while at < audio.len() {
        let n = tx.next_block(&mut block);
        for (i, s) in block[..n].iter().enumerate() {
            if at + i < audio.len() {
                audio[at + i] = *s;
            }
        }
        at += n;
    }
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: RATE as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut w = hound::WavWriter::create(out, spec).expect("wav");
    for s in &audio {
        w.write_sample((s * 0.9 * f32::from(i16::MAX)) as i16).expect("write");
    }
    w.finalize().expect("finalize");
    println!(
        "wrote {out}: {:.1}s of {baud:.0} baud, carrier {CARRIER:.0} Hz, text {}",
        audio.len() as f64 / RATE,
        TEXT.len()
    );
}

fn decode(raw: &str, baud: f64) {
    let bytes = std::fs::read(raw).expect("capture");
    let cs: Vec<f32> =
        bytes.chunks_exact(4).map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect();
    let iq: Vec<(f32, f32)> = cs.chunks_exact(2).map(|p| (p[0], p[1])).collect();
    assert!(!iq.is_empty(), "empty capture");

    // **The capture's own rate**, passed in through the environment because a
    // radio may refuse the rate it was asked for and stream at another.
    let rate: f64 =
        std::env::var("CAPTURE_RATE").ok().and_then(|v| v.parse().ok()).unwrap_or(2_000_000.0);

    // Find the carrier: coarse DFT over ±3 kHz in 25 Hz steps, on a slice in the
    // middle of the capture where the transmission certainly is.
    let mid = iq.len() / 2;
    let slice = &iq[mid.saturating_sub((rate * 0.2) as usize)..mid];
    let mut best = (0.0f64, 0.0f64);
    let mut hz = -3000.0;
    while hz <= 3000.0 {
        let w = std::f64::consts::TAU * hz / rate;
        let (mut re, mut im) = (0.0f64, 0.0f64);
        for (n, (i, q)) in slice.iter().enumerate().step_by(8) {
            let ph = w * n as f64;
            re += f64::from(*i) * ph.cos() + f64::from(*q) * ph.sin();
            im += f64::from(*q) * ph.cos() - f64::from(*i) * ph.sin();
        }
        let mag = re.hypot(im);
        if mag > best.0 {
            best = (mag, hz);
        }
        hz += 25.0;
    }
    let carrier = best.1;
    println!("carrier at {carrier:+.0} Hz (capture rate {rate:.0})");
    // **Both sidebands, both offsets**: a rig may be in LSB where the operator
    // said USB, and a suppressed-carrier signal's audio can land either side of
    // where the peak search put it. Decoding both and taking the better is
    // honest — one of them is the signal and the other is its mirror.

    let mut best_words = 0usize;
    for (shift_hz, invert) in [(1500.0f64, false), (1500.0, true), (-1500.0, false)] {
        let (words, text) = decode_at(&iq, rate, carrier, shift_hz, invert, baud);
        println!(
            "  {shift_hz:+.0} Hz{}, {words} words: {text}",
            if invert { " inverted" } else { "" }
        );
        if words > best_words {
            best_words = words;
        }
    }
    let want: Vec<&str> = TEXT.split(' ').collect();
    println!("{best_words} of {} words at best", want.len());
    if best_words != want.len() {
        std::process::exit(1);
    }
    return;
    #[allow(unreachable_code)]
    // Mix down to audio and decimate to 12 kHz with a box average. PSK31 is
    // 31 Hz wide, so a boxcar's own response is far wider than the signal.
    let decim = (rate / 12_000.0).round() as usize;
    // **The rate the boxcar actually produces**, not the 12 kHz it was aiming
    // at: 2 MS/s over 167 samples is 11 976 Hz, and telling the receiver 12 000
    // smears a 31-baud message by a symbol and a half across twenty seconds.
    let audio_rate = rate / decim as f64;
    let mut audio = Vec::with_capacity(iq.len() / decim.max(1));
    let w = -std::f64::consts::TAU * carrier / rate;
    let mut acc = 0.0f64;
    for (n, (i, q)) in iq.iter().enumerate() {
        let ph = w * n as f64;
        // Real part after mixing: the sideband is USB, so the audio is the
        // in-phase term.
        acc += f64::from(*i) * ph.cos() - f64::from(*q) * ph.sin();
        if (n + 1) % decim == 0 {
            audio.push((acc / decim as f64) as f32);
            acc = 0.0;
        }
    }
    // A 12 kHz audio stream whose carrier is at 1500 Hz: the mix put DC on the
    // carrier, so move it where the receiver expects it.
    let shift = std::f64::consts::TAU * 1500.0 / audio_rate;
    for (n, s) in audio.iter_mut().enumerate() {
        let ph = shift * n as f64;
        *s = (*s as f64 * ph.cos()) as f32;
    }

    let mut rx = PskRx::new_baud(audio_rate, 1500.0, baud);
    let mut text = String::new();
    for block in audio.chunks(480) {
        text.push_str(&rx.process(block));
    }
    println!("decoded: {text}");
    let want: Vec<&str> = TEXT.split(' ').collect();
    let got: Vec<&str> = text.split(' ').collect();
    let hits = want.iter().filter(|w| got.contains(w)).count();
    println!("{hits} of {} words", want.len());
    if hits != want.len() {
        std::process::exit(1);
    }
}

/// Mix a capture down at `carrier`, put the audio at `shift_hz` (inverting the
/// sideband when asked), decimate to 12 kHz and decode.
fn decode_at(
    iq: &[(f32, f32)],
    rate: f64,
    carrier: f64,
    shift_hz: f64,
    invert: bool,
    baud: f64,
) -> (usize, String) {
    let decim = (rate / 12_000.0).round() as usize;
    let audio_rate = rate / decim as f64;
    // The middle of the capture: the transmission is there and the edges are
    // silence.
    let centre = iq.len() / 2;
    let span = (rate * 8.0) as usize;
    let lo = centre.saturating_sub(span.min(centre));
    let slice = &iq[lo..(lo + 2 * span).min(iq.len())];

    let mut audio = Vec::with_capacity(slice.len() / decim.max(1));
    let w = -std::f64::consts::TAU * carrier / rate;
    let mut acc = 0.0f64;
    for (n, (i, q)) in slice.iter().enumerate() {
        let ph = w * n as f64;
        // USB keeps the in-phase term, LSB the quadrature one with the sign
        // flipped — which is the same as negating the baseband's imaginary part.
        let base = if invert {
            -(f64::from(*i) * ph.sin() + f64::from(*q) * ph.cos())
        } else {
            f64::from(*i) * ph.cos() - f64::from(*q) * ph.sin()
        };
        acc += base;
        if (n + 1) % decim == 0 {
            audio.push((acc / decim as f64) as f32);
            acc = 0.0;
        }
    }
    // The audio now has the carrier at DC; put it where the receiver listens.
    let shift = std::f64::consts::TAU * shift_hz / audio_rate;
    for (n, s) in audio.iter_mut().enumerate() {
        let ph = shift * n as f64;
        *s = (*s as f64 * ph.cos()) as f32;
    }

    let mut rx = PskRx::new_baud(audio_rate, shift_hz.abs(), baud);
    let mut text = String::new();
    for block in audio.chunks(480) {
        text.push_str(&rx.process(block));
    }
    let want: Vec<&str> = TEXT.split(' ').collect();
    let got: Vec<&str> = text.split(' ').collect();
    let words = want.iter().filter(|w| got.contains(w)).count();
    (words, text)
}

/// Decode the transmitter's own WAV, decimated 48 kHz → 12 kHz. Four samples to
/// one is exact, so this is a fair test of the receiver and the harness.
fn selftest(wav: &str, baud: f64, carrier: f64) {
    let mut r = hound::WavReader::open(wav).expect("wav");
    let spec = r.spec();
    let x: Vec<f32> = r.samples::<i16>().map(|s| f32::from(s.unwrap()) / 32768.0).collect();
    assert_eq!(spec.channels, 1);
    // **The file's own rate**, whatever it is: a capture decimated to 11 976 Hz
    // is not 12 000, and pretending it is smears a 31-baud message.
    let rate = f64::from(spec.sample_rate);
    let audio: Vec<f32> = x;
    let mut rx = PskRx::new_baud(rate, carrier, baud);
    let mut text = String::new();
    for block in audio.chunks(480) {
        text.push_str(&rx.process(block));
    }
    println!("selftest decoded: {text}");
    let want: Vec<&str> = TEXT.split(' ').collect();
    let got: Vec<&str> = text.split(' ').collect();
    let words = want.iter().filter(|w| got.contains(w)).count();
    println!("{words} of {} words", want.len());
    if words != want.len() {
        std::process::exit(1);
    }
}

/// Extract the audio from a capture and write it as a WAV, so the extraction and
/// the decoding can be judged separately.
fn split(raw: &str, out: &str) {
    let bytes = std::fs::read(raw).expect("capture");
    let cs: Vec<f32> =
        bytes.chunks_exact(4).map(|b| f32::from_le_bytes([b[0], b[1], b[2], b[3]])).collect();
    let iq: Vec<(f32, f32)> = cs.chunks_exact(2).map(|p| (p[0], p[1])).collect();
    let rate: f64 =
        std::env::var("CAPTURE_RATE").ok().and_then(|v| v.parse().ok()).unwrap_or(2_000_000.0);
    // The carrier: the strongest tone inside ±4 kHz, as `decode` finds it.
    let mid = iq.len() / 2;
    let slice = &iq[mid.saturating_sub((rate * 0.2) as usize)..mid];
    let mut best = (0.0f64, 0.0f64);
    let mut hz = -4000.0;
    while hz <= 4000.0 {
        let w = std::f64::consts::TAU * hz / rate;
        let (mut re, mut im) = (0.0f64, 0.0f64);
        for (n, (i, q)) in slice.iter().enumerate().step_by(8) {
            let ph = w * n as f64;
            re += f64::from(*i) * ph.cos() + f64::from(*q) * ph.sin();
            im += f64::from(*q) * ph.cos() - f64::from(*i) * ph.sin();
        }
        let mag = re.hypot(im);
        if mag > best.0 {
            best = (mag, hz);
        }
        hz += 10.0;
    }
    let carrier = best.1;
    println!("carrier {carrier:+.0} Hz");
    let decim = (rate / 12_000.0).round() as usize;
    let audio_rate = rate / decim as f64;
    let mut audio = Vec::with_capacity(iq.len() / decim.max(1));
    let w = -std::f64::consts::TAU * carrier / rate;
    let mut acc = 0.0f64;
    for (n, (i, q)) in iq.iter().enumerate() {
        let ph = w * n as f64;
        // Complex baseband, then the real part shifted up to 1500 Hz.
        let base = f64::from(*i) * ph.cos() - f64::from(*q) * ph.sin();
        acc += base;
        if (n + 1) % decim == 0 {
            let t = (n / decim) as f64;
            let up = std::f64::consts::TAU * 1500.0 / audio_rate * t;
            audio.push(((acc / decim as f64) * up.cos()) as f32);
            acc = 0.0;
        }
    }
    // **Resample to exactly 12 kHz.** The boxcar lands on 11 976 Hz for a 2 MS/s
    // capture, and the receiver's symbol clock is built from an integer number of
    // samples per symbol: 0.2 % of the rate is enough to walk a 31-baud message
    // off its own timing. A linear stretch is plenty for a signal this narrow.
    let audio = linear_resample(&audio, audio_rate, 12_000.0);
    let audio_rate = 12_000.0f64;
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: audio_rate.round() as u32,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut w = hound::WavWriter::create(out, spec).expect("wav");
    let peak = audio.iter().fold(0f32, |p, s| p.max(s.abs())).max(1e-6);
    for s in &audio {
        w.write_sample((s / peak * 0.9 * f32::from(i16::MAX)) as i16).expect("write");
    }
    w.finalize().expect("finalize");
    println!("wrote {out}: {:.1}s at {audio_rate:.1} Hz", audio.len() as f64 / audio_rate);
}

/// Linear resampling, which is all a signal 60 Hz wide needs.
fn linear_resample(x: &[f32], from: f64, to: f64) -> Vec<f32> {
    if (from - to).abs() < 1e-9 || x.is_empty() {
        return x.to_vec();
    }
    let ratio = from / to;
    let n = ((x.len() as f64) / ratio).floor() as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f64 * ratio;
        let j = t.floor() as usize;
        let f = (t - j as f64) as f32;
        let a = x[j.min(x.len() - 1)];
        let b = x[(j + 1).min(x.len() - 1)];
        out.push(a + (b - a) * f);
    }
    out
}
