//! Contest logging: the contest list, the exchange each one wants, a live score
//! estimate, and the Cabrillo export.
//!
//! The logger is **mode-agnostic** — the operator types the exchange by hand
//! for CW, SSB or anything else, and the FT8 side auto-fills the same entry
//! when it is driving. Nothing here is on the wire: a contest session is the
//! operator's, held in the UI, and it writes into the ordinary logbook as
//! [`crate::QsoRecord`]s.

use serde::{Deserialize, Serialize};

use crate::digi::QsoRecord;

/// The contests the logger ships with.
///
/// [`Generic`](Self::Generic) is the fallback for anything not listed;
/// [`CbActivity`](Self::CbActivity) is the 11 m side's flexible format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ContestId {
    /// No contest — the logger is idle.
    #[default]
    None,
    /// CQ World Wide: RST + CQ zone.
    CqWw,
    /// CQ WPX: RST + serial.
    CqWpx,
    /// ARRL DX: RST + state (or power from outside the US and Canada).
    ArrlDx,
    /// European VHF: RST + serial + a 6-character locator — the FT8 layout too.
    EuVhf,
    /// 11 m / CB activity: a report and a free-text exchange (channel, name,
    /// area — whatever the activity settles on).
    CbActivity,
    /// RST + serial, for anything else.
    Generic,
}

/// One part of a contest exchange.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exchange {
    /// The signal report — `599`/`59`, or an FT8 dB.
    Rst,
    /// The serial number: auto-incremented when sent, captured when received.
    Serial,
    /// CQ zone (1–40).
    CqZone,
    /// ITU zone (1–90).
    ItuZone,
    /// State or province.
    State,
    /// Maidenhead locator.
    Grid,
    /// Transmitter power.
    Power,
    /// Free text — the CB activity's channel, name or area.
    Text,
}

impl Exchange {
    /// The column heading the entry form and the log show.
    pub fn label(self) -> &'static str {
        match self {
            Exchange::Rst => "RST",
            Exchange::Serial => "SERIAL",
            Exchange::CqZone => "CQ ZONE",
            Exchange::ItuZone => "ITU ZONE",
            Exchange::State => "STATE",
            Exchange::Grid => "GRID",
            Exchange::Power => "POWER",
            Exchange::Text => "EXCHANGE",
        }
    }
}

/// How a contest's multipliers are counted, for the live estimate. The
/// sponsor's own adjudication is authoritative; this is what the operator
/// watches during the contest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Multiplier {
    /// Every QSO counts 1 and there are no multipliers.
    None,
    /// Distinct CQ zones worked.
    CqZone,
    /// Distinct callsign prefixes worked.
    WpxPrefix,
    /// ARRL DX's rule, which is not "either" but **which one, decided by where
    /// the station is**: a W/VE station counts for the DXCC entity, and
    /// everyone else counts for the state or province. Counting `dxcc` first
    /// unconditionally — as this did — inverted it: every US and Canadian
    /// station counted its entity, and every DX station counted nothing at all,
    /// because a state is only recorded when there is a state and `dxcc`
    /// always wins.
    DxccOrState,
}

/// What a contest exchanges, and how its score is estimated.
#[derive(Debug, Clone, Copy)]
pub struct ContestSpec {
    pub name: &'static str,
    /// The base `CONTEST:` value Cabrillo wants — CW/SSB/RTTY is appended for
    /// the sponsors that split by mode, see [`cabrillo_contest`].
    pub cabrillo: &'static str,
    /// What we send.
    pub sent: &'static [Exchange],
    /// What they send.
    pub rcvd: &'static [Exchange],
    pub multiplier: Multiplier,
}

const RST_SERIAL: &[Exchange] = &[Exchange::Rst, Exchange::Serial];
const RST_ZONE: &[Exchange] = &[Exchange::Rst, Exchange::CqZone];
const RST_TEXT: &[Exchange] = &[Exchange::Rst, Exchange::Text];

impl ContestId {
    /// Every value, `None` included, for serialisation and tests.
    pub const ALL: [ContestId; 7] = [
        ContestId::None,
        ContestId::CqWw,
        ContestId::CqWpx,
        ContestId::ArrlDx,
        ContestId::EuVhf,
        ContestId::CbActivity,
        ContestId::Generic,
    ];

    /// What the picker offers — `None` is the "off" state, not a choice.
    pub const CHOICES: [ContestId; 6] = [
        ContestId::CqWw,
        ContestId::CqWpx,
        ContestId::ArrlDx,
        ContestId::EuVhf,
        ContestId::CbActivity,
        ContestId::Generic,
    ];

    pub fn label(self) -> &'static str {
        match self {
            ContestId::None => "No contest",
            ContestId::CqWw => "CQ World Wide",
            ContestId::CqWpx => "CQ WPX",
            ContestId::ArrlDx => "ARRL DX",
            ContestId::EuVhf => "EU VHF",
            ContestId::CbActivity => "CB / 11 m activity",
            ContestId::Generic => "Serial (generic)",
        }
    }

    pub fn spec(self) -> ContestSpec {
        match self {
            ContestId::None => ContestSpec {
                name: "No contest",
                cabrillo: "",
                sent: &[],
                rcvd: &[],
                multiplier: Multiplier::None,
            },
            ContestId::CqWw => ContestSpec {
                name: "CQ World Wide",
                cabrillo: "CQ-WW",
                sent: RST_ZONE,
                rcvd: RST_ZONE,
                multiplier: Multiplier::CqZone,
            },
            ContestId::CqWpx => ContestSpec {
                name: "CQ WPX",
                cabrillo: "CQ-WPX",
                sent: RST_SERIAL,
                rcvd: RST_SERIAL,
                multiplier: Multiplier::WpxPrefix,
            },
            ContestId::ArrlDx => ContestSpec {
                name: "ARRL DX",
                cabrillo: "ARRL-DX",
                // We send our state, they send theirs (or their power from
                // outside the US and Canada). One free-ish field either way.
                sent: &[Exchange::Rst, Exchange::State],
                rcvd: &[Exchange::Rst, Exchange::State],
                multiplier: Multiplier::DxccOrState,
            },
            ContestId::EuVhf => ContestSpec {
                name: "EU VHF",
                cabrillo: "EU-VHF",
                // RST + serial + our 6-character locator — the same shape we
                // receive. Sending only the serial, as this did, left our own
                // grid out of the row entirely.
                sent: &[Exchange::Rst, Exchange::Serial, Exchange::Grid],
                rcvd: &[Exchange::Rst, Exchange::Serial, Exchange::Grid],
                multiplier: Multiplier::None,
            },
            ContestId::CbActivity => ContestSpec {
                name: "CB / 11 m activity",
                cabrillo: "CB-ACTIVITY",
                sent: RST_TEXT,
                rcvd: RST_TEXT,
                multiplier: Multiplier::None,
            },
            ContestId::Generic => ContestSpec {
                name: "Serial (generic)",
                cabrillo: "GENERIC",
                sent: RST_SERIAL,
                rcvd: RST_SERIAL,
                multiplier: Multiplier::None,
            },
        }
    }

    /// The received exchange elements the operator types a box for: everything
    /// after the report.
    ///
    /// The report is separate because it is the one part filled in from the
    /// mode rather than typed — a CW report is `599` and an FT8 one is the dB
    /// the decoder measured — and the old form drew a single box for the whole
    /// received exchange, which is why an EU VHF QSO could keep its grid or its
    /// serial but never both.
    pub fn received_fields(self) -> &'static [Exchange] {
        let r = self.spec().rcvd;
        match r.first() {
            Some(Exchange::Rst) => &r[1..],
            _ => r,
        }
    }

    /// The id a QSO of this contest is tagged with in the logbook.
    ///
    /// The sponsor's identifier (`CQ-WW`, `ARRL-DX`), not the UI's label
    /// ("CQ World Wide"). The label is for people and reads badly in an
    /// exchange with anyone else's logger or in an ADIF `CONTEST_ID` field,
    /// which wants what the sponsor calls it. It also means the session's own
    /// filter does not depend on a piece of display text staying as it is.
    ///
    /// Empty for [`None`](Self::None), so an un-tagged row is what "not in a
    /// contest" looks like.
    pub fn log_id(self) -> &'static str {
        self.spec().cabrillo
    }

    /// The serial to start a fresh session at, given the rows already logged
    /// for this contest.
    ///
    /// **One past the highest serial already used**, so a station that stops
    /// and starts a session — or restarts the program — carries on rather than
    /// sending `001` again and logging a dupe of every QSO it already worked.
    /// Only rows carrying this contest's id are considered: another contest's
    /// serials are another contest's numbering.
    ///
    /// Starts at 1 when the contest has not been worked before, which is the
    /// ordinary case.
    pub fn seed_serial(self, log: &[QsoRecord]) -> u32 {
        let id = self.log_id();
        let highest = log.iter().filter(|q| q.contest_id == id).filter_map(|q| q.stx).max();
        match highest {
            Some(h) => h.saturating_add(1).max(1),
            None => 1,
        }
    }

    /// Whether the sent exchange carries an auto-incremented serial.
    pub fn sends_serial(self) -> bool {
        matches!(self.spec().sent, s if s.contains(&Exchange::Serial))
    }

    /// The contests that ride the FT8 contest exchange rather than a hand-typed
    /// one. `EU VHF` is the layout the digi side already speaks; the serial
    /// contests match WSJT-X's `RTTY Roundup` shape.
    pub fn is_ft8_contest(self) -> bool {
        matches!(self, ContestId::EuVhf | ContestId::CqWpx | ContestId::Generic)
    }
}

/// A running contest session — the operator's own state, held in the UI and
/// never on the wire.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ContestSession {
    pub contest: ContestId,
    /// The next serial to send. Kept even for contests that do not send one, so
    /// switching into a serial contest mid-session does not start at 1 again.
    pub next_serial: u32,
    /// What we send as our own exchange — our zone, state, power or CB text.
    pub my_exchange: String,
    pub started_utc: i64,
}

impl ContestSession {
    /// A session starting now, at the next serial **this contest has not
    /// already used** — see [`ContestId::seed_serial`].
    pub fn new(contest: ContestId, my_exchange: String, now: i64) -> Self {
        ContestSession { contest, next_serial: 1, my_exchange, started_utc: now }
    }

    /// The same, seeded from `log` so a restarted session does not reuse
    /// serials.
    pub fn seeded(contest: ContestId, my_exchange: String, now: i64, log: &[QsoRecord]) -> Self {
        ContestSession {
            contest,
            next_serial: contest.seed_serial(log),
            my_exchange,
            started_utc: now,
        }
    }

    /// Our own exchange as the log stores it, given the serial sitting in front
    /// of it.
    ///
    /// The serial and the typed exchange belong **on the same line** — the old
    /// code wrote the typed text *instead of* the serial, so an EU VHF row went
    /// out with our locator and no serial, which is half the exchange the other
    /// station copied.
    pub fn sent_exchange(&self, serial: Option<u32>) -> String {
        let mut parts: Vec<String> = Vec::new();
        if let Some(n) = serial {
            parts.push(n.to_string());
        }
        let ex = self.my_exchange.trim();
        if !ex.is_empty() {
            parts.push(ex.to_ascii_uppercase());
        }
        parts.join(" ")
    }
}

/// The live score the operator watches. The sponsor's own adjudication is
/// authoritative — this is an estimate, and it says so on screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ContestScore {
    pub qsos: usize,
    pub points: u32,
    pub mults: u32,
    pub total: u32,
}

/// Estimate the score of `log` for `contest`.
///
/// Points are **1 per QSO**: the sponsors' band and continent weighting is
/// theirs to adjudicate, and a wrong guess would be worse than an honest
/// baseline. The multiplier count is real, though, and is what the operator
/// watches.
pub fn score(log: &[QsoRecord], contest: ContestId) -> ContestScore {
    let spec = contest.spec();
    let mults = match spec.multiplier {
        Multiplier::None => 0,
        Multiplier::CqZone => distinct(log, |q| cq_zone_of(q)),
        Multiplier::WpxPrefix => distinct(log, |q| Some(wpx_prefix(&q.call))),
        Multiplier::DxccOrState => distinct(log, |q| arrl_dx_multiplier(q)),
    };
    let qsos = log.len() as u32;
    let total = if mults == 0 { qsos } else { qsos * mults };
    ContestScore { qsos: log.len(), points: qsos, mults, total }
}

/// A row's CQ zone, read from the field or — when the field is empty — from the
/// typed exchange.
///
/// The zone arrives as *typed text*, which is what the entry form collects, so
/// it lands in `srx_string` and `cq_zone` stays empty. Counting only the field
/// therefore counted nothing for a hand-typed session, which is the whole of
/// CQ WW's multipliers. Reading the text as well fixes the rows already in the
/// log without needing the operator to retype them.
fn cq_zone_of(q: &QsoRecord) -> Option<String> {
    if let Some(z) = q.cq_zone {
        return Some(z.to_string());
    }
    let t = q.srx_string.trim();
    if t.is_empty() {
        return None;
    }
    // A zone is 1–40 and stands alone in the exchange. Anything with other
    // text around it is not a zone and is not guessed at.
    let digits: String = t.chars().take_while(|c| c.is_ascii_digit()).collect();
    match digits.parse::<u8>() {
        Ok(z) if (1..=40).contains(&z) && t[digits.len()..].trim().is_empty() => {
            Some(z.to_string())
        }
        _ => None,
    }
}

/// ARRL DX's multiplier for one row: the entity for a W/VE station, the state
/// or province for anyone else.
fn arrl_dx_multiplier(q: &QsoRecord) -> Option<String> {
    if is_w_ve(&q.call) { q.dxcc.map(|d| d.to_string()) } else { non_empty(&q.state) }
}

/// Whether a callsign is a **W/VE** station — the United States or Canada — for
/// the contests whose multiplier rule turns on it.
///
/// Answered from the country file this program already ships rather than from
/// a hand-written prefix list. A prefix list has to enumerate every US and
/// Canadian amateur prefix (`K`, `W`, `N`, `AA`–`AL`, `VA`–`VE`, `VO`, `VY`,
/// `CY`–`CZ`, `KL`, `KP`, `VP8/K`, `KG4`, `KH6`, `KL7`, …) and would still be
/// wrong for a station operating portable from inside the country — the rule is
/// about *where the station is*, which is what the entity lookup answers.
///
/// Unknown callsigns are not W/VE. That is the conservative answer: it credits
/// a state rather than an entity, and a state is the thing that has to be
/// typed in by hand anyway.
pub fn is_w_ve(call: &str) -> bool {
    crate::entity::resolve_callsign(call).is_some_and(|e| e.flag == "US" || e.flag == "CA")
}

/// Distinct, non-empty values of `pick`, in first-seen order.
fn distinct(log: &[QsoRecord], pick: impl Fn(&QsoRecord) -> Option<String>) -> u32 {
    let mut seen: Vec<String> = Vec::new();
    for q in log {
        if let Some(v) = pick(q).filter(|v| !v.is_empty())
            && !seen.contains(&v)
        {
            seen.push(v);
        }
    }
    seen.len() as u32
}

fn non_empty(s: &str) -> Option<String> {
    let t = s.trim();
    (!t.is_empty()).then(|| t.to_ascii_uppercase())
}

/// The CQ **WPX** prefix of a callsign.
///
/// The rule is *letters, then the digit* — but three shapes need saying out
/// loud, and each one is a case where the obvious loop gives a wrong answer:
///
/// - **A callsign that starts with a digit** (`2E0`, `4X4`) is all prefix and
///   no area to find, so it is taken whole. Taking "letters then the first
///   digit" here yields `"2"` or `"4"`, which is not a prefix any station has.
/// - **A portable with an area** (`W1AW/7`) counts for the area it is working,
///   so `W7` — the digit is the one in the suffix, not the one in the base.
/// - **A call with no digit at all** (`DL/W1AW`, or a suffix-only portable)
///   takes a `0`, because WPX prefixes are letter-plus-digit and there is no
///   digit to read.
///
/// `/MM` and `/AM` are *not* portable areas — the station is at home — so they
/// fall through to the base call, which is what `split('/')` already gives.
///
/// Exact enough for the live count; WPX's special cases (a `3DA0` contest
/// prefix, say) are the sponsor's to settle.
pub fn wpx_prefix(call: &str) -> String {
    let up = call.trim().to_ascii_uppercase();
    let mut parts = up.split('/');

    let Some(base) = parts.next() else { return String::new() };

    // `W1AW/7` — a numeric suffix is the area being worked, so the prefix is
    // the base's letters plus that digit.
    if let Some(area) =
        parts.clone().find(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii_digit()))
        && let Some(d) = area.chars().next()
    {
        return format!("{}{}", letters_only(base), d);
    }

    // A leading digit means there is no letter-then-digit split to find: take
    // the call through its second digit, which is the whole of `2E0`/`4X4`.
    if base.starts_with(|c: char| c.is_ascii_digit()) {
        let mut digits = 0;
        let mut p = String::new();
        for c in base.chars() {
            p.push(c);
            if c.is_ascii_digit() {
                digits += 1;
                if digits == 2 {
                    break;
                }
            }
        }
        return p;
    }

    // The ordinary case: letters, then the first digit. No digit means the
    // `0` that makes it a prefix at all.
    match base.chars().find(|c| c.is_ascii_digit()) {
        Some(d) => format!("{}{}", letters_only(base), d),
        None => format!("{base}0"),
    }
}

/// The leading run of letters in `base` — the part of a callsign before its
/// area digit.
fn letters_only(base: &str) -> String {
    base.chars().take_while(|c| c.is_ascii_alphabetic()).collect()
}

/// QSOs whose start falls within `window_s` seconds of `now` — the rate panel's
/// number, counted over the window it names.
pub fn rate(log: &[QsoRecord], now: i64, window_s: i64) -> usize {
    log.iter().filter(|q| (0..=window_s).contains(&(now - q.start_utc))).count()
}

/// The Cabrillo v3 mode code for a logged mode string.
///
/// A table over the mode names this program actually writes to the log, not a
/// pile of `contains()` guesses. The guessing was wrong in both directions:
/// `FM` has its own Cabrillo code (added in v3 for the FM contests) and was
/// falling through to `PH`, while the data modes — `DIGU` above all, plus
/// `NAVTEX`, `PACKET`, `ALE`, `PI4`, `FSK441` and the rest — were all landing
/// on `PH` as well because none of them spelled `FT`.
///
/// Everything not named is `DG`, which is the honest default here: the phone
/// modes are a short and closed list, and everything else this program decodes
/// is a data or picture mode.
pub fn cabrillo_mode(mode: &str) -> &'static str {
    match mode.trim().to_ascii_uppercase().as_str() {
        "CW" => "CW",
        "RTTY" | "RTTY-FM" => "RY",
        // FM has carried its own code since Cabrillo v3. `FM`/`NFM`/`WFM` are
        // the VHF and UHF FM contests; logging them as phone loses the
        // distinction the sponsor's category is asking about.
        "FM" | "NFM" | "WFM" => "FM",
        // Voice and the AM family. `C-QUAM` is the C4FM modulation, which is
        // an AM relative and reads as phone; `DSB` is double sideband.
        "LSB" | "USB" | "SSB" | "AM" | "SAM" | "DSB" | "C-QUAM" => "PH",
        _ => "DG",
    }
}

/// The `CONTEST:` value, with the CW/SSB/RTTY split the sponsor's Cabrillo
/// wants where there is one, chosen from the log's own modes.
///
/// **ARRL DX does not split**, and appending a mode to it invented a contest
/// that does not exist — an RTTY-only ARRL DX session exported as
/// `ARRL-DX-RTTY`, which no sponsor recognises and no logger will read back.
/// Only CQ WW and CQ WPX are listed per mode.
pub fn cabrillo_contest(contest: ContestId, log: &[QsoRecord]) -> String {
    let spec = contest.spec();
    if spec.cabrillo.is_empty() {
        return String::new();
    }
    match contest {
        ContestId::CqWw | ContestId::CqWpx => format!("{}{}", spec.cabrillo, mode_suffix(log)),
        _ => spec.cabrillo.to_string(),
    }
}

/// `-CW`, `-SSB` or `-RTTY` from the log's own modes, whichever dominates.
/// Empty when the log is empty.
fn mode_suffix(log: &[QsoRecord]) -> &'static str {
    let count = |code: &str| log.iter().filter(|q| cabrillo_mode(&q.mode) == code).count();
    let (cw, phone, ry) = (count("CW"), count("PH"), count("RY"));
    if cw == 0 && phone == 0 && ry == 0 {
        return "";
    }
    if cw >= phone && cw >= ry {
        "-CW"
    } else if ry > phone {
        "-RTTY"
    } else {
        "-SSB"
    }
}

/// The `CATEGORY-MODE:` value: the one mode code the log is made of, or `MIXED`.
///
/// A category that is hardcoded to `MIXED` is a category that claims the
/// operator did something they may not have — an all-CW session has to say
/// `CW`, or the sponsor reads the entry as covering modes that were never on
/// the air.
pub fn cabrillo_category_mode(log: &[QsoRecord]) -> &'static str {
    let mut seen: Vec<&'static str> = Vec::new();
    for q in log {
        let m = cabrillo_mode(&q.mode);
        if !seen.contains(&m) {
            seen.push(m);
        }
    }
    match seen.len() {
        0 => "MIXED",
        1 => seen[0],
        _ => "MIXED",
    }
}

/// The whole log as a Cabrillo v3 file.
///
/// `my_exchange` is the session's own exchange, used when a QSO carries no
/// exchange of its own. Each row's own `stx_string`/`srx_string` win when set,
/// so a record logged from a session that has since changed still exports what
/// actually went out.
///
/// `created_by` is the logging program and its version, as
/// `CREATED-BY:` wants it — passed in rather than read from a crate this one
/// does not depend on, so the types stay a plain library.
pub fn to_cabrillo(
    contest: ContestId,
    my_call: &str,
    my_exchange: &str,
    log: &[QsoRecord],
    created_by: &str,
) -> String {
    let score = score(log, contest);
    let mut out = String::new();
    out.push_str("START-OF-LOG: 3.0\n");
    let c = cabrillo_contest(contest, log);
    if !c.is_empty() {
        out.push_str(&format!("CONTEST: {c}\n"));
    }
    out.push_str(&format!("CALLSIGN: {}\n", my_call.trim().to_ascii_uppercase()));
    out.push_str(&format!("CLAIMED-SCORE: {}\n", score.total));
    out.push_str("CATEGORY-OPERATOR: SINGLE-OP\n");
    out.push_str("CATEGORY-ASSISTED: NON-ASSISTED\n");
    out.push_str("CATEGORY-BAND: ALL\n");
    out.push_str(&format!("CATEGORY-MODE: {}\n", cabrillo_category_mode(log)));
    out.push_str("CATEGORY-POWER: LOW\n");
    out.push_str("CATEGORY-TRANSMITTER: ONE\n");
    out.push_str(&format!("CREATED-BY: {}\n", created_by.trim()));

    // A QSO with no frequency cannot be placed on the band, so it cannot be a
    // `QSO:` line — but dropping it silently is how a session loses QSOs it
    // actually worked and the operator never finds out. Count them and say so
    // in a comment, where a Cabrillo reader will see it and we still export a
    // file the sponsor's parser accepts.
    let mut dropped = 0usize;
    for q in log {
        let freq_khz = (q.freq_hz / 1000.0).round() as i64;
        if freq_khz <= 0 {
            dropped += 1;
            continue;
        }
        let (y, mo, d, h, mi, _) = crate::utc_ymd_hms(q.start_utc);
        let sent = cabrillo_exchange(q.stx_string.as_str(), q.stx, q.rst_sent, my_exchange);
        let rcvd = cabrillo_exchange(q.srx_string.as_str(), q.srx, q.rst_rcvd, "");
        out.push_str(&format!(
            "QSO: {freq_khz:5} {} {y:04}-{mo:02}-{d:02} {h:02}{mi:02} {} {sent} {} {rcvd}\n",
            cabrillo_mode(&q.mode),
            up_or(my_call, "NOCALL"),
            up_or(&q.call, "NOCALL"),
        ));
    }
    if dropped > 0 {
        out.push_str(&format!(
            "# {dropped} QSO{} omitted: no frequency recorded. Log them with a dial reading \
             before exporting.\n",
            if dropped == 1 { "" } else { "s" }
        ));
    }
    out.push_str("END-OF-LOG:\n");
    out
}

/// One side of a Cabrillo `QSO:` line: report then exchange. The exchange is
/// the row's own string or serial when it has one, else the session's — an
/// FT8 row carries its own, a hand-typed one usually does not.
///
/// The report is printed as the number it is, sign and all: an FT8 report is a
/// dB figure and `-12` means something different from `12`, so taking the
/// absolute value — as this did — reported a station 24 dB stronger than it
/// was. A typed `599` has no sign to lose.
fn cabrillo_exchange(s: &str, serial: Option<u32>, rst: Option<i16>, fallback: &str) -> String {
    let rpt = rst.map(|r| r.to_string()).unwrap_or_else(|| "59".to_string());
    let ex = if !s.trim().is_empty() {
        s.trim().to_ascii_uppercase()
    } else if let Some(n) = serial {
        n.to_string()
    } else {
        fallback.trim().to_ascii_uppercase()
    };
    if ex.is_empty() { rpt } else { format!("{rpt} {ex}") }
}

fn up_or(s: &str, fallback: &str) -> String {
    let t = s.trim();
    if t.is_empty() { fallback.to_string() } else { t.to_ascii_uppercase() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Mode, QsoRecord};

    fn qso(call: &str, band: &str, mode: &str, when: i64) -> QsoRecord {
        QsoRecord {
            call: call.to_string(),
            band: band.to_string(),
            mode: mode.to_string(),
            start_utc: when,
            ..Default::default()
        }
    }

    #[test]
    fn every_contest_has_a_spec_and_a_label() {
        for c in ContestId::ALL {
            let s = c.spec();
            assert!(!c.label().is_empty());
            if c != ContestId::None {
                assert!(!s.name.is_empty(), "{c:?} has no name");
                assert!(!s.sent.is_empty(), "{c:?} sends nothing");
                assert!(!s.rcvd.is_empty(), "{c:?} receives nothing");
            }
        }
        // The picker never offers "None".
        assert!(!ContestId::CHOICES.contains(&ContestId::None));
    }

    #[test]
    fn serial_contests_are_the_ones_that_say_so() {
        assert!(ContestId::CqWpx.sends_serial());
        assert!(ContestId::EuVhf.sends_serial());
        assert!(ContestId::Generic.sends_serial());
        assert!(!ContestId::CqWw.sends_serial());
        assert!(!ContestId::CbActivity.sends_serial());
    }

    #[test]
    fn wpx_prefixes_ignore_the_suffix() {
        assert_eq!(wpx_prefix("K1ABC"), "K1");
        assert_eq!(wpx_prefix("DL1ABC"), "DL1");
        assert_eq!(wpx_prefix("VP2E"), "VP2");
        assert_eq!(wpx_prefix("K1ABC/P"), "K1");
        assert_eq!(wpx_prefix("dl1abc"), "DL1");
        // `/MM` is not a portable area: the station is at sea on its home
        // prefix, so the base call decides.
        assert_eq!(wpx_prefix("K1ABC/MM"), "K1");
        assert_eq!(wpx_prefix("K1ABC/AM"), "K1");
    }

    #[test]
    fn wpx_prefixes_handle_the_shapes_the_simple_loop_gets_wrong() {
        // A call that starts with a digit has no letter-then-digit split to
        // find, and is its own prefix whole.
        assert_eq!(wpx_prefix("2E0ABC"), "2E0");
        assert_eq!(wpx_prefix("4X4ABC"), "4X4");
        // A portable with a worked area counts for that area.
        assert_eq!(wpx_prefix("W1AW/7"), "W7");
        assert_eq!(wpx_prefix("K1ABC/4"), "K4");
        // A call with no digit at all takes the `0` that makes a prefix.
        assert_eq!(wpx_prefix("DL/W1AW"), "DL0");
        assert_eq!(wpx_prefix("G0ABC/P"), "G0");
    }

    #[test]
    fn cabrillo_mode_gives_fm_and_the_data_modes_their_own_codes() {
        // FM has a code of its own in Cabrillo v3; mapping it to phone loses
        // the distinction a VHF/UHF FM contest asks about.
        assert_eq!(cabrillo_mode("FM"), "FM");
        assert_eq!(cabrillo_mode("NFM"), "FM");
        assert_eq!(cabrillo_mode("WFM"), "FM");
        // The data modes that used to fall through to PH because they do not
        // spell "FT".
        for m in [
            "DIGU",
            "DIGL",
            "NAVTEX",
            "DSC",
            "PACKET",
            "PACKET-HF",
            "ALE",
            "PI4",
            "FSK441",
            "ACARS",
            "UVPACKET",
            "RADE",
            "APRS",
        ] {
            assert_eq!(cabrillo_mode(m), "DG", "{m} should be a data mode");
        }
        // Voice keeps its code, and the AM family reads as phone.
        for m in ["SSB", "LSB", "USB", "AM", "SAM", "DSB", "C-QUAM"] {
            assert_eq!(cabrillo_mode(m), "PH", "{m} should be phone");
        }
        assert_eq!(cabrillo_mode("CW"), "CW");
        assert_eq!(cabrillo_mode("RTTY"), "RY");
    }

    #[test]
    fn arrl_dx_is_not_split_by_mode() {
        // There is no ARRL-DX-RTTY contest; inventing one writes a CONTEST:
        // line no sponsor recognises and no logger reads back.
        let cw = [qso("DL1ABC", "20M", "CW", 0)];
        assert_eq!(cabrillo_contest(ContestId::ArrlDx, &cw), "ARRL-DX");
        assert_eq!(cabrillo_contest(ContestId::EuVhf, &cw), "EU-VHF");
        // The ones that do split still do.
        assert_eq!(cabrillo_contest(ContestId::CqWw, &cw), "CQ-WW-CW");
    }

    #[test]
    fn cabrillo_category_mode_says_what_the_log_actually_is() {
        let cw = [qso("A", "20M", "CW", 0), qso("B", "20M", "CW", 1)];
        assert_eq!(cabrillo_category_mode(&cw), "CW");
        let ssb = [qso("A", "20M", "SSB", 0)];
        assert_eq!(cabrillo_category_mode(&ssb), "PH");
        let mixed = [qso("A", "20M", "CW", 0), qso("B", "20M", "SSB", 1)];
        assert_eq!(cabrillo_category_mode(&mixed), "MIXED");
        // Nothing logged is not a mode claim.
        assert_eq!(cabrillo_category_mode(&[]), "MIXED");
    }

    #[test]
    fn a_zone_typed_into_the_exchange_still_counts() {
        // The entry form collects the exchange as text, so a hand-typed CQ WW
        // session has the zone in `srx_string` and an empty `cq_zone`. Counting
        // only the field counts nothing, which is CQ WW's whole multiplier.
        let a = {
            let mut q = qso("DL1ABC", "20M", "SSB", 0);
            q.srx_string = "14".into();
            q
        };
        let b = {
            let mut q = qso("JA1ABC", "20M", "SSB", 1);
            q.srx_string = "25".into();
            q
        };
        let c = {
            let mut q = qso("DL2XYZ", "20M", "SSB", 2);
            q.srx_string = "14".into();
            q
        };
        let s = score(&[a, b, c], ContestId::CqWw);
        assert_eq!(s.mults, 2, "14 and 25, read from the typed exchange");
        // Other text in the exchange is not guessed at as a zone.
        let mut junk = qso("DL3ABC", "20M", "SSB", 3);
        junk.srx_string = "599 14".into();
        assert_eq!(score(&[junk], ContestId::CqWw).mults, 0);
    }

    #[test]
    fn arrl_dx_counts_entities_for_w_ve_and_states_for_everyone_else() {
        // A W/VE station counts its entity...
        let mut k = qso("K1ABC", "20M", "SSB", 0);
        k.dxcc = Some(291);
        // ...and a DX station counts its state/province, even though it also
        // resolved to an entity — which is what made the old rule dead.
        let mut dl = qso("DL1ABC", "20M", "SSB", 1);
        dl.dxcc = Some(230);
        dl.state = "BY".into();
        let s = score(&[k, dl], ContestId::ArrlDx);
        assert_eq!(s.mults, 2, "one entity and one state, not two entities");
        // A DX station with no state is no multiplier, rather than silently
        // counting its entity.
        let mut ja = qso("JA1ABC", "20M", "SSB", 2);
        ja.dxcc = Some(339);
        assert_eq!(score(&[ja], ContestId::ArrlDx).mults, 0);
    }

    #[test]
    fn w_ve_is_answered_from_the_country_file() {
        assert!(is_w_ve("K1ABC"));
        assert!(is_w_ve("W1AW"));
        assert!(is_w_ve("N5XYZ"));
        assert!(is_w_ve("VE3ABC"));
        assert!(is_w_ve("VA2XYZ"));
        // Portable from inside the country is still inside it.
        assert!(is_w_ve("K1ABC/7"));
        assert!(!is_w_ve("DL1ABC"));
        assert!(!is_w_ve("JA1ABC"));
        assert!(!is_w_ve("G4ABC"));
    }

    #[test]
    fn a_serial_contest_seeds_from_the_highest_serial_already_logged() {
        let mut a = qso("A", "20M", "FT8", 0);
        a.contest_id = "CQ-WPX".into();
        a.stx = Some(7);
        let mut b = qso("B", "20M", "FT8", 1);
        b.contest_id = "CQ-WPX".into();
        b.stx = Some(12);
        // Another contest's serial does not count towards this one.
        let mut c = qso("C", "20M", "FT8", 2);
        c.contest_id = "EU-VHF".into();
        c.stx = Some(900);
        // Nor does an untagged row.
        let mut d = qso("D", "20M", "FT8", 3);
        d.stx = Some(500);

        let log = [a, b, c, d];
        assert_eq!(ContestId::CqWpx.seed_serial(&log), 13);
        assert_eq!(ContestId::EuVhf.seed_serial(&log), 901);
        assert_eq!(ContestId::Generic.seed_serial(&log), 1, "nothing logged yet");

        let s = ContestSession::seeded(ContestId::CqWpx, "05".into(), 100, &log);
        assert_eq!(s.next_serial, 13);
    }

    #[test]
    fn a_qso_is_tagged_with_the_sponsor_id_not_the_ui_label() {
        assert_eq!(ContestId::CqWw.log_id(), "CQ-WW");
        assert_eq!(ContestId::ArrlDx.log_id(), "ARRL-DX");
        assert_eq!(ContestId::CbActivity.log_id(), "CB-ACTIVITY");
        assert_eq!(ContestId::None.log_id(), "", "not in a contest");
        // And it is genuinely different from the label, which is the bug.
        assert_ne!(ContestId::CqWw.log_id(), ContestId::CqWw.label());
    }

    #[test]
    fn every_received_element_gets_its_own_box() {
        // The entry form draws one box per element here, which is why an EU VHF
        // contact can carry its serial *and* its locator rather than whichever
        // was typed last.
        assert_eq!(
            ContestId::EuVhf.received_fields(),
            [Exchange::Serial, Exchange::Grid].as_slice()
        );
        assert_eq!(ContestId::CqWw.received_fields(), [Exchange::CqZone].as_slice());
        assert_eq!(ContestId::CbActivity.received_fields(), [Exchange::Text].as_slice());
        assert_eq!(ContestId::CqWpx.received_fields(), [Exchange::Serial].as_slice());
        assert_eq!(ContestId::None.received_fields(), [].as_slice());
    }

    #[test]
    fn our_sent_exchange_keeps_the_serial_and_our_own_text_on_one_line() {
        let eu = ContestSession::new(ContestId::EuVhf, "fn42ab".into(), 0);
        assert_eq!(eu.sent_exchange(Some(1)), "1 FN42AB");
        // A contest with no serial still sends our text.
        let cb = ContestSession::new(ContestId::CbActivity, "channel 23".into(), 0);
        assert_eq!(cb.sent_exchange(None), "CHANNEL 23");
        // A serial contest with no typed text still sends the serial.
        let wpx = ContestSession::new(ContestId::CqWpx, String::new(), 0);
        assert_eq!(wpx.sent_exchange(Some(5)), "5");
    }

    #[test]
    fn the_eu_vhf_sent_exchange_carries_our_locator() {
        // Sending only the serial left our own grid out of the row. Both sides
        // of an EU VHF QSO are RST + serial + locator.
        let spec = ContestId::EuVhf.spec();
        assert!(spec.sent.contains(&Exchange::Grid), "we send our locator");
        assert!(spec.rcvd.contains(&Exchange::Grid), "and receive theirs");
        assert!(ContestId::EuVhf.sends_serial());
    }

    #[test]
    fn a_report_keeps_its_sign_in_the_export() {
        // An FT8 report is a dB figure: -12 is not 12, and taking the absolute
        // value reported a station 24 dB stronger than it was.
        let mut q = qso("DL1ABC", "20M", "FT8", 1_700_000_000);
        q.freq_hz = 144_174_000.0;
        q.rst_sent = Some(-12);
        q.rst_rcvd = Some(-7);
        q.stx_string = "5".into();
        q.srx_string = "12".into();
        let out = to_cabrillo(ContestId::Generic, "k1abc", "5", &[q], "sdroxide 1.9.15");
        assert!(out.contains("K1ABC -12 5 DL1ABC -7 12"), "signs kept: {out}");
    }

    #[test]
    fn the_export_names_the_program_that_made_it_and_its_category() {
        let mut q = qso("DL1ABC", "20M", "CW", 1_700_000_000);
        q.freq_hz = 14_000_000.0;
        let out = to_cabrillo(ContestId::CqWw, "k1abc", "14", &[q], "sdroxide 1.9.15");
        assert!(out.contains("CREATED-BY: sdroxide 1.9.15\n"), "{out}");
        assert!(out.contains("CATEGORY-MODE: CW\n"), "one mode is not MIXED: {out}");
    }

    #[test]
    fn the_export_says_so_when_it_drops_a_qso_for_having_no_frequency() {
        let mut good = qso("DL1ABC", "20M", "SSB", 1_700_000_000);
        good.freq_hz = 14_250_000.0;
        // A row logged with no dial reading cannot be placed on a band, so it
        // cannot be a QSO: line — but dropping it in silence loses it.
        let mut lost = qso("K2ABC", "20M", "SSB", 1_700_000_100);
        lost.freq_hz = 0.0;
        let out = to_cabrillo(ContestId::CqWw, "k1abc", "05", &[good, lost], "sdroxide 1.9.15");
        assert!(out.contains("QSO: 14250 PH "), "{out}");
        assert!(!out.contains("K2ABC"), "the dead row is not exported: {out}");
        assert!(out.contains("# 1 QSO omitted: no frequency recorded"), "{out}");
    }

    #[test]
    fn cabrillo_has_the_header_and_a_line_per_qso() {
        let mut q = qso("DL1ABC", "20M", "SSB", 1_700_000_000);
        q.freq_hz = 14_250_000.0;
        q.rst_sent = Some(59);
        q.rst_rcvd = Some(59);
        q.stx_string = "05".into();
        q.srx_string = "14".into();
        let out = to_cabrillo(ContestId::CqWw, "k1abc", "05", &[q], "sdroxide 1.9.15");
        assert!(out.starts_with("START-OF-LOG: 3.0\n"));
        assert!(out.contains("CONTEST: CQ-WW-SSB\n"));
        assert!(out.contains("CALLSIGN: K1ABC\n"), "the call is upcased: {out}");
        assert!(out.contains("QSO: 14250 PH "), "freq in kHz, phone: {out}");
        assert!(out.contains("K1ABC 59 05 DL1ABC 59 14"), "the exchange line: {out}");
        assert!(out.trim_end().ends_with("END-OF-LOG:"));
    }

    #[test]
    fn the_zone_contest_counts_distinct_zones() {
        let mut a = qso("DL1ABC", "20M", "SSB", 1000);
        a.cq_zone = Some(14);
        let mut b = qso("JA1ABC", "20M", "SSB", 1001);
        b.cq_zone = Some(25);
        let mut c = qso("DL2XYZ", "40M", "SSB", 1002);
        c.cq_zone = Some(14); // a dupe zone, not a new multiplier
        let log = [a, b, c];
        let s = score(&log, ContestId::CqWw);
        assert_eq!(s.qsos, 3);
        assert_eq!(s.points, 3);
        assert_eq!(s.mults, 2, "14 and 25, not the repeat of 14");
        assert_eq!(s.total, 6);
    }

    #[test]
    fn a_plain_serial_contest_scores_one_each() {
        let log = [qso("K1ABC", "20M", "FT8", 10), qso("K2ABC", "20M", "FT8", 11)];
        let s = score(&log, ContestId::Generic);
        assert_eq!(s.mults, 0);
        assert_eq!(s.total, 2, "no multipliers: the score is the QSO count");
    }

    #[test]
    fn the_rate_counts_only_the_window() {
        let log = [
            qso("A", "20M", "SSB", 100),
            qso("B", "20M", "SSB", 900),
            qso("C", "20M", "SSB", 1000),
        ];
        assert_eq!(rate(&log, 1000, 600), 2, "the 900 and the 1000, not the 100");
        assert_eq!(rate(&log, 1000, 3600), 3);
    }

    #[test]
    fn cabrillo_picks_the_mode_suffix_from_the_log() {
        let ssb = [qso("DL1ABC", "20M", "SSB", 0)];
        assert_eq!(cabrillo_contest(ContestId::CqWw, &ssb), "CQ-WW-SSB");
        let cw = [qso("DL1ABC", "20M", "CW", 0), qso("G1ABC", "20M", "CW", 1)];
        assert_eq!(cabrillo_contest(ContestId::CqWw, &cw), "CQ-WW-CW");
        // A contest with no mode split keeps its base name.
        assert_eq!(cabrillo_contest(ContestId::EuVhf, &cw), "EU-VHF");
    }

    #[test]
    fn a_session_starts_at_serial_one() {
        let s = ContestSession::new(ContestId::CqWpx, "05".into(), 0);
        assert_eq!(s.next_serial, 1);
        assert_eq!(s.contest, ContestId::CqWpx);
        // The mode code mapping is what the export relies on.
        assert_eq!(cabrillo_mode("FT8"), "DG");
        assert_eq!(cabrillo_mode("USB"), "PH");
        assert_eq!(cabrillo_mode("CW"), "CW");
    }

    #[test]
    fn ft8_contests_are_the_ones_the_digi_side_can_drive() {
        assert!(ContestId::EuVhf.is_ft8_contest());
        assert!(ContestId::CqWpx.is_ft8_contest());
        assert!(ContestId::Generic.is_ft8_contest());
        assert!(!ContestId::CqWw.is_ft8_contest(), "a zone exchange is not an FT8 layout");
        assert!(!ContestId::CbActivity.is_ft8_contest());
    }

    // `Mode` is referenced so the test module fails to compile if the log's
    // mode field ever stops being a plain string the mapping can read.
    #[allow(dead_code)]
    fn _mode_is_not_structural(_m: Mode) {}
}
