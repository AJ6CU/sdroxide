//! Errors, and the translation of USB failures into sentences an operator can
//! act on.
//!
//! Everything here ends up in front of a user, so "permission denied (os error
//! 13)" is not good enough — the message has to say which physical thing to do.
//! This backend has one failure mode the RTL-SDR does not: the receiver needs
//! more bandwidth than USB 2.0 has, and a USB 2.0 cable fits the Mk2's USB 3.0
//! micro-B socket perfectly well. That misconnection is silent, common, and
//! presents as a receiver that merely seems poor, so it gets its own variant.

use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// No receiver matched — either none is plugged in, or none with the
    /// configured serial.
    #[error("{0}")]
    NotFound(String),

    /// The device is there but we cannot have it. Carries the actionable
    /// sentence, not the errno.
    #[error("{0}")]
    Access(String),

    /// A USB transfer failed.
    #[error("USB {op} failed: {source}")]
    Transfer { op: &'static str, source: nusb::transfer::TransferError },

    /// The bundled or supplied FX3 image is not a firmware image we can load.
    #[error("firmware image is invalid: {0}")]
    BadFirmware(String),

    /// The image loaded but the device never came back under its programmed id.
    #[error(
        "the RX-888 did not re-appear after the firmware was uploaded — \
         unplug it, plug it back in, and try again"
    )]
    FirmwareNoReenumerate,

    /// A setting the hardware cannot produce.
    #[error("{0}")]
    Unsupported(String),

    /// The VHF tuner refused.
    ///
    /// Kept as its own variant rather than flattened into a string because it
    /// is the one failure the band machine acts on: a tuner that will not come
    /// up sends the receiver back to HF and latches, instead of retrying an
    /// I2C conversation on every step of a dial spin.
    #[error("RX-888 VHF tuner: {0}")]
    Tuner(#[from] sdroxide_r82xx::Error),

    #[error("USB error: {0}")]
    Usb(#[from] nusb::Error),
}

impl Error {
    /// Translate a device-open failure into an instruction.
    pub fn from_open(e: nusb::Error, what: &dyn fmt::Display) -> Error {
        use nusb::ErrorKind;
        match e.kind() {
            ErrorKind::PermissionDenied => Error::Access(format!(
                "permission denied opening {what} — install the udev rule \
                 (see the README) and re-plug the receiver"
            )),
            ErrorKind::Busy => Error::Access(format!(
                "{what} is held by another program (ka9q-radio's radiod, \
                 rx888_stream, a SoapySDR client)"
            )),
            // On Windows the FX3 must be bound to WinUSB before anything can
            // claim it; unbound, the open fails as unsupported or not-found
            // rather than as a permission problem.
            ErrorKind::Unsupported | ErrorKind::NotFound if cfg!(windows) => {
                Error::Access(format!(
                    "{what} is not bound to WinUSB — run Zadig and select the \
                     WinUSB driver for this device"
                ))
            }
            ErrorKind::Disconnected => {
                Error::NotFound(format!("{what} was unplugged while opening it"))
            }
            _ => Error::Access(format!("cannot open {what}: {e}")),
        }
    }
}
