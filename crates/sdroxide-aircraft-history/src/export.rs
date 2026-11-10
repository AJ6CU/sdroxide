//! The tracked fleet as a table: one row per aircraft, as CSV or JSON.
//!
//! What a row holds is what the station already knows and the operator can already
//! see on a card: the callsign, the registration, the type, the operator, the two
//! ends of the route, when it was first and last heard, how many positions are
//! kept, and where each part came from (the radio, the relay, or both).
//!
//! What it never holds: a position of any kind, a picture or its address, the
//! page or the credit of a photo, or anything about who was looking. A callsign is
//! text a decoder read off the air, so a field that a spreadsheet would take for a
//! formula is made inert. Pure: no clock, no file, no socket.

use serde::Serialize;

use crate::{FORMAT_VERSION, History};

/// The columns of the CSV, in order, and the keys of each aircraft in the JSON.
pub const COLUMNS: [&str; 12] = [
    "icao",
    "callsign",
    "registration",
    "category",
    "type",
    "operator",
    "origin",
    "destination",
    "first_heard_utc",
    "last_heard_utc",
    "positions_kept",
    "source",
];

/// A format the table can be written in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    Csv,
    Json,
}

impl Format {
    /// `csv` or `json`, in any case and with spaces round it.
    pub fn parse(raw: &str) -> Option<Format> {
        match raw.trim().to_ascii_lowercase().as_str() {
            "csv" => Some(Format::Csv),
            "json" => Some(Format::Json),
            _ => None,
        }
    }

    pub fn ext(self) -> &'static str {
        match self {
            Format::Csv => "csv",
            Format::Json => "json",
        }
    }

    /// The file name a download is offered under: `aircraft-20261025.csv`.
    pub fn file_name(self, now: i64) -> String {
        let day = iso_utc(now);
        format!("aircraft-{}.{}", day[..10].replace('-', ""), self.ext())
    }

    /// The table in this format.
    pub fn render(self, rows: &[Row], scope: Scope, now: i64) -> String {
        match self {
            Format::Csv => csv(rows),
            Format::Json => json(rows, scope, now),
        }
    }
}

/// What the rows were made from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Scope {
    /// The station's record of the last day.
    History,
    /// Only the cards held in memory now.
    Live,
}

impl Scope {
    pub fn label(self) -> &'static str {
        match self {
            Scope::History => "station history, last 24 h",
            Scope::Live => "live cards, last 10 minutes",
        }
    }
}

/// One aircraft.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Row {
    pub icao: u32,
    pub callsign: Option<String>,
    pub registration: Option<String>,
    pub category: Option<String>,
    /// The airframe as the relay's card gives it.
    pub type_name: Option<String>,
    pub operator: Option<String>,
    /// Airport codes.
    pub origin: Option<String>,
    pub destination: Option<String>,
    pub first_heard: Option<i64>,
    pub last_heard: Option<i64>,
    /// Positions kept by the station; nothing where there is no history.
    pub positions: Option<usize>,
    /// Something here was heard on the air.
    pub radio: bool,
    /// Something here came from the relay's card.
    pub online: bool,
}

impl Row {
    /// Where the row's facts came from.
    pub fn source(&self) -> &'static str {
        match (self.radio, self.online) {
            (true, true) => "radio + online",
            (true, false) => "radio",
            (false, true) => "online",
            (false, false) => "",
        }
    }
}

/// A unix time as `2026-10-25T14:03:12Z`.
pub fn iso_utc(unix: i64) -> String {
    let days = unix.div_euclid(86_400);
    let secs = unix.rem_euclid(86_400);
    // Days since 1970-01-01 to a calendar date (Hinnant's civil_from_days).
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    format!(
        "{year:04}-{month:02}-{day:02}T{:02}:{:02}:{:02}Z",
        secs / 3_600,
        secs % 3_600 / 60,
        secs % 60
    )
}

/// The rows of a history, the one heard last first.
pub fn rows_from_history(h: &History) -> Vec<Row> {
    let mut rows: Vec<Row> = h
        .by_icao
        .values()
        .map(|j| {
            let card = j.card.as_ref();
            let kind: Vec<&str> = card
                .map(|c| [c.manufacturer.as_deref(), c.type_name.as_deref()])
                .into_iter()
                .flatten()
                .flatten()
                .collect();
            let heard = j.last_seen > 0;
            Row {
                icao: j.icao,
                callsign: j.callsign().map(str::to_string),
                registration: j
                    .registration()
                    .map(str::to_string)
                    .or_else(|| card.and_then(|c| c.registration.clone())),
                category: j.category.clone(),
                type_name: (!kind.is_empty()).then(|| kind.join(" ")),
                operator: card.and_then(|c| c.operator.clone()),
                origin: card.and_then(|c| c.origin.as_ref()).map(|p| p.code.clone()),
                destination: card.and_then(|c| c.destination.as_ref()).map(|p| p.code.clone()),
                first_heard: heard.then_some(j.first_seen),
                last_heard: heard.then_some(j.last_seen),
                positions: Some(j.points.len()),
                radio: heard
                    || !j.callsigns.is_empty()
                    || !j.registrations.is_empty()
                    || j.category.is_some()
                    || !j.points.is_empty(),
                online: card.is_some(),
            }
        })
        .collect();
    rows.sort_by(|a, b| b.last_heard.cmp(&a.last_heard).then(a.icao.cmp(&b.icao)));
    rows
}

/// A field made safe to open in a spreadsheet and to sit in a CSV.
pub fn csv_field(raw: &str) -> String {
    // A callsign is text off the air: one that starts like a formula is not run.
    let mut s = if raw.starts_with(['=', '+', '-', '@', '\t', '\r']) {
        format!("'{raw}")
    } else {
        raw.to_string()
    };
    if s.contains([',', '"', '\n', '\r']) {
        s = format!("\"{}\"", s.replace('"', "\"\""));
    }
    s
}

pub fn csv(rows: &[Row]) -> String {
    let mut out = COLUMNS.join(",");
    out.push('\n');
    for r in rows {
        let text = |v: &Option<String>| v.as_deref().map(csv_field).unwrap_or_default();
        let time = |v: Option<i64>| v.map(iso_utc).unwrap_or_default();
        let fields = [
            format!("{:06X}", r.icao),
            text(&r.callsign),
            text(&r.registration),
            text(&r.category),
            text(&r.type_name),
            text(&r.operator),
            text(&r.origin),
            text(&r.destination),
            time(r.first_heard),
            time(r.last_heard),
            r.positions.map(|n| n.to_string()).unwrap_or_default(),
            r.source().to_string(),
        ];
        out.push_str(&fields.join(","));
        out.push('\n');
    }
    out
}

#[derive(Serialize)]
struct JsonAircraft<'a> {
    icao: String,
    callsign: Option<&'a str>,
    registration: Option<&'a str>,
    category: Option<&'a str>,
    #[serde(rename = "type")]
    type_name: Option<&'a str>,
    operator: Option<&'a str>,
    origin: Option<&'a str>,
    destination: Option<&'a str>,
    first_heard_utc: Option<String>,
    last_heard_utc: Option<String>,
    positions_kept: Option<usize>,
    source: &'a str,
}

#[derive(Serialize)]
struct JsonOut<'a> {
    v: u32,
    generated_utc: String,
    scope: &'a str,
    aircraft: Vec<JsonAircraft<'a>>,
}

pub fn json(rows: &[Row], scope: Scope, generated_at: i64) -> String {
    let aircraft = rows
        .iter()
        .map(|r| JsonAircraft {
            icao: format!("{:06X}", r.icao),
            callsign: r.callsign.as_deref(),
            registration: r.registration.as_deref(),
            category: r.category.as_deref(),
            type_name: r.type_name.as_deref(),
            operator: r.operator.as_deref(),
            origin: r.origin.as_deref(),
            destination: r.destination.as_deref(),
            first_heard_utc: r.first_heard.map(iso_utc),
            last_heard_utc: r.last_heard.map(iso_utc),
            positions_kept: r.positions,
            source: r.source(),
        })
        .collect();
    let out = JsonOut {
        v: FORMAT_VERSION,
        generated_utc: iso_utc(generated_at),
        scope: scope.label(),
        aircraft,
    };
    serde_json::to_string_pretty(&out).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Fix, Place, SeenCard};

    const NOW: i64 = 1_000_000;
    const A: u32 = 0x4840D6;
    const B: u32 = 0x3C6444;

    fn fix(at: i64) -> Fix {
        Fix { at, lat: 52.3123, lon: 4.7634, altitude_ft: Some(35_000), on_ground: false }
    }

    fn card() -> SeenCard {
        SeenCard {
            at: NOW,
            type_name: Some("747-8".into()),
            manufacturer: Some("Boeing".into()),
            operator: Some("KLM".into()),
            registration: Some("PH-BKA".into()),
            origin: Some(Place { code: "AMS".into(), name: "Schiphol".into() }),
            destination: Some(Place { code: "JFK".into(), name: "Kennedy".into() }),
            photo_page: Some("https://www.planespotters.net/photo/1".into()),
            photo_credit: Some("A. Photographer".into()),
        }
    }

    fn history() -> History {
        let mut h = History::default();
        h.note_heard(NOW, A, Some("KLM1234"), Some("heavy"), Some(fix(NOW)));
        h.note_heard(NOW + 60, A, Some("KLM1234"), Some("heavy"), Some(fix(NOW + 60)));
        h.note_card(NOW + 5, A, card());
        h.note_heard(NOW + 500, B, Some("BAW9"), None, None);
        h
    }

    // ── the time ──────────────────────────────────────────────────────────

    #[test]
    fn a_time_is_iso_8601_in_utc() {
        assert_eq!(iso_utc(0), "1970-01-01T00:00:00Z");
        assert_eq!(iso_utc(1_000_000_000), "2001-09-09T01:46:40Z");
        assert_eq!(iso_utc(1_782_397_392), "2026-06-25T14:23:12Z");
        assert_eq!(iso_utc(951_782_400), "2000-02-29T00:00:00Z", "a leap day");
        assert_eq!(iso_utc(4_102_444_799), "2099-12-31T23:59:59Z");
        assert_eq!(iso_utc(-1), "1969-12-31T23:59:59Z");
    }

    // ── the format ────────────────────────────────────────────────────────

    #[test]
    fn a_format_is_csv_or_json_in_any_case() {
        assert_eq!(Format::parse("csv"), Some(Format::Csv));
        assert_eq!(Format::parse(" JSON "), Some(Format::Json));
        assert_eq!(Format::parse("Csv"), Some(Format::Csv));
        for bad in ["", "xml", "csv,json", "cs", "jsonl"] {
            assert_eq!(Format::parse(bad), None, "{bad:?}");
        }
        assert_eq!((Format::Csv.ext(), Format::Json.ext()), ("csv", "json"));
    }

    #[test]
    fn the_download_is_offered_under_a_dated_name() {
        let t = 1_000_000_000; // 2001-09-09T01:46:40Z
        assert_eq!(Format::Csv.file_name(t), "aircraft-20010909.csv");
        assert_eq!(Format::Json.file_name(t), "aircraft-20010909.json");
    }

    #[test]
    fn a_format_renders_the_same_table_as_the_functions() {
        let rows = rows_from_history(&history());
        assert_eq!(Format::Csv.render(&rows, Scope::History, NOW), csv(&rows));
        assert_eq!(Format::Json.render(&rows, Scope::Live, NOW), json(&rows, Scope::Live, NOW));
    }

    // ── the rows ──────────────────────────────────────────────────────────

    #[test]
    fn a_history_gives_one_row_per_aircraft_the_last_heard_first() {
        let rows = rows_from_history(&history());
        assert_eq!(rows.iter().map(|r| r.icao).collect::<Vec<_>>(), vec![B, A]);
    }

    #[test]
    fn a_row_has_the_radio_facts_and_the_cards() {
        let rows = rows_from_history(&history());
        let a = rows.iter().find(|r| r.icao == A).unwrap();
        assert_eq!(a.callsign.as_deref(), Some("KLM1234"));
        assert_eq!(a.registration.as_deref(), Some("PH-BKA"), "from the card, none was heard");
        assert_eq!(a.category.as_deref(), Some("heavy"));
        assert_eq!(a.type_name.as_deref(), Some("Boeing 747-8"));
        assert_eq!(a.operator.as_deref(), Some("KLM"));
        assert_eq!((a.origin.as_deref(), a.destination.as_deref()), (Some("AMS"), Some("JFK")));
        assert_eq!((a.first_heard, a.last_heard), (Some(NOW), Some(NOW + 60)));
        assert_eq!(a.positions, Some(2));
        assert_eq!(a.source(), "radio + online");
        let b = rows.iter().find(|r| r.icao == B).unwrap();
        assert_eq!((b.type_name.as_deref(), b.operator.as_deref()), (None, None));
        assert_eq!(b.positions, Some(0));
        assert_eq!(b.source(), "radio");
    }

    #[test]
    fn an_aircraft_known_only_by_a_card_is_online_with_no_times() {
        let mut h = History::default();
        h.note_card(NOW, A, card());
        let r = &rows_from_history(&h)[0];
        assert_eq!(r.source(), "online");
        assert_eq!((r.first_heard, r.last_heard), (None, None), "never heard on the air");
    }

    #[test]
    fn an_aircraft_heard_with_nothing_else_is_still_radio() {
        let mut h = History::default();
        h.note_heard(NOW, A, None, None, None);
        let r = &rows_from_history(&h)[0];
        assert_eq!(
            (r.callsign.as_deref(), r.category.as_deref(), r.positions),
            (None, None, Some(0))
        );
        assert_eq!(r.source(), "radio", "heard, though there is nothing to say about it");
    }

    #[test]
    fn nothing_kept_is_no_rows() {
        assert!(rows_from_history(&History::default()).is_empty());
        assert_eq!(csv(&[]).lines().count(), 1, "just the header");
        assert!(json(&[], Scope::History, NOW).contains("\"aircraft\": []"));
    }

    // ── the CSV ───────────────────────────────────────────────────────────

    #[test]
    fn the_csv_has_a_header_and_a_line_per_aircraft() {
        let text = csv(&rows_from_history(&history()));
        let mut lines = text.lines();
        assert_eq!(lines.next().unwrap(), COLUMNS.join(","));
        let b = lines.next().unwrap();
        assert!(b.starts_with("3C6444,BAW9,"), "{b}");
        let a = lines.next().unwrap();
        assert_eq!(
            a,
            "4840D6,KLM1234,PH-BKA,heavy,Boeing 747-8,KLM,AMS,JFK,\
             1970-01-12T13:46:40Z,1970-01-12T13:47:40Z,2,radio + online"
        );
        assert!(lines.next().is_none());
    }

    #[test]
    fn every_line_has_as_many_fields_as_the_header() {
        let mut rows = rows_from_history(&history());
        rows[0].callsign = Some("A,B \"C\"\nD".into());
        let text = csv(&rows);
        // Parse it back the way a CSV reader would.
        let mut fields_per_record = Vec::new();
        let (mut n, mut quoted) = (1, false);
        let mut chars = text.chars().peekable();
        while let Some(c) = chars.next() {
            match (c, quoted) {
                ('"', true) if chars.peek() == Some(&'"') => {
                    chars.next();
                }
                ('"', _) => quoted = !quoted,
                (',', false) => n += 1,
                ('\n', false) => {
                    fields_per_record.push(n);
                    n = 1;
                }
                _ => {}
            }
        }
        assert_eq!(fields_per_record, vec![COLUMNS.len(); 3], "{text}");
    }

    #[test]
    fn a_field_with_a_comma_a_quote_or_a_newline_is_quoted() {
        assert_eq!(csv_field("plain"), "plain");
        assert_eq!(csv_field("a,b"), "\"a,b\"");
        assert_eq!(csv_field("say \"hi\""), "\"say \"\"hi\"\"\"");
        assert_eq!(csv_field("two\nlines"), "\"two\nlines\"");
        assert_eq!(csv_field("carriage\rreturn"), "\"carriage\rreturn\"");
        assert_eq!(csv_field(""), "");
    }

    #[test]
    fn a_field_a_spreadsheet_would_run_is_made_inert() {
        for evil in ["=1+1", "+SUM(A1)", "-2+3", "@cmd", "\tx", "\rx"] {
            let out = csv_field(evil);
            assert!(out.starts_with('\'') || out.starts_with("\"'"), "{evil:?} -> {out:?}");
        }
        assert_eq!(csv_field("KLM1234"), "KLM1234");
        assert_eq!(csv_field("N-123"), "N-123", "a dash in the middle is nothing");
    }

    // ── the JSON ──────────────────────────────────────────────────────────

    #[test]
    fn the_json_names_its_scope_and_time_and_holds_the_same_aircraft() {
        let rows = rows_from_history(&history());
        let text = json(&rows, Scope::History, NOW + 100);
        let v: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(v["v"], FORMAT_VERSION);
        assert_eq!(v["scope"], Scope::History.label());
        assert_eq!(v["generated_utc"], iso_utc(NOW + 100));
        let list = v["aircraft"].as_array().unwrap();
        assert_eq!(list.len(), 2);
        let a = list.iter().find(|x| x["icao"] == "4840D6").unwrap();
        assert_eq!(a["callsign"], "KLM1234");
        assert_eq!(a["type"], "Boeing 747-8");
        assert_eq!(a["positions_kept"], 2);
        assert_eq!(a["source"], "radio + online");
        assert_eq!(a["last_heard_utc"], iso_utc(NOW + 60));
        let b = list.iter().find(|x| x["icao"] == "3C6444").unwrap();
        assert!(b["operator"].is_null() && b["type"].is_null(), "missing is null: {b}");
        let live = json(&rows, Scope::Live, NOW);
        assert!(live.contains(Scope::Live.label()));
    }

    #[test]
    fn the_json_keys_are_the_csv_columns() {
        let v: serde_json::Value =
            serde_json::from_str(&json(&rows_from_history(&history()), Scope::History, NOW))
                .unwrap();
        let keys: Vec<&str> =
            v["aircraft"][0].as_object().unwrap().keys().map(String::as_str).collect();
        let mut want: Vec<&str> = COLUMNS.to_vec();
        want.sort_unstable();
        let mut got = keys.clone();
        got.sort_unstable();
        assert_eq!(got, want);
    }

    // ── what it never holds ───────────────────────────────────────────────

    #[test]
    fn no_position_no_photo_no_address_in_either_format() {
        let rows = rows_from_history(&history());
        for text in [csv(&rows), json(&rows, Scope::History, NOW)] {
            for banned in [
                "52.31",
                "4.76",
                "lat",
                "lon",
                "35000",
                "altitude",
                "http",
                "planespotters",
                "Photographer",
                "cookie",
                "session",
                "thumb",
            ] {
                assert!(!text.contains(banned), "`{banned}` in {text}");
            }
        }
    }

    #[test]
    fn this_module_touches_no_file_and_no_socket() {
        let src = include_str!("export.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap()];
        for banned in ["std::fs", "File", "TcpStream", "ureq", "SystemTime"] {
            assert!(!code.contains(banned), "export.rs must not touch `{banned}`");
        }
        assert!(code.contains("rows_from_history"), "the control: the scan sees what is there");
    }
}
