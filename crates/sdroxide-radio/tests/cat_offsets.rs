//! RIT / XIT / split on a CAT rig.
//!
//! A CAT radio has no DDC of its own, so none of the three can be a software
//! offset the way they are on an SDR: the engine has to hand the front end a
//! receive frequency that already carries RIT, and a transmit frequency that
//! already carries XIT and split. These tests drive the engine with a mock
//! audio-mode source that records what it was asked for.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DeviceCaps, RadioEvent, Vfo};

/// What the front end was told to do with its dial.
#[derive(Default)]
struct Log {
    /// The VFO, as `set_center_hz` received it.
    vfo: f64,
    /// The RIT offset, as `set_rit_hz` received it.
    rit: f64,
    /// Transmit frequencies handed to `tx_begin`, in order.
    tx: Vec<f64>,
}

impl Log {
    /// Where the rig's dial ends up while receiving.
    fn rx_dial(&self) -> f64 {
        self.vfo + self.rit
    }
}

/// A stand-in for a CAT rig on a sound card: demodulated audio in, PTT and dial
/// over "CAT" (here, into `log`).
struct MockCat {
    log: Arc<Mutex<Log>>,
    rate: f64,
}

impl IqSource for MockCat {
    fn sample_rate(&self) -> f64 {
        self.rate
    }
    fn center_hz(&self) -> f64 {
        self.log.lock().unwrap().vfo
    }
    fn set_center_hz(&mut self, hz: f64) -> Result<()> {
        self.log.lock().unwrap().vfo = hz;
        Ok(())
    }
    fn set_rit_hz(&mut self, hz: f64) {
        self.log.lock().unwrap().rit = hz;
    }
    fn display_bandwidth(&self) -> Option<f64> {
        Some(3_000.0)
    }
    fn read(&mut self, buf: &mut [Complex32]) -> Result<usize> {
        std::thread::sleep(Duration::from_millis(5));
        let n = buf.len().min(1024);
        buf[..n].fill(Complex32::new(0.0, 0.0));
        Ok(n)
    }
    fn describe(&self) -> String {
        "mock CAT rig".into()
    }
    fn tx_begin(&mut self, center_hz: f64, rate: f64) -> Result<f64> {
        self.log.lock().unwrap().tx.push(center_hz);
        Ok(rate)
    }
    fn tx_write_audio(&mut self, _audio: &[f32]) -> Result<()> {
        Ok(())
    }
    fn tx_end(&mut self) -> Result<()> {
        Ok(())
    }
}

fn cat_caps() -> DeviceCaps {
    DeviceCaps {
        driver: "mock-cat".into(),
        label: "mock CAT".into(),
        rx_channels: 1,
        tx_channels: 1,
        audio_mode: true,
        freq_ranges_rx: vec![(100_000.0, 148_000_000.0)],
        freq_ranges_tx: vec![(1_800_000.0, 54_000_000.0)],
        ..DeviceCaps::default()
    }
}

/// Run `cmds` against the engine and return what the front end saw, together
/// with the last published state.
fn run(cmds: &[Command]) -> (Log, sdroxide_types::RadioState) {
    let log = Arc::new(Mutex::new(Log::default()));
    let src = MockCat { log: Arc::clone(&log), rate: 48_000.0 };
    let cfg = EngineConfig { tx_bands_open: true, tx_ham_only: false, ..Default::default() };
    let mut h = start_engine(Box::new(src), cat_caps(), cfg);
    let thread = h.thread.take();

    for c in cmds {
        h.cmd_tx.send(c.clone()).unwrap();
    }
    // Let the engine drain the queue and settle (TX begins on the loop's next
    // pass, after the command that asked for it).
    let mut state = None;
    let deadline = Instant::now() + Duration::from_secs(2);
    while Instant::now() < deadline {
        while let Ok(ev) = h.event_rx.try_recv() {
            if let RadioEvent::State(s) = ev {
                state = Some(s);
            }
        }
        std::thread::sleep(Duration::from_millis(20));
        if state.is_some() && !log.lock().unwrap().tx.is_empty() {
            break;
        }
    }

    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    let out = std::mem::take(&mut *log.lock().unwrap());
    (out, state.expect("the engine should publish state"))
}

/// The dial the rig ends up on has to be the frequency we are actually
/// listening to, because on a CAT rig that is the only thing that decides it.
#[test]
fn rit_moves_the_receive_dial_without_moving_the_vfo() {
    let (log, state) = run(&[
        Command::SetVfo { vfo: Vfo::A, hz: 14_074_000.0 },
        Command::SetRit { enabled: true, hz: 700 },
    ]);
    assert_eq!(log.vfo, 14_074_000.0, "the VFO itself must not move");
    assert_eq!(log.rit, 700.0);
    assert_eq!(log.rx_dial(), 14_074_700.0);
    assert_eq!(state.vfo_a_hz, 14_074_000.0);
    // The panadapter axis has to describe the audio the rig is really sending,
    // which comes from the dial — not from the VFO the offset was measured off.
    // A demod-audio window is centred on the dial: see
    // `Engine::update_display_center`.
    assert_eq!(state.center_hz, 14_074_700.0, "the window is centred on the dial");
}

/// RIT switched off puts the dial back, rather than leaving the last offset
/// silently applied.
#[test]
fn clearing_rit_returns_the_dial_to_the_vfo() {
    let (log, _) = run(&[
        Command::SetVfo { vfo: Vfo::A, hz: 14_074_000.0 },
        Command::SetRit { enabled: true, hz: 700 },
        Command::SetRit { enabled: false, hz: 700 },
    ]);
    assert_eq!(log.rit, 0.0);
    assert_eq!(log.rx_dial(), 14_074_000.0);
}

/// XIT has to reach the rig as a transmit frequency: there is no transmit DDC
/// on a CAT rig to shift instead.
#[test]
fn xit_offsets_the_transmit_frequency() {
    let (log, _) = run(&[
        Command::SetVfo { vfo: Vfo::A, hz: 14_074_000.0 },
        Command::SetXit { enabled: true, hz: -1_200 },
        Command::SetPtt(true),
    ]);
    assert_eq!(log.tx, vec![14_072_800.0], "transmit goes out at dial + XIT");
}

/// Split transmits on the *other* VFO — including when RIT has moved the
/// receive dial somewhere else again.
#[test]
fn split_transmits_on_the_inactive_vfo() {
    let (log, _) = run(&[
        Command::SetVfo { vfo: Vfo::A, hz: 14_074_000.0 },
        Command::SetVfo { vfo: Vfo::B, hz: 14_200_000.0 },
        Command::SetRit { enabled: true, hz: 500 },
        Command::SetSplit(true),
        Command::SetPtt(true),
    ]);
    assert_eq!(log.rx_dial(), 14_074_500.0, "receive still on VFO A + RIT");
    assert_eq!(log.tx, vec![14_200_000.0], "transmit on VFO B");
}

/// Split and XIT stack, the same way they do on a rig's own controls.
#[test]
fn split_and_xit_stack() {
    let (log, _) = run(&[
        Command::SetVfo { vfo: Vfo::A, hz: 14_074_000.0 },
        Command::SetVfo { vfo: Vfo::B, hz: 14_200_000.0 },
        Command::SetSplit(true),
        Command::SetXit { enabled: true, hz: 2_000 },
        Command::SetPtt(true),
    ]);
    assert_eq!(log.tx, vec![14_202_000.0]);
}
