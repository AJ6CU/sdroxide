//! The `rtl_tcp` wire protocol.
//!
//! Twelve bytes of greeting, then an unframed stream of 8-bit unsigned I/Q —
//! byte for byte what the RTL2832U's bulk endpoint produces, which is why the
//! conversion path in [`crate::stream`] is shared between this backend and the
//! USB one. Control travels the other way as five-byte commands and is never
//! answered: the protocol has no replies, no acknowledgements and no error
//! reporting, so everything this end knows about the far end's state is what
//! it asked for rather than what happened.
//!
//! # Provenance
//!
//! Command numbering and the greeting layout are from osmocom `librtlsdr`
//! (`src/rtl_tcp.c`), with [`Cmd::BiasTee`] as the rtl-sdr-blog fork and
//! recent osmocom builds both number it. GPL-2.0-or-later, compatible with
//! this workspace's GPL-3.0-or-later.

use crate::error::{Error, Result};

/// The greeting's magic. Every server speaking this protocol sends it —
/// including `rsp_tcp`, the SDRplay one, which is worth stating because it is
/// easy to assume otherwise. See [`RspCapabilities`].
pub const MAGIC: &[u8; 4] = b"RTL0";

/// The magic of the block an `rsp_tcp` server sends *after* the greeting when
/// it was started in extended mode.
pub const RSP_MAGIC: &[u8; 4] = b"RSP0";

/// Length of the greeting: magic, tuner type, gain count.
pub const GREETING_LEN: usize = 12;

/// A command opcode. The values are the wire numbers and must not be
/// renumbered.
///
/// Only the commands this backend sends are listed. The gaps are real
/// commands that are deliberately not used: `0x06` sets an individual IF
/// stage's gain (E4000 only), `0x07` is the demodulator's test-pattern
/// generator, `0x0a` is offset tuning (an E4000 feature; the R82xx ignores
/// it), and `0x0b`–`0x0d` override crystal frequencies, which is a far better
/// way to make a receiver silently wrong than to correct it — [`Cmd::Ppm`]
/// does the same job through the calibration the server keeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Cmd {
    /// Centre frequency in Hz. On a server that is direct sampling this sets
    /// the DDC's IF instead, which is handled inside its tuning call.
    Freq = 0x01,
    SampleRate = 0x02,
    /// Tuner gain mode: 1 is manual, 0 hands the tuner its own AGC loop.
    /// Note the polarity — this is `rtlsdr_set_tuner_gain_mode(manual)`, so
    /// the value that reads like "automatic" is the one that isn't.
    GainMode = 0x03,
    /// Tuner gain in tenths of a dB. Ignored by the server while its tuner
    /// AGC is running, so [`Cmd::GainMode`] has to go first.
    Gain = 0x04,
    /// Crystal error in ppm, applied to the server's dongle.
    Ppm = 0x05,
    /// The demodulator's own digital AGC (not the tuner's).
    RtlAgc = 0x08,
    /// Direct sampling: 0 off, 1 the ADC's I branch, 2 its Q branch.
    DirectSampling = 0x09,
    /// ~4.5 V up the far end's coax. Absent from older servers, which ignore
    /// unknown opcodes silently.
    BiasTee = 0x0e,
}

/// Encode one command. Five bytes: the opcode, then the argument big-endian.
///
/// Signed arguments (ppm, gain) are sent as their two's-complement bit
/// pattern, which is what the server reads them back out as.
pub fn frame(cmd: Cmd, arg: u32) -> [u8; 5] {
    let a = arg.to_be_bytes();
    [cmd as u8, a[0], a[1], a[2], a[3]]
}

/// The tuner chip the server reports in its greeting.
///
/// This is the *far end's* hardware. It is the only thing a client ever learns
/// about the dongle — there is no serial, no product string and no rate
/// readback in the protocol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tuner {
    Unknown,
    E4000,
    Fc0012,
    Fc0013,
    Fc2580,
    R820T,
    R828D,
}

impl Tuner {
    /// From the greeting's tuner-type word (`enum rtlsdr_tuner`).
    pub fn from_code(code: u32) -> Tuner {
        match code {
            1 => Tuner::E4000,
            2 => Tuner::Fc0012,
            3 => Tuner::Fc0013,
            4 => Tuner::Fc2580,
            5 => Tuner::R820T,
            6 => Tuner::R828D,
            _ => Tuner::Unknown,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Tuner::Unknown => "unknown tuner",
            Tuner::E4000 => "E4000",
            Tuner::Fc0012 => "FC0012",
            Tuner::Fc0013 => "FC0013",
            Tuner::Fc2580 => "FC2580",
            // The protocol cannot tell an R820T from an R820T2: librtlsdr
            // reports both as this.
            Tuner::R820T => "R820T/R820T2",
            Tuner::R828D => "R828D",
        }
    }

    /// Whether a dongle with this tuner reaches HF by tuning alone, with no
    /// direct sampling asked of it.
    ///
    /// Only one dongle does: the RTL-SDR Blog V4, which upconverts in hardware
    /// below 28.8 MHz inside the blog fork's tuning call. Over USB this is
    /// read from the EEPROM, but the protocol carries no such thing, so the
    /// tuner is all there is to go on — and a V4 is always an R828D.
    ///
    /// The inference runs the other way for a plain R828D dongle, of which
    /// there are a few: it reports the same chip, cannot upconvert, and so
    /// hears nothing on HF until direct sampling is selected explicitly. That
    /// is why [`sdroxide_types::RtlTcpConfig::hf_mode`]'s `DirectQ` is obeyed
    /// even here.
    pub fn upconverts_hf(self) -> bool {
        self == Tuner::R828D
    }
}

/// What the server says about itself when the connection opens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Greeting {
    pub tuner: Tuner,
    /// How many gain steps the far end's tuner has. Only the count is sent —
    /// never the values — so this is good for a log line and for telling a
    /// tuner with gain control from one without, and nothing else.
    pub gain_count: u32,
}

impl Greeting {
    /// Parse the twelve-byte greeting.
    pub fn parse(buf: &[u8]) -> Result<Greeting> {
        if buf.len() < GREETING_LEN {
            return Err(Error::Net(format!(
                "the server sent a {}-byte greeting; an rtl_tcp server sends {GREETING_LEN}",
                buf.len()
            )));
        }
        if &buf[..4] != MAGIC {
            let seen = String::from_utf8_lossy(&buf[..4]).to_string();
            return Err(Error::Net(format!(
                "the server greeted with {seen:?}, not \"RTL0\" — check that this \
                 is an rtl_tcp or rsp_tcp server and not another service on the \
                 same port"
            )));
        }
        Ok(Greeting {
            tuner: Tuner::from_code(u32::from_be_bytes([buf[4], buf[5], buf[6], buf[7]])),
            gain_count: u32::from_be_bytes([buf[8], buf[9], buf[10], buf[11]]),
        })
    }

    /// One-line description for logs and the device label.
    pub fn describe(&self) -> String {
        match self.gain_count {
            0 => self.tuner.name().to_string(),
            n => format!("{}, {n} gain steps", self.tuner.name()),
        }
    }
}

/// What an `rsp_tcp` server says about the RSP behind it.
///
/// # Why this is not a greeting
///
/// It is easy to assume an SDRplay server announces itself, and this driver
/// used to. It does not. Checked against SDRplay's own `RSPTCPServer` and the
/// ON5HB fork, **both greet with `"RTL0"` and a tuner type of `R820T`** — an
/// RSP poses as a dongle so that every existing rtl_tcp client keeps working.
/// The `"RSP0"` magic belongs to *this* 45-byte block, which the official
/// server sends immediately after the greeting and **only when it was started
/// with `-E`** (`rsp_tcp.c:2394-2416`).
///
/// So the two cases a client has to handle are "greeting, then samples" and
/// "greeting, then this, then samples" — not a different greeting. Reading it
/// wrong does not fail loudly: the 45 bytes are simply consumed as I/Q, which
/// puts a burst of noise at the head of the stream and nothing on screen to say
/// why.
///
/// # Field widths
///
/// Packed, 45 bytes, and **not uniformly byte-swapped**. The server sends
/// `magic`, `version`, `capabilities`, `hardware_version` and `sample_format`
/// through `htonl`, and leaves `third_antenna_freq_limit` in host order. That
/// is not a guess — it is what `rsp_tcp.c:2400-2413` does, and a client that
/// swaps it anyway gets a nonsensical frequency limit.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RspCapabilities {
    pub version: u32,
    pub capabilities: u32,
    pub hardware_version: u32,
    pub sample_format: RspSampleFormat,
    pub antenna_input_count: u8,
    pub third_antenna_name: String,
    pub tuner_count: u8,
    /// The IF gain-reduction range, in dB. The RSP's gain model is an
    /// *attenuation*, so a bigger number is less signal.
    pub ifgr_min: u8,
    pub ifgr_max: u8,
}

/// The on-wire length of [`RspCapabilities`].
pub const RSP_CAPABILITIES_LEN: usize = 45;

/// Capability bits, from `rsp_tcp_api.h`.
pub const RSP_CAP_BIAS_T: u32 = 1 << 0;
pub const RSP_CAP_REF_OUT: u32 = 1 << 1;
pub const RSP_CAP_REF_IN: u32 = 1 << 2;
pub const RSP_CAP_BROADCAST_NOTCH: u32 = 1 << 3;
pub const RSP_CAP_DAB_NOTCH: u32 = 1 << 4;
pub const RSP_CAP_AM_NOTCH: u32 = 1 << 5;
pub const RSP_CAP_RF_NOTCH: u32 = 1 << 6;
pub const RSP_CAP_AGC: u32 = 1 << 7;

/// How the far end is packing samples.
///
/// A server started with `-b 16` streams signed 16-bit little-endian instead of
/// the 8-bit unsigned every rtl_tcp client expects. There is no way to discover
/// that outside extended mode — the protocol carries no such field — so a
/// non-extended server on `-b 16` is simply undetectable, and its stream reads
/// as noise. Extended mode is what makes 16-bit safe to use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RspSampleFormat {
    #[default]
    Uint8,
    Int16,
}

impl RspSampleFormat {
    pub fn from_code(code: u32) -> RspSampleFormat {
        match code {
            2 => RspSampleFormat::Int16,
            _ => RspSampleFormat::Uint8,
        }
    }

    /// Bytes per component on the wire.
    pub fn bytes_per_component(self) -> usize {
        match self {
            RspSampleFormat::Uint8 => 1,
            RspSampleFormat::Int16 => 2,
        }
    }
}

impl RspCapabilities {
    /// Parse the 45-byte block. `None` if it is short or does not carry the
    /// magic — in which case the bytes are samples and belong to the stream.
    pub fn parse(b: &[u8]) -> Option<RspCapabilities> {
        if b.len() < RSP_CAPABILITIES_LEN || &b[..4] != RSP_MAGIC {
            return None;
        }
        let be = |i: usize| u32::from_be_bytes([b[i], b[i + 1], b[i + 2], b[i + 3]]);
        // The one field the server does not byte-swap.
        let name_end = b[25..38].iter().position(|c| *c == 0).unwrap_or(13);
        Some(RspCapabilities {
            version: be(4),
            capabilities: be(8),
            // 12..16 is `__reserved__`.
            hardware_version: be(16),
            sample_format: RspSampleFormat::from_code(be(20)),
            antenna_input_count: b[24],
            third_antenna_name: String::from_utf8_lossy(&b[25..25 + name_end]).trim().to_string(),
            tuner_count: b[42],
            ifgr_min: b[43],
            ifgr_max: b[44],
        })
    }

    pub fn has(&self, bit: u32) -> bool {
        self.capabilities & bit != 0
    }

    /// One-line summary for logs and the device label.
    pub fn describe(&self) -> String {
        let mut caps: Vec<&str> = Vec::new();
        for (bit, name) in [
            (RSP_CAP_BIAS_T, "bias-T"),
            (RSP_CAP_REF_OUT, "ref out"),
            (RSP_CAP_REF_IN, "ref in"),
            (RSP_CAP_BROADCAST_NOTCH, "BC notch"),
            (RSP_CAP_DAB_NOTCH, "DAB notch"),
            (RSP_CAP_AM_NOTCH, "AM notch"),
            (RSP_CAP_RF_NOTCH, "RF notch"),
            (RSP_CAP_AGC, "AGC"),
        ] {
            if self.has(bit) {
                caps.push(name);
            }
        }
        format!(
            "RSP hw {}, {} antenna input(s), {} tuner(s), IF gain reduction {}–{} dB, {} [{}]",
            self.hardware_version,
            self.antenna_input_count,
            self.tuner_count,
            self.ifgr_min,
            self.ifgr_max,
            match self.sample_format {
                RspSampleFormat::Uint8 => "8-bit",
                RspSampleFormat::Int16 => "16-bit",
            },
            caps.join(", ")
        )
    }
}

/// The commands an `rsp_tcp` server understands on top of the rtl_tcp ones.
///
/// Same five-byte framing as [`frame`] — these are additional opcodes on the
/// same channel, not a second protocol. Values from `rsp_tcp_api.h`, base
/// `0x1f`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum RspCmd {
    /// 0 = input A, 1 = input B, 2 = high impedance.
    SetAntenna = 0x1f,
    /// LNA state, an index rather than a dB figure: which one is "least gain"
    /// depends on the model *and* the band, which is why the RSP's own
    /// interface exposes a step control rather than a slider.
    SetLnaState = 0x20,
    /// IF gain **reduction** in dB, so a bigger number is less signal.
    SetIfGainR = 0x21,
    SetAgc = 0x22,
    SetAgcSetPoint = 0x23,
    /// Bitmask, built by the settings tab from
    /// `RtlTcpConfig::RSP_NOTCH_*` — those live in `sdroxide-types` because the
    /// settings UI is shared with the wasm client and cannot see this crate.
    /// Nothing here branches on the individual bits; the mask is passed
    /// through.
    SetNotch = 0x24,
    SetBiasT = 0x25,
    SetRefOut = 0x26,
}

/// Encode one RSP command. Identical framing to [`frame`]; separate only so the
/// two opcode spaces cannot be mixed up at a call site.
pub fn frame_rsp(cmd: RspCmd, arg: u32) -> [u8; 5] {
    let a = arg.to_be_bytes();
    [cmd as u8, a[0], a[1], a[2], a[3]]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn commands_are_opcode_then_big_endian_argument() {
        // 100.1 MHz, as `rtl_tcp` receives it.
        assert_eq!(frame(Cmd::Freq, 100_100_000), [0x01, 0x05, 0xF7, 0x67, 0xA0]);
        assert_eq!(frame(Cmd::SampleRate, 1_024_000), [0x02, 0x00, 0x0F, 0xA0, 0x00]);
        assert_eq!(frame(Cmd::GainMode, 1), [0x03, 0, 0, 0, 1]);
        assert_eq!(frame(Cmd::DirectSampling, 2), [0x09, 0, 0, 0, 2]);
        assert_eq!(frame(Cmd::BiasTee, 0), [0x0e, 0, 0, 0, 0]);
    }

    /// Gain is tenths of a dB and ppm can be negative; both go out as the
    /// two's-complement pattern the server casts back to `int`.
    #[test]
    fn negative_arguments_survive_the_round_trip() {
        let f = frame(Cmd::Ppm, (-42i32) as u32);
        assert_eq!(f[0], 0x05);
        let arg = u32::from_be_bytes([f[1], f[2], f[3], f[4]]) as i32;
        assert_eq!(arg, -42);

        // 30.0 dB as the wire carries it.
        let f = frame(Cmd::Gain, 300);
        assert_eq!(u32::from_be_bytes([f[1], f[2], f[3], f[4]]), 300);
    }

    #[test]
    fn greeting_reports_the_far_end_tuner() {
        let mut buf = Vec::from(*MAGIC);
        buf.extend_from_slice(&5u32.to_be_bytes());
        buf.extend_from_slice(&29u32.to_be_bytes());
        let g = Greeting::parse(&buf).expect("parses");
        assert_eq!(g.tuner, Tuner::R820T);
        assert_eq!(g.gain_count, 29);
        assert!(g.describe().contains("R820T"));
        assert!(!g.tuner.upconverts_hf(), "a V3 needs direct sampling for HF");
    }

    #[test]
    fn an_r828d_is_taken_to_upconvert() {
        let mut buf = Vec::from(*MAGIC);
        buf.extend_from_slice(&6u32.to_be_bytes());
        buf.extend_from_slice(&29u32.to_be_bytes());
        let g = Greeting::parse(&buf).expect("parses");
        assert_eq!(g.tuner, Tuner::R828D);
        assert!(g.tuner.upconverts_hf());
    }

    /// The two failure modes worth telling apart: something that is not an
    /// rtl_tcp server, and a server that hung up mid-greeting.
    #[test]
    fn a_foreign_greeting_names_what_arrived() {
        let mut buf = Vec::from(*b"RSP0");
        buf.extend_from_slice(&[0u8; 8]);
        let e = Greeting::parse(&buf).expect_err("must not be taken for rtl_tcp");
        assert!(e.to_string().contains("rsp_tcp"), "{e}");

        let e = Greeting::parse(b"RTL0").expect_err("a short greeting is not a greeting");
        assert!(e.to_string().contains("4-byte"), "{e}");
    }

    #[test]
    fn unknown_tuner_codes_do_not_panic() {
        assert_eq!(Tuner::from_code(0), Tuner::Unknown);
        assert_eq!(Tuner::from_code(99), Tuner::Unknown);
        assert_eq!(Tuner::from_code(6), Tuner::R828D);
    }

    /// The block is packed, mostly big-endian, and has exactly one field the
    /// server does not byte-swap. Asserted against literals because a wrong
    /// guess here produces plausible-looking nonsense rather than a failure.
    #[test]
    fn the_capability_block_parses_field_by_field() {
        let mut b = Vec::from(*b"RSP0");
        b.extend_from_slice(&1u32.to_be_bytes()); // version
        b.extend_from_slice(&(RSP_CAP_BIAS_T | RSP_CAP_AGC).to_be_bytes());
        b.extend_from_slice(&0u32.to_be_bytes()); // __reserved__
        b.extend_from_slice(&3u32.to_be_bytes()); // hardware_version
        b.extend_from_slice(&2u32.to_be_bytes()); // sample_format = INT16
        b.push(3); // antenna_input_count
        let mut name = [0u8; 13];
        name[..3].copy_from_slice(b"HiZ");
        b.extend_from_slice(&name);
        // Host order, not network order — this is what rsp_tcp.c actually does.
        b.extend_from_slice(&30_000_000i32.to_le_bytes());
        b.push(2); // tuner_count
        b.push(20); // ifgr_min
        b.push(59); // ifgr_max
        assert_eq!(b.len(), RSP_CAPABILITIES_LEN);

        let c = RspCapabilities::parse(&b).expect("a whole block parses");
        assert_eq!(c.version, 1);
        assert_eq!(c.hardware_version, 3);
        assert_eq!(c.sample_format, RspSampleFormat::Int16);
        assert_eq!(c.antenna_input_count, 3);
        assert_eq!(c.third_antenna_name, "HiZ");
        assert_eq!(c.tuner_count, 2);
        assert_eq!((c.ifgr_min, c.ifgr_max), (20, 59));
        assert!(c.has(RSP_CAP_BIAS_T) && c.has(RSP_CAP_AGC));
        assert!(!c.has(RSP_CAP_DAB_NOTCH));
        assert!(c.describe().contains("bias-T"), "{}", c.describe());
        assert!(c.describe().contains("16-bit"), "{}", c.describe());
    }

    /// Anything that is not the block is samples, and must be reported as such
    /// rather than half-parsed — those bytes go into the stream.
    #[test]
    fn anything_that_is_not_the_block_is_samples() {
        let mut b = Vec::from(*b"RSP0");
        b.extend_from_slice(&[0u8; RSP_CAPABILITIES_LEN - 4]);
        assert!(RspCapabilities::parse(&b).is_some());
        // One byte short: not enough to be sure, so not a block.
        assert!(RspCapabilities::parse(&b[..RSP_CAPABILITIES_LEN - 1]).is_none());
        // Right length, wrong magic — this is what an ordinary rtl_tcp server's
        // first 45 sample bytes look like.
        let mut samples = vec![0x7fu8; RSP_CAPABILITIES_LEN];
        samples[..4].copy_from_slice(b"RTL0");
        assert!(RspCapabilities::parse(&samples).is_none());
        assert!(RspCapabilities::parse(&[]).is_none());
    }

    /// The extra opcodes share the framing but not the space; mixing the two up
    /// at a call site would send an antenna change as a frequency.
    #[test]
    fn the_rsp_opcodes_sit_above_the_rtl_tcp_ones() {
        assert_eq!(frame_rsp(RspCmd::SetAntenna, 2), [0x1f, 0, 0, 0, 2]);
        assert_eq!(frame_rsp(RspCmd::SetLnaState, 4), [0x20, 0, 0, 0, 4]);
        assert_eq!(frame_rsp(RspCmd::SetIfGainR, 40), [0x21, 0, 0, 0, 40]);
        assert_eq!(frame_rsp(RspCmd::SetRefOut, 1), [0x26, 0, 0, 0, 1]);
        // Every RSP opcode is above every rtl_tcp one, which is what makes the
        // two safe to carry on one channel.
        for rsp in [RspCmd::SetAntenna, RspCmd::SetRefOut] {
            assert!(rsp as u8 > Cmd::BiasTee as u8, "{rsp:?} collides with an rtl_tcp opcode");
        }
    }

    #[test]
    fn the_sample_format_defaults_to_eight_bit() {
        assert_eq!(RspSampleFormat::from_code(1), RspSampleFormat::Uint8);
        assert_eq!(RspSampleFormat::from_code(2), RspSampleFormat::Int16);
        // An unknown code is 8-bit: that is what every non-extended server
        // sends, so it is the safe reading of a value we do not recognise.
        assert_eq!(RspSampleFormat::from_code(99), RspSampleFormat::Uint8);
        assert_eq!(RspSampleFormat::default(), RspSampleFormat::Uint8);
        assert_eq!(RspSampleFormat::Uint8.bytes_per_component(), 1);
        assert_eq!(RspSampleFormat::Int16.bytes_per_component(), 2);
    }
}
