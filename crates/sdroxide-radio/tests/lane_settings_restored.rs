//! A lane the operator left running comes back running.
//!
//! HFDL and QO-100 were the only wideband lanes whose settings were never
//! written to disk — every other one (ADS-B, AIS, VDL2, ISM, DAB, the scanner)
//! has saved and restored its own from the start, and builds its decoder from
//! the file at launch. So those two alone came up off, on a default channel,
//! every time the program opened, which is the shape r/ShortwavePlus and
//! discussion #90 both described without being able to name it.
//!
//! This is the wiring half of that fix: the file is read into the engine's
//! state, and the state is what the lane and the panels read. The file
//! round-trip itself is `sdroxide-config/tests/lane_settings_persist.rs`.

use std::time::Duration;

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{DeviceCaps, HfdlSettings, Qo100Settings, RadioEvent};

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
fn the_lanes_a_previous_session_left_on_are_running_again() {
    let root = std::env::temp_dir().join(format!("sdroxide-lane-restore-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };

    // What a previous session left behind.
    let hfdl_wanted = HfdlSettings { enabled: true, frequency_hz: 5_451_000.0 };
    let qo100_wanted = Qo100Settings {
        enabled: true,
        park_lo_hz: 3_000.0,
        park_hi_hz: 9_000.0,
        ..Qo100Settings::default()
    };
    sdroxide_config::save_hfdl_config(&hfdl_wanted).unwrap();
    sdroxide_config::save_qo100_config(&qo100_wanted).unwrap();

    let caps = DeviceCaps {
        driver: "mock".into(),
        label: "mock".into(),
        rx_channels: 1,
        sample_rates: vec![RATE],
        freq_ranges_rx: vec![(0.0, 1_000_000_000.0)],
        ..DeviceCaps::default()
    };
    let mut h =
        start_engine(Box::new(MockSource { center: 5_500_000.0 }), caps, EngineConfig::default());
    let thread = h.thread.take();
    std::thread::sleep(Duration::from_millis(250));

    let (mut hfdl, mut qo100) = (None, None);
    while let Ok(ev) = h.event_rx.try_recv() {
        if let RadioEvent::State(s) = ev {
            hfdl = Some(s.hfdl);
            qo100 = Some(s.qo100);
        }
    }
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }

    let hfdl = hfdl.expect("the engine published no state at all");
    assert!(
        hfdl.enabled,
        "the HFDL lane was on when the program closed and came back off — the operator \
         has to find the decoder and the channel again every time"
    );
    assert_eq!(hfdl.frequency_hz, 5_451_000.0, "and on the default channel, not theirs");

    let qo100 = qo100.expect("the engine published no state at all");
    assert!(qo100.enabled, "the QO-100 tracker was on and came back off");
    assert_eq!(qo100.park_hi_hz, 9_000.0, "and with its parking window forgotten");

    let _ = std::fs::remove_dir_all(&root);
    unsafe { std::env::remove_var("SDROXIDE_CONFIG_DIR") };
}
