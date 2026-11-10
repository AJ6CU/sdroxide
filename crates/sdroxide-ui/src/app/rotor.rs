//! The antenna rotator window: a compass the operator points by hand, with the
//! hardware's own reported bearing drawn against the commanded one.
//!
//! The satellite lock already steers the antenna from the SAT window; this is
//! the *manual* half — point at a bearing on the dial, at a map click, or at a
//! country, and the beam follows. It is a window rather than a panel because it
//! is a tool the operator opens to swing a beam and then leaves, and because an
//! operator who configured a rotator but never runs satellites had, before this,
//! no way to reach it at all: the only control that existed lived inside the
//! SAT window.
//!
//! It holds no aiming state the engine needs: every control pushes an ordinary
//! [`Command::PointRotator`], [`Command::StopRotator`] or
//! [`Command::SetRotatorAuthority`], and the hardware's answer comes back as
//! [`RadioEvent::RotatorStatus`]. What little it remembers — the last target it
//! asked for, the entry fields — is so the compass can show the commanded
//! bearing next to the reported one.

use crate::i18n::tr;
use eframe::egui::{self, Align2, FontId, RichText};

use sdroxide_types::{Command, RotatorAuthority, all_entities, bearing_deg};

use crate::app::SdroxideApp;
use crate::theme;

/// Where a compass bearing (degrees clockwise from north) lands on a dial
/// centred at `centre` with the given radius. North is up, east is right —
/// screen y grows downward, so north is `-y`.
/// Pure so the drag can be tested without a window.
pub(in crate::app) fn bearing_to_screen(centre: egui::Pos2, radius: f32, deg: f64) -> egui::Pos2 {
    let rad = deg.to_radians();
    egui::pos2(centre.x + radius * rad.sin() as f32, centre.y - radius * rad.cos() as f32)
}

/// The compass bearing under a pointer, degrees clockwise from north.
///
/// Always an answer: a click anywhere in the dial is a direction from its
/// centre, and clamping it to a rim the pointer is outside of would make the
/// edge of a small window unreachable.
pub(in crate::app) fn screen_to_bearing(centre: egui::Pos2, pos: egui::Pos2) -> f64 {
    let dx = (pos.x - centre.x) as f64;
    // Screen y grows downward; a bearing grows upward, so the sign flips here.
    let dy = (centre.y - pos.y) as f64;
    dx.atan2(dy).to_degrees().rem_euclid(360.0)
}

/// Normalise a heading into `0..360`.
fn norm360(deg: f64) -> f64 {
    deg.rem_euclid(360.0)
}

impl SdroxideApp {
    /// The rotator window's body. Drawn both by the detached window and, when
    /// docked, is simply not shown — the rotator is a window-only tool, so this
    /// has one caller.
    pub(in crate::app) fn rotor_body(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        let (connected, hw_az, hw_el, err) =
            self.rotator_status.clone().unwrap_or((false, 0.0, 0.0, None));
        let configured = self.rot_cfg_edit.enabled;

        if !configured {
            // A control that can silently do nothing is a bug in this fork, so
            // say exactly which half is missing rather than drawing a dial that
            // moves nothing.
            ui.add_space(10.0);
            let msg = RichText::new(tr!("rotor-no-antenna-rotator-is-configured"))
                .size(12.0)
                .strong()
                .color(theme::YELLOW());
            ui.label(msg);
            ui.add_space(6.0);
            ui.label(
                RichText::new(
                    tr!("rotor-set-one-up-in-settings-servers-a"),
                )
                .size(10.5)
                .color(theme::CYAN_DIM()),
            );
            return;
        }

        // ── status line: is the station's rotator answering? ──
        ui.horizontal(|ui| {
            let (dot, line) = if connected {
                (theme::GREEN(), tr!("rotor-connected", az = format!("{hw_az:.0}")))
            } else if let Some(e) = &err {
                (theme::ALERT(), tr!("rotor-not-connected-why", e = e))
            } else {
                (theme::CYAN_DIM(), tr!("rfe-link-off"))
            };
            let (r, _) = ui.allocate_exact_size(egui::vec2(8.0, 8.0), egui::Sense::hover());
            ui.painter().circle_filled(r.center(), 3.5, dot);
            ui.label(RichText::new(line).size(11.0).color(theme::TEXT()));
        });

        ui.add_space(6.0);
        self.rotor_compass(ui, cmds);

        // ── the readout under the dial ──
        ui.add_space(4.0);
        ui.horizontal(|ui| {
            ui.label(
                RichText::new(tr!("rotor-beam", az = format!("{hw_az:.0}"), el = format!("{hw_el:.0}")))
                    .size(12.0)
                    .strong()
                    .color(theme::CYAN()),
            );
            if let Some((taz, tel)) = self.rotor_target {
                crate::chrome::row_tail(ui, |ui| {
                    ui.label(
                        RichText::new(tr!("rotor-target", az = format!("{taz:.0}"), el = format!("{tel:.0}")))
                            .size(10.5)
                            .color(theme::YELLOW()),
                    );
                });
            }
        });

        ui.separator();

        // ── controls ──
        ui.horizontal_wrapped(|ui| {
            if crate::chrome::chip(ui, false, tr!("windows-stop"))
                .on_hover_text(tr!("rotor-halt-the-antenna-where-it-is"))
                .clicked()
            {
                cmds.push(Command::StopRotator);
                self.rotor_target = None;
            }
            if let Some(park) = self.rot_cfg_edit.park {
                if crate::chrome::chip(ui, false, tr!("rotor-park"))
                    .on_hover_text(tr!("rotor-park-hover", az = format!("{:.0}", park.0), el = format!("{:.0}", park.1)))
                    .clicked()
                {
                    cmds.push(Command::PointRotator { az: park.0, el: park.1 });
                    self.rotor_target = Some(park);
                }
            }
            if crate::chrome::chip(ui, false, tr!("topbar-auto"))
                .on_hover_text(
                    tr!("rotor-hand-the-antenna-back-to-the"),
                )
                .clicked()
            {
                cmds.push(Command::SetRotatorAuthority(RotatorAuthority::Auto));
                self.rotor_target = None;
            }
        });

        ui.add_space(6.0);

        // ── manual entry ──
        let mut entry_az = norm360(self.rotor_entry.0);
        let mut entry_el = self.rotor_entry.1;
        let mut point = false;
        ui.horizontal(|ui| {
            ui.label(RichText::new("Az").size(11.0).color(theme::CYAN_DIM()));
            ui.add(
                egui::DragValue::new(&mut entry_az)
                    .speed(1.0)
                    .range(0.0..=360.0)
                    .suffix("\u{b0}")
                    .fixed_decimals(0),
            );
            ui.label(RichText::new("El").size(11.0).color(theme::CYAN_DIM()));
            ui.add(
                egui::DragValue::new(&mut entry_el)
                    .speed(1.0)
                    .range(0.0..=90.0)
                    .suffix("\u{b0}")
                    .fixed_decimals(0),
            );
            if crate::chrome::chip(ui, false, tr!("rotor-point")).clicked() {
                point = true;
            }
        });
        self.rotor_entry = (entry_az, entry_el);
        if point {
            cmds.push(Command::PointRotator { az: entry_az, el: entry_el });
            self.rotor_target = Some((entry_az, entry_el));
        }

        ui.add_space(6.0);
        self.rotor_country_picker(ui, cmds);
    }

    /// The compass dial. Drag or click a bearing to point; the hardware's
    /// reported bearing is the cyan needle, the operator's target is the amber
    /// marker on the rim.
    fn rotor_compass(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        let (_, hw_az, _, _) = self.rotator_status.clone().unwrap_or((false, 0.0, 0.0, None));

        let side = ui.available_width().min(300.0).max(150.0);
        let (rect, resp) =
            ui.allocate_exact_size(egui::vec2(side, side), egui::Sense::click_and_drag());
        let centre = rect.center();
        let radius = side * 0.5 - 20.0;
        let painter = ui.painter_at(rect);

        // The dial itself.
        painter.circle_filled(centre, radius, theme::INPUT_BG());
        painter.circle_stroke(centre, radius, egui::Stroke::new(1.5, theme::LINE_LIT()));

        // Cardinal ticks and labels, every 30° a small tick.
        for step in 0..12 {
            let deg = (step * 30) as f64;
            let major = step % 3 == 0;
            let inner = bearing_to_screen(centre, radius - if major { 12.0 } else { 6.0 }, deg);
            let outer = bearing_to_screen(centre, radius, deg);
            painter.line_segment([inner, outer], egui::Stroke::new(1.0, theme::CYAN_DIM()));
        }
        for (deg, label) in [(0.0, "N"), (90.0, "E"), (180.0, "S"), (270.0, "W")] {
            let p = bearing_to_screen(centre, radius - 26.0, deg);
            painter.text(
                p,
                Align2::CENTER_CENTER,
                label,
                FontId::proportional(12.0),
                theme::CYAN(),
            );
        }

        // The hardware's reported bearing: a needle from the centre.
        let hw_tip = bearing_to_screen(centre, radius - 6.0, hw_az);
        painter.line_segment([centre, hw_tip], egui::Stroke::new(2.5, theme::CYAN()));
        painter.circle_filled(hw_tip, 3.0, theme::CYAN());

        // The operator's target, as an amber marker on the rim.
        if let Some((taz, _)) = self.rotor_target {
            let tp = bearing_to_screen(centre, radius - 3.0, taz);
            painter.circle_filled(tp, 4.0, theme::YELLOW());
            let tp_in = bearing_to_screen(centre, radius - 14.0, taz);
            painter.line_segment([tp_in, tp], egui::Stroke::new(1.5, theme::YELLOW()));
        }

        // A live preview of the bearing under the pointer, so a click is never
        // a guess about which way it will send the beam.
        let pointer = resp.interact_pointer_pos().or_else(|| resp.hover_pos());
        if let Some(p) = pointer {
            let az = screen_to_bearing(centre, p);
            let tip = bearing_to_screen(centre, radius - 20.0, az);
            painter.line_segment([centre, tip], egui::Stroke::new(1.0, theme::CYAN_DIM()));
            painter.text(
                bearing_to_screen(centre, radius * 0.55, az),
                Align2::CENTER_CENTER,
                format!("{az:.0}\u{b0}"),
                FontId::proportional(11.0),
                theme::TEXT(),
            );
        }

        // Pointing: a drag steers continuously, a click lands once.
        if (resp.dragged() || resp.clicked()) && resp.hovered() {
            if let Some(p) = resp.interact_pointer_pos() {
                let az = screen_to_bearing(centre, p);
                self.rotor_entry.0 = az;
                self.rotor_target = Some((az, self.rotor_entry.1));
                cmds.push(Command::PointRotator { az, el: self.rotor_entry.1 });
            }
        }
        if resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Crosshair);
        }
    }

    /// The country lookup: pick a DX entity and swing the beam onto its bearing
    /// from the station's own grid — HRD's country drop-down, in the same place.
    fn rotor_country_picker(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        let home = self.home_latlon();

        ui.horizontal(|ui| {
            ui.label(RichText::new(tr!("rotor-dx-country")).size(11.0).color(theme::CYAN_DIM()));
            let selected = self
                .rotor_country
                .clone()
                .unwrap_or_else(|| tr!("settings-amp-choose"));
            egui::ComboBox::from_id_salt("rotor-country")
                .selected_text(selected)
                .width(190.0)
                .show_ui(ui, |ui| {
                    egui::ScrollArea::vertical().max_height(260.0).show(ui, |ui| {
                        for e in all_entities() {
                            if ui.selectable_label(false, e.name).clicked() {
                                self.rotor_country = Some(e.name.to_string());
                                if let Some(home) = home {
                                    let az = bearing_deg(home, (e.lat, e.lon));
                                    self.rotor_entry.0 = az;
                                    self.rotor_target = Some((az, self.rotor_entry.1));
                                    cmds.push(Command::PointRotator { az, el: self.rotor_entry.1 });
                                }
                            }
                        }
                    });
                });
        });
        // Say why a pick did nothing rather than letting it look like a control
        // that is broken: a bearing needs somewhere to measure from.
        match (self.rotor_country.as_ref(), home) {
            (Some(_), None) => {
                ui.label(
                    RichText::new(
                        tr!("rotor-set-your-grid-locator-settings"),
                    )
                    .size(10.0)
                    .color(theme::YELLOW()),
                );
            }
            (Some(name), Some(home)) => {
                // Recompute for the caption only; the pick above already sent
                // the command.
                if let Some(e) = all_entities().iter().find(|e| e.name == name.as_str()) {
                    let az = bearing_deg(home, (e.lat, e.lon));
                    ui.label(
                        RichText::new(tr!("rotor-country-bearing", name = name, az = format!("{az:.0}")))
                            .size(10.0)
                            .color(theme::CYAN_DIM()),
                    );
                }
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use egui::pos2;

    fn close(a: f64, b: f64) -> bool {
        (a - b).abs() < 0.5
    }

    /// The compass's two directions are exact inverses, so a click lands on the
    /// bearing it looks like it lands on.
    #[test]
    fn a_bearing_round_trips_through_the_screen() {
        let centre = pos2(100.0, 100.0);
        for deg in [0.0, 45.0, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0, 359.0] {
            let p = bearing_to_screen(centre, 80.0, deg);
            assert!(
                close(screen_to_bearing(centre, p), deg),
                "{deg}° came back as {}°",
                screen_to_bearing(centre, p)
            );
        }
    }

    /// North is up and east is right — the one convention a compass cannot get
    /// wrong, and the one a sign slip flips.
    #[test]
    fn north_is_up_and_east_is_right() {
        let centre = pos2(50.0, 50.0);
        let n = bearing_to_screen(centre, 10.0, 0.0);
        assert!(n.y < centre.y && close(n.x as f64, centre.x as f64), "north is not up: {n:?}");
        let e = bearing_to_screen(centre, 10.0, 90.0);
        assert!(e.x > centre.x && close(e.y as f64, centre.y as f64), "east is not right: {e:?}");
        let s = bearing_to_screen(centre, 10.0, 180.0);
        assert!(s.y > centre.y && close(s.x as f64, centre.x as f64), "south is not down: {s:?}");
        let w = bearing_to_screen(centre, 10.0, 270.0);
        assert!(w.x < centre.x && close(w.y as f64, centre.y as f64), "west is not left: {w:?}");
    }

    /// A pointer to the north-east reads as 45°, not 315° — the sign of the
    /// screen's y axis is the thing that decides it.
    #[test]
    fn the_quadrants_read_the_right_way_round() {
        let centre = pos2(0.0, 0.0);
        assert!(close(screen_to_bearing(centre, pos2(1.0, -1.0)), 45.0));
        assert!(close(screen_to_bearing(centre, pos2(1.0, 1.0)), 135.0));
        assert!(close(screen_to_bearing(centre, pos2(-1.0, 1.0)), 225.0));
        assert!(close(screen_to_bearing(centre, pos2(-1.0, -1.0)), 315.0));
    }
}
