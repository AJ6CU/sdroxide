//! The Scanner window: what to scan, how loud a signal has to be to stop it,
//! and what to do once it has stopped.

use crate::i18n::{Tr, tr};
use eframe::egui::{self, RichText};
use sdroxide_types::{
    Command, Mode, SCAN_STEPS_HZ, SQUELCH_OPEN_DB, ScanKind, ScanResume, ScannerConfig,
};

use crate::app::SdroxideApp;
use crate::chrome::StyledCombo;

/// What the scan dwell becomes when the VOICE gate is switched on or off.
///
/// On: a dwell too short to hear a syllable is raised to the floor the gate
/// needs, and what was there is remembered. Off: the remembered figure comes
/// back — but only while the field still holds the raised one, because a dwell
/// the operator chose *while* the gate was on is theirs and must not be
/// overwritten. Either way the memory is cleared, so the next press cannot
/// restore a figure from two toggles ago.
///
/// A pure function so the rule can be tested without a window (issue #254).
fn voice_dwell(on: bool, dwell: u32, prev: &mut Option<u32>) -> u32 {
    if on {
        if dwell < sdroxide_types::MIN_LISTEN_MS {
            *prev = Some(dwell);
            return sdroxide_types::MIN_LISTEN_MS;
        }
        dwell
    } else {
        match prev.take() {
            Some(was) if dwell == sdroxide_types::MIN_LISTEN_MS => was,
            _ => dwell,
        }
    }
}

impl SdroxideApp {
    pub(in crate::app) fn scanner_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        if !self.show_scanner {
            return;
        }
        // Edited in place and sent whole on any change, the way the skimmer's
        // settings are; the engine persists it and echoes it back.
        let mut cfg = self.scanner.clone();
        let show = self.tool_window(ctx, "scanner", &tr!("scanner-scanner"), [420.0, 520.0], true, |me, ui| {
            me.scanner_body(ui, &mut cfg, cmds);
        });
        if cfg != self.scanner {
            self.scanner = cfg.clone();
            cmds.push(Command::SetScannerConfig(cfg));
        }
        self.show_scanner = show;
    }

    fn scanner_body(&mut self, ui: &mut egui::Ui, cfg: &mut ScannerConfig, cmds: &mut Vec<Command>) {
        let scan = self.state.scan;

        // What to scan.
        ui.horizontal(|ui| {
            for kind in ScanKind::ALL {
                if crate::chrome::chip(ui, cfg.kind == kind, kind.tr())
                    .on_hover_text(match kind {
                        ScanKind::Memories => tr!("scanner-work-through-stored-memory-channels"),
                        ScanKind::Range => tr!("scanner-work-through-slice-band-channel"),
                    })
                    .clicked()
                {
                    cfg.kind = kind;
                }
            }
            crate::chrome::row_tail(ui, |ui| {
                let label = if scan.running { tr!("scanner-stop") } else { tr!("scanner-start") };
                if crate::chrome::chip_accent(
                    ui,
                    scan.running,
                    label,
                    crate::theme::GREEN(),
                    crate::theme::INK_ON_BRIGHT(),
                )
                .clicked()
                {
                    cmds.push(Command::SetScanning(!scan.running));
                }
            });
        });
        ui.separator();

        match cfg.kind {
            ScanKind::Range => self.scanner_range(ui, cfg),
            ScanKind::Memories => self.scanner_memories(ui, cfg),
        }

        ui.separator();
        self.scanner_thresholds(ui, cfg);
        ui.separator();
        self.scanner_status(ui, cmds);
    }

    fn scanner_range(&self, ui: &mut egui::Ui, cfg: &mut ScannerConfig) {
        // Broadcast bands as one-click ranges: a listener walks 49 m, not a
        // pair of typed frequencies. The metre table is the schedule's, so the
        // band named here is the band named there. LW and MW lead it.
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(tr!("scanner-broadcast-band")).weak());
            let mut preset = |ui: &mut egui::Ui, name: &str, lo: f64, hi: f64| {
                let on = (cfg.range_lo_hz - lo).abs() < 1.0 && (cfg.range_hi_hz - hi).abs() < 1.0;
                if crate::chrome::chip(ui, on, name)
                    .on_hover_text(tr!("scanner-scan-band-hover", name = name, lo = format!("{:.0}", lo / 1e3), hi = format!("{:.0}", hi / 1e3)))
                    .clicked()
                {
                    cfg.range_lo_hz = lo;
                    cfg.range_hi_hz = hi;
                    cfg.mode = Mode::Am;
                }
            };
            preset(ui, "LW", 148_500.0, 283_500.0);
            preset(ui, "MW", 526_500.0, 1_606_500.0);
            for &(name, lo, hi) in sdroxide_types::broadcast::METRE_BANDS {
                preset(ui, name, lo * 1e3, hi * 1e3);
            }
        });
        ui.add_space(4.0);
        egui::Grid::new("scan-range").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| {
            ui.label(tr!("scanner-from"));
            ui.horizontal(|ui| {
                mhz_edit(ui, &mut cfg.range_lo_hz);
                ui.label(tr!("scanner-range-to"));
                mhz_edit(ui, &mut cfg.range_hi_hz);
                ui.label(RichText::new("MHz").weak());
            });
            ui.end_row();

            ui.label(tr!("scanner-step"));
            ui.horizontal(|ui| {
                for step in SCAN_STEPS_HZ {
                    let label = if step >= 10_000.0 {
                        format!("{:.0}k", step / 1000.0)
                    } else {
                        format!("{:.2}k", step / 1000.0)
                    };
                    if crate::chrome::chip(ui, (cfg.step_hz - step).abs() < 1.0, label).clicked() {
                        cfg.step_hz = step;
                    }
                }
            });
            ui.end_row();

            ui.label(tr!("scanner-mode"));
            egui::ComboBox::from_id_salt("scan-mode").selected_text(cfg.mode.label()).show_styled(
                ui,
                |ui| {
                    // The modes anyone scans in. A range scan sets one mode for
                    // the whole range; a memory scan takes each channel's own.
                    for m in [Mode::Nfm, Mode::Am, Mode::Wfm, Mode::Usb, Mode::Lsb] {
                        ui.selectable_value(&mut cfg.mode, m, m.label());
                    }
                },
            );
            ui.end_row();
        });
        if !cfg.range_is_usable() {
            ui.label(
                RichText::new(tr!("scanner-range-empty-high-edge-has"))
                    .color(crate::theme::ALERT()),
            );
        }
        // Done after the edits above, so retuning the range takes its skipped
        // channels off the screen at once rather than a round trip later.
        cfg.forget_stale_skips();
        self.scanner_range_skips(ui, cfg);
    }

    /// The channels a range scan has been told to pass over, and the way back.
    ///
    /// Shown rather than merely remembered: a scanner that silently refuses to
    /// stop somewhere is indistinguishable from a scanner that cannot hear it,
    /// and the operator who pressed SKIP three passes ago is exactly the person
    /// who will wonder why the repeater never comes up.
    fn scanner_range_skips(&self, ui: &mut egui::Ui, cfg: &mut ScannerConfig) {
        if cfg.skip_freq_hz.is_empty() {
            ui.label(RichText::new(tr!("scanner-skip-while-holding-pass-over")).size(10.0).weak());
            return;
        }
        let listed: Vec<f64> = cfg.skip_freq_hz.clone();
        let mut drop_at: Option<usize> = None;
        let mut clear = false;
        ui.horizontal(|ui| {
            ui.label(RichText::new(tr!("scanner-skipping")).size(10.5).weak());
            if crate::chrome::chip(ui, false, tr!("scanner-clear"))
                .on_hover_text(tr!("scanner-stop-passing-over-any-them"))
                .clicked()
            {
                clear = true;
            }
        });
        // Bounded: an evening spent dismissing a busy band's pagers and data
        // channels runs to dozens of them, and they must not push the rest of
        // the window off the bottom.
        egui::ScrollArea::vertical().id_salt("scan-skips").max_height(72.0).show(ui, |ui| {
            ui.horizontal_wrapped(|ui| {
                for (i, f) in listed.iter().enumerate() {
                    if crate::chrome::chip(ui, true, format!("{:.4}", f / 1e6))
                        .on_hover_text(tr!("scanner-stop-passing-over-channel"))
                        .clicked()
                    {
                        drop_at = Some(i);
                    }
                }
            });
        });
        if clear {
            cfg.skip_freq_hz.clear();
        } else if let Some(i) = drop_at {
            cfg.skip_freq_hz.remove(i);
        }
    }

    fn scanner_memories(&self, ui: &mut egui::Ui, cfg: &mut ScannerConfig) {
        if self.memories.is_empty() {
            ui.label(RichText::new(tr!("scanner-no-memory-channels-stored-yet")).weak());
            return;
        }
        self.scanner_folders(ui, cfg);
        self.scanner_fast(ui, cfg);
        ui.label(RichText::new(tr!("scanner-skip-channel-pass-over")).size(10.0).weak());
        // Resolved before the loop: the rows borrow `cfg` mutably to toggle a
        // skip, so the filter cannot still be holding it.
        let listed: Vec<&sdroxide_types::MemoryChannel> =
            self.memories.iter().filter(|m| cfg.scans_folder(self.filed_under(m))).collect();
        egui::ScrollArea::vertical().max_height(180.0).show(ui, |ui| {
            egui::Grid::new("scan-mems").num_columns(3).spacing([8.0, 4.0]).show(ui, |ui| {
                for m in listed {
                    let skipped = cfg.skip.contains(&m.id);
                    if crate::chrome::chip(ui, skipped, tr!("scanner-skip")).clicked() {
                        if skipped {
                            cfg.skip.retain(|&id| id != m.id);
                        } else {
                            cfg.skip.push(m.id);
                        }
                    }
                    let name =
                        if m.name.is_empty() { format!("#{}", m.id) } else { m.name.clone() };
                    let text = RichText::new(name);
                    ui.label(if skipped { text.weak() } else { text });
                    ui.label(
                        RichText::new(format!(
                            "{e6:.6} MHz {mode}",
                            e6 = m.freq_hz / 1e6,
                            mode = m.mode.label()
                        ))
                        .weak(),
                    );
                    ui.end_row();
                }
            });
        });
    }

    /// The FAST switch: read the channels off the wideband spectrum instead of
    /// visiting each one (issue #228).
    ///
    /// Greyed rather than hidden on a front end that has no span to search — a
    /// CAT rig on a sound card — for the reason every greyed control here is:
    /// a row that comes and goes with the radio is a row nobody can find
    /// twice, and what it says is still true, it just cannot be had *here*.
    fn scanner_fast(&self, ui: &mut egui::Ui, cfg: &mut ScannerConfig) {
        // The same test the engine makes: a demod-audio front end delivers no
        // spectrum to read the channels off.
        let can_sweep = !self.caps.as_ref().is_some_and(|c| c.audio_mode);
        ui.horizontal_wrapped(|ui| {
            let hint = if can_sweep {
                tr!("scanner-look-all-channels-fall-inside")
            } else {
                tr!("scanner-radio-hands-over-demodulated-audio")
            };
            let resp = ui.add_enabled_ui(can_sweep, |ui| {
                crate::chrome::chip(ui, cfg.mem_fast && can_sweep, tr!("scanner-fast"))
            });
            if resp.inner.on_hover_text(hint).clicked() {
                cfg.mem_fast = !cfg.mem_fast;
            }
            ui.label(
                RichText::new(tr!("scanner-read-list-off-spectrum-instead")).size(10.0).weak(),
            );
        });
    }

    /// Which folder a memory reads as being in — `None` for the top level, and
    /// for one whose folder has gone from under it, exactly as the memory
    /// window lists it and as the engine's own scan filter resolves it.
    fn filed_under(&self, m: &sdroxide_types::MemoryChannel) -> Option<u32> {
        m.folder.filter(|id| self.mem_folders.iter().any(|f| f.id == *id))
    }

    /// Which folders the memory scan runs over: **ALL**, then one chip per
    /// folder, then the unfiled channels (issue #236).
    ///
    /// Drawn only where there is a folder to choose — with everything at the
    /// top level there is one place for a channel to be, and a row offering it
    /// is a control that can only be set wrong.
    ///
    /// ALL is not "every chip lit": it is an empty selection, which is also
    /// what a folder made tomorrow falls into. Lighting every folder instead
    /// would quietly leave the next one out of the scan.
    fn scanner_folders(&self, ui: &mut egui::Ui, cfg: &mut ScannerConfig) {
        if self.mem_folders.is_empty() {
            return;
        }
        // The counts are what makes the row readable: a folder with nothing in
        // it, or one whose every channel is skipped, is worth seeing before the
        // scan says it has nothing to visit.
        let count = |folder: Option<u32>| {
            self.memories.iter().filter(|m| self.filed_under(m) == folder).count()
        };
        let unfiled = count(None);
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(tr!("scanner-folders")).size(10.5).weak());
            if crate::chrome::chip(ui, cfg.folders.is_empty(), tr!("scanner-all"))
                .on_hover_text(tr!("scanner-scan-every-folder-every-folder"))
                .clicked()
            {
                cfg.folders.clear();
            }
            let mut toggle = |ui: &mut egui::Ui, which: Option<u32>, label: String, n: usize| {
                let on = cfg.folders.contains(&which);
                let text = RichText::new(format!("{label} ({n})")).size(11.0);
                if crate::chrome::chip(ui, on, text).clicked() {
                    if on {
                        cfg.folders.retain(|f| *f != which);
                    } else {
                        cfg.folders.push(which);
                    }
                }
            };
            for f in &self.mem_folders {
                toggle(ui, Some(f.id), f.name.clone(), count(Some(f.id)));
            }
            if unfiled > 0 {
                toggle(ui, None, tr!("scanner-unfiled"), unfiled);
            }
        });
        if !cfg.folders.is_empty()
            && !self.memories.iter().any(|m| cfg.scans_folder(self.filed_under(m)))
        {
            ui.label(
                RichText::new(tr!("scanner-nothing-filed-under-folders-you"))
                    .color(crate::theme::ALERT()),
            );
        }
    }

    /// The VOICE switch: stop only where somebody is talking, rather than on
    /// every carrier, data burst and birdie that reaches the threshold.
    ///
    /// Greyed — with the reason named — where it could not keep that promise: a
    /// scan whose modes carry no speech would run and stop nowhere. That is the
    /// same test the engine makes when the scan starts, so the operator sees the
    /// answer here rather than as a notice after pressing START.
    fn scanner_voice(&mut self, ui: &mut egui::Ui, cfg: &mut ScannerConfig) {
        ui.horizontal_wrapped(|ui| {
            // A range scan has one mode for the whole range; a memory scan has
            // one per channel, so the question is whether *any* of the channels
            // it would visit could hold a voice.
            let any_speech = match cfg.kind {
                ScanKind::Range => cfg.mode.carries_speech(),
                ScanKind::Memories => self
                    .memories
                    .iter()
                    .filter(|m| cfg.scans_folder(self.filed_under(m)))
                    .any(|m| m.mode.carries_speech()),
            };
            let hint = if any_speech {
                tr!("scanner-stop-only-on-channels-that-sound")
            } else {
                tr!("scanner-nothing-in-this-scan-is-in-a-mode")
            };
            let resp = ui.add_enabled_ui(any_speech, |ui| {
                crate::chrome::chip(ui, cfg.voice_only && any_speech, tr!("scanner-voice"))
            });
            if resp.inner.on_hover_text(hint).clicked() && any_speech {
                cfg.voice_only = !cfg.voice_only;
                // **A dwell set by the voice check is given back when it is
                // switched off** (issue #254). Turning the gate on raises the
                // dwell to the floor it needs — visibly, because the field must
                // not say one thing and the engine do another — and leaving it
                // raised afterwards silently changes the scan's speed for good.
                // The raised figure is remembered here, and restored only while
                // the field still holds it: if the operator has since chosen a
                // dwell of their own, that is theirs to keep.
                cfg.dwell_ms =
                    voice_dwell(cfg.voice_only, cfg.dwell_ms, &mut self.scan_voice_prev_dwell);
            }
            ui.label(
                RichText::new(if any_speech {
                    tr!("scanner-stop-only-where-somebody-is")
                } else {
                    tr!("scanner-no-voice-to-look-for-in-these")
                })
                .size(10.0)
                .weak(),
            );
        });
    }

    fn scanner_thresholds(&mut self, ui: &mut egui::Ui, cfg: &mut ScannerConfig) {
        egui::Grid::new("scan-levels").num_columns(2).spacing([10.0, 6.0]).show(ui, |ui| {
            ui.label(tr!("scanner-stops"));
            ui.horizontal(|ui| {
                if crate::chrome::chip(ui, cfg.follow_squelch, tr!("scanner-sql"))
                    .on_hover_text(tr!("scanner-use-receiver-s-own-squelch"))
                    .clicked()
                {
                    cfg.follow_squelch = !cfg.follow_squelch;
                }
                if cfg.follow_squelch {
                    let sql = self.state.rx[0].squelch_db;
                    ui.label(
                        RichText::new(if sql <= SQUELCH_OPEN_DB + 1.0 {
                            tr!("scanner-squelch-off-scan-will-stop")
                        } else {
                            format!("{sql:.0} dBFS", sql = sql)
                        })
                        .weak(),
                    );
                } else {
                    ui.add(
                        egui::DragValue::new(&mut cfg.threshold_db)
                            .speed(0.5)
                            .range(-140.0..=-10.0)
                            .suffix(" dBFS"),
                    )
                    .on_hover_text(tr!("scanner-channel-power-signal-has-reach"));
                }
            });
            ui.end_row();

            ui.label(tr!("bandplan-kind-phone"));
            self.scanner_voice(ui, cfg);
            ui.end_row();
            ui.label(tr!("scanner-listens"));
            ui.add(
                egui::DragValue::new(&mut cfg.dwell_ms)
                    .speed(5.0)
                    .range(if cfg.voice_only { sdroxide_types::MIN_LISTEN_MS } else { 40 }..=2000)
                    .suffix(" ms"),
            )
            .on_hover_text(if cfg.voice_only {
                tr!("scanner-how-long-to-stay-on-a-candidate")
            } else {
                tr!("scanner-how-long-stay-candidate-before")
            });
            ui.end_row();

            ui.label(tr!("scanner-resumes"));
            ui.horizontal(|ui| {
                for r in ScanResume::ALL {
                    if crate::chrome::chip(ui, cfg.resume == r, r.tr())
                        .on_hover_text(match r {
                            ScanResume::Carrier => tr!("scanner-carry-once-signal-drops"),
                            ScanResume::Timed => tr!("scanner-carry-after-fixed-time-regardless"),
                            ScanResume::Manual => tr!("scanner-stay-until-you-press-next"),
                        })
                        .clicked()
                    {
                        cfg.resume = r;
                    }
                }
                if cfg.resume != ScanResume::Manual {
                    ui.add(
                        egui::DragValue::new(&mut cfg.resume_ms)
                            .speed(50.0)
                            .range(200..=60_000)
                            .suffix(" ms"),
                    )
                    .on_hover_text(match cfg.resume {
                        ScanResume::Timed => tr!("scanner-how-long-stay"),
                        // Long enough to ride out the gap between overs, or the
                        // scan leaves in the middle of a conversation.
                        _ => tr!("scanner-how-long-wait-after-signal"),
                    });
                }
            });
            ui.end_row();
        });
    }

    fn scanner_status(&self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        let scan = self.state.scan;
        ui.horizontal(|ui| {
            if !scan.running {
                ui.label(RichText::new(tr!("scanner-stopped")).weak());
                return;
            }
            let here = self.state.active_freq_hz() / 1e6;
            let (text, colour) = if scan.holding {
                (tr!("scanner-holding-mhz", here = format!("{:.6}", here)), crate::theme::GREEN())
            } else {
                (tr!("scanner-scanning-mhz", here = format!("{:.6}", here)), crate::theme::CYAN())
            };
            ui.label(RichText::new(text).color(colour).strong());
            // Name what it stopped on, from the broadcast table, so a listener
            // does not have to know 6.185 MHz is Taiwan.
            if scan.holding
                && let Some(st) = sdroxide_types::broadcast::at_dial(
                    &self.broadcast,
                    self.state.active_freq_hz(),
                    crate::time::now_unix(),
                )
            {
                ui.label(
                    RichText::new(format!("· {}", st.name)).color(crate::theme::TEXT()).strong(),
                );
            }
            crate::chrome::row_tail(ui, |ui| {
                if crate::chrome::chip(ui, false, tr!("scanner-skip"))
                    .on_hover_text(tr!("scanner-move-don-t-stop-channel"))
                    .clicked()
                {
                    cmds.push(Command::ScanSkip);
                }
                if crate::chrome::chip(ui, false, tr!("scanner-next"))
                    .on_hover_text(tr!("scanner-move-now"))
                    .clicked()
                {
                    cmds.push(Command::ScanNext);
                }
            });
        });
        if scan.running && self.scanner.voice_only {
            self.scanner_voice_status(ui, scan);
        }
    }

    /// What the speech detector made of the channel, when the voice gate is on.
    ///
    /// The point of the line is the channel it *passed*: a scan that declines to
    /// stop somewhere is otherwise indistinguishable from one that cannot hear
    /// it, and the operator watching a strong signal go by deserves the number
    /// that explains it.
    fn scanner_voice_status(&self, ui: &mut egui::Ui, scan: sdroxide_types::ScanState) {
        if !self.state.rx[0].mode.carries_speech() {
            ui.label(
                RichText::new(tr!("scanner-mode-no-speech", mode = self.state.rx[0].mode.label()))
                .size(10.5)
                .weak(),
            );
            return;
        }
        let Some(v) = scan.voice else {
            ui.label(RichText::new(tr!("scanner-listening-for-a-voice")).size(10.5).weak());
            return;
        };
        let (text, colour) = if scan.holding {
            (tr!("scanner-voice-holding", score = v.score), crate::theme::GREEN())
        } else {
            (
                tr!("scanner-voice-passed", mhz = format!("{:.4}", v.freq_hz as f64 / 1e6), score = v.score),
                crate::theme::TEXT(),
            )
        };
        ui.label(RichText::new(text).size(10.5).color(colour));
    }
}

/// A frequency box in MHz, which is how an operator says one.
fn mhz_edit(ui: &mut egui::Ui, hz: &mut f64) {
    let mut mhz = *hz / 1e6;
    if ui
        .add(
            egui::DragValue::new(&mut mhz)
                .speed(0.01)
                .range(0.0..=6000.0)
                .max_decimals(6)
                .fixed_decimals(4),
        )
        .changed()
    {
        *hz = mhz * 1e6;
    }
}

#[cfg(test)]
mod tests {
    use super::voice_dwell;

    /// **The dwell the voice gate raised is given back when it is switched off**
    /// (issue #254), and a dwell chosen while the gate was on is not stolen.
    #[test]
    fn the_voice_gate_gives_the_dwell_back() {
        let mut prev = None;
        // On, from a dwell too short for a syllable: raised and remembered.
        assert_eq!(voice_dwell(true, 100, &mut prev), 400);
        assert_eq!(prev, Some(100));
        // Off: the remembered figure returns, and the memory clears.
        assert_eq!(voice_dwell(false, 400, &mut prev), 100);
        assert_eq!(prev, None);

        // A dwell already long enough is left alone and nothing is remembered.
        let mut prev = None;
        assert_eq!(voice_dwell(true, 800, &mut prev), 800);
        assert_eq!(prev, None);

        // The operator picks a dwell while the gate is on: switching it off
        // keeps their figure rather than restoring the old one.
        let mut prev = None;
        assert_eq!(voice_dwell(true, 100, &mut prev), 400);
        assert_eq!(voice_dwell(false, 900, &mut prev), 900);
        assert_eq!(prev, None, "a stale figure would be restored on the next off");

        // Off with nothing remembered is a no-op.
        let mut prev = None;
        assert_eq!(voice_dwell(false, 400, &mut prev), 400);
    }
}
