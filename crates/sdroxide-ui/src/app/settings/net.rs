//! The Spots, Uploads and FreeDV tabs — the network cockpit's settings.
//!
//! All three edit one [`NetworkConfig`], which the engine persists when it
//! applies it, so each has an APPLY button rather than saving as you type:
//! these are credentials and connection details, and half-typed ones would
//! reconnect the feeds on every keystroke.

use crate::i18n::tr;
use eframe::egui::{self, Color32, RichText};

/// A dropdown over an enum's `ALL`, using its `label()`.
/// UI / display preferences: frame rate, waterfall scroll speed, spectrum speed.
/// Section heading for the Spots / Uploads settings tabs.
pub(in crate::app) fn net_heading(ui: &mut egui::Ui, text: &str) {
    ui.add_space(6.0);
    ui.label(RichText::new(text).size(12.0).strong().color(crate::theme::CYAN()));
}

/// One Reverse Beacon Network connection: the fields are the same for the
/// first and the second, so only the heading and the config differ.
pub(in crate::app) fn rbn_block(
    ui: &mut egui::Ui,
    title: &str,
    cfg: &mut sdroxide_types::RbnConfig,
    cmds: &mut String,
) {
    net_heading(ui, title);
    crate::chrome::checkbox(ui, &mut cfg.enabled, tr!("settings-enabled")).on_hover_text(
        tr!("settings-net-rbn-enabled-hint"),
    );
    net_row(ui, &tr!("settings-host"), &mut cfg.host, 220.0);
    ui.horizontal(|ui| {
        ui.add_sized([96.0, 22.0], egui::Label::new(tr!("settings-port")));
        ui.add(egui::DragValue::new(&mut cfg.port).range(1..=65535))
            .on_hover_text(tr!("settings-net-rbn-port-hint"));
    });
    net_row(ui, &tr!("settings-login-call"), &mut cfg.login, 140.0);
    ui.horizontal(|ui| {
        ui.add_sized([96.0, 22.0], egui::Label::new(tr!("settings-commands")));
        crate::chrome::field(
            ui,
            egui::TextEdit::multiline(cmds)
                .desired_rows(2)
                .hint_text(tr!("settings-one-per-line-e-g-2"))
                .desired_width(220.0),
        )
        .on_hover_text(
            tr!("settings-sent-after-login-place-narrow"),
        );
    });
}

/// The DX cluster's own status line on the Spots tab: whether the link the
/// operator configured is delivering spots. Same shape as the FreeDV Reporter's
/// line on its own tab, with the "DX" the operator asked to put in front of the
/// station.
pub(in crate::app) fn dx_cluster_status(ui: &mut egui::Ui, status: &Option<String>) {
    if let Some(s) = status
        && let Some(body) = s.strip_prefix("Cluster:")
    {
        ui.label(RichText::new(tr!("settings-net-dx-status", body = body.trim_start())).weak());
    }
}

/// The operator's own **Wavelog**: address, key, certificate pin, station
/// location, and the two things that make it a logbook rather than a box that
/// takes uploads — a download, and a test that says what the server answered.
///
/// Its own section rather than a row beside eQSL and Club Log, because it is a
/// different kind of service: theirs are companies with one endpoint and an
/// account, this is a server somebody runs, at an address they type, whose
/// certificate may be its own.
pub(in crate::app) fn wavelog_section(
    ui: &mut egui::Ui,
    app: &crate::app::SdroxideApp,
    cmds: &mut Vec<sdroxide_types::Command>,
    cfg: &mut sdroxide_types::WavelogConfig,
) {
    super::note_rich(
        ui,
        &tr!("settings-net-wavelog-intro"),
        Some(11.0),
        None,
    );
    net_row(ui, &tr!("settings-wavelog-address"), &mut cfg.url, 300.0);
    ui.label(
        RichText::new(
            tr!("settings-net-wavelog-address-hint"),
        )
        .size(10.5)
        .color(crate::theme::gray(140)),
    );
    net_secret(ui, &tr!("settings-wavelog-api-key"), &mut cfg.api_key, 220.0);
    ui.label(
        RichText::new(
            tr!("settings-net-wavelog-key-hint"),
        )
        .size(10.5)
        .color(crate::theme::gray(140)),
    );

    // The station location, once the server has said which ones it has. Until
    // then a number the operator cannot check, which is why the test button
    // comes before it.
    let stations = app.wavelog_probe.as_ref().map(|p| p.stations.clone()).unwrap_or_default();
    ui.horizontal(|ui| {
        if !stations.is_empty() {
            ui.add_sized([96.0, 22.0], egui::Label::new(tr!("settings-station")));
            let current = stations.iter().find(|s| s.id == cfg.station_id).map(|s| s.name.clone());
            egui::ComboBox::from_id_salt("wavelog-station")
                .width(220.0)
                .selected_text(current.unwrap_or_else(|| tr!("settings-net-pick-one").into()))
                .show_ui(ui, |ui| {
                    for st in &stations {
                        let label = if st.callsign.trim().is_empty() {
                            st.name.clone()
                        } else {
                            format!("{} ({})", st.name, st.callsign)
                        };
                        ui.selectable_value(&mut cfg.station_id, st.id, label);
                    }
                });
        } else {
            ui.add_sized([96.0, 22.0], egui::Label::new(tr!("settings-station")));
            ui.label(
                RichText::new(tr!("settings-net-test-to-list-locations"))
                    .size(10.5)
                    .color(crate::theme::gray(150)),
            );
        }
    });
    // The pin, when the server answered with a certificate of its own.
    if let Some(pin) = app.wavelog_probe.as_ref().and_then(|p| p.untrusted_cert.clone()) {
        ui.horizontal(|ui| {
            ui.add_sized([96.0, 22.0], egui::Label::new(tr!("settings-picup-certificate")));
            ui.label(
                RichText::new(tr!("settings-net-pin-own", pin = &pin[..pin.len().min(16)]))
                    .size(10.5)
                    .color(crate::theme::YELLOW()),
            );
            if crate::chrome::chip(ui, !cfg.cert_sha256.is_empty(), tr!("settings-net-trust"))
                .on_hover_text(
                    tr!("settings-net-trust-hint"),
                )
                .clicked()
            {
                cfg.cert_sha256 = pin;
            }
        });
    }

    ui.horizontal(|ui| {
        let pending = app.login_tests_pending.contains(&sdroxide_types::LoginTarget::Wavelog);
        if ui
            .add_enabled(!pending, egui::Button::new(tr!("settings-net-test-wavelog")))
            .on_hover_text(
                tr!("settings-net-test-wavelog-hint"),
            )
            .clicked()
        {
            // The edited copy, not the applied one: the engine tests what it
            // has, so a key typed a moment ago would otherwise be checked
            // against the old one — see `login_test_row`.
            cmds.push(sdroxide_types::Command::SetNetworkConfig(app.net_edit_snapshot()));
            cmds.push(sdroxide_types::Command::TestLogin(sdroxide_types::LoginTarget::Wavelog));
        }
        if let Some(r) = app.login_tests.get(&sdroxide_types::LoginTarget::Wavelog) {
            let (mark, colour) = if r.ok {
                ("✔", crate::theme::GREEN())
            } else {
                ("✖", Color32::from_rgb(255, 120, 120))
            };
            ui.label(RichText::new(format!("{mark} {}", r.message)).size(11.0).color(colour));
        }
    });

    ui.add_space(6.0);
    crate::chrome::checkbox(ui, &mut cfg.download, tr!("settings-net-wavelog-download"));
    ui.add_enabled_ui(cfg.download, |ui| {
        net_row_int(ui, &tr!("settings-net-every-min"), &mut cfg.sync_interval_min);
        if !stations.is_empty() {
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(tr!("settings-net-locations")).size(11.0));
                for st in &stations {
                    let mut on = cfg.download_station_ids.contains(&st.id)
                        || (cfg.download_station_ids.is_empty() && st.id == cfg.station_id);
                    if crate::chrome::chip(ui, on, st.name.as_str()).clicked() {
                        on = !on;
                        let mut ids = cfg.download_station_ids.clone();
                        if ids.is_empty() {
                            // The empty list means "the one uploads go to", so
                            // the first tick has to say that one explicitly.
                            ids.push(cfg.station_id);
                            ids.retain(|i| *i != st.id || on);
                            if on && !ids.contains(&st.id) {
                                ids.push(st.id);
                            }
                        } else if on {
                            if !ids.contains(&st.id) {
                                ids.push(st.id);
                            }
                        } else {
                            ids.retain(|i| *i != st.id);
                        }
                        cfg.download_station_ids = ids;
                    }
                }
            });
        }
    });
    ui.horizontal(|ui| {
        if ui
            .add_enabled(
                cfg.is_configured() && !app.wavelog_sync_running,
                egui::Button::new(tr!("settings-net-download-now")),
            )
            .on_hover_text(
                tr!("settings-net-download-now-hint"),
            )
            .clicked()
        {
            app.wavelog_sync_request.set(true);
        }
        if let Some(s) = &app.wavelog_sync_status {
            ui.label(RichText::new(s).size(11.0).color(crate::theme::gray(170)));
        }
    });
    super::note_at(
        ui,
        &tr!("settings-net-wavelog-upload-note"),
        Some(10.5),
    );
}

/// A whole-number field for the network settings tabs.
fn net_row_int(ui: &mut egui::Ui, label: &str, val: &mut u32) {
    ui.horizontal(|ui| {
        ui.add_sized([96.0, 22.0], egui::Label::new(label));
        let mut v = *val as i64;
        if crate::chrome::field(ui, egui::DragValue::new(&mut v).range(1..=1440).speed(1.0))
            .changed()
        {
            *val = v.clamp(1, 1440) as u32;
        }
    });
}

/// **Dynamic DNS**: one update address, one interval, and nothing else.
///
/// Empty is off — the hint says so, and the engine of this is a thread that is
/// never started while the field is empty. The address carries the operator's
/// token, so what is shown back is the provider's answer and never the address.
pub(in crate::app) fn dyndns_section(
    ui: &mut egui::Ui,
    cfg: &mut sdroxide_types::NetworkConfig,
    // (unix, ok, provider's word) — the last answer, when there is one.
    last: Option<(i64, bool, String)>,
) {
    net_heading(ui, &tr!("settings-dyndns"));
    ui.label(
        RichText::new(tr!("settings-dyndns-hint"))
            .size(10.5)
            .color(crate::theme::gray(140)),
    );
    ui.add_space(2.0);
    // A secret: the address carries the operator's token, so the field hides it
    // rather than putting it on a screenshot (review of #425).
    net_secret(ui, &tr!("settings-dyndns-address"), &mut cfg.dyndns_url, 300.0);
    ui.horizontal(|ui| {
        ui.add_sized([96.0, 22.0], egui::Label::new(tr!("settings-dyndns-every")));
        let mut minutes = cfg.dyndns_minutes;
        if ui
            .add(egui::DragValue::new(&mut minutes).range(1..=1440).suffix(" min"))
            .changed()
        {
            cfg.dyndns_minutes = minutes;
        }
    });
    ui.label(
        RichText::new(tr!("settings-dyndns-token-note"))
            .size(10.0)
            .color(crate::theme::gray(130)),
    );
    let line = if cfg.dyndns_url.trim().is_empty() {
        Some((tr!("settings-dyndns-off").to_string(), crate::theme::gray(140)))
    } else {
        match last {
            Some((at, ok, text)) => {
                let when = crate::time::utc_clock(at);
                let colour = if ok { crate::theme::GREEN() } else { crate::theme::ALERT() };
                Some((tr!("settings-dyndns-last", when = when, answer = text), colour))
            }
            None => Some((tr!("settings-dyndns-waiting").to_string(), crate::theme::gray(140))),
        }
    };
    if let Some((text, colour)) = line {
        ui.label(RichText::new(text).size(10.5).color(colour));
    }
}

/// A labelled single-line text field for the network settings tabs.
pub(in crate::app) fn net_row(ui: &mut egui::Ui, label: &str, val: &mut String, w: f32) {
    ui.horizontal(|ui| {
        ui.add_sized([96.0, 22.0], egui::Label::new(label));
        crate::chrome::field(ui, egui::TextEdit::singleline(val).desired_width(w));
    });
}

/// A labelled password field (masked) for the network settings tabs.
pub(in crate::app) fn net_secret(ui: &mut egui::Ui, label: &str, val: &mut String, w: f32) {
    ui.horizontal(|ui| {
        ui.add_sized([96.0, 22.0], egui::Label::new(label));
        crate::chrome::field(ui, egui::TextEdit::singleline(val).password(true).desired_width(w));
    });
}

/// Where the operator's callsign and grid actually live, for the network tabs
/// that report under them but deliberately do not offer a second copy to edit.
pub(in crate::app) fn operator_identity_note(
    ui: &mut egui::Ui,
    digi: &sdroxide_types::DigiConfig,
    seeded: bool,
) {
    net_heading(ui, &tr!("settings-net-operator"));
    if !seeded {
        super::note(ui, &tr!("settings-net-callsign-grid-are-set-general"));
        return;
    }
    let (call, grid) = (digi.my_call.trim(), digi.my_grid.trim());
    if call.is_empty() || grid.is_empty() {
        ui.label(
            RichText::new(tr!("settings-net-set-your-callsign-grid-general"))
                .color(Color32::from_rgb(230, 170, 60)),
        );
    } else {
        super::note(ui, &tr!("settings-net-call-grid-set-on-general", call = call, grid = grid));
    }
}

/// FreeDV Reporter (<https://qso.freedv.org/>): announce this station and show
/// everyone else's as spots.
///
/// `call`/`grid` are the operator identity from the General tab, shown here but
/// not editable: reporting under a second copy would only let the two disagree.
#[allow(clippy::too_many_arguments)]
pub(in crate::app) fn settings_freedv_tab(
    ui: &mut egui::Ui,
    net: &mut sdroxide_types::NetworkConfig,
    call: &str,
    grid: &str,
    digi_seeded: bool,
    status: &Option<String>,
    apply: &mut bool,
) {
    ui.label(
        RichText::new(tr!("settings-net-freedv-reporter"))
            .size(14.0)
            .strong()
            .color(crate::theme::CYAN()),
    );
    ui.add_space(6.0);
    crate::chrome::checkbox(ui, &mut net.freedv_reporter.enabled, tr!("settings-net-enable"))
        .on_hover_text(tr!("settings-net-connects-whenever-enabled-your-station"));
    ui.add_space(6.0);

    let enabled = net.freedv_reporter.enabled;

    ui.add_enabled_ui(enabled, |ui| {
        let c = &mut net.freedv_reporter;

        net_heading(ui, &tr!("settings-net-station"));
        net_row(ui, &tr!("settings-net-message"), &mut c.message, 260.0);
        crate::chrome::checkbox(
            ui,
            &mut c.rx_only,
            tr!("settings-net-receive-only-i-cannot-transmit"),
        );

        net_heading(ui, &tr!("settings-net-server"));
        net_row(ui, &tr!("settings-net-host"), &mut c.host, 220.0);
        ui.horizontal(|ui| {
            ui.add_sized([96.0, 22.0], egui::Label::new(tr!("settings-net-port")));
            ui.add(egui::DragValue::new(&mut c.port).range(1..=65535));
        });
        ui.add_enabled_ui(false, |ui| {
            crate::chrome::checkbox(ui, &mut c.tls, tr!("settings-net-tls-wss"))
                .on_hover_text(tr!("settings-net-not-yet-implemented-freedv-gui"));
        });

        net_heading(ui, &tr!("settings-net-reporting"));
        crate::chrome::checkbox(ui, &mut c.report_rx, tr!("settings-net-report-stations-i-decode"))
            .on_hover_text(tr!("settings-net-sends-rx-report-each-callsign"));
        crate::chrome::checkbox(
            ui,
            &mut c.show_spots,
            tr!("settings-net-show-other-reporter-stations-spots"),
        )
        .on_hover_text(tr!("settings-net-adds-them-panadapter-overlay-world"));
    });

    ui.add_space(8.0);
    if enabled && digi_seeded && (call.is_empty() || grid.is_empty()) {
        ui.label(
            RichText::new(tr!("settings-net-set-your-callsign-grid-general-2"))
                .color(Color32::from_rgb(230, 170, 60)),
        );
    } else if enabled && digi_seeded {
        super::note(
            ui,
            &tr!(
                "settings-net-reporting-as",
                call = call,
                grid = grid,
                version = env!("CARGO_PKG_VERSION")
            ),
        );
        super::note(ui, &tr!("settings-net-callsign-grid-are-set-general"));
    }
    // The status line is shared by every network feed, so only show it here
    // when it is actually ours.
    if let Some(s) = status {
        if s.starts_with("FreeDV Reporter") {
            ui.label(RichText::new(s).weak());
        }
    }

    ui.add_space(8.0);
    if crate::chrome::chip_accent(
        ui,
        false,
        RichText::new(tr!("settings-net-apply")).strong(),
        crate::theme::GREEN(),
        crate::theme::INK_ON_CYAN(),
    )
    .on_hover_text(tr!("settings-net-persist-re-connect"))
    .clicked()
    {
        *apply = true;
    }
}

/// The broadcast-station block on the Spots settings tab: where the schedule
/// came from, and the three things that can be done to it.
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn broadcast_stations_settings(
    ui: &mut egui::Ui,
    reload: &mut bool,
    refetch: &mut bool,
    refetch_source: &mut Option<sdroxide_types::ScheduleSource>,
    import: &mut Option<Result<String, String>>,
    fetching: bool,
    status: Option<&Result<String, String>>,
) {
    let (season, cached) = sdroxide_config::broadcast_schedule_status();
    super::note(
        ui,
        &tr!("settings-net-broadcast-note"),
    );
    ui.horizontal(|ui| {
        ui.label(
            RichText::new(tr!("settings-net-season")).size(11.0).color(crate::theme::gray(150)),
        );
        ui.label(
            RichText::new(season.to_uppercase())
                .monospace()
                .size(11.0)
                .color(crate::theme::TEXT_STRONG()),
        );
        let (text, colour) = if fetching {
            (tr!("settings-net-downloading"), crate::theme::YELLOW())
        } else if cached {
            (tr!("settings-net-downloaded").to_string(), crate::theme::GREEN())
        } else {
            (tr!("settings-net-using-built-copy"), crate::theme::gray(150))
        };
        ui.label(RichText::new(text).size(11.0).color(colour));
    });
    if let Some(Err(e)) = status {
        // Worth showing, not worth alarming over: the built-in schedule is still
        // in use, so the only thing lost is freshness.
        ui.label(
            RichText::new(tr!("settings-net-schedule-download-failed", error = e.to_string()))
                .color(crate::theme::YELLOW())
                .size(11.0),
        );
    }

    ui.add_space(4.0);
    super::note(
        ui,
        &tr!("settings-net-your-own-stations-corrections-go"),
    );
    if let Ok(p) = sdroxide_config::broadcast_stations_path() {
        ui.add(
            egui::Label::new(
                RichText::new(p.display().to_string())
                    .monospace()
                    .size(10.5)
                    .color(crate::theme::gray(150)),
            )
            .truncate(),
        );
    }
    // Which sources are actually here, one line each. **A single "download" button
    // over several sources is a control that cannot be misread**: it reports one
    // count for what are now three downloads, so a source that silently failed
    // looks exactly like one that worked — which is the whole failure this page
    // is for.
    ui.label(RichText::new(tr!("settings-net-schedules")).strong());
    ui.end_row();
    for (source, rows) in sdroxide_config::cached_broadcast_sources() {
        let name = match source {
            sdroxide_types::ScheduleSource::Eibi => "EiBi".to_string(),
            sdroxide_types::ScheduleSource::Aoki => "Aoki".to_string(),
            sdroxide_types::ScheduleSource::Hfcc => "HFCC".to_string(),
            sdroxide_types::ScheduleSource::User => tr!("settings-net-schedule-yours"),
        };
        ui.horizontal(|ui| {
            // **Every source is listed, always.** A row that appeared only once it
            // had downloaded made the one thing this page exists to answer — "did
            // Aoki get downloaded?" — render as *nothing at all*, which is how it
            // went unnoticed the first time.
            let text = match rows {
                Some(n) => tr!(
                    "settings-net-schedule-rows",
                    name = format!("{name:<6}"),
                    n = format!("{n:>6}")
                ),
                None => tr!("settings-net-schedule-not-downloaded", name = format!("{name:<6}")),
            };
            ui.label(RichText::new(text).monospace().size(11.0).color(match rows {
                Some(_) => crate::theme::gray(190),
                None => crate::theme::YELLOW(),
            }));
            let can = !fetching && source != sdroxide_types::ScheduleSource::User;
            if ui
                .add_enabled(can, egui::Button::new(tr!("settings-net-download")))
                .on_hover_text(match source {
                    sdroxide_types::ScheduleSource::Hfcc => {
                        tr!("settings-net-hfcc-hint")
                    }
                    _ => tr!("settings-net-refetch-hint"),
                })
                .clicked()
            {
                *refetch_source = Some(source);
            }
            // **The same file, handed over by hand.** Aoki's is published on a
            // site whose path changes with the season, so the automatic download
            // is the part that breaks — and an operator who has the file already
            // should not be stopped by that. The file is read, checked and cached
            // exactly as a download is, so what happens next is the same code.
            // HFCC has no button: its names live in a package of three files, and
            // a schedule of codes is worse than none.
            if matches!(
                source,
                sdroxide_types::ScheduleSource::Eibi | sdroxide_types::ScheduleSource::Aoki
            ) && ui
                .add_enabled(!fetching, egui::Button::new(tr!("settings-net-from-file")))
                .on_hover_text(match source {
                    sdroxide_types::ScheduleSource::Aoki => {
                        tr!("settings-net-aoki-file-hint")
                    }
                    _ => tr!("settings-net-eibi-file-hint"),
                })
                .clicked()
                && let Some(path) = rfd::FileDialog::new()
                    .add_filter(tr!("settings-net-filter-aoki"), &["zip", "txt"])
                    .add_filter(tr!("settings-net-filter-eibi"), &["csv"])
                    .pick_file()
            {
                *import = Some(match sdroxide_config::import_schedule_file(&path) {
                    Ok(got) => Ok(tr!(
                        "settings-net-schedule-imported",
                        source = got.source.label(),
                        season = got.season,
                        n = got.stations
                    )),
                    Err(e) => Err(e),
                });
                *reload = true;
            }
        });
    }

    ui.horizontal(|ui| {
        if ui
            .button(tr!("settings-net-reload"))
            .on_hover_text(tr!("settings-net-re-read-your-station-file"))
            .clicked()
        {
            *reload = true;
        }
        if ui
            .add_enabled(!fetching, egui::Button::new(tr!("settings-net-download-all")))
            .on_hover_text(
                tr!("settings-net-download-all-hint"),
            )
            .clicked()
        {
            *refetch = true;
        }
    });
    super::note_at(
        ui,
        &tr!("settings-net-own-list-note"),
        Some(11.0),
    );
    ui.horizontal(|ui| {
        if ui
            .add_enabled(!fetching, egui::Button::new(tr!("settings-net-import-list")))
            .on_hover_text(
                tr!("settings-net-import-list-hint"),
            )
            .clicked()
        {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter(tr!("settings-net-filter-station-lists"), &["csv", "txt", "json"])
                .pick_file()
            {
                *import = Some(match sdroxide_config::import_broadcast_stations(&path) {
                    Ok(n) => Ok(tr!("settings-net-stations-imported", n = n)),
                    Err(e) => Err(e),
                });
                *reload = true;
            }
        }
    });
    if let Some(msg) = import {
        let (text, colour) = match &*msg {
            Ok(t) => (t.clone(), crate::theme::GREEN()),
            Err(e) => (format!("⚠ {e}"), crate::theme::YELLOW()),
        };
        ui.label(RichText::new(text).color(colour).size(11.0));
    }
}

/// The browser client reads the schedule compiled into the wasm bundle, so there
/// is nothing to download and no file to overlay.
#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn broadcast_stations_settings(
    ui: &mut egui::Ui,
    _reload: &mut bool,
    _refetch: &mut bool,
    _refetch_source: &mut Option<sdroxide_types::ScheduleSource>,
    _import: &mut Option<Result<String, String>>,
    _fetching: bool,
    _status: Option<&Result<String, String>>,
) {
    super::note(
        ui,
        &tr!("settings-net-broadcast-stations-labelled-waterfall-come"),
    );
}

impl crate::app::SdroxideApp {
    /// One "Test" button and its answer, for a logging service's credentials.
    ///
    /// The point of it: today the first sign that a password is wrong is a QSO
    /// that failed to upload, hours after the contact. This asks the service
    /// directly, before anything depends on the answer.
    ///
    /// ⛔ It sends the EDITED config first. The engine tests what it has
    /// applied, so without that a freshly pasted key would be checked against
    /// the old one, and the operator would be told their new credentials were
    /// wrong when they had simply not been saved yet. Pressing Test therefore
    /// also applies the network settings, which is what someone pressing it
    /// means, and the label says so.
    ///
    /// Nothing is cached between sessions: a green tick from an hour ago says
    /// nothing about the password typed since.
    pub(in crate::app) fn login_test_row(
        &self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<sdroxide_types::Command>,
        edited: &sdroxide_types::NetworkConfig,
        target: sdroxide_types::LoginTarget,
    ) {
        ui.horizontal(|ui| {
            let pending = self.login_tests_pending.contains(&target);
            let btn = ui.add_enabled(
                !pending,
                egui::Button::new(
                    RichText::new(tr!("settings-net-test-service", service = target.label()))
                        .size(11.0),
                ),
            );
            if btn
                .on_hover_text(tr!("settings-net-ask-service-whether-these-credentials"))
                .clicked()
            {
                cmds.push(sdroxide_types::Command::SetNetworkConfig(edited.clone()));
                cmds.push(sdroxide_types::Command::TestLogin(target));
            }
            if pending {
                super::note_at(ui, &tr!("settings-net-checking"), Some(11.0));
            } else if let Some(r) = self.login_tests.get(&target) {
                let (mark, colour) = if r.ok {
                    ("✔", crate::theme::GREEN())
                } else {
                    ("✖", Color32::from_rgb(255, 120, 120))
                };
                ui.label(RichText::new(format!("{mark} {}", r.message)).size(11.0).color(colour));
            }
        });
    }
}
