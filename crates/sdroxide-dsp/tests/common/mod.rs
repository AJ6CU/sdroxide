//! Synthetic HF channel + scoring helpers for the keyboard-mode decoder tests.
//!
//! The modems are cheap to run open-loop, so the interesting question is never
//! "does the loopback close" — it is "how far down into the noise does it still
//! read, and what happens when the operator is mistuned or the path fades".
//! This module builds the impairments that answer that:
//!
//! * AWGN at a stated SNR in a 3 kHz reference bandwidth (the HF convention),
//! * a receiver tuning error (the operator's cursor a few Hz off the signal),
//! * a Watterson-style two-path Rayleigh fading channel (CCIR good/moderate/
//!   poor), and
//! * a plain level scale, so a decoder that hides a fixed threshold shows it.
//!
//! Everything is seeded, so a failing sweep point reproduces exactly.

#![allow(dead_code)] // each test binary uses a different subset

use num_complex::Complex;
use rustfft::FftPlanner;

type C32 = Complex<f32>;

/// Reference bandwidth for the SNR figures, matching how HF digimode SNR is
/// normally quoted (and what WSJT-X reports).
pub const NOISE_BW: f64 = 3000.0;

// ─────────────────────────────── rng ────────────────────────────────────

/// splitmix64 — deterministic, seedable, and good enough for channel noise.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) ^ 0xDEAD_BEEF_CAFE_F00D)
    }

    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// Uniform in [0, 1).
    pub fn uniform(&mut self) -> f64 {
        (self.next_u64() >> 11) as f64 / (1u64 << 53) as f64
    }

    /// Standard normal via Box–Muller.
    pub fn normal(&mut self) -> f64 {
        let u1 = self.uniform().max(1e-12);
        let u2 = self.uniform();
        (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
    }

    fn complex_normal(&mut self) -> C32 {
        // Unit total power: each component gets variance 1/2.
        let s = std::f64::consts::FRAC_1_SQRT_2;
        C32::new((self.normal() * s) as f32, (self.normal() * s) as f32)
    }
}

// ───────────────────────────── channel ──────────────────────────────────

/// One Watterson path pair: a differential delay and a Doppler spread.
#[derive(Clone, Copy, Debug)]
pub struct Fading {
    pub delay_ms: f64,
    pub doppler_hz: f64,
}

impl Fading {
    /// CCIR 520-2 "good" HF path.
    pub const GOOD: Fading = Fading { delay_ms: 0.5, doppler_hz: 0.1 };
    /// CCIR "moderate" — the usual benchmark for HF digital modes.
    pub const MODERATE: Fading = Fading { delay_ms: 1.0, doppler_hz: 0.5 };
    /// CCIR "poor" — fast, deep fading.
    pub const POOR: Fading = Fading { delay_ms: 2.0, doppler_hz: 1.0 };
}

/// A synthetic receive path: fading, then mistuning, then level, then noise.
#[derive(Clone, Copy, Debug)]
pub struct Channel {
    /// SNR in [`NOISE_BW`]; `f64::INFINITY` for a noiseless channel.
    pub snr_db: f64,
    /// Receiver tuning error: the signal appears this far above where the
    /// decoder is looking.
    pub freq_offset_hz: f64,
    /// Linear amplitude scale applied to the signal (noise is scaled with it,
    /// so this changes absolute level without changing SNR).
    pub gain: f64,
    /// Whole samples of delay prepended before the signal. Without this the
    /// transmitter and receiver start sample-aligned and a symbol-timing loop
    /// is never asked to acquire anything — it looks perfect while doing
    /// nothing. Sweeping this across one symbol period is what tests it.
    pub delay: usize,
    /// Sample-clock error between transmitter and receiver, in parts per
    /// million. Two sound cards are never quite the same speed, so a symbol
    /// timing loop tuned against a shared clock will look far better in a test
    /// than it behaves on the air. Cheap sound cards run to ±100 ppm.
    pub clock_ppm: f64,
    pub fading: Option<Fading>,
    pub seed: u64,
}

impl Default for Channel {
    fn default() -> Self {
        Channel {
            snr_db: f64::INFINITY,
            freq_offset_hz: 0.0,
            gain: 1.0,
            delay: 0,
            clock_ppm: 0.0,
            fading: None,
            seed: 1,
        }
    }
}

impl Channel {
    /// Noiseless, on-frequency, unity gain.
    pub fn clean() -> Self {
        Channel::default()
    }

    pub fn snr(mut self, db: f64) -> Self {
        self.snr_db = db;
        self
    }
    pub fn offset(mut self, hz: f64) -> Self {
        self.freq_offset_hz = hz;
        self
    }
    pub fn gain(mut self, g: f64) -> Self {
        self.gain = g;
        self
    }
    pub fn fading(mut self, f: Fading) -> Self {
        self.fading = Some(f);
        self
    }
    pub fn clock_ppm(mut self, ppm: f64) -> Self {
        self.clock_ppm = ppm;
        self
    }
    pub fn delay(mut self, samples: usize) -> Self {
        self.delay = samples;
        self
    }
    pub fn seed(mut self, s: u64) -> Self {
        self.seed = s;
        self
    }

    /// Push `audio` through the channel. The returned buffer is the same
    /// length; SNR is referenced to the *average* pre-fading signal power, so
    /// a fading run and a flat run at the same `snr_db` carry the same energy.
    pub fn apply(&self, audio: &[f32], rate: f64) -> Vec<f32> {
        let n = audio.len();
        if n == 0 {
            return Vec::new();
        }
        let mut rng = Rng::new(self.seed);
        let sig_pow = audio.iter().map(|&v| (v as f64) * (v as f64)).sum::<f64>() / n as f64;

        // Resample and delay first: both are properties of the transmitter and
        // the path, so they happen before anything else touches the signal.
        let scratch;
        let audio = if self.clock_ppm != 0.0 || self.delay != 0 {
            let mut v = vec![0.0f32; self.delay];
            if self.clock_ppm != 0.0 {
                v.extend_from_slice(&resample(audio, 1.0 + self.clock_ppm * 1e-6));
            } else {
                v.extend_from_slice(audio);
            }
            scratch = v;
            &scratch[..]
        } else {
            audio
        };

        let needs_complex = self.fading.is_some() || self.freq_offset_hz != 0.0;
        let mut out: Vec<f32> = if needs_complex {
            let xa = analytic(audio);
            let faded = match self.fading {
                Some(f) => watterson(&xa, rate, f, &mut rng),
                None => xa,
            };
            let dph = std::f64::consts::TAU * self.freq_offset_hz / rate;
            faded
                .iter()
                .enumerate()
                .map(|(i, z)| {
                    let ph = dph * i as f64;
                    (z.re as f64 * ph.cos() - z.im as f64 * ph.sin()) as f32
                })
                .collect()
        } else {
            audio.to_vec()
        };

        for v in out.iter_mut() {
            *v *= self.gain as f32;
        }

        if self.snr_db.is_finite() {
            // Real noise of variance s² spreads over rate/2; the power landing
            // in NOISE_BW is s²·NOISE_BW/(rate/2).
            let snr_lin = 10f64.powf(self.snr_db / 10.0);
            let rx_pow = sig_pow * self.gain * self.gain;
            let sigma = (rx_pow * rate / (2.0 * NOISE_BW * snr_lin)).sqrt();
            for v in out.iter_mut() {
                *v += (rng.normal() * sigma) as f32;
            }
        }
        out
    }
}

/// Resample by `factor` with Catmull-Rom interpolation: `factor > 1` means the
/// transmitter's clock ran fast, so the signal arrives compressed in time.
/// Cubic rather than linear because linear interpolation at only 8 samples per
/// carrier cycle would colour the signal enough to muddy what is being measured.
pub fn resample(x: &[f32], factor: f64) -> Vec<f32> {
    fn at(x: &[f32], i: isize) -> f32 {
        if i < 0 || i as usize >= x.len() { 0.0 } else { x[i as usize] }
    }
    let out_len = ((x.len() as f64) / factor).floor() as usize;
    (0..out_len)
        .map(|j| {
            let t = j as f64 * factor;
            let i = t.floor() as isize;
            let f = (t - i as f64) as f32;
            let (p0, p1, p2, p3) = (at(x, i - 1), at(x, i), at(x, i + 1), at(x, i + 2));
            0.5 * (2.0 * p1
                + (p2 - p0) * f
                + (2.0 * p0 - 5.0 * p1 + 4.0 * p2 - p3) * f * f
                + (3.0 * p1 - p0 - 3.0 * p2 + p3) * f * f * f)
        })
        .collect()
}

/// Analytic (one-sided) signal, so a complex fading gain and a frequency shift
/// can be applied without producing an image.
pub fn analytic(x: &[f32]) -> Vec<C32> {
    let n = x.len();
    let mut planner = FftPlanner::new();
    let fwd = planner.plan_fft_forward(n);
    let inv = planner.plan_fft_inverse(n);
    let mut buf: Vec<C32> = x.iter().map(|&v| C32::new(v, 0.0)).collect();
    fwd.process(&mut buf);
    let half = n / 2;
    for (i, v) in buf.iter_mut().enumerate() {
        if i == 0 || (n % 2 == 0 && i == half) {
            // DC and Nyquist stay as they are.
        } else if i < half {
            *v *= 2.0;
        } else {
            *v = C32::new(0.0, 0.0);
        }
    }
    inv.process(&mut buf);
    let s = 1.0 / n as f32;
    buf.iter().map(|v| v * s).collect()
}

/// Two independent Rayleigh paths with a differential delay — the standard
/// Watterson HF model, at the fidelity a decoder test needs.
fn watterson(xa: &[C32], rate: f64, f: Fading, rng: &mut Rng) -> Vec<C32> {
    let n = xa.len();
    let delay = ((f.delay_ms / 1000.0) * rate).round() as usize;
    let g0 = doppler_process(n, rate, f.doppler_hz, rng);
    let g1 = doppler_process(n, rate, f.doppler_hz, rng);
    // Split the power evenly so the two-path sum keeps unit average gain.
    let s = std::f64::consts::FRAC_1_SQRT_2 as f32;
    (0..n)
        .map(|i| {
            let a = xa[i] * g0[i] * s;
            let b = if i >= delay { xa[i - delay] * g1[i] * s } else { C32::new(0.0, 0.0) };
            a + b
        })
        .collect()
}

/// Narrowband complex Gaussian with the given Doppler spread — two cascaded
/// one-pole lowpasses on complex white noise, normalised to unit mean power.
fn doppler_process(n: usize, rate: f64, doppler_hz: f64, rng: &mut Rng) -> Vec<C32> {
    let fc = (doppler_hz / 2.0).max(1e-3);
    let a = (-std::f64::consts::TAU * fc / rate).exp() as f32;
    let b = 1.0 - a;
    let mut s1 = C32::new(0.0, 0.0);
    let mut s2 = C32::new(0.0, 0.0);
    // Run the filter in long enough to be stationary by sample 0.
    let warm = ((rate / fc) as usize).min(400_000);
    let mut out = Vec::with_capacity(n);
    for i in 0..(warm + n) {
        let w = rng.complex_normal();
        s1 = s1 * a + w * b;
        s2 = s2 * a + s1 * b;
        if i >= warm {
            out.push(s2);
        }
    }
    let pow = out.iter().map(|z| z.norm_sqr() as f64).sum::<f64>() / n as f64;
    let k = (1.0 / pow.max(1e-30)).sqrt() as f32;
    for z in out.iter_mut() {
        *z *= k;
    }
    out
}

// ───────────────────────────── scoring ──────────────────────────────────

/// Levenshtein distance (chars), used for an alignment-tolerant score: a
/// decoder that drops a character shifts everything after it, and a decoder
/// that invents characters out of noise should be charged for them.
pub fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    if a.is_empty() {
        return b.len();
    }
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    let mut cur = vec![0usize; b.len() + 1];
    for (i, &ca) in a.iter().enumerate() {
        cur[0] = i + 1;
        for (j, &cb) in b.iter().enumerate() {
            let sub = prev[j] + usize::from(ca != cb);
            cur[j + 1] = sub.min(prev[j + 1] + 1).min(cur[j] + 1);
        }
        std::mem::swap(&mut prev, &mut cur);
    }
    prev[b.len()]
}

/// Character accuracy in [0, 1]: 1 − edit distance normalised by the longer of
/// the two strings. Spurious output counts against the score, so a decoder
/// that spews garbage from noise cannot score well by accident.
pub fn accuracy(decoded: &str, reference: &str) -> f64 {
    let dn = decoded.chars().count();
    let rn = reference.chars().count();
    if dn == 0 && rn == 0 {
        return 1.0;
    }
    1.0 - levenshtein(decoded, reference) as f64 / dn.max(rn) as f64
}

/// Non-overlapping exact occurrences of `msg` in `decoded` — the "how many
/// clean copies came through" figure an operator would recognise.
pub fn copies(decoded: &str, msg: &str) -> usize {
    if msg.is_empty() {
        return 0;
    }
    let mut n = 0;
    let mut rest = decoded;
    while let Some(i) = rest.find(msg) {
        n += 1;
        rest = &rest[i + msg.len()..];
    }
    n
}

/// `msg` repeated `k` times — the reference stream for [`accuracy`].
pub fn repeated(msg: &str, k: usize) -> String {
    msg.repeat(k)
}
