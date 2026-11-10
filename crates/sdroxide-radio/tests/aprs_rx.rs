//! What a received APRS burst becomes by the time it reaches the panel.
//!
//! Report: the APRS panel showed "0 stn" with a burst plainly visible on
//! 144.800. The controller's own tests stop at `DigiEngine::on_rx_audio` and
//! the modem's stop at the deframer; neither can see what the engine does in
//! between — which demodulator the packet mode gets, what rate the digital
//! tap runs at, and whether anything it produces reaches the status event.
//!
//! So this stands the whole engine up on a source that behaves like the radio
//! (an SDR handing back I/Q, or a transceiver handing back audio), puts a real
//! frame on the channel, and reads the station list from the status event the
//! panel itself draws from.
//!
//! ⚠️ `SetDigiConfig` makes the engine save, and `SDROXIDE_CONFIG_DIR` is
//! process-global: see [`isolate_config`].

use std::time::{Duration, Instant};

use sdroxide_ax25::Framer;
use sdroxide_dsp::{AfskProfile, AfskTx, make_modulator};
use sdroxide_radio::{AudioParams, Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DeviceCaps, DigiConfig, Mode, RadioEvent, RxId};

const DIAL_HZ: f64 = 144_800_000.0;

/// What the radio puts out: a quadrature signal (an SDR, FM on the carrier) or
/// demodulated audio in the I component (a transceiver on a sound card).
#[derive(Clone, Copy, Debug)]
enum Shape {
    FmIq,
    Audio,
}

/// A UI frame `from>APRS:info`, serialised the way the air carries it.
fn frame(from: &str, info: &str) -> Vec<u8> {
    let mut f = Vec::new();
    for (call, last) in [("APRS", false), (from, true)] {
        let mut s = call.to_string();
        while s.len() < 6 {
            s.push(' ');
        }
        for b in s.bytes() {
            f.push(b << 1);
        }
        f.push(0x60 | u8::from(last));
    }
    f.extend_from_slice(&[0x03, 0xF0]);
    f.extend_from_slice(info.as_bytes());
    f
}

/// One burst at `rate`: a lead-in of silence, TXDELAY flags, the frame, a
/// tail. Audio, not yet modulated.
fn burst_audio(rate: f64, f: &[u8]) -> Vec<f32> {
    let mut framer = Framer::new();
    framer.push_flags(40);
    framer.push_frame(f);
    framer.push_flags(8);
    let mut tx = AfskTx::new(rate, AfskProfile::Vhf1200);
    tx.push_bits(&framer.take());
    let mut out = vec![0.0; (rate * 0.2) as usize];
    let mut block = [0.0f32; 1024];
    loop {
        let n = tx.next_block(&mut block);
        if n == 0 {
            break;
        }
        out.extend_from_slice(&block[..n]);
    }
    out.extend(vec![0.0; (rate * 0.2) as usize]);
    out
}

/// The same burst as the radio would hand it back, repeated for ever with a
/// gap: the engine is not ready the instant it is started, and a burst that
/// went past in that window would fake the very symptom under test.
struct Radio {
    rate: f64,
    cycle: Vec<Complex32>,
    at: usize,
}

impl Radio {
    fn new(shape: Shape, rate: f64, f: &[u8]) -> Radio {
        let audio = burst_audio(rate, f);
        let mut cycle: Vec<Complex32> = match shape {
            Shape::Audio => audio.iter().map(|&a| Complex32::new(a, 0.0)).collect(),
            Shape::FmIq => {
                let mut m = make_modulator(Mode::Aprs, rate, (-8_000.0, 8_000.0))
                    .expect("APRS has a modulator");
                let mut iq = Vec::new();
                m.process(&audio, &mut iq);
                iq
            }
        };
        cycle.extend(vec![Complex32::new(0.0, 0.0); (rate * 0.5) as usize]);
        Radio { rate, cycle, at: 0 }
    }
}

impl IqSource for Radio {
    fn sample_rate(&self) -> f64 {
        self.rate
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
        std::thread::sleep(Duration::from_millis(2));
        let n = buf.len().min(2048);
        for z in &mut buf[..n] {
            *z = self.cycle[self.at];
            self.at = (self.at + 1) % self.cycle.len();
        }
        Ok(n)
    }
    fn describe(&self) -> String {
        "mock APRS receiver".into()
    }
}

fn isolate_config() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let root = std::env::temp_dir().join(format!("sdroxide-aprs-rx-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };
    });
}

/// Names of the stations the panel would list, and the texts of its messages,
/// after `secs` of a source putting `f` on the channel.
fn heard(shape: Shape, rate: f64, f: &[u8], secs: u64) -> (Vec<String>, Vec<String>) {
    isolate_config();
    let caps = DeviceCaps {
        driver: "mock-aprs".into(),
        label: "mock APRS receiver".into(),
        rx_channels: 1,
        tx_channels: 1,
        audio_mode: matches!(shape, Shape::Audio),
        freq_ranges_rx: vec![(10_000.0, 500_000_000.0)],
        freq_ranges_tx: vec![(144_000_000.0, 148_000_000.0)],
        ..DeviceCaps::default()
    };
    let (producer, _consumer) = rtrb::RingBuffer::<f32>::new(48_000);
    let cfg = EngineConfig {
        tx_ham_only: false,
        remember_session: false,
        audio: Some(AudioParams { producer, out_rate: 48_000.0 }),
        ..Default::default()
    };
    let mut h = start_engine(Box::new(Radio::new(shape, rate, f)), caps, cfg);
    let thread = h.thread.take();
    std::thread::sleep(Duration::from_millis(200));
    for c in [
        Command::SetMode { rx: RxId::Main, mode: Mode::Aprs },
        Command::SetDigiConfig(DigiConfig {
            my_call: "OE3JJS".into(),
            my_grid: "JN88ec".into(),
            ..DigiConfig::default()
        }),
    ] {
        h.cmd_tx.send(c).expect("engine gone");
        std::thread::sleep(Duration::from_millis(80));
    }

    let (mut stations, mut messages) = (Vec::new(), Vec::new());
    let deadline = Instant::now() + Duration::from_secs(secs);
    while Instant::now() < deadline && stations.is_empty() {
        while let Ok(ev) = h.event_rx.try_recv() {
            if let RadioEvent::Ft8Status(s) = ev
                && let Some(a) = s.aprs
            {
                stations = a.stations.iter().map(|st| st.name.clone()).collect();
                messages = a.messages.iter().map(|m| m.text.clone()).collect();
            }
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    drop(h.cmd_tx);
    drop(h.event_rx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    (stations, messages)
}

/// A position report, the commonest thing on the channel, on each shape of
/// radio and at the rates a PlutoSDR-class front end runs.
#[test]
fn a_position_report_reaches_the_station_list() {
    let f = frame("S57LN", "=4603.63N/01431.26E-Op. Andrej");
    for (shape, rate) in
        [(Shape::Audio, 48_000.0), (Shape::FmIq, 48_000.0), (Shape::FmIq, 576_000.0)]
    {
        let (stations, _) = heard(shape, rate, &f, 12);
        assert!(
            stations.iter().any(|s| s == "S57LN"),
            "{shape:?} @ {rate}: a burst was on the channel and the panel lists {stations:?}"
        );
    }
}

/// Free text rides in a status report; "hello hello" is what the bench signal
/// says. It is the same path with a different payload, and a decoder that
/// reads positions but not text would pass the test above.
#[test]
fn a_status_text_reaches_the_station_list() {
    let f = frame("N0CALL", ">hello hello");
    let (stations, _) = heard(Shape::FmIq, 576_000.0, &f, 12);
    assert!(
        stations.iter().any(|s| s == "N0CALL"),
        "a status report was on the channel and the panel lists {stations:?}"
    );
}
