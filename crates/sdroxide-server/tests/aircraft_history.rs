//! `/aircraft-history` on a real station: that what a radio's decoder reports
//! is kept, that the list and one aircraft are served where the clients look
//! for them, behind the same door as the sockets, and through a proxy's prefix.
//!
//! Nothing here reaches the internet. The events come from a channel the test
//! holds, in place of an engine.

use std::sync::Once;
use std::time::Duration;

use sdroxide_server::{AccessFn, RadioParams, ServerParams, serve};
use sdroxide_types::{AdsbAircraft, AdsbStatus, RadioEvent, RemoteAccess, SpectrumFrame};

const USER: &str = "f6kim";
const PASS: &str = "correct horse battery staple";

/// One port per test: they run side by side.
const PORT_OPEN: u16 = 39491;
const PORT_PASSWORD: u16 = 39492;
const PORT_BAD: u16 = 39493;
const PORT_EXPORT: u16 = 39510;
const PORT_EXPORT_PASSWORD: u16 = 39511;

/// The station reads and writes its history in the config directory; this
/// test's is a folder of its own, so the operator's is never touched.
fn isolated_config() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let dir = std::env::temp_dir().join(format!("sdroxide-history-it-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        // SAFETY: first thing every test here does, before any thread of ours
        // reads the environment.
        unsafe {
            std::env::set_var("SDROXIDE_CONFIG_DIR", &dir);
            // Off unless the operator says so; these tests are the operator.
            std::env::set_var("SDROXIDE_AIRCRAFT_HISTORY", "1");
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

/// The JSON of an answer, whatever the framing round it.
fn body(answer: &str) -> serde_json::Value {
    let start = answer.find('{').expect("a JSON body");
    let end = answer.rfind('}').expect("a JSON body");
    serde_json::from_str(&answer[start..=end]).expect("valid JSON")
}

fn sign_in(port: u16) -> String {
    let b = serde_json::json!({ "username": USER, "password": PASS, "remember": false });
    let resp = ureq::post(&format!("http://127.0.0.1:{port}/signin"))
        .header("content-type", "application/json")
        .send(b.to_string())
        .expect("sign in");
    resp.headers()
        .get("set-cookie")
        .and_then(|v| v.to_str().ok())
        .map(|c| c.split(';').next().unwrap_or("").to_string())
        .expect("a cookie")
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

/// What a radio's decoder reports is on the list, and the aircraft's own
/// answer has its position — through the station's door and through a proxy's
/// prefix alike. An address never heard is an empty answer, not an error.
#[tokio::test]
async fn what_a_radio_reports_is_served_on_the_list_and_per_aircraft() {
    let events = spawn_station(PORT_OPEN, false).await;
    events.send(status_with_a_plane()).unwrap();
    tokio::time::sleep(Duration::from_millis(400)).await;

    let list = get(PORT_OPEN, "/aircraft-history", None).await;
    assert!(list.starts_with("HTTP/1.1 200"), "{list}");
    assert!(list.to_ascii_lowercase().contains("application/json"), "{list}");
    assert!(list.to_ascii_lowercase().contains("cache-control: no-store"), "{list}");
    let j = body(&list);
    assert_eq!(j["status"], "ok");
    assert_eq!(j["aircraft"][0]["icao"], 0x4840D6);
    assert_eq!(j["aircraft"][0]["callsign"], "KLM1234");
    assert_eq!(j["aircraft"][0]["points"], 1);

    for path in ["/aircraft-history/4840d6", "/sdroxide/aircraft-history/4840D6"] {
        let one = get(PORT_OPEN, path, None).await;
        assert!(one.starts_with("HTTP/1.1 200"), "{path}: {one}");
        let j = body(&one);
        assert_eq!(j["journal"]["points"][0][1], 52.0, "{path}");
        assert_eq!(j["journal"]["callsigns"][0]["text"], "KLM1234", "{path}");
    }

    let none = get(PORT_OPEN, "/aircraft-history/123456", None).await;
    assert!(none.starts_with("HTTP/1.1 200"), "{none}");
    let j = body(&none);
    assert_eq!(j["status"], "ok");
    assert!(j["journal"].is_null(), "{j}");
}

/// A question that is not an address is refused with 400 before anything is read.
#[tokio::test]
async fn a_bad_address_is_refused_with_400() {
    spawn_station(PORT_BAD, false).await;
    for path in [
        "/aircraft-history/zz",
        "/aircraft-history/48400",
        "/sdroxide/aircraft-history/not-an-address",
    ] {
        let answer = get(PORT_BAD, path, None).await;
        assert!(answer.starts_with("HTTP/1.1 400"), "{path}: {answer}");
        assert!(answer.contains("\"status\":\"invalid\""), "{path}: {answer}");
    }
}

/// With a password on the station, both endpoints ask for it like the sockets
/// do — and the cookie the sign-in handed out is what answers.
#[tokio::test]
async fn behind_a_password_it_wants_the_cookie() {
    spawn_station(PORT_PASSWORD, true).await;
    for path in ["/aircraft-history", "/aircraft-history/4840d6"] {
        let without = get(PORT_PASSWORD, path, None).await;
        assert!(without.starts_with("HTTP/1.1 401"), "{path}: {without}");
        assert!(without.contains("\"status\":\"unauthorized\""), "{without}");
        let forged = get(PORT_PASSWORD, path, Some("sdroxide_session=forged.token")).await;
        assert!(forged.starts_with("HTTP/1.1 401"), "{forged}");
    }
    let cookie = tokio::task::spawn_blocking(|| sign_in(PORT_PASSWORD)).await.unwrap();
    for path in ["/aircraft-history", "/aircraft-history/4840d6"] {
        let signed = get(PORT_PASSWORD, path, Some(&cookie)).await;
        assert!(signed.starts_with("HTTP/1.1 200"), "{path}: {signed}");
    }
}

/// The kept aircraft as a table: CSV and JSON on request, as a download, through a
/// proxy's prefix too, with no position in either, and a format that is neither is
/// a 400.
#[tokio::test]
async fn the_fleet_is_exported_as_csv_or_json_without_a_position() {
    let events = spawn_station(PORT_EXPORT, false).await;
    events.send(status_with_a_plane()).unwrap();
    tokio::time::sleep(Duration::from_millis(400)).await;

    let csv = get(PORT_EXPORT, "/aircraft-export?format=csv", None).await;
    assert!(csv.starts_with("HTTP/1.1 200"), "{csv}");
    let lower = csv.to_ascii_lowercase();
    assert!(lower.contains("content-type: text/csv"), "{csv}");
    assert!(lower.contains("content-disposition: attachment; filename=\"aircraft-"), "{csv}");
    assert!(lower.contains("cache-control: no-store"), "{csv}");
    assert!(csv.contains("icao,callsign,registration,"), "{csv}");
    assert!(csv.contains("4840D6,KLM1234,"), "{csv}");

    for path in ["/aircraft-export?format=json", "/sdroxide/aircraft-export?format=JSON"] {
        let json = get(PORT_EXPORT, path, None).await;
        assert!(json.starts_with("HTTP/1.1 200"), "{path}: {json}");
        assert!(json.to_ascii_lowercase().contains("application/json"), "{json}");
        let j = body(&json);
        assert_eq!(j["aircraft"][0]["icao"], "4840D6", "{path}");
        assert_eq!(j["aircraft"][0]["callsign"], "KLM1234", "{path}");
        assert_eq!(j["aircraft"][0]["positions_kept"], 1, "{path}");
    }
    for answer in [&csv, &get(PORT_EXPORT, "/aircraft-export?format=json", None).await] {
        for banned in ["52.0", "35000", "\"lat\"", "\"lon\"", "altitude"] {
            assert!(!answer.contains(banned), "`{banned}` in {answer}");
        }
    }

    for path in ["/aircraft-export", "/aircraft-export?format=xml", "/aircraft-export?format="] {
        let bad = get(PORT_EXPORT, path, None).await;
        assert!(bad.starts_with("HTTP/1.1 400"), "{path}: {bad}");
        assert!(bad.contains("\"status\":\"invalid\""), "{path}: {bad}");
    }
}

/// Behind a password the export wants the cookie like everything else.
#[tokio::test]
async fn the_export_behind_a_password_wants_the_cookie() {
    spawn_station(PORT_EXPORT_PASSWORD, true).await;
    let without = get(PORT_EXPORT_PASSWORD, "/aircraft-export?format=csv", None).await;
    assert!(without.starts_with("HTTP/1.1 401"), "{without}");
    let forged = get(
        PORT_EXPORT_PASSWORD,
        "/aircraft-export?format=csv",
        Some("sdroxide_session=forged.token"),
    )
    .await;
    assert!(forged.starts_with("HTTP/1.1 401"), "{forged}");
    let cookie = tokio::task::spawn_blocking(|| sign_in(PORT_EXPORT_PASSWORD)).await.unwrap();
    let signed = get(PORT_EXPORT_PASSWORD, "/aircraft-export?format=csv", Some(&cookie)).await;
    assert!(signed.starts_with("HTTP/1.1 200"), "{signed}");
}
