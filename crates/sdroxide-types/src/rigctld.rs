use serde::{Deserialize, Serialize};

/// Built-in Hamlib rigctld server configuration (`rigctld.json`).
///
/// This is sdroxide *being* a rig for the wider ham software ecosystem: WSJT-X,
/// fldigi, JS8Call, N1MM, Log4OM, GPredict and CQRLOG all speak Hamlib's
/// "NET rigctl" (rig model 2) over TCP. It complements
/// [`crate::TciServerConfig`], which serves the smaller set of TCI-speaking
/// programs — and unlike TCI it carries no audio or IQ, only control.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RigctldConfig {
    /// Off by default. Port 4532 is commonly already held by a real `rigctld`,
    /// and the protocol has no authentication whatsoever — turning this on is
    /// a decision the operator should make deliberately.
    pub enabled: bool,
    /// Interface to bind. `127.0.0.1` keeps the server on this machine;
    /// `0.0.0.0` exposes it to the whole network.
    pub bind: String,
    /// Listen port. 4532 is the rigctld default every client assumes.
    pub port: u16,
    /// Let clients key the transmitter. Off refuses every `set_ptt` *and*
    /// stops advertising any transmit range, so Hamlib itself declines to key
    /// before it even asks.
    pub allow_tx: bool,
    pub max_clients: usize,
    /// Name reported by `get_info` and `dump_caps`.
    pub rig_name: String,
}

impl Default for RigctldConfig {
    fn default() -> Self {
        RigctldConfig {
            enabled: false,
            bind: "127.0.0.1".into(),
            port: 4532,
            allow_tx: true,
            max_clients: 4,
            rig_name: "sdroxide".into(),
        }
    }
}

impl RigctldConfig {
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
