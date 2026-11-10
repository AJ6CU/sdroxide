//! The level of the main receiver's audio, for the meters of a client that has
//! no other way to see it.
//!
//! A native screen could measure what it plays; a browser tab or a remote
//! screen is sent the audio late and in pieces, and a bar that moves to the
//! beat has to move with what the speakers are doing *now*. So the station
//! measures it where the sound is handed to the output and sends the result
//! about thirty times a second. Only the DAB status carried a level before.

use serde::{Deserialize, Serialize};

/// How many bands [`AudioLevel::bands`] holds: log-spaced from
/// [`AUDIO_BANDS_LO_HZ`] to [`AUDIO_BANDS_HI_HZ`].
pub const AUDIO_BANDS: usize = 48;
/// Lower edge of the first band, in hertz.
pub const AUDIO_BANDS_LO_HZ: f32 = 50.0;
/// Upper edge of the last band, in hertz. Above the 15 kHz of a broadcast FM
/// programme, so the top bar is the pilot's neighbourhood and not the signal.
pub const AUDIO_BANDS_HI_HZ: f32 = 16_000.0;
/// How long one [`crate::Command::WantAudioLevel`] keeps the stream on, in
/// seconds. A panel renews it about once a second, so a lost command or two
/// does not blink the bars.
pub const AUDIO_LEVEL_LEASE_SECS: f64 = 3.0;
/// What a band of `0` stands for, in dB below full scale.
pub const AUDIO_BAND_FLOOR_DB: f32 = -80.0;

/// One reading of the speaker feed, left and right.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioLevel {
    /// RMS of the last stretch, in dB below full scale, as it goes to the
    /// speakers (after the volume). A mono signal repeats the left value on the
    /// right.
    pub rms_db: [f32; 2],
    /// [`AUDIO_BANDS`] bars, `0` = [`AUDIO_BAND_FLOOR_DB`] or below, `255` =
    /// full scale. Empty when the station sent only the level.
    pub bands: Vec<u8>,
}

impl AudioLevel {
    /// Silence: what a client draws when the station has stopped sending.
    pub fn silent() -> Self {
        Self { rms_db: [-120.0; 2], bands: Vec::new() }
    }
}

/// Map a band power in dB below full scale onto the byte [`AudioLevel::bands`]
/// carries.
pub fn band_to_byte(db: f32) -> u8 {
    if !db.is_finite() {
        return 0;
    }
    let t = (db - AUDIO_BAND_FLOOR_DB) / -AUDIO_BAND_FLOOR_DB;
    (t.clamp(0.0, 1.0) * 255.0).round() as u8
}

/// The inverse of [`band_to_byte`], for a client drawing a bar.
pub fn byte_to_unit(b: u8) -> f32 {
    f32::from(b) / 255.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bytes_cover_the_range_and_clamp() {
        assert_eq!(band_to_byte(0.0), 255);
        assert_eq!(band_to_byte(6.0), 255);
        assert_eq!(band_to_byte(AUDIO_BAND_FLOOR_DB), 0);
        assert_eq!(band_to_byte(-200.0), 0);
        assert_eq!(band_to_byte(f32::NEG_INFINITY), 0);
        assert_eq!(band_to_byte(f32::NAN), 0);
        let mid = band_to_byte(-40.0);
        assert!((126..=129).contains(&mid), "{mid}");
        assert!((byte_to_unit(255) - 1.0).abs() < 1e-6);
    }
}
