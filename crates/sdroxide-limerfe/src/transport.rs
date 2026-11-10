//! The seam the board's two control paths meet at.
//!
//! Semantic methods rather than raw frames, because only one of the two paths
//! has frames: the serial transport encodes them itself, while the board path
//! hands the same request to LimeSuite's `RFE_*` calls and never sees a byte.
//! A trait in terms of buffers would fit one and be a lie for the other.
//!
//! [`RfeTransport::round_trip`] is the load-bearing method. One transport costs
//! about 40 ms per exchange and the other the better part of a second, and
//! everything above this trait — the rate limit, whether an over can afford a
//! mode change — is derived from that number rather than assumed.

use std::time::Duration;

use crate::error::Result;
use crate::frame::{RfeInfo, RfeState};
use sdroxide_types::RfeMode;

/// One LimeRFE, reached either over its own USB serial port or through the SDR
/// board it is bolted to.
///
/// Every method is one round trip and every one blocks, which is why the whole
/// trait lives behind a thread — see [`crate::spawn`].
pub trait RfeTransport: Send {
    /// Firmware and hardware version. Also the liveness probe: a board that
    /// answers this is present, and one that does not is not.
    fn info(&mut self) -> Result<RfeInfo>;

    /// Channels, ports, mode, notch and attenuation, in one transaction.
    fn configure(&mut self, state: RfeState) -> Result<()>;

    /// The relays only — the one that has to happen at key-down, and the reason
    /// it is not just a `configure` with a different mode.
    fn set_mode(&mut self, mode: RfeMode) -> Result<()>;

    fn set_fan(&mut self, on: bool) -> Result<()>;

    /// Roughly what one exchange costs on this link. Measured where it can be,
    /// estimated where it cannot; either way it is what the rate limit is
    /// derived from.
    fn round_trip(&self) -> Duration;

    /// One line naming this link, for logs and the status area.
    fn describe(&self) -> String;
}
