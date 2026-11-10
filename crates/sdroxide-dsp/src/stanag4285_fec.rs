//! STANAG 4285 Annex E — the coded rates: the convolutional code, the
//! interleaver and the message framing.
//!
//! Transcribed from Annex E of the standard (`docs/internal/STANAG4285-EDITION1.md`, "Annex E"),
//! and **none of it is cryptography**: a constraint-length-7 rate-1/2
//! convolutional code, a convolutional interleaver and a start/end-of-message
//! convention. The traffic's own encryption, where there is any, sits above this
//! layer and is neither described by the standard nor implemented here.
//!
//! The code is the ordinary NASA/CCSDS one — `G1 = 133`, `G2 = 171` octal, T1
//! first — and the lower rates are that code's output *pairs* repeated, never the
//! first bit repeated and then the second.
//!
//! ## What is certain, and what is not
//!
//! Every number below is read off the annex: the polynomials, the 32 rows, the
//! input commutator (`n·9 mod 32`), the 24-of-32 output commutator for 2400, the
//! SOM and EOM words, the flush counts and the decoder's structure.
//!
//! Two things the annex leaves open, and both are marked where they are used
//! because a reader has to know which way a wire was cut:
//!
//! * **which bit of each four is punctured** for the 2400 bps 8-PSK rate. The
//!   annex says "rate 1/2 punctured to rate 2/3" and nothing more, so that rate
//!   is not built here — a guess in a demapper is indistinguishable from a
//!   decode.
//! * **the character layer above the message.** Annex E carries a message
//!   protocol (SOM, EOM, flush) and calls what is between them "message bits";
//!   it does not say they are ITA2. A 75 bps link is exactly a 75 baud
//!   teleprinter, so ITA2 5N1.5 is what a test tape is expected to send, and
//!   both readings are offered rather than one assumed.

use std::collections::VecDeque;

/// Constraint length: seven stages, six of them memory.
pub const K: usize = 7;
/// Number of encoder states.
pub const STATES: usize = 1 << (K - 1);
/// `x⁶ + x⁴ + x³ + x + 1` — the T1 polynomial, 133 octal.
pub const G1: u32 = 0o133;
/// `x⁶ + x⁵ + x³ + x² + 1` — the T2 polynomial, 171 octal.
pub const G2: u32 = 0o171;
/// Start of message, `03873C3C`, leftmost bit sent first.
pub const SOM: u32 = 0x0387_3C3C;
/// End of message, `4B65A5B2`.
pub const EOM: u32 = 0x4B65_A5B2;
/// Rows in the convolutional interleaver, at every rate.
pub const ROWS: usize = 32;
/// The input commutator's row order: the normal sequence multiplied by 9 mod 32.
pub const IN_ORDER: [usize; ROWS] = [
    0, 9, 18, 27, 4, 13, 22, 31, 8, 17, 26, 3, 12, 21, 30, 7, 16, 25, 2, 11, 20, 29, 6, 15, 24, 1,
    10, 19, 28, 5, 14, 23,
];
/// The output commutator for the 2400 bps 8-PSK rate: 24 of the 32 rows, every
/// fourth one skipped.
pub const OUT_ORDER_2400: [usize; 24] =
    [0, 1, 2, 4, 5, 6, 8, 9, 10, 12, 13, 14, 16, 17, 18, 20, 21, 22, 24, 25, 26, 28, 29, 30];

/// A coded rate, as Annex E names it.
///
/// The four BPSK rates cannot be told apart by the constellation — all of them
/// are one bit per symbol at 1200 channel bits a second — so which one is on the
/// air comes out as the rate whose decode produces the start-of-message pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodedRate {
    /// 1200 bps data, QPSK, unmodified rate 1/2.
    Bps1200,
    /// 600 bps data, BPSK, unmodified rate 1/2.
    Bps600,
    /// 300 bps data, BPSK, rate 1/2 repeated twice.
    Bps300,
    /// 150 bps data, BPSK, rate 1/2 repeated four times.
    Bps150,
    /// 75 bps data, BPSK, rate 1/2 repeated eight times.
    Bps75,
}

impl CodedRate {
    /// The four a BPSK (1200 channel bit) transmission can be.
    pub const BPSK: [CodedRate; 4] =
        [CodedRate::Bps600, CodedRate::Bps300, CodedRate::Bps150, CodedRate::Bps75];

    pub fn label(self) -> &'static str {
        match self {
            CodedRate::Bps1200 => "1200 bps coded (QPSK, rate 1/2)",
            CodedRate::Bps600 => "600 bps coded (BPSK, rate 1/2)",
            CodedRate::Bps300 => "300 bps coded (BPSK, rate 1/4)",
            CodedRate::Bps150 => "150 bps coded (BPSK, rate 1/8)",
            CodedRate::Bps75 => "75 bps coded (BPSK, rate 1/16)",
        }
    }

    /// User data rate in bits per second.
    pub fn bps(self) -> f64 {
        match self {
            CodedRate::Bps1200 => 1200.0,
            CodedRate::Bps600 => 600.0,
            CodedRate::Bps300 => 300.0,
            CodedRate::Bps150 => 150.0,
            CodedRate::Bps75 => 75.0,
        }
    }

    /// How many times the rate-1/2 output pair is repeated.
    pub fn repeat(self) -> usize {
        match self {
            CodedRate::Bps1200 | CodedRate::Bps600 => 1,
            CodedRate::Bps300 => 2,
            CodedRate::Bps150 => 4,
            CodedRate::Bps75 => 8,
        }
    }

    /// The interleaver's delay increment per row, `j`.
    ///
    /// The annex's long-interleave column: 48 at 2400, 24 at 1200, and 12 for
    /// 600–75 — the last four put the same 1200 coded bits a second into the
    /// interleaver, so one `j` covers all of them.
    pub fn j(self) -> usize {
        match self {
            CodedRate::Bps1200 => 24,
            _ => 12,
        }
    }
}

/// Convolutional encoder, rate 1/2, register starting at zero.
pub fn encode(bits: &[u8]) -> Vec<u8> {
    let mut reg = 0u32;
    let mut out = Vec::with_capacity(bits.len() * 2);
    for &b in bits {
        reg = ((reg << 1) | (b as u32 & 1)) & 0x7F;
        out.push((reg & G1).count_ones() as u8 & 1);
        out.push((reg & G2).count_ones() as u8 & 1);
    }
    out
}

/// Repeat each *pair* of coded bits, as Annex E requires: T1,T2,T1,T2 — never
/// the first repeated and then the second.
pub fn repeat_pairs(bits: &[u8], times: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(bits.len() * times);
    for pair in bits.chunks(2) {
        for _ in 0..times {
            out.extend_from_slice(pair);
        }
    }
    out
}

/// Undo [`repeat_pairs`] on soft decisions: the even and the odd soft values are
/// **averaged separately** (Annex E, Appendix 1 §4), which is what turns four,
/// eight or sixteen soft decisions back into two.
pub fn average_pairs(soft: &[f32], times: usize) -> Vec<f32> {
    let mut out = Vec::with_capacity(soft.len() / times);
    for group in soft.chunks(times * 2) {
        if group.len() < times * 2 {
            break;
        }
        let (mut even, mut odd) = (0.0f32, 0.0f32);
        for k in 0..times {
            even += group[2 * k];
            odd += group[2 * k + 1];
        }
        out.push(even / times as f32);
        out.push(odd / times as f32);
    }
    out
}

/// The convolutional interleaver and its inverse.
///
/// Row `r` of the **interleaver** holds `r·j` entries; row `r` of the
/// **deinterleaver** holds `(I-1-r)·j`, because the annex gives it "j fewer bits
/// of storage than the preceding one above it". That is what makes the two a
/// pair: a bit spends `r·j` steps in one and `(I-1-r)·j` in the other, so the
/// total is a constant `(I-1)·j` and the message comes out in order.
///
/// The interleaver writes in the `n·9 mod 32` order and reads in the normal one;
/// the deinterleaver is the same machine with its two commutators exchanged,
/// which is what the annex says the receiver does ("soft decisions are entered
/// … in the normal commutator sequence and are taken out using the …
/// non-sequential commutator sequence").
///
/// The rows are pre-filled with zeros, which is both their "initialised to all
/// zeros" state and, for the deinterleaver, the erasures Appendix 1 asks for.
/// **Which way round the deinterleaver's two commutators are wired.**
///
/// Annex E describes a convolutional interleaver as a pair, and the annex's own
/// words ("entered … in the normal commutator sequence and are taken out using
/// the … non-sequential commutator sequence") do not say which side of the pair
/// is which in a receiver. Both directions round-trip perfectly through our own
/// encoder, so a loopback cannot choose between them — a real signal is the only
/// judge, which is what [`Interleave::ALL`] exists for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interleave {
    /// As transcribed: write in `IN_ORDER`, read sequentially.
    Announce,
    /// The two commutators exchanged.
    Swapped,
    /// The row lengths the other way round, `r·j` on the write side.
    RowsFlipped,
    /// Both of the above.
    SwappedFlipped,
}

impl Interleave {
    pub const ALL: [Interleave; 4] = [
        Interleave::Announce,
        Interleave::Swapped,
        Interleave::RowsFlipped,
        Interleave::SwappedFlipped,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Interleave::Announce => "announce",
            Interleave::Swapped => "swapped",
            Interleave::RowsFlipped => "rows-flipped",
            Interleave::SwappedFlipped => "swapped+flipped",
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConvInterleaver {
    rows: Vec<VecDeque<f32>>,
    j: usize,
    transmit: bool,
    out_order: Option<[usize; 24]>,
    /// Only the receiver's side is ever varied — see [`Interleave`].
    style: Interleave,
    /// Symbols collected towards the next **32-symbol data segment**, and the
    /// outputs of the last completed one. The standard's interleaver works in
    /// segments — one symbol into each row per segment, rows measured in segments
    /// — so a commutator advanced per symbol, as this was, gives the same shape
    /// with a delay 32 times shorter while round-tripping through its own encoder.
    block: Vec<f32>,
    pending: VecDeque<f32>,
}

/// The interleaver's unit: the standard's 32-symbol data segment.
const BLOCK: usize = ROWS;

impl ConvInterleaver {
    /// The transmitter's interleaver: row `r` holds `r·j`.
    pub fn interleaver(j: usize) -> Self {
        ConvInterleaver {
            rows: (0..ROWS).map(|r| VecDeque::from(vec![0.0; r * j])).collect(),
            j,
            transmit: true,
            out_order: None,
            style: Interleave::Announce,
            block: Vec::with_capacity(BLOCK),
            pending: VecDeque::with_capacity(BLOCK),
        }
    }

    /// The receiver's deinterleaver: row `r` holds `(I-1-r)·j`.
    pub fn deinterleaver(j: usize) -> Self {
        Self::deinterleaver_with(j, Interleave::Announce)
    }

    /// The same, with the commutators wired a chosen way — see [`Interleave`].
    pub fn deinterleaver_with(j: usize, style: Interleave) -> Self {
        let flipped = matches!(style, Interleave::RowsFlipped | Interleave::SwappedFlipped);
        ConvInterleaver {
            rows: (0..ROWS)
                .map(|r| {
                    let len = if flipped { r * j } else { (ROWS - 1 - r) * j };
                    VecDeque::from(vec![0.0; len])
                })
                .collect(),
            j,
            transmit: false,
            out_order: None,
            style,
            block: Vec::with_capacity(BLOCK),
            pending: VecDeque::with_capacity(BLOCK),
        }
    }

    /// The 2400 bps rate takes its output from 24 of the 32 rows.
    pub fn with_psk8(mut self, yes: bool) -> Self {
        self.out_order = yes.then_some(OUT_ORDER_2400);
        self
    }

    pub fn reset(&mut self) {
        for (r, row) in self.rows.iter_mut().enumerate() {
            let len = if self.transmit { r * self.j } else { (ROWS - 1 - r) * self.j };
            row.clear();
            row.resize(len, 0.0);
        }
        self.block.clear();
        self.pending.clear();
    }

    fn out_row(&self, step: usize) -> usize {
        match &self.out_order {
            Some(list) => list[step % list.len()],
            None => step % ROWS,
        }
    }

    /// One soft value in, one out, in **32-symbol data segments**.
    pub fn push(&mut self, v: f32) -> f32 {
        self.block.push(v);
        if self.block.len() == BLOCK {
            let swapped = matches!(self.style, Interleave::Swapped | Interleave::SwappedFlipped);
            // **Every row is written before any row is read.** The shortest row
            // holds nothing at all — row 31 on the receive side, row 0 on the
            // transmit side — so a write/read pair interleaved row by row reads that
            // position *before* this segment's value has been written into it, and
            // emits a zero. Measured against an independent implementation on the
            // same soft decisions, one symbol in every 32 was a zero where a real
            // value belonged, in exactly the position whose row is empty.
            for i in 0..BLOCK {
                let write = if self.transmit || swapped { IN_ORDER[i] } else { i };
                self.rows[write].push_back(self.block[i]);
            }
            for i in 0..BLOCK {
                let read = if self.transmit || swapped { self.out_row(i) } else { IN_ORDER[i] };
                self.pending.push_back(self.rows[read].pop_front().unwrap_or(0.0));
            }
            self.block.clear();
        }
        self.pending.pop_front().unwrap_or(0.0)
    }
}

/// Soft-decision Viterbi decoder for the rate-1/2 code.
///
/// Soft values are positive for a zero and negative for a one, which is Annex E's
/// own convention. The metric is the correlation with the expected pair, so a
/// larger path metric is better.
///
/// The traceback stores the **previous state** for every state at every step, not
/// the input bit: this code's state transition drops the oldest bit, so the
/// received state and the input bit do not identify the predecessor — two states
/// differ only in the bit that has just left the register, and remembering the
/// input alone picks the wrong one half the time, which is a decode that looks
/// plausible and is 30 % wrong.
pub fn viterbi_decode(soft: &[f32]) -> Vec<u8> {
    viterbi_decode_with_metric(soft).0
}

/// The decoder and the best path's metric, which is what says whether the stream
/// carries this code at all: a stream that does has a metric far above the same
/// values shuffled, because shuffling destroys the trellis and nothing else.
pub fn viterbi_decode_with_metric(soft: &[f32]) -> (Vec<u8>, f32) {
    if soft.len() < 2 {
        return (Vec::new(), 0.0);
    }
    let steps = soft.len() / 2;
    let mut expected = [[0.0f32; 2]; STATES * 2];
    let mut next = vec![[0usize; 2]; STATES];
    for s in 0..STATES {
        for b in 0..2 {
            let full = ((s as u32) << 1) | b as u32;
            let o1 = if (full & G1).count_ones() & 1 == 0 { 1.0 } else { -1.0 };
            let o2 = if (full & G2).count_ones() & 1 == 0 { 1.0 } else { -1.0 };
            expected[s * 2 + b] = [o1, o2];
            next[s][b] = (full & 0x3F) as usize;
        }
    }
    let mut metric = vec![f32::NEG_INFINITY; STATES];
    metric[0] = 0.0;
    let mut m2 = metric.clone();
    let mut from = vec![[0u8; STATES]; steps];
    for (t, pair) in soft.chunks_exact(2).enumerate() {
        let (y1, y2) = (pair[0], pair[1]);
        m2.fill(f32::NEG_INFINITY);
        for s in 0..STATES {
            let m = metric[s];
            if m == f32::NEG_INFINITY {
                continue;
            }
            for b in 0..2 {
                let o = &expected[s * 2 + b];
                let v = m + y1 * o[0] + y2 * o[1];
                let ns = next[s][b];
                if v > m2[ns] {
                    m2[ns] = v;
                    from[t][ns] = s as u8;
                }
            }
        }
        std::mem::swap(&mut metric, &mut m2);
    }
    let mut s = (0..STATES)
        .max_by(|a, b| metric[*a].partial_cmp(&metric[*b]).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or(0);
    let metric_best = metric[s];
    let mut out = vec![0u8; steps];
    for t in (0..steps).rev() {
        // The input bit is the low bit of the state we have arrived in.
        out[t] = (s & 1) as u8;
        s = from[t][s] as usize;
    }
    (out, metric_best)
}

/// What the message layer found.
#[derive(Debug, Clone, PartialEq)]
pub struct CodedMessage {
    /// Which coded rate produced it.
    pub rate: CodedRate,
    /// The message bits between the SOM and the EOM, oldest first.
    pub bits: Vec<u8>,
    /// True when an EOM was actually seen — otherwise the message is whatever
    /// had arrived when the receiver gave up waiting for it.
    pub closed: bool,
}

/// The message layer: finds the SOM at the decoder's output and collects until
/// the EOM, exactly as Annex E, Appendix 1 §5 describes.
#[derive(Debug, Clone)]
pub struct MessageFramer {
    rate: CodedRate,
    window: u32,
    filled: usize,
    bits: Vec<u8>,
    collecting: bool,
    closed: bool,
    /// Bits seen since the last SOM, for the "no SOM appeared" fallback.
    since: usize,
    seen: u64,
    /// The closest the decoder's output has come to the SOM, in matching bits of
    /// 32. A decoder that is working on real traffic gets close even when a fade
    /// stops it matching exactly, and one working on noise stays at 16 ± 3 —
    /// which is the difference between "not yet" and "not there".
    pub best_som: u8,
}

impl MessageFramer {
    pub fn new(rate: CodedRate) -> Self {
        MessageFramer {
            rate,
            window: 0,
            filled: 0,
            bits: Vec::new(),
            collecting: false,
            closed: false,
            since: 0,
            seen: 0,
            best_som: 0,
        }
    }

    pub fn reset(&mut self) {
        self.window = 0;
        self.filled = 0;
        self.bits.clear();
        self.collecting = false;
        self.closed = false;
        self.since = 0;
    }

    /// Feed one decoded bit; returns a message when one finishes.
    pub fn push(&mut self, bit: u8) -> Option<CodedMessage> {
        self.seen += 1;
        if self.filled >= 31 {
            let m = (self.window as u32).count_ones() as u8;
            // The window is the newest 32 bits with the newest at bit 0.
            // **Both bit orders, because ours was never validated.** The SOM and
            // EOM patterns are this crate's own addition — the independent s4285
            // implementation stops at the decoded bit stream and does no framing at
            // all — so if the annex sends the pattern least-significant bit first,
            // a correct trellis would never frame a message here and the failure
            // would look exactly like "no code". Both orders are accepted while
            // that question is open; whichever one frames a message settles it.
            let score = if self.filled >= 32 {
                let w = self.window;
                let against = |pat: u32| {
                    (0..32).filter(|k| ((w >> (31 - k)) & 1) == ((pat >> (31 - k)) & 1)).count()
                        as u8
                };
                against(SOM).max(against(SOM.reverse_bits()))
            } else {
                m.min(0)
            };
            if score > self.best_som {
                self.best_som = score;
            }
        }
        if self.collecting {
            self.bits.push(bit);
            self.window = (self.window << 1) | bit as u32;
            self.since += 1;
            if self.window == EOM || self.window == EOM.reverse_bits() {
                self.collecting = false;
                self.closed = true;
                // The EOM itself is not message data.
                self.bits.truncate(self.bits.len().saturating_sub(32));
                return Some(CodedMessage {
                    rate: self.rate,
                    bits: std::mem::take(&mut self.bits),
                    closed: true,
                });
            }
            // Give up on a message that never closes, rather than growing a
            // buffer for as long as the station transmits.
            if self.since > 8 * 1024 * 8 {
                self.collecting = false;
                return Some(CodedMessage {
                    rate: self.rate,
                    bits: std::mem::take(&mut self.bits),
                    closed: false,
                });
            }
        } else {
            self.window = (self.window << 1) | bit as u32;
            self.filled += 1;
            self.since += 1;
            // **The last 26 bits, not all 32.** Annex E's own decoding rule is "the
            // last 26 bits of the SOM are correlated against the decoder output
            // until they match; that is the start of the message" — and the SOM's
            // top six bits are zeros, so requiring them is six bits of evidence
            // that the standard does not ask for and that a marginal stream does
            // not always have. The position the message starts at is unchanged: the
            // 26 bits are the *last* 26 of the word, so they end where the 32 do.
            if self.filled >= 32 && som_match(self.window) {
                // Drop the SOM itself: a message starts after it.
                self.collecting = true;
                self.closed = false;
                self.since = 0;
                self.bits.clear();
            }
        }
        None
    }

    /// The bits between the SOM and the EOM, when one is complete.
    pub fn closed(&self) -> bool {
        self.closed
    }
}

/// ITA2 letters, in the order the five-bit code numbers them.
pub const ITA2_LETTERS: [char; 32] = [
    ' ', 'E', '\n', 'A', ' ', 'S', 'I', 'U', '\r', 'D', 'R', 'J', 'N', 'F', 'C', 'K', 'T', 'Z',
    'L', 'W', 'H', 'Y', 'P', 'Q', 'O', 'B', 'G', ' ', 'M', 'X', 'V', ' ',
];

/// The bits of ITA2 text in **5N1.5** framing: one start bit, five data bits least
/// significant first, then one and a half stop bits, which is what a 75 baud
/// teleprinter sends and what a test tape carries.
///
/// The stop bits are written as two ones: 1.5 bits of stop is what a mechanical
/// printer needs and two is what a receiver can always see, and a reader that
/// accepts "at least one" reads either.
pub fn ita2_bits(text: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(text.len() * 8);
    for ch in text.to_uppercase().chars() {
        let Some(v) = ITA2_LETTERS.iter().position(|c| *c == ch) else { continue };
        out.push(0); // start
        for i in 0..5 {
            out.push((v as u8 >> i) & 1);
        }
        out.push(1);
        out.push(1);
    }
    out
}

/// A **5N1.5 teleprinter framer** over one continuous bit stream.
///
/// The uncoded rates carry one bit per data symbol, in order, so a teleprinter
/// transmission is a single character stream across the whole transmission — not
/// five bits per frame. This hunts for a start bit (a zero), takes the five data
/// bits least significant first, and requires a stop bit (a one) before the next
/// start, which is what makes it self-synchronising.
///
/// It exists because the receiver used to chunk each frame's 128 bits into fives,
/// which can never read ITA2: 5N1.5 puts **seven** bits on the wire per character,
/// so five-bit groups are the wrong width *and* restart every frame. Measured on a
/// synthetic 1200 bps transmission of `RYRY TEST 12345`, the old path produced
/// letters that meant nothing and this one returns the text.
#[derive(Debug, Clone, Default)]
pub struct Ita2Framer {
    /// Bits held since the last character, oldest first.
    buf: VecDeque<u8>,
    /// Characters that passed the framing test.
    pub chars: u64,
}

impl Ita2Framer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reset(&mut self) {
        self.buf.clear();
    }

    /// Feed one bit; returns the five-bit ITA2 value when a framed character
    /// completes.
    pub fn push(&mut self, bit: u8) -> Option<u8> {
        self.buf.push_back(bit & 1);
        // Nothing before a start bit is a character.
        while self.buf.front() == Some(&1) {
            self.buf.pop_front();
        }
        // A start bit, five data bits and the first stop bit.
        while self.buf.len() >= 7 {
            if self.buf[6] == 1 {
                let mut v = 0u8;
                for i in 0..5 {
                    v |= self.buf[i + 1] << i;
                }
                // Drop start and data; the stop bit becomes the next leading one.
                for _ in 0..6 {
                    self.buf.pop_front();
                }
                self.chars += 1;
                return Some(v);
            }
            // A zero where a stop bit belongs: this is not the framing, so give up
            // one bit and look for the next start.
            self.buf.pop_front();
            while self.buf.front() == Some(&1) {
                self.buf.pop_front();
            }
        }
        None
    }
}

/// Does the last 26 received bits hold the SOM, in either bit order?
///
/// The window is the newest 32 bits with the newest at bit 0, so its **low 26
/// bits** are the last 26 received. Each candidate pattern is compared on the
/// same slice of the form the window would hold: for a least-significant-bit-first
/// transmission the window holds the pattern reversed, so the comparison is
/// against the reversed candidate's low 26 bits.
fn som_match(window: u32) -> bool {
    const TAIL: u32 = 0x03FF_FFFF;
    let tail = window & TAIL;
    [SOM, SOM.reverse_bits(), !SOM, (!SOM).reverse_bits()].iter().any(|c| tail == (c & TAIL))
}

/// A best reading of a message's bits as text.
///
/// Annex E calls what it carries "message bits" and does not name a character
/// set; a 75 bps link is exactly a 75 baud teleprinter, so ITA2 is what a test
/// tape is expected to send. Both framings are tried — synchronous five-bit
/// groups at every alignment, and 5N1.5 with its start and stop bits — and the
/// reading is kept only if it looks like text, because a wrong reading is
/// indistinguishable from a right one to a reader who cannot check it.
pub fn message_text(bits: &[u8]) -> Option<String> {
    if bits.len() < 40 {
        return None;
    }
    // **Two framings, and the synchronous one is preferred.**
    //
    // Annex E calls what the coded chain carries "message bits" and names no
    // character set; the coded rates put five data bits a character with no start
    // or stop, so synchronous groups are the reading the mode actually defines.
    // 5N1.5 (start bit, five data, stop) is what an *uncoded* teleprinter link
    // sends, and it is kept as a second attempt.
    //
    // The order matters, and it cost a good message to learn: this preferred the
    // 5N1.5 reading, and on the reference decoder's bits for the 24 October
    // capture — `RYRYRYRYRYRYRYRY`, the French Navy test tape — a wrong
    // start/stop alignment scored higher than the right synchronous one, so the
    // function returned `None` and a message that was there was thrown away.
    let mut framed: Option<(f32, String)> = None;
    for shift in 0..8 {
        let mut out = String::new();
        let mut p = shift;
        while p + 6 <= bits.len() {
            if bits[p] != 0 {
                p += 1;
                continue;
            }
            let mut v = 0u8;
            for i in 0..5 {
                v |= bits[p + 1 + i] << i;
            }
            out.push(ITA2_LETTERS[v as usize & 31]);
            p += 6;
            // At least one stop bit, then the next start bit: skip the ones.
            let mut stop = 0;
            while p < bits.len() && bits[p] == 1 && stop < 3 {
                p += 1;
                stop += 1;
            }
            if stop == 0 {
                // No stop bit where one is required: **resynchronise**, do not
                // give up. The reader is documented as self-synchronising and it
                // was not — it ended the reading at the first violation, so one
                // bad bit threw away everything after it. Measured on the
                // reference decoder's bits for the 24 October capture: a hand
                // reader that resynchronises reads `RYRYRYRYRYRYRYRY`, this one
                // read nothing at all.
                continue;
            }
        }
        let covered = out.chars().count() as f32 * 7.5 / bits.len() as f32;
        if covered < 0.5 {
            continue;
        }
        // The tape is evidence in itself and is not scaled by coverage: a
        // sixteen-character diddle inside a short reading is a station testing a
        // link, not a lucky slip. Prose still has to cover the stream.
        let score = (text_score(&out) * (0.5 + 0.5 * covered.min(1.0))).max(tape_score(&out));
        // Score first, then length: two alignments can score alike and the
        // longer reading is strictly more evidence — the shorter one has lost a
        // character at the front.
        // A tenth of a score is inside the noise of this measure; where two
        // alignments are that close, the longer reading is the one that did not
        // lose its first character.
        let better = framed
            .as_ref()
            .is_none_or(|(b, t)| score > *b + 0.1 || (score > *b - 0.1 && out.len() > t.len()));
        if better {
            framed = Some((score, out));
        }
    }

    let mut sync: Option<(f32, String)> = None;
    for shift in 0..5 {
        let n = (bits.len().saturating_sub(shift)) / 5;
        let mut out = String::with_capacity(n);
        for k in 0..n {
            let mut v = 0u8;
            for i in 0..5 {
                v |= bits[shift + 5 * k + i] << i;
            }
            out.push(ITA2_LETTERS[v as usize & 31]);
        }
        let covered = (out.chars().count() as f32 * 5.0 / bits.len() as f32).min(1.0);
        let score = (text_score(&out) * (0.5 + 0.5 * covered)).max(tape_score(&out));
        if sync.as_ref().is_none_or(|(b, _)| score > *b) {
            sync = Some((score, out));
        }
    }

    // The better reading wins, and a stream that only stands up under one of them
    // still reads: gating the fallback on "the first attempt found nothing" is
    // what threw a good message away.
    // **The framed reading wins when it stands up.** A coded message's payload is
    // a teleprinter stream — the reference decoder's bits for the 24 October
    // capture frame as 5N1.5 and read `RYRYRYRY…` — so a framed reading that
    // clears the gate is preferred even when the synchronous one scores a little
    // higher, which it does on a stream with start and stop bits in it because
    // those bits make a repeating pattern of their own.
    let chosen = match (sync, framed) {
        (Some(c), Some(f)) => Some(if f.0 >= 0.45 && f.0 >= c.0 * 0.8 { f } else { c }),
        (Some(c), None) => Some(c),
        (None, f) => f,
    };
    let (score, text) = chosen?;
    (score > 0.45).then_some(text)
}

/// How much a string looks like text.
///
/// ITA2 has only 32 codes and most of them are letters, so "it is all letters"
/// separates nothing: uniform noise over those codes is 37.5 % common letters, and
/// a wrong reading of a real message is 60 %. Two numbers do separate them — the
/// share of the twelve letters that dominate English and French, which is about
/// 70 % in text against 37.5 % in noise, and the share of spaces, about 15 %
/// against 3 % — and the score is those two against those references rather than
/// an absolute count, so a short message is judged on its own scale.
/// How much of the reading is a **repeating pattern**, 0..1.
///
/// A station testing a link sends a tape: `RYRYRY…`, `SGSGSG…`, or one of the
/// shift rehearsals, repeated for minutes. That is not English prose and this
/// mode does not carry English prose, so [`text_score`] — built on the common
/// letters of English — rejects exactly the transmission the mode is used for.
/// Measured on the reference decoder's bits for the 24 October capture: a hand
/// reader over synchronous groups reads `RYRYRYRYRYRYRYRY` and `text_score`
/// gave it 0.23, under the 0.45 gate, so a good message was thrown away.
///
/// So the reward is for structure that a wrong reading cannot fake: a short
/// pattern occupying most of the reading.
fn tape_score(s: &str) -> f32 {
    let c: Vec<char> = s.chars().collect();
    if c.len() < 24 {
        return 0.0;
    }
    let mut best = 0usize;
    for period in 2..=8usize {
        let mut run = 0usize;
        let mut longest = 0usize;
        for i in period..c.len() {
            if c[i] == c[i - period] {
                run += 1;
                longest = longest.max(run);
            } else {
                run = 0;
            }
        }
        best = best.max(longest + period);
    }
    // **Absolute, not a share of the reading.** A share was the first attempt and
    // it scored the very case it was written for at zero: `RYRYRYRYRYRYRYRY` is
    // sixteen characters inside a two-hundred-character reading, so a quarter of
    // the stream never repeats even though the tape is unmistakable. Eight
    // characters of a repeating pattern is chance; sixteen is a tape.
    ((best as f32 - 8.0) / 16.0).clamp(0.0, 1.0)
}

fn text_score(s: &str) -> f32 {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < 8 {
        return 0.0;
    }
    let n = chars.len() as f32;
    let common = chars
        .iter()
        .filter(|c| {
            matches!(c, 'E' | 'T' | 'A' | 'O' | 'I' | 'N' | 'S' | 'H' | 'R' | 'D' | 'U' | 'L')
        })
        .count() as f32;
    let spaces = chars.iter().filter(|c| **c == ' ').count() as f32;
    let (common_rate, space_rate) = (common / n, spaces / n);
    // Normalised: 0 at noise, 1 at text.
    let letters = ((common_rate - 0.375) / (0.70 - 0.375)).clamp(0.0, 1.0);
    let spaces = (1.0 - (space_rate - 0.15).abs() / 0.15).clamp(0.0, 1.0);
    0.6 * letters + 0.4 * spaces
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The encoder is the standard's: `G1 = 133`, `G2 = 171`, T1 first. Checked
    /// against the impulse response of the two polynomials and against a
    /// hand-computed sequence, because a swapped pair decodes to noise.
    #[test]
    fn the_encoder_is_the_standard_polynomials() {
        // One bit in, register `1`: both parities are 1 for G1 (bit 0 set) and
        // for G2 (bit 0 set).
        assert_eq!(encode(&[1]), vec![1, 1]);
        // Two bits: `11` → the second step's register is `11`, G1 has bits 0 and
        // 1 so its parity is even, G2 has only bit 0 of those two so its parity
        // is odd.
        assert_eq!(encode(&[1, 1]), vec![1, 1, 0, 1]);
        // The all-zero input encodes to all zeros, which is what the annex's
        // zeroed registers mean.
        assert_eq!(encode(&[0; 6]), vec![0; 12]);
        // A single one, then zeros, walks the impulse response of both taps.
        let imp = encode(&[1, 0, 0, 0, 0, 0, 0]);
        assert_eq!(&imp[..2], &[1, 1]);
        // x⁶ takes six steps to leave the register.
        assert_eq!(&imp[12..14], &[1, 1]);
    }

    /// Repeating pairs and averaging them back is the identity on soft values.
    #[test]
    fn repeated_pairs_come_back_through_the_averager() {
        for times in [2, 4, 8] {
            let coded = encode(&[1, 0, 1, 1, 0, 0, 1, 0]);
            let repeated = repeat_pairs(&coded, times);
            assert_eq!(repeated.len(), coded.len() * times);
            // Annex E repeats pairs, so the first four entries at 4× are the
            // first pair four times.
            let first = &repeated[..times * 2];
            for pair in first.chunks(2) {
                assert_eq!(pair, &coded[..2]);
            }
            let soft: Vec<f32> =
                repeated.iter().map(|b| if *b == 1 { -1.0 } else { 1.0 }).collect();
            let back = average_pairs(&soft, times);
            assert_eq!(back.len(), coded.len());
            for (i, v) in back.iter().enumerate() {
                assert_eq!(*v, if coded[i] == 1 { -1.0 } else { 1.0 }, "bit {i} at {times}x");
            }
        }
    }

    /// **The deinterleaver, against a model written out longhand.**
    ///
    /// Row `r` holds `(31-r)·j` segments; input element `i` of a segment goes into
    /// row `i` and output element `i` comes out of row `(9i) mod 32`. The model here
    /// is a plain FIFO per row with **every row written before any is read**, which
    /// is the standard's machine and not the same code.
    ///
    /// This exists because a loopback could not see the fault it pins: the shortest
    /// row holds *nothing at all*, so a write/read pair interleaved row by row reads
    /// that row before this segment's value has been written into it, and **one
    /// soft decision in every 32 came out as a zero**. Encoder and decoder carried
    /// the same quirk, so the pair still round-tripped and every test passed.
    /// Measured against an independent implementation on 130 689 real values: one
    /// mismatched position in 32 before the fix, none after.
    #[test]
    fn the_deinterleaver_matches_the_standard_machine_at_every_position() {
        let j = 2usize;
        let segments = 200usize;
        let input: Vec<f32> = (0..segments * ROWS).map(|t| t as f32).collect();
        let mut de = ConvInterleaver::deinterleaver(j);
        let got: Vec<f32> = input.iter().map(|v| de.push(*v)).collect();

        // The model: row r is a FIFO of (31-r)·j zeros, and a segment is written
        // to every row before anything is taken out.
        let mut rows: Vec<VecDeque<f32>> =
            (0..ROWS).map(|r| VecDeque::from(vec![0.0f32; (ROWS - 1 - r) * j])).collect();
        let mut want: Vec<f32> = Vec::new();
        for seg in input.chunks(ROWS) {
            for (i, v) in seg.iter().enumerate() {
                rows[i].push_back(*v);
            }
            for i in 0..ROWS {
                want.push(rows[(9 * i) & 31].pop_front().unwrap_or(0.0));
            }
        }
        // The machine's own latency: nothing comes out until the first segment has
        // been filled, so the model is one segment behind. The offset is **found**
        // rather than assumed, and then it has to be exact — a lag that had to be
        // fudged would mean the mapping itself was wrong.
        let mut best = (usize::MAX, 0usize);
        for lag in 0..2 * ROWS {
            let mism = (lag..got.len()).filter(|t| (want[t - lag] - got[*t]).abs() > 0.5).count();
            if mism < best.0 {
                best = (mism, lag);
            }
        }
        assert_eq!(best.0, 0, "{} positions disagree with the standard machine", best.0);
        assert_eq!(best.1, ROWS - 1, "unexpected latency {} for the segment buffer", best.1);
        // And the empty row is not empty *in the output*: its value belongs there.
        assert!(
            (1..segments).any(|n| got[n * ROWS + 7] != 0.0),
            "the shortest row's output position is still a hole"
        );
    }

    /// Encoder → interleaver → channel → deinterleaver → Viterbi returns the bits.
    ///
    /// The two commutators have to be exact inverses: a single row out of phase
    /// turns the whole message into noise, and the only symptom is that the SOM
    /// never appears.
    #[test]
    fn a_coded_message_survives_the_interleaver_and_the_decoder() {
        let bits: Vec<u8> = (0..2000u32).map(|i| ((i * 7 + (i >> 3)) % 3 == 0) as u8).collect();
        let coded = encode(&bits);
        // j = 1, so the interleaver's whole span is 31·32 = 992 coded bits and a
        // 2000-bit message runs well past it. At a real j the span is ten seconds
        // of signal and a unit test would be minutes of silence.
        let mut il = ConvInterleaver::interleaver(1);
        let sent: Vec<f32> = coded
            .iter()
            .map(|b| {
                let v = if *b == 1 { -1.0 } else { 1.0 };
                il.push(v)
            })
            .collect();
        let mut di = ConvInterleaver::deinterleaver(1);
        let soft: Vec<f32> = sent.iter().map(|v| di.push(*v)).collect();
        let back = viterbi_decode(&soft);
        // The interleaver delays, so compare the tail with the tail: the first
        // `(I-1)·j·I` bits of the decode are the registers emptying.
        let skip = (ROWS - 1) * ROWS + 1;
        assert!(back.len() > skip + 64, "block too short to compare");
        // **Where the message comes out is measured, not assumed.** Segment
        // buffering costs a segment on each side and the traceback adds its own, so
        // the offset is a property of the pipeline: slide the tail past the decode
        // and take the best match. A wrong alignment scores at chance.
        let n = 64;
        let want = &bits[bits.len() - n..];
        let mut best = 0usize;
        for off in 0..back.len().saturating_sub(n) {
            let hits = (0..n).filter(|k| back[off + k] == want[*k]).count();
            best = best.max(hits);
        }
        assert_eq!(best, n, "the interleaver and decoder did not return the message");
    }

    /// A teleprinter's message reads back as its text, in both framings.
    #[test]
    fn the_message_bits_read_as_ita2_text() {
        // "RYRY TEST RYRY" as ITA2 5N1.5 at 75 baud: one start bit, five data
        // bits least significant first, one and a half stop bits. Letters only:
        // the figures live behind a shift code and are not what a test tape's
        // diddle is made of.
        let text = "RYRY TEST RYRY";
        let mut bits = Vec::new();
        for c in text.chars() {
            let v = ITA2_LETTERS.iter().position(|x| *x == c).expect("in the table") as u8;
            bits.push(0u8);
            for i in 0..5 {
                bits.push((v >> i) & 1);
            }
            bits.push(1);
            bits.push(1);
        }
        let read = message_text(&bits).expect("the bits did not read as text");
        assert!(read.starts_with(text), "read {read:?} instead of {text:?}");
        // Noise is not text, which is the half that matters: a wrong reading that
        // looks like letters is worse than silence.
        let mut seed = 0x1234_5678u32;
        let noise: Vec<u8> = (0..800)
            .map(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                (seed & 1) as u8
            })
            .collect();
        assert_eq!(message_text(&noise), None, "noise read as text");
    }

    /// The metric-versus-null test that says whether a stream carries the code at
    /// all. A coded stream has to beat the same values shuffled; an uncoded one
    /// must not, and that is the half that matters — it is what says a real
    /// capture is uncoded rather than that the decoder is broken.
    #[test]
    fn the_decoder_metric_separates_a_coded_stream_from_a_shuffled_one() {
        let mut seed = 0xDEAD_BEEFu32;
        let bits: Vec<u8> = (0..4000)
            .map(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                (seed & 1) as u8
            })
            .collect();
        let coded = encode(&bits);
        let soft: Vec<f32> = coded.iter().map(|b| if *b == 1 { -1.0 } else { 1.0 }).collect();
        let (_, coded_metric) = viterbi_decode_with_metric(&soft);
        // The null: the same values shuffled, so the trellis is gone and nothing
        // else has changed.
        let mut shuffled = soft.clone();
        let mut s = 0x9E37_79B9u32;
        for i in (1..shuffled.len()).rev() {
            s ^= s << 13;
            s ^= s >> 17;
            s ^= s << 5;
            let j = (s as usize) % (i + 1);
            shuffled.swap(i, j);
        }
        let (_, null_metric) = viterbi_decode_with_metric(&shuffled);
        assert!(
            coded_metric > null_metric * 1.2,
            "the metric does not separate a coded stream ({coded_metric}) from its null ({null_metric})"
        );
    }

    /// The SOM is found at the decoder output and the EOM closes the message.
    #[test]
    fn the_framer_finds_the_start_and_the_end_of_a_message() {
        let msg: Vec<u8> = (0..200u32).map(|i| (i % 5 == 0) as u8).collect();
        let mut wire = Vec::new();
        for k in (0..32).rev() {
            wire.push(((SOM >> k) & 1) as u8);
        }
        wire.extend_from_slice(&msg);
        for k in (0..32).rev() {
            wire.push(((EOM >> k) & 1) as u8);
        }
        // A little leading noise, as a real decoder output has.
        let mut f = MessageFramer::new(CodedRate::Bps75);
        let mut got = None;
        for b in (0..100).map(|i| (i * 13 % 7 == 0) as u8).chain(wire) {
            if let Some(m) = f.push(b) {
                got = Some(m);
            }
        }
        let m = got.expect("no message framed");
        assert!(m.closed, "the EOM did not close the message");
        assert_eq!(m.bits, msg, "the message bits are not the ones between SOM and EOM");
    }

    /// **Annex E matches 26 bits of the SOM, not 32, and this crate required 32.**
    ///
    /// The SOM's top six bits are zeros, so demanding them is six bits of evidence
    /// the standard does not ask for — and on a marginal stream those are six ways
    /// to lose a message. Measured on the one real coded tape, the best SOM the
    /// decoder ever produced was **30 of 32**; whether the two wrong bits sat in
    /// those six decided whether a message framed at all.
    ///
    /// Here the first six bits of the SOM are wrong and the message must still
    /// frame. The old rule refuses this stream, which is what makes it a test.
    #[test]
    fn a_som_damaged_in_its_first_six_bits_still_frames() {
        let msg: Vec<u8> = (0..64u32).map(|i| (i % 3 == 0) as u8).collect();
        let mut wire = Vec::new();
        for (k, bit) in (0..32).rev().map(|k| ((SOM >> k) & 1) as u8).enumerate() {
            // The six most significant bits are the ones the annex does not match.
            wire.push(if k < 6 { bit ^ 1 } else { bit });
        }
        wire.extend_from_slice(&msg);
        for k in (0..32).rev() {
            wire.push(((EOM >> k) & 1) as u8);
        }
        let mut f = MessageFramer::new(CodedRate::Bps75);
        let mut got = None;
        for b in wire {
            if let Some(m) = f.push(b) {
                got = Some(m);
            }
        }
        let m = got.expect("a SOM damaged in its first six bits did not frame");
        assert_eq!(m.bits, msg, "the message bits are not the ones between SOM and EOM");
    }

    /// **A test tape reads, and it is not English prose.**
    ///
    /// The three faults this pins were each measured on the reference decoder's
    /// bits for this station's 24 October capture, where the answer is a French
    /// Navy tape: `RYRYRYRY…`, `SGSGSG…`, `ZNR UUUUU`, `ZUI TESTING`.
    ///
    /// * the score was built on the common letters of English and on spaces, so
    ///   `RYRYRYRYRYRYRYRY` scored 0.23 against a 0.45 gate — the mode's own test
    ///   transmission was rejected by its own text filter;
    /// * the reader gave up at the first missing stop bit instead of
    ///   resynchronising, throwing away everything after one bad bit;
    /// * the synchronous reading was gated on the framed one finding *nothing*,
    ///   so a wrong 5N1.5 alignment could block the right reading.
    #[test]
    fn a_test_tape_reads_as_text() {
        // 5N1.5, one start bit, five data bits least significant first, 1.5 stop.
        let tape = "ZUI TESTING RYRYRYRYRYRYRYRY ZNR UUUUU";
        let mut bits = Vec::new();
        for c in tape.chars() {
            let v = ITA2_LETTERS.iter().position(|x| *x == c).expect("in the table") as u8;
            bits.push(0u8);
            for i in 0..5 {
                bits.push((v >> i) & 1);
            }
            bits.push(1);
            bits.push(1);
        }
        let read = message_text(&bits).expect("the tape did not read as text");
        assert!(read.contains("ZUI TESTING"), "read {read:?}");
        assert!(read.contains("RYRYRYRY"), "the diddle was lost: {read:?}");
        // A single corrupted character must not end the reading: the reader
        // resynchronises rather than giving up.
        let mut damaged = bits.clone();
        damaged[70] = 1;
        damaged[71] = 1;
        let read = message_text(&damaged).expect("one bad bit ended the reading");
        assert!(read.contains("ZNR UUUUU"), "the tail was lost: {read:?}");
        // And noise is still not text.
        let mut seed = 0x1234_5678u32;
        let noise: Vec<u8> = (0..1200)
            .map(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 17;
                seed ^= seed << 5;
                (seed & 1) as u8
            })
            .collect();
        assert_eq!(message_text(&noise), None, "noise read as text");
    }
}
