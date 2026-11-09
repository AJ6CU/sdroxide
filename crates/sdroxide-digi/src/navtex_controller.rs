//! `NavtexController` — the maritime safety broadcast, receive only.
//!
//! The decoder is [`sdroxide_dsp::NavtexRx`]; what lives here is everything
//! above the characters: the message framing, the duplicate suppression that
//! makes a station's four-hourly repeat cycle readable, and the status the
//! panel draws.
//!
//! # Framing
//!
//! A NAVTEX message is `ZCZC B1B2B3B4`, a body, and `NNNN`. The four header
//! characters are the transmitter (`B1`), the subject (`B2`) and a serial
//! number (`B3B4`) — and they are the whole of the addressing the service has,
//! so they are parsed out rather than left in the body. A station repeats each
//! message on its next slot, four hours later; a receiver that printed both
//! would fill the screen with yesterday, so a header already held replaces the
//! copy that is there rather than adding to it — **unless** the new copy is
//! better, which is the point of listening to a repeat at all.
//!
//! # Why there is no transmitter
//!
//! Not a limitation. NAVTEX is a coast station's service on a frequency
//! adjacent to distress traffic, and an amateur station putting characters on
//! it would be transmitting safety information nobody may act on. See
//! [`Mode::is_rx_only`].

use std::time::SystemTime;

use sdroxide_dsp::NavtexRx;
use sdroxide_types::{
    DigiConfig, DigiStatus, Mode, NavtexMessage, NavtexStatus, QsoStep, TranscriptLine,
    NAVTEX_MESSAGE_MAX,
};

use crate::controller::DigiAction;
use crate::DigiEngine;

/// Characters of loose text kept — what was decoded outside any message.
const TEXT_CAP: usize = 8_000;
/// The longest a message may run before it is closed without its `NNNN`.
///
/// A NAVTEX slot is ten minutes at a hundred baud, so a message that has
/// reached this has not ended: the closing sequence was lost, or the signal
/// was. Either way the text so far is worth keeping and the next `ZCZC` must
/// not append to it.
const BODY_CAP: usize = 20_000;

/// Whether `text` ends with `mark` — `ZCZC` or `NNNN` — allowing one of the
/// four characters to be a lost one (`*`).
///
/// The two sequences are the whole of the framing, and a single character
/// lost in either used to cost the message: an opener with a hole in it left
/// the broadcast in the loose text, never filed, and a closer with one ran
/// the message on into the next. Only a *lost* character is forgiven, not a
/// wrong one: `Z*ZC` is an opener nothing else could have been, `ZAZC` is
/// not.
fn ends_with_mark(text: &str, mark: &str) -> bool {
    let mut tail = text.chars().rev();
    let mut holes = 0;
    for want in mark.chars().rev() {
        match tail.next() {
            Some(c) if c == want => {}
            Some('*') => holes += 1,
            _ => return false,
        }
    }
    holes <= 1
}

pub struct NavtexController {
    cfg: DigiConfig,
    rx: NavtexRx,
    tap_rate: f64,
    /// Text decoded but not yet consumed by the framer.
    pending: String,
    /// The message being received, if a `ZCZC` has been seen.
    live: Option<NavtexMessage>,
    messages: Vec<NavtexMessage>,
    text: String,
    status_dirty: bool,
    last_status: Option<SystemTime>,
    /// Where to persist the received messages, if anywhere. `None` for
    /// controllers created without a config directory (tests).
    config_dir: Option<std::path::PathBuf>,
}

impl NavtexController {
    pub fn new(cfg: DigiConfig, tap_rate: f64) -> Self {
        Self::new_with_config_dir(cfg, tap_rate, None)
    }

    pub fn new_with_config_dir(
        cfg: DigiConfig,
        tap_rate: f64,
        config_dir: Option<std::path::PathBuf>,
    ) -> Self {
        let mut rx = NavtexRx::new(tap_rate);
        rx.set_reverse(cfg.navtex_reverse);
        let mut ctrl = NavtexController {
            cfg,
            rx,
            tap_rate,
            pending: String::new(),
            live: None,
            messages: Vec::new(),
            text: String::new(),
            status_dirty: true,
            last_status: None,
            config_dir,
        };
        ctrl.load_messages();
        ctrl
    }

    /// Load persisted messages from disk, if any.
    fn load_messages(&mut self) {
        let Some(dir) = &self.config_dir else {
            return;
        };
        let path = dir.join("navtex_messages.json");
        if let Ok(data) = std::fs::read_to_string(&path) {
            if let Ok(msgs) = serde_json::from_str::<Vec<NavtexMessage>>(&data) {
                self.messages = msgs;
                self.status_dirty = true;
            }
        }
    }

    /// Persist the current messages to disk.
    fn save_messages(&self) {
        let Some(dir) = &self.config_dir else {
            return;
        };
        let path = dir.join("navtex_messages.json");
        if let Ok(data) = serde_json::to_string_pretty(&self.messages) {
            let _ = std::fs::write(&path, data);
        }
    }

    fn now_unix() -> i64 {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0)
    }

    /// Take whatever complete framing is in [`Self::pending`].
    ///
    /// Character at a time rather than by searching the buffer, because a
    /// message arrives over ten minutes and the `ZCZC` that opens it may be
    /// split across any two calls. Once opened, everything goes into the body
    /// until `NNNN` closes it — including the line breaks, which is how a
    /// warning keeps its shape.
    ///
    /// A `ZCZC` that arrives while a message is already open means the
    /// previous closing sequence was lost: the open message is filed as it
    /// stands and a new one is opened, so the next broadcast is not appended
    /// to the end of the one before it.
    fn frame(&mut self) {
        let taken = std::mem::take(&mut self.pending);
        for ch in taken.chars() {
            self.frame_one(ch);
        }
    }

    fn frame_one(&mut self, ch: char) {
        if self.live.is_none() {
            self.text.push(ch);
            if self.text.len() > TEXT_CAP {
                let cut = self.text.len() - TEXT_CAP;
                let cut = (cut..self.text.len())
                    .find(|&i| self.text.is_char_boundary(i))
                    .unwrap_or(self.text.len());
                self.text.drain(..cut);
            }
            if ends_with_mark(&self.text, "ZCZC") {
                self.live = Some(NavtexMessage {
                    station: '?',
                    kind: '?',
                    serial: 0,
                    text: String::new(),
                    at: Self::now_unix(),
                    complete: false,
                    lost: 0,
                });
                self.status_dirty = true;
            }
            return;
        }

        let mut msg = self.live.take().expect("live checked above");
        if ch == '*' {
            msg.lost += 1;
        }
        msg.text.push(ch);
        // The four header characters follow `ZCZC` and a space, and are
        // parsed as soon as they have all arrived. A station that sends no
        // space still lands them here: the parse skips leading whitespace of
        // its own.
        if msg.station == '?' {
            let head: Vec<char> = msg.text.chars().filter(|c| !c.is_whitespace()).collect();
            if head.len() >= 4 {
                msg.station = head[0];
                msg.kind = head[1];
                msg.serial = head[2..4].iter().collect::<String>().parse::<u8>().unwrap_or(0);
                // The header is addressing, not text: drop it out of the body
                // so the panel's first line is the warning rather than four
                // letters of routing.
                msg.text.clear();
                self.status_dirty = true;
            }
        }

        let do_close;
        let reopen;
        // Both marks are four one-byte characters, with or without a hole.
        if msg.station != '?' && ends_with_mark(&msg.text, "NNNN") {
            let holes = msg.text[msg.text.len() - 4..].matches('*').count() as u32;
            msg.lost = msg.lost.saturating_sub(holes);
            msg.text.truncate(msg.text.len() - 4);
            msg.complete = true;
            do_close = true;
            reopen = false;
        } else if msg.text.len() > BODY_CAP {
            do_close = true;
            reopen = false;
        } else if ends_with_mark(&msg.text, "ZCZC") && msg.text.len() > 4 {
            // A second opener inside an open message: the previous closing
            // sequence was lost. Drop it out of the body and reopen.
            let holes = msg.text[msg.text.len() - 4..].matches('*').count() as u32;
            msg.lost = msg.lost.saturating_sub(holes);
            msg.text.truncate(msg.text.len() - 4);
            do_close = true;
            reopen = true;
        } else {
            do_close = false;
            reopen = false;
        }

        self.live = Some(msg);
        if do_close {
            self.close();
        }
        if reopen {
            self.live = Some(NavtexMessage {
                station: '?',
                kind: '?',
                serial: 0,
                text: String::new(),
                at: Self::now_unix(),
                complete: false,
                lost: 0,
            });
            self.status_dirty = true;
        }
    }

    /// File the message being received.
    ///
    /// A repeat of one already held replaces it when it is *better* — fewer
    /// characters lost, or the first copy that reached its `NNNN`. That is the
    /// whole reason a station repeats: the second pass through a fade fills in
    /// what the first missed, and a receiver that kept only the first, or kept
    /// both, throws that away.
    fn close(&mut self) {
        let Some(msg) = self.live.take() else { return };
        self.status_dirty = true;
        if msg.station == '?' {
            // A `ZCZC` whose header never arrived is not a message; the text is
            // in the loose stream already.
            return;
        }
        let key = (msg.station, msg.kind, msg.serial);
        if let Some(held) = self.messages.iter_mut().find(|m| (m.station, m.kind, m.serial) == key)
        {
            let better = (msg.complete && !held.complete)
                || (msg.complete == held.complete && msg.lost < held.lost);
            if better {
                let at = held.at;
                *held = msg;
                // The time it was *first* heard, which is what a reader is
                // looking for when a warning is timestamped.
                held.at = at;
            }
            return;
        }
        self.messages.push(msg);
        while self.messages.len() > NAVTEX_MESSAGE_MAX {
            self.messages.remove(0);
        }
        self.save_messages();
    }

    fn navtex_status(&self) -> NavtexStatus {
        let (direct, repaired, lost) = self.rx.counts();
        NavtexStatus {
            in_sync: self.rx.in_sync(),
            level: self.rx.magnitude(),
            messages: self.messages.clone(),
            live: self.live.clone(),
            text: self.text.clone(),
            direct,
            repaired,
            lost,
            reverse: self.rx.reverse(),
        }
    }

    fn digi_status(&self) -> DigiStatus {
        DigiStatus {
            stanag: None,
            mode: Mode::Navtex,
            step: QsoStep::Idle,
            dx_call: None,
            dx_grid: None,
            tx_next: false,
            tx_pending_msg: None,
            audio_hz: sdroxide_types::NAVTEX_TONE_HZ,
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
            navtex: Some(self.navtex_status()),
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

impl DigiEngine for NavtexController {
    fn mode(&self) -> Mode {
        Mode::Navtex
    }

    fn on_rx_audio(&mut self, tap: &[f32]) {
        let text = self.rx.process(tap);
        if !text.is_empty() {
            self.pending.push_str(&text);
            self.frame();
            self.status_dirty = true;
        }
    }

    fn poll(&mut self, _now: SystemTime, _dial_hz: f64) -> Vec<DigiAction> {
        let now = SystemTime::now();
        // On change, and a few times a second regardless: the sync flag and
        // the level are what an operator watches while tuning, and neither
        // changes the text.
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

    /// Receive only — every transmit hook is a no-op rather than a refusal,
    /// so nothing upstream has to ask first.
    fn tx_burst_active(&self) -> bool {
        false
    }

    fn fill_tx_block(&mut self, _out: &mut [f32]) -> bool {
        false
    }

    fn on_burst_done(&mut self) {}

    /// Throw away the message in progress and the loose text — the page, not
    /// the file of messages already received.
    fn abort(&mut self) {
        self.live = None;
        self.pending.clear();
        self.text.clear();
        self.status_dirty = true;
    }

    fn clear_rx(&mut self) {
        self.messages.clear();
        self.live = None;
        self.text.clear();
        self.pending.clear();
        // Restart the decoder from scratch: drop any latched sync so a stale
        // phase cannot keep producing garbage on a quiet channel.
        self.rx = NavtexRx::new(self.tap_rate);
        self.rx.set_reverse(self.cfg.navtex_reverse);
        self.status_dirty = true;
        self.save_messages();
    }

    fn abort_tx(&mut self) {}

    fn set_audio_hz(&mut self, _hz: f32) {}

    /// Fixed by the standard: the tones are 1615 and 1785 Hz, and there is
    /// nothing here for an operator to tune.
    fn audio_hz(&self) -> f32 {
        sdroxide_types::NAVTEX_TONE_HZ
    }

    fn status(&self) -> DigiStatus {
        self.digi_status()
    }

    fn set_config(&mut self, cfg: DigiConfig) {
        if cfg.navtex_reverse != self.cfg.navtex_reverse {
            self.rx.set_reverse(cfg.navtex_reverse);
            self.status_dirty = true;
        }
        // `navtex_rx_sensitivity` and `navtex_rx_auto` are still in the
        // config, because taking a field out of it is a wire change, and are
        // read by nothing: the receiver decides for itself whether a signal
        // is there.
        self.cfg = cfg;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdroxide_dsp::navtex_test::{encode_bits, synth};

    /// One transmission, with the silence either side of it that a real slot
    /// has. The gap matters: two broadcasts butted together share no character
    /// phase, and a receiver that stayed latched on the first would read the
    /// front of the second at the wrong offset.
    fn feed(c: &mut NavtexController, text: &str) {
        let mut audio = vec![0.0f32; 8000];
        audio.extend(synth(&encode_bits(text), 8000.0, 1700.0, 0.4));
        audio.extend(std::iter::repeat_n(0.0f32, 8000));
        for chunk in audio.chunks(512) {
            c.on_rx_audio(chunk);
        }
    }

    fn ctrl() -> NavtexController {
        NavtexController::new(DigiConfig::default(), 8000.0)
    }

    #[test]
    fn it_is_a_receive_only_engine() {
        let c = ctrl();
        assert_eq!(c.mode(), Mode::Navtex);
        assert!(Mode::Navtex.is_rx_only());
        assert!(!c.tx_burst_active());
    }

    /// A whole broadcast comes out as one message, with the header taken apart
    /// and the body left as it was sent.
    #[test]
    fn a_broadcast_is_framed_into_a_message() {
        let mut c = ctrl();
        feed(&mut c, "ZCZC FA12 GALE WARNING NORTH SEA NNNN");
        let s = c.navtex_status();
        assert_eq!(
            s.messages.len(),
            1,
            "framed {:?} text={:?} live={:?}",
            s.messages,
            s.text,
            s.live
        );
        let m = &s.messages[0];
        assert_eq!(m.station, 'F');
        assert_eq!(m.kind, 'A');
        assert_eq!(m.serial, 12);
        assert!(m.complete, "the closing NNNN was not seen");
        assert!(m.text.contains("GALE WARNING NORTH SEA"), "body {:?}", m.text);
        assert_eq!(m.kind_label(), "Navigational warning");
        assert!(m.is_mandatory(), "a navigational warning may not be filtered out");
        assert!(s.live.is_none(), "the message was left open");
    }

    /// The four-hourly repeat replaces the copy already held when it is better,
    /// and is dropped when it is not — the whole reason a station repeats.
    #[test]
    fn a_repeat_replaces_a_worse_copy_and_never_doubles_the_list() {
        let mut c = ctrl();
        // First pass: the closing sequence is missed, so the message is filed
        // incomplete when the next one starts.
        feed(&mut c, "ZCZC OB07 FIRST PASS");
        feed(&mut c, "ZCZC OB07 SECOND PASS NNNN");
        let s = c.navtex_status();
        assert_eq!(s.messages.len(), 1, "the repeat was filed as a second message");
        let m = &s.messages[0];
        assert!(m.complete, "the complete copy did not replace the truncated one");
        assert!(m.text.contains("SECOND PASS"), "body {:?}", m.text);

        // …and a third, worse copy does not undo that.
        feed(&mut c, "ZCZC OB07 THIRD");
        let s = c.navtex_status();
        assert_eq!(s.messages.len(), 1);
        assert!(s.messages[0].text.contains("SECOND PASS"), "a worse copy replaced a better one");
    }

    /// The whole path, mistuned: the audio arrives a couple of hundred hertz
    /// from where the decoder expects the tones, and a whole broadcast is still
    /// framed out of it. Issue #608 as reported — a strong steady signal
    /// decoding to asterisks while fldigi read it — so this drives the
    /// controller and the framer, not just the receiver.
    #[test]
    fn a_mistuned_broadcast_is_still_framed_as_a_message() {
        let mut c = ctrl();
        let mut audio = vec![0.0f32; 8000];
        audio.extend(synth(
            &encode_bits("ZCZC FA12 GALE WARNING NORTH SEA NNNN"),
            8000.0,
            // 1950 Hz — the 516.6 kHz dial against an expected 516.3 kHz.
            1950.0,
            0.4,
        ));
        audio.extend(std::iter::repeat_n(0.0f32, 8000));
        for chunk in audio.chunks(512) {
            c.on_rx_audio(chunk);
        }
        let s = c.navtex_status();
        assert_eq!(
            s.messages.len(),
            1,
            "a 250 Hz mistune filed no message: text={:?} live={:?}",
            s.text,
            s.live
        );
        let m = &s.messages[0];
        assert_eq!(m.station, 'F');
        assert_eq!(m.serial, 12);
        assert!(m.complete, "the closing sequence was lost: {:?}", m.text);
        assert!(m.text.contains("GALE WARNING NORTH SEA"), "body {:?}", m.text);
    }

    /// The time a message states survives framing and is readable from the
    /// filed message — the panel shows it beside the station header
    /// (issue #212).
    #[test]
    fn the_time_in_a_body_survives_framing() {
        let mut c = ctrl();
        feed(&mut c, "ZCZC FA12 GALE WARNING NORTH SEA AT 1200 UTC NNNN");
        let s = c.navtex_status();
        let m = &s.messages[0];
        assert!(m.text.contains("AT 1200 UTC"), "body {:?}", m.text);
        assert_eq!(m.body_time_utc(), Some((12, 0)));

        // A message with no time of day in it reports none, rather than a
        // zero that would read as midnight.
        feed(&mut c, "ZCZC PA01 NAVAREA ONE NNNN");
        let s = c.navtex_status();
        let m = s.messages.iter().find(|m| m.serial == 1).expect("second message");
        assert_eq!(m.body_time_utc(), None);
    }

    /// Two different messages from the same transmitter are two messages: the
    /// serial number is part of what identifies one.
    #[test]
    fn different_serials_are_different_messages() {
        let mut c = ctrl();
        feed(&mut c, "ZCZC PA01 ONE NNNN");
        feed(&mut c, "ZCZC PA02 TWO NNNN");
        let s = c.navtex_status();
        assert_eq!(s.messages.len(), 2, "text={:?} live={:?}", s.text, s.live);
        assert_eq!(s.messages[0].serial, 1, "{:?}", s.messages);
        assert_eq!(s.messages[1].serial, 2, "{:?}", s.messages);
    }

    /// One character lost in the opening or the closing sequence does not cost
    /// the message. Both used to: `Z*ZC` left the broadcast unfiled in the
    /// loose text, and `NN*N` ran it on into whatever came next.
    #[test]
    fn a_hole_in_the_opening_or_closing_sequence_still_frames_the_message() {
        let mut c = ctrl();
        c.pending.push_str("Z*ZC FA12\r\nGALE WARNING\r\nNN*N\r\n");
        c.frame();
        let s = c.navtex_status();
        assert_eq!(s.messages.len(), 1, "text={:?} live={:?}", s.text, s.live);
        let m = &s.messages[0];
        assert_eq!((m.station, m.kind, m.serial), ('F', 'A', 12));
        assert!(m.complete, "the closing sequence with a hole was not seen");
        assert_eq!(m.text.trim(), "GALE WARNING");
        assert_eq!(m.lost, 0, "the hole in the closing sequence was counted against the body");
        assert!(s.live.is_none());

        // A wrong character is not a lost one: this is not an opener.
        let mut c = ctrl();
        c.pending.push_str("ZAZC FA12 GALE WARNING NNNN");
        c.frame();
        assert!(c.navtex_status().messages.is_empty());
        assert!(!ends_with_mark("Z**C", "ZCZC"), "two holes is not a mark");
    }

    /// A broadcast whose closing `NNNN` was lost must not swallow the next one:
    /// the next `ZCZC` closes it and starts a new message. (#481)
    #[test]
    fn a_lost_closing_sequence_does_not_merge_two_broadcasts() {
        let mut c = ctrl();
        feed(&mut c, "ZCZC FA12 FIRST WITHOUT CLOSING");
        feed(&mut c, "ZCZC PA01 SECOND NNNN");
        let s = c.navtex_status();
        assert_eq!(
            s.messages.len(),
            2,
            "two broadcasts merged into one: {:?} live={:?} text={:?}",
            s.messages,
            s.live,
            s.text
        );
        assert!(
            s.messages.iter().any(|m| m.station == 'P' && m.serial == 1 && m.complete),
            "second broadcast not filed complete: {:?}",
            s.messages
        );
    }
}
