//! A SpyServer client: the protocol Airspy's `spyserver` speaks, and with it
//! every Airspy, Airspy HF+ and RTL-SDR published that way.
//!
//! Two shapes of receiver come out of the same connection, and which one is
//! asked for is the interface the operator picked rather than anything this
//! crate decides:
//!
//! * **Wideband** — one I/Q stream at a decimation stage of the server's
//!   ladder, exactly like any other SDR here.
//! * **VFO + FFT** — a *narrow* I/Q stream that follows the dial, plus a
//!   low-rate FFT of the whole band. A couple of kilobytes a frame buys a band
//!   view that would otherwise cost megabits of I/Q, which is what makes a
//!   remote receiver usable over WiFi or a cellular modem.
//!
//! The FFT lane is offered in both, because it is nearly free and the strip it
//! feeds shows what the I/Q cannot.
//!
//! # What this crate is not
//!
//! It does not implement the protocol's audio (`AF`) streams — this program
//! demodulates its own — nor its 24-bit I/Q or 4-bit differential FFT
//! encodings, neither of which is documented anywhere or implemented by any
//! open client. Each is refused by name rather than guessed at.

pub mod decode;
mod error;
mod handle;
mod net;
pub mod proto;

use std::time::Duration;

use sdroxide_types::SpyServerConfig;

pub use error::{Error, Result};
pub use handle::{FftFrame, SpyServerHandle};
pub use net::CLIENT_NAME;
pub use proto::{ClientSync, DeviceInfo, DeviceKind};

impl SpyServerHandle {
    /// Connect to a SpyServer and start a wideband I/Q stream.
    ///
    /// Blocks until the connection is up and the far end configured, or has
    /// failed, so a wrong address or a server that is not running comes back
    /// as an ordinary error rather than as a stream that never starts.
    pub fn connect_wideband(cfg: &SpyServerConfig, center_hz: f64) -> Result<SpyServerHandle> {
        net::spawn(cfg, center_hz, false)
    }

    /// The same, in the low-bandwidth shape: a narrow I/Q stream that follows
    /// the dial, and the FFT lane as the only band view.
    pub fn connect_vfo(cfg: &SpyServerConfig, center_hz: f64) -> Result<SpyServerHandle> {
        net::spawn(cfg, center_hz, true)
    }
}

/// Connect, read what the server says about itself, and disconnect.
///
/// For the settings tab's Test button, and worth having here rather than in
/// the UI: this protocol *answers*, so a test can report the receiver on the
/// far end, the rates it offers and whether this client would be allowed to
/// tune it — which is most of what an operator wants to know before pressing
/// Apply.
pub fn probe(address: &str, timeout: Duration) -> Result<(DeviceInfo, bool)> {
    let cfg = SpyServerConfig {
        address: address.to_string(),
        // Nothing is streamed, so neither of these reaches the wire; they only
        // have to be values the planner accepts.
        fft_enabled: false,
        ..SpyServerConfig::default()
    };
    net::probe(&cfg, timeout)
}

/// [`probe`] as one line for the settings tab, or the reason it failed.
pub fn test_connection(address: &str, timeout: Duration) -> std::result::Result<String, String> {
    match probe(address, timeout) {
        Ok((info, can_control)) => {
            let rates = info.iq_rates(false);
            let widest = rates.first().map(|&(_, r)| r).unwrap_or(0.0);
            let narrowest = rates.last().map(|&(_, r)| r).unwrap_or(0.0);
            Ok(format!(
                "{} — I/Q from {:.3} ksps to {:.3} Msps; {}",
                info.describe(),
                narrowest / 1e3,
                widest / 1e6,
                if can_control {
                    "this client may tune it"
                } else {
                    "another client owns it, so tuning is limited to the slice it is receiving"
                },
            ))
        }
        Err(e) => Err(e.to_string()),
    }
}
