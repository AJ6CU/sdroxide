//! `AcarsController` — the VHF aircraft datalink, receive only.
//!
//! The decoder is [`sdroxide_dsp::AcarsRx`]; what lives here is the rolling
//! message list and the status the panel draws. ACARS frames are short and
//! self-contained (there is no multi-slot framing to reassemble above the
//! decoder), so this is thin: stamp each frame, keep the newest few hundred,
//! and report the meter.
//!
//! # Why there is no transmitter
//!
//! Not a limitation. ACARS is an airline service on airband channels shared
//! with air traffic control, and nothing about it is an amateur emission. See
//! [`Mode::is_rx_only`].

use std::time::SystemTime;

use sdroxide_dsp::{ACARS_CENTER_HZ, AcarsEvent, AcarsRx};
use sdroxide_types::{
    ACARS_MESSAGE_MAX, AcarsMessage, AcarsStatus, DigiConfig, DigiStatus, Mode, QsoStep,
    TranscriptLine,
};

use crate::DigiEngine;
use crate::controller::DigiAction;

pub struct AcarsController {
    cfg: DigiConfig,
    rx: AcarsRx,
    messages: Vec<AcarsMessage>,
    status_dirty: bool,
    last_status: Option<SystemTime>,
}

impl AcarsController {
    pub fn new(cfg: DigiConfig, tap_rate: f64) -> Self {
        AcarsController {
            cfg,
            rx: AcarsRx::new(tap_rate),
            messages: Vec::new(),
            status_dirty: true,
            last_status: None,
        }
    }

    fn now_unix() -> i64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    }

    /// File one message. The log is oldest first and keeps the newest
    /// [`ACARS_MESSAGE_MAX`]; the panel lists it newest first.
    fn record(&mut self, msg: AcarsMessage) {
        self.messages.push(msg);
        if self.messages.len() > ACARS_MESSAGE_MAX {
            let cut = self.messages.len() - ACARS_MESSAGE_MAX;
            self.messages.drain(..cut);
        }
        self.status_dirty = true;
    }

    fn acars_status(&self) -> AcarsStatus {
        AcarsStatus {
            level: self.rx.level(),
            messages: self.messages.clone(),
            frames: self.rx.frames(),
            bad: self.rx.bad(),
        }
    }

    fn digi_status(&self) -> DigiStatus {
        DigiStatus {
            stanag: None,
            mode: Mode::Acars,
            step: QsoStep::Idle,
            dx_call: None,
            dx_grid: None,
            tx_next: false,
            tx_pending_msg: None,
            audio_hz: ACARS_CENTER_HZ as f32,
            tx_even: false,
            transmitting: false,
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
            acars: Some(self.acars_status()),
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

impl DigiEngine for AcarsController {
    fn mode(&self) -> Mode {
        Mode::Acars
    }

    fn on_rx_audio(&mut self, tap: &[f32]) {
        let mut events = Vec::new();
        self.rx.process(tap, &mut events);
        for e in events {
            match e {
                AcarsEvent::Message(f) => {
                    let msg = AcarsMessage {
                        mode: f.mode,
                        address: f.address,
                        ack: f.ack,
                        label: f.label,
                        block_id: f.block_id,
                        text: f.text,
                        crc_ok: f.crc_ok,
                        at: Self::now_unix(),
                    };
                    self.record(msg);
                }
            }
        }
    }

    fn poll(&mut self, _now: SystemTime, _dial_hz: f64) -> Vec<DigiAction> {
        let now = SystemTime::now();
        // On change, and a few times a second regardless: the meter is what an
        // operator watches while tuning, and it changes without the text.
        let due = self
            .last_status
            .map(|t| now.duration_since(t).map(|d| d.as_secs_f32() > 0.25).unwrap_or(true))
            .unwrap_or(true);
        if !self.status_dirty && !due {
            return Vec::new();
        }
        self.status_dirty = false;
        self.last_status = Some(now);
        vec![DigiAction::Status(self.digi_status())]
    }

    fn tx_burst_active(&self) -> bool {
        false
    }

    fn fill_tx_block(&mut self, _out: &mut [f32]) -> bool {
        false
    }

    fn on_burst_done(&mut self) {}

    fn abort(&mut self) {
        self.status_dirty = true;
    }

    fn abort_tx(&mut self) {}

    fn set_config(&mut self, cfg: DigiConfig) {
        self.cfg = cfg;
    }

    fn clear_rx(&mut self) {
        self.messages.clear();
        self.status_dirty = true;
    }

    fn set_audio_hz(&mut self, _hz: f32) {}

    /// Fixed by the standard: the MSK sits on 1800 Hz, and there is nothing for
    /// an operator to tune.
    fn audio_hz(&self) -> f32 {
        ACARS_CENTER_HZ as f32
    }

    fn status(&self) -> DigiStatus {
        self.digi_status()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn message(n: i64) -> AcarsMessage {
        AcarsMessage {
            mode: "2".into(),
            address: format!("N{n}"),
            ack: "\x15".into(),
            label: "H1".into(),
            block_id: "1".into(),
            text: format!("message {n}"),
            crc_ok: true,
            at: n,
        }
    }

    /// 201 messages leave 200: the newest is kept, the oldest is the one lost.
    #[test]
    fn the_log_keeps_two_hundred_and_drops_the_oldest() {
        let mut c = AcarsController::new(DigiConfig::default(), 48_000.0);
        for n in 1..=201 {
            c.record(message(n));
        }
        let st = c.acars_status();
        assert_eq!(st.messages.len(), 200);
        assert_eq!(st.messages.first().map(|m| m.at), Some(2), "message 1 was dropped");
        assert_eq!(st.messages.last().map(|m| m.at), Some(201), "the newest is kept");
    }
}
