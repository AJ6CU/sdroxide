//! `Cis11Controller` — the CIS-11 (TORG-11) teleprinter receiver, receive only.
//!
//! CIS-11 is a continuous broadcast, not a slot: the receiver runs as audio
//! arrives, finds its own two tones and tracks the character clock. Decoded
//! characters go into `text_rx`, which is what the shared keyboard-mode panel
//! already draws — so the mode needs no panel of its own.
//!
//! The panel also carries the receiver's own readout, because there is nowhere
//! else for it and the numbers are the honest part of this mode: the tones it
//! locked to, the shift, and how many characters passed their parity check. A
//! listener reading an encrypted transmission sees the parity count instead of
//! nothing, which is the difference between "this is CIS-11" and "there is
//! nothing here".

use std::time::SystemTime;

use sdroxide_dsp::MonoResampler;
use sdroxide_dsp::cis11::Cis11Rx;
use sdroxide_types::{DigiConfig, DigiStatus, Mode, QsoStep};

use crate::{DigiAction, DigiEngine};

/// The receiver's own rate. 100 baud needs very little, and 8 kHz is what the
/// other narrowband decoders here use.
const RATE: f64 = 8000.0;
/// How much decoded text is kept for the panel.
const TEXT_MAX: usize = 4000;

pub struct Cis11Controller {
    cfg: DigiConfig,
    resampler: Option<MonoResampler>,
    scratch: Vec<f32>,
    rx: Cis11Rx,
    text: String,
    level: f32,
    status_dirty: bool,
    /// The tone pair last announced, so the readout is written once rather than
    /// on every poll.
    announced: Option<(i32, i32)>,
}

impl Cis11Controller {
    pub fn new(cfg: DigiConfig, tap_rate: f64) -> Self {
        Cis11Controller {
            cfg,
            resampler: MonoResampler::new(tap_rate, RATE),
            scratch: Vec::new(),
            rx: Cis11Rx::new(RATE),
            text: String::new(),
            level: 0.0,
            status_dirty: true,
            announced: None,
        }
    }

    fn push_line(&mut self, line: &str) {
        self.text.push_str(line);
        self.text.push('\n');
        if self.text.len() > TEXT_MAX {
            let cut = self.text.len() - TEXT_MAX;
            self.text.drain(..cut);
        }
    }

    fn digi_status(&self) -> DigiStatus {
        let mut s = DigiStatus::idle(self.cfg.clone());
        s.mode = Mode::Cis11;
        s.step = QsoStep::Idle;
        s.text_rx = self.text.clone();
        s
    }
}

impl DigiEngine for Cis11Controller {
    fn mode(&self) -> Mode {
        Mode::Cis11
    }

    fn on_rx_audio(&mut self, tap: &[f32]) {
        if tap.is_empty() {
            return;
        }
        self.scratch.clear();
        match &mut self.resampler {
            Some(r) => r.push(tap, &mut self.scratch),
            None => self.scratch.extend_from_slice(tap),
        }
        let e = tap.iter().map(|s| s * s).sum::<f32>() / tap.len() as f32;
        self.level = 0.9 * self.level + 0.1 * e.sqrt();

        let chars = self.rx.process(&self.scratch);
        let text = self.rx.take_text();
        if !text.is_empty() {
            // Characters arrive through the receiver's own ITA-2 reader; the raw
            // bits are logged rather than drawn, because how the ten data bits
            // divide is not settled (see `docs/internal/MODE-ROADMAP.md`).
            self.text.push_str(&text);
            if self.text.len() > TEXT_MAX {
                let cut = self.text.len() - TEXT_MAX;
                self.text.drain(..cut);
            }
            self.status_dirty = true;
        }
        for c in &chars {
            tracing::debug!(
                "CIS-11 {:011b} parity {} clean {} confidence {:.2}",
                c.bits.iter().enumerate().fold(0u16, |v, (i, b)| v | ((*b as u16) << i)),
                c.parity,
                c.clean(),
                c.confidence
            );
        }

        // The readout, once per lock: the tones, the shift and how much of what
        // arrived passed parity. Without it a listener has no way to tell an
        // encrypted CIS-11 from a quieter band.
        let r = self.rx.report();
        if r.locked {
            let key = (r.tone_low_hz.round() as i32, r.tone_high_hz.round() as i32);
            if self.announced != Some(key) {
                self.announced = Some(key);
                let line = format!(
                    "· CIS-11: tones {:.0}/{:.0} Hz, shift {:.0} Hz, {} of {} characters \
                     parity-valid",
                    r.tone_low_hz, r.tone_high_hz, r.shift_hz, r.clean, r.characters
                );
                self.push_line(&line);
                self.status_dirty = true;
                tracing::info!("{line}");
            }
        }
    }

    fn poll(&mut self, _now: SystemTime, _dial_hz: f64) -> Vec<DigiAction> {
        if !self.status_dirty {
            return Vec::new();
        }
        self.status_dirty = false;
        vec![DigiAction::Status(self.digi_status())]
    }

    fn abort(&mut self) {
        self.status_dirty = true;
    }

    fn set_config(&mut self, cfg: DigiConfig) {
        self.cfg = cfg;
        self.status_dirty = true;
    }

    // Receive only, like every other broadcast mode here: the trait's transmit
    // half answers "nothing to send" rather than being unimplemented.
    fn tx_burst_active(&self) -> bool {
        false
    }

    fn fill_tx_block(&mut self, _out: &mut [f32]) -> bool {
        false
    }

    fn on_burst_done(&mut self) {}

    fn abort_tx(&mut self) {}

    fn set_audio_hz(&mut self, _hz: f32) {}

    fn audio_hz(&self) -> f32 {
        1500.0
    }

    fn clear_rx(&mut self) {
        self.text.clear();
        self.status_dirty = true;
    }

    fn status(&self) -> DigiStatus {
        self.digi_status()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The controller's readout and its text both reach the panel: a synthesized
    /// CIS-11 burst goes in at 48 kHz and `text_rx` comes out with characters and
    /// the tone line.
    #[test]
    fn a_burst_reaches_the_panel_as_text_and_a_readout() {
        let mut c = Cis11Controller::new(DigiConfig::default(), 48_000.0);
        // Five characters, 11 bits each, 100 baud, then repeated so the receiver
        // has a second to acquire on.
        let sps = 480; // 48 kHz / 100 baud
        let mut audio = Vec::new();
        let mut phase = 0.0f64;
        for _ in 0..40 {
            // Ten data bits and the odd-parity bit, built the way the mode sends
            // them — a pattern with the parity wrong never becomes "clean", so the
            // receiver never calls itself locked and the readout never appears.
            let mut bits = [1u8, 0, 1, 0, 1, 0, 1, 0, 1, 0, 0];
            bits[10] = if bits[..10].iter().sum::<u8>() % 2 == 0 { 1 } else { 0 };
            for b in bits {
                let f = if b == 1 { 1250.0 } else { 750.0 };
                for _ in 0..sps {
                    phase += std::f64::consts::TAU * f / 48_000.0;
                    audio.push((0.4 * phase.sin()) as f32);
                }
            }
        }
        for chunk in audio.chunks(1024) {
            c.on_rx_audio(chunk);
        }
        let s = c.poll(SystemTime::now(), 0.0);
        let DigiAction::Status(st) = &s[0] else { panic!("no status") };
        assert_eq!(st.mode, Mode::Cis11);
        assert!(st.text_rx.contains("CIS-11:"), "no readout: {:?}", st.text_rx);
        assert!(st.text_rx.contains("parity-valid"), "{:?}", st.text_rx);
    }
}
