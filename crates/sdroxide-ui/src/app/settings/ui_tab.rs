//! The UI tab: layout, theme and font sizes, the frame rate, the waterfall's
//! palette and the colours the overlays are drawn in, and the spoken
//! announcements.
//!
//! These take effect live — the frame rate reaches the engine through the
//! spectrum-config diff on the next frame, and the rest is read straight out of
//! the settings each frame.
//!
//! The panadapter's own settings — its detail, the spectrum's reaction and the
//! waterfall's scroll speed — used to be here and are now on the SPEC popup,
//! beside the picture they change (`SdroxideApp::panadapter_controls`). They
//! are still the same [`sdroxide_types::UiSettings`] fields, written and
//! persisted the same way.

use eframe::egui::{self, Color32, ComboBox, RichText};
use sdroxide_types::{BandplanKind, CallsignStyle, FreqStyle, SpeechSettings, SpotKind, Verbosity};

use crate::colormap;
use crate::i18n::{Tr, tr};

use crate::app::settings::enum_combo;
use crate::app::settings::general::device_combo;
use crate::app::speech::SpeechStatus;
use crate::chrome::StyledCombo;

/// What the "Screen follows your profile" row asked the app to do. The row is
/// drawn by a free function, so the action travels back to the caller.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileAction {
    Save,
    Revert,
}

pub(in crate::app) fn settings_ui_tab(
    ui: &mut egui::Ui,
    cfg: &mut sdroxide_types::UiSettings,
    radio: Option<&mut sdroxide_types::RadioConfig>,
    swl_hide: &mut bool,
    cloud_march: Option<&mut bool>,
    profile_action: &std::cell::Cell<Option<ProfileAction>>,
    profile_from: Option<&Option<String>>,
    profile_status: Option<&String>,
    report: &mut sdroxide_types::SwlReportPrefs,
    report_pick: &std::sync::Arc<std::sync::Mutex<Option<String>>>,
) {
    use sdroxide_types::{ChromeStyle, FontSize, LayoutMode, UiSettings, UiTheme};
    ui.label(RichText::new(tr!("settings-ui-display")).size(14.0).strong().color(crate::theme::CYAN()));
    {
        // **How many broadcast stations a listener's screen may show** (#80).
        // The window is the panadapter's span, which is right for a label on
        // that waterfall and wrong for a whole-band map: a listener at 6190 kHz
        // with 20 kHz of span was told there were four stations in the world.
        use sdroxide_types::BroadcastScope as B;
        ui.label(tr!("settings-broadcast-stations"));
        ComboBox::from_id_salt("ui-broadcast-scope")
            .selected_text(broadcast_scope_label(cfg.broadcast_scope))
            .show_styled(ui, |ui| {
                for scope in [B::InView, B::WholeBand] {
                    ui.selectable_value(&mut cfg.broadcast_scope, scope, broadcast_scope_label(scope));
                }
            })
            .response
            .on_hover_text(
                tr!("settings-ui-broadcast-stations-hint"),
            );
        // The **other** question about broadcasters, and the one the operator kept
        // having to answer with ⭐ ALL: whether the waterfall names the station the
        // dial is on or the whole band. The chip is in the SPOTS window, where it
        // is used; this is beside the scope combo because that is where a
        // preference is looked for.
        ui.label(tr!("settings-ui-waterfall-labels"));
        ui.horizontal(|ui| {
            if crate::chrome::chip(ui, !cfg.broadcast_labels_all, tr!("settings-ui-tuned-only"))
                .on_hover_text(
                    tr!("settings-ui-tuned-only-hint"),
                )
                .clicked()
            {
                cfg.broadcast_labels_all = false;
            }
            if crate::chrome::chip(ui, cfg.broadcast_labels_all, tr!("settings-ui-whole-band"))
                .on_hover_text(
                    tr!("settings-ui-whole-band-hint"),
                )
                .clicked()
            {
                cfg.broadcast_labels_all = true;
            }
        });
    }
    ui.add_space(6.0);
    // Said here because this is where they used to be, and where an operator
    // who remembers them will come looking. They moved to the panadapter's own
    // button, beside the picture they change — see
    // `SdroxideApp::panadapter_controls`.
    super::note_at(
        ui,
        &tr!("settings-ui-moved-note"),
        Some(11.0),
    );
    ui.add_space(6.0);
    egui::Grid::new("ui-grid").num_columns(2).spacing([12.0, 8.0]).min_row_height(ui.spacing().interact_size.y).show(ui, |ui| {
        ui.label(tr!("settings-ui-layout"));
        enum_combo(ui, "ui-layout", &mut cfg.layout, &LayoutMode::ALL, LayoutMode::tr);
        ui.end_row();

        // The two arrangements of the desktop. It is beside Layout because it
        // is the same kind of choice — where things live on this screen — and
        // not under it, because it is not a size: the column is offered at
        // every desktop size and left out where there is no room for it.
        ui.label(tr!("settings-ui-classic-ui")).on_hover_text(
            tr!("settings-ui-classic-ui-hint"),
        );
        crate::chrome::checkbox(ui, &mut cfg.classic_ui, tr!("settings-ui-classic-ui-check"));
        ui.end_row();

        ui.label(tr!("settings-ui-theme"));
        enum_combo(ui, "ui-theme", &mut cfg.theme, &UiTheme::ALL, UiTheme::tr);
        ui.end_row();

        ui.label(tr!("settings-ui-button-style"));
        enum_combo(ui, "ui-btn-style", &mut cfg.button_style, &ChromeStyle::ALL, ChromeStyle::tr);
        ui.end_row();

        ui.label(tr!("settings-ui-updates"));
        crate::chrome::checkbox(
            ui,
            &mut cfg.check_for_updates,
            tr!("settings-ui-updates-check"),
        )
        .on_hover_text(
            tr!("settings-ui-updates-hint"),
        );
        ui.end_row();

        // The **undocked** modules. A window arrangement, chosen once and kept,
        // rather than something reached for mid-QSO — so it lives here with the
        // other display settings. One row per [`DetachableModule`], so the next
        // detachable module appears here for free. Native only: the browser
        // keeps every module in its one window.
        #[cfg(not(target_arch = "wasm32"))]
        {
            use sdroxide_types::DetachableModule;
            for module in DetachableModule::ALL {
                let mut on = cfg.is_detached(module);
                ui.label(tr!("settings-ui-window-label", module = module_label(module)));
                ComboBox::from_id_salt(("ui-undocked", module))
                    .selected_text(if on {
                        tr!("settings-ui-undocked")
                    } else {
                        tr!("settings-ui-docked")
                    })
                    .show_styled(ui, |ui| {
                        ui.selectable_value(&mut on, false, tr!("settings-ui-docked"));
                        ui.selectable_value(&mut on, true, tr!("settings-ui-undocked"));
                    })
                    .response
                    .on_hover_text(tr!("settings-ui-undock-hint", app_id = module.app_id()));
                cfg.set_detached(module, on);
                ui.end_row();
            }
            // Public SDRs is not a module of its own but a tool window, with a
            // third place the modules do not have: docked under the waterfall,
            // as Radio Classic is. Where that slot is taken (a digital mode's
            // panel, CW, a phone) it is a window whatever is chosen here.
            {
                let mut docked = cfg.public_sdr_docked;
                ui.label(tr!("settings-ui-window-label", module = tr!("publicsdr-public-sdrs")));
                ComboBox::from_id_salt("ui-publicsdr-place")
                    .selected_text(if docked {
                        tr!("settings-ui-publicsdr-under-waterfall")
                    } else {
                        tr!("settings-ui-undocked")
                    })
                    .show_styled(ui, |ui| {
                        ui.selectable_value(
                            &mut docked,
                            true,
                            tr!("settings-ui-publicsdr-under-waterfall"),
                        );
                        ui.selectable_value(&mut docked, false, tr!("settings-ui-undocked"));
                    })
                    .response
                    .on_hover_text(tr!("settings-ui-publicsdr-place-hint"));
                cfg.public_sdr_docked = docked;
                ui.end_row();
            }
            // Both directions at once, for arranging a whole screen in one
            // press: every module out, or every module home. Shown only while
            // they would do something, so neither is a button that changes
            // nothing.
            let any_out = DetachableModule::ALL.iter().any(|m| cfg.is_detached(*m));
            let any_in = DetachableModule::ALL.iter().any(|m| !cfg.is_detached(*m));
            if any_out || any_in {
                ui.label("");
                ui.horizontal(|ui| {
                    if any_out
                        && crate::chrome::chip(ui, false, tr!("multi-dock-all-windows"))
                            .on_hover_text(
                                tr!("settings-ui-dock-all-hint"),
                            )
                            .clicked()
                    {
                        for m in DetachableModule::ALL {
                            cfg.set_detached(m, false);
                        }
                    }
                    if any_in
                        && crate::chrome::chip(ui, false, tr!("multi-undock-all-windows"))
                            .on_hover_text(
                                tr!("settings-ui-undock-all-hint"),
                            )
                            .clicked()
                    {
                        for m in DetachableModule::ALL {
                            cfg.set_detached(m, true);
                        }
                    }
                });
                ui.end_row();
            }
        }

        ui.label(tr!("settings-ui-frame-rate"));
        ComboBox::from_id_salt("ui-fps")
            .selected_text(format!("{fps} fps", fps = cfg.frame_rate_fps))
            .show_styled(ui, |ui| {
                for f in UiSettings::FPS_OPTIONS {
                    ui.selectable_value(&mut cfg.frame_rate_fps, f, format!("{fps} fps", fps = f));
                }
            });
        ui.end_row();

        ui.label(tr!("settings-ui-palette"));
        ComboBox::from_id_salt("ui-palette")
            .selected_text(colormap::name_tr(cfg.waterfall_palette.min(colormap::NAMES.len() - 1)))
            .show_styled(ui, |ui| {
                for i in 0..colormap::NAMES.len() {
                    ui.selectable_value(&mut cfg.waterfall_palette, i, colormap::name_tr(i));
                }
            });
        ui.end_row();

        ui.label(tr!("settings-ui-tuning-buttons"));
        ui.horizontal(|ui| {
            crate::chrome::checkbox(ui, &mut cfg.tune_step_buttons, tr!("settings-ui-tuning-buttons-show"))
                .on_hover_text(
                    tr!("settings-ui-tuning-buttons-show", "hint"),
                );
            super::note(ui, &tr!("settings-ui-tuning-step", step = cfg.tune_step_label()));
        });
        ui.end_row();

        // Only meaningful with the row above, so it sits under it.
        if cfg.tune_step_buttons {
            ui.label("");
            crate::chrome::checkbox(
                ui,
                &mut cfg.tune_step_round_first,
                tr!("settings-ui-tuning-snap"),
            )
            .on_hover_text(tr!("settings-ui-tuning-snap", "hint"));
            ui.end_row();
        }

        ui.label(tr!("settings-ui-smoothing"));
        crate::chrome::checkbox(
            ui,
            &mut cfg.waterfall_smooth,
            tr!("settings-ui-smoothing-interpolate"),
        )
        .on_hover_text(tr!("settings-ui-smoothing-interpolate", "hint"));
        ui.end_row();

        ui.label(tr!("settings-ui-background"));
        ui.horizontal(|ui| {
            crate::chrome::checkbox(
                ui,
                &mut cfg.spectrum_gradient,
                tr!("settings-ui-background-gradient"),
            );
            ui.add_enabled_ui(cfg.spectrum_gradient, |ui| {
                ui.label(tr!("settings-ui-background-top"));
                ui.color_edit_button_srgb(&mut cfg.gradient_top);
                ui.label(tr!("settings-ui-background-bottom"));
                ui.color_edit_button_srgb(&mut cfg.gradient_bottom);
            });
        });
        ui.end_row();

        ui.label(tr!("settings-ui-spectrum-line")).on_hover_text(
            tr!("settings-ui-spectrum-line-hint"),
        );
        ui.horizontal(|ui| {
            crate::chrome::checkbox(ui, &mut cfg.spectrum_fill, tr!("settings-ui-fill"));
            ui.add_enabled_ui(cfg.spectrum_fill, |ui| {
                ui.label(tr!("settings-ui-fill-colour"));
                ui.color_edit_button_srgb(&mut cfg.spectrum_fill_color);
                ComboBox::from_id_salt("ui-spectrum-fill-style")
                    .selected_text(fill_style_label(cfg.spectrum_fill_style))
                    .show_styled(ui, |ui| {
                        for s in sdroxide_types::SpectrumFillStyle::ALL {
                            ui.selectable_value(&mut cfg.spectrum_fill_style, s, fill_style_label(s));
                        }
                    });
            });
        });
        ui.end_row();

        first_line_label(ui, &tr!("settings-ui-spot-colours")).on_hover_text(
            tr!("settings-ui-spot-colours", "hint"),
        );
        // Wrapped inside a column, not straight in the grid cell: laid out
        // directly in the cell, a row that wraps onto a second line is not
        // given that line's full height, and its Reset button ran into the
        // picker in the row below.
        ui.vertical(|ui| {
            ui.horizontal_wrapped(|ui| {
                for kind in SpotKind::ALL {
                    ui.color_edit_button_srgb(&mut cfg.spot_colors[kind.index()])
                        .on_hover_text(tr!("settings-ui-spot-colour", kind = spot_kind_label(kind)));
                    let [r, g, b] = cfg.spot_colors[kind.index()];
                    // Tinted with what was just picked, so the row is its own
                    // preview — a colour that vanishes into the panel here would
                    // vanish into the waterfall too.
                    ui.label(
                        RichText::new(spot_kind_label(kind))
                            .size(11.0)
                            .strong()
                            .color(crate::theme::data_ink((r, g, b))),
                    );
                    ui.add_space(6.0);
                }
                if ui
                    .button(tr!("settings-ui-reset"))
                    .on_hover_text(tr!("settings-ui-spot-colours-reset"))
                    .clicked()
                {
                    for kind in SpotKind::ALL {
                        let (r, g, b) = kind.default_color();
                        cfg.spot_colors[kind.index()] = [r, g, b];
                    }
                }
            });
        });
        ui.end_row();

        first_line_label(ui, &tr!("settings-ui-bandplan-colours")).on_hover_text(
            tr!("settings-ui-bandplan-colours", "hint"),
        );
        ui.vertical(|ui| {
            ui.horizontal_wrapped(|ui| {
                for kind in BandplanKind::ALL {
                    ui.color_edit_button_srgb(&mut cfg.bandplan_colors[kind.index()])
                        .on_hover_text(tr!("settings-ui-bandplan-colour", kind = kind.tr()));
                    let [r, g, b] = cfg.bandplan_colors[kind.index()];
                    ui.label(
                        RichText::new(kind.tr())
                            .size(11.0)
                            .strong()
                            .color(crate::theme::data_ink((r, g, b))),
                    );
                    ui.add_space(6.0);
                }
                if ui
                    .button(tr!("settings-ui-reset"))
                    .on_hover_text(tr!("settings-ui-bandplan-colours-reset"))
                    .clicked()
                {
                    for kind in BandplanKind::ALL {
                        let (r, g, b) = kind.default_color();
                        cfg.bandplan_colors[kind.index()] = [r, g, b];
                    }
                }
            });
        });
        ui.end_row();

        ui.label(tr!("settings-ui-skimmer-font"));
        enum_combo(ui, "ui-skim-font", &mut cfg.skimmer_font_size, &FontSize::ALL, FontSize::tr);
        ui.end_row();

        ui.label(tr!("settings-ui-waterfall-font"));
        enum_combo(ui, "ui-wf-font", &mut cfg.waterfall_font_size, &FontSize::ALL, FontSize::tr);
        ui.end_row();

        ui.label(tr!("settings-ui-interface-font"))
            .on_hover_text(tr!("settings-ui-interface-font", "hint"));
        enum_combo(ui, "ui-menu-font", &mut cfg.menu_font_size, &FontSize::ALL, FontSize::tr);
        ui.end_row();

        ui.label(tr!("settings-ui-simple-ui")).on_hover_text(
            tr!("settings-ui-simple-ui-hint"),
        );
        crate::chrome::checkbox(ui, &mut cfg.simple_ui, tr!("settings-ui-simple-ui-check"));
        ui.end_row();

        ui.label(tr!("settings-ui-wake-lock")).on_hover_text(
            tr!("settings-ui-wake-lock-hint"),
        );
        ui.horizontal(|ui| {
            crate::chrome::checkbox(ui, &mut cfg.keep_screen_on, tr!("settings-ui-wake-lock-check"));
            let status = crate::wake_lock::status(cfg.keep_screen_on);
            if !wake_label(status).is_empty() {
                let ink = if status.is_problem() {
                    crate::theme::YELLOW()
                } else {
                    ui.visuals().weak_text_color()
                };
                ui.label(egui::RichText::new(wake_label(status)).small().color(ink));
            }
        });
        ui.end_row();

        ui.label(tr!("settings-ui-sound-welcome")).on_hover_text(
            tr!("settings-ui-sound-welcome-hint"),
        );
        ui.horizontal(|ui| {
            crate::chrome::checkbox(
                ui,
                &mut cfg.sound_welcome,
                egui::RichText::new(tr!("settings-ui-sound-welcome-check")).color(crate::theme::YELLOW()),
            );
            if !crate::sound_welcome::available() {
                ui.label(
                    egui::RichText::new(
                        tr!("settings-ui-sound-welcome-browser-only"),
                    )
                    .small()
                    .color(crate::theme::YELLOW()),
                );
            }
        });
        ui.end_row();


        ui.end_row();

        ui.label(tr!("settings-ui-profile-screen")).on_hover_text(
            tr!("settings-ui-profile-screen-hint"),
        );
        ui.horizontal(|ui| {
            if ui.button(tr!("settings-ui-save-to-profile")).clicked() {
                profile_action.set(Some(ProfileAction::Save));
            }
            if ui.button(tr!("settings-ui-back-to-profile")).clicked() {
                profile_action.set(Some(ProfileAction::Revert));
            }
        });
        // Say where the look in force came from, and what the last save did. A
        // save that says nothing is a save nobody can trust.
        ui.label(
            RichText::new(match profile_from {
                Some(Some(name)) => tr!("settings-ui-profile-in-use-named", name = name.as_str()),
                Some(None) => tr!("settings-ui-profile-in-use-default"),
                None => tr!("settings-ui-profile-not-in-use"),
            })
            .weak()
            .small(),
        );
        if let Some(status) = profile_status {
            ui.label(RichText::new(status.clone()).small().color(crate::theme::YELLOW()));
        }
        ui.end_row();

        // The opt-in stays, deliberately. The shared keyboard is the one thing
        // in a profile that can hurt another operator, so it is behind an
        // acknowledgement and off by default — and it is **not** to be removed
        // as unnecessary: it is asked for. Someone running their own server and
        // reaching it from several of their own devices wants their keys to
        // follow them. The answer to a shared station is the acknowledgement
        // and the default, not taking the feature away.
        ui.label(tr!("settings-ui-carry-bindings")).on_hover_text(
            tr!("settings-ui-carry-bindings-hint"),
        );
        {
            // The opt-in is not a plain checkbox: turning it ON must be
            // acknowledged, so the click opens a modal and the flag stays off
            // until the operator confirms. Turning it OFF is immediate.
            let confirm_id = egui::Id::new("client-share-bindings-confirm");
            ui.horizontal(|ui| {
                ui.scope(|ui| {
                    let was = cfg.client_share_bindings;
                    let mut shown = was;
                    let resp = crate::chrome::checkbox(
                        ui,
                        &mut shown,
                        if was {
                            tr!("settings-ui-carry-bindings-on")
                        } else {
                            tr!("settings-ui-carry-bindings-off")
                        },
                    );
                    if resp.clicked() {
                        if shown && !was {
                            ui.data_mut(|d| d.insert_temp(confirm_id, true));
                        } else {
                            cfg.client_share_bindings = shown;
                            // Turning it off is an answer too — "keep mine" —
                            // so the server stops offering the profile's keys.
                            cfg.client_bindings_declined = !shown;
                        }
                    }
                    resp
                });
            });
            let confirming: bool = ui.data(|d| d.get_temp(confirm_id)).unwrap_or(false);
            if confirming {
                let mut enable = false;
                let mut cancel = false;
                egui::Modal::new(egui::Id::new("client-share-bindings-modal")).show(
                    ui.ctx(),
                    |ui| {
                        ui.set_max_width(470.0);
                        ui.heading(
                            RichText::new(tr!("settings-ui-carry-bindings-title"))
                                .color(crate::theme::ALERT()),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            tr!("settings-ui-carry-bindings-body"),
                        );
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if ui
                                .button(
                                    RichText::new(tr!("settings-ui-carry-bindings-confirm"))
                                        .color(crate::theme::ALERT()),
                                )
                                .clicked()
                            {
                                enable = true;
                            }
                            if ui.button(tr!("settings-wavelog-cancel")).clicked() {
                                cancel = true;
                            }
                        });
                    },
                );
                if enable {
                    cfg.client_share_bindings = true;
                    cfg.client_bindings_declined = false;
                }
                if enable || cancel {
                    ui.data_mut(|d| d.remove_temp::<bool>(confirm_id));
                }
            }
        }
        ui.end_row();

        ui.label(tr!("settings-ui-start-swl")).on_hover_text(
            tr!("settings-ui-start-swl-hint"),
        );
        crate::chrome::checkbox(ui, &mut cfg.start_swl, tr!("settings-ui-start-swl-check"));
        ui.end_row();

        // Issue #146: the LISTEN tab in the band/mode menu carrying the listening
        // screen. **It hides; it never grants.** Off by default, and it asks once
        // — the reporter's own words were "I accept this operating mode at my own
        // risk", and the person who wrote them is the one who has to read what it
        // does not do.
        ui.label(tr!("settings-ui-listen-hides-tx")).on_hover_text(
            tr!("settings-ui-listen-hides-tx-hint"),
        );
        {
            let confirm_id = egui::Id::new("listen-tab-hides-tx-confirm");
            let mut shown = cfg.listen_tab_hides_tx;
            let resp = crate::chrome::checkbox(ui, &mut shown, tr!("settings-ui-listen-hides-tx-check"));
            let was = cfg.listen_tab_hides_tx;
            if resp.changed() {
                if shown && !was {
                    ui.data_mut(|d| d.insert_temp(confirm_id, true));
                } else {
                    cfg.listen_tab_hides_tx = shown;
                }
            }
            let confirming: bool = ui.data(|d| d.get_temp(confirm_id)).unwrap_or(false);
            if confirming {
                let mut enable = false;
                let mut cancel = false;
                egui::Modal::new(egui::Id::new("listen-tab-hides-tx-modal")).show(
                    ui.ctx(),
                    |ui| {
                        ui.set_max_width(470.0);
                        ui.heading(
                            RichText::new(tr!("settings-ui-listen-hides-tx-title"))
                                .color(crate::theme::ALERT()),
                        );
                        ui.add_space(6.0);
                        ui.label(
                            tr!("settings-ui-listen-hides-tx-body"),
                        );
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            if ui
                                .button(
                                    RichText::new(tr!("settings-ui-listen-hides-tx-confirm"))
                                        .color(crate::theme::ALERT()),
                                )
                                .clicked()
                            {
                                enable = true;
                            }
                            if ui.button(tr!("settings-wavelog-cancel")).clicked() {
                                cancel = true;
                            }
                        });
                    },
                );
                if enable {
                    cfg.listen_tab_hides_tx = true;
                }
                if enable || cancel {
                    ui.data_mut(|d| d.remove_temp::<bool>(confirm_id));
                }
            }
        }
        ui.end_row();

        // The switch itself, not only the seed: the same per-radio
        // `RadioConfig::hide_tx` that Settings → Radio → Transmit controls
        // writes, reachable here because a listener looks for it in the UI
        // menu. Off `radio` (a remote or browser client with no config yet)
        // there is nothing to set, so the row is left out rather than shown
        // dead.
        if let Some(radio) = radio {
            ui.label(tr!("settings-ui-swl-mode")).on_hover_text(
                tr!("settings-ui-swl-mode-hint"),
            );
            // The checkbox reflects the whole per-radio SWL state (the
            // start-in-SWL seed *or* `hide_tx`), so a session that came up in
            // SWL mode reads as "on" and one toggle turns it off, not two.
            if crate::chrome::checkbox(ui, swl_hide, tr!("settings-ui-swl-mode-check")).changed() {
                radio.hide_tx = *swl_hide;
            }
            ui.end_row();
        }

        // ── Reception reports ──
        // What a mailed reception report carries besides the reception itself.
        // Both optional and both genuinely so: a plain-text report is a
        // perfectly good report, so leaving these empty breaks nothing.
        //
        // Native-only, because the picture is a **path on this machine's disk**
        // and a browser tab has no disk to name. Storing a string there could
        // never resolve to a file, so the row is left out rather than shown
        // dead — the same rule the rest of this crate follows for paths.
        if !cfg!(target_arch = "wasm32") {
            ui.label(tr!("settings-ui-report-picture")).on_hover_text(
                tr!("settings-ui-report-picture-hint"),
            );
            // Whatever the picker thread found, taken once and cleared so it is
            // applied exactly once.
            if let Ok(mut inbox) = report_pick.lock()
                && let Some(picked) = inbox.take()
            {
                report.picture = picked;
            }
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut report.picture)
                        .desired_width(240.0)
                        .hint_text(tr!("settings-ui-report-picture-none")),
                );
                if crate::chrome::chip(ui, false, tr!("settings-ui-choose"))
                    .on_hover_text(tr!("settings-ui-report-picture-pick"))
                    .clicked()
                {
                    pick_report_picture(report_pick);
                }
                if !report.picture.is_empty()
                    && crate::chrome::chip(ui, false, tr!("sstv-clear")).clicked()
                {
                    report.picture.clear();
                }
            });
            ui.label(tr!("settings-ui-report-note")).on_hover_text(
                tr!("settings-ui-report-note-hint"),
            );
            crate::chrome::field(
                ui,
                egui::TextEdit::multiline(&mut report.message)
                    .desired_width(380.0)
                    .desired_rows(3)
                    .hint_text(tr!("settings-ui-report-note-hint-text")),
            );
        }

        ui.label(tr!("settings-ui-cities")).on_hover_text(tr!("settings-ui-cities", "hint"));
        crate::chrome::checkbox(ui, &mut cfg.map_cities, tr!("settings-ui-cities-show"));
        ui.end_row();
    });

    let Some(cloud_march) = cloud_march else { return };
    ui.add_space(14.0);
    ui.label(
        RichText::new(tr!("settings-ui-3d-view")).size(14.0).strong().color(crate::theme::CYAN()),
    );
    ui.add_space(6.0);
    egui::Grid::new("ui-grid-3d")
        .num_columns(2)
        .spacing([12.0, 8.0])
        .min_row_height(ui.spacing().interact_size.y)
        .show(ui, |ui| {
            ui.label(tr!("settings-ui-clouds"));
            ComboBox::from_id_salt("ui-cloud-march")
                .selected_text(if *cloud_march { tr!("settings-ui-clouds-volumetric") } else { tr!("settings-ui-clouds-layered") })
                .show_styled(ui, |ui| {
                    ui.selectable_value(cloud_march, false, tr!("settings-ui-clouds-layered"));
                    ui.selectable_value(cloud_march, true, tr!("settings-ui-clouds-volumetric"));
                });
            ui.end_row();
        });
    ui.add_space(8.0);
    super::note(
        ui,
        &tr!("settings-ui-clouds-note"),
    );
}

/// Spoken announcements.
///
/// Sized so the UI tab stays scannable: the controls an operator sets up
/// once are visible, and the two dozen per-category switches live behind a
/// collapsing header. Every value is written as it is changed, like the control
/// bindings — the rate and volume take effect on the next utterance without a
/// restart, and there is no separate step for an APPLY to stand for.
pub(in crate::app) fn speech_settings(
    ui: &mut egui::Ui,
    cfg: &mut SpeechSettings,
    voices: &[String],
    outputs: &[String],
    status: &SpeechStatus,
    test: &mut bool,
) {
    ui.label(RichText::new(tr!("speech-title")).size(14.0).strong().color(crate::theme::CYAN()));
    ui.add_space(6.0);
    crate::chrome::checkbox(ui, &mut cfg.enabled, tr!("speech-enabled"))
        .on_hover_text(tr!("speech-enabled", "hint"));

    ui.add_enabled_ui(cfg.enabled, |ui| {
        egui::Grid::new("speech-grid")
            .num_columns(2)
            .spacing([12.0, 8.0])
            .min_row_height(ui.spacing().interact_size.y)
            .show(ui, |ui| {
                ui.label(tr!("speech-voice"));
                let shown = if cfg.voice.is_empty() {
                    tr!("speech-voice-shipped").to_string()
                } else {
                    cfg.voice.clone()
                };
                ComboBox::from_id_salt("speech-voice")
                    .width(300.0)
                    .selected_text(shown)
                    .show_styled(ui, |ui| {
                        if ui.selectable_label(cfg.voice.is_empty(), tr!("speech-voice-shipped")).clicked() {
                            cfg.voice.clear();
                        }
                        for v in voices {
                            if ui.selectable_label(&cfg.voice == v, v).clicked() {
                                cfg.voice = v.clone();
                            }
                        }
                    });
                ui.end_row();

                ui.label(tr!("speech-speed"));
                crate::chrome::slider(
                    ui,
                    egui::Slider::new(&mut cfg.rate, SpeechSettings::RATE_RANGE)
                        .step_by(0.1)
                        .suffix("×"),
                )
                .on_hover_text(
                    tr!("speech-speed", "hint"),
                );
                ui.end_row();

                ui.label(tr!("speech-volume"));
                crate::chrome::slider(
                    ui,
                    egui::Slider::new(&mut cfg.volume, 0.0..=1.0).step_by(0.05),
                );
                ui.end_row();

                ui.label(tr!("speech-output"));
                // `device_combo` borrows the current selection while handing the
                // new one to the closure, so the two cannot both be `cfg.device`.
                let cur = cfg.device.clone();
                device_combo(ui, "speech-out", outputs, &cur, |n| cfg.device = n);
                ui.end_row();

                ui.label(tr!("speech-detail"));
                crate::app::settings::enum_combo(
                    ui,
                    "speech-verbosity",
                    &mut cfg.verbosity,
                    &Verbosity::ALL,
                    Verbosity::tr,
                );
                ui.end_row();

                ui.label(tr!("speech-duck"));
                ui.horizontal(|ui| {
                    crate::chrome::checkbox(ui, &mut cfg.duck_rx, tr!("speech-duck-while"));
                    ui.add_enabled_ui(cfg.duck_rx, |ui| {
                        crate::chrome::slider(
                            ui,
                            egui::Slider::new(&mut cfg.duck_level, 0.0..=1.0).step_by(0.05),
                        );
                    });
                });
                ui.end_row();
            });

        ui.add_space(6.0);
        ui.horizontal(|ui| {
            if ui.button(tr!("speech-test")).clicked() {
                *test = true;
            }
            if let Some(note) = status.note() {
                let text = RichText::new(note);
                ui.label(if status.is_failed() {
                    text.color(Color32::from_rgb(0xE0, 0x6C, 0x4B))
                } else {
                    text.weak()
                });
            }
        });

        ui.add_space(4.0);
        egui::CollapsingHeader::new(tr!("speech-what")).default_open(false).show(ui, |ui| {
            egui::Grid::new("speech-cats")
                .num_columns(2)
                .spacing([16.0, 4.0])
                .min_row_height(ui.spacing().interact_size.y)
                .show(ui, |ui| {
                    let c = &mut cfg.cat;
                    crate::chrome::checkbox(ui, &mut c.frequency, tr!("speech-cat-frequency"));
                    crate::chrome::checkbox(ui, &mut c.mode_band, tr!("speech-cat-mode-band"));
                    ui.end_row();
                    crate::chrome::checkbox(ui, &mut c.vfo_split, tr!("speech-cat-vfo-split"));
                    crate::chrome::checkbox(ui, &mut c.agc_gain, tr!("speech-cat-agc-gain"));
                    ui.end_row();
                    crate::chrome::checkbox(ui, &mut c.levels, tr!("speech-cat-levels"));
                    crate::chrome::checkbox(ui, &mut c.ptt, tr!("speech-cat-ptt"));
                    ui.end_row();
                    crate::chrome::checkbox(ui, &mut c.rit_xit, tr!("speech-cat-rit-xit"));
                    crate::chrome::checkbox(ui, &mut c.memory_scan, tr!("speech-cat-memory-scan"));
                    ui.end_row();
                    crate::chrome::checkbox(ui, &mut c.band_edge, tr!("speech-cat-band-edge"));
                    crate::chrome::checkbox(ui, &mut c.notices, tr!("speech-cat-notices"));
                    ui.end_row();
                    crate::chrome::checkbox(
                        ui,
                        &mut c.filters,
                        tr!("speech-cat-filters"),
                    )
                    .on_hover_text(tr!("speech-cat-filters", "hint"));
                    ui.end_row();
                });

            ui.add_space(8.0);
            ui.label(RichText::new(tr!("speech-decoded")).strong());
            ui.end_row();
            egui::Grid::new("speech-decodes")
                .num_columns(2)
                .spacing([16.0, 4.0])
                .min_row_height(ui.spacing().interact_size.y)
                .show(ui, |ui| {
                    let d = &mut cfg.decodes;
                    crate::chrome::checkbox(ui, &mut d.ft8_to_me, tr!("speech-ft8-to-me"));
                    crate::chrome::checkbox(ui, &mut d.ft8_cq_for_me, tr!("speech-ft8-cq"))
                        .on_hover_text(
                            tr!("speech-ft8-cq", "hint"),
                        );
                    ui.end_row();
                    crate::chrome::checkbox(ui, &mut d.js8, tr!("speech-js8"));
                    crate::chrome::checkbox(ui, &mut d.js8_allcall, tr!("speech-js8-allcall"));
                    ui.end_row();
                    crate::chrome::checkbox(ui, &mut d.fsq, tr!("speech-fsq"));
                    crate::chrome::checkbox(ui, &mut d.include_snr, tr!("speech-include-snr"))
                    .on_hover_text(
                        tr!("speech-include-snr", "hint"),
                    );
                    ui.end_row();
                    crate::chrome::checkbox(ui, &mut d.ft8_qso, tr!("speech-ft8-qso"))
                        .on_hover_text(
                            tr!("speech-ft8-qso", "hint"),
                        );
                    ui.end_row();
                });

            ui.add_space(8.0);
            ui.label(RichText::new(tr!("speech-reading-text")).strong());
            ui.end_row();
            ui.horizontal(|ui| {
                crate::chrome::checkbox(ui, &mut cfg.text.cw, tr!("speech-text-cw"));
                crate::chrome::checkbox(ui, &mut cfg.text.rtty_psk, tr!("speech-text-rtty-psk"));
            });
            crate::chrome::checkbox(
                ui,
                &mut cfg.text.cw_only_when_locked,
                tr!("speech-text-cw-locked"),
            )
            .on_hover_text(tr!("speech-text-cw-locked", "hint"));
            super::note(
                ui,
                &tr!("speech-text-note"),
            );

            ui.add_space(8.0);
            ui.label(RichText::new(tr!("speech-tuning-up")).strong());
            crate::chrome::checkbox(
                ui,
                &mut cfg.tune.swr_while_tuning,
                tr!("speech-swr-while-tuning"),
            );
            ui.add_enabled_ui(cfg.tune.swr_while_tuning, |ui| {
                ui.horizontal(|ui| {
                    ui.label(tr!("speech-swr-every"));
                    crate::chrome::slider(
                        ui,
                        egui::Slider::new(&mut cfg.tune.period_s, 1.0..=10.0)
                            .step_by(0.5)
                            .suffix(" s"),
                    );
                });
            });
            crate::chrome::checkbox(
                ui,
                &mut cfg.tune.summary_after_tune,
                tr!("speech-swr-summary"),
            );
            crate::chrome::checkbox(ui, &mut cfg.tune.alarm_always, tr!("speech-swr-alarm"));

            ui.add_space(8.0);
            ui.label(RichText::new(tr!("speech-how-read")).strong());
            egui::Grid::new("speech-style").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
                ui.label(tr!("speech-frequencies"));
                crate::app::settings::enum_combo(
                    ui,
                    "speech-freq-style",
                    &mut cfg.freq_style,
                    &FreqStyle::ALL,
                    FreqStyle::tr,
                );
                ui.end_row();
                ui.label(tr!("speech-callsigns"));
                crate::app::settings::enum_combo(
                    ui,
                    "speech-call-style",
                    &mut cfg.callsign_style,
                    &CallsignStyle::ALL,
                    CallsignStyle::tr,
                );
                ui.end_row();
            });

            ui.add_space(6.0);
            crate::chrome::checkbox(ui, &mut cfg.duck_on_ptt, tr!("speech-quiet-on-tx"))
                .on_hover_text(tr!("speech-quiet-on-tx", "hint"));
        });
    });

    ui.add_space(8.0);
    super::note(
        ui,
        &tr!("speech-device-note"),
    );
}

/// Open a file dialog for the report picture, off the UI thread.
///
/// `rfd`'s dialog is blocking, so doing it here would freeze the frame loop for
/// as long as the operator takes to choose. The path comes back through the
/// inbox rather than a channel, because there is exactly one string and the
/// window that asked for it is the one that reads it.
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn pick_report_picture(
    inbox: &std::sync::Arc<std::sync::Mutex<Option<String>>>,
) {
    let inbox = inbox.clone();
    std::thread::Builder::new()
        .name("sdroxide-report-pick".into())
        .spawn(move || {
            if let Some(path) =
                rfd::FileDialog::new().add_filter(tr!("widgets-image"), &["png", "jpg", "jpeg"]).pick_file()
            {
                if let Ok(mut g) = inbox.lock() {
                    *g = Some(path.to_string_lossy().into_owned());
                }
            }
        })
        .ok();
}

/// The browser cannot name a file on this computer, so there is nothing to pick
/// — see where the row is drawn.
#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn pick_report_picture(
    _inbox: &std::sync::Arc<std::sync::Mutex<Option<String>>>,
) {
}

/// The **Workspaces** section: SDRuno's ten named arrangements. Save the current
/// one, or put a saved one back.
///
/// A workspace is the window arrangement and nothing else — which modules are in
/// their own OS windows and where those windows are. Save captures the
/// arrangement as it stands; a name is a chip that puts it back. Editing the
/// list here rather than behind a shortcut keeps it with the undocked rows it
/// acts on: the same subject, one screen.
///
/// Native only, like the undocked rows above it — the browser has one window and
/// nothing to arrange.
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn workspaces_section(
    ui: &mut egui::Ui,
    cfg: &mut sdroxide_types::UiSettings,
    workspaces: &mut Vec<sdroxide_types::Workspace>,
    name: &mut String,
) {
    use sdroxide_types::{WORKSPACE_MAX, Workspace};
    ui.label(RichText::new(tr!("settings-ui-workspaces")).size(14.0).strong().color(crate::theme::CYAN()));
    ui.add_space(2.0);
    super::note_at(
        ui,
        &tr!("settings-ui-workspaces-note"),
        Some(11.0),
    );
    ui.add_space(6.0);

    if workspaces.is_empty() {
        super::note_at(ui, &tr!("settings-ui-workspaces-none"), Some(11.0));
    }
    let mut apply: Option<usize> = None;
    let mut delete: Option<usize> = None;
    for (i, ws) in workspaces.iter().enumerate() {
        ui.horizontal(|ui| {
            let out = ws.slots.iter().filter(|s| s.detached).count();
            let chip =
                crate::chrome::chip(ui, false, ws.name.as_str()).on_hover_text(if out == 0 {
                    tr!("settings-ui-workspace-restore-home").to_string()
                } else {
                    tr!("settings-ui-workspace-restore-out", n = out)
                });
            if chip.clicked() {
                apply = Some(i);
            }
            if crate::chrome::chip(ui, false, "×").on_hover_text(tr!("settings-ui-workspace-forget")).clicked()
            {
                delete = Some(i);
            }
        });
    }

    ui.add_space(6.0);
    ui.horizontal(|ui| {
        crate::chrome::field_sized(
            ui,
            egui::vec2(220.0, 0.0),
            egui::TextEdit::singleline(name).hint_text(tr!("settings-ui-workspace-name")),
        );
        let named = !name.trim().is_empty();
        let full = workspaces.len() >= WORKSPACE_MAX;
        let can_save = named && !full;
        let save = crate::chrome::chip_enabled(ui, can_save, false, &tr!("settings-ui-workspace-save")).on_hover_text(
            if full {
                tr!("settings-ui-workspace-full")
            } else if named {
                tr!("settings-ui-workspace-save-hint").to_string()
            } else {
                tr!("settings-ui-workspace-name-first").to_string()
            },
        );
        if save.clicked() {
            let n = name.trim().to_string();
            // Saving over an existing name replaces it rather than making a
            // duplicate row the operator then has to tell apart.
            if let Some(slot) = workspaces.iter_mut().find(|w| w.name == n) {
                *slot = Workspace::capture(n, cfg);
            } else {
                workspaces.push(Workspace::capture(n, cfg));
            }
            name.clear();
        }
    });

    if let Some(i) = delete {
        workspaces.remove(i);
    }
    if let Some(i) = apply {
        workspaces[i].apply(cfg);
    }
}

/// A detachable module's name in the settings rows, in the interface language.
fn module_label(m: sdroxide_types::DetachableModule) -> String {
    use sdroxide_types::DetachableModule as M;
    match m {
        M::Panadapter => tr!("settings-ui-module-panadapter"),
        M::Panel => tr!("multi-module-panel"),
        M::Controls => tr!("settings-ui-module-controls"),
        M::AuxPanadapter => tr!("settings-ui-module-aux"),
        M::BandMenu => tr!("settings-ui-module-band-menu"),
        M::Rotor => tr!("multi-module-rotor"),
    }
}

fn broadcast_scope_label(s: sdroxide_types::BroadcastScope) -> String {
    match s {
        sdroxide_types::BroadcastScope::InView => tr!("settings-ui-broadcast-scope-in-view"),
        sdroxide_types::BroadcastScope::WholeBand => tr!("settings-ui-whole-band"),
    }
}

fn fill_style_label(s: sdroxide_types::SpectrumFillStyle) -> String {
    use sdroxide_types::SpectrumFillStyle as F;
    match s {
        F::Fixed => tr!("settings-ui-fill-fixed"),
        F::Gradient => tr!("settings-ui-fill-gradient"),
        F::Mixed => tr!("settings-ui-fill-mixed"),
    }
}

fn spot_kind_label(k: SpotKind) -> String {
    match k {
        SpotKind::HeardMe => tr!("settings-ui-spot-kind-heard-me"),
        other => other.label().to_string(),
    }
}

fn wake_label(s: crate::wake_lock::WakeStatus) -> String {
    use crate::wake_lock::WakeStatus as W;
    match s {
        W::Off => String::new(),
        W::Held => tr!("settings-ui-wake-held"),
        W::Asking => tr!("settings-ui-wake-asking"),
        W::Unsupported => tr!("settings-ui-wake-unsupported"),
        W::Refused => tr!("settings-ui-wake-refused"),
        W::Native => tr!("settings-ui-wake-native"),
    }
}

/// A grid label that sits level with the first line of a wrapped row beside it.
///
/// A grid centres a label on its whole row, so beside a row of swatches that
/// wraps onto a second line it lands between the two lines. Pinned to the top
/// of the cell at one line's height, it reads with the swatches it names.
fn first_line_label(ui: &mut egui::Ui, text: &str) -> egui::Response {
    let line = ui.spacing().interact_size.y;
    ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
        ui.allocate_ui_with_layout(
            egui::vec2(0.0, line),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| ui.label(text),
        )
        .inner
    })
    .inner
}
