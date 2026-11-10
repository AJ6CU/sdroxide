//! What *kind* of signal a burst is, without decoding it.
//!
//! # Why this is worth having on its own
//!
//! A decoder answers "what did it say". Most of the traffic on this band will
//! never have a decoder here, and for all of it the useful questions are much
//! smaller: is that a data burst or a chirp, is it keyed on and off or shifted in
//! frequency, how wide is it, how fast. Those can all be answered from the burst
//! itself with no idea what protocol it belongs to — and answering them is what
//! turns "440 bursts, 37 decoded" from a dead end into a list of things to go and
//! look at.
//!
//! # What each class is decided by
//!
//! Every one of the three measurements below is taken over the *signal* region of
//! the burst, never the gate's padding — see [`crate::demod::Stats`].
//!
//! They are asked in this order, and the order is load-bearing — see the comment
//! in [`classify`].
//!
//! 1. **[`BurstClass::Ook`]** — the envelope falls to the channel's own noise
//!    floor partway through. Not "the envelope varies a lot": at the
//!    signal-to-noise ratios a burst gate passes, noise alone makes an FSK
//!    burst's envelope vary a lot. What distinguishes keying is that the quiet
//!    part is *as quiet as no signal at all*, and that needs the floor to compare
//!    against. Decided on the envelope alone, so it is safe to ask before
//!    anything about frequency has been established.
//! 2. **[`BurstClass::Carrier`]** — too little frequency spread for the tone
//!    measurement below to mean anything: an unmodulated or very narrowly
//!    modulated carrier.
//! 3. **[`BurstClass::Chirp`]** — the instantaneous frequency sweeps rather than
//!    settling on tones, so [`Stats::bimodal`](crate::demod::Stats::bimodal) is
//!    low. Needs no symbol rate, which matters because a chirp has none to
//!    measure and every attempt to measure one returns a meaningless number.
//! 4. **[`BurstClass::Fsk2`]** — two tones, constant envelope. What is left.
//!
//! # Calibration
//!
//! The thresholds are set from signals whose identity is known independently, not
//! from feel: the Bresser, Z-Wave and Homematic traffic this crate already
//! decodes is known-good two-level FSK, and the wideband bursts on the same
//! captures that no symbol-rate estimator can hold on to are the chirps. The
//! tests assert the separation those give rather than the thresholds themselves,
//! so a change that narrows the gap fails even if it keeps the labels.

use sdroxide_types::IsmBurstClass as BurstClass;

use crate::demod::Stats;
use crate::gate::Burst;

/// Below this fraction at the tones, the burst is sweeping rather than shifting.
///
/// A two-level signal scores near 0.9 and a uniform sweep near 0.35, so the
/// decision is being made in the middle of a wide gap rather than on a cliff.
const CHIRP_BIMODAL: f32 = 0.6;

/// How close the quiet part of an on-off keyed burst gets to the noise floor, dB.
///
/// Keying takes the envelope to nothing, so its quiet symbols land within a few
/// dB of where the channel sits with no signal in it at all. Three dB of slack
/// covers the gate's own hangover and the fact that a percentile is not a
/// minimum.
const OOK_FLOOR_MARGIN_DB: f32 = 3.0;

/// The envelope also has to actually swing. Without this a burst whose *whole*
/// envelope sits near the floor — a weak one the gate barely opened on — would
/// be called keyed.
const OOK_MIN_DEPTH: f32 = 0.5;

/// Widest deviation that is a carrier rather than two tones, Hz.
///
/// A percentile spread this small is the noise on an unmodulated carrier, not a
/// tone separation. Deliberately well below the narrowest real deviation in the
/// band — the Fine Offset sensors run about ±25 kHz and the narrowest thing seen
/// here is around 12 kHz.
const CARRIER_DEV_HZ: f32 = 2_000.0;

/// Classify a gated burst.
///
/// `measured_baud` is the symbol rate the burst verified for itself, when it had
/// one — passed in rather than re-derived so that the classifier and the decoders
/// are looking at the same number.
pub fn classify(burst: &Burst, stats: &Stats, measured_baud: Option<(f64, f32)>) -> BurstClass {
    // The channel's noise floor, from the burst's own measurement of itself.
    let floor_dbfs = burst.peak_dbfs - burst.snr_db;

    // The order below is not arbitrary, and getting it wrong is how a plain
    // carrier gets called a chirp: with no frequency spread at all the tone
    // tolerance collapses to nothing, so almost no sample counts as being "at" a
    // tone and the bimodality reads near zero — the same number a sweep gives,
    // for the opposite reason. So each test is applied only where its measurement
    // means something.

    // Keyed on and off. Decided on the envelope alone, which is independent of
    // whatever the frequency is doing, so it is safe to ask first.
    if stats.env_depth >= OOK_MIN_DEPTH && stats.env_low_dbfs <= floor_dbfs + OOK_FLOOR_MARGIN_DB {
        return BurstClass::Ook;
    }

    // Too little frequency spread for the tone measurements below to mean
    // anything.
    if stats.dev_hz <= CARRIER_DEV_HZ {
        return BurstClass::Carrier;
    }

    // Sweeping rather than shifting. Now that there is a spread to measure
    // against, a low score really does mean the frequency never settles.
    if stats.bimodal < CHIRP_BIMODAL {
        return BurstClass::Chirp;
    }

    // Two tones and a constant envelope. A recovered symbol rate is not required
    // to say so — plenty of real FSK arrives too short or too noisy for the rate
    // estimator to verify itself, and it is still plainly FSK.
    let _ = measured_baud;
    BurstClass::Fsk2
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdroxide_dsp::Complex32;

    /// Build a burst from an instantaneous-frequency function and an envelope
    /// function, so a test can state the signal it means rather than a bit
    /// pattern. Deliberately not built with anything under test.
    fn burst(
        n: usize,
        rate: f64,
        freq: impl Fn(usize) -> f32,
        amp: impl Fn(usize) -> f32,
    ) -> (Burst, Stats) {
        let mut iq = Vec::with_capacity(n);
        let mut phase = 0.0f64;
        for i in 0..n {
            phase += f64::from(freq(i)) * std::f64::consts::TAU / rate;
            let a = amp(i);
            iq.push(Complex32::new(a * phase.cos() as f32, a * phase.sin() as f32));
        }
        let mut disc = Vec::new();
        let mut scratch = Vec::new();
        crate::demod::discriminate(&iq, rate, &mut disc);
        let stats = crate::demod::stats(&iq, &disc, &mut scratch);
        let peak = 20.0 * amp(n / 2).max(1e-12).log10();
        (
            Burst { iq, rate_hz: rate, center_hz: 868_300_000.0, snr_db: 20.0, peak_dbfs: peak },
            stats,
        )
    }

    const RATE: f64 = 250_000.0;

    /// Two tones at a constant amplitude is FSK, and has to be called FSK even
    /// though nothing here recovered a symbol rate.
    #[test]
    fn two_tones_at_constant_amplitude_are_fsk() {
        let sps = 25;
        let (b, s) =
            burst(8000, RATE, |i| if (i / sps) % 2 == 0 { 25_000.0 } else { -25_000.0 }, |_| 1.0);
        assert!(s.bimodal > 0.85, "a square frequency wave sits at its tones: {}", s.bimodal);
        assert_eq!(classify(&b, &s, None), BurstClass::Fsk2);
    }

    /// A frequency ramp is a chirp, and must not be called FSK however much it
    /// looks like a wide signal.
    #[test]
    fn a_frequency_sweep_is_a_chirp() {
        let (b, s) =
            burst(8000, RATE, |i| -60_000.0 + 120_000.0 * (i % 2000) as f32 / 2000.0, |_| 1.0);
        assert!(s.bimodal < 0.5, "a sweep spends its time everywhere: {}", s.bimodal);
        assert_eq!(classify(&b, &s, None), BurstClass::Chirp);

        // And the separation from FSK has to be wide, not marginal — this is the
        // property the threshold rests on.
        let (_, fsk) =
            burst(8000, RATE, |i| if (i / 25) % 2 == 0 { 25_000.0 } else { -25_000.0 }, |_| 1.0);
        assert!(
            fsk.bimodal - s.bimodal > 0.35,
            "FSK {} and chirp {} are too close to tell apart",
            fsk.bimodal,
            s.bimodal
        );
    }

    /// An envelope that drops to nothing partway through is keying, not fading.
    #[test]
    fn an_envelope_that_collapses_to_the_floor_is_ook() {
        let sps = 25;
        // One tone, amplitude keyed on and off. The "off" symbols sit at the
        // noise floor the burst reports.
        let (b, s) =
            burst(8000, RATE, |_| 20_000.0, |i| if (i / sps) % 2 == 0 { 1.0 } else { 0.01 });
        assert!(s.env_depth > 0.9, "keying swings the envelope: {}", s.env_depth);
        assert_eq!(classify(&b, &s, None), BurstClass::Ook);
    }

    /// A weak constant-envelope burst must not be mistaken for keying. This is
    /// the case that made the old envelope measurement useless: noise lifts the
    /// swing, and only the comparison against the floor separates them.
    #[test]
    fn a_weak_constant_envelope_burst_is_not_called_keyed() {
        let sps = 25;
        let mut seed = 0x1234_5678_9ABC_DEF0u64;
        let mut noise = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            (seed >> 40) as f32 / 8_388_608.0 - 0.5
        };
        let n = 8000;
        let mut iq = Vec::with_capacity(n);
        let mut phase = 0.0f64;
        for i in 0..n {
            let f = if (i / sps) % 2 == 0 { 25_000.0f32 } else { -25_000.0 };
            phase += f64::from(f) * std::f64::consts::TAU / RATE;
            // Constant envelope plus a lot of noise: the swing is large, but
            // nothing ever reaches the floor.
            let (c, sn) = (phase.cos() as f32, phase.sin() as f32);
            iq.push(Complex32::new(c + 0.6 * noise(), sn + 0.6 * noise()));
        }
        let mut disc = Vec::new();
        let mut scratch = Vec::new();
        crate::demod::discriminate(&iq, RATE, &mut disc);
        let s = crate::demod::stats(&iq, &disc, &mut scratch);
        let b = Burst { iq, rate_hz: RATE, center_hz: 868_300_000.0, snr_db: 10.0, peak_dbfs: 0.0 };
        assert_ne!(
            classify(&b, &s, None),
            BurstClass::Ook,
            "a noisy FSK burst was called keyed; env_depth {} low {} floor {}",
            s.env_depth,
            s.env_low_dbfs,
            b.peak_dbfs - b.snr_db
        );
    }

    /// An unmodulated carrier is neither of the data classes.
    #[test]
    fn a_bare_carrier_is_reported_as_one() {
        let (b, s) = burst(8000, RATE, |_| 5_000.0, |_| 1.0);
        assert_eq!(classify(&b, &s, None), BurstClass::Carrier);
    }
}
