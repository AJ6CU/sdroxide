//! `AleController` — the ALE word receiver, receive only.
//!
//! ALE is not slotted, so this holds a rolling window of 8 kHz audio and
//! re-scans it as it advances, decoding the words it hears and de-duplicating
//! them. Words are appended to `ale.log` in the config directory as they are
//! decoded, which is the log to watch while testing on the air.

use std::io::Write;
use std::time::SystemTime;

use sdroxide_dsp::MonoResampler;
use sdroxide_dsp::ale::{self, ALE_RATE};
use sdroxide_types::{ALE_WORD_MAX, AleMessage, AleStatus, DigiConfig, DigiStatus, Mode, QsoStep};

use crate::{DigiAction, DigiEngine};

/// Audio window scanned, in seconds.
const WINDOW_S: f64 = 3.0;
/// How much new audio accumulates before the window is scanned again.
const SCAN_STEP_S: f64 = 1.0;
/// How long a decoded word suppresses a repeat, in seconds.
const DEDUP_TTL_S: f64 = 10.0;

pub struct AleController {
    cfg: DigiConfig,
    resampler: Option<MonoResampler>,
    buf: Vec<f32>,
    tap_scratch: Vec<f32>,
    level: f32,
    messages: Vec<AleMessage>,
    total: u64,
    seen: Vec<(String, SystemTime)>,
    since_scan: usize,
    status_dirty: bool,
    log: Option<std::fs::File>,
}

impl AleController {
    pub fn new(cfg: DigiConfig, tap_rate: f64) -> Self {
        AleController {
            cfg,
            resampler: MonoResampler::new(tap_rate, ALE_RATE),
            buf: Vec::new(),
            tap_scratch: Vec::new(),
            level: 0.0,
            messages: Vec::new(),
            total: 0,
            seen: Vec::new(),
            since_scan: 0,
            status_dirty: true,
            log: open_log(),
        }
    }

    fn window_samples(&self) -> usize {
        (WINDOW_S * ALE_RATE) as usize
    }

    fn step_samples(&self) -> usize {
        (SCAN_STEP_S * ALE_RATE) as usize
    }

    fn seen_recently(&mut self, key: &str, now: SystemTime) -> bool {
        self.seen.retain(|(_, t)| {
            now.duration_since(*t).map(|d| d.as_secs_f64() < DEDUP_TTL_S).unwrap_or(false)
        });
        self.seen.iter().any(|(k, _)| k == key)
    }

    fn digi_status(&self) -> DigiStatus {
        let mut s = DigiStatus::idle(self.cfg.clone());
        s.mode = Mode::Ale;
        s.step = QsoStep::Idle;
        s.ale = Some(AleStatus {
            level: self.level,
            messages: self.messages.clone(),
            total: self.total,
        });
        s
    }
}

impl DigiEngine for AleController {
    fn mode(&self) -> Mode {
        Mode::Ale
    }

    fn on_rx_audio(&mut self, tap: &[f32]) {
        self.tap_scratch.clear();
        match &mut self.resampler {
            Some(r) => r.push(tap, &mut self.tap_scratch),
            None => self.tap_scratch.extend_from_slice(tap),
        }
        self.buf.extend_from_slice(&self.tap_scratch);
        let cap = self.window_samples();
        if self.buf.len() > cap {
            let excess = self.buf.len() - cap;
            self.buf.drain(..excess);
        }
        self.since_scan += self.tap_scratch.len();
        if !tap.is_empty() {
            let e = tap.iter().map(|s| s * s).sum::<f32>() / tap.len() as f32;
            self.level = 0.9 * self.level + 0.1 * e.sqrt();
        }
    }

    fn poll(&mut self, now: SystemTime, _dial_hz: f64) -> Vec<DigiAction> {
        if self.since_scan >= self.step_samples() {
            self.since_scan = 0;
            let words = ale::decode_burst(&self.buf);
            for w in words {
                let key = format!("{} {}", w.kind.label(), w.address());
                if !self.seen_recently(&key, now) {
                    self.seen.push((key.clone(), now));
                    let at = now
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .map(|d| d.as_secs() as i64)
                        .unwrap_or(0);
                    self.messages.push(AleMessage {
                        at_unix: at,
                        kind: w.kind.label().into(),
                        address: w.address(),
                    });
                    self.total += 1;
                    self.status_dirty = true;
                    tracing::info!("ALE {key}");
                    if let Some(f) = &mut self.log {
                        let _ = writeln!(f, "{at} {key}");
                        let _ = f.flush();
                    }
                }
            }
            if self.messages.len() > ALE_WORD_MAX {
                let excess = self.messages.len() - ALE_WORD_MAX;
                self.messages.drain(..excess);
            }
        }
        if self.status_dirty {
            self.status_dirty = false;
            return vec![DigiAction::Status(self.digi_status())];
        }
        Vec::new()
    }

    fn tx_burst_active(&self) -> bool {
        false
    }

    fn fill_tx_block(&mut self, _out: &mut [f32]) -> bool {
        false
    }

    fn on_burst_done(&mut self) {}

    fn abort(&mut self) {
        self.buf.clear();
        self.since_scan = 0;
        self.status_dirty = true;
    }

    fn abort_tx(&mut self) {}

    fn set_config(&mut self, cfg: DigiConfig) {
        self.cfg = cfg;
        self.status_dirty = true;
    }

    fn set_audio_hz(&mut self, _hz: f32) {}

    fn audio_hz(&self) -> f32 {
        1500.0
    }

    fn status(&self) -> DigiStatus {
        self.digi_status()
    }
}

/// The debug log, appended beside the rest of the config. `None` if it cannot
/// be opened — the decoder still runs and the panel still shows words.
fn open_log() -> Option<std::fs::File> {
    let base =
        std::env::var("XDG_CONFIG_HOME").map(std::path::PathBuf::from).ok().or_else(|| {
            std::env::var("HOME").ok().map(|h| std::path::PathBuf::from(h).join(".config"))
        })?;
    let dir = base.join("sdroxide-brown");
    let _ = std::fs::create_dir_all(&dir);
    std::fs::OpenOptions::new().create(true).append(true).open(dir.join("ale.log")).ok()
}
