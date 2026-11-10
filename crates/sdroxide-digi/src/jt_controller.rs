//! `JtController` — JT65 and JT9, the weak-signal EME and HF modes.
//!
//! Slotted like FT8 and shaped like [`crate::WsprController`], but with
//! something to show rather than sequence: a JT slot decodes into ordinary
//! [`Decode`]s (`<to> <from> <grid|report>`), so the panel is the decode list
//! the FT8 modes already share. There is no QSO sequencer — a JT65 exchange is
//! a minutes-long handshake and the operator drives it — but the **transmitter
//! is wired**: the message box takes WSJT-X's own three words (`CQ K1ABC FN42`,
//! `K1ABC W1XYZ -15`) and one burst goes out per slot the operator has armed,
//! starting at the mode's own offset into the slot.
//!
//! # Why a worker thread
//!
//! A JT65 scan over a 60-second slot is seconds of work, and JT9's is heavier
//! again. The engine polls the controller on the audio thread, so the decode
//! runs on its own thread and the result is drained from [`Self::poll`] — the
//! same shape [`crate::WsprController`] uses, and for the same reason. One
//! slot in flight at a time: a machine that cannot keep up must drop a slot
//! rather than fall a slot further behind every minute.

use std::sync::mpsc::{Receiver, Sender, TryRecvError};
use std::time::SystemTime;

use sdroxide_dsp::MonoResampler;
use sdroxide_types::{Decode, DigiConfig, DigiStatus, Mode, QsoStep};

use crate::DigiEngine;
use crate::controller::DigiAction;
use crate::modem::decode_jt_slot;
use crate::params::{DECODE_RATE, DigiParams};
use crate::scheduler::SlotScheduler;

/// How late into a slot the burst may still start, in seconds. WSJT-X decodes a
/// dt of about this much either way; past it the message is on the air at a
/// time nobody is listening for.
const JT_LATE_START_S: f64 = 2.0;

/// One slot of audio handed to the decode worker.
struct DecodeJob {
    audio: Vec<i16>,
    mode: Mode,
    slot_utc: i64,
}

/// What the worker sends back.
struct DecodeResult {
    decodes: Vec<Decode>,
}

pub struct JtController {
    mode: Mode,
    params: DigiParams,
    scheduler: SlotScheduler,
    cfg: DigiConfig,
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
    /// Where the operator's audio cursor sits, for the readout and the
    /// passband marker. JT tones are worked anywhere in the passband.
    audio_hz: f32,

    job_tx: Sender<DecodeJob>,
    res_rx: Receiver<DecodeResult>,
    _worker: std::thread::JoinHandle<()>,
    /// A slot dispatched but not yet answered. One at a time: a machine that
    /// cannot keep up drops a slot rather than queueing them.
    pending: bool,

    /// Decodes from the last completed slot, and how many, for the status.
    last_count: u32,
    status_dirty: bool,

    /// The operator's message, in WSJT-X's three words.
    tx_text: String,
    /// Whether the next eligible slot keys the transmitter.
    tx_active: bool,
    /// The burst going out, once synthesised. `None` between over(s).
    burst: Option<crate::controller::BurstPlayer>,
    /// The slot index already transmitted in, so one arming is one burst.
    tx_fired_slot: i64,
    /// Why the message cannot be sent, if it cannot. Recomputed from the text
    /// so the panel and the key-up cannot disagree.
    tx_problem: Option<String>,
}

impl JtController {
    pub fn new(mode: Mode, cfg: DigiConfig, tap_rate: f64) -> Self {
        let params = DigiParams::for_mode(mode);
        let (job_tx, job_rx) = std::sync::mpsc::channel::<DecodeJob>();
        let (res_tx, res_rx) = std::sync::mpsc::channel::<DecodeResult>();
        let worker = std::thread::Builder::new()
            .name("sdroxide-jt-decode".into())
            .spawn(move || {
                while let Ok(job) = job_rx.recv() {
                    let decodes = decode_jt_slot(&job.audio, job.mode, job.slot_utc);
                    if res_tx.send(DecodeResult { decodes }).is_err() {
                        break;
                    }
                }
            })
            .expect("spawn jt decode worker");

        JtController {
            mode,
            params,
            scheduler: SlotScheduler::for_mode(mode),
            cfg,
            resampler: MonoResampler::new(tap_rate, DECODE_RATE),
            slot_buf: Vec::new(),
            buf_aligned: false,
            tap_scratch: Vec::new(),
            last_slot_idx: i64::MIN,
            // WSJT-X's own JT working frequency: 1000 Hz up from the dial.
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
        self.params.slot_samples()
    }

    fn digi_status(&self) -> DigiStatus {
        let mut s = DigiStatus::idle(self.cfg.clone());
        s.mode = self.mode;
        s.step = QsoStep::Idle;
        s.audio_hz = self.audio_hz;
        s.tx_next = self.tx_active;
        s.transmitting = self.burst.is_some();
        s.tx_pending_msg = (!self.tx_text.is_empty()).then(|| self.tx_text.clone());
        // A message that will not pack is named *here*, so the panel says why
        // rather than greying a control with no reason beside it. The reason
        // outlives the arming: a refused key-up leaves `tx_active` false, and
        // that is exactly the moment the operator needs to be told.
        s.tx_refused = self.tx_problem.clone();
        s
    }

    /// The three words WSJT-X's JT message is made of. Anything else is refused
    /// by name instead of being silently truncated into a different message.
    fn jt_words(&self) -> Option<(&str, &str, &str)> {
        let mut it = self.tx_text.split_whitespace();
        let (a, b, c) = (it.next()?, it.next()?, it.next()?);
        if it.next().is_some() {
            return None;
        }
        Some((a, b, c))
    }

    /// Synthesise the operator's message as 12 kHz audio, then resample it to
    /// the 48 kHz the engine wants.
    ///
    /// The packing and the FEC are mfsk-core's — the same `pack_standard` the
    /// decoder unpacks — so the two halves cannot disagree about the message,
    /// only about the waveform. Returns the reason on failure rather than an
    /// empty burst: a key-up that puts out nothing is the case the house rule
    /// about a control that silently does nothing is about.
    /// The MSK144 frame for a three-word message, as complex baseband at 12 kHz.
    ///
    /// The path is WSJT-X's `genmsk_128_90.f90` as mfsk-core splits it: the
    /// 77-bit message, a 13-bit CRC over it, LDPC(128,90), then the channel
    /// framing and OQPSK waveform. The packing is the same `pack77` the decoder
    /// unpacks, so the message cannot disagree between the halves.
    fn msk144_frame(&self, a: &str, b: &str, c: &str) -> Result<Option<Vec<(f32, f32)>>, String> {
        use mfsk_core::engine::FecCodec;
        let Some(bits77) = mfsk_core::msg::wsjt77::pack77(a, b, c) else {
            return Err(format!(
                "\"{a} {b} {c}\" is not a message MSK144 can carry — the third word is a \
                 four-character grid, a report, or free text of the usual shape"
            ));
        };
        let mut info = [0u8; 90];
        info[..77].copy_from_slice(&bits77);
        let mut bytes = [0u8; 12];
        for (i, &bit) in bits77.iter().enumerate() {
            bytes[i / 8] |= (bit & 1) << (7 - (i % 8));
        }
        let crc = mfsk_core::fec::ldpc_128_90::crc13(&bytes);
        for i in 0..13 {
            info[77 + i] = ((crc >> (12 - i)) & 1) as u8;
        }
        let mut codeword = [0u8; 128];
        mfsk_core::fec::Ldpc128_90.encode(&info, &mut codeword);
        let frame = mfsk_core::msk144::tx::synth_codeword_frame(&codeword);
        Ok(Some(frame.iter().map(|c| (c.re, c.im)).collect()))
    }

    /// The frame repeated back to back for the rest of the period, shifted onto
    /// the operator's audio frequency.
    ///
    /// A frame is 864 samples at 12 kHz — 72 ms — and the last thing the period
    /// should do is cut one in half, so the stream stops at the last whole frame
    /// that fits.
    fn msk144_stream(&self, frame: &[(f32, f32)]) -> Vec<f32> {
        let rate = DECODE_RATE;
        let room = ((self.params.slot_s - self.params.tx_offset_s - 0.1) * rate) as usize;
        let frames = room / frame.len();
        let inc = std::f64::consts::TAU * f64::from(self.audio_hz) / rate;
        let mut out = Vec::with_capacity(frames * frame.len());
        let mut n = 0usize;
        for _ in 0..frames {
            for &(re, im) in frame {
                let ph = inc * n as f64;
                out.push(0.5 * (re as f64 * ph.cos() - im as f64 * ph.sin()) as f32);
                n += 1;
            }
        }
        out
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

    /// The same at the modem's own 12 kHz, which is what the decoder takes.
    fn synthesize_burst_12k(&self) -> Result<Vec<f32>, String> {
        let Some((a, b, c)) = self.jt_words() else {
            return Err(format!(
                "{} wants WSJT-X's three words — \"CQ K1ABC FN42\" or \"K1ABC W1XYZ -15\"",
                self.mode.label()
            ));
        };
        let (rate, hz, amp) = (DECODE_RATE as u32, self.audio_hz, 0.5f32);
        let audio = match self.mode {
            Mode::Jt65 => mfsk_core::jt65::tx::synthesize_standard(a, b, c, rate, hz, amp),
            Mode::Jt9 => mfsk_core::jt9::tx::synthesize_standard(a, b, c, rate, hz, amp),
            Mode::Msk144 => {
                // MSK144 is not a "message → tones" mode like the two above: it
                // is one 72 ms frame of OQPSK, transmitted **continuously**
                // through the period so a meteor's brief trail catches part of
                // one. That is what `Mode::slot_timing` says it does, and why
                // the stream is repeated here rather than sent once.
                let Some(frame) = self.msk144_frame(a, b, c)? else { unreachable!() };
                return Ok(self.msk144_stream(&frame));
            }
            other => return Err(format!("{} has no transmitter here", other.label())),
        };
        audio.ok_or_else(|| {
            format!(
                "\"{a} {b} {c}\" is not a message {} can carry — the third word is a \
                 four-character grid or a report, and the two calls have to be plain",
                self.mode.label()
            )
        })
    }
}

impl DigiEngine for JtController {
    fn mode(&self) -> Mode {
        self.mode
    }

    fn on_rx_audio(&mut self, tap: &[f32]) {
        self.tap_scratch.clear();
        match &mut self.resampler {
            Some(r) => r.push(tap, &mut self.tap_scratch),
            None => self.tap_scratch.extend_from_slice(tap),
        }
        // The buffer is the same i16 the slotted FT8 path carries, so mfsk-core
        // and the panel see one scale; `decode_jt_slot` converts back to f32.
        let cap = self.slot_samples() + self.slot_samples() / 8;
        for &s in &self.tap_scratch {
            if self.slot_buf.len() < cap {
                self.slot_buf.push((s.clamp(-1.0, 1.0) * 28_000.0) as i16);
            }
        }
    }

    fn poll(&mut self, now: SystemTime, _dial_hz: f64) -> Vec<DigiAction> {
        let mut actions = Vec::new();

        // 1. Drain the worker.
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

        // 2. Slot boundary: dispatch the completed slot to the worker.
        let idx = self.scheduler.slot_index(now);
        if idx != self.last_slot_idx {
            // Only a slot whose audio began on its own boundary is decoded, and
            // half a slot of it is the floor: less than that is a stream
            // hiccup, not a transmission.
            let min_samples = (self.params.slot_s * DECODE_RATE * 0.5) as usize;
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
        // 3. Transmit. One burst per slot, started inside the window where the
        //    message still lands where the receiver expects it: WSJT-X decodes
        //    a dt of a couple of seconds either way, so a late start is allowed
        //    that far and no further.
        if self.burst.is_none() && idx != self.tx_fired_slot && self.tx_active {
            let into = self.scheduler.secs_into_slot(now);
            let latest = self.params.tx_offset_s + JT_LATE_START_S;
            if into >= self.params.tx_offset_s && into <= latest {
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
        // Recompute the reason now, so arming says whether it can go out.
        self.tx_problem = None;
        self.status_dirty = true;
    }

    fn set_tx_active(&mut self, on: bool) {
        if on {
            // Packing is checked before anything is armed: a message that
            // cannot go out must not key the radio and then say nothing.
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

    /// **JT65 and JT9 transmit, and the decoder this build already had reads
    /// them back.**
    ///
    /// The message is placed where the transmitter puts it — `tx_offset_s` into
    /// a 60-second slot — and handed to the same `decode_jt_slot` the receive
    /// path uses, so the two halves are the program's own packing *and* its own
    /// waveform. What that does not prove is interoperability, which needs a
    /// station; it is written down here so nobody mistakes it for one.
    #[test]
    fn a_jt_transmission_reads_back() {
        for mode in [Mode::Jt65, Mode::Jt9] {
            let params = DigiParams::for_mode(mode);
            let mut ctl = JtController::new(mode, DigiConfig::default(), DECODE_RATE);
            ctl.set_audio_hz(1000.0);
            ctl.set_tx_text("CQ K1ABC FN42".into());
            let burst = ctl.synthesize_burst_12k().expect("the message packs");
            // One slot of audio, with the burst where the transmitter starts it.
            let mut slot = vec![0.0f32; params.slot_samples()];
            let at = (params.tx_offset_s * DECODE_RATE) as usize;
            for (i, s) in burst.iter().enumerate() {
                if let Some(d) = slot.get_mut(at + i) {
                    *d += *s;
                }
            }
            let audio: Vec<i16> =
                slot.iter().map(|v| (v.clamp(-1.0, 1.0) * 28_000.0) as i16).collect();
            let decodes = decode_jt_slot(&audio, mode, 0);
            assert!(
                decodes.iter().any(|d| d.message.contains("K1ABC")),
                "{mode:?} did not read its own transmission back: {:?}",
                decodes.iter().map(|d| &d.message).collect::<Vec<_>>()
            );
        }
    }

    /// **MSK144 transmits too, and its decoder reads the stream back.**
    ///
    /// A different protocol from JT65/JT9 inside the same controller: one 72 ms
    /// OQPSK frame repeated through the period, LDPC(128,90) with a 13-bit CRC
    /// on top of the same 77-bit message. The frame is placed at the mode's own
    /// offset into a 15-second slot and handed to `decode_msk144_slot`, which
    /// scans the whole slot as it does on the air.
    #[test]
    fn an_msk144_transmission_reads_back() {
        let params = DigiParams::for_mode(Mode::Msk144);
        let mut ctl = JtController::new(Mode::Msk144, DigiConfig::default(), DECODE_RATE);
        ctl.set_audio_hz(1500.0);
        ctl.set_tx_text("CQ K1ABC FN42".into());
        let burst = ctl.synthesize_burst_12k().expect("the message packs");
        let mut slot = vec![0.0f32; params.slot_samples()];
        let at = (params.tx_offset_s * DECODE_RATE) as usize;
        for (i, s) in burst.iter().enumerate() {
            if let Some(d) = slot.get_mut(at + i) {
                *d += *s;
            }
        }
        let audio: Vec<i16> = slot.iter().map(|v| (v.clamp(-1.0, 1.0) * 28_000.0) as i16).collect();
        let decodes = crate::decode_msk144_slot(&audio, 0);
        assert!(
            decodes.iter().any(|d| d.message.contains("K1ABC")),
            "MSK144 did not read its own transmission back: {:?}",
            decodes.iter().map(|d| &d.message).collect::<Vec<_>>()
        );
    }

    /// A message that is not WSJT-X's three words is refused **by name**, and
    /// the radio is not keyed: a control that cannot do the thing says so.
    #[test]
    fn a_message_that_will_not_pack_is_named() {
        let mut ctl = JtController::new(Mode::Jt65, DigiConfig::default(), DECODE_RATE);
        ctl.set_audio_hz(1000.0);
        ctl.set_tx_text("this is not a jt message at all".into());
        ctl.set_tx_active(true);
        let st = ctl.status();
        assert!(!st.tx_next, "it armed a message it cannot send");
        assert!(!ctl.tx_burst_active(), "it asked the engine to key");
        let why = st.tx_refused.expect("it refused in silence");
        assert!(why.contains("three words"), "the refusal does not say what is wanted: {why:?}");

        // And a well-formed one arms.
        ctl.set_tx_text("CQ K1ABC FN42".into());
        ctl.set_tx_active(true);
        let st = ctl.status();
        assert!(st.tx_next, "it refused a message that packs");
        assert!(st.tx_refused.is_none(), "{:?}", st.tx_refused);
    }
}
