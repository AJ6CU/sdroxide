//! The broadcast schedule window: what is on, when, where — and a way to tune
//! to it and log it.
//!
//! The EiBi table is already in the program (it labels the waterfall); this
//! turns it into the list a listener works from. Filter by time, band, language
//! and a free-text query, tune a row, or open a reception with the station
//! filled in.

use crate::i18n::tr;
use eframe::egui::{self, RichText};
use sdroxide_types::{
    BroadcastStation, Command, ProgramsFilters, ProgramsSort, RxId, ScheduleSource, Vfo, broadcast,
};

use crate::app::SdroxideApp;
use crate::app::swl_log::SwlEditForm;
use crate::time::now_unix;

/// The SCHEDULE window's default width.
pub(in crate::app) const SCHEDULE_WINDOW_W: f32 = 1000.0;

/// The narrowest the window goes. **The top bar is two rows and this is the
/// budget they are drawn to**: each row's widgets add up to less than this in
/// every language (see the `bar_budget` test), so the chips never stack on one
/// another or wrap onto a third row. Below it the window simply stops shrinking.
pub(in crate::app) const SCHEDULE_WINDOW_MIN_W: f32 = 860.0;

/// Widths of the three text fields on the first bar row.
const FIND_W: f32 = 100.0;
const LANG_W: f32 = 80.0;
const TARGET_W: f32 = 80.0;
/// Widths of the two comboboxes on the second bar row.
const BAND_W: f32 = 72.0;
const SOURCE_W: f32 = 96.0;
/// The gap between two columns of the list.
const GAP: f32 = 8.0;

/// The sources offered in the source filter, in the order the store ranks them.
const SOURCE_FILTERS: &[(ScheduleSource, &str)] = &[
    (ScheduleSource::Eibi, "EiBi"),
    (ScheduleSource::Aoki, "Aoki"),
    (ScheduleSource::Hfcc, "HFCC"),
    (ScheduleSource::User, "mine"),
];

/// The filter's own label, which must say **all of them** rather than any source's
/// name — the default is the store, and a combobox reading "EiBi" when three
/// schedules are loaded would be the control quietly lying about its state.
fn source_label(source: Option<ScheduleSource>) -> String {
    match source {
        None => tr!("schedule-all-of-them"),
        Some(s) => SOURCE_FILTERS
            .iter()
            .find(|(x, _)| *x == s)
            .map(|(x, n)| source_name(*x, n))
            .unwrap_or_else(|| format!("{s:?}")),
    }
}

/// A source's name as drawn: the schedules are proper names, but "mine" is a word.
fn source_name(source: ScheduleSource, name: &str) -> String {
    if source == ScheduleSource::User { tr!("schedule-source-mine") } else { name.to_string() }
}

/// The metre bands offered in the band filter. Empty = any.
const BAND_FILTERS: [&str; 20] = [
    "", "LW", "MW", "120m", "90m", "75m", "60m", "49m", "41m", "31m", "25m", "22m", "19m", "16m",
    "15m", "13m", "11m", "FM", "AIR", "MIL",
];

/// The schedule window's state: whether it is open, and its filters.
pub(in crate::app) struct ScheduleUi {
    pub show: bool,
    pub query: String,
    pub lang: String,
    pub target: String,
    /// Empty = any; otherwise a metre band name from [`BAND_FILTERS`].
    pub band: String,
    /// The filter time as UTC `HHMM`, ignored while `use_now`.
    pub hhmm: u16,
    pub use_now: bool,
    /// Show only the listener's favourite stations.
    pub favourites_only: bool,
    /// Which schedule to show. `None` is **all of them**, which is the default:
    /// the master store holds every downloaded source and the repetition is
    /// accepted, because this window is where the detail belongs.
    pub source: Option<ScheduleSource>,
    /// Show each row's local time at the transmitter (mean solar time, from the
    /// site's longitude). Off by default: most listeners work in UTC, and the
    /// column is only as good as the site coordinate.
    pub local_time: bool,
    /// The column the list is ordered by, and whether it is read backwards.
    pub sort: ProgramsSort,
    pub sort_desc: bool,
    /// A filter changed and has not been written to the config yet. It waits for
    /// the keyboard to be free so typing does not rewrite the file per letter.
    unsaved: bool,
    /// The filtered list, and the key it was built for. Rebuilt only when the
    /// filters, the favourites, the loaded schedule or the UTC minute change —
    /// not sixty times a second over four and a half thousand rows.
    cache_key: Option<CacheKey>,
    cache: Vec<BroadcastStation>,
    /// The width the list needs in the language on screen, measured by the last
    /// frame. The window never goes narrower than this, so a long language grows
    /// the window instead of pushing the right-hand chips out of it.
    floor_w: f32,
    /// What the last import said, shown under the bar until dismissed. A message,
    /// not a setting: nothing here is stored or sent.
    import_note: Option<Result<String, String>>,
}

type CacheKey =
    (String, String, String, String, bool, usize, i64, Option<ScheduleSource>, ProgramsSort, bool);

impl Default for ScheduleUi {
    fn default() -> Self {
        ScheduleUi {
            show: false,
            query: String::new(),
            lang: String::new(),
            target: String::new(),
            band: String::new(),
            hhmm: 0,
            use_now: true,
            favourites_only: false,
            source: None,
            local_time: false,
            sort: ProgramsSort::Freq,
            sort_desc: false,
            unsaved: false,
            cache_key: None,
            cache: Vec::new(),
            floor_w: 0.0,
            import_note: None,
        }
    }
}

impl ScheduleUi {
    /// The window as it was left, from the saved filters.
    pub(in crate::app) fn from_settings(f: &ProgramsFilters) -> Self {
        ScheduleUi {
            query: f.query.clone(),
            lang: f.lang.clone(),
            target: f.target.clone(),
            band: f.band.clone(),
            hhmm: f.hhmm,
            use_now: f.use_now,
            favourites_only: f.favourites_only,
            source: f.source,
            local_time: f.local_time,
            sort: f.sort,
            sort_desc: f.sort_desc,
            ..Default::default()
        }
    }

    /// Take the filters the station sent, leaving the window itself (open or
    /// not, where it sits) as it is. The list is rebuilt next frame.
    pub(in crate::app) fn apply_filters(&mut self, f: &ProgramsFilters) {
        self.query = f.query.clone();
        self.lang = f.lang.clone();
        self.target = f.target.clone();
        self.band = f.band.clone();
        self.hhmm = f.hhmm;
        self.use_now = f.use_now;
        self.favourites_only = f.favourites_only;
        self.source = f.source;
        self.local_time = f.local_time;
        self.sort = f.sort;
        self.sort_desc = f.sort_desc;
        self.cache_key = None;
    }

    /// The filters to save. `all` is the ALL chip, which lives on the app.
    fn to_settings(&self, all: bool) -> ProgramsFilters {
        ProgramsFilters {
            query: self.query.clone(),
            lang: self.lang.clone(),
            target: self.target.clone(),
            band: self.band.clone(),
            hhmm: self.hhmm,
            use_now: self.use_now,
            favourites_only: self.favourites_only,
            source: self.source,
            local_time: self.local_time,
            all,
            sort: self.sort,
            sort_desc: self.sort_desc,
        }
    }
}

/// Order the list by `col`, the other way round when `desc`. Ties fall back to
/// frequency (always ascending), so equal names or bands stay in dial order.
fn sort_rows(rows: &mut [BroadcastStation], col: ProgramsSort, desc: bool) {
    use std::cmp::Ordering;
    fn by_freq(a: &BroadcastStation, b: &BroadcastStation) -> Ordering {
        a.freq_khz.partial_cmp(&b.freq_khz).unwrap_or(Ordering::Equal)
    }
    fn band_rank(khz: f64) -> usize {
        broadcast::metre_band(khz)
            .and_then(|b| BAND_FILTERS.iter().position(|x| *x == b))
            .unwrap_or(usize::MAX)
    }
    rows.sort_by(|a, b| {
        let primary = match col {
            ProgramsSort::Freq => by_freq(a, b),
            ProgramsSort::Band => band_rank(a.freq_khz).cmp(&band_rank(b.freq_khz)),
            ProgramsSort::Name => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
            ProgramsSort::Lang => a.lang.to_lowercase().cmp(&b.lang.to_lowercase()),
            ProgramsSort::Target => a.target.to_lowercase().cmp(&b.target.to_lowercase()),
            ProgramsSort::Site => a.site.to_lowercase().cmp(&b.site.to_lowercase()),
        };
        (if desc { primary.reverse() } else { primary }).then_with(|| by_freq(a, b))
    });
}

/// The UTC instant the schedule is filtered at: now, or today at `hhmm`.
fn schedule_time(now: i64, ui: &ScheduleUi) -> i64 {
    if ui.use_now {
        return now;
    }
    let day = now.div_euclid(86_400) * 86_400;
    day + (ui.hhmm as i64 / 100) * 3600 + (ui.hhmm as i64 % 100) * 60
}

fn hhmm_text(hhmm: u16) -> String {
    format!("{:02}:{:02}", hhmm / 100, hhmm % 100)
}

impl SdroxideApp {
    /// The SCHEDULE window. Tuning and logging are done by the commands and the
    /// entry form it fills in, after the window has closed the borrow of the log
    /// — so the body carries the three asks back and the tail acts on them.
    pub(in crate::app) fn schedule_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let mut tune: Option<BroadcastStation> = None;
        let mut log: Option<BroadcastStation> = None;
        let mut fav_toggle: Option<(String, bool)> = None;
        let mut all_toggle: Option<bool> = None;
        if !self.schedule.show {
            self.schedule_was_open = false;
            // Closed with a filter still unwritten (the keyboard was busy).
            if self.schedule.unsaved {
                self.save_programs_filters(ctx);
            }
        } else {
            // Each opening gets its own window id, so the remembered geometry
            // is what applies — see `remembered_tool_window`.
            if !self.schedule_was_open {
                self.schedule_epoch += 1;
                self.schedule_was_open = true;
            }
            let epoch = self.schedule_epoch;
            self.schedule.show = self.remembered_tool_window(
                ctx,
                "schedule",
                &tr!("schedule-title"),
                [SCHEDULE_WINDOW_W, 560.0],
                [SCHEDULE_WINDOW_MIN_W.max(self.schedule.floor_w), 240.0],
                epoch,
                |me, ui| {
                    me.schedule_body(ui, &mut tune, &mut log, &mut fav_toggle, &mut all_toggle)
                },
            );
            self.save_programs_filters(ctx);
        }

        if let Some((name, on)) = fav_toggle {
            self.broadcast_favs.retain(|n| n != &name);
            if on {
                self.broadcast_favs.push(name);
                self.broadcast_favs.sort();
            }
            crate::app::persist::persist_broadcast_favourites(&self.broadcast_favs);
        }
        if let Some(on) = all_toggle {
            self.set_broadcast_all(on);
        }
        if let Some(s) = tune {
            self.tune_to_station(&s, cmds);
        }
        if let Some(s) = log {
            let smeter = self.meters.map(|m| m.s_dbm);
            let grid = self.my_grid();
            let antenna = self.swl_antenna.clone();
            self.swl_edit = Some(SwlEditForm::from_station(
                s.freq_hz(),
                s.mode(),
                &s.name,
                &s.lang,
                &s.site,
                &s.email,
                &s.address,
                s.lat,
                s.lon,
                smeter,
                grid,
                antenna,
            ));
            self.show_swl = true;
        }
    }

    /// Keep the PROGRAMS filters between sessions. A change is copied into the
    /// settings at once; it is written to the config when no text box is being
    /// typed in (or the window has closed), so a search is one write and not one
    /// per letter.
    fn save_programs_filters(&mut self, ctx: &egui::Context) {
        let now = self.schedule.to_settings(self.ui_settings.programs.all);
        if now != self.ui_settings.programs {
            self.ui_settings.programs = now;
            self.schedule.unsaved = true;
        }
        if self.schedule.unsaved && (ctx.memory(|m| m.focused()).is_none() || !self.schedule.show) {
            self.schedule.unsaved = false;
            crate::app::persist::persist_ui_settings(&self.ui_settings);
        }
    }

    /// Tunes the receiver to a PROGRAMS row.
    pub(in crate::app) fn tune_to_station(
        &mut self,
        s: &BroadcastStation,
        cmds: &mut Vec<Command>,
    ) {
        // The VFO the receiver is on, not always A: the engine ignores a
        // retune of the other one, and the dial would stay where it was.
        let vfo = self.state.active_vfo;
        let (hz, mode) = (s.freq_hz(), s.mode());
        cmds.push(Command::SetVfo { vfo, hz });
        cmds.push(Command::SetMode { rx: RxId::Main, mode });
        // Echoed now, as every other route that tunes from the screen does, so
        // the dial does not show the old frequency while the engine answers.
        self.state.rx[0].mode = mode;
        match vfo {
            Vfo::A => self.state.vfo_a_hz = hz,
            Vfo::B => self.state.vfo_b_hz = hz,
        }
    }

    /// The schedule's body: the filters, the on-air list and its row actions.
    /// Split out of [`Self::schedule_window`] so the shell can draw it in a
    /// window of its own; the three row asks go back through the parameters.
    fn schedule_body(
        &mut self,
        ui: &mut egui::Ui,
        tune: &mut Option<BroadcastStation>,
        log: &mut Option<BroadcastStation>,
        fav_toggle: &mut Option<(String, bool)>,
        all_toggle: &mut Option<bool>,
    ) {
        crate::chrome::window_body_bg(ui);
        let now = now_unix() as i64;
        let at = schedule_time(now, &self.schedule);

        // The matching rows, cloned out before the closure borrows `self` to
        // edit the filters and to open the log.
        // On-air status changes at the minute, so the cache keys on the minute,
        // not on the second.
        let key: CacheKey = (
            self.schedule.query.clone(),
            self.schedule.lang.clone(),
            self.schedule.target.clone(),
            self.schedule.band.clone(),
            self.schedule.favourites_only,
            self.broadcast.len(),
            at.div_euclid(60),
            self.schedule.source,
            self.schedule.sort,
            self.schedule.sort_desc,
        );
        let rows: Vec<BroadcastStation> = if self.schedule.cache_key.as_ref() == Some(&key) {
            // Lifted out for the frame and put back afterwards, so the closure
            // below is free to borrow `self` mutably.
            std::mem::take(&mut self.schedule.cache)
        } else {
            let f = &self.schedule;
            let mut v: Vec<BroadcastStation> = self
                .broadcast
                .iter()
                .filter(|s| {
                    s.on_air_at(at)
                        && f.source.is_none_or(|w| s.source == w)
                        && s.matches_query(&f.query)
                        && broadcast::contains_ascii_ci(&s.lang, &f.lang)
                        && broadcast::contains_ascii_ci(&s.target, &f.target)
                        && (f.band.is_empty()
                            || broadcast::metre_band(s.freq_khz) == Some(f.band.as_str()))
                        && (!f.favourites_only || self.broadcast_favs.iter().any(|n| n == &s.name))
                })
                .cloned()
                .collect();
            sort_rows(&mut v, f.sort, f.sort_desc);
            v
        };
        let count = rows.len();

        // **Two rows, no more.** The first is the time and the three text
        // searches; the second the two dropdowns, the chips and the clock. Each is
        // budgeted against `SCHEDULE_WINDOW_MIN_W` (see `bar_budget`), so the chips
        // do not climb over one another when the window is narrow. The first row
        // is not wrapped (its import button sits at the right edge); the second is,
        // as the safety net for a screen narrower than the window's floor.
        let peer = self.ctrl.peer_url();
        let mut imported: Option<Result<String, String>> = None;
        // Not wrapped: the window never goes narrower than this row needs (the
        // `bar_budget` test), and a wrapped row cannot push the button to the edge.
        ui.horizontal(|ui| {
            ui.label(tr!("schedule-at"));
            if crate::chrome::chip(ui, self.schedule.use_now, tr!("schedule-now-chip")).clicked() {
                self.schedule.use_now = true;
            }
            let mut hh = self.schedule.hhmm as i32;
            let r = ui.add_enabled(
                !self.schedule.use_now,
                egui::DragValue::new(&mut hh)
                    .speed(1.0)
                    .range(0..=2359)
                    .custom_formatter(|v, _| hhmm_text(v as u16)),
            );
            if r.changed() {
                self.schedule.hhmm = hh.clamp(0, 2359) as u16;
            }
            if ui.button(tr!("schedule-set-time")).clicked() {
                self.schedule.use_now = false;
            }
            ui.separator();
            ui.label(tr!("signalid-find"));
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut self.schedule.query)
                    .desired_width(FIND_W)
                    .hint_text(tr!("schedule-find-hint")),
            );
            ui.label(tr!("settings-ui-language"));
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut self.schedule.lang).desired_width(LANG_W),
            );
            ui.label(tr!("schedule-target"));
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut self.schedule.target).desired_width(TARGET_W),
            );
            // **Import, at the right-hand end of the row**: the schedule is read
            // here, so this is where an operator looks for it. Same button and same
            // code as Settings > Spots (any file: EiBi, Aoki, HFCC or an own list).
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if let Some(result) =
                    crate::app::settings::net::import_button(ui, peer.as_deref())
                {
                    imported = Some(result);
                }
            });
        });
        if let Some(result) = imported {
            self.schedule.import_note = Some(result);
            self.reload_broadcast_after_import();
        }
        if let Some(note) = self.schedule.import_note.clone() {
            ui.horizontal(|ui| {
                let (text, colour) = match &note {
                    Ok(t) => (t.clone(), crate::theme::GREEN()),
                    Err(e) => (format!("⚠ {e}"), crate::theme::YELLOW()),
                };
                ui.label(RichText::new(text).color(colour).size(11.0));
                if ui.small_button("✕").clicked() {
                    self.schedule.import_note = None;
                }
            });
        }
        ui.horizontal_wrapped(|ui| {
            ui.label(tr!("settings-relay-band"));
            egui::ComboBox::from_id_salt("sched-band")
                .width(BAND_W)
                .selected_text(if self.schedule.band.is_empty() {
                    tr!("schedule-any")
                } else {
                    self.schedule.band.clone()
                })
                .show_ui(ui, |ui| {
                    for b in BAND_FILTERS {
                        let label = if b.is_empty() { tr!("schedule-any") } else { b.to_string() };
                        ui.selectable_value(&mut self.schedule.band, b.to_string(), label);
                    }
                });
            ui.label(tr!("schedule-source"));
            egui::ComboBox::from_id_salt("sched-source")
                .width(SOURCE_W)
                .selected_text(source_label(self.schedule.source))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut self.schedule.source,
                        None,
                        tr!("schedule-all-of-them"),
                    )
                    .on_hover_text(tr!("schedule-all-hint"));
                    for (src, what) in SOURCE_FILTERS {
                        ui.selectable_value(
                            &mut self.schedule.source,
                            Some(*src),
                            source_name(*src, what),
                        );
                    }
                });
            if crate::chrome::chip(ui, self.schedule.favourites_only, tr!("schedule-favs"))
                .on_hover_text(tr!("schedule-favs-hint"))
                .clicked()
            {
                self.schedule.favourites_only = !self.schedule.favourites_only;
            }
            if crate::chrome::chip(ui, self.broadcast_all, tr!("schedule-all-favs"))
                .on_hover_text(tr!("schedule-all-favs-hint"))
                .clicked()
            {
                *all_toggle = Some(!self.broadcast_all);
            }
            if crate::chrome::chip(ui, self.schedule.local_time, tr!("schedule-solar-time"))
                .on_hover_text(tr!("schedule-solar-time-hint"))
                .clicked()
            {
                self.schedule.local_time = !self.schedule.local_time;
            }
            ui.separator();
            // **The way into LISTEN**, and deliberately here rather than as a
            // chip of its own on the top bar: the SWL row is already at the
            // two-row limit at 1400 pt, and a third chip pushed AM — the main
            // SWL mode — onto a third row. SCHEDULE is already in that row and is
            // the same data, so this costs no width and loses nothing.
            if crate::chrome::chip(ui, self.show_listen, tr!("schedule-map"))
                .on_hover_text(tr!("schedule-map-hint"))
                .clicked()
            {
                self.show_listen = !self.show_listen;
                if self.show_listen
                    && let Some(s) = sdroxide_types::broadcast::at_dial(
                        &self.broadcast,
                        self.state.rx_freq_hz(),
                        now_unix(),
                    )
                {
                    // Open it with **what is on the dial** already selected, which
                    // is the station the operator is listening to and the one they
                    // most often want to write down.
                    let name = s.name.clone();
                    self.listen_select(&name);
                }
            }
        });
        ui.add_space(2.0);
        // The count on the left; the clock and the build on the right. They were
        // on the chip row until that row ran out of width.
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(tr!(
                    "schedule-on-air",
                    count = count,
                    time = if self.schedule.use_now {
                        tr!("schedule-now")
                    } else {
                        hhmm_text(self.schedule.hhmm)
                    }
                ))
                .size(11.0)
                .color(crate::theme::gray(150)),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // The build, beside the clock — see the SWL LOG window.
                ui.label(
                    RichText::new(format!("v{}", sdroxide_version::VERSION))
                        .size(10.5)
                        .color(crate::theme::gray(120)),
                )
                .on_hover_text(sdroxide_version::LONG_VERSION);
                ui.label(
                    RichText::new(crate::time::utc_and_local_now(now))
                        .monospace()
                        .color(crate::theme::CYAN()),
                );
            });
        });
        ui.add_space(2.0);

        // The list's columns. The header and every row are cut to the same
        // widths, so a long name or a missing language no longer moves the rest
        // of its row, and the controls sit over the column they act on.
        // What the rows get is the width left once the scroll bar has its own: the
        // list is built for that, so its right-hand chip is not under the bar.
        let list_w = ui.available_width() - ui.spacing().scroll.allocated_width();
        // The name column is as wide as the longest name plus 40 characters.
        let body = egui::TextStyle::Body.resolve(ui.style());
        let name_w = rows
            .iter()
            .map(|s| &s.name)
            .max_by_key(|n| n.chars().count())
            .map_or(0.0, |n| crate::chrome::text_width(ui, n, body.clone()))
            + NAME_SLACK_CHARS as f32 * crate::chrome::text_width(ui, "0", body);
        let cols = Cols::new(ui, list_w, self.schedule.local_time, name_w);
        // What the list needs, window margins and scroll bar included, for the
        // window's floor next frame.
        self.schedule.floor_w =
            Cols::floor_width(ui, &ColLabels::current(), self.schedule.local_time)
                + ui.spacing().scroll.allocated_width()
                + 2.0 * 10.0;
        let row_h = crate::chrome::chip_height(ui, None);
        let x0 = ui.cursor().min.x;
        let names_top = self.schedule_header(ui, &cols, row_h);
        ui.separator();

        let local_time = self.schedule.local_time;
        let list = egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .id_salt("sched-list")
            .show_rows(ui, row_h, rows.len(), |ui, range| {
                for s in &rows[range] {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = GAP;
                        cell(ui, cols.freq, row_h, egui::Align::Min, |ui| {
                            ui.label(
                                RichText::new(format!("{}", s.freq_khz.round() as i64)).monospace(),
                            );
                        });
                        cell(ui, cols.band, row_h, egui::Align::Min, |ui| {
                            ui.label(
                                RichText::new(broadcast::metre_band(s.freq_khz).unwrap_or(""))
                                    .size(11.0)
                                    .color(crate::theme::CYAN()),
                            );
                        });
                        cell(ui, cols.name, row_h, egui::Align::Min, |ui| {
                            ui.add(egui::Label::new(RichText::new(&s.name).strong()).truncate());
                        });
                        for (w, text, grey) in [
                            (cols.lang, &s.lang, 160),
                            (cols.target, &s.target, 160),
                            (cols.site, &s.site, 140),
                        ] {
                            cell(ui, w, row_h, egui::Align::Min, |ui| {
                                ui.add(
                                    egui::Label::new(
                                        RichText::new(text)
                                            .size(11.0)
                                            .color(crate::theme::gray(grey)),
                                    )
                                    .truncate(),
                                );
                            });
                        }
                        if local_time {
                            cell(ui, cols.local, row_h, egui::Align::Min, |ui| {
                                if let Some(lon) = s.lon {
                                    let (_, _, _, h, mi, _) = sdroxide_types::utc_ymd_hms(now);
                                    let local = broadcast::local_solar_hhmm(
                                        h as u16 * 100 + mi as u16,
                                        lon,
                                    );
                                    ui.label(
                                        RichText::new(local.clone())
                                            .size(11.0)
                                            .monospace()
                                            .color(crate::theme::gray(140)),
                                    )
                                    .on_hover_text(tr!(
                                        "schedule-solar-hover",
                                        local = local,
                                        site = s.site,
                                        lon = format!("{lon:.1}")
                                    ));
                                }
                            });
                        }
                        ui.add_space(cols.free);
                        let stock = crate::theme::spot_color(sdroxide_types::SpotKind::Broadcast);
                        let mut col = self
                            .ui_settings
                            .programs_colours
                            .get(&s.name)
                            .copied()
                            .or(self.ui_settings.programs_colour_all)
                            .unwrap_or([stock.0, stock.1, stock.2]);
                        cell(ui, cols.colour, row_h, egui::Align::Center, |ui| {
                            if super::colour_edit::colour_swatch_framed(ui, &mut col)
                                .on_hover_text(tr!("schedule-colour-station-hint"))
                                .changed()
                            {
                                self.ui_settings.programs_colours.insert(s.name.clone(), col);
                                crate::app::persist::persist_ui_settings(&self.ui_settings);
                            }
                        });
                        let fav = self.broadcast_favs.iter().any(|n| n == &s.name);
                        cell(ui, cols.star, row_h, egui::Align::Center, |ui| {
                            if ui
                                .small_button(if fav { "★" } else { "☆" })
                                .on_hover_text(tr!("schedule-favourite-hint"))
                                .clicked()
                            {
                                *fav_toggle = Some((s.name.clone(), !fav));
                            }
                        });
                        cell(ui, cols.listen, row_h, egui::Align::Min, |ui| {
                            if crate::chrome::chip(ui, false, tr!("schedule-listen"))
                                .on_hover_text(tr!("schedule-listen-hint"))
                                .clicked()
                            {
                                *tune = Some(s.clone());
                            }
                        });
                        cell(ui, cols.log, row_h, egui::Align::Min, |ui| {
                            if crate::chrome::chip(ui, false, tr!("topbar-chip-log")).clicked() {
                                *log = Some(s.clone());
                            }
                        });
                    });
                }
            });
        // Like a spreadsheet: a thin rule between the columns, from the names
        // down to the bottom of the list, at the same x as the header's cells.
        {
            let mut x = x0;
            let widths = [
                cols.freq,
                cols.band,
                cols.name,
                cols.lang,
                cols.target,
                cols.site,
                cols.local,
                cols.free,
                cols.colour,
                cols.star,
                cols.listen,
            ];
            let bottom = list.inner_rect.bottom();
            let stroke = egui::Stroke::new(1.0, crate::theme::gray(48));
            for w in widths {
                if w == 0.0 {
                    continue;
                }
                x += w + GAP;
                ui.painter().vline(x - GAP / 2.0, names_top..=bottom, stroke);
            }
        }

        self.schedule.cache = rows;
        self.schedule.cache_key = Some(key);
    }

    /// The list's header, in two rows over the same columns as the data, the
    /// names touching the first row of data: a SORT menu over each data column
    /// and, on the same row, the colour controls over the right-hand group — the
    /// palette over the colour column, PST over the stars and DEFAULT over
    /// LISTEN — then the column names. Numbers are right-aligned, header
    /// included.
    ///
    /// Returns the y of the names row, where the column rules begin.
    fn schedule_header(&mut self, ui: &mut egui::Ui, cols: &Cols, row_h: f32) -> f32 {
        let local = self.schedule.local_time;
        let data: [(ProgramsSort, f32, String); 6] = [
            (ProgramsSort::Freq, cols.freq, tr!("schedule-col-freq")),
            (ProgramsSort::Band, cols.band, tr!("schedule-col-band")),
            (ProgramsSort::Name, cols.name, tr!("schedule-col-name")),
            (ProgramsSort::Lang, cols.lang, tr!("schedule-col-lang")),
            (ProgramsSort::Target, cols.target, tr!("schedule-col-target")),
            (ProgramsSort::Site, cols.site, tr!("schedule-col-site")),
        ];

        // Row 1 — the SORT menus.
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = GAP;
            for (col, w, _) in &data {
                let active = self.schedule.sort == *col;
                let arrow = match (active, self.schedule.sort_desc) {
                    (false, _) => "",
                    (true, false) => " ▲",
                    (true, true) => " ▼",
                };
                cell(ui, *w, row_h, egui::Align::Min, |ui| {
                    egui::ComboBox::from_id_salt(("sched-sort", *col as u8))
                        .width((*w).min(SORT_MENU_MAX_W))
                        .selected_text(
                            RichText::new(format!("{}{arrow}", tr!("schedule-sort"))).size(10.5),
                        )
                        .show_ui(ui, |ui| {
                            let asc = active && !self.schedule.sort_desc;
                            let desc = active && self.schedule.sort_desc;
                            if ui.selectable_label(asc, tr!("schedule-sort-asc")).clicked() {
                                self.schedule.sort = *col;
                                self.schedule.sort_desc = false;
                            }
                            if ui.selectable_label(desc, tr!("schedule-sort-desc")).clicked() {
                                self.schedule.sort = *col;
                                self.schedule.sort_desc = true;
                            }
                        });
                });
            }
            if local {
                cell(ui, cols.local, row_h, egui::Align::Min, |_| {});
            }
            ui.add_space(cols.free);
            let stock = crate::theme::spot_color(sdroxide_types::SpotKind::Broadcast);
            let mut all =
                self.ui_settings.programs_colour_all.unwrap_or([stock.0, stock.1, stock.2]);
            let swatch = cell(ui, cols.colour, row_h, egui::Align::Center, |ui| {
                super::colour_edit::colour_swatch_framed(ui, &mut all)
            });
            let paste = cell(ui, cols.star, row_h, egui::Align::Center, |ui| {
                crate::chrome::chip(ui, false, "PST")
            });
            if super::colour_edit::colour_paste(ui, &mut all, &swatch, paste) | swatch.changed() {
                // "All" means all: the colours picked one by one give way to it.
                self.ui_settings.programs_colour_all = Some(all);
                self.ui_settings.programs_colours.clear();
                crate::app::persist::persist_ui_settings(&self.ui_settings);
            }
            cell(ui, cols.listen, row_h, egui::Align::Min, |ui| {
                if crate::chrome::chip(ui, false, tr!("schedule-colour-reset"))
                    .on_hover_text(tr!("schedule-colour-reset-hint"))
                    .clicked()
                {
                    self.ui_settings.programs_colour_all = None;
                    self.ui_settings.programs_colours.clear();
                    crate::app::persist::persist_ui_settings(&self.ui_settings);
                }
            });
        });

        // Row 2 — the column names, right against the data.
        let names_top = ui.cursor().min.y;
        let names_h = 18.0;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = GAP;
            for (_, w, name) in &data {
                cell(ui, *w, names_h, egui::Align::Min, |ui| {
                    ui.label(RichText::new(name).size(10.5).color(crate::theme::gray(150)));
                });
            }
            if local {
                cell(ui, cols.local, names_h, egui::Align::Min, |ui| {
                    ui.label(
                        RichText::new(tr!("schedule-col-local"))
                            .size(10.5)
                            .color(crate::theme::gray(150)),
                    );
                });
            }
            ui.add_space(cols.free);
            cell(ui, cols.colour, names_h, egui::Align::Center, |ui| {
                ui.label(
                    RichText::new(tr!("schedule-colour-all"))
                        .size(10.5)
                        .color(crate::theme::gray(150)),
                )
                .on_hover_text(tr!("schedule-colour-all-hint"));
            });
        });
        names_top
    }
}

/// One cell of the list: `w` wide whatever it holds, so the columns line up
/// from row to row. The content is aligned inside it by `align`.
fn cell<R>(
    ui: &mut egui::Ui,
    w: f32,
    h: f32,
    align: egui::Align,
    add: impl FnOnce(&mut egui::Ui) -> R,
) -> R {
    let layout = match align {
        egui::Align::Max => egui::Layout::right_to_left(egui::Align::Center),
        // Centred by the layout, not by a justified one that would stretch the
        // widget: the cell is as tall as the widgets it holds.
        egui::Align::Center => egui::Layout::top_down(egui::Align::Center),
        egui::Align::Min => egui::Layout::left_to_right(egui::Align::Center),
    };
    let mut out = None;
    ui.allocate_ui_with_layout(egui::vec2(w, h), layout, |ui| {
        ui.set_width(w);
        ui.set_clip_rect(ui.clip_rect().intersect(ui.max_rect()));
        out = Some(add(ui));
    });
    out.expect("the cell body ran")
}

/// The widths of the list's columns.
struct Cols {
    freq: f32,
    band: f32,
    name: f32,
    lang: f32,
    target: f32,
    site: f32,
    local: f32,
    /// The empty stretch between the text columns and the right-hand group
    /// (colour, PST, LISTEN, LOG), which stays against the right edge.
    free: f32,
    colour: f32,
    star: f32,
    listen: f32,
    log: f32,
}

/// The words that set a column's width, in the language on screen. Kept apart
/// from egui so a test can measure every language without switching the
/// process-wide one.
struct ColLabels {
    sort: String,
    listen: String,
    reset: String,
    log: String,
}

impl ColLabels {
    fn current() -> Self {
        ColLabels {
            sort: format!("{} ▼", tr!("schedule-sort")),
            listen: tr!("schedule-listen"),
            reset: tr!("schedule-colour-reset"),
            log: tr!("topbar-chip-log"),
        }
    }
}

/// The most width the three text columns (language, target, site) take beyond
/// their floors, once the name has what it asks for.
const GROW_TEXT_MAX: f32 = 160.0;

/// The empty room kept after the longest name, in characters: the gap between
/// the name column and the language never goes past it.
const NAME_SLACK_CHARS: usize = 40;

/// The widest a SORT menu is drawn, however wide its column.
const SORT_MENU_MAX_W: f32 = 120.0;

/// The narrowest the name column goes: wide enough to tell two stations apart.
const NAME_MIN_W: f32 = 140.0;

impl Cols {
    /// The columns for a list `avail` wide — **the width the rows really get**,
    /// scroll bar already taken off. Each column has a floor, the widest thing
    /// that ever goes in it (its SORT menu, its chip) measured in the language on
    /// screen, so nothing overlaps its neighbour. Width beyond the floors is
    /// shared out: the name first, up to `name_w` (its longest text plus 40
    /// characters), then the three text columns, and what is left becomes an
    /// empty stretch before the right-hand group — never a gap after the name.
    fn new(ui: &egui::Ui, avail: f32, local: bool, name_w: f32) -> Self {
        Self::for_labels(ui, &ColLabels::current(), avail, local, name_w)
    }

    /// The width every column needs at its floor, gaps and the 2 pt margin
    /// included. A window narrower than this cannot hold its own list.
    fn floor_width(ui: &egui::Ui, labels: &ColLabels, local: bool) -> f32 {
        let c = Self::floors(ui, labels, local);
        let n = if local { 11.0 } else { 10.0 };
        c.freq
            + c.band
            + c.name
            + c.lang
            + c.target
            + c.site
            + c.local
            + c.colour
            + c.star
            + c.listen
            + c.log
            + n * GAP
            + 2.0
    }

    fn floors(ui: &egui::Ui, labels: &ColLabels, local: bool) -> Self {
        let font = egui::FontId::proportional(10.5);
        // The SORT menu is as wide as its text, arrow and caret need — a
        // sorted one reads "SORT ▲" and is the widest. The column is at least that.
        let sort_w = crate::chrome::text_width(ui, &labels.sort, font) + 42.0;
        let chip = |label: &str| crate::chrome::chip_width(ui, label, None);
        let colour = super::colour_edit::colour_swatch_framed_width();
        let star = chip("PST").max(30.0);
        let listen = chip(&labels.listen).max(chip(&labels.reset));
        let log = chip(&labels.log);
        Cols {
            freq: sort_w.max(54.0),
            band: sort_w.max(46.0),
            name: NAME_MIN_W,
            lang: sort_w.max(64.0),
            target: sort_w.max(64.0),
            site: sort_w.max(100.0),
            local: if local { 52.0 } else { 0.0 },
            free: 0.0,
            colour,
            star,
            listen,
            log,
        }
    }

    fn for_labels(ui: &egui::Ui, labels: &ColLabels, avail: f32, local: bool, name_w: f32) -> Self {
        let mut c = Self::floors(ui, labels, local);
        let mut spare = (avail - Self::floor_width(ui, labels, local)).max(0.0);
        // The name is the column whose text is long: it takes what it asks for,
        // so a long name is shown whole whenever the window allows.
        let name = spare.min((name_w - c.name).max(0.0));
        c.name += name;
        spare -= name;
        // The three text columns truncate, so they get some room next, up to a cap.
        let text = spare.min(GROW_TEXT_MAX);
        c.lang += text * 0.25;
        c.target += text * 0.25;
        c.site += text * 0.5;
        spare -= text;
        // Past that the window is simply wider than the list needs.
        c.free = spare;
        c
    }
}

#[cfg(test)]
mod source_filter_tests {
    use super::{ScheduleUi, source_label};
    use sdroxide_types::ScheduleSource;

    /// **The default has to say what it actually does.** With three schedules in
    /// the store, a combobox reading "EiBi" would be this fork's house rule in
    /// miniature — a control that cannot be misread. It reads *all of them*.
    #[test]
    fn the_source_filter_default_says_all_of_them() {
        assert_eq!(source_label(None), "all of them");
        assert_eq!(ScheduleUi::default().source, None, "and starts on the store");
    }

    /// Every source is **distinguishable** in the filter. Two sources sharing a
    /// label would make the combobox a control that cannot say which one is
    /// selected — the same fault as the default being wrong, one row over.
    #[test]
    fn every_source_has_its_own_name() {
        let all = [
            ScheduleSource::Eibi,
            ScheduleSource::Aoki,
            ScheduleSource::Hfcc,
            ScheduleSource::User,
        ];
        let mut seen: Vec<String> = Vec::new();
        for src in all {
            let label = source_label(Some(src));
            assert!(!label.is_empty(), "{src:?} has no name at all");
            assert!(
                !seen.contains(&label),
                "{src:?} reads {label:?}, which another source also uses"
            );
            seen.push(label);
        }
        assert_eq!(source_label(Some(ScheduleSource::Aoki)), "Aoki");
    }
}

#[cfg(test)]
mod bar_budget {
    use super::*;

    /// What each of the two bar rows asks of the window, in points, measured
    /// with the real chip and text metrics. Labels are English stretched by a
    /// third — German and Dutch run about that much longer — because the language
    /// is process-wide and a test cannot switch it without racing the others.
    fn rows_need(ctx: &egui::Context) -> (f32, f32) {
        let mut out = (0.0, 0.0);
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1000.0, 600.0),
            )),
            ..Default::default()
        };
        ctx.run_ui(input, |ui| {
            let gap = ui.spacing().item_spacing.x;
            let label =
                |t: &str| crate::chrome::text_width(ui, t, egui::FontId::proportional(14.0)) * 1.33;
            let chip = |t: &str| crate::chrome::chip_width(ui, t, None) * 1.33;
            let btn = |t: &str| label(t) + 2.0 * ui.spacing().button_padding.x;
            let sep = 1.0;
            let row1 = [
                label("At"),
                chip("NOW"),
                50.0,
                btn("set time"),
                sep,
                label("Find"),
                FIND_W,
                label("Language"),
                LANG_W,
                label("Target"),
                TARGET_W,
                btn("Import…"),
            ];
            let row2 = [
                label("Band"),
                BAND_W + 20.0,
                label("Source"),
                SOURCE_W + 20.0,
                chip("★ FAVS"),
                chip("⭐ ALL"),
                chip("LOCAL TIME"),
                sep,
                chip("SHOW MAP"),
            ];
            let total = |r: &[f32]| r.iter().sum::<f32>() + gap * (r.len() as f32 - 1.0);
            out = (total(&row1), total(&row2));
        })
        .drop_without_applying_deltas();
        out
    }

    /// **Two rows, and each fits the window's floor.** The chips of the old
    /// single wrapping row stacked on one another in a narrow window; the bar is
    /// now two rows whose widths are checked here against the narrowest the
    /// window goes, window margins taken off.
    #[test]
    fn each_bar_row_fits_the_narrowest_window() {
        let ctx = egui::Context::default();
        let inner = SCHEDULE_WINDOW_MIN_W - 2.0 * 10.0 - 2.0;
        let (r1, r2) = rows_need(&ctx);
        assert!(r1 < inner, "the time and search row needs {r1:.0} pt of {inner:.0}");
        assert!(r2 < inner, "the dropdown and chip row needs {r2:.0} pt of {inner:.0}");
    }
}

#[cfg(test)]
mod sort_and_filters {
    use super::*;

    fn st(name: &str, khz: f64, lang: &str) -> BroadcastStation {
        BroadcastStation {
            name: name.into(),
            freq_khz: khz,
            lang: lang.into(),
            ..Default::default()
        }
    }

    fn names(rows: &[BroadcastStation]) -> Vec<&str> {
        rows.iter().map(|s| s.name.as_str()).collect()
    }

    #[test]
    fn the_list_sorts_by_each_column_both_ways() {
        let mut v = vec![
            st("bravo", 9410.0, "FR"),
            st("Alpha", 6070.0, "en"),
            st("charlie", 15735.0, "FR"),
        ];
        sort_rows(&mut v, ProgramsSort::Freq, false);
        assert_eq!(names(&v), ["Alpha", "bravo", "charlie"]);
        sort_rows(&mut v, ProgramsSort::Freq, true);
        assert_eq!(names(&v), ["charlie", "bravo", "Alpha"]);
        sort_rows(&mut v, ProgramsSort::Name, false);
        assert_eq!(names(&v), ["Alpha", "bravo", "charlie"], "names compare without case");
        sort_rows(&mut v, ProgramsSort::Name, true);
        assert_eq!(names(&v), ["charlie", "bravo", "Alpha"]);
        sort_rows(&mut v, ProgramsSort::Lang, false);
        assert_eq!(names(&v), ["Alpha", "bravo", "charlie"], "equal languages keep dial order");
        sort_rows(&mut v, ProgramsSort::Band, false);
        assert_eq!(
            names(&v),
            ["Alpha", "bravo", "charlie"],
            "49m, 31m, 19m is 6070 before 9410 before 15735"
        );
    }

    /// Every filter, the sort and the TOUT chip go out and come back as they
    /// were, and the window built from them filters the same way.
    #[test]
    fn the_window_reopens_as_it_was_left() {
        let mut ui = ScheduleUi::default();
        ui.query = "bbc".into();
        ui.lang = "en".into();
        ui.target = "af".into();
        ui.band = "49m".into();
        ui.hhmm = 1423;
        ui.use_now = false;
        ui.favourites_only = true;
        ui.source = Some(ScheduleSource::Hfcc);
        ui.local_time = true;
        ui.sort = ProgramsSort::Site;
        ui.sort_desc = true;
        let saved = ui.to_settings(true);
        assert!(saved.all, "TOUT is kept");
        let back = ScheduleUi::from_settings(&saved);
        assert_eq!(back.to_settings(true), saved);
        assert!(!back.use_now && back.local_time && back.sort_desc);
        assert_eq!(back.source, Some(ScheduleSource::Hfcc));
        // A fresh install opens on "now", sorted by frequency.
        let fresh = ScheduleUi::from_settings(&ProgramsFilters::default());
        assert!(fresh.use_now);
        assert_eq!(fresh.sort, ProgramsSort::Freq);
    }
}

/// A picture of the real PROGRAMS window, for looking at rather than asserting.
///
/// ```sh
/// SCHED_LANGS=en,fr,de,ru SCHED_W=860 SCHED_LOCAL=1 \
///   cargo test --release -p sdroxide-ui --lib schedule_preview -- --ignored --nocapture
/// ```
/// `SCHED_W` is the window width (default: the narrowest it goes), `SCHED_LOCAL=1`
/// turns on the LOCAL TIME column, `SCHED_OUT` the folder the PNGs go to. The
/// language is process-wide, so one run draws each of `SCHED_LANGS` in turn.
#[cfg(all(test, not(target_arch = "wasm32")))]
mod preview {
    use super::*;
    use eframe::App as _;
    use sdroxide_types::{DetachedWindow, RadioController, RadioEvent};

    struct Quiet;
    impl RadioController for Quiet {
        fn send(&mut self, _cmd: Command) {}
        fn poll_event(&mut self) -> Option<RadioEvent> {
            None
        }
    }

    #[test]
    #[ignore = "writes pictures to look at; not an assertion"]
    fn schedule_preview() {
        let _guard = crate::multi::frame_test_lock();
        let dir =
            std::env::temp_dir().join(format!("sdroxide-sched-preview-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };
        let num =
            |var: &str, d: f32| std::env::var(var).ok().and_then(|v| v.parse().ok()).unwrap_or(d);
        let width = num("SCHED_W", SCHEDULE_WINDOW_MIN_W);
        let local = std::env::var("SCHED_LOCAL").is_ok_and(|v| v == "1");
        let langs = std::env::var("SCHED_LANGS").unwrap_or_else(|_| "en".into());
        let out_dir = std::env::var("SCHED_OUT").unwrap_or_else(|_| {
            format!("{}/../../target/schedule_preview", env!("CARGO_MANIFEST_DIR"))
        });
        std::fs::create_dir_all(&out_dir).unwrap();
        let screen = egui::vec2(width.max(1150.0) + 24.0, 640.0);

        for tag in langs.split(',').map(str::trim).filter(|t| !t.is_empty()) {
            let ctx = egui::Context::default();
            let mut app = SdroxideApp::new_tab(&ctx, None, None, Box::new(Quiet), 0, true);
            crate::i18n::set_language(sdroxide_types::UiLanguage::from_tag(tag).unwrap());
            app.schedule.show = true;
            app.schedule.local_time = local;
            app.ui_settings.schedule_window =
                Some(DetachedWindow { size: [width, 600.0], pos: Some([4.0, 4.0]) });
            let mut tex = crate::test_raster::Textures::default();
            let mut last = None;
            for i in 0..6 {
                let input = egui::RawInput {
                    screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, screen)),
                    time: Some(10.0 + i as f64 / 30.0),
                    ..Default::default()
                };
                let mut full = ctx.run_ui(input, |ui| {
                    app.ui(ui, &mut eframe::Frame::_new_kittest());
                });
                tex.apply(&full.textures_delta);
                full.textures_delta.clear();
                last = Some(full);
            }
            let img = crate::test_raster::rasterize(
                &ctx,
                last.unwrap(),
                &tex,
                screen,
                crate::theme::BG_DEEP(),
            );
            let out = format!("{out_dir}/{tag}_{}.png", if local { "local" } else { "utc" });
            img.save(&out).expect("write the preview");
            println!("{tag}: window {:?} -> {out}", app.tool_rect);
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod fit_and_keep {
    use super::*;
    use eframe::App as _;
    use sdroxide_types::{RadioController, RadioEvent};

    struct Quiet;
    impl RadioController for Quiet {
        fn send(&mut self, _cmd: Command) {}
        fn poll_event(&mut self) -> Option<RadioEvent> {
            None
        }
    }

    /// One message's text from a language's `.ftl` file, so every language can be
    /// measured without switching the process-wide one under other tests.
    fn message(lang: &str, id: &str) -> String {
        let path = format!("{}/i18n/{lang}/sdroxide.ftl", env!("CARGO_MANIFEST_DIR"));
        let text = std::fs::read_to_string(&path).unwrap();
        text.lines()
            .find_map(|l| l.strip_prefix(&format!("{id} = ")))
            .unwrap_or_else(|| panic!("{id} is missing from {path}"))
            .trim()
            .to_owned()
    }

    /// **No language needs a window wider than a laptop screen**, and the floor
    /// the window reports covers it. Russian asks for ~1040 pt (six "Сортировка"
    /// menus and "ПО УМОЛЧАНИЮ"); before, the floor was a fixed 860, so the row
    /// came out wider than the frame, the window grew by itself and the
    /// right-hand chips left it (German 922 pt, Russian 1055 pt at 860).
    #[test]
    fn every_language_fits_a_reasonable_window() {
        let ctx = egui::Context::default();
        let chrome = 2.0 * 10.0;
        let langs: Vec<String> = std::fs::read_dir(format!("{}/i18n", env!("CARGO_MANIFEST_DIR")))
            .unwrap()
            .filter_map(|e| e.ok()?.file_name().into_string().ok())
            .filter(|n| n != "README.md" && !n.contains('.'))
            .collect();
        assert!(langs.len() >= 11, "{langs:?}");
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1000.0, 600.0),
            )),
            ..Default::default()
        };
        ctx.run_ui(input, |ui| {
            let bar = ui.spacing().scroll.allocated_width();
            for lang in &langs {
                let labels = ColLabels {
                    sort: format!("{} ▼", message(lang, "schedule-sort")),
                    listen: message(lang, "schedule-listen"),
                    reset: message(lang, "schedule-colour-reset"),
                    log: message(lang, "topbar-chip-log"),
                };
                let need = Cols::floor_width(ui, &labels, true) + bar + chrome;
                assert!(need <= 1150.0, "{lang} needs {need:.0} pt for its list");
            }
        })
        .drop_without_applying_deltas();
    }

    /// The columns add up to the width they were given, wide or narrow, so the
    /// row is neither short of the frame nor past it.
    #[test]
    fn the_columns_use_exactly_the_width_they_are_given() {
        let ctx = egui::Context::default();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1000.0, 600.0),
            )),
            ..Default::default()
        };
        ctx.run_ui(input, |ui| {
            let labels = ColLabels {
                sort: "Sort ▼".into(),
                listen: "LISTEN".into(),
                reset: "DEFAULT".into(),
                log: "LOG".into(),
            };
            for local in [false, true] {
                let floor = Cols::floor_width(ui, &labels, local);
                for avail in [floor, floor + 120.0, 1400.0, 1960.0] {
                    let c = Cols::for_labels(ui, &labels, avail, local, 400.0);
                    let n = if local { 11.0 } else { 10.0 };
                    let sum = c.freq
                        + c.band
                        + c.name
                        + c.lang
                        + c.target
                        + c.site
                        + c.local
                        + c.free
                        + c.colour
                        + c.star
                        + c.listen
                        + c.log
                        + n * GAP
                        + 2.0;
                    assert!((sum - avail).abs() < 0.01, "local={local}: {sum} for {avail}");
                }
            }
        })
        .drop_without_applying_deltas();
    }

    /// **The name never gets more than its longest text plus 40 characters.**
    /// A wide window puts the rest after the text columns, not between the name
    /// and the language; a longer name than that still gets all the room it asks
    /// for, as far as the window goes.
    #[test]
    fn the_gap_after_the_name_stays_within_forty_characters() {
        let ctx = egui::Context::default();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1000.0, 600.0),
            )),
            ..Default::default()
        };
        ctx.run_ui(input, |ui| {
            let labels = ColLabels {
                sort: "Sort ▼".into(),
                listen: "LISTEN".into(),
                reset: "DEFAULT".into(),
                log: "LOG".into(),
            };
            let floor = Cols::floor_width(ui, &labels, false);
            let wide = Cols::for_labels(ui, &labels, 1960.0, false, 300.0);
            assert!((wide.name - 300.0).abs() < 0.01, "name {}", wide.name);
            assert!(wide.free > 0.0, "the surplus goes before the right-hand group");
            // An 80-character name (about 560 pt) is shown whole when the room exists.
            let long = Cols::for_labels(ui, &labels, 1960.0, false, 560.0);
            assert!(long.name >= 560.0 - 0.01);
            // Narrow: the name takes the room first and nothing is left over.
            let tight = Cols::for_labels(ui, &labels, floor + 50.0, false, 300.0);
            assert!((tight.name - (NAME_MIN_W + 50.0)).abs() < 0.01);
            assert_eq!(tight.free, 0.0);
        })
        .drop_without_applying_deltas();
    }

    /// **What the station sends is what the window shows.** TOUT, the stars, the
    /// filters and the colours arrive in one piece, and the app hands the same
    /// piece back when it is asked for its state — so the round trip through the
    /// station changes nothing.
    #[test]
    fn the_stations_state_is_shown_and_handed_back_as_it_came() {
        let _guard = crate::multi::frame_test_lock();
        let dir = std::env::temp_dir().join(format!("sdroxide-state-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };
        let ctx = egui::Context::default();
        let mut app = SdroxideApp::new_tab(&ctx, None, None, Box::new(Quiet), 0, true);
        let mut state = sdroxide_types::ProgramsState::default();
        state.filters.all = true;
        state.filters.query = "BBC".into();
        state.filters.lang = "French".into();
        state.filters.favourites_only = true;
        state.favourites = vec!["BBC".into(), "RFI".into()];
        state.colour_all = Some([1, 2, 3]);
        state.colours.insert("RFI".into(), [4, 5, 6]);
        app.apply_programs_state(state.clone());
        assert!(app.broadcast_all, "the chip is lit");
        assert_eq!(app.broadcast_favs, ["BBC", "RFI"], "the labels have their stations");
        assert_eq!((app.schedule.query.as_str(), app.schedule.lang.as_str()), ("BBC", "French"));
        assert!(app.schedule.favourites_only);
        assert_eq!(app.programs_state(), state, "and it is handed back as it came");
        // No station to ask: the tick sends nothing and changes nothing.
        app.programs_sync_tick(&ctx);
        assert_eq!(app.programs_state(), state);
    }

    /// **TOUT survives a restart.** Lit, written, the app built again over the
    /// same config folder: the chip is lit and the favourites it made are kept.
    #[test]
    fn tout_is_still_lit_in_the_next_session() {
        let _guard = crate::multi::frame_test_lock();
        let dir = std::env::temp_dir().join(format!("sdroxide-tout-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };

        let ctx = egui::Context::default();
        let mut app = SdroxideApp::new_tab(&ctx, None, None, Box::new(Quiet), 0, true);
        assert!(!app.broadcast_all, "a fresh install starts with TOUT off");
        app.set_broadcast_all(true);
        let favs = app.broadcast_favs.len();
        drop(app);

        let ctx = egui::Context::default();
        let mut app = SdroxideApp::new_tab(&ctx, None, None, Box::new(Quiet), 0, true);
        assert!(app.broadcast_all, "TOUT was lit and is lost");
        assert_eq!(app.broadcast_favs.len(), favs, "the favourites it made are lost");
        // And a frame with the window open keeps it, whatever the bar writes back.
        app.schedule.show = true;
        for i in 0..3 {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1100.0, 640.0),
                )),
                time: Some(1.0 + i as f64 / 30.0),
                ..Default::default()
            };
            ctx.run_ui(input, |ui| {
                app.ui(ui, &mut eframe::Frame::_new_kittest());
            })
            .drop_without_applying_deltas();
        }
        assert!(app.broadcast_all && app.ui_settings.programs.all);
        drop(app);
        let ctx = egui::Context::default();
        let app = SdroxideApp::new_tab(&ctx, None, None, Box::new(Quiet), 0, true);
        assert!(app.broadcast_all, "TOUT is lost after the window was drawn");
    }
}
