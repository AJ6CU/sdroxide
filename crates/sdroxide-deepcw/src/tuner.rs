//! Bringing a CW signal into the band the model reads.
//!
//! The model only looks at 400–1200 Hz of a 3200 Hz stream, and it was trained
//! on signals sitting inside that. Nothing else about a receiver obliges the
//! tone to be there: the CW pitch is an operator preference and this one ranges
//! over 200–3000 Hz, the top of which would not even survive the resample. So
//! the signal is moved rather than hoped for — mixed down to DC, filtered,
//! resampled, and put back up at 800 Hz.
//!
//! Mixing a *real* signal down to DC leaves an image at twice the tone
//! frequency, mirrored to the negative side. Taking the real part at the end
//! folds negative frequencies back onto positive ones, so that image would come
//! out as a second tone next to the wanted one — at 600 Hz for a 700 Hz pitch,
//! comfortably inside the band and audible to the model as a second station.
//! The complex low-pass is what removes it, and why its width is chosen against
//! the lowest pitch the operator can dial rather than against CW's bandwidth.

use sdroxide_dsp::{Complex32, ComplexFir, ComplexResampler, Nco, bandpass_taps};

use crate::spectrogram::{CENTER_FREQ_HZ, SAMPLE_RATE};

/// Half-width of the passband kept around the tone.
///
/// Wide enough for CW at any speed — 40 WPM keys at 33 dits a second and needs
/// barely a tenth of this — and narrow enough that the mixing image is gone for
/// any pitch above about 175 Hz, which is below anything the operator can set.
const HALF_BW_HZ: f64 = 350.0;

/// Long enough to make that passband edge steep at 3200 Hz.
const TAPS: usize = 129;

/// Real audio at any rate, tone anywhere → mono 3200 Hz with the tone at 800 Hz.
pub struct Tuner {
    in_rate: f64,
    tone_hz: f64,
    down: Nco,
    resampler: Option<ComplexResampler>,
    filter: ComplexFir,
    up: Nco,
    mixed: Vec<Complex32>,
    resampled: Vec<Complex32>,
    filtered: Vec<Complex32>,
}

impl Tuner {
    /// `in_rate` is the rate of the audio being pushed; `tone_hz` is where the
    /// CW tone sits in it.
    pub fn new(in_rate: f64, tone_hz: f64) -> Self {
        Tuner {
            in_rate,
            tone_hz,
            down: Nco::new(-tone_hz, in_rate),
            // Rubato's anti-aliasing handles everything above 1600 Hz; the
            // narrow filter below runs at the low rate where it is cheap.
            resampler: ComplexResampler::new(in_rate, SAMPLE_RATE),
            filter: ComplexFir::new(bandpass_taps(TAPS, -HALF_BW_HZ, HALF_BW_HZ, SAMPLE_RATE)),
            up: Nco::new(CENTER_FREQ_HZ, SAMPLE_RATE),
            mixed: Vec::new(),
            resampled: Vec::new(),
            filtered: Vec::new(),
        }
    }

    /// Follow the tone. Cheap, and phase-continuous — the AFC in the classic
    /// front end moves this by a few Hz at a time as it tracks drift.
    pub fn set_tone(&mut self, tone_hz: f64) {
        if (self.tone_hz - tone_hz).abs() < 0.5 {
            return;
        }
        self.tone_hz = tone_hz;
        self.down.set_freq(-tone_hz, self.in_rate);
    }

    pub fn tone_hz(&self) -> f64 {
        self.tone_hz
    }

    /// Feed real audio; appends model-band audio to `out`.
    pub fn push(&mut self, audio: &[f32], out: &mut Vec<f32>) {
        if audio.is_empty() {
            return;
        }
        self.mixed.clear();
        self.mixed.extend(audio.iter().map(|&s| Complex32::new(s, 0.0)));
        self.down.mix_in_place(&mut self.mixed);
        self.finish(out);
    }

    /// Feed complex baseband already centred on the tone — what a DDC hands
    /// back — at the rate this tuner was built for.
    pub fn push_baseband(&mut self, iq: &[Complex32], out: &mut Vec<f32>) {
        if iq.is_empty() {
            return;
        }
        self.mixed.clear();
        self.mixed.extend_from_slice(iq);
        self.finish(out);
    }

    fn finish(&mut self, out: &mut Vec<f32>) {
        self.resampled.clear();
        match self.resampler.as_mut() {
            Some(r) => r.push(&self.mixed, &mut self.resampled),
            None => self.resampled.extend_from_slice(&self.mixed),
        }
        if self.resampled.is_empty() {
            return;
        }

        self.filtered.clear();
        self.filter.process(&self.resampled, &mut self.filtered);
        self.up.mix_in_place(&mut self.filtered);
        // Real part only: the model's front end is a real FFT, and the mirror a
        // real signal carries is what its 400–1200 Hz window is defined against.
        out.extend(self.filtered.iter().map(|z| z.re));
    }

    pub fn reset(&mut self) {
        self.filter.reset();
        self.mixed.clear();
        self.resampled.clear();
        self.filtered.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spectrogram::{BINS, Spectrogram};

    /// Peak bin index of the middle frame, in the model's cropped band.
    fn peak_bin(audio: &[f32]) -> usize {
        let mut sg = Spectrogram::new();
        let spec = sg.compute(audio);
        let frames = spec.len() / BINS;
        assert!(frames > 4, "not enough audio to look at");
        let row = &spec[(frames / 2) * BINS..][..BINS];
        row.iter().enumerate().max_by(|a, b| a.1.total_cmp(b.1)).unwrap().0
    }

    fn tone(rate: f64, hz: f64, seconds: f64) -> Vec<f32> {
        let n = (rate * seconds) as usize;
        (0..n).map(|i| (std::f64::consts::TAU * hz * i as f64 / rate).sin() as f32).collect()
    }

    #[test]
    fn any_pitch_comes_out_at_the_centre_of_the_band() {
        // 800 Hz is bin 64, index 32 of the cropped band.
        for pitch in [200.0, 400.0, 700.0, 1200.0, 2000.0, 3000.0] {
            let mut tuner = Tuner::new(48_000.0, pitch);
            let mut out = Vec::new();
            tuner.push(&tone(48_000.0, pitch, 3.0), &mut out);
            let bin = peak_bin(&out);
            assert_eq!(bin, 32, "a {pitch} Hz tone landed at bin {bin}, not the centre");
        }
    }

    #[test]
    fn the_mixing_image_does_not_survive() {
        // Mixing a real 700 Hz tone to DC leaves an image at -1400 Hz, which
        // would fold to 600 Hz — bin 16 — when the real part is taken.
        let mut tuner = Tuner::new(48_000.0, 700.0);
        let mut out = Vec::new();
        tuner.push(&tone(48_000.0, 700.0, 3.0), &mut out);

        let mut sg = Spectrogram::new();
        let spec = sg.compute(&out);
        let frames = spec.len() / BINS;
        let row = &spec[(frames / 2) * BINS..][..BINS];
        let wanted = row[32];
        let image = row[16];
        assert!(image < wanted * 0.05, "image at bin 16 is {image:.3} against {wanted:.3} wanted");
    }

    #[test]
    fn a_signal_outside_the_passband_is_rejected() {
        // A second station 900 Hz away must not be dragged into the band along
        // with the one being copied.
        let mut tuner = Tuner::new(48_000.0, 700.0);
        let mut out = Vec::new();
        let mut audio = tone(48_000.0, 1600.0, 3.0);
        for (i, s) in audio.iter_mut().enumerate() {
            *s += (std::f64::consts::TAU * 700.0 * i as f64 / 48_000.0).sin() as f32 * 0.02;
        }
        tuner.push(&audio, &mut out);
        // The wanted signal is 34 dB weaker and still has to be the peak.
        assert_eq!(peak_bin(&out), 32);
    }

    #[test]
    fn baseband_input_lands_in_the_same_place() {
        // What the skimmer feeds: complex baseband already centred on the tone.
        let mut tuner = Tuner::new(SAMPLE_RATE, 0.0);
        let n = (SAMPLE_RATE * 3.0) as usize;
        let iq: Vec<Complex32> = (0..n).map(|_| Complex32::new(1.0, 0.0)).collect();
        let mut out = Vec::new();
        tuner.push_baseband(&iq, &mut out);
        assert_eq!(peak_bin(&out), 32);
    }

    #[test]
    fn retuning_is_ignored_below_half_a_hertz() {
        let mut tuner = Tuner::new(48_000.0, 700.0);
        tuner.set_tone(700.2);
        assert_eq!(tuner.tone_hz(), 700.0);
        tuner.set_tone(706.0);
        assert_eq!(tuner.tone_hz(), 706.0);
    }
}
