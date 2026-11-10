//! An APRS message sent from an I/Q transmitter — a PlutoSDR is one — comes out
//! as FM that a receiver decodes.
//!
//! `aprs_tx.rs` captures the audio handed to a transceiver on a sound card.
//! An SDR is the other shape: the engine modulates, and what reaches the
//! hardware is complex baseband. This captures that, runs it back through the
//! same FM discriminator a receiving SDROXIDE uses, and decodes the frame.
//!
//! ⚠️ `SetDigiConfig` makes the engine save, and `SDROXIDE_CONFIG_DIR` is
//! process-global: see [`isolate_config`].

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sdroxide_ax25::{Deframer, Packet, PacketType};
use sdroxide_dsp::{AfskProfile, AfskRx, make_demod};
use sdroxide_radio::{AudioParams, Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DeviceCaps, DigiConfig, Mode, RxId};

const DIAL_HZ: f64 = 144_800_000.0;
/// What a PlutoSDR-class front end transmits at.
const TX_RATE: f64 = 192_000.0;

#[derive(Default)]
struct Sent {
    iq: Vec<Complex32>,
    blocks: usize,
}

struct Sdr {
    sent: Arc<Mutex<Sent>>,
    rng: u32,
}

impl IqSource for Sdr {
    fn sample_rate(&self) -> f64 {
        48_000.0
    }
    fn center_hz(&self) -> f64 {
        DIAL_HZ
    }
    fn set_center_hz(&mut self, _hz: f64) -> Result<()> {
        Ok(())
    }
    fn center_is_dial(&self) -> bool {
        true
    }
    fn read(&mut self, buf: &mut [Complex32]) -> Result<usize> {
        std::thread::sleep(Duration::from_millis(5));
        let n = buf.len().min(1024);
        // Noise, not silence: CSMA counts its slots on the receive clock.
        for z in &mut buf[..n] {
            self.rng = self.rng.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            let a = (self.rng >> 16) as f32 / 32_768.0 - 1.0;
            *z = Complex32::new(a * 0.02, a * 0.02);
        }
        Ok(n)
    }
    fn describe(&self) -> String {
        "mock I/Q transmitter".into()
    }
    fn tx_begin(&mut self, _center_hz: f64, _rate: f64) -> Result<f64> {
        Ok(TX_RATE)
    }
    fn tx_write(&mut self, samples: &[Complex32]) -> Result<()> {
        let mut s = self.sent.lock().unwrap();
        s.blocks += 1;
        s.iq.extend_from_slice(samples);
        Ok(())
    }
    fn tx_end(&mut self) -> Result<()> {
        Ok(())
    }
}

fn isolate_config() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let root = std::env::temp_dir().join(format!("sdroxide-aprs-tx-iq-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };
    });
}

fn iq_over(cmds: Vec<Command>) -> Vec<Complex32> {
    isolate_config();
    let sent = Arc::new(Mutex::new(Sent::default()));
    let caps = DeviceCaps {
        driver: "mock-sdr".into(),
        label: "mock I/Q transmitter".into(),
        rx_channels: 1,
        tx_channels: 1,
        freq_ranges_rx: vec![(10_000.0, 500_000_000.0)],
        freq_ranges_tx: vec![(144_000_000.0, 148_000_000.0)],
        ..DeviceCaps::default()
    };
    let (producer, _consumer) = rtrb::RingBuffer::<f32>::new(48_000);
    let cfg = EngineConfig { tx_bands_open: true,
        tx_ham_only: false,
        remember_session: false,
        audio: Some(AudioParams { producer, out_rate: 48_000.0 }),
        ..Default::default()
    };
    let mut h = start_engine(Box::new(Sdr { sent: Arc::clone(&sent), rng: 0x1234_5678 }), caps, cfg);
    let thread = h.thread.take();
    std::thread::sleep(Duration::from_millis(200));
    for c in cmds {
        h.cmd_tx.send(c).expect("engine gone");
        std::thread::sleep(Duration::from_millis(80));
    }
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut quiet_since: Option<Instant> = None;
    while Instant::now() < deadline {
        while h.event_rx.try_recv().is_ok() {}
        let before = sent.lock().unwrap().blocks;
        std::thread::sleep(Duration::from_millis(50));
        let after = sent.lock().unwrap().blocks;
        if after > 0 && after == before {
            match quiet_since {
                Some(t) if t.elapsed() > Duration::from_millis(250) => break,
                Some(_) => {}
                None => quiet_since = Some(Instant::now()),
            }
        } else {
            quiet_since = None;
        }
    }
    let out = std::mem::take(&mut sent.lock().unwrap().iq);
    drop(h.cmd_tx);
    drop(h.event_rx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    assert!(!out.is_empty(), "nothing ever reached the transmitter");
    out
}

#[test]
fn a_message_from_an_iq_transmitter_decodes_after_fm_demodulation() {
    let iq = iq_over(vec![
        Command::SetMode { rx: RxId::Main, mode: Mode::Aprs },
        Command::SetDigiConfig(DigiConfig {
            my_call: "OE3JJS".into(),
            my_grid: "JN88ec".into(),
            packet_persist: 255,
            packet_slottime_ms: 10,
            ..DigiConfig::default()
        }),
        Command::AprsSendMessage { to: "VK2ABC".into(), text: "hello hello".into() },
    ]);
    let secs = iq.len() as f64 / TX_RATE;
    assert!(secs > 0.5, "the over was only {secs:.3} s long");

    let mut demod = make_demod(Mode::Aprs, TX_RATE).expect("APRS has a demodulator");
    let mut audio = Vec::new();
    for chunk in iq.chunks(1920) {
        demod.process(chunk, &mut audio);
    }
    let mut modem = AfskRx::new(demod.audio_rate(), AfskProfile::Vhf1200);
    let mut deframer = Deframer::new();
    let (mut levels, mut frames) = (Vec::new(), Vec::new());
    for chunk in audio.chunks(1920) {
        levels.clear();
        modem.process(chunk, &mut levels);
        for lvl in levels.drain(..) {
            deframer.push_level(lvl, &mut frames);
        }
    }
    assert!(!frames.is_empty(), "{secs:.3} s of I/Q reached the transmitter and nothing decoded");
    let p = Packet::parse(&frames[0], None).expect("the frame does not parse");
    assert_eq!(p.src().call(), "OE3JJS");
    let PacketType::Ui(ui) = p.packet_type() else { panic!("must be a UI frame") };
    match sdroxide_aprs::parse(p.dst().call(), &ui.payload).expect("not APRS") {
        sdroxide_aprs::AprsData::Message(m) => {
            assert_eq!(m.addressee, "VK2ABC");
            assert_eq!(m.text, "hello hello");
        }
        other => panic!("{other:?}"),
    }
}
