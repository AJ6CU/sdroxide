//! Turning text into audio: phonemes, Piper VITS inference, and playback.
//!
//! Behind the `synth` feature. Everything above this module is pure and builds
//! anywhere; this half links a model runtime and a sound card.

use std::path::PathBuf;

pub mod engine;
pub mod model;
pub mod phonemes;
pub mod voice;

pub use engine::{SpeechEngine, SpeechHandle, SpeechStatus};
pub use model::Synth;
pub use phonemes::Phonemes;
pub use voice::{DEFAULT_VOICE, Voice, VoiceConfig, find_voice, list_voices, voice_dirs};

/// Why speech could not start, or could not speak.
///
/// Every variant is reported to the operator in the settings dialog rather than
/// panicking: a missing voice file must leave the radio working.
#[derive(Debug, thiserror::Error)]
pub enum SynthError {
    #[error("no speech voice found (looked for {want} in {})", fmt_dirs(.dirs))]
    NoVoice { want: String, dirs: Vec<PathBuf> },

    #[error("voice config {0}: {1}")]
    VoiceConfig(PathBuf, String),

    #[error("loading voice model {0}: {1}")]
    ModelLoad(PathBuf, String),

    #[error("speech synthesis failed: {0}")]
    Inference(String),
}

fn fmt_dirs(dirs: &[PathBuf]) -> String {
    if dirs.is_empty() {
        return "no search path".into();
    }
    dirs.iter().map(|d| d.display().to_string()).collect::<Vec<_>>().join(", ")
}
