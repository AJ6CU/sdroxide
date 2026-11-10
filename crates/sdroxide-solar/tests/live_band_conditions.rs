//! Live check that HamQSL still publishes what the parser expects.
//! Ignored by default (needs network); run with:
//!   cargo test -p sdroxide-solar --test live_band_conditions -- --ignored --nocapture
//!
//! One request. The publisher asks for hourly polling at most and this is a
//! test somebody runs by hand, but the point is worth keeping in view: do not
//! put this in a loop and do not put it in CI.

use sdroxide_solar::indices::{BAND_CONDITIONS_URL, parse_band_conditions};
use sdroxide_types::Band;

#[test]
#[ignore = "hits N0NBH's live server"]
fn the_published_feed_still_has_the_shape_we_parse() {
    let xml = ureq::get(BAND_CONDITIONS_URL)
        .call()
        .expect("fetch")
        .body_mut()
        .read_to_string()
        .expect("body");
    println!("{} bytes", xml.len());

    let c = parse_band_conditions(&xml).expect("the live document must parse");
    println!("updated {} — {:?}", c.observed_unix, c.hf);

    assert!(c.observed_unix > 1_700_000_000, "no usable update stamp");
    // The four groups, both halves of the day. If the publisher regroups, this
    // is what should fail — quietly showing three bands out of eight would not.
    assert_eq!(c.hf.len(), 8, "the published group set changed: {:?}", c.hf);
    for b in
        [Band::M80, Band::M40, Band::M30, Band::M20, Band::M17, Band::M15, Band::M12, Band::M10]
    {
        assert!(c.for_band(b, true).is_some(), "{b:?} lost its daytime verdict");
        assert!(c.for_band(b, false).is_some(), "{b:?} lost its night verdict");
    }
    assert!(!c.vhf.is_empty(), "the VHF phenomena disappeared");
}

/// The path the band menu actually uses: fetch, cache, then serve from cache.
///
/// The second call must not go to the network — that is the whole reason the
/// publisher's hourly limit is honoured without the caller having to think
/// about it — and it must return the same verdicts, not an empty document.
#[test]
#[ignore = "hits N0NBH's live server (once)"]
fn the_cached_fetch_serves_the_second_call_without_asking_again() {
    let first = sdroxide_solar::band_conditions_cached().expect("first fetch");
    let t0 = std::time::Instant::now();
    let second = sdroxide_solar::band_conditions_cached().expect("second read");
    let elapsed = t0.elapsed();

    println!("second call took {elapsed:?}");
    assert_eq!(first, second, "the cached copy differs from what was fetched");
    assert!(
        elapsed < std::time::Duration::from_millis(200),
        "the second call took {elapsed:?} — it went to the network instead of the cache"
    );
    assert!(first.for_band(Band::M20, true).is_some(), "20 m came back without a verdict");
}

/// A cached document that has gone missing must be refetched, not skipped.
///
/// The index and the document are two files. If the first says "fetched five
/// minutes ago" about a second that was deleted, truncated or half written,
/// honouring the interval would blank the band menu for an hour over a file
/// that could have been fetched in a second. Found by deleting the file by hand
/// and watching the band menu stay empty.
#[test]
#[ignore = "hits N0NBH's live server and touches the real solar cache"]
fn a_missing_cache_file_is_refetched_rather_than_reported_as_fresh() {
    // Prime the cache, so the index says "fresh".
    sdroxide_solar::band_conditions_cached().expect("prime");

    let path = sdroxide_config::solar_cache_dir().expect("cache dir").join("hamqsl.xml");
    assert!(path.exists(), "the prime did not write {path:?}");
    std::fs::remove_file(&path).expect("remove the cached document");

    let again = sdroxide_solar::band_conditions_cached();
    assert!(again.is_some(), "a missing cached document was reported as nothing to show");
    assert!(path.exists(), "the document was not written back");
}
