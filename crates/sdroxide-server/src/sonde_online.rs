//! `GET /sonde-online?lat=..&lon=..`: the radiosondes other stations have heard
//! near a place, asked of SondeHub once for every client of the station.
//!
//! A browser cannot ask SondeHub itself on a page served by the station, and a
//! remote native client should not each reach the internet either. So the
//! client sends the place it wants (its locator, rounded here to 0.1° so
//! nothing finer is ever sent on) and gets the list back as JSON.
//!
//! What goes out, and only this: one `GET` of SondeHub's `/sondes` with the
//! rounded place and a radius, at most once a minute for each tenth-degree
//! cell and at most [`MAX_ASKS_PER_MINUTE`] a minute in all. The reply is kept
//! in memory for [`HOLD`] and never written to disk. Nothing is uploaded, and
//! nothing is asked until a client's operator has switched the layer on.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};

use sdroxide_types::sonde_online::{OnlineSonde, parse, place_ok, tenths, url};

use crate::Station;

/// How long an answer is served again without asking SondeHub.
pub(crate) const HOLD: Duration = Duration::from_secs(60);
/// Cells kept at once; the oldest goes first.
const MAX_CELLS: usize = 32;
/// Questions to SondeHub per minute, whoever is asking.
pub(crate) const MAX_ASKS_PER_MINUTE: usize = 12;
/// The most bytes read of a reply.
const MAX_BYTES: u64 = 4 * 1024 * 1024;

/// How a request ended.
#[derive(Debug, PartialEq)]
pub(crate) enum Answer {
    /// The list, as JSON.
    Held(Arc<String>),
    /// The station's ceiling on questions.
    Busy,
    Failed,
}

/// The answers held, by cell (tenths of a degree), and the times of the
/// questions sent in the last minute.
#[derive(Default)]
pub(crate) struct SondeRelay {
    book: Mutex<Vec<((i32, i32), Instant, Arc<String>)>>,
    asks: Mutex<Vec<Instant>>,
}

impl SondeRelay {
    /// The list for a cell: the held one, else `fetch(url)`, within the ceiling.
    pub(crate) fn get_at(
        &self,
        now: Instant,
        cell: (i32, i32),
        fetch: impl FnOnce(&str) -> Result<String, String>,
    ) -> Answer {
        {
            let mut book = self.book.lock().unwrap_or_else(|e| e.into_inner());
            book.retain(|(_, until, _)| *until > now);
            if let Some((_, _, body)) = book.iter().find(|(c, ..)| *c == cell) {
                return Answer::Held(body.clone());
            }
        }
        {
            let mut asks = self.asks.lock().unwrap_or_else(|e| e.into_inner());
            asks.retain(|t| now.saturating_duration_since(*t) < Duration::from_secs(60));
            if asks.len() >= MAX_ASKS_PER_MINUTE {
                return Answer::Busy;
            }
            asks.push(now);
        }
        let address = url(cell.0 as f64 / 10.0, cell.1 as f64 / 10.0);
        match fetch(&address) {
            Ok(body) => {
                let body = Arc::new(body);
                let mut book = self.book.lock().unwrap_or_else(|e| e.into_inner());
                book.retain(|(c, ..)| *c != cell);
                book.push((cell, now + HOLD, body.clone()));
                while book.len() > MAX_CELLS {
                    book.remove(0);
                }
                Answer::Held(body)
            }
            Err(_) => Answer::Failed,
        }
    }
}

/// What the route answers before anything is sent: the cell, or the refusal.
pub(crate) fn judge(
    sign_in_needed: bool,
    signed_in: bool,
    query: &HashMap<String, String>,
) -> Result<(i32, i32), StatusCode> {
    if sign_in_needed && !signed_in {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let num = |k: &str| query.get(k).and_then(|v| v.trim().parse::<f64>().ok());
    match (num("lat"), num("lon")) {
        (Some(lat), Some(lon)) if place_ok(lat, lon) => Ok((tenths(lat), tenths(lon))),
        _ => Err(StatusCode::BAD_REQUEST),
    }
}

/// SondeHub's list for the address, parsed and written back as the JSON the
/// clients read.
fn ask_sondehub(address: &str) -> Result<String, String> {
    let agent: ureq::Agent = ureq::Agent::config_builder()
        .timeout_connect(Some(Duration::from_secs(10)))
        .timeout_global(Some(Duration::from_secs(30)))
        .http_status_as_error(false)
        .build()
        .into();
    let mut resp = agent
        .get(address)
        .header("User-Agent", "sdroxide-brown")
        .header("Accept", "application/json")
        .call()
        .map_err(|e| e.to_string())?;
    let status = resp.status().as_u16();
    if status != 200 {
        return Err(format!("HTTP {status}"));
    }
    let body = resp
        .body_mut()
        .with_config()
        .limit(MAX_BYTES)
        .read_to_string()
        .map_err(|e| e.to_string())?;
    let list: Vec<OnlineSonde> = parse(&body);
    serde_json::to_string(&list).map_err(|e| e.to_string())
}

pub(crate) async fn route(
    State(station): State<Arc<Station>>,
    Query(query): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Response {
    let needed = station.auth.required().is_some();
    let signed = crate::session_cookie::from_headers(&headers, &station.auth).is_some();
    let plain =
        |status: StatusCode| (status, [(header::CACHE_CONTROL, "no-store")]).into_response();
    let cell = match judge(needed, signed, &query) {
        Ok(c) => c,
        Err(status) => return plain(status),
    };
    let relay = station.sonde.clone();
    let got = tokio::task::spawn_blocking(move || relay.get_at(Instant::now(), cell, ask_sondehub))
        .await
        .unwrap_or(Answer::Failed);
    match got {
        Answer::Held(body) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json"), (header::CACHE_CONTROL, "no-store")],
            body.as_str().to_string(),
        )
            .into_response(),
        Answer::Busy => plain(StatusCode::TOO_MANY_REQUESTS),
        Answer::Failed => plain(StatusCode::BAD_GATEWAY),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    fn q(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(a, b)| (a.to_string(), b.to_string())).collect()
    }

    #[test]
    fn the_place_is_rounded_to_a_tenth_and_nonsense_is_refused() {
        assert_eq!(
            judge(false, false, &q(&[("lat", "48.8566"), ("lon", "2.3522")])),
            Ok((489, 24))
        );
        for bad in [
            q(&[]),
            q(&[("lat", "48")]),
            q(&[("lat", "x"), ("lon", "2")]),
            q(&[("lat", "95"), ("lon", "2")]),
            q(&[("lat", "NaN"), ("lon", "2")]),
        ] {
            assert_eq!(judge(false, false, &bad), Err(StatusCode::BAD_REQUEST));
        }
    }

    #[test]
    fn a_station_with_a_password_wants_the_sign_in_first() {
        let ok = q(&[("lat", "1"), ("lon", "2")]);
        assert_eq!(judge(true, false, &ok), Err(StatusCode::UNAUTHORIZED));
        assert!(judge(true, true, &ok).is_ok());
    }

    #[test]
    fn one_question_serves_every_client_of_a_cell_for_a_minute() {
        let relay = SondeRelay::default();
        let t0 = Instant::now();
        let asked = Cell::new(0);
        let fetch = |a: &str| {
            asked.set(asked.get() + 1);
            assert_eq!(a, "https://api.v2.sondehub.org/sondes?lat=48.9&lon=2.4&distance=400000");
            Ok("[]".to_string())
        };
        assert!(matches!(relay.get_at(t0, (489, 24), fetch), Answer::Held(_)));
        let soon = t0 + Duration::from_secs(59);
        assert!(matches!(relay.get_at(soon, (489, 24), fetch), Answer::Held(_)));
        assert_eq!(asked.get(), 1);
        let later = t0 + Duration::from_secs(61);
        assert!(matches!(relay.get_at(later, (489, 24), fetch), Answer::Held(_)));
        assert_eq!(asked.get(), 2);
    }

    #[test]
    fn the_ceiling_stops_a_station_being_used_to_hammer_sondehub() {
        let relay = SondeRelay::default();
        let t0 = Instant::now();
        for i in 0..MAX_ASKS_PER_MINUTE as i32 {
            assert!(matches!(relay.get_at(t0, (i, 0), |_| Ok("[]".into())), Answer::Held(_)));
        }
        assert_eq!(relay.get_at(t0, (999, 0), |_| Ok("[]".into())), Answer::Busy);
        // A minute later the room is back.
        let later = t0 + Duration::from_secs(61);
        assert!(matches!(relay.get_at(later, (999, 0), |_| Ok("[]".into())), Answer::Held(_)));
    }

    #[test]
    fn a_failure_is_not_kept() {
        let relay = SondeRelay::default();
        let t0 = Instant::now();
        assert_eq!(relay.get_at(t0, (1, 1), |_| Err("down".into())), Answer::Failed);
        assert!(matches!(relay.get_at(t0, (1, 1), |_| Ok("[]".into())), Answer::Held(_)));
    }
}
