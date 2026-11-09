//! NAVTEX: SITOR-B (ITU-R M.625 collective B-mode) at 100 baud, 170 Hz shift.
//!
//! The maritime safety broadcast every coast station in the world sends on
//! 518 kHz in English, on 490 kHz in the national language and on 4209.5 kHz in
//! the tropics. Navigational and meteorological warnings, search-and-rescue
//! bulletins, ice reports and pilot notices — one of the few utility services
//! still worth leaving a receiver on (issue #212).
//!
//! # What the mode is
//!
//! Frequency-shift keying, 170 Hz apart, 100 bits a second, in a **7-bit
//! constant-ratio alphabet**: every valid character has exactly four mark bits
//! and three space bits, so any single bit error makes a code that cannot
//! exist. That is the whole of the error *detection*, and the correction is
//! time diversity — **every character is sent twice, five character times
//! apart**, so the stream alternates between a leading copy and the repeat of a
//! character sent five slots earlier:
//!
//! ```text
//! N  ·  A  ·  U  N  T  A  I  U  C  T  A  I  L  C  _  A  _  L
//! ```
//!
//! ("NAUTICAL", with the odd slots five behind the even ones.) A character
//! whose leading copy is corrupt is taken from its repeat, and only when both
//! are bad is anything lost.
//!
//! # Why the front end is written out again
//!
//! It is the same shape as [`crate::rtty::RttyRx`]'s — mix, band-limit,
//! integrate two tones over a bit, decide against each tone's own recent level
//! — and it is deliberately not shared with it. RTTY's framing is asynchronous
//! start/stop and this is a continuous synchronous stream with no framing bits
//! at all, so only the front half would be common; and RTTY is a mode that
//! works, on the air, today. Refactoring underneath it to save eighty lines
//! here is a trade nobody asked for.

use std::collections::VecDeque;

use crate::Complex32;
use crate::fir::{ComplexFir, bandpass_taps};
use crate::rtty::Integrator;

/// The audio centre a NAVTEX signal is expected at, in Hz.
///
/// The two tones sit ±85 Hz either side. The channel frequencies are quoted as
/// the *assigned* frequency, which is this centre, so a receiver in USB tunes
/// its dial 1700 Hz below the channel — 516.300 kHz for the 518 kHz service —
/// and [`crate::Mode::Navtex`]'s tone offset is what does that arithmetic.
pub const NAVTEX_CENTER_HZ: f32 = 1700.0;
/// Shift between mark and space, in Hz.
pub const NAVTEX_SHIFT_HZ: f32 = 170.0;
/// Symbol rate.
pub const NAVTEX_BAUD: f64 = 100.0;

/// Bits in one character.
const BITS: usize = 7;
/// Character slots between a character and its repeat.
const FEC_CHARS: usize = 5;
/// Bits between a character and its repeat.
const FEC_BITS: usize = FEC_CHARS * BITS;
/// Bits between one leading character and the next: the stream alternates
/// leading copies and repeats, so a character's own slots are two apart.
const PAIR_BITS: usize = 2 * BITS;

// ─────────────────────────── CCIR 476 ───────────────────────────

/// Letter shift.
pub const CODE_LTRS: u8 = 0x5a;
/// Figure shift.
pub const CODE_FIGS: u8 = 0x36;
/// Idle "alpha" — one half of the phasing pair, and the filler a transmitter
/// sends between characters.
pub const CODE_ALPHA: u8 = 0x0f;
/// Idle "beta".
pub const CODE_BETA: u8 = 0x33;
/// Idle "rep" — the other half of the phasing pair.
pub const CODE_REP: u8 = 0x66;
/// Signal repetition (character 32).
pub const CODE_CHAR32: u8 = 0x6a;

/// The CCIR 476 alphabet: `(code, letter, figure)`.
///
/// Transcribed from the ITU-R M.476 table as fldigi's `navtex.cxx` carries it,
/// and checked by [`the_alphabet_is_a_constant_ratio_code`]: all thirty-five
/// seven-bit patterns with four mark bits are accounted for — twenty-nine
/// characters here and the six control codes above.
const ALPHABET: [(u8, char, char); 29] = [
    (0x17, 'J', '\''),
    (0x1b, 'F', '!'),
    (0x1d, 'C', ':'),
    (0x1e, 'K', '('),
    (0x27, 'W', '2'),
    (0x2b, 'Y', '6'),
    (0x2d, 'P', '0'),
    (0x2e, 'Q', '1'),
    (0x35, 'G', '&'),
    (0x39, 'M', '.'),
    (0x3a, 'X', '/'),
    (0x3c, 'V', ';'),
    (0x47, 'A', '-'),
    (0x4b, 'S', '\u{7}'),
    (0x4d, 'I', '8'),
    (0x4e, 'U', '7'),
    (0x53, 'D', '$'),
    (0x55, 'R', '4'),
    (0x56, 'E', '3'),
    (0x59, 'N', ','),
    (0x5c, ' ', ' '),
    (0x63, 'Z', '"'),
    (0x65, 'L', ')'),
    (0x69, 'H', '#'),
    (0x6c, '\n', '\n'),
    (0x71, 'O', '9'),
    (0x72, 'B', '?'),
    (0x74, 'T', '5'),
    (0x78, '\r', '\r'),
];

/// Whether a 7-bit pattern is a legal CCIR 476 code: four mark bits, three
/// space. Every single-bit error breaks this, which is what the alphabet is
/// for.
///
/// The receiver no longer asks: it picks the legal code nearest what it
/// heard (see [`decide`]) and so never holds an illegal one. The tests do.
#[cfg(test)]
#[must_use]
pub fn is_valid_code(code: u8) -> bool {
    code < 0x80 && code.count_ones() == 4
}

/// The character a code stands for in the current shift, or `None` for a
/// control code (and for anything that is not a character at all).
#[must_use]
pub fn code_to_char(code: u8, figures: bool) -> Option<char> {
    ALPHABET.iter().find(|&&(c, _, _)| c == code).map(|&(_, l, f)| if figures { f } else { l })
}

/// The code for a character in the letter shift — the transmit side, and what
/// the tests synthesise with.
#[must_use]
pub fn char_to_code(ch: char) -> Option<u8> {
    let up = ch.to_ascii_uppercase();
    ALPHABET.iter().find(|&&(_, l, _)| l == up).map(|&(c, _, _)| c)
}

/// The code for a character in the figure shift.
#[must_use]
pub fn figure_to_code(ch: char) -> Option<u8> {
    ALPHABET.iter().find(|&&(_, _, f)| f == ch).map(|&(c, _, _)| c)
}

// ─────────────────────────── the receiver ───────────────────────────

/// The rate the receiver works at internally, in Hz, give or take: the input
/// is averaged down by a whole factor to the nearest rate at or above this.
///
/// Everything after the first mixer runs here, whatever the caller's rate.
/// The engine feeds 48 kHz and the constants below were once tuned on 8 kHz
/// test signals and applied per *sample*, so every time constant in the
/// receiver was six times too fast on the air — the references, the level
/// peak and the AFC all chased noise. Working at one rate removes the class
/// of fault rather than the instances.
const WORK_RATE_HZ: f64 = 8_000.0;

/// How hard an observed transition pulls the bit clock.
const CLOCK_TRACK_GAIN: f32 = 0.05;

/// AFC loop gain per working sample at [`WORK_RATE_HZ`], and the widest tuning
/// error it will chase.
///
/// The range is far wider than RTTY's ±90 Hz, and deliberately so. RTTY is
/// tuned by the operator to a station they can hear and adjust; NAVTEX is a
/// published service on assigned frequencies that every published tuning
/// instruction rounds differently — 516.5 kHz, 516.6 kHz, 1700 Hz below the
/// channel — so the dial is routinely a couple of hundred hertz from where the
/// decoder expects the tones. Issue #608, reported as a strong steady signal
/// decoding to mostly asterisks while fldigi read it cleanly.
const AFC_GAIN: f32 = 3.0e-4;
/// ±350 Hz, which covers every published NAVTEX dial figure and then some.
const AFC_LIMIT_HZ: f32 = 350.0;
/// AFC needs the integrator settled on a single tone: its phase means nothing
/// while the window straddles a transition. A quarter of a bit excludes
/// exactly that.
const AFC_SETTLE: f32 = 0.25;
/// How far apart the two tones must be, against the signal level, before the
/// AFC believes it is looking at one of them. Keeps the loop out of noise.
const AFC_SEP_MIN: f32 = 0.3;
/// The coarse estimate is taken over this many bits — half a second.
const COARSE_BLOCK_BITS: f32 = 50.0;
/// Tuning errors below this are the fine loop's; the coarse estimate leaves
/// them alone, which is also what keeps it still on an empty channel.
const COARSE_MIN_HZ: f32 = 40.0;
/// How much of a coarse error is taken out in one step.
const COARSE_STEP: f32 = 0.7;

/// Half-width of the front-end band-pass, in Hz.
///
/// Both tones, the keying sidebands of a 100-baud square wave, and the whole
/// AFC pull-in range with a margin: a filter narrower than the loop's range
/// would be discarding the outer tone of a signal the loop is still trying to
/// find.
fn front_end_hz() -> f64 {
    f64::from(NAVTEX_SHIFT_HZ / 2.0 + AFC_LIMIT_HZ + 20.0) + 2.0 * NAVTEX_BAUD
}

// ───────────────────── soft decisions and the framer ─────────────────────

/// Character pairs the phase search looks at: sixteen characters, a little
/// over two seconds of signal.
const SYNC_SLOTS: usize = 16;
/// Mean fit over those pairs before the stream is declared in phase.
///
/// See [`decide`] for what the fit is. A real signal sits near 1.0 and noise
/// near 0.6, and *that* is the squelch: the receiver used to lock on about
/// one window of noise in twenty, several times a minute, and then hold the
/// lock for as long as the noise lasted because a random seven bits is a legal
/// code a quarter of the time.
const Q_ACQUIRE: f32 = 0.86;
/// A pair fitting worse than this is not a character.
const Q_BAD: f32 = 0.74;
/// Pairs of the search window allowed to be bad.
const SYNC_BAD_MAX: usize = 2;
/// The phase is given up when this many of the last [`RECENT_SLOTS`] pairs
/// were bad…
const DROP_BAD: usize = 4;
/// …out of this many.
const RECENT_SLOTS: usize = 6;
/// Characters held back before they are printed, so that when the phase is
/// given up the noise decoded while that was being noticed can be taken back
/// instead of printed. As long as the window the loss is judged over, so all
/// of it is still in hand; an idle pair releases everything before it at
/// once, so the end of a message is not kept waiting.
const HOLD_SLOTS: usize = RECENT_SLOTS;
/// A character whose decision was closer than this is printed as `*`.
///
/// In units of the mean soft-bit size: 8 is a clean character, 4 one with a
/// single bit wrong in one copy. Set on the Bermuda recording, where raising
/// it turned right characters into `*` faster than it caught wrong ones.
const LOST_CONF: f32 = 0.6;
/// A fade this long, in character pairs, is bridged with `*` when the same
/// transmission comes back on the same character phase, so a message keeps its
/// length and its holes show.
const GAP_SLOTS: usize = 32;
/// Soft bits kept: the search window, the longest gap bridged, and the reach
/// of one pair.
const HISTORY_BITS: usize = (SYNC_SLOTS + GAP_SLOTS + 2) * PAIR_BITS + FEC_BITS + BITS;

/// The state of one character slot, for the panel's quality readout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CharSource {
    /// The leading copy was a legal code, and the one that was read.
    Direct,
    /// It was not: the character came out of the two copies taken together.
    Repeat,
    /// Neither copy, nor both together, said what it was: it stands in the
    /// text as `*`.
    Lost,
}

/// What one pair of copies turned out to be.
#[derive(Debug, Clone, Copy, PartialEq)]
enum Decided {
    /// A character or a control code.
    Code(u8),
    /// The phasing pair a transmitter idles on.
    Idle,
}

/// One decided pair.
#[derive(Debug, Clone, Copy)]
struct Slot {
    what: Decided,
    /// How well the fourteen soft bits fit what was decided, `0..=1`.
    q: f32,
    /// How far ahead of the next-best reading that was.
    conf: f32,
    /// Whether the leading copy alone, sliced hard, already said the same.
    direct: bool,
}

/// `+1` / `-1` for each bit of `code`, least significant first.
fn signs(code: u8) -> [f32; BITS] {
    let mut s = [0.0; BITS];
    for (i, v) in s.iter_mut().enumerate() {
        *v = if code & (1 << i) != 0 { 1.0 } else { -1.0 };
    }
    s
}

/// Decide one character from the soft bits of its two copies.
///
/// This is the whole of the error correction, and it replaces "take the first
/// copy if it is a legal code, else the second". That rule threw away half of
/// what the time diversity sends: a copy with *two* bits wrong is a legal code
/// one time in three and was printed as it stood (`MMRINE`), and two copies
/// each with one bit wrong was a lost character (`M*RINE`) although between
/// them they nearly always say what was sent.
///
/// Here the two copies are added bit for bit — a copy received in a fade is
/// smaller, so it counts for less — and the answer is the legal code nearest
/// the sum. For a constant-ratio alphabet that is simply the four largest of
/// the seven sums, which is why no table search is needed. The one pair that
/// legitimately differs, the idle a transmitter sends between messages, is
/// tried as a reading of its own.
fn decide(a: &[f32; BITS], b: &[f32; BITS]) -> Slot {
    let mut s = [0.0f32; BITS];
    let mut norm = 0.0f32;
    for i in 0..BITS {
        s[i] = a[i] + b[i];
        norm += a[i].abs() + b[i].abs();
    }
    if norm < 1e-6 {
        return Slot { what: Decided::Idle, q: 0.0, conf: 0.0, direct: false };
    }
    let mut idx = [0usize, 1, 2, 3, 4, 5, 6];
    idx.sort_by(|&x, &y| s[y].total_cmp(&s[x]));
    let mut code = 0u8;
    let mut m_char = 0.0f32;
    for (rank, &i) in idx.iter().enumerate() {
        if rank < 4 {
            code |= 1 << i;
            m_char += s[i];
        } else {
            m_char -= s[i];
        }
    }
    let gap = s[idx[3]] - s[idx[4]];

    let (rep, alpha) = (signs(CODE_REP), signs(CODE_ALPHA));
    let mut m_idle = 0.0f32;
    for i in 0..BITS {
        m_idle += a[i] * rep[i] + b[i] * alpha[i];
    }

    // The leading copy on its own, sliced hard: what the old receiver read.
    let mut hard = 0u8;
    for (i, &v) in a.iter().enumerate() {
        if v > 0.0 {
            hard |= 1 << i;
        }
    }
    let unit = norm / (2 * BITS) as f32;
    // Idle against a phasing code is not a doubt worth marking: neither
    // prints. It is also exactly what the last pairs of a transmission look
    // like, with the leading copy still on the air and the repeat already in
    // the noise — and a `*` for each of those was a tail on every message.
    let silent = matches!(code, CODE_ALPHA | CODE_BETA | CODE_REP);
    if m_idle > m_char {
        Slot {
            what: Decided::Idle,
            q: m_idle / norm,
            conf: if silent { 2.0 * BITS as f32 } else { (m_idle - m_char) / unit },
            direct: hard == CODE_REP,
        }
    } else {
        let over_idle = if silent { f32::MAX } else { m_char - m_idle };
        Slot {
            what: Decided::Code(code),
            q: m_char / norm,
            conf: (2.0 * gap).min(over_idle) / unit,
            direct: hard == code,
        }
    }
}

/// A character decoded and not yet printed — see [`HOLD_SLOTS`].
struct Held {
    /// Bit index of the leading copy.
    at: u64,
    text: Option<char>,
    source: Option<CharSource>,
    good: bool,
    /// The shift before this character, so taking it back restores it.
    figures_before: bool,
}

/// The soft bits, the character phase found in them, and the text.
struct Framer {
    /// Soft-bit history, newest last. Positive is a mark bit.
    bits: VecDeque<f32>,
    /// Bits seen since the receiver started, so a latched character phase can
    /// be held as an absolute position rather than an index into a moving
    /// window.
    seen: u64,
    /// Bit index of the next leading copy to decode, once in phase.
    next_at: Option<u64>,
    /// Bit index past the last slot turned into text, so a re-sync cannot
    /// read the same bits twice.
    emitted_to: u64,
    held: VecDeque<Held>,
    recent: VecDeque<bool>,
    figures: bool,
    last_source: CharSource,
    direct: u64,
    repaired: u64,
    lost: u64,
}

impl Framer {
    fn new() -> Self {
        Framer {
            bits: VecDeque::with_capacity(HISTORY_BITS + BITS),
            seen: 0,
            next_at: None,
            emitted_to: 0,
            held: VecDeque::new(),
            recent: VecDeque::new(),
            figures: false,
            last_source: CharSource::Direct,
            direct: 0,
            repaired: 0,
            lost: 0,
        }
    }

    fn in_sync(&self) -> bool {
        self.next_at.is_some()
    }

    /// Index of the oldest bit still held.
    fn base(&self) -> u64 {
        self.seen - self.bits.len() as u64
    }

    /// The seven soft bits at absolute index `at`, if they are all held.
    fn soft_at(&self, at: u64) -> Option<[f32; BITS]> {
        let base = self.base();
        if at < base || at + BITS as u64 > self.seen {
            return None;
        }
        let off = (at - base) as usize;
        let mut v = [0.0; BITS];
        for (i, x) in v.iter_mut().enumerate() {
            *x = self.bits[off + i];
        }
        Some(v)
    }

    /// The pair whose leading copy starts at `at`.
    fn slot_at(&self, at: u64) -> Option<Slot> {
        let a = self.soft_at(at)?;
        let b = self.soft_at(at + FEC_BITS as u64)?;
        Some(decide(&a, &b))
    }

    /// One soft bit into the history, and whatever that completes.
    fn push_bit(&mut self, soft: f32, out: &mut String) {
        self.bits.push_back(soft);
        self.seen += 1;
        while self.bits.len() > HISTORY_BITS {
            self.bits.pop_front();
        }
        if self.next_at.is_none() {
            self.acquire(out);
        }
        while let Some(at) = self.next_at {
            if self.seen < at + (FEC_BITS + BITS) as u64 {
                break;
            }
            self.step(at, out);
        }
    }

    /// Look for the character phase, and on finding it catch up over what the
    /// window already holds — a transmission's first characters are inside it,
    /// and a decoder that started from *now* would eat the `ZCZC` every time.
    fn acquire(&mut self, out: &mut String) {
        let Some((first, last)) = self.find_phase() else { return };
        let pair = PAIR_BITS as u64;
        // Not from a slot that is not a character: the window is allowed a
        // couple, and they are the noise in front of the signal.
        let mut start = first;
        while start < last && self.slot_at(start).is_none_or(|s| s.q < Q_BAD) {
            start += pair;
        }
        if self.emitted_to > 0
            && start > self.emitted_to
            && (start - self.emitted_to) % pair == 0
            && (start - self.emitted_to) / pair <= GAP_SLOTS as u64
            && self.emitted_to >= self.base()
        {
            // The same transmission back after a fade: read the gap too, so
            // what was lost in it is marked rather than silently missing.
            let mut at = self.emitted_to;
            while at < start {
                if let Some(slot) = self.slot_at(at) {
                    let h = self.read(at, slot);
                    self.print(h, out);
                }
                at += pair;
            }
        } else {
            // Never back over bits already turned into text: that would print
            // the last few characters twice, put a second `ZCZC` in the
            // stream and frame a message out of the middle of one.
            while start < self.emitted_to {
                start += pair;
            }
        }
        self.recent.clear();
        self.next_at = Some(start);
        self.emitted_to = start;
    }

    /// Find the character phase in the bits held.
    ///
    /// Fourteen candidates, not seven: the phase has to say which slot is a
    /// leading copy and which is a repeat as well as where a character starts.
    /// A wrong one fits badly because its two "copies" are different
    /// characters, and noise fits badly at all fourteen.
    ///
    /// Returns the bit index of the first and the last leading copy of the
    /// window on the winning phase.
    fn find_phase(&self) -> Option<(u64, u64)> {
        let reach = (FEC_BITS + BITS) as u64;
        let span = ((SYNC_SLOTS - 1) * PAIR_BITS) as u64;
        if (self.bits.len() as u64) < span + reach + PAIR_BITS as u64 {
            return None;
        }
        let mut best: Option<(f32, u64)> = None;
        for off in 0..PAIR_BITS as u64 {
            let last = self.seen - reach - off;
            let mut sum = 0.0f32;
            let mut bad = 0usize;
            for k in 0..SYNC_SLOTS as u64 {
                let Some(slot) = self.slot_at(last - k * PAIR_BITS as u64) else {
                    bad = usize::MAX;
                    break;
                };
                sum += slot.q;
                if slot.q < Q_BAD {
                    bad += 1;
                    if bad > SYNC_BAD_MAX {
                        break;
                    }
                }
            }
            if bad > SYNC_BAD_MAX {
                continue;
            }
            let mean = sum / SYNC_SLOTS as f32;
            if mean >= Q_ACQUIRE && best.is_none_or(|(b, _)| mean > b) {
                best = Some((mean, last));
            }
        }
        best.map(|(_, last)| (last - span, last))
    }

    /// Turn a decided pair into a character, moving the shift as it goes.
    fn read(&mut self, at: u64, slot: Slot) -> Held {
        let figures_before = self.figures;
        let good = slot.q >= Q_BAD;
        if !good || slot.conf < LOST_CONF {
            // A hole, marked rather than hidden: a warning with a character
            // missing has to look like one.
            return Held {
                at,
                text: Some('*'),
                source: Some(CharSource::Lost),
                good,
                figures_before,
            };
        }
        let source = if slot.direct { CharSource::Direct } else { CharSource::Repeat };
        let text = match slot.what {
            Decided::Idle => None,
            Decided::Code(CODE_LTRS) => {
                self.figures = false;
                None
            }
            Decided::Code(CODE_FIGS) => {
                self.figures = true;
                None
            }
            // The idle and phasing codes carry no text. `CHAR32` is the
            // repetition signal and `BETA` the other half of the phasing
            // pair; neither prints.
            Decided::Code(CODE_ALPHA | CODE_BETA | CODE_REP | CODE_CHAR32) => None,
            Decided::Code(code) => code_to_char(code, self.figures),
        };
        // The idle is not a character and is not counted as one: a receiver
        // left on a quiet channel would otherwise run the "good" figure up by
        // seven a second while nothing was being sent.
        let source = (slot.what != Decided::Idle).then_some(source);
        Held { at, text, source, good, figures_before }
    }

    fn print(&mut self, h: Held, out: &mut String) {
        if let Some(source) = h.source {
            self.last_source = source;
            match source {
                CharSource::Direct => self.direct += 1,
                CharSource::Repeat => self.repaired += 1,
                CharSource::Lost => self.lost += 1,
            }
        }
        if let Some(ch) = h.text {
            out.push(ch);
        }
    }

    /// Decode the pair at `at`, in phase.
    fn step(&mut self, at: u64, out: &mut String) {
        let pair = PAIR_BITS as u64;
        let Some(slot) = self.slot_at(at) else {
            // Cannot happen while the history is longer than a pair's reach;
            // if it does, the phase is not worth keeping.
            self.next_at = None;
            return;
        };
        let idle = slot.what == Decided::Idle && slot.q >= Q_BAD && slot.conf >= LOST_CONF;
        let h = self.read(at, slot);
        self.recent.push_back(h.good);
        while self.recent.len() > RECENT_SLOTS {
            self.recent.pop_front();
        }
        self.held.push_back(h);
        self.next_at = Some(at + pair);
        self.emitted_to = at + pair;

        if self.recent.iter().filter(|g| !**g).count() >= DROP_BAD {
            // The signal has gone, or the phase was never right. Take back
            // what was decoded while that was being found out, from the first
            // pair that did not fit: it is noise, and printing it is what
            // filled the screen after every message. If it was a fade and the
            // transmission comes back, `acquire` reads these again.
            while self.held.front().is_some_and(|h| h.good) {
                let h = self.held.pop_front().expect("checked above");
                self.print(h, out);
            }
            if let Some(h) = self.held.front() {
                self.figures = h.figures_before;
                self.emitted_to = h.at;
            }
            self.held.clear();
            self.next_at = None;
            self.recent.clear();
            return;
        }
        let keep = if idle { 0 } else { HOLD_SLOTS };
        while self.held.len() > keep {
            let h = self.held.pop_front().expect("length checked above");
            self.print(h, out);
        }
    }
}

/// A NAVTEX / SITOR-B receiver: audio in, text out.
pub struct NavtexRx {
    rate: f64,
    /// Input samples averaged into one working sample — see [`WORK_RATE_HZ`].
    decim: usize,
    /// The working rate, `rate / decim`.
    fs: f32,
    center_hz: f32,
    reverse: bool,

    // Front end.
    ph: f32,
    ph_inc: f32,
    dec_acc: Complex32,
    dec_n: usize,
    lpf: ComplexFir,
    tone_ph: f32,
    tone_inc: f32,
    mark_int: Integrator,
    space_int: Integrator,
    /// The louder tone's recent level: the scale the soft bits are in.
    level: f32,
    mag: f32,

    /// AFC: the residual tuning error it has found so far, folded into
    /// `ph_inc` — see [`NavtexRx::afc_offset_hz`].
    afc_hz: f32,
    afc_enabled: bool,
    /// The winning branch's integrator output last sample, and which branch
    /// won. Their phase difference is the residual frequency, already averaged
    /// over a bit, so no separate discriminator is needed — the same trick
    /// RTTY uses.
    prev_win: Complex32,
    prev_win_mark: bool,
    have_win: bool,
    /// Samples the same branch has gone on winning, for [`AFC_SETTLE`].
    win_run: f32,
    /// The coarse estimate in progress — see [`NavtexRx::coarse_afc`].
    coarse_acc: Complex32,
    coarse_prev: Complex32,
    coarse_n: usize,

    // Bit clock.
    spb: f32,
    clk: f32,
    last_bit: bool,
    have_last: bool,

    framer: Framer,
}

impl NavtexRx {
    pub fn new(rate: f64) -> Self {
        let decim = ((rate / WORK_RATE_HZ).floor() as usize).max(1);
        let fs = rate / decim as f64;
        let spb = fs / NAVTEX_BAUD;
        let mut rx = NavtexRx {
            rate,
            decim,
            fs: fs as f32,
            center_hz: NAVTEX_CENTER_HZ,
            reverse: false,
            ph: 0.0,
            ph_inc: 0.0,
            dec_acc: Complex32::new(0.0, 0.0),
            dec_n: 0,
            // Wide enough for both tones, the keying sidebands of a 100-baud
            // square wave, and the whole AFC pull-in range. The matched
            // filters below do the real noise rejection, so all this has to
            // do is keep neighbours out.
            lpf: ComplexFir::new(bandpass_taps(129, -front_end_hz(), front_end_hz(), fs)),
            tone_ph: 0.0,
            tone_inc: 0.0,
            mark_int: Integrator::new(spb.round() as usize),
            space_int: Integrator::new(spb.round() as usize),
            level: 0.0,
            mag: 0.0,
            afc_hz: 0.0,
            afc_enabled: true,
            prev_win: Complex32::new(0.0, 0.0),
            prev_win_mark: false,
            have_win: false,
            win_run: 0.0,
            coarse_acc: Complex32::new(0.0, 0.0),
            coarse_prev: Complex32::new(0.0, 0.0),
            coarse_n: 0,
            spb: spb as f32,
            clk: 0.0,
            last_bit: false,
            have_last: false,
            framer: Framer::new(),
        };
        rx.retune();
        rx
    }

    /// Swap the sense of the tones, for a signal received on the other
    /// sideband. The same control RTTY calls Reverse, and needed for the same
    /// reason.
    pub fn set_reverse(&mut self, on: bool) {
        self.reverse = on;
    }

    #[must_use]
    pub fn reverse(&self) -> bool {
        self.reverse
    }

    fn retune(&mut self) {
        self.ph_inc = std::f32::consts::TAU * (self.center_hz + self.afc_hz) / self.rate as f32;
        self.tone_inc = std::f32::consts::TAU * (NAVTEX_SHIFT_HZ / 2.0) / self.fs;
    }

    /// Move the expected audio centre. The shift is fixed by the standard, but
    /// the dial's idea of where that is moves with the cursor, so a retune
    /// drops the AFC pull: the loop then re-finds the signal from wherever the
    /// operator has just put it.
    pub fn set_center_hz(&mut self, hz: f32) {
        self.center_hz = hz;
        self.afc_hz = 0.0;
        self.retune();
    }

    /// Enable automatic frequency correction. On by default; turning it off
    /// pins the decoder to exactly where the operator put the cursor.
    pub fn set_afc(&mut self, on: bool) {
        self.afc_enabled = on;
        if !on {
            self.afc_hz = 0.0;
            self.retune();
        }
    }

    /// Current AFC pull in Hz — how far off the cursor the signal actually is.
    ///
    /// Read this before telling anyone to retune: on this mode the answer is
    /// usually that the dial is right and the expected figure is the thing to
    /// question.
    #[must_use]
    pub fn afc_offset_hz(&self) -> f32 {
        self.afc_hz
    }

    /// Restore a previously found AFC pull. Called after the receiver is
    /// created; the loop then starts from where it left off.
    pub fn set_afc_hz(&mut self, hz: f32) {
        self.afc_hz = hz.clamp(-AFC_LIMIT_HZ, AFC_LIMIT_HZ);
        self.retune();
    }

    /// Signal level at the two tones, for a squelch or a meter.
    #[must_use]
    pub fn magnitude(&self) -> f32 {
        self.mag
    }

    /// Whether the character phase is latched — a signal is being read.
    #[must_use]
    pub fn in_sync(&self) -> bool {
        self.framer.in_sync()
    }

    /// How the last character was obtained.
    #[must_use]
    pub fn last_source(&self) -> CharSource {
        self.framer.last_source
    }

    /// `(read from the leading copy, recovered from the two together, lost)`
    /// since the receiver started — the only honest quality figure a mode
    /// with no CRC has. Characters only: the idle between messages is not
    /// counted.
    #[must_use]
    pub fn counts(&self) -> (u64, u64, u64) {
        (self.framer.direct, self.framer.repaired, self.framer.lost)
    }

    /// Feed audio; returns whatever text it completed.
    pub fn process(&mut self, audio: &[f32]) -> String {
        let mut out = String::new();
        let mut mixed = Vec::with_capacity(audio.len() / self.decim + 1);
        for &a in audio {
            self.dec_acc += Complex32::new(a * self.ph.cos(), -a * self.ph.sin());
            self.ph += self.ph_inc;
            if self.ph > std::f32::consts::TAU {
                self.ph -= std::f32::consts::TAU;
            } else if self.ph < 0.0 {
                self.ph += std::f32::consts::TAU;
            }
            self.dec_n += 1;
            if self.dec_n == self.decim {
                mixed.push(self.dec_acc / self.decim as f32);
                self.dec_acc = Complex32::new(0.0, 0.0);
                self.dec_n = 0;
            }
        }
        let mut bb = Vec::with_capacity(mixed.len());
        self.lpf.process(&mixed, &mut bb);
        for z in bb {
            self.coarse_afc(z);
            self.sample(z, &mut out);
        }
        out
    }

    /// The coarse half of the AFC: where the signal's energy is, on average.
    ///
    /// The loop in [`NavtexRx::sample`] measures one tone against the filter
    /// it fell into, which is exact near the right tuning and has a second,
    /// wrong, resting place 170 Hz away from it: the lower tone sitting in the
    /// upper filter, turning at nothing, with the loop perfectly content. A
    /// dial a couple of hundred hertz off lands there as often as not.
    ///
    /// The mean frequency of the whole band has no such place. Every character
    /// is four marks and three spaces, so an FSK signal's mean is a known
    /// twelve hertz to one side of its centre; half a second of it says how
    /// far off the centre is, and anything too far for the fine loop is
    /// stepped out at once. Noise is symmetric about the centre and reads
    /// zero, so on an empty channel this does nothing.
    fn coarse_afc(&mut self, z: Complex32) {
        self.coarse_acc += z * self.coarse_prev.conj();
        self.coarse_prev = z;
        self.coarse_n += 1;
        if (self.coarse_n as f32) < COARSE_BLOCK_BITS * self.spb {
            return;
        }
        let acc = std::mem::replace(&mut self.coarse_acc, Complex32::new(0.0, 0.0));
        self.coarse_n = 0;
        if !self.afc_enabled || acc.norm_sqr() < 1e-24 {
            return;
        }
        let lean = NAVTEX_SHIFT_HZ / 2.0 / BITS as f32;
        let lean = if self.reverse { -lean } else { lean };
        let off = acc.arg() * self.fs / std::f32::consts::TAU - lean;
        if off.abs() > COARSE_MIN_HZ {
            self.afc_hz = (self.afc_hz + COARSE_STEP * off).clamp(-AFC_LIMIT_HZ, AFC_LIMIT_HZ);
            self.retune();
        }
    }

    /// One working sample: the two matched filters, the decision between
    /// them, the AFC and the bit clock.
    fn sample(&mut self, z: Complex32, out: &mut String) {
        let (s, c) = (self.tone_ph.sin(), self.tone_ph.cos());
        self.tone_ph += self.tone_inc;
        if self.tone_ph > std::f32::consts::TAU {
            self.tone_ph -= std::f32::consts::TAU;
        }
        let m_c = self.mark_int.push(z * Complex32::new(c, -s));
        let sp_c = self.space_int.push(z * Complex32::new(c, s));
        let m = m_c.norm();
        let sp = sp_c.norm();
        // Time constants in bits, so they are the same on the air at any rate.
        let per_bit = 1.0 / self.spb;
        self.mag += 1.6 * per_bit * (m + sp - self.mag);

        // The plain difference of the two tones. Measuring each against its
        // own recent peak instead (the RTTY receiver's trick against selective
        // fading) was tried on the Bermuda recording and doubled the errors:
        // at 518 kHz the two tones fade together, and the peaks only added
        // their own noise to the decision.
        let d = m - sp;

        // The scale: up in a couple of bits, down over a few seconds, so the
        // copy of a character that arrived in a fade is the smaller of the
        // two when they are added — see [`decide`].
        let pk = m.max(sp);
        let k = if pk > self.level { 0.5 * per_bit } else { per_bit / 300.0 };
        self.level += k * (pk - self.level);
        let soft = if self.level > 1e-12 { (d / self.level).clamp(-1.0, 1.0) } else { 0.0 };
        let mark = d > 0.0;

        // AFC. The winning branch's integrator output is a phasor turning at
        // exactly the residual tuning error, already averaged over a bit, so
        // its phase advance *is* the error in Hz.
        if self.have_win && mark == self.prev_win_mark {
            self.win_run += 1.0;
        } else {
            self.win_run = 0.0;
        }
        let win = if mark { m_c } else { sp_c };
        if self.afc_enabled && soft.abs() > AFC_SEP_MIN && self.win_run > AFC_SETTLE * self.spb {
            let dz = win * self.prev_win.conj();
            if dz.norm_sqr() > 1e-24 {
                let hz_per_rad = self.fs / std::f32::consts::TAU;
                let err = (dz.arg() * hz_per_rad).clamp(-AFC_LIMIT_HZ, AFC_LIMIT_HZ);
                let gain = AFC_GAIN * (WORK_RATE_HZ as f32 / self.fs);
                let was = self.afc_hz;
                self.afc_hz = (self.afc_hz + gain * err).clamp(-AFC_LIMIT_HZ, AFC_LIMIT_HZ);
                if self.afc_hz != was {
                    self.retune();
                }
            }
        }
        // Tracked whether or not the loop is running, so turning AFC back on
        // starts from a current phasor instead of one from before it was
        // switched off.
        self.prev_win = win;
        self.prev_win_mark = mark;
        self.have_win = true;

        // The bit clock. Synchronous: there is no start bit to acquire on, so
        // it free-runs and every observed transition nudges it towards putting
        // the strobe where the matched filter has just finished a symbol — a
        // half-bit after the crossover. The soft bit is the filter's output at
        // that instant, which is the whole symbol and nothing of its
        // neighbours.
        if self.have_last && mark != self.last_bit {
            let mut e = 0.5 - self.clk;
            if e > 0.5 {
                e -= 1.0;
            } else if e < -0.5 {
                e += 1.0;
            }
            self.clk = (self.clk + CLOCK_TRACK_GAIN * e).rem_euclid(1.0);
        }
        self.last_bit = mark;
        self.have_last = true;
        self.clk += per_bit;
        if self.clk >= 1.0 {
            self.clk -= 1.0;
            let bit = if self.reverse { -soft } else { soft };
            self.framer.push_bit(bit, out);
        }
    }
}

/// Render text as a SITOR-B bit stream, for tests and for anyone building a
/// signal to check a receiver against.
///
/// The arrangement is the one the receiver's phase search is looking for: a
/// leading copy in every even slot and, five slots later, its repeat. Slots
/// with nothing to repeat carry the alternating phasing pair, which is what a
/// transmitter idles with.
#[must_use]
pub fn encode_bits(text: &str) -> Vec<bool> {
    // The character stream, with shifts inserted where the register changes.
    let mut codes: Vec<u8> = Vec::new();
    let mut figures = false;
    for ch in text.chars() {
        let shared = matches!(ch, ' ' | '\r' | '\n');
        if let Some(c) = figure_to_code(ch).filter(|_| !shared && char_to_code(ch).is_none()) {
            if !figures {
                codes.push(CODE_FIGS);
                figures = true;
            }
            codes.push(c);
        } else if let Some(c) = char_to_code(ch) {
            if figures && !shared {
                codes.push(CODE_LTRS);
                figures = false;
            }
            codes.push(c);
        }
    }
    // Idle in front so a receiver has something to run its clock on, and
    // behind so the last characters' repeats are actually sent.
    let lead = 20usize;
    let total = lead + 2 * codes.len() + FEC_CHARS + 8;
    let mut slots: Vec<Option<u8>> = vec![None; total];
    for (k, &code) in codes.iter().enumerate() {
        slots[lead + 2 * k] = Some(code);
        slots[lead + 2 * k + FEC_CHARS] = Some(code);
    }
    let mut bits = Vec::with_capacity(total * BITS);
    for (i, slot) in slots.iter().enumerate() {
        // The phasing pair on anything unfilled: alpha and rep alternating,
        // which is what makes the idle stream *not* look like a message to the
        // phase search — no slot agrees with the one five later.
        let code = slot.unwrap_or(if i % 2 == 0 { CODE_REP } else { CODE_ALPHA });
        for b in 0..BITS {
            bits.push(code & (1 << b) != 0);
        }
    }
    bits
}

/// An FSK audio signal carrying `bits` at the NAVTEX rate and shift.
///
/// A mark bit is the **upper** tone. ITU-R M.625 sends the code's B elements —
/// the ones — on the higher frequency, and a receiver in upper sideband keeps
/// that order, so a signal built here decodes with Reverse off. That is also
/// the reasoning behind the default; it has not been checked against a real
/// coast station.
#[must_use]
pub fn synth(bits: &[bool], rate: f64, center_hz: f32, amplitude: f32) -> Vec<f32> {
    let spb = (rate / NAVTEX_BAUD) as usize;
    let mut out = Vec::with_capacity(bits.len() * spb);
    let mut ph = 0.0f32;
    for &b in bits {
        let hz = center_hz + if b { NAVTEX_SHIFT_HZ / 2.0 } else { -NAVTEX_SHIFT_HZ / 2.0 };
        let inc = std::f32::consts::TAU * hz / rate as f32;
        for _ in 0..spb {
            out.push(ph.cos() * amplitude);
            ph += inc;
            if ph > std::f32::consts::TAU {
                ph -= std::f32::consts::TAU;
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The alphabet is what makes the mode self-checking: every legal code has
    /// four mark bits of seven, all thirty-five such patterns are accounted
    /// for, and none is claimed twice.
    #[test]
    fn the_alphabet_is_a_constant_ratio_code() {
        let controls = [CODE_LTRS, CODE_FIGS, CODE_ALPHA, CODE_BETA, CODE_REP, CODE_CHAR32];
        let mut seen: Vec<u8> = ALPHABET.iter().map(|&(c, _, _)| c).collect();
        seen.extend_from_slice(&controls);
        for &c in &seen {
            assert!(is_valid_code(c), "0x{c:02x} is not a four-of-seven code");
        }
        seen.sort_unstable();
        let before = seen.len();
        seen.dedup();
        assert_eq!(before, seen.len(), "a code is used twice");
        // C(7,4) = 35: the alphabet uses the whole space, which is why an
        // unlisted valid code cannot exist.
        assert_eq!(seen.len(), 35);
        assert_eq!((0u8..128).filter(|&c| is_valid_code(c)).count(), 35);
    }

    /// Deterministic white noise, `amplitude` peak.
    fn noise(len: usize, amplitude: f32, seed: u64) -> Vec<f32> {
        let mut x = 0x2545_f491_4f6c_dd1du64 ^ seed.wrapping_mul(0x9e37_79b9_7f4a_7c15);
        (0..len)
            .map(|_| {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                ((x >> 40) as f32 / 8_388_608.0 - 1.0) * amplitude
            })
            .collect()
    }

    /// A channel with nobody on it prints nothing. Ten minutes of noise — the
    /// gap between two stations' slots, which is what a receiver left on
    /// 518 kHz hears most of the time. The receiver used to find a character
    /// phase in it several times a minute and print whatever the noise spelled
    /// (F4NRM: "sans signal il décode n'importe quoi").
    #[test]
    fn noise_alone_prints_nothing() {
        for (rate, seed) in [(8000.0, 1u64), (48_000.0, 2)] {
            let audio = noise((rate * 600.0) as usize, 0.3, seed);
            let mut rx = NavtexRx::new(rate);
            let mut got = String::new();
            for chunk in audio.chunks(2048) {
                got.push_str(&rx.process(chunk));
            }
            assert!(got.is_empty(), "at {rate} Hz noise decoded as {got:?}");
            assert_eq!(rx.counts(), (0, 0, 0), "noise was counted as characters");
        }
    }

    /// …and nothing after a message either: when the station stops, the text
    /// stops with it. Two minutes of the same noise the message arrived in.
    #[test]
    fn the_noise_after_a_message_prints_nothing() {
        let rate = 8000.0;
        let mut audio = vec![0.0f32; 8000];
        let msg = encode_bits("ZCZC FA12 GALE WARNING NNNN");
        audio.extend(synth(&msg, rate, NAVTEX_CENTER_HZ, 0.4));
        audio.extend(std::iter::repeat_n(0.0f32, 120 * 8000));
        for (a, n) in audio.iter_mut().zip(noise(121 * 8000 + 200_000, 0.1, 3)) {
            *a += n;
        }
        let mut rx = NavtexRx::new(rate);
        let mut got = String::new();
        for chunk in audio.chunks(512) {
            got.push_str(&rx.process(chunk));
        }
        let (_, tail) = got.split_once("NNNN").unwrap_or_else(|| panic!("no message in {got:?}"));
        assert!(got.contains("ZCZC FA12 GALE WARNING"), "decoded {got:?}");
        assert!(tail.trim().is_empty(), "printed {tail:?} after the station stopped");
        assert!(!rx.in_sync(), "still claiming a signal two minutes after it ended");
    }

    /// Soft bits for `code`, all `level` in size.
    fn soft(code: u8, level: f32) -> [f32; BITS] {
        signs(code).map(|s| s * level)
    }

    /// The two copies are read together, weighted by how well each arrived.
    /// These are the two failures of taking "the first copy that is a legal
    /// code": `M*RINE`, where each copy has a bit wrong and neither is legal,
    /// and `MMRINE`, where the leading copy has two wrong and happens to be
    /// another letter.
    #[test]
    fn two_damaged_copies_still_make_one_character() {
        let a_code = char_to_code('A').unwrap();
        let m_code = char_to_code('M').unwrap();

        // Each copy with a different bit wrong, and weak where it is wrong —
        // which is what a wrong bit is: one the noise only just tipped over.
        let mut a = soft(a_code, 1.0);
        let mut b = soft(a_code, 1.0);
        a[0] *= -0.3;
        b[4] *= -0.3;
        let hard = |v: &[f32; BITS]| {
            v.iter().enumerate().fold(0u8, |c, (i, &x)| if x > 0.0 { c | 1 << i } else { c })
        };
        assert!(!is_valid_code(hard(&a)) && !is_valid_code(hard(&b)), "the test damaged nothing");
        let slot = decide(&a, &b);
        assert_eq!(slot.what, Decided::Code(a_code), "two damaged copies lost the character");
        assert!(slot.conf >= LOST_CONF, "…or left it too doubtful to print: {}", slot.conf);

        // The leading copy received in a fade as a legal *wrong* letter, the
        // repeat clean: the repeat wins.
        let slot = decide(&soft(m_code, 0.25), &soft(a_code, 1.0));
        assert_eq!(slot.what, Decided::Code(a_code), "a faded wrong copy beat a clean right one");
        assert!(slot.conf >= LOST_CONF);
        assert!(!slot.direct, "it was not the leading copy that was read");

        // Two copies that flatly contradict each other at full strength are
        // not a character: neither can be preferred, and a guess printed as
        // text is worse than a hole.
        let slot = decide(&soft(m_code, 1.0), &soft(a_code, 1.0));
        assert!(slot.conf < LOST_CONF, "a coin toss was printed as a character");

        // The idle a transmitter sends between messages is neither.
        let slot = decide(&soft(CODE_REP, 1.0), &soft(CODE_ALPHA, 1.0));
        assert_eq!(slot.what, Decided::Idle);
    }

    /// A transmission that starts after a spell of silence decodes: a real slot
    /// is ten minutes of nothing and then a station, and a receiver that only
    /// worked from the first sample would never decode anything at all.
    #[test]
    fn a_signal_that_starts_after_silence_decodes() {
        let rate = 8000.0;
        let msg = "ZCZC FA12 GALE WARNING NNNN";
        let mut audio = vec![0.0f32; 8000];
        audio.extend(synth(&encode_bits(msg), rate, NAVTEX_CENTER_HZ, 0.4));
        let mut rx = NavtexRx::new(rate);
        let mut got = String::new();
        for chunk in audio.chunks(512) {
            got.push_str(&rx.process(chunk));
        }
        assert!(
            got.contains("GALE WARNING"),
            "decoded {got:?} sync={} counts={:?}",
            rx.in_sync(),
            rx.counts()
        );
    }

    /// Two slots with silence between them: a coast station transmits for ten
    /// minutes every four hours and its neighbours fill the gaps, so a receiver
    /// left on hears exactly this. The second must decode as well as the first.
    #[test]
    fn a_second_transmission_after_a_gap_decodes_too() {
        let rate = 8000.0;
        let mut audio = vec![0.0f32; 8000];
        audio.extend(synth(&encode_bits("ZCZC PA01 ONE NNNN"), rate, NAVTEX_CENTER_HZ, 0.4));
        audio.extend(std::iter::repeat_n(0.0f32, 8000));
        audio.extend(synth(&encode_bits("ZCZC PA02 TWO NNNN"), rate, NAVTEX_CENTER_HZ, 0.4));
        let mut rx = NavtexRx::new(rate);
        let mut got = String::new();
        for chunk in audio.chunks(512) {
            got.push_str(&rx.process(chunk));
        }
        assert!(got.contains("ZCZC PA01 ONE NNNN"), "first: {got:?}");
        assert!(got.contains("ZCZC PA02 TWO NNNN"), "second: {got:?}");
    }

    /// A clean signal decodes to the text that was sent.
    #[test]
    fn a_clean_broadcast_decodes() {
        let rate = 8000.0;
        let msg = "ZCZC FA12\r\nGALE WARNING\r\nNNNN";
        let bits = encode_bits(msg);
        let audio = synth(&bits, rate, NAVTEX_CENTER_HZ, 0.4);
        let mut rx = NavtexRx::new(rate);
        let mut got = String::new();
        for chunk in audio.chunks(512) {
            got.push_str(&rx.process(chunk));
        }
        assert!(rx.in_sync(), "the phase was never found");
        assert!(got.contains("ZCZC FA12"), "decoded {got:?}");
        assert!(got.contains("GALE WARNING"), "decoded {got:?}");
        assert!(got.contains("NNNN"), "decoded {got:?}");
        let (_, _, lost) = rx.counts();
        assert_eq!(lost, 0, "a clean signal lost {lost} characters");
    }

    /// …and it still decodes with the noise a 518 kHz signal actually arrives
    /// under. Not a sensitivity claim — a synthetic signal cannot make one
    /// (see the DSP notes in `validate-dsp-against-real-signals`) — but a
    /// decoder that only works on a noiseless tone is not one.
    /// SITOR-B is NAVTEX's broadcast mode, so the corpus file is the decoder's
    /// own mode and not a new one. The sample is only 8 s, which is part of one
    /// broadcast, so the score is the words it does carry.
    #[test]
    #[ignore]
    fn the_corpus_sitorb_sample_decodes() {
        let path = std::env::var("SDROXIDE_SITOR_SAMPLE").unwrap_or_else(|_| {
            concat!(env!("CARGO_MANIFEST_DIR"), "/../../samples/fldigi/SITORB.wav").to_string()
        });
        let mut reader = hound::WavReader::open(&path).expect("open");
        let spec = reader.spec();
        let audio: Vec<f32> =
            reader.samples::<i16>().map(|s| s.expect("s") as f32 / 32_768.0).collect();
        let mut rx = NavtexRx::new(spec.sample_rate as f64);
        let mut out = String::new();
        for chunk in audio.chunks(512) {
            out.push_str(&rx.process(chunk));
        }
        out.push_str(&rx.process(&vec![0.0f32; 16000]));
        eprintln!("SITOR-B: {} chars: {:?}", out.len(), out);
        for w in ["EAGLE", "LANDED", "SDRBROWN", "12345"] {
            eprintln!("  {w}: {}", out.contains(w));
        }
    }

    #[test]
    fn noise_does_not_stop_it() {
        let rate = 8000.0;
        let msg = "ZCZC PB07 NAVAREA ONE";
        let bits = encode_bits(msg);
        let mut audio = synth(&bits, rate, NAVTEX_CENTER_HZ, 0.4);
        // A deterministic pseudo-noise at a third of the signal's amplitude.
        let mut x = 0x2545_f491_4f6c_dd1du64;
        for a in &mut audio {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            *a += ((x >> 40) as f32 / 8_388_608.0 - 1.0) * 0.13;
        }
        let mut rx = NavtexRx::new(rate);
        let mut got = String::new();
        for chunk in audio.chunks(512) {
            got.push_str(&rx.process(chunk));
        }
        assert!(got.contains("NAVAREA ONE"), "decoded {got:?}");
    }

    /// The time diversity is the whole point: a burst that destroys the
    /// leading copy of a run of characters still comes back readable, because
    /// each of them is sent again five slots later.
    #[test]
    fn a_burst_over_the_leading_copies_is_repaired_from_the_repeats() {
        let rate = 8000.0;
        let msg = "ZCZC GB34 SECURITE";
        let mut bits = encode_bits(msg);
        // Corrupt one bit in every other character slot — the leading copies —
        // right through the message. A constant-ratio code cannot absorb a
        // single bit error, so every one of these is a hole the repeat has to
        // fill.
        let mut hit = 0;
        let mut slot = 0;
        while (slot + 1) * BITS <= bits.len() {
            if slot % 2 == 0 && slot > 40 && slot < 70 {
                let at = slot * BITS + 3;
                bits[at] = !bits[at];
                hit += 1;
            }
            slot += 1;
        }
        assert!(hit > 5, "the test corrupted nothing");
        let audio = synth(&bits, rate, NAVTEX_CENTER_HZ, 0.4);
        let mut rx = NavtexRx::new(rate);
        let mut got = String::new();
        for chunk in audio.chunks(512) {
            got.push_str(&rx.process(chunk));
        }
        assert!(got.contains("SECURITE"), "decoded {got:?}");
        let (_, repaired, lost) = rx.counts();
        assert!(repaired > 0, "nothing came from the repeats, so nothing was being tested");
        assert_eq!(lost, 0, "{lost} characters were lost that the repeat should have covered");
    }

    /// A receiver on the other sideband hears the tones swapped, and Reverse
    /// is what puts them back — the same control, and the same failure, as
    /// RTTY.
    #[test]
    fn the_reverse_control_recovers_an_inverted_signal() {
        let rate = 8000.0;
        let msg = "ZCZC ZZ01 TEST";
        let bits: Vec<bool> = encode_bits(msg).into_iter().map(|b| !b).collect();
        let audio = synth(&bits, rate, NAVTEX_CENTER_HZ, 0.4);

        let mut plain = NavtexRx::new(rate);
        let mut wrong = String::new();
        for chunk in audio.chunks(512) {
            wrong.push_str(&plain.process(chunk));
        }
        assert!(!wrong.contains("TEST"), "an inverted signal decoded anyway: {wrong:?}");

        let mut rx = NavtexRx::new(rate);
        rx.set_reverse(true);
        let mut got = String::new();
        for chunk in audio.chunks(512) {
            got.push_str(&rx.process(chunk));
        }
        assert!(got.contains("ZCZC ZZ01 TEST"), "decoded {got:?}");
    }

    /// Figures come back as figures: a warning is mostly positions and times,
    /// and a shift missed turns 51-03N into QA-ZDN.
    #[test]
    fn the_figure_shift_survives_the_round_trip() {
        let rate = 8000.0;
        let msg = "ZCZC OA01 5103N 00109E AT 1200 UTC";
        let bits = encode_bits(msg);
        let audio = synth(&bits, rate, NAVTEX_CENTER_HZ, 0.4);
        let mut rx = NavtexRx::new(rate);
        let mut got = String::new();
        for chunk in audio.chunks(512) {
            got.push_str(&rx.process(chunk));
        }
        assert!(got.contains("5103N 00109E AT 1200 UTC"), "decoded {got:?}");
    }

    /// The dial is a couple of hundred hertz off the tones and the message
    /// still arrives whole. This is the whole of issue #608, reported as a
    /// strong steady signal decoding to mostly asterisks while fldigi read the
    /// same signal cleanly: every published NAVTEX tuning figure rounds
    /// differently, and a matched-filter detector with no frequency tracking
    /// decodes the rounding error rather than the message.
    #[test]
    fn a_mistuned_dial_is_tracked_rather_than_decoded() {
        let rate = 8000.0;
        let msg = "ZCZC FA12 GALE WARNING NNNN WIND SPEED 55 KTS";
        let bits = encode_bits(msg);
        // Both directions, and inside the pull-in range at each end.
        for off in [-300.0f32, -150.0, 150.0, 300.0] {
            let audio = synth(&bits, rate, NAVTEX_CENTER_HZ + off, 0.4);
            let mut rx = NavtexRx::new(rate);
            let mut got = String::new();
            for chunk in audio.chunks(512) {
                got.push_str(&rx.process(chunk));
            }
            assert!(
                got.contains("GALE WARNING") && got.contains("WIND SPEED 55 KTS"),
                "off by {off} Hz the message was lost: {got:?}"
            );
            assert!(
                (rx.afc_offset_hz() - off).abs() < 30.0,
                "AFC found {:?} Hz for a signal {off} Hz off",
                rx.afc_offset_hz()
            );
        }
    }

    /// The loop reads the signal, not the clock: with it off the same mistuned
    /// signal is unreadable, which is what makes the test above a measurement
    /// of the AFC rather than of the widened filter.
    #[test]
    fn without_the_loop_the_same_mistuned_signal_is_lost() {
        let rate = 8000.0;
        let bits = encode_bits("ZCZC FA12 GALE WARNING NNNN");
        let audio = synth(&bits, rate, NAVTEX_CENTER_HZ + 300.0, 0.4);
        let mut rx = NavtexRx::new(rate);
        rx.set_afc(false);
        let mut got = String::new();
        for chunk in audio.chunks(512) {
            got.push_str(&rx.process(chunk));
        }
        assert!(
            !got.contains("GALE WARNING"),
            "a signal 300 Hz off decoded with no frequency tracking: {got:?}"
        );
        assert_eq!(rx.afc_offset_hz(), 0.0, "the loop moved with AFC off");
    }

    /// The wider front end is a price paid for the pull-in range, and a wider
    /// filter is more noise in principle. Measured against the pre-fix front
    /// end on the same noisy signals — 60 decodes each at 15, 12, 9 and 6 dB —
    /// it costs nothing: the message count is 60/48/35/29 against 59/55/37/30,
    /// so the wide one is ahead at every step but one and never behind by more
    /// than a single decode. Pinned as a floor rather than as the A/B it was
    /// measured from, because sixteen noisy decodes will not tie and the
    /// margin this leaves is wider than the difference it guards.
    #[test]
    fn the_wider_front_end_has_not_cost_the_tuned_case_its_sensitivity() {
        let rate = 8000.0;
        let bits = encode_bits("ZCZC PB07 NAVAREA ONE WIND 55 KTS");
        // Twelve decibels in band-to-band ratio, above the point where the mode
        // is used in anger.
        let noise = 0.4 * 10.0f32.powf(-12.0 / 20.0);
        let mut decoded = 0;
        for seed in 0..24u64 {
            let mut audio = synth(&bits, rate, NAVTEX_CENTER_HZ, 0.4);
            let mut x =
                0x2545_f491_4f6c_dd1du64.wrapping_add(seed.wrapping_mul(0x9e37_79b9_7f4a_7c15));
            for a in &mut audio {
                x ^= x << 13;
                x ^= x >> 7;
                x ^= x << 17;
                *a += ((x >> 40) as f32 / 8_388_608.0 - 1.0) * noise;
            }
            let mut rx = NavtexRx::new(rate);
            let mut got = String::new();
            for chunk in audio.chunks(512) {
                got.push_str(&rx.process(chunk));
            }
            if got.contains("NAVAREA ONE") {
                decoded += 1;
            }
        }
        assert!(
            decoded >= 21,
            "only {decoded} of 24 decodes at 12 dB — the wider front end cost the tuned case"
        );
    }

    /// Moving the cursor drops the pull, so a retune cannot leave the loop
    /// holding a correction for a frequency the operator has just changed. The
    /// loop then re-finds the signal from where the cursor now is.
    #[test]
    fn a_retune_drops_the_pull_and_the_loop_re_finds_the_signal() {
        let rate = 8000.0;
        let bits = encode_bits("ZCZC FA12 GALE WARNING NNNN WIND SPEED 55 KTS");
        let audio = synth(&bits, rate, NAVTEX_CENTER_HZ + 250.0, 0.4);
        let mut rx = NavtexRx::new(rate);
        let split = (audio.len() * 3 / 4) / 512 * 512;
        let mut got = String::new();
        for chunk in audio[..split].chunks(512) {
            got.push_str(&rx.process(chunk));
        }
        assert!(rx.afc_offset_hz() > 200.0, "the loop had not pulled in yet");

        rx.set_center_hz(NAVTEX_CENTER_HZ);
        assert_eq!(rx.afc_offset_hz(), 0.0, "a retune kept the old pull");
        for chunk in audio[split..].chunks(512) {
            got.push_str(&rx.process(chunk));
        }
        assert!(got.contains("GALE WARNING"), "lost across a retune: {got:?}");
        // The signal is 250 Hz off the new centre, so it must have found it again.
        assert!(
            rx.afc_offset_hz() > 200.0,
            "the loop did not re-lock after the retune: {:?}",
            rx.afc_offset_hz()
        );
    }

    /// The pull-in range is bounded, and the front-end filter is wide enough to
    /// hold a signal at the edge of it — those two numbers are the same design
    /// decision, so pin them together rather than letting one drift.
    #[test]
    fn the_front_end_covers_the_pull_in_range() {
        assert!(front_end_hz() > f64::from(AFC_LIMIT_HZ + NAVTEX_SHIFT_HZ));
    }
}
