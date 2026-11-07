//! The VOR view of the AIRCRAFT window: the beacon the radio is tuned to, its
//! radial on a compass, its Morse ident and how well its two 30 Hz signals are
//! heard, with the charted VORs nearest the station to tune to.
//!
//! Like SONDE and Cards it is a view, not a radio mode. The decoder runs
//! whenever the dial is inside 108-118 MHz (see `sdroxide_vor`); this only
//! reads [`sdroxide_types::VorStatus`] and offers the dial a way there.

use crate::i18n::tr;
use crate::{airdata, theme};
use eframe::egui::{self, Align2, Color32, FontId, RichText, Sense, Stroke, vec2};
use sdroxide_types::{Command, Mode, RxId, VOR_BAND_HZ, VorUnavailable};

/// How many charted VORs the tuning list offers.
const NEARBY: usize = 6;

/// A VOR the list can tune to.
struct Charted {
    ident: String,
    name: String,
    freq_hz: f64,
    km: Option<f64>,
}

/// Whether a navaid is one a VOR receiver hears: the VOR itself, with or
/// without DME or a TACAN beside it.
fn is_vor(k: airdata::NavaidKind) -> bool {
    matches!(
        k,
        airdata::NavaidKind::Vor | airdata::NavaidKind::VorDme | airdata::NavaidKind::Vortac
    )
}

/// The charted VORs on this frequency, nearest `home` first.
fn charted_on(dial_hz: f64, home: Option<(f64, f64)>) -> Vec<Charted> {
    let mut out: Vec<Charted> = airdata::navaids()
        .iter()
        .filter(|n| is_vor(n.kind) && n.freq_khz > 0)
        .filter(|n| (n.freq_khz as f64 * 1e3 - dial_hz).abs() < 5_000.0)
        .map(|n| Charted {
            ident: n.ident.clone(),
            name: n.name.clone(),
            freq_hz: n.freq_khz as f64 * 1e3,
            km: home.map(|h| sdroxide_types::distance_km(h, (n.lat, n.lon))),
        })
        .collect();
    out.sort_by(|a, b| a.km.partial_cmp(&b.km).unwrap_or(std::cmp::Ordering::Equal));
    out
}

/// The VORs nearest `home`, as the tuning list shows them.
fn nearest(home: (f64, f64)) -> Vec<Charted> {
    let mut all: Vec<Charted> = airdata::navaids()
        .iter()
        .filter(|n| is_vor(n.kind) && n.freq_khz > 0)
        .map(|n| Charted {
            ident: n.ident.clone(),
            name: n.name.clone(),
            freq_hz: n.freq_khz as f64 * 1e3,
            km: Some(sdroxide_types::distance_km(home, (n.lat, n.lon))),
        })
        .collect();
    all.sort_by(|a, b| a.km.partial_cmp(&b.km).unwrap_or(std::cmp::Ordering::Equal));
    all.truncate(NEARBY);
    all
}

/// The commands that bring the radio to a VOR: its channel, in AM, so the
/// ident can be heard as well as read.
pub(in crate::app) fn tune_commands(vfo: sdroxide_types::Vfo, hz: f64) -> Vec<Command> {
    vec![Command::SetVfo { vfo, hz }, Command::SetMode { rx: RxId::Main, mode: Mode::Am }]
}

impl super::SdroxideApp {
    pub(in crate::app) fn aircraft_vor_pane(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        h: f32,
    ) {
        let st = self.vor_status.as_deref().cloned().unwrap_or_default();
        let dial = self.state.rx_freq_hz();
        let in_band = (VOR_BAND_HZ.0..=VOR_BAND_HZ.1).contains(&dial);
        let grid = self.digi_cfg_edit.my_grid.trim().to_owned();
        let home = (!grid.is_empty()).then(|| sdroxide_types::grid_to_latlon(&grid)).flatten();
        let charted = if in_band { charted_on(dial, home) } else { Vec::new() };

        egui::ScrollArea::vertical().id_salt("aircraft_vor").auto_shrink([false, false]).show(
            ui,
            |ui| {
                ui.set_min_height(h - 8.0);
                // Where the dial is, and whether anything is being read.
                ui.horizontal_wrapped(|ui| {
                    ui.label(
                        RichText::new(format!("{:.3} MHz", dial / 1e6))
                            .size(13.0)
                            .strong()
                            .color(theme::CYAN()),
                    );
                    if let Some(why) = st.unavailable.filter(|_| in_band) {
                        let text = match why {
                            VorUnavailable::AudioFront => tr!("vor-audio-front"),
                            VorUnavailable::OutsideWindow => tr!("vor-outside-window"),
                        };
                        ui.label(RichText::new(text).size(11.0).color(theme::HAZARD()));
                    } else if !in_band {
                        ui.label(RichText::new(tr!("vor-not-in-band")).size(11.0).weak());
                    } else if st.running && st.locked {
                        ui.label(
                            RichText::new(tr!("vor-locked"))
                                .size(11.0)
                                .strong()
                                .color(theme::GREEN()),
                        );
                    } else if st.running {
                        ui.label(
                            RichText::new(tr!("vor-no-lock"))
                                .size(11.0)
                                .strong()
                                .color(theme::YELLOW()),
                        )
                        .on_hover_text(tr!("vor-no-lock-hover"));
                    }
                });

                // The ident: what was heard against what the chart says is on
                // this channel.
                if in_band && st.running {
                    ui.horizontal_wrapped(|ui| {
                        match st.ident.as_deref() {
                            Some(id) => {
                                ui.label(
                                    RichText::new(tr!("vor-ident-heard", ident = id))
                                        .size(12.0)
                                        .strong(),
                                );
                            }
                            None => {
                                ui.label(RichText::new(tr!("vor-ident-waiting")).size(11.0).weak());
                            }
                        }
                        if let Some(c) = charted.first() {
                            let same = st.ident.as_deref() == Some(c.ident.as_str());
                            let text = tr!("vor-charted", ident = c.ident.as_str(), name = c.name.as_str());
                            let ink = if same { theme::GREEN() } else { theme::gray(150) };
                            ui.label(RichText::new(text).size(11.0).color(ink));
                        }
                    });
                }

                ui.add_space(4.0);
                let side = (ui.available_width().min(h - 150.0)).clamp(140.0, 300.0);
                compass(ui, side, in_band && st.running && st.locked, st.radial_deg);

                if in_band && st.running {
                    ui.add_space(2.0);
                    ui.label(
                        RichText::new(tr!(
                            "vor-var-depth",
                            pct = format!("{:.0}", st.var_depth * 100.0)
                        ))
                        .size(11.0)
                        .weak(),
                    );
                    ui.label(
                        RichText::new(tr!(
                            "vor-ref-depth",
                            pct = format!("{:.0}", st.ref_depth * 100.0),
                            dev = format!("{:.0}", st.deviation_hz)
                        ))
                        .size(11.0)
                        .weak(),
                    );
                    ui.label(RichText::new(tr!("vor-caveat")).size(10.0).weak());
                }

                // Somewhere to tune to.
                ui.add_space(6.0);
                crate::chrome::menu_caption(ui, &tr!("vor-nearby"));
                match home {
                    None => {
                        ui.label(RichText::new(tr!("vor-no-grid")).size(11.0).weak());
                    }
                    Some(home) => {
                        for c in nearest(home) {
                            let lit = (c.freq_hz - dial).abs() < 5_000.0;
                            let label = format!(
                                "{:<4} {:.2} MHz  {}  {:.0} km",
                                c.ident,
                                c.freq_hz / 1e6,
                                c.name,
                                c.km.unwrap_or(0.0)
                            );
                            if crate::chrome::chip(ui, lit, label)
                                .on_hover_text(tr!("vor-tune-hover"))
                                .clicked()
                            {
                                cmds.extend(tune_commands(self.state.active_vfo, c.freq_hz));
                            }
                        }
                    }
                }
            },
        );
    }
}

/// A compass with the beacon at its centre and, when the radial is known, a dot
/// on the rim where the receiver is.
fn compass(ui: &mut egui::Ui, side: f32, show: bool, radial_deg: f32) {
    let (resp, p) = ui.allocate_painter(vec2(side, side), Sense::hover());
    let c = resp.rect.center();
    let r = side / 2.0 - 16.0;
    let ink = theme::gray(150);
    p.circle_stroke(c, r, Stroke::new(1.2, ink));
    for k in 0..36 {
        let a = (k as f32 * 10.0).to_radians();
        let (s, co) = a.sin_cos();
        let len = if k % 3 == 0 { 9.0 } else { 4.5 };
        p.line_segment(
            [c + vec2(s, -co) * (r - len), c + vec2(s, -co) * r],
            Stroke::new(if k % 3 == 0 { 1.2 } else { 0.8 }, ink),
        );
    }
    for (deg, label) in [(0.0_f32, "N"), (90.0, "E"), (180.0, "S"), (270.0, "W")] {
        let a = deg.to_radians();
        p.text(
            c + vec2(a.sin(), -a.cos()) * (r + 10.0),
            Align2::CENTER_CENTER,
            label,
            FontId::proportional(11.0),
            ink,
        );
    }
    p.circle_filled(c, 3.0, ink);
    if show {
        let a = radial_deg.to_radians();
        let tip = c + vec2(a.sin(), -a.cos()) * r;
        p.line_segment([c, tip], Stroke::new(2.0, theme::GREEN()));
        p.circle_filled(tip, 5.5, theme::GREEN());
        p.text(
            c + vec2(0.0, r * 0.45),
            Align2::CENTER_CENTER,
            format!("{:03.0}°", radial_deg),
            FontId::proportional(22.0),
            Color32::WHITE,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use airdata::NavaidKind;

    #[test]
    fn only_the_vor_family_is_a_vor() {
        for k in [NavaidKind::Vor, NavaidKind::VorDme, NavaidKind::Vortac] {
            assert!(is_vor(k), "{k:?}");
        }
        for k in [NavaidKind::Ndb, NavaidKind::Tacan, NavaidKind::Dme, NavaidKind::NdbDme] {
            assert!(!is_vor(k), "{k:?}");
        }
    }

    #[test]
    fn tuning_a_vor_sets_the_channel_then_am() {
        let cmds = tune_commands(sdroxide_types::Vfo::A, 108_400_000.0);
        assert_eq!(cmds.len(), 2);
        assert!(matches!(cmds[0], Command::SetVfo { hz, .. } if hz == 108_400_000.0));
        assert!(matches!(cmds[1], Command::SetMode { rx: RxId::Main, mode: Mode::Am }));
    }

    /// Schiphol's VOR/DME is charted on 108.400 MHz (see `airdata`'s own tests):
    /// found from a dial a couple of kilohertz off, nearest to home first.
    #[test]
    fn a_charted_channel_is_found_nearest_first() {
        let schiphol_side = Some((52.37, 4.90));
        let found = charted_on(108_401_500.0, schiphol_side);
        assert!(!found.is_empty());
        assert_eq!(found[0].ident, "SPL");
        assert!(found.iter().all(|c| (c.freq_hz - 108_400_000.0).abs() < 1.0));
        let km: Vec<f64> = found.iter().filter_map(|c| c.km).collect();
        assert!(km.windows(2).all(|w| w[0] <= w[1]), "{km:?}");
        assert!(km[0] < 30.0, "{km:?}");
    }

    #[test]
    fn without_a_home_the_distance_is_unknown() {
        let found = charted_on(108_400_000.0, None);
        assert!(found.iter().any(|c| c.ident == "SPL"));
        assert!(found.iter().all(|c| c.km.is_none()));
    }

    #[test]
    fn a_channel_with_no_vor_gives_nothing() {
        // An ILS localiser channel (108.10) and a frequency outside the band.
        assert!(charted_on(100_000_000.0, None).is_empty());
        assert!(charted_on(1_090_000_000.0, None).is_empty());
    }
}
