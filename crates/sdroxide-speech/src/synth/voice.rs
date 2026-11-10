//! Finding a Piper voice on disk, and reading the JSON that describes it.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use super::SynthError;

/// The voice sdroxide ships. Others may be dropped into the speech voice
/// directory and named in the settings.
pub const DEFAULT_VOICE: &str = "en_US-hfc_female-medium";

/// A voice model plus the config that goes with it.
#[derive(Debug, Clone)]
pub struct Voice {
    /// Basename without extension, e.g. `en_US-hfc_female-medium`.
    pub name: String,
    /// The `.onnx` file.
    pub model: PathBuf,
    pub config: VoiceConfig,
}

/// The parts of a Piper `.onnx.json` we act on.
///
/// Piper writes considerably more than this — dataset provenance, the training
/// language, the espeak voice used to phonemize the corpus. `#[serde(default)]`
/// is deliberately *not* set: a voice whose config is missing `phoneme_id_map`
/// cannot be spoken with, and failing at load is better than synthesizing
/// silence.
#[derive(Debug, Clone, Deserialize)]
pub struct VoiceConfig {
    pub audio: AudioConfig,
    pub inference: InferenceConfig,
    /// IPA phoneme character -> model input IDs. Around 150 entries; symbols
    /// outside it cannot be spoken and are dropped with a warning.
    pub phoneme_id_map: HashMap<char, Vec<i32>>,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct AudioConfig {
    /// Rate the model generates at — 22050 for `medium`, 16000 for `low`.
    /// Everything downstream resamples from this to the sound card's rate.
    pub sample_rate: u32,
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub struct InferenceConfig {
    pub noise_scale: f32,
    /// The voice's natural tempo. Dividing this by the operator's speech-rate
    /// setting is how the rate control works: the model stretches or
    /// compresses the phoneme durations itself, so there is no resampling and
    /// no pitch shift.
    pub length_scale: f32,
    pub noise_w: f32,
}

impl Voice {
    /// Load the `.onnx.json` beside `model`.
    pub fn load(model: PathBuf) -> Result<Self, SynthError> {
        let name = model.file_stem().and_then(|s| s.to_str()).unwrap_or(DEFAULT_VOICE).to_string();
        // Piper names it `<voice>.onnx.json`, i.e. appended to the whole
        // filename rather than replacing the extension.
        let mut cfg_path = model.clone().into_os_string();
        cfg_path.push(".json");
        let cfg_path = PathBuf::from(cfg_path);
        let json = std::fs::read_to_string(&cfg_path)
            .map_err(|e| SynthError::VoiceConfig(cfg_path.clone(), e.to_string()))?;
        let config: VoiceConfig = serde_json::from_str(&json)
            .map_err(|e| SynthError::VoiceConfig(cfg_path, e.to_string()))?;
        Ok(Voice { name, model, config })
    }
}

/// Every directory a voice might live in, most specific first.
///
/// The list covers the three ways sdroxide is run: from a package install, from
/// a portable directory next to the executable, and from a source checkout.
/// Operator-supplied voices come first so a hand-picked voice always wins over
/// the shipped one.
pub fn voice_dirs(user_dir: Option<PathBuf>) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Some(d) = user_dir {
        dirs.push(d);
    }
    if let Ok(exe) = std::env::current_exe()
        && let Some(bin) = exe.parent()
    {
        dirs.push(bin.join("voices"));
        // macOS .app bundle: Contents/MacOS/sdroxide -> Contents/Resources.
        if let Some(contents) = bin.parent() {
            dirs.push(contents.join("Resources").join("voices"));
        }
    }
    #[cfg(target_os = "linux")]
    dirs.push(PathBuf::from("/usr/share/sdroxide/voices"));
    // Running from a source checkout: crates/sdroxide-speech/../../assets.
    // Debug builds only — a released binary must never reach into a build tree.
    if cfg!(debug_assertions) {
        dirs.push(
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("..")
                .join("..")
                .join("assets")
                .join("voices"),
        );
    }
    dirs
}

/// Voice basenames found across `dirs`, deduplicated, first occurrence winning.
///
/// A voice is a `.onnx` with a `.onnx.json` beside it; a lone model file is not
/// usable and is not listed.
pub fn list_voices(dirs: &[PathBuf]) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    for dir in dirs {
        let Ok(rd) = std::fs::read_dir(dir) else {
            continue;
        };
        for entry in rd.flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|e| e != "onnx") {
                continue;
            }
            let mut cfg = path.clone().into_os_string();
            cfg.push(".json");
            if !Path::new(&cfg).is_file() {
                continue;
            }
            if let Some(name) = path.file_stem().and_then(|s| s.to_str())
                && !out.iter().any(|n| n == name)
            {
                out.push(name.to_string());
            }
        }
    }
    out.sort();
    out
}

/// Resolve `want` (or, when empty, the shipped voice and then whatever is
/// available) to a model file.
pub fn find_voice(dirs: &[PathBuf], want: &str) -> Result<PathBuf, SynthError> {
    let mut names: Vec<String> = Vec::new();
    if !want.is_empty() {
        names.push(want.to_string());
    }
    names.push(DEFAULT_VOICE.to_string());

    for name in &names {
        for dir in dirs {
            let model = dir.join(format!("{name}.onnx"));
            let mut cfg = model.clone().into_os_string();
            cfg.push(".json");
            if model.is_file() && Path::new(&cfg).is_file() {
                return Ok(model);
            }
        }
    }
    // Nothing named; fall back to the first complete voice anywhere.
    if let Some(any) = list_voices(dirs).first() {
        for dir in dirs {
            let model = dir.join(format!("{any}.onnx"));
            if model.is_file() {
                return Ok(model);
            }
        }
    }
    Err(SynthError::NoVoice {
        want: if want.is_empty() { DEFAULT_VOICE.into() } else { want.into() },
        dirs: dirs.to_vec(),
    })
}
