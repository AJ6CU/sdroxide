//! `ControlUpdate::AtsMiniTuning` is a condition, not a control.
//!
//! The ATS Mini tunes by stepping its own band cycle, so the dial on screen is
//! already the requested frequency while the radio is still walking to it. The
//! engine has nothing to apply — the dial is where the operator put it — it
//! only forwards the level to the screen, whose readout says the radio is
//! catching up. This pins that forwarding, since it is native-only and has no
//! other test end to end.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sdroxide_radio::{Complex32, ControlUpdate, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DeviceCaps, RadioEvent, Vfo};

const RATE: f64 = 48_000.0;
const DIAL: f64 = 14_074_000.0;

/// What the test sets and the source reports on its next poll.
#[derive(Default)]
struct Flags {
    /// A tuning level to report, once, the way the source sends it on a change.
    tuning: Option<bool>,
}

struct MockSource {
    flags: Arc<Mutex<Flags>>,
}

impl IqSource for MockSource {
    fn sample_rate(&self) -> f64 {
        RATE
    }
    fn center_hz(&self) -> f64 {
        DIAL
    }
    fn set_center_hz(&mut self, _hz: f64) -> Result<()> {
        Ok(())
    }
    fn read(&mut self, buf: &mut [Complex32]) -> Result<usize> {
        std::thread::sleep(Duration::from_millis(5));
        let n = buf.len().min(1024);
        buf[..n].fill(Complex32::new(0.0, 0.0));
        Ok(n)
    }
    fn describe(&self) -> String {
        "mock ATS Mini".into()
    }
    fn poll_control(&mut self) -> Vec<ControlUpdate> {
        self.flags
            .lock()
            .unwrap()
            .tuning
            .take()
            .map(ControlUpdate::AtsMiniTuning)
            .into_iter()
            .collect()
    }
}

fn caps() -> DeviceCaps {
    DeviceCaps {
        driver: "ats-mini".into(),
        label: "mock ATS Mini".into(),
        rx_channels: 1,
        tx_channels: 0,
        audio_mode: true,
        sample_rates: vec![RATE],
        freq_ranges_rx: vec![(150_000.0, 30_000_000.0)],
        ..DeviceCaps::default()
    }
}

/// Wait for the level `want` to arrive, and return when it does — the event is
/// only sent on a change, so one per flip is what to expect.
fn wait_for(h: &sdroxide_radio::EngineHandles, want: bool) {
    let deadline = Instant::now() + Duration::from_secs(3);
    while Instant::now() < deadline {
        while let Ok(ev) = h.event_rx.try_recv() {
            if let RadioEvent::AtsMiniTuning(on) = ev
                && on == want
            {
                return;
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    panic!("no AtsMiniTuning({want}) event arrived");
}

#[test]
fn a_source_tuning_level_reaches_the_screen() {
    let flags = Arc::new(Mutex::new(Flags::default()));
    let src = MockSource { flags: Arc::clone(&flags) };
    let cfg = EngineConfig { tx_ham_only: false, ..Default::default() };
    let mut h = start_engine(Box::new(src), caps(), cfg);
    h.cmd_tx.send(Command::SetVfo { vfo: Vfo::A, hz: DIAL }).unwrap();

    flags.lock().unwrap().tuning = Some(true);
    wait_for(&h, true);

    flags.lock().unwrap().tuning = Some(false);
    wait_for(&h, false);

    let thread = h.thread.take();
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
}
