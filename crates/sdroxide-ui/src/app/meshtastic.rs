//! The MESHTASTIC window: the LoRa mesh traffic of one channel, the nodes
//! heard and the packets as they come in.
//!
//! Like SONDE and WXSAT it is a view, not a radio mode: the receiver runs off
//! its own downconversion of the sample window (see `sdroxide_meshtastic`) and
//! the radio keeps its mode. State comes from
//! [`sdroxide_types::MeshtasticStatus`] (what was heard) and
//! [`sdroxide_types::RadioState::meshtastic`] (what the station is set to).
//!
//! The panel never decides whether the radio may listen at a frequency: that
//! belongs to the receive band list in the Radio menu, which this file only
//! reads. Choosing a channel sends the ordinary tune command, which that list
//! already gates, and the status says why when the lane stays idle.

use crate::i18n::tr;
use eframe::egui::{self, RichText};
use sdroxide_meshtastic::protocol::{
    PRESETS, Preset, REGIONS, Region, lookup_preset, lookup_region, port_name,
};
use sdroxide_types::{Command, MeshtasticSettings, MeshtasticStatus, RxBlock};

const WINDOW_SIZE: [f32; 2] = [780.0, 600.0];

/// Temp-memory keys that carry the panel's chips from the places that have no
/// access to the app (the experimental bench in the mode menu) to the frame.
const AVAILABLE_KEY: &str = "meshtastic-available";
const OPEN_KEY: &str = "meshtastic-open";
const TOGGLE_KEY: &str = "meshtastic-toggle";

/// The preset the settings name, the default one when the name is unknown.
fn preset_of(cfg: &MeshtasticSettings) -> &'static Preset {
    lookup_preset(&cfg.preset).unwrap_or(sdroxide_meshtastic::protocol::DEFAULT_PRESET)
}

/// The region the settings name, EU 868 when the name is unknown.
fn region_of(cfg: &MeshtasticSettings) -> &'static Region {
    lookup_region(&cfg.region).unwrap_or(&REGIONS[2])
}

/// The slot centres of a region for a preset, in hertz.
pub(in crate::app) fn channel_slots(region: &Region, preset: &Preset) -> Vec<f64> {
    let bw = if region.wide { preset.bw_hz_wide } else { preset.bw_hz };
    (0..region.channel_count(bw))
        .filter_map(|i| region.channel_freq_hz(bw, i))
        .map(|f| f as f64)
        .collect()
}

/// The slot a channel name lands on, as the firmware picks it: the djb2 hash of
/// the name modulo the number of slots. `0` when there is no slot.
pub(in crate::app) fn default_slot(channel_name: &str, slots: usize) -> usize {
    if slots == 0 {
        return 0;
    }
    let mut h: u32 = 5381;
    for b in channel_name.bytes() {
        h = h.wrapping_shl(5).wrapping_add(h).wrapping_add(u32::from(b));
    }
    (h as usize) % slots
}

/// `cfg` with the region, preset and frequency of the channel the firmware
/// would pick for `region` and `preset`.
pub(in crate::app) fn with_default_channel(
    cfg: &MeshtasticSettings,
    region: &Region,
    preset: &Preset,
) -> MeshtasticSettings {
    let slots = channel_slots(region, preset);
    let hz = slots
        .get(default_slot(preset.channel_name, slots.len()))
        .copied()
        .unwrap_or(cfg.frequency_hz);
    MeshtasticSettings {
        region: region.name.to_string(),
        preset: preset.channel_name.to_string(),
        frequency_hz: hz,
        ..cfg.clone()
    }
}

/// The commands "Listen here" sends: switch the receiver on with the channel,
/// then bring the dial to it. The first goes first so that the engine notes
/// where the radio was before the tune moves it; the second is the ordinary
/// tune, which the receive band list gates like any other.
pub(in crate::app) fn tune_commands(cfg: &MeshtasticSettings) -> Vec<Command> {
    vec![
        Command::SetMeshtasticConfig(MeshtasticSettings { enabled: true, ..cfg.clone() }),
        Command::TuneWidebandTo(cfg.frequency_hz),
    ]
}

/// "!1a2b3c4d", the way the apps write a node number.
pub(in crate::app) fn node_label(id: u32) -> String {
    format!("!{id:08x}")
}

/// HH:MM:SS (UTC) of a Unix second.
fn clock(t: i64) -> String {
    let s = t.rem_euclid(86_400);
    format!("{:02}:{:02}:{:02}", s / 3600, (s / 60) % 60, s % 60)
}

/// The refusal of the receive band list, in the operator's language.
fn rx_block_text(why: RxBlock, hz: f64) -> String {
    match why {
        RxBlock::BandOff(b) => tr!(
            "meshtastic-refused-band",
            freq = format!("{:.3}", hz / 1e6),
            band = crate::app::tx_bands::tx_band_label(b)
        ),
        RxBlock::Unconfirmed => tr!("meshtastic-refused-unconfirmed"),
    }
}

/// Publish what the chips outside the app need to know this frame.
pub(in crate::app) fn publish(ctx: &egui::Context, available: bool, open: bool) {
    ctx.data_mut(|d| {
        d.insert_temp(egui::Id::new(AVAILABLE_KEY), available);
        d.insert_temp(egui::Id::new(OPEN_KEY), open);
    });
}

/// Whether a chip asked to open or close the window since the last call.
pub(in crate::app) fn take_toggle(ctx: &egui::Context) -> bool {
    ctx.data_mut(|d| d.remove_temp::<bool>(egui::Id::new(TOGGLE_KEY))).unwrap_or(false)
}

/// The chip of the experimental bench. Nothing is drawn while the panel is
/// switched off in the settings.
pub(in crate::app) fn bench_chip(ui: &mut egui::Ui) {
    let ctx = ui.ctx().clone();
    let (available, open) = ctx.data(|d| {
        (
            d.get_temp::<bool>(egui::Id::new(AVAILABLE_KEY)).unwrap_or(false),
            d.get_temp::<bool>(egui::Id::new(OPEN_KEY)).unwrap_or(false),
        )
    });
    if !available {
        return;
    }
    if crate::chrome::chip(ui, open, tr!("meshtastic-title"))
        .on_hover_text(tr!("meshtastic-bench-hint"))
        .clicked()
    {
        ctx.data_mut(|d| d.insert_temp(egui::Id::new(TOGGLE_KEY), true));
    }
}

impl super::SdroxideApp {
    /// Apply the chips' request and the settings switch, then draw the window.
    pub(in crate::app) fn meshtastic_window(
        &mut self,
        ctx: &egui::Context,
        cmds: &mut Vec<Command>,
    ) {
        let available = self.ui_settings.meshtastic_panel;
        if take_toggle(ctx) && available {
            self.show_meshtastic = !self.show_meshtastic;
        }
        if !available {
            self.show_meshtastic = false;
        }
        publish(ctx, available, self.show_meshtastic);
        if !self.show_meshtastic {
            return;
        }
        let mut out = Vec::new();
        let open = self.tool_window(
            ctx,
            "meshtastic",
            &tr!("meshtastic-title"),
            WINDOW_SIZE,
            true,
            |me, ui| me.meshtastic_body(ui, &mut out),
        );
        cmds.extend(out);
        self.show_meshtastic = open;
    }

    fn meshtastic_body(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        let cfg = self.state.meshtastic.clone();
        let mut edit = cfg.clone();
        let mut tune = false;

        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut edit.enabled, tr!("meshtastic-receive"));

            ui.label(tr!("meshtastic-region"));
            egui::ComboBox::from_id_salt("meshtastic-region")
                .selected_text(region_of(&edit).name)
                .show_ui(ui, |ui| {
                    for r in &REGIONS {
                        if ui.selectable_label(region_of(&edit).name == r.name, r.name).clicked() {
                            edit = with_default_channel(&edit, r, preset_of(&edit));
                            tune = true;
                        }
                    }
                });

            ui.label(tr!("meshtastic-preset"));
            egui::ComboBox::from_id_salt("meshtastic-preset")
                .selected_text(preset_of(&edit).channel_name)
                .show_ui(ui, |ui| {
                    for p in &PRESETS {
                        if ui
                            .selectable_label(preset_of(&edit).name == p.name, p.channel_name)
                            .clicked()
                        {
                            edit = with_default_channel(&edit, region_of(&edit), p);
                            tune = true;
                        }
                    }
                });

            let slots = channel_slots(region_of(&edit), preset_of(&edit));
            ui.label(tr!("meshtastic-channel"));
            egui::ComboBox::from_id_salt("meshtastic-slot")
                .selected_text(format!("{:.4} MHz", edit.frequency_hz / 1e6))
                .show_ui(ui, |ui| {
                    for (i, hz) in slots.iter().enumerate() {
                        let label = format!("#{}  {:.4} MHz", i + 1, hz / 1e6);
                        if ui
                            .selectable_label((edit.frequency_hz - hz).abs() < 1.0, label)
                            .clicked()
                        {
                            edit.frequency_hz = *hz;
                            tune = true;
                        }
                    }
                });

            if ui
                .button(tr!("meshtastic-tune"))
                .on_hover_text(tr!("meshtastic-tune-hint"))
                .clicked()
            {
                tune = true;
            }
        });

        // The keys: committed when the field is left, not on every key, so a
        // half-typed key never reconfigures the receiver.
        ui.horizontal(|ui| {
            ui.label(tr!("meshtastic-keys"));
            let draft = self.meshtastic_keys.get_or_insert_with(|| cfg.keys.clone());
            let r = crate::chrome::field_sized(
                ui,
                egui::vec2(ui.available_width().max(160.0), 0.0),
                egui::TextEdit::singleline(draft).hint_text(tr!("meshtastic-keys-hint")),
            );
            if r.lost_focus() && *draft != cfg.keys {
                edit.keys = draft.clone();
            }
        });

        if tune {
            // Only the channel moves the radio; a change of key or of the
            // switch alone must not retune it.
            cmds.extend(tune_commands(&edit));
            edit.enabled = true;
        } else if edit != cfg {
            cmds.push(Command::SetMeshtasticConfig(edit.clone()));
        }

        self.meshtastic_status_line(ui, &cfg);
        ui.separator();
        self.meshtastic_tables(ui);
    }

    fn meshtastic_status_line(&self, ui: &mut egui::Ui, cfg: &MeshtasticSettings) {
        let dim = |s: String| RichText::new(s).size(11.0).color(crate::theme::gray(160));
        let st: Option<&MeshtasticStatus> = self.meshtastic_status.as_deref();
        ui.horizontal_wrapped(|ui| match st {
            // The band list comes first: when it refuses, say so in red and say
            // nothing else, for nothing is being heard.
            Some(st) if cfg.enabled && st.refused.is_some() => {
                let why = st.refused.unwrap();
                ui.label(
                    RichText::new(rx_block_text(why, cfg.frequency_hz))
                        .color(egui::Color32::from_rgb(230, 90, 80)),
                );
            }
            Some(st) if cfg.enabled && st.running => {
                ui.label(dim(tr!("meshtastic-level", db = format!("{:.0}", st.level_dbfs))));
                ui.label(dim(tr!(
                    "meshtastic-counts",
                    pre = st.preambles,
                    ok = st.frames,
                    bad = st.crc_bad
                )));
            }
            Some(st) if cfg.enabled && !st.in_window => {
                ui.label(
                    RichText::new(tr!("meshtastic-out-of-window"))
                        .size(11.0)
                        .color(egui::Color32::from_rgb(230, 170, 60)),
                );
            }
            _ => {
                ui.label(dim(tr!("meshtastic-state-off")));
            }
        });
    }

    fn meshtastic_tables(&self, ui: &mut egui::Ui) {
        let Some(st) = self.meshtastic_status.as_deref() else {
            ui.label(RichText::new(tr!("meshtastic-none-yet")).color(crate::theme::gray(140)));
            return;
        };
        if st.nodes.is_empty() && st.log.is_empty() {
            ui.label(RichText::new(tr!("meshtastic-none-yet")).color(crate::theme::gray(140)));
            return;
        }
        let half = (ui.available_height() * 0.5).max(80.0);
        ui.label(RichText::new(tr!("meshtastic-nodes", n = st.nodes.len())).strong());
        egui::ScrollArea::vertical().id_salt("meshtastic-nodes").max_height(half).show(ui, |ui| {
            egui::Grid::new("meshtastic-node-grid").striped(true).num_columns(7).show(ui, |ui| {
                for h in [
                    tr!("meshtastic-col-id"),
                    tr!("meshtastic-col-name"),
                    tr!("meshtastic-col-heard"),
                    tr!("meshtastic-col-snr"),
                    tr!("meshtastic-col-hops"),
                    tr!("meshtastic-col-position"),
                    tr!("meshtastic-col-battery"),
                ] {
                    ui.label(RichText::new(h).size(11.0).color(crate::theme::gray(160)));
                }
                ui.end_row();
                for n in &st.nodes {
                    ui.label(node_label(n.id));
                    let name = match (n.short_name.is_empty(), n.long_name.is_empty()) {
                        (true, true) => String::new(),
                        (false, true) => n.short_name.clone(),
                        (true, false) => n.long_name.clone(),
                        (false, false) => format!("{} · {}", n.short_name, n.long_name),
                    };
                    ui.label(name);
                    ui.label(clock(n.last_heard));
                    ui.label(format!("{:.1} dB", n.snr_db));
                    ui.label(n.hops.map_or_else(String::new, |h| h.to_string()));
                    ui.label(match (n.lat_deg, n.lon_deg) {
                        (Some(la), Some(lo)) => format!("{la:.4}, {lo:.4}"),
                        _ => String::new(),
                    });
                    ui.label(match (n.battery_pct, n.voltage) {
                        (Some(b), _) => format!("{b} %"),
                        (None, Some(v)) => format!("{v:.2} V"),
                        _ => String::new(),
                    });
                    ui.end_row();
                }
            });
        });
        ui.add_space(6.0);
        ui.label(RichText::new(tr!("meshtastic-log", n = st.log.len())).strong());
        egui::ScrollArea::vertical().id_salt("meshtastic-log").show(ui, |ui| {
            for e in &st.log {
                let what = match (&e.text, e.encrypted) {
                    (Some(t), _) => t.clone(),
                    (None, true) => tr!("meshtastic-encrypted"),
                    (None, false) => port_name(e.port).to_string(),
                };
                let mqtt = if e.via_mqtt {
                    format!(" · {}", tr!("meshtastic-via-mqtt"))
                } else {
                    String::new()
                };
                ui.label(format!(
                    "{}  {} → {}  {:.1} dB  {}{}",
                    clock(e.time),
                    node_label(e.from),
                    if e.to == sdroxide_meshtastic::protocol::BROADCAST_NODE {
                        "*".to_string()
                    } else {
                        node_label(e.to)
                    },
                    e.snr_db,
                    what,
                    mqtt
                ));
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eu_868_long_fast_has_the_one_default_slot() {
        let r = lookup_region("EU_868").unwrap();
        let p = lookup_preset("LongFast").unwrap();
        assert_eq!(channel_slots(r, p), vec![869_525_000.0]);
        let cfg = with_default_channel(&MeshtasticSettings::default(), r, p);
        assert_eq!(cfg.frequency_hz, 869_525_000.0);
        assert_eq!(cfg.region, "EU_868");
        assert_eq!(cfg.preset, "LongFast");
    }

    #[test]
    fn the_us_channel_is_one_of_its_slots() {
        let r = lookup_region("US").unwrap();
        let p = lookup_preset("LongFast").unwrap();
        let slots = channel_slots(r, p);
        assert_eq!(slots.len(), 104);
        let cfg = with_default_channel(&MeshtasticSettings::default(), r, p);
        assert!(slots.contains(&cfg.frequency_hz));
        // The firmware puts LongFast on 906.875 MHz in the US.
        assert_eq!(cfg.frequency_hz, 906_875_000.0);
    }

    #[test]
    fn a_slot_of_nothing_is_the_first() {
        assert_eq!(default_slot("LongFast", 0), 0);
        assert_eq!(default_slot("LongFast", 1), 0);
    }

    #[test]
    fn listening_switches_the_lane_on_before_it_moves_the_dial() {
        let cmds = tune_commands(&MeshtasticSettings::default());
        assert_eq!(cmds.len(), 2);
        assert!(matches!(&cmds[0], Command::SetMeshtasticConfig(c) if c.enabled));
        assert!(matches!(cmds[1], Command::TuneWidebandTo(f) if f == 869_525_000.0));
    }

    #[test]
    fn a_node_is_written_as_the_apps_write_it() {
        assert_eq!(node_label(0x1a2b3c4d), "!1a2b3c4d");
        assert_eq!(node_label(0xff), "!000000ff");
        assert_eq!(clock(3661), "01:01:01");
    }
}
