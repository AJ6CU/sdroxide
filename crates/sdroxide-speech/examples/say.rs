//! Listen to the voice.
//!
//! Pronunciation is the one thing in this crate that cannot be asserted in a
//! test — whether "ritty" sounds like RTTY is an ear judgement. This is the
//! tool for making it, and for checking a new voice or a lexicon entry.
//!
//! ```text
//! cargo run -p sdroxide-speech --features synth --example say -- \
//!     "ðɪs ɪz ɐ tˈɛkst tə spˈiːtʃ sˈɪstəm."
//! ```
//!
//! Writes `say.wav` in the working directory. Arguments after the text are
//! `--rate <f32>` and `--voice <name>`.

use std::error::Error;
use std::time::Instant;

use sdroxide_speech::synth::{Phonemes, Synth, Voice, find_voice, list_voices, voice_dirs};

/// Phrases the announcer actually produces, for the `--check` coverage sweep.
const CORPUS: &[&str] = &[
    "fourteen point zero seven four megahertz",
    "seven point one four two three five megahertz",
    "eight hundred ten kilohertz",
    "twenty meters, upper sideband",
    "one sixty meters",
    "seventy centimeters",
    "general coverage",
    "split on, transmit seven point one eight five",
    "V F O B",
    "A G C medium",
    "A G C slow",
    "noise reduction high",
    "drive thirty five percent",
    "tune level five percent",
    "mic fifty percent",
    "S W R one point four to one",
    "high S W R, four point two to one",
    "over ten to one",
    "no S W R reading, twenty watts forward",
    "tuned, one point five to one",
    "outside the band",
    "in band, forty meters",
    "S nine plus twelve",
    "minus seventy three d B m",
    "minus twelve decibels",
    "one point five kilowatts",
    "R I T minus two hundred fifty hertz",
    "kilo one alpha bravo charlie calling you, minus twelve",
    "delta lima stroke kilo one alpha bravo charlie",
    "juliett november four seven alpha bravo",
    "five nine nine",
    "roger roger seventy three",
    "C Q C Q from whiskey one alpha whiskey, over",
    "recording started",
    "transmit",
    "receive",
    "ritty",
    "C W, fourteen point zero three zero",
    "F T eight",
    "data segment",
    "phone segment",
    "beacon segment",
    "scanning stopped",
    "memory recall, channel three",
    "connection lost",
];

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let mut text = String::new();
    let mut rate = 1.0f32;
    let mut want = String::new();
    let mut dir = None;
    let mut check = false;
    let mut play = false;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--rate" => rate = args.next().unwrap_or_default().parse()?,
            "--voice" => want = args.next().unwrap_or_default(),
            // The source-checkout fallback in `voice_dirs` is debug-only, so a
            // release build of this example needs to be pointed at the tree.
            "--dir" => dir = args.next().map(std::path::PathBuf::from),
            "--check" => check = true,
            "--play" => play = true,
            _ => text = a,
        }
    }
    if text.is_empty() {
        text = "ðɪs ɪz ɐ tˈɛkst tə spˈiːtʃ sˈɪstəm.".into();
    }

    let dirs = voice_dirs(dir);
    println!("searching: {dirs:?}");
    println!("available: {:?}", list_voices(&dirs));

    let path = find_voice(&dirs, &want)?;
    let t0 = Instant::now();
    let synth = Synth::load(Voice::load(path)?)?;
    println!("loaded {} in {:?}", synth.voice().name, t0.elapsed());

    let t0 = Instant::now();
    let g2p = Phonemes::new()?;
    println!("loaded the English dictionary in {:?}", t0.elapsed());

    if check {
        return check_coverage(&g2p, &synth);
    }

    if play {
        return play_it(&dirs, &want, &text, rate);
    }

    // Anything that is not already IPA goes through the phonemizer, so the
    // example can be handed either.
    let phonemes = if text.is_ascii() { g2p.to_ipa(&text)? } else { text.clone() };
    if phonemes != text {
        println!("phonemes: {phonemes}");
    }

    let t1 = Instant::now();
    let pcm = synth.speak_phonemes(&phonemes, rate)?;
    let elapsed = t1.elapsed();
    let rate_hz = synth.sample_rate();
    let audio_s = pcm.len() as f32 / rate_hz as f32;
    let peak = pcm.iter().fold(0.0f32, |m, s| m.max(s.abs()));
    println!(
        "{} samples, {audio_s:.2} s at {rate_hz} Hz, peak {peak:.3}, \
         synthesized in {elapsed:?} (RTF {:.3})",
        pcm.len(),
        elapsed.as_secs_f32() / audio_s.max(1e-6),
    );

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: rate_hz,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut w = hound::WavWriter::create("say.wav", spec)?;
    for s in &pcm {
        w.write_sample(*s)?;
    }
    w.finalize()?;
    println!("wrote say.wav");
    Ok(())
}

/// Phonemize every phrase the announcer can produce and check the result
/// against the voice's symbol map.
///
/// This is the join that has to hold: `piper-plus-g2p` was not built for this
/// particular voice, and a symbol it emits that the voice does not know is a
/// dropped sound rather than a loud failure. Running the real corpus through
/// it is the only way to find that out.
fn check_coverage(g2p: &Phonemes, synth: &Synth) -> Result<(), Box<dyn Error>> {
    let map = &synth.voice().config.phoneme_id_map;
    let mut unknown: std::collections::BTreeMap<char, usize> = Default::default();
    let mut used: std::collections::BTreeSet<char> = Default::default();
    let mut total = 0usize;

    for phrase in CORPUS {
        let ipa = g2p.to_ipa(phrase)?;
        for ch in ipa.chars() {
            total += 1;
            if map.contains_key(&ch) {
                used.insert(ch);
            } else {
                *unknown.entry(ch).or_default() += 1;
            }
        }
        println!("  {phrase}\n    -> {ipa}");
    }

    println!("\n{} phrases, {total} phoneme characters", CORPUS.len());
    println!("{} distinct symbols used, of {} in the voice", used.len(), map.len());
    if unknown.is_empty() {
        println!("coverage: every symbol the phonemizer produced is in the voice's map");
    } else {
        println!("UNMAPPED symbols (these would be dropped, i.e. silently missing sounds):");
        for (ch, n) in &unknown {
            println!("  {ch:?} (U+{:04X}) x{n}", *ch as u32);
        }
    }
    Ok(())
}

/// Speak through the real worker and the real sound card — the whole path the
/// app uses, including the pacing that keeps barge-in cheap.
fn play_it(
    dirs: &[std::path::PathBuf],
    want: &str,
    text: &str,
    rate: f32,
) -> Result<(), Box<dyn Error>> {
    use sdroxide_speech::sink::SpeechSink;
    use sdroxide_speech::synth::{SpeechEngine, SpeechStatus};

    let mut engine = SpeechEngine::start(dirs.to_vec(), want.to_string(), None, rate, 0.9);
    let t0 = Instant::now();
    loop {
        match engine.status() {
            SpeechStatus::Loading if t0.elapsed().as_secs() < 30 => {
                std::thread::sleep(std::time::Duration::from_millis(20))
            }
            SpeechStatus::Loading => return Err("speech backend never became ready".into()),
            SpeechStatus::Failed(e) => return Err(e.into()),
            SpeechStatus::Ready(v) => {
                println!("backend ready ({v}) in {:?}", t0.elapsed());
                break;
            }
        }
    }

    let t1 = Instant::now();
    engine.speak(text, 1);
    while engine.busy() {
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    println!("spoke in {:?}", t1.elapsed());
    Ok(())
}
