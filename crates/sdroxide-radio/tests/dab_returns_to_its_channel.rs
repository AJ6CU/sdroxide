//! Coming back to DAB brings the radio back to the DAB channel.
//!
//! A report: listening in DAB, the operator pressed 6M (50.150 MHz), then went
//! back to DAB — and the receiver stayed on 50.150. The decoder looks for its
//! channel *relative to where the radio is tuned*, so 201.072 MHz was far
//! outside what was being received: no sound, and picking a station changed
//! nothing either. Only picking a channel moved the radio. Like APRS, ADS-B,
//! VDL2 and AIS, DAB is a channel the receiver has to be on, so choosing the
//! mode — or a station in it — tunes there.

use std::time::Duration;

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{
    Band, Command, DabKnownStation, DabSettings, DeviceCaps, Mode, RadioEvent, RxId, Vfo,
};

const RATE: f64 = 2_500_000.0;

const CH_8D: f64 = 201_072_000.0;
const CH_11D: f64 = 222_064_000.0;
const SIX_METRES: f64 = 50_150_000.0;

struct Silence {
    center: f64,
}

impl IqSource for Silence {
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
        "silence".into()
    }
}

/// Run `cmds` on a fresh engine and return the dial and the front end's centre
/// from its last state.
fn dial_after(cmds: Vec<Command>) -> (f64, f64, Mode) {
    dial_after_seen(cmds).0
}

/// [`dial_after`], and whether a `ModeRefused` event came out.
fn dial_after_seen(cmds: Vec<Command>) -> ((f64, f64, Mode), bool) {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let root = std::env::temp_dir().join(format!("sdroxide-dab-return-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };
    });
    let caps = DeviceCaps {
        driver: "mock".into(),
        label: "mock".into(),
        rx_channels: 1,
        sample_rates: vec![RATE],
        freq_ranges_rx: vec![(0.0, 1_000_000_000.0)],
        ..DeviceCaps::default()
    };
    let mut h =
        start_engine(Box::new(Silence { center: 14_200_000.0 }), caps, EngineConfig::default());
    let thread = h.thread.take();
    std::thread::sleep(Duration::from_millis(150));
    for c in cmds {
        h.cmd_tx.send(c).unwrap();
        std::thread::sleep(Duration::from_millis(120));
    }
    std::thread::sleep(Duration::from_millis(200));
    let mut last = None;
    let mut refused = false;
    while let Ok(ev) = h.event_rx.try_recv() {
        match ev {
            RadioEvent::State(s) => last = Some((s.vfo_a_hz, s.center_hz, s.rx[0].mode)),
            RadioEvent::ModeRefused(..) => refused = true,
            _ => {}
        }
    }
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    (last.expect("the engine publishes state"), refused)
}

fn dab(channel: &str, found: &[&str]) -> DabSettings {
    DabSettings {
        channel: channel.into(),
        found: found.iter().map(|s| s.to_string()).collect(),
        ..DabSettings::default()
    }
}

fn on_channel((dial, centre, _): (f64, f64, Mode), hz: f64, what: &str) {
    assert!(
        (dial - hz).abs() < 1_000.0,
        "{what}: the dial is on {:.3} MHz, not the DAB channel at {:.3}",
        dial / 1e6,
        hz / 1e6
    );
    assert!(
        (centre - hz).abs() < RATE / 2.0,
        "{what}: the front end is on {:.3} MHz, too far to hear {:.3}",
        centre / 1e6,
        hz / 1e6
    );
}

const fn set_mode(mode: Mode) -> Command {
    Command::SetMode { rx: RxId::Main, mode }
}

fn tune(hz: f64) -> Command {
    Command::SetVfo { vfo: Vfo::A, hz }
}

/// The report itself: DAB, then 6 m, then DAB again.
#[test]
fn back_to_dab_from_six_metres_is_back_on_the_channel() {
    let at = dial_after(vec![
        Command::SetDabConfig(dab("8D", &["8D"])),
        set_mode(Mode::Dab),
        tune(CH_8D),
        set_mode(Mode::Usb),
        tune(SIX_METRES),
        set_mode(Mode::Dab),
    ]);
    on_channel(at, CH_8D, "DAB chosen again after 6 m");
}

/// Picking a station, in DAB, with the radio off its channel, brings it back
/// — it was only picking a *channel* that did.
#[test]
fn picking_a_station_brings_the_radio_back_to_its_channel() {
    let mut picked = dab("8D", &["8D"]);
    picked.service_id = "F0E1".into();
    picked.stations = vec![DabKnownStation {
        channel: "8D".into(),
        service_id: "F0E1".into(),
        label: "ICI SUD LORRAINE".into(),
    }];
    let at = dial_after(vec![
        Command::SetDabConfig(dab("8D", &["8D"])),
        set_mode(Mode::Dab),
        tune(SIX_METRES),
        Command::SetDabConfig(picked),
    ]);
    on_channel(at, CH_8D, "a station picked");
}

/// No channel in force: the first one found is the one to go to.
#[test]
fn with_no_channel_chosen_the_first_found_is_used() {
    let at = dial_after(vec![
        Command::SetDabConfig(dab("", &["11D", "8D"])),
        tune(SIX_METRES),
        set_mode(Mode::Dab),
    ]);
    on_channel(at, CH_11D, "no channel chosen");
}

/// Already on the channel: nothing moves, and the lane is on it.
#[test]
fn a_radio_already_on_its_channel_is_left_there() {
    let at = dial_after(vec![
        Command::SetDabConfig(dab("8D", &["8D"])),
        tune(CH_8D + 200.0),
        set_mode(Mode::Dab),
    ]);
    assert_eq!(at.0, CH_8D + 200.0, "a dial already on the channel is not moved");
}

/// The bug: DAB chosen while the dial sat on the FM broadcast band (the classic
/// radio's own band) was refused with "DAB is not used on FM". DAB lives on its
/// own channels, so the mode is taken and the dial goes to the channel.
#[test]
fn dab_from_the_fm_band_is_taken_and_goes_to_its_channel() {
    let (at, refused) = dial_after_seen(vec![
        Command::SetDabConfig(dab("8D", &["8D"])),
        Command::SetBand(Band::Fm),
        set_mode(Mode::Dab),
    ]);
    assert_eq!(at.2, Mode::Dab, "the mode was taken");
    assert!(!refused, "nothing was refused");
    on_channel(at, CH_8D, "DAB chosen from the FM band");
}

/// A mode a band really does not carry is still refused — now as a typed event
/// the screen words in the operator's language, not an English notice.
#[test]
fn a_mode_the_band_does_not_carry_is_refused_with_a_typed_event() {
    let (at, refused) = dial_after_seen(vec![Command::SetBand(Band::Fm), set_mode(Mode::Usb)]);
    assert!(refused, "ModeRefused was sent");
    assert_ne!(at.2, Mode::Usb, "the mode did not change");
}
