//! Spectral-bleach noise reduction — a Rust port of the adaptive denoiser from
//! [libspecbleach], the engine behind the *noise-repellent* plugins.
//!
//! Ported from libspecbleach **v0.3.0** (Luciano Dato, LGPL-2.1-or-later). Only
//! the 1-D adaptive path is here: the 2-D denoiser, the non-local-means filter,
//! the Martin and Brandt estimators, the manual noise-profile capture API and
//! the tonal-peak queries are all left behind. What remains is the chain that
//! makes this engine sound unlike [`crate::SpectralNr`]:
//!
//!  1. **SPP-MMSE noise estimation** — a speech-presence-probability tracker
//!     that updates the noise PSD in proportion to how likely each bin is to be
//!     noise, rather than tracking a minimum.
//!  2. **Berouti per-bin over-subtraction** — the subtraction factor `alpha`
//!     slides with each bin's SNR, so quiet bins are cut hard and loud ones are
//!     left alone.
//!  3. **A psychoacoustic veto** — Johnston's masking model over Bark bands
//!     (level-dependent Schroeder spreading, SFM tonality, forward temporal
//!     masking, absolute hearing threshold). Where the noise already sits below
//!     the masking threshold it is inaudible, so `alpha` is pulled back toward
//!     1: energy that costs nothing to keep is kept.
//!  4. **A whitened noise floor** — the signature of this engine. Rather than
//!     letting suppression carve the residue into birdies, the gain is floored
//!     at a level whose *shape* is the inverse of the noise profile, so what is
//!     left reads as flat hiss.
//!
//! Two deliberate departures from the reference:
//!
//!  * It runs at the audio rate rather than resampling, so unlike
//!    [`crate::NeuralNr`] and [`crate::DeepFilterNr`] there is no
//!    [`crate::frame48::Frame48`] around it. The frame is sized in
//!    milliseconds, as upstream does.
//!  * Upstream maps a critical band's edge frequency to a bin with
//!    `freq * real_spectrum_size / sample_rate`, which is half the bin that
//!    frequency actually falls in — the Bark bands land at half their intended
//!    frequencies and the last one swallows the top half of the spectrum. This
//!    port uses `freq * fft_size / sample_rate`, so the psychoacoustic model
//!    sits where the model says it should.
//!
//! Backward (pre-)masking is not implemented: it needs the *next* frame, and
//! buying it would cost another frame of delay on a live receiver.
//!
//! In place, same length in and out, with a fixed latency of
//! [`SpecBleachNr::latency_samples`] — one frame less one hop.
//!
//! [libspecbleach]: https://github.com/lucianodato/libspecbleach

use std::collections::VecDeque;
use std::sync::Arc;

use realfft::num_complex::Complex32;
use realfft::{ComplexToReal, RealFftPlanner, RealToComplex};

// ---------------------------------------------------------------- constants

/// Numerical stability floor (`SPECTRAL_EPSILON`).
const EPS: f32 = 1e-12;

/// STFT frame length in milliseconds. Upstream recommends 20–100; 20 is the
/// shortest it suggests, and on a radio the latency is worth more than the
/// extra frequency resolution.
const FRAME_MS: f32 = 20.0;
/// 75 % overlap (`OVERLAP_FACTOR_1D`).
const OVERLAP: usize = 4;
/// Fixed zero-padding, in samples (`ZEROPADDING_AMOUNT_1D`).
const ZEROPAD: usize = 800;

// Berouti over-subtraction.
const ALPHA_MIN: f32 = 1.0;
const ALPHA_MAX: f32 = 4.0;
const SUPPRESSION_LOWER_SNR_DB: f32 = -5.0;
const SUPPRESSION_HIGHER_SNR_DB: f32 = 20.0;

// SPP-MMSE noise estimation.
const ESTIMATOR_SILENCE_THRESHOLD: f32 = 1e-10;
const NOISE_ESTIMATION_INTERPOLATION_THRESHOLD: f32 = 1e-9;
const ADAPTIVE_NOISE_FLOOR_SMOOTHING: f32 = 0.5;
/// Fixed a-priori SNR, 15 dB in linear power (`SPP_FIXED_XI_H1`).
const SPP_FIXED_XI_H1: f32 = 31.62;
const SPP_ALPHA_POW: f32 = 0.8;
const SPP_SMOOTH_SPP: f32 = 0.9;
const SPP_CURRENT_SPP: f32 = 0.1;
/// Ceiling on the smoothed SPP, so a bin cannot lock itself out of updating.
const SPP_STAGNATION_CAP: f32 = 0.99;

// Masking veto.
const MASKING_VETO_SMOOTHING: f32 = 0.5;
const MASKING_VETO_NMR_RANGE: f32 = 20.0;
const MASKING_VETO_SNR_THRESHOLD: f32 = 1.0;

// Johnston's psychoacoustic model (1988).
const DB_FS_TO_SPL_REF: f32 = 96.0;
const POWER_LAW_EXPONENT: f32 = 0.6;
/// PEAQ additivity exponent (`SPECTRAL_ADDITIVITY_EXPONENT_PEAQ`).
const ADDITIVITY_EXPONENT: f32 = 0.4;
// Schroeder spreading, dB per Bark.
const S_MIN_UPWARD: f32 = 5.0;
const S_MAX_UPWARD: f32 = 15.0;
const S_DOWNWARD: f32 = 25.0;
const S_LEVEL_REF_DB: f32 = 40.0;
const S_SLOPE_FACTOR: f32 = 0.2;
/// Noise-masking-tone offset.
const NMT_OFFSET_DB: f32 = 6.0;
/// Tone-masking-noise offset, before the Bark-dependent term.
const TMN_OFFSET_BASE: f32 = 14.5;
const SFM_MIN_DB: f32 = -60.0;
const SFM_MAX_DB: f32 = 0.0;
// Temporal masking time constants, in seconds.
const FORWARD_TAU_LOW: f32 = 0.100;
const FORWARD_TAU_HIGH: f32 = 0.025;

/// Upper edge of each Bark critical band, in Hz.
const BARK_BANDS: [f32; 25] = [
    100.0, 200.0, 300.0, 400.0, 510.0, 630.0, 770.0, 920.0, 1080.0, 1270.0, 1480.0, 1720.0, 2000.0,
    2320.0, 2700.0, 3150.0, 3700.0, 4400.0, 5300.0, 6400.0, 7700.0, 9500.0, 12000.0, 15500.0,
    19000.0,
];

// ------------------------------------------------------------ critical bands

/// Bark-band layout over a real spectrum: which bins belong to which band.
struct CriticalBands {
    /// `[start, end)` bin range per band.
    ranges: Vec<(usize, usize)>,
}

impl CriticalBands {
    fn new(rate: f64, fft_size: usize, bins: usize) -> Self {
        let nyquist = rate as f32 / 2.0;
        // The last band whose edge is still below Nyquist, plus one — the final
        // band always runs to the top of the spectrum.
        let count = BARK_BANDS.iter().filter(|f| **f < nyquist).count().max(1);

        let mut ranges = Vec::with_capacity(count);
        let mut prev_end = 0usize;
        for (j, &edge) in BARK_BANDS.iter().take(count).enumerate() {
            // The last band always runs to the top of the spectrum, and any
            // band that would start past the end is simply not there — a short
            // FFT can run out of bins before the Bark scale runs out of bands.
            if prev_end >= bins {
                break;
            }
            let end = if j == count - 1 {
                bins
            } else {
                // bin = round(freq * fft_size / rate) — see the module note on
                // where this departs from the reference.
                let bin = (edge as f64 * fft_size as f64 / rate + 0.5) as usize;
                bin.max(prev_end + 1).min(bins)
            };
            ranges.push((prev_end, end));
            prev_end = end;
        }
        if ranges.is_empty() {
            ranges.push((0, bins));
        } else {
            // Whatever is left over belongs to the top band.
            ranges.last_mut().expect("non-empty").1 = bins;
        }
        CriticalBands { ranges }
    }

    fn len(&self) -> usize {
        self.ranges.len()
    }

    /// Sum `spectrum` within each band.
    fn fold(&self, spectrum: &[f32], out: &mut [f32]) {
        for (o, &(s, e)) in out.iter_mut().zip(&self.ranges) {
            *o = spectrum[s..e].iter().sum();
        }
    }
}

// --------------------------------------------------------- masking estimator

/// Johnston's masking model: how loud a noise may be in each Bark band before
/// the ear can pick it out of the signal that is already there.
struct MaskingEstimator {
    bands: CriticalBands,
    /// Absolute threshold of hearing per bin, as a linear power.
    absolute: Vec<f32>,
    /// Forward-masking decay per band, per hop.
    forward_decay: Vec<f32>,
    /// Last frame's threshold per band — the forward-masking state.
    prev_threshold: Vec<f32>,

    // Scratch.
    band_power: Vec<f32>,
    band_level_db: Vec<f32>,
    spread: Vec<f32>,
    /// `spreading[i * bands + j]`: how much band `j` masks band `i`.
    spreading: Vec<f32>,
}

impl MaskingEstimator {
    fn new(rate: f64, fft_size: usize, bins: usize, hop: usize) -> Self {
        let bands = CriticalBands::new(rate, fft_size, bins);
        let n = bands.len();

        // Absolute threshold of hearing (Terhardt), in dB SPL, converted to a
        // linear power against the same 0 dBFS = 96 dB SPL reference the rest
        // of the model uses. The reference resynthesises a calibration sine to
        // find that anchor; pinning it to DB_FS_TO_SPL_REF is the same
        // normalisation without the FFT.
        let absolute = (0..bins)
            .map(|k| {
                let f = (k as f64 * rate / fft_size as f64).max(20.0) as f32 / 1000.0;
                let db =
                    3.64 * f.powf(-0.8) - 6.5 * (-0.6 * (f - 3.3).powi(2)).exp() + 1e-3 * f.powi(4);
                10f32.powf((db - DB_FS_TO_SPL_REF) / 10.0)
            })
            .collect();

        // Forward masking decays faster at high frequencies: 100 ms at the
        // bottom of the Bark scale, 25 ms at the top.
        let hop_time = hop as f32 / rate as f32;
        let forward_decay = (0..n)
            .map(|j| {
                let w = (j as f32).min(24.0) / 24.0;
                let tau = (1.0 - w) * FORWARD_TAU_LOW + w * FORWARD_TAU_HIGH;
                (-hop_time / tau).exp()
            })
            .collect();

        MaskingEstimator {
            bands,
            absolute,
            forward_decay,
            prev_threshold: vec![0.0; n],
            band_power: vec![0.0; n],
            band_level_db: vec![0.0; n],
            spread: vec![0.0; n],
            spreading: vec![0.0; n * n],
        }
    }

    fn reset(&mut self) {
        self.prev_threshold.fill(0.0);
    }

    /// Masking threshold per bin for `spectrum` (a power spectrum).
    fn compute(&mut self, spectrum: &[f32], out: &mut [f32]) {
        let n = self.bands.len();
        self.bands.fold(spectrum, &mut self.band_power);

        for (db, &p) in self.band_level_db.iter_mut().zip(&self.band_power) {
            *db = 10.0 * (p + EPS).log10() + DB_FS_TO_SPL_REF;
        }

        // Level-dependent Schroeder spreading, summed with the PEAQ additivity
        // exponent rather than linearly.
        for i in 0..n {
            let mut acc = 0.0f32;
            for j in 0..n {
                let dz = i as f32 - j as f32;
                let gain = spreading_gain(dz, self.band_level_db[j]);
                self.spreading[i * n + j] = gain;
                acc += (self.band_power[j] * gain).powf(ADDITIVITY_EXPONENT);
            }
            self.spread[i] = acc.powf(1.0 / ADDITIVITY_EXPONENT);
        }

        for j in 0..n {
            // A tonal masker hides less noise than a noisy one, so the offset
            // below the spread energy runs from ~6 dB (noise masking a tone) to
            // ~15-40 dB (a tone masking noise), by spectral flatness.
            let tonality = tonality_factor(spectrum, self.bands.ranges[j]);
            let bark = ((j + 1) as f32).min(25.0);
            let offset = tonality * (TMN_OFFSET_BASE + bark) + NMT_OFFSET_DB * (1.0 - tonality);
            let mut threshold = 10f32.powf((self.spread[j] + EPS).log10() - offset / 10.0);

            // Forward masking, summed with the others under Johnston's power
            // law rather than added — multiple maskers do not simply stack.
            let mut t_p = threshold.powf(POWER_LAW_EXPONENT);
            let forward = self.prev_threshold[j] * self.forward_decay[j];
            t_p += forward.powf(POWER_LAW_EXPONENT);
            threshold = t_p.powf(1.0 / POWER_LAW_EXPONENT);

            self.prev_threshold[j] = threshold;
            let (s, e) = self.bands.ranges[j];
            out[s..e].fill(threshold);
        }

        // Nothing can be masked below what the ear cannot hear at all.
        for (o, &a) in out.iter_mut().zip(&self.absolute) {
            *o = o.max(a);
        }
    }
}

/// Modified Schroeder spreading: the upward skirt broadens as the masker gets
/// louder, the downward one does not.
fn spreading_gain(dz: f32, level_db: f32) -> f32 {
    let s_up = (S_MAX_UPWARD - (level_db - S_LEVEL_REF_DB) * S_SLOPE_FACTOR)
        .clamp(S_MIN_UPWARD, S_MAX_UPWARD);
    let s_total = (S_DOWNWARD + s_up) / 2.0;
    let s_offset = (S_DOWNWARD - s_up) / 2.0;
    let y = dz + 0.474;
    let sf_db = 15.81 + s_offset * y - s_total * (1.0 + y * y).sqrt();
    10f32.powf(sf_db / 10.0)
}

/// Spectral flatness of one band, mapped to `[0, 1]`: 1 is a pure tone, 0 is
/// noise.
fn tonality_factor(spectrum: &[f32], (start, end): (usize, usize)) -> f32 {
    let n = end.saturating_sub(start);
    if n <= 1 {
        return 1.0;
    }
    let mut sum = 0.0f32;
    let mut sum_log = 0.0f32;
    for &v in &spectrum[start..end] {
        let v = v.max(EPS);
        sum += v;
        sum_log += v.log10();
    }
    let n = n as f32;
    let sfm = 10.0 * (sum_log / n - (sum / n).log10());
    ((sfm - SFM_MAX_DB) / (SFM_MIN_DB - SFM_MAX_DB)).clamp(0.0, 1.0)
}

// -------------------------------------------------------------- masking veto

/// Pulls `alpha` back toward 1 wherever the masking model says the noise is
/// already inaudible — energy that costs nothing to keep.
struct MaskingVeto {
    masking: MaskingEstimator,
    /// Bin centre of each band, for interpolating protection between them.
    centres: Vec<f32>,
    clean: Vec<f32>,
    stable_clean: Vec<f32>,
    thresholds: Vec<f32>,
    protection: Vec<f32>,
    protection_memory: Vec<f32>,
}

impl MaskingVeto {
    /// A placeholder with no bands, for the moment before the rate is known.
    /// [`SpecBleachNr::rebuild`] replaces it before any audio arrives.
    fn empty() -> Self {
        MaskingVeto {
            masking: MaskingEstimator {
                bands: CriticalBands { ranges: Vec::new() },
                absolute: Vec::new(),
                forward_decay: Vec::new(),
                prev_threshold: Vec::new(),
                band_power: Vec::new(),
                band_level_db: Vec::new(),
                spread: Vec::new(),
                spreading: Vec::new(),
            },
            centres: Vec::new(),
            clean: Vec::new(),
            stable_clean: Vec::new(),
            thresholds: Vec::new(),
            protection: Vec::new(),
            protection_memory: Vec::new(),
        }
    }

    fn new(rate: f64, fft_size: usize, bins: usize, hop: usize) -> Self {
        let masking = MaskingEstimator::new(rate, fft_size, bins, hop);
        let n = masking.bands.len();
        let centres =
            masking.bands.ranges.iter().map(|&(s, e)| s as f32 + (e - s) as f32 / 2.0).collect();
        MaskingVeto {
            masking,
            centres,
            clean: vec![0.0; bins],
            stable_clean: vec![0.0; bins],
            thresholds: vec![0.0; bins],
            protection: vec![0.0; n],
            protection_memory: vec![0.0; n],
        }
    }

    fn reset(&mut self) {
        self.masking.reset();
        self.stable_clean.fill(0.0);
        self.protection_memory.fill(0.0);
    }

    fn apply(&mut self, spectrum: &[f32], noise: &[f32], alpha: &mut [f32], depth: f32) {
        if depth <= 0.0 {
            return;
        }
        // A clean-signal estimate, smoothed so the spreading skirts do not
        // gargle from frame to frame.
        for k in 0..spectrum.len() {
            let current = (spectrum[k] - noise[k]).max(0.0);
            self.stable_clean[k] = MASKING_VETO_SMOOTHING * current
                + (1.0 - MASKING_VETO_SMOOTHING) * self.stable_clean[k];
            self.clean[k] = self.stable_clean[k];
        }

        self.masking.compute(&self.clean, &mut self.thresholds);

        // Noise-to-mask ratio per band: at or below 0 dB the noise is buried
        // and gets full protection; by +20 dB it is plainly audible and gets
        // none.
        let n = self.masking.bands.len();
        for j in 0..n {
            let (s, e) = self.masking.bands.ranges[j];
            if e <= s {
                self.protection[j] = 0.0;
                continue;
            }
            let band_noise: f32 = noise[s..e].iter().sum();
            let band_threshold: f32 = self.thresholds[s..e].iter().sum();
            let nmr_db = 10.0 * ((band_noise + EPS) / (band_threshold + EPS)).log10();
            self.protection[j] = if nmr_db <= 0.0 {
                1.0
            } else if nmr_db >= MASKING_VETO_NMR_RANGE {
                0.0
            } else {
                1.0 - nmr_db / MASKING_VETO_NMR_RANGE
            };
        }

        // Stabilise in time, then across bands, so protection cannot flip on
        // and off or draw a hard edge at a band boundary.
        for (p, m) in self.protection.iter_mut().zip(&mut self.protection_memory) {
            *m = MASKING_VETO_SMOOTHING * *p + (1.0 - MASKING_VETO_SMOOTHING) * *m;
            *p = *m;
        }
        smooth_across(&mut self.protection);

        // Interpolate band protection onto bins, weighted down where the bin's
        // own SNR says there is little signal there to protect.
        let mut band = 0usize;
        for k in 0..alpha.len() {
            while band + 1 < n && k as f32 > self.centres[band + 1] {
                band += 1;
            }
            let bin_protection = if band + 1 < n {
                let (x0, x1) = (self.centres[band], self.centres[band + 1]);
                let (y0, y1) = (self.protection[band], self.protection[band + 1]);
                if x1 > x0 {
                    let t = ((k as f32 - x0) / (x1 - x0)).clamp(0.0, 1.0);
                    y0 * (1.0 - t) + y1 * t
                } else {
                    y0
                }
            } else {
                self.protection[n - 1]
            };

            let snr = self.clean[k] / (noise[k] + EPS);
            let snr_scale = (snr / MASKING_VETO_SNR_THRESHOLD).min(1.0);
            let veto = bin_protection * snr_scale * depth;
            alpha[k] += (1.0 - alpha[k]) * veto;
        }
    }
}

// ------------------------------------------------------- SPP-MMSE estimation

/// Speech-presence-probability noise tracker. Each bin's noise PSD is nudged
/// toward the observation in proportion to how likely that bin is to be noise
/// alone, which tracks a rising noise floor without waiting for a minimum.
struct SppMmse {
    prev_psd: Vec<f32>,
    smoothed_spp: Vec<f32>,
    first: bool,
}

impl SppMmse {
    fn new(bins: usize) -> Self {
        SppMmse { prev_psd: vec![0.0; bins], smoothed_spp: vec![0.0; bins], first: true }
    }

    fn reset(&mut self) {
        self.prev_psd.fill(0.0);
        self.smoothed_spp.fill(0.0);
        self.first = true;
    }

    fn run(&mut self, spectrum: &[f32], noise: &mut [f32]) {
        let n = spectrum.len();
        let energy = spectrum.iter().sum::<f32>() / n as f32;

        if self.first {
            if energy < ESTIMATOR_SILENCE_THRESHOLD {
                noise.fill(0.0);
                return;
            }
            self.prev_psd.copy_from_slice(spectrum);
            self.smoothed_spp.fill(0.0);
            noise.copy_from_slice(spectrum);
            self.first = false;
            return;
        }

        if energy < ESTIMATOR_SILENCE_THRESHOLD {
            noise.copy_from_slice(&self.prev_psd);
            return;
        }

        for k in 0..n {
            let prev = self.prev_psd[k].max(EPS);
            // P(speech present | observation) under a fixed a-priori SNR.
            let exponent = -(spectrum[k] / prev) * (SPP_FIXED_XI_H1 / (1.0 + SPP_FIXED_XI_H1));
            let exp_term = exponent.exp();
            let exp_term = if exp_term.is_finite() {
                exp_term
            } else if exponent > 0.0 {
                f32::MAX
            } else {
                0.0
            };
            let mut spp = (1.0 / (1.0 + (1.0 + SPP_FIXED_XI_H1) * exp_term)).clamp(0.0, 1.0);
            // A bin that has been "speech" for a long time would otherwise stop
            // updating for good.
            if self.smoothed_spp[k] > SPP_STAGNATION_CAP {
                spp = spp.min(SPP_STAGNATION_CAP);
            }

            let mmse = (1.0 - spp) * spectrum[k] + spp * self.prev_psd[k];
            noise[k] = SPP_ALPHA_POW * self.prev_psd[k] + (1.0 - SPP_ALPHA_POW) * mmse;
            self.smoothed_spp[k] = SPP_SMOOTH_SPP * self.smoothed_spp[k] + SPP_CURRENT_SPP * spp;
            self.prev_psd[k] = noise[k];
        }
    }
}

// -------------------------------------------------------------------- engine

pub struct SpecBleachNr {
    rate: f64,
    frame: usize,
    hop: usize,
    fft_size: usize,
    bins: usize,
    /// Where the frame sits inside the zero-padded FFT buffer.
    offset: usize,

    fwd: Arc<dyn RealToComplex<f32>>,
    inv: Arc<dyn ComplexToReal<f32>>,
    window: Vec<f32>,
    /// `1 / (sum(w^2) * fft_size / hop)` — undoes the unnormalised FFT and the
    /// windowed overlap-add together, so the chain is unity gain.
    inv_scale: f32,

    /// Gain floor, as a linear amplitude (1 = no reduction).
    floor: f32,
    whitening: f32,
    /// Veto depth. Fixed rather than exposed: the picker has three settings and
    /// this is not one of the two worth spending them on.
    masking_depth: f32,
    /// Berouti strength, 0..1.
    strength: f32,

    spp: SppMmse,
    veto: MaskingVeto,

    // Per-frame scratch.
    time: Vec<f32>,
    spec: Vec<Complex32>,
    power: Vec<f32>,
    smoothed: Vec<f32>,
    prev_smoothed: Vec<f32>,
    noise: Vec<f32>,
    alpha: Vec<f32>,
    gain: Vec<f32>,
    whiten_w: Vec<f32>,
    sort_buf: Vec<f32>,

    /// Input samples awaiting a hop, and the overlap-add accumulator.
    inbuf: Vec<f32>,
    acc: Vec<f32>,
    out: VecDeque<f32>,
}

impl SpecBleachNr {
    pub fn new() -> Self {
        // Default to 48 kHz; `set_rate` rebuilds for whatever the demod hands
        // over. Every rate-dependent field below is replaced by `rebuild`, so
        // they start empty rather than being built twice.
        let mut planner = RealFftPlanner::<f32>::new();
        let mut nr = SpecBleachNr {
            rate: 0.0,
            frame: 0,
            hop: 0,
            fft_size: 0,
            bins: 0,
            offset: 0,
            fwd: planner.plan_fft_forward(2),
            inv: planner.plan_fft_inverse(2),
            window: Vec::new(),
            inv_scale: 1.0,
            floor: 10f32.powf(-12.0 / 20.0),
            whitening: 0.15,
            masking_depth: 0.5,
            strength: 0.5,
            spp: SppMmse::new(0),
            veto: MaskingVeto::empty(),
            time: Vec::new(),
            spec: Vec::new(),
            power: Vec::new(),
            smoothed: Vec::new(),
            prev_smoothed: Vec::new(),
            noise: Vec::new(),
            alpha: Vec::new(),
            gain: Vec::new(),
            whiten_w: Vec::new(),
            sort_buf: Vec::new(),
            inbuf: Vec::new(),
            acc: Vec::new(),
            out: VecDeque::new(),
        };
        nr.rebuild(48_000.0);
        nr
    }

    /// Point the denoiser at a new audio rate, rebuilding the band layout and
    /// clearing state if it changed. Cheap no-op when the rate is unchanged.
    pub fn set_rate(&mut self, rate: f64) {
        if (rate - self.rate).abs() < 0.01 || rate < 1000.0 {
            return;
        }
        self.rebuild(rate);
    }

    /// `reduction_db` is how far below the input the residue is floored (0 is
    /// bypass, 20 is a lot on a radio signal); `whitening` (0..=1) flattens the
    /// shape of what is left so it reads as hiss rather than as birdies.
    pub fn set_params(&mut self, reduction_db: f32, whitening: f32) {
        // The floor is a linear amplitude, as upstream's loader makes it.
        self.floor = 10f32.powf(-reduction_db.clamp(0.0, 40.0) / 20.0);
        self.whitening = whitening.clamp(0.0, 1.0);
        // Push the Berouti factor up with the reduction being asked for: at
        // 20 dB the subtraction has to be doing real work for the floor to have
        // anything to clamp.
        self.strength = (reduction_db / 20.0).clamp(0.0, 1.0);
    }

    /// Clear all streaming state.
    pub fn reset(&mut self) {
        self.spp.reset();
        self.veto.reset();
        self.prev_smoothed.fill(0.0);
        self.inbuf.clear();
        self.acc.fill(0.0);
        self.prime_out();
    }

    /// Seed the output queue with one delay's worth of silence.
    ///
    /// Without it the queue simply runs short on the first call and the
    /// shortfall lands at the *end* of that block — so the first block comes
    /// out undelayed and every later one is delayed, which is a step in the
    /// audio. Priming makes the delay the same from the first sample on.
    fn prime_out(&mut self) {
        self.out.clear();
        for _ in 0..self.frame - self.hop {
            self.out.push_back(0.0);
        }
    }

    /// Delay the denoiser adds, in samples at the current rate.
    ///
    /// A frame must be complete before its first hop can be emitted, so the
    /// output runs `frame - hop` behind the input. (Upstream reports the whole
    /// frame here; this is the delay actually measurable at the output.)
    pub fn latency_samples(&self) -> usize {
        self.frame - self.hop
    }

    fn rebuild(&mut self, rate: f64) {
        self.rate = rate;
        // A whole number of hops, so the overlap-add is exact. Rounded, not
        // truncated: 20 ms at 48 kHz is 960 samples, and `0.02f32 as f64` is a
        // hair under 0.02, which truncates to 959 and then down to 956.
        let frame =
            (((FRAME_MS as f64 / 1000.0 * rate).round() as usize / OVERLAP).max(2)) * OVERLAP;
        let fft_size = (frame + ZEROPAD).next_multiple_of(2);
        let bins = fft_size / 2 + 1;

        self.frame = frame;
        self.hop = frame / OVERLAP;
        self.fft_size = fft_size;
        self.bins = bins;
        self.offset = (fft_size - frame) / 2;

        let mut planner = RealFftPlanner::<f32>::new();
        self.fwd = planner.plan_fft_forward(fft_size);
        self.inv = planner.plan_fft_inverse(fft_size);

        // Periodic Hann, used for both analysis and synthesis.
        self.window = (0..frame)
            .map(|i| {
                let x = std::f32::consts::TAU * i as f32 / frame as f32;
                0.5 - 0.5 * x.cos()
            })
            .collect();
        let sum_sq: f32 = self.window.iter().map(|w| w * w).sum();
        self.inv_scale = 1.0 / (sum_sq * fft_size as f32 / self.hop as f32);

        self.spp = SppMmse::new(bins);
        self.veto = MaskingVeto::new(rate, fft_size, bins, self.hop);

        self.time = vec![0.0; fft_size];
        self.spec = vec![Complex32::default(); bins];
        self.power = vec![0.0; bins];
        self.smoothed = vec![0.0; bins];
        self.prev_smoothed = vec![0.0; bins];
        self.noise = vec![0.0; bins];
        self.alpha = vec![1.0; bins];
        self.gain = vec![1.0; bins];
        self.whiten_w = vec![1.0; bins];
        self.sort_buf = vec![0.0; bins];

        self.inbuf = Vec::with_capacity(frame + self.hop);
        self.acc = vec![0.0; frame];
        self.prime_out();
    }

    /// Denoise `audio` in place. Same length in and out; the first
    /// [`SpecBleachNr::latency_samples`] samples are zero-filled while the
    /// overlap-add primes.
    pub fn process(&mut self, audio: &mut [f32]) {
        self.inbuf.extend_from_slice(audio);

        while self.inbuf.len() >= self.frame {
            self.run_frame();
            // Emit one hop, then slide.
            self.out.extend(self.acc[..self.hop].iter().copied());
            self.acc.copy_within(self.hop.., 0);
            self.acc[self.frame - self.hop..].fill(0.0);
            self.inbuf.drain(..self.hop);
        }

        // One output per input sample. The queue starts `frame - hop` short —
        // a whole frame has to arrive before its first hop can be emitted — so
        // it zero-fills through the priming and runs one delay behind after.
        for s in audio.iter_mut() {
            *s = self.out.pop_front().unwrap_or(0.0);
        }
    }

    fn run_frame(&mut self) {
        // Analysis: window the frame into the middle of the zero-padded buffer.
        self.time.fill(0.0);
        for i in 0..self.frame {
            self.time[self.offset + i] = self.inbuf[i] * self.window[i];
        }
        self.fwd.process(&mut self.time, &mut self.spec).expect("forward fft");

        for (p, c) in self.power.iter_mut().zip(&self.spec) {
            *p = c.re * c.re + c.im * c.im;
        }

        // 1. Track the noise floor, then tidy it: fill the holes a bin-wise
        //    estimator leaves, and smooth across frequency.
        self.spp.run(&self.power, &mut self.noise);
        interpolate_gaps(&mut self.noise, NOISE_ESTIMATION_INTERPOLATION_THRESHOLD);
        smooth_spectrum(&mut self.noise, ADAPTIVE_NOISE_FLOOR_SMOOTHING);

        // 2. Berouti: over-subtract in proportion to how bad each bin's SNR is.
        berouti_per_bin(&self.power, &self.noise, self.strength, &mut self.alpha);

        // 3. Smooth the signal spectrum in time, upward only — so a rising bin
        //    is held back but a decaying one is not smeared.
        for k in 0..self.bins {
            let mut v = self.power[k];
            if v > self.prev_smoothed[k] {
                v = 0.5 * self.prev_smoothed[k] + 0.5 * v;
            }
            self.smoothed[k] = v;
            self.prev_smoothed[k] = v;
        }

        // 4. Give back what the ear could not have heard anyway.
        self.veto.apply(&self.smoothed, &self.noise, &mut self.alpha, self.masking_depth);

        // 5. Wiener gain against the over-subtracted noise.
        for k in 0..self.bins {
            let scaled = self.noise[k] * self.alpha[k];
            self.gain[k] = if scaled > f32::MIN_POSITIVE {
                if self.smoothed[k] > scaled {
                    (self.smoothed[k] - scaled) / self.smoothed[k]
                } else {
                    0.0
                }
            } else {
                1.0
            };
        }

        // 6. Floor the gain at a whitened noise level, so the residue is flat
        //    hiss rather than the carved shape of the noise it came from.
        self.apply_noise_floor();

        for (c, &g) in self.spec.iter_mut().zip(&self.gain) {
            *c *= g;
        }

        // Synthesis: inverse, window again, overlap-add.
        self.inv.process(&mut self.spec, &mut self.time).expect("inverse fft");
        for i in 0..self.frame {
            self.acc[i] += self.time[self.offset + i] * self.window[i] * self.inv_scale;
        }
    }

    /// The "bleach": replace the suppressed residue with a floor whose shape is
    /// the inverse of the noise profile, so what is left has no spectral
    /// structure for the ear to latch onto.
    fn apply_noise_floor(&mut self) {
        if self.floor >= 0.999 {
            self.gain.fill(1.0);
            return;
        }

        // Whitening weights: a valley in the noise profile gets a weight above
        // 1, a spike below it. Anchored to the median rather than the peak, so
        // a single carrier cannot drag the whole ceiling up with it.
        self.whiten_w.copy_from_slice(&self.noise);
        smooth_spectrum(&mut self.whiten_w, 0.5);
        self.sort_buf.copy_from_slice(&self.whiten_w);
        let mid = self.sort_buf.len() / 2;
        self.sort_buf.select_nth_unstable_by(mid, |a, b| a.total_cmp(b));
        let anchor = self.sort_buf[mid].max(EPS);

        let depth = 1.0 - self.floor;
        for k in 0..self.bins {
            let weight = if self.whitening > 0.0 && self.whiten_w[k] > EPS {
                (anchor / self.whiten_w[k]).powf(self.whitening)
            } else {
                1.0
            };
            // Damp the whitening toward unity as the reduction approaches
            // bypass, so a light setting stays light.
            let effective = 1.0 + depth * (weight - 1.0);
            let whitened_floor = (self.floor * effective).min(1.0);
            self.gain[k] = self.gain[k].max(whitened_floor);
        }
    }
}

impl Default for SpecBleachNr {
    fn default() -> Self {
        Self::new()
    }
}

// ------------------------------------------------------------------- helpers

/// Berouti's per-bin over-subtraction factor: full strength at or below
/// −5 dB SNR, none at or above +20 dB, linear in dB between.
fn berouti_per_bin(spectrum: &[f32], noise: &[f32], strength: f32, alpha: &mut [f32]) {
    let alpha_max = ALPHA_MIN + strength * (ALPHA_MAX - ALPHA_MIN);
    let range_db = SUPPRESSION_HIGHER_SNR_DB - SUPPRESSION_LOWER_SNR_DB;
    let lower = 10f32.powf(SUPPRESSION_LOWER_SNR_DB / 10.0);
    let higher = 10f32.powf(SUPPRESSION_HIGHER_SNR_DB / 10.0);

    for k in 0..alpha.len() {
        let ratio = spectrum[k] / (noise[k] + EPS);
        alpha[k] = if ratio <= lower {
            alpha_max
        } else if ratio >= higher {
            ALPHA_MIN
        } else {
            let snr_db = 10.0 * ratio.log10();
            let t = (snr_db - SUPPRESSION_LOWER_SNR_DB) / range_db;
            alpha_max - t * (alpha_max - ALPHA_MIN)
        };
    }
}

/// Three-point moving average blended with the original by `factor`, with
/// two-point averages at both edges so the smoothing reaches Nyquist.
fn smooth_spectrum(spectrum: &mut [f32], factor: f32) {
    let n = spectrum.len();
    if n < 2 || factor <= 0.0 {
        return;
    }
    let first = spectrum[0];
    spectrum[0] = (spectrum[0] + spectrum[1]) / 2.0 * factor + spectrum[0] * (1.0 - factor);
    let mut prev = first;
    for i in 1..n - 1 {
        let current = spectrum[i];
        spectrum[i] =
            ((prev + current + spectrum[i + 1]) / 3.0) * factor + current * (1.0 - factor);
        prev = current;
    }
    spectrum[n - 1] = ((prev + spectrum[n - 1]) / 2.0) * factor + spectrum[n - 1] * (1.0 - factor);
}

/// Linearly bridge runs of near-zero bins, which a per-bin estimator leaves
/// behind and which would otherwise read as infinite SNR.
fn interpolate_gaps(spectrum: &mut [f32], threshold: f32) {
    let n = spectrum.len();
    if n < 3 {
        return;
    }
    let mut i = 1;
    while i < n - 1 {
        if spectrum[i] >= threshold {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < n && spectrum[j] < threshold {
            j += 1;
        }
        if j >= n {
            break;
        }
        let start = spectrum[i - 1];
        let step = (spectrum[j] - start) / (j - (i - 1)) as f32;
        for (offset, bin) in spectrum[i..j].iter_mut().enumerate() {
            *bin = start + step * (offset + 1) as f32;
        }
        i = j + 1;
    }
}

/// In-place 3-point (0.25, 0.5, 0.25) smoothing across an array.
fn smooth_across(data: &mut [f32]) {
    let n = data.len();
    if n < 2 {
        return;
    }
    let mut prev = data[0];
    for i in 1..n {
        let current = data[i];
        let next = if i < n - 1 { data[i + 1] } else { current };
        data[i] = 0.25 * prev + 0.5 * current + 0.25 * next;
        prev = current;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rms(x: &[f32]) -> f32 {
        (x.iter().map(|&s| s * s).sum::<f32>() / x.len().max(1) as f32).sqrt()
    }

    fn noise(n: usize, amp: f32, seed: u64) -> Vec<f32> {
        let mut r = seed;
        (0..n)
            .map(|_| {
                r ^= r << 13;
                r ^= r >> 7;
                r ^= r << 17;
                (((r >> 40) as i32) as f32 / (1 << 23) as f32 - 1.0) * amp
            })
            .collect()
    }

    #[test]
    fn preserves_sample_count() {
        let mut nr = SpecBleachNr::new();
        for &len in &[512usize, 300, 480, 1024, 137] {
            let mut buf = vec![0.1f32; len];
            nr.process(&mut buf);
            assert_eq!(buf.len(), len);
        }
    }

    #[test]
    fn reduces_broadband_noise_at_full_strength() {
        let mut nr = SpecBleachNr::new();
        nr.set_rate(48_000.0);
        nr.set_params(20.0, 0.3);
        let mut buf = noise(96_000, 0.2, 0x51DE);
        let before = rms(&buf);
        nr.process(&mut buf);
        let after = rms(&buf[nr.latency_samples() * 4..]);
        assert!(after < 0.9 * before, "noise not reduced: {before} -> {after}");
    }

    #[test]
    fn low_strength_reduces_less_than_high() {
        let run = |db: f32| {
            let mut nr = SpecBleachNr::new();
            nr.set_rate(48_000.0);
            nr.set_params(db, 0.0);
            let mut buf = noise(96_000, 0.2, 0xD00D);
            nr.process(&mut buf);
            rms(&buf[nr.latency_samples() * 4..])
        };
        let (low, high) = (run(6.0), run(20.0));
        assert!(low > high, "6 dB was not gentler than 20 dB: {low} vs {high}");
    }

    #[test]
    fn zero_reduction_is_transparent() {
        // At 0 dB the floor is unity, every gain is forced to 1, and the STFT
        // has to be an identity. This pins three things at once that an RMS
        // test cannot see: the window scaling, the ms-to-samples frame size,
        // and that the delay really is `latency_samples`.
        let mut nr = SpecBleachNr::new();
        nr.set_rate(48_000.0);
        nr.set_params(0.0, 0.0);
        let input = noise(48_000, 0.2, 0xFEED);
        let mut buf = input.clone();
        nr.process(&mut buf);
        let lat = nr.latency_samples();
        // Skip the overlap-add warm-up, where the first output region has only
        // some of its four window contributions.
        for i in lat * 2..input.len() {
            assert!(
                (buf[i] - input[i - lat]).abs() < 1e-3,
                "not transparent at {i}: {} vs {}",
                buf[i],
                input[i - lat]
            );
        }
    }

    #[test]
    fn runs_at_other_rates() {
        for &rate in &[8_000.0f64, 16_000.0, 44_100.0] {
            let mut nr = SpecBleachNr::new();
            nr.set_rate(rate);
            nr.set_params(12.0, 0.15);
            let n = rate as usize;
            let mut buf = noise(n, 0.15, 0xBEE5);
            nr.process(&mut buf);
            assert_eq!(buf.len(), n);
            assert!(buf.iter().all(|s| s.is_finite()), "non-finite output at {rate} Hz");
        }
    }

    #[test]
    fn latency_is_a_frame_not_a_millisecond() {
        // Upstream sizes the frame in *milliseconds* and reports latency in
        // *samples*. Getting that conversion wrong is inaudible in an RMS test,
        // so pin it: 20 ms at 48 kHz is about 960 samples — not 20, not 960000.
        let mut nr = SpecBleachNr::new();
        nr.set_rate(48_000.0);
        let l = nr.latency_samples();
        assert!((480..=4800).contains(&l), "implausible latency {l} samples at 48 kHz");
    }
}
