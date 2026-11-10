//! **Radio Classic**: a classic-radio face for broadcast listening, with the
//! FM radio (WFM + RDS) as the first thing it does well.
//!
//! One framed panel in three columns of equal height. Docked, it takes the slot
//! under the waterfall that a digital mode's panel takes (and CW's), at the same
//! 46 % of the column; detached, it is an ordinary tool window. The colours all
//! come from the theme in force.
//!
//! | column | content |
//! | --- | --- |
//! | left | bands, modes, MEM and PROGRAMS |
//! | middle | the mode, a mini spectrum, the S-meter, BW/STEP/AGC/SQL, the VU |
//! | right | clocks, the VFO, the station name, RadioText, RDS chips, audio bars |
//!
//! The plan and the decisions are in `docs/internal/RADIO-CLASSIC-PLAN.md`.

use crate::i18n::{Tr, tr};
use std::collections::VecDeque;

use eframe::egui::{self, Align, Align2, Color32, FontId, Pos2, Rect, RichText, Stroke, StrokeKind, vec2};
use sdroxide_types::{
    AUDIO_BANDS, AgcMode, Band, Command, Mode, RdsData, RxId, SQUELCH_CLOSED_DB, SQUELCH_OPEN_DB,
    BroadcastStation, MemoryChannel, broadcast, byte_to_unit, pty_name,
};

use crate::app::SdroxideApp;
use crate::theme;
use crate::widgets::vu_meter::{self, VuBallistics};

/// Where the panel is drawn this frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::app) enum RadioClassicPlace {
    /// Not in use.
    Hidden,
    /// Under the waterfall, in the slot a mode panel would take.
    Docked,
    /// In a window of its own.
    Window,
}

/// Where Radio Classic goes: a pure rule, so it can be pinned without a frame.
///
/// * `on` — switched on from EXTRAS; `detached` — the Detachable page's switch.
/// * A detached panel is a window, whatever else is true.
/// * Switched on, it docks in the modes that have nothing under the waterfall.
///   Where the slot is taken — a digital mode's panel, DAB, CW — or on a phone,
///   where there is no room for it, it is a window instead, so that switching it
///   on never does nothing.
pub(in crate::app) fn place(
    on: bool,
    detached: bool,
    mode: Mode,
    phone: bool,
) -> RadioClassicPlace {
    if detached {
        RadioClassicPlace::Window
    } else if !on {
        RadioClassicPlace::Hidden
    } else if phone || mode.has_bottom_panel() || mode == Mode::Cw {
        RadioClassicPlace::Window
    } else {
        RadioClassicPlace::Docked
    }
}

/// The picture of the arrow in the panel's corner, or `None` where it has
/// nowhere to go: a panel that is a window only because the slot under the
/// waterfall is taken has nowhere to dock, and a phone has no room to float.
pub(in crate::app) fn detach_icon(
    placed: RadioClassicPlace,
    detached: bool,
    phone: bool,
) -> Option<crate::chrome::WindowIcon> {
    use crate::chrome::WindowIcon;
    match placed {
        RadioClassicPlace::Docked if !phone => Some(WindowIcon::Out),
        RadioClassicPlace::Window if detached => Some(WindowIcon::Dock),
        _ => None,
    }
}

// ───────────────────────────────────────────────────────────────── the bands

/// The groups of the left column's first row.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(in crate::app) enum Group {
    #[default]
    Broadcast,
    Ham,
    Air,
    Marine,
    Cb,
}

impl Group {
    const ALL: [(Group, &'static str); 5] = [
        (Group::Broadcast, "BROADCAST"),
        (Group::Ham, "HAM"),
        (Group::Air, "AIR"),
        (Group::Marine, "MARINE"),
        (Group::Cb, "CB"),
    ];
}

/// One button of the band rows: where it tunes and when it reads as selected.
#[derive(Clone, Debug, PartialEq)]
struct Preset {
    label: String,
    /// The range the button stands for: it is lit while the dial is inside.
    lo_hz: f64,
    hi_hz: f64,
    /// Where a click goes, and in which mode.
    tune_hz: f64,
    mode: Mode,
    /// A band the engine already knows (its own band stack and rules), when
    /// there is one; the click asks for that rather than for a bare frequency.
    band: Option<Band>,
}

impl Preset {
    fn range(label: &str, lo_mhz: f64, hi_mhz: f64, tune_mhz: f64, mode: Mode) -> Self {
        Self {
            label: label.into(),
            lo_hz: lo_mhz * 1e6,
            hi_hz: hi_mhz * 1e6,
            tune_hz: tune_mhz * 1e6,
            mode,
            band: None,
        }
    }

    fn channel(label: &str, mhz: f64, mode: Mode) -> Self {
        Self::range(label, mhz - 0.0025, mhz + 0.0025, mhz, mode)
    }

    fn band(label: &str, band: Band, lo_mhz: f64, hi_mhz: f64, mode: Mode) -> Self {
        Self {
            band: Some(band),
            ..Self::range(label, lo_mhz, hi_mhz, (lo_mhz + hi_mhz) / 2.0, mode)
        }
    }

    fn holds(&self, hz: f64) -> bool {
        (self.lo_hz..=self.hi_hz).contains(&hz)
    }
}

/// The longwave, mediumwave and FM row of the broadcast group.
fn broadcast_row() -> Vec<Preset> {
    vec![
        Preset::band("LW", Band::Lw, 0.1485, 0.2835, Mode::Am),
        Preset::range("MW EU", 0.531, 1.602, 0.999, Mode::Am),
        Preset::range("MW US", 0.530, 1.700, 1.010, Mode::Am),
        Preset::range("FM 87.5–108", 87.5, 108.0, 100.0, Mode::Wfm),
        Preset::range("FM ext 64–87.5", 64.0, 87.5, 76.0, Mode::Wfm),
    ]
}

/// The shortwave broadcast bands, `120 … 11`.
fn shortwave_row() -> Vec<Preset> {
    broadcast::METRE_BANDS
        .iter()
        .map(|&(name, lo_khz, hi_khz)| {
            let mid_khz = ((lo_khz + hi_khz) / 2.0 / 5.0).round() * 5.0;
            Preset::range(
                name.trim_end_matches('m'),
                lo_khz / 1e3,
                hi_khz / 1e3,
                mid_khz / 1e3,
                Mode::Am,
            )
        })
        .collect()
}

/// The rows a group offers, as `(caption, buttons)`.
fn group_rows(group: Group) -> Vec<(&'static str, Vec<Preset>)> {
    match group {
        Group::Broadcast => {
            vec![("OL / OM · FM", broadcast_row()), ("OC · SHORTWAVE", shortwave_row())]
        }
        Group::Ham => vec![(
            "AMATEUR BANDS",
            [
                (Band::M160, 1.8, 2.0, Mode::Lsb),
                (Band::M80, 3.5, 3.8, Mode::Lsb),
                (Band::M60, 5.30, 5.41, Mode::Usb),
                (Band::M40, 7.0, 7.2, Mode::Lsb),
                (Band::M30, 10.1, 10.15, Mode::Cw),
                (Band::M20, 14.0, 14.35, Mode::Usb),
                (Band::M17, 18.068, 18.168, Mode::Usb),
                (Band::M15, 21.0, 21.45, Mode::Usb),
                (Band::M12, 24.89, 24.99, Mode::Usb),
                (Band::M10, 28.0, 29.7, Mode::Usb),
                (Band::M6, 50.0, 54.0, Mode::Usb),
                (Band::M2, 144.0, 146.0, Mode::Nfm),
                (Band::M70, 430.0, 440.0, Mode::Nfm),
            ]
            .into_iter()
            .map(|(b, lo, hi, m)| Preset::band(b.label(), b, lo, hi, m))
            .collect(),
        )],
        Group::Air => vec![(
            "AIRBAND",
            vec![
                Preset::band("AIR 118–137", Band::Air, 118.0, 137.0, Mode::Am),
                Preset::band("MIL 225–400", Band::Mil, 225.0, 400.0, Mode::Am),
            ],
        )],
        Group::Marine => vec![(
            "MARINE VHF · MF",
            vec![
                Preset::channel("16", 156.800, Mode::Nfm),
                Preset::channel("9", 156.450, Mode::Nfm),
                Preset::channel("6", 156.300, Mode::Nfm),
                Preset::channel("12", 156.600, Mode::Nfm),
                Preset::channel("13", 156.650, Mode::Nfm),
                Preset::channel("14", 156.700, Mode::Nfm),
                Preset::channel("70 DSC", 156.525, Mode::Nfm),
                Preset::channel("2182 kHz", 2.182, Mode::Usb),
            ],
        )],
        Group::Cb => vec![(
            "CITIZENS BAND",
            vec![
                Preset::band("11 M", Band::M11, 26.965, 27.405, Mode::Am),
                Preset::channel("CH 9", 27.065, Mode::Am),
                Preset::channel("CH 19", 27.185, Mode::Am),
            ],
        )],
    }
}

/// The group chips' captions, translated at draw time (the table keeps English).
fn group_tr(label: &str) -> String {
    match label {
        "BROADCAST" => tr!("rc-group-broadcast"),
        "HAM" => tr!("rc-group-ham"),
        "AIR" => tr!("rc-group-air"),
        "MARINE" => tr!("rc-group-marine"),
        other => other.to_string(),
    }
}

/// A row caption of the band groups, translated at draw time.
fn caption_tr(caption: &str) -> String {
    match caption {
        "OL / OM · FM" => tr!("rc-cap-lw-mw-fm"),
        "OC · SHORTWAVE" => tr!("rc-cap-shortwave"),
        "AMATEUR BANDS" => tr!("rc-cap-amateur-bands"),
        "AIRBAND" => tr!("rc-cap-airband"),
        "MARINE VHF · MF" => tr!("rc-cap-marine"),
        "CITIZENS BAND" => tr!("rc-cap-cb"),
        other => other.to_string(),
    }
}

/// A preset button's label, translated at draw time where it has a word in it.
fn preset_tr(label: &str) -> String {
    match label {
        "FM ext 64–87.5" => tr!("rc-preset-fm-ext"),
        other => other.to_string(),
    }
}

/// The modes the left column offers.
const MODES: [(&str, Mode); 6] = [
    ("WFM", Mode::Wfm),
    ("FM", Mode::Nfm),
    ("AM", Mode::Am),
    ("USB", Mode::Usb),
    ("LSB", Mode::Lsb),
    ("CW", Mode::Cw),
];

// ──────────────────────────────────────────────────────────── what is on show

/// What the right-hand display writes about the station on the dial: the big
/// line, an optional small line above it and small lines under it.
#[derive(Clone, Debug, Default, PartialEq)]
struct StationLines {
    /// The big line, in capitals.
    big: String,
    /// The small line over it.
    above: Option<String>,
    /// The small lines under it, in order.
    below: Vec<String>,
}

/// What the big line shows: the station's own name on FM, the memory's or the
/// schedule's everywhere else (and on FM too while the station sends no name).
///
/// * A **schedule** row is its metre band and name on the big line, with what
///   the table says about the transmission — language, country, site — above.
/// * A **memory** is its name, with its frequency and mode under it.
/// * A name that carries a bracket ("Klara (Sint-Pieters-Leeuw)") keeps the
///   bracket in small type under the name.
fn station_lines(
    mode: Mode,
    ps: Option<&str>,
    memory: Option<&MemoryChannel>,
    schedule: Option<&BroadcastStation>,
) -> Option<StationLines> {
    let ps = ps.map(str::trim).filter(|s| !s.is_empty());
    let named = |name: &str| {
        let (main, note) = split_name(name);
        StationLines {
            big: main.to_uppercase(),
            above: None,
            below: note.map(str::to_string).into_iter().collect(),
        }
    };
    if let Some(ps) = ps.filter(|_| mode == Mode::Wfm) {
        return Some(named(ps));
    }
    if let Some(m) = memory {
        let mut lines = named(&m.name);
        lines.below.push(format!("{:.6} MHz {}", m.freq_hz / 1e6, mode_label(m.mode)));
        return Some(lines);
    }
    schedule.map(|s| {
        let big = match broadcast::metre_band(s.freq_khz) {
            Some(band) => format!("{band} {}", s.name.to_uppercase()),
            None => s.name.to_uppercase(),
        };
        let above = [s.lang.as_str(), s.country.as_str(), s.site.as_str()]
            .iter()
            .map(|p| p.trim())
            .filter(|p| !p.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        StationLines { big, above: (!above.is_empty()).then_some(above), below: Vec::new() }
    })
}

/// The mode as the display writes it.
fn mode_label(mode: Mode) -> String {
    MODES
        .iter()
        .find(|(_, m)| *m == mode)
        .map_or_else(|| format!("{mode:?}").to_uppercase(), |(l, _)| (*l).to_string())
}

/// The width of the three columns for a panel `w` wide, with the gap between
/// them: the left one a quarter, the middle (mini spectrum and VU) a quarter,
/// the right one the rest — and the middle never narrower than the VU plus a
/// usable spectrum.
fn column_widths(w: f32) -> (f32, f32, f32) {
    const GAP: f32 = 8.0;
    let usable = (w - 2.0 * GAP).max(300.0);
    let left = (usable * 0.265).max(150.0);
    let mid = (usable * 0.25).max(200.0);
    let right = (usable - left - mid).max(120.0);
    (left, mid, right)
}

/// The S-meter: how full it reads, `0.0..=1.0`, for a level in dBm. S0 is
/// -127 dBm and +60 dB over S9 (-13 dBm) is the top, as on the scale below it.
fn smeter_fill(s_dbm: f32) -> f32 {
    ((s_dbm + 127.0) / 114.0).clamp(0.0, 1.0)
}

/// `S6`, `S9+20` — the reading a listener says aloud.
fn s_reading(s_dbm: f32) -> String {
    let over = s_dbm + 73.0;
    if over >= 0.0 {
        if over < 3.0 { "S9".into() } else { format!("S9+{}", (over / 5.0).round() as i32 * 5) }
    } else {
        format!("S{}", (9.0 + over / 6.0).max(0.0) as u8)
    }
}

/// Where Radio Classic tunes the very first time, in WFM.
const FIRST_HZ: f64 = 100e6;
/// The detach arrow in the panel's corner.
const ICON_W: f32 = 22.0;
const ICON_H: f32 = 16.0;
/// After it takes the receiver, how long the station's state is not believed
/// over the panel's own: a message sent before the commands were applied can
/// still be on its way.
const ENTER_GRACE_S: f64 = 1.5;
/// The frequency it listens on is written to disk at most this often.
const SAVE_EVERY_S: f64 = 2.0;

/// Whether `mode` is one Radio Classic listens in. Anything else — a digital
/// mode, DAB, ADS-B — is another receiver's job.
fn is_radio_mode(mode: Mode) -> bool {
    MODES.iter().any(|(_, m)| *m == mode)
        || matches!(mode, Mode::Sam | Mode::Cquam | Mode::Dsb | Mode::Isb)
}

/// How long ago a reading may be and still count as the sound that is playing.
const LEVEL_FRESH_S: f32 = 0.4;
/// Rows kept by the mini waterfall, and the bins in each.
const WF_ROWS: usize = 400;
const WF_BINS: usize = 96;
/// Of every [`WF_OF`] spectrum frames, [`WF_KEEP`] become a row: three in four,
/// so the scope runs one and a half times as fast as it did at one in two. The
/// ten times as many rows make the picture finer, not faster.
const WF_KEEP: u32 = 3;
const WF_OF: u32 = 4;
/// The least span the scope zooms to, in Hz — a CW filter is a few hundred.
const SCOPE_MIN_SPAN_HZ: f64 = 1_000.0;
/// Room left around the filter's edges, as a fraction of its width.
const SCOPE_MARGIN: f64 = 0.1;

/// The part of the spectrum the scope draws, `(lo_hz, span_hz)`: the receive
/// filter with a margin round it, so the bandwidth in use fills the width —
/// all of a broadcast channel on WFM, a few kHz of a voice filter on SSB.
fn scope_window(hz: f64, filter_lo: f32, filter_hi: f32) -> (f64, f64) {
    let (lo, hi) = (f64::from(filter_lo.min(filter_hi)), f64::from(filter_lo.max(filter_hi)));
    let span = ((hi - lo) * (1.0 + 2.0 * SCOPE_MARGIN)).max(SCOPE_MIN_SPAN_HZ);
    let centre = hz + (lo + hi) / 2.0;
    (centre - span / 2.0, span)
}

/// The level of `bins` (spread over `span_hz` from `lo_hz`) at `f`, read with a
/// straight line between the neighbouring bins so a narrow window over coarse
/// bins is a slope and not a staircase. Nothing outside the frame.
fn level_at(bins: &[u8], lo_hz: f64, span_hz: f64, f: f64) -> u8 {
    let x = (f - lo_hz) / span_hz * bins.len() as f64 - 0.5;
    if x < -0.5 || x > bins.len() as f64 - 0.5 {
        return 0;
    }
    let i = x.floor().max(0.0) as usize;
    let j = (i + 1).min(bins.len() - 1);
    let t = (x - i as f64).clamp(0.0, 1.0);
    (f64::from(bins[i]) * (1.0 - t) + f64::from(bins[j]) * t).round() as u8
}

/// The panel's own state: what has to live between frames.
pub(in crate::app) struct State {
    pub group: Group,
    pub vu: VuBallistics,
    vu_at: Option<f64>,
    bars: Vec<f32>,
    peaks: Vec<f32>,
    pub(in crate::app) wf: VecDeque<Vec<u8>>,
    wf_seq: Option<u32>,
    /// Spectrum frames seen, to keep [`WF_KEEP`] in every [`WF_OF`].
    wf_tick: u32,
    /// The span the rows were made for: a row of another one does not line up
    /// with them.
    wf_span: f64,
    /// The waterfall as a picture, and what it was drawn for: its height in
    /// pixels and the three colours it is made from.
    wf_tex: Option<egui::TextureHandle>,
    wf_drawn: Option<(usize, [Color32; 3])>,
    wf_dirty: bool,
    /// The squelch threshold to put back when SQL is clicked open again.
    sql_last: f32,
    /// Whether the panel was on the last frame (`None` before the first one,
    /// so an app that starts with it on is not "switching it on").
    on_seen: Option<bool>,
    /// When it last took the receiver, on the UI clock.
    entered_at: f64,
    /// When the last frequency was last written to disk.
    saved_at: f64,
    /// When `WantAudioLevel` was last sent (wall clock), so it is renewed about
    /// once a second while the panel is on screen.
    lease_at: f64,
    /// The spectrum frame, handed over by the docked layout: it holds the frame
    /// out of `SdroxideApp::frame` while it draws, so the panel under the
    /// waterfall would otherwise find nothing there.
    pub frame: Option<std::sync::Arc<sdroxide_types::SpectrumFrame>>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            group: Group::default(),
            vu: VuBallistics::default(),
            vu_at: None,
            bars: vec![0.0; AUDIO_BANDS],
            peaks: vec![0.0; AUDIO_BANDS],
            wf: VecDeque::new(),
            wf_seq: None,
            wf_tick: 0,
            wf_span: 0.0,
            wf_tex: None,
            wf_drawn: None,
            wf_dirty: false,
            sql_last: -70.0,
            on_seen: None,
            entered_at: f64::NEG_INFINITY,
            saved_at: f64::NEG_INFINITY,
            lease_at: 0.0,
            frame: None,
        }
    }
}

impl State {
    /// Take a new spectrum row: [`WF_KEEP`] of every [`WF_OF`] are kept, newest
    /// first, up to [`WF_ROWS`]. Returns whether the picture changed.
    fn wf_push(&mut self, row: Vec<u8>) -> bool {
        let keep = self.wf_tick % WF_OF < WF_KEEP;
        self.wf_tick = self.wf_tick.wrapping_add(1);
        if keep {
            self.wf.push_front(row);
            self.wf.truncate(WF_ROWS);
        }
        keep
    }

    /// The scope now shows `span_hz`: rows made for another span are cleared.
    fn wf_span_is(&mut self, span_hz: f64) {
        if (self.wf_span - span_hz).abs() > span_hz * 0.01 {
            self.wf_span = span_hz;
            self.wf.clear();
            self.wf_dirty = true;
        }
    }
}

/// The colour of a waterfall cell for a level `0.0..=1.0`.
fn wf_colour(t: f32, ink: &[Color32; 3]) -> Color32 {
    let [ground, accent, strong] = *ink;
    if t < 0.8 { mix(ground, accent, t / 0.8) } else { mix(accent, strong, (t - 0.8) * 5.0) }
}

/// The waterfall as an image `out_h` pixels tall: the [`WF_ROWS`] rows are
/// averaged down to the pixels they land on (a row per pixel would shimmer),
/// newest at the top, and the part not filled yet is the ground.
fn wf_image(rows: &VecDeque<Vec<u8>>, out_h: usize, ink: &[Color32; 3]) -> egui::ColorImage {
    let out_h = out_h.clamp(1, WF_ROWS);
    let mut pixels = Vec::with_capacity(WF_BINS * out_h);
    for y in 0..out_h {
        let from = y * WF_ROWS / out_h;
        let to = ((y + 1) * WF_ROWS / out_h).max(from + 1);
        let mut sum = [0u32; WF_BINS];
        let mut n = 0u32;
        for row in (from..to).filter_map(|r| rows.get(r)) {
            for (acc, v) in sum.iter_mut().zip(row) {
                *acc += u32::from(*v);
            }
            n += 1;
        }
        for acc in sum {
            pixels.push(if n == 0 {
                ink[0]
            } else {
                wf_colour(acc as f32 / (n as f32 * 255.0), ink)
            });
        }
    }
    egui::ColorImage {
        size: [WF_BINS, out_h],
        source_size: vec2(WF_BINS as f32, out_h as f32),
        pixels,
    }
}

/// The ink the panel uses, read from the theme once a frame.
struct Ink {
    ground: Color32,
    line: Color32,
    text: Color32,
    strong: Color32,
    dim: Color32,
    accent: Color32,
    amber: Color32,
    frame: Color32,
    green: Color32,
    red: Color32,
}

impl Ink {
    fn now() -> Self {
        Self {
            ground: theme::BG_DEEP(),
            line: theme::LINE_LIT(),
            text: theme::TEXT(),
            strong: theme::TEXT_STRONG(),
            dim: theme::CYAN_DIM(),
            accent: theme::CYAN(),
            amber: theme::YELLOW(),
            frame: theme::PINK(),
            green: theme::GREEN(),
            red: theme::ALERT(),
        }
    }
}

fn mix(a: Color32, b: Color32, t: f32) -> Color32 {
    let t = t.clamp(0.0, 1.0);
    let f = |x: u8, y: u8| (f32::from(x) + (f32::from(y) - f32::from(x)) * t).round() as u8;
    Color32::from_rgb(f(a.r(), b.r()), f(a.g(), b.g()), f(a.b(), b.b()))
}

impl SdroxideApp {
    /// The docked panel: the frame under the waterfall, then the panel in it.
    pub(in crate::app) fn radio_classic_docked(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        panel_h: f32,
    ) {
        crate::layout::tighten(ui);
        egui::Frame::new()
            .fill(theme::BG_DEEP())
            .inner_margin(egui::Margin { left: 0, right: 0, top: 6, bottom: 0 })
            .show(ui, |ui| {
                self.radio_classic_panel(ui, cmds, (panel_h - 6.0).max(0.0));
            });
    }

    /// The panel itself, `h` points tall and as wide as `ui` is.
    pub(in crate::app) fn radio_classic_panel(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        h: f32,
    ) {
        let ink = Ink::now();
        let w = ui.available_width();
        let (outer, _) = ui.allocate_exact_size(vec2(w, h.max(120.0)), egui::Sense::hover());
        // One frame around all three columns.
        ui.painter().rect_filled(outer, 10.0, ink.ground);
        ui.painter().rect_stroke(
            outer.shrink(1.0),
            10.0,
            Stroke::new(1.5, ink.frame),
            StrokeKind::Inside,
        );
        let inner = outer.shrink2(vec2(12.0, 10.0));
        let (lw, mw, rw) = column_widths(inner.width());
        let left = Rect::from_min_size(inner.min, vec2(lw, inner.height()));
        let mid = Rect::from_min_size(
            Pos2::new(left.right() + 8.0, inner.top()),
            vec2(mw, inner.height()),
        );
        let right = Rect::from_min_size(
            Pos2::new(mid.right() + 8.0, inner.top()),
            vec2(rw, inner.height()),
        );

        let now = crate::time::now_unix();
        let wall = crate::time::now_unix_f64();
        if wall - self.radio_classic.lease_at >= 1.0 || wall < self.radio_classic.lease_at {
            self.radio_classic.lease_at = wall;
            cmds.push(Command::WantAudioLevel);
        }
        let hz = self.state.active_freq_hz();
        let mode = self.state.rx[0].mode;
        let lines = self.rc_name(hz, mode, now);

        self.rc_left(ui, left, cmds, &ink, hz, mode);
        self.rc_middle(ui, mid, cmds, &ink, hz, mode, lines.as_ref());
        self.rc_right(ui, right, cmds, &ink, hz, mode, lines.as_ref(), now);

        // Bars, needles and the marquee move between messages: ask for frames
        // only while there is something to move (and once a second for the
        // clocks, which the frame loop already does).
        let moving = self.radio_classic.vu.is_live()
            || self.radio_classic.bars.iter().any(|b| *b > 0.01)
            || self.audio_level.is_some();
        crate::repaint::after_ms(ui.ctx(), if moving { 33 } else { 250 });
        self.rc_detach_icon(ui, outer);
    }

    /// The arrow in the panel's top-right corner: docked it lifts the panel out
    /// into a window, detached it puts it back — the same two pictures and
    /// hints every other panel wears, so it need not be found in Settings.
    fn rc_detach_icon(&mut self, ui: &mut egui::Ui, outer: Rect) {
        use crate::chrome::{WindowIcon, chip_window_icon_sized};
        let phone = crate::layout::tier(ui.ctx()) == crate::layout::Tier::Phone;
        let mode = self.state.rx[0].mode;
        let placed = place(self.ui_settings.radio_classic_on, self.show_radio_classic, mode, phone);
        let Some(icon) = detach_icon(placed, self.show_radio_classic, phone) else { return };
        let hover = match icon {
            WindowIcon::Dock => tr!("frame-dock-panel-pane-hint"),
            _ => tr!("panels-float-chip-hint"),
        };
        let size = vec2(ICON_W, ICON_H);
        let at = Rect::from_min_size(Pos2::new(outer.right() - size.x - 4.0, outer.top() + 3.0), size);
        let clicked = ui
            .scope_builder(egui::UiBuilder::new().max_rect(at), |ui| {
                chip_window_icon_sized(ui, icon, size).on_hover_text(hover).clicked()
            })
            .inner;
        if clicked {
            // The frame's `sync!` carries the flag into the saved windows bit.
            self.show_radio_classic = !self.show_radio_classic;
        }
    }

    // ───────────────────────────────────────────────── the receiver is its own

    /// Radio Classic's hold on the receiver, once a frame.
    ///
    /// Switched on, it takes everything: [`Self::radio_classic_enter`]. While it
    /// is on, the frequency and mode it ends up on are what it comes back to,
    /// and another mode (or a second receiver, or a scan) taking the receiver
    /// hands it back — RADIO goes off.
    pub(in crate::app) fn radio_classic_tick(&mut self, now: f64, cmds: &mut Vec<Command>) {
        let on = self.radio_classic_active();
        match self.radio_classic.on_seen.replace(on) {
            Some(false) if on => self.radio_classic_enter(now, cmds),
            Some(true) if !on => crate::app::persist::persist_ui_settings(&self.ui_settings),
            _ => {}
        }
        if !on || now - self.radio_classic.entered_at < ENTER_GRACE_S {
            return;
        }
        let mode = self.state.rx[0].mode;
        if !is_radio_mode(mode) || self.state.sub_rx_enabled || self.state.scan.running {
            self.ui_settings.radio_classic_on = false;
            self.show_radio_classic = false;
            self.ui_settings.tool_windows_open &= !crate::app::frame::TW_RADIO_CLASSIC;
            crate::app::persist::persist_ui_settings(&self.ui_settings);
            return;
        }
        let hz = self.state.active_freq_hz();
        let ui = &mut self.ui_settings;
        if ui.radio_classic_last_hz != hz || ui.radio_classic_last_mode != Some(mode) {
            ui.radio_classic_last_hz = hz;
            ui.radio_classic_last_mode = Some(mode);
            if now - self.radio_classic.saved_at >= SAVE_EVERY_S {
                self.radio_classic.saved_at = now;
                crate::app::persist::persist_ui_settings(&self.ui_settings);
            }
        }
    }

    /// Radio Classic has just been switched on: every other mode is quit, its
    /// window shut, and the dial goes to Radio Classic's frequency at once — the
    /// last one it listened on, or 100 MHz in WFM the first time.
    fn radio_classic_enter(&mut self, now: f64, cmds: &mut Vec<Command>) {
        self.radio_classic.entered_at = now;
        let (hz, mode) = match self.ui_settings.radio_classic_last_hz {
            hz if hz > 0.0 => (
                hz,
                self.ui_settings
                    .radio_classic_last_mode
                    .filter(|m| is_radio_mode(*m))
                    .unwrap_or(Mode::Wfm),
            ),
            _ => (FIRST_HZ, Mode::Wfm),
        };
        // The other flows: the second receiver, the scan, the ISM decoder.
        if self.state.sub_rx_enabled {
            self.state.sub_rx_enabled = false;
            cmds.push(Command::SetSubRx(false));
        }
        if self.state.scan.running {
            self.state.scan.running = false;
            cmds.push(Command::SetScanning(false));
        }
        if self.state.ism.any_enabled() {
            self.state.ism.enabled = false;
            cmds.push(Command::SetIsmConfig(self.state.ism));
        }
        // The windows of the modes and decoders that were in use.
        for open in [
            &mut self.show_sstv,
            &mut self.show_dab,
            &mut self.show_aircraft,
            &mut self.show_ism,
            &mut self.show_adsb,
            &mut self.show_ais,
            &mut self.show_acars,
            &mut self.show_hfdl,
            &mut self.show_vdl2,
            &mut self.show_navtex,
            &mut self.show_dsc,
            &mut self.show_aprs,
            &mut self.show_wefax,
            &mut self.show_js8,
            &mut self.show_fsq,
            &mut self.show_hell,
            &mut self.show_packet,
            &mut self.show_wspr,
            &mut self.show_pi4,
            &mut self.show_text,
            &mut self.show_uvpacket,
            &mut self.show_jtty,
            &mut self.show_ale,
            &mut self.show_rf_paint,
            &mut self.show_image,
            &mut self.show_drm,
            &mut self.show_hd,
            &mut self.show_scanner,
        ] {
            *open = false;
        }
        // The broadcast stereo decode, which only WFM has and which the
        // operator may have switched off in some other mode.
        if mode == Mode::Wfm && !self.state.rx[0].wfm_stereo {
            self.state.rx[0].wfm_stereo = true;
            cmds.push(Command::SetWfmStereo { rx: RxId::Main, on: true });
        }
        // And the dial: the frequency first, as the band buttons do.
        let vfo = self.state.active_vfo;
        cmds.push(Command::SetVfo { vfo, hz });
        cmds.push(Command::SetMode { rx: RxId::Main, mode });
        // Echoed now, so nothing of the old mode is drawn while the station
        // answers.
        self.state.rx[0].mode = mode;
        match vfo {
            sdroxide_types::Vfo::A => self.state.vfo_a_hz = hz,
            sdroxide_types::Vfo::B => self.state.vfo_b_hz = hz,
        }
    }

    /// The lines the station is written in, from what is known about the dial.
    fn rc_name(&self, hz: f64, mode: Mode, now: i64) -> Option<StationLines> {
        let rounded = hz.round() as i64;
        let memory =
            self.memories.iter().find(|m| m.mode == mode && m.freq_hz.round() as i64 == rounded);
        let scheduled = broadcast::at_dial(&self.broadcast, hz, now);
        let ps = self.rds.as_ref().and_then(|r| r.ps.as_deref());
        station_lines(mode, ps, memory, scheduled)
    }

    // ───────────────────────────────────────────────────────────── left column

    fn rc_left(
        &mut self,
        ui: &mut egui::Ui,
        rect: Rect,
        cmds: &mut Vec<Command>,
        ink: &Ink,
        hz: f64,
        mode: Mode,
    ) {
        let band = self.state.band;
        ui.scope_builder(egui::UiBuilder::new().max_rect(rect), |ui| {
            ui.set_clip_rect(rect.intersect(ui.clip_rect()));
            // The sections share whatever height is left over, so the column
            // ends where the other two do instead of stopping short.
            let gap = ((rect.height() - 300.0) / 6.0).clamp(0.0, 16.0);
            ui.spacing_mut().item_spacing = vec2(6.0, 6.0);
            ui.label(RichText::new(tr!("rc-radio-classic")).strong().size(14.0).color(ink.strong));
            ui.add_space(2.0 + gap);

            crate::chrome::menu_caption(ui, &tr!("rc-bands"));
            ui.horizontal_wrapped(|ui| {
                for (g, label) in Group::ALL {
                    if crate::chrome::chip(ui, self.radio_classic.group == g, group_tr(label)).clicked() {
                        self.radio_classic.group = g;
                    }
                }
            });

            for (caption, row) in group_rows(self.radio_classic.group) {
                ui.add_space(gap);
                crate::chrome::menu_caption(ui, &caption_tr(caption));
                ui.horizontal_wrapped(|ui| {
                    for p in row {
                        let lit = match p.band {
                            Some(b) if b != Band::Gen => band == b && p.holds(hz),
                            _ => p.holds(hz),
                        };
                        if crate::chrome::chip(ui, lit, preset_tr(p.label.as_str())).clicked() && !lit {
                            match p.band {
                                Some(b) => cmds.push(Command::SetBand(b)),
                                None => {
                                    cmds.push(Command::SetVfo {
                                        vfo: self.state.active_vfo,
                                        hz: p.tune_hz,
                                    });
                                    cmds.push(Command::SetMode { rx: RxId::Main, mode: p.mode });
                                }
                            }
                        }
                    }
                });
            }

            ui.add_space(gap);
            crate::chrome::menu_caption(ui, &tr!("vdl2-mode"));
            ui.horizontal_wrapped(|ui| {
                for (label, m) in MODES {
                    if crate::chrome::chip(ui, mode == m, label).clicked() && mode != m {
                        cmds.push(Command::SetMode { rx: RxId::Main, mode: m });
                    }
                }
            });

            ui.add_space(gap);
            crate::chrome::menu_caption(ui, &tr!("rc-mem-programs"));
            self.rc_memories(ui, cmds, hz, mode);
        });
    }

    /// The memory buttons and the PROGRAMS chip.
    fn rc_memories(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>, hz: f64, mode: Mode) {
        let rounded = hz.round() as i64;
        ui.horizontal_wrapped(|ui| {
            for m in self.memories.iter().take(4) {
                let lit = m.mode == mode && m.freq_hz.round() as i64 == rounded;
                let label = format!("{} · {:.3}", m.name, m.freq_hz / 1e6);
                if crate::chrome::chip(ui, lit, label)
                    .on_hover_text(tr!("rc-recall-band-mode-frequency-and"))
                    .clicked()
                {
                    cmds.push(Command::RecallMemory(m.id));
                }
            }
            if self.memories.is_empty() {
                ui.label(
                    RichText::new(tr!("rc-no-memories-yet-store-one-with-mem"))
                        .size(11.0)
                        .color(theme::gray(130)),
                );
            }
            // The same window the top bar opens, with its stations to pick from.
            if crate::chrome::chip(ui, self.schedule.show, tr!("schedule-title"))
                .on_hover_text(tr!("rc-broadcast-schedule-what-is-on-when"))
                .clicked()
            {
                self.schedule.show = !self.schedule.show;
            }
        });
    }

    // ─────────────────────────────────────────────────────────── middle column

    #[allow(clippy::too_many_arguments)]
    fn rc_middle(
        &mut self,
        ui: &mut egui::Ui,
        rect: Rect,
        cmds: &mut Vec<Command>,
        ink: &Ink,
        hz: f64,
        mode: Mode,
        name: Option<&StationLines>,
    ) {
        let vu_w = vu_meter::WIDTH_COMPACT.max(64.0);
        let main =
            Rect::from_min_max(rect.min, Pos2::new(rect.right() - vu_w - 8.0, rect.bottom()));
        let vu_rect = Rect::from_min_max(Pos2::new(rect.right() - vu_w, rect.top()), rect.max);

        // The header: the name small at the top left, the band's range at the
        // right, the mode centred and big.
        let head_h = (rect.height() * 0.17).clamp(54.0, 90.0);
        let head = Rect::from_min_size(main.min, vec2(main.width(), head_h));
        let p = ui.painter();
        p.rect_filled(head, 6.0, ink.ground);
        p.rect_stroke(head, 6.0, Stroke::new(1.0, ink.line), StrokeKind::Inside);
        let range = self.rc_range(hz);
        if let Some(n) = name {
            // The name proper, without its bracket, on one line.
            let room = head.width() - 20.0 - if range.is_some() { 96.0 } else { 0.0 };
            let mut job = egui::text::LayoutJob::single_section(
                n.big.clone(),
                egui::TextFormat::simple(FontId::proportional(14.0), ink.amber),
            );
            job.wrap = egui::text::TextWrapping {
                max_width: room.max(40.0),
                max_rows: 1,
                break_anywhere: true,
                overflow_character: Some('…'),
            };
            let galley = p.layout_job(job);
            p.galley(head.left_top() + vec2(10.0, 8.0), galley, ink.amber);
        }
        if let Some((lo, hi)) = range {
            p.text(
                head.right_top() + vec2(-10.0, 8.0),
                Align2::RIGHT_TOP,
                format!("{lo}–{hi} MHz"),
                FontId::proportional(10.0),
                ink.dim,
            );
        }
        p.text(
            head.center() + vec2(0.0, 8.0),
            Align2::CENTER_CENTER,
            mode_label(mode),
            FontId::proportional((head_h * 0.52).clamp(24.0, 46.0)),
            ink.accent,
        );
        p.text(
            head.right_bottom() + vec2(-10.0, -6.0),
            Align2::RIGHT_BOTTOM,
            tr!("settings-tle-mode"),
            FontId::proportional(10.0),
            ink.dim,
        );

        // The boxes at the foot: BW, STEP, AGC, SQL.
        let boxes_h = 34.0;
        let meter_h = (rect.height() * 0.2).clamp(52.0, 84.0);
        let boxes = Rect::from_min_size(
            Pos2::new(main.left(), main.bottom() - boxes_h),
            vec2(main.width(), boxes_h),
        );
        let meter = Rect::from_min_size(
            Pos2::new(main.left(), boxes.top() - 6.0 - meter_h),
            vec2(main.width(), meter_h),
        );
        let scope = Rect::from_min_max(
            Pos2::new(main.left(), head.bottom() + 6.0),
            Pos2::new(main.right(), meter.top() - 6.0),
        );
        if scope.height() > 30.0 {
            self.rc_scope(ui, scope, ink, hz);
        }
        self.rc_smeter(ui, meter, ink);

        self.rc_boxes(ui, boxes, cmds, ink, mode);

        // The VU: the volume control too, as in the DAB panel.
        self.rc_vu_step(ui);
        ui.scope_builder(egui::UiBuilder::new().max_rect(vu_rect), |ui| {
            let out = vu_meter::stereo(
                ui,
                &self.radio_classic.vu,
                vu_rect.size(),
                true,
                Some(self.state.rx[0].volume),
            );
            self.dab_vu_volume(ui, cmds, out);
        });
    }

    /// The boxes at the foot, BW · STEP · AGC · SQL — each one acts.
    ///
    /// Click takes the next setting, right-click the previous one, and the wheel
    /// over a box moves it a notch; every route is the one the top bar uses
    /// (`set_rx_filter`, `SetAgc`, `SetSquelch`, the tuning step).
    fn rc_boxes(
        &mut self,
        ui: &mut egui::Ui,
        boxes: Rect,
        cmds: &mut Vec<Command>,
        ink: &Ink,
        mode: Mode,
    ) {
        let rx = self.state.rx[0];
        let bw = (rx.filter_hi - rx.filter_lo).abs();
        let open = rx.squelch_db <= SQUELCH_OPEN_DB + 0.5;
        if !open {
            self.radio_classic.sql_last = rx.squelch_db;
        }
        let presets = mode.filter_presets();
        let items: [(String, String, String); 4] = [
            (
                "BW".to_string(),
                if bw >= 1000.0 {
                    format!("{:.1} kHz", bw / 1000.0)
                } else {
                    format!("{bw:.0} Hz")
                },
                if presets.is_empty() {
                    tr!("rc-bw-this-mode-has-one-fixed")
                } else {
                    tr!("rc-bw-click-next-filter-right-click")
                },
            ),
            (
                tr!("rc-step"),
                format!("{:.0}", self.ui_settings.tune_step_hz),
                tr!("rc-step-click-next-tuning-step-right"),
            ),
            (
                "AGC".to_string(),
                rx.agc.tr().to_uppercase(),
                tr!("rc-agc-click-next-speed-right-click"),
            ),
            (
                "SQL".to_string(),
                if open { tr!("rc-open") } else { format!("{:.0} dB", rx.squelch_db) },
                tr!("rc-sql-click-open-back-to-the-last")
                    .to_string(),
            ),
        ];
        let cell_w = (boxes.width() - 3.0 * 6.0) / 4.0;
        // (clicks forward, clicks back, wheel notches) of each box this frame.
        let mut acts = [(false, false, 0i32); 4];
        for (i, (cap, val, hint)) in items.iter().enumerate() {
            let r = Rect::from_min_size(
                boxes.min + vec2(i as f32 * (cell_w + 6.0), 0.0),
                vec2(cell_w, boxes.height()),
            );
            let resp = ui
                .interact(r, ui.id().with(("radio-classic-box", cap.as_str())), egui::Sense::click())
                .on_hover_text(hint.as_str());
            // One notch per wheel event, whatever the smoothing makes of it.
            let notches = if resp.hovered() {
                ui.input(|i| {
                    i.events
                        .iter()
                        .map(|e| match e {
                            egui::Event::MouseWheel { delta, .. } if delta.y != 0.0 => {
                                delta.y.signum() as i32
                            }
                            _ => 0,
                        })
                        .sum()
                })
            } else {
                0
            };
            acts[i] = (resp.clicked(), resp.secondary_clicked(), notches);
            let p = ui.painter();
            let edge = if resp.hovered() { ink.accent } else { ink.line };
            p.rect_stroke(r, 3.0, Stroke::new(1.0, edge), StrokeKind::Inside);
            p.text(
                r.left_top() + vec2(5.0, 3.0),
                Align2::LEFT_TOP,
                cap.as_str(),
                FontId::proportional(8.5),
                ink.dim,
            );
            p.text(
                r.left_bottom() + vec2(5.0, -4.0),
                Align2::LEFT_BOTTOM,
                val,
                FontId::proportional(12.0),
                ink.strong,
            );
        }

        // BW: the mode's own filter presets.
        let (fwd, back, wheel) = acts[0];
        if let Some(dir) = direction(fwd, back, wheel)
            && let Some(i) = next_preset(presets, mode, rx.filter_lo, rx.filter_hi, dir > 0)
        {
            let (_, lo, hi) = presets[i];
            let (lo, hi) = crate::app::top_bar::preset_edges(mode, lo, hi, self.cw_pitch_hz());
            self.set_rx_filter(lo, hi, cmds);
        }
        // STEP.
        let (fwd, back, wheel) = acts[1];
        if let Some(dir) = direction(fwd, back, wheel) {
            self.ui_settings.tune_step_hz = next_step(self.ui_settings.tune_step_hz, dir > 0);
            crate::app::persist::persist_ui_settings(&self.ui_settings);
        }
        // AGC.
        let (fwd, back, wheel) = acts[2];
        if let Some(dir) = direction(fwd, back, wheel) {
            let agc = next_agc(rx.agc, dir > 0);
            self.state.rx[0].agc = agc; // optimistic echo
            cmds.push(Command::SetAgc { rx: RxId::Main, agc });
        }
        // SQL: a click holds the gate open or puts the last threshold back; the
        // wheel moves it.
        let (fwd, back, wheel) = acts[3];
        let db = if fwd || back {
            Some(if open { self.radio_classic.sql_last } else { SQUELCH_OPEN_DB })
        } else if wheel != 0 {
            Some(sql_wheel(rx.squelch_db, wheel))
        } else {
            None
        };
        if let Some(db) = db {
            self.sql_hold = None;
            self.state.rx[0].squelch_db = db; // optimistic echo
            cmds.push(Command::SetSquelch { rx: RxId::Main, db });
        }
    }

    /// The range of the band the dial sits in, in MHz, for the header.
    fn rc_range(&self, hz: f64) -> Option<(String, String)> {
        let preset = group_rows(Group::Broadcast)
            .into_iter()
            .flat_map(|(_, r)| r)
            .filter(|p| p.holds(hz))
            .min_by(|a, b| (a.hi_hz - a.lo_hz).total_cmp(&(b.hi_hz - b.lo_hz)))?;
        let fmt = |v: f64| {
            let s = format!("{:.3}", v / 1e6);
            s.trim_end_matches('0').trim_end_matches('.').to_string()
        };
        Some((fmt(preset.lo_hz), fmt(preset.hi_hz)))
    }

    /// The mini spectrum and its waterfall, around the dial.
    fn rc_scope(&mut self, ui: &mut egui::Ui, rect: Rect, ink: &Ink, hz: f64) {
        let rx = &self.state.rx[0];
        let (win_lo, win_span) = scope_window(hz, rx.filter_lo, rx.filter_hi);
        self.radio_classic.wf_span_is(win_span);
        let held = self.frame.clone().or_else(|| self.radio_classic.frame.clone());
        if let Some(frame) = held.as_ref()
            && self.radio_classic.wf_seq != Some(frame.seq)
            && !frame.bins.is_empty()
            && frame.span_hz > 0.0
        {
            self.radio_classic.wf_seq = Some(frame.seq);
            let lo = frame.center_hz - frame.span_hz / 2.0;
            let row: Vec<u8> = (0..WF_BINS)
                .map(|i| {
                    let f = win_lo + win_span * (i as f64 + 0.5) / WF_BINS as f64;
                    level_at(&frame.bins, lo, frame.span_hz, f)
                })
                .collect();
            if self.radio_classic.wf_push(row) {
                self.radio_classic.wf_dirty = true;
            }
        }
        let p = ui.painter_at(rect);
        p.rect_filled(rect, 4.0, ink.ground);
        let line_h = (rect.height() * 0.28).max(18.0);
        let line_rect = Rect::from_min_size(rect.min, vec2(rect.width(), line_h));
        let wf_rect = Rect::from_min_max(Pos2::new(rect.left(), line_rect.bottom()), rect.max);
        if let Some(newest) = self.radio_classic.wf.front() {
            let pts: Vec<Pos2> = newest
                .iter()
                .enumerate()
                .map(|(i, v)| {
                    Pos2::new(
                        line_rect.left() + line_rect.width() * (i as f32 + 0.5) / WF_BINS as f32,
                        line_rect.bottom()
                            - 2.0
                            - (line_rect.height() - 6.0) * f32::from(*v) / 255.0,
                    )
                })
                .collect();
            p.add(egui::Shape::line(pts, Stroke::new(1.2, ink.accent)));
        }
        // The cascade is one picture, rebuilt only when a row came in or the
        // size or the colours changed — not 96 × 400 rectangles every frame.
        let ppp = ui.ctx().pixels_per_point();
        let out_h = ((wf_rect.height() * ppp).round() as usize).clamp(1, WF_ROWS);
        let colours = [ink.ground, ink.accent, ink.strong];
        let st = &mut self.radio_classic;
        if st.wf_tex.is_none() || st.wf_drawn != Some((out_h, colours)) || st.wf_dirty {
            let image = wf_image(&st.wf, out_h, &colours);
            match st.wf_tex.as_mut() {
                Some(tex) => tex.set(image, egui::TextureOptions::LINEAR),
                None => {
                    st.wf_tex = Some(ui.ctx().load_texture(
                        "radio-classic-waterfall",
                        image,
                        egui::TextureOptions::LINEAR,
                    ));
                }
            }
            st.wf_drawn = Some((out_h, colours));
            st.wf_dirty = false;
        }
        if let Some(tex) = &st.wf_tex {
            p.image(
                tex.id(),
                wf_rect,
                Rect::from_min_max(Pos2::ZERO, Pos2::new(1.0, 1.0)),
                Color32::WHITE,
            );
        }
        // The dial's own mark, where the dial falls in the window: the centre
        // on a symmetric filter, off to one side on SSB.
        let x = (rect.left() + rect.width() * ((hz - win_lo) / win_span) as f32)
            .clamp(rect.left(), rect.right());
        p.line_segment(
            [Pos2::new(x, rect.top()), Pos2::new(x, rect.bottom())],
            Stroke::new(1.0, ink.amber.gamma_multiply(0.6)),
        );
        p.rect_stroke(rect, 4.0, Stroke::new(1.0, ink.line), StrokeKind::Inside);
    }

    /// The segmented S-meter, red above S9.
    fn rc_smeter(&self, ui: &mut egui::Ui, rect: Rect, ink: &Ink) {
        let s_dbm = self.meters.map(|m| m.s_dbm);
        let p = ui.painter_at(rect);
        p.rect_stroke(rect, 4.0, Stroke::new(1.0, ink.line), StrokeKind::Inside);
        let reading = s_dbm.map_or_else(|| "—".to_string(), s_reading);
        p.text(
            rect.left_top() + vec2(8.0, 5.0),
            Align2::LEFT_TOP,
            reading,
            FontId::proportional(13.0),
            ink.strong,
        );
        p.text(
            rect.right_top() + vec2(-8.0, 6.0),
            Align2::RIGHT_TOP,
            tr!("rc-s-meter"),
            FontId::proportional(10.0),
            ink.dim,
        );
        let bar = Rect::from_min_max(
            Pos2::new(rect.left() + 8.0, rect.top() + 24.0),
            Pos2::new(
                rect.right() - 8.0,
                rect.top() + 24.0 + (rect.height() * 0.28).clamp(10.0, 18.0),
            ),
        );
        const SEGS: usize = 38;
        let fill = s_dbm.map_or(0.0, smeter_fill);
        let s9 = smeter_fill(-73.0);
        let seg_w = bar.width() / SEGS as f32;
        for i in 0..SEGS {
            let t = (i as f32 + 0.5) / SEGS as f32;
            let lit = t <= fill;
            let hot = t > s9;
            let c = match (lit, hot) {
                (true, false) => ink.green,
                (true, true) => ink.red,
                (false, false) => mix(ink.ground, ink.green, 0.12),
                (false, true) => mix(ink.ground, ink.red, 0.12),
            };
            let r = Rect::from_min_size(
                Pos2::new(bar.left() + i as f32 * seg_w, bar.top()),
                vec2((seg_w - 1.5).max(1.0), bar.height()),
            );
            p.rect_filled(r, 1.0, c);
        }
        for (label, dbm, hot) in [
            ("1", -121.0, false),
            ("3", -109.0, false),
            ("5", -97.0, false),
            ("7", -85.0, false),
            ("9", -73.0, false),
            ("+20", -53.0, true),
            ("+40", -33.0, true),
            ("+60", -13.0, true),
        ] {
            let x = bar.left() + bar.width() * smeter_fill(dbm);
            p.text(
                Pos2::new(x.min(bar.right() - 6.0), bar.bottom() + 2.0),
                Align2::CENTER_TOP,
                label,
                FontId::proportional(9.0),
                if hot { ink.red } else { ink.dim },
            );
        }
        if let Some(d) = s_dbm {
            p.text(
                rect.left_bottom() + vec2(8.0, -4.0),
                Align2::LEFT_BOTTOM,
                format!("RSSI {:.0} dBµV", d + 107.0),
                FontId::proportional(10.0),
                ink.text,
            );
            p.text(
                rect.right_bottom() + vec2(-8.0, -4.0),
                Align2::RIGHT_BOTTOM,
                format!("{d:.0} dBm"),
                FontId::proportional(10.0),
                ink.text,
            );
        }
    }

    /// Move the VU and the bars on by this frame's time, from the last level
    /// the station sent (and to silence when it has stopped sending).
    fn rc_vu_step(&mut self, ui: &egui::Ui) {
        let now = ui.input(|i| i.time);
        let dt = self.radio_classic.vu_at.map_or(0.0, |t| (now - t) as f32);
        self.radio_classic.vu_at = Some(now);
        let wall = crate::time::now_unix_f64();
        let fresh = self
            .audio_level
            .as_ref()
            .filter(|(at, _)| wall - *at < f64::from(LEVEL_FRESH_S))
            .map(|(_, l)| l);
        // A real stereo broadcast is shown as it is. Everything else is one
        // signal in two ears, and its meter is given a false stereo — the
        // picture only, the sound is not touched.
        let (hz, mode) = (self.state.active_freq_hz(), self.state.rx[0].mode);
        let pilot = self.meters.is_some_and(|m| m.stereo);
        let real = stereo_chip(hz, mode, stereo_playing(&self.state.rx[0], pilot)) == Some(true)
            || self.state.sub_rx_enabled;
        let rms = fresh.map(|l| if real { l.rms_db } else { false_stereo(l.rms_db, &l.bands) });
        self.radio_classic.vu.step(rms, dt);
        let st = &mut self.radio_classic;
        for i in 0..AUDIO_BANDS {
            let target = fresh.and_then(|l| l.bands.get(i)).map_or(0.0, |b| byte_to_unit(*b));
            // Quick up, slower down: a bar, not a trace.
            let k = if target > st.bars[i] { 0.6 } else { (dt * 6.0).clamp(0.05, 0.5) };
            st.bars[i] += (target - st.bars[i]) * k;
            if st.bars[i] >= st.peaks[i] {
                st.peaks[i] = st.bars[i];
            } else {
                st.peaks[i] = (st.peaks[i] - dt * 0.35).max(st.bars[i]);
            }
        }
    }

    // ──────────────────────────────────────────────────────────── right column

    #[allow(clippy::too_many_arguments)]
    fn rc_right(
        &mut self,
        ui: &mut egui::Ui,
        rect: Rect,
        cmds: &mut Vec<Command>,
        ink: &Ink,
        hz: f64,
        mode: Mode,
        name: Option<&StationLines>,
        now: i64,
    ) {
        {
            let p = ui.painter();
            p.rect_filled(rect, 8.0, ink.ground);
            p.rect_stroke(rect, 8.0, Stroke::new(1.0, ink.line), StrokeKind::Inside);
        }
        let inner = rect.shrink2(vec2(14.0, 10.0));
        let h = inner.height();
        let rds = self.rds.as_ref().filter(|r| mode == Mode::Wfm && !r.is_empty());

        // Row 1: the mode tag, STEREO and MEM chips on the left; the clocks on
        // the right.
        let head_h = 34.0;
        let p = ui.painter();
        let tag = if mode == Mode::Wfm { "FM".to_string() } else { mode_tag(mode) };
        let tag_rect =
            p.text(inner.left_top(), Align2::LEFT_TOP, tag, FontId::proportional(24.0), ink.amber);
        let mut x = tag_rect.right() + 12.0;
        let pilot = self.meters.is_some_and(|m| m.stereo);
        if let Some(lit) = stereo_chip(hz, mode, stereo_playing(&self.state.rx[0], pilot)) {
            let at = Pos2::new(x, inner.top() + 3.0);
            let r = stereo_pill(p, at, lit, ink);
            // Lit means stereo; a click is the top bar's ST switch, so the
            // panel never needs it.
            let resp = ui
                .interact(r, ui.id().with("radio-classic-stereo"), egui::Sense::click())
                .on_hover_text(stereo_note(&self.state.rx[0], lit))
                .on_hover_cursor(egui::CursorIcon::PointingHand);
            if resp.clicked() {
                let on = !self.state.rx[0].wfm_stereo;
                self.state.rx[0].wfm_stereo = on; // optimistic echo
                cmds.push(Command::SetWfmStereo { rx: RxId::Main, on });
            }
            x = r.right() + 8.0;
        }
        let memory = self
            .memories
            .iter()
            .find(|m| m.mode == mode && m.freq_hz.round() as i64 == hz.round() as i64);
        if let Some(m) = memory {
            let label = tr!("rc-mem-n", id = m.id);
            x = pill(p, Pos2::new(x, inner.top() + 3.0), &label, ink.strong, ink.red) + 8.0;
        }
        p.text(
            Pos2::new(x, inner.top() + 6.0),
            Align2::LEFT_TOP,
            format!(
                "{} {:.1}",
                if (64e6..=108e6).contains(&hz) { "FM" } else { self.state.band.label() },
                hz / 1e6
            ),
            FontId::proportional(15.0),
            ink.strong,
        );
        let off = crate::time::local_offset_at(now);
        for (i, (cap, clock)) in
            [("LOC", crate::time::utc_clock(now + off)), ("UTC", crate::time::utc_clock(now))]
                .into_iter()
                .enumerate()
        {
            let y = inner.top() + i as f32 * 17.0;
            p.text(
                Pos2::new(inner.right() - 92.0, y + 3.0),
                Align2::RIGHT_TOP,
                cap,
                FontId::proportional(9.0),
                ink.dim,
            );
            p.text(
                Pos2::new(inner.right(), y),
                Align2::RIGHT_TOP,
                clock,
                FontId::monospace(15.0),
                if i == 1 { ink.amber } else { ink.strong },
            );
        }

        // Row 2: the VFO, in the same widget as the top bar's.
        let digit = (h * 0.15).clamp(26.0, 62.0);
        let vfo_rect = Rect::from_min_size(
            Pos2::new(inner.left(), inner.top() + head_h + 6.0),
            vec2(inner.width(), digit * 1.25),
        );
        let wheel = self.input.cfg.wheel;
        let active = self.state.active_vfo;
        let mut new_hz = None;
        // The readout is as wide as its digits and sits at the left of whatever
        // room it is given, so it is placed by the width it had last frame.
        let width_id = crate::layout::salted_id(ui.ctx(), "radio-classic-freq-width");
        let known = ui.data(|d| d.get_temp::<f32>(width_id));
        let dial_rect = match known {
            Some(w) => centred_in(vfo_rect, w),
            None => vfo_rect,
        };
        let used = ui
            .scope_builder(egui::UiBuilder::new().max_rect(dial_rect), |ui| {
                new_hz = crate::widgets::freq_display::show(
                    ui,
                    crate::layout::salted_id(ui.ctx(), "radio-classic-freq"),
                    hz,
                    wheel,
                    digit,
                    Some(ink.amber),
                    9,
                );
            })
            .response
            .rect
            .width();
        if known.is_none_or(|w| (w - used).abs() > 0.5) {
            ui.data_mut(|d| d.insert_temp(width_id, used));
            ui.ctx().request_repaint();
        }
        if let Some(hz) = new_hz {
            cmds.push(Command::SetVfo { vfo: active, hz });
        }
        let hint_y = vfo_rect.bottom() + 1.0;
        ui.painter().text(
            Pos2::new(inner.center().x, hint_y),
            Align2::CENTER_TOP,
            tr!("rc-wheel-on-a-digit-click-upper-lower"),
            FontId::proportional(9.5),
            ink.dim,
        );

        // Row 3: the station name at its size, in the small type of the
        // RadioText line around it: what the table says about a scheduled
        // transmission over it, and under it the bracket of a long name and a
        // memory's frequency and mode, all in the same colour. A name that is
        // still too long for the column wraps onto a second line.
        let name_y = hint_y + 18.0;
        let name_size = (h * 0.1).clamp(20.0, 40.0);
        let small = |text: &str| {
            let mut job = egui::text::LayoutJob::single_section(
                text.to_string(),
                egui::TextFormat::simple(FontId::proportional(15.0), ink.amber),
            );
            job.wrap.max_width = inner.width();
            job.wrap.max_rows = 1;
            job.halign = Align::Center;
            ui.painter().layout_job(job)
        };
        let mut y = name_y;
        if let Some(above) = name.and_then(|n| n.above.as_deref()) {
            let galley = small(above);
            let gh = galley.size().y;
            ui.painter().galley(Pos2::new(inner.center().x, y), galley, ink.amber);
            y += gh + 2.0;
        }
        let big = name.map_or("—", |n| n.big.as_str());
        let galley = ui.painter().layout_job(name_job(big, name_size, inner.width(), ink.amber));
        let big_h = galley.size().y;
        ui.painter().galley(Pos2::new(inner.center().x, y), galley, ink.amber);
        y += big_h;
        for line in name.map(|n| n.below.as_slice()).unwrap_or_default() {
            let galley = small(line);
            let gh = galley.size().y;
            y += 2.0;
            ui.painter().galley(Pos2::new(inner.center().x, y), galley, ink.amber);
            y += gh;
        }
        let name_h = y - name_y;

        // Row 4: RadioText.
        let rt_rect = Rect::from_min_size(
            Pos2::new(inner.left(), name_y + name_h + 8.0),
            vec2(inner.width(), 30.0),
        );
        let rt_text =
            rds.and_then(|r| r.radiotext.as_deref()).map(str::trim).filter(|s| !s.is_empty());
        if let Some(rt) = rt_text {
            marquee(ui, rt_rect, rt, ink);
        }

        // Row 5: the RDS chips — only what the station sends.
        let chips_y = rt_rect.bottom() + 8.0;
        if let Some(r) = rds {
            self.rds_chips(ui, Pos2::new(inner.left(), chips_y), inner.right(), r, ink);
        }

        // Row 6: the audio bars, down to the foot.
        let bars_top = chips_y + 30.0;
        let bars = Rect::from_min_max(Pos2::new(inner.left(), bars_top), inner.right_bottom());
        if bars.height() > 20.0 {
            self.rc_bars(ui, bars, ink);
        }
    }

    fn rds_chips(&self, ui: &mut egui::Ui, at: Pos2, right: f32, r: &RdsData, ink: &Ink) {
        let standard = self.view.rds_standard.resolve(r.pi, r.ecc);
        let mut chips: Vec<(&str, String, Color32)> = Vec::new();
        if let Some(pty) = r.pty {
            chips.push(("PTY", pty_name(pty, standard).to_string(), ink.line));
        }
        if let Some(pi) = r.pi {
            chips.push(("PI", format!("{pi:04X}"), ink.line));
        }
        if r.tp {
            chips.push(("", "TP".into(), ink.green));
        }
        if r.ta {
            chips.push(("", "TA".into(), ink.red));
        }
        match r.music {
            Some(true) => chips.push(("", tr!("rc-music").into(), ink.line)),
            Some(false) => chips.push(("", tr!("rc-speech").into(), ink.line)),
            None => {}
        }
        if !r.af.is_empty() {
            let list: Vec<String> =
                r.af.iter().take(3).map(|hz| format!("{:.1}", f64::from(*hz) / 1e6)).collect();
            let more = if r.af.len() > 3 { format!(" +{}", r.af.len() - 3) } else { String::new() };
            chips.push(("AF", format!("{}{more} MHz", list.join(" · ")), ink.line));
        }
        if let Some(c) = &r.clock {
            chips.push(("CT", c.label(), ink.line));
        }
        let mut x = at.x;
        let p = ui.painter();
        for (cap, val, edge) in chips {
            let text = if cap.is_empty() { val } else { format!("{cap}  {val}") };
            let end = pill(p, Pos2::new(x, at.y), &text, ink.strong, edge);
            if end > right {
                break;
            }
            x = end + 6.0;
        }
    }

    /// The audio bars with their peak caps.
    fn rc_bars(&self, ui: &mut egui::Ui, rect: Rect, ink: &Ink) {
        let st = &self.radio_classic;
        let n = st.bars.len();
        let slot = rect.width() / n as f32;
        let p = ui.painter_at(rect);
        for i in 0..n {
            let x = rect.left() + i as f32 * slot;
            let hgt = rect.height() * st.bars[i].clamp(0.0, 1.0);
            let bar = Rect::from_min_max(
                Pos2::new(x + 1.0, rect.bottom() - hgt),
                Pos2::new(x + slot - 1.0, rect.bottom()),
            );
            p.rect_filled(bar, 1.5, mix(ink.accent, ink.ground, 0.15));
            let cap_y = rect.bottom() - rect.height() * st.peaks[i].clamp(0.0, 1.0);
            p.rect_filled(
                Rect::from_min_size(
                    Pos2::new(x + 1.0, cap_y - 3.0),
                    vec2((slot - 2.0).max(1.0), 2.0),
                ),
                0.0,
                ink.strong,
            );
        }
    }
}

/// The STEREO chip: `Some(lit)` on the FM broadcast band (64–108 MHz in WFM),
/// where a station may send stereo, lit while the sound is stereo
/// ([`stereo_playing`]); `None` everywhere else, which is mono.
fn stereo_chip(hz: f64, mode: Mode, stereo: bool) -> Option<bool> {
    (mode == Mode::Wfm && (64e6..=108e6).contains(&hz)).then_some(stereo)
}

/// Whether the sound really is stereo: a pilot is locked **and** the decode is
/// on and not yielded to noise reduction. The pilot alone is not enough — with
/// ST switched off the pilot still locks and the sound is mono, and a lamp that
/// read the pilot lit up over a mono meter.
fn stereo_playing(rx: &sdroxide_types::RxState, pilot_locked: bool) -> bool {
    pilot_locked && rx.wfm_stereo && !rx.noise_reduction.is_on()
}

/// What the STEREO chip says on hover, and — dark — why it is dark.
fn stereo_note(rx: &sdroxide_types::RxState, lit: bool) -> String {
    if lit {
        tr!("rc-stereo-the-19-khz-pilot-is-locked").into()
    } else if !rx.wfm_stereo {
        tr!("rc-mono-the-broadcast-stereo-decode").into()
    } else if rx.noise_reduction.is_on() {
        tr!("rc-mono-noise-reduction-is-on-and").into()
    } else {
        tr!("rc-mono-no-19-khz-pilot-the-station").into()
    }
}

/// A false stereo for the meter of a mono signal: the two needles read the
/// even and the odd bands of the same sound, so they move a little apart as the
/// spectrum does, while together they keep the level the single ear measured.
/// The spread is held to ±2 dB. With no bands (the station sent only the
/// level) there is nothing to part them by, and they stay equal.
fn false_stereo(rms_db: [f32; 2], bands: &[u8]) -> [f32; 2] {
    const SPREAD_DB: f32 = 2.0;
    let power = |parity: usize| -> f32 {
        bands
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == parity)
            .map(|(_, b)| 10f32.powf((f32::from(*b) / 255.0 * 80.0 - 80.0) / 10.0))
            .sum()
    };
    let (even, odd) = (power(0), power(1));
    let total = even + odd;
    if total <= 0.0 {
        return rms_db;
    }
    // Each ear's share of the pair, 1.0 being an equal one.
    let side =
        |share: f32| (10.0 * (2.0 * share / total).max(1e-6).log10()).clamp(-SPREAD_DB, SPREAD_DB);
    let (l, r) = (side(even), side(odd));
    // Held to ±2 dB each, the pair would be a little louder than the single ear
    // was: put the difference back so the level stays what was measured.
    let mean = 10.0 * ((10f32.powf(l / 10.0) + 10f32.powf(r / 10.0)) / 2.0).log10();
    [rms_db[0] + l - mean, rms_db[1] + r - mean]
}

/// The STEREO pill: filled and bright while the pilot is locked, an empty dim
/// outline otherwise. Returns where it was drawn.
fn stereo_pill(p: &egui::Painter, at: Pos2, lit: bool, ink: &Ink) -> Rect {
    let (text, edge, fill) =
        if lit { (ink.ground, ink.accent, Some(ink.accent)) } else { (ink.dim, ink.line, None) };
    pill_rect(p, at, &tr!("rc-stereo"), text, edge, fill)
}

/// `w` points wide, in the middle of `outer` (all of it when it is wider).
fn centred_in(outer: Rect, w: f32) -> Rect {
    let w = w.min(outer.width());
    Rect::from_min_size(Pos2::new(outer.center().x - w / 2.0, outer.top()), vec2(w, outer.height()))
}

/// Which way a box was asked to move: `+1` forward (a click, wheel up), `-1`
/// back (a right-click, wheel down), `None` when it was left alone.
fn direction(click: bool, secondary: bool, wheel: i32) -> Option<i32> {
    if click {
        Some(1)
    } else if secondary {
        Some(-1)
    } else if wheel != 0 {
        Some(wheel.signum())
    } else {
        None
    }
}

/// The index, in the mode's presets, of the filter to switch to. The current
/// width is matched to a preset's (the CW presets move with the pitch, their
/// widths do not); when it is on none, the nearest one is the starting point.
fn next_preset(
    presets: &[(&str, f32, f32)],
    _mode: Mode,
    lo: f32,
    hi: f32,
    forward: bool,
) -> Option<usize> {
    if presets.is_empty() {
        return None;
    }
    let width = (hi - lo).abs();
    let at = presets
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| {
            ((a.2 - a.1).abs() - width).abs().total_cmp(&((b.2 - b.1).abs() - width).abs())
        })
        .map_or(0, |(i, _)| i);
    let n = presets.len();
    Some(if forward { (at + 1) % n } else { (at + n - 1) % n })
}

/// The tuning step after (or before) `hz` in the list the top bar cycles.
fn next_step(hz: f64, forward: bool) -> f64 {
    let steps = sdroxide_types::UiSettings::TUNE_STEPS_HZ;
    let n = steps.len();
    let at = steps.iter().position(|s| (s - hz).abs() < 0.5);
    let i = match (at, forward) {
        (Some(i), true) => (i + 1) % n,
        (Some(i), false) => (i + n - 1) % n,
        (None, true) => 0,
        (None, false) => n - 1,
    };
    steps[i]
}

/// The AGC speed after (or before) `agc`: Off → Slow → Med → Fast.
fn next_agc(agc: AgcMode, forward: bool) -> AgcMode {
    let all = AgcMode::ALL;
    let at = all.iter().position(|a| *a == agc).unwrap_or(0);
    all[if forward { (at + 1) % all.len() } else { (at + all.len() - 1) % all.len() }]
}

/// One wheel notch on the squelch: 3 dB a notch, `up` raising the gate. From
/// open the first notch up lands at -110 dBFS, and the way down ends open.
fn sql_wheel(db: f32, notches: i32) -> f32 {
    const STEP_DB: f32 = 3.0;
    const FIRST_DB: f32 = -110.0;
    let from = if db <= SQUELCH_OPEN_DB + 0.5 { FIRST_DB - STEP_DB } else { db };
    let to = from + STEP_DB * notches as f32;
    if to < FIRST_DB - 0.5 { SQUELCH_OPEN_DB } else { to.min(SQUELCH_CLOSED_DB) }
}

/// A short mode tag for the right column's corner.
fn mode_tag(mode: Mode) -> String {
    MODES
        .iter()
        .find(|(_, m)| *m == mode)
        .map_or_else(|| format!("{mode:?}").to_uppercase(), |(l, _)| (*l).to_string())
}

/// A small outlined label; returns the x it ends at.
fn pill(p: &egui::Painter, at: Pos2, text: &str, ink: Color32, edge: Color32) -> f32 {
    pill_rect(p, at, text, ink, edge, None).right()
}

/// [`pill`], optionally filled, and returning the rectangle it took.
fn pill_rect(
    p: &egui::Painter,
    at: Pos2,
    text: &str,
    ink: Color32,
    edge: Color32,
    fill: Option<Color32>,
) -> Rect {
    let galley = p.layout_no_wrap(text.to_string(), FontId::proportional(11.5), ink);
    let r = Rect::from_min_size(at, galley.size() + vec2(16.0, 8.0));
    if let Some(fill) = fill {
        p.rect_filled(r, r.height() / 2.0, fill);
    }
    p.rect_stroke(r, r.height() / 2.0, Stroke::new(1.0, edge), StrokeKind::Inside);
    p.galley(Pos2::new(r.left() + 8.0, r.top() + 4.0), galley, ink);
    r
}

/// A station name split into the name proper and what it carries in brackets:
/// `"Belgien Klara (Sint-Pieters-Leeuw)"` is `("Belgien Klara", Some("(Sint-Pieters-Leeuw)"))`.
/// Without a bracket after a first word the name is all main.
fn split_name(name: &str) -> (&str, Option<&str>) {
    match name.find(" (") {
        Some(i) if i > 0 => (name[..i].trim_end(), Some(name[i..].trim())),
        _ => (name, None),
    }
}

/// The big station name: letter-spaced, centred on its anchor and wrapped to
/// two lines of at most `width`, with an ellipsis if it is longer still.
fn name_job(text: &str, size: f32, width: f32, colour: Color32) -> egui::text::LayoutJob {
    let mut format = egui::TextFormat::simple(FontId::proportional(size), colour);
    format.extra_letter_spacing = size * 0.3;
    let mut job = egui::text::LayoutJob::single_section(text.to_string(), format);
    job.wrap = egui::text::TextWrapping {
        max_width: width,
        max_rows: 2,
        break_anywhere: false,
        overflow_character: Some('…'),
    };
    job.halign = Align::Center;
    job
}

/// RadioText in its box, scrolling when it is longer than the box.
fn marquee(ui: &egui::Ui, rect: Rect, text: &str, ink: &Ink) {
    let p = ui.painter_at(rect);
    p.rect_filled(rect, 6.0, mix(ink.ground, ink.line, 0.12));
    p.rect_stroke(rect, 6.0, Stroke::new(1.0, ink.line), StrokeKind::Inside);
    let tag_end = pill(&p, rect.left_top() + vec2(8.0, 5.0), "RT", ink.accent, ink.accent);
    let text_rect =
        Rect::from_min_max(Pos2::new(tag_end + 10.0, rect.top()), rect.max - vec2(8.0, 0.0));
    let clip = ui.painter_at(text_rect);
    let font = FontId::proportional(15.0);
    let galley = clip.layout_no_wrap(text.to_string(), font.clone(), ink.strong);
    let overflow = galley.size().x - text_rect.width();
    if overflow <= 0.0 {
        clip.text(
            Pos2::new(text_rect.left(), rect.center().y),
            Align2::LEFT_CENTER,
            text,
            font,
            ink.strong,
        );
    } else {
        let gap = 48.0;
        let t = ui.input(|i| i.time) as f32;
        let off = (t * 36.0) % (galley.size().x + gap);
        for k in 0..2 {
            clip.text(
                Pos2::new(
                    text_rect.left() - off + k as f32 * (galley.size().x + gap),
                    rect.center().y,
                ),
                Align2::LEFT_CENTER,
                text,
                font.clone(),
                ink.strong,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_name_splits_at_its_bracket() {
        assert_eq!(
            split_name("Belgien Klara (Sint-Pieters-Leeuw Nahe Brüssel)"),
            ("Belgien Klara", Some("(Sint-Pieters-Leeuw Nahe Brüssel)"))
        );
        assert_eq!(split_name("EUROPE 1"), ("EUROPE 1", None));
        assert_eq!(split_name("(x)"), ("(x)", None));
    }
    use RadioClassicPlace::*;

    #[test]
    fn off_is_hidden_and_on_docks_in_a_voice_mode() {
        assert_eq!(place(false, false, Mode::Wfm, false), Hidden);
        assert_eq!(place(true, false, Mode::Wfm, false), Docked);
        assert_eq!(place(true, false, Mode::Usb, false), Docked);
    }

    #[test]
    fn a_detached_panel_is_a_window_whatever_else_is_true() {
        for mode in [Mode::Wfm, Mode::Cw, Mode::Ft8, Mode::Dab] {
            for on in [false, true] {
                assert_eq!(place(on, true, mode, false), Window, "{mode:?} on={on}");
            }
        }
    }

    #[test]
    fn where_the_slot_is_taken_or_there_is_no_room_it_is_a_window_not_nothing() {
        for mode in [Mode::Ft8, Mode::Dab, Mode::Cw] {
            assert_eq!(place(true, false, mode, false), Window, "{mode:?}");
        }
        assert_eq!(place(true, false, Mode::Wfm, true), Window, "a phone has no room to dock");
    }

    fn memory(name: &str, hz: f64, mode: Mode) -> MemoryChannel {
        MemoryChannel {
            id: 1,
            name: name.into(),
            freq_hz: hz,
            mode,
            filter_lo: 0.0,
            filter_hi: 0.0,
            folder: None,
            rtty: None,
            repeater: None,
            antenna: None,
            color: None,
        }
    }

    fn tropical() -> BroadcastStation {
        BroadcastStation {
            name: "Tropical R.Program 4TRP".into(),
            freq_khz: 2355.0,
            lang: "English".into(),
            country: "Australia".into(),
            site: "Kuranda QLD".into(),
            ..Default::default()
        }
    }

    #[test]
    fn on_fm_the_name_is_the_stations_and_elsewhere_the_memorys_or_the_schedules() {
        let big = |m, ps, mem: Option<&MemoryChannel>, sch: Option<&BroadcastStation>| {
            station_lines(m, ps, mem, sch).map(|l| l.big)
        };
        let m3 = memory("M3", 99.0e6, Mode::Wfm);
        let bbc = BroadcastStation { name: "BBC".into(), freq_khz: 9410.0, ..Default::default() };
        assert_eq!(big(Mode::Wfm, Some("EUROPE 1"), Some(&m3), None).as_deref(), Some("EUROPE 1"));
        // A station that sends no name falls back to the memory, then the schedule.
        assert_eq!(big(Mode::Wfm, Some("  "), Some(&m3), Some(&bbc)).as_deref(), Some("M3"));
        assert_eq!(big(Mode::Wfm, None, None, Some(&bbc)).as_deref(), Some("31m BBC"));
        // Off FM the RDS name is never used, even if one is lying around.
        let rfi = BroadcastStation { name: "RFI".into(), freq_khz: 9410.0, ..Default::default() };
        assert_eq!(big(Mode::Am, Some("EUROPE 1"), None, Some(&rfi)).as_deref(), Some("31m RFI"));
        let r3 = memory("Radio 3", 9.41e6, Mode::Am);
        assert_eq!(
            big(Mode::Am, Some("EUROPE 1"), Some(&r3), Some(&rfi)).as_deref(),
            Some("RADIO 3")
        );
        assert_eq!(big(Mode::Usb, None, None, None), None);
    }

    #[test]
    fn a_scheduled_station_is_band_and_name_big_and_what_it_says_about_itself_above() {
        let lines = station_lines(Mode::Am, None, None, Some(&tropical())).unwrap();
        assert_eq!(lines.big, "120m TROPICAL R.PROGRAM 4TRP");
        assert_eq!(lines.above.as_deref(), Some("English Australia Kuranda QLD"));
        assert!(lines.below.is_empty());
        // What the table leaves empty is not written as a gap.
        let bare = BroadcastStation { site: String::new(), ..tropical() };
        let lines = station_lines(Mode::Am, None, None, Some(&bare)).unwrap();
        assert_eq!(lines.above.as_deref(), Some("English Australia"));
    }

    #[test]
    fn a_memory_is_its_name_big_and_its_dial_and_mode_under_it() {
        let m = memory("Qso MATIN", 3.6e6, Mode::Lsb);
        let lines = station_lines(Mode::Lsb, None, Some(&m), None).unwrap();
        assert_eq!(lines.big, "QSO MATIN");
        assert_eq!(lines.above, None);
        assert_eq!(lines.below, vec!["3.600000 MHz LSB".to_string()]);
    }

    #[test]
    fn the_s_meter_reads_like_a_radio() {
        assert_eq!(s_reading(-73.0), "S9");
        assert_eq!(s_reading(-53.0), "S9+20");
        assert_eq!(s_reading(-109.0), "S3");
        assert_eq!(s_reading(-140.0), "S0");
        assert!(smeter_fill(-127.0) < 0.001 && smeter_fill(-13.0) > 0.999);
        assert!(smeter_fill(-200.0) == 0.0 && smeter_fill(40.0) == 1.0);
        // S9 sits where the scale prints it, a little under the middle.
        assert!((smeter_fill(-73.0) - 0.4737).abs() < 0.001);
    }

    #[test]
    fn the_fm_button_is_lit_inside_the_band_and_the_extension_below_it() {
        let row = broadcast_row();
        let lit = |hz: f64| -> Vec<&str> {
            row.iter().filter(|p| p.holds(hz)).map(|p| p.label.as_str()).collect()
        };
        assert_eq!(lit(105.5e6), vec!["FM 87.5–108"]);
        assert_eq!(lit(76.0e6), vec!["FM ext 64–87.5"]);
        assert!(lit(1.0e6).contains(&"MW EU"));
    }

    #[test]
    fn the_shortwave_row_is_the_broadcast_bands_in_order() {
        let row = shortwave_row();
        let labels: Vec<&str> = row.iter().map(|p| p.label.as_str()).collect();
        assert_eq!(labels.first(), Some(&"120"));
        assert_eq!(labels.last(), Some(&"11"));
        assert_eq!(labels.len(), broadcast::METRE_BANDS.len());
        // A click lands inside the band it names, in AM, on a 5 kHz channel.
        for p in &row {
            assert!(p.holds(p.tune_hz), "{} tunes outside itself", p.label);
            assert_eq!(p.mode, Mode::Am);
            assert_eq!((p.tune_hz / 1e3) % 5.0, 0.0, "{}", p.label);
        }
    }

    #[test]
    fn every_group_has_something_to_click() {
        for (g, _) in Group::ALL {
            let rows = group_rows(g);
            assert!(!rows.is_empty() && rows.iter().all(|(_, r)| !r.is_empty()), "{g:?}");
        }
    }

    #[test]
    fn a_readout_is_placed_in_the_middle_of_its_room() {
        let room = Rect::from_min_size(Pos2::new(100.0, 10.0), vec2(400.0, 50.0));
        let r = centred_in(room, 200.0);
        assert!((r.center().x - room.center().x).abs() < 0.01 && r.width() == 200.0);
        assert_eq!(centred_in(room, 900.0), room, "wider than the room: the room");
    }

    #[test]
    fn the_waterfall_keeps_three_rows_in_four_up_to_four_hundred() {
        let mut st = State::default();
        for i in 0..20u8 {
            st.wf_push(vec![i; WF_BINS]);
        }
        assert_eq!(st.wf.len(), 15);
        // Newest first: the last kept frame is the 19th (index 18).
        assert_eq!(st.wf.front().map(|r| r[0]), Some(18));
        for _ in 0..2000 {
            st.wf_push(vec![0; WF_BINS]);
        }
        assert_eq!(st.wf.len(), WF_ROWS);
        assert_eq!(WF_ROWS, 400);
    }

    #[test]
    fn the_scope_shows_the_filter_with_a_margin_round_it() {
        // WFM: a wide filter fills the width, centred on the dial.
        let (lo, span) = scope_window(100e6, -96_000.0, 96_000.0);
        assert!((span - 192_000.0 * 1.2).abs() < 1.0);
        assert!((lo + span / 2.0 - 100e6).abs() < 1.0);
        // USB: the window sits over the passband, so the dial is near its left edge.
        let (lo, span) = scope_window(14.2e6, 200.0, 2600.0);
        assert!((span - 2400.0 * 1.2).abs() < 1.0);
        assert!(lo > 14.2e6 - 100.0 && lo < 14.2e6 + 200.0);
        assert!(lo + span > 14.2e6 + 2600.0);
        // A narrow CW filter does not zoom past what the bins can show.
        assert_eq!(scope_window(7.03e6, -50.0, 50.0).1, SCOPE_MIN_SPAN_HZ);
    }

    #[test]
    fn a_level_between_two_bins_is_read_on_the_line_between_them() {
        let bins = [0u8, 100, 200];
        // Bin centres at lo + span * (0.5, 1.5, 2.5) / 3.
        assert_eq!(level_at(&bins, 0.0, 3.0, 1.5), 100);
        assert_eq!(level_at(&bins, 0.0, 3.0, 1.0), 50);
        assert_eq!(level_at(&bins, 0.0, 3.0, -1.0), 0, "outside the frame");
        assert_eq!(level_at(&bins, 0.0, 3.0, 4.0), 0, "outside the frame");
    }

    #[test]
    fn rows_made_for_another_span_are_dropped() {
        let mut st = State::default();
        st.wf_span_is(10_000.0);
        st.wf_push(vec![1; WF_BINS]);
        st.wf_span_is(10_050.0);
        assert_eq!(st.wf.len(), 1, "a hair of change keeps the picture");
        st.wf_span_is(2_400.0);
        assert!(st.wf.is_empty());
    }

    #[test]
    fn the_waterfall_picture_averages_rows_to_the_pixels_it_lands_on() {
        let ink = [Color32::BLACK, Color32::from_rgb(0, 100, 200), Color32::WHITE];
        let mut rows = VecDeque::new();
        // Two bright rows on top of two dark ones, nothing below.
        for v in [255u8, 255, 0, 0] {
            rows.push_back(vec![v; WF_BINS]);
        }
        let img = wf_image(&rows, 100, &ink);
        assert_eq!(img.size, [WF_BINS, 100]);
        // 400 rows over 100 pixels: four rows a pixel, so the first pixel row
        // holds all four, half of them bright.
        let first = img.pixels[0];
        assert!(first != Color32::BLACK && first != Color32::WHITE, "{first:?}");
        // Below the filled part it is the ground.
        assert_eq!(img.pixels[WF_BINS * 50], Color32::BLACK);
    }

    #[test]
    fn the_boxes_step_through_their_settings_both_ways() {
        // Filters: the mode's presets, wrapping at both ends.
        let usb = Mode::Usb.filter_presets();
        assert_eq!(next_preset(usb, Mode::Usb, 200.0, 2600.0, true), Some(2));
        assert_eq!(next_preset(usb, Mode::Usb, 200.0, 2600.0, false), Some(0));
        assert_eq!(next_preset(usb, Mode::Usb, 100.0, 3400.0, true), Some(0));
        assert_eq!(next_preset(Mode::Wfm.filter_presets(), Mode::Wfm, 0.0, 1.0, true), None);
        // The tuning step goes round the top bar's list.
        let steps = sdroxide_types::UiSettings::TUNE_STEPS_HZ;
        assert_eq!(next_step(steps[0], true), steps[1]);
        assert_eq!(next_step(steps[0], false), steps[steps.len() - 1]);
        // AGC: Off → Slow → Med → Fast → Off.
        assert_eq!(next_agc(AgcMode::Med, true), AgcMode::Fast);
        assert_eq!(next_agc(AgcMode::Off, false), AgcMode::Fast);
        assert_eq!(next_agc(AgcMode::Fast, true), AgcMode::Off);
        // Click forward, right-click back, wheel by its sign.
        assert_eq!(direction(true, false, 0), Some(1));
        assert_eq!(direction(false, true, 3), Some(-1));
        assert_eq!(direction(false, false, -2), Some(-1));
        assert_eq!(direction(false, false, 0), None);
    }

    #[test]
    fn the_squelch_wheel_runs_from_open_up_to_closed_and_back() {
        let up = sql_wheel(SQUELCH_OPEN_DB, 1);
        assert_eq!(up, -110.0);
        assert_eq!(sql_wheel(up, 1), -107.0);
        // One notch down from the first threshold is open again.
        assert_eq!(sql_wheel(up, -1), SQUELCH_OPEN_DB);
        assert_eq!(sql_wheel(SQUELCH_OPEN_DB, -1), SQUELCH_OPEN_DB);
        assert_eq!(sql_wheel(-1.0, 5), SQUELCH_CLOSED_DB);
    }

    #[test]
    fn the_stereo_chip_exists_only_on_the_fm_band_and_is_lit_by_the_pilot() {
        assert_eq!(stereo_chip(98e6, Mode::Wfm, true), Some(true));
        assert_eq!(stereo_chip(98e6, Mode::Wfm, false), Some(false));
        assert_eq!(stereo_chip(64e6, Mode::Wfm, true), Some(true));
        assert_eq!(stereo_chip(108e6, Mode::Wfm, true), Some(true));
        assert_eq!(stereo_chip(150e6, Mode::Wfm, true), None);
        assert_eq!(stereo_chip(63.9e6, Mode::Wfm, true), None);
        assert_eq!(stereo_chip(98e6, Mode::Nfm, true), None);
    }

    #[test]
    fn the_stereo_lamp_needs_the_decode_on_as_well_as_the_pilot() {
        let mut rx = sdroxide_types::RxState::with_mode(Mode::Wfm);
        rx.wfm_stereo = true;
        assert!(stereo_playing(&rx, true));
        assert!(!stereo_playing(&rx, false), "no pilot: mono");
        rx.wfm_stereo = false;
        assert!(!stereo_playing(&rx, true), "ST off: the pilot locks and the sound is mono");
        rx.wfm_stereo = true;
        rx.noise_reduction = sdroxide_types::NrLevel::Medium;
        assert!(!stereo_playing(&rx, true), "stereo yields to noise reduction");
    }

    #[test]
    fn a_dark_stereo_chip_says_why() {
        let mut rx = sdroxide_types::RxState::with_mode(Mode::Wfm);
        assert!(stereo_note(&rx, false).contains("no 19 kHz pilot"));
        rx.noise_reduction = sdroxide_types::NrLevel::Medium;
        assert!(stereo_note(&rx, false).contains("noise reduction"));
        rx.wfm_stereo = false;
        assert!(stereo_note(&rx, false).contains("switched off"));
        assert!(stereo_note(&rx, true).starts_with("Stereo"));
    }

    #[test]
    fn a_false_stereo_parts_the_needles_a_little_and_keeps_the_level() {
        let bands: Vec<u8> = (0..AUDIO_BANDS).map(|i| if i % 2 == 0 { 220 } else { 140 }).collect();
        let [l, r] = false_stereo([-20.0, -20.0], &bands);
        assert!(l > r, "the louder bands are the left ear: {l} {r}");
        assert!((l - r).abs() <= 4.0 + 1e-3, "spread held to ±2 dB: {l} {r}");
        // Together they stay where the single ear measured.
        let mean = 10.0 * ((10f32.powf(l / 10.0) + 10f32.powf(r / 10.0)) / 2.0).log10();
        assert!((mean + 20.0).abs() < 0.01, "{mean}");
        // Nothing to part them by: equal, as they were.
        assert_eq!(false_stereo([-20.0, -20.0], &[]), [-20.0, -20.0]);
        assert_eq!(false_stereo([-20.0, -20.0], &[0; AUDIO_BANDS]), [-20.0, -20.0]);
    }

    #[test]
    fn the_columns_fill_the_panel_without_overlapping() {
        for w in [1180.0f32, 1500.0, 1896.0] {
            let (l, m, r) = column_widths(w);
            assert!(l + m + r + 16.0 <= w + 0.5, "{w}: {l}+{m}+{r}");
            assert!(r > l && r > m, "the readout is the widest column at {w}");
        }
    }
}
