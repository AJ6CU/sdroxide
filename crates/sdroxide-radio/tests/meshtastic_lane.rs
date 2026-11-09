//! The Meshtastic receiver in the engine.
//!
//! The lane takes the radio's raw IQ, mixes the chosen channel down and hands
//! it to the demodulator; the receiver reads the receive band list and never
//! writes it; and turning the receiver off puts the dial and the mode back.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use sdroxide_meshtastic::packet::{Data, Header, build_frame};
use sdroxide_meshtastic::protocol::{BROADCAST_NODE, DEFAULT_PRESET, DEFAULT_PSK, port};
use sdroxide_meshtastic::{TxParams, modulate};
use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{
    Band, Command, DeviceCaps, MeshtasticSettings, MeshtasticStatus, Mode, RadioConfig, RadioEvent,
    RadioState, RxBlock, Vfo,
};

const RATE: f64 = 2_000_000.0;
const CHANNEL_HZ: f64 = 869_525_000.0;

/// A front end that plays a prepared IQ buffer once `go` is set, silence
/// before and after. The buffer is at the source rate, so the lane's
/// downconverter sees exactly what a real radio on the channel would send.
struct MockLora {
    center: f64,
    iq: Vec<Complex32>,
    pos: usize,
    go: Arc<AtomicBool>,
}

impl IqSource for MockLora {
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
        std::thread::sleep(Duration::from_millis(1));
        let n = buf.len().min(8192);
        for s in buf[..n].iter_mut() {
            *s = if self.go.load(Ordering::Relaxed) && self.pos < self.iq.len() {
                self.pos += 1;
                self.iq[self.pos - 1]
            } else {
                Complex32::new(0.0, 0.0)
            };
        }
        Ok(n)
    }
    fn describe(&self) -> String {
        "mock lora".into()
    }
}

fn isolate_config() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let root =
            std::env::temp_dir().join(format!("sdroxide-meshtastic-lane-{}", std::process::id()));
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
        sample_rates: vec![RATE],
        freq_ranges_rx: vec![(0.0, 1_000_000_000.0)],
        ..DeviceCaps::default()
    }
}

/// One text frame on the default channel, as IQ at the source rate, with
/// silence round it.
fn frame_iq(text: &str) -> Vec<Complex32> {
    let p = DEFAULT_PRESET;
    let header = Header {
        to: BROADCAST_NODE,
        from: 0xF4_0000_01,
        packet_id: 77,
        flags: 0x07,
        ..Header::default()
    };
    let data =
        Data { portnum: port::TEXT_MESSAGE, payload: text.as_bytes().to_vec(), ..Data::default() };
    let frame = build_frame(header, p.channel_name, &DEFAULT_PSK, &data).unwrap();
    let mut tx = TxParams::meshtastic(p.sf, p.cr, p.bw_hz);
    tx.os = (RATE / f64::from(p.bw_hz)) as usize;
    let pad = vec![Complex32::new(0.0, 0.0); 8 * (1usize << p.sf) * tx.os];
    let mut iq = pad.clone();
    iq.extend(modulate(&frame, &tx));
    iq.extend(pad);
    iq
}

fn last_state(h: &sdroxide_radio::EngineHandles, within: Duration) -> RadioState {
    let end = Instant::now() + within;
    let mut last = None;
    while Instant::now() < end {
        while let Ok(ev) = h.event_rx.try_recv() {
            if let RadioEvent::State(s) = ev {
                last = Some(s);
            }
        }
        if last.is_some() {
            std::thread::sleep(Duration::from_millis(50));
            while let Ok(ev) = h.event_rx.try_recv() {
                if let RadioEvent::State(s) = ev {
                    last = Some(s);
                }
            }
            break;
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    last.expect("the engine should publish state")
}

/// Wait for a receiver status that satisfies `pred`.
fn status_where(
    h: &sdroxide_radio::EngineHandles,
    within: Duration,
    seen: &mut Option<RadioState>,
    pred: impl Fn(&MeshtasticStatus) -> bool,
) -> Option<MeshtasticStatus> {
    let end = Instant::now() + within;
    while Instant::now() < end {
        while let Ok(ev) = h.event_rx.try_recv() {
            match ev {
                RadioEvent::State(st) => *seen = Some(st),
                RadioEvent::MeshtasticStatus(s) if pred(&s) => return Some(*s),
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    None
}

fn on() -> MeshtasticSettings {
    MeshtasticSettings { enabled: true, preset: "LongFast".into(), ..Default::default() }
}

#[test]
fn a_frame_in_the_window_is_decoded_end_to_end() {
    isolate_config();
    let go = Arc::new(AtomicBool::new(false));
    let mut h = start_engine(
        Box::new(MockLora {
            center: CHANNEL_HZ,
            iq: frame_iq("73 de F4NRM"),
            pos: 0,
            go: go.clone(),
        }),
        caps(),
        EngineConfig { tx_ham_only: false, ..Default::default() },
    );
    let thread = h.thread.take();
    std::thread::sleep(Duration::from_millis(150));
    h.cmd_tx.send(Command::SetMeshtasticConfig(on())).unwrap();
    std::thread::sleep(Duration::from_millis(250));
    go.store(true, Ordering::Relaxed);

    let st = status_where(&h, Duration::from_secs(20), &mut None, |s| s.frames >= 1);
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    let st = st.expect("a status with a decoded frame");
    assert!(st.running);
    assert!(st.in_window);
    assert_eq!(st.refused, None);
    assert_eq!(st.log[0].text.as_deref(), Some("73 de F4NRM"));
    assert_eq!(st.log[0].from, 0xF4_0000_01);
    assert_eq!(st.log[0].channel, "LongFast");
    assert_eq!(st.nodes[0].id, 0xF4_0000_01);
}

#[test]
fn turning_the_receiver_off_puts_the_dial_and_the_mode_back() {
    isolate_config();
    let mut h = start_engine(
        Box::new(MockLora {
            center: 14_100_000.0,
            iq: Vec::new(),
            pos: 0,
            go: Arc::new(AtomicBool::new(false)),
        }),
        caps(),
        EngineConfig { tx_ham_only: false, ..Default::default() },
    );
    let thread = h.thread.take();
    std::thread::sleep(Duration::from_millis(150));
    h.cmd_tx.send(Command::SetVfo { vfo: Vfo::A, hz: 14_200_000.0 }).unwrap();
    std::thread::sleep(Duration::from_millis(150));
    let before = last_state(&h, Duration::from_secs(2));
    let mode_before = before.rx[0].mode;
    assert_eq!(before.vfo_a_hz, 14_200_000.0);

    // On: the engine records where the radio is, then the panel moves it.
    h.cmd_tx.send(Command::SetMeshtasticConfig(on())).unwrap();
    h.cmd_tx.send(Command::TuneWidebandTo(CHANNEL_HZ)).unwrap();
    std::thread::sleep(Duration::from_millis(300));
    let during = last_state(&h, Duration::from_secs(2));
    assert!(during.meshtastic.enabled);
    let restore = during.meshtastic.restore.expect("the radio's place was recorded");
    assert_eq!(restore.dial_hz, 14_200_000.0);
    assert_eq!(restore.mode, mode_before);
    assert_ne!(during.vfo_a_hz, 14_200_000.0, "the panel's tune moved the radio");

    // Off: back where it was, restore point cleared, the file says off.
    h.cmd_tx.send(Command::SetMeshtasticConfig(MeshtasticSettings::default())).unwrap();
    std::thread::sleep(Duration::from_millis(300));
    let after = last_state(&h, Duration::from_secs(2));
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    assert!(!after.meshtastic.enabled);
    assert_eq!(after.meshtastic.restore, None);
    assert_eq!(after.vfo_a_hz, 14_200_000.0);
    assert_eq!(after.rx[0].mode, mode_before);
    let stored = sdroxide_config::load_meshtastic_config();
    assert!(!stored.enabled);
    assert_eq!(stored.restore, None);
}

#[test]
fn a_restore_point_sent_by_a_client_is_ignored() {
    isolate_config();
    let mut h = start_engine(
        Box::new(MockLora {
            center: 14_100_000.0,
            iq: Vec::new(),
            pos: 0,
            go: Arc::new(AtomicBool::new(false)),
        }),
        caps(),
        EngineConfig { tx_ham_only: false, ..Default::default() },
    );
    let thread = h.thread.take();
    std::thread::sleep(Duration::from_millis(150));
    h.cmd_tx.send(Command::SetVfo { vfo: Vfo::A, hz: 14_200_000.0 }).unwrap();
    std::thread::sleep(Duration::from_millis(150));
    let forged = MeshtasticSettings {
        restore: Some(sdroxide_types::MeshtasticRestore { dial_hz: 1_000_000.0, mode: Mode::Cw }),
        ..on()
    };
    h.cmd_tx.send(Command::SetMeshtasticConfig(forged)).unwrap();
    std::thread::sleep(Duration::from_millis(300));
    let s = last_state(&h, Duration::from_secs(2));
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    assert_eq!(s.meshtastic.restore.unwrap().dial_hz, 14_200_000.0);
}

/// The 868 band is never shut by the receive band list, confirmed or not: a
/// list that does not name it must not keep the receiver idle (that locked the
/// VFO and the tuning whenever the panel moved to 868).
#[test]
fn an_unconfirmed_receive_list_never_shuts_the_868_band() {
    isolate_config();
    let go = Arc::new(AtomicBool::new(true));
    let mut h = start_engine(
        Box::new(MockLora { center: CHANNEL_HZ, iq: frame_iq("hi"), pos: 0, go }),
        caps(),
        EngineConfig { rx_bands_enforced: true, ..Default::default() },
    );
    let thread = h.thread.take();
    let list = RadioConfig {
        rx_band_enable: Some(vec![Band::M20]),
        rx_bands_confirmed: false,
        ..RadioConfig::default()
    };
    h.cmd_tx.send(Command::SetRadioConfig { cfg: Box::new(list), reopen: false }).unwrap();
    std::thread::sleep(Duration::from_millis(150));
    h.cmd_tx.send(Command::SetMeshtasticConfig(on())).unwrap();

    let mut state = None;
    let refused = status_where(&h, Duration::from_secs(3), &mut state, |s| s.refused.is_some());
    let state = state.expect("the engine should publish state");
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    assert!(refused.is_none(), "868 is not refused: {refused:?}");
    assert!(state.meshtastic.enabled);
    assert_eq!(state.center_hz, CHANNEL_HZ);
}

#[test]
fn a_channel_outside_the_window_is_reported_not_chased() {
    isolate_config();
    let mut h = start_engine(
        Box::new(MockLora {
            center: 14_100_000.0,
            iq: Vec::new(),
            pos: 0,
            go: Arc::new(AtomicBool::new(false)),
        }),
        caps(),
        EngineConfig::default(),
    );
    let thread = h.thread.take();
    std::thread::sleep(Duration::from_millis(150));
    h.cmd_tx.send(Command::SetMeshtasticConfig(on())).unwrap();
    let mut state = None;
    let st = status_where(&h, Duration::from_secs(5), &mut state, |s| s.running);
    let state = state.expect("the engine should publish state");
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    let st = st.expect("a status");
    assert!(!st.in_window, "14 MHz is nowhere near 869.525 MHz");
    // The lane did not move the radio: only the panel's tune command does.
    assert!((state.center_hz - 14_100_000.0).abs() < RATE / 2.0);
}
