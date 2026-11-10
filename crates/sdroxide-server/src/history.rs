//! What the station remembers of each aircraft: `GET /aircraft-history` and
//! `GET /aircraft-history/{icao}`.
//!
//! The ADS-B and VDL2 decoders already tell this server everything it keeps —
//! positions, callsigns, registrations — and the relay's answers pass through it
//! on their way to a client. This module only writes them down
//! ([`sdroxide_aircraft_history`] is the model), beside the rest of the config,
//! and hands them back. It asks nobody for anything: no network, no radio.
//!
//! **Off unless the operator switched it on** ([`enabled_by`]): it keeps
//! positions on disk, and that is the operator's call. When it is off — which
//! is the default — nothing is read, kept, written or served.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::extract::{Path, Query, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};

use sdroxide_aircraft_history::export::{self, Format};
use sdroxide_aircraft_history::gz::{self, COMPRESS_ABOVE_BYTES};
use sdroxide_aircraft_history::{AircraftReply, Fix, History, ListReply, Place, SeenCard, Status};
use sdroxide_aircraft_online::{Info, Status as InfoStatus};
use sdroxide_types::{AdsbStatus, RadioEvent, Vdl2AddrKind, Vdl2Payload, Vdl2Status};

use crate::Station;

/// The variable that switches it on.
pub const ENV_SWITCH: &str = "SDROXIDE_AIRCRAFT_HISTORY";

/// The file, in the config directory.
pub(crate) const FILE: &str = "aircraft-history.json";

/// The file is written at most this often, and only when something changed.
pub(crate) const FLUSH_EVERY_S: i64 = 60;

/// Whether a value of [`ENV_SWITCH`] switches it on. Anything but a plain yes is
/// off: a typo must not start keeping a record.
pub fn enabled_by(value: Option<&str>) -> bool {
    value.is_some_and(|v| {
        matches!(v.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on")
    })
}

fn now_unix() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs() as i64)
}

struct Inner {
    history: History,
    last_flush: i64,
    /// The newest VDL2 message already taken from the decoder's log, which
    /// arrives whole every time.
    vdl2_until: i64,
}

pub(crate) struct HistoryHub {
    pub enabled: bool,
    inner: Mutex<Inner>,
    file: Option<PathBuf>,
    /// A write is running on its thread: no second one is started over it.
    writing: Arc<std::sync::atomic::AtomicBool>,
}

impl HistoryHub {
    /// From the environment and the config directory.
    pub(crate) fn start() -> HistoryHub {
        let enabled = enabled_by(std::env::var(ENV_SWITCH).ok().as_deref());
        let file =
            enabled.then(|| sdroxide_config::config_dir().ok().map(|d| d.join(FILE))).flatten();
        HistoryHub::open(enabled, file, now_unix())
    }

    /// A hub over `file`, which is read now, and pruned, if the hub is on. A hub
    /// that is off never looks at the file.
    pub(crate) fn open(enabled: bool, file: Option<PathBuf>, now: i64) -> HistoryHub {
        let file = file.filter(|_| enabled);
        let history = file
            .as_ref()
            .and_then(|f| std::fs::read(f).ok())
            .and_then(|bytes| gz::decode(&bytes))
            .map(|text| History::from_json(&text, now))
            .unwrap_or_default();
        HistoryHub {
            enabled,
            inner: Mutex::new(Inner { history, last_flush: now, vdl2_until: 0 }),
            file,
            writing: Arc::default(),
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }

    /// Every aircraft on the decoder's list, as of the last time it was heard.
    pub(crate) fn note_adsb(&self, st: &AdsbStatus) {
        if !self.enabled {
            return;
        }
        let mut inner = self.lock();
        for a in st.aircraft.iter().filter(|a| a.last_at > 0) {
            let fix = a.lat.zip(a.lon).filter(|_| a.last_pos_at > 0).map(|(lat, lon)| Fix {
                at: a.last_pos_at,
                lat,
                lon,
                altitude_ft: a.altitude_ft,
                on_ground: a.on_ground,
            });
            inner.history.note_heard(
                a.last_at,
                a.icao,
                Some(&a.callsign),
                a.category.as_deref(),
                fix,
            );
        }
    }

    /// The decoder's message log, which comes whole every time: a message that
    /// was already taken changes nothing, and older ones are not looked at.
    pub(crate) fn note_vdl2(&self, st: &Vdl2Status) {
        if !self.enabled {
            return;
        }
        let mut inner = self.lock();
        let until = inner.vdl2_until;
        let mut newest = until;
        for m in st.messages.iter().filter(|m| m.at >= until) {
            newest = newest.max(m.at);
            let from_aircraft = m.src_kind == Vdl2AddrKind::Aircraft;
            let icao = if from_aircraft {
                m.src
            } else if m.dst_kind == Vdl2AddrKind::Aircraft {
                m.dst
            } else {
                continue;
            };
            match &m.payload {
                Vdl2Payload::Acars(ac) => inner.history.note_identity(
                    m.at,
                    icao,
                    Some(&ac.flight),
                    Some(ac.registration.trim_start_matches(['.', ' '])),
                ),
                Vdl2Payload::Xid(x) if from_aircraft => {
                    if let Some((lat, lon)) = x.lat.zip(x.lon) {
                        let fix = Fix { at: m.at, lat, lon, altitude_ft: None, on_ground: false };
                        inner.history.note_fix(m.at, icao, fix);
                    }
                }
                _ => {}
            }
        }
        inner.vdl2_until = newest;
    }

    /// The relay answered for `icao`: keep the card.
    pub(crate) fn note_card(&self, now: i64, icao: u32, info: &Info) {
        if !self.enabled {
            return;
        }
        if let Some(card) = seen_card(info, now) {
            self.lock().history.note_card(now, icao, card);
        }
    }

    /// The text to write and where, if a write is due: something changed and
    /// [`FLUSH_EVERY_S`] have passed since the last one.
    pub(crate) fn due_write(&self, now: i64) -> Option<(PathBuf, String)> {
        let file = self.file.as_ref().filter(|_| self.enabled)?;
        let mut inner = self.lock();
        if now - inner.last_flush < FLUSH_EVERY_S {
            return None;
        }
        inner.history.prune(now);
        if !inner.history.take_dirty() {
            return None;
        }
        inner.last_flush = now;
        Some((file.clone(), inner.history.to_json()))
    }

    /// Write the file if it is due, on a thread of its own: the file is large
    /// and the caller is a radio's event loop.
    pub(crate) fn flush_if_due(&self, now: i64) {
        use std::sync::atomic::Ordering;
        // One write at a time: a slow disk must not pile writes up, and what is
        // due now is still due when the running one is done.
        if self.writing.load(Ordering::SeqCst) {
            return;
        }
        if let Some((path, text)) = self.due_write(now) {
            let writing = self.writing.clone();
            writing.store(true, Ordering::SeqCst);
            std::thread::spawn(move || {
                let bytes = gz::encode(&text, COMPRESS_ABOVE_BYTES);
                if let Err(e) = write_atomic_bytes(&path, &bytes) {
                    tracing::warn!(error = %e, "could not write the aircraft history");
                }
                writing.store(false, Ordering::SeqCst);
            });
        }
    }

    pub(crate) fn list(&self) -> ListReply {
        self.lock().history.list_reply()
    }

    /// The kept aircraft as a table, one row each, in the format asked for. No
    /// position, no picture and no address is in it (see [`export`]).
    pub(crate) fn export(&self, format: Format, now: i64) -> String {
        let rows = export::rows_from_history(&self.lock().history);
        format.render(&rows, export::Scope::History, now)
    }

    pub(crate) fn one(&self, icao: u32) -> AircraftReply {
        self.lock().history.aircraft_reply(icao)
    }

    #[cfg(test)]
    fn with<R>(&self, f: impl FnOnce(&History) -> R) -> R {
        f(&self.lock().history)
    }
}

/// Replace `path` by a sibling temp file renamed into place, so a crash leaves
/// the old file, never half of a new one.
pub(crate) fn write_atomic(path: &std::path::Path, text: &str) -> std::io::Result<()> {
    write_atomic_bytes(path, text.as_bytes())
}

pub(crate) fn write_atomic_bytes(path: &std::path::Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write;
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let mut tmp = path.as_os_str().to_owned();
    tmp.push(".tmp");
    let tmp = PathBuf::from(tmp);
    {
        let mut f = std::fs::File::create(&tmp)?;
        f.write_all(bytes)?;
        // On disk before the rename makes it the real file.
        f.sync_all()?;
    }
    std::fs::rename(&tmp, path)
}

/// What an event adds to the history, if it adds anything.
pub(crate) fn observe_at(hub: &HistoryHub, ev: &RadioEvent, now: i64) {
    if !hub.enabled {
        return;
    }
    match ev {
        RadioEvent::AdsbStatus(st) => hub.note_adsb(st),
        RadioEvent::Vdl2Status(st) => hub.note_vdl2(st),
        _ => return,
    }
    hub.flush_if_due(now);
}

pub(crate) fn observe(hub: &HistoryHub, ev: &RadioEvent) {
    observe_at(hub, ev, now_unix());
}

/// The relay's card as the history keeps it: the airframe, the route, and the
/// page and credit of the photo — never the picture or its address. Nothing
/// when the relay had nothing to say.
pub(crate) fn seen_card(info: &Info, now: i64) -> Option<SeenCard> {
    if info.status != InfoStatus::Ok {
        return None;
    }
    let place = |p: &sdroxide_aircraft_online::Airport| Place {
        code: p.code.clone(),
        name: p.name.clone(),
    };
    let air = info.aircraft.as_ref();
    let card = SeenCard {
        at: info.fetched_at.unwrap_or(now),
        type_name: air.and_then(|a| a.type_name.clone()),
        manufacturer: air.and_then(|a| a.manufacturer.clone()),
        operator: air.and_then(|a| a.operator.clone()),
        registration: air.and_then(|a| a.registration.clone()),
        origin: info.route.as_ref().and_then(|r| r.origin.as_ref()).map(place),
        destination: info.route.as_ref().and_then(|r| r.destination.as_ref()).map(place),
        photo_page: info.photo.as_ref().map(|p| p.page_url.clone()),
        photo_credit: info.photo.as_ref().map(|p| p.photographer.clone()),
    };
    let nothing = SeenCard { at: card.at, ..SeenCard::default() };
    (card != nothing).then_some(card)
}

/// What the route answers to, decided before anything is read.
#[derive(Debug, PartialEq)]
pub(crate) enum Verdict {
    Reject(Status),
    List,
    One(u32),
}

/// Sign-in first, then the switch, then the question itself.
pub(crate) fn judge(
    enabled: bool,
    sign_in_needed: bool,
    signed_in: bool,
    icao: Option<&str>,
) -> Verdict {
    if sign_in_needed && !signed_in {
        return Verdict::Reject(Status::Unauthorized);
    }
    if !enabled {
        return Verdict::Reject(Status::Disabled);
    }
    match icao {
        None => Verdict::List,
        Some(s) if s.len() == 6 && s.bytes().all(|b| b.is_ascii_hexdigit()) => {
            u32::from_str_radix(s, 16).map_or(Verdict::Reject(Status::Invalid), Verdict::One)
        }
        Some(_) => Verdict::Reject(Status::Invalid),
    }
}

/// What the export route answers to, decided before anything is read: sign-in
/// first, then the switch, then the format.
#[derive(Debug, PartialEq)]
pub(crate) enum ExportVerdict {
    Reject(Status),
    Send(Format),
}

pub(crate) fn judge_export(
    enabled: bool,
    sign_in_needed: bool,
    signed_in: bool,
    format: Option<&str>,
) -> ExportVerdict {
    if sign_in_needed && !signed_in {
        return ExportVerdict::Reject(Status::Unauthorized);
    }
    if !enabled {
        return ExportVerdict::Reject(Status::Disabled);
    }
    match format.and_then(Format::parse) {
        Some(f) => ExportVerdict::Send(f),
        None => ExportVerdict::Reject(Status::Invalid),
    }
}

pub(crate) fn http_status(s: Status) -> StatusCode {
    match s {
        Status::Ok => StatusCode::OK,
        Status::Disabled => StatusCode::SERVICE_UNAVAILABLE,
        Status::Invalid => StatusCode::BAD_REQUEST,
        Status::Unauthorized => StatusCode::UNAUTHORIZED,
    }
}

fn json<T: serde::Serialize>(status: StatusCode, body: &T) -> Response {
    (
        status,
        [(header::CONTENT_TYPE, "application/json"), (header::CACHE_CONTROL, "no-store")],
        serde_json::to_string(body).unwrap_or_default(),
    )
        .into_response()
}

fn verdict_for(station: &Station, headers: &HeaderMap, icao: Option<&str>) -> Verdict {
    let needed = station.auth.required().is_some();
    let signed = crate::session_cookie::from_headers(headers, &station.auth).is_some();
    judge(station.history.enabled, needed, signed, icao)
}

pub(crate) async fn list_route(
    State(station): State<Arc<Station>>,
    headers: HeaderMap,
) -> Response {
    match verdict_for(&station, &headers, None) {
        Verdict::Reject(s) => json(http_status(s), &ListReply::bare(s)),
        _ => json(StatusCode::OK, &station.history.list()),
    }
}

pub(crate) async fn one_route(
    State(station): State<Arc<Station>>,
    Path(icao): Path<String>,
    headers: HeaderMap,
) -> Response {
    match verdict_for(&station, &headers, Some(&icao)) {
        Verdict::Reject(s) => json(http_status(s), &AircraftReply::bare(s)),
        Verdict::One(icao) => json(StatusCode::OK, &station.history.one(icao)),
        Verdict::List => json(StatusCode::OK, &station.history.list()),
    }
}

#[derive(serde::Deserialize)]
pub(crate) struct ExportQuery {
    format: Option<String>,
}

pub(crate) async fn export_route(
    State(station): State<Arc<Station>>,
    Query(q): Query<ExportQuery>,
    headers: HeaderMap,
) -> Response {
    let needed = station.auth.required().is_some();
    let signed = crate::session_cookie::from_headers(&headers, &station.auth).is_some();
    match judge_export(station.history.enabled, needed, signed, q.format.as_deref()) {
        ExportVerdict::Reject(s) => json(http_status(s), &ListReply::bare(s)),
        ExportVerdict::Send(format) => {
            let now = now_unix();
            let (kind, body) = (
                match format {
                    Format::Csv => "text/csv; charset=utf-8",
                    Format::Json => "application/json",
                },
                station.history.export(format, now),
            );
            (
                StatusCode::OK,
                [
                    (header::CONTENT_TYPE, kind.to_string()),
                    (header::CACHE_CONTROL, "no-store".to_string()),
                    (
                        header::CONTENT_DISPOSITION,
                        format!("attachment; filename=\"{}\"", format.file_name(now)),
                    ),
                ],
                body,
            )
                .into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdroxide_aircraft_online::{AircraftInfo, Airport, Photo, Route};
    use sdroxide_types::{AdsbAircraft, Vdl2Acars, Vdl2Frame, Vdl2Message, Vdl2Xid};

    const NOW: i64 = 1_000_000;
    const A: u32 = 0x4840D6;

    fn plane(icao: u32, call: &str, lat: f64, lon: f64, at: i64) -> AdsbAircraft {
        let mut a = AdsbAircraft::new(icao, at);
        a.callsign = call.into();
        a.category = Some("heavy".into());
        a.lat = Some(lat);
        a.lon = Some(lon);
        a.altitude_ft = Some(35_000);
        a.last_at = at;
        a.last_pos_at = at;
        a
    }

    fn adsb(list: Vec<AdsbAircraft>) -> RadioEvent {
        RadioEvent::AdsbStatus(Box::new(AdsbStatus { aircraft: list, ..AdsbStatus::default() }))
    }

    fn vdl2_msg(at: i64, src: u32, reg: &str, flight: &str) -> Vdl2Message {
        Vdl2Message {
            at,
            freq_hz: 136_975_000.0,
            src,
            src_kind: Vdl2AddrKind::Aircraft,
            dst: 0x123456,
            dst_kind: Vdl2AddrKind::GroundAdmin,
            command: true,
            frame: Vdl2Frame::I { ns: 0, nr: 0, p: false },
            payload: Vdl2Payload::Acars(Vdl2Acars {
                registration: reg.into(),
                flight: flight.into(),
                label: "H1".into(),
                ..Default::default()
            }),
            snr_db: 10.0,
            rssi_dbfs: -30.0,
            evm_deg: 5.0,
            freq_err_hz: 0.0,
            rs_corrected: 0,
            raw_hex: String::new(),
        }
    }

    fn vdl2(msgs: Vec<Vdl2Message>) -> RadioEvent {
        RadioEvent::Vdl2Status(Box::new(Vdl2Status { messages: msgs, ..Vdl2Status::default() }))
    }

    fn on() -> HistoryHub {
        HistoryHub::open(true, None, NOW)
    }

    fn tmp(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("sdroxide-hist-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d.join(FILE)
    }

    // ── the switch ────────────────────────────────────────────────────────

    #[test]
    fn it_is_off_unless_the_operator_says_a_plain_yes() {
        for on in ["1", "true", "TRUE", "yes", " on "] {
            assert!(enabled_by(Some(on)), "{on:?}");
        }
        for off in ["", "0", "false", "no", "off", "tru", "enabled", "2", "whatever"] {
            assert!(!enabled_by(Some(off)), "{off:?}");
        }
        assert!(!enabled_by(None), "off by default: nothing is read, kept, written or served");
    }

    // ── what the decoders add ─────────────────────────────────────────────

    #[test]
    fn an_adsb_status_leaves_the_position_the_callsign_and_the_category() {
        let hub = on();
        observe_at(&hub, &adsb(vec![plane(A, "KLM1234", 52.0, 5.0, NOW)]), NOW);
        hub.with(|h| {
            let j = h.journal(A).expect("a journal");
            assert_eq!(j.points.len(), 1);
            assert_eq!(j.points[0].lat(), 52.0);
            assert_eq!(j.points[0].altitude_ft(), Some(35_000));
            assert_eq!(j.callsign(), Some("KLM1234"));
            assert_eq!(j.category.as_deref(), Some("heavy"));
        });
        // The same status again, as it arrives twice a second, adds nothing.
        observe_at(&hub, &adsb(vec![plane(A, "KLM1234", 52.0, 5.0, NOW)]), NOW + 1);
        hub.with(|h| assert_eq!(h.journal(A).unwrap().points.len(), 1));
    }

    #[test]
    fn an_aircraft_with_no_position_leaves_only_who_it_is() {
        let hub = on();
        let mut a = plane(A, "KLM1234", 0.0, 0.0, NOW);
        a.lat = None;
        a.lon = None;
        a.last_pos_at = 0;
        let mut b = plane(0x3C6444, "", 52.0, 5.0, NOW);
        b.callsign = String::new();
        observe_at(&hub, &adsb(vec![a, b]), NOW);
        hub.with(|h| {
            assert!(h.journal(A).unwrap().points.is_empty());
            assert_eq!(h.journal(A).unwrap().callsign(), Some("KLM1234"));
            let b = h.journal(0x3C6444).unwrap();
            assert_eq!(b.points.len(), 1);
            assert!(b.callsigns.is_empty(), "no callsign said yet, none made up");
        });
    }

    #[test]
    fn a_vdl2_log_gives_registrations_and_a_message_is_taken_once() {
        let hub = on();
        let log = vec![vdl2_msg(NOW, A, ".PH-BXA", "KL1234")];
        observe_at(&hub, &vdl2(log.clone()), NOW);
        hub.with(|h| {
            let j = h.journal(A).unwrap();
            assert_eq!(j.registration(), Some("PH-BXA"), "the leading pad is not part of it");
            assert_eq!(j.callsign(), Some("KL1234"));
        });
        hub.with(|_| ());
        assert!(hub.inner.lock().unwrap().history.take_dirty());
        // The decoder sends the whole log every time: the same one changes nothing.
        observe_at(&hub, &vdl2(log.clone()), NOW + 5);
        assert!(!hub.inner.lock().unwrap().history.take_dirty());
        // A new message in it is taken, and the old one is not taken again.
        let mut more = log;
        more.push(vdl2_msg(NOW + 30, 0x3C6444, "D-AIXA", ""));
        observe_at(&hub, &vdl2(more), NOW + 31);
        hub.with(|h| assert_eq!(h.journal(0x3C6444).unwrap().registration(), Some("D-AIXA")));
    }

    #[test]
    fn a_vdl2_message_addressed_to_the_aircraft_counts_and_one_between_ground_stations_does_not() {
        let hub = on();
        let mut down = vdl2_msg(NOW, 0x123456, "G-EUUU", "");
        down.src_kind = Vdl2AddrKind::GroundAdmin;
        down.dst = 0x400001;
        down.dst_kind = Vdl2AddrKind::Aircraft;
        let mut ground = vdl2_msg(NOW + 1, 0x111111, "X-NONE", "");
        ground.src_kind = Vdl2AddrKind::GroundAdmin;
        observe_at(&hub, &vdl2(vec![down, ground]), NOW + 2);
        hub.with(|h| {
            assert_eq!(h.journal(0x400001).unwrap().registration(), Some("G-EUUU"));
            assert!(h.journal(0x111111).is_none() && h.journal(0x123456).is_none());
            assert_eq!(h.len(), 1);
        });
    }

    #[test]
    fn a_vdl2_position_report_leaves_a_point() {
        let hub = on();
        let mut m = vdl2_msg(NOW, A, "PH-BXA", "");
        m.payload = Vdl2Payload::Xid(Box::new(Vdl2Xid {
            lat: Some(52.3),
            lon: Some(4.8),
            ..Default::default()
        }));
        observe_at(&hub, &vdl2(vec![m]), NOW);
        hub.with(|h| {
            let p = &h.journal(A).unwrap().points;
            assert_eq!(p.len(), 1);
            assert!((p[0].lat() - 52.3).abs() < 1e-4);
        });
    }

    #[test]
    fn only_adsb_and_vdl2_add_anything() {
        let hub = on();
        observe_at(&hub, &RadioEvent::IsmReports(Vec::new()), NOW);
        hub.with(|h| assert!(h.is_empty()));
        // The control: the same call with an ADS-B event does add.
        observe_at(&hub, &adsb(vec![plane(A, "KLM1", 52.0, 5.0, NOW)]), NOW);
        hub.with(|h| assert_eq!(h.len(), 1));
    }

    #[test]
    fn two_radios_feed_the_one_history() {
        let hub = Arc::new(on());
        let (r1, r2) = (hub.clone(), hub.clone());
        observe_at(&r1, &adsb(vec![plane(A, "KLM1", 52.0, 5.0, NOW)]), NOW);
        observe_at(&r2, &adsb(vec![plane(0x3C6444, "DLH2", 53.0, 6.0, NOW)]), NOW);
        hub.with(|h| assert_eq!(h.len(), 2));
    }

    #[test]
    fn switched_off_it_keeps_and_serves_nothing() {
        let hub = HistoryHub::open(false, None, NOW);
        observe_at(&hub, &adsb(vec![plane(A, "KLM1", 52.0, 5.0, NOW)]), NOW);
        observe_at(&hub, &vdl2(vec![vdl2_msg(NOW, A, "PH-BXA", "")]), NOW);
        hub.note_card(NOW, A, &an_info());
        hub.with(|h| assert!(h.is_empty()));
        assert!(hub.due_write(NOW + 1000).is_none());
        // The control: the same three calls on a hub that is on do keep.
        let hub = on();
        observe_at(&hub, &adsb(vec![plane(A, "KLM1", 52.0, 5.0, NOW)]), NOW);
        observe_at(&hub, &vdl2(vec![vdl2_msg(NOW, 0x3C6444, "D-AIXA", "")]), NOW);
        hub.note_card(NOW, 0x484000, &an_info());
        hub.with(|h| assert_eq!(h.len(), 3));
    }

    // ── the card ──────────────────────────────────────────────────────────

    fn an_info() -> Info {
        let mut i = Info::bare(InfoStatus::Ok);
        i.fetched_at = Some(NOW - 5);
        i.aircraft = Some(AircraftInfo {
            type_name: Some("737-800".into()),
            manufacturer: Some("Boeing".into()),
            operator: Some("KLM".into()),
            registration: Some("PH-BXA".into()),
        });
        i.route = Some(Route {
            origin: Some(Airport { code: "EHAM".into(), name: "Amsterdam Schiphol".into() }),
            destination: None,
        });
        i.photo = Some(Photo {
            thumb_url: "https://t.plnspttrs.net/12345/abc_t.jpg".into(),
            page_url: "https://www.planespotters.net/photo/12345/x".into(),
            photographer: "A. Spotter".into(),
        });
        i
    }

    #[test]
    fn the_relays_card_is_kept_with_its_page_and_credit_and_never_the_picture() {
        let c = seen_card(&an_info(), NOW).expect("a card");
        assert_eq!(c.at, NOW - 5, "when the relay got it, not when it was passed on");
        assert_eq!(c.type_name.as_deref(), Some("737-800"));
        assert_eq!(c.operator.as_deref(), Some("KLM"));
        assert_eq!(c.origin.as_ref().map(|p| p.code.as_str()), Some("EHAM"));
        assert_eq!(c.photo_page.as_deref(), Some("https://www.planespotters.net/photo/12345/x"));
        assert_eq!(c.photo_credit.as_deref(), Some("A. Spotter"));
        let text = serde_json::to_string(&c).unwrap();
        assert!(!text.contains("plnspttrs") && !text.contains("thumb"), "{text}");
    }

    #[test]
    fn an_answer_that_is_not_a_card_is_not_kept() {
        for s in [
            InfoStatus::Unknown,
            InfoStatus::Unavailable,
            InfoStatus::Busy,
            InfoStatus::Disabled,
            InfoStatus::Invalid,
            InfoStatus::Unauthorized,
        ] {
            let mut i = an_info();
            i.status = s;
            assert!(seen_card(&i, NOW).is_none(), "{s:?}");
        }
        assert!(seen_card(&Info::bare(InfoStatus::Ok), NOW).is_none(), "an Ok with nothing in it");
        // The control: the same answer with status Ok is kept.
        assert!(seen_card(&an_info(), NOW).is_some());
    }

    #[test]
    fn a_card_the_relay_gave_is_in_the_history() {
        let hub = on();
        hub.note_card(NOW, A, &an_info());
        hub.with(|h| {
            let j = h.journal(A).unwrap();
            assert_eq!(j.card.as_ref().unwrap().type_name.as_deref(), Some("737-800"));
        });
        let json = serde_json::to_string(&hub.one(A)).unwrap();
        assert!(json.contains("\"status\":\"ok\"") && json.contains("737-800"), "{json}");
    }

    // ── what the routes answer ────────────────────────────────────────────

    #[test]
    fn the_sign_in_is_asked_for_first_then_the_switch_then_the_question() {
        assert_eq!(judge(true, true, false, None), Verdict::Reject(Status::Unauthorized));
        assert_eq!(judge(false, true, false, Some("zz")), Verdict::Reject(Status::Unauthorized));
        assert_eq!(judge(false, false, false, None), Verdict::Reject(Status::Disabled));
        assert_eq!(judge(false, true, true, Some("484006")), Verdict::Reject(Status::Disabled));
        assert_eq!(judge(true, true, true, None), Verdict::List);
        assert_eq!(judge(true, false, false, Some("4840d6")), Verdict::One(0x4840D6));
    }

    #[test]
    fn an_address_that_is_not_six_hex_digits_is_refused() {
        for bad in ["", "48400", "4840066", "48400G", "0x4840", "48 006", "../..", "-48400"] {
            assert_eq!(
                judge(true, false, false, Some(bad)),
                Verdict::Reject(Status::Invalid),
                "{bad:?}"
            );
        }
    }

    #[test]
    fn the_statuses_are_the_ones_the_relay_uses() {
        assert_eq!(http_status(Status::Ok), StatusCode::OK);
        assert_eq!(http_status(Status::Disabled), StatusCode::SERVICE_UNAVAILABLE);
        assert_eq!(http_status(Status::Invalid), StatusCode::BAD_REQUEST);
        assert_eq!(http_status(Status::Unauthorized), StatusCode::UNAUTHORIZED);
    }

    #[test]
    fn an_address_never_heard_is_an_empty_ok() {
        let hub = on();
        let r = hub.one(0x123456);
        assert_eq!((r.status, r.journal), (Status::Ok, None));
        assert_eq!(hub.list().status, Status::Ok);
    }

    // ── the file ──────────────────────────────────────────────────────────

    #[test]
    fn a_write_that_fails_leaves_the_old_file_whole() {
        let path = tmp("failed");
        std::fs::write(&path, "the old file").unwrap();
        // The temp file cannot be made: its name is a folder.
        let mut tmp_name = path.as_os_str().to_owned();
        tmp_name.push(".tmp");
        std::fs::create_dir(&tmp_name).unwrap();
        assert!(write_atomic(&path, "a new one that never lands").is_err());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "the old file");
    }

    #[test]
    fn what_is_written_has_been_pruned_first() {
        let path = tmp("pruned");
        let hub = HistoryHub::open(true, Some(path), NOW);
        observe_at(&hub, &adsb(vec![plane(A, "KLM1", 52.0, 5.0, NOW)]), NOW);
        let later = NOW + 25 * 3600;
        let (_, text) = hub.due_write(later).expect("a removal is a change worth writing");
        assert!(
            !text.contains("KLM1"),
            "a day and an hour on, none of it goes to the file: {text}"
        );
    }

    #[test]
    fn a_second_write_is_not_started_while_the_first_is_still_running() {
        let path = tmp("busy");
        let hub = HistoryHub::open(true, Some(path.clone()), NOW);
        observe_at(&hub, &adsb(vec![plane(A, "KLM1", 52.0, 5.0, NOW)]), NOW);
        hub.writing.store(true, std::sync::atomic::Ordering::SeqCst);
        hub.flush_if_due(NOW + 1000);
        std::thread::sleep(std::time::Duration::from_millis(200));
        assert!(!path.exists(), "a write was already running, so none was started");
        // And what it would have written is still due once that one is done.
        hub.writing.store(false, std::sync::atomic::Ordering::SeqCst);
        hub.flush_if_due(NOW + 1000);
        for _ in 0..50 {
            if path.exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert!(path.exists());
    }

    #[test]
    fn the_file_is_written_whole_with_no_temp_file_left() {
        let path = tmp("atomic");
        write_atomic(&path, "{\"v\":1,\"journals\":[]}").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "{\"v\":1,\"journals\":[]}");
        write_atomic(&path, "second").unwrap();
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "second", "replaced, not appended");
        let names: Vec<_> = std::fs::read_dir(path.parent().unwrap())
            .unwrap()
            .map(|e| e.unwrap().file_name().to_string_lossy().into_owned())
            .collect();
        assert_eq!(names, [FILE], "the temp file was renamed away");
    }

    #[test]
    fn a_write_is_due_once_something_changed_and_a_minute_has_passed() {
        let path = tmp("due");
        let hub = HistoryHub::open(true, Some(path.clone()), NOW);
        assert!(hub.due_write(NOW + 1000).is_none(), "nothing changed, nothing to write");
        observe_at(&hub, &adsb(vec![plane(A, "KLM1", 52.0, 5.0, NOW)]), NOW + 1);
        assert!(hub.due_write(NOW + 10).is_none(), "changed, but not a minute since it opened");
        let (p, text) = hub.due_write(NOW + FLUSH_EVERY_S).expect("a minute on, and it changed");
        assert_eq!(p, path);
        assert!(text.contains("KLM1"), "{text}");
        assert!(hub.due_write(NOW + FLUSH_EVERY_S + 1).is_none(), "written, so not again");
        observe_at(&hub, &adsb(vec![plane(A, "KLM1", 52.5, 5.0, NOW + 30)]), NOW + 30);
        assert!(
            hub.due_write(NOW + FLUSH_EVERY_S + 10).is_none(),
            "changed, but only 10 s since the write"
        );
        assert!(hub.due_write(NOW + 2 * FLUSH_EVERY_S + 1).is_some());
    }

    #[test]
    fn what_was_written_is_read_back_and_pruned_on_opening() {
        let path = tmp("reopen");
        let hub = HistoryHub::open(true, Some(path.clone()), NOW);
        observe_at(&hub, &adsb(vec![plane(A, "KLM1", 52.0, 5.0, NOW)]), NOW);
        let (p, text) = hub.due_write(NOW + FLUSH_EVERY_S).unwrap();
        write_atomic(&p, &text).unwrap();

        let again = HistoryHub::open(true, Some(path.clone()), NOW + 100);
        again.with(|h| assert_eq!(h.journal(A).unwrap().callsign(), Some("KLM1")));
        let later = HistoryHub::open(true, Some(path), NOW + 25 * 3600);
        later.with(|h| assert!(h.is_empty(), "a day and an hour on, none of it is kept"));
    }

    #[test]
    fn switched_off_it_neither_reads_nor_writes_the_file() {
        let path = tmp("off");
        std::fs::write(&path, "left alone").unwrap();
        let fresh = tmp("off-absent");
        let hub = HistoryHub::open(false, Some(path.clone()), NOW);
        observe_at(&hub, &adsb(vec![plane(A, "KLM1", 52.0, 5.0, NOW)]), NOW + 500);
        hub.flush_if_due(NOW + 5000);
        std::thread::sleep(std::time::Duration::from_millis(200));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "left alone");
        let hub = HistoryHub::open(false, Some(fresh.clone()), NOW);
        observe_at(&hub, &adsb(vec![plane(A, "KLM1", 52.0, 5.0, NOW)]), NOW + 500);
        hub.flush_if_due(NOW + 5000);
        std::thread::sleep(std::time::Duration::from_millis(200));
        assert!(!fresh.exists(), "and it does not make the file");
        // The control: switched on, the same calls do write it.
        let hub = HistoryHub::open(true, Some(fresh.clone()), NOW);
        observe_at(&hub, &adsb(vec![plane(A, "KLM1", 52.0, 5.0, NOW)]), NOW + 500);
        hub.flush_if_due(NOW + 5000);
        for _ in 0..50 {
            if fresh.exists() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
        assert!(fresh.exists(), "switched on, the file is written");
    }

    // ── the export ────────────────────────────────────────────────────────

    #[test]
    fn the_export_wants_the_sign_in_then_the_switch_then_a_format() {
        use ExportVerdict::{Reject, Send};
        use Format::{Csv, Json};
        assert_eq!(judge_export(true, true, false, Some("csv")), Reject(Status::Unauthorized));
        assert_eq!(judge_export(false, true, false, Some("csv")), Reject(Status::Unauthorized));
        assert_eq!(judge_export(false, false, false, Some("csv")), Reject(Status::Disabled));
        assert_eq!(judge_export(true, false, false, None), Reject(Status::Invalid));
        assert_eq!(judge_export(true, false, false, Some("xml")), Reject(Status::Invalid));
        assert_eq!(judge_export(true, false, false, Some("")), Reject(Status::Invalid));
        assert_eq!(judge_export(true, false, false, Some("csv")), Send(Csv));
        assert_eq!(judge_export(true, true, true, Some(" JSON ")), Send(Json));
    }

    #[test]
    fn the_export_holds_what_was_heard_and_no_position() {
        let hub = on();
        observe_at(&hub, &adsb(vec![plane(A, "KLM1234", 52.3123, 4.7634, NOW)]), NOW);
        let csv = hub.export(Format::Csv, NOW + 5);
        assert!(csv.starts_with("icao,callsign,"), "{csv}");
        assert!(csv.contains("4840D6,KLM1234,"), "{csv}");
        let json = hub.export(Format::Json, NOW + 5);
        let v: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(v["aircraft"][0]["callsign"], "KLM1234");
        assert_eq!(v["aircraft"][0]["positions_kept"], 1);
        assert_eq!(v["generated_utc"], export::iso_utc(NOW + 5));
        for text in [csv, json] {
            for banned in ["52.31", "4.76", "35000", "\"lat\"", "\"lon\"", "altitude"] {
                assert!(!text.contains(banned), "`{banned}` in {text}");
            }
        }
    }

    #[test]
    fn a_hub_that_is_off_exports_nothing_it_never_kept() {
        let hub = HistoryHub::open(false, None, NOW);
        observe_at(&hub, &adsb(vec![plane(A, "KLM1234", 52.0, 5.0, NOW)]), NOW);
        assert_eq!(hub.export(Format::Csv, NOW).lines().count(), 1, "the header only");
    }

    #[test]
    fn the_export_route_is_one_line_in_the_station() {
        let src = include_str!("lib.rs");
        assert!(src.contains("history::export_route"));
        assert!(src.contains("\"/aircraft-export\""));
    }

    // ── the file, once it grows ───────────────────────────────────────────

    fn busy(n: u32) -> HistoryHub {
        let hub = on();
        let planes: Vec<_> = (0..n)
            .map(|i| plane(0x400000 + i, &format!("BAW{i}"), 51.0 + f64::from(i) * 1e-3, 0.5, NOW))
            .collect();
        observe_at(&hub, &adsb(planes), NOW);
        hub
    }

    #[test]
    fn a_big_history_is_written_gzipped_and_read_back_whole() {
        let path = tmp("gz-roundtrip");
        let text = busy(2000).with(|h| h.to_json());
        write_atomic_bytes(&path, &gz::encode(&text, 1000)).unwrap();
        let on_disk = std::fs::read(&path).unwrap();
        assert_eq!(on_disk[..2], [0x1f, 0x8b]);
        assert!((on_disk.len() as u64) < text.len() as u64 / 3);
        HistoryHub::open(true, Some(path), NOW + 10).with(|h| {
            assert_eq!(h.len(), 2000);
            assert_eq!(h.journal(0x400000).unwrap().callsign(), Some("BAW0"));
        });
    }

    #[test]
    fn a_plain_file_from_before_is_still_read() {
        let path = tmp("plain-before");
        std::fs::write(&path, busy(50).with(|h| h.to_json())).unwrap();
        HistoryHub::open(true, Some(path), NOW + 10).with(|h| assert_eq!(h.len(), 50));
    }

    #[test]
    fn a_damaged_file_starts_an_empty_history() {
        let path = tmp("damaged-gz");
        std::fs::write(&path, [0x1f, 0x8b, 0x08, 0x00, 0xde, 0xad]).unwrap();
        HistoryHub::open(true, Some(path), NOW).with(|h| assert!(h.is_empty()));
    }

    #[test]
    fn a_failed_gzip_write_leaves_the_old_file_whole() {
        let path = tmp("gz-failed");
        std::fs::write(&path, "the old file").unwrap();
        let mut tmp_name = path.as_os_str().to_owned();
        tmp_name.push(".tmp");
        std::fs::create_dir(&tmp_name).unwrap();
        let text = busy(300).with(|h| h.to_json());
        assert!(write_atomic_bytes(&path, &gz::encode(&text, 10)).is_err());
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "the old file");
    }

    #[test]
    fn the_flush_thread_encodes_with_the_one_threshold() {
        let src = include_str!("history.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap()];
        assert_eq!(code.matches("gz::encode(&text, COMPRESS_ABOVE_BYTES)").count(), 1);
    }

    // ── what this module may not do ───────────────────────────────────────

    #[test]
    fn this_module_reaches_nothing_and_keeps_no_picture() {
        let src = include_str!("history.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap()];
        for banned in [
            "ureq",
            "reqwest",
            "TcpStream",
            "adsbdb",
            "plnspttrs",
            "thumb_url",
            "raw_hex",
            "Upstream",
        ] {
            assert!(!code.contains(banned), "history.rs must not touch `{banned}`");
        }
        assert!(code.contains("write_atomic"), "the control: the scan sees what is there");
    }

    #[test]
    fn the_relay_is_touched_by_one_line_and_that_line_only_hands_over_its_answer() {
        let src = include_str!("aircraft.rs");
        let calls: Vec<&str> = src.lines().filter(|l| l.contains("history.")).collect();
        assert_eq!(calls.len(), 1, "{calls:?}");
        assert!(calls[0].contains("note_card"), "{calls:?}");
    }

    #[test]
    fn the_station_wires_it_in_for_every_radio() {
        let src = include_str!("lib.rs");
        assert!(src.contains("history: self.history.clone()"), "each radio shares the station's");
        assert!(src.contains("history::observe(&shared.history"), "and reports its events to it");
        assert!(src.contains("history::list_route") && src.contains("history::one_route"));
    }
}
