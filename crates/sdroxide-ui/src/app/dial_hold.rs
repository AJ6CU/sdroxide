//! The dial and the front-end centre this screen has just asked for, held
//! against the engine's snapshots that have not caught up with them yet.
//!
//! Every route that tunes from the screen — a key, the wheel, the readout's
//! digits, CTR's recentring — writes the new value into the local state at once
//! and sends the command. `RadioEvent::State` is the engine's whole snapshot,
//! and the one that arrives next is often its answer to an *earlier* command,
//! so adopting it as it stands puts the old dial and the old centre back for a
//! frame or two before the right ones land.
//!
//! With CTR lit that is visible: the panadapter recentres on the old dial, asks
//! the front end to move back, then forward again when the next snapshot comes
//! — the waterfall jumps, flickers and shakes for as long as the keys are
//! pressed, and settles the moment they stop. A tuning drag already keeps its
//! own dial against this (see `gesture-dial` in the panadapter); this is the
//! same protection for every other route, in the one place every command
//! leaves by.
//!
//! The DAB settings are held the same way, whole. They are edited here and
//! sent back as one value, so a snapshot from before an edit put the old list
//! back on screen — a channel cleared with × reappeared at once — and the next
//! settings sent from the screen wrote it back at the station for good.
//!
//! The engine still has the last word. A value it never echoes — a tune it
//! refused, a centre its downconverter clamped — is held only for
//! [`DIAL_HOLD_S`] from the first snapshot that disagreed, and then the
//! engine's value is taken.

use sdroxide_types::{Command, DabSettings, RadioState, Vfo};

/// How long a value this screen sent outranks a snapshot that disagrees with
/// it, counted from the first such snapshot. Long enough for a reply to cross a
/// network to a remote station; short enough that a tune the engine refused
/// snaps back without anyone wondering why.
pub(in crate::app) const DIAL_HOLD_S: f64 = 0.5;

/// The same for the DAB settings. Longer, because the engine never refuses
/// them — a snapshot that disagrees is always one still on its way — and a
/// remote station's replies can queue behind its audio and spectrum.
const DAB_HOLD_S: f64 = 3.0;

/// Within this, a snapshot is taken to have caught up with what was sent.
const SAME_HZ: f64 = 0.5;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Pending {
    hz: f64,
    /// When the first snapshot that disagreed arrived; `None` until one has.
    since: Option<f64>,
}

/// See the module docs.
#[derive(Clone, Debug, Default)]
pub(in crate::app) struct DialHold {
    vfo: [Option<Pending>; 2],
    center: Option<Pending>,
    /// The DAB settings last sent, and when a snapshot first disagreed.
    dab: Option<(DabSettings, Option<f64>)>,
}

impl DialHold {
    /// Note a command on its way out. Only the dial and the centre are held;
    /// everything else passes untouched.
    pub(in crate::app) fn note(&mut self, cmd: &Command) {
        let fresh = |hz: f64| Some(Pending { hz, since: None });
        match *cmd {
            Command::SetVfo { vfo, hz } => self.vfo[slot(vfo)] = fresh(hz),
            Command::SetCenter(hz) => self.center = fresh(hz),
            Command::SetDabConfig(ref d) => self.dab = Some((d.clone(), None)),
            _ => {}
        }
    }

    /// Lay what this screen sent over an incoming snapshot, field by field,
    /// for as long as the snapshot has not caught up and the hold has not run
    /// out.
    pub(in crate::app) fn apply(&mut self, s: &mut RadioState, now: f64) {
        hold(&mut self.vfo[0], &mut s.vfo_a_hz, now);
        hold(&mut self.vfo[1], &mut s.vfo_b_hz, now);
        hold(&mut self.center, &mut s.center_hz, now);
        if let Some((sent, since)) = &mut self.dab {
            if s.dab == *sent {
                self.dab = None;
            } else if now - *since.get_or_insert(now) < DAB_HOLD_S {
                s.dab = sent.clone();
            } else {
                self.dab = None;
            }
        }
    }
}

fn slot(vfo: Vfo) -> usize {
    match vfo {
        Vfo::A => 0,
        Vfo::B => 1,
    }
}

fn hold(pending: &mut Option<Pending>, field: &mut f64, now: f64) {
    let Some(p) = pending else { return };
    if (*field - p.hz).abs() < SAME_HZ {
        // Caught up: the engine agrees, and from here its word stands alone.
        *pending = None;
        return;
    }
    let since = *p.since.get_or_insert(now);
    if now - since < DIAL_HOLD_S {
        *field = p.hz;
    } else {
        // Never echoed: refused or clamped. The engine's value wins.
        *pending = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn state(vfo: f64, center: f64) -> RadioState {
        RadioState { vfo_a_hz: vfo, center_hz: center, ..RadioState::default() }
    }

    #[test]
    fn a_snapshot_behind_the_dial_does_not_pull_it_back() {
        let mut h = DialHold::default();
        h.note(&Command::SetVfo { vfo: Vfo::A, hz: 7_100_100.0 });
        h.note(&Command::SetCenter(7_100_100.0));
        let mut s = state(7_100_000.0, 7_000_000.0);
        h.apply(&mut s, 10.0);
        assert_eq!((s.vfo_a_hz, s.center_hz), (7_100_100.0, 7_100_100.0));
    }

    #[test]
    fn once_the_engine_agrees_its_word_stands_alone() {
        let mut h = DialHold::default();
        h.note(&Command::SetVfo { vfo: Vfo::A, hz: 7_100_100.0 });
        let mut s = state(7_100_100.0, 7_000_000.0);
        h.apply(&mut s, 10.0);
        // Another client moves the dial afterwards: nothing is held any more.
        let mut s = state(7_200_000.0, 7_000_000.0);
        h.apply(&mut s, 10.1);
        assert_eq!(s.vfo_a_hz, 7_200_000.0);
    }

    #[test]
    fn a_value_the_engine_never_echoes_is_given_up() {
        let mut h = DialHold::default();
        h.note(&Command::SetVfo { vfo: Vfo::A, hz: 7_100_100.0 });
        let mut s = state(7_100_000.0, 7_000_000.0);
        h.apply(&mut s, 10.0);
        assert_eq!(s.vfo_a_hz, 7_100_100.0, "held while the reply may be in flight");
        let mut s = state(7_100_000.0, 7_000_000.0);
        h.apply(&mut s, 10.0 + DIAL_HOLD_S);
        assert_eq!(s.vfo_a_hz, 7_100_000.0, "a refused tune snaps back");
    }

    #[test]
    fn only_the_vfo_that_was_tuned_is_held() {
        let mut h = DialHold::default();
        h.note(&Command::SetVfo { vfo: Vfo::B, hz: 14_074_000.0 });
        let mut s = state(7_100_000.0, 7_000_000.0);
        s.vfo_b_hz = 14_000_000.0;
        h.apply(&mut s, 1.0);
        assert_eq!((s.vfo_a_hz, s.vfo_b_hz, s.center_hz), (7_100_000.0, 14_074_000.0, 7_000_000.0));
    }
}
