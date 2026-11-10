//! Reading the keyboard modes' decoded text without falling behind it.
//!
//! # The rolling-snapshot problem
//!
//! `DigiStatus::text_rx` is not a stream of new characters — it is a whole
//! 8000-byte snapshot, shared by CW, RTTY, PSK31, Olivia, THOR and FSQ, and
//! trimmed from the *front* once it fills. The trim rounds up to a character
//! boundary, so it removes a number of bytes that is a multiple of nothing.
//! Byte offsets are therefore worthless, and `snapshot.starts_with(previous)`
//! is wrong from the moment the cap is first reached.
//!
//! # Anchoring by suffix
//!
//! So we track not a position but the last characters already handed out, and
//! find them again in each new snapshot. Everything after them is new. If the
//! anchor has scrolled off entirely, progressively shorter suffixes are tried;
//! if none matches, the stream has jumped and we re-anchor **silently** rather
//! than read eight kilobytes of backlog aloud.

use crate::queue::{Key, Priority, Utterance};
use crate::text::Speaker;

/// How much already-spoken text to keep as the resync anchor. Long enough to
/// be unique in ordinary traffic, short enough that one decoder hiccup does not
/// lose the thread.
const ANCHOR_CHARS: usize = 48;

/// Shorter anchors, tried in turn when the long one has gone. Below eight, the
/// false-match risk on repetitive traffic — "CQ CQ CQ " — outweighs the gain.
const FALLBACK_ANCHORS: [usize; 3] = [24, 12, 8];

/// A ragged word held this long without growing is emitted anyway: the sender
/// stopped mid-word, or the transmission ended without a trailing space.
const FLUSH_S: f64 = 1.5;

/// Longest chunk handed to the synthesizer at once. Piper generates a whole
/// utterance in one go, so chunking gets the first word out sooner and makes
/// barge-in cheap.
const CHUNK_MAX: usize = 160;

/// Tracks how far through the rolling buffer we have read.
#[derive(Debug, Default)]
pub struct RollingTail {
    /// The last characters already emitted. Empty means unanchored.
    anchor: String,
    /// New text not yet emitted, waiting for a word boundary.
    held: String,
    /// When `held` last grew.
    held_at: f64,
    /// Discontinuities seen, for diagnosis.
    pub resyncs: u64,
}

impl RollingTail {
    pub fn new() -> Self {
        Self::default()
    }

    /// Adopt the current snapshot as already-seen, emitting nothing.
    ///
    /// Called on first sight, on a mode change, and after an unrecoverable
    /// discontinuity.
    pub fn anchor_to(&mut self, snapshot: &str) {
        self.anchor = last_chars(snapshot, ANCHOR_CHARS);
        self.held.clear();
    }

    pub fn reset(&mut self) {
        self.anchor.clear();
        self.held.clear();
    }

    /// Fold in a snapshot; return text ready to speak.
    pub fn advance(&mut self, snapshot: &str, now: f64) -> Option<String> {
        if snapshot.is_empty() {
            // The decoder cleared, or the mode changed under us.
            self.reset();
            return None;
        }
        if self.anchor.is_empty() {
            // Never read the backlog aloud on first sight.
            self.anchor_to(snapshot);
            return None;
        }

        let new: Option<String> = if let Some(i) = snapshot.rfind(&self.anchor) {
            Some(snapshot[i + self.anchor.len()..].to_string())
        } else {
            // The anchor has scrolled off, or the decoder reset. Try shorter
            // suffixes of it. `rfind`, not `find`: when a run repeats, the
            // latest occurrence is the right guess.
            let mut found = None;
            for n in FALLBACK_ANCHORS {
                let short = last_chars(&self.anchor, n);
                if short.is_empty() {
                    continue;
                }
                if let Some(i) = snapshot.rfind(&short) {
                    self.resyncs += 1;
                    found = Some(snapshot[i + short.len()..].to_string());
                    break;
                }
            }
            if found.is_none() {
                // Total discontinuity. Emit nothing — silence beats replaying
                // eight kilobytes — and anything held is by definition older
                // than the break.
                self.resyncs += 1;
                self.anchor_to(snapshot);
                return None;
            }
            found
        };

        if let Some(new) = new
            && !new.is_empty()
        {
            self.held.push_str(&new);
            self.held_at = now;
            // The anchor advances by what was *consumed*, while `held` is what
            // has not yet been *spoken*; the invariant "anchor is inside
            // already-consumed text" holds even while `held` lags behind.
            let mut merged = std::mem::take(&mut self.anchor);
            merged.push_str(&new);
            self.anchor = last_chars(&merged, ANCHOR_CHARS);
        }

        // Hold a ragged word until a boundary, or until it stops growing.
        if let Some(cut) = self.held.rfind(char::is_whitespace) {
            let ready: String = self.held[..cut].trim().to_string();
            self.held = self.held[cut..].trim_start().to_string();
            if !ready.is_empty() {
                return Some(ready);
            }
        }
        if !self.held.trim().is_empty() && now - self.held_at >= FLUSH_S {
            return Some(std::mem::take(&mut self.held).trim().to_string());
        }
        None
    }

    /// When a held fragment would be flushed, for the repaint request.
    pub fn deadline(&self) -> Option<f64> {
        (!self.held.trim().is_empty()).then_some(self.held_at + FLUSH_S)
    }
}

/// Meters decoded text into the queue at a rate a listener can follow.
///
/// The rule that matters: **drop, do not queue**. A CW decoder in good
/// conditions produces text faster than speech reads it, and queueing puts the
/// operator minutes behind the audio they are also listening to. So there is
/// one bounded backlog, only ever one pending utterance, and overflow is
/// discarded from the front.
#[derive(Debug, Default)]
pub struct TextPump {
    tail: RollingTail,
    backlog: String,
    /// Characters silently discarded, for diagnosis.
    pub dropped: u64,
}

impl TextPump {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.tail.reset();
        self.backlog.clear();
    }

    /// Re-anchor without emitting — for a mode change, or CW losing lock.
    pub fn anchor_to(&mut self, snapshot: &str) {
        self.tail.anchor_to(snapshot);
        self.backlog.clear();
    }

    /// Fold in a snapshot and, if there is room, queue the next chunk.
    pub fn pump(
        &mut self,
        snapshot: &str,
        now: f64,
        sp: &Speaker<'_>,
        max_backlog: usize,
        queue_has_text: bool,
        sink_busy: bool,
        out: &mut Vec<Utterance>,
    ) {
        if let Some(new) = self.tail.advance(snapshot, now) {
            if !self.backlog.is_empty() {
                self.backlog.push(' ');
            }
            self.backlog.push_str(&new);
        }

        let max = max_backlog.max(40);
        let len = self.backlog.chars().count();
        if len > max {
            // Drop from the front: reading two-minute-old CW while the operator
            // listens to live audio is worse than skipping it.
            let skip = len - max;
            let cut: String = self.backlog.chars().skip(skip).collect();
            // Resume on a word boundary so the survivor does not start mid-word.
            let cut = match cut.find(char::is_whitespace) {
                Some(i) => cut[i..].trim_start().to_string(),
                None => cut,
            };
            self.dropped += (len - cut.chars().count()) as u64;
            self.backlog = cut;
        }

        if queue_has_text || sink_busy || self.backlog.trim().is_empty() {
            return;
        }
        let chunk = take_chunk(&mut self.backlog, CHUNK_MAX);
        let spoken = sp.message(&chunk);
        if spoken.trim().is_empty() {
            return;
        }
        out.push(Utterance::new(spoken, Priority::Chatter, now).key(Key::Text).ttl(8.0));
    }

    pub fn deadline(&self) -> Option<f64> {
        self.tail.deadline()
    }

    pub fn resyncs(&self) -> u64 {
        self.tail.resyncs
    }
}

/// Take up to `max` characters off the front of `s`, splitting at whitespace.
fn take_chunk(s: &mut String, max: usize) -> String {
    if s.chars().count() <= max {
        return std::mem::take(s);
    }
    let end = s.char_indices().nth(max).map(|(i, _)| i).unwrap_or(s.len());
    let cut = s[..end].rfind(char::is_whitespace).unwrap_or(end);
    let head = s[..cut].to_string();
    *s = s[cut..].trim_start().to_string();
    head
}

/// The last `n` characters of `s`, on a character boundary.
fn last_chars(s: &str, n: usize) -> String {
    let count = s.chars().count();
    if count <= n {
        return s.to_string();
    }
    s.chars().skip(count - n).collect()
}
