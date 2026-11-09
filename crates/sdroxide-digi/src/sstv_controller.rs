//! `SstvController` — the image-mode counterpart to the text/slotted digital
//! controllers. RX decodes incoming pictures scanline-by-scanline (emitting one
//! action per row so the UI paints progressively); TX encodes a composed image
//! that the UI has already cropped, captioned, and PNG-decoded into RGB.

use std::time::SystemTime;

use sdroxide_dsp::{SstvEvent, SstvRx, SstvTx};
use sdroxide_types::{DigiConfig, DigiStatus, Mode, QsoStep, SstvMode, SstvStatus, TranscriptLine};

use crate::controller::DigiAction;
use crate::DigiEngine;

/// TX audio sample rate (injected as "mic", then USB-modulated).
const OUT_RATE: f64 = 48_000.0;

/// The **primary** sign, and the one that does the work: how much neighbouring
/// rows agree, as a Pearson correlation over the finished frame.
///
/// Measured on the bench (2026-11-04): speckle scores 0.00–0.20 between rows,
/// real pictures 0.38–0.96 — including the ones whose syncs were poor, and
/// including a picture whose top third arrived before the signal went. A
/// threshold in the gap, and deliberately low, because a weak picture is worth
/// keeping.
const SAVE_ROW_AGREEMENT: f32 = 0.25;

/// Whether a finished frame is a picture worth keeping.
///
/// **One sign, and it is the rows.** The share of lines that found their sync
/// was the other, and the bench retired it: of nine frames kept in one evening,
/// four were speckle with a sliver of picture at the top, and their rows scored
/// 0.03–0.10 while the share had let them through (2026-11-04). A share is high
/// on noise that happens to put a pulse near where a pulse belongs; a frame
/// whose rows have nothing to do with each other is not a picture whatever its
/// syncs did.
///
/// The casualty is a genuinely flat frame — a black or uniform picture scores
/// near zero on the rows too. That is what SAVE ALL is for, and it is why the
/// gate is a filter with a switch rather than a silent loss.
fn keep_frame(rows: f32) -> bool {
    rows >= SAVE_ROW_AGREEMENT
}

/// How much one row of `rgb` agrees with the next, as a Pearson correlation:
/// 1.0 for a picture that repeats across the frame, 0.0 for speckle, whose
/// rows have nothing to do with each other.
///
/// Cheap enough to run once per picture: two sums over the frame.
fn row_agreement(rgb: &[u8], w: usize, h: usize) -> f32 {
    if w == 0 || h < 2 || rgb.len() < w * h * 3 {
        return 0.0;
    }
    let row = |y: usize| &rgb[y * w * 3..(y + 1) * w * 3];
    let (mut n, mut sa, mut sb, mut saa, mut sbb, mut sab) =
        (0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64);
    for y in 0..h - 1 {
        for (&a, &b) in row(y).iter().zip(row(y + 1)) {
            let (a, b) = (f64::from(a), f64::from(b));
            n += 1.0;
            sa += a;
            sb += b;
            saa += a * a;
            sbb += b * b;
            sab += a * b;
        }
    }
    if n < 1.0 {
        return 0.0;
    }
    let cov = sab / n - (sa / n) * (sb / n);
    let va = saa / n - (sa / n).powi(2);
    let vb = sbb / n - (sb / n).powi(2);
    let den = (va * vb).sqrt();
    if den <= f64::EPSILON { 0.0 } else { (cov / den) as f32 }
}

pub struct SstvController {
    cfg: DigiConfig,
    dial_hz: f64,
    /// Which of the two SSTV modes this controller is running —
    /// [`Mode::Sstv`] on a sideband, [`Mode::SstvFm`] on an FM carrier. The
    /// picture, the decoder and the panel are identical; only the radio
    /// underneath differs, and the engine has already arranged that. Carried so
    /// the mode reported back is the one the operator chose, exactly as
    /// `PacketController` carries its own.
    mode: Mode,

    // RX
    rx: SstvRx,
    rx_scratch: Vec<SstvEvent>,
    /// Accumulating RGB of the image currently being received.
    rx_image: Vec<u8>,
    rx_mode: SstvMode,
    rx_w: u16,
    rx_h: u16,
    rx_active: bool,
    detected: Option<SstvMode>,
    /// The callsign the last station sent in its FSK ID, held until another one
    /// does. It arrives after the picture, so it cannot travel with it.
    rx_id: Option<String>,
    /// The last header for a mode this build cannot draw — see
    /// [`sdroxide_types::SstvStatus::unsupported`].
    unsupported: Option<String>,
    /// Why the last finished frame was not kept, for the panel to show.
    not_saved: Option<sdroxide_types::SstvDrop>,
    image_id: u32,

    // TX
    tx: Option<SstvTx>,
    tx_mode: SstvMode,
    /// Auto mode: RX auto-detects; TX defaults to Martin 1 until a mode is heard.
    auto: bool,
    keyed: bool,
    /// Samples of silence still owed before the picture's calibration header
    /// goes out, from [`DigiConfig::sstv_txdelay_ms`] (issue #351).
    ///
    /// Keying the transmitter and being on the air are not the same instant on
    /// a CAT rig, and the part of an SSTV frame that lands in the gap between
    /// them is the leader and VIS code — the part a decoder needs *before* it
    /// will start a picture at all. Losing it does not shorten the image, it
    /// loses the whole transmission, which is what an OpenWebRX on the far end
    /// showed: nothing. Counted down in [`Self::fill_tx_block`] rather than
    /// waited out in `poll`, so the block cadence is untouched and the rig
    /// simply carries dead air for as long as it needs.
    tx_lead: usize,

    // Actions queued for the next `poll`.
    queued: Vec<DigiAction>,
    status_dirty: bool,
    last_status: Option<SystemTime>,
}

impl SstvController {
    pub fn new(mode: Mode, cfg: DigiConfig, tap_rate: f64) -> Self {
        let mut rx = SstvRx::new(tap_rate);
        // Default to Auto: the RX auto-detects the mode; TX defaults to Martin 1.
        rx.set_expected(None);
        rx.set_sync_pulses(cfg.sstv_sync_pulses);
        SstvController {
            cfg,
            dial_hz: 0.0,
            mode,
            rx,
            rx_scratch: Vec::new(),
            rx_image: Vec::new(),
            rx_mode: SstvMode::default(),
            rx_w: 0,
            rx_h: 0,
            rx_active: false,
            detected: None,
            rx_id: None,
            unsupported: None,
            not_saved: None,
            image_id: 0,
            tx: None,
            tx_mode: SstvMode::Martin1,
            auto: true,
            keyed: false,
            tx_lead: 0,
            queued: Vec::new(),
            status_dirty: true,
            last_status: None,
        }
    }

    fn sstv_status(&self) -> SstvStatus {
        let progress = if self.keyed {
            self.tx.as_ref().map(|t| t.progress()).unwrap_or(0.0)
        } else if self.rx_active {
            self.rx.progress()
        } else {
            0.0
        };
        SstvStatus {
            tx_mode: self.tx_mode,
            tx_active: self.keyed,
            rx_active: self.rx_active,
            detected: self.detected,
            progress,
            signal: self.rx.level(),
            unsupported: self.unsupported.clone(),
            rx_id: self.rx_id.clone(),
            not_saved: self.not_saved.clone(),
        }
    }

    /// A minimal FT8-style status; SSTV state travels via `DigiAction::SstvStatus`.
    fn digi_status(&self) -> DigiStatus {
        DigiStatus {
            stanag: None,
            mode: self.mode,
            step: QsoStep::Idle,
            dx_call: None,
            dx_grid: None,
            tx_next: self.keyed,
            tx_pending_msg: None,
            audio_hz: 1500.0,
            tx_even: false,
            transmitting: self.keyed,
            tx_watchdog: false,
            tx_refused: None,
            transcript: Vec::<TranscriptLine>::new(),
            config: self.cfg.clone(),
            text_rx: String::new(),
            tx_sent: 0,
            fsq_heard: Vec::new(),
            fsq_messages: Vec::new(),
            rade: None,
            packet: None,
            navtex: None,
            acars: None,
            dsc: None,
            uvpacket: None,
            jtty: None,
            ale: None,
            aprs: None,
            js8: None,
            atchat: None,
            fox_queue: Vec::new(),
            call_queue: Vec::new(),
            clock_offset_s: None,
            cw: None,
            wspr: None,
            pi4: None,
            qso: None,
        }
    }
}

impl DigiEngine for SstvController {
    fn mode(&self) -> Mode {
        self.mode
    }

    fn on_rx_audio(&mut self, tap: &[f32]) {
        // Receive sensitivity: a manual gain on the audio fed to the decoder.
        // Auto (or 0) leaves the level untouched; the decoder itself is not
        // changed. Persisted through DigiConfig.
        let gain = if self.cfg.sstv_rx_auto || self.cfg.sstv_rx_sensitivity == 0 {
            1.0f32
        } else {
            10f32.powf(self.cfg.sstv_rx_sensitivity as f32 * 2.0 / 20.0)
        };
        self.rx_scratch.clear();
        let mut events = std::mem::take(&mut self.rx_scratch);
        if (gain - 1.0).abs() < f32::EPSILON {
            self.rx.process(tap, &mut events);
        } else {
            let scaled: Vec<f32> = tap.iter().map(|&s| s * gain).collect();
            self.rx.process(&scaled, &mut events);
        }
        for e in events.drain(..) {
            match e {
                SstvEvent::ModeDetected(mode) => {
                    self.image_id = self.image_id.wrapping_add(1);
                    self.rx_mode = mode;
                    let (w, h) = mode.dimensions();
                    self.rx_w = w;
                    self.rx_h = h;
                    self.rx_image = vec![0u8; w as usize * h as usize * 3];
                    self.rx_active = true;
                    self.detected = Some(mode);
                    // A picture is starting, so whatever could not be drawn
                    // last time is no longer what is on the air.
                    self.unsupported = None;
                    // RX mode determines the next transmit mode. In Auto, keep the
                    // RX auto-detecting; otherwise pin free-run to this mode.
                    self.tx_mode = mode;
                    if !self.auto {
                        self.rx.set_expected(Some(mode));
                    }
                    self.status_dirty = true;
                }
                SstvEvent::Line { y, rgb } => {
                    let w = self.rx_w as usize;
                    let row = y as usize * w * 3;
                    if row + rgb.len() <= self.rx_image.len() {
                        self.rx_image[row..row + rgb.len()].copy_from_slice(&rgb);
                    }
                    self.queued.push(DigiAction::SstvLine { image_id: self.image_id, y, rgb });
                    self.status_dirty = true;
                }
                SstvEvent::FskId(id) => {
                    // A station identifying itself. Worth keeping even with no
                    // picture behind it: a receiver tuned in halfway through a
                    // transmission gets no VIS and no whole frame, and the
                    // callsign at the end is then the only thing that arrives.
                    self.queued.push(DigiAction::SstvId(id.clone()));
                    self.rx_id = Some(id);
                    self.status_dirty = true;
                }
                SstvEvent::UnsupportedMode { code, name } => {
                    // The receiver read the header and knows exactly what it
                    // cannot draw. Naming it is the difference between "the
                    // SSTV decoder is broken" and "that station is sending
                    // Pasokon P3" (issue #421).
                    let label = match name {
                        Some(n) => n.to_string(),
                        None => format!("VIS ${code:02X}"),
                    };
                    tracing::info!(
                        "SSTV: a header arrived for {label}, which this build does not decode"
                    );
                    self.unsupported = Some(label);
                    self.status_dirty = true;
                }
                SstvEvent::GaveUp => {
                    self.not_saved = Some(sdroxide_types::SstvDrop::NotAPicture);
                    self.rx_active = false;
                    self.rx_image.clear();
                    self.image_id = self.image_id.wrapping_add(1);
                    self.detected = None;
                    self.status_dirty = true;
                }
                SstvEvent::ImageComplete { sync_share } => {
                    // **Not everything that decodes is a picture.** A frame the
                    // receiver locked onto noise draws itself and would be
                    // saved to the gallery: 9 of the 54 images on the bench were
                    // speckle, with rows that had nothing to do with each other
                    // (2026-11-04). The share of lines that found their sync is
                    // logged beside it because it is still worth reading, but it
                    // is no longer a bar: noise reaches it too.
                    let rows =
                        row_agreement(&self.rx_image, self.rx_w as usize, self.rx_h as usize);
                    // The operator can have every frame, doubtful ones
                    // included: the gate's margin is narrow and a near-flat
                    // picture lives on its sync alone (review, 2026-11-04).
                    let keep = self.cfg.sstv_save_all || keep_frame(rows);
                    tracing::info!(
                        "SSTV: {}x{} {:?} complete — sync {:.2}, rows {:.2} → {}",
                        self.rx_w,
                        self.rx_h,
                        self.rx_mode,
                        sync_share,
                        rows,
                        if keep { "kept" } else { "not a picture, not saved" }
                    );
                    if keep {
                        self.not_saved = None;
                        self.queued.push(DigiAction::SstvImage {
                            image_id: self.image_id,
                            mode: self.rx_mode,
                            w: self.rx_w,
                            h: self.rx_h,
                            rgb: std::mem::take(&mut self.rx_image),
                        });
                    }
                    if !keep {
                        // Syncs everywhere and no picture in the rows is a
                        // different failure from noise that looked like a
                        // header, and worth saying apart.
                        self.not_saved = Some(sdroxide_types::SstvDrop::NotAPicture);
                    }
                    self.rx_active = false;
                    self.status_dirty = true;
                }
            }
        }
        self.rx_scratch = events; // give the buffer back for reuse
    }

    fn poll(&mut self, now: SystemTime, dial_hz: f64) -> Vec<DigiAction> {
        self.dial_hz = dial_hz;
        let mut actions = std::mem::take(&mut self.queued);
        if self.tx.is_some() && !self.keyed {
            self.keyed = true;
            self.status_dirty = true;
            actions.push(DigiAction::KeyTx);
        }
        // Emit status on change, and at least a few times a second regardless so
        // the signal-level meter and progress stay live even while hunting.
        let periodic = match self.last_status {
            Some(t) => now.duration_since(t).map(|d| d.as_millis() >= 150).unwrap_or(true),
            None => true,
        };
        if self.status_dirty || periodic {
            self.status_dirty = false;
            self.last_status = Some(now);
            actions.push(DigiAction::SstvStatus(self.sstv_status()));
        }
        actions
    }

    fn tx_burst_active(&self) -> bool {
        self.keyed
    }

    fn fill_tx_block(&mut self, out: &mut [f32]) -> bool {
        // Dead air first, and only what is left of this block after it: rounding
        // the lead up to a whole block would put the header where the block
        // cadence happened to fall, and on a sound-card rig a block is 341 ms.
        let lead = self.tx_lead.min(out.len());
        if lead > 0 {
            self.tx_lead -= lead;
            out[..lead].fill(0.0);
        }
        let out = &mut out[lead..];
        match &mut self.tx {
            Some(tx) => {
                tx.next_block(out);
                self.status_dirty = true;
                tx.done()
            }
            None => {
                out.fill(0.0);
                true
            }
        }
    }

    fn on_burst_done(&mut self) {
        self.tx = None;
        self.tx_lead = 0;
        self.keyed = false;
        self.status_dirty = true;
    }

    fn abort(&mut self) {
        self.abort_tx();
    }

    fn abort_tx(&mut self) {
        self.tx = None;
        self.tx_lead = 0;
        self.keyed = false;
        self.status_dirty = true;
    }

    fn set_config(&mut self, cfg: DigiConfig) {
        // The lock's evidence reaches the receiver, not just the config: it is
        // the decoder that counts the pulses.
        self.rx.set_sync_pulses(cfg.sstv_sync_pulses);
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

    fn set_sstv_mode(&mut self, mode: Option<SstvMode>) {
        match mode {
            Some(m) => {
                self.auto = false;
                self.tx_mode = m;
                self.rx.set_expected(Some(m));
            }
            None => {
                self.auto = true;
                self.tx_mode = SstvMode::Martin1;
                self.rx.set_expected(None);
            }
        }
        self.status_dirty = true;
    }

    /// Abandon the picture being received and listen for the next header.
    ///
    /// The partial image is dropped rather than emitted: half a picture in the
    /// wrong mode is not a picture, and filing one would put it in the gallery
    /// beside the real ones. `image_id` still moves on, so a scanline of the
    /// abandoned frame that is already in flight to a remote client lands on a
    /// canvas nothing else is using instead of on the next station's picture.
    ///
    /// The operator's mode selection is left alone — this is not a way to get
    /// back to Auto — and so is the callsign heard from the last FSK ID, which
    /// belongs to a station rather than to a frame.
    fn sstv_restart_rx(&mut self) {
        self.rx.restart();
        self.image_id = self.image_id.wrapping_add(1);
        self.rx_image = Vec::new();
        self.rx_w = 0;
        self.rx_h = 0;
        self.rx_active = false;
        self.detected = None;
        self.status_dirty = true;
    }

    fn set_sstv_image(&mut self, mode: SstvMode, rgb: Vec<u8>, w: u16, h: u16) {
        self.tx_mode = mode;
        let tx = SstvTx::new(mode, &rgb, w, h, OUT_RATE, self.cfg.sstv_tx_ppm);
        // The callsign in tones after the picture, for the repeaters and the
        // programs that read one (issue #287). A station that has not set a
        // callsign sends nothing extra — `with_fsk_id` takes an empty string as
        // "no ID" rather than transmitting a header with nothing in it.
        let id = if self.cfg.sstv_fsk_id { self.cfg.my_call.trim() } else { "" };
        self.tx = Some(tx.with_fsk_id(id));
        self.tx_lead = (OUT_RATE * self.cfg.sstv_txdelay_ms as f64 / 1000.0) as usize;
        self.status_dirty = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DigiEngine;

    /// A grey 8×8 picture, which is all the encoder needs to build a plan.
    fn image() -> (Vec<u8>, u16, u16) {
        (vec![128u8; 8 * 8 * 3], 8, 8)
    }

    fn controller(txdelay_ms: u16) -> SstvController {
        let cfg = DigiConfig { sstv_txdelay_ms: txdelay_ms, ..Default::default() };
        SstvController::new(Mode::SstvFm, cfg, 48_000.0)
    }

    /// The calibration header must not go out before the transmitter is really
    /// on the air: a decoder that misses it draws no picture at all, which is
    /// what an OpenWebRX on the far end of an IC-9700 showed (issue #351). So
    /// the configured lead comes out as silence first, to the sample.
    #[test]
    fn the_picture_starts_only_after_the_configured_lead() {
        let mut c = controller(500);
        let (rgb, w, h) = image();
        c.set_sstv_image(SstvMode::Martin1, rgb, w, h);
        let lead = (OUT_RATE * 0.5) as usize;

        // Blocks that do not divide the lead, so a lead rounded up to a whole
        // block would show as extra silence and fail the count below.
        let mut silent = 0usize;
        let mut block = vec![0.0f32; 1024];
        let mut first_tone = None;
        for i in 0..64 {
            block.fill(f32::NAN); // every sample has to be written by the fill
            c.fill_tx_block(&mut block);
            assert!(block.iter().all(|s| s.is_finite()), "block {i} was left half-written");
            match block.iter().position(|&s| s != 0.0) {
                Some(at) => {
                    first_tone = Some(silent + at);
                    break;
                }
                None => silent += block.len(),
            }
        }
        assert_eq!(first_tone, Some(lead), "the header goes out the sample the lead ends");
    }

    /// ...and a station that does not need it — an SDR that keys in
    /// milliseconds — gets the picture immediately, as before.
    #[test]
    fn no_lead_means_the_picture_starts_at_once() {
        let mut c = controller(0);
        let (rgb, w, h) = image();
        c.set_sstv_image(SstvMode::Martin1, rgb, w, h);
        let mut block = vec![0.0f32; 1024];
        c.fill_tx_block(&mut block);
        assert!(block.iter().any(|&s| s != 0.0), "nothing should be waited for");
    }

    /// Aborting mid-lead leaves nothing owed, or the next picture would open
    /// with the dead air the abandoned one never spent.
    #[test]
    fn aborting_during_the_lead_forgets_it() {
        let mut c = controller(500);
        let (rgb, w, h) = image();
        c.set_sstv_image(SstvMode::Martin1, rgb, w, h);
        c.abort_tx();
        assert_eq!(c.tx_lead, 0);
    }

    /// Issue #397: restarting the receiver drops the picture in progress and
    /// says so, and starts a fresh canvas so a scanline of the abandoned frame
    /// still in flight cannot land on the next station's picture.
    #[test]
    fn restarting_the_receiver_drops_the_picture_in_progress() {
        let rate = 48_000.0;
        let mut c = SstvController::new(Mode::Sstv, DigiConfig::default(), rate);
        let (w, h) = SstvMode::ScottieDx.dimensions();
        let rgb = vec![96u8; w as usize * h as usize * 3];
        let mut tx = sdroxide_dsp::SstvTx::new(SstvMode::ScottieDx, &rgb, w, h, rate, 0.0);
        let mut block = vec![0.0f32; 4096];
        while !c.rx_active {
            let n = tx.next_block(&mut block);
            assert!(n > 0, "the transmission ran out before the picture started");
            c.on_rx_audio(&block[..n]);
        }
        assert_eq!(c.detected, Some(SstvMode::ScottieDx));
        let was = c.image_id;

        c.sstv_restart_rx();
        assert!(!c.rx_active, "it is still receiving");
        assert_eq!(c.detected, None, "the panel would still say a mode was locked");
        assert!(c.rx_image.is_empty(), "the half-picture was kept");
        assert_ne!(c.image_id, was, "the next picture would be painted onto this one");
        let st = c.sstv_status();
        assert!(!st.rx_active);
        assert_eq!(st.progress, 0.0);

        // Transmit is untouched: this is the receiver's button, and an operator
        // who presses it mid-over must not lose the picture going out.
        let (rgb, w, h) = image();
        c.set_sstv_image(SstvMode::Martin1, rgb, w, h);
        c.sstv_restart_rx();
        assert!(c.tx.is_some(), "the transmission was aborted by a receive control");
    }

    /// **The setting reaches the receiver.** It is the decoder that counts the
    /// pulses, so a config change that stayed in the config would be a control
    /// that silently does nothing.
    #[test]
    fn the_sync_pulse_setting_reaches_the_receiver() {
        let mut c = SstvController::new(Mode::Sstv, DigiConfig::default(), 48_000.0);
        assert_eq!(c.rx.sync_pulses(), 6, "six by default");
        let mut cfg = DigiConfig::default();
        cfg.sstv_sync_pulses = 4;
        c.set_config(cfg);
        assert_eq!(c.rx.sync_pulses(), 4);
    }

    /// **Speckle is not a picture, and a picture is.**
    ///
    /// Nine of the fifty-four images on the bench were frames the receiver had
    /// locked onto noise: rows with nothing to do with each other (2026-11-04).
    /// The row agreement is what tells them apart without looking at content:
    /// a gradient that repeats down the frame scores high, speckle scores near
    /// zero.
    #[test]
    fn row_agreement_separates_speckle_from_a_picture() {
        let (w, h) = (64usize, 48usize);
        // A picture: the same column pattern on every row, so each row agrees
        // with the next one exactly.
        let mut picture = vec![0u8; w * h * 3];
        for y in 0..h {
            for x in 0..w {
                let v = ((x * 255) / w) as u8;
                let i = (y * w + x) * 3;
                picture[i] = v;
                picture[i + 1] = v;
                picture[i + 2] = v;
            }
        }
        // Speckle: a deterministic hash per pixel, unrelated across rows.
        let mut noise = vec![0u8; w * h * 3];
        let mut seed = 0x2545_F491_4F6C_DD1Du64;
        for b in noise.iter_mut() {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            *b = (seed >> 40) as u8;
        }
        let picture_rows = row_agreement(&picture, w, h);
        let noise_rows = row_agreement(&noise, w, h);
        assert!(picture_rows > 0.9, "a picture should agree with itself, got {picture_rows}");
        assert!(noise_rows < 0.2, "speckle should not agree, got {noise_rows}");
        assert!(keep_frame(picture_rows), "a picture is kept on its rows");
        assert!(!keep_frame(noise_rows), "speckle is not kept");
    }

    /// **The rows decide, and the bench's own numbers are the test.**
    ///
    /// The sync share was the second sign and the bench retired it in one
    /// evening: four frames kept between 17:53 and 18:28 had rows of 0.03–0.10
    /// — speckle with a sliver of picture at the top — and the share alone had
    /// let them through. So a share no longer keeps anything, however high.
    #[test]
    fn the_rows_decide_and_a_sync_share_no_longer_keeps_anything() {
        // The evening's pictures, by their rows:
        assert!(keep_frame(0.64), "a real picture");
        assert!(keep_frame(0.36), "a weak one, still a picture");
        // ...and the four that were speckle:
        for rows in [0.10, 0.09, 0.03, 0.03] {
            assert!(!keep_frame(rows), "rows {rows} is not a picture");
        }
        assert!(!keep_frame(0.0), "an empty frame is not a picture");
        // A flat frame scores near zero too, which is what SAVE ALL is for.
        assert!(!keep_frame(0.05), "a flat frame is dropped unless SAVE ALL is on");
    }
}
