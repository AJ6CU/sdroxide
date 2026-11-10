//! The listener's reception log: what was **heard**, not worked.
//!
//! A window in the same family as the LOGBOOK, with its own file
//! (`swl_log.json`) and its own record ([`sdroxide_types::SwlEntry`]). It logs
//! a station, a frequency, a time and a SINPO or SIO judgement, and can print
//! the entry as a reception report to send to the broadcaster.
//!
//! Owned by the UI, exactly like the QSO log next door: the table lives in the
//! app, is loaded from the config directory, and is written back when it
//! changes. The engine is not involved.

use eframe::egui::{self, RichText};
use sdroxide_types::{Command, Mode, RxId, SignalReport, Sinpo, Sio, SwlEntry, Vfo};

use crate::app::SdroxideApp;
use crate::i18n::tr;
use crate::app::persist::persist_swl_log;
use crate::time::now_unix;

/// The modes a listener is likely to log, in the order that makes sense for a
/// picker. A short list rather than every mode: a reception is AM or a sideband
/// or CW or one of the broadcast ones, and forty entries would bury those.
const SWL_MODES: [Mode; 9] = [
    Mode::Am,
    Mode::Sam,
    Mode::Cquam,
    Mode::Lsb,
    Mode::Usb,
    Mode::Cw,
    Mode::Nfm,
    Mode::Wfm,
    Mode::Drm,
];

/// `2026-09-16 19:42 UTC` from Unix seconds.
fn utc_text(unix: u64) -> String {
    let (y, mo, d, h, mi, _s) = sdroxide_types::utc_ymd_hms(unix as i64);
    format!("{y:04}-{mo:02}-{d:02} {h:02}:{mi:02} UTC")
}

/// A SINPO strength figure, 1–5, for a receiver S-meter reading.
///
/// The meter reads dBm, where S9 is −73 dBm and an S-unit is 6 dB; the five
/// SINPO grades are coarser, so the boundaries sit a few S-units apart. Only
/// the strength can come from the meter — interference, noise and propagation
/// are the operator's judgement.
fn sinpo_strength(dbm: f32) -> u8 {
    if dbm >= -73.0 {
        5
    } else if dbm >= -83.0 {
        4
    } else if dbm >= -93.0 {
        3
    } else if dbm >= -103.0 {
        2
    } else {
        1
    }
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(n.saturating_sub(1)).collect();
        out.push('…');
        out
    }
}

/// How the listener is looking through the reception log: a find box, a band,
/// a day and the pirate-only switch. Session state beside the window, not a
/// property of the log — a filter is how the log is read today, and it never
/// changes what was recorded.
#[derive(Default)]
pub(in crate::app) struct SwlFilter {
    /// Free text, matched against station, language, site, notes and the
    /// broadcaster's contact.
    find: String,
    /// Only one broadcast band, or `None` for every band.
    band: Option<sdroxide_types::Band>,
    /// Only one UTC day (`YYYY-MM-DD`), or `None` for every day.
    day: Option<String>,
    /// Only the unlicensed — "pirate" — catches.
    pirates_only: bool,
}

impl SwlFilter {
    /// Whether a reception passes the filter, `day` being the row's own
    /// `YYYY-MM-DD` (already computed for the grouping).
    fn matches(&self, e: &SwlEntry, day: &str) -> bool {
        if self.pirates_only && !e.pirate {
            return false;
        }
        if let Some(b) = self.band
            && sdroxide_types::Band::containing(e.freq_hz) != b
        {
            return false;
        }
        if let Some(d) = &self.day
            && d != day
        {
            return false;
        }
        let q = self.find.trim().to_lowercase();
        if !q.is_empty() {
            let hay = format!(
                "{} {} {} {} {} {}",
                e.station, e.language, e.site, e.notes, e.email, e.address
            )
            .to_lowercase();
            if !hay.contains(&q) {
                return false;
            }
        }
        true
    }

    /// Whether anything is being filtered at all — the list says "N of M" only
    /// when it is.
    fn active(&self) -> bool {
        self.pirates_only
            || self.band.is_some()
            || self.day.is_some()
            || !self.find.trim().is_empty()
    }
}

/// A reception being typed, the frequency kept as text so partial input never
/// fights the operator. Parsed into a [`SwlEntry`] on save.
#[derive(Clone)]
pub(in crate::app) struct SwlEditForm {
    /// 0 = new entry; otherwise the id of the entry being edited.
    id: u64,
    /// Where the transmitter stands, when that is known — carried rather than
    /// typed, so an edit round-trips and a station logged off the schedule can
    /// be plotted afterwards. There is no field for it in the form on purpose:
    /// coordinates are not something an operator enters by hand, and a box that
    /// invites it is a box that gets it wrong.
    lat: Option<f64>,
    lon: Option<f64>,
    heard_at: u64,
    station: String,
    freq_khz: String,
    mode: Mode,
    language: String,
    /// True = SINPO (five figures), false = SIO (three).
    sinpo: bool,
    /// Whether a report was given at all. Off = "not judged".
    judged: bool,
    s: u8,
    i: u8,
    n: u8,
    p: u8,
    o: u8,
    smeter_dbm: Option<f32>,
    site: String,
    /// The broadcaster's report contact, copied from the schedule row when the
    /// entry is logged from SCHEDULE. See [`SwlEntry::email`].
    email: String,
    /// The broadcaster's postal address. See [`SwlEntry::address`].
    address: String,
    /// The receiving station's Maidenhead locator. See [`SwlEntry::recv_grid`].
    recv_grid: String,
    /// The antenna in use, from the SWL LOG window's session field. See
    /// [`SwlEntry::antenna`].
    antenna: String,
    notes: String,
    /// The listener marked this reception as an unlicensed ("pirate")
    /// broadcast. See [`SwlEntry::pirate`].
    pirate: bool,
    /// When the report was sent and when a QSL came back. See
    /// [`SwlEntry::report_sent_unix`] and [`SwlEntry::qsl_received_unix`].
    report_sent_unix: Option<u64>,
    qsl_received_unix: Option<u64>,
}

impl Default for SwlEditForm {
    fn default() -> Self {
        SwlEditForm {
            id: 0,
            lat: None,
            lon: None,
            heard_at: 0,
            station: String::new(),
            freq_khz: String::new(),
            mode: Mode::Am,
            language: String::new(),
            sinpo: true,
            judged: false,
            s: 3,
            i: 3,
            n: 3,
            p: 3,
            o: 3,
            smeter_dbm: None,
            site: String::new(),
            email: String::new(),
            address: String::new(),
            recv_grid: String::new(),
            antenna: String::new(),
            notes: String::new(),
            pirate: false,
            report_sent_unix: None,
            qsl_received_unix: None,
        }
    }
}

impl SwlEditForm {
    /// A fresh entry, pre-filled from the dial so logging a station found by
    /// tuning is a name and a judgement. `grid` is the receiving station's
    /// Maidenhead locator, which the log keeps as *where it was heard*, and
    /// `antenna` the aerial in use, which the log keeps for the report.
    pub(in crate::app) fn new(
        freq_hz: f64,
        mode: Mode,
        smeter_dbm: Option<f32>,
        grid: String,
        antenna: String,
    ) -> Self {
        SwlEditForm {
            id: 0,
            lat: None,
            lon: None,
            heard_at: now_unix().max(0) as u64,
            freq_khz: format!("{:.0}", freq_hz / 1e3),
            mode,
            smeter_dbm,
            recv_grid: grid,
            antenna,
            sinpo: true,
            s: 3,
            i: 3,
            n: 3,
            p: 3,
            o: 3,
            ..Default::default()
        }
    }

    /// A fresh entry from a schedule row: the station, language and site are
    /// already known, so a reception from the schedule is a judgement away.
    pub(in crate::app) fn from_station(
        freq_hz: f64,
        mode: Mode,
        station: &str,
        language: &str,
        site: &str,
        email: &str,
        address: &str,
        lat: Option<f64>,
        lon: Option<f64>,
        smeter_dbm: Option<f32>,
        grid: String,
        antenna: String,
    ) -> Self {
        let mut f = Self::new(freq_hz, mode, smeter_dbm, grid, antenna);
        f.lat = lat;
        f.lon = lon;
        f.station = station.to_string();
        f.language = language.to_string();
        f.site = site.to_string();
        f.email = email.to_string();
        f.address = address.to_string();
        f
    }

    fn from_entry(e: &SwlEntry) -> Self {
        let (sinpo, judged, s, i, n, p, o) = match e.report {
            Some(SignalReport::Sinpo(r)) => (true, true, r.s, r.i, r.n, r.p, r.o),
            Some(SignalReport::Sio(r)) => (false, true, r.s, r.i, 3, 3, r.o),
            None => (true, false, 3, 3, 3, 3, 3),
        };
        SwlEditForm {
            id: e.id,
            lat: e.lat,
            lon: e.lon,
            heard_at: e.heard_at_unix,
            station: e.station.clone(),
            freq_khz: format!("{:.0}", e.freq_hz / 1e3),
            mode: e.mode,
            language: e.language.clone(),
            sinpo,
            judged,
            s,
            i,
            n,
            p,
            o,
            smeter_dbm: e.smeter_dbm,
            site: e.site.clone(),
            email: e.email.clone(),
            address: e.address.clone(),
            recv_grid: e.recv_grid.clone(),
            antenna: e.antenna.clone(),
            notes: e.notes.clone(),
            pirate: e.pirate,
            report_sent_unix: e.report_sent_unix,
            qsl_received_unix: e.qsl_received_unix,
        }
    }

    /// Put a note in the form, replacing whatever was there.
    ///
    /// A setter rather than a `pub` field because the LISTEN window writes the
    /// operator's own words into a form it does not own, and this is the one
    /// place that is allowed to: the note is [`SwlEntry::notes`] and nothing
    /// else, and it is set **before** the form is opened so the operator can
    /// still change it.
    pub(in crate::app) fn set_notes(&mut self, notes: &str) {
        self.notes = notes.to_string();
    }

    fn to_entry(&self) -> SwlEntry {
        let freq_hz = self.freq_khz.trim().parse::<f64>().ok().map(|k| k * 1e3).unwrap_or(0.0);
        let cl = |v: u8| v.clamp(1, 5);
        let report = self.judged.then(|| {
            if self.sinpo {
                SignalReport::Sinpo(Sinpo {
                    s: cl(self.s),
                    i: cl(self.i),
                    n: cl(self.n),
                    p: cl(self.p),
                    o: cl(self.o),
                })
            } else {
                SignalReport::Sio(Sio { s: cl(self.s), i: cl(self.i), o: cl(self.o) })
            }
        });
        SwlEntry {
            id: self.id,
            heard_at_unix: self.heard_at,
            station: self.station.trim().to_string(),
            freq_hz,
            mode: self.mode,
            language: self.language.trim().to_string(),
            report,
            smeter_dbm: self.smeter_dbm,
            site: self.site.trim().to_string(),
            email: self.email.trim().to_string(),
            address: self.address.trim().to_string(),
            recv_grid: self.recv_grid.trim().to_uppercase(),
            antenna: self.antenna.trim().to_string(),
            notes: self.notes.trim().to_string(),
            lat: self.lat,
            lon: self.lon,
            pirate: self.pirate,
            report_sent_unix: self.report_sent_unix,
            qsl_received_unix: self.qsl_received_unix,
        }
    }
}

/// The station last logged on `freq_hz`, if the log already has one there.
///
/// Logged frequencies are whole kilohertz — the form parses them that way — so
/// "the same frequency" is the same kHz, not the same hertz: a dial landing a
/// few tens of hertz off a remembered broadcast is still that broadcast. The
/// most recently heard name wins, so a channel that has carried two stations
/// suggests the one logged there last. Names are only ever a suggestion: the
/// field stays editable for when the channel is carrying something else.
fn station_at(entries: &[SwlEntry], freq_hz: f64) -> Option<String> {
    let khz = (freq_hz / 1e3).round() as i64;
    entries
        .iter()
        .filter(|e| !e.station.trim().is_empty())
        .filter(|e| (e.freq_hz / 1e3).round() as i64 == khz)
        .max_by_key(|e| e.heard_at_unix)
        .map(|e| e.station.clone())
}

/// Fill a fresh form's station fields from the best source for the dial: the
/// schedule station sitting on it, or else the log's own name for the channel.
///
/// The schedule wins because it carries what a reception report needs — the
/// language, the transmitter site and the broadcaster's report contact — none
/// of which the log's remembered name has. `at_dial` is the same lookup the
/// path arc uses, so a station named here is one the map agrees is on the dial.
/// The fields stay editable: the suggestion is for when the channel is carrying
/// what the schedule says.
fn prefill_station(
    form: &mut SwlEditForm,
    broadcast: &[sdroxide_types::BroadcastStation],
    log: &[SwlEntry],
    freq_hz: f64,
    now: i64,
) {
    if let Some(st) = sdroxide_types::broadcast::at_dial(broadcast, freq_hz, now) {
        form.station = st.name.clone();
        form.language = st.lang.clone();
        form.site = st.site.clone();
        form.email = st.email.clone();
        form.address = st.address.clone();
    } else if let Some(name) = station_at(log, freq_hz) {
        form.station = name;
    }
}

impl SdroxideApp {
    /// The SWL LOG window: the reception log, its entry form and its report.
    pub(in crate::app) fn swl_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        // The reception REPORT was asked for, so its "report sent" is stamped
        // after the window has closed the borrow of the log — see below.
        let mut mark_sent: Option<u64> = None;
        // The shell's window: an egui window or its own OS one, with the ⇱
        // WINDOW chip. Titled "SWL LOG" to match its top-strip chip.
        let open =
            self.tool_window(ctx, "swl-log", &tr!("swl-title"), [720.0, 520.0], self.show_swl, |me, ui| {
                me.swl_log_body(ctx, ui, cmds, &mut mark_sent)
            });
        self.show_swl = open;
        if let Some(id) = mark_sent {
            let now = now_unix().max(0) as u64;
            let mut changed = false;
            if let Some(slot) = self.swl_log.iter_mut().find(|e| e.id == id)
                && slot.report_sent_unix.is_none()
            {
                slot.report_sent_unix = Some(now);
                changed = true;
            }
            if changed {
                persist_swl_log(&self.swl_log);
            }
        }
    }

    /// The reception log's body: the heard stations with their reception reports.
    /// Split out of [`Self::swl_window`] so the shell can draw it in a window of
    /// its own.
    fn swl_log_body(
        &mut self,
        ctx: &egui::Context,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        mark_sent: &mut Option<u64>,
    ) {
        crate::chrome::window_body_bg(ui);
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(crate::time::utc_and_local_now(now_unix()))
                    .monospace()
                    .color(crate::theme::CYAN()),
            );
            // Which build this is, beside the clock: a QSL screenshot
            // then names the version it came from.
            ui.label(
                RichText::new(format!("v{}", sdroxide_version::VERSION))
                    .size(10.5)
                    .color(crate::theme::gray(120)),
            )
            .on_hover_text(sdroxide_version::LONG_VERSION);
            ui.separator();
            if crate::chrome::chip(ui, false, tr!("swl-new")).clicked() {
                let freq = self.on_air_freq_hz();
                let mode = self.state.rx[0].mode;
                let s = self.meters.map(|m| m.s_dbm);
                let grid = self.my_grid();
                let antenna = self.swl_antenna.clone();
                let mut form = SwlEditForm::new(freq, mode, s, grid, antenna);
                // A known broadcast on this dial comes in filled with
                // what a reception report needs; see `prefill_station`.
                prefill_station(&mut form, &self.broadcast, &self.swl_log, freq, now_unix());
                self.swl_edit = Some(form);
            }
            if crate::chrome::chip(ui, false, tr!("swl-jobs"))
                .on_hover_text(tr!("swl-jobs-hint"))
                .clicked()
            {
                self.jobs.show = true;
            }
            if crate::chrome::chip(ui, false, tr!("swl-sigid"))
                .on_hover_text(tr!("swl-sigid-hint"))
                .clicked()
            {
                self.signal_id.show = true;
            }
            let replay = self.state.replay;
            if crate::chrome::chip(ui, replay, tr!("swl-replay"))
                .on_hover_text(tr!("swl-replay-hint"))
                .clicked()
            {
                cmds.push(Command::SetReplay(!replay));
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let selected =
                    self.swl_selected.and_then(|id| self.swl_log.iter().find(|e| e.id == id));
                ui.add_enabled_ui(selected.is_some(), |ui| {
                    if crate::chrome::chip(ui, false, tr!("swl-report"))
                        .on_hover_text(tr!("swl-report-hint"))
                        .clicked()
                        && let Some(e) = selected
                    {
                        let listener = self.report_identity();
                        // Where it was heard and the aerial that heard
                        // it come from the entry, which captured both at
                        // logging time; only an entry too old to have a
                        // locator falls back to the current grid.
                        let grid = if e.recv_grid.is_empty() {
                            self.my_grid()
                        } else {
                            e.recv_grid.clone()
                        };
                        let text = e.report_text(&listener, &grid, "sdroxide_SWL", &e.antenna);
                        crate::download::save("reception-report.txt", text.as_bytes());
                        // Writing the report is the "report" step of the
                        // loop; mark it done unless it already was.
                        *mark_sent = Some(e.id);
                    }
                });
                // The whole log at a glance — how many heard, and of
                // those how far round the SWL's loop they have got. The
                // counts that are zero are left out, so a fresh log reads
                // simply as "n heard".
                let n = self.swl_log.len();
                let reported = self.swl_log.iter().filter(|e| e.report_sent_unix.is_some()).count();
                let qsl = self.swl_log.iter().filter(|e| e.qsl_received_unix.is_some()).count();
                let pirates = self.swl_log.iter().filter(|e| e.pirate).count();
                let mut stats = tr!("swl-stats-heard", n = n);
                if reported > 0 {
                    stats.push_str(&format!(" · {}", tr!("swl-stats-reported", n = reported)));
                }
                if qsl > 0 {
                    stats.push_str(&format!(" · {}", tr!("swl-stats-qsl", n = qsl)));
                }
                if pirates > 0 {
                    stats.push_str(&format!(" · {}", tr!("swl-stats-pirates", n = pirates)));
                }
                ui.label(RichText::new(stats).size(11.0).color(crate::theme::gray(150)))
                    .on_hover_text(tr!("swl-stats-hint"));
            });
        });
        // The aerial, in the listener's own words, for the reception
        // report. Session state set once — an aerial is swapped far
        // more often than a settings page is opened, and it is not a
        // property of the radio, so it does not ride the wire.
        ui.horizontal(|ui| {
            ui.label(RichText::new(tr!("swl-antenna")).size(11.0).color(crate::theme::gray(150)));
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut self.swl_antenna)
                    .desired_width(240.0)
                    .hint_text(tr!("swl-antenna-hint")),
            )
            .on_hover_text(tr!("swl-antenna-tip"));
        });
        // The listener's tone control: shelves on the demodulated
        // audio, in front of the speakers. Broadcast audio wants a
        // tone control the ham speech chain never needed.
        ui.horizontal(|ui| {
            ui.label(RichText::new(tr!("swl-tone")).size(11.0).color(crate::theme::gray(150)));
            let mut tone = self.state.rx[0].tone;
            let before = tone;
            crate::chrome::checkbox(ui, &mut tone.enabled, &tr!("swl-tone-on"));
            let band = |ui: &mut egui::Ui, name: &str, b: &mut sdroxide_types::TxEqBand| {
                ui.label(RichText::new(name).size(11.0));
                ui.add(
                    egui::DragValue::new(&mut b.gain_db)
                        .speed(0.2)
                        .range(-12.0..=12.0)
                        .suffix(" dB"),
                );
            };
            band(ui, &tr!("swl-tone-bass"), &mut tone.low);
            band(ui, &tr!("swl-tone-mid"), &mut tone.mid);
            band(ui, &tr!("swl-tone-treble"), &mut tone.high);
            if tone != before {
                self.state.rx[0].tone = tone;
                cmds.push(Command::SetRxTone(Box::new(tone)));
            }
            self.loudness_toggle(ui, cmds);
        });
        if self.swl_edit.is_some() {
            ui.add_space(4.0);
            self.swl_entry_form(ui, ctx);
        }
        ui.add_space(2.0);
        self.swl_filter_row(ui);
        ui.separator();
        self.swl_list(ui, cmds);
    }

    /// The reception log's controls, drawn as the list's own header: what to
    /// show, and the way to save the whole log. The band and day choices come
    /// from the log itself, so a filter can never offer one that shows nothing.
    /// Write the reception report out and open it in the operator's mail client.
    ///
    /// **It sends nothing.** A `mailto:` cannot carry a file, so the picture the
    /// listener configured is written to disk first and the message opens with
    /// its name in the body; attaching it is the operator's own click, which is
    /// also where they can add a soundclip. This program does everything up to
    /// that point and nothing beyond it — the alternative is to become a mail
    /// client with an account and a password, which is a different program with
    /// its own security to consider.
    fn mail_reception_report(&mut self, entry: &SwlEntry, to: &str, ctx: &egui::Context) {
        if !report_address_is_usable(to) {
            // Unreachable through the UI, which greys the chip; here it is the
            // second gate, because a report delivered to a stranger is worse
            // than no report.
            return;
        }
        let mut body = reception_report_body(entry, &self.my_call(), &self.swl_report.message);
        let picture = self.swl_report.picture.trim().to_string();
        if !picture.is_empty() {
            // Named in the body whether or not the file is still there. A line
            // saying "attach: /gone/photo.png" tells the listener exactly what
            // is missing, which a silently absent line would not.
            let present = std::path::Path::new(&picture).exists();
            body.push_str("\n\n");
            body.push_str(&format!(
                "{}\n{}",
                if present {
                    "Attached: (attach this file)"
                } else {
                    "Picture (this file is gone):"
                },
                picture
            ));
        }
        let subject = reception_report_subject(entry);
        let Some(link) = reception_report_mailto(to, &subject, &body) else { return };
        // **The report goes on the clipboard whatever happens.** This program's
        // own `open_url` does nothing on the desktop — eframe only implements
        // that command in its web target — and even where it does work, the
        // handler is another process that may be missing entirely. A reception
        // report the listener typed and then lost because no mail client
        // answered is the whole failure this avoids: pasting it is one key, and
        // it is on the clipboard either way.
        ctx.copy_text(body.clone());
        if !crate::download::open_external(&link) {
            // Said rather than assumed: the clipboard is the fallback and the
            // operator has to know it is the fallback.
            self.swl_report_note =
                Some(tr!("swl-report-copied"));
        }
    }

    fn swl_filter_row(&mut self, ui: &mut egui::Ui) {
        let mut bands: Vec<sdroxide_types::Band> =
            self.swl_log.iter().map(|e| sdroxide_types::Band::containing(e.freq_hz)).collect();
        bands.sort_by(|a, b| {
            let ka = a.edges().map(|(lo, _)| lo).unwrap_or(f64::INFINITY);
            let kb = b.edges().map(|(lo, _)| lo).unwrap_or(f64::INFINITY);
            ka.partial_cmp(&kb).unwrap_or(std::cmp::Ordering::Equal)
        });
        bands.dedup();
        let mut days: Vec<String> =
            self.swl_log.iter().map(|e| utc_text(e.heard_at_unix)[..10].to_string()).collect();
        days.sort();
        days.dedup();
        days.reverse();

        ui.horizontal(|ui| {
            ui.label(RichText::new(tr!("swl-show")).size(11.0).color(crate::theme::gray(150)));
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut self.swl_filter.find)
                    .desired_width(150.0)
                    .hint_text(tr!("swl-find-hint")),
            )
            .on_hover_text(tr!("swl-find-tip"));
            let band_text = self
                .swl_filter
                .band
                .map(|b| b.label().to_string())
                .unwrap_or_else(|| tr!("swl-all-bands"));
            egui::ComboBox::from_id_salt("swl-filter-band")
                .width(74.0)
                .selected_text(band_text)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.swl_filter.band, None, tr!("swl-all-bands"));
                    for b in &bands {
                        ui.selectable_value(&mut self.swl_filter.band, Some(*b), b.label());
                    }
                });
            let day_text = self.swl_filter.day.clone().unwrap_or_else(|| tr!("swl-all-days"));
            egui::ComboBox::from_id_salt("swl-filter-day")
                .width(84.0)
                .selected_text(day_text)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.swl_filter.day, None, tr!("swl-all-days"));
                    for d in &days {
                        ui.selectable_value(&mut self.swl_filter.day, Some(d.clone()), d);
                    }
                });
            crate::chrome::checkbox(ui, &mut self.swl_filter.pirates_only, &tr!("swl-pirates-only"))
                .on_hover_text(tr!("swl-pirates-only-tip"));
            if self.swl_filter.active() && crate::chrome::chip(ui, false, tr!("decodes-clear")).clicked() {
                self.swl_filter = SwlFilter::default();
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let n = self.swl_log.len();
                // The whole reception log's export — the single-entry REPORT is
                // not that. ADIF for a logger, CSV for a spreadsheet.
                if ui
                    .add_enabled(n > 0, egui::Button::new("ADIF"))
                    .on_hover_text(tr!("swl-adif-tip"))
                    .clicked()
                {
                    let adif = sdroxide_types::swl_log_to_adif(&self.swl_log);
                    crate::download::save("sdroxide-swl-log.adi", adif.as_bytes());
                }
                if ui
                    .add_enabled(n > 0, egui::Button::new("CSV"))
                    .on_hover_text(tr!("swl-csv-tip"))
                    .clicked()
                {
                    let csv = sdroxide_types::swl_log_to_csv(&self.swl_log);
                    crate::download::save("sdroxide-swl-log.csv", csv.as_bytes());
                }
                if self.swl_filter.active() {
                    let shown = self
                        .swl_log
                        .iter()
                        .filter(|e| self.swl_filter.matches(e, &utc_text(e.heard_at_unix)[..10]))
                        .count();
                    ui.label(
                        RichText::new(tr!("swl-shown-of", shown = shown, n = n))
                            .size(11.0)
                            .color(crate::theme::gray(150)),
                    );
                }
            });
        });
    }

    /// The reception log list, newest first, grouped by day.
    fn swl_list(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        // Moved out for the frame and put back after, so the list can be drawn
        // while `self` stays free to record a selection or a delete — without
        // cloning the whole log every frame.
        let mut rows = std::mem::take(&mut self.swl_log);
        rows.sort_by_key(|e| std::cmp::Reverse(e.heard_at_unix));
        let mut selected = self.swl_selected;
        let mut edit: Option<u64> = None;
        let mut delete: Option<u64> = None;
        // A reception to tune back to: the frequency and mode, applied after the
        // list is drawn so the borrow of `rows` is done with.
        let mut recall: Option<(f64, Mode)> = None;
        egui::ScrollArea::vertical().auto_shrink([false, false]).id_salt("swl-list").show(
            ui,
            |ui| {
                if rows.is_empty() {
                    ui.label(
                        RichText::new(tr!("swl-empty"))
                            .color(crate::theme::gray(150)),
                    );
                }
                let mut last_day = String::new();
                let mut shown = 0usize;
                for e in &rows {
                    let utc = utc_text(e.heard_at_unix);
                    let day = utc[..10].to_string();
                    // A filtered-out row is skipped whole — no day header either
                    // — so a day with nothing to show is a day that is not there.
                    if !self.swl_filter.matches(e, &day) {
                        continue;
                    }
                    shown += 1;
                    if day != last_day {
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(&day).size(11.0).strong().color(crate::theme::CYAN()),
                        );
                        last_day = day;
                    }
                    let report = e
                        .report
                        .map(|r| format!("{} {}", r.label(), r.digits()))
                        .unwrap_or_default();
                    let label = format!(
                        "{:>6} kHz  {:<4}  {:<26} {:<10} {}",
                        format!("{:.0}", e.freq_hz / 1e3),
                        e.mode.label(),
                        truncate(&e.station, 26),
                        truncate(&e.language, 10),
                        report,
                    );
                    ui.horizontal(|ui| {
                        let is_sel = selected == Some(e.id);
                        // The notes are a hover now, not a column: a long line of
                        // them shoved the rest of the row about, and the row
                        // needs its one line for the things read at a glance.
                        // Hovering the station shows what was on, and what the
                        // schedule told us about the transmitter where it did.
                        let mut tip = e.notes.clone();
                        if !e.site.is_empty() {
                            if !tip.is_empty() {
                                tip.push('\n');
                            }
                            tip.push_str(&tr!("swl-transmitted-from", site = e.site));
                        }
                        let station = ui.selectable_label(is_sel, RichText::new(label).monospace());
                        let station =
                            if tip.is_empty() { station } else { station.on_hover_text(tip) };
                        if station.clicked() {
                            selected = Some(e.id);
                        }
                        if e.pirate {
                            crate::flags::pirate(ui, 14.0);
                        }
                        ui.label(RichText::new(&utc).size(10.5).color(crate::theme::gray(140)));
                        // In the notes' old place, *where this was heard* — the
                        // receiving station's locator. Empty until a grid is set.
                        if !e.recv_grid.is_empty() {
                            ui.label(
                                RichText::new(&e.recv_grid)
                                    .size(10.5)
                                    .color(crate::theme::gray(160)),
                            );
                        }
                        // The broadcaster's report contact, when the schedule
                        // gave one. One button because the report goes to the
                        // email when there is one and to the address otherwise;
                        // the hover names both, and a click copies the
                        // destination.
                        let email = e.email.trim();
                        let address = e.address.trim();
                        if !email.is_empty() || !address.is_empty() {
                            let mut tip = String::new();
                            if !email.is_empty() {
                                tip.push_str(email);
                            }
                            if !address.is_empty() {
                                if !tip.is_empty() {
                                    tip.push('\n');
                                }
                                tip.push_str(address);
                            }
                            tip.push('\n');
                            tip.push_str(&tr!("swl-click-to-copy"));
                            let label =
                                if email.is_empty() { tr!("swl-addr-btn") } else { tr!("swl-mail-btn") };
                            if ui.small_button(label).on_hover_text(tip).clicked() {
                                let dest = if email.is_empty() { address } else { email };
                                ui.ctx().copy_text(dest.to_string());
                            }
                        }
                        // Where this reception sits in the SWL's loop: reported,
                        // or reported and verified. Blank once heard and left,
                        // but always the same width, so the buttons after it do
                        // not jitter from row to row.
                        let (mark, ink) = if e.qsl_received_unix.is_some() {
                            ("QSL".to_string(), crate::theme::GREEN())
                        } else if e.report_sent_unix.is_some() {
                            (tr!("swl-mark-sent"), crate::theme::gray(150))
                        } else {
                            (String::new(), crate::theme::gray(150))
                        };
                        ui.add_sized(
                            [26.0, 14.0],
                            egui::Label::new(RichText::new(mark).size(10.5).color(ink)),
                        );
                        if e.freq_hz > 0.0
                            && ui
                                .small_button(tr!("swl-recall"))
                                .on_hover_text(tr!("swl-recall-tip"))
                                .clicked()
                        {
                            recall = Some((e.freq_hz, e.mode));
                        }
                        if ui.small_button(tr!("swl-edit")).clicked() {
                            edit = Some(e.id);
                        }
                        if ui.small_button(tr!("swl-del")).clicked() {
                            delete = Some(e.id);
                        }
                    });
                }
                if !rows.is_empty() && shown == 0 {
                    ui.label(
                        RichText::new(tr!("swl-no-match"))
                            .color(crate::theme::gray(150)),
                    );
                }
            },
        );
        self.swl_selected = selected;
        if let Some((hz, mode)) = recall {
            // The dial, then the mode — through `SetModeListen`, so a logged
            // reception always comes back as it was logged: the band rule that
            // `SetMode` enforces is about what may be *operated*, and a
            // listener recalling a catch is not choosing a mode to transmit on.
            cmds.push(Command::SetVfo { vfo: Vfo::A, hz });
            cmds.push(Command::SetModeListen { rx: RxId::Main, mode });
        }
        if let Some(id) = edit
            && let Some(e) = rows.iter().find(|e| e.id == id)
        {
            self.swl_edit = Some(SwlEditForm::from_entry(e));
        }
        let deleted = delete.is_some();
        if let Some(id) = delete {
            rows.retain(|e| e.id != id);
            if self.swl_selected == Some(id) {
                self.swl_selected = None;
            }
        }
        self.swl_log = rows;
        if deleted {
            persist_swl_log(&self.swl_log);
        }
    }

    /// The reception entry form.
    fn swl_entry_form(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let mut save = false;
        let mut cancel = false;
        let mut mail_report = false;
        let mut heard_now = false;
        {
            let f = self.swl_edit.as_mut().unwrap();
            egui::Frame::new()
                .fill(crate::theme::ROW_BG())
                .stroke(egui::Stroke::new(1.0, crate::theme::RED_DEEP()))
                .inner_margin(egui::Margin::same(9))
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.label(
                        RichText::new(if f.id == 0 { tr!("swl-form-new") } else { tr!("swl-form-edit") })
                            .size(11.0)
                            .strong()
                            .color(crate::theme::CYAN()),
                    );
                    ui.add_space(4.0);
                    egui::Grid::new("swl-form").num_columns(4).spacing([10.0, 6.0]).show(
                        ui,
                        |ui| {
                            ui.label(tr!("settings-station"));
                            crate::chrome::field(
                                ui,
                                egui::TextEdit::singleline(&mut f.station)
                                    .desired_width(220.0)
                                    .hint_text(tr!("swl-station-hint")),
                            );
                            ui.label(tr!("settings-ui-language"));
                            crate::chrome::field(
                                ui,
                                egui::TextEdit::singleline(&mut f.language)
                                    .desired_width(120.0)
                                    .hint_text(tr!("swl-language-hint")),
                            );
                            ui.end_row();

                            ui.label(tr!("settings-frequency"));
                            ui.horizontal(|ui| {
                                crate::chrome::field(
                                    ui,
                                    egui::TextEdit::singleline(&mut f.freq_khz).desired_width(84.0),
                                );
                                ui.label("kHz");
                                egui::ComboBox::from_id_salt("swl-mode")
                                    .width(84.0)
                                    .selected_text(f.mode.label())
                                    .show_ui(ui, |ui| {
                                        for m in SWL_MODES {
                                            ui.selectable_value(&mut f.mode, m, m.label());
                                        }
                                    });
                            });
                            // The transmitter site is deliberately not a field:
                            // a listener almost never knows it, and the one
                            // place it comes from is the schedule, which fills
                            // it in on LOG. It rides along on the saved entry
                            // and shows on the row's hover when it is known.
                            ui.label(tr!("swl-received"));
                            crate::chrome::field(
                                ui,
                                egui::TextEdit::singleline(&mut f.recv_grid)
                                    .desired_width(90.0)
                                    .hint_text(tr!("swl-received-hint")),
                            );
                            ui.end_row();

                            // The broadcaster's report contact, filled in from
                            // the schedule on LOG. Editable, because a schedule
                            // that has no address is no reason the listener
                            // cannot write one in.
                            ui.label(tr!("swl-email"));
                            crate::chrome::field(
                                ui,
                                egui::TextEdit::singleline(&mut f.email)
                                    .desired_width(220.0)
                                    .hint_text(tr!("swl-email-hint")),
                            )
                            .on_hover_text(tr!("swl-email-tip"));
                            ui.label(tr!("swl-postal"));
                            crate::chrome::field(
                                ui,
                                egui::TextEdit::singleline(&mut f.address)
                                    .desired_width(220.0)
                                    .hint_text(tr!("swl-postal-hint")),
                            )
                            .on_hover_text(tr!("swl-postal-tip"));
                            ui.end_row();

                            ui.label(tr!("swl-heard"));
                            ui.horizontal(|ui| {
                                ui.label(RichText::new(utc_text(f.heard_at)).monospace());
                                if crate::chrome::chip(ui, false, tr!("schedule-now-chip")).clicked() {
                                    heard_now = true;
                                }
                            });
                            ui.label(tr!("swl-smeter"));
                            ui.label(match f.smeter_dbm {
                                Some(db) => format!("{db:.0} dBm"),
                                None => "—".into(),
                            });
                            ui.end_row();

                            ui.label(tr!("swl-report-label"));
                            ui.horizontal(|ui| {
                                let judged = crate::chrome::checkbox(ui, &mut f.judged, &tr!("swl-judged"));
                                if judged.clicked() && f.judged {
                                    // A ticked report starts filled in: the
                                    // strength is the one figure the meter can
                                    // give, and the rest start at 5 — a clean
                                    // signal — for the operator to adjust. The
                                    // point is not to judge for them but to
                                    // save retyping five figures per station.
                                    if let Some(db) = f.smeter_dbm {
                                        f.s = sinpo_strength(db);
                                    }
                                    f.i = 5;
                                    f.n = 5;
                                    f.p = 5;
                                    f.o = 5;
                                }
                                ui.add_enabled_ui(f.judged, |ui| {
                                    ui.selectable_value(&mut f.sinpo, true, "SINPO");
                                    ui.selectable_value(&mut f.sinpo, false, "SIO");
                                });
                            });
                            if f.judged {
                                ui.horizontal(|ui| {
                                    let fig = |ui: &mut egui::Ui, name: &str, v: &mut u8| {
                                        ui.label(name);
                                        ui.add(egui::DragValue::new(v).speed(0.1).range(1..=5u8));
                                    };
                                    fig(ui, "S", &mut f.s);
                                    fig(ui, "I", &mut f.i);
                                    if f.sinpo {
                                        fig(ui, "N", &mut f.n);
                                        fig(ui, "P", &mut f.p);
                                    }
                                    fig(ui, "O", &mut f.o);
                                });
                                ui.label("");
                                ui.label("");
                            } else {
                                ui.label("");
                            }
                            ui.end_row();

                            // The SWL's loop: *hear → report → await QSL*. Both
                            // steps are stamped now, easy to forget otherwise.
                            ui.label(tr!("swl-report-sent"));
                            ui.horizontal(|ui| {
                                let mut sent = f.report_sent_unix.is_some();
                                if crate::chrome::checkbox(ui, &mut sent, "").changed() {
                                    f.report_sent_unix = sent.then(|| now_unix().max(0) as u64);
                                }
                                ui.label(
                                    RichText::new(
                                        f.report_sent_unix
                                            .map(utc_text)
                                            .unwrap_or_else(|| tr!("swl-not-yet")),
                                    )
                                    .size(10.5)
                                    .color(crate::theme::gray(150)),
                                );
                            });
                            ui.label(tr!("swl-qsl-received"));
                            ui.horizontal(|ui| {
                                let mut got = f.qsl_received_unix.is_some();
                                if crate::chrome::checkbox(ui, &mut got, "").changed() {
                                    f.qsl_received_unix = got.then(|| now_unix().max(0) as u64);
                                }
                                ui.label(
                                    RichText::new(
                                        f.qsl_received_unix
                                            .map(utc_text)
                                            .unwrap_or_else(|| tr!("swl-awaiting")),
                                    )
                                    .size(10.5)
                                    .color(crate::theme::gray(150)),
                                );
                            });
                            ui.end_row();

                            ui.label(tr!("swl-notes"));
                            crate::chrome::field(
                                ui,
                                egui::TextEdit::singleline(&mut f.notes)
                                    .desired_width(320.0)
                                    .hint_text(tr!("swl-notes-hint")),
                            );
                            ui.label("");
                            crate::chrome::checkbox(ui, &mut f.pirate, &tr!("swl-pirate"))
                                .on_hover_text(tr!("swl-pirate-tip"));
                            ui.end_row();
                        },
                    );
                    ui.add_space(6.0);
                    // ── Mail the reception report ──
                    // Greyed, with the reason, where it cannot work — which is
                    // the house rule rather than a refusal the operator has to
                    // discover. Three ways it cannot: no address on the form
                    // (which is the *unknown station* case), an address that is
                    // not one address, or no reception to report.
                    let to = f.email.trim().to_string();
                    let usable = report_address_is_usable(&to);
                    let why = match (to.is_empty(), usable) {
                        (true, _) => {
                            tr!("swl-mail-why-no-address")
                        }
                        (false, false) => {
                            tr!("swl-mail-why-bad-address")
                        }
                        (false, true) => {
                            tr!("swl-mail-why-ok")
                        }
                    };
                    let report = crate::chrome::chip(ui, usable, tr!("swl-mail-report")).on_hover_text(why);
                    if usable && report.clicked() {
                        mail_report = true;
                    }
                    ui.horizontal(|ui| {
                        if crate::chrome::chip(ui, true, tr!("recjobs-save")).clicked() {
                            save = true;
                        }
                        if crate::chrome::chip(ui, false, tr!("logbook-cancel")).clicked() {
                            cancel = true;
                        }
                    });
                });
        }
        if heard_now {
            let s = self.meters.map(|m| m.s_dbm);
            if let Some(f) = self.swl_edit.as_mut() {
                f.heard_at = now_unix().max(0) as u64;
                f.smeter_dbm = s;
            }
        }
        // **The report is built from the form as it stands, not from a saved
        // entry.** A listener who has typed a better SINPO and then mailed it
        // without pressing SAVE expects the report to carry what they typed —
        // and `to_entry()` is exactly the same conversion SAVE uses, so the two
        // cannot drift.
        // The form is converted and taken first, because building the report
        // needs `&mut self` (the station's own callsign and the configured
        // picture) while the form is only borrowed.
        if mail_report
            && let Some(f) = self.swl_edit.as_ref().map(|f| (f.to_entry(), f.email.clone()))
        {
            self.mail_reception_report(&f.0, &f.1, ctx);
        }
        if save && let Some(f) = self.swl_edit.take() {
            let mut entry = f.to_entry();
            if entry.id == 0 {
                entry.id = self.swl_log.iter().map(|e| e.id).max().unwrap_or(0) + 1;
                self.swl_log.push(entry);
            } else if let Some(slot) = self.swl_log.iter_mut().find(|e| e.id == entry.id) {
                *slot = entry;
            }
            persist_swl_log(&self.swl_log);
        }
        if cancel {
            self.swl_edit = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{SwlEditForm, SwlEntry, SwlFilter, prefill_station, sinpo_strength, station_at};
    use sdroxide_types::{Mode, SignalReport, Sinpo};

    /// The meter grades into the five SINPO figures, strongest at S9 and up.
    #[test]
    fn the_meter_grades_into_sinpo_strength() {
        assert_eq!(sinpo_strength(-60.0), 5, "S9+10 is a 5");
        assert_eq!(sinpo_strength(-73.0), 5, "S9 is a 5");
        assert_eq!(sinpo_strength(-76.0), 4);
        assert_eq!(sinpo_strength(-86.0), 3);
        assert_eq!(sinpo_strength(-96.0), 2);
        assert_eq!(sinpo_strength(-120.0), 1, "the noise floor is a 1");
    }

    /// The pirate tick travels from the form into the record and back into the
    /// form when the entry is edited again — the flag is a property of the
    /// reception, not of the session that logged it.
    #[test]
    fn the_pirate_tick_survives_the_form_and_an_edit() {
        let mut f = SwlEditForm::new(6_185_000.0, Mode::Am, None, "JO22".into(), String::new());
        assert!(!f.to_entry().pirate, "off unless the listener ticks it");
        f.pirate = true;
        let e = f.to_entry();
        assert!(e.pirate);
        assert!(SwlEditForm::from_entry(&e).pirate, "reopens ticked");
    }

    /// Logging a second reception on a frequency the log already names comes in
    /// with that name pre-filled, so a regular broadcast is not retyped. It is
    /// the most recent name at that kilohertz, a nameless entry is no
    /// suggestion, and a frequency the log has never had returns nothing.
    #[test]
    fn a_logged_frequency_pre_fills_the_station_name() {
        let entry = |id: u64, station: &str, freq_hz: f64, heard: u64| SwlEntry {
            id,
            station: station.into(),
            freq_hz,
            heard_at_unix: heard,
            ..Default::default()
        };
        let log = vec![
            entry(1, "Radio Taiwan International", 6_185_000.0, 100),
            // Same kilohertz, heard later, so it is the one suggested.
            entry(2, "BBC World Service", 6_185_040.0, 200),
            // Nameless entries are no help.
            entry(3, "", 6_185_000.0, 300),
            // A different frequency.
            entry(4, "Voice of America", 9_760_000.0, 400),
        ];
        assert_eq!(station_at(&log, 6_185_000.0).as_deref(), Some("BBC World Service"));
        // A dial a few tens of hertz off the remembered one is the same channel.
        assert_eq!(station_at(&log, 6_184_960.0).as_deref(), Some("BBC World Service"));
        // Never logged here: nothing to suggest.
        assert_eq!(station_at(&log, 7_200_000.0), None);
    }

    /// Tuning a station the schedule knows pre-fills what a reception report
    /// needs — the language, the transmitter site and the broadcaster's report
    /// contact — and where the schedule has nothing, the log's own name for the
    /// channel is the fallback.
    #[test]
    fn the_dial_pre_fills_from_the_schedule_then_the_log() {
        use sdroxide_types::BroadcastStation;
        let broadcast = vec![BroadcastStation {
            name: "Radio Taiwan International".into(),
            freq_khz: 6_185.0,
            lang: "Chinese".into(),
            site: "Tamsui".into(),
            email: "rti@example.org".into(),
            address: "P.O. Box 123, Taipei".into(),
            ..Default::default()
        }];
        let mut f = SwlEditForm::new(6_185_000.0, Mode::Am, None, "JO22".into(), String::new());
        prefill_station(&mut f, &broadcast, &[], 6_185_000.0, 0);
        assert_eq!(f.station, "Radio Taiwan International");
        assert_eq!(f.language, "Chinese");
        assert_eq!(f.site, "Tamsui");
        assert_eq!(f.email, "rti@example.org");
        assert_eq!(f.address, "P.O. Box 123, Taipei");

        // Nothing in the schedule on 9.76 MHz: the log's remembered name, and
        // the schedule-only fields stay empty.
        let log = vec![SwlEntry {
            id: 1,
            station: "Voice of America".into(),
            freq_hz: 9_760_000.0,
            heard_at_unix: 100,
            ..Default::default()
        }];
        let mut f = SwlEditForm::new(9_760_000.0, Mode::Am, None, "JO22".into(), String::new());
        prefill_station(&mut f, &broadcast, &log, 9_760_000.0, 0);
        assert_eq!(f.station, "Voice of America");
        assert!(f.language.is_empty() && f.email.is_empty());
    }

    /// The reception locator is pre-filled from the screen's grid and carried
    /// into the record, upper-cased — a locator is written in capitals, and a
    /// listener typing `jo22` means `JO22`.
    #[test]
    fn the_reception_grid_is_carried_and_upper_cased() {
        let f = SwlEditForm::new(6_185_000.0, Mode::Am, None, "jo22".into(), String::new());
        assert_eq!(f.to_entry().recv_grid, "JO22");
    }

    /// The session antenna — the aerial the operator typed in the SWL LOG window
    /// — is captured into the entry at log time, so a report of a later
    /// reception does not name the aerial that heard an earlier one.
    #[test]
    fn the_session_antenna_is_captured_into_the_entry() {
        let f = SwlEditForm::new(6_185_000.0, Mode::Am, None, "JO22".into(), "MLA-30".into());
        assert_eq!(f.to_entry().antenna, "MLA-30");
    }

    /// The log filter: the find box covers station, language, site and notes,
    /// and the band, day and pirate switch each narrow it. Case does not
    /// matter — a listener typing `dutch` means `Dutch`.
    #[test]
    fn the_log_filter_narrows_by_text_band_day_and_pirate() {
        let mut e = SwlEntry {
            heard_at_unix: 1_789_587_720,
            station: "Radio Taiwan International".into(),
            freq_hz: 6_185_000.0,
            language: "English".into(),
            report: Some(SignalReport::Sinpo(Sinpo { s: 4, i: 3, n: 3, p: 4, o: 4 })),
            site: "Tamsui".into(),
            notes: "News, then music".into(),
            pirate: true,
            ..Default::default()
        };
        let day = "2026-09-16";
        let all = SwlFilter::default();
        assert!(all.matches(&e, day) && !all.active());
        let find = |q: &str| SwlFilter { find: q.into(), ..Default::default() };
        assert!(find("taiwan").matches(&e, day));
        assert!(find("english").matches(&e, day));
        assert!(find("tamsui").matches(&e, day));
        assert!(find("MUSIC").matches(&e, day));
        assert!(!find("dutch").matches(&e, day));

        let here = sdroxide_types::Band::containing(e.freq_hz);
        assert!(
            SwlFilter { band: Some(here), ..Default::default() }.matches(&e, day),
            "the band it was heard in"
        );
        assert!(
            !SwlFilter {
                band: Some(sdroxide_types::Band::containing(14_200_000.0)),
                ..Default::default()
            }
            .matches(&e, day)
        );
        assert!(
            !SwlFilter { day: Some("2026-01-01".into()), ..Default::default() }.matches(&e, day)
        );

        e.pirate = false;
        assert!(
            !SwlFilter { pirates_only: true, ..Default::default() }.matches(&e, day),
            "a licensed station is not a pirate catch"
        );
        e.pirate = true;
        assert!(SwlFilter { pirates_only: true, ..Default::default() }.matches(&e, day));
    }

    /// The reporting dates — the loop's *report → QSL* — are off until stamped,
    /// survive the form and an edit, and clear back to `None` when unticked.
    #[test]
    fn the_sent_and_qsl_dates_survive_the_form_and_an_edit() {
        let mut f = SwlEditForm::new(6_185_000.0, Mode::Am, None, "JO22".into(), String::new());
        let fresh = f.to_entry();
        assert!(fresh.report_sent_unix.is_none() && fresh.qsl_received_unix.is_none());
        f.report_sent_unix = Some(1_789_588_000);
        f.qsl_received_unix = Some(1_790_000_000);
        let e = f.to_entry();
        assert_eq!(e.report_sent_unix, Some(1_789_588_000));
        assert_eq!(e.qsl_received_unix, Some(1_790_000_000));
        let reopened = SwlEditForm::from_entry(&e);
        assert_eq!(reopened.report_sent_unix, Some(1_789_588_000));
        assert_eq!(reopened.qsl_received_unix, Some(1_790_000_000));
        f.report_sent_unix = None;
        assert!(f.to_entry().report_sent_unix.is_none(), "unticking clears it");
    }
}

/// The reception report a listener mails to a broadcaster.
///
/// **Plain text, and only what the listener actually entered.** A report is read
/// by a station engineer deciding whether to send a QSL card, and every line in
/// it is either something they will act on or something they will distrust. A
/// line reading `Grid: —` or `Antenna: unknown` is worse than the line's
/// absence, so **an empty field is left out entirely** rather than blanked or
/// filled with a dash.
///
/// This is the whole feature and it is deliberately not clever: it composes text,
/// it does not send anything, and the operator reads it before it goes.
pub(in crate::app) fn reception_report_body(
    entry: &SwlEntry,
    listener: &str,
    standing_note: &str,
) -> String {
    let mut out: Vec<String> = Vec::new();
    let station = entry.station.trim();
    if !station.is_empty() {
        out.push(format!("Station: {station}"));
    }
    out.push(format!("Date/Time (UTC): {}", utc_text(entry.heard_at_unix.max(0) as u64)));
    if entry.freq_hz > 0.0 {
        // Whole kilohertz, as the log stores it — a broadcast is not known to
        // better than that, and a report quoting more decimals than the log
        // holds would be a number the listener cannot later reproduce.
        out.push(format!("Frequency: {:.1} kHz", entry.freq_hz / 1e3));
    }
    out.push(format!("Mode: {}", entry.mode.label()));
    let report: Option<String> = entry.report.map(|r| format!("{} {}", r.label(), r.digits()));
    let lang = entry.language.trim();
    let site = entry.site.trim();
    let grid = entry.recv_grid.trim();
    let ant = entry.antenna.trim();
    // The four the station asks for by name, in the order a printed form asks
    // them: what it was, how it sounded, where it was heard, on what.
    match (report.clone(), lang.is_empty(), site.is_empty(), grid.is_empty(), ant.is_empty()) {
        (Some(r), false, false, false, false) => out.push(format!(
            "Reception: {r} · Language: {lang} · Site: {site} · Grid: {grid} · Antenna: {ant}"
        )),
        _ => {
            if let Some(r) = report {
                out.push(format!("Reception: {r}"));
            }
            if !lang.is_empty() {
                out.push(format!("Language: {lang}"));
            }
            if !site.is_empty() {
                out.push(format!("Site: {site}"));
            }
            if !grid.is_empty() {
                out.push(format!("Receiver location: {grid}"));
            }
            if !ant.is_empty() {
                out.push(format!("Antenna: {ant}"));
            }
        }
    }
    if !listener.trim().is_empty() {
        out.push(format!("Received by: {}", listener.trim()));
    }
    if entry.pirate {
        // Said plainly rather than as an accusation in parentheses: a listener
        // reporting an unlicensed transmission is telling the station something
        // useful, and the station decides what it means.
        out.push("This transmission appears to be unlicensed.".to_string());
    }
    if let Some(db) = entry.smeter_dbm {
        out.push(format!("S-meter: {} dBm", db.round() as i32));
    }
    if !entry.notes.trim().is_empty() {
        out.push(format!("Notes: {}", entry.notes.trim()));
    }
    if !standing_note.trim().is_empty() {
        out.push(String::new());
        out.push(standing_note.trim().to_string());
    }
    out.join("\n")
}

/// The subject line for a report: the station and when it was heard.
///
/// No report count and no call sign, deliberately — this goes to a broadcast
/// engineer, not to a contest logger, and the date is what they file it under.
pub(in crate::app) fn reception_report_subject(entry: &SwlEntry) -> String {
    let station = entry.station.trim();
    let (y, mo, d) = {
        let (y, mo, d, _h, _mi, _s) = sdroxide_types::utc_ymd_hms(entry.heard_at_unix as i64);
        (y, mo, d)
    };
    match station.is_empty() {
        true => format!("Reception report — {y:04}-{mo:02}-{d:02}"),
        false => format!("Reception report — {station} — {y:04}-{mo:02}-{d:02}"),
    }
}

/// The `mailto:` for a reception report.
///
/// The address is **not** put in the link by default: an empty recipient is a
/// link that cannot address anyone, which is safe, where a wrong one is a report
/// delivered to a stranger. The caller decides whether to include it, and this
/// returns `None` when it is not one this can vouch for — see
/// [`report_address_is_usable`].
pub(in crate::app) fn reception_report_mailto(
    to: &str,
    subject: &str,
    body: &str,
) -> Option<String> {
    if !report_address_is_usable(to) {
        return None;
    }
    Some(format!(
        "mailto:{}?subject={}&body={}",
        mailto_encode(to),
        mailto_encode(subject),
        mailto_encode(body)
    ))
}

/// Whether `to` is an address this will put in a link.
///
/// Deliberately strict, because the cost of being wrong is a reception report
/// delivered to somebody who never heard the station: a space (two addresses
/// joined), a missing `@`, a leading `@`, or a comma (a list, which `mailto:`
/// would treat as one recipient). Everything else is left alone — an address
/// this rejects is an address a listener cannot correct from here.
pub(in crate::app) fn report_address_is_usable(to: &str) -> bool {
    let t = to.trim();
    !t.is_empty()
        && t.contains('@')
        && !t.contains(char::is_whitespace)
        && !t.contains(',')
        && !t.starts_with('@')
        && !t.ends_with('@')
        && !t.contains("..")
}

/// Percent-encode for a `mailto:` field.
///
/// **Every** part of a report is operator-typed and a report is full of the
/// characters that break the scheme: `&` separates the header fields, so a
/// reception report reading "SINPO 4 3 3 4 4, best 4 4 3 4 4 & thanks" would
/// silently truncate and put the remainder in the subject line. A newline would
/// end a header outright.
pub(in crate::app) fn mailto_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.as_bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*b as char)
            }
            other => out.push_str(&format!("%{other:02X}")),
        }
    }
    out
}

#[cfg(test)]
mod report_tests {
    use super::*;

    /// A complete reception, the case a station engineer would actually receive.
    fn full() -> SwlEntry {
        SwlEntry {
            id: 1,
            heard_at_unix: 1_790_622_960, // 2026-09-28 19:16 UTC
            station: "Radio Example".into(),
            lat: Some(25.13),
            lon: Some(121.24),
            freq_hz: 3_730_000.0,
            mode: Mode::Am,
            language: "English".into(),
            report: Some(SignalReport::Sinpo(Sinpo { s: 4, i: 3, n: 3, p: 4, o: 4 })),
            smeter_dbm: Some(-62.0),
            site: "Tamsui".into(),
            email: "reports@example.org".into(),
            address: "P.O. Box 123".into(),
            recv_grid: "IO93WQ".into(),
            antenna: "80 m dipole".into(),
            notes: "Familiar interval signal.".into(),
            pirate: false,
            report_sent_unix: None,
            qsl_received_unix: None,
        }
    }

    /// The whole reception is in the report: a station decides whether to send a
    /// QSL card from exactly these facts, and any one of them missing is a
    /// reason not to reply.
    #[test]
    fn a_full_reception_reaches_the_station() {
        let body = reception_report_body(&full(), "A Listener", "");
        for want in [
            "Station: Radio Example",
            "Date/Time (UTC)",
            "Frequency: 3730.0 kHz",
            "Mode: AM",
            "SINPO 4 3 3 4 4",
            "English",
            "Tamsui",
            "IO93WQ",
            "80 m dipole",
            "A Listener",
        ] {
            assert!(body.contains(want), "the report is missing {want:?}:\n{body}");
        }
        assert!(!body.contains("unlicensed"), "not a pirate");
    }

    /// **The house rule this is built to honour: an empty field is left out, not
    /// blanked.** A report reading `Grid: —` is a line the station has to
    /// interpret, and one reading `Grid:` with nothing after it is worse — it
    /// looks like the listener's grid was an empty string rather than unknown.
    #[test]
    fn an_unfilled_field_is_left_out_of_the_report_entirely() {
        let mut e = full();
        e.recv_grid = String::new();
        e.site = String::new();
        e.antenna = String::new();
        e.notes = String::new();
        let body = reception_report_body(&e, "A Listener", "");
        for absent in ["Grid", "Site", "Antenna", "Notes"] {
            assert!(
                !body.contains(absent),
                "{absent} was not filled and must not appear at all:\n{body}"
            );
        }
        // The lines that *were* filled are all still there.
        assert!(body.contains("SINPO 4 3 3 4 4"), "{body}");
        assert!(body.contains("Tamsui".replace("Tamsui", "English").as_str()), "{body}");
    }

    /// A pirate is reported as a fact, not as an accusation the report softens.
    #[test]
    fn an_unlicensed_transmission_is_reported_plainly() {
        let mut e = full();
        e.pirate = true;
        let body = reception_report_body(&e, "", "");
        assert!(body.contains("unlicensed"), "{body}");
    }

    /// The S-meter is quoted because a station asks how strong it was, and it is
    /// rounded: the log holds a float from a meter that reads in whole units.
    #[test]
    fn the_s_meter_reads_as_a_whole_number() {
        let mut e = full();
        e.smeter_dbm = Some(-61.6);
        assert!(reception_report_body(&e, "", "").contains("S-meter: -62 dBm"));
        e.smeter_dbm = None;
        assert!(!reception_report_body(&e, "", "").contains("S-meter"));
    }

    /// The listener's standing note goes on the end, after a blank line, so the
    /// reception itself reads as a block.
    #[test]
    fn the_listener_note_sits_after_the_reception() {
        let body = reception_report_body(&full(), "A Listener", "Details on request.");
        let reception_at = body.find("Antenna:").expect("the reception is there");
        let note_at = body.find("Details on request.").expect("the note is there");
        assert!(note_at > reception_at, "the note comes last:\n{body}");
        assert!(body.contains("\n\nDetails"), "separated by a blank line:\n{body}");
    }

    /// An empty note changes nothing about the reception block — it must not
    /// leave a stray blank line where it would have been.
    #[test]
    fn no_note_leaves_no_gap() {
        let body = reception_report_body(&full(), "A Listener", "   ");
        assert!(!body.trim_end().ends_with("\n"), "{body:?}");
        assert!(!body.contains("\n\n"), "no double blank line:\n{body}");
    }

    /// **The reason this exists.** A reception report is full of `&`, commas and
    /// newlines — "SINPO 4 3 3 4 4, best 4 4 3 4 4 & thanks" is what a listener
    /// writes. Unencoded, `&` ends the header field and the rest of the report
    /// lands in the subject line, so the station receives a truncated report and
    /// never knows.
    #[test]
    fn an_ampersand_in_a_report_does_not_truncate_it() {
        let mut e = full();
        e.notes = "Best on 3955 kHz & again after 2100".into();
        let link = reception_report_mailto(
            "reports@example.org",
            "Reception report",
            &reception_report_body(&e, "", ""),
        )
        .expect("an address");
        assert_eq!(link.matches("subject=").count(), 1, "the separator leaked: {link}");
        assert_eq!(link.matches("body=").count(), 1, "{link}");
        assert!(link.contains("%26"), "the ampersand is encoded: {link}");
        // The `&` between `subject=` and `body=` is the scheme's own separator
        // and is the only bare one allowed — so this checks the body *alone*,
        // which is where an unencoded ampersand in the report would show up.
        let body_part = link.split_once("&body=").expect("a body field").1;
        assert!(
            !body_part.contains('&'),
            "no bare ampersand survives inside the body: {body_part}"
        );
        assert!(
            body_part.contains("3955%20kHz%20%26%20again"),
            "the note arrives whole: {body_part}"
        );
    }

    /// A newline would end a header line outright, and a report is written in
    /// lines — so this is not an exotic input, it is every multi-line report.
    #[test]
    fn a_multiline_report_stays_one_body() {
        let body = reception_report_body(&full(), "A Listener", "");
        assert!(body.contains('\n'), "a report is several lines");
        let enc = mailto_encode(&body);
        assert!(!enc.contains('\n') && !enc.contains('\r'), "newlines are encoded");
        assert!(enc.contains("%0A"), "encoded as %0A, not stripped");
    }

    /// **A wrong recipient is worse than none.** The cost of guessing here is a
    /// reception report delivered to a stranger who never heard the station, so
    /// anything that is not plainly one address is refused.
    #[test]
    fn an_address_that_is_not_one_address_is_refused() {
        for bad in [
            "",
            "   ",
            "reports",
            "reports@",
            "@example.org",
            "a@b.example, c@d.example",
            "two words@example.org",
            "a@b..example.org",
            "a b",
        ] {
            assert!(!report_address_is_usable(bad), "{bad:?} must not become a recipient");
            assert!(
                reception_report_mailto(bad, "s", "b").is_none(),
                "{bad:?} must not produce a link"
            );
        }
        // And the ones that are fine still work.
        for good in ["reports@example.org", "rti@bbc.co.uk", "a.b+c@sub.example.org"] {
            assert!(report_address_is_usable(good), "{good:?} is a valid address");
            assert!(reception_report_mailto(good, "s", "b").is_some());
        }
    }

    /// The subject names the station and the date, and nothing else — no report
    /// count, no call sign. It goes to a broadcast engineer filing it by date.
    #[test]
    fn the_subject_names_the_station_and_the_date() {
        let subject = reception_report_subject(&full());
        assert!(subject.starts_with("Reception report — Radio Example — "), "{subject}");
        assert!(subject.contains("2026-09-28"), "{subject}");
        // An un-named station still gets a usable subject.
        let mut e = full();
        e.station = String::new();
        assert!(reception_report_subject(&e).starts_with("Reception report — 2026-"));
    }

    /// The report is generated from a **saved** entry, so what went out and what
    /// is in the log cannot disagree.
    #[test]
    fn the_report_is_built_from_the_entry_that_was_logged() {
        let form = SwlEditForm {
            station: "Radio Example".into(),
            freq_khz: "3730".into(),
            judged: true,
            sinpo: true,
            s: 4,
            i: 3,
            n: 3,
            p: 4,
            o: 4,
            site: "Tamsui".into(),
            recv_grid: "io93wq".into(),
            antenna: "80 m dipole".into(),
            ..Default::default()
        };
        let entry = form.to_entry();
        assert_eq!(entry.freq_hz, 3_730_000.0, "kHz became Hz");
        let body = reception_report_body(&entry, "", "");
        assert!(body.contains("Radio Example"), "{body}");
        assert!(body.contains("3730.0 kHz"), "{body}");
        assert!(body.contains("SINPO 4 3 3 4 4"), "{body}");
        assert!(body.contains("IO93WQ"), "the grid is filed in capitals: {body}");
    }

    /// An unjudged reception says nothing about quality — the listener declined
    /// to grade it, and inventing a default grade would put five figures in a
    /// station's records that nobody ever heard.
    #[test]
    fn an_unjudged_reception_states_no_figures() {
        let form = SwlEditForm { station: "X".into(), judged: false, ..Default::default() };
        let entry = form.to_entry();
        assert!(entry.report.is_none(), "not judged means no report");
        let body = reception_report_body(&entry, "", "");
        assert!(!body.contains("SINPO") && !body.contains("SIO"), "{body}");
    }
}
