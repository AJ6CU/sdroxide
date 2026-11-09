//! The radiosonde view of the AIRCRAFT window: a status line, the channel, the
//! map of every sonde heard with its track, and a table under it.
//!
//! Like Cards it is a view, not a radio mode: the SONDE button switches the
//! decoder on, brings the dial to the sonde channel (the lane works off the
//! sample window, so the window has to contain it) and shows this. State comes
//! from [`sdroxide_types::SondeStatus`]; the settings are edited as a copy and
//! diffed at the end, the convention the HFDL and AIS panels keep.

use crate::i18n::tr;
use eframe::egui::{self, RichText};
use sdroxide_types::{Command, SONDE_BAND_HZ, SondeSettings, SondeTrack};

use crate::app::{SdroxideApp, sonde_card};
use crate::sonde_map::{self, Phase};

/// The commands the SONDE button sends: leave a mode that holds the front end
/// elsewhere, switch the decoder on if it is off and bring the dial to its
/// channel, so the channel is inside the sample window.
///
/// ADS-B, VDL2, AIS, DAB and HFDL each park the receiver on their own channel
/// and own the panel under the waterfall; staying in one of them left that
/// panel on show and the dial pulled back to it. They are left for NFM, which
/// is what a sonde's FSK sounds like and which asks nothing of the window.
pub(in crate::app) fn button_commands(
    cfg: &SondeSettings,
    vfo: sdroxide_types::Vfo,
    mode: sdroxide_types::Mode,
) -> Vec<Command> {
    let mut out = Vec::new();
    if mode.is_wideband_lane() || mode.is_hfdl() {
        out.push(Command::SetModeListen {
            rx: sdroxide_types::RxId::Main,
            mode: sdroxide_types::Mode::Nfm,
        });
    }
    if !cfg.enabled {
        out.push(Command::SetSondeConfig(SondeSettings { enabled: true, ..*cfg }));
    }
    out.push(Command::SetVfo { vfo, hz: cfg.frequency_hz });
    out
}

/// The map never gets less than this many points of height; under it the
/// view scrolls.
const MAP_MIN_H: f32 = 280.0;

/// A card's rows cut in two, the first half the longer by one when odd.
fn card_halves(rows: &[(String, String)]) -> (&[(String, String)], &[(String, String)]) {
    rows.split_at(rows.len().div_ceil(2))
}

/// A short age: `12 s`, `4 min`, `2 h`.
fn age(secs: i64) -> String {
    let s = secs.max(0);
    if s < 90 {
        format!("{s} s")
    } else if s < 5400 {
        format!("{} min", s / 60)
    } else {
        format!("{} h", s / 3600)
    }
}

/// The widest the card is drawn, points.
const CARD_W: f32 = 520.0;

/// What was pressed in the card.
enum CardAct {
    None,
    Close,
    Detach,
    Attach,
}

/// Draw a card: its title, the buttons, then the rows in two blocks side by
/// side (half the height of one long column).
fn draw_card(ui: &mut egui::Ui, card: &sonde_card::Card, detached: bool) -> CardAct {
    let mut act = CardAct::None;
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(&card.title).size(12.0).strong());
        if ui.small_button(tr!("sonde-card-close")).clicked() {
            act = CardAct::Close;
        }
        let (label, next) = if detached {
            (tr!("sonde-card-attach"), CardAct::Attach)
        } else {
            (tr!("sonde-card-detach"), CardAct::Detach)
        };
        if ui.small_button(label).clicked() {
            act = next;
        }
        for (site, url) in sonde_card::links(&card.serial) {
            if ui.small_button(sonde_card::open_label(site)).on_hover_text(&url).clicked()
                && !crate::download::open_external(&url)
            {
                tracing::warn!(%url, "no handler for the sonde page");
            }
        }
    });
    let (left, right) = card_halves(&card.rows);
    ui.columns(2, |cols| {
        for (i, rows) in [left, right].into_iter().enumerate() {
            egui::Grid::new(("sonde_card", i)).num_columns(2).spacing([14.0, 2.0]).show(
                &mut cols[i],
                |ui| {
                    for (label, value) in rows {
                        ui.label(RichText::new(label).size(10.5).weak());
                        ui.label(RichText::new(value).size(10.5));
                        ui.end_row();
                    }
                },
            );
        }
    });
    act
}

impl SdroxideApp {
    pub(in crate::app) fn aircraft_sonde_pane(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        h: f32,
    ) {
        let now = crate::time::now_unix();
        let mut cfg = self.state.sonde;
        let status = self.sonde_status.as_deref().cloned();
        let (running, level, frames) = match &status {
            Some(s) => (s.running, Some(s.level_dbfs), s.frames),
            None => (false, None, 0),
        };
        let sondes: Vec<SondeTrack> = status.map(|s| s.sondes).unwrap_or_default();

        ui.horizontal_wrapped(|ui| {
            let (txt, ink) = if running {
                (tr!("sonde-decoding"), crate::theme::GREEN())
            } else {
                (tr!("sonde-decoding-off"), crate::theme::HAZARD())
            };
            ui.label(RichText::new(txt).size(11.0).strong().color(ink));
            if let Some(l) = level.filter(|l| *l > -90.0) {
                ui.label(RichText::new(format!("{l:.0} dBFS")).size(11.0).weak());
            }
            ui.label(RichText::new(tr!("sonde-count", n = sondes.len())).size(11.0));
            ui.label(RichText::new(tr!("sonde-frames", n = frames)).size(11.0).weak());
            ui.separator();
            ui.label(RichText::new(tr!("sonde-channel")).size(11.0));
            let mut mhz = cfg.frequency_hz / 1e6;
            let r = ui
                .add(
                    egui::DragValue::new(&mut mhz)
                        .range(SONDE_BAND_HZ.0 / 1e6..=SONDE_BAND_HZ.1 / 1e6)
                        .speed(0.01)
                        .fixed_decimals(3)
                        .suffix(" MHz"),
                )
                .on_hover_text(tr!("sonde-channel-hover"));
            if r.changed() {
                cfg.frequency_hz = (mhz * 1e6).round();
            }
            ui.checkbox(&mut cfg.scan, RichText::new(tr!("sonde-scan")).size(11.0))
                .on_hover_text(tr!("sonde-scan-hover"));
        });
        if !cfg.enabled {
            ui.label(RichText::new(tr!("sonde-off-note")).size(11.0).color(crate::theme::YELLOW()));
        }
        if cfg != self.state.sonde {
            cmds.push(Command::SetSondeConfig(cfg));
            // The lane mixes from the sample window: follow the channel.
            if (cfg.frequency_hz - self.state.sonde.frequency_hz).abs() >= 1.0 {
                cmds.push(Command::SetVfo { vfo: self.state.active_vfo, hz: cfg.frequency_hz });
            }
        }

        // Sondes other stations hear: read-only, off until the operator says.
        let home = self.hfdl_home_for_maps();
        let peer = self.ctrl.peer_url();
        self.sonde_online.tick(ui.ctx(), home, now, peer.as_deref());
        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut self.sonde_online.on, RichText::new(tr!("sonde-online")).size(11.0))
                .on_hover_text(tr!("sonde-online-hover"));
            if self.sonde_online.on {
                let note = if home.is_none() {
                    Some(tr!("sonde-online-no-grid"))
                } else if self.sonde_online.failed {
                    Some(tr!("sonde-online-failed"))
                } else {
                    Some(tr!("sonde-online-count", n = self.sonde_online.sondes.len()))
                };
                if let Some(n) = note {
                    let r = ui.label(RichText::new(n).size(10.5).weak());
                    if let (true, Some(why)) = (self.sonde_online.failed, &self.sonde_online.why) {
                        r.on_hover_text(why.text());
                    }
                }
            }
        });

        // The table is a few rows; the map takes the rest, never less than
        // `MAP_MIN_H`: below that the window scrolls instead of squeezing the
        // map (and cutting the card off at the bottom, as it used to).
        let rows = sondes.len().min(4) as f32;
        let table_h = if sondes.is_empty() { 0.0 } else { 22.0 + rows * 18.0 };
        let map_h = (h - 60.0 - table_h).max(MAP_MIN_H);
        let night = self.night_texture(ui.ctx());
        egui::ScrollArea::vertical().id_salt("sonde_scroll").auto_shrink([false, false]).show(
            ui,
            |ui| {
                self.sonde_scroll_body(ui, &sondes, home, night, map_h, now);
            },
        );
        // The card is not part of the scrolling page: it is pinned beside its
        // sonde on the map, or a window of its own.
        let ctx = ui.ctx().clone();
        self.sonde_card_view(&ctx, &sondes, home, now);
    }

    fn sonde_scroll_body(
        &mut self,
        ui: &mut egui::Ui,
        sondes: &[SondeTrack],
        home: Option<(f64, f64)>,
        night: Option<egui::TextureId>,
        map_h: f32,
        now: i64,
    ) {
        let clicked = crate::sonde_map::show(
            ui,
            &mut self.sonde_map,
            sondes,
            &self.sonde_online.sondes,
            home,
            now,
            night,
            map_h,
        );
        if clicked.is_some() {
            // A picked sonde should not wait for the minute to be up.
            self.sonde_online.hurry(now);
        }

        if !sondes.is_empty() {
            egui::Grid::new("sonde_table").striped(true).num_columns(10).show(ui, |ui| {
                for head in [
                    tr!("sonde-col-serial"),
                    tr!("sonde-col-alt"),
                    tr!("sonde-col-climb"),
                    tr!("sonde-col-speed"),
                    tr!("sonde-channel"),
                    tr!("sonde-col-battery"),
                    tr!("sonde-col-temp"),
                    tr!("sonde-col-humidity"),
                    tr!("sonde-col-pressure"),
                    tr!("sonde-col-heard"),
                ] {
                    ui.label(RichText::new(head).size(10.0).weak());
                }
                ui.end_row();
                for s in sondes {
                    let selected = self.sonde_map.selected.as_deref() == Some(s.serial.as_str());
                    let ink = match sonde_map::phase(s, now) {
                        Phase::Silent => crate::theme::gray(120),
                        _ => crate::theme::gray(210),
                    };
                    let name = RichText::new(format!("{} {}", s.model, s.serial))
                        .size(10.5)
                        .strong()
                        .color(ink);
                    if ui.selectable_label(selected, name).clicked() {
                        self.sonde_map.select(s.serial.clone());
                    }
                    let cell = |ui: &mut egui::Ui, t: String| {
                        ui.label(RichText::new(t).size(10.5).color(ink));
                    };
                    cell(ui, s.alt_m.map_or("—".into(), |a| format!("{a:.0} m")));
                    cell(ui, s.climb_ms.map_or("—".into(), |c| format!("{c:+.1} m/s")));
                    cell(ui, s.speed_ms.map_or("—".into(), |v| format!("{:.0} km/h", v * 3.6)));
                    cell(ui, format!("{:.3} MHz", s.freq_hz / 1e6));
                    cell(ui, s.battery_v.map_or("—".into(), |v| format!("{v:.1} V")));
                    cell(ui, s.temp_c.map_or("—".into(), |t| format!("{t:.1} °C")));
                    cell(ui, s.humidity_pct.map_or("—".into(), |h| format!("{h:.0} %")));
                    cell(ui, s.pressure_hpa.map_or("—".into(), |p| format!("{p:.0} hPa")));
                    cell(ui, age(now - s.last_unix));
                    ui.end_row();
                }
            });
            self.sonde_export_row(ui, sondes, now);
        }
    }

    /// The card of the selected sonde: this radio's frames and SondeHub's line
    /// together when both know it, a kept copy when neither does any more.
    ///
    /// Pinned beside the sonde on the map like an aircraft's card, or, once the
    /// operator detaches it, a window of its own. It never sits under the map
    /// where the page has to be scrolled to find it.
    fn sonde_card_view(
        &mut self,
        ctx: &egui::Context,
        sondes: &[SondeTrack],
        home: Option<(f64, f64)>,
        now: i64,
    ) {
        let Some(sel) = self.sonde_map.selected.clone() else { return };
        // Esc closes it, unless a text field has the keyboard.
        if !ctx.egui_wants_keyboard_input()
            && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape))
        {
            self.sonde_map.deselect();
            return;
        }
        let live_radio = sondes.iter().find(|s| s.serial == sel);
        let live_hub = self.sonde_online.sondes.iter().find(|o| o.serial == sel);
        let live = self.sonde_map.kept.update(live_radio, live_hub);
        let radio = live_radio.or(self.sonde_map.kept.radio.as_ref());
        let hub = live_hub.or(self.sonde_map.kept.hub.as_ref());
        let Some(mut card) = sonde_card::compose(radio, hub, home, now) else { return };
        if !live {
            sonde_card::mark_stale(&mut card);
        }
        let act = if self.sonde_map.detached {
            self.sonde_card_window(ctx, &card)
        } else {
            self.sonde_card_pinned(ctx, &card)
        };
        match act {
            CardAct::None => {}
            CardAct::Close => self.sonde_map.deselect(),
            CardAct::Detach => self.sonde_map.detached = true,
            CardAct::Attach => self.sonde_map.detached = false,
        }
    }

    /// The card over the map, beside its sonde, kept inside the map.
    fn sonde_card_pinned(&mut self, ctx: &egui::Context, card: &sonde_card::Card) -> CardAct {
        let Some(bounds) = self.sonde_map.bounds else { return CardAct::None };
        let anchor = self.sonde_map.anchor.unwrap_or(bounds.center());
        let size = self.sonde_map.card_size.unwrap_or(egui::vec2(CARD_W, 230.0));
        let at = crate::adsb_map::pin_placement(anchor, size, bounds);
        let shown = egui::Area::new(egui::Id::new("sonde-pinned-card"))
            .order(egui::Order::Middle)
            .fixed_pos(at)
            .constrain_to(bounds)
            .show(ctx, |ui| {
                egui::Frame::popup(ui.style())
                    .show(ui, |ui| {
                        ui.set_max_width(CARD_W);
                        draw_card(ui, card, false)
                    })
                    .inner
            });
        self.sonde_map.card_size = Some(shown.response.rect.size());
        shown.inner
    }

    /// The card as a window the operator can move off the map and resize.
    fn sonde_card_window(&mut self, ctx: &egui::Context, card: &sonde_card::Card) -> CardAct {
        let mut open = true;
        let mut window = egui::Window::new(card.title.clone())
            .id(egui::Id::new("sonde-card-window"))
            .open(&mut open)
            .resizable(true)
            .default_width(CARD_W);
        if let Some(b) = self.sonde_map.bounds {
            window = window.default_pos(b.min + egui::vec2(24.0, 24.0));
        }
        let act = window
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| draw_card(ui, card, true)).inner
            })
            .and_then(|r| r.inner)
            .unwrap_or(CardAct::None);
        if open { act } else { CardAct::Close }
    }

    /// Save the selected sonde's flight (the newest if none is selected) as a
    /// KML track or a CSV table.
    fn sonde_export_row(&mut self, ui: &mut egui::Ui, sondes: &[SondeTrack], now: i64) {
        let target = self
            .sonde_map
            .selected
            .as_deref()
            .and_then(|sel| sondes.iter().find(|s| s.serial == sel))
            .or_else(|| sondes.first());
        let Some(s) = target else { return };
        ui.horizontal(|ui| {
            ui.label(RichText::new(tr!("sonde-export")).size(10.5).weak());
            for (ext, hover) in
                [("kml", tr!("sonde-export-kml-hover")), ("csv", tr!("sonde-export-csv-hover"))]
            {
                let b = ui.small_button(ext.to_ascii_uppercase()).on_hover_text(hover);
                if !b.clicked() {
                    continue;
                }
                let text = if ext == "kml" {
                    sdroxide_types::sonde_export::kml(s)
                } else {
                    sdroxide_types::sonde_export::csv(s)
                };
                self.sonde_map.note = Some(match text {
                    Some(text) => {
                        let name = sdroxide_types::sonde_export::file_name(s, now, ext);
                        crate::download::save(&name, text.as_bytes());
                        tr!("aircraft-track-kml-saved", name = name)
                    }
                    None => tr!("sonde-export-none"),
                });
            }
            if let Some(note) = &self.sonde_map.note {
                ui.label(RichText::new(note).size(10.5).color(crate::theme::gray(160)));
            }
        });
    }

    /// The operator's own position, from the grid in the digital-mode setup —
    /// the same source every map uses, so they never disagree about it.
    fn hfdl_home_for_maps(&self) -> Option<(f64, f64)> {
        let grid = self.digi_cfg_edit.my_grid.trim();
        (!grid.is_empty()).then(|| sdroxide_types::grid_to_latlon(grid)).flatten()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_button_switches_the_decoder_on_and_goes_to_its_channel() {
        let off = SondeSettings::default();
        let cmds = button_commands(&off, sdroxide_types::Vfo::A, sdroxide_types::Mode::Usb);
        assert!(matches!(cmds[0], Command::SetSondeConfig(c) if c.enabled));
        assert!(matches!(
            cmds[1],
            Command::SetVfo { hz, .. } if hz == sdroxide_types::SONDE_DEFAULT_HZ
        ));
    }

    #[test]
    fn the_button_leaves_the_aviation_modes_before_it_retunes() {
        use sdroxide_types::Mode;
        let on = SondeSettings { enabled: true, ..SondeSettings::default() };
        for mode in [Mode::Adsb, Mode::Vdl2, Mode::Hfdl, Mode::Ais, Mode::Dab] {
            let cmds = button_commands(&on, sdroxide_types::Vfo::A, mode);
            assert!(
                matches!(cmds[0], Command::SetModeListen { mode: Mode::Nfm, .. }),
                "{mode:?} must be left first"
            );
            assert!(matches!(cmds[1], Command::SetVfo { .. }));
        }
        let cmds = button_commands(&on, sdroxide_types::Vfo::A, Mode::Usb);
        assert!(matches!(cmds[0], Command::SetVfo { .. }), "other modes are left alone");
    }

    #[test]
    fn a_running_decoder_is_not_switched_on_again() {
        let on = SondeSettings { enabled: true, frequency_hz: 404_500_000.0, scan: false };
        let cmds = button_commands(&on, sdroxide_types::Vfo::A, sdroxide_types::Mode::Usb);
        assert_eq!(cmds.len(), 1, "only the retune");
        assert!(matches!(cmds[0], Command::SetVfo { hz, .. } if hz == 404_500_000.0));
    }

    #[test]
    fn a_card_is_cut_in_two_halves() {
        let rows: Vec<(String, String)> =
            (0..5).map(|i| (format!("l{i}"), format!("v{i}"))).collect();
        let (a, b) = card_halves(&rows);
        assert_eq!((a.len(), b.len()), (3, 2));
        let (a, b) = card_halves(&rows[..4]);
        assert_eq!((a.len(), b.len()), (2, 2));
        let (a, b) = card_halves(&[]);
        assert_eq!((a.len(), b.len()), (0, 0));
    }

    #[test]
    fn an_age_reads_in_the_unit_that_fits() {
        assert_eq!(age(12), "12 s");
        assert_eq!(age(300), "5 min");
        assert_eq!(age(7300), "2 h");
        assert_eq!(age(-5), "0 s");
    }
}
