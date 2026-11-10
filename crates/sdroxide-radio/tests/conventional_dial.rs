//! Choosing a digital mode that has a published calling frequency moves the
//! dial onto it — and must not when the operator is already listening there.
//!
//! The rule has been here for the slotted modes and WSPR for a long time. SSTV
//! was missing from it, which is not a gap anyone noticed until an operator
//! reported the obvious thing: pick a band, pick SSTV, and be somewhere
//! useless, with the only way out being the **FREQ** chip in the SSTV panel.
//! SSTV is a one-frequency-per-band mode by the same argument as every other
//! mode this covers, and its table (`SSTV_DIALS`) is region-tagged and
//! published, so it belongs in the same place.
//!
//! What makes this safe enough to do without being asked is the check *inside*
//! `conventional_dial_for`: a dial already sitting on one of the mode's own
//! frequencies is never moved. So the only case this rescues is a dial that is
//! nowhere the mode would put anyone.

use std::time::Duration;

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DeviceCaps, Mode, RadioEvent, RxId, Vfo};

const RATE: f64 = 2_400_000.0;
/// 11 m, the fork's own band and the one this was reported on.
const M11: f64 = 27_700_000.0;
/// A frequency on 11 m that is no part of any mode's convention.
const M11_NOT_SSTV: f64 = 27_100_000.0;
/// 20 m, where SSTV is worked at 14.230 MHz in USB territory.
const TWENTY_M: f64 = 14_000_000.0;
const SSTV_20M: f64 = 14_230_000.0;
/// 20 m, where Olivia's calling centre is 14.1075 MHz.
const OLIVIA_20M: f64 = 14_107_500.0;
/// 11 m, which has no Olivia convention — so the dial must be left alone there.
const M11_NOT_OLIVIA: f64 = 27_100_000.0;

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

/// Point the engine at a scratch config directory.
///
/// The engine loads its `DigiConfig` from the station's store, and
/// `SDROXIDE_CONFIG_DIR` is process-global — without this the test would read
/// (and, on a mode change, write) the real station's settings, and an operator
/// who had switched **KEEP DIAL** on would find SSTV never lands on the calling
/// frequency and the tests red on their machine alone. Every other radio test
/// isolates itself the same way; this one did not.
fn isolate_config() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let root =
            std::env::temp_dir().join(format!("sdroxide-conventional-dial-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };
    });
}

/// Run `cmds`, then return the dial from the last state the engine published.
fn dial_after(cmds: &[Command]) -> f64 {
    isolate_config();
    let mut h = start_engine(
        Box::new(MockSource { center: M11 }),
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

fn sstv() -> Command {
    Command::SetMode { rx: RxId::Main, mode: Mode::Sstv }
}

/// The report: on 11 m, choosing SSTV took the operator to 27.700 rather than
/// leaving them where the last mode had put the dial.
#[test]
fn choosing_sstv_lands_on_the_bands_sstv_frequency() {
    assert_eq!(dial_after(&[tune(M11_NOT_SSTV), sstv()]), M11);
    // And the same rule on an amateur band, where it is 14.230.
    assert_eq!(dial_after(&[tune(TWENTY_M), sstv()]), SSTV_20M);
}

/// The half of the rule that makes doing this without being asked acceptable:
/// a dial the operator put where they want it is left exactly there.
#[test]
fn a_dial_already_on_an_sstv_frequency_is_not_moved() {
    // 27.255 is a published in-band picture channel on 11 m, not the calling
    // one — an operator who chose it must keep it.
    let in_band = 27_255_000.0;
    assert_eq!(dial_after(&[tune(in_band), sstv()]), in_band);
    assert_eq!(dial_after(&[tune(SSTV_20M), sstv()]), SSTV_20M);
}

/// **The detune, reported** (fork discussion #7, Kevin). An operator receiving
/// an SSTV picture a little below the band's calling frequency selected SSTV to
/// decode it; the rule moved the dial onto the published frequency and left
/// their picture off the passband centre — "completely shifted to the left",
/// and "audio present · no SSTV header yet". A dial already inside the mode's
/// own activity is the operator being where the station is.
///
/// 20 m's frequency is used here because it is the one entry in the table with
/// no region mask, so the case does not depend on the station's region. The
/// reported one was 80 m, 1.5 kHz below 3.7300 — `3_728_500.0` — which the same
/// rule holds.
#[test]
fn a_dial_inside_the_sstv_segment_is_left_alone() {
    for off in [-1_500.0_f64, -2_900.0, 2_000.0] {
        let listening = SSTV_20M + off;
        assert_eq!(dial_after(&[tune(listening), sstv()]), listening, "{off} Hz off");
    }
    // Outside the segment the rule still rescues, which is what it is for.
    assert_eq!(dial_after(&[tune(SSTV_20M + 40_000.0), sstv()]), SSTV_20M);
}

/// Re-selecting the mode already in force is not a change, so it must not move
/// the dial either — otherwise standing in SSTV and pressing the chip again
/// would retune under the operator's feet.
#[test]
fn reselecting_sstv_does_not_move_the_dial() {
    let off = M11_NOT_SSTV;
    // The first selection rescues the dial; the second changes nothing, so the
    // value is whatever the first left.
    let once = dial_after(&[tune(off), sstv()]);
    let twice = dial_after(&[tune(off), sstv(), sstv()]);
    assert_eq!(once, twice);
}

/// The listener's screen is the same rule: `SetModeListen` skips the band
/// check but must still land on the mode's frequency, or a listener exploring
/// SSTV on 11 m gets the same dead dial the OPERATE tab used to hand them.
#[test]
fn the_listen_path_lands_on_the_sstv_frequency_too() {
    let listen = Command::SetModeListen { rx: RxId::Main, mode: Mode::Sstv };
    assert_eq!(dial_after(&[tune(M11_NOT_SSTV), listen]), M11);
}

/// A band with no published convention for the mode must be left alone, or the
/// rule would drag the dial across the world to reach a frequency that is not
/// this band's. 6 m is the case: analog SSTV's table stops at 28.69 MHz, so
/// there is nothing on 6 m to move to.
#[test]
fn a_band_with_no_sstv_frequency_is_left_alone() {
    let six_m = 50_000_000.0;
    let after = dial_after(&[tune(six_m), sstv()]);
    assert_eq!(after, six_m, "no SSTV frequency on this band, so the dial should not move");
}

fn olivia() -> Command {
    Command::SetMode { rx: RxId::Main, mode: Mode::Olivia }
}

/// Choosing Olivia lands the dial on the band's Olivia calling centre, the
/// same rule SSTV and the slotted modes already follow. 20 m's is 14.1075.
#[test]
fn choosing_olivia_lands_on_the_bands_calling_centre() {
    assert_eq!(dial_after(&[tune(TWENTY_M), olivia()]), OLIVIA_20M);
}

/// The operator's own choice is respected: a dial already on one of Olivia's
/// per-band centres is not moved (14.073 and 14.1075 are both on 20 m).
#[test]
fn a_dial_already_on_an_olivia_frequency_is_not_moved() {
    let plain_centre = 14_073_000.0;
    assert_eq!(dial_after(&[tune(plain_centre), olivia()]), plain_centre);
    assert_eq!(dial_after(&[tune(OLIVIA_20M), olivia()]), OLIVIA_20M);
}

/// A band with no Olivia convention is left alone: Olivia's table has no 11 m
/// entry, so there is nothing to rescue a dial there to.
#[test]
fn a_band_with_no_olivia_frequency_is_left_alone() {
    assert_eq!(dial_after(&[tune(M11_NOT_OLIVIA), olivia()]), M11_NOT_OLIVIA);
}
