//! The local time and the temperature at an airport, for the airport card.
//!
//! The station is told a latitude and a longitude (the airport's, from the table
//! embedded in the program, but a client could send anything) and builds the
//! address itself: [`Spot::new`] accepts only two finite numbers inside the
//! globe and rounds them to two decimals (about a kilometre), so the host is
//! always Open-Meteo's and a request carries nothing but a place that is
//! already public. Nothing of the station's or the client's position goes out.
//!
//! What comes back is reduced to [`Weather`]: the offset from UTC (so the
//! client can show the exact local time, summer time included, from its own
//! clock) and, when there is one, the current temperature. Nothing is kept on
//! disk; the station holds an answer in memory for half a minute.

use serde::{Deserialize, Serialize};

/// Where Open-Meteo answers. Free for non-commercial use, no key.
#[cfg(feature = "fetch")]
pub(crate) const OPEN_METEO_BASE: &str = "https://api.open-meteo.com";

/// The version of [`Weather`]'s shape. Bump on any change a client cannot ignore.
pub const WEATHER_VERSION: u32 = 1;

/// The farthest a zone is from UTC, in seconds (UTC+14 is the largest in use).
const MAX_OFFSET_S: i32 = 15 * 3600;

/// Temperatures outside this are not the air at an airport but a bad reading.
const TEMP_RANGE_C: std::ops::RangeInclusive<f32> = -90.0..=60.0;

/// A place on the globe, as far as the station will ask about it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Spot {
    lat: f64,
    lon: f64,
}

impl Spot {
    /// `lat` in -90..=90 and `lon` in -180..=180, both finite; rounded to two
    /// decimals so that two cards for one airport share an answer and a request
    /// says no more than a kilometre's worth of place.
    pub fn new(lat: f64, lon: f64) -> Option<Spot> {
        let ok = lat.is_finite()
            && lon.is_finite()
            && (-90.0..=90.0).contains(&lat)
            && (-180.0..=180.0).contains(&lon);
        let round = |v: f64| (v * 100.0).round() / 100.0 + 0.0; // `+ 0.0` turns -0.0 into 0.0
        ok.then(|| Spot { lat: round(lat), lon: round(lon) })
    }

    /// `52.31,4.76`: what two requests for the same place share.
    pub fn key(&self) -> String {
        format!("{:.2},{:.2}", self.lat, self.lon)
    }

    /// The forecast API's path and query for this place: the current
    /// temperature, and the zone chosen by the place (`timezone=auto`) so the
    /// answer carries its offset from UTC.
    pub fn path_and_query(&self) -> String {
        format!(
            "/v1/forecast?latitude={:.2}&longitude={:.2}&current=temperature_2m&timezone=auto",
            self.lat, self.lon
        )
    }
}

/// What the station tells a client about the weather at a place.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Weather {
    /// [`WEATHER_VERSION`].
    pub v: u32,
    /// The zone's offset from UTC right now, in seconds, east positive. Summer
    /// time is in it.
    pub utc_offset_s: i32,
    /// The air temperature in °C, when the service gave one.
    pub temp_c: Option<f32>,
}

#[derive(Deserialize)]
struct Forecast {
    utc_offset_seconds: Option<i64>,
    current: Option<Current>,
}

#[derive(Deserialize)]
struct Current {
    temperature_2m: Option<f64>,
}

/// The weather in an Open-Meteo reply, or `None` if the body is not one, has no
/// zone, or the zone is not one on Earth. A temperature that is missing or not
/// believable is left out; the offset alone is still worth an answer.
pub fn parse_forecast(json: &str) -> Option<Weather> {
    let f: Forecast = serde_json::from_str(json).ok()?;
    let offset = i32::try_from(f.utc_offset_seconds?).ok()?;
    if offset.abs() > MAX_OFFSET_S {
        return None;
    }
    let temp_c = f
        .current
        .and_then(|c| c.temperature_2m)
        .map(|t| t as f32)
        .filter(|t| t.is_finite() && TEMP_RANGE_C.contains(t));
    Some(Weather { v: WEATHER_VERSION, utc_offset_s: offset, temp_c })
}

#[cfg(test)]
pub(crate) const FORECAST: &str = r#"{"latitude":52.3,"longitude":4.76,"generationtime_ms":0.05,
    "utc_offset_seconds":3600,"timezone":"Europe/Amsterdam","timezone_abbreviation":"GMT+1",
    "elevation":-3.0,"current_units":{"time":"iso8601","interval":"seconds","temperature_2m":"°C"},
    "current":{"time":"2026-10-31T14:15","interval":900,"temperature_2m":11.6}}"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_place_is_rounded_to_a_kilometre_and_builds_the_whole_request() {
        let s = Spot::new(52.308_613, 4.763_889).unwrap();
        assert_eq!(s.key(), "52.31,4.76");
        assert_eq!(
            s.path_and_query(),
            "/v1/forecast?latitude=52.31&longitude=4.76&current=temperature_2m&timezone=auto"
        );
        let south_west = Spot::new(-33.946_1, -118.408).unwrap();
        assert_eq!(south_west.key(), "-33.95,-118.41");
        assert_eq!(Spot::new(-0.001, 0.0).unwrap().key(), "0.00,0.00", "no negative zero");
    }

    #[test]
    fn a_place_off_the_globe_or_not_a_number_is_not_asked_about() {
        for (lat, lon) in [
            (91.0, 0.0),
            (-90.5, 0.0),
            (0.0, 180.5),
            (0.0, -181.0),
            (f64::NAN, 0.0),
            (0.0, f64::INFINITY),
        ] {
            assert!(Spot::new(lat, lon).is_none(), "{lat},{lon}");
        }
        assert!(Spot::new(90.0, 180.0).is_some() && Spot::new(-90.0, -180.0).is_some());
    }

    #[test]
    fn a_reply_gives_the_offset_and_the_temperature() {
        let w = parse_forecast(FORECAST).unwrap();
        assert_eq!(w.utc_offset_s, 3600);
        assert_eq!(w.v, WEATHER_VERSION);
        assert!((w.temp_c.unwrap() - 11.6).abs() < 0.01);
    }

    #[test]
    fn a_reply_without_a_temperature_still_gives_the_clock() {
        for body in [
            r#"{"utc_offset_seconds":19800}"#,
            r#"{"utc_offset_seconds":19800,"current":{"time":"x"}}"#,
            r#"{"utc_offset_seconds":19800,"current":{"temperature_2m":null}}"#,
            r#"{"utc_offset_seconds":19800,"current":{"temperature_2m":900.0}}"#,
        ] {
            let w = parse_forecast(body).unwrap();
            assert_eq!((w.utc_offset_s, w.temp_c), (19800, None), "{body}");
        }
    }

    #[test]
    fn a_reply_that_is_not_a_forecast_or_has_no_real_zone_is_nothing() {
        for body in [
            "",
            "<html>",
            "{}",
            r#"{"error":true,"reason":"Latitude must be in range"}"#,
            r#"{"utc_offset_seconds":999999,"current":{"temperature_2m":10.0}}"#,
            r#"{"utc_offset_seconds":-99999999999,"current":{"temperature_2m":10.0}}"#,
            r#"{"utc_offset_seconds":"3600"}"#,
        ] {
            assert_eq!(parse_forecast(body), None, "{body}");
        }
    }

    #[test]
    fn the_answer_to_a_client_survives_a_round_trip_as_json() {
        let w = parse_forecast(FORECAST).unwrap();
        let back: Weather = serde_json::from_str(&serde_json::to_string(&w).unwrap()).unwrap();
        assert_eq!(back, w);
    }
}
