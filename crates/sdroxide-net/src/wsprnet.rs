//! WSPRnet: uploading what we decoded, and downloading who decoded us.
//!
//! Two halves of one conversation with <https://wsprnet.org>, and the reason
//! the mode exists at all — a WSPR receiver that keeps its spots to itself has
//! measured one end of a path and told nobody.
//!
//! **Upload** is one HTTP GET per spot against `/post`, the interface every
//! `wsprd`-based client uses. There is no batch form and no authentication: the
//! callsign in the query *is* the identity. When a slot decoded nothing there
//! is still something to say — `function=wsprstat` reports that this station
//! was listening and heard silence, which is how the network can tell a band
//! that was shut from a receiver that was off.
//!
//! **Download** is `/drupal/wsprnet/spots/json`, filtered by callsign. Two
//! questions are worth asking it: who heard *us* (`callsign=`), which is the
//! only way a transmit-only beacon learns anything at all, and what we
//! ourselves reported (`reporter=`), which reconciles our uploads against what
//! the network actually stored.
//!
//! Both run on their own threads over the blocking [`crate::http`] helpers, in
//! the shape [`crate::pskupload`] and [`crate::poll`] already use.

use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender};
use sdroxide_types::WsprSpot;

use crate::event::{EventTx, NetEvent};

/// Where spots are posted.
const POST_URL: &str = "https://wsprnet.org/post";
/// Where they are read back.
const SPOTS_URL: &str = "https://www.wsprnet.org/drupal/wsprnet/spots/json";

/// The `version=` every request carries, so WSPRnet can see which client
/// produced a spot. Other clients send `0.1_wsprd`, `1.0_multi_wspr` and so on;
/// the convention is `<version>_<program>`.
fn client_version() -> String {
    format!("{}_sdroxide", env!("CARGO_PKG_VERSION"))
}

/// Never queue more than this. A network outage lasting hours must not turn
/// into unbounded memory, and stale spots are worth less than fresh ones
/// anyway.
const MAX_PENDING: usize = 512;

/// How quiet the queue must go before it is flushed.
///
/// A slot's decodes arrive together, in one burst, once every two minutes — so
/// waiting for the burst to finish uploads a whole slot in one round of
/// requests, and does it seconds after the slot rather than at the next tick of
/// some unrelated timer. A fixed interval instead had a slot's spots sitting
/// unsent for up to a minute for no benefit to anybody.
const QUIET: Duration = Duration::from_secs(2);

// ── Upload ──────────────────────────────────────────────────────────────────

/// Who is doing the hearing.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Reporter {
    pub call: String,
    pub grid: String,
}

/// One thing to tell WSPRnet: a spot, or the fact that a slot was quiet.
#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Spot(WsprSpot),
    /// A slot on `dial_hz` that decoded nothing, with the transmit duty cycle
    /// this station is running.
    Quiet {
        dial_hz: f64,
        tx_percent: u8,
    },
}

/// Build the query for one spot.
///
/// Frequencies go in **megahertz to six places**, the date as `YYMMDD` and the
/// time as `HHMM`, which is what every other client sends and what the server
/// parses. `mode=2` is WSPR-2, the two-minute variant; WSPR-15 would be 15.
///
/// Split out from the request so it can be tested without a network: WSPRnet
/// answers a malformed spot with "0 spot(s) added" and no explanation, so the
/// query is the only place the format can actually be checked.
pub fn spot_query(rx: &Reporter, s: &WsprSpot, version: &str) -> Vec<(String, String)> {
    let (y, mo, d, h, mi) = utc_parts(s.slot_utc);
    vec![
        ("function".into(), "wspr".into()),
        ("rcall".into(), rx.call.trim().to_uppercase()),
        ("rgrid".into(), rx.grid.trim().to_uppercase()),
        ("rqrg".into(), format!("{:.6}", s.freq_hz / 1e6)),
        ("date".into(), format!("{y:02}{mo:02}{d:02}")),
        ("time".into(), format!("{h:02}{mi:02}")),
        ("sig".into(), format!("{}", s.snr_db)),
        ("dt".into(), format!("{:.1}", s.dt)),
        ("drift".into(), format!("{:.0}", s.drift_hz)),
        ("tqrg".into(), format!("{:.6}", s.freq_hz / 1e6)),
        ("tcall".into(), s.call.trim().to_uppercase()),
        ("tgrid".into(), s.grid.clone().unwrap_or_default().trim().to_uppercase()),
        ("dbm".into(), format!("{}", s.power_dbm)),
        ("version".into(), version.into()),
        ("mode".into(), "2".into()),
    ]
}

/// Build the query for "I listened here and heard nothing".
pub fn quiet_query(
    rx: &Reporter,
    dial_hz: f64,
    tx_percent: u8,
    version: &str,
) -> Vec<(String, String)> {
    let mhz = format!("{:.6}", dial_hz / 1e6);
    vec![
        ("function".into(), "wsprstat".into()),
        ("rcall".into(), rx.call.trim().to_uppercase()),
        ("rgrid".into(), rx.grid.trim().to_uppercase()),
        ("rqrg".into(), mhz.clone()),
        ("tpct".into(), format!("{:.2}", tx_percent as f32)),
        ("tqrg".into(), mhz),
        ("dbm".into(), "0".into()),
        ("version".into(), version.into()),
        ("mode".into(), "2".into()),
    ]
}

/// `base?k=v&…`, percent-encoding each value.
pub fn build_url(base: &str, params: &[(String, String)]) -> String {
    let mut url = String::from(base);
    for (i, (k, v)) in params.iter().enumerate() {
        url.push(if i == 0 { '?' } else { '&' });
        url.push_str(k);
        url.push('=');
        url.push_str(&percent_encode(v));
    }
    url
}

/// Percent-encode everything that is not unreserved. Callsigns carry `/`, grids
/// do not, and a stray `&` in either would silently truncate the spot.
fn percent_encode(s: &str) -> String {
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

/// Split unix seconds into the UTC pieces WSPRnet's date and time fields want.
///
/// Two-digit year, because that is the field's width. Written out rather than
/// pulled from a date library: this crate has none, the epoch-to-civil
/// conversion is a dozen lines, and it is exercised by its own test.
fn utc_parts(unix: i64) -> (i64, i64, i64, i64, i64) {
    let days = unix.div_euclid(86_400);
    let secs = unix.rem_euclid(86_400);
    // Howard Hinnant's civil_from_days.
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y.rem_euclid(100), m, d, secs / 3600, (secs % 3600) / 60)
}

/// How long a worker that has been stood down may keep uploading before it
/// gives up and drops what is left.
///
/// A reconfigure should not silently bin a slot's spots, so a stopped worker
/// gets a moment to finish — but only a moment. Anything still queued after
/// this was going to be late anyway, and a thread that outlives its handle by
/// minutes is a thread nobody is watching.
const FAREWELL: Duration = Duration::from_secs(10);

/// Longest a worker sleeps before noticing it has been stood down.
const TICK: Duration = Duration::from_millis(200);

/// What performs a request. Real code passes [`crate::http::get`]; the tests
/// pass something slow and counted, which is the only way to *check* that
/// stopping a busy worker is prompt rather than to assert it in a comment.
type Sender6 = Arc<dyn Fn(&str) -> Result<String, String> + Send + Sync>;

/// A running uploader.
///
/// Dropping it signals the worker and **returns immediately**. It deliberately
/// does not join: `SpotManager` rebuilds these handles from
/// [`crate::SpotManager::set_config`] and `set_operator`, both of which run on
/// the engine's own thread — the one carrying the receiver. Joining a worker
/// that happens to be inside a twenty-second HTTP timeout, with a slot's worth
/// of spots still queued behind it, would stop the radio for as long as
/// wsprnet.org took to answer. Nothing borrows from the handle, so letting the
/// thread finish on its own is safe as well as necessary.
pub struct WsprUploadHandle {
    tx: Sender<Item>,
    stopping: Arc<AtomicBool>,
}

impl Drop for WsprUploadHandle {
    fn drop(&mut self) {
        self.stopping.store(true, Ordering::Relaxed);
    }
}

impl WsprUploadHandle {
    /// Queue something to tell WSPRnet.
    ///
    /// A channel send and nothing else: this is called from the engine thread
    /// as each slot finishes decoding, and it must never wait on a network.
    pub fn report(&self, item: Item) {
        let _ = self.tx.send(item);
    }
}

/// Start the uploader.
pub fn spawn_upload(rx: Reporter, events: EventTx) -> WsprUploadHandle {
    spawn_upload_with(rx, events, Arc::new(|url: &str| crate::http::get(url)), FAREWELL)
}

/// [`spawn_upload`] with the request function and the farewell budget injected,
/// so the tests can drive it in milliseconds instead of tens of seconds.
fn spawn_upload_with(
    rx: Reporter,
    events: EventTx,
    send: Sender6,
    farewell: Duration,
) -> WsprUploadHandle {
    let (tx, item_rx) = crossbeam_channel::unbounded::<Item>();
    let stopping = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&stopping);
    std::thread::Builder::new()
        .name("sdroxide-wsprnet".into())
        .spawn(move || run_upload(rx, item_rx, flag, events, send, farewell))
        .expect("spawn wsprnet upload worker");
    WsprUploadHandle { tx, stopping }
}

fn run_upload(
    rx: Reporter,
    items: Receiver<Item>,
    stopping: Arc<AtomicBool>,
    events: EventTx,
    send: Sender6,
    farewell: Duration,
) {
    let version = client_version();
    let mut pending: VecDeque<Item> = VecDeque::new();
    // Far enough in the past that the first burst to arrive goes at once.
    let mut last_item = Instant::now() - QUIET;
    let mut reported_error = false;
    loop {
        // The handle's sender is dropped with the handle, so a disconnect is
        // the other half of the stop signal and wakes this immediately.
        let disconnected = match items.recv_timeout(TICK) {
            Ok(item) => {
                // Full: shed the *oldest*. An outage long enough to fill this
                // has already made the head of the queue stale, and a fresh
                // spot is worth more to the network than a half-hour-old one.
                if pending.len() >= MAX_PENDING {
                    pending.pop_front();
                }
                pending.push_back(item);
                last_item = Instant::now();
                false
            }
            Err(RecvTimeoutError::Timeout) => false,
            Err(RecvTimeoutError::Disconnected) => true,
        };
        let stop = disconnected || stopping.load(Ordering::Relaxed);
        if (last_item.elapsed() >= QUIET || stop) && !pending.is_empty() {
            // A stopped worker is on borrowed time; a running one may take as
            // long as the requests take.
            let deadline = stop.then(|| Instant::now() + farewell);
            let (sent, err) = flush(&rx, &version, &mut pending, deadline, &stopping, &send);
            match err {
                None => {
                    reported_error = false;
                    if sent > 0 {
                        let _ = events.send(NetEvent::Status(Some(format!(
                            "WSPRnet: {sent} spot{} uploaded",
                            if sent == 1 { "" } else { "s" }
                        ))));
                    }
                }
                // One status line per outage, not one per flush.
                Some(e) => {
                    if !reported_error {
                        reported_error = true;
                        let _ = events.send(NetEvent::Status(Some(format!("WSPRnet: {e}"))));
                    }
                }
            }
        }
        if stop {
            break;
        }
    }
}

/// Post what is queued, one request per item, oldest first.
///
/// Drains `pending` as it goes, so anything it does not reach stays queued for
/// the next pass instead of being thrown away — including the item a failure
/// stopped on, which is how a spot survives an outage rather than being lost to
/// one.
///
/// The stop flag and the deadline are checked **between** requests, so standing
/// a worker down while it is halfway through forty spots costs one request's
/// timeout rather than forty. That is the difference between a reconfigure that
/// feels instant and one that appears to hang the radio.
fn flush(
    rx: &Reporter,
    version: &str,
    pending: &mut VecDeque<Item>,
    deadline: Option<Instant>,
    stopping: &AtomicBool,
    send: &Sender6,
) -> (usize, Option<String>) {
    let mut sent = 0usize;
    while let Some(item) = pending.front() {
        // Not checked before the first request: a worker stopped with one spot
        // queued should still manage to send it.
        if sent > 0 {
            let out_of_time = deadline.is_some_and(|d| Instant::now() >= d);
            // Newly stopped mid-batch: come back round and finish under the
            // farewell budget rather than running the whole queue out here.
            let newly_stopped = deadline.is_none() && stopping.load(Ordering::Relaxed);
            if out_of_time || newly_stopped {
                break;
            }
        }
        let params = match item {
            Item::Spot(s) => spot_query(rx, s, version),
            Item::Quiet { dial_hz, tx_percent } => quiet_query(rx, *dial_hz, *tx_percent, version),
        };
        match send(&build_url(POST_URL, &params)) {
            Ok(_) => {
                pending.pop_front();
                sent += 1;
            }
            // Left at the front to be retried: the usual failure here is the
            // network being away, and it will be back.
            Err(e) => return (sent, Some(e)),
        }
    }
    (sent, None)
}

// ── Download ────────────────────────────────────────────────────────────────

/// Which side of the network to ask about.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Query {
    /// Spots *of* this callsign: who heard us. The only thing a transmitting
    /// beacon can learn about its own reach.
    HeardUs,
    /// Spots *by* this callsign: what we reported, as the network stored it.
    WeReported,
}

/// Fetch recent spots for `call`.
///
/// `minutes` is the lookback window the server applies; it defaults to four and
/// is capped server-side, so asking for a day does not get one.
pub fn fetch(call: &str, side: Query, minutes: u32) -> Result<Vec<WsprSpot>, String> {
    let call = call.trim().to_uppercase();
    if call.is_empty() {
        return Ok(Vec::new());
    }
    let key = match side {
        Query::HeardUs => "callsign",
        Query::WeReported => "reporter",
    };
    let params = vec![
        (key.to_string(), call),
        ("minutes".to_string(), minutes.max(1).to_string()),
        // Balloon telemetry uses the callsign field for encoded payload data.
        // It is not a station and it is not a path anybody can work.
        ("exclude_special".to_string(), "1".to_string()),
    ];
    let body = crate::http::get(&build_url(SPOTS_URL, &params))?;
    parse_spots(&body, side)
}

/// Turn a download error into something calm on the status line.
///
/// WSPRnet has been answering 403 to whole client classes at the server (its
/// Drupal front-end presses back on non-browser traffic in spurts), which is
/// neither an app fault nor something a retry fixes — the line should say that
/// and stop there, not dump a chunk of HTML.
fn friendly_download_error(e: &str) -> String {
    if e.starts_with("HTTP 403") {
        "refusing requests (HTTP 403) — a server-side block, clears on its own".to_string()
    } else {
        e.to_string()
    }
}

/// Parse the `spots/json` payload.
///
/// The endpoint answers with an array of objects whose keys are capitalised and
/// whose numbers are strings — `"dB": "-24"`, `"MHz": "14.097090"`. Everything
/// is read defensively: a field that changed shape should cost one spot, not
/// the whole poll.
pub fn parse_spots(body: &str, side: Query) -> Result<Vec<WsprSpot>, String> {
    let v: serde_json::Value =
        serde_json::from_str(body).map_err(|e| format!("bad JSON from wsprnet: {e}"))?;
    let arr = v.as_array().ok_or_else(|| "wsprnet did not answer with a list".to_string())?;
    let mut out = Vec::with_capacity(arr.len());
    for o in arr {
        let Some(call) = str_field(o, "CallSign") else { continue };
        let Some(mhz) = num_field(o, "MHz") else { continue };
        let Some(when) = num_field(o, "Date") else { continue };
        let grid = str_field(o, "Grid").filter(|g| !g.is_empty());
        let reporter = str_field(o, "Reporter").filter(|r| !r.is_empty());
        let reporter_grid = str_field(o, "ReporterGrid").filter(|g| !g.is_empty());
        out.push(WsprSpot {
            // WSPR slots start on even minutes; the server stamps the slot, but
            // rounding is cheap insurance against one that does not.
            slot_utc: (when as i64).div_euclid(120) * 120,
            call: call.to_uppercase(),
            grid,
            power_dbm: num_field(o, "Power").unwrap_or(0.0) as i16,
            freq_hz: mhz * 1e6,
            snr_db: num_field(o, "dB").unwrap_or(0.0) as i16,
            // The endpoint does not carry `dt`; it is a receiver-side quantity
            // and the network keeps only what it needs. Zero rather than a
            // guess, and nothing downstream reads it for a remote spot.
            dt: 0.0,
            drift_hz: num_field(o, "Drift").unwrap_or(0.0) as f32,
            reporter,
            reporter_grid,
        });
    }
    // `WeReported` spots are our own uploads coming back; they carry a reporter
    // (us) but describe a station we heard, which is what a local decode
    // already said. Kept anyway — reconciliation is the point of asking.
    let _ = side;
    Ok(out)
}

/// A running download poller.
///
/// Dropping it signals the thread and returns at once, for the reason
/// [`WsprUploadHandle`] does: these are rebuilt from the engine thread, and a
/// poller that happens to be inside a twenty-second fetch must not take the
/// receiver down with it. The thread notices on its next wake and exits.
pub struct WsprPollHandle {
    stopping: Arc<AtomicBool>,
    /// Dropped with the handle, which wakes the sleeping thread immediately
    /// rather than leaving it to time out its interval first.
    _stop: Sender<()>,
}

impl Drop for WsprPollHandle {
    fn drop(&mut self) {
        self.stopping.store(true, Ordering::Relaxed);
    }
}

/// Poll WSPRnet for `call` on `interval`, forwarding new spots as
/// [`NetEvent::WsprSpots`].
///
/// Its own poller rather than [`crate::poll::spawn`] because that one deals in
/// [`sdroxide_types::Spot`], and a WSPR report is not one: it carries a power
/// level, a drift and a reporter, and flattening it into a cluster-style spot
/// would throw away exactly the fields that make it a measurement.
///
/// Spots already seen are filtered here rather than downstream — the endpoint
/// answers with a whole window every time, so without this every poll would
/// re-report the same reception.
pub fn spawn_download(
    call: String,
    side: Query,
    interval: Duration,
    window_min: u32,
    events: EventTx,
) -> WsprPollHandle {
    let (stop_tx, stop_rx) = crossbeam_channel::bounded::<()>(1);
    let interval = interval.max(Duration::from_secs(60));
    let stopping = Arc::new(AtomicBool::new(false));
    let flag = Arc::clone(&stopping);
    std::thread::Builder::new()
        .name("sdroxide-wsprnet-poll".into())
        .spawn(move || {
            let mut seen: std::collections::HashSet<(i64, String, i64, Option<String>)> =
                Default::default();
            let mut last_err: Option<String> = None;
            let mut last_err_at = Instant::now() - Duration::from_secs(600);
            loop {
                // Checked before the request rather than only after the sleep,
                // so a handle dropped while this was sleeping does not go on to
                // make one more pointless twenty-second fetch.
                if flag.load(Ordering::Relaxed) {
                    return;
                }
                match fetch(&call, side, window_min) {
                    Ok(spots) => {
                        if last_err.take().is_some() {
                            let _ = events.send(NetEvent::Status(None));
                        }
                        let fresh: Vec<WsprSpot> =
                            spots.into_iter().filter(|s| seen.insert(s.dedup_key())).collect();
                        // The window is bounded, so the seen-set is too — but
                        // only once it has been pruned. Cheapest correct prune:
                        // drop everything older than twice the window.
                        let cutoff = fresh.iter().map(|s| s.slot_utc).max().unwrap_or(0)
                            - 2 * window_min as i64 * 60;
                        if cutoff > 0 {
                            seen.retain(|(slot, _, _, _)| *slot >= cutoff);
                        }
                        if !fresh.is_empty() {
                            let _ = events.send(NetEvent::WsprSpots(fresh));
                        }
                    }
                    Err(e) => {
                        let e = friendly_download_error(&e);
                        // Re-announce the same stalemate on a slow beat, so a
                        // server that has been pressing every request for hours
                        // neither floods the status line nor looks like it has
                        // gone quiet.
                        let first = last_err.as_deref() != Some(e.as_str());
                        let due = last_err_at.elapsed() >= Duration::from_secs(600);
                        if first || due {
                            let _ = events.send(NetEvent::Status(Some(format!("WSPRnet: {e}"))));
                            last_err = Some(e);
                            last_err_at = Instant::now();
                        }
                    }
                }
                match stop_rx.recv_timeout(interval) {
                    Ok(()) | Err(crossbeam_channel::RecvTimeoutError::Disconnected) => return,
                    Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
                }
            }
        })
        .expect("spawn wsprnet download poller");
    WsprPollHandle { stopping, _stop: stop_tx }
}

fn str_field(o: &serde_json::Value, key: &str) -> Option<String> {
    o.get(key).and_then(|v| v.as_str()).map(|s| s.trim().to_string())
}

/// A number that may have arrived as a JSON number or as a string.
fn num_field(o: &serde_json::Value, key: &str) -> Option<f64> {
    match o.get(key)? {
        serde_json::Value::Number(n) => n.as_f64(),
        serde_json::Value::String(s) => s.trim().parse().ok(),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn spot() -> WsprSpot {
        WsprSpot {
            // 2026-08-03 12:34:00 UTC, which is a slot boundary.
            slot_utc: 1_785_760_440,
            call: "K1ABC".into(),
            grid: Some("FN42".into()),
            power_dbm: 37,
            freq_hz: 14_097_090.0,
            snr_db: -24,
            dt: 0.34,
            drift_hz: -1.0,
            reporter: None,
            reporter_grid: None,
        }
    }

    fn rx() -> Reporter {
        Reporter { call: "g0xyz".into(), grid: "io91np".into() }
    }

    /// The download error gets a quiet, accurate rendering when the server is
    /// blocking us, and everything else passes through untouched.
    #[test]
    fn a_server_side_block_is_described_gently_but_other_errors_pass_through() {
        assert_eq!(
            friendly_download_error("HTTP 403: <html>… whatever the drupal page says"),
            "refusing requests (HTTP 403) — a server-side block, clears on its own",
        );
        assert_eq!(
            friendly_download_error("HTTP 502: Bad Gateway"),
            "HTTP 502: Bad Gateway",
            "a transient gateway error is not a block and keeps its real wording",
        );
        assert_eq!(
            friendly_download_error("connection refused"),
            "connection refused",
        );
    }

    /// The upload format, checked here because the server will not check it for
    /// us: WSPRnet answers a malformed spot with "0 spot(s) added" and no
    /// explanation at all.
    #[test]
    fn a_spot_is_posted_in_the_form_every_other_client_sends() {
        let url = build_url(POST_URL, &spot_query(&rx(), &spot(), "0.9.0_sdroxide"));
        for want in [
            "function=wspr",
            "rcall=G0XYZ",
            "rgrid=IO91NP",
            "rqrg=14.097090",
            "tqrg=14.097090",
            "tcall=K1ABC",
            "tgrid=FN42",
            "dbm=37",
            "sig=-24",
            "dt=0.3",
            "drift=-1",
            "mode=2",
            "version=0.9.0_sdroxide",
        ] {
            assert!(url.contains(want), "{want} missing from {url}");
        }
        // Frequencies in megahertz, not hertz — the commonest way to get this
        // wrong, and it fails silently.
        assert!(!url.contains("rqrg=14097090"), "frequency went out in Hz: {url}");
    }

    #[test]
    fn the_date_and_time_are_the_utc_slot_the_beacon_transmitted_in() {
        let url = build_url(POST_URL, &spot_query(&rx(), &spot(), "v"));
        assert!(url.contains("date=260803"), "{url}");
        assert!(url.contains("time=1234"), "{url}");
    }

    #[test]
    fn epoch_zero_and_a_leap_day_both_convert() {
        assert_eq!(utc_parts(0), (70, 1, 1, 0, 0));
        // 2024-02-29 23:59:00 UTC
        assert_eq!(utc_parts(1_709_251_140), (24, 2, 29, 23, 59));
    }

    /// A callsign with a slash must not truncate the query at the next field.
    #[test]
    fn a_compound_callsign_is_escaped_rather_than_splitting_the_query() {
        let s = WsprSpot { call: "PJ4/K1ABC".into(), grid: None, ..spot() };
        let url = build_url(POST_URL, &spot_query(&rx(), &s, "v"));
        assert!(url.contains("tcall=PJ4%2FK1ABC"), "{url}");
        assert!(url.contains("tgrid=&"), "an absent grid must still be sent empty: {url}");
    }

    #[test]
    fn a_quiet_slot_still_says_this_station_was_listening() {
        let url = build_url(POST_URL, &quiet_query(&rx(), 14_095_600.0, 20, "v"));
        assert!(url.contains("function=wsprstat"), "{url}");
        assert!(url.contains("rqrg=14.095600"), "{url}");
        assert!(url.contains("tpct=20.00"), "{url}");
    }

    #[test]
    fn the_download_reads_the_numbers_wsprnet_sends_as_strings() {
        let body = r#"[
          {"Spotnum":"1234","Date":"1785242040","Reporter":"G0XYZ",
           "ReporterGrid":"IO91np","dB":"-24","MHz":"14.097090","CallSign":"K1ABC",
           "Grid":"FN42","Power":"37","Drift":"-1","distance":"5400","azimuth":"51",
           "Band":"14","version":"","code":"0"}
        ]"#;
        let got = parse_spots(body, Query::HeardUs).expect("parses");
        assert_eq!(got.len(), 1);
        let s = &got[0];
        assert_eq!(s.call, "K1ABC");
        assert_eq!(s.grid.as_deref(), Some("FN42"));
        assert_eq!(s.snr_db, -24);
        assert_eq!(s.power_dbm, 37);
        assert_eq!(s.drift_hz, -1.0);
        assert!((s.freq_hz - 14_097_090.0).abs() < 0.5, "{}", s.freq_hz);
        assert_eq!(s.reporter.as_deref(), Some("G0XYZ"));
        assert_eq!(s.reporter_grid.as_deref(), Some("IO91np"));
        // The slot, not the second within it.
        assert_eq!(s.slot_utc % 120, 0);
    }

    /// A field that changed shape should cost one spot, not the whole poll.
    #[test]
    fn a_malformed_entry_is_skipped_rather_than_failing_the_poll() {
        let body = r#"[
          {"CallSign":"K1ABC","MHz":"14.097090","Date":"1785242040","dB":"-24"},
          {"MHz":"14.097090","Date":"1785242040"},
          {"CallSign":"W1XYZ","Date":"1785242040"},
          {"CallSign":"G0ABC","MHz":14.09709,"Date":1785242040,"dB":-11}
        ]"#;
        let got = parse_spots(body, Query::HeardUs).expect("parses");
        assert_eq!(got.len(), 2, "{got:?}");
        assert_eq!(got[1].call, "G0ABC");
        assert_eq!(got[1].snr_db, -11);
    }

    // ── the worker must never hold up the engine ───────────────────────────

    /// A request function that takes `delay` and counts its calls, standing in
    /// for wsprnet.org on a bad day.
    fn slow_sender(delay: Duration) -> (Sender6, Arc<std::sync::atomic::AtomicUsize>) {
        let calls = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let c = Arc::clone(&calls);
        let f: Sender6 = Arc::new(move |_url: &str| {
            c.fetch_add(1, Ordering::Relaxed);
            std::thread::sleep(delay);
            Ok(String::new())
        });
        (f, calls)
    }

    fn queue(h: &WsprUploadHandle, n: usize) {
        for i in 0..n {
            h.report(Item::Spot(WsprSpot { slot_utc: 1_785_760_440 + i as i64 * 120, ..spot() }));
        }
    }

    /// The one that matters: `SpotManager` rebuilds these handles from the
    /// engine's own thread — the thread carrying the receiver — so dropping one
    /// while it is mid-upload must return at once. Joining instead would stop
    /// the radio for as long as wsprnet.org took to answer, times however many
    /// spots the slot decoded.
    #[test]
    fn dropping_a_busy_uploader_does_not_wait_for_it() {
        let (events, _rx) = crossbeam_channel::unbounded();
        // 150 ms a request × 40 spots is six seconds of work if it were joined.
        let (send, calls) = slow_sender(Duration::from_millis(150));
        let h = spawn_upload_with(
            Reporter { call: "K1ABC".into(), grid: "FN42".into() },
            events,
            send,
            Duration::from_millis(300),
        );
        queue(&h, 40);
        // Wait until it is genuinely inside a request rather than idling.
        let start = Instant::now();
        while calls.load(Ordering::Relaxed) == 0 {
            assert!(start.elapsed() < Duration::from_secs(10), "the worker never started");
            std::thread::sleep(Duration::from_millis(5));
        }

        let t = Instant::now();
        drop(h);
        let took = t.elapsed();
        assert!(
            took < Duration::from_millis(50),
            "dropping a busy uploader blocked for {took:?} — that would be the engine thread"
        );
    }

    /// And it does stop: detaching must not leak a thread that uploads for ever.
    /// The farewell budget is what bounds it.
    #[test]
    fn a_stopped_uploader_finishes_shortly_and_stops() {
        let (events, _rx) = crossbeam_channel::unbounded();
        let farewell = Duration::from_millis(300);
        let (send, calls) = slow_sender(Duration::from_millis(30));
        let h = spawn_upload_with(
            Reporter { call: "K1ABC".into(), grid: "FN42".into() },
            events,
            send,
            farewell,
        );
        queue(&h, 400);
        while calls.load(Ordering::Relaxed) == 0 {
            std::thread::sleep(Duration::from_millis(5));
        }
        drop(h);
        // Past the farewell budget with margin, then check it has gone quiet
        // and did *not* drain the whole queue.
        std::thread::sleep(farewell + Duration::from_millis(600));
        let a = calls.load(Ordering::Relaxed);
        std::thread::sleep(Duration::from_millis(400));
        let b = calls.load(Ordering::Relaxed);
        assert_eq!(a, b, "a stopped uploader was still working {a} → {b}");
        assert!(b < 400, "it drained the whole queue rather than giving up ({b})");
    }

    /// A slot's spots arrive in one burst and should go out in one round of
    /// requests, seconds later — not sit waiting on an unrelated timer.
    #[test]
    fn a_slots_worth_of_spots_goes_out_together_and_soon() {
        let (events, _rx) = crossbeam_channel::unbounded();
        let (send, calls) = slow_sender(Duration::from_millis(0));
        let h = spawn_upload_with(
            Reporter { call: "K1ABC".into(), grid: "FN42".into() },
            events,
            send,
            Duration::from_millis(300),
        );
        queue(&h, 12);
        let start = Instant::now();
        while calls.load(Ordering::Relaxed) < 12 {
            assert!(
                start.elapsed() < QUIET * 3,
                "only {} of 12 spots went in {:?}",
                calls.load(Ordering::Relaxed),
                start.elapsed()
            );
            std::thread::sleep(Duration::from_millis(20));
        }
        drop(h);
    }

    /// A flush that is cut short must leave what it did not send in the queue.
    /// Losing a slot's spots to a reconfigure would be the quiet kind of bug.
    #[test]
    fn an_interrupted_flush_keeps_what_it_did_not_send() {
        let rx = Reporter { call: "K1ABC".into(), grid: "FN42".into() };
        let mut pending: VecDeque<Item> = (0..5)
            .map(|i| Item::Spot(WsprSpot { slot_utc: 1_785_760_440 + i * 120, ..spot() }))
            .collect();
        let stopping = AtomicBool::new(false);
        // A deadline already in the past stops it after the first request.
        let (send, calls) = slow_sender(Duration::from_millis(0));
        let (sent, err) = flush(
            &rx,
            "v",
            &mut pending,
            Some(Instant::now() - Duration::from_secs(1)),
            &stopping,
            &send,
        );
        assert_eq!(sent, 1, "the first spot should always go");
        assert_eq!(calls.load(Ordering::Relaxed), 1);
        assert!(err.is_none());
        assert_eq!(pending.len(), 4, "the rest were thrown away instead of kept");
    }

    /// A failure leaves the spot at the head of the queue, so an outage costs
    /// nothing once the network is back.
    #[test]
    fn a_failed_upload_is_retried_rather_than_dropped() {
        let rx = Reporter { call: "K1ABC".into(), grid: "FN42".into() };
        let mut pending: VecDeque<Item> = (0..3)
            .map(|i| Item::Spot(WsprSpot { slot_utc: 1_785_760_440 + i * 120, ..spot() }))
            .collect();
        let stopping = AtomicBool::new(false);
        let dead: Sender6 = Arc::new(|_: &str| Err("connection refused".into()));
        let (sent, err) = flush(&rx, "v", &mut pending, None, &stopping, &dead);
        assert_eq!(sent, 0);
        assert!(err.is_some());
        assert_eq!(pending.len(), 3, "an outage ate the queue");

        // Network back: everything goes, oldest first.
        let (send, calls) = slow_sender(Duration::from_millis(0));
        let (sent, err) = flush(&rx, "v", &mut pending, None, &stopping, &send);
        assert_eq!(sent, 3);
        assert!(err.is_none());
        assert!(pending.is_empty());
        assert_eq!(calls.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn a_body_that_is_not_a_list_is_an_error_rather_than_an_empty_result() {
        assert!(parse_spots("{\"error\":\"nope\"}", Query::HeardUs).is_err());
        assert!(parse_spots("not json", Query::HeardUs).is_err());
        assert!(parse_spots("[]", Query::HeardUs).expect("empty list is fine").is_empty());
    }
}
