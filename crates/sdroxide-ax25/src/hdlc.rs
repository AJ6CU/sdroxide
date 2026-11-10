//! Bit-level HDLC framing: flags, zero-bit stuffing, NRZI and the FCS.
//!
//! This is the layer rax25 never had to write. A KISS TNC does all of it in
//! firmware and hands the host bare frames, so a host-side AX.25 stack can start
//! at [`crate::packet`]. We own the modem, so we own this too.
//!
//! # The chain, and its order
//!
//! Transmit, outermost last:
//!
//! ```text
//! frame bytes ──▶ append FCS ──▶ wrap in 0x7E flags ──▶ stuff a 0 after
//!                                 five 1s (data only) ──▶ NRZI ──▶ modem
//! ```
//!
//! Receive is the exact inverse. The order matters and is not guessable:
//!
//! * **Stuffing then NRZI**, not the other way round. Stuffing exists to stop
//!   data ever containing the flag pattern `01111110`; it therefore has to
//!   happen while the bits still *are* the data. Direwolf's `hdlc_send.c` calls
//!   `send_bit_nrzi` from inside its stuffing loop, which is the same statement.
//! * **Flags are not stuffed.** The flag is the one place six consecutive 1s
//!   are allowed, and that is what makes it findable.
//! * **The FCS is inside the stuffing**, computed over the unstuffed frame.
//!
//! NRZI is "0 inverts the line, 1 leaves it alone" — so the receiver recovers
//! bits from *transitions*, and neither a modem that inverts the signal nor an
//! operator who has mark and space the wrong way round can break the decode.
//! That is the property being bought, and it is why NRZI is applied at this
//! layer and not left to the modem.
//!
//! The G3RUH scrambler is deliberately **not** here: it sits between this and
//! the 9600 baud modem only, and 300/1200 baud AFSK does without it. It belongs
//! with the modem that needs it.

use crate::fcs;

/// The HDLC flag: the one byte pattern the data can never contain.
pub const FLAG: u8 = 0x7E;

/// Shortest frame worth handing up: two addresses and a control byte. Anything
/// less cannot be an AX.25 frame whatever its FCS says.
pub const MIN_FRAME: usize = 15;

/// How many bits of a closing flag have already been collected by the time the
/// flag is recognised. Seven, not eight — the deframer spots the flag on its
/// last bit and returns before pushing it. See [`Deframer::push_bit`].
const FLAG_BITS_COLLECTED: usize = 7;

/// Longest frame accepted off the air, address through FCS.
///
/// AX.25 sets no hard limit; this is a sanity rail against a stuck receiver
/// growing a buffer without bound from noise. 2 kB is comfortably above the
/// largest real frame — paclen 256 plus eight digipeaters is under 300 bytes.
pub const MAX_FRAME: usize = 2048;

/// Bit-stuffing, NRZI and flag generation for the transmit side.
///
/// Fed whole frames, drained as bits. Holds no timing: how long a bit lasts is
/// the modem's business.
#[derive(Debug, Default)]
pub struct Framer {
    /// Bits ready to go, in transmission order.
    out: Vec<bool>,
    /// Consecutive 1s emitted in the data, for the stuffing rule.
    ones: u8,
    /// The NRZI line state.
    level: bool,
}

impl Framer {
    #[must_use]
    pub fn new() -> Self {
        Framer::default()
    }

    /// Queue `count` flags — the transmit delay, or the tail after a frame.
    ///
    /// The operator's TXDELAY is expressed in milliseconds and converted by the
    /// caller, which is the only place that knows the baud rate.
    pub fn push_flags(&mut self, count: usize) {
        for _ in 0..count {
            // A flag is sent LSB-first like everything else, and is *not*
            // stuffed: it is the only legal run of six 1s and that is what makes
            // it a delimiter.
            for bit in 0..8 {
                self.nrzi(FLAG >> bit & 1 != 0);
            }
        }
        // Six 1s have gone by, but they were a flag, not data. Leaving the
        // counter set would stuff a spurious 0 into the first data byte.
        self.ones = 0;
    }

    /// Queue one frame: FCS appended, then stuffed. Does **not** emit the
    /// surrounding flags — the caller decides how many, because between two
    /// queued frames a single shared flag is enough while the start of a
    /// transmission wants many.
    pub fn push_frame(&mut self, frame: &[u8]) {
        let f = fcs::fcs(frame);
        for &byte in frame.iter().chain(f.iter()) {
            for bit in 0..8 {
                self.data_bit(byte >> bit & 1 != 0);
            }
        }
    }

    /// One data bit: stuffed if it completes a run of five 1s, then NRZI.
    fn data_bit(&mut self, bit: bool) {
        self.nrzi(bit);
        if bit {
            self.ones += 1;
            if self.ones == 5 {
                // The stuffed 0 is transparent to the receiver and does not
                // itself count towards a run.
                self.nrzi(false);
                self.ones = 0;
            }
        } else {
            self.ones = 0;
        }
    }

    /// NRZI: a 0 inverts the line, a 1 leaves it where it is.
    fn nrzi(&mut self, bit: bool) {
        if !bit {
            self.level = !self.level;
        }
        self.out.push(self.level);
    }

    /// Take everything queued so far.
    pub fn take(&mut self) -> Vec<bool> {
        std::mem::take(&mut self.out)
    }

    /// Bits waiting to be taken.
    #[must_use]
    pub fn len(&self) -> usize {
        self.out.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.out.is_empty()
    }
}

/// Why a frame that looked complete was thrown away.
///
/// Kept rather than folded into a single "bad frame" count because on the air
/// they mean different things: a bad FCS is a collision or a fade, a runt is
/// usually a squelch tail, and an overlong one is a receiver that never saw a
/// closing flag.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Discard {
    /// The frame check sequence did not verify.
    BadFcs,
    /// Shorter than [`MIN_FRAME`].
    TooShort,
    /// Longer than [`MAX_FRAME`] with no closing flag in sight.
    TooLong,
}

/// Flag hunting, de-stuffing, NRZI decode and FCS check for the receive side.
///
/// Fed bits as the modem slices them, yields whole verified frames with the FCS
/// already stripped.
#[derive(Debug)]
pub struct Deframer {
    /// The previous NRZI line level, for transition decoding.
    last: bool,
    /// Have we ever seen a level? The first bit has no predecessor and must not
    /// be invented as a transition.
    primed: bool,
    /// The last eight decoded (pre-destuffing) bits, newest in the high
    /// position, for flag detection.
    window: u8,
    /// Consecutive 1s in the decoded stream.
    ones: u8,
    /// True once a flag has been seen, so bits are worth collecting.
    in_frame: bool,
    /// Bits of the frame under construction.
    bits: Vec<bool>,
    /// Frames discarded, by reason.
    discards: Vec<Discard>,
}

impl Default for Deframer {
    fn default() -> Self {
        Deframer {
            last: false,
            primed: false,
            window: 0,
            ones: 0,
            in_frame: false,
            bits: Vec::new(),
            discards: Vec::new(),
        }
    }
}

impl Deframer {
    #[must_use]
    pub fn new() -> Self {
        Deframer::default()
    }

    /// Take the discards recorded since the last call — for the monitor pane,
    /// where a rising bad-FCS count is how an operator sees a marginal path.
    pub fn take_discards(&mut self) -> Vec<Discard> {
        std::mem::take(&mut self.discards)
    }

    /// Feed one line level from the modem's slicer, appending any completed
    /// frame to `out`. Frames arrive with the FCS verified and removed.
    pub fn push_level(&mut self, level: bool, out: &mut Vec<Vec<u8>>) {
        // NRZI: no transition means a 1.
        let bit = if self.primed { level == self.last } else { true };
        self.last = level;
        // The very first level carries no information — there is nothing for it
        // to have transitioned from — so it is consumed to prime the decoder
        // rather than decoded. Treating it as a 1 would put a phantom bit at
        // the head of the stream and shift the first frame by one.
        if !self.primed {
            self.primed = true;
            return;
        }
        self.push_bit(bit, out);
    }

    /// Feed one already-NRZI-decoded bit. Used by tests and by any modem that
    /// recovers data bits directly.
    pub fn push_bit(&mut self, bit: bool, out: &mut Vec<Vec<u8>>) {
        self.window = (self.window >> 1) | if bit { 0x80 } else { 0 };

        if self.window == FLAG {
            // A flag closes whatever was open and opens the next.
            //
            // Exactly **seven** bits of this flag are already in `bits`, not
            // eight: the flag is only recognised on its last bit, and this
            // branch returns before that one is pushed. Taking eight off would
            // eat the frame's final data bit, and the only symptom would be
            // that nothing ever passes its FCS.
            //
            // Those seven are safe to count on. The flag's leading 0 could in
            // principle have been swallowed as a stuffed bit, but only if five
            // 1s had just gone by — and the sender's own stuffing guarantees at
            // most four in a row at the end of a frame.
            if self.in_frame {
                let n = self.bits.len().saturating_sub(FLAG_BITS_COLLECTED);
                self.bits.truncate(n);
                self.finish(out);
            }
            self.in_frame = true;
            self.bits.clear();
            self.ones = 0;
            return;
        }

        if !self.in_frame {
            return;
        }

        if bit {
            self.ones += 1;
            // Seven 1s in a row cannot happen in stuffed data and is not a
            // flag: the line is idle or the signal is gone. Abandon quietly —
            // this is the normal end of a transmission, not an error.
            if self.ones > 6 {
                self.in_frame = false;
                self.bits.clear();
                self.ones = 0;
                return;
            }
        } else {
            // A 0 after exactly five 1s was stuffed in by the sender and is not
            // data. Any other 0 is.
            let stuffed = self.ones == 5;
            self.ones = 0;
            if stuffed {
                return;
            }
        }
        self.bits.push(bit);
        if self.bits.len() > MAX_FRAME * 8 {
            self.discards.push(Discard::TooLong);
            self.in_frame = false;
            self.bits.clear();
        }
    }

    /// Judge the bits collected between two flags.
    fn finish(&mut self, out: &mut Vec<Vec<u8>>) {
        let bits = std::mem::take(&mut self.bits);
        // A partial byte means the sender's flag did not land on a byte
        // boundary, which means these were never a frame — noise that happened
        // to contain the flag pattern, most often.
        if bits.len() < MIN_FRAME * 8 || bits.len() % 8 != 0 {
            if !bits.is_empty() {
                self.discards.push(Discard::TooShort);
            }
            return;
        }
        let mut frame = Vec::with_capacity(bits.len() / 8);
        for byte in bits.chunks(8) {
            let mut b = 0u8;
            for (i, &bit) in byte.iter().enumerate() {
                if bit {
                    b |= 1 << i;
                }
            }
            frame.push(b);
        }
        if !fcs::check(&frame) {
            self.discards.push(Discard::BadFcs);
            return;
        }
        frame.truncate(frame.len() - 2);
        out.push(frame);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A frame with a run of 1s long enough to force stuffing, and a `0x7E`
    /// byte in the payload — the two things the framing exists to survive.
    fn awkward_frame() -> Vec<u8> {
        let mut f = Vec::new();
        f.extend_from_slice(&[b'A' << 1, b'P' << 1, b'R' << 1, b'S' << 1, b' ' << 1, b' ' << 1]);
        f.push(0b1110_0000);
        f.extend_from_slice(&[b'O' << 1, b'E' << 1, b'3' << 1, b'J' << 1, b'J' << 1, b'S' << 1]);
        f.push(0b0110_0001);
        f.push(0x03);
        f.push(0xF0);
        // 0xFF is eight 1s: two stuffed zeros. 0x7E is the flag pattern itself,
        // which must survive as data.
        f.extend_from_slice(&[0xFF, 0xFF, 0x7E, 0x00, 0x7E, 0xFF]);
        f
    }

    fn round_trip(frame: &[u8]) -> Vec<Vec<u8>> {
        let mut tx = Framer::new();
        tx.push_flags(4);
        tx.push_frame(frame);
        tx.push_flags(4);
        let bits = tx.take();

        let mut rx = Deframer::new();
        let mut out = Vec::new();
        for b in bits {
            rx.push_level(b, &mut out);
        }
        out
    }

    #[test]
    fn a_frame_survives_the_round_trip() {
        let frame = awkward_frame();
        assert_eq!(round_trip(&frame), vec![frame]);
    }

    /// The flag byte inside the payload must come back as payload, not split
    /// the frame in two. This is what stuffing is for and the one case that
    /// proves it is working.
    #[test]
    fn a_flag_byte_in_the_payload_is_not_a_flag() {
        let frame = awkward_frame();
        assert!(frame.contains(&FLAG), "the fixture must actually contain one");
        let got = round_trip(&frame);
        assert_eq!(got.len(), 1, "the payload flag split the frame");
        assert_eq!(got[0], frame);
    }

    /// Back-to-back frames sharing one flag between them, which is what a real
    /// sender does to avoid paying the preamble twice.
    #[test]
    fn two_frames_share_a_flag() {
        let a = awkward_frame();
        let mut b = awkward_frame();
        b.truncate(20);

        let mut tx = Framer::new();
        tx.push_flags(2);
        tx.push_frame(&a);
        tx.push_flags(1);
        tx.push_frame(&b);
        tx.push_flags(2);

        let mut rx = Deframer::new();
        let mut out = Vec::new();
        for bit in tx.take() {
            rx.push_level(bit, &mut out);
        }
        assert_eq!(out, vec![a, b]);
    }

    /// NRZI decodes from transitions, so a receiver that has the signal upside
    /// down — an inverting modem, a rig on the wrong sideband — must still
    /// decode. If this fails, NRZI is not doing the one job it is here for.
    #[test]
    fn an_inverted_signal_still_decodes() {
        let frame = awkward_frame();
        let mut tx = Framer::new();
        tx.push_flags(4);
        tx.push_frame(&frame);
        tx.push_flags(4);

        let mut rx = Deframer::new();
        let mut out = Vec::new();
        for bit in tx.take() {
            rx.push_level(!bit, &mut out);
        }
        assert_eq!(out, vec![frame]);
    }

    /// A corrupted frame must be dropped and counted, not handed up.
    #[test]
    fn a_bit_error_is_caught_and_counted() {
        let frame = awkward_frame();
        let mut tx = Framer::new();
        tx.push_flags(4);
        tx.push_frame(&frame);
        tx.push_flags(4);
        let mut bits = tx.take();

        // Flip a bit inside the frame, past the opening flags.
        let n = bits.len() / 2;
        bits[n] = !bits[n];

        let mut rx = Deframer::new();
        let mut out = Vec::new();
        for b in bits {
            rx.push_level(b, &mut out);
        }
        assert!(out.is_empty(), "a corrupted frame must not be handed up");
        assert!(
            rx.take_discards().contains(&Discard::BadFcs),
            "and the operator must be able to see that it happened"
        );
    }

    /// Noise before the first flag must not become a frame, and must not
    /// prevent the frame that follows from decoding.
    #[test]
    fn noise_ahead_of_the_first_flag_is_ignored() {
        let frame = awkward_frame();
        let mut tx = Framer::new();
        tx.push_frame(&frame);
        tx.push_flags(4);
        let real = tx.take();

        let mut rx = Deframer::new();
        let mut out = Vec::new();
        // A stretch of alternating levels: plenty of transitions, no flag.
        for i in 0..200 {
            rx.push_level(i % 3 == 0, &mut out);
        }
        // Then a proper opening flag and the frame.
        let mut opener = Framer::new();
        opener.push_flags(4);
        for b in opener.take() {
            rx.push_level(b, &mut out);
        }
        for b in real {
            rx.push_level(b, &mut out);
        }
        assert_eq!(out, vec![frame], "the frame after the noise must still arrive");
    }

    /// Seven or more 1s cannot occur in stuffed data. A receiver that kept
    /// collecting through an idle line would grow a buffer out of silence.
    #[test]
    fn an_idle_line_abandons_the_frame_in_progress() {
        let mut rx = Deframer::new();
        let mut out = Vec::new();
        // Open a frame, put a few bytes in, then hold the line idle.
        for bit in [false, true, true, true, true, true, true, false] {
            rx.push_bit(bit, &mut out);
        }
        for _ in 0..64 {
            rx.push_bit(true, &mut out);
        }
        assert!(out.is_empty());
        assert!(!rx.in_frame, "an idle line must close the frame in progress");
    }
}
