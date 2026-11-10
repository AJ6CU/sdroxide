//! `/aircraft-history` on a station whose operator has **not** switched it on,
//! which is every station by default: the endpoints say so, and a radio's
//! decoder reporting aircraft leaves nothing behind to be served.
//!
//! A binary of its own because the switch is read from the environment when
//! the station starts; `aircraft_history.rs` is the station with it on.

use std::sync::Once;
use std::time::Duration;

use sdroxide_server::{AccessFn, RadioParams, ServerParams, serve};
use sdroxide_types::{AdsbAircraft, AdsbStatus, RadioEvent, RemoteAccess, SpectrumFrame};

const USER: &str = "f6kim";
const PASS: &str = "correct horse battery staple";

/// One port per test: they run side by side.

const PORT_OFF: u16 = 39494;
const PORT_OFF_EXPORT: u16 = 39512;

/// The station reads its switch from the environment; this one is left unset,
/// and the config folder is a folder of its own.
fn isolated_config() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let dir = std::env::temp_dir().join(format!("sdroxide-history-off-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        // SAFETY: first thing the test does, before any thread of ours reads
        // the environment.
        unsafe {
            std::env::set_var("SDROXIDE_CONFIG_DIR", &dir);
            std::env::remove_var("SDROXIDE_AIRCRAFT_HISTORY");
        }
    });
}

async fn spawn_station(port: u16, password: bool) -> crossbeam_channel::Sender<RadioEvent> {
    isolated_config();
    let frame = SpectrumFrame {
        seq: 0,
        center_hz: 1_090_000_000.0,
        span_hz: 2_000_000.0,
        db_floor: -120.0,
        db_ceil: 0.0,
        bins: Vec::new(),
        rows: Vec::new(),
        rows_clocked: false,
    };
    let (_spec_in, spectrum_out) = triple_buffer::triple_buffer(&frame);
    let (_wide_in, wide_spectrum_out) = triple_buffer::triple_buffer(&frame);
    let (cmd_tx, _cmd_rx) = crossbeam_channel::unbounded();
    let (event_tx, event_rx) = crossbeam_channel::unbounded();
    let (_audio_tx, audio_rx) = rtrb::RingBuffer::<f32>::new(96_000);
    let (mic_tx, _mic_rx) = rtrb::RingBuffer::<f32>::new(48_000);
    std::mem::forget((_spec_in, _wide_in, _cmd_rx, _audio_tx, _mic_rx));
    let access: Option<AccessFn> = password.then(|| -> AccessFn {
        Box::new(|| RemoteAccess { username: USER.into(), password: PASS.into() })
    });
    tokio::spawn(serve(ServerParams {
        radios: vec![RadioParams {
            id: 0,
            name: String::new(),
            cmd_tx,
            event_rx,
            spectrum_out,
            wide_spectrum_out,
            audio_rx,
            mic_tx,
        }],
        bind: "127.0.0.1".into(),
        port,
        web_root: None,
        access,
        probe: None,
        add_radio: None,
        remove_radio: None,
        rename_radio: None,
        radio_power: None,
        aircraft_online: Some(false),
    }));
    tokio::time::sleep(Duration::from_millis(400)).await;
    event_tx
}

async fn get(port: u16, path: &str, cookie: Option<&str>) -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut sock = tokio::net::TcpStream::connect(("127.0.0.1", port)).await.expect("connect");
    let cookie = cookie.map(|c| format!("Cookie: {c}\r\n")).unwrap_or_default();
    let req =
        format!("GET {path} HTTP/1.1\r\nHost: 127.0.0.1\r\n{cookie}Connection: close\r\n\r\n");
    sock.write_all(req.as_bytes()).await.expect("write");
    let mut body = String::new();
    tokio::time::timeout(Duration::from_secs(5), sock.read_to_string(&mut body))
        .await
        .expect("timeout reading the answer")
        .expect("read");
    body
}

fn now() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as i64
}

fn status_with_a_plane() -> RadioEvent {
    let t = now();
    let mut a = AdsbAircraft::new(0x4840D6, t);
    a.callsign = "KLM1234".into();
    a.lat = Some(52.0);
    a.lon = Some(5.0);
    a.altitude_ft = Some(35_000);
    a.last_at = t;
    a.last_pos_at = t;
    RadioEvent::AdsbStatus(Box::new(AdsbStatus { aircraft: vec![a], ..AdsbStatus::default() }))
}

/// Out of the box the history is shut: both endpoints say 503 and "disabled",
/// through a proxy's prefix too, and what a radio reported is not served — it was
/// never kept.
#[tokio::test]
async fn a_station_that_has_not_switched_it_on_keeps_and_serves_nothing() {
    let events = spawn_station(PORT_OFF, false).await;
    events.send(status_with_a_plane()).unwrap();
    tokio::time::sleep(Duration::from_millis(400)).await;
    for path in [
        "/aircraft-history",
        "/aircraft-history/4840d6",
        "/sdroxide/aircraft-history",
        "/shack/sdr/aircraft-history/4840D6",
    ] {
        let answer = get(PORT_OFF, path, None).await;
        assert!(answer.starts_with("HTTP/1.1 503"), "{path}: {answer}");
        assert!(answer.contains("\"status\":\"disabled\""), "{path}: {answer}");
        assert!(!answer.contains("KLM1234"), "{path}: {answer}");
    }
}

/// The export is as shut as the rest: 503 and "disabled" for both formats, and a
/// bad format is not looked at before the switch.
#[tokio::test]
async fn the_export_of_a_station_that_keeps_nothing_says_disabled() {
    let events = spawn_station(PORT_OFF_EXPORT, false).await;
    events.send(status_with_a_plane()).unwrap();
    tokio::time::sleep(Duration::from_millis(400)).await;
    for path in [
        "/aircraft-export?format=csv",
        "/aircraft-export?format=json",
        "/shack/sdr/aircraft-export?format=csv",
        "/aircraft-export",
    ] {
        let answer = get(PORT_OFF_EXPORT, path, None).await;
        assert!(answer.starts_with("HTTP/1.1 503"), "{path}: {answer}");
        assert!(answer.contains("\"status\":\"disabled\""), "{path}: {answer}");
        assert!(!answer.contains("KLM1234"), "{path}: {answer}");
    }
}
