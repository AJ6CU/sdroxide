//! AX.25: the link layer under packet radio, and under Winlink over the air.
//!
//! Three layers live here and nothing above them:
//!
//! * **[`hdlc`]** — the bit level. Flags, zero-bit stuffing, NRZI and the frame
//!   check sequence: bits in, verified frames out. Written here rather than
//!   vendored, because the upstream this crate borrows from talks to KISS TNCs
//!   that do all of it in firmware.
//! * **[`fcs`]** — CRC-16/X.25, used by [`hdlc`] and by nothing else.
//! * the packet codec and the connected-mode state machine — vendored, see
//!   below — which turn frames into a reliable byte stream.
//!
//! Everything below the bit level belongs to `sdroxide-dsp` (the modems) and
//! everything above the byte stream to its user: `sdroxide-winlink` for a
//! forwarding session, `sdroxide-digi` for the operator's panel.
//!
//! # Provenance
//!
//! The connected-mode state machine and the packet codec are vendored from
//! [rax25](https://github.com/ThomasHabets/rax25) by Thomas Habets, MIT
//! licensed. This crate is MIT for that reason, unlike the rest of the
//! workspace — see `vendor/rax25/PROVENANCE.md` for what was taken, what was
//! changed and why.
//!
//! # No I/O, no threads, no clock
//!
//! Nothing here reads, writes, sleeps or asks the time. The state machine is
//! driven by events and answers with actions; the framer is driven by bits.
//! Whose bits, and when, is the caller's business — which is what lets the same
//! code serve a live modem, a WAV file and a unit test.

pub mod addr;
pub mod fcs;
pub mod hdlc;
pub mod kiss;
pub mod packet;
pub mod port;
pub mod state;

pub use addr::Addr;
pub use hdlc::{Deframer, Discard, Framer};
pub use packet::{Packet, PacketType};
pub use port::{
    LinkConfig, PortClosed, PortEndpoint, PortEvent, PortHandle, PortLease, PortRequest, port_pair,
};

/// What can go wrong below the byte stream.
///
/// `thiserror` rather than upstream's `anyhow`: a leaf protocol crate should
/// name its failures so a caller can tell "that is not a callsign" from "that
/// frame was truncated", and should not oblige every dependent to link an
/// error-reporting framework.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Ax25Error {
    #[error("not a valid callsign: {0}")]
    Callsign(String),
    #[error("address field is {0} bytes, not 7")]
    AddrLength(usize),
    #[error("frame too short: {0} bytes")]
    Short(usize),
    #[error("{0}")]
    Parse(String),
}
