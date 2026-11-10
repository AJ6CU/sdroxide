//! STANAG 4285 — the receive side, uncoded rates.
//!
//! Transcribed from the standard itself (`docs/internal/STANAG4285-EDITION1.md`) rather than
//! from somebody's description of it, and **receive-only**: no modulator, and the
//! coded rates (Annex E) are not built.
//!
//! The frame, in full, because every one of these matters:
//!
//! * **256 symbols at 2400 baud** = 106.67 ms: **80 synchronisation** symbols,
//!   then 176 scrambled ones — three groups of `32 data + 16 reference`, then a
//!   final 32 data symbols with no reference after them.
//! * The synchronisation is **2PSK**, PN31 (two periods plus the first 18
//!   symbols), generator `x⁵ + x² + 1`, register set to `11010` at the start of
//!   every frame, **first symbol the LSB**, then 79 clocks.
//! * The 176 reference and data symbols are **scrambled** by `x⁹ + x⁴ + 1`, set
//!   to 1 each frame, **three bits per symbol**: `n = 4x₂ + 2x₁ + x₀` and
//!   `Bk = e^(jnπ/4)`, the register advancing three positions per symbol.
//! * The reference symbols are **all symbol 0 before scrambling**, which is the
//!   handle the whole receiver hangs on.
//!
//! ## Acquisition, and why it is a fold and not a peak
//!
//! The synchronisation sequence repeats every frame, so the correlation of the
//! received differential stream against the differentially encoded PN31 is a
//! **periodic** statistic: the right question is not "is there a peak here" but
//! "at which of the 256 symbol phases does this average over *many* frames stand
//! up". Measured on the night's captures: a single peak reaches 0.86 and so does
//! a strong PSK carrier of another mode (0.86 against 0.14 for the control when
//! the correlation is **folded over twelve frames**), while one frame's maximum
//! over every offset reaches 0.738 on white noise.
//!
//! Two more things have to be measured rather than assumed, and both were the
//! difference between 0.28 and 0.70 on the reference blocks:
//!
//! * **the sampling instant.** A receiver that resamples at a fixed phase lands
//!   anywhere inside the symbol. The fold is therefore searched over ten
//!   sub-symbol phases, and each frame refines its own timing again.
//! * **the carrier offset inside a block.** 16 symbols is 6.7 ms, so 150 Hz turns
//!   a whole circle inside one reference block and a perfectly good frame
//!   measures 0.3 instead of 0.9. It is corrected per block by a search, and the
//!   same correction is applied to the data.
//!
//! ## What this deliberately does not claim
//!
//! The three uncoded rates are 1200 BPSK, 2400 QPSK and 3600 8PSK, and **each of
//! those constellations is also used by coded rates** (1200 BPSK carries the
//! 75–600 bps rates at 1/16…1/2, QPSK carries 1200 at 1/2, 8PSK carries 2400 at
//! 2/3). A constellation test cannot tell them apart, so the rate is reported as
//! "uncoded, or a coded rate" and never as a claim. The coded payload needs
//! Annex E's FEC and interleaver, which are not transcribed, so the text of a
//! coded transmission is not readable here — and the payload bits are exposed so
//! that shows as bits rather than as silence.

use crate::rtty::BaudotRx;
use crate::stanag4285_fec as fec;
use std::f32::consts::TAU;

/// Symbol rate, fixed by the standard.
pub const BAUD: f64 = 2400.0;
/// Symbols per frame.
pub const FRAME: usize = 256;
/// Synchronisation symbols at the head of every frame.
pub const SYNC: usize = 80;
/// Scrambled symbols: 176 = 3 × (32 data + 16 reference) + 32 data.
pub const SCRAMBLED: usize = FRAME - SYNC;
/// Samples per symbol at the modem rate.
pub const SPS: usize = 5;
/// The modem's internal sample rate: five samples per symbol.
pub const MODEM_RATE: f64 = BAUD * SPS as f64;
/// The reference blocks, as offsets into the 176 scrambled symbols.
pub const REF_BLOCKS: [(usize, usize); 3] = [(32, 48), (80, 96), (128, 144)];
/// The data blocks, as offsets into the 176 scrambled symbols.
pub const DATA_BLOCKS: [(usize, usize); 4] = [(0, 32), (48, 80), (96, 128), (144, 176)];

/// Frames folded together to acquire, and to keep saying the lock is still there.
const ACQ_FRAMES: usize = 12;
/// Sub-symbol phases the acquisition searches. A tenth of a symbol is 42 µs,
/// and on a real capture the two best phases differed by exactly that.
const PHASES: usize = 10;

/// A complex value as a pair, so this module needs no extra dependency.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct C {
    pub re: f32,
    pub im: f32,
}

impl C {
    pub fn new(re: f32, im: f32) -> Self {
        C { re, im }
    }
    pub fn mul(self, o: C) -> C {
        C::new(self.re * o.re - self.im * o.im, self.re * o.im + self.im * o.re)
    }
    pub fn conj(self) -> C {
        C::new(self.re, -self.im)
    }
    pub fn norm(self) -> f32 {
        (self.re * self.re + self.im * self.im).sqrt()
    }
    pub fn scale(self, k: f32) -> C {
        C::new(self.re * k, self.im * k)
    }
    pub fn add(self, o: C) -> C {
        C::new(self.re + o.re, self.im + o.im)
    }
    pub fn sub(self, o: C) -> C {
        C::new(self.re - o.re, self.im - o.im)
    }
    pub fn from_phase(a: f32) -> C {
        C::new(a.cos(), a.sin())
    }
    pub fn arg(self) -> f32 {
        self.im.atan2(self.re)
    }
    fn lerp(self, o: C, t: f32) -> C {
        C::new(self.re + (o.re - self.re) * t, self.im + (o.im - self.im) * t)
    }
}

/// The data rate a frame carries, as far as its constellation can say.
///
/// Uncoded 3600 and **every coded rate** use all eight phases, so they cannot be
/// told apart at this layer; nor can 1200 BPSK from the 75–600 bps coded rates,
/// nor QPSK from 1200 at 1/2. The report says both rather than guessing, and that
/// is the honest ceiling of a constellation test.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S4285Rate {
    /// 1200 bps, one bit per symbol, 2PSK — or a coded BPSK rate (75–600).
    Bps1200,
    /// 2400 bps, a dibit per symbol, QPSK — or 1200 at rate 1/2.
    Bps2400,
    /// 3600 bps uncoded (8PSK) — or 2400 at rate 2/3.
    Bps3600OrCoded,
    /// Not yet decided — a frame whose references did not collapse.
    Unknown,
}

impl S4285Rate {
    pub fn label(self) -> &'static str {
        match self {
            S4285Rate::Bps1200 => "1200 BPSK uncoded, or a coded BPSK rate",
            S4285Rate::Bps2400 => "2400 QPSK uncoded, or 1200 QPSK rate 1/2",
            S4285Rate::Bps3600OrCoded => "3600 8PSK uncoded, or a coded rate",
            S4285Rate::Unknown => "unknown",
        }
    }

    /// Bits carried per symbol.
    pub fn bits_per_symbol(self) -> usize {
        match self {
            S4285Rate::Bps1200 => 1,
            S4285Rate::Bps2400 => 2,
            _ => 3,
        }
    }
}

/// One demodulated frame.
#[derive(Debug, Clone, PartialEq)]
pub struct S4285Frame {
    /// The three reference blocks' concentrations after descrambling, in order.
    /// **Reported separately and never averaged**: a channel that rotates within
    /// one frame separates the blocks while leaving each coherent, and an average
    /// hides exactly that.
    pub reference: [f32; 3],
    /// Carrier offset the reference blocks agree on, in hertz.
    pub carrier_offset_hz: f32,
    /// Which scrambler sign the frame needed.
    pub reversed: bool,
    /// What the descrambled constellation says.
    pub rate: S4285Rate,
    /// The distinct phases the descrambled data symbols used, 0..8.
    pub phases_used: usize,
    /// How tightly the data symbols sit on the 2-, 4- and 8-point lattices. Each
    /// is `|mean e^(jnφ)|`, which is independent of a rotation. A clean BPSK frame
    /// gives ~1, ~1, ~1; QPSK ~0, ~1, ~1; 8PSK ~0, ~0, ~1. Noise lowers all three,
    /// so the verdict compares them against each other rather than an absolute.
    pub constellation: [f32; 3],
    /// The same concentration, taken **per data block** rather than across the
    /// frame: the best of each block's 2-, 4- and 8-fold figures.
    ///
    /// This is the diagnostic that separates two faults a whole-frame number
    /// cannot. A phase that walks *between* blocks — one that a line through the
    /// three reference points cannot follow — leaves each block coherent and the
    /// frame spread, and shows up here as four figures well above
    /// [`S4285Frame::constellation`]. A phase that moves *inside* a block of 32
    /// symbols leaves even these flat, and only a tracker can follow that.
    pub block: [f32; 4],
    /// Payload bits of this frame, oldest first.
    pub bits: Vec<u8>,
    /// The frame's 128 soft decisions, positive for a zero — the values the
    /// Annex E chains are fed from. Exposed so a probe can ask whether a capture
    /// carries the code at all, which the hard bits cannot answer.
    pub soft: Vec<f32>,
    /// The 176 descrambled, channel-corrected symbols after the sync, oldest
    /// first. A frame's structure is otherwise inferred from its four published
    /// concentrations, which cannot say *where* an assumed layout went wrong:
    /// with the raw symbols a probe can look for the runs of constant phase the
    /// reference blocks are made of, and compare their positions with the ones the
    /// standard says they occupy.
    pub symbols: Vec<C>,
}

/// A message the Annex E layer assembled.
#[derive(Debug, Clone, PartialEq)]
pub struct S4285Message {
    /// Which coded rate produced it — the start-of-message correlation is what
    /// tells the four BPSK rates apart, since their constellations are identical.
    pub rate: fec::CodedRate,
    /// The message bits between the SOM and the EOM, oldest first.
    pub bits: Vec<u8>,
    /// An EOM was actually seen. Otherwise this is whatever had arrived.
    pub closed: bool,
}

/// Something the receiver produced.
#[derive(Debug, Clone, PartialEq)]
pub enum S4285Event {
    /// A frame was demodulated.
    Frame(S4285Frame),
    /// Text from the character layer, ITA2 5N1. The standard's payload is a
    /// synchronous bit stream, so this is a best reading of it — the raw bits are
    /// on the frame event for anything that is not ITA2.
    Text(String),
    /// A coded message came out of the Annex E chain, with the rate that
    /// produced it.
    Message(S4285Message),
}

/// The receiver's standing state, for a panel or a probe.
#[derive(Debug, Clone, PartialEq)]
pub struct S4285Report {
    /// A frame is being tracked.
    pub locked: bool,
    /// The folded differential sync correlation at the lock, 0..1. Folded over
    /// [`ACQ_FRAMES`] frames, which is what makes it a measurement: one frame's
    /// best peak over every offset reaches 0.74 on white noise.
    pub sync_quality: f32,
    /// The same correlation's median over the 256 phases: the null it is read
    /// against. On the night's captures the signal reads 0.85 and the control
    /// 0.14.
    pub correlation_floor: f32,
    /// The frame-period autocorrelation: how much of a frame repeats one frame
    /// later. A correct 4285 measures about **0.31-0.60** — the 80 sync symbols
    /// repeat — while a strong PSK carrier that is a different mode measures about
    /// **0.03**. Measured off the air, and the reason a sync correlation alone
    /// cannot tell the two apart.
    pub frame_acf: f32,
    /// Carrier offset measured from the reference blocks, in hertz.
    pub carrier_offset_hz: f32,
    /// Frames demodulated since the last reset.
    pub frames: u64,
    /// The last frame's rate verdict.
    pub rate: S4285Rate,
    /// The last frame's three reference concentrations.
    pub reference: [f32; 3],
    /// The last frame's 2-, 4- and 8-point constellation concentrations.
    pub constellation: [f32; 3],
    /// The sampling phase being tracked, in symbols (0..1).
    pub timing: f32,
    /// The coded rate a start-of-message pattern has been seen for. `None` until
    /// one has, because the four BPSK rates are identical at the constellation.
    pub coded: Option<fec::CodedRate>,
    /// Coded messages assembled by the Annex E layer this session.
    pub messages: u64,
    /// The closest the decoder's output has come to the start-of-message pattern,
    /// in matching bits out of 32. Noise gives about 16; a working decode on real
    /// traffic gets into the twenties even when a fade stops it matching.
    pub best_som: u8,
}

impl Default for S4285Report {
    fn default() -> Self {
        S4285Report {
            locked: false,
            sync_quality: 0.0,
            correlation_floor: 0.0,
            frame_acf: 0.0,
            carrier_offset_hz: 0.0,
            frames: 0,
            rate: S4285Rate::Unknown,
            reference: [0.0; 3],
            constellation: [0.0; 3],
            timing: 0.0,
            coded: None,
            messages: 0,
            best_som: 0,
        }
    }
}

/// A Fibonacci LFSR, least-significant bit first, as both of the standard's
/// generators are specified: the output is `x₀`, the feedback is the XOR of the
/// tapped stages, and the register shifts one position per output bit.
///
/// The stage count is the polynomial's degree, **not** one more than its highest
/// tap. `x⁵ + x² + 1` is five stages with taps {0, 2} and `x⁹ + x⁴ + 1` is nine
/// with taps {0, 4}; giving either a stage too many makes the register empty out
/// and stay at zero, which produced a sync sequence of period 16 and a scrambler
/// that was a constant — and the references then could not collapse, which read
/// as "no frame". `the_generators_are_maximal_and_match_the_standard` pins it.
fn lfsr(stages: usize, taps: &[usize], n: usize, init: u32) -> Vec<u8> {
    let mut state: Vec<u8> = (0..stages).map(|i| ((init >> i) & 1) as u8).collect();
    let mut out = Vec::with_capacity(n);
    for _ in 0..n {
        out.push(state[0]);
        let mut feedback = 0;
        for &t in taps {
            feedback ^= state[t];
        }
        state.remove(0);
        state.push(feedback);
    }
    out
}

/// The 80-symbol synchronisation sequence as ±1, PN31 clocked from `11010`.
pub fn sync_symbols() -> [f32; SYNC] {
    let bits = lfsr(5, &[0, 2], SYNC, 0b11010);
    let mut out = [0.0f32; SYNC];
    for (o, b) in out.iter_mut().zip(&bits) {
        *o = if *b == 1 { 1.0 } else { -1.0 };
    }
    out
}

/// The products of adjacent sync symbols: what a differential detector produces,
/// so this is what the correlator has to be matched to.
pub fn sync_differential() -> [f32; SYNC - 1] {
    let s = sync_symbols();
    let mut out = [0.0f32; SYNC - 1];
    for i in 0..SYNC - 1 {
        out[i] = s[i + 1] * s[i];
    }
    out
}

/// The 176-symbol scrambling sequence, `Bk = e^(jnπ/4)` with
/// `n = 4x₂ + 2x₁ + x₀`, the register advancing three positions per symbol.
pub fn scrambling() -> [C; SCRAMBLED] {
    // **Initialised to all ones.** This started at `1` — a single bit set — and
    // the sequence differs from the first symbol on. The sync is transmitted
    // *unscrambled*, so a receiver with the wrong scrambler still locks (0.90 on
    // Mark's recording) and still folds its frames, while the reference blocks and
    // the data come out smeared. Measured on that recording: references 0.55 of
    // collapsed, data constellation 0.396. With all ones they read 0.845 and
    // 0.750. All ones is the standard's own convention and it is what the
    // independent s4285 implementation in `luwangg/signal-analysis` uses.
    let bits = lfsr(9, &[0, 4], SCRAMBLED * 3, 0x1FF);
    let mut out = [C::default(); SCRAMBLED];
    for (k, o) in out.iter_mut().enumerate() {
        let n = 4 * bits[3 * k + 2] as i32 + 2 * bits[3 * k + 1] as i32 + bits[3 * k] as i32;
        *o = C::from_phase(n as f32 * std::f32::consts::FRAC_PI_4);
    }
    out
}

/// A windowed-sinc low-pass at the input rate, applied **before** any resampling.
/// Filtering first is not tidiness: interpolating a full-band signal folds the
/// rest of the band into the channel.
struct Lowpass {
    taps: Vec<f32>,
    buf: Vec<C>,
    pos: usize,
}

impl Lowpass {
    fn new(rate: f64, cutoff_hz: f64, len: usize) -> Self {
        let mid = (len / 2) as isize;
        let fc = (cutoff_hz / rate) as f32;
        let mut taps = Vec::with_capacity(len);
        let mut sum = 0.0f32;
        for i in 0..len {
            let x = i as isize - mid;
            let sinc = if x == 0 {
                2.0 * fc
            } else {
                (TAU as f32 * fc * x as f32).sin() / (std::f32::consts::PI * x as f32)
            };
            let w = 0.54 - 0.46 * ((TAU as f32) * i as f32 / (len - 1) as f32).cos();
            taps.push(sinc * w);
            sum += sinc * w;
        }
        for t in taps.iter_mut() {
            *t /= sum;
        }
        Lowpass { taps, buf: vec![C::default(); len], pos: 0 }
    }

    fn push(&mut self, z: C) -> C {
        let n = self.taps.len();
        self.buf[self.pos] = z;
        let mut acc = C::default();
        let mut k = self.pos;
        for &t in &self.taps {
            acc = acc.add(self.buf[k].scale(t));
            k = if k + 1 == n { 0 } else { k + 1 };
        }
        self.pos = if self.pos + 1 == n { 0 } else { self.pos + 1 };
        acc
    }
}

/// One coded-rate branch: deinterleaved soft bits in, decoded bits into a
/// message framer.
///
/// The four BPSK rates share one deinterleaved stream — the interleaver's `j` is
/// 12 for all of them — and differ only in how many repetitions of each coded
/// pair are averaged back. Two signs are run because the frame's absolute phase
/// is only known up to ±π until a start-of-message pattern says otherwise; a
/// flipped stream decodes to noise, so the branch that never sees an SOM costs
/// nothing but cycles.
const CHUNK_PAIRS: usize = 256;
/// Bits decoded at the front of each chunk and discarded, so the decoder's
/// forced start state has left the trace before the decisions are used.
const VITERBI_OVERLAP: usize = 64;

/// **Every branch the code can be on.** Four BPSK rates, both scrambler signs;
/// and the QPSK rate, both signs and both orders of its two bits.
///
/// The QPSK branches used to be created *inside* a loop over
/// [`fec::CodedRate::BPSK`], which is the four BPSK rates — so `r == Bps1200`
/// was never true and the QPSK chain was dead code. A capture whose verdict is
/// QPSK therefore had nothing listening for it at all. Built as a list rather
/// than nested conditionals so that cannot happen again: a rate is missing from
/// here or it is missing everywhere.
fn coded_chains() -> Vec<CodedChain> {
    let mut out = Vec::with_capacity(12);
    for sign in [1.0f32, -1.0] {
        for r in fec::CodedRate::BPSK {
            out.push(CodedChain::new(r, sign));
        }
        // **A QPSK symbol carries two bits and the annex does not say which of
        // I and Q enters the code first**, so both orders are run; only the one
        // that frames a message can be right.
        out.push(CodedChain::new(fec::CodedRate::Bps1200, sign));
        out.push(CodedChain::new_ordered(fec::CodedRate::Bps1200, sign, true));
    }
    out
}

/// One coded branch's counters — see [`S4285Receiver::chain_stats`].
#[derive(Debug, Clone, Copy)]
pub struct ChainStat {
    pub rate: fec::CodedRate,
    pub sign: f32,
    /// For a QPSK rate, which of the two bits comes first.
    pub swap: bool,
    /// Soft decisions this branch has been fed; one per data symbol per stride.
    pub pushed: usize,
    /// Viterbi chunks it has decoded, after the interleaver.
    pub chunks: u64,
    pub messages: u64,
    /// Closest approach to the start-of-message pattern, in matching bits of 32.
    pub som: u8,
    /// How many times this branch was restarted.
    pub resets: u64,
}

struct CodedChain {
    rate: fec::CodedRate,
    times: usize,
    sign: f32,
    /// For a rate whose symbols carry **two** bits (QPSK), which of the two comes
    /// first. Both orders are run: the annexes this fork was built from do not say,
    /// and only the one that frames a message can be right.
    swap: bool,
    deint: fec::ConvInterleaver,
    /// Deinterleaved soft values not yet decoded.
    soft: Vec<f32>,
    consumed: usize,
    framer: fec::MessageFramer,
    /// Messages this branch has framed; the first one is what names the rate.
    messages: u64,
    /// Soft decisions it has ever been fed — the honest count, unlike the buffer
    /// length, which is drained as it is consumed.
    fed: u64,
    /// Viterbi chunks decoded. **A counter, not `consumed / 512`**: the buffer is
    /// drained as it fills, so a figure derived from `consumed` collapses back to
    /// nothing every 64 kB and reads as "the chain decoded ten chunks out of a
    /// hundred and forty". It did read that, and it was the counter lying.
    decoded: u64,
    /// How many times it has been restarted. A restart throws away the
    /// interleaver's ten seconds of history, so a stream that restarts often can
    /// never complete a message.
    resets: u64,
}

impl CodedChain {
    /// Soft values this branch takes per data symbol: two for a QPSK rate, one
    /// for the BPSK ones.
    fn stride(&self) -> usize {
        if self.rate == fec::CodedRate::Bps1200 { 2 } else { 1 }
    }
}

impl CodedChain {
    fn new(rate: fec::CodedRate, sign: f32) -> Self {
        Self::new_ordered(rate, sign, false)
    }

    fn new_ordered(rate: fec::CodedRate, sign: f32, swap: bool) -> Self {
        CodedChain {
            rate,
            times: rate.repeat(),
            sign,
            swap,
            deint: fec::ConvInterleaver::deinterleaver(rate.j()),
            soft: Vec::new(),
            consumed: 0,
            framer: fec::MessageFramer::new(rate),
            messages: 0,
            fed: 0,
            decoded: 0,
            resets: 0,
        }
    }

    fn reset(&mut self) {
        self.resets += 1;
        self.deint.reset();
        self.soft.clear();
        self.consumed = 0;
        self.framer.reset();
    }

    /// One soft decision from the demodulator (positive means a zero).
    fn push(&mut self, v: f32, out: &mut Vec<S4285Message>) {
        self.fed += 1;
        self.soft.push(self.deint.push(v * self.sign));
        // A chunk of `CHUNK_PAIRS` coded pairs, plus an overlap in front of it
        // that is decoded and **thrown away**.
        //
        // The overlap is in front, not behind, and that is the whole point: the
        // decoder is started with all its metric on state zero, and the traceback
        // runs backwards from the end, so the *opening* of a chunk is the part the
        // forced start distorts. Using the first `CHUNK_PAIRS` decisions threw
        // away the good ones and kept the bad: on a clean generated signal it put
        // a burst of errors in the middle of every message, 256 bits apart.
        let need = (CHUNK_PAIRS + VITERBI_OVERLAP) * 2 * self.times;
        while self.soft.len() - self.consumed >= need {
            let slice = &self.soft[self.consumed..self.consumed + need];
            let pairs = fec::average_pairs(slice, self.times);
            self.decoded += 1;
            let bits = fec::viterbi_decode(&pairs);
            for b in bits.iter().skip(VITERBI_OVERLAP).take(CHUNK_PAIRS) {
                if let Some(m) = self.framer.push(*b) {
                    self.messages += 1;
                    out.push(S4285Message { rate: m.rate, bits: m.bits, closed: m.closed });
                }
            }
            self.consumed += CHUNK_PAIRS * 2 * self.times;
        }
        // Keep the buffer from growing without bound: everything before
        // `consumed` has been decoded and is dead weight.
        if self.consumed > 64 * 1024 {
            self.soft.drain(..self.consumed);
            self.consumed = 0;
        }
    }
}

/// Where the frame head sits, once acquisition has found it.
#[derive(Debug, Clone, Copy)]
struct Lock {
    /// Absolute index, in baseband samples, of the next frame's first symbol.
    ///
    /// Fractional, because the frame **step** is tracked and not fixed: two sound
    /// cards' clocks differ by far more than the standard's 10 ppm, and a fixed
    /// 256-symbol stride walks out of the timing window after a handful of frames
    /// — measured at ~500 ppm on this bench, which is one symbol every eight
    /// frames.
    next: f64,
}

/// The receiver. Feed it audio at `rate_hz`; it acquires on a fold of the
/// synchronisation, emits a frame per frame once locked, and text as the
/// character layer decodes.
/// Which of the two channel paths a coded frame is read through.
///
/// The frame's own reading (`take_frame`) already chooses between the plain
/// symbols and the least-squares equaliser by asking which one collapses the
/// known reference blocks harder, and that is the right question for a *displayed*
/// constellation. It is not obviously the right question for the **code**: an
/// equaliser that sharpens nine reference symbols per block can still leave the
/// data's eye worse, and on a real recording the trellis has never been seen.
/// So the choice is a parameter, swept against the metric in `s4285_fec_check`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum EqMode {
    /// Whichever path the reference blocks prefer, frame by frame. The default,
    /// and what a station watching the constellation wants.
    #[default]
    Auto,
    /// The plain symbols, with no equaliser at all.
    Off,
    /// The equaliser's output, whether or not the references prefer it.
    On,
    /// **The reference implementation's own equaliser**, ported: a 121-tap
    /// fractionally-spaced LMS filter, four symbol taps, one training pass symbol
    /// by symbol through the sync and the reference blocks, exactly as
    /// `luwangg/signal-analysis`'s `channel_estimator` does it. That decoder reads
    /// this station's capture and ours does not, and this is the difference.
    Lms,
    /// **An adaptive decision-feedback equaliser, run symbol by symbol.** This is
    /// the one thing every other decoder for this mode has and this receiver did
    /// not: a fractionally-spaced feedforward filter trained on the 80 sync
    /// symbols, then decision-directed through the frame with the known reference
    /// blocks correcting it, so that inter-symbol interference — which a line
    /// through three reference points cannot see — is taken out of the data. See
    /// [`S4285Receiver::dfe_symbols`].
    Dfe,
}

pub struct S4285Receiver {
    rate: f64,
    carrier: f64,
    nco: f32,
    nco_inc: f32,
    lpf: Lowpass,
    step: f64,
    frac: f64,
    prev_bb: C,
    have_prev: bool,
    /// Baseband at [`SPS`] samples per symbol, newest last.
    bb: Vec<C>,
    /// Absolute index of `bb[0]`, in baseband samples.
    base: u64,
    /// Frames searched at each acquisition attempt, so a hopeless channel is not
    /// re-folded on every audio block.
    acq_wait: usize,
    lock: Option<Lock>,
    /// The tracked sampling phase, in symbols.
    timing: f32,
    scrambling: [C; SCRAMBLED],
    sync_diff: [f32; SYNC - 1],
    baudot: BaudotRx,
    /// The uncoded path's 5N1.5 framer — see [`fec::Ita2Framer`].
    ita2: fec::Ita2Framer,
    /// The eight Annex E branches: four BPSK rates, each with both signs.
    coded: Vec<CodedChain>,
    /// The rate a start-of-message pattern has been seen for, once one has.
    coded_rate: Option<fec::CodedRate>,
    /// Messages the coded layer has assembled.
    coded_messages: u64,
    /// Messages ready to be handed out by the next `process` call.
    pending: Vec<S4285Message>,
    /// How the coded path picks its channel — see [`EqMode`].
    eq_mode: EqMode,
    /// Forward taps, feedback taps, training passes and step size for
    /// [`Self::dfe_symbols`]. **Settable from the bench only** — the defaults
    /// are what the receiver runs — because the choice has to be swept against
    /// *decoded bits*, not against how tightly the constellation reads.
    dfe_params: (usize, usize, usize, f32),
    /// Running noise estimate for the log-likelihood soft decisions: the mean
    /// squared distance from each symbol to the point it decided on, smoothed
    /// the way the reference implementation smooths it.
    llr_var: f32,
    /// How many symbols have gone into that estimate.
    llr_n: u32,
    /// The equaliser's weights, carried between frames. SCRATCH: on trial.
    dfe_taps: Option<(Vec<C>, Vec<C>)>,
    /// Every soft decision the Annex E chains were actually fed, erasures
    /// included, when tracing is on — see [`S4285Receiver::coded_stream`].
    coded_trace: Option<Vec<f32>>,
    /// The same, one entry per frame: the frame's mean reference concentration, or
    /// zero where it was fed as erasures. See [`S4285Receiver::coded_quality`].
    coded_quality: Vec<f32>,
    /// The last frame reached the coded-chain floor, whether or not it was good
    /// enough to report. A frame that is merely poor must not cost the lock: the
    /// lock is about the frame *position*, which such a frame still proves.
    last_frame_usable: bool,
    bit_run: Vec<u8>,
    /// Characters decoded but not yet handed out.
    events_text: String,
    report: S4285Report,
}

/// How many baseband samples the acquisition window spans.
const ACQ_SPAN: u64 = (ACQ_FRAMES as u64 + 1) * FRAME as u64 * SPS as u64;

impl S4285Receiver {
    /// Choose how the coded path picks its channel. Only the frames that feed the
    /// Annex E chains are affected; the displayed frame keeps [`EqMode::Auto`].
    /// Sweep the equaliser's shape — see [`Self::dfe_params`]. Bench only.
    pub fn set_dfe_params(&mut self, ff: usize, fb: usize, passes: usize, mu: f32) {
        self.dfe_params = (ff.max(3) | 1, fb.max(1), passes.max(1), mu);
    }

    pub fn set_eq_mode(&mut self, mode: EqMode) {
        self.eq_mode = mode;
    }

    /// Start or stop recording the stream the Annex E chains are fed.
    ///
    /// **This is not the same stream as the frames' own soft decisions**, and the
    /// difference matters to anything judging whether the code is there. A frame
    /// too poor to report is still fed — as 128 zeros — so the interleaver stays
    /// aligned with the frame index through a fade; those values never reach a
    /// frame event. A judge that reads the frame events alone is therefore reading
    /// a stream with holes that the decoder did not have, and a hole shifts every
    /// soft decision after it by a whole frame.
    pub fn trace_coded_stream(&mut self, on: bool) {
        if on {
            if self.coded_trace.is_none() {
                self.coded_trace = Some(Vec::new());
                self.coded_quality.clear();
            }
        } else {
            self.coded_trace = None;
        }
    }

    /// The per-frame quality behind [`S4285Receiver::coded_stream`]: each frame's
    /// mean reference concentration, in order, with zero for a frame fed as
    /// erasures.
    ///
    /// **A coded message needs a continuous run of these, not a good average.** A
    /// convolutional interleaver spreads one message over ten seconds, so a
    /// recording whose good frames are scattered through a fade cannot carry a
    /// message whatever the decoder does — and that is a fact about the recording,
    /// measurable before any decode, which is the difference between "the receiver
    /// is broken" and "this tape is too short".
    pub fn coded_quality(&self) -> &[f32] {
        &self.coded_quality
    }

    /// The values [`S4285Receiver::trace_coded_stream`] collected, in order.
    pub fn coded_stream(&self) -> &[f32] {
        self.coded_trace.as_deref().unwrap_or(&[])
    }

    /// `rate_hz` is the audio tap rate; `carrier_hz` is the audio frequency of the
    /// suppressed carrier — 1800 Hz when the dial is on it, but a fixed-frequency
    /// log entry can be a kilohertz out, so it is a parameter.
    pub fn new(rate_hz: f64, carrier_hz: f64) -> Self {
        S4285Receiver {
            rate: rate_hz,
            carrier: carrier_hz,
            nco: 0.0,
            nco_inc: (TAU as f32) * carrier_hz as f32 / rate_hz as f32,
            lpf: Lowpass::new(rate_hz, 1500.0, 129),
            step: rate_hz / MODEM_RATE,
            frac: 0.0,
            prev_bb: C::default(),
            have_prev: false,
            bb: Vec::new(),
            base: 0,
            acq_wait: 0,
            lock: None,
            timing: 0.0,
            scrambling: scrambling(),
            sync_diff: sync_differential(),
            baudot: BaudotRx::new(),
            ita2: fec::Ita2Framer::new(),
            coded: Vec::new(),
            coded_rate: None,
            coded_messages: 0,
            pending: Vec::new(),
            eq_mode: EqMode::Auto,
            dfe_params: (11, 4, 2, 0.05),
            llr_var: 0.0,
            llr_n: 0,
            dfe_taps: None,
            coded_trace: None,
            coded_quality: Vec::new(),
            last_frame_usable: false,
            bit_run: Vec::new(),
            events_text: String::new(),
            report: S4285Report::default(),
        }
    }

    /// **Per-chain counters, for a test that has to tell "the chains never
    /// filled" from "they filled and the code still failed".** Not part of the
    /// report: this is diagnostics for the bench, not something a panel shows.
    pub fn chain_stats(&self) -> Vec<ChainStat> {
        self.coded
            .iter()
            .map(|c| ChainStat {
                rate: c.rate,
                sign: c.sign,
                swap: c.swap,
                pushed: c.fed as usize,
                chunks: c.decoded,
                messages: c.messages,
                som: c.framer.best_som,
                resets: c.resets,
            })
            .collect()
    }

    pub fn report(&self) -> &S4285Report {
        &self.report
    }

    /// The raw payload bits recovered so far, oldest first — exposed because a
    /// transmission that is not ITA2 must be visible as bits, not as silence.
    pub fn bits(&self) -> &[u8] {
        &self.bit_run
    }

    pub fn carrier_hz(&self) -> f64 {
        self.carrier
    }

    /// Retune the suppressed carrier and start again: acquisition cannot survive a
    /// jump, and a stale lock on a new frequency would be a lie.
    pub fn set_carrier(&mut self, carrier_hz: f64) {
        self.carrier = carrier_hz;
        self.nco_inc = (TAU as f32) * carrier_hz as f32 / self.rate as f32;
        self.reset();
    }

    pub fn reset(&mut self) {
        self.nco = 0.0;
        self.frac = 0.0;
        self.have_prev = false;
        self.bb.clear();
        self.base = 0;
        self.acq_wait = 0;
        self.lock = None;
        self.timing = 0.0;
        for c in self.coded.iter_mut() {
            c.reset();
        }
        self.coded_rate = None;
        self.bit_run.clear();
        self.ita2.reset();
        self.report = S4285Report::default();
    }

    /// Feed audio; returns whatever it produced.
    pub fn process(&mut self, audio: &[f32]) -> Vec<S4285Event> {
        let mut events = Vec::new();
        for &s in audio {
            let lo = C::from_phase(-self.nco);
            self.nco += self.nco_inc;
            if self.nco > TAU as f32 {
                self.nco -= TAU as f32;
            }
            let z = self.lpf.push(C::new(s, 0.0).mul(lo));
            if !self.have_prev {
                self.prev_bb = z;
                self.have_prev = true;
                continue;
            }
            while self.frac < 1.0 {
                self.bb.push(self.prev_bb.lerp(z, self.frac as f32));
                self.frac += self.step;
            }
            self.frac -= 1.0;
            self.prev_bb = z;
        }

        match self.lock {
            None => {
                if self.bb.len() as u64 >= ACQ_SPAN + SPS as u64 {
                    self.acquire();
                }
            }
            Some(_) => self.drain_frames(&mut events),
        }
        self.trim();
        events
    }

    /// A symbol at a fractional sample position, linearly interpolated. The
    /// baseband is band-limited to 1.5 kHz and sampled at 12 kHz, so a straight
    /// line between two samples is four times oversampled.
    fn at(&self, pos: f64) -> C {
        let i = pos.floor();
        if i < 0.0 {
            return C::default();
        }
        let i = i as usize;
        if i + 1 >= self.bb.len() {
            return self.bb.last().copied().unwrap_or_default();
        }
        self.bb[i].lerp(self.bb[i + 1], (pos - i as f64) as f32)
    }

    /// `FRAME` symbols from a frame head at `start` (a baseband sample index) and
    /// a sampling phase in symbols.
    fn frame_symbols(&self, start: u64, phase: f32) -> [C; FRAME] {
        let mut out = [C::default(); FRAME];
        for (i, o) in out.iter_mut().enumerate() {
            let pos = start as f64 - self.base as f64 + (i as f32 + phase) as f64 * SPS as f64;
            *o = self.at(pos);
        }
        out
    }

    /// The frame-period autocorrelation of the baseband: how much of a frame
    /// repeats one frame later. Independent of the sampling phase, which is why it
    /// can be reported before acquisition has found one.
    ///
    /// It is taken over **everything buffered**, not over one frame. One frame's
    /// worth of a modulated signal is 1280 samples of a statistic that has to see
    /// many frames to mean anything: measured on a live 12.857 MHz capture, one
    /// frame's estimate read 0.06 where the whole 6-second buffer read 0.355
    /// against its own null of 0.019 — the instrument, not the signal.
    fn frame_acf(&self) -> f32 {
        let lag = FRAME * SPS;
        let n = self.bb.len();
        if n < lag + FRAME * SPS {
            return 0.0;
        }
        let take = n - lag;
        let mut acc = C::default();
        let (mut ea, mut eb) = (0.0f32, 0.0f32);
        for k in 0..take {
            let a = self.bb[n - take - lag + k];
            let b = self.bb[n - take + k];
            acc = acc.add(b.mul(a.conj()));
            ea += a.norm() * a.norm();
            eb += b.norm() * b.norm();
        }
        let denom = (ea * eb).sqrt();
        if denom <= 1e-20 {
            return 0.0;
        }
        (acc.norm() / denom).min(1.0)
    }

    /// Fold the differential sync correlation over every frame phase, at each
    /// sub-symbol sampling phase, and take the standing-up one.
    fn acquire(&mut self) {
        if self.acq_wait > 0 {
            self.acq_wait -= 1;
            return;
        }
        let n = self.bb.len();
        if n < ACQ_SPAN as usize {
            return;
        }
        let start = self.base + (n as u64 - ACQ_SPAN);
        let acf = self.frame_acf();
        self.report.frame_acf = acf;
        if acf < MIN_FRAME_ACF {
            self.report.locked = false;
            self.report.correlation_floor = 0.0;
            self.report.sync_quality = 0.0;
            self.acq_wait = ACQ_FRAMES;
            return;
        }

        let count = ACQ_SPAN as usize / SPS;
        let mut best = (0.0f32, 0.0f32, 0usize, 0.0f32);
        for p in 0..PHASES {
            let phase = p as f32 / PHASES as f32;
            // Symbols over the whole window, at this sampling phase.
            let mut sym = Vec::with_capacity(count);
            for k in 0..count {
                let pos = start as f64 - self.base as f64 + (k as f32 + phase) as f64 * SPS as f64;
                sym.push(self.at(pos));
            }
            // Differential products, then the correlation over the sync window.
            let mut corr = vec![0.0f32; count];
            let n = self.sync_diff.len();
            for k in n..count {
                let mut acc = C::default();
                let mut mag = 0.0f32;
                for (j, &w) in self.sync_diff.iter().enumerate() {
                    let i = k - n + j;
                    let d = sym[i + 1].mul(sym[i].conj());
                    acc = acc.add(d.scale(w));
                    mag += d.norm();
                }
                if mag > 1e-12 {
                    corr[k] = acc.norm() / mag;
                }
            }
            // Fold: mean per phase index, skipping the head where no window fits.
            // `corr[k]` covers symbols `k - n ..= k`, so the window's **first**
            // symbol — the frame head — is `k - n`, and that is what is folded.
            let mut sum = [0.0f32; FRAME];
            let mut num = [0u32; FRAME];
            for (k, &c) in corr.iter().enumerate().skip(n) {
                sum[(k - n) % FRAME] += c;
                num[(k - n) % FRAME] += 1;
            }
            let folded: Vec<f32> = sum
                .iter()
                .zip(&num)
                .map(|(s, m)| if *m > 0 { *s / *m as f32 } else { 0.0 })
                .collect();
            let mut sorted = folded.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let median = sorted[FRAME / 2];
            let (o, &score) = folded
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap_or((0, &0.0));
            if score > best.0 {
                best = (score, median, o, phase);
            }
        }

        let (score, median, offset, phase) = best;
        self.report.correlation_floor = median;
        self.report.sync_quality = score;
        // A folded correlation has to stand well clear of its own null. Measured:
        // 0.85 on the signal, 0.14 on the control, so 2.2x is the middle with room.
        if score < MIN_SYNC || score < median * 2.2 {
            self.report.locked = false;
            self.acq_wait = ACQ_FRAMES;
            return;
        }
        // The fold says where the frame head is; from here every frame is taken at
        // a fixed stride and refines its own timing.
        self.lock = Some(Lock { next: (start + (offset * SPS) as u64) as f64 });
        self.timing = phase;
        self.report.locked = true;
    }

    fn drain_frames(&mut self, events: &mut Vec<S4285Event>) {
        loop {
            let Some(lock) = self.lock else { return };
            if lock.next + (FRAME * SPS) as f64 > (self.base + self.bb.len() as u64) as f64 {
                return;
            }
            let frame = self.take_frame(lock.next.round() as u64);
            match frame {
                Some(f) => {
                    events.push(S4285Event::Frame(f.clone()));
                    for m in self.pending.drain(..) {
                        // The character layer, if the message reads as text. A
                        // reading that does not pass the test stays silent: letters
                        // that mean nothing are the one failure a listener cannot
                        // check.
                        if let Some(t) = fec::message_text(&m.bits) {
                            self.events_text.push_str(&t);
                            events.push(S4285Event::Text(std::mem::take(&mut self.events_text)));
                        }
                        events.push(S4285Event::Message(m));
                    }
                    // Text only from a frame whose constellation is resolved: an
                    // unresolved one produces letters that look plausible and mean
                    // nothing, which is the one failure a listener cannot see. The
                    // bits are still on the event and in `bits()`.
                    //
                    // **5N1.5, across the whole transmission.** The bits are a
                    // continuous teleprinter stream, not five-bit groups: a
                    // character puts a start bit, five data bits and a stop bit on
                    // the wire, so chunking each frame's 128 bits into fives is the
                    // wrong width *and* restarts the alignment every frame — which
                    // is why the uncoded path could never read ITA2 however clean
                    // the signal was. See `fec::Ita2Framer`.
                    if f.rate != S4285Rate::Unknown {
                        for b in &f.bits {
                            if let Some(v) = self.ita2.push(*b) {
                                if let Some(ch) = self.baudot.decode(v) {
                                    self.events_text.push(ch);
                                }
                            }
                        }
                    }
                    if !self.events_text.is_empty() {
                        let t: String = self.events_text.drain(..).collect();
                        events.push(S4285Event::Text(t));
                    }
                    if let Some(l) = self.lock.as_mut() {
                        l.next += (FRAME * SPS) as f64;
                    }
                }
                None => {
                    // Poor enough to report nothing — but if it still reached the
                    // coded floor its *position* is sound and the lock stands, with
                    // its 128 soft decisions already in the chains as erasures.
                    // Letting go here would restart the interleaver on the first
                    // fade of every message.
                    if self.last_frame_usable {
                        if let Some(l) = self.lock.as_mut() {
                            l.next += (FRAME * SPS) as f64;
                        }
                        continue;
                    }
                    // The frame did not stand up at all: let go and acquire again
                    // from what is still buffered rather than demodulating a lie.
                    self.report.locked = false;
                    self.lock = None;
                    self.acq_wait = 0;
                    return;
                }
            }
        }
    }

    /// Demodulate the frame whose head is at absolute sample `start`.
    /// The frame's 176 scrambled symbols, equalised across the path.
    ///
    /// The one real coded tape we have cannot put its data symbols on a lattice
    /// while its sync and reference blocks are healthy, and its data *blocks* are
    /// no better than the frame — so the damage is inside 32 symbols, which is
    /// delay spread and not a phase drift. **The channel is estimated from the 80
    /// known sync symbols by least squares and inverted in closed form**; an
    /// adaptive loop trained on the same symbols does not converge (measured, see
    /// the handover). It is used only on a frame where it makes the known
    /// reference symbols collapse harder than they already do, so a channel that
    /// needs no equalising is read exactly as before.
    /// A decision-feedback equaliser over one frame, symbol by symbol.
    ///
    /// The receiver's channel model was a straight line through the three
    /// reference blocks, and that is enough for phase and level but not for
    /// **inter-symbol interference**: on Mark's recording the reference blocks
    /// collapse at 0.76–0.95 while the data constellation sits at 0.75 of a
    /// two-fold concentration, which is interference and not noise. Both other
    /// decoders for this mode — Sigmira and `luwangg/signal-analysis` — answer it
    /// with an LMS decision-feedback equaliser, and this is that, in the same
    /// shape: a fractionally-spaced feedforward filter (eleven taps at five
    /// samples per symbol, one symbol either side) plus a short decision-feedback
    /// filter on the symbols already decided.
    ///
    /// The known symbols do the training and the correcting: the 80-symbol sync
    /// at the head of the frame is a transition on almost every symbol, and the
    /// three 16-symbol reference blocks — which the standard puts there for
    /// exactly this — pull the equaliser back on track mid-frame. Between them the
    /// filter is decision-directed.
    ///
    /// Returns the **descrambled** symbols, so everything downstream (the soft
    /// decisions, the Annex E chains) is unchanged.
    fn dfe_symbols(
        &mut self,
        start: u64,
        phase: f32,
        reversed: bool,
        offset: f32,
    ) -> [C; SCRAMBLED] {
        // Swept on Mark's recording, judged by the data constellation the frame
        // reports: 11 forward taps with 4 feedback taps over two passes reads
        // 0.833 where the receiver's own channel fit reads 0.750, and both more
        // taps and more passes measured worse (21/8 → 0.713, 15/6 over three →
        // 0.761) because 256 symbols a frame is not enough to train a longer
        // filter.
        let (ff, fb, passes, mu) = self.dfe_params;
        let centre = ff / 2;
        let sync = sync_symbols();
        let pos0 = start as f64 - self.base as f64 + (phase as f64) * SPS as f64;
        // One sample vector per symbol, deramped so a carrier offset does not look
        // like a channel ramp, then scaled to unit power so one step size works at
        // any recording level.
        let mut xs: Vec<Vec<C>> = Vec::with_capacity(FRAME);
        for i in 0..FRAME {
            let mut v = vec![C::default(); ff];
            for (j, slot) in v.iter_mut().enumerate() {
                let d = j as f64 - centre as f64;
                let t = (i as f64 * SPS as f64 + d) / MODEM_RATE as f64;
                *slot = self
                    .at(pos0 + i as f64 * SPS as f64 + d)
                    .mul(C::from_phase(-offset * TAU * t as f32));
            }
            xs.push(v);
        }
        let power = xs.iter().map(|v| v[centre].norm().powi(2)).sum::<f32>() / FRAME as f32;
        let norm = if power > 1e-12 { 1.0 / power.sqrt() } else { 1.0 };
        for v in xs.iter_mut() {
            for c in v.iter_mut() {
                *c = c.scale(norm);
            }
        }

        let (mut w, mut b) = match self.dfe_taps.take() {
            Some((w, b)) if w.len() == ff && b.len() == fb => (w, b),
            _ => {
                let mut w = vec![C::default(); ff];
                w[centre] = C::new(1.0, 0.0);
                (w, vec![C::default(); fb])
            }
        };
        let mut past = vec![C::default(); fb];
        let mut out = [C::default(); SCRAMBLED];
        // Two passes: the first trains the filter on the known symbols and its own
        // decisions, the second reads the frame through the filter it has learnt.
        for pass in 0..passes {
            for i in 0..FRAME {
                let mut y = C::default();
                for (j, c) in xs[i].iter().enumerate() {
                    y = y.add(c.mul(w[j]));
                }
                for (l, d) in past.iter().enumerate() {
                    y = y.add(d.mul(b[l]));
                }
                let want = if i < SYNC {
                    C::new(sync[i], 0.0)
                } else {
                    let k = i - SYNC;
                    let sc = if reversed { self.scrambling[k].conj() } else { self.scrambling[k] };
                    if REF_BLOCKS.iter().any(|&(lo, hi)| k >= lo && k < hi) {
                        sc
                    } else {
                        // Decision-directed: decide on the descrambled symbol, then
                        // return it to the transmitted (scrambled) domain, which is
                        // the domain the interference lives in.
                        let d = y.mul(sc.conj());
                        let bit = if d.re >= 0.0 { 1.0 } else { -1.0 };
                        sc.scale(bit)
                    }
                };
                let err = y.sub(want);
                for (j, c) in xs[i].iter().enumerate() {
                    w[j] = w[j].sub(err.mul(c.conj()).scale(mu));
                }
                for l in 0..fb {
                    b[l] = b[l].sub(err.mul(past[l].conj()).scale(mu));
                }
                // Shift the decided symbols into the feedback line, newest first.
                for l in (1..fb).rev() {
                    past[l] = past[l - 1];
                }
                past[0] = want;
                if pass + 1 == passes && i >= SYNC {
                    let k = i - SYNC;
                    let sc = if reversed { self.scrambling[k].conj() } else { self.scrambling[k] };
                    let mut y2 = C::default();
                    for (j, c) in xs[i].iter().enumerate() {
                        y2 = y2.add(c.mul(w[j]));
                    }
                    for (l, d) in past.iter().enumerate() {
                        y2 = y2.add(d.mul(b[l]));
                    }
                    out[k] = y2.mul(sc.conj());
                }
            }
        }
        self.dfe_taps = Some((w, b));
        out
    }

    /// **The reference implementation's equaliser, ported.**
    ///
    /// `luwangg/signal-analysis`'s `channel_estimator::process_frame`, which
    /// decodes this station's capture where this receiver does not. Their numbers:
    /// 121 feedforward taps spanning ±12 symbols at five samples per symbol, four
    /// decision-feedback taps, one pass, `mu = 0.05` on **raw** samples, the 80
    /// sync symbols as known ±1, the three reference blocks as known scrambled
    /// symbols, and decision-directed elsewhere. The filter starts from **zero**
    /// taps, not a unit centre: LMS from zero converges to the matched filter
    /// across the sync, which is what those 80 symbols are for.
    fn lms_symbols(&self, start: u64, phase: f32, reversed: bool, offset: f32) -> [C; SCRAMBLED] {
        const FF: usize = 121; // twelve taps at two samples a symbol, plus the centre
        const FB: usize = 4;
        const MU: f32 = 0.05;
        let sync = sync_symbols();
        let pos0 = start as f64 - self.base as f64 + (phase as f64) * SPS as f64;
        let ctx_len = FRAME * SPS + FF;
        // **No per-sample deramp here.** The reference's PLL has already taken the
        // carrier off its frame buffer; a ramp of our own across the filter's
        // ±12-symbol reach is a rotation the filter then has to fight, which is
        // what makes it fail to converge.
        let _ = offset;
        let mut ctx: Vec<C> = Vec::with_capacity(ctx_len);
        for k in 0..ctx_len {
            let d = k as f64 - (FF / 2) as f64;
            ctx.push(self.at(pos0 + d));
        }
        let mut w = vec![C::default(); FF];
        let mut b = vec![C::default(); FB];
        let mut known = vec![C::default(); FB];
        let mut out = [C::default(); SCRAMBLED];
        for i in 0..FRAME {
            let x = &ctx[i * SPS..i * SPS + FF];
            let mut y = C::default();
            for (j, c) in x.iter().enumerate() {
                y = y.add(c.mul(w[j]));
            }
            for (l, d) in known.iter().enumerate() {
                y = y.add(d.mul(b[l]));
            }
            let k = i.wrapping_sub(SYNC);
            let sc = if i < SYNC {
                C::default()
            } else {
                let c = self.scrambling[k];
                if reversed { c.conj() } else { c }
            };
            let target = if i < SYNC {
                C::new(sync[i], 0.0)
            } else if REF_BLOCKS.iter().any(|&(lo, hi)| k >= lo && k < hi) {
                sc
            } else {
                let d = y.mul(sc.conj());
                let decided = C::new(if d.re >= 0.0 { 1.0 } else { -1.0 }, 0.0);
                sc.mul(decided)
            };
            let err = y.sub(target);
            for (j, c) in x.iter().enumerate() {
                w[j] = w[j].sub(err.mul(c.conj()).scale(MU));
            }
            for l in 0..FB {
                b[l] = b[l].sub(err.mul(known[l].conj()).scale(MU));
            }
            for l in (1..FB).rev() {
                known[l] = known[l - 1];
            }
            known[0] = target;
            if i >= SYNC {
                out[k] = y.mul(sc.conj());
            }
        }
        out
    }

    fn coded_symbols(
        &mut self,
        start: u64,
        phase: f32,
        reversed: bool,
        offset: f32,
    ) -> [C; SCRAMBLED] {
        if self.eq_mode == EqMode::Lms {
            return self.lms_symbols(start, phase, reversed, offset);
        }
        if self.eq_mode == EqMode::Dfe {
            return self.dfe_symbols(start, phase, reversed, offset);
        }
        let frame = self.frame_symbols(start, phase);
        // **The half-symbol grid as well as the symbol instants.** A T-spaced
        // filter cannot describe a pulse-shaped signal — see [`ls_equaliser`] — so
        // the equaliser is fed the frame sampled at twice the symbol rate. The odd
        // samples are the same frame half a symbol later.
        let half = self.frame_symbols(start, phase + 0.5);
        // The carrier comes off first: the estimate is taken against real ±1 sync
        // symbols, and a ramp left in looks like a channel that turns a whole
        // circle every 27 symbols.
        let deramp = |t: f32, v: C| -> C { v.mul(C::from_phase(-offset * TAU * t)) };
        let mut d = [C::default(); FRAME];
        let mut rf = [C::default(); 2 * FRAME];
        for k in 0..FRAME {
            let t = k as f32 / BAUD as f32;
            d[k] = deramp(t, frame[k]);
            rf[2 * k] = d[k];
            rf[2 * k + 1] = deramp(t + 0.5 / BAUD as f32, half[k]);
        }
        let mut plain = [C::default(); SCRAMBLED];
        for k in 0..SCRAMBLED {
            let sc = if reversed { self.scrambling[k].conj() } else { self.scrambling[k] };
            plain[k] = d[SYNC + k].mul(sc.conj());
        }

        // **The frame's own phase reference.** The reference blocks are known to be
        // symbol 0, so their mean phase is the global rotation the path has put on
        // the frame, and the sync symbols the estimate is taken against are real.
        let mut ref_phase = C::default();
        for &(lo, hi) in &REF_BLOCKS {
            for k in lo..hi {
                ref_phase = ref_phase.add(plain[k]);
            }
        }
        let rot = C::from_phase(-ref_phase.arg());
        let mut r = d;
        for v in r.iter_mut() {
            *v = v.mul(rot);
        }
        for v in rf.iter_mut() {
            *v = v.mul(rot);
        }

        let sync = sync_symbols();
        let mut corr = C::default();
        for k in 0..SYNC {
            corr = corr.add(r[k].scale(sync[k]));
        }
        let sg = if corr.re < 0.0 { -1.0 } else { 1.0 };
        let known: Vec<C> = sync.iter().map(|v| C::new(sg * v, 0.0)).collect();
        let equaliser = ls_equaliser(&rf, &known, LS_FRAC_TAPS);

        // **No group delay to search.** The least-squares filter's target is the
        // symbol sent at each instant, so its output is aligned by construction —
        // the three shifts the old inverse needed were a guess at its delay, and
        // getting it wrong sampled the data either side of itself.
        let mut equalised = [C::default(); SCRAMBLED];
        if let Some(w) = &equaliser {
            for k in 0..SCRAMBLED {
                let centre = 2 * (SYNC + k);
                let mut y = C::default();
                for (i, wi) in w.iter().enumerate() {
                    if let Some(v) = centre.checked_sub(i).and_then(|j| rf.get(j)) {
                        y = y.add(v.mul(*wi));
                    }
                }
                let sc = if reversed { self.scrambling[k].conj() } else { self.scrambling[k] };
                equalised[k] = y.mul(sc.conj());
            }
        }

        // **It has to earn its place, frame by frame.** On a channel that needed
        // nothing it can only add a little of its own noise — measured, it cost a
        // ramped clean signal its exact bits. The sixteen known reference symbols
        // in each block decide it: whichever path makes them collapse harder is
        // the path the frame is read through.
        let score = |z: &[C; SCRAMBLED]| {
            let c = ramp_concentrations(z, 0.0);
            c[0] + c[1] + c[2]
        };
        match self.eq_mode {
            // The displayed frame's own rule, which is also the default here.
            EqMode::Auto => {
                if equaliser.is_some() && score(&equalised) > score(&plain) {
                    equalised
                } else {
                    plain
                }
            }
            EqMode::Off => plain,
            EqMode::On => {
                if equaliser.is_some() {
                    equalised
                } else {
                    plain
                }
            }
            // Unreachable: `coded_symbols` returns the DFE's and the LMS filter's
            // own symbols before any of this runs.
            EqMode::Dfe | EqMode::Lms => plain,
        }
    }

    /// Refine the carrier offset against the **known sync**, and not only against
    /// the reference blocks.
    ///
    /// The reference blocks are three samples of the channel's phase across a
    /// frame, and a two-path channel makes them read as a ramp: measured on the
    /// synthetic two-path channel the search lands on **13 Hz where the truth is
    /// 0**, which rotates the frame by nearly three radians across the sync and
    /// leaves a least-squares channel unable to fit anything. The 80 known symbols
    /// are 160 constraints rather than three points, so they answer the question
    /// the references are being asked to guess at.
    fn refine_offset(&self, start: u64, phase: f32, offset: f32) -> f32 {
        let frame = self.frame_symbols(start, phase);
        let sync = sync_symbols();
        let mut best = (f32::INFINITY, offset);
        let mut hz = offset - 20.0;
        while hz <= offset + 20.0 {
            let mut d = [C::default(); FRAME];
            for (k, v) in d.iter_mut().enumerate() {
                let t = k as f32 / BAUD as f32;
                *v = frame[k].mul(C::from_phase(-hz * TAU * t));
            }
            let (res, _) = sync_fit(&d, &sync);
            if res < best.0 {
                best = (res, hz);
            }
            hz += 0.5;
        }
        best.1
    }

    fn take_frame(&mut self, start: u64) -> Option<S4285Frame> {
        // The sampling phase is searched over **most of a symbol**, not over a
        // fifth of one. Acquisition answers it to a tenth of a symbol, and a
        // tracked value that starts half a symbol out never recovers if the
        // per-frame search can only reach ±0.2: on a clean generated signal that
        // cost a handful of bit errors in the middle of a message, which is
        // exactly how a decode looks when it is nearly right.
        let mut best: Option<(f32, f32, bool, f32)> = None;
        for step in -9i32..=9 {
            // The window is anchored at **half a symbol inside the frame**, not on
            // the last frame's answer: it is the clock tracker below that keeps the
            // true sampling instant there, and a window that follows its own
            // previous answer simply walks away with it.
            let phase = 0.5 + step as f32 * 0.05;
            let frame = self.frame_symbols(start, phase);
            for &sign in &[1.0f32, -1.0f32] {
                let mut z = [C::default(); SCRAMBLED];
                for k in 0..SCRAMBLED {
                    let s = if sign > 0.0 { self.scrambling[k] } else { self.scrambling[k].conj() };
                    z[k] = frame[SYNC + k].mul(s.conj());
                }
                // Coarse ramp, then a fine one round the best. A 16-symbol block
                // is 6.67 ms, so 150 Hz turns a whole circle inside it.
                let mut coarse = (0.0f32, 0.0f32);
                let mut hz = -150.0f32;
                while hz <= 150.0 {
                    let c = ramp_concentrations(&z, hz);
                    let r = (c[0] + c[1] + c[2]) / 3.0;
                    if r > coarse.0 {
                        coarse = (r, hz);
                    }
                    hz += 5.0;
                }
                let mut fine = coarse;
                let mut hz = coarse.1 - 5.0;
                while hz <= coarse.1 + 5.0 {
                    let c = ramp_concentrations(&z, hz);
                    let r = (c[0] + c[1] + c[2]) / 3.0;
                    if r > fine.0 {
                        fine = (r, hz);
                    }
                    hz += 1.0;
                }
                if best.as_ref().is_none_or(|b| fine.0 > b.0) {
                    best = Some((fine.0, phase, sign < 0.0, fine.1));
                }
            }
        }
        let (score, mut phase, reversed, offset) = best?;
        // **Not from the references alone.** A two-path channel bends the three
        // reference points into a ramp the search mistakes for a carrier error;
        // the known sync symbols do not lie about it.
        let offset = self.refine_offset(start, phase, offset);
        self.last_frame_usable = score >= MIN_CODED_REFERENCE;
        // **The sampling instant has to come from the data, not from the
        // references.** Sixteen identical symbols score equally well anywhere
        // across them, so a phase search that maximises the reference
        // concentration is flat in time — it cannot tell a correct sampling
        // instant from half a symbol out, and on a real path a frame decoded half
        // a symbol late has every data symbol smeared while its reference blocks
        // still read 0.9. That is exactly the shape of the failure this cost on
        // the bench.
        //
        // So once the sign and the carrier offset are known, the phase is chosen
        // again — this time by how tightly the *data* symbols land on the
        // constellation.
        {
            let mut tight = (0.0f32, phase);
            for step in -10i32..=10 {
                let p = phase + step as f32 * 0.05;
                let frame = self.frame_symbols(start, p);
                let mut z = [C::default(); SCRAMBLED];
                for k in 0..SCRAMBLED {
                    let sc = if reversed { self.scrambling[k].conj() } else { self.scrambling[k] };
                    let t = k as f32 / BAUD as f32;
                    z[k] = frame[SYNC + k].mul(sc.conj()).mul(C::from_phase(-offset * TAU * t));
                }
                // The channel first, then the data: on a real path the three
                // reference blocks differ in phase *and level*, and a
                // concentration measured on the raw data is dominated by that
                // rotation rather than by the sampling instant — which makes the
                // sweep answer noise.
                let fit = ChannelFit::from_references(&z);
                let (mut c2, mut c4, mut c8) = (C::default(), C::default(), C::default());
                let mut n = 0.0f32;
                for &(lo, hi) in &DATA_BLOCKS {
                    for k in lo..hi {
                        let c = fit.apply(k, z[k]);
                        let a = c.arg();
                        c2 = c2.add(C::from_phase(2.0 * a));
                        c4 = c4.add(C::from_phase(4.0 * a));
                        c8 = c8.add(C::from_phase(8.0 * a));
                        n += 1.0;
                    }
                }
                let data_conc = (c2.norm() / n).max(c4.norm() / n).max(c8.norm() / n);
                // The references are added in, not used alone. The data is what
                // decides where the eye is open — sixteen identical symbols
                // cannot — but the reference concentration is the number the
                // panel shows and the gate reads, and it does vary a little with
                // the instant; scoring both keeps the reported figure honest at
                // the instant the data wants.
                let refs = ramp_concentrations(&z, 0.0);
                // **And the sync's own fit, which is the only honest timing
                // signal here.** `data_conc` is circular on a smeared channel: it
                // asks the data where the eye is, and the smear is what is closing
                // it. The 80 known sync symbols have a transition on almost every
                // symbol, so their least-squares residual says whether the sampling
                // instant is right without asking the data anything. Weighted to
                // dominate, because when the two disagree the sequence is right.
                let mut deramped = [C::default(); FRAME];
                for (k, v) in deramped.iter_mut().enumerate() {
                    let t = k as f32 / BAUD as f32;
                    *v = frame[k].mul(C::from_phase(-offset * TAU * t));
                }
                let (residual, _) = sync_fit(&deramped, &sync_symbols());
                let timing = 1.0 - residual.min(1.0);
                let v = data_conc + (refs[0] + refs[1] + refs[2]) / 3.0 + 2.0 * timing;
                if v > tight.0 {
                    tight = (v, p);
                }
            }
            phase = tight.1;
        }
        // Annex E's chains are fed from the frame *before* the reference gate: the
        // stream into the interleaver has to be continuous, and a frame too poor to
        // report is still a frame's worth of soft decisions that the code can use.
        // A frame that is missing altogether breaks the stream, so the chains are
        // reset wherever one is skipped.
        let mut coded_msgs = Vec::new();
        let mut soft = Vec::new();
        if score >= MIN_CODED_REFERENCE {
            soft = self.feed_coded(start, phase, reversed, offset, &mut coded_msgs);
        } else {
            // A frame too poor to decode is **not** a reason to start the chains
            // again: its position in the stream is known, so its 128 missing soft
            // decisions go in as erasures. A zero contributes nothing to either
            // hypothesis, which is what an erasure is — and a restart would throw
            // away the ten seconds of interleaver that precede this frame, which
            // on a fading path means never completing a message at all.
            self.feed_erasures(&mut coded_msgs);
        }
        for m in coded_msgs {
            self.coded_messages += 1;
            self.report.messages = self.coded_messages;
            if self.coded_rate.is_none() {
                self.coded_rate = Some(m.rate);
                self.report.coded = Some(m.rate);
            }
            self.pending.push(m);
        }
        if score < MIN_REFERENCE {
            return None;
        }
        // The carrier offset has to come off the symbols themselves, not just off
        // the score: the channel phase is read from the reference blocks, and a
        // ramp left in place turns a whole circle between two of them, so the
        // interpolation across the frame would be nonsense.
        let z = self.coded_symbols(start, phase, reversed, offset);
        let reference = ramp_concentrations(&z, 0.0);
        // **Track the symbol clock, not only the phase.** The frame stride is a
        // number of samples, and two clocks that differ by even a few hundred ppm
        // walk that stride out of the ±0.45-symbol window in a handful of frames.
        // The best sampling phase inside a frame is the error signal: it sits at
        // 0.5 when the stride is right, and its departure from 0.5 is how far the
        // next frame head has to move.
        // The error is taken **modulo one symbol**: the phase sweep can answer
        // outside 0..1 (it reaches ±0.5 past the window's centre), and an error of
        // −0.6 read literally walks the frame head the wrong way every frame until
        // the timing runs away.
        let mut err = phase - 0.5;
        if err > 0.5 {
            err -= 1.0;
        } else if err < -0.5 {
            err += 1.0;
        }
        if let Some(l) = self.lock.as_mut() {
            if err.abs() < 0.45 {
                l.next += (err * SPS as f32 * 0.5).clamp(-2.0, 2.0) as f64;
            }
        }
        self.timing = phase.rem_euclid(1.0);

        // The channel inside the frame, from its three reference blocks — phase
        // and level. See `ChannelFit`.
        let fit = ChannelFit::from_references(&z);

        let mut used = [false; 8];
        let mut data: Vec<C> = Vec::with_capacity(128);
        for &(lo, hi) in &DATA_BLOCKS {
            for k in lo..hi {
                let c = fit.apply(k, z[k]);
                let idx = (c.arg() / std::f32::consts::FRAC_PI_4).round() as i32;
                used[idx.rem_euclid(8) as usize] = true;
                data.push(c);
            }
        }
        let phases_used = used.iter().filter(|u| **u).count();

        // The constellation, as concentrations that a rotation cannot change.
        let (mut c2, mut c4, mut c8) = (C::default(), C::default(), C::default());
        for c in &data {
            c2 = c2.add(C::from_phase(2.0 * c.arg()));
            c4 = c4.add(C::from_phase(4.0 * c.arg()));
            c8 = c8.add(C::from_phase(8.0 * c.arg()));
        }
        let n = data.len().max(1) as f32;
        let constellation = [c2.norm() / n, c4.norm() / n, c8.norm() / n];

        // Per block, with the same channel: see `S4285Frame::block`.
        let mut block = [0.0f32; 4];
        for (b, &(lo, hi)) in DATA_BLOCKS.iter().enumerate() {
            let (mut b2, mut b4, mut b8) = (C::default(), C::default(), C::default());
            for k in lo..hi {
                let a = fit.apply(k, z[k]).arg();
                b2 = b2.add(C::from_phase(2.0 * a));
                b4 = b4.add(C::from_phase(4.0 * a));
                b8 = b8.add(C::from_phase(8.0 * a));
            }
            let nb = (hi - lo).max(1) as f32;
            block[b] = (b2.norm() / nb).max(b4.norm() / nb).max(b8.norm() / nb);
        }

        // A 2PSK frame leaves the even symbol numbers only; a dibit frame the
        // multiples of two; a tribit frame all eight. Noise smears each of those,
        // so the classes are told apart by which concentration stands up, not by
        // counting phases that noise has already spread over all eight.
        let rate = rate_from_concentrations(constellation);
        let bits = transcode(&data, rate);

        self.report.frames += 1;
        self.report.rate = rate;
        self.report.reference = reference;
        self.report.constellation = constellation;
        self.report.carrier_offset_hz = offset;
        self.report.timing = self.timing;
        self.bit_run.extend_from_slice(&bits);
        if self.bit_run.len() > 8192 {
            let cut = self.bit_run.len() - 8192;
            self.bit_run.drain(..cut);
        }
        // `z` is **already descrambled** by `coded_symbols`; applying the scrambler
        // again here is what made this dump look like noise on every capture and
        // sent a session's worth of phase histograms down the wrong road.
        let symbols: Vec<C> = z.to_vec();
        Some(S4285Frame {
            symbols,
            reference,
            carrier_offset_hz: offset,
            reversed,
            rate,
            phases_used,
            constellation,
            block,
            bits,
            soft,
        })
    }

    /// Descramble one frame's data again for the Annex E chains and push its 128
    /// soft decisions into each branch.
    ///
    /// The chain runs on the *raw* constellation, not on the rate verdict: the
    /// four BPSK rates are identical at the constellation, and the SOM is what
    /// tells them apart.
    fn feed_coded(
        &mut self,
        start: u64,
        phase: f32,
        reversed: bool,
        offset: f32,
        out: &mut Vec<S4285Message>,
    ) -> Vec<f32> {
        let z = self.coded_symbols(start, phase, reversed, offset);
        // **The same channel line the frame itself is read through.** Without it
        // the soft decisions keep the frame's residual phase, which is a different
        // number every frame on anything that is not exactly on frequency — and a
        // BPSK decision's whole content is its sign, so a rotating phase turns the
        // code stream into noise while the constellation still reads 0.99 and the
        // references still collapse. Measured: a 1 Hz/s ramp, well inside the
        // standard's 3.5 Hz/s, took the message from readable to nothing. The
        // level is corrected with the phase (see `ChannelFit`), which is what a
        // recording whose audio filter tilts across the frame needs.
        let fit = ChannelFit::from_references(&z);
        let mut data: Vec<C> = Vec::with_capacity(128);
        let mut sum = 0.0f32;
        for &(lo, hi) in &DATA_BLOCKS {
            for k in lo..hi {
                let c = fit.apply(k, z[k]);
                sum += c.norm();
                data.push(c);
            }
        }
        if sum <= 1e-9 {
            return Vec::new();
        }
        // **Log-likelihood soft decisions, the way the reference implementation
        // makes them.** A BPSK symbol's evidence is the distance to the two
        // points it could have been, divided by a running estimate of the noise
        // variance: a clean symbol saturates, a marginal one counts for little,
        // and a fading frame does not drown the strong symbols beside it.
        //
        // The linear real part (what this did) weights every symbol by its own
        // amplitude, so on a fading path a burst of strong symbols outvotes
        // everything else and the Viterbi follows them into error.
        let mut llr: Vec<f32> = Vec::with_capacity(data.len());
        for c in &data {
            // The two BPSK points are +1 and -1 on the real axis.
            let d0 = (c.re - 1.0) * (c.re - 1.0) + c.im * c.im;
            let d1 = (c.re + 1.0) * (c.re + 1.0) + c.im * c.im;
            let near = d0.min(d1);
            self.llr_n = (self.llr_n + 1).min(4096);
            let alpha = 1.0 / self.llr_n as f32;
            self.llr_var += (2.0 * near - self.llr_var) * alpha;
            let var = self.llr_var.max(1e-4);
            // Positive for a symbol that decided on "zero", which is this tree's
            // signed convention; scaled so a clean one is about ±3.
            llr.push(((d1 - d0) / var).clamp(-6.0, 6.0));
        }
        let scale = data.len() as f32 / sum;
        if self.coded.is_empty() {
            self.coded = coded_chains();
        }
        // **The linear reading, because the log-likelihood one measured worse on a
        // clean signal.** Swapping it in broke two synthetic coded chains that had
        // been decoding (a two-path channel and a truncated band), and produced no
        // message on the capture either. The estimate is kept — it is what the
        // reference uses and it may be what the front-end port needs — but it is
        // not wired in on this evidence.
        let _ = &llr;
        let soft: Vec<f32> = data.iter().map(|c| c.re * scale).collect();
        // The same symbols as two bits each.
        let mut soft2: Vec<f32> = Vec::with_capacity(data.len() * 2);
        for c in &data {
            soft2.push(c.re * scale);
            soft2.push(c.im * scale);
        }
        // **The trace is the two-bit stream**, because that is what the QPSK chain
        // is fed; a judge reading one bit per symbol would be judging a stream that
        // chain never saw. The BPSK branches take every second value of it.
        if self.coded_trace.is_some() {
            let r = ramp_concentrations(&z, 0.0);
            self.coded_quality.push((r[0] + r[1] + r[2]) / 3.0);
        }
        if let Some(t) = self.coded_trace.as_mut() {
            t.extend_from_slice(&soft2);
        }
        for chain in self.coded.iter_mut() {
            if chain.rate == fec::CodedRate::Bps1200 {
                for pair in soft2.chunks_exact(2) {
                    let (first, second) =
                        if chain.swap { (pair[1], pair[0]) } else { (pair[0], pair[1]) };
                    chain.push(first, out);
                    chain.push(second, out);
                }
                if chain.framer.best_som > self.report.best_som {
                    self.report.best_som = chain.framer.best_som;
                }
                continue;
            }
            for v in &soft {
                chain.push(*v, out);
            }
            if chain.framer.best_som > self.report.best_som {
                self.report.best_som = chain.framer.best_som;
            }
        }
        if let Some(rate) = self.coded_rate {
            // Only the branch that has produced a message is worth decoding; the
            // others are noise by definition once the rate is known.
            self.coded.retain(|c| c.rate == rate && c.messages > 0);
        }
        soft
    }

    /// One frame's worth of soft decisions as erasures, to keep the Annex E
    /// chains aligned with the frame index when a frame is too poor to read.
    fn feed_erasures(&mut self, out: &mut Vec<S4285Message>) {
        if self.coded.is_empty() {
            return;
        }
        if self.coded_trace.is_some() {
            self.coded_quality.push(0.0);
        }
        if let Some(t) = self.coded_trace.as_mut() {
            // Two values per symbol in the trace; a frame's worth.
            t.extend(std::iter::repeat_n(0.0f32, 256));
        }
        for chain in self.coded.iter_mut() {
            // A frame's worth for that chain's stride: a QPSK chain takes two
            // values per symbol, so half a frame of zeros would shift it for good.
            for _ in 0..128 * chain.stride() {
                chain.push(0.0, out);
            }
        }
    }

    /// Keep only what the next frame and the acquisition window need.
    fn trim(&mut self) {
        let need = match self.lock {
            Some(l) => (l.next as u64).saturating_sub(self.base),
            None => 0,
        };
        let keep_from = need.min(self.bb.len() as u64) as usize;
        let drop = match self.lock {
            Some(_) => keep_from,
            None => self.bb.len().saturating_sub(ACQ_SPAN as usize + SPS),
        };
        if drop > 4096 {
            self.bb.drain(..drop);
            self.base += drop as u64;
        }
    }

    /// Characters decoded but not yet handed out.
    pub fn take_text(&mut self) -> String {
        std::mem::take(&mut self.events_text)
    }
}

/// The mean reference concentration a block needs before it is called a frame.
///
/// Sixteen symbols of noise give `1/sqrt(16)` = 0.25; a frame whose blocks have
/// collapsed measured 0.65–0.81 across the night's captures. 0.45 sits between
/// them and is deliberately nearer the noise floor than the signal: a frame that
/// is wrongly accepted is visible in the three numbers, while one wrongly rejected
/// is silence.
const MIN_REFERENCE: f32 = 0.45;

/// The reference concentration below which a frame's soft decisions are too poor
/// to be worth handing to the code, but above which they still are: the reported
/// three numbers and the coded stream want different things from a fading frame.
const MIN_CODED_REFERENCE: f32 = 0.30;

/// The folded sync correlation a lock needs, and the margin over its own null.
///
/// A correct 4285 measures 0.85 folded over twelve frames; the strong PSK carrier
/// that is a different mode measures 0.14 on the same statistic, and a single
/// frame's maximum over every offset reaches 0.738 on white noise — which is why
/// the peak alone was never a detector.
const MIN_SYNC: f32 = 0.40;

/// The frame-period autocorrelation a lock needs.
///
/// A correct 4285 measures 0.31-0.60 (its 80 sync symbols repeat out of 256) and a
/// strong PSK carrier of another mode measures ~0.03, both measured. 0.12 sits
/// between them with room for a fading capture.
const MIN_FRAME_ACF: f32 = 0.12;

/// The channel across one frame, from its three reference blocks: the phase at
/// each block, and the least-squares line through them.
///
/// This is what the reference symbols are *for*. A single ramp corrects a carrier
/// offset; it does not correct a channel whose phase at the third block is not
/// where the first two predict — and on a real path it usually is not. The data
/// has to be derotated by the line, not by the ramp alone, or every data block
/// after the first is rotated by whatever the channel did between them. On a
/// clean file that difference is nil, which is exactly why it survived so long.
/// The complex channel across one frame, read from its three reference blocks.
///
/// A straight **phase** line was enough on a clean file and on the bench path, and
/// it is not enough on a real station: the level and the phase both move inside
/// the 107 ms frame, so the data reads as a smear while the references still
/// collapse. The first real coded capture we were sent measures exactly that —
/// references 0.69 / 0.76 / 0.61 and a data constellation of 0.34 — and its
/// spectrum is 2.2 kHz wide, so the tilt is the recording's filter as much as the
/// path. This fits **I and Q** linearly across the frame, interpolating between
/// the reference blocks and extrapolating to the data blocks at either end, so a
/// symbol is corrected in amplitude as well as in phase.
///
/// The slope is unambiguous because the carrier offset is taken off the symbols
/// before this is called: left in, 55 Hz turns a whole circle over 48 symbols and
/// the fit wraps. See `take_frame`.
#[derive(Clone, Copy)]
struct ChannelFit {
    slope: C,
    intercept: C,
}

/// Taps in the least-squares channel estimate used by [`sync_fit`]. Five covers
/// an echo up to two symbols late once the fractional delay is split across two
/// taps, and it is short enough to estimate from one frame's 80 known sync
/// symbols.
const LS_TAPS: usize = 5;
/// Taps in the equaliser, at **two per symbol** — nine and a half symbols of span.
///
/// A symbol-spaced estimate cannot describe a pulse-shaped signal: the raised
/// cosine's own memory is in the received samples, so a five-tap T-spaced fit
/// leaves the pulse in the residual and its inverse then fights it. Measured on
/// the two-path synthetic, that inverse made the known reference blocks collapse
/// *worse* than the plain symbols (2.81 → 2.02) and the frame's own gate declined
/// it, so the equaliser was never used on the audio it existed for. Half-symbol
/// spacing gives the filter the fractional delay a path puts on a symbol and the
/// pulse's shape as well as the path's echoes; nine symbols is longer than the
/// pulse plus a 1.5-symbol echo and still short enough to train on one frame.
const LS_FRAC_TAPS: usize = 19;

/// Divide by a complex number, which `C` has no operator for.
fn cdiv(a: C, b: C) -> C {
    let d = b.norm() * b.norm();
    if d <= 1e-12 {
        return C::default();
    }
    a.mul(b.conj()).scale(1.0 / d)
}

/// Solve `a·x = b` for a small dense complex system, by Gauss-Jordan with
/// partial pivoting. `n` is at most [`LS_EQ`], so the cost is nothing.
fn solve(mut a: Vec<Vec<C>>, mut b: Vec<C>) -> Option<Vec<C>> {
    let n = b.len();
    for col in 0..n {
        let (piv, _) = (col..n)
            .map(|r| (r, a[r][col].norm()))
            .max_by(|x, y| x.1.partial_cmp(&y.1).unwrap_or(std::cmp::Ordering::Equal))?;
        if a[piv][col].norm() < 1e-9 {
            return None;
        }
        a.swap(col, piv);
        b.swap(col, piv);
        let d = a[col][col];
        for j in col..n {
            a[col][j] = cdiv(a[col][j], d);
        }
        b[col] = cdiv(b[col], d);
        for r in 0..n {
            if r == col {
                continue;
            }
            let f = a[r][col];
            if f.norm() <= 1e-12 {
                continue;
            }
            for j in col..n {
                a[r][j] = a[r][j].sub(f.mul(a[col][j]));
            }
            b[r] = b[r].sub(f.mul(b[col]));
        }
    }
    Some(b)
}

/// Estimate the channel from **known** symbols, by least squares.
///
/// `rx[k]` is the received symbol at instant `k`, `known[k]` what was sent, and
/// the model is `rx[k] = Σ h[i]·known[k-i]` — an FIR, which is what a path with
/// a few milliseconds of delay spread looks like at 2400 baud. The normal
/// equations are small and real-symmetric in structure, so they are solved
/// directly rather than iterated towards: an adaptive loop on 80 symbols does
/// not converge here, measured, and this does not have to.
fn ls_channel(rx: &[C], known: &[C], taps: usize) -> Option<Vec<C>> {
    if rx.len() <= taps || known.len() != rx.len() {
        return None;
    }
    // Rows are the equations, columns the tap index: A[i][j] = conj(k[n-i])·k[n-j].
    let mut ata = vec![vec![C::default(); taps]; taps];
    let mut atb = vec![C::default(); taps];
    for n in taps - 1..rx.len() {
        for i in 0..taps {
            let xi = known[n - i].conj();
            for j in 0..taps {
                ata[i][j] = ata[i][j].add(xi.mul(known[n - j]));
            }
            atb[i] = atb[i].add(xi.mul(rx[n]));
        }
    }
    solve(ata, atb)
}

/// The least-squares residual of the **known** sync against a frame, and the
/// frame's own phase rotation.
///
/// This is the timing error signal the data cannot give: the sync has a transition
/// on almost every symbol, so a misfit means the sampling instant is wrong, while
/// the data-tightness statistic is circular on a smeared channel — the channel
/// estimate and the constellation both want the eye that the smear is closing.
/// The residual is normalised by the sync's own received amplitude, so it is a
/// fraction rather than a level and one threshold works at any AGC setting.
fn sync_fit(r: &[C; FRAME], sync: &[f32; SYNC]) -> (f32, C) {
    let mut ref_phase = C::default();
    for &(lo, hi) in &REF_BLOCKS {
        for k in lo..hi {
            ref_phase = ref_phase.add(r[SYNC + k]);
        }
    }
    let rot = C::from_phase(-ref_phase.arg());
    let mut r2 = *r;
    for v in r2.iter_mut() {
        *v = v.mul(rot);
    }
    let mut corr = C::default();
    let mut amp = 0.0f32;
    for k in 0..SYNC {
        corr = corr.add(r2[k].scale(sync[k]));
        amp += r2[k].norm();
    }
    amp /= SYNC as f32;
    if amp <= 1e-9 {
        return (1.0, rot);
    }
    let sg = if corr.re < 0.0 { -1.0 } else { 1.0 };
    let known: Vec<C> = sync.iter().map(|v| C::new(sg * v, 0.0)).collect();
    let Some(h) = ls_channel(&r2[..SYNC], &known, LS_TAPS) else { return (1.0, rot) };
    let mut err = 0.0f32;
    let mut n = 0.0f32;
    for k in LS_TAPS - 1..SYNC {
        let mut y = C::default();
        for (i, hi) in h.iter().enumerate() {
            if let Some(v) = k.checked_sub(i).map(|j| known[j]) {
                y = y.add(v.mul(*hi));
            }
        }
        let e = r2[k].sub(y).norm();
        err += e * e;
        n += 1.0;
    }
    ((err / n.max(1.0)).sqrt() / amp, rot)
}

/// The least-squares equaliser, over **half-symbol-spaced** samples.
///
/// `rf[2n]` is the received sample at symbol instant `n` and `rf[2n+1]` the one
/// half a symbol later; `known[n]` is what was sent at `n`. The filter is the `w`
/// that minimises `Σ_n |Σ_j w[j]·rf[2n−j] − known[n]|²`, which is the normal
/// equations `(AᴴA)·w = Aᴴd` written out — the closed form of "undo the pulse,
/// the echo and the sampling instant together", instead of estimating a channel
/// and then inverting it and hoping the two models agree.
///
/// **The answer is aligned, and that is the other half of the fault.** The target
/// is the symbol sent at `n`, not a delayed copy of it, so the solution's own
/// group delay is zero by construction: where a unit pulse lands is a property of
/// the target, not something the caller has to search over. The first version
/// estimated a channel and inverted it in the minimum-energy form, which does
/// have a delay, and the three shifts it tried were a guess at it.
fn ls_equaliser(rf: &[C], known: &[C], taps: usize) -> Option<Vec<C>> {
    let n = taps;
    if known.len() * 2 > rf.len() || known.len() <= n + 8 {
        return None;
    }
    let mut ata = vec![vec![C::default(); n]; n];
    let mut atb = vec![C::default(); n];
    let mut rows = 0usize;
    for (sym, d) in known.iter().enumerate() {
        // The filter needs `n` half-samples of history, so the first few symbols
        // of a frame cannot be trained on — there is nothing before the sync.
        let centre = 2 * sym;
        if centre < n {
            continue;
        }
        for i in 0..n {
            let xi = rf[centre - i].conj();
            for j in 0..n {
                ata[i][j] = ata[i][j].add(xi.mul(rf[centre - j]));
            }
            atb[i] = atb[i].add(xi.mul(*d));
        }
        rows += 1;
    }
    if rows < n + 8 {
        return None;
    }
    // **Loading, for the reason the old inverse needed it.** The filter is longer
    // than the channel's own memory, so `AᴴA` is rank-deficient by construction
    // and the diagonal is what picks the minimum-energy answer out of the null
    // space. Relative to the matrix's own trace, so one number works at any level.
    let trace: f32 = (0..n).map(|i| ata[i][i].re).sum::<f32>() / n as f32;
    let load = C::new((trace * LS_LOAD).max(1e-12), 0.0);
    for i in 0..n {
        ata[i][i] = ata[i][i].add(load);
    }
    solve(ata, atb)
}

/// Diagonal loading for [`ls_equaliser`], as a fraction of the normal matrix's
/// mean diagonal.
///
/// **Not a tuning knob, measured.** Swept at 0.003 / 0.01 / 0.03 / 0.1 / 0.3 on
/// the one real coded tape: its data constellation moved between 0.721 and 0.729
/// across the whole range, so the answer is set by the filter length and not by
/// this. What it must not be is zero — the filter is longer than the channel's
/// memory, so the matrix is singular without it.
const LS_LOAD: f32 = 1e-2;

/// The centres of the three reference blocks, in symbols from the frame head.
const REF_CENTRES: [f32; 3] = [
    (REF_BLOCKS[0].0 + REF_BLOCKS[0].1 - 1) as f32 / 2.0,
    (REF_BLOCKS[1].0 + REF_BLOCKS[1].1 - 1) as f32 / 2.0,
    (REF_BLOCKS[2].0 + REF_BLOCKS[2].1 - 1) as f32 / 2.0,
];

impl ChannelFit {
    fn from_references(z: &[C; SCRAMBLED]) -> Self {
        let mut g = [C::default(); 3];
        for (i, &(lo, hi)) in REF_BLOCKS.iter().enumerate() {
            let mut acc = C::default();
            for k in lo..hi {
                acc = acc.add(z[k]);
            }
            // The mean, not the sum: the level of the reference is the level the
            // data has to be brought to.
            g[i] = acc.scale(1.0 / (hi - lo) as f32);
        }
        let mean_c = (REF_CENTRES[0] + REF_CENTRES[1] + REF_CENTRES[2]) / 3.0;
        let mean_g = g[0].add(g[1]).add(g[2]).scale(1.0 / 3.0);
        let mut num = C::default();
        let mut den = 0.0f32;
        for i in 0..3 {
            let dc = REF_CENTRES[i] - mean_c;
            num = num.add(g[i].sub(mean_g).scale(dc));
            den += dc * dc;
        }
        let slope = if den > 1e-6 { num.scale(1.0 / den) } else { C::default() };
        let intercept = mean_g.sub(slope.scale(mean_c));
        ChannelFit { slope, intercept }
    }

    /// The channel at symbol `k` of the frame (0 is the first data symbol).
    fn at(&self, k: f32) -> C {
        self.slope.scale(k).add(self.intercept)
    }

    /// One symbol, corrected to the reference's phase **and** level.
    fn apply(&self, k: usize, z: C) -> C {
        let g = self.at(k as f32);
        let n2 = g.norm().powi(2);
        if n2 > 1e-9 { z.mul(g.conj()).scale(1.0 / n2) } else { z }
    }
}

/// The three reference blocks' concentrations with a carrier offset removed
/// inside each block.
fn ramp_concentrations(z: &[C], hz: f32) -> [f32; 3] {
    let mut out = [0.0f32; 3];
    for (i, &(lo, hi)) in REF_BLOCKS.iter().enumerate() {
        let mut sum = C::default();
        let mut mag = 0.0f32;
        for k in lo..hi {
            let t = k as f32 / BAUD as f32;
            let c = z[k].mul(C::from_phase(-hz * TAU * t));
            sum = sum.add(c);
            mag += c.norm();
        }
        out[i] = if mag > 1e-12 { (sum.norm() / mag).min(1.0) } else { 0.0 };
    }
    out
}

/// Which rate a frame's three concentrations say it carries.
///
/// **A cascade, not a ratio between the leader and the runner.** For BPSK on the
/// π/4 lattice 0 and π are *both* lattice points, so a clean frame reads about 1
/// at the 2-, the 4- **and** the 8-fold — and a rule that asks the leader to beat
/// the runner by half again therefore answers **unknown** on the cleanest BPSK
/// signal this station has ever recorded: FUF 13031.2 kHz, 24 Oct, **0.933 /
/// 0.760 / 0.357**, where the ratio is 1.23. The noisier a BPSK frame was, the
/// more confidently it was called BPSK, which is backwards, and `Unknown`
/// suppresses the uncoded text path altogether.
///
/// The orders separate by **which concentration is the first to stand up**,
/// because a phase that steps by π/2 makes `e^(j2φ)` random and averages it away,
/// and one that steps by π/4 does the same to the 4-fold as well:
///
/// | | 2-fold | 4-fold | 8-fold |
/// |---|---|---|---|
/// | BPSK | stands up | stands up | stands up |
/// | QPSK | flat | stands up | stands up |
/// | 8PSK | flat | flat | stands up |
///
/// Under [`RESOLVED`] nothing is claimed: a rate with no evidence behind it is
/// how a wrong demodulation looks right.
fn rate_from_concentrations(constellation: [f32; 3]) -> S4285Rate {
    let [c2, c4, c8] = constellation;
    if c2 > RESOLVED {
        S4285Rate::Bps1200
    } else if c4 > RESOLVED {
        S4285Rate::Bps2400
    } else if c8 > RESOLVED {
        S4285Rate::Bps3600OrCoded
    } else {
        S4285Rate::Unknown
    }
}

/// A concentration below this is a lattice the symbols are not on.
const RESOLVED: f32 = 0.20;

/// Symbol phases to payload bits, oldest first, by Annex A's tables.
///
/// The symbols are descrambled and quantised to the nearest of eight phases, and
/// the table is chosen by the rate the constellation implied. At
/// [`S4285Rate::Bps3600OrCoded`] the 8PSK table is used, which is right for the
/// uncoded 3600 rate and is a best-effort tribit split for the coded ones — the
/// raw bits are exposed so that a coded frame is visible as wrong bits rather
/// than as silence.
fn transcode(data: &[C], rate: S4285Rate) -> Vec<u8> {
    let mut bits = Vec::with_capacity(data.len() * 3);
    for z in data {
        let idx = ((z.arg() / std::f32::consts::FRAC_PI_4).round() as i32).rem_euclid(8) as u8;
        match rate {
            S4285Rate::Bps1200 => bits.push(u8::from(idx != 0)),
            S4285Rate::Bps2400 => match idx {
                0 => bits.extend_from_slice(&[0, 0]),
                2 => bits.extend_from_slice(&[0, 1]),
                6 => bits.extend_from_slice(&[1, 0]),
                _ => bits.extend_from_slice(&[1, 1]),
            },
            _ => match idx {
                1 => bits.extend_from_slice(&[0, 0, 0]),
                0 => bits.extend_from_slice(&[0, 0, 1]),
                2 => bits.extend_from_slice(&[0, 1, 0]),
                3 => bits.extend_from_slice(&[0, 1, 1]),
                6 => bits.extend_from_slice(&[1, 0, 0]),
                7 => bits.extend_from_slice(&[1, 0, 1]),
                5 => bits.extend_from_slice(&[1, 1, 0]),
                _ => bits.extend_from_slice(&[1, 1, 1]),
            },
        }
    }
    bits
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both generators are the standard's, and both got wrong once: a register
    /// given one stage too many empties out and stays at zero, which made the
    /// sync sequence periodic with 16 and the scrambler a constant. Nothing
    /// failed loudly — the sync correlated *better* against the degenerate
    /// pattern and the references simply never collapsed, which read as "no frame
    /// in this file".
    #[test]
    fn the_generators_are_maximal_and_match_the_standard() {
        let sync = sync_symbols();
        // PN31: two periods plus the first 18, and the period really is 31.
        let bits: Vec<i8> = sync.iter().map(|v| if *v > 0.0 { 1 } else { 0 }).collect();
        assert_eq!(&bits[..31], &bits[31..62], "the sync is not periodic with 31");
        assert_ne!(&bits[..31], &bits[16..47], "the sync has a shorter period than 31");
        // The first ten symbols, from the register `11010` clocked as specified.
        assert_eq!(&bits[..10], &[0, 1, 0, 1, 1, 0, 0, 1, 1, 1], "sync head");

        let scr = scrambling();
        // A 511-length maximal sequence: over one frame's worth of symbols all
        // eight phases appear, and the sequence is not constant.
        let mut seen = [false; 8];
        for z in scr {
            let n = ((z.arg() / std::f32::consts::FRAC_PI_4).round() as i32).rem_euclid(8) as usize;
            seen[n] = true;
        }
        assert!(seen.iter().filter(|s| **s).count() >= 7, "the scrambler is not maximal");
        // **The register starts all ones**, which is the standard's convention and
        // what the independent s4285 implementation uses; the first three stages
        // are therefore 1,1,1, so the first symbol's phase index is 4+2+1 = 7. This
        // assertion used to expect 1, from a register that started with a single
        // bit set — the fault that left this receiver locking the sync of a real
        // transmission while descrambling its reference blocks and data into noise.
        assert_eq!(((scr[0].arg() / std::f32::consts::FRAC_PI_4).round() as i32) & 7, 7);
    }

    /// Is this scrambled offset a reference symbol?
    fn is_reference(k: usize) -> bool {
        REF_BLOCKS.iter().any(|&(lo, hi)| k >= lo && k < hi)
    }

    /// A frame-accurate 4285 BPSK transmitter, for the regression test only.
    ///
    /// Raised-cosine pulses with 30 % excess bandwidth — the standard's 3000 Hz
    /// occupied bandwidth at 2400 baud — so the loopback is a fair signal and not
    /// a rectangular-pulse spectrum the receiver's own filter would have to
    /// truncate. Frame `n` carries `bits[n*128 ..]`, so the payload is a known
    /// sequence rather than noise.
    fn modulate(bits: &[u8], frames: usize, rate: f64, ramp_hz_s: f64) -> Vec<f32> {
        let sync = sync_symbols();
        let scr = scrambling();
        // Symbol phases on a 5-sample grid, then pulse-shaped.
        let mut sym = Vec::with_capacity(frames * FRAME * 5);
        for f in 0..frames {
            let mut frame = [0.0f32; FRAME];
            for (i, s) in sync.iter().enumerate() {
                frame[i] = if *s > 0.0 { 0.0 } else { 4.0 };
            }
            let mut d = 0;
            for k in 0..SCRAMBLED {
                let sym_phase = if is_reference(k) {
                    0.0
                } else {
                    let b = bits[(f * 128 + d) % bits.len().max(1)];
                    d += 1;
                    if b == 1 { 4.0 } else { 0.0 }
                };
                frame[SYNC + k] =
                    (sym_phase + scr[k].arg() / std::f32::consts::FRAC_PI_4).rem_euclid(8.0);
            }
            for v in frame {
                sym.push(C::from_phase(v * std::f32::consts::FRAC_PI_4));
                for _ in 1..5 {
                    sym.push(C::default());
                }
            }
        }
        // Raised cosine, alpha 0.3, five symbols either side.
        let alpha = 0.3f32;
        let taps: Vec<f32> = (-20i32..=20)
            .map(|i| {
                let x = i as f32 / 5.0;
                let sinc = if x.abs() < 1e-6 {
                    1.0
                } else {
                    (std::f32::consts::PI * x).sin() / (std::f32::consts::PI * x)
                };
                let d = 1.0 - (2.0 * alpha * x).powi(2);
                let rc = if d.abs() < 1e-6 {
                    std::f32::consts::FRAC_PI_4 * (std::f32::consts::PI * x / 2.0).sin()
                } else {
                    (std::f32::consts::PI * alpha * x).cos() / d
                };
                sinc * rc
            })
            .collect();
        let mut bb = vec![C::default(); sym.len()];
        for (i, s) in sym.iter().enumerate() {
            if s.norm() < 1e-9 {
                continue;
            }
            for (j, t) in taps.iter().enumerate() {
                let k = i as isize + j as isize - 20;
                if k >= 0 && (k as usize) < bb.len() {
                    bb[k as usize] = bb[k as usize].add(s.scale(*t));
                }
            }
        }
        bb.iter()
            .enumerate()
            .map(|(i, z)| {
                // The phase is accumulated in f64 and reduced: `TAU * f * i` in
                // f32 loses its fractional part within a second and the carrier
                // lands on the wrong frequency entirely.
                let t = i as f64 / rate;
                // `ramp_hz_s` is a carrier *rate* error, in hertz per second: the
                // phase is quadratic in time, which is what a drifting station and
                // a free-running sound card both look like.
                let ph = (TAU as f64 * (1800.0 * t + 0.5 * ramp_hz_s * t * t))
                    .rem_euclid(TAU as f64) as f32;
                0.5 * z.mul(C::from_phase(ph)).re
            })
            .collect()
    }

    #[test]
    fn a_synthetic_frame_locks_and_its_references_collapse() {
        // A pseudo-random payload, so the data cannot pass for references. A
        // plain multiplicative hash has period 256 here and makes frame *n+1*
        // the exact complement of frame *n*, which cancels the sync in the frame
        // ACF — that fixture was the thing that was wrong, not the receiver.
        let mut seed = 0x2545_f491u32;
        let bits: Vec<u8> = (0..4096)
            .map(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                (seed & 1) as u8
            })
            .collect();
        // Noise, because a noiseless loopback hides the decoder's own faults: a
        // chunk's forced start state is corrected by the metric within a few
        // constraint lengths when the symbols are perfect, and only bites when
        // they are not. Ten decibels is what Annex E's own tables say a 600 bps
        // rate needs (BER 3.4e-4 at 10 dB in 3 kHz).
        let mut audio = modulate(&bits, 40, 12000.0, 0.0);
        let rms = (audio.iter().map(|v| v * v).sum::<f32>() / audio.len() as f32).sqrt();
        let noise = rms / 10f32.powf(10.0 / 20.0) * 1.4;
        let mut seed = 0x5EED_1234u32;
        for v in audio.iter_mut() {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            // A sum of four uniforms is close enough to Gaussian for a floor.
            let mut n = 0.0f32;
            for _ in 0..4 {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                n += (seed >> 8) as f32 / 16_777_216.0 - 0.5;
            }
            *v += noise * n;
        }
        let mut rx = S4285Receiver::new(12000.0, 1800.0);
        for chunk in audio.chunks(1024) {
            rx.process(chunk);
        }
        let r = rx.report();
        assert!(r.locked, "a clean synthetic frame did not lock: {:?}", r);
        assert!(r.frames > 8, "only {} frames demodulated", r.frames);
        for (i, c) in r.reference.iter().enumerate() {
            assert!(*c > 0.85, "reference block {i} read {c:.3}, and it should collapse");
        }
        assert!(r.frame_acf > 0.25, "frame ACF {:.3}", r.frame_acf);
    }

    /// The whole coded chain, end to end: SOM + payload + EOM, convolutional
    /// code, pair repetition, interleaver, BPSK modulation — through the receiver,
    /// the deinterleaver, the Viterbi decoder and the framer.
    ///
    /// This is the regression test for the plumbing. It cannot prove the
    /// interleaver's row structure against the standard (a loopback shares whatever
    /// convention both halves use); what proves that is a start-of-message pattern
    /// on real traffic, which is what the off-air captures are for.
    fn coded_round_trip(ramp_hz_s: f64) -> (Vec<u8>, Vec<S4285Message>) {
        coded_round_trip_band(ramp_hz_s, None)
    }

    /// The same, through an audio chain that only passes `band`.
    fn coded_round_trip_band(
        ramp_hz_s: f64,
        band: Option<(f64, f64)>,
    ) -> (Vec<u8>, Vec<S4285Message>) {
        coded_round_trip_path(ramp_hz_s, band, None)
    }

    /// The same, through a multipath channel as well.
    fn coded_round_trip_path(
        ramp_hz_s: f64,
        band: Option<(f64, f64)>,
        taps: Option<&[(f64, f32)]>,
    ) -> (Vec<u8>, Vec<S4285Message>) {
        coded_round_trip_path_mode(ramp_hz_s, band, taps, EqMode::Auto, 10.0)
    }

    /// The same, with the noise budget set explicitly.
    ///
    /// **Re-baselined with the segment interleaver.** The chain now spreads a fade
    /// over ten seconds instead of a third of one, which changes the error pattern a
    /// given impairment produces, and the two impairment tests sat exactly on the old
    /// chain's edge. They are re-based the honest way — the impairment's own cost
    /// comes *out* of the noise budget instead of being added on top, so the total
    /// stays what the standard's tables budget for: a third of the band missing is
    /// worth about 2 dB, and "two equal Rayleigh paths 2 ms apart cost roughly 5 dB"
    /// is the standard's own figure.
    fn coded_round_trip_noise(
        ramp_hz_s: f64,
        band: Option<(f64, f64)>,
        taps: Option<&[(f64, f32)]>,
        noise_db: f64,
    ) -> (Vec<u8>, Vec<S4285Message>) {
        coded_round_trip_path_mode(ramp_hz_s, band, taps, EqMode::Auto, noise_db)
    }

    /// The same, with the coded path's channel chosen rather than scored.
    #[allow(clippy::too_many_lines)]
    fn coded_round_trip_path_mode(
        ramp_hz_s: f64,
        band: Option<(f64, f64)>,
        taps: Option<&[(f64, f32)]>,
        mode: EqMode,
        noise_db: f64,
    ) -> (Vec<u8>, Vec<S4285Message>) {
        use super::fec::{self, CodedRate};
        let rate = CodedRate::Bps600;
        // A payload whose bits are not all the same, so a decoder that drops
        // them cannot pass by luck.
        // Long enough to cross several of the decoder's chunks: the overlap that
        // protects a chunk's opening is only exercised when a message spans more
        // than one, and a 128-bit payload fits inside a single 256-pair chunk.
        let payload: Vec<u8> = (0..600u32).map(|i| ((i * 5 + (i >> 2)) % 3 == 0) as u8).collect();
        let mut msg = vec![0u8; 6]; // six zero bits, so the SOM is not at the head
        for k in (0..32).rev() {
            msg.push(((fec::SOM >> k) & 1) as u8);
        }
        msg.extend_from_slice(&payload);
        for k in (0..32).rev() {
            msg.push(((fec::EOM >> k) & 1) as u8);
        }
        // Annex E's flush: zeros to the interleaver's delay plus 102.
        // **Derived from the depth, not typed in.** The annex's 6 246 covers the
        // short interleave; in the standard's 32-symbol segments the deepest row
        // holds 31·j segments = 11 904 symbols at j = 12.
        msg.extend(std::iter::repeat_n(0u8, 32 * 31 * fec::CodedRate::Bps600.j() + 102));

        let coded = fec::encode(&msg);
        let repeated = fec::repeat_pairs(&coded, rate.repeat());
        let mut il = fec::ConvInterleaver::interleaver(rate.j());
        let sent: Vec<u8> = repeated
            .iter()
            .map(|b| {
                let v = il.push(if *b == 1 { -1.0 } else { 1.0 });
                if v < 0.0 { 1 } else { 0 }
            })
            .collect();
        // **Two interleaver spans of margin.** The chain spans ten seconds at both
        // ends: the receiver's deinterleaver fills behind the transmitted lead-in,
        // and the message's own flush has to carry the rest out.
        let frames = sent.len() / 128 + 2 * (32 * 31 * fec::CodedRate::Bps600.j() / 128 + 2) + 32;
        let mut audio = modulate(&sent, frames, 12000.0, ramp_hz_s);
        // Noise, because a noiseless loopback hides the decoder's own faults: the
        // forced start state at the front of each chunk is pulled right by the
        // metric within a few constraint lengths when the symbols are perfect, and
        // only bites when they are not. Ten decibels is what Annex E's own tables
        // say 600 bps needs (3.4e-4 at 10 dB in 3 kHz).
        {
            let rms = (audio.iter().map(|v| v * v).sum::<f32>() / audio.len() as f32).sqrt();
            // The sum of four uniforms has σ = 1/√3, so scale to a real 10 dB.
            let amp = rms / 10f32.powf(noise_db as f32 / 20.0) / (4.0f32 / 12.0).sqrt();
            let mut seed = 0x5EED_1234u32;
            for v in audio.iter_mut() {
                let mut n = 0.0f32;
                for _ in 0..4 {
                    seed ^= seed << 13;
                    seed ^= seed >> 17;
                    seed ^= seed << 5;
                    n += (seed >> 8) as f32 / 16_777_216.0 - 0.5;
                }
                *v += amp * n;
            }
        }

        // The chain in front of the receiver: a rig's filter, a sound card, a
        // WebSDR's audio. Applied after the noise so a truncated band is the only
        // thing being measured.
        let audio = match band {
            Some((lo, hi)) => band_limit(&audio, 12000.0, lo, hi),
            None => audio,
        };
        // A real path arrives as several delayed copies of itself: the impairment
        // the standard's own tables budget for ("two equal Rayleigh paths 2 ms
        // apart cost roughly 5 dB") and the one a receiver answers with an
        // equaliser.
        let audio = match taps {
            Some(t) => multipath(&audio, t),
            None => audio,
        };
        let mut rx = S4285Receiver::new(12000.0, 1800.0);
        rx.set_eq_mode(mode);
        rx.trace_coded_stream(true);
        let mut got: Vec<S4285Message> = Vec::new();
        for chunk in audio.chunks(1024) {
            for ev in rx.process(chunk) {
                if let S4285Event::Message(m) = ev {
                    got.push(m);
                }
            }
        }
        if got.is_empty() {
            eprintln!(
                "no coded message at {ramp_hz_s} Hz/s, band {band:?}, {mode:?}: {:?}",
                rx.report()
            );
        }
        // **The chains are fed whole frames and nothing else**, erasures included:
        // 128 soft decisions per frame, in order. A stream with a hole in it
        // desynchronises the interleaver for good, so this is the shape the
        // recorded stream has to have before anything judges it.
        let traced = rx.coded_stream().len();
        assert!(traced > 0, "nothing traced into the coded stream");
        assert_eq!(traced % 128, 0, "the coded stream is not whole frames: {traced}");
        (payload, got)
    }

    /// A steep band-pass, to model the audio chain in front of a real capture.
    ///
    /// A rig's SSB filter, a sound card's anti-alias filter and a WebSDR's audio
    /// stream all truncate the 300–3300 Hz an STANAG 4285 occupies, and the
    /// question this measures is how much of that band the coded chain actually
    /// needs. Hann-windowed, 401 taps at 12 kHz: a transition of about 60 Hz, so
    /// a corner is where it says it is rather than a slope.
    fn band_limit(audio: &[f32], rate: f64, lo: f64, hi: f64) -> Vec<f32> {
        const TAPS: usize = 401;
        let mid = (TAPS - 1) as f64 / 2.0;
        let mut h = vec![0.0f32; TAPS];
        for (i, v) in h.iter_mut().enumerate() {
            let n = i as f64 - mid;
            let hann = 0.5 - 0.5 * (std::f64::consts::TAU * i as f64 / (TAPS - 1) as f64).cos();
            // An ideal low-pass of cutoff f is 2f/fs·sinc(2fn/fs); a band-pass is
            // the difference of two, and both have unity gain in their flat region.
            let lp = |f: f64| {
                let x = std::f64::consts::PI * 2.0 * f * n / rate;
                let s = if x.abs() < 1e-9 { 1.0 } else { x.sin() / x };
                2.0 * f / rate * s
            };
            *v = ((lp(hi) - lp(lo)) * hann) as f32;
        }
        let mut out = vec![0.0f32; audio.len()];
        for (k, o) in out.iter_mut().enumerate() {
            let start = k.saturating_sub(TAPS - 1);
            let mut acc = 0.0f32;
            for j in start..=k {
                acc += audio[j] * h[k - j];
            }
            *o = acc;
        }
        out
    }

    /// A multipath channel: the sum of `taps` scaled and delayed copies.
    ///
    /// `delay` is in **symbols**, at 2400 baud on 12 kHz audio, and a fractional
    /// delay is interpolated — crude, but the point is to place energy a symbol
    /// or two after the symbol that caused it, which is what a real path does and
    /// what a phase fit cannot undo.
    fn multipath(audio: &[f32], taps: &[(f64, f32)]) -> Vec<f32> {
        let mut out = vec![0.0f32; audio.len()];
        for &(delay, gain) in taps {
            let d = delay * (12000.0 / BAUD);
            let whole = d.floor() as usize;
            let frac = (d - whole as f64) as f32;
            for (k, v) in out.iter_mut().enumerate() {
                let a = k.checked_sub(whole).map(|i| audio[i]).unwrap_or(0.0);
                let b = k.checked_sub(whole + 1).map(|i| audio[i]).unwrap_or(0.0);
                *v += gain * (a * (1.0 - frac) + b * frac);
            }
        }
        out
    }

    /// The whole chain, over a channel that is exactly on frequency.
    #[test]
    fn a_coded_message_survives_the_whole_chain() {
        use super::fec::CodedRate;
        let (payload, got) = coded_round_trip(0.0);
        let m = &got[0];
        assert_eq!(m.rate, CodedRate::Bps600, "the wrong rate named the message");
        assert!(m.closed, "the EOM did not close the message");
        assert_eq!(m.bits, payload, "the message bits are not the ones that were sent");
    }

    /// **The coded chain reads the same message down all three channel paths.**
    ///
    /// The frame's own rule picks between the plain symbols and the equaliser by
    /// asking which one collapses the nine known reference symbols per block
    /// harder, and a real recording made it worth asking whether that is also the
    /// right question for the *data* — so [`EqMode`] exists and had to be proved
    /// not to break a signal known to be good. On the generated 600 bps
    /// transmission, all three positions decode the message's own bits.
    #[test]
    fn a_coded_message_survives_every_channel_path() {
        for mode in [EqMode::Auto, EqMode::Off, EqMode::On] {
            let (payload, got) = coded_round_trip_path_mode(0.0, None, None, mode, 10.0);
            assert!(!got.is_empty(), "no message with {mode:?}");
            assert_eq!(got[0].bits, payload, "the wrong bits with {mode:?}");
        }
    }

    /// The same chain on a channel that is **not** exactly on frequency.
    ///
    /// A 1 Hz/s ramp is well inside the 3.5 Hz/s the standard allows, and it is
    /// the case that used to break the receiver outright: the soft decisions
    /// handed to Annex E kept the frame's residual phase, so the bits going into
    /// the code arrived rotated by a different amount every frame while the
    /// constellation still read 0.99 and the references still collapsed at 0.998.
    /// Verified failing against the code before `feed_coded` was given the same
    /// channel line the frame itself is read through.
    #[test]
    fn a_coded_message_survives_a_carrier_ramp() {
        for ramp in [1.0f64, 4.0] {
            let (payload, got) = coded_round_trip(ramp);
            assert_eq!(got[0].bits, payload, "the wrong bits at {ramp} Hz/s");
        }
    }

    /// The closed-form equaliser, at the symbol level and with nothing else going
    /// on: train it on known symbols, run it over the rest, recover them.
    ///
    /// This is the piece the adaptive attempt failed at, and the shape the
    /// receiver uses: a **half-symbol-spaced** filter whose target is the symbol
    /// sent at each instant, so its output is aligned without a delay search. Two
    /// paths, the second a symbol and a half late at 0.67 — the same channel the
    /// audio test builds.
    #[test]
    fn the_least_squares_equaliser_undoes_a_symbol_level_echo() {
        let sync = sync_symbols();
        // The transmitted symbols: the sync, then eight-phase symbols on the π/4
        // lattice that a 4285 actually sends.
        let mut sent = [C::default(); FRAME];
        for k in 0..FRAME {
            let p = if k < SYNC {
                if sync[k] > 0.0 { 0.0 } else { std::f32::consts::PI }
            } else {
                ((k * 5) % 8) as f32 * std::f32::consts::FRAC_PI_4
            };
            sent[k] = C::from_phase(p);
        }
        // The same signal on the half-symbol grid the equaliser is fed: a symbol
        // at each even index, nothing between (zero-stuffed), which is what the
        // filter has to turn back into one symbol per symbol.
        let mut tx = vec![C::default(); 2 * FRAME];
        for (k, s) in sent.iter().enumerate() {
            tx[2 * k] = *s;
        }
        // Two paths: one symbol and a half of delay, which on this grid is three
        // samples, and nothing to interpolate.
        let rx: Vec<C> = (0..2 * FRAME)
            .map(|k| {
                let echo = if k >= 3 { tx[k - 3].scale(0.67) } else { C::default() };
                tx[k].add(echo)
            })
            .collect();

        let known: Vec<C> = sent[..SYNC].to_vec();
        let w = ls_equaliser(&rx, &known, LS_FRAC_TAPS).expect("an equaliser");
        let mut worst = 0.0f32;
        let mut clean = 0;
        for k in SYNC..FRAME {
            let centre = 2 * k;
            let mut y = C::default();
            for (i, wi) in w.iter().enumerate() {
                if let Some(v) = centre.checked_sub(i).and_then(|j| rx.get(j)) {
                    y = y.add(v.mul(*wi));
                }
            }
            let e = y.sub(sent[k]).norm();
            worst = worst.max(e);
            if e < 0.35 {
                clean += 1;
            }
        }
        println!("LS equaliser: {clean} of {} symbols, worst error {worst:.2}", FRAME - SYNC);
        assert!(
            clean > (FRAME - SYNC) * 9 / 10,
            "only {clean} of {} symbols recovered",
            FRAME - SYNC
        );
    }

    /// **The rate verdict has to be right on a clean BPSK frame, not only on a
    /// noisy one.**
    ///
    /// Every number here is off the air. The first is the cleanest BPSK frame this
    /// station has ever recorded — FUF 13031.2 kHz, 24 Oct, 2/4/8-fold **0.933 /
    /// 0.760 / 0.357** — and the old rule ("the leader beats the runner by half
    /// again") answered **unknown** on it, because on a clean BPSK frame the 4-fold
    /// stands up nearly as hard as the 2-fold. `Unknown` then suppressed the
    /// uncoded text path, so the best signal of the set was the one that produced
    /// nothing. The second is the same station a day earlier (0.728 / 0.387 /
    /// 0.189), which the ratio rule did call correctly — so the rule was at its
    /// best where the signal was at its worst.
    #[test]
    fn the_rate_verdict_is_right_on_the_cleanest_bpsk_frame() {
        assert_eq!(rate_from_concentrations([0.933, 0.760, 0.357]), S4285Rate::Bps1200);
        assert_eq!(rate_from_concentrations([0.728, 0.387, 0.189]), S4285Rate::Bps1200);
        // A clean frame of each of the other two orders.
        assert_eq!(rate_from_concentrations([0.02, 0.93, 0.86]), S4285Rate::Bps2400);
        assert_eq!(rate_from_concentrations([0.03, 0.04, 0.91]), S4285Rate::Bps3600OrCoded);
        // The corpus's noise floor: the 4232 kHz files, which are another mode.
        assert_eq!(rate_from_concentrations([0.063, 0.038, 0.185]), S4285Rate::Unknown);
    }

    /// The same, end to end: a synthetic 1200 bps BPSK transmission has to be
    /// reported as BPSK and not as a lattice it is not on.
    #[test]
    fn a_clean_synthetic_frame_is_reported_as_bpsk() {
        let mut seed = 0x2545_f491u32;
        let bits: Vec<u8> = (0..4096)
            .map(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                (seed & 1) as u8
            })
            .collect();
        let audio = modulate(&bits, 40, 12000.0, 0.0);
        let mut rx = S4285Receiver::new(12000.0, 1800.0);
        for chunk in audio.chunks(1024) {
            rx.process(chunk);
        }
        let r = rx.report();
        assert_eq!(r.rate, S4285Rate::Bps1200, "a clean BPSK synthetic was called {:?}", r.rate);
        assert!(
            r.constellation[1] > RESOLVED,
            "a clean BPSK frame's 4-fold reads {:.3} — this test exists because that \
             is the number the old ratio rule choked on",
            r.constellation[1]
        );
    }

    /// **The uncoded rates read their text, and that is a capability the mode did
    /// not have.**
    ///
    /// A 1200 bps BPSK transmission can be a plain teleprinter — one bit per data
    /// symbol, in order, with ITA2 5N1.5 characters on top. The receiver used to
    /// cut each frame's 128 bits into groups of five, which cannot read ITA2 at
    /// all: 5N1.5 puts **seven** bits on the wire per character, so the width is
    /// wrong and the alignment restarts every frame. Measured on this fixture, the
    /// old path emitted letters that meant nothing; with `fec::Ita2Framer` the text
    /// comes back exactly.
    ///
    /// A noiseless loopback, deliberately: the subject here is the bit and character
    /// layer, and noise would make a wrong reading indistinguishable from a right
    /// one.
    #[test]
    fn an_uncoded_transmission_reads_its_text() {
        // The text a French Navy test tape actually sends. No digits: ITA2 puts
        // figures behind a shift, and this fixture is about the framing.
        let text = "FAAA DE FUF ZNR UUUUU ZUI TESTING RYRYRYRYRYRYRYRY";
        let bits = fec::ita2_bits(text);
        assert!(bits.len() >= 256, "the fixture needs several frames: {}", bits.len());
        // Acquisition needs its own window before the frames start, so the fixture
        // is several repeats of the text rather than one pass.
        let audio = modulate(&bits, bits.len().div_ceil(128) + 20, 12000.0, 0.0);
        let mut rx = S4285Receiver::new(12000.0, 1800.0);
        let mut got = String::new();
        for chunk in audio.chunks(1024) {
            for ev in rx.process(chunk) {
                if let S4285Event::Text(t) = ev {
                    got.push_str(&t);
                }
            }
        }
        let r = rx.report();
        got.retain(|c| !c.is_whitespace());
        assert!(
            got.contains(&text.replace(' ', "")),
            "an uncoded 1200 bps teleprinter did not read back: {:?} (locked {}, {} frames, rate {:?})",
            got,
            r.locked,
            r.frames,
            r.rate
        );
    }

    /// The framer itself: an idle line is all ones, so the first zero after it is
    /// a start bit, and arbitrary junk resynchronises within a few characters.
    #[test]
    fn the_ita2_framer_syncs_on_an_idle_line_and_recovers_from_junk() {
        let bits = fec::ita2_bits("RYRY TEST");
        let want: Vec<char> = "RYRY TEST".chars().collect();

        // An idle line is mark (ones): the very first character must frame.
        let mut f = fec::Ita2Framer::new();
        for _ in 0..64 {
            assert!(f.push(1).is_none(), "idle ones framed a character");
        }
        let mut values = Vec::new();
        for b in &bits {
            if let Some(v) = f.push(*b) {
                values.push(v);
            }
        }
        assert_eq!(
            values.len(),
            want.len(),
            "idle line: {} of {} characters",
            values.len(),
            want.len()
        );
        let letters: Vec<char> =
            values.iter().map(|v| fec::ITA2_LETTERS[*v as usize & 31]).collect();
        assert_eq!(letters, want, "idle line: {:?}", letters);

        // Junk that is not a framed character: it may burst for a character or two
        // before the stop bit fails, but it has to synchronise and stay there.
        let mut f = fec::Ita2Framer::new();
        for b in [1u8, 1, 0, 0, 1, 0, 1, 0, 0, 1] {
            f.push(b);
        }
        let mut letters = String::new();
        for b in &bits {
            if let Some(v) = f.push(*b) {
                letters.push(fec::ITA2_LETTERS[v as usize & 31]);
            }
        }
        assert!(
            letters.contains("RYRY TEST"),
            "the framer did not recover from junk: {:?}",
            letters
        );
    }

    /// **The impairment the one real coded tape has: two paths a symbol apart,
    /// and the equaliser has to be what reads it.**
    ///
    /// The tape's data symbols could not be put on a lattice (2/4/8-fold
    /// **0.234 / 0.070 / 0.080**) while its sync and its sixteen-identical-symbol
    /// reference blocks were healthy, and its data *blocks* were no better than the
    /// frame — so the damage was inside 32 symbols, which is delay spread and not a
    /// phase drift. This reproduces it on a synthetic whose bits are known: the same
    /// coded transmission plus a copy of itself 1.5 symbols late at two thirds the
    /// amplitude.
    ///
    /// **The two measurements that make this a test rather than a pass.** At 10 dB
    /// the plain path reads **no message** and the equaliser reads it bit-exactly;
    /// both are asserted here, so a future change that lets the plain path through
    /// says so instead of quietly retiring the equaliser. Measured sweep, this
    /// channel, first level that decodes:
    ///
    /// | path | 15 dB | 10 dB | 8 dB | 6 dB | 4 dB | 2 dB |
    /// |---|---|---|---|---|---|---|
    /// | plain | reads | — | — | — | — | — |
    /// | equaliser | reads | reads | reads | reads | reads | — |
    ///
    /// **Why the filter is fractionally spaced.** A T-spaced estimate cannot
    /// describe a pulse: the raised cosine's own memory is in the received samples,
    /// so a five-tap T-spaced fit leaves the pulse in the residual and its inverse
    /// then fights it. Measured, that inverse made the known reference blocks
    /// collapse *worse* than the plain symbols (2.81 → 2.02), the frame's gate
    /// declined it, and the equaliser was never used on the audio it existed for.
    /// [`ls_equaliser`] is half-symbol spaced and its target is the symbol sent at
    /// each instant, so it absorbs the pulse, the echo and the sampling phase at
    /// once and its output is aligned without a delay search.
    ///
    /// **The tap count is the tape's, not the synthetic's.** Swept on both: 9 and 13
    /// taps read this 1.5-symbol channel but not the standard's 2 ms one; 25 taps
    /// read the 2 ms channel at 12 dB where 19 needs 15, and on the real tape 25
    /// taps cost the data constellation **0.790 → 0.727**. Nineteen is the choice
    /// because the tape is the material and 25's extra span is bought with a fit
    /// that a fading path does not hold.
    #[test]
    fn a_coded_message_survives_a_two_path_channel() {
        // -3.5 dB of echo 1.5 symbols late: a symbol is 0.417 ms, so this is
        // 0.63 ms of delay spread, the low end of what a 13 MHz path does.
        let taps = [(0.0, 1.0f32), (1.5, 0.67)];
        let (_, plain) =
            coded_round_trip_path_mode(0.0, Some((300.0, 3300.0)), Some(&taps), EqMode::Off, 10.0);
        assert!(
            plain.is_empty(),
            "the plain path now reads this channel at 10 dB, so this test no longer \
             shows the equaliser earning its place — re-measure the sweep above"
        );
        let (payload, got) = coded_round_trip_noise(0.0, Some((300.0, 3300.0)), Some(&taps), 10.0);
        assert!(!got.is_empty(), "the equaliser did not read a two-path message at 10 dB");
        assert_eq!(got[0].bits, payload, "the bits do not survive two paths");
    }

    /// **The standard's own worst case: two equal Rayleigh paths 2 ms apart.**
    ///
    /// Annex A's performance tables budget about 5 dB for this, and it is 4.8
    /// symbols of delay spread at 2400 baud — three times the tape's own echo. The
    /// plain receiver reads nothing at any level swept, up to 18 dB; the equaliser
    /// reads it at 15. This is the assertion that the filter models a real path
    /// rather than the one synthetic it was tuned on.
    #[test]
    fn a_coded_message_survives_the_two_equal_paths_the_standard_budgets_for() {
        let taps = [(0.0, 1.0f32), (4.8, 1.0)];
        let (_, plain) =
            coded_round_trip_path_mode(0.0, Some((300.0, 3300.0)), Some(&taps), EqMode::Off, 18.0);
        assert!(plain.is_empty(), "the plain path read two equal paths 2 ms apart");
        let (payload, got) = coded_round_trip_noise(0.0, Some((300.0, 3300.0)), Some(&taps), 15.0);
        assert!(!got.is_empty(), "two equal paths 2 ms apart lost the message");
        assert_eq!(got[0].bits, payload, "the bits do not survive two equal paths");
    }

    /// **How much of the 300–3300 Hz band does the coded chain need?**
    ///
    /// The question the one real coded capture we were sent turned on. Mark's FUF
    /// tape (13031.2 kHz, 23 Oct) arrives with its audio running about 800–3000 Hz
    /// where an STANAG 4285 occupies 300–3300, and every Annex E branch sits at
    /// its shuffled null on it — so either the missing third of the waveform is
    /// what kills the code, or the band is a coincidence and the fault is
    /// elsewhere.
    ///
    /// **It is not the band.** Measured on a synthetic whose bits are known, the
    /// chain decodes the message exactly through every one of these:
    ///
    /// | high-pass | low-pass | |
    /// |---|---|---|
    /// | 300, 500, 650, 800, 900, 1000 Hz | 3300 | decodes |
    /// | 300 | 3300, 3100, 3000, 2800, 2600 | decodes |
    /// | 800 | 3000 — his tape's own band | decodes |
    ///
    /// So a receiver whose audio chain is narrower than the standard's own
    /// 3000 Hz is not what stops a coded tape being read, and asking a reporter
    /// for a wider filter on that evidence would have been a wrong errand. The
    /// two corners are pinned here; the rest of the table is what the run that
    /// found this printed.
    #[test]
    fn the_coded_chain_survives_a_truncated_audio_band() {
        let reads = |band: (f64, f64), noise_db: f64| {
            let (payload, got) = coded_round_trip_noise(0.0, Some(band), None, noise_db);
            got.first().is_some_and(|m| m.bits == payload)
        };
        // **Measured with the segment interleaver, and the table changed with it.**
        // Swept at 10/14/18/22/26 dB before being written down, so these are numbers
        // rather than hopes:
        assert!(reads((300.0, 3300.0), 10.0), "the chain lost a message inside its own band");
        assert!(
            reads((300.0, 2600.0), 10.0),
            "the high corner moved: 2600 Hz low-pass used to decode"
        );
        // Mark's tape arrives 800-3000 Hz wide, and that costs about 4 dB: it reads at
        // 14 dB where the full band reads at 10, and it does not read at 10.
        assert!(
            reads((800.0, 3000.0), 14.0),
            "the tape's own band decoded at 14 dB after the change"
        );
        assert!(
            !reads((800.0, 3000.0), 10.0),
            "the tape's band now reads at 10 dB, where it was measured to need 14 — \
             the re-baseline above is out of date"
        );
        // **The low corner is the one that kills it.** A 1000 Hz high-pass does not
        // decode at any level swept, up to 26 dB: the bottom third of a 4285 is not
        // optional at 600 bps rate 1/2, and that is worth knowing before asking a
        // reporter for a wider filter or blaming a decoding fault on a narrow one.
        assert!(
            !reads((1000.0, 3300.0), 26.0),
            "a 1000 Hz high-pass decoded, which it never did in the sweep"
        );
    }

    /// The equaliser corrects **level** as well as phase, over the whole frame.
    ///
    /// This is what the first real coded capture we were sent needed: its audio
    /// filter tilts the signal across the 107 ms frame, so a straight phase line
    /// leaves the data smeared while the references still collapse (references
    /// 0.69 / 0.76 / 0.61, constellation 0.34). The fit is read from the three
    /// reference blocks and has to hold out to the **last** data block, which has
    /// no reference block after it — that is the extrapolated end, and it is the
    /// half a naively-interpolated fit gets wrong.
    #[test]
    fn the_channel_fit_recovers_level_and_phase_across_the_frame() {
        // A channel that is linear in I/Q — which is what the fit models — with a
        // level that falls from 1.22 to 0.86 and a phase that swings across the
        // frame. Written this way the fit is exact, so the test measures the
        // correction rather than the interpolation error.
        let g = |k: f32| {
            let t = k / 176.0;
            C::new(1.22 - 0.36 * t, 0.40 - 0.55 * t)
        };
        let mut z = [C::default(); SCRAMBLED];
        for k in 0..SCRAMBLED {
            // References are symbol 0, i.e. +1: the fit reads the channel from
            // them. Data carries a +1 BPSK symbol, so it must come back as +1.
            z[k] = g(k as f32);
        }
        let fit = ChannelFit::from_references(&z);
        for k in [0usize, 31, 48, 79, 96, 127, 144, 175] {
            let c = fit.apply(k, z[k]);
            assert!(
                (c.norm() - 1.0).abs() < 0.02,
                "symbol {k}: level {:.3}, not corrected",
                c.norm()
            );
            assert!(c.arg().abs() < 0.04, "symbol {k}: phase {:.3} rad, not corrected", c.arg());
        }
    }

    /// The negative that makes the lock a detector rather than a lamp: noise must
    /// not lock, and neither must a strong carrier of a different shape.
    #[test]
    fn noise_and_a_plain_carrier_do_not_lock() {
        let mut seed = 0x1234_5678u32;
        let mut noise = Vec::new();
        for _ in 0..12 * 12000 {
            seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12345);
            noise.push(((seed >> 16) as f32 / 32768.0 - 1.0) * 0.2);
        }
        let mut rx = S4285Receiver::new(12000.0, 1800.0);
        for chunk in noise.chunks(1024) {
            rx.process(chunk);
        }
        assert!(!rx.report().locked, "white noise locked: {:?}", rx.report());

        // A single steady tone at the carrier: a strong carrier is not a frame.
        let tone: Vec<f32> = (0..12 * 12000)
            .map(|i| 0.5 * (TAU as f32 * 1800.0 * i as f32 / 12000.0).cos())
            .collect();
        let mut rx = S4285Receiver::new(12000.0, 1800.0);
        for chunk in tone.chunks(1024) {
            rx.process(chunk);
        }
        assert!(!rx.report().locked, "a plain carrier locked: {:?}", rx.report());
    }

    /// The material this was built against: a real station off the air. Not in
    /// the tree — the capture is the operator's — so it runs only when pointed at
    /// one, exactly as the other off-air gates here do.
    ///
    ///     SDROXIDE_S4285_SAMPLE=~/s4285-captures/6348.000kHz-20261022-045545Z.wav \
    ///       cargo test -p sdroxide-dsp --release -- --ignored --nocapture an_off_air_capture
    #[test]
    #[ignore]
    fn an_off_air_capture_locks_and_its_references_collapse() {
        let path = std::env::var("SDROXIDE_S4285_SAMPLE").expect("set SDROXIDE_S4285_SAMPLE");
        let mut reader = hound::WavReader::open(&path).expect("open the wav");
        let spec = reader.spec();
        let raw: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
            hound::SampleFormat::Int => {
                reader.samples::<i16>().map(|s| s.unwrap() as f32 / 32_768.0).collect()
            }
        };
        // A capture can be stereo, and its samples are then interleaved: read as
        // mono, the rate is effectively halved and two channels are spliced into
        // one stream, so nothing locks and this gate measures nothing. The real
        // coded capture we were sent was stereo for exactly that reason.
        let ch = spec.channels.max(1) as usize;
        let audio: Vec<f32> = if ch == 1 {
            raw
        } else {
            raw.chunks(ch).map(|f| f.iter().sum::<f32>() / ch as f32).collect()
        };
        let mut rx = S4285Receiver::new(spec.sample_rate as f64, 1800.0);
        let mut refs = [0.0f64; 3];
        let mut n = 0.0f64;
        for chunk in audio.chunks(4096) {
            for ev in rx.process(chunk) {
                if let S4285Event::Frame(f) = ev {
                    for i in 0..3 {
                        refs[i] += f.reference[i] as f64;
                    }
                    n += 1.0;
                }
            }
        }
        let r = rx.report();
        println!("{:?}", r);
        assert!(n > 50.0, "only {n} frames demodulated");
        let mean = [refs[0] / n, refs[1] / n, refs[2] / n];
        println!("mean references {mean:?}");
        for (i, c) in mean.iter().enumerate() {
            assert!(*c > 0.55, "reference block {i} averaged {c:.3} over {n} frames");
        }
    }

    /// Dump the best chain's decoded bit stream, so its structure can be examined
    /// rather than argued about: an ITA2 tape has a start bit every seven, and a
    /// periodic decoder fault shows as a biased bit position.
    #[test]
    #[ignore]
    fn the_capture_bit_stream_dump() {
        let path = std::env::var("SDROXIDE_S4285_SAMPLE").expect("set SDROXIDE_S4285_SAMPLE");
        let mut reader = hound::WavReader::open(&path).expect("open the wav");
        let spec = reader.spec();
        let raw: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
            hound::SampleFormat::Int => {
                let scale = 1.0 / (1i64 << (spec.bits_per_sample - 1)) as f32;
                reader.samples::<i32>().map(|s| s.unwrap() as f32 * scale).collect()
            }
        };
        let ch = spec.channels.max(1) as usize;
        let audio: Vec<f32> = if ch == 1 {
            raw
        } else {
            raw.chunks(ch).map(|f| f.iter().sum::<f32>() / ch as f32).collect()
        };
        let mut rx = S4285Receiver::new(spec.sample_rate as f64, 1800.0);
        rx.trace_coded_stream(true);
        for chunk in audio.chunks(1024) {
            rx.process(chunk);
        }
        let soft = rx.coded_stream().to_vec();
        eprintln!("traced soft values {}", soft.len());
        // The 600 bps BPSK chain, rebuilt from the trace: deinterleave, then the
        // same chunked Viterbi the chain runs.
        let mut de = fec::ConvInterleaver::deinterleaver(fec::CodedRate::Bps600.j());
        let mut deint: Vec<f32> = Vec::with_capacity(soft.len());
        for v in &soft {
            deint.push(de.push(*v));
        }
        let need = (CHUNK_PAIRS + VITERBI_OVERLAP) * 2;
        let mut bits: Vec<u8> = Vec::new();
        let mut at = 0usize;
        while at + need <= deint.len() {
            let pairs = fec::average_pairs(&deint[at..at + need], 1);
            let d = fec::viterbi_decode(&pairs);
            bits.extend(d.iter().skip(VITERBI_OVERLAP).take(CHUNK_PAIRS));
            at += CHUNK_PAIRS * 2;
        }
        eprintln!("decoded bits {}", bits.len());
        // **The pair order**, which a loopback cannot judge: our own encoder
        // emits the two outputs of one bit in G1-then-G2 order, and if the air
        // sends them the other way round every decoded bit is wrong while the
        // metric stays flat. Decode both ways and look for the SOM.
        let best_match = |bits: &[u8]| -> (String, usize, i32) {
            let pat =
                |p: u32| -> Vec<u8> { (0..32).map(|k| ((p >> (31 - k)) & 1) as u8).collect() };
            let mut out = (String::new(), 0usize, i32::MIN);
            for (name, p) in [("SOM", fec::SOM), ("EOM", fec::EOM)] {
                let want = pat(p);
                for w in 0..bits.len().saturating_sub(32) {
                    let score: i32 =
                        (0..32).map(|k| if bits[w + k] == want[k] { 1 } else { -1 }).sum();
                    if score > out.2 {
                        out = (name.to_string(), w, score);
                    }
                }
            }
            out
        };
        let mut swapped: Vec<u8> = Vec::with_capacity(bits.len());
        {
            let mut at2 = 0usize;
            let mut d2: Vec<f32> = deint.clone();
            for pair in d2.chunks_exact_mut(2) {
                pair.swap(0, 1);
            }
            let mut out2: Vec<u8> = Vec::new();
            while at2 + need <= d2.len() {
                let pairs = fec::average_pairs(&d2[at2..at2 + need], 1);
                let d = fec::viterbi_decode(&pairs);
                out2.extend(d.iter().skip(VITERBI_OVERLAP).take(CHUNK_PAIRS));
                at2 += CHUNK_PAIRS * 2;
            }
            swapped = out2;
        }
        // **How the soft value is shaped**, which a loopback cannot judge either:
        // ours is the linear distance from the decision boundary, the reference
        // implementation's is an LLR quantised to 25..230 — nearly saturated, so
        // a weak symbol counts for little and a strong one counts for a lot but
        // not without limit. Try both extremes before blaming the geometry.
        let shaped = |f: &dyn Fn(f32) -> f32| -> Vec<u8> {
            let mut d: Vec<f32> = deint.iter().map(|v| f(*v)).collect();
            let mut out: Vec<u8> = Vec::new();
            let mut at = 0usize;
            while at + need <= d.len() {
                let pairs = fec::average_pairs(&d[at..at + need], 1);
                let dec = fec::viterbi_decode(&pairs);
                out.extend(dec.iter().skip(VITERBI_OVERLAP).take(CHUNK_PAIRS));
                at += CHUNK_PAIRS * 2;
            }
            d.clear();
            out
        };
        let hard = shaped(&|v: f32| if v >= 0.0 { 1.0 } else { -1.0 });
        let sat = shaped(&|v: f32| v.clamp(-1.0, 1.0).tanh() * 2.0 - v.signum() * 0.0);
        for (name, stream) in [("hard", &hard), ("tanh", &sat)] {
            let m = best_match(stream);
            eprintln!("value {name}: best {} {} of 32 at {}", m.0, m.2, m.1);
        }
        let a = best_match(&bits);
        let b2 = best_match(&swapped);
        eprintln!("pair order as-is: best {} {} of 32 at {}", a.0, a.2, a.1);
        eprintln!("pair order swapped: best {} {} of 32 at {}", b2.0, b2.2, b2.1);
        if b2.2 > a.2 {
            let text: String = swapped.iter().map(|b| if *b == 1 { '1' } else { '0' }).collect();
            std::fs::write("/tmp/opencode/s4285_bits_swapped.txt", &text).expect("write");
        }
        let text: String = bits.iter().map(|b| if *b == 1 { '1' } else { '0' }).collect();
        std::fs::write("/tmp/opencode/s4285_bits.txt", &text).expect("write");
        // Where the SOM and EOM patterns land, and how far off they are.
        let pat = |p: u32| -> Vec<u8> { (0..32).map(|k| ((p >> (31 - k)) & 1) as u8).collect() };
        for (name, p) in [("SOM", fec::SOM), ("EOM", fec::EOM)] {
            let mut best = (0usize, 0i32);
            let want = pat(p);
            for w in 0..bits.len().saturating_sub(32) {
                let score: i32 = (0..32).map(|k| if bits[w + k] == want[k] { 1 } else { -1 }).sum();
                if score > best.1 {
                    best = (w, score);
                }
            }
            eprintln!("{name}: best match {} of 32 at bit {}", best.1, best.0);
        }
    }

    /// **The capture, chain by chain**: does any branch fill its interleaver and
    /// run the Viterbi, or is the coded stream being reset before it can?
    ///
    /// This is the experiment that separates "the material is short" from "the
    /// soft decisions are wrong". A branch that never reaches a chunk has been
    /// starved; one that decodes chunks and never frames a message has been fed
    /// wrong values, or the framing is wrong.
    #[test]
    #[ignore]
    fn the_capture_chain_by_chain() {
        let path = std::env::var("SDROXIDE_S4285_SAMPLE").expect("set SDROXIDE_S4285_SAMPLE");
        let mut reader = hound::WavReader::open(&path).expect("open the wav");
        let spec = reader.spec();
        let raw: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
            hound::SampleFormat::Int => {
                let scale = 1.0 / (1i64 << (spec.bits_per_sample - 1)) as f32;
                reader.samples::<i32>().map(|s| s.unwrap() as f32 * scale).collect()
            }
        };
        // The capture is stereo; splicing it as mono halves the effective rate.
        let ch = spec.channels.max(1) as usize;
        let audio: Vec<f32> = if ch == 1 {
            raw
        } else {
            raw.chunks(ch).map(|f| f.iter().sum::<f32>() / ch as f32).collect()
        };
        eprintln!("capture: {} Hz, {ch} ch, {} samples", spec.sample_rate, audio.len());
        for mode in [EqMode::Lms, EqMode::Auto, EqMode::Dfe] {
            let mut rx = S4285Receiver::new(spec.sample_rate as f64, 1800.0);
            rx.set_eq_mode(mode);
            let mut msgs = 0usize;
            for chunk in audio.chunks(1024) {
                for ev in rx.process(chunk) {
                    if let S4285Event::Message(_) = ev {
                        msgs += 1;
                    }
                }
            }
            eprintln!(
                "== {mode:?}: frames {} msgs {msgs} coded {:?} som {}",
                rx.report().frames,
                rx.report().coded.map(|c| c.repeat()),
                rx.report().best_som
            );
            for c in rx.chain_stats() {
                eprintln!(
                    "   rate {:?} sign {:+.0} swap {} pushed {} chunks {} msgs {} som {} resets {}",
                    c.rate, c.sign, c.swap, c.pushed, c.chunks, c.messages, c.som, c.resets
                );
            }
        }
    }

    /// **Every coded rate has a branch, the QPSK one included.**
    ///
    /// The QPSK chain was created inside a loop over the four *BPSK* rates, so
    /// it never was: a QPSK capture — the rate the verdict names when the
    /// constellation is four-point — had no branch listening at all, and the
    /// panel showed "coded: none" no matter how long the capture. This pins the
    /// list rather than the loop.
    #[test]
    fn every_coded_rate_has_a_branch() {
        let chains = coded_chains();
        for rate in [
            fec::CodedRate::Bps1200,
            fec::CodedRate::Bps600,
            fec::CodedRate::Bps300,
            fec::CodedRate::Bps150,
            fec::CodedRate::Bps75,
        ] {
            let n = chains.iter().filter(|c| c.rate == rate).count();
            assert!(n >= 2, "{rate:?} has {n} branches — both signs are the floor");
        }
        // QPSK carries two bits, so it needs both orders as well: two signs times
        // two orders.
        let qpsk = chains.iter().filter(|c| c.rate == fec::CodedRate::Bps1200).count();
        assert_eq!(qpsk, 4, "the QPSK rate has {qpsk} branches, not four");
        // And nothing is duplicated or invented.
        assert_eq!(chains.len(), 12, "twelve branches: 4 BPSK rates x 2 signs + 4 QPSK");
    }

    /// **Our deinterleaver and Viterbi against the reference decoder's own soft
    /// decisions.**
    ///
    /// The reference implementation (`luwangg/signal-analysis`, driven by a
    /// small C++ driver in `/tmp/opencode/sa`) decodes this station's capture:
    /// its ITA2 reading is `RYRYRYRY…`. This takes *its* 128 soft decisions per
    /// frame, runs **our** deinterleaver and Viterbi over them, and compares the
    /// bits. If they agree, the two implementations' coding chain is the same and
    /// the fault is in the soft decisions this receiver makes — i.e. the front
    /// end, not the code.
    #[test]
    #[ignore]
    fn the_reference_soft_decisions_through_our_chain() {
        let soft_path = std::env::var("SDROXIDE_S4285_REF_SOFT")
            .unwrap_or_else(|_| "/tmp/opencode/ref_soft.bin".into());
        let bits_path = std::env::var("SDROXIDE_S4285_REF_BITS")
            .unwrap_or_else(|_| "/tmp/opencode/ref_bits_clean.txt".into());
        let raw = std::fs::read(&soft_path).expect("ref soft");
        let theirs = std::fs::read_to_string(&bits_path).expect("ref bits");
        let theirs: Vec<u8> =
            theirs.trim().bytes().map(|b| if b == b'1' { 1 } else { 0 }).collect();
        assert_eq!(raw.len() % 128, 0, "not whole frames");
        let frames = raw.len() / 128;
        eprintln!("reference: {frames} frames, {} bits", theirs.len());

        // Their values are 0..255 with 128 as the erasure and **high = one**;
        // ours are signed with positive = zero, so `128 - v` is the same number
        // the other way round. Only the sign and relative weight matter to the
        // metric.
        let mut de = fec::ConvInterleaver::deinterleaver(fec::CodedRate::Bps600.j());
        let mut deint: Vec<f32> = Vec::with_capacity(raw.len());
        for &v in &raw {
            deint.push(de.push(128.0 - v as f32));
        }
        let mut ours: Vec<u8> = Vec::new();
        for chunk in deint.chunks_exact(256) {
            let pairs = fec::average_pairs(chunk, 1);
            ours.extend(fec::viterbi_decode(&pairs));
        }
        eprintln!("ours: {} bits", ours.len());
        let text: String = ours.iter().map(|b| if *b == 1 { '1' } else { '0' }).collect();
        let _ = std::fs::write("/tmp/opencode/our_bits_on_ref_soft.txt", &text);
        let n = ours.len().min(theirs.len());
        let same = (0..n).filter(|&i| ours[i] == theirs[i]).count();
        eprintln!("agree on {same} of {n} bits ({:.1}%)", 100.0 * same as f64 / n as f64);
        // **Does our chain decode at all when the soft decisions are good?** Both
        // pair orders, and the text each produces: the reference's own bits read
        // `RYRY`, so this is the question of whether the fault is ours alone.
        let ry = |bits: &[u8]| -> usize {
            match fec::message_text(bits) {
                Some(t) => t.matches("RY").count(),
                None => 0,
            }
        };
        for swap in [false, true] {
            let mut de2 = fec::ConvInterleaver::deinterleaver(fec::CodedRate::Bps600.j());
            let mut d2: Vec<f32> = Vec::with_capacity(raw.len());
            for &v in &raw {
                d2.push(de2.push(128.0 - v as f32));
            }
            let mut ours2: Vec<u8> = Vec::new();
            for chunk in d2.chunks_exact(256) {
                let mut pairs = fec::average_pairs(chunk, 1);
                if swap {
                    for p in pairs.chunks_exact_mut(2) {
                        p.swap(0, 1);
                    }
                }
                ours2.extend(fec::viterbi_decode(&pairs));
            }
            eprintln!(
                "our chain on their soft, swap {swap}: {} bits, RY pairs {}, text {:?}",
                ours2.len(),
                ry(&ours2),
                fec::message_text(&ours2).map(|t| t.chars().take(60).collect::<String>())
            );
        }
        eprintln!(
            "reference's own bits: RY pairs {} text {:?}",
            ry(&theirs),
            fec::message_text(&theirs).map(|t| t.chars().take(80).collect::<String>())
        );
        // Also look for the reference's own text: ITA2 5N1.5 over their bits.
        if let Some(t) = fec::message_text(&theirs) {
            eprintln!("reference bits as text: {:?}", &t[..t.len().min(120)]);
        }
        if let Some(t) = fec::message_text(&ours) {
            eprintln!("our bits on their soft decisions: {:?}", &t[..t.len().min(120)]);
        }
        // **A measurement, not a gate.** Agreement here is 36.5 % raw and 66.5 %
        // at the best alignment, so the two chains are *not* the same machine:
        // the reference's metric is an XOR of saturated bytes (a hard decision in
        // disguise, `viterbi2.hpp`), ours is a soft correlation. What this probe
        // establishes is that the reference decodes this capture — its ITA2
        // reading is `RYRYRYRY…` — and ours does not, on the same soft
        // decisions. The next session starts from that.
    }

    /// **Sweep the equaliser against decoded text, not against the
    /// constellation.**
    ///
    /// The DFE's shape was chosen by how tightly the data constellation
    /// collapsed, which is not the same question as whether the bits decode — the
    /// reference implementation's 121-tap filter reads this capture and our
    /// 11-tap one does not, while reading *better* on the constellation. So this
    /// prints the text each shape produces and how much of the French Navy test
    /// tape pattern (`RYRY`) appears in it.
    #[test]
    #[ignore]
    fn the_capture_equaliser_swept_against_text() {
        let path = std::env::var("SDROXIDE_S4285_SAMPLE").expect("set SDROXIDE_S4285_SAMPLE");
        let mut reader = hound::WavReader::open(&path).expect("open the wav");
        let spec = reader.spec();
        let raw: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
            hound::SampleFormat::Int => {
                let scale = 1.0 / (1i64 << (spec.bits_per_sample - 1)) as f32;
                reader.samples::<i32>().map(|s| s.unwrap() as f32 * scale).collect()
            }
        };
        let ch = spec.channels.max(1) as usize;
        let audio: Vec<f32> = if ch == 1 {
            raw
        } else {
            raw.chunks(ch).map(|f| f.iter().sum::<f32>() / ch as f32).collect()
        };
        let rate = spec.sample_rate as f64;
        for (ff, fb, passes, mu) in [
            (11usize, 4usize, 2usize, 0.05f32),
            // **The reference implementation's own shape** (taps_data 12 at two
            // samples a symbol = 121 taps at five, four symbol taps, one pass,
            // mu 0.05): the setting that decodes this capture there.
            // **The step size, scaled to our unit-power inputs.** The reference
            // filters raw samples of amplitude about 0.5 with mu 0.05; ours are
            // normalised to unit power first, so the same mu is a step about ten
            // times too large and the filter rings instead of converging.
            (121, 4, 1, 0.002),
            (121, 4, 1, 0.005),
            (121, 4, 3, 0.002),
            (41, 4, 3, 0.002),
        ] {
            let mut rx = S4285Receiver::new(rate, 1800.0);
            rx.set_eq_mode(EqMode::Dfe);
            rx.set_dfe_params(ff, fb, passes, mu);
            let mut text = String::new();
            let mut msgs = 0usize;
            for chunk in audio.chunks(1024) {
                for ev in rx.process(chunk) {
                    match ev {
                        S4285Event::Text(t) => text.push_str(&t),
                        S4285Event::Message(_) => msgs += 1,
                        _ => {}
                    }
                }
            }
            let ry = text.matches("RY").count();
            eprintln!(
                "ff {ff:2} fb {fb:2} passes {passes} mu {mu:.3}: RY {ry:3} msgs {msgs} report {:?}",
                rx.report().coded
            );
            if ry > 0 {
                eprintln!("    text: {:?}", &text[..text.len().min(160)]);
            }
        }
    }

    /// Dump **our** soft decisions in the reference's own format (128 per frame,
    /// 0..255, high = one) so the C++ driver can run the two chains on the same
    /// numbers and say which half is at fault.
    #[test]
    #[ignore]
    fn dump_our_soft_decisions_for_the_reference() {
        let path = std::env::var("SDROXIDE_S4285_SAMPLE").expect("set SDROXIDE_S4285_SAMPLE");
        let mut reader = hound::WavReader::open(&path).expect("open the wav");
        let spec = reader.spec();
        let raw: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
            hound::SampleFormat::Int => {
                let scale = 1.0 / (1i64 << (spec.bits_per_sample - 1)) as f32;
                reader.samples::<i32>().map(|s| s.unwrap() as f32 * scale).collect()
            }
        };
        let ch = spec.channels.max(1) as usize;
        let audio: Vec<f32> = if ch == 1 {
            raw
        } else {
            raw.chunks(ch).map(|f| f.iter().sum::<f32>() / ch as f32).collect()
        };
        let mut rx = S4285Receiver::new(spec.sample_rate as f64, 1800.0);
        rx.trace_coded_stream(true);
        for chunk in audio.chunks(1024) {
            rx.process(chunk);
        }
        // **The trace is two values per data symbol** — the in-phase and the
        // quadrature part — because the QPSK chains are fed both. The BPSK chain
        // takes every second one, so that is what the reference's deinterleaver
        // must be given: one soft decision per data symbol, 128 a frame.
        let trace = rx.coded_stream();
        let soft: Vec<f32> = trace.iter().step_by(2).copied().collect();
        let mut bytes: Vec<u8> = Vec::with_capacity(soft.len());
        for &v in &soft {
            // Ours: positive is zero; theirs: 255 is one. Clamp to the same
            // saturation their LLR table has (25..230) so the comparison is of
            // decisions, not of scale.
            let p = (0.5 - 0.5 * v.clamp(-1.0, 1.0)) * 255.0;
            bytes.push(p.clamp(25.0, 230.0) as u8);
        }
        eprintln!("soft values {} ({} frames)", soft.len(), soft.len() / 128);
        let n = bytes.len() / 128 * 128;
        std::fs::write("/tmp/opencode/our_soft.bin", &bytes[..n]).expect("write");
    }

    /// **Which wiring of the deinterleaver reproduces the reference decoder's own
    /// bits?**
    ///
    /// The annex's wording does not say which of the interleaver's two commutators
    /// is the receiver's, so this tree carries four wirings and runs one of them
    /// ([`fec::Interleave::Announce`]). A loopback cannot tell them apart — all
    /// four round-trip through our own encoder — so this asks an independent
    /// decoder instead: feed its own soft decisions through each wiring and see
    /// which reproduces the bits it produced.
    #[test]
    #[ignore]
    fn which_interleaver_wiring_matches_the_reference() {
        let soft_path = std::env::var("SDROXIDE_S4285_REF_SOFT")
            .unwrap_or_else(|_| "/tmp/opencode/ref_soft.bin".into());
        let bits_path = std::env::var("SDROXIDE_S4285_REF_BITS")
            .unwrap_or_else(|_| "/tmp/opencode/ref_bits_clean.txt".into());
        let raw = std::fs::read(&soft_path).expect("ref soft");
        let theirs_txt = std::fs::read_to_string(&bits_path).expect("ref bits");
        let theirs: Vec<u8> =
            theirs_txt.trim().bytes().map(|b| if b == b'1' { 1 } else { 0 }).collect();
        let best_match = |ours: &[u8], theirs: &[u8]| -> (f64, usize) {
            let mut best = (0.0f64, 0usize);
            for off in 0..ours.len().saturating_sub(theirs.len()).min(40_000) {
                let hits = (0..theirs.len()).filter(|&i| ours[off + i] == theirs[i]).count();
                let frac = hits as f64 / theirs.len() as f64;
                if frac > best.0 {
                    best = (frac, off);
                }
            }
            best
        };
        for style in fec::Interleave::ALL {
            for swap in [false, true] {
                let mut de =
                    fec::ConvInterleaver::deinterleaver_with(fec::CodedRate::Bps600.j(), style);
                let mut deint: Vec<f32> = Vec::with_capacity(raw.len());
                for &v in &raw {
                    deint.push(de.push(128.0 - v as f32));
                }
                let mut ours: Vec<u8> = Vec::new();
                for chunk in deint.chunks_exact(256) {
                    let mut pairs = fec::average_pairs(chunk, 1);
                    if swap {
                        for p in pairs.chunks_exact_mut(2) {
                            p.swap(0, 1);
                        }
                    }
                    ours.extend(fec::viterbi_decode(&pairs));
                }
                let (frac, off) = best_match(&ours, &theirs);
                eprintln!(
                    "wiring {:12} swap {swap}: {:.1}% at offset {off}",
                    style.label(),
                    frac * 100.0
                );
            }
        }
    }

    /// **How much of the stream the chain actually decodes.** Pushing values with
    /// no message in them should still run the Viterbi once per 512 values; if it
    /// does not, most of the capture is being thrown away before the decoder sees
    /// it — which is what the counters on the real capture say.
    #[test]
    fn the_chain_decodes_every_chunk_it_is_fed() {
        let mut out = Vec::new();
        let mut chain = CodedChain::new(fec::CodedRate::Bps600, 1.0);
        let fed = 71_552usize;
        for i in 0..fed {
            chain.push(((i % 7) as f32) - 3.0, &mut out);
        }
        eprintln!("fed {fed} decoded {} chunks", chain.decoded);
        assert!(
            chain.decoded >= (fed / 512 - 2) as u64,
            "only {} chunks decoded from {fed} values — the rest is discarded",
            chain.decoded
        );
    }

    /// Our descrambled data symbols beside the reference decoder's, for the same
    /// audio: the front ends' difference, in numbers.
    #[test]
    #[ignore]
    fn our_symbols_beside_the_references() {
        let path = std::env::var("SDROXIDE_S4285_SAMPLE").expect("set SDROXIDE_S4285_SAMPLE");
        let mut reader = hound::WavReader::open(&path).expect("open the wav");
        let spec = reader.spec();
        let raw: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
            hound::SampleFormat::Int => {
                let scale = 1.0 / (1i64 << (spec.bits_per_sample - 1)) as f32;
                reader.samples::<i32>().map(|s| s.unwrap() as f32 * scale).collect()
            }
        };
        let ch = spec.channels.max(1) as usize;
        let audio: Vec<f32> = if ch == 1 {
            raw
        } else {
            raw.chunks(ch).map(|f| f.iter().sum::<f32>() / ch as f32).collect()
        };
        let mut rx = S4285Receiver::new(spec.sample_rate as f64, 1800.0);
        rx.set_eq_mode(EqMode::Lms);
        let mut shown = 0;
        for chunk in audio.chunks(1024) {
            for ev in rx.process(chunk) {
                if let S4285Event::Frame(f) = ev {
                    if shown < 2 {
                        let re: Vec<f32> = f
                            .symbols
                            .iter()
                            .take(24)
                            .map(|c| (c.re * 100.0).round() / 100.0)
                            .collect();
                        let im: Vec<f32> = f
                            .symbols
                            .iter()
                            .take(24)
                            .map(|c| (c.im * 100.0).round() / 100.0)
                            .collect();
                        let m: Vec<f32> = f.symbols.iter().map(|c| c.norm()).collect();
                        eprintln!(
                            "ours frame {shown}: |s| {:.3}",
                            m.iter().sum::<f32>() / m.len() as f32
                        );
                        eprintln!("   re {re:?}");
                        eprintln!("   im {im:?}");
                        shown += 1;
                    }
                }
            }
        }
    }
}
