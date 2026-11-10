//! RIFP (Radio Image Framing Protocol) vocabulary shared by the engine, the
//! wire protocol, and the UI — draft-dulaunoy-rifp-00.
//!
//! RIFP is a packetised image-transfer protocol: a JSON manifest describes the
//! encoded object, numbered DATA frames carry it, and an END frame repeats the
//! integrity values. Only the identity of the profiles/encodings and the status
//! a panel renders live here; the framing, the modem, and the object codecs are
//! native-side (`sdroxide-dsp::rifp`, `sdroxide-digi::rifp_controller`).

use serde::{Deserialize, Serialize};

/// The RIFP calling frequency (Hz). RIFP itself assigns no frequency; 433.92 MHz
/// is the deployment example the draft names and where the reference
/// implementation defaults, so it is what the panel offers to jump to.
pub const RIFP_CALLING_HZ: f64 = 433_920_000.0;

/// A named RIFP radio profile: the modulation and synchronisation parameters
/// that carry the frames.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum RifpProfile {
    /// `rifp-cpfsk-4800` — the draft's initial profile. Continuous-phase binary
    /// FSK, 4800 baud, ±4000 Hz deviation, 25 kHz recommended occupied
    /// bandwidth.
    #[default]
    Cpfsk4800,
}

impl RifpProfile {
    pub const ALL: [RifpProfile; 1] = [RifpProfile::Cpfsk4800];

    /// The registered profile name, as it travels in the manifest and the
    /// Radio Profile header TLV.
    pub fn name(self) -> &'static str {
        match self {
            RifpProfile::Cpfsk4800 => "rifp-cpfsk-4800",
        }
    }

    /// Short label for a button.
    pub fn label(self) -> &'static str {
        match self {
            RifpProfile::Cpfsk4800 => "CPFSK 4800",
        }
    }

    /// Symbols (= bits, the profile is binary) per second.
    pub fn symbol_rate(self) -> f64 {
        match self {
            RifpProfile::Cpfsk4800 => 4800.0,
        }
    }

    /// Peak frequency deviation in Hz; mark is `+dev`, space is `-dev`.
    pub fn deviation_hz(self) -> f64 {
        match self {
            RifpProfile::Cpfsk4800 => 4000.0,
        }
    }

    /// Recommended occupied channel bandwidth in Hz. Wider than a narrow-band
    /// segment allows — see [`RifpProfile::wide_segments`].
    pub fn bandwidth_hz(self) -> f64 {
        match self {
            RifpProfile::Cpfsk4800 => 25_000.0,
        }
    }

    /// Payload octets per DATA frame the profile recommends.
    pub fn chunk_size(self) -> u16 {
        match self {
            RifpProfile::Cpfsk4800 => 192,
        }
    }

    /// True where this profile is narrow enough to sit in an ordinary
    /// narrow-band segment. No profile is yet: 25 kHz is far past the few kHz a
    /// narrow-band allocation allows.
    pub fn needs_wide_segment(self) -> bool {
        self.bandwidth_hz() > 12_000.0
    }

    /// The segments where a channel this wide is normally permitted in the
    /// station's configured region, as `(low, high, label)` in Hz.
    pub fn wide_segments(self) -> &'static [(f64, f64, &'static str)] {
        self.wide_segments_in(crate::region())
    }

    /// The same for `region`.
    ///
    /// RIFP assigns no frequency and sdroxide will transmit it wherever it is
    /// tuned; this is only what the panel uses to decide whether to say
    /// something. It is deliberately a short list of the places wideband or FM
    /// operation is generally allowed — the amateur allocations differ by
    /// country and none of this is a substitute for the operator's own licence
    /// conditions, which may well be narrower than 25 kHz even inside these.
    ///
    /// The regional differences are the ones the band edges already carry:
    /// 6 m runs to 54 MHz and 70 cm to 450 MHz outside Region 1, and Region 2's
    /// 2 m FM simplex lives above 146 MHz rather than in the 144.5 all-modes
    /// segment Region 1 uses.
    pub fn wide_segments_in(self, region: crate::Region) -> &'static [(f64, f64, &'static str)] {
        match (self, region) {
            (RifpProfile::Cpfsk4800, crate::Region::R1) => &[
                // 10 m FM.
                (29_510_000.0, 29_700_000.0, "10 m FM (29.510–29.700)"),
                // 6 m all-modes: the segment above the narrow-band part.
                (50_500_000.0, 52_000_000.0, "6 m all-modes (50.5–52.0)"),
                // 2 m all-modes — the part of the band the image and facsimile
                // modes have always lived in.
                (144_500_000.0, 144_794_000.0, "2 m all-modes (144.500–144.794)"),
                // 70 cm, where 25 kHz channels are the norm and where the
                // draft's own deployment example lives.
                (430_000_000.0, 440_000_000.0, "70 cm (430–440)"),
            ],
            (RifpProfile::Cpfsk4800, crate::Region::R2) => &[
                (29_520_000.0, 29_700_000.0, "10 m FM (29.520–29.700)"),
                (50_500_000.0, 54_000_000.0, "6 m all-modes (50.5–54.0)"),
                (146_000_000.0, 148_000_000.0, "2 m FM (146–148)"),
                (420_000_000.0, 450_000_000.0, "70 cm (420–450)"),
            ],
            (RifpProfile::Cpfsk4800, crate::Region::R3) => &[
                (29_520_000.0, 29_700_000.0, "10 m FM (29.520–29.700)"),
                (50_500_000.0, 54_000_000.0, "6 m all-modes (50.5–54.0)"),
                (144_500_000.0, 144_794_000.0, "2 m all-modes (144.500–144.794)"),
                (430_000_000.0, 450_000_000.0, "70 cm (430–450)"),
            ],
        }
    }

    /// True when `hz` falls in one of [`RifpProfile::wide_segments`], or when
    /// the profile is narrow enough not to need one.
    pub fn fits_at(self, hz: f64) -> bool {
        !self.needs_wide_segment()
            || self.wide_segments().iter().any(|&(lo, hi, _)| (lo..=hi).contains(&hz))
    }

    /// The wide segments as one human-readable list, for the panel's warning.
    pub fn wide_segments_text(self) -> String {
        self.wide_segments().iter().map(|&(_, _, name)| name).collect::<Vec<_>>().join(", ")
    }
}

/// The media-type + content-encoding pair carried in the manifest. Each variant
/// is one row of the draft's "Initial Media and Encoding Combinations" table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum RifpEncoding {
    /// Try every encoding this build can produce and send the smallest.
    #[default]
    Auto,
    /// `image/png` + `identity` — a complete PNG file.
    Png,
    /// `image/jpeg` + `identity` — a complete JPEG file (lossy).
    Jpeg,
    /// `image/rifp-raster` + `raw` — the packed grayscale raster.
    Raw,
    /// `image/rifp-raster` + `rle8` — byte-oriented run-length coding.
    Rle8,
    /// `image/rifp-raster` + `zlib` — ZLIB-compressed packed raster.
    Zlib,
    /// `image/tiff` + `ccitt-group4` — bilevel Group 4 facsimile (1 bpp only).
    Group4,
    /// `image/tiff` + `ccitt-group3` — bilevel Group 3 facsimile (1 bpp only).
    Group3,
}

impl RifpEncoding {
    /// Every encoding, including the ones this build can only receive.
    pub const ALL: [RifpEncoding; 8] = [
        RifpEncoding::Auto,
        RifpEncoding::Group4,
        RifpEncoding::Group3,
        RifpEncoding::Png,
        RifpEncoding::Zlib,
        RifpEncoding::Rle8,
        RifpEncoding::Raw,
        RifpEncoding::Jpeg,
    ];

    /// The encodings offered for transmit, in menu order. Group 3 is missing on
    /// purpose: the bundled facsimile codec writes and reads Group 4 only, so a
    /// received Group 3 object is named but not decoded.
    pub const TX_MENU: [RifpEncoding; 7] = [
        RifpEncoding::Auto,
        RifpEncoding::Group4,
        RifpEncoding::Png,
        RifpEncoding::Zlib,
        RifpEncoding::Rle8,
        RifpEncoding::Raw,
        RifpEncoding::Jpeg,
    ];

    /// Short label for a button.
    pub fn label(self) -> &'static str {
        match self {
            RifpEncoding::Auto => "Auto",
            RifpEncoding::Png => "PNG",
            RifpEncoding::Jpeg => "JPEG",
            RifpEncoding::Raw => "Raw",
            RifpEncoding::Rle8 => "RLE8",
            RifpEncoding::Zlib => "Zlib",
            RifpEncoding::Group4 => "G4",
            RifpEncoding::Group3 => "G3",
        }
    }

    /// `(media_type, content_encoding)` as they appear in the manifest.
    /// `None` for [`RifpEncoding::Auto`], which is a choice, not an encoding.
    pub fn manifest_pair(self) -> Option<(&'static str, &'static str)> {
        match self {
            RifpEncoding::Auto => None,
            RifpEncoding::Png => Some(("image/png", "identity")),
            RifpEncoding::Jpeg => Some(("image/jpeg", "identity")),
            RifpEncoding::Raw => Some(("image/rifp-raster", "raw")),
            RifpEncoding::Rle8 => Some(("image/rifp-raster", "rle8")),
            RifpEncoding::Zlib => Some(("image/rifp-raster", "zlib")),
            RifpEncoding::Group4 => Some(("image/tiff", "ccitt-group4")),
            RifpEncoding::Group3 => Some(("image/tiff", "ccitt-group3")),
        }
    }

    /// Map a received manifest's pair back to an encoding.
    pub fn from_manifest(media_type: &str, content_encoding: &str) -> Option<RifpEncoding> {
        RifpEncoding::ALL
            .into_iter()
            .find(|e| e.manifest_pair() == Some((media_type, content_encoding)))
    }

    /// True for the encodings that carry the packed raster itself, so a partly
    /// received object can still be painted row by row.
    pub fn is_raster(self) -> bool {
        matches!(self, RifpEncoding::Raw | RifpEncoding::Rle8 | RifpEncoding::Zlib)
    }

    /// True for the bilevel facsimile encodings, which are 1 bpp by definition.
    pub fn is_bilevel(self) -> bool {
        matches!(self, RifpEncoding::Group3 | RifpEncoding::Group4)
    }

    /// True where the encoding throws detail away.
    pub fn is_lossy(self) -> bool {
        matches!(self, RifpEncoding::Jpeg)
    }
}

/// Transmit image size. RIFP fixes no dimensions — these are the sizes the
/// panel offers, chosen so a picture stays a sane length at 4800 baud.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum RifpSize {
    /// 160×120 — the reference implementation's "small" preset.
    Tiny,
    /// 320×240.
    #[default]
    Small,
    /// 480×360.
    Medium,
    /// 640×480.
    Large,
}

impl RifpSize {
    pub const ALL: [RifpSize; 4] =
        [RifpSize::Tiny, RifpSize::Small, RifpSize::Medium, RifpSize::Large];

    /// Transmitted image size in pixels, `(width, height)`.
    pub fn dimensions(self) -> (u16, u16) {
        match self {
            RifpSize::Tiny => (160, 120),
            RifpSize::Small => (320, 240),
            RifpSize::Medium => (480, 360),
            RifpSize::Large => (640, 480),
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            RifpSize::Tiny => "160×120",
            RifpSize::Small => "320×240",
            RifpSize::Medium => "480×360",
            RifpSize::Large => "640×480",
        }
    }
}

/// Metadata recovered from a received object's manifest, for the panel's
/// caption under a decoded picture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RifpMeta {
    /// Session ID as sixteen lowercase hex digits.
    pub session: String,
    /// The manifest's suggested filename (already sanitised).
    pub filename: String,
    /// Sender ID header TLV, when the sender included one.
    pub sender: Option<String>,
    /// Content Hint header TLV, when the sender included one.
    pub hint: Option<String>,
    pub media_type: String,
    pub content_encoding: String,
    pub width: u16,
    pub height: u16,
    pub bits_per_pixel: u8,
    /// Complete encoded object size in octets.
    pub encoded_size: u32,
    /// Number of DATA chunks the object was carried in.
    pub chunk_count: u32,
    /// How many of those chunks arrived on their first transmission, i.e.
    /// without needing a repeat. Purely informational.
    pub chunks_first_pass: u32,
}

/// One in-progress incoming transfer, as shown in the panel's session list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RifpSession {
    /// Session ID as sixteen lowercase hex digits.
    pub session: String,
    /// Sender ID header TLV, when one was seen.
    pub sender: Option<String>,
    /// True once a valid manifest has been received for this session.
    pub have_manifest: bool,
    /// Chunks received so far, and the total the manifest/header declares
    /// (0 when nothing has told us yet).
    pub have: u32,
    pub total: u32,
    /// Which chunks are present, one bit per sequence number, LSB first within
    /// each byte. Truncated to [`RIFP_MAP_MAX_CHUNKS`] chunks.
    pub map: Vec<u8>,
    /// Seconds since a frame for this session last arrived.
    pub idle_s: u32,
}

/// Longest chunk map carried in [`RifpSession`]; beyond this the panel draws a
/// plain progress bar instead of a per-chunk grid. Keeps the status message
/// small on a long transfer.
pub const RIFP_MAP_MAX_CHUNKS: usize = 8192;

/// Broadcast status for the RIFP panel: what is being sent and received and how
/// far along. Rides the wire as part of the digital-mode event stream.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RifpStatus {
    /// The profile in use for transmit and receive.
    pub profile: RifpProfile,
    /// True while an object is being transmitted.
    pub tx_active: bool,
    /// Fraction of the outgoing transmission completed, 0.0..=1.0.
    pub tx_progress: f32,
    /// Frames sent / total frames queued for the current transmission.
    pub tx_frame: u32,
    pub tx_frames: u32,
    /// Seconds of air time left in the current transmission.
    pub tx_remaining_s: u32,
    /// What the outgoing object was encoded as, and how big it came out.
    pub tx_encoding: Option<RifpEncoding>,
    pub tx_bytes: u32,
    /// Modem lock quality, ~0 (noise) … 1 (a clean FSK signal in the passband).
    pub signal: f32,
    /// Valid frames received since the mode was entered.
    pub rx_frames: u32,
    /// Frames that failed their CRC — the honest measure of channel quality.
    pub rx_bad_frames: u32,
    /// Objects completed and verified since the mode was entered.
    pub rx_objects: u32,
    /// Incoming transfers currently being reassembled, newest first.
    pub sessions: Vec<RifpSession>,
    /// The last thing that went wrong (a failed digest, an object we cannot
    /// decode, a session abandoned), for the panel's status line.
    pub last_error: Option<String>,
}

impl Default for RifpStatus {
    fn default() -> Self {
        RifpStatus {
            profile: RifpProfile::default(),
            tx_active: false,
            tx_progress: 0.0,
            tx_frame: 0,
            tx_frames: 0,
            tx_remaining_s: 0,
            tx_encoding: None,
            tx_bytes: 0,
            signal: 0.0,
            rx_frames: 0,
            rx_bad_frames: 0,
            rx_objects: 0,
            sessions: Vec::new(),
            last_error: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_wide_segments_are_where_the_channel_fits() {
        let p = RifpProfile::Cpfsk4800;
        assert!(p.needs_wide_segment());
        // The draft's own deployment example, and the 6 m and 10 m segments
        // where wideband operation is generally allowed.
        assert!(p.fits_at(RIFP_CALLING_HZ));
        assert!(p.fits_at(51_250_000.0));
        assert!(p.fits_at(29_600_000.0));
        // Narrow-band segments, on any band.
        assert!(!p.fits_at(14_230_000.0));
        assert!(!p.fits_at(29_000_000.0));
        assert!(!p.fits_at(50_200_000.0));
        assert!(p.fits_at(144_700_000.0));
        assert!(!p.fits_at(145_500_000.0));
        assert!(p.wide_segments_text().contains("70 cm"));
    }

    #[test]
    fn manifest_pairs_round_trip() {
        for e in RifpEncoding::ALL {
            let Some((media, encoding)) = e.manifest_pair() else { continue };
            assert_eq!(RifpEncoding::from_manifest(media, encoding), Some(e));
        }
        assert_eq!(RifpEncoding::from_manifest("image/gif", "identity"), None);
    }
}
