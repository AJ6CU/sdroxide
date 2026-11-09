//! What the station has heard of each aircraft in the last day.
//!
//! A pure model: no clock, no file, no socket. The server feeds it what the
//! radio decoded and what the relay answered, writes its JSON beside the rest
//! of the config, and serves it on `/aircraft-history`; the client reads the
//! same types. Nothing here is asked of anybody — it only remembers what has
//! already arrived.
//!
//! What it keeps is deliberately narrow: positions, the callsigns and
//! registrations an address used, and the card the relay gave. Never audio, raw
//! frames, a picture, or anything about who was looking.

pub mod export;
pub mod kml;
#[cfg(feature = "gzip")]
pub mod gz;
pub mod watch;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// The version of the file and of the replies. A file of another version is
/// started afresh rather than guessed at.
pub const FORMAT_VERSION: u32 = 1;
/// How long anything is kept after the aircraft was last heard.
pub const RETENTION_S: i64 = 24 * 3600;
/// Most points kept for one aircraft. The oldest go first. At one point every
/// ten seconds that is fifty minutes — longer than most flights stay in range.
pub const MAX_POINTS_PER_AIRCRAFT: usize = 300;
/// Most points kept in all. The aircraft heard least recently go first. With
/// the ceiling on aircraft below, the file stays a few megabytes at most: a
/// test fills a history to its ceilings and measures it.
pub const MAX_POINTS_TOTAL: usize = 50_000;
/// Most aircraft kept, with or without points.
pub const MAX_AIRCRAFT: usize = 4000;
/// A point is kept at most this often, unless the aircraft has moved far.
pub const MIN_GAP_S: i64 = 10;
/// How far it must have moved to be kept after [`MIN_GAP_S`].
pub const MIN_MOVE_M: f64 = 100.0;
/// A point is kept after this long whether or not it moved.
pub const HEARTBEAT_S: i64 = 60;
/// A position older than this when it arrives is not a position of now.
pub const STALE_FIX_S: i64 = 120;

/// The ceilings, so a test can use small ones.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    pub retention_s: i64,
    pub per_aircraft: usize,
    pub total_points: usize,
    pub aircraft: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            retention_s: RETENTION_S,
            per_aircraft: MAX_POINTS_PER_AIRCRAFT,
            total_points: MAX_POINTS_TOTAL,
            aircraft: MAX_AIRCRAFT,
        }
    }
}

/// One position: unix seconds, latitude, longitude, altitude in feet, on the
/// ground. Written as a bare array, which is a third of the size of an object.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Point(pub i64, pub f32, pub f32, pub Option<i32>, pub bool);

impl Point {
    pub fn time(&self) -> i64 {
        self.0
    }
    pub fn lat(&self) -> f32 {
        self.1
    }
    pub fn lon(&self) -> f32 {
        self.2
    }
    pub fn altitude_ft(&self) -> Option<i32> {
        self.3
    }
    pub fn on_ground(&self) -> bool {
        self.4
    }
}

/// A position as it arrives, before it is judged.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Fix {
    pub at: i64,
    pub lat: f64,
    pub lon: f64,
    pub altitude_ft: Option<i32>,
    pub on_ground: bool,
}

/// A callsign or registration an address used, and when.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Seen {
    pub text: String,
    pub first: i64,
    pub last: i64,
}

/// An airport as the card names it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Place {
    pub code: String,
    pub name: String,
}

/// The card the relay gave for an aircraft: the airframe, the route, and the
/// page and credit of its photo. There is no place in it for a picture or its
/// address.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct SeenCard {
    /// When the relay's answer was obtained.
    pub at: i64,
    pub type_name: Option<String>,
    pub manufacturer: Option<String>,
    pub operator: Option<String>,
    pub registration: Option<String>,
    pub origin: Option<Place>,
    pub destination: Option<Place>,
    pub photo_page: Option<String>,
    pub photo_credit: Option<String>,
}

/// Everything kept for one address.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Journal {
    pub icao: u32,
    pub first_seen: i64,
    /// Last time the radio heard it; `0` for an aircraft known only by a card.
    pub last_seen: i64,
    /// Last time anything was added: what retention counts from.
    pub touched: i64,
    pub callsigns: Vec<Seen>,
    pub registrations: Vec<Seen>,
    pub category: Option<String>,
    pub points: Vec<Point>,
    pub card: Option<SeenCard>,
}

impl Journal {
    /// The callsign used most recently.
    pub fn callsign(&self) -> Option<&str> {
        self.callsigns.iter().max_by_key(|s| s.last).map(|s| s.text.as_str())
    }

    /// The registration used most recently.
    pub fn registration(&self) -> Option<&str> {
        self.registrations.iter().max_by_key(|s| s.last).map(|s| s.text.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Ok,
    Disabled,
    Unauthorized,
    Invalid,
}

/// One line of the list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Summary {
    pub icao: u32,
    pub callsign: Option<String>,
    pub registration: Option<String>,
    pub first_seen: i64,
    pub last_seen: i64,
    pub points: usize,
    /// The ADS-B emitter category the radio reported, as the decoder wrote it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// From the relay's card, when it gave one: the airframe, the operator and
    /// the airport codes of the route. The statistics read these.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
}

/// The answer to `GET /aircraft-history`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListReply {
    pub v: u32,
    pub status: Status,
    pub aircraft: Vec<Summary>,
}

/// The answer to `GET /aircraft-history/{icao}`. An address never heard is
/// `Ok` with no journal: nothing to show is not an error.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AircraftReply {
    pub v: u32,
    pub status: Status,
    pub journal: Option<Journal>,
}

impl ListReply {
    pub fn bare(status: Status) -> ListReply {
        ListReply { v: FORMAT_VERSION, status, aircraft: Vec::new() }
    }
}

impl AircraftReply {
    pub fn bare(status: Status) -> AircraftReply {
        AircraftReply { v: FORMAT_VERSION, status, journal: None }
    }
}

/// The station's memory of the aircraft it has heard.
#[derive(Debug, Default)]
pub struct History {
    limits: Option<Limits>,
    by_icao: HashMap<u32, Journal>,
    dirty: bool,
}

impl History {
    pub fn new() -> History {
        History::default()
    }

    pub fn with_limits(limits: Limits) -> History {
        History { limits: Some(limits), ..History::default() }
    }

    fn limits(&self) -> Limits {
        self.limits.unwrap_or_default()
    }

    /// Number of aircraft kept.
    pub fn len(&self) -> usize {
        self.by_icao.len()
    }

    pub fn is_empty(&self) -> bool {
        self.by_icao.is_empty()
    }

    /// Points kept, all aircraft together.
    pub fn point_count(&self) -> usize {
        self.by_icao.values().map(|j| j.points.len()).sum()
    }

    pub fn journal(&self, icao: u32) -> Option<&Journal> {
        self.by_icao.get(&(icao & 0xff_ffff))
    }

    /// Whether anything changed since the last call, and forget it.
    pub fn take_dirty(&mut self) -> bool {
        std::mem::take(&mut self.dirty)
    }

    /// The radio heard this aircraft at `now`: its callsign and category if it
    /// said them, and its position if it sent one.
    pub fn note_heard(
        &mut self,
        now: i64,
        icao: u32,
        callsign: Option<&str>,
        category: Option<&str>,
        fix: Option<Fix>,
    ) {
        let (per, key) = (self.limits().per_aircraft, icao & ADDRESS_MASK);
        let j = self.by_icao.entry(key).or_insert_with(|| Journal::new(key, now));
        let mut changed = j.touch(now, true);
        if let Some(text) = callsign.and_then(upper) {
            changed |= upsert(&mut j.callsigns, text, now);
        }
        if let Some(c) = category.map(str::trim).filter(|c| !c.is_empty()) {
            if j.category.as_deref() != Some(c) {
                j.category = Some(c.to_string());
                changed = true;
            }
        }
        if let Some(f) = fix.filter(|f| good_fix(now, f)) {
            changed |= add_point(j, &f, per);
        }
        self.dirty |= changed;
    }

    /// A position, judged against `now`.
    pub fn note_fix(&mut self, now: i64, icao: u32, fix: Fix) {
        if !good_fix(now, &fix) {
            return;
        }
        let (per, key) = (self.limits().per_aircraft, icao & ADDRESS_MASK);
        let j = self.by_icao.entry(key).or_insert_with(|| Journal::new(key, now));
        let changed = j.touch(now, true) | add_point(j, &fix, per);
        self.dirty |= changed;
    }

    /// A message dated `at` gave this aircraft's callsign or registration. Safe
    /// to repeat: a message already counted changes nothing.
    pub fn note_identity(
        &mut self,
        at: i64,
        icao: u32,
        callsign: Option<&str>,
        registration: Option<&str>,
    ) {
        let (callsign, registration) = (callsign.and_then(upper), registration.and_then(upper));
        if callsign.is_none() && registration.is_none() {
            return;
        }
        let key = icao & ADDRESS_MASK;
        let fresh = !self.by_icao.contains_key(&key);
        let j = self.by_icao.entry(key).or_insert_with(|| Journal::new(key, at));
        let mut changed = fresh | j.touch(at, true);
        if let Some(text) = callsign {
            changed |= upsert(&mut j.callsigns, text, at);
        }
        if let Some(text) = registration {
            changed |= upsert(&mut j.registrations, text, at);
        }
        self.dirty |= changed;
    }

    /// The relay's card for an aircraft, obtained at `card.at`.
    pub fn note_card(&mut self, now: i64, icao: u32, card: SeenCard) {
        let key = icao & ADDRESS_MASK;
        let j = self.by_icao.entry(key).or_insert_with(|| Journal::new(key, now));
        j.touch(now, false);
        j.card = Some(card);
        self.dirty = true;
    }

    /// Forget what is past its retention and bring the rest under the ceilings.
    pub fn prune(&mut self, now: i64) {
        let l = self.limits();
        let cutoff = now - l.retention_s;
        let size = |h: &History| {
            h.by_icao
                .values()
                .map(|j| {
                    1 + j.points.len()
                        + j.callsigns.len()
                        + j.registrations.len()
                        + usize::from(j.card.is_some())
                })
                .sum::<usize>()
        };
        let before = size(self);
        self.by_icao.retain(|_, j| j.touched >= cutoff);
        for j in self.by_icao.values_mut() {
            j.points.retain(|p| p.time() >= cutoff);
            let excess = j.points.len().saturating_sub(l.per_aircraft);
            j.points.drain(..excess);
            j.callsigns.retain(|s| s.last >= cutoff);
            j.registrations.retain(|s| s.last >= cutoff);
            if j.card.as_ref().is_some_and(|c| c.at < cutoff) {
                j.card = None;
            }
        }
        let mut points = self.point_count();
        if points > l.total_points || self.by_icao.len() > l.aircraft {
            let mut order: Vec<(i64, u32)> =
                self.by_icao.values().map(|j| (j.touched, j.icao)).collect();
            order.sort_unstable();
            for (_, icao) in order {
                if points <= l.total_points && self.by_icao.len() <= l.aircraft {
                    break;
                }
                if let Some(j) = self.by_icao.remove(&icao) {
                    points -= j.points.len();
                }
            }
        }
        if size(self) != before {
            self.dirty = true;
        }
    }

    /// One line per aircraft, the one touched most recently first.
    pub fn summaries(&self) -> Vec<Summary> {
        let mut v: Vec<&Journal> = self.by_icao.values().collect();
        v.sort_by(|a, b| b.touched.cmp(&a.touched).then(a.icao.cmp(&b.icao)));
        v.into_iter()
            .map(|j| Summary {
                icao: j.icao,
                callsign: j.callsign().map(str::to_string),
                registration: j.registration().map(str::to_string),
                first_seen: j.first_seen,
                last_seen: j.last_seen,
                points: j.points.len(),
                category: j.category.clone(),
                type_name: j.card.as_ref().and_then(|c| c.type_name.clone()),
                operator: j.card.as_ref().and_then(|c| c.operator.clone()),
                origin: j.card.as_ref().and_then(|c| c.origin.as_ref()).map(|p| p.code.clone()),
                destination: j
                    .card
                    .as_ref()
                    .and_then(|c| c.destination.as_ref())
                    .map(|p| p.code.clone()),
            })
            .collect()
    }

    pub fn list_reply(&self) -> ListReply {
        ListReply { v: FORMAT_VERSION, status: Status::Ok, aircraft: self.summaries() }
    }

    pub fn aircraft_reply(&self, icao: u32) -> AircraftReply {
        AircraftReply {
            v: FORMAT_VERSION,
            status: Status::Ok,
            journal: self.journal(icao).cloned(),
        }
    }

    /// The file's text.
    pub fn to_json(&self) -> String {
        let mut journals: Vec<&Journal> = self.by_icao.values().collect();
        journals.sort_by_key(|j| j.icao);
        serde_json::to_string(&DiskOut { v: FORMAT_VERSION, journals }).unwrap_or_default()
    }

    /// Read a file's text, pruned against `now`. Anything that is not a file of
    /// this version gives an empty history.
    pub fn from_json(text: &str, now: i64) -> History {
        let Ok(disk) = serde_json::from_str::<DiskIn>(text) else { return History::default() };
        if disk.v != FORMAT_VERSION {
            return History::default();
        }
        let mut h = History::default();
        for j in disk.journals {
            h.by_icao.insert(j.icao & ADDRESS_MASK, j);
        }
        h.prune(now);
        h.dirty = false;
        h
    }
}

/// An aircraft's address is 24 bits.
const ADDRESS_MASK: u32 = 0xff_ffff;

#[derive(Serialize)]
struct DiskOut<'a> {
    v: u32,
    journals: Vec<&'a Journal>,
}

#[derive(Deserialize)]
struct DiskIn {
    v: u32,
    journals: Vec<Journal>,
}

impl Journal {
    fn new(icao: u32, now: i64) -> Journal {
        Journal { icao, first_seen: now, touched: now, ..Journal::default() }
    }

    /// Note a contact at `at`; `heard` says the radio made it. Whether this
    /// changed anything.
    fn touch(&mut self, at: i64, heard: bool) -> bool {
        let mut changed = false;
        if at > self.touched {
            self.touched = at;
            changed = true;
        }
        if heard && at > self.last_seen {
            self.last_seen = at;
            changed = true;
        }
        changed
    }
}

/// Trimmed, upper-case, and nothing when nothing is left.
fn upper(s: &str) -> Option<String> {
    let t = s.trim().to_uppercase();
    (!t.is_empty()).then_some(t)
}

/// Record `text` as used at `at`; whether that changed the list.
fn upsert(list: &mut Vec<Seen>, text: String, at: i64) -> bool {
    match list.iter_mut().find(|s| s.text == text) {
        Some(s) if at > s.last => {
            s.last = at;
            true
        }
        Some(_) => false,
        None => {
            list.push(Seen { text, first: at, last: at });
            true
        }
    }
}

/// A place on Earth that is the position of now, not of a while ago.
fn good_fix(now: i64, f: &Fix) -> bool {
    f.lat.is_finite()
        && f.lon.is_finite()
        && (-90.0..=90.0).contains(&f.lat)
        && (-180.0..=180.0).contains(&f.lon)
        && f.at >= now - STALE_FIX_S
        && f.at <= now + 60
}

/// Metres between two points, flat-earth: exact enough for a hundred.
fn distance_m(a: (f64, f64), b: (f64, f64)) -> f64 {
    let dlat = (b.0 - a.0).to_radians();
    let mut dlon = b.1 - a.1;
    if dlon > 180.0 {
        dlon -= 360.0;
    } else if dlon < -180.0 {
        dlon += 360.0;
    }
    let dlon = dlon.to_radians() * ((a.0 + b.0) / 2.0).to_radians().cos();
    6_371_000.0 * dlat.hypot(dlon)
}

/// Keep the fix if it is worth a point; whether it was kept.
fn add_point(j: &mut Journal, f: &Fix, per_aircraft: usize) -> bool {
    if let Some(last) = j.points.last() {
        let dt = f.at - last.time();
        if dt <= 0 {
            return false;
        }
        let far = || {
            distance_m((f64::from(last.lat()), f64::from(last.lon())), (f.lat, f.lon)) >= MIN_MOVE_M
        };
        if dt < HEARTBEAT_S && !(dt >= MIN_GAP_S && far()) {
            return false;
        }
    }
    j.points.push(Point(f.at, f.lat as f32, f.lon as f32, f.altitude_ft, f.on_ground));
    let excess = j.points.len().saturating_sub(per_aircraft);
    j.points.drain(..excess);
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: i64 = 1_000_000;
    const A: u32 = 0x4840D6;
    const B: u32 = 0x3C6444;

    fn fix(at: i64, lat: f64, lon: f64) -> Fix {
        Fix { at, lat, lon, altitude_ft: Some(35_000), on_ground: false }
    }

    fn heard(h: &mut History, now: i64, icao: u32, f: Fix) {
        h.note_heard(now, icao, Some("KLM1234"), Some("heavy"), Some(f));
    }

    fn small(per: usize, total: usize, aircraft: usize) -> History {
        History::with_limits(Limits {
            retention_s: 10_000,
            per_aircraft: per,
            total_points: total,
            aircraft,
        })
    }

    // ── what the list says about each aircraft ────────────────────────────

    fn carded(h: &mut History, icao: u32) {
        let place = |c: &str| Some(Place { code: c.into(), name: format!("{c} airport") });
        h.note_card(
            NOW,
            icao,
            SeenCard {
                at: NOW,
                type_name: Some("747-8".into()),
                operator: Some("KLM".into()),
                origin: place("AMS"),
                destination: place("JFK"),
                ..SeenCard::default()
            },
        );
    }

    #[test]
    fn the_list_carries_the_category_and_the_relays_card_facts() {
        let mut h = History::default();
        heard(&mut h, NOW, A, fix(NOW, 52.0, 5.0));
        carded(&mut h, A);
        let s = &h.summaries()[0];
        assert_eq!(s.category.as_deref(), Some("heavy"));
        assert_eq!(s.type_name.as_deref(), Some("747-8"));
        assert_eq!(s.operator.as_deref(), Some("KLM"));
        assert_eq!((s.origin.as_deref(), s.destination.as_deref()), (Some("AMS"), Some("JFK")));
    }

    #[test]
    fn an_aircraft_with_no_card_has_none_of_the_card_facts() {
        let mut h = History::default();
        heard(&mut h, NOW, B, fix(NOW, 52.0, 5.0));
        let s = &h.summaries()[0];
        assert_eq!(s.category.as_deref(), Some("heavy"), "the radio's own");
        assert_eq!(
            (&s.type_name, &s.operator, &s.origin, &s.destination),
            (&None, &None, &None, &None)
        );
    }

    #[test]
    fn the_list_leaves_out_what_it_does_not_have_and_reads_an_older_list() {
        let mut h = History::default();
        h.note_heard(NOW, B, None, None, None);
        let json = serde_json::to_string(&h.list_reply()).unwrap();
        for key in ["category", "type_name", "operator", "origin", "destination"] {
            assert!(!json.contains(key), "{key} in {json}");
        }
        let old = r#"{"v":1,"status":"ok","aircraft":[{"icao":1,"callsign":null,"registration":null,"first_seen":1,"last_seen":2,"points":0}]}"#;
        let back: ListReply = serde_json::from_str(old).unwrap();
        assert_eq!(back.aircraft[0].type_name, None);
    }

    #[test]
    fn the_list_with_the_card_facts_stays_small_for_four_thousand_aircraft() {
        let mut h = History::default();
        for i in 0..MAX_AIRCRAFT as u32 {
            heard(&mut h, NOW, 0x400000 + i, fix(NOW, 52.0, 5.0));
            carded(&mut h, 0x400000 + i);
        }
        let json = serde_json::to_string(&h.list_reply()).unwrap();
        assert_eq!(h.summaries().len(), MAX_AIRCRAFT);
        assert!(json.len() < 1_500_000, "{} bytes", json.len());
    }

    // ── which positions are kept ──────────────────────────────────────────

    #[test]
    fn the_first_position_is_kept() {
        let mut h = History::new();
        heard(&mut h, NOW, A, fix(NOW, 52.0, 5.0));
        let j = h.journal(A).expect("a journal");
        assert_eq!(j.points, [Point(NOW, 52.0, 5.0, Some(35_000), false)]);
        assert_eq!((j.first_seen, j.last_seen), (NOW, NOW));
    }

    #[test]
    fn a_position_too_soon_or_too_near_is_dropped() {
        let mut h = History::new();
        heard(&mut h, NOW, A, fix(NOW, 52.0, 5.0));
        // 5 s later and far: too soon.
        heard(&mut h, NOW + 5, A, fix(NOW + 5, 52.1, 5.0));
        // 20 s later and 10 m away: too near.
        heard(&mut h, NOW + 20, A, fix(NOW + 20, 52.0001, 5.0));
        assert_eq!(h.journal(A).unwrap().points.len(), 1);
    }

    #[test]
    fn a_position_after_ten_seconds_and_a_hundred_metres_is_kept() {
        let mut h = History::new();
        heard(&mut h, NOW, A, fix(NOW, 52.0, 5.0));
        // 0.002° of latitude is about 222 m.
        heard(&mut h, NOW + 10, A, fix(NOW + 10, 52.002, 5.0));
        assert_eq!(h.journal(A).unwrap().points.len(), 2);
        // Just under 100 m (0.0008° ≈ 89 m), 10 s on: still dropped.
        heard(&mut h, NOW + 20, A, fix(NOW + 20, 52.0028, 5.0));
        assert_eq!(h.journal(A).unwrap().points.len(), 2);
    }

    #[test]
    fn a_parked_aircraft_still_leaves_a_point_a_minute() {
        let mut h = History::new();
        heard(&mut h, NOW, A, fix(NOW, 52.0, 5.0));
        heard(&mut h, NOW + 59, A, fix(NOW + 59, 52.0, 5.0));
        assert_eq!(h.journal(A).unwrap().points.len(), 1);
        heard(&mut h, NOW + 60, A, fix(NOW + 60, 52.0, 5.0));
        assert_eq!(h.journal(A).unwrap().points.len(), 2);
    }

    #[test]
    fn a_position_that_is_stale_future_or_not_a_place_is_ignored() {
        let mut h = History::new();
        for f in [
            fix(NOW - STALE_FIX_S - 1, 52.0, 5.0),
            fix(NOW + 3600, 52.0, 5.0),
            fix(NOW, 91.0, 5.0),
            fix(NOW, -91.0, 5.0),
            fix(NOW, 52.0, 181.0),
            fix(NOW, f64::NAN, 5.0),
            fix(NOW, 52.0, f64::INFINITY),
        ] {
            h.note_fix(NOW, A, f);
        }
        assert!(h.journal(A).is_none_or(|j| j.points.is_empty()), "{:?}", h.journal(A));
        // The control: a good one on the same aircraft is kept.
        h.note_fix(NOW, A, fix(NOW - STALE_FIX_S, 52.0, 5.0));
        assert_eq!(h.journal(A).unwrap().points.len(), 1);
    }

    #[test]
    fn the_same_position_twice_counts_once() {
        let mut h = History::new();
        heard(&mut h, NOW, A, fix(NOW - 30, 52.0, 5.0));
        heard(&mut h, NOW + 1, A, fix(NOW - 30, 52.5, 5.5));
        heard(&mut h, NOW + 2, A, fix(NOW - 40, 53.0, 6.0));
        assert_eq!(
            h.journal(A).unwrap().points.len(),
            1,
            "a fix not newer than the last is old news"
        );
    }

    // ── what is forgotten ─────────────────────────────────────────────────

    #[test]
    fn what_is_past_retention_is_forgotten_and_what_is_not_stays() {
        let mut h = History::with_limits(Limits {
            retention_s: 100,
            per_aircraft: 10,
            total_points: 100,
            aircraft: 10,
        });
        heard(&mut h, 1000, A, fix(1000, 52.0, 5.0));
        heard(&mut h, 1100, A, fix(1100, 52.1, 5.0));
        h.note_card(1000, B, SeenCard { at: 1000, ..SeenCard::default() });
        h.prune(1150);
        let j = h.journal(A).expect("heard 50 s ago, kept");
        assert_eq!(j.points.len(), 1, "the 150 s old point went, the 50 s one stayed");
        assert_eq!(j.points[0].time(), 1100);
        assert!(h.journal(B).is_none(), "a card nobody touched for 150 s went with its aircraft");
        h.prune(1250);
        assert!(h.is_empty(), "nothing left once all of it is old");
    }

    #[test]
    fn an_aircraft_keeps_only_its_newest_points() {
        let mut h = small(3, 100, 10);
        for i in 0..6 {
            let t = NOW + i * 60;
            heard(&mut h, t, A, fix(t, 52.0 + i as f64 * 0.01, 5.0));
        }
        let ts: Vec<i64> = h.journal(A).unwrap().points.iter().map(Point::time).collect();
        assert_eq!(ts, [NOW + 180, NOW + 240, NOW + 300]);
    }

    #[test]
    fn the_total_budget_drops_the_aircraft_heard_least_recently() {
        let mut h = small(10, 4, 10);
        for (icao, base) in [(1u32, 0i64), (2, 100), (3, 200)] {
            for i in 0..2 {
                let t = NOW + base + i * 60;
                heard(&mut h, t, icao, fix(t, 52.0 + i as f64 * 0.01, 5.0));
            }
        }
        assert_eq!(h.point_count(), 6);
        h.prune(NOW + 260);
        assert_eq!(h.point_count(), 4);
        assert!(h.journal(1).is_none(), "the one heard longest ago went whole");
        assert!(h.journal(2).is_some() && h.journal(3).is_some());
    }

    #[test]
    fn the_aircraft_ceiling_drops_the_one_touched_least_recently() {
        let mut h = small(10, 100, 2);
        for (i, icao) in [1u32, 2, 3].into_iter().enumerate() {
            h.note_heard(NOW + i as i64, icao, Some("X"), None, None);
        }
        h.prune(NOW + 10);
        assert_eq!(h.len(), 2);
        assert!(h.journal(1).is_none() && h.journal(2).is_some() && h.journal(3).is_some());
    }

    // ── who it was ────────────────────────────────────────────────────────

    #[test]
    fn a_new_callsign_is_a_second_identity_and_an_old_one_is_not_repeated() {
        let mut h = History::new();
        h.note_heard(NOW, A, Some("KLM1234 "), None, None);
        h.note_heard(NOW + 100, A, Some("KLM1234"), None, None);
        h.note_heard(NOW + 200, A, Some("KLM5678"), None, None);
        h.note_heard(NOW + 300, A, Some("klm1234"), None, None);
        h.note_heard(NOW + 400, A, Some("   "), None, None);
        h.note_heard(NOW + 400, A, None, None, None);
        let j = h.journal(A).unwrap();
        assert_eq!(j.callsigns.len(), 2, "{:?}", j.callsigns);
        let first = j.callsigns.iter().find(|s| s.text == "KLM1234").unwrap();
        assert_eq!((first.first, first.last), (NOW, NOW + 300));
        assert_eq!(j.callsign(), Some("KLM1234"), "the one used last");
    }

    #[test]
    fn a_vdl2_message_gives_a_registration_and_repeating_it_changes_nothing() {
        let mut h = History::new();
        h.note_identity(NOW, A, Some("KL1234"), Some("ph-bxa"));
        assert!(h.take_dirty());
        let before = h.journal(A).cloned();
        h.note_identity(NOW, A, Some("KL1234"), Some("PH-BXA"));
        h.note_identity(NOW - 50, A, Some("KL1234"), Some("PH-BXA"));
        assert_eq!(
            h.journal(A).cloned(),
            before,
            "a message already counted, or older, moves nothing"
        );
        assert!(!h.take_dirty(), "and is not worth a write");
        let j = h.journal(A).unwrap();
        assert_eq!(j.registration(), Some("PH-BXA"));
        assert_eq!(j.callsign(), Some("KL1234"));
        assert_eq!((j.first_seen, j.last_seen), (NOW, NOW));
    }

    #[test]
    fn the_category_is_kept_once_it_is_said() {
        let mut h = History::new();
        h.note_heard(NOW, A, None, Some("heavy"), None);
        h.note_heard(NOW + 1, A, None, None, None);
        assert_eq!(h.journal(A).unwrap().category.as_deref(), Some("heavy"));
    }

    // ── the card ──────────────────────────────────────────────────────────

    #[test]
    fn a_card_is_kept_with_its_page_and_credit_and_nothing_to_show_a_picture() {
        let mut h = History::new();
        h.note_card(
            NOW,
            A,
            SeenCard {
                at: NOW,
                type_name: Some("737-800".into()),
                operator: Some("KLM".into()),
                origin: Some(Place { code: "EHAM".into(), name: "Amsterdam".into() }),
                photo_page: Some("https://www.planespotters.net/photo/1/x".into()),
                photo_credit: Some("A. Spotter".into()),
                ..SeenCard::default()
            },
        );
        let j = h.journal(A).unwrap();
        assert_eq!(j.card.as_ref().unwrap().type_name.as_deref(), Some("737-800"));
        assert_eq!(j.last_seen, 0, "a card is not the radio hearing it");
        assert_eq!(j.touched, NOW);
        let json = serde_json::to_value(j.card.as_ref().unwrap()).unwrap();
        let mut keys: Vec<&str> = json.as_object().unwrap().keys().map(String::as_str).collect();
        keys.sort_unstable();
        assert_eq!(
            keys,
            [
                "at",
                "destination",
                "manufacturer",
                "operator",
                "origin",
                "photo_credit",
                "photo_page",
                "registration",
                "type_name"
            ],
            "no field for a picture or its address"
        );
    }

    // ── the file ──────────────────────────────────────────────────────────

    fn busy() -> History {
        let mut h = History::new();
        heard(&mut h, NOW, A, fix(NOW, 52.0, 5.0));
        heard(&mut h, NOW + 20, A, fix(NOW + 20, 52.01, 5.0));
        h.note_identity(NOW, A, None, Some("PH-BXA"));
        h.note_card(
            NOW,
            B,
            SeenCard { at: NOW, operator: Some("DLH".into()), ..SeenCard::default() },
        );
        h
    }

    #[test]
    fn the_file_reads_back_as_it_was_written() {
        let h = busy();
        let back = History::from_json(&h.to_json(), NOW + 60);
        assert_eq!(back.len(), 2);
        assert_eq!(back.journal(A), h.journal(A));
        assert_eq!(back.journal(B), h.journal(B));
    }

    #[test]
    fn a_position_is_written_as_a_bare_array() {
        let mut h = History::new();
        heard(&mut h, NOW, A, fix(NOW, 52.0, 5.0));
        let json = h.to_json();
        assert!(json.contains("[1000000,52.0,5.0,35000,false]"), "{json}");
    }

    #[test]
    fn the_file_is_pruned_as_it_is_read() {
        let json = busy().to_json();
        let back = History::from_json(&json, NOW + RETENTION_S + 100);
        assert!(back.is_empty(), "all of it is more than a day old");
        let back = History::from_json(&json, NOW + 100);
        assert_eq!(back.len(), 2);
    }

    #[test]
    fn a_file_that_is_not_ours_gives_an_empty_history_and_no_panic() {
        for text in
            ["", "not json", "[]", "{}", r#"{"v":99,"journals":[]}"#, r#"{"v":1,"journals":"x"}"#]
        {
            assert!(History::from_json(text, NOW).is_empty(), "{text:?}");
        }
        // The control: a real one is not empty.
        assert!(!History::from_json(&busy().to_json(), NOW).is_empty());
    }

    #[test]
    fn something_changed_is_told_once() {
        let mut h = History::new();
        assert!(!h.take_dirty());
        heard(&mut h, NOW, A, fix(NOW, 52.0, 5.0));
        assert!(h.take_dirty());
        assert!(!h.take_dirty(), "told once");
    }

    // ── what is answered ──────────────────────────────────────────────────

    #[test]
    fn the_list_has_a_line_per_aircraft_the_most_recent_first() {
        let mut h = busy();
        h.note_heard(NOW + 500, B, Some("DLH9AB"), None, None);
        let r = h.list_reply();
        assert_eq!((r.v, r.status), (FORMAT_VERSION, Status::Ok));
        let icaos: Vec<u32> = r.aircraft.iter().map(|s| s.icao).collect();
        assert_eq!(icaos, [B, A]);
        let a = &r.aircraft[1];
        assert_eq!(
            (a.callsign.as_deref(), a.registration.as_deref(), a.points, a.first_seen, a.last_seen),
            (Some("KLM1234"), Some("PH-BXA"), 2, NOW, NOW + 20)
        );
    }

    #[test]
    fn an_aircraft_never_heard_is_ok_and_empty() {
        let h = busy();
        let r = h.aircraft_reply(0x123456);
        assert_eq!((r.status, r.journal), (Status::Ok, None));
        let r = h.aircraft_reply(A | 0xFF00_0000);
        assert_eq!(r.journal.map(|j| j.icao), Some(A), "only the 24 bits are the address");
    }

    // ── how big the file can get ──────────────────────────────────────────

    #[test]
    fn the_ceilings_are_the_ones_that_keep_the_file_small() {
        assert_eq!(MAX_POINTS_TOTAL, 50_000);
        assert_eq!(MAX_POINTS_PER_AIRCRAFT, 300);
        assert_eq!(MAX_AIRCRAFT, 4000);
        assert_eq!(RETENTION_S, 24 * 3600);
    }

    /// The worst case the ceilings allow: the most aircraft, each with two
    /// callsigns, a registration, a category and a full card, and more points
    /// than the budget — pruned the way the server prunes before it writes.
    fn at_the_ceilings() -> (History, String) {
        let mut h = History::new();
        let per = 13; // 4000 × 13 = 52 000, over the budget of 50 000
        let mut last = NOW;
        for j in 0..per {
            for i in 0..MAX_AIRCRAFT as i64 {
                let t = NOW + j * 60 + i;
                last = last.max(t);
                let icao = 0x400000 + i as u32;
                let f = Fix {
                    at: t,
                    lat: 52.123456 + 0.001 * (j as f64) + 1e-6 * (i as f64),
                    lon: 5.654321 + 0.001 * (j as f64),
                    altitude_ft: Some(35_000 + (j as i32) * 100),
                    on_ground: false,
                };
                h.note_heard(
                    t,
                    icao,
                    Some(&format!("KLM{:04}", i % 10_000)),
                    Some("heavy"),
                    Some(f),
                );
            }
        }
        for i in 0..MAX_AIRCRAFT as u32 {
            let icao = 0x400000 + i;
            h.note_identity(NOW, icao, Some("KLM9999"), Some("PH-BXA"));
            h.note_card(
                last,
                icao,
                SeenCard {
                    at: last,
                    type_name: Some("A330-300 (a rather long type name)".into()),
                    manufacturer: Some("Airbus Industrie".into()),
                    operator: Some("Koninklijke Luchtvaart Maatschappij".into()),
                    registration: Some("PH-BXA".into()),
                    origin: Some(Place { code: "EHAM".into(), name: "Amsterdam Schiphol".into() }),
                    destination: Some(Place {
                        code: "EGLL".into(),
                        name: "London Heathrow".into(),
                    }),
                    photo_page: Some("https://example.invalid/photo/1234567/abcdef".into()),
                    photo_credit: Some("A. Photographer".into()),
                },
            );
        }
        h.prune(last);
        let json = h.to_json();
        (h, json)
    }

    #[test]
    fn the_total_budget_is_what_bounds_the_points() {
        let (h, _) = at_the_ceilings();
        assert!(h.point_count() <= 50_000, "{}", h.point_count());
        assert!(h.point_count() > 40_000, "and not pruned far below it: {}", h.point_count());
        assert!(h.len() <= MAX_AIRCRAFT);
    }

    #[test]
    fn the_file_at_its_ceilings_stays_under_six_megabytes() {
        let (h, json) = at_the_ceilings();
        eprintln!(
            "at the ceilings: {} aircraft, {} points, {} bytes of JSON",
            h.len(),
            h.point_count(),
            json.len()
        );
        assert!(json.len() < 6_000_000, "{} bytes", json.len());
        // The control: it is a real file, not an empty one.
        assert!(json.len() > 1_000_000, "{} bytes", json.len());
    }

    // ── what it may not be ────────────────────────────────────────────────

    #[test]
    fn this_crate_reaches_nothing_and_names_no_service() {
        let src = include_str!("lib.rs");
        let code = &src[..src.find("#[cfg(test)]").expect("the tests follow the code")];
        for banned in [
            "ureq",
            "reqwest",
            "hyper",
            "TcpStream",
            "UdpSocket",
            "std::net",
            "std::fs",
            "tokio",
            "adsbdb",
            "planespotters",
            "plnspttrs",
            "localStorage",
            "web_sys",
            "raw_hex",
        ] {
            assert!(!code.contains(banned), "the model must not touch `{banned}`");
        }
        assert!(code.contains("HashMap"), "the control: the scan sees what is there");
    }
}
