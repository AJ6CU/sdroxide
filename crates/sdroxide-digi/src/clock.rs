//! What the decoders' `dt` values say about our own slot timing.
//!
//! Every decode carries `dt`: where a station's transmission actually started,
//! measured against where our clock said the slot began. Individual stations
//! scatter, but they scatter around *our* error rather than around zero, so the
//! median across a slot reads our offset straight off the band. It costs
//! nothing — the decoder has already computed every one of those numbers — and
//! it catches the commonest reason a station calls all evening and is never
//! answered.

use std::collections::VecDeque;

use sdroxide_types::Decode;

/// Slot medians kept for the reported estimate. Eight FT8 slots is two minutes:
/// long enough to ride out a slot where one loud off-timing station dominates,
/// short enough to notice a clock being corrected.
const SLOTS: usize = 8;
/// A slot with fewer decodes than this says more about one station's timing
/// than about ours, so it is not counted at all.
const MIN_DECODES: usize = 3;

#[derive(Default)]
pub struct ClockMonitor {
    /// Per-slot medians, oldest first.
    recent: VecDeque<f32>,
}

impl ClockMonitor {
    pub fn new() -> Self {
        Self::default()
    }

    /// Fold in one slot's decodes. Returns true if the estimate changed.
    pub fn observe(&mut self, decodes: &[Decode]) -> bool {
        self.observe_dts(decodes.iter().map(|d| d.dt))
    }

    /// Fold in one slot's timing offsets directly.
    ///
    /// WSPR reaches this way: its decodes are [`sdroxide_types::WsprSpot`]s
    /// rather than [`Decode`]s, and `dt` is the only field the estimate reads.
    /// On a quiet band a WSPR slot rarely carries [`MIN_DECODES`] beacons, so
    /// the estimate stays `None` for long stretches — which is the truth, and
    /// better than a number drawn from one station's oscillator.
    pub fn observe_dts(&mut self, dts: impl IntoIterator<Item = f32>) -> bool {
        let mut dts: Vec<f32> = dts.into_iter().collect();
        if dts.len() < MIN_DECODES {
            return false;
        }
        let before = self.offset_s();
        self.recent.push_back(median(&mut dts));
        while self.recent.len() > SLOTS {
            self.recent.pop_front();
        }
        self.offset_s() != before
    }

    /// Our timing offset in seconds — positive means our clock runs ahead of
    /// the stations we hear. `None` until a slot with enough decodes has been
    /// heard.
    ///
    /// The median of the per-slot medians, so neither one crowded slot nor one
    /// badly-timed station moves it.
    pub fn offset_s(&self) -> Option<f32> {
        if self.recent.is_empty() {
            return None;
        }
        let mut v: Vec<f32> = self.recent.iter().copied().collect();
        Some(median(&mut v))
    }
}

/// Median of `v`, which it sorts in place. `v` must not be empty.
fn median(v: &mut [f32]) -> f32 {
    v.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = v.len();
    if n % 2 == 1 { v[n / 2] } else { 0.5 * (v[n / 2 - 1] + v[n / 2]) }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot(dts: &[f32]) -> Vec<Decode> {
        dts.iter()
            .map(|&dt| Decode {
                slot_utc: 0,
                snr_db: -10,
                dt,
                audio_hz: 1500.0,
                message: "CQ AB1CD FN42".into(),
                to: None,
                from: Some("AB1CD".into()),
                grid: None,
                is_cq: true,
                cq_to: None,
                free_text: false,
                rr73_to: None,
            })
            .collect()
    }

    #[test]
    fn nothing_is_claimed_until_a_slot_says_something() {
        let mut c = ClockMonitor::new();
        assert_eq!(c.offset_s(), None);
        // Two decodes is one station's opinion, not a measurement.
        assert!(!c.observe(&slot(&[0.4, 0.5])));
        assert_eq!(c.offset_s(), None);
    }

    #[test]
    fn the_median_reads_our_offset_off_the_band() {
        let mut c = ClockMonitor::new();
        assert!(c.observe(&slot(&[0.38, 0.41, 0.44, 0.40, 0.39])));
        assert_eq!(c.offset_s(), Some(0.40));
    }

    #[test]
    fn one_badly_timed_station_does_not_move_it() {
        let mut c = ClockMonitor::new();
        c.observe(&slot(&[0.05, -0.02, 0.01, 2.40, -0.03]));
        let off = c.offset_s().expect("measured");
        assert!(off.abs() < 0.1, "a single 2.4 s outlier dragged the estimate to {off}");
    }

    #[test]
    fn the_estimate_follows_a_corrected_clock() {
        let mut c = ClockMonitor::new();
        for _ in 0..SLOTS {
            c.observe(&slot(&[1.9, 2.0, 2.1]));
        }
        assert_eq!(c.offset_s(), Some(2.0));
        // The operator fixes the clock; the old slots age out of the window.
        for _ in 0..SLOTS {
            c.observe(&slot(&[-0.1, 0.0, 0.1]));
        }
        assert_eq!(c.offset_s(), Some(0.0));
    }
}
