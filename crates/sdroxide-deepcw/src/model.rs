//! The network itself.
//!
//! A Conformer-CTC of about 3.6 M parameters, shipped as ONNX beside this file
//! and run by `rten`, which reads ONNX directly and is pure Rust — the same
//! runtime the speech synthesiser already uses, so this adds a model rather than
//! a toolchain.
//!
//! The weights are `include_bytes!`d and handed to rten as a `'static` slice, so
//! the 15 MB lives in the binary's read-only data and is never copied onto the
//! heap. Loading is a graph walk over that slice and takes a few milliseconds;
//! one process-wide instance is shared because the graph is immutable during a
//! run and rten takes `&self`.

use std::sync::Arc;
use std::sync::OnceLock;

use rten::{Model, RunOptions, ThreadPool, ValueOrView};
use rten_tensor::NdTensor;
use rten_tensor::prelude::*;

use crate::ctc::{self, CLASSES, Decoded};
use crate::spectrogram::{self, BINS, Spectrogram};

/// The trained model. AGPL-3.0-only, from https://github.com/e04/deepcw-engine.
static WEIGHTS: &[u8] = include_bytes!("../assets/model.onnx");

static SHARED: OnceLock<Result<Network, String>> = OnceLock::new();

/// Anything that stops us decoding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// The embedded model would not load. Not recoverable: the bytes are fixed
    /// at compile time, so this means the build is wrong, not the input.
    Load(String),
    /// One inference failed. The next one may not.
    Run(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Load(e) => write!(f, "loading the DeepCW model: {e}"),
            Error::Run(e) => write!(f, "running the DeepCW model: {e}"),
        }
    }
}

impl std::error::Error for Error {}

/// The loaded graph, shared by every decoder in the process.
struct Network {
    model: Model,
    input: rten::NodeId,
    output: rten::NodeId,
}

// rten runs a graph through `&self` and keeps no per-run state on the model, so
// one instance serves every worker thread.
const _: () = {
    const fn assert_shareable<T: Send + Sync>() {}
    assert_shareable::<Model>();
};

fn shared() -> Result<&'static Network, Error> {
    SHARED
        .get_or_init(|| {
            let started = std::time::Instant::now();
            let model = Model::load_static_slice(WEIGHTS).map_err(|e| e.to_string())?;
            let input = *model.input_ids().first().ok_or("model has no input")?;
            let output = *model.output_ids().first().ok_or("model has no output")?;
            tracing::debug!(
                bytes = WEIGHTS.len(),
                elapsed_ms = started.elapsed().as_secs_f32() * 1e3,
                "loaded DeepCW model"
            );
            Ok(Network { model, input, output })
        })
        .as_ref()
        .map_err(|e| Error::Load(e.clone()))
}

/// Loads the model now rather than on the first decode, so a broken build shows
/// up at startup instead of the first time somebody tunes to CW.
pub fn preload() -> Result<(), Error> {
    shared().map(|_| ())
}

/// What to decode: audio we analyse ourselves, or a spectrogram the caller
/// already has.
///
/// The skimmer is the reason for the second form. It runs one wide transform
/// across the whole band and lifts each station's 65 bins straight out of it,
/// which costs a memcpy per signal instead of a filter chain per signal. The
/// bins have to be on the model's grid — [`crate::BIN_HZ`] apart, from a
/// [`crate::WINDOW_S`] window, scaled as though the audio had been sampled at
/// [`SAMPLE_RATE`] — and then it is the same tensor either way.
#[derive(Debug, Clone, PartialEq)]
pub enum Window {
    /// Mono at [`spectrogram::SAMPLE_RATE`], signal inside 400–1200 Hz.
    Audio(Vec<f32>),
    /// A `[frames][BINS]` row-major `log1p` magnitude spectrogram.
    Spectrogram(Vec<f32>),
}

impl Window {
    fn is_empty(&self) -> bool {
        match self {
            Window::Audio(a) => a.len() < spectrogram::MIN_SAMPLES,
            Window::Spectrogram(s) => s.len() < BINS,
        }
    }
}

/// One decoder: an STFT front end plus a handle on the shared network.
///
/// Cheap to make and not shared between threads — give each worker its own. A
/// decode is stateless, so the same `Decoder` can serve unrelated signals.
pub struct Decoder {
    net: &'static Network,
    spectrogram: Spectrogram,
    /// Where inference runs, or the process-wide pool if `None`.
    pool: Option<Arc<ThreadPool>>,
}

impl Decoder {
    /// A decoder that runs on rten's process-wide pool, which is sized to the
    /// physical core count. Right for a caller decoding one signal at a time,
    /// where finishing sooner is the whole point.
    pub fn new() -> Result<Self, Error> {
        Ok(Decoder { net: shared()?, spectrogram: Spectrogram::new(), pool: None })
    }

    /// A decoder that runs on `pool` instead.
    ///
    /// rten parallelises *within* one inference over a global pool sized to the
    /// machine, and it has no idea how many other inferences are in flight. Run
    /// several concurrently — which is exactly what a skimmer does — and each one
    /// asks for every core, so four decodes on a sixteen-core machine put
    /// sixty-four-way demand on it and spend real time contending instead of
    /// decoding. Handing every worker the *same* explicitly sized pool caps the
    /// total at that size however many decodes are running, and still lets a lone
    /// decode on a quiet band spread across all of it.
    pub fn with_thread_pool(pool: Arc<ThreadPool>) -> Result<Self, Error> {
        Ok(Decoder { net: shared()?, spectrogram: Spectrogram::new(), pool: Some(pool) })
    }

    /// Decode `audio`, which must be mono at [`spectrogram::SAMPLE_RATE`] with
    /// the signal inside 400–1200 Hz. Too-short input decodes to nothing.
    pub fn decode(&mut self, audio: &[f32]) -> Result<Decoded, Error> {
        let spec = self.spectrogram.compute(audio);
        self.run(spec)
    }

    /// Decode either form of input.
    pub fn decode_window(&mut self, window: &Window) -> Result<Decoded, Error> {
        if window.is_empty() {
            return Ok(Decoded::default());
        }
        match window {
            Window::Audio(audio) => self.decode(audio),
            // Trim any partial row rather than letting it reshape the tensor.
            Window::Spectrogram(spec) => self.run(spec[..spec.len() / BINS * BINS].to_vec()),
        }
    }

    fn run(&mut self, spec: Vec<f32>) -> Result<Decoded, Error> {
        if spec.is_empty() {
            return Ok(Decoded::default());
        }
        let frames = spec.len() / BINS;
        let input = NdTensor::from_data([1, 1, frames, BINS], spec);

        let opts = self.pool.clone().map(|p| RunOptions::default().with_thread_pool(Some(p)));
        let mut outputs = self
            .net
            .model
            .run(vec![(self.net.input, ValueOrView::from(input.view()))], &[self.net.output], opts)
            .map_err(|e| Error::Run(e.to_string()))?;

        let value = outputs.pop().ok_or_else(|| Error::Run("model returned no output".into()))?;
        let log_probs: NdTensor<f32, 3> =
            value.try_into().map_err(|_| Error::Run("unexpected output tensor".into()))?;

        let out_frames = log_probs.size(1);
        debug_assert_eq!(log_probs.size(2), CLASSES);
        Ok(ctc::greedy(&log_probs.to_vec(), out_frames))
    }

    /// Seconds of audio one frame of the decoded spans covers.
    pub fn frame_seconds(&self) -> f64 {
        spectrogram::FRAME_S
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_signal;

    #[test]
    fn the_embedded_model_loads_and_reports_its_shape() {
        let net = shared().expect("model loads");
        let info = net.model.node_info(net.input).expect("input info");
        assert_eq!(info.name(), Some("spectrogram"));
        // [batch, 1, time, 65] — batch and time stay dynamic.
        let shape = info.shape().expect("input shape");
        assert_eq!(shape.len(), 4);
        let out = net.model.node_info(net.output).expect("output info");
        assert_eq!(out.name(), Some("log_probs"));
    }

    #[test]
    fn decodes_a_clean_signal() {
        let mut d = Decoder::new().expect("decoder");
        let audio = test_signal::render("CQ CQ DE W1AW W1AW K", 25.0, 800.0, 30.0, 1);
        assert_eq!(d.decode(&audio).unwrap().normalized(), "CQ CQ DE W1AW W1AW K");
    }

    #[test]
    fn silence_decodes_to_nothing() {
        let mut d = Decoder::new().expect("decoder");
        let audio = vec![0.0f32; (spectrogram::SAMPLE_RATE * 6.0) as usize];
        assert_eq!(d.decode(&audio).unwrap().normalized(), "");
    }

    #[test]
    fn too_little_audio_decodes_to_nothing() {
        let mut d = Decoder::new().expect("decoder");
        assert_eq!(d.decode(&[0.0; 32]).unwrap(), Decoded::default());
    }

    const MSG: &str = "CQ CQ DE W1AW W1AW K UR RST 599 599 QTH BOSTON MA 73";
    const MSG_MEDIUM: &str = "CQ CQ DE W1AW W1AW K UR 599";
    const MSG_SHORT: &str = "CQ DE W1AW K";

    /// A message that renders inside the 5–20 s the model documents. Sending the
    /// long one at 12 WPM takes the better part of a minute, and accuracy falls
    /// off well before that — which is why [`crate::Stream`] caps its window.
    fn message_for(wpm: f32) -> &'static str {
        match wpm {
            w if w < 16.0 => MSG_SHORT,
            w if w < 22.0 => MSG_MEDIUM,
            _ => MSG,
        }
    }

    fn copy_msg(msg: &str, wpm: f32, pitch: f64, snr_db: f32, seed: u64) -> String {
        let mut d = Decoder::new().expect("decoder");
        let audio = test_signal::render(msg, wpm, pitch, snr_db, seed);
        d.decode(&audio).expect("decode").normalized()
    }

    fn copy(wpm: f32, pitch: f64, snr_db: f32, seed: u64) -> String {
        copy_msg(message_for(wpm), wpm, pitch, snr_db, seed)
    }

    #[test]
    fn copies_across_the_speed_range() {
        // 12 to 40 WPM covers everything from a careful beginner to contest
        // speed. The model reads timing rather than fitting it, so none of this
        // needs a speed setting.
        for (i, wpm) in [12.0, 18.0, 25.0, 32.0, 40.0].into_iter().enumerate() {
            let sent = message_for(wpm);
            let got = copy(wpm, 800.0, 20.0, i as u64 + 1);
            let acc = test_signal::accuracy(sent, &got);
            assert!(acc > 0.95, "{wpm} WPM copied {acc:.2}\n sent {sent:?}\n  got {got:?}");
        }
    }

    #[test]
    fn copies_anywhere_in_the_band() {
        // The band is 400–1200 Hz and the model was trained across it, so a
        // front end that lands the tone anywhere inside is good enough.
        for (i, pitch) in [450.0, 600.0, 800.0, 1000.0, 1150.0].into_iter().enumerate() {
            let got = copy(25.0, pitch, 20.0, i as u64 + 10);
            let acc = test_signal::accuracy(MSG, &got);
            assert!(acc > 0.95, "{pitch} Hz copied {acc:.2}\n sent {MSG:?}\n  got {got:?}");
        }
    }

    #[test]
    fn copies_down_to_low_signal_to_noise() {
        // SNR is referred to 2500 Hz, the reference the DeepCW benchmarks use.
        // These are the numbers this port actually reaches; they are here to
        // catch a regression in the front end, which is the part that can drift.
        for (snr_db, want) in [(20.0, 0.99), (10.0, 0.99), (3.0, 0.99), (0.0, 0.97), (-6.0, 0.95)] {
            let got = copy(25.0, 800.0, snr_db, 42);
            let acc = test_signal::accuracy(MSG, &got);
            assert!(acc >= want, "{snr_db} dB copied {acc:.2}, wanted {want}\n  got {got:?}");
        }
    }

    #[test]
    fn noise_alone_decodes_to_almost_nothing() {
        let mut d = Decoder::new().expect("decoder");
        let mut rng = test_signal::Rng(5);
        let audio: Vec<f32> =
            (0..(spectrogram::SAMPLE_RATE * 10.0) as usize).map(|_| rng.normal() * 0.1).collect();
        let got = d.decode(&audio).expect("decode").normalized();
        assert!(got.len() < 8, "noise decoded to {got:?}");
    }

    /// CPU consumed by this process and every thread in it, in seconds. See the
    /// note on the skimmer's own cost harness: wall time cannot tell 25 ms on one
    /// core from 25 ms on sixteen, and the difference is the whole question.
    #[cfg(target_os = "linux")]
    fn cpu_seconds() -> f64 {
        let stat = std::fs::read_to_string("/proc/self/stat").unwrap_or_default();
        let after = stat.rfind(')').map_or("", |i| &stat[i + 2..]);
        let f: Vec<&str> = after.split_whitespace().collect();
        let at = |i: usize| -> f64 { f.get(i).and_then(|v| v.parse().ok()).unwrap_or(0.0) };
        (at(11) + at(12)) / 100.0
    }

    #[cfg(not(target_os = "linux"))]
    fn cpu_seconds() -> f64 {
        f64::NAN
    }

    /// What one inference costs, by window length and by how many cores it is
    /// allowed to use.
    ///
    /// The first table sizes the window trade-off: the model has no time
    /// subsampling, so cost is linear in window length with the attention term
    /// growing on top of that. The second sizes the fan-out: if CPU per decode
    /// climbs with pool width while wall time stops falling, the extra threads
    /// are contending rather than working, and a skimmer running several decodes
    /// at once is paying for it several times over.
    #[test]
    #[ignore = "a measurement, not an assertion; needs --release to mean anything"]
    fn inference_cost_by_window_and_threads() {
        use std::sync::Arc;

        let mut text = String::new();
        for _ in 0..4 {
            text.push_str(MSG);
            text.push(' ');
        }
        let audio = test_signal::render(&text, 25.0, 800.0, 20.0, 7);
        let window = |secs: f64| -> Window {
            let n = (spectrogram::SAMPLE_RATE * secs) as usize;
            let from = audio.len().saturating_sub(n);
            Window::Spectrogram(Spectrogram::new().compute(&audio[from..]))
        };

        let time = |d: &mut Decoder, w: &Window| -> (f64, f64) {
            for _ in 0..3 {
                d.decode_window(w).expect("decode");
            }
            let cpu0 = cpu_seconds();
            let wall = std::time::Instant::now();
            const N: u32 = 8;
            for _ in 0..N {
                d.decode_window(w).expect("decode");
            }
            let wall = wall.elapsed().as_secs_f64() / N as f64;
            ((cpu_seconds() - cpu0) / N as f64, wall)
        };

        let mut by_window = Vec::new();
        for secs in [5.0, 8.0, 10.0, 12.0, 20.0] {
            let w = window(secs);
            let frames = match &w {
                Window::Spectrogram(s) => s.len() / BINS,
                _ => 0,
            };
            let mut d = Decoder::with_thread_pool(Arc::new(ThreadPool::with_num_threads(1)))
                .expect("decoder");
            let (cpu, wall) = time(&mut d, &w);
            by_window.push((secs, frames, wall, cpu));
        }
        let base = by_window.iter().find(|r| r.0 == 12.0).map_or(1.0, |r| r.3);
        println!("\n  one thread, by window length:");
        println!(
            "{:>9}{:>9}{:>11}{:>11}{:>10}",
            "window", "frames", "wall ms", "cpu ms", "vs 12 s"
        );
        for (secs, frames, wall, cpu) in by_window {
            println!(
                "{secs:>8.0}s{frames:>9}{:>11.1}{:>11.1}{:>10.2}",
                wall * 1e3,
                cpu * 1e3,
                cpu / base
            );
        }

        println!("\n  12 s window, by inference pool width:");
        println!("{:>9}{:>11}{:>11}{:>13}", "threads", "wall ms", "cpu ms", "efficiency");
        let w = window(12.0);
        for n in [1usize, 2, 4, 8, 16] {
            let mut d = Decoder::with_thread_pool(Arc::new(ThreadPool::with_num_threads(n)))
                .expect("decoder");
            let (cpu, wall) = time(&mut d, &w);
            println!(
                "{n:>9}{:>11.1}{:>11.1}{:>13.2}",
                wall * 1e3,
                cpu * 1e3,
                cpu / wall / n as f64
            );
        }
        let mut d = Decoder::new().expect("decoder");
        let (cpu, wall) = time(&mut d, &w);
        println!("{:>9}{:>11.1}{:>11.1}", "global", wall * 1e3, cpu * 1e3);
    }

    /// How much of a transmission the model still copies when it is given less
    /// of it to look at.
    ///
    /// The skimmer holds a rolling window per station and re-reads the whole
    /// thing each time, so the window length sets both the cost and how much
    /// context the model gets. `deepcw_matrix` cannot answer this — it always
    /// feeds a whole rendered message — so this scores the *trailing* N seconds
    /// against the part of the text that actually falls inside them, which is
    /// what a rolling buffer holds.
    #[test]
    #[ignore = "reporting harness, not a pass/fail test"]
    fn accuracy_by_window_length() {
        const WINDOWS: [f64; 7] = [5.0, 6.0, 8.0, 10.0, 12.0, 16.0, 20.0];
        let snrs = [20.0f32, 10.0, 3.0, 0.0, -6.0, -9.0];
        // Long enough that even the widest window is a strict tail of it.
        let mut text = String::new();
        for _ in 0..3 {
            text.push_str(MSG);
            text.push(' ');
        }
        let sent = text.trim();

        println!("\n  copy accuracy on the trailing N seconds, 25 WPM");
        print!("{:>8}", "SNR dB");
        for w in WINDOWS {
            print!("{w:>8.0}s");
        }
        println!();
        for snr in snrs {
            let audio = test_signal::render(sent, 25.0, 800.0, snr, 11);
            print!("{snr:>8.0}");
            for w in WINDOWS {
                let n = (spectrogram::SAMPLE_RATE * w) as usize;
                let from = audio.len().saturating_sub(n);
                let mut d = Decoder::new().expect("decoder");
                let got = d.decode(&audio[from..]).expect("decode").normalized();
                // Where the window cuts into the text is not known in advance, so
                // score against whichever tail of the message it matches best.
                // Guessing the cut from the ratio of durations instead puts a
                // character or two of alignment error into every cell and buries
                // the effect being measured.
                let chars: Vec<char> = sent.chars().collect();
                let best = (0..chars.len())
                    .map(|cut| {
                        let want: String = chars[cut..].iter().collect();
                        test_signal::accuracy(want.trim(), &got)
                    })
                    .fold(0.0f32, f32::max);
                print!("{best:>9.2}");
            }
            println!();
        }
        println!("  (ratios of the 12 s column are what a shorter skimmer window costs)");
    }

    /// Prints the speed × SNR accuracy surface. Not a pass/fail test — run it
    /// when tuning a front end:
    /// `cargo test -p sdroxide-deepcw deepcw_matrix -- --ignored --nocapture`
    #[test]
    #[ignore = "reporting harness, not a pass/fail test"]
    fn deepcw_matrix() {
        let snrs = [20.0, 10.0, 6.0, 3.0, 0.0, -3.0, -6.0, -9.0, -12.0];
        print!("{:>6}", "WPM");
        for s in snrs {
            print!("{s:>8.0}");
        }
        println!("  dB SNR (2500 Hz)");
        for wpm in [12.0, 18.0, 25.0, 32.0, 40.0] {
            print!("{wpm:>6.0}");
            for snr in snrs {
                let acc = test_signal::accuracy(message_for(wpm), &copy(wpm, 800.0, snr, 42));
                print!("{acc:>8.2}");
            }
            println!();
        }
    }
}
