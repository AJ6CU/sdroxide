//! KISS framing — the byte-level protocol between a host and a TNC.
//!
//! sdroxide is the TNC in this arrangement, not the host: the KISS server lets
//! Pat, an APRS client or the Linux `ax25` stack use the radio, the same way the
//! TCI and rigctld servers already do. Adapted from rax25's `escape`/`unescape`;
//! see `vendor/rax25/PROVENANCE.md`.
//!
//! # One deliberate difference from upstream
//!
//! Upstream's `unescape` calls `panic!` on an unrecognised escape and asserts on
//! a trailing one. That is reasonable where the input comes from a TNC's
//! firmware over a serial cable. Here it arrives from whatever connected to a
//! TCP socket, so a malformed byte would take the whole radio down — including
//! the operator's receiver, mid-QSO, with no explanation. [`unescape`] returns a
//! [`KissError`] instead.

/// Frame delimiter.
pub const FEND: u8 = 0xC0;
/// Escape prefix.
pub const FESC: u8 = 0xDB;
/// Stands in for a literal [`FEND`] inside a frame.
pub const TFEND: u8 = 0xDC;
/// Stands in for a literal [`FESC`] inside a frame.
pub const TFESC: u8 = 0xDD;

/// The KISS command nibble. Only `Data` matters for carrying frames; the rest
/// are the host adjusting the TNC, and are worth naming so the server can log
/// what it was asked to do rather than dropping it silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Data,
    TxDelay,
    Persistence,
    SlotTime,
    TxTail,
    FullDuplex,
    SetHardware,
    Return,
    Unknown(u8),
}

impl Command {
    #[must_use]
    pub fn from_nibble(n: u8) -> Command {
        match n & 0x0F {
            0 => Command::Data,
            1 => Command::TxDelay,
            2 => Command::Persistence,
            3 => Command::SlotTime,
            4 => Command::TxTail,
            5 => Command::FullDuplex,
            6 => Command::SetHardware,
            0x0F => Command::Return,
            other => Command::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum KissError {
    #[error("KISS escape followed by {0:#04x}, which is not TFEND or TFESC")]
    BadEscape(u8),
    #[error("KISS frame ends inside an escape sequence")]
    TruncatedEscape,
}

/// Wrap `frame` in a KISS envelope on port 0, escaping as required.
#[must_use]
pub fn escape(frame: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(frame.len() + frame.len() / 8 + 3);
    out.push(FEND);
    out.push(0); // port 0, command 0 (data)
    for &b in frame {
        match b {
            FEND => out.extend_from_slice(&[FESC, TFEND]),
            FESC => out.extend_from_slice(&[FESC, TFESC]),
            b => out.push(b),
        }
    }
    out.push(FEND);
    out
}

/// Undo [`escape`] over one frame's payload — the bytes between two `FEND`s,
/// with the command byte already removed.
pub fn unescape(data: &[u8]) -> Result<Vec<u8>, KissError> {
    let mut out = Vec::with_capacity(data.len());
    let mut escaped = false;
    for &b in data {
        if escaped {
            out.push(match b {
                TFEND => FEND,
                TFESC => FESC,
                other => return Err(KissError::BadEscape(other)),
            });
            escaped = false;
        } else if b == FESC {
            escaped = true;
        } else {
            out.push(b);
        }
    }
    if escaped {
        return Err(KissError::TruncatedEscape);
    }
    Ok(out)
}

/// Accumulates bytes from a socket and yields whole KISS frames.
#[derive(Debug, Default)]
pub struct Decoder {
    buf: Vec<u8>,
    in_frame: bool,
}

impl Decoder {
    #[must_use]
    pub fn new() -> Self {
        Decoder::default()
    }

    /// Feed received bytes, appending `(command, payload)` for each complete
    /// frame. A frame that fails to unescape is dropped and reported, not
    /// fatal — the stream resynchronises on the next `FEND`.
    pub fn push(
        &mut self,
        bytes: &[u8],
        out: &mut Vec<(Command, Vec<u8>)>,
    ) -> Result<(), KissError> {
        let mut err = Ok(());
        for &b in bytes {
            if b == FEND {
                if self.in_frame && !self.buf.is_empty() {
                    let cmd = Command::from_nibble(self.buf[0]);
                    match unescape(&self.buf[1..]) {
                        Ok(payload) => out.push((cmd, payload)),
                        Err(e) => err = Err(e),
                    }
                }
                // A FEND both closes and opens: back-to-back frames share one,
                // and a run of them is an idle line.
                self.buf.clear();
                self.in_frame = true;
                continue;
            }
            if self.in_frame {
                self.buf.push(b);
            }
        }
        err
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_frame_round_trips() {
        let frame = vec![0x01, FEND, 0x02, FESC, 0x03, TFEND, TFESC];
        let wire = escape(&frame);
        assert_eq!(wire[0], FEND);
        assert_eq!(*wire.last().unwrap(), FEND);
        assert_eq!(unescape(&wire[2..wire.len() - 1]).unwrap(), frame);
    }

    #[test]
    fn the_decoder_finds_frames_in_a_stream() {
        let a = vec![0x11, FEND, 0x22];
        let b = vec![0x33, FESC];
        let mut wire = escape(&a);
        wire.extend(escape(&b));

        let mut d = Decoder::new();
        let mut out = Vec::new();
        // Split the stream mid-frame, as a socket read would.
        d.push(&wire[..5], &mut out).unwrap();
        d.push(&wire[5..], &mut out).unwrap();
        assert_eq!(out, vec![(Command::Data, a), (Command::Data, b)]);
    }

    /// Malformed input must be an error, never a panic: this arrives from a
    /// socket, and taking the radio down would take the operator's receiver
    /// with it.
    #[test]
    fn malformed_escapes_are_errors_not_panics() {
        assert_eq!(unescape(&[FESC, 0x99]), Err(KissError::BadEscape(0x99)));
        assert_eq!(unescape(&[0x01, FESC]), Err(KissError::TruncatedEscape));
    }

    /// A bad frame must not poison the stream — the next one still arrives.
    #[test]
    fn a_bad_frame_does_not_stop_the_next_one() {
        let good = vec![0x42, 0x43, 0x44];
        let mut wire = vec![FEND, 0x00, FESC, 0x99, FEND];
        wire.extend(escape(&good));

        let mut d = Decoder::new();
        let mut out = Vec::new();
        let _ = d.push(&wire, &mut out);
        assert_eq!(out, vec![(Command::Data, good)]);
    }

    #[test]
    fn the_command_nibble_is_decoded() {
        assert_eq!(Command::from_nibble(0x00), Command::Data);
        assert_eq!(Command::from_nibble(0x01), Command::TxDelay);
        assert_eq!(Command::from_nibble(0x0F), Command::Return);
        // The high nibble is the port number and must not change the command.
        assert_eq!(Command::from_nibble(0x30), Command::Data);
    }
}
