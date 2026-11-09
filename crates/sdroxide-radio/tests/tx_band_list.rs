//! The operator's transmit band list comes before every other transmit rail.
//!
//! A radio that nobody has set up transmits nowhere; an upgraded one carries
//! its bands over but stays blocked until they are confirmed; and a confirmed
//! list is checked on the real dial frequency, so neither the ALL selector nor
//! a band-to-band jump can put a carrier on a band that is switched off.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Band, Command, DeviceCaps, RadioConfig, RadioEvent, Vfo};

/// A transmit-capable stand-in that records the frequencies it was keyed on.
struct MockRig {
    center: f64,
    keyed: Arc<Mutex<Vec<f64>>>,
}

impl IqSource for MockRig {
    fn sample_rate(&self) -> f64 {
        600_000.0
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
        "mock rig".into()
    }
    fn tx_begin(&mut self, center_hz: f64, rate: f64) -> Result<f64> {
        self.keyed.lock().unwrap().push(center_hz);
        Ok(rate)
    }
    fn tx_write(&mut self, _samples: &[Complex32]) -> Result<()> {
        Ok(())
    }
    fn tx_write_audio(&mut self, _audio: &[f32]) -> Result<()> {
        Ok(())
    }
    fn tx_end(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Point the config at a scratch directory: this test writes a `radio.json`.
fn isolate_config() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let root = std::env::temp_dir().join(format!("sdroxide-tx-bands-{}", std::process::id()));
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
        tx_channels: 1,
        sample_rates: vec![600_000.0],
        freq_ranges_rx: vec![(100_000.0, 60_000_000.0)],
        freq_ranges_tx: vec![(1_000_000.0, 60_000_000.0)],
        ..DeviceCaps::default()
    }
}

fn list(bands: &[Band], confirmed: bool) -> RadioConfig {
    RadioConfig {
        tx_band_enable: Some(bands.to_vec()),
        tx_bands_confirmed: confirmed,
        ..RadioConfig::default()
    }
}

/// Run `dials` in order under `cfg`, keying on each, and report which of them
/// the transmitter actually started on.
fn keyed_on(cfg: RadioConfig, dials: &[f64]) -> Vec<bool> {
    keyed_on_with(cfg, dials, false)
}

/// The same, with the amateur-bands-only rail armed or not.
fn keyed_on_with(cfg: RadioConfig, dials: &[f64], tx_ham_only: bool) -> Vec<bool> {
    keyed_run(cfg, dials, tx_ham_only, true)
}

/// The same, with the station's "Allow transmit on 11 m (CB)" given.
fn keyed_run(cfg: RadioConfig, dials: &[f64], tx_ham_only: bool, cb_tx_allowed: bool) -> Vec<bool> {
    isolate_config();
    let keyed = Arc::new(Mutex::new(Vec::new()));
    let src = MockRig { center: dials[0], keyed: Arc::clone(&keyed) };
    let mut h = start_engine(
        Box::new(src),
        caps(),
        EngineConfig { cb_tx_allowed, tx_ham_only, ..Default::default() },
    );
    let thread = h.thread.take();
    h.cmd_tx.send(Command::SetRadioConfig { cfg: Box::new(cfg), reopen: false }).unwrap();
    std::thread::sleep(Duration::from_millis(150));

    let mut out = Vec::new();
    for &hz in dials {
        let before = keyed.lock().unwrap().len();
        h.cmd_tx.send(Command::SetVfo { vfo: Vfo::A, hz }).unwrap();
        h.cmd_tx.send(Command::SetPtt(true)).unwrap();
        let deadline = Instant::now() + Duration::from_millis(600);
        while Instant::now() < deadline && keyed.lock().unwrap().len() == before {
            while h.event_rx.try_recv().is_ok() {}
            std::thread::sleep(Duration::from_millis(20));
        }
        out.push(keyed.lock().unwrap().len() > before);
        h.cmd_tx.send(Command::SetPtt(false)).unwrap();
        std::thread::sleep(Duration::from_millis(150));
    }
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    let _ = RadioEvent::Notice(None);
    out
}

#[test]
fn a_new_radio_transmits_nowhere() {
    let new = list(&[], false);
    assert_eq!(keyed_on(new, &[7_074_000.0, 14_074_000.0]), [false, false]);
}

#[test]
fn an_upgraded_radio_is_blocked_until_the_bands_are_confirmed() {
    let carried = list(&[Band::M40, Band::M20], false);
    assert_eq!(keyed_on(carried, &[7_074_000.0]), [false]);
}

#[test]
fn only_the_confirmed_bands_key_up() {
    let cfg = list(&[Band::M40], true);
    assert_eq!(keyed_on(cfg, &[7_074_000.0, 3_573_000.0, 7_074_000.0]), [true, false, true]);
}

/// CB to 80 m with 80 m off: the jump lands on a band that is switched off.
#[test]
fn a_jump_from_cb_to_a_band_that_is_off_does_not_key() {
    let cfg = list(&[Band::M11], true);
    assert_eq!(keyed_on(cfg, &[27_185_000.0, 3_573_000.0]), [true, false]);
}

/// The ALL selector tunes anywhere, and anywhere outside the bands is never
/// keyed: there is no "other" row, and a list that still names it grants nothing.
#[test]
fn a_dial_outside_every_band_is_never_keyed() {
    // 35 MHz is above 10 m, below 6 m and in no broadcast band.
    const OUTSIDE: f64 = 35_000_000.0;
    assert_eq!(Band::containing(OUTSIDE), Band::Gen);
    let off = list(&[Band::M40, Band::M20], true);
    assert_eq!(keyed_on(off, &[OUTSIDE]), [false]);
    let on = list(&[Band::Gen], true);
    assert_eq!(keyed_on(on, &[OUTSIDE, 7_074_000.0]), [false, false]);
}

/// One rule: with the list in force, a ticked and validated band keys, even
/// with the amateur-only flag armed (a listening band the radio can transmit on
/// included) — and a band that is not ticked never does.
#[test]
fn the_list_is_the_single_rule_even_with_the_amateur_only_flag() {
    let cfg = list(&[Band::Gen, Band::M40], true);
    assert_eq!(keyed_on_with(cfg, &[35_000_000.0, 7_074_000.0], true), [false, true]);
    let no_other = list(&[Band::M40], true);
    assert_eq!(keyed_on_with(no_other, &[35_000_000.0, 7_074_000.0], true), [false, true]);
}

/// 11 m has two locks. The list comes first (11M-CB ticked and confirmed);
/// "Allow transmit on 11 m (CB)" second. Either one missing keeps the radio
/// from keying on 11 m, and neither opens any other band.
#[test]
fn eleven_metres_needs_the_list_and_the_cb_permission() {
    const CB: f64 = 27_185_000.0;
    let ticked = list(&[Band::M11, Band::M40], true);
    assert_eq!(keyed_run(ticked.clone(), &[CB, 7_074_000.0], false, true), [true, true]);
    assert_eq!(
        keyed_run(ticked, &[CB, 7_074_000.0], false, false),
        [false, true],
        "permission off: 11 m is refused, 40 m is not touched"
    );
    let unticked = list(&[Band::M40], true);
    assert_eq!(keyed_run(unticked, &[CB], false, true), [false], "permission alone opens nothing");
}
