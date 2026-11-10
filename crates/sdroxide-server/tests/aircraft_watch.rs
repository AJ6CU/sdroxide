//! `/aircraft-watch` on a real station: that the operator's list is kept and
//! served where the clients look for it, behind the same door as the sockets,
//! through a proxy's prefix, and still there when the station starts again.
//!
//! Nothing here reaches the internet.

use std::sync::Once;
use std::time::Duration;

use sdroxide_server::{AccessFn, RadioParams, ServerParams, serve};
use sdroxide_types::{RadioEvent, RemoteAccess, SpectrumFrame};

const USER: &str = "f6kim";
const PASS: &str = "correct horse battery staple";

/// One port per test: they run side by side.
const PORT_OPEN: u16 = 39500;
const PORT_PASSWORD: u16 = 39501;
const PORT_BAD: u16 = 39502;

/// The station reads and writes its history in the config directory; this
/// test's is a folder of its own, so the operator's is never touched.
fn isolated_config() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let dir = std::env::temp_dir().join(format!("sdroxide-watch-it-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        // SAFETY: first thing every test here does, before any thread of ours
        // reads the environment.
        unsafe {
            std::env::set_var("SDROXIDE_CONFIG_DIR", &dir);
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

async fn post(port: u16, path: &str, cookie: Option<&str>, json: &str) -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut sock = tokio::net::TcpStream::connect(("127.0.0.1", port)).await.expect("connect");
    let cookie = cookie.map(|c| format!("Cookie: {c}\r\n")).unwrap_or_default();
    let req = format!(
        "POST {path} HTTP/1.1\r\nHost: 127.0.0.1\r\n{cookie}Content-Type: application/json\r\n\
         Content-Length: {}\r\nConnection: close\r\n\r\n{json}",
        json.len()
    );
    sock.write_all(req.as_bytes()).await.expect("write");
    let mut body = String::new();
    tokio::time::timeout(Duration::from_secs(5), sock.read_to_string(&mut body))
        .await
        .expect("timeout reading the answer")
        .expect("read");
    body
}

/// The list starts empty, an add is answered with the list and kept, the same
/// list is read back through a proxy's prefix, and a remove takes it off.
#[tokio::test]
async fn the_list_is_added_to_read_back_and_removed_from() {
    spawn_station(PORT_OPEN, false).await;
    let empty = get(PORT_OPEN, "/aircraft-watch", None).await;
    assert!(empty.starts_with("HTTP/1.1 200"), "{empty}");
    assert!(empty.to_ascii_lowercase().contains("application/json"), "{empty}");
    assert!(empty.to_ascii_lowercase().contains("cache-control: no-store"), "{empty}");
    assert_eq!(body(&empty)["entries"].as_array().unwrap().len(), 0);

    let added = post(PORT_OPEN, "/aircraft-watch", None, r#"{"add":"ph-bxa"}"#).await;
    assert!(added.starts_with("HTTP/1.1 200"), "{added}");
    let j = body(&added);
    assert_eq!((j["status"].as_str(), j["outcome"].as_str()), (Some("ok"), Some("added")));
    assert_eq!(j["entries"][0], "PHBXA");

    let again = post(PORT_OPEN, "/aircraft-watch", None, r#"{"add":"PHBXA"}"#).await;
    assert_eq!(body(&again)["outcome"], "already");

    for path in ["/aircraft-watch", "/shack/sdr/aircraft-watch"] {
        let list = get(PORT_OPEN, path, None).await;
        assert!(list.starts_with("HTTP/1.1 200"), "{path}: {list}");
        assert_eq!(body(&list)["entries"][0], "PHBXA", "{path}");
    }
    let through = post(PORT_OPEN, "/shack/sdr/aircraft-watch", None, r#"{"add":"KLM1234"}"#).await;
    assert_eq!(body(&through)["outcome"], "added", "{through}");

    let gone = post(PORT_OPEN, "/aircraft-watch", None, r#"{"remove":"phbxa"}"#).await;
    let j = body(&gone);
    assert_eq!(j["outcome"], "removed");
    assert_eq!(j["entries"].as_array().unwrap().len(), 1);
    assert_eq!(j["entries"][0], "KLM1234");
}

/// A body that is not an edit, or an edit that is not one thing, is a 400 and
/// the list is as it was.
#[tokio::test]
async fn a_bad_edit_is_refused_with_400_and_changes_nothing() {
    spawn_station(PORT_BAD, false).await;
    for json in [
        "garbage",
        "[]",
        r#"{"add":7}"#,
        r#"{"add":"x"}"#,
        "{}",
        r#"{"add":"KLM1","remove":"KLM1"}"#,
    ] {
        let answer = post(PORT_BAD, "/aircraft-watch", None, json).await;
        assert!(answer.starts_with("HTTP/1.1 400"), "{json}: {answer}");
    }
    let list = get(PORT_BAD, "/aircraft-watch", None).await;
    assert_eq!(body(&list)["entries"].as_array().unwrap().len(), 0);
}

/// With a password on the station, both the list and an edit want the cookie
/// the sign-in handed out, and an edit without it changes nothing.
#[tokio::test]
async fn behind_a_password_it_wants_the_cookie_to_read_and_to_change() {
    spawn_station(PORT_PASSWORD, true).await;
    let without = get(PORT_PASSWORD, "/aircraft-watch", None).await;
    assert!(without.starts_with("HTTP/1.1 401"), "{without}");
    assert!(without.contains("\"status\":\"unauthorized\""), "{without}");
    let forged = get(PORT_PASSWORD, "/aircraft-watch", Some("sdroxide_session=forged.token")).await;
    assert!(forged.starts_with("HTTP/1.1 401"), "{forged}");
    let edit = post(PORT_PASSWORD, "/aircraft-watch", None, r#"{"add":"KLM1234"}"#).await;
    assert!(edit.starts_with("HTTP/1.1 401"), "{edit}");

    let cookie = tokio::task::spawn_blocking(|| sign_in(PORT_PASSWORD)).await.unwrap();
    let list = get(PORT_PASSWORD, "/aircraft-watch", Some(&cookie)).await;
    assert!(list.starts_with("HTTP/1.1 200"), "{list}");
    assert_eq!(body(&list)["entries"].as_array().unwrap().len(), 0, "the refused edit did nothing");
    let ok = post(PORT_PASSWORD, "/aircraft-watch", Some(&cookie), r#"{"add":"KLM1234"}"#).await;
    assert!(ok.starts_with("HTTP/1.1 200"), "{ok}");
    assert_eq!(body(&ok)["entries"][0], "KLM1234");
}
