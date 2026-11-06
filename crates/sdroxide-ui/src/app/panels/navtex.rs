//! The NAVTEX panel: the messages received, and the one arriving.
//!
//! Two panes, because they move at different rates and are read differently.
//! The list on the left is a file — a station's four-hourly cycle collapsed to
//! one entry per message, newest first, marked by what kind of warning it is.
//! The pane on the right is whatever is being copied *now*, which on a mode
//! with a ten-minute slot is the thing an operator is actually watching.
//!
//! There is nothing to transmit and no callsign to set, so there is no setup
//! window and no transmit half — the two controls are which tone sense the
//! receiver reads and whether to show the loose text below the message list.

use crate::i18n::tr;
use eframe::egui::{self, RichText};
use sdroxide_types::{Command, NavtexMessage, NavtexStatus, Vfo};

use crate::app::SdroxideApp;
use crate::theme;

/// The NAVTEX channels the panel offers, as dial frequencies — see
/// `panels::mode_dials`. The MF/HF service is the one a shortwave listener
/// reaches.
const CHANNELS: &[(f64, &str)] = &[
    (6_314_000.0, "6314"),
    (8_416_500.0, "8416.5"),
    (12_579_000.0, "12579"),
    (16_806_500.0, "16806.5"),
    (22_376_000.0, "22376"),
];

/// The message list's header row: the count, the sync state, what the time
/// diversity repaired, SAVE and REV. Returns whether REV was pressed; `rev` is
/// its state and `status` the app's `digi_status`, which SAVE writes from.
///
/// Wrapped: on a phone the row is wider than the screen, and a plain one ran
/// REV off its edge.
#[allow(clippy::too_many_arguments)]
pub(super) fn navtex_list_header(
    ui: &mut egui::Ui,
    st: &NavtexStatus,
    status: Option<&sdroxide_types::DigiStatus>,
    cmds: &mut Vec<Command>,
    rev: bool,
    cfg: &mut sdroxide_types::DigiConfig,
    seeded: bool,
) -> bool {
    let mut flip = false;
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(tr!("navtex-messages")).strong().color(theme::CYAN()));
        ui.label(RichText::new(format!("{}", st.messages.len())).weak());
        // The mode's own carrier detect: a constant-ratio code either
        // frames or it does not, and there is no in-between to show.
        if st.in_sync {
            ui.label(RichText::new(tr!("navtex-sync")).strong().color(theme::GREEN()))
                .on_hover_text(tr!("navtex-character-phase-locked-signal-being"));
        } else {
            ui.label(RichText::new(tr!("navtex-hunting")).weak())
                .on_hover_text(tr!("navtex-hunting-hint"));
        }
        // What the time diversity actually did, which is the only quality
        // figure a mode with no checksum has. All three counts, always: the
        // good ones are the number an operator judges the band by, and a
        // figure that only appears once something has gone wrong reads as a
        // broken counter rather than as a clean copy (F4NRM, #481).
        ui.label(
            RichText::new(tr!(
                "navtex-copies",
                direct = st.direct,
                repaired = st.repaired,
                lost = st.lost
            ))
            .size(10.0)
            .color(if st.lost > 0 { theme::YELLOW() } else { theme::CYAN_DIM() }),
        )
        .on_hover_text(tr!("navtex-characters-whose-first-copy-was"));
    });
    flip
}

impl SdroxideApp {
    pub(in crate::app) fn navtex_panel(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        panel_h: f32,
    ) {
        let st: Option<NavtexStatus> = self.digi_status.as_ref().and_then(|s| s.navtex.clone());
        let Some(st) = st else {
            ui.label(RichText::new(tr!("navtex-starting-navtex-receiver")).weak());
            return;
        };
        // A NAVTEX receiver driven into overload hears a wall of noise, not
        // characters, and the panel would only say "hunting".
        self.front_end_overload_warning(ui);
        let pane = self.phone_pane(ui, sdroxide_types::Mode::Navtex);

        // MF NAVTEX: the two medium-frequency services. The button is labelled
        // with the published channel frequency, but the VFO lands 1.710 kHz
        // below it (USB), because NAVTEX is a 170 Hz FSK signal 1.5 kHz above
        // the suppressed carrier.
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("MF").size(10.0).weak());
            for (label, vfo_hz) in [("490", 488_290.0), ("518", 516_290.0)] {
                let active_vfo_hz = match self.state.active_vfo {
                    Vfo::A => self.state.vfo_a_hz,
                    Vfo::B => self.state.vfo_b_hz,
                };
                let is_active = (active_vfo_hz - vfo_hz).abs() < 1.0;
                if crate::chrome::chip(
                    ui,
                    is_active,
                    RichText::new(format!("{label} kHz")).size(10.0),
                )
                .on_hover_text(tr!("navtex-tune-dial-usb-below-channel"))
                .clicked()
                {
                    cmds.push(Command::SetVfo { vfo: Vfo::A, hz: vfo_hz });
                }
            }
            ui.add_space(8.0);
            ui.label(RichText::new(tr!("aprs-channel")).size(10.0).weak());
            for (hz, label) in CHANNELS {
                let active_vfo_hz = match self.state.active_vfo {
                    Vfo::A => self.state.vfo_a_hz,
                    Vfo::B => self.state.vfo_b_hz,
                };
                let is_active = (active_vfo_hz - *hz).abs() < 1.0;
                if crate::chrome::chip(ui, is_active, RichText::new(*label).size(10.0))
                    .on_hover_text(tr!("acars-tune-dial-here"))
                    .clicked()
                {
                    cmds.push(Command::SetVfo { vfo: Vfo::A, hz: *hz });
                }
            }
            ui.add_space(8.0);
            if crate::chrome::chip(ui, self.digi_cfg_edit.navtex_reverse, RichText::new("REV").size(10.5))
                .on_hover_text(tr!("navtex-swap-mark-space-tones-signal"))
                .clicked()
            {
                self.digi_cfg_edit.navtex_reverse = !self.digi_cfg_edit.navtex_reverse;
                if self.digi_cfg_seeded { cmds.push(Command::SetDigiConfig(self.digi_cfg_edit.clone())); }
            }
        });

        // REV / save / clear RX on the next line
        ui.horizontal_wrapped(|ui| {
            crate::chrome::row_tail(ui, |ui| {
                crate::app::panels::clear_rx_chip_at(ui, cmds, !st.messages.is_empty() || st.live.is_some());
                crate::app::panels::save_rx_chip_for(ui, self.digi_status.as_ref());
            });
        });

        // Sliders stacked vertically in a sidebar, so the message area gets
        // the full remaining height.
        let h = ui.available_height();
        ui.allocate_ui(egui::vec2(ui.available_width(), h), |ui| {
            ui.set_min_height(h);
            ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(160.0);
                ui.set_min_height(h);

                ui.label(RichText::new(tr!("panels-sensitivity")).size(10.0).weak());
                let mut v = self.digi_cfg_edit.navtex_rx_sensitivity;
                let resp = crate::chrome::slider(ui, egui::Slider::new(&mut v, -5..=5).fixed_decimals(0))
                    .on_hover_text(tr!("navtex-sensitivity-hint"));
                if resp.changed() {
                    self.digi_cfg_edit.navtex_rx_sensitivity = v;
                    self.digi_cfg_edit.navtex_rx_auto = v == 0;
                    if self.digi_cfg_seeded { cmds.push(Command::SetDigiConfig(self.digi_cfg_edit.clone())); }
                }
                if crate::chrome::chip(ui, self.digi_cfg_edit.navtex_rx_auto, tr!("sstv-auto"))
                    .on_hover_text(tr!("navtex-auto-hint"))
                    .clicked()
                {
                    self.digi_cfg_edit.navtex_rx_auto = true;
                    self.digi_cfg_edit.navtex_rx_sensitivity = 0;
                    if self.digi_cfg_seeded { cmds.push(Command::SetDigiConfig(self.digi_cfg_edit.clone())); }
                }
            });
            ui.separator();
            if pane.is_none_or(|p| p == 0) {
                ui.vertical(|ui| {
                    ui.set_min_height(h);
                    if pane.is_none() {
                        ui.set_width(ui.available_width() * 0.46);
                    }
                    self.navtex_list(ui, cmds, &st, panel_h);
                });
            }
            if pane.is_none() {
                ui.separator();
            }
            if pane.is_none_or(|p| p == 1) {
                ui.vertical(|ui| {
                    ui.set_min_height(h);
                    self.navtex_reading(ui, &st, panel_h);
                });
            }
        });
        });
    }

    /// The messages received, newest first.
    fn navtex_list(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        st: &NavtexStatus,
        panel_h: f32,
    ) {
        let rev_pressed = navtex_list_header(
            ui,
            st,
            self.digi_status.as_ref(),
            cmds,
            self.digi_cfg_edit.navtex_reverse,
            &mut self.digi_cfg_edit,
            self.digi_cfg_seeded,
        );
        if rev_pressed {
            self.digi_cfg_edit.navtex_reverse = !self.digi_cfg_edit.navtex_reverse;
            if self.digi_cfg_seeded {
                cmds.push(Command::SetDigiConfig(self.digi_cfg_edit.clone()));
            }
        }

        egui::ScrollArea::vertical()
            .id_salt("navtex-list")
            .max_height(ui.available_height().max(60.0))
            .show(ui, |ui| {
                if st.messages.is_empty() && st.live.is_none() {
                    ui.label(RichText::new(tr!("navtex-nothing-yet-station-transmits-ten")).weak());
                }
                for (i, m) in st.messages.iter().enumerate().rev() {
                    let on = self.navtex_open == Some(i);
                    let head =
                        format!("{}{}{:02}  {}", m.station, m.kind, m.serial, m.kind_label());
                    let colour = if m.is_mandatory() {
                        // The three classes a ship's receiver may not switch
                        // off. sdroxide does not offer to hide them either, and
                        // they are marked so an eye finds them first.
                        theme::ALERT()
                    } else if m.complete {
                        theme::CYAN()
                    } else {
                        theme::YELLOW()
                    };
                    let mut text = RichText::new(head).monospace().color(colour);
                    if on {
                        text = text.strong();
                    }
                    if ui.selectable_label(on, text).clicked() {
                        self.navtex_open = if on { None } else { Some(i) };
                    }
                    let first = m.text.lines().find(|l| !l.trim().is_empty()).unwrap_or("");
                    let mut line = format!("      {}", first.trim());
                    line.truncate(60);
                    ui.label(RichText::new(line).size(10.0).weak());
                    if !m.complete || m.lost > 0 {
                        let mut why = Vec::new();
                        if !m.complete {
                            why.push(tr!("navtex-cut-short").to_string());
                        }
                        if m.lost > 0 {
                            why.push(tr!("navtex-characters-lost", lost = m.lost));
                        }
                        ui.label(
                            RichText::new(format!("      {}", why.join(", ")))
                                .size(9.5)
                                .color(theme::YELLOW()),
                        );
                    }
                }
            });
    }

    /// The message being read: the one selected, or the one arriving.
    fn navtex_reading(&mut self, ui: &mut egui::Ui, st: &NavtexStatus, panel_h: f32) {
        let selected: Option<&NavtexMessage> =
            self.navtex_open.and_then(|i| st.messages.get(i)).or(st.live.as_ref());
        ui.horizontal(|ui| match selected {
            Some(m) if self.navtex_open.is_some() => {
                ui.label(RichText::new(tr!("navtex-message")).strong().color(theme::CYAN()));
                ui.label(
                    RichText::new(format!("{}{}{:02}", m.station, m.kind, m.serial))
                        .monospace()
                        .color(theme::CYAN_DIM()),
                );
                ui.label(RichText::new(m.kind_label()).size(10.5).weak());
                // The time the message states, pulled out of the body: a
                // warning is read against when it was issued, and finding the
                // figure in a column of positions by eye is the tedious part.
                // Display only — the system clock is never set from it.
                if let Some((h, min)) = m.body_time_utc() {
                    ui.label(
                        RichText::new(format!("{h:02}{value:02} UTC", h = h, value = min))
                            .monospace()
                            .size(10.5)
                            .color(theme::GREEN()),
                    )
                    .on_hover_text(tr!("navtex-time-day-stated-message-utc"));
                }
            }
            Some(_) => {
                ui.label(RichText::new(tr!("navtex-receiving")).strong().color(theme::ALERT()));
            }
            None => {
                ui.label(RichText::new(tr!("navtex-monitor")).strong().color(theme::CYAN()));
                ui.label(
                    RichText::new(tr!("navtex-everything-decoded-message-not")).size(10.5).weak(),
                );
            }
        });

        let body = match selected {
            Some(m) => m.text.clone(),
            // Only show the loose text when the receiver is actually in
            // sync: otherwise it is noise decoded as characters.
            None => {
                if st.in_sync {
                    st.text.clone()
                } else {
                    String::new()
                }
            }
        };
        egui::ScrollArea::vertical()
            .id_salt("navtex-body")
            .max_height(ui.available_height().max(60.0))
            .stick_to_bottom(selected.is_none() || self.navtex_open.is_none())
            .show(ui, |ui| {
                if body.trim().is_empty() {
                    ui.label(RichText::new("—").weak());
                }
                // Monospace and unwrapped: a NAVTEX message is laid out in
                // columns by the station that sent it — positions, times and
                // tables — and reflowing it destroys the only formatting it has.
                ui.label(RichText::new(body).monospace().size(11.5));
            });
    }
}
