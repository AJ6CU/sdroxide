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

/// The HF NAVTEX channels the panel offers, by their assigned frequency. The
/// dial goes [`NAVTEX_TONE_HZ`] below it — see [`dial_hz`].
const CHANNELS: &[(f64, &str)] = &[
    (6_314_000.0, "6314"),
    (8_416_500.0, "8416.5"),
    (12_579_000.0, "12579"),
    (16_806_500.0, "16806.5"),
    (22_376_000.0, "22376"),
];

/// The two medium-frequency services every coast station shares.
const MF_CHANNELS: &[(f64, &str)] = &[(490_000.0, "490"), (518_000.0, "518")];

/// What is sent on one of [`LISTED`].
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Service {
    /// 518 kHz: the international service, in English.
    International,
    /// 490 kHz: the same stations in their own language.
    National,
    /// SITOR-B on a regional frequency — the decoder reads it.
    Navtex,
    /// A coast station's weather bulletin in Morse. Listed with NAVTEX
    /// because it is the same listener's other reason to be on this band;
    /// this decoder does not read it.
    Cw,
}

/// One row of the frequency menu.
struct Listed {
    channel_hz: f64,
    /// The channel as a schedule prints it, in kHz.
    khz: &'static str,
    /// Country (ITU code) and stations. Names, so not translated.
    who: &'static str,
    service: Service,
}

/// A CW bulletin's dial offset: the carrier 600 Hz up the passband, the note
/// the published list recommends for a receiver left in USB.
const CW_NOTE_HZ: f64 = 600.0;

/// Every frequency the DX Info Centre NAVTEX list carries (W. Hepburn,
/// dxinfocentre.com/navtex.htm, edition of 2026-09-26), in frequency order.
///
/// 500 kHz is left out on purpose: it is NAVDAT, a 10 kHz OFDM service still
/// being planned, and nothing here could tune or read it.
const LISTED: &[Listed] = &[
    Listed {
        channel_hz: 424_000.0,
        khz: "424",
        who: "JPN · Naha, Moji, Yokohama, Otaru, Kushiro",
        service: Service::Navtex,
    },
    Listed {
        channel_hz: 426_000.0,
        khz: "426",
        who: "USA · KPH Point Reyes",
        service: Service::Cw,
    },
    Listed {
        channel_hz: 446_000.0,
        khz: "446",
        who: "RUS · UGE Arkhangelsk",
        service: Service::Cw,
    },
    Listed {
        channel_hz: 446_500.0,
        khz: "446.5",
        who: "ROU · YQI Constanza",
        service: Service::Cw,
    },
    Listed { channel_hz: 472_000.0, khz: "472", who: "USA · WNE Stoneham", service: Service::Cw },
    Listed {
        channel_hz: 478_000.0,
        khz: "478",
        who: "TUR · YMB-20 Bandirma",
        service: Service::Cw,
    },
    Listed {
        channel_hz: 486_000.0,
        khz: "486",
        who: "CHN · Sanya, Guangzhou, Fuzhou, Zhanjiang, Shanghai, Dalian, Tianjin",
        service: Service::Navtex,
    },
    Listed { channel_hz: 490_000.0, khz: "490", who: "", service: Service::National },
    Listed { channel_hz: 518_000.0, khz: "518", who: "", service: Service::International },
    Listed {
        channel_hz: 521_500.0, khz: "521.5", who: "RUS · UDK Murmansk", service: Service::Cw
    },
    Listed {
        channel_hz: 2_590_000.0,
        khz: "2590",
        who: "RUS · UFM-2 Nevelsk",
        service: Service::Cw,
    },
];

/// Where the dial goes, in upper sideband, for a NAVTEX channel.
///
/// A channel is quoted by its assigned frequency, which is the centre of the
/// two tones; the decoder expects that centre at [`NAVTEX_TONE_HZ`] of audio.
/// One figure for every button: the MF pair used to sit 1710 Hz below and
/// the HF five on the channel itself, where the tones are at zero audio and
/// nothing decodes.
fn dial_hz(channel_hz: f64) -> f64 {
    channel_hz - f64::from(sdroxide_types::NAVTEX_TONE_HZ)
}

impl Listed {
    fn dial_hz(&self) -> f64 {
        match self.service {
            Service::Cw => self.channel_hz - CW_NOTE_HZ,
            _ => dial_hz(self.channel_hz),
        }
    }

    /// The row as the menu shows it.
    fn label(&self) -> String {
        let what = match self.service {
            Service::International => tr!("navtex-freq-international"),
            Service::National => tr!("navtex-freq-national"),
            Service::Navtex => self.who.to_string(),
            Service::Cw => format!("{} · {}", self.who, tr!("navtex-freq-cw")),
        };
        format!("{} kHz   {what}", self.khz)
    }
}

/// Whether the panel has anything on it to clear or to save: a message filed,
/// one arriving, or text in the monitor.
///
/// The two chips used to wake only for a filed message, so a page of monitor
/// text — which is all there is until a `ZCZC` has been copied — could be
/// neither wiped nor kept (F4NRM).
fn has_content(st: &NavtexStatus) -> bool {
    !st.messages.is_empty() || st.live.is_some() || !st.text.trim().is_empty()
}

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
        // with the published channel frequency and the dial lands below it
        // (USB) — see `dial_hz`.
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("MF").size(10.0).weak());
            for (channel_hz, label) in MF_CHANNELS {
                let vfo_hz = dial_hz(*channel_hz);
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
            for (channel_hz, label) in CHANNELS {
                let hz = dial_hz(*channel_hz);
                let active_vfo_hz = match self.state.active_vfo {
                    Vfo::A => self.state.vfo_a_hz,
                    Vfo::B => self.state.vfo_b_hz,
                };
                let is_active = (active_vfo_hz - hz).abs() < 1.0;
                if crate::chrome::chip(ui, is_active, RichText::new(*label).size(10.0))
                    .on_hover_text(tr!("navtex-tune-dial-usb-below-channel"))
                    .clicked()
                {
                    cmds.push(Command::SetVfo { vfo: Vfo::A, hz });
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
            ui.add_space(8.0);
            self.navtex_freq_menu(ui, cmds);
        });

        // Save / clear RX on the next line
        ui.horizontal_wrapped(|ui| {
            crate::chrome::row_tail(ui, |ui| {
                crate::app::panels::clear_rx_chip_at(ui, cmds, has_content(&st));
                crate::app::panels::save_rx_chip_for(ui, self.digi_status.as_ref());
            });
        });

        // No sensitivity control: the receiver decides for itself whether a
        // signal is there, so the two panes have the whole width.
        let h = ui.available_height();
        ui.allocate_ui(egui::vec2(ui.available_width(), h), |ui| {
            ui.set_min_height(h);
            ui.horizontal(|ui| {
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

    /// The other frequencies the published list carries, as a menu.
    ///
    /// The chips beside it are the channels a listener uses every day; these
    /// are the regional ones and the coast stations' Morse bulletins, which
    /// are too many for a row and are looked up by where the station is.
    fn navtex_freq_menu(&self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        let dial = self.state.active_freq_hz();
        let here = LISTED.iter().find(|l| (l.dial_hz() - dial).abs() < 1.0);
        let btn =
            crate::chrome::chip(ui, here.is_some(), RichText::new(tr!("panels-freq")).size(10.5))
                .on_hover_text(tr!("navtex-other-frequencies-hint"));
        let mut pick = None;
        egui::Popup::from_toggle_button_response(&btn)
            .frame(crate::chrome::window_frame())
            .close_behavior(egui::PopupCloseBehavior::CloseOnClick)
            .show(|ui| {
                crate::chrome::window_body_bg(ui);
                ui.set_max_width(420.0);
                ui.label(
                    RichText::new(tr!("navtex-other-frequencies"))
                        .color(theme::CYAN_DIM())
                        .size(9.5)
                        .strong(),
                );
                ui.add_space(2.0);
                egui::ScrollArea::vertical().max_height(360.0).show(ui, |ui| {
                    for l in LISTED {
                        let on = here.is_some_and(|h| h.channel_hz == l.channel_hz);
                        let mut text = RichText::new(l.label()).size(12.0);
                        if l.service == Service::Cw {
                            text = text.color(theme::YELLOW());
                        }
                        if ui.selectable_label(on, text).clicked() {
                            pick = Some(l.dial_hz());
                        }
                    }
                });
            });
        if let Some(hz) = pick {
            cmds.push(Command::SetVfo { vfo: Vfo::A, hz });
        }
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
            // The loose text, in sync or not. It used to be hidden the moment
            // the signal went, because what the receiver printed without one
            // was noise — and so a bulletin copied outside a `ZCZC` vanished
            // from the screen as its station signed off, while still being
            // there to save. The receiver no longer prints noise.
            None => st.text.clone(),
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

#[cfg(test)]
mod tests {
    use super::*;

    /// Every NAVTEX button puts the channel's centre at the audio frequency
    /// the decoder expects it at. The HF five used to tune the dial onto the
    /// channel itself, with the tones at zero audio, and the MF pair 10 Hz
    /// out.
    #[test]
    fn every_navtex_button_puts_the_tones_where_the_decoder_looks() {
        let tone = f64::from(sdroxide_types::NAVTEX_TONE_HZ);
        for (channel, label) in MF_CHANNELS.iter().chain(CHANNELS) {
            assert_eq!(channel - dial_hz(*channel), tone, "{label}");
            let khz: f64 = label.parse().expect("a frequency in kHz");
            assert_eq!(khz * 1000.0, *channel, "{label} is labelled as another channel");
        }
        for l in LISTED {
            let up = l.channel_hz - l.dial_hz();
            let want = if l.service == Service::Cw { CW_NOTE_HZ } else { tone };
            assert_eq!(up, want, "{} kHz", l.khz);
            let khz: f64 = l.khz.parse().expect("a frequency in kHz");
            assert_eq!(khz * 1000.0, l.channel_hz, "{} is labelled as another channel", l.khz);
        }
    }

    /// The menu is the published list: in frequency order, the two shared
    /// channels in it, and NAVDAT's 500 kHz — which nothing here can read —
    /// not.
    #[test]
    fn the_frequency_menu_is_the_published_list() {
        assert!(LISTED.windows(2).all(|w| w[0].channel_hz < w[1].channel_hz), "not in order");
        for hz in [490_000.0, 518_000.0, 424_000.0, 486_000.0] {
            assert!(
                LISTED.iter().any(|l| l.channel_hz == hz && l.service != Service::Cw),
                "{hz} Hz is missing or marked as Morse"
            );
        }
        assert!(LISTED.iter().all(|l| l.channel_hz != 500_000.0), "NAVDAT is not NAVTEX");
        assert!(
            LISTED.iter().all(|l| {
                matches!(l.service, Service::International | Service::National) != !l.who.is_empty()
            }),
            "a regional entry has no station, or a shared channel names one"
        );
    }

    /// Clear and Save wake for anything on the panel, not only a filed
    /// message: monitor text alone used to leave both grey.
    #[test]
    fn monitor_text_alone_is_something_to_clear_and_to_save() {
        assert!(!has_content(&NavtexStatus::default()));
        assert!(!has_content(&NavtexStatus { text: " \r\n".into(), ..Default::default() }));
        assert!(has_content(&NavtexStatus { text: "TENDENZA".into(), ..Default::default() }));
    }
}
