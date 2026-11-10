//! The station's record of an aircraft, on the client.
//!
//! The station keeps a day of what its radios heard of each aircraft and serves
//! it on `/aircraft-history/{icao}` ([`sdroxide_aircraft_history`]). This is the
//! client's half: when to ask the station for it, what the answer means, and
//! how it is worded and drawn — as a trail on the map and as lines on the card.
//!
//! It is never asked for by itself. A person presses **Show history**, for one
//! aircraft, and only then does a request go — to the station this client is
//! already connected to, never anywhere else — and again every 30 seconds while
//! it stays on, one at a time. Nothing is stored here.

use sdroxide_aircraft_history::{AircraftReply, FORMAT_VERSION, Journal, RETENTION_S, Status};

use super::aircraft_card::icao_hex;
use crate::i18n::tr;
use super::aircraft_online::{Badge, Row};
use super::util::fmt_age;
use crate::adsb_map::TrailSeg;

/// How often the record is fetched again while it is on.
pub(in crate::app) const REFRESH_S: i64 = 30;
/// How long a request is waited for before the line is free again.
pub(in crate::app) const FLIGHT_TIMEOUT_S: i64 = 20;
/// A gap in the track this long is a flight that ended and another that began:
/// no line is drawn across it.
pub(in crate::app) const TRAIL_GAP_S: i64 = 600;

/// Why there is no record to show.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::app) enum Reason {
    /// The station's operator has not switched the history on.
    Disabled,
    /// The station wants a sign-in this client does not have.
    Unauthorized,
    /// No answer: the station is out of reach.
    Offline,
    /// This client is not connected to a station.
    NoStation,
    /// An answer that was not understood.
    Unknown,
}

impl Reason {
    /// What to tell the person.
    pub(in crate::app) fn note(self) -> String {
        match self {
            Reason::Disabled => tr!("aircraft-history-reason-disabled"),
            Reason::Unauthorized => tr!("aircraft-history-reason-unauthorized"),
            Reason::Offline => tr!("aircraft-switch-no-answer"),
            Reason::NoStation => tr!("aircraft-history-reason-no-station"),
            Reason::Unknown => tr!("aircraft-history-reason-unknown"),
        }
    }

    /// Whether asking again later could help.
    pub(in crate::app) fn retryable(self) -> bool {
        matches!(self, Reason::Offline | Reason::Unknown)
    }
}

/// What the record for the aircraft that is on looks like.
#[derive(Debug, Clone, PartialEq, Default)]
pub(in crate::app) enum HistoryState {
    #[default]
    Idle,
    Loading,
    Ready(Box<Journal>),
    Unavailable(Reason),
}

/// What the station's reply came to.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) enum Answer {
    Journal(Box<Journal>),
    /// The station is well and has nothing for this address.
    Empty,
    Unavailable(Reason),
}

/// Turn the status and body of the station's reply into an [`Answer`].
pub(in crate::app) fn classify(http_status: Option<u16>, body: &str) -> Answer {
    let Some(code) = http_status else { return Answer::Unavailable(Reason::Offline) };
    let Ok(reply) = serde_json::from_str::<AircraftReply>(body) else {
        return Answer::Unavailable(Reason::Unknown);
    };
    if reply.v != FORMAT_VERSION {
        return Answer::Unavailable(Reason::Unknown);
    }
    match reply.status {
        Status::Ok if code == 200 => match reply.journal {
            Some(j) => Answer::Journal(Box::new(j)),
            None => Answer::Empty,
        },
        Status::Disabled => Answer::Unavailable(Reason::Disabled),
        Status::Unauthorized => Answer::Unavailable(Reason::Unauthorized),
        Status::Ok | Status::Invalid => Answer::Unavailable(Reason::Unknown),
    }
}

static IDLE: HistoryState = HistoryState::Idle;

/// Which aircraft's record is on, what has come back for it, and the line to
/// the station. Memory only.
#[derive(Default)]
pub(in crate::app) struct HistoryStore {
    on: Option<u32>,
    state: HistoryState,
    /// When the last answer arrived.
    at: i64,
    in_flight: Option<(u32, i64)>,
}

impl HistoryStore {
    /// The aircraft whose record is on.
    pub(in crate::app) fn on(&self) -> Option<u32> {
        self.on
    }

    /// Put an aircraft's record on, or take it off with `None`. Changing the
    /// aircraft drops what was held for the last one.
    pub(in crate::app) fn set_on(&mut self, icao: Option<u32>) {
        if self.on != icao {
            self.on = icao;
            self.state = HistoryState::Idle;
            self.at = 0;
        }
    }

    /// Press of the button: on if it was off for this aircraft, off if on.
    pub(in crate::app) fn toggle(&mut self, icao: u32) {
        let next = if self.on == Some(icao) { None } else { Some(icao) };
        self.set_on(next);
    }

    pub(in crate::app) fn state(&self, icao: u32) -> &HistoryState {
        if self.on == Some(icao) { &self.state } else { &IDLE }
    }

    pub(in crate::app) fn journal(&self, icao: u32) -> Option<&Journal> {
        match self.state(icao) {
            HistoryState::Ready(j) => Some(j),
            _ => None,
        }
    }

    /// Whether a request is out and still being waited for.
    pub(in crate::app) fn line_busy(&self, now: i64) -> bool {
        self.in_flight.is_some_and(|(_, at)| now - at < FLIGHT_TIMEOUT_S)
    }

    /// The aircraft to fetch now, if one is due. Never while it is off, never
    /// without a station, never while another request is out.
    pub(in crate::app) fn next_request(&mut self, has_station: bool, now: i64) -> Option<u32> {
        let icao = self.on?;
        if !has_station || self.line_busy(now) {
            return None;
        }
        let since = now - self.at;
        let due = match &self.state {
            HistoryState::Idle => true,
            // Loading with the line free is a request that never came back.
            HistoryState::Loading => true,
            HistoryState::Ready(_) => since >= REFRESH_S,
            HistoryState::Unavailable(r) => r.retryable() && since >= REFRESH_S,
        };
        if !due {
            return None;
        }
        self.in_flight = Some((icao, now));
        if self.state == HistoryState::Idle {
            self.state = HistoryState::Loading;
        }
        Some(icao)
    }

    /// The station's answer for `icao`. It frees the line whichever aircraft it
    /// was for; it is kept only if that aircraft is still the one that is on.
    pub(in crate::app) fn deliver(&mut self, icao: u32, answer: Answer, now: i64) {
        if self.in_flight.is_some_and(|(i, _)| i == icao) {
            self.in_flight = None;
        }
        if self.on != Some(icao) {
            return;
        }
        self.at = now;
        self.state = match answer {
            Answer::Journal(j) => HistoryState::Ready(j),
            Answer::Empty => HistoryState::Ready(Box::new(Journal { icao, ..Journal::default() })),
            Answer::Unavailable(r) => HistoryState::Unavailable(r),
        };
    }
}

/// Where the request goes: the station this client is connected to, by its
/// socket address (`wss://host/prefix/ws/2` → `https://host/prefix/…`).
pub(in crate::app) fn history_url(peer_url: &str, icao: u32) -> Option<String> {
    let base = crate::login::http_base(&crate::login::station_key(peer_url))?;
    Some(format!("{base}/aircraft-history/{}", icao_hex(icao)))
}

/// What the button in a card offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::app) enum HistoryOffer {
    /// No button: no station, or no address to ask about.
    Hidden,
    Show,
    /// It is on; the button takes it off.
    On,
}

impl HistoryOffer {
    pub(in crate::app) fn label(self) -> String {
        match self {
            HistoryOffer::Hidden => String::new(),
            HistoryOffer::Show => tr!("aircraft-history-show"),
            HistoryOffer::On => tr!("aircraft-history-hide"),
        }
    }
}

pub(in crate::app) fn offer(on: Option<u32>, icao: Option<u32>, has_station: bool) -> HistoryOffer {
    match icao {
        Some(i) if has_station && on == Some(i) => HistoryOffer::On,
        Some(_) if has_station => HistoryOffer::Show,
        _ => HistoryOffer::Hidden,
    }
}

/// The history after the pinned card changed: the record that was turned on
/// from the pin goes with the pin. Returns what is on and who owns it.
pub(in crate::app) fn after_pin(
    on: Option<u32>,
    owner: Option<u32>,
    pinned: Option<u32>,
) -> (Option<u32>, Option<u32>) {
    match owner {
        Some(o) if pinned != Some(o) => (on.filter(|i| *i != o), None),
        _ => (on, owner),
    }
}

/// The trail of an aircraft: its kept positions as line segments, the newest the
/// most opaque. No line across a silence of ten minutes or a leap over the
/// date line.
pub(in crate::app) fn trail_segments(j: &Journal, now: i64) -> Vec<TrailSeg> {
    j.points
        .windows(2)
        .filter(|w| {
            let dt = w[1].time() - w[0].time();
            dt > 0 && dt <= TRAIL_GAP_S && (w[1].lon() - w[0].lon()).abs() <= 180.0
        })
        .map(|w| {
            let age = (now - w[1].time()).max(0) as f32;
            let faint = (age / RETENTION_S as f32).clamp(0.0, 1.0);
            TrailSeg {
                from: (f64::from(w[0].lat()), f64::from(w[0].lon())),
                to: (f64::from(w[1].lat()), f64::from(w[1].lon())),
                alpha: 0.9 - 0.75 * faint,
            }
        })
        .collect()
}

fn line(label: &'static str, value: String, note: impl Into<String>) -> Row {
    Row { label, value, badge: Badge::History, note: note.into() }
}

/// The lines of the record, for the card. A line nothing is behind is left out.
pub(in crate::app) fn rows(j: &Journal, now: i64) -> Vec<Row> {
    let kept = j.first_seen > 0
        || !j.points.is_empty()
        || !j.callsigns.is_empty()
        || !j.registrations.is_empty()
        || j.card.is_some();
    if !kept {
        return vec![Row {
            label: "History",
            value: "—".to_string(),
            badge: Badge::Missing,
            note: tr!("aircraft-history-nothing-kept"),
        }];
    }
    let by_station = tr!("aircraft-history-by-station");
    let mut out = Vec::new();
    if j.first_seen > 0 {
        out.push(line("First heard", fmt_age(now - j.first_seen), by_station.clone()));
    }
    if j.last_seen > 0 {
        out.push(line("Last heard", fmt_age(now - j.last_seen), by_station));
    }
    out.push(line("Track points", j.points.len().to_string(), tr!("aircraft-history-kept-24h")));
    if let Some(top) = j.points.iter().filter_map(|p| p.altitude_ft()).max() {
        out.push(line(
            "Highest",
            sdroxide_types::AirUnits::current().altitude(top),
            tr!("aircraft-history-of-kept"),
        ));
    }
    let joined = |list: &[sdroxide_aircraft_history::Seen]| {
        list.iter().map(|s| s.text.as_str()).collect::<Vec<_>>().join(", ")
    };
    if !j.callsigns.is_empty() {
        out.push(line("Callsigns", joined(&j.callsigns), tr!("aircraft-history-used-24h")));
    }
    if !j.registrations.is_empty() {
        out.push(line("Registrations", joined(&j.registrations), tr!("aircraft-history-seen-24h")));
    }
    if let Some(c) = &j.card {
        let note = tr!("aircraft-history-looked-up", age = fmt_age(now - c.at));
        let kind: Vec<&str> =
            [c.manufacturer.as_deref(), c.type_name.as_deref()].into_iter().flatten().collect();
        if !kind.is_empty() {
            out.push(line("Type", kind.join(" "), note.clone()));
        }
        if let Some(op) = &c.operator {
            out.push(line("Operator", op.clone(), note.clone()));
        }
        for (label, place) in [("Origin", &c.origin), ("Destination", &c.destination)] {
            if let Some(p) = place {
                out.push(line(label, format!("{} {}", p.code, p.name), note.clone()));
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdroxide_aircraft_history::{Place, Point, Seen, SeenCard};

    const NOW: i64 = 1_000_000;
    const A: u32 = 0x4840D6;
    const B: u32 = 0x3C6444;

    fn journal() -> Journal {
        Journal {
            icao: A,
            first_seen: NOW - 7200,
            last_seen: NOW - 60,
            touched: NOW - 60,
            callsigns: vec![
                Seen { text: "KLM1234".into(), first: NOW - 7200, last: NOW - 3600 },
                Seen { text: "KLM5678".into(), first: NOW - 3000, last: NOW - 60 },
            ],
            registrations: vec![Seen { text: "PH-BXA".into(), first: NOW - 7000, last: NOW - 60 }],
            category: Some("heavy".into()),
            points: vec![
                Point(NOW - 7200, 52.0, 5.0, Some(35_000), false),
                Point(NOW - 7140, 52.1, 5.1, Some(36_000), false),
                Point(NOW - 7080, 52.2, 5.2, None, false),
            ],
            card: Some(SeenCard {
                at: NOW - 600,
                type_name: Some("737-800".into()),
                manufacturer: Some("Boeing".into()),
                operator: Some("KLM".into()),
                origin: Some(Place { code: "EHAM".into(), name: "Amsterdam Schiphol".into() }),
                destination: Some(Place { code: "EGLL".into(), name: "London Heathrow".into() }),
                ..SeenCard::default()
            }),
        }
    }

    fn reply(status: Status, journal: Option<Journal>) -> String {
        serde_json::to_string(&AircraftReply { v: 1, status, journal }).unwrap()
    }

    // ── what the station said ─────────────────────────────────────────────

    #[test]
    fn a_good_reply_is_the_record_or_nothing_kept() {
        match classify(Some(200), &reply(Status::Ok, Some(journal()))) {
            Answer::Journal(j) => assert_eq!(*j, journal()),
            other => panic!("{other:?}"),
        }
        assert_eq!(classify(Some(200), &reply(Status::Ok, None)), Answer::Empty);
    }

    #[test]
    fn a_refusal_says_why() {
        assert_eq!(
            classify(Some(503), &reply(Status::Disabled, None)),
            Answer::Unavailable(Reason::Disabled)
        );
        assert_eq!(
            classify(Some(401), &reply(Status::Unauthorized, None)),
            Answer::Unavailable(Reason::Unauthorized)
        );
        assert_eq!(classify(None, ""), Answer::Unavailable(Reason::Offline), "no answer at all");
    }

    #[test]
    fn what_is_not_a_reply_is_not_understood() {
        for (status, body) in [
            (Some(200), "<html>a proxy's page</html>"),
            (Some(200), ""),
            (Some(400), &reply(Status::Invalid, None)[..]),
            (Some(500), "oops"),
        ] {
            assert_eq!(
                classify(status, body),
                Answer::Unavailable(Reason::Unknown),
                "{status:?} {body:?}"
            );
        }
        // A 200 whose body says something else than Ok is not a record either.
        assert_eq!(
            classify(Some(200), &reply(Status::Disabled, None)),
            Answer::Unavailable(Reason::Disabled)
        );
    }

    // ── when it is asked for ──────────────────────────────────────────────

    fn on_for(icao: u32) -> HistoryStore {
        let mut s = HistoryStore::default();
        s.set_on(Some(icao));
        s
    }

    #[test]
    fn nothing_is_asked_until_it_is_turned_on() {
        let mut s = HistoryStore::default();
        assert_eq!(s.next_request(true, NOW), None);
        s.set_on(Some(A));
        assert_eq!(s.next_request(true, NOW), Some(A), "the control: on, and it is asked");
    }

    #[test]
    fn nothing_is_asked_without_a_station() {
        let mut s = on_for(A);
        assert_eq!(s.next_request(false, NOW), None);
        assert_eq!(s.next_request(true, NOW), Some(A), "the control: with one it is");
    }

    #[test]
    fn one_request_at_a_time_and_none_again_while_it_is_out() {
        let mut s = on_for(A);
        assert_eq!(s.next_request(true, NOW), Some(A));
        assert!(s.line_busy(NOW + 1));
        assert_eq!(s.state(A), &HistoryState::Loading);
        assert_eq!(s.next_request(true, NOW + 1), None, "not again while it is out");
        assert_eq!(s.next_request(true, NOW + FLIGHT_TIMEOUT_S - 1), None);
        // An answer that never came frees the line after the timeout.
        assert!(!s.line_busy(NOW + FLIGHT_TIMEOUT_S));
        assert_eq!(s.next_request(true, NOW + FLIGHT_TIMEOUT_S), Some(A));
    }

    #[test]
    fn the_answer_is_held_and_fetched_again_every_thirty_seconds() {
        let mut s = on_for(A);
        s.next_request(true, NOW);
        s.deliver(A, Answer::Journal(Box::new(journal())), NOW + 2);
        assert!(!s.line_busy(NOW + 2));
        assert_eq!(s.journal(A).map(|j| j.icao), Some(A));
        assert_eq!(s.next_request(true, NOW + 2 + REFRESH_S - 1), None);
        assert_eq!(s.next_request(true, NOW + 2 + REFRESH_S), Some(A));
        // While it is fetched again the last answer is still what is shown.
        assert!(matches!(s.state(A), HistoryState::Ready(_)));
        s.deliver(A, Answer::Empty, NOW + 40);
        assert_eq!(
            s.journal(A).map(|j| j.points.len()),
            Some(0),
            "an empty record for this address"
        );
        assert_eq!(s.journal(A).map(|j| j.icao), Some(A));
    }

    #[test]
    fn a_refusal_is_not_pressed_again_but_a_silence_is_retried() {
        for reason in [Reason::Disabled, Reason::Unauthorized] {
            let mut s = on_for(A);
            s.next_request(true, NOW);
            s.deliver(A, Answer::Unavailable(reason), NOW + 1);
            assert_eq!(s.state(A), &HistoryState::Unavailable(reason));
            assert_eq!(s.next_request(true, NOW + 10_000), None, "{reason:?}");
            // Turning it off and on again is the person asking afresh.
            s.toggle(A);
            s.toggle(A);
            assert_eq!(s.next_request(true, NOW + 10_001), Some(A), "{reason:?}");
        }
        for reason in [Reason::Offline, Reason::Unknown] {
            let mut s = on_for(A);
            s.next_request(true, NOW);
            s.deliver(A, Answer::Unavailable(reason), NOW + 1);
            assert_eq!(s.next_request(true, NOW + REFRESH_S), None, "{reason:?} not at once");
            assert_eq!(s.next_request(true, NOW + 1 + REFRESH_S), Some(A), "{reason:?}");
        }
    }

    #[test]
    fn turning_it_off_drops_the_record_and_a_late_answer_for_another_aircraft_is_ignored() {
        let mut s = on_for(A);
        assert_eq!(s.on(), Some(A));
        s.next_request(true, NOW);
        s.set_on(Some(B));
        assert_eq!(s.on(), Some(B));
        assert_eq!(s.state(A), &HistoryState::Idle, "A is no longer the one that is on");
        assert_eq!(s.state(B), &HistoryState::Idle);
        // A's answer arrives after the person moved on: it frees the line, and
        // it is not B's record.
        s.deliver(A, Answer::Journal(Box::new(journal())), NOW + 3);
        assert!(!s.line_busy(NOW + 3));
        assert_eq!(s.journal(B), None);
        assert_eq!(s.journal(A), None);
        s.toggle(B);
        assert_eq!(s.on(), None);
        assert_eq!(s.next_request(true, NOW + 100), None, "off, so nothing");
    }

    #[test]
    fn toggling_turns_it_on_for_that_aircraft_and_off_again() {
        let mut s = HistoryStore::default();
        s.toggle(A);
        assert_eq!(s.on(), Some(A));
        s.toggle(B);
        assert_eq!(s.on(), Some(B), "pressing another aircraft's button moves it");
        s.toggle(B);
        assert_eq!(s.on(), None);
    }

    // ── where it goes ─────────────────────────────────────────────────────

    #[test]
    fn the_request_goes_to_the_station_the_client_is_connected_to() {
        assert_eq!(
            history_url("wss://shack.example/sdroxide/ws/2", A).as_deref(),
            Some("https://shack.example/sdroxide/aircraft-history/4840D6")
        );
        assert_eq!(
            history_url("ws://192.168.1.5:8080/ws", 0x1).as_deref(),
            Some("http://192.168.1.5:8080/aircraft-history/000001")
        );
        assert_eq!(history_url("not an address", A), None);
        assert_eq!(history_url("", A), None);
    }

    // ── the button ────────────────────────────────────────────────────────

    #[test]
    fn the_button_needs_a_station_and_an_address_and_flips_when_on() {
        assert_eq!(offer(None, Some(A), true), HistoryOffer::Show);
        assert_eq!(offer(Some(A), Some(A), true), HistoryOffer::On);
        assert_eq!(
            offer(Some(B), Some(A), true),
            HistoryOffer::Show,
            "another's is on, this one is not"
        );
        assert_eq!(offer(None, Some(A), false), HistoryOffer::Hidden);
        assert_eq!(offer(None, None, true), HistoryOffer::Hidden);
        assert_eq!(HistoryOffer::Show.label(), "Show history");
        assert_eq!(HistoryOffer::On.label(), "Hide history");
    }

    #[test]
    fn the_record_turned_on_from_a_pin_goes_with_the_pin() {
        // Turned on from the card pinned to A: A owns it.
        assert_eq!(
            after_pin(Some(A), None, Some(A)),
            (Some(A), None),
            "not from the pin, left alone"
        );
        assert_eq!(after_pin(Some(A), Some(A), Some(A)), (Some(A), Some(A)), "still pinned");
        assert_eq!(after_pin(Some(A), Some(A), Some(B)), (None, None), "the pin moved");
        assert_eq!(after_pin(Some(A), Some(A), None), (None, None), "the pin closed");
        // Something else is on by then: it is not this pin's to turn off.
        assert_eq!(after_pin(Some(B), Some(A), None), (Some(B), None));
    }

    // ── what is drawn ─────────────────────────────────────────────────────

    fn pts(list: &[(i64, f32, f32)]) -> Journal {
        Journal {
            icao: A,
            points: list.iter().map(|&(t, la, lo)| Point(t, la, lo, None, false)).collect(),
            ..Journal::default()
        }
    }

    #[test]
    fn the_trail_joins_the_kept_positions_in_order() {
        let j = pts(&[(NOW - 300, 52.0, 5.0), (NOW - 200, 52.5, 5.5), (NOW - 100, 53.0, 6.0)]);
        let t = trail_segments(&j, NOW);
        assert_eq!(t.len(), 2);
        assert_eq!((t[0].from, t[0].to), ((52.0, 5.0), (52.5, 5.5)));
        assert_eq!((t[1].from, t[1].to), ((52.5, 5.5), (53.0, 6.0)));
        assert!(trail_segments(&pts(&[(NOW, 52.0, 5.0)]), NOW).is_empty(), "one point is no line");
        assert!(trail_segments(&pts(&[]), NOW).is_empty());
    }

    #[test]
    fn the_older_the_fainter_and_never_invisible() {
        let j = pts(&[
            (NOW - 20 * 3600, 52.0, 5.0),
            (NOW - 20 * 3600 + 60, 52.1, 5.0),
            (NOW - 3600, 52.2, 5.0),
            (NOW - 3540, 52.3, 5.0),
            (NOW - 60, 52.4, 5.0),
            (NOW, 52.5, 5.0),
        ]);
        let t = trail_segments(&j, NOW);
        let alphas: Vec<f32> = t.iter().map(|s| s.alpha).collect();
        assert_eq!(alphas.len(), 3, "three runs of points, three lines: {alphas:?}");
        assert!(alphas[0] < alphas[1] && alphas[1] < alphas[2], "{alphas:?}");
        assert!(alphas.iter().all(|a| (0.1..=1.0).contains(a)), "{alphas:?}");
        let old = trail_segments(
            &pts(&[(NOW - RETENTION_S, 52.0, 5.0), (NOW - RETENTION_S + 60, 52.1, 5.0)]),
            NOW,
        );
        assert!(old[0].alpha >= 0.1, "a day old is faint, not gone: {}", old[0].alpha);
    }

    #[test]
    fn a_long_silence_or_a_leap_over_the_date_line_is_not_a_line() {
        let j = pts(&[
            (NOW - 3000, 52.0, 5.0),
            (NOW - 3000 + TRAIL_GAP_S + 1, 52.5, 5.0),
            (NOW - 3000 + TRAIL_GAP_S + 1 + 60, 53.0, 5.0),
        ]);
        let t = trail_segments(&j, NOW);
        assert_eq!(t.len(), 1, "the 10 min hole is not bridged: {t:?}");
        assert_eq!(t[0].from, (52.5, 5.0));
        let j = pts(&[(NOW - 100, 10.0, 179.9), (NOW - 50, 10.0, -179.9)]);
        assert!(trail_segments(&j, NOW).is_empty(), "no streak across the whole map");
    }

    #[test]
    fn the_record_reads_as_lines_each_marked_history() {
        let rows = rows(&journal(), NOW);
        let labels: Vec<&str> = rows.iter().map(|r| r.label).collect();
        assert_eq!(
            labels,
            [
                "First heard",
                "Last heard",
                "Track points",
                "Highest",
                "Callsigns",
                "Registrations",
                "Type",
                "Operator",
                "Origin",
                "Destination"
            ]
        );
        assert!(rows.iter().all(|r| r.badge == Badge::History), "{rows:?}");
        let value = |l: &str| rows.iter().find(|r| r.label == l).unwrap().value.clone();
        assert_eq!(value("First heard"), "2h");
        assert_eq!(value("Last heard"), "1m");
        assert_eq!(value("Track points"), "3");
        assert_eq!(value("Highest"), "36000 ft");
        assert_eq!(value("Callsigns"), "KLM1234, KLM5678");
        assert_eq!(value("Registrations"), "PH-BXA");
        assert_eq!(value("Type"), "Boeing 737-800");
        assert_eq!(value("Operator"), "KLM");
        assert_eq!(value("Origin"), "EHAM Amsterdam Schiphol");
        assert_eq!(value("Destination"), "EGLL London Heathrow");
        let note = |l: &str| rows.iter().find(|r| r.label == l).unwrap().note.clone();
        assert!(note("Type").contains("10m"), "when the relay was asked: {}", note("Type"));
    }

    #[test]
    fn lines_with_nothing_behind_them_are_left_out_and_an_empty_record_says_so() {
        let j =
            Journal { icao: A, first_seen: NOW - 100, last_seen: NOW - 50, ..Journal::default() };
        let labels: Vec<&str> = rows(&j, NOW).iter().map(|r| r.label).collect();
        assert_eq!(labels, ["First heard", "Last heard", "Track points"]);
        let none = rows(&Journal { icao: A, ..Journal::default() }, NOW);
        assert_eq!(none.len(), 1, "{none:?}");
        assert_eq!(none[0].badge, Badge::Missing);
        assert!(none[0].note.contains("nothing kept"), "{}", none[0].note);
    }

    // ── what this file may not do ─────────────────────────────────────────

    #[test]
    fn this_file_stores_nothing_and_names_no_service() {
        let src = include_str!("aircraft_history.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap()];
        for banned in [
            "localStorage",
            "sessionStorage",
            "IndexedDB",
            "web_sys",
            "adsbdb",
            "planespotters",
            "plnspttrs",
            "http://",
            "https://api",
            "ureq",
        ] {
            assert!(!code.contains(banned), "the record's client must not touch `{banned}`");
        }
        assert!(code.contains("history_url"), "the control: the scan sees what is there");
    }
}
