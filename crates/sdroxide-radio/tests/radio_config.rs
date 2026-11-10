//! The engine owns `radio.json` and says what is in it.
//!
//! An SDR's own settings are not all gain stages: an RTL-SDR's AGC mode, its
//! ppm correction, its HF path and its bias tee have no `DeviceCaps` entry, and
//! the panel that drives them reads and writes this file. Where the radio is on
//! another machine that file is on *that* machine, so the engine is the only
//! thing in a position to read it, write it, and tell whoever is driving the
//! radio what it says. Everything a remote operator can do to an interface goes
//! through the two events asserted here.
//!
//! One test function on purpose: `SDROXIDE_CONFIG_DIR` is process-global, and
//! this one writes a real `radio.json` under it.

use std::time::{Duration, Instant};

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{
    Backend, Command, DeviceCaps, RadioConfig, RadioEvent, RtlSdrAgc, RtlSdrConfig, RtlSdrHfMode,
};

const RATE: f64 = 48_000.0;
const CENTER: f64 = 14_100_000.0;

struct Quiet;

impl IqSource for Quiet {
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
        let n = buf.len().min(256);
        buf[..n].fill(Complex32::new(0.0, 0.0));
        Ok(n)
    }
    fn describe(&self) -> String {
        "config rig".into()
    }
}

fn caps() -> DeviceCaps {
    DeviceCaps {
        driver: "bench".into(),
        label: "bench rig".into(),
        rx_channels: 1,
        sample_rates: vec![RATE],
        freq_ranges_rx: vec![(0.0, 60_000_000.0)],
        ..DeviceCaps::default()
    }
}

/// Wait for the next announced interface configuration.
fn next_radio_config(rx: &crossbeam_channel::Receiver<RadioEvent>) -> Option<RadioConfig> {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        if let Ok(RadioEvent::RadioConfig(c)) = rx.recv_timeout(Duration::from_millis(100)) {
            return Some(*c);
        }
    }
    None
}

/// A dongle configured well away from the defaults. Every field here would
/// still look plausible against `RadioConfig::default()` if the announcement
/// arrived empty, which is exactly why none of them is the default.
fn configured_dongle() -> RadioConfig {
    RadioConfig {
        backend: Backend::RtlSdr,
        rtlsdr: RtlSdrConfig {
            serial: Some("00000042".into()),
            sample_rate_hz: 1_536_000.0,
            ppm: -11,
            tuner_gain_db: 22.5,
            agc: RtlSdrAgc::Manual,
            hf_mode: RtlSdrHfMode::Auto,
            bias_tee: false,
            iq_correction: true,
            ..RtlSdrConfig::default()
        },
        ..RadioConfig::default()
    }
}

#[test]
fn the_interface_configuration_is_announced_and_can_be_written_through_the_engine() {
    let root = std::env::temp_dir().join(format!("sdroxide-radio-config-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    // SAFETY: set before anything reads it, and this binary holds one test.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };

    // Seeded through the store the engine reads it with: a hand-written file
    // could pass this test against a shape the engine cannot load.
    let store = sdroxide_config::Store::station();
    let stored = configured_dongle();
    store.save_radio_config(&stored).expect("seed radio.json");

    let mut h = start_engine(Box::new(Quiet), caps(), EngineConfig::default());
    let thread = h.thread.take();

    // Announced at startup, without being asked. This is what seeds the Radio
    // tab on a client that has no copy of the file — without it the panel opens
    // on defaults and writes them back over the operator's the moment anything
    // is touched.
    let announced = next_radio_config(&h.event_rx).expect("no config announced at startup");
    // One field is the engine's own doing: a `radio.json` written before the
    // per-band transmit list existed carries none, and the engine hands the
    // amateur bands over **unconfirmed** rather than leaving a radio that
    // cannot transmit at all (#358). Everything else has to come back exactly
    // as it was stored.
    assert!(announced.tx_band_enable.is_some(), "the amateur bands are carried over");
    assert!(!announced.tx_bands_confirmed, "and none of them is confirmed by that");
    let mut same = announced.clone();
    same.tx_band_enable = stored.tx_band_enable.clone();
    same.tx_bands_confirmed = stored.tx_bands_confirmed;
    assert_eq!(same, stored, "the startup announcement did not carry the stored config");
    // The settings with no `DeviceCaps` entry, which are the ones a remote
    // operator could not reach at all before this lane existed.
    assert_eq!(announced.rtlsdr.agc, RtlSdrAgc::Manual);
    assert_eq!(announced.rtlsdr.ppm, -11);
    assert_eq!(announced.rtlsdr.tuner_gain_db, 22.5);

    // An edit is written here, where the radio is, and echoed back. `reopen`
    // stays false: that is the half that persists, and the knob itself has
    // already reached the hardware through `SetGain` as it moved.
    let mut edited = stored.clone();
    edited.rtlsdr.tuner_gain_db = 41.2;
    edited.rtlsdr.agc = RtlSdrAgc::Tuner;
    edited.rtlsdr.bias_tee = true;
    h.cmd_tx
        .send(Command::SetRadioConfig { cfg: Box::new(edited.clone()), reopen: false })
        .unwrap();

    let echoed = next_radio_config(&h.event_rx).expect("no config echoed after the edit");
    assert_eq!(echoed, edited, "the echo did not carry the edit");

    // The echo is read back out of the store rather than bounced off the
    // command, so it is most of the proof that the file was written — but read
    // the file too. "Announced" and "persisted" are the two halves that have to
    // agree for the next start to come up on these settings.
    assert_eq!(
        store.load_radio_config(),
        edited,
        "radio.json on the engine's machine was not updated"
    );

    drop(h);
    let _ = thread.map(|t| t.join());
    let _ = std::fs::remove_dir_all(&root);
}
