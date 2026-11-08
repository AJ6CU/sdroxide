//! Callsign lookup via QRZ.com XML or HamQTH. Both use a session-key login
//! (cached briefly to avoid re-authenticating on every call), then a per-call
//! query. Responses are XML, parsed with `roxmltree`.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use std::time::{Duration, Instant};

use sdroxide_types::{CallsignInfo, Credentials, LookupProvider};

use crate::http;

/// Cached session keys: `"provider:user" -> (key, obtained_at)`.
fn cache() -> &'static Mutex<HashMap<String, (String, Instant)>> {
    static C: OnceLock<Mutex<HashMap<String, (String, Instant)>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(HashMap::new()))
}

fn cached_key(id: &str, ttl: Duration) -> Option<String> {
    let map = cache().lock().ok()?;
    map.get(id).filter(|(_, t)| t.elapsed() < ttl).map(|(k, _)| k.clone())
}

fn store_key(id: &str, key: &str) {
    if let Ok(mut map) = cache().lock() {
        map.insert(id.to_string(), (key.to_string(), Instant::now()));
    }
}

fn drop_key(id: &str) {
    if let Ok(mut map) = cache().lock() {
        map.remove(id);
    }
}

// ── The QRZ card: cache and politeness ───────────────────────────────────

/// How long a card is kept. A person's record changes rarely, and QRZ counts
/// every query against the account, so a callsign seen again within the day is
/// answered from here.
const CARD_TTL: Duration = Duration::from_secs(24 * 3600);
/// Cap on kept cards, so a long session on a busy band cannot grow without end.
const CARD_MAX: usize = 400;
/// A photo bigger than this is not fetched, and the thumbnail kept is smaller.
const PHOTO_FETCH_MAX: u64 = 400 * 1024;
const PHOTO_SIDE: u32 = 96;
/// Lookups running at once. The rest are dropped; the screen asks again when the
/// pointer returns to the callsign.
const MAX_IN_FLIGHT: usize = 4;

/// `call -> (card, obtained_at, photo_was_asked_for)`.
type Cards = HashMap<String, (CallsignInfo, Instant, bool)>;

fn cards() -> &'static Mutex<Cards> {
    static C: OnceLock<Mutex<Cards>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(HashMap::new()))
}

fn in_flight() -> &'static Mutex<std::collections::HashSet<String>> {
    static C: OnceLock<Mutex<std::collections::HashSet<String>>> = OnceLock::new();
    C.get_or_init(|| Mutex::new(std::collections::HashSet::new()))
}

/// Claim `call` for a lookup. `false` when the same call is already being
/// looked up or too many lookups are running: the caller does not start one.
pub(crate) fn begin(call: &str) -> bool {
    let Ok(mut set) = in_flight().lock() else { return false };
    if set.len() >= MAX_IN_FLIGHT {
        return false;
    }
    set.insert(call.trim().to_ascii_uppercase())
}

pub(crate) fn finish(call: &str) {
    if let Ok(mut set) = in_flight().lock() {
        set.remove(&call.trim().to_ascii_uppercase());
    }
}

fn cached_card(call: &str, with_photo: bool) -> Option<CallsignInfo> {
    let map = cards().lock().ok()?;
    let (info, at, had_photo) = map.get(call)?;
    (at.elapsed() < CARD_TTL && (*had_photo || !with_photo)).then(|| info.clone())
}

fn store_card(call: &str, info: &CallsignInfo, with_photo: bool) {
    if let Ok(mut map) = cards().lock() {
        if map.len() >= CARD_MAX {
            // Drop the oldest tenth; cheap, and the cache is only an optimisation.
            let mut ages: Vec<(String, Instant)> =
                map.iter().map(|(k, (_, t, _))| (k.clone(), *t)).collect();
            ages.sort_by_key(|(_, t)| *t);
            for (k, _) in ages.into_iter().take(CARD_MAX / 10) {
                map.remove(&k);
            }
        }
        map.insert(call.to_string(), (info.clone(), Instant::now(), with_photo));
    }
}

/// Look up `call` with the configured provider.
///
/// `card` is the QRZ card switch ([`sdroxide_types::NetworkConfig::qrz_card_active`]):
/// when it is on, the QRZ answer also carries the operator's thumbnail, an
/// unknown callsign is an empty record rather than an error (so the screen
/// stops asking), and the answer is kept for a day.
pub fn lookup(
    provider: LookupProvider,
    qrz: &Credentials,
    hamqth: &Credentials,
    call: &str,
    card: bool,
) -> Result<CallsignInfo, String> {
    let call = call.trim().to_ascii_uppercase();
    if call.is_empty() {
        return Err("empty callsign".into());
    }
    match provider {
        LookupProvider::None => Err("lookup provider not configured".into()),
        LookupProvider::Qrz => {
            if let Some(mut hit) = cached_card(&call, card) {
                if !card {
                    hit.photo = None;
                }
                return Ok(hit);
            }
            let info = match qrz_lookup(qrz, &call, card) {
                Ok(i) => i,
                Err(e) if card && e.to_ascii_lowercase().contains("not found") => {
                    CallsignInfo::for_call(&call)
                }
                Err(e) => return Err(e),
            };
            store_card(&call, &info, card);
            Ok(info)
        }
        LookupProvider::HamQth => hamqth_lookup(hamqth, &call),
    }
}

// ── QRZ.com XML ──────────────────────────────────────────────────────────

fn qrz_lookup(creds: &Credentials, call: &str, card: bool) -> Result<CallsignInfo, String> {
    if creds.user.trim().is_empty() {
        return Err("QRZ username/password not set".into());
    }
    let id = format!("qrz:{}", creds.user.trim());
    // Try a cached session first, re-authenticating once if it's stale.
    for attempt in 0..2 {
        let key = match cached_key(&id, Duration::from_secs(12 * 3600)) {
            Some(k) => k,
            None => {
                let k = qrz_login(creds)?;
                store_key(&id, &k);
                k
            }
        };
        let url = format!("https://xmldata.qrz.com/xml/current/?s={key};callsign={call}");
        let body = http::get(&url)?;
        let doc = roxmltree::Document::parse(&body).map_err(|e| e.to_string())?;
        if let Some(err) = tag_text(&doc, "Error") {
            let el = err.to_ascii_lowercase();
            if attempt == 0
                && (el.contains("session") || el.contains("timeout") || el.contains("invalid"))
            {
                drop_key(&id);
                continue;
            }
            return Err(err);
        }
        let mut info = parse_qrz(&doc, call, today());
        if card && let Some(url) = tag_text(&doc, "image").filter(|u| u.starts_with("https://")) {
            // A picture that cannot be had is no reason to lose the rest.
            info.photo = http::get_bytes(&url, PHOTO_FETCH_MAX).ok().and_then(|b| thumbnail(&b));
        }
        return Ok(info);
    }
    Err("QRZ session could not be established".into())
}

fn qrz_login(creds: &Credentials) -> Result<String, String> {
    let url = format!(
        "https://xmldata.qrz.com/xml/current/?username={};password={};agent=sdroxide",
        urlencode(creds.user.trim()),
        urlencode(creds.password.trim())
    );
    let body = http::get(&url)?;
    let doc = roxmltree::Document::parse(&body).map_err(|e| e.to_string())?;
    if let Some(k) = tag_text(&doc, "Key") {
        return Ok(k);
    }
    Err(tag_text(&doc, "Error").unwrap_or_else(|| "QRZ login failed".into()))
}

/// Today as (year, month, day), UTC, from the system clock.
fn today() -> (i32, u32, u32) {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    civil_from_days((secs / 86_400) as i64)
}

/// Days since 1970-01-01 to a civil date (Howard Hinnant's algorithm).
fn civil_from_days(z: i64) -> (i32, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    ((y + i64::from(m <= 2)) as i32, m, d)
}

/// Age in whole years from QRZ's `born`, which is a full date (`YYYY-MM-DD` or
/// `MM/DD/YYYY`) for some records and only a year for most. With a full date
/// the birthday is honoured; with a year alone the age is the year difference.
/// `None` for anything implausible: the date itself is never kept or sent.
fn age_from_born(born: &str, today: (i32, u32, u32)) -> Option<u8> {
    let b = born.trim();
    let nums: Vec<u32> = b
        .split(|c: char| !c.is_ascii_digit())
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse().ok())
        .collect();
    let (year, month, day) = match nums.as_slice() {
        [y] if *y >= 1900 => (*y as i32, 0, 0),
        [y, m, d] if *y >= 1900 => (*y as i32, *m, *d),
        [m, d, y] if *y >= 1900 => (*y as i32, *m, *d),
        _ => return None,
    };
    let mut age = today.0 - year;
    if month != 0 && (today.1, today.2) < (month, day) {
        age -= 1;
    }
    (0..=120).contains(&age).then_some(age as u8)
}

/// A JPEG thumbnail of at most [`PHOTO_SIDE`] pixels a side, or `None` when the
/// bytes are not a picture this build can read.
fn thumbnail(bytes: &[u8]) -> Option<Vec<u8>> {
    let img = image::load_from_memory_with_format(bytes, image::ImageFormat::Jpeg).ok()?;
    let small = img.thumbnail(PHOTO_SIDE, PHOTO_SIDE).to_rgb8();
    let mut out = Vec::new();
    image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 80)
        .encode_image(&small)
        .ok()?;
    Some(out)
}

fn parse_qrz(doc: &roxmltree::Document, call: &str, today: (i32, u32, u32)) -> CallsignInfo {
    let mut info = CallsignInfo::for_call(call);
    info.addr1 = tag_text(doc, "addr1");
    info.email = tag_text(doc, "email");
    info.class = tag_text(doc, "class");
    info.age = tag_text(doc, "born").and_then(|b| age_from_born(&b, today));
    let yes = |tag: &str| {
        tag_text(doc, tag).is_some_and(|v| matches!(v.as_str(), "1" | "Y" | "y" | "yes" | "Yes"))
    };
    info.qsl_lotw = yes("lotw");
    info.qsl_eqsl = yes("eqsl");
    info.qsl_paper = yes("mqsl");
    let fname = tag_text(doc, "fname").unwrap_or_default();
    let lname = tag_text(doc, "name").unwrap_or_default();
    let name = format!("{fname} {lname}").trim().to_string();
    info.name = (!name.is_empty()).then_some(name);
    info.qth = tag_text(doc, "addr2").filter(|s| !s.is_empty());
    info.grid = tag_text(doc, "grid").filter(|s| !s.is_empty());
    info.state = tag_text(doc, "state").filter(|s| !s.is_empty());
    info.county = tag_text(doc, "county").filter(|s| !s.is_empty());
    info.country = tag_text(doc, "country").filter(|s| !s.is_empty());
    info.dxcc = tag_text(doc, "dxcc").and_then(|s| s.parse().ok());
    info.cq_zone = tag_text(doc, "cqzone").and_then(|s| s.parse().ok());
    info.itu_zone = tag_text(doc, "ituzone").and_then(|s| s.parse().ok());
    info.qsl_via = tag_text(doc, "qslmgr").filter(|s| !s.is_empty());
    info
}

// ── HamQTH XML ───────────────────────────────────────────────────────────

fn hamqth_lookup(creds: &Credentials, call: &str) -> Result<CallsignInfo, String> {
    if creds.user.trim().is_empty() {
        return Err("HamQTH username/password not set".into());
    }
    let id = format!("hamqth:{}", creds.user.trim());
    for attempt in 0..2 {
        let key = match cached_key(&id, Duration::from_secs(50 * 60)) {
            Some(k) => k,
            None => {
                let k = hamqth_login(creds)?;
                store_key(&id, &k);
                k
            }
        };
        let url = format!("https://www.hamqth.com/xml.php?id={key}&callsign={call}&prg=sdroxide");
        let body = http::get(&url)?;
        let doc = roxmltree::Document::parse(&body).map_err(|e| e.to_string())?;
        if let Some(err) = tag_text(&doc, "error") {
            let el = err.to_ascii_lowercase();
            if attempt == 0 && el.contains("session") {
                drop_key(&id);
                continue;
            }
            return Err(err);
        }
        return Ok(parse_hamqth(&doc, call));
    }
    Err("HamQTH session could not be established".into())
}

/// Ask HamQTH for a session id, which is also the cheapest read-only proof
/// that a username and password are good — [`crate::upload::test_login`] uses
/// it for the logbook, since HamQTH authenticates both with the same account.
pub(crate) fn hamqth_login(creds: &Credentials) -> Result<String, String> {
    let url = format!(
        "https://www.hamqth.com/xml.php?u={}&p={}",
        urlencode(creds.user.trim()),
        urlencode(creds.password.trim())
    );
    let body = http::get(&url)?;
    let doc = roxmltree::Document::parse(&body).map_err(|e| e.to_string())?;
    if let Some(k) = tag_text(&doc, "session_id") {
        return Ok(k);
    }
    Err(tag_text(&doc, "error").unwrap_or_else(|| "HamQTH login failed".into()))
}

fn parse_hamqth(doc: &roxmltree::Document, call: &str) -> CallsignInfo {
    let mut info = CallsignInfo::for_call(call);
    info.name =
        tag_text(doc, "adr_name").or_else(|| tag_text(doc, "nick")).filter(|s| !s.is_empty());
    info.qth = tag_text(doc, "qth").or_else(|| tag_text(doc, "adr_city")).filter(|s| !s.is_empty());
    info.grid = tag_text(doc, "grid").filter(|s| !s.is_empty());
    info.state = tag_text(doc, "us_state").filter(|s| !s.is_empty());
    info.county = tag_text(doc, "us_county").filter(|s| !s.is_empty());
    info.country = tag_text(doc, "country").filter(|s| !s.is_empty());
    info.dxcc = tag_text(doc, "adif").and_then(|s| s.parse().ok());
    info.cq_zone = tag_text(doc, "cq").and_then(|s| s.parse().ok());
    info.itu_zone = tag_text(doc, "itu").and_then(|s| s.parse().ok());
    info.qsl_via = tag_text(doc, "qsl_via").filter(|s| !s.is_empty());
    info
}

// ── helpers ──────────────────────────────────────────────────────────────

/// First descendant element with tag `name` (case-insensitive), trimmed text.
fn tag_text(doc: &roxmltree::Document, name: &str) -> Option<String> {
    doc.descendants()
        .find(|n| n.tag_name().name().eq_ignore_ascii_case(name))
        .and_then(|n| n.text())
        .map(|t| t.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// Minimal percent-encoding for credentials in a query string.
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A record as QRZ's XML API answers a subscriber.
    const QRZ_XML: &str = r#"<?xml version="1.0" encoding="utf-8" ?>
<QRZDatabase version="1.34" xmlns="http://xmldata.qrz.com">
 <Callsign>
  <call>F4ABC</call><fname>Jean</fname><name>Dupont</name>
  <addr1>12 rue des Lilas</addr1><addr2>Lyon</addr2><country>France</country>
  <grid>JN25ab</grid><dxcc>227</dxcc><cqzone>14</cqzone><ituzone>27</ituzone>
  <class>Novice</class><email>f4abc@example.org</email><born>1967-06-15</born>
  <lotw>1</lotw><eqsl>0</eqsl><mqsl>1</mqsl><qslmgr>F4DEF</qslmgr>
  <image>https://s3.amazonaws.com/files.qrz.com/c/f4abc/me.jpg</image>
 </Callsign>
 <Session><Key>k</Key></Session>
</QRZDatabase>"#;

    #[test]
    fn a_full_qrz_record_fills_the_card() {
        let doc = roxmltree::Document::parse(QRZ_XML).unwrap();
        let i = parse_qrz(&doc, "F4ABC", (2026, 11, 8));
        assert_eq!(i.name.as_deref(), Some("Jean Dupont"));
        assert_eq!(i.addr1.as_deref(), Some("12 rue des Lilas"));
        assert_eq!(i.qth.as_deref(), Some("Lyon"));
        assert_eq!(i.grid.as_deref(), Some("JN25ab"));
        assert_eq!(i.class.as_deref(), Some("Novice"));
        assert_eq!(i.email.as_deref(), Some("f4abc@example.org"));
        assert_eq!(i.age, Some(59));
        assert!(i.qsl_lotw && !i.qsl_eqsl && i.qsl_paper);
        assert_eq!(i.qsl_via.as_deref(), Some("F4DEF"));
        assert!(i.photo.is_none(), "the picture is fetched separately");
    }

    #[test]
    fn a_record_without_subscriber_fields_leaves_them_empty() {
        let xml = r#"<QRZDatabase><Callsign><call>F4ABC</call><name>Dupont</name></Callsign></QRZDatabase>"#;
        let doc = roxmltree::Document::parse(xml).unwrap();
        let i = parse_qrz(&doc, "F4ABC", (2026, 11, 8));
        assert_eq!(i.name.as_deref(), Some("Dupont"));
        assert!(i.addr1.is_none() && i.email.is_none() && i.age.is_none());
        assert!(!i.qsl_lotw && !i.qsl_eqsl && !i.qsl_paper);
    }

    #[test]
    fn the_age_honours_the_birthday() {
        let d = |s| age_from_born(s, (2026, 11, 8));
        assert_eq!(d("1967-11-08"), Some(59), "birthday today");
        assert_eq!(d("1967-11-09"), Some(58), "birthday tomorrow");
        assert_eq!(d("11/09/1967"), Some(58), "US order");
        assert_eq!(d("1967"), Some(59), "a year alone is the year difference");
        assert_eq!(d("2000-02-29"), Some(26), "leap-day birth, after the day");
        assert_eq!(age_from_born("2000-02-29", (2026, 2, 28)), Some(25));
        assert_eq!(d("nonsense"), None);
        assert_eq!(d("1850"), None);
        assert_eq!(d("2030-01-01"), None, "born in the future");
    }

    #[test]
    fn the_calendar_conversion_is_right() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(19_723), (2024, 1, 1));
        assert_eq!(civil_from_days(19_782), (2024, 2, 29));
    }

    #[test]
    fn a_picture_is_shrunk_and_junk_is_refused() {
        // A 300x200 gradient, encoded as the JPEG QRZ would serve.
        let mut img = image::RgbImage::new(300, 200);
        for (x, y, p) in img.enumerate_pixels_mut() {
            *p = image::Rgb([(x % 256) as u8, (y % 256) as u8, 128]);
        }
        let mut jpeg = Vec::new();
        image::codecs::jpeg::JpegEncoder::new_with_quality(&mut jpeg, 90)
            .encode_image(&img)
            .unwrap();
        let t = thumbnail(&jpeg).expect("a JPEG shrinks");
        let back = image::load_from_memory(&t).unwrap();
        assert!(back.width() <= PHOTO_SIDE && back.height() <= PHOTO_SIDE);
        assert!(t.len() < jpeg.len());
        assert!(thumbnail(b"not a picture").is_none());
    }

    #[test]
    fn one_lookup_per_call_and_a_bounded_crowd() {
        assert!(begin("QCARD1"));
        assert!(!begin("qcard1"), "the same call is not looked up twice at once");
        finish("QCARD1");
        assert!(begin("QCARD1"));
        finish("QCARD1");
    }

    #[test]
    fn a_kept_card_answers_within_the_day_and_a_photo_request_wants_a_photo_card() {
        let mut i = CallsignInfo::for_call("QCACHE1");
        i.name = Some("Test".into());
        store_card("QCACHE1", &i, false);
        assert!(cached_card("QCACHE1", false).is_some());
        assert!(cached_card("QCACHE1", true).is_none(), "kept without a photo attempt");
        store_card("QCACHE1", &i, true);
        assert!(cached_card("QCACHE1", true).is_some());
    }
}
