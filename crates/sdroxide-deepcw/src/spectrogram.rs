//! Audio in, model input out.
//!
//! The model was trained on one specific view of the signal and there is no
//! latitude here: 3200 Hz mono, a 256-point periodic Hann window every 48
//! samples, magnitude, `log1p`, and only the 65 bins covering 400–1200 Hz. Every
//! constant below is read off the metadata that ships beside the weights
//! (`assets/model.onnx.json`) and is checked against it in the tests, because a
//! silent mismatch here does not fail — it just decodes badly, which is a much
//! worse way to find out.

use rustfft::num_complex::Complex32;
use rustfft::{Fft, FftPlanner};
use std::sync::Arc;

/// Sample rate the model expects. Callers resample to this before pushing.
pub const SAMPLE_RATE: f64 = 3200.0;

const FFT_LENGTH: usize = 256;
const HOP_LENGTH: usize = 48;

/// Width of one FFT bin, 12.5 Hz.
pub const BIN_HZ: f64 = SAMPLE_RATE / FFT_LENGTH as f64;

const MIN_FREQ_HZ: f64 = 400.0;
const MAX_FREQ_HZ: f64 = 1200.0;

/// First and last (exclusive) rfft bin the model sees: 32..97.
const START_BIN: usize = (MIN_FREQ_HZ / BIN_HZ) as usize;
const STOP_BIN: usize = (MAX_FREQ_HZ / BIN_HZ) as usize + 1;

/// Frequency bins per frame, the model's input width.
pub const BINS: usize = STOP_BIN - START_BIN;

/// Index of 800 Hz within the cropped band — where a front end should put the
/// carrier when it builds the model's input itself rather than handing us audio.
pub const CENTER_BIN: usize = (STOP_BIN - 1 + START_BIN) / 2 - START_BIN;

/// Length of the analysis window in seconds, 80 ms.
///
/// A front end building its own spectrogram has to match this and [`BIN_HZ`]
/// exactly: together they are the time-frequency trade the model was trained on,
/// and it reads keying against both.
pub const WINDOW_S: f64 = FFT_LENGTH as f64 / SAMPLE_RATE;

/// Centre of the model's band, 800 Hz. Front ends place the signal here: it is
/// the frequency the model has seen most, and it leaves the most room either
/// side for a tone that drifts or was mistuned.
pub const CENTER_FREQ_HZ: f64 = (MIN_FREQ_HZ + MAX_FREQ_HZ) / 2.0;

/// Seconds of audio per output frame, 15 ms.
pub const FRAME_S: f64 = HOP_LENGTH as f64 / SAMPLE_RATE;

/// Reflect padding applied at both ends, so frame `i` is centred on sample
/// `i * HOP_LENGTH` rather than starting there.
const PAD: usize = FFT_LENGTH / 2;

/// Shortest input we will look at. Below this the reflect padding has nothing
/// to reflect.
pub const MIN_SAMPLES: usize = PAD + 1;

/// Number of frames [`Spectrogram::compute`] produces for `samples` samples.
pub fn frames_for(samples: usize) -> usize {
    if samples < MIN_SAMPLES {
        return 0;
    }
    1 + (samples + 2 * PAD - FFT_LENGTH) / HOP_LENGTH
}

/// The sample a frame is centred on, inverting [`frames_for`]. Used to turn a
/// frame index the model reported back into a position in the audio.
pub fn frame_to_sample(frame: usize) -> usize {
    frame * HOP_LENGTH
}

/// Reusable STFT front end. Holds the window, the FFT plan and its scratch, so
/// a decode allocates only the spectrogram it returns.
pub struct Spectrogram {
    fft: Arc<dyn Fft<f32>>,
    window: [f32; FFT_LENGTH],
    padded: Vec<f32>,
    frame: Vec<Complex32>,
    scratch: Vec<Complex32>,
}

impl Spectrogram {
    pub fn new() -> Self {
        let fft = FftPlanner::<f32>::new().plan_fft_forward(FFT_LENGTH);
        // Periodic Hann — numpy's `np.hanning(n + 1)[:-1]`, which divides by n
        // rather than n - 1. The symmetric window is a different taper and
        // shifts every magnitude slightly.
        let mut window = [0.0f32; FFT_LENGTH];
        for (i, w) in window.iter_mut().enumerate() {
            *w = 0.5 * (1.0 - (std::f64::consts::TAU * i as f64 / FFT_LENGTH as f64).cos()) as f32;
        }
        let scratch = vec![Complex32::default(); fft.get_inplace_scratch_len()];
        Spectrogram {
            fft,
            window,
            padded: Vec::new(),
            frame: vec![Complex32::default(); FFT_LENGTH],
            scratch,
        }
    }

    /// `[frames][BINS]` row-major, ready to hand to the model as
    /// `[1, 1, frames, BINS]`. Empty when the input is too short.
    pub fn compute(&mut self, audio: &[f32]) -> Vec<f32> {
        let frames = frames_for(audio.len());
        if frames == 0 {
            return Vec::new();
        }

        self.reflect_pad(audio);

        let mut out = vec![0.0f32; frames * BINS];
        for f in 0..frames {
            let start = f * HOP_LENGTH;
            for (i, slot) in self.frame.iter_mut().enumerate() {
                *slot = Complex32::new(self.padded[start + i] * self.window[i], 0.0);
            }
            self.fft.process_with_scratch(&mut self.frame, &mut self.scratch);

            let row = &mut out[f * BINS..][..BINS];
            for (bin, slot) in row.iter_mut().enumerate() {
                // log1p on the magnitude. Not dB and not normalised: the model
                // learned an absolute scale, so nothing here is level-invariant
                // and the caller's gain matters.
                *slot = self.frame[START_BIN + bin].norm().ln_1p();
            }
        }
        out
    }

    /// numpy's `mode="reflect"`: the edge sample is the mirror line and is not
    /// repeated, so padding `[1,2,3,4,5]` by 2 gives `[3,2,1,2,3,4,5,4,3]`.
    fn reflect_pad(&mut self, audio: &[f32]) {
        self.padded.clear();
        self.padded.reserve(audio.len() + 2 * PAD);
        for j in 0..PAD {
            self.padded.push(audio[(PAD - j).min(audio.len() - 1)]);
        }
        self.padded.extend_from_slice(audio);
        let n = audio.len();
        for j in 0..PAD {
            self.padded.push(audio[n.saturating_sub(2 + j).min(n - 1)]);
        }
    }
}

impl Default for Spectrogram {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn band_matches_the_model_metadata() {
        // The weights and this front end have to agree exactly, and a mismatch
        // does not fail loudly — it just decodes badly. So check the constants
        // against the metadata that shipped with the model.
        let meta = include_str!("../assets/model.onnx.json");
        for expected in [
            "\"sample_rate\": 3200",
            "\"fft_length\": 256",
            "\"hop_length\": 48",
            "\"spectrogram_min_freq_hz\": 400.0",
            "\"spectrogram_max_freq_hz\": 1200.0",
            "\"spectrogram_frequency_bins\": 65",
            "\"normalization\": \"log1p\"",
        ] {
            assert!(meta.contains(expected), "model metadata is missing {expected}");
        }
        assert_eq!(SAMPLE_RATE, 3200.0);
        assert_eq!(FFT_LENGTH, 256);
        assert_eq!(HOP_LENGTH, 48);
        assert_eq!(BINS, 65);
        assert_eq!(START_BIN, 32);
        assert_eq!(STOP_BIN, 97);
    }

    #[test]
    fn reflect_padding_mirrors_without_repeating_the_edge() {
        let mut sg = Spectrogram::new();
        let audio: Vec<f32> = (1..=300).map(|v| v as f32).collect();
        sg.reflect_pad(&audio);
        assert_eq!(sg.padded.len(), audio.len() + 2 * PAD);
        // Leading pad counts back down to the second sample.
        assert_eq!(sg.padded[0], audio[PAD]);
        assert_eq!(sg.padded[PAD - 1], audio[1]);
        assert_eq!(sg.padded[PAD], audio[0]);
        // Trailing pad counts back up from the second-to-last.
        let n = audio.len();
        assert_eq!(sg.padded[PAD + n], audio[n - 2]);
        assert_eq!(sg.padded[PAD + n + PAD - 1], audio[n - 1 - PAD]);
    }

    #[test]
    fn a_tone_lands_in_the_bin_it_belongs_to() {
        let mut sg = Spectrogram::new();
        let n = 3200;
        let audio: Vec<f32> = (0..n)
            .map(|i| (std::f64::consts::TAU * 800.0 * i as f64 / SAMPLE_RATE).sin() as f32)
            .collect();
        let spec = sg.compute(&audio);
        let frames = frames_for(n);
        assert_eq!(spec.len(), frames * BINS);

        // 800 Hz is bin 64, which is index 32 of the cropped band.
        let mid = frames / 2;
        let row = &spec[mid * BINS..][..BINS];
        let peak = row.iter().enumerate().max_by(|a, b| a.1.total_cmp(b.1)).unwrap().0;
        assert_eq!(peak, 32, "800 Hz should peak at the centre of the band");
    }

    #[test]
    fn short_input_produces_nothing() {
        let mut sg = Spectrogram::new();
        assert!(sg.compute(&[0.0; 64]).is_empty());
        assert_eq!(frames_for(64), 0);
    }
}
