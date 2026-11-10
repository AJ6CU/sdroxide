//! The other planets, the major moons, and how each body is oriented.
//!
//! Same frame and units as [`crate::ephem`]: heliocentric ecliptic **of date**,
//! right-handed, gigametres. Everything here is computed in the J2000 frame the
//! source data is published in and then rotated forward by the general
//! precession in longitude, so a planet and the Earth can be differenced
//! directly.
//!
//! Three sources, three accuracies — all of them stated rather than implied:
//!
//! * **Positions** — JPL's *Approximate Positions of the Major Planets*
//!   Keplerian element set, valid 1800–2050. Errors run from a few arcseconds
//!   (Mercury) to ~10′ (Neptune): under a tenth of a rendered pixel at any
//!   framing this view offers.
//! * **Orientation** — IAU/WGCCRE 2015 rotational elements. The small
//!   periodic terms are dropped, which matters only for Neptune (±0.7° of pole
//!   direction).
//! * **Moons** — circular orbits *fitted to JPL Horizons* over 2010–2040 by
//!   `tools/fit_bodies.py`, each carrying the worst-case error the fit actually
//!   left ([`Moon::fit_error_deg`]). A circle cannot express Titan's
//!   eccentricity or Triton's precessing orbit plane, and those two are the
//!   worst at ~4°; the Galilean moons land inside 1°.
//!
//! `tests/fixtures/horizons.json` holds Horizons samples for every body here,
//! and the tests below replay them — which is what keeps a mistyped digit in
//! the element table from silently moving a planet.

use crate::ephem::{centuries, wrap360};
use crate::vec3::{Basis, Vec3, vec3};

/// Mean obliquity at J2000.0, degrees. The IAU pole directions are given in
/// equatorial J2000 coordinates, so this is the angle that takes them to the
/// ecliptic frame everything else here uses.
const OBLIQUITY_J2000: f64 = 23.439_291_1;

/// Days in a Julian year, for turning a semi-major axis into a period.
const JULIAN_YEAR_D: f64 = 365.256_898;

/// What a body's surface looks like — the one physical fact the renderer needs
/// in order to pick a shading model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Surface {
    /// Airless rock, saturated with craters: Mercury, Callisto, Phobos.
    Cratered,
    /// Featureless sulphuric-acid cloud deck: Venus.
    Cloudy,
    /// Iron-oxide desert with polar caps: Mars.
    Desert,
    /// Banded hydrogen atmosphere with visible storms: Jupiter, Saturn.
    GasBands,
    /// Methane-blue, nearly featureless: Uranus, Neptune.
    IceGiant,
    /// Water ice, bright and fractured: Europa, Enceladus, the Uranian moons.
    Icy,
    /// Resurfaced by volcanism: Io.
    Volcanic,
    /// Under an opaque organic haze: Titan.
    Haze,
}

/// A ring system, in units of the parent's mean radius.
#[derive(Debug, Clone, Copy)]
pub struct Rings {
    pub inner: f64,
    pub outer: f64,
    /// How much of the light gets through — Saturn's rings are a sheet you
    /// cannot see past, Uranus's are threads. 0 = invisible, 1 = opaque.
    pub opacity: f64,
}

/// The Keplerian element set, each element as `[value at J2000, per century]`.
#[derive(Debug, Clone, Copy)]
struct Elements {
    /// Semi-major axis, AU.
    a: [f64; 2],
    e: [f64; 2],
    /// Inclination to the J2000 ecliptic, degrees.
    incl: [f64; 2],
    /// Mean longitude, degrees.
    l: [f64; 2],
    /// Longitude of perihelion, degrees.
    peri: [f64; 2],
    /// Longitude of the ascending node, degrees.
    node: [f64; 2],
}

/// IAU rotational elements: pole direction in equatorial J2000 and the prime
/// meridian's position.
#[derive(Debug, Clone, Copy)]
struct Rotation {
    /// Right ascension of the north pole: `[deg, deg/century]`.
    ra: [f64; 2],
    /// Declination of the north pole: `[deg, deg/century]`.
    dec: [f64; 2],
    /// Prime meridian: `[W0 in deg, deg/day]`. Negative rate = retrograde.
    w: [f64; 2],
}

/// Everything about a planet that does not change.
#[derive(Debug, Clone, Copy)]
pub struct PlanetInfo {
    pub name: &'static str,
    /// Mean radius, gigametres.
    pub radius: f64,
    pub surface: Surface,
    pub rings: Option<Rings>,
    elements: Elements,
    rotation: Rotation,
}

impl PlanetInfo {
    /// Sidereal orbital period, days — Kepler's third law from the semi-major
    /// axis, which is all the orbit-path sampler needs.
    pub fn orbit_days(&self) -> f64 {
        JULIAN_YEAR_D * self.elements.a[0].powf(1.5)
    }
}

/// A planet other than the Earth, in orbital order.
///
/// The Earth is deliberately absent: [`crate::ephem::earth_heliocentric`] gives
/// it from Meeus's solar theory, which is both more accurate and already the
/// frame the Sun, the terminator and the QTH marker are built on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Planet {
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
    Uranus,
    Neptune,
}

impl Planet {
    pub const ALL: [Planet; 7] = [
        Planet::Mercury,
        Planet::Venus,
        Planet::Mars,
        Planet::Jupiter,
        Planet::Saturn,
        Planet::Uranus,
        Planet::Neptune,
    ];

    pub fn index(self) -> usize {
        Planet::ALL.iter().position(|p| *p == self).unwrap_or(0)
    }

    pub fn info(self) -> &'static PlanetInfo {
        &PLANETS[self.index()]
    }

    pub fn name(self) -> &'static str {
        self.info().name
    }

    /// Heliocentric position, gigametres, ecliptic of date.
    pub fn heliocentric(self, jd: f64) -> Vec3 {
        precess(kepler_position(&self.info().elements, jd), jd)
    }

    /// The body-fixed frame, as columns expressed in the ecliptic of date:
    /// `x` at (0° latitude, 0° longitude), `z` at the north pole. Matches the
    /// convention the sphere mesh and the land mask use for the Earth.
    pub fn basis(self, jd: f64) -> Basis {
        body_basis(&self.info().rotation, jd)
    }

    /// The moons this view draws for the planet.
    pub fn moons(self) -> impl Iterator<Item = &'static Moon> {
        MOONS.iter().filter(move |m| m.parent == self)
    }
}

/// A moon, as a circle fitted to Horizons (see the module docs).
#[derive(Debug, Clone, Copy)]
pub struct Moon {
    pub name: &'static str,
    pub parent: Planet,
    /// Orbit radius, gigametres.
    pub a: f64,
    /// Sidereal period, days.
    pub period_d: f64,
    /// Inclination of the orbit plane to the J2000 ecliptic, degrees. Past 90°
    /// the orbit is retrograde (Triton), which is not a rounding artefact but
    /// the thing itself.
    pub incl_deg: f64,
    /// Longitude of the ascending node on the J2000 ecliptic, degrees.
    pub node_deg: f64,
    /// Angle from that node at J2000.0, degrees.
    pub l0_deg: f64,
    /// Mean radius, gigametres.
    pub radius: f64,
    pub surface: Surface,
    /// Worst-case angular error of this circle against Horizons, degrees of
    /// orbital phase, measured over the fixture's 2024–2032 span. Published
    /// because a circular model of Titan is off by degrees, and pretending
    /// otherwise would be a lie; the tests assert against these numbers.
    pub fit_error_deg: f64,
}

impl Moon {
    /// Offset from its planet, gigametres, ecliptic of date.
    pub fn offset(&self, jd: f64) -> Vec3 {
        let (i, o) = (self.incl_deg.to_radians(), self.node_deg.to_radians());
        // The orbit's normal, and the pair of in-plane axes the phase is
        // measured from — exactly the parameterisation the fit produces.
        let n = vec3(i.sin() * o.sin(), -i.sin() * o.cos(), i.cos());
        let u = vec3(o.cos(), o.sin(), 0.0);
        let v = n.cross(u);
        let th = (self.l0_deg + 360.0 * (jd - 2_451_545.0) / self.period_d).to_radians();
        precess((u * th.cos() + v * th.sin()) * self.a, jd)
    }
}

/// Every moon the view draws, grouped by parent.
///
/// Orbit geometry is the output of `tools/fit_bodies.py`; radii are the IAU
/// mean values.
///
/// **The order is a persisted format.** The 3D view stores its camera target as
/// an index into this table, so a new moon is appended at the *end* — inserting
/// one mid-table would re-point everybody's saved target at a different body.
/// Grouping is by lookup rather than by position, so appending costs nothing.
pub static MOONS: &[Moon] = &[
    moon(
        "Phobos",
        Planet::Mars,
        Surface::Cratered,
        0.000_011_1,
        0.009_378,
        0.318_910_1,
        26.9468,
        81.3020,
        175.2965,
        3.1,
    ),
    moon(
        "Deimos",
        Planet::Mars,
        Surface::Cratered,
        0.000_006_2,
        0.023_457,
        1.262_441_0,
        26.0340,
        83.1084,
        217.9145,
        2.1,
    ),
    moon(
        "Io",
        Planet::Jupiter,
        Surface::Volcanic,
        0.001_821_6,
        0.421_734,
        1.769_137_8,
        2.1863,
        338.5922,
        39.6030,
        0.5,
    ),
    moon(
        "Europa",
        Planet::Jupiter,
        Surface::Icy,
        0.001_560_8,
        0.670_784,
        3.551_181_3,
        2.3008,
        349.6050,
        223.3522,
        1.4,
    ),
    moon(
        "Ganymede",
        Planet::Jupiter,
        Surface::Cratered,
        0.002_634_1,
        1.070_477,
        7.154_553_8,
        2.2921,
        338.4866,
        241.5283,
        0.4,
    ),
    moon(
        "Callisto",
        Planet::Jupiter,
        Surface::Cratered,
        0.002_410_3,
        1.882_801,
        16.689_044_3,
        1.9602,
        337.0372,
        102.4867,
        0.9,
    ),
    moon(
        "Enceladus",
        Planet::Saturn,
        Surface::Icy,
        0.000_252_1,
        0.238_093,
        1.370_218_3,
        28.0483,
        169.5092,
        142.6489,
        0.8,
    ),
    moon(
        "Tethys",
        Planet::Saturn,
        Surface::Icy,
        0.000_531_1,
        0.294_675,
        1.887_802_4,
        27.1398,
        168.2413,
        149.9646,
        2.5,
    ),
    moon(
        "Dione",
        Planet::Saturn,
        Surface::Icy,
        0.000_561_4,
        0.377_426,
        2.736_915_5,
        28.0537,
        169.5225,
        136.8684,
        0.4,
    ),
    moon(
        "Rhea",
        Planet::Saturn,
        Surface::Icy,
        0.000_763_8,
        0.527_075,
        4.517_502_9,
        27.8771,
        168.9550,
        12.7417,
        0.6,
    ),
    moon(
        "Titan",
        Planet::Saturn,
        Surface::Haze,
        0.002_574_7,
        1.222_836,
        15.945_438_8,
        27.7089,
        169.0684,
        327.2068,
        4.0,
    ),
    moon(
        "Iapetus",
        Planet::Saturn,
        Surface::Cratered,
        0.000_734_5,
        3.563_010,
        79.330_326_1,
        17.0000,
        138.8233,
        77.7233,
        3.7,
    ),
    moon(
        "Miranda",
        Planet::Uranus,
        Surface::Icy,
        0.000_235_8,
        0.129_841,
        1.413_478_5,
        99.4000,
        165.7880,
        321.6880,
        6.5,
    ),
    moon(
        "Ariel",
        Planet::Uranus,
        Surface::Icy,
        0.000_578_9,
        0.190_932,
        2.520_379_4,
        97.7212,
        167.6678,
        198.1876,
        0.2,
    ),
    moon(
        "Umbriel",
        Planet::Uranus,
        Surface::Icy,
        0.000_584_7,
        0.265_977,
        4.144_177_4,
        97.7181,
        167.7028,
        246.2747,
        0.6,
    ),
    moon(
        "Titania",
        Planet::Uranus,
        Surface::Icy,
        0.000_788_4,
        0.436_301,
        8.705_867_1,
        97.7794,
        167.6473,
        276.4197,
        0.3,
    ),
    moon(
        "Oberon",
        Planet::Uranus,
        Surface::Icy,
        0.000_761_4,
        0.583_447,
        13.463_238_7,
        97.9102,
        167.7100,
        347.5247,
        0.3,
    ),
    moon(
        "Triton",
        Planet::Neptune,
        Surface::Icy,
        0.001_353_4,
        0.354_759,
        5.876_843_3,
        129.3400,
        222.4621,
        78.8495,
        1.2,
    ),
];

/// Terse constructor so the table above stays one line per moon.
#[allow(clippy::too_many_arguments)]
const fn moon(
    name: &'static str,
    parent: Planet,
    surface: Surface,
    radius: f64,
    a: f64,
    period_d: f64,
    incl_deg: f64,
    node_deg: f64,
    l0_deg: f64,
    fit_error_deg: f64,
) -> Moon {
    Moon { name, parent, a, period_d, incl_deg, node_deg, l0_deg, radius, surface, fit_error_deg }
}

/// Element and rotation tables, in [`Planet::ALL`] order.
static PLANETS: [PlanetInfo; 7] = [
    PlanetInfo {
        name: "Mercury",
        radius: 0.002_439_7,
        surface: Surface::Cratered,
        rings: None,
        elements: Elements {
            a: [0.387_099_27, 0.000_000_37],
            e: [0.205_635_93, 0.000_019_06],
            incl: [7.004_979_02, -0.005_947_49],
            l: [252.250_323_50, 149_472.674_111_75],
            peri: [77.457_796_28, 0.160_476_89],
            node: [48.330_765_93, -0.125_340_81],
        },
        rotation: Rotation {
            ra: [281.0103, -0.0328],
            dec: [61.4155, -0.0049],
            w: [329.5988, 6.138_510_8],
        },
    },
    PlanetInfo {
        name: "Venus",
        radius: 0.006_051_8,
        surface: Surface::Cloudy,
        rings: None,
        elements: Elements {
            a: [0.723_335_66, 0.000_003_90],
            e: [0.006_776_72, -0.000_041_07],
            incl: [3.394_676_05, -0.000_788_90],
            l: [181.979_099_50, 58_517.815_387_29],
            peri: [131.602_467_18, 0.002_683_29],
            node: [76.679_842_55, -0.277_694_18],
        },
        // Venus turns backwards, hence the negative rate.
        rotation: Rotation { ra: [272.76, 0.0], dec: [67.16, 0.0], w: [160.20, -1.481_368_8] },
    },
    PlanetInfo {
        name: "Mars",
        radius: 0.003_389_5,
        surface: Surface::Desert,
        rings: None,
        elements: Elements {
            a: [1.523_710_34, 0.000_018_47],
            e: [0.093_394_10, 0.000_078_82],
            incl: [1.849_691_42, -0.008_131_31],
            l: [-4.553_432_05, 19_140.302_684_99],
            peri: [-23.943_629_59, 0.444_410_88],
            node: [49.559_538_91, -0.292_573_43],
        },
        rotation: Rotation {
            ra: [317.681_43, -0.1061],
            dec: [52.886_50, -0.0609],
            w: [176.630, 350.891_982_26],
        },
    },
    PlanetInfo {
        name: "Jupiter",
        radius: 0.069_911,
        surface: Surface::GasBands,
        // The main and halo rings are a whisper — not drawn.
        rings: None,
        elements: Elements {
            a: [5.202_887_00, -0.000_116_07],
            e: [0.048_386_24, -0.000_132_53],
            incl: [1.304_396_95, -0.001_837_14],
            l: [34.396_440_51, 3_034.746_127_75],
            peri: [14.728_479_83, 0.212_526_68],
            node: [100.473_909_09, 0.204_691_06],
        },
        // System III, the rotation of the magnetic field — the only sense in
        // which a gas giant has a fixed prime meridian at all.
        rotation: Rotation {
            ra: [268.056_595, -0.006_499],
            dec: [64.495_303, 0.002_413],
            w: [284.95, 870.536],
        },
    },
    PlanetInfo {
        name: "Saturn",
        radius: 0.058_232,
        surface: Surface::GasBands,
        // C-ring inner edge to A-ring outer edge, 74 660 km to 136 780 km,
        // in units of the mean radius the body is drawn at.
        rings: Some(Rings { inner: 1.282, outer: 2.349, opacity: 1.0 }),
        elements: Elements {
            a: [9.536_675_94, -0.001_250_60],
            e: [0.053_861_79, -0.000_509_91],
            incl: [2.485_991_87, 0.001_936_09],
            l: [49.954_244_23, 1_222.493_622_01],
            peri: [92.598_878_31, -0.418_972_16],
            node: [113.662_424_48, -0.288_677_94],
        },
        rotation: Rotation {
            ra: [40.589, -0.036],
            dec: [83.537, -0.004],
            w: [38.90, 810.793_902_4],
        },
    },
    PlanetInfo {
        name: "Uranus",
        radius: 0.025_362,
        surface: Surface::IceGiant,
        // The nine narrow rings, from 6 to ε (41 800–51 150 km). Thread-thin,
        // but they stand almost vertical from here, which is the point.
        rings: Some(Rings { inner: 1.648, outer: 2.017, opacity: 0.22 }),
        elements: Elements {
            a: [19.189_164_64, -0.001_961_76],
            e: [0.047_257_44, -0.000_043_97],
            incl: [0.772_637_83, -0.002_429_39],
            l: [313.238_104_51, 428.482_027_85],
            peri: [170.954_276_30, 0.408_052_81],
            node: [74.016_925_03, 0.042_405_89],
        },
        rotation: Rotation { ra: [257.311, 0.0], dec: [-15.175, 0.0], w: [203.81, -501.160_092_8] },
    },
    PlanetInfo {
        name: "Neptune",
        radius: 0.024_622,
        surface: Surface::IceGiant,
        rings: None,
        elements: Elements {
            a: [30.069_922_76, 0.000_262_91],
            e: [0.008_590_48, 0.000_051_05],
            incl: [1.770_043_47, 0.000_353_72],
            l: [-55.120_029_69, 218.459_453_25],
            peri: [44.964_762_27, -0.322_414_64],
            node: [131.784_225_74, -0.005_086_64],
        },
        // The ±0.7° periodic terms in Neptune's pole are dropped; they move
        // Triton's orbit plane by less than the fit's own error.
        rotation: Rotation { ra: [299.36, 0.0], dec: [43.46, 0.0], w: [253.18, 536.312_849_2] },
    },
];

/// The Earth–Moon barycentre's elements, from the same JPL table.
///
/// Not used to place anything — Meeus does that — but the tests solve it and
/// compare against [`crate::ephem::earth_heliocentric`], which is what proves
/// the solver and the frame conversion rather than any one planet's row.
#[cfg(test)]
const EARTH_ELEMENTS: Elements = Elements {
    a: [1.000_002_61, 0.000_005_62],
    e: [0.016_711_23, -0.000_043_92],
    incl: [-0.000_015_31, -0.012_946_68],
    l: [100.464_571_66, 35_999.372_449_81],
    peri: [102.937_681_93, 0.323_273_64],
    node: [0.0, 0.0],
};

/// Solve the element set for a heliocentric position, gigametres, in the
/// **J2000** ecliptic frame the elements are defined in.
fn kepler_position(el: &Elements, jd: f64) -> Vec3 {
    let t = centuries(jd);
    let at = |v: [f64; 2]| v[0] + v[1] * t;
    let (a, e) = (at(el.a), at(el.e));
    let (incl, node) = (at(el.incl).to_radians(), at(el.node).to_radians());
    let peri = at(el.peri);
    // Argument of perihelion and mean anomaly, the two the solver wants.
    let arg = (peri - at(el.node)).to_radians();
    let m = wrap180_deg(at(el.l) - peri);

    let ea = eccentric_anomaly(m, e).to_radians();
    // In the orbital plane, with +x toward perihelion.
    let px = a * (ea.cos() - e);
    let py = a * (1.0 - e * e).max(0.0).sqrt() * ea.sin();

    let (cw, sw) = (arg.cos(), arg.sin());
    let (co, so) = (node.cos(), node.sin());
    let (ci, si) = (incl.cos(), incl.sin());
    vec3(
        (cw * co - sw * so * ci) * px + (-sw * co - cw * so * ci) * py,
        (cw * so + sw * co * ci) * px + (-sw * so + cw * co * ci) * py,
        (sw * si) * px + (cw * si) * py,
    ) * crate::ephem::AU
}

/// Newton's method on Kepler's equation, in degrees (JPL's own formulation:
/// `M = E − e* sin E` with `e*` the eccentricity expressed in degrees).
fn eccentric_anomaly(m_deg: f64, e: f64) -> f64 {
    let e_star = e.to_degrees();
    let mut ea = m_deg + e_star * m_deg.to_radians().sin();
    for _ in 0..16 {
        let dm = m_deg - (ea - e_star * ea.to_radians().sin());
        let de = dm / (1.0 - e * ea.to_radians().cos());
        ea += de;
        if de.abs() < 1e-11 {
            break;
        }
    }
    ea
}

/// Wrap to `(-180, 180]`, as Kepler's equation wants its mean anomaly.
fn wrap180_deg(d: f64) -> f64 {
    let r = wrap360(d);
    if r > 180.0 { r - 360.0 } else { r }
}

/// General precession in longitude since J2000, degrees (Lieske): 1.397° per
/// century, so 0.36° by 2026.
fn precession_deg(jd: f64) -> f64 {
    let t = centuries(jd);
    1.396_971_3 * t + 0.000_307_0 * t * t
}

/// Rotate a J2000 ecliptic vector into the ecliptic **of date**.
///
/// One rotation about the ecliptic pole. Small, but it is the difference
/// between the planets and the Earth being in the same frame and merely being
/// in similar ones.
pub(crate) fn precess(v: Vec3, jd: f64) -> Vec3 {
    rotate_z(v, precession_deg(jd))
}

fn rotate_z(v: Vec3, deg: f64) -> Vec3 {
    let (c, s) = (deg.to_radians().cos(), deg.to_radians().sin());
    vec3(v.x * c - v.y * s, v.x * s + v.y * c, v.z)
}

/// Turn IAU rotational elements into a body-fixed basis in the ecliptic of date.
fn body_basis(rot: &Rotation, jd: f64) -> Basis {
    let t = centuries(jd);
    let d = jd - 2_451_545.0;
    let ra = (rot.ra[0] + rot.ra[1] * t).to_radians();
    let dec = (rot.dec[0] + rot.dec[1] * t).to_radians();
    let w = (rot.w[0] + rot.w[1] * d).to_radians();

    // Pole, and the ascending node of the body's equator on the J2000 equator —
    // both in equatorial J2000, which is how the IAU publishes them.
    let pole = vec3(dec.cos() * ra.cos(), dec.cos() * ra.sin(), dec.sin());
    let node = vec3(-ra.sin(), ra.cos(), 0.0);
    // The prime meridian sits `W` prograde of that node.
    let x = node * w.cos() + pole.cross(node) * w.sin();
    let z = pole;
    let y = z.cross(x);

    let to_frame = |v: Vec3| precess(equatorial_j2000_to_ecliptic(v), jd);
    Basis { x: to_frame(x), y: to_frame(y), z: to_frame(z) }
}

/// Equatorial J2000 to ecliptic J2000: a rotation by −ε₀ about the X axis.
fn equatorial_j2000_to_ecliptic(v: Vec3) -> Vec3 {
    let e = OBLIQUITY_J2000.to_radians();
    vec3(v.x, v.y * e.cos() + v.z * e.sin(), -v.y * e.sin() + v.z * e.cos())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ephem;

    /// The Horizons samples every test here measures against: geometric
    /// positions in the J2000 ecliptic frame, gigametres.
    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("../tests/fixtures/horizons.json")).unwrap()
    }

    fn samples(fx: &serde_json::Value, group: &str, name: &str) -> Vec<(f64, Vec3)> {
        fx[group][name]
            .as_array()
            .unwrap_or_else(|| panic!("no {group}/{name} in the fixture"))
            .iter()
            .map(|row| {
                let v: Vec<f64> =
                    row.as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect();
                (v[0], vec3(v[1], v[2], v[3]))
            })
            .collect()
    }

    /// Angle between two directions, degrees.
    fn angle_deg(a: Vec3, b: Vec3) -> f64 {
        (a.normalize().dot(b.normalize())).clamp(-1.0, 1.0).acos().to_degrees()
    }

    /// The element table, against JPL Horizons itself.
    ///
    /// This is the test that makes the table trustworthy: every row is 12
    /// numbers, a single mistyped digit moves the planet by degrees, and no
    /// amount of reading catches that. The tolerances are what the element set
    /// actually achieves over 2015–2045 (rounded up a little), which is well
    /// inside JPL's published bound of 10′ for the outer planets — asserting
    /// the looser published figure would let a real typo through.
    #[test]
    fn planet_positions_match_horizons() {
        let fx = fixture();
        for p in Planet::ALL {
            let tol = match p {
                Planet::Saturn => 0.15,
                Planet::Jupiter => 0.1,
                Planet::Uranus | Planet::Neptune => 0.05,
                _ => 0.03,
            };
            let mut worst: f64 = 0.0;
            for (jd, want) in samples(&fx, "planets", p.name()) {
                // The fixture is J2000; `heliocentric` is of date, so compare
                // against the un-precessed solution.
                let got = kepler_position(&p.info().elements, jd);
                worst = worst.max(angle_deg(got, want));
                let radial = (got.len() - want.len()).abs() / want.len();
                assert!(radial < 0.002, "{}: {radial:.5} radial error at jd {jd}", p.name());
            }
            assert!(worst < tol, "{}: worst {worst:.4}° against Horizons", p.name());
        }
    }

    /// The solver and the frame conversion, checked against a completely
    /// different theory: Meeus's Sun, which is what places the Earth in this
    /// view. Agreement here means an error in a planet's position can only be
    /// in that planet's own row.
    #[test]
    fn the_earth_from_the_element_table_agrees_with_meeus() {
        for k in 0..40 {
            let jd = 2_451_545.0 + k as f64 * 200.0;
            let kepler = precess(kepler_position(&EARTH_ELEMENTS, jd), jd);
            let meeus = ephem::earth_heliocentric(jd);
            // The element set tracks the Earth–Moon *barycentre*, which swings
            // ~4700 km either side of the Earth over a month — 1.8e-3° at 1 AU.
            assert!(angle_deg(kepler, meeus) < 0.01, "jd {jd}: {}°", angle_deg(kepler, meeus));
            assert!((kepler.len() - meeus.len()).abs() < 1.0e-4 * ephem::AU);
        }
    }

    /// Precession is the whole reason the two frames can be differenced. If it
    /// were dropped (or applied backwards) the planets would sit a third of a
    /// degree off the Earth's frame — invisible on a planet, but exactly the
    /// sort of thing that never gets found later.
    #[test]
    fn precession_advances_ecliptic_longitude() {
        let v = vec3(1.0, 0.0, 0.0);
        let jd = 2_451_545.0 + 100.0 * 365.25; // one century on
        let moved = precess(v, jd);
        assert!((moved.lon_deg() - 1.3973).abs() < 0.001, "moved {}°", moved.lon_deg());
        // ...and it is the identity at the epoch itself.
        assert!((precess(v, 2_451_545.0) - v).len() < 1e-12);
    }

    #[test]
    fn every_planet_stays_inside_its_own_orbit() {
        // Perihelion and aphelion, AU, from the element set's own a and e.
        for p in Planet::ALL {
            let el = &p.info().elements;
            let (a, e) = (el.a[0], el.e[0]);
            let (mut lo, mut hi) = (f64::MAX, f64::MIN);
            let mut max_lat: f64 = 0.0;
            for k in 0..400 {
                let jd = 2_451_545.0 + k as f64 * p.info().orbit_days() / 400.0;
                let v = p.heliocentric(jd);
                lo = lo.min(v.len() / ephem::AU);
                hi = hi.max(v.len() / ephem::AU);
                max_lat = max_lat.max(v.lat_deg().abs());
            }
            assert!((lo - a * (1.0 - e)).abs() < 0.01, "{}: perihelion {lo}", p.name());
            assert!((hi - a * (1.0 + e)).abs() < 0.01, "{}: aphelion {hi}", p.name());
            // A full orbit reaches its inclination, and never exceeds it.
            assert!(
                (max_lat - el.incl[0]).abs() < 0.05,
                "{}: max ecliptic latitude {max_lat}, inclination {}",
                p.name(),
                el.incl[0]
            );
        }
    }

    /// Distances between the planets are the thing the whole view is a picture
    /// of, so check the ordering holds at an arbitrary instant.
    #[test]
    fn the_planets_are_in_orbital_order() {
        let jd = ephem::julian_day(1_784_937_600.0);
        let mut prev = 0.0;
        for p in Planet::ALL {
            let r = p.heliocentric(jd).len();
            assert!(r > prev, "{} at {r} Gm is inside the previous planet", p.name());
            prev = r;
        }
        assert!(prev / ephem::AU > 29.0, "Neptune should be ~30 AU out");
    }

    /// Every moon, against Horizons. The tolerance is the error the fit
    /// published for itself, plus a little slack for the fixture window
    /// extending beyond the fit's.
    #[test]
    fn moon_positions_match_horizons() {
        let fx = fixture();
        for m in MOONS {
            let mut worst: f64 = 0.0;
            for (jd, want) in samples(&fx, "moons", m.name) {
                // As above: the fixture is J2000, so undo the precession the
                // public method applies.
                let got = rotate_z(m.offset(jd), -precession_deg(jd));
                worst = worst.max(angle_deg(got, want));
                let radial = (got.len() - want.len()).abs() / want.len();
                assert!(radial < 0.05, "{}: {radial:.4} radial error at jd {jd}", m.name);
            }
            assert!(
                worst < m.fit_error_deg + 0.5,
                "{}: {worst:.2}° against Horizons, the table claims {:.2}°",
                m.name,
                m.fit_error_deg
            );
        }
    }

    #[test]
    fn moons_orbit_their_planet_at_the_right_distance_and_rate() {
        for m in MOONS {
            // The orbit is a circle, so the radius is invariant...
            for k in 0..60 {
                let jd = 2_451_545.0 + k as f64 * m.period_d / 60.0;
                assert!((m.offset(jd).len() - m.a).abs() < 1e-9, "{} changed radius", m.name);
            }
            // ...and one period brings it back to where it started.
            let a = m.offset(2_460_000.0);
            let b = m.offset(2_460_000.0 + m.period_d);
            assert!(angle_deg(a, b) < 0.01, "{} is not periodic: {}°", m.name, angle_deg(a, b));
            // Past 90° the orbit runs backwards *around the ecliptic*, which is
            // ordinary for a moon of a planet that is itself tipped past 90°
            // (all five Uranian ones) and remarkable only for Triton.
            let ecliptic_retrograde = m.incl_deg > 90.0;
            assert_eq!(
                ecliptic_retrograde,
                m.parent == Planet::Uranus || m.name == "Triton",
                "{}: ecliptic-retrograde = {ecliptic_retrograde}",
                m.name
            );
        }
    }

    /// The Laplace resonance, which is the strongest possible check on three
    /// independent rows of the moon table: Io, Europa and Ganymede have mean
    /// motions satisfying `n₁ − 3n₂ + 2n₃ = 0` to the precision the periods are
    /// known at. No typo survives it — and note the naive form (period ratios
    /// of exactly 2) is *not* true, they are 2.0073.
    #[test]
    fn the_galilean_moons_are_in_laplace_resonance() {
        let n: Vec<f64> = Planet::Jupiter.moons().map(|m| 360.0 / m.period_d).collect();
        assert_eq!(n.len(), 4, "the four Galilean moons");
        let residual = n[0] - 3.0 * n[1] + 2.0 * n[2];
        assert!(residual.abs() < 1e-3, "n1 − 3n2 + 2n3 = {residual} °/day");
        // The ratios themselves are near 2 but not 2, which is the point.
        assert!((n[0] / n[1] - 2.0).abs() < 0.02 && (n[1] / n[2] - 2.0).abs() < 0.02);
    }

    /// The body frames: a planet's pole must point where the IAU says it does,
    /// and the frame must stay orthonormal and right-handed as it turns.
    #[test]
    fn body_frames_are_orthonormal_and_point_at_the_real_poles() {
        let jd = ephem::julian_day(1_784_937_600.0);
        for p in Planet::ALL {
            let b = p.basis(jd);
            for v in [b.x, b.y, b.z] {
                assert!((v.len() - 1.0).abs() < 1e-12, "{} basis not unit", p.name());
            }
            assert!(b.x.dot(b.y).abs() < 1e-12 && b.x.dot(b.z).abs() < 1e-12);
            assert!((b.x.cross(b.y) - b.z).len() < 1e-12, "{} is left-handed", p.name());
        }
        // Axial tilt, as the angle between the pole and the orbit normal.
        //
        // The IAU north pole is the one on the *north side of the invariable
        // plane*, not the one the right-hand rule picks. For the two retrograde
        // rotators the two differ, and the published obliquity is the
        // right-hand-rule one — so the angle measured here is its supplement,
        // and the backwards spin lives in a negative W rate instead. Getting
        // that convention wrong is exactly how a planet ends up rotating the
        // wrong way with a pole that still looks plausible, so both halves are
        // asserted.
        let tilt = |p: Planet| {
            let n = p.heliocentric(jd).cross(p.heliocentric(jd + 30.0)).normalize();
            p.basis(jd).z.dot(n).clamp(-1.0, 1.0).acos().to_degrees()
        };
        // Textbook obliquities, about the rotational (right-hand) pole.
        for (p, obliquity) in [
            (Planet::Mercury, 0.03),
            (Planet::Venus, 177.36),
            (Planet::Mars, 25.19),
            (Planet::Jupiter, 3.13),
            (Planet::Saturn, 26.73),
            (Planet::Uranus, 97.77),
            (Planet::Neptune, 28.32),
        ] {
            let retro = p.info().rotation.w[1] < 0.0;
            assert_eq!(
                retro,
                obliquity > 90.0,
                "{}: a body tipped past 90° must have a negative W rate",
                p.name()
            );
            let want = if retro { 180.0 - obliquity } else { obliquity };
            assert!(
                (tilt(p) - want).abs() < 0.6,
                "{} tilt {:.2}°, want {want:.2}°",
                p.name(),
                tilt(p)
            );
        }
    }

    /// Moons orbit in their planet's equatorial plane — and the two tables that
    /// say so were produced independently: the orbit planes come from a fit to
    /// Horizons, the poles from the IAU element set. They can only agree if
    /// both are right.
    ///
    /// Checked as a *plane* rather than a direction, because the sense of the
    /// motion is a separate fact: the Uranian moons run backwards around the
    /// IAU north pole for the same reason Uranus itself does.
    #[test]
    fn moon_orbits_sit_in_their_planets_equator() {
        let jd = 2_451_545.0;
        let orbit_normal =
            |m: &Moon| m.offset(jd).cross(m.offset(jd + m.period_d * 0.25)).normalize();
        for m in MOONS {
            let pole = m.parent.basis(jd).z;
            let off = orbit_normal(m).dot(pole).clamp(-1.0, 1.0).acos().to_degrees();
            // Fold to the plane: 0° and 180° are both "in the equator".
            let plane_off = off.min(180.0 - off);
            let limit = match m.name {
                // Iapetus is the textbook exception: far enough out that the
                // Sun's pull tilts its orbit ~15° off Saturn's equator, towards
                // Saturn's orbital plane.
                "Iapetus" => 17.0,
                // Triton's orbit is 157° to Neptune's equator — the most famous
                // irregular orbit in the system, checked exactly below.
                "Triton" => 90.0,
                _ => 5.0,
            };
            assert!(
                plane_off < limit,
                "{} orbit is {plane_off:.1}° off {}'s equatorial plane",
                m.name,
                m.parent.name()
            );

            // ...and the sense of the motion matches the planet's own spin,
            // Triton excepted.
            let prograde = orbit_normal(m).dot(pole) > 0.0;
            let spins_forward = m.parent.info().rotation.w[1] > 0.0;
            assert_eq!(
                prograde,
                spins_forward && m.name != "Triton",
                "{} runs the wrong way around {}",
                m.name,
                m.parent.name()
            );
        }
        let t = MOONS.iter().find(|m| m.name == "Triton").unwrap();
        let off =
            orbit_normal(t).dot(Planet::Neptune.basis(jd).z).clamp(-1.0, 1.0).acos().to_degrees();
        assert!((off - 157.0).abs() < 3.0, "Triton is {off:.1}° to Neptune's equator");
    }

    #[test]
    fn rings_are_outside_the_planet_and_inside_its_moons() {
        for p in Planet::ALL {
            let Some(r) = p.info().rings else { continue };
            assert!(r.inner > 1.0 && r.outer > r.inner, "{}: {r:?}", p.name());
            let innermost = p.moons().map(|m| m.a).fold(f64::MAX, f64::min);
            assert!(
                r.outer * p.info().radius < innermost,
                "{}'s rings reach past its innermost moon",
                p.name()
            );
        }
    }
}
