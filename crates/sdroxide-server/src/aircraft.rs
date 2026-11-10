//! `GET /aircraft-info/{icao}`: what the internet knows about one aircraft,
//! asked once for every client of the station.
//!
//! A client with the AIRCRAFT window open sends the address of the card it has
//! open, and optionally the callsign and registration the radio heard
//! (`?callsign=KLM1234&reg=PH-BXA`). It gets one JSON [`Info`] back: the
//! airframe, the route, and the address, page and credit of a photo. The picture
//! is `GET /aircraft-photo/{icao}`'s: a few thumbnails, in memory, for half a
//! minute or until the card is closed (see [`PhotoStore`]).
//!
//! The decision of what to answer is [`judge`], a function of plain values, so
//! the status codes are tested without a socket. The questions themselves are
//! the [`Relay`]'s: one out per key, a ceiling per minute, 24 h in the cache.
//!
//! Off unless the station's operator switched it on ([`enabled_by`]): it makes
//! this machine reach out to two services on behalf of whoever is connected,
//! and that is the operator's call, not a client's.

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};

use sdroxide_aircraft_online::fetch::HttpUpstream;
use sdroxide_aircraft_online::{
    Article, Image, Info, OnlineCache, Outlook, Relay, Spot, Status, Upstream, Weather,
    clean_callsign, clean_registration,
};

use crate::Station;

/// Questions sent to the two services per minute, whoever is asking. Generous
/// for a person clicking through cards; a station being used to hammer them
/// runs into it.
pub(crate) const MAX_QUESTIONS_PER_MINUTE: usize = 30;

/// The variable that holds the relay on or off whatever the window says.
pub const ENV_SWITCH: &str = "SDROXIDE_AIRCRAFT_ONLINE";

/// What a value of [`ENV_SWITCH`] decides: `Some(true)` holds the relay on,
/// `Some(false)` holds it off, `None` leaves it to the operator's switch in the
/// AIRCRAFT window. Unset or blank is `None`; a plain yes is on; anything else
/// is off, because a typo must not open a way out of the machine.
pub fn forced_by(value: Option<&str>) -> Option<bool> {
    let v = value?.trim().to_ascii_lowercase();
    if v.is_empty() {
        return None;
    }
    Some(matches!(v.as_str(), "1" | "true" | "yes" | "on"))
}

/// Whether the relay answers when the station starts: the environment variable
/// first, then the operator's "off" mark, else on. On is the default so that a
/// card shows its type, route and photo without anyone hunting for a switch.
pub(crate) fn starts_on(forced: Option<bool>, off_mark: bool) -> bool {
    forced.unwrap_or(!off_mark)
}

/// What the route answers to, decided before anything is sent.
#[derive(Debug, PartialEq)]
pub(crate) enum Verdict {
    Reject(Status),
    Run { icao: u32, callsign: Option<String>, registration: Option<String> },
}

/// A query value that is absent or empty is not there; one that is there must
/// be clean, or the whole question is refused rather than quietly changed.
fn clean_param(
    v: Option<&String>,
    clean: fn(&str) -> Option<String>,
) -> Result<Option<String>, ()> {
    match v.map(|s| s.trim()).filter(|s| !s.is_empty()) {
        None => Ok(None),
        Some(s) => clean(s).map(Some).ok_or(()),
    }
}

/// A Mode S address: exactly six hex digits.
fn parse_icao(icao: &str) -> Option<u32> {
    (icao.len() == 6 && icao.bytes().all(|b| b.is_ascii_hexdigit()))
        .then(|| u32::from_str_radix(icao, 16).ok())
        .flatten()
}

/// Sign-in first, then the switch, then the question itself.
pub(crate) fn judge(
    enabled: bool,
    sign_in_needed: bool,
    signed_in: bool,
    icao: &str,
    query: &HashMap<String, String>,
) -> Verdict {
    if sign_in_needed && !signed_in {
        return Verdict::Reject(Status::Unauthorized);
    }
    if !enabled {
        return Verdict::Reject(Status::Disabled);
    }
    let icao = parse_icao(icao);
    let callsign = clean_param(query.get("callsign"), clean_callsign);
    let registration = clean_param(query.get("reg"), clean_registration);
    match (icao, callsign, registration) {
        (Some(icao), Ok(callsign), Ok(registration)) => {
            Verdict::Run { icao, callsign, registration }
        }
        _ => Verdict::Reject(Status::Invalid),
    }
}

pub(crate) fn http_status(s: Status) -> StatusCode {
    match s {
        Status::Ok | Status::Unknown | Status::Unavailable => StatusCode::OK,
        Status::Busy => StatusCode::TOO_MANY_REQUESTS,
        Status::Disabled => StatusCode::SERVICE_UNAVAILABLE,
        Status::Invalid => StatusCode::BAD_REQUEST,
        Status::Unauthorized => StatusCode::UNAUTHORIZED,
    }
}

/// The file whose presence in the config directory means the operator switched
/// the relay off from the AIRCRAFT window. Presence only; there is nothing in it.
/// Without it the relay is on. (Older versions wrote `aircraft-online.on` when
/// it was switched on; that file is no longer read.)
pub(crate) const OFF_FILE: &str = "aircraft-online.off";

/// The header a switch request must carry. A page on another site cannot add a
/// custom header to a request without the station's say-so (a preflight), so
/// this keeps a stray link or form from flipping the relay.
pub(crate) const SWITCH_HEADER: &str = "x-sdroxide";

/// How long the station keeps a thumbnail nobody has let go of: a client that
/// vanished (tab closed, connection cut) cannot release it, so it expires. Long
/// enough that going back to a card just seen costs no download, short enough
/// that nothing lingers.
pub(crate) const PHOTO_HOLD: Duration = Duration::from_secs(30);

/// Thumbnails held at once, one per aircraft; the oldest goes first.
pub(crate) const PHOTO_KEEP: usize = 10;

/// Thumbnails being downloaded at once. A card clicked while another is still
/// loading is served in parallel, not turned away.
pub(crate) const PHOTO_PARALLEL: usize = 3;

/// How long a client that asks for a picture already on its way waits for it.
const PHOTO_WAIT: Duration = Duration::from_secs(10);

/// How long an airport's weather is kept in memory, and how many places at once.
/// Temperature moves slowly; half a minute is what makes ten clients on one
/// airport cost one question.
pub(crate) const WEATHER_HOLD: Duration = Duration::from_secs(30);
const WEATHER_KEEP: usize = 32;

/// The thumbnails' addresses the relay has handed out, newest last. The photo
/// route looks the address up here and never takes one from a client.
const MAX_THUMBS: usize = 64;

struct Held<K> {
    key: K,
    image: Image,
    expires: Instant,
}

/// The few pictures the station holds: in memory only, never written to disk,
/// at most [`PHOTO_KEEP`], each gone [`PHOTO_HOLD`] after it was fetched or as
/// soon as its card is closed. Keyed by aircraft address for the aircraft cards
/// and by article for the airport cards, each in a store of its own.
pub(crate) struct PhotoStore<K = u32> {
    held: Mutex<Vec<Held<K>>>,
}

impl<K> Default for PhotoStore<K> {
    fn default() -> Self {
        PhotoStore { held: Mutex::new(Vec::new()) }
    }
}

impl<K: PartialEq + Clone> PhotoStore<K> {
    fn lock(&self) -> std::sync::MutexGuard<'_, Vec<Held<K>>> {
        self.held.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Hold `image` for `key` from `now`, in place of that key's older one.
    pub(crate) fn put_at(&self, now: Instant, key: K, image: Image) {
        let mut g = self.lock();
        g.retain(|h| h.key != key && h.expires > now);
        g.push(Held { key, image, expires: now + PHOTO_HOLD });
        while g.len() > PHOTO_KEEP {
            g.remove(0);
        }
    }

    pub(crate) fn get_at(&self, now: Instant, key: &K) -> Option<Image> {
        self.lock().iter().find(|h| h.key == *key && h.expires > now).map(|h| h.image.clone())
    }

    pub(crate) fn put(&self, key: K, image: Image) {
        self.put_at(Instant::now(), key, image);
    }

    pub(crate) fn get(&self, key: &K) -> Option<Image> {
        self.get_at(Instant::now(), key)
    }

    /// Drop the picture of one key now: its card was closed.
    pub(crate) fn remove(&self, key: &K) {
        self.lock().retain(|h| h.key != *key);
    }

    /// Drop every picture now.
    pub(crate) fn clear(&self) {
        self.lock().clear();
    }

    /// Drop what has run out of time.
    pub(crate) fn sweep_at(&self, now: Instant) {
        self.lock().retain(|h| h.expires > now);
    }

    /// The keys whose pictures are held, oldest first.
    pub(crate) fn holds(&self) -> Vec<K> {
        self.lock().iter().map(|h| h.key.clone()).collect()
    }
}

/// The pictures being downloaded, by key: at most [`PHOTO_PARALLEL`], and
/// never two for the same one.
struct Fetching<K = u32> {
    set: Mutex<HashSet<K>>,
    done: Condvar,
}

impl<K> Default for Fetching<K> {
    fn default() -> Self {
        Fetching { set: Mutex::new(HashSet::new()), done: Condvar::new() }
    }
}

/// Held while one picture is downloaded. Dropping it, even by a panic, frees the
/// place and wakes whoever waits for that key.
struct Place<'a, K: std::hash::Hash + Eq> {
    fetching: &'a Fetching<K>,
    key: K,
}

impl<K: std::hash::Hash + Eq> Drop for Place<'_, K> {
    fn drop(&mut self) {
        self.fetching.set.lock().unwrap_or_else(|e| e.into_inner()).remove(&self.key);
        self.fetching.done.notify_all();
    }
}

/// The one way a picture is obtained, for aircraft and airports alike: the held
/// one if there is one; else, if `known`, the single download for that key
/// (waited for by whoever asks meanwhile), within the [`PHOTO_PARALLEL`] ceiling.
fn held_or_fetch<K: std::hash::Hash + Eq + Clone>(
    store: &PhotoStore<K>,
    fetching: &Fetching<K>,
    key: K,
    known: bool,
    fetch: impl FnOnce() -> Option<Image>,
) -> Photo {
    let deadline = Instant::now() + PHOTO_WAIT;
    let mut set = fetching.set.lock().unwrap_or_else(|e| e.into_inner());
    loop {
        if let Some(image) = store.get(&key) {
            return Photo::Held { image };
        }
        if !known {
            return Photo::None;
        }
        if !set.contains(&key) {
            if set.len() >= PHOTO_PARALLEL {
                return Photo::Busy;
            }
            set.insert(key.clone());
            break;
        }
        let Some(left) = deadline.checked_duration_since(Instant::now()) else {
            return Photo::Busy;
        };
        set = fetching.done.wait_timeout(set, left).unwrap_or_else(|e| e.into_inner()).0;
    }
    drop(set);
    let _place = Place { fetching, key: key.clone() };
    match fetch() {
        Some(image) => {
            store.put(key, image.clone());
            Photo::Held { image }
        }
        None => Photo::Failed,
    }
}

/// How a request for an airport's weather ended.
#[derive(Debug, PartialEq)]
pub(crate) enum Forecast {
    Held(Weather),
    /// The station's ceiling on questions, or too many places on their way.
    Busy,
    Failed,
}

/// The weather of the places asked about, in memory only and never on disk:
/// at most [`WEATHER_KEEP`], each gone [`WEATHER_HOLD`] after it was got.
#[derive(Default)]
struct WeatherBook {
    held: Mutex<Vec<(String, Instant, Weather)>>,
}

impl WeatherBook {
    fn lock(&self) -> std::sync::MutexGuard<'_, Vec<(String, Instant, Weather)>> {
        self.held.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn get_at(&self, now: Instant, key: &str) -> Option<Weather> {
        self.lock().iter().find(|(k, until, _)| k == key && *until > now).map(|&(.., w)| w)
    }

    fn put_at(&self, now: Instant, key: String, w: Weather) {
        let mut g = self.lock();
        g.retain(|(k, until, _)| *k != key && *until > now);
        g.push((key, now + WEATHER_HOLD, w));
        while g.len() > WEATHER_KEEP {
            g.remove(0);
        }
    }
}

/// How a request for a picture ended.
pub(crate) enum Photo {
    Held {
        image: Image,
    },
    /// The relay has not told this station of a photo for that aircraft.
    None,
    /// Too many pictures are on their way, or the one asked for did not come in time.
    Busy,
    Failed,
}

/// The relay and the file its cache lives in.
pub(crate) struct AircraftHub<U = HttpUpstream> {
    /// Whether the relay answers. Starts from the environment variable or the
    /// off file, and the AIRCRAFT window can change it while running.
    enabled: AtomicBool,
    /// What the environment variable holds the relay at, which no window can
    /// undo; `None` leaves it to the window.
    forced: Option<bool>,
    relay: Relay<U>,
    file: Option<PathBuf>,
    switch_file: Option<PathBuf>,
    thumbs: Mutex<Vec<(u32, String)>>,
    pub(crate) store: PhotoStore,
    fetching: Fetching,
    /// The airport cards' thumbnails, by article.
    pub(crate) airport_store: PhotoStore<String>,
    airport_fetching: Fetching<String>,
    /// The airport cards' local time and temperature, by place.
    weather_book: WeatherBook,
    weather_fetching: Fetching<String>,
}

fn now_unix() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64)
}

impl AircraftHub<HttpUpstream> {
    /// The station's relay. When it is off nothing is read, nothing is
    /// written and no connection is ever made.
    pub(crate) fn start(forced: Option<bool>) -> AircraftHub {
        let dir = sdroxide_config::config_dir().ok();
        let switch_file = dir.as_ref().map(|d| d.join(OFF_FILE));
        let enabled = starts_on(forced, switch_file.as_ref().is_some_and(|f| f.exists()));
        let file = enabled.then(|| dir.as_ref().map(|d| d.join("aircraft-online.json"))).flatten();
        let cache = file
            .as_ref()
            .and_then(|f| std::fs::read_to_string(f).ok())
            .map(|text| OnlineCache::from_json(&text, now_unix()))
            .unwrap_or_default();
        AircraftHub {
            enabled: AtomicBool::new(enabled),
            forced,
            relay: Relay::new(HttpUpstream::new(), MAX_QUESTIONS_PER_MINUTE, cache),
            file,
            switch_file,
            thumbs: Mutex::default(),
            store: PhotoStore::default(),
            fetching: Fetching::default(),
            airport_store: PhotoStore::default(),
            airport_fetching: Fetching::default(),
            weather_book: WeatherBook::default(),
            weather_fetching: Fetching::default(),
        }
    }
}

impl<U: Upstream> AircraftHub<U> {
    #[cfg(test)]
    pub(crate) fn with(enabled: bool, upstream: U, max_per_minute: usize) -> AircraftHub<U> {
        AircraftHub {
            enabled: AtomicBool::new(enabled),
            forced: None,
            relay: Relay::new(upstream, max_per_minute, OnlineCache::default()),
            file: None,
            switch_file: None,
            thumbs: Mutex::default(),
            store: PhotoStore::default(),
            fetching: Fetching::default(),
            airport_store: PhotoStore::default(),
            airport_fetching: Fetching::default(),
            weather_book: WeatherBook::default(),
            weather_fetching: Fetching::default(),
        }
    }

    pub(crate) fn enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }

    /// Switch the relay on or off from a client, and remember an "off" in the
    /// config directory so a restart keeps it. The environment variable wins
    /// over a request either way: it is the operator's, set on the machine.
    /// Returns what the relay is now.
    pub(crate) fn set_enabled(&self, on: bool) -> bool {
        if let Some(held) = self.forced {
            self.enabled.store(held, Ordering::Relaxed);
            return held;
        }
        if let Some(f) = &self.switch_file {
            // A switch that cannot be remembered still works until restart.
            if on {
                let _ = std::fs::remove_file(f);
            } else {
                let _ = std::fs::write(f, b"");
            }
        }
        self.enabled.store(on, Ordering::Relaxed);
        on
    }

    /// Ask, and write the cache out if a minute has passed with something new
    /// in it. Blocking: the route runs it off the async threads.
    pub(crate) fn run(
        &self,
        icao: u32,
        callsign: Option<&str>,
        registration: Option<&str>,
    ) -> Info {
        let now = now_unix();
        let info = self.relay.lookup(now, icao, callsign, registration);
        if let (Some(file), Some(json)) = (&self.file, self.relay.snapshot_if_due(now)) {
            // A cache that cannot be written costs a question tomorrow.
            let _ = std::fs::write(file, json);
        }
        if let Some(p) = &info.photo {
            let mut t = self.thumbs.lock().unwrap_or_else(|e| e.into_inner());
            t.retain(|(i, _)| *i != icao);
            t.push((icao, p.thumb_url.clone()));
            if t.len() > MAX_THUMBS {
                t.remove(0);
            }
        }
        info
    }

    /// The thumbnail of `icao`, held in memory for a short while. Blocking.
    /// Another aircraft's download does not stand in its way; the same
    /// aircraft's is waited for instead of started twice.
    pub(crate) fn photo(&self, icao: u32) -> Photo {
        let url = self
            .thumbs
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .iter()
            .find(|(i, _)| *i == icao)
            .map(|(_, u)| u.clone());
        held_or_fetch(&self.store, &self.fetching, icao, url.is_some(), || self.relay.image(&url?))
    }

    /// The Wikipedia thumbnail of an airport's article, held in memory like an
    /// aircraft's. Blocking. The article is whatever [`Article::new`] accepted,
    /// so the only addresses ever asked are Wikipedia's.
    pub(crate) fn airport_photo(&self, article: &Article) -> Photo {
        held_or_fetch(&self.airport_store, &self.airport_fetching, article.key(), true, || {
            self.relay.wikipedia_thumbnail(article)
        })
    }

    /// The weather at an airport: the one held in memory if it is under half a
    /// minute old; else the single question for that place (waited for by
    /// whoever asks meanwhile), within the same [`PHOTO_PARALLEL`] ceiling and
    /// the relay's per-minute one. Blocking.
    pub(crate) fn weather(&self, spot: &Spot) -> Forecast {
        let key = spot.key();
        let deadline = Instant::now() + PHOTO_WAIT;
        let mut set = self.weather_fetching.set.lock().unwrap_or_else(|e| e.into_inner());
        loop {
            if let Some(w) = self.weather_book.get_at(Instant::now(), &key) {
                return Forecast::Held(w);
            }
            if !set.contains(&key) {
                if set.len() >= PHOTO_PARALLEL {
                    return Forecast::Busy;
                }
                set.insert(key.clone());
                break;
            }
            let Some(left) = deadline.checked_duration_since(Instant::now()) else {
                return Forecast::Busy;
            };
            set = self
                .weather_fetching
                .done
                .wait_timeout(set, left)
                .unwrap_or_else(|e| e.into_inner())
                .0;
        }
        drop(set);
        let _place = Place { fetching: &self.weather_fetching, key: key.clone() };
        match self.relay.weather(now_unix(), spot) {
            Outlook::Got(w) => {
                self.weather_book.put_at(Instant::now(), key, w);
                Forecast::Held(w)
            }
            Outlook::Busy => Forecast::Busy,
            Outlook::Failed => Forecast::Failed,
        }
    }
}

fn json(status: StatusCode, info: &Info) -> Response {
    (
        status,
        [(header::CONTENT_TYPE, "application/json"), (header::CACHE_CONTROL, "no-store")],
        serde_json::to_string(info).unwrap_or_default(),
    )
        .into_response()
}

pub(crate) async fn info_route(
    State(station): State<Arc<Station>>,
    Path(icao): Path<String>,
    Query(query): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Response {
    let hub = station.aircraft.clone();
    // The same door as the sockets: a station that asks for a password asks
    // for it here, and the cookie it handed out at sign-in is what answers.
    let needed = station.auth.required().is_some();
    let signed = crate::session_cookie::from_headers(&headers, &station.auth).is_some();
    match judge(hub.enabled(), needed, signed, &icao, &query) {
        Verdict::Reject(status) => json(http_status(status), &Info::bare(status)),
        Verdict::Run { icao, callsign, registration } => {
            let info = tokio::task::spawn_blocking(move || {
                hub.run(icao, callsign.as_deref(), registration.as_deref())
            })
            .await
            .unwrap_or_else(|_| Info::bare(Status::Unavailable));
            station.history.note_card(now_unix(), icao, &info);
            json(http_status(info.status), &info)
        }
    }
}

fn picture(status: StatusCode, image: Option<&Image>) -> Response {
    match image {
        Some(i) => (
            status,
            [
                (header::CONTENT_TYPE, i.content_type.as_str()),
                (header::CACHE_CONTROL, "no-store"),
                (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
            ],
            i.bytes.clone(),
        )
            .into_response(),
        None => (status, [(header::CACHE_CONTROL, "no-store")]).into_response(),
    }
}

/// `GET /aircraft-photo/{icao}`: the thumbnail of the card that is open. Same
/// door and same switch as `/aircraft-info`. The address of the picture is the
/// one the relay gave for that aircraft; a client says which aircraft and
/// nothing more.
pub(crate) async fn photo_route(
    State(station): State<Arc<Station>>,
    Path(icao): Path<String>,
    headers: HeaderMap,
) -> Response {
    let hub = station.aircraft.clone();
    let needed = station.auth.required().is_some();
    let signed = crate::session_cookie::from_headers(&headers, &station.auth).is_some();
    let icao = match judge(hub.enabled(), needed, signed, &icao, &HashMap::new()) {
        Verdict::Reject(status) => return picture(http_status(status), None),
        Verdict::Run { icao, .. } => icao,
    };
    let h = hub.clone();
    let got = tokio::task::spawn_blocking(move || h.photo(icao)).await.unwrap_or(Photo::Failed);
    match got {
        Photo::Held { image } => {
            // The picture runs out by itself; this sweep is what frees it when
            // nobody asks again.
            tokio::spawn(async move {
                tokio::time::sleep(PHOTO_HOLD + Duration::from_secs(1)).await;
                hub.store.sweep_at(Instant::now());
            });
            picture(StatusCode::OK, Some(&image))
        }
        Photo::None => picture(StatusCode::NOT_FOUND, None),
        Photo::Busy => picture(StatusCode::TOO_MANY_REQUESTS, None),
        Photo::Failed => picture(StatusCode::BAD_GATEWAY, None),
    }
}

/// What `/airport-photo` is asked: `?lang=en&title=Amsterdam_Airport_Schiphol`.
/// Only the language and the title; the station builds every address itself.
fn article_of(query: &HashMap<String, String>) -> Option<Article> {
    Article::new(query.get("lang")?, query.get("title")?)
}

/// `GET /airport-photo?lang=xx&title=...`: the Wikipedia thumbnail of the
/// airport card that is open. Same door and same switch as the aircraft's; the
/// client says which article and nothing more.
pub(crate) async fn airport_photo_route(
    State(station): State<Arc<Station>>,
    Query(query): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Response {
    let hub = station.aircraft.clone();
    let needed = station.auth.required().is_some();
    let signed = crate::session_cookie::from_headers(&headers, &station.auth).is_some();
    match airport_verdict(hub.enabled(), needed, signed, &query) {
        Err(status) => return picture(http_status(status), None),
        Ok(article) => {
            let h = hub.clone();
            let got = tokio::task::spawn_blocking(move || h.airport_photo(&article))
                .await
                .unwrap_or(Photo::Failed);
            match got {
                Photo::Held { image } => {
                    tokio::spawn(async move {
                        tokio::time::sleep(PHOTO_HOLD + Duration::from_secs(1)).await;
                        hub.airport_store.sweep_at(Instant::now());
                    });
                    picture(StatusCode::OK, Some(&image))
                }
                Photo::None => picture(StatusCode::NOT_FOUND, None),
                Photo::Busy => picture(StatusCode::TOO_MANY_REQUESTS, None),
                Photo::Failed => picture(StatusCode::BAD_GATEWAY, None),
            }
        }
    }
}

/// What `/airport-photo` answers before any work is done, as [`judge`] does for
/// the aircraft's: sign-in first, then the switch, then the query.
pub(crate) fn airport_verdict(
    enabled: bool,
    sign_in_needed: bool,
    signed_in: bool,
    query: &HashMap<String, String>,
) -> Result<Article, Status> {
    if sign_in_needed && !signed_in {
        return Err(Status::Unauthorized);
    }
    if !enabled {
        return Err(Status::Disabled);
    }
    article_of(query).ok_or(Status::Invalid)
}

/// `DELETE /airport-photo?lang=xx&title=...`: that card is closed, only its
/// picture goes. Same sign-in and header as the aircraft's release.
pub(crate) async fn airport_photo_release(
    State(station): State<Arc<Station>>,
    Query(query): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Response {
    if let Some(code) = release_refusal(&station, &headers) {
        return code.into_response();
    }
    match article_of(&query) {
        Some(article) => station.aircraft.airport_store.remove(&article.key()),
        None => return picture(StatusCode::BAD_REQUEST, None),
    }
    picture(StatusCode::NO_CONTENT, None)
}

/// `?lat=52.31&lon=4.76`: the place of the airport whose card is open, and
/// nothing else; the station builds the address itself.
fn spot_of(query: &HashMap<String, String>) -> Option<Spot> {
    let number = |k: &str| {
        let v = query.get(k)?;
        (v.len() <= 16).then(|| v.parse::<f64>().ok()).flatten()
    };
    Spot::new(number("lat")?, number("lon")?)
}

/// What `/airport-weather` answers before any work is done, as [`judge`] does
/// for the aircraft's: sign-in first, then the switch, then the query.
pub(crate) fn weather_verdict(
    enabled: bool,
    sign_in_needed: bool,
    signed_in: bool,
    query: &HashMap<String, String>,
) -> Result<Spot, Status> {
    if sign_in_needed && !signed_in {
        return Err(Status::Unauthorized);
    }
    if !enabled {
        return Err(Status::Disabled);
    }
    spot_of(query).ok_or(Status::Invalid)
}

/// `GET /airport-weather?lat=..&lon=..`: the local time's offset from UTC and the
/// temperature at the airport whose card is open. Same door and same switch as
/// the other online routes; the client says which place and nothing more, and
/// only that place goes to the service.
pub(crate) async fn weather_route(
    State(station): State<Arc<Station>>,
    Query(query): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Response {
    let hub = station.aircraft.clone();
    let needed = station.auth.required().is_some();
    let signed = crate::session_cookie::from_headers(&headers, &station.auth).is_some();
    let plain =
        |status: StatusCode| (status, [(header::CACHE_CONTROL, "no-store")]).into_response();
    let spot = match weather_verdict(hub.enabled(), needed, signed, &query) {
        Err(status) => return plain(http_status(status)),
        Ok(spot) => spot,
    };
    let got =
        tokio::task::spawn_blocking(move || hub.weather(&spot)).await.unwrap_or(Forecast::Failed);
    match got {
        Forecast::Held(w) => (
            StatusCode::OK,
            [(header::CONTENT_TYPE, "application/json"), (header::CACHE_CONTROL, "no-store")],
            serde_json::to_string(&w).unwrap_or_default(),
        )
            .into_response(),
        Forecast::Busy => plain(StatusCode::TOO_MANY_REQUESTS),
        Forecast::Failed => plain(StatusCode::BAD_GATEWAY),
    }
}

/// The sign-in and header a release needs, as for the switch.
fn release_refusal(station: &Station, headers: &HeaderMap) -> Option<StatusCode> {
    let needed = station.auth.required().is_some();
    let signed = crate::session_cookie::from_headers(headers, &station.auth).is_some();
    switch_refusal(needed, signed, true, headers.contains_key(SWITCH_HEADER))
}

/// `DELETE /aircraft-photo`: every picture goes, for a client that does not say
/// which card closed. Same sign-in as the switch, plus the custom header (see
/// [`SWITCH_HEADER`]); works whether or not the relay is still on.
pub(crate) async fn photo_release(
    State(station): State<Arc<Station>>,
    headers: HeaderMap,
) -> Response {
    if let Some(code) = release_refusal(&station, &headers) {
        return code.into_response();
    }
    station.aircraft.store.clear();
    picture(StatusCode::NO_CONTENT, None)
}

/// `DELETE /aircraft-photo/{icao}`: that card is closed, only its picture goes,
/// so a card opened a moment before keeps its own.
pub(crate) async fn photo_release_one(
    State(station): State<Arc<Station>>,
    Path(icao): Path<String>,
    headers: HeaderMap,
) -> Response {
    if let Some(code) = release_refusal(&station, &headers) {
        return code.into_response();
    }
    match parse_icao(&icao) {
        Some(icao) => station.aircraft.store.remove(&icao),
        None => return picture(StatusCode::BAD_REQUEST, None),
    }
    picture(StatusCode::NO_CONTENT, None)
}

/// What `/aircraft-online` says: whether the relay is on, and whether the
/// station's environment variable holds it there.
#[derive(serde::Serialize)]
struct Switch {
    enabled: bool,
    forced: bool,
}

fn switch_json(status: StatusCode, hub: &AircraftHub) -> Response {
    let body = Switch { enabled: hub.enabled(), forced: hub.forced.is_some() };
    (
        status,
        [(header::CONTENT_TYPE, "application/json"), (header::CACHE_CONTROL, "no-store")],
        serde_json::to_string(&body).unwrap_or_default(),
    )
        .into_response()
}

/// What a request to the switch is answered with before it is allowed to do
/// anything: the sign-in first, then (for a change) the custom header.
pub(crate) fn switch_refusal(
    sign_in_needed: bool,
    signed_in: bool,
    changing: bool,
    has_header: bool,
) -> Option<StatusCode> {
    if sign_in_needed && !signed_in {
        return Some(StatusCode::UNAUTHORIZED);
    }
    (changing && !has_header).then_some(StatusCode::FORBIDDEN)
}

/// `GET /aircraft-online`: is the relay on.
pub(crate) async fn switch_get(
    State(station): State<Arc<Station>>,
    headers: HeaderMap,
) -> Response {
    let needed = station.auth.required().is_some();
    let signed = crate::session_cookie::from_headers(&headers, &station.auth).is_some();
    if let Some(code) = switch_refusal(needed, signed, false, false) {
        return code.into_response();
    }
    switch_json(StatusCode::OK, &station.aircraft)
}

/// `POST /aircraft-online?on=1|0`: switch the relay on or off. Same sign-in as
/// the sockets, plus the custom header (see [`SWITCH_HEADER`]).
pub(crate) async fn switch_post(
    State(station): State<Arc<Station>>,
    Query(query): Query<HashMap<String, String>>,
    headers: HeaderMap,
) -> Response {
    let needed = station.auth.required().is_some();
    let signed = crate::session_cookie::from_headers(&headers, &station.auth).is_some();
    if let Some(code) = switch_refusal(needed, signed, true, headers.contains_key(SWITCH_HEADER)) {
        return code.into_response();
    }
    let Some(on) = query.get("on").map(|v| forced_by(Some(v.as_str())).unwrap_or(false)) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    station.aircraft.set_enabled(on);
    switch_json(StatusCode::OK, &station.aircraft)
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::AtomicUsize;

    use sdroxide_aircraft_online::{Lookup, Photo as OnlinePhoto, Reply, Request, parse_adsbdb};

    use super::*;

    fn q(pairs: &[(&str, &str)]) -> HashMap<String, String> {
        pairs.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect()
    }

    #[test]
    fn a_change_needs_the_sign_in_and_the_custom_header() {
        // Sign-in first.
        assert_eq!(switch_refusal(true, false, true, true), Some(StatusCode::UNAUTHORIZED));
        assert_eq!(switch_refusal(true, false, false, false), Some(StatusCode::UNAUTHORIZED));
        // A read needs no header; a change does.
        assert_eq!(switch_refusal(false, false, false, false), None);
        assert_eq!(switch_refusal(false, false, true, false), Some(StatusCode::FORBIDDEN));
        assert_eq!(switch_refusal(true, true, true, true), None);
    }

    #[test]
    fn a_client_can_switch_the_relay_on_and_off_unless_the_environment_holds_it() {
        let hub = AircraftHub::with(false, Counting(Arc::new(AtomicUsize::new(0))), 10);
        assert!(!hub.enabled());
        assert!(hub.set_enabled(true) && hub.enabled());
        assert!(!hub.set_enabled(false) && !hub.enabled());
        let held_on = AircraftHub {
            forced: Some(true),
            ..AircraftHub::with(true, Counting(Arc::new(AtomicUsize::new(0))), 10)
        };
        assert!(held_on.set_enabled(false), "the environment variable wins");
        assert!(held_on.enabled());
        let held_off = AircraftHub {
            forced: Some(false),
            ..AircraftHub::with(false, Counting(Arc::new(AtomicUsize::new(0))), 10)
        };
        assert!(!held_off.set_enabled(true), "it holds the relay off too");
        assert!(!held_off.enabled());
    }

    #[test]
    fn the_relay_starts_on_unless_the_environment_or_the_operator_said_off() {
        // Nothing configured: on.
        assert!(starts_on(None, false));
        // The operator's off mark keeps it off across a restart.
        assert!(!starts_on(None, true));
        // The environment variable outranks the mark, both ways.
        assert!(starts_on(Some(true), true));
        assert!(!starts_on(Some(false), false));
    }

    #[test]
    fn the_off_choice_is_remembered_in_a_file_and_the_route_turns_a_question_away_while_off() {
        let dir =
            std::env::temp_dir().join(format!("sdroxide-aircraft-switch-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let file = dir.join(OFF_FILE);
        let mut hub = AircraftHub::with(true, Counting(Arc::new(AtomicUsize::new(0))), 10);
        hub.switch_file = Some(file.clone());
        hub.set_enabled(false);
        assert!(file.exists(), "off is written down");
        assert_eq!(
            judge(hub.enabled(), false, false, "484006", &q(&[])),
            Verdict::Reject(Status::Disabled)
        );
        hub.set_enabled(true);
        assert!(!file.exists(), "on removes the mark: on is the default, nothing to remember");
        assert!(matches!(
            judge(hub.enabled(), false, false, "484006", &q(&[])),
            Verdict::Run { .. }
        ));
    }

    #[test]
    fn the_environment_variable_holds_the_relay_on_or_off_and_blank_leaves_it_to_the_window() {
        for on in ["1", "true", "TRUE", "yes", " on "] {
            assert_eq!(forced_by(Some(on)), Some(true), "{on:?}");
        }
        // Anything else that is written is off: a typo must not open a way out.
        for off in ["0", "false", "no", "off", "tru", "enabled", "2"] {
            assert_eq!(forced_by(Some(off)), Some(false), "{off:?}");
        }
        for free in [None, Some(""), Some("  ")] {
            assert_eq!(forced_by(free), None, "{free:?}");
        }
    }

    #[test]
    fn a_good_question_runs_with_its_address_callsign_and_registration_cleaned() {
        let v =
            judge(true, false, false, "484006", &q(&[("callsign", "klm1234"), ("reg", "ph-bxa")]));
        assert_eq!(
            v,
            Verdict::Run {
                icao: 0x484006,
                callsign: Some("KLM1234".into()),
                registration: Some("PH-BXA".into())
            }
        );
        let v = judge(true, false, false, "4ca1fa", &q(&[("callsign", ""), ("reg", "  ")]));
        assert_eq!(v, Verdict::Run { icao: 0x4CA1FA, callsign: None, registration: None });
    }

    #[test]
    fn an_address_that_is_not_six_hex_digits_is_refused() {
        for bad in ["", "48400", "4840066", "48400G", "0x4840", "48 006", "../..", "-48400"] {
            assert_eq!(
                judge(true, false, false, bad, &q(&[])),
                Verdict::Reject(Status::Invalid),
                "{bad:?}"
            );
        }
    }

    #[test]
    fn a_callsign_or_registration_that_is_not_clean_refuses_the_question() {
        for (k, v) in
            [("callsign", "KLM 123"), ("callsign", "KLM1234&x=1"), ("reg", "PH/BXA"), ("reg", "A")]
        {
            assert_eq!(
                judge(true, false, false, "484006", &q(&[(k, v)])),
                Verdict::Reject(Status::Invalid),
                "{k}={v}"
            );
        }
    }

    #[test]
    fn the_door_is_checked_before_the_switch_and_the_switch_before_the_question() {
        // No sign-in where one is needed: 401, even for a question that is also bad.
        assert_eq!(judge(true, true, false, "zz", &q(&[])), Verdict::Reject(Status::Unauthorized));
        assert_eq!(
            judge(false, true, false, "484006", &q(&[])),
            Verdict::Reject(Status::Unauthorized)
        );
        // Signed in but switched off: disabled, not invalid.
        assert_eq!(judge(false, true, true, "zz", &q(&[])), Verdict::Reject(Status::Disabled));
        // Signed in and on: the question is judged.
        assert_eq!(judge(true, true, true, "zz", &q(&[])), Verdict::Reject(Status::Invalid));
        // No password on the station: nobody needs a cookie.
        assert!(matches!(judge(true, false, false, "484006", &q(&[])), Verdict::Run { .. }));
    }

    #[test]
    fn every_status_has_the_http_code_a_client_can_act_on() {
        assert_eq!(http_status(Status::Ok), StatusCode::OK);
        assert_eq!(http_status(Status::Unknown), StatusCode::OK);
        assert_eq!(http_status(Status::Unavailable), StatusCode::OK);
        assert_eq!(http_status(Status::Busy), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(http_status(Status::Disabled), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(http_status(Status::Invalid), StatusCode::BAD_REQUEST);
        assert_eq!(http_status(Status::Unauthorized), StatusCode::UNAUTHORIZED);
    }

    struct Counting(Arc<AtomicUsize>);

    impl Upstream for Counting {
        fn fetch(&self, req: &Request) -> Reply {
            self.0.fetch_add(1, Ordering::SeqCst);
            match req {
                Request::Adsbdb { .. } => Reply::Adsbdb(Lookup::Found(
                    parse_adsbdb(r#"{"response":{"aircraft":{"type":"A320"}}}"#).unwrap(),
                )),
                Request::Photo { .. } => Reply::Photo(Lookup::NotFound),
            }
        }
    }

    #[test]
    fn a_hub_answers_through_the_relay_and_the_second_ask_costs_nothing() {
        let asked = Arc::new(AtomicUsize::new(0));
        let hub = AircraftHub::with(true, Counting(asked.clone()), 100);
        let a = hub.run(0x484006, None, None);
        assert_eq!(a.status, Status::Ok);
        assert_eq!(a.aircraft.unwrap().type_name.as_deref(), Some("A320"));
        let b = hub.run(0x484006, None, None);
        assert!(b.cached);
        assert_eq!(asked.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn past_the_ceiling_the_hub_says_busy_and_the_route_would_say_429() {
        let asked = Arc::new(AtomicUsize::new(0));
        let hub = AircraftHub::with(true, Counting(asked.clone()), 1);
        assert_eq!(hub.run(1, None, None).status, Status::Ok);
        let busy = hub.run(2, None, None);
        assert_eq!(busy.status, Status::Busy);
        assert_eq!(http_status(busy.status), StatusCode::TOO_MANY_REQUESTS);
        assert_eq!(asked.load(Ordering::SeqCst), 1);
    }

    /// A relay whose services know every aircraft and hand out a picture whose
    /// bytes are the aircraft's address, so a test can tell whose it got.
    struct Pictures {
        pictures_asked: Arc<AtomicUsize>,
        kind: &'static str,
    }

    impl Upstream for Pictures {
        fn fetch(&self, req: &Request) -> Reply {
            match req {
                Request::Adsbdb { .. } => Reply::Adsbdb(Lookup::Found(
                    parse_adsbdb(r#"{"response":{"aircraft":{"registration":"PH-BXA"}}}"#).unwrap(),
                )),
                Request::Photo { registration } => Reply::Photo(Lookup::Found(OnlinePhoto {
                    thumb_url: format!("https://t.plnspttrs.net/{registration}.jpg"),
                    page_url: "https://www.planespotters.net/photo/1".into(),
                    photographer: "A. Person".into(),
                })),
            }
        }

        fn image(&self, url: &str) -> Option<Image> {
            self.pictures_asked.fetch_add(1, Ordering::SeqCst);
            Some(Image { content_type: self.kind.into(), bytes: url.as_bytes().to_vec() })
        }

        fn wikipedia_thumbnail(&self, article: &Article) -> Option<Image> {
            self.pictures_asked.fetch_add(1, Ordering::SeqCst);
            Some(Image { content_type: self.kind.into(), bytes: article.key().into_bytes() })
        }

        fn weather(&self, spot: &Spot) -> Option<Weather> {
            self.pictures_asked.fetch_add(1, Ordering::SeqCst);
            Some(Weather { v: 1, utc_offset_s: 3600, temp_c: Some(spot.key().len() as f32) })
        }
    }

    fn hub(kind: &'static str) -> (AircraftHub<Pictures>, Arc<AtomicUsize>) {
        let n = Arc::new(AtomicUsize::new(0));
        (AircraftHub::with(true, Pictures { pictures_asked: n.clone(), kind }, 100), n)
    }

    fn bytes(p: Photo) -> Vec<u8> {
        match p {
            Photo::Held { image } => image.bytes,
            _ => panic!("no picture"),
        }
    }

    #[test]
    fn a_picture_is_only_fetched_for_an_aircraft_the_relay_gave_a_photo_for() {
        let (hub, asked) = hub("image/jpeg");
        assert!(matches!(hub.photo(0x484006), Photo::None), "never asked about: nothing to fetch");
        assert_eq!(asked.load(Ordering::SeqCst), 0);
        hub.run(0x484006, None, None);
        assert_eq!(bytes(hub.photo(0x484006)), b"https://t.plnspttrs.net/PH-BXA.jpg");
        assert_eq!(asked.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn several_pictures_are_held_so_a_card_seen_again_costs_no_download() {
        let (hub, asked) = hub("image/png");
        hub.run(0x484006, Some("KLM1"), Some("PH-AAA"));
        hub.run(0x4ca1fa, Some("EIN2"), Some("EI-BBB"));
        hub.photo(0x484006);
        hub.photo(0x4ca1fa);
        assert_eq!(hub.store.holds(), vec![0x484006, 0x4ca1fa], "both are kept");
        assert_eq!(asked.load(Ordering::SeqCst), 2);
        hub.photo(0x484006);
        assert_eq!(asked.load(Ordering::SeqCst), 2, "the first card again: from memory");
    }

    fn image(tag: u8) -> Image {
        Image { content_type: "image/png".into(), bytes: vec![tag] }
    }

    #[test]
    fn at_most_ten_pictures_are_held_and_the_oldest_goes_first() {
        let store = PhotoStore::default();
        let t0 = Instant::now();
        for i in 0..12u32 {
            store.put_at(t0, i, image(i as u8));
        }
        assert_eq!(store.holds(), (2..12).collect::<Vec<u32>>());
        store.put_at(t0, 5, image(99));
        assert_eq!(store.holds().last(), Some(&5), "a picture fetched again is the newest");
        assert_eq!(store.holds().len(), PHOTO_KEEP);
    }

    #[test]
    fn a_picture_runs_out_thirty_seconds_after_it_was_fetched() {
        let store = PhotoStore::default();
        let t0 = Instant::now();
        store.put_at(t0, 1, image(1));
        assert!(store.get_at(t0 + Duration::from_secs(29), &1).is_some());
        assert!(store.get_at(t0 + Duration::from_secs(30), &1).is_none());
        store.sweep_at(t0 + Duration::from_secs(30));
        assert!(store.holds().is_empty(), "swept, not just hidden");
    }

    #[test]
    fn closing_a_card_destroys_its_picture_and_only_its_own() {
        let (hub, _) = hub("image/jpeg");
        hub.run(0x484006, None, None);
        hub.run(0x4ca1fa, None, None);
        hub.photo(0x484006);
        hub.photo(0x4ca1fa);
        hub.store.remove(&0x484006);
        assert_eq!(hub.store.holds(), vec![0x4ca1fa], "a late release of A leaves B alone");
        hub.store.clear();
        assert!(hub.store.holds().is_empty());
    }

    /// A relay whose picture takes a while and can be told to fail by panic.
    struct Slow {
        asked: Arc<AtomicUsize>,
        pause: Duration,
        panic_first: bool,
    }

    impl Upstream for Slow {
        fn fetch(&self, req: &Request) -> Reply {
            Pictures { pictures_asked: self.asked.clone(), kind: "image/jpeg" }.fetch(req)
        }

        fn image(&self, url: &str) -> Option<Image> {
            let n = self.asked.fetch_add(1, Ordering::SeqCst);
            std::thread::sleep(self.pause);
            if self.panic_first && n == 0 {
                panic!("the picture's host blew up");
            }
            Some(Image { content_type: "image/jpeg".into(), bytes: url.as_bytes().to_vec() })
        }

        fn wikipedia_thumbnail(&self, article: &Article) -> Option<Image> {
            let n = self.asked.fetch_add(1, Ordering::SeqCst);
            std::thread::sleep(self.pause);
            if self.panic_first && n == 0 {
                panic!("the picture's host blew up");
            }
            Some(Image { content_type: "image/jpeg".into(), bytes: article.key().into_bytes() })
        }

        fn weather(&self, spot: &Spot) -> Option<Weather> {
            let n = self.asked.fetch_add(1, Ordering::SeqCst);
            std::thread::sleep(self.pause);
            if self.panic_first && n == 0 {
                panic!("the weather's host blew up");
            }
            Some(Weather { v: 1, utc_offset_s: -18000, temp_c: Some(spot.key().len() as f32) })
        }
    }

    fn slow(pause_ms: u64, panic_first: bool) -> (Arc<AircraftHub<Slow>>, Arc<AtomicUsize>) {
        let n = Arc::new(AtomicUsize::new(0));
        let up = Slow { asked: n.clone(), pause: Duration::from_millis(pause_ms), panic_first };
        (Arc::new(AircraftHub::with(true, up, 100)), n)
    }

    #[test]
    fn a_card_clicked_while_another_picture_loads_is_served_not_turned_away() {
        let (hub, _) = slow(300, false);
        hub.run(0x484006, Some("KLM1"), Some("PH-AAA"));
        hub.run(0x4ca1fa, Some("EIN2"), Some("EI-BBB"));
        let h = hub.clone();
        let first = std::thread::spawn(move || h.photo(0x484006));
        std::thread::sleep(Duration::from_millis(80));
        assert!(matches!(hub.photo(0x4ca1fa), Photo::Held { .. }), "B while A loads");
        assert!(matches!(first.join().unwrap(), Photo::Held { .. }));
    }

    #[test]
    fn the_same_aircraft_asked_twice_at_once_is_fetched_once() {
        let (hub, asked) = slow(300, false);
        hub.run(0x484006, None, None);
        let h = hub.clone();
        let first = std::thread::spawn(move || h.photo(0x484006));
        std::thread::sleep(Duration::from_millis(80));
        assert!(matches!(hub.photo(0x484006), Photo::Held { .. }));
        assert!(matches!(first.join().unwrap(), Photo::Held { .. }));
        assert_eq!(asked.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn a_download_that_blows_up_does_not_block_the_next_one() {
        let (hub, _) = slow(0, true);
        hub.run(0x484006, None, None);
        let h = hub.clone();
        assert!(std::thread::spawn(move || h.photo(0x484006)).join().is_err());
        assert!(matches!(hub.photo(0x484006), Photo::Held { .. }), "not stuck busy for ever");
    }

    #[test]
    fn past_three_pictures_at_once_the_next_is_told_to_come_back() {
        let (hub, _) = slow(400, false);
        for i in 1..=4u32 {
            hub.run(i, None, None);
        }
        let mut running = Vec::new();
        for i in 1..=3u32 {
            let h = hub.clone();
            running.push(std::thread::spawn(move || h.photo(i)));
        }
        std::thread::sleep(Duration::from_millis(100));
        assert!(matches!(hub.photo(4), Photo::Busy));
        for r in running {
            assert!(matches!(r.join().unwrap(), Photo::Held { .. }));
        }
    }

    #[test]
    fn something_that_is_not_a_picture_is_never_held() {
        let (hub, _) = hub("text/html");
        hub.run(0x484006, None, None);
        assert!(matches!(hub.photo(0x484006), Photo::Failed));
        assert!(hub.store.holds().is_empty());
    }

    fn schiphol() -> Article {
        Article::new("en", "Amsterdam_Airport_Schiphol").unwrap()
    }

    fn query(lang: &str, title: &str) -> HashMap<String, String> {
        HashMap::from([
            ("lang".to_string(), lang.to_string()),
            ("title".to_string(), title.to_string()),
        ])
    }

    #[test]
    fn an_airport_picture_is_fetched_once_held_and_kept_apart_from_the_aircraft() {
        let (hub, asked) = hub("image/jpeg");
        assert_eq!(bytes(hub.airport_photo(&schiphol())), b"en:Amsterdam_Airport_Schiphol");
        assert_eq!(asked.load(Ordering::SeqCst), 1);
        hub.airport_photo(&schiphol());
        assert_eq!(asked.load(Ordering::SeqCst), 1, "the card again: from memory");
        assert_eq!(hub.airport_store.holds(), vec!["en:Amsterdam_Airport_Schiphol".to_string()]);
        assert!(hub.store.holds().is_empty(), "no aircraft picture was made of it");
    }

    #[test]
    fn closing_an_airport_card_destroys_its_picture_and_only_its_own() {
        let (hub, _) = hub("image/png");
        let other = Article::new("fr", "Aéroport_de_Paris-Charles-de-Gaulle").unwrap();
        hub.airport_photo(&schiphol());
        hub.airport_photo(&other);
        hub.airport_store.remove(&schiphol().key());
        assert_eq!(hub.airport_store.holds(), vec![other.key()], "a late release leaves B alone");
    }

    #[test]
    fn a_second_airport_is_served_while_the_first_loads_and_the_same_one_is_fetched_once() {
        let (hub, asked) = slow(300, false);
        let other = Article::new("fr", "Orly").unwrap();
        let h = hub.clone();
        let first = std::thread::spawn(move || h.airport_photo(&schiphol()));
        std::thread::sleep(Duration::from_millis(80));
        assert!(matches!(hub.airport_photo(&other), Photo::Held { .. }), "B while A loads");
        assert!(matches!(hub.airport_photo(&schiphol()), Photo::Held { .. }), "A waited for");
        assert!(matches!(first.join().unwrap(), Photo::Held { .. }));
        assert_eq!(asked.load(Ordering::SeqCst), 2, "one download each");
    }

    #[test]
    fn an_airport_download_that_blows_up_does_not_block_the_next_one() {
        let (hub, _) = slow(0, true);
        let h = hub.clone();
        assert!(std::thread::spawn(move || h.airport_photo(&schiphol())).join().is_err());
        assert!(matches!(hub.airport_photo(&schiphol()), Photo::Held { .. }));
    }

    #[test]
    fn past_three_airport_pictures_at_once_the_next_is_told_to_come_back() {
        let (hub, _) = slow(400, false);
        let mut running = Vec::new();
        for t in ["A", "B", "C"] {
            let h = hub.clone();
            running
                .push(std::thread::spawn(move || h.airport_photo(&Article::new("en", t).unwrap())));
        }
        std::thread::sleep(Duration::from_millis(100));
        assert!(matches!(hub.airport_photo(&Article::new("en", "D").unwrap()), Photo::Busy));
        for r in running {
            assert!(matches!(r.join().unwrap(), Photo::Held { .. }));
        }
    }

    #[test]
    fn the_eleventh_airport_picture_pushes_out_the_oldest() {
        let (hub, _) = hub("image/png");
        for i in 0..11 {
            hub.airport_photo(&Article::new("en", &format!("Airport_{i}")).unwrap());
        }
        let held = hub.airport_store.holds();
        assert_eq!(held.len(), PHOTO_KEEP);
        assert_eq!(held.first().map(String::as_str), Some("en:Airport_1"));
    }

    #[test]
    fn something_that_is_not_a_picture_is_never_held_for_an_airport_either() {
        let (hub, _) = hub("text/html");
        assert!(matches!(hub.airport_photo(&schiphol()), Photo::Failed));
        assert!(hub.airport_store.holds().is_empty());
    }

    #[test]
    fn the_airport_route_answers_the_door_then_the_switch_then_the_query() {
        let q = query("en", "Amsterdam_Airport_Schiphol");
        assert_eq!(airport_verdict(true, true, false, &q), Err(Status::Unauthorized));
        assert_eq!(airport_verdict(false, false, false, &q), Err(Status::Disabled));
        assert_eq!(
            airport_verdict(true, true, true, &q).unwrap().key(),
            "en:Amsterdam_Airport_Schiphol"
        );
        for bad in [query("", "x"), query("en", "../etc"), query("en", "a/b"), query("EN", "x")] {
            assert_eq!(airport_verdict(true, false, false, &bad), Err(Status::Invalid));
        }
        assert_eq!(
            airport_verdict(true, false, false, &HashMap::from([("lang".into(), "en".into())])),
            Err(Status::Invalid),
            "a title is needed"
        );
    }

    fn spot(lat: f64, lon: f64) -> Spot {
        Spot::new(lat, lon).unwrap()
    }

    fn place(lat: &str, lon: &str) -> HashMap<String, String> {
        HashMap::from([("lat".to_string(), lat.to_string()), ("lon".to_string(), lon.to_string())])
    }

    #[test]
    fn the_weather_of_a_place_is_asked_once_and_held_in_memory() {
        let (hub, asked) = hub("image/jpeg");
        let here = spot(52.31, 4.76);
        assert!(matches!(hub.weather(&here), Forecast::Held(w) if w.utc_offset_s == 3600));
        assert!(matches!(hub.weather(&spot(52.308, 4.763)), Forecast::Held(_)));
        assert_eq!(asked.load(Ordering::SeqCst), 1, "the same kilometre: one question");
        assert!(matches!(hub.weather(&spot(40.64, -73.78)), Forecast::Held(_)));
        assert_eq!(asked.load(Ordering::SeqCst), 2, "another airport: its own");
    }

    #[test]
    fn the_weather_runs_out_thirty_seconds_after_it_was_got() {
        let book = WeatherBook::default();
        let t0 = Instant::now();
        let w = Weather { v: 1, utc_offset_s: 0, temp_c: None };
        book.put_at(t0, "a".into(), w);
        assert!(book.get_at(t0 + Duration::from_secs(29), "a").is_some());
        assert!(book.get_at(t0 + Duration::from_secs(31), "a").is_none());
        assert_eq!(WEATHER_HOLD, Duration::from_secs(30));
    }

    #[test]
    fn only_thirty_two_places_are_held_and_the_oldest_goes_first() {
        let book = WeatherBook::default();
        let t0 = Instant::now();
        let w = Weather { v: 1, utc_offset_s: 0, temp_c: None };
        for i in 0..=WEATHER_KEEP {
            book.put_at(t0, i.to_string(), w);
        }
        assert!(book.get_at(t0, "0").is_none());
        assert!(
            book.get_at(t0, "1").is_some() && book.get_at(t0, &WEATHER_KEEP.to_string()).is_some()
        );
    }

    #[test]
    fn a_place_asked_by_two_clients_at_once_is_one_question_and_a_fourth_place_waits() {
        let (hub, asked) = slow(300, false);
        let h = hub.clone();
        let first = std::thread::spawn(move || h.weather(&spot(1.0, 1.0)));
        std::thread::sleep(Duration::from_millis(80));
        assert!(matches!(hub.weather(&spot(1.0, 1.0)), Forecast::Held(_)), "waited for");
        assert!(matches!(first.join().unwrap(), Forecast::Held(_)));
        assert_eq!(asked.load(Ordering::SeqCst), 1);

        let mut running = Vec::new();
        for i in 0..3 {
            let h = hub.clone();
            running.push(std::thread::spawn(move || h.weather(&spot(10.0 + f64::from(i), 0.0))));
        }
        std::thread::sleep(Duration::from_millis(100));
        assert_eq!(hub.weather(&spot(20.0, 0.0)), Forecast::Busy, "past three at once");
        for r in running {
            assert!(matches!(r.join().unwrap(), Forecast::Held(_)));
        }
    }

    #[test]
    fn a_weather_question_that_blows_up_does_not_block_the_next_one() {
        let (hub, _) = slow(0, true);
        let h = hub.clone();
        assert!(std::thread::spawn(move || h.weather(&spot(5.0, 5.0))).join().is_err());
        assert!(matches!(hub.weather(&spot(5.0, 5.0)), Forecast::Held(_)));
    }

    #[test]
    fn no_weather_is_a_failure_and_the_ceiling_a_busy_and_neither_is_held() {
        struct Nothing;
        impl Upstream for Nothing {
            fn fetch(&self, _: &Request) -> Reply {
                Reply::Photo(Lookup::Failed)
            }
        }
        let hub = AircraftHub::with(true, Nothing, 100);
        assert_eq!(hub.weather(&spot(1.0, 2.0)), Forecast::Failed);
        let (hub, asked) = {
            let n = Arc::new(AtomicUsize::new(0));
            (
                AircraftHub::with(
                    true,
                    Pictures { pictures_asked: n.clone(), kind: "image/png" },
                    1,
                ),
                n,
            )
        };
        assert!(matches!(hub.weather(&spot(1.0, 2.0)), Forecast::Held(_)));
        assert_eq!(hub.weather(&spot(3.0, 4.0)), Forecast::Busy, "one question a minute");
        assert_eq!(asked.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn the_weather_route_answers_the_door_then_the_switch_then_the_query() {
        let q = place("52.31", "4.76");
        assert_eq!(weather_verdict(true, true, false, &q), Err(Status::Unauthorized));
        assert_eq!(weather_verdict(false, false, false, &q), Err(Status::Disabled));
        assert_eq!(weather_verdict(true, true, true, &q).unwrap().key(), "52.31,4.76");
        for bad in [
            place("", "4.76"),
            place("abc", "4.76"),
            place("91", "0"),
            place("0", "181"),
            place("NaN", "0"),
            place("inf", "0"),
            place("0.000000000000000001", "0"),
            place("52.31", "4.76&x=1"),
        ] {
            assert_eq!(weather_verdict(true, false, false, &bad), Err(Status::Invalid), "{bad:?}");
        }
        assert_eq!(
            weather_verdict(true, false, false, &HashMap::from([("lat".into(), "1".into())])),
            Err(Status::Invalid),
            "both numbers are needed"
        );
    }

    #[test]
    fn the_hold_is_half_a_minute_for_ten_pictures() {
        assert_eq!(PHOTO_HOLD, Duration::from_secs(30));
        assert_eq!((PHOTO_KEEP, PHOTO_PARALLEL), (10, 3));
    }
}
