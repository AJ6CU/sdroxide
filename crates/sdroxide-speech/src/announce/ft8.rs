//! The FT8 exchange, in words.
//!
//! An FT8 QSO is five messages of thirteen characters, and four of them look
//! almost identical: `OE3ABC K1ABC JN88`, `OE3ABC K1ABC -12`, `OE3ABC K1ABC
//! R-12`, `OE3ABC K1ABC RR73`. Reading the callsigns out and appending our own
//! signal report — which is what this used to do — says the same sentence five
//! times and never once says which stage of the contact it is. An operator
//! listening rather than watching hears a number repeat and cannot tell a
//! report from a sign-off.
//!
//! So the payload is classified and *that* is what gets read: "reports minus
//! twelve", "confirms minus twelve", "rogers and seventy three". The stage is
//! the information; the callsign only has to be said because more than one
//! station may be calling.
//!
//! # Where the numbers go
//!
//! A message that carries a report says that report and nothing else — the
//! report they sent is the one that matters, and appending our own measurement
//! of them puts two different numbers in one breath. Our measurement rides on
//! the messages that carry no number of their own: the call and the CQ, where
//! it is what decides whether the contact is worth having.
//!
//! # Why the classifier is duplicated
//!
//! `sdroxide-digi` has one of these, and it is the sequencer's. This crate does
//! not depend on that one — sdroxide-digi is GPL-3.0-or-later and this crate is
//! linked into every build — so the twenty lines are written again here rather
//! than dragging the licence across the whole tree. They are a wire format that
//! has not changed since 2017; if they ever diverge it is because the sequencer
//! grew a case, and the worst outcome is a message read as free text.

use sdroxide_types::{DigiStatus, Mode, QsoStep, SpeechSettings};

use crate::queue::{Key, Priority, Utterance};
use crate::text::{Speaker, phonetic};

/// What an FT8 message is *for*, past the two callsigns.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Payload {
    /// A general call. Anything from `CQ` to `CQ POTA K1ABC FN42`.
    Cq,
    /// A reply carrying a grid square — the opening message of an exchange.
    Grid(String),
    /// Their signal report of us.
    Report(i16),
    /// `R` and their report: they have ours, and this is theirs.
    RReport(i16),
    Rrr,
    Rr73,
    B73,
    /// Free text, or a form we do not know.
    Other,
}

impl Payload {
    /// Whether the message carries a signal report of its own.
    ///
    /// The one question the announcer asks: a message that already has a number
    /// in it must not also carry ours.
    pub fn has_report(&self) -> bool {
        matches!(self, Payload::Report(_) | Payload::RReport(_))
    }

    /// Whether this message closes the contact.
    ///
    /// Nothing is decided after a sign-off, so nothing needs a number on it —
    /// our measurement of a station saying goodbye is the definition of a
    /// figure nobody can act on.
    pub fn is_closing(&self) -> bool {
        matches!(self, Payload::Rrr | Payload::Rr73 | Payload::B73)
    }
}

/// Classify a whole FT8 message: `OE3ABC K1ABC R-12` is an [`Payload::RReport`].
pub fn classify(message: &str) -> Payload {
    let toks: Vec<&str> = message.split_whitespace().collect();
    if toks.first().is_some_and(|t| t.eq_ignore_ascii_case("CQ")) {
        return Payload::Cq;
    }
    // The payload is the third token: `<to> <from> <payload>`. A message with
    // fewer is a bare call — `K1ABC OE3ABC` — with nothing to classify.
    let Some(p) = toks.get(2) else { return Payload::Other };
    let signed = |s: &str| -> Option<i16> {
        let n: i16 = s.strip_prefix(['+', '-'])?.parse().ok()?;
        Some(if s.starts_with('-') { -n } else { n })
    };
    match *p {
        "RR73" => Payload::Rr73,
        "RRR" => Payload::Rrr,
        "73" => Payload::B73,
        s if s.starts_with("R-") || s.starts_with("R+") => {
            signed(&s[1..]).map(Payload::RReport).unwrap_or(Payload::Other)
        }
        s => match signed(s) {
            Some(n) => Payload::Report(n),
            None if phonetic::looks_like_grid(s) => Payload::Grid(s.to_string()),
            None => Payload::Other,
        },
    }
}

/// The part of a message that is not the two callsigns.
///
/// Free text is read out whole, and reading `OE3ABC K1ABC HW CPY` whole means
/// spelling both callsigns before the four words that matter — twelve seconds
/// of phonetics for a two-word question. The caller has already said who is
/// talking, so the calls come off here.
pub fn payload_text<'a>(message: &'a str, to: &str, from: &str) -> &'a str {
    let mut it = message.split_whitespace();
    let (Some(a), Some(b)) = (it.next(), it.next()) else { return message };
    if !a.eq_ignore_ascii_case(to) || !b.eq_ignore_ascii_case(from) {
        return message;
    }
    let rest = message.trim_start();
    let rest = rest[a.len()..].trim_start();
    rest[b.len()..].trim_start()
}

/// What the other station just said, as a phrase to follow their callsign.
///
/// `to_me` separates the two readings of a grid message: it is an answer when
/// it names us and somebody else's business when it does not.
pub fn heard(payload: &Payload, to_me: bool, sp: &Speaker<'_>) -> String {
    match payload {
        Payload::Cq => "calling C Q".to_string(),
        Payload::Grid(_) if to_me => "calling you".into(),
        Payload::Grid(_) => "calling C Q".into(),
        Payload::Report(r) => format!("reports {}", sp.snr(*r)),
        Payload::RReport(r) => format!("confirms {}", sp.snr(*r)),
        Payload::Rrr => "rogers".into(),
        Payload::Rr73 => "rogers and seventy three".into(),
        Payload::B73 => "says seventy three".into(),
        Payload::Other if to_me => "calling you".into(),
        Payload::Other => String::new(),
    }
}

/// Our own side of the exchange: the transmissions and the end of the contact.
///
/// # Why the *pending* message and not the sent one
///
/// [`DigiStatus::transcript`] records what actually went out, which sounds like
/// the better source until you notice when it appears: the line is written as
/// the burst is keyed, and by then PTT is up and the speech queue is gagged
/// (see [`crate::queue::Gag`]) precisely so that announcements do not go out
/// over the air. Every one of them would be refused.
///
/// [`DigiStatus::tx_pending_msg`] holds the same text one slot earlier, in the
/// gap where there is time to say it and nothing transmitting. It also changes
/// only when the exchange advances, so a CQ repeated eleven times is announced
/// once — the sequencer's own dedup, for free.
#[derive(Debug)]
pub struct QsoWatch {
    /// Adopted the first snapshot. Until then everything is history: attaching
    /// to a station mid-QSO must not recite the exchange so far.
    seeded: bool,
    step: QsoStep,
    pending: Option<String>,
}

impl Default for QsoWatch {
    fn default() -> Self {
        QsoWatch::new()
    }
}

impl QsoWatch {
    pub fn new() -> Self {
        QsoWatch { seeded: false, step: QsoStep::Idle, pending: None }
    }

    /// Forget everything, without saying anything — a reconnect, or speech
    /// being switched off and on again.
    pub fn reset(&mut self) {
        *self = QsoWatch::new();
    }

    /// Fold in a status snapshot.
    pub fn observe(
        &mut self,
        st: &DigiStatus,
        cfg: &SpeechSettings,
        sp: &Speaker<'_>,
        now: f64,
        out: &mut Vec<Utterance>,
    ) {
        // Only the sequenced modes have an exchange. The keyboard modes put
        // their typed buffer in `tx_pending_msg`, and reading the operator's
        // own typing back to them is not an announcement.
        if !matches!(st.mode, Mode::Ft8 | Mode::Ft4 | Mode::Ft2) {
            self.reset();
            return;
        }
        let pending = st.tx_pending_msg.clone().filter(|_| st.tx_next);
        let speak = cfg.decodes.ft8_qso && self.seeded;
        let (was_step, was_pending) = (self.step, std::mem::replace(&mut self.pending, pending));
        self.step = st.step;
        self.seeded = true;
        if !speak {
            return;
        }

        // The contact just finished and went in the log. `Confirming` is
        // reached from exactly one place in the sequencer, which is what makes
        // this edge trustworthy: it is not "we sent a 73", it is "logged".
        if st.step == QsoStep::Confirming && was_step != QsoStep::Confirming {
            let text = match st.dx_call.as_deref().filter(|c| !c.is_empty()) {
                Some(call) => format!("Q S O with {} complete", sp.callsign(call)),
                None => "Q S O complete".to_string(),
            };
            out.push(Utterance::new(text, Priority::Notable, now).ttl(30.0));
        }

        if let Some(msg) = self.pending.as_deref()
            && was_pending.as_deref() != Some(msg)
        {
            let text = sending(msg, sp);
            if !text.trim().is_empty() {
                // Keyed, so a plan overtaken by the exchange is replaced rather
                // than queued behind itself, and short-lived, because a message
                // for a slot that has already gone is not worth hearing.
                out.push(Utterance::new(text, Priority::Notable, now).key(Key::Ft8Tx).ttl(12.0));
            }
        }
    }
}

/// How the next transmission is read out.
///
/// The station is named only where the operator has a decision in it — which
/// call we answered, and who we are working. By the time we are sending a
/// report they know perfectly well who to; repeating six syllables of phonetics
/// every slot is how an announcement becomes something to switch off.
fn sending(msg: &str, sp: &Speaker<'_>) -> String {
    let to = msg.split_whitespace().next().unwrap_or("");
    match classify(msg) {
        Payload::Cq => "calling C Q".to_string(),
        Payload::Grid(_) => format!("answering {}", sp.callsign(to)),
        Payload::Report(r) => format!("sending {} to {}", sp.snr(r), sp.callsign(to)),
        Payload::RReport(r) => format!("sending roger {}", sp.snr(r)),
        Payload::Rrr => "sending roger roger".into(),
        Payload::Rr73 => "sending roger roger seventy three".into(),
        Payload::B73 => "sending seventy three".into(),
        // A message the operator typed by hand, read as it stands.
        Payload::Other => format!("sending {}", sp.message(msg)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_five_messages_of_an_exchange() {
        assert_eq!(classify("CQ K1ABC FN42"), Payload::Cq);
        assert_eq!(classify("CQ DX K1ABC FN42"), Payload::Cq);
        assert_eq!(classify("OE3ABC K1ABC JN88"), Payload::Grid("JN88".into()));
        assert_eq!(classify("OE3ABC K1ABC -12"), Payload::Report(-12));
        assert_eq!(classify("OE3ABC K1ABC +03"), Payload::Report(3));
        assert_eq!(classify("OE3ABC K1ABC R-12"), Payload::RReport(-12));
        assert_eq!(classify("OE3ABC K1ABC RRR"), Payload::Rrr);
        assert_eq!(classify("OE3ABC K1ABC RR73"), Payload::Rr73);
        assert_eq!(classify("OE3ABC K1ABC 73"), Payload::B73);
        assert_eq!(classify("OE3ABC K1ABC HW CPY"), Payload::Other);
        assert_eq!(classify("K1ABC OE3ABC"), Payload::Other);
    }

    #[test]
    fn only_a_report_carries_a_number() {
        assert!(classify("OE3ABC K1ABC -12").has_report());
        assert!(classify("OE3ABC K1ABC R-12").has_report());
        assert!(!classify("OE3ABC K1ABC RR73").has_report());
        assert!(!classify("CQ K1ABC FN42").has_report());
    }

    #[test]
    fn free_text_loses_the_callsigns() {
        assert_eq!(payload_text("OE3ABC K1ABC HW CPY", "OE3ABC", "K1ABC"), "HW CPY");
        // Not the two calls we were told about: read it whole rather than
        // guess which words were meant to be dropped.
        assert_eq!(payload_text("OE3ABC K1ABC HW CPY", "W9XYZ", "K1ABC"), "OE3ABC K1ABC HW CPY");
        assert_eq!(payload_text("CQ K1ABC FN42", "", "K1ABC"), "CQ K1ABC FN42");
    }
}
