//! [`RadeWorker`] — RADE driven on its own thread, with lock-free rings at the
//! edges.
//!
//! Acquisition, demodulation and neural decode all happen here, on one thread,
//! because the inference cost is small but variable and must not land on the
//! engine's sample-rate loop. Audio crosses in and out through `rtrb` rings at
//! the caller's rate; every sample-rate conversion lives inside the worker so
//! there is exactly one place where the clock domains meet.
//!
//! Shape follows `SkimmerController`: bounded rings for realtime data (dropped
//! under backpressure, counted), an unbounded channel for control (never
//! dropped).

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI32, AtomicU64, Ordering};
use std::thread::JoinHandle;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender, unbounded};
use num_complex::Complex32;
use sdroxide_dsp::MonoResampler;
use tracing::{info, warn};

use crate::{
    MODEM_RATE, RX_REAL_SCALE, Rade, RadeError, SPEECH_RATE, TX_REAL_SCALE, VocoderDec, VocoderEnc,
};

/// Ring capacity, in samples at the ring's own rate — one second, so a stall
/// anywhere shows up as dropped samples rather than unbounded growth.
const RING_SECS: f64 = 1.0;
/// How long the worker blocks waiting for a control message before servicing
/// the rings again. RADE frames are 120 ms, so this is comfortably fine-grained.
const POLL: Duration = Duration::from_millis(5);
/// Speech level decays this fast per output block, so the UI meter falls back.
const LEVEL_DECAY: f32 = 0.85;

enum Ctl {
    SetTx(bool),
    /// The callsign to transmit in the End-of-Over frame. Re-encoded on change
    /// so keying up costs nothing.
    SetCallsign(String),
    /// Drop receive state — used when re-entering the mode or after a big
    /// discontinuity.
    Reset,
    Stop,
}

/// A remote station's callsign, recovered from its End-of-Over frame.
///
/// This is the only station identification RADE carries, and it is what a
/// FreeDV Reporter reception report is built from.
#[derive(Debug, Clone, PartialEq)]
pub struct RadeTextRx {
    pub call: String,
    /// The SNR the receiver was reporting while the over was still in sync.
    pub snr_db: f32,
}

#[derive(Default)]
struct Shared {
    sync: AtomicBool,
    snr_mdb: AtomicI32,
    foff_mhz: AtomicI32,
    level_q16: AtomicI32,
    eoo_count: AtomicU64,
    dropped: AtomicU64,
    /// Set once the end-of-over frame has been generated and the TX chain has
    /// nothing further to emit.
    tx_finished: AtomicBool,
}

/// A snapshot of receive state for the UI.
#[derive(Debug, Clone, Copy, Default)]
pub struct RadeStats {
    /// Receiver is locked to a signal.
    pub sync: bool,
    /// SNR estimate in a 3 kHz noise bandwidth (valid while `sync`).
    pub snr_db: f32,
    /// Frequency offset of the received signal (valid while `sync`).
    pub freq_offset_hz: f32,
    /// Peak level of decoded speech, 0..1.
    pub rx_level: f32,
    /// Count of End-of-Over frames seen; a change means the far end unkeyed.
    pub eoo_count: u64,
    /// Samples dropped at a ring boundary since start.
    pub dropped: u64,
}

/// Handle to the RADE worker thread.
///
/// Push receive audio in with [`RadeWorker::push_rx`], take decoded speech out
/// with [`RadeWorker::pop_rx`]; mirror that with [`RadeWorker::push_mic`] /
/// [`RadeWorker::pop_tx`] while transmitting.
pub struct RadeWorker {
    rx_in: rtrb::Producer<f32>,
    rx_out: rtrb::Consumer<f32>,
    tx_in: rtrb::Producer<f32>,
    tx_out: rtrb::Consumer<f32>,
    ctl: Sender<Ctl>,
    text_rx: Receiver<RadeTextRx>,
    shared: Arc<Shared>,
    thread: Option<JoinHandle<()>>,
}

impl RadeWorker {
    /// Spawn the worker.
    ///
    /// `rx_rate` is the rate of the audio pushed to [`RadeWorker::push_rx`]
    /// (the engine's demod tap); `audio_rate` is the rate of the decoded speech
    /// and of the microphone audio, i.e. the app's audio rate.
    pub fn new(rx_rate: f64, audio_rate: f64) -> Result<Self, RadeError> {
        // Construct the C state on this thread so a failure is reported to the
        // caller instead of killing the worker at startup.
        let rade = Rade::open_v1()?;
        let enc = VocoderEnc::new()?;
        let dec = VocoderDec::new()?;

        let (rx_in, rx_in_rx) = rtrb::RingBuffer::new((rx_rate * RING_SECS) as usize);
        let (rx_out_tx, rx_out) = rtrb::RingBuffer::new((audio_rate * RING_SECS) as usize);
        let (tx_in, tx_in_rx) = rtrb::RingBuffer::new((audio_rate * RING_SECS) as usize);
        let (tx_out_tx, tx_out) = rtrb::RingBuffer::new((audio_rate * RING_SECS) as usize);
        let (ctl, ctl_rx) = unbounded();
        let (text_tx, text_rx) = unbounded();
        let shared = Arc::new(Shared::default());

        // Never leave the End-of-Over payload at the library's all-zero
        // default: even with no callsign the frame carries the known sequence
        // the far end measures its noise floor from.
        let mut eoo_tx = vec![0.0f32; rade.n_eoo_bits()];
        crate::text::encode("", &mut eoo_tx);

        let inner = Inner {
            rade,
            enc,
            dec,
            rx_in: rx_in_rx,
            rx_out: rx_out_tx,
            tx_in: tx_in_rx,
            tx_out: tx_out_tx,
            shared: shared.clone(),
            rx_down: MonoResampler::new(rx_rate, MODEM_RATE),
            rx_up: MonoResampler::new(SPEECH_RATE, audio_rate),
            tx_down: MonoResampler::new(audio_rate, SPEECH_RATE),
            tx_up: MonoResampler::new(MODEM_RATE, audio_rate),
            buf8: Vec::new(),
            buf16: Vec::new(),
            features: Vec::new(),
            tx_features: Vec::new(),
            scratch: Vec::new(),
            audio: Vec::new(),
            pcm: Vec::new(),
            iq: Vec::new(),
            eoo_tx,
            eoo_rx: Vec::new(),
            last_sync_snr: 0.0,
            text_tx,
            transmitting: false,
            was_sync: false,
            level: 0.0,
        };

        let thread = std::thread::Builder::new()
            .name("sdroxide-rade".into())
            .spawn(move || inner.run(ctl_rx))
            .expect("spawn sdroxide-rade thread");

        info!(rx_rate, audio_rate, "RADE worker started");
        Ok(RadeWorker { rx_in, rx_out, tx_in, tx_out, ctl, text_rx, shared, thread: Some(thread) })
    }

    /// Set the callsign transmitted in the End-of-Over frame. Empty clears it.
    ///
    /// This is how other FreeDV stations — and, through them, FreeDV Reporter —
    /// learn who we are.
    pub fn set_callsign(&self, call: &str) {
        let _ = self.ctl.send(Ctl::SetCallsign(call.to_string()));
    }

    /// Drain the callsigns decoded from remote End-of-Over frames since the
    /// last call. Usually empty: one arrives per received over.
    pub fn poll_text(&self) -> Vec<RadeTextRx> {
        self.text_rx.try_iter().collect()
    }

    /// Hand the worker receive audio at the rate given to [`RadeWorker::new`].
    pub fn push_rx(&mut self, audio: &[f32]) {
        push(&mut self.rx_in, audio, &self.shared);
    }

    /// Free space in the receive ring, in samples.
    ///
    /// Live audio arrives in real time and never needs this; a caller feeding
    /// from a file can use it to avoid outrunning the worker, whose decode is
    /// faster than real time but not instant.
    pub fn rx_free(&self) -> usize {
        self.rx_in.slots()
    }

    /// Drain decoded speech, appending to `out`. Returns how many samples were
    /// taken.
    pub fn pop_rx(&mut self, out: &mut Vec<f32>) -> usize {
        let mut n = 0;
        while let Ok(s) = self.rx_out.pop() {
            out.push(s);
            n += 1;
        }
        n
    }

    /// Hand the worker microphone audio while transmitting.
    pub fn push_mic(&mut self, audio: &[f32]) {
        push(&mut self.tx_in, audio, &self.shared);
    }

    /// Microphone audio pushed but not yet modulated, in samples.
    ///
    /// The transmit-side counterpart of [`RadeWorker::rx_free`], and needed for
    /// the same reason: a live microphone delivers in real time and never gets
    /// ahead, but a caller feeding from a file has nothing pacing it. Free
    /// space is the wrong thing to watch there — the worker converts the whole
    /// backlog in one wake, so letting this ring fill only moves the overflow
    /// to the modulated side. Keep the backlog shallow instead.
    pub fn tx_pending(&self) -> usize {
        self.tx_in.buffer().capacity() - self.tx_in.slots()
    }

    /// Fill `out` with modulated modem audio, padding with silence when the
    /// worker has not produced enough yet (which is normal for the first
    /// ~120 ms of an over, while the first modem frame is still being built).
    ///
    /// Returns how many leading samples were real. A sound card wants the
    /// whole buffer filled and can ignore this; a caller splicing the result
    /// into a stream has to know, because the padding is silence inserted into
    /// the signal rather than part of it.
    pub fn pop_tx(&mut self, out: &mut [f32]) -> usize {
        let mut n = 0;
        for s in out.iter_mut() {
            match self.tx_out.pop() {
                Ok(v) => {
                    *s = v;
                    n += 1;
                }
                Err(_) => *s = 0.0,
            }
        }
        n
    }

    /// Enter or leave transmit. Leaving queues the End-of-Over frame; watch
    /// [`RadeWorker::tx_drained`] to know when it is safe to unkey.
    pub fn set_tx(&self, on: bool) {
        if on {
            self.shared.tx_finished.store(false, Ordering::Release);
        }
        let _ = self.ctl.send(Ctl::SetTx(on));
    }

    /// True once the End-of-Over frame has been generated and every modulated
    /// sample has been read out.
    pub fn tx_drained(&self) -> bool {
        self.shared.tx_finished.load(Ordering::Acquire) && self.tx_out.is_empty()
    }

    /// Drop receive state (sync, vocoder warm-up, buffered audio).
    pub fn reset(&self) {
        let _ = self.ctl.send(Ctl::Reset);
    }

    /// Current receive state, for the UI.
    pub fn stats(&self) -> RadeStats {
        let s = &self.shared;
        RadeStats {
            sync: s.sync.load(Ordering::Relaxed),
            snr_db: s.snr_mdb.load(Ordering::Relaxed) as f32 / 1000.0,
            freq_offset_hz: s.foff_mhz.load(Ordering::Relaxed) as f32 / 1000.0,
            rx_level: s.level_q16.load(Ordering::Relaxed) as f32 / 65536.0,
            eoo_count: s.eoo_count.load(Ordering::Relaxed),
            dropped: s.dropped.load(Ordering::Relaxed),
        }
    }
}

impl Drop for RadeWorker {
    fn drop(&mut self) {
        let _ = self.ctl.send(Ctl::Stop);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

fn push(ring: &mut rtrb::Producer<f32>, audio: &[f32], shared: &Shared) {
    if ring.slots() < audio.len() {
        let n =
            shared.dropped.fetch_add(audio.len() as u64, Ordering::Relaxed) + audio.len() as u64;
        if n.is_power_of_two() {
            warn!(dropped = n, "RADE ring full, dropping audio");
        }
        return;
    }
    for &s in audio {
        let _ = ring.push(s);
    }
}

struct Inner {
    rade: Rade,
    enc: VocoderEnc,
    dec: VocoderDec,

    rx_in: rtrb::Consumer<f32>,
    rx_out: rtrb::Producer<f32>,
    tx_in: rtrb::Consumer<f32>,
    tx_out: rtrb::Producer<f32>,
    shared: Arc<Shared>,

    rx_down: Option<MonoResampler>, // tap rate -> 8 kHz
    rx_up: Option<MonoResampler>,   // 16 kHz -> audio rate
    tx_down: Option<MonoResampler>, // audio rate -> 16 kHz
    tx_up: Option<MonoResampler>,   // 8 kHz -> audio rate

    buf8: Vec<f32>,        // modem-rate receive audio awaiting `nin`
    buf16: Vec<f32>,       // 16 kHz mic audio awaiting a vocoder frame
    features: Vec<f32>,    // rade_rx output
    tx_features: Vec<f32>, // vocoder output awaiting a full modem frame
    scratch: Vec<f32>,
    audio: Vec<f32>,
    pcm: Vec<i16>,
    iq: Vec<Complex32>,

    /// Our callsign, pre-encoded as End-of-Over soft bits so keying down costs
    /// nothing. Always `rade.n_eoo_bits()` long.
    eoo_tx: Vec<f32>,
    /// Scratch for the End-of-Over bits `rade_rx` hands back.
    eoo_rx: Vec<f32>,
    /// The last SNR seen while in sync. `snr_mdb` freezes when sync drops, and
    /// the End-of-Over frame is the last thing to arrive before that happens,
    /// so the report needs the latched value rather than a later read.
    last_sync_snr: f32,
    text_tx: Sender<RadeTextRx>,

    transmitting: bool,
    was_sync: bool,
    level: f32,
}

impl Inner {
    fn run(mut self, ctl: Receiver<Ctl>) {
        loop {
            match ctl.recv_timeout(POLL) {
                Ok(Ctl::Stop) | Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
                Ok(Ctl::SetTx(on)) => self.set_tx(on),
                Ok(Ctl::SetCallsign(c)) => crate::text::encode(&c, &mut self.eoo_tx),
                Ok(Ctl::Reset) => self.reset_rx(),
                Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
            }
            if self.transmitting {
                self.service_tx();
            } else {
                self.service_rx();
            }
        }
        info!("RADE worker stopped");
    }

    fn set_tx(&mut self, on: bool) {
        if on == self.transmitting {
            return;
        }
        self.transmitting = on;
        if on {
            // Entering transmit: discard anything still queued for receive so
            // the next over starts clean.
            while self.rx_in.pop().is_ok() {}
            self.buf8.clear();
            self.reset_rx();
        } else {
            // Leaving transmit: flush whatever mic audio is left, then emit the
            // End-of-Over frame the far end uses to close the over.
            self.service_tx();
            // A modem frame carries 120 ms of speech; zero-pad the last partial
            // one so the tail of the over goes out rather than being cut.
            if !self.tx_features.is_empty() {
                let n = self.rade.n_features();
                self.tx_features.resize(n, 0.0);
                let block = std::mem::take(&mut self.tx_features);
                self.iq.clear();
                match self.rade.tx(&block, &mut self.iq) {
                    Ok(()) => self.emit_tx_iq(),
                    Err(e) => warn!(?e, "rade_tx failed flushing the over"),
                }
            }
            self.iq.clear();
            // Load our callsign into the frame about to go out. Done here
            // rather than when the callsign changes, so the value is current
            // even if the operator edited it mid-over.
            if let Err(e) = self.rade.set_tx_eoo_bits(&self.eoo_tx) {
                warn!(?e, "rade_tx_set_eoo_bits failed");
            }
            if let Err(e) = self.rade.tx_eoo(&mut self.iq) {
                warn!(?e, "rade_tx_eoo failed");
            }
            self.emit_tx_iq();
            self.buf16.clear();
            self.tx_features.clear();
            while self.tx_in.pop().is_ok() {}
            self.shared.tx_finished.store(true, Ordering::Release);
        }
    }

    fn reset_rx(&mut self) {
        self.dec.reset();
        self.shared.sync.store(false, Ordering::Relaxed);
        self.level = 0.0;
    }

    fn service_rx(&mut self) {
        // Tap audio -> modem rate.
        self.scratch.clear();
        while let Ok(s) = self.rx_in.pop() {
            self.scratch.push(s);
        }
        if self.scratch.is_empty() && self.buf8.len() < self.rade.nin() {
            return;
        }
        match self.rx_down.as_mut() {
            Some(r) => r.push(&self.scratch, &mut self.buf8),
            None => self.buf8.extend_from_slice(&self.scratch),
        }

        // `nin` moves between calls as timing tracking pulls the clock, so it
        // is re-read every iteration.
        while self.buf8.len() >= self.rade.nin() {
            let nin = self.rade.nin();
            self.iq.clear();
            self.iq
                .extend(self.buf8[..nin].iter().map(|&s| Complex32::new(s * RX_REAL_SCALE, 0.0)));
            self.buf8.drain(..nin);

            let out = match self.rade.rx(&self.iq, &mut self.features, &mut self.eoo_rx) {
                Ok(o) => o,
                Err(e) => {
                    warn!(?e, "rade_rx failed");
                    continue;
                }
            };
            // Before the End-of-Over handling, so a frame that arrived while
            // still in sync refreshes the SNR the report will carry.
            self.publish_rx_state();
            if out.has_eoo {
                self.shared.eoo_count.fetch_add(1, Ordering::Relaxed);
                // Tens of microseconds of belief propagation, once per over,
                // on this thread — never the audio callback.
                if let Some(call) = crate::text::decode(&self.eoo_rx) {
                    info!(%call, snr_db = self.last_sync_snr, "RADE End-of-Over callsign");
                    let _ = self.text_tx.send(RadeTextRx { call, snr_db: self.last_sync_snr });
                }
            }
            if out.n_features == 0 {
                continue;
            }
            self.synthesize();
        }
    }

    /// Vocode the feature vectors just returned and push the speech out.
    fn synthesize(&mut self) {
        let n_feat = crate::vocoder::n_features();
        self.pcm.clear();
        for frame in self.features.chunks_exact(n_feat) {
            if let Err(e) = self.dec.decode(frame, &mut self.pcm) {
                warn!(?e, "vocoder decode failed");
            }
        }
        if self.pcm.is_empty() {
            return; // still warming FARGAN up
        }
        self.scratch.clear();
        self.scratch.extend(self.pcm.iter().map(|&s| s as f32 / 32768.0));

        let peak = self.scratch.iter().fold(0.0f32, |a, s| a.max(s.abs()));
        self.level = (self.level * LEVEL_DECAY).max(peak);
        self.shared
            .level_q16
            .store((self.level.clamp(0.0, 1.0) * 65536.0) as i32, Ordering::Relaxed);

        self.audio.clear();
        match self.rx_up.as_mut() {
            Some(r) => r.push(&self.scratch, &mut self.audio),
            None => self.audio.extend_from_slice(&self.scratch),
        }
        for &s in &self.audio {
            if self.rx_out.push(s.clamp(-1.0, 1.0)).is_err() {
                let n = self.shared.dropped.fetch_add(1, Ordering::Relaxed) + 1;
                if n.is_power_of_two() {
                    warn!(dropped = n, "RADE speech ring full, dropping");
                }
            }
        }
    }

    fn publish_rx_state(&mut self) {
        let sync = self.rade.sync();
        if self.was_sync && !sync {
            // Losing sync ends the over: start the vocoder's warm-up again so
            // the next one doesn't continue from stale state.
            self.dec.reset();
        }
        self.was_sync = sync;
        self.shared.sync.store(sync, Ordering::Relaxed);
        if sync {
            let snr = self.rade.snr_3k_db();
            // Latched for the End-of-Over report: by the time an over's
            // callsign is decoded, sync may already have dropped and the
            // published value stopped moving.
            self.last_sync_snr = snr;
            self.shared.snr_mdb.store((snr * 1000.0) as i32, Ordering::Relaxed);
            self.shared
                .foff_mhz
                .store((self.rade.freq_offset_hz() * 1000.0) as i32, Ordering::Relaxed);
        }
    }

    fn service_tx(&mut self) {
        // Mic -> 16 kHz speech.
        self.scratch.clear();
        while let Ok(s) = self.tx_in.pop() {
            self.scratch.push(s);
        }
        match self.tx_down.as_mut() {
            Some(r) => r.push(&self.scratch, &mut self.buf16),
            None => self.buf16.extend_from_slice(&self.scratch),
        }

        // 16 kHz speech -> feature vectors.
        let frame = self.enc.frame_size();
        while self.buf16.len() >= frame {
            self.pcm.clear();
            self.pcm.extend(self.buf16[..frame].iter().copied().map(crate::f32_to_i16));
            self.buf16.drain(..frame);
            if let Err(e) = self.enc.encode(&self.pcm, &mut self.tx_features) {
                warn!(?e, "vocoder encode failed");
            }
        }

        // Feature vectors -> modem frames.
        let n = self.rade.n_features();
        while self.tx_features.len() >= n {
            self.iq.clear();
            let block: Vec<f32> = self.tx_features.drain(..n).collect();
            if let Err(e) = self.rade.tx(&block, &mut self.iq) {
                warn!(?e, "rade_tx failed");
                continue;
            }
            self.emit_tx_iq();
        }
    }

    /// Take the real part of the modulated IQ, scale it to full-scale audio and
    /// resample it up for the transmit chain.
    fn emit_tx_iq(&mut self) {
        if self.iq.is_empty() {
            return;
        }
        self.scratch.clear();
        self.scratch.extend(self.iq.iter().map(|z| z.re * TX_REAL_SCALE));
        self.audio.clear();
        match self.tx_up.as_mut() {
            Some(r) => r.push(&self.scratch, &mut self.audio),
            None => self.audio.extend_from_slice(&self.scratch),
        }
        for &s in &self.audio {
            if self.tx_out.push(s.clamp(-1.0, 1.0)).is_err() {
                let n = self.shared.dropped.fetch_add(1, Ordering::Relaxed) + 1;
                if n.is_power_of_two() {
                    warn!(dropped = n, "RADE modem ring full, dropping");
                }
            }
        }
    }
}
