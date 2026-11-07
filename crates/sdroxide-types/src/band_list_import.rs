//! Reading a band list written in another layout.
//!
//! A great many band plans exist as a flat list of rows — a name, a type, a
//! start and an end in **hertz**:
//!
//! ```json
//! {"name": "France", "country_code": "FR",
//!  "bands": [{"name": "40m - Radioamateur", "type": "amateur", "start": 7000000, "end": 7200000}]}
//! ```
//!
//! The station takes such a file in as it takes one of its own: the engine sees
//! that it is not in the band plan layout ([`looks_like_band_list`]) and converts
//! it ([`convert_band_list`]) before the usual check, so a plan written for
//! somebody else's program is one import away and needs no tool of its own.
//!
//! ## What a list can and cannot say
//!
//! A list like this is a **map of who uses what**: broadcasters, the sea, the
//! air, mobile phones — and, among the rest, the amateur bands. It is not a
//! statement of what *this operator may transmit*. Two things follow, and they
//! are the reason this converter is more careful than a field-for-field copy:
//!
//! - **It never decides the transmit lockout.** `bands` — the allocations the
//!   lockout holds the station to — are kept from the built-in IARU tables for
//!   the region. A converted plan changes what the waterfall strip draws and
//!   names; it can neither open a band for transmitting nor close one.
//! - **`amateur` does not mean amateur.** CB, PMR 446, MURS, FRS, pagers and
//!   cordless phones are all filed under it in the lists seen. Those rows are
//!   drawn as what they are named, and never as a band.
//!
//! ## What is done to the rows
//!
//! - A row that is not a range (start not before end, a number that is not one,
//!   below 0 Hz or above 300 GHz) is dropped and reported, never copied.
//! - Types are read without their trailing digit (`amateur1` is `amateur`: the
//!   digit only alternates a colour in the program the list came from).
//! - Overlapping rows are cut so that nothing overlaps: amateur rows first, then
//!   the others, each in file order, and a later row keeps only what an earlier
//!   one left free.
//! - The kind of an amateur row (`Cw`, `Digi`, `Phone`, `Beacon`, `All`) is read
//!   from its name when the name says (`CW`, `Digital`, `SSB`, `Beacon`,
//!   `All modes`) and is `All` otherwise; any other row is `Digi` for the
//!   digital services by name (DAB, DVB, LTE, GSM, ISM, …), `Phone` for
//!   broadcasting and `All` for the rest. The row's own name is its label.
//! - The plan goes into the **region of its country** and the other two regions
//!   keep the built-in tables; a list for the whole world, or for a country this
//!   table does not know, goes into all three.

use serde_json::Value;

use crate::{BandPlan, PlanMeta, Region};

/// The largest list taken in, in bytes.
pub const BAND_LIST_MAX_BYTES: usize = 1024 * 1024;

/// The most rows a list may hold.
const MAX_ROWS: usize = 20_000;

/// Frequencies above this are not radio as this program means it.
const MAX_HZ: f64 = 300e9;

/// A piece of a row narrower than this after cutting is dropped: it is a
/// rounding artefact of two rows that nearly meet, not a segment.
const MIN_PIECE_HZ: f64 = 100.0;

/// What a conversion made.
#[derive(Debug, Clone, PartialEq)]
pub struct Converted {
    /// The plan, as the text of a band plan file.
    pub text: String,
    /// Rows that were left out, each said in a line.
    pub dropped: Vec<String>,
    /// The regions the list's segments went into.
    pub regions: Vec<Region>,
}

/// Whether `text` is a band list of this layout rather than a band plan file:
/// a JSON object whose `bands` is a list of rows with a numeric `start` and
/// `end`. Cheap enough to ask of every file that arrives.
pub fn looks_like_band_list(text: &str) -> bool {
    let Ok(v) = serde_json::from_str::<Value>(text.trim_start_matches('\u{feff}')) else {
        return false;
    };
    let Some(rows) = v.get("bands").and_then(Value::as_array) else { return false };
    v.get("region1").is_none()
        && rows.first().is_some_and(|r| r.get("start").is_some_and(Value::is_number)
            && r.get("end").is_some_and(Value::is_number))
}

/// The region a country's plan belongs to, by ISO 3166 code (and `UK`, which the
/// lists use for the United Kingdom). `None` where the country is not known or
/// the list is for the whole world.
fn region_of(code: &str) -> Option<Region> {
    const R1: &[&str] = &[
        "AD", "AL", "AM", "AT", "AZ", "BA", "BE", "BG", "BY", "CH", "CY", "CZ", "DE", "DK", "EE",
        "ES", "FI", "FR", "GB", "UK", "GE", "GR", "HR", "HU", "IE", "IS", "IT", "LI", "LT", "LU",
        "LV", "MC", "MD", "ME", "MK", "MT", "NL", "NO", "PL", "PT", "RO", "RS", "RU", "SE", "SI",
        "SK", "SM", "TR", "UA", "XK", "KZ", "IL", "JO", "LB", "SY", "SA", "AE", "KW", "QA", "BH",
        "OM", "IQ", "YE", "EG", "ZA", "NG", "KE", "MA", "DZ", "TN", "LY", "GH", "ET", "TZ", "UG",
        "SN", "CI", "CM", "AO", "MZ", "ZW", "ZM", "NA", "BW",
    ];
    const R2: &[&str] = &[
        "US", "CA", "MX", "BR", "AR", "CL", "CO", "PE", "VE", "UY", "PY", "BO", "EC", "CU", "DO",
        "HT", "JM", "TT", "BS", "BB", "PA", "CR", "NI", "HN", "SV", "GT", "BZ", "GY", "SR", "PR",
        "GL",
    ];
    const R3: &[&str] = &[
        "AU", "NZ", "CN", "JP", "KR", "KP", "ID", "IN", "TH", "VN", "MY", "SG", "PH", "PK", "BD",
        "LK", "NP", "MM", "KH", "LA", "MN", "TW", "HK", "MO", "FJ", "PG", "BN", "BT", "MV", "AF",
        "IR",
    ];
    let code = code.trim().to_ascii_uppercase();
    if R1.contains(&code.as_str()) {
        Some(Region::R1)
    } else if R2.contains(&code.as_str()) {
        Some(Region::R2)
    } else if R3.contains(&code.as_str()) {
        Some(Region::R3)
    } else {
        None
    }
}

/// One row, once read.
struct Row {
    label: String,
    lo: f64,
    hi: f64,
    kind: &'static str,
    /// Drawn first when rows overlap: an amateur row, not one of the services
    /// the lists file under that name (CB, PMR, pagers…).
    ham: bool,
}

/// A name's words, lower case, split on anything that is not a letter or digit.
fn words(name: &str) -> Vec<String> {
    name.to_lowercase()
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(str::to_owned)
        .collect()
}

/// Words that say a row filed under `amateur` is not an amateur allocation.
const NOT_HAM: &[&str] = &[
    "cb", "citizen", "citizens", "pmr", "pmr446", "murs", "frs", "gmrs", "freenet", "pager",
    "pagers", "mosques", "cordless", "licence", "license", "ism", "wifi", "rfid",
];

fn is_ham_row(ty: &str, w: &[String]) -> bool {
    ty == "amateur"
        && !w.iter().any(|x| NOT_HAM.contains(&x.as_str()))
        && !(w.contains(&"low".to_string()) && w.contains(&"power".to_string()))
}

/// The kind a row is drawn as.
fn kind_of(ty: &str, ham: bool, w: &[String]) -> &'static str {
    let has = |list: &[&str]| w.iter().any(|x| list.contains(&x.as_str()));
    if ham {
        if has(&["beacon", "beacons", "ibp", "ncdxf"]) {
            return "Beacon";
        }
        if w.windows(2).any(|p| p[0] == "all" && (p[1] == "mode" || p[1] == "modes")) {
            return "All";
        }
        let cw = has(&["cw", "telegraphy", "morse"]);
        let data = has(&[
            "digital", "digi", "digimodes", "rtty", "packet", "psk", "ft8", "data", "nb", "sstv",
        ]);
        let voice = has(&[
            "ssb", "phone", "voice", "fm", "am", "dv", "repeater", "repeaters", "simplex", "lsb",
            "usb",
        ]);
        return match (cw, data, voice) {
            (_, _, true) if cw || data => "All",
            (_, _, true) => "Phone",
            (_, true, false) => "Digi",
            (true, false, false) => "Cw",
            _ => "All",
        };
    }
    let digital = has(&[
        "dab", "dvb", "lte", "gsm", "umts", "gprs", "dect", "tetra", "ism", "wifi", "lora",
        "rfid", "digital", "data", "adsb", "ads", "acars", "datalink", "cpdlc", "5g", "4g", "3g",
    ]);
    if digital {
        "Digi"
    } else if ty == "broadcast" {
        "Phone"
    } else {
        "All"
    }
}

/// The name as a label: the markers a list may put round the first and last row
/// of a band (`|40m Ham Band`) are not part of it, and spaces are single.
fn clean_label(name: &str) -> String {
    let mut out = String::new();
    for w in name.replace('|', " ").split_whitespace() {
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(w);
    }
    out.chars().take(80).collect()
}

fn number(v: &Value) -> Option<f64> {
    v.as_f64().filter(|n| n.is_finite())
}

/// Cut `[lo, hi)` by what is already taken (sorted, not overlapping) and say what
/// is left, in order.
fn free_parts(lo: f64, hi: f64, taken: &[(f64, f64)]) -> Vec<(f64, f64)> {
    let mut parts = Vec::new();
    let mut at = lo;
    for &(a, b) in taken {
        if b <= at {
            continue;
        }
        if a >= hi {
            break;
        }
        if a > at {
            parts.push((at, a.min(hi)));
        }
        at = at.max(b);
        if at >= hi {
            break;
        }
    }
    if at < hi {
        parts.push((at, hi));
    }
    parts
}

/// Add `[lo, hi)` to `taken`, keeping it sorted and merged.
fn take(taken: &mut Vec<(f64, f64)>, lo: f64, hi: f64) {
    taken.push((lo, hi));
    taken.sort_by(|a, b| a.0.total_cmp(&b.0));
    let mut merged: Vec<(f64, f64)> = Vec::with_capacity(taken.len());
    for &(a, b) in taken.iter() {
        match merged.last_mut() {
            Some(last) if a <= last.1 => last.1 = last.1.max(b),
            _ => merged.push((a, b)),
        }
    }
    *taken = merged;
}

/// Convert a band list into the text of a band plan file.
///
/// `author` is written as the plan's author: pass the name of whoever made the
/// plan, or `""` to claim nobody — which is what a plan imported at run time
/// does, since the list says nothing the station can vouch for.
pub fn convert_band_list(text: &str, author: &str) -> Result<Converted, String> {
    if text.len() > BAND_LIST_MAX_BYTES {
        return Err(format!(
            "That list is {} kB; a band list is a few tens of kB at most.",
            text.len() / 1024
        ));
    }
    let v: Value = serde_json::from_str(text.trim_start_matches('\u{feff}'))
        .map_err(|e| format!("That file is not a band list: {e}"))?;
    let rows = v
        .get("bands")
        .and_then(Value::as_array)
        .ok_or_else(|| "That file has no list of bands.".to_string())?;
    if rows.len() > MAX_ROWS {
        return Err(format!("That list has {} rows; the most taken is {MAX_ROWS}.", rows.len()));
    }

    let mut dropped = Vec::new();
    let mut read: Vec<Row> = Vec::new();
    for (i, r) in rows.iter().enumerate() {
        let n = i + 1;
        let raw_name = r.get("name").and_then(Value::as_str).unwrap_or("");
        let ty = r
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or("")
            .trim()
            .trim_end_matches(|c: char| c.is_ascii_digit())
            .to_ascii_lowercase();
        let shown = if raw_name.is_empty() { format!("row {n}") } else { format!("row {n} “{raw_name}”") };
        let (Some(lo), Some(hi)) =
            (r.get("start").and_then(number), r.get("end").and_then(number))
        else {
            dropped.push(format!("{shown}: start or end is not a number."));
            continue;
        };
        if lo < 0.0 || hi > MAX_HZ {
            dropped.push(format!("{shown}: {lo} to {hi} Hz is outside what a band can be."));
            continue;
        }
        if lo >= hi {
            dropped.push(format!("{shown}: starts at {lo} Hz and ends at {hi} Hz, which is not a range."));
            continue;
        }
        let mut label = clean_label(raw_name);
        if label.is_empty() {
            label = clean_label(&ty);
        }
        let w = words(&label);
        let ham = is_ham_row(&ty, &w);
        read.push(Row { kind: kind_of(&ty, ham, &w), label, lo, hi, ham });
    }
    if read.is_empty() {
        return Err("That list has no usable row.".into());
    }

    // Amateur rows first, then the rest, each in file order; a row keeps what
    // the ones before it left free.
    let mut taken: Vec<(f64, f64)> = Vec::new();
    let mut segments: Vec<(f64, f64, &'static str, String)> = Vec::new();
    for ham in [true, false] {
        for row in read.iter().filter(|r| r.ham == ham) {
            for (a, b) in free_parts(row.lo, row.hi, &taken) {
                if b - a < MIN_PIECE_HZ {
                    continue;
                }
                segments.push((a, b, row.kind, row.label.clone()));
                take(&mut taken, a, b);
            }
        }
    }
    segments.sort_by(|a, b| a.0.total_cmp(&b.0));

    let country = v.get("country_name").and_then(Value::as_str).unwrap_or("").trim().to_string();
    let code = v.get("country_code").and_then(Value::as_str).unwrap_or("");
    let name = v
        .get("name")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|n| !n.is_empty())
        .map_or_else(|| country.clone(), str::to_string);
    let regions: Vec<Region> = match region_of(code) {
        Some(r) => vec![r],
        None => Region::ALL.to_vec(),
    };

    // Everything but the segments of the target regions is the built-in tables:
    // the bands (so the lockout is not this list's to move), the skimmer windows,
    // and the whole of any region the list is not for.
    let mut plan = serde_json::to_value(BandPlan::iaru_defaults())
        .map_err(|e| format!("The built-in tables could not be read: {e}"))?;
    let rows_json: Vec<Value> = segments
        .iter()
        .map(|(a, b, kind, label)| {
            serde_json::json!({
                "lo_mhz": a / 1e6,
                "hi_mhz": b / 1e6,
                "kind": kind,
                "label": label,
            })
        })
        .collect();
    for region in &regions {
        let key = format!("region{}", region.number());
        plan[&key]["segments"] = Value::Array(rows_json.clone());
    }
    plan["meta"] = serde_json::to_value(PlanMeta {
        name: name.chars().take(80).collect(),
        country: country.chars().take(80).collect(),
        author: author.chars().take(80).collect(),
    })
    .map_err(|e| e.to_string())?;
    let built: BandPlan = serde_json::from_value(plan)
        .map_err(|e| format!("The converted plan could not be built: {e}"))?;
    Ok(Converted { text: built.to_json_document(), dropped, regions })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Band;

    fn list(rows: &str) -> String {
        format!(r#"{{"name":"Testland","country_name":"Testland","country_code":"FR","bands":[{rows}]}}"#)
    }

    fn row(name: &str, ty: &str, lo: u64, hi: u64) -> String {
        format!(r#"{{"name":"{name}","type":"{ty}","start":{lo},"end":{hi}}}"#)
    }

    fn convert(rows: &[String]) -> (BandPlan, Converted) {
        let c = convert_band_list(&list(&rows.join(",")), "Me").expect("converts");
        let plan: BandPlan = serde_json::from_str(&c.text).expect("the result is a band plan");
        assert!(plan.problems().is_empty(), "{:?}", plan.problems());
        (plan, c)
    }

    fn segs(plan: &BandPlan, region: Region) -> Vec<(f64, f64, String, String)> {
        plan.region(region)
            .segments()
            .iter()
            .map(|s| (s.lo, s.hi, format!("{:?}", s.kind), s.label.unwrap_or("").to_string()))
            .collect()
    }

    #[test]
    fn a_list_is_told_from_a_plan_file() {
        assert!(looks_like_band_list(&list(&row("40m", "amateur", 7_000_000, 7_200_000))));
        assert!(!looks_like_band_list(&BandPlan::default().to_json_document()));
        assert!(!looks_like_band_list("not json"));
        assert!(!looks_like_band_list(r#"{"bands": []}"#));
        assert!(!looks_like_band_list(r#"{"bands": [{"band": "M2", "lo_mhz": 144.0}]}"#));
        assert!(!looks_like_band_list("[1,2,3]"));
    }

    /// The lockout is not a list's to move: whatever a list says, the allocations
    /// are the built-in ones.
    #[test]
    fn a_converted_plan_never_moves_the_transmit_lockout() {
        let (plan, _) = convert(&[
            row("40m", "amateur", 7_000_000, 7_100_000),
            row("CB", "amateur", 26_965_000, 27_405_000),
            row("20m", "amateur", 14_000_000, 14_100_000),
        ]);
        let stock = BandPlan::default();
        for region in Region::ALL {
            for band in Band::ALL {
                assert_eq!(
                    plan.region(region).edges(band),
                    stock.region(region).edges(band),
                    "{region:?} {band:?}"
                );
            }
            assert_eq!(plan.region(region).psk_ranges(), stock.region(region).psk_ranges());
        }
    }

    #[test]
    fn the_segments_go_into_the_countrys_region_and_the_others_keep_the_tables() {
        let (plan, c) = convert(&[row("40m Ham Band", "amateur", 7_000_000, 7_200_000)]);
        assert_eq!(c.regions, vec![Region::R1], "FR is Region 1");
        let stock = BandPlan::default();
        assert_eq!(segs(&plan, Region::R1), vec![(7e6, 7.2e6, "All".into(), "40m Ham Band".into())]);
        assert_eq!(plan.region(Region::R2), stock.region(Region::R2));
        assert_eq!(plan.region(Region::R3), stock.region(Region::R3));
        assert_eq!(plan.meta().author, "Me");
        assert_eq!(plan.meta().country, "Testland");
        assert_eq!(plan.meta().name, "Testland");

        for (code, region) in [("US", Region::R2), ("AU", Region::R3), ("uk", Region::R1)] {
            let t = list(&row("x", "amateur", 7_000_000, 7_200_000)).replace("\"FR\"", &format!("\"{code}\""));
            assert_eq!(convert_band_list(&t, "").unwrap().regions, vec![region], "{code}");
        }
        // The whole world, or a country this table does not know: all three.
        for code in ["--", "ZZ", ""] {
            let t = list(&row("x", "amateur", 7_000_000, 7_200_000)).replace("\"FR\"", &format!("\"{code}\""));
            let c = convert_band_list(&t, "").unwrap();
            assert_eq!(c.regions, Region::ALL.to_vec(), "{code:?}");
        }
    }

    #[test]
    fn rows_that_are_not_ranges_are_dropped_and_said() {
        let rows = format!(
            "{},{},{},{},{},{}",
            row("good", "amateur", 7_000_000, 7_200_000),
            row("backwards", "amateur", 14_350_000, 14_000_000),
            row("empty", "amateur", 3_500_000, 3_500_000),
            row("too high", "amateur", 1_000_000_000_000, 1_000_000_001_000),
            r#"{"name":"words","type":"amateur","start":"7000000","end":7200000}"#,
            r#"{"name":"negative","type":"amateur","start":-5,"end":10}"#,
        );
        let c = convert_band_list(&list(&rows), "").unwrap();
        assert_eq!(c.dropped.len(), 5, "{:?}", c.dropped);
        assert!(c.dropped.iter().any(|d| d.contains("backwards") && d.contains("not a range")));
        assert!(c.dropped.iter().any(|d| d.contains("words")));
        let plan: BandPlan = serde_json::from_str(&c.text).unwrap();
        assert_eq!(segs(&plan, Region::R1).len(), 1, "only the good row is kept");
        assert!(convert_band_list(&list(&row("bad", "amateur", 5, 1)), "").is_err(), "nothing usable");
    }

    /// Typos of the kind a list can carry — a number with a digit too many — are
    /// either a row dropped or a segment where it is written, never silently
    /// somewhere else.
    #[test]
    fn a_row_with_a_wrong_number_is_dropped_not_guessed() {
        let (plan, c) = convert(&[
            row("fine", "marine", 2_000_000, 2_100_000),
            // 921 MHz written with a digit too many, ending before it starts.
            row("GSM-R", "cellular", 9_210_000_000, 925_000_000),
        ]);
        assert_eq!(c.dropped.len(), 1);
        assert_eq!(segs(&plan, Region::R1).len(), 1);
    }

    #[test]
    fn overlapping_rows_are_cut_amateur_first() {
        // The sea band lies across the amateur one, as in the lists seen.
        let (plan, _) = convert(&[
            row("Marine - HF", "marine", 3_500_000, 3_600_000),
            row("80m - Amateur", "amateur", 3_500_000, 3_800_000),
            row("Aviation", "aviation", 3_700_000, 3_900_000),
        ]);
        let s = segs(&plan, Region::R1);
        assert_eq!(
            s,
            vec![
                (3.5e6, 3.8e6, "All".into(), "80m - Amateur".into()),
                (3.8e6, 3.9e6, "All".into(), "Aviation".into()),
            ],
            "{s:?}"
        );
        for w in s.windows(2) {
            assert!(w[0].1 <= w[1].0, "overlap: {s:?}");
        }
    }

    #[test]
    fn a_row_is_cut_in_two_by_one_inside_it() {
        let (plan, _) = convert(&[
            row("Broadcast", "broadcast", 5_000_000, 6_000_000),
            row("Ham", "amateur", 5_400_000, 5_500_000),
        ]);
        let s = segs(&plan, Region::R1);
        assert_eq!(s.len(), 3, "{s:?}");
        assert_eq!((s[0].0, s[0].1, s[0].3.as_str()), (5e6, 5.4e6, "Broadcast"));
        assert_eq!((s[1].0, s[1].1, s[1].3.as_str()), (5.4e6, 5.5e6, "Ham"));
        assert_eq!((s[2].0, s[2].1, s[2].3.as_str()), (5.5e6, 6e6, "Broadcast"));
    }

    #[test]
    fn a_trailing_digit_on_a_type_is_a_colour_not_a_type() {
        let (plan, _) = convert(&[
            row("CW", "amateur1", 7_000_000, 7_040_000),
            row("Digital", "amateur", 7_040_000, 7_100_000),
        ]);
        let kinds: Vec<_> = segs(&plan, Region::R1).into_iter().map(|s| s.2).collect();
        assert_eq!(kinds, vec!["Cw", "Digi"]);
    }

    #[test]
    fn what_a_name_says_sets_the_kind_of_an_amateur_row() {
        for (name, kind) in [
            ("CW", "Cw"),
            ("CW, Digital", "Digi"),
            ("RTTY/Data DX", "Digi"),
            ("SSB", "Phone"),
            ("FM, DV - Repeater input", "Phone"),
            ("CW, SSB, Digital", "All"),
            ("All Modes", "All"),
            ("All modes - Satellites", "All"),
            ("IBP, regional time shared beacons", "Beacon"),
            ("NCDXF Beacons", "Beacon"),
            ("40m Ham Band", "All"),
            ("|2200m Ham Band  Tests, Transatlantic Window", "All"),
        ] {
            let (plan, _) = convert(&[row(name, "amateur", 7_000_000, 7_100_000)]);
            assert_eq!(segs(&plan, Region::R1)[0].2, kind, "{name}");
        }
    }

    #[test]
    fn other_services_are_drawn_by_what_they_are() {
        for (name, ty, kind) in [
            ("FM Broadcast", "broadcast", "Phone"),
            ("Medium Wave", "broadcast", "Phone"),
            ("DAB+ radio", "broadcast", "Digi"),
            ("DVB-T Television UHF", "broadcast", "Digi"),
            ("Mobile LTE band 20", "cellular", "Digi"),
            ("ADS-B", "aviation", "Digi"),
            ("Marine", "marine", "All"),
            ("Weather Satellites", "satellite", "All"),
        ] {
            let (plan, _) = convert(&[row(name, ty, 100_000_000, 101_000_000)]);
            assert_eq!(segs(&plan, Region::R1)[0].2, kind, "{name}");
        }
    }

    /// CB, PMR, pagers and the like are filed under `amateur` in the lists seen:
    /// they are drawn as what they are named and, being segments, can open
    /// nothing — and an amateur row over them still wins.
    #[test]
    fn services_filed_under_amateur_are_not_amateur_rows() {
        let (plan, _) = convert(&[
            row("11m - CB", "amateur", 26_960_000, 27_410_000),
            row("PMR446", "amateur", 446_000_000, 446_200_000),
            row("Pagers", "amateur", 27_750_000, 28_000_000),
            row("10m - Radioamateur", "amateur", 27_900_000, 29_700_000),
        ]);
        let s = segs(&plan, Region::R1);
        let ham: Vec<_> = s.iter().filter(|x| x.3.contains("10m")).collect();
        assert_eq!(ham.len(), 1);
        assert_eq!((ham[0].0, ham[0].1), (27.9e6, 29.7e6));
        let pagers: Vec<_> = s.iter().filter(|x| x.3 == "Pagers").collect();
        assert_eq!((pagers[0].0, pagers[0].1), (27.75e6, 27.9e6), "cut by the ham row");
        assert!(s.iter().any(|x| x.3 == "PMR446"));
    }

    #[test]
    fn labels_lose_their_band_markers_and_stray_spaces() {
        let (plan, _) = convert(&[row("|160m Ham Band   CW, Digital", "amateur", 1_800_000, 1_810_000)]);
        assert_eq!(segs(&plan, Region::R1)[0].3, "160m Ham Band CW, Digital");
    }

    #[test]
    fn what_cannot_be_a_list_is_refused() {
        assert!(convert_band_list("not json", "").is_err());
        assert!(convert_band_list(r#"{"name": "x"}"#, "").is_err());
        assert!(convert_band_list(r#"{"bands": []}"#, "").is_err());
        assert!(convert_band_list(&"x".repeat(BAND_LIST_MAX_BYTES + 1), "").is_err());
        let many = (0..MAX_ROWS + 1).map(|i| row("r", "amateur", 1000 + i as u64 * 10, 1005 + i as u64 * 10)).collect::<Vec<_>>();
        assert!(convert_band_list(&list(&many.join(",")), "").is_err(), "too many rows");
    }

    #[test]
    fn a_byte_order_mark_is_not_a_reason_to_refuse() {
        let t = format!("\u{feff}{}", list(&row("40m", "amateur", 7_000_000, 7_200_000)));
        assert!(looks_like_band_list(&t));
        assert!(convert_band_list(&t, "").is_ok());
    }
}
