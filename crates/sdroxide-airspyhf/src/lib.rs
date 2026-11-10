//! Native Airspy HF+ (Dual / Discovery / Ranger) driver.
//!
//! Pure Rust over `nusb`: no libairspyhf, no libusb, no SoapySDR module, so
//! this backend is in every build variant on every platform.
//!
//! # Shape
//!
//! One blocking thread owns the receiver ([`stream`]), control reaches it over
//! a crossbeam channel, and samples come back through an `rtrb` ring of
//! interleaved `f32` — the same shape as [`sdroxide_rtlsdr`] and
//! [`sdroxide_hpsdr`]. [`protocol`] holds the wire with no `nusb` in it, which
//! is what makes the fiddly half of this driver testable without hardware.
//!
//! # Host DSP
//!
//! The receiver's firmware does the filtering; the host does three things the
//! vendor library also does, in this order ([`convert::HostDsp`]):
//!
//! 1. **DC cancellation**, a one-pole average subtracted from I and Q.
//! 2. **IQ image balancing** ([`iqbalancer`]) — only on the zero-IF rates,
//!    where the mirror image is not already rejected in hardware.
//! 3. **A fine-tuning oscillator**, which brings the signal back from the
//!    5 kHz the synthesiser was deliberately parked away from it, absorbs the
//!    sub-kHz rounding of an LO programmed in whole kHz, and does *all* of the
//!    tuning below the synthesiser's 180 kHz floor — which is how this
//!    receiver reaches VLF at all.
//!
//! All three can be switched off together (`AirspyHfConfig::lib_dsp`), which is
//! also the one-click bisect between "the driver" and "the DSP" when something
//! looks wrong.
//!
//! # Provenance
//!
//! The protocol, the tuning arithmetic and the image balancer are transcribed
//! from Airspy's [libairspyhf](https://github.com/airspy/airspyhf) —
//! `libairspyhf/src/airspyhf.c`, `airspyhf_commands.h` and `iqbalancer.c`
//! (BSD-3-Clause / GPL-2.0-or-later, both compatible with this workspace's
//! GPL-3.0-or-later). No code is copied; the constants and the algorithm are
//! what matter and both are cited where they are used.
//!
//! # Not verified against hardware
//!
//! This driver was written from the reference implementation, not on a bench.
//! Every uncertain point carries a comment saying so, and [`diagnostics`]
//! renders a session trace built for exactly one purpose: making a first bug
//! report from an owner actionable.

pub mod convert;
pub mod device;
pub mod error;
pub mod handle;
pub mod iqbalancer;
pub mod protocol;
pub mod stream;
pub mod trace;
pub mod usb;

pub use device::Device;
pub use error::{Error, Result};
pub use handle::AirspyHfHandle;
pub use iqbalancer::IqBalancer;
pub use trace::{FIELD_REPORT_HINT, diagnostics};
pub use usb::list;
