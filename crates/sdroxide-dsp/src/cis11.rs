//! CIS-11 (TORG-11) — the Russian meteorological teleprinter.
//!
//! 2-FSK at **100 baud**, the two tones about **500 Hz apart** (a 400 Hz variant
//! exists), carrying **ITA-2** — or the Russian M2 adaptation of it — in a
//! **11-bit character**, sent synchronously: no start or stop bits, the clock
//! runs continuously.
//!
//! The 11 bits are read as **5 data + the same 5 repeated + 1 parity**, which is
//! what makes the mode's "error correction: parity" true in the catalogue: a
//! character arrives twice and the parity bit covers the pair. That was measured,
//! not assumed — see `docs/internal/MODE-ROADMAP.md`, "CIS-11, scoped": the idle
//! transmission is one fixed 11-bit word repeated, and one rotation of it is
//! `00110 00110 1`.
//!
//! What is proven here and what is not: the receiver finds its own tones, tracks
//! the character clock and decodes the idle recording to a constant character
//! with valid parity. **The traffic is encrypted** on every recording we have, so
//! what a station's *text* looks like is not proven — the bits are exposed for
//! exactly that reason.
//!
//! Everything is receive-only: this is a broadcast service, and its content is
//! somebody else's traffic.

use std::f32::consts::TAU;

/// Baud rate. Always 100: the 50 and 200 baud teleprinters are different modes.
pub const BAUD: f64 = 100.0;
/// Bits in one character: **ten data and one odd-parity bit**.
///
/// Measured from the idle recording, not assumed: every candidate grouping of 4
/// to 16 bits was scored by how consistently its last bit makes the group's
/// parity odd, and 11 wins at 85 % on a crude demodulator — against 13 % for the
/// "5 data + 5 repeated" reading, which the traffic then rules out anyway (its
/// halves agree only 5 % of the time). See `docs/internal/MODE-ROADMAP.md`.
pub const CHAR_BITS: usize = 11;
/// The usual shift. 400 Hz is the other variant the recordings show.
pub const SHIFT_HZ: f32 = 500.0;

/// One decoded character.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cis11Char {
    /// The **ten** data bits, oldest first, exactly as they arrived. How they
    /// divide into characters is not settled (two five-bit ITA-2, or something
    /// else) — the bits are the honest output until a clear transmission says.
    pub bits: [u8; 10],
    /// The parity bit as received (bit 10 of the character).
    pub parity: u8,
    /// The two halves of the data agree — reported because the catalogue implies
    /// a repeated character, and it is worth seeing that they do not.
    pub repeat_ok: bool,
    /// The parity bit matches the pair (odd parity over all eleven bits).
    pub parity_ok: bool,
    /// Character-level confidence 0..1: the mean eye opening over its bits.
    pub confidence: f32,
    /// The audio instant of the character's first bit, seconds into the stream.
    pub at: f64,
}

impl Cis11Char {
    /// The first five bits as an ITA-2 code, LSB first — the reading a
    /// teleprinter would use if the ten bits are two characters.
    pub fn code(&self) -> u8 {
        self.bits[..5].iter().enumerate().fold(0u8, |v, (i, b)| v | (b << i))
    }

    /// The ITA-2 character the first five bits name, or `None` for a control.
    pub fn letter(&self) -> Option<char> {
        ita2(self.code())
    }

    /// Whether the parity bit agrees. That is what "clean" can mean until the
    /// character layer is known: the parity is measured, the grouping is not.
    pub fn clean(&self) -> bool {
        self.parity_ok
    }
}

/// The ITA-2 letter set (CCITT-2), letters shift. `None` for the codes that are
/// controls in this shift — FIGS is reported as `None` because what it selects
/// is the next character's table, which the caller tracks if it wants figures.
pub fn ita2(code: u8) -> Option<char> {
    const TABLE: [Option<char>; 32] = [
        None,       // 0 NUL
        Some('E'),  // 1
        Some('\n'), // 2 LF
        Some('A'),  // 3
        Some(' '),  // 4
        Some('S'),  // 5
        Some('I'),  // 6
        Some('U'),  // 7
        Some('\r'), // 8 CR
        Some('D'),  // 9
        Some('R'),  // 10
        Some('J'),  // 11
        Some('N'),  // 12
        Some('F'),  // 13
        Some('C'),  // 14
        Some('K'),  // 15
        Some('T'),  // 16
        Some('Z'),  // 17
        Some('L'),  // 18
        Some('W'),  // 19
        Some('H'),  // 20
        Some('Y'),  // 21
        Some('P'),  // 22
        Some('Q'),  // 23
        Some('O'),  // 24
        Some('B'),  // 25
        Some('G'),  // 26
        None,       // 27 FIGS
        Some('M'),  // 28
        Some('X'),  // 29
        Some('V'),  // 30
        None,       // 31 LTRS
    ];
    TABLE[code as usize & 31]
}

/// What the receiver has found so far, for the panel.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Cis11Report {
    /// Tones the acquisition picked, in Hz of audio.
    pub tone_low_hz: f32,
    pub tone_high_hz: f32,
    /// Shift between them, and the midpoint the operator should tune to.
    pub shift_hz: f32,
    pub centre_hz: f32,
    /// The signal is inverted (the low tone carries a one).
    pub inverted: bool,
    pub characters: u64,
    /// Characters whose repeat and parity both agreed.
    pub clean: u64,
    /// Mean eye opening of the last character, 0..1.
    pub confidence: f32,
    /// True once the tones have been found and characters are arriving.
    pub locked: bool,
}

/// Samples per character at this rate.
fn char_samples(rate: f64) -> f64 {
    rate / BAUD * CHAR_BITS as f64
}

/// A two-tone FSK receiver that finds its own tones, tracks the character clock
/// and decodes 11-bit characters.
#[derive(Clone)]
pub struct Cis11Rx {
    rate: f64,
    /// Tone separations tried at acquisition, in Hz, best first.
    tones: Option<(f32, f32)>,
    /// Audio held until there is enough of it to find the tones in. Fed through
    /// the demodulator once the acquisition has run, so nothing is lost.
    acq: Vec<f32>,
    /// One-pole complex resonators at the two tones. The **complex** product has
    /// to be integrated before the magnitude is taken: `|s·cos|` is just `|s|`,
    /// which is the trap this sat in first — every tone measured the same.
    lo_i: f32,
    lo_q: f32,
    hi_i: f32,
    hi_q: f32,
    /// The two magnitudes, once settled.
    sm_low: f32,
    sm_high: f32,
    /// Discriminator history, one sample per input sample. `disc` holds the
    /// samples from `disc_base` onwards, so everything that refers to a sample
    /// number can do so in **absolute** stream samples and survive the trim.
    disc: Vec<f32>,
    disc_base: u64,
    /// Where the next character starts, in absolute stream samples.
    next: Option<f64>,
    /// Which rotation of the 11-bit window the character starts at — found by
    /// decoding, not assumed.
    framing_bits: usize,
    /// Samples consumed, for timestamps and for dropping history.
    seen: u64,
    /// The character clock was aligned: the last alignment's eye opening.
    last_eye: f32,
    report: Cis11Report,
    text: String,
    /// Figures shift is on.
    figures: bool,
    /// Rolling decoder for both polarities, until the parity says which is which.
    inverted: bool,
    polarity_votes: i32,
}

impl Cis11Rx {
    pub fn new(rate_hz: f64) -> Self {
        Cis11Rx {
            rate: rate_hz,
            tones: None,
            acq: Vec::new(),
            lo_i: 0.0,
            lo_q: 0.0,
            hi_i: 0.0,
            hi_q: 0.0,
            sm_low: 0.0,
            sm_high: 0.0,
            disc: Vec::new(),
            disc_base: 0,
            next: None,
            framing_bits: 0,
            seen: 0,
            last_eye: 0.0,
            report: Cis11Report::default(),
            text: String::new(),
            figures: false,
            inverted: false,
            polarity_votes: 0,
        }
    }

    pub fn report(&self) -> &Cis11Report {
        &self.report
    }

    pub fn take_text(&mut self) -> String {
        std::mem::take(&mut self.text)
    }

    /// Forget the acquisition — a retune, a band change.
    pub fn reset(&mut self) {
        self.tones = None;
        self.acq.clear();
        self.disc.clear();
        self.disc_base = 0;
        self.next = None;
        self.seen = 0;
        self.report = Cis11Report::default();
        self.polarity_votes = 0;
    }

    /// Push audio. Returns the characters decoded from it.
    pub fn process(&mut self, audio: &[f32]) -> Vec<Cis11Char> {
        let mut out = Vec::new();
        if self.tones.is_none() {
            // The tones are found on a block, not on a stream of small chunks, so
            // hold audio until there is enough of it — and then run that same
            // audio through the demodulator, because a receiver that drops the
            // first half second of every transmission drops its first words.
            self.acq.extend_from_slice(audio);
            // A second: enough for nine characters, which is what the tone
            // refinement needs to tell a good pair from a bad one.
            let need = (self.rate * 1.0) as usize;
            if self.acq.len() < need {
                return out;
            }
            let Some((a, b)) = acquire(&self.acq, self.rate) else {
                // Slide the window rather than grow forever.
                let drop = self.acq.len().saturating_sub(need / 2);
                self.acq.drain(..drop);
                return out;
            };
            // The spectrum's tallest bins are not the tones: an FSK burst's
            // spectrum has lobes at the tone ± the baud rate, so the peak of a
            // lobe can sit 20 Hz off the tone it belongs to — measured on the
            // idle recording, where the acquisition read 795/1270 for a signal
            // whose tones are 773/1276. Refine by what actually matters:
            // characters with an intact repeat and a valid parity bit.
            let held = std::mem::take(&mut self.acq);
            let (a, b) = self.refine_tones(&held, a, b);
            self.tones = Some((a, b));
            self.report.tone_low_hz = a;
            self.report.tone_high_hz = b;
            self.report.shift_hz = b - a;
            self.report.centre_hz = (a + b) / 2.0;
            self.demodulate(&held);
            self.decode_ready(&mut out);
            return out;
        }
        self.demodulate(audio);
        self.decode_ready(&mut out);
        // Keep a couple of characters of history for the alignment search, and
        // remember what was dropped so absolute sample numbers still index it.
        let keep = (self.rate * 0.5) as usize;
        if self.disc.len() > keep * 3 {
            let drop = self.disc.len() - keep * 2;
            self.disc.drain(..drop);
            self.disc_base += drop as u64;
        }
        out
    }

    /// How many characters a pair of tones decodes cleanly from this audio.
    ///
    /// A trial runs on a copy of the receiver's configuration rather than on the
    /// receiver, so nothing that half-decoded survives the search.
    fn trial(&self, audio: &[f32], f_low: f32, f_high: f32, rot_bits: usize) -> (usize, usize) {
        let mut probe = Cis11Rx::new(self.rate);
        probe.framing_bits = rot_bits;
        probe.tones = Some((f_low, f_high));
        // **The character boundary is not the bit boundary.** The stream is a
        // periodic 11-bit word, but where an 11-bit window *starts* is a rotation
        // of it, and only one rotation is the 5 + 5 + 1 the character is. Which
        // one is a property of the transmission, not of the receiver, so it is
        // searched — 15 clean characters out of 123 was what this cost before it
        // was: every other rotation fails both the repeat and the parity.
        probe.next = Some(rot_bits as f64 * probe.rate / BAUD);
        probe.demodulate(audio);
        let mut chars = Vec::new();
        probe.decode_ready(&mut chars);
        let clean = chars.iter().filter(|c| c.clean()).count();
        (clean, chars.len())
    }

    /// Choose the tone pair by decoding, not by the spectrum's peak.
    fn refine_tones(&mut self, audio: &[f32], a: f32, b: f32) -> (f32, f32) {
        let centre = (a + b) / 2.0;
        let mut best = ((a, b), 0.0f32);
        let shifts = [500.0f32, 400.0, b - a];
        for shift in shifts {
            let half = shift / 2.0;
            for dc in -10i32..=10 {
                let c = centre + dc as f32 * 4.0;
                if c - half < 200.0 || c + half > self.rate as f32 * 0.45 {
                    continue;
                }
                let (clean, total) = self.trial(audio, c - half, c + half, 0);
                if total < 4 {
                    continue;
                }
                // A fraction smoothed by the evidence: three clean characters out
                // of three is not better than twenty out of twenty-four.
                let score = clean as f32 / (total as f32 + 2.0);
                if score > best.1 {
                    best = ((c - half, c + half), score);
                }
            }
        }
        let tones = if best.1 > 0.0 { best.0 } else { (a, b) };
        // Then the rotation of the character window, with the tones settled.
        let mut rot = 0usize;
        let mut rot_score = best.1;
        for r in 1..CHAR_BITS {
            let (clean, total) = self.trial(audio, tones.0, tones.1, r);
            if total < 4 {
                continue;
            }
            let score = clean as f32 / (total as f32 + 2.0);
            if score > rot_score {
                rot_score = score;
                rot = r;
            }
        }
        self.framing_bits = rot;
        tones
    }

    /// One complex multiply per tone, then a one-pole magnitude tracker. At 100
    /// baud a symbol is 10 ms, so a 250 Hz pole settles well inside it.
    fn demodulate(&mut self, audio: &[f32]) {
        let (f_low, f_high) = self.tones.expect("acquire runs before this");
        let a = 1.0 - (-TAU * 250.0 / self.rate as f32).exp();
        for &s in audio {
            let t = self.seen as f32 / self.rate as f32;
            let (sl, cl) = (TAU * f_low * t).sin_cos();
            let (sh, ch) = (TAU * f_high * t).sin_cos();
            self.lo_i += a * (s * cl - self.lo_i);
            self.lo_q += a * (s * sl - self.lo_q);
            self.hi_i += a * (s * ch - self.hi_i);
            self.hi_q += a * (s * sh - self.hi_q);
            self.sm_low = self.lo_i.hypot(self.lo_q);
            self.sm_high = self.hi_i.hypot(self.hi_q);
            self.disc.push(self.sm_high - self.sm_low);
            self.seen += 1;
        }
    }

    /// Decode every whole character that the history now holds.
    fn decode_ready(&mut self, out: &mut Vec<Cis11Char>) {
        let sps = self.rate / BAUD; // samples per bit
        loop {
            let Some(next) = self.next else {
                // Nothing aligned yet: try to find the character clock once there
                // is a character's worth of history.
                if (self.disc.len() as f64) < char_samples(self.rate) + sps {
                    return;
                }
                // The first alignment is taken over three characters, so a
                // single awkward bit cannot choose the clock.
                let guess = self.framing_bits as f64 * self.rate / BAUD;
                match self.align(guess, 0.6, CHAR_BITS * 3) {
                    Some((start, eye)) if eye > 0.15 => {
                        self.next = Some(start);
                        self.last_eye = eye;
                    }
                    _ => return,
                }
                continue;
            };
            let start = next;
            let end = start + char_samples(self.rate);
            if end + 1.0 > (self.disc_base + self.disc.len() as u64) as f64 {
                return;
            }
            // Re-align inside the character by the eye opening, which is what
            // tracks a clock that is not exactly 100 baud. The span is small: the
            // prediction is where the clock says it is, and a wide search per
            // character lets neighbouring characters disagree about the phase.
            let (start, eye) = self.align(start, 0.18, CHAR_BITS).unwrap_or((start, self.last_eye));
            self.last_eye = eye;
            let ch = self.read_char(start, eye);
            self.report.confidence = ch.confidence;
            self.report.characters += 1;
            if ch.clean() {
                self.report.clean += 1;
                // "Locked" is a character with valid parity and an intact
                // repeat, not merely a pair of tones in the passband.
                self.report.locked = true;
                self.polarity_votes += if self.inverted { -1 } else { 1 };
                if self.polarity_votes < -6 && !self.inverted {
                    self.inverted = true;
                    self.polarity_votes = 0;
                } else if self.polarity_votes > 6 && self.inverted {
                    self.inverted = false;
                    self.polarity_votes = 0;
                }
            }
            self.report.inverted = self.inverted;
            if ch.clean() {
                if let Some(c) = self.glyph(&ch) {
                    self.text.push(c);
                }
            }
            out.push(ch);
            self.next = Some(start + char_samples(self.rate));
        }
    }

    /// The discriminator at an absolute sample number, if it is still held.
    fn at(&self, n: f64) -> Option<f32> {
        let i = n - self.disc_base as f64;
        if i < 0.0 {
            return None;
        }
        self.disc.get(i as usize).copied()
    }

    /// The best bit-sample phase for a character starting near `guess`, and the
    /// eye opening it gives.
    fn align(&self, guess: f64, range_bits: f64, span_bits: usize) -> Option<(f64, f32)> {
        let sps = self.rate / BAUD;
        let step = sps / 20.0;
        let steps = (range_bits * 20.0).round() as i32;
        let mut best = (guess, 0.0f32);
        for k in -steps..=steps {
            let start = guess + k as f64 * step;
            if start < 0.0 {
                continue;
            }
            let eye = self.eye(start, span_bits);
            if eye > best.1 {
                best = (start, eye);
            }
        }
        Some(best)
    }

    /// The **lower quartile** of |discriminator| at the bit centres over `bits`
    /// bits, normalised by the strongest sample held.
    ///
    /// A mean would do on a clean signal and is what this used first; on a real
    /// one it lets a phase that lands half a bit out still score well, because
    /// the samples either side of a transition are as strong as any. The lower
    /// quartile asks "are *all* of these samples off the transition", which is
    /// the question an eye diagram answers.
    fn eye(&self, start: f64, bits: usize) -> f32 {
        let sps = self.rate / BAUD;
        let mut vals: Vec<f32> = Vec::with_capacity(bits);
        for b in 0..bits {
            match self.at(start + (b as f64 + 0.5) * sps) {
                Some(v) => vals.push(v.abs()),
                None => return 0.0,
            }
        }
        vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let q = vals[vals.len() / 4];
        let peak = self.disc.iter().fold(0.0f32, |m, v| m.max(v.abs())).max(1e-9);
        (q / peak).min(1.0)
    }

    fn read_char(&self, start: f64, eye: f32) -> Cis11Char {
        let sps = self.rate / BAUD;
        let mut bits = [0u8; CHAR_BITS];
        for (b, slot) in bits.iter_mut().enumerate() {
            let v = self.at(start + (b as f64 + 0.5) * sps).unwrap_or(0.0);
            // Inverted: the low tone carries a one.
            let one = if self.inverted { v < 0.0 } else { v > 0.0 };
            *slot = u8::from(one);
        }
        let mut data = [0u8; 10];
        data.copy_from_slice(&bits[0..10]);
        let parity = bits[10];
        let ones: u8 = bits.iter().sum();
        Cis11Char {
            bits: data,
            parity,
            repeat_ok: data[..5] == data[5..],
            // Odd parity over the whole character: the idle word 00110 00110 1
            // sums to five ones.
            parity_ok: ones % 2 == 1,
            confidence: eye,
            at: start / self.rate,
        }
    }

    /// The glyph for a character's first five bits, tracking FIGS/LTRS.
    fn glyph(&mut self, ch: &Cis11Char) -> Option<char> {
        let code = ch.code();
        match code {
            27 => {
                self.figures = true;
                None
            }
            31 => {
                self.figures = false;
                None
            }
            _ if self.figures => {
                const FIGS: [char; 32] = [
                    '\0', '3', '\n', '-', ' ', '\'', '8', '7', '\r', '$', '4', '\0', ',', '!', ':',
                    '(', '5', '"', ')', '2', '#', '6', '0', '1', '9', '?', '&', '\0', '.', '/',
                    ';', '\0',
                ];
                let c = FIGS[code as usize & 31];
                (c != '\0').then_some(c)
            }
            _ => ita2(code),
        }
    }
}

/// Find the two FSK tones in a burst of audio.
///
/// A Goertzel sweep over the passband, then the best pair at least 250 Hz apart
/// with the largest combined power. The pair has to stand well above the rest,
/// or the answer is "no signal" rather than a guess.
fn acquire(audio: &[f32], rate: f64) -> Option<(f32, f32)> {
    let n = audio.len().min((rate * 0.5) as usize);
    let a = &audio[..n];
    let mut power = Vec::new();
    let mut f = 300.0f32;
    while (f as f64) < rate * 0.45 {
        let mut re = 0.0f32;
        let mut im = 0.0f32;
        let w = TAU * f / rate as f32;
        for (i, &s) in a.iter().enumerate() {
            let t = i as f32;
            // A short window keeps the sweep cheap without smearing the tones.
            let win = 0.5 - 0.5 * (TAU * i as f32 / n as f32).cos();
            re += s * win * (w * t).cos();
            im += s * win * (w * t).sin();
        }
        power.push((f, re * re + im * im));
        f += 5.0;
    }
    let max = power.iter().fold(0.0f32, |m, (_, p)| m.max(*p));
    let mean = power.iter().map(|(_, p)| *p).sum::<f32>() / power.len().max(1) as f32;
    // Two tones stand far above the rest of the sweep, but they are also part of
    // its mean — a threshold on the mean alone rejects a clean FSK burst, which
    // is the trap this line fell into first. Median against maximum is the honest
    // test, and 12:1 is well clear of a noise floor.
    let mut sorted: Vec<f32> = power.iter().map(|(_, p)| *p).collect();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let median = sorted[sorted.len() / 2].max(1e-12);
    if max < median * 12.0 || max < mean * 3.0 {
        return None;
    }
    // The two tones of the mode: 500 Hz apart (400 on the variant the
    // recordings show). Both have to be individually well clear of the noise, so
    // a strong carrier next to noise cannot pass for a pair.
    let mut best: Option<(f32, f32, f32)> = None;
    for (i, &(fa, pa)) in power.iter().enumerate() {
        // Both tones of an FSK pair arrive at about the same level, so a second
        // "tone" that is 15 dB weaker is the window's leakage off the first —
        // which is exactly how a single carrier passed for a pair here.
        // 8 % of the strongest bin: the window's own sidelobes are about -13 dB
        // (5 %), so this keeps them out while allowing a real pair whose tones
        // are not equally busy in the audio analysed.
        if pa < median * 10.0 || pa < max * 0.08 {
            continue;
        }
        for &(fb, pb) in power.iter().skip(i + 1) {
            if pb < median * 10.0 || pb < max * 0.08 {
                continue;
            }
            if !(330.0..=600.0).contains(&(fb - fa)) {
                continue;
            }
            let score = pa + pb;
            if best.is_none_or(|(_, _, s)| score > s) {
                best = Some((fa, fb, score));
            }
        }
    }
    best.map(|(a, b, _)| (a, b))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The ITA-2 table, spot-checked where a wiring mistake would show: the five
    /// bits a common idle uses, and the shift codes.
    #[test]
    fn the_alphabet_is_ita2() {
        assert_eq!(ita2(0b00110), Some('I')); // 6
        assert_eq!(ita2(0b01010), Some('R')); // 10
        assert_eq!(ita2(0b10101), Some('Y')); // 21
        assert_eq!(ita2(0b00011), Some('A')); // 3
        assert_eq!(ita2(27), None); // FIGS
        assert_eq!(ita2(31), None); // LTRS
    }

    /// An 11-bit character: ten data bits and the odd-parity bit. The generator
    /// puts one ITA-2 character in the first five and zeros in the second, which
    /// is enough to exercise the path — the real division is unproven.
    fn character(bits: [u8; 5]) -> [u8; CHAR_BITS] {
        let mut out = [0u8; CHAR_BITS];
        out[..5].copy_from_slice(&bits);
        // A balanced second half: leaving it zero made the low tone six times
        // the high one, which is not what a transmission looks like and which the
        // acquisition's level test rightly complains about.
        for (i, slot) in out[5..10].iter_mut().enumerate() {
            *slot = ((bits[i % 5] as usize + i) % 2) as u8;
        }
        let ones: u8 = out.iter().sum::<u8>();
        out[10] = if ones % 2 == 0 { 1 } else { 0 };
        out
    }

    /// A CIS-11 burst at 8 kHz: 2-FSK, 100 baud, given shift and centre.
    fn burst(chars: &[[u8; 5]], shift: f32, centre: f32, rate: f64) -> Vec<f32> {
        let sps = (rate / BAUD) as usize;
        let f_low = centre - shift / 2.0;
        let f_high = centre + shift / 2.0;
        let mut out = Vec::new();
        let mut phase = 0.0f64;
        for c in chars {
            let bits = character(*c);
            for b in bits {
                let f = if b == 1 { f_high } else { f_low };
                for _ in 0..sps {
                    phase += TAU as f64 * f as f64 / rate;
                    out.push((0.5 * phase.sin()) as f32);
                }
            }
        }
        out
    }

    /// The whole path: a synthesized character stream comes back with its text.
    #[test]
    fn a_synthesized_message_decodes() {
        let text = "RYRY TEST 123";
        let mut chars: Vec<[u8; 5]> = Vec::new();
        let mut figures = false;
        for ch in text.chars() {
            match ch {
                ' ' => chars.push([0, 0, 1, 0, 0]),
                '0'..='9' => {
                    if !figures {
                        chars.push([1, 1, 0, 1, 1]); // LTRS is 31; FIGS is 27
                        figures = true;
                    }
                    let idx = match ch {
                        '1' => 27,
                        '2' => 19,
                        '3' => 1,
                        _ => 27,
                    };
                    let mut b = [0u8; 5];
                    for i in 0..5 {
                        b[i] = (idx >> i) & 1;
                    }
                    chars.push(b);
                }
                _ => {
                    if figures {
                        chars.push([1, 1, 1, 1, 1]);
                        figures = false;
                    }
                    let code = (0..32).find(|c| ita2(*c) == Some(ch)).expect("in the table");
                    let mut b = [0u8; 5];
                    for i in 0..5 {
                        b[i] = ((code >> i) & 1) as u8;
                    }
                    chars.push(b);
                }
            }
        }
        let audio = burst(&chars, SHIFT_HZ, 1000.0, 8000.0);
        let mut rx = Cis11Rx::new(8000.0);
        let mut got = Vec::new();
        for chunk in audio.chunks(256) {
            got.extend(rx.process(chunk));
        }
        let r = rx.report();
        assert!(
            got.len() >= chars.len().saturating_sub(1),
            "decoded {} of {}: tones {:.0}/{:.0}, locked {}, clean {}",
            got.len(),
            chars.len(),
            r.tone_low_hz,
            r.tone_high_hz,
            r.locked,
            r.clean
        );
        let clean = got.iter().filter(|c| c.clean()).count();
        assert!(clean * 10 >= got.len() * 9, "{clean} of {} characters clean", got.len());
        // The repeat must NOT look present: the catalogue's "5 + 5" reading is
        // what the measurement ruled out, and a receiver that pretended otherwise
        // would be hiding the finding.
        let repeats = got.iter().filter(|c| c.repeat_ok).count();
        assert!(repeats * 4 < got.len(), "{repeats} of {} claimed a repeat", got.len());
        assert_eq!(rx.report().shift_hz.round() as i32, SHIFT_HZ as i32);
        let text_back = rx.take_text();
        assert!(text_back.contains("RYRY"), "text: {text_back:?}");
    }

    /// Noise alone does not acquire, and neither does a single carrier.
    #[test]
    fn noise_and_a_carrier_do_not_lock() {
        let mut rx = Cis11Rx::new(8000.0);
        let mut seed = 0x1234_5678u32;
        let mut noise = Vec::new();
        for _ in 0..(8000 * 2) {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            noise.push((seed >> 8) as f32 / 8_388_608.0 - 1.0);
        }
        for chunk in noise.chunks(512) {
            let got = rx.process(chunk);
            assert!(got.is_empty(), "noise gave {} characters, tones {:?}", got.len(), rx.report());
        }
        assert!(!rx.report().locked);

        let mut rx = Cis11Rx::new(8000.0);
        let tone: Vec<f32> =
            (0..8000 * 2).map(|i| 0.5 * (TAU * 1000.0 * i as f32 / 8000.0).cos()).collect();
        for chunk in tone.chunks(512) {
            let got = rx.process(chunk);
            assert!(
                got.is_empty(),
                "a carrier gave {} characters, tones {:?}",
                got.len(),
                rx.report()
            );
        }
    }

    /// **The off-air gate.** Point `SDROXIDE_CIS11_SAMPLE` at a 16-bit mono WAV
    /// of a CIS-11 transmission (any rate) and the receiver must find the tones
    /// and read characters with valid parity. The idle recording is the oracle:
    /// it is one character repeated, so a correct framing gives a constant
    /// character and a wrong one does not.
    #[test]
    #[ignore]
    fn an_off_air_recording_decodes() {
        let Ok(path) = std::env::var("SDROXIDE_CIS11_SAMPLE") else {
            return;
        };
        let mut reader = hound::WavReader::open(&path).expect("open the wav");
        let spec = reader.spec();
        let audio: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
            hound::SampleFormat::Int => {
                reader.samples::<i16>().map(|s| s.unwrap() as f32 / 32_768.0).collect()
            }
        };
        let mut rx = Cis11Rx::new(spec.sample_rate as f64);
        let mut chars = Vec::new();
        for chunk in audio.chunks(1024) {
            chars.extend(rx.process(chunk));
        }
        let r = rx.report();
        eprintln!(
            "tones {:.0}/{:.0} Hz (shift {:.0}), {} characters, {} clean, confidence {:.2}",
            r.tone_low_hz, r.tone_high_hz, r.shift_hz, r.characters, r.clean, r.confidence
        );
        assert!(r.locked, "the tones were never found");
        assert!(chars.len() > 20, "only {} characters", chars.len());
        let clean = chars.iter().filter(|c| c.clean()).count();
        assert!(clean * 2 >= chars.len(), "{clean} clean of {}", chars.len());
    }
}
