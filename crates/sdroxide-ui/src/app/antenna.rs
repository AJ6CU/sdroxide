//! The antenna calculator — a frequency in, dimensions and SWR out — for the
//! shapes the closed-form calculator knows (dipole, inverted-V, vertical,
//! ground plane, loop, Yagi), an on-demand method-of-moments solve of the
//! presets, and a Yagi optimizer. It lives in the EXTRAS launcher.

use crate::i18n::tr;
use eframe::egui;

use antenna_model::obstacle::{Obstacle, attenuation_db};
use antenna_model::optimize::Objective;

/// One preset's solved impedance and gain.
pub(in crate::app) struct SolvedRow {
    pub(in crate::app) name: String,
    pub(in crate::app) r: f64,
    pub(in crate::app) x: f64,
    pub(in crate::app) swr: f64,
    pub(in crate::app) gain: f64,
    /// The takeoff angle — the elevation of the peak gain, above the horizon.
    pub(in crate::app) takeoff: f64,
}

/// The optimizer's objective rendered for a person.
fn objective_value(obj: Objective, score: f64) -> String {
    match obj {
        Objective::Gain => format!("{:.1} dBi", -score),
        Objective::Swr => format!("{:.2}:1", score),
        Objective::Fb => format!("{:.1} dB", -score),
        Objective::Jx => format!("{:.1} Ω", score),
    }
}

/// Draw an azimuth cut (azimuth in degrees, gain in dBi) as a polar field
/// pattern: filled, colored by gain like the ADS-B range display.
fn pattern_plot(ui: &mut egui::Ui, name: &str, cut: &[(f64, f64)]) {
    let size = egui::vec2(240.0, 240.0);
    let (response, painter) = ui.allocate_painter(size, egui::Sense::hover());
    let rect = response.rect;
    let center = rect.center();
    let radius = rect.width().min(rect.height()) / 2.0 - 10.0;
    // Draw concentric rings
    for r in [0.25, 0.5, 0.75, 1.0].iter() {
        painter.circle_stroke(
            center,
            radius * (*r as f32),
            egui::Stroke::new(0.5, egui::Color32::from_gray(40)),
        );
    }
    // Draw antenna symbol in the middle
    painter.line_segment(
        [center + egui::vec2(-5.0, 0.0), center + egui::vec2(5.0, 0.0)],
        egui::Stroke::new(2.0, egui::Color32::from_gray(220)),
    );
    painter.circle_filled(center, 1.5, egui::Color32::from_gray(220));
    let max_g = cut.iter().map(|&(_, g)| g).fold(f64::NEG_INFINITY, f64::max);
    // Draw filled pattern — radius is the relative field (10^((G-Gmax)/20)),
    // so nulls land near zero and the peak at the rim.
    let n = cut.len();
    for i in 0..n {
        let (az1, g1) = cut[i];
        let (az2, g2) = cut[(i + 1) % n];
        let a1 = az1.to_radians();
        let a2 = az2.to_radians();
        let r1 = 10f64.powf((g1 - max_g) / 20.0).clamp(0.0, 1.0);
        let r2 = 10f64.powf((g2 - max_g) / 20.0).clamp(0.0, 1.0);
        let c1 = gain_color(r1 as f32);
        let p0 = center;
        let p1 = center
            + egui::vec2(
                (a1.sin() * radius as f64 * r1) as f32,
                (-(a1.cos()) * radius as f64 * r1) as f32,
            );
        let p2 = center
            + egui::vec2(
                (a2.sin() * radius as f64 * r2) as f32,
                (-(a2.cos()) * radius as f64 * r2) as f32,
            );
        let fill = egui::Color32::from_rgba_unmultiplied(c1.r(), c1.g(), c1.b(), 160);
        painter.add(egui::Shape::convex_polygon(vec![p0, p1, p2], fill, egui::Stroke::NONE));
    }
    // Outline
    let mut points: Vec<egui::Pos2> = cut
        .iter()
        .map(|&(az, g)| {
            let a = az.to_radians();
            let r = 10f64.powf((g - max_g) / 20.0).clamp(0.05, 1.0);
            center
                + egui::vec2(
                    (a.sin() * radius as f64 * r) as f32,
                    (-(a.cos()) * radius as f64 * r) as f32,
                )
        })
        .collect();
    if let Some(first) = points.first().copied() {
        points.push(first);
    }
    painter.add(egui::Shape::line(points, egui::Stroke::new(2.0, egui::Color32::WHITE)));
    painter.text(
        center + egui::vec2(0.0, radius + 6.0),
        egui::Align2::CENTER_TOP,
        format!("{name} — {max_g:.1} dBi"),
        egui::FontId::proportional(11.0),
        egui::Color32::from_gray(150),
    );
}

fn gain_color(t: f32) -> egui::Color32 {
    // t from 0 (weaker) to 1 (stronger) - blue to cyan to green to yellow to red
    if t < 0.25 {
        egui::Color32::from_rgb((50.0 + t * 4.0 * 50.0) as u8, (100.0 + t * 4.0 * 100.0) as u8, 255)
    } else if t < 0.5 {
        let s = (t - 0.25) * 4.0;
        egui::Color32::from_rgb((100.0 + s * 50.0) as u8, 255, (255.0 - s * 100.0) as u8)
    } else if t < 0.75 {
        let s = (t - 0.5) * 4.0;
        egui::Color32::from_rgb((150.0 + s * 105.0) as u8, 255, (155.0 - s * 155.0) as u8)
    } else {
        let s = (t - 0.75) * 4.0;
        egui::Color32::from_rgb(255, (255.0 - s * 100.0) as u8, 0)
    }
}

/// Draw the 3D pattern surface as a heatmapped rotatable wireframe, viewed
/// from near horizon by default to show the takeoff angle.
fn surface_plot(
    ui: &mut egui::Ui,
    verts: &[[f64; 3]],
    nt: usize,
    np: usize,
    yaw: &mut f32,
    pitch: &mut f32,
    height_m: f64,
    wires: &[([f64; 3], [f64; 3])],
) {
    // Scale the antenna height into the normalised pattern space so the ground
    // plane (z = 0) stays put and the antenna lifts off it.
    let height_scaled = (height_m / 30.0).clamp(0.0, 1.0);
    let size = egui::vec2(280.0, 280.0);
    let (response, painter) = ui.allocate_painter(size, egui::Sense::drag());
    let rect = response.rect;
    let center = rect.center();
    let scale = rect.width().min(rect.height()) / 2.0 - 15.0;
    if response.dragged() {
        let d = response.drag_delta();
        *yaw += d.x * 0.01;
        *pitch = (*pitch + d.y * 0.01).clamp(-1.2, 1.2);
    }
    // Start from horizon view if not dragged much (first view)
    if yaw.abs() < 0.001 && pitch.abs() < 0.001 {
        *yaw = -0.8;
        *pitch = 0.6;
    }
    let (cy, sy) = (yaw.cos(), yaw.sin());
    let (cp, sp) = (pitch.cos(), pitch.sin());
    let proj = |v: [f64; 3]| -> egui::Pos2 {
        let (x, y, z) = (v[0] as f32, v[1] as f32, (v[2] + height_scaled as f64) as f32);
        let (x1, y1) = (x * cy - y * sy, x * sy + y * cy); // yaw around z
        let y2 = y1 * cp - z * sp; // pitch around x
        center + egui::vec2(x1 * scale, -y2 * scale)
    };
    // Draw ground plane as an opaque disk (the horizon blocks radiation).
    let ground_fill = egui::Color32::from_rgba_unmultiplied(60, 120, 60, 180);
    let ground_stroke = egui::Stroke::new(1.4, egui::Color32::from_rgb(110, 180, 110));
    let mut ground_pts: Vec<egui::Pos2> = Vec::with_capacity(12);
    for i in 0..12 {
        let ang = (i as f32) * std::f32::consts::PI * 2.0 / 12.0;
        let r = 0.9;
        let x0 = ang.cos() * r * scale;
        let y0 = ang.sin() * r * scale;
        let (x1, y1) = (x0 * cy - y0 * sy, x0 * sy + y0 * cy);
        let y2a = y1 * cp;
        ground_pts.push(center + egui::vec2(x1, -y2a));
    }
    painter.add(egui::Shape::convex_polygon(
        ground_pts.clone(),
        ground_fill,
        ground_stroke,
    ));
    // Outline ring
    let mut ring = ground_pts.clone();
    if let Some(first) = ring.first().copied() {
        ring.push(first);
    }
    painter.add(egui::Shape::line(ring, ground_stroke));

    // Draw the antenna wires in alarm red, normalised to the pattern scale so
    // they sit at the centre of the view instead of far outside it.
    let mut extent = 0.0_f64;
    for w in wires {
        for c in [w.0, w.1] {
            extent = extent.max(c[0].abs()).max(c[1].abs()).max(c[2].abs());
        }
    }
    let wire_scale = if extent > 0.0 { 0.25 / extent } else { 1.0 };
    let wire_stroke = egui::Stroke::new(2.5, egui::Color32::from_rgb(255, 40, 40));
    for w in wires {
        let a = [
            w.0[0] * wire_scale,
            w.0[1] * wire_scale,
            w.0[2] * wire_scale + height_scaled as f64,
        ];
        let b = [
            w.1[0] * wire_scale,
            w.1[1] * wire_scale,
            w.1[2] * wire_scale + height_scaled as f64,
        ];
        let pa = proj(a);
        let pb = proj(b);
        painter.line_segment([pa, pb], wire_stroke);
    }
    // Axis lines
    for i in 0..4 {
        let ang = (i as f32) * std::f32::consts::PI / 2.0;
        let x0 = ang.cos() * 0.9 * scale;
        let y0 = ang.sin() * 0.9 * scale;
        let x1p = 0.0;
        let y1p = 0.0;
        let (x1, y1) = (x0 * cy - y0 * sy, x0 * sy + y0 * cy);
        let (x2, y2p) = (x1p * cy - y1p * sy, x1p * sy + y1p * cy);
        let y2a = y1 * cp;
        let y2b = y2p * cp;
        painter.line_segment(
            [center + egui::vec2(x1, -y2a), center + egui::vec2(x2, -y2b)],
            egui::Stroke::new(1.0, egui::Color32::from_rgb(140, 200, 140)),
        );
    }
    // Find min/max for heatmap
    let mut max_r = 0.0f32;
    for v in verts {
        let r = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt() as f32;
        if r > max_r {
            max_r = r;
        }
    }
    if max_r < 0.01 {
        max_r = 1.0;
    }
    // Draw heatmapped mesh
    for t in 0..(nt.saturating_sub(1)) {
        for p in 0..np {
            let v0 = verts[t * np + p];
            let v1 = verts[t * np + (p + 1) % np];
            let v2 = verts[(t + 1) * np + (p + 1) % np];
            let v3 = verts[(t + 1) * np + p];
            // Skip quads entirely below the ground plane (the horizon blocks them).
            if v0[2] + height_scaled < 0.0
                && v1[2] + height_scaled < 0.0
                && v2[2] + height_scaled < 0.0
                && v3[2] + height_scaled < 0.0
            {
                continue;
            }
            let r0 = ((v0[0] * v0[0] + v0[1] * v0[1] + v0[2] * v0[2]).sqrt() as f32 / max_r)
                .clamp(0.0, 1.0);
            let c0 = gain_color(r0);
            let p0 = proj(v0);
            let p1 = proj(v1);
            let p2 = proj(v2);
            let p3 = proj(v3);
            let fill = egui::Color32::from_rgba_unmultiplied(c0.r(), c0.g(), c0.b(), 120);
            painter.add(egui::Shape::convex_polygon(
                vec![p0, p1, p2, p3],
                fill,
                egui::Stroke::NONE,
            ));
        }
    }
    // Outline
    let stroke = egui::Stroke::new(1.0, egui::Color32::from_rgba_unmultiplied(255, 255, 255, 80));
    for t in 0..nt {
        let ring: Vec<egui::Pos2> = (0..=np).map(|p| proj(verts[t * np + (p % np)])).collect();
        painter.add(egui::Shape::line(ring, stroke));
    }
    for p in 0..np {
        let meridian: Vec<egui::Pos2> = (0..nt).map(|t| proj(verts[t * np + p])).collect();
        painter.add(egui::Shape::line(meridian, stroke));
    }
}

impl super::SdroxideApp {
    pub(in crate::app) fn antenna_window(&mut self, ctx: &egui::Context) {
        if !self.show_antenna {
            return;
        }
        self.antenna_poll();
        let show = self.tool_window(ctx, "antenna", &tr!("antenna-antenna"), [560.0, 720.0], true, |me, ui| {
            me.antenna_body(ui);
        });
        self.show_antenna = show;
    }

    /// Collect a background-thread optimization that has finished.
    fn antenna_poll(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(rx) = &self.antenna_optimize_rx {
            if let Ok(result) = rx.try_recv() {
                self.antenna_optimized = result;
                self.antenna_optimize_rx = None;
                self.antenna_optimizing = false;
            }
        }
    }

    fn antenna_body(&mut self, ui: &mut egui::Ui) {
        crate::chrome::window_body_bg(ui);
        crate::chrome::menu_caption(ui, &tr!("antenna-cutting-lengths"));
        ui.horizontal(|ui| {
            ui.label(tr!("speech-cat-frequency"));
            let mut mhz = self.antenna_freq_mhz;
            if ui
                .add(egui::DragValue::new(&mut mhz).speed(0.05).range(0.05..=3000.0))
                .on_hover_text(tr!("antenna-the-frequency-to-cut-for"))
                .changed()
            {
                self.antenna_freq_mhz = mhz;
                // A new frequency invalidates the earlier solve and optimization.
                self.antenna_solved = None;
                self.antenna_optimized = None;
            }
            ui.label("MHz");
            ui.add_space(12.0);
            ui.label(tr!("antenna-feed-z0"));
            if ui
                .add(egui::DragValue::new(&mut self.antenna_z0).speed(1.0).range(10.0..=600.0))
                .on_hover_text(tr!("antenna-feed-z0-hint"))
                .changed()
            {
                self.antenna_solved = None;
            }
            ui.label("Ω");
            ui.add_space(12.0);
            ui.label(tr!("antenna-height-label"));
            if ui
                .add(egui::DragValue::new(&mut self.antenna_height_m).speed(0.5).range(1.0..=30.0))
                .on_hover_text(tr!("antenna-height-hint"))
                .changed()
            {
                self.antenna_solved = None;
            }
            ui.label(tr!("antenna-m-agl"));
        });

        ui.horizontal(|ui| {
            if ui
                .button(tr!("antenna-apply"))
                .on_hover_text(tr!("antenna-apply-hint"))
                .clicked()
            {
                self.antenna_solve();
            }
            if self.antenna_solved.is_none() {
                self.antenna_solve();
            }
        });
        ui.add_space(4.0);

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add_space(4.0);

            ui.separator();
            crate::chrome::menu_caption(ui, &tr!("antenna-yagi-optimizer"));
            ui.horizontal(|ui| {
                ui.label(tr!("antenna-stack"));
                for n in [1, 2, 4] {
                    let sel = self.antenna_stack == n;
                    if ui.selectable_label(sel, format!("{n}×")).clicked() {
                        self.antenna_stack = n;
                        self.antenna_solved = None;
                    }
                }
                ui.add_space(12.0);
                ui.label(tr!("antenna-objective"));
                egui::ComboBox::from_id_salt("antenna-obj")
                    .selected_text(self.antenna_objective.label())
                    .show_ui(ui, |ui| {
                        for o in Objective::ALL {
                            ui.selectable_value(&mut self.antenna_objective, o, o.label());
                        }
                    });
            });
            #[cfg(not(target_arch = "wasm32"))]
            let optimizing = self.antenna_optimizing;
            #[cfg(target_arch = "wasm32")]
            let optimizing = false;
            if optimizing {
                ui.label(egui::RichText::new(tr!("antenna-optimizing")).italics());
            } else if ui
                .button(tr!("antenna-optimize"))
                .on_hover_text(
                    tr!("antenna-tune-the-yagi-s-element-lengths"),
                )
                .clicked()
            {
                self.antenna_optimize();
            }
            ui.separator();

            let shapes = antenna_model::all(self.antenna_freq_mhz * 1e6);
            ui.columns(2, |cols| {
                cols[0].vertical(|ui| {
                    for d in &shapes {
                        ui.group(|ui| {
                            ui.label(egui::RichText::new(d.name).strong());
                            ui.label(egui::RichText::new(&d.notes).small().weak());
                            for dim in &d.dims {
                                ui.horizontal(|ui| {
                                    ui.label(&dim.what);
                                    ui.add_space(8.0);
                                    ui.monospace(format!("{:.2} m", dim.metres));
                                });
                            }
                            if let Some((r, x)) = d.feed_z {
                                ui.horizontal(|ui| {
                                    ui.label(tr!("antenna-feed-ohms", r = format!("{r:.0}")));
                                    ui.add_space(8.0);
                                    ui.monospace(tr!(
                                        "antenna-swr-value",
                                        v = format!("{:.2}", antenna_model::swr((r, x), self.antenna_z0))
                                    ));
                                });
                            }
                        });
                        ui.add_space(4.0);
                    }
                });
                cols[1].vertical(|ui| {
                    if let Some(rows) = &self.antenna_solved {
                        crate::chrome::menu_caption(ui, &tr!("antenna-solved"));
                        for row in rows {
                    ui.group(|ui| {
                        ui.label(egui::RichText::new(&row.name).strong());
                        ui.horizontal(|ui| {
                            ui.label(format!("Zin {:.1}{:+.1}j Ω", row.r, row.x));
                            ui.add_space(8.0);
                            let swr_c = if row.swr >= 2.0 {
                                egui::Color32::from_rgb(255, 80, 80)
                            } else if row.swr >= 1.5 {
                                egui::Color32::from_rgb(255, 200, 80)
                            } else {
                                egui::Color32::from_rgb(120, 220, 120)
                            };
                            ui.monospace(
                                egui::RichText::new(tr!(
                                    "antenna-swr-ref",
                                    v = format!("{:.2}", row.swr),
                                    z = format!("{:.0}", self.antenna_z0)
                                ))
                                .color(swr_c),
                            );
                            if row.swr >= 2.0 {
                                ui.label(
                                    egui::RichText::new("(!)")
                                        .color(egui::Color32::from_rgb(255, 80, 80))
                                        .small(),
                                );
                            }
                            ui.add_space(8.0);
                            ui.monospace(format!("{:.1} dBi", row.gain));
                            ui.add_space(8.0);
                            ui.monospace(
                                egui::RichText::new(tr!("antenna-takeoff", deg = format!("{:.0}", row.takeoff))).weak(),
                            );
                        });
                    });
                    ui.add_space(4.0);
                }
                ui.separator();
            }
            });
        });

            if !self.antenna_patterns.is_empty() {
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        crate::chrome::menu_caption(ui, &tr!("antenna-radiation-pattern"));
                        let mut sel = self.antenna_pattern_sel;
                        let names: Vec<&str> =
                            self.antenna_patterns.iter().map(|(n, _)| n.as_str()).collect();
                        egui::ComboBox::from_id_salt("antenna-pattern-sel")
                            .selected_text(names[sel])
                            .show_ui(ui, |ui| {
                                for (i, name) in names.iter().enumerate() {
                                    ui.selectable_value(&mut sel, i, *name);
                                }
                            });
                        self.antenna_pattern_sel = sel;
                    });
                    ui.add_space(12.0);
                    ui.vertical(|ui| {
                        crate::chrome::menu_caption(ui, &tr!("antenna-view"));
                        let view_before = self.antenna_pattern_view;
                        egui::ComboBox::from_id_salt("antenna-view")
                            .selected_text(match self.antenna_pattern_view {
                                1 => tr!("antenna-view-front"),
                                2 => tr!("antenna-view-side"),
                                _ => tr!("antenna-view-top"),
                            })
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut self.antenna_pattern_view, 0, tr!("antenna-view-top"));
                                ui.selectable_value(&mut self.antenna_pattern_view, 1, tr!("antenna-view-front"));
                                ui.selectable_value(&mut self.antenna_pattern_view, 2, tr!("antenna-view-side"));
                            });
                        if self.antenna_pattern_view != view_before {
                            match self.antenna_pattern_view {
                                1 => {
                                    self.antenna_yaw = 0.0;
                                    self.antenna_pitch = 0.0;
                                }
                                2 => {
                                    self.antenna_yaw = std::f32::consts::FRAC_PI_2;
                                    self.antenna_pitch = 0.0;
                                }
                                _ => {
                                    self.antenna_yaw = 0.0;
                                    self.antenna_pitch = std::f32::consts::FRAC_PI_2;
                                }
                            }
                        }
                    });
                });
                ui.horizontal(|ui| {
                    if let Some((name, cuts)) = self.antenna_patterns.get(self.antenna_pattern_sel) {
                        let cut = &cuts[self.antenna_pattern_view.min(2)];
                        pattern_plot(ui, name, cut);
                    }
                    if let Some((verts, nt, np)) =
                        self.antenna_surfaces.get(self.antenna_pattern_sel)
                    {
                        let w = self
                            .antenna_wires
                            .get(self.antenna_pattern_sel)
                            .cloned()
                            .unwrap_or_default();
                        surface_plot(
                            ui,
                            verts,
                            *nt,
                            *np,
                            &mut self.antenna_yaw,
                            &mut self.antenna_pitch,
                            self.antenna_height_m,
                            &w,
                        );
                    }
                });
                ui.label(
                    egui::RichText::new(tr!("antenna-3d-hint")).small().weak(),
                );
                ui.separator();
            }

            if let Some((a, score)) = &self.antenna_optimized {
                ui.separator();
                crate::chrome::menu_caption(ui, &tr!("antenna-optimized-yagi"));
                ui.label(format!(
                    "{}: {}",
                    self.antenna_objective.label(),
                    objective_value(self.antenna_objective, *score)
                ));
                let mut y = 0.0_f64;
                for (i, w) in a.wires.iter().enumerate() {
                    let len = (w.to[0] - w.from[0]).abs();
                    let gap = if i == 0 { 0.0 } else { w.from[1] - y };
                    let label = match i {
                        0 => tr!("antenna-reflector"),
                        1 => tr!("antenna-driven"),
                        n => tr!("antenna-director", n = n - 1),
                    };
                    ui.horizontal(|ui| {
                        ui.label(label);
                        ui.add_space(8.0);
                        ui.monospace(format!("{len:.2} m"));
                        if i > 0 {
                            ui.monospace(egui::RichText::new(format!("  {}", tr!("antenna-gap", v = format!("{gap:.2}")))).weak());
                        }
                    });
                    y = w.from[1];
                }
            }

            // Obstacles: a coarse, labelled attenuation model (vegetation and
            // knife-edge), not a terrain solver.
            ui.separator();
            crate::chrome::menu_caption(ui, &tr!("antenna-obstacles-approximate"));
            let mut obstacles: Vec<Obstacle> = Vec::new();
            ui.horizontal(|ui| {
                let mut on = self.antenna_trees.is_some();
                if ui.checkbox(&mut on, tr!("antenna-trees")).changed() {
                    self.antenna_trees = if on { Some((0.0, 40.0, true)) } else { None };
                }
                if let Some((az, depth, leaf)) = &mut self.antenna_trees {
                    ui.add(egui::DragValue::new(az).speed(1.0).range(0.0..=359.0).prefix(tr!("antenna-az") + " "));
                    ui.add(
                        egui::DragValue::new(depth).speed(1.0).range(0.0..=400.0).prefix(tr!("antenna-depth") + " "),
                    );
                    ui.checkbox(leaf, tr!("antenna-in-leaf"));
                }
            });
            if let Some((az, depth, leaf)) = self.antenna_trees {
                obstacles.push(Obstacle::Vegetation {
                    az_deg: az,
                    width_deg: 30.0,
                    distance_m: 30.0,
                    depth_m: depth,
                    in_leaf: leaf,
                });
            }
            ui.horizontal(|ui| {
                let mut on = self.antenna_ridge.is_some();
                if ui.checkbox(&mut on, tr!("antenna-ridge")).changed() {
                    self.antenna_ridge = if on { Some((0.0, 50.0)) } else { None };
                }
                if let Some((az, height)) = &mut self.antenna_ridge {
                    ui.add(egui::DragValue::new(az).speed(1.0).range(0.0..=359.0).prefix(tr!("antenna-az") + " "));
                    ui.add(
                        egui::DragValue::new(height)
                            .speed(1.0)
                            .range(0.0..=500.0)
                            .prefix(tr!("antenna-height") + " "),
                    );
                }
            });
            if let Some((az, height)) = self.antenna_ridge {
                obstacles.push(Obstacle::KnifeEdge {
                    az_deg: az,
                    width_deg: 15.0,
                    distance_m: 100.0,
                    height_m: height,
                });
            }
            if !obstacles.is_empty() {
                let att = attenuation_db(&obstacles, 0.0, self.antenna_freq_mhz * 1e6);
                ui.label(tr!("antenna-attenuation", att = format!("{att:.1}")));
            }
        });
    }

    /// Solve the presets at the current frequency (synchronously — the solver is
    /// a few seconds, which is the honest cost of the method of moments).
    fn antenna_solve(&mut self) {
        use antenna_model::solver::{SolverConfig, analyze, cut, max_gain, pattern_surface, swr};
        let freq_hz = self.antenna_freq_mhz * 1e6;
        let mut rows = Vec::new();
        let mut patterns = Vec::new();
        let mut surfaces = Vec::new();
        let mut wires_list = Vec::new();
        let z0 = self.antenna_z0;
        let height = self.antenna_height_m;
        let presets = [
            antenna_model::presets::dipole(freq_hz, height),
            antenna_model::presets::inverted_v(freq_hz, height),
            antenna_model::presets::vertical(freq_hz),
            antenna_model::presets::ground_plane(freq_hz),
            antenna_model::presets::delta_loop(freq_hz, height),
            antenna_model::presets::yagi(freq_hz, 3, height),
        ];
        for a in presets {
            if let Ok(sol) = analyze(&a, &SolverConfig::default()) {
                let (g, th, _) = max_gain(&sol);
                let name = a.name.clone();
                let cuts = [
                    cut(&sol, true, 90.0, 5.0),
                    cut(&sol, false, 0.0, 5.0),
                    cut(&sol, false, 90.0, 5.0),
                ];
                patterns.push((name.clone(), cuts));
                surfaces.push(pattern_surface(&sol, 37, 48));
                wires_list.push(a.wires.iter().map(|w| (w.from, w.to)).collect());
                rows.push(SolvedRow {
                    name,
                    r: sol.zin.re,
                    x: sol.zin.im,
                    swr: swr(sol.zin, z0),
                    gain: g,
                    takeoff: 90.0 - th,
                });
            }
        }
        // A stacked Yagi, when asked for — the array's combined gain.
        if self.antenna_stack > 1 {
            let single = antenna_model::presets::yagi(freq_hz, 3, height);
            let stacked = antenna_model::transform::stack(
                &single,
                self.antenna_stack as usize,
                0.5 * antenna_model::wavelength_m(freq_hz),
                true,
            );
            if let Ok(sol) = analyze(&stacked, &SolverConfig::default()) {
                let (g, th, _) = max_gain(&sol);
                let name = tr!("antenna-yagi-x", n = self.antenna_stack);
                let cuts = [
                    cut(&sol, true, 90.0, 5.0),
                    cut(&sol, false, 0.0, 5.0),
                    cut(&sol, false, 90.0, 5.0),
                ];
                patterns.push((name.clone(), cuts));
                surfaces.push(pattern_surface(&sol, 37, 48));
                wires_list.push(stacked.wires.iter().map(|w| (w.from, w.to)).collect());
                rows.push(SolvedRow {
                    name,
                    r: sol.zin.re,
                    x: sol.zin.im,
                    swr: swr(sol.zin, z0),
                    gain: g,
                    takeoff: 90.0 - th,
                });
            }
        }
        self.antenna_solved = Some(rows);
        self.antenna_patterns = patterns;
        self.antenna_surfaces = surfaces;
        self.antenna_wires = wires_list;
        self.antenna_pattern_sel =
            self.antenna_pattern_sel.min(self.antenna_patterns.len().saturating_sub(1));
    }

    /// Optimize the 3-element Yagi against the chosen objective, at the dense
    /// solver setting (the coarse one misleads the far field into spurious
    /// optima). Native runs it on a background thread; the browser has no
    /// threads and runs it inline.
    fn antenna_optimize(&mut self) {
        use antenna_model::optimize::optimize_yagi;
        use antenna_model::solver::SolverConfig;
        let freq_hz = self.antenna_freq_mhz * 1e6;
        let height = self.antenna_height_m;
        let obj = self.antenna_objective;
        let cfg = SolverConfig { segs_per_wavelength: 80.0 };
        #[cfg(not(target_arch = "wasm32"))]
        {
            let (tx, rx) = std::sync::mpsc::channel();
            std::thread::spawn(move || {
                let result = optimize_yagi(freq_hz, height, 3, obj, &cfg);
                let _ = tx.send(result);
            });
            self.antenna_optimizing = true;
            self.antenna_optimize_rx = Some(rx);
        }
        #[cfg(target_arch = "wasm32")]
        {
            self.antenna_optimized = optimize_yagi(freq_hz, height, 3, obj, &cfg);
        }
    }
}
