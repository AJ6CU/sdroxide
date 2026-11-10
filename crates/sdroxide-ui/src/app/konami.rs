//! Just here for the 11 m band.

use eframe::egui::{self, Color32, CornerRadius, vec2};

/// Held down for this long.
const SHOW_S: f32 = 5.0;

/// Shown as it was drawn — nothing added to it. Downscaled from the original
/// for the binary's sake; it is drawn at about this size.
const CARD: &[u8] = include_bytes!("../../assets/qsl.jpg");

/// The sequence, in the order it has always been.
const SEQ: [egui::Key; 10] = [
    egui::Key::ArrowUp,
    egui::Key::ArrowUp,
    egui::Key::ArrowDown,
    egui::Key::ArrowDown,
    egui::Key::ArrowLeft,
    egui::Key::ArrowRight,
    egui::Key::ArrowLeft,
    egui::Key::ArrowRight,
    egui::Key::B,
    egui::Key::A,
];

#[derive(Default)]
pub(in crate::app) struct Konami {
    step: usize,
    left_s: f32,
    card: Option<egui::TextureHandle>,
}

impl Konami {
    pub(in crate::app) fn tick(&mut self, dt: f32) {
        self.left_s = (self.left_s - dt).max(0.0);
    }

    pub(in crate::app) fn showing(&self) -> bool {
        self.left_s > 0.0
    }

    /// Watch this frame's key presses. `armed` is whether the band is the one
    /// this is for; the sequence is still tracked when it is not, so the last
    /// key has to land on the right band rather than the first.
    pub(in crate::app) fn feed(&mut self, events: &[egui::Event], armed: bool) {
        for event in events {
            let egui::Event::Key { key, pressed: true, repeat: false, .. } = event else {
                continue;
            };
            if *key == SEQ[self.step] {
                self.step += 1;
                if self.step == SEQ.len() {
                    self.step = 0;
                    if armed {
                        self.left_s = SHOW_S;
                    }
                }
            } else {
                // A wrong key is not a dead end: it can be the first of a new
                // attempt, which is what someone mistyping the middle expects.
                self.step = usize::from(*key == SEQ[0]);
            }
        }
    }

    pub(in crate::app) fn draw(&mut self, ui: &egui::Ui, salt: u64) {
        if !self.showing() {
            return;
        }
        let ctx = ui.ctx().clone();
        let pane = ui.max_rect();
        let painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Foreground,
            egui::Id::new(("distant sail", salt)),
        ));
        painter.rect_filled(pane, CornerRadius::ZERO, Color32::from_black_alpha(90));

        let elapsed = SHOW_S - self.left_s;
        let a = (elapsed / 0.25).clamp(0.0, 1.0) * (self.left_s / 0.7).clamp(0.0, 1.0);

        let card = self.card.get_or_insert_with(|| card_texture(&ctx)).clone();
        let size = card.size_vec2();
        // Most of the room where there is room, and never more than the pane:
        // the card keeps its own shape, so the smaller of the two fits wins.
        let scale = (pane.width() * 0.60 / size.x).min(pane.height() * 0.60 / size.y);
        let shown = vec2(size.x * scale, size.y * scale);
        painter.image(
            card.id(),
            egui::Rect::from_center_size(pane.center(), shown),
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            Color32::from_rgba_unmultiplied(255, 255, 255, (255.0 * a) as u8),
        );

        // Keep the fade moving while it is up; the frame loop may otherwise be
        // idling between spectrum frames.
        ctx.request_repaint();
    }
}

fn card_texture(ctx: &egui::Context) -> egui::TextureHandle {
    let image = image::load_from_memory(CARD)
        .map(|d| d.to_rgba8())
        .unwrap_or_else(|_| image::RgbaImage::new(1, 1));
    let (w, h) = image.dimensions();
    let pixels = egui::ColorImage::from_rgba_unmultiplied([w as usize, h as usize], image.as_raw());
    ctx.load_texture("distant sail", pixels, egui::TextureOptions::LINEAR)
}

/// Where `Band::M11` lives, for the caller's `armed` test. Kept here so the
/// one place that cares is the one place that names it.
pub(in crate::app) fn armed(band: sdroxide_types::Band) -> bool {
    band == sdroxide_types::Band::M11
}
