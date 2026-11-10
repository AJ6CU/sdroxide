//! Neural (RNNoise) audio noise reduction — a recurrent-network speech
//! denoiser that estimates a per-band gain from learned features, pulling voice
//! out of noise that stationary spectral NR ([`crate::SpectralNr`]) can't touch
//! (babble, non-stationary hiss, some tones).
//!
//! Wraps the pure-Rust [`nnnoiseless`] port of Xiph's RNNoise. RNNoise is fixed
//! at **48 kHz** and **480-sample** frames; the streaming, framing and
//! resampling that arbitrary input needs around that belong to
//! [`crate::frame48::Frame48`], shared with [`crate::DeepFilterNr`].
//!
//! RNNoise itself has no intensity knob, so the **Low/Med/High** setting is a
//! wet/dry depth: the denoised frame is blended with the *dry* input. RNNoise
//! carries a one-frame processing delay (its analysis window spans the previous
//! and current frame; synthesis emits the previous frame's region), so the dry
//! side is the **previous** input frame — delay-matched, no comb filtering.
//! At `mix = 1.0` (High) the output is pure denoised, so alignment is moot.
//!
//! Pure Rust, wasm-clean. In-place, same length in and out.

use nnnoiseless::DenoiseState;

use crate::frame48::Frame48;

/// RNNoise frame length (samples at 48 kHz).
const FRAME: usize = DenoiseState::FRAME_SIZE;
/// RNNoise expects samples scaled to the `i16` range, not `[-1, 1]`.
const SCALE: f32 = 32_768.0;

pub struct NeuralNr {
    denoise: Box<DenoiseState<'static>>,
    /// Resampling, framing and the one-in/one-out queue.
    shell: Frame48,
    /// Wet/dry blend depth (0 = bypass, 1 = full RNNoise).
    mix: f32,

    /// Previous 48 kHz input frame — the delay-matched dry signal.
    prev_frame: Vec<f32>,
    have_prev: bool,

    // Per-call scratch.
    frame_in: Vec<f32>,
    frame_out: Vec<f32>,
}

impl NeuralNr {
    pub fn new() -> Self {
        NeuralNr {
            denoise: DenoiseState::new(),
            shell: Frame48::new(FRAME),
            mix: 1.0,
            prev_frame: vec![0.0; FRAME],
            have_prev: false,
            frame_in: vec![0.0; FRAME],
            frame_out: vec![0.0; FRAME],
        }
    }

    /// Set the wet/dry depth (clamped to `[0, 1]`): higher removes more noise.
    pub fn set_mix(&mut self, mix: f32) {
        self.mix = mix.clamp(0.0, 1.0);
    }

    /// Point the denoiser at a new input sample rate, rebuilding the resamplers
    /// and clearing state if it changed. Cheap no-op when the rate is unchanged.
    /// Deliberately does *not* recreate the RNN — only [`NeuralNr::reset`] does.
    pub fn set_rate(&mut self, in_rate: f64) {
        if self.shell.set_rate(in_rate) {
            self.clear_dry();
        }
    }

    /// Clear all streaming state (call when switching NR on so a stale ring
    /// doesn't glitch). Recreates the RNN so its hidden state starts fresh.
    pub fn reset(&mut self) {
        self.denoise = DenoiseState::new();
        self.shell.reset();
        self.clear_dry();
    }

    fn clear_dry(&mut self) {
        self.prev_frame.iter_mut().for_each(|x| *x = 0.0);
        self.have_prev = false;
    }

    /// Denoise `audio` in place. Same length in and out; introduces a priming
    /// latency (zero-filled) of roughly one frame plus the resampler delay.
    pub fn process(&mut self, audio: &mut [f32]) {
        // Destructured so the closure borrows only the fields it touches.
        let Self { denoise, shell, mix, prev_frame, have_prev, frame_in, frame_out } = self;
        let mix = *mix;
        shell.process(audio, |frame, den| {
            // RNNoise works in i16 scale.
            for (d, &s) in frame_in.iter_mut().zip(frame) {
                *d = s * SCALE;
            }
            denoise.process_frame(frame_out, frame_in);
            if *have_prev {
                // `frame_out` is the denoised *previous* input frame; blend it
                // with that same (delay-matched) dry frame.
                for i in 0..FRAME {
                    let wet = frame_out[i] / SCALE;
                    let dry = prev_frame[i];
                    den.push(dry + mix * (wet - dry));
                }
            } else {
                // First frame out is priming garbage — emit silence, keeping the
                // one-in/one-out sample count.
                den.resize(den.len() + FRAME, 0.0);
            }
            prev_frame.copy_from_slice(frame);
            *have_prev = true;
        });
    }
}

impl Default for NeuralNr {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rms(x: &[f32]) -> f32 {
        (x.iter().map(|&s| s * s).sum::<f32>() / x.len().max(1) as f32).sqrt()
    }

    fn noise(n: usize, amp: f32, seed: u64) -> Vec<f32> {
        let mut r = seed;
        (0..n)
            .map(|_| {
                r ^= r << 13;
                r ^= r >> 7;
                r ^= r << 17;
                (((r >> 40) as i32) as f32 / (1 << 23) as f32 - 1.0) * amp
            })
            .collect()
    }

    #[test]
    fn preserves_sample_count() {
        let mut nr = NeuralNr::new();
        nr.set_mix(1.0);
        // Odd block sizes at 48 kHz (identity resample path).
        for &len in &[512usize, 300, 480, 1024, 137] {
            let mut buf = vec![0.1f32; len];
            nr.process(&mut buf);
            assert_eq!(buf.len(), len);
        }
    }

    #[test]
    fn reduces_broadband_noise_at_full_depth() {
        let mut nr = NeuralNr::new();
        nr.set_rate(48_000.0);
        nr.set_mix(1.0); // High: pure denoised output
        let mut buf = noise(96_000, 0.2, 0x51DE);
        let before = rms(&buf);
        nr.process(&mut buf);
        // Skip priming latency.
        let after = rms(&buf[FRAME * 8..]);
        assert!(after < 0.9 * before, "noise not reduced: {before} -> {after}");
    }

    #[test]
    fn low_depth_stays_near_dry() {
        // At mix = 0 the output is the (delayed) dry signal, so its level tracks
        // the input rather than collapsing.
        let mut nr = NeuralNr::new();
        nr.set_mix(0.0);
        let mut buf = noise(48_000, 0.2, 0xD00D);
        let before = rms(&buf);
        nr.process(&mut buf);
        let after = rms(&buf[FRAME * 4..]);
        assert!(after > 0.9 * before, "dry path attenuated: {before} -> {after}");
    }

    #[test]
    fn dry_path_is_delayed_by_exactly_one_frame() {
        // The wet/dry blend assumes RNNoise emits the *previous* input frame
        // (one-frame delay). At mix = 0 the output is pure dry, so on the
        // 48 kHz identity path it must equal the input delayed by exactly
        // FRAME samples — otherwise a partial blend would comb-filter.
        let mut nr = NeuralNr::new();
        nr.set_mix(0.0);
        let n = FRAME * 6;
        let input = noise(n, 0.2, 0xA11);
        let mut buf = input.clone();
        nr.process(&mut buf);
        for i in 0..(n - FRAME) {
            assert!(
                (buf[FRAME + i] - input[i]).abs() < 1e-6,
                "misaligned at {i}: {} vs {}",
                buf[FRAME + i],
                input[i]
            );
        }
    }

    #[test]
    fn resamples_non_48k_rate() {
        // A non-48 kHz rate exercises the up/down resampler path and must still
        // return the same number of samples.
        let mut nr = NeuralNr::new();
        nr.set_rate(44_100.0);
        nr.set_mix(1.0);
        let mut buf = noise(44_100, 0.15, 0xBEE5);
        nr.process(&mut buf);
        assert_eq!(buf.len(), 44_100);
    }
}
