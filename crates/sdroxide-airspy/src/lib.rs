//! Native Airspy R2 / Mini driver.
//!
//! Pure Rust over `nusb`: no libairspy, no libusb, no SoapySDR module, so this
//! backend is in every build variant on every platform.
//!
//! # Not the Airspy HF+
//!
//! `sdroxide-airspyhf` next door drives a *different radio*. The two share a
//! vendor and nothing else: different silicon, different USB id, a different
//! protocol, and a different host-DSP problem. Nothing here is a port of that
//! crate and the two should not be made to look like one.
//!
//! # What the host has to do
//!
//! This receiver's ADC is **real**, and the wanted signal sits at a quarter of
//! the sample rate. So the host does the downconversion: DC removal, a
//! multiply-free fs/4 rotation, and a half-band decimator — see
//! [`convert::HostDsp`]. That is also why the rate programmed into the receiver
//! is twice the complex rate the operator picks; see
//! [`protocol::program_rate_hz`].
//!
//! # Provenance
//!
//! The protocol, the rate arithmetic and the gain curves are transcribed from
//! Airspy's [libairspy](https://github.com/airspy/airspyone_host) —
//! `libairspy/src/airspy.c` and `airspy_commands.h` (BSD-3-Clause, compatible
//! with this workspace's GPL-3.0-or-later). The half-band filter is **not**
//! transcribed: it is designed here and measured by a test, because a copied
//! coefficient table cannot tell you what it actually achieves.
//!
//! # Not verified against hardware
//!
//! This driver was written from the reference implementation, not on a bench.
//! Field reports have since settled five things that only a receiver could, and
//! each is now pinned where it was got wrong:
//!
//! * There is **no alternate setting 1** — the interface has one setting and
//!   the bulk endpoints are in it. Selecting one is what stopped the receiver
//!   opening at all on macOS. See [`protocol::ALT_SETTING`].
//! * The rate list the receiver reports is in **complex** rates, not ADC rates.
//!   See [`protocol::parse_rates`].
//! * Most of the *setters* are control **reads** carrying a return byte, and
//!   nearly all of them take their argument in `wIndex`. See [`usb::UsbDev::set`].
//! * A completion does not end on a packed group, so the remainder has to be
//!   carried into the next one.
//! * **Reads must not be posted until the receiver has been started.**
//!   `set_receiver_mode(RX)` re-initialises the bulk endpoint in the firmware,
//!   and reads posted across that call leave the pipe wedged: a receiver that
//!   configures perfectly and then sends nothing at all. See
//!   [`stream`]`::prime`.
//!
//! What a bench still has to settle is **which way the spectrum runs**: the
//! fs/4 rotation now matches libairspy's sign, which is the convention every
//! other program's picture of this hardware is drawn in, but nothing here has
//! watched a known carrier land on the correct side of the dial. The `probe`
//! example prints the two lines that answer it.

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
pub use handle::AirspyHandle;
pub use trace::{FIELD_REPORT_HINT, diagnostics};
pub use usb::list;
