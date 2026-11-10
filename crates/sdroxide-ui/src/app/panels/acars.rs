//! The ACARS panel: the airline datalink frames decoded, newest first.
//!
//! One list, because that is the whole shape of the mode: a frame is a single
//! self-contained burst of a second or two, complete in itself, and there is no
//! session to reassemble and nothing being copied *now* to pin beside it. What
//! a listener watches is the channel's level between bursts and the addresses
//! and text as they arrive.
//!
//! The channel row is the one control. ACARS lives on a handful of shared
//! airband frequencies and an operator moves between them by hand all evening,
//! so the published ones are chips rather than something to type into the dial.

use crate::i18n::tr;
use eframe::egui::{self, RichText};
use sdroxide_types::{AcarsStatus, Command, Vfo};

use crate::app::SdroxideApp;
use crate::theme;

/// The airline channels ACARS is commonly found on. Widely published: 131.550
/// is the primary almost everywhere, Europe works 131.525, 131.725 and 131.825
/// (the three acarsdec's own examples listen to), and the rest are North
/// American.
const CHANNELS: &[(u32, &str)] = &[
    (129_125_000, "129.125"),
    (130_025_000, "130.025"),
    (130_450_000, "130.450"),
    (131_125_000, "131.125"),
    (131_525_000, "131.525"),
    (131_550_000, "131.550"),
    (131_725_000, "131.725"),
    (131_825_000, "131.825"),
    (136_700_000, "136.700"),
];

/// The header row: the level, the frame counts, CLEAR RX and SAVE. `status`
/// is the app's `digi_status`, which SAVE writes from.
///
/// Wrapped: on a phone the row is wider than the screen, and a plain one ran
/// SAVE off its edge.
pub(super) fn acars_header(
    ui: &mut egui::Ui,
    st: &AcarsStatus,
    status: Option<&sdroxide_types::DigiStatus>,
    cmds: &mut Vec<Command>,
) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(tr!("acars-acars")).strong().color(theme::CYAN()));
        // The audio level: ACARS speaks in bursts, and the meter is how an
        // operator tells "nothing on this channel" from "nothing decoded".
        ui.add(
            egui::ProgressBar::new(st.level.clamp(0.0, 1.0))
                .desired_width(70.0)
                .fill(theme::CYAN_DIM()),
        )
        .on_hover_text(tr!("acars-audio-decoder-s-passband"));
        ui.label(
            RichText::new(tr!("acars-frames-bad", frames = st.frames, bad = st.bad))
                .size(10.0)
                .color(if st.bad > 0 { theme::YELLOW() } else { theme::CYAN_DIM() }),
        )
        .on_hover_text(tr!("acars-blocks-framed-blocks-whose-check"));
        super::clear_rx_chip_at(ui, cmds, true);
        super::save_rx_chip_for(ui, status);
    });
}

/// The published channel the dial is on, if it is on one: the chip to light.
fn lit_channel(dial_hz: f64) -> Option<u32> {
    CHANNELS.iter().find(|(hz, _)| (dial_hz - *hz as f64).abs() < 500.0).map(|(hz, _)| *hz)
}

/// Whether the radio can receive `hz`. A receiver that states no range is
/// believed, as everywhere else.
fn in_range(ranges: &[(f64, f64)], hz: f64) -> bool {
    ranges.is_empty() || ranges.iter().any(|&(lo, hi)| hz >= lo && hz <= hi)
}

/// The CHANNEL row. A chip tunes **the VFO in use** — the engine only carries a
/// dial move into the receiver for the active VFO, so a chip wired to VFO A did
/// nothing audible whenever B was the one being listened on. The chip of the
/// channel the dial is really on is lit, and one the radio cannot reach is
/// greyed and says so rather than doing nothing.
fn acars_channel_row(
    ui: &mut egui::Ui,
    active: Vfo,
    dial_hz: f64,
    ranges: &[(f64, f64)],
    cmds: &mut Vec<Command>,
) {
    let lit = lit_channel(dial_hz);
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(tr!("acars-channel")).size(10.0).weak());
        for (hz, label) in CHANNELS {
            let reachable = in_range(ranges, *hz as f64);
            let r = ui.add_enabled_ui(reachable, |ui| {
                crate::chrome::chip(ui, lit == Some(*hz), RichText::new(*label).size(10.0))
            });
            let r = if reachable {
                r.inner.on_hover_text(tr!("acars-tune-dial-here"))
            } else {
                r.response.on_hover_text(tr!("acars-channel-out-of-range"))
            };
            if reachable && r.clicked() {
                cmds.push(Command::SetVfo { vfo: active, hz: *hz as f64 });
            }
        }
    });
}

impl SdroxideApp {
    pub(in crate::app) fn acars_panel(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        panel_h: f32,
    ) {
        let st: Option<AcarsStatus> = self.digi_status.as_ref().and_then(|s| s.acars.clone());
        let Some(st) = st else {
            ui.label(RichText::new(tr!("acars-starting-acars-receiver")).weak());
            return;
        };

        acars_header(ui, &st, self.digi_status.as_ref(), cmds);

        let ranges = self.caps.as_ref().map(|c| c.freq_ranges_rx.clone()).unwrap_or_default();
        acars_channel_row(ui, self.state.active_vfo, self.state.active_freq_hz(), &ranges, cmds);

        ui.separator();

        egui::ScrollArea::vertical()
            .id_salt("acars-frames")
            .max_height((panel_h - 60.0).max(60.0))
            .show(ui, |ui| {
                if st.messages.is_empty() {
                    ui.label(RichText::new(tr!("acars-nothing-yet-acars-quiet-between")).weak());
                    return;
                }
                for m in st.messages.iter().rev() {
                    ui.horizontal(|ui| {
                        let (_, _, _, h, mi, s) = sdroxide_types::utc_ymd_hms(m.at);
                        ui.label(
                            RichText::new(format!("{h:02}:{mi:02}:{s:02}"))
                                .monospace()
                                .size(10.0)
                                .color(theme::CYAN_DIM()),
                        );
                        ui.label(
                            RichText::new(&m.address).monospace().strong().color(theme::CYAN()),
                        );
                        if !m.mode.is_empty() {
                            ui.label(
                                RichText::new(tr!("acars-mode", mode = m.mode)).size(9.5).weak(),
                            );
                        }
                        if !m.label.is_empty() {
                            ui.label(RichText::new(&m.label).monospace().size(10.0).weak());
                        }
                        if !m.ack.is_empty() {
                            ui.label(RichText::new(&m.ack).monospace().size(9.5).weak());
                        }
                        if !m.block_id.is_empty() {
                            ui.label(RichText::new(&m.block_id).size(9.5).weak());
                        }
                        if !m.crc_ok {
                            ui.label(
                                RichText::new(tr!("acars-check-failed"))
                                    .size(9.5)
                                    .color(theme::YELLOW()),
                            )
                            .on_hover_text(tr!("acars-block-check-sequence-did-not"));
                        }
                    });
                    if m.text.trim().is_empty() {
                        ui.label(RichText::new(tr!("acars-no-text")).size(10.0).weak());
                    } else {
                        // Unwrapped: an ACARS message is columns and line
                        // breaks as the sender wrote them.
                        ui.label(RichText::new(&m.text).monospace().size(11.5));
                    }
                    ui.add_space(3.0);
                }
            });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Run one frame of the channel row and return its commands and where each
    /// chip is.
    fn frame(
        ctx: &egui::Context,
        active: Vfo,
        dial_hz: f64,
        ranges: &[(f64, f64)],
        events: Vec<egui::Event>,
    ) -> (Vec<Command>, Vec<egui::Rect>) {
        let mut cmds = Vec::new();
        let mut rects = Vec::new();
        let raw = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(900.0, 120.0),
            )),
            events,
            ..Default::default()
        };
        ctx.run_ui(raw, |ui| {
            let r = ui.scope(|ui| acars_channel_row(ui, active, dial_hz, ranges, &mut cmds));
            rects.push(r.response.rect);
        })
        .drop_without_applying_deltas();
        (cmds, rects)
    }

    /// Click the `n`th chip and return what the row sent.
    fn click_chip(active: Vfo, dial_hz: f64, ranges: &[(f64, f64)], n: usize) -> Vec<Command> {
        let ctx = egui::Context::default();
        crate::theme::apply(&ctx);
        // A chip's place is not published by the row, so find it by sweeping
        // the row's own height and taking the n-th distinct click that sends.
        let (_, rows) = frame(&ctx, active, dial_hz, ranges, vec![]);
        let row = rows[0];
        let mut found = 0;
        let mut x = row.left() + 2.0;
        let y = row.center().y;
        let mut last_sent: Option<Command> = None;
        while x < row.right() {
            let at = egui::pos2(x, y);
            let press = |pressed| egui::Event::PointerButton {
                pos: at,
                button: egui::PointerButton::Primary,
                pressed,
                modifiers: Default::default(),
            };
            frame(&ctx, active, dial_hz, ranges, vec![egui::Event::PointerMoved(at)]);
            frame(&ctx, active, dial_hz, ranges, vec![egui::Event::PointerMoved(at), press(true)]);
            let (cmds, _) = frame(&ctx, active, dial_hz, ranges, vec![press(false)]);
            if let Some(c) = cmds.first() {
                if last_sent.as_ref() != Some(c) {
                    if found == n {
                        return cmds;
                    }
                    found += 1;
                    last_sent = Some(c.clone());
                }
            }
            x += 4.0;
        }
        Vec::new()
    }

    #[test]
    fn a_channel_chip_tunes_the_vfo_in_use_whichever_it_is() {
        for active in [Vfo::A, Vfo::B] {
            for (n, (hz, _)) in CHANNELS.iter().enumerate() {
                let sent = click_chip(active, 100_000_000.0, &[], n);
                assert_eq!(
                    sent,
                    vec![Command::SetVfo { vfo: active, hz: *hz as f64 }],
                    "chip {n} with {active:?} in use"
                );
            }
        }
    }

    #[test]
    fn the_chip_of_the_channel_the_dial_is_on_is_the_only_one_lit() {
        assert_eq!(lit_channel(131_725_000.0), Some(131_725_000));
        assert_eq!(lit_channel(131_725_300.0), Some(131_725_000), "a few hundred Hz off");
        assert_eq!(lit_channel(131_726_000.0), None);
        assert_eq!(lit_channel(100_000_000.0), None);
        // Every published channel lights exactly itself.
        for (hz, _) in CHANNELS {
            let lit: Vec<u32> = CHANNELS
                .iter()
                .map(|(h, _)| *h)
                .filter(|h| lit_channel(*hz as f64) == Some(*h))
                .collect();
            assert_eq!(lit, vec![*hz]);
        }
    }

    #[test]
    fn a_channel_the_radio_cannot_reach_sends_nothing_and_is_not_silent() {
        // A receiver that stops at 130 MHz reaches the first three channels only.
        let ranges = [(0.0, 130_100_000.0)];
        assert!(in_range(&ranges, 129_125_000.0));
        assert!(!in_range(&ranges, 131_125_000.0));
        assert!(in_range(&[], 136_700_000.0), "no stated range is believed");
        let last = CHANNELS.len() - 1;
        assert_eq!(
            click_chip(Vfo::A, 100_000_000.0, &ranges, 0),
            vec![Command::SetVfo { vfo: Vfo::A, hz: CHANNELS[0].0 as f64 }]
        );
        // The chips beyond the range never send, so the sweep finds only the
        // reachable ones.
        assert_eq!(click_chip(Vfo::A, 100_000_000.0, &ranges, last), Vec::new());
        assert!(!tr!("acars-channel-out-of-range").is_empty());
    }
}
