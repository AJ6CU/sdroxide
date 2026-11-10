//! **STANAG 4285, decoded by the vendored reference implementation.**
//!
//! This tree has its own receiver for the mode — sync, reference blocks,
//! constellation, the uncoded text — and it has never read the coded payload off
//! the air. The reference decoder in [`vendor/s4285-ref`] does: on this station's
//! 24 October capture of 13031.2 kHz it reads a French Navy test tape
//! (`RYRYRYRY…`, `ZNR UUUUU`, `ZUI TESTING`) where ours produces noise.
//!
//! **Licensing.** The vendored code is GPL-3.0-or-later, as this tree is; it is
//! credited in the README's Acknowledgements. This crate is native-only: a C++
//! build script must never become a dependency of anything that targets wasm.

use std::os::raw::{c_double, c_float, c_int};

unsafe extern "C" {
    fn sdroxide_s4285_decode(
        audio: *const c_float,
        n: usize,
        rate: c_double,
        mod_: c_int,
        carrier: c_double,
        out: *mut u8,
        outlen: usize,
    ) -> c_int;
}

/// What one decode produced.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Decoded {
    /// The ITA2 reading of the message bits — a *reading*, since Annex E names no
    /// character set; a test tape is what this expects.
    pub text: String,
    /// Message bits the chain produced. Zero means it did not lock at all.
    pub bits: usize,
}

impl Decoded {
    pub fn is_empty(&self) -> bool {
        self.text.trim().is_empty()
    }
}

/// Decode a stretch of audio at `rate` hertz.
///
/// `mod_` is bits per symbol — 2 for the BPSK rates (600/300/150/75), 4 for the
/// QPSK rate, 8 for 8PSK — and `carrier` is where the signal sits in the audio,
/// 1800 Hz by the mode's own convention.
pub fn decode(audio: &[f32], rate: f64, mod_: u8, carrier: f64) -> Decoded {
    if audio.is_empty() || rate <= 0.0 {
        return Decoded::default();
    }
    // **The decoder works at 12 kHz**, five samples a symbol at 2400 baud. A
    // capture arrives at the sound card's rate, so it is resampled here rather
    // than insisting the caller do it.
    let resampled: Vec<f32>;
    let (audio, rate) = if (rate - 12_000.0).abs() < 0.5 {
        (audio, rate)
    } else {
        match sdroxide_dsp::MonoResampler::new(rate, 12_000.0) {
            Some(mut r) => {
                let mut out = Vec::with_capacity(audio.len() / 4 + 1024);
                r.push(audio, &mut out);
                resampled = out;
                (&resampled[..], 12_000.0)
            }
            None => (audio, rate),
        }
    };
    // Generous: a long over at 75 bps is a great deal of text.
    let mut buf = vec![0u8; 512 * 1024];
    let bits = unsafe {
        sdroxide_s4285_decode(
            audio.as_ptr(),
            audio.len(),
            rate as c_double,
            mod_ as c_int,
            carrier,
            buf.as_mut_ptr(),
            buf.len(),
        )
    };
    if bits <= 0 {
        return Decoded::default();
    }
    let n = (bits as usize).min(buf.len());
    let text = sdroxide_dsp::stanag4285_fec::message_text(&buf[..n])
        .unwrap_or_default()
        .trim_end()
        .to_string();
    Decoded { text, bits: n }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The real measurement: this station's own capture, which the vendored
    /// decoder reads and our own chain does not.
    ///
    ///     SDROXIDE_S4285_SAMPLE=~/Downloads/....wav \
    ///       cargo test -p sdroxide-s4285 -- --ignored --nocapture
    #[test]
    #[ignore]
    fn the_capture_decodes() {
        let path = std::env::var("SDROXIDE_S4285_SAMPLE").expect("set SDROXIDE_S4285_SAMPLE");
        let mut reader = hound::WavReader::open(&path).expect("open the wav");
        let spec = reader.spec();
        let raw: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
            hound::SampleFormat::Int => {
                let scale = 1.0 / (1i64 << (spec.bits_per_sample - 1)) as f32;
                reader.samples::<i32>().map(|s| s.unwrap() as f32 * scale).collect()
            }
        };
        let ch = spec.channels.max(1) as usize;
        let audio: Vec<f32> = if ch == 1 {
            raw
        } else {
            raw.chunks(ch).map(|f| f.iter().sum::<f32>() / ch as f32).collect()
        };
        let d = decode(&audio, spec.sample_rate as f64, 2, 1800.0);
        eprintln!("{} bits, text {:?}", d.bits, &d.text[..d.text.len().min(200)]);
        assert!(!d.is_empty(), "the vendored decoder read nothing");
    }

    /// Nothing in, nothing out — and no panic.
    #[test]
    fn silence_decodes_to_nothing() {
        let d = decode(&vec![0.0f32; 48_000], 48_000.0, 2, 1800.0);
        assert!(d.is_empty(), "silence decoded as {:?}", d.text);
    }
}
