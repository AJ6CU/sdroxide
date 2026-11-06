//! The relay a station runs so that its clients need not reach the internet.
//!
//! A client asks one question — an ICAO address, optionally with a callsign and
//! a registration — and gets one [`Info`]. Behind it:
//!
//! * one cache for everyone ([`OnlineCache`]): 24 h for an answer, five
//!   minutes for a failure;
//! * one question out per key, however many clients ask at once: the others
//!   wait for the first and read what it brought back;
//! * a ceiling on questions out per minute, whoever is asking, so a station
//!   cannot be turned into a way of hammering the two services. A question the
//!   ceiling refuses comes back as [`Status::Busy`], at once;
//! * nothing about the clients reaches the services: the `User-Agent` is the
//!   station's, and the only things in a request are the cleaned address,
//!   callsign and registration.

use std::collections::VecDeque;
use std::sync::{Condvar, Mutex, MutexGuard};
use std::time::{Duration, Instant};

use serde::{Deserialize, Serialize};

use crate::cache::OnlineCache;
use crate::model::{
    AircraftInfo, Image, Lookup, Photo, Reply, Request, Route, clean_callsign, clean_registration,
    image_ok, thumb_url_ok,
};
use crate::weather::{Spot, Weather};
use crate::wikipedia::Article;

/// The version of [`Info`]'s shape. Bump on any change a client cannot ignore.
pub const FORMAT_VERSION: u32 = 1;

/// How long a client waits for a question somebody else has already sent.
const WAIT: Duration = Duration::from_secs(10);
/// How often the cache is swept and the file is due.
const HOUSEKEEPING_S: i64 = 60;

/// How the lookup ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// adsbdb knows the aircraft.
    Ok,
    /// adsbdb answered and does not know it.
    Unknown,
    /// No answer from adsbdb: offline, refused, or not understood.
    Unavailable,
    /// The station's ceiling on questions per minute was reached.
    Busy,
    /// The station's operator has not switched the relay on.
    Disabled,
    /// The question was not an ICAO address with a clean callsign and registration.
    Invalid,
    /// The station asks for a sign-in and there was none.
    Unauthorized,
}

/// The answer to one question, as it goes to a client.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Info {
    /// [`FORMAT_VERSION`].
    pub v: u32,
    pub status: Status,
    pub aircraft: Option<AircraftInfo>,
    pub route: Option<Route>,
    /// Planespotters' photo: its address unchanged, its page and its credit.
    /// The picture itself is the client's to load.
    pub photo: Option<Photo>,
    /// When adsbdb's answer was got, in unix seconds.
    pub fetched_at: Option<i64>,
    /// Whether adsbdb's answer came out of the cache rather than the internet.
    pub cached: bool,
}

impl Info {
    /// An answer that says only how it ended.
    pub fn bare(status: Status) -> Info {
        Info {
            v: FORMAT_VERSION,
            status,
            aircraft: None,
            route: None,
            photo: None,
            fetched_at: None,
            cached: false,
        }
    }
}

/// Whatever actually sends a [`Request`] and brings back a [`Reply`].
pub trait Upstream: Send + Sync {
    fn fetch(&self, req: &Request) -> Reply;

    /// The picture at `url`, or `None` for any failure. Only ever called with
    /// an address [`thumb_url_ok`] accepted. A stand-in that has no pictures
    /// need not say so.
    fn image(&self, _url: &str) -> Option<Image> {
        None
    }

    /// The thumbnail of a Wikipedia article, or `None` for any failure: the
    /// page has no picture, the host is down, the reply is not a small JPEG or
    /// PNG. A stand-in that has none need not say so.
    fn wikipedia_thumbnail(&self, _article: &Article) -> Option<Image> {
        None
    }

    /// The weather at `spot`, or `None` for any failure: no route, a refusal, a
    /// body that is not a forecast. A stand-in that has none need not say so.
    fn weather(&self, _spot: &Spot) -> Option<Weather> {
        None
    }
}

/// How a question about the weather at a place ended.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Outlook {
    Got(Weather),
    /// The station's ceiling on questions per minute was reached.
    Busy,
    /// No usable answer: offline, refused, or not understood.
    Failed,
}

struct RateLimit {
    max: usize,
    stamps: VecDeque<i64>,
}

impl RateLimit {
    /// Whether one more question may go out at `now`, and if so, count it.
    fn take(&mut self, now: i64) -> bool {
        while self.stamps.front().is_some_and(|&t| now - t >= 60) {
            self.stamps.pop_front();
        }
        if self.stamps.len() >= self.max {
            return false;
        }
        self.stamps.push_back(now);
        true
    }
}

struct State {
    cache: OnlineCache,
    limit: RateLimit,
    swept_at: i64,
    saved_at: i64,
}

pub struct Relay<U> {
    upstream: U,
    state: Mutex<State>,
    done: Condvar,
    wait: Duration,
}

/// Puts a begun request back if the thread sending it never gets to report —
/// a panic in the sender must not leave every later asker waiting on it.
struct Flight<'a, U> {
    relay: &'a Relay<U>,
    req: &'a Request,
    armed: bool,
}

impl<U> Drop for Flight<'_, U> {
    fn drop(&mut self) {
        if self.armed {
            self.relay.lock().cache.abort(self.req);
            self.relay.done.notify_all();
        }
    }
}

enum Got {
    /// The reply, when it was got, and whether it came out of the cache.
    Reply(Reply, i64, bool),
    Busy,
    /// Nobody answered in time.
    Failed,
}

impl<U> Relay<U> {
    fn lock(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }
}

impl<U: Upstream> Relay<U> {
    /// A relay over `upstream` that sends at most `max_per_minute` questions a
    /// minute, starting from whatever `cache` already holds.
    pub fn new(upstream: U, max_per_minute: usize, cache: OnlineCache) -> Relay<U> {
        Relay {
            upstream,
            state: Mutex::new(State {
                cache,
                limit: RateLimit { max: max_per_minute, stamps: VecDeque::new() },
                swept_at: 0,
                saved_at: 0,
            }),
            done: Condvar::new(),
            wait: WAIT,
        }
    }

    /// The thumbnail at `url`, if it is a Planespotters picture address, comes
    /// back a JPEG or PNG and is not too big. Blocking, and not cached here: the
    /// caller keeps it in memory for as long as one card is open and no longer.
    pub fn image(&self, url: &str) -> Option<Image> {
        if !thumb_url_ok(url) {
            return None;
        }
        self.upstream.image(url).filter(|i| image_ok(&i.content_type, i.bytes.len()))
    }

    /// The Wikipedia thumbnail of an airport's article, if it has one and it
    /// comes back a JPEG or PNG that is not too big. Blocking, not cached here:
    /// like [`Relay::image`], the caller keeps it for as long as one card is open.
    pub fn wikipedia_thumbnail(&self, article: &Article) -> Option<Image> {
        self.upstream
            .wikipedia_thumbnail(article)
            .filter(|i| image_ok(&i.content_type, i.bytes.len()))
    }

    /// The weather at `spot`. Counts against the same ceiling as every other
    /// question out. Blocking, and not cached here: the caller keeps the answer
    /// in memory for half a minute and no longer.
    pub fn weather(&self, now: i64, spot: &Spot) -> Outlook {
        if !self.lock().limit.take(now) {
            return Outlook::Busy;
        }
        self.upstream.weather(spot).map_or(Outlook::Failed, Outlook::Got)
    }

    /// How long a client waits for a question already out. For tests.
    pub fn with_wait(mut self, wait: Duration) -> Relay<U> {
        self.wait = wait;
        self
    }

    /// The answer for one aircraft. Blocks while a question is out, so a
    /// server runs it off its async threads.
    pub fn lookup(
        &self,
        now: i64,
        icao: u32,
        callsign: Option<&str>,
        registration: Option<&str>,
    ) -> Info {
        self.sweep(now);
        let req =
            Request::Adsbdb { icao: icao & 0xff_ffff, callsign: callsign.and_then(clean_callsign) };
        // The registration the radio heard wins; failing that, the one adsbdb gives.
        let mut registration = registration.and_then(clean_registration);
        let mut info = Info::bare(Status::Unavailable);
        let mut got = self.obtain(now, &req);
        // adsbdb answers 404 "unknown callsign" for a flight it has no route for,
        // even when it knows the airframe: ask again for the airframe alone.
        if let (Request::Adsbdb { icao, callsign: Some(_) }, Got::Reply(Reply::Adsbdb(Lookup::NotFound), ..)) =
            (&req, &got)
        {
            got = self.obtain(now, &Request::Adsbdb { icao: *icao, callsign: None });
        }
        match got {
            Got::Busy => info.status = Status::Busy,
            Got::Reply(Reply::Adsbdb(lookup), at, cached) => {
                info.fetched_at = Some(at);
                info.cached = cached;
                info.status = match lookup {
                    Lookup::Found(a) => {
                        registration = registration
                            .or_else(|| a.aircraft.as_ref().and_then(|x| x.registration.clone()));
                        info.aircraft = a.aircraft;
                        info.route = a.route;
                        Status::Ok
                    }
                    Lookup::NotFound => Status::Unknown,
                    Lookup::Failed => Status::Unavailable,
                };
            }
            Got::Reply(..) | Got::Failed => {}
        }
        if info.status != Status::Busy {
            if let Some(registration) = registration {
                if let Got::Reply(Reply::Photo(Lookup::Found(p)), ..) =
                    self.obtain(now, &Request::Photo { registration })
                {
                    info.photo = Some(p);
                }
            }
        }
        info
    }

    /// The answer to `req`: from the cache, from a question somebody else has
    /// out, or from one this thread sends.
    fn obtain(&self, now: i64, req: &Request) -> Got {
        let deadline = Instant::now() + self.wait;
        let mut st = self.lock();
        loop {
            if let Some((reply, at)) = st.cache.answer(now, req) {
                return Got::Reply(reply, at, true);
            }
            if st.cache.begin(req) {
                break;
            }
            let left = deadline.saturating_duration_since(Instant::now());
            if left.is_zero() {
                return Got::Failed;
            }
            st = self.done.wait_timeout(st, left).unwrap_or_else(|e| e.into_inner()).0;
        }
        let mut flight = Flight { relay: self, req, armed: true };
        if !st.limit.take(now) {
            drop(st);
            return Got::Busy; // `flight` puts the request back as it goes
        }
        drop(st);
        let reply = self.upstream.fetch(req);
        flight.armed = false;
        self.lock().cache.complete(now, req.clone(), reply.clone());
        self.done.notify_all();
        Got::Reply(reply, now, false)
    }

    fn sweep(&self, now: i64) {
        let mut st = self.lock();
        if now - st.swept_at >= HOUSEKEEPING_S {
            st.swept_at = now;
            st.cache.purge(now);
        }
    }

    /// The cache as a file's contents, if there is something new to write and
    /// a minute has passed since the last time.
    pub fn snapshot_if_due(&self, now: i64) -> Option<String> {
        let mut st = self.lock();
        if now - st.saved_at < HOUSEKEEPING_S || !st.cache.take_dirty() {
            return None;
        }
        st.saved_at = now;
        Some(st.cache.to_json())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, mpsc};

    use super::*;
    use crate::Adsbdb;
    use crate::parse::{fixtures, parse_adsbdb, parse_photo};

    const NOW: i64 = 1_800_000_000;

    /// An upstream that counts the questions it is asked.
    struct Fake {
        calls: Mutex<Vec<Request>>,
        reply: Box<dyn Fn(&Request) -> Reply + Send + Sync>,
        delay: Duration,
    }

    impl Fake {
        fn new(reply: impl Fn(&Request) -> Reply + Send + Sync + 'static) -> Arc<Fake> {
            Arc::new(Fake {
                calls: Mutex::new(Vec::new()),
                reply: Box::new(reply),
                delay: Duration::ZERO,
            })
        }

        fn working() -> Arc<Fake> {
            Fake::new(|r| match r {
                Request::Adsbdb { .. } => {
                    Reply::Adsbdb(Lookup::Found(parse_adsbdb(fixtures::ADSBDB).unwrap()))
                }
                Request::Photo { .. } => {
                    Reply::Photo(Lookup::Found(parse_photo(fixtures::PHOTO).unwrap().unwrap()))
                }
            })
        }

        fn count(&self) -> usize {
            self.calls.lock().unwrap().len()
        }

        fn photo_asked(&self) -> Vec<String> {
            self.calls
                .lock()
                .unwrap()
                .iter()
                .filter_map(|r| match r {
                    Request::Photo { registration } => Some(registration.clone()),
                    _ => None,
                })
                .collect()
        }
    }

    impl Upstream for Arc<Fake> {
        fn fetch(&self, req: &Request) -> Reply {
            self.calls.lock().unwrap().push(req.clone());
            std::thread::sleep(self.delay);
            (self.reply)(req)
        }
    }

    #[test]
    fn a_picture_is_fetched_only_from_the_photo_host_and_only_if_it_is_one() {
        struct Pics;
        impl Upstream for Pics {
            fn fetch(&self, _: &Request) -> Reply {
                Reply::Photo(Lookup::Failed)
            }
            fn image(&self, url: &str) -> Option<Image> {
                let content_type = if url.ends_with("page") { "text/html" } else { "image/jpeg" };
                Some(Image { content_type: content_type.into(), bytes: vec![1, 2, 3] })
            }
        }
        let r = Relay::new(Pics, 10, OnlineCache::default());
        assert!(r.image("https://t.plnspttrs.net/a.jpg").is_some());
        assert!(r.image("https://t.plnspttrs.net/page").is_none(), "not a picture");
        for bad in [
            "http://t.plnspttrs.net/a.jpg",
            "https://cdn.plnspttrs.net/a.jpg",
            "https://evil.example/a.jpg",
            "https://t.plnspttrs.net@evil.example/a.jpg",
        ] {
            assert!(r.image(bad).is_none(), "{bad}");
        }
    }

    #[test]
    fn the_weather_asks_the_upstream_once_per_call_and_shares_the_ceiling() {
        struct Sunny(AtomicUsize);
        impl Upstream for Sunny {
            fn fetch(&self, _: &Request) -> Reply {
                Reply::Photo(Lookup::Failed)
            }
            fn weather(&self, _: &Spot) -> Option<Weather> {
                self.0.fetch_add(1, Ordering::SeqCst);
                crate::parse_forecast(crate::weather::FORECAST)
            }
        }
        let spot = Spot::new(52.31, 4.76).unwrap();
        let r = Relay::new(Sunny(AtomicUsize::new(0)), 2, OnlineCache::default());
        assert!(matches!(r.weather(NOW, &spot), Outlook::Got(w) if w.utc_offset_s == 3600));
        assert!(matches!(r.weather(NOW, &spot), Outlook::Got(_)));
        assert_eq!(r.weather(NOW, &spot), Outlook::Busy, "the third in a minute is turned away");
        assert_eq!(r.upstream.0.load(Ordering::SeqCst), 2, "a refused one never went out");
        assert!(matches!(r.weather(NOW + 60, &spot), Outlook::Got(_)), "a minute later it may");
    }

    #[test]
    fn an_upstream_with_no_weather_is_a_failure_not_a_panic() {
        let up = Fake::working();
        let r = Relay::new(up, 10, OnlineCache::default());
        assert_eq!(r.weather(NOW, &Spot::new(0.0, 0.0).unwrap()), Outlook::Failed);
    }

    fn relay(up: &Arc<Fake>) -> Relay<Arc<Fake>> {
        Relay::new(up.clone(), 1000, OnlineCache::default())
    }

    #[test]
    fn a_lookup_gives_the_airframe_the_route_and_the_photo() {
        let up = Fake::working();
        let info = relay(&up).lookup(NOW, 0x484006, Some("KLM1234"), None);
        assert_eq!((info.v, info.status), (FORMAT_VERSION, Status::Ok));
        assert_eq!(info.aircraft.unwrap().type_name.as_deref(), Some("A320-214"));
        assert_eq!(info.route.unwrap().destination.unwrap().code, "LHR");
        let p = info.photo.unwrap();
        assert_eq!(p.thumb_url, "https://t.plnspttrs.net/a/large.jpg");
        assert_eq!(p.photographer, "Jan Jansen");
        assert_eq!((info.fetched_at, info.cached), (Some(NOW), false));
    }

    #[test]
    fn the_second_asker_is_answered_from_the_cache_without_a_question_out() {
        let up = Fake::working();
        let r = relay(&up);
        r.lookup(NOW, 0x484006, Some("KLM1234"), None);
        let asked = up.count();
        let again = r.lookup(NOW + 3600, 0x484006, Some("KLM1234"), None);
        assert_eq!(up.count(), asked, "nothing went out the second time");
        assert!(again.cached && again.photo.is_some());
        assert_eq!(again.fetched_at, Some(NOW));
    }

    #[test]
    fn after_24_hours_the_question_goes_out_again() {
        let up = Fake::working();
        let r = relay(&up);
        r.lookup(NOW, 0x484006, None, None);
        let asked = up.count();
        r.lookup(NOW + crate::TTL_S, 0x484006, None, None);
        assert!(up.count() > asked);
    }

    #[test]
    fn an_aircraft_adsbdb_does_not_know_is_unknown_and_remembered() {
        let up = Fake::new(|_| Reply::Adsbdb(Lookup::NotFound));
        let r = relay(&up);
        let info = r.lookup(NOW, 0x1, None, None);
        assert_eq!(info.status, Status::Unknown);
        assert!(info.aircraft.is_none() && info.photo.is_none());
        r.lookup(NOW + 10, 0x1, None, None);
        assert_eq!(up.count(), 1);
    }

    #[test]
    fn a_callsign_adsbdb_does_not_know_still_gives_the_airframe() {
        let up = Fake::new(|r| match r {
            Request::Adsbdb { callsign: Some(_), .. } => Reply::Adsbdb(Lookup::NotFound),
            Request::Adsbdb { callsign: None, .. } => Reply::Adsbdb(Lookup::Found(Adsbdb {
                aircraft: Some(AircraftInfo { type_name: Some("R44".into()), ..Default::default() }),
                route: None,
            })),
            Request::Photo { .. } => Reply::Photo(Lookup::NotFound),
        });
        let r = relay(&up);
        let info = r.lookup(NOW, 0x484006, Some("ZZZ9999"), None);
        assert_eq!(info.status, Status::Ok);
        assert_eq!(info.aircraft.unwrap().type_name.as_deref(), Some("R44"));
        assert!(info.route.is_none());
        assert_eq!(up.count(), 2, "the flight, then the airframe alone");
        r.lookup(NOW + 5, 0x484006, Some("ZZZ9999"), None);
        assert_eq!(up.count(), 2, "both answers are cached");
    }

    #[test]
    fn an_aircraft_unknown_with_and_without_its_callsign_is_unknown() {
        let up = Fake::new(|_| Reply::Adsbdb(Lookup::NotFound));
        let info = relay(&up).lookup(NOW, 0x1, Some("ZZZ9999"), None);
        assert_eq!(info.status, Status::Unknown);
    }

    #[test]
    fn a_failure_is_unavailable_and_is_not_retried_for_five_minutes() {
        let up = Fake::new(|r| match r {
            Request::Adsbdb { .. } => Reply::Adsbdb(Lookup::Failed),
            Request::Photo { .. } => Reply::Photo(Lookup::Failed),
        });
        let r = relay(&up);
        assert_eq!(r.lookup(NOW, 0x484006, None, None).status, Status::Unavailable);
        assert_eq!(
            r.lookup(NOW + crate::BACKOFF_S - 1, 0x484006, None, None).status,
            Status::Unavailable
        );
        assert_eq!(up.count(), 1, "inside the back-off nothing goes out");
        r.lookup(NOW + crate::BACKOFF_S, 0x484006, None, None);
        assert_eq!(up.count(), 2);
    }

    #[test]
    fn the_registration_the_radio_heard_wins_over_the_one_adsbdb_gives() {
        let up = Fake::working();
        relay(&up).lookup(NOW, 0x484006, None, Some("ph-zzz"));
        assert_eq!(up.photo_asked(), vec!["PH-ZZZ".to_string()]);
        let up = Fake::working();
        relay(&up).lookup(NOW, 0x484006, None, None);
        assert_eq!(
            up.photo_asked(),
            vec!["PH-BXA".to_string()],
            "adsbdb's, when the radio has none"
        );
    }

    #[test]
    fn a_registration_that_could_carry_anything_else_is_not_sent_on() {
        let up = Fake::new(|r| match r {
            Request::Adsbdb { .. } => Reply::Adsbdb(Lookup::NotFound),
            Request::Photo { .. } => Reply::Photo(Lookup::NotFound),
        });
        relay(&up).lookup(NOW, 0x1, Some("KLM 1&x"), Some("PH-BXA/../x"));
        let calls = up.calls.lock().unwrap();
        assert_eq!(*calls, vec![Request::Adsbdb { icao: 1, callsign: None }]);
    }

    #[test]
    fn no_registration_means_no_photo_question() {
        let up = Fake::new(|_| {
            Reply::Adsbdb(Lookup::Found(
                parse_adsbdb(r#"{"response":{"aircraft":{"type":"A320"}}}"#).unwrap(),
            ))
        });
        let info = relay(&up).lookup(NOW, 0x1, None, None);
        assert_eq!(up.photo_asked(), Vec::<String>::new());
        assert!(info.photo.is_none() && info.status == Status::Ok);
    }

    #[test]
    fn the_address_is_masked_to_24_bits() {
        let up = Fake::working();
        relay(&up).lookup(NOW, 0x1_484006, None, None);
        assert_eq!(up.calls.lock().unwrap()[0], Request::Adsbdb { icao: 0x484006, callsign: None });
    }

    #[test]
    fn past_the_ceiling_a_question_is_busy_at_once_and_nothing_more_goes_out() {
        let up = Fake::new(|_| Reply::Adsbdb(Lookup::NotFound));
        let r = Relay::new(up.clone(), 2, OnlineCache::default());
        assert_eq!(r.lookup(NOW, 1, None, None).status, Status::Unknown);
        assert_eq!(r.lookup(NOW, 2, None, None).status, Status::Unknown);
        assert_eq!(r.lookup(NOW + 1, 3, None, None).status, Status::Busy);
        assert_eq!(up.count(), 2);
        // A key already answered is not refused by the ceiling.
        assert_eq!(r.lookup(NOW + 1, 1, None, None).status, Status::Unknown);
        // ...and a minute later the ceiling has room again.
        assert_eq!(r.lookup(NOW + 60, 3, None, None).status, Status::Unknown);
        assert_eq!(up.count(), 3);
    }

    #[test]
    fn a_busy_answer_carries_nothing_even_when_a_photo_is_already_known() {
        let up = Fake::working();
        let r = Relay::new(up.clone(), 2, OnlineCache::default());
        // Two questions out (airframe and photo) use the whole ceiling, and the
        // photo for PH-ZZZ is now in the cache.
        assert!(r.lookup(NOW, 1, None, Some("PH-ZZZ")).photo.is_some());
        let busy = r.lookup(NOW + 1, 2, None, Some("PH-ZZZ"));
        assert_eq!(busy.status, Status::Busy);
        assert!(busy.photo.is_none() && busy.aircraft.is_none(), "{busy:?}");
    }

    #[test]
    fn a_refused_question_does_not_wedge_the_next_try() {
        let up = Fake::new(|_| Reply::Adsbdb(Lookup::NotFound));
        let r = Relay::new(up.clone(), 0, OnlineCache::default());
        assert_eq!(r.lookup(NOW, 1, None, None).status, Status::Busy);
        assert_eq!(r.lookup(NOW, 1, None, None).status, Status::Busy, "not left 'on its way'");
        assert_eq!(up.count(), 0);
    }

    #[test]
    fn many_clients_asking_at_once_cost_one_question() {
        let up = Arc::new(Fake {
            calls: Mutex::new(Vec::new()),
            reply: Box::new(|r| match r {
                Request::Adsbdb { .. } => {
                    Reply::Adsbdb(Lookup::Found(parse_adsbdb(fixtures::ADSBDB).unwrap()))
                }
                Request::Photo { .. } => Reply::Photo(Lookup::NotFound),
            }),
            delay: Duration::from_millis(150),
        });
        let r = Arc::new(Relay::new(up.clone(), 1000, OnlineCache::default()));
        let handles: Vec<_> = (0..8)
            .map(|_| {
                let r = r.clone();
                std::thread::spawn(move || r.lookup(NOW, 0x484006, Some("KLM1234"), None))
            })
            .collect();
        let infos: Vec<Info> = handles.into_iter().map(|h| h.join().unwrap()).collect();
        assert!(infos.iter().all(|i| i.status == Status::Ok), "{infos:?}");
        let adsb =
            up.calls.lock().unwrap().iter().filter(|r| matches!(r, Request::Adsbdb { .. })).count();
        assert_eq!(adsb, 1, "eight clients, one question to adsbdb");
    }

    #[test]
    fn a_client_does_not_wait_for_ever_on_a_question_that_never_comes_back() {
        let (release, stuck) = mpsc::channel::<()>();
        let stuck = Mutex::new(stuck);
        let up = Fake::new(move |_| {
            let _ = stuck.lock().unwrap().recv_timeout(Duration::from_secs(5));
            Reply::Adsbdb(Lookup::NotFound)
        });
        let r = Arc::new(
            Relay::new(up.clone(), 1000, OnlineCache::default())
                .with_wait(Duration::from_millis(100)),
        );
        let first = {
            let r = r.clone();
            std::thread::spawn(move || r.lookup(NOW, 1, None, None))
        };
        std::thread::sleep(Duration::from_millis(50));
        let started = Instant::now();
        let second = r.lookup(NOW, 1, None, None);
        assert_eq!(second.status, Status::Unavailable);
        assert!(started.elapsed() < Duration::from_secs(2));
        release.send(()).unwrap();
        assert_eq!(first.join().unwrap().status, Status::Unknown);
    }

    #[test]
    fn a_panic_in_the_sender_does_not_leave_the_key_on_its_way() {
        let tripped = AtomicUsize::new(0);
        let up = Fake::new(move |_| {
            if tripped.fetch_add(1, Ordering::SeqCst) == 0 {
                panic!("the sender broke");
            }
            Reply::Adsbdb(Lookup::NotFound)
        });
        let r = Arc::new(relay(&up));
        let first = {
            let r = r.clone();
            std::thread::spawn(move || r.lookup(NOW, 1, None, None)).join()
        };
        assert!(first.is_err());
        assert_eq!(r.lookup(NOW, 1, None, None).status, Status::Unknown);
    }

    #[test]
    fn the_file_is_due_at_most_once_a_minute_and_only_when_there_is_something_new() {
        let up = Fake::working();
        let r = relay(&up);
        assert!(r.snapshot_if_due(NOW).is_none(), "nothing to write yet");
        r.lookup(NOW, 0x484006, None, None);
        let json = r.snapshot_if_due(NOW + 1).expect("the first answer is written at once");
        assert!(json.contains("A320-214"));
        assert!(r.snapshot_if_due(NOW + 2).is_none(), "nothing new since");
        r.lookup(NOW + 3, 0x484007, None, None);
        assert!(r.snapshot_if_due(NOW + 30).is_none(), "new, but written a moment ago");
        assert!(r.snapshot_if_due(NOW + 62).is_some(), "a minute later it goes");
    }

    #[test]
    fn a_relay_started_from_a_file_answers_from_it() {
        let up = Fake::working();
        let first = relay(&up);
        first.lookup(NOW, 0x484006, Some("KLM1234"), None);
        let json = first.snapshot_if_due(NOW + 61).unwrap();
        let up2 = Fake::working();
        let second = Relay::new(up2.clone(), 1000, OnlineCache::from_json(&json, NOW + 100));
        let info = second.lookup(NOW + 100, 0x484006, Some("KLM1234"), None);
        assert!(info.cached && info.status == Status::Ok);
        assert_eq!(up2.count(), 0);
    }

    #[test]
    fn the_answer_is_json_with_a_version_and_a_lowercase_status() {
        let info = Info::bare(Status::Disabled);
        let j = serde_json::to_string(&info).unwrap();
        assert!(j.contains("\"v\":1") && j.contains("\"status\":\"disabled\""), "{j}");
        let back: Info = serde_json::from_str(&j).unwrap();
        assert_eq!(back, info);
        for (s, w) in [
            (Status::Ok, "ok"),
            (Status::Unknown, "unknown"),
            (Status::Unavailable, "unavailable"),
            (Status::Busy, "busy"),
            (Status::Invalid, "invalid"),
            (Status::Unauthorized, "unauthorized"),
        ] {
            assert_eq!(serde_json::to_string(&s).unwrap(), format!("\"{w}\""));
        }
    }
}
