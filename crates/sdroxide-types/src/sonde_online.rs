//! Radiosondes other stations have heard, as SondeHub's public API tells it.
//!
//! The pieces that need no network and no window, shared by the station (which
//! asks SondeHub once for all its clients, see `sdroxide-server`) and by the
//! client (which draws them, and asks SondeHub itself only when it runs next to
//! the radio with no station in between).

/// The query radius, metres.
pub const RADIUS_M: u32 = 400_000;

/// A sonde as SondeHub last heard it.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OnlineSonde {
    pub serial: String,
    pub model: String,
    pub lat: f64,
    pub lon: f64,
    pub alt_m: Option<f64>,
    /// Vertical speed, m/s (positive up).
    pub climb_ms: Option<f64>,
    /// Horizontal speed, m/s.
    pub speed_ms: Option<f64>,
    pub freq_mhz: Option<f64>,
    /// Unix second of its last frame.
    pub unix: i64,
}

/// The address to ask, the locator rounded to 0.1° so the exact place stays
/// with the operator.
pub fn url(lat: f64, lon: f64) -> String {
    format!(
        "https://api.v2.sondehub.org/sondes?lat={:.1}&lon={:.1}&distance={RADIUS_M}",
        lat, lon
    )
}

/// Days since 1970-01-01 of a civil date (Howard Hinnant's algorithm).
fn days_from_civil(y: i64, m: i64, d: i64) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y.rem_euclid(400);
    let doy = (153 * (if m > 2 { m - 3 } else { m + 9 }) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

/// `2026-11-05T23:48:43.004000Z` → unix seconds.
fn parse_time(s: &str) -> Option<i64> {
    let (date, time) = s.split_once('T')?;
    let mut d = date.split('-');
    let (y, m, day) = (d.next()?.parse().ok()?, d.next()?.parse().ok()?, d.next()?.parse().ok()?);
    let time = time.trim_end_matches('Z');
    let time = time.split(['.', '+']).next()?;
    let mut t = time.split(':');
    let (h, mi, sec): (i64, i64, i64) =
        (t.next()?.parse().ok()?, t.next()?.parse().ok()?, t.next()?.parse().ok()?);
    Some(days_from_civil(y, m, day) * 86_400 + h * 3600 + mi * 60 + sec)
}

/// The reply: an object keyed by serial, each value the sonde's latest frame.
/// An entry without a serial, a place on Earth or a time is left out.
pub fn parse(json: &str) -> Vec<OnlineSonde> {
    let Ok(serde_json::Value::Object(map)) = serde_json::from_str::<serde_json::Value>(json)
    else {
        return Vec::new();
    };
    let mut out: Vec<OnlineSonde> = map
        .values()
        .filter_map(|v| {
            let serial = v.get("serial")?.as_str()?.trim().to_string();
            let lat = v.get("lat")?.as_f64()?;
            let lon = v.get("lon")?.as_f64()?;
            if serial.is_empty() || lat.abs() > 90.0 || lon.abs() > 180.0 {
                return None;
            }
            Some(OnlineSonde {
                serial,
                model: v.get("type").and_then(|t| t.as_str()).unwrap_or("").to_string(),
                lat,
                lon,
                alt_m: v.get("alt").and_then(|x| x.as_f64()),
                climb_ms: v.get("vel_v").and_then(|x| x.as_f64()),
                speed_ms: v.get("vel_h").and_then(|x| x.as_f64()),
                freq_mhz: v.get("frequency").and_then(|x| x.as_f64()),
                unix: parse_time(v.get("datetime")?.as_str()?)?,
            })
        })
        .collect();
    // Newest first, so a crowded map keeps the live ones.
    out.sort_by(|a, b| b.unix.cmp(&a.unix));
    out
}

/// A coordinate in tenths of a degree, the grain the station is asked at and
/// the key its answers are kept under.
pub fn tenths(v: f64) -> i32 {
    (v * 10.0).round() as i32
}

/// A station position that can be asked about: finite and on Earth.
pub fn place_ok(lat: f64, lon: f64) -> bool {
    lat.is_finite() && lon.is_finite() && lat.abs() <= 90.0 && lon.abs() <= 180.0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two entries in the shape the API answers with: the first is a real
    /// sample (a Vaisala RS41 over the Channel), the second has no serial.
    const SAMPLE: &str = r#"{
      "Y1432023": {"software_name": "radiosonde_auto_rx", "datetime": "2026-11-05T23:48:43.004000Z",
        "manufacturer": "Vaisala", "type": "RS41", "serial": "Y1432023", "subtype": "RS41-SG",
        "frame": 3490, "lat": 51.05377, "lon": -0.0041, "alt": 8799.39254, "temp": -47.6,
        "vel_v": 4.51079, "vel_h": 29.0039, "heading": 304.36783, "sats": 9, "batt": 2.8,
        "frequency": 405.799875},
      "x": {"datetime": "2026-11-05T23:48:43Z", "lat": 1.0, "lon": 2.0}
    }"#;

    #[test]
    fn a_reply_becomes_sondes_and_an_entry_without_a_serial_is_left_out() {
        let v = parse(SAMPLE);
        assert_eq!(v.len(), 1);
        let s = &v[0];
        assert_eq!(s.serial, "Y1432023");
        assert_eq!(s.model, "RS41");
        assert!((s.lat - 51.05377).abs() < 1e-9 && (s.lon + 0.0041).abs() < 1e-9);
        assert_eq!(s.alt_m, Some(8799.39254));
        assert_eq!(s.climb_ms, Some(4.51079));
        assert_eq!(s.freq_mhz, Some(405.799875));
        // 2026-11-05T23:48:43Z (checked with calendar.timegm)
        assert_eq!(s.unix, 1_793_922_523);
    }

    /// Needs the network: `cargo test -p sdroxide-ui --lib sonde_online -- --ignored --nocapture`.

    #[test]
    fn nonsense_gives_an_empty_list() {
        assert!(parse("").is_empty());
        assert!(parse("[]").is_empty());
        assert!(parse("<html>").is_empty());
        assert!(parse(r#"{"a": {"serial": "A", "lat": 95.0, "lon": 0.0, "datetime": "2026-01-01T00:00:00Z"}}"#).is_empty());
    }

    #[test]
    fn the_time_parser_matches_known_dates() {
        assert_eq!(parse_time("1970-01-01T00:00:00Z"), Some(0));
        assert_eq!(parse_time("2000-03-01T00:00:00Z"), Some(951_868_800));
        assert_eq!(parse_time("2026-11-05T23:48:43.004000Z"), Some(1_793_922_523));
        assert_eq!(parse_time("garbage"), None);
    }

    #[test]
    fn the_address_rounds_the_stations_place_and_names_no_one() {
        let u = url(48.8566, 2.3522);
        assert_eq!(u, "https://api.v2.sondehub.org/sondes?lat=48.9&lon=2.4&distance=400000");
        assert!(!u.contains("48.85") && !u.contains("2.352"));
    }

    #[test]
    fn places_are_kept_to_a_tenth_and_nonsense_is_refused() {
        assert_eq!(tenths(48.8566), 489);
        assert_eq!(tenths(-0.04), 0);
        assert!(place_ok(48.0, 2.0) && place_ok(-90.0, 180.0));
        assert!(!place_ok(91.0, 0.0) && !place_ok(0.0, f64::NAN) && !place_ok(f64::INFINITY, 0.0));
    }
}
