//! The session ignore list for the FT8/FT4/FT2 decode list.
//!
//! A **−** button on each decode mutes that station for the rest of the
//! session: their decodes stop arriving in the list, stop sounding the audible
//! alerts, stop being read out by the announcer, and stop being offered to auto
//! mode as a station to answer.
//!
//! **Session-only, on purpose.** It is a `HashSet` in the app with no file, no
//! wire type and no schema. A memory list is cheaper to run and cheaper to
//! build than a persisted one, and — the reason that matters — it cannot leave
//! a station permanently hidden because of something pressed by accident one
//! evening. Restarting sdroxide and hearing the band again is the reset.
//!
//! The filter reads as `session_ignored ∪ (future persisted)`, so a stored list
//! can be folded in later without touching a single call site: every consumer
//! asks [`is_ignored`] and nothing else knows how the set is built.
//!
//! Where the filter *runs* is the other decision, and it is made once, at the
//! `RadioEvent::Ft8Decodes` arm in `frame.rs`: as early as a decode enters the
//! app, so a muted station is genuinely out of the way rather than hidden from
//! one view and still ringing, still being auto-answered. The rows already on
//! screen are drawn dimmed rather than yanked — the row is how a mistaken press
//! is taken back, and no new ones arrive to replace them.

use crate::i18n::tr;
use std::collections::HashSet;

use sdroxide_types::Decode;

use super::SdroxideApp;

/// The callsigns muted for this session, keyed by [`key`] rather than by the
/// string the decoder happened to produce.
pub(in crate::app) type Ignored = HashSet<String>;

/// How a callsign is spelled in the set: trimmed and upper case.
///
/// The decoder hands these over upper case, but a set keyed on the raw string
/// would treat `ja1abc` and `JA1ABC` as two stations — and a mute that only
/// half works is worse than none, because the operator will not know which
/// spelling it caught.
fn key(call: &str) -> String {
    call.trim().to_ascii_uppercase()
}

/// Whether this decode's sender is muted.
///
/// A decode that names no sender — free text, or a callsign nobody could read
/// back out of the hash — has nothing to mute, so it is never hidden. The
/// empty key is not in the set, but the guard keeps that from being an
/// accident of a decode that carries `from: Some("")`.
pub(in crate::app) fn is_ignored(set: &Ignored, from: Option<&str>) -> bool {
    from.filter(|c| !c.trim().is_empty()).is_some_and(|c| set.contains(&key(c)))
}

/// Mute `from`, or unmute it if it was already muted. Returns whether it is
/// muted afterwards, so a caller can say which way a press went.
pub(in crate::app) fn toggle(set: &mut Ignored, from: &str) -> bool {
    if set.remove(&key(from)) {
        false
    } else {
        set.insert(key(from));
        true
    }
}

/// The decodes of one arriving slot that are not muted, in arrival order.
///
/// This is the ingress filter, and it is the whole feature: everything
/// downstream of it — the decode list, the announcer, the audible alerts, the
/// propagation field, and auto mode's choice of a station to answer — sees only
/// what is left, so one rule covers all of them.
///
/// An empty set is the common case and hands the batch straight back rather
/// than re-collecting it.
pub(in crate::app) fn retain_unignored(set: &Ignored, batch: Vec<Decode>) -> Vec<Decode> {
    if set.is_empty() {
        return batch;
    }
    batch.into_iter().filter(|d| !is_ignored(set, d.from.as_deref())).collect()
}

impl SdroxideApp {
    /// Mute a station, or unmute one already muted, from a decode row's button.
    /// Returns the state it is in afterwards, so the caller could say which way
    /// the press went.
    pub(in crate::app) fn toggle_ignore(&mut self, from: &str) -> bool {
        toggle(&mut self.session_ignored, from)
    }

    /// Empty the list, from the decode header's chip.
    pub(in crate::app) fn clear_ignored(&mut self) {
        self.session_ignored.clear();
    }

    /// What the header chip's hover says: who is in the list, how long it lasts,
    /// what muting does and does not reach, and both ways out of it.
    ///
    /// A beginner cannot be left guessing whether a muted station is still
    /// being uploaded, still answering, still counted — the chip that mutes is
    /// the one place that can answer that.
    pub(in crate::app) fn ignored_hover(&self) -> String {
        let mut names: Vec<&str> = self.session_ignored.iter().map(String::as_str).collect();
        names.sort_unstable();
        tr!("ignore-muted-hover", names = names.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn d(from: Option<&str>) -> Decode {
        Decode {
            slot_utc: 0,
            snr_db: 0,
            dt: 0.0,
            audio_hz: 1500.0,
            message: "CQ".into(),
            to: None,
            from: from.map(str::to_string),
            grid: None,
            is_cq: true,
            cq_to: None,
            free_text: false,
            rr73_to: None,
        }
    }

    fn set_of(calls: &[&str]) -> Ignored {
        calls.iter().map(|c| key(c)).collect()
    }

    #[test]
    fn a_muted_station_is_dropped_at_ingress() {
        let set = set_of(&["19LR121"]);
        let kept = retain_unignored(&set, vec![d(Some("29AT715")), d(Some("19LR121"))]);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].from.as_deref(), Some("29AT715"));
    }

    #[test]
    fn the_list_is_case_and_space_insensitive() {
        // The set is written upper case; the decode arrives however the decoder
        // spelled it, and it is the same station either way.
        let set = set_of(&["ja1abc"]);
        assert!(is_ignored(&set, Some("JA1ABC")));
        assert!(is_ignored(&set, Some("JA1ABC ")));
        // And unmuting the same way round takes it back out.
        let mut set = set_of(&["JA1ABC"]);
        assert!(!toggle(&mut set, "ja1abc "));
        assert!(!is_ignored(&set, Some("JA1ABC")));
    }

    #[test]
    fn a_press_mutes_then_unmutes() {
        let mut set = Ignored::new();
        assert!(toggle(&mut set, "26AT715"));
        assert!(is_ignored(&set, Some("26AT715")));
        assert!(!toggle(&mut set, "26AT715"));
        assert!(!is_ignored(&set, Some("26AT715")));
    }

    #[test]
    fn a_decode_naming_no_station_is_never_muted() {
        // Free text and an unreadable hash both carry no sender, and there is
        // nothing on the row to press − against.
        let set = set_of(&[""]);
        assert!(!is_ignored(&set, None));
        assert!(!is_ignored(&set, Some("")));
        let kept = retain_unignored(&set, vec![d(None)]);
        assert_eq!(kept.len(), 1);
    }

    #[test]
    fn an_empty_list_hands_the_whole_batch_back() {
        let batch = vec![d(Some("A")), d(Some("B")), d(Some("C"))];
        assert_eq!(retain_unignored(&Ignored::new(), batch).len(), 3);
    }

    #[test]
    fn a_decode_to_us_is_muted_like_any_other() {
        // The row an operator owes an answer to is not a special case: they
        // pressed the button, and the button said it would stop arriving.
        let set = set_of(&["26AT715"]);
        assert!(is_ignored(&set, Some("26AT715")));
    }
}
