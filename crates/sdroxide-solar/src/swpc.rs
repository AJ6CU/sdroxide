//! NOAA SWPC's daily Solar Region Summary — the actual sunspot groups.
//!
//! <https://services.swpc.noaa.gov/json/solar_regions.json>, no API key.
//!
//! DONKI has no sunspot endpoint; its closest data is flare source regions,
//! which only exist for regions that have actually flared. This gives every
//! numbered group with its position, area, spot count and magnetic class — so
//! the markers can be sized by real sunspot area and still be drawn when the
//! app is offline.
//!
//! **The numeric `longitude` here is East-positive, the opposite of DONKI's.**
//! Rather than trust that, the `location` string (`N05W46`) is parsed first
//! because it spells the hemisphere out; the numeric fields are only a
//! fallback. Everything leaving this module is West-positive.

use serde::{Deserialize, Serialize};

use crate::helio::parse_location;
use crate::timefmt;

pub const REGIONS_URL: &str = "https://services.swpc.noaa.gov/json/solar_regions.json";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ActiveRegion {
    /// NOAA region number, e.g. 4493.
    pub number: u32,
    pub observed_unix: i64,
    pub lat_deg: f64,
    /// Stonyhurst longitude, **West-positive** (SWPC's own field is negated).
    pub lon_west_deg: f64,
    /// Carrington longitude, which unlike the Stonyhurst one does not move as
    /// the Sun rotates — this is what lets a region be tracked over the limb.
    pub carrington_deg: f64,
    /// Corrected area in millionths of a solar hemisphere. `None` for a plage
    /// region with no spots, which is a third of the list on a quiet day.
    pub area: Option<u32>,
    pub num_spots: Option<u32>,
    /// McIntosh class, e.g. `Eki`.
    pub spot_class: Option<String>,
    /// Mount Wilson magnetic class, e.g. `BG`.
    pub mag_class: Option<String>,
    /// NOAA's next-24-hour flare probabilities, percent.
    pub c_prob: f64,
    pub m_prob: f64,
    pub x_prob: f64,
}

impl ActiveRegion {
    /// Angular radius of the spot group as a fraction of the solar radius.
    ///
    /// Area is in millionths of a *hemisphere*, so a group of area `A` covers
    /// `A·1e-6·2πR²`; equating that to a circular patch `πr²` gives
    /// `r/R = sqrt(2A·1e-6)`. Clamped to something visible.
    pub fn angular_radius(&self) -> f64 {
        let a = self.area.unwrap_or(0) as f64;
        ((2.0e-6 * a).sqrt()).clamp(0.012, 0.25)
    }

    /// Highest of the three flare probabilities, for colour-coding.
    pub fn threat(&self) -> f64 {
        self.x_prob.max(self.m_prob).max(self.c_prob) / 100.0
    }
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct RawRegion {
    observed_date: Option<String>,
    region: Option<u32>,
    latitude: Option<f64>,
    longitude: Option<f64>,
    location: Option<String>,
    carrington_longitude: Option<f64>,
    area: Option<u32>,
    number_spots: Option<u32>,
    spot_class: Option<String>,
    mag_class: Option<String>,
    c_flare_probability: Option<f64>,
    m_flare_probability: Option<f64>,
    x_flare_probability: Option<f64>,
}

/// Parse the region summary, keeping only the most recent report.
///
/// The file holds a rolling history — around 200 records covering several weeks
/// — and only the newest day describes the Sun as it is now.
pub fn parse_regions(json: &str) -> Result<Vec<ActiveRegion>, serde_json::Error> {
    let raw: Vec<RawRegion> = serde_json::from_str(json)?;
    let mut all: Vec<ActiveRegion> = raw.into_iter().filter_map(convert).collect();

    let latest = all.iter().map(|r| r.observed_unix).max().unwrap_or(0);
    all.retain(|r| r.observed_unix == latest);
    all.sort_by_key(|r| r.number);
    Ok(all)
}

fn convert(r: RawRegion) -> Option<ActiveRegion> {
    let observed_unix = timefmt::parse_unix(r.observed_date.as_deref()?)?;
    // The string form first: it cannot be sign-flipped by a misreading.
    let (lat_deg, lon_west_deg) = match r.location.as_deref().and_then(parse_location) {
        Some(v) => v,
        None => (r.latitude?, -r.longitude?),
    };
    Some(ActiveRegion {
        number: r.region?,
        observed_unix,
        lat_deg,
        lon_west_deg,
        carrington_deg: r.carrington_longitude.unwrap_or(0.0),
        area: r.area,
        num_spots: r.number_spots,
        spot_class: r.spot_class,
        mag_class: r.mag_class,
        c_prob: r.c_flare_probability.unwrap_or(0.0),
        m_prob: r.m_flare_probability.unwrap_or(0.0),
        x_prob: r.x_flare_probability.unwrap_or(0.0),
    })
}

/// The Carrington longitude of the disk centre implied by a set of regions.
///
/// `L_carrington = L0 + L_stonyhurst_west`, so every region in a report agrees
/// on `L0`. Comparing that against [`crate::ephem::solar_p_b0_l0`] is an
/// end-to-end check of the whole heliographic chain against NOAA's own numbers.
pub fn implied_l0(regions: &[ActiveRegion]) -> Option<f64> {
    if regions.is_empty() {
        return None;
    }
    // Average as unit vectors so the 0/360 wrap does not skew the mean.
    let (mut sx, mut sy) = (0.0, 0.0);
    for r in regions {
        let a = (r.carrington_deg - r.lon_west_deg).to_radians();
        sx += a.cos();
        sy += a.sin();
    }
    Some(crate::ephem::wrap360(sy.atan2(sx).to_degrees()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const REGIONS_JSON: &str = include_str!("../tests/fixtures/regions.json");

    #[test]
    fn parses_the_real_feed_including_spotless_regions() {
        let regions = parse_regions(REGIONS_JSON).expect("fixture must parse");
        assert!(!regions.is_empty());
        for r in &regions {
            assert!(r.number > 0);
            assert!((-90.0..=90.0).contains(&r.lat_deg), "{}: lat {}", r.number, r.lat_deg);
            assert!((-180.0..=180.0).contains(&r.lon_west_deg));
            assert!(r.angular_radius() > 0.0);
        }
        // A third of a real report is plage with no spots at all.
        assert!(regions.iter().any(|r| r.area.is_none()), "fixture lost its spotless regions");
        assert!(regions.iter().any(|r| r.area.is_some()));
        // Only the newest report survives.
        let days: std::collections::HashSet<_> = regions.iter().map(|r| r.observed_unix).collect();
        assert_eq!(days.len(), 1, "kept {} different report days", days.len());
    }

    /// SWPC's numeric `longitude` is East-positive while ours is West-positive.
    /// The `location` string is authoritative, so this asserts the numeric
    /// field is exactly its negation — the check that would fail loudly if the
    /// convention ever changed under us.
    #[test]
    fn swpc_numeric_longitude_is_the_negation_of_ours() {
        #[derive(Deserialize)]
        struct Probe {
            latitude: Option<f64>,
            longitude: Option<f64>,
            location: Option<String>,
        }
        let probes: Vec<Probe> = serde_json::from_str(REGIONS_JSON).unwrap();
        let mut checked = 0;
        for p in &probes {
            let (Some((lat, lon_west)), Some(nlat), Some(nlon)) =
                (p.location.as_deref().and_then(parse_location), p.latitude, p.longitude)
            else {
                continue;
            };
            assert_eq!(lat, nlat, "latitude disagrees with {:?}", p.location);
            assert_eq!(
                lon_west, -nlon,
                "SWPC longitude {nlon} is not the negation of {:?}",
                p.location
            );
            checked += 1;
        }
        assert!(checked > 3, "only {checked} regions cross-checked");
    }

    /// End-to-end validation of the heliographic chain: NOAA's own Carrington
    /// longitudes imply a disk-centre longitude, and our ephemeris must agree.
    #[test]
    fn regions_agree_with_our_ephemeris_on_l0() {
        let regions = parse_regions(REGIONS_JSON).unwrap();
        let implied = implied_l0(&regions).expect("fixture has regions");
        // The report's positions are for 2400 UT of the observation day.
        let jd = crate::ephem::julian_day(regions[0].observed_unix as f64 + 86_400.0);
        let (_, _, ours) = crate::ephem::solar_p_b0_l0(jd);
        let diff = crate::ephem::wrap180(implied - ours).abs();
        assert!(diff < 2.0, "NOAA implies L0 = {implied}, we compute {ours} ({diff}° apart)");
    }

    #[test]
    fn area_maps_to_a_sensible_spot_size() {
        let region = |area: Option<u32>| ActiveRegion {
            number: 1,
            observed_unix: 0,
            lat_deg: 0.0,
            lon_west_deg: 0.0,
            carrington_deg: 0.0,
            area,
            num_spots: None,
            spot_class: None,
            mag_class: None,
            c_prob: 0.0,
            m_prob: 0.0,
            x_prob: 0.0,
        };
        // A 400-millionths group is about 2.8% of the solar radius across.
        assert!((region(Some(400)).angular_radius() - 0.0283).abs() < 0.002);
        // Spotless and enormous both stay inside the drawable range.
        assert!(region(None).angular_radius() >= 0.012);
        assert!(region(Some(4000)).angular_radius() <= 0.25);
    }

    #[test]
    fn empty_input_is_survivable() {
        assert!(parse_regions("[]").unwrap().is_empty());
        assert!(implied_l0(&[]).is_none());
        assert!(parse_regions("nonsense").is_err());
    }
}
