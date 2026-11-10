//! WSPR — the beacon mode, over `mfsk-core`'s decoder and our own transmitter.
//!
//! ## What the protocol is
//!
//! 4-FSK at 1.4648 baud with 1.4648 Hz tone spacing: 162 symbols, 110.6 s, six
//! hertz wide, starting one second into a two-minute slot. The payload is 50
//! bits — a callsign, a four-character grid and a power level — protected by a
//! rate-½ K=32 convolutional code decoded by a Fano sequential decoder. Sync is
//! not a Costas block: the low bit of every symbol reproduces a fixed 162-bit
//! pseudorandom vector, so timing is recovered by correlating against it.
//!
//! ## What is here and what is not
//!
//! The protocol lives in `mfsk-core`. What this module adds is the two things
//! that crate does not give us:
//!
//! * [`decode`] — a scan loop that keeps the **signal-to-noise ratio and the
//!   drift**. `mfsk_core::wspr::decode_scan` computes both in its coarse search
//!   and then drops them on the floor, and a WSPR spot without an SNR is not a
//!   spot: it is the number the whole network is built to collect. So the loop
//!   is rebuilt here out of that crate's public primitives, which is the entire
//!   reason this module exists.
//! * [`tx`] — a transmitter with symbol shaping. `mfsk_core::wspr::tx` says of
//!   itself that it applies none, and hard frequency steps would splatter a six
//!   hertz signal across the whole 200 Hz window.
//!
//! Plus [`hashes`], which explains why the Type-3 messages that carry a 15-bit
//! hash instead of a callsign cannot have that callsign recovered here.

pub mod decode;
pub mod hashes;
pub mod tx;

pub use decode::{WsprDecode, decode_slot};
pub use hashes::{hashed_label, is_resolved};
