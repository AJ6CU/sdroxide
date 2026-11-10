//! **KEEP DIAL**: choosing SSTV leaves the dial where the operator tuned.
//!
//! Selecting SSTV moves a dial that is nowhere near a published SSTV frequency
//! onto the band's calling one (`conventional_dial.rs`). That is the right
//! default for somebody who has just picked a band, and the wrong one for an
//! operator decoding a picture heard off the list — a local net, a station
//! somewhere else in the band. `DigiConfig::sstv_keep_dial` switches the move
//! off, and only for SSTV.
//!
//! A file of its own, not a test in `conventional_dial.rs`: `SetDigiConfig`
//! makes the engine save, and an engine started afterwards loads what was
//! saved. Sharing a process with the tests that expect the move would let this
//! one's setting leak into theirs.

use std::time::Duration;

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DeviceCaps, DigiConfig, Mode, RadioEvent, RxId, Vfo};

const RATE: f64 = 2_400_000.0;
/// On 11 m and no part of any mode's convention.
const M11_NOT_SSTV: f64 = 27_100_000.0;
/// 20 m, well clear of 14.230.
const TWENTY_M: f64 = 14_000_000.0;
/// 20 m's FT8 dial, for the check that the switch is SSTV's alone.
const FT8_20M: f64 = 14_074_000.0;

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

/// `SetDigiConfig` saves, and `SDROXIDE_CONFIG_DIR` is process-global: without
/// this the test would write over the real station's settings.
fn isolate_config() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let root =
            std::env::temp_dir().join(format!("sdroxide-sstv-keep-dial-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };
    });
}

/// Run `cmds`, then return the dial from the last state the engine published.
fn dial_after(cmds: &[Command]) -> f64 {
    isolate_config();
    let mut h = start_engine(
        Box::new(MockSource { center: M11_NOT_SSTV }),
        caps(),
        EngineConfig { tx_ham_only: false, ..Default::default() },
    );
    let thread = h.thread.take();
    std::thread::sleep(Duration::from_millis(150));
    for c in cmds {
        h.cmd_tx.send(c.clone()).unwrap();
    }
    std::thread::sleep(Duration::from_millis(300));

    let mut last = None;
    while let Ok(ev) = h.event_rx.try_recv() {
        if let RadioEvent::State(s) = ev {
            last = Some(s.active_freq_hz());
        }
    }

    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    last.expect("the engine should publish state")
}

fn tune(hz: f64) -> Command {
    Command::SetVfo { vfo: Vfo::A, hz }
}

fn mode(m: Mode) -> Command {
    Command::SetMode { rx: RxId::Main, mode: m }
}

fn keep_dial(on: bool) -> Command {
    Command::SetDigiConfig(DigiConfig { sstv_keep_dial: on, ..DigiConfig::default() })
}

/// The request: with the switch on, choosing SSTV off the list leaves the dial
/// there — on 11 m and on an amateur band, and from the listener's screen too.
/// Then off again, the rescue is back, which is what proves the switch is what
/// held the dial rather than something else in this harness.
#[test]
fn keep_dial_leaves_sstv_where_it_was_tuned() {
    assert_eq!(dial_after(&[keep_dial(true), tune(M11_NOT_SSTV), mode(Mode::Sstv)]), M11_NOT_SSTV);
    assert_eq!(dial_after(&[keep_dial(true), tune(TWENTY_M), mode(Mode::Sstv)]), TWENTY_M);
    let listen = Command::SetModeListen { rx: RxId::Main, mode: Mode::Sstv };
    assert_eq!(dial_after(&[keep_dial(true), tune(M11_NOT_SSTV), listen]), M11_NOT_SSTV);

    assert_eq!(dial_after(&[keep_dial(false), tune(TWENTY_M), mode(Mode::Sstv)]), 14_230_000.0);

    // SSTV's switch only: FT8 is decoded in a 50 Hz window and a dial off its
    // frequency hears nothing, so it is still moved.
    assert_eq!(dial_after(&[keep_dial(true), tune(TWENTY_M), mode(Mode::Ft8)]), FT8_20M);
}
