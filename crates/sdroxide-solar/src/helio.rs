//! Heliographic position strings, and the one sign convention that matters.
//!
//! Three sources, three conventions:
//!
//! * DONKI `cmeAnalyses[].longitude` — numeric Stonyhurst, **West-positive**.
//! * NOAA SWPC `longitude` — numeric Stonyhurst, **East-positive**, the exact
//!   opposite.
//! * Both — a `sourceLocation` / `location` string like `N19W23`, which is
//!   unambiguous because it spells the hemisphere out.
//!
//! Everything past this module is West-positive, matching
//! [`crate::ephem::SunFrame::direction`]. Where a source offers the string
//! form, prefer it: a letter cannot be sign-flipped by a misreading.

/// Parse `N19W23` / `S08E80` into `(latitude, longitude_west)`, both degrees.
pub fn parse_location(s: &str) -> Option<(f64, f64)> {
    let s = s.trim().to_ascii_uppercase();
    if s.len() < 4 {
        return None;
    }
    let b = s.as_bytes();
    let lat_sign = match b[0] {
        b'N' => 1.0,
        b'S' => -1.0,
        _ => return None,
    };
    let split = s[1..].find(['E', 'W'])? + 1;
    let lat: f64 = s[1..split].parse().ok()?;
    let lon_sign = match b[split] {
        b'W' => 1.0,
        b'E' => -1.0,
        _ => return None,
    };
    let lon: f64 = s[split + 1..].parse().ok()?;
    if !(0.0..=90.0).contains(&lat) || !(0.0..=180.0).contains(&lon) {
        return None;
    }
    Some((lat_sign * lat, lon_sign * lon))
}

/// Render `(lat, lon_west)` back into the conventional string.
pub fn format_location(lat: f64, lon_west: f64) -> String {
    format!(
        "{}{:02.0}{}{:02.0}",
        if lat < 0.0 { 'S' } else { 'N' },
        lat.abs().round(),
        if lon_west < 0.0 { 'E' } else { 'W' },
        lon_west.abs().round(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_the_conventional_forms() {
        assert_eq!(parse_location("N19W23"), Some((19.0, 23.0)));
        assert_eq!(parse_location("S08E80"), Some((-8.0, -80.0)));
        assert_eq!(parse_location("N05W46"), Some((5.0, 46.0)));
        assert_eq!(parse_location("s25w150"), Some((-25.0, 150.0)));
        assert_eq!(parse_location(" N00E00 "), Some((0.0, 0.0)));
    }

    #[test]
    fn rejects_junk() {
        for bad in ["", "N19", "X19W23", "N19X23", "NxxWyy", "N99W23", "N19W999"] {
            assert_eq!(parse_location(bad), None, "accepted {bad:?}");
        }
    }

    #[test]
    fn round_trips() {
        for s in ["N19W23", "S08E80", "N00W00", "S25W150"] {
            let (lat, lon) = parse_location(s).unwrap();
            assert_eq!(format_location(lat, lon), s.to_ascii_uppercase());
        }
    }
}
