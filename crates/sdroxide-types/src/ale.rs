//! Interface facts for the ALE decoder: what the panel and the wire need to
//! talk about a decoded word. The modem and FEC live in `sdroxide-dsp`.

use serde::{Deserialize, Serialize};

/// The most received words kept in the rolling list.
pub const ALE_WORD_MAX: usize = 300;

/// One decoded ALE word: its type (`TO`, `FROM`, …) and 3-character address.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AleMessage {
    /// When the word was decoded, unix seconds.
    pub at_unix: i64,
    /// The word type, e.g. `TO` or `FROM`.
    pub kind: String,
    /// The address, e.g. `RAK`.
    pub address: String,
}

/// ALE's live state, for the panel and the wire.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AleStatus {
    /// Smoothed audio level, for the meter.
    pub level: f32,
    /// Words decoded, newest last.
    pub messages: Vec<AleMessage>,
    /// Total decoded since entering the mode.
    pub total: u64,
}

impl Default for AleStatus {
    fn default() -> Self {
        AleStatus { level: 0.0, messages: Vec::new(), total: 0 }
    }
}
