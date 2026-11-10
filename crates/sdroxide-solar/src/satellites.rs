//! Amateur-radio satellite tracking: TLEs from CelesTrak, propagated with SGP4.
//!
//! # Reference frames
//!
//! SGP4 returns TEME (True Equator, Mean Equinox) positions — an *equatorial*
//! frame that does not rotate with the Earth. Two conversions come out of that,
//! and it matters which is used where:
//!
//! * **Position in space**, for the marker and the orbit ring: TEME → ecliptic
//!   is just the obliquity rotation. No sidereal time involved, which is exactly
//!   right — an orbit is fixed in inertial space and should sit still while the
//!   Earth turns inside it.
//! * **Sub-satellite point**, for the ground position and footprint: TEME →
//!   ECEF, which *does* need the sidereal rotation.
//!
//! Getting these two the wrong way round produces an orbit that visibly
//! corkscrews as the Earth rotates, which is the classic bug here.

use crate::ephem::{self, wrap180};
use crate::vec3::{Vec3, vec3};

/// Earth equatorial radius, km — the reference SGP4 positions are relative to.
pub const EARTH_R_KM: f64 = 6378.137;

/// CelesTrak's amateur-satellite element set.
pub const AMATEUR_URL: &str =
    "https://celestrak.org/NORAD/elements/gp.php?GROUP=amateur&FORMAT=tle";

/// QO-100 / Es'hail-2 is a geostationary commercial satellite carrying an
/// amateur transponder, so it is absent from the amateur group and has to be
/// requested by catalogue number.
pub const QO100_URL: &str = "https://celestrak.org/NORAD/elements/gp.php?CATNR=43700&FORMAT=tle";
pub const QO100_NORAD: u64 = 43700;

/// The satellites shown by default, by catalogue number.
///
/// Everything in the amateur element set is tracked, but drawing ninety orbit
/// rings at once is unreadable; these are the ones an operator is most likely to
/// actually work. QO-100 leads because its geostationary ring is the one orbit
/// in the set that never moves.
pub const POPULAR: &[(u64, &str)] = &[
    (QO100_NORAD, "QO-100"),
    (25544, "ISS"),
    (7530, "AO-7"),
    (24278, "FO-29"),
    (27607, "SO-50"),
    (39444, "AO-73"),
    (43803, "JO-97"),
    (44909, "RS-44"),
    (50466, "XW-3"),
    (53109, "IO-117"),
];

/// A satellite with its propagator ready to run.
pub struct Satellite {
    pub norad_id: u64,
    /// The catalogue name, shortened to its amateur designator where there is
    /// one — `FUNCUBE-1 (AO-73)` is listed as `AO-73`.
    pub name: String,
    pub epoch_unix: i64,
    /// Orbital period in minutes, from the mean motion.
    pub period_min: f64,
    /// Whether this one is in [`POPULAR`].
    pub popular: bool,
    /// Whether the operator supplied the element set themselves. Custom
    /// satellites are drawn like the curated ones — someone who pasted a TLE in
    /// by hand wants to see that satellite, not a dot among ninety others.
    pub custom: bool,
    constants: sgp4::Constants,
}

/// Where a satellite is at one instant.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SatState {
    /// Unit direction in the **ecliptic** frame — ready to place in the scene.
    pub dir_ecliptic: Vec3,
    /// Geocentric distance in Earth radii, so the renderer can scale it by
    /// whatever radius the exaggerated Earth is being drawn at.
    pub radii: f64,
    /// Altitude above the equatorial radius, km.
    pub alt_km: f64,
    /// Sub-satellite point: geodetic latitude and longitude, degrees.
    pub lat: f64,
    pub lon: f64,
    /// Speed, km/s.
    pub speed_km_s: f64,
}

impl SatState {
    /// Half-angle of the satellite's footprint as seen from the Earth's centre,
    /// degrees — the radius of the circle it can be worked from.
    pub fn footprint_deg(&self) -> f64 {
        (1.0 / self.radii.max(1.0)).acos().to_degrees()
    }

    /// Look angles from an observer: `(azimuth, elevation)` in degrees, azimuth
    /// measured clockwise from true north.
    ///
    /// Spherical-Earth geometry. The error against a full geodetic reduction is
    /// a fraction of a degree — irrelevant next to the couple of degrees of TLE
    /// uncertainty that a day-old element set already carries.
    pub fn az_el_from(&self, lat: f64, lon: f64) -> (f64, f64) {
        let (p1, p2) = (lat.to_radians(), self.lat.to_radians());
        let dl = (self.lon - lon).to_radians();
        // Angular distance from the observer to the sub-satellite point.
        let cos_c = (p1.sin() * p2.sin() + p1.cos() * p2.cos() * dl.cos()).clamp(-1.0, 1.0);
        let c = cos_c.acos();
        // Standard look-angle geometry: elevation falls to zero when the
        // satellite is exactly at the geometric horizon, cos c = 1/r.
        let el = (cos_c - 1.0 / self.radii).atan2(c.sin()).to_degrees();
        let az = (dl.sin() * p2.cos())
            .atan2(p1.cos() * p2.sin() - p1.sin() * p2.cos() * dl.cos())
            .to_degrees();
        ((az + 360.0) % 360.0, el)
    }

    /// Elevation above an observer's horizon, degrees. Negative is below it.
    pub fn elevation_from(&self, lat: f64, lon: f64) -> f64 {
        self.az_el_from(lat, lon).1
    }
}

/// One visible pass over an observer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Pass {
    pub rise_unix: i64,
    pub set_unix: i64,
    /// Azimuth at the horizon, degrees clockwise from north — which way to
    /// point the antenna to acquire, and where it will end up.
    pub rise_az: f64,
    pub set_az: f64,
    /// Highest elevation reached, and when.
    pub max_el: f64,
    pub max_el_unix: i64,
    pub max_el_az: f64,
}

impl Pass {
    pub fn duration_s(&self) -> i64 {
        (self.set_unix - self.rise_unix).max(0)
    }

    /// A pass that barely clears the horizon is workable in theory and often
    /// not in practice; one over 30° is a good one.
    pub fn quality(&self) -> &'static str {
        match self.max_el {
            e if e >= 60.0 => "overhead",
            e if e >= 30.0 => "good",
            e if e >= 15.0 => "fair",
            _ => "marginal",
        }
    }
}

/// The result of a pass search — the geostationary case is not an empty list.
#[derive(Clone, Debug, PartialEq)]
pub enum PassSearch {
    Passes(Vec<Pass>),
    /// Above the horizon for the whole search window: a geostationary bird like
    /// QO-100, which has no passes because it never sets.
    AlwaysVisible {
        elevation: f64,
        azimuth: f64,
    },
    /// Never rises during the search window — the wrong hemisphere, or an orbit
    /// whose inclination never reaches the observer's latitude.
    NeverVisible,
}

impl Satellite {
    /// Find the next passes over an observer.
    ///
    /// Coarse-steps the elevation looking for horizon crossings, then bisects
    /// each one to the second. The step is a fraction of the orbital period, so
    /// a 90-minute LEO orbit is sampled finely enough not to miss a short pass
    /// while a geostationary one is not sampled pointlessly hard.
    pub fn next_passes(
        &self,
        lat: f64,
        lon: f64,
        from_unix: f64,
        span_s: f64,
        max_passes: usize,
    ) -> PassSearch {
        if self.period_min <= 0.0 || max_passes == 0 {
            return PassSearch::NeverVisible;
        }
        // A pass lasts a few percent of the period; sampling at a hundredth of
        // it cannot step over one.
        let step = (self.period_min * 60.0 / 100.0).clamp(15.0, 300.0);
        let el_at = |t: f64| self.at(t).map(|s| s.az_el_from(lat, lon).1);

        let Some(mut prev_el) = el_at(from_unix) else {
            return PassSearch::NeverVisible;
        };
        let mut passes: Vec<Pass> = Vec::new();
        // If the search starts mid-pass, the rise already happened; report the
        // pass from the search start rather than pretending it begins later.
        let mut rise: Option<f64> = (prev_el > 0.0).then_some(from_unix);
        let mut ever_below = prev_el <= 0.0;
        let mut t = from_unix;

        while t < from_unix + span_s && passes.len() < max_passes {
            let next = t + step;
            let Some(el) = el_at(next) else { break };
            if el <= 0.0 {
                ever_below = true;
            }
            match (prev_el <= 0.0, el > 0.0) {
                // Rising through the horizon.
                (true, true) => rise = Some(self.cross(lat, lon, t, next, true)),
                // Setting.
                (false, false) => {
                    if let Some(r) = rise.take() {
                        let s = self.cross(lat, lon, t, next, false);
                        if let Some(p) = self.summarise(lat, lon, r, s) {
                            passes.push(p);
                        }
                    }
                }
                _ => {}
            }
            prev_el = el;
            t = next;
        }

        if !ever_below {
            // Never dipped below the horizon anywhere in the window.
            return match self.at(from_unix) {
                Some(s) => {
                    let (az, el) = s.az_el_from(lat, lon);
                    PassSearch::AlwaysVisible { elevation: el, azimuth: az }
                }
                None => PassSearch::NeverVisible,
            };
        }
        if passes.is_empty() {
            return PassSearch::NeverVisible;
        }
        PassSearch::Passes(passes)
    }

    /// Bisect a bracketed horizon crossing to within a second.
    fn cross(&self, lat: f64, lon: f64, mut lo: f64, mut hi: f64, rising: bool) -> f64 {
        let el = |t: f64| self.at(t).map(|s| s.az_el_from(lat, lon).1).unwrap_or(0.0);
        for _ in 0..40 {
            if hi - lo <= 1.0 {
                break;
            }
            let mid = 0.5 * (lo + hi);
            // Above the horizon at the midpoint moves whichever end keeps the
            // crossing bracketed.
            if (el(mid) > 0.0) == rising { hi = mid } else { lo = mid }
        }
        0.5 * (lo + hi)
    }

    /// Fill in the peak of a pass whose endpoints are known.
    fn summarise(&self, lat: f64, lon: f64, rise: f64, set: f64) -> Option<Pass> {
        const SAMPLES: usize = 60;
        let mut best = (f64::MIN, rise);
        for k in 0..=SAMPLES {
            let t = rise + (set - rise) * k as f64 / SAMPLES as f64;
            if let Some(s) = self.at(t) {
                let el = s.az_el_from(lat, lon).1;
                if el > best.0 {
                    best = (el, t);
                }
            }
        }
        let (r_az, _) = self.at(rise)?.az_el_from(lat, lon);
        let (s_az, _) = self.at(set)?.az_el_from(lat, lon);
        let (m_az, m_el) = self.at(best.1)?.az_el_from(lat, lon);
        Some(Pass {
            rise_unix: rise.round() as i64,
            set_unix: set.round() as i64,
            rise_az: r_az,
            set_az: s_az,
            max_el: m_el,
            max_el_unix: best.1.round() as i64,
            max_el_az: m_az,
        })
    }
}

/// Compass point for an azimuth, for a readable table.
pub fn compass(az_deg: f64) -> &'static str {
    const POINTS: [&str; 16] = [
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW",
        "NW", "NNW",
    ];
    let i = (((az_deg % 360.0 + 360.0) % 360.0) / 22.5).round() as usize % 16;
    POINTS[i]
}

/// Trim a CelesTrak catalogue name to its amateur designator where it has one.
///
/// `FUNCUBE-1 (AO-73)` → `AO-73`, `RS-44 & BREEZE-KM R/B` → `RS-44`,
/// `ISS (ZARYA)` → `ISS`.
fn short_name(raw: &str, norad_id: u64) -> String {
    if let Some((_, id)) = POPULAR.iter().find(|(n, _)| *n == norad_id) {
        return (*id).to_string();
    }
    let raw = raw.trim();
    if let (Some(open), Some(close)) = (raw.find('('), raw.rfind(')')) {
        if close > open + 1 {
            return raw[open + 1..close].trim().to_string();
        }
    }
    raw.split('&').next().unwrap_or(raw).trim().to_string()
}

/// Parse element sets the operator pasted in by hand.
///
/// Every satellite comes back flagged [`Satellite::custom`] *and* popular: they
/// are drawn and labelled like the curated ones, because typing a TLE in by
/// hand is a stronger statement of interest than any built-in list.
pub fn parse_pasted_tles(text: &str) -> Vec<Satellite> {
    let mut v = parse_subscribed_tles(text);
    for s in &mut v {
        s.popular = true;
    }
    v
}

/// Parse element sets from a subscribed listing.
///
/// Flagged [`Satellite::custom`] — the operator asked for this listing — but
/// `popular` is left as the built-in curated list set it. A group endpoint is
/// ninety satellites, and drawing ninety rings and labels because the operator
/// subscribed to one listing would make the view unreadable. The subscription's
/// own orbit-ring switch is what raises it.
pub fn parse_subscribed_tles(text: &str) -> Vec<Satellite> {
    let mut v = parse_tles(text);
    for s in &mut v {
        s.custom = true;
    }
    v
}

/// Parse a CelesTrak 3-line element listing.
///
/// Elements that SGP4 rejects are skipped rather than failing the batch: a
/// decayed or malformed entry should not cost the other ninety.
///
/// Repeated catalogue numbers keep the **first** entry. A single CelesTrak
/// group never repeats one, but several concatenated do — the stations group
/// and the cubesat group share plenty with the amateur one — and a duplicate
/// would be a second marker drawn a few metres from the first.
pub fn parse_tles(text: &str) -> Vec<Satellite> {
    // Blank lines are fatal to a three-line parser, which counts rather than
    // looks. They turn up wherever two listings have been concatenated — which
    // is exactly what the subscribed groups are handed over as — so they are
    // dropped here rather than costing the whole batch.
    let text: String = text.lines().filter(|l| !l.trim().is_empty()).collect::<Vec<_>>().join("\n");
    let elements = match sgp4::parse_3les(&text) {
        Ok(e) => e,
        Err(e) => {
            tracing::warn!("satellite element set did not parse: {e}");
            return Vec::new();
        }
    };
    elements
        .into_iter()
        .filter_map(|el| {
            let constants = sgp4::Constants::from_elements(&el)
                .map_err(|e| tracing::debug!("skipping satellite {}: {e}", el.norad_id))
                .ok()?;
            // Mean motion is revolutions per day.
            let period_min = if el.mean_motion > 0.0 { 1440.0 / el.mean_motion } else { 0.0 };
            let raw = el.object_name.clone().unwrap_or_default();
            Some(Satellite {
                norad_id: el.norad_id,
                name: short_name(&raw, el.norad_id),
                epoch_unix: el.datetime.and_utc().timestamp(),
                period_min,
                popular: POPULAR.iter().any(|(n, _)| *n == el.norad_id),
                custom: false,
                constants,
            })
        })
        .fold(Vec::new(), |mut acc, s: Satellite| {
            if !acc.iter().any(|e: &Satellite| e.norad_id == s.norad_id) {
                acc.push(s);
            }
            acc
        })
}

/// Elements older than this are not worth propagating: SGP4 accuracy decays
/// quickly, and a fortnight-old TLE can be tens of kilometres out.
pub const MAX_ELEMENT_AGE_S: i64 = 14 * 86_400;

impl Satellite {
    /// Age of the elements at `unix_s`, in seconds. Large values mean the
    /// position is extrapolated well past where SGP4 is trustworthy.
    pub fn element_age_s(&self, unix_s: f64) -> f64 {
        unix_s - self.epoch_unix as f64
    }

    /// Position at an instant, or `None` if SGP4 fails (a decayed orbit) or the
    /// elements are too old to mean anything.
    pub fn at(&self, unix_s: f64) -> Option<SatState> {
        if self.element_age_s(unix_s).abs() > MAX_ELEMENT_AGE_S as f64 {
            return None;
        }
        let minutes = (unix_s - self.epoch_unix as f64) / 60.0;
        let p = self.constants.propagate(sgp4::MinutesSinceEpoch(minutes)).ok()?;
        let teme = vec3(p.position[0], p.position[1], p.position[2]);
        let r_km = teme.len();
        if !r_km.is_finite() || r_km < EARTH_R_KM * 0.9 {
            return None; // re-entered, or a numerically broken element set
        }

        let jd = ephem::julian_day(unix_s);
        // Space: the obliquity rotation only. The orbit is inertial, so it must
        // not pick up the Earth's rotation.
        let dir_ecliptic = ephem::equatorial_to_ecliptic(teme, jd).normalize();
        // Ground: this one *does* need sidereal time.
        let ecef = teme_to_ecef(teme, jd);
        Some(SatState {
            dir_ecliptic,
            radii: r_km / EARTH_R_KM,
            alt_km: r_km - EARTH_R_KM,
            lat: (ecef.z / r_km).clamp(-1.0, 1.0).asin().to_degrees(),
            lon: wrap180(ecef.y.atan2(ecef.x).to_degrees()),
            speed_km_s: vec3(p.velocity[0], p.velocity[1], p.velocity[2]).len(),
        })
    }

    /// The orbit as a closed path of ecliptic-frame unit directions paired with
    /// their geocentric distance in Earth radii.
    ///
    /// Sampled over one full period *centred on now*, so what is drawn is the
    /// orbit the satellite is actually on rather than a mean ellipse.
    pub fn orbit(&self, unix_s: f64, steps: usize) -> Vec<(Vec3, f64)> {
        if self.period_min <= 0.0 || steps < 3 {
            return Vec::new();
        }
        let span = self.period_min * 60.0;
        (0..=steps)
            .filter_map(|k| {
                let t = unix_s + span * (k as f64 / steps as f64 - 0.5);
                self.at(t).map(|s| (s.dir_ecliptic, s.radii))
            })
            .collect()
    }
}

/// A satellite as seen from a ground station: where to point, how far, and how
/// fast the distance is changing.
///
/// This is the *operating* view, as opposed to [`SatState`], which is the
/// *display* view. The difference that matters is `range_rate_km_s`: Doppler
/// correction needs the radial velocity toward the observer, which
/// [`Satellite::at`] collapses into a scalar speed. The look angles here come
/// from the true topocentric geometry rather than [`SatState::az_el_from`]'s
/// sub-satellite-point approximation — the two agree to a fraction of a
/// degree, but this one comes for free once the vectors exist.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Observation {
    /// Degrees clockwise from true north.
    pub az_deg: f64,
    /// Degrees above the horizon; negative below it.
    pub el_deg: f64,
    pub range_km: f64,
    /// Positive receding. `-range_rate/c` scaled by the frequency is the
    /// receive Doppler correction.
    pub range_rate_km_s: f64,
}

/// The Earth's rotation rate, rad/s — needed to carry a velocity from the
/// inertial TEME frame into the rotating ECEF frame.
const OMEGA_EARTH: f64 = 7.292_115e-5;

impl Satellite {
    /// The full topocentric look from a ground observer, or `None` under the
    /// same conditions [`Satellite::at`] gives it: a decayed orbit, or
    /// elements too old to trust — which for an *operating* quantity like
    /// Doppler is a feature, not a gap.
    pub fn observe(&self, unix_s: f64, lat_deg: f64, lon_deg: f64) -> Option<Observation> {
        if self.element_age_s(unix_s).abs() > MAX_ELEMENT_AGE_S as f64 {
            return None;
        }
        let minutes = (unix_s - self.epoch_unix as f64) / 60.0;
        let p = self.constants.propagate(sgp4::MinutesSinceEpoch(minutes)).ok()?;
        let teme_r = vec3(p.position[0], p.position[1], p.position[2]);
        let teme_v = vec3(p.velocity[0], p.velocity[1], p.velocity[2]);
        if !teme_r.len().is_finite() || teme_r.len() < EARTH_R_KM * 0.9 {
            return None;
        }

        let jd = ephem::julian_day(unix_s);
        let r = teme_to_ecef(teme_r, jd);
        // A velocity does not rotate into a rotating frame the way a position
        // does: the frame itself moves, and the transport term ω⊕ × r it
        // contributes is up to half a kilometre per second — a couple of
        // hundred hertz of Doppler at UHF, so not ignorable.
        let v = teme_to_ecef(teme_v, jd) - vec3(0.0, 0.0, OMEGA_EARTH).cross(r);

        // The observer, on the spherical Earth the rest of this module uses.
        // The flattening this ignores moves the answer by less than the
        // day-old element set already has.
        let (lat, lon) = (lat_deg.to_radians(), lon_deg.to_radians());
        let up = vec3(lat.cos() * lon.cos(), lat.cos() * lon.sin(), lat.sin());
        let obs = up * EARTH_R_KM;

        let rho = r - obs;
        let range = rho.len();
        if range <= 0.0 || !range.is_finite() {
            return None;
        }
        // The observer is fixed in ECEF, so the relative velocity is `v`.
        let range_rate = rho.dot(v) / range;

        let east = vec3(-lon.sin(), lon.cos(), 0.0);
        let north = up.cross(east);
        let az = rho.dot(east).atan2(rho.dot(north)).to_degrees();
        let el = (rho.dot(up) / range).clamp(-1.0, 1.0).asin().to_degrees();
        Some(Observation {
            az_deg: (az + 360.0) % 360.0,
            el_deg: el,
            range_km: range,
            range_rate_km_s: range_rate,
        })
    }
}

/// TEME → ECEF: a rotation by Greenwich sidereal time about the polar axis.
///
/// Mean rather than apparent sidereal time, and polar motion is ignored; both
/// are arcsecond-scale, far below anything this display resolves.
fn teme_to_ecef(teme: Vec3, jd: f64) -> Vec3 {
    let g = ephem::gmst_deg(jd).to_radians();
    vec3(teme.x * g.cos() + teme.y * g.sin(), -teme.x * g.sin() + teme.y * g.cos(), teme.z)
}

#[cfg(test)]
mod tests {
    use super::*;

    const AMATEUR: &str = include_str!("../tests/fixtures/amateur.txt");
    const QO100: &str = include_str!("../tests/fixtures/qo100.txt");

    /// Just after the fixture epochs, so nothing is stale.
    const NOW: f64 = 1_784_937_600.0; // 2026-07-25T00:00Z

    /// Concatenated listings overlap; the first entry for a catalogue number
    /// wins and the rest are dropped, or the sky would have two of everything.
    #[test]
    fn a_repeated_catalogue_number_is_kept_only_once() {
        let one = parse_tles(AMATEUR);
        let twice = parse_tles(&format!("{AMATEUR}\n\n{AMATEUR}"));
        assert_eq!(twice.len(), one.len(), "concatenating a listing with itself duplicated it");
        // ...and the survivors are the same satellites, in the same order.
        let a: Vec<u64> = one.iter().map(|s| s.norad_id).collect();
        let b: Vec<u64> = twice.iter().map(|s| s.norad_id).collect();
        assert_eq!(a, b);
    }

    #[test]
    fn parses_the_celestrak_amateur_set() {
        let sats = parse_tles(AMATEUR);
        assert!(sats.len() > 80, "only {} satellites", sats.len());
        for s in &sats {
            assert!(!s.name.is_empty());
            assert!(s.norad_id > 0);
            // Every amateur satellite is in Earth orbit: periods run from ~90
            // minutes in LEO to a day at geostationary.
            assert!((80.0..1600.0).contains(&s.period_min), "{}: {} min", s.name, s.period_min);
        }
        // The curated ones are all present and flagged.
        for (id, name) in POPULAR.iter().filter(|(id, _)| *id != QO100_NORAD) {
            let s = sats
                .iter()
                .find(|s| s.norad_id == *id)
                .unwrap_or_else(|| panic!("{name} ({id}) missing from the amateur set"));
            assert!(s.popular);
            assert_eq!(s.name, *name);
        }
    }

    #[test]
    fn names_are_shortened_to_their_amateur_designator() {
        assert_eq!(short_name("FUNCUBE-1 (AO-73)", 39444), "AO-73");
        assert_eq!(short_name("ISS (ZARYA)", 25544), "ISS");
        assert_eq!(short_name("RS-44 & BREEZE-KM R/B", 44909), "RS-44");
        // Not in the curated list: fall back to the parenthetical, then the
        // part before an ampersand, then the raw name.
        assert_eq!(short_name("SOMESAT (XX-99)", 999_999), "XX-99");
        assert_eq!(short_name("DEBRIS & R/B", 999_999), "DEBRIS");
        assert_eq!(short_name("SWISSCUBE", 999_999), "SWISSCUBE");
        assert_eq!(short_name("BROKEN (", 999_999), "BROKEN (");
    }

    /// QO-100 is the reference case: geostationary, so its altitude and its
    /// sub-satellite longitude are both known constants.
    #[test]
    fn qo100_sits_where_a_geostationary_satellite_should() {
        let sats = parse_tles(QO100);
        assert_eq!(sats.len(), 1);
        let qo = &sats[0];
        assert_eq!(qo.norad_id, QO100_NORAD);
        assert_eq!(qo.name, "QO-100");
        // A geostationary orbit is one sidereal day.
        assert!((qo.period_min - 1436.1).abs() < 2.0, "period {} min", qo.period_min);

        let s = qo.at(NOW).expect("QO-100 must propagate");
        // Geostationary altitude is 35 786 km, i.e. 6.61 Earth radii out.
        assert!((s.alt_km - 35_786.0).abs() < 250.0, "altitude {} km", s.alt_km);
        assert!((s.radii - 6.611).abs() < 0.05, "{} Earth radii", s.radii);
        // Over the equator...
        assert!(s.lat.abs() < 0.5, "latitude {}", s.lat);
        // ...and parked at 25.9°E, which is what makes it usable from Europe.
        assert!((s.lon - 25.9).abs() < 2.0, "longitude {}", s.lon);
        // Its footprint covers most of a hemisphere.
        assert!((s.footprint_deg() - 81.3).abs() < 1.0, "footprint {}", s.footprint_deg());
    }

    /// The test that distinguishes the two frame conversions. A geostationary
    /// satellite stays over the same *ground* longitude while its *inertial*
    /// direction sweeps a full circle every day. Swapping the conversions makes
    /// the orbit corkscrew and the ground track drift.
    #[test]
    fn a_geostationary_orbit_is_fixed_on_the_ground_and_turns_in_space() {
        let qo = &parse_tles(QO100)[0];
        let a = qo.at(NOW).unwrap();
        let b = qo.at(NOW + 6.0 * 3600.0).unwrap();

        // Ground: unchanged after six hours.
        assert!(
            wrap180(b.lon - a.lon).abs() < 1.0,
            "sub-satellite longitude drifted {}° in 6 h",
            wrap180(b.lon - a.lon)
        );
        // Space: a quarter turn of the ecliptic direction.
        let swept = a.dir_ecliptic.dot(b.dir_ecliptic).clamp(-1.0, 1.0).acos().to_degrees();
        assert!((swept - 90.0).abs() < 2.0, "inertial direction swept {swept}° in 6 h");
    }

    #[test]
    fn the_iss_is_in_a_plausible_low_earth_orbit() {
        let sats = parse_tles(AMATEUR);
        let iss = sats.iter().find(|s| s.norad_id == 25544).expect("ISS in the set");
        let s = iss.at(NOW).expect("ISS must propagate");
        assert!((380.0..470.0).contains(&s.alt_km), "ISS altitude {} km", s.alt_km);
        assert!((7.5..7.8).contains(&s.speed_km_s), "ISS speed {} km/s", s.speed_km_s);
        // Its orbit is inclined 51.6°, so it never goes beyond that latitude.
        let mut max_lat = 0.0f64;
        for k in 0..200 {
            if let Some(p) = iss.at(NOW + k as f64 * 60.0) {
                max_lat = max_lat.max(p.lat.abs());
            }
        }
        assert!((50.0..53.0).contains(&max_lat), "reached latitude {max_lat}");
    }

    #[test]
    fn an_orbit_path_is_closed_and_at_a_consistent_radius() {
        let qo = &parse_tles(QO100)[0];
        let path = qo.orbit(NOW, 64);
        assert_eq!(path.len(), 65);
        // A geostationary orbit is very nearly circular.
        let radii: Vec<f64> = path.iter().map(|(_, r)| *r).collect();
        let (lo, hi) = (
            radii.iter().copied().fold(f64::MAX, f64::min),
            radii.iter().copied().fold(0.0, f64::max),
        );
        assert!((hi - lo) < 0.02, "radius varied {} Earth radii", hi - lo);
        // Closed: the ends meet.
        let gap = (path[0].0 - path[path.len() - 1].0).len();
        assert!(gap < 0.02, "orbit does not close, gap {gap}");
        // And it stays in one plane — the equator, so ecliptic latitude never
        // exceeds the obliquity.
        for (d, _) in &path {
            assert!(d.lat_deg().abs() < 24.0, "orbit reached ecliptic latitude {}", d.lat_deg());
        }
    }

    #[test]
    fn elevation_says_whether_a_pass_is_workable() {
        let qo = &parse_tles(QO100)[0];
        let s = qo.at(NOW).unwrap();
        // From central Europe, QO-100 sits well up — that is the whole point of it.
        let vienna = s.elevation_from(48.2, 16.4);
        assert!((20.0..40.0).contains(&vienna), "elevation from Vienna {vienna}°");
        // Directly beneath it, it is overhead.
        let under = s.elevation_from(s.lat, s.lon);
        assert!((under - 90.0).abs() < 0.5, "overhead elevation {under}°");
        // From the far side of the planet it is below the horizon.
        let antipode = s.elevation_from(-s.lat, wrap180(s.lon + 180.0));
        assert!(antipode < 0.0, "visible from the antipode at {antipode}°");
    }

    /// Vienna, which is where the test QTH (JN78ve) is.
    const OBS: (f64, f64) = (48.2, 16.4);

    /// QO-100 is geostationary: it has no passes, and reporting an empty table
    /// would read as "never visible" when in fact it never sets.
    #[test]
    fn a_geostationary_satellite_reports_always_visible() {
        let qo = &parse_tles(QO100)[0];
        match qo.next_passes(OBS.0, OBS.1, NOW, 48.0 * 3600.0, 6) {
            PassSearch::AlwaysVisible { elevation, azimuth } => {
                assert!((20.0..40.0).contains(&elevation), "elevation {elevation}");
                // QO-100 is parked at 25.9°E, so from Vienna (16.4°E) it sits
                // south and a little east.
                assert!((140.0..175.0).contains(&azimuth), "azimuth {azimuth}");
                assert_eq!(compass(azimuth), "SSE");
            }
            other => panic!("expected AlwaysVisible, got {other:?}"),
        }
    }

    #[test]
    fn the_iss_passes_are_physically_sensible() {
        let sats = parse_tles(AMATEUR);
        let iss = sats.iter().find(|s| s.norad_id == 25544).unwrap();
        let PassSearch::Passes(passes) = iss.next_passes(OBS.0, OBS.1, NOW, 48.0 * 3600.0, 6)
        else {
            panic!("the ISS must pass over Vienna within two days");
        };
        assert_eq!(passes.len(), 6);
        for (i, p) in passes.iter().enumerate() {
            // A LEO pass runs from a couple of minutes to about twelve.
            let d = p.duration_s();
            assert!((60..=900).contains(&d), "pass {i} lasted {d} s");
            assert!(p.set_unix > p.rise_unix);
            // The peak is inside the pass and is the highest point of it.
            assert!(p.max_el_unix >= p.rise_unix && p.max_el_unix <= p.set_unix);
            assert!(p.max_el > 0.0 && p.max_el <= 90.0, "peak elevation {}", p.max_el);
            for az in [p.rise_az, p.set_az, p.max_el_az] {
                assert!((0.0..360.0).contains(&az), "azimuth {az} out of range");
            }
            // Passes come in order and do not overlap.
            if i > 0 {
                assert!(p.rise_unix > passes[i - 1].set_unix, "pass {i} overlaps the previous");
            }
            // Successive ISS passes are one or more orbits apart.
            if i > 0 {
                let gap = p.rise_unix - passes[i - 1].rise_unix;
                assert!(gap > 80 * 60, "only {gap} s between passes");
            }
        }
    }

    /// The endpoints must actually be the horizon, and the peak must actually be
    /// the peak — the two things a bisection bug would get wrong.
    #[test]
    fn pass_endpoints_sit_on_the_horizon() {
        let sats = parse_tles(AMATEUR);
        let iss = sats.iter().find(|s| s.norad_id == 25544).unwrap();
        let PassSearch::Passes(passes) = iss.next_passes(OBS.0, OBS.1, NOW, 48.0 * 3600.0, 3)
        else {
            panic!("no passes");
        };
        for p in &passes {
            let at = |t: i64| iss.at(t as f64).unwrap().az_el_from(OBS.0, OBS.1).1;
            // Within a second of the crossing, elevation is essentially zero.
            assert!(at(p.rise_unix).abs() < 0.15, "rise elevation {}", at(p.rise_unix));
            assert!(at(p.set_unix).abs() < 0.15, "set elevation {}", at(p.set_unix));
            // Just inside the pass it is above the horizon; just outside, below.
            assert!(at(p.rise_unix + 30) > 0.0);
            assert!(at(p.set_unix - 30) > 0.0);
            assert!(at(p.rise_unix - 30) < 0.0);
            assert!(at(p.set_unix + 30) < 0.0);
            // Nothing in the pass beats the reported maximum.
            for k in 0..=20 {
                let t = p.rise_unix + (p.set_unix - p.rise_unix) * k / 20;
                assert!(at(t) <= p.max_el + 0.5, "found {} above the peak {}", at(t), p.max_el);
            }
        }
    }

    /// A pass already in progress must be reported from now, not skipped.
    #[test]
    fn a_pass_already_underway_is_reported() {
        let sats = parse_tles(AMATEUR);
        let iss = sats.iter().find(|s| s.norad_id == 25544).unwrap();
        let PassSearch::Passes(first) = iss.next_passes(OBS.0, OBS.1, NOW, 48.0 * 3600.0, 1) else {
            panic!("no passes");
        };
        // Restart the search from the middle of that pass.
        let mid = (first[0].rise_unix + first[0].set_unix) as f64 / 2.0;
        let PassSearch::Passes(again) = iss.next_passes(OBS.0, OBS.1, mid, 48.0 * 3600.0, 1) else {
            panic!("mid-pass search found nothing");
        };
        assert_eq!(again[0].rise_unix, mid.round() as i64, "should start at the search time");
        assert!((again[0].set_unix - first[0].set_unix).abs() <= 2, "same set time");
    }

    #[test]
    fn a_polar_satellite_never_rises_at_the_wrong_latitude() {
        let sats = parse_tles(AMATEUR);
        let iss = sats.iter().find(|s| s.norad_id == 25544).unwrap();
        // The ISS is inclined 51.6°, so from the geographic South Pole it never
        // comes up.
        assert_eq!(iss.next_passes(-89.9, 0.0, NOW, 24.0 * 3600.0, 4), PassSearch::NeverVisible);
        // ...and a zero-length search finds nothing rather than panicking.
        assert_eq!(iss.next_passes(OBS.0, OBS.1, NOW, 0.0, 4), PassSearch::NeverVisible);
        assert_eq!(iss.next_passes(OBS.0, OBS.1, NOW, 86_400.0, 0), PassSearch::NeverVisible);
    }

    #[test]
    fn compass_points_cover_the_circle() {
        assert_eq!(compass(0.0), "N");
        assert_eq!(compass(90.0), "E");
        assert_eq!(compass(180.0), "S");
        assert_eq!(compass(270.0), "W");
        assert_eq!(compass(45.0), "NE");
        // Wraps, and tolerates out-of-range input.
        assert_eq!(compass(359.9), "N");
        assert_eq!(compass(360.0), "N");
        assert_eq!(compass(-90.0), "W");
        assert_eq!(compass(720.0), "N");
    }

    #[test]
    fn pass_quality_reflects_the_peak() {
        let p = |max_el| Pass {
            rise_unix: 0,
            set_unix: 600,
            rise_az: 0.0,
            set_az: 0.0,
            max_el,
            max_el_unix: 300,
            max_el_az: 0.0,
        };
        assert_eq!(p(75.0).quality(), "overhead");
        assert_eq!(p(40.0).quality(), "good");
        assert_eq!(p(20.0).quality(), "fair");
        assert_eq!(p(4.0).quality(), "marginal");
        assert_eq!(p(0.0).duration_s(), 600);
    }

    #[test]
    fn stale_and_broken_element_sets_are_refused() {
        let qo = &parse_tles(QO100)[0];
        // A month past the epoch, SGP4 is no longer worth trusting.
        assert!(qo.at(qo.epoch_unix as f64 + 40.0 * 86_400.0).is_none());
        assert!(qo.at(qo.epoch_unix as f64 - 40.0 * 86_400.0).is_none());
        assert!(qo.at(qo.epoch_unix as f64).is_some());

        assert!(parse_tles("").is_empty());
        assert!(parse_tles("not a tle at all").is_empty());
    }

    /// QO-100 is the observation reference case too: from Vienna the slant
    /// range to a geostationary bird at 25.9°E is a known constant, and its
    /// range rate is (nearly) zero — a wrong Earth-rotation term in the
    /// velocity conversion shows up here as hundreds of metres per second.
    #[test]
    fn qo100_observation_matches_the_known_geometry() {
        let qo = &parse_tles(QO100)[0];
        let o = qo.observe(NOW, OBS.0, OBS.1).expect("QO-100 must observe");
        assert!((o.range_km - 38_300.0).abs() < 800.0, "range {} km", o.range_km);
        assert!(o.range_rate_km_s.abs() < 0.05, "range rate {} km/s", o.range_rate_km_s);
        // The topocentric look angles agree with the display approximation.
        let (az, el) = qo.at(NOW).unwrap().az_el_from(OBS.0, OBS.1);
        assert!((o.az_deg - az).abs() < 1.0, "azimuth {} vs {}", o.az_deg, az);
        assert!((o.el_deg - el).abs() < 1.0, "elevation {} vs {}", o.el_deg, el);
    }

    /// The shape of a LEO Doppler curve: fastest at the horizon, zero at
    /// culmination (closest approach), approaching before it and receding
    /// after. A sign error anywhere in `observe` breaks one of these.
    #[test]
    fn iss_range_rate_peaks_at_the_horizon_and_nulls_at_culmination() {
        let sats = parse_tles(AMATEUR);
        let iss = sats.iter().find(|s| s.norad_id == 25544).unwrap();

        // Over a day, the extreme radial velocity has to approach — but never
        // exceed — the orbital velocity itself.
        let mut peak = 0.0f64;
        for k in 0..1440 {
            if let Some(o) = iss.observe(NOW + k as f64 * 60.0, OBS.0, OBS.1) {
                peak = peak.max(o.range_rate_km_s.abs());
            }
        }
        assert!((4.5..8.0).contains(&peak), "peak |range rate| {peak} km/s");

        let PassSearch::Passes(passes) = iss.next_passes(OBS.0, OBS.1, NOW, 48.0 * 3600.0, 3)
        else {
            panic!("no passes");
        };
        for p in &passes {
            let at = |t: f64| iss.observe(t, OBS.0, OBS.1).unwrap().range_rate_km_s;
            // Culmination is closest approach: the radial velocity crosses
            // zero there. The peak time is quantized to the second and the
            // rate turns over at up to ~0.15 km/s per second, hence the slack.
            assert!(at(p.max_el_unix as f64).abs() < 0.35, "at peak: {}", at(p.max_el_unix as f64));
            assert!(at(p.max_el_unix as f64 - 60.0) < 0.0, "not approaching before the peak");
            assert!(at(p.max_el_unix as f64 + 60.0) > 0.0, "not receding after the peak");
        }
    }

    /// The range rate must *be* the derivative of the range — checked
    /// numerically across a whole pass. This is the test a Doppler sign flip
    /// or a dropped transport term cannot get past.
    #[test]
    fn range_rate_is_the_derivative_of_range() {
        let sats = parse_tles(AMATEUR);
        let iss = sats.iter().find(|s| s.norad_id == 25544).unwrap();
        let PassSearch::Passes(passes) = iss.next_passes(OBS.0, OBS.1, NOW, 48.0 * 3600.0, 1)
        else {
            panic!("no passes");
        };
        let p = &passes[0];
        for k in 0..=20 {
            let t = p.rise_unix as f64 + (p.set_unix - p.rise_unix) as f64 * k as f64 / 20.0;
            let o = iss.observe(t, OBS.0, OBS.1).unwrap();
            let before = iss.observe(t - 1.0, OBS.0, OBS.1).unwrap().range_km;
            let after = iss.observe(t + 1.0, OBS.0, OBS.1).unwrap().range_km;
            let numeric = (after - before) / 2.0;
            assert!(
                (o.range_rate_km_s - numeric).abs() < 0.01,
                "at t+{k}: analytic {} vs numeric {numeric}",
                o.range_rate_km_s
            );
        }
    }

    #[test]
    fn observation_is_refused_for_stale_elements() {
        let qo = &parse_tles(QO100)[0];
        assert!(qo.observe(qo.epoch_unix as f64 + 40.0 * 86_400.0, OBS.0, OBS.1).is_none());
        assert!(qo.observe(NOW, OBS.0, OBS.1).is_some());
    }
}
