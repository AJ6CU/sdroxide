//! Settings → Audio: your own sound, this radio's sound, and where recordings go.
//!
//! **One tab, because "audio" was spread over four of them under four different
//! names** — the operator's speakers and microphone on General, the rig's sound
//! card on Radio (differently again on each backend), speech on UI and the alert
//! tones on Alerts. None of those is wrong where it was; they are simply not
//! findable as one thing, and a beginner cannot tell which one owns a setting.
//!
//! The organising idea is **whose audio it is**, not which file it is saved in:
//!
//! - **Yours** is a property of this machine and the person at it. One screen,
//!   one operator, so it is station-wide and there is nothing to be per-radio
//!   about. Your microphone is here, which is the one an SSB operator asks for.
//! - **This radio's** is a property of the rig and what it is plugged into. The
//!   SS9900v on channel 26 and an SDR on the bench share this machine but not a
//!   sound card, so these stay with the radio — see [`RadioConfig`].
//!
//! A note on what a picker can and cannot promise: the device list is what the
//! host audio API enumerates, and a sound server may hold devices it does not
//! report. Every picker here is therefore also a text field ([`device_field`]),
//! because being unable to *name* a device is being unable to use it.

use eframe::egui::{self, ComboBox, RichText};

use super::general::{device_field, settings_rx_audio_gain};
use crate::app::SdroxideApp;
use crate::i18n::{Tr as _, tr};
use crate::chrome::StyledCombo as _;
use crate::theme::gray;

/// Whether this device belongs to the machine's sound server rather than to a
/// sound card, and so is chosen by taking the machine's default.
///
/// A browser client has no sound server of its own to take over and no ALSA to
/// ask, so the answer there is always no and the warning never appears — which
/// is right, because in a browser the page really does own its own capture.
#[cfg(not(target_arch = "wasm32"))]
fn is_machine_default_device(name: &str, output: bool) -> bool {
    // **The shape first, as a pure check, and the list only to confirm.**
    //
    // This function is called every frame the page draws, and asking the list
    // means spawning a `pactl` process — tens of them a second, each one a
    // PipeWire client that connects and is destroyed. WirePlumber aborts an
    // ALSA node's activation when a client vanishes mid-activation, and the
    // sound card was disappearing from the graph for good as a result. So the
    // common case — a cpal display name like `Generic KM_B2 Audio, USB Audio
    // [Audio]` — is answered here by shape, with **no process spawned at all**.
    //
    // The shape alone is not the answer, and that is the trap this had already
    // fallen into once: `is_graph_node_name` matches `alsa_input.`, and the
    // ordinary analog card is literally `alsa_input.pci-0000_06_00.6.analog-stereo`,
    // so shape alone warned that a plain sound card takes over the machine's
    // recording. Shape decides whether the list is *worth asking*; the list
    // decides. And the list itself is cached now, so a page that does ask costs
    // one process every few seconds rather than one per frame.
    if !sdroxide_audio::graph::is_graph_node_name(name) {
        return false;
    }
    sdroxide_audio::graph::extra_device_names(output).iter().any(|n| n == name)
}
#[cfg(target_arch = "wasm32")]
fn is_machine_default_device(_name: &str, _output: bool) -> bool {
    false
}

/// What picking a sound-server device costs, said where it is chosen.
///
/// The cost is real and unavoidable, and the first version of this feature hid
/// it in the log, which is the one place an operator never reads. **PipeWire has
/// no way for one program to record from a source of its own** — playback has a
/// per-stream target, capture does not — so a Bluetooth headset can only be
/// used by becoming the machine's default microphone. That moves the desktop's
/// recording too, and until this note existed the program took over the
/// machine's microphone and said nothing.
///
/// So it says what happened and gives the way back. A control that changes
/// something outside itself without saying so is this fork's own bug class.
fn sound_server_takes_over(ui: &mut egui::Ui, what: &str, device: &str, output: bool) {
    let device = device.to_string();
    let warning = match (what, output) {
        ("speakers", _) => tr!("settings-audio-takes-over-speakers"),
        _ => tr!("settings-audio-takes-over-microphone"),
    };
    ui.add(
        egui::Label::new(
            RichText::new(tr!("settings-audio-currently-chosen", device = device))
                .size(10.5)
                .color(egui::Color32::from_rgb(220, 170, 40)),
        )
        .wrap(),
    );
    note(ui, &warning);
}

/// Which sound-card rows this radio has, and which way they run.
///
/// Per backend rather than one shape for everything, and the difference is not
/// cosmetic: the ATS Mini is receive-only — it has no microphone socket — and the
/// ELAD's transmit audio goes to a control of its own. Offering a transmit row
/// for a radio that cannot transmit is a control that does nothing, which this
/// fork treats as a fault rather than as tidiness.
fn sound_card_roles(backend: sdroxide_types::Backend) -> Option<(bool, bool)> {
    use sdroxide_types::Backend as B;
    // Qualified, not globbed: `Backend::None` would shadow `Option::None` for
    // the whole function, and the arms stop agreeing on what they return.
    match backend {
        B::Cat | B::UsbAudio => Some((true, true)),
        B::AtsMini => Some((true, false)),
        B::Elad => Some((false, true)),
        _ => None,
    }
}

/// A heading, with the explanation that belongs under it.
///
/// The explanation is a **wrapped** label: a plain `ui.label` on a `RichText`
/// lays out on one line and is clipped at the page's edge, so every paragraph
/// here ended mid-sentence against the window. Rendering it is what showed that.
fn section(ui: &mut egui::Ui, title: &str, why: &str) {
    ui.add_space(10.0);
    ui.label(RichText::new(title).strong());
    ui.add(egui::Label::new(RichText::new(why).size(10.5).color(gray(140))).wrap());
    ui.add_space(2.0);
}

/// One labelled control: the name above it, the control under it.
///
/// **Not a two-column grid.** In a grid the label is centred against the whole
/// cell, and a device picker is two controls stacked — so the label landed
/// beside the entry field instead of the dropdown, and the field was left at the
/// dropdown's width with its hint cut off. Both were visible in a screenshot
/// before anything here had been run by a person.
fn field(ui: &mut egui::Ui, label: &str, add: impl FnOnce(&mut egui::Ui)) {
    ui.label(RichText::new(label).size(10.5).color(gray(150)));
    add(ui);
    ui.add_space(6.0);
}

/// How wide a device control is drawn. Wider than the old 300 pt: a sound card
/// is named in full — *Generic AB13X USB Audio, USB Audio [Audio]…* — and 300
/// cut it off mid-name, so a list of four devices looked like one.
pub(super) const DEVICE_WIDTH: f32 = 460.0;

/// The audio page's notes, drawn by the shared helper so every settings page
/// shows its explanations the same way — one elided line, the rest on the
/// hover. See [`super::note`].
fn note(ui: &mut egui::Ui, text: &str) {
    super::note_small(ui, text);
}

impl SdroxideApp {
    /// Settings → Audio.
    pub(in crate::app) fn settings_audio_tab(
        &self,
        ui: &mut egui::Ui,
        audio_pick: &mut Option<(bool, Option<String>)>,
        routing_pick: &mut Option<sdroxide_types::AudioRouting>,
        radio_edit: &mut Option<sdroxide_types::RadioConfig>,
        can_probe: bool,
        audio_rescan: &mut bool,
        recordings_choose: &mut bool,
        recordings_reset: &mut bool,
    ) {
        self.your_audio(ui, audio_pick, routing_pick);
        self.radios_audio(ui, radio_edit, can_probe, audio_rescan);
        // After the sound-card rows, and outside them: it is about this radio's
        // received audio and not about a card.
        self.stream_audio_out(ui, radio_edit);
        self.recordings(ui, recordings_choose, recordings_reset);
    }

    /// Your speakers, your microphone, and which way the audio reaches you.
    ///
    /// The microphone is here because it is the one an operator transmitting
    /// voice asks for and the one that is hardest to find: it is what carries
    /// your voice to the rig when the rig is keyed by sound (MCW) or by VOX.
    fn your_audio(
        &self,
        ui: &mut egui::Ui,
        audio_pick: &mut Option<(bool, Option<String>)>,
        routing_pick: &mut Option<sdroxide_types::AudioRouting>,
    ) {
        section(
            ui,
            &tr!("settings-audio-your-audio"),
            &tr!("settings-audio-your-audio-why"),
        );

        let Some(devs) = &self.audio_devices else {
            // The browser client has no device list to offer — the page plays
            // and records through the browser, and the choices live in the
            // browser's own site settings. Saying so is the whole of the job
            // here: a row that renders nothing is a control that silently does
            // nothing.
            note(
                ui,
                &tr!("settings-audio-browser-note"),
            );
            return;
        };

        let mut out = devs.selected_output.clone();
        let out_chosen = out.clone();
        field(ui, &tr!("settings-audio-speakers"), |ui| {
            device_field(ui, "a-your-out", &devs.outputs, &mut out, "speakers");
        });
        if out != devs.selected_output {
            *audio_pick = Some((true, out));
        }
        if let Some(chosen) = out_chosen.as_deref() {
            if is_machine_default_device(chosen, true) {
                sound_server_takes_over(ui, "speakers", chosen, true);
            }
        }

        field(ui, &tr!("settings-general-channels"), |ui| {
            let mut routing = devs.routing;
            ComboBox::from_id_salt("a-your-routing")
                .width(200.0)
                .selected_text(routing.tr())
                .show_styled(ui, |ui| {
                    for r in sdroxide_types::AudioRouting::ALL {
                        ui.selectable_value(&mut routing, r, r.tr());
                    }
                })
                .response
                .on_hover_text(
                    tr!("settings-audio-channels-hint"),
                );
            if routing != devs.routing {
                *routing_pick = Some(routing);
            }
        });

        let mut mic = devs.selected_input.clone();
        field(ui, &tr!("elad-tx-input-mic"), |ui| {
            device_field(ui, "a-your-in", &devs.inputs, &mut mic, "microphone");
        });
        let mic_chosen = mic.clone();
        if mic != devs.selected_input {
            *audio_pick = Some((false, mic));
        }
        if let Some(chosen) = mic_chosen.as_deref() {
            if is_machine_default_device(chosen, false) {
                sound_server_takes_over(ui, "microphone", chosen, false);
            }
        }

        note(
            ui,
            &tr!("settings-audio-microphone-note"),
        );
    }

    /// **The received audio, streamed out to another program.**
    ///
    /// Its own section rather than a row of the sound-card one, because a sound
    /// card is not what it needs: **every radio has received audio**. An SDR
    /// demodulates in the program and has plenty; a CAT rig gets it from a
    /// card. Gating it on `sound_card_roles` made the setting unreachable for
    /// every SDR in the list — SdrPlay, RTL-SDR, Soapy, Pluto and the rest,
    /// which is most of them — and it was found by somebody trying to follow the
    /// instruction to set it. The setting existed, the engine honoured it, and
    /// nothing on screen could turn it on.
    ///
    /// Per radio, because the audio is: two radios here have two streams.
    fn stream_audio_out(
        &self,
        ui: &mut egui::Ui,
        radio_edit: &mut Option<sdroxide_types::RadioConfig>,
    ) {
        let Some(cfg) = radio_edit.as_mut() else { return };
        section(
            ui,
            &tr!("settings-audio-stream-out"),
            &tr!("settings-audio-stream-out-why"),
        );
        ui.horizontal(|ui| {
            ui.label(RichText::new(tr!("settings-servers-send")).strong());
            ui.end_row();
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut cfg.udp_audio)
                    .desired_width(150.0)
                    .hint_text(tr!("topbar-off")),
            )
            .on_hover_text(
                tr!("settings-audio-send-to-hint"),
            );
        });
        note(
            ui,
            &tr!("settings-audio-send-to-note"),
        );
    }

    /// This radio's sound card: what comes in from the rig, what goes out to it.
    fn radios_audio(
        &self,
        ui: &mut egui::Ui,
        radio_edit: &mut Option<sdroxide_types::RadioConfig>,
        can_probe: bool,
        rescan: &mut bool,
    ) {
        let Some(cfg) = radio_edit.as_mut() else { return };
        let Some((has_rx, has_tx)) = sound_card_roles(cfg.backend) else {
            return;
        };

        section(
            ui,
            &tr!("settings-audio-radio-audio"),
            &tr!("settings-audio-radio-audio-why"),
        );

        let cards = self
            .radio_audio_devices
            .as_ref()
            .map(|(inputs, outputs)| (inputs.as_slice(), outputs.as_slice()));

        if cards.is_none() {
            super::note_rich(
                ui,
                &tr!("settings-audio-no-cards-yet"),
                Some(10.5),
                None,
            );
        }
        let (inputs, outputs) = cards.unwrap_or((&[], &[]));

        // The rescan belongs beside the pickers it refreshes rather than at the
        // foot of the Radio tab: it is this row the rescan is *for*. It asks the
        // machine again which sound cards it has, and opens nothing, so it is
        // safe to press while receiving.
        crate::app::settings::radio::probe_only(ui, can_probe, |ui| {
            if ui
                .button(tr!("settings-audio-rescan"))
                .on_hover_text(
                    tr!("settings-audio-rescan-hint"),
                )
                .clicked()
            {
                *rescan = true;
            }
        });

        if has_rx {
            let mut rx = cfg.radio_audio_in.clone();
            field(ui, &tr!("settings-audio-from-radio"), |ui| {
                device_field(ui, "a-radio-in", inputs, &mut rx, "radio-rx");
            });
            if rx != cfg.radio_audio_in {
                cfg.radio_audio_in = rx;
            }
        }
        if has_tx {
            let mut tx = cfg.radio_audio_out.clone();
            field(ui, &tr!("settings-audio-to-radio"), |ui| {
                device_field(ui, "a-radio-out", outputs, &mut tx, "radio-tx");
            });
            if tx != cfg.radio_audio_out {
                cfg.radio_audio_out = tx;
            }
            // Mixer level for the radio's TX audio device, so a USB interface
            // (SS9900v) is loud enough to trigger its VOX. Only meaningful for
            // an ALSA card; a no-op elsewhere.
            let mut out_level = cfg.radio_audio_out_level.unwrap_or(80);
            field(ui, &tr!("settings-audio-radio-out-level"), |ui| {
                let resp = ui.add(egui::Slider::new(&mut out_level, 0..=100).suffix("%"));
                if resp.changed() {
                    cfg.radio_audio_out_level = Some(out_level);
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(name) = cfg.radio_audio_out.as_deref() {
                        sdroxide_audio::set_mixer_level(name, true, out_level);
                    }
                }
                resp.on_hover_text(tr!("settings-audio-radio-out-level-hint"));
            });
        }

        if has_rx {
            let mut in_level = cfg.radio_audio_in_level.unwrap_or(80);
            field(ui, &tr!("settings-audio-radio-in-level"), |ui| {
                let resp = ui.add(egui::Slider::new(&mut in_level, 0..=100).suffix("%"));
                if resp.changed() {
                    cfg.radio_audio_in_level = Some(in_level);
                    #[cfg(not(target_arch = "wasm32"))]
                    if let Some(name) = cfg.radio_audio_in.as_deref() {
                        sdroxide_audio::set_mixer_level(name, false, in_level);
                    }
                }
                resp.on_hover_text(tr!("settings-audio-radio-in-level-hint"));
            });
        }

        if has_rx {
            settings_rx_audio_gain(ui, cfg);
        }

        note(
            ui,
            &tr!("settings-audio-radio-note"),
        );
    }

    /// Where recordings are written.
    ///
    /// Only the folder: the rest of the recording controls — how long a clip
    /// runs, whether a file opens on the squelch — are answers to *this*
    /// session rather than settings, so they stay on the REC chip where they are
    /// pressed.
    fn recordings(&self, ui: &mut egui::Ui, choose: &mut bool, reset: &mut bool) {
        // The section heading is this one; the folder row underneath brings its
        // own, which would otherwise read "Recording" and then "Recordings" two
        // lines apart.
        section(
            ui,
            &tr!("settings-audio-recording"),
            &tr!("settings-audio-recording-why"),
        );
        self.settings_recordings_dir(ui, choose, reset);
    }
}

#[cfg(test)]
mod tests {
    use super::sound_card_roles;
    use sdroxide_types::Backend;

    #[test]
    fn a_sound_card_radio_offers_both_directions() {
        for b in [Backend::Cat, Backend::UsbAudio] {
            assert_eq!(sound_card_roles(b), Some((true, true)), "{b:?}");
        }
    }

    #[test]
    fn a_receive_only_radio_is_not_offered_a_transmit_row() {
        // The ATS Mini has no microphone socket: offering the row would be a
        // control that cannot do the thing.
        assert_eq!(sound_card_roles(Backend::AtsMini), Some((true, false)));
    }

    #[test]
    fn an_sdr_is_offered_no_sound_card_at_all() {
        for b in [Backend::SdrPlay, Backend::RtlSdr, Backend::None, Backend::SpyServer] {
            assert_eq!(sound_card_roles(b), None, "{b:?}");
        }
    }
}
