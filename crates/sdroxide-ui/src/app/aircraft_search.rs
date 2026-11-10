//! Searching the aircraft the program already holds.
//!
//! One query box serves the ADS-B table, the AIRCRAFT window's Cards list and
//! the map. It reads what is in memory — the ADS-B list, the registry's cards
//! and the answers the station's relay has already given — and **asks for
//! nothing**: no request, no download, no storage. The type and operator of an
//! aircraft are the relay's to give, so they are searchable only once its card
//! has been looked up; a query that names them says how many aircraft that
//! leaves out, so an empty result is never a silent one.
//!
//! The matching is pure and kept apart from the drawing so it can be tested.

use std::collections::HashSet;

use crate::i18n::tr;
use eframe::egui::{self, RichText};
use sdroxide_types::AdsbAircraft;

use super::aircraft_card::{self, CardKey, Row, icao_hex};
use super::aircraft_online::{OnlineState, OnlineStore};
use super::aircraft_registry::{AircraftCard, AircraftRegistry, FallbackKey};

/// The field a word is limited to by its prefix (`reg:PH`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Field {
    Icao,
    Call,
    Reg,
    Type,
    Operator,
}

/// What was typed, as words that must all match.
#[derive(Debug, Clone, Default, PartialEq)]
pub(in crate::app) struct Query {
    terms: Vec<(Option<Field>, String)>,
}

/// Everything about one aircraft the search can read, upper-cased. A field
/// nobody has is empty.
#[derive(Debug, Clone, Default, PartialEq)]
pub(in crate::app) struct Haystack {
    pub icao: String,
    pub callsign: String,
    pub registration: String,
    /// Manufacturer and type, from the relay.
    pub kind: String,
    /// The operator, from the relay.
    pub operator: String,
    /// The ADS-B emitter category ("heavy", "rotorcraft").
    pub category: String,
    /// Whether the relay's answer is in memory for this aircraft.
    pub online: bool,
}

impl Field {
    fn from_prefix(p: &str) -> Option<Field> {
        match p.to_ascii_lowercase().as_str() {
            "icao" => Some(Field::Icao),
            "call" => Some(Field::Call),
            "reg" => Some(Field::Reg),
            "type" => Some(Field::Type),
            "op" => Some(Field::Operator),
            _ => None,
        }
    }
}

impl Query {
    /// Words split on whitespace, upper-cased. A word that begins with a known
    /// prefix and a colon is limited to that field; any other word, a colon in
    /// it or not, is plain text. A prefix with nothing after it yet is left out,
    /// so a half-typed `reg:` narrows nothing instead of emptying the list.
    pub(in crate::app) fn parse(text: &str) -> Query {
        let terms = text
            .split_whitespace()
            .filter_map(|w| match w.split_once(':') {
                Some((p, rest)) if Field::from_prefix(p).is_some() => {
                    (!rest.is_empty()).then(|| (Field::from_prefix(p), rest.to_uppercase()))
                }
                _ => Some((None, w.to_uppercase())),
            })
            .collect();
        Query { terms }
    }

    pub(in crate::app) fn is_blank(&self) -> bool {
        self.terms.is_empty()
    }

    /// Whether a word is limited to the type or the operator, which only the
    /// relay can supply.
    pub(in crate::app) fn names_online_fields(&self) -> bool {
        self.terms.iter().any(|(f, _)| matches!(f, Some(Field::Type | Field::Operator)))
    }

    pub(in crate::app) fn matches(&self, h: &Haystack) -> bool {
        self.terms.iter().all(|(field, t)| match field {
            Some(Field::Icao) => h.icao.contains(t.as_str()),
            Some(Field::Call) => h.callsign.contains(t.as_str()),
            Some(Field::Reg) => h.registration.contains(t.as_str()),
            Some(Field::Type) => h.kind.contains(t.as_str()),
            Some(Field::Operator) => h.operator.contains(t.as_str()),
            None => [&h.icao, &h.callsign, &h.registration, &h.kind, &h.operator, &h.category]
                .iter()
                .any(|f| f.contains(t.as_str())),
        })
    }
}

/// What the relay's answer adds, when it is in memory: nothing otherwise.
fn add_online(h: &mut Haystack, state: &OnlineState) {
    let OnlineState::Ready(info) = state else { return };
    h.online = true;
    let Some(a) = &info.aircraft else { return };
    h.kind = [a.manufacturer.as_deref(), a.type_name.as_deref()]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .join(" ")
        .to_uppercase();
    h.operator = a.operator.as_deref().unwrap_or_default().to_uppercase();
    // The radio's registration is the one shown first; the relay fills a gap.
    if h.registration.is_empty() {
        h.registration = a.registration.as_deref().unwrap_or_default().to_uppercase();
    }
}

fn upper(s: Option<&str>) -> String {
    s.map(|s| s.trim().to_uppercase()).unwrap_or_default()
}

/// What the search reads of an ADS-B aircraft.
pub(in crate::app) fn haystack_adsb(
    a: &AdsbAircraft,
    card: Option<&AircraftCard>,
    state: &OnlineState,
) -> Haystack {
    let flight = card.and_then(|c| c.flight.as_ref()).map(|f| f.value.as_str());
    let own = a.callsign.trim();
    let mut h = Haystack {
        icao: a.hex(),
        callsign: upper(Some(if own.is_empty() { flight.unwrap_or_default() } else { own })),
        registration: upper(card.and_then(|c| c.registration.as_ref()).map(|r| r.value.as_str())),
        category: upper(a.category.as_deref()),
        ..Haystack::default()
    };
    add_online(&mut h, state);
    h
}

/// What the search reads of a card in the registry.
pub(in crate::app) fn haystack_card(
    key: &CardKey,
    card: &AircraftCard,
    state: &OnlineState,
) -> Haystack {
    let flight = card.flight.as_ref().map(|f| f.value.as_str());
    let (icao, callsign) = match key {
        CardKey::Icao(i) => (icao_hex(*i), flight.map(str::to_string)),
        CardKey::Fallback(FallbackKey::Flight(f)) => {
            (String::new(), Some(flight.unwrap_or(f.as_str()).to_string()))
        }
        CardKey::Fallback(FallbackKey::AircraftId(_)) => {
            (String::new(), flight.map(str::to_string))
        }
    };
    let mut h = Haystack {
        icao,
        callsign: upper(callsign.as_deref()),
        registration: upper(card.registration.as_ref().map(|r| r.value.as_str())),
        ..Haystack::default()
    };
    add_online(&mut h, state);
    h
}

/// The note under the box when the query names a field only the relay gives:
/// how many of the aircraft searched have no answer in memory.
pub(in crate::app) fn online_hint(q: &Query, all: &[Haystack]) -> Option<String> {
    if !q.names_online_fields() {
        return None;
    }
    let missing = all.iter().filter(|h| !h.online).count();
    (missing > 0).then(|| {
        tr!("aircraft-search-missing-online", missing = missing, total = all.len())
    })
}

/// `"12 of 47"`.
pub(in crate::app) fn shown_label(shown: usize, total: usize) -> String {
    tr!("aircraft-search-shown-of", shown = shown, total = total)
}

/// What a search over the ADS-B list found.
#[derive(Debug, Default, PartialEq)]
pub(in crate::app) struct AdsbFound {
    /// The addresses that match; `None` when nothing was asked, so everything shows.
    pub visible: Option<HashSet<u32>>,
    pub total: usize,
    pub hint: Option<String>,
}

/// Search the ADS-B list. Reads the registry and the relay's answers already in
/// memory; the store is only borrowed to be read.
pub(in crate::app) fn search_adsb(
    q: &Query,
    aircraft: &[AdsbAircraft],
    registry: &AircraftRegistry,
    online: &OnlineStore,
    has_station: bool,
) -> AdsbFound {
    if q.is_blank() {
        return AdsbFound { visible: None, total: aircraft.len(), hint: None };
    }
    let hays: Vec<Haystack> = aircraft
        .iter()
        .map(|a| haystack_adsb(a, registry.card(a.icao), online.state_for(a.icao, has_station)))
        .collect();
    let visible =
        aircraft.iter().zip(&hays).filter(|(_, h)| q.matches(h)).map(|(a, _)| a.icao).collect();
    AdsbFound { visible: Some(visible), total: aircraft.len(), hint: online_hint(q, &hays) }
}

/// What a search over the Cards list found.
#[derive(Debug, Default, PartialEq)]
pub(in crate::app) struct RowsFound {
    pub rows: Vec<Row>,
    pub total: usize,
    pub hint: Option<String>,
}

/// Keep the rows that match, in the order they came. A card without an address
/// has no relay answer (the relay is asked by address), so it counts as
/// without online data.
pub(in crate::app) fn search_rows(
    q: &Query,
    rows: Vec<Row>,
    registry: &AircraftRegistry,
    online: &OnlineStore,
    has_station: bool,
) -> RowsFound {
    let total = rows.len();
    if q.is_blank() {
        return RowsFound { rows, total, hint: None };
    }
    let mut hays = Vec::with_capacity(total);
    let mut kept = Vec::new();
    for r in rows {
        let Some(card) = aircraft_card::lookup(registry, &r.key) else { continue };
        let state = match &r.key {
            CardKey::Icao(i) => online.state_for(*i, has_station),
            CardKey::Fallback(_) => &OnlineState::Idle,
        };
        let h = haystack_card(&r.key, card, state);
        if q.matches(&h) {
            kept.push(r);
        }
        hays.push(h);
    }
    RowsFound { rows: kept, total, hint: online_hint(q, &hays) }
}

/// The card to show on the right of the Cards list.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) struct Open {
    pub key: CardKey,
    /// Shown only because the filter left it first, not because it was chosen.
    /// Such a card is not asked about and its photo is not fetched: the box
    /// must not cause a request.
    pub by_filter: bool,
}

/// The remembered card while it is among `rows`; else the first row, with the
/// remembered choice left to the caller to keep. `filtering` is whether a query
/// is typed: without one the list is as it always was and nothing is "by filter".
pub(in crate::app) fn open_card(
    remembered: Option<&CardKey>,
    rows: &[Row],
    filtering: bool,
) -> Option<Open> {
    if let Some(k) = remembered.filter(|k| rows.iter().any(|r| &r.key == *k)) {
        return Some(Open { key: k.clone(), by_filter: false });
    }
    rows.first().map(|r| Open { key: r.key.clone(), by_filter: filtering })
}

/// What the box accepts, for its hover text.
fn help() -> String {
    tr!("aircraft-search-help")
}

/// The search box: the field, a button to clear it, how many aircraft it leaves,
/// and the note when the query names a field only the online lookup gives.
pub(in crate::app) fn search_box(
    ui: &mut egui::Ui,
    text: &mut String,
    width: f32,
    counts: Option<(usize, usize)>,
    hint: Option<&str>,
) {
    ui.horizontal(|ui| {
        ui.add(egui::TextEdit::singleline(text).hint_text(tr!("aircraft-search-hint")).desired_width(width))
            .on_hover_text(help());
        if !text.is_empty() && ui.small_button("✕").on_hover_text(tr!("aircraft-search-clear")).clicked() {
            text.clear();
        }
        if let Some((shown, total)) = counts {
            ui.label(RichText::new(shown_label(shown, total)).size(10.5).weak());
        }
    });
    if let Some(h) = hint {
        ui.label(RichText::new(h).size(10.5).color(crate::theme::YELLOW()));
    }
}

#[cfg(test)]
mod tests {
    use super::super::aircraft_online::Answer;
    use super::super::aircraft_registry::{Source, Stamped};
    use super::*;
    use sdroxide_aircraft_online::{AircraftInfo, Info, Status};

    const NOW: i64 = 1_000_000;

    fn stamped<T>(value: T) -> Option<Stamped<T>> {
        Some(Stamped { value, source: Source::Vdl2, at: NOW })
    }

    fn adsb(icao: u32, call: &str) -> AdsbAircraft {
        let mut a = AdsbAircraft::new(icao, NOW);
        a.callsign = call.to_string();
        a.lat = Some(52.0);
        a.lon = Some(5.0);
        a.last_pos_at = NOW;
        a
    }

    fn card(flight: Option<&str>, reg: Option<&str>) -> AircraftCard {
        AircraftCard {
            flight: flight.and_then(|f| stamped(f.to_string())),
            registration: reg.and_then(|r| stamped(r.to_string())),
            last_seen: NOW,
            ..AircraftCard::default()
        }
    }

    fn info(kind: Option<&str>, maker: Option<&str>, op: Option<&str>, reg: Option<&str>) -> Info {
        let mut i = Info::bare(Status::Ok);
        i.aircraft = Some(AircraftInfo {
            type_name: kind.map(Into::into),
            manufacturer: maker.map(Into::into),
            operator: op.map(Into::into),
            registration: reg.map(Into::into),
        });
        i
    }

    fn ready(i: Info) -> OnlineState {
        OnlineState::Ready(Box::new(i))
    }

    fn q(text: &str) -> Query {
        Query::parse(text)
    }

    /// A KLM 737 with everything known, as the box sees it.
    fn klm() -> Haystack {
        haystack_adsb(
            &adsb(0x4840D6, "KLM1234 "),
            Some(&card(Some("KLM1234"), Some("PH-BXA"))),
            &ready(info(Some("737-800"), Some("Boeing"), Some("KLM Royal Dutch"), None)),
        )
    }

    /// The same aircraft before anyone looked it up online.
    fn klm_unlooked() -> Haystack {
        haystack_adsb(
            &adsb(0x4840D6, "KLM1234"),
            Some(&card(Some("KLM1234"), Some("PH-BXA"))),
            &OnlineState::Idle,
        )
    }

    // ── what matches ──────────────────────────────────────────────────────

    #[test]
    fn an_empty_or_blank_query_matches_everything() {
        for text in ["", "   ", "\t"] {
            assert!(q(text).is_blank(), "{text:?}");
            assert!(q(text).matches(&klm()), "{text:?}");
        }
        // The control: a real query is not blank and does not match everything.
        assert!(!q("zzz").is_blank());
        assert!(!q("zzz").matches(&klm()));
    }

    #[test]
    fn the_icao_matches_in_part_and_in_any_case() {
        for text in ["4840d6", "4840D6", "840d", "icao:4840"] {
            assert!(q(text).matches(&klm()), "{text}");
        }
        assert!(!q("4840d7").matches(&klm()));
        assert!(!q("icao:klm").matches(&klm()), "the address is not the callsign");
    }

    #[test]
    fn the_callsign_matches_ignoring_case_and_padding() {
        // The squitter pads the callsign to eight characters.
        for text in ["klm1234", "KLM", "m12", "call:klm"] {
            assert!(q(text).matches(&klm()), "{text}");
        }
        assert!(!q("tra").matches(&klm()));
        assert_eq!(klm().callsign, "KLM1234", "the padding is not part of what is searched");
    }

    #[test]
    fn the_callsign_falls_back_to_the_flight_a_vdl2_message_gave() {
        let h = haystack_adsb(
            &adsb(0x4840D6, ""),
            Some(&card(Some("KL1234"), None)),
            &OnlineState::Idle,
        );
        assert!(q("kl1234").matches(&h));
        assert!(!q("kl9999").matches(&h), "the flight is searched, not everything");
        assert_eq!(h.callsign, "KL1234");
    }

    #[test]
    fn the_registration_matches_from_vdl2() {
        for text in ["ph-bxa", "bxa", "reg:ph"] {
            assert!(q(text).matches(&klm_unlooked()), "{text}");
        }
        assert!(!q("ph-zzz").matches(&klm_unlooked()));
    }

    #[test]
    fn type_manufacturer_and_operator_match_only_once_loaded() {
        for text in ["737", "boeing", "type:737-800", "type:boeing", "klm royal", "op:royal"] {
            let words: Vec<&str> = text.split(' ').collect();
            assert!(q(text).matches(&klm()), "{text} ({words:?}) should match a loaded card");
            assert!(
                !q(text).matches(&klm_unlooked()),
                "{text} must not match before the relay has answered"
            );
        }
    }

    #[test]
    fn only_a_ready_answer_counts_as_loaded() {
        let i = || info(Some("A320"), Some("Airbus"), Some("Transavia"), None);
        for (state, loaded) in [
            (ready(i()), true),
            (OnlineState::Idle, false),
            (OnlineState::Loading, false),
            (OnlineState::Unavailable(super::super::aircraft_online::Reason::Busy), false),
        ] {
            let h = haystack_adsb(&adsb(1, "TRA1"), None, &state);
            assert_eq!(h.online, loaded);
            assert_eq!(q("airbus").matches(&h), loaded);
        }
    }

    #[test]
    fn the_radio_registration_wins_over_the_online_one() {
        let h = haystack_adsb(
            &adsb(1, "KLM1"),
            Some(&card(None, Some("PH-BXA"))),
            &ready(info(None, None, None, Some("PH-ONLINE"))),
        );
        assert!(q("reg:bxa").matches(&h));
        assert!(!q("reg:online").matches(&h), "one registration, the radio's");
        // With none heard, the relay's is used.
        let h = haystack_adsb(
            &adsb(1, "KLM1"),
            None,
            &ready(info(None, None, None, Some("PH-ONLINE"))),
        );
        assert!(q("reg:online").matches(&h));
    }

    #[test]
    fn the_emitter_category_matches_a_plain_word() {
        let mut a = adsb(1, "KLM1");
        a.category = Some("heavy".into());
        let h = haystack_adsb(&a, None, &OnlineState::Idle);
        assert!(q("heavy").matches(&h));
        assert!(!q("type:heavy").matches(&h), "the category is not the type");
    }

    #[test]
    fn every_word_must_match_and_may_land_on_different_fields() {
        assert!(q("klm boeing").matches(&klm()));
        assert!(q("  klm   ph-bxa ").matches(&klm()));
        assert!(!q("klm airbus").matches(&klm()), "one word missing is no match");
        assert!(!q("tra boeing").matches(&klm()));
    }

    #[test]
    fn a_field_prefix_limits_the_search_to_that_field() {
        let h = haystack_adsb(
            &adsb(0xABC123, "ABC123"),
            Some(&card(None, Some("PH-BXA"))),
            &OnlineState::Idle,
        );
        assert!(q("abc").matches(&h), "a plain word looks everywhere");
        assert!(q("call:abc").matches(&h));
        assert!(q("icao:abc").matches(&h));
        assert!(!q("reg:abc").matches(&h), "reg: looks at the registration alone");
        assert!(!q("type:abc").matches(&h));
        assert!(!q("op:abc").matches(&h));
    }

    #[test]
    fn an_unknown_prefix_or_a_bare_colon_is_searched_as_plain_text() {
        let h = haystack_adsb(&adsb(1, "AB:CD"), None, &OnlineState::Idle);
        assert!(q("ab:cd").matches(&h), "not a field, so the whole word is the text");
        assert!(!q("zz:cd").matches(&h));
        // A prefix with nothing after it yet — the operator is still typing —
        // narrows nothing rather than emptying the list.
        assert!(q("reg:").matches(&h));
        assert!(q("reg:").is_blank());
    }

    #[test]
    fn a_card_without_an_address_is_searched_by_its_flight_and_registration() {
        let key = CardKey::Fallback(FallbackKey::Flight("AF123".into()));
        let h = haystack_card(&key, &card(None, Some("F-HBXA")), &OnlineState::Idle);
        assert!(q("af123").matches(&h), "the key's flight is its callsign");
        assert!(q("f-hbxa").matches(&h));
        assert!(!q("icao:af").matches(&h), "no address, so nothing for icao: to find");

        let key = CardKey::Icao(0x3C6444);
        let h = haystack_card(&key, &card(Some("DLH9"), None), &OnlineState::Idle);
        assert!(q("3c6444").matches(&h));
        assert!(q("dlh9").matches(&h));
    }

    // ── what it says when it cannot know ──────────────────────────────────

    #[test]
    fn the_hint_counts_aircraft_with_no_online_data_when_the_query_names_type_or_operator() {
        let all = [klm(), klm_unlooked(), klm_unlooked()];
        for text in ["type:737", "op:klm", "reg:ph op:klm"] {
            assert!(q(text).names_online_fields(), "{text}");
            assert_eq!(
                online_hint(&q(text), &all).as_deref(),
                Some("2 of 3 aircraft without online data — open their card"),
                "{text}"
            );
        }
    }

    #[test]
    fn the_hint_is_silent_unless_the_query_names_type_or_operator_and_something_is_missing() {
        let all = [klm(), klm_unlooked()];
        // Names other fields only.
        for text in ["", "klm", "reg:ph", "call:klm icao:48"] {
            assert!(!q(text).names_online_fields(), "{text}");
            assert_eq!(online_hint(&q(text), &all), None, "{text}");
        }
        // Names the type, but nothing is missing.
        assert_eq!(online_hint(&q("type:737"), &[klm(), klm()]), None);
        // The control: the same query with one missing does speak.
        assert!(online_hint(&q("type:737"), &[klm(), klm_unlooked()]).is_some());
    }

    #[test]
    fn the_count_label_reads_shown_of_total() {
        assert_eq!(shown_label(12, 47), "12 of 47");
        assert_eq!(shown_label(0, 5), "0 of 5");
    }

    // ── over the lists the program holds ──────────────────────────────────

    fn world() -> (Vec<AdsbAircraft>, AircraftRegistry, OnlineStore) {
        let list =
            vec![adsb(0x4840D6, "KLM1234"), adsb(0x3C6444, "DLH9AB"), adsb(0x4CA1FA, "RYR55")];
        let mut reg = AircraftRegistry::default();
        for a in &list {
            reg.ingest_adsb(a);
        }
        let mut store = OnlineStore::default();
        store.deliver(
            0x4840D6,
            Answer::Info(Box::new(info(Some("737-800"), Some("Boeing"), Some("KLM"), None))),
            NOW,
        );
        (list, reg, store)
    }

    #[test]
    fn the_adsb_list_is_searched_by_address_and_by_callsign_as_the_old_filter_was() {
        let (list, reg, store) = world();
        let f = search_adsb(&q("klm"), &list, &reg, &store, true);
        assert_eq!(f.visible, Some(HashSet::from([0x4840D6])));
        assert_eq!(f.total, 3);
        let f = search_adsb(&q("3c64"), &list, &reg, &store, true);
        assert_eq!(f.visible, Some(HashSet::from([0x3C6444])));
        // Two match, the third does not.
        let f = search_adsb(&q("c"), &list, &reg, &store, true);
        assert_eq!(f.visible, Some(HashSet::from([0x3C6444, 0x4CA1FA])));
        let f = search_adsb(&q("zzzz"), &list, &reg, &store, true);
        assert_eq!(f.visible, Some(HashSet::new()), "nothing matches: an empty set, not no filter");
    }

    #[test]
    fn a_blank_query_filters_nothing() {
        let (list, reg, store) = world();
        let f = search_adsb(&q("  "), &list, &reg, &store, true);
        assert_eq!(f.visible, None);
        assert_eq!(f.hint, None);
        // The control: it is the query that decides.
        assert!(search_adsb(&q("klm"), &list, &reg, &store, true).visible.is_some());
    }

    #[test]
    fn the_adsb_search_finds_a_type_only_for_the_aircraft_already_looked_up() {
        let (list, reg, store) = world();
        let f = search_adsb(&q("type:boeing"), &list, &reg, &store, true);
        assert_eq!(f.visible, Some(HashSet::from([0x4840D6])));
        assert_eq!(
            f.hint.as_deref(),
            Some("2 of 3 aircraft without online data — open their card")
        );
    }

    #[test]
    fn searching_asks_for_nothing() {
        let (list, reg, mut store) = world();
        let loaded = store.len();
        for text in ["klm", "type:boeing", "op:klm", "zzz", ""] {
            let _ = search_adsb(&q(text), &list, &reg, &store, true);
            let rows = super::super::aircraft_card::rows(&reg, NOW);
            let _ = search_rows(&q(text), rows, &reg, &store, true);
        }
        assert_eq!(store.len(), loaded, "no entry was added");
        assert!(!store.line_busy(NOW), "no question went out");
        assert_eq!(
            *store.state_for(0x3C6444, true),
            OnlineState::Idle,
            "an unlooked card stays idle"
        );
        // The control: the store does change when something does ask.
        store.deliver(0x3C6444, Answer::Info(Box::new(Info::bare(Status::Ok))), NOW);
        assert_eq!(store.len(), loaded + 1);
    }

    #[test]
    fn the_cards_list_is_filtered_and_keeps_its_order() {
        let (_, mut reg, store) = world();
        let mut late = adsb(0x4CA1FA, "RYR55");
        late.last_at = NOW + 5;
        reg.ingest_adsb(&late);
        let all = super::super::aircraft_card::rows(&reg, NOW);
        let order: Vec<_> = all.iter().map(|r| r.key.clone()).collect();

        let f = search_rows(&q("c"), all.clone(), &reg, &store, true);
        assert_eq!(f.total, 3);
        let kept: Vec<_> = f.rows.iter().map(|r| r.key.clone()).collect();
        let expect: Vec<_> = order
            .iter()
            .filter(|k| matches!(k, CardKey::Icao(0x3C6444 | 0x4CA1FA)))
            .cloned()
            .collect();
        assert_eq!(expect.len(), 2);
        assert_eq!(kept, expect, "the matching cards, in the order the list already had");

        let f = search_rows(&q("ryr"), all.clone(), &reg, &store, true);
        assert_eq!(f.rows.len(), 1);
        assert_eq!(f.rows[0].key, CardKey::Icao(0x4CA1FA));

        let f = search_rows(&q("type:boeing"), all.clone(), &reg, &store, true);
        assert_eq!(f.rows.len(), 1, "the loaded card is found by its type");
        assert!(f.hint.is_some());

        assert_eq!(search_rows(&q(""), all.clone(), &reg, &store, true).rows.len(), 3);
        assert!(search_rows(&q("zzz"), all, &reg, &store, true).rows.is_empty());
    }

    #[test]
    fn the_cards_list_includes_the_aircraft_heard_without_an_address() {
        let mut reg = AircraftRegistry::default();
        let hf = sdroxide_types::HfdlDecode {
            unix: NOW,
            kind: "performance-data".into(),
            gs: None,
            freq_khz: 8_912,
            snr_db: None,
            freq_skew_hz: None,
            fec_corrected: None,
            details: String::new(),
            position: Some(sdroxide_types::HfdlFix {
                lat: 10.0,
                lon: -30.0,
                aircraft_id: Some(77),
                icao: None,
                flight: Some("AF123".into()),
            }),
        };
        reg.ingest_hfdl(&hf);
        let rows = super::super::aircraft_card::rows(&reg, NOW);
        assert_eq!(rows.len(), 1);
        let store = OnlineStore::default();
        assert_eq!(search_rows(&q("af123"), rows.clone(), &reg, &store, true).rows.len(), 1);
        assert!(search_rows(&q("lh400"), rows, &reg, &store, true).rows.is_empty());
    }

    // ── the card that is open ─────────────────────────────────────────────

    fn row(icao: u32) -> Row {
        Row { key: CardKey::Icao(icao), title: String::new(), detail: String::new(), age_s: 0 }
    }

    #[test]
    fn the_remembered_card_stays_open_while_it_matches() {
        let rows = [row(1), row(2), row(3)];
        let open = open_card(Some(&CardKey::Icao(2)), &rows, true).unwrap();
        assert_eq!(open, Open { key: CardKey::Icao(2), by_filter: false });
    }

    #[test]
    fn a_filtered_out_card_gives_way_to_the_first_match_without_being_forgotten() {
        let remembered = CardKey::Icao(9);
        let rows = [row(1), row(2)];
        let open = open_card(Some(&remembered), &rows, true).unwrap();
        assert_eq!(open, Open { key: CardKey::Icao(1), by_filter: true });
        // The choice is the caller's to keep: nothing here takes it away, and
        // once the filter lets it back it is the card shown again.
        let back = [row(1), row(9)];
        assert_eq!(open_card(Some(&remembered), &back, true).unwrap().key, remembered);
    }

    #[test]
    fn a_card_shown_only_by_the_filter_is_not_one_to_ask_about() {
        let rows = [row(1), row(2)];
        // With no choice made and a query typed, the first match is the filter's.
        assert!(open_card(None, &rows, true).unwrap().by_filter);
        // Without a query the list behaves as it always did.
        assert_eq!(
            open_card(None, &rows, false),
            Some(Open { key: CardKey::Icao(1), by_filter: false })
        );
        assert!(open_card(None, &[], true).is_none());
    }

    // ── what the search is allowed to do ──────────────────────────────────

    /// The search reads memory and nothing else: no question to the relay, no
    /// download, no storage, and no name of a service it must never reach.
    #[test]
    fn this_file_asks_for_nothing_and_stores_nothing() {
        let src = include_str!("aircraft_search.rs");
        let code = &src[..src.find("#[cfg(test)]").expect("the tests follow the code")];
        let banned = [
            ".ask(",
            "want_thumb(",
            "next_request(",
            ".poll(",
            "open_external",
            "localStorage",
            "sessionStorage",
            "IndexedDB",
            "web_sys",
            "adsbdb",
            "planespotters",
            "plnspttrs",
            "http",
        ];
        for b in banned {
            // The module's own prose names none of them either.
            assert!(!code.contains(b), "the search must not touch `{b}`");
        }
        // The control: the scan can see a thing that is there.
        assert!(code.contains("OnlineStore"));
    }
}
