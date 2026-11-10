//! Per-region noise-floor estimation for anything that scans an STFT for
//! signals: the CW and PSK/RTTY skimmers, and the ISM burst gate.
//!
//! They all want the same thing from their transform: a per-bin estimate of the
//! *noise* power, robust to the signals sitting in it. The estimator is the
//! median of each ~64-bin region, scaled to a mean. Signals are narrow and
//! sparse, so a region's median bin is a noise bin however strong or persistent
//! the signals in that region are — which is what stops a carrier inflating the
//! very floor that is supposed to reveal it.
//!
//! It lives here rather than in one of the callers because the second caller
//! would otherwise have copied it, and a copy of this particular function is
//! how the positive-feedback bug it exists to prevent comes back.

/// |FFT|² of Gaussian noise is exponential, whose median is `ln 2`·mean; scale
/// the region median back up to estimate the mean noise the thresholds expect.
///
/// The factor is `1 / ln 2`, which is `log₂ e` — hence the constant, rather
/// than the rounded literal the two skimmers each used to carry.
pub const MEDIAN_TO_MEAN: f32 = std::f32::consts::LOG2_E;

/// The smoothing coefficient that reproduces a per-frame `alpha` when the floor
/// is only updated once every `every` frames.
///
/// `1 - alpha` is the fraction of the old estimate that survives one frame, so
/// `(1 - alpha)^every` is what survives the whole gap. Solving for the
/// coefficient that decays that far in a single step leaves the floor's time
/// constant *in seconds* exactly where it was — only the number of times it is
/// recomputed changes.
pub fn stride_alpha(alpha: f32, every: u32) -> f32 {
    1.0 - (1.0 - alpha).powi(every as i32)
}

/// Re-estimate `floor` from `power`, one region of `region_bins` at a time.
///
/// `seed` writes each estimate straight in rather than easing towards it, for
/// the first frame on a fresh band where there is no history worth keeping.
///
/// The median runs over the samples' *bit patterns* rather than the floats.
/// Power is `norm_sqr`, hence non-negative, and non-negative IEEE-754 floats
/// order identically to their bit patterns read as unsigned integers — so this
/// selects exactly the same element while getting an `Ord` selection with no
/// comparator closure and no `partial_cmp` NaN fallback, on a path that runs
/// tens of thousands of times a second.
pub fn update_regions(
    floor: &mut [f32],
    power: &[f32],
    scratch: &mut Vec<u32>,
    region_bins: usize,
    alpha: f32,
    seed: bool,
) {
    let n = floor.len().min(power.len());
    for base in (0..n).step_by(region_bins) {
        let end = (base + region_bins).min(n);
        scratch.clear();
        scratch.extend(power[base..end].iter().map(|p| p.to_bits()));
        let mid = scratch.len() / 2;
        scratch.select_nth_unstable(mid);
        let est = f32::from_bits(scratch[mid]) * MEDIAN_TO_MEAN;
        for f in &mut floor[base..end] {
            *f = if seed { est } else { *f + alpha * (est - *f) };
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The bit-pattern selection has to agree with a plain float median for the
    /// values this actually sees — including the denormals and zeros a quiet
    /// bin produces.
    #[test]
    fn bit_pattern_median_matches_a_float_median() {
        let cases: Vec<Vec<f32>> = vec![
            (0..64).map(|i| i as f32 * 1e-3).collect(),
            (0..64).map(|i| (63 - i) as f32).collect(),
            vec![0.0; 64],
            (0..64).map(|i| if i % 2 == 0 { 0.0 } else { f32::MIN_POSITIVE }).collect(),
            (0..64).map(|i| ((i * 37) % 64) as f32 * 2.5e-9).collect(),
            // One very strong signal in an otherwise quiet region: the case the
            // whole median approach exists for.
            (0..64).map(|i| if i == 31 { 1e6 } else { 1e-6 }).collect(),
        ];
        for power in cases {
            let mut floor = vec![0.0; power.len()];
            let mut scratch = Vec::new();
            update_regions(&mut floor, &power, &mut scratch, 64, 1.0, true);

            let mut sorted = power.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let want = sorted[power.len() / 2] * MEDIAN_TO_MEAN;
            assert_eq!(floor[0], want, "power {power:?}");
        }
    }

    /// A strided update must land on the same floor as the per-frame one it
    /// replaces, given the same estimate repeated over the gap.
    #[test]
    fn stride_alpha_preserves_the_time_constant() {
        for (alpha, every) in [(0.1f32, 16u32), (0.05, 4), (0.3, 16), (0.3, 4)] {
            let est = 7.0f32;
            let mut per_frame = 1.0f32;
            for _ in 0..every {
                per_frame += alpha * (est - per_frame);
            }
            let strided = 1.0 + stride_alpha(alpha, every) * (est - 1.0);
            assert!(
                (per_frame - strided).abs() < 1e-5,
                "alpha {alpha} every {every}: {per_frame} vs {strided}"
            );
        }
    }

    /// What the floor estimate costs per second of skimming, at each skimmer's
    /// frame rate. This runs on every frame of every enabled skimmer, so it is
    /// paid whether or not anything is being decoded.
    #[test]
    #[ignore = "a measurement, not an assertion; needs --release to mean anything"]
    fn floor_cost_per_second_of_skimming() {
        use std::time::Instant;

        // Exponentially distributed power, as |FFT|² of noise actually is.
        let mut seed = 0x2545_F491_4F6C_DD1Du64;
        let power: Vec<f32> = (0..4096)
            .map(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                let u = (seed >> 11) as f64 / (1u64 << 53) as f64;
                -(1.0 - u).ln() as f32 * 1e-6
            })
            .collect();

        println!("{:>18}{:>12}{:>12}{:>16}", "skimmer", "frames/s", "ms/s", "% of one core");
        // (label, frames/s at ~202 kHz skim rate, floor updates per frame)
        for (label, fps, every) in [("cw", 198.0, 4u32), ("psk/rtty", 791.0, 16)] {
            let mut floor = vec![0.0f32; power.len()];
            let mut scratch = Vec::new();
            let updates = (fps / every as f64) as usize;

            let t = Instant::now();
            for _ in 0..updates {
                update_regions(&mut floor, &power, &mut scratch, 64, 0.5, false);
            }
            let ms = t.elapsed().as_secs_f64() * 1e3;
            println!("{label:>18}{fps:>12.0}{ms:>12.3}{:>16.3}", ms / 10.0);
        }
    }
}
