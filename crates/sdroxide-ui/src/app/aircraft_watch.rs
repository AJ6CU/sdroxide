//! Telling the operator that an aircraft they asked about is on the air.
//!
//! The list of callsigns and registrations is the station's
//! ([`sdroxide_aircraft_history::watch`], served on `/aircraft-watch`), so it is
//! the same from every screen. Whether one of them is on the air is decided
//! here, from the cards the client already holds: **nothing is asked of
//! anybody to find out**, and the only requests are to the station this client
//! is connected to, to read the list and to change it.
//!
//! An alert is the moment a watched aircraft goes from not there to there: a new
//! contact, or one back after its card expired (ten minutes without a message).
//! One that was already on the air when it was put on the list, or when the
//! program started, is not news and raises nothing. No sound, no mail: a count on
//! the AIRCRAFT chip and a line in the window.

use std::collections::{HashMap, HashSet, VecDeque};

use sdroxide_aircraft_history::watch::{Outcome, WatchEdit, WatchReply, Watchlist, normalize};
use sdroxide_aircraft_history::{FORMAT_VERSION, Status};

use super::aircraft_history::Reason;
use crate::i18n::tr;
use super::aircraft_registry::AircraftCard;

/// How often the list is read again from the station.
pub(in crate::app) const REFRESH_S: i64 = 120;
/// How long a request is waited for before the line is free again.
pub(in crate::app) const FLIGHT_TIMEOUT_S: i64 = 20;
/// After the list first arrives nothing is announced for this long: the cards
/// of the aircraft already on the air are still coming in, and they are not news.
pub(in crate::app) const WARMUP_S: i64 = 30;
/// The tracker looks at the cards at most this often.
pub(in crate::app) const LOOK_EVERY_S: i64 = 1;
/// Alerts kept, newest first.
pub(in crate::app) const MAX_ALERTS: usize = 30;
/// Edits waiting for the line.
const MAX_QUEUED: usize = 8;

/// An aircraft on the air now, as far as the watch cares.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) struct Presence {
    pub icao: u32,
    pub callsign: Option<String>,
    pub registration: Option<String>,
}

pub(in crate::app) fn presences<'a>(
    cards: impl Iterator<Item = (u32, &'a AircraftCard)>,
) -> Vec<Presence> {
    cards
        .map(|(icao, c)| Presence {
            icao,
            callsign: c.flight.as_ref().map(|f| f.value.clone()),
            registration: c.registration.as_ref().map(|r| r.value.clone()),
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) struct Alert {
    /// The entry of the list it answers to.
    pub entry: String,
    pub icao: u32,
    /// What it was calling itself when it appeared.
    pub callsign: Option<String>,
    pub at: i64,
    pub seen: bool,
}

/// Decides when a watched aircraft has just appeared.
#[derive(Debug, Default)]
pub(in crate::app) struct Tracker {
    /// Entries seen on the list at the last look: one that is new to it is
    /// taken as it stands, not announced.
    known: HashSet<String>,
    /// Entries on the air at the last look, and by which address.
    on_air: HashMap<String, u32>,
    /// Nothing is announced before this.
    armed_at: Option<i64>,
    last_look: i64,
    alerts: VecDeque<Alert>,
}

impl Tracker {
    /// Look at the cards against the list. Returns how many alerts it raised.
    /// At most once a second; a call in between does nothing.
    pub(in crate::app) fn observe(
        &mut self,
        now: i64,
        list: &Watchlist,
        here: &[Presence],
    ) -> usize {
        if self.last_look != 0 && now - self.last_look < LOOK_EVERY_S {
            return 0;
        }
        self.last_look = now;
        // The warm-up runs from the first look, which is when the list arrived,
        // whether or not there was anything on it yet.
        self.armed_at.get_or_insert(now + WARMUP_S);
        // What is no longer on the list is forgotten, so that putting it back is
        // taken as it stands, like any new entry.
        self.known.retain(|e| list.entries().contains(e));
        self.on_air.retain(|e, _| list.entries().contains(e));
        let mut present: HashMap<&str, (u32, Option<&str>)> = HashMap::new();
        if !list.is_empty() {
            for p in here {
                for e in list.matching(p.callsign.as_deref(), p.registration.as_deref()) {
                    present.entry(e).or_insert((p.icao, p.callsign.as_deref()));
                }
            }
        }
        let armed = self.armed_at.is_some_and(|a| now >= a);
        let mut raised = 0;
        for entry in list.entries() {
            let was_known = self.known.contains(entry);
            let was_on = self.on_air.contains_key(entry);
            match present.get(entry.as_str()) {
                Some(&(icao, callsign)) => {
                    if was_known && !was_on && armed {
                        self.alerts.push_front(Alert {
                            entry: entry.clone(),
                            icao,
                            callsign: callsign.map(str::to_string),
                            at: now,
                            seen: false,
                        });
                        raised += 1;
                    }
                    self.on_air.insert(entry.clone(), icao);
                }
                None => {
                    self.on_air.remove(entry);
                }
            }
            self.known.insert(entry.clone());
        }
        self.alerts.truncate(MAX_ALERTS);
        raised
    }

    /// Whether a look is due, so the caller need not gather the cards for nothing.
    pub(in crate::app) fn due(&self, now: i64) -> bool {
        self.last_look == 0 || now - self.last_look >= LOOK_EVERY_S
    }

    pub(in crate::app) fn alerts(&self) -> impl Iterator<Item = &Alert> {
        self.alerts.iter()
    }

    pub(in crate::app) fn unseen(&self) -> usize {
        self.alerts.iter().filter(|a| !a.seen).count()
    }

    pub(in crate::app) fn mark_seen(&mut self) {
        self.alerts.iter_mut().for_each(|a| a.seen = true);
    }

    pub(in crate::app) fn clear(&mut self) {
        self.alerts.clear();
    }

    /// Whether this entry is on the air as of the last look.
    pub(in crate::app) fn is_on_air(&self, entry: &str) -> bool {
        self.on_air.contains_key(entry)
    }
}

/// What the person is told about an alert.
pub(in crate::app) fn alert_text(a: &Alert) -> String {
    let called = a.callsign.as_deref().map(str::trim).filter(|c| !c.is_empty());
    match called {
        Some(c) if normalize(c).as_deref() != Some(a.entry.as_str()) => {
            tr!("aircraft-alert-on-air-as", entry = a.entry, callsign = c)
        }
        _ => tr!("aircraft-alert-on-air", entry = a.entry),
    }
}

/// What the person is told of an edit that did not take; nothing for one that did.
pub(in crate::app) fn outcome_note(o: Outcome) -> Option<String> {
    match o {
        Outcome::Added | Outcome::Removed => None,
        Outcome::Already => Some(tr!("aircraft-watch-already")),
        Outcome::Absent => Some(tr!("aircraft-watch-absent")),
        Outcome::Invalid => Some(tr!("aircraft-watch-invalid")),
        Outcome::Full => Some(tr!("aircraft-watch-full")),
    }
}

/// A count for the end of the AIRCRAFT chip's label: nothing when there is
/// nothing unseen.
pub(in crate::app) fn chip_suffix(unseen: usize) -> String {
    match unseen {
        0 => String::new(),
        1..=99 => format!(" \u{25cf}{unseen}"),
        _ => " \u{25cf}99+".to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub(in crate::app) enum WatchState {
    #[default]
    Idle,
    Loading,
    Ready,
    Unavailable(Reason),
}

/// What the station's reply came to.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) enum Answer {
    Reply(WatchReply),
    Unavailable(Reason),
}

pub(in crate::app) fn classify(http_status: Option<u16>, body: &str) -> Answer {
    let Some(code) = http_status else { return Answer::Unavailable(Reason::Offline) };
    let Ok(reply) = serde_json::from_str::<WatchReply>(body) else {
        return Answer::Unavailable(Reason::Unknown);
    };
    if reply.v != FORMAT_VERSION {
        return Answer::Unavailable(Reason::Unknown);
    }
    match reply.status {
        // A 400 is an edit that was not one: its answer still carries the list.
        Status::Ok if code == 200 || (code == 400 && reply.outcome == Some(Outcome::Invalid)) => {
            Answer::Reply(reply)
        }
        Status::Disabled => Answer::Unavailable(Reason::Disabled),
        Status::Unauthorized => Answer::Unavailable(Reason::Unauthorized),
        Status::Ok | Status::Invalid => Answer::Unavailable(Reason::Unknown),
    }
}

/// What the store wants sent next.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) enum Request {
    Read,
    Edit(WatchEdit),
}

/// The list as the station last said it, and the line to the station.
#[derive(Default)]
pub(in crate::app) struct WatchStore {
    list: Watchlist,
    state: WatchState,
    at: i64,
    in_flight: Option<i64>,
    queue: VecDeque<WatchEdit>,
    /// What the last edit came to, for a note beside the box.
    last_outcome: Option<Outcome>,
}

impl WatchStore {
    pub(in crate::app) fn list(&self) -> &Watchlist {
        &self.list
    }

    pub(in crate::app) fn state(&self) -> &WatchState {
        &self.state
    }

    pub(in crate::app) fn last_outcome(&self) -> Option<Outcome> {
        self.last_outcome
    }

    pub(in crate::app) fn queued(&self) -> usize {
        self.queue.len()
    }

    pub(in crate::app) fn line_busy(&self, now: i64) -> bool {
        self.in_flight.is_some_and(|at| now - at < FLIGHT_TIMEOUT_S)
    }

    /// Ask for the list to change. Nothing is applied here: the list is what the
    /// station says it is.
    pub(in crate::app) fn request_edit(&mut self, edit: WatchEdit) -> bool {
        if self.queue.len() >= MAX_QUEUED {
            return false;
        }
        self.queue.push_back(edit);
        true
    }

    /// What to send now, if anything. Never without a station, never while a
    /// request is out; an edit goes before a read.
    pub(in crate::app) fn next_request(&mut self, has_station: bool, now: i64) -> Option<Request> {
        if !has_station || self.line_busy(now) {
            return None;
        }
        if let Some(edit) = self.queue.pop_front() {
            self.in_flight = Some(now);
            return Some(Request::Edit(edit));
        }
        let since = now - self.at;
        let due = match &self.state {
            WatchState::Idle | WatchState::Loading => true,
            WatchState::Ready => since >= REFRESH_S,
            WatchState::Unavailable(r) => r.retryable() && since >= REFRESH_S,
        };
        if !due {
            return None;
        }
        self.in_flight = Some(now);
        if self.state == WatchState::Idle {
            self.state = WatchState::Loading;
        }
        Some(Request::Read)
    }

    pub(in crate::app) fn deliver(&mut self, answer: Answer, now: i64) {
        self.in_flight = None;
        self.at = now;
        match answer {
            Answer::Reply(r) => {
                let mut list = Watchlist::default();
                r.entries.iter().for_each(|e| {
                    list.add(e);
                });
                self.list = list;
                self.last_outcome = r.outcome;
                self.state = WatchState::Ready;
            }
            Answer::Unavailable(reason) => self.state = WatchState::Unavailable(reason),
        }
    }
}

/// The edit for what was typed in the box: none for a blank box. What it is
/// spelled like is the station's to judge, and it answers with why not.
pub(in crate::app) fn add_edit(typed: &str) -> Option<WatchEdit> {
    let t = typed.trim();
    (!t.is_empty()).then(|| WatchEdit { add: Some(t.to_string()), remove: None })
}

pub(in crate::app) fn remove_edit(entry: &str) -> WatchEdit {
    WatchEdit { add: None, remove: Some(entry.to_string()) }
}

/// The body to send for a request: none for a read, the edit as JSON for an edit.
pub(in crate::app) fn body_of(req: &Request) -> Option<String> {
    match req {
        Request::Read => None,
        Request::Edit(e) => serde_json::to_string(e).ok(),
    }
}

/// Where the requests go: the station this client is connected to.
pub(in crate::app) fn watch_url(peer_url: &str) -> Option<String> {
    let base = crate::login::http_base(&crate::login::station_key(peer_url))?;
    Some(format!("{base}/aircraft-watch"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::aircraft_registry::{Source, Stamped};

    const T0: i64 = 1_000_000;

    fn here(icao: u32, call: &str, reg: Option<&str>) -> Presence {
        Presence { icao, callsign: Some(call.into()), registration: reg.map(str::to_string) }
    }

    fn list(of: &[&str]) -> Watchlist {
        let mut l = Watchlist::default();
        of.iter().for_each(|e| {
            l.add(e);
        });
        l
    }

    /// A tracker that has been looking for long enough to announce.
    fn armed(l: &Watchlist) -> Tracker {
        let mut t = Tracker::default();
        t.observe(T0, l, &[]);
        t
    }

    const ARMED: i64 = T0 + WARMUP_S + 1;

    // ── what is on the air ────────────────────────────────────────────────

    #[test]
    fn a_card_gives_its_flight_and_its_registration() {
        let mut c = AircraftCard::default();
        c.flight = Some(Stamped { value: "KLM1234".into(), source: Source::Adsb, at: 1 });
        c.registration = Some(Stamped { value: "PH-BXA".into(), source: Source::Vdl2, at: 1 });
        let bare = AircraftCard::default();
        let p = presences([(7, &c), (8, &bare)].into_iter());
        assert_eq!(p.len(), 2);
        let seven = p.iter().find(|x| x.icao == 7).unwrap();
        assert_eq!(
            (seven.callsign.as_deref(), seven.registration.as_deref()),
            (Some("KLM1234"), Some("PH-BXA"))
        );
        let eight = p.iter().find(|x| x.icao == 8).unwrap();
        assert_eq!((&eight.callsign, &eight.registration), (&None, &None));
    }

    // ── when it is news ───────────────────────────────────────────────────

    #[test]
    fn a_watched_aircraft_that_appears_is_an_alert() {
        let l = list(&["KLM1234"]);
        let mut t = armed(&l);
        assert_eq!(t.observe(ARMED, &l, &[]), 0);
        assert_eq!(t.observe(ARMED + 1, &l, &[here(7, "KLM1234", None)]), 1);
        let a = t.alerts().next().unwrap();
        assert_eq!((a.entry.as_str(), a.icao, a.at, a.seen), ("KLM1234", 7, ARMED + 1, false));
        assert_eq!(t.unseen(), 1);
        assert!(t.is_on_air("KLM1234"));
    }

    #[test]
    fn it_is_found_by_its_registration_too() {
        let l = list(&["PHBXA"]);
        let mut t = armed(&l);
        let n = t.observe(ARMED, &l, &[here(7, "KLM1234", Some("PH-BXA"))]);
        assert_eq!(n, 1);
        assert_eq!(t.alerts().next().unwrap().entry, "PHBXA");
    }

    #[test]
    fn an_aircraft_that_is_not_on_the_list_raises_nothing() {
        let l = list(&["KLM1234"]);
        let mut t = armed(&l);
        assert_eq!(t.observe(ARMED, &l, &[here(7, "KLM1235", Some("PH-BXB"))]), 0);
        assert_eq!(t.unseen(), 0);
        assert!(!t.is_on_air("KLM1234"));
        let mut empty = armed(&Watchlist::default());
        assert_eq!(empty.observe(ARMED, &Watchlist::default(), &[here(7, "KLM1234", None)]), 0);
    }

    #[test]
    fn staying_on_the_air_is_one_alert_and_coming_back_is_another() {
        let l = list(&["KLM1234"]);
        let mut t = armed(&l);
        let on = [here(7, "KLM1234", None)];
        assert_eq!(t.observe(ARMED, &l, &on), 1);
        assert_eq!(t.observe(ARMED + 1, &l, &on), 0, "still there");
        assert_eq!(t.observe(ARMED + 2, &l, &on), 0);
        assert_eq!(t.observe(ARMED + 3, &l, &[]), 0, "gone");
        assert!(!t.is_on_air("KLM1234"));
        assert_eq!(t.observe(ARMED + 4, &l, &on), 1, "back after an absence");
        assert_eq!(t.alerts().count(), 2);
    }

    #[test]
    fn two_watched_aircraft_raise_two_alerts() {
        let l = list(&["KLM1234", "BAW9"]);
        let mut t = armed(&l);
        let both = [here(7, "KLM1234", None), here(8, "BAW9", None)];
        assert_eq!(t.observe(ARMED, &l, &both), 2);
    }

    #[test]
    fn one_aircraft_that_answers_to_two_entries_raises_one_alert_for_each() {
        let l = list(&["KLM1234", "PHBXA"]);
        let mut t = armed(&l);
        assert_eq!(t.observe(ARMED, &l, &[here(7, "KLM1234", Some("PH-BXA"))]), 2);
    }

    #[test]
    fn what_was_already_there_when_the_list_arrived_is_not_news() {
        let l = list(&["KLM1234"]);
        let mut t = Tracker::default();
        let on = [here(7, "KLM1234", None)];
        assert_eq!(t.observe(T0, &l, &on), 0, "first look");
        assert_eq!(t.observe(T0 + 5, &l, &on), 0);
        assert_eq!(t.observe(ARMED + 10, &l, &on), 0, "and still there once armed");
        assert!(t.is_on_air("KLM1234"));
    }

    #[test]
    fn nothing_is_announced_during_the_warm_up() {
        let l = list(&["KLM1234"]);
        let mut t = Tracker::default();
        t.observe(T0, &l, &[]);
        assert_eq!(t.observe(T0 + WARMUP_S - 1, &l, &[here(7, "KLM1234", None)]), 0);
        // It was seen during the warm-up, so it is not new afterwards either.
        assert_eq!(t.observe(T0 + WARMUP_S + 1, &l, &[here(7, "KLM1234", None)]), 0);
        assert_eq!(t.observe(T0 + WARMUP_S + 2, &l, &[]), 0);
        assert_eq!(t.observe(T0 + WARMUP_S + 3, &l, &[here(7, "KLM1234", None)]), 1);
    }

    #[test]
    fn an_entry_added_while_its_aircraft_is_on_the_air_is_not_announced() {
        let mut l = list(&["BAW9"]);
        let mut t = armed(&l);
        let on = [here(7, "KLM1234", None)];
        assert_eq!(t.observe(ARMED, &l, &on), 0);
        l.add("KLM1234");
        assert_eq!(t.observe(ARMED + 1, &l, &on), 0, "the person can see it is there");
        assert!(t.is_on_air("KLM1234"));
        assert_eq!(t.observe(ARMED + 2, &l, &[]), 0);
        assert_eq!(t.observe(ARMED + 3, &l, &on), 1, "but the next time it is");
    }

    #[test]
    fn an_entry_taken_off_and_put_back_is_taken_as_it_stands() {
        let mut l = list(&["KLM1234"]);
        let mut t = armed(&l);
        let on = [here(7, "KLM1234", None)];
        t.observe(ARMED, &l, &on);
        l.remove("KLM1234");
        t.observe(ARMED + 1, &l, &on);
        assert!(!t.is_on_air("KLM1234"), "no longer watched");
        l.add("KLM1234");
        assert_eq!(t.observe(ARMED + 2, &l, &on), 0);
    }

    #[test]
    fn the_cards_are_looked_at_once_a_second_at_most() {
        let l = list(&["KLM1234"]);
        let mut t = armed(&l);
        assert_eq!(t.observe(ARMED, &l, &[]), 0);
        assert_eq!(
            t.observe(ARMED, &l, &[here(7, "KLM1234", None)]),
            0,
            "the same second: not looked at"
        );
        assert_eq!(t.observe(ARMED + LOOK_EVERY_S, &l, &[here(7, "KLM1234", None)]), 1);
    }

    #[test]
    fn the_warm_up_runs_from_the_first_look_even_when_the_list_was_empty() {
        let mut l = Watchlist::default();
        let mut t = Tracker::default();
        t.observe(T0, &l, &[]);
        l.add("KLM1234");
        let on = [here(7, "KLM1234", None)];
        assert_eq!(t.observe(T0 + WARMUP_S + 10, &l, &[]), 0, "taken as it stands");
        assert_eq!(t.observe(T0 + WARMUP_S + 11, &l, &on), 1, "armed since the first look");
    }

    #[test]
    fn a_look_is_due_once_a_second_and_the_first_is_always_due() {
        let l = list(&["KLM1234"]);
        let mut t = Tracker::default();
        assert!(t.due(T0));
        t.observe(T0, &l, &[]);
        assert!(!t.due(T0));
        assert!(t.due(T0 + LOOK_EVERY_S));
    }

    // ── what is kept ──────────────────────────────────────────────────────

    #[test]
    fn seen_clear_and_the_ceiling() {
        let l = list(&["KLM1234"]);
        let mut t = armed(&l);
        let on = [here(7, "KLM1234", None)];
        let mut at = ARMED;
        for _ in 0..MAX_ALERTS + 5 {
            t.observe(at, &l, &on);
            t.observe(at + 1, &l, &[]);
            at += 2;
        }
        assert_eq!(t.alerts().count(), MAX_ALERTS, "the oldest go first");
        let newest = t.alerts().next().unwrap().at;
        assert!(t.alerts().all(|a| a.at <= newest), "newest first: {newest}");
        assert_eq!(t.alerts().next().unwrap().at, at - 2);
        assert_eq!(t.unseen(), MAX_ALERTS);
        t.mark_seen();
        assert_eq!(t.unseen(), 0);
        assert_eq!(t.alerts().count(), MAX_ALERTS, "seen is not cleared");
        t.clear();
        assert_eq!(t.alerts().count(), 0);
    }

    // ── the words ─────────────────────────────────────────────────────────

    fn alert(entry: &str, call: Option<&str>) -> Alert {
        Alert {
            entry: entry.into(),
            icao: 7,
            callsign: call.map(str::to_string),
            at: T0,
            seen: false,
        }
    }

    #[test]
    fn the_alert_says_what_was_watched_and_what_it_is_calling_itself() {
        assert_eq!(alert_text(&alert("KLM1234", Some("KLM1234"))), "KLM1234 is on the air");
        assert_eq!(alert_text(&alert("KLM1234", Some("klm1234 "))), "KLM1234 is on the air");
        assert_eq!(alert_text(&alert("PHBXA", Some("KLM1234"))), "PHBXA is on the air, as KLM1234");
        assert_eq!(alert_text(&alert("PHBXA", None)), "PHBXA is on the air");
    }

    #[test]
    fn an_edit_that_did_not_take_says_why_and_one_that_did_says_nothing() {
        assert_eq!(outcome_note(Outcome::Added), None);
        assert_eq!(outcome_note(Outcome::Removed), None);
        for o in [Outcome::Already, Outcome::Absent, Outcome::Invalid, Outcome::Full] {
            assert!(outcome_note(o).is_some(), "{o:?}");
        }
        assert!(outcome_note(Outcome::Invalid).unwrap().contains("3"), "says how long");
        assert!(outcome_note(Outcome::Full).unwrap().contains("100"));
    }

    #[test]
    fn the_chip_wears_a_count_only_when_there_is_something_unseen() {
        assert_eq!(chip_suffix(0), "");
        assert_eq!(chip_suffix(2), " \u{25cf}2");
        assert_eq!(chip_suffix(120), " \u{25cf}99+");
    }

    // ── the line to the station ───────────────────────────────────────────

    fn reply(status: &str, outcome: Option<&str>, entries: &[&str]) -> String {
        let outcome = outcome.map(|o| format!(r#","outcome":"{o}""#)).unwrap_or_default();
        let e = entries.iter().map(|e| format!("\"{e}\"")).collect::<Vec<_>>().join(",");
        format!(r#"{{"v":{FORMAT_VERSION},"status":"{status}"{outcome},"entries":[{e}]}}"#)
    }

    #[test]
    fn the_stations_reply_is_a_list_or_a_reason() {
        let ok = classify(Some(200), &reply("ok", None, &["KLM1"]));
        assert!(matches!(&ok, Answer::Reply(r) if r.entries == ["KLM1"]), "{ok:?}");
        let refused = classify(Some(400), &reply("ok", Some("invalid"), &["KLM1"]));
        assert!(
            matches!(&refused, Answer::Reply(r) if r.outcome == Some(Outcome::Invalid)),
            "an edit that was not one still carries the list: {refused:?}"
        );
        assert_eq!(classify(None, ""), Answer::Unavailable(Reason::Offline));
        assert_eq!(
            classify(Some(401), &reply("unauthorized", None, &[])),
            Answer::Unavailable(Reason::Unauthorized)
        );
        assert_eq!(classify(Some(200), "<html>"), Answer::Unavailable(Reason::Unknown));
        let other = r#"{"v":999,"status":"ok","entries":[]}"#;
        assert_eq!(classify(Some(200), other), Answer::Unavailable(Reason::Unknown));
        assert_eq!(
            classify(Some(400), &reply("ok", None, &[])),
            Answer::Unavailable(Reason::Unknown),
            "a 400 that is not about the edit"
        );
        assert_eq!(classify(Some(404), "not found"), Answer::Unavailable(Reason::Unknown));
    }

    fn ready(entries: &[&str]) -> Answer {
        Answer::Reply(WatchReply::of(&list(entries), None))
    }

    #[test]
    fn nothing_is_asked_without_a_station_and_the_first_ask_is_a_read() {
        let mut s = WatchStore::default();
        assert_eq!(s.next_request(false, T0), None);
        assert_eq!(s.next_request(true, T0), Some(Request::Read));
        assert_eq!(*s.state(), WatchState::Loading);
        assert!(s.line_busy(T0 + 1));
        assert_eq!(s.next_request(true, T0 + 1), None, "one at a time");
    }

    #[test]
    fn the_answer_is_the_list_and_frees_the_line() {
        let mut s = WatchStore::default();
        s.next_request(true, T0);
        s.deliver(ready(&["KLM1", "BAW9"]), T0 + 1);
        assert_eq!(*s.state(), WatchState::Ready);
        assert_eq!(s.list().entries(), ["BAW9", "KLM1"]);
        assert!(!s.line_busy(T0 + 1));
        s.deliver(ready(&["KLM1"]), T0 + 500);
        assert_eq!(s.list().entries(), ["KLM1"]);
    }

    #[test]
    fn the_list_is_read_again_every_two_minutes_and_a_refusal_is_not_retried() {
        let mut s = WatchStore::default();
        s.next_request(true, T0);
        s.deliver(ready(&[]), T0);
        assert_eq!(s.next_request(true, T0 + REFRESH_S - 1), None);
        assert_eq!(s.next_request(true, T0 + REFRESH_S), Some(Request::Read));
        s.deliver(Answer::Unavailable(Reason::Offline), T0 + REFRESH_S);
        assert_eq!(s.next_request(true, T0 + 2 * REFRESH_S - 1), None);
        assert_eq!(s.next_request(true, T0 + 2 * REFRESH_S), Some(Request::Read), "may come back");
        s.deliver(Answer::Unavailable(Reason::Unauthorized), T0 + 2 * REFRESH_S);
        assert_eq!(s.next_request(true, T0 + 100 * REFRESH_S), None, "needs a sign-in, not time");
    }

    #[test]
    fn an_edit_goes_before_a_read_and_one_at_a_time_in_order() {
        let mut s = WatchStore::default();
        let add = |e: &str| WatchEdit { add: Some(e.into()), remove: None };
        assert!(s.request_edit(add("KLM1")));
        assert!(s.request_edit(add("BAW9")));
        assert_eq!(s.queued(), 2);
        assert_eq!(s.next_request(true, T0), Some(Request::Edit(add("KLM1"))));
        assert_eq!(s.next_request(true, T0 + 1), None, "the first is still out");
        s.deliver(ready(&["KLM1"]), T0 + 1);
        assert_eq!(s.next_request(true, T0 + 1), Some(Request::Edit(add("BAW9"))));
        s.deliver(ready(&["BAW9", "KLM1"]), T0 + 2);
        assert_eq!(s.next_request(true, T0 + 2), None, "the list was just read in the answer");
    }

    #[test]
    fn an_edit_waits_for_a_station_and_the_queue_has_a_ceiling() {
        let mut s = WatchStore::default();
        let add = |e: &str| WatchEdit { add: Some(e.into()), remove: None };
        assert!(s.request_edit(add("KLM1")));
        assert_eq!(s.next_request(false, T0), None, "no station: it waits");
        assert_eq!(s.queued(), 1);
        for i in 0..20 {
            s.request_edit(add(&format!("AAA{i}")));
        }
        assert!(s.queued() <= MAX_QUEUED);
        assert!(!s.request_edit(add("ZZZ9")), "refused when full");
    }

    #[test]
    fn what_an_edit_came_to_is_kept_for_a_note() {
        let mut s = WatchStore::default();
        s.request_edit(WatchEdit { add: Some("x".into()), remove: None });
        s.next_request(true, T0);
        let r = WatchReply::of(&list(&[]), Some(Outcome::Invalid));
        s.deliver(Answer::Reply(r), T0 + 1);
        assert_eq!(s.last_outcome(), Some(Outcome::Invalid));
        s.next_request(true, T0 + REFRESH_S + 5);
        s.deliver(ready(&[]), T0 + REFRESH_S + 6);
        assert_eq!(s.last_outcome(), None, "a plain read clears it");
    }

    #[test]
    fn what_is_typed_becomes_an_add_and_a_blank_box_becomes_nothing() {
        assert_eq!(
            add_edit("  ph-bxa "),
            Some(WatchEdit { add: Some("ph-bxa".into()), remove: None })
        );
        assert_eq!(add_edit(""), None);
        assert_eq!(add_edit("   "), None);
        assert_eq!(remove_edit("PHBXA"), WatchEdit { add: None, remove: Some("PHBXA".into()) });
    }

    #[test]
    fn a_read_has_no_body_and_an_edit_is_the_edit_as_json() {
        assert_eq!(body_of(&Request::Read), None);
        let add = WatchEdit { add: Some("KLM1".into()), remove: None };
        let body = body_of(&Request::Edit(add.clone())).unwrap();
        assert_eq!(serde_json::from_str::<WatchEdit>(&body).unwrap(), add);
        assert!(!body.contains("cookie") && !body.contains("session"), "{body}");
    }

    #[test]
    fn the_requests_go_to_the_connected_station_only() {
        assert_eq!(
            watch_url("wss://shack.example/ws").as_deref(),
            Some("https://shack.example/aircraft-watch")
        );
        assert_eq!(watch_url("not an address"), None);
    }

    #[test]
    fn this_module_reaches_nothing_and_draws_nothing() {
        let src = include_str!("aircraft_watch.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap()];
        for banned in ["ureq", "reqwest", "TcpStream", "adsbdb", "plnspttrs", "std::fs", "egui"] {
            assert!(!code.contains(banned), "aircraft_watch.rs must not touch `{banned}`");
        }
        assert!(code.contains("WatchStore"), "the control: the scan sees what is there");
    }
}
