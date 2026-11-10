//! CTCSS and DCS: the sub-audible squelch signalling on analogue FM.
//!
//! Both live under the voice on the same carrier and both come out of the FM
//! discriminator intact — the NFM path here applies no de-emphasis, and the
//! detector taps the discriminator ahead of the listener high-pass, so nothing
//! has been taken out of the 67-254 Hz range by the time it arrives. The stream
//! is decimated to a 1 kHz working rate and handed to two detectors.
//!
//! **CTCSS** is a continuous tone from the 50-entry standard table. A Goertzel
//! per tone over a 1.024 s window resolves them: the table's tightest pair is
//! 67.0 and 69.3 Hz, 2.3 Hz apart, which needs about a second of signal however
//! it is measured — so expect roughly that long before a tone appears after a
//! repeater keys up. Three things then have to agree before it is believed. The
//! winner must beat the runner-up and the median of the whole bank, which makes
//! the decision independent of level and rejects noise; and its frequency,
//! measured from the phase advance between successive windows rather than from
//! which bin won, must land within tolerance of the table entry. That last check
//! is what stops an arbitrary piece of low-frequency rumble being labelled as
//! whichever standard tone it happened to sit nearest — and the table is dense
//! enough that something always is near.
//!
//! **DCS** is a continuous stream of 23-bit Golay(23,12) codewords at 134.4 bps,
//! and what is reported is that it is *there*, not which of the 104 codes it
//! carries. See [`SubTone::Dcs`] for why. What is left is convention-free and
//! decisive: recover the bit clock, then look for a bitstream that repeats
//! exactly every 23 bits over three words running, whose word is a Golay
//! codeword of a plausible weight. Nothing else on an FM channel does that.

use std::sync::OnceLock;

use sdroxide_types::{CTCSS_TONES, SubTone};

use crate::Complex32;
use crate::decim::RealFirDecim;
use crate::demod::DcBlock;
use crate::window::hann;

/// Rate both detectors work at. Above twice the 254.1 Hz top tone and above the
/// DCS bit rate, and low enough that a second of history is only 1000 samples.
const WORK_RATE: f64 = 1000.0;
/// Long, because the anti-alias filter has to fall from 300 Hz to the 500 Hz
/// output Nyquist: a short one would fold voice energy around 750 Hz onto the
/// top of the CTCSS band. Only the kept outputs are computed, so this costs
/// about 1.4 million multiply-accumulates a second and no more.
const DECIM_TAPS: usize = 1401;
const DECIM_CUTOFF_HZ: f64 = 300.0;
/// Corner of the detector's own DC blocker, at the working rate. Slow enough to
/// leave both the 67 Hz bottom of the CTCSS table and the 67.2 Hz DCS
/// fundamental alone, and fast enough to take out the constant offset an
/// off-tune carrier puts on the discriminator. The listener path removes that
/// offset far more aggressively than DCS data could survive, which is why the
/// detector is fed from ahead of it and does its own.
const DC_HZ: f64 = 3.0;

/// 1.024 s of history: bin spacing 0.98 Hz, so the 2.3 Hz gap between 67.0 and
/// 69.3 is a little over two bins — resolvable under a Hann window, and not
/// under anything much shorter.
const WINDOW: usize = 1024;
/// Re-measure roughly eight times a second.
const HOP: usize = 128;

/// How far the winning tone must stand above the whole bank (linear ratio,
/// ~15.6 dB). Rejects noise, which is flat across the band.
const PEAK_OVER_MEDIAN: f32 = 6.0;
/// …and above the next-best tone (~8 dB).
const PEAK_OVER_RUNNER_UP: f32 = 2.5;
/// Amplitude floor in discriminator units, where 1.0 is 5 kHz deviation — so
/// this is about 75 Hz of deviation, well under the 500-1000 Hz a transmitter
/// actually uses for CTCSS.
const MIN_AMPLITUDE: f32 = 0.015;
/// How far the measured frequency may sit from the table entry: the standard's
/// own ±0.5 % transmit tolerance, with a floor that leaves room for measurement
/// noise on the low tones, where 0.5 % is only a third of a hertz.
const TOLERANCE_FRAC: f64 = 0.005;
const TOLERANCE_MIN_HZ: f64 = 1.0;
/// Consecutive agreeing measurements before a tone is believed, and consecutive
/// disagreeing ones before it is dropped. Asymmetric on purpose: slow to claim,
/// slower to let go, so the readout doesn't flicker through a mobile flutter.
/// At [`HOP`] per measurement that is a quarter-second to claim a tone and half
/// a second of holding it through a signal that comes and goes.
const CTCSS_LOCK_HITS: u32 = 2;
const CTCSS_DROP_MISSES: u32 = 4;

const DCS_BIT_RATE: f64 = 134.4;
/// Bit-clock loop gain. Low enough to ride out a run of same-value bits.
const DCS_CLOCK_GAIN: f64 = 0.05;
/// Bits of exact 23-periodicity before a DCS stream is believed — three words.
const DCS_PERIOD_BITS: u32 = 69;
/// Bits without a confirming word before it is dropped.
const DCS_HOLD_BITS: u32 = 69;
/// A Golay codeword has weight 0, 7, 8, 11, 12, 15, 16 or 23. The two extremes
/// are the constant streams — a dead carrier is perfectly 23-periodic and would
/// otherwise read as DCS — so a real word has to sit between them.
const DCS_MIN_WEIGHT: u32 = 7;
const DCS_MAX_WEIGHT: u32 = 16;

/// Golay(23,12) generator polynomial x^11 + x^10 + x^6 + x^5 + x^4 + x^2 + 1,
/// and its reciprocal. Either may be the one a transmitter used; accepting a
/// codeword under either costs a dozen shifts.
const GOLAY_GEN: u32 = 0xC75;
const GOLAY_GEN_ALT: u32 = 0xAE3;
const WORD_MASK: u32 = 0x7F_FFFF;

/// Remainder of a 23-bit word divided by a generator. Zero exactly for that
/// generator's 4096 codewords.
fn syndrome_with(word: u32, generator: u32) -> u32 {
    let mut r = word & WORD_MASK;
    for i in (11..23).rev() {
        if r & (1 << i) != 0 {
            r ^= generator << (i - 11);
        }
    }
    r & 0x7FF
}

fn golay_syndrome(word: u32) -> u32 {
    syndrome_with(word, GOLAY_GEN)
}

/// Syndrome → the error pattern that produced it.
///
/// The binary Golay code is *perfect*: every one of the 2^23 words lies within
/// three bit errors of exactly one codeword, and the 1 + 23 + 253 + 1771 = 2048
/// patterns of weight at most three map one-to-one onto the 2048 syndromes. So
/// this table is a bijection and decoding is a single lookup — and, less
/// conveniently, a zero syndrome proves nothing on its own, because every word
/// decodes to *something*.
fn error_table() -> &'static [u32; 2048] {
    static TABLE: OnceLock<[u32; 2048]> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut t = [0u32; 2048];
        let mut set = |e: u32| t[golay_syndrome(e) as usize] = e;
        set(0);
        for a in 0..23 {
            set(1 << a);
            for b in (a + 1)..23 {
                set((1 << a) | (1 << b));
                for c in (b + 1)..23 {
                    set((1 << a) | (1 << b) | (1 << c));
                }
            }
        }
        t
    })
}

/// Correct a received 23-bit word. Returns the corrected word and how many bits
/// had to be flipped (never more than three — see [`error_table`]).
pub fn golay23_decode(word: u32) -> (u32, u32) {
    let e = error_table()[golay_syndrome(word) as usize];
    ((word & WORD_MASK) ^ e, e.count_ones())
}

/// Encode 12 information bits into a 23-bit systematic codeword, information in
/// the high bits. The inverse of [`golay23_decode`] with no errors.
pub fn golay23_encode(info: u32) -> u32 {
    let shifted = (info & 0xFFF) << 11;
    shifted | golay_syndrome(shifted)
}

/// Whether a 23-bit window could be a DCS word: a codeword under one of the two
/// standard generators, and not one of the two constant streams.
fn plausible_dcs_word(word: u32) -> bool {
    let w = word.count_ones();
    (DCS_MIN_WEIGHT..=DCS_MAX_WEIGHT).contains(&w)
        && (golay_syndrome(word) == 0 || syndrome_with(word, GOLAY_GEN_ALT) == 0)
}

/// Watches the FM discriminator for a CTCSS tone or a DCS data stream.
pub struct SubToneDetect {
    decim: RealFirDecim,
    dc: DcBlock,
    work_rate: f64,
    decimated: Vec<f32>,

    // CTCSS
    hist: Vec<f32>,
    pos: usize,
    filled: usize,
    since_eval: usize,
    window: Vec<f32>,
    scratch: Vec<f32>,
    bins: Vec<Complex32>,
    /// The previous evaluation's bins, kept so the winner's frequency can be
    /// measured from how far its phase advanced rather than from which bin it
    /// happened to be.
    prev_bins: Vec<Complex32>,
    have_prev: bool,
    mags: Vec<f32>,
    sorted: Vec<f32>,
    candidate: Option<SubTone>,
    hits: u32,
    misses: u32,
    ctcss_locked: Option<SubTone>,

    // DCS
    dcs_phase: f64,
    dcs_inc: f64,
    dcs_acc: f32,
    dcs_prev: f32,
    reg: u32,
    bits: u32,
    period_run: u32,
    dcs_idle: u32,
    dcs_locked: bool,
}

impl SubToneDetect {
    /// `rate` is the discriminator's own sample rate (the channel rate).
    pub fn new(rate: f64) -> Self {
        let factor = (rate / WORK_RATE).round().max(1.0) as usize;
        let work_rate = rate / factor as f64;
        let n = CTCSS_TONES.len();
        SubToneDetect {
            decim: RealFirDecim::new(DECIM_TAPS, DECIM_CUTOFF_HZ, rate, factor),
            dc: DcBlock::new(DC_HZ, work_rate),
            work_rate,
            decimated: Vec::new(),
            hist: vec![0.0; WINDOW],
            pos: 0,
            filled: 0,
            since_eval: 0,
            window: hann(WINDOW),
            scratch: vec![0.0; WINDOW],
            bins: vec![Complex32::default(); n],
            prev_bins: vec![Complex32::default(); n],
            have_prev: false,
            mags: vec![0.0; n],
            sorted: vec![0.0; n],
            candidate: None,
            hits: 0,
            misses: 0,
            ctcss_locked: None,
            dcs_phase: 0.0,
            dcs_inc: DCS_BIT_RATE / work_rate,
            dcs_acc: 0.0,
            dcs_prev: 0.0,
            reg: 0,
            bits: 0,
            period_run: 0,
            dcs_idle: 0,
            dcs_locked: false,
        }
    }

    /// What is being received, or `None`.
    ///
    /// CTCSS wins where both somehow claim the signal: a tone names the system
    /// it came from, where "DCS" only says which kind of signalling it is, so it
    /// is both the more useful answer and the better-evidenced one.
    pub fn detected(&self) -> Option<SubTone> {
        self.ctcss_locked.or(self.dcs_locked.then_some(SubTone::Dcs))
    }

    /// Forget everything — for a mode or filter change, where the history
    /// belongs to a signal that is no longer there.
    pub fn reset(&mut self) {
        self.hist.fill(0.0);
        self.pos = 0;
        self.filled = 0;
        self.since_eval = 0;
        self.have_prev = false;
        self.candidate = None;
        self.hits = 0;
        self.misses = 0;
        self.ctcss_locked = None;
        self.dcs_acc = 0.0;
        self.dcs_prev = 0.0;
        self.reg = 0;
        self.bits = 0;
        self.period_run = 0;
        self.dcs_idle = 0;
        self.dcs_locked = false;
    }

    /// Feed discriminator output.
    ///
    /// The evaluation sits *inside* the sample loop, and has to. [`on_frequency`]
    /// reads the tone's frequency out of how far its phase turned between two
    /// windows, which is only a frequency if those windows really are [`HOP`]
    /// samples apart. Draining the block first and evaluating afterwards would
    /// instead measure every window against the end of the block — an advance
    /// set by whatever the front end happened to hand over, which for a
    /// SoapySDR read is a transfer size nobody here chose and which varies from
    /// one call to the next. The residual that check computes is an angle, so a
    /// wrong advance does not read as a wild frequency and get thrown out; it
    /// wraps, and lands somewhere arbitrary but plausible. The tone then passes
    /// or fails by coincidence, differently for each entry in the table.
    ///
    /// [`on_frequency`]: Self::on_frequency
    pub fn process(&mut self, audio: &[f32]) {
        self.decimated.clear();
        self.decim.process(audio, &mut self.decimated);
        for i in 0..self.decimated.len() {
            let x = self.dc.run(self.decimated[i]);
            self.hist[self.pos] = x;
            self.pos = (self.pos + 1) % WINDOW;
            self.filled = (self.filled + 1).min(WINDOW);
            self.since_eval += 1;
            self.push_dcs(x);
            if self.since_eval >= HOP && self.filled >= WINDOW {
                self.since_eval = 0;
                self.evaluate_ctcss();
            }
        }
    }

    fn evaluate_ctcss(&mut self) {
        for i in 0..WINDOW {
            self.scratch[i] = self.hist[(self.pos + i) % WINDOW] * self.window[i];
        }
        // A Hann-windowed sinusoid of amplitude A reads N*A/4 through a
        // Goertzel, so this puts the bank into discriminator units — where 1.0
        // is 5 kHz of deviation and the floor below means something.
        let norm = 4.0 / WINDOW as f32;
        for ((bin, mag), &tenths) in
            self.bins.iter_mut().zip(self.mags.iter_mut()).zip(CTCSS_TONES.iter())
        {
            *bin = goertzel(&self.scratch, tenths as f64 / 10.0, self.work_rate);
            *mag = bin.norm() * norm;
        }

        let mut best = 0usize;
        for (i, &m) in self.mags.iter().enumerate() {
            if m > self.mags[best] {
                best = i;
            }
        }
        let peak = self.mags[best];
        let runner_up = self
            .mags
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != best)
            .fold(0.0f32, |a, (_, &m)| a.max(m));
        self.sorted.copy_from_slice(&self.mags);
        self.sorted.sort_by(|a, b| a.partial_cmp(b).expect("magnitudes are finite"));
        let median = self.sorted[self.sorted.len() / 2].max(f32::MIN_POSITIVE);

        let seen = (peak >= MIN_AMPLITUDE
            && peak >= median * PEAK_OVER_MEDIAN
            && peak >= runner_up * PEAK_OVER_RUNNER_UP
            && self.on_frequency(best))
        .then(|| SubTone::Ctcss(CTCSS_TONES[best]));

        self.prev_bins.copy_from_slice(&self.bins);
        self.have_prev = true;
        self.update_lock(seen);
    }

    /// Fold one window's verdict into the lock.
    ///
    /// Two counters, and deliberately independent ones: `hits` is a run of
    /// windows agreeing on a tone that is not locked yet, `misses` a run of
    /// windows failing to confirm the tone that is. Both are runs — a window
    /// that confirms the lock spends the misses standing against it outright.
    /// Counting misses cumulatively instead would mean a signal that fluttered
    /// at all lost its tone eventually no matter how many windows in between
    /// found it, which is the opposite of what the hold is for.
    fn update_lock(&mut self, seen: Option<SubTone>) {
        if seen.is_some() && seen == self.candidate {
            self.hits = self.hits.saturating_add(1);
        } else {
            self.candidate = seen;
            self.hits = 1;
        }
        if self.candidate.is_some() && self.hits >= CTCSS_LOCK_HITS {
            self.ctcss_locked = self.candidate;
            self.misses = 0;
        }
        if self.ctcss_locked.is_some() {
            if seen == self.ctcss_locked {
                self.misses = 0;
            } else {
                self.misses += 1;
                if self.misses >= CTCSS_DROP_MISSES {
                    self.ctcss_locked = None;
                    self.misses = 0;
                }
            }
        }
    }

    /// Whether the signal in bin `i` is really at that bin's frequency.
    ///
    /// Successive analysis windows start `HOP` samples apart, so a component at
    /// frequency `f` turns its bin's phase by `2*pi*f*HOP/rate` between them.
    /// Comparing that against what the bin's own frequency would have turned
    /// gives the offset directly, to a fraction of a hertz — far finer than the
    /// 0.98 Hz bin spacing, which is the whole point. `HOP` also has to be short
    /// enough that the measurement cannot wrap: 128 samples at 1 kHz is
    /// unambiguous out to ±3.9 Hz, wider than any gap in the table.
    fn on_frequency(&self, i: usize) -> bool {
        if !self.have_prev {
            return false;
        }
        let hz = CTCSS_TONES[i] as f64 / 10.0;
        let per_hz = std::f64::consts::TAU * HOP as f64 / self.work_rate;
        let turned = (self.bins[i] * self.prev_bins[i].conj()).arg() as f64;
        let residual = turned - hz * per_hz;
        let wrapped = residual - std::f64::consts::TAU * (residual / std::f64::consts::TAU).round();
        (wrapped / per_hz).abs() <= (hz * TOLERANCE_FRAC).max(TOLERANCE_MIN_HZ)
    }

    fn push_dcs(&mut self, x: f32) {
        self.dcs_acc += x;
        self.dcs_phase += self.dcs_inc;
        // A data transition belongs on a bit boundary, so drag the clock towards
        // wherever they keep happening.
        if (x > 0.0) != (self.dcs_prev > 0.0) {
            let err = if self.dcs_phase < 0.5 { self.dcs_phase } else { self.dcs_phase - 1.0 };
            self.dcs_phase -= DCS_CLOCK_GAIN * err;
        }
        self.dcs_prev = x;
        if self.dcs_phase < 1.0 {
            return;
        }
        self.dcs_phase -= 1.0;
        // Integrate-and-dump: the matched filter for a rectangular bit.
        let bit = self.dcs_acc > 0.0;
        self.dcs_acc = 0.0;
        self.push_bit(bit);
    }

    fn push_bit(&mut self, bit: bool) {
        // The oldest bit in the register is the one from exactly a word ago.
        let a_word_ago = self.reg & 1 != 0;
        self.reg = (self.reg >> 1) | ((bit as u32) << 22);
        self.bits = self.bits.saturating_add(1);
        if self.bits <= 23 {
            return;
        }
        if bit == a_word_ago {
            self.period_run += 1;
        } else {
            self.period_run = 0;
        }
        self.dcs_idle += 1;
        // Exactly 23-bit periodic for three words, carrying something that could
        // be a codeword. A word repeating forever is what DCS *is*, and noticing
        // that needs no assumption about how the word is laid out.
        if self.period_run >= DCS_PERIOD_BITS && plausible_dcs_word(self.reg) {
            self.dcs_locked = true;
            self.dcs_idle = 0;
        }
        if self.dcs_idle > DCS_HOLD_BITS {
            self.dcs_locked = false;
        }
    }
}

/// One Goertzel bin: `samples` at `hz`, two multiplies per sample and no
/// trigonometry inside the loop. Complex, because the phase across windows is
/// what tells the caller the true frequency.
fn goertzel(samples: &[f32], hz: f64, rate: f64) -> Complex32 {
    let w = std::f64::consts::TAU * hz / rate;
    let (sin_w, cos_w) = w.sin_cos();
    let coeff = 2.0 * cos_w;
    let (mut s1, mut s2) = (0.0f64, 0.0f64);
    for &x in samples {
        let s = x as f64 + coeff * s1 - s2;
        s2 = s1;
        s1 = s;
    }
    Complex32::new((s1 - cos_w * s2) as f32, (sin_w * s2) as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The claim the Golay side rests on: the code is perfect, so the syndrome
    /// table is a bijection and every word decodes.
    #[test]
    fn the_golay_syndrome_table_is_a_bijection() {
        let t = error_table();
        let mut seen = std::collections::HashSet::new();
        for (s, &e) in t.iter().enumerate() {
            assert!(e.count_ones() <= 3, "syndrome {s} maps to weight {}", e.count_ones());
            assert_eq!(golay_syndrome(e), s as u32, "syndrome {s} round trip");
            assert!(seen.insert(e), "two syndromes map to the same pattern {e:#x}");
        }
        assert_eq!(seen.len(), 2048);
    }

    /// Every codeword survives every error pattern the code is supposed to fix.
    #[test]
    fn golay_corrects_up_to_three_errors_in_every_word() {
        for info in [0u32, 1, 0xFFF, 0xABC, 0x555, 0xAAA, 0x123, 0x800] {
            let code = golay23_encode(info);
            assert_eq!(golay_syndrome(code), 0, "encoded word must be a codeword");
            assert_eq!(golay23_decode(code), (code, 0));
            for a in 0..23 {
                for b in 0..23 {
                    for c in 0..23 {
                        let e = (1 << a) | (1 << b) | (1 << c);
                        let (fixed, n) = golay23_decode(code ^ e);
                        assert_eq!(fixed, code, "3 errors {e:#x} on info {info:#x}");
                        assert_eq!(n, e.count_ones());
                    }
                }
            }
        }
    }

    /// A dead carrier gives a perfectly 23-periodic stream of identical bits,
    /// and the all-zeros and all-ones words are both Golay codewords. Neither
    /// may be mistaken for data.
    #[test]
    fn a_constant_stream_is_not_a_dcs_word() {
        assert!(!plausible_dcs_word(0));
        assert!(!plausible_dcs_word(WORD_MASK));
        // Something with a real word's weight is allowed through.
        assert!(plausible_dcs_word(golay23_encode(0x213)));
    }

    const A: Option<SubTone> = Some(SubTone::Ctcss(885));
    const B: Option<SubTone> = Some(SubTone::Ctcss(1035));

    /// Drive the lock with a written-out run of window verdicts and report what
    /// it ends up holding.
    fn feed(verdicts: &[Option<SubTone>]) -> Option<SubTone> {
        let mut d = SubToneDetect::new(48_000.0);
        for &v in verdicts {
            d.update_lock(v);
        }
        d.detected()
    }

    /// Two agreeing windows and no fewer: one window that liked the look of some
    /// rumble must not put a tone on the readout.
    #[test]
    fn a_tone_needs_two_agreeing_windows() {
        assert_eq!(feed(&[A]), None);
        assert_eq!(feed(&[A, A]), A);
        assert_eq!(feed(&[A, B]), None);
    }

    /// The reported fault: a repeater sending a continuous tone, identified and
    /// then lost and then found again. Whatever makes a window miss — flutter,
    /// a burst of noise, a passing car — the tone is still being transmitted,
    /// and windows that do find it have to count for something.
    #[test]
    fn windows_that_find_the_tone_hold_it_against_ones_that_miss() {
        let mut d = SubToneDetect::new(48_000.0);
        d.update_lock(A);
        d.update_lock(A);
        assert_eq!(d.detected(), A, "locked after two agreeing windows");
        // Every other window misses, for far longer than the hold alone covers.
        for i in 0..64 {
            d.update_lock(if i % 2 == 0 { None } else { A });
            assert_eq!(d.detected(), A, "dropped the tone at window {i}");
        }
    }

    /// The hold is not unconditional, though. A tone that has actually stopped
    /// has to clear, or the squelch stays open on an empty channel.
    #[test]
    fn a_tone_that_stops_is_let_go_of() {
        let mut d = SubToneDetect::new(48_000.0);
        d.update_lock(A);
        d.update_lock(A);
        for _ in 0..CTCSS_DROP_MISSES - 1 {
            d.update_lock(None);
            assert_eq!(d.detected(), A, "let go too early");
        }
        d.update_lock(None);
        assert_eq!(d.detected(), None);
    }

    /// A second system keying up on the same channel replaces the first, and on
    /// its own evidence — the same two windows any tone needs.
    #[test]
    fn a_different_tone_takes_over_the_readout() {
        let mut d = SubToneDetect::new(48_000.0);
        d.update_lock(A);
        d.update_lock(A);
        d.update_lock(B);
        assert_eq!(d.detected(), A, "one window of another tone proves nothing");
        d.update_lock(B);
        assert_eq!(d.detected(), B);
    }

    /// The window has to be long enough to separate the tightest standard pair,
    /// which is what fixes it at 1024 samples.
    #[test]
    fn the_analysis_window_separates_the_closest_tones() {
        let win = hann(WINDOW);
        let a: Vec<f32> = win
            .iter()
            .enumerate()
            .map(|(n, w)| {
                let t = n as f32 / WORK_RATE as f32;
                (std::f32::consts::TAU * 67.0 * t).sin() * w
            })
            .collect();
        let on = goertzel(&a, 67.0, WORK_RATE).norm();
        let next = goertzel(&a, 69.3, WORK_RATE).norm();
        assert!(on > next * 6.0, "67.0 Hz leaks {:.1} dB into 69.3", 20.0 * (next / on).log10());
    }

    /// The Goertzel's phase has to carry the frequency, or the tolerance check
    /// that rejects off-table tones is measuring nothing.
    #[test]
    fn phase_advance_measures_the_offset_between_windows() {
        let win = hann(WINDOW);
        let build = |f: f64, start: usize| -> Complex32 {
            let a: Vec<f32> = win
                .iter()
                .enumerate()
                .map(|(n, w)| {
                    let t = (n + start) as f64 / WORK_RATE;
                    ((std::f64::consts::TAU * f * t).sin() as f32) * w
                })
                .collect();
            goertzel(&a, 100.0, WORK_RATE)
        };
        let per_hz = std::f64::consts::TAU * HOP as f64 / WORK_RATE;
        for off in [-1.5f64, -0.4, 0.0, 0.7, 2.0] {
            let (a, b) = (build(100.0 + off, 0), build(100.0 + off, HOP));
            let turned = (b * a.conj()).arg() as f64;
            let residual = turned - 100.0 * per_hz;
            let wrapped =
                residual - std::f64::consts::TAU * (residual / std::f64::consts::TAU).round();
            let measured = wrapped / per_hz;
            assert!((measured - off).abs() < 0.05, "offset {off} measured as {measured}");
        }
    }
}
