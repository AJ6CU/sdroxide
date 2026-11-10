//! Voice-keyer safety rails, driven through the real engine.
//!
//! The keyer is the one feature that can key the transmitter from a single
//! keypress, so what it *refuses* to do matters more than what it does. These
//! tests use a transmit-capable mock source and watch the engine's own state
//! events, so they hold regardless of what the operator has recorded (the
//! recordings themselves are covered by the unit tests in `voice.rs`).

use std::time::Duration;

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DeviceCaps, Mode, RadioEvent, RxId};

struct MockSource {
    center: f64,
    rate: f64,
}

impl IqSource for MockSource {
    fn sample_rate(&self) -> f64 {
        self.rate
    }
    fn center_hz(&self) -> f64 {
        self.center
    }
    fn set_center_hz(&mut self, hz: f64) -> Result<()> {
        self.center = hz;
        Ok(())
    }
    fn read(&mut self, buf: &mut [Complex32]) -> Result<usize> {
        std::thread::sleep(Duration::from_millis(5));
        let n = buf.len().min(2048);
        buf[..n].fill(Complex32::new(0.0, 0.0));
        Ok(n)
    }
    fn describe(&self) -> String {
        "mock tx source".into()
    }
    fn tx_begin(&mut self, _center_hz: f64, rate: f64) -> Result<f64> {
        Ok(rate)
    }
    fn tx_write(&mut self, _samples: &[Complex32]) -> Result<()> {
        Ok(())
    }
    fn tx_write_audio(&mut self, _audio: &[f32]) -> Result<()> {
        Ok(())
    }
    fn tx_end(&mut self) -> Result<()> {
        Ok(())
    }
}

fn tx_caps(rate: f64) -> DeviceCaps {
    DeviceCaps {
        driver: "mock".into(),
        label: "mock".into(),
        rx_channels: 1,
        tx_channels: 1,
        sample_rates: vec![rate],
        freq_ranges_rx: vec![(0.0, 1_000_000_000.0)],
        freq_ranges_tx: vec![(0.0, 1_000_000_000.0)],
        ..DeviceCaps::default()
    }
}

/// Run `cmds` against the engine and report whether it ever announced PTT, and
/// the last voice-keyer status it published.
fn run(cmds: &[Command], settle: Duration) -> (bool, Option<sdroxide_types::VoiceStatus>) {
    let rate = 2_400_000.0;
    let src = MockSource { center: 14_200_000.0, rate };
    let cfg = EngineConfig { tx_bands_open: true, tx_ham_only: false, ..Default::default() };
    let mut h = start_engine(Box::new(src), tx_caps(rate), cfg);
    let thread = h.thread.take();

    std::thread::sleep(Duration::from_millis(150));
    for c in cmds {
        h.cmd_tx.send(c.clone()).unwrap();
    }

    let deadline = std::time::Instant::now() + settle;
    let mut keyed = false;
    let mut voice = None;
    while std::time::Instant::now() < deadline {
        while let Ok(ev) = h.event_rx.try_recv() {
            match ev {
                RadioEvent::State(s) => keyed |= s.tx.ptt,
                RadioEvent::VoiceStatus(v) => voice = Some(v),
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }

    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    (keyed, voice)
}

/// The digital modes synthesise their own transmit audio, so a bound numpad key
/// pressed by accident in FT8 must not key up — whatever is in the slot.
#[test]
fn a_voice_message_never_keys_up_in_a_digital_mode() {
    let cmds = [Command::SetMode { rx: RxId::Main, mode: Mode::Ft8 }, Command::VoicePlay(Some(0))];
    let (keyed, _) = run(&cmds, Duration::from_millis(700));
    assert!(!keyed, "the voice keyer must be refused in FT8");
}

/// TUNE holds the transmitter at the tune level, where a message would go
/// nowhere. Asking for one is refused rather than half-transmitted.
#[test]
fn a_voice_message_is_refused_while_tuning() {
    let cmds = [Command::SetTune(true), Command::VoicePlay(Some(0))];
    let (keyed, _) = run(&cmds, Duration::from_millis(700));
    assert!(!keyed, "PTT must not be asserted on top of TUNE");
}

/// Recording reads the same microphone the transmitter does, so it cannot start
/// mid-over — and the engine says so through the status it publishes.
#[test]
fn recording_is_refused_while_transmitting() {
    let cmds = [Command::SetPtt(true), Command::VoiceRecord(Some(0))];
    let (keyed, voice) = run(&cmds, Duration::from_millis(700));
    assert!(keyed, "PTT should have been asserted");
    let voice = voice.expect("the engine announces the keyer's slots at startup");
    assert_eq!(voice.recording, None, "recording must not start while keyed");
    assert_eq!(voice.slots.len(), sdroxide_types::VOICE_SLOTS);
}
