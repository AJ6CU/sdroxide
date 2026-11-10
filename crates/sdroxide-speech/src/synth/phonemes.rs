//! Text to IPA phonemes.
//!
//! Piper voices are trained on eSpeak's IPA, so a voice's `phoneme_id_map` is
//! an eSpeak symbol inventory. We reach it without eSpeak: `piper-plus-g2p` is
//! CMUdict plus an ARPAbet-to-IPA conversion, pure Rust, and needs no system
//! library.
//!
//! The dictionary is **embedded** rather than looked up on disk. `piper-plus-g2p`
//! searches for a `cmudict_data.json` in the working directory, an environment
//! variable and `/usr/share/piper`, none of which is a thing sdroxide can rely
//! on being there; `EnglishPhonemizer::new_with_hashmap` takes the table
//! directly, so the program carries its own. CMUdict is BSD-2-Clause, which
//! sits inside GPL-3.0-or-later without trouble — see `assets/LICENSE-cmudict.txt`.
//!
//! Three things guard the join to the voice:
//!
//! - the [`crate::lexicon`] table is consulted first, so the vocabulary an
//!   operator hears hundreds of times a day — the NATO alphabet, unit names,
//!   mode names — is pronounced from a table we control rather than from
//!   whatever CMUdict happens to say;
//! - words in neither the lexicon nor the dictionary are **spelled out** rather
//!   than dropped. `piper-plus-g2p` silently emits nothing for a word it does
//!   not know, and a silently missing word in a decoded message is worse than
//!   a slowly spelled one — which for an unrecognised token on the air, most
//!   often a callsign, is usually the right reading anyway;
//! - everything arriving here has been through `Speaker::sanitize`, so it is
//!   letters and a little punctuation. No digits, no symbols, nothing the
//!   dictionary would have no entry for by construction.

use std::collections::HashMap;
use std::sync::OnceLock;

use piper_plus_g2p::english::EnglishPhonemizer;
use piper_plus_g2p::phonemizer::Phonemizer;

use super::SynthError;

/// CMUdict, trimmed: lowercased, alternate pronunciations dropped, comments
/// stripped. Around 126 000 entries and 3.3 MB, which is small beside the
/// voice model and buys a program that never has to find a file to talk.
const CMUDICT: &str = include_str!("../../assets/cmudict.dict");

/// Parsed once. Both the phonemizer and our own out-of-vocabulary check read
/// it, so it is built here rather than handed to the phonemizer and forgotten.
fn cmudict() -> &'static HashMap<String, String> {
    static DICT: OnceLock<HashMap<String, String>> = OnceLock::new();
    DICT.get_or_init(|| {
        let mut m = HashMap::with_capacity(128 * 1024);
        for line in CMUDICT.lines() {
            if let Some((word, arpa)) = line.split_once(' ') {
                m.insert(word.to_string(), arpa.to_string());
            }
        }
        m
    })
}

/// English grapheme-to-phoneme, with sdroxide's own lexicon in front of it.
pub struct Phonemes {
    en: EnglishPhonemizer,
}

impl Phonemes {
    pub fn new() -> Result<Self, SynthError> {
        let dict = cmudict();
        if dict.is_empty() {
            return Err(SynthError::Inference("the embedded CMU dictionary is empty".into()));
        }
        Ok(Phonemes { en: EnglishPhonemizer::new_with_hashmap(dict.clone()) })
    }

    /// Convert one already-sanitized phrase to a string of IPA characters.
    pub fn to_ipa(&self, text: &str) -> Result<String, SynthError> {
        let mut out = String::new();
        for word in text.split_whitespace() {
            // Punctuation carries prosody in Piper — a comma or a full stop is
            // in the phoneme map and shapes the phrase — so it is split off the
            // ends and kept, rather than handed to the dictionary.
            let is_word = |c: char| c.is_alphabetic() || c == '\'';
            let lead_len = word.len() - word.trim_start_matches(|c| !is_word(c)).len();
            let (lead, rest) = word.split_at(lead_len);
            let core_len = rest.trim_end_matches(|c| !is_word(c)).len();
            let (core, tail) = rest.split_at(core_len);

            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(lead);
            if core.is_empty() {
                out.push_str(tail);
                continue;
            }
            out.push_str(&self.word_to_ipa(core)?);
            out.push_str(tail);
        }
        Ok(out)
    }

    /// One word: lexicon, then dictionary, then spelled out.
    fn word_to_ipa(&self, word: &str) -> Result<String, SynthError> {
        if let Some(ipa) = crate::lexicon::lookup(word) {
            return Ok(ipa.to_string());
        }
        let lower = word.to_ascii_lowercase();
        if !cmudict().contains_key(&lower) && !is_derived(&lower) {
            return self.spell_out(word);
        }
        Ok(self.phonemize(&lower)?)
    }

    /// Read a word one letter at a time — "K1ABC" having already become
    /// "kilo one alpha bravo charlie" upstream, this catches what is left:
    /// misdecoded RTTY, an unusual name, a mode nobody taught us.
    fn spell_out(&self, word: &str) -> Result<String, SynthError> {
        let mut parts: Vec<String> = Vec::new();
        for ch in word.chars() {
            let name = letter_name(ch);
            if name.is_empty() {
                continue;
            }
            parts.push(match crate::lexicon::lookup(name) {
                Some(ipa) => ipa.to_string(),
                None => self.phonemize(name)?,
            });
        }
        // Joined with spaces so the voice puts a beat between letters instead
        // of running them into one unintelligible word.
        Ok(parts.join(" "))
    }

    fn phonemize(&self, word: &str) -> Result<String, SynthError> {
        let (tokens, _prosody) = self
            .en
            .phonemize_with_prosody(word)
            .map_err(|e| SynthError::Inference(format!("phonemizing {word:?}: {e}")))?;
        Ok(tokens.concat())
    }
}

/// Whether `piper-plus-g2p`'s morphological fallback is likely to reach a base
/// form for this word — it strips a plural, past tense or gerund and looks the
/// stem up again. Checking the same suffixes here keeps us from spelling out a
/// word the phonemizer would in fact have pronounced.
fn is_derived(word: &str) -> bool {
    const SUFFIXES: [&str; 6] = ["s", "es", "ed", "ing", "ly", "er"];
    let dict = cmudict();
    SUFFIXES.iter().any(|suf| {
        word.strip_suffix(suf).is_some_and(|stem| {
            stem.len() >= 3
                && (dict.contains_key(stem)
                    // "making" -> "mak" -> "make"
                    || dict.contains_key(&format!("{stem}e")))
        })
    })
}

/// The name of a letter, as a word the dictionary or the lexicon can speak.
///
/// `w` and `y` are the ones that matter: CMUdict has "double" and "u" but not
/// "w", and a callsign read as a gap is a callsign lost.
fn letter_name(ch: char) -> &'static str {
    match ch.to_ascii_lowercase() {
        'a' => "ay",
        'b' => "bee",
        'c' => "see",
        'd' => "dee",
        'e' => "ee",
        'f' => "eff",
        'g' => "gee",
        'h' => "aitch",
        'i' => "eye",
        'j' => "jay",
        'k' => "kay",
        'l' => "el",
        'm' => "em",
        'n' => "en",
        'o' => "oh",
        'p' => "pee",
        'q' => "cue",
        'r' => "are",
        's' => "ess",
        't' => "tee",
        'u' => "you",
        'v' => "vee",
        'w' => "doubleyou",
        'x' => "ex",
        'y' => "why",
        'z' => "zee",
        _ => "",
    }
}
