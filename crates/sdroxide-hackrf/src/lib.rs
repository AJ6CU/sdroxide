//! Native HackRF One and HackRF Pro (and Jawbreaker / rad1o) driver.
//!
//! Pure Rust over `nusb`: no libhackrf, no libusb, no SoapySDR module, so this
//! backend is in every build variant on every platform.
//!
//! # Why this exists when SoapyHackRF already works
//!
//! Because it does not, quite. `sdroxide-radio`'s SoapySDR path carries four
//! workarounds that exist only for this radio — the receive amp going dead
//! after the first over, the transmit amp never working at all, a receive
//! stream that returns aliased buffers unless it is fully closed for transmit,
//! and one 14 dB switch presented as two independent gain elements. All four
//! are SoapyHackRF's shape rather than the hardware's, and none of them is
//! reachable from here: this driver re-programs the front end on every
//! direction change, which is what those workarounds were compensating for.
//!
//! # Shape
//!
//! One blocking thread owns the radio, control reaches it over a crossbeam
//! channel, and samples come back through an `rtrb` ring — the same shape as
//! [`sdroxide_rtlsdr`] and `sdroxide_airspyhf`. [`protocol`] holds the wire
//! with no `nusb` in it, which is what makes the fiddly half of this driver
//! testable without hardware.
//!
//! Unlike the two receivers next door, this radio transmits, and it is **half
//! duplex**: receive stops for the length of an over. Two consequences run
//! through the whole driver and are worth stating once here.
//!
//! *A mis-ordered control sequence emits RF.* A receive-only driver that gets
//! its orderings wrong produces bad samples; this one produces a signal. That
//! is why [`protocol`] is pure, why every ordering is asserted in a test
//! against a recording transport rather than trusted, and why every teardown
//! path — including `Drop` — ends at [`protocol::TransceiverMode::Off`].
//!
//! *Silence is not death.* There is no receive traffic during a transmission,
//! so the "the device has gone quiet, reopen it" timer every USB backend runs
//! has to be suppressed while the transmitter is keyed. Unsuppressed it
//! declares the radio dead three seconds into any long over.
//!
//! # Provenance
//!
//! The protocol is transcribed from Great Scott Gadgets'
//! [libhackrf](https://github.com/greatscottgadgets/hackrf) —
//! `host/libhackrf/src/hackrf.c` and `hackrf.h` (GPL-2.0-or-later, compatible
//! with this workspace's GPL-3.0-or-later). No code is copied; the request
//! codes, the transfer directions and the arithmetic are what matter, and each
//! is cited where it is used.

pub mod convert;
pub mod device;
pub mod error;
pub mod handle;
pub mod protocol;
pub mod stream;
pub mod trace;
pub mod usb;

pub use device::Device;
pub use error::{Error, Result};
pub use handle::HackRfHandle;
pub use trace::{FIELD_REPORT_HINT, diagnostics};
pub use usb::list;
