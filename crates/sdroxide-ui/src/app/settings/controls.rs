//! The Controls tab: keyboard, mouse and MIDI bindings.
//!
//! A rebind has no APPLY step — it takes effect on the next frame and is
//! written straight out, because a binding the operator cannot see saved is
//! one they will make again after the next restart. MIDI learn works the same
//! way: arm a row, move the control, and the row captures whatever arrives.

use crate::i18n::{Tr, tr};
use eframe::egui::{self, Color32, ComboBox, RichText};
use sdroxide_types::MemoryChannel;

use crate::app::settings::SettingsIo;
use crate::chrome::StyledCombo;

/// The built-in TCI *server*: this app acting as a TCI rig for third-party
/// clients (WSJT-X's TCI rig type, JTDX, MSHV, skimmers). Distinct from the TCI
/// *client* section on the Radio tab, which connects sdroxide to another rig.

/// An action's name in the interface's language.
fn action_label(a: sdroxide_types::Action) -> String {
    use sdroxide_types::{Action::*, Vfo};
    match a {
        Tune => tr!("controls-action-tune"),
        RitOffset => tr!("controls-action-rit-offset"),
        XitOffset => tr!("controls-action-xit-offset"),
        Volume => tr!("controls-action-volume"),
        SubVolume => tr!("controls-action-sub-volume"),
        Squelch => tr!("controls-action-squelch"),
        FilterWidth => tr!("controls-action-filter-width"),
        FilterShift => tr!("controls-action-filter-shift"),
        AgcMaxGain => tr!("controls-action-agc-max-gain"),
        ManualGain => tr!("controls-action-manual-gain"),
        TxDrive => tr!("controls-action-tx-drive"),
        TuneDrive => tr!("controls-action-tune-drive"),
        MicGain => tr!("controls-action-mic-gain"),
        DigiAudioFreq => tr!("controls-action-digi-audio-freq"),
        SpectrumZoom => tr!("controls-action-spectrum-zoom"),
        SpectrumPan => tr!("controls-action-spectrum-pan"),
        SpectrumFloorDb => tr!("controls-action-spectrum-floor"),
        SpectrumCeilDb => tr!("controls-action-spectrum-ceiling"),
        Ptt => tr!("controls-action-ptt"),
        TuneCarrier => tr!("controls-action-tune-carrier"),
        Mute => tr!("controls-action-mute"),
        NoiseBlanker => tr!("controls-action-noise-blanker"),
        NoiseReductionCycle => tr!("controls-action-nr-cycle"),
        AutoNotch => tr!("controls-action-auto-notch"),
        Binaural => tr!("controls-action-binaural"),
        AgcCycle => tr!("controls-action-agc-cycle"),
        SubRx => tr!("controls-action-sub-rx"),
        Split => tr!("controls-action-split"),
        RitEnable => tr!("controls-action-rit-enable"),
        XitEnable => tr!("controls-action-xit-enable"),
        RitClear => tr!("controls-action-rit-clear"),
        XitClear => tr!("controls-action-xit-clear"),
        VfoToggle => tr!("controls-action-vfo-toggle"),
        SwapVfos => tr!("controls-action-swap-vfos"),
        CopyAtoB => tr!("controls-action-copy-a-to-b"),
        BandUp => tr!("controls-action-band-up"),
        BandDown => tr!("controls-action-band-down"),
        ModeNext => tr!("controls-action-mode-next"),
        ModePrev => tr!("controls-action-mode-prev"),
        RecordToggle => tr!("controls-action-record-toggle"),
        AbortTx => tr!("controls-action-abort-tx"),
        CwStraight => tr!("controls-action-cw-straight"),
        VoiceStop => tr!("controls-action-voice-stop"),
        ToneBurst => tr!("controls-action-tone-burst"),
        FitSpan => tr!("controls-action-fit-span"),
        ZoomIn => tr!("controls-action-zoom-in"),
        ZoomOut => tr!("controls-action-zoom-out"),
        PeakHold => tr!("controls-action-peak-hold"),
        SpectrumCollapse => tr!("controls-action-spectrum-collapse"),
        WaterfallCollapse => tr!("controls-action-waterfall-collapse"),
        WaterfallFlip => tr!("controls-action-waterfall-flip"),
        ToggleHelp => tr!("controls-action-toggle-help"),
        ToggleSettings => tr!("controls-action-toggle-settings"),
        ToggleLogbook => tr!("controls-action-toggle-logbook"),
        ToggleSpots => tr!("controls-action-toggle-spots"),
        ToggleMemories => tr!("controls-action-toggle-memories"),
        ToggleVoice => tr!("controls-action-toggle-voice"),
        Workspaces => tr!("controls-action-workspaces"),
        SpeakStatus => tr!("controls-action-speak-status"),
        SpeakRepeat => tr!("controls-action-speak-repeat"),
        SpeechSilence => tr!("controls-action-speech-silence"),
        SpeechToggle => tr!("controls-action-speech-toggle"),
        BroadcastLabels => tr!("controls-action-broadcast-labels"),
        VfoSelect(v) => {
            tr!("controls-action-select-vfo", vfo = if v == Vfo::A { "A" } else { "B" })
        }
        BandSelect(b) => tr!("controls-action-band", band = b.label()),
        ModeSelect(m) => tr!("controls-action-mode", mode = m.label()),
        MemoryRecall(n) => tr!("controls-action-recall-memory", n = n),
        VoicePlay(n) => tr!("controls-action-voice-slot", n = u32::from(n) + 1),
    }
}

/// A section heading of the bindings editor, from [`sdroxide_types::Action::group`].
fn action_group_label(group: &str) -> String {
    match group {
        "Tuning" => tr!("controls-group-tuning"),
        "Receive" => tr!("controls-group-receive"),
        "Transmit" => tr!("controls-group-transmit"),
        "Display" => tr!("controls-group-display"),
        "Windows" => tr!("controls-group-windows"),
        "Speech" => tr!("controls-group-speech"),
        other => other.to_string(),
    }
}

/// A MIDI control's identity, in the interface's language.
fn midi_msg_label(msg: &sdroxide_types::MidiMsg) -> String {
    use sdroxide_types::{MidiMsg, MidiMsgKind};
    let ch = if msg.channel == MidiMsg::ANY_CHANNEL {
        tr!("controls-midi-any")
    } else {
        (msg.channel + 1).to_string()
    };
    match msg.kind {
        MidiMsgKind::PitchBend => tr!("controls-midi-msg-pitch-bend", ch = ch),
        k => {
            let kind = match k {
                MidiMsgKind::Note => tr!("controls-midi-note"),
                MidiMsgKind::Cc => tr!("controls-midi-cc"),
                MidiMsgKind::PitchBend => tr!("controls-midi-pitch-bend"),
                MidiMsgKind::ProgramChange => tr!("controls-midi-program"),
            };
            tr!("controls-midi-msg", kind = kind, data = msg.data1, ch = ch)
        }
    }
}

/// A dropdown over every bindable [`Action`], grouped by section. `memories`
/// contributes one recall entry per stored channel, since those are the only
/// actions whose parameter comes from the operator's own data.
fn action_combo(
    ui: &mut egui::Ui,
    id: impl std::hash::Hash + std::fmt::Debug,
    action: &mut sdroxide_types::Action,
    memories: &[MemoryChannel],
) -> bool {
    use sdroxide_types::Action;
    let mut changed = false;
    ComboBox::from_id_salt(id).width(210.0).selected_text(action_label(*action)).show_styled(ui, |ui| {
        let mut group = "";
        let all =
            Action::all().into_iter().chain(memories.iter().map(|m| Action::MemoryRecall(m.id)));
        for a in all {
            if a.group() != group {
                group = a.group();
                ui.add_space(4.0);
                ui.label(RichText::new(action_group_label(group)).small().weak());
            }
            if ui.selectable_label(*action == a, action_label(a)).clicked() {
                *action = a;
                changed = true;
            }
        }
    });
    changed
}

/// Keyboard chords, panadapter mouse behaviour and mouse-button bindings.
///
/// Edits here are live and self-persisting; there is no APPLY chip, because a
/// binding is only useful once it is already in effect.
#[allow(clippy::too_many_arguments)]
pub(in crate::app) fn settings_controls_tab(
    ui: &mut egui::Ui,
    io: &mut SettingsIo,
    memories: &[MemoryChannel],
    midi_in: &[(String, String)],
    midi_out: &[(String, String)],
    midi_status: &crate::input::MidiStatusView,
    last_midi: Option<(sdroxide_types::MidiMsg, u8)>,
) {
    use sdroxide_types::{
        Action, ActionKind, ButtonMode, KeyBinding, MouseButton, MouseButtonBinding, WheelAction,
        WheelSettings,
    };
    let cfg = &mut *io.input_edit;
    let key_capture = &mut *io.key_capture;

    ui.label(RichText::new(tr!("settings-controls-keyboard")).size(14.0).strong().color(crate::theme::CYAN()));
    ui.add_space(4.0);
    super::note(
        ui,
        &tr!("settings-controls-click-shortcut-rebind-then-press"),
    );
    ui.add_space(4.0);
    ui.label(RichText::new(tr!("settings-controls-click-shortcut-rebind-then-press")).weak());
    ui.add_space(6.0);

    let mut remove: Option<usize> = None;
    egui::Grid::new("keys-grid")
        .num_columns(6)
        .spacing([10.0, 6.0])
        .min_row_height(ui.spacing().interact_size.y)
        .striped(true)
        .show(ui, |ui| {
            ui.label(RichText::new(tr!("settings-controls-shortcut")).small().weak());
            ui.label(RichText::new(tr!("settings-controls-does")).small().weak());
            ui.label(RichText::new(tr!("settings-controls-step-mode")).small().weak());
            ui.label(RichText::new(tr!("settings-controls-accel")).small().weak());
            ui.label(RichText::new(tr!("settings-controls-on")).small().weak());
            ui.label("");
            ui.end_row();

            for (i, b) in cfg.keys.iter_mut().enumerate() {
                let capturing = *key_capture == Some(i);
                let label = if capturing { tr!("settings-controls-press-key").to_string() } else { b.chord.label() };
                if crate::chrome::chip(ui, capturing, RichText::new(label).monospace()).clicked() {
                    *key_capture = if capturing { None } else { Some(i) };
                }

                if action_combo(ui, ("keyact", i), &mut b.action, memories) {
                    b.tuning.step = b.action.default_step();
                }

                match b.action.kind() {
                    ActionKind::Continuous => {
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::DragValue::new(&mut b.tuning.step)
                                    .speed(1.0)
                                    .range(0.0001..=1_000_000.0),
                            );
                            // The sign of `value` is the direction, so one
                            // action can have an up key and a down key.
                            let mut down = b.value < 0.0;
                            if crate::chrome::checkbox(ui, &mut down, tr!("settings-controls-down")).changed() {
                                b.value = if down { -1.0 } else { 1.0 };
                            }
                        });
                        ui.add(
                            egui::DragValue::new(&mut b.tuning.accel).speed(0.05).range(0.0..=4.0),
                        );
                    }
                    ActionKind::Momentary => {
                        ui.horizontal(|ui| {
                            for m in ButtonMode::ALL {
                                if crate::chrome::chip(ui, b.button == m, m.tr()).clicked() {
                                    b.button = m;
                                }
                            }
                        });
                        ui.label("");
                    }
                }

                crate::chrome::checkbox(ui, &mut b.enabled, "");
                if ui.small_button("✕").on_hover_text(tr!("settings-controls-remove-binding")).clicked() {
                    remove = Some(i);
                }
                ui.end_row();
            }
        });
    if let Some(i) = remove {
        cfg.keys.remove(i);
        if *key_capture == Some(i) {
            *key_capture = None;
        }
    }

    ui.add_space(6.0);
    ui.horizontal(|ui| {
        if crate::chrome::chip(ui, false, tr!("settings-controls-add-shortcut")).clicked() {
            cfg.keys.push(KeyBinding::default());
            *key_capture = Some(cfg.keys.len() - 1);
        }
        if crate::chrome::chip(ui, false, tr!("settings-controls-restore-defaults")).clicked() {
            cfg.keys = KeyBinding::defaults();
            *key_capture = None;
        }
        // PTT ships unbound on purpose; this is the one-click opt-in.
        let has_ptt = cfg.keys.iter().any(|b| b.action == Action::Ptt);
        if !has_ptt
            && crate::chrome::chip(ui, false, tr!("settings-controls-bind-hold-talk-space"))
                .on_hover_text(tr!("settings-controls-hold-space-transmit-releasing-losing"))
                .clicked()
        {
            cfg.keys.push(KeyBinding {
                chord: sdroxide_types::KeyChord::plain("Space"),
                action: Action::Ptt,
                button: ButtonMode::Momentary,
                ..KeyBinding::default()
            });
        }
    });

    ui.add_space(4.0);
    ui.horizontal(|ui| {
        ui.label(tr!("settings-controls-unkey-held-ptt-after"));
        ui.add(
            egui::DragValue::new(&mut cfg.ptt_hold_timeout_s)
                .speed(5.0)
                .range(0.0..=3600.0)
                .suffix(" s"),
        );
    })
    .response
    .on_hover_text(tr!("settings-controls-backstop-against-stuck-key-controller"));

    ui.add_space(10.0);
    ui.separator();
    ui.end_row();
    ui.add_space(6.0);
    ui.label(
        RichText::new(tr!("settings-controls-panadapter-mouse"))
            .size(14.0)
            .strong()
            .color(crate::theme::CYAN()),
    );
    ui.add_space(6.0);

    let w = &mut cfg.wheel;
    egui::Grid::new("mouse-grid")
        .num_columns(2)
        .spacing([12.0, 6.0])
        .min_row_height(ui.spacing().interact_size.y)
        .show(ui, |ui| {
            ui.label(tr!("settings-controls-wheel"));
            wheel_action_combo(ui, "wheel-plain", &mut w.wheel);
            ui.end_row();

            ui.label(tr!("settings-controls-wheel-shift"));
            wheel_action_combo(ui, "wheel-shift", &mut w.wheel_shift);
            ui.end_row();

            ui.label(tr!("settings-controls-tune-step"));
            ui.add(
                egui::DragValue::new(&mut w.tune_step_hz)
                    .speed(10.0)
                    .range(1.0..=1e6)
                    .suffix(" Hz"),
            );
            ui.end_row();

            ui.label(tr!("settings-controls-zoom-rate"));
            ui.add(egui::DragValue::new(&mut w.zoom_rate).speed(0.05).range(0.1..=5.0));
            ui.end_row();

            ui.label(tr!("settings-controls-click-tune-rounding"));
            ui.add(
                egui::DragValue::new(&mut w.click_tune_step_hz)
                    .speed(1.0)
                    .range(1.0..=10_000.0)
                    .suffix(" Hz"),
            )
            .on_hover_text(tr!("settings-controls-cw-keeps-the-fine-grid"));
            ui.end_row();
        });
    ui.add_space(4.0);
    crate::chrome::checkbox(ui, &mut w.invert, tr!("settings-controls-invert-wheel-direction"));
    crate::chrome::checkbox(
        ui,
        &mut w.drag_tunes,
        tr!("settings-controls-left-drag-tunes-well-pans"),
    )
    .on_hover_text(tr!("settings-controls-off-makes-left-drag-pan"));
    crate::chrome::checkbox(
        ui,
        &mut w.digit_wheel,
        tr!("settings-controls-scroll-digit-frequency-readout-tune"),
    );
    if w.wheel == WheelAction::Tune && w.wheel_shift == WheelAction::Tune {
        ui.label(
            RichText::new(tr!("settings-controls-both-wheel-actions-are-tune"))
                .color(Color32::from_rgb(230, 170, 60)),
        );
    }
    ui.add_space(6.0);
    if crate::chrome::chip(ui, false, tr!("settings-controls-restore-mouse-defaults")).clicked() {
        cfg.wheel = WheelSettings::default();
    }

    ui.add_space(10.0);
    ui.separator();
    ui.end_row();
    ui.add_space(6.0);
    ui.label(RichText::new(tr!("settings-controls-mouse-buttons")).size(14.0).strong().color(crate::theme::CYAN()));
    ui.add_space(4.0);
    super::note(
        ui,
        &tr!("settings-controls-left-right-buttons-are-reserved"),
    );
    ui.add_space(4.0);
    ui.label(RichText::new(tr!("settings-controls-left-right-buttons-are-reserved")).weak());
    ui.add_space(6.0);

    let mut remove: Option<usize> = None;
    egui::Grid::new("mousebtn-grid")
        .num_columns(5)
        .spacing([10.0, 6.0])
        .min_row_height(ui.spacing().interact_size.y)
        .striped(true)
        .show(ui, |ui| {
            for (i, b) in cfg.mouse_buttons.iter_mut().enumerate() {
                ComboBox::from_id_salt(("mb", i))
                    .width(130.0)
                    .selected_text(b.button.tr())
                    .show_styled(ui, |ui| {
                        for m in MouseButton::ALL {
                            if ui.selectable_label(b.button == m, m.tr()).clicked() {
                                b.button = m;
                            }
                        }
                    });
                action_combo(ui, ("mbact", i), &mut b.action, memories);
                ui.horizontal(|ui| {
                    for m in ButtonMode::ALL {
                        if crate::chrome::chip(ui, b.button_mode == m, m.tr()).clicked() {
                            b.button_mode = m;
                        }
                    }
                });
                crate::chrome::checkbox(ui, &mut b.enabled, "");
                if ui.small_button("✕").clicked() {
                    remove = Some(i);
                }
                ui.end_row();
            }
        });
    if let Some(i) = remove {
        cfg.mouse_buttons.remove(i);
    }
    ui.add_space(6.0);
    if crate::chrome::chip(ui, false, tr!("settings-controls-add-mouse-button")).clicked() {
        cfg.mouse_buttons.push(MouseButtonBinding::default());
    }

    ui.add_space(8.0);
    super::note(ui, &tr!("settings-controls-f1-always-opens-manual-even"));

    ui.add_space(10.0);
    ui.separator();
    ui.end_row();
    ui.add_space(6.0);
    settings_midi_section(
        ui,
        cfg,
        io.midi_learn,
        io.midi_rescan,
        memories,
        midi_in,
        midi_out,
        midi_status,
        last_midi,
    );
}

/// MIDI control surfaces: port selection, a live message readout, and the
/// binding table with its LEARN capture.
#[allow(clippy::too_many_arguments)]
fn settings_midi_section(
    ui: &mut egui::Ui,
    cfg: &mut sdroxide_types::InputSettings,
    learn: &mut Option<crate::input::MidiLearn>,
    rescan: &mut bool,
    memories: &[MemoryChannel],
    midi_in: &[(String, String)],
    midi_out: &[(String, String)],
    status: &crate::input::MidiStatusView,
    last_midi: Option<(sdroxide_types::MidiMsg, u8)>,
) {
    use sdroxide_types::{ActionKind, ButtonMode, MidiBinding, RelativeMode};

    ui.label(
        RichText::new(tr!("settings-controls-midi-controller"))
            .size(14.0)
            .strong()
            .color(crate::theme::CYAN()),
    );
    ui.add_space(4.0);
    if !status.supported {
        super::note(
            ui,
            &tr!("settings-controls-midi-controllers-need-native-app"),
        );
        return;
    }
    super::note(
        ui,
        &tr!("settings-controls-any-class-compliant-midi-surface"),
    );
    ui.add_space(6.0);
    crate::chrome::checkbox(ui, &mut cfg.midi.enabled, tr!("settings-controls-enable"));
    ui.add_space(6.0);

    ui.add_enabled_ui(cfg.midi.enabled, |ui| {
        egui::Grid::new("midi-ports")
            .num_columns(2)
            .spacing([12.0, 6.0])
            .min_row_height(ui.spacing().interact_size.y)
            .show(ui, |ui| {
                ui.label(tr!("settings-controls-controller"));
                midi_port_combo(
                    ui,
                    "midi-in",
                    midi_in,
                    &mut cfg.midi.in_port_id,
                    &mut cfg.midi.in_port_name,
                );
                ui.end_row();

                ui.label(tr!("settings-controls-feedback"));
                midi_port_combo(
                    ui,
                    "midi-out",
                    midi_out,
                    &mut cfg.midi.out_port_id,
                    &mut cfg.midi.out_port_name,
                );
                ui.end_row();
            });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            if crate::chrome::chip(ui, false, tr!("settings-controls-rescan-ports")).clicked() {
                *rescan = true;
            }
            if status.connected {
                ui.label(
                    RichText::new(format!("● {}", status.port))
                        .color(Color32::from_rgb(90, 200, 110)),
                );
            } else if let Some(e) = &status.error {
                ui.label(RichText::new(e).color(Color32::from_rgb(230, 90, 80)));
            } else {
                super::note(ui, &tr!("settings-controls-not-connected"));
            }
        });
        ui.add_space(4.0);
        // Naming the control that just moved is what makes an unlabelled
        // surface bindable at all.
        match last_midi {
            Some((msg, v)) => super::note(ui, &tr!("settings-controls-last-midi", message = midi_msg_label(&msg), value = v)),
            None => super::note(ui, &tr!("settings-controls-move-control-see-here")),
        };
    });

    ui.add_space(8.0);
    let mut remove: Option<usize> = None;
    egui::Grid::new("midi-grid")
        .num_columns(7)
        .spacing([10.0, 6.0])
        .min_row_height(ui.spacing().interact_size.y)
        .striped(true)
        .show(ui, |ui| {
            ui.label(RichText::new(tr!("settings-controls-control")).small().weak());
            ui.label(RichText::new(tr!("settings-controls-does")).small().weak());
            ui.label(RichText::new(tr!("settings-controls-reads")).small().weak());
            ui.label(RichText::new(tr!("settings-controls-step-mode")).small().weak());
            ui.label(RichText::new(tr!("settings-controls-led")).small().weak());
            ui.label(RichText::new(tr!("settings-controls-on")).small().weak());
            ui.label("");
            ui.end_row();

            for (i, b) in cfg.midi.bindings.iter_mut().enumerate() {
                let learning = learn.map(|l| l.row) == Some(i);
                let label = if learning { tr!("settings-controls-move").to_string() } else { midi_msg_label(&b.msg) };
                if crate::chrome::chip(ui, learning, RichText::new(label).monospace())
                    .on_hover_text(tr!("settings-controls-click-then-move-control-you"))
                    .clicked()
                {
                    *learn = if learning { None } else { Some(crate::input::MidiLearn { row: i }) };
                }

                if action_combo(ui, ("midiact", i), &mut b.action, memories) {
                    b.tuning.step = b.action.default_step();
                }

                match b.action.kind() {
                    ActionKind::Continuous => {
                        ComboBox::from_id_salt(("midirel", i))
                            .width(170.0)
                            .selected_text(b.relative.tr())
                            .show_styled(ui, |ui| {
                                for m in RelativeMode::ALL {
                                    if ui.selectable_label(b.relative == m, m.tr()).clicked() {
                                        b.relative = m;
                                    }
                                }
                            });
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::DragValue::new(&mut b.tuning.step)
                                    .speed(1.0)
                                    .range(0.0001..=1_000_000.0),
                            );
                            ui.add(
                                egui::DragValue::new(&mut b.tuning.accel)
                                    .speed(0.05)
                                    .range(0.0..=4.0)
                                    .prefix("×"),
                            )
                            .on_hover_text(tr!("settings-controls-speed-sensitivity-spin-faster-tune"));
                            // Sign/magnitude and 64-centred encoders are
                            // indistinguishable from small movements, so a wrong
                            // guess shows up as a knob that turns the wrong way.
                            crate::chrome::checkbox(ui, &mut b.tuning.invert, tr!("settings-controls-rev"));
                        });
                    }
                    ActionKind::Momentary => {
                        ui.label("");
                        ui.horizontal(|ui| {
                            for m in ButtonMode::ALL {
                                if crate::chrome::chip(ui, b.button_mode == m, m.tr()).clicked()
                                {
                                    b.button_mode = m;
                                }
                            }
                        });
                    }
                }

                crate::chrome::checkbox(ui, &mut b.feedback, "")
                    .on_hover_text(tr!("settings-controls-send-current-value-back-light"));
                crate::chrome::checkbox(ui, &mut b.enabled, "");
                if ui.small_button("✕").clicked() {
                    remove = Some(i);
                }
                ui.end_row();
            }
        });
    if let Some(i) = remove {
        cfg.midi.bindings.remove(i);
        if learn.map(|l| l.row) == Some(i) {
            *learn = None;
        }
    }

    ui.add_space(6.0);
    ui.horizontal(|ui| {
        if crate::chrome::chip(ui, false, tr!("settings-controls-add-midi-control")).clicked() {
            cfg.midi.bindings.push(MidiBinding::default());
            *learn = Some(crate::input::MidiLearn { row: cfg.midi.bindings.len() - 1 });
        }
        if !cfg.midi.bindings.is_empty()
            && crate::chrome::chip(ui, false, tr!("settings-controls-clear-all")).clicked()
        {
            cfg.midi.bindings.clear();
            *learn = None;
        }
    });

    ui.add_space(8.0);
    super::note(
        ui,
        &tr!("settings-controls-endless-jog-encoders-send-relative"),
    );
}

/// Port dropdown that keeps both the stable id and the human name — the id
/// reconnects across a replug, the name is the label and the fallback.
fn midi_port_combo(
    ui: &mut egui::Ui,
    id: &str,
    ports: &[(String, String)],
    sel_id: &mut String,
    sel_name: &mut String,
) {
    let shown = if sel_name.is_empty() {
        tr!("settings-controls-none")
    } else {
        String::from(sel_name.as_str())
    };
    ComboBox::from_id_salt(id).width(280.0).selected_text(shown).show_styled(ui, |ui| {
        if ui.selectable_label(sel_name.is_empty(), tr!("settings-controls-none")).clicked() {
            sel_id.clear();
            sel_name.clear();
        }
        for (pid, name) in ports {
            if ui.selectable_label(sel_name == name, name).clicked() {
                *sel_id = pid.clone();
                *sel_name = name.clone();
            }
        }
    });
}

/// Dropdown over [`WheelAction`].
fn wheel_action_combo(ui: &mut egui::Ui, id: &str, act: &mut sdroxide_types::WheelAction) {
    ComboBox::from_id_salt(id).width(130.0).selected_text(act.tr()).show_styled(ui, |ui| {
        for a in sdroxide_types::WheelAction::ALL {
            if ui.selectable_label(*act == a, a.tr()).clicked() {
                *act = a;
            }
        }
    });
}
