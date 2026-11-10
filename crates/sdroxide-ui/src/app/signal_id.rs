use eframe::egui::{self, RichText};
use crate::i18n::tr;
use sdroxide_types::{Band, SignalProfile, identify, search_profiles};

pub(in crate::app) struct SignalIdState {
    pub(in crate::app) show: bool,
    query: String,
}

impl Default for SignalIdState {
    fn default() -> Self {
        SignalIdState { show: false, query: String::new() }
    }
}

fn fmt_hz(hz: f64) -> String {
    if hz >= 1e9 {
        format!("{:.6} GHz", hz / 1e9)
    } else if hz >= 1e6 {
        format!("{:.6} MHz", hz / 1e6)
    } else if hz >= 1e3 {
        format!("{:.3} kHz", hz / 1e3)
    } else {
        format!("{hz:.0} Hz")
    }
}

impl super::SdroxideApp {
    pub(in crate::app) fn signal_id_window(&mut self, ctx: &egui::Context) {
        if !self.signal_id.show {
            return;
        }
        let show =
            self.tool_window(ctx, "signal-id", &tr!("signalid-title"), [620.0, 520.0], true, |me, ui| {
                me.signal_id_body(ui);
            });
        self.signal_id.show = show;
    }

    fn signal_id_body(&mut self, ui: &mut egui::Ui) {
        let rx = &self.state.rx[0];
        let dial_hz = self.state.rx_freq_hz();
        let freq_hz = self.on_air_freq_hz();
        let mode = rx.mode;
        let bw_hz = f64::from((rx.filter_hi - rx.filter_lo).abs()).max(1.0);
        let band = Band::containing(dial_hz);

        ui.horizontal_wrapped(|ui| {
            ui.label(RichText::new(tr!("signalid-tuned-now")).size(11.0).color(crate::theme::gray(150)));
            ui.label(
                RichText::new(fmt_hz(freq_hz)).monospace().strong().color(crate::theme::CYAN()),
            );
            ui.label(RichText::new(mode.label()).strong());
            ui.label(
                RichText::new(tr!("signalid-passband", bw = fmt_hz(bw_hz)))
                    .size(11.0)
                    .color(crate::theme::gray(150)),
            );
            ui.label(
                RichText::new(tr!("signalid-band", band = band.label()))
                    .size(11.0)
                    .color(crate::theme::gray(150)),
            );
        });
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new(tr!("signalid-find")).size(11.0).color(crate::theme::gray(150)));
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut self.signal_id.query)
                    .desired_width(220.0)
                    .hint_text(tr!("signalid-find-hint")),
            );
            if crate::chrome::chip(ui, false, tr!("decodes-clear")).clicked() {
                self.signal_id.query.clear();
            }
        });
        ui.add_space(4.0);
        ui.separator();

        let query = self.signal_id.query.trim().to_string();
        let hits = if query.is_empty() {
            identify(freq_hz, Some(band), mode, Some(bw_hz))
        } else {
            search_profiles(&query)
        };

        ui.label(
            RichText::new(if query.is_empty() {
                tr!("signalid-candidates", n = hits.len())
            } else {
                tr!("signalid-matches", n = hits.len(), query = query)
            })
            .size(10.5)
            .color(crate::theme::gray(150)),
        );

        egui::ScrollArea::vertical().auto_shrink([false, false]).id_salt("signal-id-list").show(
            ui,
            |ui| {
                if hits.is_empty() {
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new(if query.is_empty() {
                            tr!("signalid-nothing-fits")
                        } else {
                            tr!("signalid-no-match")
                        })
                        .color(crate::theme::gray(160)),
                    );
                    return;
                }
                for p in hits {
                    profile_row(ui, p);
                }
            },
        );
    }
}

fn profile_row(ui: &mut egui::Ui, p: &SignalProfile) {
    egui::Frame::new()
        .fill(crate::theme::ROW_BG())
        .stroke(egui::Stroke::new(1.0, crate::theme::LINE()))
        .inner_margin(egui::Margin::symmetric(9, 7))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                ui.label(RichText::new(p.name).strong().color(crate::theme::CYAN()));
                ui.label(RichText::new(p.family.label()).size(10.5).color(crate::theme::gray(150)));
                ui.label(RichText::new(p.modulation).size(10.5).color(crate::theme::gray(170)));
                ui.label(
                    RichText::new(p.bandwidth_text()).size(10.5).color(crate::theme::gray(150)),
                );
                if let Some(url) = p.wiki_url()
                    && crate::chrome::chip(ui, false, "sigidwiki")
                        .on_hover_text(tr!("signalid-open-url", url = url))
                        .clicked()
                {
                    // `ctx.open_url` is dropped by eframe on the desktop, so
                    // this link has never opened anything in the native build.
                    if !crate::download::open_external(&url) {
                        tracing::warn!(%url, "no handler for the sigidwiki page");
                    }
                }
            });
            ui.label(RichText::new(p.summary).size(11.0));
        });
    ui.add_space(4.0);
}

#[cfg(test)]
mod tests {
    use super::fmt_hz;

    #[test]
    fn frequencies_read_in_a_human_unit() {
        assert_eq!(fmt_hz(27_185_000.0), "27.185000 MHz");
        assert_eq!(fmt_hz(518_000.0), "518.000 kHz");
        assert_eq!(fmt_hz(1_090_000_000.0), "1.090000 GHz");
        assert_eq!(fmt_hz(700.0), "700 Hz");
    }
}
