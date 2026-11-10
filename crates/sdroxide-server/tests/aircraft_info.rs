//! `/aircraft-info/{icao}` on a real station: that it is wired where the
//! clients look for it, behind the same door as the sockets, and shut until the
//! operator opens it.
//!
//! What it answers when open — the airframe, the route, the credit — is the
//! relay's, tested beside it against a stand-in for the two services. Nothing
//! here reaches the internet: a question that would is never asked, because
//! these stations are either shut or refuse the question first.

use std::time::Duration;

use sdroxide_server::{AccessFn, RadioParams, ServerParams, serve};
use sdroxide_types::{RemoteAccess, SpectrumFrame};

const USER: &str = "f6kim";
const PASS: &str = "correct horse battery staple";

/// One port per test: they run side by side.
const PORT_SHUT: u16 = 39481;
const PORT_PASSWORD: u16 = 39482;
const PORT_OPEN: u16 = 39483;

async fn spawn_station(port: u16, password: bool, open: bool) {
    // No engine: this endpoint never touches the radio, so the station's one
    // radio is a set of channels nobody is on the other end of.
    let frame = SpectrumFrame {
        seq: 0,
        center_hz: 14_200_000.0,
        span_hz: 1_536_000.0,
        db_floor: -120.0,
        db_ceil: 0.0,
        bins: Vec::new(),
        rows: Vec::new(),
        rows_clocked: false,
    };
    let (_spec_in, spectrum_out) = triple_buffer::triple_buffer(&frame);
    let (_wide_in, wide_spectrum_out) = triple_buffer::triple_buffer(&frame);
    let (cmd_tx, _cmd_rx) = crossbeam_channel::unbounded();
    let (_event_tx, event_rx) = crossbeam_channel::unbounded();
    let (_audio_tx, audio_rx) = rtrb::RingBuffer::<f32>::new(96_000);
    let (mic_tx, _mic_rx) = rtrb::RingBuffer::<f32>::new(48_000);
    // Kept alive for the life of the test: a dropped end would stop the pump.
    std::mem::forget((_spec_in, _wide_in, _cmd_rx, _event_tx, _audio_tx, _mic_rx));
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
        aircraft_online: Some(open),
    }));
    tokio::time::sleep(Duration::from_millis(400)).await;
}

/// A bare HTTP GET, optionally carrying a cookie, handing back the whole answer.
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

fn sign_in(port: u16) -> String {
    let body = serde_json::json!({ "username": USER, "password": PASS, "remember": false });
    let resp = ureq::post(&format!("http://127.0.0.1:{port}/signin"))
        .header("content-type", "application/json")
        .send(body.to_string())
        .expect("sign in");
    resp.headers()
        .get("set-cookie")
        .and_then(|v| v.to_str().ok())
        .map(|c| c.split(';').next().unwrap_or("").to_string())
        .expect("a cookie")
}

/// A station whose operator has not switched it on says so, with a 503 and a
/// JSON body a client can read — and says it through a proxy's prefix too.
#[tokio::test]
async fn a_station_that_has_not_switched_it_on_says_disabled() {
    spawn_station(PORT_SHUT, false, false).await;
    for path in [
        "/aircraft-info/484006",
        "/sdroxide/aircraft-info/484006",
        "/shack/sdr/aircraft-info/4ca1fa?callsign=RYR123",
    ] {
        let answer = get(PORT_SHUT, path, None).await;
        assert!(answer.starts_with("HTTP/1.1 503"), "{path}: {answer}");
        assert!(answer.contains("\"status\":\"disabled\""), "{path}: {answer}");
        assert!(answer.to_ascii_lowercase().contains("application/json"), "{path}: {answer}");
        assert!(
            answer.to_ascii_lowercase().contains("cache-control: no-store"),
            "{path}: {answer}"
        );
    }
}

/// With a password on the station, the endpoint asks for it like the sockets
/// do — and the cookie the sign-in handed out is what answers.
#[tokio::test]
async fn behind_a_password_it_wants_the_cookie() {
    spawn_station(PORT_PASSWORD, true, false).await;

    let without = get(PORT_PASSWORD, "/aircraft-info/484006", None).await;
    assert!(without.starts_with("HTTP/1.1 401"), "{without}");
    assert!(without.contains("\"status\":\"unauthorized\""), "{without}");

    let forged =
        get(PORT_PASSWORD, "/aircraft-info/484006", Some("sdroxide_session=forged.token")).await;
    assert!(forged.starts_with("HTTP/1.1 401"), "{forged}");

    // ureq blocks and the station is on this very runtime, so it is asked from a thread of its own.
    let cookie = tokio::task::spawn_blocking(|| sign_in(PORT_PASSWORD)).await.unwrap();
    let signed = get(PORT_PASSWORD, "/aircraft-info/484006", Some(&cookie)).await;
    // Through the door, and then shut: the switch, not the password, answers.
    assert!(signed.starts_with("HTTP/1.1 503"), "{signed}");
    assert!(signed.contains("\"status\":\"disabled\""), "{signed}");
}

/// An open station refuses a question that is not one before it asks anybody.
#[tokio::test]
async fn an_open_station_refuses_a_bad_question_with_400() {
    spawn_station(PORT_OPEN, false, true).await;
    for path in [
        "/aircraft-info/zz",
        "/aircraft-info/48400",
        "/aircraft-info/484006?callsign=KLM%20123",
        "/aircraft-info/484006?reg=PH%2FBXA",
        "/sdroxide/aircraft-info/not-an-address",
    ] {
        let answer = get(PORT_OPEN, path, None).await;
        assert!(answer.starts_with("HTTP/1.1 400"), "{path}: {answer}");
        assert!(answer.contains("\"status\":\"invalid\""), "{path}: {answer}");
    }
}

/// A bare HTTP request of any method, handing back the whole answer.
async fn send(port: u16, method: &str, path: &str, extra: &str) -> String {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut sock = tokio::net::TcpStream::connect(("127.0.0.1", port)).await.expect("connect");
    let req = format!(
        "{method} {path} HTTP/1.1\r\nHost: 127.0.0.1\r\n{extra}Content-Length: 0\r\nConnection: close\r\n\r\n"
    );
    sock.write_all(req.as_bytes()).await.expect("write");
    let mut body = String::new();
    tokio::time::timeout(Duration::from_secs(5), sock.read_to_string(&mut body))
        .await
        .expect("timeout reading the answer")
        .expect("read");
    body
}

const PORT_PHOTO_SHUT: u16 = 39484;
const PORT_PHOTO_PASSWORD: u16 = 39485;
const PORT_PHOTO_OPEN: u16 = 39486;

/// The picture of the open card follows the relay's switch, through a prefix too.
#[tokio::test]
async fn the_photo_is_shut_while_the_relay_is_off() {
    spawn_station(PORT_PHOTO_SHUT, false, false).await;
    for path in ["/aircraft-photo/484006", "/sdroxide/aircraft-photo/484006"] {
        let answer = get(PORT_PHOTO_SHUT, path, None).await;
        assert!(answer.starts_with("HTTP/1.1 503"), "{path}: {answer}");
        assert!(answer.to_ascii_lowercase().contains("cache-control: no-store"), "{answer}");
    }
}

/// Same door as the sockets, for the picture and for letting it go.
#[tokio::test]
async fn the_photo_and_its_release_want_the_cookie() {
    spawn_station(PORT_PHOTO_PASSWORD, true, true).await;
    let without = get(PORT_PHOTO_PASSWORD, "/aircraft-photo/484006", None).await;
    assert!(without.starts_with("HTTP/1.1 401"), "{without}");
    let release = send(PORT_PHOTO_PASSWORD, "DELETE", "/aircraft-photo", "x-sdroxide: 1\r\n").await;
    assert!(release.starts_with("HTTP/1.1 401"), "{release}");
}

/// An aircraft the relay never named a photo for is not fetched from anywhere:
/// the client gives an address of an aircraft, never of a picture. And a release
/// needs the custom header a stray page cannot send.
#[tokio::test]
async fn nothing_is_fetched_for_an_aircraft_nobody_asked_about_and_a_release_needs_the_header() {
    spawn_station(PORT_PHOTO_OPEN, false, true).await;
    let answer = get(PORT_PHOTO_OPEN, "/aircraft-photo/484006", None).await;
    assert!(answer.starts_with("HTTP/1.1 404"), "{answer}");
    let bad =
        get(PORT_PHOTO_OPEN, "/aircraft-photo/zz?url=https://t.plnspttrs.net/x.jpg", None).await;
    assert!(bad.starts_with("HTTP/1.1 400"), "{bad}");
    let bare = send(PORT_PHOTO_OPEN, "DELETE", "/aircraft-photo", "").await;
    assert!(bare.starts_with("HTTP/1.1 403"), "{bare}");
    let ok = send(PORT_PHOTO_OPEN, "DELETE", "/aircraft-photo", "x-sdroxide: 1\r\n").await;
    assert!(ok.starts_with("HTTP/1.1 204"), "{ok}");
    let one = send(PORT_PHOTO_OPEN, "DELETE", "/aircraft-photo/484006", "x-sdroxide: 1\r\n").await;
    assert!(one.starts_with("HTTP/1.1 204"), "{one}");
    let bare_one = send(PORT_PHOTO_OPEN, "DELETE", "/aircraft-photo/484006", "").await;
    assert!(bare_one.starts_with("HTTP/1.1 403"), "{bare_one}");
    let junk = send(PORT_PHOTO_OPEN, "DELETE", "/aircraft-photo/zz", "x-sdroxide: 1\r\n").await;
    assert!(junk.starts_with("HTTP/1.1 400"), "{junk}");
}

const PORT_AIRPORT_SHUT: u16 = 39487;
const PORT_AIRPORT_PASSWORD: u16 = 39488;
const PORT_AIRPORT_OPEN: u16 = 39489;

const ARTICLE: &str = "?lang=en&title=Amsterdam_Airport_Schiphol";

/// An airport's picture follows the same switch as an aircraft's, through a prefix too.
#[tokio::test]
async fn the_airport_photo_is_shut_while_the_relay_is_off() {
    spawn_station(PORT_AIRPORT_SHUT, false, false).await;
    for path in ["/airport-photo", "/sdroxide/airport-photo"] {
        let answer = get(PORT_AIRPORT_SHUT, &format!("{path}{ARTICLE}"), None).await;
        assert!(answer.starts_with("HTTP/1.1 503"), "{path}: {answer}");
        assert!(answer.to_ascii_lowercase().contains("cache-control: no-store"), "{answer}");
    }
}

/// Same door as the sockets, for the picture and for letting it go.
#[tokio::test]
async fn the_airport_photo_and_its_release_want_the_cookie() {
    spawn_station(PORT_AIRPORT_PASSWORD, true, true).await;
    let without = get(PORT_AIRPORT_PASSWORD, &format!("/airport-photo{ARTICLE}"), None).await;
    assert!(without.starts_with("HTTP/1.1 401"), "{without}");
    let release = send(
        PORT_AIRPORT_PASSWORD,
        "DELETE",
        &format!("/airport-photo{ARTICLE}"),
        "x-sdroxide: 1\r\n",
    )
    .await;
    assert!(release.starts_with("HTTP/1.1 401"), "{release}");
}

/// The client names a language and a title, never an address: anything that could
/// aim the request elsewhere is refused before the station reaches out, and a
/// release needs the custom header a stray page cannot send.
#[tokio::test]
async fn an_airport_title_that_is_not_plain_is_refused_and_a_release_needs_the_header() {
    spawn_station(PORT_AIRPORT_OPEN, false, true).await;
    for query in [
        "",
        "?lang=en",
        "?title=Paris",
        "?lang=en&title=..%2Fetc",
        "?lang=en&title=a/b",
        "?lang=eng&title=Paris",
        "?lang=en&title=https://evil.example/x.jpg",
        "?lang=en.evil.example&title=Paris",
    ] {
        let bad = get(PORT_AIRPORT_OPEN, &format!("/airport-photo{query}"), None).await;
        assert!(bad.starts_with("HTTP/1.1 400"), "{query}: {bad}");
    }
    let path = format!("/airport-photo{ARTICLE}");
    let bare = send(PORT_AIRPORT_OPEN, "DELETE", &path, "").await;
    assert!(bare.starts_with("HTTP/1.1 403"), "{bare}");
    let ok = send(PORT_AIRPORT_OPEN, "DELETE", &path, "x-sdroxide: 1\r\n").await;
    assert!(ok.starts_with("HTTP/1.1 204"), "{ok}");
    let junk =
        send(PORT_AIRPORT_OPEN, "DELETE", "/airport-photo?lang=EN&title=x", "x-sdroxide: 1\r\n")
            .await;
    assert!(junk.starts_with("HTTP/1.1 400"), "{junk}");
}

const PORT_WEATHER_SHUT: u16 = 39490;
const PORT_WEATHER_PASSWORD: u16 = 39491;
const PORT_WEATHER_OPEN: u16 = 39492;

const PLACE: &str = "?lat=52.31&lon=4.76";

/// An airport's weather follows the same switch as the aircraft's, through a prefix too.
#[tokio::test]
async fn the_airport_weather_is_shut_while_the_relay_is_off() {
    spawn_station(PORT_WEATHER_SHUT, false, false).await;
    for path in ["/airport-weather", "/sdroxide/airport-weather"] {
        let answer = get(PORT_WEATHER_SHUT, &format!("{path}{PLACE}"), None).await;
        assert!(answer.starts_with("HTTP/1.1 503"), "{path}: {answer}");
        assert!(answer.to_ascii_lowercase().contains("cache-control: no-store"), "{answer}");
    }
}

/// Same door as the sockets.
#[tokio::test]
async fn the_airport_weather_wants_the_cookie() {
    spawn_station(PORT_WEATHER_PASSWORD, true, true).await;
    let without = get(PORT_WEATHER_PASSWORD, &format!("/airport-weather{PLACE}"), None).await;
    assert!(without.starts_with("HTTP/1.1 401"), "{without}");
}

/// The client names a place and nothing else: what is not two numbers on the
/// globe is refused before the station reaches out.
#[tokio::test]
async fn a_place_that_is_not_on_the_globe_is_refused_before_anything_goes_out() {
    spawn_station(PORT_WEATHER_OPEN, false, true).await;
    for query in [
        "",
        "?lat=52.31",
        "?lon=4.76",
        "?lat=abc&lon=4.76",
        "?lat=91&lon=0",
        "?lat=0&lon=181",
        "?lat=NaN&lon=0",
        "?lat=52.31&lon=4.76%26x%3D1",
    ] {
        let bad = get(PORT_WEATHER_OPEN, &format!("/airport-weather{query}"), None).await;
        assert!(bad.starts_with("HTTP/1.1 400"), "{query}: {bad}");
    }
}
