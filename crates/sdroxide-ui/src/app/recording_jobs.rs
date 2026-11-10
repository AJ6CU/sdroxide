//! Scheduled recordings: the jobs list, and the clock that starts and stops
//! them.
//!
//! A job is a start time, a frequency, a duration and what to capture (see
//! [`sdroxide_types::RecordingJob`]). This window is where they are edited; the
//! scheduler that acts on them runs once a frame and drives the engine's
//! existing recording commands.
//!
//! The scheduler does not fight the operator: if a recording is already running
//! when a job comes due, the job is reported and skipped rather than retuning
//! the radio out from under whoever is listening.

use eframe::egui::{self, RichText};
use sdroxide_types::{Command, JobAction, Mode, RecordingJob, RecordingKind, RxId, Vfo};

use crate::app::SdroxideApp;
use crate::i18n::tr;
use crate::app::persist::persist_recording_jobs;
use crate::app::util::{date_str, parse_utc, time_str};
use crate::time::now_unix;

/// The modes a listener is likely to schedule, as in the reception log.
const MODES: [Mode; 9] = [
    Mode::Am,
    Mode::Sam,
    Mode::Cquam,
    Mode::Lsb,
    Mode::Usb,
    Mode::Cw,
    Mode::Nfm,
    Mode::Wfm,
    Mode::Drm,
];

pub(in crate::app) struct JobsUi {
    pub show: bool,
    /// The job being edited, with its date/time kept as text.
    pub edit: Option<RecordingJob>,
    date: String,
    time: String,
    /// What the scheduler last did, shown in the window.
    pub status: Option<String>,
    /// The job whose recording is currently running, if the scheduler started
    /// it.
    pub running: Option<u64>,
}

impl Default for JobsUi {
    fn default() -> Self {
        JobsUi {
            show: false,
            edit: None,
            date: String::new(),
            time: String::new(),
            status: None,
            running: None,
        }
    }
}

impl SdroxideApp {
    /// Start and stop scheduled recordings. Runs once a frame.
    pub(in crate::app) fn poll_recording_jobs(&mut self, cmds: &mut Vec<Command>) {
        let now = now_unix().max(0) as u64;
        let running = self.jobs.running;

        // Decide first, act after: the actions borrow the job list, and the
        // application reads the engine state beside it.
        let mut start: Option<usize> = None;
        let mut stop: Option<usize> = None;
        for (i, j) in self.recording_jobs.iter().enumerate() {
            match j.action(now, running == Some(j.id)) {
                JobAction::Start => start = Some(i),
                JobAction::Stop => stop = Some(i),
                JobAction::Idle => {}
            }
        }

        if let Some(i) = stop {
            let j = &self.recording_jobs[i];
            if matches!(j.kind, RecordingKind::Audio | RecordingKind::Both) {
                cmds.push(Command::SetRecording(false));
            }
            if matches!(j.kind, RecordingKind::Iq | RecordingKind::Both) {
                cmds.push(Command::SetIqRecording(false));
            }
            let name = j.name.clone();
            self.recording_jobs[i].done = true;
            self.jobs.running = None;
            self.jobs.status = Some(tr!("recjobs-finished", name = name));
            persist_recording_jobs(&self.recording_jobs);
        }

        if let Some(i) = start {
            // Do not retune or overwrite a recording the operator started.
            if self.state.recording || self.state.iq_recording {
                let name = self.recording_jobs[i].name.clone();
                self.recording_jobs[i].done = true;
                self.jobs.status = Some(tr!("recjobs-skipped", name = name));
                persist_recording_jobs(&self.recording_jobs);
                return;
            }
            let j = self.recording_jobs[i].clone();
            cmds.push(Command::SetVfo { vfo: Vfo::A, hz: j.freq_hz });
            cmds.push(Command::SetMode { rx: RxId::Main, mode: j.mode });
            if matches!(j.kind, RecordingKind::Audio | RecordingKind::Both) {
                cmds.push(Command::SetRecording(true));
            }
            if matches!(j.kind, RecordingKind::Iq | RecordingKind::Both) {
                cmds.push(Command::SetIqRecording(true));
            }
            self.jobs.running = Some(j.id);
            let name = if j.name.is_empty() { j.utc_text() } else { j.name };
            self.jobs.status = Some(tr!("recjobs-recording-name", name = name));
        }
    }

    /// The RECORDINGS window: the jobs list and its form. The body carries its
    /// asks back and the tail acts on them, after the window has closed the
    /// borrow of the jobs list.
    pub(in crate::app) fn recordings_window(
        &mut self,
        ctx: &egui::Context,
        cmds: &mut Vec<Command>,
    ) {
        let mut new_job = false;
        let mut save = false;
        let mut cancel = false;
        let mut edit: Option<u64> = None;
        let mut delete: Option<u64> = None;
        let open = self.tool_window(
            ctx,
            "recordings",
            &tr!("recjobs-title"),
            [720.0, 480.0],
            self.jobs.show,
            |me, ui| {
                me.recordings_body(ui, &mut new_job, &mut save, &mut cancel, &mut edit, &mut delete)
            },
        );
        self.jobs.show = open;

        if new_job {
            let now = now_unix().max(0);
            let mut j = RecordingJob::default();
            j.freq_hz = self.on_air_freq_hz();
            j.mode = self.state.rx[0].mode;
            j.at_unix = now as u64;
            self.jobs.date = date_str(now);
            self.jobs.time = time_str(now);
            self.jobs.edit = Some(j);
        }
        // Load the picked job into the form. This was the local the old window
        // set and never read, so the **edit** button did nothing at all.
        if let Some(id) = edit {
            if let Some(j) = self.recording_jobs.iter().find(|x| x.id == id) {
                self.jobs.date = date_str(j.at_unix as i64);
                self.jobs.time = time_str(j.at_unix as i64);
                self.jobs.edit = Some(j.clone());
            }
        }
        if save {
            if let Some(mut j) = self.jobs.edit.take() {
                j.at_unix = parse_utc(&self.jobs.date, &self.jobs.time, 0).max(0) as u64;
                if j.id == 0 {
                    j.id = self.recording_jobs.iter().map(|x| x.id).max().unwrap_or(0) + 1;
                    self.recording_jobs.push(j);
                } else if let Some(slot) = self.recording_jobs.iter_mut().find(|x| x.id == j.id) {
                    *slot = j;
                }
                persist_recording_jobs(&self.recording_jobs);
            }
        }
        if cancel {
            self.jobs.edit = None;
        }
        if let Some(id) = delete {
            self.recording_jobs.retain(|x| x.id != id);
            persist_recording_jobs(&self.recording_jobs);
        }
        let _ = cmds;
    }

    /// The recordings list's body, split out so the shell can draw it in a
    /// window of its own.
    fn recordings_body(
        &mut self,
        ui: &mut egui::Ui,
        new_job: &mut bool,
        save: &mut bool,
        cancel: &mut bool,
        edit: &mut Option<u64>,
        delete: &mut Option<u64>,
    ) {
        crate::chrome::window_body_bg(ui);
        let rows = self.recording_jobs.clone();
        ui.horizontal(|ui| {
            if crate::chrome::chip(ui, false, tr!("recjobs-new-job")).clicked() {
                *new_job = true;
            }
            if let Some(s) = &self.jobs.status {
                ui.label(RichText::new(s).size(11.0).color(crate::theme::CYAN()));
            }
            if self.jobs.running.is_some() {
                ui.label(RichText::new(tr!("recjobs-recording")).size(11.0).color(crate::theme::ALERT()));
            }
        });
        if self.jobs.edit.is_some() {
            ui.add_space(4.0);
            self.job_form(ui, save, cancel);
        }
        ui.separator();
        egui::ScrollArea::vertical().auto_shrink([false, false]).id_salt("jobs-list").show(
            ui,
            |ui| {
                if rows.is_empty() {
                    ui.label(
                        RichText::new(tr!("recjobs-none"))
                            .color(crate::theme::gray(150)),
                    );
                }
                for j in &rows {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new(j.utc_text()).monospace().size(11.0));
                        ui.label(
                            RichText::new(format!("{:>6} kHz", j.freq_hz.round() as i64))
                                .monospace(),
                        );
                        ui.label(RichText::new(j.mode.label()).size(11.0));
                        ui.label(RichText::new(truncate(&j.name, 26)).strong());
                        ui.label(
                            RichText::new(format!("{} min", j.duration_s / 60))
                                .size(11.0)
                                .color(crate::theme::gray(160)),
                        );
                        ui.label(
                            RichText::new(kind_label(j.kind)).size(11.0).color(crate::theme::gray(160)),
                        );
                        if j.done {
                            ui.label(
                                RichText::new(tr!("recjobs-done")).size(10.5).color(crate::theme::gray(130)),
                            );
                        }
                        if ui.small_button(tr!("recjobs-edit")).clicked() {
                            *edit = Some(j.id);
                        }
                        if ui.small_button(tr!("recjobs-del")).clicked() {
                            *delete = Some(j.id);
                        }
                    });
                }
            },
        );
    }

    fn job_form(&mut self, ui: &mut egui::Ui, save: &mut bool, cancel: &mut bool) {
        let Some(j) = self.jobs.edit.as_mut() else { return };
        egui::Frame::new()
            .fill(crate::theme::ROW_BG())
            .stroke(egui::Stroke::new(1.0, crate::theme::RED_DEEP()))
            .inner_margin(egui::Margin::same(9))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                egui::Grid::new("job-form").num_columns(4).spacing([10.0, 6.0]).show(ui, |ui| {
                    ui.label(tr!("recjobs-station"));
                    crate::chrome::field(
                        ui,
                        egui::TextEdit::singleline(&mut j.name)
                            .desired_width(220.0)
                            .hint_text(tr!("recjobs-station-hint")),
                    );
                    ui.label(tr!("recjobs-kind"));
                    egui::ComboBox::from_id_salt("job-kind")
                        .width(90.0)
                        .selected_text(kind_label(j.kind))
                        .show_ui(ui, |ui| {
                            for k in [RecordingKind::Audio, RecordingKind::Iq, RecordingKind::Both]
                            {
                                ui.selectable_value(&mut j.kind, k, kind_label(k));
                            }
                        });
                    ui.end_row();

                    ui.label(tr!("recjobs-start-utc"));
                    ui.horizontal(|ui| {
                        crate::chrome::field(
                            ui,
                            egui::TextEdit::singleline(&mut self.jobs.date).desired_width(96.0),
                        );
                        crate::chrome::field(
                            ui,
                            egui::TextEdit::singleline(&mut self.jobs.time).desired_width(64.0),
                        );
                    });
                    ui.label(tr!("recjobs-for"));
                    ui.horizontal(|ui| {
                        let mut minutes = (j.duration_s / 60) as i32;
                        if ui
                            .add(egui::DragValue::new(&mut minutes).speed(1.0).range(1..=720))
                            .changed()
                        {
                            j.duration_s = (minutes as u32) * 60;
                        }
                        ui.label("min");
                    });
                    ui.end_row();

                    ui.label(tr!("settings-frequency"));
                    ui.horizontal(|ui| {
                        let mut khz = (j.freq_hz / 1e3).round() as i64;
                        if ui
                            .add(egui::DragValue::new(&mut khz).speed(1.0).range(0..=1_000_000))
                            .changed()
                        {
                            j.freq_hz = khz as f64 * 1e3;
                        }
                        ui.label("kHz");
                        egui::ComboBox::from_id_salt("job-mode")
                            .width(84.0)
                            .selected_text(j.mode.label())
                            .show_ui(ui, |ui| {
                                for m in MODES {
                                    ui.selectable_value(&mut j.mode, m, m.label());
                                }
                            });
                    });
                    ui.label("");
                    ui.label("");
                    ui.end_row();
                });
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    if crate::chrome::chip(ui, true, tr!("recjobs-save")).clicked() {
                        *save = true;
                    }
                    if crate::chrome::chip(ui, false, tr!("logbook-cancel")).clicked() {
                        *cancel = true;
                    }
                });
            });
    }
}

fn kind_label(k: RecordingKind) -> String {
    match k {
        RecordingKind::Audio => tr!("recjobs-kind-audio"),
        RecordingKind::Iq => tr!("recjobs-kind-iq"),
        RecordingKind::Both => tr!("recjobs-kind-both"),
    }
}

fn truncate(s: &str, n: usize) -> String {
    if s.chars().count() <= n {
        s.to_string()
    } else {
        let mut out: String = s.chars().take(n.saturating_sub(1)).collect();
        out.push('…');
        out
    }
}
