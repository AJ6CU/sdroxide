//! Weather-satellite image decoders: the threading wrapper and the DSP behind
//! the WXSAT view. APT (NOAA-15/18/19, 137 MHz, FM with a 2400 Hz AM
//! subcarrier) lives in [`apt`]; Meteor LRPT will follow in its own module.

pub mod apt;
pub mod lane;
pub mod lrpt;

pub use apt::{AptDecoder, AptLine, LINE_PIXELS};
pub use lane::{WxsatController, WxsatLane};

/// The rate of the complex baseband lane the engine feeds a decoder, Hz.
/// A NOAA signal is about 40 kHz wide (±17 kHz deviation plus the subcarrier).
pub const WXSAT_LANE_RATE_HZ: f64 = 48_000.0;
