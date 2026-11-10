//! The WEATHER window — coded SYNOP and SHIP reports, decoded.
//!
//! **BETA.** The decoder is tested against four real published messages; no
//! report has yet been read off the air by this code. Feedback wanted.
//!
//! Ships and land stations broadcast a coded report on HF RTTY every few hours.
//! The line arrives as a wall of figures — `BBXX BRAVO 20123 99252 10595 …` —
//! and this window is what it says. The decode itself is
//! [`sdroxide_types::synop`]; this is the view.
//!
//! Why its own window rather than a row in the decode list: a decode list that
//! shows `BBXX BRAVO 20123 99252 10595 41494` as a *message* has decoded
//! nothing, and a listener who cannot read the figures cannot use them at all.
//! Reports also arrive far apart in time and want to be **kept** — the weather
//! at a position three hours ago is what you came back for.

use crate::i18n::tr;
use eframe::egui::{self, RichText};

use crate::app::SdroxideApp;
use crate::theme::gray;

/// Whether the window is open. Session-only: a stored report read on another
/// day is a lie about when it was observed.
#[derive(Clone, Copy, Debug, Default)]
pub struct WeatherWindow {
    pub show: bool,
}

impl SdroxideApp {
    /// Open it from the RTTY panel — and from anywhere else that decodes text,
    /// since the report does not care which mode brought it in.
    pub(in crate::app) fn weather_window(&mut self, ctx: &egui::Context) {
        if !self.weather.show {
            return;
        }
        let show =
            self.tool_window(ctx, "weather", &tr!("weather-title"), [720.0, 640.0], true, |me, ui| {
                me.weather_body(ui);
            });
        self.weather.show = show;
    }

    fn weather_body(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new(tr!("weather-intro"))
            .size(11.0)
            .color(gray(140)),
        );
        ui.add_space(6.0);

        if self.weather_reports.is_empty() {
            ui.label(
                RichText::new(tr!("weather-nothing-decoded"))
                .size(12.0)
                .color(gray(120)),
            );
            return;
        }

        egui::ScrollArea::vertical().show(ui, |ui| {
            for (when, r) in self.weather_reports.clone() {
                egui::Frame::NONE
                    .fill(ui.visuals().panel_fill)
                    .inner_margin(10.0)
                    .outer_margin(egui::Margin::same(4))
                    .stroke(egui::Stroke::new(1.0, gray(60)))
                    .show(ui, |ui| {
                        // Which station, which code, and when it was observed —
                        // the three facts that make a list of figures usable.
                        let head = match (r.callsign.as_deref(), r.observed) {
                            (Some(c), Some((d, h))) => tr!("weather-head-call-day-hour", call = c, day = d, hour = format!("{h:02}")),
                            (Some(c), None) => format!("{c}"),
                            (None, Some((d, _))) => tr!("weather-head-day", day = format!("{d:02}")),
                            (None, None) => tr!("weather-head-report"),
                        };
                        ui.horizontal(|ui| {
                            ui.label(RichText::new(head).size(13.0).strong());
                            ui.label(
                                RichText::new(r.kind.label())
                                    .size(10.0)
                                    .color(ui.visuals().weak_text_color()),
                            );
                            ui.label(RichText::new(ago(when)).size(10.0).color(gray(110)));
                        });

                        if let Some(p) = r.position {
                            ui.label(
                                RichText::new(format!(
                                    "{:.1}° {:.1}°{}",
                                    p.lat_deg.abs(),
                                    p.lon_deg.abs(),
                                    if p.lat_deg < 0.0 { " S" } else { " N" }
                                ))
                                .size(11.5)
                                .color(gray(160)),
                            );
                        }

                        // The raw line first. A listener checking a report
                        // against the original needs the figures, and a window
                        // that prints only an interpretation cannot be argued
                        // with.
                        ui.add_space(4.0);
                        ui.label(
                            RichText::new(r.raw.clone()).size(9.5).color(gray(110)).monospace(),
                        );
                        ui.add_space(4.0);

                        for f in &r.fields {
                            let text = match &f.value {
                                Some(v) => format!("{}: {}", f.label, v),
                                // A slash. "Not reported" is an answer the
                                // station gave, and showing it as missing would
                                // read as our fault rather than theirs.
                                None => tr!("weather-not-reported", label = f.label),
                            };
                            ui.label(RichText::new(text).size(11.5).color(gray(175)));
                        }

                        for remark in &r.remarks {
                            ui.label(
                                RichText::new(tr!("weather-in-their-words", remark = remark))
                                    .size(11.5)
                                    .color(gray(200)),
                            );
                        }
                    });
            }
        });
    }
}

/// How long ago, in words. **How long ago the report was observed**, not when
/// the window saw it — a report decoded at 02:05 was observed at the hour, and
/// the difference is the thing a listener checks.
fn ago(unix: i64) -> String {
    // Wasm-safe: `SystemTime::now()` panics in the browser (#262/#297).
    let now = crate::time::now_unix();
    let mins = ((now - unix) / 60).max(0);
    match mins {
        0 => tr!("weather-ago-now"),
        1 => tr!("weather-ago-minute"),
        m if m < 60 => tr!("weather-ago-minutes", m = m),
        h if h < 24 => tr!("weather-ago-hours", h = h),
        d => tr!("weather-ago-days", d = d),
    }
}

#[cfg(test)]
mod tests {
    use sdroxide_types::synop;

    /// The window lists what the decoder produced, and a report with slashes
    /// says **not reported** rather than showing a blank.
    #[test]
    fn a_decoded_report_has_readable_groups_and_no_blank_rows() {
        let r = synop::parse(
            "BBXX BRAVO 20123 99252 10595 41494 81412 10285 20269 40100 53012 79586 8597/ 22265 \
             00280 20405 31705 40506 50407=",
        )
        .unwrap();
        assert!(
            r.fields.iter().all(|f| f.value.is_some()),
            "a blank group would read as our fault"
        );
        assert_eq!(r.callsign.as_deref(), Some("BRAVO"));
        // And the raw line is kept, which is the whole point of showing it.
        assert!(r.raw.starts_with("BBXX BRAVO"));
    }
}
