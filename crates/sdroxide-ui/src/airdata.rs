//! The aircraft-navigation tables: airports, and radio navigation aids.
//!
//! Baked into the binary from `assets/airdata/`, built by
//! `assets/airdata/make_airdata.py` — the same arrangement as the Earth rasters
//! beside them. **Source: OurAirports, which is public domain** (the Unlicense),
//! so there is nothing to attribute and nothing to pay: see the house rule on
//! paid and keyed sources in `AGENTS.md`. Nothing here fetches anything, which
//! is the point — the maps work with the network unplugged.
//!
//! Two tables rather than one because they answer two questions. An airport is
//! a *place*, and the map wants the few a person could fly to. A navaid is a
//! *radio station*, and the map wants all of them — 6 700 are NDBs below
//! 1 800 kHz, which are receivable on an ordinary medium-wave receiver, so the
//! navaid table is a listening target list as much as a map layer.
//!
//! The format is a magic, a count, then fixed-size records — see the generator
//! for the layout, which is the only place it is written down twice.
//!
//! # Why these are `f64` positions from `f32` bytes
//!
//! The file stores `f32`, because a map position does not need more — about a
//! metre at the equator. They are widened once on load so everything downstream
//! works in the same precision as the rest of the geometry code, rather than
//! every caller remembering which tables are short.

// **Nothing draws from these yet.** The tables are the data half of the survey
// map's airport and navaid layers; the drawing is the next step, and until it
// lands every type here is dead code and warns as such on every target. An
// allow rather than eighteen warnings, because a tree that warns trains
// everyone to read past warnings — and these are exactly the warnings that
// would hide a real one. Take this off with the first consumer.
use core::sync::atomic::{AtomicU8, Ordering};

const NAVAIDS: &[u8] = include_bytes!("../assets/airdata/navaids.bin");
const AIRPORTS: &[u8] = include_bytes!("../assets/airdata/airports.bin");
const RUNWAYS: &[u8] = include_bytes!("../assets/airdata/runways.bin");
const FREQS: &[u8] = include_bytes!("../assets/airdata/freqs.bin");
const WIKI: &[u8] = include_bytes!("../assets/airdata/wiki.bin");

/// The kind of radio navigation aid — which is also its symbol on a chart, and
/// which bands it can be heard on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavaidKind {
    /// Non-directional beacon. **Medium wave**, 190–1750 kHz: the one an SWL
    /// can hear on a broadcast receiver.
    Ndb,
    /// VHF omnidirectional range, 108–118 MHz.
    Vor,
    /// A VOR with distance-measuring equipment on the same site.
    VorDme,
    /// A VOR paired with a military TACAN: one station, both services.
    Vortac,
    /// TACAN alone — military, UHF.
    Tacan,
    /// Distance-measuring equipment alone.
    Dme,
    NdbDme,
}

impl NavaidKind {
    fn from_index(i: u8) -> NavaidKind {
        match i {
            0 => NavaidKind::Ndb,
            1 => NavaidKind::Vor,
            2 => NavaidKind::VorDme,
            3 => NavaidKind::Vortac,
            4 => NavaidKind::Tacan,
            5 => NavaidKind::Dme,
            _ => NavaidKind::NdbDme,
        }
    }

    /// Whether this aid transmits below 1 800 kHz, where a short-wave listener
    /// can hear it on a medium-wave receiver.
    pub fn is_audible_on_mf(self) -> bool {
        matches!(self, NavaidKind::Ndb | NavaidKind::NdbDme)
    }

    /// The three or four letters a chart writes beside the symbol, and what the
    /// hover over a navaid says beside its ident.
    pub fn label(self) -> &'static str {
        match self {
            NavaidKind::Ndb => "NDB",
            NavaidKind::Vor => "VOR",
            NavaidKind::VorDme => "VOR/DME",
            NavaidKind::Vortac => "VORTAC",
            NavaidKind::Tacan => "TACAN",
            NavaidKind::Dme => "DME",
            NavaidKind::NdbDme => "NDB/DME",
        }
    }
}

/// One radio navigation aid.
#[derive(Debug, Clone, PartialEq)]
pub struct Navaid {
    pub ident: String,
    pub name: String,
    pub kind: NavaidKind,
    /// ISO country code, for a flag or a filter.
    pub country: String,
    /// Published frequency in kHz — the unit both kinds are published in, so
    /// 291 is an NDB and 116_300 is a VOR. 0 means the source published none.
    pub freq_khz: u32,
    pub lat: f64,
    pub lon: f64,
}

/// One airport a person could actually fly to.
#[derive(Debug, Clone, PartialEq)]
pub struct Airport {
    /// The ICAO or local ident, e.g. `EHAM`.
    pub ident: String,
    pub name: String,
    /// The three-letter code on a bag tag — empty for most small fields.
    pub iata: String,
    pub icao: String,
    /// A large airport, as opposed to a medium one. The two are kept apart
    /// because the map draws them differently and a zoomed-out view wants only
    /// the large ones.
    pub large: bool,
    pub lat: f64,
    pub lon: f64,
    pub elev_ft: i16,
}

/// What a runway is made of — the few classes a pilot or a listener cares
/// about, mapped from OurAirports' free-text surface codes by the generator.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Surface {
    Unknown,
    Asphalt,
    Concrete,
    Grass,
    Gravel,
    Dirt,
    Water,
}

impl Surface {
    fn from_index(i: u8) -> Surface {
        match i {
            1 => Surface::Asphalt,
            2 => Surface::Concrete,
            3 => Surface::Grass,
            4 => Surface::Gravel,
            5 => Surface::Dirt,
            6 => Surface::Water,
            _ => Surface::Unknown,
        }
    }
}

/// One open runway: both ends' designators, e.g. `09` / `27`.
#[derive(Debug, Clone, PartialEq)]
pub struct Runway {
    pub low: String,
    pub high: String,
    pub length_ft: u16,
    pub width_ft: u16,
    pub surface: Surface,
    pub lit: bool,
}

/// What a published frequency is for. The labels are the abbreviations a
/// pilot reads on a chart, so they are not translated (like a mode name).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FreqKind {
    Tower,
    Ground,
    Atis,
    Approach,
    Departure,
    Ctaf,
    Unicom,
    Delivery,
    Afis,
}

impl FreqKind {
    fn from_index(i: u8) -> Option<FreqKind> {
        Some(match i {
            0 => FreqKind::Tower,
            1 => FreqKind::Ground,
            2 => FreqKind::Atis,
            3 => FreqKind::Approach,
            4 => FreqKind::Departure,
            5 => FreqKind::Ctaf,
            6 => FreqKind::Unicom,
            7 => FreqKind::Delivery,
            8 => FreqKind::Afis,
            _ => return None,
        })
    }

    pub fn label(self) -> &'static str {
        match self {
            FreqKind::Tower => "TWR",
            FreqKind::Ground => "GND",
            FreqKind::Atis => "ATIS",
            FreqKind::Approach => "APP",
            FreqKind::Departure => "DEP",
            FreqKind::Ctaf => "CTAF",
            FreqKind::Unicom => "UNICOM",
            FreqKind::Delivery => "DEL",
            FreqKind::Afis => "AFIS",
        }
    }
}

/// One published radio frequency of an airport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frequency {
    pub kind: FreqKind,
    pub khz: u32,
}

/// Runways and frequencies of every airport, with the day the tables were built.
struct Extras {
    date: u32,
    runways: std::collections::HashMap<String, Vec<Runway>>,
    freqs: std::collections::HashMap<String, Vec<Frequency>>,
    /// Language and article title of the airport's Wikipedia page.
    wiki: std::collections::HashMap<String, (String, String)>,
}

const RUNWAY_LEN: usize = 22;
const FREQ_LEN: usize = 13;

/// The shared shape of the two dated tables: magic, `u32` date, `u32` count,
/// then fixed records. `None` when the file is not ours or is short.
fn dated<'a>(
    blob: &'a [u8],
    magic: &[u8],
    size: usize,
    name: &str,
) -> Option<(u32, &'a [u8], usize)> {
    let Some(rest) = blob.strip_prefix(magic) else {
        eprintln!("sdroxide: {name} is not the table it should be");
        return None;
    };
    if rest.len() < 8 {
        eprintln!("sdroxide: {name} is short of its own header");
        return None;
    }
    let (date, count) = (u32_at(rest, 0), u32_at(rest, 4) as usize);
    let body = &rest[8..];
    if body.len() < count.saturating_mul(size) {
        eprintln!("sdroxide: {name} is short of its own count");
        return None;
    }
    Some((date, body, count))
}

fn parse_runways(blob: &[u8]) -> (u32, std::collections::HashMap<String, Vec<Runway>>) {
    let mut map: std::collections::HashMap<String, Vec<Runway>> = Default::default();
    let Some((date, body, count)) = dated(blob, b"SDXRWY01", RUNWAY_LEN, "runways.bin") else {
        return (0, map);
    };
    for i in 0..count {
        let r = &body[i * RUNWAY_LEN..(i + 1) * RUNWAY_LEN];
        map.entry(field(&r[0..8])).or_default().push(Runway {
            low: field(&r[8..12]),
            high: field(&r[12..16]),
            length_ft: u16_at(r, 16),
            width_ft: u16_at(r, 18),
            surface: Surface::from_index(r[20]),
            lit: r[21] != 0,
        });
    }
    (date, map)
}

fn parse_freqs(blob: &[u8]) -> (u32, std::collections::HashMap<String, Vec<Frequency>>) {
    let mut map: std::collections::HashMap<String, Vec<Frequency>> = Default::default();
    let Some((date, body, count)) = dated(blob, b"SDXFRQ01", FREQ_LEN, "freqs.bin") else {
        return (0, map);
    };
    for i in 0..count {
        let r = &body[i * FREQ_LEN..(i + 1) * FREQ_LEN];
        if let Some(kind) = FreqKind::from_index(r[8]) {
            map.entry(field(&r[0..8])).or_default().push(Frequency { kind, khz: u32_at(r, 9) });
        }
    }
    (date, map)
}

/// `wiki.bin`: magic, `u32` date, `u32` count, then `{ident[8] lang[2] len title[len]}`
/// records. A record that runs past the end ends the table there; nothing is guessed.
fn parse_wiki(blob: &[u8]) -> std::collections::HashMap<String, (String, String)> {
    let mut map = std::collections::HashMap::new();
    let Some((_, mut body, count)) = dated(blob, b"SDXWIK01", 0, "wiki.bin") else {
        return map;
    };
    for _ in 0..count {
        if body.len() < 11 || body.len() < 11 + usize::from(body[10]) {
            eprintln!("sdroxide: wiki.bin ends in the middle of a record");
            break;
        }
        let len = usize::from(body[10]);
        let (ident, lang) = (field(&body[0..8]), field(&body[8..10]));
        let title = String::from_utf8_lossy(&body[11..11 + len]).into_owned();
        map.insert(ident, (lang, title));
        body = &body[11 + len..];
    }
    map
}

fn extras() -> &'static Extras {
    static ONCE: std::sync::OnceLock<Extras> = std::sync::OnceLock::new();
    ONCE.get_or_init(|| {
        let (d1, runways) = parse_runways(RUNWAYS);
        let (d2, freqs) = parse_freqs(FREQS);
        Extras { date: d1.min(d2), runways, freqs, wiki: parse_wiki(WIKI) }
    })
}

/// The open runways of an airport (by its `ident`), longest first. Empty when
/// the source publishes none — nothing is made up.
pub fn runways_of(ident: &str) -> &'static [Runway] {
    extras().runways.get(ident).map_or(&[], Vec::as_slice)
}

/// The published frequencies of an airport, most useful first (tower, ground,
/// ATIS, approach…). Empty when the source publishes none.
pub fn frequencies_of(ident: &str) -> &'static [Frequency] {
    extras().freqs.get(ident).map_or(&[], Vec::as_slice)
}

/// The Wikipedia page of an airport (by its `ident`) as `(language, title)`, the
/// title with underscores and no escapes, e.g. `("en", "Amsterdam_Airport_Schiphol")`.
/// `None` for an airport OurAirports links to no article.
pub fn wikipedia_of(ident: &str) -> Option<(&'static str, &'static str)> {
    extras().wiki.get(ident).map(|(l, t)| (l.as_str(), t.as_str()))
}

/// The day the runway and frequency tables were built, `yyyymmdd`; 0 when they
/// did not load. Frequencies change, so the airport card says how old they are.
pub fn extras_date() -> u32 {
    extras().date
}

/// Read a NUL-padded field, losing the padding.
fn field(bytes: &[u8]) -> String {
    let end = bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..end]).trim().to_string()
}

fn u16_at(b: &[u8], at: usize) -> u16 {
    u16::from_le_bytes([b[at], b[at + 1]])
}

fn u32_at(b: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]])
}

fn f32_at(b: &[u8], at: usize) -> f64 {
    f64::from(f32::from_le_bytes([b[at], b[at + 1], b[at + 2], b[at + 3]]))
}

/// The record size the generator writes, asserted against the count on load.
const NAVAID_LEN: usize = 36;
const AIRPORT_LEN: usize = 54;

fn parse_navaids(blob: &[u8]) -> Vec<Navaid> {
    let Some(rest) = blob.strip_prefix(b"SDXNAV01") else {
        eprintln!("sdroxide: navaids.bin is not a navaid table");
        return Vec::new();
    };
    let count = usize::from(u16_at(rest, 0));
    let body = &rest[2..];
    if body.len() < count * NAVAID_LEN {
        eprintln!("sdroxide: navaids.bin is short of its own count");
        return Vec::new();
    }
    (0..count)
        .map(|i| {
            let r = &body[i * NAVAID_LEN..(i + 1) * NAVAID_LEN];
            Navaid {
                ident: field(&r[0..5]),
                name: field(&r[5..21]),
                kind: NavaidKind::from_index(r[21]),
                country: field(&r[22..24]),
                freq_khz: u32_at(r, 24),
                lat: f32_at(r, 28),
                lon: f32_at(r, 32),
            }
        })
        .collect()
}

fn parse_airports(blob: &[u8]) -> Vec<Airport> {
    let Some(rest) = blob.strip_prefix(b"SDXAPT01") else {
        eprintln!("sdroxide: airports.bin is not an airport table");
        return Vec::new();
    };
    let count = usize::from(u16_at(rest, 0));
    let body = &rest[2..];
    if body.len() < count * AIRPORT_LEN {
        eprintln!("sdroxide: airports.bin is short of its own count");
        return Vec::new();
    }
    (0..count)
        .map(|i| {
            let r = &body[i * AIRPORT_LEN..(i + 1) * AIRPORT_LEN];
            Airport {
                ident: field(&r[0..8]),
                name: field(&r[8..36]),
                iata: field(&r[36..39]),
                icao: field(&r[39..43]),
                large: r[43] == 0,
                lat: f32_at(r, 44),
                lon: f32_at(r, 48),
                elev_ft: i16::from_le_bytes([r[52], r[53]]),
            }
        })
        .collect()
}

/// Every navigation aid, read once.
pub fn navaids() -> &'static [Navaid] {
    static ONCE: std::sync::OnceLock<Vec<Navaid>> = std::sync::OnceLock::new();
    ONCE.get_or_init(|| parse_navaids(NAVAIDS))
}

/// Every large and medium airport, read once.
pub fn airports() -> &'static [Airport] {
    static ONCE: std::sync::OnceLock<Vec<Airport>> = std::sync::OnceLock::new();
    ONCE.get_or_init(|| parse_airports(AIRPORTS))
}

/// How close a radio aid must be to an airport to be part of it, km. A VOR
/// on the field, or the NDB of its approach, is the airport's own; a VOR ten
/// kilometres off is an en-route aid and stays a symbol of its own.
pub const NAVAID_ON_FIELD_KM: f64 = 3.0;

/// Which airport each navaid belongs to, and the navaids of each airport.
struct NavLinks {
    /// Per navaid (same index as [`navaids`]): the airport it belongs to.
    owner: Vec<Option<u32>>,
    /// Per airport (same index as [`airports`]): its navaids, nearest first.
    of: Vec<Vec<u32>>,
}

/// Each navaid goes to the *nearest* airport within [`NAVAID_ON_FIELD_KM`], so
/// an aid is never listed on two cards. Worked out once, on first use, with a
/// one-degree grid so it is a few thousand lookups rather than 58 million.
fn nav_links() -> &'static NavLinks {
    static ONCE: std::sync::OnceLock<NavLinks> = std::sync::OnceLock::new();
    ONCE.get_or_init(|| {
        let (navs, apts) = (navaids(), airports());
        let cell = |lat: f64, lon: f64| (lat.floor() as i32, lon.floor() as i32);
        let mut grid: std::collections::HashMap<(i32, i32), Vec<usize>> = Default::default();
        for (i, a) in apts.iter().enumerate() {
            grid.entry(cell(a.lat, a.lon)).or_default().push(i);
        }
        let mut owner = vec![None; navs.len()];
        let mut of: Vec<Vec<(f64, u32)>> = vec![Vec::new(); apts.len()];
        for (ni, n) in navs.iter().enumerate() {
            let (cl, co) = cell(n.lat, n.lon);
            let mut best: Option<(f64, usize)> = None;
            for dl in -1..=1 {
                for dc in -1..=1 {
                    // Longitude wraps at the antimeridian.
                    let key = (cl + dl, (co + dc + 180).rem_euclid(360) - 180);
                    for &ai in grid.get(&key).map_or(&[][..], Vec::as_slice) {
                        let d = sdroxide_types::distance_km(
                            (n.lat, n.lon),
                            (apts[ai].lat, apts[ai].lon),
                        );
                        if d <= NAVAID_ON_FIELD_KM && best.is_none_or(|(b, _)| d < b) {
                            best = Some((d, ai));
                        }
                    }
                }
            }
            if let Some((d, ai)) = best {
                owner[ni] = Some(ai as u32);
                of[ai].push((d, ni as u32));
            }
        }
        let of = of
            .into_iter()
            .map(|mut v| {
                v.sort_by(|a, b| a.0.total_cmp(&b.0));
                v.into_iter().map(|(_, i)| i).collect()
            })
            .collect();
        NavLinks { owner, of }
    })
}

/// The airport (index into [`airports`]) a navaid (index into [`navaids`])
/// belongs to, when it stands on the field. Such an aid has no symbol of its
/// own on the map: it is listed on the airport's card.
pub fn navaid_airport(navaid: usize) -> Option<usize> {
    nav_links().owner.get(navaid).copied().flatten().map(|i| i as usize)
}

/// The radio aids on an airport's field (by index into [`airports`]), nearest
/// first, each with its distance from the airport reference point in km.
pub fn navaids_of(airport: usize) -> Vec<(&'static Navaid, f64)> {
    let (navs, apts) = (navaids(), airports());
    let Some(a) = apts.get(airport) else {
        return Vec::new();
    };
    nav_links().of.get(airport).map_or_else(Vec::new, |v| {
        v.iter()
            .map(|&i| {
                let n = &navs[i as usize];
                (n, sdroxide_types::distance_km((a.lat, a.lon), (n.lat, n.lon)))
            })
            .collect()
    })
}

/// Whether the tables were read at all — a data file that failed its magic
/// leaves empty tables, and empty tables look exactly like "no airports near
/// here" on a map. A screen that can tell the difference can say so.
pub fn loaded() -> bool {
    static STATE: AtomicU8 = AtomicU8::new(0);
    match STATE.load(Ordering::Relaxed) {
        1 => true,
        2 => false,
        _ => {
            let ok = !navaids().is_empty() && !airports().is_empty();
            STATE.store(if ok { 1 } else { 2 }, Ordering::Relaxed);
            ok
        }
    }
}

/// The nearest navaid of any kind to a position, within `max_km`.
///
/// `allow(dead_code)`: used by the aircraft data block, which names the nearest
/// aid — the step after the map layer, and tested here in the meantime so it is
/// not untried code waiting for a caller.
///
/// A linear scan, and deliberately: 11 000 records is a few hundred
/// microseconds, and this is asked once per aircraft per *fix* rather than once
/// per frame. It becomes a grid the day something asks it sixty times a second.
#[allow(dead_code)]
pub fn nearest_navaid(lat: f64, lon: f64, max_km: f64) -> Option<(&'static Navaid, f64)> {
    let mut best: Option<(&Navaid, f64)> = None;
    for n in navaids() {
        let d = sdroxide_types::distance_km((lat, lon), (n.lat, n.lon));
        if d <= max_km && best.is_none_or(|(_, b)| d < b) {
            best = Some((n, d));
        }
    }
    best
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The tables are the world's, and they read: the numbers come from the
    /// generator's own output, so a rebuild that drops a table fails here
    /// rather than showing an empty map.
    #[test]
    fn the_tables_are_the_worlds() {
        assert!(loaded(), "the assets did not load — check make_airdata.py");
        assert_eq!(navaids().len(), 11_008);
        assert_eq!(airports().len(), 5_280);
    }

    /// **A navaid either exists where and as the source says it does, or the
    /// parser is wrong.** Amsterdam's Schiphol VOR and a named NDB, both read
    /// back field by field against OurAirports' own rows.
    #[test]
    fn a_known_navaid_reads_back_field_for_field() {
        // Filtered by country because `SPL` is **two** navaids: Schiphol's here
        // and Split's in Croatia. The first draft of this test asserted a
        // frequency from memory, and the source row says 108 400 — the same
        // lesson as the FSK441 and RATT entries: read the row, do not recall it.
        let vor = navaids()
            .iter()
            .find(|n| n.ident == "SPL" && n.country == "NL")
            .expect("Schiphol's VOR is in the table");
        assert_eq!(vor.kind, NavaidKind::VorDme);
        assert_eq!(vor.freq_khz, 108_400);
        assert_eq!(vor.name, "Schiphol");
        assert!((vor.lat - 52.332).abs() < 0.01, "lat {}", vor.lat);
        assert!((vor.lon - 4.750).abs() < 0.01, "lon {}", vor.lon);
        assert!(
            navaids().iter().any(|n| n.ident == "SPL" && n.country == "HR"),
            "the ident is not unique, which is why the lookup filters on country"
        );

        let ndb = navaids()
            .iter()
            .find(|n| n.ident == "1A")
            .expect("Williams Harbour's NDB is in the table");
        assert_eq!(ndb.kind, NavaidKind::Ndb);
        assert_eq!(ndb.freq_khz, 373);
        assert_eq!(ndb.name, "Williams Harbour");
    }

    /// The listening half: most of the table is receivable on a medium-wave
    /// receiver, which is what makes it worth shipping for an SWL rather than
    /// only for the air picture.
    #[test]
    fn most_of_the_navaids_are_audible_on_medium_wave() {
        let mf: Vec<&Navaid> = navaids().iter().filter(|n| n.kind.is_audible_on_mf()).collect();
        assert!(mf.len() > 6_000, "only {} MF navaids — is the table truncated?", mf.len());
        assert!(
            mf.iter().all(|n| n.freq_khz < 1_800),
            "an MF navaid is published on a VHF frequency"
        );
    }

    /// Nothing in either table is off the Earth, and nothing is a NaN — the
    /// two ways a float format fails quietly and puts a symbol in the sea.
    #[test]
    fn every_position_is_on_the_earth() {
        for n in navaids() {
            assert!(
                (-90.0..=90.0).contains(&n.lat) && (-180.0..=180.0).contains(&n.lon),
                "{} is at {}, {}",
                n.ident,
                n.lat,
                n.lon
            );
        }
        for a in airports() {
            assert!(
                (-90.0..=90.0).contains(&a.lat) && (-180.0..=180.0).contains(&a.lon),
                "{} is at {}, {}",
                a.ident,
                a.lat,
                a.lon
            );
        }
    }

    /// Schiphol's runways read back as the source publishes them: six, the
    /// longest first, asphalt and lit; and its tower is on the list.
    /// The VOR of Épinal stands on the field of Épinal-Mirecourt: it is the
    /// airport's, not a second symbol next to it.
    #[test]
    fn a_vor_on_the_field_belongs_to_its_airport() {
        let ai = airports().iter().position(|a| a.ident == "LFSG").expect("LFSG");
        let epl =
            navaids().iter().position(|n| n.ident == "EPL" && n.country == "FR").expect("EPL");
        assert_eq!(navaid_airport(epl), Some(ai));
        assert!(navaids_of(ai).iter().any(|(n, _)| n.ident == "EPL"));
        assert!(navaids_of(ai).iter().all(|(_, km)| *km <= NAVAID_ON_FIELD_KM));
        // The Venezuelan El Pinal shares the ident and is nowhere near.
        let other = navaids().iter().position(|n| n.ident == "EPL" && n.country != "FR").unwrap();
        assert_ne!(navaid_airport(other), Some(ai));
    }

    /// An aid belongs to one airport only, and the two directions agree.
    #[test]
    fn every_navaid_is_listed_on_at_most_one_airport() {
        let mut seen = std::collections::HashSet::new();
        for ai in 0..airports().len() {
            for (n, _) in navaids_of(ai) {
                let ni = navaids().iter().position(|x| std::ptr::eq(x, n)).unwrap();
                assert!(seen.insert(ni), "{} listed twice", n.ident);
                assert_eq!(navaid_airport(ni), Some(ai));
            }
        }
        assert!(!seen.is_empty());
    }

    /// Every article title in the table is one the station will look up: a
    /// title it refuses is an airport that never gets its picture.
    #[test]
    fn every_wikipedia_title_in_the_table_is_an_article_the_station_accepts() {
        let refused: Vec<_> = extras()
            .wiki
            .iter()
            .filter(|(_, (l, t))| sdroxide_aircraft_online::Article::new(l, t).is_none())
            .map(|(i, (l, t))| format!("{i} {l}:{t}"))
            .collect();
        assert!(
            refused.is_empty(),
            "{} refused: {:?}",
            refused.len(),
            &refused[..refused.len().min(5)]
        );
    }

    #[test]
    fn schiphol_has_its_runways_and_a_tower_frequency() {
        let rw = runways_of("EHAM");
        assert!(rw.len() >= 5, "{rw:?}");
        assert!(rw.windows(2).all(|w| w[0].length_ft >= w[1].length_ft), "not longest first");
        let polderbaan = rw.iter().find(|r| r.low == "18R").expect("18R/36L is there");
        assert_eq!(polderbaan.high, "36L");
        assert_eq!(polderbaan.surface, Surface::Asphalt);
        assert!(polderbaan.lit);
        assert!((11_000..=12_500).contains(&polderbaan.length_ft), "{}", polderbaan.length_ft);
        let f = frequencies_of("EHAM");
        assert!(f.iter().any(|f| f.kind == FreqKind::Tower), "{f:?}");
        assert!(f.iter().any(|f| f.kind == FreqKind::Atis), "{f:?}");
        assert!(f.windows(2).all(|w| (w[0].kind as u8) <= (w[1].kind as u8)), "kinds out of order");
    }

    /// An airport the source says nothing about has no runway and no frequency,
    /// rather than an invented one; and the build date is a real date.
    #[test]
    fn an_unknown_airport_has_nothing_and_the_tables_are_dated() {
        assert!(runways_of("ZZZZ").is_empty());
        assert!(frequencies_of("ZZZZ").is_empty());
        let d = extras_date();
        assert!((2026_0101..=2099_1231).contains(&d), "{d}");
        assert!((1..=12).contains(&(d / 100 % 100)) && (1..=31).contains(&(d % 100)), "{d}");
    }

    #[test]
    fn schiphol_has_its_wikipedia_article_and_an_unknown_airport_has_none() {
        assert_eq!(wikipedia_of("EHAM"), Some(("en", "Amsterdam_Airport_Schiphol")));
        assert_eq!(wikipedia_of("ZZZZ"), None);
        // Percent-escapes are undone in the table, so the station escapes once.
        let (lang, title) = wikipedia_of("LEMG").expect("Malaga has an article");
        assert!(!title.contains('%') && lang.len() == 2, "{lang} {title}");
    }

    #[test]
    fn a_foreign_or_cut_wikipedia_table_is_refused_or_ends_where_it_is_cut() {
        assert!(parse_wiki(b"nope").is_empty());
        let mut b = b"SDXWIK01".to_vec();
        b.extend_from_slice(&[0; 4]);
        b.extend_from_slice(&2u32.to_le_bytes());
        b.extend_from_slice(b"EHAM\0\0\0\0en\x04Spam");
        b.extend_from_slice(b"LFPG\0\0\0\0fr\x09Paris");
        let got = parse_wiki(&b);
        assert_eq!(got.get("EHAM"), Some(&("en".to_string(), "Spam".to_string())));
        assert_eq!(got.len(), 1, "the cut record is dropped, not half-read");
    }

    /// Foreign or short runway/frequency files are refused, never half-read.
    #[test]
    fn foreign_runway_and_frequency_files_are_refused() {
        assert!(parse_runways(b"nope").1.is_empty());
        assert!(parse_freqs(b"nope").1.is_empty());
        // Right magic, a count that promises more than is there.
        let mut b = b"SDXRWY01".to_vec();
        b.extend_from_slice(&[0; 4]);
        b.extend_from_slice(&5u32.to_le_bytes());
        assert!(parse_runways(&b).1.is_empty());
    }

    /// A file that is not one of ours is refused rather than half-read.
    #[test]
    fn a_foreign_file_is_refused() {
        assert!(parse_navaids(b"not a table").is_empty());
        assert!(parse_airports(b"not a table").is_empty());
        // Right magic, wrong length: the count says more than is there.
        assert!(parse_navaids(b"SDXNAV01\xff\xffshort").is_empty());
    }

    /// The nearest-aid lookup finds a station that is really there and refuses
    /// to invent one that is not.
    #[test]
    fn the_nearest_navaid_is_where_it_should_be() {
        // Amsterdam, and the middle of the Atlantic.
        let (n, _) = nearest_navaid(52.3, 4.8, 60.0).expect("something is near Amsterdam");
        assert_eq!(n.country, "NL");
        assert!(nearest_navaid(0.0, -30.0, 60.0).is_none(), "found a navaid mid-Atlantic");
    }
}
