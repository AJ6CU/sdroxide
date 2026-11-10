//! Both VFOs have to be where the operator left them at the next start.
//!
//! Remembering only the dial that happened to be in use is what a radio does
//! when it has one VFO; this one has two, and they are used as a pair — B
//! holding the DX's transmit frequency of a split, or the net the operator
//! swaps back to. A start that collapses B onto A throws away half of that
//! setup, and does it silently: the readout looks right, and the mistake only
//! turns up on the first swap.
//!
//! One test function on purpose: `SDROXIDE_CONFIG_DIR` is process-global, and
//! this one writes a real `session.json` under it.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DeviceCaps, RadioEvent, Vfo};

const RATE: f64 = 192_000.0;
const VFO_A: f64 = 14_200_000.0;
const VFO_B: f64 = 7_100_000.0;

/// A front end that tunes, so the dial the engine follows is the one the
/// hardware is actually on — the same thing the next start is opened on.
struct Rig {
    center: Arc<Mutex<f64>>,
}

impl IqSource for Rig {
    fn sample_rate(&self) -> f64 {
        RATE
    }
    fn center_hz(&self) -> f64 {
        *self.center.lock().unwrap()
    }
    fn set_center_hz(&mut self, hz: f64) -> Result<()> {
        *self.center.lock().unwrap() = hz;
        Ok(())
    }
    fn read(&mut self, buf: &mut [Complex32]) -> Result<usize> {
        std::thread::sleep(Duration::from_millis(5));
        let n = buf.len().min(256);
        buf[..n].fill(Complex32::new(0.0, 0.0));
        Ok(n)
    }
    fn describe(&self) -> String {
        "vfo rig".into()
    }
}

fn caps() -> DeviceCaps {
    DeviceCaps {
        driver: "bench".into(),
        label: "bench rig".into(),
        rx_channels: 1,
        sample_rates: vec![RATE],
        freq_ranges_rx: vec![(1_000_000.0, 60_000_000.0)],
        ..DeviceCaps::default()
    }
}

/// An engine brought up the way the binary brings one up: on the dial the
/// remembered session was left on, whichever VFO that was.
fn restart() -> sdroxide_radio::EngineHandles {
    let center = sdroxide_config::load_session().active_dial_hz();
    let cfg = EngineConfig { remember_session: true, ..Default::default() };
    start_engine(Box::new(Rig { center: Arc::new(Mutex::new(center)) }), caps(), cfg)
}

fn wait_for_state(
    rx: &crossbeam_channel::Receiver<RadioEvent>,
    want: impl Fn(&sdroxide_types::RadioState) -> bool,
) -> bool {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        match rx.recv_timeout(Duration::from_millis(100)) {
            Ok(RadioEvent::State(s)) if want(&s) => return true,
            Ok(_) | Err(_) => {}
        }
    }
    false
}

#[test]
fn both_vfos_and_the_active_one_survive_a_restart() {
    let root = std::env::temp_dir().join(format!("sdroxide-vfo-memory-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };

    // ---- Set the pair up and leave the radio listening on B ----
    let mut h = restart();
    let thread = h.thread.take();
    h.cmd_tx.send(Command::SetVfo { vfo: Vfo::A, hz: VFO_A }).unwrap();
    h.cmd_tx.send(Command::SetVfo { vfo: Vfo::B, hz: VFO_B }).unwrap();
    h.cmd_tx.send(Command::SelectVfo(Vfo::B)).unwrap();
    assert!(
        wait_for_state(&h.event_rx, |s| s.vfo_a_hz == VFO_A
            && s.vfo_b_hz == VFO_B
            && s.active_vfo == Vfo::B),
        "the two dials must reach the published state first"
    );
    // Quitting is what writes the session; joining makes sure the file is on
    // disk before it is read back.
    drop(h);
    let _ = thread.map(|t| t.join());

    let saved = sdroxide_config::load_session();
    assert_eq!(saved.freq_hz, VFO_A, "VFO A goes to session.json as VFO A");
    assert_eq!(saved.vfo_b_hz, Some(VFO_B), "and B as B");
    assert_eq!(saved.active_vfo, Vfo::B);
    assert_eq!(saved.active_dial_hz(), VFO_B, "which is the dial the next start opens on");

    // ---- The next start ----
    let mut h = restart();
    let thread = h.thread.take();
    assert!(
        wait_for_state(&h.event_rx, |s| s.vfo_a_hz == VFO_A
            && s.vfo_b_hz == VFO_B
            && s.active_vfo == Vfo::B),
        "the second start must come up on both dials, still on B"
    );
    // Swapping back has to land on A's remembered frequency rather than on the
    // dial the receiver was opened at — the point of remembering the pair.
    h.cmd_tx.send(Command::SelectVfo(Vfo::A)).unwrap();
    assert!(
        wait_for_state(&h.event_rx, |s| s.active_vfo == Vfo::A && s.vfo_a_hz == VFO_A),
        "VFO A has to still be on the frequency it was left on"
    );
    drop(h);
    let _ = thread.map(|t| t.join());

    // ---- A hand-edited file with a nonsense B ----
    // A is what the receiver opens on and is checked; B costs only itself.
    let mut edited = sdroxide_config::load_session();
    edited.vfo_b_hz = Some(-1.0);
    sdroxide_config::save_session(&edited).unwrap();
    let mut h = restart();
    let thread = h.thread.take();
    assert!(
        wait_for_state(&h.event_rx, |s| s.vfo_a_hz == VFO_A && s.vfo_b_hz == VFO_A),
        "an unusable B falls back to A rather than taking the session down with it"
    );
    drop(h);
    let _ = thread.map(|t| t.join());

    unsafe { std::env::remove_var("SDROXIDE_CONFIG_DIR") };
    let _ = std::fs::remove_dir_all(&root);
}
