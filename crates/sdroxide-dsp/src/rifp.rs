//! RIFP — Radio Image Framing Protocol, draft-dulaunoy-rifp-00.
//!
//! Two layers live here, and nothing above them:
//!
//! * the **frame**: a 28-octet base header, optional TLV extensions, a payload,
//!   and a CRC-32/ISO-HDLC trailer, built and parsed byte-exactly as the draft
//!   specifies (§4, §5, §9);
//! * the **modem** for the `rifp-cpfsk-4800` radio profile (§10): 4800 baud
//!   continuous-phase binary FSK, ±4000 Hz, MSB first, behind a 48-octet 0x55
//!   preamble and the 0xD391C5A7 synchronisation word.
//!
//! The modem's "audio" is not audio: [`RifpTx`] emits the ±1 NRZ symbol
//! waveform that [`crate::modulator::CpfskMod`] integrates into carrier phase,
//! and [`RifpRx`] consumes the discriminator output of
//! [`crate::demod::FskDemod`]. RIFP is not a sideband mode — the dial is the
//! centre of the signal, not its edge.
//!
//! Everything above a frame — the JSON manifest, the object codecs, session
//! reassembly — belongs to the caller (`sdroxide-digi::rifp_controller`).

use std::collections::VecDeque;

/// Protocol major version this implementation speaks (§4.1: a receiver MUST
/// discard a frame whose major version it does not support).
pub const VERSION_MAJOR: u8 = 1;
/// Protocol minor version this implementation emits.
pub const VERSION_MINOR: u8 = 0;

/// The fixed part of every header, in octets.
pub const BASE_HEADER_LEN: usize = 28;
/// Header Length is one octet, so the header plus all its TLVs cannot exceed
/// this.
pub const MAX_HEADER_LEN: usize = 255;
/// Payload Length is sixteen bits.
pub const MAX_PAYLOAD_LEN: usize = 65_535;

/// Frame types (§6).
pub const FRAME_MANIFEST: u8 = 0x01;
pub const FRAME_DATA: u8 = 0x02;
pub const FRAME_END: u8 = 0x03;
pub const FRAME_CANCEL: u8 = 0x04;

/// The payload of an END frame: encoded size, CRC-32, SHA-256 (§6.3).
pub const END_PAYLOAD_LEN: usize = 8 + 4 + 32;

/// Flag bit 0: this frame repeats information already sent (§4.2).
pub const FLAG_RETRANSMISSION: u32 = 1 << 0;
/// Bits 16..31 are critical: an unrecognised one means discard the frame.
pub const CRITICAL_FLAGS_MASK: u32 = 0xFFFF_0000;
/// The critical flags this implementation understands — none are defined yet.
pub const KNOWN_CRITICAL_FLAGS: u32 = 0;

/// TLV type bit 15 marks a critical extension (§5).
pub const TLV_CRITICAL: u16 = 0x8000;
pub const TLV_TYPE_MASK: u16 = 0x7FFF;
pub const TLV_SENDER_ID: u16 = 1;
pub const TLV_RADIO_PROFILE: u16 = 2;
pub const TLV_CONTENT_HINT: u16 = 3;

/// The radio profile's synchronisation word, sent MSB first ahead of the
/// header.
pub const SYNC_WORD: u32 = 0xD391_C5A7;
/// Preamble: 48 octets of 0x55 — 384 alternating bits for the receiver's
/// slicer and timing to settle on.
pub const PREAMBLE_OCTETS: usize = 48;

/// Symbols per second in `rifp-cpfsk-4800`.
pub const SYMBOL_RATE: f64 = 4800.0;
/// Peak deviation: mark is `+DEVIATION_HZ`, space is `-DEVIATION_HZ`.
pub const DEVIATION_HZ: f64 = 4000.0;

/// Longest payload a receiver will buffer from a frame it has not yet checked.
/// A sync word matched out of noise cannot make us allocate 64 KB.
const RX_MAX_PAYLOAD: usize = 32_768;

/// Sync-word bit errors tolerated when hunting. Two costs a false match roughly
/// once every twenty seconds across all timing phases, every one of which then
/// has to survive a header sanity check and a CRC — while it buys real
/// sensitivity on a noisy channel.
const SYNC_MAX_ERRORS: u32 = 2;

/// Silence between transmitted frames. Long enough for the modulator's envelope
/// to close and reopen cleanly (so each burst has its own ramped edges) and for
/// a receiver to see a frame boundary, short enough not to dominate a transfer.
pub const INTER_FRAME_GAP_S: f64 = 0.03;

/// CRC-32/ISO-HDLC — the "Ethernet"/ZIP CRC: polynomial 0x04C11DB7 reflected,
/// init and final XOR 0xFFFFFFFF, reflected in and out (§9). The check value
/// for `123456789` is 0xCBF43926.
pub fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in data {
        crc ^= byte as u32;
        for _ in 0..8 {
            // Branch-free: mask is all-ones when the low bit is set.
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}

/// One header extension (§5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tlv {
    pub type_id: u16,
    pub value: Vec<u8>,
}

impl Tlv {
    /// A non-critical UTF-8 text extension.
    pub fn text(base_type: u16, value: &str) -> Tlv {
        Tlv { type_id: base_type & TLV_TYPE_MASK, value: value.as_bytes().to_vec() }
    }

    pub fn critical(&self) -> bool {
        self.type_id & TLV_CRITICAL != 0
    }

    pub fn base_type(&self) -> u16 {
        self.type_id & TLV_TYPE_MASK
    }
}

/// Why a candidate frame was rejected. Every variant is a normal event on a
/// radio channel, not an exceptional one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameError {
    /// Fewer bytes than the frame's own lengths call for.
    Truncated,
    /// Major version we do not implement.
    Version(u8),
    /// Header Length outside 28..=255.
    HeaderLength(u8),
    /// The Reserved field was not zero (§4: discard).
    Reserved,
    /// A TLV ran off the end of the header.
    BadTlv,
    /// A critical flag or TLV we do not understand (§4.2, §5).
    UnknownCritical,
    /// The frame's CRC-32 did not match its contents.
    Crc,
}

/// A complete, CRC-validated RIFP frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RifpFrame {
    pub frame_type: u8,
    pub flags: u32,
    pub session_id: u64,
    pub sequence: u32,
    pub total: u32,
    pub tlvs: Vec<Tlv>,
    pub payload: Vec<u8>,
}

impl RifpFrame {
    /// A frame with no extensions and no flags.
    pub fn new(
        frame_type: u8,
        session_id: u64,
        sequence: u32,
        total: u32,
        payload: Vec<u8>,
    ) -> Self {
        RifpFrame { frame_type, flags: 0, session_id, sequence, total, tlvs: Vec::new(), payload }
    }

    /// The value of the first TLV with this base type, as text. Values that are
    /// not valid UTF-8, or that contain a NUL, are rejected (§5).
    pub fn text_tlv(&self, base_type: u16) -> Option<&str> {
        let tlv = self.tlvs.iter().find(|t| t.base_type() == base_type)?;
        let text = std::str::from_utf8(&tlv.value).ok()?;
        (!text.contains('\0')).then_some(text)
    }

    /// Serialise header + payload + CRC — the frame as the CRC covers it, with
    /// no preamble or sync word.
    pub fn encode(&self) -> Vec<u8> {
        let mut ext = Vec::new();
        for tlv in &self.tlvs {
            ext.extend_from_slice(&tlv.type_id.to_be_bytes());
            ext.extend_from_slice(&(tlv.value.len() as u16).to_be_bytes());
            ext.extend_from_slice(&tlv.value);
        }
        let header_len = BASE_HEADER_LEN + ext.len();
        debug_assert!(header_len <= MAX_HEADER_LEN, "RIFP header {header_len} > 255 octets");
        debug_assert!(self.payload.len() <= MAX_PAYLOAD_LEN);

        let mut out = Vec::with_capacity(header_len + self.payload.len() + 4);
        out.push(VERSION_MAJOR);
        out.push(VERSION_MINOR);
        out.push(self.frame_type);
        out.push(header_len as u8);
        out.extend_from_slice(&self.flags.to_be_bytes());
        out.extend_from_slice(&self.session_id.to_be_bytes());
        out.extend_from_slice(&self.sequence.to_be_bytes());
        out.extend_from_slice(&self.total.to_be_bytes());
        out.extend_from_slice(&(self.payload.len() as u16).to_be_bytes());
        out.extend_from_slice(&0u16.to_be_bytes()); // Reserved
        out.extend_from_slice(&ext);
        out.extend_from_slice(&self.payload);
        let crc = crc32(&out);
        out.extend_from_slice(&crc.to_be_bytes());
        out
    }

    /// The frame as it goes on the air: preamble, sync word, then [`encode`].
    ///
    /// [`encode`]: RifpFrame::encode
    pub fn air_frame(&self) -> Vec<u8> {
        let body = self.encode();
        let mut out = Vec::with_capacity(PREAMBLE_OCTETS + 4 + body.len());
        out.resize(PREAMBLE_OCTETS, 0x55);
        out.extend_from_slice(&SYNC_WORD.to_be_bytes());
        out.extend_from_slice(&body);
        out
    }

    /// Parse and validate a complete frame (header + payload + CRC). `bytes`
    /// must be exactly the frame; trailing data is an error, because the
    /// lengths in the header say how long the frame is.
    pub fn parse(bytes: &[u8]) -> Result<RifpFrame, FrameError> {
        let base = BaseHeader::parse(bytes)?;
        let want = base.frame_len();
        if bytes.len() != want {
            return Err(FrameError::Truncated);
        }
        let unknown_critical = base.flags & CRITICAL_FLAGS_MASK & !KNOWN_CRITICAL_FLAGS;
        if unknown_critical != 0 {
            return Err(FrameError::UnknownCritical);
        }

        let tlvs = parse_tlvs(&bytes[BASE_HEADER_LEN..base.header_len as usize])?;
        let payload_start = base.header_len as usize;
        let payload_end = payload_start + base.payload_len as usize;
        let received = u32::from_be_bytes([
            bytes[payload_end],
            bytes[payload_end + 1],
            bytes[payload_end + 2],
            bytes[payload_end + 3],
        ]);
        if received != crc32(&bytes[..payload_end]) {
            return Err(FrameError::Crc);
        }
        Ok(RifpFrame {
            frame_type: base.frame_type,
            flags: base.flags,
            session_id: base.session_id,
            sequence: base.sequence,
            total: base.total,
            tlvs,
            payload: bytes[payload_start..payload_end].to_vec(),
        })
    }
}

/// The fixed 28-octet header, parsed on its own so a streaming receiver can
/// learn how many more bits a frame needs before it has them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BaseHeader {
    pub major: u8,
    pub minor: u8,
    pub frame_type: u8,
    pub header_len: u8,
    pub flags: u32,
    pub session_id: u64,
    pub sequence: u32,
    pub total: u32,
    pub payload_len: u16,
}

impl BaseHeader {
    /// Total frame length in octets: header (with extensions), payload, CRC.
    pub fn frame_len(&self) -> usize {
        self.header_len as usize + self.payload_len as usize + 4
    }

    pub fn parse(bytes: &[u8]) -> Result<BaseHeader, FrameError> {
        if bytes.len() < BASE_HEADER_LEN {
            return Err(FrameError::Truncated);
        }
        let be32 = |o: usize| u32::from_be_bytes(bytes[o..o + 4].try_into().expect("4 bytes"));
        let header_len = bytes[3];
        if (header_len as usize) < BASE_HEADER_LEN {
            return Err(FrameError::HeaderLength(header_len));
        }
        // Reserved MUST be transmitted as zero; a receiver MUST discard a frame
        // in which it is not (§4).
        if bytes[26] != 0 || bytes[27] != 0 {
            return Err(FrameError::Reserved);
        }
        let major = bytes[0];
        if major != VERSION_MAJOR {
            return Err(FrameError::Version(major));
        }
        Ok(BaseHeader {
            major,
            minor: bytes[1],
            frame_type: bytes[2],
            header_len,
            flags: be32(4),
            session_id: u64::from_be_bytes(bytes[8..16].try_into().expect("8 bytes")),
            sequence: be32(16),
            total: be32(20),
            payload_len: u16::from_be_bytes([bytes[24], bytes[25]]),
        })
    }
}

fn parse_tlvs(mut data: &[u8]) -> Result<Vec<Tlv>, FrameError> {
    let mut out = Vec::new();
    while !data.is_empty() {
        if data.len() < 4 {
            return Err(FrameError::BadTlv);
        }
        let type_id = u16::from_be_bytes([data[0], data[1]]);
        let len = u16::from_be_bytes([data[2], data[3]]) as usize;
        if data.len() < 4 + len {
            return Err(FrameError::BadTlv);
        }
        let tlv = Tlv { type_id, value: data[4..4 + len].to_vec() };
        // An unknown critical TLV means the frame cannot be safely processed;
        // an unknown non-critical one is simply carried through (§5).
        if tlv.critical()
            && !matches!(tlv.base_type(), TLV_SENDER_ID | TLV_RADIO_PROFILE | TLV_CONTENT_HINT)
        {
            return Err(FrameError::UnknownCritical);
        }
        out.push(tlv);
        data = &data[4 + len..];
    }
    Ok(out)
}

// ───────────────────────────── transmitter ─────────────────────────────

/// Plays a queue of air frames out as the ±1 NRZ symbol waveform the CPFSK
/// modulator integrates. Silence (exactly 0.0) between frames closes the
/// modulator's envelope, so every frame is its own ramped burst.
pub struct RifpTx {
    /// Air frames (preamble + sync + frame), in transmission order.
    frames: Vec<Vec<u8>>,
    /// Samples per symbol — the profile's symbol rate against the audio rate.
    sps: usize,
    /// Silence between frames, in samples.
    gap: usize,

    frame: usize,
    /// Bit index within the current frame.
    bit: usize,
    /// Sample index within the current symbol.
    sample: usize,
    /// Samples of the trailing gap already emitted.
    gap_done: usize,

    total_samples: u64,
    sent_samples: u64,
    rate: f64,
}

impl RifpTx {
    /// `frames` are complete air frames from [`RifpFrame::air_frame`].
    pub fn new(frames: Vec<Vec<u8>>, rate: f64) -> Self {
        let sps = (rate / SYMBOL_RATE).round().max(1.0) as usize;
        let gap = (rate * INTER_FRAME_GAP_S).round().max(0.0) as usize;
        let total_samples = frames.iter().map(|f| (f.len() * 8 * sps + gap) as u64).sum::<u64>();
        RifpTx {
            frames,
            sps,
            gap,
            frame: 0,
            bit: 0,
            sample: 0,
            gap_done: 0,
            total_samples,
            sent_samples: 0,
            rate,
        }
    }

    /// Fraction of the transmission already played, 0.0..=1.0.
    pub fn progress(&self) -> f32 {
        if self.total_samples == 0 {
            return 1.0;
        }
        (self.sent_samples as f64 / self.total_samples as f64).clamp(0.0, 1.0) as f32
    }

    /// Seconds of transmission still to play.
    pub fn remaining_s(&self) -> f32 {
        let left = self.total_samples.saturating_sub(self.sent_samples);
        (left as f64 / self.rate) as f32
    }

    /// Total seconds the queued transmission takes.
    pub fn duration_s(&self) -> f32 {
        (self.total_samples as f64 / self.rate) as f32
    }

    /// Index of the frame being sent, and how many there are.
    pub fn frame_position(&self) -> (u32, u32) {
        (self.frame.min(self.frames.len()) as u32, self.frames.len() as u32)
    }

    pub fn done(&self) -> bool {
        self.frame >= self.frames.len()
    }

    /// Fill `out` with the next block of symbol waveform. Returns true once the
    /// last frame's trailing gap has been played out.
    pub fn next_block(&mut self, out: &mut [f32]) -> bool {
        for slot in out.iter_mut() {
            *slot = self.next_sample();
        }
        self.done()
    }

    fn next_sample(&mut self) -> f32 {
        let Some(frame) = self.frames.get(self.frame) else { return 0.0 };
        self.sent_samples += 1;

        let bits = frame.len() * 8;
        if self.bit < bits {
            // Most significant bit first, mark (1) high (§10).
            let byte = frame[self.bit / 8];
            let value = if byte >> (7 - (self.bit % 8)) & 1 != 0 { 1.0 } else { -1.0 };
            self.sample += 1;
            if self.sample >= self.sps {
                self.sample = 0;
                self.bit += 1;
            }
            return value;
        }

        self.gap_done += 1;
        if self.gap_done >= self.gap {
            self.frame += 1;
            self.bit = 0;
            self.sample = 0;
            self.gap_done = 0;
        }
        0.0
    }
}

// ────────────────────────────── receiver ───────────────────────────────

/// Streaming CPFSK receiver: discriminator output in, validated frames out.
///
/// Timing recovery is brute force, exactly as the reference implementation
/// does it offline: one independent bit slicer and frame assembler per symbol
/// phase. At ten samples per symbol that is ten cheap state machines, and it
/// costs nothing to be right about timing on a burst that arrives without
/// warning. Frames found by more than one phase are identical, and the caller
/// de-duplicates by session, type, sequence and content.
pub struct RifpRx {
    sps: usize,
    /// Ring of the last `sps` samples and their running sum — the matched
    /// filter, evaluated once per sample and handed to the phase it completes.
    ring: VecDeque<f32>,
    sum: f32,
    /// Sample counter, for choosing which phase a completed window belongs to.
    n: u64,
    /// Hunting slicer level: a leaky mean of the matched filter, which over the
    /// preamble's perfectly balanced 0x55 pattern *is* the residual frequency
    /// offset. Frozen into a frame when its sync word matches — see
    /// [`PhaseRx::push_symbol`].
    dc: f32,
    dc_alpha: f32,
    /// Smoothed |symbol| and |sample|, whose ratio is the lock indicator.
    sym_mag: f32,
    raw_mag: f32,
    phases: Vec<PhaseRx>,
    /// Sample index of a CRC failure not yet counted, held in case another
    /// timing phase is about to decode the same transmitted frame cleanly.
    pending_bad: Option<u64>,
    /// Sample index of the last valid frame, so a failure just *after* one is
    /// recognised as the same frame seen by a different phase.
    last_good: Option<u64>,
    /// Frames whose CRC failed and that no timing phase recovered.
    bad_frames: u32,
}

/// Per-phase bit slicer and frame assembler.
struct PhaseRx {
    /// The last 32 sliced bits, for the sync-word correlator.
    shift: u32,
    /// Bits seen, so the correlator does not fire on a half-full register.
    filled: u32,
    /// None while hunting for a sync word.
    frame: Option<PartialFrame>,
}

struct PartialFrame {
    bytes: Vec<u8>,
    bit: u8,
    acc: u8,
    /// Total frame length once the base header has been parsed.
    need: Option<usize>,
    /// The slicer level measured over the preamble, held for the whole frame.
    ///
    /// A tracking level cannot be used here: a payload run of identical bits —
    /// 0xFF fill, a white margin, a zlib block of zeros — drags any leaky mean
    /// onto the data itself and the slicer stops discriminating. The preamble
    /// exists precisely so the offset can be measured while the data cannot
    /// bias it.
    level: f32,
}

impl RifpRx {
    pub fn new(rate: f64) -> Self {
        let sps = (rate / SYMBOL_RATE).round().max(1.0) as usize;
        RifpRx {
            sps,
            ring: VecDeque::from(vec![0.0; sps]),
            sum: 0.0,
            n: 0,
            dc: 0.0,
            // ~30 ms: tracks a mistuned carrier without following the data.
            dc_alpha: (1.0 / (0.030 * rate)) as f32,
            sym_mag: 0.0,
            raw_mag: 0.0,
            phases: (0..sps).map(|_| PhaseRx::new()).collect(),
            pending_bad: None,
            last_good: None,
            bad_frames: 0,
        }
    }

    /// Frames that failed CRC and that no timing phase managed to recover — an
    /// honest measure of how the channel is doing.
    ///
    /// It has to be counted per transmitted *frame* rather than per decode.
    /// Several timing phases decode each frame; the ones reading across symbol
    /// boundaries make occasional bit errors on patterns the aligned phase gets
    /// right, so counting every failure would report a clean signal as a
    /// channel full of errors. A failure is therefore ignored when a valid
    /// frame lands within a byte-time either side of it — comfortably more than
    /// the `sps` samples that separate two phases finishing the same frame, and
    /// far less than the gap between two transmitted ones.
    pub fn bad_frames(&self) -> u32 {
        self.bad_frames
    }

    /// How long a CRC failure waits to see whether another phase recovers the
    /// same frame.
    fn recovery_window(&self) -> u64 {
        (self.sps * 8) as u64
    }

    /// Modem lock quality, ~0 in noise and ~1 on a clean in-band FSK signal.
    ///
    /// The ratio of the matched filter's output magnitude to the raw
    /// discriminator's: averaging a symbol's worth of a real signal leaves its
    /// amplitude alone, while averaging the same span of noise shrinks it
    /// towards zero. Deliberately a *ratio*, because the digital tap is
    /// post-AGC and absolute levels there mean nothing. Noise lands near 0.29,
    /// random data near 0.75.
    pub fn level(&self) -> f32 {
        if self.raw_mag <= 1e-6 {
            return 0.0;
        }
        let ratio = self.sym_mag / self.raw_mag;
        ((ratio - 0.35) / 0.35).clamp(0.0, 1.0)
    }

    /// Consume a block of discriminator samples, appending complete frames.
    pub fn process(&mut self, input: &[f32], out: &mut Vec<RifpFrame>) {
        for &x in input {
            let old = self.ring.pop_front().unwrap_or(0.0);
            self.ring.push_back(x);
            self.sum += x - old;
            let symbol = self.sum / self.sps as f32;

            self.dc += self.dc_alpha * (symbol - self.dc);
            self.sym_mag += 0.0005 * (symbol.abs() - self.sym_mag);
            self.raw_mag += 0.0005 * (x.abs() - self.raw_mag);

            // The window just closed starts at sample n+1-sps, so it belongs to
            // phase (n + 1) mod sps.
            self.n = self.n.wrapping_add(1);
            // A held failure that nothing recovered in time is a real one.
            if self.pending_bad.is_some_and(|at| self.n - at > self.recovery_window()) {
                self.pending_bad = None;
                self.bad_frames += 1;
            }
            let phase = (self.n % self.sps as u64) as usize;
            if let Some(frame) = self.phases[phase].push_symbol(symbol, self.dc) {
                match RifpFrame::parse(&frame) {
                    Ok(f) => {
                        self.pending_bad = None;
                        self.last_good = Some(self.n);
                        out.push(f);
                    }
                    // Only the first failure of a group counts, and only when
                    // no phase has just decoded the same frame cleanly.
                    Err(FrameError::Crc)
                        if self.last_good.is_none_or(|at| self.n - at > self.recovery_window()) =>
                    {
                        self.pending_bad = self.pending_bad.or(Some(self.n));
                    }
                    Err(_) => {}
                }
            }
        }
    }
}

impl PhaseRx {
    fn new() -> Self {
        PhaseRx { shift: 0, filled: 0, frame: None }
    }

    /// Feed one matched-filter symbol. `hunt_level` is the tracking slicer
    /// level, used while looking for a sync word and frozen into the frame that
    /// follows one. Returns the raw bytes of a frame once one is fully
    /// collected (the caller parses and CRC-checks it).
    fn push_symbol(&mut self, symbol: f32, hunt_level: f32) -> Option<Vec<u8>> {
        let Some(frame) = self.frame.as_mut() else {
            self.shift = (self.shift << 1) | (symbol > hunt_level) as u32;
            self.filled = self.filled.saturating_add(1);
            if self.filled >= 32 && (self.shift ^ SYNC_WORD).count_ones() <= SYNC_MAX_ERRORS {
                // Locked: the header starts at the next bit. Do not clear the
                // shift register — a sync word inside the next frame's data is
                // harmless, and clearing would blind us to a nearer match.
                self.frame = Some(PartialFrame {
                    bytes: Vec::with_capacity(BASE_HEADER_LEN),
                    bit: 0,
                    acc: 0,
                    need: None,
                    level: hunt_level,
                });
            }
            return None;
        };

        let bit = symbol > frame.level;
        frame.acc = (frame.acc << 1) | bit as u8;
        frame.bit += 1;
        if frame.bit < 8 {
            return None;
        }
        frame.bytes.push(frame.acc);
        frame.bit = 0;
        frame.acc = 0;

        if frame.need.is_none() && frame.bytes.len() == BASE_HEADER_LEN {
            match BaseHeader::parse(&frame.bytes) {
                // Reject implausible frames before buffering them: a sync word
                // matched out of noise must not be able to hold this phase deaf
                // for 64 KB of bits.
                Ok(h)
                    if matches!(
                        h.frame_type,
                        FRAME_MANIFEST | FRAME_DATA | FRAME_END | FRAME_CANCEL
                    ) && h.payload_len as usize <= RX_MAX_PAYLOAD =>
                {
                    frame.need = Some(h.frame_len());
                }
                _ => {
                    self.frame = None;
                    return None;
                }
            }
        }
        if frame.need == Some(frame.bytes.len()) {
            let done = self.frame.take().expect("checked above");
            return Some(done.bytes);
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc32_check_value() {
        // §9: the check value for the ASCII octets 123456789.
        assert_eq!(crc32(b"123456789"), 0xCBF4_3926);
        assert_eq!(crc32(b""), 0);
    }

    /// §16 Test Vector: a DATA frame, session 0x0123456789ABCDEF, sequence 1 of
    /// 2, no flags, no extensions, payload "abc".
    #[test]
    fn draft_test_vector() {
        let frame = RifpFrame::new(FRAME_DATA, 0x0123_4567_89AB_CDEF, 1, 2, b"abc".to_vec());
        let encoded = frame.encode();
        let hex: String = encoded.iter().map(|b| format!("{b:02x}")).collect();
        assert_eq!(hex, "0100021c000000000123456789abcdef000000010000000200030000616263ed2bb111");
        assert_eq!(RifpFrame::parse(&encoded).unwrap(), frame);
    }

    #[test]
    fn tlvs_round_trip() {
        let mut frame = RifpFrame::new(FRAME_MANIFEST, 7, 0, 3, b"{}".to_vec());
        frame.tlvs.push(Tlv::text(TLV_SENDER_ID, "OE1XYZ"));
        frame.tlvs.push(Tlv::text(TLV_RADIO_PROFILE, "rifp-cpfsk-4800"));
        let encoded = frame.encode();
        // 28 + (4+6) + (4+15)
        assert_eq!(encoded[3], 57);
        let back = RifpFrame::parse(&encoded).unwrap();
        assert_eq!(back.text_tlv(TLV_SENDER_ID), Some("OE1XYZ"));
        assert_eq!(back.text_tlv(TLV_RADIO_PROFILE), Some("rifp-cpfsk-4800"));
        assert_eq!(back, frame);
    }

    #[test]
    fn rejects_malformed_frames() {
        let frame = RifpFrame::new(FRAME_DATA, 1, 0, 1, b"xy".to_vec());
        let good = frame.encode();

        let mut bad_crc = good.clone();
        *bad_crc.last_mut().unwrap() ^= 0xFF;
        assert_eq!(RifpFrame::parse(&bad_crc), Err(FrameError::Crc));

        let mut bad_major = good.clone();
        bad_major[0] = 2;
        assert_eq!(RifpFrame::parse(&bad_major), Err(FrameError::Version(2)));

        let mut reserved = good.clone();
        reserved[27] = 1;
        assert_eq!(RifpFrame::parse(&reserved), Err(FrameError::Reserved));

        let mut short_header = good.clone();
        short_header[3] = 27;
        assert_eq!(RifpFrame::parse(&short_header), Err(FrameError::HeaderLength(27)));

        assert_eq!(RifpFrame::parse(&good[..good.len() - 1]), Err(FrameError::Truncated));
    }

    #[test]
    fn rejects_unknown_critical_features() {
        let mut flagged = RifpFrame::new(FRAME_DATA, 1, 0, 1, Vec::new());
        flagged.flags = 1 << 16; // an unassigned critical flag
        assert_eq!(RifpFrame::parse(&flagged.encode()), Err(FrameError::UnknownCritical));

        let mut tlv = RifpFrame::new(FRAME_DATA, 1, 0, 1, Vec::new());
        tlv.tlvs.push(Tlv { type_id: TLV_CRITICAL | 0x0123, value: vec![1, 2] });
        assert_eq!(RifpFrame::parse(&tlv.encode()), Err(FrameError::UnknownCritical));

        // An advisory flag and an unknown non-critical TLV are both fine.
        let mut ok = RifpFrame::new(FRAME_DATA, 1, 0, 1, Vec::new());
        ok.flags = FLAG_RETRANSMISSION;
        ok.tlvs.push(Tlv { type_id: 0x0456, value: vec![9] });
        assert_eq!(RifpFrame::parse(&ok.encode()), Ok(ok));
    }

    /// Modulate a few frames and demodulate them straight back: the NRZ
    /// waveform the modulator would integrate is, after an ideal discriminator,
    /// the same waveform — so TX and RX meet without a radio in between.
    #[test]
    fn modem_loopback() {
        let rate = 48_000.0;
        let frames: Vec<RifpFrame> = (0..3)
            .map(|i| {
                let mut f = RifpFrame::new(
                    FRAME_DATA,
                    0xDEAD_BEEF_0000_0001,
                    i,
                    3,
                    (0..192u32).map(|b| (b as u8).wrapping_mul(37).wrapping_add(i as u8)).collect(),
                );
                f.tlvs.push(Tlv::text(TLV_SENDER_ID, "TEST"));
                f
            })
            .collect();

        let mut tx = RifpTx::new(frames.iter().map(|f| f.air_frame()).collect(), rate);
        let mut rx = RifpRx::new(rate);
        let mut got = Vec::new();
        let mut block = [0.0f32; 480];
        let mut peak_level = 0.0f32;
        while !tx.done() {
            tx.next_block(&mut block);
            rx.process(&block, &mut got);
            peak_level = peak_level.max(rx.level());
        }
        // Flush the receiver's pipeline.
        rx.process(&[0.0; 480], &mut got);

        // Several timing phases decode the same frame; they land next to each
        // other, which is exactly what the session layer de-duplicates.
        got.dedup();
        assert_eq!(got, frames, "{} frames recovered", got.len());
        assert_eq!(rx.bad_frames(), 0);
        assert!(peak_level > 0.7, "lock indicator only reached {peak_level}");
    }

    /// Noise alone must not read as a signal, and must not manufacture frames.
    #[test]
    fn noise_does_not_lock() {
        let mut rx = RifpRx::new(48_000.0);
        let mut got = Vec::new();
        let mut seed = 0xACE1_2345u32;
        let mut peak = 0.0f32;
        for _ in 0..200 {
            let mut block = [0.0f32; 480];
            for s in &mut block {
                seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                *s = (seed >> 16) as f32 / 32_768.0 - 1.0;
            }
            rx.process(&block, &mut got);
            peak = peak.max(rx.level());
        }
        assert!(got.is_empty(), "{} frames out of pure noise", got.len());
        assert!(peak < 0.35, "noise read as a locked signal ({peak})");
    }

    /// A payload that is one long run of identical bits: the case a tracking
    /// slicer level gets wrong, because the run drags the level onto the data.
    #[test]
    fn modem_survives_unbalanced_payload() {
        let rate = 48_000.0;
        let frame = RifpFrame::new(FRAME_DATA, 0x5555_0000_AAAA_1111, 3, 9, vec![0xFF; 256]);
        let mut tx = RifpTx::new(vec![frame.air_frame()], rate);
        let mut rx = RifpRx::new(rate);
        let mut got = Vec::new();
        let mut block = [0.0f32; 480];
        while !tx.done() {
            tx.next_block(&mut block);
            rx.process(&block, &mut got);
        }
        rx.process(&[0.0; 480], &mut got);
        got.dedup();
        assert_eq!(got, vec![frame]);
    }

    /// The same, through noise and a residual frequency offset — the slicer
    /// tracks the offset and the matched filter carries the rest.
    #[test]
    fn modem_survives_noise_and_offset() {
        let rate = 48_000.0;
        let frame = RifpFrame::new(FRAME_MANIFEST, 0x0102_0304_0506_0708, 0, 4, vec![0xA5; 64]);
        let mut tx = RifpTx::new(vec![frame.air_frame()], rate);
        let mut rx = RifpRx::new(rate);
        let mut got = Vec::new();
        let mut block = [0.0f32; 480];
        // Deterministic pseudo-noise: a linear congruential generator, ±0.25
        // peak against a ±1 signal, plus a 400 Hz tuning error (0.1 of full
        // deviation).
        let mut seed = 0x1234_5678u32;
        while !tx.done() {
            tx.next_block(&mut block);
            for s in &mut block {
                seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                let noise = (seed >> 16) as f32 / 32_768.0 - 1.0;
                *s += 0.25 * noise + 0.1;
            }
            rx.process(&block, &mut got);
        }
        rx.process(&[0.1; 4800], &mut got);
        got.dedup();
        assert_eq!(got, vec![frame]);
    }
}
