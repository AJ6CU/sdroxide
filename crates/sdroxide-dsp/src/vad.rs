//! Speech detection: which of the signals a scan stops on are somebody talking.
//!
//! A scanner that stops on a level stops on everything — a data burst, an
//! unmodulated carrier, a pager, a birdie, the tail of a CTCSS burst. This
//! answers the other question: is there a *human voice* in this channel, which
//! is what an operator hunting a band is actually looking for.
//!
//! **It scores speech-likeness, not "a person is talking".** Music with a
//! vocal, or a talk station with a bed of music under it, scores as voice, and
//! that is the right answer for the job: the operator wants the channels
//! carrying programme, not a semantic classification. A singing contest and a
//! QSO look alike to four measurements, and there is no fifth measurement that
//! would help.
//!
//! The four measurements come from the demodulated audio, which a software
//! radio has and a handheld's chip registers do not:
//!
//! - **structure** — speech is neither flat noise nor one steady tone. Spectral
//!   flatness (Wiener entropy) separates speech from noise; the share of the
//!   band's power sitting in its single strongest bin separates both from a
//!   carrier, which is what a beep, a birdie and a CTCSS tail all are.
//! - **voicing** — vowels are quasi-periodic, so a normalised autocorrelation
//!   over the pitch range peaks. Noise does not.
//! - **spread** — speech fills the speech band. A tone, a whistle or a narrow
//!   data subcarrier sits in one corner of it.
//! - **syllables** — the loudness of speech rises and falls a few times a
//!   second. A steady carrier is flat, noise has no rhythm, and a data burst's
//!   is far faster than a syllable.
//!
//! The score is 0–100 and the verdict is hysteresis on it, so a pause between
//! words does not drop the channel. [`SpeechDetector::score`] is the number to
//! show an operator who is wondering why the scan went past a strong signal;
//! [`SpeechDetector::speaking`] is the verdict the scan uses.
//!
//! Everything here is a pure function of the audio handed to it: no clock, no
//! I/O, no state that a test cannot set up by feeding samples.

use std::collections::VecDeque;
use std::sync::Arc;

use realfft::{RealFftPlanner, RealToComplex};

use crate::Complex32;

/// Frame length in seconds. Twenty milliseconds is two pitch periods of a low
/// male voice and two hundred of the highest harmonic that matters, which is
/// enough for both the transform and the autocorrelation; a tenth of a second
/// would be finer in frequency and too coarse in time for a syllable.
pub const FRAME_S: f64 = 0.020;

/// Frames of envelope kept for the syllable test: 0.8 s, so four syllables at
/// the usual rate.
const ENV_FRAMES: usize = 40;

/// The speech band, in hertz. Below this is hum and the CTCSS tones; above it,
/// on a 3 kHz channel, there is nothing left.
const BAND_LO_HZ: f32 = 250.0;
const BAND_HI_HZ: f32 = 3500.0;

/// Pitch search range for the voicing test.
const PITCH_LO_HZ: f64 = 60.0;
const PITCH_HI_HZ: f64 = 400.0;

/// Rate the autocorrelation runs at. It is decimated to this from whatever the
/// demodulator produces (48 kHz for most modes, 12 kHz for WFM), which cuts the
/// pitch search by a factor of four to sixteen and costs nothing that matters:
/// the pitch information is below 400 Hz.
const ACF_RATE_HZ: f64 = 12_000.0;

/// A frame at or below this is not worth scoring at all — a demodulator with no
/// modulation on it produces something close to digital silence, and the noise
/// floor of an empty channel can be sixty decibels down.
const SILENCE_DB: f32 = -90.0;

/// How far below the recently-heard peak a frame has to fall to count as a gap.
/// Speech swings much further than this between syllables; a steady carrier
/// does not move at all; and the floor of a quiet channel never rises to the
/// peak in the first place.
const ACTIVE_RANGE_DB: f32 = 15.0;

/// Peak decay per frame, so a signal that stops leaves the gate behind within
/// about a second instead of holding the next, quieter signal at zero.
const PEAK_DECAY_DB: f32 = 0.5;

/// The score a frame must reach to count as speech.
///
/// **Above what structure, voicing and spread alone can add up to** (25 + 30 +
/// 10 = 65), so a signal has to show a fourth thing — the rhythm of syllables —
/// before any frame of it counts. That is what keeps a buzzer, a raucous
/// carrier and a receiver's own settling thump out without needing a special
/// case for any of them: measured on this bench, a bare AM carrier produces
/// exactly one 65-scoring frame as its DDC settles after a retune, and a
/// steady pulse train scores 65 on every frame it has.
pub const ACCEPT: f32 = 70.0;
pub const RELEASE: f32 = 50.0;

/// Frames of verdict kept: half a second, which is also how long the call
/// survives a pause between words.
const VERDICT_FRAMES: usize = 25;

/// How many of those frames have to have been speech. Four is two syllables'
/// worth of accepted audio, and more than any single transient has ever
/// produced here.
const VERDICT_MIN: usize = 4;

/// A single frame is scored only when it carries this much of the band's power
/// in one bin, *and* is as unstructured as a tone. One of the two alone is not a
/// carrier: a vowel can put a third of its power in a formant, and windowed
/// noise can be flat without being unstructured in the same way.
const TONAL_SHARE: f32 = 0.45;
const TONAL_FLATNESS: f32 = 0.08;

/// Speech detection for one audio stream.
pub struct SpeechDetector {
    rate: f64,
    /// Samples per frame at `rate`.
    frame: usize,
    /// Decimation for the autocorrelation, and the rate it runs at.
    decim: usize,
    acf_rate: f64,

    fft: Arc<dyn RealToComplex<f32>>,
    window: Vec<f32>,
    /// Raw frame samples, then the windowed copy the transform consumes.
    frame_buf: Vec<f32>,
    fft_in: Vec<f32>,
    spec: Vec<Complex32>,
    scratch: Vec<Complex32>,
    /// Power per bin of the speech band only, so the tests that walk it walk
    /// the part that matters.
    band_lo_bin: usize,
    band_hi_bin: usize,

    /// Decimated frame for the autocorrelation.
    low: Vec<f32>,
    /// Samples that do not yet make a whole frame.
    pending: Vec<f32>,

    /// Recent frames: level in dBFS, and whether it was above the gate.
    env: VecDeque<(f32, bool)>,
    /// Frames analysed since the last reset, capped at [`ENV_FRAMES`], so the
    /// syllable rate is not taken over a window that has not filled yet.
    seen: usize,
    /// Decaying peak of the frame level, and the gate derived from it.
    peak_db: f32,
    /// Smoothed score for display.
    score: f32,
    /// One entry per recent frame: did it count as speech. The verdict is a
    /// count over this window rather than a latch, so one transient frame
    /// cannot make a channel into a conversation.
    verdict: VecDeque<bool>,
}

impl SpeechDetector {
    pub fn new(rate: f64) -> Self {
        let rate = if rate.is_finite() && rate > 1000.0 { rate } else { 48_000.0 };
        let frame = ((rate * FRAME_S).round() as usize).max(32);
        let decim = ((rate / ACF_RATE_HZ).round() as usize).max(1);
        let acf_rate = rate / decim as f64;
        let fft = RealFftPlanner::<f32>::new().plan_fft_forward(frame);
        // Hann: the flatness test compares bins with each other, so a window
        // that tapers to nothing at the edges is what stops a frame boundary
        // from reading as broadband energy.
        let window: Vec<f32> = (0..frame)
            .map(|i| {
                let x = std::f32::consts::TAU * i as f32 / frame as f32;
                // Periodic Hann, the form that keeps a steady tone in one bin.
                0.5 - 0.5 * x.cos()
            })
            .collect();
        let bin_hz = rate / frame as f64;
        let band_lo_bin = (BAND_LO_HZ as f64 / bin_hz).floor().max(1.0) as usize;
        let band_hi_bin = ((BAND_HI_HZ as f64 / bin_hz).ceil() as usize).min(frame / 2);
        let mut d = SpeechDetector {
            rate,
            frame,
            decim,
            acf_rate,
            fft,
            window,
            frame_buf: vec![0.0; frame],
            fft_in: vec![0.0; frame],
            spec: vec![Complex32::new(0.0, 0.0); frame / 2 + 1],
            scratch: Vec::new(),
            band_lo_bin: band_lo_bin.min(band_hi_bin.saturating_sub(1)),
            band_hi_bin,
            low: Vec::with_capacity(frame / 4),
            pending: Vec::with_capacity(frame * 4),
            env: VecDeque::with_capacity(ENV_FRAMES),
            seen: 0,
            peak_db: SILENCE_DB,
            score: 0.0,
            verdict: VecDeque::with_capacity(VERDICT_FRAMES),
        };
        d.scratch = d.fft.make_scratch_vec();
        d
    }

    pub fn set_rate(&mut self, rate: f64) {
        if (rate - self.rate).abs() > 1.0 && rate.is_finite() && rate > 1000.0 {
            *self = SpeechDetector::new(rate);
        }
    }

    /// Forget everything heard so far.
    ///
    /// Called when the receiver moves: a scan's next candidate is a different
    /// channel, and a syllable rhythm carried across the retune would be a
    /// rhythm from somewhere else.
    pub fn reset(&mut self) {
        self.pending.clear();
        self.env.clear();
        self.seen = 0;
        self.peak_db = SILENCE_DB;
        self.score = 0.0;
        self.verdict.clear();
    }

    /// Consume audio at the detector's rate.
    pub fn process(&mut self, audio: &[f32]) {
        self.pending.extend_from_slice(audio);
        while self.pending.len() >= self.frame {
            self.frame_buf.copy_from_slice(&self.pending[..self.frame]);
            self.pending.drain(..self.frame);
            // Handed over rather than borrowed so `analyse` can take `&mut
            // self`: the frame buffer goes with it and comes back, which keeps
            // one allocation for the life of the detector instead of one every
            // twenty milliseconds.
            let frame = std::mem::take(&mut self.frame_buf);
            self.analyse(&frame);
            self.frame_buf = frame;
        }
    }

    /// Speech-likeness of what has been heard, 0–100, smoothed for display.
    pub fn score(&self) -> f32 {
        self.score
    }

    /// Is somebody speaking.
    ///
    /// A count over the last half second rather than the reading from the last
    /// frame: speech keeps producing accepted frames, a transient produces one,
    /// and a pause of up to half a second does not empty the window — which is
    /// what rides out the gap between words.
    pub fn speaking(&self) -> bool {
        self.verdict.iter().filter(|a| **a).count() >= VERDICT_MIN
    }

    fn analyse(&mut self, frame: &[f32]) {
        let n = frame.len();
        let ms = frame.iter().map(|s| s * s).sum::<f32>() / n as f32;
        let db = 10.0 * (ms + 1e-12).log10();
        self.peak_db = if db > self.peak_db { db } else { self.peak_db - PEAK_DECAY_DB };
        self.peak_db = self.peak_db.max(SILENCE_DB);
        let active = db > SILENCE_DB && db > self.peak_db - ACTIVE_RANGE_DB;

        if self.env.len() == ENV_FRAMES {
            self.env.pop_front();
        }
        self.env.push_back((db, active));
        self.seen = (self.seen + 1).min(ENV_FRAMES);

        let raw = if active { self.frame_score(frame) } else { 0.0 };

        if self.verdict.len() == VERDICT_FRAMES {
            self.verdict.pop_front();
        }
        self.verdict.push_back(raw >= ACCEPT);

        // The number shown does not pretend to be the verdict, so an operator
        // reading 58 is reading a channel that is not being called speech.
        if !self.speaking() && raw < RELEASE {
            self.score = raw;
        } else {
            // Attack faster than it releases: a syllable should show up in the
            // number the operator is watching, and a gap should not flicker it
            // to zero and back.
            let a = if raw > self.score { 0.6 } else { 0.25 };
            self.score += (raw - self.score) * a;
        }
    }

    /// Score one frame of audio against the four measurements.
    fn frame_score(&mut self, frame: &[f32]) -> f32 {
        // The transform: windowed, so the tone test measures the tone and not
        // the frame edge.
        for (o, (s, w)) in self.fft_in.iter_mut().zip(frame.iter().zip(&self.window)) {
            *o = s * w;
        }
        if self
            .fft
            .process_with_scratch(&mut self.fft_in, &mut self.spec, &mut self.scratch)
            .is_err()
        {
            return 0.0;
        }
        let lo = self.band_lo_bin;
        let hi = self.band_hi_bin.max(lo + 1);
        let mut total = 0.0f32;
        let mut peak = 0.0f32;
        for c in &self.spec[lo..hi] {
            let p = c.norm_sqr();
            total += p;
            if p > peak {
                peak = p;
            }
        }
        if !(total > 0.0) {
            return 0.0;
        }

        let flatness = flatness(&self.spec, lo, hi);
        let share = peak / total;
        let tonal = share > TONAL_SHARE && flatness < TONAL_FLATNESS;

        let mut s = 0.0;

        // 1. Structure. Speech has peaks and valleys across the band; noise is
        // flat; a tone is one spike.
        if tonal {
            // Nothing else about a carrier is worth scoring: it is periodic, it
            // fills the band it is in, and a beep can even be keyed in syllables.
            return 5.0;
        }
        if flatness <= 0.35 {
            s += 25.0;
        } else if flatness <= 0.55 {
            s += 8.0;
        }

        // 2. Voicing.
        self.decimate(frame);
        let r = periodicity(
            &self.low,
            (self.acf_rate / PITCH_HI_HZ) as usize,
            ((self.acf_rate / PITCH_LO_HZ) as usize).min(self.low.len().saturating_sub(2)),
        );
        if r >= 0.40 {
            s += 30.0;
        } else if r >= 0.25 {
            s += 15.0;
        }

        // 3. Spread across the speech band.
        s += 10.0 * band_occupancy(&self.spec, lo, hi, self.rate / self.frame as f64) / 3.0;

        // 4. Syllables. Gated on voicing because the rhythm of speech rides on
        // its vowels; an unvoiced burst of noise has no rhythm worth the name,
        // and a data burst's is far faster than a syllable.
        if r >= 0.20 {
            let rate = self.syllable_rate();
            if (2.0..=9.0).contains(&rate) {
                s += 25.0;
            } else if (1.2..2.0).contains(&rate) {
                s += 12.0;
            }
        }

        s.min(95.0)
    }

    /// Boxcar-decimate the frame to the autocorrelation's rate.
    fn decimate(&mut self, frame: &[f32]) {
        self.low.clear();
        if self.decim == 1 {
            self.low.extend_from_slice(frame);
            return;
        }
        for chunk in frame.chunks_exact(self.decim) {
            self.low.push(chunk.iter().sum::<f32>() / self.decim as f32);
        }
    }

    /// How many times a second the loudness has risen lately, in hertz.
    ///
    /// Counted with hysteresis around the mean of the frames that were above
    /// the gate: a syllable has to come back up by three decibels over the mean
    /// to count, so the ripple on a steady signal does not.
    fn syllable_rate(&self) -> f32 {
        let frames = self.seen.max(1);
        if frames < 5 {
            return 0.0;
        }
        let env: Vec<&(f32, bool)> = self.env.iter().collect();
        // Only the window that has actually been heard, so a fresh detector
        // does not report four syllables a second from a quarter second of
        // audio.
        let env = &env[env.len().saturating_sub(frames)..];
        let active: Vec<f32> = env.iter().filter(|(_, a)| *a).map(|(db, _)| *db).collect();
        if active.len() < 4 {
            return 0.0;
        }
        let mean = active.iter().sum::<f32>() / active.len() as f32;
        let (hi, lo) = (mean + 3.0, mean - 3.0);
        let mut rises = 0;
        let mut armed = false;
        for (db, _) in env {
            if *db < lo {
                armed = true;
            } else if armed && *db > hi {
                rises += 1;
                armed = false;
            }
        }
        let seconds = env.len() as f32 * FRAME_S as f32;
        if seconds <= 0.0 { 0.0 } else { rises as f32 / seconds }
    }
}

/// Spectral flatness (Wiener entropy) of `power[lo..hi]`: 1 for white noise,
/// approaching 0 as the energy concentrates into fewer and fewer bins.
pub fn flatness(power: &[Complex32], lo: usize, hi: usize) -> f32 {
    let hi = hi.min(power.len());
    if lo >= hi {
        return 0.0;
    }
    let n = hi - lo;
    let mut log_sum = 0.0f64;
    let mut sum = 0.0f64;
    for c in &power[lo..hi] {
        // Floored rather than allowed to reach zero: one empty bin would take
        // the mean of the logarithms to minus infinity and call a tone flatter
        // than noise.
        let p = c.norm_sqr().max(1e-20) as f64;
        log_sum += p.ln();
        sum += p;
    }
    let mean = sum / n as f64;
    if mean <= 0.0 {
        return 0.0;
    }
    ((log_sum / n as f64).exp() / mean) as f32
}

/// How many of three sub-bands carry a worth-mentioning share of the power:
/// 0 for a whistle, 3 for speech or noise.
pub fn band_occupancy(power: &[Complex32], lo: usize, hi: usize, bin_hz: f64) -> f32 {
    let hi = hi.min(power.len());
    if lo >= hi {
        return 0.0;
    }
    let edges = [250.0, 700.0, 2000.0, 3500.0];
    let mut total = 0.0f32;
    let mut bands = [0.0f32; 3];
    for (i, c) in power[lo..hi].iter().enumerate() {
        let hz = (lo + i) as f64 * bin_hz;
        let p = c.norm_sqr();
        total += p;
        for b in 0..3 {
            if hz >= edges[b] && hz < edges[b + 1] {
                bands[b] += p;
            }
        }
    }
    if total <= 0.0 {
        return 0.0;
    }
    bands.iter().filter(|b| **b / total >= 0.03).count() as f32
}

/// Normalised autocorrelation of `x` over `lo..hi` lags, at its strongest: 1
/// for a periodic signal, near 0 for noise.
///
/// Normalised by the energy of the two overlapping halves rather than by
/// `r(0)`, so a frame that is half speech and half silence — the frame a
/// syllable starts in — does not read as unvoiced just because most of its
/// energy is at one end.
pub fn periodicity(x: &[f32], lo: usize, hi: usize) -> f32 {
    let n = x.len();
    let lo = lo.max(1);
    let hi = hi.min(n.saturating_sub(2));
    if lo >= hi {
        return 0.0;
    }
    let mean = x.iter().sum::<f32>() / n.max(1) as f32;
    let mut best = 0.0f32;
    for lag in lo..=hi {
        let mut xy = 0.0f32;
        let mut xx = 0.0f32;
        let mut yy = 0.0f32;
        for i in 0..n - lag {
            let a = x[i] - mean;
            let b = x[i + lag] - mean;
            xy += a * b;
            xx += a * a;
            yy += b * b;
        }
        let denom = (xx * yy).sqrt();
        if denom > 0.0 {
            let r = xy / denom;
            if r > best {
                best = r;
            }
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: f64 = 48_000.0;

    /// A voiced syllable stream: a 120 Hz glottal pulse train (harmonics with a
    /// 1/k rolloff and two formant peaks), gated into syllables at 4 Hz with a
    /// pause between phrases, and a little noise on top as a real channel has.
    fn speech(secs: f64) -> Vec<f32> {
        let n = (secs * RATE) as usize;
        let mut out = Vec::with_capacity(n);
        let mut rng = Rng::new(7);
        for i in 0..n {
            let t = i as f64 / RATE;
            // Syllables: 4 Hz, with 0.3 s of quiet between phrases.
            let syll = 0.5 - 0.5 * (std::f64::consts::TAU * 4.0 * t).cos();
            let gate = if (t % 1.3) < 1.0 { 1.0 } else { 0.0 };
            let mut s = 0.0f64;
            for k in 1..=30 {
                let f = 120.0 * k as f64;
                let tilt = 1.0 / k as f64;
                // Two formants, so the spectrum is speech-shaped rather than a
                // plain sawtooth.
                let formant = 1.0 + 2.5 * gauss(f, 700.0, 130.0) + 1.8 * gauss(f, 1900.0, 200.0);
                s += tilt * formant * (std::f64::consts::TAU * f * t).sin();
            }
            let noise = rng.next() as f64 * 0.02;
            out.push((0.12 * s * syll * gate + noise) as f32);
        }
        normalise(&out)
    }

    fn gauss(x: f64, mu: f64, sigma: f64) -> f64 {
        (-((x - mu).powi(2)) / (2.0 * sigma * sigma)).exp()
    }

    fn tone(hz: f64, secs: f64, amp: f32) -> Vec<f32> {
        let n = (secs * RATE) as usize;
        (0..n)
            .map(|i| {
                let t = i as f64 / RATE;
                (amp as f64 * (std::f64::consts::TAU * hz * t).sin()) as f32
            })
            .collect()
    }

    /// Band-limited noise: what a data channel and an empty FM channel both
    /// sound like.
    fn noise(secs: f64) -> Vec<f32> {
        let n = (secs * RATE) as usize;
        let mut rng = Rng::new(11);
        let mut lp = 0.0f32;
        let mut out = Vec::with_capacity(n);
        for _ in 0..n {
            // A one-pole low-pass at roughly 2.5 kHz, then a DC block.
            lp += 0.28 * (rng.next() - lp);
            out.push(lp * 6.0);
        }
        normalise(&out)
    }

    /// Two-tone AFSK at 1200 baud: the data burst that a level-only scan stops
    /// on and a listener does not want.
    fn afsk(secs: f64) -> Vec<f32> {
        let n = (secs * RATE) as usize;
        let mut phase = 0.0f64;
        let mut out = Vec::with_capacity(n);
        for i in 0..n {
            let sym = (i as f64 / RATE * 1200.0) as usize;
            let f = if sym % 2 == 0 { 1200.0 } else { 2200.0 };
            phase += std::f64::consts::TAU * f / RATE;
            out.push((0.5 * phase.sin()) as f32);
        }
        normalise(&out)
    }

    fn normalise(x: &[f32]) -> Vec<f32> {
        let peak = x.iter().fold(0.0f32, |m, s| m.max(s.abs()));
        if peak <= 0.0 {
            return x.to_vec();
        }
        x.iter().map(|s| s * 0.5 / peak).collect()
    }

    fn detect(signal: &[f32]) -> SpeechDetector {
        let mut d = SpeechDetector::new(RATE);
        d.process(signal);
        d
    }

    /// A tiny deterministic generator: the tests must not depend on a random
    /// crate, and a fixed sequence makes a failure reproducible.
    struct Rng(u64);
    impl Rng {
        fn new(seed: u64) -> Self {
            Rng(seed.wrapping_mul(6364136223846793005).wrapping_add(1))
        }
        fn next(&mut self) -> f32 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            ((self.0 >> 33) as f32 / (1u64 << 31) as f32) * 2.0 - 1.0
        }
    }

    #[test]
    fn speech_is_voice() {
        let d = detect(&speech(2.0));
        assert!(
            d.speaking(),
            "a voiced syllable stream scored {} and was not called speech",
            d.score()
        );
        // The number is the reading for the *last* frame, and a conversation
        // that ends on a pause reads low at the moment it is cut — the verdict
        // is the count, not this. What it must not be is a tone's reading.
        assert!(d.score() > RELEASE, "score {}", d.score());
    }

    /// The whole point of the feature: a carrier is a carrier, however strong,
    /// and a scan must not stop on it.
    #[test]
    fn a_steady_tone_is_not_voice() {
        let d = detect(&tone(1_000.0, 2.0, 0.5));
        assert!(!d.speaking(), "a 1 kHz tone scored {} and was called speech", d.score());
        assert!(d.score() <= 25.0, "score {}", d.score());
    }

    #[test]
    fn a_ctcss_style_sub_audible_tone_is_not_voice() {
        // 88.5 Hz is a real CTCSS tone and a plank of many repeater tails.
        let d = detect(&tone(88.5, 2.0, 0.5));
        assert!(!d.speaking(), "score {}", d.score());
    }

    #[test]
    fn noise_is_not_voice() {
        let d = detect(&noise(2.0));
        assert!(!d.speaking(), "noise scored {} and was called speech", d.score());
    }

    #[test]
    fn a_data_burst_is_not_voice() {
        let d = detect(&afsk(2.0));
        assert!(!d.speaking(), "1200 baud AFSK scored {} and was called speech", d.score());
    }

    /// The measured fault that the four-of-half-a-second rule and the accept
    /// threshold above it exist for: a receiver settling after a retune produces
    /// one frame of a periodic, band-filling thump, which scores *exactly* what
    /// structure, voicing and spread can add up to with no rhythm at all. A
    /// scan that took one frame for a voice would stop on every retune.
    #[test]
    fn one_settling_thump_is_not_a_voice() {
        // Twenty milliseconds — one frame — of a 120 Hz pulse train, which is
        // what a DDC's settling looks like to this: periodic, structured and
        // filling the band. Then a silent channel.
        let mut signal: Vec<f32> = (0..(RATE * FRAME_S) as usize)
            .map(|i| {
                let t = i as f64 / RATE;
                let s: f64 = (1..=30)
                    .map(|k| (std::f64::consts::TAU * 120.0 * k as f64 * t).sin() / k as f64)
                    .sum();
                (0.15 * s) as f32
            })
            .collect();
        signal.extend(std::iter::repeat_n(0.0, (RATE * 0.5) as usize));
        let d = detect(&signal);
        assert!(!d.speaking(), "a single settling frame was called a voice");
    }

    /// And a signal that never stops being one steady buzz — a 400 Hz-ish
    /// harmonic pile with no rhythm to it — is not a conversation either,
    /// however much it fills the band.
    #[test]
    fn a_steady_buzz_is_not_a_voice() {
        let signal: Vec<f32> = (0..(RATE * 2.0) as usize)
            .map(|i| {
                let t = i as f64 / RATE;
                let s: f64 = (1..=30)
                    .map(|k| (std::f64::consts::TAU * 120.0 * k as f64 * t).sin() / k as f64)
                    .sum();
                (0.15 * s) as f32
            })
            .collect();
        let d = detect(&signal);
        assert!(!d.speaking(), "a steady pulse train scored {} and was called speech", d.score());
    }

    #[test]
    fn silence_scores_zero() {
        let d = detect(&vec![0.0; (RATE * 1.0) as usize]);
        assert_eq!(d.score(), 0.0);
        assert!(!d.speaking());
        assert!(!SpeechDetector::new(RATE).speaking(), "a fresh detector has heard nothing");
    }

    /// The pause between sentences is longer than a syllable and shorter than a
    /// QSO's other station. The number falls; the verdict waits.
    #[test]
    fn a_pause_does_not_end_the_call_at_once() {
        let mut signal = speech(1.5);
        signal.extend(std::iter::repeat_n(0.0, (RATE * 0.3) as usize));
        assert!(detect(&signal).speaking(), "the verdict did not survive a 300 ms pause");

        // Half a second of quiet is a different matter: the call is over.
        let mut longer = speech(1.5);
        longer.extend(std::iter::repeat_n(0.0, (RATE * 0.6) as usize));
        assert!(!detect(&longer).speaking(), "the verdict outlived the conversation");
    }

    /// A new channel is a new question: the rhythm of the last one must not be
    /// carried across a retune.
    #[test]
    fn reset_forgets_everything() {
        let mut d = SpeechDetector::new(RATE);
        d.process(&speech(1.5));
        assert!(d.speaking());
        d.reset();
        assert!(!d.speaking());
        assert_eq!(d.score(), 0.0);
        // And a tone straight after a reset is a tone, not the tail of a voice.
        d.process(&tone(1_000.0, 1.0, 0.5));
        assert!(!d.speaking(), "score {} after reset", d.score());
    }

    /// A detector handed a block at a time must reach the same verdict as one
    /// handed the lot: the engine's blocks are whatever the sound card gives.
    #[test]
    fn block_size_does_not_change_the_answer() {
        let signal = speech(2.0);
        let whole = detect(&signal);
        let mut piecemeal = SpeechDetector::new(RATE);
        for chunk in signal.chunks(1_024) {
            piecemeal.process(chunk);
        }
        assert_eq!(whole.speaking(), piecemeal.speaking());
        assert!((whole.score() - piecemeal.score()).abs() < 1.0, "{}", piecemeal.score());
    }

    #[test]
    fn the_score_is_bounded_and_finite() {
        for signal in [speech(1.0), noise(1.0), tone(440.0, 1.0, 1.0)] {
            let s = detect(&signal).score();
            assert!(s.is_finite() && (0.0..=100.0).contains(&s), "{s}");
        }
    }

    #[test]
    fn flatness_of_a_tone_is_far_below_that_of_noise() {
        let mut d = SpeechDetector::new(RATE);
        d.process(&tone(1_000.0, 0.1, 0.5));
        let t = flatness(&d.spec, d.band_lo_bin, d.band_hi_bin);
        d.process(&noise(0.1));
        let n = flatness(&d.spec, d.band_lo_bin, d.band_hi_bin);
        assert!(t < 0.01, "tone flatness {t}");
        assert!(n > 0.3, "noise flatness {n}");
    }

    #[test]
    fn periodicity_of_a_periodic_signal_is_high_and_of_noise_is_low() {
        let x: Vec<f32> =
            (0..240).map(|i| (std::f32::consts::TAU * 120.0 * i as f32 / 12_000.0).sin()).collect();
        assert!(periodicity(&x, 30, 200) > 0.9);
        let mut rng = Rng::new(3);
        let y: Vec<f32> = (0..240).map(|_| rng.next()).collect();
        assert!(periodicity(&y, 30, 200) < 0.4, "{}", periodicity(&y, 30, 200));
    }

    #[test]
    fn the_syllable_rate_of_a_tone_is_zero_and_of_a_gate_is_not() {
        let steady = detect(&tone(1_000.0, 1.0, 0.5));
        assert_eq!(steady.syllable_rate(), 0.0);

        // Four rises a second, which is a syllable rate.
        let mut gated = SpeechDetector::new(RATE);
        let mut sig = Vec::new();
        for i in 0..(RATE * 1.0) as usize {
            let t = i as f64 / RATE;
            let env = 0.5 - 0.5 * (std::f64::consts::TAU * 4.0 * t).cos();
            sig.push((env * (std::f64::consts::TAU * 300.0 * t).sin()) as f32);
        }
        gated.process(&sig);
        let r = gated.syllable_rate();
        assert!((3.0..=5.0).contains(&r), "syllable rate {r}");
    }
}
