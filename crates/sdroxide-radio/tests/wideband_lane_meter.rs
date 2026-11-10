//! The S-meter in a wideband lane (DAB, ADS-B, AIS, VDL2).
//!
//! These modes run no demodulator, and the receive chain's passband power is
//! what the meter was built on — so with no chain reading, no meter was
//! published at all. The UI kept the last reading of the mode before, and a
//! tester on a DAB ensemble saw S9+5 that never moved, whatever the gain or the
//! signal did. The meter now reads the lane's bandwidth off the spectrum.
//!
//! As in `gain_referred_meter`, every assertion is a difference between runs:
//! the absolute figure depends on the FFT's window and is not the point.

use std::time::Duration;

use sdroxide_radio::{AudioParams, Complex32, EngineConfig, IqSource, Result, rtrb, start_engine};
use sdroxide_types::{Command, DeviceCaps, Mode, RadioEvent, RxId, Vfo};

const RATE: f64 = 2_048_000.0;
/// Block 8B, where the report came from.
const CENTER: f64 = 197_648_000.0;

/// A front end delivering a constant carrier on its centre at `dbfs`.
struct Rig {
    dbfs: f32,
}

impl IqSource for Rig {
    fn sample_rate(&self) -> f64 {
        RATE
    }
    fn center_hz(&self) -> f64 {
        CENTER
    }
    fn set_center_hz(&mut self, _hz: f64) -> Result<()> {
        Ok(())
    }
    fn read(&mut self, buf: &mut [Complex32]) -> Result<usize> {
        std::thread::sleep(Duration::from_millis(5));
        let n = buf.len().min(8192);
        buf[..n].fill(Complex32::new(10f32.powf(self.dbfs / 20.0), 0.0));
        Ok(n)
    }
    fn describe(&self) -> String {
        "mock Band III front end".into()
    }
}

fn caps() -> DeviceCaps {
    DeviceCaps {
        driver: "bench".into(),
        label: "bench front end".into(),
        rx_channels: 1,
        sample_rates: vec![RATE],
        freq_ranges_rx: vec![(70_000_000.0, 6_000_000_000.0)],
        ..DeviceCaps::default()
    }
}

/// The last S-meter reading published in DAB, or `None` when there was none.
fn dab_s_dbm(dbfs: f32) -> Option<f32> {
    let (producer, mut consumer) = rtrb::RingBuffer::<f32>::new(1 << 16);
    let mut h = start_engine(
        Box::new(Rig { dbfs }),
        caps(),
        EngineConfig {
            remember_session: false,
            audio: Some(AudioParams { producer, out_rate: 48_000.0 }),
            ..Default::default()
        },
    );
    let thread = h.thread.take();
    h.cmd_tx.send(Command::SetVfo { vfo: Vfo::A, hz: CENTER }).unwrap();
    h.cmd_tx.send(Command::SetModeListen { rx: RxId::Main, mode: Mode::Dab }).unwrap();

    // Only readings taken once the mode is DAB count: the one the meter used
    // to freeze on came from the mode before it.
    let mut in_dab = false;
    let mut last = None;
    let deadline = std::time::Instant::now() + Duration::from_secs(2);
    while std::time::Instant::now() < deadline {
        while let Ok(ev) = h.event_rx.try_recv() {
            match ev {
                RadioEvent::State(s) => in_dab = s.rx[0].mode == Mode::Dab,
                RadioEvent::Meters(m) if in_dab => last = Some(m.s_dbm),
                _ => {}
            }
        }
        while consumer.pop().is_ok() {}
        std::thread::sleep(Duration::from_millis(20));
    }

    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    last
}

/// Kevin's report: in DAB the meter sat on S9+5 and did not move.
#[test]
fn a_dab_ensemble_still_has_a_meter() {
    assert!(dab_s_dbm(-60.0).is_some(), "no S-meter reading was published in DAB");
}

/// And it is a meter of the signal: 10 dB more at the antenna reads 10 dB more.
#[test]
fn a_stronger_ensemble_reads_stronger() {
    let weak = dab_s_dbm(-60.0).expect("a reading in DAB");
    let strong = dab_s_dbm(-50.0).expect("a reading in DAB");
    let rise = strong - weak;
    assert!(
        (rise - 10.0).abs() < 1.5,
        "a 10 dB stronger ensemble moved the meter by {rise:.1} dB ({weak:.1} → {strong:.1})"
    );
}
