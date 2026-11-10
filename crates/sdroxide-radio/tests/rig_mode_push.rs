//! A radio that is the rig as well as the front end follows the mode control.
//!
//! An Icom on its LAN port handing us the 12 kHz IF is not in audio mode — the
//! stream is ordinary complex baseband and sdroxide demodulates it — so for as
//! long as the engine keyed "command the rig's mode" on `audio_mode` alone, the
//! mode control moved nothing on the radio. Mode travelled rig→app, on the
//! source's own poll, and never the other way: switching USB↔LSB in sdroxide
//! left the radio where it was.
//!
//! What holds the fix is [`IqSource::commands_rx_mode`]: the operator's mode has
//! to reach such a radio, *without* the session's mode being imposed the moment
//! the connection opens — that backend deliberately adopts the dial and the mode
//! the transceiver is already sitting on.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DeviceCaps, Mode, RxId};

const RATE: f64 = 24_000.0;
const DIAL: f64 = 7_074_000.0;

/// A front end that demodulates nowhere near here — the engine does that — but
/// whose mode is still the radio's own, like an Icom's 12 kHz IF.
struct MockRig {
    center: f64,
    commands_mode: bool,
    modes: Arc<Mutex<Vec<Mode>>>,
}

impl IqSource for MockRig {
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
        let n = buf.len().min(512);
        buf[..n].fill(Complex32::new(0.0, 0.0));
        Ok(n)
    }
    fn describe(&self) -> String {
        "mock icom over LAN".into()
    }
    fn commands_rx_mode(&self) -> bool {
        self.commands_mode
    }
    fn set_control_mode(&mut self, mode: Mode) -> Result<()> {
        self.modes.lock().unwrap().push(mode);
        Ok(())
    }
}

/// Not `audio_mode`: the 12 kHz IF is complex baseband and takes the ordinary
/// DDC/demod path, which is exactly the arrangement the old gate missed.
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

/// Start on `DIAL` in USB, then run `cmds`. Returns every mode the source was
/// commanded, in order.
fn modes_commanded(commands_mode: bool, cmds: &[Command]) -> Vec<Mode> {
    let modes = Arc::new(Mutex::new(Vec::new()));
    let mut h = start_engine(
        Box::new(MockRig { center: DIAL, commands_mode, modes: Arc::clone(&modes) }),
        caps(),
        EngineConfig { tx_ham_only: false, ..Default::default() },
    );
    let thread = h.thread.take();

    std::thread::sleep(Duration::from_millis(150));
    h.cmd_tx.send(Command::SetMode { rx: RxId::Main, mode: Mode::Usb }).unwrap();
    std::thread::sleep(Duration::from_millis(150));
    modes.lock().unwrap().clear();

    for c in cmds {
        h.cmd_tx.send(c.clone()).unwrap();
    }
    std::thread::sleep(Duration::from_millis(300));

    let out = modes.lock().unwrap().clone();
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    out
}

#[test]
fn switching_sideband_reaches_a_radio_that_owns_its_mode() {
    let seen = modes_commanded(true, &[Command::SetMode { rx: RxId::Main, mode: Mode::Lsb }]);
    assert!(seen.contains(&Mode::Lsb), "the radio was never told about LSB: {seen:?}");
}

/// The bug as reported: without the flag the mode control moves nothing on the
/// radio, because the stream is not audio mode.
#[test]
fn a_plain_sdr_is_still_left_alone() {
    let seen = modes_commanded(false, &[Command::SetMode { rx: RxId::Main, mode: Mode::Lsb }]);
    assert!(seen.is_empty(), "an SDR has no mode of its own to command: {seen:?}");
}

/// Opening the connection must not rearrange somebody's transceiver: this
/// backend adopts the mode the radio is already in, so nothing is asserted
/// until the operator actually changes something.
#[test]
fn opening_the_connection_imposes_nothing() {
    let modes = Arc::new(Mutex::new(Vec::new()));
    let mut h = start_engine(
        Box::new(MockRig { center: DIAL, commands_mode: true, modes: Arc::clone(&modes) }),
        caps(),
        EngineConfig { tx_ham_only: false, ..Default::default() },
    );
    let thread = h.thread.take();
    std::thread::sleep(Duration::from_millis(300));
    let seen = modes.lock().unwrap().clone();
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    assert!(seen.is_empty(), "the session imposed a mode at connect: {seen:?}");
}
