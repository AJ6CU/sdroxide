//! Solar disk imagery from SDO.
//!
//! <https://sdo.gsfc.nasa.gov/assets/img/latest/> publishes a "latest" browse
//! JPEG per instrument channel and resolution, updated every few minutes. They
//! are **orthographic images of the Earth-facing hemisphere with solar north
//! up** (the P angle is already removed), which is what lets the renderer map
//! one straight onto a sphere: a surface point's image coordinate is exactly
//! its component along the image's right/up axes, in solar radii.
//!
//! That mapping needs to know how large the disk is within the frame, and the
//! answer differs between instruments and resolutions — so it is measured from
//! the image rather than hard-coded.

use serde::{Deserialize, Serialize};

const BASE: &str = "https://sdo.gsfc.nasa.gov/assets/img/latest";

/// Fallback disk radius (as a fraction of image width) when detection fails.
const FALLBACK_DISK_RADIUS: f32 = 0.45;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum SdoChannel {
    /// HMI white-light continuum: the photosphere, where sunspots are actually
    /// visible. The sensible default for a view about sunspots.
    HmiContinuum,
    Aia0193,
    Aia0304,
    Aia0171,
    Aia0211,
    /// The 211/193/171 false-colour composite.
    Composite,
}

impl SdoChannel {
    pub const ALL: [SdoChannel; 6] = [
        SdoChannel::HmiContinuum,
        SdoChannel::Aia0193,
        SdoChannel::Aia0304,
        SdoChannel::Aia0171,
        SdoChannel::Aia0211,
        SdoChannel::Composite,
    ];

    pub fn from_u8(v: u8) -> SdoChannel {
        *SdoChannel::ALL.get(v as usize).unwrap_or(&SdoChannel::HmiContinuum)
    }

    pub fn to_u8(self) -> u8 {
        SdoChannel::ALL.iter().position(|c| *c == self).unwrap_or(0) as u8
    }

    /// The code in the image filename.
    pub fn code(self) -> &'static str {
        match self {
            SdoChannel::HmiContinuum => "HMIIC",
            SdoChannel::Aia0193 => "0193",
            SdoChannel::Aia0304 => "0304",
            SdoChannel::Aia0171 => "0171",
            SdoChannel::Aia0211 => "0211",
            SdoChannel::Composite => "211193171",
        }
    }

    /// Short label for a chip.
    pub fn label(self) -> &'static str {
        match self {
            SdoChannel::HmiContinuum => "HMI",
            SdoChannel::Aia0193 => "193",
            SdoChannel::Aia0304 => "304",
            SdoChannel::Aia0171 => "171",
            SdoChannel::Aia0211 => "211",
            SdoChannel::Composite => "MIX",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            SdoChannel::HmiContinuum => "HMI continuum — white light, shows sunspots",
            SdoChannel::Aia0193 => "AIA 193 Å — corona and coronal holes, ~1.2 MK",
            SdoChannel::Aia0304 => "AIA 304 Å — chromosphere and filaments, ~50 000 K",
            SdoChannel::Aia0171 => "AIA 171 Å — quiet corona and coronal loops, ~600 000 K",
            SdoChannel::Aia0211 => "AIA 211 Å — active-region corona, ~2 MK",
            SdoChannel::Composite => "AIA 211/193/171 composite",
        }
    }

    /// Colour of the procedural surface used before the first download and
    /// whenever the far side is in view.
    pub fn base_rgb(self) -> [f32; 3] {
        match self {
            SdoChannel::HmiContinuum => [1.00, 0.84, 0.60],
            SdoChannel::Aia0193 => [0.85, 0.72, 0.35],
            SdoChannel::Aia0304 => [1.00, 0.52, 0.20],
            SdoChannel::Aia0171 => [1.00, 0.78, 0.35],
            SdoChannel::Aia0211 => [0.86, 0.55, 0.95],
            SdoChannel::Composite => [1.00, 0.70, 0.45],
        }
    }

    pub fn url(self, resolution: u32) -> String {
        format!("{BASE}/latest_{resolution}_{}.jpg", self.code())
    }

    /// Cache filename for this channel and resolution.
    pub fn cache_name(self, resolution: u32) -> String {
        format!("sun_{}_{resolution}.jpg", self.code())
    }
}

/// A decoded solar disk, ready to upload as an RGBA texture.
pub struct SunImage {
    pub channel: SdoChannel,
    pub fetched_unix: i64,
    pub w: u32,
    pub h: u32,
    pub rgba: Vec<u8>,
    /// Radius of the solar disk as a fraction of the image width. The shader
    /// needs this to place surface points in the photograph.
    pub disk_radius_frac: f32,
}

impl std::fmt::Debug for SunImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SunImage")
            .field("channel", &self.channel)
            .field("size", &(self.w, self.h))
            .field("disk_radius_frac", &self.disk_radius_frac)
            .finish_non_exhaustive()
    }
}

/// Decode a browse JPEG and measure its solar disk.
pub fn decode(bytes: &[u8], channel: SdoChannel, fetched_unix: i64) -> Option<SunImage> {
    let img = image::load_from_memory(bytes).ok()?.to_rgba8();
    let (w, h) = img.dimensions();
    if w < 32 || h < 32 {
        return None;
    }
    let rgba = img.into_raw();
    let disk_radius_frac = detect_disk_radius(&rgba, w, h);
    Some(SunImage { channel, fetched_unix, w, h, rgba, disk_radius_frac })
}

fn luma(rgba: &[u8], w: u32, x: u32, y: u32) -> f32 {
    let i = ((y * w + x) * 4) as usize;
    0.2126 * rgba[i] as f32 + 0.7152 * rgba[i + 1] as f32 + 0.0722 * rgba[i + 2] as f32
}

/// Median luminance at each integer radius from the image centre.
///
/// The median over azimuth, not the mean, is what makes this work on every
/// product: coronal loops, streamers, filaments and instrument watermarks are
/// all *azimuthally local*, so they move the mean but not the median. What
/// survives is the radially symmetric part — the disk.
fn radial_median_profile(rgba: &[u8], w: u32, h: u32) -> Vec<f32> {
    const AZIMUTHS: usize = 64;
    let (cx, cy) = (w as f32 * 0.5, h as f32 * 0.5);
    let r_max = (w.min(h) / 2).saturating_sub(1) as usize;
    let dirs: Vec<(f32, f32)> = (0..AZIMUTHS)
        .map(|i| {
            let a = i as f32 / AZIMUTHS as f32 * std::f32::consts::TAU;
            (a.cos(), a.sin())
        })
        .collect();

    let mut profile = Vec::with_capacity(r_max + 1);
    let mut ring = Vec::with_capacity(AZIMUTHS);
    for r in 0..=r_max {
        ring.clear();
        for (dx, dy) in &dirs {
            let x = (cx + dx * r as f32).round();
            let y = (cy + dy * r as f32).round();
            if x >= 0.0 && y >= 0.0 && (x as u32) < w && (y as u32) < h {
                ring.push(luma(rgba, w, x as u32, y as u32));
            }
        }
        if ring.is_empty() {
            profile.push(0.0);
            continue;
        }
        ring.sort_by(f32::total_cmp);
        profile.push(ring[ring.len() / 2]);
    }
    profile
}

/// Measure the solar disk's radius as a fraction of the image width.
///
/// The limb is found as the steepest fall in the radial median profile. A
/// simple brightness threshold does not work across products: HMI's photosphere
/// drops to black outside the limb, but in the EUV channels the corona is still
/// bright well beyond it, so "the first bright pixel inward from the edge"
/// finds coronal emission and over-measures by a wide margin. The steepest
/// gradient is the limb in both cases.
///
/// Returns [`FALLBACK_DISK_RADIUS`] when the profile has no clear edge — better
/// a known-approximate constant than a confidently wrong measurement.
pub fn detect_disk_radius(rgba: &[u8], w: u32, h: u32) -> f32 {
    let profile = radial_median_profile(rgba, w, h);
    if profile.len() < 32 {
        return FALLBACK_DISK_RADIUS;
    }

    // Light smoothing, so JPEG ringing at the limb does not win the search.
    let smooth: Vec<f32> = (0..profile.len())
        .map(|i| {
            let lo = i.saturating_sub(2);
            let hi = (i + 3).min(profile.len());
            profile[lo..hi].iter().sum::<f32>() / (hi - lo) as f32
        })
        .collect();

    // The disk plateau: the median of the inner third, which is inside the limb
    // on every product at every framing.
    let mut inner: Vec<f32> = smooth[..smooth.len() / 3].to_vec();
    inner.sort_by(f32::total_cmp);
    let plateau = inner[inner.len() / 2];
    if plateau < 8.0 {
        return FALLBACK_DISK_RADIUS;
    }

    // Steepest fall, searched only where a solar limb can plausibly be.
    let (lo, hi) = (smooth.len() / 5, smooth.len() - 3);
    let mut best = (0.0f32, 0usize);
    for r in lo..hi {
        let drop = smooth[r - 2] - smooth[r + 2];
        if drop > best.0 {
            best = (drop, r);
        }
    }
    // A real limb is a substantial step, not gradual shading.
    if best.0 < plateau * 0.15 {
        return FALLBACK_DISK_RADIUS;
    }
    best.1 as f32 / w as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    const HMI: &[u8] = include_bytes!("../tests/fixtures/sdo_512_hmiic.jpg");
    const AIA193: &[u8] = include_bytes!("../tests/fixtures/sdo_512_0193.jpg");

    #[test]
    fn decodes_real_sdo_browse_images() {
        for (bytes, channel) in [(HMI, SdoChannel::HmiContinuum), (AIA193, SdoChannel::Aia0193)] {
            let img = decode(bytes, channel, 1_784_937_600).expect("fixture must decode");
            assert_eq!((img.w, img.h), (512, 512));
            assert_eq!(img.rgba.len(), 512 * 512 * 4);
            assert_eq!(img.channel, channel);
            // The disk fills most but not all of the frame on every product.
            assert!(
                (0.35..0.50).contains(&img.disk_radius_frac),
                "{channel:?} disk radius {} looks wrong",
                img.disk_radius_frac
            );
        }
    }

    /// The reason detection exists: HMI and AIA have different plate scales and
    /// the EUV limb sits a few megametres above the photospheric one, so a
    /// single hard-coded radius would misplace surface features on one of them.
    #[test]
    fn detected_radius_is_product_specific() {
        let hmi = decode(HMI, SdoChannel::HmiContinuum, 0).unwrap().disk_radius_frac;
        let aia = decode(AIA193, SdoChannel::Aia0193, 0).unwrap().disk_radius_frac;
        assert_ne!(FALLBACK_DISK_RADIUS, hmi, "HMI fell back instead of measuring");
        assert_ne!(FALLBACK_DISK_RADIUS, aia, "AIA fell back instead of measuring");
        // HMI's white-light limb is the sharpest edge in any solar image, so
        // this one is worth pinning: 234/512 px.
        assert!((hmi - 0.457).abs() < 0.01, "HMI disk radius {hmi}");
        assert!(
            (hmi - aia).abs() > 0.01,
            "the two products measured the same ({hmi} vs {aia}) — detection is not discriminating"
        );
    }

    /// A synthetic disk of exactly known size, to check the measurement is
    /// accurate and not merely self-consistent.
    #[test]
    fn measures_a_disk_of_known_radius() {
        for radius in [80.0f32, 150.0, 200.0] {
            let (w, h) = (512u32, 512u32);
            let mut rgba = vec![0u8; (w * h * 4) as usize];
            for y in 0..h {
                for x in 0..w {
                    let (dx, dy) = (x as f32 - 256.0, y as f32 - 256.0);
                    let v = if (dx * dx + dy * dy).sqrt() <= radius { 230 } else { 2 };
                    let i = ((y * w + x) * 4) as usize;
                    rgba[i..i + 4].copy_from_slice(&[v, v, v, 255]);
                }
            }
            let got = detect_disk_radius(&rgba, w, h) * w as f32;
            assert!((got - radius).abs() <= 2.0, "radius {radius} measured as {got}");
        }
    }

    /// An image with a bright artefact hard against one edge must not fool the
    /// scan — that is what the median is for.
    #[test]
    fn a_bright_edge_artefact_does_not_skew_the_measurement() {
        let (w, h) = (512u32, 512u32);
        let mut rgba = vec![0u8; (w * h * 4) as usize];
        for y in 0..h {
            for x in 0..w {
                let (dx, dy) = (x as f32 - 256.0, y as f32 - 256.0);
                let v = if (dx * dx + dy * dy).sqrt() <= 150.0 { 230 } else { 2 };
                let i = ((y * w + x) * 4) as usize;
                rgba[i..i + 4].copy_from_slice(&[v, v, v, 255]);
            }
        }
        // A watermark stripe touching the left edge on the centre row.
        for x in 0..40 {
            let i = ((256 * w + x) * 4) as usize;
            rgba[i..i + 4].copy_from_slice(&[255, 255, 255, 255]);
        }
        let got = detect_disk_radius(&rgba, w, h) * w as f32;
        assert!((got - 150.0).abs() <= 3.0, "artefact skewed the radius to {got}");
    }

    #[test]
    fn a_blank_image_falls_back_rather_than_guessing() {
        let rgba = vec![0u8; 512 * 512 * 4];
        assert_eq!(detect_disk_radius(&rgba, 512, 512), FALLBACK_DISK_RADIUS);
    }

    #[test]
    fn rejects_non_images() {
        assert!(decode(b"not a jpeg", SdoChannel::Aia0193, 0).is_none());
        assert!(decode(&[], SdoChannel::Aia0193, 0).is_none());
    }

    #[test]
    fn urls_and_codes_match_the_published_naming() {
        assert_eq!(
            SdoChannel::HmiContinuum.url(1024),
            "https://sdo.gsfc.nasa.gov/assets/img/latest/latest_1024_HMIIC.jpg"
        );
        assert_eq!(
            SdoChannel::Aia0193.url(2048),
            "https://sdo.gsfc.nasa.gov/assets/img/latest/latest_2048_0193.jpg"
        );
        assert_eq!(SdoChannel::HmiContinuum.cache_name(1024), "sun_HMIIC_1024.jpg");
        for c in SdoChannel::ALL {
            assert_eq!(SdoChannel::from_u8(c.to_u8()), c);
        }
        // Out-of-range indices fall back rather than panicking.
        assert_eq!(SdoChannel::from_u8(99), SdoChannel::HmiContinuum);
    }
}
