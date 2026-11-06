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

use crate::app::SdroxideApp;
use crate::sonde_map::{self, Phase};

/// The commands the SONDE button sends: switch the decoder on if it is off and
/// bring the dial to its channel, so the channel is inside the sample window.
pub(in crate::app) fn button_commands(
    cfg: &SondeSettings,
    vfo: sdroxide_types::Vfo,
) -> Vec<Command> {
    let mut out = Vec::new();
    if !cfg.enabled {
        out.push(Command::SetSondeConfig(SondeSettings { enabled: true, ..*cfg }));
    }
    out.push(Command::SetVfo { vfo, hz: cfg.frequency_hz });
    out
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
            ui.label(
                RichText::new(tr!("sonde-off-note")).size(11.0).color(crate::theme::YELLOW()),
            );
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
        self.sonde_online.tick(ui.ctx(), home, now);
        ui.horizontal_wrapped(|ui| {
            ui.checkbox(
                &mut self.sonde_online.on,
                RichText::new(tr!("sonde-online")).size(11.0),
            )
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
                    ui.label(RichText::new(n).size(10.5).weak());
                }
            }
        });

        // The table is a few rows; the map takes the rest.
        let rows = sondes.len().min(4) as f32;
        let table_h = if sondes.is_empty() { 0.0 } else { 22.0 + rows * 18.0 };
        let map_h = (h - 60.0 - table_h).max(100.0);
        let night = self.night_texture(ui.ctx());
        crate::sonde_map::show(
            ui,
            &mut self.sonde_map,
            &sondes,
            &self.sonde_online.sondes,
            home,
            now,
            night,
            map_h,
        );

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
                for s in &sondes {
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
                        self.sonde_map.selected = Some(s.serial.clone());
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
            self.sonde_export_row(ui, &sondes, now);
        }
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
        let cmds = button_commands(&off, sdroxide_types::Vfo::A);
        assert!(matches!(cmds[0], Command::SetSondeConfig(c) if c.enabled));
        assert!(matches!(
            cmds[1],
            Command::SetVfo { hz, .. } if hz == sdroxide_types::SONDE_DEFAULT_HZ
        ));
    }

    #[test]
    fn a_running_decoder_is_not_switched_on_again() {
        let on = SondeSettings { enabled: true, frequency_hz: 404_500_000.0, scan: false };
        let cmds = button_commands(&on, sdroxide_types::Vfo::A);
        assert_eq!(cmds.len(), 1, "only the retune");
        assert!(matches!(cmds[0], Command::SetVfo { hz, .. } if hz == 404_500_000.0));
    }

    #[test]
    fn an_age_reads_in_the_unit_that_fits() {
        assert_eq!(age(12), "12 s");
        assert_eq!(age(300), "5 min");
        assert_eq!(age(7300), "2 h");
        assert_eq!(age(-5), "0 s");
    }
}
