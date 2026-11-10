//! The LISTEN tab may put any mode on any band.
//!
//! `Command::SetMode` refuses a mode the band does not carry — that is the
//! OPERATE tab's rule and the engine's own guard against a pair that can only
//! ever be silence. The listener's screen sends `Command::SetModeListen`
//! instead, which applies the same mode change without the band rule: trying a
//! decoder where it does not belong is the exercise, not a mistake. Transmit
//! legality is untouched — the band lockout and the rails still decide what may
//! be keyed.

use std::time::Duration;

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DeviceCaps, Mode, RadioEvent, RxId, Vfo};

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

fn isolate_config() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let root =
            std::env::temp_dir().join(format!("sdroxide-listen-unlock-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };
    });
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

/// Tune to `from`, send `cmd`, and report the main receiver's mode afterwards.
fn mode_after(from: f64, cmd: Command) -> Mode {
    isolate_config();
    let mut h = start_engine(
        Box::new(MockSource { center: from }),
        caps(),
        EngineConfig { tx_ham_only: false, ..Default::default() },
    );
    let thread = h.thread.take();
    std::thread::sleep(Duration::from_millis(150));
    h.cmd_tx.send(Command::SetVfo { vfo: Vfo::A, hz: from }).unwrap();
    std::thread::sleep(Duration::from_millis(150));
    h.cmd_tx.send(cmd).unwrap();
    std::thread::sleep(Duration::from_millis(300));

    let mut last = None;
    while let Ok(ev) = h.event_rx.try_recv() {
        if let RadioEvent::State(s) = ev {
            last = Some(s.rx[0].mode);
        }
    }
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    last.expect("the engine should publish state")
}

/// The FM broadcast band carries WFM alone, so AM is the pair the band rule
/// refuses. On an FM frequency the default mode is already WFM, so a refused
/// change leaves the receiver there.
#[test]
fn the_band_rule_refuses_a_mode_the_band_does_not_carry() {
    let m = mode_after(100_000_000.0, Command::SetMode { rx: RxId::Main, mode: Mode::Am });
    assert_ne!(m, Mode::Am, "SetMode must honour the band rule on the FM broadcast band");
}

/// ...and the listener's command puts it there anyway.
#[test]
fn the_listen_command_puts_the_mode_on_the_band_anyway() {
    let m = mode_after(100_000_000.0, Command::SetModeListen { rx: RxId::Main, mode: Mode::Am });
    assert_eq!(m, Mode::Am, "SetModeListen must bypass the band rule");
}

/// The Aircraft window's buttons. The airband carries AM, VDL2 and ACARS in the
/// band table, so ADS-B and HFDL are the pairs `SetMode` refuses there — which
/// is why those buttons did nothing but raise a notice. The listener's command
/// puts either on the airband.
#[test]
fn the_aviation_modes_can_be_put_on_the_airband_by_the_listen_command() {
    for mode in [Mode::Adsb, Mode::Hfdl] {
        let refused = mode_after(125_000_000.0, Command::SetMode { rx: RxId::Main, mode });
        assert_ne!(refused, mode, "{mode:?}: SetMode should be refused on the airband");
        let ok = mode_after(125_000_000.0, Command::SetModeListen { rx: RxId::Main, mode });
        assert_eq!(ok, mode, "{mode:?}: SetModeListen must take it");
    }
}
