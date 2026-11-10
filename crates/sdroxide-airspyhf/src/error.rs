//! Errors, and the translation of USB failures into sentences an operator can
//! act on.
//!
//! Everything here ends up in front of a user: [`crate::Error`] is what
//! `AirspyHfSource::open` returns and what `IqSource::open_status` puts on
//! screen. "permission denied (os error 13)" tells nobody what to do; "install
//! the udev rule and re-plug the receiver" does.

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

    /// A control transfer returned fewer bytes than the caller needed.
    ///
    /// Not automatically a fault: vendor request 14 deliberately asks for four
    /// times the bytes the firmware returns. This is for the requests where a
    /// short reply really is one.
    #[error("short control read on request {request}: wanted {want} bytes, got {got}")]
    ShortRead { request: u8, want: usize, got: usize },

    /// The receiver's descriptors are not the shape this driver expects — the
    /// bulk endpoint is missing from the alternate setting, most likely.
    /// Carries what was found, so a bug report names the real layout.
    #[error("{0}")]
    Descriptor(String),

    /// A setting the hardware cannot produce.
    #[error("{0}")]
    Unsupported(String),

    #[error("USB error: {0}")]
    Usb(#[from] nusb::Error),
}

impl Error {
    /// Translate a device-open failure into an instruction.
    ///
    /// The cases below are the entire support burden of this backend, so they
    /// are worth naming precisely. `EBUSY` in particular is nearly always
    /// another SDR program still holding the receiver, not a broken install —
    /// and unlike the RTL-SDR there is no kernel driver to blame, because
    /// nothing in-tree claims `03eb:800c`.
    pub fn from_open(e: nusb::Error, what: &dyn fmt::Display) -> Error {
        use nusb::ErrorKind;
        match e.kind() {
            ErrorKind::PermissionDenied => Error::Access(format!(
                "permission denied opening {what} — install the udev rule \
                 (see the README) and re-plug the receiver"
            )),
            ErrorKind::Busy => Error::Access(format!(
                "{what} is held by another program (SDR#, SDR++, SDRangel, \
                 airspyhf_rx, gqrx, a SoapySDR client)"
            )),
            // On Windows the receiver must be bound to WinUSB before anything
            // can claim it; unbound, the open fails as unsupported or
            // not-found rather than as a permission problem.
            ErrorKind::Unsupported | ErrorKind::NotFound if cfg!(windows) => {
                Error::Access(format!(
                    "{what} is not bound to WinUSB — run Zadig and select the \
                     WinUSB driver for this device, or install the Airspy \
                     package which does the same thing"
                ))
            }
            ErrorKind::Disconnected => {
                Error::NotFound(format!("{what} was unplugged while opening it"))
            }
            // macOS passes unrecognised IOKit failures through as a bare hex
            // `IOReturn`, and the one that shows up here — kIOReturnNoResources,
            // 0xe00002be, from claiming an interface on a device that is
            // mid-hotplug — tells an operator nothing. Both remedies are
            // physical.
            ErrorKind::Other if cfg!(target_os = "macos") => Error::Access(format!(
                "cannot open {what}: {e} — quit any other SDR software holding \
                 the receiver, then unplug it and plug it back in"
            )),
            _ => Error::Access(format!("cannot open {what}: {e}")),
        }
    }
}
