//! The operator's message buttons, shared by the panels that send free text.
//!
//! Both the CW panel and the keyboard-mode (PSK / RTTY / Olivia / Thor) panel
//! offer the same control: a row of chips under the send buttons, each sending
//! a pre-written line in one piece, with F1–F9 firing the first nine and a
//! small window to write them. The two lists are separate — a CW abbreviation
//! and a PSK sentence are not the same message, and a station that works both
//! wants each where it belongs — but the control is one thing, so it lives here
//! rather than twice over.

use crate::i18n::tr;
use eframe::egui::{self, RichText};
use sdroxide_types::{Command, CwMacro};

use crate::app::tx_gated;

/// The message row: one chip per filled-in button, F1–F9 for the first nine.
///
/// A press does the same three steps CALL CQ takes — abandon whatever is going
/// out, show the text in the box, send it as one piece — so a whole line is one
/// hand-off to the radio instead of one per word.
///
/// **F1–F9 fire only while nothing on screen holds the keyboard.** That
/// exclusion is the point: an operator part-way through typing has the caret in
/// the transmit box, and a function key that fired a message from under them
/// would put the wrong thing on the air. A function key that does nothing is
/// the cheap mistake; a message going out unbidden is the expensive one.
pub(in crate::app) fn macro_row(
    ui: &mut egui::Ui,
    cmds: &mut Vec<Command>,
    tx_ok: bool,
    my_call: &str,
    my_grid: &str,
    macros: &[CwMacro],
    text_tx: &mut String,
) {
    let typing = ui.memory(|m| m.focused().is_some());
    let mut fire: Option<usize> = None;
    if !typing && tx_ok {
        const KEYS: [egui::Key; 9] = [
            egui::Key::F1,
            egui::Key::F2,
            egui::Key::F3,
            egui::Key::F4,
            egui::Key::F5,
            egui::Key::F6,
            egui::Key::F7,
            egui::Key::F8,
            egui::Key::F9,
        ];
        for (i, key) in KEYS.iter().enumerate().take(macros.len()) {
            if ui.input_mut(|input| input.consume_key(egui::Modifiers::NONE, *key)) {
                fire = Some(i);
            }
        }
    }
    ui.add_space(4.0);
    ui.horizontal_wrapped(|ui| {
        // An empty row is one the operator has added and not filled in: it
        // keeps its place and its function key in the editor, but there is
        // nothing for a chip on the panel to send.
        for (i, m) in macros.iter().enumerate().filter(|(_, m)| !m.text.trim().is_empty()) {
            let hint = if i < 9 {
                tr!("macros-f-sends", i = i + 1, text = m.text.trim())
            } else {
                tr!("macros-sends", text = m.text.trim())
            };
            if tx_gated(ui, tx_ok, |ui| {
                crate::chrome::chip(ui, false, m.chip_label()).on_hover_text(&hint)
            })
            .clicked()
            {
                fire = Some(i);
            }
        }
    });
    if let Some(m) = fire.and_then(|i| macros.get(i)) {
        let call = if my_call.is_empty() { tr!("macros-nocall") } else { String::from(my_call) };
        let text = m.expand(&call, my_grid);
        if !text.trim().is_empty() {
            cmds.push(Command::DigiAbortTx);
            *text_tx = text.clone();
            cmds.push(Command::DigiTxText(text));
            cmds.push(Command::DigiTxActive(true));
        }
    }
}

/// The editor: one row per button, plus somewhere to add another.
///
/// A window rather than a fold-out inside the panel. The panel's receive pane
/// is sized against the real panel bottom, so anything that can grow under it
/// has to be budgeted for — and a table that grows a row every time ADD is
/// pressed cannot be. A window also survives the panel being short, which is
/// the case an operator setting these up on a laptop is in.
///
/// `title` and `id` name the window and its grid, so the two lists (CW and the
/// keyboard modes) can be open at once without sharing state. Returns whether
/// anything was edited, so the caller can persist its own config.
pub(in crate::app) fn macro_window(
    ctx: &egui::Context,
    title: &str,
    id: &str,
    open: &mut bool,
    macros: &mut Vec<CwMacro>,
) -> bool {
    if !*open {
        return false;
    }
    let mut is_open = true;
    let mut changed = false;
    let mut remove = None;
    let resp = egui::Window::new(title)
        .id(crate::layout::salted_id(ctx, id))
        .open(&mut is_open)
        .frame(crate::chrome::window_frame())
        .resizable(false)
        .default_width(crate::layout::window_w(ctx, 560.0))
        .show(ctx, |ui| {
            crate::chrome::window_body_bg(ui);
            // Claimed before the grid, because a `TextEdit` is never wider than
            // the space it is given however wide it asks to be — and an
            // auto-sized window takes its width from its widest child, which
            // without this is the paragraph below.
            ui.set_min_width(crate::layout::window_w(ctx, 520.0));
            ui.label(
                RichText::new(tr!("macros-buttons-note"))
                .size(10.5)
                .color(crate::theme::gray(150)),
            );
            ui.add_space(6.0);
            egui::Grid::new(format!("{id}-grid")).num_columns(4).spacing([6.0, 4.0]).show(
                ui,
                |ui| {
                    ui.label(RichText::new(tr!("macros-column-key")).size(10.0).color(crate::theme::gray(140)));
                    ui.label(RichText::new(tr!("macros-column-button")).size(10.0).color(crate::theme::gray(140)));
                    ui.label(RichText::new(tr!("macros-column-sends")).size(10.0).color(crate::theme::gray(140)));
                    ui.label("");
                    ui.end_row();
                    for (i, m) in macros.iter_mut().enumerate() {
                        ui.label(
                            RichText::new(if i < 9 {
                                format!("F{}", i + 1)
                            } else {
                                String::new()
                            })
                            .size(10.5)
                            .color(crate::theme::gray(150)),
                        );
                        // Sized rather than asked for: inside a `Grid` a
                        // `TextEdit`'s `desired_width` is clamped to a column
                        // that has not been measured yet, and both boxes come
                        // out a few characters wide.
                        changed |= crate::chrome::field_sized(
                            ui,
                            [80.0, 22.0],
                            egui::TextEdit::singleline(&mut m.label).hint_text(tr!("macros-label-hint")),
                        )
                        .changed();
                        changed |= crate::chrome::field_sized(
                            ui,
                            [320.0, 22.0],
                            egui::TextEdit::singleline(&mut m.text)
                                .hint_text("{MYCALL} DE {MYCALL}"),
                        )
                        .changed();
                        if crate::chrome::chip(ui, false, "×")
                            .on_hover_text(tr!("macros-remove-button"))
                            .clicked()
                        {
                            remove = Some(i);
                        }
                        ui.end_row();
                    }
                },
            );
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                let full = macros.len() >= CwMacro::MAX;
                if ui
                    .add_enabled(!full, egui::Button::new(tr!("macros-add")))
                    .on_disabled_hover_text(tr!("macros-most", value = CwMacro::MAX))
                    .clicked()
                {
                    macros.push(CwMacro::default());
                    changed = true;
                }
                ui.label(
                    RichText::new(tr!("macros-note-placeholders", mycall = "{MYCALL}", mygrid = "{MYGRID}"))
                        .size(10.5)
                        .color(crate::theme::gray(140)),
                );
            });
        });
    if let Some(r) = &resp {
        crate::chrome::paint_window_border(ctx, &r.response);
    }
    if let Some(i) = remove {
        macros.remove(i);
        changed = true;
    }
    *open = is_open;
    changed
}
