//! The ALE panel: the words decoded, newest first — the type and the
//! three-character address each word carries.

use eframe::egui::{self, RichText};
use sdroxide_types::AleStatus;

use crate::app::SdroxideApp;
use crate::i18n::tr;
use crate::theme::ThemedScroll;

impl SdroxideApp {
    pub(in crate::app) fn ale_panel(
        &mut self,
        ui: &mut egui::Ui,
        _cmds: &mut Vec<sdroxide_types::Command>,
        _panel_h: f32,
    ) {
        let st: Option<AleStatus> = self.digi_status.as_ref().and_then(|s| s.ale.clone());
        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new("ALE").size(11.0).strong().color(crate::theme::CYAN()));
            if let Some(st) = &st {
                ui.label(
                    RichText::new(tr!("ale-words-count", n = st.total))
                        .size(11.0)
                        .color(crate::theme::gray(150)),
                );
                let bars = (st.level * 20.0).clamp(0.0, 20.0) as usize;
                ui.label(RichText::new("█".repeat(bars)).size(10.0).color(crate::theme::GREEN()));
            }
        });
        ui.separator();
        egui::ScrollArea::vertical().auto_shrink([false, false]).id_salt("ale-words").show_themed(
            ui,
            |ui| {
                let Some(st) = &st else {
                    ui.label(RichText::new(tr!("ale-waiting-audio")).weak());
                    return;
                };
                if st.messages.is_empty() {
                    ui.label(
                        RichText::new(tr!("ale-no-words-yet"))
                        .weak(),
                    );
                }
                for m in st.messages.iter().rev() {
                    let (_, _, _, h, mi, s) = sdroxide_types::utc_ymd_hms(m.at_unix);
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new(format!("{h:02}:{mi:02}:{s:02}"))
                                .size(10.5)
                                .color(crate::theme::gray(140)),
                        );
                        ui.label(RichText::new(&m.kind).size(11.0).strong());
                        ui.label(RichText::new(&m.address).size(11.5).color(crate::theme::GREEN()));
                    });
                }
            },
        );
    }
}
