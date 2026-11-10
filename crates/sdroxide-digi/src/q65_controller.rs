//! `Q65Controller` — Q65, the modern weak-signal mode, receive only.
//!
//! Slotted like FST4 and shaped the same way, with the same twist: Q65's
//! sub-mode ([`Q65Mode`]) fixes both the T/R period and the tone-spacing
//! letter, so it is an operator setting rather than part of the mode. All ten
//! sub-modes share one waveform shape and one 77-bit message, so the decode is
//! the same call with a different protocol type — and the slot the buffer
//! holds changes length when the operator picks a new sub-mode.
//!
//! # Why a worker thread
//!
//! A Q65-300 scan is tens of seconds of work over a five-minute slot, and even
//! the short sub-modes are far from free. The decode runs on its own thread and
//! the result is drained from [`Self::poll`], as WSPR's and FST4's do — one
//! slot in flight at a time, dropping rather than queueing when the machine
//! cannot keep up.
//!
//! **Transmit is wired**: the message box takes WSJT-X's three words (`CQ K1ABC
//! FN42`, `K1ABC W1XYZ -15`), and one burst goes out in the first slot the
//! operator arms, starting at the sub-mode's own delay into the slot.

use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::time::SystemTime;

use sdroxide_dsp::MonoResampler;
use sdroxide_types::{Decode, DigiConfig, DigiStatus, Mode, Q65Mode, QsoStep};

use crate::DigiEngine;
use crate::controller::DigiAction;
use crate::modem::decode_q65_slot;
use crate::params::DECODE_RATE;
use crate::scheduler::SlotScheduler;

/// One slot of audio handed to the decode worker.
struct DecodeJob {
    audio: Vec<i16>,
    mode: Q65Mode,
    slot_utc: i64,
}

/// What the worker sends back.
struct DecodeResult {
    decodes: Vec<Decode>,
}

pub struct Q65Controller {
    cfg: DigiConfig,
    /// The sub-mode in force, mirroring `cfg.q65_mode`; re-read whenever the
    /// config changes, because it decides both the slot length and the decode.
    mode: Q65Mode,
    scheduler: SlotScheduler,
    resampler: Option<MonoResampler>,
    /// 12 kHz audio accumulated for the slot in progress.
    slot_buf: Vec<i16>,
    /// Whether `slot_buf` began at a slot boundary. It does not after a start,
    /// a reset or a period change part-way through a slot, and a buffer that
    /// began mid-slot is not the slot the decoder is told it is — so such a
    /// slot is dropped rather than decoded.
    buf_aligned: bool,
    tap_scratch: Vec<f32>,
    last_slot_idx: i64,
    audio_hz: f32,

    job_tx: Sender<DecodeJob>,
    res_rx: Receiver<DecodeResult>,
    _worker: std::thread::JoinHandle<()>,
    pending: bool,
    last_count: u32,
    status_dirty: bool,

    /// The operator's message, in WSJT-X's three words.
    tx_text: String,
    tx_active: bool,
    burst: Option<crate::controller::BurstPlayer>,
    tx_fired_slot: i64,
    tx_problem: Option<String>,
}

impl Q65Controller {
    pub fn new(cfg: DigiConfig, tap_rate: f64) -> Self {
        let mode = cfg.q65_mode;
        let (job_tx, job_rx) = std::sync::mpsc::channel::<DecodeJob>();
        let (res_tx, res_rx) = std::sync::mpsc::channel::<DecodeResult>();
        let worker = std::thread::Builder::new()
            .name("sdroxide-q65-decode".into())
            .spawn(move || {
                while let Ok(job) = job_rx.recv() {
                    let decodes = decode_q65_slot(&job.audio, job.mode, job.slot_utc);
                    if res_tx.send(DecodeResult { decodes }).is_err() {
                        break;
                    }
                }
            })
            .expect("spawn q65 decode worker");

        Q65Controller {
            cfg,
            mode,
            scheduler: SlotScheduler::new(mode.slot_s(), mode.start_delay_s()),
            resampler: MonoResampler::new(tap_rate, DECODE_RATE),
            slot_buf: Vec::new(),
            buf_aligned: false,
            tap_scratch: Vec::new(),
            last_slot_idx: i64::MIN,
            audio_hz: 1000.0,
            job_tx,
            res_rx,
            _worker: worker,
            pending: false,
            last_count: 0,
            status_dirty: true,
            tx_text: String::new(),
            tx_active: false,
            burst: None,
            tx_fired_slot: i64::MIN,
            tx_problem: None,
        }
    }

    fn slot_samples(&self) -> usize {
        (self.mode.slot_s() * DECODE_RATE) as usize
    }

    fn digi_status(&self) -> DigiStatus {
        let mut s = DigiStatus::idle(self.cfg.clone());
        s.mode = Mode::Q65;
        s.step = QsoStep::Idle;
        s.audio_hz = self.audio_hz;
        s.tx_next = self.tx_active;
        s.transmitting = self.burst.is_some();
        s.tx_pending_msg = (!self.tx_text.is_empty()).then(|| self.tx_text.clone());
        // Named here as well as at the key-up: the reason outlives the refusal,
        // which is the moment the operator needs it.
        s.tx_refused = self.tx_problem.clone();
        s
    }

    /// The three words a Q65 message is made of, refuse-named if it is not.
    fn q65_words(&self) -> Option<(&str, &str, &str)> {
        let mut it = self.tx_text.split_whitespace();
        let (a, b, c) = (it.next()?, it.next()?, it.next()?);
        if it.next().is_some() {
            return None;
        }
        Some((a, b, c))
    }

    /// Synthesise the operator's message at the modem's own 12 kHz — the rate
    /// the decoder takes.
    fn synthesize_burst_12k(&self) -> Result<Vec<f32>, String> {
        let Some((a, b, c)) = self.q65_words() else {
            return Err(format!(
                "{} wants WSJT-X's three words — \"CQ K1ABC FN42\" or \"K1ABC W1XYZ -15\"",
                self.mode.label()
            ));
        };
        let (rate, hz, amp) = (DECODE_RATE as u32, self.audio_hz, 0.5f32);
        let audio = match self.mode {
            Q65Mode::A15 => mfsk_core::q65::tx::synthesize_standard_for::<mfsk_core::q65::Q65a15>(
                a, b, c, rate, hz, amp,
            ),
            Q65Mode::A30 => mfsk_core::q65::tx::synthesize_standard_for::<mfsk_core::q65::Q65a30>(
                a, b, c, rate, hz, amp,
            ),
            Q65Mode::A60 => mfsk_core::q65::tx::synthesize_standard_for::<mfsk_core::q65::Q65a60>(
                a, b, c, rate, hz, amp,
            ),
            Q65Mode::B60 => mfsk_core::q65::tx::synthesize_standard_for::<mfsk_core::q65::Q65b60>(
                a, b, c, rate, hz, amp,
            ),
            Q65Mode::C60 => mfsk_core::q65::tx::synthesize_standard_for::<mfsk_core::q65::Q65c60>(
                a, b, c, rate, hz, amp,
            ),
            Q65Mode::D60 => mfsk_core::q65::tx::synthesize_standard_for::<mfsk_core::q65::Q65d60>(
                a, b, c, rate, hz, amp,
            ),
            Q65Mode::E60 => mfsk_core::q65::tx::synthesize_standard_for::<mfsk_core::q65::Q65e60>(
                a, b, c, rate, hz, amp,
            ),
            Q65Mode::D120 => {
                mfsk_core::q65::tx::synthesize_standard_for::<mfsk_core::q65::Q65d120>(
                    a, b, c, rate, hz, amp,
                )
            }
            Q65Mode::E120 => {
                mfsk_core::q65::tx::synthesize_standard_for::<mfsk_core::q65::Q65e120>(
                    a, b, c, rate, hz, amp,
                )
            }
            Q65Mode::A300 => {
                mfsk_core::q65::tx::synthesize_standard_for::<mfsk_core::q65::Q65a300>(
                    a, b, c, rate, hz, amp,
                )
            }
        };
        audio.ok_or_else(|| {
            format!(
                "\"{a} {b} {c}\" is not a message {} can carry — the third word is a \
                 four-character grid or a report, and the two calls have to be plain",
                self.mode.label()
            )
        })
    }

    fn synthesize_burst(&self) -> Result<Vec<f32>, String> {
        let audio = self.synthesize_burst_12k()?;
        match MonoResampler::new(DECODE_RATE, 48_000.0) {
            Some(mut r) => {
                let mut out = Vec::with_capacity(audio.len() * 4 + 2048);
                r.push(&audio, &mut out);
                Ok(out)
            }
            None => Ok(audio),
        }
    }
}

impl DigiEngine for Q65Controller {
    fn mode(&self) -> Mode {
        Mode::Q65
    }

    fn on_rx_audio(&mut self, tap: &[f32]) {
        self.tap_scratch.clear();
        match &mut self.resampler {
            Some(r) => r.push(tap, &mut self.tap_scratch),
            None => self.tap_scratch.extend_from_slice(tap),
        }
        let cap = self.slot_samples() + self.slot_samples() / 8;
        for &s in &self.tap_scratch {
            if self.slot_buf.len() < cap {
                self.slot_buf.push((s.clamp(-1.0, 1.0) * 28_000.0) as i16);
            }
        }
    }

    fn poll(&mut self, now: SystemTime, _dial_hz: f64) -> Vec<DigiAction> {
        let mut actions = Vec::new();

        // Drain the worker.
        loop {
            match self.res_rx.try_recv() {
                Ok(res) => {
                    self.pending = false;
                    self.last_count = res.decodes.len() as u32;
                    self.status_dirty = true;
                    if !res.decodes.is_empty() {
                        actions.push(DigiAction::Decodes(res.decodes));
                    }
                }
                Err(TryRecvError::Empty) => break,
                // The worker is gone and no answer is coming. Release the slot
                // rather than holding `pending` for the rest of the session,
                // which would stop every later slot being dispatched.
                Err(TryRecvError::Disconnected) => {
                    self.pending = false;
                    break;
                }
            }
        }

        let idx = self.scheduler.slot_index(now);
        if idx != self.last_slot_idx {
            // Only a slot whose audio began on its own boundary is decoded, and
            // half a slot of it is the floor: less than that is a stream
            // hiccup, not a transmission.
            let min_samples = (self.mode.slot_s() * DECODE_RATE * 0.5) as usize;
            if self.buf_aligned && self.slot_buf.len() >= min_samples && !self.pending {
                let audio = std::mem::take(&mut self.slot_buf);
                // The slot that just ended is the one before this boundary.
                let slot_utc = self.scheduler.slot_start_unix(idx - 1) as i64;
                self.pending =
                    self.job_tx.send(DecodeJob { audio, mode: self.mode, slot_utc }).is_ok();
            }
            self.slot_buf.clear();
            // The very first poll is not a boundary crossing, only the first
            // look at the clock — part-way through a slot.
            self.buf_aligned = self.last_slot_idx != i64::MIN;
            self.last_slot_idx = idx;
        }
        // Transmit. One burst per slot, started inside the window where the
        // message still lands where the receiver looks for it.
        if self.burst.is_none() && idx != self.tx_fired_slot && self.tx_active {
            let into = self.scheduler.secs_into_slot(now);
            let start = self.mode.start_delay_s();
            if into >= start && into <= start + 2.0 {
                self.tx_fired_slot = idx;
                match self.synthesize_burst() {
                    Ok(samples) => {
                        self.tx_problem = None;
                        self.burst = Some(crate::controller::BurstPlayer { samples, pos: 0 });
                        actions.push(DigiAction::KeyTx);
                    }
                    Err(why) => {
                        self.tx_problem = Some(why);
                        self.tx_active = false;
                    }
                }
                self.status_dirty = true;
            }
        }

        if self.status_dirty {
            self.status_dirty = false;
            actions.push(DigiAction::Status(self.digi_status()));
        }
        actions
    }

    fn tx_burst_active(&self) -> bool {
        self.burst.is_some()
    }

    fn fill_tx_block(&mut self, out: &mut [f32]) -> bool {
        let Some(b) = self.burst.as_mut() else {
            out.fill(0.0);
            return true;
        };
        let mut done = false;
        for slot in out.iter_mut() {
            if b.pos < b.samples.len() {
                *slot = b.samples[b.pos];
                b.pos += 1;
            } else {
                *slot = 0.0;
                done = true;
            }
        }
        if done {
            self.burst = None;
            self.status_dirty = true;
        }
        done
    }

    fn on_burst_done(&mut self) {
        self.status_dirty = true;
    }

    fn set_tx_text(&mut self, text: String) {
        self.tx_text = text;
        self.tx_problem = None;
        self.status_dirty = true;
    }

    fn set_tx_active(&mut self, on: bool) {
        if on {
            match self.synthesize_burst() {
                Ok(_) => {
                    self.tx_problem = None;
                    self.tx_active = true;
                }
                Err(why) => {
                    self.tx_problem = Some(why);
                    self.tx_active = false;
                }
            }
        } else {
            self.tx_active = false;
        }
        self.status_dirty = true;
    }

    fn abort(&mut self) {
        self.slot_buf.clear();
        self.buf_aligned = false;
        self.status_dirty = true;
    }

    fn abort_tx(&mut self) {
        self.burst = None;
        self.tx_active = false;
        self.status_dirty = true;
    }

    fn set_config(&mut self, cfg: DigiConfig) {
        // A sub-mode change moves the slot length and the decode, so the
        // scheduler and the buffer bound follow it. Drop the audio in hand: it
        // belongs to the old geometry.
        if cfg.q65_mode != self.mode {
            self.mode = cfg.q65_mode;
            self.scheduler = SlotScheduler::new(self.mode.slot_s(), self.mode.start_delay_s());
            self.slot_buf.clear();
            self.buf_aligned = false;
            self.last_slot_idx = i64::MIN;
        }
        self.cfg = cfg;
        self.status_dirty = true;
    }

    fn clear_rx(&mut self) {
        self.last_count = 0;
        self.status_dirty = true;
    }

    fn set_audio_hz(&mut self, hz: f32) {
        self.audio_hz = hz.clamp(200.0, 3500.0);
    }

    fn audio_hz(&self) -> f32 {
        self.audio_hz
    }

    fn status(&self) -> DigiStatus {
        self.digi_status()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **Q65 transmits, and the decoder this build already had reads it back.**
    ///
    /// Two sub-modes at the ends of the family: the shortest period (`Q65-15A`,
    /// 15 s) and the deepest tone spacing (`Q65-60E`), so a fault in the
    /// period or in the spacing shows rather than hiding behind one working
    /// case. The message is placed where the transmitter puts it and handed to
    /// the same `decode_q65_slot` the receive path uses — our own two halves,
    /// which is not a station.
    #[test]
    fn a_q65_transmission_reads_back() {
        for sub in [Q65Mode::A15, Q65Mode::E60] {
            let mut cfg = DigiConfig::default();
            cfg.q65_mode = sub;
            let mut ctl = Q65Controller::new(cfg, DECODE_RATE);
            ctl.set_audio_hz(1000.0);
            ctl.set_tx_text("CQ K1ABC FN42".into());
            let burst = ctl.synthesize_burst_12k().expect("the message packs");
            let mut slot = vec![0.0f32; ctl.slot_samples()];
            let at = (sub.start_delay_s() * DECODE_RATE) as usize;
            for (i, s) in burst.iter().enumerate() {
                if let Some(d) = slot.get_mut(at + i) {
                    *d += *s;
                }
            }
            let audio: Vec<i16> =
                slot.iter().map(|v| (v.clamp(-1.0, 1.0) * 28_000.0) as i16).collect();
            let decodes = decode_q65_slot(&audio, sub, 0);
            assert!(
                decodes.iter().any(|d| d.message.contains("K1ABC")),
                "{sub:?} did not read its own transmission back: {:?}",
                decodes.iter().map(|d| &d.message).collect::<Vec<_>>()
            );
        }
    }

    /// A message that is not three words never keys the radio, and says why.
    #[test]
    fn a_q65_message_that_will_not_pack_is_named() {
        let mut ctl = Q65Controller::new(DigiConfig::default(), DECODE_RATE);
        ctl.set_audio_hz(1000.0);
        ctl.set_tx_text("not a q65 message".into());
        ctl.set_tx_active(true);
        let st = ctl.status();
        assert!(!st.tx_next, "it armed a message it cannot send");
        let why = st.tx_refused.expect("it refused in silence");
        assert!(why.contains("three words"), "{why:?}");
    }
}
