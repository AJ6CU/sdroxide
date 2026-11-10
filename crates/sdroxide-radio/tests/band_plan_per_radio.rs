//! Two radios at one station, each on its own band plan.
//!
//! A radio can run a plan of its own while another follows the station's. What
//! the one does must never reach the other: its band edges, its TX and RX band
//! confirmations, its `radio.json`. One test in a binary of its own — the
//! station's plan is a process-wide global and the config directory is
//! redirected for the whole process.

use std::time::Duration;

use sdroxide_config::Store;
use sdroxide_radio::{Complex32, EngineConfig, EngineHandles, IqSource, Result, start_engine};
use sdroxide_types::{Band, Command, DeviceCaps, RadioConfig, RadioEvent, Vfo};

struct MockRx {
    center: f64,
}

impl IqSource for MockRx {
    fn sample_rate(&self) -> f64 {
        600_000.0
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
        "mock rx".into()
    }
}

fn caps() -> DeviceCaps {
    DeviceCaps {
        driver: "mock".into(),
        label: "mock".into(),
        rx_channels: 1,
        sample_rates: vec![600_000.0],
        freq_ranges_rx: vec![(100_000.0, 60_000_000.0)],
        ..DeviceCaps::default()
    }
}

/// The built-in tables without 20 m, so 14.074 MHz is in no amateur band.
fn without_20m() -> String {
    let doc = sdroxide_types::BandPlan::default().to_json_document();
    let row = r#"      {"band": "M20", "lo_mhz": 14.0, "hi_mhz": 14.35},"#;
    assert!(doc.contains(row), "the built-in 20 m row moved");
    doc.replace(&format!("{row}\n"), "").replace(r#""name": """#, r#""name": "No 20 m""#)
}

#[derive(Default)]
struct Seen {
    dial: f64,
    radio_cfg: Option<RadioConfig>,
    notices: Vec<String>,
}

struct Radio {
    h: EngineHandles,
    seen: Seen,
}

impl Radio {
    fn start(id: u32) -> Radio {
        let h = start_engine(
            Box::new(MockRx { center: 14_074_000.0 }),
            caps(),
            EngineConfig {
                rx_bands_enforced: true,
                store: Store::radio(id),
                instance: id,
                ..Default::default()
            },
        );
        Radio { h, seen: Seen::default() }
    }

    fn drain(&mut self) {
        while let Ok(ev) = self.h.event_rx.try_recv() {
            match ev {
                RadioEvent::State(s) => self.seen.dial = s.vfo_a_hz,
                RadioEvent::RadioConfig(c) => self.seen.radio_cfg = Some(*c),
                RadioEvent::Notice(Some(n)) => self.seen.notices.push(n),
                _ => {}
            }
        }
    }

    fn send(&mut self, cmd: Command) {
        self.h.cmd_tx.send(cmd).unwrap();
        std::thread::sleep(Duration::from_millis(350));
        self.drain();
    }

    fn tune(&mut self, hz: f64) -> f64 {
        self.send(Command::SetVfo { vfo: Vfo::A, hz });
        self.seen.dial
    }

    fn confirm(&mut self) {
        self.confirm_bands(vec![Band::M20, Band::M40]);
    }

    /// Confirm TX and RX lists that tick exactly `bands`.
    fn confirm_bands(&mut self, bands: Vec<Band>) {
        let cfg = RadioConfig {
            tx_band_enable: Some(bands.clone()),
            tx_bands_confirmed: true,
            rx_band_enable: Some(bands),
            rx_bands_confirmed: true,
            ..RadioConfig::default()
        };
        self.send(Command::SetRadioConfig { cfg: Box::new(cfg), reopen: false });
    }

    fn confirmed(&self) -> bool {
        self.seen.radio_cfg.as_ref().is_some_and(|c| c.tx_bands_confirmed && c.rx_bands_confirmed)
    }
}

#[test]
fn a_radio_on_its_own_plan_does_not_touch_another_and_is_not_touched() {
    let root = std::env::temp_dir().join(format!("sdroxide-per-radio-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };

    let mut a = Radio::start(1);
    let mut b = Radio::start(2);
    let (ta, tb) = (a.h.thread.take(), b.h.thread.take());

    // Both radios checked and confirmed their lists against the built-in plan.
    a.confirm();
    b.confirm();
    assert_eq!(a.tune(14_074_000.0), 14_074_000.0, "A: 20 m is YES");
    assert_eq!(b.tune(14_074_000.0), 14_074_000.0, "B: 20 m is YES");

    // A plan is added to the station's library (by A; the library is the station's).
    a.send(Command::ImportBandPlan { name: "No 20m".into(), json: without_20m() });

    // A runs it on its own. A's confirmations go; B's stay, and B still tunes.
    a.send(Command::SelectRadioBandPlan { name: Some("no-20m".into()) });
    assert!(!a.confirmed(), "A's lists are unconfirmed again");
    assert_eq!(a.seen.radio_cfg.as_ref().unwrap().band_plan.as_deref(), Some("no-20m"));
    assert!(
        a.seen.notices.iter().any(|n| n.contains("This radio now runs")),
        "{:?}",
        a.seen.notices
    );
    b.drain();
    assert!(b.confirmed() || b.seen.radio_cfg.is_none(), "B was not asked to withdraw anything");
    assert_eq!(b.tune(14_074_000.0), 14_074_000.0, "B's plan is the station's: 20 m is still there");
    assert_eq!(b.tune(7_074_000.0), 7_074_000.0);

    // Both radios confirm lists with 40 m ticked and 20 m not. A frequency in
    // no band at all is open once a list is confirmed (there is no row for it),
    // so the plan is what tells the two radios apart: under A's own plan there
    // is no 20 m and 14.074 MHz is in no band, so A tunes there; under the
    // station's plan it is 20 m, which B's list leaves off, so B is refused.
    a.confirm_bands(vec![Band::M40]);
    b.confirm_bands(vec![Band::M40]);
    assert_eq!(a.tune(7_074_000.0), 7_074_000.0, "40 m is YES on A");
    assert_eq!(b.tune(7_074_000.0), 7_074_000.0, "40 m is YES on B");
    assert_eq!(a.tune(14_074_000.0), 14_074_000.0, "no 20 m on A's own plan: in no band, open");
    assert_eq!(b.tune(14_074_000.0), 7_074_000.0, "20 m is on B's (the station's) plan and off its list");

    // The choice is in A's radio.json and in no other.
    let a_json = std::fs::read_to_string(root.join("radio-1").join("radio.json")).unwrap();
    assert!(a_json.contains("\"band_plan\": \"no-20m\"") || a_json.contains("\"band_plan\":\"no-20m\""), "{a_json}");
    let b_json = std::fs::read_to_string(root.join("radio-2").join("radio.json")).unwrap_or_default();
    assert!(!b_json.contains("no-20m"), "B's file knows nothing of A's plan: {b_json}");

    // A plan some radio runs cannot be deleted from under it.
    b.send(Command::DeleteBandPlan { name: "no-20m".into() });
    assert!(
        b.seen.notices.iter().any(|n| n.contains("run by a radio of its own")),
        "{:?}",
        b.seen.notices
    );

    // The station moves to another plan (chosen from B's screen): B follows the
    // station, so B's confirmations go; A runs its own plan and keeps its.
    let a_was = a.confirmed();
    assert!(a_was);
    b.send(Command::SelectBandPlan { name: Some("bandplan-de".into()) });
    a.send(Command::ReloadBandPlan); // A hears nothing new from the station
    assert!(!b.confirmed(), "B follows the station: unconfirmed");
    a.drain();
    assert!(a.confirmed(), "A runs its own plan: the station's change is not its change");

    // A leaves its own plan for the station's, which is now Germany: a different
    // plan from the one A was on, so A is unconfirmed again.
    a.send(Command::SelectRadioBandPlan { name: None });
    assert!(!a.confirmed(), "A moved from its own plan to the station's");
    assert_eq!(a.seen.radio_cfg.as_ref().unwrap().band_plan, None);

    // Choosing what a radio already runs is not a change.
    a.confirm();
    a.send(Command::SelectRadioBandPlan { name: None });
    assert!(a.confirmed(), "A already follows the station: nothing to withdraw");

    drop(a.h.cmd_tx);
    drop(b.h.cmd_tx);
    for t in [ta, tb].into_iter().flatten() {
        let _ = t.join();
    }
}
