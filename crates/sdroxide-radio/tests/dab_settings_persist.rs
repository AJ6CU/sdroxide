//! The DAB panel's memory survives a restart of the station.
//!
//! What a scan found — the channels, the stations on each, the operator's
//! starred presets and the station chosen — is a fact about where the receiver
//! is, and finding it again costs a two-minute sweep. The engine used to start
//! every session from `DabSettings::default()` and write nothing, so all of it
//! was gone each time the server restarted. This drives the real command path:
//! set it, stop the engine, start a fresh one on the same config directory, and
//! read it back from the first state.

use std::time::{Duration, Instant};

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DabKnownStation, DabSettings, DeviceCaps, RadioEvent, RadioState};

const RATE: f64 = 2_500_000.0;

struct Silence {
    center: f64,
}

impl IqSource for Silence {
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
        std::thread::sleep(Duration::from_millis(10));
        let n = buf.len().min(2048);
        buf[..n].fill(Complex32::new(0.0, 0.0));
        Ok(n)
    }
    fn describe(&self) -> String {
        "silence".into()
    }
}

fn caps() -> DeviceCaps {
    DeviceCaps {
        driver: "mock".into(),
        label: "mock".into(),
        rx_channels: 1,
        sample_rates: vec![RATE],
        freq_ranges_rx: vec![(0.0, 1_000_000_000.0)],
        ..DeviceCaps::default()
    }
}

struct Harness {
    h: sdroxide_radio::EngineHandles,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Harness {
    fn start() -> Self {
        let mut h = start_engine(
            Box::new(Silence { center: 197_648_000.0 }),
            caps(),
            EngineConfig { store: sdroxide_config::Store::radio(0), ..Default::default() },
        );
        let thread = h.thread.take();
        Harness { h, thread }
    }

    fn state_where(&self, what: &str, mut pred: impl FnMut(&RadioState) -> bool) -> RadioState {
        let deadline = Instant::now() + Duration::from_secs(5);
        while Instant::now() < deadline {
            while let Ok(ev) = self.h.event_rx.try_recv() {
                if let RadioEvent::State(s) = ev
                    && pred(&s)
                {
                    return s;
                }
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        panic!("timed out waiting for {what}");
    }

    fn stop(mut self) {
        drop(self.h.cmd_tx);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

fn station(channel: &str, id: &str, label: &str) -> DabKnownStation {
    DabKnownStation { channel: channel.into(), service_id: id.into(), label: label.into() }
}

/// One test, because it redirects the config directory through the
/// environment, which is process-global.
#[test]
fn what_a_scan_found_survives_a_restart() {
    let root = std::env::temp_dir().join(format!("sdroxide-dab-persist-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("scratch dir");
    // SAFETY: this is the only test in this binary; nothing races the setter.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };
    let tci_off = sdroxide_types::TciServerConfig { enabled: false, ..Default::default() };
    sdroxide_config::Store::radio(0).save_tci_server_config(&tci_off).unwrap();

    let want = DabSettings {
        channel: "8B".into(),
        service_id: "F201".into(),
        found: vec!["8B".into(), "11C".into()],
        stations: vec![
            station("8B", "F201", "BFM BUSINESS"),
            station("8B", "F203", "FIP"),
            station("11C", "F401", "RADIO NOVA"),
        ],
        presets: vec![station("8B", "F203", "FIP")],
        ..DabSettings::default()
    };

    let eng = Harness::start();
    eng.state_where("the first state", |_| true);
    eng.h.cmd_tx.send(Command::SetDabConfig(want.clone())).unwrap();
    eng.state_where("the settings applied", |s| s.dab.found == want.found);
    eng.stop();

    let restarted = Harness::start();
    let s = restarted.state_where("the first state after a restart", |_| true);
    assert_eq!(s.dab.found, want.found, "the channels the scan found were forgotten");
    assert_eq!(s.dab.stations, want.stations, "the stations on each channel were forgotten");
    assert_eq!(s.dab.presets, want.presets, "the starred presets were forgotten");
    assert_eq!(
        (s.dab.channel.as_str(), s.dab.service_id.as_str()),
        ("8B", "F201"),
        "the station chosen was forgotten"
    );

    restarted.stop();
    let _ = std::fs::remove_dir_all(&root);
}
