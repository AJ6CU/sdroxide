//! Reading the two services' replies.

use crate::model::{
    Adsbdb, AircraftInfo, Airport, Lookup, Photo, Reply, Request, Route, clean_registration,
    page_url_ok, thumb_url_ok,
};

fn text(v: &serde_json::Value, key: &str) -> Option<String> {
    let s: String = v.get(key)?.as_str()?.chars().filter(|c| !c.is_control()).collect();
    let s = s.trim();
    (!s.is_empty()).then(|| s.chars().take(80).collect())
}

fn airport(v: Option<&serde_json::Value>) -> Option<Airport> {
    let v = v?;
    let code = text(v, "iata_code").or_else(|| text(v, "icao_code"))?;
    Some(Airport { code, name: text(v, "name").or_else(|| text(v, "municipality"))? })
}

/// Read an adsbdb reply. `None` is a reply that is not adsbdb's at all (a
/// proxy's error page, say), which the caller treats as a failure; an
/// aircraft it does not know is `Some` with nothing in it.
pub fn parse_adsbdb(body: &str) -> Option<Adsbdb> {
    let v: serde_json::Value = serde_json::from_str(body).ok()?;
    let resp = v.get("response")?;
    // "unknown aircraft" and "unknown callsign" arrive as a bare string.
    let Some(obj) = resp.as_object() else { return Some(Adsbdb::default()) };
    let aircraft = obj.get("aircraft").filter(|a| a.is_object()).map(|a| AircraftInfo {
        type_name: text(a, "type"),
        manufacturer: text(a, "manufacturer"),
        operator: text(a, "registered_owner"),
        registration: text(a, "registration").and_then(|r| clean_registration(&r)),
    });
    let route = obj.get("flightroute").filter(|r| r.is_object()).map(|r| Route {
        origin: airport(r.get("origin")),
        destination: airport(r.get("destination")),
    });
    Some(Adsbdb { aircraft, route })
}

/// Read a Planespotters reply. `None` is not understood; `Some(None)` is a
/// registration it has no photo of.
pub fn parse_photo(body: &str) -> Option<Option<Photo>> {
    let v: serde_json::Value = serde_json::from_str(body).ok()?;
    let photos = v.get("photos")?.as_array()?;
    let Some(p) = photos.first() else { return Some(None) };
    let thumb = p
        .get("thumbnail_large")
        .or_else(|| p.get("thumbnail"))
        .and_then(|t| t.get("src"))
        .and_then(|s| s.as_str())
        .filter(|u| thumb_url_ok(u));
    let page = p.get("link").and_then(|s| s.as_str()).filter(|u| page_url_ok(u));
    let who = text(p, "photographer");
    // The credit and the link are the price of the picture: without both it
    // is not passed on.
    Some(match (thumb, page, who) {
        (Some(t), Some(l), Some(w)) => {
            Some(Photo { thumb_url: t.to_string(), page_url: l.to_string(), photographer: w })
        }
        _ => None,
    })
}

/// What an HTTP answer to a data request means. 404 is "I do not know it";
/// anything that is not a 200 with a body we understand is a failure to be
/// retried later, not an answer to be remembered for a day.
pub fn reply_for(req: &Request, status: u16, body: &str) -> Reply {
    match req {
        Request::Adsbdb { .. } => Reply::Adsbdb(match status {
            200 => match parse_adsbdb(body) {
                Some(a) if a.aircraft.is_some() || a.route.is_some() => Lookup::Found(a),
                Some(_) => Lookup::NotFound,
                None => Lookup::Failed,
            },
            404 => Lookup::NotFound,
            _ => Lookup::Failed,
        }),
        Request::Photo { .. } => Reply::Photo(match (status, parse_photo(body)) {
            (200, Some(Some(p))) => Lookup::Found(p),
            (200, Some(None)) | (404, _) => Lookup::NotFound,
            _ => Lookup::Failed,
        }),
    }
}

#[cfg(test)]
pub(crate) mod fixtures {
    pub const ADSBDB: &str = r#"{"response":{
        "aircraft":{"type":"A320-214","icao_type":"A320","manufacturer":"Airbus",
          "mode_s":"484506","registration":"PH-BXA",
          "registered_owner":"KLM Royal Dutch Airlines"},
        "flightroute":{"callsign":"KLM1234",
          "origin":{"iata_code":"AMS","icao_code":"EHAM","name":"Amsterdam Airport Schiphol"},
          "destination":{"iata_code":"LHR","icao_code":"EGLL","name":"London Heathrow Airport"}}}}"#;

    pub const PHOTO: &str = r#"{"photos":[{"id":"1","thumbnail":{"src":"https://t.plnspttrs.net/a/small.jpg"},
        "thumbnail_large":{"src":"https://t.plnspttrs.net/a/large.jpg"},
        "link":"https://www.planespotters.net/photo/123/ph-bxa","photographer":"Jan Jansen"}]}"#;
}

#[cfg(test)]
mod tests {
    use super::fixtures::*;
    use super::*;

    #[test]
    fn an_adsbdb_reply_gives_the_airframe_and_the_route() {
        let a = parse_adsbdb(ADSBDB).unwrap();
        let ac = a.aircraft.unwrap();
        assert_eq!(ac.type_name.as_deref(), Some("A320-214"));
        assert_eq!(ac.manufacturer.as_deref(), Some("Airbus"));
        assert_eq!(ac.operator.as_deref(), Some("KLM Royal Dutch Airlines"));
        assert_eq!(ac.registration.as_deref(), Some("PH-BXA"));
        let r = a.route.unwrap();
        assert_eq!(r.origin.unwrap().code, "AMS");
        assert_eq!(r.destination.unwrap().name, "London Heathrow Airport");
    }

    #[test]
    fn an_unknown_aircraft_is_an_answer_with_nothing_in_it() {
        assert_eq!(parse_adsbdb(r#"{"response":"unknown aircraft"}"#), Some(Adsbdb::default()));
    }

    #[test]
    fn an_unknown_flight_still_gives_the_airframe() {
        let a =
            parse_adsbdb(r#"{"response":{"aircraft":{"type":"B738","registration":"EI-DCL"}}}"#)
                .unwrap();
        assert_eq!(a.aircraft.unwrap().type_name.as_deref(), Some("B738"));
        assert!(a.route.is_none());
    }

    #[test]
    fn a_reply_that_is_not_adsbdbs_is_not_an_answer() {
        assert_eq!(parse_adsbdb("<html>502 Bad Gateway</html>"), None);
        assert_eq!(parse_adsbdb(r#"{"error":"x"}"#), None);
        assert_eq!(parse_photo("nope"), None);
        assert_eq!(parse_photo(r#"{"x":1}"#), None);
    }

    #[test]
    fn a_missing_field_is_none_and_control_characters_are_dropped() {
        let a =
            parse_adsbdb(r#"{"response":{"aircraft":{"type":"A3\u0007 20","manufacturer":""}}}"#)
                .unwrap()
                .aircraft
                .unwrap();
        assert_eq!(a.type_name.as_deref(), Some("A3 20"));
        assert_eq!(a.manufacturer, None);
        assert_eq!(a.operator, None);
    }

    #[test]
    fn a_photo_reply_keeps_the_address_the_page_and_the_credit_unchanged() {
        let p = parse_photo(PHOTO).unwrap().unwrap();
        assert_eq!(p.thumb_url, "https://t.plnspttrs.net/a/large.jpg");
        assert_eq!(p.page_url, "https://www.planespotters.net/photo/123/ph-bxa");
        assert_eq!(p.photographer, "Jan Jansen");
        assert_eq!(parse_photo(r#"{"photos":[]}"#), Some(None));
    }

    #[test]
    fn a_photo_without_its_credit_or_link_is_not_passed_on() {
        let no_credit = PHOTO.replace("Jan Jansen", "");
        assert_eq!(parse_photo(&no_credit), Some(None));
        let no_link = PHOTO.replace("https://www.planespotters.net/photo/123/ph-bxa", "");
        assert_eq!(parse_photo(&no_link), Some(None));
    }

    #[test]
    fn a_photo_on_another_host_is_not_passed_on() {
        let elsewhere = PHOTO.replace("t.plnspttrs.net", "evil.example");
        assert_eq!(parse_photo(&elsewhere), Some(None));
    }

    #[test]
    fn an_http_answer_is_read_as_found_unknown_or_failed() {
        let req = Request::Adsbdb { icao: 1, callsign: None };
        assert!(matches!(reply_for(&req, 200, ADSBDB), Reply::Adsbdb(Lookup::Found(_))));
        assert_eq!(
            reply_for(&req, 200, r#"{"response":"unknown aircraft"}"#),
            Reply::Adsbdb(Lookup::NotFound)
        );
        assert_eq!(reply_for(&req, 404, ""), Reply::Adsbdb(Lookup::NotFound));
        for status in [429, 500, 502, 403] {
            assert_eq!(reply_for(&req, status, "x"), Reply::Adsbdb(Lookup::Failed), "{status}");
        }
        assert_eq!(reply_for(&req, 200, "<html>"), Reply::Adsbdb(Lookup::Failed));
        let req = Request::Photo { registration: "PH-BXA".into() };
        assert!(matches!(reply_for(&req, 200, PHOTO), Reply::Photo(Lookup::Found(_))));
        assert_eq!(reply_for(&req, 200, r#"{"photos":[]}"#), Reply::Photo(Lookup::NotFound));
        assert_eq!(reply_for(&req, 429, PHOTO), Reply::Photo(Lookup::Failed));
    }
}
