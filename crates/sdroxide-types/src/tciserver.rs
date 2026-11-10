use serde::{Deserialize, Serialize};

/// Built-in TCI server configuration (`tciserver.json`).
///
/// This is sdroxide *being* a TCI rig so third-party clients (WSJT-X's TCI rig
/// type, JTDX, MSHV, skimmers) can drive it. Not to be confused with
/// [`crate::TciConfig`], which is the client that connects sdroxide to somebody
/// else's rig.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TciServerConfig {
    pub enabled: bool,
    /// Interface to bind. `127.0.0.1` keeps the server on this machine;
    /// `0.0.0.0` exposes it to the whole network — and TCI has no
    /// authentication, so anyone who can reach the port can transmit.
    pub bind: String,
    /// Listen port. 50001 is the TCI default that clients expect, and also the
    /// port a local ExpertSDR3/Thetis would already hold.
    pub port: u16,
    /// Name reported in the `device:` line of the connect burst.
    pub device_name: String,
    /// Let clients key the transmitter (`trx:<rx>,true,tci;`). Off leaves
    /// control and the receive streams working but refuses every key request.
    pub allow_tx: bool,
    pub max_clients: usize,
}

impl Default for TciServerConfig {
    fn default() -> Self {
        TciServerConfig {
            enabled: true,
            bind: "127.0.0.1".into(),
            port: 50001,
            device_name: "sdroxide".into(),
            allow_tx: true,
            max_clients: 8,
        }
    }
}

impl TciServerConfig {
    /// Bind addresses offered in the UI.
    pub const BINDS: [&'static str; 2] = ["127.0.0.1", "0.0.0.0"];

    pub fn addr(&self) -> String {
        format!("{}:{}", self.bind, self.port)
    }

    /// True when this configuration hands the transmitter to the network at
    /// large, which deserves a warning in the UI.
    pub fn is_open_to_network(&self) -> bool {
        self.allow_tx && self.bind != "127.0.0.1" && self.bind != "localhost"
    }
}
