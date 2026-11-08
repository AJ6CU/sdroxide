//! Meshtastic receive: what the operator asks the engine for and what it
//! reports back. Pure data + serde, shared by the native engine and the UI
//! (native + WASM). The demodulation and decoding live in the native
//! `sdroxide-meshtastic` crate, the same split as [`crate::sonde`].
//!
//! Like the sonde and HFDL lanes, this is not a radio mode: it rides its own
//! downconversion of the raw I/Q and the radio keeps its mode. The panel moves
//! the dial with the ordinary tune command, so the receive band list decides
//! whether it may; this lane never touches that list.

use serde::{Deserialize, Serialize};

/// The default channel: EU 868 MHz, LongFast, the one slot of the 869.4 to
/// 869.65 MHz sub-band.
pub const MESH_DEFAULT_HZ: f64 = 869_525_000.0;

/// The default region and preset, by the names the protocol gives them.
pub const MESH_DEFAULT_REGION: &str = "EU_868";
pub const MESH_DEFAULT_PRESET: &str = "LongFast";

/// Lowest and highest channel frequency the lane accepts (the 433, 868 and
/// 915 MHz regions, with a margin).
pub const MESH_BAND_HZ: (f64, f64) = (430_000_000.0, 930_000_000.0);

/// How many entries the packet log keeps (the most recent).
pub const MESH_LOG_DEPTH: usize = 100;

/// How many nodes the table keeps (the most recently heard).
pub const MESH_MAX_NODES: usize = 64;

/// The longest key list, name or region accepted, in bytes.
const MESH_TEXT_MAX: usize = 4096;

/// Where the radio was before the lane took it over. Kept on the station in
/// `meshtastic.json`, so the dial and the mode come back when the lane is
/// switched off or the program quits, whichever client turned it on.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MeshtasticRestore {
    /// The active VFO's frequency, Hz.
    pub dial_hz: f64,
    /// The first receiver's mode.
    pub mode: crate::Mode,
}

/// What the operator asks the Meshtastic receiver to do.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshtasticSettings {
    /// Whether the receiver runs at all. Off by default: it costs a
    /// downconverter and a worker thread.
    pub enabled: bool,
    /// The region, by its protocol name (`EU_868`, `US`, ...).
    pub region: String,
    /// The modem preset, by its protocol name (`LongFast`, ...).
    pub preset: String,
    /// The channel the lane centres on, in Hz.
    pub frequency_hz: f64,
    /// Extra channel keys, comma separated: `Name=key`, where the key is
    /// `hex:...`, `base64:...`, a bare base64 key or `simple1`..`simple10`.
    /// The default channel key is always tried.
    pub keys: String,
    /// Where the radio was before the lane tuned it; `None` while the lane has
    /// not moved it.
    pub restore: Option<MeshtasticRestore>,
    /// This station's node number, the sender of what it transmits. `0` until
    /// the first transmission, when the engine draws one and keeps it here
    /// (a client's value is replaced by the station's). Appended last, with a
    /// default, so a `meshtastic.json` written before it still loads.
    #[serde(default)]
    pub node_id: u32,
}

impl Default for MeshtasticSettings {
    fn default() -> Self {
        Self {
            enabled: false,
            region: MESH_DEFAULT_REGION.to_string(),
            preset: MESH_DEFAULT_PRESET.to_string(),
            frequency_hz: MESH_DEFAULT_HZ,
            keys: String::new(),
            restore: None,
            node_id: 0,
        }
    }
}

/// A text message the operator asks the station to send on the channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MeshtasticSend {
    /// The words. Empty, or longer than [`MESH_TX_TEXT_MAX`] bytes, is refused.
    pub text: String,
    /// The destination node, or `0xFFFFFFFF` for everyone.
    pub to: u32,
}

/// The longest text one frame carries, in bytes.
pub const MESH_TX_TEXT_MAX: usize = 200;

/// Why the station did not transmit a message, for the panel to say in the
/// operator's language.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum MeshTxRefusal {
    /// The transmit band list refuses the channel's frequency (not ticked, not
    /// confirmed, or "Other"): the same lock every transmission meets.
    Band(crate::TxBlock),
    /// The duty cycle of the region leaves no room yet; try again in this many
    /// seconds.
    DutyCycle(u32),
    /// One frame is longer than the whole duty-cycle budget.
    TooLong,
    /// The radio is not on the channel: switch the receiver on with "Listen
    /// here" first, so the dial is on it.
    OffChannel,
    /// This radio cannot send raw I/Q (a rig that takes audio, or a receive-only
    /// device).
    NoTransmitter,
    /// The text is empty or too long, or the key cannot encrypt it.
    BadText,
    /// A transmission is already in progress.
    Busy,
    /// The key-down itself was refused by the transmit rails (a receive-only
    /// mode, the SWR latch, the station interlock...); the transmit message
    /// that came with it says which.
    Rails,
}

fn clip(s: &mut String, max: usize) {
    if s.len() > max {
        let mut end = max;
        while !s.is_char_boundary(end) {
            end -= 1;
        }
        s.truncate(end);
    }
}

impl MeshtasticSettings {
    /// The settings with the frequency inside the supported bands and the text
    /// bounded, applied where they arrive (a config file an operator may have
    /// edited, a remote client) rather than where they are used.
    pub fn sane(mut self) -> Self {
        self.frequency_hz = if self.frequency_hz.is_finite() {
            self.frequency_hz.clamp(MESH_BAND_HZ.0, MESH_BAND_HZ.1)
        } else {
            MESH_DEFAULT_HZ
        };
        clip(&mut self.region, 32);
        clip(&mut self.preset, 32);
        clip(&mut self.keys, MESH_TEXT_MAX);
        if let Some(r) = self.restore
            && !r.dial_hz.is_finite()
        {
            self.restore = None;
        }
        self
    }
}

/// Whether a sample window of `rate_hz` centred on `center_hz` holds the whole
/// channel of `bandwidth_hz` centred on `channel_hz`. The lane mixes from the
/// window, so a channel that sticks out of it cannot be decoded.
pub fn meshtastic_window_covers(
    center_hz: f64,
    rate_hz: f64,
    channel_hz: f64,
    bandwidth_hz: f64,
) -> bool {
    let half = rate_hz.abs() / 2.0;
    let edge = bandwidth_hz / 2.0;
    channel_hz - edge >= center_hz - half && channel_hz + edge <= center_hz + half
}

/// One node of the mesh, as far as it has been heard.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MeshNode {
    pub id: u32,
    pub long_name: String,
    pub short_name: String,
    pub hw_model: u32,
    pub role: u32,
    /// Unix second of the last packet.
    pub last_heard: i64,
    pub snr_db: f32,
    pub packets: u32,
    /// Relays between the node and us, from the header; `None` when the
    /// sender's firmware does not say.
    pub hops: Option<u8>,
    pub lat_deg: Option<f64>,
    pub lon_deg: Option<f64>,
    pub altitude_m: Option<i32>,
    pub battery_pct: Option<u32>,
    pub voltage: Option<f32>,
}

/// One packet heard on the air.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MeshEntry {
    /// Unix second it was heard.
    pub time: i64,
    pub from: u32,
    pub to: u32,
    pub packet_id: u32,
    /// The application port number; the name is the protocol's.
    pub port: u32,
    /// The channel it was read on; empty when no key opened it.
    pub channel: String,
    pub snr_db: f32,
    pub hops: Option<u8>,
    /// No key we hold opened it: only the header is known.
    pub encrypted: bool,
    pub via_mqtt: bool,
    /// The words, for a text message.
    pub text: Option<String>,
}

/// What the receiver reports: level, counters, the nodes and the packet log.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MeshtasticStatus {
    pub running: bool,
    /// The channel lies inside the sample window the radio is capturing.
    pub in_window: bool,
    /// Why the receive band list does not let the radio listen at the channel;
    /// the lane stays idle while this is set.
    pub refused: Option<crate::RxBlock>,
    pub level_dbfs: f32,
    /// Preambles the demodulator locked on.
    pub preambles: u64,
    /// Frames delivered with a good CRC.
    pub frames: u64,
    /// Frames whose CRC failed.
    pub crc_bad: u64,
    /// Newest first.
    pub nodes: Vec<MeshNode>,
    /// Newest first.
    pub log: Vec<MeshEntry>,
    /// Why the last attempt to transmit was refused; `None` after one went out
    /// or before any was tried. Appended with the transmit side.
    pub tx_refused: Option<MeshTxRefusal>,
    /// A transmission is in progress.
    pub tx_busy: bool,
    /// Messages sent since the receiver was switched on.
    pub tx_sent: u32,
    /// Air time used in the last hour, in milliseconds.
    pub tx_used_ms: u32,
    /// The hour's air-time budget in milliseconds; `None` where the region has
    /// no limit here.
    pub tx_budget_ms: Option<u32>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_defaults_are_eu_868_long_fast() {
        let s = MeshtasticSettings::default();
        assert!(!s.enabled);
        assert_eq!(s.region, "EU_868");
        assert_eq!(s.preset, "LongFast");
        assert_eq!(s.frequency_hz, 869_525_000.0);
        assert_eq!(s.restore, None);
    }

    #[test]
    fn sane_pulls_a_wild_frequency_into_the_bands() {
        let s = MeshtasticSettings { frequency_hz: 14_074_000.0, ..Default::default() }.sane();
        assert_eq!(s.frequency_hz, MESH_BAND_HZ.0);
        let s = MeshtasticSettings { frequency_hz: f64::NAN, ..Default::default() }.sane();
        assert_eq!(s.frequency_hz, MESH_DEFAULT_HZ);
        let s = MeshtasticSettings { frequency_hz: 906_875_000.0, ..Default::default() }.sane();
        assert_eq!(s.frequency_hz, 906_875_000.0);
    }

    #[test]
    fn sane_bounds_the_text_on_a_character_boundary() {
        let s = MeshtasticSettings { keys: "é".repeat(MESH_TEXT_MAX), ..Default::default() }.sane();
        assert!(s.keys.len() <= MESH_TEXT_MAX);
        assert!(s.keys.chars().all(|c| c == 'é'));
    }

    #[test]
    fn the_window_must_hold_the_whole_channel() {
        // A 2.4 MS/s window on 869.5 MHz holds a 250 kHz channel at 869.525.
        assert!(meshtastic_window_covers(869_500_000.0, 2_400_000.0, 869_525_000.0, 250_000.0));
        // The same window does not hold it when its edge cuts the channel.
        assert!(!meshtastic_window_covers(870_900_000.0, 2_400_000.0, 869_525_000.0, 250_000.0));
        assert!(!meshtastic_window_covers(868_400_000.0, 2_000_000.0, 869_525_000.0, 250_000.0));
    }
}
