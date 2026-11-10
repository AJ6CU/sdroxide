//! What is asked, and what comes back.

use serde::{Deserialize, Serialize};

/// What adsbdb says about the airframe.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct AircraftInfo {
    pub type_name: Option<String>,
    pub manufacturer: Option<String>,
    pub operator: Option<String>,
    pub registration: Option<String>,
}

/// An airport as it is written on the card: a code and a name.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Airport {
    pub code: String,
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Route {
    pub origin: Option<Airport>,
    pub destination: Option<Airport>,
}

/// One adsbdb reply. Either half can be absent: an unknown flight number still
/// leaves the airframe, and an unknown airframe leaves nothing.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct Adsbdb {
    pub aircraft: Option<AircraftInfo>,
    pub route: Option<Route>,
}

/// Where a photo is, who took it and which page it belongs to — as Planespotters
/// gave them, the addresses unchanged. The picture itself is never fetched
/// here: a client loads it from `thumb_url` itself.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Photo {
    pub thumb_url: String,
    pub page_url: String,
    pub photographer: String,
}

/// What a lookup came back with.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Lookup<T> {
    Found(T),
    /// The service answered and does not know it.
    NotFound,
    /// No answer: offline, refused, or not understood. Never written to disk.
    Failed,
}

/// One request to the internet.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Request {
    Adsbdb { icao: u32, callsign: Option<String> },
    Photo { registration: String },
}

/// What came back for a [`Request`].
#[derive(Debug, Clone, PartialEq)]
pub enum Reply {
    Adsbdb(Lookup<Adsbdb>),
    Photo(Lookup<Photo>),
}

/// Upper-case letters and digits, 2 to `max` of them, with the dash a
/// registration may have when `dash` is set; anything else is not sent.
fn clean(s: &str, max: usize, dash: bool) -> Option<String> {
    let s = s.trim().to_ascii_uppercase();
    let ok = (2..=max).contains(&s.len())
        && s.bytes().all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || (dash && b == b'-'));
    ok.then_some(s)
}

pub fn clean_callsign(s: &str) -> Option<String> {
    clean(s, 8, false)
}

pub fn clean_registration(s: &str) -> Option<String> {
    clean(s, 10, true)
}

pub(crate) const ADSBDB_BASE: &str = "https://api.adsbdb.com";
pub(crate) const PLANESPOTTERS_BASE: &str = "https://api.planespotters.net";

impl Request {
    /// The part of the address after the host: exactly what the lookup needs
    /// and nothing else. The host is fixed by [`Request::base`].
    pub fn path_and_query(&self) -> String {
        match self {
            Request::Adsbdb { icao, callsign } => {
                let path = format!("/v0/aircraft/{:06X}", icao & 0xff_ffff);
                match callsign {
                    Some(c) => format!("{path}?callsign={c}"),
                    None => path,
                }
            }
            Request::Photo { registration } => format!("/pub/photos/reg/{registration}"),
        }
    }

    /// The one host this kind of request goes to.
    pub fn base(&self) -> &'static str {
        match self {
            Request::Adsbdb { .. } => ADSBDB_BASE,
            Request::Photo { .. } => PLANESPOTTERS_BASE,
        }
    }

    pub fn url(&self) -> String {
        format!("{}{}", self.base(), self.path_and_query())
    }
}

/// The `User-Agent` both services ask for: the application and a contact.
pub fn user_agent() -> String {
    format!(
        "SDR-Oxide-Brown/{} (+https://github.com/madmedicnl/sdroxide-brown)",
        env!("CARGO_PKG_VERSION")
    )
}

/// Whether an address is one Planespotters hands out for a picture or a photo
/// page. A reply naming anything else is not passed on.
fn planespotters_url(url: &str, hosts: &[&str]) -> bool {
    let Some(rest) = url.strip_prefix("https://") else { return false };
    let host = rest.split(['/', '?', '#']).next().unwrap_or("");
    !host.contains('@') && !host.contains(':') && hosts.iter().any(|h| host == *h)
}

pub fn thumb_url_ok(url: &str) -> bool {
    planespotters_url(url, &["t.plnspttrs.net"])
}

/// The largest picture the station will hold for a client.
pub const MAX_IMAGE: usize = 2 * 1024 * 1024;

/// One thumbnail, as Planespotters sent it: only ever held in memory, for the
/// one card that is open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    pub content_type: String,
    pub bytes: Vec<u8>,
}

/// Whether a reply is a picture the station keeps: a JPEG or a PNG, not empty
/// and not past [`MAX_IMAGE`]. Anything else (an HTML error page behind a 200,
/// say) is not passed on.
pub fn image_ok(content_type: &str, len: usize) -> bool {
    let kind = content_type.split(';').next().unwrap_or("").trim().to_ascii_lowercase();
    matches!(kind.as_str(), "image/jpeg" | "image/png") && len > 0 && len <= MAX_IMAGE
}

pub(crate) fn page_url_ok(url: &str) -> bool {
    planespotters_url(url, &["www.planespotters.net", "planespotters.net"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_request_carries_only_what_the_lookup_needs() {
        let r = Request::Adsbdb { icao: 0x484006, callsign: Some("KLM1234".into()) };
        assert_eq!(r.url(), "https://api.adsbdb.com/v0/aircraft/484006?callsign=KLM1234");
        let r = Request::Adsbdb { icao: 0x0a, callsign: None };
        assert_eq!(r.url(), "https://api.adsbdb.com/v0/aircraft/00000A");
        let r = Request::Photo { registration: "PH-BXA".into() };
        assert_eq!(r.url(), "https://api.planespotters.net/pub/photos/reg/PH-BXA");
    }

    #[test]
    fn each_kind_of_request_goes_to_its_own_fixed_host() {
        assert_eq!(Request::Adsbdb { icao: 1, callsign: None }.base(), "https://api.adsbdb.com");
        assert_eq!(
            Request::Photo { registration: "PH-BXA".into() }.base(),
            "https://api.planespotters.net"
        );
    }

    #[test]
    fn a_callsign_or_registration_that_could_carry_anything_else_is_not_sent() {
        assert_eq!(clean_callsign(" klm1234 "), Some("KLM1234".into()));
        assert_eq!(clean_registration("ph-bxa"), Some("PH-BXA".into()));
        for bad in ["", "A", "KLM 123", "KLM1234&x=1", "KLM/../x", "ÉÉÉÉ", "TOOLONG123"] {
            assert_eq!(clean_callsign(bad), None, "{bad:?}");
        }
        assert_eq!(clean_registration("PH-BXA?x=1"), None);
        assert_eq!(clean_callsign("KLM-12"), None, "a dash is for registrations");
    }

    #[test]
    fn the_user_agent_names_the_program_and_gives_a_contact() {
        let ua = user_agent();
        assert!(ua.starts_with("SDR-Oxide-Brown/"), "{ua}");
        assert!(ua.ends_with("(+https://github.com/madmedicnl/sdroxide-brown)"), "{ua}");
    }

    #[test]
    fn a_picture_or_page_on_another_host_is_not_followed() {
        for bad in [
            "http://t.plnspttrs.net/a.jpg",
            "https://evil.example/a.jpg",
            "https://t.plnspttrs.net.evil.example/a.jpg",
            "https://user@t.plnspttrs.net/a.jpg",
            "https://t.plnspttrs.net:8443/a.jpg",
            "https://other.plnspttrs.net/a.jpg",
            "https://plnspttrs.net/a.jpg",
        ] {
            assert!(!thumb_url_ok(bad), "{bad}");
        }
        assert!(thumb_url_ok("https://t.plnspttrs.net/a.jpg"));
        assert!(!page_url_ok("https://evil.example/photo/1"));
        assert!(page_url_ok("https://www.planespotters.net/photo/1/x"));
    }
}
