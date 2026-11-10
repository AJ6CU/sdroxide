//! WSJT-CB spot server: post decoded 11 m spots to the CB community's spotting
//! service.
//!
//! Separate from PSK Reporter, which the WSJT-CB client also supports. This is
//! the project's own server (`xzgroup.net/spots/api/ingest.php`), and the
//! payload is the JSON its client posts: who is spotting, the station decoded,
//! the frequency, mode, SNR, timing and the raw message text. It is where 11 m
//! operators watch each other, and CB-shaped calls and grids are what they
//! expect to see in it.
//!
//! Opt-in and **off by default**, like WSJT-CB's own switch: it is a
//! third-party service, and nothing leaves the station until the operator asks.
//! Posting is one request per spot, as WSJT-CB does, on a worker thread so the
//! engine's poll loop never waits on a network.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender};

use crate::event::{EventTx, NetEvent};

/// How long the worker waits for the next spot before checking whether it has
/// been stood down. Only the idle path uses it — a post blocks the loop.
const TICK: Duration = Duration::from_millis(500);

/// Who is doing the spotting. The callsign is the server's `spotter_call` and
/// is required; the grid is sent only when the operator has one (many CB
/// operators do not).
#[derive(Clone, Debug)]
pub struct Station {
    pub call: String,
    pub grid: String,
}

/// One decoded spot, as the worker posts it.
#[derive(Clone, Debug, PartialEq)]
pub struct Spot {
    /// The station decoded. Never empty.
    pub dx_call: String,
    /// Its locator, when the decode carried one. CB exchanges usually do not.
    pub dx_grid: Option<String>,
    /// The signal's radio frequency, in Hz — the dial plus the audio offset.
    pub freq_hz: f64,
    /// The mode's label, e.g. `FT8`.
    pub mode: String,
    pub snr_db: i16,
    /// Time offset of the decode within the slot, seconds.
    pub dt: f32,
    /// Audio offset of the signal within the passband, Hz.
    pub df_hz: i32,
    /// The decoded message, verbatim.
    pub message: String,
    /// Slot start, Unix seconds UTC.
    pub when_utc: i64,
}

/// A running reporter.
///
/// Dropping it signals the worker and **returns immediately** — it does not
/// join. `SpotManager` rebuilds these from `set_config` and `set_operator`,
/// both of which run on the engine's thread, and blocking the receiver behind a
/// twenty-second HTTP timeout would stop the radio. Nothing borrows from the
/// handle, so the thread finishing on its own is safe as well as necessary.
pub struct WsjtCbHandle {
    tx: Sender<Spot>,
    stopping: Arc<AtomicBool>,
}

impl Drop for WsjtCbHandle {
    fn drop(&mut self) {
        self.stopping.store(true, Ordering::Relaxed);
    }
}

impl WsjtCbHandle {
    /// Queue a spot. A channel send and nothing else: this is called from the
    /// engine thread as a slot decodes, and must never wait on a network.
    pub fn report(&self, spot: Spot) {
        let _ = self.tx.send(spot);
    }
}

/// The request function, injected so the tests can drive the worker without a
/// network.
type Post = Arc<dyn Fn(&str, &str) -> Result<String, String> + Send + Sync>;

/// Start the reporter against the configured endpoint.
pub fn spawn(url: String, station: Station, events: EventTx) -> WsjtCbHandle {
    spawn_with(url, station, events, Arc::new(crate::http::post_json))
}

fn spawn_with(url: String, station: Station, events: EventTx, post: Post) -> WsjtCbHandle {
    let (tx, rx) = crossbeam_channel::unbounded::<Spot>();
    let stopping = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&stopping);
    std::thread::Builder::new()
        .name("sdroxide-wsjtcb".into())
        .spawn(move || run(url, station, rx, flag, events, post))
        .expect("spawn WSJT-CB spot worker");
    WsjtCbHandle { tx, stopping }
}

fn run(
    url: String,
    station: Station,
    spots: Receiver<Spot>,
    stopping: Arc<AtomicBool>,
    events: EventTx,
    post: Post,
) {
    let mut reported_error = false;
    loop {
        // A disconnect is the other half of the stop signal and wakes this at
        // once; the timeout is what lets a stop be noticed while idle.
        let spot = match spots.recv_timeout(TICK) {
            Ok(spot) => Some(spot),
            Err(RecvTimeoutError::Timeout) => None,
            Err(RecvTimeoutError::Disconnected) => break,
        };
        if stopping.load(Ordering::Relaxed) {
            break;
        }
        let Some(spot) = spot else { continue };
        let body = body_json(&spot, &station);
        match post(&url, &body) {
            Ok(_) => {
                // One line per outage: clear the error the moment one post
                // gets through, and say nothing otherwise — a line per spot on
                // a busy band would be noise.
                if reported_error {
                    reported_error = false;
                    let _ = events.send(NetEvent::Status(None));
                }
            }
            Err(e) => {
                if !reported_error {
                    reported_error = true;
                    let _ = events.send(NetEvent::Status(Some(format!("WSJT-CB: {e}"))));
                }
            }
        }
    }
}

/// The JSON body WSJT-CB's client posts: `{ "spot": { … } }`.
///
/// Optional fields are omitted rather than sent empty, exactly as the client
/// does, so a decode with no locator does not put an empty `dx_grid` in the
/// server's database.
fn body_json(spot: &Spot, station: &Station) -> String {
    let mut s = serde_json::Map::new();
    if !station.call.is_empty() {
        s.insert("spotter_call".into(), station.call.clone().into());
    }
    if !station.grid.is_empty() {
        s.insert("spotter_grid".into(), station.grid.clone().into());
    }
    s.insert("dx_call".into(), spot.dx_call.clone().into());
    if let Some(g) = spot.dx_grid.as_deref().filter(|g| !g.is_empty()) {
        s.insert("dx_grid".into(), g.into());
    }
    s.insert("frequency_hz".into(), spot.freq_hz.into());
    if !spot.mode.is_empty() {
        s.insert("mode".into(), spot.mode.clone().into());
    }
    s.insert("snr".into(), spot.snr_db.into());
    s.insert("dt".into(), f64::from(spot.dt).into());
    s.insert("df".into(), spot.df_hz.into());
    if !spot.message.is_empty() {
        s.insert("message_text".into(), spot.message.clone().into());
    }
    s.insert("timestamp_utc".into(), iso_utc(spot.when_utc).into());
    s.insert("source".into(), "sdroxide".into());
    s.insert("client_version".into(), env!("CARGO_PKG_VERSION").into());
    serde_json::json!({ "spot": serde_json::Value::Object(s) }).to_string()
}

/// A Unix time as the ISO-8601 UTC the server expects
/// (`2026-09-17T09:00:00Z`), not the client's local time.
fn iso_utc(unix: i64) -> String {
    let (y, mo, d, h, mi, s) = sdroxide_types::utc_ymd_hms(unix);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{mi:02}:{s:02}Z")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    fn station() -> Station {
        Station { call: "26AT715".into(), grid: "JO21".into() }
    }

    fn spot() -> Spot {
        Spot {
            dx_call: "1AT106".into(),
            dx_grid: None,
            freq_hz: 27_265_000.0,
            mode: "FT8".into(),
            snr_db: -12,
            dt: 0.1,
            df_hz: 1500,
            message: "CQ 1AT106".into(),
            when_utc: 1_787_000_000,
        }
    }

    /// The payload has to be the one WSJT-CB's server is fed, field for field:
    /// a decode with no locator carries no `dx_grid` key at all rather than an
    /// empty one.
    #[test]
    fn the_payload_matches_wsjt_cb() {
        let v: serde_json::Value = serde_json::from_str(&body_json(&spot(), &station())).unwrap();
        let s = &v["spot"];
        assert_eq!(s["spotter_call"], "26AT715");
        assert_eq!(s["spotter_grid"], "JO21");
        assert_eq!(s["dx_call"], "1AT106");
        assert!(s.get("dx_grid").is_none(), "no locator, no key: {s}");
        assert_eq!(s["frequency_hz"], 27_265_000.0);
        assert_eq!(s["mode"], "FT8");
        assert_eq!(s["snr"], -12);
        assert_eq!(s["df"], 1500);
        assert_eq!(s["message_text"], "CQ 1AT106");
        assert_eq!(s["source"], "sdroxide");
        assert!(s["client_version"].as_str().is_some_and(|v| !v.is_empty()));
        // UTC, and readable as such.
        assert!(s["timestamp_utc"].as_str().unwrap().ends_with('Z'));
    }

    /// A grid, when there is one, is carried through.
    #[test]
    fn a_locator_is_reported_when_present() {
        let mut sp = spot();
        sp.dx_grid = Some("JO21AB".into());
        let v: serde_json::Value = serde_json::from_str(&body_json(&sp, &station())).unwrap();
        assert_eq!(v["spot"]["dx_grid"], "JO21AB");
    }

    /// Posts what it is given, and reports an outage once rather than once per
    /// spot.
    #[test]
    fn posts_and_reports_one_error_per_outage() {
        let (events_tx, events_rx) = crossbeam_channel::unbounded();
        let seen: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let log = Arc::clone(&seen);
        let fail = Arc::new(AtomicBool::new(true));
        let flag = Arc::clone(&fail);
        let post: Post = Arc::new(move |_url: &str, body: &str| {
            log.lock().unwrap().push(body.to_string());
            if flag.load(Ordering::Relaxed) { Err("boom".into()) } else { Ok(String::new()) }
        });
        let h = spawn_with("http://example.invalid/ingest".into(), station(), events_tx, post);
        h.report(spot());
        h.report(spot());
        // Two posts, one error line despite the second failing the same way.
        let first = events_rx.recv_timeout(Duration::from_secs(2)).unwrap();
        assert!(matches!(first, NetEvent::Status(Some(_))));
        assert!(events_rx.recv_timeout(Duration::from_millis(200)).is_err());
        assert_eq!(seen.lock().unwrap().len(), 2);
        // Recovery clears the line.
        fail.store(false, Ordering::Relaxed);
        h.report(spot());
        assert!(matches!(
            events_rx.recv_timeout(Duration::from_secs(2)),
            Ok(NetEvent::Status(None))
        ));
    }
}
