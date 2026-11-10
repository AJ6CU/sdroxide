//! Dwarf planets, near-Earth asteroids and periodic comets — the bodies the
//! next fifty years actually turn on.
//!
//! Same frame and units as [`crate::ephem`] and [`crate::planets`]:
//! heliocentric ecliptic **of date**, right-handed, gigametres. Solved in J2000
//! and rotated forward by the same general precession the planets use, so a
//! comet and the Earth can be differenced directly.
//!
//! ## Which bodies, and why those
//!
//! Not a judgement call. `tools/fit_smallbodies.py` asks JPL's close-approach
//! database for everything that passes inside 0.02 AU of the Earth between 2026
//! and 2076 and is bright enough to be worth naming, and those objects are in
//! the table with the date and distance of the pass written beside them. On top
//! of that sit the bodies anyone would expect to find — the five dwarf planets,
//! the large main-belt asteroids, the mission targets, and the periodic comets
//! with a perihelion inside the window. The tool checks that last claim per
//! comet and drops any that fails it, which is why Swift-Tuttle is absent: the
//! Perseids' parent does not come back until 2126.
//!
//! ## How well they are placed
//!
//! One Keplerian ellipse per body is what [`crate::planets`] uses, and for a
//! body out beyond Neptune it is superb — Eris holds 0.004° across the whole
//! fifty years. It falls apart for anything that crosses a planet's path:
//! fitted in one piece, Encke ends up 54° from where it really is.
//!
//! So each body carries a *chain* of ellipses instead. The window is
//! subdivided until every piece holds, each piece is fitted over its span
//! widened by [`BLEND_D`], and the two either side of a boundary are
//! cross-faded across that width — without which a body would visibly jump as
//! the clock was scrubbed through a boundary. [`SmallBody::fit_error_deg`] is
//! the worst error the finished chain leaves against Horizons, measured
//! *through the blend* rather than against the individual fits, and the tests
//! below replay `tests/fixtures/smallbodies.json` to assert it.
//!
//! Every body lands inside 0.16° across the whole fifty years, and the typical
//! body is an order better than that. Apophis is the exception at 0.66°: the
//! Earth bends its path on 13 April 2029, changing its year from eleven months
//! to thirteen, and twenty-four arcs is not quite enough to carry a fast
//! Earth-crossing orbit through that and out the far side. Its median error is
//! still 0.03°.
//!
//! Which is to say: this places Apophis on the right orbit arriving within a
//! day or so of the right time, and it is nowhere near an impact monitor. The
//! 38 000 km its caption quotes comes from JPL's own close-approach solution,
//! not from here — the right division of labour, because this model could not
//! produce that number and should not look as though it had.
//!
//! Outside 2026–2076 the first and last arcs simply run on. That is a two-body
//! extrapolation of a perturbed orbit and it decays quickly; [`covers`] is what
//! the view asks before believing any of it.

use crate::ephem::AU;
use crate::planets::precess;
use crate::vec3::{Vec3, vec3};

/// What kind of body it is — which decides how it is drawn and how it is
/// grouped in the view.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Class {
    /// Big enough to be round, not big enough to have cleared its orbit.
    Dwarf,
    Asteroid,
    Comet,
}

/// What a body grows when the Sun gets to it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tail {
    None,
    /// Dust only. Phaethon is the case that needs this: an asteroid that comes
    /// close enough to the Sun to shed a dust tail, but has no ice left to make
    /// an ion one — the reason it is called a rock comet.
    Dust,
    /// Both, as any active comet has: a straight blue ion tail pointing away
    /// from the Sun, and a broader dust tail curving back along the orbit.
    Ion,
}

/// The window `tools/fit_smallbodies.py` fitted, as Julian days: 2026-01-01 to
/// 2076-01-01.
pub const WINDOW: (f64, f64) = (2_461_041.5, 2_479_308.5);

/// How long the cross-fade at an arc boundary lasts, days. The tool fits each
/// arc over its own span widened by this much and measures the published error
/// through the same blend, so the two numbers have to agree.
///
/// Short on purpose. A boundary exists where the orbit *changed*, and the two
/// arcs meeting there agree on where the body is but not on where it is going;
/// averaging them over a long fade therefore cuts the corner off a real bend.
/// Apophis is the case that proves it — a nine-day fade across its 2029
/// encounter put it 4.8 million km wrong, and half a day puts it 0.3 million.
pub const BLEND_D: f64 = 0.5;

/// Beyond this distance from the Sun, in AU, water ice is too cold to
/// sublimate: no gas, no coma, no tails. The classical figure is 2.5–3 AU and
/// it is why a comet is a faint smudge for most of its orbit and a spectacle
/// for a few months of it.
pub const ACTIVE_AU: f64 = 3.0;

/// The same line for a body with no ice left. Phaethon's dust comes from its
/// own rock being cooked apart, which needs the 1000 K it only reaches within a
/// fifth of an AU of the Sun — an order closer in than a comet needs, and why
/// its tail is a brief spurt rather than a season.
pub const ROCK_ACTIVE_AU: f64 = 0.2;

/// Speed of the solar wind, km/s. It varies from 300 in the slow streams to
/// 750 in the fast ones; 400 is the usual quiet-Sun figure, and what sets the
/// few degrees by which an ion tail trails the anti-solar line.
pub const SOLAR_WIND_KM_S: f64 = 400.0;

/// km/s to Gm/day: 86 400 seconds of a kilometre is 8.64e-5 gigametres.
const KM_S_TO_GM_D: f64 = 86_400.0 / 1.0e6;

/// Halley's nucleus radius, Gm — the size the tail model is scaled against.
const HALLEY_RADIUS: f64 = 0.000_005_5;

/// Ion-tail length, gigametres, for a Halley-sized nucleus at full activity.
///
/// 40 Gm is 0.27 AU. Halley's ion tail ran to about a third of that at the 1986
/// apparition, where its activity works out at 0.73 — so this reference puts it
/// at 29 Gm, which is what was photographed. The number is a picture rather
/// than a measurement, and it is here rather than in the shader so that it can
/// be argued with.
const TAIL_REFERENCE_GM: f64 = 40.0;

/// Coma diameter, gigametres, at the same reference. A big comet's coma reaches
/// 10⁵ km and can exceed the Sun; a tenth of a gigametre is 100 000 km.
const COMA_REFERENCE_GM: f64 = 0.12;

/// A comet's tails as they stand at one instant — see [`SmallBody::tails`].
#[derive(Debug, Clone, Copy)]
pub struct Tails {
    /// [`SmallBody::activity`] at this instant, for brightness.
    pub activity: f64,
    /// Unit vector the ion tail runs along, away from the Sun and lagging it by
    /// the aberration angle.
    pub ion: Vec3,
    /// Ion-tail length, gigametres. Zero for a body with only dust.
    pub lag: Vec3,
    pub ion_gm: f64,
    /// Dust-tail length, gigametres, and the direction it curves towards —
    /// perpendicular to [`Tails::ion`], back along the orbit.
    pub dust_gm: f64,
    /// Coma diameter, gigametres.
    pub coma_gm: f64,
}

impl Tails {
    /// How far the ion tail is swung off the straight anti-solar line, degrees.
    ///
    /// Published rather than merely applied: it is the one number that says
    /// whether the aberration is being modelled at all, and the tests pin it
    /// against the few degrees photographs show.
    pub fn aberration_deg(&self, sunward: Vec3) -> f64 {
        self.ion.dot(sunward).clamp(-1.0, 1.0).acos().to_degrees()
    }
}

/// One Keplerian ellipse, valid from `start_jd` until the next arc begins.
///
/// The mean motion is stored rather than derived from `a`, because the fit is
/// allowed to trim it by up to 2% to absorb the along-track drift a perturbed
/// orbit accumulates. Deriving it here would quietly throw that away.
#[derive(Debug, Clone, Copy)]
pub struct OrbitArc {
    /// First Julian day this arc owns.
    pub start_jd: f64,
    /// Semi-major axis, AU.
    pub a: f64,
    pub e: f64,
    /// Inclination to the J2000 ecliptic, degrees.
    pub incl: f64,
    /// Longitude of the ascending node, degrees.
    pub node: f64,
    /// Argument of perihelion, degrees — measured from the node, unlike
    /// [`crate::planets`], whose JPL table uses the longitude of perihelion.
    pub peri: f64,
    /// Mean anomaly at J2000.0, degrees.
    pub m0: f64,
    /// Mean motion, degrees per day.
    pub n: f64,
}

impl OrbitArc {
    /// Position on this ellipse alone, AU, J2000 ecliptic.
    fn position(&self, jd: f64) -> Vec3 {
        let m = self.m0 + self.n * (jd - 2_451_545.0);
        self.at_eccentric(eccentric_anomaly(wrap180(m), self.e))
    }

    /// Position at a given eccentric anomaly, AU, J2000 ecliptic.
    ///
    /// The parameterisation to draw the ellipse with: stepping this evenly
    /// walks evenly *around* the orbit, where stepping time crowds the samples
    /// at aphelion and skips the corner at perihelion. On Phaethon, whose
    /// perihelion is 0.14 AU, uniform-in-time sampling cut that corner by 3%.
    fn at_eccentric(&self, ea_deg: f64) -> Vec3 {
        let ea = ea_deg.to_radians();
        let px = self.a * (ea.cos() - self.e);
        let py = self.a * (1.0 - self.e * self.e).max(0.0).sqrt() * ea.sin();

        let (cw, sw) = (self.peri.to_radians().cos(), self.peri.to_radians().sin());
        let (co, so) = (self.node.to_radians().cos(), self.node.to_radians().sin());
        let (ci, si) = (self.incl.to_radians().cos(), self.incl.to_radians().sin());
        vec3(
            (cw * co - sw * so * ci) * px + (-sw * co - cw * so * ci) * py,
            (cw * so + sw * co * ci) * px + (-sw * so + cw * co * ci) * py,
            (sw * si) * px + (cw * si) * py,
        )
    }

    /// Sidereal period, days.
    pub fn period_d(&self) -> f64 {
        360.0 / self.n
    }

    /// Perihelion and aphelion distance, AU.
    pub fn q(&self) -> f64 {
        self.a * (1.0 - self.e)
    }

    pub fn aphelion(&self) -> f64 {
        self.a * (1.0 + self.e)
    }

    /// The perihelion passage at or after `jd`, as a Julian day.
    pub fn next_perihelion(&self, jd: f64) -> f64 {
        // M = 0 at perihelion, so the passages are where m0 + n(t − J2000) is a
        // multiple of 360.
        let turns = (self.m0 + self.n * (jd - 2_451_545.0)) / 360.0;
        2_451_545.0 + (360.0 * turns.ceil() - self.m0) / self.n
    }
}

/// A dwarf planet, asteroid or comet, and everything the view says about it.
#[derive(Debug, Clone, Copy)]
pub struct SmallBody {
    /// What it is called: `Apophis`, `Halley`, `2024 YR4`.
    pub name: &'static str,
    /// Its catalogue entry in full: `99942 Apophis`, `1P/Halley`. Searching
    /// matches either this or the name, so a number finds it as well as a word.
    pub designation: &'static str,
    pub class: Class,
    pub tail: Tail,
    /// Mean radius, gigametres. For a comet this is the nucleus, which is never
    /// what you see; for the unnamed close-approach objects it is inferred from
    /// brightness rather than measured, and their captions say so.
    pub radius: f64,
    /// Worst heliocentric direction error the arc chain leaves against JPL
    /// Horizons across 2026–2076, degrees. See the module docs — this is a
    /// measured number, not a hoped-for one, and the tests assert it.
    pub fit_error_deg: f64,
    /// Why this body is in a fifty-year view at all. Shown in the info card.
    pub why: &'static str,
    arcs: &'static [OrbitArc],
}

impl SmallBody {
    /// Heliocentric position, gigametres, ecliptic of date.
    pub fn heliocentric(&self, jd: f64) -> Vec3 {
        precess(self.position_j2000(jd) * AU, jd)
    }

    /// The blended chain, in AU and the J2000 frame it was fitted in.
    fn position_j2000(&self, jd: f64) -> Vec3 {
        let i = self.arc_index(jd);
        let cur = self.arcs[i].position(jd);
        // Only ever two arcs at once: every arc is far longer than two blends,
        // so a boundary's fade cannot reach the next boundary along.
        let start = self.arcs[i].start_jd;
        if i > 0 && jd < start + BLEND_D {
            let u = smoothstep((jd - (start - BLEND_D)) / (2.0 * BLEND_D));
            return self.arcs[i - 1].position(jd) * (1.0 - u) + cur * u;
        }
        if let Some(next) = self.arcs.get(i + 1)
            && jd >= next.start_jd - BLEND_D
        {
            let u = smoothstep((jd - (next.start_jd - BLEND_D)) / (2.0 * BLEND_D));
            return cur * (1.0 - u) + next.position(jd) * u;
        }
        cur
    }

    /// Which arc owns this instant. Before the first and after the last, the
    /// end arcs run on — see the module docs on what that is worth.
    fn arc_index(&self, jd: f64) -> usize {
        self.arcs.partition_point(|a| a.start_jd <= jd).saturating_sub(1)
    }

    /// The osculating ellipse in force at this instant, for the numbers the
    /// info card quotes — period, perihelion, aphelion.
    pub fn arc(&self, jd: f64) -> &'static OrbitArc {
        &self.arcs[self.arc_index(jd)]
    }

    /// Distance from the Sun, AU.
    pub fn distance_au(&self, jd: f64) -> f64 {
        self.position_j2000(jd).len()
    }

    /// The next perihelion passage at or after `jd`, as a Julian day.
    ///
    /// Solved from the arc in force *at the answer*, not at `jd`: on a comet
    /// with a six-year period the passage is usually several arcs away, and the
    /// elements will have moved by then. Two rounds converge — the arcs of one
    /// body describe near-enough the same ellipse, so the first answer is
    /// already within days of the second.
    pub fn next_perihelion(&self, jd: f64) -> f64 {
        let mut t = self.arc(jd).next_perihelion(jd);
        for _ in 0..4 {
            let next = self.arc(t).next_perihelion(jd);
            if (next - t).abs() < 0.5 {
                return next;
            }
            t = next;
        }
        t
    }

    /// Heliocentric velocity, gigametres per day, ecliptic of date.
    ///
    /// A central difference rather than a closed form: the model is a *blend*
    /// of two ellipses near an arc boundary, and the analytic velocity of
    /// either one alone would be wrong exactly where the blend is doing its
    /// work. Half a day is short against every period here — the fastest body
    /// in the table takes eleven months — and long enough to stay well clear of
    /// f64 cancellation.
    pub fn velocity(&self, jd: f64) -> Vec3 {
        (self.heliocentric(jd + 0.25) - self.heliocentric(jd - 0.25)) * 2.0
    }

    /// The ellipse it is on *now*, as a closed path in gigametres, ecliptic of
    /// date — one full revolution of the arc in force at `jd`.
    ///
    /// Drawn from a single arc rather than by walking the chain, and that is
    /// deliberate: walking the chain over Eris's 559-year year would run
    /// centuries outside the window the table was fitted over, and over
    /// Apophis's would splice the orbit it is on before 2029 to the different
    /// one it is on afterwards. One arc is one honest ellipse, and for a body
    /// whose orbit is about to be changed, the one it is on now is the answer
    /// to what it is doing now.
    pub fn orbit_path(&self, jd: f64, steps: usize) -> impl Iterator<Item = Vec3> {
        let arc = self.arc(jd);
        (0..=steps)
            .map(move |k| precess(arc.at_eccentric(360.0 * k as f64 / steps as f64) * AU, jd))
    }

    /// How far out the Sun can still drive this body's tail, AU.
    fn active_au(&self) -> f64 {
        match self.tail {
            Tail::None => 0.0,
            Tail::Dust => ROCK_ACTIVE_AU,
            Tail::Ion => ACTIVE_AU,
        }
    }

    /// How hard the Sun is working on it, 0 to 1.
    ///
    /// Sunlight falls off as the square of the distance and a comet's gas
    /// production follows it closely, so that is the whole model: saturated
    /// inside the 0.5 AU where the ices boil as fast as the surface can shed
    /// them, and faded out through the [`active_au`] line where the Sun stops
    /// being able to drive anything at all. Zero for a body with nothing to
    /// shed.
    ///
    /// [`active_au`]: SmallBody::active_au
    pub fn activity(&self, jd: f64) -> f64 {
        let limit = self.active_au();
        if limit <= 0.0 {
            return 0.0;
        }
        let r = self.distance_au(jd).max(1e-6);
        let insolation = (1.0 / (r * r) / 4.0).clamp(0.0, 1.0);
        // Softened over the last sixth of the range rather than switched, so a
        // comet fades in over weeks instead of appearing.
        let fade = limit / 6.0;
        insolation * (1.0 - smoothstep((r - (limit - fade)) / fade))
    }

    /// The tails as they stand at this instant, or `None` when the body has
    /// none or is too far out to have grown them.
    ///
    /// Everything here is geometry the renderer needs and physics decides:
    ///
    /// * The **ion tail** points along the solar wind *as the comet meets it*,
    ///   which is not quite away from the Sun. The wind blows radially at some
    ///   [`SOLAR_WIND_KM_S`]; the comet is crossing it at its own orbital
    ///   speed; the ions are picked up by what is left, so the tail lies along
    ///   `v_wind − v_comet` and trails the anti-solar line by a few degrees.
    ///   That lag is the aberration angle, it is real, it is what makes a
    ///   photograph of a comet look the way it does, and it costs one
    ///   subtraction to get right.
    ///
    /// * The **dust tail** is grains, not ions. They are far too heavy for the
    ///   wind to sweep, so radiation pressure eases them outwards while they
    ///   keep the orbital velocity they were released with — and the tail bends
    ///   away from the anti-solar line, back along the orbit, more the further
    ///   from the nucleus you look.
    ///
    /// Lengths are the one part that is a picture rather than a measurement.
    /// They scale with [`SmallBody::activity`] and with the cube root of the
    /// nucleus radius — more surface, more gas — against a reference chosen so
    /// Halley at its 2061 perihelion draws the tail Halley actually had.
    pub fn tails(&self, jd: f64) -> Option<Tails> {
        let activity = self.activity(jd);
        if activity <= 0.01 {
            return None;
        }
        let pos = self.heliocentric(jd);
        let sunward = pos.normalize();
        // Bigger nucleus, more sublimating surface, more gas — but weakly, so
        // a 0.6 km comet is a third of a 5.5 km one rather than a hundredth.
        let size = (self.radius / HALLEY_RADIUS).cbrt().clamp(0.3, 1.5);

        // The wind the comet flies into, in gigametres per day so it can be
        // subtracted from the orbital velocity directly.
        let wind = sunward * (SOLAR_WIND_KM_S * KM_S_TO_GM_D);
        let ion = (wind - self.velocity(jd)).normalize();
        // What is left of the orbital motion once the ion direction is taken
        // out: the way the dust tail curves.
        let v = self.velocity(jd);
        let lag = (v * -1.0 - ion * (v * -1.0).dot(ion)).normalize();

        let full = TAIL_REFERENCE_GM * activity * size;
        let (ion_gm, dust_gm) = match self.tail {
            Tail::None => return None,
            // A rock has no ions to lose, only dust to shed, and not much of it.
            Tail::Dust => (0.0, full * 0.10),
            Tail::Ion => (full, full * 0.45),
        };
        Some(Tails {
            activity,
            ion,
            ion_gm,
            lag,
            dust_gm,
            // The coma is the nucleus's own atmosphere: 10⁵ km across on an
            // active comet, and the only part of it big enough to see.
            coma_gm: COMA_REFERENCE_GM * activity.sqrt() * size,
        })
    }

    /// Does the query string name this body? Case-insensitive substring, over
    /// both the name and the full designation — so `apophis`, `99942` and
    /// `1P` all find what you would expect.
    pub fn matches(&self, query: &str) -> bool {
        let q = query.trim();
        !q.is_empty() && (contains_fold(self.name, q) || contains_fold(self.designation, q))
    }
}

/// Is `jd` inside the window the table was fitted over?
pub fn covers(jd: f64) -> bool {
    (WINDOW.0..=WINDOW.1).contains(&jd)
}

/// Look a body up by name, for tests and for anything that wants one by hand.
pub fn find(name: &str) -> Option<&'static SmallBody> {
    BODIES.iter().find(|b| b.name.eq_ignore_ascii_case(name))
}

/// Every body matching a search, in table order.
pub fn search(query: &str) -> impl Iterator<Item = (usize, &'static SmallBody)> {
    let q = query.trim().to_string();
    BODIES.iter().enumerate().filter(move |(_, b)| b.matches(&q))
}

fn smoothstep(t: f64) -> f64 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn wrap180(d: f64) -> f64 {
    let r = d.rem_euclid(360.0);
    if r > 180.0 { r - 360.0 } else { r }
}

/// Newton's method on Kepler's equation, in degrees — JPL's own formulation,
/// as [`crate::planets`] uses. Comets reach e = 0.97, where the naive starting
/// guess is poor, so this iterates further than the planets' solver needs to.
fn eccentric_anomaly(m_deg: f64, e: f64) -> f64 {
    let e_star = e.to_degrees();
    let mut ea = m_deg + e_star * m_deg.to_radians().sin();
    for _ in 0..64 {
        let dm = m_deg - (ea - e_star * ea.to_radians().sin());
        let de = dm / (1.0 - e * ea.to_radians().cos());
        ea += de;
        if de.abs() < 1e-11 {
            break;
        }
    }
    ea
}

/// ASCII-case-insensitive substring test, allocation-free.
fn contains_fold(haystack: &str, needle: &str) -> bool {
    let (h, n) = (haystack.as_bytes(), needle.as_bytes());
    !n.is_empty() && h.len() >= n.len() && h.windows(n.len()).any(|w| w.eq_ignore_ascii_case(n))
}

/// Terse constructors, so the generated table stays one line per arc.
#[allow(clippy::too_many_arguments)]
const fn small(
    name: &'static str,
    designation: &'static str,
    class: Class,
    radius: f64,
    tail: Tail,
    fit_error_deg: f64,
    why: &'static str,
    arcs: &'static [OrbitArc],
) -> SmallBody {
    SmallBody { name, designation, class, tail, radius, fit_error_deg, why, arcs }
}

#[allow(clippy::too_many_arguments)]
const fn arc(
    start_jd: f64,
    a: f64,
    e: f64,
    incl: f64,
    node: f64,
    peri: f64,
    m0: f64,
    n: f64,
) -> OrbitArc {
    OrbitArc { start_jd, a, e, incl, node, peri, m0, n }
}

/// Every small body the view draws, as produced by `tools/fit_smallbodies.py`.
///
/// **The order is a persisted format**, exactly as [`crate::planets::MOONS`] is:
/// the 3D view stores its camera target as an index into this table, so a new
/// body is appended at the *end*. Inserting one mid-table would re-point
/// everybody's saved target at a different body.
///
/// Each row is a body followed by its arcs, each arc valid from its own Julian
/// day until the next one starts. The trailing comment on each body is the
/// tool's own report: perihelion distance, how many arcs it took, the worst
/// radial error, and the perihelion passages that fall inside the window.
pub static BODIES: &[SmallBody] = &[
    small(
        "Pluto",
        "134340 Pluto",
        Class::Dwarf,
        0.001188300,
        Tail::None,
        0.011,
        "Largest Kuiper-belt body; New Horizons flew past in July 2015",
        &[arc(
            2461041.5,
            39.475028489,
            0.249086998,
            17.141124,
            110.303469,
            113.690528,
            14.885575,
            0.003977190,
        )],
    ), // q 29.642 AU, 1 arc(s), radial 0.018%
    small(
        "Ceres",
        "1 Ceres",
        Class::Dwarf,
        0.000469700,
        Tail::None,
        0.085,
        concat!(
            "Largest main-belt body and the only dwarf planet inside Neptune; Dawn orbited it ",
            "2015-18"
        ),
        &[
            arc(
                2461041.5,
                2.766508020,
                0.079080362,
                10.582663,
                80.188120,
                72.528923,
                7.096958,
                0.214188762,
            ),
            arc(
                2464351.5,
                2.770334012,
                0.075841939,
                10.590942,
                79.940242,
                73.299013,
                12.551818,
                0.213702854,
            ),
            arc(
                2464801.5,
                2.766532523,
                0.075251652,
                10.595498,
                79.943598,
                74.199737,
                5.115975,
                0.214199987,
            ),
            arc(
                2464981.5,
                2.768180315,
                0.076129077,
                10.596445,
                79.918956,
                74.624245,
                7.294848,
                0.214009620,
            ),
            arc(
                2467871.5,
                2.766756336,
                0.077631010,
                10.600091,
                79.886702,
                73.713120,
                5.824525,
                0.214160271,
            ),
            arc(
                2469821.5,
                2.766861030,
                0.077583658,
                10.590950,
                79.833984,
                72.563316,
                6.751099,
                0.214172411,
            ),
            arc(
                2471341.5,
                2.767071980,
                0.076767255,
                10.592138,
                79.816881,
                72.709420,
                7.405879,
                0.214127805,
            ),
            arc(
                2471911.5,
                2.770134565,
                0.076520107,
                10.578811,
                79.680886,
                74.338871,
                15.382460,
                0.213668407,
            ),
            arc(
                2473461.5,
                2.766862875,
                0.077665415,
                10.584528,
                79.507192,
                76.097802,
                3.955024,
                0.214127867,
            ),
            arc(
                2476511.5,
                2.766092397,
                0.077956758,
                10.588780,
                79.457387,
                74.902284,
                0.940018,
                0.214297223,
            ),
            arc(
                2478311.5,
                2.766883122,
                0.076148153,
                10.585449,
                79.434752,
                73.790745,
                5.471335,
                0.214160907,
            ),
        ],
    ), // q 2.548 AU, 11 arc(s), radial 0.205%  perihelion 2027-07-09, 2032-02-14, 2036-09-29, 2041-05-10, …
    small(
        "Eris",
        "136199 Eris",
        Class::Dwarf,
        0.001163000,
        Tail::None,
        0.004,
        "More massive than Pluto — the discovery that ended Pluto's planethood",
        &[arc(
            2461041.5,
            67.816746022,
            0.438780030,
            43.991107,
            35.977651,
            151.243285,
            193.800879,
            0.001766628,
        )],
    ), // q 38.060 AU, 1 arc(s), radial 0.008%
    small(
        "Haumea",
        "136108 Haumea",
        Class::Dwarf,
        0.000780000,
        Tail::None,
        0.009,
        "Spins in under four hours, which has pulled it into an egg, and carries a ring",
        &[arc(
            2461041.5,
            43.095332947,
            0.195086958,
            28.205952,
            121.941143,
            239.992224,
            190.366814,
            0.003486257,
        )],
    ), // q 34.688 AU, 1 arc(s), radial 0.016%
    small(
        "Makemake",
        "136472 Makemake",
        Class::Dwarf,
        0.000715000,
        Tail::None,
        0.007,
        "Brightest Kuiper-belt object after Pluto",
        &[arc(
            2461041.5,
            45.501602202,
            0.160290889,
            29.003358,
            79.436862,
            296.069176,
            140.115493,
            0.003213087,
        )],
    ), // q 38.208 AU, 1 arc(s), radial 0.014%
    small(
        "Vesta",
        "4 Vesta",
        Class::Asteroid,
        0.000262700,
        Tail::None,
        0.097,
        concat!(
            "Brightest asteroid — the only one ever visible to the naked eye; Dawn orbited it in ",
            "2011"
        ),
        &[
            arc(
                2461041.5,
                2.361471591,
                0.089848818,
                7.141912,
                103.654091,
                151.297830,
                339.339461,
                0.271567417,
            ),
            arc(
                2464641.5,
                2.361360628,
                0.089909036,
                7.137376,
                103.568277,
                150.912679,
                339.412512,
                0.271603698,
            ),
            arc(
                2469141.5,
                2.362842404,
                0.088668809,
                7.136837,
                103.531492,
                150.866991,
                345.005974,
                0.271295960,
            ),
            arc(
                2470191.5,
                2.361706420,
                0.088736909,
                7.137308,
                103.502121,
                151.238345,
                339.955051,
                0.271553314,
            ),
            arc(
                2473511.5,
                2.361198463,
                0.089043441,
                7.144995,
                103.414774,
                152.438286,
                336.153797,
                0.271666770,
            ),
            arc(
                2474231.5,
                2.361682264,
                0.089131555,
                7.144263,
                103.382456,
                152.167028,
                339.155168,
                0.271547679,
            ),
            arc(
                2477361.5,
                2.360604932,
                0.090511087,
                7.145453,
                103.272695,
                152.608040,
                332.958011,
                0.271769946,
            ),
            arc(
                2477831.5,
                2.361136604,
                0.090482055,
                7.145240,
                103.269297,
                152.225528,
                335.770046,
                0.271676454,
            ),
        ],
    ), // q 2.149 AU, 8 arc(s), radial 0.094%  perihelion 2029-03-30, 2032-11-15, 2036-06-30, 2040-02-16, …
    small(
        "Pallas",
        "2 Pallas",
        Class::Asteroid,
        0.000255500,
        Tail::None,
        0.088,
        "Third-largest main-belt body, on an orbit tilted 35 degrees out of the ecliptic",
        &[
            arc(
                2461041.5,
                2.769058658,
                0.230755103,
                34.934234,
                172.885399,
                310.940820,
                349.185806,
                0.213876263,
            ),
            arc(
                2463011.5,
                2.768774729,
                0.230612928,
                34.922921,
                172.877363,
                310.578417,
                348.303938,
                0.213984844,
            ),
            arc(
                2465031.5,
                2.771495234,
                0.229737836,
                34.898515,
                172.789081,
                310.555959,
                353.752456,
                0.213584970,
            ),
            arc(
                2468091.5,
                2.769557019,
                0.231042716,
                34.937017,
                172.669834,
                310.794697,
                350.034005,
                0.213809988,
            ),
            arc(
                2469901.5,
                2.767954033,
                0.231387014,
                34.952896,
                172.649952,
                310.730427,
                346.476846,
                0.214013093,
            ),
            arc(
                2473111.5,
                2.772932633,
                0.228260663,
                34.908477,
                172.585242,
                310.601077,
                359.639129,
                0.213402569,
            ),
            arc(
                2475711.5,
                2.771671936,
                0.229011528,
                34.907983,
                172.495253,
                310.609142,
                354.736574,
                0.213609330,
            ),
            arc(
                2477471.5,
                2.770533934,
                0.230223213,
                34.921033,
                172.468779,
                310.464449,
                351.926702,
                0.213722110,
            ),
        ],
    ), // q 2.130 AU, 8 arc(s), radial 0.104%  perihelion 2027-10-16, 2032-05-23, 2036-12-30, 2041-08-12, …
    small(
        "Psyche",
        "16 Psyche",
        Class::Asteroid,
        0.000111000,
        Tail::None,
        0.087,
        "Metal-rich remnant core; NASA's Psyche arrives in 2029",
        &[
            arc(
                2461041.5,
                2.921253734,
                0.137995923,
                3.104283,
                149.955058,
                229.680883,
                333.824226,
                0.197403569,
            ),
            arc(
                2464261.5,
                2.921247997,
                0.139608701,
                3.103618,
                149.911124,
                229.094352,
                334.744222,
                0.197384537,
            ),
            arc(
                2465291.5,
                2.920542503,
                0.138816497,
                3.104595,
                149.884108,
                228.536507,
                334.447405,
                0.197459212,
            ),
            arc(
                2467491.5,
                2.923073034,
                0.135893653,
                3.108591,
                149.730876,
                228.167883,
                338.719521,
                0.197214670,
            ),
            arc(
                2468571.5,
                2.924338243,
                0.134744211,
                3.108815,
                149.726732,
                228.424354,
                341.182640,
                0.197062222,
            ),
            arc(
                2469831.5,
                2.926474474,
                0.134521167,
                3.108723,
                149.709693,
                229.296430,
                344.915679,
                0.196823556,
            ),
            arc(
                2471191.5,
                2.923501351,
                0.135416259,
                3.107621,
                149.582901,
                230.735033,
                337.583484,
                0.197126484,
            ),
            arc(
                2473411.5,
                2.921334567,
                0.136102550,
                3.107978,
                149.573587,
                231.163721,
                331.319340,
                0.197400108,
            ),
            arc(
                2473711.5,
                2.920585620,
                0.138701900,
                3.113384,
                149.462973,
                230.471906,
                329.246753,
                0.197515831,
            ),
            arc(
                2476371.5,
                2.919548261,
                0.139715840,
                3.115053,
                149.465114,
                230.081768,
                327.783239,
                0.197591351,
            ),
            arc(
                2477111.5,
                2.919769389,
                0.139086144,
                3.114978,
                149.449039,
                229.217068,
                328.972220,
                0.197570530,
            ),
            arc(
                2477991.5,
                2.922132137,
                0.138059674,
                3.114536,
                149.428622,
                229.122427,
                336.558562,
                0.197290983,
            ),
        ],
    ), // q 2.518 AU, 12 arc(s), radial 0.317%  perihelion 2030-04-28, 2035-04-22, 2040-04-15, 2045-04-11, …
    small(
        "Eros",
        "433 Eros",
        Class::Asteroid,
        0.000008420,
        Tail::None,
        0.070,
        "Largest near-Earth asteroid; NEAR Shoemaker landed on it in 2001",
        &[
            arc(
                2461041.5,
                1.458263244,
                0.222833639,
                10.827621,
                304.218137,
                179.003881,
                58.401347,
                0.559687926,
            ),
            arc(
                2468801.5,
                1.458069186,
                0.222724416,
                10.826254,
                304.113839,
                179.198303,
                56.051669,
                0.559820044,
            ),
            arc(
                2474611.5,
                1.458266668,
                0.222808248,
                10.825566,
                304.033815,
                179.366299,
                59.124614,
                0.559683773,
            ),
        ],
    ), // q 1.133 AU, 3 arc(s), radial 0.040%  perihelion 2026-02-17, 2027-11-22, 2029-08-26, 2031-06-01, …
    small(
        "Phaethon",
        "3200 Phaethon",
        Class::Asteroid,
        0.000002720,
        Tail::Dust,
        0.085,
        concat!(
            "Parent of the Geminid meteor shower; sheds a dust tail at perihelion, DESTINY+ ",
            "target"
        ),
        &[
            arc(
                2461041.5,
                1.271371420,
                0.889679929,
                22.311448,
                265.091271,
                322.303514,
                142.799638,
                0.687554056,
            ),
            arc(
                2461811.5,
                1.271493289,
                0.889764054,
                22.319984,
                265.065456,
                322.322030,
                144.092299,
                0.687428067,
            ),
            arc(
                2462331.5,
                1.271515956,
                0.889793445,
                22.329852,
                265.045417,
                322.348632,
                144.381025,
                0.687401312,
            ),
            arc(
                2462861.5,
                1.271456540,
                0.889719151,
                22.334014,
                265.040259,
                322.361929,
                143.780604,
                0.687454536,
            ),
            arc(
                2463381.5,
                1.271391192,
                0.889649296,
                22.331856,
                265.045259,
                322.355260,
                142.833731,
                0.687534581,
            ),
            arc(
                2463901.5,
                1.271424457,
                0.889693823,
                22.336931,
                265.016471,
                322.364419,
                143.215002,
                0.687503763,
            ),
            arc(
                2464431.5,
                1.271637750,
                0.889734987,
                22.363791,
                264.944692,
                322.450373,
                146.485437,
                0.687249948,
            ),
            arc(
                2464951.5,
                1.271412752,
                0.889624581,
                22.390003,
                264.914654,
                322.492189,
                142.911344,
                0.687516532,
            ),
            arc(
                2465471.5,
                1.271478285,
                0.889550542,
                22.390907,
                264.903353,
                322.482418,
                143.734451,
                0.687457192,
            ),
            arc(
                2466521.5,
                1.271472235,
                0.889630649,
                22.406503,
                264.870041,
                322.523581,
                143.929869,
                0.687444259,
            ),
            arc(
                2467571.5,
                1.271383062,
                0.889536174,
                22.408599,
                264.868915,
                322.529591,
                142.513411,
                0.687532649,
            ),
            arc(
                2468091.5,
                1.271349940,
                0.889532859,
                22.409518,
                264.856678,
                322.527022,
                141.900159,
                0.687569712,
            ),
            arc(
                2468621.5,
                1.271595450,
                0.889655957,
                22.432355,
                264.781071,
                322.599380,
                146.480788,
                0.687301301,
            ),
            arc(
                2469141.5,
                1.271439545,
                0.889574807,
                22.464849,
                264.742642,
                322.664160,
                143.464162,
                0.687472737,
            ),
            arc(
                2469661.5,
                1.271340600,
                0.889426186,
                22.465108,
                264.738259,
                322.651368,
                141.573256,
                0.687576978,
            ),
            arc(
                2470191.5,
                1.271263980,
                0.889495524,
                22.479840,
                264.707303,
                322.684628,
                140.688937,
                0.687624317,
            ),
            arc(
                2472281.5,
                1.271178335,
                0.889412340,
                22.485327,
                264.692421,
                322.695550,
                138.918384,
                0.687709589,
            ),
            arc(
                2472801.5,
                1.271332789,
                0.889536645,
                22.502035,
                264.628555,
                322.744384,
                142.532658,
                0.687539630,
            ),
            arc(
                2473331.5,
                1.271392216,
                0.889492824,
                22.536130,
                264.575327,
                322.830330,
                143.792584,
                0.687481829,
            ),
            arc(
                2473851.5,
                1.271363343,
                0.889317645,
                22.542802,
                264.563383,
                322.821098,
                142.263699,
                0.687550341,
            ),
            arc(
                2474901.5,
                1.271457280,
                0.889398049,
                22.554070,
                264.534407,
                322.845399,
                144.550498,
                0.687452334,
            ),
            arc(
                2475951.5,
                1.271414541,
                0.889321307,
                22.562937,
                264.510890,
                322.867828,
                143.472687,
                0.687496685,
            ),
            arc(
                2477521.5,
                1.271676173,
                0.889353874,
                22.597963,
                264.420576,
                322.967628,
                150.603523,
                0.687222181,
            ),
            arc(
                2478041.5,
                1.271486556,
                0.889185908,
                22.615839,
                264.398109,
                322.981602,
                144.457557,
                0.687454096,
            ),
        ],
    ), // q 0.140 AU, 24 arc(s), radial 0.097%  perihelion 2026-09-02, 2028-02-07, 2029-07-15, 2030-12-21, …
    small(
        "Didymos",
        "65803 Didymos",
        Class::Asteroid,
        0.000000390,
        Tail::None,
        0.099,
        "DART struck its moon Dimorphos in 2022; ESA's Hera surveys the result from 2026",
        &[
            arc(
                2461041.5,
                1.642766083,
                0.383362163,
                3.414006,
                72.969018,
                319.608577,
                60.979124,
                0.468114735,
            ),
            arc(
                2462931.5,
                1.642542170,
                0.383079269,
                3.414154,
                72.954956,
                319.662633,
                59.782866,
                0.468215076,
            ),
            arc(
                2464011.5,
                1.642698975,
                0.383300957,
                3.413811,
                72.901600,
                319.738675,
                60.765538,
                0.468132560,
            ),
            arc(
                2468281.5,
                1.642992574,
                0.383299685,
                3.412928,
                72.815292,
                319.858807,
                63.021596,
                0.467992980,
            ),
            arc(
                2470661.5,
                1.643014237,
                0.383441986,
                3.413346,
                72.788999,
                319.953706,
                63.660587,
                0.467957879,
            ),
            arc(
                2472061.5,
                1.642969538,
                0.383306127,
                3.415083,
                72.734893,
                319.994289,
                62.954633,
                0.467994928,
            ),
            arc(
                2474681.5,
                1.641708536,
                0.382729811,
                3.429947,
                72.387143,
                320.397220,
                50.838888,
                0.468522590,
            ),
            arc(
                2474911.5,
                1.641298817,
                0.383062552,
                3.430492,
                72.380027,
                320.407741,
                45.685864,
                0.468743254,
            ),
            arc(
                2475101.5,
                1.641349652,
                0.383074979,
                3.430483,
                72.379742,
                320.403887,
                46.388209,
                0.468713689,
            ),
            arc(
                2475191.5,
                1.641480408,
                0.383105610,
                3.430470,
                72.378911,
                320.397128,
                47.815057,
                0.468653636,
            ),
            arc(
                2475281.5,
                1.641759421,
                0.382839672,
                3.430447,
                72.376459,
                320.447087,
                51.666092,
                0.468491180,
            ),
            arc(
                2476071.5,
                1.641482641,
                0.382745633,
                3.430190,
                72.361203,
                320.457387,
                48.067804,
                0.468638892,
            ),
        ],
    ), // q 1.013 AU, 12 arc(s), radial 0.072%  perihelion 2027-01-06, 2029-02-13, 2031-03-24, 2033-05-01, …
    small(
        "Ryugu",
        "162173 Ryugu",
        Class::Asteroid,
        0.000000448,
        Tail::None,
        0.076,
        "Hayabusa2 returned 5 g of it in December 2020",
        &[
            arc(
                2461041.5,
                1.190903009,
                0.191015835,
                5.866358,
                251.280934,
                211.622939,
                299.721923,
                0.758387835,
            ),
            arc(
                2462431.5,
                1.191000025,
                0.191139341,
                5.866647,
                251.263190,
                211.629106,
                300.869074,
                0.758284348,
            ),
            arc(
                2463561.5,
                1.190970768,
                0.191172314,
                5.866374,
                251.257805,
                211.658137,
                300.436534,
                0.758318930,
            ),
            arc(
                2463601.5,
                1.190951225,
                0.191173140,
                5.866402,
                251.257510,
                211.653548,
                300.150745,
                0.758343048,
            ),
            arc(
                2463661.5,
                1.191404571,
                0.191064941,
                5.856255,
                251.200934,
                211.595313,
                304.722702,
                0.757980268,
            ),
            arc(
                2464071.5,
                1.192606676,
                0.192111302,
                5.844112,
                250.934323,
                211.743710,
                319.557430,
                0.756786928,
            ),
            arc(
                2468861.5,
                1.193317646,
                0.192495211,
                5.842963,
                250.826213,
                211.879544,
                331.567645,
                0.756086130,
            ),
            arc(
                2470121.5,
                1.193119267,
                0.192415697,
                5.842089,
                250.785667,
                211.932365,
                328.052075,
                0.756274407,
            ),
            arc(
                2473101.5,
                1.193235093,
                0.192515635,
                5.841337,
                250.735280,
                211.982906,
                331.852194,
                0.756100495,
            ),
            arc(
                2475051.5,
                1.192645769,
                0.192184887,
                5.842105,
                250.662989,
                212.114918,
                316.987863,
                0.756734082,
            ),
        ],
    ), // q 0.963 AU, 10 arc(s), radial 0.099%  perihelion 2026-03-18, 2027-07-06, 2028-10-23, 2030-02-09, …
    small(
        "Itokawa",
        "25143 Itokawa",
        Class::Asteroid,
        0.000000165,
        Tail::None,
        0.071,
        "First asteroid ever sampled — Hayabusa, 2005",
        &[
            arc(
                2461041.5,
                1.324072739,
                0.280105105,
                1.620935,
                69.073604,
                162.834337,
                44.598208,
                0.646891355,
            ),
            arc(
                2462051.5,
                1.324022069,
                0.280206654,
                1.620923,
                69.071886,
                162.847734,
                44.233287,
                0.646925158,
            ),
            arc(
                2463021.5,
                1.323927898,
                0.280139843,
                1.620823,
                69.069856,
                162.866008,
                43.366583,
                0.647000815,
            ),
            arc(
                2463321.5,
                1.323842776,
                0.280161208,
                1.620814,
                69.068363,
                162.852867,
                42.443254,
                0.647080609,
            ),
            arc(
                2463391.5,
                1.323905703,
                0.280105139,
                1.620788,
                69.068899,
                162.856308,
                43.143133,
                0.647021242,
            ),
            arc(
                2463491.5,
                1.322974279,
                0.279722970,
                1.621816,
                68.969681,
                163.019142,
                33.929092,
                0.647775249,
            ),
            arc(
                2464811.5,
                1.321901711,
                0.279361189,
                1.616098,
                68.814504,
                162.950623,
                24.257315,
                0.648512147,
            ),
            arc(
                2466041.5,
                1.322286503,
                0.279520491,
                1.615454,
                68.782747,
                162.989052,
                28.700843,
                0.648205126,
            ),
            arc(
                2468141.5,
                1.322430246,
                0.279586331,
                1.615139,
                68.777795,
                163.009973,
                30.254900,
                0.648110273,
            ),
            arc(
                2469881.5,
                1.322481343,
                0.279588476,
                1.614543,
                68.772015,
                163.035042,
                31.303751,
                0.648054100,
            ),
            arc(
                2475391.5,
                1.322260601,
                0.279472776,
                1.614609,
                68.767759,
                163.098148,
                28.531152,
                0.648168818,
            ),
            arc(
                2476881.5,
                1.322034451,
                0.279300600,
                1.614821,
                68.757737,
                163.065995,
                23.082653,
                0.648389334,
            ),
            arc(
                2477041.5,
                1.322025918,
                0.279405594,
                1.613821,
                68.673999,
                163.151174,
                22.512591,
                0.648411436,
            ),
            arc(
                2477611.5,
                1.322886992,
                0.279647960,
                1.641239,
                67.448916,
                164.183655,
                36.942940,
                0.647861825,
            ),
            arc(
                2478921.5,
                1.324833078,
                0.280354129,
                1.635100,
                67.253706,
                164.529116,
                78.201396,
                0.646340605,
            ),
            arc(
                2478981.5,
                1.324800133,
                0.280385508,
                1.635104,
                67.253672,
                164.528410,
                77.495804,
                0.646366349,
            ),
            arc(
                2479031.5,
                1.324778349,
                0.280405576,
                1.635114,
                67.253654,
                164.529558,
                76.910330,
                0.646387612,
            ),
            arc(
                2479071.5,
                1.324776848,
                0.280406731,
                1.635120,
                67.253716,
                164.529652,
                76.818133,
                0.646390955,
            ),
            arc(
                2479121.5,
                1.324816348,
                0.280395854,
                1.635113,
                67.253485,
                164.523845,
                77.719473,
                0.646358503,
            ),
            arc(
                2479181.5,
                1.324871966,
                0.280393175,
                1.635110,
                67.252772,
                164.515408,
                79.071909,
                0.646309854,
            ),
        ],
    ), // q 0.953 AU, 20 arc(s), radial 0.112%  perihelion 2027-03-28, 2028-10-05, 2030-04-14, 2031-10-23, …
    small(
        "Patroclus",
        "617 Patroclus",
        Class::Asteroid,
        0.000051000,
        Tail::None,
        0.033,
        "Jupiter Trojan, and a binary of two near-equal bodies; Lucy's final flyby in 2033",
        &[
            arc(
                2461041.5,
                5.201178766,
                0.138708989,
                22.061938,
                44.338297,
                308.543568,
                336.348300,
                0.083128389,
            ),
            arc(
                2464151.5,
                5.191572666,
                0.137882067,
                22.073537,
                44.341093,
                309.138038,
                332.967586,
                0.083349679,
            ),
            arc(
                2469251.5,
                5.184980261,
                0.136872736,
                22.075692,
                44.327108,
                309.795100,
                329.692080,
                0.083506883,
            ),
            arc(
                2473451.5,
                5.181009220,
                0.136988440,
                22.072053,
                44.285324,
                310.571791,
                326.928537,
                0.083607404,
            ),
            arc(
                2477681.5,
                5.178322784,
                0.137276459,
                22.074170,
                44.273121,
                311.067468,
                324.676024,
                0.083680065,
            ),
        ],
    ), // q 4.480 AU, 5 arc(s), radial 0.089%  perihelion 2036-05-13, 2048-03-10, 2060-01-11, 2071-10-30
    small(
        "Eurybates",
        "3548 Eurybates",
        Class::Asteroid,
        0.000031900,
        Tail::None,
        0.080,
        "Jupiter Trojan with its own satellite; Lucy flies past in August 2027",
        &[
            arc(
                2461041.5,
                5.224289108,
                0.089156995,
                8.046600,
                43.565383,
                28.311079,
                49.099030,
                0.082564228,
            ),
            arc(
                2464501.5,
                5.252939215,
                0.090515859,
                8.037680,
                43.542073,
                28.787535,
                57.338940,
                0.081892657,
            ),
            arc(
                2468801.5,
                5.279693291,
                0.091008097,
                8.029156,
                43.557198,
                28.732276,
                68.015419,
                0.081272841,
            ),
            arc(
                2473121.5,
                5.294718923,
                0.090961038,
                8.023793,
                43.560505,
                28.522399,
                75.711858,
                0.080921884,
            ),
            arc(
                2478041.5,
                5.291518191,
                0.090278831,
                8.023803,
                43.553335,
                28.666499,
                73.181207,
                0.081015501,
            ),
        ],
    ), // q 4.759 AU, 5 arc(s), radial 0.241%  perihelion 2034-03-09, 2046-03-24, 2058-05-07, 2070-07-09
    small(
        "2024 YR4",
        "2024 YR4",
        Class::Asteroid,
        0.000000030,
        Tail::None,
        0.092,
        concat!(
            "Ruled out for Earth, but still has a few per cent chance of hitting the Moon on 22 ",
            "Dec 2032"
        ),
        &[
            arc(
                2461041.5,
                2.516444186,
                0.661001034,
                3.407214,
                271.381101,
                134.340398,
                275.316822,
                0.246900276,
            ),
            arc(
                2462141.5,
                2.514385626,
                0.660730214,
                3.407645,
                271.369906,
                134.285296,
                272.012066,
                0.247214597,
            ),
            arc(
                2462181.5,
                2.514942102,
                0.660780984,
                3.407508,
                271.367337,
                134.296553,
                272.969790,
                0.247123751,
            ),
            arc(
                2462251.5,
                2.518131011,
                0.660328530,
                3.407744,
                271.224555,
                134.549746,
                278.736064,
                0.246580305,
            ),
            arc(
                2462801.5,
                2.525857376,
                0.655981930,
                3.428785,
                270.791511,
                134.966003,
                291.776743,
                0.245427014,
            ),
            arc(
                2463581.5,
                2.433166093,
                0.644929700,
                3.481068,
                270.702719,
                133.844376,
                120.922364,
                0.259666667,
            ),
            arc(
                2464921.5,
                2.431988060,
                0.645019758,
                3.484260,
                270.680856,
                133.935453,
                117.833202,
                0.259897519,
            ),
            arc(
                2466271.5,
                2.432992460,
                0.644663094,
                3.482889,
                270.682077,
                133.890184,
                120.529468,
                0.259713762,
            ),
            arc(
                2466491.5,
                2.434398388,
                0.644663878,
                3.482535,
                270.657876,
                133.953669,
                124.365790,
                0.259454456,
            ),
            arc(
                2466631.5,
                2.443154492,
                0.640661634,
                3.524018,
                269.592587,
                135.250059,
                148.843780,
                0.257832145,
            ),
            arc(
                2467951.5,
                2.439014909,
                0.640530369,
                3.538563,
                269.544362,
                135.354495,
                132.893960,
                0.258802240,
            ),
            arc(
                2470231.5,
                2.439968906,
                0.639805235,
                3.537687,
                269.555096,
                135.322255,
                136.736836,
                0.258597144,
            ),
            arc(
                2470521.5,
                2.444480961,
                0.640154511,
                3.533997,
                269.402154,
                135.581504,
                152.408717,
                0.257770072,
            ),
            arc(
                2471221.5,
                2.444047737,
                0.640072933,
                3.643341,
                267.662559,
                137.640896,
                149.789387,
                0.257847646,
            ),
            arc(
                2471901.5,
                2.439531453,
                0.639757748,
                3.646949,
                267.656966,
                137.732885,
                133.430505,
                0.258650543,
            ),
            arc(
                2472891.5,
                2.438050890,
                0.640506511,
                3.651473,
                267.615216,
                137.797107,
                127.937458,
                0.258908924,
            ),
            arc(
                2473401.5,
                2.439213072,
                0.640008973,
                3.651210,
                267.615370,
                137.840850,
                132.466595,
                0.258702541,
            ),
            arc(
                2474661.5,
                2.442366156,
                0.640153936,
                3.647285,
                267.443972,
                138.028083,
                146.462610,
                0.258097263,
            ),
            arc(
                2475471.5,
                2.434781719,
                0.644075160,
                3.728532,
                266.330818,
                139.327929,
                114.851258,
                0.259397841,
            ),
            arc(
                2476061.5,
                2.432445299,
                0.643993308,
                3.730391,
                266.343540,
                139.369987,
                105.157536,
                0.259792710,
            ),
            arc(
                2476721.5,
                2.430906030,
                0.644751534,
                3.733935,
                266.296381,
                139.405876,
                98.300189,
                0.260065850,
            ),
            arc(
                2477491.5,
                2.432662693,
                0.644320034,
                3.734329,
                266.269104,
                139.484447,
                106.910849,
                0.259734526,
            ),
        ],
    ), // q 0.853 AU, 22 arc(s), radial 0.185%  perihelion 2028-11-19, 2032-11-21, 2036-09-06, 2040-06-22, …
    small(
        "Apophis",
        "99942 Apophis",
        Class::Asteroid,
        0.000000185,
        Tail::None,
        0.660,
        concat!(
            "Passes inside the geostationary belt — the closest approach by anything this size in ",
            "recorded history, and naked-eye visible from Europe and Africa. Passes 38 011 km ",
            "from the Earth on 13 Apr 2029"
        ),
        &[
            arc(
                2461041.5,
                0.922339324,
                0.191178253,
                3.341028,
                203.893951,
                126.680536,
                231.911381,
                1.112673551,
            ),
            arc(
                2461551.5,
                0.922307433,
                0.191166517,
                3.341270,
                203.888074,
                126.699426,
                231.289617,
                1.112734646,
            ),
            arc(
                2461681.5,
                0.922363822,
                0.191197257,
                3.341095,
                203.885519,
                126.709192,
                232.247474,
                1.112639735,
            ),
            arc(
                2461881.5,
                0.922434647,
                0.191117249,
                3.341305,
                203.878336,
                126.724975,
                233.637891,
                1.112503690,
            ),
            arc(
                2461951.5,
                0.922428397,
                0.191118948,
                3.341320,
                203.878398,
                126.727048,
                233.497566,
                1.112516995,
            ),
            arc(
                2461981.5,
                0.922454788,
                0.191137281,
                3.341311,
                203.877614,
                126.723132,
                234.058427,
                1.112463724,
            ),
            arc(
                2462011.5,
                0.922499591,
                0.191176249,
                3.341363,
                203.876218,
                126.723143,
                235.028852,
                1.112371116,
            ),
            arc(
                2462031.5,
                0.922471371,
                0.191155163,
                3.341444,
                203.875608,
                126.719318,
                234.432096,
                1.112428408,
            ),
            arc(
                2462061.5,
                0.922429004,
                0.191136000,
                3.341468,
                203.875716,
                126.708322,
                233.520983,
                1.112516103,
            ),
            arc(
                2462071.5,
                0.922420419,
                0.191135821,
                3.341471,
                203.875746,
                126.705483,
                233.361481,
                1.112531503,
            ),
            arc(
                2462081.5,
                0.922411648,
                0.191136746,
                3.341471,
                203.875735,
                126.702619,
                233.192929,
                1.112547761,
            ),
            arc(
                2462091.5,
                0.922403997,
                0.191138750,
                3.341468,
                203.875667,
                126.700188,
                233.047944,
                1.112561740,
            ),
            arc(
                2462101.5,
                0.922396292,
                0.191141873,
                3.341464,
                203.875523,
                126.697866,
                232.896536,
                1.112576316,
            ),
            arc(
                2462111.5,
                0.922388996,
                0.191146036,
                3.341460,
                203.875278,
                126.695876,
                232.750065,
                1.112590392,
            ),
            arc(
                2462121.5,
                0.922383032,
                0.191150371,
                3.341457,
                203.874902,
                126.694550,
                232.633502,
                1.112601578,
            ),
            arc(
                2462131.5,
                0.922355497,
                0.191179889,
                3.341492,
                203.871524,
                126.692628,
                232.026672,
                1.112659344,
            ),
            arc(
                2462191.5,
                0.922337546,
                0.191205717,
                3.341912,
                203.864457,
                126.697274,
                231.831426,
                1.112678074,
            ),
            arc(
                2462201.5,
                0.922285151,
                0.191272814,
                3.342318,
                203.859934,
                126.706585,
                230.553990,
                1.112797648,
            ),
            arc(
                2462211.5,
                0.922262212,
                0.191318454,
                3.343197,
                203.852917,
                126.710806,
                230.285721,
                1.112823458,
            ),
            arc(
                2462221.5,
                0.922189018,
                0.191472444,
                3.345722,
                203.839964,
                126.721412,
                229.022531,
                1.112943307,
            ),
            arc(
                2462231.5,
                1.103028332,
                0.189065276,
                2.221528,
                203.530089,
                71.477883,
                208.683837,
                0.850803371,
            ),
            arc(
                2467341.5,
                1.102692813,
                0.189037822,
                2.226045,
                203.306399,
                71.531075,
                201.712793,
                0.851235292,
            ),
            arc(
                2469731.5,
                1.103753127,
                0.189331964,
                2.225394,
                203.121428,
                71.302190,
                227.345435,
                0.849883863,
            ),
            arc(
                2475871.5,
                1.104711935,
                0.189560392,
                2.230032,
                202.911578,
                71.649589,
                252.972774,
                0.848826153,
            ),
        ],
    ), // q 0.746 AU, 24 arc(s), radial 0.318%  perihelion 2026-01-02, 2026-11-21, 2027-10-11, 2028-08-30, …
    small(
        "Bennu",
        "101955 Bennu",
        Class::Asteroid,
        0.000000245,
        Tail::None,
        0.075,
        "OSIRIS-REx returned 122 g of it in September 2023",
        &[
            arc(
                2461041.5,
                1.125935525,
                0.203676289,
                6.032762,
                1.962553,
                66.396731,
                27.043878,
                0.824962661,
            ),
            arc(
                2462861.5,
                1.126185506,
                0.203761010,
                6.032177,
                1.927759,
                66.463535,
                30.127861,
                0.824687746,
            ),
            arc(
                2464831.5,
                1.126159504,
                0.203793029,
                6.031675,
                1.915534,
                66.482986,
                29.498731,
                0.824734350,
            ),
            arc(
                2464901.5,
                1.126736097,
                0.203968235,
                6.035873,
                1.846574,
                66.607393,
                39.091860,
                0.824019846,
            ),
            arc(
                2466331.5,
                1.126837323,
                0.203887085,
                6.039846,
                1.718901,
                66.675627,
                40.070946,
                0.823957651,
            ),
            arc(
                2469381.5,
                1.126914002,
                0.203830447,
                6.040166,
                1.572593,
                66.629006,
                41.306020,
                0.823897676,
            ),
            arc(
                2471411.5,
                1.128303921,
                0.204285785,
                6.036910,
                1.508557,
                66.333526,
                75.598485,
                0.822198314,
            ),
            arc(
                2471731.5,
                1.128309668,
                0.204369809,
                6.032241,
                1.471339,
                66.295142,
                72.061503,
                0.822377284,
            ),
            arc(
                2471811.5,
                1.128408704,
                0.204266192,
                6.031995,
                1.465855,
                66.303277,
                74.860116,
                0.822239017,
            ),
            arc(
                2471911.5,
                1.128320678,
                0.204285280,
                6.032209,
                1.462527,
                66.327806,
                72.583153,
                0.822349711,
            ),
            arc(
                2472051.5,
                1.128316208,
                0.204303252,
                6.032236,
                1.461242,
                66.319528,
                72.365896,
                0.822360579,
            ),
            arc(
                2472361.5,
                1.128306148,
                0.204228420,
                6.032452,
                1.457445,
                66.320450,
                72.355828,
                0.822360709,
            ),
            arc(
                2472861.5,
                1.128327877,
                0.204286244,
                6.032323,
                1.432546,
                66.340434,
                73.221513,
                0.822320537,
            ),
            arc(
                2473741.5,
                1.114536276,
                0.199077420,
                6.206424,
                1.301631,
                69.103135,
                90.670455,
                0.837645409,
            ),
            arc(
                2473751.5,
                1.114411861,
                0.198984907,
                6.205038,
                1.295004,
                69.116280,
                87.486866,
                0.837788308,
            ),
            arc(
                2473761.5,
                1.114392825,
                0.198968278,
                6.204317,
                1.289347,
                69.120427,
                87.357789,
                0.837794131,
            ),
            arc(
                2473781.5,
                1.114104528,
                0.198760755,
                6.203963,
                1.275080,
                69.107136,
                79.508747,
                0.838147956,
            ),
            arc(
                2474241.5,
                1.114128359,
                0.198801071,
                6.203393,
                1.258111,
                69.119716,
                80.420231,
                0.838108166,
            ),
            arc(
                2476371.5,
                1.114488733,
                0.198752052,
                6.215446,
                1.077578,
                69.325875,
                90.584046,
                0.837697970,
            ),
        ],
    ), // q 0.897 AU, 19 arc(s), radial 0.075%  perihelion 2026-03-13, 2027-05-23, 2028-08-01, 2029-10-12, …
    small(
        "Halley",
        "1P/Halley",
        Class::Comet,
        0.000005500,
        Tail::Ion,
        0.025,
        "The comet; next perihelion 28 July 2061, its first since 1986",
        &[
            arc(
                2461041.5,
                17.869969457,
                0.967219529,
                162.245759,
                58.839736,
                111.726250,
                66.505970,
                0.013050984,
            ),
            arc(
                2473531.5,
                17.906292814,
                0.966899742,
                161.978105,
                59.407419,
                112.056012,
                67.507893,
                0.013005883,
            ),
            arc(
                2474121.5,
                17.368669309,
                0.965890194,
                161.990978,
                59.573877,
                112.161769,
                53.379797,
                0.013634040,
            ),
            arc(
                2475311.5,
                17.485157731,
                0.965555484,
                161.991200,
                60.450284,
                113.219961,
                57.316144,
                0.013461235,
            ),
        ],
    ), // q 0.586 AU, 4 arc(s), radial 0.044%  perihelion 2061-07-28
    small(
        "Encke",
        "2P/Encke",
        Class::Comet,
        0.000002400,
        Tail::Ion,
        0.083,
        "Shortest period of any known comet at 3.3 years; parent of the Taurids",
        &[
            arc(
                2461041.5,
                2.218049674,
                0.847356470,
                11.347259,
                334.018554,
                187.293050,
                285.680108,
                0.298364125,
            ),
            arc(
                2462351.5,
                2.218326342,
                0.847073201,
                11.333891,
                333.998273,
                187.319096,
                286.259330,
                0.298304330,
            ),
            arc(
                2462661.5,
                2.217240851,
                0.847238591,
                11.338771,
                333.986368,
                187.304265,
                283.334045,
                0.298567208,
            ),
            arc(
                2463861.5,
                2.217724797,
                0.847418142,
                11.342467,
                333.998730,
                187.320953,
                284.804913,
                0.298447787,
            ),
            arc(
                2464331.5,
                2.219053747,
                0.846584969,
                11.307960,
                333.993746,
                187.343359,
                288.256021,
                0.298180377,
            ),
            arc(
                2465061.5,
                2.220231106,
                0.846736321,
                11.310475,
                333.970269,
                187.337704,
                292.064757,
                0.297898663,
            ),
            arc(
                2466271.5,
                2.219377120,
                0.846847384,
                11.311632,
                333.970814,
                187.364167,
                289.043118,
                0.298103930,
            ),
            arc(
                2466851.5,
                2.220111868,
                0.846436857,
                11.295280,
                333.959171,
                187.375462,
                291.568480,
                0.297939166,
            ),
            arc(
                2467511.5,
                2.220687908,
                0.846611466,
                11.302564,
                333.924641,
                187.387360,
                294.258602,
                0.297770258,
            ),
            arc(
                2468091.5,
                2.214758350,
                0.850348415,
                11.262342,
                333.804678,
                187.585723,
                275.097497,
                0.298903043,
            ),
            arc(
                2468681.5,
                2.210064515,
                0.850251629,
                11.253139,
                333.729435,
                187.727876,
                256.486009,
                0.299988666,
            ),
            arc(
                2469221.5,
                2.209411292,
                0.850602933,
                11.265881,
                333.737272,
                187.714308,
                253.956909,
                0.300133226,
            ),
            arc(
                2469891.5,
                2.210348435,
                0.850650639,
                11.264337,
                333.740246,
                187.724231,
                258.368827,
                0.299892731,
            ),
            arc(
                2471081.5,
                2.209407668,
                0.850496429,
                11.255061,
                333.729977,
                187.746286,
                253.558020,
                0.300138868,
            ),
            arc(
                2472291.5,
                2.212228340,
                0.850490221,
                11.248402,
                333.726688,
                187.772132,
                266.412987,
                0.299519136,
            ),
            arc(
                2472821.5,
                2.217233637,
                0.847680346,
                11.105275,
                333.647565,
                187.904278,
                288.207493,
                0.298489440,
            ),
            arc(
                2473511.5,
                2.217219787,
                0.847964756,
                11.108250,
                333.595446,
                187.914882,
                287.449883,
                0.298523480,
            ),
            arc(
                2474711.5,
                2.217151384,
                0.847869195,
                11.106566,
                333.604271,
                187.960052,
                287.351142,
                0.298527968,
            ),
            arc(
                2475911.5,
                2.216703338,
                0.847780474,
                11.102470,
                333.565986,
                187.961520,
                284.780733,
                0.298633470,
            ),
            arc(
                2476631.5,
                2.214271146,
                0.849219146,
                11.124859,
                333.574852,
                187.985406,
                272.001544,
                0.299142823,
            ),
            arc(
                2477111.5,
                2.213767226,
                0.849171856,
                11.123192,
                333.562616,
                187.996509,
                265.972102,
                0.299378686,
            ),
        ],
    ), // q 0.339 AU, 21 arc(s), radial 0.256%  perihelion 2027-02-10, 2030-06-01, 2033-09-18, 2037-01-07, …
    small(
        "Tuttle",
        "8P/Tuttle",
        Class::Comet,
        0.000002300,
        Tail::Ion,
        0.090,
        "Parent of the Ursids",
        &[
            arc(
                2461041.5,
                5.708366852,
                0.820457226,
                54.873991,
                270.138851,
                207.525080,
                148.325551,
                0.072265682,
            ),
            arc(
                2462951.5,
                5.720421194,
                0.820209144,
                54.773764,
                270.037852,
                207.482093,
                151.198834,
                0.072051501,
            ),
            arc(
                2464631.5,
                5.742174256,
                0.820800358,
                54.787716,
                270.015195,
                207.517513,
                156.938387,
                0.071606609,
            ),
            arc(
                2465361.5,
                5.746296992,
                0.820586172,
                54.693872,
                269.997578,
                207.566722,
                157.833151,
                0.071536813,
            ),
            arc(
                2466921.5,
                5.750050410,
                0.820001583,
                54.607214,
                269.979803,
                207.511025,
                158.353191,
                0.071530287,
            ),
            arc(
                2469471.5,
                5.767804607,
                0.820336434,
                54.533937,
                270.048278,
                207.443597,
                163.886610,
                0.071221392,
            ),
            arc(
                2473641.5,
                5.762273665,
                0.820710808,
                54.605986,
                270.158565,
                207.447988,
                163.005607,
                0.071259165,
            ),
            arc(
                2475041.5,
                5.743943737,
                0.820881397,
                54.592899,
                270.166088,
                207.310506,
                151.547180,
                0.071753789,
            ),
            arc(
                2477791.5,
                5.743183993,
                0.821080968,
                54.748762,
                270.287375,
                207.247716,
                156.626471,
                0.071558936,
            ),
            arc(
                2478621.5,
                5.738504447,
                0.821447011,
                54.725106,
                270.250373,
                207.324097,
                153.290406,
                0.071679677,
            ),
            arc(
                2479011.5,
                5.735485384,
                0.821455267,
                54.713664,
                270.220452,
                207.363018,
                151.299315,
                0.071751038,
            ),
        ],
    ), // q 1.025 AU, 11 arc(s), radial 0.089%  perihelion 2035-04-18, 2049-01-21, 2062-11-23
    small(
        "Tempel 1",
        "9P/Tempel 1",
        Class::Comet,
        0.000003000,
        Tail::Ion,
        0.074,
        "Deep Impact fired a projectile into it in 2005",
        &[
            arc(
                2461041.5,
                3.304481369,
                0.463914939,
                10.450238,
                66.590234,
                184.815165,
                114.967078,
                0.164089861,
            ),
            arc(
                2463231.5,
                3.304195759,
                0.463699010,
                10.450298,
                66.534411,
                185.002414,
                114.713463,
                0.164085294,
            ),
            arc(
                2464011.5,
                3.306034912,
                0.463891003,
                10.449162,
                66.501692,
                185.154318,
                116.353544,
                0.163950542,
            ),
            arc(
                2464571.5,
                3.348388600,
                0.459027301,
                10.519638,
                66.181027,
                187.006631,
                165.039790,
                0.160072642,
            ),
            arc(
                2464771.5,
                3.370824228,
                0.451303897,
                10.544724,
                66.114605,
                187.691476,
                179.806485,
                0.158918339,
            ),
            arc(
                2465051.5,
                3.403728024,
                0.437778161,
                10.333973,
                65.932710,
                187.777627,
                207.890583,
                0.156873089,
            ),
            arc(
                2465911.5,
                3.413681108,
                0.435287936,
                10.290342,
                65.476232,
                187.800884,
                217.542668,
                0.156212775,
            ),
            arc(
                2466121.5,
                3.407249363,
                0.434463591,
                10.292848,
                65.450856,
                187.944641,
                210.088572,
                0.156716005,
            ),
            arc(
                2466541.5,
                3.411072636,
                0.434399570,
                10.288430,
                65.430722,
                188.108975,
                214.603621,
                0.156407486,
            ),
            arc(
                2469331.5,
                3.426855035,
                0.428464060,
                10.250214,
                65.326001,
                188.197919,
                235.248536,
                0.155291887,
            ),
            arc(
                2472041.5,
                3.431207904,
                0.427751864,
                10.249975,
                65.254820,
                188.325218,
                240.155941,
                0.155043867,
            ),
            arc(
                2472391.5,
                3.432784608,
                0.427297746,
                10.244919,
                65.225724,
                188.297857,
                242.174176,
                0.154949912,
            ),
            arc(
                2473681.5,
                3.430733138,
                0.426031982,
                10.240467,
                65.219820,
                188.227000,
                238.575113,
                0.155124234,
            ),
            arc(
                2475621.5,
                3.436032671,
                0.425333781,
                10.234487,
                65.199248,
                188.240376,
                247.687569,
                0.154746764,
            ),
            arc(
                2477741.5,
                3.437969369,
                0.424259834,
                10.229646,
                65.159658,
                188.017899,
                251.212016,
                0.154613876,
            ),
        ],
    ), // q 1.771 AU, 15 arc(s), radial 0.351%  perihelion 2028-02-12, 2034-02-16, 2040-05-09, 2046-08-28, …
    small(
        "Borrelly",
        "19P/Borrelly",
        Class::Comet,
        0.000002400,
        Tail::Ion,
        0.090,
        "Deep Space 1 photographed its nucleus in 2001",
        &[
            arc(
                2461041.5,
                3.609658489,
                0.637053723,
                29.279914,
                74.187367,
                352.030955,
                280.205069,
                0.143751229,
            ),
            arc(
                2463121.5,
                3.620896559,
                0.633870432,
                29.301784,
                74.127327,
                352.263466,
                287.814160,
                0.143076888,
            ),
            arc(
                2464701.5,
                3.626305546,
                0.633293637,
                29.282818,
                74.153999,
                352.301632,
                292.301605,
                0.142736685,
            ),
            arc(
                2467131.5,
                3.627967368,
                0.632833173,
                29.265168,
                74.118795,
                352.176647,
                294.021384,
                0.142626832,
            ),
            arc(
                2468921.5,
                3.622465249,
                0.633959117,
                29.307909,
                74.113030,
                352.269929,
                288.166102,
                0.142957223,
            ),
            arc(
                2469721.5,
                3.617707403,
                0.633799716,
                29.307471,
                74.097782,
                352.212383,
                282.354174,
                0.143277980,
            ),
            arc(
                2471281.5,
                3.605574114,
                0.637594009,
                29.356908,
                74.088711,
                352.425654,
                268.029636,
                0.143996193,
            ),
            arc(
                2472141.5,
                3.603615471,
                0.638168819,
                29.370483,
                74.019246,
                352.551945,
                267.141803,
                0.144038487,
            ),
            arc(
                2474901.5,
                3.603184199,
                0.638251142,
                29.388469,
                73.905286,
                352.741020,
                266.368657,
                0.144071279,
            ),
            arc(
                2475361.5,
                3.589871143,
                0.641393529,
                29.810814,
                73.374336,
                352.871810,
                234.311801,
                0.145431696,
            ),
            arc(
                2475761.5,
                3.577166400,
                0.645466208,
                30.706172,
                72.874432,
                352.969820,
                165.523195,
                0.148310495,
            ),
            arc(
                2475911.5,
                3.606220811,
                0.632233353,
                30.700655,
                72.871602,
                352.994522,
                246.380416,
                0.144990137,
            ),
            arc(
                2476051.5,
                3.619684795,
                0.626391471,
                30.591388,
                72.876392,
                352.873358,
                281.439983,
                0.143565152,
            ),
            arc(
                2476201.5,
                3.626236272,
                0.623908593,
                30.510214,
                72.861508,
                352.774211,
                296.788088,
                0.142947981,
            ),
            arc(
                2476371.5,
                3.636172961,
                0.621844408,
                30.402811,
                72.787232,
                352.593823,
                316.784222,
                0.142157667,
            ),
            arc(
                2477081.5,
                3.643029062,
                0.622007484,
                30.398824,
                72.721608,
                352.689531,
                327.777651,
                0.141728078,
            ),
        ],
    ), // q 1.310 AU, 16 arc(s), radial 0.217%  perihelion 2028-12-11, 2035-10-29, 2042-09-24, 2049-08-21, …
    small(
        "Giacobini-Zinner",
        "21P/Giacobini-Zinner",
        Class::Comet,
        0.000001000,
        Tail::Ion,
        0.091,
        "Parent of the Draconids; first comet ever visited, by ICE in 1985",
        &[
            arc(
                2461041.5,
                3.490096321,
                0.711103847,
                32.057816,
                195.298656,
                172.926189,
                46.868498,
                0.151182554,
            ),
            arc(
                2461161.5,
                3.488517892,
                0.711227876,
                32.069552,
                195.276987,
                172.911009,
                45.664502,
                0.151311747,
            ),
            arc(
                2461301.5,
                3.485594673,
                0.711842348,
                32.120102,
                195.217528,
                172.891160,
                43.066313,
                0.151585012,
            ),
            arc(
                2461711.5,
                3.479517698,
                0.714390671,
                32.312600,
                195.113226,
                172.886497,
                35.642633,
                0.152328328,
            ),
            arc(
                2461941.5,
                3.465237470,
                0.721432837,
                32.655084,
                195.029044,
                172.948909,
                11.198353,
                0.154681367,
            ),
            arc(
                2462051.5,
                3.453636744,
                0.727580519,
                33.198953,
                194.974395,
                172.996909,
                350.930825,
                0.156635162,
            ),
            arc(
                2462141.5,
                3.475203984,
                0.724278524,
                33.456009,
                194.964573,
                172.234989,
                9.238860,
                0.155179302,
            ),
            arc(
                2462251.5,
                3.558533747,
                0.699975521,
                31.784788,
                194.612368,
                170.691530,
                103.371388,
                0.146722325,
            ),
            arc(
                2462871.5,
                3.557183273,
                0.699231488,
                31.668511,
                194.396678,
                170.896128,
                101.555822,
                0.146878363,
            ),
            arc(
                2465731.5,
                3.558912035,
                0.698918121,
                31.664771,
                194.340975,
                170.965498,
                102.818254,
                0.146788439,
            ),
            arc(
                2465991.5,
                3.563077780,
                0.698365241,
                31.725200,
                194.221801,
                171.166803,
                106.564163,
                0.146523166,
            ),
            arc(
                2466231.5,
                3.575140145,
                0.692549053,
                31.785511,
                194.107178,
                171.365136,
                117.710344,
                0.145796478,
            ),
            arc(
                2468241.5,
                3.583764830,
                0.691770880,
                31.749233,
                194.091222,
                171.409420,
                126.694557,
                0.145258148,
            ),
            arc(
                2470551.5,
                3.583506481,
                0.691982478,
                31.747103,
                194.048574,
                171.324918,
                125.627778,
                0.145313927,
            ),
            arc(
                2472991.5,
                3.579162584,
                0.692387758,
                31.772710,
                194.102481,
                171.375983,
                119.949625,
                0.145577898,
            ),
            arc(
                2476441.5,
                3.571026389,
                0.693625899,
                31.861994,
                194.027134,
                171.549403,
                108.210998,
                0.146030366,
            ),
        ],
    ), // q 1.008 AU, 16 arc(s), radial 0.291%  perihelion 2031-08-30, 2038-05-16, 2045-02-10, 2051-11-24, …
    small(
        "Wirtanen",
        "46P/Wirtanen",
        Class::Comet,
        0.000000600,
        Tail::Ion,
        0.128,
        "Hyperactive for its size, and passes close enough to be a naked-eye object",
        &[
            arc(
                2461041.5,
                3.094635970,
                0.657739855,
                11.737587,
                82.169371,
                356.346439,
                188.267304,
                0.181023912,
            ),
            arc(
                2462601.5,
                3.103239999,
                0.654922699,
                11.722102,
                82.163470,
                356.585682,
                196.283031,
                0.180307518,
            ),
            arc(
                2464401.5,
                3.109648275,
                0.654222712,
                11.709788,
                82.107924,
                356.475609,
                203.684408,
                0.179732023,
            ),
            arc(
                2465281.5,
                3.112270155,
                0.653360277,
                11.699498,
                82.101490,
                356.507942,
                207.009874,
                0.179485836,
            ),
            arc(
                2466491.5,
                3.129354393,
                0.653979197,
                11.714678,
                81.857097,
                357.090806,
                233.063198,
                0.177735848,
            ),
            arc(
                2467261.5,
                3.189926429,
                0.635486722,
                12.374089,
                80.818431,
                359.681830,
                326.693412,
                0.171549719,
            ),
            arc(
                2467321.5,
                3.192579041,
                0.633762231,
                12.342360,
                80.836731,
                359.634162,
                308.642169,
                0.172706073,
            ),
            arc(
                2467441.5,
                3.219670523,
                0.621544075,
                12.090558,
                80.707092,
                359.463784,
                344.285423,
                0.170519114,
            ),
            arc(
                2468501.5,
                3.224179482,
                0.620672919,
                12.071611,
                80.509753,
                359.622256,
                349.246151,
                0.170227798,
            ),
            arc(
                2471381.5,
                3.254103015,
                0.620127834,
                12.719079,
                79.343598,
                1.629899,
                92.587730,
                0.164883164,
            ),
            arc(
                2471501.5,
                3.239777121,
                0.635502735,
                14.167334,
                78.334869,
                3.123098,
                73.869477,
                0.165674407,
            ),
            arc(
                2471561.5,
                3.313953665,
                0.629482120,
                16.984317,
                77.372651,
                7.723218,
                173.814528,
                0.160107067,
            ),
            arc(
                2471601.5,
                3.580132401,
                0.545797592,
                16.720741,
                77.380201,
                14.416275,
                152.719551,
                0.142600398,
            ),
            arc(
                2471651.5,
                3.607698898,
                0.530162342,
                15.309105,
                77.380547,
                14.355006,
                71.483175,
                0.146709697,
            ),
            arc(
                2471721.5,
                3.657741001,
                0.509985733,
                14.664875,
                77.210510,
                15.104429,
                140.530623,
                0.143268960,
            ),
            arc(
                2471811.5,
                3.699639256,
                0.493374847,
                14.355494,
                77.013645,
                15.671751,
                214.081631,
                0.139618275,
            ),
            arc(
                2471921.5,
                3.728224605,
                0.481959762,
                14.127935,
                76.743501,
                15.912006,
                260.752436,
                0.137330914,
            ),
            arc(
                2472201.5,
                3.743243645,
                0.476637858,
                14.018223,
                76.482324,
                15.908056,
                283.037443,
                0.136263328,
            ),
            arc(
                2472421.5,
                3.752797960,
                0.474294449,
                13.974339,
                76.296172,
                15.810465,
                296.343866,
                0.135639373,
            ),
            arc(
                2472691.5,
                3.760185906,
                0.473449062,
                13.960930,
                76.177787,
                15.683506,
                306.357738,
                0.135176216,
            ),
            arc(
                2472781.5,
                3.767663994,
                0.473289069,
                13.956913,
                76.084379,
                15.551324,
                316.250974,
                0.134721191,
            ),
            arc(
                2472981.5,
                3.763150112,
                0.472399505,
                13.965511,
                76.005915,
                15.661573,
                309.539333,
                0.135029637,
            ),
            arc(
                2474561.5,
                3.771337930,
                0.470608259,
                13.945432,
                75.975288,
                15.727195,
                320.531728,
                0.134548768,
            ),
            arc(
                2476561.5,
                3.761809591,
                0.472299011,
                13.960970,
                75.952236,
                15.641421,
                306.185756,
                0.135120840,
            ),
        ],
    ), // q 1.059 AU, 24 arc(s), radial 0.302%  perihelion 2029-10-27, 2035-04-15, 2040-10-09, 2046-06-30, …
    small(
        "Tempel-Tuttle",
        "55P/Tempel-Tuttle",
        Class::Comet,
        0.000001800,
        Tail::Ion,
        0.071,
        "Parent of the Leonids, whose storms follow its 33-year return",
        &[
            arc(
                2461041.5,
                10.316838765,
                0.906443437,
                162.561599,
                235.515220,
                172.818136,
                18.642297,
                0.029780340,
            ),
            arc(
                2463161.5,
                10.463816092,
                0.907753626,
                162.470325,
                236.245541,
                173.500559,
                26.217234,
                0.029119738,
            ),
            arc(
                2466051.5,
                10.460036648,
                0.908283689,
                162.368679,
                236.410527,
                173.591064,
                25.864543,
                0.029152763,
            ),
            arc(
                2472711.5,
                10.461124292,
                0.907502055,
                162.504087,
                236.522056,
                173.634563,
                26.421801,
                0.029126542,
            ),
            arc(
                2475521.5,
                10.294151327,
                0.906048428,
                162.509489,
                236.827866,
                173.805658,
                7.918046,
                0.029903356,
            ),
            arc(
                2477861.5,
                10.357522917,
                0.905173897,
                162.473498,
                236.871410,
                174.117097,
                17.123517,
                0.029522377,
            ),
        ],
    ), // q 0.965 AU, 6 arc(s), radial 0.098%  perihelion 2031-05-21, 2065-03-13
    small(
        "Churyumov-Gerasimenko",
        "67P/Churyumov-Gerasimenko",
        Class::Comet,
        0.000001650,
        Tail::Ion,
        0.087,
        "Rosetta orbited it for two years and landed Philae on it in 2014",
        &[
            arc(
                2461041.5,
                3.459689668,
                0.649374900,
                3.865617,
                36.280224,
                22.228512,
                218.590212,
                0.153146393,
            ),
            arc(
                2461761.5,
                3.461567530,
                0.649508043,
                3.865602,
                36.275237,
                22.257557,
                219.633293,
                0.153044892,
            ),
            arc(
                2462471.5,
                3.500874816,
                0.645404687,
                3.820231,
                36.361837,
                23.099001,
                252.388552,
                0.149909168,
            ),
            arc(
                2462591.5,
                3.514808126,
                0.637393249,
                3.694453,
                36.118059,
                23.446414,
                256.071917,
                0.149630181,
            ),
            arc(
                2464151.5,
                3.523119853,
                0.636728555,
                3.691488,
                36.002906,
                23.346211,
                263.445648,
                0.149048518,
            ),
            arc(
                2465091.5,
                3.528573181,
                0.635293395,
                3.684850,
                35.978026,
                23.417279,
                268.440758,
                0.148678469,
            ),
            arc(
                2466751.5,
                3.529042498,
                0.635121390,
                3.679384,
                35.984072,
                23.386139,
                268.319795,
                0.148686766,
            ),
            arc(
                2469041.5,
                3.529054060,
                0.635200053,
                3.673975,
                35.950545,
                23.520108,
                268.069905,
                0.148701078,
            ),
            arc(
                2471441.5,
                3.531594282,
                0.634516952,
                3.673107,
                35.925314,
                23.429037,
                272.118169,
                0.148496789,
            ),
            arc(
                2472841.5,
                3.525702472,
                0.636241050,
                3.674806,
                35.914381,
                23.462821,
                263.827559,
                0.148884307,
            ),
            arc(
                2474021.5,
                3.519840115,
                0.636128086,
                3.675519,
                35.884141,
                23.418742,
                254.592425,
                0.149296337,
            ),
            arc(
                2475501.5,
                3.509869526,
                0.639234223,
                3.663270,
                35.655592,
                23.810925,
                239.522933,
                0.149920432,
            ),
            arc(
                2476351.5,
                3.508460966,
                0.639921624,
                3.663198,
                35.587658,
                23.867075,
                239.089103,
                0.149937588,
            ),
            arc(
                2478701.5,
                3.507655345,
                0.640789316,
                3.661498,
                35.513225,
                24.052736,
                237.331174,
                0.150003055,
            ),
        ],
    ), // q 1.213 AU, 14 arc(s), radial 0.200%  perihelion 2028-04-09, 2034-11-02, 2041-06-18, 2048-02-03, …
    small(
        "Schwassmann-Wachmann 3",
        "73P/Schwassmann-Wachmann 3",
        Class::Comet,
        0.000000550,
        Tail::Ion,
        0.156,
        "Broke into dozens of fragments in 1995 and is still coming apart",
        &[
            arc(
                2461041.5,
                3.063076815,
                0.700155159,
                6.204983,
                52.110850,
                214.739507,
                281.961086,
                0.183795175,
            ),
            arc(
                2462041.5,
                3.059802290,
                0.700611048,
                6.206609,
                52.026926,
                214.813844,
                278.497403,
                0.184124660,
            ),
            arc(
                2463721.5,
                3.053565636,
                0.700530031,
                6.206953,
                51.973489,
                214.852842,
                270.993871,
                0.184741203,
            ),
            arc(
                2464991.5,
                3.044419458,
                0.704529710,
                6.172779,
                51.523843,
                215.428937,
                260.632210,
                0.185515814,
            ),
            arc(
                2465581.5,
                3.038254123,
                0.704828648,
                6.176365,
                51.427057,
                215.562834,
                252.449788,
                0.186096199,
            ),
            arc(
                2467561.5,
                3.036259500,
                0.705288450,
                6.173170,
                51.410085,
                215.637685,
                249.237762,
                0.186299440,
            ),
            arc(
                2471481.5,
                3.039835617,
                0.705139194,
                6.167815,
                51.394437,
                215.706670,
                256.117246,
                0.185955430,
            ),
            arc(
                2473431.5,
                3.043705643,
                0.704701485,
                6.161089,
                51.369136,
                215.697044,
                262.897478,
                0.185645309,
            ),
            arc(
                2474261.5,
                3.048280499,
                0.702827734,
                6.133094,
                51.230948,
                215.874753,
                273.070574,
                0.185197106,
            ),
            arc(
                2475361.5,
                3.052338017,
                0.702562963,
                6.126361,
                51.166689,
                215.947655,
                282.109978,
                0.184818084,
            ),
            arc(
                2477841.5,
                3.110304909,
                0.701540949,
                5.643465,
                49.783369,
                218.577019,
                147.859462,
                0.176088382,
            ),
            arc(
                2477951.5,
                3.134029422,
                0.698855381,
                4.134442,
                40.945487,
                228.039279,
                198.639461,
                0.174095602,
            ),
            arc(
                2478021.5,
                3.126280468,
                0.699474968,
                3.364899,
                31.388892,
                237.314851,
                89.376938,
                0.178255108,
            ),
            arc(
                2478081.5,
                3.132405555,
                0.695848219,
                3.149256,
                27.009435,
                241.750822,
                81.941139,
                0.178539865,
            ),
            arc(
                2478141.5,
                3.140579823,
                0.691663901,
                3.069702,
                24.895529,
                243.962490,
                102.672596,
                0.177757313,
            ),
            arc(
                2478211.5,
                3.147318345,
                0.688122565,
                3.033002,
                23.693257,
                245.214924,
                123.185902,
                0.176986134,
            ),
            arc(
                2478301.5,
                3.156588975,
                0.683210023,
                3.007730,
                22.585144,
                246.296776,
                152.139345,
                0.175906814,
            ),
            arc(
                2478681.5,
                3.162796473,
                0.680724036,
                3.004984,
                22.220061,
                246.519937,
                170.244146,
                0.175243816,
            ),
            arc(
                2478901.5,
                3.165625849,
                0.680120850,
                3.005389,
                22.163251,
                246.495800,
                177.675272,
                0.174974699,
            ),
            arc(
                2479031.5,
                3.167276053,
                0.679971079,
                3.005660,
                22.149813,
                246.466282,
                181.949881,
                0.174820518,
            ),
            arc(
                2479111.5,
                3.167436894,
                0.679925938,
                3.005770,
                22.146651,
                246.461493,
                182.066995,
                0.174816319,
            ),
            arc(
                2479171.5,
                3.167077467,
                0.679891764,
                3.005822,
                22.145703,
                246.465368,
                180.990558,
                0.174855070,
            ),
            arc(
                2479221.5,
                3.166439523,
                0.679842783,
                3.005840,
                22.145521,
                246.472095,
                179.328342,
                0.174914874,
            ),
            arc(
                2479251.5,
                3.165691289,
                0.679778777,
                3.005834,
                22.145526,
                246.478277,
                177.470978,
                0.174981689,
            ),
        ],
    ), // q 0.918 AU, 24 arc(s), radial 0.287%  perihelion 2027-12-23, 2033-05-01, 2038-08-28, 2043-12-15, …
    small(
        "Wild 2",
        "81P/Wild 2",
        Class::Comet,
        0.000001980,
        Tail::Ion,
        0.117,
        "Stardust flew through its coma and returned the dust in 2006",
        &[
            arc(
                2461041.5,
                3.444914818,
                0.538150772,
                3.238731,
                136.100250,
                41.621735,
                145.962962,
                0.154210132,
            ),
            arc(
                2464451.5,
                3.444293370,
                0.539668901,
                3.239909,
                136.075023,
                41.569281,
                146.930776,
                0.154148775,
            ),
            arc(
                2466671.5,
                3.439411663,
                0.540589150,
                3.240418,
                136.072949,
                41.676405,
                141.324245,
                0.154520355,
            ),
            arc(
                2467071.5,
                3.433451560,
                0.541282754,
                3.243469,
                136.034384,
                41.471055,
                134.461946,
                0.154966268,
            ),
            arc(
                2468451.5,
                3.401182618,
                0.550362424,
                3.233800,
                135.378468,
                42.790045,
                96.785164,
                0.157154618,
            ),
            arc(
                2469261.5,
                3.395353468,
                0.550823859,
                3.238115,
                135.305799,
                43.009529,
                90.548098,
                0.157502940,
            ),
            arc(
                2471861.5,
                3.382534137,
                0.553878290,
                3.249747,
                135.248179,
                42.722902,
                70.896952,
                0.158475989,
            ),
            arc(
                2472741.5,
                3.291257233,
                0.596194054,
                3.097856,
                131.991225,
                46.042192,
                221.761604,
                0.168368757,
            ),
            arc(
                2472921.5,
                3.231440160,
                0.642395229,
                4.749587,
                45.522311,
                132.027137,
                126.591984,
                0.173065327,
            ),
            arc(
                2472981.5,
                3.483755745,
                0.555070337,
                11.395619,
                26.547596,
                144.625139,
                298.719049,
                0.148545193,
            ),
            arc(
                2473021.5,
                3.555376672,
                0.522193012,
                11.815357,
                26.178493,
                143.346484,
                33.671144,
                0.144108879,
            ),
            arc(
                2473051.5,
                3.582149819,
                0.510312250,
                11.745713,
                26.221997,
                142.626066,
                68.884031,
                0.142468370,
            ),
            arc(
                2473081.5,
                3.597177281,
                0.503950844,
                11.642854,
                26.268177,
                142.162629,
                86.657364,
                0.141643641,
            ),
            arc(
                2473121.5,
                3.608073733,
                0.500513454,
                11.549596,
                26.287490,
                141.806129,
                99.564180,
                0.141053444,
            ),
            arc(
                2473161.5,
                3.585297180,
                0.502537316,
                11.474813,
                26.281806,
                142.529047,
                47.695775,
                0.143407439,
            ),
            arc(
                2473211.5,
                3.566635990,
                0.503767462,
                11.409027,
                26.251555,
                143.135196,
                6.319375,
                0.145281304,
            ),
            arc(
                2473281.5,
                3.552056706,
                0.504291694,
                11.360413,
                26.200875,
                143.618154,
                335.777415,
                0.146660696,
            ),
            arc(
                2473361.5,
                3.539576718,
                0.504335547,
                11.330446,
                26.143884,
                144.022587,
                311.708176,
                0.147744532,
            ),
            arc(
                2473461.5,
                3.528190433,
                0.503918082,
                11.315852,
                26.093384,
                144.355332,
                291.643139,
                0.148645546,
            ),
            arc(
                2473591.5,
                3.518467124,
                0.502996505,
                11.312567,
                26.061886,
                144.554576,
                276.078098,
                0.149343278,
            ),
            arc(
                2473941.5,
                3.522537993,
                0.503052172,
                11.312288,
                26.014644,
                144.694871,
                282.723985,
                0.149044632,
            ),
            arc(
                2476421.5,
                3.535948536,
                0.500876129,
                11.306276,
                25.598338,
                145.653864,
                300.593339,
                0.148319859,
            ),
            arc(
                2477861.5,
                3.554072852,
                0.495920424,
                11.324914,
                25.442362,
                145.569181,
                333.331807,
                0.147079559,
            ),
            arc(
                2478831.5,
                3.554031622,
                0.495761069,
                11.325177,
                25.407390,
                145.613522,
                332.794035,
                0.147099451,
            ),
        ],
    ), // q 1.591 AU, 24 arc(s), radial 0.327%  perihelion 2029-05-14, 2035-10-03, 2042-02-23, 2048-06-27, …
    small(
        "Machholz 1",
        "96P/Machholz 1",
        Class::Comet,
        0.000003200,
        Tail::Ion,
        0.076,
        "Passes 0.12 AU from the Sun — closer than any other short-period comet",
        &[
            arc(
                2461041.5,
                3.029896126,
                0.961703179,
                57.485680,
                93.916084,
                14.768468,
                224.191366,
                0.186879429,
            ),
            arc(
                2462741.5,
                3.027949400,
                0.961913527,
                57.571125,
                93.937829,
                14.740197,
                219.846140,
                0.187285286,
            ),
            arc(
                2463041.5,
                3.025998696,
                0.963468745,
                57.064603,
                93.695754,
                14.956728,
                220.115738,
                0.187272551,
            ),
            arc(
                2463821.5,
                3.025177649,
                0.963511103,
                57.007469,
                93.622029,
                15.013251,
                219.483427,
                0.187324082,
            ),
            arc(
                2464631.5,
                3.024650327,
                0.963767530,
                57.159559,
                93.657325,
                14.992557,
                219.221057,
                0.187340267,
            ),
            arc(
                2465741.5,
                3.022883298,
                0.963746908,
                57.107537,
                93.656267,
                15.012855,
                216.013089,
                0.187566172,
            ),
            arc(
                2467341.5,
                3.023289229,
                0.963967915,
                57.083287,
                93.663325,
                15.048834,
                217.233085,
                0.187491809,
            ),
            arc(
                2467661.5,
                3.023050640,
                0.963973534,
                57.081951,
                93.664491,
                15.056040,
                216.838199,
                0.187516308,
            ),
            arc(
                2467911.5,
                3.022620366,
                0.963907939,
                56.961439,
                93.659509,
                15.069180,
                214.837911,
                0.187639601,
            ),
            arc(
                2468631.5,
                3.021882903,
                0.964205172,
                57.170482,
                93.733523,
                15.045083,
                215.090036,
                0.187621190,
            ),
            arc(
                2469591.5,
                3.022051151,
                0.964209044,
                57.170531,
                93.741262,
                15.058835,
                215.542890,
                0.187596092,
            ),
            arc(
                2470231.5,
                3.022741529,
                0.964013838,
                56.889937,
                93.665757,
                15.106978,
                216.151880,
                0.187562415,
            ),
            arc(
                2471511.5,
                3.024551269,
                0.964031787,
                56.880267,
                93.657240,
                15.106476,
                219.896561,
                0.187374811,
            ),
            arc(
                2471571.5,
                3.026049013,
                0.964043832,
                56.884498,
                93.656309,
                15.111493,
                223.017188,
                0.187218505,
            ),
            arc(
                2471651.5,
                3.029315446,
                0.964063245,
                56.901127,
                93.655391,
                15.124367,
                230.638109,
                0.186837017,
            ),
            arc(
                2471721.5,
                3.033691248,
                0.964078541,
                56.924927,
                93.655859,
                15.143979,
                241.995724,
                0.186268993,
            ),
            arc(
                2471771.5,
                3.038998099,
                0.964084817,
                56.945468,
                93.657082,
                15.170588,
                257.011740,
                0.185518713,
            ),
            arc(
                2471811.5,
                3.046408690,
                0.964077206,
                56.950045,
                93.657437,
                15.212220,
                279.536563,
                0.184394296,
            ),
            arc(
                2471851.5,
                3.038767637,
                0.964785510,
                54.962108,
                93.331109,
                15.260021,
                244.673653,
                0.186123936,
            ),
            arc(
                2473441.5,
                3.042244986,
                0.964842134,
                54.951395,
                93.335880,
                15.266651,
                253.866226,
                0.185704101,
            ),
            arc(
                2474901.5,
                3.042285190,
                0.964750852,
                54.762187,
                93.252460,
                15.308589,
                252.963356,
                0.185741865,
            ),
            arc(
                2475381.5,
                3.040294641,
                0.964775683,
                54.800685,
                93.237251,
                15.286818,
                248.253947,
                0.185939440,
            ),
            arc(
                2476261.5,
                3.035205771,
                0.966796533,
                54.044769,
                92.929465,
                15.308465,
                236.884940,
                0.186440518,
            ),
            arc(
                2477311.5,
                3.038985087,
                0.966947799,
                54.043341,
                92.893872,
                15.300576,
                248.012127,
                0.186008533,
            ),
        ],
    ), // q 0.116 AU, 24 arc(s), radial 0.082%  perihelion 2028-05-12, 2033-08-16, 2038-11-20, 2044-02-21, …
    small(
        "Hartley 2",
        "103P/Hartley 2",
        Class::Comet,
        0.000000580,
        Tail::Ion,
        0.085,
        "EPOXI found jets of CO2 blasting ice out of it in 2010",
        &[
            arc(
                2461041.5,
                3.477640304,
                0.692841741,
                13.591141,
                219.763238,
                181.297820,
                120.398564,
                0.151972753,
            ),
            arc(
                2463561.5,
                3.473520066,
                0.694217304,
                13.617936,
                219.755028,
                181.286637,
                116.966808,
                0.152254928,
            ),
            arc(
                2465021.5,
                3.468481430,
                0.694030829,
                13.619628,
                219.750960,
                181.277832,
                112.070621,
                0.152619380,
            ),
            arc(
                2466511.5,
                3.463003947,
                0.695925832,
                13.641152,
                219.770603,
                181.354614,
                106.790169,
                0.152969926,
            ),
            arc(
                2467301.5,
                3.461861715,
                0.696022481,
                13.641931,
                219.752820,
                181.386058,
                106.230839,
                0.153005309,
            ),
            arc(
                2468391.5,
                3.459306272,
                0.696868652,
                13.660822,
                219.731024,
                181.404798,
                103.500653,
                0.153164969,
            ),
            arc(
                2469681.5,
                3.451655452,
                0.696858035,
                13.672185,
                219.632036,
                181.404818,
                93.779512,
                0.153701474,
            ),
            arc(
                2471181.5,
                3.430651252,
                0.707109438,
                13.793003,
                219.652592,
                181.502501,
                65.402545,
                0.155175420,
            ),
            arc(
                2471301.5,
                3.392400064,
                0.715372994,
                13.211997,
                218.482391,
                183.520029,
                17.661228,
                0.157518133,
            ),
            arc(
                2471921.5,
                3.374359043,
                0.715054997,
                13.201722,
                218.244826,
                184.007434,
                348.013689,
                0.158967923,
            ),
            arc(
                2474261.5,
                3.370522678,
                0.715388108,
                13.217290,
                218.098104,
                184.125895,
                342.391356,
                0.159215824,
            ),
            arc(
                2475371.5,
                3.328283388,
                0.735015054,
                13.605473,
                218.027204,
                183.839806,
                206.126252,
                0.164977403,
            ),
            arc(
                2475481.5,
                3.306094190,
                0.747579802,
                13.644816,
                218.040665,
                183.791579,
                152.754732,
                0.167236632,
            ),
            arc(
                2475571.5,
                3.305394744,
                0.757796343,
                11.999449,
                216.512269,
                184.611508,
                155.368631,
                0.167289717,
            ),
            arc(
                2475651.5,
                3.375053437,
                0.736128936,
                9.542849,
                212.451876,
                187.040001,
                347.666140,
                0.159458138,
            ),
            arc(
                2475791.5,
                3.411889377,
                0.723704900,
                8.876524,
                210.405191,
                188.190457,
                72.608297,
                0.155987076,
            ),
            arc(
                2476111.5,
                3.403672628,
                0.722557021,
                8.800671,
                209.900020,
                188.749748,
                48.799632,
                0.156944107,
            ),
            arc(
                2476481.5,
                3.403445664,
                0.722750793,
                8.807628,
                209.838847,
                188.849922,
                49.213107,
                0.156927042,
            ),
        ],
    ), // q 1.068 AU, 18 arc(s), radial 0.158%  perihelion 2030-04-05, 2036-09-27, 2043-03-12, 2049-08-19, …
    small(
        "308635",
        "308635 (2005 YU55)",
        Class::Asteroid,
        0.000000074,
        Tail::None,
        0.095,
        concat!(
            "Passes 203 778 km from the Earth on 08 Nov 2075. Roughly 147 m across, estimated ",
            "from its brightness"
        ),
        &[
            arc(
                2461041.5,
                1.156650644,
                0.430501246,
                0.339692,
                35.933139,
                273.574807,
                214.103272,
                0.792345911,
            ),
            arc(
                2461731.5,
                1.156101595,
                0.430228424,
                0.339161,
                35.947080,
                273.582516,
                208.543300,
                0.792891283,
            ),
            arc(
                2462151.5,
                1.166083171,
                0.434113462,
                0.492389,
                53.448538,
                255.453183,
                317.042563,
                0.782709915,
            ),
            arc(
                2463101.5,
                1.166210668,
                0.434262731,
                0.490523,
                53.948732,
                254.937477,
                318.362131,
                0.782598131,
            ),
            arc(
                2465031.5,
                1.165914317,
                0.434325676,
                0.490388,
                53.950495,
                254.922203,
                314.411438,
                0.782892297,
            ),
            arc(
                2466321.5,
                1.165669345,
                0.434113320,
                0.490393,
                53.975175,
                254.926352,
                310.554689,
                0.783152675,
            ),
            arc(
                2466861.5,
                1.166045326,
                0.434180767,
                0.490295,
                53.966736,
                254.944110,
                316.063991,
                0.782792586,
            ),
            arc(
                2468331.5,
                1.166542224,
                0.434317049,
                0.490543,
                53.960358,
                254.921151,
                324.807667,
                0.782269699,
            ),
            arc(
                2470001.5,
                1.166611748,
                0.434283615,
                0.490346,
                53.975968,
                254.917225,
                326.453760,
                0.782177580,
            ),
            arc(
                2471821.5,
                1.165107334,
                0.433749579,
                0.489058,
                53.993068,
                254.957289,
                295.219566,
                0.783717907,
            ),
            arc(
                2472001.5,
                1.165112988,
                0.433736167,
                0.488970,
                53.990612,
                254.960759,
                295.786638,
                0.783690144,
            ),
            arc(
                2472151.5,
                1.165060857,
                0.433776182,
                0.488968,
                53.992937,
                254.963454,
                294.368607,
                0.783758709,
            ),
            arc(
                2472221.5,
                1.165003521,
                0.433779657,
                0.488919,
                53.996367,
                254.966071,
                292.896482,
                0.783829625,
            ),
            arc(
                2472261.5,
                1.165427248,
                0.433957545,
                0.489310,
                54.024736,
                254.919399,
                302.562427,
                0.783364192,
            ),
            arc(
                2472311.5,
                1.165163624,
                0.433782626,
                0.488101,
                54.455363,
                254.459424,
                294.528648,
                0.783752516,
            ),
            arc(
                2473131.5,
                1.165487879,
                0.433903561,
                0.487387,
                54.547731,
                254.381737,
                303.714778,
                0.783322901,
            ),
            arc(
                2475071.5,
                1.165363934,
                0.433852000,
                0.487177,
                54.569354,
                254.358973,
                300.672935,
                0.783451790,
            ),
            arc(
                2476451.5,
                1.165273462,
                0.433853421,
                0.487091,
                54.582121,
                254.353326,
                297.519049,
                0.783577156,
            ),
            arc(
                2477921.5,
                1.165108565,
                0.433953779,
                0.486894,
                54.581339,
                254.346030,
                295.890440,
                0.783640797,
            ),
            arc(
                2479081.5,
                1.165276841,
                0.433951681,
                0.491806,
                53.350117,
                255.593356,
                300.890460,
                0.783460289,
            ),
            arc(
                2479261.5,
                1.155522391,
                0.432687894,
                1.005653,
                49.614154,
                258.417033,
                23.996460,
                0.793486731,
            ),
            arc(
                2479271.5,
                1.155528580,
                0.432670801,
                1.005619,
                49.613684,
                258.419255,
                24.270296,
                0.793476864,
            ),
            arc(
                2479281.5,
                1.155529331,
                0.432662671,
                1.005605,
                49.613403,
                258.419985,
                24.311184,
                0.793475408,
            ),
            arc(
                2479291.5,
                1.155528343,
                0.432657724,
                1.005599,
                49.613233,
                258.420187,
                24.283461,
                0.793476429,
            ),
        ],
    ), // q 0.659 AU, 24 arc(s), radial 0.084%  perihelion 2026-08-17, 2027-11-15, 2029-02-11, 2030-05-16, …
    small(
        "2024 QP2",
        "(2024 QP2)",
        Class::Asteroid,
        0.000000088,
        Tail::None,
        0.061,
        concat!(
            "Passes 220 532 km from the Earth on 15 Oct 2028. Roughly 176 m across, estimated ",
            "from its brightness"
        ),
        &[
            arc(
                2461041.5,
                2.544298803,
                0.622569088,
                1.643845,
                205.309624,
                202.153750,
                321.534883,
                0.242867584,
            ),
            arc(
                2461981.5,
                2.544964516,
                0.622450163,
                1.643694,
                205.290923,
                202.144538,
                322.579681,
                0.242767962,
            ),
            arc(
                2462021.5,
                2.545366802,
                0.622503562,
                1.643730,
                205.289644,
                202.141714,
                323.244847,
                0.242704881,
            ),
            arc(
                2462031.5,
                2.545493197,
                0.622524594,
                1.643786,
                205.288128,
                202.143331,
                323.435405,
                0.242686804,
            ),
            arc(
                2462041.5,
                2.545981322,
                0.622604814,
                1.644120,
                205.282460,
                202.150704,
                324.132270,
                0.242620670,
            ),
            arc(
                2462051.5,
                2.556020586,
                0.623510743,
                2.191255,
                204.387577,
                202.510292,
                339.317707,
                0.241188427,
            ),
            arc(
                2462371.5,
                2.562985254,
                0.620611293,
                2.186980,
                204.513695,
                202.613661,
                351.341634,
                0.240075574,
            ),
            arc(
                2463281.5,
                2.565690587,
                0.617925185,
                2.178504,
                204.160495,
                202.848244,
                354.433682,
                0.239798166,
            ),
            arc(
                2465071.5,
                2.563579701,
                0.618058767,
                2.179365,
                204.162770,
                202.890652,
                349.677956,
                0.240150630,
            ),
            arc(
                2466601.5,
                2.565351302,
                0.617815651,
                2.179010,
                204.149110,
                202.896308,
                353.300882,
                0.239909735,
            ),
            arc(
                2467451.5,
                2.571400385,
                0.614949488,
                2.171607,
                203.993264,
                202.995365,
                8.008929,
                0.238987245,
            ),
            arc(
                2468101.5,
                2.571136160,
                0.614922548,
                2.173341,
                203.939363,
                203.060932,
                7.292970,
                0.239030309,
            ),
            arc(
                2469561.5,
                2.568647555,
                0.614982265,
                2.174213,
                203.920600,
                203.074082,
                359.726975,
                0.239450095,
            ),
            arc(
                2471121.5,
                2.569336176,
                0.614953949,
                2.174036,
                203.923875,
                203.051910,
                1.449096,
                0.239362277,
            ),
            arc(
                2472131.5,
                2.572697329,
                0.613844857,
                2.171040,
                203.845562,
                203.049227,
                12.437528,
                0.238832923,
            ),
            arc(
                2472621.5,
                2.573843867,
                0.613859484,
                2.172072,
                203.820991,
                203.107337,
                16.291510,
                0.238649763,
            ),
            arc(
                2474101.5,
                2.570686519,
                0.614185616,
                2.174443,
                203.783461,
                203.072810,
                4.624520,
                0.239166867,
            ),
            arc(
                2475621.5,
                2.572105332,
                0.614786855,
                2.174578,
                203.763125,
                203.107967,
                9.315025,
                0.238972233,
            ),
            arc(
                2477111.5,
                2.575174697,
                0.614648084,
                2.175132,
                203.702759,
                203.165587,
                22.610139,
                0.238452373,
            ),
        ],
    ), // q 0.960 AU, 19 arc(s), radial 0.184%  perihelion 2028-11-03, 2032-12-11, 2037-01-21, 2041-02-28, …
    small(
        "153814",
        "153814 (2001 WN5)",
        Class::Asteroid,
        0.000000392,
        Tail::None,
        0.131,
        concat!(
            "Passes 248 711 km from the Earth on 26 Jun 2028. Roughly 784 m across, estimated ",
            "from its brightness"
        ),
        &[
            arc(
                2461041.5,
                1.711616428,
                0.467317491,
                1.919545,
                277.351597,
                44.642596,
                84.834903,
                0.440141306,
            ),
            arc(
                2461191.5,
                1.711574411,
                0.467304803,
                1.919532,
                277.350714,
                44.642522,
                84.665079,
                0.440158990,
            ),
            arc(
                2461221.5,
                1.711562477,
                0.467301509,
                1.919540,
                277.349996,
                44.642932,
                84.615386,
                0.440164164,
            ),
            arc(
                2461251.5,
                1.711784751,
                0.467335759,
                1.919668,
                277.346212,
                44.657617,
                85.581298,
                0.440063552,
            ),
            arc(
                2461281.5,
                1.711712938,
                0.467302180,
                1.920359,
                277.333697,
                44.668779,
                85.014441,
                0.440122305,
            ),
            arc(
                2461321.5,
                1.711894034,
                0.467283396,
                1.920246,
                277.334463,
                44.680497,
                85.789425,
                0.440041736,
            ),
            arc(
                2461371.5,
                1.712088618,
                0.467214275,
                1.920031,
                277.334112,
                44.694862,
                86.666077,
                0.439951151,
            ),
            arc(
                2461441.5,
                1.712540913,
                0.466926379,
                1.919076,
                277.321918,
                44.732279,
                89.047941,
                0.439708155,
            ),
            arc(
                2461601.5,
                1.712733673,
                0.466759954,
                1.917518,
                277.268956,
                44.787270,
                89.927533,
                0.439619952,
            ),
            arc(
                2461731.5,
                1.712379211,
                0.466916515,
                1.917097,
                277.225551,
                44.854773,
                87.877160,
                0.439818410,
            ),
            arc(
                2461841.5,
                1.712054251,
                0.466946427,
                1.917170,
                277.215033,
                44.887503,
                86.225775,
                0.439976706,
            ),
            arc(
                2461881.5,
                1.711908631,
                0.466933017,
                1.917205,
                277.213446,
                44.897262,
                85.533820,
                0.440042854,
            ),
            arc(
                2461901.5,
                1.712043943,
                0.466959501,
                1.917223,
                277.212942,
                44.891959,
                86.204103,
                0.439978761,
            ),
            arc(
                2461911.5,
                1.712033849,
                0.466970133,
                1.917286,
                277.211470,
                44.896017,
                86.143677,
                0.439984532,
            ),
            arc(
                2461921.5,
                1.712039035,
                0.466984743,
                1.917393,
                277.209739,
                44.900376,
                86.135843,
                0.439985264,
            ),
            arc(
                2461931.5,
                1.683277037,
                0.459685485,
                2.392379,
                276.528648,
                46.502181,
                327.212610,
                0.451344253,
            ),
            arc(
                2462611.5,
                1.683448679,
                0.459643011,
                2.396210,
                276.651589,
                46.367530,
                328.534399,
                0.451224198,
            ),
            arc(
                2465201.5,
                1.683845838,
                0.459855367,
                2.394691,
                276.606398,
                46.419902,
                331.234641,
                0.451027415,
            ),
            arc(
                2465961.5,
                1.683123907,
                0.459783735,
                2.400423,
                276.440027,
                46.653956,
                326.273228,
                0.451371431,
            ),
            arc(
                2466811.5,
                1.683335114,
                0.459749876,
                2.401917,
                276.374691,
                46.694086,
                327.547127,
                0.451286991,
            ),
            arc(
                2469201.5,
                1.683324056,
                0.460132861,
                2.401743,
                276.318905,
                46.734687,
                327.079250,
                0.451311964,
            ),
            arc(
                2470151.5,
                1.683659718,
                0.459765445,
                2.398816,
                276.141210,
                46.939085,
                330.111766,
                0.451145661,
            ),
            arc(
                2473941.5,
                1.684065935,
                0.459980406,
                2.397148,
                276.072270,
                47.006294,
                335.077804,
                0.450926134,
            ),
            arc(
                2474771.5,
                1.683397863,
                0.459878061,
                2.395578,
                275.941269,
                47.203789,
                327.663650,
                0.451246330,
            ),
        ],
    ), // q 0.912 AU, 24 arc(s), radial 0.093%  perihelion 2026-05-06, 2028-08-03, 2030-10-09, 2032-12-15, …
    small(
        "2005 WY55",
        "(2005 WY55)",
        Class::Asteroid,
        0.000000129,
        Tail::None,
        0.099,
        concat!(
            "Passes 332 741 km from the Earth on 28 May 2065. Roughly 257 m across, estimated ",
            "from its brightness"
        ),
        &[
            arc(
                2461041.5,
                2.493734137,
                0.718816792,
                7.272537,
                247.869210,
                286.487823,
                165.951391,
                0.250249209,
            ),
            arc(
                2462371.5,
                2.492267186,
                0.718581832,
                7.278170,
                247.835091,
                286.556843,
                162.908582,
                0.250529977,
            ),
            arc(
                2463831.5,
                2.495119804,
                0.718017376,
                7.269694,
                247.711969,
                286.715220,
                167.556215,
                0.250151422,
            ),
            arc(
                2465101.5,
                2.498916587,
                0.716809522,
                7.274878,
                247.543190,
                286.800757,
                176.737195,
                0.249470583,
            ),
            arc(
                2466751.5,
                2.497584779,
                0.716501500,
                7.280015,
                247.491741,
                286.897021,
                172.942276,
                0.249721708,
            ),
            arc(
                2468161.5,
                2.498656710,
                0.716287030,
                7.275630,
                247.472319,
                286.894515,
                175.394886,
                0.249574026,
            ),
            arc(
                2468801.5,
                2.502043987,
                0.714467909,
                7.278536,
                247.234375,
                287.145875,
                184.474221,
                0.249050050,
            ),
            arc(
                2469561.5,
                2.503597848,
                0.714567989,
                7.277060,
                247.173716,
                287.160162,
                189.438554,
                0.248774924,
            ),
            arc(
                2471041.5,
                2.502474665,
                0.714447137,
                7.281882,
                247.109794,
                287.279500,
                185.272984,
                0.248988856,
            ),
            arc(
                2472491.5,
                2.503951211,
                0.714213879,
                7.279500,
                247.078158,
                287.288130,
                189.556486,
                0.248784182,
            ),
            arc(
                2473131.5,
                2.507438081,
                0.712370690,
                7.282243,
                246.849826,
                287.529317,
                201.293855,
                0.248242606,
            ),
            arc(
                2473911.5,
                2.509262527,
                0.712578569,
                7.292186,
                246.660454,
                287.682800,
                210.412087,
                0.247835344,
            ),
            arc(
                2475621.5,
                2.461040249,
                0.707802574,
                7.383255,
                246.746741,
                287.003791,
                32.180968,
                0.255318153,
            ),
            arc(
                2477471.5,
                2.467531417,
                0.705817237,
                7.382773,
                246.431191,
                287.303121,
                60.559986,
                0.254225649,
            ),
        ],
    ), // q 0.701 AU, 14 arc(s), radial 0.413%  perihelion 2029-09-10, 2033-08-17, 2037-07-27, 2041-07-09, …
    small(
        "137108",
        "137108 (1999 AN10)",
        Class::Asteroid,
        0.000000436,
        Tail::None,
        0.095,
        concat!(
            "Passes 389 855 km from the Earth on 07 Aug 2027. Roughly 872 m across, estimated ",
            "from its brightness"
        ),
        &[
            arc(
                2461041.5,
                1.458541423,
                0.562051707,
                39.932344,
                314.321787,
                268.339429,
                150.077177,
                0.559540966,
            ),
            arc(
                2461231.5,
                1.458572444,
                0.562047522,
                39.932177,
                314.321840,
                268.330720,
                150.336638,
                0.559516483,
            ),
            arc(
                2461571.5,
                1.448566334,
                0.560129830,
                40.001832,
                314.303555,
                267.909096,
                91.042450,
                0.565440422,
            ),
            arc(
                2462071.5,
                1.448113887,
                0.560431398,
                40.007735,
                314.297326,
                267.914532,
                89.521110,
                0.565588778,
            ),
            arc(
                2463511.5,
                1.447941860,
                0.560344689,
                40.006991,
                314.248576,
                267.913939,
                88.124324,
                0.565704843,
            ),
            arc(
                2466561.5,
                1.447838522,
                0.560373967,
                40.005586,
                314.195984,
                267.929950,
                87.282479,
                0.565761152,
            ),
            arc(
                2469151.5,
                1.447782951,
                0.560348871,
                40.005870,
                314.124748,
                267.950187,
                87.055811,
                0.565775214,
            ),
            arc(
                2473031.5,
                1.447859978,
                0.560388452,
                40.002517,
                314.053093,
                267.971277,
                87.695458,
                0.565746346,
            ),
            arc(
                2475591.5,
                1.447905781,
                0.560370098,
                40.003849,
                314.021839,
                267.991123,
                88.226985,
                0.565723369,
            ),
            arc(
                2476851.5,
                1.448038960,
                0.560413978,
                39.999308,
                313.978846,
                267.984729,
                90.119793,
                0.565649091,
            ),
        ],
    ), // q 0.639 AU, 10 arc(s), radial 0.132%  perihelion 2027-06-14, 2029-03-11, 2030-12-08, 2032-09-04, …
    small(
        "523654",
        "523654 (2011 SR5)",
        Class::Asteroid,
        0.000000116,
        Tail::None,
        0.097,
        concat!(
            "Passes 476 205 km from the Earth on 23 Sep 2066. Roughly 231 m across, estimated ",
            "from its brightness"
        ),
        &[
            arc(
                2461041.5,
                1.178189674,
                0.705662188,
                29.132901,
                180.080392,
                305.580019,
                257.636677,
                0.770719567,
            ),
            arc(
                2462411.5,
                1.178178078,
                0.705604911,
                29.133881,
                180.067032,
                305.577345,
                257.711831,
                0.770714337,
            ),
            arc(
                2463351.5,
                1.178272994,
                0.705692259,
                29.142122,
                180.027631,
                305.624439,
                259.280235,
                0.770580979,
            ),
            arc(
                2463831.5,
                1.178131720,
                0.705644084,
                29.151827,
                180.015948,
                305.640992,
                257.202765,
                0.770750100,
            ),
            arc(
                2463841.5,
                1.178233549,
                0.705453537,
                29.151501,
                180.011097,
                305.635954,
                258.905692,
                0.770614917,
            ),
            arc(
                2464741.5,
                1.178350764,
                0.705516009,
                29.155574,
                179.987586,
                305.647118,
                259.914273,
                0.770537063,
            ),
            arc(
                2466631.5,
                1.178226838,
                0.705472802,
                29.160571,
                179.959252,
                305.668270,
                258.162229,
                0.770652914,
            ),
            arc(
                2468031.5,
                1.178107009,
                0.705363352,
                29.177715,
                179.911446,
                305.724629,
                255.989016,
                0.770785207,
            ),
            arc(
                2469421.5,
                1.178214879,
                0.705412271,
                29.180880,
                179.893677,
                305.731819,
                258.279947,
                0.770656867,
            ),
            arc(
                2469891.5,
                1.178106203,
                0.705372644,
                29.185195,
                179.879805,
                305.760285,
                256.166320,
                0.770772070,
            ),
            arc(
                2470831.5,
                1.178090389,
                0.705278138,
                29.185879,
                179.870636,
                305.755283,
                255.352720,
                0.770814101,
            ),
            arc(
                2472111.5,
                1.178099027,
                0.705357370,
                29.198404,
                179.824311,
                305.808473,
                256.316698,
                0.770766740,
            ),
            arc(
                2472701.5,
                1.178055915,
                0.705160533,
                29.204579,
                179.806228,
                305.808226,
                254.993245,
                0.770829288,
            ),
            arc(
                2474091.5,
                1.178098396,
                0.705228274,
                29.210630,
                179.783539,
                305.835484,
                256.065376,
                0.770781702,
            ),
            arc(
                2475021.5,
                1.177976748,
                0.705185464,
                29.209086,
                179.782695,
                305.827645,
                255.159397,
                0.770819911,
            ),
            arc(
                2475961.5,
                1.174328756,
                0.705857267,
                29.153179,
                179.768501,
                306.106905,
                165.376867,
                0.774496627,
            ),
            arc(
                2475971.5,
                1.174326651,
                0.705856837,
                29.153197,
                179.768447,
                306.107024,
                165.326037,
                0.774498708,
            ),
            arc(
                2475981.5,
                1.174332657,
                0.705918154,
                29.154515,
                179.750053,
                306.120881,
                165.515719,
                0.774490830,
            ),
            arc(
                2476421.5,
                1.174347394,
                0.705987831,
                29.160741,
                179.741838,
                306.133182,
                165.838893,
                0.774477917,
            ),
            arc(
                2476441.5,
                1.174415010,
                0.705905134,
                29.163034,
                179.722669,
                306.160721,
                168.174797,
                0.774384153,
            ),
            arc(
                2476891.5,
                1.174294133,
                0.705819684,
                29.171726,
                179.711839,
                306.171916,
                163.890872,
                0.774553091,
            ),
            arc(
                2477311.5,
                1.174307429,
                0.705657773,
                29.171071,
                179.709782,
                306.159524,
                164.577523,
                0.774525962,
            ),
            arc(
                2477791.5,
                1.174361844,
                0.705742955,
                29.171073,
                179.701048,
                306.156299,
                166.202908,
                0.774464410,
            ),
            arc(
                2478291.5,
                1.174331169,
                0.705799676,
                29.176608,
                179.682734,
                306.182883,
                165.398491,
                0.774494302,
            ),
        ],
    ), // q 0.347 AU, 24 arc(s), radial 0.231%  perihelion 2027-03-22, 2028-07-01, 2029-10-11, 2031-01-21, …
    small(
        "789058",
        "789058 (2017 MB1)",
        Class::Asteroid,
        0.000000303,
        Tail::None,
        0.104,
        concat!(
            "Passes 479 967 km from the Earth on 25 Jul 2072. Roughly 606 m across, estimated ",
            "from its brightness"
        ),
        &[
            arc(
                2461041.5,
                2.373311181,
                0.751983404,
                8.505252,
                126.781846,
                264.795380,
                59.077132,
                0.269578651,
            ),
            arc(
                2462161.5,
                2.378844900,
                0.749645675,
                8.474165,
                126.258390,
                265.448842,
                70.436685,
                0.268517770,
            ),
            arc(
                2463331.5,
                2.382530369,
                0.749381567,
                8.475644,
                125.961336,
                265.592172,
                76.678930,
                0.267988138,
            ),
            arc(
                2464691.5,
                2.380611480,
                0.749255420,
                8.476439,
                125.914059,
                265.722937,
                72.069510,
                0.268339226,
            ),
            arc(
                2466041.5,
                2.383839256,
                0.749183641,
                8.467787,
                125.873002,
                265.783416,
                81.530770,
                0.267685989,
            ),
            arc(
                2466651.5,
                2.385614809,
                0.748745196,
                8.439897,
                123.078477,
                268.706201,
                87.004003,
                0.267273481,
            ),
            arc(
                2467351.5,
                2.380729336,
                0.748494449,
                8.460538,
                122.955852,
                268.904004,
                70.702863,
                0.268303158,
            ),
            arc(
                2468111.5,
                2.379530360,
                0.749071258,
                8.461328,
                122.928159,
                268.935854,
                67.034385,
                0.268524491,
            ),
            arc(
                2468761.5,
                2.380461315,
                0.748586643,
                8.459237,
                122.893369,
                269.014170,
                70.242486,
                0.268339277,
            ),
            arc(
                2470101.5,
                2.382786915,
                0.748777264,
                8.454654,
                122.847948,
                268.997986,
                78.403862,
                0.267898802,
            ),
            arc(
                2470761.5,
                2.377505613,
                0.751870810,
                8.459539,
                122.132205,
                269.742194,
                60.046540,
                0.268848363,
            ),
            arc(
                2471391.5,
                2.374975314,
                0.752031571,
                8.466419,
                122.104909,
                269.780831,
                50.869766,
                0.269310380,
            ),
            arc(
                2472761.5,
                2.376508673,
                0.752058278,
                8.463344,
                122.056893,
                269.879117,
                57.551770,
                0.268995626,
            ),
            arc(
                2474121.5,
                2.375817279,
                0.752602782,
                8.462745,
                121.977000,
                269.861035,
                53.240265,
                0.269185719,
            ),
            arc(
                2475401.5,
                2.375225160,
                0.752851611,
                8.464888,
                121.901300,
                269.993216,
                50.851021,
                0.269286214,
            ),
            arc(
                2476741.5,
                2.377448129,
                0.752756369,
                8.459770,
                121.866439,
                269.989089,
                62.649134,
                0.268817849,
            ),
            arc(
                2478091.5,
                2.369384955,
                0.752451669,
                8.335233,
                121.757959,
                270.271685,
                24.863628,
                0.270240645,
            ),
            arc(
                2478101.5,
                2.369501035,
                0.752463730,
                8.335238,
                121.757857,
                270.271978,
                25.410870,
                0.270220033,
            ),
            arc(
                2478121.5,
                2.369576446,
                0.752471396,
                8.335270,
                121.757746,
                270.272397,
                25.776674,
                0.270206255,
            ),
            arc(
                2478151.5,
                2.369338723,
                0.752455389,
                8.335332,
                121.757810,
                270.269122,
                24.609787,
                0.270250207,
            ),
            arc(
                2478221.5,
                2.368790986,
                0.752442598,
                8.335406,
                121.758243,
                270.258296,
                21.590638,
                0.270363852,
            ),
            arc(
                2478361.5,
                2.368702757,
                0.752438102,
                8.335334,
                121.756776,
                270.257523,
                20.726633,
                0.270396271,
            ),
            arc(
                2478701.5,
                2.368827840,
                0.752342803,
                8.335310,
                121.752160,
                270.261958,
                21.160458,
                0.270380510,
            ),
            arc(
                2479121.5,
                2.369051328,
                0.752302668,
                8.335552,
                121.748127,
                270.259499,
                23.045099,
                0.270312564,
            ),
        ],
    ), // q 0.589 AU, 24 arc(s), radial 0.210%  perihelion 2028-08-25, 2032-04-26, 2035-12-30, 2039-09-02, …
    small(
        "2021 MK1",
        "(2021 MK1)",
        Class::Asteroid,
        0.000000097,
        Tail::None,
        0.080,
        concat!(
            "Passes 550 336 km from the Earth on 25 Jun 2066. Roughly 193 m across, estimated ",
            "from its brightness"
        ),
        &[
            arc(
                2461041.5,
                0.807700120,
                0.287946416,
                19.241742,
                273.979266,
                199.000719,
                298.881373,
                1.357776418,
            ),
            arc(
                2461871.5,
                0.807660562,
                0.287891633,
                19.240979,
                273.968877,
                198.989362,
                298.253841,
                1.357836366,
            ),
            arc(
                2462461.5,
                0.807557207,
                0.288094691,
                19.239818,
                273.940627,
                199.014909,
                294.856417,
                1.358150945,
            ),
            arc(
                2465201.5,
                0.807481410,
                0.288179315,
                19.238524,
                273.910571,
                199.041440,
                292.203703,
                1.358339541,
            ),
            arc(
                2466971.5,
                0.807548870,
                0.288139255,
                19.237793,
                273.886858,
                199.060140,
                295.232882,
                1.358142321,
            ),
            arc(
                2469081.5,
                0.807528155,
                0.288196954,
                19.237906,
                273.869976,
                199.074246,
                293.983952,
                1.358216333,
            ),
            arc(
                2469891.5,
                0.807693935,
                0.287977689,
                19.237135,
                273.850206,
                199.106599,
                302.254544,
                1.357765365,
            ),
            arc(
                2471751.5,
                0.807705593,
                0.288022564,
                19.236901,
                273.833917,
                199.125681,
                302.210984,
                1.357768788,
            ),
            arc(
                2472891.5,
                0.808428522,
                0.287006582,
                19.241987,
                273.804626,
                199.246981,
                340.636487,
                1.355964610,
            ),
            arc(
                2473331.5,
                0.808424824,
                0.287039236,
                19.243320,
                273.797952,
                199.240268,
                340.987367,
                1.355948937,
            ),
            arc(
                2473831.5,
                0.808401819,
                0.287100266,
                19.243388,
                273.795340,
                199.246404,
                339.364325,
                1.356021826,
            ),
            arc(
                2474391.5,
                0.808407618,
                0.287054895,
                19.242488,
                273.784884,
                199.258459,
                340.203291,
                1.355985580,
            ),
            arc(
                2475681.5,
                0.808419454,
                0.287064249,
                19.242851,
                273.779154,
                199.254808,
                340.625833,
                1.355968244,
            ),
            arc(
                2475771.5,
                0.808457021,
                0.287071726,
                19.243156,
                273.777324,
                199.264622,
                343.304864,
                1.355857288,
            ),
            arc(
                2475791.5,
                0.808432289,
                0.287090166,
                19.243894,
                273.775818,
                199.260129,
                341.009827,
                1.355952169,
            ),
            arc(
                2475801.5,
                0.808444387,
                0.287092875,
                19.245180,
                273.774162,
                199.265390,
                342.001672,
                1.355911052,
            ),
            arc(
                2475811.5,
                0.813378723,
                0.279820285,
                19.326137,
                273.736899,
                199.834670,
                281.438143,
                1.343561034,
            ),
            arc(
                2476271.5,
                0.813378685,
                0.279820223,
                19.326595,
                273.733338,
                199.836551,
                280.877223,
                1.343583680,
            ),
            arc(
                2476751.5,
                0.813334399,
                0.279818767,
                19.325576,
                273.726242,
                199.844011,
                278.007525,
                1.343697475,
            ),
            arc(
                2477261.5,
                0.813288282,
                0.279800913,
                19.325957,
                273.715190,
                199.849692,
                274.811534,
                1.343821886,
            ),
            arc(
                2477901.5,
                0.813295667,
                0.279828706,
                19.326070,
                273.709004,
                199.858956,
                275.657159,
                1.343790173,
            ),
        ],
    ), // q 0.575 AU, 21 arc(s), radial 0.046%  perihelion 2026-04-04, 2026-12-25, 2027-09-16, 2028-06-07, …
    small(
        "2018 BM3",
        "(2018 BM3)",
        Class::Asteroid,
        0.000000071,
        Tail::None,
        0.090,
        concat!(
            "Passes 581 893 km from the Earth on 05 Jan 2051. Roughly 142 m across, estimated ",
            "from its brightness"
        ),
        &[
            arc(
                2461041.5,
                1.116934981,
                0.510657224,
                19.612220,
                105.408093,
                250.841625,
                333.231844,
                0.834940648,
            ),
            arc(
                2465391.5,
                1.117405143,
                0.510744621,
                19.609740,
                105.366512,
                250.900771,
                340.534570,
                0.834414689,
            ),
            arc(
                2466221.5,
                1.117283341,
                0.510647366,
                19.608998,
                105.317050,
                250.963802,
                338.652086,
                0.834543436,
            ),
            arc(
                2470101.5,
                1.115314591,
                0.509850563,
                19.694154,
                105.214151,
                250.978366,
                292.576292,
                0.837026664,
            ),
            arc(
                2470371.5,
                1.115408234,
                0.509642386,
                19.733633,
                105.275142,
                250.941829,
                299.278392,
                0.836667061,
            ),
            arc(
                2470441.5,
                1.115373504,
                0.509666512,
                19.733791,
                105.273603,
                250.946767,
                298.370784,
                0.836714937,
            ),
            arc(
                2470481.5,
                1.115376435,
                0.509668574,
                19.733860,
                105.273368,
                250.946873,
                298.505058,
                0.836707866,
            ),
            arc(
                2470501.5,
                1.115384989,
                0.509670511,
                19.733873,
                105.273346,
                250.946333,
                298.731197,
                0.836695970,
            ),
            arc(
                2470521.5,
                1.115397478,
                0.509674681,
                19.733848,
                105.273341,
                250.945744,
                299.022091,
                0.836680672,
            ),
            arc(
                2470541.5,
                1.115496136,
                0.509684380,
                19.733030,
                105.268611,
                250.959939,
                301.188414,
                0.836566545,
            ),
            arc(
                2470901.5,
                1.115523395,
                0.509634611,
                19.733299,
                105.254907,
                250.976605,
                301.729106,
                0.836538056,
            ),
            arc(
                2471001.5,
                1.115516176,
                0.509635526,
                19.733396,
                105.253031,
                250.975858,
                301.528534,
                0.836548412,
            ),
            arc(
                2471021.5,
                1.115456619,
                0.509622602,
                19.733627,
                105.252800,
                250.972096,
                300.061529,
                0.836623931,
            ),
            arc(
                2471051.5,
                1.115445305,
                0.509622922,
                19.733717,
                105.252827,
                250.971066,
                299.827070,
                0.836636007,
            ),
            arc(
                2471121.5,
                1.115117447,
                0.509392581,
                19.742137,
                105.223103,
                251.004603,
                291.823028,
                0.837038212,
            ),
            arc(
                2474741.5,
                1.115015840,
                0.509440138,
                19.741188,
                105.165192,
                251.051836,
                290.216872,
                0.837105477,
            ),
            arc(
                2477501.5,
                1.115170026,
                0.509424705,
                19.739699,
                105.129709,
                251.099658,
                294.696807,
                0.836933447,
            ),
        ],
    ), // q 0.547 AU, 17 arc(s), radial 0.078%  perihelion 2026-01-22, 2027-03-29, 2028-06-02, 2029-08-07, …
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ephem;

    /// The Horizons samples every test here measures against: geometric
    /// positions in the J2000 ecliptic frame, AU, spread across 2026–2076.
    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("../tests/fixtures/smallbodies.json")).unwrap()
    }

    fn samples(fx: &serde_json::Value, name: &str) -> Vec<(f64, Vec3)> {
        fx["bodies"][name]["samples"]
            .as_array()
            .unwrap_or_else(|| panic!("no {name} in the fixture"))
            .iter()
            .map(|row| {
                let v: Vec<f64> =
                    row.as_array().unwrap().iter().map(|x| x.as_f64().unwrap()).collect();
                (v[0], vec3(v[1], v[2], v[3]))
            })
            .collect()
    }

    fn angle_deg(a: Vec3, b: Vec3) -> f64 {
        (a.normalize().dot(b.normalize())).clamp(-1.0, 1.0).acos().to_degrees()
    }

    /// The table, against JPL Horizons itself.
    ///
    /// This is the test that makes five hundred rows of numbers trustworthy.
    /// Every arc is seven figures, a single mistyped digit moves a body by
    /// degrees, and no amount of reading catches that. Each body is held to the
    /// error *it published for itself*, which is what stops a regression from
    /// hiding behind a tolerance loose enough to cover the worst case.
    #[test]
    fn positions_match_horizons_to_the_published_error() {
        let fx = fixture();
        for b in BODIES {
            let mut worst: f64 = 0.0;
            for (jd, want) in samples(&fx, b.name) {
                // The fixture is J2000; `heliocentric` is of date, so compare
                // against the un-precessed solution.
                worst = worst.max(angle_deg(b.position_j2000(jd), want));
            }
            // A hair of slack for the fixture being a subset of what the fit
            // was measured over — not enough to hide a moved digit.
            assert!(
                worst < b.fit_error_deg + 0.02,
                "{}: {worst:.3}° against Horizons, the table claims {:.3}°",
                b.name,
                b.fit_error_deg
            );
        }
    }

    /// ...and the published errors are themselves small enough to be worth
    /// having. Without this the test above would pass just as happily if a body
    /// claimed 90°.
    #[test]
    fn every_body_is_placed_well_enough_to_draw() {
        let fx = fixture();
        for b in BODIES {
            // The headline figure: what the worst sample in fifty years costs.
            let limit = if b.name == "Apophis" { 0.7 } else { 0.2 };
            assert!(
                b.fit_error_deg < limit,
                "{} claims {:.3}°, which is too far to draw honestly",
                b.name,
                b.fit_error_deg
            );
            // ...and the typical sample, which is what anybody actually looks
            // at. A worst case is allowed to be an excursion; a median this
            // size means the body is simply in the wrong place, and no
            // published-error assertion above would notice.
            let mut errs: Vec<f64> = samples(&fx, b.name)
                .iter()
                .map(|(jd, want)| angle_deg(b.position_j2000(*jd), *want))
                .collect();
            errs.sort_by(f64::total_cmp);
            let median = errs[errs.len() / 2];
            assert!(median < 0.05, "{} is typically {median:.3}° out, not merely at worst", b.name);
        }
    }

    /// The chain has to be continuous. A body that jumped at an arc boundary
    /// would be the one artefact of this model anybody would actually see —
    /// scrub the clock, and it teleports.
    #[test]
    fn nothing_jumps_at_an_arc_boundary() {
        for b in BODIES {
            for w in b.arcs.windows(2) {
                let t = w[1].start_jd;
                // The two arcs meeting at a boundary are each fitted through
                // it, so they must agree about where the body is there. What
                // they disagree about is where it is *going*, and spreading
                // that over `BLEND_D` is the whole job of the cross-fade.
                let gap = angle_deg(w[0].position(t), w[1].position(t));
                assert!(
                    gap < 2.0 * b.fit_error_deg + 0.02,
                    "{}'s arcs disagree by {gap:.4}° at the boundary at {t}",
                    b.name
                );
                // ...and the cross-fade really is spreading that disagreement
                // out rather than letting it land in one frame. Without the
                // blend a single step of the clock across the boundary would
                // move the body by the whole gap on top of its own motion;
                // smoothed over `BLEND_D` it can only take a fraction of it,
                // and this pins that fraction. Both neighbouring steps are
                // measured because a boundary near perihelion has the body
                // travelling several times faster than it does a day later.
                let gap = (w[0].position(t) - w[1].position(t)).len();
                let dt = 0.1;
                let step = |t: f64| {
                    (b.position_j2000(t + dt / 2.0) - b.position_j2000(t - dt / 2.0)).len()
                };
                let (edge, before, after) = (step(t), step(t - BLEND_D), step(t + BLEND_D));
                // Peak rate of a smoothstep is 1.5/width, so `dt` of clock can
                // pick up at most that share of the gap — plus a little, since
                // the two arcs differ in velocity as well as position. A fifth,
                // against the whole gap in one step if the blend were dropped.
                let share = 2.0 * dt / (2.0 * BLEND_D);
                assert!(
                    edge - before.max(after) < share * gap + 1e-12,
                    "{} moves an extra {:.3e} AU across the boundary at {t}, out of a \
                     {gap:.3e} AU disagreement the blend should be spreading over {BLEND_D} days",
                    b.name,
                    edge - before.max(after),
                );
            }
        }
    }

    /// Arcs must be in order and must not overlap, or `arc_index` picks the
    /// wrong one — and they must all start inside the window they were fitted
    /// over, because outside it there is nothing to have fitted against.
    #[test]
    fn arcs_tile_the_window_in_order() {
        for b in BODIES {
            assert!(!b.arcs.is_empty(), "{} has no arcs", b.name);
            assert_eq!(b.arcs[0].start_jd, WINDOW.0, "{} does not start at the window", b.name);
            for w in b.arcs.windows(2) {
                assert!(w[0].start_jd < w[1].start_jd, "{}'s arcs are out of order", b.name);
                // Longer than two cross-fades, or the blend either side of a
                // boundary would reach past the next boundary and the weights
                // would stop summing to one.
                assert!(
                    w[1].start_jd - w[0].start_jd > 4.0 * BLEND_D,
                    "{} has an arc shorter than the blend needs",
                    b.name
                );
            }
            assert!(b.arcs.last().unwrap().start_jd < WINDOW.1, "{} runs past the window", b.name);
        }
    }

    /// Every arc has to be the ellipse the body is *actually* on, not merely
    /// one that reproduces the stretch of path it covers.
    ///
    /// This is the subtle failure mode of the whole scheme, and it bit during
    /// development: an arc covering a fraction of a revolution is fitted almost
    /// as well by a whole family of ellipses, so the fitter is free to return
    /// any member of it. Positions still come out right — that is what it was
    /// optimising — while `a`, `e` and the perihelion come out nonsense. An
    /// early run had Apophis at `a` = 0.987, `e` = 0.222 on one arc and 0.985,
    /// 0.104 on the next, against a true 0.922 and 0.191, and nothing about the
    /// rendered positions gave it away.
    ///
    /// So the check is against the model's own path rather than against the
    /// other arcs: follow the body over a full revolution and see whether it
    /// really does come as close and go as far as this arc claims.
    #[test]
    fn every_arc_is_the_ellipse_the_body_is_really_on() {
        for b in BODIES {
            for arc in b.arcs {
                assert!(arc.a > 0.0 && (0.0..0.999).contains(&arc.e), "{}: {arc:?}", b.name);
                // Kepler's third law, to the 2% the fit was allowed to trim.
                let kepler = 0.985_607_668_6 / arc.a.powf(1.5);
                assert!(
                    (arc.n / kepler - 1.0).abs() < 0.021,
                    "{}: mean motion {} is not Kepler's {kepler}",
                    b.name,
                    arc.n
                );
            }

            // Only the bodies whose year fits inside the window can be walked
            // round one — Pluto's is 248 of them — and a body covered by a
            // single arc has no room to be degenerate anyway.
            let peri = b.next_perihelion(WINDOW.0);
            let arc = b.arc(peri);
            let period = arc.period_d();
            if peri + period > WINDOW.1 {
                continue;
            }
            // Aphelion is half a period on, and the far point of the ellipse
            // this arc claims. `a` and `e` follow from that and from the
            // perihelion distance the test above already pins, so a fit that
            // settled on the wrong member of the family cannot survive both.
            let far = b.distance_au(peri + period * 0.5);
            assert!(
                (far / arc.aphelion() - 1.0).abs() < 0.04,
                "{} claims aphelion at {:.4} AU but reaches {far:.4} AU",
                b.name,
                arc.aphelion()
            );
            // ...and the period is the interval between two of its perihelia,
            // rather than a number that merely lets the phase come out right.
            let again = b.next_perihelion(peri + period * 0.5);
            assert!(
                ((again - peri) / period - 1.0).abs() < 0.04,
                "{} claims a {period:.1} day year but takes {:.1} days between perihelia",
                b.name,
                again - peri
            );
        }
    }

    /// The table's own claims about itself: a dwarf planet is round and far
    /// out, a comet is small and eccentric, and every body says why it is here.
    #[test]
    fn the_table_says_what_each_body_is() {
        let jd = WINDOW.0;
        for b in BODIES {
            assert!(!b.why.is_empty(), "{} does not say why it is in the table", b.name);
            assert!(b.radius > 0.0, "{} has no size", b.name);
            assert!(
                b.designation.contains(b.name) || b.class == Class::Asteroid,
                "{}'s designation {} does not contain its name",
                b.name,
                b.designation
            );
            let arc = b.arc(jd);
            match b.class {
                // Round enough to be a dwarf planet means at least ~400 km
                // across, and all five of ours are outside the main belt or in
                // it — Ceres is the innermost at 2.77 AU.
                Class::Dwarf => {
                    assert!(b.radius > 0.0004, "{} is too small to be round", b.name);
                    assert!(arc.a > 2.5, "{} is inside the main belt", b.name);
                    assert_eq!(b.tail, Tail::None);
                }
                Class::Comet => {
                    assert!(b.tail == Tail::Ion, "{} is a comet with no ion tail", b.name);
                    assert!(arc.e > 0.4, "{} is a comet on a near-circular orbit", b.name);
                    // In the window, or the tool would have dropped it.
                    assert!(
                        arc.next_perihelion(WINDOW.0) < WINDOW.1,
                        "{} has no perihelion inside the window",
                        b.name
                    );
                }
                Class::Asteroid => assert!(b.radius < 0.3, "{} is dwarf-planet sized", b.name),
            }
        }
        // The five the IAU recognises, and no more.
        let dwarfs: Vec<&str> =
            BODIES.iter().filter(|b| b.class == Class::Dwarf).map(|b| b.name).collect();
        assert_eq!(dwarfs, ["Pluto", "Ceres", "Eris", "Haumea", "Makemake"]);
    }

    /// A comet's tails are a function of where it is, and the whole point of
    /// the model is that they switch on near the Sun and are gone far from it.
    #[test]
    fn comets_are_active_at_perihelion_and_dead_at_aphelion() {
        for b in BODIES.iter().filter(|b| b.tail != Tail::None) {
            let peri = b.next_perihelion(WINDOW.0);
            let arc = b.arc(peri);
            assert!(peri < WINDOW.1, "{}'s next perihelion is outside the window", b.name);
            // Not a fixed floor: a comet with a 1.6 AU perihelion is genuinely
            // a fainter thing than one that dives to 0.12 AU, and the model
            // says so. What has to hold is that perihelion is where it peaks —
            // checked against the whole orbit, not asserted.
            let at_peri = b.activity(peri);
            assert!(at_peri > 0.05, "{} is only {at_peri:.3} active at perihelion", b.name);
            for k in 1..40 {
                let t = peri + arc.period_d() * k as f64 / 40.0;
                assert!(
                    b.activity(t) <= at_peri + 1e-9,
                    "{} is more active away from perihelion than at it",
                    b.name
                );
            }
            // The comets that dive inside the Earth's orbit are the ones that
            // put on a show, and the model has to agree.
            if arc.q() < 0.6 {
                assert!(
                    at_peri > 0.5,
                    "{} dives to {:.2} AU and barely lights up",
                    b.name,
                    arc.q()
                );
            }

            // Half a period on is aphelion, and there it must be off.
            let at_aph = b.activity(peri + arc.period_d() * 0.5);
            assert!(
                at_aph < 0.02 * at_peri,
                "{} is still {at_aph:.3} active at its aphelion of {:.2} AU",
                b.name,
                arc.aphelion()
            );
            // Every comet here goes out past the sublimation line and switches
            // off completely — the shortest-period one, Encke, still reaches
            // 4.1 AU. Phaethon does not: its aphelion is 2.4 AU, well inside
            // the line for water ice, and it is only quiet out there because it
            // is a rock with a far closer threshold of its own.
            let quiet = if b.tail == Tail::Dust { ROCK_ACTIVE_AU } else { ACTIVE_AU };
            assert!(
                arc.aphelion() > quiet,
                "{} never gets far enough out to go quiet: aphelion {:.2} AU against {quiet} AU",
                b.name,
                arc.aphelion()
            );
        }
        // ...and nothing else has anything to switch on.
        for b in BODIES.iter().filter(|b| b.tail == Tail::None) {
            assert_eq!(b.activity(WINDOW.0), 0.0, "{} has activity without a tail", b.name);
        }
        // Phaethon is the one that separates the two thresholds: at 1 AU an
        // actual comet is busy and a rock is not.
        let rock = find("Phaethon").unwrap();
        let comet = find("Halley").unwrap();
        assert_eq!(rock.tail, Tail::Dust);
        assert_eq!(rock.active_au(), ROCK_ACTIVE_AU);
        assert_eq!(comet.active_au(), ACTIVE_AU);
    }

    /// Perihelion passages have to be the ones the ephemeris actually shows,
    /// not merely self-consistent arithmetic on the mean anomaly. Solved from
    /// the elements, checked against the distance the model itself reports.
    #[test]
    fn the_next_perihelion_is_where_the_body_is_closest() {
        for b in BODIES {
            let t = b.next_perihelion(WINDOW.0);
            // Solved through the arc in force at the answer, which is what
            // `SmallBody::next_perihelion` is for — the arc in force at the
            // *question* is several elements out of date by then.
            let arc = b.arc(t);
            assert!(t >= WINDOW.0, "{}'s next perihelion is in the past", b.name);
            let r = b.distance_au(t);
            assert!(
                (r / arc.q() - 1.0).abs() < 0.02,
                "{}: at its perihelion it is {r:.4} AU out, but q is {:.4} AU",
                b.name,
                arc.q()
            );
            // ...and a quarter period either side it is further away.
            for dt in [-0.25, 0.25] {
                assert!(
                    b.distance_au(t + dt * arc.period_d()) > r,
                    "{} is not at its closest at its own perihelion",
                    b.name
                );
            }
        }
    }

    /// Precession is what puts these bodies in the same frame as the Earth. If
    /// it were dropped, every one of them would sit a third of a degree off —
    /// larger than the fit error the table works so hard for.
    #[test]
    fn the_frame_matches_the_planets() {
        let jd = ephem::julian_day(1_784_937_600.0);
        let pluto = find("Pluto").unwrap();
        let of_date = pluto.heliocentric(jd);
        let j2000 = pluto.position_j2000(jd) * AU;
        let moved = angle_deg(of_date, j2000);
        assert!((0.3..0.45).contains(&moved), "precession moved Pluto {moved}°");
        // Same magnitude, only rotated.
        assert!((of_date.len() / j2000.len() - 1.0).abs() < 1e-12);
    }

    /// The ion tail's whole claim to realism is that it points along the wind
    /// the comet meets rather than straight away from the Sun. That difference
    /// is a few degrees, it is the reason a photographed ion tail is not quite
    /// radial, and it is exactly the sort of detail that gets written and then
    /// silently multiplied by zero.
    #[test]
    fn an_ion_tail_lags_the_anti_solar_line_by_the_aberration_angle() {
        let halley = find("Halley").unwrap();
        let peri = halley.next_perihelion(WINDOW.0);
        let t = halley.tails(peri).expect("Halley has tails at perihelion");
        let sunward = halley.heliocentric(peri).normalize();

        // Off the radial line, but only just: at Halley's perihelion speed of
        // 54 km/s against a 400 km/s wind the geometry gives a few degrees.
        let ab = t.aberration_deg(sunward);
        assert!((1.0..12.0).contains(&ab), "aberration is {ab}°, which is not a few degrees");
        // ...and it lags — the tail is swung backwards along the track, not
        // forwards. Getting this sign wrong would point every tail the wrong
        // way round its orbit and still look plausible in a still frame.
        let v = halley.velocity(peri).normalize();
        assert!(t.ion.dot(v) < 0.0, "the ion tail leads the comet instead of trailing it");

        // The dust curves away from the ion tail, back along the orbit — so it
        // is perpendicular to the ion direction and on the trailing side.
        assert!(t.lag.dot(t.ion).abs() < 1e-9, "the dust bend is not perpendicular to the tail");
        assert!(t.lag.dot(v) < 0.0, "the dust curves forwards along the orbit");
        // Ion tail longer than the dust tail, and both far longer than the coma.
        assert!(t.ion_gm > t.dust_gm && t.dust_gm > t.coma_gm * 4.0, "{t:?}");
    }

    /// The tails have to grow and die with the comet's distance, or the model
    /// is a decoration that happens to sit near a comet.
    #[test]
    fn tails_are_longest_at_perihelion_and_absent_far_out() {
        let halley = find("Halley").unwrap();
        let peri = halley.next_perihelion(WINDOW.0);
        let at_peri = halley.tails(peri).unwrap();
        // A year before perihelion Halley is still out past 3 AU, with nothing.
        assert!(halley.tails(peri - 365.0).is_none(), "Halley has a tail a year out");
        // Coming in, it grows monotonically.
        let mut last = 0.0;
        for d in [-120.0, -90.0, -60.0, -30.0, 0.0] {
            let len = halley.tails(peri + d).map_or(0.0, |t| t.ion_gm);
            assert!(len >= last, "the tail shrank on the way in at {d} days");
            last = len;
        }
        assert!((25.0..35.0).contains(&at_peri.ion_gm), "{} Gm of ion tail", at_peri.ion_gm);

        // Phaethon is the case the two thresholds exist for: at its perihelion
        // it sheds dust and nothing else, and it is inert for the rest of its
        // orbit, unlike a comet of the same perihelion distance.
        let rock = find("Phaethon").unwrap();
        let rp = rock.next_perihelion(WINDOW.0);
        let dust = rock.tails(rp).expect("Phaethon sheds dust at perihelion");
        assert_eq!(dust.ion_gm, 0.0, "a rock has no ions to lose");
        assert!(dust.dust_gm > 0.0);
        // A tenth of an AU out it is already done, where a comet would be at
        // its most active.
        assert!(rock.tails(rp + 20.0).is_none(), "Phaethon is still shedding 20 days out");
        // ...and nothing without a tail ever grows one.
        for b in BODIES.iter().filter(|b| b.tail == Tail::None) {
            assert!(b.tails(b.next_perihelion(WINDOW.0)).is_none(), "{} grew a tail", b.name);
        }
    }

    /// The drawn orbit is one arc's ellipse, so it has to close, lie in the
    /// body's own orbit plane, and span the right distances.
    #[test]
    fn the_drawn_orbit_is_a_closed_ellipse_of_the_right_size() {
        for b in BODIES {
            let jd = WINDOW.0;
            let arc = b.arc(jd);
            let path: Vec<Vec3> = b.orbit_path(jd, 256).collect();
            assert_eq!(path.len(), 257);
            // Closed: a full turn of the eccentric anomaly comes back.
            assert!((path[0] - path[256]).len() < 1e-9 * AU, "{}'s orbit does not close", b.name);
            // ...and it starts at perihelion, which is where E = 0 is.
            assert!(
                (path[0].len() / AU / arc.q() - 1.0).abs() < 1e-9,
                "{}: the path does not begin at perihelion",
                b.name
            );
            let lo = path.iter().map(|p| p.len()).fold(f64::MAX, f64::min) / AU;
            let hi = path.iter().map(|p| p.len()).fold(0.0, f64::max) / AU;
            assert!((lo / arc.q() - 1.0).abs() < 0.01, "{}: path perihelion {lo}", b.name);
            assert!((hi / arc.aphelion() - 1.0).abs() < 0.01, "{}: path aphelion {hi}", b.name);
            // The body sits on its own drawn path, which is the thing anybody
            // would notice if it did not. Measured against the spacing of the
            // ring's own samples, since a point between two of them is at worst
            // half a step from either — comparing against the orbit's size
            // instead would flatter a wide orbit and fail a tight one for
            // nothing but arithmetic.
            let here = b.heliocentric(jd);
            let closest = path.iter().map(|p| (*p - here).len()).fold(f64::MAX, f64::min);
            let step = path.windows(2).map(|w| (w[1] - w[0]).len()).fold(0.0, f64::max);
            assert!(
                closest < step,
                "{} sits {:.4} AU off its own ring, which is drawn in {:.4} AU steps",
                b.name,
                closest / AU,
                step / AU
            );
        }
    }

    /// The search is how asteroids are found at all — there is no chip for
    /// them — so it has to match every way somebody would type one.
    #[test]
    fn the_search_finds_bodies_by_name_number_and_designation() {
        let hit = |q: &str| search(q).map(|(_, b)| b.name).collect::<Vec<_>>();
        assert_eq!(hit("apophis"), ["Apophis"]);
        assert_eq!(hit("99942"), ["Apophis"]);
        assert_eq!(hit("halley"), ["Halley"]);
        assert_eq!(hit("2024 YR4"), ["2024 YR4"]);
        // A plain substring, deliberately — the same rule the satellite search
        // uses, and the same consequence: `1P` is inside `21P` and `81P` too.
        // Narrowing it is the searcher's job, and the name always does.
        assert_eq!(hit("1P"), ["Halley", "Giacobini-Zinner", "Wild 2"]);
        assert_eq!(hit("1P/H"), ["Halley"]);
        // A provisional designation finds an unnamed body, which is the only
        // handle it has.
        assert!(BODIES.iter().any(|b| b.matches("2005 YU55")));
        // Partial words work, and case does not matter.
        assert!(find("pluto").is_some());
        assert!(hit("tempel").len() >= 2, "9P and 55P both contain Tempel");
        // An empty query matches nothing rather than everything: it drives a
        // highlight, and highlighting all forty is the same as highlighting none.
        assert!(hit("").is_empty() && hit("   ").is_empty());
    }

    /// The close-approach bodies are in the table *because* they come close, so
    /// the model had better show them doing it.
    #[test]
    fn the_close_approach_bodies_really_do_come_close() {
        for (name, year) in [("Apophis", 2029.0), ("153814", 2028.0), ("137108", 2027.0)] {
            let b = find(name).unwrap_or_else(|| panic!("{name} is not in the table"));
            let start = WINDOW.0 + (year - 2026.0) * 365.25;
            let mut closest = f64::MAX;
            for k in 0..3660 {
                let jd = start + k as f64 * 0.2;
                let d = (b.heliocentric(jd) - ephem::earth_heliocentric(jd)).len();
                closest = closest.min(d);
            }
            // Within a fiftieth of an AU of the Earth at some point that year —
            // the CAD query's own threshold, which is what put it in the table.
            assert!(
                closest < 0.02 * AU,
                "{name} never gets closer than {:.4} AU in {year}",
                closest / AU
            );
        }
    }
}
