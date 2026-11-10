//! What the sky has been made of: the most common categories, airlines, models,
//! operators and routes among the aircraft this station knows.
//!
//! **Radio first.** The category the radio reported and the airline code in the
//! callsign are counted from what was heard, and wear the green *radio* badge.
//! The model, the operator and the route come from the relay's cards, only for
//! the aircraft it has answered about, and wear the blue *online* one; they
//! complement the radio's tables and never replace them. There is no new source
//! here: the facts are the live registry, the cards the relay already gave this
//! session, and — when the station keeps one — its history list.
//!
//! The counting is pure. The fetch of the list is [`StatsStore`], which asks only
//! while the view is open, only the station this client is connected to, one at
//! a time and once a minute.

use std::collections::HashMap;

use sdroxide_aircraft_history::{FORMAT_VERSION, ListReply, Status, Summary};
use sdroxide_aircraft_online::{Info, Status as InfoStatus};

use super::aircraft_history::Reason;
use crate::i18n::tr;
use super::aircraft_online::Badge;
use super::aircraft_registry::AircraftCard;

/// How often the station's list is fetched again while the view is open.
pub(in crate::app) const REFRESH_S: i64 = 60;
/// How long a request is waited for before the line is free again.
pub(in crate::app) const FLIGHT_TIMEOUT_S: i64 = 20;
/// Rows kept in each table.
pub(in crate::app) const TOP: usize = 10;

/// What is known about one aircraft, from whichever place knew it.
#[derive(Debug, Clone, Default, PartialEq)]
pub(in crate::app) struct Fact {
    pub icao: u32,
    /// Radio: the flight number or callsign as heard.
    pub callsign: Option<String>,
    /// Radio: the ADS-B emitter category.
    pub category: Option<String>,
    /// Online: from the relay's card.
    pub type_name: Option<String>,
    pub operator: Option<String>,
    pub origin: Option<String>,
    pub destination: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::app) enum SortBy {
    /// Most first, ties by name.
    #[default]
    Count,
    Name,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) struct Top {
    pub label: String,
    pub count: usize,
}

/// One table: where its numbers come from, its biggest rows, and how many
/// different values there were in all.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) struct Table {
    pub title: &'static str,
    pub badge: Badge,
    pub rows: Vec<Top>,
    pub distinct: usize,
    /// Aircraft that have a value for it.
    pub known: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) struct Stats {
    /// Aircraft counted.
    pub total: usize,
    pub tables: [Table; 5],
}

/// The airline of a callsign: three letters then a digit (`KLM1234`, `BAW12A`).
/// A registration, a squawk or a bare tail number has none.
pub(in crate::app) fn airline_code(callsign: &str) -> Option<String> {
    let b = callsign.trim().as_bytes();
    (b.len() >= 4 && b[..3].iter().all(u8::is_ascii_alphabetic) && b[3].is_ascii_digit())
        .then(|| String::from_utf8_lossy(&b[..3]).to_ascii_uppercase())
}

/// The same aircraft from two places, one fact each: the radio's own values come
/// from `live` when it has them, and the card's from `live` first too, since it
/// is the newer; `history` fills what `live` lacks. Ordered by address.
pub(in crate::app) fn merge(live: Vec<Fact>, history: Vec<Fact>) -> Vec<Fact> {
    let mut by_icao: HashMap<u32, Fact> = live.into_iter().map(|f| (f.icao, f)).collect();
    for h in history {
        match by_icao.get_mut(&h.icao) {
            Some(f) => {
                f.callsign = f.callsign.take().or(h.callsign);
                f.category = f.category.take().or(h.category);
                f.type_name = f.type_name.take().or(h.type_name);
                f.operator = f.operator.take().or(h.operator);
                f.origin = f.origin.take().or(h.origin);
                f.destination = f.destination.take().or(h.destination);
            }
            None => {
                by_icao.insert(h.icao, h);
            }
        }
    }
    let mut out: Vec<Fact> = by_icao.into_values().collect();
    out.sort_by_key(|f| f.icao);
    out
}

pub(in crate::app) fn facts_from_summaries(list: &[Summary]) -> Vec<Fact> {
    list.iter()
        .map(|s| Fact {
            icao: s.icao,
            callsign: s.callsign.clone(),
            category: s.category.clone(),
            type_name: s.type_name.clone(),
            operator: s.operator.clone(),
            origin: s.origin.clone(),
            destination: s.destination.clone(),
        })
        .collect()
}

pub(in crate::app) fn facts_from_live<'a>(
    cards: impl Iterator<Item = (u32, &'a AircraftCard)>,
    infos: impl Iterator<Item = (u32, &'a Info)>,
) -> Vec<Fact> {
    let mut by_icao: HashMap<u32, Fact> = cards
        .map(|(icao, c)| {
            (
                icao,
                Fact {
                    icao,
                    callsign: c.flight.as_ref().map(|f| f.value.clone()),
                    ..Fact::default()
                },
            )
        })
        .collect();
    for (icao, info) in infos.filter(|(_, i)| i.status == InfoStatus::Ok) {
        let air = info.aircraft.as_ref();
        let code = |p: Option<&sdroxide_aircraft_online::Airport>| p.map(|p| p.code.clone());
        let route = info.route.as_ref();
        let card = Fact {
            icao,
            type_name: air.and_then(|a| a.type_name.clone()),
            operator: air.and_then(|a| a.operator.clone()),
            origin: code(route.and_then(|r| r.origin.as_ref())),
            destination: code(route.and_then(|r| r.destination.as_ref())),
            ..Fact::default()
        };
        let nothing = Fact { icao, ..Fact::default() };
        if card == nothing {
            continue;
        }
        let f = by_icao.entry(icao).or_insert_with(|| Fact { icao, ..Fact::default() });
        f.type_name = card.type_name;
        f.operator = card.operator;
        f.origin = card.origin;
        f.destination = card.destination;
    }
    let mut out: Vec<Fact> = by_icao.into_values().collect();
    out.sort_by_key(|f| f.icao);
    out
}

fn clean(v: &Option<String>) -> Option<&str> {
    v.as_deref().map(str::trim).filter(|s| !s.is_empty())
}

fn table(
    title: &'static str,
    badge: Badge,
    values: impl Iterator<Item = String>,
    sort: SortBy,
    limit: usize,
) -> Table {
    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut known = 0;
    for v in values {
        known += 1;
        *counts.entry(v).or_default() += 1;
    }
    let distinct = counts.len();
    let mut rows: Vec<Top> =
        counts.into_iter().map(|(label, count)| Top { label, count }).collect();
    let by_name = |a: &Top, b: &Top| {
        a.label.to_lowercase().cmp(&b.label.to_lowercase()).then(a.label.cmp(&b.label))
    };
    match sort {
        SortBy::Count => rows.sort_by(|a, b| b.count.cmp(&a.count).then_with(|| by_name(a, b))),
        SortBy::Name => rows.sort_by(by_name),
    }
    rows.truncate(limit);
    Table { title, badge, rows, distinct, known }
}

pub(in crate::app) fn compute(facts: &[Fact], sort: SortBy, limit: usize) -> Stats {
    let own = |pick: fn(&Fact) -> Option<&str>| {
        facts.iter().filter_map(move |f| pick(f).map(str::to_string))
    };
    Stats {
        total: facts.len(),
        tables: [
            table("Category", Badge::Radio, own(|f| clean(&f.category)), sort, limit),
            table(
                "Airline",
                Badge::Radio,
                facts.iter().filter_map(|f| clean(&f.callsign).and_then(airline_code)),
                sort,
                limit,
            ),
            table("Model", Badge::Online, own(|f| clean(&f.type_name)), sort, limit),
            table("Operator", Badge::Online, own(|f| clean(&f.operator)), sort, limit),
            table(
                "Route",
                Badge::Online,
                facts.iter().filter_map(|f| {
                    Some(format!("{} → {}", clean(&f.origin)?, clean(&f.destination)?))
                }),
                sort,
                limit,
            ),
        ],
    }
}

/// What the station's list came to.
#[derive(Debug, Clone, PartialEq, Default)]
pub(in crate::app) enum StatsState {
    #[default]
    Idle,
    Loading,
    Ready(Vec<Summary>),
    Unavailable(Reason),
}

/// Turn the status and body of the station's reply into a state.
pub(in crate::app) fn classify(http_status: Option<u16>, body: &str) -> StatsState {
    let Some(code) = http_status else { return StatsState::Unavailable(Reason::Offline) };
    let Ok(reply) = serde_json::from_str::<ListReply>(body) else {
        return StatsState::Unavailable(Reason::Unknown);
    };
    if reply.v != FORMAT_VERSION {
        return StatsState::Unavailable(Reason::Unknown);
    }
    match reply.status {
        Status::Ok if code == 200 => StatsState::Ready(reply.aircraft),
        Status::Disabled => StatsState::Unavailable(Reason::Disabled),
        Status::Unauthorized => StatsState::Unavailable(Reason::Unauthorized),
        Status::Ok | Status::Invalid => StatsState::Unavailable(Reason::Unknown),
    }
}

/// Whether the view is open, what has come back, and the line to the station.
#[derive(Default)]
pub(in crate::app) struct StatsStore {
    on: bool,
    state: StatsState,
    at: i64,
    in_flight: Option<i64>,
}

impl StatsStore {
    pub(in crate::app) fn set_on(&mut self, on: bool) {
        if self.on != on {
            self.on = on;
            self.state = StatsState::Idle;
            self.at = 0;
        }
    }

    pub(in crate::app) fn state(&self) -> &StatsState {
        &self.state
    }

    pub(in crate::app) fn line_busy(&self, now: i64) -> bool {
        self.in_flight.is_some_and(|at| now - at < FLIGHT_TIMEOUT_S)
    }

    /// Whether to fetch now. Never while the view is closed, never without a
    /// station, never while a request is out.
    pub(in crate::app) fn next_request(&mut self, has_station: bool, now: i64) -> bool {
        if !self.on || !has_station || self.line_busy(now) {
            return false;
        }
        let since = now - self.at;
        let due = match &self.state {
            StatsState::Idle | StatsState::Loading => true,
            StatsState::Ready(_) => since >= REFRESH_S,
            StatsState::Unavailable(r) => r.retryable() && since >= REFRESH_S,
        };
        if due {
            self.in_flight = Some(now);
            if self.state == StatsState::Idle {
                self.state = StatsState::Loading;
            }
        }
        due
    }

    /// The station's answer. It frees the line; it is kept only while the view
    /// is still open.
    pub(in crate::app) fn deliver(&mut self, state: StatsState, now: i64) {
        self.in_flight = None;
        if self.on {
            self.at = now;
            self.state = state;
        }
    }
}

/// One line on where the numbers come from, for the top of the view.
pub(in crate::app) fn source_note(state: &StatsState, has_station: bool) -> String {
    if !has_station {
        return tr!("aircraft-stats-src-no-station");
    }
    match state {
        StatsState::Idle | StatsState::Loading => tr!("aircraft-stats-src-asking"),
        StatsState::Ready(_) => tr!("aircraft-stats-src-ready"),
        StatsState::Unavailable(Reason::Disabled) => tr!("aircraft-stats-src-disabled"),
        StatsState::Unavailable(r) => {
            tr!("aircraft-stats-src-reason", reason = capitalised(&r.note()))
        }
    }
}

/// A table's title as drawn; the title itself is the English key.
pub(in crate::app) fn table_title(title: &str) -> String {
    match title {
        "Category" => tr!("aircraft-stats-category"),
        "Airline" => tr!("aircraft-stats-airline"),
        "Model" => tr!("aircraft-stats-model"),
        "Operator" => tr!("aircraft-stats-operator"),
        "Route" => tr!("aircraft-stats-route"),
        other => other.to_string(),
    }
}

fn capitalised(s: &str) -> String {
    let mut c = s.chars();
    c.next().map_or_else(String::new, |f| f.to_uppercase().chain(c).collect())
}

/// Where the request goes: the station this client is connected to.
pub(in crate::app) fn list_url(peer_url: &str) -> Option<String> {
    let base = crate::login::http_base(&crate::login::station_key(peer_url))?;
    Some(format!("{base}/aircraft-history"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdroxide_aircraft_online::{AircraftInfo, Airport, Route, Status as InfoStatus};

    fn fact(icao: u32, call: &str, cat: &str) -> Fact {
        Fact { icao, callsign: Some(call.into()), category: Some(cat.into()), ..Fact::default() }
    }

    fn carded(mut f: Fact, ty: &str, op: &str, from: &str, to: &str) -> Fact {
        f.type_name = Some(ty.into());
        f.operator = Some(op.into());
        f.origin = Some(from.into());
        f.destination = Some(to.into());
        f
    }

    fn table<'a>(s: &'a Stats, title: &str) -> &'a Table {
        s.tables.iter().find(|t| t.title == title).unwrap_or_else(|| panic!("no {title}"))
    }

    fn labels(t: &Table) -> Vec<(&str, usize)> {
        t.rows.iter().map(|r| (r.label.as_str(), r.count)).collect()
    }

    // ── the airline in a callsign ─────────────────────────────────────────

    #[test]
    fn the_airline_is_three_letters_then_a_digit() {
        assert_eq!(airline_code("KLM1234").as_deref(), Some("KLM"));
        assert_eq!(airline_code("baw12a").as_deref(), Some("BAW"), "case is not the airline's");
        assert_eq!(airline_code("  EZY45AB ").as_deref(), Some("EZY"));
        assert_eq!(airline_code("DLH4TY").as_deref(), Some("DLH"));
    }

    #[test]
    fn a_registration_or_a_scrap_has_no_airline() {
        for s in ["PHBXA", "N123AB", "D-AIMA", "KLM", "KL1234", "1234567", "", "   ", "É1É2345"] {
            assert_eq!(airline_code(s), None, "{s:?}");
        }
    }

    // ── the tables ────────────────────────────────────────────────────────

    #[test]
    fn nothing_known_is_empty_tables_and_not_a_failure() {
        let s = compute(&[], SortBy::Count, TOP);
        assert_eq!(s.total, 0);
        assert!(s.tables.iter().all(|t| t.rows.is_empty() && t.distinct == 0 && t.known == 0));
    }

    #[test]
    fn the_radio_tables_count_what_was_heard_with_the_radio_badge() {
        let facts = vec![
            fact(1, "KLM1", "heavy"),
            fact(2, "KLM2", "heavy"),
            fact(3, "BAW9", "large"),
            fact(4, "PHBXA", "light"),
        ];
        let s = compute(&facts, SortBy::Count, TOP);
        assert_eq!(s.total, 4);
        let c = table(&s, "Category");
        assert_eq!(c.badge, Badge::Radio);
        assert_eq!(labels(c), vec![("heavy", 2), ("large", 1), ("light", 1)]);
        let a = table(&s, "Airline");
        assert_eq!(a.badge, Badge::Radio);
        assert_eq!(labels(a), vec![("KLM", 2), ("BAW", 1)]);
        assert_eq!((a.known, a.distinct), (3, 2), "the tail number is not an airline");
    }

    #[test]
    fn the_online_tables_count_only_what_the_relay_gave_with_the_online_badge() {
        let facts = vec![
            carded(fact(1, "KLM1", "heavy"), "747-8", "KLM", "AMS", "JFK"),
            carded(fact(2, "KLM2", "heavy"), "747-8", "KLM", "AMS", "JFK"),
            carded(fact(3, "BAW9", "large"), "A320", "British Airways", "LHR", "AMS"),
            fact(4, "XXX1", "light"),
        ];
        let s = compute(&facts, SortBy::Count, TOP);
        let m = table(&s, "Model");
        assert_eq!(m.badge, Badge::Online);
        assert_eq!(labels(m), vec![("747-8", 2), ("A320", 1)]);
        assert_eq!(m.known, 3, "the fourth has no card");
        assert_eq!(labels(table(&s, "Operator")), vec![("KLM", 2), ("British Airways", 1)]);
        let r = table(&s, "Route");
        assert_eq!(r.badge, Badge::Online);
        assert_eq!(labels(r), vec![("AMS → JFK", 2), ("LHR → AMS", 1)]);
    }

    #[test]
    fn a_route_needs_both_ends() {
        let mut f = fact(1, "KLM1", "heavy");
        f.origin = Some("AMS".into());
        let mut g = fact(2, "KLM2", "heavy");
        g.destination = Some("JFK".into());
        let mut h = fact(3, "KLM3", "heavy");
        h.origin = Some("  ".into());
        h.destination = Some("JFK".into());
        let s = compute(&[f, g, h], SortBy::Count, TOP);
        assert!(table(&s, "Route").rows.is_empty());
    }

    #[test]
    fn blank_values_are_not_counted_and_spaces_do_not_split_a_row() {
        let mut a = fact(1, "KLM1", " heavy ");
        a.type_name = Some("  ".into());
        let b = fact(2, "KLM2", "heavy");
        let s = compute(&[a, b], SortBy::Count, TOP);
        assert_eq!(labels(table(&s, "Category")), vec![("heavy", 2)]);
        assert!(table(&s, "Model").rows.is_empty());
    }

    // ── the order ─────────────────────────────────────────────────────────

    fn spread() -> Vec<Fact> {
        let mut v = Vec::new();
        for (i, c) in ["zeta", "alpha", "alpha", "mid", "mid", "mid", "beta"].iter().enumerate() {
            v.push(fact(i as u32, "KLM1", c));
        }
        v
    }

    #[test]
    fn by_count_the_most_first_and_ties_by_name() {
        let s = compute(&spread(), SortBy::Count, TOP);
        assert_eq!(
            labels(table(&s, "Category")),
            vec![("mid", 3), ("alpha", 2), ("beta", 1), ("zeta", 1)]
        );
    }

    #[test]
    fn by_name_alphabetical_whatever_the_count() {
        let s = compute(&spread(), SortBy::Name, TOP);
        assert_eq!(
            labels(table(&s, "Category")),
            vec![("alpha", 2), ("beta", 1), ("mid", 3), ("zeta", 1)]
        );
    }

    #[test]
    fn a_table_is_cut_to_its_limit_after_sorting_and_still_says_how_many_there_were() {
        let s = compute(&spread(), SortBy::Count, 2);
        let c = table(&s, "Category");
        assert_eq!(labels(c), vec![("mid", 3), ("alpha", 2)]);
        assert_eq!((c.distinct, c.known), (4, 7));
        let by_name = compute(&spread(), SortBy::Name, 2);
        assert_eq!(labels(table(&by_name, "Category")), vec![("alpha", 2), ("beta", 1)]);
    }

    #[test]
    fn the_same_facts_give_the_same_tables_in_any_input_order() {
        let mut v = spread();
        let a = compute(&v, SortBy::Count, TOP);
        v.reverse();
        assert_eq!(a, compute(&v, SortBy::Count, TOP));
    }

    #[test]
    fn four_thousand_aircraft_are_counted_at_once_and_fast() {
        let facts: Vec<Fact> = (0..4000u32)
            .map(|i| {
                carded(
                    fact(i, &format!("A{:02}{i}", i % 40), &format!("c{}", i % 7)),
                    &format!("T{}", i % 90),
                    &format!("Op{}", i % 60),
                    &format!("O{}", i % 30),
                    &format!("D{}", i % 25),
                )
            })
            .collect();
        let t0 = std::time::Instant::now();
        let s = compute(&facts, SortBy::Count, TOP);
        let took = t0.elapsed();
        assert_eq!(s.total, 4000);
        assert_eq!(table(&s, "Category").distinct, 7);
        assert_eq!(table(&s, "Model").rows.len(), TOP);
        assert!(took < std::time::Duration::from_millis(250), "{took:?}");
    }

    // ── from where the facts come ─────────────────────────────────────────

    fn summary(icao: u32) -> Summary {
        Summary {
            icao,
            callsign: Some("KLM1234".into()),
            registration: None,
            first_seen: 1,
            last_seen: 2,
            points: 0,
            category: Some("heavy".into()),
            type_name: Some("747-8".into()),
            operator: Some("KLM".into()),
            origin: Some("AMS".into()),
            destination: Some("JFK".into()),
        }
    }

    #[test]
    fn the_stations_list_becomes_facts() {
        let f = facts_from_summaries(&[summary(7)]);
        assert_eq!(f, vec![carded(fact(7, "KLM1234", "heavy"), "747-8", "KLM", "AMS", "JFK")]);
    }

    #[test]
    fn live_cards_and_the_relays_answers_become_facts() {
        let mut card = AircraftCard::default();
        card.flight = Some(super::super::aircraft_registry::Stamped {
            value: "KLM1234".into(),
            source: super::super::aircraft_registry::Source::Adsb,
            at: 5,
        });
        let info = Info {
            aircraft: Some(AircraftInfo {
                type_name: Some("747-8".into()),
                manufacturer: None,
                operator: Some("KLM".into()),
                registration: None,
            }),
            route: Some(Route {
                origin: Some(Airport { code: "AMS".into(), name: "Schiphol".into() }),
                destination: Some(Airport { code: "JFK".into(), name: "Kennedy".into() }),
            }),
            ..Info::bare(InfoStatus::Ok)
        };
        let f = facts_from_live([(7, &card)].into_iter(), [(7, &info), (8, &info)].into_iter());
        assert_eq!(f.len(), 2, "8 has a card from the relay and was not on the air now");
        let seven = f.iter().find(|x| x.icao == 7).unwrap();
        assert_eq!(seven.callsign.as_deref(), Some("KLM1234"));
        assert_eq!(seven.category, None, "the registry does not hold the category");
        assert_eq!(seven.origin.as_deref(), Some("AMS"));
        let bare = Info::bare(InfoStatus::Unknown);
        let none = facts_from_live(std::iter::empty(), [(9, &bare)].into_iter());
        assert!(none.is_empty(), "an answer with nothing in it is not a fact");
    }

    #[test]
    fn merging_keeps_each_aircraft_once_the_live_value_first_and_the_gaps_filled() {
        let live =
            vec![fact(1, "KLM1", ""), carded(fact(3, "XXX3", "light"), "A320", "X", "A", "B")];
        let mut hist_one = fact(1, "OLD1", "heavy");
        hist_one.type_name = Some("747-8".into());
        let hist = vec![hist_one, fact(2, "BAW2", "large"), fact(3, "OLD3", "heavy")];
        let m = merge(live, hist);
        assert_eq!(m.iter().map(|f| f.icao).collect::<Vec<_>>(), vec![1, 2, 3]);
        assert_eq!(m[0].callsign.as_deref(), Some("KLM1"), "live callsign");
        assert_eq!(m[0].category.as_deref(), Some(""), "a live value is kept even when thin");
        assert_eq!(m[0].type_name.as_deref(), Some("747-8"), "history fills the card's gap");
        assert_eq!(m[2].category.as_deref(), Some("light"));
        assert_eq!(m[2].type_name.as_deref(), Some("A320"));
    }

    #[test]
    fn when_both_have_the_card_the_live_one_wins() {
        let mut live = fact(1, "KLM1", "heavy");
        live.type_name = Some("new".into());
        live.operator = Some("new op".into());
        let mut old = fact(1, "KLM1", "heavy");
        old.type_name = Some("old".into());
        old.operator = Some("old op".into());
        old.origin = Some("AMS".into());
        let m = merge(vec![live], vec![old]);
        assert_eq!(m.len(), 1);
        assert_eq!(m[0].type_name.as_deref(), Some("new"));
        assert_eq!(m[0].operator.as_deref(), Some("new op"));
        assert_eq!(m[0].origin.as_deref(), Some("AMS"), "and the gap is still filled");
    }

    #[test]
    fn an_answer_that_is_not_ok_gives_no_fact_even_with_something_in_it() {
        for status in [InfoStatus::Unknown, InfoStatus::Unavailable, InfoStatus::Busy] {
            let info = Info {
                aircraft: Some(AircraftInfo {
                    type_name: Some("A320".into()),
                    manufacturer: None,
                    operator: None,
                    registration: None,
                }),
                ..Info::bare(status)
            };
            let f = facts_from_live(std::iter::empty(), [(1, &info)].into_iter());
            assert!(f.is_empty(), "{status:?}");
        }
    }

    // ── the line to the station ───────────────────────────────────────────

    fn reply(status: &str) -> String {
        format!(r#"{{"v":{FORMAT_VERSION},"status":"{status}","aircraft":[]}}"#)
    }

    #[test]
    fn the_stations_reply_is_a_list_or_a_reason() {
        assert_eq!(classify(Some(200), &reply("ok")), StatsState::Ready(vec![]));
        assert_eq!(classify(None, ""), StatsState::Unavailable(Reason::Offline));
        assert_eq!(
            classify(Some(503), &reply("disabled")),
            StatsState::Unavailable(Reason::Disabled)
        );
        assert_eq!(
            classify(Some(401), &reply("unauthorized")),
            StatsState::Unavailable(Reason::Unauthorized)
        );
        assert_eq!(classify(Some(200), "<html>"), StatsState::Unavailable(Reason::Unknown));
        let other = r#"{"v":999,"status":"ok","aircraft":[]}"#;
        assert_eq!(classify(Some(200), other), StatsState::Unavailable(Reason::Unknown));
        assert_eq!(classify(Some(400), &reply("ok")), StatsState::Unavailable(Reason::Unknown));
    }

    #[test]
    fn nothing_is_asked_while_the_view_is_closed_or_there_is_no_station() {
        let mut s = StatsStore::default();
        assert!(!s.next_request(true, 0), "closed");
        s.set_on(true);
        assert!(!s.next_request(false, 0), "no station");
        assert!(s.next_request(true, 0), "open, with a station");
    }

    #[test]
    fn one_request_at_a_time_and_the_line_frees_on_the_answer_or_the_timeout() {
        let mut s = StatsStore::default();
        s.set_on(true);
        assert!(s.next_request(true, 100));
        assert!(s.line_busy(101));
        assert!(!s.next_request(true, 101), "one is out");
        s.deliver(StatsState::Ready(vec![]), 105);
        assert!(!s.line_busy(105));

        let mut t = StatsStore::default();
        t.set_on(true);
        assert!(t.next_request(true, 0));
        assert!(!t.next_request(true, FLIGHT_TIMEOUT_S - 1));
        assert!(t.next_request(true, FLIGHT_TIMEOUT_S), "a request that never came back");
    }

    #[test]
    fn a_list_is_fetched_again_a_minute_on_and_a_dead_end_is_not_retried() {
        let mut s = StatsStore::default();
        s.set_on(true);
        assert!(s.next_request(true, 0));
        s.deliver(StatsState::Ready(vec![]), 1);
        assert!(!s.next_request(true, REFRESH_S));
        assert!(s.next_request(true, 1 + REFRESH_S));
        s.deliver(StatsState::Unavailable(Reason::Disabled), 100);
        assert!(!s.next_request(true, 100 + 10 * REFRESH_S), "off at the station: not asked again");
        s.deliver(StatsState::Unavailable(Reason::Offline), 100);
        assert!(!s.next_request(true, 100 + REFRESH_S - 1));
        assert!(s.next_request(true, 100 + REFRESH_S), "out of reach may come back");
    }

    #[test]
    fn closing_the_view_forgets_the_list() {
        let mut s = StatsStore::default();
        s.set_on(true);
        s.next_request(true, 0);
        s.deliver(StatsState::Ready(vec![summary(1)]), 1);
        s.set_on(false);
        assert_eq!(*s.state(), StatsState::Idle);
        s.deliver(StatsState::Ready(vec![summary(2)]), 2);
        assert_eq!(*s.state(), StatsState::Idle, "an answer that arrives after closing is dropped");
    }

    #[test]
    fn the_list_comes_from_the_connected_station_only() {
        assert_eq!(
            list_url("wss://shack.example/ws").as_deref(),
            Some("https://shack.example/aircraft-history")
        );
        assert_eq!(list_url("not an address"), None);
    }

    // ── the line on where the numbers come from ──────────────────────────

    #[test]
    fn the_note_says_where_the_numbers_come_from() {
        let n = |s: &StatsState, st| source_note(s, st);
        assert!(n(&StatsState::Idle, false).contains("heard now"), "no station");
        assert!(n(&StatsState::Idle, true).contains("Asking"));
        assert!(n(&StatsState::Loading, true).contains("Asking"));
        assert!(n(&StatsState::Ready(vec![]), true).contains("24 h"));
        let off = n(&StatsState::Unavailable(Reason::Disabled), true);
        assert!(off.contains("not switched its history on") && off.contains("heard now"), "{off}");
        let out = n(&StatsState::Unavailable(Reason::Offline), true);
        assert!(out.starts_with("The station did not answer"), "{out}");
        assert_ne!(out, off, "each reason has its own words");
    }

    // ── what this module may not do ───────────────────────────────────────

    #[test]
    fn this_module_reaches_nothing_and_holds_no_picture() {
        let src = include_str!("aircraft_stats.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap()];
        for banned in
            ["ureq", "reqwest", "TcpStream", "adsbdb", "plnspttrs", "thumb_url", "std::fs", "egui"]
        {
            assert!(!code.contains(banned), "aircraft_stats.rs must not touch `{banned}`");
        }
        assert!(code.contains("StatsStore"), "the control: the scan sees what is there");
    }
}
