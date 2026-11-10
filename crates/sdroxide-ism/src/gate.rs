//! The burst gate: turns a continuously running channel into discrete bursts.
//!
//! ISM devices transmit for a few milliseconds and then sleep for a minute. The
//! gate watches one channel's baseband power, notices when something arrives,
//! and hands the whole transmission — preamble included — to the decoders as a
//! single buffer they can make several passes over.
//!
//! # The noise floor
//!
//! A threshold is only meaningful against a floor, and the floor has to be
//! learned from a channel that is *usually* quiet but sometimes not. The obvious
//! rule — only update the floor while the gate is shut — deadlocks in both
//! directions: seeded too low the gate never shuts, so the floor never updates;
//! seeded too high it never opens, and again never updates.
//!
//! So the floor tracks *downward fast and upward very slowly*. It converges on
//! the quietest power seen recently, which on a channel like this is the noise,
//! and a burst lasting a thousandth of the rise time barely moves it. It cannot
//! get stuck, and it needs no priming period.
//!
//! Min-tracking of a noisy quantity settles below that quantity's mean, so the
//! estimate is scaled back up by [`FLOOR_BIAS`]. That constant is *measured* by
//! this module's own test against synthetic noise of known power, not reasoned
//! about and hoped for.
//!
//! # Pre-trigger
//!
//! The gate cannot open until power has already risen, by which time some of the
//! preamble is gone — and the preamble is what the bit slicers recover timing
//! from. A short ring of past samples is therefore prepended to every burst, so
//! a decoder sees the transmission from before the gate noticed it.

use sdroxide_dsp::Complex32;

/// Samples per power measurement.
///
/// Long enough that the mean of `|z|²` is a usable estimate — the relative
/// spread of a mean of *n* exponential samples is `1/√n`, so 32 samples is about
/// 18 %, or 0.7 dB — and short enough to place the edge of a burst to well
/// inside one symbol at any rate these protocols use.
const BLOCK: usize = 32;

/// Per-block smoothing when the measurement is *below* the current floor.
const FLOOR_FALL: f32 = 0.05;

/// Per-block smoothing when it is above. Three orders of magnitude slower, so a
/// burst has to be a substantial fraction of the channel's duty cycle before it
/// starts lifting the floor that is meant to reveal it.
const FLOOR_RISE: f32 = 0.00005;

/// Correction from where min-tracking settles to the actual mean noise power.
///
/// Verified by [`tests::the_floor_estimate_lands_on_the_true_noise_power`],
/// which feeds noise of known power and checks the settled estimate to within
/// 1 dB. Changing `BLOCK`, `FLOOR_FALL` or `FLOOR_RISE` changes this number and
/// that test is what will say so.
const FLOOR_BIAS: f32 = 1.34;

/// How far the power must fall back below the open threshold before the burst is
/// considered over, as a power ratio. 6 dB of hysteresis stops a burst with a
/// momentary dip — a long run of identical symbols in a deviation-limited FSK
/// signal, say — being chopped into two.
const CLOSE_RATIO: f32 = 0.25;

/// Blocks below the close threshold before the gate shuts. At 250 kHz this is
/// about 1 ms, comfortably longer than any inter-symbol dip and far shorter than
/// the gap between two transmissions.
const HANG_BLOCKS: u32 = 8;

/// Shortest burst worth decoding, in seconds. Below this there is not room for a
/// preamble, a sync word and a payload at any rate in the plan, so it is a click
/// — impulse noise, or a switching supply.
const MIN_BURST_S: f64 = 0.001;

/// Longest burst kept, in seconds.
///
/// A transmission longer than this is not one of these protocols. Capping bounds
/// the memory a stuck-on carrier can cost, and — because a capped burst is
/// *dropped* rather than emitted — it also stops a continuously occupied channel
/// handing the decoders the same rubbish over and over.
const MAX_BURST_S: f64 = 0.25;

/// Pre-trigger history, in seconds. Long enough to recover the run-up on the
/// slowest channel in the plan.
const PRE_S: f64 = 0.002;

/// One captured transmission.
pub struct Burst {
    /// Baseband samples, pre-trigger history first.
    pub iq: Vec<Complex32>,
    pub rate_hz: f64,
    /// Absolute RF centre of the channel it came from.
    pub center_hz: f64,
    /// Peak block power against the channel's noise floor, dB.
    pub snr_db: f32,
    /// Peak block power in absolute dBFS.
    ///
    /// The comparable one when the same transmission is heard by two receivers:
    /// `snr_db` is referred to each channel's *own* learned floor, so two channels
    /// can report different SNRs for one signal and the larger need not be the one
    /// that heard it better. Absolute power has no such per-channel reference.
    pub peak_dbfs: f32,
}

/// Burst gate for one channel.
pub struct Gate {
    rate_hz: f64,
    center_hz: f64,
    /// Power ratio a block must exceed to open the gate.
    open_ratio: f32,

    /// Un-measured input, always fewer than `BLOCK` samples at rest.
    inbuf: Vec<Complex32>,
    /// Mean-noise-power estimate, already corrected by [`FLOOR_BIAS`].
    floor: f32,
    /// Raw min-tracking state, before the correction.
    tracked: f32,
    seeded: bool,

    /// Pre-trigger ring, and where the next sample goes.
    pre: Vec<Complex32>,
    pre_w: usize,
    pre_filled: usize,

    /// Samples of the burst in progress.
    cur: Vec<Complex32>,
    /// How much of `cur` is pre-trigger history rather than signal. The minimum
    /// length has to be judged on the signal alone — measuring `cur` instead
    /// lets a two-sample click past, because the history in front of it is
    /// longer than the minimum on its own.
    cur_pre: usize,
    open: bool,
    hang: u32,
    peak: f32,

    min_samples: usize,
    max_samples: usize,

    /// Bursts the gate has opened on, and bursts dropped for running past
    /// [`MAX_BURST_S`]. Reported so a channel sitting under a carrier looks
    /// like what it is rather than like a broken decoder.
    pub opened: u64,
    pub overlong: u64,
}

impl Gate {
    pub fn new(rate_hz: f64, center_hz: f64, threshold_db: f32) -> Gate {
        let pre = (PRE_S * rate_hz).round().max(BLOCK as f64) as usize;
        Gate {
            rate_hz,
            center_hz,
            open_ratio: 10f32.powf(threshold_db / 10.0),
            inbuf: Vec::with_capacity(BLOCK * 2),
            floor: 0.0,
            tracked: 0.0,
            seeded: false,
            pre: vec![Complex32::default(); pre],
            pre_w: 0,
            pre_filled: 0,
            cur: Vec::new(),
            cur_pre: 0,
            open: false,
            hang: 0,
            peak: 0.0,
            min_samples: (MIN_BURST_S * rate_hz) as usize,
            max_samples: (MAX_BURST_S * rate_hz) as usize,
            opened: 0,
            overlong: 0,
        }
    }

    /// Change the threshold without losing the learned floor — the noise did not
    /// move just because the operator dragged a slider.
    pub fn set_threshold_db(&mut self, db: f32) {
        self.open_ratio = 10f32.powf(db / 10.0);
    }

    /// Current noise-floor estimate as dBFS, for diagnostics.
    pub fn floor_dbfs(&self) -> f32 {
        10.0 * self.floor.max(1e-30).log10()
    }

    /// Feed baseband samples; append any completed bursts to `out`.
    pub fn push(&mut self, iq: &[Complex32], out: &mut Vec<Burst>) {
        // Lifted out of `self` for the loop so that `advance` can take `&mut
        // self`; the Vec's capacity comes back with it, so this is not an
        // allocation per call.
        let mut buf = std::mem::take(&mut self.inbuf);
        buf.extend_from_slice(iq);

        // Walk with an index and compact once, rather than draining per block:
        // draining inside the loop is quadratic in the block count.
        let mut pos = 0usize;
        while pos + BLOCK <= buf.len() {
            let block = &buf[pos..pos + BLOCK];
            let power = block.iter().map(|z| z.norm_sqr()).sum::<f32>() / BLOCK as f32;
            self.advance(block, power, out);
            pos += BLOCK;
        }
        if pos > 0 {
            buf.drain(..pos);
        }
        self.inbuf = buf;
    }

    fn advance(&mut self, block: &[Complex32], power: f32, out: &mut Vec<Burst>) {
        // The floor first, so the very first block is compared against
        // something. Seeding straight to the first measurement rather than
        // easing up from zero is what removes the need for a priming period.
        if !self.seeded {
            self.tracked = power;
            self.seeded = true;
        } else {
            let alpha = if power < self.tracked { FLOOR_FALL } else { FLOOR_RISE };
            self.tracked += alpha * (power - self.tracked);
        }
        self.floor = self.tracked * FLOOR_BIAS;

        let open_at = self.floor * self.open_ratio;

        if !self.open {
            self.push_pre(block);
            if power > open_at {
                self.open = true;
                self.hang = 0;
                self.peak = power;
                self.opened += 1;
                self.cur.clear();
                self.take_pre_into_cur();
                self.cur_pre = self.cur.len();
            }
            return;
        }

        self.cur.extend_from_slice(block);
        self.peak = self.peak.max(power);

        if power < open_at * CLOSE_RATIO {
            self.hang += 1;
        } else {
            self.hang = 0;
        }

        let too_long = self.cur.len() > self.max_samples;
        if self.hang < HANG_BLOCKS && !too_long {
            return;
        }

        self.open = false;
        // The hangover is silence by definition; leaving it on would drag the
        // decoders' own level estimates towards the noise.
        let keep = self.cur.len().saturating_sub(HANG_BLOCKS as usize * BLOCK);
        self.cur.truncate(keep);

        if too_long {
            self.overlong += 1;
        } else if self.cur.len().saturating_sub(self.cur_pre) >= self.min_samples {
            out.push(Burst {
                iq: std::mem::take(&mut self.cur),
                rate_hz: self.rate_hz,
                center_hz: self.center_hz,
                snr_db: 10.0 * (self.peak / self.floor.max(1e-30)).max(1e-30).log10(),
                peak_dbfs: 10.0 * self.peak.max(1e-30).log10(),
            });
        }
        self.cur.clear();
        // The pre-trigger history is now the tail of the burst just emitted;
        // dropping it stops the next burst being prefixed with the last one.
        self.pre_filled = 0;
        self.pre_w = 0;
    }

    fn push_pre(&mut self, block: &[Complex32]) {
        for &z in block {
            self.pre[self.pre_w] = z;
            self.pre_w = (self.pre_w + 1) % self.pre.len();
            self.pre_filled = (self.pre_filled + 1).min(self.pre.len());
        }
    }

    /// Copy the pre-trigger ring into the burst, oldest first.
    fn take_pre_into_cur(&mut self) {
        let n = self.pre_filled;
        let len = self.pre.len();
        let start = (self.pre_w + len - n) % len;
        for i in 0..n {
            self.cur.push(self.pre[(start + i) % len]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Deterministic noise — no `rand` in this tree.
    struct Rng(u64);

    impl Rng {
        fn next_f32(&mut self) -> f32 {
            self.0 ^= self.0 << 13;
            self.0 ^= self.0 >> 7;
            self.0 ^= self.0 << 17;
            ((self.0 >> 11) as f64 / (1u64 << 53) as f64) as f32
        }

        /// Box-Muller, so the noise really is Gaussian and `|z|²` really is
        /// exponential — which is the distribution `FLOOR_BIAS` was fitted to.
        fn normal(&mut self) -> f32 {
            let u1 = self.next_f32().max(1e-9);
            let u2 = self.next_f32();
            (-2.0 * u1.ln()).sqrt() * (std::f32::consts::TAU * u2).cos()
        }

        fn noise(&mut self, sigma: f32) -> Complex32 {
            Complex32::new(self.normal() * sigma, self.normal() * sigma)
        }
    }

    const RATE: f64 = 250_000.0;

    /// `FLOOR_BIAS` exists to correct min-tracking's downward bias, and the only
    /// honest way to set it is to measure it. Noise of a known power in, and the
    /// settled estimate has to come back within 1 dB.
    #[test]
    fn the_floor_estimate_lands_on_the_true_noise_power() {
        for sigma in [0.001f32, 0.01, 0.1] {
            let mut g = Gate::new(RATE, 868_300_000.0, 12.0);
            let mut rng = Rng(0x2545_F491_4F6C_DD1D);
            let mut out = Vec::new();
            // Half a second, long enough for the tracker to settle.
            let mut buf = vec![Complex32::default(); BLOCK * 64];
            for _ in 0..(RATE as usize / buf.len() / 2) {
                for z in buf.iter_mut() {
                    *z = rng.noise(sigma);
                }
                g.push(&buf, &mut out);
            }
            // Each component has variance sigma², so E|z|² = 2 sigma².
            let truth = 2.0 * sigma * sigma;
            let err_db = 10.0 * (g.floor / truth).log10();
            assert!(
                err_db.abs() < 1.0,
                "sigma {sigma}: floor is {err_db:.2} dB off ({} vs {truth})",
                g.floor
            );
            assert!(out.is_empty(), "noise alone opened the gate {} times", out.len());
        }
    }

    /// The gate must not fire on noise, at any of the thresholds the UI offers.
    #[test]
    fn noise_alone_never_produces_a_burst() {
        for thr in [6.0f32, 12.0, 20.0] {
            let mut g = Gate::new(RATE, 868_300_000.0, thr);
            let mut rng = Rng(0xDEAD_BEEF_1234_5678);
            let mut out = Vec::new();
            let mut buf = vec![Complex32::default(); 4096];
            for _ in 0..200 {
                for z in buf.iter_mut() {
                    *z = rng.noise(0.01);
                }
                g.push(&buf, &mut out);
            }
            assert!(out.is_empty(), "threshold {thr} dB fired {} times on noise", out.len());
        }
    }

    /// Build noise with one tone burst in the middle of it.
    fn noise_with_burst(
        rng: &mut Rng,
        total: usize,
        at: usize,
        len: usize,
        amp: f32,
        sigma: f32,
    ) -> Vec<Complex32> {
        (0..total)
            .map(|i| {
                let mut z = rng.noise(sigma);
                if i >= at && i < at + len {
                    let ph = std::f32::consts::TAU * 20_000.0 * i as f32 / RATE as f32;
                    z += Complex32::new(amp * ph.cos(), amp * ph.sin());
                }
                z
            })
            .collect()
    }

    /// A burst has to come out whole, and it has to start *before* the point the
    /// power actually rose — that is what the pre-trigger ring is for, and a
    /// decoder without it loses the preamble it recovers timing from.
    #[test]
    fn a_burst_is_captured_with_history_in_front_of_it() {
        let mut g = Gate::new(RATE, 868_300_000.0, 12.0);
        let mut rng = Rng(0x1234_5678_9ABC_DEF0);
        let mut out = Vec::new();

        // Let the floor settle on noise first.
        let mut warm = vec![Complex32::default(); 8192];
        for _ in 0..40 {
            for z in warm.iter_mut() {
                *z = rng.noise(0.01);
            }
            g.push(&warm, &mut out);
        }
        assert!(out.is_empty());

        // 5 ms of signal, well clear of the noise.
        let len = (0.005 * RATE) as usize;
        let at = 4096;
        let sig = noise_with_burst(&mut rng, 40_000, at, len, 0.2, 0.01);
        g.push(&sig, &mut out);
        // Trailing quiet so the gate shuts.
        for _ in 0..4 {
            for z in warm.iter_mut() {
                *z = rng.noise(0.01);
            }
            g.push(&warm, &mut out);
        }

        assert_eq!(out.len(), 1, "expected exactly one burst, got {}", out.len());
        let b = &out[0];
        assert!(
            b.iq.len() >= len,
            "burst is {} samples, shorter than the {len} that were transmitted",
            b.iq.len()
        );
        // The extra is the pre-trigger plus at most the detection lag, not a
        // runaway.
        assert!(b.iq.len() < len + (PRE_S * RATE) as usize + 4 * BLOCK, "{}", b.iq.len());
        assert!(b.snr_db > 15.0, "SNR came out {:.1} dB", b.snr_db);
        assert_eq!(b.center_hz, 868_300_000.0);

        // The leading samples must be the quiet run-up, not the signal: that is
        // the pre-trigger doing its job.
        let head: f32 = b.iq[..BLOCK].iter().map(|z| z.norm_sqr()).sum::<f32>() / BLOCK as f32;
        let mid_at = b.iq.len() / 2;
        let mid: f32 =
            b.iq[mid_at..mid_at + BLOCK].iter().map(|z| z.norm_sqr()).sum::<f32>() / BLOCK as f32;
        assert!(head < mid / 10.0, "no quiet run-up: head {head}, middle {mid}");
    }

    /// Two transmissions must not merge, and the second must not be prefixed
    /// with the tail of the first.
    #[test]
    fn two_bursts_stay_two_bursts() {
        let mut g = Gate::new(RATE, 868_300_000.0, 12.0);
        let mut rng = Rng(0x0F1E_2D3C_4B5A_6978);
        let mut out = Vec::new();
        let mut warm = vec![Complex32::default(); 8192];
        for _ in 0..40 {
            for z in warm.iter_mut() {
                *z = rng.noise(0.01);
            }
            g.push(&warm, &mut out);
        }

        let len = (0.004 * RATE) as usize;
        let total = 60_000;
        let mut sig = noise_with_burst(&mut rng, total, 2_000, len, 0.2, 0.01);
        // Second burst, well separated.
        let second = 30_000;
        for i in second..second + len {
            let ph = std::f32::consts::TAU * 20_000.0 * i as f32 / RATE as f32;
            sig[i] += Complex32::new(0.2 * ph.cos(), 0.2 * ph.sin());
        }
        g.push(&sig, &mut out);
        for _ in 0..4 {
            for z in warm.iter_mut() {
                *z = rng.noise(0.01);
            }
            g.push(&warm, &mut out);
        }

        assert_eq!(out.len(), 2, "expected two bursts, got {}", out.len());
        for b in &out {
            assert!(b.iq.len() < len * 3, "burst ran long at {} samples", b.iq.len());
        }
    }

    /// A channel sitting under a continuous carrier must cost one gate open and
    /// then nothing — not a stream of quarter-second buffers for the decoders to
    /// chew through.
    #[test]
    fn a_continuous_carrier_is_dropped_rather_than_emitted() {
        let mut g = Gate::new(RATE, 868_300_000.0, 12.0);
        let mut rng = Rng(0xAAAA_5555_CCCC_3333);
        let mut out = Vec::new();
        let mut warm = vec![Complex32::default(); 8192];
        for _ in 0..40 {
            for z in warm.iter_mut() {
                *z = rng.noise(0.01);
            }
            g.push(&warm, &mut out);
        }

        // Two seconds of unbroken carrier.
        let mut buf = vec![Complex32::default(); 8192];
        let mut n = 0usize;
        for _ in 0..((2.0 * RATE) as usize / buf.len()) {
            for z in buf.iter_mut() {
                let ph = std::f32::consts::TAU * 20_000.0 * n as f32 / RATE as f32;
                *z = rng.noise(0.01) + Complex32::new(0.2 * ph.cos(), 0.2 * ph.sin());
                n += 1;
            }
            g.push(&buf, &mut out);
        }

        assert!(out.is_empty(), "a carrier produced {} bursts", out.len());
        assert!(g.overlong > 0, "the over-long counter never moved");
    }

    /// Impulse noise is not a transmission.
    #[test]
    fn a_click_is_too_short_to_report() {
        let mut g = Gate::new(RATE, 868_300_000.0, 12.0);
        let mut rng = Rng(0x1111_2222_3333_4444);
        let mut out = Vec::new();
        let mut warm = vec![Complex32::default(); 8192];
        for _ in 0..40 {
            for z in warm.iter_mut() {
                *z = rng.noise(0.01);
            }
            g.push(&warm, &mut out);
        }

        // 100 µs, a quarter of the minimum.
        let len = (0.0001 * RATE) as usize;
        let sig = noise_with_burst(&mut rng, 20_000, 2_000, len, 0.3, 0.01);
        g.push(&sig, &mut out);
        for _ in 0..4 {
            for z in warm.iter_mut() {
                *z = rng.noise(0.01);
            }
            g.push(&warm, &mut out);
        }

        assert!(out.is_empty(), "a {len}-sample click was reported as a burst");
    }
}
