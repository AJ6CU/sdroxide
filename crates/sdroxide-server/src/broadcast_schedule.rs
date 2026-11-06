//! `/broadcast-schedule`: the station's broadcast schedule (imported and cached),
//! as JSON, for browser clients that have no filesystem of their own.
//!
//! `GET` returns the master store — every cached source merged, with the
//! operator's own entries laid over. `POST /broadcast-schedule/import` accepts
//! a published schedule (EiBi/Aoki) or a listener's own list as raw bytes and
//! files it under the same cache the desktop program uses, so it survives a
//! restart and is seen by every client.

use std::fs;

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};

use sdroxide_types::BroadcastStation;

use crate::Station;
use crate::programs_state::refused;

pub(crate) const MAX_BODY_BYTES: usize = 20 * 1024 * 1024;

fn json(status: StatusCode, body: String) -> Response {
    (
        status,
        [(header::CONTENT_TYPE, "application/json"), (header::CACHE_CONTROL, "no-store")],
        body,
    )
        .into_response()
}

fn turned_away(station: &Station, headers: &HeaderMap) -> bool {
    let needed = station.auth.required().is_some();
    let signed = crate::session_cookie::from_headers(headers, &station.auth).is_some();
    refused(needed, signed)
}

pub(crate) async fn get_route(
    State(station): State<std::sync::Arc<Station>>,
    headers: HeaderMap,
) -> Response {
    if turned_away(&station, &headers) {
        return json(StatusCode::UNAUTHORIZED, String::new());
    }
    let stations: Vec<BroadcastStation> = sdroxide_config::load_broadcast_master();
    match serde_json::to_string(&stations) {
        Ok(body) => json(StatusCode::OK, body),
        Err(_) => json(StatusCode::INTERNAL_SERVER_ERROR, String::new()),
    }
}

pub(crate) async fn post_import_route(
    State(station): State<std::sync::Arc<Station>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if turned_away(&station, &headers) {
        return json(StatusCode::UNAUTHORIZED, String::new());
    }
    if body.len() > MAX_BODY_BYTES {
        return json(StatusCode::PAYLOAD_TOO_LARGE, String::new());
    }
    if body.is_empty() {
        return json(StatusCode::BAD_REQUEST, String::new());
    }

    let tmp = std::env::temp_dir().join(format!(
        "sdroxide-import-{}-{}",
        std::process::id(),
        body.len()
    ));
    if let Err(_) = fs::write(&tmp, &body) {
        return json(StatusCode::INTERNAL_SERVER_ERROR, String::new());
    }

    let result = if let Ok(got) = sdroxide_config::import_schedule_file(&tmp) {
        serde_json::json!({
            "kind": "schedule",
            "source": format!("{:?}", got.source),
            "season": got.season,
            "stations": got.stations,
        })
    } else if let Ok(n) = sdroxide_config::import_broadcast_stations(&tmp) {
        serde_json::json!({
            "kind": "list",
            "stations": n,
        })
    } else {
        let _ = fs::remove_file(&tmp);
        return json(StatusCode::BAD_REQUEST, String::new());
    };

    let _ = fs::remove_file(&tmp);
    json(StatusCode::OK, result.to_string())
}