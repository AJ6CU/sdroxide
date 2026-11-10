//! The part that really talks to the network: one blocking request at a time.

use std::time::Duration;

use crate::model::{ADSBDB_BASE, MAX_IMAGE, PLANESPOTTERS_BASE};
use crate::weather::OPEN_METEO_BASE;
use crate::{
    Article, Image, Reply, Request, Spot, Upstream, Weather, image_ok, image_url_ok,
    parse_forecast, parse_summary, reply_for, user_agent,
};

/// A reply bigger than this is not read: both services answer in a few hundred
/// bytes, and a body past this is not theirs.
const MAX_BODY: u64 = 256 * 1024;
const TIMEOUT: Duration = Duration::from_secs(8);

/// Sends [`Request`]s to adsbdb and Planespotters, with the station's own
/// `User-Agent`. Every failure — no route, a refusal, a timeout, a body that is
/// too large — is a [`reply_for`] status 0, which is a failure to try again in
/// five minutes and never a panic or an error to the caller.
pub struct HttpUpstream {
    agent: ureq::Agent,
    adsbdb: String,
    planespotters: String,
    /// Where Wikipedia summaries are asked; `None` is `https://xx.wikipedia.org`.
    /// Only a test sets it.
    wikipedia: Option<String>,
    /// Where Open-Meteo is asked; `None` is the real host. Only a test sets it.
    open_meteo: Option<String>,
}

impl HttpUpstream {
    /// The two real hosts.
    pub fn new() -> HttpUpstream {
        HttpUpstream::with_bases(ADSBDB_BASE, PLANESPOTTERS_BASE)
    }

    /// Other hosts, for a test that stands one up on loopback. The program
    /// itself only ever uses [`HttpUpstream::new`].
    pub fn with_bases(adsbdb: &str, planespotters: &str) -> HttpUpstream {
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .user_agent(user_agent())
            .timeout_global(Some(TIMEOUT))
            .http_status_as_error(false)
            .build()
            .into();
        HttpUpstream {
            agent,
            adsbdb: adsbdb.to_string(),
            planespotters: planespotters.to_string(),
            wikipedia: None,
            open_meteo: None,
        }
    }

    /// Ask Wikipedia summaries of another host, for a test on loopback.
    pub fn with_wikipedia(mut self, base: &str) -> HttpUpstream {
        self.wikipedia = Some(base.to_string());
        self
    }

    /// Ask Open-Meteo of another host, for a test on loopback.
    pub fn with_open_meteo(mut self, base: &str) -> HttpUpstream {
        self.open_meteo = Some(base.to_string());
        self
    }

    /// The address of the article's thumbnail, from its summary. Wikipedia's
    /// REST API wants the `User-Agent` the other two do.
    fn wikipedia_thumb_url(&self, article: &Article) -> Option<String> {
        let url = match &self.wikipedia {
            Some(base) => format!("{base}{}", article.summary_path()),
            None => format!("https://{}{}", article.host(), article.summary_path()),
        };
        let mut resp = self.agent.get(&url).call().ok()?;
        if resp.status().as_u16() != 200 {
            return None;
        }
        let body = resp.body_mut().with_config().limit(MAX_BODY).read_to_string().ok()?;
        parse_summary(&body)
    }

    fn url(&self, req: &Request) -> String {
        let base = match req {
            Request::Adsbdb { .. } => &self.adsbdb,
            Request::Photo { .. } => &self.planespotters,
        };
        format!("{base}{}", req.path_and_query())
    }
}

impl Default for HttpUpstream {
    fn default() -> Self {
        HttpUpstream::new()
    }
}

impl Upstream for HttpUpstream {
    fn fetch(&self, req: &Request) -> Reply {
        let (status, body) = match self.agent.get(&self.url(req)).call() {
            Ok(mut resp) => {
                let status = resp.status().as_u16();
                match resp.body_mut().with_config().limit(MAX_BODY).read_to_string() {
                    Ok(body) => (status, body),
                    Err(_) => (0, String::new()),
                }
            }
            Err(_) => (0, String::new()),
        };
        reply_for(req, status, &body)
    }

    /// A plain GET of the picture, nothing of the client sent along. The caller
    /// has already checked the address; the type and size are checked here too
    /// so a body is never read past [`MAX_IMAGE`].
    fn image(&self, url: &str) -> Option<Image> {
        let mut resp = self.agent.get(url).call().ok()?;
        if resp.status().as_u16() != 200 {
            return None;
        }
        let content_type =
            resp.headers().get("content-type").and_then(|v| v.to_str().ok())?.to_string();
        if !image_ok(&content_type, 1) {
            return None;
        }
        let bytes = resp.body_mut().with_config().limit(MAX_IMAGE as u64).read_to_vec().ok()?;
        image_ok(&content_type, bytes.len()).then_some(Image { content_type, bytes })
    }

    /// The summary names the thumbnail, then the thumbnail is fetched: two plain
    /// GETs, the second only to an address [`image_url_ok`] accepted.
    fn wikipedia_thumbnail(&self, article: &Article) -> Option<Image> {
        let url = self.wikipedia_thumb_url(article).filter(|u| image_url_ok(u))?;
        self.image(&url)
    }

    /// One plain GET of the forecast API, with the station's `User-Agent` and
    /// nothing of the client: the place is the only thing in the request.
    fn weather(&self, spot: &Spot) -> Option<Weather> {
        let base = self.open_meteo.as_deref().unwrap_or(OPEN_METEO_BASE);
        let mut resp = self.agent.get(&format!("{base}{}", spot.path_and_query())).call().ok()?;
        if resp.status().as_u16() != 200 {
            return None;
        }
        let body = resp.body_mut().with_config().limit(MAX_BODY).read_to_string().ok()?;
        parse_forecast(&body)
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::sync::mpsc;

    use super::*;
    use crate::Lookup;
    use crate::parse::fixtures;

    /// A one-shot HTTP server on loopback: answers the first request with
    /// `status` and `body`, and hands back what the request looked like.
    fn serve_once(status: &str, body: String) -> (String, mpsc::Receiver<String>) {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let base = format!("http://{}", listener.local_addr().unwrap());
        let (tx, rx) = mpsc::channel();
        let status = status.to_string();
        std::thread::spawn(move || {
            let (mut sock, _) = listener.accept().unwrap();
            let mut buf = [0u8; 4096];
            let n = sock.read(&mut buf).unwrap();
            let _ = tx.send(String::from_utf8_lossy(&buf[..n]).into_owned());
            let resp = format!(
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nContent-Type: application/json\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = sock.write_all(resp.as_bytes());
        });
        (base, rx)
    }

    fn adsb() -> Request {
        Request::Adsbdb { icao: 0x484006, callsign: Some("KLM1234".into()) }
    }

    #[test]
    fn the_request_names_the_path_and_carries_the_stations_user_agent() {
        let (base, seen) = serve_once("200 OK", fixtures::ADSBDB.to_string());
        let up = HttpUpstream::with_bases(&base, "http://127.0.0.1:1");
        let reply = up.fetch(&adsb());
        assert!(matches!(reply, Reply::Adsbdb(Lookup::Found(_))), "{reply:?}");
        let head = seen.recv().unwrap();
        assert!(head.starts_with("GET /v0/aircraft/484006?callsign=KLM1234 HTTP/1.1"), "{head}");
        assert!(head.to_ascii_lowercase().contains("user-agent: sdr-oxide-brown/"), "{head}");
        assert!(!head.to_ascii_lowercase().contains("cookie"), "nothing of the client goes out");
    }

    #[test]
    fn a_404_is_unknown_and_a_server_error_is_a_failure() {
        let (base, _) = serve_once("404 Not Found", "{}".into());
        let up = HttpUpstream::with_bases(&base, "http://127.0.0.1:1");
        assert_eq!(up.fetch(&adsb()), Reply::Adsbdb(Lookup::NotFound));
        let (base, _) = serve_once("503 Service Unavailable", "busy".into());
        let up = HttpUpstream::with_bases(&base, "http://127.0.0.1:1");
        assert_eq!(up.fetch(&adsb()), Reply::Adsbdb(Lookup::Failed));
    }

    #[test]
    fn a_photo_request_goes_to_the_photo_host() {
        let (base, seen) = serve_once("200 OK", fixtures::PHOTO.to_string());
        let up = HttpUpstream::with_bases("http://127.0.0.1:1", &base);
        let reply = up.fetch(&Request::Photo { registration: "PH-BXA".into() });
        assert!(matches!(reply, Reply::Photo(Lookup::Found(_))), "{reply:?}");
        assert!(seen.recv().unwrap().starts_with("GET /pub/photos/reg/PH-BXA HTTP/1.1"));
    }

    #[test]
    fn a_body_past_the_limit_is_a_failure_not_a_read() {
        let huge = format!("{{\"response\":\"{}\"}}", "x".repeat(300 * 1024));
        let (base, _) = serve_once("200 OK", huge);
        let up = HttpUpstream::with_bases(&base, "http://127.0.0.1:1");
        assert_eq!(up.fetch(&adsb()), Reply::Adsbdb(Lookup::Failed));
    }

    #[test]
    fn a_picture_comes_back_with_its_type_and_a_page_in_its_place_does_not() {
        let up = HttpUpstream::with_bases("http://127.0.0.1:1", "http://127.0.0.1:1");
        let serve = |ct: &str, body: &[u8]| {
            let listener = TcpListener::bind("127.0.0.1:0").unwrap();
            let url = format!("http://{}/a.jpg", listener.local_addr().unwrap());
            let (ct, body) = (ct.to_string(), body.to_vec());
            std::thread::spawn(move || {
                let (mut sock, _) = listener.accept().unwrap();
                let mut buf = [0u8; 4096];
                let _ = sock.read(&mut buf);
                let head = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: {ct}\r\nConnection: close\r\n\r\n",
                    body.len()
                );
                let _ = sock.write_all(head.as_bytes());
                let _ = sock.write_all(&body);
            });
            url
        };
        let got = up.image(&serve("image/jpeg", b"\xff\xd8jpeg")).unwrap();
        assert_eq!(
            (got.content_type.as_str(), got.bytes.as_slice()),
            ("image/jpeg", &b"\xff\xd8jpeg"[..])
        );
        assert!(up.image(&serve("text/html", b"<html>")).is_none());
        let big = vec![0u8; MAX_IMAGE + 1];
        assert!(up.image(&serve("image/png", &big)).is_none());
    }

    #[test]
    fn a_summary_is_asked_by_path_with_the_stations_user_agent_and_gives_the_thumbnail() {
        let body = r#"{"thumbnail":{"source":"https://upload.wikimedia.org/a/b/320px-x.jpg"}}"#;
        let (base, seen) = serve_once("200 OK", body.to_string());
        let up = HttpUpstream::with_bases("http://127.0.0.1:1", "http://127.0.0.1:1")
            .with_wikipedia(&base);
        let a = Article::new("en", "Amsterdam_Airport_Schiphol").unwrap();
        assert_eq!(
            up.wikipedia_thumb_url(&a).as_deref(),
            Some("https://upload.wikimedia.org/a/b/320px-x.jpg")
        );
        let head = seen.recv().unwrap();
        assert!(
            head.starts_with("GET /api/rest_v1/page/summary/Amsterdam_Airport_Schiphol HTTP/1.1"),
            "{head}"
        );
        assert!(head.to_ascii_lowercase().contains("user-agent: sdr-oxide-brown/"), "{head}");
        assert!(!head.to_ascii_lowercase().contains("cookie"), "nothing of the client goes out");
    }

    #[test]
    fn a_page_without_a_picture_or_with_a_picture_elsewhere_gives_no_thumbnail() {
        let a = Article::new("en", "Somewhere").unwrap();
        for (status, body) in [
            ("404 Not Found", "{}"),
            ("200 OK", r#"{"title":"Somewhere"}"#),
            ("200 OK", r#"{"thumbnail":{"source":"https://evil.example/a.jpg"}}"#),
            ("200 OK", "<html>"),
        ] {
            let (base, _) = serve_once(status, body.to_string());
            let up = HttpUpstream::with_bases("http://127.0.0.1:1", "http://127.0.0.1:1")
                .with_wikipedia(&base);
            assert_eq!(up.wikipedia_thumb_url(&a), None, "{status} {body}");
        }
    }

    #[test]
    fn the_weather_is_asked_of_the_forecast_api_by_place_alone() {
        let (base, seen) = serve_once("200 OK", crate::weather::FORECAST.to_string());
        let up = HttpUpstream::with_bases("http://127.0.0.1:1", "http://127.0.0.1:1")
            .with_open_meteo(&base);
        let w = up.weather(&Spot::new(52.308_613, 4.763_889).unwrap()).unwrap();
        assert_eq!(w.utc_offset_s, 3600);
        let head = seen.recv().unwrap();
        assert!(
            head.starts_with(
                "GET /v1/forecast?latitude=52.31&longitude=4.76&current=temperature_2m&timezone=auto HTTP/1.1"
            ),
            "{head}"
        );
        assert!(head.to_ascii_lowercase().contains("user-agent: sdr-oxide-brown/"), "{head}");
        assert!(!head.to_ascii_lowercase().contains("cookie"), "nothing of the client goes out");
    }

    #[test]
    fn a_refusal_or_a_page_that_is_not_a_forecast_gives_no_weather() {
        let spot = Spot::new(52.3, 4.76).unwrap();
        for (status, body) in [
            ("400 Bad Request", r#"{"error":true,"reason":"x"}"#),
            ("429 Too Many Requests", "{}"),
            ("200 OK", "<html>"),
            ("200 OK", "{}"),
        ] {
            let (base, _) = serve_once(status, body.to_string());
            let up = HttpUpstream::with_bases("http://127.0.0.1:1", "http://127.0.0.1:1")
                .with_open_meteo(&base);
            assert_eq!(up.weather(&spot), None, "{status} {body}");
        }
        let up = HttpUpstream::with_bases("http://127.0.0.1:1", "http://127.0.0.1:1")
            .with_open_meteo("http://127.0.0.1:1");
        assert_eq!(up.weather(&spot), None, "nobody listening");
    }

    #[test]
    fn nobody_listening_is_a_failure_and_not_a_panic() {
        // Port 1 on loopback: nothing is there.
        let up = HttpUpstream::with_bases("http://127.0.0.1:1", "http://127.0.0.1:1");
        assert_eq!(up.fetch(&adsb()), Reply::Adsbdb(Lookup::Failed));
    }
}
