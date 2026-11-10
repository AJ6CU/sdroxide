//! Enumeration, opening, and the vendor requests as control transfers.
//!
//! # The one invariant in this crate
//!
//! [`UsbDev`] is deliberately **not `Clone`**, and every control transfer runs
//! on the stream thread and nowhere else. `nusb`'s `Interface` is `Send + Sync`,
//! so a second thread poking the receiver would compile and would be wrong: a
//! rate change is a stop, a reprogram and a restart, and a gain change is three
//! writes that have to land together or the front end sits in a state neither
//! curve describes.
//!
//! # Firmware differences
//!
//! Two requests postdate the product and older firmware stalls them:
//! `GetSamplerates` and `SetPacking`. Nothing here version-sniffs; the requests
//! are simply attempted, and [`UsbDev::optional_in`] turns any failure into
//! "this firmware does not have it" with a documented fallback and a trace
//! line. Version-sniffing would need a table of which build gained what, which
//! is exactly the sort of table that goes stale silently.

use nusb::MaybeFuture;
use nusb::transfer::{ControlIn, ControlOut, ControlType, Recipient};
use sdroxide_types::AirspyDevice;

use crate::error::{Error, Result};
use crate::protocol::{
    ALT_SETTING, BULK_EP, CONFIGURATION, CTRL_TIMEOUT, INTERFACE, PID, Request, VID,
    looks_like_hydrasdr, serial_matches,
};
use crate::trace::Trace;

/// Whether a device on this USB id is one of ours.
///
/// The id is not exclusively Airspy's: HydraSDR's prototype RFOne boards were
/// flashed with it, and that radio needs the other driver. See
/// [`looks_like_hydrasdr`].
fn is_ours(vid: u16, pid: u16, product: Option<&str>, serial: Option<&str>) -> bool {
    vid == VID && pid == PID && !looks_like_hydrasdr(product, serial)
}

/// Enumerate the Airspy R2 / Mini receivers on the USB bus.
///
/// Non-invasive: no device is opened. The strings come from sysfs on Linux, the
/// registry on Windows and IOKit on macOS, so this is safe to call at any time
/// — including while another receiver is streaming, which is what makes the
/// settings dialog's Rescan button harmless.
pub fn list() -> Vec<AirspyDevice> {
    let devices = match nusb::list_devices().wait() {
        Ok(d) => d,
        Err(e) => {
            tracing::warn!("USB enumeration failed: {e}");
            return Vec::new();
        }
    };
    devices
        .filter(|d| is_ours(d.vendor_id(), d.product_id(), d.product_string(), d.serial_number()))
        .map(|d| AirspyDevice {
            serial: d.serial_number().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()),
            // An R2 and a Mini share this product string as well as their USB
            // id; the only thing that separates them is the rate list, which
            // needs the device open. The list does not pretend otherwise.
            name: match d.product_string() {
                Some(p) if !p.is_empty() => p.to_string(),
                _ => "Airspy".to_string(),
            },
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
    /// Open the receiver whose serial ends with `serial`, or the first one
    /// found when it is empty.
    pub fn open(serial: &str, trace: &Trace) -> Result<UsbDev> {
        let want = serial.trim();
        let devices = nusb::list_devices().wait()?;
        let mut candidates = devices.filter(|d| {
            is_ours(d.vendor_id(), d.product_id(), d.product_string(), d.serial_number())
        });

        let info = if want.is_empty() {
            candidates
                .next()
                .ok_or_else(|| Error::NotFound("no Airspy R2 or Mini found on USB".to_string()))?
        } else {
            candidates.find(|d| serial_matches(want, d.serial_number())).ok_or_else(|| {
                Error::NotFound(format!(
                    "no Airspy whose serial ends with {want:?} is plugged in — \
                     pick another receiver in Settings → Radio, or clear the \
                     serial to use the first one found"
                ))
            })?
        };

        let serial = info.serial_number().map(|s| s.trim().to_string()).filter(|s| !s.is_empty());
        let label = match info.product_string() {
            Some(p) if !p.is_empty() => p.to_string(),
            _ => "Airspy".to_string(),
        };
        let speed = info.speed();

        let device = info.open().wait().map_err(|e| Error::from_open(e, &label))?;

        // Only set the configuration if it is not already the one we want. On
        // Linux `set_configuration` can re-enumerate the device — which would
        // invalidate the handle we are holding — and on Windows it is not
        // supported at all. Every Airspy enumerates configured, so this is
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

        // No `SET_INTERFACE` here, deliberately. This receiver declares one
        // alternate setting and the bulk endpoints are in it, so claiming the
        // interface is already all of it — see [`ALT_SETTING`], and note that
        // asking for one that does not exist is how this driver used to fail to
        // open at all on macOS.

        let dev = UsbDev { iface, label, serial, speed, trace: trace.clone() };
        dev.check_bulk_endpoint()?;

        tracing::info!(
            "opened {} (usb {VID:04x}:{PID:04x}, serial {}, {})",
            dev.label,
            dev.serial.as_deref().unwrap_or("none"),
            dev.speed_name(),
        );
        trace.note(format!(
            "claimed interface {INTERFACE} (alt {ALT_SETTING}, the only one) on {} \
             (serial {}, {})",
            dev.label,
            dev.serial.as_deref().unwrap_or("none"),
            dev.speed_name(),
        ));
        Ok(dev)
    }

    /// Fail early, and name the real layout when we do.
    ///
    /// `Interface::endpoint` resolves an address against the *current*
    /// alternate setting. If a firmware ever put the sample endpoint somewhere
    /// else, the stream would fail with a bare "not found" that says nothing;
    /// listing what the descriptor does have turns a dead end into a usable bug
    /// report.
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
             it offers [{}]. Please report this with the model and firmware version.",
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

    /// Whether the link can carry the top rate.
    ///
    /// The R2's own top rate is 10 Msps complex, which is 20 Msps real — 40 MB/s
    /// unpacked, 30 MB/s packed. This is a USB 2.0 device, so it is always on a
    /// high-speed link and 30 MB/s is near what such a link really sustains;
    /// packing is not a nicety here, which is why the driver turns it on
    /// wherever the firmware has it.
    pub fn is_high_speed_or_better(&self) -> bool {
        matches!(
            self.speed,
            Some(nusb::Speed::High) | Some(nusb::Speed::Super) | Some(nusb::Speed::SuperPlus)
        )
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
    /// matched.
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
    /// and which `TransferError` a given OS backend reports a stall as is not
    /// something a driver should depend on. The exact error still goes in the
    /// trace, so the mapping becomes visible from a field report even though
    /// nothing branches on it.
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

    /// A control-OUT with no payload, for the requests the firmware answers
    /// with a bare acknowledgement.
    pub fn out(&self, req: Request, value: u16, index: u16) -> Result<()> {
        self.control_out(req, value, index, &[])
    }

    /// A *setting* whose reply is a one-byte return code.
    ///
    /// Most of this protocol's setters are control **reads** on the wire, not
    /// writes: the firmware acts in the setup stage and then queues a byte on
    /// the IN endpoint for the host to collect, and libairspy duly issues them
    /// with `LIBUSB_ENDPOINT_IN` and a one-byte buffer. Sending such a request
    /// as an OUT does not merely lose the return code — the queued byte arrives
    /// where a zero-length status stage was expected, which fails the transfer
    /// and leaves the byte to be read as the front of the *next* control reply.
    /// See [`Request`] for which requests are which.
    ///
    /// The byte itself is the firmware's "I handled it", so it is checked for
    /// length and then discarded, exactly as libairspy does.
    pub fn set(&self, req: Request, value: u16, index: u16) -> Result<()> {
        self.control_in_exact(req, value, index, 1).map(|_| ())
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
