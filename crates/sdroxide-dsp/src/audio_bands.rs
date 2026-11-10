//! A bar graph of the speaker feed: [`AUDIO_BANDS`] log-spaced bands, one byte
//! each, from the last 1024 samples.
//!
//! For the Radio Classic panel's audio bars. Not a measurement instrument: a
//! Hann window, one real FFT, the loudest bin inside each band, and a floor at
//! -80 dBFS. A full-scale sine reads 255 in its own band.

use std::sync::Arc;
use std::time::{Duration, Instant};

use realfft::num_complex::Complex32;
use realfft::{RealFftPlanner, RealToComplex};
use sdroxide_types::{
    AUDIO_BAND_FLOOR_DB, AUDIO_BANDS, AUDIO_BANDS_HI_HZ, AUDIO_BANDS_LO_HZ, AudioLevel,
    band_to_byte,
};

/// Samples per transform.
const N: usize = 1024;

pub struct AudioBands {
    fft: Arc<dyn RealToComplex<f32>>,
    window: Vec<f32>,
    /// Sum of the window, to scale a bin to a sine's amplitude.
    window_sum: f32,
    /// The newest [`N`] samples, oldest first.
    ring: Vec<f32>,
    input: Vec<f32>,
    spectrum: Vec<Complex32>,
    scratch: Vec<Complex32>,
    /// `(first_bin, last_bin)` per band, for the rate they were built for.
    edges: Vec<(usize, usize)>,
    rate: f32,
}

impl AudioBands {
    pub fn new() -> Self {
        let fft = RealFftPlanner::<f32>::new().plan_fft_forward(N);
        let window: Vec<f32> = (0..N)
            .map(|i| 0.5 - 0.5 * (2.0 * std::f32::consts::PI * i as f32 / N as f32).cos())
            .collect();
        let window_sum = window.iter().sum();
        Self {
            spectrum: fft.make_output_vec(),
            scratch: fft.make_scratch_vec(),
            fft,
            window,
            window_sum,
            ring: vec![0.0; N],
            input: vec![0.0; N],
            edges: Vec::new(),
            rate: 0.0,
        }
    }

    /// Append a block of samples, keeping only the newest [`N`].
    pub fn push(&mut self, block: &[f32]) {
        if block.len() >= N {
            self.ring.copy_from_slice(&block[block.len() - N..]);
        } else if !block.is_empty() {
            self.ring.rotate_left(block.len());
            let at = N - block.len();
            self.ring[at..].copy_from_slice(block);
        }
    }

    /// The bands as of the samples pushed so far, at `rate_hz`.
    pub fn bands(&mut self, rate_hz: f32) -> Vec<u8> {
        if !rate_hz.is_finite() || rate_hz < 8_000.0 {
            return vec![0; AUDIO_BANDS];
        }
        if (rate_hz - self.rate).abs() > 0.5 {
            self.edges = band_edges(rate_hz);
            self.rate = rate_hz;
        }
        for (o, (s, w)) in self.input.iter_mut().zip(self.ring.iter().zip(&self.window)) {
            *o = s * w;
        }
        if self
            .fft
            .process_with_scratch(&mut self.input, &mut self.spectrum, &mut self.scratch)
            .is_err()
        {
            return vec![0; AUDIO_BANDS];
        }
        let scale = 2.0 / self.window_sum;
        self.edges
            .iter()
            .map(|&(lo, hi)| {
                let peak = self.spectrum[lo..=hi].iter().map(|c| c.norm()).fold(0.0f32, f32::max);
                let db = 20.0 * (peak * scale).max(1e-9).log10();
                band_to_byte(db.max(AUDIO_BAND_FLOOR_DB))
            })
            .collect()
    }
}

impl Default for AudioBands {
    fn default() -> Self {
        Self::new()
    }
}

/// How often the speaker feed's level goes out: about thirty times a second,
/// which is as fast as a needle needs and a bar can be seen to move.
pub const AUDIO_LEVEL_INTERVAL: Duration = Duration::from_millis(33);

/// Measures what is handed to the speakers and says so at
/// [`AUDIO_LEVEL_INTERVAL`]: the level and the bars for a client that has no
/// other way to see them (`RadioEvent::AudioLevel`).
///
/// Mean power over the interval rather than one block's, so the needle is not
/// ten times a second out of step with the beat. Silence is reported once and
/// then not at all, so an idle station does not stream zeros.
pub struct AudioMeter {
    bands: AudioBands,
    sum_sq: [f64; 2],
    count: [u64; 2],
    next: Option<Instant>,
    last_silent: bool,
}

impl AudioMeter {
    pub fn new() -> Self {
        Self {
            bands: AudioBands::new(),
            sum_sq: [0.0; 2],
            count: [0; 2],
            next: None,
            last_silent: false,
        }
    }

    /// Take one block of the feed; returns a reading when one is due.
    pub fn feed(
        &mut self,
        left: &[f32],
        right: Option<&[f32]>,
        rate_hz: f64,
        now: Instant,
    ) -> Option<AudioLevel> {
        for (i, ch) in [Some(left), right].into_iter().enumerate() {
            // A mono feed repeats the left ear, as a meter on a mono set would.
            let ch = ch.filter(|c| !c.is_empty()).unwrap_or(left);
            self.sum_sq[i] += ch.iter().map(|&x| f64::from(x) * f64::from(x)).sum::<f64>();
            self.count[i] += ch.len() as u64;
        }
        self.bands.push(left);
        if self.next.is_some_and(|t| now < t) {
            return None;
        }
        self.next = Some(now + AUDIO_LEVEL_INTERVAL);
        let db = |i: usize| {
            if self.count[i] == 0 {
                return -120.0;
            }
            let ms = (self.sum_sq[i] / self.count[i] as f64) as f32;
            (10.0 * ms.max(1e-12).log10()).max(-120.0)
        };
        let rms_db = [db(0), db(1)];
        self.sum_sq = [0.0; 2];
        self.count = [0; 2];
        let silent = rms_db.iter().all(|&d| d < -100.0);
        if silent && self.last_silent {
            return None;
        }
        self.last_silent = silent;
        let bands = if silent { Vec::new() } else { self.bands.bands(rate_hz as f32) };
        Some(AudioLevel { rms_db, bands })
    }
}

impl Default for AudioMeter {
    fn default() -> Self {
        Self::new()
    }
}

/// The bins each band covers. The lowest bands are narrower than a bin, so a
/// band always has at least the bin nearest its centre.
fn band_edges(rate_hz: f32) -> Vec<(usize, usize)> {
    let bin_hz = rate_hz / N as f32;
    let top = (N / 2) as f32;
    let hi_hz = AUDIO_BANDS_HI_HZ.min(rate_hz / 2.0 - bin_hz);
    let ratio = (hi_hz / AUDIO_BANDS_LO_HZ).powf(1.0 / AUDIO_BANDS as f32);
    (0..AUDIO_BANDS)
        .map(|i| {
            let f0 = AUDIO_BANDS_LO_HZ * ratio.powi(i as i32);
            let f1 = f0 * ratio;
            let mid = (f0 * f1).sqrt() / bin_hz;
            let lo = (f0 / bin_hz).ceil().min(top);
            let hi = (f1 / bin_hz).floor().min(top);
            let (lo, hi) = if hi < lo { (mid.round(), mid.round()) } else { (lo, hi) };
            (lo.max(1.0) as usize, hi.max(1.0) as usize)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sine(hz: f32, rate: f32, amp: f32, n: usize) -> Vec<f32> {
        (0..n).map(|i| amp * (2.0 * std::f32::consts::PI * hz * i as f32 / rate).sin()).collect()
    }

    fn loudest(b: &[u8]) -> usize {
        b.iter().enumerate().max_by_key(|(_, v)| **v).map(|(i, _)| i).unwrap()
    }

    #[test]
    fn a_tone_lights_its_own_band() {
        let rate = 48_000.0;
        let mut a = AudioBands::new();
        let (lo, hi) = (AUDIO_BANDS_LO_HZ, AUDIO_BANDS_HI_HZ);
        let ratio = (hi / lo).powf(1.0 / AUDIO_BANDS as f32);
        for target in [8usize, 20, 30, 41] {
            let hz = lo * ratio.powf(target as f32 + 0.5);
            a.push(&sine(hz, rate, 1.0, N));
            let b = a.bands(rate);
            assert_eq!(b.len(), AUDIO_BANDS);
            let at = loudest(&b);
            assert!(at.abs_diff(target) <= 1, "{hz} Hz: wanted band {target}, got {at}");
            assert!(b[at] > 230, "a full-scale tone should read near full: {}", b[at]);
        }
    }

    #[test]
    fn silence_reads_zero_and_a_quiet_tone_reads_lower() {
        let rate = 48_000.0;
        let mut a = AudioBands::new();
        a.push(&vec![0.0; N]);
        assert!(a.bands(rate).iter().all(|&v| v == 0));
        a.push(&sine(1_000.0, rate, 1.0, N));
        let loud = *a.bands(rate).iter().max().unwrap();
        a.push(&sine(1_000.0, rate, 0.01, N));
        let quiet = *a.bands(rate).iter().max().unwrap();
        assert!(quiet < loud - 100, "-40 dB should be about 128 steps down: {loud} vs {quiet}");
    }

    #[test]
    fn short_blocks_roll_into_the_window() {
        let rate = 48_000.0;
        let mut a = AudioBands::new();
        let tone = sine(2_000.0, rate, 1.0, N);
        for chunk in tone.chunks(480) {
            a.push(chunk);
        }
        assert!(*a.bands(rate).iter().max().unwrap() > 200);
    }

    #[test]
    fn the_meter_reports_at_thirty_hertz_and_goes_quiet_in_silence() {
        let rate = 48_000.0;
        let t0 = Instant::now();
        let mut m = AudioMeter::new();
        let tone = sine(1_000.0, rate as f32, 0.5, 480);
        // The first block reports at once; the next 10 ms ones wait their turn.
        let first = m.feed(&tone, Some(&tone), rate, t0).expect("first reading");
        assert_eq!(first.bands.len(), AUDIO_BANDS);
        // A 0.5-amplitude sine is -9 dBFS RMS.
        assert!((first.rms_db[0] + 9.0).abs() < 0.3, "{:?}", first.rms_db);
        assert!(m.feed(&tone, Some(&tone), rate, t0 + Duration::from_millis(10)).is_none());
        assert!(m.feed(&tone, Some(&tone), rate, t0 + Duration::from_millis(20)).is_none());
        let next = m.feed(&tone, Some(&tone), rate, t0 + Duration::from_millis(34));
        assert!(next.is_some(), "a reading is due after 33 ms");
        // Silence: said once, then nothing.
        let zeros = vec![0.0f32; 480];
        let quiet = m.feed(&zeros, None, rate, t0 + Duration::from_millis(80)).unwrap();
        assert!(quiet.bands.is_empty() || quiet.rms_db[0] > -100.0);
        let again = m.feed(&zeros, None, rate, t0 + Duration::from_millis(160));
        let again2 = m.feed(&zeros, None, rate, t0 + Duration::from_millis(240));
        assert!(again.is_none() || again2.is_none(), "silence must not stream");
    }

    #[test]
    fn a_mono_feed_reads_the_same_on_both_sides_and_a_louder_right_shows() {
        let rate = 48_000.0;
        let t0 = Instant::now();
        let quiet = sine(1_000.0, rate as f32, 0.1, 480);
        let loud = sine(1_000.0, rate as f32, 0.8, 480);
        let mut m = AudioMeter::new();
        let mono = m.feed(&loud, None, rate, t0).unwrap();
        assert_eq!(mono.rms_db[0], mono.rms_db[1]);
        let mut m = AudioMeter::new();
        let st = m.feed(&quiet, Some(&loud), rate, t0).unwrap();
        assert!(st.rms_db[1] > st.rms_db[0] + 15.0, "{:?}", st.rms_db);
    }

    #[test]
    fn a_nonsense_rate_is_all_zero_not_a_panic() {
        let mut a = AudioBands::new();
        assert!(a.bands(0.0).iter().all(|&v| v == 0));
        assert!(a.bands(f32::NAN).iter().all(|&v| v == 0));
    }
}
