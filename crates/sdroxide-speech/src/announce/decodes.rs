//! Deciding which decoded messages to read, and saying each exactly once.
//!
//! Three modes, three data shapes, three dedup keys:
//!
//! - **FT8/FT4** arrive as a *delta* per slot, so dedup is only a guard against
//!   redelivery on reconnect.
//! - **JS8** arrives as a *snapshot* whose entries mutate — `frames` grows and
//!   `complete` flips — so the key has to be built from the fields that do not
//!   move.
//! - **FSQ** carries no timestamp at all, so the key is the content.
//!
//! Each set is bounded and pruned, because a station left running overnight
//! must not accumulate one entry per decode until it runs out of memory.

use std::collections::{HashSet, VecDeque};

use sdroxide_types::{Decode, DigiConfig, FsqMsg, Js8Msg, SpeechSettings, cq_is_for_us};

use super::digest::fnv1a;
use super::ft8;
use crate::queue::{Priority, Utterance};
use crate::text::Speaker;

/// Most recent decodes remembered per mode.
const SEEN_CAP: usize = 512;

/// A bounded "already said this" set with first-in-first-out eviction.
#[derive(Debug, Default)]
struct SeenSet {
    set: HashSet<u64>,
    order: VecDeque<u64>,
    cap: usize,
}

impl SeenSet {
    fn new(cap: usize) -> Self {
        SeenSet { set: HashSet::new(), order: VecDeque::new(), cap }
    }

    /// `true` the first time a key is offered, `false` afterwards.
    fn insert(&mut self, k: u64) -> bool {
        if !self.set.insert(k) {
            return false;
        }
        self.order.push_back(k);
        while self.order.len() > self.cap {
            if let Some(old) = self.order.pop_front() {
                self.set.remove(&old);
            }
        }
        true
    }

    fn clear(&mut self) {
        self.set.clear();
        self.order.clear();
    }
}

/// Everything already read out, across the modes.
#[derive(Debug)]
pub struct SeenDecodes {
    ft8: SeenSet,
    js8: SeenSet,
    fsq: SeenSet,
}

impl Default for SeenDecodes {
    fn default() -> Self {
        SeenDecodes {
            ft8: SeenSet::new(SEEN_CAP),
            js8: SeenSet::new(SEEN_CAP),
            fsq: SeenSet::new(SEEN_CAP),
        }
    }
}

impl SeenDecodes {
    pub fn clear(&mut self) {
        self.ft8.clear();
        self.js8.clear();
        self.fsq.clear();
    }

    /// FT8/FT4 decodes worth reading.
    ///
    /// A decode counts as addressed to us when `to` names our call — the same
    /// test the decode panel makes — **or** when `rr73_to` does. The second is
    /// not redundant: in Hound mode a Fox signals that the contact is complete
    /// through that field and nowhere else, so ignoring it would leave the
    /// operator waiting for a confirmation that had already come.
    pub fn ft8(
        &mut self,
        decodes: &[Decode],
        digi: &DigiConfig,
        cfg: &SpeechSettings,
        sp: &Speaker<'_>,
        now: f64,
        out: &mut Vec<Utterance>,
    ) {
        let my_call = digi.my_call.trim();
        if my_call.is_empty() {
            return;
        }
        for d in decodes {
            let to_me = d.to.as_deref().is_some_and(|t| t.eq_ignore_ascii_case(my_call))
                || d.rr73_to.as_deref().is_some_and(|t| t.eq_ignore_ascii_case(my_call));
            let for_me = cfg.decodes.ft8_cq_for_me
                && d.is_cq
                && cq_is_for_us(d, my_call, digi.my_grid.trim());

            if !(cfg.decodes.ft8_to_me && to_me) && !for_me {
                continue;
            }
            // Two stations sending identical text in the same slot are two
            // decodes, and only the audio tone tells them apart.
            let key = fnv1a(
                format!("{}|{}|{}", d.slot_utc, d.audio_hz.round() as i64, d.message).as_bytes(),
            );
            if !self.ft8.insert(key) {
                continue;
            }
            out.push(
                Utterance::new(render_call(d, cfg, sp, to_me), Priority::Notable, now).ttl(30.0),
            );
        }
    }

    /// JS8 messages worth reading.
    ///
    /// `first_slot_utc` is stable across reassembly where `last_slot_utc` is
    /// not, so it anchors the key. Only completed messages are read by default:
    /// a partial would otherwise be spoken once now and again when it finished.
    pub fn js8(
        &mut self,
        msgs: &[Js8Msg],
        cfg: &SpeechSettings,
        sp: &Speaker<'_>,
        now: f64,
        out: &mut Vec<Utterance>,
    ) {
        if !cfg.decodes.js8 {
            return;
        }
        for m in msgs {
            if !m.to_me {
                continue;
            }
            let is_allcall = m.to.eq_ignore_ascii_case("@ALLCALL");
            if is_allcall && !cfg.decodes.js8_allcall {
                continue;
            }
            if !m.complete && !cfg.decodes.js8_partial {
                continue;
            }
            let key = fnv1a(
                format!("{}|{}|{}", m.from, m.first_slot_utc, m.audio_hz.round() as i64).as_bytes(),
            );
            if !self.js8.insert(key) {
                continue;
            }
            let mut text = format!("{}: {}", sp.callsign(&m.from), sp.message(&m.text));
            if cfg.decodes.include_snr {
                text.push_str(&format!(", {}", sp.snr(m.snr_db)));
            }
            out.push(Utterance::new(text, Priority::Notable, now).ttl(30.0));
        }
    }

    /// FSQ messages worth reading. No timestamp exists, so the key is content.
    pub fn fsq(
        &mut self,
        msgs: &[FsqMsg],
        cfg: &SpeechSettings,
        sp: &Speaker<'_>,
        now: f64,
        out: &mut Vec<Utterance>,
    ) {
        if !cfg.decodes.fsq {
            return;
        }
        for m in msgs {
            if !m.to_me || m.to.is_empty() {
                continue;
            }
            let key = fnv1a(format!("{}\u{1}{}\u{1}{}", m.from, m.to, m.text).as_bytes());
            if !self.fsq.insert(key) {
                continue;
            }
            let text = format!("{}: {}", sp.callsign(&m.from), sp.message(&m.text));
            out.push(Utterance::new(text, Priority::Notable, now).ttl(30.0));
        }
    }
}

/// How an FT8 decode is read out, by verbosity.
///
/// The shape is `<who> <what they said>` — see [`super::ft8`] for why the
/// second half is worth the classification, and why our own signal report only
/// rides along on the messages that carry no number of their own.
fn render_call(d: &Decode, cfg: &SpeechSettings, sp: &Speaker<'_>, to_me: bool) -> String {
    let who = d.from.as_deref().unwrap_or("");
    let call = if who.is_empty() { String::new() } else { sp.callsign(who) };

    if cfg.is_terse() {
        return if call.is_empty() { sp.message(&d.message) } else { call };
    }

    let payload = ft8::classify(&d.message);
    let said = ft8::heard(&payload, to_me, sp);
    let named = !call.is_empty();
    let mut s = if !named {
        sp.message(&d.message)
    } else if said.is_empty() {
        call
    } else {
        format!("{call} {said}")
    };
    // Free text is the message itself: nothing above it said what was in it.
    if payload == ft8::Payload::Other && named {
        let body = sp.message(ft8::payload_text(&d.message, d.to.as_deref().unwrap_or(""), who));
        if !body.trim().is_empty() {
            s.push_str(&format!(", {body}"));
        }
    }
    if cfg.decodes.include_snr && !payload.has_report() && !payload.is_closing() {
        s.push_str(&format!(", {}", sp.snr(d.snr_db)));
    }
    if cfg.decodes.include_grid
        && let Some(g) = d.grid.as_deref()
        && !g.is_empty()
    {
        s.push_str(&format!(", {}", sp.grid(g)));
    }
    // `Full` repeats the raw message under the reading of it — everything
    // above is an interpretation, and this is the text it was made from.
    if cfg.is_verbose() && payload != ft8::Payload::Other {
        s.push_str(&format!(", {}", sp.message(&d.message)));
    }
    s
}
