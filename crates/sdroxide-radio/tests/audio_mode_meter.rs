//! The S-meter on a front end that hands us already-demodulated audio — a CAT
//! rig on a sound card.
//!
//! There is no `RxChain` on that path (no DDC, no demodulator), and the meter
//! used to be read off the chain alone: on a CAT rig the engine published no
//! meter at all, so the S-meter sat at its stop for the whole session while the
//! waterfall and the dial worked perfectly. These hold the two ways a reading
//! can now be had — the rig's own meter, and failing that the level of the
//! audio it is sending.

use std::time::Duration;

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{DeviceCaps, RadioEvent};

const RATE: f64 = 48_000.0;
/// Amplitude of the "audio" the stand-in rig sends: −20 dBFS as a mean square.
const AMPLITUDE: f32 = 0.1;
const AUDIO_DBFS: f32 = -20.0;
/// A deliberately non-zero dBFS→dBm calibration, so a reading that has had it
/// applied is distinguishable from one that has not.
const CAL_OFFSET_DB: f32 = 7.0;

/// A CAT rig on a sound card: real audio in the I component, no IQ. `rig_dbm`
/// is what its own S-meter reports over CAT, or `None` for a rig (or a family)
/// that has no such read.
struct DemodSource {
    center: f64,
    rig_dbm: Option<f32>,
}

impl IqSource for DemodSource {
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
        // A constant-amplitude signal: mean square is exactly `AMPLITUDE²`,
        // whatever block length the engine asks for.
        buf[..n].fill(Complex32::new(AMPLITUDE, 0.0));
        Ok(n)
    }
    fn describe(&self) -> String {
        "mock demod-audio rig".into()
    }
    fn display_bandwidth(&self) -> Option<f64> {
        Some(4000.0)
    }
    fn rx_signal_dbm(&mut self) -> Option<f32> {
        self.rig_dbm
    }
}

fn caps() -> DeviceCaps {
    DeviceCaps {
        driver: "cat".into(),
        label: "mock CAT rig".into(),
        rx_channels: 1,
        tx_channels: 1,
        audio_mode: true,
        freq_ranges_rx: vec![(10_000.0, 148_000_000.0)],
        freq_ranges_tx: vec![(1_800_000.0, 54_000_000.0)],
        ..DeviceCaps::default()
    }
}

/// Run an engine on the stand-in rig and return the last S-meter reading it
/// published, or `None` if it published none.
fn last_s_dbm(rig_dbm: Option<f32>) -> Option<f32> {
    let src = DemodSource { center: 14_074_000.0, rig_dbm };
    let mut h = start_engine(
        Box::new(src),
        caps(),
        EngineConfig { cal_offset_db: CAL_OFFSET_DB, ..Default::default() },
    );
    let thread = h.thread.take();

    let mut last = None;
    let deadline = std::time::Instant::now() + Duration::from_secs(2);
    while std::time::Instant::now() < deadline {
        while let Ok(ev) = h.event_rx.try_recv() {
            if let RadioEvent::Meters(m) = ev {
                last = Some(m.s_dbm);
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }

    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    last
}

#[test]
fn a_rig_with_no_meter_read_is_metered_from_its_audio() {
    let s_dbm = last_s_dbm(None).expect("a demod-audio front end must still publish an S-meter");
    // The audio's own level, with the station's calibration on top. The one-pole
    // that smooths it has long since settled over two seconds of blocks.
    let want = AUDIO_DBFS + CAL_OFFSET_DB;
    assert!(
        (s_dbm - want).abs() < 1.0,
        "expected about {want:.1} dBm from {AUDIO_DBFS:.0} dBFS of audio, got {s_dbm:.1}"
    );
}

#[test]
fn a_rig_that_reports_its_own_meter_is_believed() {
    // The rig's meter is already in dBm — calibrated by whoever built the
    // radio — so it is passed through as it stands, with neither the audio
    // level nor the dBFS→dBm offset applied to it.
    let s_dbm = last_s_dbm(Some(-73.0)).expect("the rig's S-meter must reach the meter");
    assert_eq!(s_dbm, -73.0);
}
