//! The ADS-B panel: what is in the air, and where.
//!
//! Two things an operator watches, so two columns:
//!
//! - **AIRCRAFT** — every target being tracked, newest activity first, with the
//!   detail card for whichever one is selected pinned below the list.
//! - **MAP** — the same targets as a radar picture. See [`crate::adsb_map`].
//!
//! No third column, because unlike APRS there is nothing to say back: this is a
//! receive-only surveillance downlink and the aircraft are not listening.
//!
//! # The header says what the receiver is doing
//!
//! An empty aircraft list has three quite different causes — a quiet sky, a
//! receiver on the wrong frequency, and a stream too narrow to slice a
//! half-microsecond chip — and only one of them is anything to do with the
//! decoder. The header carries the preamble and frame counters and, when the
//! lane cannot run at all, the sentence saying why, with the retune that fixes
//! it beside it.

use crate::i18n::tr;
use eframe::egui::{self, RichText};
use sdroxide_types::{AdsbAircraft, AdsbStatus, Command};

use crate::app::SdroxideApp;
use crate::app::aircraft_history;
use crate::app::aircraft_hover;
use crate::app::aircraft_search;
use crate::app::util::fmt_age;
use crate::theme;
use crate::theme::ThemedScroll;

/// How the aircraft table is ordered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(in crate::app) enum AdsbSort {
    /// Most recently heard first. The default: on a display of things that move,
    /// "what just changed" is the question a list answers.
    #[default]
    Heard,
    Callsign,
    Altitude,
    Speed,
    /// Nearest first. Only offered once the operator's own position is known —
    /// there is no distance without one.
    Range,
    Signal,
}

impl SdroxideApp {
    pub(in crate::app) fn adsb_panel(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        panel_h: f32,
    ) {
        let st: AdsbStatus = match self.adsb_status.as_ref() {
            Some(s) => (**s).clone(),
            None => {
                ui.label(RichText::new(tr!("adsb-starting-ads-b-decoder")).weak());
                return;
            }
        };
        let now = crate::time::now_unix();
        let content_bottom = ui.cursor().top() + panel_h - 26.0;

        self.adsb_header(ui, cmds, &st);
        ui.add_space(3.0);

        let avail_h = (content_bottom - ui.cursor().top()).max(80.0);
        let pane = self.phone_pane(ui, self.state.rx[0].mode);
        let full_w = ui.available_width();

        // The list keeps a draggable share of the width. The floor is the
        // narrow table (callsign, altitude and age); the ceiling leaves the map
        // enough to still be a radar picture rather than a strip.
        const HANDLE_W: f32 = 7.0;
        let list_w = (full_w * self.view.adsb_split_fraction)
            .clamp(240.0, (full_w - HANDLE_W - 200.0).max(240.0));

        ui.horizontal_top(|ui| {
            if pane.is_none_or(|p| p == 0) {
                ui.allocate_ui_with_layout(
                    egui::vec2(if pane.is_some() { full_w } else { list_w }, avail_h),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| self.adsb_list(ui, &st, now, avail_h),
                );
            }
            if pane.is_none() {
                let h = crate::chrome::split_handle(ui, egui::vec2(HANDLE_W, avail_h), None);
                if h.dragged() {
                    self.view.adsb_split_fraction =
                        ((list_w + h.drag_delta().x) / full_w.max(1.0)).clamp(0.15, 0.85);
                }
            }
            if pane.is_none_or(|p| p == 1) {
                ui.vertical(|ui| self.adsb_map_pane(ui, cmds, &st, now, avail_h));
            }
        });
    }

    /// Where the receiver is looking, what the demodulator is finding, and the
    /// one button that fixes the commonest problem.
    ///
    /// Every volatile readout occupies a fixed-width slot, for the reason the
    /// APRS header's do: this is a `horizontal_wrapped`, and a number that grows
    /// a digit tips the whole tail onto a second line and moves every pane below
    /// it. On a busy band these counters change several times a second.
    fn adsb_header(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>, st: &AdsbStatus) {
        let dial = self.state.rx_freq_hz();
        let on_channel =
            (dial - sdroxide_types::ADSB_FREQ_HZ).abs() < 100_000.0 && st.unavailable.is_none();

        ui.horizontal_wrapped(|ui| {
            ui.set_min_height(22.0);
            ui.label(RichText::new(tr!("adsb-ads-b")).size(11.0).strong().color(theme::CYAN()));
            ui.label(RichText::new(tr!("adsb-t-1090-mhz-mode-s")).weak().size(10.5));

            // The one preset there is. A chip rather than a note, because the
            // fix for "nothing is decoding" is nearly always this.
            if crate::chrome::chip(ui, on_channel, "1090.000")
                .on_hover_text(tr!("adsb-tune-receiver-1090-000-mhz"))
                .clicked()
            {
                cmds.push(Command::SetVfo {
                    vfo: self.state.active_vfo,
                    hz: sdroxide_types::ADSB_FREQ_HZ,
                });
            }

            ui.separator();
            slot(ui, 76.0, &tr!("adsb-aircraft-3", aircraft = st.aircraft.len()), theme::CYAN());
            slot(ui, 74.0, &tr!("adsb-frames", frames = count(st.frames)), theme::gray(150));
            // A high preamble count with no frames is the honest picture of a
            // band that is busy with something the decoder cannot read, or of a
            // receiver picking up its own noise. Worth showing rather than
            // leaving the panel looking broken.
            slot(
                ui,
                88.0,
                &tr!("adsb-preambles", preambles = count(st.preambles)),
                theme::gray(120),
            );
            slot(ui, 78.0, &tr!("adsb-bad-crc", bad_crc = count(st.bad_crc)), theme::gray(120));

            if st.window_rate_hz > 0.0 {
                slot(
                    ui,
                    148.0,
                    &format!(
                        "{e6:.3} MHz / {e62:.2} Msps",
                        e6 = st.window_center_hz / 1e6,
                        e62 = st.window_rate_hz / 1e6
                    ),
                    theme::gray(120),
                );
            }

            ui.separator();
            if super::window_chip(ui) {
                self.float_operating_panel();
            }
            if crate::chrome::chip(ui, self.show_adsb_setup, "⚙")
                .on_hover_text(tr!("adsb-timeouts-trail-length-how-far"))
                .clicked()
            {
                self.show_adsb_setup = !self.show_adsb_setup;
            }
        });

        if let Some(why) = &st.unavailable {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(why).size(10.5).color(theme::HAZARD()));
                if let Some(hz) = st.suggest_center_hz
                    && (dial - hz).abs() > 1.0
                    && crate::chrome::chip(
                        ui,
                        false,
                        tr!("adsb-tune", mhz = format!("{:.3}", hz / 1e6)),
                    )
                    .clicked()
                {
                    cmds.push(Command::SetVfo { vfo: self.state.active_vfo, hz });
                }
            });
        }
        // Running, but on a stream too narrow to carry the waveform properly.
        // Said out loud because the symptom — a short list — is exactly what a
        // quiet sky looks like, and the fix is one setting away on most
        // receivers.
        // **Advice, so it can be dismissed.** It reports that this receiver is
        // only reaching part of the coverage, which stays true until the setup
        // changes — unlike a live fault, which must never be dismissible. See
        // [`crate::chrome::advisory`].
        if let Some(why) = st.degraded.clone() {
            if crate::chrome::advisory(
                ui,
                &mut self.ui_settings,
                "adsb-coverage",
                &why,
                theme::YELLOW(),
            ) {
                crate::app::persist::persist_ui_settings(&self.ui_settings);
            }
        }
    }

    /// The target list, with the detail card pinned to the bottom of the column.
    ///
    /// Pinned rather than laid out after the list, for the reason the APRS
    /// card is: on a quiet sky it would otherwise sit half way up the panel and
    /// jump every time another aeroplane came into range.
    fn adsb_list(&mut self, ui: &mut egui::Ui, st: &AdsbStatus, now: i64, avail_h: f32) {
        let home = self.adsb_home();
        let has_station = self.ctrl.peer_url().is_some();
        let query = aircraft_search::Query::parse(&self.aircraft_query);
        let found = aircraft_search::search_adsb(
            &query,
            &st.aircraft,
            &self.aircraft_registry,
            &self.aircraft_online.store,
            has_station,
        );
        ui.horizontal_wrapped(|ui| {
            ui.set_min_height(20.0);
            ui.label(RichText::new(tr!("aircraft-title")).strong().size(10.5).color(theme::CYAN()));
            let counts = found.visible.as_ref().map(|v| (v.len(), found.total));
            aircraft_search::search_box(
                ui,
                &mut self.aircraft_query,
                90.0,
                counts,
                found.hint.as_deref(),
            );
        });

        let mut rows: Vec<&AdsbAircraft> = st
            .aircraft
            .iter()
            .filter(|a| found.visible.as_ref().is_none_or(|v| v.contains(&a.icao)))
            .collect();
        sort_rows(&mut rows, self.adsb_sort, self.adsb_sort_desc, home);

        let selected = self.adsb_map.selected;
        let card = selected.and_then(|i| st.aircraft.iter().find(|a| a.icao == i));
        // The card takes a draggable share of the column, but only while there
        // is one: with nothing selected the list has the lot and there is no
        // divider to grab.
        const HANDLE_H: f32 = 7.0;
        let card_h = if card.is_some() {
            (avail_h * self.view.adsb_card_fraction).clamp(90.0, (avail_h - 120.0).max(90.0))
        } else {
            0.0
        };
        let list_h =
            (avail_h - card_h - 46.0 - if card.is_some() { HANDLE_H } else { 0.0 }).max(48.0);

        let drop_map_s = self.state.adsb.drop_map_s;
        // Outside the scroll area, so a busy sector does not scroll the column
        // headings off the top of the table it is describing — and so they stay
        // where they can be clicked to re-order it.
        adsb_head_row(ui, home.is_some(), &mut self.adsb_sort, &mut self.adsb_sort_desc);
        let mut pick = None;
        egui::ScrollArea::vertical()
            .id_salt("adsb-aircraft")
            .max_height(list_h)
            .min_scrolled_height(list_h)
            .auto_shrink([false, false])
            .show_themed(ui, |ui| {
                if rows.is_empty() {
                    ui.label(RichText::new(tr!("adsb-nothing-heard-yet")).weak());
                }
                for (i, a) in rows.iter().enumerate() {
                    if adsb_row(ui, a, now, selected, home, drop_map_s, i) {
                        pick = Some(a.icao);
                    }
                }
            });
        if let Some(icao) = pick {
            self.adsb_map.selected = (selected != Some(icao)).then_some(icao);
        }

        if let Some(a) = card {
            let w = ui.available_width();
            let h = crate::chrome::split_handle(ui, egui::vec2(w, HANDLE_H), None);
            if h.dragged() {
                // Drag up grows the card, which is the half the pointer is
                // heading for.
                self.view.adsb_card_fraction =
                    ((card_h - h.drag_delta().y) / avail_h.max(1.0)).clamp(0.12, 0.8);
            }
            self.adsb_card(ui, a, now, card_h, home);
        }
    }

    /// Everything one aircraft has said.
    fn adsb_card(
        &mut self,
        ui: &mut egui::Ui,
        a: &AdsbAircraft,
        now: i64,
        h: f32,
        home: Option<(f64, f64)>,
    ) {
        egui::ScrollArea::vertical()
            .id_salt("adsb-card")
            .max_height(h)
            .auto_shrink([false, false])
            .show_themed(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.label(RichText::new(a.label()).monospace().strong().size(13.0).color(
                        if a.emergency.is_some() { theme::HAZARD() } else { theme::YELLOW() },
                    ));
                    ui.label(RichText::new(a.hex()).monospace().size(10.5).weak());
                    if let Some(c) = &a.category {
                        ui.label(RichText::new(c).size(10.5).color(theme::CYAN_DIM()));
                    }
                    if a.on_ground {
                        ui.label(
                            RichText::new(tr!("adsb-ground")).size(10.5).color(theme::GREEN()),
                        );
                    }
                    if a.lat.is_some() && a.pos_stale(now, self.state.adsb.drop_map_s) {
                        ui.label(
                            RichText::new(tr!("adsb-position-stale"))
                                .size(10.5)
                                .color(theme::HAZARD())
                                .italics(),
                        );
                    }
                });
                if let Some(e) = &a.emergency {
                    ui.label(RichText::new(e).strong().color(theme::HAZARD()));
                }

                let mut rows: Vec<(String, String)> = Vec::new();
                if let Some(ft) = a.altitude_ft {
                    rows.push((
                        tr!("adsb-row-altitude").to_string(),
                        tr!(
                            "adsb-ft-barometric",
                            ft = sdroxide_types::AirUnits::current().altitude(ft)
                        ),
                    ));
                }
                if let Some(ft) = a.gnss_altitude_ft {
                    rows.push((
                        tr!("adsb-gnss-altitude").to_string(),
                        sdroxide_types::AirUnits::current().altitude(ft),
                    ));
                }
                if let Some(kt) = a.ground_speed_kt {
                    rows.push((
                        tr!("adsb-ground-speed").to_string(),
                        sdroxide_types::AirUnits::current().speed(kt),
                    ));
                }
                if let Some(t) = a.track_deg {
                    rows.push((tr!("adsb-row-track"), format!("{t:.0}°")));
                }
                if a.turn_rate_deg_s.abs() > 0.2 {
                    rows.push((tr!("adsb-row-turning"), format!("{:.1}°/s", a.turn_rate_deg_s)));
                }
                if let Some(v) = a.vertical_rate_fpm {
                    rows.push((
                        tr!("adsb-vertical-rate").to_string(),
                        sdroxide_types::AirUnits::current().vertical_rate(v),
                    ));
                }
                if a.squawk.is_some() {
                    rows.push((tr!("adsb-row-squawk"), a.fmt_squawk()));
                }
                if let (Some((hlat, hlon)), Some((lat, lon))) = (home, a.lat.zip(a.lon)) {
                    let km = sdroxide_types::distance_km((hlat, hlon), (lat, lon));
                    let bear = sdroxide_types::bearing_deg((hlat, hlon), (lat, lon));
                    rows.push((
                        tr!("adsb-row-range").to_string(),
                        format!("{}  {bear:.0}°", sdroxide_types::AirUnits::current().distance(km)),
                    ));
                }
                if let Some((lat, lon)) = a.lat.zip(a.lon) {
                    rows.push((tr!("adsb-row-position"), format!("{lat:.4}, {lon:.4}")));
                }
                rows.push((
                    tr!("adsb-row-signal"),
                    format!("{rssi_dbfs:.0} dBFS", rssi_dbfs = a.rssi_dbfs),
                ));
                rows.push((tr!("adsb-row-frames"), format!("{} ({})", a.frames, a.source.label())));
                rows.push((tr!("adsb-first-heard"), fmt_age(now - a.first_at)));

                egui::Grid::new("adsb-card-grid").num_columns(2).spacing([10.0, 1.0]).show(
                    ui,
                    |ui| {
                        for (k, v) in rows {
                            ui.label(RichText::new(k).size(10.0).weak());
                            // Wrapped, never cut: "37000 ft barometric" ran out of
                            // room beside its label and ended in an ellipsis.
                            ui.add(
                                egui::Label::new(RichText::new(v).monospace().size(10.5)).wrap(),
                            );
                            ui.end_row();
                        }
                    },
                );

                ui.horizontal_wrapped(|ui| {
                    if a.has_position()
                        && crate::chrome::chip(
                            ui,
                            false,
                            RichText::new(tr!("adsb-center")).size(10.0),
                        )
                        .on_hover_text(tr!("adsb-put-aircraft-middle-map"))
                        .clicked()
                        && let Some((lat, lon)) = a.lat.zip(a.lon)
                    {
                        self.adsb_map.view.centre_on(lat, lon);
                    }
                });
                if !a.raw_hex.is_empty() {
                    ui.label(
                        RichText::new(&a.raw_hex)
                            .monospace()
                            .size(9.0)
                            .color(theme::gray(110))
                            .weak(),
                    )
                    .on_hover_text(tr!("adsb-last-frame-accepted-aircraft"));
                }
            });
    }

    /// The map pane's own row: NIGHT, and the ground it is drawn on.
    pub(in crate::app) fn adsb_map_pane(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        st: &AdsbStatus,
        now: i64,
        h: f32,
    ) {
        ui.horizontal(|ui| {
            self.night_chip(ui);
            self.map_style_chip(ui);
            // The two layers the picture is read *for*: where the flight has
            // been and is going, and what is on the ground under it.
            let mut layers = self.adsb_map.layers;
            if crate::chrome::chip(
                ui,
                layers.paths,
                RichText::new(tr!("adsbpanel-paths")).size(9.5),
            )
            .on_hover_text(tr!("adsbpanel-paths-hint"))
            .clicked()
            {
                layers.paths = !layers.paths;
                self.adsb_map.layers = layers;
            }
            if crate::chrome::chip(ui, layers.nav, RichText::new(tr!("adsbpanel-nav")).size(9.5))
                .on_hover_text(tr!("adsbpanel-nav-hint"))
                .clicked()
            {
                layers.nav = !layers.nav;
                self.adsb_map.layers = layers;
            }
        });
        let home = self.adsb_home();
        let cfg = self.state.adsb;
        let night = self.night_texture(ui.ctx());
        // The search box: what it lets through is told to the map, which draws,
        // hovers and frames only that. It reads what is in memory and asks for
        // nothing — see [`aircraft_search`].
        let query = aircraft_search::Query::parse(&self.aircraft_query);
        let found = aircraft_search::search_adsb(
            &query,
            &st.aircraft,
            &self.aircraft_registry,
            &self.aircraft_online.store,
            self.ctrl.peer_url().is_some(),
        );
        let counts = found.visible.as_ref().map(|v| (v.len(), found.total));
        aircraft_search::search_box(
            ui,
            &mut self.aircraft_query,
            180.0,
            counts,
            found.hint.as_deref(),
        );
        self.adsb_map.visible = found.visible;
        // Where the aircraft whose history is on has been, from the station's
        // record; nothing when none is on or the record has not come yet.
        self.adsb_map.trail = self
            .aircraft_online
            .history
            .on()
            .and_then(|i| self.aircraft_online.history.journal(i))
            .map(|j| aircraft_history::trail_segments(j, now))
            .unwrap_or_default();
        // The box takes a row of its own, and its note another.
        let h = (h - 26.0 - if found.hint.is_some() { 16.0 } else { 0.0 }).max(100.0);
        // The hover card reads what the AIRCRAFT window's card already holds and
        // asks for nothing: no request, no download, from a pointer passing over
        // the map. See [`aircraft_hover`].
        let has_station = self.ctrl.peer_url().is_some();
        let registry = &self.aircraft_registry;
        let online = &self.aircraft_online;
        let hover = |ui: &mut egui::Ui, a: &AdsbAircraft| {
            let (card, tex) =
                aircraft_hover::card_view(a, registry, online, has_station, home, now);
            aircraft_hover::draw(ui, &card, tex);
        };
        let state = &mut self.adsb_map;
        // **A double-click on a station tunes it, on the other dial.** Never the
        // one in use: ADS-B owns that one — it is a channel, and the engine puts
        // the dial back on 1090 MHz every time that VFO is recalled — so a
        // station written there is overwritten (the operator traced that
        // himself). It goes to the free dial, **switches to it**, and puts AM on
        // it: an aid is amplitude modulated and a lane mode cannot carry it, so a
        // station parked on a dial still in ADS-B would be silence and then
        // retuned. A/B is the switch between the plane and the ground station.
        if let Some(t) = crate::adsb_map::show(
            ui,
            state,
            &st.aircraft,
            home,
            now,
            cfg,
            night,
            self.ui_settings.survey_map_style,
            h,
            self.state.active_vfo,
            Some(&hover),
        ) {
            cmds.push(Command::SelectVfo(t.vfo));
            cmds.push(Command::SetVfo { vfo: t.vfo, hz: t.hz });
            cmds.push(Command::SetModeListen {
                rx: sdroxide_types::RxId::Main,
                mode: sdroxide_types::Mode::Am,
            });
        }
        // A click on an aircraft pins its card beside it. The click asks for
        // nothing; the card's own button does.
        self.adsb_pinned_card(ui, &st.aircraft, home, now);
        self.adsb_airport_online(ui.ctx(), now);
    }

    /// The operator's own position, from the grid in the digital-mode setup.
    ///
    /// The same source the FT8 and APRS maps use, so the three never disagree
    /// about where the station is. `None` until it has been filled in, which is
    /// why the range column and the RANGE sort are conditional.
    fn adsb_home(&self) -> Option<(f64, f64)> {
        let grid = self.digi_cfg_edit.my_grid.trim();
        (!grid.is_empty()).then(|| sdroxide_types::grid_to_latlon(grid)).flatten()
    }
}

impl SdroxideApp {
    /// How the decoder behaves: the two timeouts, the trail length and the
    /// leader-line time.
    ///
    /// Its own window rather than a section of the digimode setup dialog, which
    /// is what every other panel's SETUP chip opens: that one edits a
    /// `DigiConfig` and exists to hold an operator identity and a set of
    /// message templates, and ADS-B has neither a callsign nor anything to say.
    pub(in crate::app) fn adsb_setup_window(
        &mut self,
        ctx: &egui::Context,
        cmds: &mut Vec<Command>,
    ) {
        // Edited as a copy and diffed at the end, the way the ISM window does:
        // the engine persists whatever arrives and echoes it back in the state,
        // so there is no apply step and no way for the two copies to drift.
        let mut cfg = self.state.adsb;
        let open = self.tool_window(
            ctx,
            "adsb-setup",
            &tr!("adsb-ads-b-setup"),
            [400.0, 420.0],
            self.show_adsb_setup,
            |me, ui| me.adsb_setup_body(ui, &mut cfg),
        );
        let cfg = cfg.sane();
        if cfg != self.state.adsb {
            cmds.push(Command::SetAdsbConfig(cfg));
        }
        self.show_adsb_setup = open;
    }

    /// The three unit sets side by side: the operator picks the one that suits
    /// them and the active one stays lit.
    pub(in crate::app) fn air_units_picker(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 3.0;
            for u in sdroxide_types::AirUnits::ALL {
                let on = self.ui_settings.air_units == u;
                if crate::chrome::chip(ui, on, RichText::new(u.label().to_uppercase()).size(9.5))
                    .on_hover_text(u.describe())
                    .clicked()
                    && !on
                {
                    self.ui_settings.air_units = u;
                    u.set_current();
                    crate::app::persist::persist_ui_settings(&self.ui_settings);
                }
            }
        });
    }

    /// The ADS-B setup's body, split out so the shell can draw it in a window of
    /// its own.
    fn adsb_setup_body(&mut self, ui: &mut egui::Ui, cfg: &mut sdroxide_types::AdsbSettings) {
        crate::chrome::window_body_bg(ui);
        egui::Grid::new("adsb-cfg").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| {
            ui.label(tr!("adsbpanel-units"));
            self.air_units_picker(ui);
            ui.end_row();
            ui.label("");
            ui.label(RichText::new(tr!("adsbpanel-units-hint")).size(10.0).weak());
            ui.end_row();

            ui.label(tr!("adsb-drop-map-after"));
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut cfg.drop_map_s).range(2..=600).suffix(" s"));
                ui.label(RichText::new(tr!("adsb-without-position-report")).size(9.5).weak());
            });
            ui.end_row();
            ui.label("");
            ui.label(RichText::new(tr!("adsb-past-aircraft-comes-off-map")).size(10.0).weak());
            ui.end_row();

            ui.label(tr!("adsb-drop-list-after"));
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut cfg.drop_list_s)
                        .range(i64::from(cfg.drop_map_s)..=3600)
                        .suffix(" s"),
                );
                ui.label(RichText::new(tr!("adsb-nothing-heard-all")).size(9.5).weak());
            });
            ui.end_row();

            ui.label(tr!("adsb-trail-length"));
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut cfg.history_points)
                        .range(0..=sdroxide_types::ADSB_TRACK_MAX as u16)
                        .suffix(tr!("adsb-points")),
                );
                ui.label(
                    RichText::new(tr!("adsb-history-dots-behind-each-target")).size(9.5).weak(),
                );
            });
            ui.end_row();

            ui.label(tr!("adsb-speed-vector"));
            ui.horizontal(|ui| {
                ui.add(
                    egui::DragValue::new(&mut cfg.vector_minutes)
                        .speed(0.5)
                        .range(0.0..=10.0)
                        .suffix(" min"),
                );
                ui.label(RichText::new(tr!("adsb-how-far-ahead-leader-line")).size(9.5).weak());
            });
            ui.end_row();
            ui.label("");
            ui.label(
                RichText::new(tr!("adsb-one-minute-usual-radar-convention")).size(10.0).weak(),
            );
            ui.end_row();

            ui.label(tr!("adsb-track-most"));
            ui.add(
                egui::DragValue::new(&mut cfg.max_aircraft)
                    .range(10..=2000)
                    .suffix(tr!("adsb-aircraft-2")),
            );
            ui.end_row();
        });
        ui.separator();
        ui.label(RichText::new(tr!("adsb-aircraft-ground-are-placed-against")).size(10.0).weak());
    }
}

/// The column headings, drawn with the same offsets the rows use.
fn adsb_head_row(ui: &mut egui::Ui, have_home: bool, sort: &mut AdsbSort, desc: &mut bool) {
    const L: egui::Align2 = egui::Align2::LEFT_CENTER;
    const R: egui::Align2 = egui::Align2::RIGHT_CENTER;
    let cols = columns(ui.available_width(), have_home);
    let units = sdroxide_types::AirUnits::current();
    crate::app::panels::widgets::sort_head_row(
        ui,
        &[
            (cols.call, L, &tr!("adsb-call"), Some(AdsbSort::Callsign)),
            // The address is the callsign column's fallback, not an order
            // anybody wants a sky in.
            (cols.icao, L, &tr!("adsb-icao"), None),
            (cols.alt, R, units.altitude_head(), Some(AdsbSort::Altitude)),
            (cols.spd, R, units.speed_head(), Some(AdsbSort::Speed)),
            (cols.trk, R, "TRK", None),
            (cols.vs, R, units.vertical_head(), None),
            (cols.sqk, R, "SQK", None),
            (cols.sig, R, "SIG", Some(AdsbSort::Signal)),
            (cols.range, R, units.distance_head(), Some(AdsbSort::Range)),
            (cols.age, R, &tr!("adsbpanel-age"), Some(AdsbSort::Heard)),
        ],
        sort,
        desc,
    );
}

/// Column x offsets, in points from the left of a row.
struct Cols {
    call: f32,
    icao: f32,
    alt: f32,
    spd: f32,
    sig: f32,
    trk: f32,
    vs: f32,
    sqk: f32,
    range: f32,
    age: f32,
}

/// The right edge of the fixed left-hand group (callsign, address, altitude and
/// speed) plus a little air. Everything to its right is the tail, which is laid
/// out from the right edge and loses columns, least wanted first, rather than
/// printing over the speed.
const TAIL_START: f32 = 200.0;

/// Where each column sits, given the width available.
///
/// The left group is fixed so the table reads down as well as across. The tail
/// reads TRK, V/S, SQK, SIG, then the range and the age at the right edge;
/// when it does not fit, the columns come off in the order SIG, SQK, V/S, range,
/// TRK, age — the signal strength is the least asked for, the age the most.
/// `f32::NAN` means "not drawn".
fn columns(w: f32, have_home: bool) -> Cols {
    // Left to right, with the room each takes (its widest text and the gap).
    const TAIL: [(usize, f32); 6] =
        [(0, 24.0), (1, 36.0), (2, 30.0), (3, 30.0), (4, 36.0), (5, 30.0)];
    // Indices: 0 trk, 1 vs, 2 sqk, 3 sig, 4 range, 5 age.
    const GOES: [usize; 6] = [3, 2, 1, 4, 0, 5];
    let edge = w - 4.0;
    let mut present = [true; 6];
    present[4] = have_home;
    let room = |present: &[bool; 6]| -> f32 {
        TAIL.iter().filter(|(i, _)| present[*i]).map(|(_, slot)| slot).sum()
    };
    for drop in GOES {
        if edge - room(&present) + 6.0 >= TAIL_START {
            break;
        }
        present[drop] = false;
    }
    let mut x = [f32::NAN; 6];
    let mut at = edge;
    for (i, slot) in TAIL.iter().rev() {
        if present[*i] {
            x[*i] = at;
            at -= slot;
        }
    }
    Cols {
        call: 5.0,
        icao: 62.0,
        alt: 158.0,
        spd: 196.0,
        trk: x[0],
        vs: x[1],
        sqk: x[2],
        sig: x[3],
        range: x[4],
        age: x[5],
    }
}

/// One row of the aircraft table. Returns true if it was clicked.
fn adsb_row(
    ui: &mut egui::Ui,
    a: &AdsbAircraft,
    now: i64,
    selected: Option<u32>,
    home: Option<(f64, f64)>,
    drop_map_s: u16,
    i: usize,
) -> bool {
    const ROW_H: f32 = 17.0;
    const ACCENT_W: f32 = 2.5;

    let w = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(egui::vec2(w, ROW_H), egui::Sense::hover());
    if !ui.is_rect_visible(rect) {
        return false;
    }
    let is_sel = selected == Some(a.icao);
    let stale = a.pos_stale(now, drop_map_s);
    let p = ui.painter_at(rect);

    // A target whose position has aged out is greyed, because it is no longer
    // on the map and a row that looked live would be the only thing saying it
    // was. An emergency outranks everything.
    let (accent, ink) = if a.emergency.is_some() {
        (theme::HAZARD(), theme::HAZARD())
    } else if is_sel {
        (theme::YELLOW(), theme::YELLOW())
    } else if stale {
        (theme::gray(80), theme::gray(115))
    } else if a.on_ground {
        (theme::GREEN(), theme::CYAN())
    } else {
        (theme::CYAN_DIM(), theme::CYAN())
    };
    let dim = if stale { theme::gray(95) } else { theme::gray(140) };
    if is_sel {
        p.rect_filled(rect, 0.0, theme::gray(34));
    }
    p.rect_filled(
        egui::Rect::from_min_max(
            rect.left_top(),
            egui::pos2(rect.left() + ACCENT_W, rect.bottom()),
        ),
        0.0,
        accent,
    );

    let cols = columns(w, home.is_some());
    let mono = egui::FontId::monospace(10.5);
    let small = egui::FontId::monospace(9.5);
    let y = rect.center().y;
    let put = |x: f32, align: egui::Align2, text: String, font: egui::FontId, c| {
        if x.is_nan() || text.is_empty() {
            return;
        }
        p.text(egui::pos2(rect.left() + x, y), align, text, font, c);
    };

    put(cols.call, egui::Align2::LEFT_CENTER, a.label(), mono.clone(), ink);
    // The address, but only where it is not already the label — repeating it
    // would take a column and say nothing.
    if !a.callsign.is_empty() {
        put(cols.icao, egui::Align2::LEFT_CENTER, a.hex(), small.clone(), dim);
    }
    put(cols.alt, egui::Align2::RIGHT_CENTER, a.fmt_altitude(), mono.clone(), ink);
    put(cols.spd, egui::Align2::RIGHT_CENTER, a.fmt_speed(), mono.clone(), ink);
    put(cols.sig, egui::Align2::RIGHT_CENTER, format!("{:.0}", a.rssi_dbfs), small.clone(), dim);
    put(
        cols.trk,
        egui::Align2::RIGHT_CENTER,
        a.track_deg.map(|t| format!("{t:03.0}")).unwrap_or_default(),
        small.clone(),
        dim,
    );
    put(
        cols.vs,
        egui::Align2::RIGHT_CENTER,
        a.vertical_rate_fpm
            .filter(|v| v.abs() >= 64)
            .map(|v| sdroxide_types::AirUnits::current().vertical_rate_bare(v))
            .unwrap_or_default(),
        small.clone(),
        dim,
    );
    put(cols.sqk, egui::Align2::RIGHT_CENTER, a.fmt_squawk(), small.clone(), dim);
    if let (Some(h), Some((lat, lon))) = (home, a.lat.zip(a.lon)) {
        put(
            cols.range,
            egui::Align2::RIGHT_CENTER,
            format!(
                "{:.0}",
                sdroxide_types::AirUnits::current()
                    .distance_value(sdroxide_types::distance_km(h, (lat, lon)))
            ),
            small.clone(),
            dim,
        );
    }
    put(cols.age, egui::Align2::RIGHT_CENTER, fmt_age(now - a.last_at), small, dim);

    // One click target, the whole row wide, registered after everything above
    // it — which is what makes the callsign as clickable as the empty space.
    let hit = ui.interact(rect, ui.id().with(("adsb-row", i)), egui::Sense::click());
    hit.on_hover_text(match &a.category {
        Some(c) => format!("{} — {c}", a.hex()),
        None => a.hex(),
    })
    .clicked()
}

/// Order the table. A missing value always sorts last, whichever way the arrow
/// points: an aircraft that has not said its altitude yet is not the lowest one
/// in the sky.
fn sort_rows(rows: &mut [&AdsbAircraft], by: AdsbSort, desc: bool, home: Option<(f64, f64)>) {
    let key = |a: &AdsbAircraft| -> Option<f64> {
        match by {
            AdsbSort::Heard => Some(a.last_at as f64),
            AdsbSort::Callsign => None,
            AdsbSort::Altitude => a.altitude_ft.map(f64::from),
            AdsbSort::Speed => a.ground_speed_kt.map(f64::from),
            AdsbSort::Range => {
                let h = home?;
                let (lat, lon) = a.lat.zip(a.lon)?;
                // Negated so "descending" is nearest-first, which is the way
                // round anybody actually wants a range column.
                Some(-sdroxide_types::distance_km(h, (lat, lon)))
            }
            AdsbSort::Signal => Some(f64::from(a.rssi_dbfs)),
        }
    };
    if by == AdsbSort::Callsign {
        rows.sort_by(|x, y| {
            let o = x.label().cmp(&y.label());
            if desc { o.reverse() } else { o }
        });
        return;
    }
    rows.sort_by(|x, y| {
        match (key(x), key(y)) {
            (Some(a), Some(b)) => {
                let o = a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal);
                if desc { o.reverse() } else { o }
            }
            // Missing last, both ways round.
            (Some(_), None) => std::cmp::Ordering::Less,
            (None, Some(_)) => std::cmp::Ordering::Greater,
            (None, None) => std::cmp::Ordering::Equal,
        }
    });
}

/// A counter, short enough that it cannot outgrow its slot.
///
/// The frame and preamble counts run for as long as the decoder does — an
/// afternoon on a busy sector is millions — and a header readout that grew a
/// digit an hour would eventually push everything beside it off the row.
fn count(n: u64) -> String {
    let m = n as f64;
    if n < 10_000 {
        n.to_string()
    } else if n < 995_000 {
        format!("{:.0}k", m / 1e3)
    } else if n < 999_500_000 {
        format!("{:.1}M", m / 1e6)
    } else {
        format!("{:.0}G", m / 1e9)
    }
}

/// A readout in a slot of fixed width, so a number that grows a digit cannot
/// re-flow the header.
fn slot(ui: &mut egui::Ui, w: f32, text: &str, color: egui::Color32) {
    let (rect, _) = ui.allocate_exact_size(egui::vec2(w, 16.0), egui::Sense::hover());
    if !ui.is_rect_visible(rect) {
        return;
    }
    ui.painter_at(rect).text(
        egui::pos2(rect.left(), rect.center().y),
        egui::Align2::LEFT_CENTER,
        text,
        egui::FontId::monospace(10.0),
        color,
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ac(icao: u32, call: &str, alt: Option<i32>, last: i64) -> AdsbAircraft {
        let mut a = AdsbAircraft::new(icao, last);
        a.callsign = call.to_string();
        a.altitude_ft = alt;
        a.last_at = last;
        a
    }

    /// An aircraft that has not reported an altitude yet is not the lowest one
    /// in the sky — it sorts last whichever way the column is pointing.
    #[test]
    fn a_missing_value_sorts_last_in_both_directions() {
        let a = ac(1, "AAA", Some(35_000), 10);
        let b = ac(2, "BBB", None, 20);
        let c = ac(3, "CCC", Some(3_000), 30);
        for desc in [true, false] {
            let mut rows = vec![&a, &b, &c];
            sort_rows(&mut rows, AdsbSort::Altitude, desc, None);
            assert_eq!(rows.last().unwrap().icao, 2, "desc={desc}");
        }
    }

    /// Nearest first is what a range column is for, so "descending" — the
    /// default direction for every other column — has to mean nearest here.
    #[test]
    fn the_range_column_puts_the_nearest_aircraft_at_the_top() {
        let mut near = ac(1, "NEAR", None, 0);
        near.lat = Some(48.3);
        near.lon = Some(16.4);
        let mut far = ac(2, "FAR", None, 0);
        far.lat = Some(52.0);
        far.lon = Some(4.0);
        let mut rows = vec![&far, &near];
        sort_rows(&mut rows, AdsbSort::Range, true, Some((48.2, 16.37)));
        assert_eq!(rows[0].icao, 1);
    }

    /// The header counters run all session; they must not grow.
    #[test]
    fn a_counter_never_outgrows_its_slot() {
        for n in [0u64, 9_999, 10_000, 999_999, 1_000_000, 999_999_999, 42_000_000_000] {
            assert!(count(n).len() <= 6, "{n} formatted as {}", count(n));
        }
        assert_eq!(count(1_234), "1234");
        assert_eq!(count(12_345), "12k");
        assert_eq!(count(999_999_999), "1G");
        assert_eq!(count(4_500_000), "4.5M");
    }

    /// Every column, at the widest thing it can hold, against every other one:
    /// no two may touch at any width the panel can be dragged to, and nothing may
    /// run off either edge. The earlier version of this test only looked at the
    /// right-hand five, which is how GS and TRK came to print over each other.
    #[test]
    fn no_two_columns_ever_overprint_at_any_width() {
        let mut w = 200.0f32;
        while w < 900.0 {
            for have_home in [true, false] {
                let c = columns(w, have_home);
                // (name, anchor, widest text in points, right-aligned)
                let mut spans: Vec<(&str, f32, f32)> = Vec::new();
                let mut add = |name, x: f32, width: f32, right: bool| {
                    if !x.is_nan() {
                        spans.push(if right { (name, x - width, x) } else { (name, x, x + width) });
                    }
                };
                add("call", c.call, 52.0, false);
                add("icao", c.icao, 36.0, false);
                add("alt", c.alt, 33.0, true);
                add("spd", c.spd, 25.0, true);
                add("trk", c.trk, 18.0, true);
                add("vs", c.vs, 30.0, true);
                add("sqk", c.sqk, 24.0, true);
                add("sig", c.sig, 24.0, true);
                add("range", c.range, 30.0, true);
                add("age", c.age, 24.0, true);
                spans.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
                for pair in spans.windows(2) {
                    assert!(
                        pair[1].1 - pair[0].2 >= 1.0,
                        "{} and {} collide at width {w} (home={have_home}): {pair:?}",
                        pair[0].0,
                        pair[1].0
                    );
                }
                for (name, l, r) in &spans {
                    assert!(*l >= 0.0 && *r <= w, "{name} leaves the row at width {w}");
                }
            }
            w += 3.0;
        }
    }

    /// Reading left to right the table is CALL ICAO ALT GS TRK V/S SQK SIG, then
    /// range and age: the signal strength is the least wanted, so it is the
    /// first to go and sits next to the range it belongs with.
    #[test]
    fn the_columns_keep_their_order_and_the_signal_goes_first() {
        let c = columns(900.0, true);
        let order = [c.call, c.icao, c.alt, c.spd, c.trk, c.vs, c.sqk, c.sig, c.range, c.age];
        assert!(order.iter().all(|x| !x.is_nan()), "{order:?}");
        assert!(order.windows(2).all(|p| p[0] < p[1]), "{order:?}");
        let mut w = 900.0f32;
        let mut gone = Vec::new();
        while w > 200.0 {
            let c = columns(w, true);
            for (n, x) in
                [("sig", c.sig), ("sqk", c.sqk), ("vs", c.vs), ("range", c.range), ("trk", c.trk)]
            {
                if x.is_nan() && !gone.contains(&n) {
                    gone.push(n);
                }
            }
            w -= 3.0;
        }
        assert_eq!(gone, ["sig", "sqk", "vs", "range", "trk"]);
    }
}
