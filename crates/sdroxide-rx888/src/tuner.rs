//! The R828D behind the FX3's I2C passthrough.
//!
//! The driver is [`sdroxide_r82xx`], shared with the RTL-SDR backend — the same
//! chip family, and there is no reason for two transcriptions of it. All that
//! lives here is the transport, plus the one way this bus differs from the
//! demodulator's: the register travels in `wIndex` rather than at the head of
//! the payload, so a whole 30-byte initialisation array goes out in a single
//! transfer instead of five.
//!
//! # Verified on hardware
//!
//! On an RX888mk2 running firmware 2.3: the tuner answers at
//! [`sdroxide_r82xx::R828D_I2C_ADDR`] with `0x69`, confirming that the
//! addressing in [`crate::usb::UsbDev::i2c_write`] is right and that the
//! published API reference — which states the two fields the other way round —
//! is wrong. Initialisation, the 8 MHz IF filter and tuning all work over this
//! transport, and the resulting IF carrier is exactly 4.570 MHz.
//!
//! `examples/vhf_check.rs` re-runs that whole sequence.

use sdroxide_r82xx::{BusError, BusResult, R82XX_CHECK_VAL, TunerBus};

use crate::usb::UsbDev;

impl TunerBus for UsbDev {
    fn write_regs(&self, dev_addr: u8, reg: u8, data: &[u8]) -> BusResult<()> {
        self.i2c_write(dev_addr, reg, data).map_err(|e| BusError::new("write", e))
    }

    fn read_regs(&self, dev_addr: u8, reg: u8, out: &mut [u8]) -> BusResult<()> {
        self.i2c_read(dev_addr, reg, out).map_err(|e| BusError::new("read", e))
    }

    // `set_upconverter_gate` keeps its default no-op: this board has no
    // upconverter, and the HF it does reach is sampled directly rather than
    // mixed up to the tuner.
}

/// Whether an R82xx answers at `addr`.
///
/// Compares the **raw** byte, with no bit reversal — the same as the RTL-SDR's
/// probe, and for the same reason its comment gives: reversing in both the
/// probe and the driver's read path makes every receiver look like it has no
/// tuner. The part returns `0x69` on the wire, and it is `R82xx::read` that
/// reverses so the driver sees registers the right way up.
///
/// Verified on hardware: this receiver's R828D answers `0x69` at 0x74.
pub fn present(usb: &UsbDev, addr: u8) -> bool {
    let mut b = [0u8; 1];
    usb.i2c_read(addr, 0, &mut b).is_ok() && b[0] == R82XX_CHECK_VAL
}
