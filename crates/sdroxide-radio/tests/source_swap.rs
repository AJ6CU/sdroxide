//! A front-end swap must recompute every rate-derived figure in the state,
//! not just `sample_rate`.
//!
//! Issue #263: a radio switched off at exit comes back as the 48 kHz stand-in
//! (`NullSource`). Linking the real device swaps it for the SDR, whose state
//! took a new `sample_rate` and `decimation` but carried the stand-in's
//! `usable_hz` over untouched — so the display span came up at 48 kHz and the
//! zoom snapped back to it on every zoom-out.

use std::time::{Duration, Instant};

use sdroxide_radio::{
    start_engine, Complex32, EngineConfig, EngineHandles, EngineSwap, IqSource, Result,
};
use sdroxide_types::{DeviceCaps, RadioEvent, RadioState};

/// The stand-in a switched-off radio runs: the null source's rate, no samples.
const STANDBY_RATE: f64 = 48_000.0;
/// The real front end the operator links.
const REAL_RATE: f64 = 3_000_000.0;
/// Its filter narrows the stream (the Airspy HF+ figure, reused so the
/// expected usable span is not simply the rate and a stale carry-over cannot
/// pass for a recompute).
const REAL_FRAC: f64 = 0.86;
const CENTER: f64 = 6_040_000.0;

struct StandIn {
    center_hz: f64,
}

impl IqSource for StandIn {
    fn sample_rate(&self) -> f64 {
        STANDBY_RATE
    }
    fn center_hz(&self) -> f64 {
        self.center_hz
    }
    fn set_center_hz(&mut self, hz: f64) -> Result<()> {
        self.center_hz = hz;
        Ok(())
    }
    fn read(&mut self, _buf: &mut [Complex32]) -> Result<usize> {
        std::thread::sleep(Duration::from_millis(2));
        Ok(0)
    }
    fn describe(&self) -> String {
        "switched off".into()
    }
    fn needs_reopen(&self) -> bool {
        true
    }
}

struct Real {
    center_hz: f64,
}

impl IqSource for Real {
    fn sample_rate(&self) -> f64 {
        REAL_RATE
    }
    fn center_hz(&self) -> f64 {
        self.center_hz
    }
    fn set_center_hz(&mut self, hz: f64) -> Result<()> {
        self.center_hz = hz;
        Ok(())
    }
    fn read(&mut self, _buf: &mut [Complex32]) -> Result<usize> {
        std::thread::sleep(Duration::from_millis(2));
        Ok(0)
    }
    fn describe(&self) -> String {
        "real SDR".into()
    }
    fn usable_frac(&self) -> f64 {
        REAL_FRAC
    }
}

fn caps(rate: f64) -> DeviceCaps {
    DeviceCaps {
        driver: "test".into(),
        label: "test".into(),
        rx_channels: 1,
        sample_rates: vec![rate],
        freq_ranges_rx: vec![(1_000_000.0, 60_000_000.0)],
        ..DeviceCaps::default()
    }
}

/// Drain the event queue and return the newest state snapshot seen.
fn latest_state(h: &EngineHandles) -> Option<RadioState> {
    let mut last = None;
    while let Ok(ev) = h.event_rx.try_recv() {
        if let RadioEvent::State(s) = ev {
            last = Some(s);
        }
    }
    last
}

/// Poll until a state satisfying `cond` arrives, then hand it back.
fn wait_until(h: &EngineHandles, secs: f64, cond: impl Fn(&RadioState) -> bool) -> RadioState {
    let deadline = Instant::now() + Duration::from_secs_f64(secs);
    while Instant::now() < deadline {
        if let Some(s) = latest_state(h) {
            if cond(&s) {
                return s;
            }
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    panic!("no matching state within {secs}s");
}

#[test]
fn a_swap_recomputes_the_usable_span_instead_of_carrying_the_stand_in_s() {
    let h = start_engine(
        Box::new(StandIn { center_hz: CENTER }),
        caps(STANDBY_RATE),
        EngineConfig {
            remember_session: false,
            reopen: Some(Box::new(|center| {
                Ok((Box::new(Real { center_hz: center }) as Box<dyn IqSource>, caps(REAL_RATE)))
            })),
            ..Default::default()
        },
    );

    // The stand-in's rate is the whole state until the device is linked.
    let initial = wait_until(&h, 2.0, |s| (s.sample_rate - STANDBY_RATE).abs() < 0.5);
    assert!(
        (initial.usable_hz - STANDBY_RATE).abs() < 0.5,
        "the stand-in opens at its own usable span, got {}",
        initial.usable_hz
    );

    // Link the real front end.
    h.swap_tx.send(EngineSwap::ReopenSource).unwrap();

    // The state now describes the real front end…
    let after = wait_until(&h, 3.0, |s| (s.sample_rate - REAL_RATE).abs() < 0.5);
    // …and its usable span is the new source's own, not the 48 kHz the
    // stand-in left behind.
    let expected = REAL_RATE * REAL_FRAC;
    assert!(
        (after.usable_hz - expected).abs() < 0.5,
        "usable span follows the new source ({} Hz), not the stale stand-in ({} Hz)",
        expected,
        after.usable_hz
    );
}
