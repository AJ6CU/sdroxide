//! Native RX-888 Mk2 support: a direct-sampling HF receiver driven over raw USB.
//!
//! The hardware is an LTC2208 16-bit ADC sampling HF directly at up to 130 Msps,
//! a Cypress FX3 moving the result over USB 3, an Si5351 making the sample
//! clock, and a PE4312 attenuator plus an AD8370 VGA in front of the ADC.
//!
//! Two things make this different from every other backend in the workspace:
//!
//! 1. **The device has no firmware of its own.** Its FX3 has no populated boot
//!    EEPROM, so every plug-in leaves it sitting in the Cypress boot ROM as
//!    `04b4:00f3` with no radio behaviour whatsoever. sdroxide uploads an image
//!    (see `firmware/PROVENANCE.md`) over EP0, after which the device
//!    re-enumerates as `04b4:00f1` and becomes a receiver. [`crate::usb::open`]
//!    hides this; callers never see the boot-ROM state.
//!
//! 2. **There is no hardware downconverter.** The device sends 16-bit *real*
//!    samples at the full ADC rate, so `IqSource`'s complex-baseband contract is
//!    met by an overlap-save FFT downconverter on the host, which also yields
//!    the full-band spectrum as a by-product.
//!
//! Pure Rust via `nusb` — no libusb, no libSoapySDR, no C dependency.

pub mod band;
pub mod convert;
pub mod device;
pub mod error;
pub mod firmware;
pub mod handle;
pub mod protocol;
pub mod si5351;
pub mod stream;
pub mod tuner;
pub mod usb;

pub use device::{ADC_RATES, DEFAULT_ADC_HZ, Device, Settings};
pub use error::{Error, Result};
pub use handle::Rx888Handle;
pub use stream::spawn;
pub use usb::list;
