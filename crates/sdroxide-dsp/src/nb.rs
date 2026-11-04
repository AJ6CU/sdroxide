//! Simple impulse noise blanker: zero out samples whose magnitude spikes
//! far above the running average (ignition noise, power-line crackle).
//!
//! It also counts what it does. A blanker that finds nothing to blank and one
//! that never fires look identical from the outside — both leave the noise
//! alone — so the count and the strongest sample's distance from the bar are
//! what a run on a given receiver actually says (issue #421).

use crate::Complex32;

const THRESHOLD_FACTOR: f32 = 5.0;
/// Samples of running-average settling before blanking starts.
const WARMUP: u32 = 8_192;

/// What the blanker did over a stretch of samples: one report per second while
/// it is on, so a receiver that blanks nothing says so.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct NbReport {
    /// Samples zeroed as impulses.
    pub blanked: u64,
    /// Samples offered to the blanker, after the warmup.
    pub offered: u64,
    /// The strongest sample seen, as a multiple of the running mean. The bar is
    /// [`NoiseBlanker::bar`], so this says whether the threshold was near or
    /// far — the number that decides whether to lower it or to leave it be.
    pub peak_ratio: f32,
    /// The running mean at the end of the stretch, in the samples' own units.
    pub mean: f32,
}

impl NbReport {
    /// The share of offered samples that were blanked, 0.0 when none were
    /// offered at all.
    pub fn share(&self) -> f32 {
        if self.offered == 0 { 0.0 } else { self.blanked as f32 / self.offered as f32 }
    }
}

pub struct NoiseBlanker {
    mean: f32,
    seen: u32,
    blanked: u64,
    offered: u64,
    peak_ratio: f32,
}

impl NoiseBlanker {
    pub fn new() -> Self {
        NoiseBlanker { mean: 0.0, seen: 0, blanked: 0, offered: 0, peak_ratio: 0.0 }
    }

    /// How far above the running mean a sample has to sit before it is blanked.
    pub fn bar() -> f32 {
        THRESHOLD_FACTOR
    }

    pub fn process(&mut self, samples: &mut [Complex32]) {
        for s in samples {
            let mag = s.norm();
            if self.seen < WARMUP {
                self.mean += 5e-4 * (mag - self.mean);
                self.seen = self.seen.saturating_add(1);
                continue;
            }
            self.offered += 1;
            if self.mean > 0.0 {
                let ratio = mag / self.mean;
                if ratio > self.peak_ratio {
                    self.peak_ratio = ratio;
                }
            }
            // Track the mean of ordinary samples only, so impulses don't
            // raise their own threshold.
            if mag < THRESHOLD_FACTOR * self.mean {
                self.mean += 5e-4 * (mag - self.mean);
                self.seen = self.seen.saturating_add(1);
            } else {
                *s = Complex32::default();
                self.blanked += 1;
            }
        }
    }

    /// What happened since the previous call, which starts a new count. The
    /// running mean is not reset: it is the blanker's state, not a statistic.
    pub fn read(&mut self) -> NbReport {
        let report = NbReport {
            blanked: self.blanked,
            offered: self.offered,
            peak_ratio: self.peak_ratio,
            mean: self.mean,
        };
        self.blanked = 0;
        self.offered = 0;
        self.peak_ratio = 0.0;
        report
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A tone at 0.1 with an impulse every ten thousand samples.
    fn stream(n: usize) -> Vec<Complex32> {
        (0..n)
            .map(|i| {
                let ph = std::f32::consts::TAU * 0.01 * i as f32;
                let mut z = Complex32::new(0.1 * ph.cos(), 0.1 * ph.sin());
                if i > WARMUP as usize && i % 10_000 == 0 {
                    z *= 100.0; // impulse
                }
                z
            })
            .collect()
    }

    #[test]
    fn blanks_impulses_keeps_signal() {
        let mut nb = NoiseBlanker::new();
        let mut buf = stream(60_000);
        nb.process(&mut buf);

        let peak_after = buf[WARMUP as usize..].iter().fold(0.0f32, |a, z| a.max(z.norm()));
        assert!(peak_after < 0.2, "impulse survived: {peak_after}");
        // Ordinary signal untouched.
        let idx = WARMUP as usize + 5_000;
        assert!((buf[idx].norm() - 0.1).abs() < 0.01);
    }

    /// The count says how many samples went and how close the bar came — the
    /// two numbers that tell a receiver's owner which case they are in.
    #[test]
    fn the_report_counts_what_was_blanked() {
        let mut nb = NoiseBlanker::new();
        let mut buf = stream(60_000);
        nb.process(&mut buf);

        let r = nb.read();
        assert!(r.offered > 0, "nothing was offered after the warmup");
        assert!(r.blanked > 0, "the impulses went uncounted: {r:?}");
        assert!(r.share() > 0.0 && r.share() < 0.01, "share {r:?}");
        assert!(
            r.peak_ratio > NoiseBlanker::bar(),
            "the strongest sample must be over the bar it fired on: {r:?}"
        );
    }

    /// No impulses: nothing blanked, and the peak ratio says how far short of
    /// the bar the noise came. This is the KiwiSDR case the count was added for.
    #[test]
    fn a_clean_stream_blanks_nothing_and_says_how_close_it_came() {
        let mut nb = NoiseBlanker::new();
        let mut buf: Vec<Complex32> = (0..60_000)
            .map(|i| {
                let ph = std::f32::consts::TAU * 0.01 * i as f32;
                Complex32::new(0.1 * ph.cos(), 0.1 * ph.sin())
            })
            .collect();
        nb.process(&mut buf);

        let r = nb.read();
        assert!(r.offered > 0, "nothing was offered after the warmup");
        assert_eq!(r.blanked, 0, "{r:?}");
        assert!(r.peak_ratio > 0.0, "a peak has to be measured: {r:?}");
        assert!(r.peak_ratio < NoiseBlanker::bar(), "{r:?}");
    }

    /// A report starts a new count, so each second's number is that second's.
    #[test]
    fn reading_starts_a_new_count() {
        let mut nb = NoiseBlanker::new();
        let mut buf = stream(60_000);
        nb.process(&mut buf);
        let first = nb.read();
        assert!(first.offered > 0);

        let second = nb.read();
        assert_eq!((second.offered, second.blanked), (0, 0), "{second:?}");
    }
}
