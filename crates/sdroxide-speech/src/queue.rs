//! What to say next.
//!
//! A radio produces changes far faster than anyone can be told about them. Spin
//! the dial and thirty state snapshots a second arrive; work a pileup and a
//! hundred decodes a minute do. The queue is what turns that into speech an
//! operator can actually follow, on three ideas:
//!
//! - **Coalescing.** An announcement with a [`Key`] replaces a pending one with
//!   the same key rather than queueing behind it. The tenth frequency
//!   read-out while scrolling overwrites the ninth; it does not wait for it.
//! - **A budget.** Both the item count and the *estimated speaking time* are
//!   capped. Twenty seconds is where an announcement stops describing the radio
//!   and starts describing the past.
//! - **A gag.** While the transmitter is keyed, everything below an alert is
//!   refused outright rather than deferred — a queue that unloads five seconds
//!   of stale announcements the moment you unkey is its own bug.
//!
//! No clock of its own: `now` is passed in, as egui's monotonic frame time.

use std::collections::VecDeque;

/// How much an utterance matters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum Priority {
    /// Read-along text: the CW/RTTY/PSK decode stream. First to be dropped,
    /// never interrupts anything.
    Chatter = 0,
    /// The knob you just turned — frequency, mode, levels, filters. Stale
    /// within seconds, so it carries a time-to-live.
    Routine = 1,
    /// Something that happened, which you did not necessarily do: PTT, split,
    /// a VFO swap, the scanner stopping, a message addressed to you, the
    /// two-second SWR tick.
    Notable = 2,
    /// Interrupts, and speaks through the transmit gag: out of band, high SWR,
    /// a notice from the engine, the connection lost.
    Alert = 3,
}

/// What an utterance is *about*, for coalescing.
///
/// Two announcements sharing a key are two versions of one fact, and only the
/// newer is worth hearing. `None` — used for every decoded message — means the
/// opposite: two messages addressed to you are two facts, not two drafts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    Freq,
    Mode,
    Band,
    Vfo,
    Split,
    Ptt,
    Tune,
    Agc,
    AgcMaxGain,
    ManualGain,
    Volume,
    Squelch,
    Filter,
    Drive,
    TuneDrive,
    MicGain,
    Rit,
    Xit,
    Nb,
    Nr,
    Notch,
    Antenna,
    SubRx,
    Scan,
    Memory,
    Recording,
    Notice,
    SwrTick,
    SwrAlarm,
    BandEdge,
    /// The read-along text stream. Newest wins, always.
    Text,
    /// What the FT8 sequencer is about to send. A plan the exchange has already
    /// moved past is not worth hearing.
    Ft8Tx,
    /// Answering a hotkey — never coalesced away, because the operator asked.
    OnDemand,
}

/// One thing to say.
#[derive(Debug, Clone, PartialEq)]
pub struct Utterance {
    pub text: String,
    pub priority: Priority,
    /// `None` never coalesces.
    pub key: Option<Key>,
    pub queued_at: f64,
    /// Drop it unspoken rather than say it late. `None` never expires.
    pub expires_at: Option<f64>,
    /// Stop whatever is speaking and start this instead.
    pub interrupt: bool,
}

/// Speaking rate assumed for the backlog budget: 165 words a minute at five
/// characters a word.
///
/// Deliberately independent of the operator's speech-rate setting. The budget
/// is about how much they can absorb, not how fast the synthesizer runs — a
/// listener at rate 2.0 wants twice the words per second, not twice the
/// backlog.
pub const CHARS_PER_SEC: f32 = 13.75;

/// Roughly how long `text` takes to say.
pub fn est_speech_s(text: &str) -> f32 {
    text.chars().count() as f32 / CHARS_PER_SEC
}

impl Utterance {
    pub fn new(text: impl Into<String>, priority: Priority, now: f64) -> Self {
        Utterance {
            text: text.into(),
            priority,
            key: None,
            queued_at: now,
            expires_at: None,
            interrupt: false,
        }
    }

    pub fn key(mut self, k: Key) -> Self {
        self.key = Some(k);
        self
    }

    pub fn ttl(mut self, secs: f64) -> Self {
        self.expires_at = Some(self.queued_at + secs);
        self
    }

    pub fn interrupting(mut self) -> Self {
        self.interrupt = true;
        self
    }

    pub fn est_s(&self) -> f32 {
        est_speech_s(&self.text)
    }
}

/// How much may be waiting.
#[derive(Debug, Clone, Copy)]
pub struct QueueLimits {
    pub max_items: usize,
    /// Cap on estimated pending speech, in seconds.
    pub max_pending_s: f32,
    /// Default time-to-live for a [`Priority::Routine`] item that sets none.
    pub routine_ttl_s: f64,
    /// Default time-to-live for [`Priority::Chatter`].
    pub chatter_ttl_s: f64,
}

impl Default for QueueLimits {
    fn default() -> Self {
        QueueLimits { max_items: 12, max_pending_s: 20.0, routine_ttl_s: 4.0, chatter_ttl_s: 8.0 }
    }
}

/// What, if anything, is allowed to speak.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Gag {
    #[default]
    None,
    /// Transmitting. Only [`Priority::Alert`] gets through, and everything
    /// below is refused rather than held: speech reaches the operator's
    /// speakers, and therefore the microphone.
    Transmitting,
    /// Announcements are switched off.
    Off,
}

/// What [`SpeechQueue::push`] did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Push {
    Queued,
    /// Replaced a pending item with the same key.
    Coalesced,
    /// Refused — gagged, expired, or outranked on a full queue.
    Dropped,
    /// Queued, and the caller must stop the sink now.
    InterruptNow,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct QueueStats {
    pub spoken: u64,
    pub coalesced: u64,
    pub dropped_full: u64,
    pub dropped_expired: u64,
    pub dropped_gagged: u64,
    pub interrupted: u64,
}

#[derive(Debug)]
pub struct SpeechQueue {
    items: VecDeque<Utterance>,
    limits: QueueLimits,
    gag: Gag,
    /// Key and priority of whatever the sink is working on.
    speaking: Option<(Option<Key>, Priority)>,
    stats: QueueStats,
}

impl Default for SpeechQueue {
    fn default() -> Self {
        Self::new(QueueLimits::default())
    }
}

impl SpeechQueue {
    pub fn new(limits: QueueLimits) -> Self {
        SpeechQueue {
            items: VecDeque::new(),
            limits,
            gag: Gag::None,
            speaking: None,
            stats: QueueStats::default(),
        }
    }

    pub fn push(&mut self, mut u: Utterance) -> Push {
        if u.text.trim().is_empty() {
            return Push::Dropped;
        }
        match self.gag {
            Gag::Off => {
                self.stats.dropped_gagged += 1;
                return Push::Dropped;
            }
            Gag::Transmitting if u.priority < Priority::Alert => {
                self.stats.dropped_gagged += 1;
                return Push::Dropped;
            }
            _ => {}
        }

        if u.expires_at.is_none() {
            u.expires_at = match u.priority {
                Priority::Routine => Some(u.queued_at + self.limits.routine_ttl_s),
                Priority::Chatter => Some(u.queued_at + self.limits.chatter_ttl_s),
                _ => None,
            };
        }

        // Coalesce in place, keeping the queue position: a read-out that has
        // been waiting behind two others should not jump the line just because
        // the operator nudged the dial again.
        if let Some(k) = u.key
            && let Some(slot) = self.items.iter_mut().find(|p| p.key == Some(k))
        {
            let interrupt = u.interrupt && self.speaking.is_some_and(|(sk, _)| sk == Some(k));
            *slot = u;
            self.stats.coalesced += 1;
            return if interrupt {
                self.stats.interrupted += 1;
                Push::InterruptNow
            } else {
                Push::Coalesced
            };
        }

        // Make room. A burst of low-priority items eats itself; it never eats
        // something more important, and never the newest arrival.
        while self.items.len() >= self.limits.max_items
            || self.pending_speech_s() + u.est_s() > self.limits.max_pending_s
        {
            let Some(min) = self.items.iter().map(|p| p.priority).min() else {
                // Nothing pending and it still does not fit: it is a single
                // utterance longer than the whole budget. Let it through —
                // refusing it would mean never saying anything that long.
                break;
            };
            if min > u.priority {
                self.stats.dropped_full += 1;
                return Push::Dropped;
            }
            let victim = self.items.iter().position(|p| p.priority == min);
            match victim {
                Some(i) => {
                    self.items.remove(i);
                    self.stats.dropped_full += 1;
                }
                None => break,
            }
        }

        let interrupt = u.interrupt
            && self
                .speaking
                .is_some_and(|(sk, sp)| u.priority > sp || (u.key.is_some() && sk == u.key));
        self.items.push_back(u);
        if interrupt {
            self.stats.interrupted += 1;
            Push::InterruptNow
        } else {
            Push::Queued
        }
    }

    /// Retire anything that has expired, then hand back the next thing to say.
    ///
    /// `None` while `busy`, or when nothing is pending. Highest priority first,
    /// oldest first within a priority.
    pub fn pop(&mut self, now: f64, busy: bool) -> Option<Utterance> {
        let before = self.items.len();
        self.items.retain(|u| u.expires_at.is_none_or(|t| now < t));
        self.stats.dropped_expired += (before - self.items.len()) as u64;

        if busy || self.items.is_empty() {
            return None;
        }
        let best = self.items.iter().map(|u| u.priority).max()?;
        let i = self.items.iter().position(|u| u.priority == best)?;
        self.items.remove(i)
    }

    /// Record that the sink accepted what [`SpeechQueue::pop`] returned.
    pub fn begin(&mut self, u: &Utterance) {
        self.speaking = Some((u.key, u.priority));
        self.stats.spoken += 1;
    }

    /// Record that the sink finished, or was stopped.
    pub fn finish(&mut self) {
        self.speaking = None;
    }

    pub fn set_gag(&mut self, gag: Gag) {
        if self.gag != gag {
            self.gag = gag;
            // Going quiet drops what was waiting: by the time speech is allowed
            // again it would be describing the past.
            if gag != Gag::None {
                self.items.retain(|u| u.priority >= Priority::Alert);
            }
        }
    }

    pub fn gag(&self) -> Gag {
        self.gag
    }

    /// Drop everything below `p`. An alert uses this to clear the way.
    pub fn clear_below(&mut self, p: Priority) {
        self.items.retain(|u| u.priority >= p);
    }

    /// Drop everything, pending and speaking.
    pub fn clear(&mut self) {
        self.items.clear();
        self.speaking = None;
    }

    pub fn has_pending(&self, k: Key) -> bool {
        self.items.iter().any(|u| u.key == Some(k))
    }

    pub fn pending_speech_s(&self) -> f32 {
        self.items.iter().map(|u| u.est_s()).sum()
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn stats(&self) -> QueueStats {
        self.stats
    }

    /// The earliest moment anything pending expires, for the repaint request.
    pub fn next_deadline(&self) -> Option<f64> {
        self.items
            .iter()
            .filter_map(|u| u.expires_at)
            .fold(None, |acc: Option<f64>, t| Some(acc.map_or(t, |a| a.min(t))))
    }
}
