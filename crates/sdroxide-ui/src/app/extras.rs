//! **EXTRAS**: a small launcher for the receivers that are not a band's
//! ordinary modes. The System box's EXTRAS chip opens it, and like PLAYBACK it
//! is a tool window — it can be pulled out into a window of its own with ⇱.
//!
//! Five buttons side by side is the room it is built for; DAB, AIRCRAFT, RADIO
//! (the Radio Classic panel), ANTENNA and PROGRAMS are in it so far.

use crate::i18n::tr;
use eframe::egui;
use sdroxide_types::{Command, Mode, RxId};

/// How many launcher buttons the panel is wide enough for.
pub(in crate::app) const EXTRAS_SLOTS: usize = 7;

/// One button's width and height, and the gap between buttons.
const SLOT_W: f32 = 112.0;
const SLOT_H: f32 = 30.0;
const SLOT_GAP: f32 = 6.0;
/// The window's opening height.
const EXTRAS_H: f32 = 110.0;

/// The window's width: the slots, their gaps and the frame around them.
pub(in crate::app) const EXTRAS_W: f32 =
    EXTRAS_SLOTS as f32 * SLOT_W + (EXTRAS_SLOTS as f32 - 1.0) * SLOT_GAP + 24.0;

/// Where the window opens, and how big: the remembered geometry when there is
/// one, else its own size at the top right of `avail` (the spectrum frame, just
/// below the top bar and so below the EXTRAS chip). Always kept on `screen`, so
/// a window saved on a larger display is still reachable.
pub(in crate::app) fn extras_place(
    saved: Option<sdroxide_types::DetachedWindow>,
    avail: egui::Rect,
    screen: egui::Rect,
) -> (egui::Pos2, egui::Vec2) {
    tool_place_at(
        saved,
        avail,
        screen,
        egui::vec2(EXTRAS_W, EXTRAS_H),
        egui::vec2(EXTRAS_W, EXTRAS_H),
    )
}

/// [`extras_place`] for any tool window that remembers its place: `own` is the
/// size it opens at and `floor` the smallest it may be left at.
pub(in crate::app) fn tool_place_at(
    saved: Option<sdroxide_types::DetachedWindow>,
    avail: egui::Rect,
    screen: egui::Rect,
    own: egui::Vec2,
    floor: egui::Vec2,
) -> (egui::Pos2, egui::Vec2) {
    let size = saved.map_or(own, |g| egui::vec2(g.size[0], g.size[1]));
    let size = size.max(floor).min(screen.size() * 0.95);
    let pos = match saved.and_then(|g| g.pos) {
        Some([x, y]) => egui::pos2(x, y),
        None => egui::pos2(avail.right() - size.x - 6.0, avail.top() + 6.0),
    };
    let x = pos.x.clamp(screen.left(), (screen.right() - size.x).max(screen.left()));
    let y = pos.y.clamp(screen.top(), (screen.bottom() - 40.0).max(screen.top()));
    (egui::pos2(x, y), size)
}

impl super::SdroxideApp {
    pub(in crate::app) fn extras_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        if !self.show_extras {
            self.extras_was_open = false;
            return;
        }
        // Each opening gets its own window id, so egui holds no position or
        // size for it and the remembered geometry below is what applies.
        if !self.extras_was_open {
            self.extras_epoch += 1;
            self.extras_was_open = true;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if !self.tool_windows.contains_key("extras") {
            self.tool_windows.insert(
                "extras",
                super::ToolWindowState {
                    undocked: false,
                    window: self.ui_settings.extras_os_window,
                },
            );
        }
        let screen = ctx.content_rect();
        let frame =
            egui::Rect::from_min_max(egui::pos2(screen.left(), self.top_bar_bottom), screen.max);
        let (pos, size) = extras_place(self.ui_settings.extras_window, frame, screen);
        self.tool_place = Some(super::ToolPlace { pos, size, epoch: self.extras_epoch });
        self.tool_rect = None;
        let mut out = Vec::new();
        let show = self.tool_window_sized(
            ctx,
            "extras",
            &tr!("extras-title"),
            [EXTRAS_W, EXTRAS_H],
            [EXTRAS_W, 0.0],
            false,
            true,
            |me, ui| me.extras_body(ui, &mut out),
        );
        self.tool_place = None;
        cmds.extend(out);
        self.show_extras = show;
        self.remember_tool_geometry(ctx, "extras", pos, size);
    }

    /// A tool window that keeps where the operator left it, as AIRCRAFT does:
    /// it opens at `own` until it has been moved or resized, and then at that
    /// place on every platform and in every session. `epoch` changes with each
    /// opening, so egui holds no geometry of its own for it and the remembered
    /// one is what applies. Returns whether it is still open.
    pub(in crate::app) fn remembered_tool_window(
        &mut self,
        ctx: &egui::Context,
        id: &'static str,
        title: &str,
        own: [f32; 2],
        floor: [f32; 2],
        epoch: u32,
        body: impl FnOnce(&mut Self, &mut egui::Ui),
    ) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        if !self.tool_windows.contains_key(id) {
            let os = self.tool_geometry_slots(id).1.to_owned();
            self.tool_windows.insert(id, super::ToolWindowState { undocked: false, window: os });
        }
        let screen = ctx.content_rect();
        let frame =
            egui::Rect::from_min_max(egui::pos2(screen.left(), self.top_bar_bottom), screen.max);
        let saved = *self.tool_geometry_slots(id).0;
        let (pos, size) = tool_place_at(
            saved,
            frame,
            screen,
            egui::vec2(own[0], own[1]),
            egui::vec2(floor[0], floor[1]),
        );
        self.tool_place = Some(super::ToolPlace { pos, size, epoch });
        self.tool_rect = None;
        let open = self.tool_window_sized(ctx, id, title, own, floor, false, true, body);
        self.tool_place = None;
        self.remember_tool_geometry(ctx, id, pos, size);
        open
    }

    /// The remembered docked and detached places of a tool window that keeps
    /// them, by its window id.
    fn tool_geometry_slots(
        &mut self,
        id: &str,
    ) -> (&mut Option<sdroxide_types::DetachedWindow>, &mut Option<sdroxide_types::DetachedWindow>)
    {
        match id {
            "aircraft" => {
                (&mut self.ui_settings.aircraft_window, &mut self.ui_settings.aircraft_os_window)
            }
            "schedule" => {
                (&mut self.ui_settings.schedule_window, &mut self.ui_settings.schedule_os_window)
            }
            "memories" => {
                (&mut self.ui_settings.memories_window, &mut self.ui_settings.memories_os_window)
            }
            _ => (&mut self.ui_settings.extras_window, &mut self.ui_settings.extras_os_window),
        }
    }

    /// Keep where the window was left — once the pointer is up, so a drag is
    /// one save — for the docked window and for the detached one.
    pub(in crate::app) fn remember_tool_geometry(
        &mut self,
        ctx: &egui::Context,
        id: &'static str,
        pos: egui::Pos2,
        size: egui::Vec2,
    ) {
        if ctx.input(|i| i.pointer.any_down()) {
            return;
        }
        let mut changed = false;
        #[cfg(not(target_arch = "wasm32"))]
        let undocked = self.tool_windows.get(id).is_some_and(|s| s.undocked);
        #[cfg(target_arch = "wasm32")]
        let undocked = false;
        if undocked {
            #[cfg(not(target_arch = "wasm32"))]
            if let Some(g) = self.tool_windows.get(id).and_then(|s| s.window) {
                let (_, os) = self.tool_geometry_slots(id);
                if *os != Some(g) {
                    *os = Some(g);
                    changed = true;
                }
            }
        } else if let Some(r) = self.tool_rect {
            let g = sdroxide_types::DetachedWindow {
                size: [r.width(), r.height()],
                pos: Some([r.min.x, r.min.y]),
            };
            let moved = (r.min - pos).abs().max_elem() > 1.0 || (r.size().x - size.x).abs() > 1.0;
            let (docked, _) = self.tool_geometry_slots(id);
            let had = docked.is_some();
            let differs = docked.is_none_or(|s| {
                (s.size[0] - g.size[0]).abs() > 1.0
                    || (s.size[1] - g.size[1]).abs() > 1.0
                    || s.pos
                        .zip(g.pos)
                        .is_none_or(|(a, b)| (a[0] - b[0]).abs() > 1.0 || (a[1] - b[1]).abs() > 1.0)
            });
            // Opening at the default is not a choice: only a window the
            // operator has moved or resized is remembered.
            if differs && (moved || had) {
                *self.tool_geometry_slots(id).0 = Some(g);
                changed = true;
            }
        }
        if changed {
            crate::app::persist::persist_ui_settings(&self.ui_settings);
        }
    }

    /// Whether RADIO CLASSIC is on, docked or in its window.
    pub(in crate::app) fn radio_classic_active(&self) -> bool {
        self.ui_settings.radio_classic_on || self.show_radio_classic
    }

    /// The RADIO chips' switch — EXTRAS' and the Wideband row's are this one.
    pub(in crate::app) fn toggle_radio_classic(&mut self) {
        if self.radio_classic_active() {
            self.ui_settings.radio_classic_on = false;
            self.show_radio_classic = false;
            self.ui_settings.tool_windows_open &= !super::frame::TW_RADIO_CLASSIC;
        } else {
            self.ui_settings.radio_classic_on = true;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }

    fn extras_body(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        crate::chrome::window_body_bg(ui);
        crate::chrome::menu_caption(ui, &tr!("extras-receivers"));
        let mode = self.state.rx[0].mode;
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = SLOT_GAP;
            let dab =
                crate::chrome::chip_sized(ui, mode == Mode::Dab, "DAB", egui::vec2(SLOT_W, SLOT_H))
                    .on_hover_text(tr!("extras-dab-hint"));
            if dab.clicked() {
                cmds.push(Command::SetMode { rx: RxId::Main, mode: Mode::Dab });
            }
            let unseen = self.aircraft_tracker.unseen();
            let air = crate::chrome::chip_sized(
                ui,
                self.show_aircraft,
                format!("{}{}", tr!("aircraft-title"), super::aircraft_watch::chip_suffix(unseen)),
                egui::vec2(SLOT_W, SLOT_H),
            )
            .on_hover_text(if unseen > 0 {
                tr!("extras-aircraft-watch-hint")
            } else {
                tr!("extras-aircraft-hint")
            });
            if air.clicked() {
                self.show_aircraft = !self.show_aircraft;
            }
            let wx = crate::chrome::chip_sized(
                ui,
                self.show_wxsat,
                "WXSAT",
                egui::vec2(SLOT_W, SLOT_H),
            )
            .on_hover_text(tr!("extras-wxsat-hint"));
            if wx.clicked() {
                self.show_wxsat = !self.show_wxsat;
            }
            // Where the settings have not switched the panel on, its slot is
            // left empty like the spare ones below, so the window keeps its size.
            if self.ui_settings.meshtastic_panel {
                let mesh = crate::chrome::chip_sized(
                    ui,
                    self.show_meshtastic,
                    "MESHTASTIC",
                    egui::vec2(SLOT_W, SLOT_H),
                )
                .on_hover_text(tr!("extras-meshtastic-hint"));
                if mesh.clicked() {
                    self.show_meshtastic = !self.show_meshtastic;
                }
            } else {
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(SLOT_W, SLOT_H), egui::Sense::hover());
                ui.painter().rect_stroke(
                    rect,
                    3.0,
                    egui::Stroke::new(1.0, crate::theme::gray(60)),
                    egui::StrokeKind::Inside,
                );
            }
            let rc_active = self.radio_classic_active();
            let rc = crate::chrome::chip_sized(ui, rc_active, tr!("topbar-radio"), egui::vec2(SLOT_W, SLOT_H))
                .on_hover_text(tr!("extras-radio-hint"));
            if rc.clicked() {
                self.toggle_radio_classic();
            }
            let ant = crate::chrome::chip_sized(
                ui,
                self.show_antenna,
                tr!("extras-antenna"),
                egui::vec2(SLOT_W, SLOT_H),
            )
            .on_hover_text(tr!("extras-antenna-hint"));
            if ant.clicked() {
                self.show_antenna = !self.show_antenna;
            }
            let programs = crate::chrome::chip_sized(
                ui,
                self.schedule.show,
                "PROGRAMS",
                egui::vec2(SLOT_W, SLOT_H),
            )
            .on_hover_text(tr!("topbar-programs-hint"));
            if programs.clicked() {
                self.schedule.show = !self.schedule.show;
            }
            // The other slots are kept: the panel is sized for them, so a
            // receiver added here does not widen the window.
            for _ in 7..EXTRAS_SLOTS {
                let (rect, _) =
                    ui.allocate_exact_size(egui::vec2(SLOT_W, SLOT_H), egui::Sense::hover());
                ui.painter().rect_stroke(
                    rect,
                    3.0,
                    egui::Stroke::new(1.0, crate::theme::gray(60)),
                    egui::StrokeKind::Inside,
                );
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_panel_holds_every_button_side_by_side() {
        let row = EXTRAS_SLOTS as f32 * SLOT_W + (EXTRAS_SLOTS as f32 - 1.0) * SLOT_GAP;
        assert!(EXTRAS_W >= row, "{EXTRAS_W} pt is narrower than the buttons ({row} pt)");
    }

    fn screen() -> egui::Rect {
        egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1920.0, 1080.0))
    }

    #[test]
    fn it_opens_at_the_top_right_of_the_spectrum_frame() {
        let avail = egui::Rect::from_min_max(egui::pos2(0.0, 180.0), egui::pos2(1920.0, 1080.0));
        let (pos, size) = extras_place(None, avail, screen());
        assert_eq!(pos.y, 186.0, "just under the top bar");
        assert_eq!(pos.x + size.x, 1914.0, "against the right edge of the frame");
    }

    #[test]
    fn a_remembered_place_and_size_win() {
        let g = sdroxide_types::DetachedWindow { size: [900.0, 200.0], pos: Some([300.0, 400.0]) };
        let (pos, size) = extras_place(Some(g), screen(), screen());
        assert_eq!((pos, size), (egui::pos2(300.0, 400.0), egui::vec2(900.0, 200.0)));
    }

    #[test]
    fn a_place_saved_on_a_bigger_screen_is_pulled_back() {
        let g =
            sdroxide_types::DetachedWindow { size: [700.0, 200.0], pos: Some([5000.0, 4000.0]) };
        let (pos, size) = extras_place(Some(g), screen(), screen());
        assert!(screen().contains_rect(egui::Rect::from_min_size(pos, egui::vec2(size.x, 40.0))));
    }
}
