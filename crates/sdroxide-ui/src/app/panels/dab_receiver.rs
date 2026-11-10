//! The DAB panel laid out as a modern DAB receiver, for a panel wide enough
//! to hold it: what is playing on the left — the slideshow, the station, its
//! programme type and the text it sends — the station list beside it, the VU
//! meter, and a transport bar underneath. All of it in the left half of the
//! panel, so the right half stays free.
//!
//! A narrow panel keeps the stacked lists in `dab.rs`; this is the desktop
//! face of the same state, and every press here sends the same commands.

use eframe::egui::{self, Align2, Color32, FontId, Pos2, Rect, RichText, Sense, Stroke, Vec2};
use sdroxide_types::{Command, DabKnownStation, DabService, DabStatus};

use super::dab::{
    dab_channel_choices, dab_clear_offered, dab_focus_service, dab_group_layout, dab_mhz,
    dab_service_extra, dab_station_rows, dab_status_is_for,
};
use crate::app::SdroxideApp;
use crate::i18n::tr;
use crate::theme;

/// The transport bar's height, and the gap between the panel's parts.
const BAR_H: f32 = 30.0;
const GAP: f32 = 10.0;
/// One station row.
const ROW_H: f32 = 22.0;

/// What the channel picker says while closed: which block the receiver is
/// on, said in words — "Channel received 8D 201.072".
pub(in crate::app) fn channel_caption(channel: &str) -> String {
    match sdroxide_types::DAB_BAND_III.iter().find(|(n, _)| *n == channel) {
        Some((n, hz)) => tr!("dab-channel-received-mhz", channel = *n, mhz = dab_mhz(*hz).to_string()),
        None if channel.is_empty() => tr!("dab-no-channel-received"),
        None => tr!("dab-channel-received", channel = channel),
    }
}

/// Decode a slideshow picture as the station sent it — JPEG or PNG, any size
/// — into an image the panel can draw. `None` when the bytes are not one.
pub(in crate::app) fn slide_image(data: &[u8]) -> Option<egui::ColorImage> {
    let img = image::load_from_memory(data).ok()?.to_rgba8();
    let (w, h) = img.dimensions();
    if w == 0 || h == 0 || w > 4096 || h > 4096 {
        return None;
    }
    Some(egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], &img))
}

/// Draw `tex` as large as fits in `frame`, centred, in its own proportions.
fn paint_fitted(p: &egui::Painter, tex: &egui::TextureHandle, frame: Rect) -> Rect {
    let [w, h] = tex.size();
    let fit = (frame.width() / w as f32).min(frame.height() / h as f32);
    let img = Rect::from_center_size(frame.center(), Vec2::new(w as f32, h as f32) * fit);
    p.add(
        egui::epaint::RectShape::filled(img, 6.0, Color32::WHITE)
            .with_texture(tex.id(), Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0))),
    );
    img
}

/// Up to three letters for the slideshow tile before a station sends a
/// picture: the initials of a multi-word name, else its first letters.
pub(in crate::app) fn station_initials(name: &str) -> String {
    let words: Vec<&str> = name.split_whitespace().collect();
    let s: String = if words.len() >= 2 {
        words.iter().filter_map(|w| w.chars().next()).take(3).collect()
    } else {
        name.chars().filter(|c| c.is_alphanumeric()).take(3).collect()
    };
    s.to_uppercase()
}

impl SdroxideApp {
    /// The receiver face of the DAB panel, `width` wide and `avail_h` tall.
    pub(in crate::app) fn dab_receiver(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        st: &DabStatus,
        width: f32,
        avail_h: f32,
    ) {
        let (group, vu_w, compact) = dab_group_layout(width);
        let body_h = (avail_h - BAR_H - 8.0).max(120.0);
        let rest = (group - vu_w - 2.0 * GAP).max(200.0);
        let card_w = rest * 0.56;
        let list_w = rest - card_w;
        let channel = self.state.dab.channel.clone();
        let live: Vec<DabService> =
            if dab_status_is_for(st, &channel) { st.services.clone() } else { Vec::new() };
        let rows = dab_station_rows(
            &live,
            &self.state.dab.stations,
            &channel,
            &self.dab_hidden,
            &self.state.dab.presets,
        );

        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = GAP;
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 6.0;
                ui.horizontal_top(|ui| {
                    ui.spacing_mut().item_spacing.x = GAP;
                    self.dab_now_playing(ui, st, &live, egui::vec2(card_w, body_h));
                    self.dab_station_list(ui, cmds, st, &rows, &live, egui::vec2(list_w, body_h));
                });
                self.dab_transport(ui, cmds, &rows, card_w + GAP + list_w);
            });
            self.dab_vu_meter(ui, cmds, st, body_h + BAR_H + 6.0, vu_w, compact);
            // The right of the panel: the slideshow, as large as it fits.
            let free = ui.available_width();
            if free > 120.0 {
                self.dab_slideshow(ui, egui::vec2(free, body_h + BAR_H + 6.0));
            }
        });
    }

    /// The slideshow the station broadcasts (MOT SlideShow): the picture in
    /// its own shape — 320x240 is the standard's, but stations send what they
    /// like, square included — never stretched, framed, with a caption.
    fn dab_slideshow(&self, ui: &mut egui::Ui, size: Vec2) {
        let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
        let p = ui.painter_at(rect);
        p.rect_filled(rect, 8.0, theme::BG_DEEP());
        p.rect_stroke(rect, 8.0, Stroke::new(1.0, theme::LINE()), egui::StrokeKind::Inside);
        let inner = rect.shrink(8.0);
        p.text(
            inner.left_top(),
            Align2::LEFT_TOP,
            &tr!("dab-slideshow"),
            FontId::proportional(10.0),
            theme::gray(140),
        );
        let frame = Rect::from_min_max(inner.left_top() + Vec2::new(0.0, 16.0), inner.max);
        match &self.dab_slide {
            Some(tex) => {
                let img = paint_fitted(&p, tex, frame);
                p.rect_stroke(
                    img,
                    6.0,
                    Stroke::new(1.0, theme::LINE_LIT()),
                    egui::StrokeKind::Outside,
                );
                let [w, h] = tex.size();
                p.text(
                    inner.right_top(),
                    Align2::RIGHT_TOP,
                    format!("{w}×{h}"),
                    FontId::proportional(9.5),
                    theme::gray(120),
                );
            }
            None => {
                p.text(
                    frame.center(),
                    Align2::CENTER_CENTER,
                    &tr!("dab-no-picture-yet"),
                    FontId::proportional(11.0),
                    theme::gray(120),
                );
            }
        }
    }

    /// The same receiver for a narrow panel (a phone, a small window): the
    /// card across the top, the meter on its side under it, the slideshow, the
    /// station list, and the transport bar at the foot.
    pub(in crate::app) fn dab_receiver_stacked(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        st: &DabStatus,
        width: f32,
        avail_h: f32,
    ) {
        let channel = self.state.dab.channel.clone();
        let live: Vec<DabService> =
            if dab_status_is_for(st, &channel) { st.services.clone() } else { Vec::new() };
        let rows = dab_station_rows(
            &live,
            &self.state.dab.stations,
            &channel,
            &self.dab_hidden,
            &self.state.dab.presets,
        );
        let card_h = 150.0f32.min(avail_h * 0.34);
        let vu_h = crate::widgets::vu_meter::HEIGHT_HORIZONTAL;
        let slide_h =
            if self.dab_slide.is_some() { (width * 0.6).min(avail_h * 0.30) } else { 0.0 };
        // At least five stations in view; on a short screen the whole column
        // scrolls rather than pushing the list and the bar off the bottom.
        let list_h = (avail_h - card_h - vu_h - slide_h - BAR_H - 30.0).max(ROW_H * 5.0 + 50.0);
        egui::ScrollArea::vertical().id_salt("dab_receiver_stacked").show(ui, |ui| {
            ui.spacing_mut().item_spacing.y = 6.0;
            self.dab_now_playing(ui, st, &live, egui::vec2(width, card_h));
            self.dab_vu_step(ui, st);
            let volume = self.state.rx[0].volume;
            let out = crate::widgets::vu_meter::stereo_horizontal(
                ui,
                &self.dab_vu,
                egui::vec2(width, vu_h),
                Some(volume),
            );
            self.dab_vu_volume(ui, cmds, out);
            if slide_h > 0.0 {
                self.dab_slideshow(ui, egui::vec2(width, slide_h));
            }
            self.dab_station_list(ui, cmds, st, &rows, &live, egui::vec2(width, list_h));
            self.dab_transport(ui, cmds, &rows, width);
        });
    }

    /// The left card: the slideshow, the station, what it is, and its text.
    fn dab_now_playing(&self, ui: &mut egui::Ui, st: &DabStatus, live: &[DabService], size: Vec2) {
        let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
        let p = ui.painter_at(rect);
        p.rect_filled(rect, 8.0, theme::BG_DEEP());
        p.rect_stroke(rect, 8.0, Stroke::new(1.0, theme::LINE()), egui::StrokeKind::Inside);
        let inner = rect.shrink(10.0);

        // Top line: where the receiver is.
        let freq = sdroxide_types::DAB_BAND_III
            .iter()
            .find(|(n, _)| *n == self.state.dab.channel)
            .map(|(_, hz)| format!("{}  ·  {} MHz", self.state.dab.channel, dab_mhz(*hz)))
            .unwrap_or_else(|| self.state.dab.channel.clone());
        p.text(
            inner.left_top(),
            Align2::LEFT_TOP,
            freq,
            FontId::proportional(13.0),
            theme::YELLOW(),
        );
        p.text(
            inner.right_top(),
            Align2::RIGHT_TOP,
            "DAB+",
            FontId::proportional(11.0),
            theme::CYAN_DIM(),
        );

        // What is described: the station playing, else the one chosen.
        let svc = dab_focus_service(st, &self.state.dab.service_id)
            .or_else(|| live.iter().find(|s| s.service_id == self.state.dab.service_id));
        let name = svc
            .map(|s| if s.label.trim().is_empty() { s.service_id.clone() } else { s.label.clone() })
            .unwrap_or_else(|| tr!("dab-no-station-chosen"));

        // The tile: the station's logo, else the slideshow's picture in small —
        // the logo (SPI) is not decoded, and the slide is very often the
        // station's own card — else its initials.
        let tile = self.dab_logo.as_ref().or(self.dab_slide.as_ref());
        let dls_h = 40.0;
        let art_h = (inner.height() - 22.0 - dls_h - 10.0)
            .min(inner.width() * 0.42 * 0.75)
            .clamp(48.0, 150.0);
        // The tile takes the picture's own shape: a square station logo gets a
        // square, a 320x240 slide a 4:3 frame. Never stretched.
        let aspect = tile
            .map(|t| {
                let [w, h] = t.size();
                (w as f32 / h.max(1) as f32).clamp(1.0, 4.0 / 3.0)
            })
            .unwrap_or(4.0 / 3.0);
        let art = Rect::from_min_size(
            inner.left_top() + Vec2::new(0.0, 22.0),
            Vec2::new(art_h * aspect, art_h),
        );
        if let Some(tex) = tile {
            p.rect_filled(art, 6.0, Color32::BLACK);
            paint_fitted(&p, tex, art);
        } else {
            p.rect_filled(art, 6.0, theme::FILL());
            p.text(
                art.center() - Vec2::new(0.0, 6.0),
                Align2::CENTER_CENTER,
                station_initials(&name),
                FontId::proportional(art_h * 0.32),
                theme::CYAN(),
            );
            p.text(
                art.center_bottom() - Vec2::new(0.0, 10.0),
                Align2::CENTER_CENTER,
                &tr!("dab-no-logo-yet"),
                FontId::proportional(9.0),
                theme::gray(130),
            );
        }
        p.rect_stroke(art, 6.0, Stroke::new(1.0, theme::LINE_LIT()), egui::StrokeKind::Inside);

        // Beside it: the name, the ensemble, the programme type, the numbers.
        let tx = art.right() + 12.0;
        let wrap_w = (inner.right() - tx).max(40.0);
        let mut y = art.top();
        let floor = inner.bottom() - dls_h - 4.0;
        // One line each, shrunk to fit the column and cut short past that,
        // and nothing at all once the text above the DLS box runs out of room.
        let line = |text: String, size: f32, color: Color32, y: &mut f32| {
            let mut sz = size;
            let mut g = p.layout_no_wrap(text.clone(), FontId::proportional(sz), color);
            while g.size().x > wrap_w && sz > size * 0.7 {
                sz -= 1.0;
                g = p.layout_no_wrap(text.clone(), FontId::proportional(sz), color);
            }
            if *y + g.size().y > floor {
                return;
            }
            let mut job =
                egui::text::LayoutJob::simple_singleline(text, FontId::proportional(sz), color);
            job.wrap = egui::text::TextWrapping::truncate_at_width(wrap_w);
            let g = p.layout_job(job);
            let h = g.size().y;
            p.galley(Pos2::new(tx, *y), g, color);
            *y += h + 4.0;
        };
        line(name, 20.0, theme::TEXT_STRONG(), &mut y);
        if let Some(e) = &st.ensemble {
            line(e.clone(), 12.0, theme::gray(170), &mut y);
        }
        if let Some(pty) = svc.and_then(|s| s.pty.clone())
            && y + 20.0 <= floor
        {
            let font = FontId::proportional(10.5);
            let g = p.layout_no_wrap(pty.to_uppercase(), font, theme::CYAN());
            let tag = Rect::from_min_size(Pos2::new(tx, y), g.size() + Vec2::new(12.0, 6.0));
            p.rect_stroke(tag, 4.0, Stroke::new(1.0, theme::CYAN_DIM()), egui::StrokeKind::Inside);
            p.galley(tag.min + Vec2::new(6.0, 3.0), g, theme::CYAN());
            y = tag.bottom() + 6.0;
        }
        let playing = svc.is_some_and(|s| st.playing.as_deref() == Some(s.service_id.as_str()));
        line(
            if playing { tr!("dab-playing") } else { tr!("dab-chosen") },
            10.5,
            if playing { theme::CYAN_DIM() } else { theme::gray(140) },
            &mut y,
        );
        if let Some(s) = svc {
            line(dab_service_extra(s), 10.0, theme::gray(130), &mut y);
        }

        // The station's own text (DLS), across the bottom of the card.
        let dls = Rect::from_min_max(
            Pos2::new(inner.left(), inner.bottom() - dls_h),
            inner.right_bottom(),
        );
        p.rect_filled(dls, 5.0, theme::INPUT_BG());
        let (text, color) = match st.dls.as_deref().filter(|t| !t.trim().is_empty()) {
            Some(t) => (format!("♫  {t}"), theme::TEXT()),
            None => (tr!("dab-no-text-yet"), theme::gray(120)),
        };
        let g = p.layout(text, FontId::proportional(11.5), color, dls.width() - 16.0);
        p.galley(dls.left_center() + Vec2::new(8.0, -g.size().y / 2.0), g, color);
    }

    /// The station list beside the card: the channel picker above, one thin
    /// numbered row per station, the one chosen lit, a star on a preset, and a
    /// small × on the row under the pointer to clear it.
    fn dab_station_list(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        st: &DabStatus,
        rows: &[DabService],
        live: &[DabService],
        size: Vec2,
    ) {
        let (rect, _) = ui.allocate_exact_size(size, Sense::hover());
        ui.painter_at(rect).rect_filled(rect, 8.0, theme::BG_DEEP());
        ui.painter_at(rect).rect_stroke(
            rect,
            8.0,
            Stroke::new(1.0, theme::LINE()),
            egui::StrokeKind::Inside,
        );
        let mut ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(rect.shrink(8.0))
                .layout(egui::Layout::top_down(egui::Align::Min)),
        );
        let ui = &mut ui;

        // The channel picker: every block found, each with its own ×, and a
        // way to clear the lot.
        ui.horizontal(|ui| {
            let channels = dab_channel_choices(
                &self.state.dab.found,
                self.dab_scan.as_ref().map(|s| s.found.as_slice()),
            );
            let shown = channel_caption(&self.state.dab.channel);
            // The station count beside it only while there is room for both:
            // a picker pushed past its column takes the list with it.
            let room = ui.available_width();
            let with_count = room >= 300.0;
            egui::ComboBox::from_id_salt("dab_channel_pick")
                .selected_text(RichText::new(shown).size(12.0))
                .width(if with_count { room - 70.0 } else { room - 8.0 }.max(80.0))
                .show_ui(ui, |ui| {
                    for name in &channels {
                        let freq = sdroxide_types::DAB_BAND_III
                            .iter()
                            .find(|(n, _)| n == name)
                            .map(|(_, hz)| *hz);
                        ui.horizontal(|ui| {
                            if ui
                                .small_button("×")
                                .on_hover_text(tr!("dab-clear-channel-hint"))
                                .clicked()
                            {
                                self.state.dab.forget_channel(name);
                                cmds.push(Command::SetDabConfig(self.state.dab.clone()));
                            }
                            let label = match freq {
                                Some(hz) => format!("{name}  {}", dab_mhz(hz)),
                                None => name.clone(),
                            };
                            if ui.selectable_label(self.state.dab.channel == *name, label).clicked()
                            {
                                self.dab_scan = None;
                                self.state.dab.channel = name.clone();
                                self.state.dab.service_id.clear();
                                cmds.push(Command::SetDabConfig(self.state.dab.clone()));
                                if let Some(hz) = freq {
                                    cmds.push(Command::SetVfo { vfo: self.state.active_vfo, hz });
                                }
                            }
                        });
                    }
                    // Kevin's "Add clear list" (#18): the whole list at once.
                    // Not offered mid-sweep, where stopping would write the
                    // sweep's findings straight back over the clearing.
                    if dab_clear_offered(self.dab_scan.is_some(), self.state.dab.found.len()) {
                        ui.separator();
                        if ui
                            .button(tr!("dab-clear-all-channels"))
                            .on_hover_text(tr!("dab-clear-all-channels-hint"))
                            .clicked()
                        {
                            self.state.dab.found.clear();
                            self.state.dab.stations.clear();
                            cmds.push(Command::SetDabConfig(self.state.dab.clone()));
                        }
                    }
                });
            if with_count {
                ui.label(
                    RichText::new(tr!("dab-n-stations", n = rows.len()))
                        .size(10.0)
                        .color(theme::gray(140)),
                );
            }
        });
        if let Some(scan) = self.dab_scan.as_ref() {
            let n = sdroxide_types::DAB_BAND_III.len();
            let at = scan.at.min(n - 1);
            ui.add(
                egui::ProgressBar::new((at + 1) as f32 / n as f32).desired_height(10.0).text(
                    RichText::new(tr!(
                        "dab-scanning-progress",
                        channel = sdroxide_types::DAB_BAND_III[at].0,
                        at = at + 1,
                        n = n,
                        found = scan.found.len()
                    ))
                    .size(9.0),
                ),
            );
        }
        ui.add_space(2.0);

        let channel = self.state.dab.channel.clone();
        let chosen = st.playing.clone().unwrap_or_else(|| self.state.dab.service_id.clone());
        egui::ScrollArea::vertical().id_salt("dab_receiver_rows").auto_shrink([false, false]).show(
            ui,
            |ui| {
                ui.spacing_mut().item_spacing.y = 1.0;
                for (i, s) in rows.iter().enumerate() {
                    let w = ui.available_width();
                    let (r, resp) = ui.allocate_exact_size(Vec2::new(w, ROW_H), Sense::click());
                    let lit = s.service_id == chosen;
                    let heard = live.iter().any(|l| l.service_id == s.service_id);
                    let preset = self
                        .state
                        .dab
                        .presets
                        .iter()
                        .any(|k| k.channel == channel && k.service_id == s.service_id);
                    let p = ui.painter_at(r);
                    if lit {
                        p.rect_filled(r, 4.0, theme::CYAN());
                    } else if resp.hovered() {
                        p.rect_filled(r, 4.0, theme::ROW_HOVER());
                    }
                    let ink = if lit {
                        theme::INK_ON_CYAN()
                    } else if heard {
                        theme::TEXT()
                    } else {
                        theme::gray(140)
                    };
                    p.text(
                        r.left_center() + Vec2::new(6.0, 0.0),
                        Align2::LEFT_CENTER,
                        format!("{:>2}", i + 1),
                        FontId::monospace(11.0),
                        if lit { ink } else { theme::gray(120) },
                    );
                    let name = if s.label.trim().is_empty() {
                        format!("({})", s.service_id)
                    } else {
                        s.label.clone()
                    };
                    p.text(
                        r.left_center() + Vec2::new(30.0, 0.0),
                        Align2::LEFT_CENTER,
                        name,
                        FontId::proportional(12.5),
                        ink,
                    );
                    if self.dab_show_pty
                        && let Some(pty) = &s.pty
                    {
                        p.text(
                            r.right_center() - Vec2::new(40.0, 0.0),
                            Align2::RIGHT_CENTER,
                            pty,
                            FontId::proportional(9.5),
                            if lit { ink } else { theme::CYAN_DIM() },
                        );
                    }
                    if preset {
                        p.text(
                            r.right_center() - Vec2::new(24.0, 0.0),
                            Align2::CENTER_CENTER,
                            "★",
                            FontId::proportional(11.0),
                            if lit { ink } else { theme::YELLOW() },
                        );
                    }
                    // The × shows only on the row under the pointer, so the
                    // list stays a clean column of names.
                    let mut cleared = false;
                    if resp.hovered() || ui.rect_contains_pointer(r) {
                        let c = r.right_center() - Vec2::new(10.0, 0.0);
                        let xr = Rect::from_center_size(c, Vec2::splat(16.0));
                        let x =
                            ui.interact(xr, resp.id.with("clear"), Sense::click()).on_hover_text(tr!("dab-clear-station-hint"));
                        p.text(
                            c,
                            Align2::CENTER_CENTER,
                            "×",
                            FontId::proportional(13.0),
                            if x.hovered() { theme::ALERT() } else { ink },
                        );
                        if x.clicked() {
                            cleared = true;
                            self.state.dab.forget_station(&channel, &s.service_id);
                            self.dab_hidden.push((channel.clone(), s.service_id.clone()));
                            cmds.push(Command::SetDabConfig(self.state.dab.clone()));
                        }
                    }
                    if resp.clicked() && !cleared {
                        self.state.dab.service_id = s.service_id.clone();
                        cmds.push(Command::SetDabConfig(self.state.dab.clone()));
                    }
                }
            },
        );
    }

    /// The transport bar: the previous and next station, the preset star, the
    /// programme types, and the sweep.
    fn dab_transport(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        rows: &[DabService],
        width: f32,
    ) {
        let channel = self.state.dab.channel.clone();
        let current = self.state.dab.service_id.clone();
        let at = rows.iter().position(|s| s.service_id == current);
        let is_preset =
            self.state.dab.presets.iter().any(|k| k.channel == channel && k.service_id == current);
        let scanning = self.dab_scan.is_some();
        let n = 5.0;
        let w = (width - (n - 1.0) * 6.0) / n;
        let size = egui::vec2(w, BAR_H);
        // Words while there is room for them, the symbols alone on a phone.
        let words = w >= 88.0;
        let label = |icon: &str, word: &str, after: bool| -> String {
            match (words, after) {
                (false, _) => icon.to_string(),
                (true, false) => format!("{icon}  {word}"),
                (true, true) => format!("{word}  {icon}"),
            }
        };
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            let mut pick: Option<usize> = None;
            if crate::chrome::chip_sized(ui, false, label("⏮", &tr!("dab-prev"), false), size)
                .on_hover_text(tr!("dab-prev-hint"))
                .clicked()
                && !rows.is_empty()
            {
                pick = Some(at.map_or(0, |i| (i + rows.len() - 1) % rows.len()));
            }
            if crate::chrome::chip_sized(ui, is_preset, label("★", &tr!("dab-preset"), false), size)
                .on_hover_text(tr!("dab-preset-hint"))
                .clicked()
                && !current.is_empty()
            {
                let presets = &mut self.state.dab.presets;
                if is_preset {
                    presets.retain(|k| !(k.channel == channel && k.service_id == current));
                } else {
                    let label = rows
                        .iter()
                        .find(|s| s.service_id == current)
                        .map(|s| s.label.clone())
                        .unwrap_or_default();
                    presets.push(DabKnownStation {
                        channel: channel.clone(),
                        service_id: current.clone(),
                        label,
                    });
                }
                cmds.push(Command::SetDabConfig(self.state.dab.clone()));
            }
            if crate::chrome::chip_sized(ui, false, label("⏭", &tr!("dab-next"), true), size)
                .on_hover_text(tr!("dab-next-hint"))
                .clicked()
                && !rows.is_empty()
            {
                pick = Some(at.map_or(0, |i| (i + 1) % rows.len()));
            }
            if crate::chrome::chip_sized(ui, self.dab_show_pty, label("🏷", "PTY", false), size)
                .on_hover_text(tr!("dab-pty-hint"))
                .clicked()
            {
                self.dab_show_pty = !self.dab_show_pty;
            }
            if crate::chrome::chip_sized(
                ui,
                scanning,
                if scanning { label("⏹", &tr!("windows-stop"), false) } else { label("⟳", &tr!("topbar-chip-scan"), false) },
                size,
            )
            .on_hover_text(if scanning {
                tr!("dab-stop-sweep-hint")
            } else {
                tr!("dab-scan-hint")
            })
            .clicked()
            {
                self.dab_toggle_scan(cmds);
            }
            if let Some(s) = pick.and_then(|i| rows.get(i)) {
                self.state.dab.service_id = s.service_id.clone();
                cmds.push(Command::SetDabConfig(self.state.dab.clone()));
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::{channel_caption, slide_image, station_initials};

    #[test]
    fn the_picker_names_the_channel_received() {
        assert_eq!(channel_caption("8D"), "Channel received 8D 201.072");
        assert_eq!(channel_caption(""), "No channel received");
    }

    /// A slideshow picture keeps the size the station sent it at, whatever
    /// its shape, and bytes that are no picture are refused, not drawn.
    #[test]
    fn a_slide_decodes_at_the_size_the_station_sent() {
        for (w, h) in [(320u32, 240u32), (200, 200)] {
            let mut png = Vec::new();
            image::RgbaImage::from_pixel(w, h, image::Rgba([10, 20, 200, 255]))
                .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
                .unwrap();
            let img = slide_image(&png).expect("a PNG slide");
            assert_eq!(img.size, [w as usize, h as usize]);
        }
        assert!(slide_image(b"not a picture").is_none());
    }

    #[test]
    fn the_slideshow_tile_names_the_station_until_a_picture_comes() {
        assert_eq!(station_initials("BFM BUSINESS"), "BB");
        assert_eq!(station_initials("Radio Nova Paris"), "RNP");
        assert_eq!(station_initials("FIP"), "FIP");
        assert_eq!(station_initials("skyrock"), "SKY");
        assert_eq!(station_initials(""), "");
    }
}
