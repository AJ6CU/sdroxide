//! The asking: one question to the station's relay, one thumbnail from
//! Planespotters' picture host, and the way back to the window.
//!
//! The decisions are [`super::aircraft_online`]'s; this only sends what it
//! says to send and hands back what arrives, over a channel the window drains
//! every frame. Native sends from a thread, the browser from a `fetch`.
//!
//! What goes out, and only this:
//!
//! * `GET {station}/aircraft-info/{icao}` — the station's own relay. The
//!   browser sends its cookie by itself; the native remote client signs in to
//!   the station with the password it already holds, once, and keeps the
//!   cookie it gets for the length of the run.
//! * `GET` of the thumbnail address the relay returned, unchanged, with no
//!   cookie and no referrer. It is the one request that goes anywhere but the
//!   station, and the one Planespotters' terms ask the client to make.
//!
//! The picture is decoded and drawn from memory and dropped with the window;
//! nothing is written to disk, to the browser's storage, or to the settings.

use std::sync::mpsc::{Receiver, Sender, channel};

use crate::i18n::tr;
use eframe::egui;

use super::aircraft_export::ExportStore;
use super::aircraft_history::{self, HistoryStore};
use super::aircraft_online::{
    self, Answer, OnlineStore, ThumbStatus, Want, classify, info_url, thumb_fetchable,
};
use super::aircraft_stats::{self, StatsStore};
use super::aircraft_watch::{self, WatchStore};

/// The most bytes read of a reply from the relay.
const MAX_INFO_BYTES: u64 = 64 * 1024;
/// The most bytes read of the station's record of one aircraft: up to 600
/// positions, a few dozen bytes each.
const MAX_HISTORY_BYTES: u64 = 256 * 1024;
/// The most bytes read of the station's table of the fleet: up to 4000 aircraft,
/// a few hundred bytes each.
const MAX_EXPORT_BYTES: u64 = 8 * 1024 * 1024;
/// The most bytes read of the operator's watch list: a hundred short entries.
const MAX_WATCH_BYTES: u64 = 16 * 1024;
/// The PROGRAMS state the station keeps: every station of the schedules starred.
const MAX_PROGRAMS_BYTES: u64 = 1024 * 1024 + 4096;
/// The most bytes read of the station's list of everything it kept: up to 4000
/// aircraft, a few hundred bytes each.
const MAX_STATS_BYTES: u64 = 4 * 1024 * 1024;
/// The most bytes read of the answer to the relay switch: two booleans.
const MAX_SWITCH_BYTES: u64 = 1024;
/// The station's weather answer is a few dozen bytes.
const MAX_WEATHER_BYTES: u64 = 1024;
/// The most bytes read of a thumbnail. They are a few tens of kilobytes.
const MAX_THUMB_BYTES: u64 = 2 * 1024 * 1024;
/// Widest or tallest a thumbnail may decode to.
const MAX_THUMB_SIDE: u32 = 2048;
/// Thumbnails kept at once of each kind (aircraft, airport): the open card's and no other. The station holds a
/// few for half a minute; this one lets its own go when the card is closed.
const KEEP_THUMBS: usize = 1;

/// What names an airport's thumbnail in [`OnlineClient`]: the article, not an address.
pub(in crate::app) fn airport_thumb_key(lang: &str, title: &str) -> String {
    format!("wikipedia:{lang}:{title}")
}

enum Msg {
    Info { icao: u32, status: Option<u16>, body: String },
    History { icao: u32, status: Option<u16>, body: String },
    Export { status: Option<u16>, body: String },
    Watch { status: Option<u16>, body: String },
    Programs { status: Option<u16>, body: String },
    Stats { status: Option<u16>, body: String },
    Switch { status: Option<u16>, body: String },
    Weather { airport: usize, status: Option<u16>, body: String },
    Thumb { url: String, bytes: Option<Vec<u8>>, missing: bool },
}

/// What a held thumbnail is of, which says where it is fetched from and let go.
#[derive(Clone, PartialEq)]
enum ThumbOf {
    /// An aircraft's, by address. Natively fetched from Planespotters' own host.
    Aircraft(u32),
    /// An airport's Wikipedia picture, by language and article title. Always
    /// through the station, which is the only one that knows where Wikipedia is.
    Airport { lang: String, title: String },
}

impl ThumbOf {
    fn same_kind(&self, other: &ThumbOf) -> bool {
        matches!(
            (self, other),
            (ThumbOf::Aircraft(_), ThumbOf::Aircraft(_))
                | (ThumbOf::Airport { .. }, ThumbOf::Airport { .. })
        )
    }

    /// The station's address for the picture, or `None` without a station.
    fn fetch_url(&self, peer: Option<&str>) -> Option<String> {
        match self {
            ThumbOf::Aircraft(icao) => peer.and_then(|p| aircraft_online::photo_url(p, *icao)),
            ThumbOf::Airport { lang, title } => {
                peer.and_then(|p| aircraft_online::airport_photo_url(p, lang, title))
            }
        }
    }
}

struct Thumb {
    /// An aircraft's: Planespotters' address of the picture. An airport's:
    /// [`airport_thumb_key`].
    url: String,
    of: ThumbOf,
    /// The pass in which a card last asked for it: a card that is open asks in
    /// every pass it is drawn, and one that is closed stops.
    wanted: u64,
    status: ThumbStatus,
    /// Times it was asked for, and when it last failed (egui seconds).
    tries: u8,
    failed_at: f64,
    texture: Option<egui::TextureHandle>,
}

/// The online half of the AIRCRAFT window, held for the session in memory.
pub(in crate::app) struct OnlineClient {
    pub store: OnlineStore,
    /// The station's record of the one aircraft whose history is on.
    pub history: HistoryStore,
    /// Saving the fleet as CSV or JSON: the station's table, or one made here.
    pub export: ExportStore,
    /// The operator's list of aircraft to be told about, as the station keeps it.
    pub watch: WatchStore,
    /// The station's list of every aircraft it kept, for the statistics view.
    pub stats: StatsStore,
    /// What the station answered about the PROGRAMS state, not yet taken by
    /// [`SdroxideApp::programs_sync_tick`].
    pub programs_inbox: Vec<(Option<u16>, String)>,
    /// A request to switch the station's online relay, waiting to be sent.
    switch_wanted: Option<bool>,
    /// A request is out and its answer not yet in.
    pub switch_asking: bool,
    /// What the last request asked for, to tell an answer that did not follow it.
    switch_asked: Option<bool>,
    /// What the station last said the relay is, once the switch has been used.
    switch_state: Option<bool>,
    /// Why the last request to switch did not work, if it did not.
    pub switch_note: Option<String>,
    tx: Sender<Msg>,
    rx: Receiver<Msg>,
    thumbs: Vec<Thumb>,
    /// The local time and temperature of the airport whose box is open.
    weather: aircraft_online::WeatherStore,
}

impl Default for OnlineClient {
    fn default() -> Self {
        let (tx, rx) = channel();
        Self {
            store: OnlineStore::default(),
            history: HistoryStore::default(),
            export: ExportStore::default(),
            watch: WatchStore::default(),
            stats: StatsStore::default(),
            programs_inbox: Vec::new(),
            switch_wanted: None,
            switch_asking: false,
            switch_asked: None,
            switch_state: None,
            switch_note: None,
            tx,
            rx,
            thumbs: Vec::new(),
            weather: aircraft_online::WeatherStore::default(),
        }
    }
}

/// A thumbnail as pixels, or `None` for anything that is not a small picture.
pub(in crate::app) fn decode_thumb(bytes: &[u8]) -> Option<egui::ColorImage> {
    if bytes.is_empty() || bytes.len() as u64 > MAX_THUMB_BYTES {
        return None;
    }
    let img = image::load_from_memory(bytes).ok()?;
    if img.width() == 0
        || img.height() == 0
        || img.width() > MAX_THUMB_SIDE
        || img.height() > MAX_THUMB_SIDE
    {
        return None;
    }
    let rgba = img.to_rgba8();
    Some(egui::ColorImage::from_rgba_unmultiplied(
        [rgba.width() as usize, rgba.height() as usize],
        rgba.as_raw(),
    ))
}

impl OnlineClient {
    /// Take what has arrived since the last frame.
    pub(in crate::app) fn poll(&mut self, ctx: &egui::Context, now: i64) {
        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                Msg::Info { icao, status, body } => {
                    self.store.deliver(icao, classify(status, &body), now);
                }
                Msg::History { icao, status, body } => {
                    self.history.deliver(icao, aircraft_history::classify(status, &body), now);
                }
                Msg::Export { status, body } => {
                    self.export.deliver(status, &body, now);
                }
                Msg::Watch { status, body } => {
                    self.watch.deliver(aircraft_watch::classify(status, &body), now);
                }
                Msg::Programs { status, body } => {
                    self.programs_inbox.push((status, body));
                }
                Msg::Switch { status, body } => {
                    self.switch_asking = false;
                    let answer = aircraft_online::classify_switch(status, &body);
                    let held = self
                        .switch_asked
                        .take()
                        .and_then(|asked| aircraft_online::held_note(asked, &answer));
                    match answer {
                        aircraft_online::SwitchAnswer::On => {
                            self.switch_state = Some(true);
                            self.store.unblock();
                        }
                        aircraft_online::SwitchAnswer::Off => self.switch_state = Some(false),
                        aircraft_online::SwitchAnswer::Refused(why) => {
                            self.switch_note = Some(why);
                            continue;
                        }
                    }
                    self.switch_note = held;
                }
                Msg::Stats { status, body } => {
                    self.stats.deliver(aircraft_stats::classify(status, &body), now);
                }
                Msg::Weather { airport, status, body } => {
                    let at = ctx.input(|i| i.time);
                    self.weather.deliver(
                        airport,
                        aircraft_online::classify_weather(status, &body),
                        at,
                    );
                }
                // `missing`: the station answered 404 — the page has no picture,
                // which asking again will not change.
                Msg::Thumb { url, bytes, missing } => {
                    let image = bytes.as_deref().and_then(decode_thumb);
                    if let Some(t) = self.thumbs.iter_mut().find(|t| t.url == url) {
                        match image {
                            Some(ci) => {
                                t.texture = Some(ctx.load_texture(
                                    format!("aircraft_thumb_{}", t.url),
                                    ci,
                                    egui::TextureOptions::LINEAR,
                                ));
                                t.status = ThumbStatus::Ready;
                            }
                            None => {
                                t.status = ThumbStatus::Failed;
                                t.failed_at = ctx.input(|i| i.time);
                                if missing {
                                    // No picture to wait for: no "…" and no retries.
                                    t.tries = aircraft_online::THUMB_TRIES;
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Send the question the store says is due, if there is one. `peer` is the
    /// address the radio is connected to; without one there is nobody to ask.
    pub(in crate::app) fn ask(
        &mut self,
        ctx: &egui::Context,
        peer: Option<&str>,
        want: Option<&Want>,
        now: i64,
    ) {
        let Some(w) = self.store.next_request(want, peer.is_some(), now) else { return };
        let Some(url) = peer.and_then(|p| info_url(p, &w)) else {
            // A connection that is not an address cannot be asked.
            self.store.deliver(
                w.icao,
                Answer::Unavailable(super::aircraft_online::Reason::NoStation),
                now,
            );
            return;
        };
        let station = peer.map(crate::login::station_key).unwrap_or_default();
        send_info(url, station, Kind::Info(w.icao), None, self.tx.clone(), ctx.clone());
    }

    /// Fetch the station's record of the aircraft whose history is on, if the
    /// store says one is due. Without a station, or with nothing on, nothing
    /// is sent.
    pub(in crate::app) fn ask_history(
        &mut self,
        ctx: &egui::Context,
        peer: Option<&str>,
        now: i64,
    ) {
        let Some(icao) = self.history.next_request(peer.is_some(), now) else { return };
        let Some(url) = peer.and_then(|p| aircraft_history::history_url(p, icao)) else {
            // A connection that is not an address cannot be asked.
            self.history.deliver(
                icao,
                aircraft_history::Answer::Unavailable(aircraft_history::Reason::NoStation),
                now,
            );
            return;
        };
        let station = peer.map(crate::login::station_key).unwrap_or_default();
        send_info(url, station, Kind::History(icao), None, self.tx.clone(), ctx.clone());
    }

    /// Send what the watch list's store has due: a change the person asked for,
    /// or a read of the list. Without a station nothing is sent.
    pub(in crate::app) fn ask_watch(&mut self, ctx: &egui::Context, peer: Option<&str>, now: i64) {
        let Some(req) = self.watch.next_request(peer.is_some(), now) else { return };
        let Some(url) = peer.and_then(aircraft_watch::watch_url) else {
            // A connection that is not an address cannot be asked.
            self.watch.deliver(
                aircraft_watch::Answer::Unavailable(aircraft_history::Reason::NoStation),
                now,
            );
            return;
        };
        let station = peer.map(crate::login::station_key).unwrap_or_default();
        let post = aircraft_watch::body_of(&req);
        send_info(url, station, Kind::Watch, post, self.tx.clone(), ctx.clone());
    }

    /// Send the PROGRAMS state request: a read, or the state as the body of a
    /// write. Without a station nothing is sent.
    pub(in crate::app) fn ask_programs(
        &mut self,
        ctx: &egui::Context,
        peer: Option<&str>,
        post: Option<String>,
    ) {
        let Some(url) = peer.and_then(super::programs_sync::programs_url) else { return };
        let station = peer.map(crate::login::station_key).unwrap_or_default();
        send_info(url, station, Kind::Programs, post, self.tx.clone(), ctx.clone());
    }

    /// Whether the relay is on, as far as this client knows: on until the
    /// station has said otherwise, which is its default.
    pub(in crate::app) fn relay_on(&self) -> bool {
        !self.store.relay_off() && self.switch_state != Some(false)
    }

    /// Ask to switch the station's online relay on or off; the request goes
    /// out on the next [`Self::ask_switch`].
    pub(in crate::app) fn want_switch(&mut self, on: bool) {
        self.switch_wanted = Some(on);
        self.switch_note = None;
    }

    /// Send the switch request if one was asked for. Without a station there is
    /// nobody to switch, and the note says so.
    pub(in crate::app) fn ask_switch(&mut self, ctx: &egui::Context, peer: Option<&str>) {
        let Some(on) = self.switch_wanted.take() else { return };
        let Some(url) = peer.and_then(|p| aircraft_online::switch_url(p, on)) else {
            self.switch_note = Some(tr!("aircraft-switch-no-station"));
            return;
        };
        let station = peer.map(crate::login::station_key).unwrap_or_default();
        self.switch_asking = true;
        self.switch_asked = Some(on);
        send_info(url, station, Kind::Switch, Some(String::new()), self.tx.clone(), ctx.clone());
    }

    /// Fetch the station's list for the statistics view, if the store says one
    /// is due. Closed, or without a station, nothing is sent.
    pub(in crate::app) fn ask_stats(&mut self, ctx: &egui::Context, peer: Option<&str>, now: i64) {
        if !self.stats.next_request(peer.is_some(), now) {
            return;
        }
        let Some(url) = peer.and_then(aircraft_stats::list_url) else {
            // A connection that is not an address cannot be asked.
            self.stats.deliver(
                aircraft_stats::StatsState::Unavailable(aircraft_history::Reason::NoStation),
                now,
            );
            return;
        };
        let station = peer.map(crate::login::station_key).unwrap_or_default();
        send_info(url, station, Kind::Stats, None, self.tx.clone(), ctx.clone());
    }

    /// Ask the station for its table if an export was pressed and one is due. With
    /// no station nothing is sent: the file is made here.
    pub(in crate::app) fn ask_export(&mut self, ctx: &egui::Context, peer: Option<&str>, now: i64) {
        let Some(format) = self.export.next_fetch(peer.is_some(), now) else { return };
        let Some(url) = peer.and_then(|p| super::aircraft_export::export_url(p, format)) else {
            // A connection that is not an address cannot be asked.
            self.export.give_up(aircraft_history::Reason::NoStation);
            return;
        };
        let station = peer.map(crate::login::station_key).unwrap_or_default();
        send_info(url, station, Kind::Export, None, self.tx.clone(), ctx.clone());
    }

    pub(in crate::app) fn thumb_status(&self, url: &str) -> ThumbStatus {
        self.thumbs.iter().find(|t| t.url == url).map_or(ThumbStatus::NotTried, |t| t.status)
    }

    pub(in crate::app) fn thumb_texture(&self, url: &str) -> Option<&egui::TextureHandle> {
        self.thumbs.iter().find(|t| t.url == url).and_then(|t| t.texture.as_ref())
    }

    /// What the open airport box shows above its text this frame, and the page
    /// its credit points to. Asking here, every frame the box is drawn, is what
    /// keeps the picture; an airport without an article, or no station to ask,
    /// is a box of text alone.
    pub(in crate::app) fn airport_card_photo(
        &mut self,
        ctx: &egui::Context,
        peer: Option<&str>,
        ident: &str,
    ) -> Option<(crate::adsb_map::CardPhoto, String)> {
        let (lang, title) = crate::airdata::wikipedia_of(ident)?;
        peer?;
        self.want_airport_thumb(ctx, peer, lang, title);
        let key = airport_thumb_key(lang, title);
        let photo = match self.thumb_texture(&key) {
            Some(t) => crate::adsb_map::CardPhoto::Ready { texture: t.id(), size: t.size_vec2() },
            None if self.thumb_pending(&key) => crate::adsb_map::CardPhoto::Pending,
            None => crate::adsb_map::CardPhoto::None,
        };
        Some((photo, aircraft_online::airport_page_url(lang, title)))
    }

    /// What the open airport box says about its airport's clock and air this
    /// frame. Asking here, every frame the box is drawn, is what keeps the
    /// answer; with no station, or the relay off, nothing is asked and the box
    /// shows UTC alone, never an invented local time.
    pub(in crate::app) fn airport_clock(
        &mut self,
        ctx: &egui::Context,
        peer: Option<&str>,
        airport: usize,
        place: (f64, f64),
    ) -> aircraft_online::Clock {
        let Some(peer) = peer.filter(|_| self.relay_on()) else {
            self.weather.clear();
            return aircraft_online::Clock::Offline;
        };
        let pass = ctx.cumulative_pass_nr();
        let now = ctx.input(|i| i.time);
        if self.weather.step(airport, pass, now) {
            match aircraft_online::airport_weather_url(peer, place.0, place.1) {
                Some(url) => {
                    let station = crate::login::station_key(peer);
                    send_info(
                        url,
                        station,
                        Kind::Weather(airport),
                        None,
                        self.tx.clone(),
                        ctx.clone(),
                    );
                }
                None => self.weather.deliver(airport, aircraft_online::Reading::Gone, now),
            }
        }
        let clock = self.weather.clock(airport);
        if self.weather.needs_a_later_look() || matches!(clock, aircraft_online::Clock::Known(_)) {
            // The pause before a repeat, the refresh, and the minute hand.
            ctx.request_repaint_after(std::time::Duration::from_secs(5));
        }
        clock
    }

    /// Whether a thumbnail is on its way or will be asked again: its card keeps
    /// the room for it. False once it has come, or has failed for good.
    pub(in crate::app) fn thumb_pending(&self, url: &str) -> bool {
        self.thumbs.iter().find(|t| t.url == url).is_some_and(|t| {
            t.status == ThumbStatus::Loading
                || (t.status == ThumbStatus::Failed && t.tries < aircraft_online::THUMB_TRIES)
        })
    }

    /// Start loading this thumbnail if it has not been tried and is an address
    /// Planespotters hands out.
    ///
    /// `icao` is the aircraft the card is for: in the browser the picture comes
    /// from the station (`peer`), which asks Planespotters for the address it
    /// itself gave, because Planespotters' host sends no CORS header and the
    /// browser would not let the pixels be read. Asking again every frame the
    /// card is drawn is what keeps the picture; see [`Self::release_thumbs`].
    pub(in crate::app) fn want_thumb(
        &mut self,
        ctx: &egui::Context,
        peer: Option<&str>,
        icao: u32,
        url: &str,
    ) {
        if !thumb_fetchable(url) {
            return;
        }
        self.want(ctx, peer, ThumbOf::Aircraft(icao), url);
    }

    /// The same for an airport's Wikipedia picture, by the article the table
    /// names. Always through the station; without one nothing is asked.
    pub(in crate::app) fn want_airport_thumb(
        &mut self,
        ctx: &egui::Context,
        peer: Option<&str>,
        lang: &str,
        title: &str,
    ) {
        if peer.is_none() {
            return;
        }
        let of = ThumbOf::Airport { lang: lang.to_string(), title: title.to_string() };
        self.want(ctx, peer, of, &airport_thumb_key(lang, title));
    }

    fn want(&mut self, ctx: &egui::Context, peer: Option<&str>, of: ThumbOf, key: &str) {
        let pass = ctx.cumulative_pass_nr();
        let now = ctx.input(|i| i.time);
        if let Some(t) = self.thumbs.iter_mut().find(|t| t.url == key) {
            t.wanted = pass;
            // A busy station or a slow host mends itself: ask again, a few times.
            let retry = t.status == ThumbStatus::Failed
                && t.tries < aircraft_online::THUMB_TRIES
                && now - t.failed_at >= aircraft_online::THUMB_RETRY_S;
            if !retry {
                if t.status == ThumbStatus::Failed && t.tries < aircraft_online::THUMB_TRIES {
                    ctx.request_repaint_after(std::time::Duration::from_secs(1));
                }
                return;
            }
            t.status = ThumbStatus::Loading;
            t.tries += 1;
            send_thumb(key.to_string(), &of, peer, self.tx.clone(), ctx.clone());
            return;
        }
        // Only one at a time of each kind: the card that is open is the only one.
        while self.thumbs.iter().filter(|t| t.of.same_kind(&of)).count() >= KEEP_THUMBS {
            if let Some(i) = self.thumbs.iter().position(|t| t.of.same_kind(&of)) {
                self.thumbs.remove(i);
            }
        }
        self.thumbs.push(Thumb {
            url: key.to_string(),
            of: of.clone(),
            wanted: pass,
            status: ThumbStatus::Loading,
            tries: 1,
            failed_at: 0.0,
            texture: None,
        });
        send_thumb(key.to_string(), &of, peer, self.tx.clone(), ctx.clone());
    }

    /// Once a frame: a thumbnail no card has asked for in the pass before this
    /// one or the one before that belongs to a card that is closed — destroy the
    /// picture here and tell the station to drop its own. A station that never
    /// hears (tab closed, line cut) drops it itself after half a minute.
    pub(in crate::app) fn release_thumbs(&mut self, ctx: &egui::Context, peer: Option<&str>) {
        self.weather.release(ctx.cumulative_pass_nr());
        if self.thumbs.is_empty() {
            return;
        }
        let pass = ctx.cumulative_pass_nr();
        let (closed, open): (Vec<Thumb>, Vec<Thumb>) = std::mem::take(&mut self.thumbs)
            .into_iter()
            .partition(|t| aircraft_online::thumb_unwanted(Some(t.wanted), pass));
        self.thumbs = open;
        for t in closed {
            if let Some(url) = t.of.fetch_url(peer) {
                send_release(url, &t.of, peer);
            }
        }
        if !self.thumbs.is_empty() {
            // A closed card draws nothing more, so something must look again.
            ctx.request_repaint_after(std::time::Duration::from_millis(250));
        }
    }
}

// ── native ────────────────────────────────────────────────────────────────

/// Cookies the stations handed out this run, by station. Memory only.
#[cfg(not(target_arch = "wasm32"))]
static COOKIES: std::sync::LazyLock<std::sync::Mutex<std::collections::BTreeMap<String, String>>> =
    std::sync::LazyLock::new(Default::default);

#[cfg(not(target_arch = "wasm32"))]
fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_connect(Some(std::time::Duration::from_secs(6)))
        .timeout_global(Some(std::time::Duration::from_secs(12)))
        .http_status_as_error(false)
        .build()
        .into()
}

/// Which question a reply answers: the relay's card, or the station's record.
#[derive(Clone, Copy)]
enum Kind {
    Info(u32),
    History(u32),
    Export,
    Watch,
    Programs,
    Stats,
    Switch,
    Weather(usize),
}

impl Kind {
    fn limit(self) -> u64 {
        match self {
            Kind::Info(_) => MAX_INFO_BYTES,
            Kind::History(_) => MAX_HISTORY_BYTES,
            Kind::Export => MAX_EXPORT_BYTES,
            Kind::Watch => MAX_WATCH_BYTES,
            Kind::Programs => MAX_PROGRAMS_BYTES,
            Kind::Stats => MAX_STATS_BYTES,
            Kind::Switch => MAX_SWITCH_BYTES,
            Kind::Weather(_) => MAX_WEATHER_BYTES,
        }
    }

    fn message(self, status: Option<u16>, body: String) -> Msg {
        match self {
            Kind::Info(icao) => Msg::Info { icao, status, body },
            Kind::History(icao) => Msg::History { icao, status, body },
            Kind::Export => Msg::Export { status, body },
            Kind::Watch => Msg::Watch { status, body },
            Kind::Programs => Msg::Programs { status, body },
            Kind::Stats => Msg::Stats { status, body },
            Kind::Switch => Msg::Switch { status, body },
            Kind::Weather(airport) => Msg::Weather { airport, status, body },
        }
    }
}

/// `GET` — or `POST` of a JSON `post` — with an optional cookie: the status and
/// the body, or `None` for no answer at all.
#[cfg(not(target_arch = "wasm32"))]
fn get_info(
    agent: &ureq::Agent,
    url: &str,
    cookie: Option<&str>,
    limit: u64,
    post: Option<&str>,
) -> Option<(u16, String)> {
    let mut resp = match post {
        None => {
            let mut req = agent.get(url);
            if let Some(c) = cookie {
                req = req.header("Cookie", c);
            }
            req.call().ok()?
        }
        Some(body) => {
            let mut req = agent
                .post(url)
                .header("content-type", "application/json")
                .header("x-sdroxide", "1");
            if let Some(c) = cookie {
                req = req.header("Cookie", c);
            }
            req.send(body.to_string()).ok()?
        }
    };
    let status = resp.status().as_u16();
    let body = resp.body_mut().with_config().limit(limit).read_to_string().unwrap_or_default();
    Some((status, body))
}

/// Trade the password this client already signed in with for the station's
/// cookie, the way a browser does.
#[cfg(not(target_arch = "wasm32"))]
fn sign_in(agent: &ureq::Agent, station: &str) -> Option<String> {
    let access = crate::login::session_access(station)?;
    let url = format!("{}/signin", crate::login::http_base(station)?);
    let body = serde_json::json!({
        "username": access.username,
        "password": access.password,
        "remember": false,
    })
    .to_string();
    let resp = agent.post(&url).header("content-type", "application/json").send(body).ok()?;
    if resp.status().as_u16() != 200 {
        return None;
    }
    let set = resp.headers().get("set-cookie")?.to_str().ok()?;
    Some(set.split(';').next()?.trim().to_string())
}

#[cfg(not(target_arch = "wasm32"))]
fn send_info(
    url: String,
    station: String,
    kind: Kind,
    post: Option<String>,
    tx: Sender<Msg>,
    ctx: egui::Context,
) {
    std::thread::spawn(move || {
        let agent = agent();
        let cookie = COOKIES.lock().unwrap_or_else(|e| e.into_inner()).get(&station).cloned();
        let mut got = get_info(&agent, &url, cookie.as_deref(), kind.limit(), post.as_deref());
        if got.as_ref().is_some_and(|(s, _)| *s == 401) {
            if let Some(c) = sign_in(&agent, &station) {
                COOKIES.lock().unwrap_or_else(|e| e.into_inner()).insert(station, c.clone());
                got = get_info(&agent, &url, Some(&c), kind.limit(), post.as_deref());
            }
        }
        let (status, body) = match got {
            Some((s, b)) => (Some(s), b),
            None => (None, String::new()),
        };
        let _ = tx.send(kind.message(status, body));
        ctx.request_repaint();
    });
}

/// Let the station's picture go. The native program fetches an aircraft's
/// picture from its own host (no CORS to meet), so nothing is held at the
/// station for it; an airport's is only ever the station's, so it is let go.
#[cfg(not(target_arch = "wasm32"))]
fn send_release(url: String, of: &ThumbOf, peer: Option<&str>) {
    if matches!(of, ThumbOf::Aircraft(_)) {
        return;
    }
    let station = peer.map(crate::login::station_key).unwrap_or_default();
    std::thread::spawn(move || {
        let agent = agent();
        let release = |cookie: Option<&str>| {
            let mut req = agent.delete(&url).header("x-sdroxide", "1");
            if let Some(c) = cookie {
                req = req.header("Cookie", c);
            }
            req.call().ok().map(|r| r.status().as_u16())
        };
        let cookie = COOKIES.lock().unwrap_or_else(|e| e.into_inner()).get(&station).cloned();
        if release(cookie.as_deref()) == Some(401) {
            if let Some(c) = sign_in(&agent, &station) {
                COOKIES.lock().unwrap_or_else(|e| e.into_inner()).insert(station, c.clone());
                release(Some(&c));
            }
        }
    });
}

/// `GET` of a picture from the station, with its cookie: the status and the
/// bytes, or `None` for no answer at all.
#[cfg(not(target_arch = "wasm32"))]
fn get_bytes(agent: &ureq::Agent, url: &str, cookie: Option<&str>) -> Option<(u16, Vec<u8>)> {
    let mut req = agent.get(url);
    if let Some(c) = cookie {
        req = req.header("Cookie", c);
    }
    let mut resp = req.call().ok()?;
    let status = resp.status().as_u16();
    let body =
        resp.body_mut().with_config().limit(MAX_THUMB_BYTES).read_to_vec().unwrap_or_default();
    Some((status, body))
}

/// An aircraft's picture is fetched from its host; an airport's from the
/// station, signing in the way the other questions do.
#[cfg(not(target_arch = "wasm32"))]
fn send_thumb(key: String, of: &ThumbOf, peer: Option<&str>, tx: Sender<Msg>, ctx: egui::Context) {
    let via = of.fetch_url(peer);
    let direct = matches!(of, ThumbOf::Aircraft(_));
    let station = peer.map(crate::login::station_key).unwrap_or_default();
    std::thread::spawn(move || {
        let agent = agent();
        let mut missing = false;
        let bytes = if direct {
            agent.get(&key).call().ok().filter(|r| r.status().as_u16() == 200).and_then(|mut r| {
                r.body_mut().with_config().limit(MAX_THUMB_BYTES).read_to_vec().ok()
            })
        } else {
            via.and_then(|url| {
                let cookie =
                    COOKIES.lock().unwrap_or_else(|e| e.into_inner()).get(&station).cloned();
                let mut got = get_bytes(&agent, &url, cookie.as_deref());
                if got.as_ref().is_some_and(|(s, _)| *s == 401) {
                    if let Some(c) = sign_in(&agent, &station) {
                        COOKIES
                            .lock()
                            .unwrap_or_else(|e| e.into_inner())
                            .insert(station, c.clone());
                        got = get_bytes(&agent, &url, Some(&c));
                    }
                }
                missing = got.as_ref().is_some_and(|(s, _)| *s == 404);
                got.filter(|(s, _)| *s == 200).map(|(_, b)| b)
            })
        };
        let _ = tx.send(Msg::Thumb { url: key, bytes, missing });
        ctx.request_repaint();
    });
}

// ── browser ───────────────────────────────────────────────────────────────

/// `fetch` and hand back the response. `info` is the station's own relay,
/// same-origin with its cookie; otherwise it is the picture host, which gets
/// neither a cookie nor a referrer, and which the browser may refuse for want
/// of CORS headers — that is a failed picture, not an error.
#[cfg(target_arch = "wasm32")]
async fn fetch(url: &str, info: bool, post: Option<&str>) -> Option<web_sys::Response> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window()?;
    let init = web_sys::RequestInit::new();
    match post {
        None => init.set_method("GET"),
        Some(body) => {
            init.set_method("POST");
            let headers = web_sys::Headers::new().ok()?;
            headers.set("content-type", "application/json").ok()?;
            headers.set("x-sdroxide", "1").ok()?;
            init.set_headers(&headers);
            init.set_body(&wasm_bindgen::JsValue::from_str(body));
        }
    }
    if info {
        init.set_mode(web_sys::RequestMode::SameOrigin);
    } else {
        init.set_mode(web_sys::RequestMode::Cors);
        init.set_credentials(web_sys::RequestCredentials::Omit);
        init.set_referrer_policy(web_sys::ReferrerPolicy::NoReferrer);
    }
    let request = web_sys::Request::new_with_str_and_init(url, &init).ok()?;
    let response =
        wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await.ok()?;
    response.dyn_into::<web_sys::Response>().ok()
}

#[cfg(target_arch = "wasm32")]
fn send_info(
    url: String,
    _station: String,
    kind: Kind,
    post: Option<String>,
    tx: Sender<Msg>,
    ctx: egui::Context,
) {
    wasm_bindgen_futures::spawn_local(async move {
        let (status, body) = match fetch(&url, true, post.as_deref()).await {
            Some(r) => {
                let status = r.status();
                let text = match r.text() {
                    Ok(p) => wasm_bindgen_futures::JsFuture::from(p).await.ok(),
                    Err(_) => None,
                };
                (Some(status), text.and_then(|t| t.as_string()).unwrap_or_default())
            }
            None => (None, String::new()),
        };
        let body = if body.len() as u64 > kind.limit() { String::new() } else { body };
        let _ = tx.send(kind.message(status, body));
        ctx.request_repaint();
    });
}

/// Let the station's picture go. `keepalive` lets it finish even if the page is
/// going away; the answer is not read.
#[cfg(target_arch = "wasm32")]
fn send_release(url: String, _of: &ThumbOf, _peer: Option<&str>) {
    wasm_bindgen_futures::spawn_local(async move {
        let Some(window) = web_sys::window() else { return };
        let init = web_sys::RequestInit::new();
        init.set_method("DELETE");
        init.set_mode(web_sys::RequestMode::SameOrigin);
        let Ok(headers) = web_sys::Headers::new() else { return };
        let _ = headers.set("x-sdroxide", "1");
        init.set_headers(&headers);
        let _ = js_sys::Reflect::set(&init, &"keepalive".into(), &true.into());
        if let Ok(request) = web_sys::Request::new_with_str_and_init(&url, &init) {
            let _ = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await;
        }
    });
}

/// The browser asks the station for the picture, same-origin with its cookie:
/// Planespotters' own host would not let the page read it. No station, no picture.
#[cfg(target_arch = "wasm32")]
fn send_thumb(url: String, of: &ThumbOf, peer: Option<&str>, tx: Sender<Msg>, ctx: egui::Context) {
    let via = of.fetch_url(peer);
    wasm_bindgen_futures::spawn_local(async move {
        let mut missing = false;
        let bytes = async {
            let r = fetch(via.as_deref()?, true, None).await?;
            if r.status() != 200 {
                missing = r.status() == 404;
                return None;
            }
            let buf = wasm_bindgen_futures::JsFuture::from(r.array_buffer().ok()?).await.ok()?;
            let bytes = js_sys::Uint8Array::new(&buf).to_vec();
            (bytes.len() as u64 <= MAX_THUMB_BYTES).then_some(bytes)
        }
        .await;
        let _ = tx.send(Msg::Thumb { url, bytes, missing });
        ctx.request_repaint();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png(w: u32, h: u32) -> Vec<u8> {
        let img = image::RgbaImage::from_pixel(w, h, image::Rgba([10, 200, 30, 255]));
        let mut out = std::io::Cursor::new(Vec::new());
        img.write_to(&mut out, image::ImageFormat::Png).unwrap();
        out.into_inner()
    }

    #[test]
    fn a_thumbnail_is_decoded_from_memory_to_pixels() {
        let ci = decode_thumb(&png(4, 3)).expect("a PNG decodes");
        assert_eq!(ci.size, [4, 3]);
        assert_eq!(ci.pixels[0], egui::Color32::from_rgb(10, 200, 30));
    }

    #[test]
    fn anything_that_is_not_a_small_picture_is_no_picture() {
        assert!(decode_thumb(&[]).is_none());
        assert!(decode_thumb(b"<html>not a picture</html>").is_none());
        assert!(decode_thumb(&png(MAX_THUMB_SIDE + 1, 1)).is_none(), "too wide");
        assert!(decode_thumb(&vec![0u8; MAX_THUMB_BYTES as usize + 1]).is_none(), "too big");
    }

    #[test]
    fn a_thumbnail_that_failed_is_not_asked_for_again() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        let url = "https://t.plnspttrs.net/a_t.jpg";
        c.want_thumb(&ctx, None, 7, url);
        assert_eq!(c.thumb_status(url), ThumbStatus::Loading);
        c.tx.send(Msg::Thumb { url: url.into(), bytes: Some(b"nope".to_vec()), missing: false })
            .unwrap();
        c.poll(&ctx, 0);
        assert_eq!(c.thumb_status(url), ThumbStatus::Failed);
        c.want_thumb(&ctx, None, 7, url);
        assert_eq!(c.thumb_status(url), ThumbStatus::Failed, "not retried at once");
        assert!(c.thumb_texture(url).is_none());
    }

    #[test]
    fn a_failed_thumbnail_is_asked_again_a_few_times_while_the_card_stays_open() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        let url = "https://t.plnspttrs.net/a_t.jpg";
        let at = |ctx: &egui::Context, t: f64| {
            ctx.begin_pass(egui::RawInput { time: Some(t), ..Default::default() });
            ctx.end_pass().drop_without_applying_deltas();
        };
        at(&ctx, 0.0);
        c.want_thumb(&ctx, None, 7, url);
        for (i, t) in [1.0, 6.0, 12.0].into_iter().enumerate() {
            at(&ctx, t);
            c.tx.send(Msg::Thumb { url: url.into(), bytes: None, missing: false }).unwrap();
            c.poll(&ctx, 0);
            assert_eq!(c.thumb_status(url), ThumbStatus::Failed, "try {}", i + 1);
            at(&ctx, t + 1.0);
            c.want_thumb(&ctx, None, 7, url);
            assert_eq!(c.thumb_status(url), ThumbStatus::Failed, "too soon after try {}", i + 1);
            at(&ctx, t + 5.0);
            c.want_thumb(&ctx, None, 7, url);
            let again = i + 1 < usize::from(aircraft_online::THUMB_TRIES);
            let want = if again { ThumbStatus::Loading } else { ThumbStatus::Failed };
            assert_eq!(c.thumb_status(url), want, "after try {}", i + 1);
        }
    }

    fn frame(ctx: &egui::Context) {
        ctx.begin_pass(egui::RawInput::default());
        ctx.end_pass().drop_without_applying_deltas();
    }

    #[test]
    fn closing_the_card_destroys_the_thumbnail_and_only_the_closing_does() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        let url = "https://t.plnspttrs.net/a_t.jpg";
        frame(&ctx);
        c.want_thumb(&ctx, None, 7, url);
        c.tx.send(Msg::Thumb { url: url.into(), bytes: Some(png(4, 3)), missing: false }).unwrap();
        c.poll(&ctx, 0);
        assert!(c.thumb_texture(url).is_some());
        // The card is open: it asks every frame, and the picture stays.
        for _ in 0..5 {
            c.release_thumbs(&ctx, Some("ws://shack.test:4950/ws"));
            c.want_thumb(&ctx, None, 7, url);
            frame(&ctx);
            assert!(c.thumb_texture(url).is_some(), "an open card keeps its picture");
        }
        // The card is closed: nothing asks, and it goes within two frames.
        c.release_thumbs(&ctx, Some("ws://shack.test:4950/ws"));
        frame(&ctx);
        c.release_thumbs(&ctx, Some("ws://shack.test:4950/ws"));
        frame(&ctx);
        c.release_thumbs(&ctx, Some("ws://shack.test:4950/ws"));
        assert!(c.thumb_texture(url).is_none(), "the picture is destroyed");
        assert_eq!(c.thumb_status(url), ThumbStatus::NotTried);
    }

    #[test]
    fn only_one_thumbnail_is_kept_and_the_next_replaces_it() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        let (a, b) = ("https://t.plnspttrs.net/a_t.jpg", "https://t.plnspttrs.net/b_t.jpg");
        c.want_thumb(&ctx, None, 1, a);
        c.want_thumb(&ctx, None, 2, b);
        assert_eq!(c.thumb_status(a), ThumbStatus::NotTried, "the first is gone");
        assert_eq!(c.thumb_status(b), ThumbStatus::Loading);
    }

    /// A station that is not there: a request to it fails at once. Wait for that
    /// answer, so it cannot land on top of what a test hands in.
    const NO_STATION: &str = "ws://127.0.0.1:1/ws";

    fn settled(c: &mut OnlineClient, ctx: &egui::Context, key: &str) {
        let t = std::time::Instant::now();
        while c.thumb_status(key) == ThumbStatus::Loading && t.elapsed().as_secs() < 5 {
            c.poll(ctx, 0);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    fn schiphol_key() -> String {
        airport_thumb_key("en", "Amsterdam_Airport_Schiphol")
    }

    #[test]
    fn an_airport_picture_is_asked_of_the_station_by_article_and_not_at_all_without_one() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        c.want_airport_thumb(&ctx, None, "en", "Amsterdam_Airport_Schiphol");
        assert_eq!(c.thumb_status(&schiphol_key()), ThumbStatus::NotTried, "no station, no ask");
        c.want_airport_thumb(&ctx, Some(NO_STATION), "en", "Amsterdam_Airport_Schiphol");
        assert_eq!(c.thumb_status(&schiphol_key()), ThumbStatus::Loading);
        assert!(c.thumb_pending(&schiphol_key()));
    }

    #[test]
    fn an_airport_card_gets_its_picture_as_pixels_and_the_room_kept_until_then() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        let ident = "EHAM";
        frame(&ctx);
        let (photo, page) =
            c.airport_card_photo(&ctx, Some(NO_STATION), ident).expect("an article");
        assert_eq!(photo, crate::adsb_map::CardPhoto::Pending, "room is kept while it loads");
        assert_eq!(page, "https://en.wikipedia.org/wiki/Amsterdam_Airport_Schiphol");
        settled(&mut c, &ctx, &schiphol_key());
        c.tx.send(Msg::Thumb { url: schiphol_key(), bytes: Some(png(4, 3)), missing: false })
            .unwrap();
        c.poll(&ctx, 0);
        match c.airport_card_photo(&ctx, Some(NO_STATION), ident).unwrap().0 {
            crate::adsb_map::CardPhoto::Ready { size, .. } => {
                assert_eq!(size, egui::vec2(4.0, 3.0))
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn an_airport_without_an_article_or_without_a_station_has_no_picture_to_ask_for() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        assert!(c.airport_card_photo(&ctx, Some(NO_STATION), "ZZZZ").is_none(), "no article");
        assert!(c.airport_card_photo(&ctx, None, "EHAM").is_none(), "no station");
        assert_eq!(c.thumb_status(&schiphol_key()), ThumbStatus::NotTried);
    }

    #[test]
    fn a_picture_that_never_came_leaves_the_card_as_text_once_the_tries_are_spent() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        let at = |ctx: &egui::Context, t: f64| {
            ctx.begin_pass(egui::RawInput { time: Some(t), ..Default::default() });
            ctx.end_pass().drop_without_applying_deltas();
        };
        at(&ctx, 0.0);
        c.want_airport_thumb(&ctx, Some(NO_STATION), "en", "Amsterdam_Airport_Schiphol");
        let key = schiphol_key();
        for (i, t) in [1.0, 6.0, 12.0].into_iter().enumerate() {
            at(&ctx, t);
            settled(&mut c, &ctx, &key);
            c.tx.send(Msg::Thumb { url: key.clone(), bytes: None, missing: false }).unwrap();
            c.poll(&ctx, 0);
            assert_eq!(c.thumb_status(&key), ThumbStatus::Failed, "try {}", i + 1);
            let more = i + 1 < usize::from(aircraft_online::THUMB_TRIES);
            assert_eq!(c.thumb_pending(&key), more, "room kept while tries remain: {}", i + 1);
            at(&ctx, t + 5.0);
            c.want_airport_thumb(&ctx, Some(NO_STATION), "en", "Amsterdam_Airport_Schiphol");
        }
        assert!(!c.thumb_pending(&key));
    }

    /// A 404 from the station says the page has no picture: the card goes back
    /// to text at once, with no "…" and none of the three retries a busy
    /// station gets.
    #[test]
    fn an_airport_page_without_a_picture_is_settled_at_the_first_answer() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        ctx.begin_pass(egui::RawInput { time: Some(0.0), ..Default::default() });
        ctx.end_pass().drop_without_applying_deltas();
        c.want_airport_thumb(&ctx, Some(NO_STATION), "en", "Amsterdam_Airport_Schiphol");
        let key = schiphol_key();
        settled(&mut c, &ctx, &key);
        c.tx.send(Msg::Thumb { url: key.clone(), bytes: None, missing: true }).unwrap();
        c.poll(&ctx, 0);
        assert!(!c.thumb_pending(&key), "no waiting frame for a picture that does not exist");
        assert_eq!(c.thumb_status(&key), ThumbStatus::Failed);
    }

    const BOX: usize = 3;
    const PLACE: (f64, f64) = (52.31, 4.76);

    fn clock(
        c: &mut OnlineClient,
        ctx: &egui::Context,
        peer: Option<&str>,
    ) -> aircraft_online::Clock {
        frame(ctx);
        c.airport_clock(ctx, peer, BOX, PLACE)
    }

    /// Wait for the question that went out to nobody to come back as a failure.
    fn weather_settled(c: &mut OnlineClient, ctx: &egui::Context) {
        let t = std::time::Instant::now();
        while !c.weather.needs_a_later_look() && t.elapsed().as_secs() < 5 {
            c.poll(ctx, 0);
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
    }

    fn answer(c: &mut OnlineClient, ctx: &egui::Context, body: &str) {
        let status = Some(200);
        c.tx.send(Msg::Weather { airport: BOX, status, body: body.into() }).unwrap();
        c.poll(ctx, 0);
    }

    const SUNNY: &str = r#"{"v":1,"utc_offset_s":3600,"temp_c":11.6}"#;

    #[test]
    fn an_airport_clock_is_not_asked_without_a_station_or_with_the_relay_off() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        assert_eq!(clock(&mut c, &ctx, None), aircraft_online::Clock::Offline);
        assert!(!c.weather.needs_a_later_look(), "nothing was asked");
        c.switch_state = Some(false);
        assert_eq!(clock(&mut c, &ctx, Some(NO_STATION)), aircraft_online::Clock::Offline);
        assert!(!c.weather.needs_a_later_look(), "the relay is off: nothing was asked");
    }

    #[test]
    fn an_open_box_asks_the_station_for_its_airports_place_and_reads_the_answer() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        assert_eq!(clock(&mut c, &ctx, Some(NO_STATION)), aircraft_online::Clock::Asking);
        weather_settled(&mut c, &ctx); // nobody answers at that address: a failure, and a repeat is due
        assert_eq!(clock(&mut c, &ctx, Some(NO_STATION)), aircraft_online::Clock::Asking);
        answer(&mut c, &ctx, SUNNY);
        match clock(&mut c, &ctx, Some(NO_STATION)) {
            aircraft_online::Clock::Known(w) => {
                assert_eq!(w.utc_offset_s, 3600);
                assert!((w.temp_c.unwrap() - 11.6).abs() < 0.01);
            }
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn an_answer_that_is_not_a_forecast_leaves_the_box_on_utc() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        clock(&mut c, &ctx, Some(NO_STATION));
        weather_settled(&mut c, &ctx);
        answer(&mut c, &ctx, "<html>");
        assert_eq!(clock(&mut c, &ctx, Some(NO_STATION)), aircraft_online::Clock::Offline);
    }

    #[test]
    fn closing_the_box_lets_the_clock_go_and_the_next_box_starts_clean() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        clock(&mut c, &ctx, Some(NO_STATION));
        weather_settled(&mut c, &ctx);
        answer(&mut c, &ctx, SUNNY);
        assert!(matches!(c.weather.clock(BOX), aircraft_online::Clock::Known(_)));
        // Two frames in which no box asks.
        for _ in 0..3 {
            frame(&ctx);
            c.release_thumbs(&ctx, Some(NO_STATION));
        }
        assert_eq!(c.weather.clock(BOX), aircraft_online::Clock::Offline, "nothing kept");
    }

    #[test]
    fn an_airport_and_an_aircraft_picture_live_side_by_side_and_each_goes_with_its_own_card() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        let plane = "https://t.plnspttrs.net/a_t.jpg";
        let key = schiphol_key();
        frame(&ctx);
        c.want_thumb(&ctx, None, 7, plane);
        c.want_airport_thumb(&ctx, Some(NO_STATION), "en", "Amsterdam_Airport_Schiphol");
        assert_eq!(c.thumb_status(plane), ThumbStatus::Loading, "the airport did not evict it");
        assert_eq!(c.thumb_status(&key), ThumbStatus::Loading);
        settled(&mut c, &ctx, &key);
        c.tx.send(Msg::Thumb { url: plane.into(), bytes: Some(png(4, 3)), missing: false })
            .unwrap();
        c.tx.send(Msg::Thumb { url: key.clone(), bytes: Some(png(4, 3)), missing: false }).unwrap();
        c.poll(&ctx, 0);
        // The aircraft card is closed; the airport box stays open.
        for _ in 0..3 {
            frame(&ctx);
            c.want_airport_thumb(&ctx, Some(NO_STATION), "en", "Amsterdam_Airport_Schiphol");
            c.release_thumbs(&ctx, Some(NO_STATION));
        }
        assert!(c.thumb_texture(plane).is_none(), "the closed card's picture is destroyed");
        assert!(c.thumb_texture(&key).is_some(), "the open box keeps its own");
        // Now the box closes too.
        for _ in 0..3 {
            frame(&ctx);
            c.release_thumbs(&ctx, Some(NO_STATION));
        }
        assert!(c.thumb_texture(&key).is_none(), "the picture is destroyed with its box");
        assert_eq!(c.thumb_status(&key), ThumbStatus::NotTried);
    }

    #[test]
    fn only_one_airport_picture_is_kept_and_the_next_replaces_it() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        c.want_airport_thumb(&ctx, Some(NO_STATION), "en", "Amsterdam_Airport_Schiphol");
        c.want_airport_thumb(&ctx, Some(NO_STATION), "fr", "Orly");
        assert_eq!(c.thumb_status(&schiphol_key()), ThumbStatus::NotTried, "the first is gone");
        assert_eq!(c.thumb_status(&airport_thumb_key("fr", "Orly")), ThumbStatus::Loading);
    }

    #[test]
    fn a_thumbnail_address_that_is_not_planespotters_is_never_requested() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        c.want_thumb(&ctx, None, 7, "https://evil.example/a.jpg");
        assert_eq!(c.thumb_status("https://evil.example/a.jpg"), ThumbStatus::NotTried);
    }

    #[test]
    fn a_reply_from_the_station_reaches_the_store() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        let w = Want { icao: 7, callsign: None, registration: None };
        assert!(c.store.next_request(Some(&w), true, 0).is_some());
        let body = serde_json::to_string(&sdroxide_aircraft_online::Info::bare(
            sdroxide_aircraft_online::Status::Disabled,
        ))
        .unwrap();
        c.tx.send(Msg::Info { icao: 7, status: Some(503), body }).unwrap();
        c.poll(&ctx, 1);
        assert_eq!(
            c.store.state_for(7, true),
            &super::super::aircraft_online::OnlineState::Unavailable(
                super::super::aircraft_online::Reason::Disabled
            )
        );
    }

    #[test]
    fn the_switch_reads_on_by_default_and_follows_what_the_station_answers() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        assert!(c.relay_on(), "on by default, before anything was asked");
        let reply = |c: &mut OnlineClient, on: bool| {
            c.tx.send(Msg::Switch {
                status: Some(200),
                body: format!(r#"{{"enabled":{on},"forced":false}}"#),
            })
            .unwrap();
            c.poll(&ctx, 1);
        };
        c.switch_asked = Some(false);
        reply(&mut c, false);
        assert!(!c.relay_on(), "the operator turned it off");
        assert_eq!(c.switch_note, None);
        c.switch_asked = Some(true);
        reply(&mut c, true);
        assert!(c.relay_on(), "turned on again");
    }

    #[test]
    fn turning_the_relay_on_again_lets_the_cards_ask_after_it_was_off() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        let w = Want { icao: 7, callsign: None, registration: None };
        assert!(c.store.next_request(Some(&w), true, 0).is_some());
        c.store.deliver(
            7,
            super::super::aircraft_online::Answer::Unavailable(
                super::super::aircraft_online::Reason::Disabled,
            ),
            1,
        );
        assert!(!c.relay_on(), "a card was refused: the relay is off");
        assert!(c.store.next_request(Some(&w), true, 2).is_none());
        c.switch_asked = Some(true);
        c.tx.send(Msg::Switch {
            status: Some(200),
            body: r#"{"enabled":true,"forced":false}"#.into(),
        })
        .unwrap();
        c.poll(&ctx, 3);
        assert!(c.relay_on());
        assert!(c.store.next_request(Some(&w), true, 4).is_some(), "the lock is lifted");
    }

    #[test]
    fn a_relay_the_environment_holds_says_so_instead_of_pretending() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        c.switch_asked = Some(true);
        c.tx.send(Msg::Switch {
            status: Some(200),
            body: r#"{"enabled":false,"forced":true}"#.into(),
        })
        .unwrap();
        c.poll(&ctx, 1);
        assert!(!c.relay_on(), "what the station says, not what was asked");
        assert!(c.switch_note.as_deref().is_some_and(|n| n.contains("SDROXIDE_AIRCRAFT_ONLINE")));
    }

    /// **This test cannot catch a regression of the `peer.is_some()` guard.**
    /// Without the guard the store would send the question, `info_url` would
    /// find no address to build it from, and `ask` would answer `NoStation` at
    /// once — nothing sent and the same state drawn, which is all this checks.
    /// It pins that outcome, not the path to it; the guard is only a shortcut
    /// past a request that could not have been built anyway.
    #[test]
    fn a_station_the_client_is_not_connected_to_is_not_asked_at_all() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        let w = Want { icao: 9, callsign: None, registration: None };
        c.ask(&ctx, None, Some(&w), 0);
        assert!(c.rx.try_recv().is_err(), "nothing was sent");
        assert_eq!(
            c.store.state_for(9, false),
            &super::super::aircraft_online::OnlineState::Unavailable(
                super::super::aircraft_online::Reason::NoStation
            )
        );
    }

    #[test]
    fn a_history_reply_from_the_station_reaches_the_history_store() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        c.history.set_on(Some(7));
        assert_eq!(c.history.next_request(true, 0), Some(7));
        let body = serde_json::to_string(&sdroxide_aircraft_history::AircraftReply::bare(
            sdroxide_aircraft_history::Status::Disabled,
        ))
        .unwrap();
        c.tx.send(Msg::History { icao: 7, status: Some(503), body }).unwrap();
        c.poll(&ctx, 1);
        assert_eq!(
            c.history.state(7),
            &aircraft_history::HistoryState::Unavailable(aircraft_history::Reason::Disabled)
        );
        assert!(!c.history.line_busy(1), "the answer freed the line");
        // The relay's store is a different thing and was not touched.
        assert_eq!(c.store.len(), 0);
    }

    #[test]
    fn history_is_not_asked_until_it_is_on_and_there_is_a_station() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        c.ask_history(&ctx, Some("wss://shack.example/ws"), 0);
        assert!(c.rx.try_recv().is_err(), "nothing on, nothing sent");
        assert!(!c.history.line_busy(0));
        c.history.set_on(Some(9));
        c.ask_history(&ctx, None, 0);
        assert!(!c.history.line_busy(0), "no station, nothing sent");
        // The control: on, with a station, a request is out.
        c.ask_history(&ctx, Some("wss://shack.example/ws"), 0);
        assert!(c.history.line_busy(0), "a request went");
    }

    #[test]
    fn a_connection_that_is_not_an_address_is_told_no_station_without_a_request() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        c.history.set_on(Some(9));
        c.ask_history(&ctx, Some("not an address"), 0);
        assert!(c.rx.try_recv().is_err());
        assert_eq!(
            c.history.state(9),
            &aircraft_history::HistoryState::Unavailable(aircraft_history::Reason::NoStation)
        );
    }

    #[test]
    fn the_stations_table_reaches_the_export_store() {
        use super::super::aircraft_export::Ready;
        use sdroxide_aircraft_history::export::Format;
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        c.export.start(Format::Csv);
        assert_eq!(c.export.next_fetch(true, 0), Some(Format::Csv));
        let csv = "icao,callsign,registration\n";
        c.tx.send(Msg::Export { status: Some(200), body: csv.into() }).unwrap();
        c.poll(&ctx, 1);
        assert_eq!(c.export.take_ready(), Some(Ready::Station(Format::Csv, csv.into())));
        assert_eq!(c.store.len(), 0, "the relay's store was not touched");
    }

    #[test]
    fn the_stations_list_reaches_the_watch_store_and_frees_the_line() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        assert!(c.watch.next_request(true, 0).is_some());
        let body = serde_json::to_string(&sdroxide_aircraft_history::watch::WatchReply::of(
            &sdroxide_aircraft_history::watch::Watchlist::default(),
            None,
        ))
        .unwrap();
        c.tx.send(Msg::Watch { status: Some(200), body }).unwrap();
        c.poll(&ctx, 1);
        assert_eq!(*c.watch.state(), aircraft_watch::WatchState::Ready);
        assert!(!c.watch.line_busy(1));
        assert_eq!(c.store.len(), 0, "the relay's store was not touched");
    }

    #[test]
    fn an_export_is_not_asked_without_a_station_and_the_file_is_made_here() {
        use super::super::aircraft_export::Ready;
        use sdroxide_aircraft_history::export::Format;
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        c.ask_export(&ctx, Some("wss://shack.example/ws"), 0);
        assert!(c.rx.try_recv().is_err() && !c.export.busy(), "nothing pressed, nothing sent");
        c.export.start(Format::Json);
        c.ask_export(&ctx, None, 0);
        assert_eq!(
            c.export.take_ready(),
            Some(Ready::Local(Format::Json, aircraft_history::Reason::NoStation))
        );
        // The control: with a station a request goes and the store waits for it.
        c.export.start(Format::Json);
        c.ask_export(&ctx, Some("wss://shack.example/ws"), 0);
        assert!(c.export.busy() && c.export.take_ready().is_none());
    }

    #[test]
    fn a_connection_that_is_not_an_address_makes_the_file_here() {
        use super::super::aircraft_export::Ready;
        use sdroxide_aircraft_history::export::Format;
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        c.export.start(Format::Csv);
        c.ask_export(&ctx, Some("not an address"), 0);
        assert!(c.rx.try_recv().is_err());
        assert_eq!(
            c.export.take_ready(),
            Some(Ready::Local(Format::Csv, aircraft_history::Reason::NoStation))
        );
    }

    #[test]
    fn the_watch_list_is_not_asked_without_a_station_and_an_edit_waits_for_one() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        c.ask_watch(&ctx, None, 0);
        assert!(!c.watch.line_busy(0), "no station, nothing sent");
        assert!(c.watch.request_edit(sdroxide_aircraft_history::watch::WatchEdit {
            add: Some("KLM1234".into()),
            remove: None
        }));
        c.ask_watch(&ctx, None, 0);
        assert_eq!(c.watch.queued(), 1, "still waiting");
        c.ask_watch(&ctx, Some("wss://shack.example/ws"), 0);
        assert!(c.watch.line_busy(0), "the control: with a station, the edit went");
        assert_eq!(c.watch.queued(), 0);
    }

    #[test]
    fn a_connection_that_is_not_an_address_is_told_no_station_for_the_watch_list() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        c.ask_watch(&ctx, Some("not an address"), 0);
        assert!(c.rx.try_recv().is_err());
        assert_eq!(
            *c.watch.state(),
            aircraft_watch::WatchState::Unavailable(aircraft_history::Reason::NoStation)
        );
    }

    #[test]
    fn a_list_from_the_station_reaches_the_stats_store_and_frees_the_line() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        c.stats.set_on(true);
        assert!(c.stats.next_request(true, 0));
        let body = serde_json::to_string(&sdroxide_aircraft_history::ListReply::bare(
            sdroxide_aircraft_history::Status::Disabled,
        ))
        .unwrap();
        c.tx.send(Msg::Stats { status: Some(503), body }).unwrap();
        c.poll(&ctx, 1);
        assert_eq!(
            c.stats.state(),
            &aircraft_stats::StatsState::Unavailable(aircraft_history::Reason::Disabled)
        );
        assert!(!c.stats.line_busy(1));
        assert_eq!(c.store.len(), 0, "the relay's store was not touched");
        assert_eq!(c.history.on(), None, "nor the history's");
    }

    #[test]
    fn the_list_is_not_asked_until_the_view_is_open_and_there_is_a_station() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        c.ask_stats(&ctx, Some("wss://shack.example/ws"), 0);
        assert!(!c.stats.line_busy(0), "closed, nothing sent");
        c.stats.set_on(true);
        c.ask_stats(&ctx, None, 0);
        assert!(!c.stats.line_busy(0), "no station, nothing sent");
        c.ask_stats(&ctx, Some("wss://shack.example/ws"), 0);
        assert!(c.stats.line_busy(0), "the control: open, with a station, a request went");
    }

    #[test]
    fn a_connection_that_is_not_an_address_is_told_no_station_for_the_list() {
        let ctx = egui::Context::default();
        let mut c = OnlineClient::default();
        c.stats.set_on(true);
        c.ask_stats(&ctx, Some("not an address"), 0);
        assert!(c.rx.try_recv().is_err());
        assert_eq!(
            c.stats.state(),
            &aircraft_stats::StatsState::Unavailable(aircraft_history::Reason::NoStation)
        );
    }
}
