//! Transmitting on a rig that sends quadrature and takes audio back.
//!
//! A CAT rig on a sound card modulates the audio we put into it, whatever its
//! receive stream is carrying — so it has no I/Q transmitter, and its source
//! implements [`IqSource::tx_write_audio`] and not `tx_write`. Which of the
//! two the engine reaches for is `DeviceCaps::audio_mode || tx_audio`, and a
//! quadrature rig is not `audio_mode`: its stream is ordinary complex
//! baseband. So `tx_audio` is the only thing standing between such a rig and
//! the modulated-I/Q path, where the very first block would ask an
//! `IqSource` with no transmitter to write one and fail the over with "device
//! is not transmit capable" — which is exactly what a Kenwood in I/Q did.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DeviceCaps, RadioEvent};

const DIAL_HZ: f64 = 14_100_000.0;

/// What the mock rig was asked to transmit.
#[derive(Default)]
struct Log {
    /// Blocks handed to `tx_write_audio` — the path this rig actually has.
    audio_blocks: usize,
    /// Whether `tx_begin` was reached at all.
    keyed: bool,
}

/// A CAT rig sending quadrature: complex baseband in, 48 kHz audio out, and
/// deliberately no `tx_write` — inheriting the trait's default is the whole
/// point, because that default is the failure being tested for.
struct MockIqCat {
    log: Arc<Mutex<Log>>,
}

impl IqSource for MockIqCat {
    fn sample_rate(&self) -> f64 {
        48_000.0
    }
    fn center_hz(&self) -> f64 {
        DIAL_HZ
    }
    fn set_center_hz(&mut self, _hz: f64) -> Result<()> {
        Ok(())
    }
    fn center_is_dial(&self) -> bool {
        true
    }
    fn read(&mut self, buf: &mut [Complex32]) -> Result<usize> {
        std::thread::sleep(Duration::from_millis(5));
        let n = buf.len().min(1024);
        buf[..n].fill(Complex32::new(0.0, 0.0));
        Ok(n)
    }
    fn describe(&self) -> String {
        "mock CAT rig (I/Q)".into()
    }
    fn tx_begin(&mut self, _center_hz: f64, rate: f64) -> Result<f64> {
        self.log.lock().unwrap().keyed = true;
        Ok(rate)
    }
    fn tx_write_audio(&mut self, _audio: &[f32]) -> Result<()> {
        self.log.lock().unwrap().audio_blocks += 1;
        Ok(())
    }
    fn tx_end(&mut self) -> Result<()> {
        Ok(())
    }
}

/// The shape of a CAT rig on a sound card. `tx_audio` is the switch under
/// test; `audio_mode` is what the *receive* format decides, and quadrature
/// leaves it false.
fn caps(tx_audio: bool) -> DeviceCaps {
    DeviceCaps {
        driver: "mock-cat".into(),
        label: "mock CAT (I/Q)".into(),
        rx_channels: 1,
        tx_channels: 1,
        audio_mode: false,
        tx_audio,
        freq_ranges_rx: vec![(10_000.0, 148_000_000.0)],
        freq_ranges_tx: vec![(1_800_000.0, 54_000_000.0)],
        ..DeviceCaps::default()
    }
}

/// Key the rig up and report what it saw, plus any failure the engine
/// announced. TUNE rather than PTT, so the over needs no microphone: it puts
/// a carrier out through whichever transmit path the capabilities chose.
fn key_up(tx_audio: bool) -> (Log, Option<String>) {
    let log = Arc::new(Mutex::new(Log::default()));
    let src = MockIqCat { log: Arc::clone(&log) };
    let cfg = EngineConfig { tx_bands_open: true, tx_ham_only: false, ..Default::default() };
    let mut h = start_engine(Box::new(src), caps(tx_audio), cfg);
    let thread = h.thread.take();

    h.cmd_tx.send(Command::SetTune(true)).unwrap();

    let mut lost = None;
    let deadline = Instant::now() + Duration::from_secs(2);
    while Instant::now() < deadline {
        while let Ok(ev) = h.event_rx.try_recv() {
            if let RadioEvent::ConnectionLost(msg) = ev {
                lost = Some(msg);
            }
        }
        // Enough blocks to be sure the over is running, not merely beginning.
        if lost.is_some() || log.lock().unwrap().audio_blocks > 4 {
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
    }

    // The engine may already have stopped — that is the fault this test is
    // about ("the over dies on the first block"), so asking it to stop tuning
    // is best effort rather than something to unwrap on.
    let _ = h.cmd_tx.send(Command::SetTune(false));
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    let out = std::mem::take(&mut *log.lock().unwrap());
    (out, lost)
}

/// The fix: a quadrature CAT rig transmits by feeding its sound card, and the
/// over runs.
#[test]
fn a_quadrature_cat_rig_transmits_through_its_sound_card() {
    let (log, lost) = key_up(true);
    assert_eq!(lost, None, "the over must not fail");
    assert!(log.keyed, "the rig was keyed");
    assert!(log.audio_blocks > 0, "audio went to the rig's sound card");
}

/// The bug, pinned so the capability cannot quietly go missing again: without
/// `tx_audio` the engine builds modulated I/Q for a source that has no
/// transmitter to take it, and the over dies on the first block.
#[test]
fn without_the_capability_the_over_dies_on_the_first_block() {
    let (log, lost) = key_up(false);
    assert_eq!(
        lost.as_deref(),
        Some("device is not transmit capable"),
        "this is the failure the operator saw"
    );
    assert_eq!(log.audio_blocks, 0, "nothing ever reached the sound card");
}
