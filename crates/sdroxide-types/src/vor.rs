//! VOR navigation beacon read-out: what the engine tells the window about the
//! beacon the receiver is parked on. Pure data + serde; the demodulation lives
//! in the native `sdroxide-vor` crate.
//!
//! Like HFDL and the radiosondes, the VOR lane is not a radio mode: it takes
//! its own down-conversion of the raw I/Q around whatever the receiver is tuned
//! to, whenever that is inside the 108–118 MHz navigation band. Nothing is
//! asked of the operator.

use serde::{Deserialize, Serialize};

/// The VOR channels, Hz: 108.00 to 117.95 MHz. The ILS localisers share the
/// bottom of it (108.1 to 111.95 MHz, odd tenths), and a localiser has no
/// 30 Hz pair to lock on, so it simply never locks.
pub const VOR_BAND_HZ: (f64, f64) = (108_000_000.0, 117_975_000.0);

/// Why the lane is not running although the receiver is in the VOR band.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VorUnavailable {
    /// The front end hands over demodulated audio; the lane needs raw I/Q.
    AudioFront,
    /// The tuned channel is outside, or too close to the edge of, the span the
    /// receiver delivers.
    OutsideWindow,
}

/// The beacon, as heard.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct VorStatus {
    /// Whether the lane is running. `false` is sent once when it stops, so the
    /// window can drop the read-out.
    pub running: bool,
    /// Why not, when the receiver is in the band but the lane cannot run.
    pub unavailable: Option<VorUnavailable>,
    /// The channel it listens to, Hz.
    pub freq_hz: f64,
    /// Bearing of the receiver from the beacon, degrees clockwise from the
    /// beacon's north. Meaningful only while `locked`.
    pub radial_deg: f32,
    /// Depth of the 30 Hz amplitude modulation, fraction of the carrier.
    pub var_depth: f32,
    /// Depth of the 9960 Hz subcarrier, fraction of the carrier.
    pub ref_depth: f32,
    /// Measured deviation of the subcarrier, Hz (480 on a healthy beacon).
    pub deviation_hz: f32,
    /// Both 30 Hz signals are present and strong enough to trust the radial.
    pub locked: bool,
    /// The Morse ident, once heard twice running.
    pub ident: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_status_survives_the_wire() {
        let st = VorStatus {
            running: true,
            freq_hz: 114_100_000.0,
            radial_deg: 138.5,
            var_depth: 0.3,
            ref_depth: 0.17,
            deviation_hz: 480.0,
            locked: true,
            ident: Some("BBG".into()),
            ..Default::default()
        };
        let bytes = postcard::to_stdvec(&st).unwrap();
        assert_eq!(postcard::from_bytes::<VorStatus>(&bytes).unwrap(), st);
    }
}
