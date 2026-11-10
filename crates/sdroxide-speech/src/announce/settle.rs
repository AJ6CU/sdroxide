//! "Has it stopped moving yet?"
//!
//! A dial that is still turning is not a frequency. The engine broadcasts a
//! full state snapshot on every change, so a scroll produces thirty a second,
//! and announcing each one is useless noise. Continuous quantities therefore
//! go through a settle latch: record the newest value on every event, and speak
//! only once it has held still for a while.
//!
//! The clock is passed in as an `f64` — egui's monotonic frame time — rather
//! than read from `Instant`. That is what makes the announcer deterministic
//! under test: a case drives forty snapshots thirty milliseconds apart and
//! asserts exactly one utterance came out.

/// A dial that is still moving is not a frequency yet. Scroll detents arrive
/// 30–60 ms apart during a turn, so 600 ms of stillness means the operator has
/// stopped — and it is short enough that the read-out still feels like a
/// response rather than an afterthought.
pub const SETTLE_FREQ_S: f64 = 0.60;

/// Sliders and encoders: drive, mic, volume, the AGC ceiling, manual gain,
/// squelch, filter width. Shorter than the dial, because a fader is released
/// decisively and these are one short phrase.
pub const SETTLE_LEVEL_S: f64 = 0.45;

/// RIT and XIT offsets: the same feel as the dial, because that is what they
/// are.
pub const SETTLE_OFFSET_S: f64 = 0.60;

/// The band-edge check. Shorter, because being outside a band is not something
/// to learn about a second late — but not zero, or tuning through 7.200 would
/// chatter on the way past.
pub const SETTLE_OOB_S: f64 = 0.35;

/// One field's settle latch.
#[derive(Debug, Clone)]
pub struct Settle<T> {
    /// What was last announced. `None` before the first seed.
    spoken: Option<T>,
    /// The newest value seen, and when it last differed from the one before.
    pending: Option<(T, f64)>,
    quiet_s: f64,
}

impl<T: Copy + PartialEq> Settle<T> {
    pub fn new(quiet_s: f64) -> Self {
        Settle { spoken: None, pending: None, quiet_s }
    }

    /// Adopt `v` as already-announced, without saying it.
    ///
    /// Used at startup and after a reconnect. Without this, attaching to a
    /// radio makes it recite its entire configuration.
    pub fn seed(&mut self, v: T) {
        self.spoken = Some(v);
        self.pending = None;
    }

    /// Record the newest value. Cheap; call on every state event.
    ///
    /// The quiet window restarts only when the value actually differs, so a
    /// stream of identical snapshots lets the timer run down.
    ///
    /// A value equal to what was last announced clears the latch outright. That
    /// is not only an optimisation: [`Settle::deadline`] feeds the caller's
    /// repaint request, and without it every snapshot would schedule a wake-up
    /// for all twelve latches whether or not anything had moved.
    pub fn observe(&mut self, v: T, now: f64) {
        if self.spoken == Some(v) {
            self.pending = None;
            return;
        }
        match self.pending {
            Some((p, _)) if p == v => {}
            _ => self.pending = Some((v, now)),
        }
    }

    /// `Some(v)` exactly once, when `v` has held still for `quiet_s` and
    /// differs from what was last announced.
    pub fn poll(&mut self, now: f64) -> Option<T> {
        let (v, since) = self.pending?;
        if now - since < self.quiet_s {
            return None;
        }
        self.pending = None;
        // Spin the dial up two kilohertz and back down again and nothing is
        // said, because the frequency settled on is the one already announced.
        if self.spoken == Some(v) {
            return None;
        }
        self.spoken = Some(v);
        Some(v)
    }

    /// The value last announced.
    pub fn current(&self) -> Option<T> {
        self.spoken
    }

    /// When `poll` could next fire, for the repaint request.
    pub fn deadline(&self) -> Option<f64> {
        self.pending.map(|(_, since)| since + self.quiet_s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_moving_value_says_nothing() {
        let mut s = Settle::new(0.6);
        s.seed(0i64);
        // Forty detents, thirty milliseconds apart.
        for i in 1..=40 {
            let t = i as f64 * 0.03;
            s.observe(i, t);
            assert_eq!(s.poll(t), None, "spoke mid-scroll at {t}");
        }
        // ...then the operator lets go.
        assert_eq!(s.poll(40.0 * 0.03 + 0.61), Some(40));
    }

    #[test]
    fn settling_back_where_you_started_says_nothing() {
        let mut s = Settle::new(0.6);
        s.seed(100i64);
        s.observe(102, 0.0);
        s.observe(100, 0.1);
        assert_eq!(s.poll(0.8), None);
    }

    #[test]
    fn it_speaks_once_and_only_once() {
        let mut s = Settle::new(0.5);
        s.seed(1i64);
        s.observe(2, 0.0);
        assert_eq!(s.poll(0.6), Some(2));
        assert_eq!(s.poll(0.7), None);
        assert_eq!(s.poll(10.0), None);
    }

    #[test]
    fn an_unseeded_latch_speaks_its_first_value() {
        let mut s: Settle<i64> = Settle::new(0.5);
        s.observe(7, 0.0);
        assert_eq!(s.poll(0.6), Some(7));
    }

    #[test]
    fn the_deadline_tracks_the_pending_value() {
        let mut s = Settle::new(0.6);
        s.seed(0i64);
        assert_eq!(s.deadline(), None);
        s.observe(1, 1.0);
        assert_eq!(s.deadline(), Some(1.6));
        // An identical observation does not push the deadline out.
        s.observe(1, 1.3);
        assert_eq!(s.deadline(), Some(1.6));
        // A different one restarts it.
        s.observe(2, 1.4);
        assert_eq!(s.deadline(), Some(2.0));
    }
}
