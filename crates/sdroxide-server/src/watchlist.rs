//! The aircraft the operator asked to be told about: `GET` and `POST
//! /aircraft-watch`.
//!
//! The list is the station's, so it is the same from the browser on the phone and
//! the program on the desk ([`sdroxide_aircraft_history::watch`] is the model).
//! It is only a list of callsigns and registrations: whether one of them is on
//! the air is the client's to see, from the cards it holds. This module asks
//! nobody for anything and reads nothing from the radios.
//!
//! Always on: unlike the history it keeps nothing about an aircraft, only what
//! the operator typed. It wants the same sign-in as the sockets when the station
//! has a password, for reading and for changing alike.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::body::Bytes;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};

use sdroxide_aircraft_history::Status;
use sdroxide_aircraft_history::watch::{Outcome, WatchEdit, WatchReply, Watchlist};

use crate::Station;

/// The file, in the config directory.
pub(crate) const FILE: &str = "aircraft-watch.json";

/// The most bytes of an edit that is read: one entry, with room to spare.
pub(crate) const MAX_EDIT_BYTES: usize = 1024;

pub(crate) struct WatchHub {
    list: Mutex<Watchlist>,
    file: Option<PathBuf>,
}

impl WatchHub {
    /// From the config directory.
    pub(crate) fn start() -> WatchHub {
        WatchHub::open(sdroxide_config::config_dir().ok().map(|d| d.join(FILE)))
    }

    /// A hub over `file`, which is read now. No file is an empty list that is
    /// kept in memory only.
    pub(crate) fn open(file: Option<PathBuf>) -> WatchHub {
        let list = file
            .as_ref()
            .and_then(|f| std::fs::read_to_string(f).ok())
            .map(|text| Watchlist::from_json(&text))
            .unwrap_or_default();
        WatchHub { list: Mutex::new(list), file }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Watchlist> {
        self.list.lock().unwrap_or_else(|e| e.into_inner())
    }

    pub(crate) fn reply(&self) -> WatchReply {
        WatchReply::of(&self.lock(), None)
    }

    /// Apply an edit; a change that took is written to the file before the
    /// answer goes back.
    pub(crate) fn edit(&self, edit: &WatchEdit) -> WatchReply {
        let mut list = self.lock();
        let outcome = edit.apply(&mut list);
        if matches!(outcome, Outcome::Added | Outcome::Removed) {
            if let Some(path) = &self.file {
                if let Err(e) = crate::history::write_atomic(path, &list.to_json()) {
                    tracing::warn!(error = %e, "could not write the aircraft watch list");
                }
            }
        }
        WatchReply::of(&list, Some(outcome))
    }
}

/// What the route answers to, decided before anything is read.
pub(crate) fn judge(sign_in_needed: bool, signed_in: bool) -> Option<Status> {
    (sign_in_needed && !signed_in).then_some(Status::Unauthorized)
}

/// The edit in a request's body, if it is one.
pub(crate) fn parse_edit(body: &[u8]) -> Option<WatchEdit> {
    if body.len() > MAX_EDIT_BYTES {
        return None;
    }
    serde_json::from_slice(body).ok()
}

fn http_status(s: Status) -> StatusCode {
    match s {
        Status::Ok => StatusCode::OK,
        Status::Disabled => StatusCode::SERVICE_UNAVAILABLE,
        Status::Invalid => StatusCode::BAD_REQUEST,
        Status::Unauthorized => StatusCode::UNAUTHORIZED,
    }
}

fn json(status: StatusCode, body: &WatchReply) -> Response {
    (
        status,
        [(header::CONTENT_TYPE, "application/json"), (header::CACHE_CONTROL, "no-store")],
        serde_json::to_string(body).unwrap_or_default(),
    )
        .into_response()
}

fn refusal(station: &Station, headers: &HeaderMap) -> Option<Status> {
    let needed = station.auth.required().is_some();
    let signed = crate::session_cookie::from_headers(headers, &station.auth).is_some();
    judge(needed, signed)
}

pub(crate) async fn get_route(State(station): State<Arc<Station>>, headers: HeaderMap) -> Response {
    match refusal(&station, &headers) {
        Some(s) => json(http_status(s), &WatchReply::bare(s)),
        None => json(StatusCode::OK, &station.watch.reply()),
    }
}

pub(crate) async fn post_route(
    State(station): State<Arc<Station>>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    if let Some(s) = refusal(&station, &headers) {
        return json(http_status(s), &WatchReply::bare(s));
    }
    match parse_edit(&body) {
        Some(edit) => {
            let reply = station.watch.edit(&edit);
            let code = if reply.outcome == Some(Outcome::Invalid) {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::OK
            };
            json(code, &reply)
        }
        None => json(StatusCode::BAD_REQUEST, &WatchReply::bare(Status::Invalid)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("sdroxide-watch-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d.join(FILE)
    }

    fn add(e: &str) -> WatchEdit {
        WatchEdit { add: Some(e.into()), remove: None }
    }

    fn remove(e: &str) -> WatchEdit {
        WatchEdit { add: None, remove: Some(e.into()) }
    }

    #[test]
    fn a_new_hub_is_an_empty_ok_list() {
        let hub = WatchHub::open(Some(tmp("empty")));
        let r = hub.reply();
        assert_eq!((r.status, r.outcome, r.entries.len()), (Status::Ok, None, 0));
    }

    #[test]
    fn an_edit_that_took_is_in_the_file_before_the_answer_and_read_back() {
        let path = tmp("persist");
        let hub = WatchHub::open(Some(path.clone()));
        let r = hub.edit(&add("ph-bxa"));
        assert_eq!((r.status, r.outcome), (Status::Ok, Some(Outcome::Added)));
        assert_eq!(r.entries, ["PHBXA"]);
        let on_disk = std::fs::read_to_string(&path).unwrap();
        assert!(on_disk.contains("PHBXA"), "{on_disk}");
        hub.edit(&add("KLM1234"));
        let again = WatchHub::open(Some(path.clone()));
        assert_eq!(again.reply().entries, ["KLM1234", "PHBXA"]);
        assert_eq!(again.edit(&remove("phbxa")).outcome, Some(Outcome::Removed));
        assert_eq!(WatchHub::open(Some(path)).reply().entries, ["KLM1234"]);
    }

    #[test]
    fn an_edit_that_changed_nothing_does_not_touch_the_file() {
        let path = tmp("quiet");
        let hub = WatchHub::open(Some(path.clone()));
        for e in [remove("KLM1234"), add("x"), WatchEdit::default()] {
            hub.edit(&e);
        }
        assert!(!path.exists(), "nothing was added, so nothing was written");
        hub.edit(&add("KLM1234"));
        let before = std::fs::read(&path).unwrap();
        assert_eq!(hub.edit(&add("klm1234")).outcome, Some(Outcome::Already));
        assert_eq!(std::fs::read(&path).unwrap(), before);
    }

    #[test]
    fn without_a_file_the_list_lives_in_memory_only() {
        let hub = WatchHub::open(None);
        assert_eq!(hub.edit(&add("KLM1234")).outcome, Some(Outcome::Added));
        assert_eq!(hub.reply().entries, ["KLM1234"]);
    }

    #[test]
    fn a_damaged_or_foreign_file_starts_an_empty_list() {
        let path = tmp("damaged");
        std::fs::write(&path, "not json at all").unwrap();
        let hub = WatchHub::open(Some(path.clone()));
        assert!(hub.reply().entries.is_empty());
        hub.edit(&add("KLM1234"));
        assert!(std::fs::read_to_string(&path).unwrap().contains("KLM1234"), "and it is replaced");
    }

    #[test]
    fn a_failed_write_leaves_the_old_file_whole_and_the_list_still_answers() {
        let path = tmp("failed");
        let hub = WatchHub::open(Some(path.clone()));
        hub.edit(&add("KLM1234"));
        let old = std::fs::read_to_string(&path).unwrap();
        // The temp file cannot be made: its name is a folder.
        let mut tmp_name = path.as_os_str().to_owned();
        tmp_name.push(".tmp");
        std::fs::create_dir(&tmp_name).unwrap();
        let r = hub.edit(&add("BAW9"));
        assert_eq!(r.status, Status::Ok);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), old, "the old file, whole");
    }

    #[test]
    fn the_sign_in_is_asked_only_when_the_station_wants_one() {
        assert_eq!(judge(false, false), None);
        assert_eq!(judge(false, true), None);
        assert_eq!(judge(true, true), None);
        assert_eq!(judge(true, false), Some(Status::Unauthorized));
    }

    #[test]
    fn an_edit_in_a_body_is_read_or_refused() {
        assert_eq!(parse_edit(br#"{"add":"KLM1"}"#), Some(add("KLM1")));
        assert_eq!(parse_edit(br#"{"remove":"KLM1"}"#), Some(remove("KLM1")));
        assert_eq!(parse_edit(b"{}"), Some(WatchEdit::default()));
        // serde reads an empty list as a struct of nothing; it is an empty edit,
        // which `apply` refuses, and the route answers 400 either way.
        assert_eq!(parse_edit(b"[]"), Some(WatchEdit::default()));
        for bad in [&b""[..], b"garbage", br#"{"add":7}"#, &[0xff, 0xfe]] {
            assert_eq!(parse_edit(bad), None, "{bad:?}");
        }
        let long = format!(r#"{{"add":"{}"}}"#, "A".repeat(MAX_EDIT_BYTES));
        assert_eq!(parse_edit(long.as_bytes()), None, "longer than any entry needs");
    }

    #[test]
    fn this_module_reaches_nothing_and_reads_nothing_from_a_radio() {
        let src = include_str!("watchlist.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap()];
        for banned in ["ureq", "reqwest", "TcpStream", "adsbdb", "plnspttrs", "RadioEvent", "Vdl2"]
        {
            assert!(!code.contains(banned), "watchlist.rs must not touch `{banned}`");
        }
        assert!(code.contains("write_atomic"), "the control: the scan sees what is there");
    }
}
