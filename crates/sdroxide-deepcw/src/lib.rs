//! DeepCW — a neural CW decoder.
//!
//! A Rust port of [DeepCW](https://github.com/e04/deepcw-engine), carrying its
//! trained Conformer-CTC model. Where the classic decoder in `sdroxide-dsp`
//! measures an envelope and fits a timing model to it, this one reads an 800 Hz
//! slice of the spectrogram and predicts characters directly, which is why it
//! keeps copying several dB below where a timing fit falls apart, and why it
//! copies hand-sent CW whose timing no fit would accept.
//!
//! **This crate is AGPL-3.0-only**, unlike the rest of the workspace. Both the
//! model and the reference implementation it was ported from are AGPL, and §13
//! reaches anything that links it and is offered over a network.
//!
//! The shape of the thing is different from a streaming decoder and the callers
//! have to be built around that. The model consumes a *window* — several seconds
//! of audio at once — and returns the whole transcript for it. There is no
//! incremental "here are the characters that just completed". Two layers deal
//! with that:
//!
//! * [`Decoder`] is one window in, one transcript out. Everything else is built
//!   on it.
//! * [`Stream`] keeps a rolling buffer, re-decodes it as audio arrives, and
//!   splits the result into text that has stopped changing and a tail that has
//!   not. That is what a receive window wants.
//!
//! Inference costs roughly 25 ms for a 10 s window, so neither layer should be
//! driven from an audio callback. [`Worker`] and [`Pool`] move it onto threads.

mod ctc;
mod model;
mod pool;
mod spectrogram;
mod stream;
mod tuner;

pub use ctc::{BLANK, CHARS, CharSpan, Decoded, FrameSpan};
pub use model::{Decoder, Error, Window, preload};
pub use pool::{Job, Pool};
/// rten's thread pool, re-exported so callers can bound where inference runs
/// without depending on rten themselves. See [`Decoder::with_thread_pool`].
pub use rten::ThreadPool;
pub use spectrogram::{
    BIN_HZ, BINS, CENTER_BIN, CENTER_FREQ_HZ, FRAME_S, MIN_SAMPLES, SAMPLE_RATE, WINDOW_S,
};
pub use stream::{Stream, Update, Worker};
pub use tuner::Tuner;

/// Synthetic CW, shared by the tests in every module here.
///
/// Not a channel simulator — there is one in `sdroxide-dsp`'s integration tests
/// for that. This renders a keyed tone with additive noise, which is enough to
/// tell "the port matches the reference" from "the port is broken".
#[cfg(test)]
pub(crate) mod test_signal {
    /// Morse table, enough of it for callsigns and exchanges.
    const TABLE: [(char, &str); 41] = [
        ('A', ".-"),
        ('B', "-..."),
        ('C', "-.-."),
        ('D', "-.."),
        ('E', "."),
        ('F', "..-."),
        ('G', "--."),
        ('H', "...."),
        ('I', ".."),
        ('J', ".---"),
        ('K', "-.-"),
        ('L', ".-.."),
        ('M', "--"),
        ('N', "-."),
        ('O', "---"),
        ('P', ".--."),
        ('Q', "--.-"),
        ('R', ".-."),
        ('S', "..."),
        ('T', "-"),
        ('U', "..-"),
        ('V', "...-"),
        ('W', ".--"),
        ('X', "-..-"),
        ('Y', "-.--"),
        ('Z', "--.."),
        ('0', "-----"),
        ('1', ".----"),
        ('2', "..---"),
        ('3', "...--"),
        ('4', "....-"),
        ('5', "....."),
        ('6', "-...."),
        ('7', "--..."),
        ('8', "---.."),
        ('9', "----."),
        ('/', "-..-."),
        ('?', "..--.."),
        ('.', ".-.-.-"),
        (',', "--..--"),
        (' ', ""),
    ];

    fn elements(text: &str) -> Vec<(bool, f32)> {
        let mut out = Vec::new();
        for ch in text.chars() {
            if ch == ' ' {
                // Word gap is seven dits; four more on top of the three that
                // already follow the previous character.
                out.push((false, 4.0));
                continue;
            }
            let Some((_, code)) = TABLE.iter().find(|(c, _)| *c == ch) else { continue };
            for e in code.chars() {
                out.push((true, if e == '.' { 1.0 } else { 3.0 }));
                out.push((false, 1.0));
            }
            out.push((false, 2.0));
        }
        out
    }

    /// A keyed tone at `pitch_hz`, `wpm` words per minute, with white noise at
    /// `snr_db` referred to a 2500 Hz bandwidth — the reference the DeepCW
    /// benchmarks use. Half a second of noise leads and trails the message.
    pub fn render(text: &str, wpm: f32, pitch_hz: f64, snr_db: f32, seed: u64) -> Vec<f32> {
        let rate = super::SAMPLE_RATE;
        let dit = 1.2 / wpm as f64;

        let mut key: Vec<f32> = Vec::new();
        for (on, units) in elements(text) {
            let n = (units as f64 * dit * rate).round() as usize;
            key.extend(std::iter::repeat_n(if on { 1.0f32 } else { 0.0 }, n));
        }

        // Raised-cosine edges. A hard-keyed square edge is a click that splatters
        // across the whole band the model looks at.
        let ramp = (0.005 * rate) as usize;
        let smooth: Vec<f32> = (0..key.len())
            .map(|i| {
                let lo = i.saturating_sub(ramp);
                let hi = (i + ramp + 1).min(key.len());
                let mut sum = 0.0;
                let mut norm = 0.0;
                for (j, v) in key[lo..hi].iter().enumerate() {
                    let d = (lo + j) as f32 - i as f32;
                    let w = 0.5 * (1.0 + (std::f32::consts::PI * d / (ramp as f32 + 1.0)).cos());
                    sum += v * w;
                    norm += w;
                }
                sum / norm.max(1e-9)
            })
            .collect();

        let lead = (0.5 * rate) as usize;
        let mut out = vec![0.0f32; lead + smooth.len() + lead];
        for (i, k) in smooth.iter().enumerate() {
            let t = i as f64 / rate;
            out[lead + i] = k * (std::f64::consts::TAU * pitch_hz * t).sin() as f32;
        }

        let key_power: f32 = smooth.iter().map(|k| k * k).sum::<f32>() / smooth.len().max(1) as f32;
        let signal = key_power * 0.5;
        let noise = signal / 10f32.powf(snr_db / 10.0) * (rate as f32 / 2.0) / 2500.0;
        let sigma = noise.sqrt();

        let mut rng = Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1);
        for s in out.iter_mut() {
            *s += rng.normal() * sigma;
        }
        out
    }

    /// splitmix64 plus Box–Muller, so the tests do not need a `rand` dependency.
    pub struct Rng(pub u64);

    impl Rng {
        fn next_u64(&mut self) -> u64 {
            self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = self.0;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        }

        fn uniform(&mut self) -> f32 {
            (self.next_u64() >> 11) as f32 / (1u64 << 53) as f32
        }

        pub fn normal(&mut self) -> f32 {
            let u = self.uniform().max(1e-9);
            let v = self.uniform();
            (-2.0 * u.ln()).sqrt() * (std::f32::consts::TAU * v).cos()
        }
    }

    /// Levenshtein similarity, `1 - distance / length`, for scoring a copy.
    pub fn accuracy(sent: &str, got: &str) -> f32 {
        let a: Vec<char> = sent.chars().collect();
        let b: Vec<char> = got.chars().collect();
        if a.is_empty() {
            return if b.is_empty() { 1.0 } else { 0.0 };
        }
        let mut prev: Vec<usize> = (0..=b.len()).collect();
        let mut cur = vec![0usize; b.len() + 1];
        for i in 1..=a.len() {
            cur[0] = i;
            for j in 1..=b.len() {
                let cost = usize::from(a[i - 1] != b[j - 1]);
                cur[j] = (prev[j] + 1).min(cur[j - 1] + 1).min(prev[j - 1] + cost);
            }
            std::mem::swap(&mut prev, &mut cur);
        }
        1.0 - prev[b.len()] as f32 / a.len() as f32
    }
}
