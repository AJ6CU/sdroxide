//! Errors, and the translation of USB failures into sentences an operator can
//! act on.
//!
//! Everything here ends up in front of a user: [`Error`] is what
//! `EladSource::open` returns and what `IqSource::open_status` puts on screen.
//! "permission denied (os error 13)" tells nobody what to do; "install the udev
//! rule and re-plug the receiver" does.

use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// No device matched — either none is plugged in, or none with the
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
    #[error("short control read on request 0x{request:02X}: wanted {want} bytes, got {got}")]
    ShortRead { request: u8, want: usize, got: usize },

    /// The device answered a command with something other than the
    /// acknowledgement it is documented to answer with.
    #[error("{what} was not acknowledged: expected 0x{want:02X}, got {got}")]
    NotAcknowledged { what: &'static str, want: u8, got: String },

    /// The device's descriptors are not the shape this driver expects — the
    /// bulk endpoint is missing, most likely. Carries what was found, so a bug
    /// report names the real layout.
    #[error("{0}")]
    Descriptor(String),

    #[error("USB error: {0}")]
    Usb(#[from] nusb::Error),
}

impl Error {
    /// Translate a device-open failure into an instruction.
    ///
    /// The cases below are the entire support burden of this backend. `EBUSY`
    /// on an ELAD is nearly always ELAD's own FDM-SW2 still running under Wine
    /// or in a VM with the device passed through — there is no in-tree kernel
    /// driver that claims `1721:*`, so nothing else can be holding it.
    pub fn from_open(e: nusb::Error, what: &dyn fmt::Display) -> Error {
        use nusb::ErrorKind;
        match e.kind() {
            ErrorKind::PermissionDenied => Error::Access(format!(
                "permission denied opening {what} — install the udev rule \
                 (see the README) and re-plug the receiver"
            )),
            ErrorKind::Busy => Error::Access(format!(
                "{what} is held by another program (ELAD FDM-SW2, a GNU Radio \
                 flowgraph using gr-elad, or a virtual machine the device has \
                 been passed through to)"
            )),
            // On Windows the device must be bound to WinUSB before anything can
            // claim it. ELAD's own driver package binds it to *their* Cypress
            // driver instead, which is the usual reason this fails there.
            ErrorKind::Unsupported | ErrorKind::NotFound if cfg!(windows) => {
                Error::Access(format!(
                    "{what} is not bound to WinUSB — it is most likely still on \
                     ELAD's own driver, which only their software can use. Run \
                     Zadig and select WinUSB for this device (note that this \
                     stops FDM-SW2 from seeing it until the driver is put back)"
                ))
            }
            ErrorKind::Disconnected => {
                Error::NotFound(format!("{what} was unplugged while opening it"))
            }
            // macOS passes unrecognised IOKit failures through as a bare hex
            // `IOReturn`, which tells an operator nothing. Both remedies are
            // physical.
            ErrorKind::Other if cfg!(target_os = "macos") => Error::Access(format!(
                "cannot open {what}: {e} — quit any other software holding the \
                 receiver, then unplug it and plug it back in"
            )),
            _ => Error::Access(format!("cannot open {what}: {e}")),
        }
    }
}
