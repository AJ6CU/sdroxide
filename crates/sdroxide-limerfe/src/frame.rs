//! The LimeRFE wire format, with no serial port in it.
//!
//! Transcribed from LimeSuite's `src/limeRFE/limeRFE_cmd.cpp`, and the details
//! that look like details are not:
//!
//! * **Frames are three different lengths.** Most commands are 16 bytes each
//!   way (`RFE_BUFFER_SIZE`); `MODE` and `FAN` are *two*
//!   (`RFE_BUFFER_SIZE_MODE`); and the hello handshake is a **single byte** out
//!   and a single byte back. Sending sixteen where the firmware expects two
//!   leaves fourteen stray bytes in its input, and waiting for sixteen where it
//!   sends one simply times out.
//! * **Only `CONFIG` and `MODE` report a status.** For those, `buf[1]` is the
//!   result and zero means success. For `GET_INFO`, `GET_CONFIG` and the ADC
//!   reads, `buf[1]` is *data* — so testing it as a status turns a board
//!   reporting firmware version 4 into error code 4, which is a real refusal
//!   about cellular band modes.
//! * **The reply's data starts at `buf[1]`, not `buf[2]`.** `buf[0]` is the
//!   echoed command.
//!
//! Keeping this module free of `serialport` is what makes the fiddly half of
//! the driver testable with nothing plugged in, the same split every native USB
//! driver here makes with its `protocol.rs`.

use sdroxide_types::{RfeChannel, RfeMode, RfePort};

use crate::error::{Error, Result};

/// Command codes, from LimeSuite's `limeRFE_constants.h`.
pub mod cmd {
    pub const HELLO: u8 = 0x00;
    pub const MODE: u8 = 0xd1;
    pub const CONFIG: u8 = 0xd2;
    pub const READ_ADC1: u8 = 0xa1;
    pub const READ_ADC2: u8 = 0xa2;
    pub const READ_TEMP: u8 = 0xa3;
    pub const FAN: u8 = 0xc1;
    pub const GET_INFO: u8 = 0xe1;
    pub const RESET: u8 = 0xe2;
    pub const GET_CONFIG: u8 = 0xe3;
}

/// `RFE_BUFFER_SIZE` — the ordinary frame.
pub const LEN_FULL: usize = 16;
/// `RFE_BUFFER_SIZE_MODE` — `MODE` and `FAN` only.
pub const LEN_MODE: usize = 2;
/// The hello handshake is one byte each way and is not a framed command at all.
pub const LEN_HELLO: usize = 1;

/// How many times to say hello before deciding nothing is listening, and how
/// long to leave between tries. Both are LimeSuite's numbers
/// (`RFE_MAX_HELLO_ATTEMPTS`, `RFE_TIME_BETWEEN_HELLO_MS`): the board's
/// microcontroller can be mid-boot when the port opens.
pub const MAX_HELLO_ATTEMPTS: u32 = 10;
pub const HELLO_INTERVAL_MS: u64 = 200;

/// One command, ready to put on the wire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cmd {
    bytes: [u8; LEN_FULL],
    /// How much of [`Self::bytes`] to send.
    len: usize,
    /// How many bytes the board answers with.
    reply_len: usize,
    /// Whether `buf[1]` of the reply is a status code. True only for `CONFIG`
    /// and `MODE`; everywhere else that byte carries data.
    has_status: bool,
}

impl Cmd {
    pub fn wire(&self) -> &[u8] {
        &self.bytes[..self.len]
    }
    pub fn reply_len(&self) -> usize {
        self.reply_len
    }
    pub fn code(&self) -> u8 {
        self.bytes[0]
    }

    /// Check a reply for the errors this command can report.
    ///
    /// Length always; the status byte only where there is one. The echoed
    /// command in `buf[0]` is deliberately *not* enforced — LimeSuite never
    /// checks it, so treating a mismatch as fatal would reject a board that is
    /// behaving exactly as its own software expects.
    pub fn check(&self, reply: &[u8]) -> Result<()> {
        if reply.len() != self.reply_len {
            return Err(Error::ShortReply { want: self.reply_len, got: reply.len() });
        }
        if self.has_status && reply[1] != 0 {
            return Err(Error::from_board(reply[1]));
        }
        Ok(())
    }
}

fn full(code: u8, has_status: bool) -> Cmd {
    let mut bytes = [0u8; LEN_FULL];
    bytes[0] = code;
    Cmd { bytes, len: LEN_FULL, reply_len: LEN_FULL, has_status }
}

/// The hello handshake: one byte out, the same byte back.
pub fn hello() -> Cmd {
    let mut bytes = [0u8; LEN_FULL];
    bytes[0] = cmd::HELLO;
    Cmd { bytes, len: LEN_HELLO, reply_len: LEN_HELLO, has_status: false }
}

/// Whether a hello reply is the board saying hello back.
pub fn hello_answered(reply: &[u8]) -> bool {
    reply.len() == LEN_HELLO && reply[0] == cmd::HELLO
}

pub fn get_info() -> Cmd {
    full(cmd::GET_INFO, false)
}
pub fn get_config() -> Cmd {
    full(cmd::GET_CONFIG, false)
}
pub fn reset() -> Cmd {
    full(cmd::RESET, false)
}
pub fn read_adc(adc: u8) -> Cmd {
    full(if adc == 0 { cmd::READ_ADC1 } else { cmd::READ_ADC2 }, false)
}

/// `CONFIG` — channels, ports, mode, notch and attenuation in one transaction.
///
/// The byte order is LimeSuite's and is not guessable: getting `selPortRX` and
/// `selPortTX` the wrong way round would route receive into the transmit
/// amplifier's filter and look, from the spectrum, like a dead antenna.
pub fn config(st: &RfeState) -> Cmd {
    let mut c = full(cmd::CONFIG, true);
    c.bytes[1] = st.channel_rx.code();
    c.bytes[2] = st.channel_tx.code();
    c.bytes[3] = st.port_rx.code();
    c.bytes[4] = st.port_tx.code();
    c.bytes[5] = st.mode.code();
    c.bytes[6] = u8::from(st.notch);
    c.bytes[7] = st.atten_steps.min(7);
    c.bytes[8] = u8::from(st.swr_enable);
    c.bytes[9] = u8::from(st.swr_source_cell);
    c
}

/// `MODE` — the relays only, and a **two-byte** frame. This is the one that has
/// to happen at key-down, which is why it is a command of its own rather than a
/// whole `CONFIG`.
pub fn mode(mode: RfeMode) -> Cmd {
    let mut bytes = [0u8; LEN_FULL];
    bytes[0] = cmd::MODE;
    bytes[1] = mode.code();
    Cmd { bytes, len: LEN_MODE, reply_len: LEN_MODE, has_status: true }
}

/// `FAN` — also a two-byte frame, and its reply carries no status.
pub fn fan(on: bool) -> Cmd {
    let mut bytes = [0u8; LEN_FULL];
    bytes[0] = cmd::FAN;
    bytes[1] = u8::from(on);
    Cmd { bytes, len: LEN_MODE, reply_len: LEN_MODE, has_status: false }
}

/// What the board reports about itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RfeInfo {
    pub firmware: u8,
    pub hardware: u8,
}

/// Everything the board is set to, in one transaction.
///
/// The Rust mirror of LimeSuite's `rfe_boardState`, with real types instead of
/// nine `char`s. Both transports consume this: the serial one encodes it into
/// a `CONFIG` frame, the board one memcpys it into the C struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RfeState {
    pub channel_rx: RfeChannel,
    pub channel_tx: RfeChannel,
    pub port_rx: RfePort,
    pub port_tx: RfePort,
    pub mode: RfeMode,
    pub notch: bool,
    /// Receive attenuation in 2 dB steps, 0–7.
    pub atten_steps: u8,
    pub swr_enable: bool,
    /// SWR pickup source: false = external coupler, true = the cellular
    /// amplifier's internal detector.
    pub swr_source_cell: bool,
}

impl Default for RfeState {
    fn default() -> Self {
        RfeState {
            channel_rx: RfeChannel::Wb1000,
            channel_tx: RfeChannel::Wb1000,
            port_rx: RfePort::J3,
            port_tx: RfePort::J4,
            mode: RfeMode::Rx,
            notch: false,
            atten_steps: 0,
            swr_enable: false,
            swr_source_cell: false,
        }
    }
}

/// Decode a `GET_INFO` reply. Firmware in `buf[1]`, hardware in `buf[2]`.
pub fn decode_info(reply: &[u8]) -> Result<RfeInfo> {
    get_info().check(reply)?;
    Ok(RfeInfo { firmware: reply[1], hardware: reply[2] })
}

/// Decode a `GET_CONFIG` reply — the board's own account of its state, in the
/// same byte order [`config`] writes, starting at `buf[1]`.
pub fn decode_state(reply: &[u8]) -> Result<RfeState> {
    get_config().check(reply)?;
    Ok(RfeState {
        channel_rx: RfeChannel::from_code(reply[1]),
        channel_tx: RfeChannel::from_code(reply[2]),
        port_rx: port_from_code(reply[3]),
        port_tx: port_from_code(reply[4]),
        mode: mode_from_code(reply[5]),
        notch: reply[6] != 0,
        atten_steps: reply[7].min(7),
        swr_enable: reply[8] != 0,
        swr_source_cell: reply[9] != 0,
    })
}

/// Decode an ADC reading: **low byte first**, `buf[2] * 256 + buf[1]`.
pub fn decode_adc(adc: u8, reply: &[u8]) -> Result<u16> {
    read_adc(adc).check(reply)?;
    Ok(u16::from(reply[2]) << 8 | u16::from(reply[1]))
}

fn port_from_code(code: u8) -> RfePort {
    match code {
        2 => RfePort::J4,
        3 => RfePort::J5,
        _ => RfePort::J3,
    }
}

fn mode_from_code(code: u8) -> RfeMode {
    match code {
        1 => RfeMode::Tx,
        2 => RfeMode::None,
        3 => RfeMode::TxRx,
        _ => RfeMode::Rx,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The three frame lengths, which is what the first field report was about:
    /// a hello sent as sixteen bytes is answered with one, and waiting for the
    /// other fifteen times out.
    #[test]
    fn each_command_has_the_length_limesuite_uses() {
        assert_eq!(hello().wire(), &[0x00], "RFE_CMD_HELLO is one byte, not a frame");
        assert_eq!(hello().reply_len(), 1);

        // RFE_BUFFER_SIZE_MODE.
        assert_eq!(mode(RfeMode::Tx).wire(), &[0xd1, 1]);
        assert_eq!(mode(RfeMode::Tx).reply_len(), 2);
        assert_eq!(fan(true).wire(), &[0xc1, 1]);
        assert_eq!(fan(true).reply_len(), 2);

        // RFE_BUFFER_SIZE for everything else.
        for c in [get_info(), get_config(), reset(), read_adc(0), config(&RfeState::default())] {
            assert_eq!(c.wire().len(), 16, "{:#04x} should be a full frame", c.code());
            assert_eq!(c.reply_len(), 16);
        }
    }

    /// The exact bytes for a known state. Written out literally rather than
    /// computed, because a test that builds the expectation the same way the
    /// code does would agree with any byte order at all.
    #[test]
    fn a_config_frame_is_the_bytes_limesuite_sends() {
        let st = RfeState {
            channel_rx: RfeChannel::Ham0145,
            channel_tx: RfeChannel::Ham0435,
            port_rx: RfePort::J3,
            port_tx: RfePort::J4,
            mode: RfeMode::TxRx,
            notch: true,
            atten_steps: 3,
            swr_enable: true,
            swr_source_cell: false,
        };
        assert_eq!(
            config(&st).wire(),
            &[0xd2, 5, 7, 1, 2, 3, 1, 3, 1, 0, 0, 0, 0, 0, 0, 0],
            "command, rx ch, tx ch, rx port, tx port, mode, notch, att, swr, swr src"
        );
    }

    #[test]
    fn attenuation_is_clamped_into_the_frame() {
        let st = RfeState { atten_steps: 200, ..RfeState::default() };
        assert_eq!(config(&st).wire()[7], 7);
    }

    /// The board answers hello with the same byte it was sent.
    #[test]
    fn the_hello_answer_is_the_hello_byte() {
        assert!(hello_answered(&[0x00]));
        assert!(!hello_answered(&[0xd2]), "some other command's leftover reply");
        assert!(!hello_answered(&[]), "nothing at all");
        assert!(!hello_answered(&[0x00, 0x00]), "a framed reply is not a hello");
    }

    /// A truncated read is a timeout, never a small success.
    #[test]
    fn a_short_reply_is_an_error_not_a_partial_success() {
        let err = get_config().check(&[cmd::GET_CONFIG, 0, 0, 0]).unwrap_err();
        assert!(matches!(err, Error::ShortReply { want: 16, got: 4 }), "{err}");
        // And a *long* one is equally wrong — that is a desynchronised link.
        assert!(mode(RfeMode::Rx).check(&[0; 16]).is_err());
    }

    /// The trap that would have shipped: only `CONFIG` and `MODE` put a status
    /// in `buf[1]`. Everywhere else that byte is data, and testing it as a
    /// status turns a board reporting firmware version 4 into error code 4 —
    /// a real refusal about cellular band modes.
    #[test]
    fn only_config_and_mode_have_a_status_byte() {
        let mut reply = [0u8; 16];
        reply[0] = cmd::GET_INFO;
        reply[1] = 4; // firmware version 4
        reply[2] = 7; // hardware version 7
        assert_eq!(
            decode_info(&reply).unwrap(),
            RfeInfo { firmware: 4, hardware: 7 },
            "a firmware version must not be read as an error code"
        );

        // Whereas a CONFIG really does report there.
        let mut bad = [0u8; 16];
        bad[0] = cmd::CONFIG;
        bad[1] = 3;
        assert!(config(&RfeState::default()).check(&bad).is_err());
        // ...and zero is success.
        bad[1] = 0;
        assert!(config(&RfeState::default()).check(&bad).is_ok());
    }

    /// Every refusal the board can give back has to arrive as a sentence, not
    /// a number — these are the ones an operator can actually act on.
    #[test]
    fn every_board_error_code_becomes_something_actionable() {
        let mut reply = [0u8; 16];
        reply[0] = cmd::CONFIG;
        let c = config(&RfeState::default());
        for code in [1u8, 2, 3, 4, 5, 6] {
            reply[1] = code;
            let text = c.check(&reply).unwrap_err().to_string();
            assert!(text.len() > 20, "code {code} gave only {text:?}");
            assert!(!text.contains("unknown"), "code {code} fell through: {text}");
        }
        reply[1] = 99;
        assert!(c.check(&reply).is_err());
    }

    /// A state written and read back is the same state. The reply's fields
    /// start at `buf[1]` — the same offset the request uses — which is the
    /// off-by-one this test exists to pin.
    #[test]
    fn a_state_survives_the_board_reporting_it_back() {
        let st = RfeState {
            channel_rx: RfeChannel::Ham0030,
            channel_tx: RfeChannel::Ham0030,
            port_rx: RfePort::J5,
            port_tx: RfePort::J5,
            mode: RfeMode::Rx,
            notch: true,
            atten_steps: 7,
            swr_enable: true,
            swr_source_cell: true,
        };
        // The board answers in the same layout the request carries.
        let mut reply = config(&st).wire().to_vec();
        reply[0] = cmd::GET_CONFIG;
        assert_eq!(decode_state(&reply).unwrap(), st);
    }

    /// The ADC is low byte first, in `buf[1]`, with the high byte after it —
    /// not the other way round and not at `buf[2..4]`.
    #[test]
    fn an_adc_reading_is_low_byte_first() {
        let mut reply = [0u8; 16];
        reply[0] = cmd::READ_ADC1;
        reply[1] = 0x34; // low
        reply[2] = 0x02; // high
        assert_eq!(decode_adc(0, &reply).unwrap(), 0x0234);

        reply[1] = 0xff;
        reply[2] = 0x03;
        assert_eq!(decode_adc(0, &reply).unwrap(), 1023, "full scale on a 10-bit converter");
    }
}
