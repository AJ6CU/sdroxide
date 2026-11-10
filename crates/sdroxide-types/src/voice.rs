//! Voice keyer: ten recorded messages the operator can transmit on demand.
//!
//! The recordings themselves live engine-side (it owns the microphone and the
//! transmit chain); everything here is the small status snapshot the UI, the
//! remote client and the bindings editor need.

use serde::{Deserialize, Serialize};

/// How many voice messages can be stored. Fixed so a binding — a numpad key, a
/// MIDI pad, a `send_voice_mem` from rigctld — always means the same slot.
pub const VOICE_SLOTS: usize = 10;

/// Longest single recording, in seconds. A voice keyer message is a call or a
/// short over; the cap keeps a forgotten recording from filling the disk.
pub const VOICE_MAX_LEN_S: f32 = 120.0;

/// One slot as the UI shows it. Empty slots have `len_s == 0`.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(default)]
pub struct VoiceSlotInfo {
    /// Operator-chosen label ("CQ", "QRZ?", "73"), empty when never set.
    pub name: String,
    /// Length of the stored recording in seconds; 0 when the slot is empty.
    pub len_s: f32,
}

impl VoiceSlotInfo {
    pub fn is_empty(&self) -> bool {
        self.len_s <= 0.0
    }
}

/// The voice keyer's whole observable state, emitted on every change and while
/// recording/playing so the UI can show a running position.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct VoiceStatus {
    /// Always [`VOICE_SLOTS`] entries, index = slot number.
    pub slots: Vec<VoiceSlotInfo>,
    /// Slot currently being recorded into.
    pub recording: Option<u8>,
    /// Slot currently being transmitted.
    pub playing: Option<u8>,
    /// Slot currently being monitored through the speakers (not on the air).
    pub previewing: Option<u8>,
    /// Seconds into whichever of the three is running.
    pub position_s: f32,
    /// Cap enforced on a recording, so the UI can show how much is left.
    pub max_len_s: f32,
}

impl Default for VoiceStatus {
    fn default() -> Self {
        VoiceStatus {
            slots: vec![VoiceSlotInfo::default(); VOICE_SLOTS],
            recording: None,
            playing: None,
            previewing: None,
            position_s: 0.0,
            max_len_s: VOICE_MAX_LEN_S,
        }
    }
}

impl VoiceStatus {
    pub fn slot(&self, i: usize) -> VoiceSlotInfo {
        self.slots.get(i).cloned().unwrap_or_default()
    }

    /// Whether anything is being recorded, transmitted or monitored right now.
    pub fn busy(&self) -> bool {
        self.recording.is_some() || self.playing.is_some() || self.previewing.is_some()
    }
}

/// The label a slot shows: the operator's name, or "Slot N" while it has none.
pub fn slot_label(i: usize, name: &str) -> String {
    if name.trim().is_empty() { format!("Slot {}", i + 1) } else { name.trim().to_string() }
}
