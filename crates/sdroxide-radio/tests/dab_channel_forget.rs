//! A DAB channel cleared from the list stays cleared.
//!
//! The panel's × removes a channel and its stations and sends the whole
//! settings back with `SetDabConfig`. The engine kept them — and saved them —
//! but published no state, so the server's cached snapshot still held the old
//! list. A browser that reloaded or reconnected was handed that snapshot, the
//! channel was back, and the next settings it sent wrote it into `dab.json`
//! again. The change has to go out as state, like any other setting.

use std::time::Duration;

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DabSettings, DeviceCaps, RadioEvent};

const RATE: f64 = 2_400_000.0;

struct MockSource {
    center: f64,
}

impl IqSource for MockSource {
    fn sample_rate(&self) -> f64 {
        RATE
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
        "mock rx source".into()
    }
}

#[test]
fn a_forgotten_channel_reaches_the_published_state() {
    let root = std::env::temp_dir().join(format!("sdroxide-dab-forget-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };

    let caps = DeviceCaps {
        driver: "mock".into(),
        label: "mock".into(),
        rx_channels: 1,
        sample_rates: vec![RATE],
        freq_ranges_rx: vec![(0.0, 1_000_000_000.0)],
        ..DeviceCaps::default()
    };
    let mut h =
        start_engine(Box::new(MockSource { center: 222_064_000.0 }), caps, EngineConfig::default());
    let thread = h.thread.take();
    std::thread::sleep(Duration::from_millis(150));

    let mut dab = DabSettings {
        channel: "11D".into(),
        found: ["8B", "11D", "5A"].map(String::from).to_vec(),
        ..DabSettings::default()
    };
    h.cmd_tx.send(Command::SetDabConfig(dab.clone())).unwrap();
    std::thread::sleep(Duration::from_millis(200));
    while h.event_rx.try_recv().is_ok() {}

    // The ×: the channel and its stations go, and the settings go back.
    dab.forget_channel("5A");
    h.cmd_tx.send(Command::SetDabConfig(dab.clone())).unwrap();
    std::thread::sleep(Duration::from_millis(300));

    let mut last = None;
    while let Ok(ev) = h.event_rx.try_recv() {
        if let RadioEvent::State(s) = ev {
            last = Some(s.dab.found);
        }
    }
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    let found = last.expect(
        "the engine published no state after the channel was cleared, so a client that \
         reconnects is handed the old list with 5A still in it",
    );
    assert_eq!(found, ["8B", "11D"].map(String::from).to_vec());
}
