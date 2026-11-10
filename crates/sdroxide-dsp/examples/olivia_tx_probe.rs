//! Render an Olivia transmission to a WAV — the same bytes the transmitter
//! would put on the air.
//!
//! **Why this exists.** The open question on Olivia is whether a real decoder
//! copies *us*. We have never been able to answer it without an over, and an
//! over is the most expensive possible way to find out: an unproven signal on a
//! shared band, needing a second station to report back. But the transmitter is
//! a pure function of a message, so the thing a decoder would hear can simply be
//! written to a file instead.
//!
//! ```sh
//! cargo run -p sdroxide-dsp --example olivia_tx_probe -- "CQ CQ DE W1AW" /tmp/opencode/olivia_tx.wav
//! ```
//!
//! then open that WAV in **fldigi** (Olivia, same tones/bandwidth) or
//! **MultiPSK**. What it decodes is exactly what a station would have copied.
//! That is the whole experiment, and it costs nobody any air time.
//!
//! **What is deliberately not settled by this.** Our own receiver reads our own
//! transmission, and has since the loopbacks — that proves the modulator and
//! demodulator agree with each other, which is what they did while the
//! scrambler was a local invention and the mode could not read a single Olivia
//! station. The value here is entirely in what a *third-party* decoder makes of
//! the file. Passing here and failing there is the interesting result, and it is
//! the one that has never been obtained.
//!
//! The audio is written at the rate the mode generates at (8 kHz), because that
//! is the rate its own generator is defined for; fldigi and MultiPSK both read
//! 8 kHz mono comfortably.

use sdroxide_dsp::OliviaTx;

/// Samples per symbol at 8 kHz for the mode's common 16 tones / 500 Hz.
const RATE: f64 = 8000.0;
/// The centre tone frequency the engine uses for Olivia (`text_modem.rs`).
const AUDIO_HZ: f64 = 1500.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let msg = args.next().unwrap_or_else(|| "CQ CQ DE W1AW".into());
    let path = args.next().unwrap_or_else(|| "/tmp/opencode/olivia_tx.wav".into());

    // 16 tones over 500 Hz is the common Olivia format — 31.25 Hz spacing, which
    // is also what the mode's own tests use. Told to the user, because a decoder
    // set to anything else will report nothing and that is not evidence.
    let (tones, bw) = (16usize, 500.0f64);

    let mut tx = OliviaTx::new(RATE, AUDIO_HZ, tones, bw);
    tx.push_text(&msg);

    let mut pcm: Vec<i16> = Vec::new();
    let mut buf = [0.0f32; 2048];

    // The opening bracket comes first, from a transmitter that has never been
    // asked for audio — so the file contains the whole framing, not just data.
    tx.next_block(&mut buf);
    pcm.extend(buf.iter().map(|s| to_pcm(*s)));

    // Then the message, and then the closing bracket. `tail_pending` is asked
    // rather than `drained`, because the modem keeps producing idle audio
    // for ever once the bracket is spent.
    let mut guard = 0usize;
    while (tx.sent_chars() < tx.total_chars() || tx.tail_pending()) && guard < 200_000 {
        tx.next_block(&mut buf);
        pcm.extend(buf.iter().map(|s| to_pcm(*s)));
        guard += 1;
    }
    // A little air after the bracket, so a decoder's own tail handling is not
    // looking at a file that simply stops.
    pcm.extend(std::iter::repeat(0).take(RATE as usize / 2));

    write_wav(&path, &pcm)?;
    let secs = pcm.len() as f64 / RATE;
    println!("wrote {path}");
    println!("  message: {msg:?}");
    println!("  {tones} tones / {bw:.0} Hz, centre {AUDIO_HZ:.0} Hz, {secs:.1} s at {RATE:.0} Hz");
    println!("  open it in fldigi (Olivia, {tones}/{bw:.0}) or MultiPSK");
    Ok(())
}

fn to_pcm(s: f32) -> i16 {
    (s.clamp(-1.0, 1.0) * 32767.0) as i16
}

/// A minimal 16-bit mono PCM WAV. Written by hand rather than pulled from a
/// crate for the same reason the whole probe is hand-written: this is a
/// measurement aid, and it must build in every configuration `cargo test
/// --workspace` uses.
fn write_wav(path: &str, pcm: &[i16]) -> std::io::Result<()> {
    let data_len = (pcm.len() * 2) as u32;
    let mut w: Vec<u8> = Vec::with_capacity(44 + data_len as usize);
    w.extend_from_slice(b"RIFF");
    w.extend_from_slice(&(36 + data_len).to_le_bytes());
    w.extend_from_slice(b"WAVEfmt ");
    w.extend_from_slice(&16u32.to_le_bytes()); // fmt chunk size
    w.extend_from_slice(&1u16.to_le_bytes()); // PCM
    w.extend_from_slice(&1u16.to_le_bytes()); // mono
    w.extend_from_slice(&(RATE as u32).to_le_bytes());
    w.extend_from_slice(&((RATE as u32) * 2).to_le_bytes()); // byte rate
    w.extend_from_slice(&2u16.to_le_bytes()); // block align
    w.extend_from_slice(&16u16.to_le_bytes()); // bits per sample
    w.extend_from_slice(b"data");
    w.extend_from_slice(&data_len.to_le_bytes());
    for s in pcm {
        w.extend_from_slice(&s.to_le_bytes());
    }
    if let Some(dir) = std::path::Path::new(path).parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(path, w)
}
