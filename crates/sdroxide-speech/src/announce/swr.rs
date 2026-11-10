//! Reading SWR out while tuning up.
//!
//! Tuning an antenna is the one thing in this program that genuinely cannot be
//! done without seeing the meter — you turn a knob and watch a needle. So this
//! reads the needle: every two seconds while TUNE is held, with a warning that
//! interrupts if the match goes bad, and the best figure reached when you let
//! go.

use sdroxide_types::Meters;

use crate::queue::{Key, Priority, Utterance};
use crate::text::Speaker;

/// Ignore the first moments after key-up: the PA is still ramping and the
/// bridge has not settled, so the reading there is not merely noisy but
/// alarming.
pub const SWR_SETTLE_S: f64 = 0.8;

/// One-pole smoothing time constant over meter frames.
pub const SWR_TC_S: f32 = 0.30;

/// Latch the warning at or above this. Three to one is where a solid-state PA
/// starts folding back, which is not a matter of taste.
pub const SWR_ALARM_DEFAULT: f32 = 3.0;

/// Release it only below this...
pub const SWR_CLEAR: f32 = 2.5;

/// ...and only once it has held there this long.
pub const SWR_CLEAR_HOLD_S: f64 = 1.0;

/// At or above this, say "over ten to one". Bridges peg, and a precise figure
/// up there is not information.
pub const SWR_PEG: f32 = 9.9;

/// Watches the transmit meters and decides what to say about them.
#[derive(Debug, Default)]
pub struct SwrWatch {
    /// When the next periodic reading is due; `None` when not tuning.
    next_at: Option<f64>,
    /// Nothing is believed before this instant after key-up.
    live_at: f64,
    /// Smoothed reading, once there is one.
    smoothed: Option<f32>,
    /// When the last meter frame arrived, for the smoothing step.
    last_meter_at: Option<f64>,
    /// The best (lowest) reading this tune — the figure that means something
    /// after an ATU sweep.
    best: Option<f32>,
    /// Said "no S W R reading" once already this tune.
    said_unavailable: bool,
    /// The warning is latched.
    alarm: bool,
    /// When the reading first went below the clear threshold.
    below_since: Option<f64>,
    /// Edge detector for `tune`.
    was_tuning: bool,
}

impl SwrWatch {
    pub fn new() -> Self {
        Self::default()
    }

    /// Fold in one meter update.
    ///
    /// `keyed` is PTT or TUNE; `tuning` is TUNE alone. The warning is checked
    /// whether or not we are tuning, because a runaway SWR during an ordinary
    /// transmission is the dangerous case.
    #[allow(clippy::too_many_arguments)]
    pub fn on_meters(
        &mut self,
        m: &Meters,
        keyed: bool,
        now: f64,
        sp: &Speaker<'_>,
        alarm_at: f32,
        alarm_always: bool,
        tuning: bool,
        out: &mut Vec<Utterance>,
    ) {
        if !keyed || now < self.live_at {
            return;
        }
        let Some(v) = m.tx.as_ref().and_then(|t| t.swr) else {
            return;
        };
        if !v.is_finite() {
            return;
        }

        let dt = self.last_meter_at.map_or(SWR_TC_S as f64, |t| now - t) as f32;
        self.last_meter_at = Some(now);
        let a = (dt / SWR_TC_S).clamp(0.0, 1.0);
        let s = match self.smoothed {
            Some(prev) => prev + a * (v - prev),
            None => v,
        };
        self.smoothed = Some(s);
        self.best = Some(self.best.map_or(s, |b| b.min(s)));

        if !(alarm_always || tuning) {
            return;
        }
        if !self.alarm && s >= alarm_at {
            self.alarm = true;
            self.below_since = None;
            out.push(
                Utterance::new(format!("high S W R, {}", sp.swr(s)), Priority::Alert, now)
                    .key(Key::SwrAlarm)
                    .interrupting(),
            );
            // Replace this period's routine reading rather than stacking on it.
            if self.next_at.is_some() {
                self.next_at = Some(now + sp.cfg.swr_period_s());
            }
        } else if self.alarm && s < SWR_CLEAR {
            let since = *self.below_since.get_or_insert(now);
            if now - since >= SWR_CLEAR_HOLD_S {
                self.alarm = false;
                self.below_since = None;
                out.push(Utterance::new("S W R normal", Priority::Notable, now).key(Key::SwrAlarm));
            }
        } else if s >= SWR_CLEAR {
            self.below_since = None;
        }
    }

    /// Run the edges and the two-second period. Call once per tick.
    pub fn tick(
        &mut self,
        tuning: bool,
        m: Option<&Meters>,
        tune_drive: f32,
        now: f64,
        sp: &Speaker<'_>,
        out: &mut Vec<Utterance>,
    ) {
        let cfg = sp.cfg;
        if tuning && !self.was_tuning {
            // A fresh tune is a fresh measurement, possibly of a different
            // antenna: everything resets, including a latched warning.
            *self = SwrWatch {
                next_at: Some(now + SWR_SETTLE_S),
                live_at: now + SWR_SETTLE_S,
                was_tuning: true,
                ..SwrWatch::default()
            };
            return;
        }
        if !tuning && self.was_tuning {
            self.was_tuning = false;
            self.next_at = None;
            if cfg.tune.summary_after_tune
                && let Some(best) = self.best
            {
                // The minimum, not the last sample: after an ATU sweep the
                // last value is wherever the sweep happened to stop.
                out.push(
                    Utterance::new(format!("tuned, {}", sp.swr(best)), Priority::Notable, now)
                        .key(Key::SwrTick),
                );
            }
            return;
        }
        self.was_tuning = tuning;

        if !tuning || !cfg.tune.swr_while_tuning {
            return;
        }
        let Some(due) = self.next_at else { return };
        if now < due {
            return;
        }
        self.next_at = Some(now + cfg.swr_period_s());

        if let Some(v) = self.smoothed {
            out.push(
                Utterance::new(sp.swr(v), Priority::Notable, now)
                    .key(Key::SwrTick)
                    .ttl(cfg.swr_period_s())
                    .interrupting(),
            );
            return;
        }

        // Keyed and reporting, but with no SWR figure in it: the rig has no
        // bridge. Say so once, then stay quiet — repeating it every two seconds
        // for the length of a tune-up is worse than silence.
        let reporting = m.and_then(|m| m.tx.as_ref()).is_some();
        if reporting && !self.said_unavailable {
            self.said_unavailable = true;
            let fwd = m.and_then(|m| m.tx.as_ref()).and_then(|t| t.fwd_w);
            let text = match fwd {
                Some(w) => format!("no S W R reading, {} forward", sp.watts(w)),
                None => format!("no S W R reading, tuning at {}", sp.percent(tune_drive)),
            };
            out.push(Utterance::new(text, Priority::Routine, now).key(Key::SwrTick));
        }
    }

    /// When the next periodic reading is due, for the repaint request.
    pub fn deadline(&self) -> Option<f64> {
        self.next_at
    }

    /// The latched warning state, for a caller that wants to show it.
    pub fn alarm(&self) -> bool {
        self.alarm
    }
}
