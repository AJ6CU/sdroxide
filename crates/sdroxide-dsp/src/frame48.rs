//! The streaming shell every 48 kHz fixed-frame denoiser needs.
//!
//! RNNoise and DeepFilterNet are both fixed at **48 kHz** and at **480-sample**
//! frames, and neither knows anything about a radio's audio rate. Both need the
//! same four steps around them:
//!
//!  * resample the input to 48 kHz (identity when it already is — the common
//!    voice-mode case),
//!  * hand the engine whole frames,
//!  * resample the denoised audio back to the input rate,
//!  * emit one output sample per input sample (a queue absorbs the framing /
//!    resampler jitter; it zero-fills during the initial priming latency).
//!
//! Engine-specific state — a network's hidden state, a delay-matched dry frame —
//! stays with the engine and is captured by the closure [`Frame48::process`]
//! takes. A closure rather than a trait because the two implementors' resets
//! differ (RNNoise recreates its `DenoiseState`, DeepFilterNet calls `init`) and
//! a trait would force a uniform one on them for no gain.

use std::collections::VecDeque;

use crate::resample::MonoResampler;

/// The rate both engines run at.
const RATE: f64 = 48_000.0;

pub(crate) struct Frame48 {
    /// Frame length in 48 kHz samples.
    frame: usize,
    in_rate: f64,
    /// `in_rate → 48 kHz` (None when already 48 kHz).
    up: Option<MonoResampler>,
    /// `48 kHz → in_rate` (None when already 48 kHz).
    down: Option<MonoResampler>,

    /// 48 kHz input samples awaiting a full frame.
    acc: Vec<f32>,
    /// Output samples at `in_rate`, ready to emit.
    out: VecDeque<f32>,

    // Per-call scratch.
    up_buf: Vec<f32>,
    /// Denoised 48 kHz samples awaiting downsample.
    den: Vec<f32>,
    down_buf: Vec<f32>,
}

impl Frame48 {
    pub(crate) fn new(frame: usize) -> Self {
        Frame48 {
            frame,
            // Default to 48 kHz so the common voice path needs no resampler.
            in_rate: RATE,
            up: None,
            down: None,
            acc: Vec::new(),
            out: VecDeque::new(),
            up_buf: Vec::new(),
            den: Vec::new(),
            down_buf: Vec::new(),
        }
    }

    /// Point the shell at a new input sample rate, rebuilding the resamplers and
    /// clearing state if it changed. Cheap no-op when the rate is unchanged.
    ///
    /// Returns true when it actually rebuilt, so the caller can clear whatever
    /// engine state it owns alongside.
    pub(crate) fn set_rate(&mut self, in_rate: f64) -> bool {
        if (in_rate - self.in_rate).abs() < 0.01 {
            return false;
        }
        self.in_rate = in_rate;
        self.reset();
        true
    }

    /// Clear all streaming state (call when switching the engine on so a stale
    /// ring doesn't glitch).
    pub(crate) fn reset(&mut self) {
        self.up = MonoResampler::new(self.in_rate, RATE);
        self.down = MonoResampler::new(RATE, self.in_rate);
        self.acc.clear();
        self.out.clear();
    }

    /// Denoise `audio` in place. Same length in and out; introduces a priming
    /// latency (zero-filled) of roughly one frame plus the resampler delay.
    ///
    /// `engine(frame, out)` is handed one 48 kHz frame of [`Frame48::frame`]
    /// samples and must push exactly that many to `out`.
    pub(crate) fn process(
        &mut self,
        audio: &mut [f32],
        mut engine: impl FnMut(&[f32], &mut Vec<f32>),
    ) {
        // 1. Up to 48 kHz.
        self.up_buf.clear();
        match self.up.as_mut() {
            Some(r) => r.push(audio, &mut self.up_buf),
            None => self.up_buf.extend_from_slice(audio),
        }
        self.acc.extend_from_slice(&self.up_buf);

        // 2. Hand the engine whole frames.
        self.den.clear();
        let mut consumed = 0;
        while self.acc.len() - consumed >= self.frame {
            let frame = &self.acc[consumed..consumed + self.frame];
            consumed += self.frame;
            engine(frame, &mut self.den);
        }
        self.acc.drain(..consumed);

        // 3. Back down to the input rate.
        self.down_buf.clear();
        match self.down.as_mut() {
            Some(r) => r.push(&self.den, &mut self.down_buf),
            None => self.down_buf.extend_from_slice(&self.den),
        }
        self.out.extend(self.down_buf.drain(..));

        // 4. One output per input sample; zero-fill while priming.
        for s in audio.iter_mut() {
            *s = self.out.pop_front().unwrap_or(0.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A passthrough engine: copies the frame straight to the output, so the
    /// shell's own bookkeeping is what is under test.
    fn passthrough(frame: &[f32], out: &mut Vec<f32>) {
        out.extend_from_slice(frame);
    }

    #[test]
    fn one_out_per_in_at_48k() {
        let mut s = Frame48::new(480);
        for &len in &[512usize, 300, 480, 1024, 137] {
            let mut buf = vec![0.1f32; len];
            s.process(&mut buf, passthrough);
            assert_eq!(buf.len(), len);
        }
    }

    #[test]
    fn one_out_per_in_after_a_rate_change() {
        let mut s = Frame48::new(480);
        assert!(s.set_rate(44_100.0));
        for &len in &[441usize, 1000, 137] {
            let mut buf = vec![0.1f32; len];
            s.process(&mut buf, passthrough);
            assert_eq!(buf.len(), len);
        }
    }

    #[test]
    fn set_rate_is_a_no_op_when_unchanged() {
        let mut s = Frame48::new(480);
        assert!(!s.set_rate(48_000.0));
        assert!(s.set_rate(44_100.0));
        assert!(!s.set_rate(44_100.0));
    }

    #[test]
    fn the_shell_itself_adds_no_delay_at_48k() {
        // On the identity-resample path the shell contributes nothing of its
        // own: a frame goes out as soon as it is complete. Any delay an engine
        // shows is the engine's, which is what lets `NeuralNr` reason about
        // RNNoise's one-frame offset in isolation.
        let mut s = Frame48::new(480);
        let n = 480 * 6;
        let input: Vec<f32> = (0..n).map(|i| (i as f32 * 0.001).sin()).collect();
        let mut buf = input.clone();
        s.process(&mut buf, passthrough);
        for i in 0..n {
            assert!(
                (buf[i] - input[i]).abs() < 1e-6,
                "misaligned at {i}: {} vs {}",
                buf[i],
                input[i]
            );
        }
    }

    #[test]
    fn a_partial_frame_is_held_until_it_completes() {
        // Fewer samples in than a frame means nothing can have been denoised
        // yet, so the shell zero-fills rather than emitting undenoised audio.
        let mut s = Frame48::new(480);
        let mut buf = vec![0.5f32; 100];
        s.process(&mut buf, passthrough);
        assert!(buf.iter().all(|&x| x == 0.0), "leaked un-framed audio");
    }
}
