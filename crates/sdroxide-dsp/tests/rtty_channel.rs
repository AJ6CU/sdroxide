//! RTTY decoder performance against a synthetic HF channel.
//!
//! Motivated by issue #23 (a DWD broadcast that FLdigi read cleanly and this
//! decoder turned into hash). A loopback test only proves the modem talks to
//! itself; these sweeps measure the things that actually decide whether a real
//! signal reads — sensitivity in noise, tolerance of a mistuned receiver, a
//! fading path, absolute level, a transmitter whose sample clock differs from
//! ours, and the behaviour that prompted the report: what comes out when there
//! is no signal at all.

mod common;

use common::{Channel, Fading, accuracy, copies, repeated};
use sdroxide_dsp::{RttyRx, RttyTx};

const RATE: f64 = 8000.0;
const CENTER: f64 = 1000.0;

/// A DWD-flavoured message: the mix of letters, figures and shift changes that
/// makes real RTTY traffic harder than a letters-only test vector.
const MSG: &str = "CQ CQ CQ DE DDK2 DDH7 DDK9 FREQUENCIES 4583 KHZ 7646 KHZ 10100.8 KHZ ";

/// Render `reps` copies of `MSG` plus idle mark head and tail.
fn transmit(baud: f64, shift: f64, reps: usize) -> Vec<f32> {
    let mut tx = RttyTx::new(RATE, CENTER, baud, shift);
    let mut audio = Vec::new();
    let mut pre = vec![0.0f32; 4000];
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
    let mut tail = vec![0.0f32; 4000];
    tx.next_block(&mut tail);
    audio.extend_from_slice(&tail);
    audio
}

fn decode(audio: &[f32], baud: f64, shift: f64) -> String {
    let mut rx = RttyRx::new(RATE, CENTER, baud, shift);
    let mut out = String::new();
    for chunk in audio.chunks(512) {
        out.push_str(&rx.process(chunk));
    }
    out
}

/// Mean accuracy over `seeds` independent channel realisations.
fn score(audio: &[f32], baud: f64, shift: f64, ch: Channel, seeds: u64, reps: usize) -> (f64, f64) {
    let reference = repeated(MSG, reps);
    let mut acc = 0.0;
    let mut cps = 0.0;
    for s in 0..seeds {
        let rx = ch.seed(ch.seed + s).apply(audio, RATE);
        let got = decode(&rx, baud, shift);
        acc += accuracy(&got, &reference);
        cps += copies(&got, MSG) as f64;
    }
    (acc / seeds as f64, cps / seeds as f64)
}

#[test]
fn clean_channel_is_perfect() {
    let audio = transmit(45.45, 170.0, 3);
    let got = decode(&audio, 45.45, 170.0);
    assert_eq!(copies(&got, MSG), 3, "clean decode was {got:?}");
}

#[test]
fn snr_sweep_45baud_170shift() {
    let reps = 3;
    let audio = transmit(45.45, 170.0, reps);
    println!("\nRTTY 45.45 baud / 170 Hz shift — AWGN only");
    println!("  SNR(3k)   accuracy  copies");
    let mut rows = Vec::new();
    for snr in [20.0, 15.0, 10.0, 5.0, 0.0, -3.0, -6.0, -9.0, -12.0] {
        let ch = Channel::clean().snr(snr).seed(100);
        let (a, c) = score(&audio, 45.45, 170.0, ch, 4, reps);
        println!("  {snr:>6.1} dB   {a:>6.3}   {c:>5.2}");
        rows.push((snr, a));
    }
    // Measured: 1.000 down to +10 dB, ~0.98 at 0 dB, 0.93 at -6 dB. The old
    // discriminator-slicer decoder fell off a cliff at 0 dB (0.77 at -3 dB,
    // 0.19 at -6), so these thresholds are what the matched filters bought and
    // exist to stop it being given back.
    for (snr, a) in rows {
        let want = match snr {
            s if s >= 10.0 => 0.98,
            s if s >= -3.0 => 0.93,
            s if s >= -6.0 => 0.85,
            _ => 0.0,
        };
        assert!(a >= want, "SNR {snr} dB: accuracy {a:.3} < {want}");
    }
}

#[test]
fn tolerates_receiver_mistuning() {
    let reps = 2;
    let audio = transmit(45.45, 170.0, reps);
    println!("\nRTTY 45.45/170 — mistuning at 15 dB SNR");
    println!("  offset   accuracy  copies");
    let mut rows = Vec::new();
    for off in [-80.0, -60.0, -40.0, -30.0, -20.0, -10.0, 0.0, 10.0, 20.0, 30.0, 40.0, 60.0, 80.0] {
        let ch = Channel::clean().snr(15.0).offset(off).seed(200);
        let (a, c) = score(&audio, 45.45, 170.0, ch, 3, reps);
        println!("  {off:>5.0} Hz   {a:>6.3}   {c:>5.2}");
        rows.push((off, a));
    }
    // The operator's cursor lands within a few Hz by eye off the waterfall, but
    // AFC is what makes a sloppy click harmless: measured ~1.00 out to ±40 Hz
    // and ~0.89 at ±80. Without AFC the matched filters null the wanted tone
    // around ±45 Hz and it collapses to nothing, so this is the test that
    // fails if the AFC loop is broken.
    for (off, a) in rows {
        let want = if off.abs() <= 40.0 {
            0.90
        } else if off.abs() <= 80.0 {
            0.80
        } else {
            0.0
        };
        assert!(a >= want, "offset {off} Hz: accuracy {a:.3} < {want}");
    }
}

#[test]
fn survives_fading_paths() {
    let reps = 3;
    let audio = transmit(45.45, 170.0, reps);
    println!("\nRTTY 45.45/170 — Watterson fading");
    println!("  path       SNR   accuracy  copies");
    // Selective fading is what automatic threshold correction is for: with a
    // 1-2 ms delay spread the two tones sit on different parts of the channel's
    // comb and routinely arrive tens of dB apart. Without ATC the moderate and
    // poor rows drop to ~0.87 and ~0.67.
    let cases = [
        ("good", Fading::GOOD, 15.0, 0.95),
        ("good", Fading::GOOD, 6.0, 0.85),
        ("moderate", Fading::MODERATE, 15.0, 0.93),
        ("moderate", Fading::MODERATE, 6.0, 0.88),
        ("poor", Fading::POOR, 15.0, 0.90),
        ("poor", Fading::POOR, 6.0, 0.82),
    ];
    for (name, f, snr, want) in cases {
        let ch = Channel::clean().snr(snr).fading(f).seed(300);
        let (a, c) = score(&audio, 45.45, 170.0, ch, 5, reps);
        println!("  {name:<9} {snr:>4.0} dB   {a:>6.3}   {c:>5.2}");
        assert!(a >= want, "{name} fading: accuracy {a:.3} < {want}");
    }
}

#[test]
fn level_independent() {
    let reps = 2;
    let audio = transmit(45.45, 170.0, reps);
    println!("\nRTTY 45.45/170 — absolute level at 15 dB SNR");
    println!("   gain    accuracy");
    for g in [0.001, 0.01, 0.1, 1.0, 4.0] {
        let ch = Channel::clean().snr(15.0).gain(g).seed(400);
        let (a, _) = score(&audio, 45.45, 170.0, ch, 2, reps);
        println!("  {g:>6.3}    {a:>6.3}");
        assert!(a >= 0.95, "gain {g}: accuracy {a:.3} — decoder has a fixed threshold");
    }
}

#[test]
fn other_baud_and_shift_combinations() {
    println!("\nRTTY parameter combinations at 10 dB SNR");
    println!("  baud  shift   accuracy  copies");
    // 50 baud / 450 Hz is the Deutscher Wetterdienst broadcast from #23; 75/850
    // is the widest combination the panel offers.
    for (baud, shift) in
        [(45.45, 170.0), (50.0, 170.0), (50.0, 425.0), (50.0, 450.0), (75.0, 850.0)]
    {
        let reps = 2;
        let audio = transmit(baud, shift, reps);
        let ch = Channel::clean().snr(10.0).seed(500);
        let (a, c) = score(&audio, baud, shift, ch, 3, reps);
        println!("  {baud:>5.2} {shift:>5.0}    {a:>6.3}   {c:>5.2}");
        assert!(a >= 0.95, "{baud} baud / {shift} Hz: accuracy {a:.3}");
    }
}

#[test]
fn noise_only_stays_quiet() {
    // The complaint in #23 was a stream of random characters. Pure noise with
    // no signal at all must not produce a torrent of text.
    let n = (RATE * 20.0) as usize;
    println!("\nRTTY 45.45/170 — receiver on pure noise (20 s)");
    for level in [0.01f64, 0.1, 1.0] {
        let mut rng = common::Rng::new(600 + (level * 1000.0) as u64);
        let noise: Vec<f32> = (0..n).map(|_| (rng.normal() * level) as f32).collect();
        let got = decode(&noise, 45.45, 170.0);
        let cps = got.chars().count() as f64 / (n as f64 / RATE);
        println!("  level {level:>5.2}: {:>5} chars ({cps:>5.1}/s)", got.chars().count());
        assert!(cps < 1.0, "noise at {level} produced {cps:.1} chars/s: {got:?}");
    }
}

#[test]
fn tolerates_sound_card_clock_error() {
    // Same reasoning as the PSK suite: the bit clock has to track a real
    // transmitter, not a bit-exact copy of our own sample clock.
    let reps = 2;
    let audio = transmit(45.45, 170.0, reps);
    println!("\nRTTY 45.45/170 — transmitter clock error at 15 dB SNR");
    println!("    ppm   accuracy  copies");
    for ppm in [-1000.0, -300.0, -100.0, 0.0, 100.0, 300.0, 1000.0] {
        let ch = Channel::clean().snr(15.0).clock_ppm(ppm).seed(700);
        let (a, c) = score(&audio, 45.45, 170.0, ch, 2, reps);
        println!("  {ppm:>5.0}    {a:>6.3}   {c:>5.2}");
        if ppm.abs() <= 300.0 {
            assert!(a >= 0.95, "{ppm} ppm: accuracy {a:.3}");
        }
    }
}
