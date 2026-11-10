//! Enumeration, opening, and the vendor requests as control transfers.
//!
//! # The one invariant in this crate
//!
//! [`UsbDev`] is deliberately **not `Clone`**, and every control transfer runs
//! on the stream thread and nowhere else. `nusb`'s `Interface` is `Send + Sync`,
//! so a second thread poking the receiver would compile and would be wrong: a
//! rate change re-tunes the synthesiser and re-reads the filter gain, and a
//! frequency change is a read-back-and-recompute pair. Interleave either with
//! anything else and the host's oscillator ends up computing from an LO the
//! receiver is no longer on — a tuning error with no bad transfer to blame it
//! on. Everything from outside arrives as a [`crate::handle::Ctrl`] message.
//!
//! # Firmware differences
//!
//! Requests 14 and up were added over the product's life and older firmware
//! stalls them. Nothing here parses the version string to decide; the requests
//! are simply attempted, and [`UsbDev::optional_in`] turns any failure into
//! "this firmware does not have it" with a documented fallback and a trace
//! line. Version-sniffing would need a table of which build gained what, which
//! is exactly the sort of table that goes stale silently.

use nusb::MaybeFuture;
use nusb::transfer::{ControlIn, ControlOut, ControlType, Recipient};
use sdroxide_types::AirspyHfDevice;

use crate::error::{Error, Result};
use crate::protocol::{
    ALT_SETTING, BULK_EP, CONFIGURATION, CTRL_TIMEOUT, INTERFACE, PID, PID_BOOTLOADER, Request,
    VID, serial_from_iserial, serial_matches,
};
use crate::trace::Trace;

/// Enumerate the Airspy HF+ receivers on the USB bus.
///
/// Non-invasive: no device is opened. The strings come from sysfs on Linux, the
/// registry on Windows and IOKit on macOS, so this is safe to call at any time
/// — including while another receiver is streaming, which is what makes the
/// settings dialog's Rescan button harmless.
pub fn list() -> Vec<AirspyHfDevice> {
    let devices = match nusb::list_devices().wait() {
        Ok(d) => d,
        Err(e) => {
            tracing::warn!("USB enumeration failed: {e}");
            return Vec::new();
        }
    };
    devices
        .filter(|d| d.vendor_id() == VID && matches!(d.product_id(), PID | PID_BOOTLOADER))
        .map(|d| {
            let boot = d.product_id() == PID_BOOTLOADER;
            let name = match d.product_string() {
                // Every model reports the same product string, so this is a
                // family name and not an identification — see `AirspyHfModel`.
                Some(p) if !p.is_empty() => p.to_string(),
                _ => "Airspy HF+".to_string(),
            };
            AirspyHfDevice {
                serial: d.serial_number().and_then(serial_from_iserial),
                name: if boot {
                    // Worth naming rather than hiding: a receiver left in DFU by
                    // an interrupted vendor update is otherwise just "gone".
                    format!("{name} [in the bootloader — reflash it with Airspy's own tool]")
                } else {
                    name
                },
            }
        })
        .collect()
}

/// An opened receiver: the claimed interface plus the identity we opened it by.
///
/// Not `Clone` on purpose — see the module invariant.
pub struct UsbDev {
    iface: nusb::Interface,
    label: String,
    serial: Option<String>,
    speed: Option<nusb::Speed>,
    trace: Trace,
}

impl UsbDev {
    /// Open the receiver with this serial, or the first one found when `serial`
    /// is empty.
    pub fn open(serial: &str, trace: &Trace) -> Result<UsbDev> {
        let want = serial.trim();
        let devices = nusb::list_devices().wait()?;
        let mut candidates = devices.filter(|d| d.vendor_id() == VID && d.product_id() == PID);

        let info = if want.is_empty() {
            candidates.next().ok_or_else(|| {
                // Name the bootloader case explicitly: it is a real state to end
                // up in and "no receiver found" sends people looking at cables.
                let boot = list().iter().any(|d| d.name.contains("bootloader"));
                Error::NotFound(if boot {
                    "an Airspy HF+ is present but sitting in its bootloader — \
                     reflash it with Airspy's own firmware tool, then re-plug it"
                        .to_string()
                } else {
                    "no Airspy HF+ found on USB".to_string()
                })
            })?
        } else {
            candidates
                .find(|d| {
                    serial_matches(want, d.serial_number().and_then(serial_from_iserial).as_deref())
                })
                .ok_or_else(|| {
                    Error::NotFound(format!(
                        "no Airspy HF+ with serial {want:?} is plugged in — pick \
                         another receiver in Settings → Radio, or clear the \
                         serial to use the first one found"
                    ))
                })?
        };

        let serial = info.serial_number().and_then(serial_from_iserial);
        let label = match info.product_string() {
            Some(p) if !p.is_empty() => p.to_string(),
            _ => "Airspy HF+".to_string(),
        };
        let speed = info.speed();

        let device = info.open().wait().map_err(|e| Error::from_open(e, &label))?;

        // Only set the configuration if it is not already the one we want. On
        // Linux `set_configuration` can re-enumerate the device — which would
        // invalidate the handle we are holding — and on Windows it is not
        // supported at all. Every HF+ enumerates configured, so this is
        // normally a no-op that costs one cached read.
        match device.active_configuration() {
            Ok(c) if c.configuration_value() == CONFIGURATION => {}
            Ok(c) => {
                trace.note(format!(
                    "device is on configuration {}, selecting {CONFIGURATION}",
                    c.configuration_value()
                ));
                if let Err(e) = device.set_configuration(CONFIGURATION).wait() {
                    trace.note(format!("set_configuration failed ({e}); continuing"));
                }
            }
            Err(e) => trace.note(format!("no active configuration reported ({e}); continuing")),
        }

        let iface = device
            .detach_and_claim_interface(INTERFACE)
            .wait()
            .map_err(|e| Error::from_open(e, &label))?;

        // Alt setting 1 is where the sample endpoint lives; alt 0 is the
        // zero-bandwidth setting and has no bulk endpoint at all. This must
        // happen before any endpoint is opened.
        iface.set_alt_setting(ALT_SETTING).wait().map_err(|e| {
            Error::Access(format!(
                "cannot select alternate setting {ALT_SETTING} on {label}: {e} — \
                 the receiver's descriptors are not the shape this driver expects"
            ))
        })?;

        let dev = UsbDev { iface, label, serial, speed, trace: trace.clone() };
        dev.check_bulk_endpoint()?;

        tracing::info!(
            "opened Airspy HF+ {} (usb {VID:04x}:{PID:04x}, serial {}, {})",
            dev.label,
            dev.serial.as_deref().unwrap_or("none"),
            dev.speed_name(),
        );
        trace.note(format!(
            "claimed interface {INTERFACE} alt {ALT_SETTING} on {} (serial {}, {})",
            dev.label,
            dev.serial.as_deref().unwrap_or("none"),
            dev.speed_name(),
        ));
        Ok(dev)
    }

    /// Fail early, and name the real layout when we do.
    ///
    /// `Interface::endpoint` resolves an address against the *current* alternate
    /// setting. If a firmware ever put the sample endpoint somewhere else, the
    /// stream would fail with a bare "not found" that says nothing; listing what
    /// the descriptor does have turns a dead end into a usable bug report.
    fn check_bulk_endpoint(&self) -> Result<()> {
        let found: Vec<String> = self
            .iface
            .descriptors()
            .filter(|d| d.alternate_setting() == ALT_SETTING)
            .flat_map(|d| d.endpoints().collect::<Vec<_>>())
            .map(|e| {
                format!("0x{:02x} {:?} max {}", e.address(), e.transfer_type(), e.max_packet_size())
            })
            .collect();
        self.trace.note(format!("alt {ALT_SETTING} endpoints: [{}]", found.join(", ")));
        if found.iter().any(|e| e.starts_with(&format!("0x{BULK_EP:02x} "))) {
            return Ok(());
        }
        Err(Error::Descriptor(format!(
            "{} has no bulk IN endpoint 0x{BULK_EP:02x} on alternate setting {ALT_SETTING}; \
             it offers [{}]. Please report this with the receiver's model and firmware version.",
            self.label,
            found.join(", ")
        )))
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn serial(&self) -> Option<&str> {
        self.serial.as_deref()
    }

    pub fn speed_name(&self) -> &'static str {
        match self.speed {
            Some(nusb::Speed::Low) => "low speed",
            Some(nusb::Speed::Full) => "full speed",
            Some(nusb::Speed::High) => "high speed",
            Some(nusb::Speed::Super) => "SuperSpeed",
            Some(nusb::Speed::SuperPlus) => "SuperSpeed+",
            _ => "unknown link speed",
        }
    }

    /// Borrow the interface so the streaming code can open the bulk endpoint.
    pub fn interface(&self) -> &nusb::Interface {
        &self.iface
    }

    pub fn trace(&self) -> &Trace {
        &self.trace
    }

    // ---- vendor requests -------------------------------------------------

    /// A vendor control-IN, with the reply length recorded whether or not it
    /// matched. That column is where request 14's short answer shows up.
    pub fn control_in(&self, req: Request, value: u16, index: u16, len: u16) -> Result<Vec<u8>> {
        let r = self
            .iface
            .control_in(
                ControlIn {
                    control_type: ControlType::Vendor,
                    recipient: Recipient::Device,
                    request: req.code(),
                    value,
                    index,
                    length: len,
                },
                CTRL_TIMEOUT,
            )
            .wait();
        match r {
            Ok(data) => {
                self.trace.ctrl(
                    req.code(),
                    &format!("{req:?}"),
                    value,
                    index,
                    len as usize,
                    Some(data.len()),
                    "ok",
                );
                Ok(data)
            }
            Err(source) => {
                self.trace.ctrl(
                    req.code(),
                    &format!("{req:?}"),
                    value,
                    index,
                    len as usize,
                    None,
                    &format!("FAILED: {source}"),
                );
                Err(Error::Transfer { op: "control read", source })
            }
        }
    }

    /// A control-IN that this firmware may simply not have.
    ///
    /// Any error means "absent", never a fault: an older receiver stalls the
    /// later requests, a stalled control transfer self-clears on the next one,
    /// and which `TransferError` a given OS backend reports a control stall as
    /// is not something a driver should depend on. The exact error still goes
    /// in the trace, so the mapping becomes visible from a field report even
    /// though nothing branches on it.
    pub fn optional_in(&self, req: Request, value: u16, index: u16, len: u16) -> Option<Vec<u8>> {
        debug_assert!(req.is_optional(), "{req:?} is not optional; a failure there is real");
        self.control_in(req, value, index, len).ok()
    }

    /// A vendor control-OUT.
    pub fn control_out(&self, req: Request, value: u16, index: u16, data: &[u8]) -> Result<()> {
        let r = self
            .iface
            .control_out(
                ControlOut {
                    control_type: ControlType::Vendor,
                    recipient: Recipient::Device,
                    request: req.code(),
                    value,
                    index,
                    data,
                },
                CTRL_TIMEOUT,
            )
            .wait();
        match r {
            Ok(()) => {
                self.trace.ctrl(
                    req.code(),
                    &format!("{req:?}"),
                    value,
                    index,
                    data.len(),
                    None,
                    "ok",
                );
                Ok(())
            }
            Err(source) => {
                self.trace.ctrl(
                    req.code(),
                    &format!("{req:?}"),
                    value,
                    index,
                    data.len(),
                    None,
                    &format!("FAILED: {source}"),
                );
                Err(Error::Transfer { op: "control write", source })
            }
        }
    }

    /// A control-OUT that this firmware may simply not have. Returns whether it
    /// landed, for the settings panel to report.
    pub fn optional_out(&self, req: Request, value: u16, index: u16) -> bool {
        debug_assert!(req.is_optional(), "{req:?} is not optional; a failure there is real");
        self.control_out(req, value, index, &[]).is_ok()
    }

    /// A control-IN whose reply must be at least `len` bytes.
    pub fn control_in_exact(
        &self,
        req: Request,
        value: u16,
        index: u16,
        len: u16,
    ) -> Result<Vec<u8>> {
        let data = self.control_in(req, value, index, len)?;
        if data.len() < len as usize {
            return Err(Error::ShortRead {
                request: req.code(),
                want: len as usize,
                got: data.len(),
            });
        }
        Ok(data)
    }
}
