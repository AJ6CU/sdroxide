//! Adaptive IQ image balancer, transcribed from libairspyhf's `iqbalancer.c`.
//!
//! # What it is for
//!
//! On the zero-IF sample rates (768 and 912 kSPS) the receiver hands over
//! quadrature that is not perfectly balanced: the I and Q paths differ slightly
//! in gain and in phase, and the result is a mirror image of every signal
//! reflected about the centre of the passband. Hardware alone leaves that image
//! tens of dB up; correcting it is the host's job, and it is the reason the
//! vendor library exists at all rather than the device being usable as a plain
//! bulk endpoint. The low-IF rates do not need it — the firmware has already
//! moved the wanted signal away from the mirror — so this runs only where the
//! architecture byte says zero-IF.
//!
//! # How it works
//!
//! The correction itself is two multiply-adds per sample:
//!
//! ```text
//! re' = (re + phase·im)·(1 + amplitude)
//! im' = (im + phase·re)·(1 − amplitude)
//! ```
//!
//! Finding `phase` and `amplitude` is the interesting part. Every
//! [`WORKING_BUFFER_LEN`] samples the balancer runs a handful of overlapping
//! 4096-point windowed FFTs and correlates each bin against its mirror — a
//! product that is large exactly when a signal and its image are present
//! together. It evaluates that metric twice, once at the current estimate and
//! once one step away in both parameters, and takes a secant step towards the
//! zero. Estimates are averaged over the last [`MAX_LOOKBACK`] updates.
//!
//! Convergence is deliberately slow: at the default settings a single update
//! consumes tens of working buffers, so the estimate does not chase a fading
//! signal. Expect several seconds of real signal before the image settles.
//!
//! # Divergences from upstream, and why
//!
//! * **The "optimal bin" weighting is not implemented.** Upstream can bias the
//!   search towards one bin (`airspyhf_set_optimal_iq_correction_point`); with
//!   its default of 0.0 the bias window is the identity and the whole path is
//!   inert. This driver never sets it, so the machinery — and the
//!   `integrated_image_power` accumulator that only that path reads — is left
//!   out rather than carried untested.
//! * **The per-block interpolation runs the other way.** Upstream ramps from
//!   the *new* estimate at the start of a block to the *old* one at the end,
//!   which is backwards; here a block ramps from the previous estimate to the
//!   current one. It matters for exactly one block after each update and only
//!   by the size of the update, but there is no reason to reproduce it.
//! * **`rustfft` replaces the hand-rolled radix-2**, with the same forward sign
//!   convention, the same lack of normalisation, and the same `fftshift`
//!   afterwards so that bin [`FFT_BINS`]/2 is DC.
//! * **The window comes from [`sdroxide_dsp::blackman_harris`]**, which is the
//!   same four-term window with the same coefficients and the same symmetric
//!   `n − 1` denominator.
//!
//! Copyright of the original algorithm: Youssef Touil and Leif Asbrink,
//! BSD-3-Clause.

use std::sync::Arc;

use rustfft::{Fft, FftPlanner};
use sdroxide_dsp::Complex32;

/// FFT size the whole estimator is built around.
pub const FFT_BINS: usize = 4096;

/// Bins at each edge of the spectrum excluded from the correlation. The
/// firmware's own filter rolls off there, so those bins carry shaping rather
/// than signal.
const EDGE_BINS_TO_SKIP: usize = FFT_BINS / 22;
/// Bins either side of DC excluded, where residual carrier leakage lives.
const CENTER_BINS_TO_SKIP: usize = 2;

/// How many past estimates are averaged into the value actually applied.
pub const MAX_LOOKBACK: usize = 4;

const PHASE_STEP: f32 = 1e-2;
const AMPLITUDE_STEP: f32 = 1e-2;
/// The secant step is a ratio and blows up when the denominator is small.
const MAX_MU: f32 = 50.0;
const MIN_DELTA_MU: f32 = 0.1;

/// One-pole coefficient for the DC average. At 768 kSPS the time constant is
/// about 13 ms.
const DC_TIME_CONST: f32 = 1e-4;

/// Block energy below which a window is treated as silence and left out of the
/// estimate — there is nothing in it to balance.
const MINIMUM_POWER: f32 = 0.01;

/// Working buffers ignored after a reset, before any estimate is trusted.
const BUFFERS_TO_SKIP_ON_RESET: i32 = 2;
/// The running peak decays, so a single loud burst cannot gate out every later
/// estimate for the rest of the session.
const MAX_POWER_DECAY: f32 = 0.98;

/// Guard against dividing by an empty bin in the correlation weighting.
const EPSILON: f32 = 0.01;

/// Defaults from `iqbalancer.h`'s non-ARM branch.
const BUFFERS_TO_SKIP: i32 = 2;
const FFT_INTEGRATION: usize = 4;
const FFT_OVERLAP: usize = 2;
const CORRELATION_INTEGRATION: usize = 16;

/// Samples accumulated before an estimate is attempted: enough for
/// [`FFT_INTEGRATION`] windows overlapping by [`FFT_OVERLAP`].
pub const WORKING_BUFFER_LEN: usize = FFT_BINS * (1 + FFT_INTEGRATION / FFT_OVERLAP);

/// Seed values, from `airspyhf.c`'s `INITIAL_PHASE` / `INITIAL_AMPLITUDE`.
/// They are the typical imbalance of the hardware, so the receiver starts
/// roughly right and converges from there rather than from zero.
pub const INITIAL_PHASE: f32 = 0.00006;
pub const INITIAL_AMPLITUDE: f32 = -0.0045;

/// Estimator state and the buffers it works in. One per receiver.
pub struct IqBalancer {
    phase: f32,
    last_phase: f32,
    amplitude: f32,
    last_amplitude: f32,

    iavg: f32,
    qavg: f32,

    integrated_total_power: f32,
    maximum_image_power: f32,

    raw_phases: [f32; MAX_LOOKBACK],
    raw_amplitudes: [f32; MAX_LOOKBACK],
    no_of_raw: usize,
    raw_ptr: usize,

    skipped_buffers: i32,
    buffers_to_skip: i32,
    fft_integration: usize,
    fft_overlap: usize,
    correlation_integration: usize,
    no_of_avg: i32,
    reset_flag: bool,

    working_buffer: Vec<Complex32>,
    working_buffer_pos: usize,
    corr: Vec<Complex32>,
    corr_plus: Vec<Complex32>,
    boost: Vec<f32>,
    power_flag: Vec<bool>,

    fft: Arc<dyn Fft<f32>>,
    fft_buf: Vec<Complex32>,
    fft_scratch: Vec<Complex32>,
    window: Vec<f32>,

    /// How many times the estimate has actually been updated. Reported in the
    /// diagnostics: "converged to (x, y)" means nothing without it.
    estimates: u64,
}

impl Default for IqBalancer {
    fn default() -> Self {
        IqBalancer::new()
    }
}

impl IqBalancer {
    pub fn new() -> IqBalancer {
        IqBalancer::with_seed(INITIAL_PHASE, INITIAL_AMPLITUDE)
    }

    pub fn with_seed(phase: f32, amplitude: f32) -> IqBalancer {
        // Planned once and held, along with its scratch: `process` runs on the
        // stream thread and must never allocate.
        let fft = FftPlanner::<f32>::new().plan_fft_forward(FFT_BINS);
        let fft_scratch = vec![Complex32::default(); fft.get_inplace_scratch_len()];
        IqBalancer {
            phase,
            last_phase: phase,
            amplitude,
            last_amplitude: amplitude,
            iavg: 0.0,
            qavg: 0.0,
            integrated_total_power: 0.0,
            maximum_image_power: 0.0,
            raw_phases: [0.0; MAX_LOOKBACK],
            raw_amplitudes: [0.0; MAX_LOOKBACK],
            no_of_raw: 0,
            raw_ptr: 0,
            skipped_buffers: 0,
            buffers_to_skip: BUFFERS_TO_SKIP,
            fft_integration: FFT_INTEGRATION,
            fft_overlap: FFT_OVERLAP,
            correlation_integration: CORRELATION_INTEGRATION,
            no_of_avg: 0,
            reset_flag: true,
            working_buffer: vec![Complex32::default(); WORKING_BUFFER_LEN],
            working_buffer_pos: 0,
            corr: vec![Complex32::default(); FFT_BINS],
            corr_plus: vec![Complex32::default(); FFT_BINS],
            boost: vec![0.0; FFT_BINS],
            power_flag: vec![false; FFT_INTEGRATION],
            fft,
            fft_buf: vec![Complex32::default(); FFT_BINS],
            fft_scratch,
            window: sdroxide_dsp::blackman_harris(FFT_BINS),
            estimates: 0,
        }
    }

    /// Trade convergence speed for CPU, as upstream's
    /// `airspyhf_iq_balancer_configure` does.
    ///
    /// The defaults are tuned for a receiver left running; the fast settings
    /// exist for the offline `balance_check` example and the tests, which
    /// cannot spend ten seconds of signal per update.
    pub fn configure(
        &mut self,
        buffers_to_skip: i32,
        fft_integration: usize,
        fft_overlap: usize,
        correlation_integration: usize,
    ) {
        self.buffers_to_skip = buffers_to_skip.max(0);
        self.fft_integration = fft_integration.max(1);
        self.fft_overlap = fft_overlap.max(1);
        self.correlation_integration = correlation_integration.max(1);
        self.power_flag = vec![false; self.fft_integration];
        self.reset_flag = true;
    }

    /// Start the estimate over.
    ///
    /// Called on every retune: the imbalance is frequency-dependent, so an
    /// estimate converged on the old dial is a correction pointed at the wrong
    /// place. The seed is kept — it is closer to right than zero is.
    pub fn reset(&mut self) {
        self.reset_flag = true;
    }

    /// The correction currently applied, for the diagnostics report.
    pub fn state(&self) -> (f32, f32) {
        (self.phase, self.amplitude)
    }

    /// The DC offset being subtracted, for the diagnostics report.
    pub fn dc(&self) -> (f32, f32) {
        (self.iavg, self.qavg)
    }

    /// How many times the estimate has been updated since the last reset.
    pub fn estimates(&self) -> u64 {
        self.estimates
    }

    /// Cancel DC, feed the estimator, and apply the current correction.
    ///
    /// In place, and never allocating — this runs on the stream thread between
    /// USB completions.
    pub fn process(&mut self, iq: &mut [Complex32]) {
        self.cancel_dc(iq);
        self.accumulate(iq);
        self.adjust_phase_amplitude(iq);
    }

    /// A one-pole average per channel, subtracted as it goes.
    ///
    /// Note this is the *only* DC removal in the chain, and upstream runs it
    /// only on the zero-IF rates because it lives inside the balancer. A low-IF
    /// rate has the wanted signal away from DC anyway, so what is left at DC is
    /// the converter's own offset and no one misses it.
    fn cancel_dc(&mut self, iq: &mut [Complex32]) {
        let (mut iavg, mut qavg) = (self.iavg, self.qavg);
        for x in iq.iter_mut() {
            iavg += DC_TIME_CONST * (x.re - iavg);
            qavg += DC_TIME_CONST * (x.im - qavg);
            x.re -= iavg;
            x.im -= qavg;
        }
        self.iavg = iavg;
        self.qavg = qavg;
    }

    /// Copy into the working buffer, and run an estimate when it fills.
    fn accumulate(&mut self, iq: &[Complex32]) {
        let mut src = 0;
        while src < iq.len() {
            let take = (WORKING_BUFFER_LEN - self.working_buffer_pos).min(iq.len() - src);
            let dst = self.working_buffer_pos;
            self.working_buffer[dst..dst + take].copy_from_slice(&iq[src..src + take]);
            self.working_buffer_pos += take;
            src += take;
            if self.working_buffer_pos < WORKING_BUFFER_LEN {
                break;
            }
            self.working_buffer_pos = 0;
            self.skipped_buffers += 1;
            if self.skipped_buffers > self.buffers_to_skip {
                self.skipped_buffers = 0;
                self.estimate_imbalance();
            }
        }
        // Upstream copies at most one buffer's worth per call and drops the
        // rest, which is safe there because it is only ever handed 4096 samples
        // at a time. This loop keeps every sample regardless of block size, so
        // the estimator sees the same signal whatever the transfer geometry is.
    }

    /// Interpolate from the previous estimate to the current one across the
    /// block, so an update does not step the correction discontinuously.
    fn adjust_phase_amplitude(&mut self, iq: &mut [Complex32]) {
        let n = iq.len();
        if n == 0 {
            return;
        }
        let scale = if n > 1 { 1.0 / (n - 1) as f32 } else { 0.0 };
        let (dp, da) = (self.phase - self.last_phase, self.amplitude - self.last_amplitude);
        for (i, x) in iq.iter_mut().enumerate() {
            let t = i as f32 * scale;
            let phase = self.last_phase + dp * t;
            let amplitude = self.last_amplitude + da * t;
            // Both corrections read the *original* re and im; using the updated
            // `re` in the second line would be a different transform and a
            // convergence that quietly settles somewhere else.
            let (re, im) = (x.re, x.im);
            x.re = (re + phase * im) * (1.0 + amplitude);
            x.im = (im + phase * re) * (1.0 - amplitude);
        }
        self.last_phase = self.phase;
        self.last_amplitude = self.amplitude;
    }

    fn estimate_imbalance(&mut self) {
        if self.reset_flag {
            self.reset_flag = false;
            self.no_of_avg = -BUFFERS_TO_SKIP_ON_RESET;
            self.maximum_image_power = 0.0;
            self.estimates = 0;
        }
        if self.no_of_avg < 0 {
            // The buffers straight after a retune still hold the old dial's
            // signal, so they are counted out rather than averaged in.
            self.no_of_avg += 1;
            return;
        }
        if self.no_of_avg == 0 {
            self.integrated_total_power = 0.0;
            self.boost.fill(0.0);
            self.corr.fill(Complex32::default());
            self.corr_plus.fill(Complex32::default());
        }
        self.maximum_image_power *= MAX_POWER_DECAY;

        // Taken out so the correlator can borrow the working buffer and the
        // accumulators at the same time; put back before returning.
        let wb = std::mem::take(&mut self.working_buffer);
        let mut corr = std::mem::take(&mut self.corr);
        let mut corr_plus = std::mem::take(&mut self.corr_plus);

        let count = self.compute_corr(&wb, &mut corr, 0);
        if count > 0 {
            self.no_of_avg += count as i32;
            self.compute_corr(&wb, &mut corr_plus, 1);
        }

        self.working_buffer = wb;

        if count == 0 {
            self.corr = corr;
            self.corr_plus = corr_plus;
            return;
        }
        if self.no_of_avg <= (self.correlation_integration * self.fft_integration) as i32 {
            self.corr = corr;
            self.corr_plus = corr_plus;
            return;
        }
        self.no_of_avg = 0;

        // Only let a stretch at least as strong as the running peak move the
        // estimate. A quiet band carries no image to measure, and averaging its
        // noise in would walk the correction away from the right answer.
        let gated = self.integrated_total_power < self.maximum_image_power;
        if !gated {
            self.maximum_image_power = self.integrated_total_power;
            let a = utility(&corr, &self.boost);
            let b = utility(&corr_plus, &self.boost);
            let phase = self.phase + PHASE_STEP * secant_step(a.im, b.im);
            let amplitude = self.amplitude + AMPLITUDE_STEP * secant_step(a.re, b.re);
            self.push_estimate(phase, amplitude);
            self.estimates += 1;
        }

        self.corr = corr;
        self.corr_plus = corr_plus;
    }

    /// Average the new estimate with the last [`MAX_LOOKBACK`] − 1 of them.
    fn push_estimate(&mut self, phase: f32, amplitude: f32) {
        if self.no_of_raw < MAX_LOOKBACK {
            self.no_of_raw += 1;
        }
        self.raw_phases[self.raw_ptr] = phase;
        self.raw_amplitudes[self.raw_ptr] = amplitude;

        let (mut p, mut a) = (phase, amplitude);
        let mut i = self.raw_ptr;
        for _ in 0..self.no_of_raw - 1 {
            i = (i + MAX_LOOKBACK - 1) & (MAX_LOOKBACK - 1);
            p += self.raw_phases[i];
            a += self.raw_amplitudes[i];
        }
        self.phase = p / self.no_of_raw as f32;
        self.amplitude = a / self.no_of_raw as f32;
        self.raw_ptr = (self.raw_ptr + 1) & (MAX_LOOKBACK - 1);
    }

    /// Correlate every bin against its mirror, at the current estimate
    /// (`step == 0`) or one step away from it (`step == 1`).
    ///
    /// Returns how many windows carried enough signal to be used. Zero means
    /// the band was silent and nothing should move.
    fn compute_corr(&mut self, src: &[Complex32], ccorr: &mut [Complex32], step: i32) -> usize {
        let phase = self.phase + step as f32 * PHASE_STEP;
        let amplitude = self.amplitude + step as f32 * AMPLITUDE_STEP;
        let hop = FFT_BINS / self.fft_overlap;
        let fft = Arc::clone(&self.fft);
        let mut count = 0;

        let mut m = 0;
        let mut n = 0;
        while n + FFT_BINS <= src.len() && m < self.fft_integration {
            self.fft_buf.copy_from_slice(&src[n..n + FFT_BINS]);
            let power = adjust_benchmark(&mut self.fft_buf, phase, amplitude, step == 0);

            if step == 0 {
                self.power_flag[m] = power > MINIMUM_POWER;
                if self.power_flag[m] {
                    self.integrated_total_power += power;
                }
            }

            if self.power_flag[m] {
                count += 1;
                for (x, w) in self.fft_buf.iter_mut().zip(&self.window) {
                    *x *= *w;
                }
                fft.process_with_scratch(&mut self.fft_buf, &mut self.fft_scratch);
                // Upstream's hand-rolled FFT ends with this swap, so bin
                // FFT_BINS/2 is DC and bin `FFT_BINS - i` is bin `i` mirrored
                // about it. The whole correlation is expressed in those
                // coordinates.
                self.fft_buf.rotate_left(FFT_BINS / 2);

                let (mut i, mut j) = (EDGE_BINS_TO_SKIP, FFT_BINS - EDGE_BINS_TO_SKIP);
                while i <= FFT_BINS - EDGE_BINS_TO_SKIP {
                    // A plain product, not a conjugate one: a signal and its
                    // image have opposite phase slopes, so their product has a
                    // phase that reads out the imbalance directly.
                    let cc = self.fft_buf[i] * self.fft_buf[j];
                    ccorr[i] += cc;
                    ccorr[j] = ccorr[i];
                    i += 1;
                    j -= 1;
                }

                if step == 0 {
                    for i in EDGE_BINS_TO_SKIP..=FFT_BINS - EDGE_BINS_TO_SKIP {
                        self.boost[i] += self.fft_buf[i].norm_sqr();
                    }
                }
            }

            n += hop;
            m += 1;
        }
        count
    }
}

/// Apply a candidate correction to a window, returning the energy it had before
/// the correction (or zero when the caller does not need it).
fn adjust_benchmark(buf: &mut [Complex32], phase: f32, amplitude: f32, want_power: bool) -> f32 {
    let mut sum = 0.0;
    for x in buf.iter_mut() {
        let (re, im) = (x.re, x.im);
        x.re = (re + phase * im) * (1.0 + amplitude);
        x.im = (im + phase * re) * (1.0 - amplitude);
        if want_power {
            sum += re * re + im * im;
        }
    }
    sum
}

/// Collapse the per-bin correlation into the one complex number the secant
/// search works on: the real part drives amplitude, the imaginary part phase.
///
/// Bins are weighted by how much stronger their mirror is than they are, so a
/// bin holding a real signal contributes little and a bin holding mostly that
/// signal's image contributes a lot.
fn utility(ccorr: &[Complex32], boost: &[f32]) -> Complex32 {
    let inv_skip = 1.0 / EDGE_BINS_TO_SKIP as f32;
    let mut acc = Complex32::default();
    let (mut i, mut j) = (EDGE_BINS_TO_SKIP, FFT_BINS - EDGE_BINS_TO_SKIP);
    while i <= FFT_BINS - EDGE_BINS_TO_SKIP {
        let distance = i.abs_diff(FFT_BINS / 2);
        if distance > CENTER_BINS_TO_SKIP {
            // Bins close to DC are faded in rather than switched on, so the
            // exclusion zone does not put a step in the metric.
            let mut weight =
                if distance > EDGE_BINS_TO_SKIP { 1.0 } else { distance as f32 * inv_skip };
            weight *= boost[j] / (boost[i] + EPSILON);
            acc += ccorr[i] * weight;
        }
        i += 1;
        j -= 1;
    }
    acc
}

/// One secant step towards the zero of the metric, clamped.
///
/// `a` is the metric at the current estimate and `b` one step away. When the
/// two are nearly equal the ratio is meaningless — the metric is flat, or the
/// band is noise — so the estimate stays where it is rather than being thrown
/// by a division by almost nothing.
fn secant_step(a: f32, b: f32) -> f32 {
    let delta = a - b;
    if delta.abs() > MIN_DELTA_MU { (a / delta).clamp(-MAX_MU, MAX_MU) } else { 0.0 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::TAU;

    /// The imbalance a receiver imposes, in the same functional form the
    /// corrector undoes — so a converged balancer should land near the negated
    /// parameters.
    fn distort(x: Complex32, phase: f32, amplitude: f32) -> Complex32 {
        let (re, im) = (x.re, x.im);
        Complex32::new((re + phase * im) * (1.0 + amplitude), (im + phase * re) * (1.0 - amplitude))
    }

    /// A complex tone `bin` bins above centre, imbalanced by `(p, a)`.
    fn tone(n: usize, bin: f32, p: f32, a: f32, phase0: f32) -> Vec<Complex32> {
        (0..n)
            .map(|i| {
                let t = TAU * bin * i as f32 / FFT_BINS as f32 + phase0;
                distort(Complex32::new(t.cos(), t.sin()), p, a)
            })
            .collect()
    }

    /// Power at `bin` and at its mirror, in dB, over one 4096-point window.
    fn image_ratio_db(x: &[Complex32], bin: usize) -> f32 {
        let mut buf: Vec<Complex32> = x[x.len() - FFT_BINS..].to_vec();
        let w = sdroxide_dsp::blackman_harris(FFT_BINS);
        for (v, wv) in buf.iter_mut().zip(&w) {
            *v *= *wv;
        }
        FftPlanner::<f32>::new().plan_fft_forward(FFT_BINS).process(&mut buf);
        let signal = buf[bin].norm_sqr();
        let image = buf[FFT_BINS - bin].norm_sqr();
        10.0 * (signal.max(1e-30) / image.max(1e-30)).log10()
    }

    /// The whole point of the file. An imbalanced tone must come out with its
    /// mirror image markedly further down than it went in.
    #[test]
    fn the_balancer_converges_on_a_synthetically_imbalanced_tone() {
        const BIN: usize = 700;
        let (inject_p, inject_a) = (0.02f32, 0.01f32);

        let before = image_ratio_db(&tone(FFT_BINS * 2, BIN as f32, inject_p, inject_a, 0.0), BIN);
        // A worthwhile test needs an image worth removing in the first place.
        assert!(before < 45.0, "the injected imbalance is too small to measure: {before:.1} dB");

        let mut b = IqBalancer::new();
        // The shipping settings need seconds of signal per update; the test
        // needs the same algorithm to converge in a second of it.
        b.configure(0, 4, 2, 1);

        let mut last = Vec::new();
        // Each block carries its own starting phase, so the estimator sees a
        // continuous tone rather than the same 12288 samples over and over.
        for blk in 0..40 {
            let phase0 = TAU * BIN as f32 * (blk * WORKING_BUFFER_LEN) as f32 / FFT_BINS as f32;
            let mut x = tone(WORKING_BUFFER_LEN, BIN as f32, inject_p, inject_a, phase0);
            b.process(&mut x);
            last = x;
        }

        let after = image_ratio_db(&last, BIN);
        assert!(b.estimates() > 0, "the estimator never ran");
        assert!(
            after - before >= 25.0,
            "image rejection improved by only {:.1} dB ({before:.1} → {after:.1}); \
             estimate ({:.5}, {:.5}) against an injected ({inject_p}, {inject_a})",
            after - before,
            b.state().0,
            b.state().1,
        );
        // The correction has to be the *inverse* of the imbalance. A sign slip
        // in either parameter still moves the metric, so check the values and
        // not only the improvement.
        let (p, a) = b.state();
        assert!(
            (p + inject_p).abs() < 0.2 * inject_p,
            "phase estimate {p:.5} is not near {:.5}",
            -inject_p
        );
        assert!(
            (a + inject_a).abs() < 0.2 * inject_a,
            "amplitude estimate {a:.5} is not near {:.5}",
            -inject_a
        );
    }

    /// A receiver that needs no correction must not be given one.
    ///
    /// The metric has a zero at "already balanced", and an estimator that
    /// wandered away from it would put an image on a clean signal. Judged on
    /// the estimate rather than on the measured rejection: a synthetic tone
    /// starts out 130-odd dB down, which no hardware produces, so any finite
    /// correction "worsens" a number that was never physical.
    #[test]
    fn a_clean_signal_leaves_the_correction_alone() {
        const BIN: usize = 700;
        let mut b = IqBalancer::with_seed(0.0, 0.0);
        b.configure(0, 4, 2, 1);

        let mut last = Vec::new();
        for blk in 0..40 {
            let phase0 = TAU * BIN as f32 * (blk * WORKING_BUFFER_LEN) as f32 / FFT_BINS as f32;
            let mut x = tone(WORKING_BUFFER_LEN, BIN as f32, 0.0, 0.0, phase0);
            b.process(&mut x);
            last = x;
        }

        let (p, a) = b.state();
        assert!(b.estimates() > 0, "the estimator never ran, so this proves nothing");
        // Three orders of magnitude below the imbalance the other test injects.
        assert!(
            p.abs() < 1e-4 && a.abs() < 1e-4,
            "a clean signal moved the estimate to ({p}, {a})"
        );
        // And the output is still far cleaner than the hardware itself manages.
        let after = image_ratio_db(&last, BIN);
        assert!(after > 60.0, "image rejection fell to {after:.1} dB on a balanced signal");
    }

    /// Silence carries no image to measure. Moving the estimate on it would
    /// walk the correction away between overs.
    #[test]
    fn a_silent_band_does_not_move_the_estimate() {
        let mut b = IqBalancer::new();
        b.configure(0, 4, 2, 1);
        for _ in 0..40 {
            let mut x = vec![Complex32::default(); WORKING_BUFFER_LEN];
            b.process(&mut x);
        }
        assert_eq!(b.estimates(), 0);
        assert_eq!(b.state(), (INITIAL_PHASE, INITIAL_AMPLITUDE));
    }

    /// The imbalance is frequency-dependent, so a retune has to start over —
    /// keeping the seed, which is closer to right than zero.
    #[test]
    fn a_retune_resets_the_estimate_to_its_seed() {
        let mut b = IqBalancer::new();
        b.configure(0, 4, 2, 1);
        for blk in 0..40 {
            let phase0 = TAU * 700.0 * (blk * WORKING_BUFFER_LEN) as f32 / FFT_BINS as f32;
            let mut x = tone(WORKING_BUFFER_LEN, 700.0, 0.02, 0.01, phase0);
            b.process(&mut x);
        }
        assert!(b.estimates() > 0);
        b.reset();
        // The reset lands on the next estimate attempt, which is where upstream
        // puts it too: `process` must stay cheap.
        let mut x = vec![Complex32::default(); WORKING_BUFFER_LEN];
        b.process(&mut x);
        assert_eq!(b.estimates(), 0);
    }

    /// A flat metric must not produce an unbounded step.
    #[test]
    fn the_secant_step_is_clamped_and_gated() {
        assert_eq!(secant_step(1.0, 1.0), 0.0);
        assert_eq!(secant_step(1.0, 0.95), 0.0, "a difference under the floor is not a gradient");
        // a/(a-b) with a barely above the floor: clamped, not enormous.
        assert!(secant_step(100.0, 99.85).abs() <= MAX_MU);
        assert!(secant_step(-100.0, -99.85).abs() <= MAX_MU);
        // A well-conditioned pair steps towards the zero of the line through
        // them: a = 2, b = 1 has its zero one step further on.
        assert!((secant_step(2.0, 1.0) - 2.0).abs() < 1e-6);
    }

    /// The offset is removed independently on each channel and converges within
    /// a few time constants — about 13 ms of audio at 768 kSPS.
    #[test]
    fn dc_cancellation_converges_on_both_channels() {
        let mut b = IqBalancer::with_seed(0.0, 0.0);
        let mut x = vec![Complex32::new(0.25, -0.5); 60_000];
        b.process(&mut x);
        let (i, q) = b.dc();
        assert!((i - 0.25).abs() < 0.01, "I average {i}");
        assert!((q + 0.5).abs() < 0.01, "Q average {q}");
        let tail = &x[x.len() - 1000..];
        assert!(tail.iter().all(|v| v.re.abs() < 0.01 && v.im.abs() < 0.01), "DC survived");
    }

    /// Block size is a property of the USB transfer geometry, not of the
    /// signal. Feeding the same samples in different-sized pieces has to give
    /// the same estimate, or a config change would silently retune the DSP.
    #[test]
    fn the_estimate_does_not_depend_on_the_block_size() {
        let make = || {
            let mut b = IqBalancer::new();
            b.configure(0, 4, 2, 1);
            b
        };
        let (mut whole, mut split) = (make(), make());
        for blk in 0..24 {
            let phase0 = TAU * 700.0 * (blk * WORKING_BUFFER_LEN) as f32 / FFT_BINS as f32;
            let x = tone(WORKING_BUFFER_LEN, 700.0, 0.02, 0.01, phase0);
            whole.process(&mut x.clone());
            let mut y = x.clone();
            for chunk in y.chunks_mut(1000) {
                split.process(chunk);
            }
        }
        assert_eq!(whole.estimates(), split.estimates());
        let (p0, a0) = whole.state();
        let (p1, a1) = split.state();
        assert!((p0 - p1).abs() < 1e-6, "{p0} vs {p1}");
        assert!((a0 - a1).abs() < 1e-6, "{a0} vs {a1}");
    }
}
