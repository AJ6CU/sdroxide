//! The General tab's sound-device pickers and the server's sign-in credentials.
//!
//! The operator's own card is always shown; the radio's is only relevant to
//! the CAT / Audio interface, since every other backend carries its audio
//! in-band.

use crate::i18n::{Tr, tr};
use eframe::egui::{self, Color32, ComboBox, RichText};
use sdroxide_types::{
    CbPlan, Command, Region, RemoteAccess, SWR_LIMIT_MAX, SWR_LIMIT_MIN, swr_tune_limit,
};

use crate::app::SdroxideApp;
use crate::app::persist::band_plans_dir;
use crate::chrome::StyledCombo;

/// The IARU region dropdown: the number the band plans are published under,
/// with the part of the world it covers next to it.
///
/// Both, because neither alone identifies it for most operators — "Region 2"
/// means nothing until you know it is the Americas, and the number is what
/// every band-plan document and contest rule actually says.
pub(in crate::app) fn region_combo(ui: &mut egui::Ui, region: &mut Region) {
    ComboBox::from_id_salt("iaru-region").width(360.0).selected_text(region.tr()).show_styled(
        ui,
        |ui| {
            for r in Region::ALL {
                if ui.selectable_label(*region == r, r.tr()).clicked() {
                    *region = r;
                }
            }
        },
    );
}

/// The licence-free channel-plan dropdown: which country's (or service's)
/// channels the dial reads in — the 11 m citizens' band plans, and the 446 MHz
/// PMR446 ones. Only the channels and the channel the band opens on — the
/// band's edges are left wide, so switching plans never changes what receives
/// or transmits.
pub(in crate::app) fn cb_plan_combo(ui: &mut egui::Ui, plan: &mut CbPlan) {
    ComboBox::from_id_salt("cb-plan").width(360.0).selected_text(cb_plan_label(*plan)).show_styled(
        ui,
        |ui| {
            for p in CbPlan::ALL {
                if ui.selectable_label(*plan == p, cb_plan_label(p)).clicked() {
                    *plan = p;
                }
            }
        },
    );
}

/// A CB / PMR plan's name in the interface's language.
pub(in crate::app) fn cb_plan_label(plan: CbPlan) -> String {
    match plan {
        CbPlan::World => tr!("settings-general-cb-world"),
        CbPlan::Cept => tr!("settings-general-cb-cept"),
        CbPlan::Germany80 => tr!("settings-general-cb-germany"),
        CbPlan::Uk27_81 => tr!("settings-general-cb-uk"),
        CbPlan::Us => tr!("settings-general-cb-us"),
        CbPlan::Australia => tr!("settings-general-cb-australia"),
        CbPlan::Pmr446 => tr!("settings-general-cb-pmr446"),
        CbPlan::Dpmr446 => tr!("settings-general-cb-dpmr446"),
    }
}

/// A device dropdown ("System default" + names); calls `pick(Some(name)|None)`.
pub(in crate::app) fn device_combo(
    ui: &mut egui::Ui,
    id: &str,
    names: &[String],
    selected: &Option<String>,
    mut pick: impl FnMut(Option<String>),
) {
    let shown = selected.clone().unwrap_or_else(|| tr!("settings-general-system-default").into());
    ComboBox::from_id_salt(id).width(300.0).selected_text(shown).show_styled(ui, |ui| {
        if ui.selectable_label(selected.is_none(), tr!("settings-general-system-default")).clicked()
        {
            pick(None);
        }
        for n in names {
            if ui.selectable_label(selected.as_deref() == Some(n), n).clicked() {
                pick(Some(n.clone()));
            }
        }
    });
}

/// A device picker that can also be typed into.
///
/// [`device_combo`]'s list is a convenience, not the truth. A sound server can
/// expose a device the host API does not enumerate — a PipeWire node, a
/// capture device another program is holding, one that appears only once
/// something asks for it — and a picker that can only be pointed at leaves the
/// operator with no way to name it. That is not hypothetical here: enumerating
/// PipeWire nodes is the open half of the Bluetooth microphone work.
///
/// So the value is one editable line under the list, and typing wins. The field
/// is **controlled** — re-seeded from the value every frame rather than kept in
/// a buffer of its own — so a name picked from the list and a name typed in are
/// the same value and cannot drift apart, and there is nothing per-field to
/// keep alive between frames or to go stale when a pick arrives from elsewhere.
pub(in crate::app) fn device_field(
    ui: &mut egui::Ui,
    id: &str,
    names: &[String],
    selected: &mut Option<String>,
    what: &str,
) {
    // Stacked, not side by side: in a two-column grid row the second control
    // lands in the next cell and the entry field is clipped to a few characters
    // of whatever the name happens to start with.
    ui.vertical(|ui| {
        // The list reads the value and the closure writes it, so the value is
        // copied across rather than borrowed twice.
        let shown = selected.clone();
        device_combo(ui, id, names, &shown, |n| *selected = n);
        let mut text = selected.clone().unwrap_or_default();
        let resp = ui.add(
            egui::TextEdit::singleline(&mut text)
                .id(egui::Id::new(id).with("typed"))
                .desired_width(super::audio::DEVICE_WIDTH)
                .hint_text(tr!("settings-general-device-type-name")),
        );
        if resp.changed() {
            *selected = (!text.is_empty()).then_some(text);
        }
        resp.on_hover_text(match what {
            "speakers" => tr!("settings-general-device-hint-speakers"),
            "microphone" => tr!("settings-general-device-hint-microphone"),
            "radio-rx" => tr!("settings-general-device-hint-radio-rx"),
            "radio-tx" => tr!("settings-general-device-hint-radio-tx"),
            _ => tr!("settings-general-device-hint-other"),
        });
    });
}

/// Which station this client is signed in to, and the way back out of it.
///
/// The other half of the sign-in cookie, and it lives here because a cookie
/// outlives a reload: an operator who signed in for a day has no reason to
/// think about it again until they want to, and "somebody else's browser" is
/// exactly the case where being signed in matters. A native client has no
/// cookie — it keeps a copy of the password instead — so the same button clears
/// that, and says which it is doing rather than showing a control that quietly
/// does the wrong half.
pub(in crate::app) fn signed_in_station(ui: &mut egui::Ui, station: &str) {
    let station_key = crate::login::station_key(station);
    ui.label(RichText::new(tr!("settings-general-signed-in-station")).size(14.0).strong().color(crate::theme::CYAN()));
    ui.add_space(6.0);
    ui.label(RichText::new(station_key.clone()).size(11.5).weak());
    ui.add_space(8.0);

    let mut sign_out = false;
    ui.horizontal(|ui| {
        sign_out = crate::chrome::chip_accent(
            ui,
            false,
            RichText::new(format!(" {} ", tr!("settings-general-sign-out"))).strong().size(12.0),
            crate::theme::ALERT(),
            crate::theme::INK_ON_CYAN(),
        )
        .clicked();
    });
    sign_out.then(|| crate::login::sign_out(&station_key));

    let fallback = crate::login::cookie_fallback_stations();
    if fallback.is_empty() {
        ui.add_space(4.0);
        super::note_at(
            ui,
            &tr!("settings-general-sign-out-hint"),
            Some(11.0),
        );
    } else {
        // The cookie could not be taken, so a password is being kept in the page
        // instead. Said here because the alternative is a browser holding a
        // station's password that the operator believes it does not.
        ui.add_space(6.0);
        ui.label(
            RichText::new(tr!(
                "settings-general-fallback-password",
                stations = fallback.join(", ")
            ))
            .size(11.0)
            .color(crate::theme::gray(140)),
        );
    }
}

/// Who may connect to this machine's server (`[remote_access]` in
/// `config.toml`).
///
/// Only drawn when the engine is in this process. These are a file on the
/// machine the radio is attached to: a remote client has nothing here to read
/// them from, and offering it a box that writes to its own disk instead would
/// be worse than offering nothing — it would look as though the station's
/// password had been changed when it had not.
///
/// Written as it is typed, like the control bindings, rather than behind an
/// APPLY: the server re-reads the file for every sign-in, so there is no
/// separate step for an APPLY to stand for.
pub(in crate::app) fn remote_access_settings(ui: &mut egui::Ui, access: &mut RemoteAccess) {
    ui.label(
        RichText::new(tr!("settings-general-remote-access"))
            .size(14.0)
            .strong()
            .color(crate::theme::CYAN()),
    );
    ui.add_space(6.0);
    super::note_at(
        ui,
        &tr!("settings-general-what-remote-client-browser-page"),
        Some(11.5),
    );
    ui.add_space(8.0);
    egui::Grid::new("remote-access-grid")
        .num_columns(2)
        .spacing([12.0, 8.0])
        .min_row_height(ui.spacing().interact_size.y)
        .show(ui, |ui| {
            ui.label(tr!("settings-general-username"));
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut access.username).desired_width(200.0),
            );
            ui.end_row();
            ui.label(tr!("settings-general-password"));
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut access.password)
                    .password(true)
                    .desired_width(200.0),
            );
            ui.end_row();
        });
    ui.add_space(6.0);
    // The state of the door, said plainly. "Both boxes empty means anyone may
    // key my transmitter" is not something an operator should have to infer.
    if access.is_enforced() {
        if access.username.is_empty() {
            ui.label(
                RichText::new(tr!("settings-general-clients-must-give-password-leaving"))
                    .size(11.5)
                    .color(crate::theme::GREEN()),
            );
        } else {
            ui.label(
                RichText::new(tr!("settings-general-clients-must-sign"))
                    .size(11.5)
                    .color(crate::theme::GREEN()),
            );
        }
    } else {
        ui.label(
            RichText::new(tr!("settings-general-empty-anyone-who-can-reach"))
                .size(11.5)
                .color(crate::theme::YELLOW()),
        );
    }
    ui.add_space(4.0);
    ui.label(
        RichText::new(tr!("settings-general-stored-clear-config-toml-like"))
            .size(10.5)
            .color(crate::theme::gray(140)),
    );
}

/// The fixed trim on this radio's receive audio.
///
/// Lives beside the sound-card pickers because that is what it is for: the AF
/// rail on the strip tops out at unity, which can turn a radio down and never
/// up, and a transceiver whose USB codec puts out a quiet signal is then quiet
/// at full volume in sdroxide *and* in the operating system (issue #315). It is
/// per radio, because what it corrects is that radio's interface rather than
/// how loudly anyone wants to listen.
///
/// Edited straight into `radio_edit`, which the settings window sends and saves
/// on any change — so it takes effect while the operator is listening, with no
/// Apply and no reopen.
pub(in crate::app) fn settings_rx_audio_gain(
    ui: &mut egui::Ui,
    cfg: &mut sdroxide_types::RadioConfig,
) {
    ui.add_space(8.0);
    ui.horizontal(|ui| {
        ui.label(RichText::new(tr!("settings-general-receive-audio-gain")).strong());
        ui.end_row();
        ui.add(
            egui::DragValue::new(&mut cfg.rx_audio_gain_db)
                .speed(0.5)
                .range(-20.0..=30.0)
                .fixed_decimals(1)
                .suffix(" dB"),
        );
        if cfg.rx_audio_gain_db != 0.0 && ui.button(tr!("settings-general-t-0-db")).clicked() {
            cfg.rx_audio_gain_db = 0.0;
        }
    });
    ui.add_space(4.0);
    // Wrapped: an unwrapped label here runs the width of the page and is drawn
    // over the paragraph after it, which is what rendering this page showed.
    ui.add(
        egui::Label::new(
            RichText::new(
                tr!("settings-general-extra-gain-everything-radio-sends"),
            )
            .size(10.5)
            .color(crate::theme::gray(140)),
        )
        .wrap(),
    );
}

/// What the band plan manager remembers between frames.
#[derive(Default)]
pub(in crate::app) struct BandPlanUi {
    /// The plan whose DELETE has been clicked once and is waiting for the second.
    pub(in crate::app) delete_arm: Option<String>,
    /// The name typed beside SAVE CURRENT PLAN AS.
    pub(in crate::app) save_name: String,
}

impl SdroxideApp {
    /// Hand a band plan file the operator picked to the station.
    ///
    /// The text goes as it is; the station checks it, converts it if it is in
    /// another layout, and keeps it — this side has no folder of the station's
    /// to write to and judges nothing, so a plan it would have refused cannot be
    /// accepted here and refused there.
    pub(in crate::app) fn poll_band_plan_import(&mut self, cmds: &mut Vec<sdroxide_types::Command>) {
        let loaded = self.band_plan_import_inbox.lock().ok().and_then(|mut g| g.take());
        match loaded {
            None => {}
            Some(Ok(l)) => cmds.push(sdroxide_types::Command::ImportBandPlan {
                name: l.name.unwrap_or_else(|| "imported-plan".to_string()),
                json: l.text,
            }),
            Some(Err(e)) => self.show_notice(e),
        }
    }

    /// The station's band plans: the list to choose from, IMPORT to add one, the
    /// way back to the built-in tables, and the button that re-reads the chosen
    /// file.
    ///
    /// The same on a native screen, a remote one and a browser, because none of
    /// it touches the screen's own disk: the list is what the station announced,
    /// a plan is added by sending its *text* to the station
    /// ([`sdroxide_types::Command::ImportBandPlan`]), and choosing one is a
    /// command too. No editor here on purpose — a band plan is forty rows of
    /// numbers pasted from a published table, and a text editor does that far
    /// better than any grid this dialog could hold.
    ///
    /// Choosing a plan other than the one running withdraws the confirmation of
    /// every radio's TX and RX band lists, so the hover text says so before the
    /// click rather than after.
    pub(in crate::app) fn settings_band_plan_file(
        &self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<sdroxide_types::Command>,
    ) {
        let running = self.band_plan_name.clone();
        let shown = |file: &Option<String>| -> String {
            match file {
                None => tr!("settings-general-band-plan-built-in"),
                Some(f) => self
                    .band_plans
                    .iter()
                    .find(|p| p.file == *f)
                    .map_or_else(|| f.clone(), |p| p.name.clone()),
            }
        };
        let mut choice = running.clone();
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(tr!("settings-general-band-plan")).size(10.5).weak());
            ComboBox::from_id_salt("band-plan-choice")
                .selected_text(shown(&running))
                .show_ui(ui, |ui| {
                    ui.selectable_value(
                        &mut choice,
                        None,
                        tr!("settings-general-band-plan-built-in"),
                    );
                    for p in &self.band_plans {
                        let label = if p.country.is_empty() {
                            p.name.clone()
                        } else {
                            format!("{} — {}", p.name, p.country)
                        };
                        ui.selectable_value(&mut choice, Some(p.file.clone()), label);
                    }
                });
        })
        .response
        .on_hover_text(tr!("settings-general-band-plan-choose-hint"));
        if choice != running {
            cmds.push(sdroxide_types::Command::SelectBandPlan { name: choice });
            self.follow_station_plan(cmds);
        }

        // This radio's own plan, beside the station's: one radio on a national
        // plan and another on the built-in tables is an ordinary station. Shown
        // once this radio's configuration has been announced.
        let radio_plan = self.radio_cfg.as_ref().map(|c| c.band_plan.clone());
        if let Some(radio_plan) = radio_plan {
            let radio_shown = |file: &Option<String>| -> String {
                match file {
                    None => tr!("settings-general-band-plan-radio-follow"),
                    Some(f) => self
                        .band_plans
                        .iter()
                        .find(|p| p.file == *f)
                        .map_or_else(|| f.clone(), |p| p.name.clone()),
                }
            };
            let mut radio_choice = radio_plan.clone();
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(tr!("settings-general-band-plan-radio")).size(10.5).weak());
                ComboBox::from_id_salt("band-plan-radio-choice")
                    .selected_text(radio_shown(&radio_plan))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut radio_choice,
                            None,
                            tr!("settings-general-band-plan-radio-follow"),
                        );
                        for p in &self.band_plans {
                            ui.selectable_value(&mut radio_choice, Some(p.file.clone()), p.name.clone());
                        }
                    });
            })
            .response
            .on_hover_text(tr!("settings-general-band-plan-radio-hint"));
            if radio_choice != radio_plan {
                cmds.push(sdroxide_types::Command::SelectRadioBandPlan { name: radio_choice });
            }
        }

        ui.horizontal_wrapped(|ui| {
            if crate::chrome::chip(
                ui,
                false,
                RichText::new(tr!("settings-general-band-plan-import")).size(10.5),
            )
            .on_hover_text(tr!("settings-general-band-plan-import-hint"))
            .clicked()
            {
                crate::download::load_text(
                    "Band plan",
                    &["json"],
                    self.band_plan_import_inbox.clone(),
                );
            }
            // One click back to the default, only when there is somewhere to go
            // back from.
            if running.is_some()
                && crate::chrome::chip(
                    ui,
                    false,
                    RichText::new(tr!("settings-general-band-plan-reset")).size(10.5),
                )
                .on_hover_text(tr!("settings-general-band-plan-reset-hint"))
                .clicked()
            {
                cmds.push(sdroxide_types::Command::SelectBandPlan { name: None });
            }
            if crate::chrome::chip(
                ui,
                false,
                RichText::new(tr!("settings-general-reload-band-plan")).size(10.5),
            )
            .on_hover_text(tr!("settings-general-re-read-band-plan-file"))
            .clicked()
            {
                cmds.push(sdroxide_types::Command::ReloadBandPlan);
            }
        });
        ui.add(
            egui::Label::new(
                RichText::new(match &running {
                    Some(_) => tr!("settings-general-running-band-plan", name = shown(&running)),
                    None => tr!("settings-general-running-built-iaru-tables"),
                })
                .size(10.5)
                .weak(),
            )
            .wrap(),
        );
        ui.add_space(4.0);
        // Only where the folder is actually on this machine. A remote client
        // showing its own config path would be pointing at the wrong computer.
        let dir = (!self.ctrl.engine_is_remote())
            .then(band_plans_dir)
            .flatten()
            .map(|p| p.display().to_string());
        ui.label(
            RichText::new(match &dir {
                Some(p) => tr!("settings-general-band-plan-folder", path = p.as_str()),
                None => tr!("settings-general-band-plan-station-folder"),
            })
            .size(10.5)
            .color(crate::theme::gray(140)),
        );
        ui.add_space(4.0);
        self.settings_band_plan_manager(ui, cmds);
    }

    /// After a plan is chosen for the station, put this radio back on it: a radio
    /// that kept a plan of its own would otherwise ignore the choice. A radio
    /// that already follows the station needs nothing.
    fn follow_station_plan(&self, cmds: &mut Vec<sdroxide_types::Command>) {
        if self.radio_cfg.as_ref().is_some_and(|c| c.band_plan.is_some()) {
            cmds.push(sdroxide_types::Command::SelectRadioBandPlan { name: None });
        }
    }

    /// The plans the station holds, one row each, with what can be done to
    /// them: run, download, delete — and, below, keep the plan that runs under a
    /// name and put the shipped plans back.
    ///
    /// Everything here is a command to the station or a file this screen saves
    /// on its own computer (a download, which a browser does as readily as a
    /// native screen); none of it touches a folder on this machine. A delete
    /// asks for a second click, and the plan that is running can be neither
    /// deleted nor run again.
    fn settings_band_plan_manager(&self, ui: &mut egui::Ui, cmds: &mut Vec<sdroxide_types::Command>) {
        use sdroxide_types::Command;
        let running = self.band_plan_name.clone();
        let radio_plan = self.radio_cfg.as_ref().and_then(|c| c.band_plan.clone());
        let weak = |t: String| RichText::new(t).size(10.5).color(crate::theme::gray(140));
        egui::CollapsingHeader::new(
            RichText::new(tr!("settings-general-band-plan-manager")).size(10.5),
        )
        .id_salt("band-plan-manager")
        .show(ui, |ui| {
            if self.band_plans.is_empty() {
                ui.label(weak(tr!("settings-general-band-plan-empty")));
            }
            egui::Grid::new("band-plan-manager-grid")
                .num_columns(5)
                .spacing([10.0, 3.0])
                .striped(true)
                .show(ui, |ui| {
                    for p in &self.band_plans {
                        let is_running = running.as_deref() == Some(p.file.as_str());
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(&p.name).size(10.5).strong());
                            if is_running {
                                ui.label(
                                    RichText::new(tr!("settings-general-band-plan-running-tag"))
                                        .size(9.0)
                                        .color(crate::theme::gray(190)),
                                );
                            }
                            if radio_plan.as_deref() == Some(p.file.as_str()) {
                                ui.label(
                                    RichText::new(tr!("settings-general-band-plan-radio-tag"))
                                        .size(9.0)
                                        .color(crate::theme::gray(190)),
                                );
                            }
                            if p.shipped {
                                ui.label(weak(tr!("settings-general-band-plan-shipped-tag")));
                            }
                        });
                        ui.label(weak(p.country.clone()));
                        ui.label(weak(p.author.clone()));
                        let (y, m, d, ..) = sdroxide_types::utc_ymd_hms(p.modified as i64);
                        let when = if p.modified == 0 {
                            String::new()
                        } else {
                            format!("{y:04}-{m:02}-{d:02}")
                        };
                        ui.label(weak(format!(
                            "{when}  {}",
                            tr!("settings-general-band-plan-size", kb = p.bytes.div_ceil(1024))
                        )));
                        ui.horizontal(|ui| {
                            let run = ui
                                .add_enabled_ui(!is_running, |ui| {
                                    crate::chrome::chip(
                                        ui,
                                        false,
                                        RichText::new(tr!("settings-general-band-plan-run"))
                                            .size(10.0),
                                    )
                                })
                                .inner
                                .on_hover_text(tr!("settings-general-band-plan-run-hint"));
                            if run.clicked() {
                                cmds.push(Command::SelectBandPlan { name: Some(p.file.clone()) });
                                self.follow_station_plan(cmds);
                            }
                            // Only on this radio: shown once its configuration is
                            // known, and not for the plan it already runs.
                            if self.radio_cfg.is_some() {
                                let own = radio_plan.as_deref() == Some(p.file.as_str());
                                if ui
                                    .add_enabled_ui(!own, |ui| {
                                        crate::chrome::chip(
                                            ui,
                                            false,
                                            RichText::new(tr!("settings-general-band-plan-run-radio"))
                                                .size(10.0),
                                        )
                                    })
                                    .inner
                                    .on_hover_text(tr!("settings-general-band-plan-radio-hint"))
                                    .clicked()
                                {
                                    cmds.push(Command::SelectRadioBandPlan {
                                        name: Some(p.file.clone()),
                                    });
                                }
                            }
                            if crate::chrome::chip(
                                ui,
                                false,
                                RichText::new(tr!("settings-general-band-plan-download"))
                                    .size(10.0),
                            )
                            .on_hover_text(tr!("settings-general-band-plan-download-hint"))
                            .clicked()
                            {
                                *self.band_plan_export_wanted.borrow_mut() = Some(p.file.clone());
                                cmds.push(Command::ExportBandPlan { name: p.file.clone() });
                            }
                            let armed =
                                self.band_plan_ui.borrow().delete_arm.as_deref() == Some(&p.file);
                            let del = ui
                                .add_enabled_ui(!is_running, |ui| {
                                    crate::chrome::chip(
                                        ui,
                                        armed,
                                        RichText::new(if armed {
                                            tr!("settings-general-band-plan-delete-confirm")
                                        } else {
                                            tr!("settings-general-band-plan-delete")
                                        })
                                        .size(10.0),
                                    )
                                })
                                .inner
                                .on_hover_text(tr!("settings-general-band-plan-delete-hint"));
                            if del.clicked() {
                                if armed {
                                    self.band_plan_ui.borrow_mut().delete_arm = None;
                                    cmds.push(Command::DeleteBandPlan { name: p.file.clone() });
                                } else {
                                    self.band_plan_ui.borrow_mut().delete_arm =
                                        Some(p.file.clone());
                                }
                            }
                        });
                        ui.end_row();
                    }
                });
            ui.add_space(4.0);
            ui.horizontal_wrapped(|ui| {
                let mut st = self.band_plan_ui.borrow_mut();
                ui.add(
                    egui::TextEdit::singleline(&mut st.save_name)
                        .hint_text(tr!("settings-general-band-plan-name-hint"))
                        .desired_width(150.0),
                );
                let save = crate::chrome::chip(
                    ui,
                    false,
                    RichText::new(tr!("settings-general-band-plan-save-current")).size(10.5),
                )
                .on_hover_text(tr!("settings-general-band-plan-save-current-hint"));
                if save.clicked() && !st.save_name.trim().is_empty() {
                    cmds.push(Command::SaveCurrentBandPlan { name: st.save_name.trim().to_string() });
                }
                if crate::chrome::chip(
                    ui,
                    false,
                    RichText::new(tr!("settings-general-band-plan-restore")).size(10.5),
                )
                .on_hover_text(tr!("settings-general-band-plan-restore-hint"))
                .clicked()
                {
                    cmds.push(Command::RestoreShippedBandPlans);
                }
            });
        });
    }

    /// Take this station's settings away as one file, and put one back.
    ///
    /// The answer to "how do I copy all this to my other machine" being
    /// "screenshots" (issue #356). Native only, and only where the settings are
    /// on *this* machine: a browser client has no filesystem, and a remote one
    /// would be exporting its own laptop's configuration rather than the
    /// station's, which is the opposite of what was asked for.
    #[cfg(not(target_arch = "wasm32"))]
    pub(in crate::app) fn settings_transfer(
        &self,
        ui: &mut egui::Ui,
        export: &mut bool,
        import: &mut bool,
    ) {
        ui.label(RichText::new(tr!("settings-general-settings-file")).strong());
        ui.end_row();
        if self.ctrl.engine_is_remote() {
            ui.label(
                RichText::new(tr!("settings-general-settings-are-machine-radio-attached"))
                    .size(10.5)
                    .color(crate::theme::gray(140)),
            );
            return;
        }
        ui.horizontal_wrapped(|ui| {
            if crate::chrome::chip(
                ui,
                false,
                RichText::new(tr!("settings-general-export")).size(10.5),
            )
            .on_hover_text(tr!("settings-general-write-every-setting-station-radios"))
            .clicked()
            {
                *export = true;
            }
            if crate::chrome::chip(
                ui,
                false,
                RichText::new(tr!("settings-general-import")).size(10.5),
            )
            .on_hover_text(tr!("settings-general-replace-station-s-settings-ones"))
            .clicked()
            {
                *import = true;
            }
        });
        if let Some(note) = &self.settings_transfer_note {
            ui.add_space(4.0);
            ui.add(
                egui::Label::new(RichText::new(note).size(10.5).color(Color32::LIGHT_GREEN)).wrap(),
            );
        }
        ui.add_space(4.0);
        ui.add(
            egui::Label::new(
                RichText::new(tr!("settings-general-import-overwrites-what-here-file"))
                    .size(10.5)
                    .color(crate::theme::gray(140)),
            )
            .wrap(),
        );
    }

    /// The browser client has no filesystem, and the settings it would export
    /// are on the engine's machine in any case.
    #[cfg(target_arch = "wasm32")]
    pub(in crate::app) fn settings_transfer(
        &self,
        _ui: &mut egui::Ui,
        _export: &mut bool,
        _import: &mut bool,
    ) {
    }

    /// Carry out what [`Self::settings_transfer`]'s buttons asked for, after
    /// the window closure has given `&mut self` back — see [`SettingsIo`].
    #[cfg(not(target_arch = "wasm32"))]
    pub(in crate::app) fn run_settings_transfer(&mut self, export: bool, import: bool) {
        if export {
            self.settings_transfer_note = Some(match sdroxide_config::transfer::export_json() {
                Ok(json) => {
                    let note = sdroxide_config::transfer::export()
                        .map(|b| b.summary())
                        .unwrap_or_else(|_| "settings".into());
                    crate::download::save("sdroxide-settings.json", json.as_bytes());
                    tr!("settings-general-exported", note = note)
                }
                Err(e) => tr!("settings-general-export-failed", error = e.to_string()),
            });
        }
        if import {
            self.settings_transfer_note = None;
            crate::download::load_text(
                &tr!("settings-general-sdroxide-settings"),
                &["json"],
                self.settings_import_inbox.clone(),
            );
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub(in crate::app) fn run_settings_transfer(&mut self, _export: bool, _import: bool) {}

    /// Where recordings are written, and how to change it.
    ///
    /// A `config.toml` path on the machine that records, so it is offered where
    /// the settings are on this machine — like [`Self::settings_transfer`], and
    /// for the same reason: a remote client would be changing its own laptop's
    /// folder, not the station's.
    #[cfg(not(target_arch = "wasm32"))]
    /// The folder row alone, for a page that has already said what it is
    /// called — so the two headings do not sit one line apart saying the same
    /// word twice.
    pub(in crate::app) fn settings_recordings_dir(
        &self,
        ui: &mut egui::Ui,
        choose: &mut bool,
        reset: &mut bool,
    ) {
        if self.ctrl.engine_is_remote() {
            ui.label(
                RichText::new(
                    tr!("settings-general-recordings-remote"),
                )
                .size(10.5)
                .color(crate::theme::gray(140)),
            );
            return;
        }
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(tr!("settings-general-folder")).size(11.0).color(crate::theme::gray(150)));
            ui.label(
                RichText::new(self.recordings_dir.display().to_string())
                    .size(10.5)
                    .monospace()
                    .color(crate::theme::gray(180)),
            );
            if crate::chrome::chip(ui, false, RichText::new(tr!("settings-general-choose")).size(10.5))
                .on_hover_text(tr!("settings-general-choose-hint"))
                .clicked()
            {
                *choose = true;
            }
            if crate::chrome::chip(ui, false, RichText::new(tr!("settings-general-default")).size(10.5))
                .on_hover_text(
                    tr!("settings-general-default-hint"),
                )
                .clicked()
            {
                *reset = true;
            }
        });
        ui.add_space(4.0);
        ui.add(
            egui::Label::new(
                RichText::new(
                    tr!("settings-general-recordings-dir-note"),
                )
                .size(10.5)
                .color(crate::theme::gray(140)),
            )
            .wrap(),
        );
    }

    /// The browser client has no filesystem and no engine of its own to record
    /// with.
    #[cfg(target_arch = "wasm32")]
    pub(in crate::app) fn settings_recordings_dir(
        &self,
        _ui: &mut egui::Ui,
        _choose: &mut bool,
        _reset: &mut bool,
    ) {
    }

    /// Carry out what [`Self::settings_recordings_dir`]'s buttons asked for,
    /// after the window closure has given `&mut self` back — see [`SettingsIo`].
    #[cfg(not(target_arch = "wasm32"))]
    pub(in crate::app) fn handle_recordings_dir(&mut self, choose: bool, reset: bool) {
        if !choose && !reset {
            return;
        }
        let saved = if choose {
            let mut dialog = rfd::FileDialog::new();
            if let Some(parent) = self.recordings_dir.parent() {
                dialog = dialog.set_directory(parent);
            }
            match dialog.pick_folder() {
                Some(picked) => sdroxide_config::save_recordings_dir(Some(&picked)),
                // A cancelled picker changes nothing.
                None => Ok(()),
            }
        } else {
            sdroxide_config::save_recordings_dir(None)
        };
        if let Err(e) = saved {
            eprintln!("failed to save the recordings directory: {e}");
        }
        // Re-read whatever is now in effect, so the row shows the truth at once.
        self.recordings_dir = crate::app::persist::recordings_dir_for_display();
    }

    #[cfg(target_arch = "wasm32")]
    pub(in crate::app) fn handle_recordings_dir(&mut self, _choose: bool, _reset: bool) {}

    /// Apply a settings bundle the operator picked, once the picker thread has
    /// delivered it. Drained every frame beside the ADIF import.
    #[cfg(not(target_arch = "wasm32"))]
    pub(in crate::app) fn poll_settings_import(&mut self) {
        let loaded = self.settings_import_inbox.lock().ok().and_then(|mut g| g.take());
        let Some(loaded) = loaded else { return };
        // Said on screen rather than on stderr: the operator who pressed the
        // button is looking at the window, and on Windows there is no console
        // behind it to print to.
        self.settings_transfer_note = Some(match loaded {
            Err(e) => tr!("settings-general-import-failed", error = e.to_string()),
            Ok(loaded) => match sdroxide_config::transfer::import(&loaded.text) {
                Err(e) => tr!("settings-general-import-failed", error = e.to_string()),
                Ok(report) => {
                    let mut msg = tr!("settings-general-imported", summary = report.summary());
                    for (path, why) in report.skipped.iter().take(4) {
                        msg.push('\n');
                        msg.push_str(&tr!(
                            "settings-general-import-skipped",
                            path = path.to_string(),
                            why = why.to_string()
                        ));
                    }
                    msg
                }
            },
        });
    }

    #[cfg(target_arch = "wasm32")]
    pub(in crate::app) fn poll_settings_import(&mut self) {}

    /// The SWR guard: arm it, and set the ratio it stops transmitting at.
    ///
    /// Reads the live values out of the broadcast TX state rather than off
    /// disk, so a remote client shows the radio's setting and not its own
    /// machine's `config.toml`, which would be a different antenna entirely.
    /// The command is sent only on an actual change, since a `DragValue` reports
    /// its value every frame it is dragged and each one would be a config write.
    pub(in crate::app) fn settings_swr_guard(&self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        ui.label(RichText::new(tr!("settings-general-swr-guard")).strong());
        ui.end_row();
        ui.add_space(4.0);

        let mut enabled = self.state.tx.swr_guard;
        // The engine clamps this too; matching the range here keeps the widget
        // from offering a value that would come back changed.
        let mut limit = self.state.tx.swr_limit.clamp(SWR_LIMIT_MIN, SWR_LIMIT_MAX);

        ui.horizontal(|ui| {
            if crate::chrome::checkbox(
                ui,
                &mut enabled,
                tr!("settings-general-stop-transmitting-high-swr"),
            )
            .changed()
            {
                cmds.push(Command::SetSwrGuard { enabled, limit });
            }
        });
        ui.add_enabled_ui(enabled, |ui| {
            ui.horizontal(|ui| {
                ui.label(tr!("settings-general-trip"));
                let r = ui.add(
                    egui::DragValue::new(&mut limit)
                        .speed(0.1)
                        .range(SWR_LIMIT_MIN..=SWR_LIMIT_MAX)
                        .fixed_decimals(1)
                        .suffix(":1"),
                );
                // `drag_stopped` and `lost_focus`, not `changed`: one command
                // per settled value rather than one per frame of the drag.
                if (r.drag_stopped() || r.lost_focus()) && limit != self.state.tx.swr_limit {
                    cmds.push(Command::SetSwrGuard { enabled, limit });
                }
                // The tune limit is derived from this one rather than typed, so
                // it is shown here: an operator who has just had a tune-up
                // stopped is told a figure, and this is where they find out
                // where it came from.
                ui.label(
                    RichText::new(tr!(
                        "settings-general-swr-tune-limit",
                        limit = format!("{:.1}", swr_tune_limit(limit))
                    ))
                    .size(11.0)
                    .color(crate::theme::gray(140)),
                );
            });
        });

        ui.add_space(6.0);
        super::note_rich(
            ui,
            &tr!("settings-general-stops-transmission-when-radio-reports"),
            Some(10.5),
            None,
        );
        if let Some(swr) = self.state.tx.swr_tripped {
            ui.add_space(4.0);
            ui.label(
                RichText::new(tr!("settings-general-swr-tripped", swr = format!("{swr:.1}")))
                    .size(11.0)
                    .color(Color32::from_rgb(255, 190, 70)),
            );
        }
    }

    /// Forget every mode's remembered settings overrides.
    ///
    /// The **DEFAULTS** chip beside the receiver controls puts one mode back;
    /// this is the way back from a long session of fiddling without walking
    /// every mode. A command rather than a config edit, so it reaches whichever
    /// engine is running — local or remote — and undoes itself.
    pub(in crate::app) fn settings_mode_defaults(
        &self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<sdroxide_types::Command>,
    ) {
        ui.label(RichText::new(tr!("settings-general-per-mode-settings")).strong());
        ui.end_row();
        ui.add_space(4.0);
        ui.horizontal_wrapped(|ui| {
            if crate::chrome::chip(
                ui,
                false,
                RichText::new(tr!("settings-general-reset-every-mode")).size(10.5),
            )
            .on_hover_text(tr!("settings-general-forget-every-mode-s-remembered"))
            .clicked()
            {
                cmds.push(sdroxide_types::Command::ResetModeDefaults { mode: None });
            }
        });
        ui.add_space(4.0);
        super::note_rich(
            ui,
            &tr!("settings-general-selecting-mode-lays-mode-s"),
            Some(10.5),
            None,
        );
    }
}

#[cfg(test)]
mod device_entry_tests {
    use super::device_field;
    use eframe::egui;

    const LIST: &[&str] = &["Built-in Audio", "USB Audio"];
    const UNLISTED: &str = "bluez_input.AA_BB_CC_DD_EE_FF";

    fn field_id() -> egui::Id {
        egui::Id::new("t").with("typed")
    }

    /// One frame: draw the picker, and optionally feed the entry field keys.
    fn frame(ctx: &egui::Context, selected: &mut Option<String>, keys: &[egui::Event]) {
        let names: Vec<String> = LIST.iter().map(|s| (*s).into()).collect();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::Vec2::new(700.0, 400.0),
            )),
            events: keys.to_vec(),
            ..Default::default()
        };
        let _ = crate::test_support::run_frame(ctx, input, |ui| {
            device_field(ui, "t", &names, selected, "the test device");
        });
    }

    /// One frame that only draws and claims the keyboard, so the next frame's
    /// keys go to the entry field rather than to nothing.
    fn focus(ctx: &egui::Context, selected: &mut Option<String>) {
        let _ = crate::test_support::run_frame(
            ctx,
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::Vec2::new(700.0, 400.0),
                )),
                ..Default::default()
            },
            |ui| {
                let names: Vec<String> = LIST.iter().map(|s| (*s).into()).collect();
                device_field(ui, "t", &names, selected, "the test device");
                ui.memory_mut(|m| m.request_focus(field_id()));
            },
        );
    }

    fn text(s: &str) -> egui::Event {
        egui::Event::Text(s.to_owned())
    }

    fn backspace() -> egui::Event {
        egui::Event::Key {
            key: egui::Key::Backspace,
            physical_key: None,
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::default(),
        }
    }

    /// **A name the list does not carry is still enterable.**
    ///
    /// The whole point of the text field. A sound server can offer a device the
    /// host API does not enumerate — enumerating PipeWire nodes is the open half
    /// of the Bluetooth microphone work — and a picker that can only be pointed
    /// at leaves the operator with no way to name it at all. Typed with real key
    /// events, not by poking the value: a test that sets the field itself is
    /// asserting that a struct holds what it was given.
    #[test]
    fn a_device_the_list_does_not_show_can_still_be_typed() {
        let ctx = egui::Context::default();
        let mut selected = None;
        // Focus the entry field, then type into it as a person would.
        focus(&ctx, &mut selected);
        let keys: Vec<egui::Event> = std::iter::once(text(UNLISTED)).collect();
        frame(&ctx, &mut selected, &keys);

        assert_eq!(
            selected.as_deref(),
            Some(UNLISTED),
            "a device name that is not in the list must be enterable — the list is a \
             convenience, not the truth"
        );
        assert!(
            !LIST.contains(&UNLISTED),
            "the premise of this test: {UNLISTED} is deliberately not one of {LIST:?}"
        );
    }

    /// **An emptied field means the system default**, not a device named "".
    ///
    /// `None` is how the rest of the audio code spells "system default"
    /// everywhere, so a field left holding an empty string would ask the sound
    /// server for a device called nothing.
    #[test]
    fn clearing_the_field_goes_back_to_the_system_default() {
        let ctx = egui::Context::default();
        let mut selected = Some("USB Audio".to_string());
        focus(&ctx, &mut selected);
        // One backspace per character typed so far.
        let keys: Vec<egui::Event> = (0.."USB Audio".len()).map(|_| backspace()).collect();
        frame(&ctx, &mut selected, &keys);
        assert_eq!(
            selected, None,
            "an empty entry field is the system default, not an empty device name"
        );
    }
}
