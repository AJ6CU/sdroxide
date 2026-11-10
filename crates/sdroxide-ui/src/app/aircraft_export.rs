//! Saving the tracked fleet as CSV or JSON.
//!
//! **Station first.** When the operator presses *Export CSV* or *Export JSON* the
//! station this client is connected to is asked for its table
//! (`/aircraft-export`, [`sdroxide_aircraft_history::export`]): the last 24 hours
//! and the relay's cards, if the station keeps a history. When there is no
//! station, or it keeps none, or it does not answer, the file is made here from
//! what the client holds — the live cards and the answers the relay already gave —
//! and the person is told which of the two they got and why.
//!
//! Either way the file holds no position, no picture or its address, and nothing
//! about the session: see the history crate's export module. The last ACARS
//! message, which is free text, is not in it either.

use std::collections::HashMap;

use sdroxide_aircraft_history::export::{Format, Row, Scope};
use sdroxide_aircraft_online::{Info, Status as InfoStatus};

use super::aircraft_history::Reason;
use crate::i18n::tr;
use super::aircraft_registry::AircraftCard;

/// How long the station is waited for before the file is made here instead.
pub(in crate::app) const FLIGHT_TIMEOUT_S: i64 = 30;
/// How long the note about what was saved stays on screen.
pub(in crate::app) const NOTICE_S: i64 = 20;

/// The rows for the cards held now and the relay's answers this session. An
/// aircraft whose card has expired still has a row if the relay had answered for
/// it. Never a position.
pub(in crate::app) fn local_rows<'a>(
    cards: impl Iterator<Item = (u32, &'a AircraftCard)>,
    infos: impl Iterator<Item = (u32, &'a Info)>,
) -> Vec<Row> {
    let mut by_icao: HashMap<u32, Row> = cards
        .map(|(icao, c)| {
            let row = Row {
                icao,
                callsign: c.flight.as_ref().map(|f| f.value.clone()),
                registration: c.registration.as_ref().map(|r| r.value.clone()),
                last_heard: Some(c.last_seen),
                radio: true,
                ..Row::default()
            };
            (icao, row)
        })
        .collect();
    for (icao, info) in infos.filter(|(_, i)| i.status == InfoStatus::Ok) {
        let air = info.aircraft.as_ref();
        let code = |p: Option<&sdroxide_aircraft_online::Airport>| p.map(|p| p.code.clone());
        let route = info.route.as_ref();
        let kind: Vec<&str> = air
            .map(|a| [a.manufacturer.as_deref(), a.type_name.as_deref()])
            .into_iter()
            .flatten()
            .flatten()
            .collect();
        let card = Row {
            icao,
            registration: air.and_then(|a| a.registration.clone()),
            type_name: (!kind.is_empty()).then(|| kind.join(" ")),
            operator: air.and_then(|a| a.operator.clone()),
            origin: code(route.and_then(|r| r.origin.as_ref())),
            destination: code(route.and_then(|r| r.destination.as_ref())),
            ..Row::default()
        };
        if card == (Row { icao, ..Row::default() }) {
            continue;
        }
        let row = by_icao.entry(icao).or_insert_with(|| Row { icao, ..Row::default() });
        row.registration = row.registration.take().or(card.registration);
        row.type_name = card.type_name;
        row.operator = card.operator;
        row.origin = card.origin;
        row.destination = card.destination;
        row.online = true;
    }
    let mut rows: Vec<Row> = by_icao.into_values().collect();
    rows.sort_by(|a, b| b.last_heard.cmp(&a.last_heard).then(a.icao.cmp(&b.icao)));
    rows
}

/// What the station's answer is worth: its table, or why it is not one.
pub(in crate::app) fn classify(
    format: Format,
    http_status: Option<u16>,
    body: &str,
) -> Result<String, Reason> {
    let Some(code) = http_status else { return Err(Reason::Offline) };
    match code {
        200 if is_table(format, body) => Ok(body.to_string()),
        401 => Err(Reason::Unauthorized),
        503 => Err(Reason::Disabled),
        _ => Err(Reason::Unknown),
    }
}

/// Whether a body is the table that was asked for. A station that predates the
/// route answers every unknown path with its page, and that is not to be saved
/// as a spreadsheet.
fn is_table(format: Format, body: &str) -> bool {
    match format {
        Format::Csv => body.starts_with("icao,callsign,"),
        Format::Json => serde_json::from_str::<serde_json::Value>(body)
            .is_ok_and(|v| v.get("v").is_some() && v.get("aircraft").is_some_and(|a| a.is_array())),
    }
}

/// What is ready to be saved.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) enum Ready {
    /// The station's table, as it sent it.
    Station(Format, String),
    /// Make it here, and say why.
    Local(Format, Reason),
}

/// A file to offer, and what to tell the person.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) struct Done {
    pub name: String,
    pub bytes: Vec<u8>,
    pub note: String,
}

pub(in crate::app) fn finish(ready: Ready, rows: impl FnOnce() -> Vec<Row>, now: i64) -> Done {
    match ready {
        Ready::Station(format, body) => {
            let name = format.file_name(now);
            let note = tr!("aircraft-export-saved-station", name = name);
            Done { name, bytes: body.into_bytes(), note }
        }
        Ready::Local(format, reason) => {
            let rows = rows();
            let name = format.file_name(now);
            let note = tr!(
                "aircraft-export-saved-local",
                reason = capitalised(&reason.note()),
                n = rows.len()
            );
            Done { name, bytes: format.render(&rows, Scope::Live, now).into_bytes(), note }
        }
    }
}

fn capitalised(s: &str) -> String {
    let mut c = s.chars();
    c.next().map_or_else(String::new, |f| f.to_uppercase().chain(c).collect())
}

/// One export at a time: pressed, asked, answered, saved.
#[derive(Default)]
pub(in crate::app) struct ExportStore {
    want: Option<Format>,
    in_flight: Option<(Format, i64)>,
    ready: Option<Ready>,
    notice: Option<(String, i64)>,
}

impl ExportStore {
    /// The button was pressed. Ignored while an export is under way.
    pub(in crate::app) fn start(&mut self, format: Format) -> bool {
        if self.busy() {
            return false;
        }
        self.want = Some(format);
        true
    }

    pub(in crate::app) fn busy(&self) -> bool {
        self.want.is_some() || self.in_flight.is_some() || self.ready.is_some()
    }

    /// The format to ask the station for now, if there is one. With no station the
    /// file is to be made here, and that is what [`Self::take_ready`] then gives.
    pub(in crate::app) fn next_fetch(&mut self, has_station: bool, now: i64) -> Option<Format> {
        if let Some((format, at)) = self.in_flight {
            if now - at >= FLIGHT_TIMEOUT_S {
                self.in_flight = None;
                self.ready = Some(Ready::Local(format, Reason::Offline));
            }
            return None;
        }
        let format = self.want.take()?;
        if !has_station {
            self.ready = Some(Ready::Local(format, Reason::NoStation));
            return None;
        }
        self.in_flight = Some((format, now));
        Some(format)
    }

    /// The station's answer. One that comes after the wait was given up is dropped.
    pub(in crate::app) fn deliver(&mut self, status: Option<u16>, body: &str, _now: i64) {
        let Some((format, _)) = self.in_flight.take() else { return };
        self.ready = Some(match classify(format, status, body) {
            Ok(table) => Ready::Station(format, table),
            Err(reason) => Ready::Local(format, reason),
        });
    }

    /// The request could not even be made (the connection is not an address): the
    /// file is made here.
    pub(in crate::app) fn give_up(&mut self, reason: Reason) {
        if let Some((format, _)) = self.in_flight.take() {
            self.ready = Some(Ready::Local(format, reason));
        }
    }

    pub(in crate::app) fn take_ready(&mut self) -> Option<Ready> {
        self.ready.take()
    }

    pub(in crate::app) fn set_notice(&mut self, text: String, now: i64) {
        self.notice = Some((text, now));
    }

    pub(in crate::app) fn notice(&self, now: i64) -> Option<&str> {
        self.notice.as_ref().filter(|(_, at)| now - at < NOTICE_S).map(|(t, _)| t.as_str())
    }
}

/// Where the request goes: the station this client is connected to.
pub(in crate::app) fn export_url(peer_url: &str, format: Format) -> Option<String> {
    let base = crate::login::http_base(&crate::login::station_key(peer_url))?;
    Some(format!("{base}/aircraft-export?format={}", format.ext()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::aircraft_registry::{Source, Stamped};
    use sdroxide_aircraft_online::{AircraftInfo, Airport, Route};

    const NOW: i64 = 1_800_000_000;

    fn stamped<T>(value: T) -> Stamped<T> {
        Stamped { value, source: Source::Adsb, at: NOW - 5 }
    }

    fn card(call: &str, reg: Option<&str>, last_seen: i64) -> AircraftCard {
        AircraftCard {
            flight: Some(stamped(call.to_string())),
            registration: reg.map(|r| stamped(r.to_string())),
            position: Some(stamped((52.3123, 4.7634))),
            altitude_ft: Some(stamped(35_000)),
            last_message: Some(stamped("FREE TEXT FROM THE COCKPIT".to_string())),
            last_seen,
            ..AircraftCard::default()
        }
    }

    fn info() -> Info {
        Info {
            aircraft: Some(AircraftInfo {
                type_name: Some("747-8".into()),
                manufacturer: Some("Boeing".into()),
                operator: Some("KLM".into()),
                registration: Some("PH-BKA".into()),
            }),
            route: Some(Route {
                origin: Some(Airport { code: "AMS".into(), name: "Schiphol".into() }),
                destination: Some(Airport { code: "JFK".into(), name: "Kennedy".into() }),
            }),
            fetched_at: Some(NOW - 60),
            ..Info::bare(InfoStatus::Ok)
        }
    }

    // ── the rows made here ────────────────────────────────────────────────

    #[test]
    fn a_live_card_is_a_radio_row_and_the_relays_answer_makes_it_online_too() {
        let (a, b) = (card("KLM1234", None, NOW - 3), card("BAW9", Some("G-ABCD"), NOW - 9));
        let i = info();
        let rows = local_rows([(1, &a), (2, &b)].into_iter(), [(1, &i)].into_iter());
        assert_eq!(rows.iter().map(|r| r.icao).collect::<Vec<_>>(), vec![1, 2], "last heard first");
        let one = &rows[0];
        assert_eq!(one.callsign.as_deref(), Some("KLM1234"));
        assert_eq!(one.registration.as_deref(), Some("PH-BKA"), "the card's, none was heard");
        assert_eq!(one.type_name.as_deref(), Some("Boeing 747-8"));
        assert_eq!((one.origin.as_deref(), one.destination.as_deref()), (Some("AMS"), Some("JFK")));
        assert_eq!((one.first_heard, one.last_heard, one.positions), (None, Some(NOW - 3), None));
        assert_eq!(one.source(), "radio + online");
        let two = &rows[1];
        assert_eq!(two.registration.as_deref(), Some("G-ABCD"));
        assert_eq!(two.source(), "radio");
        assert_eq!(two.type_name, None);
    }

    #[test]
    fn the_heard_registration_comes_before_the_relays() {
        let a = card("KLM1234", Some("PH-HEARD"), NOW);
        let i = info();
        let rows = local_rows([(1, &a)].into_iter(), [(1, &i)].into_iter());
        assert_eq!(rows[0].registration.as_deref(), Some("PH-HEARD"));
    }

    #[test]
    fn an_aircraft_whose_card_expired_keeps_its_row_from_the_relay() {
        let i = info();
        let rows = local_rows(std::iter::empty(), [(9, &i)].into_iter());
        assert_eq!(rows.len(), 1);
        assert_eq!((rows[0].source(), rows[0].last_heard), ("online", None));
    }

    #[test]
    fn an_answer_with_nothing_in_it_is_not_a_row() {
        let bare = Info::bare(InfoStatus::Unknown);
        let failed = Info { aircraft: info().aircraft, ..Info::bare(InfoStatus::Unavailable) };
        let rows = local_rows(std::iter::empty(), [(1, &bare), (2, &failed)].into_iter());
        assert!(rows.is_empty());
    }

    #[test]
    fn an_ok_answer_that_knows_nothing_is_not_a_row_either() {
        let empty = Info::bare(InfoStatus::Ok);
        let rows = local_rows(std::iter::empty(), [(1, &empty)].into_iter());
        assert!(rows.is_empty(), "{rows:?}");
    }

    #[test]
    fn nothing_held_is_no_rows_and_a_file_with_only_the_header() {
        let rows = local_rows(std::iter::empty(), std::iter::empty());
        assert!(rows.is_empty());
        let done = finish(Ready::Local(Format::Csv, Reason::NoStation), || rows, NOW);
        assert_eq!(String::from_utf8(done.bytes).unwrap().lines().count(), 1);
    }

    // ── the station's answer ──────────────────────────────────────────────

    #[test]
    fn a_table_is_accepted_and_anything_else_is_a_reason() {
        let csv = "icao,callsign,registration\n4840D6,KLM1234,\n";
        assert_eq!(classify(Format::Csv, Some(200), csv), Ok(csv.to_string()));
        let json = r#"{"v":1,"generated_utc":"x","scope":"y","aircraft":[]}"#;
        assert_eq!(classify(Format::Json, Some(200), json), Ok(json.to_string()));
        assert_eq!(classify(Format::Csv, None, ""), Err(Reason::Offline));
        assert_eq!(
            classify(Format::Csv, Some(503), r#"{"status":"disabled"}"#),
            Err(Reason::Disabled)
        );
        assert_eq!(classify(Format::Csv, Some(401), "{}"), Err(Reason::Unauthorized));
        assert_eq!(classify(Format::Csv, Some(400), "{}"), Err(Reason::Unknown));
        assert_eq!(classify(Format::Csv, Some(404), "not found"), Err(Reason::Unknown));
    }

    #[test]
    fn a_page_where_a_table_was_asked_for_is_not_saved_as_one() {
        // A station that predates the route answers every unknown path with its page.
        let page = "<!doctype html><html><body>sdroxide</body></html>";
        assert_eq!(classify(Format::Csv, Some(200), page), Err(Reason::Unknown));
        assert_eq!(classify(Format::Json, Some(200), page), Err(Reason::Unknown));
        assert_eq!(classify(Format::Csv, Some(200), ""), Err(Reason::Unknown));
        assert_eq!(classify(Format::Json, Some(200), r#"{"v":1}"#), Err(Reason::Unknown));
        assert_eq!(classify(Format::Json, Some(200), "[]"), Err(Reason::Unknown));
        // A table in the other format is not the one asked for.
        let csv = "icao,callsign,registration\n";
        assert_eq!(classify(Format::Json, Some(200), csv), Err(Reason::Unknown));
    }

    // ── the file ──────────────────────────────────────────────────────────

    #[test]
    fn the_stations_table_is_saved_as_it_came_under_a_dated_name() {
        let done = finish(
            Ready::Station(Format::Csv, "icao,x\n".into()),
            || panic!("not needed"),
            1_000_000_000,
        );
        assert_eq!(done.name, "aircraft-20010909.csv");
        assert_eq!(done.bytes, b"icao,x\n");
        assert!(
            done.note.contains("aircraft-20010909.csv") && done.note.contains("24 hours"),
            "{}",
            done.note
        );
    }

    #[test]
    fn made_here_it_says_why_and_how_many() {
        let a = card("KLM1234", None, NOW);
        let rows = || local_rows([(1, &a)].into_iter(), std::iter::empty());
        for (reason, words) in [
            (Reason::NoStation, "History is kept by the station you are connected to"),
            (Reason::Disabled, "The station has not switched its history on"),
            (Reason::Offline, "The station did not answer"),
        ] {
            let done = finish(Ready::Local(Format::Json, reason), rows, NOW);
            assert!(done.name.ends_with(".json"), "{}", done.name);
            assert!(done.note.starts_with(words), "{}", done.note);
            assert!(
                done.note.contains("1 aircraft heard in the last ten minutes"),
                "{}",
                done.note
            );
            let v: serde_json::Value = serde_json::from_slice(&done.bytes).unwrap();
            assert_eq!(v["scope"], Scope::Live.label());
            assert_eq!(v["aircraft"].as_array().unwrap().len(), 1);
        }
    }

    #[test]
    fn what_is_made_here_holds_no_position_no_message_and_no_address() {
        let a = card("KLM1234", Some("PH-BXA"), NOW);
        let mut i = info();
        i.photo = None;
        let rows = || local_rows([(1, &a)].into_iter(), [(1, &i)].into_iter());
        for format in [Format::Csv, Format::Json] {
            let done = finish(Ready::Local(format, Reason::NoStation), rows, NOW);
            let text = String::from_utf8(done.bytes).unwrap();
            assert!(text.contains("KLM1234"), "the control: it holds the callsign: {text}");
            for banned in [
                "52.31",
                "4.76",
                "35000",
                "altitude",
                "COCKPIT",
                "FREE TEXT",
                "http",
                "cookie",
                "session",
                "\"lat\"",
                "\"lon\"",
            ] {
                assert!(!text.contains(banned), "`{banned}` in {text}");
            }
        }
    }

    // ── the line to the station ───────────────────────────────────────────

    #[test]
    fn pressed_with_a_station_it_asks_once_and_the_answer_is_ready_to_save() {
        let mut s = ExportStore::default();
        assert!(s.start(Format::Csv));
        assert!(s.busy());
        assert!(!s.start(Format::Json), "one at a time");
        assert_eq!(s.next_fetch(true, NOW), Some(Format::Csv));
        assert_eq!(s.next_fetch(true, NOW), None, "asked once");
        assert!(s.busy());
        assert!(s.take_ready().is_none());
        s.deliver(Some(200), "icao,callsign,registration\n", NOW + 1);
        assert_eq!(
            s.take_ready(),
            Some(Ready::Station(Format::Csv, "icao,callsign,registration\n".into()))
        );
        assert!(!s.busy(), "and free again");
        assert!(s.start(Format::Json));
    }

    #[test]
    fn with_no_station_nothing_is_asked_and_the_file_is_made_here() {
        let mut s = ExportStore::default();
        assert_eq!(s.next_fetch(false, NOW), None, "nothing pressed");
        s.start(Format::Json);
        assert_eq!(s.next_fetch(false, NOW), None);
        assert_eq!(s.take_ready(), Some(Ready::Local(Format::Json, Reason::NoStation)));
    }

    #[test]
    fn a_refusal_or_a_bad_answer_makes_the_file_here() {
        for (status, body, why) in [
            (Some(503), "{}", Reason::Disabled),
            (Some(401), "{}", Reason::Unauthorized),
            (None, "", Reason::Offline),
            (Some(200), "<html>", Reason::Unknown),
        ] {
            let mut s = ExportStore::default();
            s.start(Format::Csv);
            s.next_fetch(true, NOW);
            s.deliver(status, body, NOW + 1);
            assert_eq!(s.take_ready(), Some(Ready::Local(Format::Csv, why)), "{status:?}");
        }
    }

    #[test]
    fn a_station_that_never_answers_is_given_up_on_and_a_late_answer_is_dropped() {
        let mut s = ExportStore::default();
        s.start(Format::Csv);
        s.next_fetch(true, NOW);
        assert_eq!(s.next_fetch(true, NOW + FLIGHT_TIMEOUT_S - 1), None);
        assert!(s.take_ready().is_none());
        assert_eq!(s.next_fetch(true, NOW + FLIGHT_TIMEOUT_S), None);
        assert_eq!(s.take_ready(), Some(Ready::Local(Format::Csv, Reason::Offline)));
        s.deliver(Some(200), "icao,callsign,registration\n", NOW + FLIGHT_TIMEOUT_S + 1);
        assert!(s.take_ready().is_none(), "too late: the file has been made");
        assert!(!s.busy());
    }

    #[test]
    fn a_request_that_could_not_be_made_makes_the_file_here() {
        let mut s = ExportStore::default();
        s.start(Format::Json);
        s.next_fetch(true, NOW);
        s.give_up(Reason::NoStation);
        assert_eq!(s.take_ready(), Some(Ready::Local(Format::Json, Reason::NoStation)));
        s.give_up(Reason::NoStation);
        assert!(s.take_ready().is_none(), "nothing was out, so nothing to give up");
    }

    #[test]
    fn the_note_stays_for_a_while_and_then_goes() {
        let mut s = ExportStore::default();
        assert_eq!(s.notice(NOW), None);
        s.set_notice("Saved.".into(), NOW);
        assert_eq!(s.notice(NOW + NOTICE_S - 1), Some("Saved."));
        assert_eq!(s.notice(NOW + NOTICE_S), None);
    }

    #[test]
    fn the_request_goes_to_the_connected_station_only() {
        assert_eq!(
            export_url("wss://shack.example/ws", Format::Csv).as_deref(),
            Some("https://shack.example/aircraft-export?format=csv")
        );
        assert_eq!(
            export_url("wss://shack.example/sdr/ws", Format::Json).as_deref(),
            Some("https://shack.example/sdr/aircraft-export?format=json")
        );
        assert_eq!(export_url("not an address", Format::Csv), None);
    }

    #[test]
    fn this_module_reaches_nothing_and_draws_nothing() {
        let src = include_str!("aircraft_export.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap()];
        for banned in ["ureq", "reqwest", "TcpStream", "adsbdb", "plnspttrs", "std::fs", "egui"] {
            assert!(!code.contains(banned), "aircraft_export.rs must not touch `{banned}`");
        }
        assert!(code.contains("ExportStore"), "the control: the scan sees what is there");
    }
}
