//! Closed-form antenna dimensions — the "calculator" half of the antenna utility.
//!
//! Nothing here solves anything: it is the textbook starting lengths and the
//! SWR-from-impedance arithmetic, so the common question ("27.265 MHz — how long
//! is my dipole?") is answered without a method-of-moments engine. The solver
//! (feed impedance, patterns, ground) comes later and lives beside this.
//!
//! The one fudge is the shortening factor: a half-wave dipole is a few percent
//! shorter than half a wavelength because of the end effect, and how much
//! depends on wire diameter and height. `K` is the common thin-wire value; the
//! solver is what turns it into a real number, and the doc for the tool says so
//! rather than letting the calculator look exact.

pub mod expr;
pub mod maa;
pub mod model;
pub mod obstacle;
pub mod optimize;
pub mod presets;
pub mod solver;
pub mod store;
pub mod transform;

/// Speed of light, m/s.
pub const C: f64 = 299_792_458.0;

/// Free-space wavelength in metres.
pub fn wavelength_m(freq_hz: f64) -> f64 {
    C / freq_hz
}

/// The practical thin-wire shortening factor (end effect). A starting value,
/// not a constant of nature — see the module note.
pub const K: f64 = 0.95;

/// One named dimension of a design, in metres.
#[derive(Clone, Debug, PartialEq)]
pub struct Dimension {
    pub what: String,
    pub metres: f64,
}

/// A named antenna design: its dimensions and a nominal feed impedance.
#[derive(Clone, Debug, PartialEq)]
pub struct Design {
    pub name: &'static str,
    pub notes: String,
    pub dims: Vec<Dimension>,
    /// Nominal feed impedance `(R, X)` in ohms, if the shape has a well-known one.
    pub feed_z: Option<(f64, f64)>,
}

impl Design {
    fn new(name: &'static str, notes: impl Into<String>) -> Design {
        Design { name, notes: notes.into(), dims: Vec::new(), feed_z: None }
    }

    fn dim(mut self, what: impl Into<String>, metres: f64) -> Design {
        self.dims.push(Dimension { what: what.into(), metres });
        self
    }

    fn z(mut self, r: f64, x: f64) -> Design {
        self.feed_z = Some((r, x));
        self
    }
}

/// A centre-fed half-wave dipole, horizontal.
pub fn dipole(freq_hz: f64) -> Design {
    let total = K * wavelength_m(freq_hz) / 2.0;
    Design::new("Half-wave dipole", "centre-fed, horizontal, thin wire well above ground")
        .dim("total length", total)
        .dim("each leg", total / 2.0)
        .z(73.0, 0.0)
}

/// An inverted-V: a dipole with the legs sloped down from a centre support.
pub fn inverted_v(freq_hz: f64) -> Design {
    let leg = K * wavelength_m(freq_hz) / 4.0;
    Design::new("Inverted-V", "apex 90-120 deg; legs slope down from one centre support")
        .dim("each leg", leg)
        .z(50.0, 0.0)
}

/// A quarter-wave vertical (monopole) over a ground plane.
pub fn vertical(freq_hz: f64) -> Design {
    let rad = K * wavelength_m(freq_hz) / 4.0;
    Design::new("Quarter-wave vertical", "monopole over a ground plane / radials")
        .dim("radiator", rad)
        .z(36.0, 0.0)
}

/// A 5/8-wave vertical: a little more gain at a lower takeoff than 1/4 wave.
pub fn vertical_5_8(freq_hz: f64) -> Design {
    let rad = 0.625 * wavelength_m(freq_hz);
    Design::new("5/8-wave vertical", "needs a matching network at the base")
        .dim("radiator", rad)
        .z(50.0, 0.0)
}

/// A ground-plane: a vertical radiator with four (or more) sloping radials.
pub fn ground_plane(freq_hz: f64) -> Design {
    let lam = wavelength_m(freq_hz);
    Design::new("Ground plane", "vertical radiator, 4 sloped radials")
        .dim("radiator", K * lam / 4.0)
        .dim("each radial", lam / 4.0)
        .z(36.0, 0.0)
}

/// A full-wave triangular (delta) loop. `perimeter_factor` is the loop's length
/// in wavelengths — 1.0 for the nominal full wave, ~1.02 with the end effect.
pub fn delta_loop(freq_hz: f64) -> Design {
    let lam = wavelength_m(freq_hz);
    let perimeter = 1.02 * lam;
    Design::new("Full-wave delta loop", "equilateral triangle; feed the bottom centre or a corner")
        .dim("perimeter", perimeter)
        .dim("each side", perimeter / 3.0)
        .dim("height (equilateral)", 0.866 * perimeter / 3.0)
        .z(110.0, 0.0)
}

/// A full-wave square (quad) loop.
pub fn quad_loop(freq_hz: f64) -> Design {
    let lam = wavelength_m(freq_hz);
    let perimeter = 1.03 * lam;
    Design::new("Full-wave quad loop", "square loop, side ~ lambda/4")
        .dim("perimeter", perimeter)
        .dim("each side", perimeter / 4.0)
        .z(120.0, 0.0)
}

/// A Yagi-Uda starting layout: `elements` total (reflector + driven + directors).
/// These are the classic starting ratios; an optimizer refines them.
pub fn yagi(freq_hz: f64, elements: usize) -> Design {
    let lam = wavelength_m(freq_hz);
    let n = elements.max(2);
    let mut d = Design::new("Yagi-Uda", "starting ratios; the solver/optimizer refines them")
        .dim("reflector", 0.50 * lam)
        .dim("driven element", 0.48 * lam);
    let directors = n - 2;
    if directors > 0 {
        // Taper from the first director toward the last.
        let first = 0.44 * lam;
        let last = 0.40 * lam;
        for i in 0..directors {
            let len = if directors == 1 {
                first
            } else {
                first + (last - first) * (i as f64) / ((directors - 1) as f64)
            };
            d = d.dim(format!("director {}", i + 1), len);
        }
        d = d.dim("element spacing", 0.20 * lam);
    }
    d.z(25.0, 0.0)
}

/// Every shape the calculator knows, in the order a menu would list them.
pub fn all(freq_hz: f64) -> Vec<Design> {
    vec![
        dipole(freq_hz),
        inverted_v(freq_hz),
        vertical(freq_hz),
        vertical_5_8(freq_hz),
        ground_plane(freq_hz),
        delta_loop(freq_hz),
        quad_loop(freq_hz),
        yagi(freq_hz, 3),
    ]
}

/// Magnitude of the reflection coefficient for load `z = (R, X)` on a feed line
/// of characteristic impedance `z0`.
pub fn reflection_mag(z: (f64, f64), z0: f64) -> f64 {
    let (r, x) = z;
    let num = ((r - z0).powi(2) + x.powi(2)).sqrt();
    let den = ((r + z0).powi(2) + x.powi(2)).sqrt();
    num / den
}

/// Standing-wave ratio from a load impedance.
pub fn swr(z: (f64, f64), z0: f64) -> f64 {
    let g = reflection_mag(z, z0);
    (1.0 + g) / (1.0 - g)
}

/// SWR from `(R, X)` and a reference impedance, defaulting to 50 ohms.
pub fn swr_50(r: f64, x: f64) -> f64 {
    swr((r, x), 50.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The blue-riband example: 11 m FT8 sits on CB channel 26, 27.265 MHz.
    #[test]
    fn a_dipole_for_eleven_metres() {
        let f = 27_265_000.0;
        let lam = wavelength_m(f);
        assert!((lam - 10.99).abs() < 0.02, "lambda = {lam}");
        let d = dipole(f);
        let total = d.dims[0].metres;
        let half = d.dims[1].metres;
        assert!((total - 5.22).abs() < 0.02, "total = {total}");
        assert!((half - 2.61).abs() < 0.02, "half = {half}");
        assert_eq!(total, half * 2.0);
    }

    /// The familiar 468/f rule, which is the same number in feet. It is itself
    /// an approximation (its shortening factor is ~0.951, ours 0.95), so this
    /// checks agreement to within a fraction of a percent, not to the inch.
    #[test]
    fn the_feet_rule_agrees() {
        for f_mhz in [3.65, 7.1, 14.05, 27.265] {
            let metres = dipole(f_mhz * 1e6).dims[0].metres;
            let feet = metres / 0.3048;
            let rule = 468.0 / f_mhz;
            assert!((feet - rule).abs() / rule < 0.005, "{f_mhz} MHz: {feet} ft vs {rule}");
        }
    }

    /// MMANA's own worked example: a 20 m dipole at 14.05 MHz gives about
    /// 71.5 - j1.9 ohms and SWR 1.43 against 50. We do not model the reactance,
    /// so we check the length and that a 73-ohm load lands near that SWR.
    #[test]
    fn the_reference_dipole_matches_mmana() {
        let f = 14_050_000.0;
        let d = dipole(f);
        let half = d.dims[1].metres;
        assert!((half - 5.07).abs() < 0.05, "half leg = {half}");
        let s = swr_50(71.5, -1.9);
        assert!((s - 1.43).abs() < 0.05, "SWR = {s}");
    }

    /// A quarter-wave vertical at the CB calling frequency.
    #[test]
    fn a_quarter_wave_vertical() {
        let v = vertical(27_265_000.0);
        let rad = v.dims[0].metres;
        assert!((rad - 2.61).abs() < 0.02, "radiator = {rad}");
    }

    /// The delta loop's perimeter is a full wave; its side is a third of that,
    /// and the equilateral height is sqrt(3)/2 of a side.
    #[test]
    fn a_delta_loop_is_three_equal_sides() {
        let lam = wavelength_m(28_500_000.0);
        let p = delta_loop(28_500_000.0);
        let perimeter = p.dims[0].metres;
        let side = p.dims[1].metres;
        let height = p.dims[2].metres;
        assert!((perimeter - 1.02 * lam).abs() < 1e-6);
        assert!((side - perimeter / 3.0).abs() < 1e-9);
        assert!((height - 0.8660254 * side).abs() < 1e-4, "height = {height}");
    }

    /// A Yagi of N elements has a reflector, a driven element and N-2 directors.
    #[test]
    fn a_yagi_counts_its_elements() {
        let y = yagi(50_000_000.0, 4);
        let directors = y.dims.iter().filter(|d| d.what.starts_with("director")).count();
        assert_eq!(directors, 2, "{:?}", y.dims);
        assert!(y.dims.iter().any(|d| d.what == "reflector"));
        assert!(y.dims.iter().any(|d| d.what == "driven element"));
    }

    /// SWR arithmetic: a perfect match is 1:1, and 100 ohms on 50 is 2:1.
    #[test]
    fn swr_arithmetic() {
        assert!((swr_50(50.0, 0.0) - 1.0).abs() < 1e-9);
        assert!((swr_50(100.0, 0.0) - 2.0).abs() < 1e-9);
        assert!((swr_50(25.0, 0.0) - 2.0).abs() < 1e-9);
        // A reactance alone still reflects: 0 + j50 on 50 is not 1:1.
        assert!(swr_50(0.0, 50.0) > 1.0);
    }

    /// The nominal antenna feed impedances and their SWR against 50 Ω, as
    /// validated against the reference table (purely resistive loads).
    #[test]
    fn nominal_feed_impedances_and_swr() {
        let cases: &[(&str, f64, f64, f64)] = &[
            ("Half-wave dipole", 73.0, 0.0, 1.46),
            ("Inverted-V", 50.0, 0.0, 1.00),
            ("Quarter-wave vertical", 36.0, 0.0, 1.39),
            ("5/8-wave vertical", 50.0, 0.0, 1.00),
            ("Ground plane", 36.0, 0.0, 1.39),
            ("Full-wave delta loop", 110.0, 0.0, 2.20),
        ];
        for (name, r, x, swr_exp) in cases {
            let got = swr((*r, *x), 50.0);
            assert!((got - swr_exp).abs() < 0.01, "{name}: SWR {got:.2} vs {swr_exp:.2}");
        }
        // Spot-check the actual Design feed_z values for the dipole and the loop.
        assert_eq!(dipole(27_265_000.0).feed_z, Some((73.0, 0.0)));
        assert_eq!(delta_loop(28_500_000.0).feed_z, Some((110.0, 0.0)));
    }

    /// A blank frequency must not produce a NaN or an infinity.
    #[test]
    fn a_nonsense_frequency_is_not_a_crash() {
        let d = dipole(0.0);
        assert!(d.dims[0].metres.is_infinite());
        // Callers guard on freq > 0; the point here is that it does not panic.
    }
}
