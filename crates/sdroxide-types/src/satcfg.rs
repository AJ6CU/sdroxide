//! Satellites the operator has added or corrected by hand: element sets the
//! automatic CelesTrak fetch does not carry, and frequency entries that
//! override — or extend — the built-in table.
//!
//! The tracker fetches CelesTrak's *amateur* group, which is the right default
//! and is also not everything anyone wants to track: the NOAA APT birds, a
//! cubesat launched last week and not yet in the group, a rocket body, the
//! Chinese space station. Rather than making the fetch configurable, this lets
//! the operator paste in the two lines they already have.
//!
//! The built-in frequency table lives in `sdroxide-solar` next to the tracker
//! that uses it. Only the *types* are here, so that this crate — which is the
//! one `sdroxide-config` persists through — can carry the operator's copy.

use serde::{Deserialize, Serialize};

/// A passband, or a single frequency when `lo == hi`.
///
/// Megahertz rather than hertz because that is how every published satellite
/// frequency is written, and rounding a 10.489 GHz downlink into an integer
/// number of hertz would invent precision the source never had.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Passband {
    pub lo_mhz: f64,
    pub hi_mhz: f64,
}

impl Passband {
    pub const fn at(mhz: f64) -> Passband {
        Passband { lo_mhz: mhz, hi_mhz: mhz }
    }

    pub const fn range(lo_mhz: f64, hi_mhz: f64) -> Passband {
        Passband { lo_mhz, hi_mhz }
    }

    /// True for a single frequency rather than a transponder passband.
    pub fn is_point(&self) -> bool {
        (self.hi_mhz - self.lo_mhz).abs() < 1e-9
    }

    /// Passband width in kilohertz — zero for a single frequency.
    pub fn width_khz(&self) -> f64 {
        (self.hi_mhz - self.lo_mhz).abs() * 1000.0
    }

    /// Middle of the passband, which is where a transponder is normally netted.
    pub fn centre_mhz(&self) -> f64 {
        0.5 * (self.lo_mhz + self.hi_mhz)
    }

    /// Parse what the operator typed into a frequency box: either `145.800` or
    /// a passband written `145.950-145.970` (en dash or hyphen).
    ///
    /// An empty or unparseable string is `None`, which is how a link ends up
    /// with only one direction.
    pub fn parse(s: &str) -> Option<Passband> {
        let s = s.trim();
        if s.is_empty() {
            return None;
        }
        let split = s
            .char_indices()
            // Skip index 0 so a leading sign is not read as the separator.
            .find(|(i, c)| *i > 0 && matches!(c, '-' | '–' | '—'));
        match split {
            Some((i, c)) => {
                let lo: f64 = s[..i].trim().parse().ok()?;
                let hi: f64 = s[i + c.len_utf8()..].trim().parse().ok()?;
                Some(Passband { lo_mhz: lo.min(hi), hi_mhz: lo.max(hi) })
            }
            None => s.parse().ok().map(Passband::at),
        }
    }
}

impl std::fmt::Display for Passband {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_point() {
            write!(f, "{}", fmt_mhz(self.lo_mhz))
        } else {
            write!(f, "{}–{}", fmt_mhz(self.lo_mhz), fmt_mhz(self.hi_mhz))
        }
    }
}

/// Format a frequency in MHz at the precision it was published to.
///
/// Most satellite frequencies land on a kilohertz; the NOAA APT downlinks are
/// on a quarter of one. Printing everything to four decimals would put a
/// meaningless trailing zero on almost every row.
pub fn fmt_mhz(mhz: f64) -> String {
    if (mhz * 1000.0 - (mhz * 1000.0).round()).abs() < 1e-6 {
        format!("{mhz:.3}")
    } else {
        format!("{mhz:.4}")
    }
}

/// One radio link on a satellite: a transponder, a repeater, a beacon or a
/// telemetry downlink.
///
/// Both `uplink` and `downlink` are optional because plenty of links have only
/// one: a CW beacon transmits and never listens, and there is no such thing as
/// a satellite that listens and never transmits.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SatLink {
    /// What the link is for — "V/U linear transponder", "FM voice", "APRS
    /// digipeater", "CW beacon".
    pub label: String,
    /// Emission the link carries — "SSB/CW", "FM", "BPSK 1k2", "DVB-S2".
    pub mode: String,
    pub uplink: Option<Passband>,
    pub downlink: Option<Passband>,
    /// Anything an operator has to know before keying up: a CTCSS tone, an
    /// inverting transponder, a duty schedule.
    pub note: String,
    /// A linear transponder that reverses the passband: the high end of the
    /// downlink answers the low end of the uplink, and sidebands flip. Most
    /// linear birds are built this way — it cancels the transponder's own
    /// Doppler asymmetry. Structured rather than left in `note`, because the
    /// satellite lock derives the transmit frequency from it.
    pub inverting: bool,
}

impl SatLink {
    /// A downlink-only link (beacon, telemetry, APT).
    pub fn down(label: &str, mode: &str, down: Passband) -> SatLink {
        SatLink {
            label: label.into(),
            mode: mode.into(),
            downlink: Some(down),
            ..Default::default()
        }
    }

    /// A two-way link.
    pub fn duplex(label: &str, mode: &str, up: Passband, down: Passband) -> SatLink {
        SatLink {
            label: label.into(),
            mode: mode.into(),
            uplink: Some(up),
            downlink: Some(down),
            ..Default::default()
        }
    }

    pub fn note(mut self, note: &str) -> SatLink {
        self.note = note.into();
        self
    }

    /// Mark the link's transponder as inverting.
    pub fn inverting(mut self) -> SatLink {
        self.inverting = true;
        self
    }

    /// True when neither direction has a frequency — an entry the operator
    /// started and never filled in, which is not worth a row in the table.
    pub fn is_empty(&self) -> bool {
        self.uplink.is_none() && self.downlink.is_none()
    }
}

/// Every published link for one satellite.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SatFreqs {
    pub norad_id: u64,
    /// The designator the frequencies were published under. Shown when it
    /// differs from the name the element set carries, so a table entry that has
    /// drifted onto the wrong catalogue number is visible rather than silent.
    pub name: String,
    pub links: Vec<SatLink>,
}

impl SatFreqs {
    pub fn new(norad_id: u64, name: &str, links: Vec<SatLink>) -> SatFreqs {
        SatFreqs { norad_id, name: name.into(), links }
    }

    /// Links worth showing — the ones with a frequency in at least one
    /// direction.
    pub fn usable_links(&self) -> impl Iterator<Item = &SatLink> {
        self.links.iter().filter(|l| !l.is_empty())
    }
}

/// A two-line element set the operator pasted in.
///
/// Stored as the three lines it arrived as rather than as parsed orbital
/// elements: that is the form every source hands out, it round-trips exactly,
/// and it means a bad paste can be looked at in the settings box rather than
/// having been silently normalised into something else.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct CustomTle {
    /// Line 0 of the three-line form — what the satellite is called.
    pub name: String,
    pub line1: String,
    pub line2: String,
    /// Unticked entries stay in the file but are not tracked, so a satellite
    /// can be parked without losing its elements.
    pub enabled: bool,
}

impl CustomTle {
    pub fn new(name: &str, line1: &str, line2: &str) -> CustomTle {
        CustomTle {
            name: name.trim().to_string(),
            line1: line1.trim_end().to_string(),
            line2: line2.trim_end().to_string(),
            enabled: true,
        }
    }

    /// Catalogue number from columns 3–7 of line 1.
    pub fn norad_id(&self) -> Option<u64> {
        let l1 = self.line1.as_bytes();
        if l1.len() < 7 {
            return None;
        }
        std::str::from_utf8(&l1[2..7]).ok()?.trim().parse().ok()
    }

    /// Why this entry cannot be tracked, or `None` when it looks well-formed.
    ///
    /// This is a shape check, not a validation of the orbit: SGP4 gets the
    /// final say when the elements are actually parsed. It exists so a paste
    /// that lost a line, or picked up a leading quote, says so in the settings
    /// dialog instead of quietly never appearing in the sky.
    pub fn problem(&self) -> Option<&'static str> {
        if self.name.trim().is_empty() {
            return Some("no name");
        }
        for (line, want) in [(&self.line1, '1'), (&self.line2, '2')] {
            let l = line.trim_end();
            if l.is_empty() {
                return Some("a line is missing");
            }
            if !l.starts_with(want) || l.as_bytes().get(1) != Some(&b' ') {
                return Some("lines must start \"1 \" and \"2 \"");
            }
            // The standard is 69 columns. Sources vary in what they do with
            // trailing blanks, so anything shorter is the real failure.
            if l.len() < 69 {
                return Some("a line is too short (69 columns)");
            }
        }
        if self.norad_id().is_none() {
            return Some("no catalogue number in columns 3–7");
        }
        // Both lines carry the same catalogue number; a mismatch means two
        // different satellites' lines got pasted together, which SGP4 would
        // happily propagate into a completely wrong orbit.
        let id2 = std::str::from_utf8(&self.line2.as_bytes()[2..7])
            .ok()
            .and_then(|s| s.trim().parse::<u64>().ok());
        if id2 != self.norad_id() {
            return Some("the two lines are from different satellites");
        }
        None
    }

    pub fn is_valid(&self) -> bool {
        self.problem().is_none()
    }
}

/// Which satellites in a listing get an orbit ring and a label.
///
/// Three states rather than two, because there genuinely are three useful
/// answers and a plain on/off switch hid the middle one: with it off, the
/// handful of satellites in the tracker's own curated list kept their rings
/// anyway, which made the switch look broken. Now that behaviour is a position
/// on the control rather than something happening behind it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OrbitRings {
    /// None of them. Plain dots, and only visible at all under `ALL SATS`.
    None,
    /// The ones in the tracker's built-in curated list — the handful the view
    /// is built around. The right answer for a whole group: ninety rings at
    /// once is unreadable, but no rings at all leaves ninety anonymous dots.
    ///
    /// That list is ten *amateur* satellites, so for a weather, GNSS or
    /// launch-window listing this position behaves exactly like
    /// [`OrbitRings::None`]. The settings dialog greys it out once a fetch has
    /// shown that a listing contains none of them, rather than leaving a chip
    /// that quietly does nothing.
    #[default]
    Curated,
    /// Every satellite in the listing. Only sensible for a short one.
    All,
}

impl OrbitRings {
    pub const ALL: [OrbitRings; 3] = [OrbitRings::None, OrbitRings::Curated, OrbitRings::All];

    pub fn label(self) -> &'static str {
        match self {
            OrbitRings::None => "none",
            OrbitRings::Curated => "curated",
            OrbitRings::All => "all",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            OrbitRings::None => "No orbit rings or labels from this listing at all",
            OrbitRings::Curated => {
                "Rings and labels only for the satellites in sdroxide's own curated list — ten \
                 amateur ones, so this does nothing for a listing without any of them"
            }
            OrbitRings::All => {
                "Rings and labels for every satellite in the listing. Unreadable for a whole \
                 group; right for a short one."
            }
        }
    }

    /// Whether a satellite gets a ring, given whether the built-in curated list
    /// has it.
    pub fn rings(self, curated: bool) -> bool {
        match self {
            OrbitRings::None => false,
            OrbitRings::Curated => curated,
            OrbitRings::All => true,
        }
    }
}

impl Serialize for OrbitRings {
    /// Hand-written rather than derived so it can be asymmetric: see
    /// [`OrbitRings::deserialize`] for why the two formats differ.
    ///
    /// The human-readable form is the tag name the derive used to emit, so
    /// existing `satellites.json` files keep round-tripping.
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        if s.is_human_readable() {
            s.serialize_str(match self {
                OrbitRings::None => "None",
                OrbitRings::Curated => "Curated",
                OrbitRings::All => "All",
            })
        } else {
            s.serialize_u8(*self as u8)
        }
    }
}

impl<'de> Deserialize<'de> for OrbitRings {
    /// In a config file, also accepts the boolean this used to be, so an
    /// existing `satellites.json` loads rather than failing and taking the
    /// operator's pasted element sets and frequency entries down with it.
    ///
    /// `true` ringed everything, so it becomes [`OrbitRings::All`]. `false`
    /// still left the curated few ringed, so it becomes [`OrbitRings::Curated`]
    /// — which is what those files actually meant, whatever they said.
    ///
    /// On the wire it is a plain index instead. The tolerant path is `untagged`,
    /// which means `deserialize_any`, and a non-self-describing format (postcard,
    /// which is what carries this to a remote client) cannot answer that at all.
    /// There is no legacy to accept there either: a version that did not have
    /// this field on the wire could not send it.
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        if !d.is_human_readable() {
            return Ok(match u8::deserialize(d)? {
                0 => OrbitRings::None,
                2 => OrbitRings::All,
                _ => OrbitRings::Curated,
            });
        }
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Wire {
            Legacy(bool),
            Tag(String),
        }
        Ok(match Wire::deserialize(d)? {
            Wire::Legacy(true) => OrbitRings::All,
            Wire::Legacy(false) => OrbitRings::Curated,
            Wire::Tag(t) if t == "None" => OrbitRings::None,
            Wire::Tag(t) if t == "All" => OrbitRings::All,
            // Anything unrecognised — a hand-edited file, or a value from a
            // future version — lands on the default rather than failing.
            Wire::Tag(_) => OrbitRings::Curated,
        })
    }
}

/// An element-set listing fetched from a URL and kept up to date.
///
/// Pasted elements go stale — SGP4 is worth little on a fortnight-old TLE — so
/// anything the operator intends to keep tracking wants a subscription instead.
/// The URL is whatever serves a two- or three-line listing; CelesTrak's group
/// endpoints are the usual answer, and [`CELESTRAK_GROUPS`] has the ones worth
/// offering as one-click additions.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TleSubscription {
    /// What the operator calls this listing, for the settings row and the
    /// status line. Not the satellite names — those come from the listing.
    pub name: String,
    pub url: String,
    pub enabled: bool,
    /// Which of these get an orbit ring and a label.
    pub orbits: OrbitRings,
    /// Keep only these catalogue numbers. Empty keeps everything the listing
    /// has, which is the point of subscribing to a group.
    pub only: Vec<u64>,
}

impl TleSubscription {
    pub fn new(name: &str, url: &str) -> TleSubscription {
        TleSubscription {
            name: name.trim().to_string(),
            url: url.trim().to_string(),
            enabled: true,
            orbits: OrbitRings::default(),
            only: Vec::new(),
        }
    }

    /// Why this subscription cannot be fetched, or `None` when it looks usable.
    pub fn problem(&self) -> Option<&'static str> {
        if self.name.trim().is_empty() {
            return Some("no name");
        }
        let url = self.url.trim();
        if url.is_empty() {
            return Some("no URL");
        }
        // Plain HTTP would put the element sets — and the fact that you are
        // fetching them — on the wire in clear. Every source that matters
        // serves TLS.
        if !url.starts_with("https://") {
            return Some("the URL must be https://");
        }
        None
    }

    pub fn is_valid(&self) -> bool {
        self.problem().is_none()
    }

    /// Whether a catalogue number passes this subscription's filter.
    pub fn wants(&self, norad_id: u64) -> bool {
        self.only.is_empty() || self.only.contains(&norad_id)
    }

    /// The filter as the settings box shows it: catalogue numbers, comma
    /// separated, empty for "everything".
    pub fn only_text(&self) -> String {
        self.only.iter().map(u64::to_string).collect::<Vec<_>>().join(", ")
    }

    /// Parse what was typed into that box. Anything that is not a number is
    /// dropped rather than rejecting the whole line — a trailing comma or a
    /// stray word should not clear the filter.
    pub fn set_only_text(&mut self, text: &str) {
        self.only = text
            .split([',', ' ', ';', '\n', '\t'])
            .filter_map(|s| s.trim().parse::<u64>().ok())
            .collect();
    }
}

/// One of the element-set listings offered as a one-click subscription.
pub struct CelestrakGroup {
    pub name: &'static str,
    pub url: &'static str,
    pub hint: &'static str,
    /// Subscribed on first run. The amateur satellites and the ISS are what the
    /// tracker used to fetch unconditionally, so a fresh install still shows
    /// them without the operator having to go and ask for them.
    pub default_on: bool,
    /// Which satellites in the listing start out ringed and labelled.
    pub orbits: OrbitRings,
}

/// CelesTrak group listings worth offering as one-click subscriptions.
///
/// The first two are what the tracker used to fetch on its own. They are
/// subscriptions like any other now, which is what makes them switchable,
/// filterable and refreshable rather than simply always on.
pub const CELESTRAK_GROUPS: &[CelestrakGroup] = &[
    CelestrakGroup {
        name: "Amateur radio",
        url: "https://celestrak.org/NORAD/elements/gp.php?GROUP=amateur&FORMAT=tle",
        hint: "Every amateur-radio satellite. The curated few keep their orbit rings and                labels; the rest are dots unless ALL SATS is on.",
        default_on: true,
        // Ninety rings at once is unreadable, and no rings at all leaves ninety
        // anonymous dots — so the curated few, which is the middle position.
        orbits: OrbitRings::Curated,
    },
    CelestrakGroup {
        name: "ISS",
        url: "https://celestrak.org/NORAD/elements/gp.php?CATNR=25544&FORMAT=tle",
        hint: "The ISS on its own, from its own element set — fresher than the copy in the                amateur group, and it keeps working if you unsubscribe from that.",
        default_on: true,
        orbits: OrbitRings::All,
    },
    CelestrakGroup {
        name: "Weather",
        url: "https://celestrak.org/NORAD/elements/gp.php?GROUP=weather&FORMAT=tle",
        hint: "The NOAA APT and Meteor LRPT birds on 137 MHz",
        default_on: false,
        orbits: OrbitRings::Curated,
    },
    CelestrakGroup {
        name: "CubeSats",
        url: "https://celestrak.org/NORAD/elements/gp.php?GROUP=cubesat&FORMAT=tle",
        hint: "Everything cubesat-sized, including amateur payloads too new for the amateur group",
        default_on: false,
        orbits: OrbitRings::Curated,
    },
    CelestrakGroup {
        name: "Space stations",
        url: "https://celestrak.org/NORAD/elements/gp.php?GROUP=stations&FORMAT=tle",
        hint: "The ISS, Tiangong and the vehicles docked with them",
        default_on: false,
        orbits: OrbitRings::Curated,
    },
    CelestrakGroup {
        name: "Last 30 days' launches",
        url: "https://celestrak.org/NORAD/elements/gp.php?GROUP=last-30-days&FORMAT=tle",
        hint: "Anything launched in the last month — where a brand-new amateur satellite shows                up first",
        default_on: false,
        orbits: OrbitRings::Curated,
    },
    CelestrakGroup {
        name: "Geostationary",
        url: "https://celestrak.org/NORAD/elements/gp.php?GROUP=geo&FORMAT=tle",
        hint: "The geostationary belt, QO-100 among it",
        default_on: false,
        orbits: OrbitRings::Curated,
    },
    CelestrakGroup {
        name: "GNSS",
        url: "https://celestrak.org/NORAD/elements/gp.php?GROUP=gnss&FORMAT=tle",
        hint: "GPS, Galileo, GLONASS and BeiDou",
        default_on: false,
        orbits: OrbitRings::Curated,
    },
];

/// What one subscription's last fetch did, for the settings dialog.
///
/// Produced by the tracker's fetcher (`sdroxide_solar::tlesub`), but declared
/// here because it also has to cross the wire: a remote client's settings
/// dialog edits the *engine host's* subscriptions, so the engine host is the
/// only place that knows when each listing was last fetched and what it
/// yielded.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct TleSubStatus {
    pub url: String,
    /// When the listing was last fetched successfully, or 0 for never.
    pub fetched_unix: i64,
    /// How many satellites it yielded after the filter.
    pub count: usize,
    /// How many of those are in the tracker's built-in curated list.
    ///
    /// Zero for every listing that is not the amateur group — that list is ten
    /// amateur satellites — which is what lets the settings dialog grey out an
    /// orbit-ring position that would do nothing here.
    pub curated: usize,
    /// Why the last attempt failed, if it did. A failure does not clear
    /// `count`: the cached listing is still what is being tracked.
    pub error: Option<String>,
}

/// The operator's satellite additions, persisted as `satellites.json`.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SatConfig {
    /// Element sets to track on top of the fetched amateur group.
    pub tles: Vec<CustomTle>,
    /// Element-set listings fetched and kept up to date.
    pub subs: Vec<TleSubscription>,
    /// Frequency entries that override the built-in table for their catalogue
    /// number, or add one for a satellite the table does not know.
    pub freqs: Vec<SatFreqs>,
    /// Whether the default subscriptions have been added.
    ///
    /// A one-shot marker rather than a `Default` for `subs`, because serde's
    /// per-field defaults would re-add them to any config written before they
    /// existed *and* to one where the operator has deliberately removed them.
    /// This way they arrive exactly once, and unsubscribing sticks.
    pub seeded: bool,
}

impl SatConfig {
    /// The enabled, well-formed entries as one three-line element listing —
    /// the form every TLE parser takes.
    ///
    /// Broken entries are dropped rather than emitted: a malformed line in the
    /// middle of a listing can desynchronise a three-line parser and cost the
    /// *following* satellites too.
    pub fn tle_text(&self) -> String {
        let mut out = String::new();
        for t in self.tles.iter().filter(|t| t.enabled && t.is_valid()) {
            out.push_str(t.name.trim());
            out.push('\n');
            out.push_str(t.line1.trim_end());
            out.push('\n');
            out.push_str(t.line2.trim_end());
            out.push('\n');
        }
        out
    }

    /// The operator's frequency entry for a satellite, if they have one.
    pub fn freqs_for(&self, norad_id: u64) -> Option<&SatFreqs> {
        self.freqs.iter().find(|f| f.norad_id == norad_id)
    }

    /// The operator's frequency entry for a satellite, creating an empty one if
    /// there is none yet.
    pub fn freqs_for_mut(&mut self, norad_id: u64, name: &str) -> &mut SatFreqs {
        if let Some(i) = self.freqs.iter().position(|f| f.norad_id == norad_id) {
            return &mut self.freqs[i];
        }
        self.freqs.push(SatFreqs::new(norad_id, name, Vec::new()));
        self.freqs.last_mut().expect("just pushed")
    }

    /// Drop frequency entries that have no links left, so deleting the last row
    /// of an override falls back to the built-in table rather than shadowing it
    /// with an empty list.
    pub fn prune(&mut self) {
        self.freqs.retain(|f| !f.links.is_empty());
    }

    /// Catalogue numbers of the enabled pasted element sets — the ones that
    /// should win over a fetched entry for the same satellite.
    pub fn tracked_ids(&self) -> Vec<u64> {
        self.tles
            .iter()
            .filter(|t| t.enabled && t.is_valid())
            .filter_map(|t| t.norad_id())
            .collect()
    }

    /// Add the subscriptions a fresh install starts with, once.
    ///
    /// Returns whether anything changed, so the caller knows to write the file
    /// back. Existing URLs are left alone: an operator who has already
    /// subscribed to the amateur group — or switched its orbit rings on — keeps
    /// their version of it.
    pub fn seed_defaults(&mut self) -> bool {
        if self.seeded {
            return false;
        }
        self.seeded = true;
        let mut added = false;
        for g in CELESTRAK_GROUPS.iter().filter(|g| g.default_on) {
            if self.has_sub(g.url) {
                continue;
            }
            let mut sub = TleSubscription::new(g.name, g.url);
            sub.orbits = g.orbits;
            self.subs.push(sub);
            added = true;
        }
        added
    }

    /// The subscriptions worth fetching.
    pub fn live_subs(&self) -> impl Iterator<Item = &TleSubscription> {
        self.subs.iter().filter(|s| s.enabled && s.is_valid())
    }

    /// True when a URL is already subscribed to, so the settings dialog can
    /// refuse to add it twice.
    pub fn has_sub(&self, url: &str) -> bool {
        self.subs.iter().any(|s| s.url.trim() == url.trim())
    }
}

/// Split a pasted block into two- or three-line element sets.
///
/// Accepts both forms: the three-line one with a name above each pair, and the
/// bare two-line one, which gets its catalogue number as a name because a
/// satellite with no name at all is not something the operator can pick out of
/// a list.
pub fn parse_tle_block(text: &str) -> Vec<CustomTle> {
    let lines: Vec<&str> =
        text.lines().map(str::trim_end).filter(|l| !l.trim().is_empty()).collect();
    let mut out = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let l = lines[i];
        // A name line is anything that is not an element line.
        let is_l1 = |s: &str| s.starts_with("1 ") && s.len() >= 69;
        let is_l2 = |s: &str| s.starts_with("2 ") && s.len() >= 69;
        if is_l1(l) && i + 1 < lines.len() && is_l2(lines[i + 1]) {
            let t = CustomTle::new("", l, lines[i + 1]);
            let name =
                t.norad_id().map_or_else(|| "UNNAMED".to_string(), |id| format!("NORAD {id}"));
            out.push(CustomTle::new(&name, l, lines[i + 1]));
            i += 2;
        } else if i + 2 < lines.len() && is_l1(lines[i + 1]) && is_l2(lines[i + 2]) {
            // A three-line set: strip the leading "0 " some sources put on the
            // name line of the "3LE" form.
            let name = l.strip_prefix("0 ").unwrap_or(l);
            out.push(CustomTle::new(name, lines[i + 1], lines[i + 2]));
            i += 3;
        } else {
            i += 1;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A real element set, so the column arithmetic is tested against the
    /// format as it actually arrives rather than against a hand-made string.
    const ISS: &str = "\
ISS (ZARYA)
1 25544U 98067A   26206.54791667  .00016717  00000-0  10270-3 0  9005
2 25544  51.6392 339.2461 0004669  87.5384 272.6072 15.50377579 34567";

    #[test]
    fn a_pasted_three_line_set_keeps_its_name_and_catalogue_number() {
        let v = parse_tle_block(ISS);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].name, "ISS (ZARYA)");
        assert_eq!(v[0].norad_id(), Some(25544));
        assert!(v[0].enabled);
        assert_eq!(v[0].problem(), None, "a real TLE must validate");
    }

    #[test]
    fn a_bare_two_line_set_is_named_after_its_catalogue_number() {
        let two: String = ISS.lines().skip(1).collect::<Vec<_>>().join("\n");
        let v = parse_tle_block(&two);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].name, "NORAD 25544");
        assert_eq!(v[0].problem(), None);
    }

    /// The "3LE" form puts a `0 ` on the name line; keeping it would show the
    /// satellite as "0 ISS (ZARYA)" everywhere.
    #[test]
    fn the_3le_name_prefix_is_stripped() {
        let v = parse_tle_block(&format!("0 {ISS}"));
        assert_eq!(v[0].name, "ISS (ZARYA)");
    }

    #[test]
    fn several_sets_in_one_paste_all_come_through() {
        let mut block = String::new();
        for _ in 0..3 {
            block.push_str(ISS);
            block.push('\n');
        }
        assert_eq!(parse_tle_block(&block).len(), 3);
        // Blank lines and stray prose between sets are skipped, not fatal.
        assert_eq!(parse_tle_block(&format!("{ISS}\n\nfetched today\n\n{ISS}")).len(), 2);
        assert!(parse_tle_block("").is_empty());
        assert!(parse_tle_block("nothing like a tle").is_empty());
    }

    /// Every way a paste goes wrong has to be named, because the operator can
    /// see the box it went wrong in.
    #[test]
    fn a_malformed_entry_says_what_is_wrong_with_it() {
        let good = &parse_tle_block(ISS)[0];
        let mut t = good.clone();
        t.name.clear();
        assert_eq!(t.problem(), Some("no name"));

        let mut t = good.clone();
        t.line2.clear();
        assert_eq!(t.problem(), Some("a line is missing"));

        let mut t = good.clone();
        t.line1 = t.line1.replace("1 25544", "3 25544");
        assert_eq!(t.problem(), Some("lines must start \"1 \" and \"2 \""));

        let mut t = good.clone();
        t.line1.truncate(40);
        assert_eq!(t.problem(), Some("a line is too short (69 columns)"));

        // Two satellites' lines pasted together — the one that would otherwise
        // propagate into a plausible-looking, completely wrong orbit.
        let mut t = good.clone();
        t.line2 = t.line2.replace("2 25544", "2 25545");
        assert_eq!(t.problem(), Some("the two lines are from different satellites"));
    }

    #[test]
    fn only_enabled_and_valid_entries_reach_the_tracker() {
        let mut cfg = SatConfig::default();
        cfg.tles = parse_tle_block(ISS);
        cfg.tles.push(CustomTle::new("BROKEN", "1 nope", "2 nope"));
        let mut parked = cfg.tles[0].clone();
        parked.enabled = false;
        cfg.tles.push(parked);

        let text = cfg.tle_text();
        assert_eq!(text.lines().count(), 3, "only the one good enabled set");
        assert!(text.starts_with("ISS (ZARYA)\n1 25544"));
        assert_eq!(cfg.tracked_ids(), vec![25544]);
    }

    #[test]
    fn a_frequency_override_is_created_and_pruned_in_place() {
        let mut cfg = SatConfig::default();
        assert!(cfg.freqs_for(25544).is_none());
        cfg.freqs_for_mut(25544, "ISS").links.push(SatLink::down(
            "Voice",
            "FM",
            Passband::at(145.8),
        ));
        assert_eq!(cfg.freqs_for(25544).unwrap().links.len(), 1);
        // Asking again returns the same entry rather than a second one.
        cfg.freqs_for_mut(25544, "ISS").links.push(SatLink::down(
            "SSTV",
            "FM",
            Passband::at(145.8),
        ));
        assert_eq!(cfg.freqs.len(), 1);
        assert_eq!(cfg.freqs[0].links.len(), 2);

        // Emptied of links, the override goes away so the built-in table shows
        // through again instead of being shadowed by nothing.
        cfg.freqs[0].links.clear();
        cfg.prune();
        assert!(cfg.freqs.is_empty());
    }

    #[test]
    fn a_typed_frequency_parses_as_a_point_or_a_passband() {
        assert_eq!(Passband::parse("145.800"), Some(Passband::at(145.8)));
        assert_eq!(Passband::parse(" 436.795 "), Some(Passband::at(436.795)));
        assert_eq!(Passband::parse("145.950-145.970"), Some(Passband::range(145.95, 145.97)));
        // En dash, as the display side writes it, so a copy back in round-trips.
        assert_eq!(Passband::parse("145.950–145.970"), Some(Passband::range(145.95, 145.97)));
        // Reversed ends are ordered rather than rejected.
        assert_eq!(Passband::parse("145.970-145.950"), Some(Passband::range(145.95, 145.97)));
        assert_eq!(Passband::parse(""), None);
        assert_eq!(Passband::parse("  "), None);
        assert_eq!(Passband::parse("not a number"), None);
        assert_eq!(Passband::parse("145.9-"), None);
        // What Display wrote must parse back to what it was written from.
        for b in [Passband::at(137.9125), Passband::range(2400.05, 2400.3)] {
            assert_eq!(Passband::parse(&b.to_string()), Some(b));
        }
    }

    #[test]
    fn frequencies_print_at_the_precision_they_were_published_to() {
        assert_eq!(fmt_mhz(145.8), "145.800");
        assert_eq!(fmt_mhz(137.9125), "137.9125");
        assert_eq!(fmt_mhz(10489.55), "10489.550");
        assert_eq!(Passband::range(145.95, 145.97).to_string(), "145.950–145.970");
        assert!((Passband::range(145.95, 145.97).centre_mhz() - 145.96).abs() < 1e-9);
        assert!((Passband::range(145.95, 145.97).width_khz() - 20.0).abs() < 1e-6);
        assert!(Passband::at(1.0).is_point());
        assert!(!Passband::range(1.0, 2.0).is_point());
        assert!(SatLink::default().is_empty());
        assert!(!SatLink::down("b", "CW", Passband::at(29.5)).is_empty());
    }

    /// A fresh install has to come up tracking the amateur satellites and the
    /// ISS, because that is what it did before they became subscriptions.
    /// The setting used to be a plain `bool`, and a config written then has to
    /// keep loading — failing the field would take the operator's pasted
    /// element sets and frequency entries down with the whole file.
    #[test]
    fn a_config_written_when_orbits_was_a_boolean_still_loads() {
        let old = r#"{"subs":[
            {"name":"a","url":"https://x/1","enabled":true,"orbits":true,"only":[]},
            {"name":"b","url":"https://x/2","enabled":true,"orbits":false,"only":[]}
        ]}"#;
        let cfg: SatConfig = serde_json::from_str(old).expect("an old config must still load");
        // `true` ringed everything...
        assert_eq!(cfg.subs[0].orbits, OrbitRings::All);
        // ...and `false` still left the curated few ringed, which is what that
        // file meant whatever it said.
        assert_eq!(cfg.subs[1].orbits, OrbitRings::Curated);

        // A value from a newer version, or a hand-edited one, lands on the
        // default rather than failing the file.
        let odd: SatConfig = serde_json::from_str(
            r#"{"subs":[{"name":"a","url":"https://x","orbits":"Surprise"}]}"#,
        )
        .expect("an unknown value must not fail the file");
        assert_eq!(odd.subs[0].orbits, OrbitRings::Curated);
    }

    /// What each position of the orbit-ring control does, given whether the
    /// built-in curated list has the satellite.
    #[test]
    fn the_orbit_positions_do_what_they_say() {
        assert!(!OrbitRings::None.rings(true), "off must be off even for a curated satellite");
        assert!(!OrbitRings::None.rings(false));
        assert!(OrbitRings::Curated.rings(true));
        assert!(!OrbitRings::Curated.rings(false));
        assert!(OrbitRings::All.rings(true));
        assert!(OrbitRings::All.rings(false));
        // Distinct labels, or the three chips would not be tellable apart.
        let labels: std::collections::HashSet<_> =
            OrbitRings::ALL.iter().map(|o| o.label()).collect();
        assert_eq!(labels.len(), 3);
        assert!(OrbitRings::ALL.iter().all(|o| !o.hint().is_empty()));
    }

    #[test]
    fn a_fresh_config_is_seeded_with_the_amateur_satellites_and_the_iss() {
        let mut cfg = SatConfig::default();
        assert!(cfg.subs.is_empty() && !cfg.seeded);
        assert!(cfg.seed_defaults());
        let names: Vec<&str> = cfg.subs.iter().map(|s| s.name.as_str()).collect();
        assert_eq!(names, vec!["Amateur radio", "ISS"]);
        assert!(cfg.subs.iter().all(|s| s.enabled), "the defaults must be on");
        assert!(cfg.live_subs().count() == 2);
        // The ISS is one satellite, so all of it is ringed; the amateur group
        // shows only the curated few.
        assert_eq!(cfg.subs[0].orbits, OrbitRings::Curated);
        assert_eq!(cfg.subs[1].orbits, OrbitRings::All);
    }

    /// Seeding happens once. An operator who unsubscribes must not find the
    /// subscription back on the next start — that would make the switch useless.
    #[test]
    fn seeding_happens_once_and_unsubscribing_sticks() {
        let mut cfg = SatConfig::default();
        cfg.seed_defaults();
        cfg.subs.clear();
        assert!(!cfg.seed_defaults(), "the defaults came back");
        assert!(cfg.subs.is_empty());

        // A config written before the defaults existed is seeded on the way in,
        // rather than coming up with an empty sky.
        let mut old: SatConfig = serde_json::from_str(r#"{"freqs":[]}"#).unwrap();
        assert!(!old.seeded);
        assert!(old.seed_defaults());
        assert_eq!(old.subs.len(), 2);

        // ...and one that already has the amateur group keeps its own version
        // of it rather than gaining a second.
        let mut mine = SatConfig::default();
        let mut sub = TleSubscription::new("My amateur list", CELESTRAK_GROUPS[0].url);
        sub.orbits = OrbitRings::All;
        mine.subs.push(sub);
        mine.seed_defaults();
        assert_eq!(mine.subs.len(), 2, "{:?}", mine.subs);
        assert_eq!(mine.subs[0].name, "My amateur list");
        assert_eq!(
            mine.subs[0].orbits,
            OrbitRings::All,
            "the operator's own settings were overwritten"
        );
    }

    #[test]
    fn the_whole_config_round_trips_through_json() {
        let mut cfg = SatConfig {
            tles: parse_tle_block(ISS),
            subs: vec![TleSubscription::new("Weather", CELESTRAK_GROUPS[2].url)],
            freqs: Vec::new(),
            seeded: true,
        };
        cfg.subs[0].only = vec![25338, 33591];
        cfg.freqs_for_mut(25544, "ISS").links.push(
            SatLink::duplex("FM voice", "FM", Passband::at(145.2), Passband::at(145.8))
                .note("144.490 in regions 2 and 3"),
        );
        let back: SatConfig = serde_json::from_str(&serde_json::to_string(&cfg).unwrap()).unwrap();
        assert_eq!(back, cfg);
    }
}

/// One labelled span to draw on the panadapter's frequency scale.
///
/// Her*z*, unlike [`Passband`]'s megahertz: this is what gets drawn against a
/// panadapter axis, and the axis is hertz. The conversion happens once, here,
/// rather than at the drawing site where a stray factor of a million would be a
/// label in the wrong place rather than a type error.
#[derive(Clone, Debug, PartialEq)]
pub struct SatBand {
    /// The link's own label — "V/U linear transponder", "CW beacon".
    pub label: String,
    pub lo_hz: f64,
    pub hi_hz: f64,
    /// A single frequency rather than a passband, so the drawing can decide
    /// between a line and a box without re-deriving it from the edges.
    pub is_point: bool,
}

/// The **downlink** bands of a satellite, shifted by the correction in force,
/// ready to draw against the panadapter.
///
/// **The shift is the whole point, and the direction is easy to get backwards.**
/// The panadapter's axis is the receiver's dial, and a locked satellite has the
/// dial moved *off* the published frequency by the correction — so a band drawn
/// at its published place sits the correction away from the signal it is meant to
/// name, which is exactly the fault the feature exists to prevent. Drawn at
/// `published + doppler_rx_hz`, the band lands where the signal is.
///
/// Uplinks are deliberately not drawn: they are not on this axis. A transponder's
/// uplink is on another band entirely, and showing it here would put a label on
/// a frequency the receiver is not listening to.
pub fn sat_bands(freqs: &SatFreqs, doppler_rx_hz: f64) -> Vec<SatBand> {
    let mut out = Vec::new();
    for link in freqs.usable_links() {
        let Some(down) = link.downlink else { continue };
        let shift = doppler_rx_hz / 1e6; // hz -> the MHz the table is written in
        let lo = (down.lo_mhz + shift) * 1e6;
        let hi = (down.hi_mhz + shift) * 1e6;
        let (lo, hi) = if lo <= hi { (lo, hi) } else { (hi, lo) };
        // A table entry with no frequency in it, or a nonsensical one, is not a
        // band. `usable_links` asks whether a *direction* exists, which is a
        // different question from whether there is a frequency to draw.
        if hi <= 0.0 {
            continue;
        }
        out.push(SatBand {
            label: if link.label.is_empty() { link.mode.clone() } else { link.label.clone() },
            lo_hz: lo,
            hi_hz: hi,
            is_point: down.is_point(),
        });
    }
    out
}

#[cfg(test)]
mod sat_bands_tests {
    use super::{Passband, SatFreqs, SatLink, sat_bands};

    fn bird(links: Vec<SatLink>) -> SatFreqs {
        SatFreqs::new(43_013, "Test Bird", links)
    }

    /// **The band lands on the signal, not on the published frequency.** A locked
    /// satellite has the dial moved off the published place by the correction, so
    /// drawing at the published number puts the label the correction away from
    /// what it names — the fault this feature exists to prevent.
    #[test]
    fn a_band_moves_with_the_correction() {
        let f = bird(vec![SatLink::down("FM voice", "FM", Passband::range(145.800, 145.810))]);
        let still = sat_bands(&f, 0.0);
        assert_eq!(still.len(), 1);
        assert!((still[0].lo_hz - 145_800_000.0).abs() < 1.0);
        assert!((still[0].hi_hz - 145_810_000.0).abs() < 1.0);

        // Two kilohertz of correction moves the band by two kilohertz, both
        // edges together — a passband is translated, it does not stretch.
        let moved = sat_bands(&f, 2_000.0);
        assert!((moved[0].lo_hz - 145_802_000.0).abs() < 1.0, "{:?}", moved[0]);
        assert!((moved[0].hi_hz - 145_812_000.0).abs() < 1.0, "{:?}", moved[0]);
        assert!(
            (moved[0].hi_hz - moved[0].lo_hz) - (still[0].hi_hz - still[0].lo_hz) < 1.0,
            "the width is the transponder's, not the correction's"
        );

        // And the other way, because a sign error here is the whole bug.
        let receding = sat_bands(&f, -2_000.0);
        assert!((receding[0].lo_hz - 145_798_000.0).abs() < 1.0, "{:?}", receding[0]);
    }

    /// A beacon is a point, a transponder is a span, and the drawing needs to
    /// know which without inferring it from two numbers that happen to be equal.
    #[test]
    fn a_beacon_is_a_point_and_a_transponder_is_a_span() {
        let f = bird(vec![
            SatLink::down("CW beacon", "CW", Passband::at(145.800)),
            SatLink::down("Linear transponder", "SSB/CW", Passband::range(145.900, 145.950)),
        ]);
        let bands = sat_bands(&f, 0.0);
        assert_eq!(bands.len(), 2);
        assert!(bands[0].is_point, "a beacon is one frequency");
        assert!(!bands[1].is_point, "a transponder is a passband");
        assert_eq!(bands[0].label, "CW beacon");
    }

    /// **Uplinks are not on this axis.** A transponder's uplink is on another band
    /// entirely; drawing it here would label a frequency the receiver is not
    /// listening to, which is worse than not drawing it at all.
    #[test]
    fn uplinks_are_not_drawn() {
        let f = bird(vec![SatLink {
            label: "V/U".into(),
            mode: "SSB/CW".into(),
            uplink: Some(Passband::range(145.200, 145.250)),
            downlink: None,
            note: String::new(),
            inverting: false,
        }]);
        assert!(sat_bands(&f, 0.0).is_empty(), "an uplink-only link has nothing to draw here");
    }

    /// A link with neither direction is not a link, and one with a reversed
    /// passband in the table must still come out ordered so the drawing never
    /// gets a negative width.
    #[test]
    fn an_empty_link_is_skipped_and_the_edges_are_ordered() {
        let f = bird(vec![
            SatLink::down("empty", "", Passband::at(0.0)),
            SatLink::down("backwards", "FM", Passband::range(145.900, 145.800)),
        ]);
        let bands = sat_bands(&f, 0.0);
        assert_eq!(bands.len(), 1, "a 0 MHz link is not usable");
        assert!(bands[0].lo_hz < bands[0].hi_hz, "the edges are ordered for drawing");
    }
}
