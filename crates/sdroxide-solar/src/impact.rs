//! Will a CME hit the Earth, and when?
//!
//! This is the question the whole view exists to answer for an HF operator: a
//! CME arriving at Earth drives the geomagnetic storm that wrecks HF
//! propagation a day or two later, and knowing it is coming is actionable in a
//! way that a picture of the Sun is not.
//!
//! The model is deliberately the simple one: constant radial speed from 21.5
//! solar radii, a cone of the fitted half-width, and a hit if the Earth lies
//! inside that cone. It is roughly what an operator would do by hand, and it is
//! honest about being an estimate — real forecasts (WSA-ENLIL) model drag
//! against the solar wind and typically land within ±6 hours.

use crate::donki::CmeAnalysis;
use crate::ephem::{self, AU, SUN_R};
use crate::vec3::Vec3;

/// Height the speed is quoted from: DONKI's `time21_5` is when the leading edge
/// passed 21.5 solar radii.
pub const LAUNCH_RADIUS: f64 = 21.5 * SUN_R;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Impact {
    /// Estimated arrival at the Earth, Unix seconds.
    pub eta_unix: i64,
    /// How far off the cone's axis the Earth sits, degrees. Small means a
    /// direct hit; approaching the half-width means a glancing blow.
    pub off_axis_deg: f64,
    /// Sun-to-Earth transit time, hours.
    pub transit_hours: f64,
}

impl Impact {
    /// 1.0 dead centre, falling to 0 at the cone's edge.
    pub fn directness(&self, half_angle_deg: f64) -> f64 {
        if half_angle_deg <= 0.0 {
            return 0.0;
        }
        (1.0 - self.off_axis_deg / half_angle_deg).clamp(0.0, 1.0)
    }
}

/// How far the leading edge has travelled from the Sun's centre by `now`,
/// gigametres. Never less than the launch radius.
pub fn front_distance(a: &CmeAnalysis, now_unix: i64) -> f64 {
    let elapsed = (now_unix - a.t21_5_unix).max(0) as f64;
    LAUNCH_RADIUS + a.speed_km_s * elapsed / 1.0e6
}

/// Unit vector of the cone's axis, in heliocentric ecliptic coordinates.
///
/// Evaluated in the solar frame **at the event's own `time21_5`**, not at now:
/// the sub-Earth meridian moves about a degree a day and B0 swings ±7.25° over
/// a year, so a month-old CME rendered in today's frame points ~30° wrong.
pub fn axis(a: &CmeAnalysis) -> Vec3 {
    let frame = ephem::sun_frame(ephem::julian_day(a.t21_5_unix as f64));
    frame.direction(a.lat_deg, a.lon_west_deg)
}

/// Does this CME reach the Earth, and when?
///
/// `None` when the Earth is outside the cone, or the speed is not physical.
pub fn earth_impact(a: &CmeAnalysis) -> Option<Impact> {
    if a.speed_km_s <= 0.0 || a.half_angle_deg <= 0.0 {
        return None;
    }
    let dir = axis(a);

    // First pass: assume 1 AU to get an arrival time, then re-evaluate the
    // Earth's actual position at that time. One iteration is plenty — the Earth
    // moves ~1°/day and transits are one to four days.
    let mut eta = a.t21_5_unix + travel_seconds(a, AU);
    for _ in 0..2 {
        let earth = ephem::earth_heliocentric(ephem::julian_day(eta as f64));
        eta = a.t21_5_unix + travel_seconds(a, earth.len());
    }

    let earth = ephem::earth_heliocentric(ephem::julian_day(eta as f64));
    let off_axis = dir.dot(earth.normalize()).clamp(-1.0, 1.0).acos().to_degrees();
    if off_axis > a.half_angle_deg {
        return None;
    }
    Some(Impact {
        eta_unix: eta,
        off_axis_deg: off_axis,
        transit_hours: (eta - a.t21_5_unix) as f64 / 3600.0,
    })
}

fn travel_seconds(a: &CmeAnalysis, distance_gm: f64) -> i64 {
    ((distance_gm - LAUNCH_RADIUS).max(0.0) * 1.0e6 / a.speed_km_s) as i64
}

#[cfg(test)]
mod tests {
    use super::*;

    const T0: i64 = 1_784_937_600; // 2026-07-25T00:00Z

    /// An analysis aimed straight at the Earth.
    ///
    /// Note the latitude: Stonyhurst longitude 0 is the sub-Earth *meridian*,
    /// but latitude 0 is the solar *equator*, and the Earth sits at heliographic
    /// latitude B0 — up to ±7.25° off it. A (0°, 0°) CME therefore misses by B0,
    /// which is exactly what `a_zero_zero_cme_misses_by_b0` below pins down.
    fn earth_directed(speed: f64, half_angle: f64, t: i64) -> CmeAnalysis {
        let b0 = ephem::sun_frame(ephem::julian_day(t as f64)).b0_deg();
        CmeAnalysis {
            t21_5_unix: t,
            lat_deg: b0,
            lon_west_deg: 0.0,
            half_angle_deg: half_angle,
            speed_km_s: speed,
            kind: "C".into(),
            estimated: false,
        }
    }

    /// The latitude-convention exam. If `lat_deg` were measured from the
    /// ecliptic rather than the solar equator, this would come out at zero.
    #[test]
    fn a_zero_zero_cme_misses_by_b0() {
        let mut a = earth_directed(900.0, 45.0, T0);
        a.lat_deg = 0.0;
        let hit = earth_impact(&a).expect("still inside a 45° cone");
        let b0 = ephem::sun_frame(ephem::julian_day(T0 as f64)).b0_deg();
        assert!(b0.abs() > 3.0, "test date has B0 = {b0}, too small to be diagnostic");
        // Off-axis is B0 in latitude plus the Earth's orbital motion during the
        // transit in longitude, added in quadrature.
        assert!(
            (hit.off_axis_deg - b0.abs()).abs() < 1.5,
            "off axis {}° but B0 is {b0}°",
            hit.off_axis_deg
        );
    }

    #[test]
    fn a_head_on_cme_hits_and_the_transit_time_is_physical() {
        let a = earth_directed(800.0, 45.0, T0);
        let hit = earth_impact(&a).expect("a 0°/0° CME must hit the Earth");
        // The journey is 1 AU *minus* the 21.5 R☉ launch radius — 1.35e8 km —
        // which at 800 km/s is a little under 48 hours.
        assert!((hit.transit_hours - 47.6).abs() < 1.5, "transit {} h", hit.transit_hours);
        // Not exactly zero: the Earth moves about 2° along its orbit during the
        // two-day transit, and the axis is fixed at launch.
        assert!(hit.off_axis_deg < 2.5, "off axis by {}°", hit.off_axis_deg);
        assert!(hit.directness(45.0) > 0.94);
        assert!(hit.eta_unix > T0);
    }

    #[test]
    fn faster_cmes_arrive_sooner() {
        let slow = earth_impact(&earth_directed(400.0, 45.0, T0)).unwrap();
        let fast = earth_impact(&earth_directed(2000.0, 45.0, T0)).unwrap();
        assert!(fast.eta_unix < slow.eta_unix);
        // 400 km/s is a ~4-day crawl; 2000 km/s is under a day.
        assert!((90.0..115.0).contains(&slow.transit_hours), "slow {}", slow.transit_hours);
        assert!((17.0..25.0).contains(&fast.transit_hours), "fast {}", fast.transit_hours);
    }

    #[test]
    fn a_back_sided_cme_misses() {
        let mut a = earth_directed(900.0, 30.0, T0);
        a.lon_west_deg = 170.0; // launched from the far side
        assert_eq!(earth_impact(&a), None);
    }

    #[test]
    fn the_cone_half_width_decides_a_glancing_blow() {
        let mut a = earth_directed(900.0, 20.0, T0);
        a.lon_west_deg = 35.0;
        assert_eq!(earth_impact(&a), None, "35° off-axis is outside a 20° cone");
        a.half_angle_deg = 45.0;
        let hit = earth_impact(&a).expect("35° off-axis is inside a 45° cone");
        assert!((hit.off_axis_deg - 35.0).abs() < 3.0, "off axis {}", hit.off_axis_deg);
        // A glancing blow, and reported as one.
        assert!(hit.directness(45.0) < 0.3);
    }

    #[test]
    fn latitude_counts_as_much_as_longitude() {
        let mut a = earth_directed(900.0, 20.0, T0);
        a.lat_deg = 60.0;
        assert_eq!(earth_impact(&a), None, "a 60° polar CME must miss the ecliptic");
    }

    #[test]
    fn the_front_advances_at_the_quoted_speed() {
        let a = earth_directed(1000.0, 45.0, T0);
        assert!((front_distance(&a, T0) - LAUNCH_RADIUS).abs() < 1e-9);
        // 1000 km/s for one hour is 3.6 Gm.
        assert!((front_distance(&a, T0 + 3600) - LAUNCH_RADIUS - 3.6).abs() < 1e-6);
        // Never runs backwards for an event whose epoch is in the future.
        assert_eq!(front_distance(&a, T0 - 99_999), LAUNCH_RADIUS);
    }

    /// The front must actually be at the Earth's distance when the estimated
    /// arrival comes round — the internal consistency of the two calculations.
    #[test]
    fn the_front_reaches_the_earth_exactly_at_the_eta() {
        for speed in [350.0, 700.0, 1500.0, 2500.0] {
            let a = earth_directed(speed, 45.0, T0);
            let hit = earth_impact(&a).unwrap();
            let front = front_distance(&a, hit.eta_unix);
            let earth = ephem::earth_heliocentric(ephem::julian_day(hit.eta_unix as f64)).len();
            assert!(
                (front - earth).abs() < 0.05,
                "{speed} km/s: front at {front} Gm, Earth at {earth} Gm"
            );
        }
    }

    #[test]
    fn nonsense_input_yields_no_impact() {
        assert_eq!(earth_impact(&earth_directed(0.0, 45.0, T0)), None);
        assert_eq!(earth_impact(&earth_directed(-100.0, 45.0, T0)), None);
        assert_eq!(earth_impact(&earth_directed(900.0, 0.0, T0)), None);
    }

    /// The axis is built in the solar frame of the event's own epoch. Two
    /// identical CMEs a month apart therefore point in measurably different
    /// inertial directions — the bug this guards against is evaluating both in
    /// "now"'s frame, which silently mis-aims every historical event.
    #[test]
    fn the_axis_uses_the_events_own_epoch() {
        let a = earth_directed(900.0, 45.0, T0);
        let b = earth_directed(900.0, 45.0, T0 - 30 * 86_400);
        let angle = axis(&a).dot(axis(&b)).clamp(-1.0, 1.0).acos().to_degrees();
        // A month of the Earth's orbital motion is about 30°.
        assert!((angle - 29.0).abs() < 3.0, "axes differ by {angle}°, expected ~29°");
    }
}
