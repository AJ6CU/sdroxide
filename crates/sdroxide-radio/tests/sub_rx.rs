//! The sub receiver's frequency. It used to be pinned to the inactive VFO,
//! which meant it could not be aimed at all without moving a VFO — these tests
//! hold it to being a receiver of its own: it stays where it is put, it cannot
//! be sent outside the IQ the hardware is actually delivering, and a band
//! change re-parks it somewhere it can still hear.

use std::time::Duration;

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Band, Command, DeviceCaps, RadioEvent, RadioState, Vfo};

const RATE: f64 = 2_400_000.0;
const START_CENTER: f64 = 14_200_000.0;

/// A receive-only stand-in that hands back silence at the paced rate and
/// follows retunes, so `center_hz` moves with a band change the way real
/// hardware would.
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

/// Run `cmds` through the engine and return the last state it published. The
/// engine emits a full snapshot on every change, so the final one is the
/// settled result of the whole sequence.
fn run(cmds: &[Command]) -> RadioState {
    let mut h = start_engine(
        Box::new(MockSource { center: START_CENTER }),
        caps(),
        EngineConfig { tx_ham_only: false, ..Default::default() },
    );
    let thread = h.thread.take();

    std::thread::sleep(Duration::from_millis(150));
    for c in cmds {
        h.cmd_tx.send(c.clone()).unwrap();
    }

    // Give the commands time to land, then drain everything published.
    let mut last = None;
    let deadline = std::time::Instant::now() + Duration::from_secs(2);
    while std::time::Instant::now() < deadline {
        while let Ok(ev) = h.event_rx.try_recv() {
            if let RadioEvent::State(s) = ev {
                last = Some(s);
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }

    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    last.expect("the engine should publish at least one state snapshot")
}

#[test]
fn sub_starts_on_the_inactive_vfo() {
    // Switching the sub on with no frequency of its own lands it where it
    // always used to live, so the control means the same thing it did before.
    let s = run(&[Command::SetVfo { vfo: Vfo::B, hz: 14_250_000.0 }, Command::SetSubRx(true)]);
    assert_eq!(s.sub_rx_hz, 14_250_000.0);
}

#[test]
fn sub_stays_put_when_the_vfos_move() {
    // The point of the whole change: tuning the dial, swapping A/B, or setting
    // the far VFO must not drag the second receiver along with it.
    let s = run(&[
        Command::SetVfo { vfo: Vfo::B, hz: 14_250_000.0 },
        Command::SetSubRx(true),
        Command::SetSubRxFreq(14_310_000.0),
        Command::SetVfo { vfo: Vfo::A, hz: 14_050_000.0 },
        Command::SetVfo { vfo: Vfo::B, hz: 14_090_000.0 },
        Command::SwapVfos,
    ]);
    assert_eq!(s.sub_rx_hz, 14_310_000.0, "the sub should not follow the VFOs");
}

#[test]
fn sub_cannot_leave_the_device_passband() {
    // Both receivers are DDCs on one IQ stream: asking for a frequency the
    // hardware is not delivering has to clamp, not silently point the DDC at
    // nothing.
    let (lo, hi) = (START_CENTER - RATE / 2.0, START_CENTER + RATE / 2.0);
    let s = run(&[Command::SetSubRx(true), Command::SetSubRxFreq(28_000_000.0)]);
    assert_eq!(s.sub_rx_hz, hi);

    let s = run(&[Command::SetSubRx(true), Command::SetSubRxFreq(1_000_000.0)]);
    assert_eq!(s.sub_rx_hz, lo);
}

#[test]
fn a_band_change_reparks_the_sub() {
    // A band change moves the hardware out from under the sub. Left alone it
    // would be tuned to a frequency the receiver no longer covers, deaf with
    // no indication why — so it gets re-parked inside the new window.
    let s = run(&[
        Command::SetSubRx(true),
        Command::SetSubRxFreq(14_310_000.0),
        Command::SetBand(Band::M40),
    ]);
    let (lo, hi) = (s.center_hz - s.sample_rate / 2.0, s.center_hz + s.sample_rate / 2.0);
    assert!(
        (lo..=hi).contains(&s.sub_rx_hz),
        "sub at {} should be inside the new passband {lo}..={hi}",
        s.sub_rx_hz
    );
}
