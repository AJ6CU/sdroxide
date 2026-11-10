//! What has been asked, what came back, and what is on its way.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use crate::model::{Adsbdb, Lookup, Photo, Reply, Request, clean_callsign, clean_registration};

/// How long an answer, or an "unknown", is good for.
pub const TTL_S: i64 = 24 * 3600;
/// How long a failed lookup is left alone before the next try.
pub const BACKOFF_S: i64 = 5 * 60;
/// Most answers kept, per kind. The oldest go first.
pub const MAX_ENTRIES: usize = 2000;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Slot<T> {
    at: i64,
    result: Lookup<T>,
}

impl<T> Slot<T> {
    fn fresh(&self, now: i64) -> bool {
        let limit = match self.result {
            Lookup::Failed => BACKOFF_S,
            _ => TTL_S,
        };
        (0..limit).contains(&(now - self.at))
    }
}

/// The key of an adsbdb answer: the route depends on the flight, so the same
/// airframe under another callsign is a new question.
type AdsbKey = (u32, Option<String>);

#[derive(Serialize, Deserialize)]
struct AdsbRow {
    icao: u32,
    callsign: Option<String>,
    at: i64,
    result: Lookup<Adsbdb>,
}

#[derive(Serialize, Deserialize)]
struct PhotoRow {
    registration: String,
    at: i64,
    result: Lookup<Photo>,
}

#[derive(Serialize, Deserialize, Default)]
struct Disk {
    #[serde(default)]
    adsbdb: Vec<AdsbRow>,
    #[serde(default)]
    photos: Vec<PhotoRow>,
}

#[derive(Default)]
pub struct OnlineCache {
    adsbdb: HashMap<AdsbKey, Slot<Adsbdb>>,
    photos: HashMap<String, Slot<Photo>>,
    inflight: HashSet<Request>,
    /// Set whenever an answer is stored, so the caller knows to write the file.
    dirty: bool,
}

impl OnlineCache {
    /// The good answer to `req`, with when it was got, if there is one. A
    /// failure inside its back-off counts: it is the answer for five minutes.
    pub fn answer(&self, now: i64, req: &Request) -> Option<(Reply, i64)> {
        match req {
            Request::Adsbdb { icao, callsign } => {
                let s = self.adsbdb.get(&(*icao, callsign.clone()))?;
                s.fresh(now).then(|| (Reply::Adsbdb(s.result.clone()), s.at))
            }
            Request::Photo { registration } => {
                let s = self.photos.get(registration)?;
                s.fresh(now).then(|| (Reply::Photo(s.result.clone()), s.at))
            }
        }
    }

    /// Mark `req` as on its way. `false` means it already is, and the caller
    /// should wait for that one rather than send a second.
    pub fn begin(&mut self, req: &Request) -> bool {
        self.inflight.insert(req.clone())
    }

    /// Give up on a request that was begun and never sent.
    pub fn abort(&mut self, req: &Request) {
        self.inflight.remove(req);
    }

    pub fn is_inflight(&self, req: &Request) -> bool {
        self.inflight.contains(req)
    }

    /// Store what came back. A reply that does not match its request is
    /// dropped, and the request stops being "on its way" either way.
    pub fn complete(&mut self, now: i64, req: Request, reply: Reply) {
        self.inflight.remove(&req);
        match (req, reply) {
            (Request::Adsbdb { icao, callsign }, Reply::Adsbdb(result)) => {
                self.adsbdb.insert((icao, callsign), Slot { at: now, result });
                self.dirty = true;
            }
            (Request::Photo { registration }, Reply::Photo(result)) => {
                self.photos.insert(registration, Slot { at: now, result });
                self.dirty = true;
            }
            _ => {}
        }
    }

    /// Drop what has expired and keep the newest [`MAX_ENTRIES`] of the rest.
    pub fn purge(&mut self, now: i64) {
        self.adsbdb.retain(|_, s| s.fresh(now));
        self.photos.retain(|_, s| s.fresh(now));
        cap(&mut self.adsbdb);
        cap(&mut self.photos);
    }

    /// How many answers are held, of each kind.
    pub fn len(&self) -> (usize, usize) {
        (self.adsbdb.len(), self.photos.len())
    }

    pub fn is_empty(&self) -> bool {
        self.len() == (0, 0)
    }

    /// True once after an answer was stored.
    pub fn take_dirty(&mut self) -> bool {
        std::mem::take(&mut self.dirty)
    }

    /// The file's contents: answers only, no failure.
    pub fn to_json(&self) -> String {
        let disk = Disk {
            adsbdb: self
                .adsbdb
                .iter()
                .filter(|(_, s)| s.result != Lookup::Failed)
                .map(|((icao, callsign), s)| AdsbRow {
                    icao: *icao,
                    callsign: callsign.clone(),
                    at: s.at,
                    result: s.result.clone(),
                })
                .collect(),
            photos: self
                .photos
                .iter()
                .filter(|(_, s)| s.result != Lookup::Failed)
                .map(|(r, s)| PhotoRow {
                    registration: r.clone(),
                    at: s.at,
                    result: s.result.clone(),
                })
                .collect(),
        };
        serde_json::to_string(&disk).unwrap_or_default()
    }

    /// Read the file back, keeping only what is still good and still a key we
    /// would have sent. A file that cannot be read is an empty cache, never an
    /// error.
    pub fn from_json(s: &str, now: i64) -> OnlineCache {
        let disk: Disk = serde_json::from_str(s).unwrap_or_default();
        let mut c = OnlineCache::default();
        for r in disk.adsbdb {
            let slot = Slot { at: r.at, result: r.result };
            let key_ok =
                r.callsign.as_deref().is_none_or(|c| clean_callsign(c).as_deref() == Some(c));
            if key_ok && slot.result != Lookup::Failed && slot.fresh(now) {
                c.adsbdb.insert((r.icao & 0xff_ffff, r.callsign), slot);
            }
        }
        for r in disk.photos {
            let slot = Slot { at: r.at, result: r.result };
            let key_ok = clean_registration(&r.registration).as_deref() == Some(&r.registration);
            if key_ok && slot.result != Lookup::Failed && slot.fresh(now) {
                c.photos.insert(r.registration, slot);
            }
        }
        cap(&mut c.adsbdb);
        cap(&mut c.photos);
        c
    }
}

fn cap<K: Clone + Eq + std::hash::Hash, T>(m: &mut HashMap<K, Slot<T>>) {
    if m.len() <= MAX_ENTRIES {
        return;
    }
    let mut ages: Vec<i64> = m.values().map(|s| s.at).collect();
    ages.sort_unstable();
    let cut = ages[m.len() - MAX_ENTRIES];
    m.retain(|_, s| s.at >= cut);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{fixtures, parse_adsbdb, parse_photo};

    const NOW: i64 = 1_800_000_000;

    fn found() -> Adsbdb {
        parse_adsbdb(fixtures::ADSBDB).unwrap()
    }

    fn photo() -> Photo {
        parse_photo(fixtures::PHOTO).unwrap().unwrap()
    }

    fn adsb(icao: u32, call: Option<&str>) -> Request {
        Request::Adsbdb { icao, callsign: call.map(str::to_string) }
    }

    fn store(c: &mut OnlineCache, now: i64, req: Request, reply: Reply) {
        assert!(c.begin(&req));
        c.complete(now, req, reply);
    }

    #[test]
    fn a_request_is_begun_once_while_the_answer_is_on_its_way() {
        let mut c = OnlineCache::default();
        let r = adsb(0x484006, Some("KLM1234"));
        assert!(c.begin(&r));
        assert!(!c.begin(&r), "the second asker must wait for the first");
        assert!(c.is_inflight(&r));
        c.abort(&r);
        assert!(!c.is_inflight(&r));
        assert!(c.begin(&r));
    }

    #[test]
    fn an_answer_is_good_for_24_hours_and_not_a_second_longer() {
        let mut c = OnlineCache::default();
        let r = adsb(0x484006, Some("KLM1234"));
        store(&mut c, NOW, r.clone(), Reply::Adsbdb(Lookup::Found(found())));
        assert!(c.answer(NOW + TTL_S - 1, &r).is_some());
        assert!(c.answer(NOW + TTL_S, &r).is_none());
    }

    #[test]
    fn a_not_found_is_remembered_as_long_as_an_answer() {
        let mut c = OnlineCache::default();
        let r = adsb(0x1, None);
        store(&mut c, NOW, r.clone(), Reply::Adsbdb(Lookup::NotFound));
        assert!(c.answer(NOW + 3600, &r).is_some());
    }

    #[test]
    fn a_failure_waits_five_minutes_and_then_is_forgotten() {
        let mut c = OnlineCache::default();
        let r = adsb(0x484006, None);
        store(&mut c, NOW, r.clone(), Reply::Adsbdb(Lookup::Failed));
        assert!(c.answer(NOW + BACKOFF_S - 1, &r).is_some());
        assert!(c.answer(NOW + BACKOFF_S, &r).is_none());
    }

    #[test]
    fn a_new_callsign_is_a_new_question() {
        let mut c = OnlineCache::default();
        store(&mut c, NOW, adsb(0x484006, Some("KLM1234")), Reply::Adsbdb(Lookup::Found(found())));
        assert!(c.answer(NOW + 10, &adsb(0x484006, Some("KLM5678"))).is_none());
    }

    #[test]
    fn a_reply_for_another_request_is_ignored_and_frees_the_slot() {
        let mut c = OnlineCache::default();
        let r = adsb(0x1, None);
        assert!(c.begin(&r));
        c.complete(NOW, r.clone(), Reply::Photo(Lookup::NotFound));
        assert_eq!(c.len(), (0, 0));
        assert!(!c.is_inflight(&r));
    }

    #[test]
    fn the_file_holds_answers_only_never_a_failure() {
        let mut c = OnlineCache::default();
        store(&mut c, NOW, adsb(1, None), Reply::Adsbdb(Lookup::Found(found())));
        store(&mut c, NOW, adsb(2, None), Reply::Adsbdb(Lookup::Failed));
        let p = Request::Photo { registration: "PH-BXA".into() };
        store(&mut c, NOW, p, Reply::Photo(Lookup::Found(photo())));
        assert!(!c.to_json().contains("Failed"), "a failure is not written to the file");
        let back = OnlineCache::from_json(&c.to_json(), NOW + 60);
        assert_eq!(back.len(), (1, 1));
        assert_eq!(
            back.answer(NOW + 60, &adsb(1, None)).unwrap().0,
            Reply::Adsbdb(Lookup::Found(found()))
        );
        assert!(back.answer(NOW + 60, &adsb(2, None)).is_none());
    }

    #[test]
    fn the_file_never_carries_a_picture_only_its_address() {
        let mut c = OnlineCache::default();
        let p = Request::Photo { registration: "PH-BXA".into() };
        store(&mut c, NOW, p, Reply::Photo(Lookup::Found(photo())));
        let j = c.to_json();
        assert!(j.contains("large.jpg") && j.contains("Jan Jansen"));
        assert!(!j.contains("base64") && !j.contains("bytes"));
    }

    #[test]
    fn what_is_older_than_24_hours_is_not_read_back() {
        let mut c = OnlineCache::default();
        store(&mut c, NOW, adsb(1, None), Reply::Adsbdb(Lookup::Found(found())));
        let p = Request::Photo { registration: "PH-BXA".into() };
        store(&mut c, NOW, p, Reply::Photo(Lookup::Found(photo())));
        let j = c.to_json();
        assert_eq!(OnlineCache::from_json(&j, NOW + TTL_S - 1).len(), (1, 1));
        assert_eq!(OnlineCache::from_json(&j, NOW + TTL_S).len(), (0, 0), "both kinds expire");
    }

    #[test]
    fn a_file_that_is_not_ours_is_an_empty_cache() {
        assert!(OnlineCache::from_json("not json", NOW).is_empty());
        assert!(OnlineCache::from_json("", NOW).is_empty());
        assert!(OnlineCache::from_json("{}", NOW).is_empty());
    }

    #[test]
    fn a_key_in_the_file_that_we_would_never_have_sent_is_not_read_back() {
        let row = |call: &str, reg: &str| {
            format!(
                r#"{{"adsbdb":[{{"icao":1,"callsign":"{call}","at":{NOW},"result":"NotFound"}}],
                    "photos":[{{"registration":"{reg}","at":{NOW},"result":"NotFound"}}]}}"#
            )
        };
        assert_eq!(OnlineCache::from_json(&row("KLM1234", "PH-BXA"), NOW).len(), (1, 1));
        assert_eq!(OnlineCache::from_json(&row("klm/../x", "ph bxa"), NOW).len(), (0, 0));
    }

    #[test]
    fn purge_drops_the_expired_and_keeps_the_newest_up_to_the_cap() {
        let mut c = OnlineCache::default();
        for i in 0..(MAX_ENTRIES as u32 + 10) {
            c.adsbdb.insert((i, None), Slot { at: NOW + i as i64, result: Lookup::NotFound });
        }
        c.adsbdb.insert((9_000_000, None), Slot { at: NOW - TTL_S - 5, result: Lookup::NotFound });
        c.purge(NOW + MAX_ENTRIES as i64 + 100);
        assert_eq!(c.len().0, MAX_ENTRIES);
        assert!(c.adsbdb.contains_key(&(MAX_ENTRIES as u32 + 9, None)), "the newest stays");
        assert!(!c.adsbdb.contains_key(&(0, None)), "the oldest goes");
        assert!(!c.adsbdb.contains_key(&(9_000_000, None)), "the expired goes");
    }

    #[test]
    fn storing_an_answer_marks_the_file_for_writing_once() {
        let mut c = OnlineCache::default();
        assert!(!c.take_dirty());
        store(&mut c, NOW, adsb(1, None), Reply::Adsbdb(Lookup::NotFound));
        assert!(c.take_dirty());
        assert!(!c.take_dirty());
    }
}
