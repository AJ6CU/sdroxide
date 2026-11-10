//! BPSK31 decoder performance against the same synthetic HF channel as the
//! RTTY sweeps, so the two keyboard modes are measured on comparable ground.
//!
//! PSK31's whole appeal is weak-signal readability — its ~31 Hz occupied
//! bandwidth against a 3 kHz reference is nearly 20 dB of processing gain, so a
//! healthy decoder should still be reading well below 0 dB SNR(3k). Anything
//! that throws that gain away (no matched filter, a decision made from one
//! sample per symbol) shows up here as a cliff far too high on the SNR axis.

mod common;

use common::{Channel, Fading, accuracy, copies, repeated};
use sdroxide_dsp::{PskRx, PskTx};

const RATE: f64 = 8000.0;
const CARRIER: f64 = 1000.0;

const MSG: &str = "CQ CQ de AB1CD AB1CD pse k ";

fn transmit(reps: usize) -> Vec<f32> {
    let mut tx = PskTx::new(RATE, CARRIER);
    let mut audio = Vec::new();
    // Idle reversals give the receiver time to acquire phase and timing.
    let mut pre = vec![0.0f32; 8000];
    tx.next_block(&mut pre);
    audio.extend_from_slice(&pre);

    for _ in 0..reps {
        tx.push_text(MSG);
    }
    let mut guard = 0;
    while tx.sent_chars() < tx.total_chars() && guard < 20_000 {
        let mut b = [0.0f32; 2000];
        tx.next_block(&mut b);
        audio.extend_from_slice(&b);
        guard += 1;
    }
    let mut tail = vec![0.0f32; 8000];
    tx.next_block(&mut tail);
    audio.extend_from_slice(&tail);
    audio
}

fn decode(audio: &[f32]) -> String {
    let mut rx = PskRx::new(RATE, CARRIER);
    let mut out = String::new();
    for chunk in audio.chunks(512) {
        out.push_str(&rx.process(chunk));
    }
    out
}

fn score(audio: &[f32], ch: Channel, seeds: u64, reps: usize) -> (f64, f64) {
    let reference = repeated(MSG, reps);
    let mut acc = 0.0;
    let mut cps = 0.0;
    for s in 0..seeds {
        let rx = ch.seed(ch.seed + s).apply(audio, RATE);
        let got = decode(&rx);
        acc += accuracy(&got, &reference);
        cps += copies(&got, MSG) as f64;
    }
    (acc / seeds as f64, cps / seeds as f64)
}

#[test]
fn clean_channel_is_perfect() {
    let audio = transmit(3);
    let got = decode(&audio);
    assert_eq!(copies(&got, MSG), 3, "clean decode was {got:?}");
}

#[test]
fn snr_sweep() {
    let reps = 3;
    let audio = transmit(reps);
    println!("\nPSK31 — AWGN only");
    println!("  SNR(3k)   accuracy  copies");
    let mut rows = Vec::new();
    for snr in [20.0, 15.0, 10.0, 5.0, 0.0, -3.0, -6.0, -9.0, -12.0, -15.0] {
        let ch = Channel::clean().snr(snr).seed(100);
        let (a, c) = score(&audio, ch, 4, reps);
        println!("  {snr:>6.1} dB   {a:>6.3}   {c:>5.2}");
        rows.push((snr, a));
    }
    // Measured: clean copy all the way down to -9 dB, ~0.92 at -12 dB. Before
    // the matched filter went in, the decision was made from a single baseband
    // sample per symbol and it cliffed at -6 dB (0.81 there, 0.28 at -12) —
    // roughly 6 dB of the mode's whole reason for existing.
    for (snr, a) in rows {
        let want = match snr {
            s if s >= -6.0 => 0.98,
            s if s >= -9.0 => 0.95,
            s if s >= -12.0 => 0.85,
            _ => 0.0,
        };
        assert!(a >= want, "SNR {snr} dB: accuracy {a:.3} < {want}");
    }
}

#[test]
fn tolerates_receiver_mistuning() {
    let reps = 2;
    let audio = transmit(reps);
    println!("\nPSK31 — mistuning at 10 dB SNR");
    println!("  offset   accuracy  copies");
    let mut rows = Vec::new();
    for off in [-20.0, -12.0, -8.0, -4.0, 0.0, 4.0, 8.0, 12.0, 20.0] {
        let ch = Channel::clean().snr(10.0).offset(off).seed(200);
        let (a, c) = score(&audio, ch, 3, reps);
        println!("  {off:>5.0} Hz   {a:>6.3}   {c:>5.2}");
        rows.push((off, a));
    }
    // The Costas loop pulls in a few Hz of cursor error either way. Beyond
    // about ±7.8 Hz (a quarter of the symbol rate) differential detection is
    // out of road regardless of the loop: the per-symbol rotation exceeds 90°
    // and the bit decision inverts. Widening the loop to chase it costs real
    // low-SNR performance, so this is a deliberate limit, not an oversight —
    // the operator clicks the signal on the waterfall to well inside it.
    for (off, a) in rows {
        let want = if off.abs() <= 4.0 { 0.95 } else { 0.0 };
        assert!(a >= want, "offset {off} Hz: accuracy {a:.3} < {want}");
    }
}

#[test]
fn survives_fading_paths() {
    let reps = 3;
    let audio = transmit(reps);
    println!("\nPSK31 — Watterson fading");
    println!("  path       SNR   accuracy  copies");
    let cases = [
        ("good", Fading::GOOD, 10.0, 0.97),
        ("good", Fading::GOOD, 0.0, 0.92),
        ("moderate", Fading::MODERATE, 10.0, 0.96),
        ("moderate", Fading::MODERATE, 0.0, 0.90),
        ("poor", Fading::POOR, 10.0, 0.90),
    ];
    for (name, f, snr, want) in cases {
        let ch = Channel::clean().snr(snr).fading(f).seed(300);
        let (a, c) = score(&audio, ch, 5, reps);
        println!("  {name:<9} {snr:>4.0} dB   {a:>6.3}   {c:>5.2}");
        assert!(a >= want, "{name} fading: accuracy {a:.3} < {want}");
    }
}

#[test]
fn level_independent() {
    let reps = 2;
    let audio = transmit(reps);
    println!("\nPSK31 — absolute level at 10 dB SNR");
    println!("   gain    accuracy");
    for g in [0.001, 0.01, 0.1, 1.0, 4.0] {
        let ch = Channel::clean().snr(10.0).gain(g).seed(400);
        let (a, _) = score(&audio, ch, 2, reps);
        println!("  {g:>6.3}    {a:>6.3}");
        assert!(a >= 0.95, "gain {g}: accuracy {a:.3} — decoder has a fixed threshold");
    }
}

#[test]
fn noise_only_stays_quiet() {
    let n = (RATE * 20.0) as usize;
    println!("\nPSK31 — receiver on pure noise (20 s)");
    for level in [0.01f64, 0.1, 1.0] {
        let mut rng = common::Rng::new(600 + (level * 1000.0) as u64);
        let noise: Vec<f32> = (0..n).map(|_| (rng.normal() * level) as f32).collect();
        let got = decode(&noise);
        let cps = got.chars().count() as f64 / (n as f64 / RATE);
        println!("  level {level:>5.2}: {:>5} chars ({cps:>5.1}/s)", got.chars().count());
        // Varicode has no framing to fail, so this is entirely the carrier-
        // coherence gate's doing; without it the figure is ~3.5 chars/s.
        assert!(cps < 1.5, "noise at {level} produced {cps:.1} chars/s: {got:?}");
    }
}

#[test]
fn tolerates_sound_card_clock_error() {
    // TX and RX sound cards are never the same speed. A symbol-timing loop
    // tuned against a shared clock looks perfect here and slips on the air, so
    // hold it to a realistic ±100 ppm.
    let reps = 3;
    let audio = transmit(reps);
    println!("\nPSK31 — transmitter clock error at 10 dB SNR");
    println!("    ppm   accuracy  copies");
    for ppm in [-200.0, -100.0, -50.0, 0.0, 50.0, 100.0, 200.0] {
        let ch = Channel::clean().snr(10.0).clock_ppm(ppm).seed(700);
        let (a, c) = score(&audio, ch, 2, reps);
        println!("  {ppm:>5.0}    {a:>6.3}   {c:>5.2}");
        if ppm.abs() <= 100.0 {
            assert!(a >= 0.95, "{ppm} ppm: accuracy {a:.3}");
        }
    }
}

#[test]
fn acquires_symbol_timing_from_any_phase() {
    // Sweeping the delay across a whole symbol is the only thing that actually
    // exercises the Gardner loop: at zero delay the transmitter and receiver
    // happen to be sample-aligned and the loop can be a no-op and still score
    // perfectly.
    let reps = 2;
    let audio = transmit(reps);
    let sps = (RATE / 31.25) as usize; // 256 samples per symbol
    println!("\nPSK31 — symbol-timing acquisition at 10 dB SNR");
    println!("  delay(sym)  accuracy  copies");
    for k in 0..8 {
        let d = k * sps / 8;
        let ch = Channel::clean().snr(10.0).delay(d).seed(800);
        let (a, c) = score(&audio, ch, 2, reps);
        println!("  {:>8.3}    {a:>6.3}   {c:>5.2}", d as f64 / sps as f64);
        assert!(a >= 0.95, "delay {d} samples: accuracy {a:.3}");
    }
}
