//! The Enigma window: a wartime faceplate that enciphers, and a solver that
//! breaks a copied ciphertext.
//!
//! Fork-only gimmick (`MORNING.md` §3). The machine and the solver live in
//! `sdroxide-enigma`; this is the panel — a Wehrmacht green board with white
//! lettering, the Steckerbrett across the top, the rotor windows and the
//! Modell/Rotor/Ring pickers below them, and the QWERTZ lampboard and keyboard.
//!
//! It holds no station state and touches nothing on the air: every control is
//! local, and the only thing that leaves is text the operator types. A copy of
//! a real Enigma's weakness — a letter never enciphers to itself — is shown, not
//! hidden, because it is the thing that makes the machine *this* machine.

use eframe::egui::{self, Color32, RichText};

use sdroxide_enigma::{Enigma, Plugboard, REFLECTORS, ROTORS, SolveParams, Variant, Wheel};

use crate::app::SdroxideApp;
use crate::i18n::tr;

/// The faceplate's palette. Fixed rather than themed: this is a *machine*, not
/// a screen, and an operator switching the app to a light theme should still
/// see the same war-era board. A forest-green field, off-white legends, and a
/// warm brass for the rotor drums.
const FIELD: Color32 = Color32::from_rgb(0x1B, 0x3A, 0x2A); // forest green
const FIELD_DARK: Color32 = Color32::from_rgb(0x12, 0x28, 0x1C);
const LEGEND: Color32 = Color32::from_rgb(0xF2, 0xEF, 0xE4); // off-white paint
const LEGEND_DIM: Color32 = Color32::from_rgb(0xA9, 0xB5, 0xA6);
const BRASS: Color32 = Color32::from_rgb(0xC9, 0xB7, 0x7A);
const LAMP: Color32 = Color32::from_rgb(0xF6, 0xE7, 0x9C); // warm bulb
const LAMP_DARK: Color32 = Color32::from_rgb(0x3A, 0x2F, 0x1C);

/// The Enigma window's state. Session-only, never persisted, never on the wire.
pub(in crate::app) struct EnigmaState {
    pub show: bool,
    variant: Variant,
    /// Rotor indices left to right (for M4 the first is a thin wheel).
    rotors: Vec<usize>,
    rings: Vec<u8>,
    starts: Vec<u8>,
    reflector: usize,
    plugboard: Plugboard,
    /// The first socket of a cable being run: set when a letter is clicked and
    /// cleared when the second is (or the same one again).
    pending_plug: Option<u8>,
    /// The typed message, and the result of running it through the machine.
    input: String,
    output: String,
    /// The crib text for the solver, and the last solution found.
    crib: String,
    solution: Option<String>,
    /// A note shown under the controls, e.g. why a key did nothing.
    note: Option<String>,
}

impl Default for EnigmaState {
    fn default() -> Self {
        EnigmaState {
            show: false,
            variant: Variant::EnigmaI,
            rotors: vec![0, 1, 2],
            rings: vec![0, 0, 0],
            starts: vec![0, 0, 0],
            reflector: 0,
            plugboard: Plugboard::new(),
            pending_plug: None,
            input: String::new(),
            output: String::new(),
            crib: String::new(),
            solution: None,
            note: None,
        }
    }
}

impl EnigmaState {
    /// Rebuild the machine from the current controls.
    fn machine(&self) -> Enigma {
        let wheels: Vec<Wheel> = (0..self.rotors.len())
            .map(|i| Wheel::new(self.rotors[i], self.rings[i], self.starts[i]))
            .collect();
        Enigma::new(self.variant, wheels, self.reflector).with_plugboard(self.plugboard)
    }

    /// Keep the vector lengths in step with the variant (3 vs 4 wheels).
    fn fit_wheels(&mut self) {
        let n = self.variant.wheel_count();
        while self.rotors.len() < n {
            // The M4's fourth slot is a thin wheel (Beta = 8); the standard
            // wheels fill the moving slots. Default to the historical pair.
            let next =
                if self.rotors.len() + 1 == n && n == 4 { 8 } else { self.rotors.len() as usize };
            self.rotors.push(next.min(ROTORS.len() - 1));
            self.rings.push(0);
            self.starts.push(0);
        }
        self.rotors.truncate(n);
        self.rings.truncate(n);
        self.starts.truncate(n);
    }

    fn encipher(&mut self) {
        let mut m = self.machine();
        self.output = m.encipher_text(&self.input);
    }

    /// Run the solver over the typed ciphertext. Blocking, but the search is
    /// bounded and the panel is a toy — a few seconds at worst.
    fn solve_now(&mut self) {
        let cipher = self.input.clone();
        if cipher.chars().filter(|c| c.is_ascii_alphabetic()).count() < 8 {
            self.note = Some(tr!("enigma-need-more"));
            return;
        }
        let params = SolveParams {
            variant: self.variant,
            crib: self.crib.clone(),
            search_rings: false,
            search_plugboard: true,
            m4_thin: None,
        };
        let sol = sdroxide_enigma::solve(&cipher, &params);
        // Adopt the recovered setting so the faceplate shows it.
        self.rotors = sol.rotors.clone();
        self.rings = sol.rings.clone();
        self.starts = sol.starts.clone();
        self.reflector = sol.reflector;
        self.plugboard = sol.plugboard;
        self.solution = Some(format!("{}  →  {}", sol.describe(), sol.plaintext));
    }
}

/// A small square key/legend chip in the faceplate palette.
fn face_chip(ui: &mut egui::Ui, label: &str, lit: bool, w: f32) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(w, w), egui::Sense::click());
    let bg = if lit { LAMP } else { FIELD_DARK };
    let fg = if lit { LAMP_DARK } else { LEGEND };
    ui.painter().rect_filled(rect, egui::CornerRadius::same(3), bg);
    ui.painter().rect_stroke(
        rect,
        egui::CornerRadius::same(3),
        egui::Stroke::new(1.0, LEGEND_DIM),
        egui::StrokeKind::Inside,
    );
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        label,
        egui::FontId::monospace(15.0),
        fg,
    );
    resp
}

impl SdroxideApp {
    pub(in crate::app) fn enigma_window(&mut self, ctx: &egui::Context) {
        if !self.enigma.show {
            return;
        }
        let show = self.tool_window(ctx, "enigma", &tr!("enigma-title"), [460.0, 640.0], true, |me, ui| {
            me.enigma_body(ui);
        });
        self.enigma.show = show;
    }

    /// The faceplate: everything painted onto one green field.
    fn enigma_body(&mut self, ui: &mut egui::Ui) {
        let plate = egui::Frame::new()
            .fill(FIELD)
            .inner_margin(egui::Margin::same(10))
            .corner_radius(egui::CornerRadius::same(4));
        plate.show(ui, |ui| {
            ui.set_width(ui.available_width());
            self.enigma_plugboard(ui);
            ui.add_space(8.0);
            self.enigma_rotors(ui);
            ui.add_space(8.0);
            self.enigma_lampboard(ui);
            ui.add_space(10.0);
            self.enigma_inputs(ui);
        });
    }

    /// The Steckerbrett: 26 sockets in the historical two-row layout, click one
    /// then another to run a cable.
    fn enigma_plugboard(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("STECKERBRETT").size(11.0).strong().color(LEGEND_DIM));
        // Row 1: A–J plus Z (the historical odd socket); row 2: K–Y. Kept as
        // the wartime board lays them out.
        let rows: [&[u8]; 2] = [b"ABCDEFGHIJZ", b"KLMNOPQRSTUVWXY"];
        for row in rows {
            ui.horizontal(|ui| {
                for &c in row {
                    let letter = (c as char).to_string();
                    let patched = self.enigma.plugboard.through(c - b'A') != c - b'A';
                    let r = face_chip(ui, &letter, patched, 22.0);
                    if r.clicked() {
                        self.toggle_plug(c - b'A');
                    }
                }
            });
        }
        let pairs = self.enigma.plugboard.to_string_pairs();
        ui.label(
            RichText::new(if pairs.is_empty() { tr!("enigma-no-cables") } else { pairs })
                .size(12.0)
                .monospace()
                .color(BRASS),
        );
    }

    /// Two letters in a row had no partner yet → patch them; a patched letter
    /// unplugs its cable. This mirrors pulling a cable and moving it.
    fn toggle_plug(&mut self, a: u8) {
        let pb = &mut self.enigma.plugboard;
        if pb.through(a) != a {
            pb.unplug(a);
        } else if let Some(pending) = self.enigma.pending_plug.take() {
            if pending != a {
                pb.plug(pending, a);
            }
        } else {
            self.enigma.pending_plug = Some(a);
        }
    }

    /// The rotor windows and the Modell / Rotor / Ring pickers.
    fn enigma_rotors(&mut self, ui: &mut egui::Ui) {
        // Modell (variant) and reflector.
        ui.horizontal(|ui| {
            ui.label(RichText::new("Modell").size(11.0).color(LEGEND_DIM));
            for (v, name) in [(Variant::EnigmaI, "Enigma I"), (Variant::M4, "M4")] {
                if ui
                    .selectable_label(self.enigma.variant == v, RichText::new(name).color(LEGEND))
                    .clicked()
                    && self.enigma.variant != v
                {
                    self.enigma.variant = v;
                    self.enigma.fit_wheels();
                }
            }
            ui.separator();
            ui.label(RichText::new("UKW").size(11.0).color(LEGEND_DIM));
            for (i, r) in REFLECTORS.iter().enumerate() {
                if ui
                    .selectable_label(
                        self.enigma.reflector == i,
                        RichText::new(r.name).color(LEGEND),
                    )
                    .clicked()
                {
                    self.enigma.reflector = i;
                }
            }
        });
        ui.add_space(4.0);

        // One column per wheel, laid out as the physical windows.
        ui.horizontal(|ui| {
            for i in 0..self.enigma.rotors.len() {
                ui.vertical(|ui| {
                    let thin = ROTORS[self.enigma.rotors[i]].thin;
                    ui.label(
                        RichText::new(if thin { tr!("enigma-thin") } else { tr!("enigma-rotor") })
                            .size(9.5)
                            .color(LEGEND_DIM),
                    );
                    // Rotor picker.
                    egui::ComboBox::from_id_salt(("rotor", i))
                        .selected_text(
                            RichText::new(ROTORS[self.enigma.rotors[i]].name).color(LEGEND),
                        )
                        .width(64.0)
                        .show_ui(ui, |ui| {
                            for (ri, r) in ROTORS.iter().enumerate() {
                                // A thin wheel only belongs in the M4's leftmost
                                // slot; a standard wheel never does.
                                let slot_is_thin_left =
                                    i == 0 && self.enigma.variant == Variant::M4;
                                if r.thin != slot_is_thin_left {
                                    continue;
                                }
                                ui.selectable_value(&mut self.enigma.rotors[i], ri, r.name);
                            }
                        });
                    // The window: the start letter, on a brass drum.
                    let start = self.enigma.starts[i];
                    let win =
                        face_chip(ui, &(b'A' + start).to_string().to_uppercase(), false, 40.0);
                    // Clicking the window advances the wheel one — the operator
                    // setting Grundstellung by hand.
                    if win.clicked() {
                        self.enigma.starts[i] = (self.enigma.starts[i] + 1) % 26;
                    }
                    win.on_hover_text(tr!("enigma-window-hint"));
                    // Ringstellung.
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Ring").size(9.5).color(LEGEND_DIM));
                        if ui.small_button("−").clicked() {
                            self.enigma.rings[i] = (self.enigma.rings[i] + 25) % 26;
                        }
                        ui.label(
                            RichText::new((b'A' + self.enigma.rings[i]).to_string())
                                .size(13.0)
                                .monospace()
                                .color(LEGEND),
                        );
                        if ui.small_button("+").clicked() {
                            self.enigma.rings[i] = (self.enigma.rings[i] + 1) % 26;
                        }
                    });
                });
                ui.add_space(6.0);
            }
        });
    }

    /// The QWERTZ lampboard (top) and keyboard (bottom). Clicking a key runs a
    /// letter through the machine and lights the result on the lampboard.
    fn enigma_lampboard(&mut self, ui: &mut egui::Ui) {
        const QWERTZ: [&str; 3] = ["QWERTZUIO", "ASDFGHJK", "PYXCVBNML"];
        // What the last key lit, so the lampboard shows it.
        let lit = self.enigma.output.chars().last().filter(|c| c.is_ascii_alphabetic());
        for (row, _label) in [(0, "lamp"), (1, "key")] {
            let _ = _label;
            ui.horizontal(|ui| {
                for line in QWERTZ {
                    for ch in line.chars() {
                        if row == 0 {
                            let on = lit == Some(ch);
                            let _ = face_chip(ui, &ch.to_string(), on, 26.0);
                        } else if face_chip(ui, &ch.to_string(), false, 26.0).clicked() {
                            self.enigma.input.push(ch);
                            self.enigma.encipher();
                        }
                    }
                }
            });
            if row == 0 {
                ui.label(RichText::new("LAMPENFELD").size(9.5).color(LEGEND_DIM));
            } else {
                ui.label(RichText::new("TASTATUR").size(9.5).color(LEGEND_DIM));
            }
            ui.add_space(4.0);
        }
    }

    /// The plaintext/ciphertext boxes and the solver.
    fn enigma_inputs(&mut self, ui: &mut egui::Ui) {
        ui.label(RichText::new("KLARTEXT / GEHEIMTEXT").size(11.0).color(LEGEND_DIM));
        let resp = ui.add(
            egui::TextEdit::singleline(&mut self.enigma.input)
                .desired_width(ui.available_width())
                .text_color(LEGEND)
                .hint_text(tr!("enigma-input-hint")),
        );
        if resp.changed() {
            self.enigma.encipher();
        }
        ui.label(
            RichText::new(if self.enigma.output.is_empty() {
                "—".into()
            } else {
                self.enigma.output.clone()
            })
            .size(15.0)
            .monospace()
            .color(BRASS),
        );
        ui.horizontal(|ui| {
            if ui.button(tr!("enigma-clear")).clicked() {
                self.enigma.input.clear();
                self.enigma.output.clear();
            }
            if ui.button(tr!("enigma-copy")).clicked() {
                // Feed the result back as the input, so the symmetry is visible.
                self.enigma.input = self.enigma.output.replace(' ', "");
                self.enigma.encipher();
            }
        });

        ui.add_space(8.0);
        ui.separator();
        ui.label(
            RichText::new(tr!("enigma-solve-note"))
            .size(10.5)
            .color(LEGEND_DIM),
        );
        ui.horizontal(|ui| {
            ui.label(RichText::new(tr!("enigma-crib")).size(11.0).color(LEGEND_DIM));
            ui.add(
                egui::TextEdit::singleline(&mut self.enigma.crib)
                    .desired_width(120.0)
                    .text_color(LEGEND),
            );
            if ui.button(tr!("enigma-solve")).clicked() {
                self.enigma.solve_now();
            }
        });
        if let Some(sol) = &self.enigma.solution {
            ui.label(RichText::new(sol).size(11.0).monospace().color(BRASS));
        }

        // The machine's own weakness, said out loud because it is the point.
        ui.add_space(4.0);
        ui.label(
            RichText::new(tr!("enigma-flaw"))
            .size(10.0)
            .color(LEGEND_DIM),
        );
    }
}
