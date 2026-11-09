//! Obstacle attenuation — a coarse, **labelled** model, not a terrain solver.
//!
//! A stand of trees or a ridge between the antenna and the sky attenuates the
//! far field in its direction: `|E′| = |E|·10^(−A/20)`, i.e. subtract `A` dB
//! from the gain on that ray. The two models are the ITU-R P.833 vegetation
//! family (Weissberger / COST-235) and the knife-edge Fresnel diffraction.
//! There is no terrain or DEM here, so it is an estimate, not a measurement.

/// One obstacle between the antenna and the sky. Azimuth is the bearing from
/// the antenna, width the angular spread it blocks.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Obstacle {
    /// A stand of trees `depth_m` deep at `distance_m`, blocking `width_deg`.
    Vegetation { az_deg: f64, width_deg: f64, distance_m: f64, depth_m: f64, in_leaf: bool },
    /// A knife-edge (ridge) `height_m` above the line of sight, at `distance_m`.
    KnifeEdge { az_deg: f64, width_deg: f64, distance_m: f64, height_m: f64 },
}

impl Obstacle {
    pub fn azimuth(&self) -> f64 {
        match self {
            Obstacle::Vegetation { az_deg, .. } | Obstacle::KnifeEdge { az_deg, .. } => *az_deg,
        }
    }

    pub fn width(&self) -> f64 {
        match self {
            Obstacle::Vegetation { width_deg, .. } | Obstacle::KnifeEdge { width_deg, .. } => {
                *width_deg
            }
        }
    }
}

/// Vegetation attenuation in dB (ITU-R P.833 family): Weissberger for a path
/// 14 m and longer, COST-235 for a shorter one.
pub fn vegetation_db(freq_hz: f64, depth_m: f64, in_leaf: bool) -> f64 {
    if depth_m >= 14.0 {
        let f_ghz = freq_hz / 1e9;
        1.33 * f_ghz.powf(0.284) * depth_m.powf(0.588)
    } else if in_leaf {
        let f_mhz = freq_hz / 1e6;
        15.6 * f_mhz.powf(-0.009) * depth_m.powf(0.26)
    } else {
        let f_mhz = freq_hz / 1e6;
        26.6 * f_mhz.powf(-0.2) * depth_m.powf(0.5)
    }
}

/// Knife-edge diffraction loss in dB (Fresnel). `height_m` is the edge's height
/// above the line of sight, `distance_m` its distance from the antenna, `lambda`
/// the wavelength. A far-field target is at infinity, so only `d1` enters `v`.
pub fn knife_edge_db(height_m: f64, distance_m: f64, lambda: f64) -> f64 {
    let v = height_m * (2.0 / (lambda * distance_m)).sqrt();
    if v <= -0.7 { 0.0 } else { 6.9 + 20.0 * (((v - 0.1).powi(2) + 1.0).sqrt() + v - 0.1).log10() }
}

/// Whether a ray at `az` falls inside an obstacle at `az_o` of `width` degrees
/// (wrapping at 0/360).
fn angular_within(az: f64, az_o: f64, width: f64) -> bool {
    let d = (az - az_o).rem_euclid(360.0);
    d.min(360.0 - d) <= width / 2.0
}

/// The total attenuation in dB for a ray at `az_deg` over the obstacles.
pub fn attenuation_db(obstacles: &[Obstacle], az_deg: f64, freq_hz: f64) -> f64 {
    let lambda = crate::wavelength_m(freq_hz);
    obstacles
        .iter()
        .map(|o| {
            if !angular_within(az_deg, o.azimuth(), o.width()) {
                return 0.0;
            }
            match o {
                Obstacle::Vegetation { depth_m, in_leaf, .. } => {
                    vegetation_db(freq_hz, *depth_m, *in_leaf)
                }
                Obstacle::KnifeEdge { distance_m, height_m, .. } => {
                    knife_edge_db(*height_m, *distance_m, lambda)
                }
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Vegetation loss grows with depth within each branch (the COST-235 and
    /// Weissberger models do not join smoothly at 14 m).
    #[test]
    fn vegetation_loss_grows_with_depth() {
        let f = 27e6;
        let shallow = vegetation_db(f, 2.0, true);
        let deeper = vegetation_db(f, 10.0, true);
        assert!(
            deeper > shallow,
            "COST-235: deeper trees must lose more: {shallow:.1} vs {deeper:.1}"
        );
        let deep = vegetation_db(f, 50.0, true);
        let deeper_still = vegetation_db(f, 200.0, true);
        assert!(
            deeper_still > deep,
            "Weissberger: deeper trees must lose more: {deep:.1} vs {deeper_still:.1}"
        );
    }

    /// Knife-edge loss is 0 below the first Fresnel zone and grows with height.
    #[test]
    fn knife_edge_loss_grows_with_height() {
        let lam = crate::wavelength_m(27e6);
        assert_eq!(knife_edge_db(-20.0, 100.0, lam), 0.0, "clear first Fresnel zone");
        let low = knife_edge_db(5.0, 100.0, lam);
        let high = knife_edge_db(50.0, 100.0, lam);
        assert!(high > low, "a higher ridge must diffract more: {low:.1} vs {high:.1}");
    }

    /// An obstacle attenuates its own azimuth but not a far-off one.
    #[test]
    fn attenuation_is_directional() {
        let trees = Obstacle::Vegetation {
            az_deg: 90.0,
            width_deg: 30.0,
            distance_m: 50.0,
            depth_m: 40.0,
            in_leaf: true,
        };
        let blocked = attenuation_db(&[trees], 90.0, 27e6);
        let clear = attenuation_db(&[trees], 0.0, 27e6);
        assert!(blocked > 0.0, "the ray through the trees is attenuated");
        assert_eq!(clear, 0.0, "a ray away from the trees is not");
    }
}
