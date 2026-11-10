//! The memories' colour button: egui's picker, plus a way to paste a colour
//! code back. The picker's own copy button puts `r, g, b` on the clipboard;
//! nothing reads it back, and Cmd/Ctrl+V does nothing over a swatch. A memory
//! list of a thousand names is coloured by pasting, not by dragging sliders.

use crate::i18n::tr;
use eframe::egui;

/// A colour code as an operator writes or copies it: `#rrggbb`, `rrggbb`,
/// `r, g, b` (what the picker's copy button puts on the clipboard) or
/// `rgb(r, g, b)`. Anything else is not a colour.
pub(in crate::app) fn parse_colour(text: &str) -> Option<[u8; 3]> {
    let t = text.trim();
    let t = t.strip_prefix("rgb(").and_then(|t| t.strip_suffix(')')).unwrap_or(t).trim();
    let hex = t.strip_prefix('#').unwrap_or(t);
    if hex.len() == 6 && hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        let v = u32::from_str_radix(hex, 16).ok()?;
        return Some([(v >> 16) as u8, (v >> 8) as u8, v as u8]);
    }
    let parts: Vec<&str> = t.split([',', ' ', ';']).filter(|p| !p.is_empty()).collect();
    match parts.as_slice() {
        [r, g, b] => Some([r.parse().ok()?, g.parse().ok()?, b.parse().ok()?]),
        _ => None,
    }
}

/// The picker button with a PASTE chip beside it. Returns whether the colour
/// changed, by the picker or by a paste. A paste is taken from the PASTE chip,
/// or from Cmd/Ctrl+V while the pointer is over the swatch or its picker is open.
#[cfg_attr(not(test), allow(dead_code))]
pub(in crate::app) fn colour_edit(ui: &mut egui::Ui, col: &mut [u8; 3]) -> bool {
    let swatch = ui.color_edit_button_srgb(col);
    // Small: this sits on every row of the memory list, and a roomy chip beside
    // each swatch widened the window the list lives in.
    let paste = crate::chrome::chip_sized(
        ui,
        false,
        egui::RichText::new("PST").size(8.0),
        egui::vec2(22.0, 14.0),
    );
    colour_paste(ui, col, &swatch, paste)
}

/// The swatch's size inside [`colour_swatch_framed`].
const SWATCH: egui::Vec2 = egui::vec2(30.0, 14.0);

/// How wide [`colour_swatch_framed`] stands.
pub(in crate::app) fn colour_swatch_framed_width() -> f32 {
    crate::chrome::chip_frame_width(SWATCH.x)
}

/// The picker's colour square inside a chip-shaped frame, so it reads as one of
/// the chips it sits among. The frame goes round the colour only.
pub(in crate::app) fn colour_swatch_framed(ui: &mut egui::Ui, col: &mut [u8; 3]) -> egui::Response {
    crate::chrome::chip_frame(ui, SWATCH, |ui| ui.color_edit_button_srgb(col))
}

/// The paste half of [`colour_edit`], for a swatch and a PST chip that are not
/// side by side — a column header puts them over different columns. `paste` is
/// the PST chip's response, already drawn with its tooltip untouched here.
pub(in crate::app) fn colour_paste(
    ui: &mut egui::Ui,
    col: &mut [u8; 3],
    swatch: &egui::Response,
    paste: egui::Response,
) -> bool {
    let mut changed = swatch.changed();
    let paste = paste.on_hover_text(tr!("colouredit-paste-a-colour-code-rrggbb-or-r-g"));
    let id = swatch.id.with("paste");
    if swatch.clicked() {
        ui.data_mut(|d| d.insert_temp(id, true));
    }
    let open =
        ui.data(|d| d.get_temp::<bool>(id)).unwrap_or(false) && egui::Popup::is_any_open(ui.ctx());
    if !open {
        ui.data_mut(|d| d.remove::<bool>(id));
    }
    let asked = id.with("asked");
    if paste.clicked() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::RequestPaste);
        ui.data_mut(|d| d.insert_temp(asked, true));
    }
    let waiting = ui.data(|d| d.get_temp::<bool>(asked)).unwrap_or(false);
    let take = waiting || swatch.hovered() || paste.hovered() || open;
    if take {
        let pasted = ui.input(|i| {
            i.events.iter().find_map(|e| match e {
                egui::Event::Paste(t) => Some(t.clone()),
                _ => None,
            })
        });
        if let Some(text) = pasted {
            ui.data_mut(|d| d.remove::<bool>(asked));
            if let Some(c) = parse_colour(&text) {
                changed |= c != *col;
                *col = c;
            }
        }
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_reads_the_codes_an_operator_pastes() {
        assert_eq!(parse_colour("#ff8000"), Some([255, 128, 0]));
        assert_eq!(parse_colour("FF8000"), Some([255, 128, 0]));
        assert_eq!(parse_colour(" 12, 34, 56\n"), Some([12, 34, 56]));
        assert_eq!(parse_colour("rgb(1,2,3)"), Some([1, 2, 3]));
    }

    #[test]
    fn it_refuses_what_is_not_a_colour() {
        for t in ["", "hello", "#12345", "#gggggg", "1, 2", "1, 2, 3, 4", "256, 0, 0"] {
            assert_eq!(parse_colour(t), None, "{t:?}");
        }
    }

    fn frame(ctx: &egui::Context, t: f64, col: &mut [u8; 3], events: Vec<egui::Event>) -> bool {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(400.0, 300.0),
            )),
            time: Some(t),
            events,
            ..Default::default()
        };
        let mut changed = false;
        let mut out = ctx.run_ui(input, |ui| {
            changed = colour_edit(ui, col);
        });
        out.textures_delta.clear();
        changed
    }

    #[test]
    fn cmd_v_over_the_swatch_pastes_the_colour() {
        let ctx = egui::Context::default();
        let mut col = [1, 2, 3];
        frame(&ctx, 1.0, &mut col, vec![]);
        // The swatch is the first widget, at the top left of the panel.
        let at = egui::pos2(14.0, 14.0);
        frame(&ctx, 1.1, &mut col, vec![egui::Event::PointerMoved(at)]);
        let changed = frame(&ctx, 1.2, &mut col, vec![egui::Event::Paste("#102030".into())]);
        assert!(changed);
        assert_eq!(col, [16, 32, 48]);
    }

    #[test]
    fn a_paste_that_is_not_a_colour_changes_nothing() {
        let ctx = egui::Context::default();
        let mut col = [1, 2, 3];
        frame(&ctx, 1.0, &mut col, vec![]);
        frame(&ctx, 1.1, &mut col, vec![egui::Event::PointerMoved(egui::pos2(14.0, 14.0))]);
        assert!(!frame(&ctx, 1.2, &mut col, vec![egui::Event::Paste("salut".into())]));
        assert_eq!(col, [1, 2, 3]);
    }
}
