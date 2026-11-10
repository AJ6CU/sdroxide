//! WSPR decoder against a real recording.
//!
//! The unit tests in `wspr::decode` synthesise their own signal, which proves
//! the plumbing and the SNR calibration but cannot prove recall: a decoder and
//! a transmitter written by the same hand agree with each other by construction.
//! Real air has fading, drift, splatter, and three beacons inside four hertz of
//! each other. Only a recording says how many of those come out.
//!
//! The reference is WSJT-X's own `samples/WSPR/150426_0918.wav` — 120 s of 20 m
//! at 12 kHz mono, the sample `wsprd`'s own regression figures are quoted
//! against. It is not redistributed here, so this test **skips** when it is
//! absent rather than failing. Point it at one with:
//!
//! ```text
//! SDROXIDE_WSPR_WAV=/path/to/150426_0918.wav cargo test -p sdroxide-digi --release
//! ```
//!
//! or drop the file in a WSJT-X checkout beside this repository.

use std::path::{Path, PathBuf};

use sdroxide_digi::wspr::decode_slot;

/// Callsigns `wsprd` recovers from the reference recording.
///
/// `mfsk-core`'s decoder does not reach all eight — its own test notes five —
/// so this asserts a floor rather than the full set, and prints what it got so
/// a regression shows up as a smaller number rather than as silence.
const GOLDEN: &[&str] = &["ND6P", "W5BIT", "WD4LHT", "NM7J", "KI7CI", "DJ6OL", "W3HH", "W3BI"];

/// How many of [`GOLDEN`] must come out. Raise this when the decoder improves;
/// never lower it without saying why.
const MIN_RECALL: usize = 5;

fn sample_path() -> Option<PathBuf> {
    if let Ok(p) = std::env::var("SDROXIDE_WSPR_WAV") {
        let p = PathBuf::from(p);
        return p.is_file().then_some(p);
    }
    let manifest = std::env::var("CARGO_MANIFEST_DIR").ok()?;
    // ../../../WSJT-X/samples/... — a checkout beside this repository.
    let p = Path::new(&manifest).join("../../../WSJT-X/samples/WSPR/150426_0918.wav");
    p.canonicalize().ok().filter(|p| p.is_file())
}

/// Read a mono 12 kHz WAV as `f32` in −1..1.
fn read_wav(path: &Path) -> (Vec<f32>, u32) {
    let mut r = hound::WavReader::open(path).expect("open sample");
    let spec = r.spec();
    let samples: Vec<f32> = match spec.sample_format {
        hound::SampleFormat::Int => {
            let scale = 1.0 / (1i64 << (spec.bits_per_sample - 1)) as f32;
            r.samples::<i32>().map(|s| s.expect("sample") as f32 * scale).collect()
        }
        hound::SampleFormat::Float => r.samples::<f32>().map(|s| s.expect("sample")).collect(),
    };
    // Take the left channel if it is not already mono.
    let mono = if spec.channels > 1 {
        samples.iter().step_by(spec.channels as usize).copied().collect()
    } else {
        samples
    };
    (mono, spec.sample_rate)
}

#[test]
fn the_reference_recording_yields_the_beacons_wsprd_finds() {
    let Some(path) = sample_path() else {
        eprintln!(
            "skipping: no WSPR reference recording. Set SDROXIDE_WSPR_WAV to \
             WSJT-X's samples/WSPR/150426_0918.wav to run this."
        );
        return;
    };
    let (audio, rate) = read_wav(&path);
    assert_eq!(rate, 12_000, "the reference is 12 kHz; {} is not", path.display());

    let decodes = decode_slot(&audio, rate);
    let calls: Vec<String> = decodes
        .iter()
        .map(|d| d.message.to_string().split_whitespace().next().unwrap_or("").to_string())
        .collect();
    let hits: Vec<&&str> = GOLDEN.iter().filter(|g| calls.iter().any(|c| c == **g)).collect();

    for d in &decodes {
        eprintln!(
            "{:>8.1} Hz  dt {:+5.1}  snr {:+4.0}  drift {:+3.0}  {}",
            d.carrier_hz(),
            d.dt_sec,
            d.snr_db,
            d.drift_hz,
            d.message
        );
    }
    eprintln!("recall {}/{} of the golden set: {hits:?}", hits.len(), GOLDEN.len());

    assert!(
        hits.len() >= MIN_RECALL,
        "recall fell to {}/{} (wanted at least {MIN_RECALL}); decoded {calls:?}",
        hits.len(),
        GOLDEN.len()
    );
    // Every decode must be placed inside the window it is physically confined
    // to. A call outside it is a false decode however plausible it reads.
    for d in &decodes {
        assert!(
            (1380.0..=1620.0).contains(&d.carrier_hz()),
            "{} decoded at {} Hz, outside the WSPR window",
            d.message,
            d.carrier_hz()
        );
    }
}
