//! `POST /broadcast-schedule/import` on a real station: the route the browser
//! client posts a picked file to. Before it was registered the station answered
//! with its own web page (status 200), and the Programmes panel printed that page
//! as the result of the import.
//!
//! Nothing here reaches the internet.

use std::sync::Once;
use std::time::Duration;

use sdroxide_server::{RadioParams, ServerParams, serve};
use sdroxide_types::SpectrumFrame;

const PORT: u16 = 39510;

/// An Aoki package's schedule (short-wave rows), the file the operator picks.
const AOKI: &[u8] = include_bytes!("../../sdroxide-config/tests/data/aoki/nzb25.txt");

fn isolated_config() {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| {
        let dir = std::env::temp_dir().join(format!("sdroxide-import-it-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        // SAFETY: first thing the test does, before any thread of ours reads the environment.
        unsafe {
            std::env::set_var("SDROXIDE_CONFIG_DIR", &dir);
        }
    });
}

async fn spawn_station(port: u16) {
    isolated_config();
    let frame = SpectrumFrame {
        seq: 0,
        center_hz: 6_000_000.0,
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
    let (_event_tx, event_rx) = crossbeam_channel::unbounded();
    let (_audio_tx, audio_rx) = rtrb::RingBuffer::<f32>::new(96_000);
    let (mic_tx, _mic_rx) = rtrb::RingBuffer::<f32>::new(48_000);
    std::mem::forget((_spec_in, _wide_in, _cmd_rx, _audio_tx, _mic_rx, _event_tx));
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
        access: None,
        probe: None,
        add_radio: None,
        remove_radio: None,
        rename_radio: None,
        radio_power: None,
        aircraft_online: Some(false),
    }));
    tokio::time::sleep(Duration::from_millis(400)).await;
}

/// (status, body) of a POST of raw bytes, as the browser client sends it.
fn post(path: &str, data: Vec<u8>) -> (u16, String) {
    let url = format!("http://127.0.0.1:{PORT}{path}");
    let agent: ureq::Agent =
        ureq::Agent::config_builder().http_status_as_error(false).build().into();
    let mut resp = agent
        .post(&url)
        .header("content-type", "application/octet-stream")
        .header("x-sdroxide", "1")
        .send(&data[..])
        .expect("the station answers");
    let status = resp.status().as_u16();
    (status, resp.body_mut().read_to_string().expect("a text body"))
}

#[tokio::test(flavor = "multi_thread")]
async fn a_picked_schedule_is_imported_and_answered_in_json() {
    spawn_station(PORT).await;

    let (status, body) =
        tokio::task::spawn_blocking(|| post("/broadcast-schedule/import", AOKI.to_vec()))
            .await
            .unwrap();
    assert_eq!(status, 200, "{body}");
    let json: serde_json::Value =
        serde_json::from_str(&body).expect("JSON, not the station's web page");
    assert_eq!(json["kind"], "schedule", "{body}");
    assert!(json["stations"].as_u64().unwrap() > 100, "{body}");
}

#[tokio::test(flavor = "multi_thread")]
async fn a_file_that_is_nothing_is_refused_with_a_reason() {
    spawn_station(PORT + 1).await;

    let (status, body) = tokio::task::spawn_blocking(|| {
        let url = format!("http://127.0.0.1:{}/broadcast-schedule/import", PORT + 1);
        let agent: ureq::Agent =
            ureq::Agent::config_builder().http_status_as_error(false).build().into();
        let mut resp = agent
            .post(&url)
            .header("content-type", "application/octet-stream")
            .send(&b"<!DOCTYPE html><html></html>"[..])
            .expect("the station answers");
        (resp.status().as_u16(), resp.body_mut().read_to_string().unwrap())
    })
    .await
    .unwrap();
    assert_eq!(status, 400, "{body}");
    let json: serde_json::Value = serde_json::from_str(&body).expect("JSON");
    assert!(json["error"].is_string(), "{body}");
}
