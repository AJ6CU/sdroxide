//! Sending a Meshtastic text message through the ordinary transmit path.
//!
//! The frame goes out as raw I/Q, so every lock that applies to a key-down
//! applies: the transmit band list refuses what it refuses (and this test
//! never touches it), the radio has to be on the channel, and the I/Q that
//! reaches the transmitter decodes back to the text.
//!
//! The mock transmitter runs at the modem's own rate (250 kHz), so what it
//! captures is the frame before any resampling and goes straight back into the
//! demodulator.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sdroxide_meshtastic::packet::decode_frame;
use sdroxide_meshtastic::protocol::{BROADCAST_NODE, DEFAULT_PRESET};
use sdroxide_meshtastic::{DecoderConfig, KeySet, LoraDecoder};
use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{
    Band, Command, DeviceCaps, MeshTxRefusal, MeshtasticSend, MeshtasticSettings, MeshtasticStatus,
    RadioEvent, TxBlock,
};

const RATE: f64 = 2_000_000.0;
const CHANNEL_HZ: f64 = 869_525_000.0;
/// The transmitter's own sample rate: the LongFast bandwidth.
const TX_RATE: f64 = 250_000.0;

#[derive(Default)]
struct Sent {
    iq: Vec<Complex32>,
    begun: usize,
}

struct MockTx {
    center: f64,
    sent: Arc<Mutex<Sent>>,
}

impl IqSource for MockTx {
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
        std::thread::sleep(Duration::from_millis(2));
        let n = buf.len().min(8192);
        buf[..n].fill(Complex32::new(0.0, 0.0));
        Ok(n)
    }
    fn describe(&self) -> String {
        "mock meshtastic transmitter".into()
    }
    fn tx_begin(&mut self, _center_hz: f64, _rate: f64) -> Result<f64> {
        self.sent.lock().unwrap().begun += 1;
        Ok(TX_RATE)
    }
    fn tx_write(&mut self, samples: &[Complex32]) -> Result<()> {
        self.sent.lock().unwrap().iq.extend_from_slice(samples);
        Ok(())
    }
    fn tx_end(&mut self) -> Result<()> {
        Ok(())
    }
}

fn isolate_config() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let root =
            std::env::temp_dir().join(format!("sdroxide-meshtastic-tx-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };
    });
}

fn caps() -> DeviceCaps {
    DeviceCaps {
        driver: "mock-tx".into(),
        label: "mock-tx".into(),
        rx_channels: 1,
        tx_channels: 1,
        sample_rates: vec![RATE],
        freq_ranges_rx: vec![(0.0, 1_000_000_000.0)],
        freq_ranges_tx: vec![(860_000_000.0, 880_000_000.0)],
        ..DeviceCaps::default()
    }
}

fn on() -> MeshtasticSettings {
    MeshtasticSettings { enabled: true, ..Default::default() }
}

fn hello(text: &str) -> Command {
    Command::SendMeshtastic(MeshtasticSend { text: text.into(), to: BROADCAST_NODE })
}

/// The first status after `since` that says something about transmitting.
fn tx_status(
    h: &sdroxide_radio::EngineHandles,
    within: Duration,
    pred: impl Fn(&MeshtasticStatus) -> bool,
) -> Option<MeshtasticStatus> {
    let end = Instant::now() + within;
    while Instant::now() < end {
        while let Ok(ev) = h.event_rx.try_recv() {
            if let RadioEvent::MeshtasticStatus(s) = ev
                && pred(&s)
            {
                return Some(*s);
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    None
}

fn start(
    cfg: EngineConfig,
) -> (sdroxide_radio::EngineHandles, Arc<Mutex<Sent>>, Option<std::thread::JoinHandle<()>>) {
    isolate_config();
    let sent = Arc::new(Mutex::new(Sent::default()));
    let mut h =
        start_engine(Box::new(MockTx { center: CHANNEL_HZ, sent: Arc::clone(&sent) }), caps(), cfg);
    let thread = h.thread.take();
    std::thread::sleep(Duration::from_millis(200));
    (h, sent, thread)
}

fn finish(h: sdroxide_radio::EngineHandles, thread: Option<std::thread::JoinHandle<()>>) {
    drop(h.cmd_tx);
    drop(h.event_rx);
    if let Some(t) = thread {
        let _ = t.join();
    }
}

/// The transmit band list is the one rule, and on 869.525 MHz ("Other") it
/// says no: nothing reaches the transmitter, the panel is told why.
#[test]
fn the_transmit_band_list_refuses_the_channel_and_nothing_is_sent() {
    let (h, sent, thread) = start(EngineConfig { tx_ham_only: false, ..Default::default() });
    h.cmd_tx.send(Command::SetMeshtasticConfig(on())).unwrap();
    h.cmd_tx.send(Command::TuneWidebandTo(CHANNEL_HZ)).unwrap();
    std::thread::sleep(Duration::from_millis(300));
    h.cmd_tx.send(hello("bonjour")).unwrap();
    let st = tx_status(&h, Duration::from_secs(5), |s| s.tx_refused.is_some());
    let begun = sent.lock().unwrap().begun;
    finish(h, thread);
    let st = st.expect("a status that says why");
    assert!(
        matches!(
            st.tx_refused,
            Some(MeshTxRefusal::Band(
                TxBlock::Unconfirmed | TxBlock::BandOff(Band::Gen | Band::Ism868)
            ))
        ),
        "{:?}",
        st.tx_refused
    );
    assert_eq!(begun, 0, "the transmitter was never keyed");
    assert_eq!(st.tx_sent, 0);
}

/// The real lock, not the bypass: with the 868 MHz band ticked and the list
/// confirmed, as an operator does it, the message goes; unticked, it does not.
#[test]
fn ticking_and_confirming_the_868_band_opens_the_lock() {
    let (h, sent, thread) = start(EngineConfig { tx_ham_only: false, ..Default::default() });
    h.cmd_tx.send(Command::SetMeshtasticConfig(on())).unwrap();
    h.cmd_tx.send(Command::TuneWidebandTo(CHANNEL_HZ)).unwrap();
    std::thread::sleep(Duration::from_millis(300));
    // Ticked but not confirmed: still shut.
    let mut cfg = sdroxide_types::RadioConfig::default();
    cfg.tx_band_enable = Some(vec![Band::Ism868]);
    h.cmd_tx.send(Command::SetRadioConfig { cfg: Box::new(cfg.clone()), reopen: false }).unwrap();
    std::thread::sleep(Duration::from_millis(300));
    h.cmd_tx.send(hello("avant")).unwrap();
    let st = tx_status(&h, Duration::from_secs(5), |s| s.tx_refused.is_some());
    assert_eq!(st.expect("refused").tx_refused, Some(MeshTxRefusal::Band(TxBlock::Unconfirmed)));
    assert_eq!(sent.lock().unwrap().begun, 0);
    // Confirmed: open.
    cfg.tx_bands_confirmed = true;
    h.cmd_tx.send(Command::SetRadioConfig { cfg: Box::new(cfg), reopen: false }).unwrap();
    std::thread::sleep(Duration::from_millis(300));
    h.cmd_tx.send(hello("apres")).unwrap();
    let st = tx_status(&h, Duration::from_secs(20), |s| s.tx_sent >= 1);
    let begun = sent.lock().unwrap().begun;
    finish(h, thread);
    assert_eq!(st.expect("sent").tx_refused, None);
    assert_eq!(begun, 1);
}

/// Off the channel: the dial is somewhere else, so nothing goes out.
#[test]
fn a_dial_off_the_channel_sends_nothing() {
    let (h, sent, thread) =
        start(EngineConfig { tx_bands_open: true, tx_ham_only: false, ..Default::default() });
    h.cmd_tx.send(Command::SetMeshtasticConfig(on())).unwrap();
    h.cmd_tx.send(Command::SetVfo { vfo: sdroxide_types::Vfo::A, hz: 868_100_000.0 }).unwrap();
    std::thread::sleep(Duration::from_millis(300));
    h.cmd_tx.send(hello("bonjour")).unwrap();
    let st = tx_status(&h, Duration::from_secs(5), |s| s.tx_refused.is_some());
    let begun = sent.lock().unwrap().begun;
    finish(h, thread);
    assert_eq!(st.expect("a refusal").tx_refused, Some(MeshTxRefusal::OffChannel));
    assert_eq!(begun, 0);
}

/// An empty message is not a frame.
#[test]
fn an_empty_text_is_refused() {
    let (h, sent, thread) =
        start(EngineConfig { tx_bands_open: true, tx_ham_only: false, ..Default::default() });
    h.cmd_tx.send(Command::SetMeshtasticConfig(on())).unwrap();
    h.cmd_tx.send(Command::TuneWidebandTo(CHANNEL_HZ)).unwrap();
    std::thread::sleep(Duration::from_millis(300));
    h.cmd_tx.send(hello("")).unwrap();
    let st = tx_status(&h, Duration::from_secs(5), |s| s.tx_refused.is_some());
    let begun = sent.lock().unwrap().begun;
    finish(h, thread);
    assert_eq!(st.expect("a refusal").tx_refused, Some(MeshTxRefusal::BadText));
    assert_eq!(begun, 0);
}

/// Where the lock allows it (the list is open here), the frame reaches the
/// transmitter and decodes back, with the station's node number as sender.
#[test]
fn an_allowed_message_reaches_the_transmitter_and_decodes() {
    let (h, sent, thread) =
        start(EngineConfig { tx_bands_open: true, tx_ham_only: false, ..Default::default() });
    h.cmd_tx.send(Command::SetMeshtasticConfig(on())).unwrap();
    h.cmd_tx.send(Command::TuneWidebandTo(CHANNEL_HZ)).unwrap();
    std::thread::sleep(Duration::from_millis(300));
    h.cmd_tx.send(hello("73 de F4NRM")).unwrap();
    let st = tx_status(&h, Duration::from_secs(20), |s| s.tx_sent >= 1);
    let iq = std::mem::take(&mut sent.lock().unwrap().iq);
    finish(h, thread);
    let st = st.expect("the message went out");
    assert_eq!(st.tx_refused, None);
    assert!(!st.tx_busy);
    assert!(st.tx_used_ms > 100, "the air time was counted: {} ms", st.tx_used_ms);
    // EU 868 is held to 1 % of an hour.
    assert_eq!(st.tx_budget_ms, Some(36_000));

    let p = DEFAULT_PRESET;
    let mut dec = LoraDecoder::new(DecoderConfig::new(p.sf, p.cr, p.bw_hz)).unwrap();
    let mut frames = dec.feed(&vec![Complex32::new(0.0, 0.0); 16 * (1usize << p.sf)]);
    frames.extend(dec.feed(&iq));
    frames.extend(dec.feed(&vec![Complex32::new(0.0, 0.0); 16 * (1usize << p.sf)]));
    let frame = frames.iter().find(|f| f.crc_ok).expect("a frame with a good CRC");
    let pkt = decode_frame(&frame.payload, &KeySet::with_defaults()).expect("a Meshtastic frame");
    assert!(pkt.decrypted);
    assert_eq!(pkt.data.unwrap().payload, b"73 de F4NRM");
    assert!(pkt.header.from >= 256, "the station drew a node number");
    assert_eq!(pkt.header.to, BROADCAST_NODE);
    let stored = sdroxide_config::load_meshtastic_config();
    assert_eq!(stored.node_id, pkt.header.from, "the node number is kept");
}
