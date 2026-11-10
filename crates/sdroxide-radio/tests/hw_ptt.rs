//! The radio's own PTT line — a foot switch, a mic button, or whatever is
//! wired to the board's PTT input. A source reports it through
//! [`ControlUpdate::Ptt`] and the engine must key on it exactly as it does on
//! the on-screen button.
//!
//! The second test is the reason the engine tracks who owns an over at all:
//! several boards report their PTT pin and their own MOX state on one line, so
//! an over we started comes straight back as "the operator is holding PTT". If
//! that were adopted, the release of a line we were never following would unkey
//! — or worse, its stale "closed" would hold the transmitter down after the
//! real owner let go.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use sdroxide_radio::{Complex32, ControlUpdate, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DeviceCaps};

/// What the test can see and drive from outside the engine.
#[derive(Default)]
struct Line {
    /// The radio's PTT input, as the test is holding it.
    closed: AtomicBool,
    begins: AtomicUsize,
    ends: AtomicUsize,
}

/// A transmit-capable stand-in with a PTT line the test can operate.
struct MockSource {
    line: Arc<Line>,
    /// Last level handed to the engine, so only edges are reported — the same
    /// contract the HPSDR source keeps.
    reported: bool,
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
        "mock source with a PTT line".into()
    }
    fn poll_control(&mut self) -> Vec<ControlUpdate> {
        let closed = self.line.closed.load(Ordering::Relaxed);
        if closed == self.reported {
            return Vec::new();
        }
        self.reported = closed;
        vec![ControlUpdate::Ptt(closed)]
    }
    fn tx_begin(&mut self, _center_hz: f64, rate: f64) -> Result<f64> {
        self.line.begins.fetch_add(1, Ordering::Relaxed);
        Ok(rate)
    }
    fn tx_write(&mut self, _samples: &[Complex32]) -> Result<()> {
        Ok(())
    }
    fn tx_end(&mut self) -> Result<()> {
        self.line.ends.fetch_add(1, Ordering::Relaxed);
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

/// Spin until `f` holds, or give up after `timeout`.
fn wait_for(timeout: Duration, mut f: impl FnMut() -> bool) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if f() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    false
}

/// Start an engine on a mock source and hand back its handle plus the PTT line.
fn engine() -> (sdroxide_radio::EngineHandles, Arc<Line>) {
    let rate = 384_000.0;
    let line = Arc::new(Line::default());
    let src = MockSource { line: Arc::clone(&line), reported: false, center: 14_200_000.0, rate };
    let cfg = EngineConfig { tx_bands_open: true, tx_ham_only: false, ..Default::default() };
    let h = start_engine(Box::new(src), tx_caps(rate), cfg);
    // Let the engine finish coming up before the line is operated.
    std::thread::sleep(Duration::from_millis(150));
    (h, line)
}

#[test]
fn radio_ptt_line_keys_and_unkeys() {
    let (mut h, line) = engine();
    let thread = h.thread.take();

    line.closed.store(true, Ordering::Relaxed);
    assert!(
        wait_for(Duration::from_secs(3), || line.begins.load(Ordering::Relaxed) == 1),
        "closing the radio's PTT line should key the transmitter"
    );

    line.closed.store(false, Ordering::Relaxed);
    assert!(
        wait_for(Duration::from_secs(3), || line.ends.load(Ordering::Relaxed) == 1),
        "releasing it should unkey"
    );

    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
}

#[test]
fn a_ptt_line_echoing_our_own_mox_cannot_end_someone_else_s_over() {
    let (mut h, line) = engine();
    let thread = h.thread.take();

    // TUNE owns this over.
    h.cmd_tx.send(Command::SetTune(true)).unwrap();
    assert!(
        wait_for(Duration::from_secs(3), || line.begins.load(Ordering::Relaxed) == 1),
        "TUNE should key the transmitter"
    );

    // Now the board reports PTT closed because *we* are keyed, then reports it
    // open again. Neither may touch the over TUNE is holding.
    line.closed.store(true, Ordering::Relaxed);
    std::thread::sleep(Duration::from_millis(250));
    line.closed.store(false, Ordering::Relaxed);
    std::thread::sleep(Duration::from_millis(250));
    assert_eq!(line.ends.load(Ordering::Relaxed), 0, "the tune must still be on the air");
    assert_eq!(line.begins.load(Ordering::Relaxed), 1, "and must not have been re-keyed");

    // TUNE still ends it, and the transmitter is not left down.
    h.cmd_tx.send(Command::SetTune(false)).unwrap();
    assert!(
        wait_for(Duration::from_secs(3), || line.ends.load(Ordering::Relaxed) == 1),
        "ending TUNE should unkey"
    );

    // The line is free again afterwards: a real press still works.
    line.closed.store(true, Ordering::Relaxed);
    assert!(
        wait_for(Duration::from_secs(3), || line.begins.load(Ordering::Relaxed) == 2),
        "the PTT line should key again once nothing else owns the transmitter"
    );

    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
}
