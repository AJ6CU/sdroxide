//! NAVTEX against real SITOR-B recordings.
//!
//! Two fixtures:
//!
//! 1. `SITOR-B.wav` — the SITOR-B sample from sigidwiki (MONDOLFO RADIO
//!    broadcasting a coastal warning on 518 kHz), 8 kHz mono, the original
//!    fixture. Kept as a regression test: the decoder must still read it.
//!
//! 2. `Bermuda-48k.wav` — a 48 kHz mono recording of Bermuda Radio on 518 kHz
//!    (operator F4NRM, 2026-10-22, 180 s, 17 MB). This is the production
//!    rate the engine actually feeds the decoder at; it is the fixture that
//!    exposed the ratio-discriminator bug, which only manifested at 48 kHz.
//!
//! The reference text `Bermuda-48k.txt` is itself a lossy decode (it carries
//! ~15 character-level errors and a figures/letters garbled run), so the test
//! asserts message content — anchors and decoding metrics — not byte equality.
//! See AGENTS.md: "A passing test is evidence about the test, not about the
//! program."

use sdroxide_dsp::NavtexRx;

const RATE_8K: f64 = 8_000.0;
const RATE_48K: f64 = 48_000.0;

fn load_wav(path: &str) -> Vec<f32> {
    let mut reader = hound::WavReader::open(path).unwrap_or_else(|e| panic!("{path}: {e}"));
    let spec = reader.spec();
    assert_eq!(spec.channels, 1, "{path}");
    let x: Vec<f32> = reader.samples::<i16>().map(|s| f32::from(s.unwrap()) / 32768.0).collect();
    x
}

/// What Bermuda Radio sent, restored by hand from the decodes: the forecast is
/// plain English and every doubtful character has only one reading. It is the
/// measure the Bermuda test scores against, so "close to what was sent" is a
/// number rather than a handful of anchors.
const BERMUDA_SENT: &str = "ZCZC BE22 BERMUDA RADIO 261021 1930Z BERMUDA MARINE FORCAST \
MARINE SYNOPSIS - AS LOW PRESSURE TO THE DISTANT NORTHWEST MOVES TO OUR DISTANT NORTH, WEAK \
TROUGHING MAY BRING ISOLATED SHOWERS LATER TONIGHT INTO EARLY TOMORROW. HIGH PRESSURE THEN \
PRESSES IN FROM THE NORTHWEST AND STARTS TO INTERACT WITH A COMPLEX AREA OF LOW PRESSURE \
FORMING TO THE DISTANT SOUTHWEST CREATING A PRESSURE SQUEEZE. LIGHT TO MODERATE WINDS \
GRADUALLY INCREASE TO STRONG OVER THE WEEKEND. MODERATE SEAS BUILD ROUGH TOWARDS THE END OF \
THE WEEK. TONIGHT - WINDS WEST-SOUTHWESTERLY 8 TO 12 KNOTS, TENDING SOUTHWESTERLY AT TIMES. \
CHANCE OF ISOLATED SHOWERS WITH FAIR VISIBILITY. SEAS OUTSIDE THE REEF 5 TO 8 FT. THURSDAY - \
WINDS WEST-SOUTHWESTERLY 8 TO 12 KNOTS, VEERING WEST-NORTHWESTERLY 12 TO 18 KNOTS OVERNIGHT. \
ISOLATED TO OCCASIONALLY SCATTERED SHOWERS WITH MAINLY FAIR VISIBILITY, TENDING TO DISSIPATE \
IN THE EVENING. SEAS OUTSIDE THE REEF 5 TO 8 FT, INCREASING DURING THE NIGHT. FRIDAY - WINDS \
NORTH-NORTHWESTERLY 12 TO 18 KNOTS, VEERING NORTH-NORTHEASTERLY OVERNIGHT. ISOLATED SHOWERS \
WITH FAIR VISIBILITY. SEAS INCREASING, OUTSIDE THE REEF 7 TO 11 FT. NNNN";

/// Characters to change, add or remove to turn `a` into `b`.
fn edit_distance(a: &str, b: &str) -> usize {
    let (a, b): (Vec<char>, Vec<char>) = (a.chars().collect(), b.chars().collect());
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.iter().enumerate() {
        let mut cur = vec![i + 1];
        for (j, cb) in b.iter().enumerate() {
            cur.push((prev[j + 1] + 1).min(cur[j] + 1).min(prev[j] + usize::from(ca != cb)));
        }
        prev = cur;
    }
    prev[b.len()]
}

/// Runs of white space as one space, so the score is about the characters and
/// not about where the station broke its lines.
fn flat(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn a_reference_recording_decodes_sitor_b() {
    let path = format!("{}/tests/testdata/navtex/SITOR-B.wav", env!("CARGO_MANIFEST_DIR"));
    let audio = load_wav(&path);
    let mut rx = NavtexRx::new(RATE_8K);
    rx.set_center_hz(2000.0);
    let mut got = String::new();
    for chunk in audio.chunks(512) {
        got.push_str(&rx.process(chunk));
    }
    println!("SITOR-B DECODED: {got:?}");
    println!("SITOR-B SYNC: {}", rx.in_sync());
    println!("SITOR-B COUNTS: {:?}", rx.counts());
    println!("SITOR-B AFC: {}", rx.afc_offset_hz());
    assert!(got.contains("ZCZC UA69"), "decoded {got:?}");
    assert!(got.contains("COASTAL WARNING"), "decoded {got:?}");
    assert!(got.contains("NNNN"), "decoded {got:?}");
    // A strong clean signal: every position of the firing area, to the digit.
    assert!(got.contains("G) 40 23 54N - 18 17 30E."), "decoded {got:?}");
    assert!(got.contains("SAILING, ANCHORAGE AND FISHING ARE PROHIBITED."), "decoded {got:?}");
    assert!(!got.contains('*'), "a clean recording lost characters: {got:?}");
    assert_eq!(rx.counts().2, 0);
}

#[test]
fn a_reference_recording_decodes_bermuda_48k() {
    let path = format!("{}/tests/testdata/navtex/Bermuda-48k.wav", env!("CARGO_MANIFEST_DIR"));
    let audio = load_wav(&path);
    let mut rx = NavtexRx::new(RATE_48K);
    rx.set_center_hz(1700.0);
    let mut got = String::new();
    for chunk in audio.chunks(512) {
        got.push_str(&rx.process(chunk));
    }
    println!("BERMUDA DECODED: {got:?}");
    println!("BERMUDA SYNC: {}", rx.in_sync());
    println!("BERMUDA COUNTS: {:?}", rx.counts());
    println!("BERMUDA AFC: {}", rx.afc_offset_hz());
    // Structural anchors — the message, not byte equality.
    assert!(got.contains("ZCZC BE22"), "decoded {got:?}");
    assert!(got.contains("BERMUDA"), "decoded {got:?}");
    assert!(got.contains("261021"), "decoded {got:?}");
    assert!(got.contains("NNNN"), "decoded {got:?}");
    // The recording fades towards its end. Before the two copies of each
    // character were read together this lost 52 characters and printed three
    // runs of figures where the shift had been misread; it is now within a
    // handful of characters of what was sent (measured: 6 of 1138).
    let lost = got.matches('*').count();
    assert!(lost <= 12, "too many lost chars ({lost}): {got:?}");
    let wrong = edit_distance(&flat(&got), BERMUDA_SENT);
    println!("BERMUDA DISTANCE: {wrong} of {}", BERMUDA_SENT.len());
    assert!(wrong <= 14, "{wrong} characters from what was sent: {got:?}");
    assert!(got.contains("BERMUDA MARINE FORCAST"), "decoded {got:?}");
    assert!(got.contains("CHANCE OF ISOLATED SHOWERS WITH FAIR VISIBILITY"), "decoded {got:?}");
    assert!(!got.contains('\u{7}'), "a misread shift printed figures for letters: {got:?}");
    // The synopsis and the forecast headings must be readable.
    assert!(got.contains("SYNOPSIS"), "decoded {got:?}");
    assert!(got.contains("TONIGHT"), "decoded {got:?}");
    assert!(got.contains("HURSDAY") || got.contains("THURSDAY"), "decoded {got:?}");
    assert!(got.contains("FRIDAY"), "decoded {got:?}");
}

#[test]
fn the_decoder_is_rate_independent() {
    // The same NAVTEX message, synthesised at 8 kHz and at 48 kHz, must
    // decode to the same anchors. This is the guard that would have caught
    // the ratio-discriminator bug, which only manifested at 48 kHz (the
    // production rate) because the per-sample constants were tuned at 8 kHz.
    use sdroxide_dsp::navtex_test::{encode_bits, synth};
    let text = "ZCZC BE22 BERMUDA RADIO NNNN";
    let bits = encode_bits(text);
    for &rate in &[RATE_8K, RATE_48K] {
        let audio = synth(&bits, rate, 1700.0, 0.5);
        let mut rx = NavtexRx::new(rate);
        rx.set_center_hz(1700.0);
        let mut got = String::new();
        for chunk in audio.chunks(512) {
            got.push_str(&rx.process(chunk));
        }
        assert!(got.contains("ZCZC"), "rate {rate}: {got:?}");
        assert!(got.contains("BERMUDA"), "rate {rate}: {got:?}");
        assert!(got.contains("NNNN"), "rate {rate}: {got:?}");
    }
}
