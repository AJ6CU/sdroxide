//! Piper VITS inference through `rten`.
//!
//! The model takes a sequence of phoneme IDs and three scale parameters, and
//! returns audio samples at the voice's own rate. It is a whole-utterance
//! model — there is no streaming — which is why callers chunk long text before
//! getting here.

use std::path::PathBuf;

use rten::Model;
use rten_tensor::NdTensor;
use rten_tensor::prelude::*;

use super::{SynthError, Voice};

/// A loaded voice, ready to speak phonemes.
pub struct Synth {
    model: Model,
    voice: Voice,
}

impl Synth {
    /// Memory-map the model rather than reading it: the file is around 60 MB
    /// and mapping it keeps that out of the resident set until the pages are
    /// actually touched, which for a program that may never speak matters.
    ///
    /// Mapping is unsafe because rten reads tensor data straight out of the
    /// mapping, so another process truncating or rewriting the file underneath
    /// us would be undefined behaviour rather than a decode error. The voices
    /// we map are installed read-only beside the binary, and the alternative —
    /// holding 60 MB resident for a feature most operators leave off — is a
    /// worse trade than a file nobody but root can rewrite.
    pub fn load(voice: Voice) -> Result<Self, SynthError> {
        let model = unsafe { Model::load_mmap(&voice.model) }
            .map_err(|e| SynthError::ModelLoad(voice.model.clone(), e.to_string()))?;
        Ok(Synth { model, voice })
    }

    pub fn voice(&self) -> &Voice {
        &self.voice
    }

    /// The rate the model generates at. Callers resample from this.
    pub fn sample_rate(&self) -> u32 {
        self.voice.config.audio.sample_rate
    }

    pub fn model_path(&self) -> &PathBuf {
        &self.voice.model
    }

    /// Encode IPA phonemes to model input IDs.
    ///
    /// The sequence is `^`, then each phoneme's IDs separated by 0, then `$`.
    /// Symbols outside the voice's map cannot be spoken; they are dropped, and
    /// the caller counts them so a systematically wrong phonemizer shows up as
    /// something other than mangled audio.
    fn encode(&self, phonemes: &str) -> (Vec<i32>, usize) {
        let map = &self.voice.config.phoneme_id_map;
        let mut ids: Vec<i32> = map.get(&'^').cloned().unwrap_or_default();
        let mut unknown = 0usize;
        for ch in phonemes.chars() {
            match map.get(&ch) {
                Some(v) => {
                    ids.extend_from_slice(v);
                    ids.push(0);
                }
                None => unknown += 1,
            }
        }
        ids.extend(map.get(&'$').cloned().unwrap_or_default());
        (ids, unknown)
    }

    /// Synthesize one utterance.
    ///
    /// `rate` is the operator's speech-rate multiplier: 2.0 is twice as fast.
    /// It divides the voice's own `length_scale`, so the model stretches the
    /// phoneme durations itself — no resampling, no pitch shift.
    pub fn speak_phonemes(&self, phonemes: &str, rate: f32) -> Result<Vec<f32>, SynthError> {
        let (ids, unknown) = self.encode(phonemes);
        if unknown > 0 {
            tracing::debug!(unknown, phonemes, "phonemes outside the voice's map were dropped");
        }
        // `^` and `$` alone: nothing to say. The model would still run and
        // return a click.
        if ids.len() <= 2 {
            return Ok(Vec::new());
        }

        let inf = &self.voice.config.inference;
        let length_scale = inf.length_scale / rate.clamp(0.25, 4.0);

        let phoneme_ids: NdTensor<i32, 1> = ids.into();
        let input_lengths = NdTensor::from([phoneme_ids.size(0) as i32]);
        let phoneme_ids = phoneme_ids.with_new_axis(0); // batch dim
        let scales = NdTensor::from([inf.noise_scale, length_scale, inf.noise_w]);

        let node = |name: &str| {
            self.model
                .node_id(name)
                .map_err(|e| SynthError::Inference(format!("input `{name}`: {e}")))
        };
        let [out] = self
            .model
            .run_n(
                [
                    (node("input")?, phoneme_ids.into()),
                    (node("input_lengths")?, input_lengths.into()),
                    (node("scales")?, scales.into()),
                ]
                .into(),
                [node("output")?],
                None,
            )
            .map_err(|e| SynthError::Inference(e.to_string()))?;

        // (batch, time, 1, sample)
        let samples: NdTensor<f32, 4> =
            out.try_into().map_err(|e| SynthError::Inference(format!("output: {e:?}")))?;
        Ok(samples.iter().copied().collect())
    }
}
