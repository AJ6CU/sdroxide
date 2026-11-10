//! **Ctrl+C copies the tooltip on screen.**
//!
//! egui draws a tooltip with its labels made unselectable on purpose (a
//! selectable tooltip would hold on to the pointer and stop passing hover
//! through to the widget beneath). So the text of a tooltip can be read and
//! never copied — and for an operator who does not read English, that meant
//! writing a long tooltip out by hand to put it into a translator (fork
//! discussion, F6KIM: Settings → UI → Wake lock).
//!
//! The program has well over a thousand tooltips, so this is done once, for
//! all of them, one level down: after the frame has been drawn, [`tick`] looks
//! at what was painted on the tooltip layers and, when the operator has just
//! pressed Ctrl+C (⌘C on a Mac), puts that text on the clipboard. Whatever a
//! tooltip is — a plain `on_hover_text` or a hand-built `on_hover_ui` — its
//! text is in its shapes, so nothing has to be changed at the 1100 call sites
//! and a tooltip added tomorrow is covered too.
//!
//! A short "copied" note is shown by the pointer, so a press is never silent.
//! With no tooltip on screen Ctrl+C is left alone and does what it always did.

use crate::i18n::tr;
use eframe::egui::{self, Context, Event, Id, Key, Order, Shape};

/// How long the "copied" note stays by the pointer.
const NOTE_S: f64 = 1.5;

#[derive(Clone, Default)]
struct State {
    /// The pass the copy was made on, so a frame that runs [`tick`] more than
    /// once (split view: once per radio, then once for the window) copies once.
    copied_pass: Option<u64>,
    /// When the note was raised, and where.
    note: Option<(f64, egui::Pos2)>,
}

fn state_id() -> Id {
    Id::new("sdroxide-tooltip-copy")
}

/// Whether Ctrl+C (the platform's copy shortcut) was pressed this frame.
fn copy_pressed(ctx: &Context) -> bool {
    ctx.input(|i| {
        i.events.iter().any(|e| {
            matches!(e, Event::Copy)
                || matches!(
                    e,
                    Event::Key { key: Key::C, pressed: true, modifiers, .. } if modifiers.command
                )
        })
    })
}

/// The text of every tooltip painted so far this frame, one tooltip's lines
/// after another, or `None` when no tooltip is on screen.
pub fn visible_tooltip_text(ctx: &Context) -> Option<String> {
    let layers: Vec<egui::LayerId> =
        ctx.memory(|m| m.layer_ids().filter(|l| l.order == Order::Tooltip).collect());
    let mut lines: Vec<String> = Vec::new();
    ctx.graphics(|g| {
        for layer in &layers {
            if let Some(list) = g.get(*layer) {
                for clipped in list.all_entries() {
                    collect_text(&clipped.shape, &mut lines);
                }
            }
        }
    });
    let text = lines.join("\n");
    (!text.trim().is_empty()).then_some(text)
}

fn collect_text(shape: &Shape, out: &mut Vec<String>) {
    match shape {
        Shape::Text(t) => {
            let s = t.galley.text().trim();
            if !s.is_empty() {
                out.push(s.to_owned());
            }
        }
        Shape::Vec(v) => v.iter().for_each(|s| collect_text(s, out)),
        _ => {}
    }
}

/// Run after the frame's widgets have drawn: copy the tooltip on Ctrl+C, and
/// draw the "copied" note while it lasts.
pub fn tick(ctx: &Context) {
    let pass = ctx.cumulative_pass_nr();
    let mut st: State = ctx.data(|d| d.get_temp(state_id())).unwrap_or_default();
    if st.copied_pass != Some(pass)
        && copy_pressed(ctx)
        && let Some(text) = visible_tooltip_text(ctx)
    {
        ctx.copy_text(text);
        st.copied_pass = Some(pass);
        let at = ctx.pointer_latest_pos().unwrap_or_default();
        st.note = Some((ctx.input(|i| i.time), at));
    }
    if let Some((since, at)) = st.note {
        let now = ctx.input(|i| i.time);
        if now - since > NOTE_S {
            st.note = None;
        } else {
            // Foreground, not Tooltip: the note must not be read back as a
            // tooltip by the next press.
            egui::Area::new(state_id().with("note"))
                .order(Order::Foreground)
                .fixed_pos(at + egui::vec2(12.0, -28.0))
                .interactable(false)
                .show(ctx, |ui| {
                    egui::Frame::popup(ui.style()).show(ui, |ui| {
                        ui.label(tr!("tooltipcopy-copied"));
                    });
                });
            crate::repaint::after_ms(ctx, 200);
        }
    }
    ctx.data_mut(|d| d.insert_temp(state_id(), st));
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Draw one frame with a tooltip-layer area holding `lines`, and a Ctrl+C
    /// pressed or not; return what was put on the clipboard.
    fn frame(ctx: &Context, lines: &[&str], press: bool) -> Option<String> {
        let mut raw = egui::RawInput::default();
        if press {
            raw.events.push(Event::Copy);
        }
        let out = crate::test_support::run_frame(ctx, raw, |ui| {
            let ctx = ui.ctx().clone();
            ui.label("not a tooltip");
            if !lines.is_empty() {
                egui::Area::new(Id::new("a tooltip")).order(Order::Tooltip).show(&ctx, |ui| {
                    for l in lines {
                        ui.label(*l);
                    }
                });
            }
            tick(&ctx);
        });
        out.platform_output.commands.into_iter().find_map(|c| match c {
            egui::OutputCommand::CopyText(t) => Some(t),
            _ => None,
        })
    }

    #[test]
    fn ctrl_c_copies_the_tooltip_on_screen() {
        let ctx = Context::default();
        // A new area is laid out invisibly on its first frame (egui's sizing
        // pass), so a tooltip has always been on screen a frame before anyone
        // can press anything over it.
        let lines = ["Wake lock", "keeps the screen on"];
        assert_eq!(frame(&ctx, &lines, false), None);
        let got = frame(&ctx, &lines, true);
        assert_eq!(got.as_deref(), Some("Wake lock\nkeeps the screen on"));
    }

    #[test]
    fn without_a_press_nothing_is_copied() {
        let ctx = Context::default();
        let _ = frame(&ctx, &["Wake lock"], false);
        assert_eq!(frame(&ctx, &["Wake lock"], false), None);
    }

    #[test]
    fn with_no_tooltip_ctrl_c_is_left_alone() {
        let ctx = Context::default();
        assert_eq!(frame(&ctx, &[], true), None);
    }

    #[test]
    fn the_copied_note_is_not_read_back_as_a_tooltip() {
        let ctx = Context::default();
        let _ = frame(&ctx, &["first"], false);
        assert_eq!(frame(&ctx, &["first"], true).as_deref(), Some("first"));
        // The note is still on screen; a press with no tooltip copies nothing.
        assert_eq!(frame(&ctx, &[], true), None);
    }
}
