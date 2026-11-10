//! DTMF — the eight touch-tone frequencies, as a decoder.
//!
//! fldigi's `src/dtmf/dtmf.cxx`: four row tones and four column tones, a Goertzel
//! filter on each, and the character is the strongest row paired with the
//! strongest column. The tone table is the telephone standard and the alphabet is
//! `123A456B789C*0#D`.
//!
//! Two things this does that the reference leaves to its squelch:
//!
//! * **A ratio test, not just a maximum.** Eight Goertzel outputs always have a
//!   largest one, so a plain argmax prints a digit for noise. A real pair stands
//!   well clear of the other six tones, and requiring that is what makes silence
//!   silent.
//! * **A debounce.** A digit is held for at least 40 ms and the frame is 30 ms, so
//!   a naive per-frame decoder prints every digit two or three times. One press is
//!   one character here.

use std::collections::VecDeque;

/// The four row frequencies, low to high.
const ROWS: [f64; 4] = [697.0, 770.0, 852.0, 941.0];
/// The four column frequencies, low to high. fldigi carries `1633` as the top
/// column; `1477` is the third, not the `1447` its own comment says.
const COLS: [f64; 4] = [1209.0, 1336.0, 1477.0, 1633.0];
/// Row-major, exactly fldigi's `rc[]`.
const CHARS: [char; 16] =
    ['1', '2', '3', 'A', '4', '5', '6', 'B', '7', '8', '9', 'C', '*', '0', '#', 'D'];

/// How much stronger the winning tone has to be than the next one in its group.
///
/// 5 is about 7 dB, which the tone spacing allows for: the frame is 30 ms, so the
/// Goertzel bin is 33 Hz wide and the rows are 73 Hz apart, and a real press
/// clears this by a wide margin while noise does not (measured: a 3× ratio let
/// one digit through in three seconds of noise).
const RATIO: f64 = 5.0;
/// How much of the frame's energy the pair has to carry to count at all.
const ENERGY_SHARE: f64 = 0.30;

/// Goertzel power of `f` over `x`.
fn goertzel(x: &[f32], f: f64, rate: f64) -> f64 {
    let n = x.len();
    if n == 0 {
        return 0.0;
    }
    let w = 2.0 * std::f64::consts::PI * f / rate;
    let coeff = 2.0 * w.cos();
    let (mut s1, mut s2) = (0.0f64, 0.0f64);
    for &v in x {
        let s0 = v as f64 + coeff * s1 - s2;
        s2 = s1;
        s1 = s0;
    }
    s1 * s1 + s2 * s2 - coeff * s1 * s2
}

/// One DTMF receiver.
///
/// Feed it audio at any rate; it takes 30 ms frames with a 15 ms hop and reports
/// each digit once, as the key is pressed and released.
pub struct DtmfRx {
    rate: f64,
    /// Samples per frame (30 ms) and per hop (half a frame).
    frame: usize,
    hop: usize,
    buf: VecDeque<f32>,
    /// The digit currently held down, if any.
    held: Option<char>,
    /// Frames since the last confident digit, so a press is only released after
    /// the tone has actually stopped.
    since: usize,
    mag: f32,
    snr: f32,
}

impl DtmfRx {
    pub fn new(rate: f64) -> Self {
        let frame = ((rate * 0.030).round() as usize).max(64);
        let hop = (frame / 2).max(1);
        DtmfRx { rate, frame, hop, buf: VecDeque::new(), held: None, since: 0, mag: 0.0, snr: 0.0 }
    }

    pub fn magnitude(&self) -> f32 {
        self.mag
    }

    /// The ratio the last frame's pair held over the rest, as a rough SNR. The
    /// panel shows it the way it shows every other decoder's.
    pub fn snr_db(&self) -> f32 {
        self.snr
    }

    /// Analyse the newest frame. Returns the digit if the pair is convincing.
    fn analyse(&mut self) -> Option<char> {
        if self.buf.len() < self.frame {
            return None;
        }
        let x: Vec<f32> = self.buf.iter().rev().take(self.frame).rev().copied().collect();
        let total: f64 =
            x.iter().map(|v| (*v as f64) * (*v as f64)).sum::<f64>() / self.frame as f64;
        self.mag = total.sqrt() as f32;
        let mut row = [0.0f64; 4];
        let mut col = [0.0f64; 4];
        for (i, f) in ROWS.iter().enumerate() {
            row[i] = goertzel(&x, *f, self.rate);
        }
        for (i, f) in COLS.iter().enumerate() {
            col[i] = goertzel(&x, *f, self.rate);
        }
        let (ri, rmax) = best(&row);
        let (ci, cmax) = best(&col);
        let rsecond = second(&row, ri);
        let csecond = second(&col, ci);
        // The pair against everything else in the frame, and each tone against its
        // own group's runner-up.
        let pair = rmax + cmax;
        let share = if total > 1e-20 {
            pair / (total * self.frame as f64 * (self.frame as f64) / 4.0)
        } else {
            0.0
        };
        let clear = rmax > RATIO * rsecond.max(1e-20) && cmax > RATIO * csecond.max(1e-20);
        let strong = pair > 1e-9 && share > ENERGY_SHARE;
        self.snr = if rsecond.max(csecond) > 0.0 {
            (10.0 * (rmax.min(cmax) / rsecond.max(csecond)).log10()) as f32
        } else {
            0.0
        };
        if clear && strong { Some(CHARS[ri * 4 + ci]) } else { None }
    }

    /// Feed audio; returns any digit decoded since the last call.
    pub fn process(&mut self, audio: &[f32]) -> String {
        self.buf.extend(audio.iter().copied());
        let mut out = String::new();
        while self.buf.len() >= self.frame {
            let digit = self.analyse();
            match digit {
                Some(d) => {
                    self.since = 0;
                    if self.held != Some(d) {
                        // A new press, or a different key: say it once.
                        self.held = Some(d);
                        out.push(d);
                    }
                }
                None => {
                    self.since += 1;
                    // Two clear frames without the pair is a release. One is not:
                    // a frame boundary can land between two halves of a digit.
                    if self.since >= 2 {
                        self.held = None;
                    }
                }
            }
            // Advance by one hop and drop what the frame no longer needs.
            for _ in 0..self.hop.min(self.buf.len()) {
                self.buf.pop_front();
            }
        }
        out
    }

    /// Let go of a held digit — the caller has stopped feeding audio.
    pub fn reset(&mut self) {
        self.buf.clear();
        self.held = None;
        self.since = 0;
    }
}

/// The DTMF transmitter: keys in, the standard's own tones out.
///
/// The inverse of [`DtmfRx`], and the telephone standard rather than a house
/// convention: a key is two equal-amplitude tones held for **100 ms** with
/// **50 ms** of silence between presses. The gap is not decoration — a held key
/// is one digit at the receiver, so two identical digits sent back to back would
/// be heard as one press.
///
/// Everything that is a key goes out; a space becomes a gap of the same length
/// so a typed " 123 " is spaced the way it was typed; anything else (letters past
/// `A`–`D`, punctuation) is dropped, because there is no tone for it to be. That
/// is quieter than inventing one and matches what a keypad can actually send.
pub struct DtmfTx {
    rate: f64,
    q: VecDeque<Option<char>>,
    out: VecDeque<f32>,
    sent: usize,
    total: usize,
}

/// How long a press is held, and how long the silence between presses is.
const PRESS_S: f64 = 0.100;
const GAP_S: f64 = 0.050;

impl DtmfTx {
    pub fn new(rate: f64) -> Self {
        DtmfTx { rate, q: VecDeque::new(), out: VecDeque::new(), sent: 0, total: 0 }
    }

    /// Queue keys. A space is queued as a silent press; anything else without a
    /// tone is dropped.
    pub fn push_text(&mut self, text: &str) {
        for ch in text.chars() {
            let up = ch.to_ascii_uppercase();
            if CHARS.contains(&up) {
                self.q.push_back(Some(up));
                self.total += 1;
            } else if ch == ' ' {
                self.q.push_back(None);
            }
        }
    }

    pub fn sent_chars(&self) -> usize {
        self.sent
    }

    pub fn total_chars(&self) -> usize {
        self.total
    }

    pub fn drained(&self) -> bool {
        self.q.is_empty() && self.out.is_empty()
    }

    pub fn clear(&mut self) {
        self.q.clear();
        self.out.clear();
        self.sent = 0;
        self.total = 0;
    }

    fn render(&mut self, key: Option<char>) {
        let pair = key.and_then(|ch| {
            let i = CHARS.iter().position(|&c| c == ch)?;
            Some((ROWS[i / 4], COLS[i % 4]))
        });
        let n = (self.rate * PRESS_S).round().max(1.0) as usize;
        for k in 0..n {
            let t = k as f64 / self.rate;
            let v = match pair {
                Some((row, col)) => {
                    (std::f64::consts::TAU * row * t).sin()
                        + (std::f64::consts::TAU * col * t).sin()
                }
                None => 0.0,
            };
            self.out.push_back((0.4 * v) as f32);
        }
        let gap = (self.rate * GAP_S).round().max(1.0) as usize;
        for _ in 0..gap {
            self.out.push_back(0.0);
        }
    }

    pub fn next_block(&mut self, out: &mut [f32]) -> usize {
        while self.out.len() < out.len() {
            let Some(key) = self.q.pop_front() else { break };
            self.render(key);
            self.sent += key.is_some() as usize;
        }
        for s in out.iter_mut() {
            *s = self.out.pop_front().unwrap_or(0.0);
        }
        out.len()
    }
}

fn best(v: &[f64; 4]) -> (usize, f64) {
    let mut i = 0;
    let mut m = f64::NEG_INFINITY;
    for (k, &p) in v.iter().enumerate() {
        if p > m {
            m = p;
            i = k;
        }
    }
    (i, m)
}

fn second(v: &[f64; 4], skip: usize) -> f64 {
    let mut m = 0.0f64;
    for (k, &p) in v.iter().enumerate() {
        if k != skip && p > m {
            m = p;
        }
    }
    m
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every key, as its pair of tones, read back exactly once.
    ///
    /// The transmitter is the standard's own definition — two equal-amplitude
    /// tones held for 100 ms with 50 ms of silence between presses — so this
    /// proves the decoder against the touch-tone specification rather than
    /// against one of our modulators.
    #[test]
    fn every_key_press_reads_back_once() {
        let rate = 8000.0;
        let mut rx = DtmfRx::new(rate);
        let mut audio = Vec::new();
        // A little silence first, so the receiver is running before the first key.
        audio.extend(std::iter::repeat(0.0f32).take(4000));
        for (i, &ch) in CHARS.iter().enumerate() {
            let row = ROWS[i / 4];
            let col = COLS[i % 4];
            let n = (rate * 0.100) as usize;
            for k in 0..n {
                let t = k as f64 / rate;
                let v = (std::f64::consts::TAU * row * t).sin()
                    + (std::f64::consts::TAU * col * t).sin();
                audio.push((0.4 * v) as f32);
            }
            audio.extend(std::iter::repeat(0.0f32).take((rate * 0.050) as usize));
            let _ = ch;
        }
        let mut got = String::new();
        for chunk in audio.chunks(256) {
            got.push_str(&rx.process(chunk));
        }
        let want: String = CHARS.iter().collect();
        assert_eq!(got, want, "keys decoded as {got:?}, wanted {want:?}");
    }

    /// **Every key, through our own transmitter and back.** The test above proves
    /// the decoder against the specification; this one proves the transmitter
    /// against it too, and pins the two together so a timing change on either
    /// side is a failing test rather than a silent mismatch on the air.
    #[test]
    fn the_transmitter_reads_back_every_key() {
        let rate = 8000.0;
        let keys: String = CHARS.iter().collect();
        let mut tx = DtmfTx::new(rate);
        tx.push_text(&format!(" {keys} "));
        let mut audio = Vec::new();
        let mut block = vec![0.0f32; 256];
        while !tx.drained() {
            tx.next_block(&mut block);
            audio.extend_from_slice(&block);
        }
        let mut rx = DtmfRx::new(rate);
        let mut got = String::new();
        for chunk in audio.chunks(256) {
            got.push_str(&rx.process(chunk));
        }
        assert_eq!(got, keys, "the transmitter sent {got:?}, wanted {keys:?}");
        assert_eq!(tx.sent_chars(), tx.total_chars(), "it did not count what it sent");
    }

    /// Two presses of the same key are two digits, which is the one thing the
    /// inter-press gap exists for: a held key is one press at the receiver, so
    /// `11` without a gap would be heard as `1`.
    #[test]
    fn two_presses_of_one_key_are_two_digits() {
        let rate = 8000.0;
        let mut tx = DtmfTx::new(rate);
        tx.push_text("11");
        let mut audio = Vec::new();
        let mut block = vec![0.0f32; 128];
        while !tx.drained() {
            tx.next_block(&mut block);
            audio.extend_from_slice(&block);
        }
        let mut rx = DtmfRx::new(rate);
        let mut got = String::new();
        for chunk in audio.chunks(256) {
            got.push_str(&rx.process(chunk));
        }
        assert_eq!(got, "11", "a doubled press decoded as {got:?}");
    }

    /// A character with no tone is not a tone: letters past `A`–`D` are dropped
    /// rather than sent as something else, and a space is silent.
    #[test]
    fn characters_without_a_tone_are_dropped() {
        let mut tx = DtmfTx::new(8000.0);
        tx.push_text("Q Z 7");
        assert_eq!(tx.total_chars(), 1, "it queued something with no tone");
        let mut audio = Vec::new();
        let mut block = vec![0.0f32; 64];
        while !tx.drained() {
            tx.next_block(&mut block);
            audio.extend_from_slice(&block);
        }
        let mut rx = DtmfRx::new(8000.0);
        let mut got = String::new();
        for chunk in audio.chunks(256) {
            got.push_str(&rx.process(chunk));
        }
        assert_eq!(got, "7");
    }

    /// Noise is not a key press. The eight Goertzel outputs always have a
    /// largest, which is why the ratio test exists.
    #[test]
    fn noise_does_not_produce_digits() {
        let rate = 8000.0;
        let mut rx = DtmfRx::new(rate);
        let mut seed = 0x9e37_79b9u32;
        let mut audio = Vec::new();
        for _ in 0..(rate as usize * 3) {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            audio.push(((seed >> 8) as f32 / 8_388_608.0) - 1.0);
        }
        let mut got = String::new();
        for chunk in audio.chunks(256) {
            got.push_str(&rx.process(chunk));
        }
        assert!(got.is_empty(), "noise decoded as {got:?}");
    }

    /// A held key is one digit, not one per frame.
    #[test]
    fn a_held_key_is_one_digit() {
        let rate = 8000.0;
        let mut rx = DtmfRx::new(rate);
        let n = (rate * 0.500) as usize;
        let mut audio = Vec::new();
        audio.extend(std::iter::repeat(0.0f32).take(4000));
        for k in 0..n {
            let t = k as f64 / rate;
            let v = (std::f64::consts::TAU * ROWS[1] * t).sin()
                + (std::f64::consts::TAU * COLS[1] * t).sin();
            audio.push((0.4 * v) as f32);
        }
        let mut got = String::new();
        for chunk in audio.chunks(256) {
            got.push_str(&rx.process(chunk));
        }
        assert_eq!(got, "5", "a half-second press decoded as {got:?}");
    }
}
