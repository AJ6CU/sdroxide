//! The PROGRAMS window's state, kept by the station: `GET` and `POST
//! /programs-state`.
//!
//! TOUT, the starred stations, the filters, the sort and the colours belong to
//! the station, never to a browser: the same window opens on the phone and on
//! the desk, and clearing a browser's data loses nothing. The body is a
//! [`sdroxide_types::ProgramsState`] as JSON; a `POST` replaces the whole state
//! (the last write wins, as it does for any setting) and the answer is the state
//! as stored. Like the watch list it wants the station's sign-in, to read and to
//! change, and reaches nothing outside its own file.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};

use sdroxide_types::ProgramsState;

use crate::Station;

/// The file, in the config directory.
pub(crate) const FILE: &str = "programs-state.json";

/// The most bytes of a state that is read: every station of the schedules
/// starred, with room to spare.
pub(crate) const MAX_BODY_BYTES: usize = 1024 * 1024;

pub(crate) struct ProgramsHub {
    state: Mutex<ProgramsState>,
    file: Option<PathBuf>,
}

impl ProgramsHub {
    /// From the config directory.
    pub(crate) fn start() -> ProgramsHub {
        ProgramsHub::open(sdroxide_config::config_dir().ok().map(|d| d.join(FILE)))
    }

    /// A hub over `file`, which is read now. No file, or one that is not a state,
    /// is the default state; without a path it lives in memory only.
    pub(crate) fn open(file: Option<PathBuf>) -> ProgramsHub {
        let state = file
            .as_ref()
            .and_then(|f| std::fs::read_to_string(f).ok())
            .and_then(|text| serde_json::from_str(&text).ok())
            .unwrap_or_default();
        ProgramsHub { state: Mutex::new(state), file }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, ProgramsState> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub(crate) fn get(&self) -> ProgramsState {
        self.lock().clone()
    }

    /// Replace the state; it is written to the file before the answer goes back,
    /// and only when it changed.
    pub(crate) fn set(&self, new: ProgramsState) -> ProgramsState {
        let mut state = self.lock();
        if *state != new {
            *state = new;
            if let Some(path) = &self.file {
                let text = serde_json::to_string(&*state).unwrap_or_default();
                if let Err(e) = crate::history::write_atomic(path, &text) {
                    tracing::warn!(error = %e, "could not write the PROGRAMS state");
                }
            }
        }
        state.clone()
    }
}

/// Whether the request is turned away: the station wants a sign-in and the
/// request has none.
pub(crate) fn refused(sign_in_needed: bool, signed_in: bool) -> bool {
    sign_in_needed && !signed_in
}

/// The state in a request's body, if it is one.
pub(crate) fn parse(body: &[u8]) -> Option<ProgramsState> {
    if body.len() > MAX_BODY_BYTES {
        return None;
    }
    serde_json::from_slice(body).ok()
}

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

fn state_json(s: &ProgramsState) -> String {
    serde_json::to_string(s).unwrap_or_default()
}

pub(crate) async fn get_route(State(station): State<Arc<Station>>, headers: HeaderMap) -> Response {
    if turned_away(&station, &headers) {
        return json(StatusCode::UNAUTHORIZED, String::new());
    }
    json(StatusCode::OK, state_json(&station.programs.get()))
}

pub(crate) async fn post_route(
    State(station): State<Arc<Station>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if turned_away(&station, &headers) {
        return json(StatusCode::UNAUTHORIZED, String::new());
    }
    match parse(&body) {
        Some(new) => json(StatusCode::OK, state_json(&station.programs.set(new))),
        None => json(StatusCode::BAD_REQUEST, String::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let d =
            std::env::temp_dir().join(format!("sdroxide-programs-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d.join(FILE)
    }

    fn lit() -> ProgramsState {
        let mut s = ProgramsState::default();
        s.filters.all = true;
        s.favourites = vec!["BBC".into(), "RFI".into()];
        s.colour_all = Some([1, 2, 3]);
        s.colours.insert("BBC".into(), [9, 8, 7]);
        s
    }

    #[test]
    fn a_new_hub_starts_on_the_default_state() {
        assert_eq!(ProgramsHub::open(Some(tmp("new"))).get(), ProgramsState::default());
    }

    #[test]
    fn what_was_written_is_there_for_the_next_run() {
        let path = tmp("again");
        let hub = ProgramsHub::open(Some(path.clone()));
        assert_eq!(hub.set(lit()), lit());
        assert!(std::fs::read_to_string(&path).unwrap().contains("BBC"));
        let again = ProgramsHub::open(Some(path));
        assert_eq!(again.get(), lit(), "TOUT, the stars and the colours are the station's");
        assert!(again.get().filters.all);
    }

    #[test]
    fn the_same_state_again_does_not_touch_the_file() {
        let path = tmp("quiet");
        let hub = ProgramsHub::open(Some(path.clone()));
        hub.set(lit());
        let before = std::fs::metadata(&path).unwrap().modified().unwrap();
        std::fs::remove_file(&path).unwrap();
        hub.set(lit());
        assert!(!path.exists(), "unchanged, so unwritten ({before:?})");
    }

    #[test]
    fn a_damaged_file_starts_the_default_state_and_is_replaced() {
        let path = tmp("damaged");
        std::fs::write(&path, "not json").unwrap();
        let hub = ProgramsHub::open(Some(path.clone()));
        assert_eq!(hub.get(), ProgramsState::default());
        hub.set(lit());
        assert!(std::fs::read_to_string(&path).unwrap().contains("BBC"));
    }

    #[test]
    fn without_a_path_it_lives_in_memory() {
        let hub = ProgramsHub::open(None);
        hub.set(lit());
        assert_eq!(hub.get(), lit());
    }

    #[test]
    fn the_sign_in_is_asked_only_when_the_station_wants_one() {
        assert!(!refused(false, false));
        assert!(!refused(true, true));
        assert!(refused(true, false));
    }

    #[test]
    fn a_body_is_a_state_or_it_is_refused() {
        let text = serde_json::to_string(&lit()).unwrap();
        assert_eq!(parse(text.as_bytes()), Some(lit()));
        assert_eq!(parse(b"{}"), Some(ProgramsState::default()));
        for bad in [&b""[..], b"garbage", b"[1,2]", &[0xff, 0xfe]] {
            assert_eq!(parse(bad), None, "{bad:?}");
        }
        let long = vec![b' '; MAX_BODY_BYTES + 1];
        assert_eq!(parse(&long), None, "longer than any state needs");
    }
}
