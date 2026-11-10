//! What the station's relay adds to an aircraft card, and how it is worded.
//!
//! The radio's data is the card; this is the optional extra. Everything here is
//! a decision on plain values — which question to ask, whether to ask it again,
//! how an answer reads, which badge a line wears — so it is tested without a
//! socket, a clock or a window. The asking itself is [`super::aircraft_online_io`].
//!
//! The rules it keeps:
//!
//! * the radio's lines come first and are never replaced by an online one; the
//!   online data only fills what the radio has not heard;
//! * a client asks its own station's `GET /aircraft-info/{icao}` and nothing
//!   else — no key, no account, no call to a registry or a photo service;
//! * a station that has the relay off, or none, is asked once and left alone;
//!   a failure is asked again no sooner than [`RETRY_S`] — and none of it is an
//!   error on the card;
//! * all of it lives in memory for the length of the session. Nothing is
//!   written to the browser or to the settings: the durable cache is the
//!   station's.

// ==== IMPL ====
use std::collections::HashMap;

use sdroxide_aircraft_online::{
    FORMAT_VERSION, Info, Photo, Status, clean_callsign, clean_registration,
};

use super::aircraft_card::{self, CardKey, icao_hex};
use super::aircraft_registry::AircraftCard;
use crate::i18n::tr;

/// Seconds before a question the registry could not answer may be asked again.
pub(in crate::app) const RETRY_S: i64 = 300;
/// Seconds before a read that failed (no answer from the station or from the
/// services behind it) is tried again: it mends itself, so it is soon.
pub(in crate::app) const RETRY_OFFLINE_S: i64 = 30;
/// Seconds before a read the station refused for its rate ceiling is tried again.
pub(in crate::app) const RETRY_BUSY_S: i64 = 60;
/// Seconds after which a question that never came back is given up on. A
/// browser's `fetch` has no timeout of its own, so this is the only one there.
pub(in crate::app) const FLIGHT_TIMEOUT_S: i64 = 15;
/// Questions that may be out at once. A card that is slow or failing holds one
/// place, never the whole line; a few at a time keeps the relay's own ceiling
/// in sight.
pub(in crate::app) const MAX_IN_FLIGHT: usize = 3;
/// Cards whose online state is kept; the least recently looked at go first.
pub(in crate::app) const MAX_KEPT: usize = 64;

/// Why there is nothing online on a card.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::app) enum Reason {
    /// The station's operator has not switched the relay on.
    Disabled,
    /// No answer: the station could not reach the services, or this client
    /// could not reach the station, or the reply was not understood.
    Offline,
    /// The station's ceiling on questions per minute was reached.
    Busy,
    /// The station wants a sign-in this client does not have.
    Unauthorized,
    /// The registry answered and does not know this aircraft.
    Unknown,
    /// There is no station to ask: an engine running in this very program.
    NoStation,
}

impl Reason {
    /// What the card says about it. A read that failed says so and that it
    /// will be tried again, so it never fails in silence; an aircraft the
    /// registry simply does not know says nothing.
    pub(in crate::app) fn note(self) -> Option<String> {
        match self {
            Reason::Disabled => Some(tr!("aircraft-online-unavailable-disabled")),
            Reason::NoStation => Some(tr!("aircraft-online-unavailable-no-station")),
            Reason::Unauthorized => Some(tr!("aircraft-online-unavailable-unauthorized")),
            Reason::Offline | Reason::Busy => Some(tr!("aircraft-online-unavailable-retry")),
            Reason::Unknown => None,
        }
    }

    /// Seconds before the same question may be asked again; `None` is never in
    /// this session.
    pub(in crate::app) fn retry_after(self) -> Option<i64> {
        match self {
            Reason::Offline => Some(RETRY_OFFLINE_S),
            Reason::Busy => Some(RETRY_BUSY_S),
            Reason::Unknown => Some(RETRY_S),
            Reason::Disabled | Reason::Unauthorized | Reason::NoStation => None,
        }
    }

    /// Whether this ends the questions for every card, not just this one.
    fn is_station_wide(self) -> bool {
        matches!(self, Reason::Disabled | Reason::Unauthorized)
    }
}

/// What the card knows about the online half.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) enum OnlineState {
    /// Not asked yet.
    Idle,
    /// Asked, no answer yet.
    Loading,
    Ready(Box<Info>),
    Unavailable(Reason),
}

/// What came back from a question.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) enum Answer {
    Info(Box<Info>),
    Unavailable(Reason),
}

/// Read a reply: the HTTP status (`None` when there was none) and the body.
///
/// The relay writes an [`Info`] with every status, so the body is read first
/// and the code only decides what a body that is not one means — a reverse
/// proxy's own 503 page is a failure, not a relay that is switched off.
pub(in crate::app) fn classify(http_status: Option<u16>, body: &str) -> Answer {
    let Some(code) = http_status else { return Answer::Unavailable(Reason::Offline) };
    if let Ok(info) = serde_json::from_str::<Info>(body) {
        if info.v != FORMAT_VERSION {
            return Answer::Unavailable(Reason::Offline);
        }
        return match info.status {
            Status::Ok => Answer::Info(Box::new(info)),
            Status::Unknown | Status::Invalid => Answer::Unavailable(Reason::Unknown),
            Status::Unavailable => Answer::Unavailable(Reason::Offline),
            Status::Busy => Answer::Unavailable(Reason::Busy),
            Status::Disabled => Answer::Unavailable(Reason::Disabled),
            Status::Unauthorized => Answer::Unavailable(Reason::Unauthorized),
        };
    }
    Answer::Unavailable(match code {
        401 => Reason::Unauthorized,
        429 => Reason::Busy,
        _ => Reason::Offline,
    })
}

/// One question: an address, and what the radio heard of the flight.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) struct Want {
    pub icao: u32,
    pub callsign: Option<String>,
    pub registration: Option<String>,
}

/// The question the open card asks. A card with no ICAO address has none, and
/// a callsign or registration that is not clean is left out rather than sent.
pub(in crate::app) fn want_for(key: &CardKey, card: &AircraftCard) -> Option<Want> {
    let CardKey::Icao(icao) = key else { return None };
    Some(Want {
        icao: *icao & 0xff_ffff,
        callsign: card.flight.as_ref().and_then(|f| clean_callsign(&f.value)),
        registration: card.registration.as_ref().and_then(|r| clean_registration(&r.value)),
    })
}

/// Where the question goes: the station this client is connected to, by its
/// socket address (`wss://host/prefix/ws/2` → `https://host/prefix/…`).
pub(in crate::app) fn info_url(peer_url: &str, want: &Want) -> Option<String> {
    let base = crate::login::http_base(&crate::login::station_key(peer_url))?;
    let mut url = format!("{base}/aircraft-info/{}", icao_hex(want.icao));
    let mut sep = '?';
    for (name, value) in [("callsign", &want.callsign), ("reg", &want.registration)] {
        if let Some(v) = value {
            url.push(sep);
            url.push_str(&format!("{name}={v}"));
            sep = '&';
        }
    }
    Some(url)
}

/// Where the switch request goes: `POST {station}/aircraft-online?on=1|0`.
pub(in crate::app) fn switch_url(peer_url: &str, on: bool) -> Option<String> {
    let base = crate::login::http_base(&crate::login::station_key(peer_url))?;
    Some(format!("{base}/aircraft-online?on={}", u8::from(on)))
}

/// Where the station hands over the open card's thumbnail:
/// `GET {station}/aircraft-photo/{icao}`. The aircraft, never the picture's address.
pub(in crate::app) fn photo_url(peer_url: &str, icao: u32) -> Option<String> {
    let base = crate::login::http_base(&crate::login::station_key(peer_url))?;
    Some(format!("{base}/aircraft-photo/{}", super::aircraft_card::icao_hex(icao)))
}

/// Where the card being closed lets its thumbnail go:
/// `DELETE {station}/aircraft-photo/{icao}`. Only that aircraft's, so a card
/// opened a moment before keeps its own.
pub(in crate::app) fn photo_release_url(peer_url: &str, icao: u32) -> Option<String> {
    photo_url(peer_url, icao)
}

/// Where the station hands over the Wikipedia picture of the airport whose card
/// is open: `GET {station}/airport-photo?lang=en&title=Amsterdam_Airport_Schiphol`
/// (and `DELETE` of the same lets it go). A language and a title, never an
/// address: the station builds every address itself.
pub(in crate::app) fn airport_photo_url(peer_url: &str, lang: &str, title: &str) -> Option<String> {
    let base = crate::login::http_base(&crate::login::station_key(peer_url))?;
    Some(format!("{base}/airport-photo?lang={}&title={}", escape_query(lang), escape_query(title)))
}

/// The article's page, where the picture's credit points: its author and licence
/// are on that page. `lang` and `title` are the table's own.
pub(in crate::app) fn airport_page_url(lang: &str, title: &str) -> String {
    format!("https://{}.wikipedia.org/wiki/{}", escape_query(lang), escape_query(title))
}

/// Everything but letters, digits and `-._~` as `%XX`, byte by byte.
fn escape_query(text: &str) -> String {
    text.bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                char::from(b).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}

/// Seconds a thumbnail that failed waits before it is asked for again while its
/// card stays open, and how many times: the station may have been busy or the
/// picture's host slow, which mends itself.
pub(in crate::app) const THUMB_RETRY_S: f64 = 4.0;
pub(in crate::app) const THUMB_TRIES: u8 = 3;

/// Whether the thumbnail held is for a card that is no longer open: nothing has
/// asked for it in the frame before this one or the one before that. The card
/// that is open asks every frame it is drawn.
pub(in crate::app) fn thumb_unwanted(last_wanted: Option<u64>, frame: u64) -> bool {
    last_wanted.is_none_or(|w| frame > w + 1)
}

/// Where the station hands over the local time's offset and the temperature at
/// the airport whose box is open: `GET {station}/airport-weather?lat=52.31&lon=4.76`.
/// The airport's place and nothing else: the station builds the address.
pub(in crate::app) fn airport_weather_url(peer_url: &str, lat: f64, lon: f64) -> Option<String> {
    let base = crate::login::http_base(&crate::login::station_key(peer_url))?;
    // The same rounding the station applies, so that what is asked is what is kept.
    let key = sdroxide_aircraft_online::Spot::new(lat, lon)?.key();
    let (lat, lon) = key.split_once(',')?;
    Some(format!("{base}/airport-weather?lat={lat}&lon={lon}"))
}

/// What the station's answer to a weather question means.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::app) enum Reading {
    Got(sdroxide_aircraft_online::Weather),
    /// Busy, down or out of reach: worth asking again in a moment.
    Retry,
    /// Turned away (relay off, sign-in refused, a place it will not look up) or
    /// an answer that is not one: asking again would only repeat it.
    Gone,
}

pub(in crate::app) fn classify_weather(status: Option<u16>, body: &str) -> Reading {
    match status {
        Some(200) => serde_json::from_str::<sdroxide_aircraft_online::Weather>(body)
            .ok()
            .filter(|w| w.v == sdroxide_aircraft_online::WEATHER_VERSION)
            .map_or(Reading::Gone, Reading::Got),
        Some(429 | 502 | 504) | None => Reading::Retry,
        Some(_) => Reading::Gone,
    }
}

/// How long the weather of an open box is trusted before it is asked again. The
/// station keeps an answer half a minute; the temperature moves slowly and the
/// zone's offset changes twice a year.
pub(in crate::app) const WEATHER_REFRESH_S: f64 = 600.0;

/// The weather of the one airport whose box is open, in memory, for as long as
/// the box stays open. Nothing is written anywhere.
#[derive(Default)]
pub(in crate::app) struct WeatherStore {
    entry: Option<WeatherEntry>,
}

struct WeatherEntry {
    airport: usize,
    /// The pass in which the box last asked, as for the thumbnails.
    wanted: u64,
    weather: Option<sdroxide_aircraft_online::Weather>,
    asking: bool,
    /// Questions sent since the last answer that came.
    tries: u8,
    /// When the last question was sent or answered (egui seconds).
    at: f64,
    gone: bool,
}

/// What an open box knows about its airport's clock and air.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::app) enum Clock {
    /// Not asked, not asked any more, or never answered: UTC only.
    Offline,
    /// On its way.
    Asking,
    Known(sdroxide_aircraft_online::Weather),
}

impl WeatherStore {
    /// Called every frame the box is drawn. True when a question is due: the
    /// first one for this airport, a repeat after a failure (a few times, a few
    /// seconds apart) or a refresh once the answer is old.
    pub(in crate::app) fn step(&mut self, airport: usize, pass: u64, now: f64) -> bool {
        let Some(e) = self.entry.as_mut().filter(|e| e.airport == airport) else {
            self.entry = Some(WeatherEntry {
                airport,
                wanted: pass,
                weather: None,
                asking: true,
                tries: 1,
                at: now,
                gone: false,
            });
            return true;
        };
        e.wanted = pass;
        if e.asking || e.gone {
            return false;
        }
        let due = match e.weather {
            Some(_) => now - e.at >= WEATHER_REFRESH_S,
            None => e.tries < THUMB_TRIES && now - e.at >= THUMB_RETRY_S,
        };
        if due {
            e.asking = true;
            e.tries += 1;
            e.at = now;
        }
        due
    }

    /// Take the station's answer for `airport`; one for another airport (the box
    /// moved on while it was on its way) is dropped.
    pub(in crate::app) fn deliver(&mut self, airport: usize, reading: Reading, now: f64) {
        let Some(e) = self.entry.as_mut().filter(|e| e.airport == airport) else { return };
        e.asking = false;
        e.at = now;
        match reading {
            Reading::Got(w) => {
                e.weather = Some(w);
                e.tries = 0;
            }
            Reading::Retry => {}
            Reading::Gone => e.gone = true,
        }
    }

    pub(in crate::app) fn clock(&self, airport: usize) -> Clock {
        let Some(e) = self.entry.as_ref().filter(|e| e.airport == airport) else {
            return Clock::Offline;
        };
        match e.weather {
            Some(w) => Clock::Known(w),
            None if e.asking || (!e.gone && e.tries < THUMB_TRIES) => Clock::Asking,
            None => Clock::Offline,
        }
    }

    /// Whether something must look again soon: a repeat is waiting on its pause,
    /// or a good answer is waiting to be refreshed.
    pub(in crate::app) fn needs_a_later_look(&self) -> bool {
        self.entry.as_ref().is_some_and(|e| !e.gone && !e.asking)
    }

    /// Once a frame: a box nothing has asked for in the pass before this one or
    /// the one before that is closed, and what it held goes with it.
    pub(in crate::app) fn release(&mut self, pass: u64) {
        if self.entry.as_ref().is_some_and(|e| thumb_unwanted(Some(e.wanted), pass)) {
            self.entry = None;
        }
    }

    /// Forget everything: the relay was switched off or there is no station.
    pub(in crate::app) fn clear(&mut self) {
        self.entry = None;
    }
}

#[cfg(test)]
mod weather_tests {
    use sdroxide_aircraft_online::{WEATHER_VERSION, Weather};

    use super::*;

    fn w() -> Weather {
        Weather { v: WEATHER_VERSION, utc_offset_s: 3600, temp_c: Some(11.6) }
    }

    #[test]
    fn the_station_is_asked_for_the_airports_place_and_only_that() {
        assert_eq!(
            airport_weather_url("ws://10.0.0.2:8080/ws", 52.308_613, 4.763_889).as_deref(),
            Some("http://10.0.0.2:8080/airport-weather?lat=52.31&lon=4.76")
        );
        assert_eq!(
            airport_weather_url("ws://10.0.0.2:8080/ws", -33.9461, -118.408).as_deref(),
            Some("http://10.0.0.2:8080/airport-weather?lat=-33.95&lon=-118.41")
        );
        assert_eq!(airport_weather_url("ws://10.0.0.2:8080/ws", 91.0, 0.0), None);
        assert_eq!(airport_weather_url("ws://10.0.0.2:8080/ws", f64::NAN, 0.0), None);
    }

    #[test]
    fn an_answer_is_read_by_its_status_and_its_shape() {
        let body = serde_json::to_string(&w()).unwrap();
        assert_eq!(classify_weather(Some(200), &body), Reading::Got(w()));
        for status in [Some(429), Some(502), None] {
            assert_eq!(classify_weather(status, ""), Reading::Retry, "{status:?}");
        }
        for status in [Some(401), Some(503), Some(400), Some(404)] {
            assert_eq!(classify_weather(status, ""), Reading::Gone, "{status:?}");
        }
        assert_eq!(classify_weather(Some(200), "<html>"), Reading::Gone);
        let future = body.replace(&format!("\"v\":{WEATHER_VERSION}"), "\"v\":99");
        assert_eq!(classify_weather(Some(200), &future), Reading::Gone, "a shape it does not know");
    }

    #[test]
    fn an_open_box_asks_once_and_shows_waiting_then_the_answer() {
        let mut s = WeatherStore::default();
        assert_eq!(s.clock(7), Clock::Offline, "nothing asked yet");
        assert!(s.step(7, 10, 0.0), "the first frame asks");
        assert!(!s.step(7, 11, 0.1), "the next frames wait for the answer");
        assert_eq!(s.clock(7), Clock::Asking);
        s.deliver(7, Reading::Got(w()), 0.5);
        assert_eq!(s.clock(7), Clock::Known(w()));
        assert!(!s.step(7, 12, 1.0), "a good answer is not asked for again at once");
    }

    #[test]
    fn a_failed_question_is_repeated_a_few_times_then_the_box_says_offline() {
        let mut s = WeatherStore::default();
        assert!(s.step(1, 1, 0.0));
        s.deliver(1, Reading::Retry, 0.2);
        assert_eq!(s.clock(1), Clock::Asking, "tries remain: the room is kept");
        assert!(!s.step(1, 2, 1.0), "not before the pause");
        assert!(s.step(1, 3, THUMB_RETRY_S + 1.0));
        s.deliver(1, Reading::Retry, 5.5);
        assert!(s.step(1, 4, 12.0));
        s.deliver(1, Reading::Retry, 12.1);
        assert_eq!(s.clock(1), Clock::Offline, "three tries spent");
        assert!(!s.step(1, 5, 100.0), "and no more");
    }

    #[test]
    fn a_refusal_is_not_asked_again() {
        let mut s = WeatherStore::default();
        assert!(s.step(1, 1, 0.0));
        s.deliver(1, Reading::Gone, 0.1);
        assert_eq!(s.clock(1), Clock::Offline);
        assert!(!s.step(1, 2, 60.0));
        assert!(!s.needs_a_later_look());
    }

    #[test]
    fn a_good_answer_is_refreshed_after_ten_minutes_and_kept_if_the_refresh_fails() {
        let mut s = WeatherStore::default();
        s.step(1, 1, 0.0);
        s.deliver(1, Reading::Got(w()), 0.5);
        assert!(!s.step(1, 2, WEATHER_REFRESH_S - 1.0));
        assert!(s.step(1, 3, WEATHER_REFRESH_S + 1.0));
        assert_eq!(s.clock(1), Clock::Known(w()), "the old one shows meanwhile");
        s.deliver(1, Reading::Retry, WEATHER_REFRESH_S + 2.0);
        assert_eq!(s.clock(1), Clock::Known(w()), "and after a failure too");
    }

    #[test]
    fn another_airport_replaces_the_first_and_a_late_answer_for_the_first_is_dropped() {
        let mut s = WeatherStore::default();
        s.step(1, 1, 0.0);
        assert!(s.step(2, 2, 1.0), "the box moved to another airport");
        s.deliver(1, Reading::Got(w()), 2.0);
        assert_eq!(s.clock(1), Clock::Offline);
        assert_eq!(s.clock(2), Clock::Asking, "the late answer did not become B's");
    }

    #[test]
    fn closing_the_box_lets_the_weather_go() {
        let mut s = WeatherStore::default();
        s.step(1, 5, 0.0);
        s.deliver(1, Reading::Got(w()), 0.5);
        s.release(6);
        assert_eq!(s.clock(1), Clock::Known(w()), "still asked for in the pass before");
        s.release(7);
        assert_eq!(s.clock(1), Clock::Offline, "two passes without a question: gone");
    }
}

/// What the station answered to a request to switch the relay.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) enum SwitchAnswer {
    /// The relay is on now.
    On,
    /// The relay is off now.
    Off,
    /// It did not change, and the sentence says why.
    Refused(String),
}

/// Read the reply to a switch request.
pub(in crate::app) fn classify_switch(http_status: Option<u16>, body: &str) -> SwitchAnswer {
    match http_status {
        None => SwitchAnswer::Refused(tr!("aircraft-switch-no-answer")),
        Some(401) => SwitchAnswer::Refused(tr!("aircraft-switch-not-signed-in")),
        Some(403) => SwitchAnswer::Refused(tr!("aircraft-switch-refused")),
        Some(200) => {
            #[derive(serde::Deserialize)]
            struct Reply {
                enabled: bool,
            }
            match serde_json::from_str::<Reply>(body) {
                Ok(r) if r.enabled => SwitchAnswer::On,
                Ok(_) => SwitchAnswer::Off,
                Err(_) => SwitchAnswer::Refused(tr!("aircraft-switch-too-old")),
            }
        }
        Some(404) => SwitchAnswer::Refused(tr!("aircraft-switch-too-old")),
        Some(_) => SwitchAnswer::Refused(tr!("aircraft-switch-failed")),
    }
}

/// What to say when the station did not do what the switch asked: it answered
/// with the other state, so something holds the relay there (the station's
/// environment variable). `None` when it did as asked or refused outright.
pub(in crate::app) fn held_note(asked: bool, answer: &SwitchAnswer) -> Option<String> {
    match (asked, answer) {
        (true, SwitchAnswer::Off) | (false, SwitchAnswer::On) => Some(tr!("aircraft-switch-held")),
        _ => None,
    }
}

struct Entry {
    state: OnlineState,
    /// When a failure may be asked again; `None` is not this session.
    retry_at: Option<i64>,
    /// A counter, not a time: the order cards were last looked at in.
    touched: u64,
}

static IDLE: OnlineState = OnlineState::Idle;
static NO_STATION: OnlineState = OnlineState::Unavailable(Reason::NoStation);

/// The online state of every card looked at this session. Memory only.
#[derive(Default)]
pub(in crate::app) struct OnlineStore {
    entries: HashMap<u32, Entry>,
    /// Set once the station says it will not answer anybody (relay off, sign-in
    /// missing): after that no card is asked about.
    blocked: Option<OnlineState>,
    /// The questions out, and when each went; at most [`MAX_IN_FLIGHT`].
    in_flight: Vec<(u32, i64)>,
    clock: u64,
}

impl OnlineStore {
    /// Forget that the station said no: the relay has just been switched on,
    /// so every card may ask again. Cards that did get an answer keep it.
    pub(in crate::app) fn unblock(&mut self) {
        self.blocked = None;
        self.in_flight.clear();
        self.entries.retain(|_, e| matches!(e.state, OnlineState::Ready(_)));
    }

    #[cfg(test)]
    pub(in crate::app) fn len(&self) -> usize {
        self.entries.len()
    }

    /// The station said its relay is off, for every card.
    pub(in crate::app) fn relay_off(&self) -> bool {
        matches!(self.blocked, Some(OnlineState::Unavailable(Reason::Disabled)))
    }

    /// Every card the relay has answered for this session, with its address.
    pub(in crate::app) fn infos(&self) -> impl Iterator<Item = (u32, &Info)> {
        self.entries.iter().filter_map(|(icao, e)| match &e.state {
            OnlineState::Ready(info) => Some((*icao, &**info)),
            _ => None,
        })
    }

    /// What to draw for this card.
    pub(in crate::app) fn state_for(&self, icao: u32, has_station: bool) -> &OnlineState {
        if let Some(Entry { state: s @ OnlineState::Ready(_), .. }) = self.entries.get(&icao) {
            return s;
        }
        if !has_station {
            return &NO_STATION;
        }
        if let Some(b) = &self.blocked {
            return b;
        }
        self.entries.get(&icao).map_or(&IDLE, |e| &e.state)
    }

    /// Whether every place for a question is taken by one still being waited
    /// for. Only then can no other aircraft be asked about.
    pub(in crate::app) fn line_busy(&self, now: i64) -> bool {
        self.in_flight.iter().filter(|(_, at)| now - at < FLIGHT_TIMEOUT_S).count() >= MAX_IN_FLIGHT
    }

    /// Seconds before a failed question for `icao` may be asked again, `0` when
    /// it is due. `None` when it did not fail, or will not be retried.
    pub(in crate::app) fn retry_in(&self, icao: u32, now: i64) -> Option<i64> {
        let e = self.entries.get(&icao)?;
        match (&e.state, e.retry_at) {
            (OnlineState::Unavailable(_), Some(t)) => Some((t - now).max(0)),
            _ => None,
        }
    }

    /// A press on the lookup button: whatever back-off a failure left for
    /// `icao` is over, so the next [`Self::next_request`] sends it. One press,
    /// one question; the line's own ceiling (three out at once) still holds.
    pub(in crate::app) fn retry_now(&mut self, icao: u32) {
        if let Some(e) = self.entries.get_mut(&icao)
            && matches!(e.state, OnlineState::Unavailable(_))
            && e.retry_at.is_some()
        {
            e.retry_at = Some(0);
        }
    }

    /// The question to send now, if any. Called every frame the card is open:
    /// it answers `Some` once, and then only when a back-off has run out.
    pub(in crate::app) fn next_request(
        &mut self,
        want: Option<&Want>,
        has_station: bool,
        now: i64,
    ) -> Option<Want> {
        let want = want?;
        if !has_station || self.blocked.is_some() {
            return None;
        }
        // Whatever never came back is given up on and says so: its place is
        // free and the card waits out a short back-off.
        let lost: Vec<u32> = self
            .in_flight
            .iter()
            .filter(|(_, at)| now - at >= FLIGHT_TIMEOUT_S)
            .map(|(i, _)| *i)
            .collect();
        self.in_flight.retain(|(_, at)| now - at < FLIGHT_TIMEOUT_S);
        for icao in lost {
            self.settle(
                icao,
                OnlineState::Unavailable(Reason::Offline),
                Some(now + RETRY_OFFLINE_S),
            );
        }
        if self.in_flight.iter().any(|(i, _)| *i == want.icao)
            || self.in_flight.len() >= MAX_IN_FLIGHT
        {
            return None;
        }
        self.clock += 1;
        if let Some(e) = self.entries.get_mut(&want.icao) {
            e.touched = self.clock;
            let due = match (&e.state, e.retry_at) {
                (OnlineState::Unavailable(_), Some(t)) => now >= t,
                _ => false,
            };
            if !due {
                return None;
            }
        }
        self.settle(want.icao, OnlineState::Loading, None);
        self.in_flight.push((want.icao, now));
        Some(want.clone())
    }

    /// Take an answer to the question for `icao`.
    pub(in crate::app) fn deliver(&mut self, icao: u32, answer: Answer, now: i64) {
        self.in_flight.retain(|(i, _)| *i != icao);
        match answer {
            Answer::Info(info) => self.settle(icao, OnlineState::Ready(info), None),
            Answer::Unavailable(r) => {
                if r.is_station_wide() {
                    self.blocked = Some(OnlineState::Unavailable(r));
                }
                self.settle(icao, OnlineState::Unavailable(r), r.retry_after().map(|s| now + s));
            }
        }
    }

    fn settle(&mut self, icao: u32, state: OnlineState, retry_at: Option<i64>) {
        self.clock += 1;
        self.entries.insert(icao, Entry { state, retry_at, touched: self.clock });
        while self.entries.len() > MAX_KEPT {
            let oldest = self
                .entries
                .iter()
                .filter(|(i, _)| !self.in_flight.iter().any(|(f, _)| f == *i) && **i != icao)
                .min_by_key(|(_, e)| e.touched)
                .map(|(i, _)| *i);
            match oldest {
                Some(i) => self.entries.remove(&i),
                None => break,
            };
        }
    }
}

/// Which provenance a line wears.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::app) enum Badge {
    /// Heard on the air: green, with its source and age beside it.
    Radio,
    /// From the registry or the photo service, through the station: blue.
    Online,
    /// From the station's own record of the last day: violet.
    History,
    /// Nobody has it: grey, with the reason.
    Missing,
}

/// One line of the card.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) struct Row {
    pub label: &'static str,
    pub value: String,
    pub badge: Badge,
    pub note: String,
}

fn online_row(label: &'static str, value: String) -> Row {
    Row { label, value, badge: Badge::Online, note: tr!("aircraft-online-source") }
}

fn airport(a: &sdroxide_aircraft_online::Airport) -> String {
    match (a.code.is_empty(), a.name.is_empty()) {
        (false, false) => format!("{} — {}", a.code, a.name),
        (false, true) => a.code.clone(),
        _ => a.name.clone(),
    }
}

/// Every line of the open card: what the radio heard, in its own order and
/// untouched, then what the station's relay adds.
pub(in crate::app) fn rows(card: &AircraftCard, state: &OnlineState, now: i64) -> Vec<Row> {
    let mut out: Vec<Row> = aircraft_card::lines(card, now)
        .into_iter()
        .map(|l| Row {
            label: l.label,
            value: l.value,
            badge: if l.present { Badge::Radio } else { Badge::Missing },
            note: l.note,
        })
        .collect();
    match state {
        OnlineState::Ready(info) => {
            if let Some(a) = &info.aircraft {
                if let Some(reg) = &a.registration {
                    if let Some(r) = out.iter_mut().find(|r| r.label == "Registration") {
                        if r.badge == Badge::Missing {
                            *r = online_row("Registration", reg.clone());
                        }
                    }
                }
                let kind: Vec<&str> = [a.manufacturer.as_deref(), a.type_name.as_deref()]
                    .into_iter()
                    .flatten()
                    .collect();
                if !kind.is_empty() {
                    out.push(online_row("Type", kind.join(" ")));
                }
                if let Some(op) = &a.operator {
                    out.push(online_row("Operator", op.clone()));
                }
            }
            if let Some(route) = &info.route {
                if let Some(o) = &route.origin {
                    out.push(online_row("Origin", airport(o)));
                }
                if let Some(d) = &route.destination {
                    out.push(online_row("Destination", airport(d)));
                }
            }
        }
        OnlineState::Loading => out.push(Row {
            label: "Online",
            value: "…".to_string(),
            badge: Badge::Missing,
            note: tr!("aircraft-asking-station"),
        }),
        OnlineState::Unavailable(r) => {
            if let Some(note) = r.note() {
                out.push(Row {
                    label: "Online",
                    value: "—".to_string(),
                    badge: Badge::Missing,
                    note,
                });
            }
        }
        OnlineState::Idle => {}
    }
    out
}

/// Where a card's data comes from, as the badge at its top says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::app) enum Provenance {
    Radio,
    RadioOnline,
    Online,
    Nothing,
}

impl Provenance {
    pub(in crate::app) fn label(self) -> String {
        match self {
            Provenance::Radio => tr!("aircraft-provenance-radio"),
            Provenance::RadioOnline => tr!("aircraft-provenance-radio-online"),
            Provenance::Online => tr!("aircraft-provenance-online"),
            Provenance::Nothing => String::new(),
        }
    }
}

fn has_radio_data(card: &AircraftCard) -> bool {
    aircraft_card::lines(card, 0).iter().any(|l| l.present)
}

fn has_online_data(state: &OnlineState) -> bool {
    let OnlineState::Ready(info) = state else { return false };
    let a = info.aircraft.as_ref().is_some_and(|a| {
        a.type_name.is_some()
            || a.manufacturer.is_some()
            || a.operator.is_some()
            || a.registration.is_some()
    });
    let r = info.route.as_ref().is_some_and(|r| r.origin.is_some() || r.destination.is_some());
    a || r || info.photo.is_some()
}

pub(in crate::app) fn provenance(card: &AircraftCard, state: &OnlineState) -> Provenance {
    match (has_radio_data(card), has_online_data(state)) {
        (true, true) => Provenance::RadioOnline,
        (true, false) => Provenance::Radio,
        (false, true) => Provenance::Online,
        (false, false) => Provenance::Nothing,
    }
}

/// What loading the thumbnail has come to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::app) enum ThumbStatus {
    NotTried,
    Loading,
    Ready,
    Failed,
}

/// The photo block: the credit and the link always, the picture when it can be.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) struct PhotoView {
    /// Planespotters' address for the thumbnail, exactly as the relay gave it.
    pub thumb_url: String,
    pub show_image: bool,
    pub credit: String,
    /// The photo's page on Planespotters, when it is one.
    pub page_url: Option<String>,
}

fn host_of(url: &str) -> Option<&str> {
    let rest = url.strip_prefix("https://")?;
    let host = rest.split(['/', '?', '#']).next()?;
    (!host.is_empty() && !host.contains('@') && !host.contains(':')).then_some(host)
}

/// Whether this is a picture address Planespotters hands out. The relay checks
/// it too; a client does not trust a station to have done so before it opens a
/// connection to whatever the reply names.
pub(in crate::app) fn thumb_fetchable(url: &str) -> bool {
    host_of(url) == Some("t.plnspttrs.net")
}

fn page_ok(url: &str) -> bool {
    matches!(host_of(url), Some("www.planespotters.net" | "planespotters.net"))
}

pub(in crate::app) fn photo_view(state: &OnlineState, thumb: ThumbStatus) -> Option<PhotoView> {
    let OnlineState::Ready(info) = state else { return None };
    let Photo { thumb_url, page_url, photographer } = info.photo.as_ref()?;
    Some(PhotoView {
        thumb_url: thumb_url.clone(),
        show_image: thumb != ThumbStatus::Failed && thumb_fetchable(thumb_url),
        credit: tr!("aircraft-photo-credit", photographer = photographer),
        page_url: page_ok(page_url).then(|| page_url.clone()),
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn an_airport_photo_is_asked_by_language_and_title_never_by_address() {
        assert_eq!(
            super::airport_photo_url(
                "wss://shack.example/sdr/ws/2",
                "en",
                "Amsterdam_Airport_Schiphol"
            )
            .as_deref(),
            Some(
                "https://shack.example/sdr/airport-photo?lang=en&title=Amsterdam_Airport_Schiphol"
            )
        );
        assert_eq!(
            super::airport_photo_url("ws://shack.test:4950/ws", "es", "Aeropuerto_de_Málaga_(x&y)")
                .as_deref(),
            Some(
                "http://shack.test:4950/airport-photo?lang=es&title=Aeropuerto_de_M%C3%A1laga_%28x%26y%29"
            ),
            "nothing in a title can end the query or start another"
        );
        assert_eq!(
            super::airport_page_url("en", "Amsterdam_Airport_Schiphol"),
            "https://en.wikipedia.org/wiki/Amsterdam_Airport_Schiphol"
        );
    }

    #[test]
    fn the_photo_is_asked_by_aircraft_and_released_by_one_address() {
        assert_eq!(
            super::photo_url("wss://shack.example/sdr/ws/2", 0x4ca1fa).as_deref(),
            Some("https://shack.example/sdr/aircraft-photo/4CA1FA")
        );
        assert_eq!(
            super::photo_release_url("ws://shack.test:4950/ws", 0x4CA1FA).as_deref(),
            Some("http://shack.test:4950/aircraft-photo/4CA1FA")
        );
        assert_eq!(super::photo_url("not an address", 1), None);
        assert_eq!(super::photo_release_url("not an address", 0x4CA1FA), None);
    }

    #[test]
    fn a_thumbnail_nobody_has_asked_for_lately_is_let_go() {
        use super::thumb_unwanted;
        assert!(!thumb_unwanted(Some(10), 10), "asked this frame");
        assert!(!thumb_unwanted(Some(10), 11), "asked last frame");
        assert!(thumb_unwanted(Some(10), 12), "the card is gone");
        assert!(thumb_unwanted(None, 3), "never asked, nothing to keep");
    }

    #[test]
    fn the_switch_request_names_the_station_and_the_state_wanted() {
        assert_eq!(
            switch_url("wss://shack.example/sdr/ws/2", true).as_deref(),
            Some("https://shack.example/sdr/aircraft-online?on=1")
        );
        assert_eq!(
            switch_url("ws://shack.test:4950/ws", false).as_deref(),
            Some("http://shack.test:4950/aircraft-online?on=0")
        );
        assert_eq!(switch_url("not an address", true), None);
    }

    #[test]
    fn a_reply_to_the_switch_is_read_and_a_refusal_says_why() {
        assert_eq!(
            classify_switch(Some(200), r#"{"enabled":true,"forced":false}"#),
            SwitchAnswer::On
        );
        assert_eq!(
            classify_switch(Some(200), r#"{"enabled":false,"forced":false}"#),
            SwitchAnswer::Off
        );
        for (status, body) in [
            (None, ""),
            (Some(401), ""),
            (Some(403), ""),
            (Some(404), ""),
            (Some(200), "<html>"),
            (Some(500), ""),
        ] {
            assert!(
                matches!(classify_switch(status, body), SwitchAnswer::Refused(_)),
                "{status:?}"
            );
        }
    }

    #[test]
    fn the_relay_is_taken_as_on_until_the_station_says_it_is_off() {
        let mut s = OnlineStore::default();
        assert!(!s.relay_off(), "on by default: nothing has been asked yet");
        s.deliver(1, Answer::Unavailable(Reason::Offline), 1);
        assert!(!s.relay_off(), "a failed read is not a switched-off relay");
        s.deliver(1, Answer::Unavailable(Reason::Disabled), 2);
        assert!(s.relay_off());
        s.unblock();
        assert!(!s.relay_off(), "switching it on clears it");
    }

    #[test]
    fn a_station_that_answers_with_the_other_state_is_said_to_hold_the_relay() {
        assert!(held_note(true, &SwitchAnswer::Off).is_some(), "asked on, still off");
        assert!(held_note(false, &SwitchAnswer::On).is_some(), "asked off, still on");
        assert_eq!(held_note(true, &SwitchAnswer::On), None);
        assert_eq!(held_note(false, &SwitchAnswer::Off), None);
        assert_eq!(held_note(true, &SwitchAnswer::Refused("x".into())), None);
    }

    #[test]
    fn switching_the_relay_on_lets_every_card_ask_again_and_keeps_the_answers_it_has() {
        let mut s = OnlineStore::default();
        let w = |icao| Want { icao, callsign: None, registration: None };
        assert!(s.next_request(Some(&w(1)), true, 0).is_some());
        s.deliver(1, Answer::Unavailable(Reason::Disabled), 1);
        assert_eq!(s.state_for(2, true), &OnlineState::Unavailable(Reason::Disabled));
        assert!(s.next_request(Some(&w(2)), true, 2).is_none(), "blocked while the relay is off");
        s.unblock();
        assert_eq!(s.state_for(2, true), &OnlineState::Idle);
        assert!(s.next_request(Some(&w(2)), true, 3).is_some(), "asks again after the switch");
    }
    use super::super::aircraft_card::lines;
    use super::super::aircraft_registry::{Source, Stamped};
    use super::*;
    use sdroxide_aircraft_online::{AircraftInfo, Airport, FORMAT_VERSION, Photo, Route, Status};

    const NOW: i64 = 1_000_000;

    #[test]
    fn the_line_is_busy_while_a_question_is_out_and_free_after_its_answer_or_timeout() {
        let mut st = OnlineStore::default();
        assert!(!st.line_busy(NOW));
        for i in 0..MAX_IN_FLIGHT as u32 {
            assert!(!st.line_busy(NOW), "a place is free");
            assert!(st.next_request(Some(&want(i)), true, NOW).is_some());
        }
        assert!(st.line_busy(NOW + 1), "every place is taken");
        assert!(!st.line_busy(NOW + FLIGHT_TIMEOUT_S), "a question that never came back");
        st.deliver(0, Answer::Unavailable(Reason::Offline), NOW + 2);
        assert!(!st.line_busy(NOW + 3));
    }

    fn stamped<T>(value: T, source: Source, age: i64) -> Option<Stamped<T>> {
        Some(Stamped { value, source, at: NOW - age })
    }

    fn heard() -> AircraftCard {
        AircraftCard {
            flight: stamped("KLM1234".to_string(), Source::Adsb, 12),
            registration: stamped("PH-BXA".to_string(), Source::Vdl2, 40),
            altitude_ft: stamped(35_000, Source::Adsb, 12),
            last_seen: NOW - 12,
            ..AircraftCard::default()
        }
    }

    fn no_registration() -> AircraftCard {
        AircraftCard { registration: None, ..heard() }
    }

    fn info() -> Info {
        let mut i = Info::bare(Status::Ok);
        i.aircraft = Some(AircraftInfo {
            type_name: Some("737-800".into()),
            manufacturer: Some("Boeing".into()),
            operator: Some("KLM".into()),
            registration: Some("PH-ONLINE".into()),
        });
        i.route = Some(Route {
            origin: Some(Airport { code: "EHAM".into(), name: "Amsterdam Schiphol".into() }),
            destination: Some(Airport { code: "EGLL".into(), name: "London Heathrow".into() }),
        });
        i.photo = Some(Photo {
            thumb_url: "https://t.plnspttrs.net/12345/abc_t.jpg".into(),
            page_url: "https://www.planespotters.net/photo/12345/x".into(),
            photographer: "A. Spotter".into(),
        });
        i
    }

    fn ready(i: Info) -> OnlineState {
        OnlineState::Ready(Box::new(i))
    }

    fn want(icao: u32) -> Want {
        Want { icao, callsign: None, registration: None }
    }

    fn row<'a>(rs: &'a [Row], label: &str) -> &'a Row {
        rs.iter().find(|r| r.label == label).unwrap_or_else(|| panic!("no {label} row"))
    }

    // ── what the card says ────────────────────────────────────────────────

    #[test]
    fn the_radio_lines_come_first_and_in_their_own_order() {
        let card = heard();
        let radio: Vec<&str> = lines(&card, NOW).iter().map(|l| l.label).collect();
        let all = rows(&card, &ready(info()), NOW);
        let head: Vec<&str> = all.iter().take(radio.len()).map(|r| r.label).collect();
        assert_eq!(head, radio);
        assert!(all.len() > radio.len(), "the online lines follow");
        assert!(all[radio.len()..].iter().all(|r| r.badge == Badge::Online));
    }

    #[test]
    fn what_the_radio_heard_wears_the_radio_badge_with_its_source_and_age() {
        let all = rows(&heard(), &ready(info()), NOW);
        let f = row(&all, "Flight");
        assert_eq!(f.badge, Badge::Radio);
        assert_eq!(f.value, "KLM1234");
        assert_eq!(f.note, "ADS-B, 12s ago");
        assert_eq!(row(&all, "Altitude").note, "ADS-B, 12s ago");
        // Not heard: grey, "—", and the reason stays.
        let p = row(&all, "Position");
        assert_eq!((p.badge, p.value.as_str()), (Badge::Missing, "—"));
        assert!(p.note.contains("not heard"), "{}", p.note);
    }

    #[test]
    fn the_registration_the_radio_heard_wins_over_the_online_one() {
        let all = rows(&heard(), &ready(info()), NOW);
        let r = row(&all, "Registration");
        assert_eq!((r.value.as_str(), r.badge), ("PH-BXA", Badge::Radio));
        assert_eq!(all.iter().filter(|r| r.label == "Registration").count(), 1);
    }

    #[test]
    fn an_online_registration_fills_the_gap_the_radio_left() {
        let all = rows(&no_registration(), &ready(info()), NOW);
        let r = row(&all, "Registration");
        assert_eq!((r.value.as_str(), r.badge), ("PH-ONLINE", Badge::Online));
        assert_eq!(all.iter().filter(|r| r.label == "Registration").count(), 1);
    }

    #[test]
    fn a_card_with_no_online_data_reads_exactly_as_the_radio_card_does() {
        for st in [OnlineState::Idle, OnlineState::Unavailable(Reason::Unknown)] {
            let all = rows(&heard(), &st, NOW);
            let radio = lines(&heard(), NOW);
            assert_eq!(all.len(), radio.len(), "{st:?}");
            assert!(all.iter().all(|r| r.badge != Badge::Online), "{st:?}");
            for (r, l) in all.iter().zip(&radio) {
                assert_eq!((r.label, &r.value, &r.note), (l.label, &l.value, &l.note));
            }
        }
    }

    #[test]
    fn online_data_with_nothing_heard_leaves_the_radio_lines_grey() {
        let all = rows(&AircraftCard::default(), &ready(info()), NOW);
        assert_eq!(row(&all, "Flight").badge, Badge::Missing);
        assert_eq!(row(&all, "Type").badge, Badge::Online);
    }

    #[test]
    fn an_online_field_that_is_absent_adds_no_line() {
        let mut i = Info::bare(Status::Ok);
        i.aircraft = Some(AircraftInfo { type_name: Some("A320".into()), ..Default::default() });
        let all = rows(&heard(), &ready(i), NOW);
        assert_eq!(row(&all, "Type").value, "A320");
        for gone in ["Operator", "Origin", "Destination"] {
            assert!(all.iter().all(|r| r.label != gone), "{gone}");
        }
    }

    #[test]
    fn the_online_lines_are_worded_for_a_reader() {
        let all = rows(&heard(), &ready(info()), NOW);
        assert_eq!(row(&all, "Type").value, "Boeing 737-800");
        assert_eq!(row(&all, "Operator").value, "KLM");
        assert_eq!(row(&all, "Origin").value, "EHAM — Amsterdam Schiphol");
        assert_eq!(row(&all, "Destination").value, "EGLL — London Heathrow");
        assert_eq!(row(&all, "Operator").note, "adsbdb, through the station");
    }

    #[test]
    fn the_badge_at_the_top_says_radio_or_radio_plus_online() {
        let c = heard();
        assert_eq!(provenance(&c, &OnlineState::Idle), Provenance::Radio);
        assert_eq!(provenance(&c, &OnlineState::Loading), Provenance::Radio);
        assert_eq!(provenance(&c, &OnlineState::Unavailable(Reason::Disabled)), Provenance::Radio);
        assert_eq!(provenance(&c, &ready(Info::bare(Status::Ok))), Provenance::Radio);
        assert_eq!(provenance(&c, &ready(info())), Provenance::RadioOnline);
        assert_eq!(provenance(&AircraftCard::default(), &ready(info())), Provenance::Online);
        assert_eq!(provenance(&AircraftCard::default(), &OnlineState::Idle), Provenance::Nothing);
        assert_eq!(Provenance::Radio.label(), "radio");
        assert_eq!(Provenance::RadioOnline.label(), "radio + online");
        assert_eq!(Provenance::Online.label(), "online");
        assert_eq!(Provenance::Nothing.label(), "");
    }

    // ── what an answer means ──────────────────────────────────────────────

    fn body(status: Status) -> String {
        serde_json::to_string(&Info::bare(status)).unwrap()
    }

    #[test]
    fn every_way_an_answer_can_come_back_is_a_state_and_none_is_an_error() {
        let ok = serde_json::to_string(&info()).unwrap();
        assert_eq!(classify(Some(200), &ok), Answer::Info(Box::new(info())));
        let table: [(Option<u16>, String, Reason); 12] = [
            (Some(503), body(Status::Disabled), Reason::Disabled),
            (Some(429), body(Status::Busy), Reason::Busy),
            (Some(200), body(Status::Unknown), Reason::Unknown),
            (Some(200), body(Status::Unavailable), Reason::Offline),
            (Some(401), body(Status::Unauthorized), Reason::Unauthorized),
            (Some(400), body(Status::Invalid), Reason::Unknown),
            (None, String::new(), Reason::Offline),
            (Some(200), "not json".into(), Reason::Offline),
            (Some(503), "<html>bad gateway</html>".into(), Reason::Offline),
            (Some(401), String::new(), Reason::Unauthorized),
            (Some(429), String::new(), Reason::Busy),
            (Some(404), String::new(), Reason::Offline),
        ];
        for (code, b, want) in table {
            assert_eq!(classify(code, &b), Answer::Unavailable(want), "{code:?} {b}");
        }
    }

    #[test]
    fn a_reply_in_a_shape_this_client_does_not_know_is_a_quiet_failure() {
        let mut i = info();
        i.v = FORMAT_VERSION + 1;
        let b = serde_json::to_string(&i).unwrap();
        assert_eq!(classify(Some(200), &b), Answer::Unavailable(Reason::Offline));
    }

    #[test]
    fn only_a_switched_off_relay_or_a_missing_station_says_so_on_the_card() {
        assert!(Reason::Disabled.note().unwrap().contains("online unavailable"));
        assert!(Reason::NoStation.note().unwrap().contains("online unavailable"));
        assert!(Reason::Unauthorized.note().is_some());
        assert_eq!(Reason::Unknown.note(), None, "the registry just does not know it");
        for failed in [Reason::Offline, Reason::Busy] {
            let n = failed.note().unwrap_or_else(|| panic!("{failed:?} failed in silence"));
            assert!(n.contains("again"), "{n}");
        }
        let all = rows(&heard(), &OnlineState::Unavailable(Reason::Disabled), NOW);
        let r = all.last().unwrap();
        assert_eq!((r.label, r.value.as_str(), r.badge), ("Online", "—", Badge::Missing));
        assert!(r.note.contains("online unavailable"), "{}", r.note);
        let quiet = rows(&heard(), &OnlineState::Unavailable(Reason::Unknown), NOW);
        assert_eq!(quiet.len(), lines(&heard(), NOW).len());
        let failed = rows(&heard(), &OnlineState::Unavailable(Reason::Offline), NOW);
        assert_eq!(failed.last().unwrap().label, "Online");
        assert!(failed.last().unwrap().note.contains("again"));
    }

    #[test]
    fn a_question_in_flight_is_shown_as_such() {
        let all = rows(&heard(), &OnlineState::Loading, NOW);
        let r = all.last().unwrap();
        assert_eq!((r.label, r.badge), ("Online", Badge::Missing));
    }

    // ── what is asked, and when ───────────────────────────────────────────

    #[test]
    fn a_card_is_asked_about_once() {
        let mut s = OnlineStore::default();
        let w = want(0x4CA1FA);
        assert_eq!(s.next_request(Some(&w), true, NOW), Some(w.clone()));
        assert_eq!(s.state_for(w.icao, true), &OnlineState::Loading);
        // Redrawn every frame while the question is out, asked once.
        for t in 0..FLIGHT_TIMEOUT_S {
            assert_eq!(s.next_request(Some(&w), true, NOW + t), None);
        }
        s.deliver(w.icao, Answer::Info(Box::new(info())), NOW + 1);
        assert!(matches!(s.state_for(w.icao, true), OnlineState::Ready(_)));
        assert_eq!(s.next_request(Some(&w), true, NOW + 1_000_000), None, "an answer is kept");
    }

    #[test]
    fn nothing_is_asked_without_an_open_card_an_icao_or_a_station() {
        let mut s = OnlineStore::default();
        assert_eq!(s.next_request(None, true, NOW), None);
        assert_eq!(s.next_request(Some(&want(1)), false, NOW), None);
        assert_eq!(s.state_for(1, false), &OnlineState::Unavailable(Reason::NoStation));
        // The same address, later, with a station: not poisoned by the first look.
        assert!(s.next_request(Some(&want(1)), true, NOW).is_some());
        let fallback =
            CardKey::Fallback(super::super::aircraft_registry::FallbackKey::AircraftId(7));
        assert_eq!(want_for(&fallback, &heard()), None);
    }

    #[test]
    fn a_switched_off_relay_is_not_asked_again_for_any_card_all_session() {
        let mut s = OnlineStore::default();
        assert!(s.next_request(Some(&want(1)), true, NOW).is_some());
        s.deliver(1, Answer::Unavailable(Reason::Disabled), NOW);
        for t in [0, 1, RETRY_S, RETRY_S * 100] {
            assert_eq!(s.next_request(Some(&want(1)), true, NOW + t), None);
            assert_eq!(s.next_request(Some(&want(2)), true, NOW + t), None, "another card too");
        }
        assert_eq!(s.state_for(2, true), &OnlineState::Unavailable(Reason::Disabled));
    }

    #[test]
    fn a_missing_sign_in_is_not_asked_again_either() {
        let mut s = OnlineStore::default();
        assert!(s.next_request(Some(&want(1)), true, NOW).is_some());
        s.deliver(1, Answer::Unavailable(Reason::Unauthorized), NOW);
        assert_eq!(s.next_request(Some(&want(2)), true, NOW + RETRY_S * 10), None);
    }

    #[test]
    fn a_failure_busy_or_unknown_is_asked_again_only_after_the_back_off() {
        for r in [Reason::Offline, Reason::Busy, Reason::Unknown] {
            let mut s = OnlineStore::default();
            let w = want(5);
            assert!(s.next_request(Some(&w), true, NOW).is_some());
            s.deliver(5, Answer::Unavailable(r), NOW);
            assert_eq!(s.next_request(Some(&w), true, NOW), None, "{r:?}");
            let wait = r.retry_after().unwrap();
            assert_eq!(s.next_request(Some(&w), true, NOW + wait - 1), None, "{r:?}");
            assert!(s.next_request(Some(&w), true, NOW + wait).is_some(), "{r:?}");
            // And again: each failure restarts the wait.
            s.deliver(5, Answer::Unavailable(r), NOW + wait);
            assert_eq!(s.next_request(Some(&w), true, NOW + wait + 1), None, "{r:?}");
        }
        assert!(
            Reason::Offline.retry_after().unwrap() <= 60,
            "a failed read mends itself, so it is tried again soon, not in five minutes"
        );
        assert_eq!(Reason::Unknown.retry_after(), Some(RETRY_S));
        assert_eq!(Reason::Disabled.retry_after(), None);
        assert_eq!(Reason::Unauthorized.retry_after(), None);
        assert_eq!(Reason::NoStation.retry_after(), None);
    }

    #[test]
    fn a_second_card_is_asked_while_the_first_is_still_out() {
        let mut s = OnlineStore::default();
        assert!(s.next_request(Some(&want(1)), true, NOW).is_some());
        assert!(
            s.next_request(Some(&want(2)), true, NOW + 1).is_some(),
            "a card that is slow to answer must not hold every other card"
        );
        assert_eq!(
            s.next_request(Some(&want(1)), true, NOW + 2),
            None,
            "the same card is not asked twice"
        );
    }

    #[test]
    fn only_a_few_questions_are_out_at_once() {
        let mut s = OnlineStore::default();
        for i in 0..MAX_IN_FLIGHT as u32 {
            assert!(s.next_request(Some(&want(i)), true, NOW + i as i64).is_some());
        }
        assert!(s.line_busy(NOW + 5), "every place is taken");
        assert_eq!(s.next_request(Some(&want(99)), true, NOW + 5), None);
        s.deliver(0, Answer::Unavailable(Reason::Offline), NOW + 6);
        assert!(!s.line_busy(NOW + 6), "an answer freed a place");
        assert!(s.next_request(Some(&want(99)), true, NOW + 6).is_some());
    }

    #[test]
    fn a_card_that_fails_never_stops_another_from_being_read() {
        let mut s = OnlineStore::default();
        assert!(s.next_request(Some(&want(1)), true, NOW).is_some());
        // The first card fails after a long wait; the operator opens another.
        let t = NOW + FLIGHT_TIMEOUT_S - 1;
        assert!(s.next_request(Some(&want(2)), true, t).is_some(), "card 2 asked during the wait");
        s.deliver(1, Answer::Unavailable(Reason::Offline), t + 1);
        s.deliver(2, Answer::Info(Box::new(info())), t + 2);
        assert!(matches!(s.state_for(2, true), OnlineState::Ready(_)));
        assert!(matches!(s.state_for(1, true), OnlineState::Unavailable(Reason::Offline)));
    }

    #[test]
    fn a_question_that_never_comes_back_is_given_up_quickly_and_frees_its_place() {
        assert!(FLIGHT_TIMEOUT_S <= 15, "a browser fetch has no timeout of its own");
        let mut s = OnlineStore::default();
        assert!(s.next_request(Some(&want(1)), true, NOW).is_some());
        // The lost one is settled with a reason when the time is up.
        assert!(s.next_request(Some(&want(2)), true, NOW + FLIGHT_TIMEOUT_S).is_some());
        assert!(!s.line_busy(NOW + FLIGHT_TIMEOUT_S));
        assert!(matches!(s.state_for(1, true), OnlineState::Unavailable(Reason::Offline)));
        // The lost one is not asked again at once either.
        assert_eq!(s.next_request(Some(&want(1)), true, NOW + FLIGHT_TIMEOUT_S + 1), None);
    }

    #[test]
    fn what_is_kept_is_bounded() {
        let mut s = OnlineStore::default();
        for i in 0..(MAX_KEPT as u32 + 20) {
            let t = NOW + i as i64;
            assert!(s.next_request(Some(&want(i)), true, t).is_some());
            s.deliver(i, Answer::Info(Box::new(info())), t);
        }
        assert!(s.len() <= MAX_KEPT);
        assert!(matches!(s.state_for(MAX_KEPT as u32 + 19, true), OnlineState::Ready(_)));
    }

    #[test]
    fn the_question_carries_only_a_clean_callsign_and_registration() {
        let k = CardKey::Icao(0x4CA1FA);
        let w = want_for(&k, &heard()).unwrap();
        assert_eq!(
            w,
            Want {
                icao: 0x4CA1FA,
                callsign: Some("KLM1234".into()),
                registration: Some("PH-BXA".into())
            }
        );
        let mut dirty = heard();
        dirty.flight = stamped("KLM 12&x=1".to_string(), Source::Adsb, 1);
        dirty.registration = stamped("../etc".to_string(), Source::Vdl2, 1);
        let w = want_for(&k, &dirty).unwrap();
        assert_eq!((w.callsign, w.registration), (None, None));
    }

    // ── where it is asked ─────────────────────────────────────────────────

    #[test]
    fn the_question_goes_to_the_station_the_client_is_connected_to() {
        let w = Want {
            icao: 0x4CA1FA,
            callsign: Some("KLM1234".into()),
            registration: Some("PH-BXA".into()),
        };
        assert_eq!(
            info_url("ws://shack.test:4950/ws", &w).as_deref(),
            Some("http://shack.test:4950/aircraft-info/4CA1FA?callsign=KLM1234&reg=PH-BXA")
        );
        // Behind a reverse proxy with a prefix, and on a radio other than the first.
        assert_eq!(
            info_url("wss://host.test/shack/ws/2", &want(0xAB)).as_deref(),
            Some("https://host.test/shack/aircraft-info/0000AB")
        );
        assert_eq!(info_url("https://host.test", &want(1)), None, "not a socket address");
        assert_eq!(info_url("", &want(1)), None);
    }

    // ── the photo ─────────────────────────────────────────────────────────

    #[test]
    fn a_photo_has_its_credit_and_its_page_and_its_address_untouched() {
        let v = photo_view(&ready(info()), ThumbStatus::NotTried).unwrap();
        assert_eq!(v.thumb_url, "https://t.plnspttrs.net/12345/abc_t.jpg");
        assert!(v.show_image);
        assert!(
            v.credit.contains("A. Spotter") && v.credit.contains("Planespotters"),
            "{}",
            v.credit
        );
        assert_eq!(v.page_url.as_deref(), Some("https://www.planespotters.net/photo/12345/x"));
        assert!(photo_view(&ready(Info::bare(Status::Ok)), ThumbStatus::NotTried).is_none());
        assert!(photo_view(&OnlineState::Loading, ThumbStatus::NotTried).is_none());
    }

    #[test]
    fn a_picture_that_cannot_be_shown_leaves_the_credit_and_the_link() {
        let v = photo_view(&ready(info()), ThumbStatus::Failed).unwrap();
        assert!(!v.show_image);
        assert!(v.credit.contains("A. Spotter"));
        assert!(v.page_url.is_some());
        for ok in [ThumbStatus::NotTried, ThumbStatus::Loading, ThumbStatus::Ready] {
            assert!(photo_view(&ready(info()), ok).unwrap().show_image, "{ok:?}");
        }
    }

    #[test]
    fn only_planespotters_own_addresses_are_loaded_or_opened() {
        for bad in [
            "",
            "http://t.plnspttrs.net/a.jpg",
            "https://evil.example/a.jpg",
            "https://t.plnspttrs.net.evil.example/a.jpg",
            "https://user@t.plnspttrs.net/a.jpg",
            "https://t.plnspttrs.net:8443/a.jpg",
        ] {
            assert!(!thumb_fetchable(bad), "{bad}");
            let mut i = info();
            i.photo.as_mut().unwrap().thumb_url = bad.to_string();
            let v = photo_view(&ready(i), ThumbStatus::NotTried).unwrap();
            assert!(!v.show_image, "{bad}");
            assert!(v.credit.contains("A. Spotter"), "the credit stays");
        }
        assert!(thumb_fetchable("https://t.plnspttrs.net/12345/abc_t.jpg"));
        let mut i = info();
        i.photo.as_mut().unwrap().page_url = "https://evil.example/photo".into();
        assert_eq!(photo_view(&ready(i), ThumbStatus::Ready).unwrap().page_url, None);
    }

    // ── what this client must not do ──────────────────────────────────────

    /// The files that make up this feature on the client.
    fn sources() -> [(&'static str, &'static str); 3] {
        [
            ("aircraft_online.rs", include_str!("aircraft_online.rs")),
            ("aircraft_online_io.rs", include_str!("aircraft_online_io.rs")),
            ("aircraft.rs", include_str!("aircraft.rs")),
        ]
    }

    /// The code of a file: its comments and its test module are not what ships.
    fn shipped(src: &str) -> String {
        let code = src.split("#[cfg(test)]").next().unwrap_or(src);
        code.lines().filter(|l| !l.trim_start().starts_with("//")).collect::<Vec<_>>().join("\n")
    }

    #[test]
    fn nothing_is_kept_in_the_browser_or_in_the_settings() {
        let banned = [
            concat!("local", "Storage"),
            concat!("session", "Storage"),
            concat!("Indexed", "DB"),
            concat!("indexed", "_db"),
            concat!("local", "_storage"),
            concat!("session", "_storage"),
            concat!("web_sys::", "Storage"),
            concat!("set_", "string("),
            concat!("set_", "value("),
            concat!("Cache", "Storage"),
        ];
        for (name, src) in sources() {
            let code = shipped(src);
            for b in banned {
                assert!(!code.contains(b), "{name} touches {b}");
            }
        }
    }

    /// The settings structure itself carries no field for it. The test that
    /// matters is the one in `app/mod.rs`, which saves a whole app holding an
    /// online answer and reads every stored value back.
    #[test]
    fn the_settings_structure_has_no_field_for_the_online_state() {
        let saved = ron::to_string(&sdroxide_types::UiSettings::default()).unwrap();
        for word in ["online", "Online", "planespotters", "adsbdb", "thumb"] {
            assert!(!saved.contains(word), "UiSettings mentions {word}");
        }
    }

    #[test]
    fn no_client_asks_the_registry_or_the_photo_service_and_none_carries_a_key() {
        let banned = [
            concat!("api.", "adsbdb"),
            concat!("api.", "planespotters"),
            concat!("planespotters.net/", "pub"),
            concat!("api_", "key"),
            concat!("api", "key"),
            concat!("x-api-", "key"),
            concat!("Bear", "er "),
            concat!("Request", "::Adsbdb"),
            concat!("Request", "::Photo"),
            concat!("features = [\"", "fetch"),
        ];
        for (name, src) in sources() {
            let code = shipped(src);
            for b in banned {
                assert!(!code.contains(b), "{name} contains {b}");
            }
        }
        let manifest = include_str!("../../Cargo.toml");
        assert!(
            !manifest.contains(concat!("sdroxide-aircraft-online\", features")),
            "the client must not link the half that talks to the network"
        );
    }
}
