//! The satellite lock: what the engine needs to *operate through* a satellite
//! rather than merely draw it.
//!
//! Tracking answers "where is it"; operating adds "what do I tune". While a
//! lock is active the engine propagates the orbit itself, corrects the receive
//! path for Doppler in the DDC (the dial keeps reading the published
//! frequency), derives the transmit frequency from the transponder mapping,
//! and reports what it is doing as a [`SatTrackStatus`] stream. These are the
//! types both ends of that conversation share; the orbital mechanics stay in
//! `sdroxide-solar`, which this crate cannot depend on.

use serde::{Deserialize, Serialize};

/// Speed of light in km/s, the units range rate arrives in.
pub const C_KM_S: f64 = 299_792.458;

/// Doppler correction for the receive path, Hz.
///
/// `range_rate_km_s` is positive for a receding satellite. The sign is chosen
/// so the result can be *added* to the DDC offset: an approaching satellite is
/// heard above its published frequency, so the correction is positive and the
/// DDC tunes up to where the signal actually is.
pub fn doppler_rx_hz(f_down_hz: f64, range_rate_km_s: f64) -> f64 {
    -f_down_hz * range_rate_km_s / C_KM_S
}

/// Doppler pre-correction for the transmit path, Hz — added to the nominal
/// uplink so the *satellite* hears the published frequency. The mirror image
/// of [`doppler_rx_hz`]: while approaching we transmit low, because the motion
/// will carry us up.
pub fn doppler_tx_hz(f_up_hz: f64, range_rate_km_s: f64) -> f64 {
    f_up_hz * range_rate_km_s / C_KM_S
}

/// The uplink side of a lock: how a downlink dial position maps to a transmit
/// frequency.
///
/// One struct covers all three shapes a satellite link takes. A linear
/// transponder has real passbands on both sides and a slope of ±1: tune across
/// the downlink and the uplink follows, reversed when `inverting` (the common
/// case — inverting cancels the transponder's own Doppler asymmetry). An FM
/// repeater is the degenerate case where both passbands are single points, so
/// the mapping is constant. Frequencies are Hz here, not the MHz of
/// [`crate::Passband`]: this is the tuning domain, not the publishing domain.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct SatUplink {
    pub up_lo_hz: f64,
    pub up_hi_hz: f64,
    pub down_lo_hz: f64,
    pub down_hi_hz: f64,
    /// High end of the downlink corresponds to the low end of the uplink.
    pub inverting: bool,
}

impl SatUplink {
    /// Build from a published link, if it has both directions.
    pub fn from_link(link: &crate::SatLink) -> Option<SatUplink> {
        let (up, down) = (link.uplink?, link.downlink?);
        Some(SatUplink {
            up_lo_hz: up.lo_mhz * 1e6,
            up_hi_hz: up.hi_mhz * 1e6,
            down_lo_hz: down.lo_mhz * 1e6,
            down_hi_hz: down.hi_mhz * 1e6,
            inverting: link.inverting,
        })
    }

    /// The nominal uplink for a downlink dial position.
    ///
    /// The dial is clamped into the downlink passband first and the result
    /// into the uplink passband after, so a dial parked just outside the
    /// transponder — or two published passbands of slightly different width —
    /// still yields a transmit frequency inside the allocation rather than a
    /// few kilohertz into whatever sits next to it.
    pub fn uplink_for(&self, f_down_hz: f64) -> f64 {
        let f = f_down_hz.clamp(self.down_lo_hz, self.down_hi_hz);
        let up = if self.inverting {
            self.up_hi_hz - (f - self.down_lo_hz)
        } else {
            self.up_lo_hz + (f - self.down_lo_hz)
        };
        up.clamp(self.up_lo_hz, self.up_hi_hz)
    }
}

/// Everything the engine needs to start (or re-shape) a satellite lock.
///
/// Deliberately self-contained: a remote client can send one of these with
/// nothing but a catalogue number and a downlink, and the engine — which holds
/// the element sets and the operator's grid — fills in the rest. The optional
/// fields exist so a client that *does* know better (a pasted TLE, a portable
/// station at a different grid) can say so.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SatLockConfig {
    pub norad_id: u64,
    /// Display name, carried so status can name the bird even if the element
    /// set that supplied it disappears mid-lock.
    pub name: String,
    /// Explicit element lines 1 and 2. `None` — the normal case — has the
    /// engine resolve the catalogue number from its own custom TLEs and the
    /// subscription cache.
    pub tle: Option<(String, String)>,
    /// Observer geodetic (lat, lon) in degrees. `None` derives it from the
    /// station's configured grid locator.
    pub observer: Option<(f64, f64)>,
    /// Nominal downlink to put the dial on when the lock starts, Hz.
    pub downlink_hz: f64,
    /// The transmit mapping. `None` for a downlink-only lock (a beacon, an APT
    /// bird): receive Doppler still applies, transmit is refused.
    pub uplink: Option<SatUplink>,
    /// Apply Doppler corrections. Off, the lock still tracks — az/el, passes,
    /// rotator — which is what a CAT-rig station gets regardless.
    pub doppler: bool,
    /// CTCSS tone for the uplink, in Hz (e.g. 67.0 for ISS). `None` means
    /// no tone. Used by CAT rigs to set the tone on VFO B.
    pub ctcss_hz: Option<f64>,
    /// Drive the rotator client while locked.
    pub rotator: bool,
}

/// One pass, as the engine reports it — a serde mirror of the solar crate's
/// `Pass`, which lives above this crate in the dependency graph and does not
/// serialize.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SatPass {
    pub rise_unix: i64,
    pub set_unix: i64,
    /// Azimuths at the horizon, degrees clockwise from north.
    pub rise_az: f64,
    pub set_az: f64,
    /// Highest elevation reached, and when.
    pub max_el: f64,
    pub max_el_unix: i64,
}

/// What the lock is doing right now, broadcast a couple of times a second
/// while one is active.
///
/// Its own event rather than a [`crate::RadioState`] field: state goes out
/// whole on every change, and a continuously moving az/el would turn every
/// broadcast into a full snapshot — the same reasoning that keeps the
/// scanner's dwell out of it.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SatTrackStatus {
    pub norad_id: u64,
    pub name: String,
    /// Look angles from the observer, degrees; azimuth clockwise from north.
    pub az_deg: f64,
    pub el_deg: f64,
    pub range_km: f64,
    /// Positive receding.
    pub range_rate_km_s: f64,
    /// The corrections as applied (zero when corrections are off or the
    /// elements have gone stale).
    pub doppler_rx_hz: f64,
    pub doppler_tx_hz: f64,
    /// The nominal frequencies: the dial, and where a key-down would transmit
    /// before its own Doppler correction. `None` for a downlink-only lock.
    pub downlink_hz: f64,
    pub uplink_hz: Option<f64>,
    /// Above the observer's horizon.
    pub visible: bool,
    /// The element set has aged past trust; corrections are suspended (a
    /// kilohertz of stale correction is worse than none) until a TLE refresh
    /// supplies a newer one, at which point the lock resumes by itself.
    pub stale_elements: bool,
    /// Doppler is actually being applied to the signal path — false when the
    /// operator turned it off, the elements are stale, or the source is a CAT
    /// rig whose dial cannot be ridden continuously.
    pub corrections_active: bool,
    /// The pass in progress, or the next one inside the search window. `None`
    /// for a bird that never rises from this QTH — or a geostationary one,
    /// which never sets and is described by `visible` alone.
    pub next_pass: Option<SatPass>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Passband, SatLink};

    /// RS-44's inverting transponder, from the published passbands: the low
    /// end of the downlink comes from the high end of the uplink.
    #[test]
    fn an_inverting_transponder_maps_reversed() {
        let u = SatUplink {
            up_lo_hz: 145.935e6,
            up_hi_hz: 145.995e6,
            down_lo_hz: 435.610e6,
            down_hi_hz: 435.670e6,
            inverting: true,
        };
        assert_eq!(u.uplink_for(435.610e6), 145.995e6);
        assert_eq!(u.uplink_for(435.670e6), 145.935e6);
        // Mid-passband maps to mid-passband either way round.
        assert!((u.uplink_for(435.640e6) - 145.965e6).abs() < 1.0);
        // A dial outside the passband clamps rather than extrapolating.
        assert_eq!(u.uplink_for(435.500e6), u.uplink_for(435.610e6));
    }

    #[test]
    fn a_normal_transponder_maps_straight_and_an_fm_pair_is_constant() {
        let u = SatUplink {
            up_lo_hz: 2400.050e6,
            up_hi_hz: 2400.300e6,
            down_lo_hz: 10489.550e6,
            down_hi_hz: 10489.800e6,
            inverting: false,
        };
        assert_eq!(u.uplink_for(10489.550e6), 2400.050e6);
        assert!((u.uplink_for(10489.675e6) - 2400.175e6).abs() < 1.0);

        let fm = SatUplink {
            up_lo_hz: 145.850e6,
            up_hi_hz: 145.850e6,
            down_lo_hz: 436.795e6,
            down_hi_hz: 436.795e6,
            inverting: false,
        };
        assert_eq!(fm.uplink_for(436.795e6), 145.850e6);
        assert_eq!(fm.uplink_for(437.000e6), 145.850e6);
    }

    #[test]
    fn from_link_carries_the_inversion_and_needs_both_directions() {
        let l = SatLink::duplex(
            "U/V transponder",
            "SSB/CW",
            Passband::range(435.610, 435.670),
            Passband::range(145.935, 145.995),
        )
        .inverting();
        let u = SatUplink::from_link(&l).expect("both directions present");
        assert!(u.inverting);
        assert_eq!(u.up_lo_hz, 435.610e6);
        assert_eq!(u.down_lo_hz, 145.935e6);

        let beacon = SatLink::down("Beacon", "CW", Passband::at(435.605));
        assert!(SatUplink::from_link(&beacon).is_none());
    }

    /// The classic failure in every satellite program is a Doppler sign flip.
    /// Pin the convention: approaching (negative range rate) means we *hear
    /// high* and must *transmit low*.
    #[test]
    fn doppler_signs_follow_the_approaching_satellite() {
        let approaching = -6.0; // km/s, a LEO bird coming over the horizon
        assert!(doppler_rx_hz(145.8e6, approaching) > 0.0);
        assert!(doppler_tx_hz(435.0e6, approaching) < 0.0);
        // ~2.9 kHz at 145.8 MHz for 6 km/s.
        assert!((doppler_rx_hz(145.8e6, approaching) - 2918.0).abs() < 5.0);
        // And nothing at all for a geostationary bird.
        assert_eq!(doppler_rx_hz(10.489e9, 0.0), 0.0);
    }

    /// Both config and status survive the wire format.
    #[test]
    fn the_lock_types_round_trip_through_postcard() {
        let cfg = SatLockConfig {
            norad_id: 44909,
            name: "RS-44".into(),
            tle: None,
            observer: Some((48.2, 16.4)),
            downlink_hz: 435.640e6,
            uplink: Some(SatUplink {
                up_lo_hz: 145.935e6,
                up_hi_hz: 145.995e6,
                down_lo_hz: 435.610e6,
                down_hi_hz: 435.670e6,
                inverting: true,
            }),
            doppler: true,
            ctcss_hz: Some(67.0),
            rotator: false,
        };
        let bytes = postcard::to_allocvec(&cfg).unwrap();
        assert_eq!(postcard::from_bytes::<SatLockConfig>(&bytes).unwrap(), cfg);

        let st = SatTrackStatus {
            norad_id: 44909,
            name: "RS-44".into(),
            az_deg: 123.4,
            el_deg: 45.6,
            range_km: 1200.0,
            range_rate_km_s: -3.2,
            doppler_rx_hz: 4650.0,
            doppler_tx_hz: -1557.0,
            downlink_hz: 435.640e6,
            uplink_hz: Some(145.965e6),
            visible: true,
            stale_elements: false,
            corrections_active: true,
            next_pass: Some(SatPass {
                rise_unix: 1_784_937_600,
                set_unix: 1_784_938_500,
                rise_az: 210.0,
                set_az: 20.0,
                max_el: 62.0,
                max_el_unix: 1_784_938_050,
            }),
        };
        let bytes = postcard::to_allocvec(&st).unwrap();
        assert_eq!(postcard::from_bytes::<SatTrackStatus>(&bytes).unwrap(), st);
    }
}
