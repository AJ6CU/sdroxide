//! Subscribed element-set listings: TLEs fetched from a URL and kept current.
//!
//! Pasted elements go stale, and SGP4 on a fortnight-old TLE is worth very
//! little — [`crate::satellites::MAX_ELEMENT_AGE_S`] is where this crate stops
//! propagating them at all. Anything the operator means to keep tracking wants
//! a subscription instead of a paste.
//!
//! Two callers, one implementation:
//!
//! * The feed's worker refreshes subscriptions on the same six-hourly cadence
//!   as the amateur element set, while the solar window is open.
//! * The settings dialog fetches them on demand, so a subscription added there
//!   is usable immediately rather than at the next time the window happens to
//!   be open.
//!
//! Both go through the same disk cache as every other source, so a listing
//! fetched by either is served instantly — and offline — to the other.

use sdroxide_types::TleSubscription;

use crate::cache::{Cache, Validators};
use crate::satellites::Satellite;

/// Body cap. The largest CelesTrak group is a few hundred kilobytes; a
/// megabyte is generous and still bounds a misconfigured URL.
const BODY_LIMIT: u64 = 4 * 1024 * 1024;

/// What one subscription's last fetch did, for the settings dialog.
///
/// Declared in `sdroxide-types` rather than here because it crosses the wire:
/// a remote settings dialog edits the engine host's subscriptions, so the
/// engine host is the only place that knows what each listing last yielded.
pub use sdroxide_types::TleSubStatus as SubStatus;

/// Cache file name for a subscription's body.
///
/// Keyed by a hash of the URL rather than by the operator's name for it:
/// renaming a subscription must not orphan its cached listing, and two
/// subscriptions cannot be given the same name and collide.
fn cache_name(url: &str) -> String {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    url.trim().hash(&mut h);
    format!("tlesub_{:016x}.txt", h.finish())
}

/// The satellites a subscription is currently tracking, from the disk cache.
///
/// No network: this is what the tracker uses at startup and what it falls back
/// to when a refresh fails.
pub fn cached(cache: &Cache, sub: &TleSubscription) -> Vec<Satellite> {
    match cache.read_string(&cache_name(&sub.url)) {
        Some(text) => parse(sub, &text),
        None => Vec::new(),
    }
}

/// A subscription's cached listing, exactly as it was fetched.
///
/// For the browser relay, which forwards the original text rather than the
/// parsed SGP4 constants — those cannot be serialised.
pub fn cached_text(cache: &Cache, sub: &TleSubscription) -> Option<String> {
    cache.read_string(&cache_name(&sub.url))
}

/// Status of a subscription from the cache alone, so the settings dialog can
/// report on one without opening a socket.
pub fn cached_status(cache: &Cache, sub: &TleSubscription) -> SubStatus {
    let sats = cached(cache, sub);
    SubStatus {
        url: sub.url.clone(),
        fetched_unix: cache.fetched_at(&sub.url),
        count: sats.len(),
        curated: curated_count(&sats),
        error: None,
    }
}

/// How many of these are in [`crate::satellites::POPULAR`].
///
/// Counted from the catalogue numbers rather than from `popular`, which the
/// subscription's own setting has already overwritten by this point.
fn curated_count(sats: &[Satellite]) -> usize {
    sats.iter()
        .filter(|s| crate::satellites::POPULAR.iter().any(|(id, _)| *id == s.norad_id))
        .count()
}

/// Parse a fetched listing into the satellites this subscription wants.
///
/// Everything that comes through is flagged [`Satellite::custom`] — it is here
/// because the operator asked for it — and `popular`, which is what decides
/// whether the scene draws a ring and a label, comes from the subscription's
/// orbit-ring setting.
///
/// `parse_subscribed_tles` has already set `popular` from
/// [`crate::satellites::POPULAR`], which is what [`OrbitRings::Curated`] keeps.
/// The setting is otherwise the last word: with it on `None`, nothing from this
/// listing is ringed, including the curated few. Anything else would leave part
/// of the listing lit up with no control that turns it off.
fn parse(sub: &TleSubscription, text: &str) -> Vec<Satellite> {
    let mut v = crate::satellites::parse_subscribed_tles(text);
    v.retain(|s| sub.wants(s.norad_id));
    for s in &mut v {
        s.popular = sub.orbits.rings(s.popular);
    }
    v
}

/// Fetch one subscription, updating the cache. Returns what it now tracks.
///
/// A failed fetch is not a failed subscription: the cached listing is returned
/// with the error beside it, because a day-old TLE and no network still beats
/// an empty sky.
pub fn refresh(
    agent: &ureq::Agent,
    cache: &mut Cache,
    sub: &TleSubscription,
    now_unix: i64,
) -> (Vec<Satellite>, SubStatus) {
    let url = sub.url.trim().to_string();
    let name = cache_name(&url);
    let mut status = SubStatus { url: url.clone(), ..Default::default() };

    match crate::feed::http_get(agent, &url, &cache.validators(&url), BODY_LIMIT) {
        // 304: what is on disk is current.
        Ok(None) => {
            cache.touch(&url, now_unix);
            let sats = cached(cache, sub);
            status.fetched_unix = now_unix;
            status.count = sats.len();
            status.curated = curated_count(&sats);
            (sats, status)
        }
        Ok(Some((bytes, validators, _))) => {
            let text = match String::from_utf8(bytes) {
                Ok(t) => t,
                Err(_) => {
                    status.error = Some("the listing is not text".into());
                    let sats = cached(cache, sub);
                    status.fetched_unix = cache.fetched_at(&url);
                    status.count = sats.len();
                    status.curated = curated_count(&sats);
                    return (sats, status);
                }
            };
            let sats = parse(sub, &text);
            // An empty parse means the URL served something that is not an
            // element set — an HTML error page is the usual culprit. Keeping
            // the previous body is better than caching the error page.
            if crate::satellites::parse_tles(&text).is_empty() {
                status.error = Some("no element sets in the response".into());
                let old = cached(cache, sub);
                status.fetched_unix = cache.fetched_at(&url);
                status.count = old.len();
                status.curated = curated_count(&old);
                return (old, status);
            }
            cache.write(
                &name,
                &url,
                text.as_bytes(),
                Validators { fetched_unix: now_unix, ..validators },
            );
            status.fetched_unix = now_unix;
            status.count = sats.len();
            status.curated = curated_count(&sats);
            (sats, status)
        }
        Err(e) => {
            tracing::warn!("TLE subscription {url}: {e}");
            let sats = cached(cache, sub);
            status.error = Some(e);
            status.fetched_unix = cache.fetched_at(&url);
            status.count = sats.len();
            status.curated = curated_count(&sats);
            (sats, status)
        }
    }
}

/// Fetch every enabled subscription. Used by the settings dialog's "Update
/// now", which has no feed thread to ask.
///
/// Blocking, and up to one HTTPS round trip per subscription — call it off the
/// paint path, the way the HPSDR discovery button does.
pub fn refresh_all(subs: &[TleSubscription]) -> Vec<SubStatus> {
    let agent = agent();
    let mut cache = Cache::open();
    let now = crate::feed::now_unix();
    subs.iter()
        .filter(|s| s.enabled && s.is_valid())
        .map(|s| refresh(&agent, &mut cache, s, now).1)
        .collect()
}

/// Status of every subscription from the disk cache alone — no network.
pub fn status_all(subs: &[TleSubscription]) -> Vec<SubStatus> {
    let cache = Cache::open();
    subs.iter().map(|s| cached_status(&cache, s)).collect()
}

/// An HTTP agent with the same timeout and user agent as the feed's.
pub(crate) fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_global(Some(std::time::Duration::from_secs(25)))
        .user_agent(concat!("sdroxide/", env!("CARGO_PKG_VERSION")))
        .build()
        .into()
}

#[cfg(test)]
mod tests {
    use super::*;

    const AMATEUR: &str = include_str!("../tests/fixtures/amateur.txt");

    fn sub() -> TleSubscription {
        TleSubscription::new("Test", "https://example.invalid/tle.txt")
    }

    /// A group subscription tracks the whole listing, and the orbit-ring
    /// setting decides how much of it is drawn with a ring and a label. All
    /// three positions have to do what they say — the middle one exists because
    /// ninety rings is unreadable and none at all leaves ninety anonymous dots.
    #[test]
    fn the_orbit_setting_decides_which_of_a_listing_is_ringed() {
        use sdroxide_types::OrbitRings;

        let mut s = sub();
        s.orbits = OrbitRings::Curated;
        let v = parse(&s, AMATEUR);
        assert!(v.len() > 80, "only {} satellites", v.len());
        assert!(v.iter().all(|x| x.custom), "a subscribed satellite is operator-supplied");
        let curated = v.iter().filter(|x| x.popular).count();
        assert!((1..15).contains(&curated), "{curated} ringed, expected the curated handful");
        assert!(v.iter().find(|x| x.norad_id == 25544).is_some_and(|x| x.popular), "no ISS ring");

        // Off means off — including for the curated few. Leaving them lit with
        // no control that turns them off is the bug this position exists for.
        s.orbits = OrbitRings::None;
        let v = parse(&s, AMATEUR);
        assert!(v.iter().all(|x| !x.popular), "something stayed ringed with orbits off");
        assert!(v.iter().all(|x| x.custom), "they are still tracked, just not ringed");

        // All means all.
        s.orbits = OrbitRings::All;
        let v = parse(&s, AMATEUR);
        assert!(v.iter().all(|x| x.popular));
    }

    #[test]
    fn a_filter_narrows_it_to_the_listed_catalogue_numbers() {
        let mut s = sub();
        s.only = vec![25544, 43700];
        s.orbits = sdroxide_types::OrbitRings::All;
        let v = parse(&s, AMATEUR);
        assert_eq!(v.len(), 2, "the filter let something else through");
        let mut ids: Vec<u64> = v.iter().map(|s| s.norad_id).collect();
        ids.sort_unstable();
        assert_eq!(ids, vec![25544, 43700]);
        // A short, deliberately chosen list is worth ringing and labelling.
        assert!(v.iter().all(|s| s.popular && s.custom));
        // A catalogue number the listing does not have is simply absent rather
        // than an error — a group's membership changes under you.
        let mut s = sub();
        s.only = vec![999_999];
        assert!(parse(&s, AMATEUR).is_empty());
    }

    /// The filter box takes whatever the operator types at it.
    #[test]
    fn the_filter_box_round_trips_through_its_text() {
        let mut s = sub();
        s.set_only_text("25544, 43700 33591");
        assert_eq!(s.only, vec![25544, 43700, 33591]);
        assert_eq!(s.only_text(), "25544, 43700, 33591");
        // Junk is dropped rather than clearing the filter or failing.
        s.set_only_text("25544, , oops, 7530,");
        assert_eq!(s.only, vec![25544, 7530]);
        // Emptied means "everything" again.
        s.set_only_text("   ");
        assert!(s.only.is_empty());
        assert!(s.wants(1) && s.wants(2));
    }

    /// The curated position only means something for a listing that contains
    /// curated satellites — ten amateur ones — and the settings dialog greys it
    /// out on the strength of this count, so it has to be right.
    #[test]
    fn the_curated_count_is_what_makes_that_position_meaningful() {
        let sats = crate::satellites::parse_subscribed_tles(AMATEUR);
        let n = curated_count(&sats);
        // The amateur group carries the curated set, bar any that have decayed
        // out of it.
        assert!((5..=crate::satellites::POPULAR.len()).contains(&n), "{n} curated in the group");
        // A listing with none of them counts zero, which is what greys the
        // position out.
        let none: Vec<_> = sats.into_iter().filter(|s| !s.popular).collect();
        assert_eq!(curated_count(&none), 0);
        assert_eq!(curated_count(&[]), 0);
    }

    #[test]
    fn a_subscription_says_what_is_wrong_with_it() {
        assert_eq!(sub().problem(), None);
        assert_eq!(TleSubscription::new("", "https://x/y").problem(), Some("no name"));
        assert_eq!(TleSubscription::new("n", "").problem(), Some("no URL"));
        // Element sets fetched in clear would put both the data and the fact of
        // the request on the wire.
        assert_eq!(
            TleSubscription::new("n", "http://x/y").problem(),
            Some("the URL must be https://")
        );
    }

    /// Renaming a subscription must not orphan its cached listing, and two
    /// different URLs must not land on the same file.
    #[test]
    fn the_cache_name_follows_the_url_and_nothing_else() {
        let a = cache_name("https://celestrak.org/x?GROUP=weather");
        assert_eq!(a, cache_name(" https://celestrak.org/x?GROUP=weather "));
        assert_ne!(a, cache_name("https://celestrak.org/x?GROUP=cubesat"));
        // It ends up in a path, so it has to stay a plain file name.
        assert!(!a.contains('/') && !a.contains('.') || a.ends_with(".txt"));
        assert!(a.starts_with("tlesub_") && a.ends_with(".txt"));
    }

    /// Every offered group has to be a fetchable https URL with a distinct
    /// address, and the two that are on by default have to be the ones the
    /// tracker used to fetch by itself — otherwise a fresh install comes up
    /// with an empty sky.
    #[test]
    fn the_offered_celestrak_groups_are_usable() {
        use sdroxide_types::CELESTRAK_GROUPS;
        let mut urls: Vec<&str> = Vec::new();
        for g in CELESTRAK_GROUPS {
            assert!(!g.name.is_empty() && !g.hint.is_empty(), "{} is missing its text", g.name);
            assert_eq!(
                TleSubscription::new(g.name, g.url).problem(),
                None,
                "{}: {}",
                g.name,
                g.url
            );
            urls.push(g.url);
        }
        let n = urls.len();
        urls.sort_unstable();
        urls.dedup();
        assert_eq!(urls.len(), n, "two groups share a URL");

        let on: Vec<&str> =
            CELESTRAK_GROUPS.iter().filter(|g| g.default_on).map(|g| g.name).collect();
        assert_eq!(on, vec!["Amateur radio", "ISS"]);
        // The amateur one has to be the listing the tracker used to fetch, or a
        // fresh install would quietly track a different set of satellites.
        let amateur = CELESTRAK_GROUPS.iter().find(|g| g.name == "Amateur radio").unwrap();
        assert_eq!(amateur.url, crate::satellites::AMATEUR_URL);
        assert_eq!(
            amateur.orbits,
            sdroxide_types::OrbitRings::Curated,
            "ninety rings is unreadable and none leaves ninety anonymous dots"
        );
        // The ISS is one satellite, so it is worth a ring and a label.
        let iss = CELESTRAK_GROUPS.iter().find(|g| g.name == "ISS").unwrap();
        assert!(iss.url.contains("CATNR=25544"));
        assert_eq!(iss.orbits, sdroxide_types::OrbitRings::All);
    }
}
