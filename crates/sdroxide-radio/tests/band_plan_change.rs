//! A change of band plan withdraws the operator's band confirmations.
//!
//! The operator confirmed the transmit and the receive list *against a plan*. A
//! different plan can move a band edge or take a band away, so the engine
//! withdraws both confirmations when the plan changes — the radio then neither
//! tunes (receive list) nor keys (transmit list) until they are checked and
//! confirmed again, as if they never had been. Choosing the plan that is
//! already running changes nothing.
//!
//! One test, in a binary of its own: the plan is a process-wide global and the
//! config directory is redirected for the whole process.

use std::time::Duration;

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

/// The built-in tables with 2 m narrowed in Region 1: not the built-in plan, a
/// valid one, and with every HF band still in it so the dial can go there.
fn narrow() -> String {
    let doc = sdroxide_types::BandPlan::default().to_json_document();
    let row = r#"{"band": "M2", "lo_mhz": 144.0, "hi_mhz": 146.0}"#;
    assert!(doc.contains(row), "the built-in 2 m row moved");
    doc.replace(row, r#"{"band": "M2", "lo_mhz": 144.0, "hi_mhz": 144.4}"#)
        .replace(r#""name": """#, r#""name": "Narrow 2 m""#)
}

/// What the engine last said about the radio, kept between steps.
#[derive(Default)]
struct Seen {
    dial: f64,
    radio_cfg: Option<RadioConfig>,
    notices: Vec<String>,
}

struct Rig {
    h: EngineHandles,
    seen: Seen,
}

impl Rig {
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
        // A few ticks: the engine applies the command, and withdraws on the tick
        // after a plan change.
        std::thread::sleep(Duration::from_millis(350));
        self.drain();
    }

    /// Ask for `hz` and report where the dial is afterwards. A refused request
    /// publishes nothing, so the dial is the last one seen.
    fn tune(&mut self, hz: f64) -> f64 {
        self.send(Command::SetVfo { vfo: Vfo::A, hz });
        self.seen.dial
    }

    /// The operator checks both lists and confirms them, from a screen that has
    /// heard of the current state (its copy carries no fingerprint).
    fn confirm(&mut self) {
        let cfg = RadioConfig {
            tx_band_enable: Some(vec![Band::M20, Band::M40]),
            tx_bands_confirmed: true,
            rx_band_enable: Some(vec![Band::M20, Band::M40]),
            rx_bands_confirmed: true,
            ..RadioConfig::default()
        };
        self.send(Command::SetRadioConfig { cfg: Box::new(cfg), reopen: false });
    }
}

#[test]
fn a_change_of_plan_withdraws_the_confirmations_and_the_same_plan_does_not() {
    let root = std::env::temp_dir().join(format!("sdroxide-plan-change-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };

    let h = start_engine(
        Box::new(MockRx { center: 14_074_000.0 }),
        caps(),
        EngineConfig { rx_bands_enforced: true, ..Default::default() },
    );
    let mut rig = Rig { h, seen: Seen::default() };
    let thread = rig.h.thread.take();

    // Confirmed against the built-in plan: the dial goes where it is told.
    rig.confirm();
    assert_eq!(rig.tune(7_074_000.0), 7_074_000.0, "confirmed: 40 m is YES");

    // A plan is added: nothing changes, because adding is not running it.
    rig.send(Command::ImportBandPlan { name: "Narrow 2m".into(), json: narrow() });
    assert_eq!(rig.tune(14_074_000.0), 14_074_000.0, "adding a plan withdraws nothing");
    assert!(rig.seen.radio_cfg.as_ref().is_none_or(|c| c.tx_bands_confirmed));

    // A plan the station cannot take is refused and changes nothing.
    rig.send(Command::ImportBandPlan { name: "junk".into(), json: "not a plan".into() });
    assert_eq!(rig.tune(7_074_000.0), 7_074_000.0);

    // Running it withdraws both confirmations: the lists are still ticked, but
    // the radio no longer tunes and the file on disk says unconfirmed.
    rig.send(Command::SelectBandPlan { name: Some("narrow-2m".into()) });
    let cfg = rig.seen.radio_cfg.clone().expect("the engine announced the radio's config");
    assert!(!cfg.tx_bands_confirmed, "the transmit list is unconfirmed again");
    assert!(!cfg.rx_bands_confirmed, "the receive list is unconfirmed again");
    assert_eq!(cfg.band_plan_fp, 0, "nothing is confirmed against any plan");
    assert_eq!(cfg.tx_band_enable, Some(vec![Band::M20, Band::M40]), "the lists are kept");
    assert!(
        rig.seen.notices.iter().any(|n| n.contains("band plan changed")),
        "the operator is told why: {:?}",
        rig.seen.notices
    );
    assert_eq!(rig.tune(14_074_000.0), 7_074_000.0, "unconfirmed: the dial does not move");

    // Checked and confirmed again: it tunes.
    rig.confirm();
    assert_eq!(rig.tune(14_074_000.0), 14_074_000.0);

    // Choosing the plan that is already running changes nothing.
    rig.send(Command::SelectBandPlan { name: Some("narrow-2m".into()) });
    assert_eq!(rig.tune(7_074_000.0), 7_074_000.0, "the same plan withdraws nothing");
    assert!(rig.seen.radio_cfg.as_ref().unwrap().rx_bands_confirmed);

    // One click back to the built-in tables is a change too.
    rig.send(Command::SelectBandPlan { name: None });
    assert!(!rig.seen.radio_cfg.as_ref().unwrap().rx_bands_confirmed, "reset is a change");
    assert_eq!(rig.tune(14_074_000.0), 7_074_000.0);

    // A settings dialog that opened before the change still holds a copy marked
    // confirmed, with the fingerprint of the plan it was confirmed against; an
    // Apply from it must not undo the withdrawal.
    let stale = RadioConfig {
        tx_band_enable: Some(vec![Band::M20, Band::M40]),
        tx_bands_confirmed: true,
        rx_band_enable: Some(vec![Band::M20, Band::M40]),
        rx_bands_confirmed: true,
        band_plan_fp: 0x1234_5678_9abc_def0,
        ..RadioConfig::default()
    };
    rig.send(Command::SetRadioConfig { cfg: Box::new(stale), reopen: false });
    assert!(!rig.seen.radio_cfg.as_ref().map_or(false, |c| c.rx_bands_confirmed));
    assert_eq!(rig.tune(14_074_000.0), 7_074_000.0, "a stale confirmation does not count");

    // A plan that is missing or will not be read leaves the choice as it was.
    rig.send(Command::SelectBandPlan { name: Some("no-such-plan".into()) });
    assert!(
        rig.seen.notices.iter().any(|n| n.contains("no-such-plan")),
        "said why: {:?}",
        rig.seen.notices
    );

    // Deleting the plan that is not running works; deleting the running one is
    // refused.
    rig.send(Command::SelectBandPlan { name: Some("narrow-2m".into()) });
    rig.send(Command::DeleteBandPlan { name: "narrow-2m".into() });
    assert!(
        rig.seen.notices.iter().any(|n| n.contains("is the one running")),
        "{:?}",
        rig.seen.notices
    );
    rig.send(Command::SelectBandPlan { name: None });
    rig.send(Command::DeleteBandPlan { name: "narrow-2m".into() });
    assert!(rig.seen.notices.iter().any(|n| n.contains("deleted")), "{:?}", rig.seen.notices);

    drop(rig.h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
}
