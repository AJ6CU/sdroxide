//! Radiosondes other stations have heard, from SondeHub's public API: the
//! sondes within a radius of the station, drawn beside the ones this radio
//! decodes. Read-only: **nothing is uploaded**, and nothing is asked until the
//! operator switches it on.
//!
//! Who asks depends on where this window runs:
//!
//! * connected to a station (a remote client, or the web page the station
//!   serves): `GET {station}/sonde-online?lat=..&lon=..` — the station asks
//!   SondeHub once for all its clients (`sdroxide-server`), a native client
//!   signs in with the password it already holds, and the answer is a short
//!   JSON list. A browser cannot reach SondeHub itself, so this is the only
//!   way there for the web page;
//! * on the machine with the radio, no station in between: one `GET` of
//!   `https://api.v2.sondehub.org/sondes?lat=..&lon=..&distance=..`.
//!
//! Either way it is asked every [`REFRESH_S`] seconds while the switch is on,
//! with the station's locator rounded to a tenth of a degree (about 11 km) so
//! the exact place stays with the operator. The reply is a few kilobytes, kept
//! in memory while the switch is on and dropped when it goes off.

use std::sync::mpsc::{Receiver, channel};

pub use sdroxide_types::sonde_online::OnlineSonde;
#[cfg(not(target_arch = "wasm32"))]
use sdroxide_types::sonde_online::{parse, url};

/// How often the list is asked for again.
pub const REFRESH_S: i64 = 60;
/// The most bytes read of a reply.
#[cfg(not(target_arch = "wasm32"))]
const MAX_BYTES: u64 = 4 * 1024 * 1024;

/// The station's address for the list, the locator rounded to 0.1°.
pub fn station_url(peer: &str, lat: f64, lon: f64) -> Option<String> {
    let base = crate::login::http_base(&crate::login::station_key(peer))?;
    Some(format!("{base}/sonde-online?lat={lat:.1}&lon={lon:.1}"))
}

/// The station's answer: a JSON array of sondes, or nothing usable.
pub fn parse_station(body: &str) -> Option<Vec<OnlineSonde>> {
    let mut v: Vec<OnlineSonde> = serde_json::from_str(body).ok()?;
    v.sort_by(|a, b| b.unix.cmp(&a.unix));
    Some(v)
}

/// Why an ask got no list.
#[derive(Debug, Clone, PartialEq)]
pub enum Why {
    /// The station wants the operator signed in.
    SignIn,
    /// The station is asking SondeHub as often as it may.
    Busy,
    /// The station could not reach SondeHub.
    Upstream,
    /// The answer was not a list.
    Unreadable,
    /// This window has no station and no way out of its own (the browser build).
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    NoStation,
    /// Any other status.
    Http(u16),
    /// The network library's own words (no connection, timeout…).
    Net(String),
}

impl Why {
    /// The line for the hover text.
    pub fn text(&self) -> String {
        use crate::i18n::tr;
        match self {
            Why::SignIn => tr!("sonde-online-why-signin"),
            Why::Busy => tr!("sonde-online-why-busy"),
            Why::Upstream => tr!("sonde-online-why-upstream"),
            Why::Unreadable => tr!("sonde-online-why-unreadable"),
            Why::NoStation => tr!("sonde-online-why-no-station"),
            Why::Http(s) => tr!("sonde-online-why-http", n = *s),
            Why::Net(e) => e.clone(),
        }
    }

    fn of_status(s: u16) -> Why {
        match s {
            401 => Why::SignIn,
            429 => Why::Busy,
            502 => Why::Upstream,
            s => Why::Http(s),
        }
    }
}

/// What an ask came back with.
enum Reply {
    Sondes(Vec<OnlineSonde>),
    Failed(Why),
}

/// The switch, the list and the asking.
#[derive(Default)]
pub struct Online {
    pub on: bool,
    pub sondes: Vec<OnlineSonde>,
    /// The last ask got no usable answer.
    pub failed: bool,
    /// Why, for the hover text.
    pub why: Option<Why>,
    last_ask: i64,
    inflight: Option<Receiver<Reply>>,
}

impl Online {
    /// Called every frame: collect an answer, and ask again when it is time.
    /// With the switch off, or no station position, nothing is asked and
    /// nothing is kept. `peer` is the station this window is connected to.
    pub fn tick(
        &mut self,
        ctx: &eframe::egui::Context,
        home: Option<(f64, f64)>,
        now: i64,
        peer: Option<&str>,
    ) {
        if let Some(rx) = &self.inflight {
            match rx.try_recv() {
                Ok(Reply::Sondes(v)) => {
                    self.sondes = v;
                    self.failed = false;
                    self.why = None;
                    self.inflight = None;
                }
                Ok(Reply::Failed(why)) => {
                    self.failed = true;
                    self.why = Some(why);
                    self.inflight = None;
                }
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    self.failed = true;
                    self.why = Some(Why::Net("the request stopped".into()));
                    self.inflight = None;
                }
                Err(std::sync::mpsc::TryRecvError::Empty) => {}
            }
        }
        let Some((lat, lon)) = home.filter(|_| self.on) else {
            self.sondes.clear();
            self.failed = false;
            self.why = None;
            self.inflight = None;
            self.last_ask = 0;
            return;
        };
        if self.inflight.is_none() && now - self.last_ask >= REFRESH_S {
            self.last_ask = now;
            self.ask(ctx, lat, lon, peer);
        }
    }

    fn ask(&mut self, ctx: &eframe::egui::Context, lat: f64, lon: f64, peer: Option<&str>) {
        let (tx, rx) = channel();
        self.inflight = Some(rx);
        let ctx = ctx.clone();
        let via =
            peer.and_then(|p| station_url(p, lat, lon).map(|u| (u, crate::login::station_key(p))));
        #[cfg(not(target_arch = "wasm32"))]
        std::thread::spawn(move || {
            let reply = match via {
                Some((address, station)) => fetch_via_station(&address, &station),
                None => fetch(&url(lat, lon), None).map(|b| parse(&b)),
            };
            let _ = tx.send(match reply {
                Ok(v) => Reply::Sondes(v),
                Err(why) => Reply::Failed(why),
            });
            ctx.request_repaint();
        });
        #[cfg(target_arch = "wasm32")]
        wasm_bindgen_futures::spawn_local(async move {
            let reply = match via {
                Some((address, _)) => web_fetch(&address).await,
                None => Err(Why::NoStation),
            };
            let _ = tx.send(match reply {
                Ok(v) => Reply::Sondes(v),
                Err(why) => Reply::Failed(why),
            });
            ctx.request_repaint();
        });
    }
}

/// The station's cookie for each station address, for the length of the run.
#[cfg(not(target_arch = "wasm32"))]
static COOKIES: std::sync::LazyLock<std::sync::Mutex<std::collections::BTreeMap<String, String>>> =
    std::sync::LazyLock::new(Default::default);

#[cfg(not(target_arch = "wasm32"))]
fn agent() -> ureq::Agent {
    ureq::Agent::config_builder()
        .timeout_connect(Some(std::time::Duration::from_secs(10)))
        .timeout_global(Some(std::time::Duration::from_secs(30)))
        .http_status_as_error(false)
        .build()
        .into()
}

/// One `GET`: the body of a 200, or why there is none.
#[cfg(not(target_arch = "wasm32"))]
fn fetch(url: &str, cookie: Option<&str>) -> Result<String, Why> {
    get(&agent(), url, cookie).and_then(|(status, body)| match status {
        200 => Ok(body),
        s => Err(Why::of_status(s)),
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn get(agent: &ureq::Agent, url: &str, cookie: Option<&str>) -> Result<(u16, String), Why> {
    let mut req =
        agent.get(url).header("User-Agent", "sdroxide-brown").header("Accept", "application/json");
    if let Some(c) = cookie {
        req = req.header("Cookie", c);
    }
    let mut r = req.call().map_err(|e| Why::Net(e.to_string()))?;
    let status = r.status().as_u16();
    let body = r.body_mut().with_config().limit(MAX_BYTES).read_to_string().unwrap_or_default();
    Ok((status, body))
}

/// The same trade a browser makes: the password this client already signed in
/// with for the station's cookie.
#[cfg(not(target_arch = "wasm32"))]
fn sign_in(agent: &ureq::Agent, station: &str) -> Option<String> {
    let access = crate::login::session_access(station)?;
    let url = format!("{}/signin", crate::login::http_base(station)?);
    let body = serde_json::json!({
        "username": access.username,
        "password": access.password,
        "remember": false,
    })
    .to_string();
    let resp = agent.post(&url).header("content-type", "application/json").send(body).ok()?;
    if resp.status().as_u16() != 200 {
        return None;
    }
    let set = resp.headers().get("set-cookie")?.to_str().ok()?;
    Some(set.split(';').next()?.trim().to_string())
}

#[cfg(not(target_arch = "wasm32"))]
fn fetch_via_station(address: &str, station: &str) -> Result<Vec<OnlineSonde>, Why> {
    let agent = agent();
    let cookie = COOKIES.lock().unwrap_or_else(|e| e.into_inner()).get(station).cloned();
    let mut got = get(&agent, address, cookie.as_deref())?;
    if got.0 == 401 {
        if let Some(c) = sign_in(&agent, station) {
            COOKIES
                .lock()
                .unwrap_or_else(|e| e.into_inner())
                .insert(station.to_string(), c.clone());
            got = get(&agent, address, Some(&c))?;
        }
    }
    match got {
        (200, body) => parse_station(&body).ok_or(Why::Unreadable),
        (s, _) => Err(Why::of_status(s)),
    }
}

/// The browser's ask: same origin, with the cookie it already holds.
#[cfg(target_arch = "wasm32")]
async fn web_fetch(address: &str) -> Result<Vec<OnlineSonde>, Why> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window().ok_or(Why::NoStation)?;
    let init = web_sys::RequestInit::new();
    init.set_method("GET");
    init.set_mode(web_sys::RequestMode::SameOrigin);
    let request = web_sys::Request::new_with_str_and_init(address, &init)
        .map_err(|_| Why::Unreadable)?;
    let response = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|_| Why::Net("no answer".into()))?
        .dyn_into::<web_sys::Response>()
        .map_err(|_| Why::Unreadable)?;
    match response.status() {
        200 => {}
        s => return Err(Why::of_status(s)),
    }
    let text = wasm_bindgen_futures::JsFuture::from(
        response.text().map_err(|_| Why::Unreadable)?,
    )
    .await
    .ok()
    .and_then(|t| t.as_string())
    .unwrap_or_default();
    if text.len() as u64 > 4 * 1024 * 1024 {
        return Err(Why::Unreadable);
    }
    parse_station(&text).ok_or(Why::Unreadable)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sonde(serial: &str, unix: i64) -> OnlineSonde {
        OnlineSonde {
            serial: serial.into(),
            model: "RS41".into(),
            lat: 48.0,
            lon: 2.0,
            alt_m: Some(1000.0),
            climb_ms: None,
            speed_ms: None,
            freq_mhz: Some(405.8),
            unix,
            ..Default::default()
        }
    }

    #[test]
    fn the_stations_address_names_the_rounded_place_and_nothing_else() {
        let u = station_url("wss://radio.example/ws/2", 48.8566, 2.3522).unwrap();
        assert_eq!(u, "https://radio.example/sonde-online?lat=48.9&lon=2.4");
        assert_eq!(
            station_url("ws://192.168.1.5:8080/ws", 1.0, -2.0).unwrap(),
            "http://192.168.1.5:8080/sonde-online?lat=1.0&lon=-2.0"
        );
        assert!(station_url("not an address", 1.0, 2.0).is_none());
    }

    #[test]
    fn the_stations_list_is_read_newest_first_and_nonsense_is_refused() {
        let json =
            serde_json::to_string(&vec![sonde("A", 10), sonde("B", 30), sonde("C", 20)]).unwrap();
        let v = parse_station(&json).unwrap();
        assert_eq!(v.iter().map(|s| s.serial.as_str()).collect::<Vec<_>>(), ["B", "C", "A"]);
        assert!(parse_station("").is_none() && parse_station("{}").is_none());
        assert_eq!(parse_station("[]"), Some(Vec::new()));
    }

    #[test]
    fn with_the_switch_off_nothing_is_kept() {
        let mut o = Online { sondes: vec![sonde("A", 1)], failed: true, ..Default::default() };
        o.tick(&eframe::egui::Context::default(), Some((48.0, 2.0)), 1_000, None);
        assert!(o.sondes.is_empty() && !o.failed, "off clears the list and the failure");
        assert!(o.inflight.is_none(), "and asks nothing");
    }

    #[test]
    fn with_no_station_position_nothing_is_asked() {
        let mut o = Online { on: true, ..Default::default() };
        o.tick(&eframe::egui::Context::default(), None, 1_000, Some("ws://x/ws"));
        assert!(o.inflight.is_none());
    }

    /// Needs the network: `cargo test -p sdroxide-ui --lib sonde_online -- --ignored`.
    #[test]
    #[ignore]
    fn the_real_api_answers() {
        let body = fetch(&url(48.86, 2.35), None).expect("SondeHub answers");
        assert!(!parse(&body).is_empty());
    }
}
