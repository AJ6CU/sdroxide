//! The operator's receive band list holds the dial.
//!
//! The mirror of `tx_band_list.rs`: a radio whose receive list is not confirmed
//! does not tune, a confirmed list lets the dial go only onto bands ticked YES
//! (the real frequency decides, "Other" included), and a refusal leaves the
//! radio on its last frequency. The list is independent of the transmit one.

use std::time::Duration;

use sdroxide_radio::{AudioParams, Complex32, EngineConfig, IqSource, Result, rtrb, start_engine};
use sdroxide_types::{Band, Command, DeviceCaps, Mode, RadioConfig, RadioEvent, RxId, Vfo};

struct MockRx {
    center: f64,
}

impl IqSource for MockRx {
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
        "mock rx".into()
    }
}

/// Point the config at a scratch directory: this test writes a `radio.json`.
fn isolate_config() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let root = std::env::temp_dir().join(format!("sdroxide-rx-bands-{}", std::process::id()));
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
        sample_rates: vec![600_000.0],
        freq_ranges_rx: vec![(100_000.0, 60_000_000.0)],
        ..DeviceCaps::default()
    }
}

fn list(bands: &[Band], confirmed: bool) -> RadioConfig {
    RadioConfig {
        rx_band_enable: Some(bands.to_vec()),
        rx_bands_confirmed: confirmed,
        ..RadioConfig::default()
    }
}

/// Start on 14.074 MHz under `cfg` and ask for each of `dials` in turn. Reports
/// the dial the engine shows after each request.
fn dial_after(cfg: RadioConfig, dials: &[f64]) -> Vec<f64> {
    isolate_config();
    let mut h = start_engine(
        Box::new(MockRx { center: 14_074_000.0 }),
        caps(),
        EngineConfig { rx_bands_enforced: true, ..Default::default() },
    );
    let thread = h.thread.take();
    h.cmd_tx.send(Command::SetRadioConfig { cfg: Box::new(cfg), reopen: false }).unwrap();
    std::thread::sleep(Duration::from_millis(150));
    // The engine publishes a state when something changed, so a refused request
    // publishes nothing: the dial is the last one seen.
    let mut dial = f64::NAN;
    let mut out = Vec::new();
    for &hz in dials {
        h.cmd_tx.send(Command::SetVfo { vfo: Vfo::A, hz }).unwrap();
        std::thread::sleep(Duration::from_millis(300));
        while let Ok(ev) = h.event_rx.try_recv() {
            if let RadioEvent::State(s) = ev {
                dial = s.vfo_a_hz;
            }
        }
        out.push(dial);
    }
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    out
}

#[test]
fn an_unconfirmed_receive_list_does_not_tune() {
    let cfg = list(&[Band::M40, Band::M20], false);
    let dial = dial_after(cfg, &[7_074_000.0]);
    assert_eq!(dial[0], 14_074_000.0, "unconfirmed: the radio must stay where it was");
}

#[test]
fn a_confirmed_list_lets_the_dial_onto_ticked_bands_only() {
    let cfg = list(&[Band::M40], true);
    let dial = dial_after(cfg, &[7_074_000.0, 3_573_000.0]);
    assert_eq!(dial[0], 7_074_000.0, "40 m is YES");
    assert_eq!(dial[1], 7_074_000.0, "80 m is NO: the radio stays on 40 m");
}

#[test]
fn receive_and_transmit_lists_are_independent() {
    // 160 m: TX NO, RX YES.
    let cfg = RadioConfig {
        tx_band_enable: Some(vec![Band::M20]),
        tx_bands_confirmed: true,
        rx_band_enable: Some(vec![Band::M160]),
        rx_bands_confirmed: true,
        ..RadioConfig::default()
    };
    let dial = dial_after(cfg, &[1_850_000.0, 14_074_000.0]);
    assert_eq!(dial[0], 1_850_000.0, "160 m is RX YES even though TX is NO");
    assert_eq!(dial[1], 1_850_000.0, "20 m is TX YES but RX NO: no tuning");
}


/// A source that sends a steady carrier 1 kHz above the centre: in USB, a 1 kHz
/// tone in the speakers.
struct ToneRx {
    center: f64,
    phase: f64,
}

impl IqSource for ToneRx {
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
        let step = std::f64::consts::TAU * 1_000.0 / 600_000.0;
        for s in buf[..n].iter_mut() {
            self.phase += step;
            *s = Complex32::new((0.3 * self.phase.cos()) as f32, (0.3 * self.phase.sin()) as f32);
        }
        Ok(n)
    }
    fn describe(&self) -> String {
        "tone rx".into()
    }
}

/// The largest sample sent to the speakers over about a second, with `cfg`
/// applied and the radio sitting on 14.074 MHz in USB.
fn loudest(cfg: RadioConfig) -> f32 {
    isolate_config();
    let (producer, mut consumer) = rtrb::RingBuffer::<f32>::new(1 << 16);
    let mut h = start_engine(
        Box::new(ToneRx { center: 14_074_000.0, phase: 0.0 }),
        caps(),
        EngineConfig {
            audio: Some(AudioParams { producer, out_rate: 48_000.0 }),
            rx_bands_enforced: true,
            ..Default::default()
        },
    );
    let thread = h.thread.take();
    h.cmd_tx.send(Command::SetRadioConfig { cfg: Box::new(cfg), reopen: false }).unwrap();
    h.cmd_tx.send(Command::SetMode { rx: RxId::Main, mode: Mode::Usb }).unwrap();
    let mut best = 0.0f32;
    let end = std::time::Instant::now() + Duration::from_millis(1500);
    while std::time::Instant::now() < end {
        while let Ok(s) = consumer.pop() {
            best = best.max(s.abs());
        }
        while h.event_rx.try_recv().is_ok() {}
        std::thread::sleep(Duration::from_millis(20));
    }
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    best
}

/// The receive band list cuts what is *heard*, not only the retunes: a radio
/// restored on a band that is RX NO, or with an unconfirmed list, is silent.
#[test]
fn a_dial_on_a_refused_band_is_not_heard() {
    let heard = loudest(list(&[Band::M20], true));
    assert!(heard > 0.01, "20 m is RX YES and confirmed: the tone is heard ({heard})");
    let off = loudest(list(&[Band::M40], true));
    assert_eq!(off, 0.0, "20 m is RX NO: silence");
    let unconfirmed = loudest(list(&[Band::M20], false));
    assert_eq!(unconfirmed, 0.0, "list not confirmed: silence");
}
