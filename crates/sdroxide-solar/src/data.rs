//! The snapshot the 3D view reads, and the per-source freshness bookkeeping
//! around it.
//!
//! Split out of [`crate::feed`] because both targets need these types: the
//! native build fills them from the network worker, the browser build fills
//! them from the server's relay. Nothing in this module does I/O.
//!
//! [`SolarData`] deliberately has no `serde` derive. It holds a decoded
//! [`SunImage`] (megabytes of RGBA) and [`Satellite`]s (which wrap SGP4
//! constants), neither of which belongs on a wire; the transport ships the
//! original JPEG and the original element set instead, and the receiver
//! rebuilds this struct locally.

use std::sync::Arc;

use crate::aurora::{AuroraOval, HemisphericPower, KpPoint};
use crate::clouds::{self, CloudField};
use crate::donki::{CmeEvent, FlareEvent};
use crate::imagery::SunImage;
use crate::indices::SpaceWeather;
use crate::satellites::Satellite;
use crate::swpc::ActiveRegion;

/// Failure backoff bounds, seconds.
#[cfg(not(target_arch = "wasm32"))]
const BACKOFF_BASE: i64 = 30;
#[cfg(not(target_arch = "wasm32"))]
const BACKOFF_MAX: i64 = 1800;

/// The things the feed fetches, each on its own cadence.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Source {
    Sun,
    Cme,
    Flare,
    Regions,
    /// 10.7 cm solar flux.
    Flux,
    /// Planetary K and A indices.
    Kp,
    /// Current GOES soft X-ray class.
    Xray,
    /// Ionosonde soundings, for the MUF estimate.
    Muf,
    /// QO-100, which is absent from the amateur set (see
    /// [`crate::satellites::QO100_URL`]).
    ///
    /// The only element set still fetched as a fixed source. The amateur group
    /// used to be one too and is now a subscription like any other, so that the
    /// operator can switch it off, filter it, or point it somewhere else —
    /// see [`sdroxide_types::CELESTRAK_GROUPS`].
    SatGeo,
    /// The OVATION auroral-oval grid.
    Aurora,
    /// Auroral hemispheric power, gigawatts.
    AuroraPower,
    /// Three days of predicted planetary K.
    KpForecast,
    /// The longwave infrared half of the global cloud mosaic.
    Clouds,
    /// The visible half of it: the low cloud the infrared cannot see, on
    /// whichever side of the planet the Sun is up.
    CloudsVis,
    /// N0NBH's calculated band conditions — the only source here that is one
    /// person's server rather than an institution's, and the only one whose
    /// publisher states a polling limit. See [`crate::indices::BAND_CONDITIONS_URL`].
    ///
    /// Appended last: `SourceStatus` travels to the browser as a vector indexed
    /// by [`Source::index`], so the position of every existing entry is part of
    /// the wire format.
    BandConditions,
}

impl Source {
    pub const ALL: [Source; 15] = [
        Source::Sun,
        Source::Cme,
        Source::Flare,
        Source::Regions,
        Source::Flux,
        Source::Kp,
        Source::Xray,
        Source::Muf,
        Source::SatGeo,
        Source::Aurora,
        Source::AuroraPower,
        Source::KpForecast,
        Source::Clouds,
        Source::CloudsVis,
        Source::BandConditions,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Source::Sun => "SDO",
            Source::Cme => "CME",
            Source::Flare => "FLR",
            Source::Regions => "SWPC",
            Source::Flux => "F10.7",
            Source::Kp => "Kp",
            Source::Xray => "XRAY",
            Source::Muf => "MUF",
            Source::SatGeo => "QO-100",
            Source::Aurora => "OVATION",
            Source::AuroraPower => "HPI",
            Source::KpForecast => "Kp+3d",
            Source::Clouds => "CLOUD",
            Source::CloudsVis => "CLOUD/V",
            Source::BandConditions => "BANDS",
        }
    }

    pub fn index(self) -> usize {
        Source::ALL.iter().position(|s| *s == self).unwrap_or(0)
    }
}

/// Fetch scheduling. Only the thing doing the fetching schedules, so this is
/// the native feed's policy and none of it reaches the browser, which is handed
/// finished data and just reads the freshness alongside it.
/// How often an element-set listing is refetched, seconds.
///
/// Shared by the geostationary source and by every subscribed listing: TLEs are
/// reissued a few times a day at most and SGP4 stays accurate for far longer, so
/// polling harder buys nothing.
#[cfg(not(target_arch = "wasm32"))]
pub const TLE_PERIOD_S: i64 = 21_600;

#[cfg(not(target_arch = "wasm32"))]
impl Source {
    /// Refresh cadence, seconds.
    pub(crate) fn period(self) -> i64 {
        match self {
            Source::Sun => 600, // browse images update every few minutes
            Source::Cme => 1200,
            Source::Flare => 1200,
            Source::Regions => 3600, // a once-a-day product
            Source::Flux => 3600,    // published a few times a day
            Source::Kp => 900,       // three-hourly, but revised in between
            Source::Xray => 300,     // a flare develops in minutes
            Source::Muf => 900,      // ionosondes sound every 5-15 minutes
            // Element sets are re-issued a few times a day; SGP4 stays accurate
            // for far longer than that, so there is no reason to poll hard.
            // [`TLE_PERIOD_S`] is the same figure for the subscribed listings,
            // which are not `Source`s.
            Source::SatGeo => TLE_PERIOD_S,
            // OVATION is issued every five minutes, but the grid is 900 kB —
            // by far the largest JSON here — and the oval does not move far in
            // half an hour. The overlay shows what the picture is valid for, so
            // a slow cadence costs honesty nothing.
            Source::Aurora => 1800,
            // The power figure is the number that says "something is
            // happening", and it is fifteen kilobytes.
            Source::AuroraPower => 900,
            // Issued three times a day.
            Source::KpForecast => 3600,
            // The mosaic is rebuilt hourly and there is no validator to ask
            // with, so this is a compromise: often enough that a new hour is on
            // the globe within a few minutes of being published, rarely enough
            // that most of the fetches that find nothing new are cheap ones.
            Source::Clouds => 600,
            Source::CloudsVis => 600,
            // The publisher's own limit, not a tuning choice: the HamQSL FAQ
            // asks for hourly at most, because the flux figures behind these
            // verdicts update hourly and because the feed runs on shared
            // hosting the author has said he will withdraw it from if it
            // attracts complaints. Do not shorten this.
            Source::BandConditions => 3600,
        }
    }

    /// A fixed stagger, so several never fall due on the same tick. Constant
    /// rather than random: reproducible, and it achieves the same thing.
    fn stagger(self) -> i64 {
        match self {
            Source::Sun => 0,
            Source::Cme => 7,
            Source::Flare => 13,
            Source::Regions => 23,
            Source::Flux => 31,
            Source::Kp => 41,
            Source::Xray => 53,
            Source::Muf => 61,
            Source::SatGeo => 83,
            Source::Aurora => 97,
            Source::AuroraPower => 103,
            Source::KpForecast => 109,
            Source::Clouds => 127,
            Source::CloudsVis => 137,
            Source::BandConditions => 149,
        }
    }
}

/// Per-source freshness, so the overlay can say "cached 4 h ago" instead of
/// presenting stale data as current.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SourceStatus {
    pub last_ok_unix: i64,
    pub last_error: Option<String>,
    pub consecutive_failures: u32,
    /// Last time a fetch was *attempted*, successful or not. Failures advance
    /// this even though they do not advance `last_ok_unix`, so a source that is
    /// down is not retried on every fifteen-second tick.
    ///
    /// `None`, not a zero sentinel: the epoch is a perfectly valid timestamp,
    /// and conflating the two makes "never attempted" and "attempted in 1970"
    /// indistinguishable.
    pub(crate) attempt_unix: Option<i64>,
}

impl SourceStatus {
    /// Age of the newest successful fetch, or `None` if there has never been one.
    pub fn age_secs(&self, now_unix: i64) -> Option<i64> {
        (self.last_ok_unix > 0).then(|| (now_unix - self.last_ok_unix).max(0))
    }
}

/// The other half of the fetch schedule — see the note on [`Source`]'s.
#[cfg(not(target_arch = "wasm32"))]
impl SourceStatus {
    /// When this source may next be tried, given its cadence and any backoff.
    fn next_due(&self, src: Source) -> Option<i64> {
        let last = self.attempt_unix?;
        let backoff = if self.consecutive_failures == 0 {
            0
        } else {
            (BACKOFF_BASE << (self.consecutive_failures - 1).min(6)).min(BACKOFF_MAX)
        };
        Some(last + src.period().max(backoff) + src.stagger())
    }

    /// Whether this source should be fetched now.
    pub(crate) fn is_due(&self, src: Source, now: i64) -> bool {
        self.next_due(src).is_none_or(|due| due <= now)
    }

    pub(crate) fn record_ok(&mut self, now: i64) {
        self.last_ok_unix = now;
        self.attempt_unix = Some(now);
        self.last_error = None;
        self.consecutive_failures = 0;
    }

    pub(crate) fn record_err(&mut self, now: i64, msg: String) {
        self.attempt_unix = Some(now);
        self.last_error = Some(msg);
        self.consecutive_failures = self.consecutive_failures.saturating_add(1);
    }
}

/// The snapshot the UI reads.
#[derive(Default)]
pub struct SolarData {
    pub cmes: Vec<CmeEvent>,
    pub flares: Vec<FlareEvent>,
    pub regions: Vec<ActiveRegion>,
    /// Shared by `Arc` so handing it to the renderer is a refcount bump, not a
    /// copy of several megabytes.
    pub sun: Option<Arc<SunImage>>,
    /// Bumped on every new image, so the GPU uploads once per image.
    pub sun_gen: u64,
    /// The propagation numbers: flux, K/A, X-ray level and ionosonde soundings.
    pub weather: SpaceWeather,
    /// Amateur satellites, split by source so each refresh replaces only its
    /// own set. Iterate with [`SolarData::satellites`].
    pub sats_amateur: Vec<Satellite>,
    pub sats_geo: Vec<Satellite>,
    /// Element sets the operator supplied themselves: pasted in, or fetched
    /// from a subscribed listing. Never replaced by one of the built-in
    /// sources — they are driven from the settings dialog.
    pub sats_custom: Vec<Satellite>,
    /// What each TLE subscription's last fetch did, in the order the settings
    /// dialog holds them. Empty when nothing is subscribed.
    #[cfg(not(target_arch = "wasm32"))]
    pub tle_subs: Vec<crate::tlesub::SubStatus>,
    /// The auroral oval. Shared by `Arc` for the same reason the solar image
    /// is: the renderer takes a handle, not a copy of the grid.
    pub aurora: Option<Arc<AuroraOval>>,
    /// Bumped on every new issue, so the GPU uploads once per grid.
    pub aurora_gen: u64,
    /// Power going into each auroral zone, gigawatts.
    pub aurora_power: Option<HemisphericPower>,
    /// Planetary K in three-hour bins, observed and then predicted.
    pub kp_forecast: Vec<KpPoint>,
    /// The cloud field the globe draws, and the storms in it.
    pub clouds: Option<Arc<CloudField>>,
    /// Bumped on every rebuild, so the GPU uploads once per mosaic.
    pub clouds_gen: u64,
    /// The two channels it is built from, kept because they arrive separately
    /// and each has to be able to refresh without the other.
    pub cloud_ir: Option<Arc<clouds::Plane>>,
    pub cloud_vis: Option<Arc<clouds::Plane>>,
    pub status: [SourceStatus; Source::ALL.len()],
}

impl SolarData {
    pub fn status(&self, src: Source) -> &SourceStatus {
        &self.status[src.index()]
    }

    /// Rebuild the cloud field from whichever channels have arrived.
    ///
    /// Called after either one lands. Infrared alone is a complete picture, so
    /// it is the one that gates: the visible channel on its own says nothing
    /// about how high anything is and covers half the planet, and drawing from
    /// it would be drawing half a globe.
    pub fn rebuild_clouds(&mut self) {
        let Some(ir) = self.cloud_ir.clone() else { return };
        // A visible frame from a different hour would put yesterday's daylight
        // over today's cloud. An hour of slack covers the two fetches falling
        // either side of a publication; more than that and it is dropped.
        let vis =
            self.cloud_vis.clone().filter(|v| (v.fetched_unix - ir.fetched_unix).abs() < 3600);
        self.clouds = Some(Arc::new(clouds::combine(&ir, vis.as_deref())));
        self.clouds_gen += 1;
    }

    /// True once anything at all has been loaded, from network or cache.
    pub fn has_any(&self) -> bool {
        self.sun.is_some()
            || self.aurora.is_some()
            || !self.cmes.is_empty()
            || !self.regions.is_empty()
    }

    /// Every tracked satellite, from all three element sets.
    ///
    /// The operator's own entries lead and shadow a fetched one for the same
    /// catalogue number: pasting in a fresher TLE for a satellite that is
    /// already in the amateur group has to replace it rather than put a second
    /// marker a few kilometres away from the first.
    pub fn satellites(&self) -> impl Iterator<Item = &Satellite> {
        self.sats_custom.iter().chain(
            self.sats_amateur
                .iter()
                .chain(&self.sats_geo)
                .filter(|s| !self.sats_custom.iter().any(|c| c.norad_id == s.norad_id)),
        )
    }

    /// The newest successful fetch across every source, for a single "data as
    /// of" readout.
    pub fn newest_ok_unix(&self) -> i64 {
        self.status.iter().map(|s| s.last_ok_unix).max().unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_empty_snapshot_reports_itself_empty() {
        let d = SolarData::default();
        assert!(!d.has_any());
        assert_eq!(d.status(Source::Cme).last_ok_unix, 0);
        assert!(d.status(Source::Sun).last_error.is_none());
        assert_eq!(d.status(Source::Sun).age_secs(1_784_937_600), None);
    }

    #[test]
    fn every_source_has_a_distinct_label_and_its_own_status_slot() {
        let labels: std::collections::HashSet<_> = Source::ALL.iter().map(|s| s.label()).collect();
        assert_eq!(labels.len(), Source::ALL.len(), "duplicate source labels");
        for src in Source::ALL {
            assert_eq!(Source::ALL[src.index()], src);
        }
    }
}

/// The fetch schedule, which exists only where there is a fetcher.
#[cfg(all(test, not(target_arch = "wasm32")))]
mod schedule_tests {
    use super::*;

    #[test]
    fn a_never_fetched_source_is_due_immediately() {
        let s = SourceStatus::default();
        for src in Source::ALL {
            assert_eq!(s.next_due(src), None, "{src:?} has a due time before any attempt");
            assert!(s.is_due(src, 0), "{src:?} not due at the epoch");
            assert!(s.is_due(src, 1_784_937_600), "{src:?} not due now");
        }
    }

    #[test]
    fn a_healthy_source_waits_its_cadence() {
        let mut s = SourceStatus::default();
        s.record_ok(1_000_000);
        assert_eq!(s.next_due(Source::Sun), Some(1_000_000 + 600));
        assert_eq!(s.next_due(Source::Regions), Some(1_000_000 + 3600 + 23));
        assert!(!s.is_due(Source::Sun, 1_000_100));
        assert!(s.is_due(Source::Sun, 1_000_700));
        assert_eq!(s.age_secs(1_000_100), Some(100));
    }

    /// A source that is down must back off, or a disconnected machine retries
    /// four URLs every fifteen seconds forever.
    #[test]
    fn failures_back_off_and_then_level_out() {
        let mut s = SourceStatus::default();
        let mut prev = 0;
        for attempt in 1..=10 {
            s.record_err(1_000_000, "boom".into());
            let wait = s.next_due(Source::Sun).unwrap() - 1_000_000 - Source::Sun.stagger();
            assert!(wait >= prev, "backoff went backwards at attempt {attempt}");
            assert!(wait <= BACKOFF_MAX, "backoff {wait} exceeded the cap");
            prev = wait;
        }
        assert_eq!(prev, BACKOFF_MAX, "backoff never reached its cap");
        assert_eq!(s.consecutive_failures, 10);
        assert!(s.last_error.is_some());

        // One success clears it.
        s.record_ok(1_000_100);
        assert_eq!(s.consecutive_failures, 0);
        assert!(s.last_error.is_none());
        assert_eq!(s.next_due(Source::Sun), Some(1_000_100 + 600));
    }

    /// With a dozen sources on a fifteen-second tick, two falling due together
    /// means two requests in the same instant; the staggers exist to prevent it.
    #[test]
    fn no_two_sources_ever_fall_due_together() {
        let mut s = SourceStatus::default();
        s.record_ok(0);
        let dues: Vec<i64> = Source::ALL.iter().filter_map(|src| s.next_due(*src)).collect();
        assert_eq!(dues.len(), Source::ALL.len());
        let unique: std::collections::HashSet<_> = dues.iter().collect();
        assert_eq!(unique.len(), dues.len(), "sources collide: {dues:?}");
    }

    #[test]
    fn every_source_has_a_sane_cadence() {
        for src in Source::ALL {
            // Nothing polled more than once a minute, nothing left over a day.
            assert!((60..=86_400).contains(&src.period()), "{src:?} period {}", src.period());
        }
    }
}
