//! The decoder against signals built the way the standard describes them:
//! carrier, 30 Hz variable at the receiver's bearing, FM subcarrier, keyed ident.

use std::f64::consts::TAU;

use sdroxide_vor::{morse, VorDecoder};

/// Deterministic noise in [-1, 1).
struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> f64 {
        self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((self.0 >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0
    }
}

struct Beacon {
    radial_deg: f64,
    var_depth: f64,
    ref_depth: f64,
    deviation_hz: f64,
    ident: Option<(&'static str, f64)>, // text, dot seconds
    ident_depth: f64,
    noise: f64,
}

impl Default for Beacon {
    fn default() -> Self {
        Self {
            radial_deg: 0.0,
            var_depth: 0.30,
            ref_depth: 0.30,
            deviation_hz: 480.0,
            ident: None,
            ident_depth: 0.10,
            noise: 0.0,
        }
    }
}

/// On/off per 1 ms of a keyed ident that repeats every `period_s`.
fn keying(text: &str, dot_s: f64, period_s: f64, t: f64) -> bool {
    let mut marks: Vec<(f64, f64)> = Vec::new();
    let mut at = 0.0;
    for (i, ch) in text.chars().enumerate() {
        if i > 0 {
            at += 3.0 * dot_s;
        }
        for (j, sym) in morse::encode_char(ch).unwrap().chars().enumerate() {
            if j > 0 {
                at += dot_s;
            }
            let len = if sym == '-' { 3.0 * dot_s } else { dot_s };
            marks.push((at, at + len));
            at += len;
        }
    }
    let phase = t.rem_euclid(period_s);
    marks.iter().any(|&(a, b)| phase >= a && phase < b)
}

fn render(b: &Beacon, fs: u32, seconds: f64) -> Vec<f32> {
    let n = (fs as f64 * seconds) as usize;
    let mut rng = Lcg(0x5EED);
    let theta = b.radial_deg.to_radians();
    let beta = b.deviation_hz / 30.0;
    (0..n)
        .map(|i| {
            let t = i as f64 / fs as f64;
            let var = b.var_depth * (TAU * 30.0 * t - theta).cos();
            let reference = b.ref_depth * (TAU * 9960.0 * t + beta * (TAU * 30.0 * t).sin()).cos();
            let ident = match b.ident {
                Some((text, dot)) if keying(text, dot, 6.0, t) => {
                    b.ident_depth * (TAU * 1020.0 * t).cos()
                }
                _ => 0.0,
            };
            (1.0 + var + reference + ident + b.noise * rng.next()) as f32
        })
        .collect()
}

fn err_deg(a: f32, b: f64) -> f64 {
    let d = (a as f64 - b).rem_euclid(360.0);
    d.min(360.0 - d)
}

fn last_reading(fs: u32, b: &Beacon, seconds: f64) -> sdroxide_vor::Reading {
    let mut d = VorDecoder::new(fs).unwrap();
    for chunk in render(b, fs, seconds).chunks(1000) {
        d.push(chunk);
    }
    d.reading().expect("a reading after a few seconds")
}

#[test]
fn every_radial_is_found_within_half_a_degree() {
    for deg in (0..360).step_by(15).chain([1, 89, 179, 271, 359]) {
        let b = Beacon { radial_deg: deg as f64, ..Beacon::default() };
        let r = last_reading(48_000, &b, 3.0);
        assert!(r.locked, "{deg}: not locked ({r:?})");
        assert!(err_deg(r.radial_deg, deg as f64) < 0.5, "{deg}: got {}", r.radial_deg);
        assert!((r.var_depth - 0.30).abs() < 0.01, "{deg}: depth {}", r.var_depth);
        assert!((r.ref_deviation_hz - 480.0).abs() < 15.0, "{deg}: dev {}", r.ref_deviation_hz);
    }
}

#[test]
fn other_audio_rates_agree() {
    for fs in [24_000, 32_000, 44_100, 96_000] {
        for deg in [0.0, 47.0, 133.0, 250.0, 330.0] {
            let b = Beacon { radial_deg: deg, ..Beacon::default() };
            let r = last_reading(fs, &b, 3.0);
            assert!(r.locked, "{fs} Hz {deg}: not locked");
            assert!(err_deg(r.radial_deg, deg) < 1.0, "{fs} Hz {deg}: got {}", r.radial_deg);
        }
    }
}

#[test]
fn noise_costs_degrees_not_the_radial() {
    for deg in [10.0, 100.0, 200.0, 300.0] {
        let b = Beacon { radial_deg: deg, noise: 0.15, ..Beacon::default() };
        let r = last_reading(48_000, &b, 4.0);
        assert!(r.locked, "{deg}: lost the lock in noise");
        assert!(err_deg(r.radial_deg, deg) < 3.0, "{deg}: got {}", r.radial_deg);
    }
}

#[test]
fn noise_alone_is_never_locked() {
    let b = Beacon { var_depth: 0.0, ref_depth: 0.0, noise: 0.2, ..Beacon::default() };
    let r = last_reading(48_000, &b, 4.0);
    assert!(!r.locked, "{r:?}");
}

#[test]
fn a_missing_reference_is_not_locked() {
    let b = Beacon { ref_depth: 0.0, ..Beacon::default() };
    assert!(!last_reading(48_000, &b, 3.0).locked);
}

#[test]
fn the_phase_offset_moves_the_radial() {
    let b = Beacon { radial_deg: 100.0, ..Beacon::default() };
    let mut d = VorDecoder::new(48_000).unwrap();
    d.set_phase_offset_deg(-25.0);
    for chunk in render(&b, 48_000, 3.0).chunks(1000) {
        d.push(chunk);
    }
    assert!(err_deg(d.reading().unwrap().radial_deg, 75.0) < 0.5);
}

fn ident_after(text: &'static str, dot_s: f64, noise: f64) -> Option<String> {
    let b = Beacon { radial_deg: 200.0, ident: Some((text, dot_s)), noise, ..Beacon::default() };
    let mut d = VorDecoder::new(48_000).unwrap();
    let mut announced = None;
    for chunk in render(&b, 48_000, 24.0).chunks(480) {
        if let Some(id) = d.push(chunk).new_ident {
            announced = Some(id);
        }
    }
    assert_eq!(d.ident().map(str::to_owned), announced);
    announced
}

#[test]
fn the_ident_is_read_at_the_speeds_beacons_use() {
    for (text, dot) in [("EPL", 0.12), ("TST", 0.14), ("ABR", 0.17), ("PTV", 0.12), ("K9", 0.15)] {
        assert_eq!(ident_after(text, dot, 0.0).as_deref(), Some(text), "{text} at {dot} s");
    }
}

#[test]
fn the_ident_survives_noise() {
    assert_eq!(ident_after("EPL", 0.12, 0.08).as_deref(), Some("EPL"));
}

#[test]
fn no_ident_is_invented_from_noise() {
    let b = Beacon { noise: 0.2, ..Beacon::default() };
    let mut d = VorDecoder::new(48_000).unwrap();
    for chunk in render(&b, 48_000, 30.0).chunks(480) {
        assert!(d.push(chunk).new_ident.is_none());
    }
    assert!(d.ident().is_none());
}

#[test]
fn a_steady_tone_is_not_morse() {
    // A carrier-wave whistle on 1020 Hz: always on, never an ident.
    let n = 48_000 * 10;
    let audio: Vec<f32> =
        (0..n).map(|i| 1.0 + 0.1 * (TAU * 1020.0 * i as f64 / 48_000.0).cos() as f32).collect();
    let mut d = VorDecoder::new(48_000).unwrap();
    assert!(d.push(&audio).new_ident.is_none());
}
