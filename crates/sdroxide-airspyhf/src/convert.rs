//! Wire bytes to baseband, and the host DSP chain in the order it has to run.
//!
//! Two things live here. [`Deconstructor`] turns the bulk endpoint's bytes into
//! complex samples — the one place in the driver that knows the samples arrive
//! **Q before I**. [`HostDsp`] is the chain that runs after it, in one struct so
//! the order cannot drift: DC cancellation and image balancing first, the
//! fine-tuning oscillator last.

use sdroxide_dsp::{Complex32, Nco};

use crate::iqbalancer::IqBalancer;
use crate::protocol::{BYTES_PER_SAMPLE, SAMPLE_SCALE};

/// Bulk bytes to complex samples.
///
/// Holds the 1–3 leftover bytes of a sample split across two transfers. Without
/// that carry a single odd-length completion would swap I and Q for the rest of
/// the session — a mirrored spectrum that reads like a driver bug rather than
/// the framing slip it is.
pub struct Deconstructor {
    carry: [u8; BYTES_PER_SAMPLE],
    carry_len: usize,
    /// `SAMPLE_SCALE` times the filter gain, so the whole conversion is one
    /// multiply per component.
    gain: f32,
}

impl Default for Deconstructor {
    fn default() -> Self {
        Deconstructor::new()
    }
}

impl Deconstructor {
    pub fn new() -> Deconstructor {
        Deconstructor { carry: [0; BYTES_PER_SAMPLE], carry_len: 0, gain: SAMPLE_SCALE }
    }

    /// Set the receiver's reported processing gain, which the conversion
    /// undoes. 1.0 leaves full scale at ±1.
    pub fn set_filter_gain(&mut self, filter_gain: f32) {
        self.gain = SAMPLE_SCALE * filter_gain;
    }

    /// Drop any partial sample.
    ///
    /// Called after an endpoint stall or a stream restart: the bytes either
    /// side of a break are not two halves of the same sample, and pairing them
    /// would put the stream permanently out of step.
    pub fn reset(&mut self) {
        self.carry_len = 0;
    }

    /// Append the samples in `bytes` to `out`.
    pub fn push(&mut self, bytes: &[u8], out: &mut Vec<Complex32>) {
        let mut src = 0;
        // Finish the sample left over from the previous transfer first.
        if self.carry_len > 0 {
            let need = BYTES_PER_SAMPLE - self.carry_len;
            let take = need.min(bytes.len());
            self.carry[self.carry_len..self.carry_len + take].copy_from_slice(&bytes[..take]);
            self.carry_len += take;
            src = take;
            if self.carry_len < BYTES_PER_SAMPLE {
                return;
            }
            self.carry_len = 0;
            out.push(sample(&self.carry, self.gain));
        }

        let rest = &bytes[src..];
        let whole = rest.len() - rest.len() % BYTES_PER_SAMPLE;
        out.reserve(whole / BYTES_PER_SAMPLE);
        for frame in rest[..whole].chunks_exact(BYTES_PER_SAMPLE) {
            out.push(sample(frame, self.gain));
        }

        let tail = &rest[whole..];
        self.carry[..tail.len()].copy_from_slice(tail);
        self.carry_len = tail.len();
    }
}

/// One four-byte frame: `{ i16 im; i16 re; }`, little-endian.
///
/// **Q first.** Upstream's `airspyhf_complex_int16_t` declares `im` before `re`
/// and it is not a typo; reading them the other way round mirrors the whole
/// spectrum, which on the air looks like a hardware fault rather than a driver
/// one.
#[inline]
fn sample(b: &[u8], gain: f32) -> Complex32 {
    let im = i16::from_le_bytes([b[0], b[1]]);
    let re = i16::from_le_bytes([b[2], b[3]]);
    Complex32::new(re as f32 * gain, im as f32 * gain)
}

/// The host-side chain, in the order it must run.
///
/// 1. **DC cancellation and image balancing** ([`IqBalancer`]) — zero-IF rates
///    only. A low-IF rate has the wanted signal away from both the DC spur and
///    the mirror already, and the firmware rejects the image itself.
/// 2. **The fine-tuning oscillator**, which does three jobs at once: bringing
///    the signal back from the 5 kHz the synthesiser was parked away from it,
///    absorbing the rounding of an LO programmed in whole kHz, and doing *all*
///    the tuning below the synthesiser's floor.
///
/// [`HostDsp::set_enabled`] switches the whole thing off, which is both the
/// "show me the raw hardware" setting and the one-click bisect between a driver
/// fault and a DSP one.
pub struct HostDsp {
    enabled: bool,
    low_if: bool,
    rate_hz: f64,
    shift_hz: f64,
    balancer: IqBalancer,
    nco: Nco,
}

impl HostDsp {
    pub fn new(rate_hz: f64) -> HostDsp {
        HostDsp {
            enabled: true,
            low_if: false,
            rate_hz,
            shift_hz: 0.0,
            balancer: IqBalancer::new(),
            nco: Nco::new(0.0, rate_hz),
        }
    }

    pub fn set_enabled(&mut self, on: bool) {
        self.enabled = on;
    }

    pub fn set_low_if(&mut self, low_if: bool) {
        self.low_if = low_if;
    }

    /// How far the wanted signal sits **above** the LO, in Hz.
    ///
    /// The oscillator therefore has to run at minus this to bring it to DC —
    /// which is the sign that is easy to get backwards and that puts a zero-IF
    /// signal 10 kHz out while looking exactly like a calibration error. The
    /// retune also restarts the image estimate, because the imbalance is
    /// frequency-dependent and the old one is now pointed at the wrong place.
    ///
    /// A no-op when the shift has not moved. The stream thread re-reads every
    /// DSP setting after any control change rather than tracking which one
    /// moved, so without this guard an attenuator slider would throw away a
    /// converged image estimate on every frame of the drag.
    pub fn set_shift(&mut self, shift_hz: f64) {
        if shift_hz == self.shift_hz {
            return;
        }
        self.shift_hz = shift_hz;
        self.nco.set_freq(-shift_hz, self.rate_hz);
        self.balancer.reset();
    }

    pub fn balancer(&self) -> &IqBalancer {
        &self.balancer
    }

    /// Run the chain in place.
    pub fn process(&mut self, iq: &mut [Complex32]) {
        if !self.enabled {
            return;
        }
        if !self.low_if {
            self.balancer.process(iq);
        }
        if self.shift_hz != 0.0 {
            self.nco.mix_in_place(iq);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::TAU;

    /// The single most consequential byte-order fact in the driver, asserted
    /// against literals rather than against a re-derivation of it.
    #[test]
    fn q_comes_before_i_on_the_wire() {
        let mut d = Deconstructor::new();
        let mut out = Vec::new();
        // Little-endian i16 0x2000 then 0x4000. The first is the *imaginary*
        // part; the second is the real one.
        d.push(&[0x00, 0x20, 0x00, 0x40], &mut out);
        assert_eq!(out.len(), 1);
        assert!((out[0].re - 0.5).abs() < 1e-6, "re was {}", out[0].re);
        assert!((out[0].im - 0.25).abs() < 1e-6, "im was {}", out[0].im);

        // Negative values sign-extend, and full scale stays inside ±1.
        out.clear();
        d.push(&i16::MIN.to_le_bytes(), &mut out);
        d.push(&i16::MAX.to_le_bytes(), &mut out);
        assert_eq!(out.len(), 1);
        assert!((out[0].im + 1.0).abs() < 1e-6);
        assert!(out[0].re < 1.0 && out[0].re > 0.999);
    }

    /// Completions do not have to land on sample boundaries. A carry that lost
    /// a byte would swap I and Q for the rest of the session.
    #[test]
    fn a_partial_frame_is_carried_across_transfers() {
        let bytes: Vec<u8> = (0..4096u32).map(|i| (i * 37 % 251) as u8).collect();

        let mut whole = Vec::new();
        Deconstructor::new().push(&bytes, &mut whole);
        assert_eq!(whole.len(), bytes.len() / BYTES_PER_SAMPLE);

        // Chunk sizes chosen so that every possible carry length occurs.
        for sizes in [&[1usize, 3, 5, 7, 2, 6][..], &[1][..], &[3, 1][..], &[4095, 1][..]] {
            let mut d = Deconstructor::new();
            let mut got = Vec::new();
            let mut at = 0;
            let mut i = 0;
            while at < bytes.len() {
                let n = sizes[i % sizes.len()].min(bytes.len() - at);
                d.push(&bytes[at..at + n], &mut got);
                at += n;
                i += 1;
            }
            assert_eq!(got, whole, "split by {sizes:?}");
        }
    }

    /// After a stall the bytes either side of the break are not two halves of
    /// one sample. Pairing them would put the stream out of step for good.
    #[test]
    fn a_reset_drops_the_partial_sample_rather_than_pairing_across_the_gap() {
        let mut d = Deconstructor::new();
        let mut out = Vec::new();
        d.push(&[0x11, 0x22], &mut out);
        assert!(out.is_empty());
        d.reset();
        d.push(&[0x00, 0x20, 0x00, 0x40], &mut out);
        assert_eq!(out.len(), 1);
        assert!((out[0].re - 0.5).abs() < 1e-6);
    }

    #[test]
    fn the_filter_gain_scales_the_whole_stream() {
        let mut d = Deconstructor::new();
        d.set_filter_gain(0.5);
        let mut out = Vec::new();
        d.push(&[0x00, 0x20, 0x00, 0x40], &mut out);
        assert!((out[0].re - 0.25).abs() < 1e-6);
        assert!((out[0].im - 0.125).abs() < 1e-6);
    }

    /// The residual frequency of a shifted tone, in Hz, from the mean
    /// sample-to-sample phase advance. More precise than picking an FFT peak
    /// and it needs no window.
    fn residual_hz(x: &[Complex32], rate: f64) -> f64 {
        let mut acc = num_complex::Complex::<f64>::new(0.0, 0.0);
        for w in x.windows(2) {
            let a = num_complex::Complex::<f64>::new(w[1].re as f64, w[1].im as f64);
            let b = num_complex::Complex::<f64>::new(w[0].re as f64, -w[0].im as f64);
            acc += a * b;
        }
        acc.arg() * rate / TAU
    }

    /// The chain, end to end, from wire bytes to baseband — and the one test
    /// that pins the oscillator's *sign*.
    ///
    /// A zero-IF dial parks the LO 5 kHz above the signal, so the signal
    /// arrives at −5 kHz and the shift is −5000. Getting the sign backwards
    /// puts it at −10 kHz, which on the air reads as a calibration error rather
    /// than as the driver bug it is.
    #[test]
    fn the_chain_brings_a_parked_signal_back_to_dc() {
        const RATE: f64 = 768_000.0;
        const N: usize = 768_000;
        const OFFSET: f64 = -5_000.0;

        // Build the wire bytes for a tone at OFFSET, exactly as the endpoint
        // would deliver them — Q first.
        let mut wire = Vec::with_capacity(N * BYTES_PER_SAMPLE);
        for n in 0..N {
            let t = TAU * OFFSET * n as f64 / RATE;
            let (re, im) = ((t.cos() * 20_000.0) as i16, (t.sin() * 20_000.0) as i16);
            wire.extend_from_slice(&im.to_le_bytes());
            wire.extend_from_slice(&re.to_le_bytes());
        }

        let mut d = Deconstructor::new();
        let mut dsp = HostDsp::new(RATE);
        // Low-IF for this test: it isolates the oscillator from the balancer,
        // which would otherwise be adjusting the same samples.
        dsp.set_low_if(true);
        dsp.set_shift(OFFSET);

        let mut out = Vec::with_capacity(N);
        // In transfer-sized pieces, because that is how it really arrives.
        for chunk in wire.chunks(crate::protocol::TRANSFER_BYTES) {
            let at = out.len();
            d.push(chunk, &mut out);
            dsp.process(&mut out[at..]);
        }
        assert_eq!(out.len(), N);

        let f = residual_hz(&out, RATE);
        assert!(f.abs() < 0.05, "the signal ended up {f:.4} Hz from DC, not at it");

        // And the same tone with no shift set stays where it was, so the test
        // above cannot pass by the oscillator doing nothing.
        let mut idle = HostDsp::new(RATE);
        idle.set_low_if(true);
        let mut raw = Vec::new();
        Deconstructor::new().push(&wire, &mut raw);
        idle.process(&mut raw);
        assert!((residual_hz(&raw, RATE) - OFFSET).abs() < 0.05);
    }

    /// The whole chain has one switch, and it has to reach every part: a
    /// disabled chain must not shift, balance or DC-block.
    #[test]
    fn disabling_the_host_dsp_leaves_the_samples_untouched() {
        let mut dsp = HostDsp::new(768_000.0);
        dsp.set_shift(-5_000.0);
        dsp.set_enabled(false);
        let src: Vec<Complex32> =
            (0..8192).map(|i| Complex32::new(0.25 + (i % 7) as f32 * 0.01, -0.5)).collect();
        let mut x = src.clone();
        dsp.process(&mut x);
        assert_eq!(x, src);
    }

    /// The stream thread re-reads every DSP setting after any control change
    /// rather than tracking which one moved, so a shift that has not actually
    /// moved must not restart the image estimate — or an attenuator drag would
    /// throw away a converged correction on every frame of it.
    #[test]
    fn setting_the_same_shift_again_does_not_restart_the_image_estimate() {
        let mut dsp = HostDsp::new(768_000.0);
        dsp.set_shift(-5_000.0);
        // Converge on something, so there is an estimate to lose.
        for blk in 0..40 {
            let n = crate::iqbalancer::WORKING_BUFFER_LEN;
            let mut x: Vec<Complex32> = (0..n)
                .map(|i| {
                    let t = std::f32::consts::TAU * 700.0 * (blk * n + i) as f32 / 4096.0;
                    let (re, im) = (t.cos(), t.sin());
                    Complex32::new((re + 0.02 * im) * 1.01, (im + 0.02 * re) * 0.99)
                })
                .collect();
            dsp.balancer().estimates();
            dsp.process(&mut x);
        }
        let converged = dsp.balancer().state();
        assert_ne!(converged, (0.0, 0.0));

        dsp.set_shift(-5_000.0);
        let mut x = vec![Complex32::new(0.1, 0.1); crate::iqbalancer::WORKING_BUFFER_LEN];
        dsp.process(&mut x);
        assert_eq!(dsp.balancer().state(), converged, "an unchanged shift reset the estimate");

        // A real retune does reset it, because the imbalance is
        // frequency-dependent and the old estimate is now pointed elsewhere.
        dsp.set_shift(-4_000.0);
        dsp.process(&mut x);
        assert_eq!(dsp.balancer().estimates(), 0);
    }

    /// A low-IF rate has the image rejected in the firmware, so the balancer
    /// must not run — it would spend CPU and move a correction nothing needs.
    #[test]
    fn the_balancer_runs_only_on_zero_if_rates() {
        for (low_if, want) in [(true, 0), (false, 1)] {
            let mut dsp = HostDsp::new(768_000.0);
            dsp.set_low_if(low_if);
            let mut x = vec![Complex32::new(0.5, 0.5); 4096];
            dsp.process(&mut x);
            // The DC canceller lives inside the balancer, so a constant input
            // coming out unchanged is the observable difference.
            let moved = usize::from(x[4095] != Complex32::new(0.5, 0.5));
            assert_eq!(moved, want, "low_if = {low_if}");
        }
    }
}
