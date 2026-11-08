//! The WEATHER SATELLITES window: the picture a NOAA APT satellite sends on
//! 137 MHz, drawn as the lines come in.
//!
//! Like SONDE it is a view, not a radio mode: the decoder runs off its own
//! downconversion of the sample window (see `sdroxide_wxsat`) and the radio
//! keeps its mode. State comes from [`sdroxide_types::WxsatStatus`]; the
//! picture is [`sdroxide_types::WxsatPicture`], built from the lines it carries.

use crate::i18n::tr;
use eframe::egui::{self, RichText};
use sdroxide_types::{
    Command, WXSAT_LINE_PIXELS, WxsatPicture, WxsatSettings, wxsat_window_covers,
};

/// The APT satellites still on the air: name and downlink. Names and units are
/// not translated.
const SATELLITES: [(&str, f64); 3] =
    [("NOAA-15", 137_620_000.0), ("NOAA-18", 137_912_500.0), ("NOAA-19", 137_100_000.0)];

/// Where each image strip starts in a line, and how wide it is.
const STRIP_A: usize = 86;
const STRIP_B: usize = 1040 + 86;
const STRIP_W: usize = 909;
/// Dark pixels between the two strips.
const GAP: usize = 6;
/// Width of the picture as drawn: both strips and the gap.
pub(in crate::app) const PICTURE_W: usize = 2 * STRIP_W + GAP;
/// Grey of a line that never arrived.
const MISSING: u8 = 40;
const WINDOW_SIZE: [f32; 2] = [860.0, 640.0];

/// The picture as grey pixels, both image strips side by side, each line one
/// row. `flip` turns it through half a turn (a satellite flying south is
/// drawn upside down). Returns the size and the pixels.
pub(in crate::app) fn picture_gray(pic: &WxsatPicture, flip: bool) -> ([usize; 2], Vec<u8>) {
    let h = pic.lines.len();
    let mut out = vec![MISSING; PICTURE_W * h];
    for (y, line) in pic.lines.iter().enumerate() {
        if line.len() != WXSAT_LINE_PIXELS {
            continue;
        }
        let row = &mut out[y * PICTURE_W..(y + 1) * PICTURE_W];
        row[..STRIP_W].copy_from_slice(&line[STRIP_A..STRIP_A + STRIP_W]);
        row[STRIP_W..STRIP_W + GAP].fill(0);
        row[STRIP_W + GAP..].copy_from_slice(&line[STRIP_B..STRIP_B + STRIP_W]);
    }
    if flip {
        out.reverse();
    }
    ([PICTURE_W, h], out)
}

/// The commands "Tune here" sends: leave a mode that holds the front end
/// elsewhere, switch the decoder on if it is off, and bring the dial to the
/// channel so that it is inside the sample window.
pub(in crate::app) fn tune_commands(
    cfg: &WxsatSettings,
    vfo: sdroxide_types::Vfo,
    mode: sdroxide_types::Mode,
) -> Vec<Command> {
    let mut out = Vec::new();
    if mode.is_wideband_lane() || mode.is_hfdl() {
        out.push(Command::SetModeListen {
            rx: sdroxide_types::RxId::Main,
            mode: sdroxide_types::Mode::Nfm,
        });
    }
    if !cfg.enabled {
        out.push(Command::SetWxsatConfig(WxsatSettings { enabled: true, ..*cfg }));
    }
    out.push(Command::SetVfo { vfo, hz: cfg.frequency_hz });
    out
}

/// The name of the satellite on `hz`, or the frequency in MHz.
fn channel_name(hz: f64) -> String {
    SATELLITES
        .iter()
        .find(|(_, f)| (f - hz).abs() < 1_000.0)
        .map_or_else(|| format!("{:.4} MHz", hz / 1e6), |(n, f)| format!("{n}  {:.4} MHz", f / 1e6))
}

impl super::SdroxideApp {
    pub(in crate::app) fn wxsat_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        if !self.show_wxsat {
            return;
        }
        let mut out = Vec::new();
        let open = self.tool_window(ctx, "wxsat", &tr!("wxsat-title"), WINDOW_SIZE, true, |me, ui| {
            me.wxsat_body(ui, &mut out);
        });
        cmds.extend(out);
        self.show_wxsat = open;
    }

    fn wxsat_body(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        let cfg = self.state.wxsat;
        let mut edit = cfg;
        ui.horizontal_wrapped(|ui| {
            ui.checkbox(&mut edit.enabled, tr!("wxsat-decode"));
            ui.label(tr!("wxsat-channel"));
            egui::ComboBox::from_id_salt("wxsat-channel")
                .selected_text(channel_name(edit.frequency_hz))
                .show_ui(ui, |ui| {
                    for (name, hz) in SATELLITES {
                        let label = format!("{name}  {:.4} MHz", hz / 1e6);
                        ui.selectable_value(&mut edit.frequency_hz, hz, label);
                    }
                });
            if ui.button(tr!("wxsat-tune")).on_hover_text(tr!("wxsat-tune-hint")).clicked() {
                cmds.extend(tune_commands(
                    &edit,
                    self.state.active_vfo,
                    self.state.rx[0].mode,
                ));
                edit.enabled = true;
            }
        });
        if edit != cfg && !cmds.iter().any(|c| matches!(c, Command::SetWxsatConfig(_))) {
            cmds.push(Command::SetWxsatConfig(edit));
        }

        // The status line.
        let st = self.wxsat_status.as_deref();
        ui.horizontal_wrapped(|ui| {
            let dim = |s: String| RichText::new(s).size(11.0).color(crate::theme::gray(160));
            match st {
                Some(st) if st.running => {
                    ui.label(dim(tr!("wxsat-level", db = format!("{:.0}", st.level_dbfs))));
                    ui.label(dim(if st.locked { tr!("wxsat-locked") } else { tr!("wxsat-searching") }));
                    ui.label(dim(tr!("wxsat-lines", n = st.total_lines)));
                }
                _ => {
                    ui.label(dim(tr!("wxsat-state-off")));
                }
            }
        });
        if cfg.enabled
            && !wxsat_window_covers(self.state.center_hz, self.state.sample_rate, cfg.frequency_hz)
        {
            ui.label(
                RichText::new(tr!("wxsat-outside-window"))
                    .size(11.0)
                    .color(crate::theme::gray(200)),
            );
        }

        // The picture's own controls.
        ui.horizontal(|ui| {
            if ui.button(tr!("wxsat-clear")).clicked() {
                self.wxsat_picture.clear();
                self.wxsat_tex_lines = usize::MAX;
            }
            let before = self.wxsat_flip;
            ui.checkbox(&mut self.wxsat_flip, tr!("wxsat-flip")).on_hover_text(tr!("wxsat-flip-hint"));
            if before != self.wxsat_flip {
                self.wxsat_tex_lines = usize::MAX;
            }
            let have = !self.wxsat_picture.lines.is_empty();
            if ui.add_enabled(have, egui::Button::new(tr!("wxsat-save"))).clicked() {
                let ([w, h], gray) = picture_gray(&self.wxsat_picture, self.wxsat_flip);
                let rgb: Vec<u8> = gray.iter().flat_map(|&g| [g, g, g]).collect();
                if let Some(png) = crate::sstv::encode_png(&rgb, w as u16, h as u16) {
                    crate::download::save_as(&wxsat_file_name(), &png, crate::download::Mime::Png);
                }
            }
        });
        ui.separator();

        if self.wxsat_picture.lines.is_empty() {
            ui.label(RichText::new(tr!("wxsat-waiting")).color(crate::theme::gray(160)));
            return;
        }
        // The texture follows the picture when it has grown.
        let n = self.wxsat_picture.lines.len();
        if self.wxsat_tex_lines != n || self.wxsat_tex.is_none() {
            let ([w, h], gray) = picture_gray(&self.wxsat_picture, self.wxsat_flip);
            let img = egui::ColorImage::from_gray([w, h], &gray);
            match self.wxsat_tex.as_mut() {
                Some(t) => t.set(img, egui::TextureOptions::LINEAR),
                None => {
                    self.wxsat_tex =
                        Some(ui.ctx().load_texture("wxsat-picture", img, egui::TextureOptions::LINEAR));
                }
            }
            self.wxsat_tex_lines = n;
        }
        if let Some(tex) = &self.wxsat_tex {
            let size = tex.size_vec2();
            let scale = ui.available_width() / size.x;
            egui::ScrollArea::vertical().stick_to_bottom(!self.wxsat_flip).show(ui, |ui| {
                ui.image(egui::load::SizedTexture::new(tex.id(), size * scale));
            });
        }
    }
}

/// A dated name for the saved picture.
fn wxsat_file_name() -> String {
    format!("wxsat-{}.png", crate::time::now_unix())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn line(v: u8) -> Vec<u8> {
        let mut l = vec![0u8; WXSAT_LINE_PIXELS];
        l[STRIP_A..STRIP_A + STRIP_W].fill(v);
        l[STRIP_B..STRIP_B + STRIP_W].fill(255 - v);
        l
    }

    #[test]
    fn the_two_strips_sit_side_by_side_with_a_gap() {
        let pic = WxsatPicture { pass: 1, lines: vec![line(10)], dropped: 0 };
        let ([w, h], g) = picture_gray(&pic, false);
        assert_eq!((w, h), (PICTURE_W, 1));
        assert_eq!(g[0], 10);
        assert_eq!(g[STRIP_W], 0);
        assert_eq!(g[STRIP_W + GAP], 245);
        assert_eq!(g[PICTURE_W - 1], 245);
    }

    #[test]
    fn a_missing_line_is_grey_and_flip_turns_the_picture_half_a_turn() {
        let pic = WxsatPicture { pass: 1, lines: vec![line(10), Vec::new()], dropped: 0 };
        let (_, g) = picture_gray(&pic, false);
        assert_eq!(g[PICTURE_W], MISSING);
        let (_, f) = picture_gray(&pic, true);
        assert_eq!(f[0], MISSING);
        // The first line is now last, and its pixels run backwards.
        assert_eq!(f[PICTURE_W], 245);
        assert_eq!(f[2 * PICTURE_W - 1], 10);
    }

    #[test]
    fn tune_here_leaves_a_wideband_mode_and_switches_the_decoder_on() {
        let cfg = WxsatSettings::default();
        let cmds = tune_commands(&cfg, sdroxide_types::Vfo::A, sdroxide_types::Mode::Adsb);
        assert!(matches!(cmds[0], Command::SetModeListen { .. }));
        assert!(cmds.iter().any(|c| matches!(c, Command::SetWxsatConfig(s) if s.enabled)));
        assert!(matches!(cmds.last(), Some(Command::SetVfo { hz, .. }) if *hz == cfg.frequency_hz));
        // Already on, in an ordinary mode: only the dial moves.
        let on = WxsatSettings { enabled: true, ..cfg };
        let cmds = tune_commands(&on, sdroxide_types::Vfo::A, sdroxide_types::Mode::Nfm);
        assert_eq!(cmds.len(), 1);
    }

    #[test]
    fn channel_names_know_the_satellites() {
        assert!(channel_name(137_100_000.0).starts_with("NOAA-19"));
        assert_eq!(channel_name(137_500_000.0), "137.5000 MHz");
    }
}
