//! The per-frame loop: drain what the engine sent, then lay the window out.
//!
//! [`eframe::App::update`] is the one entry point the framework calls, and
//! everything else in [`crate::app`] hangs off it. Event handling comes first
//! so the frame draws the newest state, and the repaint request at the end is
//! what keeps the app idling instead of spinning when no stream is flowing.

use crate::i18n::tr;
use eframe::egui::{self, Color32, RichText};
use sdroxide_types::{Band, Command, Mode, RadioEvent, SpectrumConfig, Spot};

use crate::time::now_unix;
use crate::widgets::spectrum_view;

use crate::app::SdroxideApp;
#[cfg(not(target_arch = "wasm32"))]
use crate::app::ToolWindowState;
use crate::app::net::auto_upload_adif;
use crate::app::persist::persist_qso_log;
use crate::app::settings::servers::TciServerStatus;
use crate::app::spectrum::CFG_DEBOUNCE_S;

/// Repaint-poll cadence when no spectrum stream is flowing (startup, connection
/// lost, stalled stream) — the app truly idles between these wakes.
const IDLE_POLL_MS: u64 = 250;

/// The stream counts as stalled after this long without a new frame (seconds).
const STREAM_STALE_S: f64 = 1.0;

/// Split the digital-mode area between the waterfall and the operating panel.
///
/// Returns `(waterfall height, panel height)`, which always sum to
/// `total - divider_h`: the panel is clamped and the waterfall takes what is
/// left, rather than both being floored independently and their sum allowed
/// past the height there actually is.
///
/// `divider_h` is everything between the two — the drag handle *and* the two
/// gaps the layout puts either side of it. Leaving the gaps out of it is what
/// let the panel start `2 × item_spacing` below where the arithmetic thought it
/// did, and hang that far off the bottom of the window: ten points on a desktop,
/// which the transcript quietly absorbed, and fourteen on a touched layout,
/// which took the action buttons with it.
///
/// `row_h` is the layout's own row height (`interact_size.y`), and it is what
/// the panel's floor is built from. Every part of the panel that cannot be done
/// without — its header, the station card, the two rows pinned to its bottom
/// edge — is a chip, a button or a text field sized from that, and the touched
/// layouts make each of them half again as tall. The 190 points this used to be
/// was measured on a desktop, so on a 1366×768 screen (which is the tablet tier,
/// touch metrics and all) the panel came up about fifty points short: the
/// transcript's scroll area hit its own minimum, overflowed the allocation it
/// had been given, and painted over the message row underneath — so the row
/// that chooses what goes out next was simply not on the screen, with nothing
/// to say it was missing (issue #231).
fn digi_split(total: f32, divider_h: f32, fraction: f32, row_h: f32) -> (f32, f32) {
    let usable = (total - divider_h).max(0.0);
    // 170 points of fixed furniture plus four rows: measured against a 1290×724
    // window, where 296 points is where the transcript stops being squeezed
    // into less than its own scroll area will accept. Capped at 60% of the
    // area so a floor can never leave the waterfall a sliver.
    let min_panel = (170.0 + 4.0 * row_h.max(18.0)).min(usable * 0.6);
    let min_wf = 80.0_f32.min(usable * 0.5);
    let panel_h = (usable * fraction).clamp(min_panel, (usable - min_wf).max(min_panel));
    (usable - panel_h, panel_h)
}

/// The panadapter/operating-panel split for one column.
///
/// Either part can be **undocked** (drawn in its own window), and the
/// panadapter can also have both its layers switched off — in every one of
/// those cases it is not drawn here, so the other part takes the whole height
/// rather than sharing with an empty strip. Split out from the frame loop so
/// the decision has a pure test: the windows themselves cannot be asserted
/// headlessly, but the *main window reclaiming the column* can.
pub(in crate::app) fn column_split(
    wf_undocked: bool,
    layers: bool,
    panel_undocked: bool,
    total: f32,
    divider_h: f32,
    fraction: f32,
    row_h: f32,
) -> (f32, f32) {
    let wf_hidden = !layers || wf_undocked;
    match (wf_hidden, panel_undocked) {
        (true, true) => (0.0, 0.0),
        (true, false) => (0.0, total),
        (false, true) => (total, 0.0),
        (false, false) => digi_split(total, divider_h, fraction, row_h),
    }
}

/// Whether a state update that kept the mode moved the receiver far enough to
/// throw the copied decodes away.
///
/// Band-level rather than a frequency delta, the way WSJT-X's own
/// `band_changed()` is: tuning about within a band — the FT8 slot to the FT4
/// slot, a DXpedition window, the other end of the digital segment — is still
/// the same band opening, and the list is still about it. Crossing into another
/// band is a different opening and a different set of stations, and a decode
/// carries nothing that would say which of the two it came from.
///
/// `now`/`prev` come from the engine's own [`sdroxide_types::RadioState::band`]
/// rather than being derived here, so a QSY made at the radio counts exactly as
/// one made in the program — the engine sets that field from the dial the CAT
/// poll reports, and this comparison never has to know where the change came
/// from.
///
/// WSPR is exempt: its band hopping crosses a band edge once a slot on purpose,
/// and its spots each carry their own RF frequency, so there is nothing
/// ambiguous to clear away.
fn qsy_clears_decodes(prev: Band, now: Band, mode: Mode) -> bool {
    now != prev && !mode.is_wspr()
}

/// The banner palette above the panadapter — (wash, rule, mark, ink), one
/// look shared by the radio warnings and the update notice. Amber wash, amber
/// rule, readable ink; the light-theme set is the same banner turned the
/// other way up, for a theme whose panels are white and which would otherwise
/// get pale text on a dark strip in the middle of a bright page.
pub(in crate::app) fn notice_banner_colors() -> (Color32, Color32, Color32, Color32) {
    if crate::theme::is_light() {
        (
            Color32::from_rgb(255, 243, 205),
            Color32::from_rgb(178, 122, 0),
            Color32::from_rgb(140, 92, 0),
            Color32::from_rgb(58, 44, 8),
        )
    } else {
        (
            Color32::from_rgb(60, 45, 10),
            Color32::from_rgb(210, 160, 40),
            Color32::from_rgb(255, 190, 70),
            Color32::from_rgb(240, 220, 180),
        )
    }
}

/// The panadapter's chrome: the pink cut-corner border and corner brackets,
/// drawn around the whole block rather than the picture alone.
///
/// The level slider's gutter is part of that block, so the frame has to take
/// the gutter in — drawn around the picture only (as `spectrum_view` used to),
/// the border stops short of the block's right edge while the control floats
/// outside it, and the whole box reads as jumped. The picture is still handed
/// its own rect; this is only the frame around the two of them.
fn paint_panadapter_chrome(ui: &egui::Ui, area: egui::Rect) {
    let painter = ui.painter();
    crate::chrome::paint_cut_border(
        painter,
        area.shrink(0.8),
        crate::theme::scope().chrome,
        crate::theme::scope().shell,
    );
    crate::chrome::corner_brackets(painter, area, crate::theme::scope().chrome);
}

/// The "the levels are hiding the picture" hint: a clickable pill centred on
/// the panadapter, shown only while auto-fit is off and every visible bin sits
/// on the display floor or ceiling. Returns true when clicked.
///
/// A waterfall in that state is indistinguishable from one with no data — the
/// frame is real, the floor/ceiling are just past everything on it — and the
/// FIT chip that fixes it lives at the bottom of the Display module, which is
/// not where anyone is looking at a flat black screen. The fix belongs next to
/// the symptom, and one click of this pill is the same as switching FIT back on
/// (`SdroxideApp::fit_levels_now`).
fn levels_hidden_chip(ui: &mut egui::Ui, area: egui::Rect) -> bool {
    let galley = ui.painter().layout_no_wrap(
        tr!("frame-no-contrast-fit"),
        egui::FontId::proportional(13.0),
        egui::Color32::WHITE,
    );
    let rect = egui::Rect::from_center_size(area.center(), galley.size() + egui::vec2(24.0, 12.0));
    let resp = ui.interact(
        rect,
        crate::layout::salted_id(ui.ctx(), "levels-hidden-hint"),
        egui::Sense::click(),
    );
    let p = ui.painter_at(rect);
    p.rect_filled(rect, 5.0, egui::Color32::from_black_alpha(205));
    if resp.hovered() {
        p.rect_stroke(
            rect,
            5.0,
            egui::Stroke::new(1.0, crate::theme::CYAN()),
            egui::StrokeKind::Inside,
        );
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    p.galley(rect.center() - galley.size() / 2.0, galley, egui::Color32::WHITE);
    resp.on_hover_text(tr!("frame-levels-hidden-hint")).clicked()
}

/// A short note centred on the panadapter, over a dark pill so it reads on any
/// waterfall. Used for the ATS Mini's tune-in-flight line, which belongs where
/// an audio-only source's eye already is.
fn centred_waterfall_note(ui: &egui::Ui, area: egui::Rect, text: &str) {
    let p = ui.painter_at(area);
    let ink = egui::Color32::LIGHT_GRAY;
    let font = egui::FontId::proportional(14.0);
    let galley = p.layout_no_wrap(text.to_owned(), font, ink);
    let box_rect =
        egui::Rect::from_center_size(area.center(), galley.size() + egui::vec2(20.0, 10.0));
    p.rect_filled(box_rect, 4.0, egui::Color32::from_black_alpha(180));
    p.galley(box_rect.center() - galley.size() / 2.0, galley, ink);
}

/// The bandwidth line's font size: the clock's 11 pt leaves the bandwidth text
/// a few points short of the clock's ink, so it is set a touch larger to fill
/// the shared pill. Measured against Chakra Petch, the app's proportional face.
const BW_LINE_FONT_PT: f32 = 11.6;

/// Gap between the clock pill and the bandwidth pill.
const PANADAPTER_PILL_GAP: f32 = 4.0;

/// `BW waterfall · 148.0 kHz`, or `BW waterfall · —` before the first frame has
/// given the view a window.
///
/// The width comes from [`ViewState::span`](crate::view::ViewState::span) — the
/// same range the frequency ruler is drawn from — not from parsing the ruler's
/// labels.
fn waterfall_bw_text(span_hz: f64) -> String {
    if span_hz.is_finite() && span_hz > 0.0 {
        tr!("frame-bw-waterfall", span = spectrum_view::format_bandwidth(span_hz))
    } else {
        tr!("frame-bw-waterfall-unknown")
    }
}

/// The two pills' rectangles: one shared width, both right-aligned to the
/// panadapter's 8 pt margin, the bandwidth line 4 pt under the clock.
///
/// The width is the wider of the two texts plus the pill's 12 pt of padding, so
/// neither line is clipped and the boxes line up. The height is the clock's own,
/// so adding the line cannot change the clock's look.
fn panadapter_pill_rects(
    area: egui::Rect,
    clock_text: egui::Vec2,
    bw_text: egui::Vec2,
) -> (egui::Rect, egui::Rect) {
    let (clock, bw, _) = panadapter_pill_stack(area, clock_text, bw_text);
    (clock, bw)
}

/// The three pills: the BAND TX pill sits under the bandwidth line with the
/// same width and twice the height.
fn panadapter_pill_stack(
    area: egui::Rect,
    clock_text: egui::Vec2,
    bw_text: egui::Vec2,
) -> (egui::Rect, egui::Rect, egui::Rect) {
    let size = egui::vec2(clock_text.x.max(bw_text.x) + 12.0, clock_text.y + 7.0);
    let clock =
        egui::Rect::from_min_size(egui::pos2(area.right() - size.x - 8.0, area.top() + 8.0), size);
    let bw = egui::Rect::from_min_size(
        egui::pos2(clock.left(), clock.bottom() + PANADAPTER_PILL_GAP),
        size,
    );
    let band_tx = egui::Rect::from_min_size(
        egui::pos2(clock.left(), bw.bottom() + PANADAPTER_PILL_GAP),
        egui::vec2(size.x, size.y * 2.0),
    );
    (clock, bw, band_tx)
}

/// The wall clock and the waterfall's visible bandwidth, top-right of the
/// panadapter: UTC and the operator's own local time, so a listener always has
/// both in view (#655), with the width the ruler spans beneath it.
///
/// Draw-only over the picture — it never reflows the strip — and the local half
/// of the clock is dropped when the operator is already on UTC.
fn paint_panadapter_clock(
    ui: &egui::Ui,
    area: egui::Rect,
    span_hz: f64,
    pill: Option<super::tx_bands::BandTxPill>,
) {
    let clock = crate::time::utc_and_local_now(crate::time::now_unix());
    let bw = waterfall_bw_text(span_hz);
    let p = ui.painter_at(area);
    let ink = egui::Color32::LIGHT_GRAY;
    let clock_galley = p.layout_no_wrap(clock, egui::FontId::proportional(11.0), ink);
    let bw_galley = p.layout_no_wrap(bw, egui::FontId::proportional(BW_LINE_FONT_PT), ink);
    let (clock_galley_size, bw_galley_size) = (clock_galley.size(), bw_galley.size());
    let (clock_rect, bw_rect) = panadapter_pill_rects(area, clock_galley.size(), bw_galley.size());
    for rect in [clock_rect, bw_rect] {
        p.rect_filled(rect, 4.0, egui::Color32::from_black_alpha(150));
    }
    p.galley(clock_rect.center() - clock_galley.size() / 2.0, clock_galley, ink);
    p.galley(bw_rect.center() - bw_galley.size() / 2.0, bw_galley, ink);
    if let Some(pill) = pill {
        use super::tx_bands::BandTxPill;
        let (_, _, rect) = panadapter_pill_stack(area, clock_galley_size, bw_galley_size);
        let t = ui.input(|i| i.time);
        // Blinking is driven by the interface clock: a repaint twice a second
        // while transmitting, nothing otherwise.
        let lit = pill != BandTxPill::Transmitting || (t * 2.0) as i64 % 2 == 0;
        if pill == BandTxPill::Transmitting {
            ui.ctx().request_repaint_after(std::time::Duration::from_millis(250));
        }
        let color = match pill {
            BandTxPill::Allowed => egui::Color32::from_rgb(46, 204, 64),
            _ => egui::Color32::from_rgb(255, 59, 48),
        };
        p.rect_filled(rect, 4.0, egui::Color32::BLACK);
        if lit {
            let g = p.layout_no_wrap(
                tr!("frame-band-tx"),
                egui::FontId::proportional(BW_LINE_FONT_PT),
                color,
            );
            p.galley(rect.center() - g.size() / 2.0, g, color);
        }
    }
}

/// The dial as a large readout string, MHz with trailing zeros trimmed —
/// "27.265", "144.8", "10.1206". Shared by the detached panadapter's overlay.
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn readout_text(hz: f64) -> String {
    let mhz = format!("{:.6}", hz.max(0.0) / 1e6);
    format!("{} MHz", mhz.trim_end_matches('0').trim_end_matches('.'))
}

/// Height of SP1's toolbar — the readout's line and the chips beside it. The
/// spectrum is drawn **below** the bar, not under it, which is the whole point of
/// moving the readout out of the middle of the picture: a waterfall you cannot
/// see the centre of is worse than a number in the way.
#[cfg(not(target_arch = "wasm32"))]
const SP1_TOOLBAR_H: f32 = 32.0;

/// The detached window's second line: the mode, and the S-meter reading when
/// there is one. Read-only — it shares the main window's meter, so it cannot
/// disagree with it.
#[cfg(not(target_arch = "wasm32"))]
fn detached_status_line(mode: Mode, meters: Option<&sdroxide_types::Meters>) -> String {
    match meters {
        Some(m) => {
            let (s, over) = m.s_units();
            let level = if over >= 1.0 { format!("S{s}+{over:.0}") } else { format!("S{s}") };
            format!("{}  ·  {}  ({} dBm)", mode.label(), level, m.s_dbm.round() as i32)
        }
        None => mode.label().to_string(),
    }
}

/// The plain face a main window shows when every part of the radio is in its own
/// window **and** the controls window already carries the band selector — so the
/// selector is not drawn twice, and the centre is never a black hole.
#[cfg(not(target_arch = "wasm32"))]
fn empty_centre(ui: &egui::Ui) {
    let area = ui.max_rect();
    let p = ui.painter_at(area);
    p.rect_filled(area, 0.0, crate::theme::BG_DEEP());
    let (ink, dim) = (crate::theme::gray(105), crate::theme::gray(85));
    let name = p.layout_no_wrap(
        sdroxide_version::FLAVOR.to_owned(),
        egui::FontId::proportional(22.0),
        ink,
    );
    let hint = p.layout_no_wrap(
        tr!("frame-every-module-own-window"),
        egui::FontId::proportional(13.0),
        dim,
    );
    let (name_w, name_h, hint_w) = (name.size().x, name.size().y, hint.size().x);
    let top = area.center().y - (name_h + 8.0 + hint.size().y) / 2.0;
    p.galley(egui::pos2(area.center().x - name_w / 2.0, top), name, ink);
    p.galley(egui::pos2(area.center().x - hint_w / 2.0, top + name_h + 8.0), hint, dim);
}

/// The **⇱** button for a tool window's **title bar**, beside egui's fold and
/// close.
///
/// egui draws its own title bar and offers no hook to add to it, so this is an
/// `Area` laid over the title bar. The overlay is the point: a control in the
/// body **reserves** a row and pushes the content down (the operator: *"the
/// button pushes the content down so a scrollbar occurs"*), while this one
/// costs the body nothing. Returns whether it was pressed.
#[cfg(not(target_arch = "wasm32"))]
fn tool_window_title_button(
    ctx: &egui::Context,
    win_id: egui::Id,
    win_rect: egui::Rect,
    body_top: f32,
) -> bool {
    if !body_top.is_finite() {
        return false;
    }
    // The title bar is the strip from the window's top down to where the body
    // begins (its fold ▾ left, its close ✕ right).
    let title_h = (body_top - win_rect.top()).clamp(18.0, 40.0);
    let side = title_h - 6.0;
    // egui's close button owns the title bar's right edge; put ours just inside
    // it, so the two never overlap.
    let rect = egui::Rect::from_min_size(
        egui::pos2(win_rect.right() - 2.0 * side - 10.0, win_rect.top() + (title_h - side) / 2.0),
        egui::vec2(side, side),
    );
    let mut clicked = false;
    // **`Order::Middle`, not `Foreground`.** This is the whole of upstream #653's
    // shape and of the operator's screenshot: a `Foreground` area paints above
    // *every* window whatever the draw order, so with two tool windows
    // overlapping, the lower one's chip showed through the upper one's body
    // while the rest of its title bar was correctly hidden. The window above was
    // hidden in window order and the chip was not — the same window, half
    // hidden, which no other explanation fits.
    //
    // `Middle` is the order egui's own windows live in, so the chip now takes
    // its place among them: it sits above the body it belongs to, and a window
    // raised over that one covers the chip with everything else. It is still an
    // `Area` rather than a widget in the body's `Ui`, because the title bar is
    // outside the body's clip rect and a widget there would be clipped away —
    // which is why this was an overlay to begin with.
    egui::Area::new(win_id.with("title-btn")).fixed_pos(rect.min).order(egui::Order::Middle).show(
        ctx,
        |ui| {
            ui.set_min_size(rect.size());
            clicked = crate::chrome::chip_window_icon(ui, crate::chrome::WindowIcon::Out)
                .on_hover_text(tr!("frame-window-chip-hint"))
                .clicked();
        },
    );
    clicked
}

/// See [`SdroxideApp::tool_window_egui`]: the id epoch that resets a tool
/// window's remembered geometry when this window's bounding changes. Bump it
/// whenever a tool window's default size or its body's height-filling changes.
const TOOL_WINDOW_EPOCH: &str = "b3";

/// The DETACH/DOCK chip a tool window wears at the top-right of its body. On the
/// browser it is absent — there is no second window to move to.
#[cfg(not(target_arch = "wasm32"))]
fn tool_window_chip(ui: &mut egui::Ui, undocked: bool) -> bool {
    {
        // A glyph, not a word. The operator's note: *"the huge buttons for
        // window and dock … there already is a lot of text in the screen, so if
        // we can strip that down a bit with symbols and a hover that would be
        // bliss."* The arrow points the way the click goes — right and out to a
        // window of its own, left and back into the main one — so the direction
        // reads without the hover, and the hover still names it in words.
        let (icon, hover) = if undocked {
            // A **floating panel** docks back into the pane; an OS window docks
            // back into the main window. Two different pictures, because they are
            // two different places to go back to.
            (
                crate::chrome::WindowIcon::Dock,
                tr!("frame-dock-panel-pane-hint"),
            )
        } else {
            (
                crate::chrome::WindowIcon::Out,
                tr!("frame-window-chip-hint"),
            )
        };
        // A **one-row** allocation, right-aligned — not `row_tail`. `row_tail`
        // lays out `right_to_left(Align::Center)` over the whole available
        // height, so once a window's area has settled it centres the chip and
        // advances the cursor to the bottom, pushing every tool's body off the
        // screen. The operator's trace was the proof: `after chip cursor_y=530`
        // in a window whose body ends at 525. A row of the chip's own height is
        // one the layout cannot fill vertically.
        let size = egui::vec2(ui.available_width(), crate::chrome::chip_height(ui, None));
        let mut clicked = false;
        ui.allocate_ui_with_layout(size, egui::Layout::right_to_left(egui::Align::Center), |ui| {
            clicked = crate::chrome::chip_window_icon(ui, icon).on_hover_text(hover).clicked();
        });
        clicked
    }
}

/// Everything about a module's **own OS window** that is not its contents: the
/// ids, the app-id a Wayland rule matches on, the title and the opening size.
/// One per [`DetachableModule`], from [`detached_spec`].
#[cfg(not(target_arch = "wasm32"))]
struct DetachedWindowSpec {
    /// Stable viewport id — **one window per module for the whole station**,
    /// not per radio. `UiSettings` is station-wide, so there is a single window
    /// and it shows the focused radio's; a stable id across radio and pane
    /// switches lets the same OS window be reused rather than torn down and
    /// remapped, which a tiling compositor treats as a *new* window sized by it
    /// rather than by the operator.
    viewport_id: egui::ViewportId,
    /// egui memory key for the last-seen geometry, used to persist once a drag
    /// settles instead of once per frame.
    settle_id: egui::Id,
    /// The Wayland/Windows app-id a compositor rule matches on to float and
    /// place the window. Owned, because a tool window's id is built at runtime
    /// (`sdroxide-tool-<id>`).
    app_id: String,
    title: String,
    default_size: [f32; 2],
    min_size: [f32; 2],
}

/// The window spec for a module. The `app_id`s are a public contract — the
/// operator's own compositor rules match on them (`docs/internal/UNDOCKED-HANDOVER.md`), so
/// changing one is a user-visible change.
#[cfg(not(target_arch = "wasm32"))]
fn detached_spec(module: sdroxide_types::DetachableModule) -> DetachedWindowSpec {
    use sdroxide_types::DetachableModule as M;
    let app_id = module.app_id();
    let (name, default_size) = match module {
        M::Panadapter => (tr!("frame-module-panadapter"), [960.0, 540.0]),
        M::Panel => (tr!("frame-module-panel"), [520.0, 700.0]),
        M::Controls => (tr!("frame-module-controls"), [1280.0, 640.0]),
        M::AuxPanadapter => (tr!("frame-module-aux-panadapter"), [960.0, 540.0]),
        // Wide enough for the mode sections in five columns — the whole reason
        // it is a window and not the docked column, which cannot be this wide
        // without squeezing the operating panel below it.
        M::BandMenu => (tr!("frame-module-band-mode"), [560.0, 760.0]),
        // A compass wants to be near square: it is a dial with a readout
        // underneath, and a wide window would just be dial with margins.
        M::Rotor => (tr!("frame-module-rotator"), [420.0, 520.0]),
    };
    DetachedWindowSpec {
        viewport_id: egui::ViewportId::from_hash_of(app_id),
        settle_id: egui::Id::new(format!("{app_id}-geom")),
        app_id: app_id.to_string(),
        title: tr!("frame-detached-title", flavor = sdroxide_version::FLAVOR, name = name),
        default_size,
        min_size: [360.0, 240.0],
    }
}

/// The window spec for a **tool window** (the scanner, the schedule, the
/// logbook …) undocked into its own OS window. Distinct from the module specs
/// only in the id grammar — `sdroxide-tool-<id>` — so an operator's rule can
/// target them as a group or one by one.
#[cfg(not(target_arch = "wasm32"))]
fn tool_spec(id: &str, title: &str, default_size: [f32; 2]) -> DetachedWindowSpec {
    let app_id = format!("sdroxide-tool-{id}");
    DetachedWindowSpec {
        viewport_id: egui::ViewportId::from_hash_of(&app_id),
        settle_id: egui::Id::new(format!("{app_id}-geom")),
        app_id,
        title: tr!("frame-detached-title", flavor = sdroxide_version::FLAVOR, name = title),
        default_size,
        min_size: [240.0, 160.0],
    }
}

/// The size and position a (re)built detached window opens at: the operator's
/// last geometry if there is one, otherwise the module's default, either way
/// shrunk to fit the monitor it is about to open on. Never grows a window.
///
/// The position is not clamped (a clamp to the monitor's *size* pulled a window
/// left on a second screen back onto the first — the same reasoning as the 3D
/// window's geometry), and is `None` on Wayland, which gives a client no
/// absolute position.
#[cfg(not(target_arch = "wasm32"))]
fn detached_geometry(
    monitor: Option<egui::Vec2>,
    seed: Option<sdroxide_types::DetachedWindow>,
    default_size: [f32; 2],
) -> (egui::Vec2, Option<egui::Pos2>) {
    let want = seed.map_or(egui::Vec2::from(default_size), |s| egui::Vec2::from(s.size));
    let Some(monitor) = monitor.filter(|m| m.x > 1.0 && m.y > 1.0) else {
        return (want, seed.and_then(|s| s.pos).map(egui::Pos2::from));
    };
    let chrome = egui::vec2(16.0, 40.0);
    let size =
        crate::layout::fit_inner_size(monitor, want + chrome, want, egui::vec2(360.0, 240.0))
            .unwrap_or(want);
    // **The saved position, brought back onto the screen.** The size was always
    // fitted to the monitor and the position never was, so a window saved on a
    // display that is no longer there — a laptop undocked from a second screen, a
    // resolution change, a DPI change — opened exactly where nothing could be
    // reached: the operator presses the chip, nothing appears, and the window is
    // live somewhere off the edge. (Reported as "I cannot access the settings at
    // all" on a Brick2, 2.1.0.2.4.)
    //
    // Only the right and bottom edges are pulled in. A negative x is a monitor to
    // the left of the primary, and a negative y one above it; both are real places
    // and neither is a lost window.
    let pos = seed.and_then(|s| s.pos).map(|[x, y]| {
        let max_x = (monitor.x - 80.0).max(0.0);
        let max_y = (monitor.y - 60.0).max(0.0);
        egui::pos2(x.min(max_x), y.min(max_y))
    });
    (size, pos)
}

/// Which saved workspace a `Ctrl+Alt+<digit>` calls back, or `None`.
///
/// The number row reads 1…9 and then 0 for the tenth, which is how SDRuno's
/// workspaces are numbered and how a keyboard operator counts them. Pure, so
/// the mapping is a thing a test can hold rather than something only a live
/// keyboard shows.
#[cfg(not(target_arch = "wasm32"))]
fn workspace_slot(key: eframe::egui::Key) -> Option<usize> {
    use eframe::egui::Key;
    Some(match key {
        Key::Num1 => 0,
        Key::Num2 => 1,
        Key::Num3 => 2,
        Key::Num4 => 3,
        Key::Num5 => 4,
        Key::Num6 => 5,
        Key::Num7 => 6,
        Key::Num8 => 7,
        Key::Num9 => 8,
        Key::Num0 => 9,
        _ => return None,
    })
}

/// What a detached window left behind this frame.
#[cfg(not(target_arch = "wasm32"))]
struct DetachedOutcome {
    /// The operator closed the window — which means "dock me again", not "come
    /// back next frame".
    close_requested: bool,
    /// Where the window ended up, for the settle-persist in
    /// [`SdroxideApp::handle_detached_outcome`].
    geometry: Option<sdroxide_types::DetachedWindow>,
}

/// Everything a **child window** needs to answer the layout questions for
/// itself instead of for the main window — see
/// [`crate::layout::ViewportScope`].
///
/// Three facts and no defaults: whose radio it is, which layout the operator
/// asked for, and whether it is a window in its own right or a desk tool that
/// stays desktop-shaped however tall it is made.
#[derive(Clone, Copy)]
struct ChildWindow {
    /// The radio whose ids this window mints. The shell emits every detached
    /// window for the **focused** radio after it has drawn whichever tabs are
    /// on screen, so the salt left published on the context belongs to the last
    /// tab drawn — another radio, in a split.
    salt: u32,
    /// The operator's layout override (Auto, or a tier said out loud), which
    /// applies to every window.
    mode: sdroxide_types::LayoutMode,
    shape: crate::layout::WindowShape,
}

impl ChildWindow {
    /// A window whose own width earns its tier — a spectrum, an operating
    /// panel, a tool.
    fn own(app: &SdroxideApp) -> Self {
        Self {
            salt: app.radio_id,
            mode: app.ui_settings.layout,
            shape: crate::layout::WindowShape::Own,
        }
    }

    /// A desk tool, desktop-shaped whatever its height — the RX control
    /// console, the band selector, the rotator.
    fn desk(app: &SdroxideApp) -> Self {
        Self {
            salt: app.radio_id,
            mode: app.ui_settings.layout,
            shape: crate::layout::WindowShape::Desk,
        }
    }

    /// An **operating panel**: one pane at a time when the window is too narrow
    /// for two, with the window drawing the pane chips itself.
    fn panes(app: &SdroxideApp) -> Self {
        Self {
            salt: app.radio_id,
            mode: app.ui_settings.layout,
            shape: crate::layout::WindowShape::Panes,
        }
    }

    /// Publish this window's answers for as long as the guard lives.
    fn scope(self, ui: &egui::Ui) -> crate::layout::ViewportScope {
        crate::layout::ViewportScope::enter(
            ui.ctx(),
            ui.max_rect().size(),
            self.mode,
            self.shape,
            self.salt,
        )
    }
}

/// Show a module's window and draw `draw` into it, returning how it closed.
///
/// The one place the multi-window plumbing lives, so a new detachable module
/// writes its contents and nothing else. `show_viewport_immediate` rather than
/// the deferred form the 3D window uses: a module draws from `&mut self`'s live
/// state and a click, zoom or drag has to flow straight back, so the published
/// snapshot the deferred closure's `Send + Sync + 'static` bound would force is
/// exactly what must not happen. Immediate runs in this same frame and may
/// borrow `self`, so `draw` captures it freely.
///
/// Where the toolkit only offers *embedded* viewports (the headless test
/// harness, and any platform without native multi-viewport support), a window
/// inside the main one is not the feature, so this draws nothing and reports
/// nothing; the caller has already given the space away, so the main window
/// still lays out. eframe sets `embed_viewports` false on every supported
/// native platform; the browser never reaches here.
///
/// While the draw runs, the window's **own** layout answers are published in
/// place of the main window's ([`ChildWindow`], and
/// [`crate::layout::ViewportScope`] behind it): a window is measured by its own
/// width, it has no band column to dock, and its ids belong to the radio it
/// shows. The guard is made inside the viewport and dropped at the end of the
/// draw, so nothing of it outlives a frame.
#[cfg(not(target_arch = "wasm32"))]
fn detached_viewport(
    ctx: &egui::Context,
    spec: &DetachedWindowSpec,
    seed: Option<sdroxide_types::DetachedWindow>,
    child: ChildWindow,
    draw: impl FnOnce(&mut egui::Ui),
) -> DetachedOutcome {
    if ctx.embed_viewports() {
        return DetachedOutcome { close_requested: false, geometry: None };
    }
    let vid = spec.viewport_id;
    let mut builder = egui::ViewportBuilder::default()
        // The app-id is not decoration: it is the one handle a Wayland window
        // rule has to float this window and pin it to a monitor.
        .with_app_id(spec.app_id.clone())
        .with_title(spec.title.clone())
        .with_min_inner_size(spec.min_size);
    // Seed the size and place only on the frame the window is (re)built, so a
    // live resize is never fought by our own request.
    if ctx.cumulative_pass_nr_for(vid) == 0 {
        let monitor = ctx.input(|i| i.viewport().monitor_size);
        let (size, pos) = detached_geometry(monitor, seed, spec.default_size);
        builder = builder.with_inner_size(size);
        if let Some(pos) = pos {
            builder = builder.with_position(pos);
        }
    }
    let mut close_requested = false;
    let mut geometry: Option<sdroxide_types::DetachedWindow> = None;
    // `show_viewport_immediate` wants an `FnMut`; the draw is a one-shot, so it
    // rides an `Option` and is taken on the frame it is drawn (and dropped
    // un-called on a frame the window is only closing).
    let mut draw = Some((child, draw));
    ctx.show_viewport_immediate(vid, builder, |ui, _class| {
        let ictx = ui.ctx();
        // `content_rect` is the inner size as the toolkit reports it — the only
        // size on Wayland, where `viewport().inner_rect` is `None` because
        // Wayland gives a client no absolute position. `outer_rect` carries that
        // position where the platform has one, and is `None` on Wayland too.
        let size = ictx.content_rect().size();
        if size.x > 1.0 && size.y > 1.0 {
            let pos = ictx.input(|i| i.viewport().outer_rect).map(|r| r.min);
            geometry = Some(sdroxide_types::DetachedWindow {
                // Whole points, so a sub-point wobble is not a resize.
                size: [size.x.round(), size.y.round()],
                pos: pos.map(|p| [p.x.round(), p.y.round()]),
            });
        }
        if ictx.input(|i| i.viewport().close_requested()) {
            close_requested = true;
            // Wake the root pass so the main window reclaims the space promptly
            // rather than on the next unrelated repaint.
            ictx.request_repaint_of(egui::ViewportId::ROOT);
            return;
        }
        if let Some((child, draw)) = draw.take() {
            // **This window's** answers, not the main window's, for as long as
            // the draw runs: its own width earns its tier, it has no band column
            // to dock, and it mints its ids for the radio it belongs to. The
            // guard puts every one of them back when it drops, so whatever the
            // next window (or the main window) draws gets its own again.
            let _scope = child.scope(ui);
            draw(ui);
        }
    });
    DetachedOutcome { close_requested, geometry }
}

/// Everything the panadapter draw needs that is derived from the radio's state
/// and the current mode, gathered by [`SdroxideApp::panadapter_inputs`].
///
/// The point is that [`SdroxideApp::draw_panadapter`] becomes self-contained:
/// it is then callable from the main window *or* a detached window without the
/// frame loop's branch having to compute a dozen locals first. The one thing
/// that must still run before the draw — the sub-band view-fitting that sets
/// `view.view_lo_hz` — stays in the frame loop, because it is state the draw
/// reads rather than an argument to it.
struct PanadapterInputs {
    cursor: Option<spectrum_view::AudioCursor>,
    dxped: sdroxide_types::DxpedMode,
    auto_tx_freq: bool,
    hold_tx_freq: bool,
    markers: Vec<f32>,
    skimmer: Vec<sdroxide_types::SkimmerSpot>,
    alpha: Vec<f32>,
    net_spots: Vec<Spot>,
    net_alpha: Vec<f32>,
    ism: Vec<spectrum_view::IsmLabel>,
    mem: Vec<crate::widgets::memories::MemMark>,
}

impl eframe::App for SdroxideApp {
    /// The anti-touch lock works on the input itself, before any widget reads
    /// it — see [`crate::touch_lock`].
    fn raw_input_hook(&mut self, ctx: &egui::Context, raw_input: &mut egui::RawInput) {
        crate::touch_lock::filter(ctx, raw_input);
    }

    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        let now = ctx.input(|i| i.time);
        self.konami.tick(ctx.input(|i| i.stable_dt).min(0.25));
        // The profile's stored key bindings, offered when this client has not
        // adopted them. Drawn before anything else so it is the first thing
        // seen, and from `ctx` rather than `ui` because it is a modal — it has
        // to be able to sit over the whole window, not inside one panel.
        self.bindings_offer_ui(&ctx);
        // The rate every animation in the tree paces itself to. Published here,
        // before anything draws, so a change in Settings → UI reaches the
        // needle and the waterfall on the same frame it reaches the scheduler.
        crate::repaint::set_frame_period_ms(1000 / u64::from(self.ui_settings.fps().max(1)));
        // Which radio the ids minted below belong to. Split view draws several
        // apps per frame; each declares itself before it draws anything.
        crate::layout::set_radio_salt(&ctx, self.radio_id);
        // The aircraft views write their numbers in this set; read once here so
        // a formatter several calls down needs no argument.
        self.ui_settings.air_units.set_current();
        // The screen wake lock is re-asked for every frame it is wanted and
        // missing: the browser takes it back whenever the page is hidden.
        crate::wake_lock::keep_awake(self.ui_settings.keep_screen_on, now);
        crate::sound_welcome::sync(self.ui_settings.sound_welcome);
        // Settle the layout for this frame before anything draws. It is a
        // property of the viewport plus the operator's override, and the
        // override lives in settings the context cannot see — so it is decided
        // once here and read from the context by everything downstream. Sized
        // from the `Ui` rather than the viewport: in a split view this app
        // owns one column of the window, and it is the column that must decide
        // whether the roomy layout still fits. For the whole-window case the
        // two are the same rect.
        let tier = crate::layout::tier_for(ui.max_rect().size(), self.ui_settings.layout);
        crate::layout::set_tier(&ctx, tier);
        // Whether the band selector can dock beside the waterfall, from the
        // same column: the top bar's chip reads it before the column is drawn.
        crate::layout::set_band_dock_room(
            &ctx,
            super::top_bar::band_dock_room(tier, ui.max_rect().width()),
        );
        // …and the two questions the tier does not answer, published the same
        // way and for the same reason — the operator's override lives in
        // settings the context cannot see. `Small screen` asked for on a tall
        // display would otherwise be measured back to "roomy" at the places
        // that draw (issue #211).
        crate::layout::set_tight(
            &ctx,
            crate::layout::tight_for(ui.max_rect().size(), self.ui_settings.layout),
        );
        // Publish the SWL flag so `tx_gated` can hide controls without
        // threading the setting through every panel. The band/mode tab is
        // remembered first: it is one of the four things that flag is made of,
        // and the selector may have been drawn in any of three places since the
        // last frame.
        self.remember_band_tab();
        super::set_swl_active(self.swl_mode());
        crate::layout::set_short_tablet(
            &ctx,
            crate::layout::short_tablet_for(ui.max_rect().size(), self.ui_settings.layout),
        );
        if self.tier != tier {
            self.tier = tier;
            crate::theme::apply_metrics(&ctx, tier);
            // The `Ui` we were handed took its copy of the style before this,
            // so the new metrics land next frame; ask for one straight away.
            ctx.request_repaint();
        }
        // Same idea for the theme and chrome styles: the settings dialog wrote
        // them into `ui_settings` at the end of last frame, and the top of the
        // frame is the one place a visuals rewrite cannot race the settings
        // window's ScrollPalette push/pop.
        let look =
            (self.ui_settings.theme, self.ui_settings.button_style, self.ui_settings.window_style);
        if self.applied_look != look {
            self.applied_look = look;
            crate::theme::set_look(look.0, look.1, look.2);
            crate::theme::apply_visuals(&ctx);
            ctx.request_repaint();
        }
        if self.applied_language != self.ui_settings.language {
            self.applied_language = self.ui_settings.language;
            crate::i18n::set_language(self.applied_language);
            ctx.request_repaint();
        }
        // The panadapter and skimmer font sizes need no visuals rewrite —
        // everything they size is laid out per frame — so they are simply
        // re-stored each frame.
        crate::theme::set_font_sizes(
            self.ui_settings.skimmer_font_size,
            self.ui_settings.waterfall_font_size,
            self.ui_settings.menu_font_size,
        );
        // Same for the spot tints and the band-plan shades: the pickers in the
        // settings window write straight into `ui_settings`, and a colour has
        // to take on the frame it is chosen in or the picker feels dead.
        crate::theme::set_spot_colors(&self.ui_settings.spot_colors);
        crate::theme::set_programs_colours(
            self.ui_settings.programs_colour_all,
            &self.ui_settings.programs_colours,
        );
        crate::theme::set_bandplan_colors(&self.ui_settings.bandplan_colors);
        // …and whether the maps carry cities, which the four of them read
        // through one shared painter.
        crate::theme::set_map_cities(self.ui_settings.map_cities);
        // The interface scale is egui's zoom factor, which egui also lets the
        // operator drive with ctrl+plus / ctrl+minus, so it is written only
        // when the setting itself moves — see `theme::apply_zoom`. It reads
        // the size the call above just stored, so it has to stay below it.
        if self.applied_ui_font != self.ui_settings.menu_font_size {
            self.applied_ui_font = self.ui_settings.menu_font_size;
            crate::theme::apply_zoom(&ctx);
            ctx.request_repaint();
        } else {
            self.remember_ui_zoom(&ctx);
        }
        self.drain_events(&ctx, now);
        // The update check answers on its own thread; pick it up cheaply.
        #[cfg(not(target_arch = "wasm32"))]
        self.update.poll();
        self.poll_adif_import();
        self.poll_settings_import();
        self.refresh_band_conditions(now);

        // A server that asks for a password gets the whole window until it has
        // one. Nothing below this point has anything to draw — no capabilities,
        // no state and no spectrum arrive until the sign-in is accepted — and
        // nothing below it may run either: the bindings would happily send PTT
        // to a socket that is not going to read it.
        let phase = self.ctrl.auth_phase();
        // Keyed by station, not by radio: a station challenges each of its
        // radios separately, and the operator signs in to the station.
        self.login.settle(&phase, &self.station_key());
        if phase.is_pending() {
            let rs = frame.wgpu_render_state();
            if let Some(login) = crate::login::screen(ui, &mut self.login, &phase, rs) {
                self.ctrl.send_auth(login.username, login.password);
            }
            // The socket wakes the UI when the server answers; this is only so
            // a spinner-less "CHECKING…" cannot look like a hung window.
            crate::repaint::schedule_ms(&ctx, IDLE_POLL_MS);
            crate::tooltip_copy::tick(&ctx);
            return;
        }

        let mut cmds = Vec::new();
        // Auto mode, on its own clock: an unattended run must not depend on
        // which pane or tab happens to be on screen.
        self.tick_auto_mode(&ctx, now, &mut cmds);
        // A DAB channel scan, app-wide too: it must run whichever pane is on
        // screen, and it is what fills the channel picker with the blocks that
        // actually carry a transmission here rather than a hardcoded list.
        self.tick_dab_scan(&ctx, now, &mut cmds);
        // A channel list chosen in the memories window: parsed here and sent
        // to the engine, which owns the list and the numbering in it.
        self.poll_chirp_import(&mut cmds);
        // A band plan the operator picked on the General tab: written and then
        // reloaded, so the station runs on it without a restart.
        self.poll_band_plan_import(&mut cmds);
        // A "stop after" deadline armed in the REC popup: stop the MP3
        // recording once it passes (issue #520).
        self.poll_recording_timer(&mut cmds);
        // A silence auto-split armed in the REC popup: start and stop the MP3
        // recording with the receiver's squelch (issue #546).
        self.poll_recording_gate(&ctx);
        // The gate decides once a frame, so while it is armed keep frames
        // coming even when nothing else is animating — otherwise an unattended
        // monitor on an idle screen would never notice a transmission. While a
        // file is actually being written the REC chip breathes, so it wants a
        // smoother clock than the gate's own once-a-frame decision does.
        if self.rec_gate_s.is_some() {
            let ms = if self.state.recording || self.state.iq_recording { 120 } else { 250 };
            crate::repaint::after_ms(&ctx, ms);
        }
        // The keyboard, the mouse buttons and the control surface belong to
        // the focused radio alone. In a split view every visible radio runs
        // this frame loop, and without the gate one arrow key would tune all
        // of them at once.
        if self.focused {
            // F1 toggles the manual — handled here (not in
            // `keyboard_shortcuts`) so it works even while a text field has
            // focus.
            if ctx.input(|i| i.key_pressed(egui::Key::F1)) {
                self.help.open = !self.help.open;
            }
            // **The two window keys**, and the reason there are two rather than
            // one per module: there are six modules and nobody remembers six.
            // `Ctrl+Alt+W` opens the module-windows list — the only route that
            // works with every window docked, which is what makes a window lost
            // on another workspace recoverable on a keyboard-only desktop — and
            // `Ctrl+Alt+F` floats the operating panel or docks it again. Both
            // handled here so they work while a text field has focus, as F1
            // does: a chord with Ctrl and Alt in it is not typing.
            #[cfg(not(target_arch = "wasm32"))]
            {
                // Read from the **events**, not from the frame's modifier state:
                // a chord pressed and released inside one frame still arrives as
                // two events, and the frame's `modifiers` are whatever the
                // pointer last said.
                let (mut list, mut float) = (false, false);
                let mut workspace: Option<usize> = None;
                ctx.input(|i| {
                    for event in &i.events {
                        if let egui::Event::Key {
                            key, pressed: true, modifiers, repeat: false, ..
                        } = event
                            && modifiers.ctrl
                            && modifiers.alt
                            && !modifiers.shift
                        {
                            match key {
                                egui::Key::W => list = true,
                                egui::Key::F => float = true,
                                other => {
                                    if let Some(slot) = workspace_slot(*other) {
                                        workspace = Some(slot);
                                    }
                                }
                            }
                        }
                    }
                });
                if list {
                    self.show_windows = !self.show_windows;
                }
                if float {
                    use sdroxide_types::DetachableModule;
                    let floating = matches!(
                        self.module_window_state(DetachableModule::Panel),
                        super::ModuleWindow::Floating
                    );
                    let next = if floating {
                        super::ModuleWindow::Docked
                    } else {
                        super::ModuleWindow::Floating
                    };
                    self.set_module_window_state(DetachableModule::Panel, next);
                }
                if let Some(i) = workspace {
                    // **A workspace on a key**, `Ctrl+Alt+1…9` and `0` for the
                    // tenth — the operator's own words: *"you spend time setting
                    // up your preferred view, save it, and call it back any
                    // time"*. An empty slot says so instead of doing nothing:
                    // a key that silently ignores a press is the one thing a
                    // control must never do.
                    match self.workspaces.get(i).cloned() {
                        Some(ws) => {
                            ws.apply(&mut self.ui_settings);
                            // A workspace stores the module windows, not the
                            // floating panel — `Workspace` is the furniture —
                            // so the panel comes back to the pane with it
                            // rather than floating over the arrangement that
                            // asked for it docked.
                            self.panel_floating = false;
                            crate::app::persist::persist_ui_settings(&self.ui_settings);
                        }
                        None => self.show_notice(tr!("frame-workspace-missing", n = i + 1)),
                    }
                }
            }
            // Nothing to see here.
            let armed = super::konami::armed(self.state.band);
            let konami = &mut self.konami;
            ctx.input(|i| konami.feed(&i.events, armed));
            // An open manual takes the scrolling keys before the bindings run,
            // so reading it never tunes the radio at the same time.
            self.help.grab_keys(&ctx);
            // Likewise the CW straight key takes the Space bar, so a PTT bound
            // to it does not key the rig under the operator's hand.
            self.swallow_straight_key(&ctx);
            self.control_inputs(&ctx, now, &mut cmds);
        }
        // (An unfocused pane's MIDI backlog is discarded in `drain_events`,
        // which ran above, same as for the hidden tabs.)
        // Shutting down with a bound key, a footswitch or the compact layout's
        // PTT still held would otherwise leave the rig transmitting.
        if ctx.input(|i| i.viewport().close_requested()) {
            if self.input.any_held() {
                self.release_held_controls(&mut cmds);
            }
            // Latched as well as held: the chip keeps the transmitter on
            // after the pointer has lifted, and a window closing on a latch is
            // exactly the over nobody is watching.
            if self.ptt.keying() {
                self.ptt = Default::default();
                cmds.push(Command::SetPtt(false));
            }
        }

        // **Classic UI**: every control in a column down the left, SDR++-style.
        //
        // Drawn **before the strip**, and that order is the point: a panel takes
        // its space off what is left, so drawing the column first leaves the top
        // panel only the picture's width — which is what lets the frequency
        // readout be centred *over the waterfall* rather than over the window.
        // Drawn after it (as this was), the strip spans the whole window and the
        // readout sits above the column it has nothing to do with. The operator
        // asked for this twice.
        //
        // The column also takes the window's full height, which is what a column
        // of controls wants: its heading is level with the bar beside it.
        //
        // Left out entirely where there is no room, and the strip behaves as it
        // always has: the two arrangements are the same program.
        //
        // **One decision, published.** Whether the column is here is decided
        // once, from the window's whole width, and put on the context for the
        // strip to read — because the strip used to re-derive it from the width
        // **left after the column took its share**, which came out "no room for
        // a column" in a window that had just drawn one. The strip then drew
        // every control again under the frequency, the fault in the operator's
        // screenshot of Classic with the panadapter out.
        #[cfg(not(target_arch = "wasm32"))]
        let classic_here = self.ui_settings.classic_ui
            && tier != crate::layout::Tier::Phone
            && super::classic::classic_surface(tier, ui.available_width(), self.classic_collapsed)
                != super::classic::ClassicSurface::None;
        #[cfg(target_arch = "wasm32")]
        let classic_here = false;
        crate::layout::set_classic_column(&ctx, classic_here);
        #[cfg(not(target_arch = "wasm32"))]
        if self.ui_settings.classic_ui
            && tier != crate::layout::Tier::Phone
            && let super::classic::ClassicSurface::Column(min, max) =
                super::classic::classic_surface(tier, ui.available_width(), self.classic_collapsed)
        {
            // A **new id**, so the one-time change of starting width is actually
            // seen: the panel's width is remembered by egui (in its persisted
            // memory), so 300 would be hidden behind whatever an earlier drag
            // left there. The section order and every fold's open-or-closed
            // state are keyed separately and are untouched by this.
            egui::Panel::left(crate::layout::salted_id(&ctx, "classic-column-panel-2"))
                .resizable(true)
                .default_size(super::classic::CLASSIC_DEFAULT_W.clamp(min, max))
                // Re-applied every frame, as the band dock's range is: a column
                // dragged wide on a big window gives the width back when the
                // window narrows.
                .size_range(min..=max)
                // A small margin and no fill of its own: the column draws
                // itself with `chrome::angled_frame`, which is what gives the
                // waterfall and the operating panel their cut-corner outline.
                // The panel is the space; the frame is the panel's look.
                .frame(egui::Frame::new().inner_margin(egui::Margin::symmetric(4, 4)))
                .show(ui, |ui| {
                    // **What makes the handle work.** egui's own note: a
                    // resizable panel needs its ui to *use* the space it was
                    // given, or the content's own minimum becomes the panel's
                    // width and the handle stops moving — which is exactly what
                    // the first version did, with a drag that looked live and
                    // did nothing.
                    ui.take_available_space();
                    self.classic_column(ui, &mut cmds);
                });
        }
        // **The folded column**, which is a rail and not nothing: the chip that
        // opens it again has to be on screen, and the operator's own idea — leave
        // it a few points wide — is what keeps the fold and the unfold in the
        // same place. Not resizable: a rail has one width.
        #[cfg(not(target_arch = "wasm32"))]
        if self.ui_settings.classic_ui
            && tier != crate::layout::Tier::Phone
            && self.classic_collapsed
        {
            // Its **own id**, not the column's: a panel stores its width, and a
            // rail that wrote 22 points into the column's state would bring the
            // column back 22 points wide.
            egui::Panel::left(crate::layout::salted_id(&ctx, "classic-rail-panel"))
                .resizable(false)
                .exact_size(super::classic::CLASSIC_RAIL_W)
                // Outlined on its right edge like the panels beside it, so the
                // rail reads as the column folded rather than as a strip of the
                // window left over.
                .frame(egui::Frame::new().fill(crate::theme::PANEL()).inner_margin(egui::Margin {
                    left: 4,
                    right: 0,
                    top: 4,
                    bottom: 4,
                }))
                .show(ui, |ui| {
                    self.classic_rail(ui, &mut cmds);
                    let r = ui.max_rect();
                    ui.painter().line_segment(
                        [
                            r.right_top() + egui::vec2(3.0, 0.0),
                            r.right_bottom() + egui::vec2(3.0, 0.0),
                        ],
                        egui::Stroke::new(1.0, crate::theme::LINE()),
                    );
                });
        }
        // The strip is drawn here unless the operator has undocked it into its
        // own window; when they have, this window is the spectrum and the
        // decoders, and the strip lives in the control window (SDRuno's split
        // of MAIN SP from RX control).
        if !self.controls_detached() {
            // The band behind the strip is painted **here** rather than by the
            // panel's own frame fill, and that is the whole of item 1b from fork
            // discussion #16 — Kevin's "the boundary line is missing, the outline
            // extends beyond the edge of the display window".
            //
            // A `Panel`'s frame fills its rect *plus a couple of points past the
            // window's right edge*, whatever the content inside it does: measured
            // at 360 pt the frame's content sat at 8..352 — comfortably inside —
            // while its fill was painted 0..362. Nothing inside the panel was
            // too wide; the panel's background was simply drawn off-screen, which
            // is the same shape of fault as a strip row that will not fit, and it
            // costs the same: the right border lands somewhere the window cannot
            // show it.
            //
            // So the frame carries no fill and the band is drawn from inside it,
            // through a painter clipped to the window. Same ink, same place, and
            // a clip rect that cannot be argued with.
            let band = egui::Margin::symmetric(8, 6);
            // A slot in the paint list, reserved before the strip is drawn and
            // filled in afterwards — the same trick `angled_frame` uses for the
            // gradient fill, and for the same reason: the band's height is only
            // known once the panel has been laid out, and a painter's order is
            // the order the shapes were added.
            let mut band_slot = None;
            let band_panel = egui::Panel::top(crate::layout::salted_id(&ctx, "topbar"))
                .frame(egui::Frame::new().inner_margin(band))
                .show(ui, |ui| {
                    band_slot = Some(ui.painter().add(egui::Shape::Noop));
                    crate::chrome::angled_frame(ui, crate::theme::PINK(), |ui| {
                        self.top_bar(ui, &mut cmds);
                    });
                });
            self.top_bar_bottom = band_panel.response.rect.bottom();
            if let Some(slot) = band_slot {
                let window = ctx.content_rect();
                ui.painter().set(
                    slot,
                    egui::Shape::rect_filled(
                        band_panel.response.rect.intersect(window),
                        egui::CornerRadius::ZERO,
                        crate::theme::BG_DEEP(),
                    ),
                );
            }
        }
        // A persistent radio-audio warning (input unavailable / mono-for-IQ)
        // rides above the panadapter with a dismiss button, so a silent RX
        // failure is explained rather than reading as "waiting for spectrum".
        //
        // A receive-only radio also gets a nudge here — most public SDRs are
        // somebody else's antenna and have no transmitter at all, and the full
        // transmit UI is clutter for them. The offer is the per-radio listening
        // screen (`hide_tx`), which is exactly "hide the transmit controls";
        // like the SWR latch, an operator who has said no is not asked again.
        //
        // Not during a playback: the radio standing in then is a file, and has
        // no transmitter by design rather than by nature.
        self.tx_band_guard();
        let rx_only = !self.swl_mode()
            && self.state.playback.is_none()
            && self.caps.as_ref().is_some_and(|c| !c.is_transmit_capable());
        // A recording playing back says so above the waterfall, so it is never
        // taken for the live band.
        self.playback_strip(ui, &mut cmds);
        // Say, once, that an undocked operating panel has nowhere to show in
        // this mode — otherwise it silently does not appear when the operator
        // switches to it. Raised once per mode, so dismissing it sticks until
        // the mode changes; cleared when a mode with a panel comes back.
        #[cfg(not(target_arch = "wasm32"))]
        {
            let mode = self.state.rx[0].mode;
            // Both of the modes with a detachable own panel are named: DSC's and
            // NAVTEX's, each raised by its own window. The generic Panel module
            // covers every other one.
            let unavailable =
                (self.ui_settings.is_detached(sdroxide_types::DetachableModule::Panel))
                    && !(mode.has_bottom_panel() || mode == Mode::Cw);
            if unavailable {
                if self.panel_undock_notice_mode != Some(mode) {
                    self.panel_undock_notice_mode = Some(mode);
                    self.radio_notice =
                        Some(tr!("frame-no-operating-panel", mode = mode.label()));
                }
            } else {
                self.panel_undock_notice_mode = None;
            }
        }
        // The engine's notice first, then another sdroxide on this machine.
        // The transmit band list: why transmit is off at the dial, then the standing
        // request to check and confirm the list (never dismissable — it stays
        // until the operator has confirmed).
        let tx_msg = self.radio_notice.is_none().then(|| self.tx_band_msg.clone()).flatten();
        let tx_note = (self.radio_notice.is_none() && tx_msg.is_none())
            .then(|| self.tx_bands_notice())
            .flatten();
        let shown = self
            .radio_notice
            .clone()
            .or_else(|| tx_msg.clone())
            .or_else(|| tx_note.clone())
            .or_else(|| self.other_instance.clone());
        let notice = shown.or_else(|| {
            (rx_only && !self.rx_only_nudge_dismissed).then(|| {
                tr!("frame-rx-only-nudge")
            })
        });
        // A refused PTT or TUNE press: big, red, every time — no cap.
        if let Some(red) = self.tx_press_banner(crate::time::now_unix_f64()) {
            egui::Frame::new()
                .fill(crate::theme::ALERT())
                .inner_margin(egui::Margin::symmetric(10, 8))
                .show(ui, |ui| {
                    ui.label(
                        RichText::new(format!("⛔ {red}"))
                            .size(20.0)
                            .strong()
                            .color(Color32::WHITE),
                    );
                });
            ui.ctx().request_repaint_after(std::time::Duration::from_millis(500));
        }
        if let Some(notice) = notice {
            let (wash, rule, mark, ink) = notice_banner_colors();
            egui::Frame::new()
                .fill(wash)
                .stroke(egui::Stroke::new(1.0, rule))
                .inner_margin(egui::Margin::symmetric(8, 5))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("⚠").size(15.0).color(mark));
                        ui.label(RichText::new(notice).size(13.0).color(ink));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // ⛔ A TRIPPED SWR GUARD IS NOT DISMISSIBLE, and the
                            // distinction is the point. Plain "Dismiss" clears
                            // the banner locally and changes nothing in the
                            // engine, which for a latch would be the worst of
                            // both worlds: the warning gone and transmit still
                            // locked out, with nothing left on screen saying
                            // why. The latch therefore gets its own button that
                            // actually clears it in the engine.
                            if let Some(swr) = self.state.tx.swr_tripped {
                                if ui
                                    .button(RichText::new(tr!("frame-acknowledge")).size(13.0))
                                    .on_hover_text(tr!(
                                        "frame-re-enables-transmit-after-1",
                                        swr = format!("{:.1}", swr)
                                    ))
                                    .clicked()
                                {
                                    cmds.push(Command::ClearSwrTrip);
                                    self.radio_notice = None;
                                }
                            } else {
                                if tx_note.is_some() {
                                    if ui
                                        .button(RichText::new(tr!("tx-bands-check")).size(13.0))
                                        .clicked()
                                    {
                                        self.open_radio_settings();
                                    }
                                } else if ui.small_button(tr!("frame-dismiss")).clicked() {
                                    self.dismiss_notice();
                                    if rx_only {
                                        self.rx_only_nudge_dismissed = true;
                                    }
                                }
                                // Only for a radio that truly cannot transmit:
                                // the listening screen is the right shape for
                                // it, and nothing is retuned.
                                if rx_only
                                    && ui
                                        .button(RichText::new(tr!("frame-listening-controls")).size(13.0))
                                        .on_hover_text(tr!("frame-listening-controls-hint"))
                                        .clicked()
                                {
                                    if let Some(cfg) = self.radio_cfg.clone() {
                                        let mut cfg = cfg;
                                        cfg.hide_tx = true;
                                        cmds.push(Command::SetRadioConfig {
                                            cfg: Box::new(cfg.clone()),
                                            reopen: false,
                                        });
                                        self.radio_cfg = Some(cfg);
                                    }
                                    // An explicit per-radio choice supersedes
                                    // the start-in-SWL seed, the same way the
                                    // Settings switch does.
                                    self.swl_start = false;
                                    self.radio_notice = None;
                                    self.rx_only_nudge_dismissed = true;
                                }
                            }
                        });
                    });
                });
        }
        // **An update is available.** The fork's own GitHub Releases, so only a
        // full release lands here — the nightly is a pre-release and asks
        // nothing. Dismissed for the session; the check itself is the
        // `check_for_updates` setting.
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(tag) = self.update.latest.clone().filter(|_| !self.update_dismissed) {
            let (wash, rule, mark, ink) = notice_banner_colors();
            egui::Frame::new()
                .fill(wash)
                .stroke(egui::Stroke::new(1.0, rule))
                .inner_margin(egui::Margin::symmetric(8, 5))
                .show(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("⬆").size(15.0).color(mark));
                        ui.label(
                            RichText::new(tr!(
                                "frame-update-available",
                                tag = tag,
                                version = sdroxide_version::VERSION
                            ))
                            .size(13.0)
                            .color(ink),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button(tr!("frame-dismiss")).clicked() {
                                self.update_dismissed = true;
                            }
                            if ui
                                .button(RichText::new(tr!("frame-download")).size(13.0))
                                .on_hover_text(tr!("frame-download-hint"))
                                .clicked()
                            {
                                crate::download::open_external(super::update::RELEASES_PAGE);
                            }
                        });
                    });
                });
        }
        // A clicked spot is captured here and pre-filled into a log entry below.
        // The broadcast stations are refreshed first, before anything reads
        // them: the panadapter's network overlay (built in `panadapter_inputs`),
        // the SPOTS list and the world map all do.
        self.refresh_broadcast_spots(now_unix());
        let mut clicked_spot: Option<Spot> = None;
        // Whether the main window's centre will be empty this frame — both the
        // panadapter and the panel in their own windows. Decided before the dock
        // so the two agree: if the centre is empty the band/mode selector fills
        // it (`band_menu_fill`), and the narrow dock is not also drawn.
        let cur_mode = self.state.rx[0].mode;
        let center_empty = !self.center_has_content(cur_mode);
        // A docked band/mode selector takes a column off the right of the
        // panadapter before either draws. Shown here, after the top bar and the
        // notices, so the column sits beside the waterfall rather than under the
        // chrome.
        if self.band_docked && self.band_dock_visible && !center_empty {
            self.band_dock_panel(ui, &mut cmds);
        }
        // Remaining space: the panadapter (+ FT8/FT4 operating panel).
        if let Some(err) = self.error.clone() {
            let offer_retry = self.ctrl.can_reconnect();
            // Seconds until the redial that is already coming, so the screen
            // can say what is going to happen rather than only what went wrong.
            let due_in = self.retry_at.map(|at| (at - crate::time::now_unix_f64()).max(0.0));
            let mut retry = false;
            ui.vertical_centered(|ui| {
                // Roughly where `centered_and_justified` used to put the text,
                // with the button under it rather than beside it.
                ui.add_space(ui.available_height() * 0.4);
                ui.label(RichText::new(err).size(18.0).color(crate::theme::ALERT()));
                if offer_retry {
                    ui.add_space(10.0);
                    ui.label(
                        RichText::new(match due_in {
                            Some(s) if s >= 1.0 => {
                                tr!("frame-reconnecting-s", value = s.ceil() as u64)
                            }
                            Some(_) => tr!("frame-reconnecting").to_string(),
                            None => tr!("frame-not-connected").to_string(),
                        })
                        .size(13.0)
                        .color(crate::theme::gray(150)),
                    );
                    ui.add_space(10.0);
                    retry =
                        ui.button(RichText::new(tr!("frame-reconnect-now")).size(16.0)).clicked();
                }
            });
            if retry {
                // Asking by hand puts the wait back to the bottom: the operator
                // has some reason to believe the far end is up, and making them
                // watch out a backoff they did not choose is not an answer.
                self.retry_backoff = super::RETRY_MIN_S;
                self.reconnect_now();
            }
        } else if center_empty {
            // Both the panadapter and the panel are in their own windows, so
            // there is nothing of the radio left to draw here. Normally the
            // band/mode selector fills it … but when the **controls** window is
            // up too it already carries the selector, so filling again would
            // draw it twice — show a plain face instead, never a black hole.
            #[cfg(not(target_arch = "wasm32"))]
            if self.controls_detached() {
                empty_centre(ui);
            } else {
                self.band_menu_fill(ui, &mut cmds);
            }
            #[cfg(target_arch = "wasm32")]
            self.band_menu_fill(ui, &mut cmds);
        } else if cur_mode.has_bottom_panel() {
            // Remember the voice-mode view once, so leaving FT8 can restore it
            // instead of leaving the panadapter zoomed to the sub-band.
            if self.view.pre_digi_view.is_none() {
                self.view.pre_digi_view = Some((self.view.view_lo_hz, self.view.view_hi_hz));
            }
            let dial = self.state.rx_freq_hz();
            let mode = self.state.rx[0].mode;
            // The digital sub-band (audio 0..3.5 kHz above dial); for RF Paint,
            // tight onto the 300..3300 Hz painting band so incoming pictures
            // are large enough to read on the waterfall. RIFP straddles the
            // dial rather than sitting above it, so its window is symmetric
            // and as wide as the profile's channel.
            let (sub_lo, sub_hi) = if mode.is_ais() {
                // Both AIS channels and a little either side, not a sub-band:
                // the two are 50 kHz apart and both are being read at once, so
                // framing one of them would be a view of half the traffic.
                (dial - 60_000.0, dial + 60_000.0)
            } else if mode.is_vdl2() {
                // The whole datalink group, not a sub-band: seven channels
                // spread over 325 kHz are all being read at once, and framing
                // one of them would be a view of a sixth of what is happening.
                (dial - 250_000.0, dial + 250_000.0)
            } else if mode.is_adsb() {
                // ADS-B is not worked inside a sub-band at all: a Mode S reply
                // is megahertz wide and the decoder reads the whole receiver
                // window. Framing it like a digital mode would zoom the
                // panadapter to a few kilohertz of one pulse's spectrum, which
                // is a view of nothing. The whole channel instead, which is
                // also all there is on this band.
                (dial - 1_500_000.0, dial + 1_500_000.0)
            } else if mode == Mode::Dab {
                // A DAB ensemble is ~1.5 MHz wide and the whole thing is
                // decoded at once, so the panadapter frames the channel as
                // ADS-B does — there is no sub-band to zoom to.
                (dial - 900_000.0, dial + 900_000.0)
            } else if mode.is_hfdl() {
                // The lane is a fixed 24 kHz channel, and entering the mode
                // put the dial on the chosen HFDL frequency; frame the channel
                // with a little either side of it. The decoder reads the lane,
                // not this view — the window is only so the operator can see
                // the signal they are decoding.
                (dial - 15_000.0, dial + 15_000.0)
            } else if mode.is_aprs() {
                // APRS is deliberately *not* framed on its own channel.
                // Every other digital mode is worked inside a sub-band, so
                // framing that sub-band is a service. APRS is one channel on a
                // band an operator has every reason to be watching — the
                // repeater outputs above it, the simplex calling frequency
                // below, a satellite passing over the top — and ±8 kHz around
                // 144.800 would take all of that away in exchange for a view
                // of a channel whose whole content is already in the panel.
                //
                // A window rather than nothing at all, though: this is the one
                // mode that moves the dial by itself, and after a 130 MHz jump
                // a view left where it was is a panadapter showing spectrum
                // from another band entirely. 300 kHz is wide enough to hold
                // the neighbourhood and is only a starting point — zoom and
                // pan stay live, and it is re-applied only on a mode change or
                // a retune that takes the channel off-screen.
                (dial - 150_000.0, dial + 150_000.0)
            } else if mode.is_carrier_centered() {
                let half = (self.state.rx[0].filter_hi - self.state.rx[0].filter_lo).abs() as f64
                    * 0.5
                    * 1.2;
                (dial - half, dial + half)
            } else if mode.is_rf_paint() {
                (dial + 150.0, dial + 3450.0)
            } else if mode == Mode::Navtex {
                // NAVTEX sits ~1.7 kHz above the dial (USB), so a narrow
                // carrier-centred-style window puts it at the far right edge
                // with the CTR chip on. Widen the default span so the signal
                // and a little context either side of it are visible instead
                // of hugging the edge. (Per-decoder remembered zoom is a
                // separate, larger feature — see the PR notes.)
                (dial - 4000.0, dial + 4000.0)
            } else if mode.is_lower_sideband_at(dial) {
                // Mirrored, because the mode is: SSTV and RADE keep phone
                // practice and ride the lower sideband on 160/80/40 m, so their
                // sub-band is *below* the dial there. Framed above it, the
                // opening view of the mode is the empty side of the signal.
                (dial - 3500.0, dial + 200.0)
            } else {
                (dial - 200.0, dial + 3500.0)
            };
            // The slotted modes and WSPR work a fixed, narrow allocation, so
            // their view stays locked to the sub-band. Every other digital mode
            // roams the band (SSTV, Hell, wefax…), so the sub-band is only the
            // *starting* view there — zoom and pan stay live, and the fit is
            // re-applied on a mode change or a QSY that takes the sub-band
            // off-screen. Tuning that keeps it in view (a drag-tune, a nudge)
            // keeps the operator's zoom.
            let locked = mode.is_slotted() || mode.is_wspr();
            let center = self.state.center_hz;
            let mode_changed = self.digi_view_fit.map(|(m, _, _)| m) != Some(mode);
            let dial_moved = self.digi_view_fit.map(|(_, d, _)| d) != Some(dial);
            // A retune moved the span the view has to live inside. Paired
            // with `!sub_visible` below rather than forcing a fit on its own:
            // an operator who panned the front end's window somewhere they
            // wanted to look should keep it, and only a window that no longer
            // contains the sub-band at all has to be re-fitted.
            let span_moved = self.digi_view_fit.map(|(_, _, c)| c) != Some(center);
            let sub_visible = self.view.view_lo_hz < sub_hi && self.view.view_hi_hz > sub_lo;
            if locked || mode_changed || ((dial_moved || span_moved) && !sub_visible) {
                self.view.view_lo_hz = sub_lo;
                self.view.view_hi_hz = sub_hi;
            }
            self.digi_view_fit = Some((mode, dial, center));

            let frame = self.frame.take();
            // A phone shows one of the three at a time, chosen by a row of
            // chips: the decode list, the QSO area and the waterfall each want
            // the whole width and most of the height, and three slivers would
            // be three things too small to use. Everything wider keeps the
            // waterfall above the panel, split by a draggable divider.
            let phone = tier == crate::layout::Tier::Phone;
            let width = ui.available_width();
            if phone {
                self.digi_tabs(ui, mode, true);
            }
            // The waterfall is the tab past the last of the mode's own panes.
            let on_waterfall =
                phone && self.digi_pane(mode) == crate::app::panels::panel_panes(mode).len();
            // Both panadapter layers switched off in the SPEC popup means no
            // panadapter: the decode panel takes the whole height rather than
            // sharing it with an empty strip.
            let layers = self.view.panadapter_visible();
            // A panadapter or panel drawn into its own window takes no room
            // here: whatever is left in the column gets the whole of it. The
            // panadapter's rule is `panadapter_window_wanted`, the same one the
            // shell emits its window by, so the two cannot disagree.
            let detached = self.panadapter_window_wanted();
            let panel_gone = self.panel_in_a_window(mode);
            let show_wf = !detached && (!phone || on_waterfall) && layers;
            let show_panel = (!phone || !on_waterfall) && !panel_gone;

            // Manual vertical split with a draggable divider: the operating
            // panel gets `digi_panel_fraction` of the height, the waterfall the
            // rest. A thin handle between them resizes the split.
            let total = ui.available_height();
            let handle_h = if phone { 0.0 } else { 7.0 };
            let (wf_h, panel_h) = if phone {
                // Whichever one is showing gets all of it.
                (total, total)
            } else {
                // The handle plus the gap the layout inserts on each side of it.
                let divider = handle_h + 2.0 * ui.spacing().item_spacing.y;
                column_split(
                    detached,
                    layers,
                    panel_gone,
                    total,
                    divider,
                    self.view.digi_panel_fraction,
                    ui.spacing().interact_size.y,
                )
            };

            // Scroll only while frames are actually arriving. The last frame is
            // kept for the spectrum line, but rows of it repeated down the
            // waterfall would be time that never happened — a switched-off
            // radio (or any stalled stream) freezes instead.
            let live = frame.is_some() && now - self.last_spectrum_at < STREAM_STALE_S;
            if show_wf {
                // The waterfall clock and the width are recorded by whoever
                // draws the panadapter — here, or the shell when it is undocked
                // — so they advance exactly once a frame either way.
                self.note_panadapter_width(ui);
                let wf_tuning = self.wf_tick(live, ui.ctx().pixels_per_point());
                let inputs = self.panadapter_inputs(now);
                self.draw_panadapter(
                    ui,
                    width,
                    wf_h,
                    frame.as_ref(),
                    &mut cmds,
                    &inputs,
                    &mut clicked_spot,
                    show_panel,
                    now,
                    wf_tuning,
                );
            }
            // Only between two things: with either part undocked or the
            // panadapter switched off there is nothing to divide, and a drag on
            // the handle would silently rewrite a split with nothing to show.
            if !phone && show_wf && show_panel {
                // Resize handle between the waterfall and the FT8/FT4 panel.
                let hresp = crate::chrome::split_handle(
                    ui,
                    egui::vec2(width, handle_h),
                    Some(crate::theme::PANEL()),
                );
                if hresp.dragged() {
                    // Drag down shrinks the panel (waterfall grows), drag up grows it.
                    let d = hresp.drag_delta().y / total;
                    self.view.digi_panel_fraction =
                        (self.view.digi_panel_fraction - d).clamp(0.2, 0.82);
                }
            }
            if show_panel {
                ui.allocate_ui(egui::vec2(width, panel_h), |ui| {
                    self.draw_operating_panel(ui, &mut cmds, mode, panel_h);
                });
            }
            self.frame = frame;
        } else {
            // Restore the pre-FT8 view span once, on the first voice frame
            // after leaving a digital mode.
            if let Some((lo, hi)) = self.view.pre_digi_view.take() {
                self.view.view_lo_hz = lo;
                self.view.view_hi_hz = hi;
            }
            // Forget the digital fit: coming back to the same mode and dial
            // must fit the sub-band afresh, not keep this voice-mode view.
            self.digi_view_fit = None;
            // The full-band strip, above the panadapter and only when a front
            // end actually supplies one — and the operator has left the Display
            // module's WIDE chip on. Never on a phone: 96 pt of strip is a
            // quarter of the height there, taken from the waterfall being
            // listened to for a band view nothing is tuned to.
            if !crate::layout::narrow_phone(ui.ctx())
                && self.view.wide_waterfall
                && let Some(wide) = self.wide_frame.clone()
            {
                crate::widgets::wide_spectrum::show(
                    ui,
                    &mut self.wide_wf,
                    &wide,
                    &self.state,
                    self.ui_settings.waterfall_palette,
                    &mut cmds,
                );
                ui.add_space(2.0);
            }
            let frame = self.frame.take();
            // As on the digital path: no fresh frames, no scroll.
            let live = frame.is_some() && now - self.last_spectrum_at < STREAM_STALE_S;
            // CW is the one analog mode with a panel under the panadapter. It
            // is not a digital mode and does not take the digital path — the
            // demodulated tone stays audible and the view stays wherever the
            // operator left it — but it has a decoder and a keyboard, and both
            // need somewhere to live. The cursor is the mode's own: a marker at
            // the pitch being copied, which a click on the waterfall moves.
            let cw_mode = self.state.rx[0].mode == Mode::Cw;
            let phone = tier == crate::layout::Tier::Phone;
            // Radio Classic takes the same slot in the voice modes that have
            // nothing under the waterfall (see `RadioClassicPlace`).
            let rc_docked = crate::app::panels::radio_classic::place(
                self.ui_settings.radio_classic_on,
                self.show_radio_classic,
                self.state.rx[0].mode,
                phone,
            ) == crate::app::panels::radio_classic::RadioClassicPlace::Docked;
            let panel_mode = cw_mode || rc_docked;
            // As on the digital path above: no layers, no panadapter. In a mode
            // with nothing under it that leaves the pane empty, which is what
            // switching both off asks for.
            let layers = self.view.panadapter_visible();
            // A panadapter or panel drawn into its own window takes no room
            // here; whatever is left in the column gets the whole of it.
            let detached = self.panadapter_window_wanted();
            let panel_gone = self.panel_in_a_window(self.state.rx[0].mode);
            let (wf_h, panel_h, show_wf, show_panel) = if !panel_mode {
                if detached {
                    (0.0, 0.0, false, false)
                } else {
                    (ui.available_height(), 0.0, layers, false)
                }
            } else if phone {
                self.digi_tabs(ui, Mode::Cw, true);
                let on_wf =
                    self.digi_pane(Mode::Cw) == crate::app::panels::panel_panes(Mode::Cw).len();
                let h = ui.available_height();
                (h, h, on_wf, !on_wf)
            } else {
                let total = ui.available_height();
                let divider = 7.0 + 2.0 * ui.spacing().item_spacing.y;
                let (w, p) = column_split(
                    detached,
                    layers,
                    panel_gone,
                    total,
                    divider,
                    self.view.digi_panel_fraction,
                    ui.spacing().interact_size.y,
                );
                (w, p, w > 0.0, p > 0.0)
            };
            let width = ui.available_width();
            if show_wf {
                self.note_panadapter_width(ui);
                let wf_tuning = self.wf_tick(live, ui.ctx().pixels_per_point());
                let inputs = self.panadapter_inputs(now);
                self.draw_panadapter(
                    ui,
                    width,
                    wf_h,
                    frame.as_ref(),
                    &mut cmds,
                    &inputs,
                    &mut clicked_spot,
                    show_panel,
                    now,
                    wf_tuning,
                );
            }
            if show_panel {
                if !phone && show_wf {
                    let hresp = crate::chrome::split_handle(
                        ui,
                        egui::vec2(width, 7.0),
                        Some(crate::theme::PANEL()),
                    );
                    if hresp.dragged() {
                        let d = hresp.drag_delta().y / ui.available_height().max(1.0);
                        self.view.digi_panel_fraction =
                            (self.view.digi_panel_fraction - d).clamp(0.2, 0.82);
                    }
                }
                ui.allocate_ui(egui::vec2(width, panel_h), |ui| {
                    if rc_docked {
                        self.radio_classic.frame = frame.clone();
                        self.radio_classic_docked(ui, &mut cmds, panel_h);
                        self.radio_classic.frame = None;
                    } else {
                        self.draw_operating_panel(ui, &mut cmds, Mode::Cw, panel_h);
                    }
                });
            }
            self.frame = frame;
        }
        // A spot clicked on the panadapter: pre-fill a log entry (tuning + mode
        // were already issued inside the widget).
        if let Some(spot) = clicked_spot {
            self.prefill_from_spot(&spot);
        }

        self.memories_window(&ctx, &mut cmds);
        self.scanner_window(&ctx, &mut cmds);
        self.ism_window(&ctx, &mut cmds);
        self.adsb_setup_window(&ctx, &mut cmds);
        self.cw_macro_window(&ctx, &mut cmds);
        self.text_macro_window(&ctx, &mut cmds);
        self.ais_setup_window(&ctx, &mut cmds);
        self.vdl2_setup_window(&ctx, &mut cmds);
        self.rds_window(&ctx);
        self.drm_window(&ctx, &mut cmds);
        self.hd_window(&ctx, &mut cmds);
        self.voice_window(&ctx, &mut cmds);
        self.settings_window(&ctx, &mut cmds);
        self.digi_settings_window(&ctx, &mut cmds);
        self.logbook_window(&ctx, &mut cmds);
        if self.show_session_qsos {
            self.session_qsos_window(&ctx, &mut cmds);
        }
        self.contest_window(&ctx, &mut cmds);
        self.swl_window(&ctx, &mut cmds);
        self.listen_window(&ctx, &mut cmds);
        self.schedule_window(&ctx, &mut cmds);
        // Sync persisted mask -> show_* flags, for the bits that changed under
        // us since the end of the last frame: that is the Settings → Detachable
        // menu. A flag a chip flipped this frame (MEM, SCAN, EXTRAS, a button in
        // EXTRAS, the Settings button) has its bit still at the old value until
        // the block below writes it, and must not be put back.
        let (mask, outside) = (
            self.ui_settings.tool_windows_open,
            self.ui_settings.tool_windows_open ^ self.prev_tool_windows_mask,
        );
        macro_rules! sync {
            ($flag:expr, $bit:expr) => {
                if outside & $bit != 0 {
                    $flag = mask & $bit != 0;
                }
            };
        }
        sync!(self.show_dsc, TW_DSC);
        sync!(self.show_navtex, TW_NAVTEX);
        sync!(self.show_aprs, TW_APRS);
        sync!(self.show_wefax, TW_WEFAX);
        sync!(self.show_sstv, TW_SSTV);
        sync!(self.show_dab, TW_DAB);
        sync!(self.show_js8, TW_JS8);
        sync!(self.show_acars, TW_ACARS);
        sync!(self.show_hfdl, TW_HFDL);
        sync!(self.show_vdl2, TW_VDL2);
        sync!(self.show_ais, TW_AIS);
        sync!(self.show_adsb, TW_ADSB);
        sync!(self.show_fsq, TW_FSQ);
        sync!(self.show_hell, TW_HELL);
        sync!(self.show_packet, TW_PACKET);
        sync!(self.show_wspr, TW_WSPR);
        sync!(self.show_pi4, TW_PI4);
        sync!(self.show_text, TW_TEXT);
        sync!(self.show_uvpacket, TW_UVPACKET);
        sync!(self.show_jtty, TW_JTTY);
        sync!(self.show_ale, TW_ALE);
        sync!(self.show_rf_paint, TW_RFPAINT);
        sync!(self.show_image, TW_IMAGE);
        // Sync existing tool windows.
        sync!(self.schedule.show, TW_SCHEDULE);
        sync!(self.show_listen, TW_LISTEN);
        sync!(self.show_logbook, TW_LOGBOOK);
        sync!(self.show_session_qsos, TW_SESSION_QSOS);
        sync!(self.show_settings, TW_SETTINGS);
        sync!(self.show_swl, TW_SWL_LOG);
        sync!(self.show_spots, TW_SPOTS);
        sync!(self.show_scanner, TW_SCANNER);
        sync!(self.show_awards, TW_AWARDS);
        sync!(self.show_grid, TW_GRID);
        sync!(self.show_sat, TW_SAT);
        sync!(self.show_memories, TW_MEMORIES);
        sync!(self.show_extras, TW_EXTRAS);
        sync!(self.show_aircraft, TW_AIRCRAFT);
        sync!(self.show_radio_classic, TW_RADIO_CLASSIC);
        sync!(self.show_rds, TW_RDS);
        sync!(self.show_drm, TW_DRM);
        sync!(self.show_hd, TW_HD_RADIO);
        sync!(self.show_ism, TW_ISM);
        sync!(self.show_contest, TW_CONTEST);
        sync!(self.show_bands, TW_BANDS);
        sync!(self.show_public_sdrs, TW_PUBLICSDR);
        if self.show_dsc {
            self.dsc_window(&ctx, &mut cmds);
        }
        if self.show_navtex {
            self.navtex_window(&ctx, &mut cmds);
        }
        if self.show_wefax {
            self.wefax_window(&ctx, &mut cmds);
        }
        let mode = self.state.rx[0].mode;
        // **A mode's panel docks under the waterfall; a window is opened by the
        // operator** (the Detachable page in Settings, the `⇱` chip in the panel's
        // own window to dock it again). This block used to open each mode's panel
        // as a floating window the first time the mode was entered *and* write that
        // into `tool_windows_open`, so every mode came up in a window instead of in
        // the pane below the waterfall, which is the default the operator asked for
        // (issue #217). The windows are all still there, all still detachable.
        if mode.is_sstv() && self.show_sstv {
            self.sstv_window(&ctx, &mut cmds);
        }
        if mode == sdroxide_types::Mode::Dab && self.show_dab {
            self.dab_window(&ctx, &mut cmds);
        }
        self.radio_classic_tick(now, &mut cmds);
        self.radio_classic_window(&ctx, &mut cmds);
        if mode.is_js8() && self.show_js8 {
            self.js8_window(&ctx, &mut cmds);
        }
        if mode == sdroxide_types::Mode::Acars && self.show_acars {
            self.acars_window(&ctx, &mut cmds);
        }
        if mode.is_aprs() && self.show_aprs {
            self.aprs_window(&ctx, &mut cmds);
        }
        if mode.is_hfdl() && self.show_hfdl {
            self.hfdl_window(&ctx, &mut cmds);
        }
        if mode.is_vdl2() && self.show_vdl2 {
            self.vdl2_window(&ctx, &mut cmds);
        }
        if mode.is_ais() && self.show_ais {
            self.ais_window(&ctx, &mut cmds);
        }
        if mode.is_adsb() && self.show_adsb {
            self.adsb_window(&ctx, &mut cmds);
        }
        if mode.is_fsq() && self.show_fsq {
            self.fsq_window(&ctx, &mut cmds);
        }
        if mode.is_hell() && self.show_hell {
            self.hell_window(&ctx, &mut cmds);
        }
        if mode.is_packet() && self.show_packet {
            self.packet_window(&ctx, &mut cmds);
        }
        if mode.is_wspr() && self.show_wspr {
            self.wspr_window(&ctx, &mut cmds);
        }
        if mode.is_pi4() && self.show_pi4 {
            self.pi4_window(&ctx, &mut cmds);
        }
        if mode.is_text_modem() && self.show_text {
            self.text_window(&ctx, &mut cmds);
        }
        if mode == sdroxide_types::Mode::UvPacket && self.show_uvpacket {
            self.uvpacket_window(&ctx, &mut cmds);
        }
        if mode == sdroxide_types::Mode::Jtty && self.show_jtty {
            self.jtty_window(&ctx, &mut cmds);
        }
        if mode == sdroxide_types::Mode::Ale && self.show_ale {
            self.ale_window(&ctx, &mut cmds);
        }
        if mode.is_rf_paint() && self.show_rf_paint {
            self.rf_paint_window(&ctx, &mut cmds);
        }
        if mode.is_image() && image_window_for(mode) == ImageWindow::Image && self.show_image {
            self.image_window(&ctx, &mut cmds, mode);
        }
        // Persist existing tool windows' mask (sync show_* -> mask).
        if self.schedule.show {
            self.ui_settings.tool_windows_open |= TW_SCHEDULE;
        } else {
            self.ui_settings.tool_windows_open &= !TW_SCHEDULE;
        }
        if self.show_listen {
            self.ui_settings.tool_windows_open |= TW_LISTEN;
        } else {
            self.ui_settings.tool_windows_open &= !TW_LISTEN;
        }
        if self.show_logbook {
            self.ui_settings.tool_windows_open |= TW_LOGBOOK;
        } else {
            self.ui_settings.tool_windows_open &= !TW_LOGBOOK;
        if self.show_session_qsos {
            self.ui_settings.tool_windows_open |= TW_SESSION_QSOS;
        } else {
            self.ui_settings.tool_windows_open &= !TW_SESSION_QSOS;
        }
        }
        if self.show_settings {
            self.ui_settings.tool_windows_open |= TW_SETTINGS;
        } else {
            self.ui_settings.tool_windows_open &= !TW_SETTINGS;
        }
        if self.show_swl {
            self.ui_settings.tool_windows_open |= TW_SWL_LOG;
        } else {
            self.ui_settings.tool_windows_open &= !TW_SWL_LOG;
        }
        if self.show_spots {
            self.ui_settings.tool_windows_open |= TW_SPOTS;
        } else {
            self.ui_settings.tool_windows_open &= !TW_SPOTS;
        }
        if self.show_scanner {
            self.ui_settings.tool_windows_open |= TW_SCANNER;
        } else {
            self.ui_settings.tool_windows_open &= !TW_SCANNER;
        }
        if self.show_awards {
            self.ui_settings.tool_windows_open |= TW_AWARDS;
        } else {
            self.ui_settings.tool_windows_open &= !TW_AWARDS;
        }
        if self.show_grid {
            self.ui_settings.tool_windows_open |= TW_GRID;
        } else {
            self.ui_settings.tool_windows_open &= !TW_GRID;
        }
        if self.show_sat {
            self.ui_settings.tool_windows_open |= TW_SAT;
        } else {
            self.ui_settings.tool_windows_open &= !TW_SAT;
        }
        if self.show_memories {
            self.ui_settings.tool_windows_open |= TW_MEMORIES;
        } else {
            self.ui_settings.tool_windows_open &= !TW_MEMORIES;
        }
        if self.show_extras {
            self.ui_settings.tool_windows_open |= TW_EXTRAS;
        } else {
            self.ui_settings.tool_windows_open &= !TW_EXTRAS;
        }
        if self.show_aircraft {
            self.ui_settings.tool_windows_open |= TW_AIRCRAFT;
        } else {
            self.ui_settings.tool_windows_open &= !TW_AIRCRAFT;
        }
        if self.show_rds {
            self.ui_settings.tool_windows_open |= TW_RDS;
        } else {
            self.ui_settings.tool_windows_open &= !TW_RDS;
        }
        if self.show_drm {
            self.ui_settings.tool_windows_open |= TW_DRM;
        } else {
            self.ui_settings.tool_windows_open &= !TW_DRM;
        }
        if self.show_hd {
            self.ui_settings.tool_windows_open |= TW_HD_RADIO;
        } else {
            self.ui_settings.tool_windows_open &= !TW_HD_RADIO;
        }
        if self.show_ism {
            self.ui_settings.tool_windows_open |= TW_ISM;
        } else {
            self.ui_settings.tool_windows_open &= !TW_ISM;
        }
        if self.show_contest {
            self.ui_settings.tool_windows_open |= TW_CONTEST;
        } else {
            self.ui_settings.tool_windows_open &= !TW_CONTEST;
        }
        if self.show_bands {
            self.ui_settings.tool_windows_open |= TW_BANDS;
        } else {
            self.ui_settings.tool_windows_open &= !TW_BANDS;
        }
        if self.show_public_sdrs {
            self.ui_settings.tool_windows_open |= TW_PUBLICSDR;
        } else {
            self.ui_settings.tool_windows_open &= !TW_PUBLICSDR;
        }
        if self.ui_settings.tool_windows_open != self.prev_tool_windows_mask {
            crate::app::persist::persist_ui_settings(&self.ui_settings);
            self.prev_tool_windows_mask = self.ui_settings.tool_windows_open;
        }
        // **The operating pane, floating in this window** — the other half of the
        // icon in a panel's header. Drawn here, beside the tool windows, because
        // it is one of them in everything but where its body comes from.
        if self.panel_floating {
            self.floating_panel_window(&ctx, &mut cmds, mode);
        }
        // **The module-windows list**, from the ▣ chip or `Ctrl+Alt+W`.
        #[cfg(not(target_arch = "wasm32"))]
        if self.show_windows {
            self.module_windows_window(&ctx);
        }
        self.recordings_window(&ctx, &mut cmds);
        self.poll_recording_jobs(&mut cmds);
        self.mail_window(&ctx, &mut cmds);
        self.mail_log_window(&ctx);
        self.spots_window(&ctx, &mut cmds);
        self.public_sdrs_window(&ctx, &mut cmds);
        self.awards_window(&ctx);
        self.grid_tracker_window(&ctx);
        self.bands_window(&ctx);
        self.sat_window(&ctx, &mut cmds);
        self.morse_window(&ctx);
        self.signal_id_window(&ctx);
        self.enigma_window(&ctx);
        self.poll_known_calls(&mut cmds);
        self.known_calls_window(&ctx);
        self.weather_window(&ctx);
        self.poll_playback_list(&mut cmds);
        self.playback_window(&ctx, &mut cmds);
        self.extras_window(&ctx, &mut cmds);
        self.antenna_window(&ctx);
        self.aircraft_watch_tick(&ctx);
        self.programs_sync_tick(&ctx);
        self.aircraft_window(&ctx, &mut cmds);
        self.help.ui(&ctx);
        // Last, so it lands on top of everything else that opened this frame.
        self.oob_tx_window(&ctx);
        self.cb_tx_confirm_window(&ctx, &mut cmds);
        #[cfg(not(target_arch = "wasm32"))]
        {
            let grid = self.my_grid();
            let traffic = self.digi_traffic(ctx.input(|i| i.time));
            // Only while the window is open: walking the whole logbook is not
            // free, and the closed window has nothing to paint it on.
            let awards = if self.solar.open { self.award_heat() } else { Default::default() };
            // Same reasoning: the field is aged when it is read, and a closed
            // window has nothing to age it for. The evidence itself keeps
            // accumulating either way — it is fed from the panels, not here.
            let prop = if self.solar.open {
                self.prop.field(crate::time::now_unix())
            } else {
                Default::default()
            };
            // The geometry the window should open at, from this screen's
            // settings. Only consulted when the window is (re)built.
            self.solar.set_window_seed(self.ui_settings.solar3d_window);
            let lock_change = self.solar.viewport(
                &ctx,
                &grid,
                traffic,
                awards,
                prop,
                self.band_conditions.clone(),
                self.band_activity.clone(),
                self.psk_activity.clone(),
                std::sync::Arc::clone(&self.sat_cfg),
                self.sat_track.as_ref().map(|t| t.norad_id),
            );
            // ...and keep where it actually ended up, so the next open — after a
            // restart, or after the window is closed and reopened — returns it
            // there.
            if let Some(geom) = self.solar.window_now() {
                self.ui_settings.solar3d_window = Some(geom);
            }
            self.view.solar3d = self.solar.persisted();
            // The pass window's LOCK button lands here: the 3D window has no
            // command path of its own, so the request is drained and acted on
            // where one exists.
            match lock_change {
                Some(crate::solar3d::LockChange::Lock(id)) => {
                    self.sat_lock_request(id, &mut cmds);
                }
                Some(crate::solar3d::LockChange::Unlock) => {
                    cmds.push(Command::SetSatLock(None));
                    self.sat_win.sent = None;
                }
                Some(crate::solar3d::LockChange::Tune(id, link_idx)) => {
                    self.sat_tune_request(id, link_idx, &mut cmds);
                }
                None => {}
            }
        }

        // Debounced spectrum-config updates with pan hysteresis.
        let now = ctx.input(|i| i.time);
        // Ahead of them, so a fit picked from this frame's spectrum rides out
        // with the config update it just changed rather than waiting for the
        // next frame. The panadapter has drawn by now, so any pan, zoom or
        // retune this frame is already in `view` for the tick to notice.
        self.auto_fit_tick(now);
        if !self.cfg_still_good() {
            let ideal = self.desired_spectrum_cfg();
            if self.desired_cfg != Some(ideal) {
                self.desired_cfg = Some(ideal);
                self.desired_at = now;
            }
            if self.sent_cfg.is_none() || now - self.desired_at >= CFG_DEBOUNCE_S {
                self.sent_cfg = Some(ideal);
                cmds.push(Command::SetSpectrumCfg(ideal));
            }
        }

        // The skimmers decode only what is on screen, and on a front end wider
        // than their window it is also what decides which slice of the band they
        // read at all — so they need the real visible span, not
        // `SpectrumConfig::viewport`, which is padded so that panning doesn't
        // clear the waterfall. Debounced on the same timer: a drag would
        // otherwise re-cut the tracked set every frame, and every re-cut throws
        // away decoders that were part-way through a callsign.
        if self.state.skimmer.any_enabled() && !self.view.is_unset() {
            let want = (self.view.view_lo_hz, self.view.view_hi_hz);
            let tol = (want.1 - want.0).abs() * 0.01;
            let moved = self
                .sent_skim_view
                .is_none_or(|(lo, hi)| (lo - want.0).abs() > tol || (hi - want.1).abs() > tol);
            if moved {
                if self.desired_skim_view != Some(want) {
                    self.desired_skim_view = Some(want);
                    self.desired_skim_view_at = now;
                }
                if now - self.desired_skim_view_at >= CFG_DEBOUNCE_S {
                    self.sent_skim_view = Some(want);
                    cmds.push(Command::SetSkimmerView(Some(want)));
                }
            }
        }

        // A picture-upload TEST whose answer never came: the station went
        // away, or dropped it. Give the button back rather than spin for ever.
        // Past the connect, login and transfer timeouts the station applies.
        if self.picup_test_pending.is_some_and(|t| crate::time::now_unix_f64() - t > 75.0) {
            self.picup_test_pending = None;
        }

        // A Wavelog download the settings tab asked for. Here rather than in
        // the settings page because it becomes a `Command`, and the settings
        // page is drawn through `&self`.
        self.wavelog_tick(&mut cmds);

        // Flush queued lookups / uploads accumulated during window rendering.
        for call in std::mem::take(&mut self.pending_lookups) {
            cmds.push(Command::LookupCallsign { call });
        }
        for (qso_id, adif, targets) in std::mem::take(&mut self.pending_uploads) {
            cmds.push(Command::UploadQso { qso_id, adif, targets });
        }

        // Run the announcer's timers after the drain, so anything a command
        // just caused is diffed on the snapshot the engine sends back rather
        // than on the optimistic local copy.
        let speech_deadline = self.speech.announcer.tick(&self.state, self.meters.as_ref(), now);
        // Attenuate the receiver while an announcement plays. Only when this
        // client owns the engine: a remote listener taps the same mixer, and
        // ducking their audio for speech they cannot hear would be rude.
        if !self.ctrl.engine_is_remote()
            && let Some(gain) = self.speech.announcer.take_duck_change()
        {
            self.ctrl.send(Command::SetAudioDuck(gain));
        }

        // Any stop control — STOP QSO, STOP TX, a bound Abort TX — disarms
        // auto mode. An unattended run must never be left sequencing after the
        // operator has told the radio to stop, whatever route they used.
        // (Also inside `dispatch_commands`, so a module's own window cannot
        // route around it.)
        self.dispatch_commands(cmds);

        // Data-driven repaint: wake at the next expected spectrum frame while
        // something is streaming, and idle-poll when nothing is. User input
        // wakes eframe by itself, so interactivity is unaffected.
        //
        // Data already waiting — it arrived while this frame was being built,
        // and is checked after the drain — counts as streaming, so a receiver
        // that is delivering keeps the frame cadence rather than dropping to
        // the idle poll. It does *not* earn a frame early: this used to ask for
        // an unpaced `request_repaint` instead, and because a real radio has
        // something waiting at the end of almost every frame, that branch was
        // the one always taken and the frame-rate setting did nothing at all.
        // A synthetic source never showed it — it has nothing to say between
        // frames — which is exactly why it stood for so long.
        let fps = self
            .sent_cfg
            .or(self.desired_cfg)
            .map(|c| c.fps)
            .unwrap_or(SpectrumConfig::default().fps)
            .max(1) as u64;
        let streaming = self.ctrl.wants_repaint_soon()
            || (self.frame.is_some()
                && self.error.is_none()
                && now - self.last_spectrum_at < STREAM_STALE_S);
        // Floor division keeps the poll period <= the stream period, so no
        // frame is ever skipped (the spectrum buffer is latest-wins).
        let mut wait_ms = if streaming { 1000 / fps } else { IDLE_POLL_MS };
        // A settle timer only fires when a frame runs. Idling at a quarter
        // of a second would smear a 600 ms frequency settle across most of
        // a second, so wake for it instead.
        if let Some(at) = speech_deadline {
            let ms = ((at - now).max(0.0) * 1000.0).ceil() as u64;
            wait_ms = wait_ms.min(ms.max(1));
        }
        crate::repaint::schedule_ms(&ctx, wait_ms);
        let salt = u64::from(self.radio_id);
        self.konami.draw(ui, salt);
        // Ctrl+C copies the tooltip on screen — after everything has drawn, so
        // this frame's tooltip is there to be read.
        crate::tooltip_copy::tick(&ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, &self.view_key, &self.view);
        // The station-wide keys are written by exactly one tab — every tab
        // holds its own copy of these, and the last to save would otherwise
        // overwrite the tab the operator actually changed them on.
        if self.station_writer {
            // On wasm this is the logbook's persistence; on native it's a
            // harmless backup (the authoritative copy is written to the config
            // dir on change).
            eframe::set_value(storage, "qso_log", &self.qso_log);
            // Same split: authoritative on native is config.toml (written on
            // change).
            // The browser keeps nothing of PROGRAMS (TOUT, stars, filters,
            // colours): the station does, and hands it back at the next start.
            #[cfg(target_arch = "wasm32")]
            let ui = {
                let mut ui = self.ui_settings.clone();
                ui.programs = Default::default();
                ui.programs_colour_all = None;
                ui.programs_colours.clear();
                ui
            };
            #[cfg(not(target_arch = "wasm32"))]
            let ui = &self.ui_settings;
            eframe::set_value(storage, "ui_settings", &ui);
            // Control-input bindings: authoritative on native is input.json.
            eframe::set_value(storage, "input", &self.input.cfg);
            // The Morse trainer's progress, so it carries to the browser build.
            eframe::set_value(storage, "morse_progress", &self.morse.progress);
            // The custom filter presets: filter_presets.json natively.
            #[cfg(target_arch = "wasm32")]
            eframe::set_value(storage, "filter_presets", &self.filter_presets);
        }
    }
}

/// A DAB channel scan in progress.
///
/// Which Band III blocks carry an ensemble is a fact about where the operator
/// is — the same 12C is a multiplex here and silence in the next country — so
/// the picker is filled by listening, not by a list that only fits one place.
/// The scan walks the blocks, dwells on each long enough for the FIC to name a
/// service, and keeps the ones that produced one.
pub(in crate::app) struct DabScan {
    /// Index into [`sdroxide_types::DAB_BAND_III`] of the block being listened
    /// to.
    pub at: usize,
    /// When the scan arrived on that block, for the dwell. Stamped by the tick
    /// itself, on egui's frame clock — the same clock the tick reads — rather
    /// than by whoever started the scan, so the two cannot be different clocks
    /// (a Unix stamp here against egui's uptime made the dwell ~never elapse).
    pub since: Option<f64>,
    /// Blocks found so far, in the order they were found.
    pub found: Vec<String>,
}

/// How long the scan listens on one block before moving on. The FIC names an
/// ensemble within a second or so of lock on any real signal; three is a
/// comfortable margin that keeps a 38-block sweep under two minutes.
const DAB_SCAN_DWELL_S: f64 = 3.0;

impl SdroxideApp {
    /// Everything the panadapter draw needs, gathered from `self` and the
    /// current mode so the draw is self-contained (see [`PanadapterInputs`]).
    /// Both frame-loop paths and the undocked window call this.
    fn panadapter_inputs(&self, now: f64) -> PanadapterInputs {
        let mode = self.state.rx[0].mode;
        let dial = self.state.rx_freq_hz();
        // Which side of the dial the mode's audio band is on. Every tone offset
        // below is a distance from the dial and every marker is drawn at
        // dial + offset, so on the bands where the mode rides the lower
        // sideband (SSTV and RADE on 160/80/40 m) they all belong below it.
        // Display only.
        let side = if mode.is_lower_sideband_at(dial) { -1.0 } else { 1.0 };
        let audio_hz = side * self.digi_status.as_ref().map(|s| s.audio_hz).unwrap_or(1500.0);
        // RTTY shows mark/space tuning lines; Olivia the tone-bank edges; PSK
        // just the centre marker.
        let markers: Vec<f32> = if mode == Mode::Rtty {
            let sh = self.digi_status.as_ref().map(|s| s.config.rtty_shift_hz).unwrap_or(170.0);
            vec![audio_hz - sh / 2.0, audio_hz + sh / 2.0]
        } else if mode == Mode::Olivia || mode == Mode::Contestia {
            let bw = self.digi_status.as_ref().map(|s| s.config.olivia_bw_hz).unwrap_or(1000.0);
            vec![audio_hz - bw / 2.0, audio_hz + bw / 2.0]
        } else if mode == Mode::Thor {
            let baud =
                self.digi_status.as_ref().map(|s| s.config.thor_mode.baud()).unwrap_or(15.625);
            let bw = 18.0 * baud;
            vec![audio_hz - bw / 2.0, audio_hz + bw / 2.0]
        } else if mode == Mode::Mfsk {
            // The tone bank is the signal: its edges are where the highest and
            // lowest tone sit, so a station that is half out of the passband
            // looks like one.
            let bw = self
                .digi_status
                .as_ref()
                .map(|s| s.config.mfsk_mode.bandwidth_hz())
                .unwrap_or_else(|| sdroxide_types::MfskMode::default().bandwidth_hz());
            vec![audio_hz - bw / 2.0, audio_hz + bw / 2.0]
        } else if mode == Mode::Mt63 {
            let bw = self
                .digi_status
                .as_ref()
                .map(|s| s.config.mt63_mode.bandwidth_hz() as f32)
                .unwrap_or(2000.0);
            vec![audio_hz - bw / 2.0, audio_hz + bw / 2.0]
        } else if mode == Mode::Stanag4285 {
            // The standard's occupied bandwidth is 3000 Hz about the 1800 Hz
            // carrier, so the picture's edges are where the signal should end.
            vec![audio_hz - 1500.0, audio_hz + 1500.0]
        } else if mode == Mode::Ardop {
            // The reference's carrier grid runs 800–2200 Hz about its 1500 Hz
            // centre, so the frame is marked at those edges relative to the
            // audio centre the mode holds.
            vec![audio_hz - 700.0, audio_hz + 700.0]
        } else if mode == Mode::Js8 {
            // Worth showing: Turbo's 160 Hz footprint against Slow's 25 Hz is
            // what decides whether a frequency is actually free.
            let bw = self
                .digi_status
                .as_ref()
                .and_then(|s| s.js8.as_ref())
                .map_or(50.0, |j| j.speed.bandwidth_hz());
            vec![audio_hz, audio_hz + bw]
        } else if mode == Mode::Fsq {
            let baud = self.digi_status.as_ref().map(|s| s.config.fsq_baud).unwrap_or(4.5);
            let bw = 33.0 * baud;
            vec![audio_hz - bw / 2.0, audio_hz + bw / 2.0]
        } else if mode == Mode::Hell {
            let v = self.digi_status.as_ref().map(|s| s.config.hell_variant).unwrap_or_default();
            let bw = v.bandwidth_hz() as f32;
            vec![audio_hz - bw / 2.0, audio_hz + bw / 2.0]
        } else if mode == Mode::RfPaint {
            // The painting band edges (300..3300 Hz).
            vec![300.0, 3300.0]
        } else if mode == Mode::Rade {
            // The RADE V1 OFDM carriers, so the operator can see whether the
            // signal is sitting inside the modem's window.
            vec![side * 1062.0, side * 1876.0]
        } else {
            Vec::new()
        };
        // Station boxes: FT8's callsign overlay for the slotted modes, the CW
        // skimmer for CW, nothing otherwise.
        let (skimmer, alpha) = if mode == Mode::Cw {
            self.cw_overlay(now)
        } else if mode.is_slotted() {
            self.ft8_overlay()
        } else {
            (Vec::new(), Vec::new())
        };
        let (net_spots, net_alpha) = self.net_overlay(now_unix());
        let ism = self.ism_overlay();
        let mem = self.memory_overlay();
        // The tuning cursor: CW's pitch marker, or the digital mode's offset.
        // Voice modes get none — there is no agreed dial to park a signal on.
        let cursor = if mode == Mode::Cw {
            Some(spectrum_view::AudioCursor {
                hz: self.cw_pitch_hz(),
                // A click tunes the dial so the signal lands on the cursor.
                click_sets_offset: false,
                // With the readout reading the signal, the tuning line follows
                // it there — see `UiSettings::cw_qrg`.
                line_on_cursor: self.ui_settings.cw_qrg,
                center_on_cursor: self.ui_settings.cw_qrg,
                center_offset_hz: None,
            })
        } else if mode.has_bottom_panel() {
            // **SSTV.** Its view is anchored on the
            // picture rather than on the tone it happens to be carrying, and CTR
            // has to centre on that anchor — with the dial as the anchor it
            // dragged the window back to the carrier every frame, which is
            // exactly what the operator saw when clicking CTR moved the view.
            // The picture's centre is the band's 1750 Hz, signed by the side the
            // mode rides (SSTV is LSB on 160/80/40 m), and it is a *view* anchor
            // only: the logged frequency stays the dial.
            //
            // Every front end, SDRs included (2026-10-12): on a HackRF the dial
            // sits on the calling frequency and the picture 1750 Hz beside it,
            // exactly as on a sound-card rig — see `focus_hz`.
            let sstv_picture = mode.is_sstv();
            // The same split, for the same reason, one mode over: the tone pair
            // is *only* off the dial on a front end that hands over demodulated
            // audio. On an SDR the dial is on the station — RTTY's own docs say
            // so — and centring 2210 Hz above it put the station outside the
            // window at any span narrow enough to read the tones, which is the
            // span a contest operator uses (Thieu, VERON).
            let audio_front_end = self.caps.as_ref().is_some_and(|c| c.audio_mode);
            Some(spectrum_view::AudioCursor {
                hz: if sstv_picture {
                    side * crate::app::spectrum::SSTV_TONE_HZ as f32
                } else {
                    audio_hz
                },
                // A click sets the digital TX offset in the modes that have one.
                // It does not on a listening source with no transmitter: a click
                // is then the only way to nudge the dial inside the passband a
                // hardware-demodulated radio hands over — so it tunes, as CW.
                //
                // Deliberately still keyed on `holds_standard_tones()` alone and
                // not on the front end: a click in RTTY tunes the dial on *every*
                // radio, which is what commit 7c8a14c3 wanted, and it is right on
                // an SDR too. Only the centring below is front-end-dependent.
                click_sets_offset: !mode.holds_standard_tones()
                    && !self.atsmini_active()
                    && !mode.holds_audio_centre(),
                line_on_cursor: false,
                // **The picture, not the dial**, for the two image modes; the
                // tone pair for an audio front end; the dial everywhere else.
                center_on_cursor: (audio_front_end && mode.holds_standard_tones())
                    || Self::centres_on_picture(mode)
                    || sstv_picture
                    || mode.holds_audio_centre(),
                // FT8 and co: the middle of the decode band, not the dial.
                center_offset_hz: mode.audio_band_centre_hz().map(|c| side * c),
            })
        } else {
            None
        };
        let dxped = if matches!(mode, Mode::Ft8 | Mode::Ft2) {
            self.digi_status.as_ref().map(|s| s.config.dxped_mode).unwrap_or_default()
        } else {
            sdroxide_types::DxpedMode::Normal
        };
        let auto_tx_freq = mode.is_slotted()
            && self.digi_status.as_ref().map(|s| s.config.auto_tx_freq).unwrap_or(true);
        let hold_tx_freq = mode.is_slotted()
            && self.digi_status.as_ref().map(|s| s.config.hold_tx_freq).unwrap_or(false);
        PanadapterInputs {
            cursor,
            dxped,
            auto_tx_freq,
            hold_tx_freq,
            markers,
            skimmer,
            alpha,
            net_spots,
            net_alpha,
            ism,
            mem,
        }
    }

    /// Whether **CTR centres the picture rather than the dial** in `mode`.
    ///
    /// The digital modes carry their information on a tone above the dial, and
    /// centring the dial on an SDR puts that tone off to one side. For the image
    /// modes that is not a convenience: the picture is the thing the operator is
    /// looking at. SSTV has had this since the beginning; a fax had not, so CTR
    /// on a weather chart centred the carrier and left 1900 Hz of the picture in
    /// the margin (reported on #239).
    fn centres_on_picture(mode: Mode) -> bool {
        mode.is_sstv() || mode.is_wefax()
    }

    /// Draw the panadapter — spectrum, waterfall, level slider and the ATS
    /// "catching up" note — into `ui`, filling `width × wf_h`.
    ///
    /// Shared by the digital path, the CW/analog path and the undocked window.
    /// Everything mode-specific rides `inputs` (gathered by
    /// [`Self::panadapter_inputs`]), so the picture is identical wherever it is
    /// drawn from. `panel_below` is `show_ext`'s bandplan-strip gate: whether
    /// the mode's own panel is on screen under this.
    fn draw_panadapter(
        &mut self,
        ui: &mut egui::Ui,
        width: f32,
        wf_h: f32,
        frame: Option<&std::sync::Arc<sdroxide_types::SpectrumFrame>>,
        cmds: &mut Vec<Command>,
        inputs: &PanadapterInputs,
        clicked_spot: &mut Option<Spot>,
        panel_below: bool,
        now: f64,
        wf_tuning: spectrum_view::WfTuning,
    ) {
        ui.allocate_ui(egui::vec2(width, wf_h), |ui| {
            let pan = spectrum_view::WindowPan::of(self.caps.as_ref(), self.state.center_hz)
                .with_outer(Some(self.zoom_out_window()), self.state.sample_rate);
            // The level slider takes a narrow column off the right of the
            // panadapter. `split` declines on a window too small to spare it,
            // and the picture then keeps every column.
            let area = ui.available_rect_before_wrap();
            let (spec_area, level) = match crate::widgets::level_slider::split(area) {
                Some((s, l)) => (s, Some(l)),
                None => (area, None),
            };
            // Reserve the panadapter's whole height in this layout before
            // drawing into a child: `new_child`'s allocations are invisible to
            // `allocate_ui`'s space accounting, so without this the split handle
            // and the panel below were laid out as if the panadapter had taken
            // no height at all — which collapsed the waterfall to a sliver in
            // the modes that have a panel under it.
            ui.allocate_space(area.size());
            let mut spec_ui = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(spec_area)
                    .layout(egui::Layout::top_down(egui::Align::Min)),
            );
            spec_ui.shrink_clip_rect(spec_area);
            // A locked satellite's transponder bands, for the frequency scale.
            // The operator's own frequency table wins over the built-in one —
            // the same rule the SAT window and the 3D pass view apply, so the
            // labels cannot disagree with the pane that lists them.
            let sat_bands: Vec<sdroxide_types::SatBand> =
                self.sat_track
                    .as_ref()
                    .and_then(|t| {
                        let freqs = self.sat_cfg.freqs_for(t.norad_id).cloned().or_else(|| {
                            sdroxide_solar::satfreq::builtin_for(t.norad_id).cloned()
                        })?;
                        Some(sdroxide_types::sat_bands(&freqs, t.doppler_rx_hz))
                    })
                    .unwrap_or_default();
            spectrum_view::show_ext(
                &mut spec_ui,
                &mut self.view,
                &mut self.state,
                frame,
                &mut self.peaks,
                &mut self.spec_smooth,
                &mut self.trace_cache,
                &mut self.spec3d,
                inputs.cursor,
                inputs.dxped,
                inputs.auto_tx_freq,
                inputs.hold_tx_freq,
                &inputs.markers,
                &inputs.skimmer,
                &inputs.alpha,
                &inputs.net_spots,
                &inputs.net_alpha,
                clicked_spot,
                &inputs.ism,
                &inputs.mem,
                self.input.cfg.wheel,
                pan,
                wf_tuning,
                panel_below,
                &sat_bands,
                self.ui_settings.memory_waterfall_lanes,
                cmds,
            );
            // The ATS Mini tunes by stepping its own band cycle, so the dial can
            // be a beat ahead of the radio. It hands us audio only, so the
            // waterfall is where the eye already is: say it here, centred,
            // rather than under a dial the radio has not reached.
            if self.atsmini_tuning && self.atsmini_active() {
                centred_waterfall_note(&spec_ui, spec_area, &tr!("frame-tuning-catching-up"));
            }
            if let Some(l) = level
                && crate::widgets::level_slider::show(ui, l, &mut self.view)
            {
                // A hand on the level is a manual override: the next automatic
                // fit would otherwise walk it back.
                self.view.auto_fit = false;
            }
            paint_panadapter_chrome(ui, area);
            paint_panadapter_clock(ui, area, self.view.span(), self.band_tx_pill());
            if self.levels_hidden(now) && levels_hidden_chip(ui, spec_area) {
                self.view.auto_fit = true;
                self.fit_levels_now(now);
            }
        });
    }

    /// SP1's toolbar: the header bar SDRuno's main-spectrum window carries.
    ///
    /// The frequency readout that used to float over the middle of the waterfall now
    /// sits in a bar across the top with the mode and the signal level beside it; a
    /// **DISP** chip opens the same layer menu the main window's SPEC chip does, so
    /// the toggles SP1 keeps are the ones the whole program uses; and a **DOCK**
    /// chip brings the window home without hunting for its close box.
    ///
    /// The readout stays display-only — a click or drag on the spectrum still tunes,
    /// and the tuning strip stays in the controls window — so there is a single owner
    /// for the frequency and the two windows cannot disagree.
    #[cfg(not(target_arch = "wasm32"))]
    fn sp1_toolbar(&mut self, ui: &mut egui::Ui, module: sdroxide_types::DetachableModule) {
        let area = ui.available_rect_before_wrap();
        ui.allocate_ui(egui::vec2(area.width(), SP1_TOOLBAR_H), |ui| {
            let bar = ui.max_rect();
            ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                ui.spacing_mut().item_spacing = egui::vec2(10.0, 0.0);
                ui.label(
                    RichText::new(readout_text(self.state.rx_freq_hz()))
                        .monospace()
                        .size(21.0)
                        .color(egui::Color32::from_rgb(255, 209, 66)),
                );
                ui.label(
                    RichText::new(detached_status_line(
                        self.state.rx[0].mode,
                        self.meters.as_ref(),
                    ))
                    .size(12.0)
                    .color(egui::Color32::LIGHT_GRAY),
                );
                crate::chrome::row_tail(ui, |ui| {
                    self.layers_button(ui, "DISP", 0.0);
                    if crate::chrome::chip_window_icon(ui, crate::chrome::WindowIcon::In)
                        .on_hover_text(tr!("frame-dock-spectrum-hint"))
                        .clicked()
                    {
                        self.ui_settings.set_detached(module, false);
                        crate::app::persist::persist_ui_settings(&self.ui_settings);
                    }
                });
            });
            // A hairline under the bar, so it reads as a bar and not as a row of
            // text floating over the picture.
            ui.painter().hline(
                egui::Rangef::new(bar.left(), bar.right()),
                bar.bottom(),
                egui::Stroke::new(1.0, crate::theme::LINE()),
            );
        });
    }

    /// One panadapter window's contents — SP1's or AUX SP's. The caller has
    /// already put the right `self.view` in place (AUX swaps its own in around
    /// this call), so the draw itself is shared.
    #[cfg(not(target_arch = "wasm32"))]
    fn draw_detached_panadapter(
        &mut self,
        module: sdroxide_types::DetachableModule,
        ctx: &egui::Context,
        spec: &DetachedWindowSpec,
        seed: Option<sdroxide_types::DetachedWindow>,
        now: f64,
        cmds: &mut Vec<Command>,
    ) {
        let live = self.frame.is_some() && now - self.last_spectrum_at < STREAM_STALE_S;
        let wf_tuning = self.wf_tick(live, ctx.pixels_per_point());
        let frame = self.frame.clone();
        let inputs = self.panadapter_inputs(now);
        let mut clicked_spot: Option<Spot> = None;
        let child = ChildWindow::own(self);
        let outcome = detached_viewport(ctx, spec, seed, child, |ui| {
            // The toolbar across the top, then the spectrum below it.
            // `panel_below` is false: the operating panel is not under this
            // window, so the band-plan strip may use the rest of the height.
            self.sp1_toolbar(ui, module);
            self.note_panadapter_width(ui);
            let w = ui.available_width();
            let h = ui.available_height();
            self.draw_panadapter(
                ui,
                w,
                h,
                frame.as_ref(),
                cmds,
                &inputs,
                &mut clicked_spot,
                false,
                now,
                wf_tuning,
            );
        });
        self.handle_detached_outcome(ctx, module, spec, outcome);
        self.dispatch_commands(std::mem::take(cmds));
        if let Some(spot) = clicked_spot {
            self.prefill_from_spot(&spot);
        }
    }

    /// Draw one module into its **own OS window** (native only), from the
    /// focused radio's state.
    ///
    /// The **shell** calls this once per undocked module per frame — see
    /// [`Self::detached_wanted`] — so a window's existence is the shell's
    /// business and a module only has to say how to draw itself. That is what
    /// keeps a window alive across a radio or mode switch instead of tearing
    /// down and re-mapping.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn show_detached_module(
        &mut self,
        module: sdroxide_types::DetachableModule,
        ctx: &egui::Context,
        now: f64,
    ) {
        use sdroxide_types::DetachableModule as M;
        let spec = detached_spec(module);
        let seed = self.ui_settings.detached_state(module).window;
        let mut cmds: Vec<Command> = Vec::new();
        match module {
            // SP1 and AUX SP are the same window drawn twice: one spectrum and
            // waterfall, its own toolbar, its own app-id, its own place on the
            // desk. AUX is a second view of the same receiver, but not a mirror
            // of it — the draw reads `self.view`, so AUX is drawn with its **own**
            // view swapped in, and the two spectra can sit on different bands of
            // zoom and different layer switches. (A `mem::swap` around the draw
            // rather than a view parameter threaded through `draw_panadapter` and
            // its callees; the operator's own complaint was that they mirrored.)
            M::Panadapter => {
                self.draw_detached_panadapter(module, ctx, &spec, seed, now, &mut cmds)
            }
            M::AuxPanadapter => {
                std::mem::swap(&mut self.view, &mut self.aux_view);
                self.draw_detached_panadapter(module, ctx, &spec, seed, now, &mut cmds);
                std::mem::swap(&mut self.view, &mut self.aux_view);
            }
            M::Panel => {
                // Only reached for a mode that has a panel — the shell asks
                // `panel_window_wanted`, which is false otherwise.
                let mode = self.state.rx[0].mode;
                let mut dock = false;
                // **This window's pane is its own.** Swapped in around the draw,
                // the same bargain AUX SP strikes with its view: the panel
                // window may sit on the decode list while the main window shows
                // something else, and a write from inside — answering a call
                // moves to the QSO pane — lands in the window being looked at.
                // A stored index from another mode, or the waterfall tab the
                // main window's phone row may have selected, is clamped to a
                // real pane first: there is no picture here for that tab.
                let panes = super::panels::panel_panes(mode).len();
                self.panel_window_pane = self.panel_window_pane.min(panes.saturating_sub(1));
                std::mem::swap(&mut self.view.digi_pane, &mut self.panel_window_pane);
                let child = ChildWindow::panes(self);
                let outcome = detached_viewport(ctx, &spec, seed, child, |ui| {
                    // **The same title bar as the float, on the panel's own
                    // background.** A tiling window manager draws no title bar,
                    // so the window's content carries one: 14 pt, the panel's
                    // name at 10 pt, and the one glyph a window of its own needs —
                    // dock — at 12 pt. It used to be a bare chip row over the
                    // window's default black, which read as two panels stacked.
                    egui::Frame::new()
                        .fill(crate::theme::PANEL())
                        // Six rather than four: the bar's buttons are 14 pt in a
                        // 16 pt row, so the frame's own margin is what stands
                        // between them and the window's top edge.
                        .inner_margin(egui::Margin::symmetric(6, 6))
                        .show(ui, |ui| {
                            ui.set_min_size(ui.available_size());
                            ui.allocate_ui_with_layout(
                                egui::vec2(ui.available_width(), crate::chrome::BAR_H),
                                egui::Layout::left_to_right(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        RichText::new(mode.label())
                                            .size(crate::chrome::BAR_FONT)
                                            .color(crate::theme::CYAN()),
                                    );
                                    ui.with_layout(
                                        egui::Layout::right_to_left(egui::Align::Center),
                                        |ui| {
                                            dock = crate::chrome::chip_window_icon_sized(
                                                ui,
                                                crate::chrome::WindowIcon::Dock,
                                                egui::vec2(
                                                    crate::chrome::BAR_CHIP,
                                                    crate::chrome::BAR_CHIP,
                                                ),
                                            )
                                            .on_hover_text(tr!("frame-dock-panel-hint"))
                                            .clicked();
                                        },
                                    );
                                },
                            );
                            ui.separator();
                            // Narrow enough for one pane at a time: the chips that
                            // reach the others are drawn **here**, because the main
                            // window's copy of them is behind this window and its
                            // waterfall tab has nothing to show in a panel window.
                            if crate::layout::tier(ui.ctx()) == crate::layout::Tier::Phone {
                                self.digi_tabs(ui, mode, false);
                            }
                            let h = ui.available_height();
                            self.draw_operating_panel(ui, &mut cmds, mode, h);
                        });
                });
                std::mem::swap(&mut self.view.digi_pane, &mut self.panel_window_pane);
                self.handle_detached_outcome(ctx, module, &spec, outcome);
                if dock {
                    self.ui_settings.set_detached(sdroxide_types::DetachableModule::Panel, false);
                    self.panel_floating = false;
                    crate::app::persist::persist_ui_settings(&self.ui_settings);
                }
                self.dispatch_commands(cmds);
            }
            M::Controls => {
                let child = ChildWindow::desk(self);
                let outcome = detached_viewport(ctx, &spec, seed, child, |ui| {
                    // The console is **desktop-shaped whatever its own height**:
                    // the compact strips are for a phone, and a control window
                    // that flipped to one the moment it opened was the first
                    // thing the operator noticed. That is `ChildWindow::desk`,
                    // which publishes the tier for this window alone and puts
                    // the main window's back when the draw ends — the old way
                    // saved and restored it by hand here, and left every other
                    // answer (the tight flag, the dock's room, the salt) saying
                    // what the main window had said.
                    egui::Frame::new()
                        .fill(crate::theme::BG_DEEP())
                        .inner_margin(egui::Margin::symmetric(8, 6))
                        .show(ui, |ui| {
                            // The strip on top, then the band keypad beside the
                            // band/mode selector — the operator's "controls and
                            // bands in one window", SDRuno's RX control with the
                            // keypad down its left side. Wide enough for the two
                            // to sit side by side; narrower than that they stack,
                            // because a band list squeezed into a column too thin
                            // to draw its own pad is a worse way to reach every
                            // band than the keypad is.
                            crate::chrome::angled_frame(ui, crate::theme::PINK(), |ui| {
                                self.top_bar(ui, &mut cmds);
                            });
                            ui.separator();
                            self.console_band_area(ui, &mut cmds);
                        });
                });
                self.handle_detached_outcome(ctx, module, &spec, outcome);
                self.dispatch_commands(cmds);
            }
            M::BandMenu => {
                let child = ChildWindow::desk(self);
                let outcome = detached_viewport(ctx, &spec, seed, child, |ui| {
                    // Desktop-shaped whatever its height, like the console: the
                    // window is a desktop surface, and the compact strips are
                    // for a phone.
                    egui::Frame::new()
                        .fill(crate::theme::BG_DEEP())
                        .inner_margin(egui::Margin::symmetric(8, 6))
                        .show(ui, |ui| {
                            // A header with its own DOCK chip, like SP1's
                            // toolbar: bring the selector home without hunting
                            // for the window's close box.
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(tr!("topbar-band-mode"))
                                        .size(11.0)
                                        .strong()
                                        .color(crate::theme::CYAN()),
                                );
                                crate::chrome::row_tail(ui, |ui| {
                                    if crate::chrome::chip_window_icon(
                                        ui,
                                        crate::chrome::WindowIcon::In,
                                    )
                                    .on_hover_text(tr!("frame-dock-band-menu-hint"))
                                    .clicked()
                                    {
                                        self.ui_settings.set_detached(
                                            sdroxide_types::DetachableModule::BandMenu,
                                            false,
                                        );
                                        crate::app::persist::persist_ui_settings(&self.ui_settings);
                                    }
                                });
                            });
                            ui.separator();
                            // The same body the docked column draws, so the
                            // window and the dock cannot come apart — and at
                            // this width `mode_chip_grid` lays the modes in five
                            // columns, which the docked column never could.
                            egui::ScrollArea::vertical()
                                .auto_shrink([false, false])
                                .id_salt("band-window-scroll")
                                .show(ui, |ui| {
                                    self.band_menu_body(ui, &mut cmds);
                                });
                        });
                });
                self.handle_detached_outcome(ctx, module, &spec, outcome);
                self.dispatch_commands(cmds);
            }
            M::Rotor => {
                let child = ChildWindow::desk(self);
                let outcome = detached_viewport(ctx, &spec, seed, child, |ui| {
                    // Desktop-shaped whatever its height, like the console and
                    // the band selector: this is a desk tool, and the compact
                    // strips are for a phone.
                    egui::Frame::new()
                        .fill(crate::theme::BG_DEEP())
                        .inner_margin(egui::Margin::symmetric(8, 6))
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    RichText::new(tr!("frame-antenna-rotator-caption"))
                                        .size(11.0)
                                        .strong()
                                        .color(crate::theme::CYAN()),
                                );
                                crate::chrome::row_tail(ui, |ui| {
                                    if crate::chrome::chip_window_icon(
                                        ui,
                                        crate::chrome::WindowIcon::In,
                                    )
                                    .on_hover_text(tr!("frame-dock-rotator-hint"))
                                    .clicked()
                                    {
                                        self.ui_settings.set_detached(
                                            sdroxide_types::DetachableModule::Rotor,
                                            false,
                                        );
                                        crate::app::persist::persist_ui_settings(&self.ui_settings);
                                    }
                                });
                            });
                            ui.separator();
                            self.rotor_body(ui, &mut cmds);
                        });
                });
                self.handle_detached_outcome(ctx, module, &spec, outcome);
                self.dispatch_commands(cmds);
            }
        }
    }

    /// Fold a detached window's outcome into the settings: remember its geometry
    /// once a drag settles, and dock it again when the operator closes it. The
    /// one place both modules' outcomes are handled, so their behaviour cannot
    /// drift apart.
    #[cfg(not(target_arch = "wasm32"))]
    fn handle_detached_outcome(
        &mut self,
        ctx: &egui::Context,
        module: sdroxide_types::DetachableModule,
        spec: &DetachedWindowSpec,
        outcome: DetachedOutcome,
    ) {
        let mut persist = false;
        {
            let state = &mut self.ui_settings.detached[module.index()];
            // Persist once a drag settles — the frame after the geometry stops
            // changing — so a resize in progress is not a config write a frame,
            // while the value that stays on screen is the one that is written.
            if let Some(g) = outcome.geometry {
                let last: Option<sdroxide_types::DetachedWindow> =
                    ctx.data(|d| d.get_temp(spec.settle_id));
                if last == Some(g) && state.window != Some(g) {
                    state.window = Some(g);
                    persist = true;
                }
                ctx.data_mut(|d| d.insert_temp(spec.settle_id, g));
            }
            if outcome.close_requested {
                // Closing the window is docking it again, not an invitation to
                // re-open it next frame — and the frame it closed on still
                // carries the geometry to keep.
                if let Some(g) = outcome.geometry {
                    state.window = Some(g);
                }
                state.detached = false;
                persist = true;
            }
        }
        if persist {
            crate::app::persist::persist_ui_settings(&self.ui_settings);
        }
    }

    /// The mode's operating panel, drawn into `ui` — the **shared body** of the
    /// in-window panel and its undocked window, so the two cannot differ.
    /// `mode` picks the CW keyboard from the digital operating panel.
    fn draw_operating_panel(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        mode: Mode,
        panel_h: f32,
    ) {
        // The operating panels, pulled in on a small screen — one call for all
        // of them, see `layout::tighten`.
        crate::layout::tighten(ui);
        egui::Frame::new()
            .fill(crate::theme::BG_DEEP())
            .inner_margin(egui::Margin { left: 0, right: 0, top: 6, bottom: 0 })
            .show(ui, |ui| {
                crate::chrome::angled_frame(ui, crate::theme::PINK(), |ui| {
                    if mode == Mode::Cw {
                        self.cw_panel(ui, cmds, panel_h);
                    } else {
                        self.operating_panel(ui, cmds, mode, panel_h);
                    }
                });
            });
    }

    /// **Float the operating pane inside the main window**, and let the waterfall
    /// take the room back.
    ///
    /// **One flag for every mode.** The operator's correction on the first version:
    /// *"you open a new window whereas it should be a floating panel in the first
    /// place"* — and then *"the chip in the panel in main window does not work"*,
    /// which it did not on any mode without a tool window of its own (FT8 among
    /// them). This is the panel itself, drawn into a floating window, so it works
    /// in every mode and there is no table to keep in step.
    pub(in crate::app) fn float_operating_panel(&mut self) {
        self.panel_floating = true;
        // Lifting the panel out is asking to *see* it: a float remembered as
        // rolled up from last time would hand back a title bar and nothing else.
        self.panel_rolled = false;
    }

    /// The operating pane as a **floating panel inside this window**.
    ///
    /// One `egui::Window` whose body is the same `draw_operating_panel` the docked
    /// pane uses, so every mode gets it with no table of flags to keep in step —
    /// which is what went wrong with the first attempt: it opened the mode's own
    /// tool window, and a mode without one (FT8 among them) got nothing at all.
    ///
    /// **Its title bar is ours**, and it carries the three things a window has:
    /// roll up, out to a window of its own, and close — which here means *dock*,
    /// because a panel is not a tool that can be shut, it is the mode's control
    /// surface. egui draws no hook for extra title-bar buttons, so the bar is
    /// drawn by us and `title_bar(false)` hands the movement to egui's
    /// drag-anywhere fallback; the bar's own empty ground drags the window.
    ///
    /// **Out is a window of its own**, not a state: the operator's tiling window
    /// manager has no title bar to put the control in, so every window carries
    /// its own dock picture in its content — but the *float* needs no picture of
    /// its own, because the title bar is exactly the thing the OS window will not
    /// have.
    fn floating_panel_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>, mode: Mode) {
        let title = tr!("frame-panel-window-title", mode = mode.label());
        let mut open = true;
        // **No dud in the browser.** Detaching is native-only, so an out button
        // there would dock the panel instead of giving it a window — a control
        // doing the wrong thing, which is worse than one that is absent.
        #[cfg(not(target_arch = "wasm32"))]
        let mut out = false;
        #[cfg(target_arch = "wasm32")]
        let out = false;
        let mut rolled = self.panel_rolled;
        // The floating panel is a panel window too, and gets the same bargain as
        // the detached one: its own pane, swapped in around the draw, clamped to
        // a real pane so the waterfall tab the main window may have selected
        // cannot ask it for a picture it has not got.
        let panes = super::panels::panel_panes(mode).len();
        self.panel_window_pane = self.panel_window_pane.min(panes.saturating_sub(1));
        std::mem::swap(&mut self.view.digi_pane, &mut self.panel_window_pane);
        egui::Window::new(title)
            .id(egui::Id::new("floating-operating-panel"))
            .title_bar(false)
            .default_size([540.0, 440.0])
            .min_width(320.0)
            .min_height(0.0)
            .max_size(ctx.content_rect().size() * 0.95)
            .resizable(true)
            .show(ctx, |ui| {
                // A floating panel is a window inside this one, and answers for
                // its own size like any other — see `layout::ViewportScope`. It
                // is where the fault showed first: at 540 pt it laid itself out
                // desktop-wide, so its chip rows ran past its right edge.
                let _scope = ChildWindow::panes(self).scope(ui);
                // **The title bar: the name, then the three buttons**, drawn
                // right to left so the order reads left-to-right as
                // out · roll-up · close, which is what every desktop does. The
                // title is a noun and the glyphs are verbs, which is the whole
                // grammar this pass is about.
                // **The bar's own numbers**: 14 pt tall, 12 pt buttons, 10 pt
                // name — see `chrome::BAR_H`. It is a title bar, not a strip of
                // controls, and a portrait of the window should be as quiet as
                // the one on any other window.
                let btn = egui::vec2(crate::chrome::BAR_CHIP, crate::chrome::BAR_CHIP);
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width(), crate::chrome::BAR_H),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        ui.label(
                            RichText::new(mode.label())
                                .size(crate::chrome::BAR_FONT)
                                .color(crate::theme::CYAN()),
                        );
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            // **One family, one size, one stroke.** All three are the
                            // drawn window pictures, in chips of the same geometry —
                            // a painted arrow beside a font `▾` and a font `×` was
                            // three weights and three sizes on one row.
                            if crate::chrome::chip_window_icon_sized(
                                ui,
                                crate::chrome::WindowIcon::Close,
                                btn,
                            )
                            .on_hover_text(tr!("frame-dock-panel-hint"))
                            .clicked()
                            {
                                open = false;
                            }
                            let roll_icon = if rolled {
                                crate::chrome::WindowIcon::Unfold
                            } else {
                                crate::chrome::WindowIcon::Collapse
                            };
                            if crate::chrome::chip_window_icon_sized(ui, roll_icon, btn)
                                .on_hover_text(if rolled {
                                    tr!("frame-unroll-hint")
                                } else {
                                    tr!("frame-roll-up-hint")
                                })
                                .clicked()
                            {
                                rolled = !rolled;
                            }
                            // **Out to a window of its own** — the one button a
                            // floating window has that a docked panel does not,
                            // and one a browser cannot honour: detaching is
                            // native-only, so it is not drawn there rather than
                            // drawn dead.
                            #[cfg(not(target_arch = "wasm32"))]
                            {
                                out = crate::chrome::chip_window_icon_sized(
                                    ui,
                                    crate::chrome::WindowIcon::Out,
                                    btn,
                                )
                                .on_hover_text(tr!("frame-window-panel-hint"))
                                .clicked();
                            }
                        });
                    },
                );
                ui.separator();
                if !rolled {
                    // One pane at a time in a narrow floating panel: the chips
                    // that reach the others belong to the window that is in front.
                    if crate::layout::tier(ui.ctx()) == crate::layout::Tier::Phone {
                        self.digi_tabs(ui, mode, false);
                    }
                    let h = ui.available_height();
                    self.draw_operating_panel(ui, cmds, mode, h);
                }
            });
        std::mem::swap(&mut self.view.digi_pane, &mut self.panel_window_pane);
        self.panel_rolled = rolled;
        if !open {
            self.panel_floating = false;
        }
        if out {
            // One at a time: the OS window takes over from the floating panel, and
            // the column keeps the room the pane gave up.
            self.panel_floating = false;
            self.detach_operating_panel();
        }
    }

    /// **Send the operating pane into an OS window of its own** — the detachable
    /// module, kept for the Settings page and for a panel that wants another
    /// monitor.
    ///
    /// Called by the window icon at the right-hand end of a panel's own header row
    /// (see [`crate::app::panels::window_chip`]). The control lives in the header
    /// rather than above the pane because there is no room above it: a row there
    /// was tried first and took a chip off WEFAX's header at 600 pt, which
    /// `the_fax_header_row_does_not_draw_on_itself` caught. In the header it costs
    /// nothing — the chip it replaces spelled a word.
    pub(in crate::app) fn detach_operating_panel(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.ui_settings.set_detached(sdroxide_types::DetachableModule::Panel, true);
            crate::app::persist::persist_ui_settings(&self.ui_settings);
        }
        // The browser has no second window to send a pane to, so it has no icon
        // to click; this is here so a panel that draws one anyway does nothing
        // surprising.
        #[cfg(target_arch = "wasm32")]
        let _ = &self.ui_settings;
    }

    /// Draw a **tool window** — the scanner, the schedule, the logbook and the
    /// rest — either as an egui window in this viewport (docked, the default) or
    /// as its own OS window (undocked). Returns whether it is still open, so the
    /// caller stores that back in its `show_*` flag.
    ///
    /// The undocked flag and the remembered geometry live in
    /// [`SdroxideApp::tool_windows`], keyed by `id`. **Session-only**, like the
    /// in-viewport positions egui keeps for the same windows: a tool is
    /// transient, and where it sat last run is not worth a config field per tool.
    ///
    /// `body` receives `self` rather than capturing it, so this can be a method —
    /// a closure that captured `self` could not also be called from a `&mut self`
    /// method.
    pub(in crate::app) fn tool_window(
        &mut self,
        ctx: &egui::Context,
        id: &'static str,
        title: &str,
        default_size: [f32; 2],
        open: bool,
        body: impl FnOnce(&mut Self, &mut egui::Ui),
    ) -> bool {
        self.tool_window_sized(ctx, id, title, default_size, [0.0, 0.0], false, open, body)
    }

    /// As [`Self::tool_window`], with a floor on how small the docked window may
    /// be dragged, and whether to open in the middle of the screen the first
    /// time rather than in a corner.
    ///
    /// Both exist for the settings box and nothing else: its tabs are laid out
    /// at fixed widths, so it becomes a column of clipped controls if it is
    /// shrunk much; and at 900x760 a corner is under the top bar, which is where
    /// the pinned [`egui::Window`] it used to be deliberately put it in the
    /// middle instead. egui remembers the position either way, so this only
    /// decides where a window lands the first time it is opened.
    pub(in crate::app) fn tool_window_sized(
        &mut self,
        ctx: &egui::Context,
        id: &'static str,
        title: &str,
        default_size: [f32; 2],
        min_size: [f32; 2],
        centre_first: bool,
        open: bool,
        body: impl FnOnce(&mut Self, &mut egui::Ui),
    ) -> bool {
        if !open {
            return false;
        }
        self.dispatch_tool_window(ctx, id, title, default_size, min_size, centre_first, body)
    }

    /// The tool as an egui window in this viewport. Returns
    /// `(still_open, detach_clicked)`.
    fn tool_window_egui(
        &mut self,
        ctx: &egui::Context,
        id: &'static str,
        title: &str,
        default_size: [f32; 2],
        min_size: [f32; 2],
        centre_first: bool,
        body: impl FnOnce(&mut Self, &mut egui::Ui),
    ) -> (bool, bool) {
        let mut win_open = true;
        let place = self.tool_place.take();
        let mut win_id = crate::layout::salted_id(ctx, id).with(TOOL_WINDOW_EPOCH);
        if let Some(p) = place {
            win_id = win_id.with(p.epoch);
        }
        #[cfg_attr(target_arch = "wasm32", allow(unused))]
        let mut body_top = f32::NAN;
        let mut w = egui::Window::new(title)
            // The id carries an **epoch**, bumped when this window's bounding
            // changes; egui remembers a window's size and eframe persists it, so
            // the epoch discards a stale one.
            .id(win_id)
            .open(&mut win_open)
            .frame(crate::chrome::window_frame())
            .resizable(true)
            .default_size(egui::vec2(
                crate::layout::window_w(ctx, default_size[0]),
                crate::layout::window_h(ctx, default_size[1]),
            ))
            // A cap so no remembered — or future — size can take the whole
            // screen. The body sits at the top now (the ⇱ chip is a one-row
            // allocation), so a large window is merely large, not broken; this
            // just keeps it sane.
            .max_size(ctx.content_rect().size() * 0.95);
        if let Some(p) = place {
            w = w.default_pos(p.pos).default_size(p.size);
        }
        if min_size[0] > 0.0 {
            w = w
                .min_width(crate::layout::window_w(ctx, min_size[0]))
                .min_height(crate::layout::window_h(ctx, min_size[1]));
        }
        // First open only — egui remembers a window's position, so this decides
        // where a window lands once and never again.
        if centre_first {
            let want = egui::vec2(
                crate::layout::window_w(ctx, default_size[0]),
                crate::layout::window_h(ctx, default_size[1]),
            );
            w = w.default_pos(ctx.content_rect().center() - want * 0.5);
        }
        let resp = w.show(ctx, |ui| {
            crate::chrome::window_body_bg(ui);
            #[cfg(not(target_arch = "wasm32"))]
            {
                body_top = ui.min_rect().min.y;
            }
            body(self, ui);
        });
        // Only ever set on the native path below: the ⇱ chip rides the title
        // bar, and the browser has no second window to send it to. Same shape
        // as `body_top` above.
        #[cfg_attr(target_arch = "wasm32", allow(unused))]
        let mut detach = false;
        if let Some(r) = &resp {
            self.tool_rect = Some(r.response.rect);
            crate::chrome::paint_window_border(ctx, &r.response);
            #[cfg(not(target_arch = "wasm32"))]
            {
                detach = tool_window_title_button(ctx, win_id, r.response.rect, body_top);
            }
        }
        (win_open, detach)
    }

    #[cfg(target_arch = "wasm32")]
    fn dispatch_tool_window(
        &mut self,
        ctx: &egui::Context,
        id: &'static str,
        title: &str,
        default_size: [f32; 2],
        min_size: [f32; 2],
        centre_first: bool,
        body: impl FnOnce(&mut Self, &mut egui::Ui),
    ) -> bool {
        // No second window in the browser: always the in-viewport window.
        self.tool_window_egui(ctx, id, title, default_size, min_size, centre_first, body).0
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn dispatch_tool_window(
        &mut self,
        ctx: &egui::Context,
        id: &'static str,
        title: &str,
        default_size: [f32; 2],
        min_size: [f32; 2],
        centre_first: bool,
        body: impl FnOnce(&mut Self, &mut egui::Ui),
    ) -> bool {
        let st = self.tool_windows.get(id).copied().unwrap_or_default();
        if st.undocked {
            self.tool_window_os(ctx, id, title, default_size, st.window, body)
        } else {
            let (open, detach) =
                self.tool_window_egui(ctx, id, title, default_size, min_size, centre_first, body);
            if detach {
                self.tool_windows.insert(id, ToolWindowState { undocked: true, window: st.window });
            }
            open
        }
    }

    /// The tool in its own OS window.
    #[cfg(not(target_arch = "wasm32"))]
    fn tool_window_os(
        &mut self,
        ctx: &egui::Context,
        id: &'static str,
        title: &str,
        default_size: [f32; 2],
        seed: Option<sdroxide_types::DetachedWindow>,
        body: impl FnOnce(&mut Self, &mut egui::Ui),
    ) -> bool {
        let spec = tool_spec(id, title, default_size);
        let mut dock = false;
        let child = ChildWindow::own(self);
        let outcome = detached_viewport(ctx, &spec, seed, child, |ui| {
            // A tool's own window gets the panel background the in-window one
            // has from its `egui::Window` frame — the module windows paint their
            // own, a tool body assumes one.
            egui::Frame::new()
                .fill(crate::theme::BG_DEEP())
                .inner_margin(egui::Margin::symmetric(10, 8))
                .show(ui, |ui| {
                    ui.set_min_size(ui.available_size());
                    dock = tool_window_chip(ui, true);
                    body(self, ui);
                });
        });
        // Settle-persist the geometry as the modules do, and treat a close as
        // *closed*, not docked: the operator shut the tool, so it should not
        // reappear in the main window.
        let st = self.tool_windows.get(id).copied().unwrap_or_default();
        let mut window = st.window;
        if let Some(g) = outcome.geometry {
            let last: Option<sdroxide_types::DetachedWindow> =
                ctx.data(|d| d.get_temp(spec.settle_id));
            if last == Some(g) || outcome.close_requested {
                window = Some(g);
            }
            ctx.data_mut(|d| d.insert_temp(spec.settle_id, g));
        }
        let open = !outcome.close_requested;
        let undocked = open && !dock;
        self.tool_windows.insert(id, ToolWindowState { undocked, window });
        open
    }

    /// Send a frame's commands, with the bookkeeping that must ride every route
    /// a command can leave by: a stop control disarms auto mode, and a login
    /// Test is marked in flight the moment it goes out. Split out so an
    /// undocked window's own commands are dispatched exactly as the main
    /// window's — there is one place that decides, not two that can drift.
    pub(in crate::app) fn dispatch_commands(&mut self, cmds: Vec<Command>) {
        // Any stop control — STOP QSO, STOP TX, a bound Abort TX — disarms auto
        // mode. An unattended run must never be left sequencing after the
        // operator has told the radio to stop, whatever route they used.
        if self.auto_role.is_some()
            && cmds.iter().any(|c| matches!(c, Command::DigiStopQso | Command::DigiAbortTx))
        {
            self.disarm_auto(tr!("frame-auto-stopped-stop-control"));
        }
        for c in cmds {
            // PTT or TUNE asked for where TX is off — a key, a MIDI button, any
            // route: it never leaves, and the operator is told in red, every
            // time (the engine would refuse it too).
            if matches!(c, Command::SetPtt(true) | Command::SetTune(true))
                && !self.tx_keyable_here()
            {
                self.tx_engine_refusal = None;
                self.tx_press_refused_at = Some(crate::time::now_unix_f64());
                continue;
            }
            // Marked here rather than at the button, because the settings panel
            // holds borrows of `self` while it draws and cannot take a mutable
            // one. This is the point where the command is definitely going out,
            // which is the honest moment to call the check "in flight" anyway.
            if let Command::TestLogin(t) = &c {
                self.login_tests_pending.insert(*t);
                self.login_tests.remove(t);
            }
            if let Command::TestPictureUpload(_) = &c {
                self.picup_test_pending = Some(crate::time::now_unix_f64());
                self.picup_test = None;
            }
            self.dial_hold.note(&c);
            self.ctrl.send(c);
        }
    }

    /// Whether the focused radio should draw its operating panel into its own
    /// window this frame. Only a mode that *has* a panel can — a digital mode's
    /// operating panel, or CW's keyboard — so a voice mode ignores the setting;
    /// the preference stays and applies again in a mode with a panel.
    fn panel_detached(&self, mode: Mode) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = mode;
            false
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            (self.ui_settings.is_detached(sdroxide_types::DetachableModule::Panel))
                && self.focused
                && (mode.has_bottom_panel() || mode == Mode::Cw)
        }
    }

    /// Whether the focused radio draws its **control strip** in its own window
    /// this frame. Every mode has one, so the only gates are the browser and the
    /// focus, exactly as for the panadapter.
    fn controls_detached(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            false
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.ui_settings.is_detached(sdroxide_types::DetachableModule::Controls) && self.focused
        }
    }

    /// **Classic UI** is in use *and* the column is drawn this frame — which the
    /// strip has to know, because wherever the column is, the strip is the
    /// readout and the meter and nothing else; the column carries every control.
    /// One place answers it and that place is where the column is drawn: the
    /// answer is **published** (see [`crate::layout::set_classic_column`]) rather
    /// than re-derived here, because the strip sees the width left over *after*
    /// the column took its share and would conclude that a window which has a
    /// column has no room for one.
    ///
    /// A child window publishes `false` for its own draw, so the RX control
    /// console — a strip in a window, with no column behind it — keeps its
    /// controls. The column is native only, so the browser answers `false`.
    pub(in crate::app) fn classic_column_this_frame(&self, ctx: &egui::Context) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            let _ = ctx;
            false
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.ui_settings.classic_ui && crate::layout::classic_column(ctx)
        }
    }

    /// The panadapter's window should exist this frame: undocked on the focused
    /// radio *and* there is a panadapter to show (its layers are on). This is the
    /// same rule the frame loop reserves space by, so the two cannot disagree.
    fn panadapter_window_wanted(&self) -> bool {
        #[cfg(target_arch = "wasm32")]
        {
            false
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.module_window_wanted(sdroxide_types::DetachableModule::Panadapter)
        }
    }

    /// The same rule for any spectrum module: undocked on the focused radio
    /// **and** there is a panadapter to show (its layers are on). One predicate
    /// for both SP1 and AUX SP, so the two cannot disagree about when a spectrum
    /// window exists — a second one that appeared without its layers would be an
    /// empty window on a monitor.
    #[cfg(not(target_arch = "wasm32"))]
    fn module_window_wanted(&self, module: sdroxide_types::DetachableModule) -> bool {
        self.ui_settings.is_detached(module) && self.focused && self.view.panadapter_visible()
    }

    /// The operating-panel window should exist this frame: undocked on the
    /// focused radio **and** the mode has a panel to show. A mode with none —
    /// a voice mode — gets no window at all; the operator is told why by the
    /// notice banner rather than by an empty window sitting on a monitor.
    #[cfg(not(target_arch = "wasm32"))]
    fn panel_window_wanted(&self, mode: Mode) -> bool {
        self.panel_detached(mode)
    }

    /// The pane is not in the column this frame: either it is an OS window
    /// (`DetachableModule::Panel`) or it has been lifted out as a **floating
    /// panel** in this window. Either way the waterfall takes the room back.
    pub(in crate::app) fn panel_in_a_window(&self, mode: Mode) -> bool {
        self.panel_detached(mode)
            || (self.panel_floating && (mode.has_bottom_panel() || mode == Mode::Cw))
    }

    /// The band/mode selector's window should exist this frame: undocked on the
    /// focused radio. Unlike the spectrum modules it needs nothing else to be
    /// present — it is a selector, always meaningful, and the operator opened it
    /// deliberately. Every mode has a band and a mode, so there is never an empty
    /// window to explain.
    #[cfg(not(target_arch = "wasm32"))]
    fn band_menu_detached(&self) -> bool {
        self.ui_settings.is_detached(sdroxide_types::DetachableModule::BandMenu) && self.focused
    }

    /// The rotator window should exist this frame: undocked on the focused
    /// radio. Like the band selector it needs nothing else to be present — the
    /// compass is meaningful whether or not a rotator is configured, and when
    /// none is, the window says so rather than disappearing.
    #[cfg(not(target_arch = "wasm32"))]
    fn rotor_detached(&self) -> bool {
        self.ui_settings.is_detached(sdroxide_types::DetachableModule::Rotor) && self.focused
    }

    /// Which of this radio's modules the shell should emit as their own windows
    /// this frame, by [`sdroxide_types::DetachableModule::index`].
    ///
    /// The **app** decides — it knows the layers, the mode and the focus, and
    /// the shell does not — and the shell emits the true ones, once, for the
    /// focused radio, every frame. That is what keeps an undocked window from
    /// being torn down and re-mapped when the radio or the mode changes under
    /// it: the shell always asks, and there is always an answer.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn detached_wanted(&self) -> [bool; sdroxide_types::DetachableModule::COUNT] {
        let mut want = [false; sdroxide_types::DetachableModule::COUNT];
        want[sdroxide_types::DetachableModule::Panadapter.index()] =
            self.panadapter_window_wanted();
        want[sdroxide_types::DetachableModule::Panel.index()] =
            self.panel_window_wanted(self.state.rx[0].mode);
        want[sdroxide_types::DetachableModule::Controls.index()] = self.controls_detached();
        want[sdroxide_types::DetachableModule::AuxPanadapter.index()] =
            self.module_window_wanted(sdroxide_types::DetachableModule::AuxPanadapter);
        want[sdroxide_types::DetachableModule::BandMenu.index()] = self.band_menu_detached();
        want[sdroxide_types::DetachableModule::Rotor.index()] = self.rotor_detached();
        want
    }

    /// Whether anything of this radio is drawn in the main window's centre this
    /// frame — the panadapter in-window, or the mode's panel. When neither is,
    /// the space is filled with the band/mode selector rather than left black
    /// (the operator's screenshots of an undocked panadapter and panel).
    pub(in crate::app) fn center_has_content(&self, mode: Mode) -> bool {
        let layers = self.view.panadapter_visible();
        let pan_here = layers && !self.panadapter_window_wanted();
        let panel_here =
            (mode.has_bottom_panel() || mode == Mode::Cw) && !self.panel_detached(mode);
        pan_here || panel_here
    }

    /// Advance a DAB scan, if one is running.
    ///
    /// A scan is an act of the operator's, so it lives in the app rather than
    /// the engine: whichever pane is on screen, the loop runs, retunes the lane
    /// block by block, and records what it heard. Selecting DAB is not required
    /// — an operator may scan before switching to the mode — but the lane only
    /// runs in the mode, so a scan started from elsewhere waits for it.
    pub(in crate::app) fn tick_dab_scan(
        &mut self,
        ctx: &egui::Context,
        now: f64,
        cmds: &mut Vec<Command>,
    ) {
        let Some(scan) = self.dab_scan.as_mut() else { return };
        // Ask for frames while it runs: a scan must reach its next block even
        // if nothing on screen is moving.
        crate::repaint::after_ms(ctx, 100);

        // A block whose status names stations joins the list, and its stations
        // are remembered with it there and then — so a sweep stopped half way,
        // or a restart, keeps every station it heard. Only a status that is
        // about *this* block counts: the one before can still be arriving for a
        // moment after the retune, and must not be credited here.
        let name = sdroxide_types::DAB_BAND_III[scan.at].0.to_string();
        let mut heard_stations = false;
        let mut multiplex = false;
        if let Some(st) = self.dab_status.as_ref() {
            if let Some(services) = crate::app::panels::dab::dab_scan_heard(st, &name) {
                heard_stations = true;
                if !scan.found.contains(&name) {
                    scan.found.push(name.clone());
                }
                self.state.dab.remember_stations(&name, services);
            } else if crate::app::panels::dab::dab_status_is_for(st, &name) {
                multiplex = crate::app::panels::dab::dab_block_has_multiplex(st);
            }
        }

        // An ensemble named but no stations yet: give the FIC a little longer.
        let dwell =
            if multiplex && !heard_stations { DAB_SCAN_DWELL_S * 2.0 } else { DAB_SCAN_DWELL_S };
        if now - scan.since.unwrap_or(now) < dwell {
            if scan.since.is_none() {
                scan.since = Some(now);
            }
            return;
        }
        // Move on. Past the last block the scan is over: keep what was found
        // and hand it to the settings, which the server remembers.
        scan.at += 1;
        scan.since = Some(now);
        if scan.at >= sdroxide_types::DAB_BAND_III.len() {
            let found = std::mem::take(&mut scan.found);
            self.state.dab.apply_scan(sdroxide_types::DAB_BAND_III.len(), &found);
            self.dab_scan = None;
            cmds.push(Command::SetDabConfig(self.state.dab.clone()));
            return;
        }
        let (name, hz) = sdroxide_types::DAB_BAND_III[scan.at];
        self.state.dab.channel = name.to_string();
        cmds.push(Command::SetVfo { vfo: self.state.active_vfo, hz });
        cmds.push(Command::SetDabConfig(self.state.dab.clone()));
    }

    /// Drain everything this radio's engine sent since the last visit.
    ///
    /// Runs at the top of every drawn frame — and, in a multi-radio session,
    /// once per frame for every *hidden* tab too (from
    /// [`crate::multi::MultiApp`]), so a background radio keeps decoding,
    /// logging and reconnecting while another tab is up front. The unbounded
    /// event channel must never be left to back up.
    /// Restart the slot bar's follower when the radio changes *to* FT8, FT4 or
    /// FT2 from some other mode: the first frame is joined part-way and nothing
    /// says where it began, so the bar sweeps from nothing until the first end
    /// of a frame or the first decode places it. A change away from them just
    /// forgets what was heard.
    fn track_slot_mode(&mut self) {
        let mode = self.state.rx[0].mode;
        if self.slot_follow_mode == Some(mode) {
            return;
        }
        self.slot_follow_mode = Some(mode);
        match (matches!(mode, Mode::Ft8 | Mode::Ft4 | Mode::Ft2), self.slot_timing()) {
            (true, Some(t)) => self.slot_follow.acquire(&t, crate::time::now_unix_f64()),
            _ => self.slot_follow.reset(),
        }
    }

    /// Give the slot bar the frame a batch of decodes came from: its start is
    /// the slot's boundary, the mode's nominal offset and the DT the stations
    /// showed (the middle one, so one odd clock does not move it). A decode
    /// does not depend on level, so it places the bar where QRM leaves the gate
    /// nothing to find.
    fn anchor_slot_follow(&mut self, decodes: &[sdroxide_types::Decode]) {
        if !matches!(self.state.rx[0].mode, Mode::Ft8 | Mode::Ft4 | Mode::Ft2) {
            return;
        }
        let (Some(t), Some(newest)) =
            (self.slot_timing(), decodes.iter().map(|d| d.slot_utc).max())
        else {
            return;
        };
        let mut dts: Vec<f32> =
            decodes.iter().filter(|d| d.slot_utc == newest).map(|d| d.dt).collect();
        dts.sort_by(|a, b| a.total_cmp(b));
        let dt = f64::from(dts[dts.len() / 2]);
        self.slot_follow.anchor(
            &t,
            newest as f64 + t.tx_offset_s + dt,
            crate::time::now_unix_f64(),
        );
    }

    pub(crate) fn drain_events(&mut self, ctx: &egui::Context, now: f64) {
        // The settings row's explicit "save to profile" / "back to profile",
        // for the same reason as the answers below: the dialog is drawn from
        // `&self`, so what it asks for is left here and done where the app can
        // be borrowed.
        if let Some(action) = self.client_settings_pending.take() {
            match action {
                crate::app::settings::ui_tab::ProfileAction::Save => {
                    self.save_screen_to_profile();
                }
                crate::app::settings::ui_tab::ProfileAction::Revert => {
                    self.revert_screen_to_profile();
                }
            }
        }
        // And the automatic half: with a signed-in profile, the screen keeps
        // itself in step. See `auto_save_screen`.
        self.auto_save_screen(now);
        // And the "do not ask me again" answers, kept on the server.
        self.sync_client_acks();
        // Answers to the settings dialog's device questions. Drained here
        // rather than in the dialog: they come from another machine, so one can
        // land in the frame after it was closed, and an answer left in the
        // queue would be applied to whatever the dialog was asking next time.
        while let Some(answer) = self.ctrl.poll_probe() {
            self.apply_probe_answer(ctx, answer);
        }
        // Whether a panadapter frame has already landed in this pass, so the
        // next one knows it is superseding a picture nothing will ever draw —
        // see the `Spectrum` arm below.
        let mut superseded = false;
        self.track_slot_mode();
        while let Some(ev) = self.ctrl.poll_event() {
            match ev {
                RadioEvent::Capabilities(c) => {
                    // The window title belongs to the focused tab; a hidden
                    // radio reconnecting must not retitle the window over it.
                    if self.focused {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Title(crate::window_title(
                            &c.label,
                        )));
                    }
                    // A different interface on this radio: the stored zoom
                    // was taken in another span and means nothing in this one.
                    if self.view.adopt_driver(&c.driver) {
                        tracing::debug!(
                            target: "sdroxide::panadapter",
                            driver = %c.driver,
                            "new front end — refitting the panadapter window"
                        );
                    }
                    // A transceiver does not start in SWL mode from the
                    // listener's "start in SWL mode" preference — that seed is
                    // for receive-only radios. Once the front end has said it
                    // can transmit, drop the seed (unless `--swl` forced it)
                    // so the LISTEN/OPERATE tab is what decides the controls.
                    let transmit_capable = c.is_transmit_capable();
                    self.caps = Some(c);
                    if super::drop_swl_seed_for_transmitter(
                        self.swl_start,
                        sdroxide_types::force_swl(),
                        transmit_capable,
                    ) {
                        self.swl_start = false;
                    }
                    // A session the far end accepted: whatever the link was
                    // doing, it is doing it again, so the next drop starts its
                    // backoff from the bottom rather than from wherever the
                    // last outage left it.
                    self.retry_backoff = super::RETRY_MIN_S;
                    // A new source may have no full-band lane at all. Without
                    // this the strip from the previous backend stays on screen
                    // for good, showing a band nothing is receiving any more.
                    self.wide_frame = None;
                    self.wide_wf.clear();
                    // This also fires on a reconnect, and pictures may have
                    // come in while the link was down. Read the stores again
                    // rather than leaving a gallery that stops at the moment
                    // the connection dropped.
                    self.sstv.listed = false;
                    self.wefax.listed = false;
                    // Same reasoning for speech: a reconnect must not make the
                    // radio recite its whole configuration.
                    self.speech.announcer.reseed();
                    self.speech.announcer.reset_decodes();
                }
                // The *same* front end, revising what it said about itself: an
                // antenna list learned from a rig that has only now answered,
                // a gain ladder as long as the band the dial has just crossed
                // into. Only the capabilities move — everything the arm above
                // throws away belongs to this radio and is still current, and
                // throwing it away on a QSY would blank the wideband strip and
                // re-read every image store for nothing.
                RadioEvent::CapabilitiesUpdated(c) => self.caps = Some(c),
                RadioEvent::State(mut s) => {
                    // The dial and centre this screen has just sent outrank a
                    // snapshot that has not caught up with them yet — or CTR
                    // recentres on the old dial for a frame and the waterfall
                    // shakes. See `dial_hold`.
                    self.dial_hold.apply(&mut s, now);
                    let prev_vfo = self.state.active_freq_hz();
                    let prev_rate = self.state.sample_rate;
                    let prev_mode = self.state.rx[0].mode;
                    let prev_band = self.state.band;
                    self.state = s;
                    let band_changed = self.state.band != prev_band;
                    if self.state.rx[0].mode != prev_mode {
                        self.clear_digi_rx();
                        // The outgoing buffer too, and only here: the engine
                        // rebuilds its controller for the new mode and its
                        // sent-character count restarts from zero, so text left
                        // over from the last mode would be redrawn as unsent and
                        // keyed again the moment transmit came on. A QSY rebuilds
                        // nothing, and takes the half-typed over with it.
                        self.text_tx.clear();
                        // The keyboard-as-straight-key toggle lives in the
                        // panel; the keyer lives in the engine. The two are
                        // kept in step only while the panel draws, so a
                        // mode change drops the latch and lets a fresh
                        // controller start unheld (issue #322).
                        self.cw_straight = false;
                        self.cw_key_down = false;
                        // Entering SSTV opens the view on the picture (and
                        // only entering it: the zoom is the operator's after).
                        if self.state.rx[0].mode.is_sstv() && !prev_mode.is_sstv() {
                            self.fit_sstv_view();
                        }
                        // The same for WEFAX: the view opens on the tuned frequency.
                        if self.state.rx[0].mode.is_wefax() && !prev_mode.is_wefax() {
                            self.fit_wefax_view();
                        }
                    } else if qsy_clears_decodes(prev_band, self.state.band, self.state.rx[0].mode)
                    {
                        self.clear_digi_band_rx();
                    }
                    self.recenter_if_tuned_away(prev_vfo, prev_rate, band_changed);
                    // The announcer diffs *this* — the engine's own snapshot —
                    // rather than `self.state` per frame, because the UI
                    // mutates that optimistically before the engine confirms
                    // and the engine has the last word on where the radio
                    // actually went. Focused tab only, here and below: two
                    // radios narrating over each other is worse than silence,
                    // and the reseed on focus gain skips the backlog.
                    if self.focused {
                        self.speech.announcer.on_state(&self.state, now);
                    }
                }
                RadioEvent::Spectrum(mut f) => {
                    // This runs once per repaint and only the frame it leaves
                    // here is drawn, so anything it replaces mid-pass is a
                    // picture nobody sees. Its *rows* are another matter: the
                    // waterfall's time axis is spaced on the assumption that
                    // every row the engine clocked reaches the texture, so
                    // dropping them makes the timestamps outrun the picture by
                    // exactly the time thrown away, and go on doing it. They
                    // ride on with the frame that superseded them instead.
                    //
                    // A client redrawing more slowly than it asked the engine
                    // to publish is the ordinary case for this, and the browser
                    // client is where it shows: its socket delivers in bursts
                    // between animation frames, so several frames routinely
                    // arrive in one pass.
                    if superseded && let Some(prev) = self.frame.as_deref() {
                        f.carry_rows_from(prev);
                    }
                    superseded = true;
                    self.frame = Some(std::sync::Arc::new(f));
                    self.last_spectrum_at = now;
                }
                // Deliberately does not touch `last_spectrum_at`: that drives
                // the "waiting for spectrum" notice, and a full-band frame is
                // not evidence that the tuned receiver is producing samples.
                RadioEvent::WideSpectrum(f) => {
                    self.wide_frame = Some(std::sync::Arc::new(f));
                }
                RadioEvent::Meters(m) => {
                    // The operator's S-meter calibration for this radio, put
                    // on here, once, so every reading of the meter — needle,
                    // readouts, a report's pre-filled level, the spoken meter —
                    // agrees with the others. Only `s_dbm` moves: the squelch
                    // and the auto-record gate read `passband_dbfs`.
                    let cal = self.radio_cfg.as_ref().map_or(0.0, |c| c.smeter_cal_db());
                    let m = m.with_smeter_cal(cal);
                    if self.focused {
                        self.speech.announcer.on_meters(&m, &self.state, now);
                    }
                    self.meters = Some(m);
                }
                RadioEvent::Memories(m) => self.memories = m,
                RadioEvent::MemoryFolders(f) => self.mem_folders = f,
                RadioEvent::Scanner(c) => self.scanner = c,
                RadioEvent::Profiles(names) => {
                    self.profiles = names;
                    if std::mem::take(&mut self.profile_apply_pending) {
                        self.digi_cfg_seeded = false;
                    }
                }
                RadioEvent::Recordings(listing) => self.playback.accept(listing),
                RadioEvent::KnownCalls(reply) => {
                    // The engine's own answer, about its own hash table — so it
                    // is taken whichever client asked, with no scope check: this
                    // is a fact about the station's decoding, not a preference
                    // the client is borrowing.
                    self.known_calls.accept(reply);
                }
                RadioEvent::ClientSettings { profile, settings, has_stored } => {
                    // Remember which profile we are on whatever happens: it is
                    // what "Save to profile" is keyed on, and a signed-in client
                    // is told its name even before it has saved anything
                    // (discussion #4). Losing it here was why a save kept going
                    // to the station default and a profile was never created.
                    self.client_settings_from = Some(profile);
                    if has_stored {
                        // A real stored set: apply it. It is presentation-only,
                        // so it lays over the client's own settings without
                        // touching anything the machine owns — see
                        // `presentation_only` — and there is nothing about a
                        // look worth gating: a shared screen is harmless, a
                        // shared keyboard is not. The gate that used to stand
                        // here made the feature invisible to anyone who did not
                        // find the picker, which is exactly how it reached a
                        // tester on three releases.
                        settings.apply_to(&mut self.ui_settings);
                        self.view.center_on_vfo = settings.center_on_vfo;
                        self.view.fft_size = settings.fft_size;
                        self.client_settings_stored = Some(settings);
                    }
                    // `has_stored == false` is a name-only offer: the profile
                    // has nothing saved yet, so we keep our own look and simply
                    // know now which profile to save it to.
                }
                RadioEvent::ClientAcks(acks) => self.apply_client_acks(acks),
                RadioEvent::ClientBindings { profile, bindings } => {
                    // Applied and written out at once, so the restored keys
                    // survive a reload the same way a rebind does.
                    let apply = |me: &mut Self| {
                        me.input.cfg = bindings.clone();
                        me.input.cfg.migrate();
                        me.input.persist();
                        me.client_settings_from = Some(profile.clone());
                    };
                    let action = bindings_action(
                        self.ui_settings.client_share_bindings,
                        bindings.keys.len(),
                        self.bindings_offer_asked,
                    );
                    if action == BindingsAction::Apply {
                        apply(self);
                    } else if action == BindingsAction::Offer {
                        // **Offer it, do not drop it.** This client has not
                        // adopted the opt-in, which is the safe default on a
                        // shared station — so the bindings are held back. But
                        // holding them back *silently* is what made this look
                        // broken: the profile had them, the server sent them,
                        // the file on disk was right, and every session quietly
                        // discarded them. Nothing told the operator anything.
                        //
                        // Why the flag can be false on a machine that meant to
                        // opt in: it is client-local, and on the web it lives
                        // in browser storage, which the same eviction that
                        // dropped this operator's screen settings can empty.
                        // Asking again is the correct response to that. Silence
                        // never was.
                        if bindings_should_offer(
                            self.bindings_offer_asked,
                            self.ui_settings.client_bindings_declined,
                        ) {
                            self.bindings_pending =
                                Some(BindingsOffer { profile: profile.clone(), bindings });
                            self.bindings_offer_asked = true;
                        }
                    }
                }
                RadioEvent::ConnectionLost(e) => {
                    if self.focused {
                        self.speech.announcer.on_error(&e, now);
                    }
                    self.error = Some(e);
                    self.arm_retry(now);
                }
                // The engine's dynamic-DNS worker answered. Only the provider's
                // word arrives — the address carries the token and stays in the
                // engine — and this is what the settings row shows.
                RadioEvent::DyndnsAnswer { at_unix, ok, text } => {
                    self.dyndns_last = Some((at_unix, ok, text));
                }
                RadioEvent::LoginTest(r) => {
                    self.login_tests_pending.remove(&r.target);
                    self.push_net_log(format!(
                        "{}: {}",
                        r.target.label(),
                        if r.ok { format!("ok, {}", r.message) } else { r.message.clone() }
                    ));
                    self.login_tests.insert(r.target, r);
                }
                // Wavelog's probe knows more than pass or fail — the station
                // locations to choose from, and a certificate waiting to be
                // trusted — so it arrives beside the verdict for the settings
                // page to fill its lists from.
                RadioEvent::PictureUpload(st) => {
                    if st.is_test {
                        self.picup_test_pending = None;
                        self.picup_test = Some(st);
                    } else {
                        self.picup_last = Some(st);
                    }
                }
                RadioEvent::WavelogProbe(p) => self.wavelog_probe = Some(*p),
                RadioEvent::WavelogBatch(b) => self.on_wavelog_batch(*b),
                RadioEvent::Notice(n) => {
                    if self.focused {
                        self.speech.announcer.on_notice(n.as_deref(), now);
                    }
                    charge_failed_public_sdr(
                        n.as_deref(),
                        &mut self.last_public_sdr_try,
                        &mut self.dead_public_sdrs,
                    );
                    self.radio_notice = n;
                }
                // The ATS Mini's memory table, answered to a `memories-dump`.
                // Held for the settings tab's Memories section.
                RadioEvent::AtsMiniMemories(m) => self.atsmini_memories = Some(m),
                // The radio is still stepping toward a tune we sent; the note
                // shows under the dial while it is true.
                RadioEvent::AtsMiniTuning(on) => self.atsmini_tuning = on,
                RadioEvent::Ft8Decodes(d) => {
                    self.anchor_slot_follow(&d);
                    // **A coded weather report is not a message.** SYNOP and
                    // SHIP arrive on the same RTTY decode path as every other
                    // line, and a decode list showing `BBXX BRAVO 20123 99252…`
                    // as text is a decode list that has decoded nothing. They go
                    // to their own window instead, newest first.
                    //
                    // Client-side, so **no wire change**: the report is found
                    // where the text already arrives rather than on a new event.
                    for dec in &d {
                        if let Some(r) = sdroxide_types::synop::parse(&dec.message) {
                            self.weather_reports.insert(0, (now as i64, r));
                        }
                    }
                    self.weather_reports.truncate(64);
                    // The session ignore list is applied here, at ingress, and
                    // nowhere else. This is the one point every consumer of a
                    // decode passes through — the announcer, the audible
                    // alarms, the propagation field, the list itself and auto
                    // mode's choice of a station — so a muted station is
                    // genuinely out of the way instead of hidden from one view
                    // while still ringing and still being auto-answered. Rows
                    // already on screen are drawn dimmed rather than yanked
                    // (`app::ignore`), and nothing new arrives to replace
                    // them. Nothing leaves the program on a muted station's
                    // behalf: spotting and the online logbooks are untouched.
                    let d = crate::app::ignore::retain_unignored(&self.session_ignored, d);
                    if d.is_empty() {
                        continue;
                    }
                    if let Some(st) = self.digi_status.as_ref()
                        && self.focused
                    {
                        self.speech.announcer.on_ft8(&d, st, now);
                    }
                    // Audible alarms are the one thing that must not wait for
                    // the window to be focused: ringing when the operator is
                    // looking at this tab is a duplicate, not a feature.
                    if self.alerts.enabled()
                        && let Some(st) = self.digi_status.clone()
                    {
                        let dial_hz = self.state.active_freq_hz();
                        let band =
                            if dial_hz > 0.0 { sdroxide_types::adif_band(dial_hz) } else { "" };
                        // A clone of the novelty index: `log_index()` borrows
                        // the whole app, and the runtime needs `&mut self` to
                        // arm its cooldowns.
                        let log = self.log_index().clone();
                        if let Some(fired) = self.alerts.on_ft8(
                            &d,
                            &st.config.my_call,
                            &st.config.my_grid,
                            &log,
                            band,
                        ) && fired.reply.speaks()
                        {
                            // Spoken from the alarm path rather than the decode
                            // read-out above, which is its own switch: an alarm
                            // is to be heard when the operator is looking at
                            // another window.
                            //
                            // Deliberately *not* gated on `self.focused`, which
                            // is what upstream's review of its #591 chose: it
                            // holds a background tab's phrase back, for fear it
                            // queues behind the one in front and is read out
                            // late as stale news. We keep speaking. A CB or SWL
                            // station is often left listening with the window
                            // behind something else, and a new DXCC that merely
                            // rings is the whole case the voice was added for —
                            // the operator is not at this window to read it.
                            let country = sdroxide_types::entity_name(&fired.call);
                            self.speech.announcer.on_alert(
                                fired.event,
                                &fired.call,
                                band,
                                country,
                                now,
                            );
                        }
                    }
                    // Prepend newest-slot decodes; keep a rolling window.
                    let dial = self.state.rx_freq_hz();
                    // ...and fold them into the propagation field as they
                    // arrive, so the 3D globe's BANDS OPEN chart and the flat
                    // maps show this station's own paths whether or not a
                    // panel with a map is on screen — until now only
                    // `prop_texture` folded them, and only while one was.
                    // Under the mode's own source, the same one `prop_texture`
                    // uses, so the store's de-duplication sees one decode
                    // once; and not at all for meteor scatter and moonbounce.
                    // A decode is placed by the grid in its message, so a
                    // station that sent none is skipped.
                    //
                    // This is what the fork wanted too, and `prop_source_for`
                    // covers its case: FT8 and FT4 are the modes WSJT-CB
                    // packs into, and both map to their own source, so an 11 m
                    // tab draws its own paths. That matters here because the
                    // RBN skimmer feed, the other source of the field, never
                    // carries 11 m — `observe_decodes` also resolves a
                    // latteless CB callsign to its country, for the ones that
                    // sent no grid.
                    if let Some(src) = super::panels::prop_source_for(self.state.rx[0].mode) {
                        let v = self.view.solar3d;
                        self.prop.set_halflife_min(v.prop_halflife_min);
                        self.prop.set_sources(crate::prop_map::PropSources(v.prop_sources));
                        let grid = self.my_grid();
                        self.prop.observe_decodes(&d, src, dial, &grid, crate::time::now_unix());
                    }
                    // Newest slot first, and each decode at the top of its own
                    // slot's rows: FT8 streams a slot's slower finds after the
                    // quick ones (#667), and on a slow machine they can land
                    // after the next slot's — they belong under it, not above.
                    for dec in d.into_iter().rev() {
                        let at = self
                            .digi_decodes
                            .iter()
                            .position(|x| x.slot_utc <= dec.slot_utc)
                            .unwrap_or(self.digi_decodes.len());
                        self.digi_decodes.insert(at, dec);
                        self.digi_decode_dials.insert(at, dial);
                    }
                    self.digi_decodes.truncate(200);
                    self.digi_decode_dials.truncate(200);
                }
                RadioEvent::WsprSpots(s) => {
                    // Newest first, and de-duplicated against what is already
                    // held: the same reception arrives twice whenever a slot we
                    // decoded ourselves also comes back from WSPRnet, and two
                    // rows for one beacon reads as the mode double-counting.
                    //
                    // Through a set rather than a scan per arrival: the list
                    // holds a night of receptions now, and a linear search
                    // through it for every spot in a WSPRnet download is the
                    // one shape of this that grows with the square of a busy
                    // band (issue #316).
                    let mut held: std::collections::HashSet<_> =
                        self.wspr_spots.iter().map(|e| e.dedup_key()).collect();
                    for spot in s.into_iter().rev() {
                        if !held.insert(spot.dedup_key()) {
                            continue;
                        }
                        self.wspr_spots.insert(0, spot);
                    }
                    self.wspr_spots.truncate(crate::app::panels::wspr::WSPR_SPOT_ROWS);
                }
                RadioEvent::Pi4Spots(s) => {
                    // Newest first. No de-duplication set, unlike WSPR's:
                    // there is no PI4 equivalent of a WSPRnet download to
                    // double-report the same reception, and the engine only
                    // ever reports one slot's decode once.
                    for spot in s.into_iter().rev() {
                        self.pi4_spots.insert(0, spot);
                    }
                    self.pi4_spots.truncate(crate::app::panels::pi4::PI4_SPOT_ROWS);
                }
                RadioEvent::Ft8Status(s) => {
                    // Seed the editable config from the engine's persisted
                    // value once (later edits are UI-owned so typing sticks).
                    if !self.digi_cfg_seeded {
                        self.digi_cfg_edit = s.config.clone();
                        self.digi_cfg_seeded = true;
                    }
                    if self.focused {
                        self.speech.announcer.on_digi(&s, &self.state, now);
                    }
                    if let Some(a) = &s.acars {
                        a.messages.iter().for_each(|m| self.aircraft_registry.ingest_acars(m));
                    }
                    self.digi_status = Some(s);
                    // Coded weather arrives as RTTY text, not as a decode, so
                    // it is picked up here rather than in the decode arm.
                    self.scan_coded_weather(now as u64);
                }
                RadioEvent::Ft8QsoLogged(mut r) => {
                    // Another tab may have logged since this copy was read;
                    // appending to a stale copy would write its QSOs away. That
                    // reload needs a store to read: where there is none the log
                    // in hand is the truth, and treating "no store" as "an empty
                    // logbook" is what left a three-radio browser station
                    // exporting a logbook of one contact (issue #236).
                    if self.shared_log {
                        self.qso_log = crate::app::persist::after_shared_log(
                            std::mem::take(&mut self.qso_log),
                            crate::app::persist::reload_qso_log(None),
                        );
                    }
                    r.id = self.next_log_id();
                    // A contest session running takes every digital QSO that
                    // completes while it is on: tag it with the contest, and
                    // fill the sent serial and our own exchange from the
                    // session, so the FT8 side logs itself while the operator
                    // only types on CW/SSB. The station's own exchange comes
                    // from the digi exchange when the mode is in a contest
                    // (`DigiConfig::contest`), and is left blank otherwise.
                    if let Some(session) = self.contest.as_mut() {
                        r.contest_id = session.contest.log_id().to_string();
                        if session.contest.sends_serial() && r.stx.is_none() {
                            r.stx = Some(session.next_serial);
                            session.next_serial =
                                sdroxide_types::next_contest_serial(session.next_serial);
                        }
                        if r.stx_string.is_empty() {
                            // Serial and our own exchange on the same line: the
                            // serial alone loses the locator an EU VHF contact
                            // needs, and the locator alone loses the serial.
                            r.stx_string = session.sent_exchange(r.stx);
                        }
                    }
                    let call = r.call.clone();
                    let adif = auto_upload_adif(&self.net_cfg_edit, &r);
                    self.last_logged_qso_id = Some(r.id);
                    self.session_qso_ids.insert(r.id);
                    self.qso_log.push(r);
                    self.session_qsos += 1;
                    persist_qso_log(&self.qso_log);
                    // Enrich + optionally upload the freshly logged QSO.
                    self.queue_lookup(call);
                    if let Some((qso_id, adif, targets)) = adif {
                        self.pending_uploads.push((qso_id, adif, targets));
                    }
                }
                RadioEvent::SstvLine { image_id, y, rgb } => {
                    self.sstv.on_line(image_id, y, &rgb, &ctx);
                }
                RadioEvent::SstvImage { png, .. } => self.sstv.on_image(&png, &ctx),
                RadioEvent::DigiImage { png } => {
                    if let Some((rgb, w, h)) = crate::sstv::decode_image(&png) {
                        let ci = crate::sstv::color_image(&rgb, w, h);
                        let tex = ctx.load_texture("fsq_rx", ci, egui::TextureOptions::LINEAR);
                        self.fsq_rx_images.insert(0, tex);
                        self.fsq_rx_images.truncate(30);
                    }
                }
                RadioEvent::HellColumns { seq, rows, cols } => {
                    self.hell.on_columns(seq, rows, &cols, &self.view.hell, &ctx);
                }
                RadioEvent::WefaxLine { image_id, y, gray } => {
                    self.wefax.push_line(image_id, y, &gray);
                }
                RadioEvent::WefaxImage { png, .. } => {
                    // The chart is held rather than filed: the engine names the
                    // file it wrote and announces it a moment later, and that
                    // name is the chart's whole metadata. Guessing it here — as
                    // this once did, off a second clock — would sooner or later
                    // label a chart a second away from the file it is.
                    self.wefax.hold_fresh(&ctx, &png);
                    self.wefax.clear_live();
                }
                RadioEvent::WefaxStatus(s) => self.wefax.status = s,
                RadioEvent::Rds(d) => self.on_rds(d),
                RadioEvent::Drm(d) => self.on_drm(d),
                RadioEvent::HdRadio(d) => self.on_hd(d),
                // A whole-table snapshot, so it replaces rather than merges: the
                // engine's table is authoritative and already carries the history
                // — first heard, times heard — that a merge here would be
                // reconstructing badly.
                RadioEvent::IsmReports(r) => self.ism_reports = r,
                RadioEvent::IsmStatus(st) => self.ism_status = Some(st),
                RadioEvent::AdsbStatus(st) => {
                    st.aircraft.iter().for_each(|a| self.aircraft_registry.ingest_adsb(a));
                    self.aircraft_registry.expire(crate::time::now_unix());
                    self.adsb_status = Some(st);
                }
                RadioEvent::Vdl2Status(st) => {
                    st.messages.iter().for_each(|m| self.aircraft_registry.ingest_vdl2(m));
                    self.aircraft_registry.expire(crate::time::now_unix());
                    self.vdl2_status = Some(st);
                }
                RadioEvent::AisStatus(st) => self.ais_status = Some(st),
                RadioEvent::DabStatus(st) => {
                    // Outside a sweep, a channel tuned by hand teaches the list
                    // too: its stations are remembered, and it joins the found
                    // channels, the moment the ensemble names them. The sweep
                    // does its own remembering, block by block.
                    let channel = self.state.dab.channel.clone();
                    if self.dab_scan.is_none()
                        && let Some(services) =
                            crate::app::panels::dab::dab_scan_heard(&st, &channel)
                    {
                        let mut changed = self.state.dab.remember_stations(&channel, services);
                        if !self.state.dab.found.contains(&channel) {
                            self.state.dab.found.push(channel.clone());
                            changed = true;
                        }
                        if changed {
                            let cmd = Command::SetDabConfig(self.state.dab.clone());
                            self.dial_hold.note(&cmd);
                            self.ctrl.send(cmd);
                        }
                    }
                    // Another station playing: the last one's picture is not
                    // this one's, so it goes until the new one sends its own.
                    let was = self.dab_status.as_ref().and_then(|o| o.playing.clone());
                    if was != st.playing {
                        self.dab_slide = None;
                    }
                    self.dab_status = Some(st);
                }
                RadioEvent::DabSlide(slide) => {
                    let playing = self.dab_status.as_ref().and_then(|s| s.playing.as_deref());
                    if playing == Some(slide.service_id.as_str()) {
                        match crate::app::panels::dab_receiver::slide_image(&slide.data) {
                            Some(img) => {
                                self.dab_slide = Some(ctx.load_texture(
                                    "dab_slide",
                                    img,
                                    egui::TextureOptions::LINEAR,
                                ));
                            }
                            None => tracing::debug!(
                                "DAB: a slideshow picture ({}) did not decode",
                                slide.content_type
                            ),
                        }
                    }
                }
                RadioEvent::HeardFrame(edge) => {
                    if let Some(t) = self.slot_timing() {
                        self.slot_follow.edge(&t, edge);
                    }
                }
                RadioEvent::TxRefused(why, hz) => self.on_tx_refused(why, hz),
                RadioEvent::AudioLevel(l) => {
                    self.audio_level = Some((crate::time::now_unix_f64(), *l))
                }
                RadioEvent::Qo100Status(st) => self.qo100_status = Some(st),
                RadioEvent::HfdlStatus(st) => {
                    // Feed the map's plot table before the log scrolls anything
                    // out of its rolling window: the table keeps an aircraft
                    // until thirty minutes of silence retires it.
                    self.hfdl_map.observe(&st.log, crate::time::now_unix());
                    st.log.iter().for_each(|d| self.aircraft_registry.ingest_hfdl(d));
                    self.aircraft_registry.expire(crate::time::now_unix());
                    self.hfdl_status = Some(st);
                }
                RadioEvent::SondeStatus(st) => self.sonde_status = Some(st),
                RadioEvent::VorStatus(st) => self.vor_status = Some(st),
                RadioEvent::SstvStatus(s) => {
                    // Adopt a *newly* detected RX mode for the next transmit, but
                    // don't re-apply a steady detection every frame — that would
                    // fight the operator's manual mode selection.
                    if s.detected != self.sstv.last_detected {
                        if let Some(m) = s.detected {
                            self.sstv.tx_mode = m;
                            self.sstv.preview_dirty = true;
                        }
                        self.sstv.last_detected = s.detected;
                    }
                    // **A frame the save gate dropped is wiped, not left on
                    // screen.** Its lines were drawn as they arrived, and on a
                    // quiet frequency nothing follows for hours: a panel holding
                    // a frame of noise reads as a receiver that has hung (bench,
                    // 2026-11-04). The panel says why instead.
                    if s.not_saved.is_some() && !s.rx_active {
                        self.sstv.rx_color = None;
                        self.sstv.rx_tex = None;
                    }
                    self.sstv.status = s;
                }
                RadioEvent::RifpRows { image_id, y, w, h, rows } => {
                    self.sstv.on_rifp_rows(image_id, y, w, h, &rows, &ctx);
                }
                RadioEvent::RifpImage { png, .. } => self.sstv.on_rifp_image(&png, &ctx),
                RadioEvent::RifpStatus(s) => {
                    self.sstv.rifp = s;
                }
                RadioEvent::SkimmerSpots(s) => {
                    // The engine sends the full current set each update; the
                    // stable `id` per spot lets the overlay keep each box (and
                    // its scroll) in place across updates.
                    for spot in &s {
                        // Remember when each spot last keyed, and seed newly
                        // seen ones to now, so alpha starts solid and fades.
                        let e = self.skimmer_active_at.entry(spot.id).or_insert(now);
                        if spot.active {
                            *e = now;
                        }
                    }
                    // Forget timings for spots the engine has dropped.
                    let live: std::collections::HashSet<u64> = s.iter().map(|x| x.id).collect();
                    self.skimmer_active_at.retain(|id, _| live.contains(id));
                    self.skimmer_spots = s;
                }
                // A tab that shares the station radio's feeds drops back to
                // them on the next frame if its own engine says anything.
                RadioEvent::Spots(s) => {
                    self.spots = s;
                    self.spots_gen += 1;
                    self.adopted_spots_gen = None;
                }
                RadioEvent::BandOpenings(o) => {
                    self.band_openings = o;
                    self.spots_gen += 1;
                    self.adopted_spots_gen = None;
                }
                RadioEvent::NetStatus(s) => {
                    self.net_status = s;
                    self.spots_gen += 1;
                    self.adopted_spots_gen = None;
                }
                RadioEvent::TciServerStatus { running, addr, clients, error } => {
                    self.tci_srv_status = Some(TciServerStatus { running, addr, clients, error });
                }
                RadioEvent::RigctldStatus { running, addr, clients, error } => {
                    self.rigctld_status = Some(TciServerStatus { running, addr, clients, error });
                }
                RadioEvent::VoiceStatus(v) => self.voice = v,
                RadioEvent::ImagePresets(p) => self.sstv.on_presets(p, &ctx),
                RadioEvent::ImageSlotSource { slot, version, png } => {
                    self.sstv.on_slot_source(slot, version, &png);
                }
                // One store per mode: SSTV and RIFP share a gallery, charts
                // have their own.
                RadioEvent::ImageListing(l) => match l.kind {
                    sdroxide_types::ImageKind::Sstv => self.sstv.on_listing(l, &ctx),
                    sdroxide_types::ImageKind::Wefax => self.wefax.on_listing(l, &ctx),
                },
                RadioEvent::ImageFile { kind, name, png } => match kind {
                    sdroxide_types::ImageKind::Sstv => self.sstv.on_file(&name, &png, &ctx),
                    sdroxide_types::ImageKind::Wefax => self.wefax.on_file(&name, &png, &ctx),
                },
                // What the station is set up to do, from the machine the engine
                // runs on. Seeded once, like the digi config above: later edits
                // are the dialog's, so typing sticks, and the engine echoes
                // every applied change back here anyway.
                RadioEvent::StationConfig(c) => {
                    if !self.net_cfg_seeded {
                        self.net_cluster_cmds = c.net.cluster.commands.join("\n");
                        self.net_rbn_cmds = c.net.rbn.commands.join("\n");
                        self.net_rbn2_cmds = c.net.rbn2.commands.join("\n");
                        self.net_cfg_edit = c.net.clone();
                        self.net_cfg_seeded = true;
                    }
                    if !self.rigctld_seeded {
                        self.rigctld_edit = c.rigctld.clone();
                        self.rigctld_seeded = true;
                    }
                    if !self.tci_srv_seeded {
                        self.tci_srv_edit = c.tci_server.clone();
                        self.tci_srv_seeded = true;
                    }
                    if !self.wsjtx_seeded {
                        self.wsjtx_edit = c.wsjtx.clone();
                        self.wsjtx_seeded = true;
                    }
                    if !self.picup_seeded {
                        self.picup_edit = c.picture_upload.clone();
                        self.picup_seeded = true;
                    }
                    if !self.sat_cfg_seeded {
                        self.sat_cfg_edit = c.sat.clone();
                        self.sat_cfg = std::sync::Arc::new(c.sat.clone());
                        self.sat_cfg_seeded = true;
                    }
                    if !self.rot_cfg_seeded {
                        self.rot_cfg_edit = c.rotator.clone();
                        self.rot_cfg_seeded = true;
                    }
                    if !self.relay_seeded {
                        self.relay_edit = c.relay.clone();
                        self.relay_seeded = true;
                    }
                    // Adopted on every announcement rather than seeded once:
                    // these are not dialog buffers the operator types into but
                    // the band plan the whole client draws with, and the
                    // station is its only authority. A remote client that kept
                    // its own would show European band edges for an American
                    // radio.
                    self.region_edit = c.region;
                    sdroxide_types::set_region(c.region);
                    self.cb_plan_edit = c.cb_plan;
                    sdroxide_types::set_cb_plan(c.cb_plan);
                    self.cb_tx_edit = c.cb_tx_allowed;
                    sdroxide_types::set_cb_tx_allowed(c.cb_tx_allowed);
                    // Only when it actually changed: installing leaks the
                    // previous plan, and this bundle arrives on every
                    // station-config edit — a password change must not cost an
                    // allocation that is never freed.
                    if sdroxide_types::band_plan() != &c.band_plan {
                        sdroxide_types::set_band_plan(c.band_plan.clone());
                    }
                    // The operator's own additions to the modes' frequency
                    // tables, on the same terms and for the same reason: the
                    // station owns the list, and this client draws the picker
                    // from it. Guarded against the same leak.
                    if sdroxide_types::digi_presets() != c.digi_presets.as_slice() {
                        sdroxide_types::set_digi_presets(c.digi_presets.clone());
                    }
                }
                RadioEvent::TleSubStatus(s) => self.on_tle_sub_status(s),
                RadioEvent::SatTrack(t) => self.sat_track = t.map(|t| *t),
                RadioEvent::RotatorStatus { connected, az_deg, el_deg, error } => {
                    self.rotator_status = Some((connected, az_deg, el_deg, error));
                }
                RadioEvent::RelayStatus(st) => self.relay_status = *st,
                RadioEvent::ImageSaved(e) => match e.kind {
                    sdroxide_types::ImageKind::Sstv => self.sstv.on_saved(e, &ctx),
                    sdroxide_types::ImageKind::Wefax => self.wefax.on_saved(e, &ctx),
                },
                // Broadcast, so this is as likely to be another screen's delete
                // as our own — either way the picture has gone.
                RadioEvent::ImageDeleted { kind, name } => match kind {
                    sdroxide_types::ImageKind::Sstv => self.sstv.on_deleted(&name),
                    sdroxide_types::ImageKind::Wefax => self.wefax.on_deleted(&name),
                },
                RadioEvent::WinlinkStatus(st) => self.mail.on_status(st),
                RadioEvent::MailListing(l) => self.mail.on_listing(l),
                RadioEvent::MailMessage(m) => self.mail.on_message(m),
                RadioEvent::MailSaved(mid) => self.mail.on_saved(mid),
                RadioEvent::MailDeleted { folder, mid } => self.mail.on_deleted(folder, &mid),
                RadioEvent::CallsignResult(info) => self.apply_callsign(info),
                RadioEvent::Upload(r) => self.on_upload_result(r),
                RadioEvent::Confirmations(recs) => self.apply_confirmations(recs),
                // Folded here rather than queued for the map's own pass: these
                // arrive whether or not a map is on screen, and a queue nobody
                // drained would grow all session. The display settings are
                // applied first because they decide whether this source counts
                // at all — `prop_texture` does the same before its own folds.
                RadioEvent::PropPaths(paths) => {
                    let v = self.view.solar3d;
                    self.prop.set_halflife_min(v.prop_halflife_min);
                    self.prop.set_sources(crate::prop_map::PropSources(v.prop_sources));
                    self.prop.observe_paths(
                        &paths,
                        sdroxide_types::PropSource::Rbn,
                        crate::time::now_unix(),
                    );
                }
                // The interface configuration, seeded once like the station
                // config above. It is read at construction too, but a remote
                // client may not have had it yet, and the per-radio SWL mode
                // (`hide_tx`) rides in it — a radio that is a listener's screen
                // must not show transmit controls while the announcement is in
                // flight.
                RadioEvent::RadioConfig(c) => {
                    // Always the latest: the engine sends it after every change,
                    // and a stale copy would judge the transmit bands wrongly.
                    self.radio_cfg = Some(*c);
                }
            }
        }
        // A switched-off skimmer stops emitting, so its last boxes would sit on
        // the waterfall until something else replaced them; drop them per kind.
        if !self.skimmer_spots.is_empty() {
            self.skimmer_spots.retain(|s| self.state.skimmer.enabled(s.kind));
        }
        // A hidden tab has no frame of its own to flush these on, and its
        // digital modes keep logging: send them now. MIDI is discarded, not
        // queued — a control surface drives the focused radio, and a backlog
        // of knob turns firing on tab switch would retune it at random.
        if !self.focused {
            for call in std::mem::take(&mut self.pending_lookups) {
                self.ctrl.send(Command::LookupCallsign { call });
            }
            for (qso_id, adif, targets) in std::mem::take(&mut self.pending_uploads) {
                self.ctrl.send(Command::UploadQso { qso_id, adif, targets });
            }
            #[cfg(not(target_arch = "wasm32"))]
            self.input.discard_midi();
            // A hidden tab's 3D window is a real OS window, and one the shell
            // is no longer drawing: keep it emitted, or it is torn down and
            // remapped on the next switch — a *new* window to a tiling
            // compositor, sized by it rather than left as the operator had it.
            //
            // Only the radio that owns the one 3D window does this: a second
            // radio opening its own takes ownership, and the first is let go, so
            // hidden radios cannot accumulate rendering windows.
            #[cfg(not(target_arch = "wasm32"))]
            if self.solar.open && crate::solar3d::solar3d_owner(ctx) == Some(self.radio_id) {
                let prev = crate::layout::radio_salt(ctx);
                crate::layout::set_radio_salt(ctx, self.radio_id);
                self.solar.keep_alive(ctx);
                crate::layout::set_radio_salt(ctx, prev);
            }
        }
    }

    /// The awards dashboard: DXCC / WAS / WAZ / grid counts (worked vs
    /// confirmed) with a band filter, plus the WAS state grid and WAZ zone grid.
    /// The out-of-band transmit warning.
    ///
    /// Modal and dismissed by hand, because the band-edge lockout is the last
    /// thing between a mistyped frequency and an out-of-band transmission, and
    /// an operator who does not know it is off is exactly the operator who will
    /// find out the expensive way. The button is a one-shot acknowledgement: it
    /// comes back next launch, because the flag has to be passed again next
    /// launch. The checkbox beside it is the remembered half — ticked, the
    /// acknowledgement is kept on this screen and the page does not come back,
    /// for an operator who runs with `--oob-tx` every time and knows what it
    /// means.
    ///
    /// Driven off the *engine's* state rather than off this process's arguments
    /// so a remote client is warned too — the licence at risk belongs to
    /// whoever is at the controls, who need not be whoever started the engine.
    fn oob_tx_window(&mut self, ctx: &egui::Context) {
        if !self.state.oob_tx || self.oob_tx_ack {
            return;
        }
        let mut dismissed = false;
        let mut remember = false;
        let resp = egui::Window::new(tr!("frame-transmit-lockout-disabled"))
            .id(crate::layout::salted_id(ctx, "oob-tx-window"))
            .frame(crate::chrome::window_frame())
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                crate::chrome::window_body_bg(ui);
                ui.set_max_width(crate::layout::window_w(ctx, 430.0));
                ui.label(
                    RichText::new(tr!("frame-engine-was-started-oob-tx"))
                        .color(crate::theme::TEXT_STRONG())
                        .size(13.0),
                );
                ui.add_space(6.0);
                ui.label(
                    RichText::new(tr!("frame-transmitting-outside-your-licence-offence"))
                        .color(crate::theme::TEXT()),
                );
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if crate::chrome::chip_accent(
                        ui,
                        false,
                        RichText::new(tr!("frame-i-understand")).strong(),
                        crate::theme::ALERT(),
                        crate::theme::TEXT_STRONG(),
                    )
                    .clicked()
                    {
                        dismissed = true;
                    }
                    // Next to the button, so the operator who knows what the
                    // lockout being off means can say so once and not be
                    // stopped by this page every launch. Ticking it is itself
                    // an acknowledgement, so the window goes away with it.
                    if ui
                        .checkbox(&mut self.ui_settings.oob_tx_dismissed, tr!("frame-dont-show-again"))
                        .on_hover_text(tr!("frame-dont-show-again-hint"))
                        .changed()
                    {
                        remember = true;
                        dismissed = true;
                    }
                });
                ui.add_space(4.0);
                ui.label(
                    RichText::new(tr!("frame-restart-without-oob-tx-put"))
                        .color(crate::theme::LINE_LIT())
                        .size(10.5),
                );
            });
        if let Some(r) = &resp {
            crate::chrome::paint_window_border(ctx, &r.response);
        }
        if dismissed {
            self.oob_tx_ack = true;
        }
        if remember {
            crate::app::persist::persist_ui_settings(&self.ui_settings);
        }
    }

    /// The one-time warning shown when 11 m transmit is switched on.
    ///
    /// 11 m is not an amateur band, so it sits behind the same lockout as every
    /// other non-amateur band until the operator deliberately opens it. This is
    /// that deliberateness made explicit: the permission is not granted until
    /// they confirm they know what the band is and that its use is governed by
    /// their own country's rules. Confirmed once per screen
    /// (`cb_tx_warning_ack`), not on every flip of the switch.
    fn cb_tx_confirm_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        if !self.cb_tx_confirm_open {
            return;
        }
        let mut confirm = false;
        let mut cancel = false;
        let resp = egui::Window::new(tr!("frame-cb-window-title"))
            .id(crate::layout::salted_id(ctx, "cb-tx-confirm-window"))
            .frame(crate::chrome::window_frame())
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                crate::chrome::window_body_bg(ui);
                ui.set_max_width(crate::layout::window_w(ctx, 460.0));
                ui.label(
                    RichText::new(tr!("frame-cb-about-to-allow"))
                        .color(crate::theme::TEXT_STRONG())
                        .size(13.0),
                );
                ui.add_space(6.0);
                ui.label(
                    RichText::new(tr!("frame-cb-not-amateur"))
                    .color(crate::theme::TEXT()),
                );
                ui.add_space(6.0);
                ui.label(
                    RichText::new(tr!("frame-cb-only-continue"))
                    .color(crate::theme::TEXT()),
                );
                ui.add_space(10.0);
                ui.horizontal(|ui| {
                    if crate::chrome::chip_accent(
                        ui,
                        false,
                        RichText::new(tr!("frame-i-understand")).strong(),
                        crate::theme::ALERT(),
                        crate::theme::TEXT_STRONG(),
                    )
                    .clicked()
                    {
                        confirm = true;
                    }
                    if crate::chrome::chip(ui, false, tr!("frame-cancel")).clicked() {
                        cancel = true;
                    }
                });
            });
        if let Some(r) = &resp {
            crate::chrome::paint_window_border(ctx, &r.response);
        }
        if confirm {
            self.ui_settings.cb_tx_warning_ack = true;
            crate::app::persist::persist_ui_settings(&self.ui_settings);
            self.cb_tx_edit = true;
            sdroxide_types::set_cb_tx_allowed(true);
            cmds.push(Command::SetCbTxAllowed(true));
            // The 11M-CB tick follows the permission, as everywhere else.
            if let Some(cfg) = self.radio_cfg.as_mut() {
                cfg.set_tx_band(Band::M11, true);
                self.ctrl.set_radio_config(cfg.clone());
            }
            self.cb_tx_confirm_open = false;
        } else if cancel {
            self.cb_tx_confirm_open = false;
        }
    }

    /// Draw whichever operating panel belongs to `mode`.
    ///
    /// One call for the normal workspace and for the Retro Radio decode window,
    /// so the two can never drift apart. `panel_h` is the height the panel was
    /// sized for; the modes that ignore it take no height at all.
    pub(in crate::app) fn operating_panel(
        &mut self,
        ui: &mut egui::Ui,
        cmds: &mut Vec<Command>,
        mode: Mode,
        panel_h: f32,
    ) {
        let is_text = mode.is_text_modem();
        if mode.is_rade() {
            self.rade_panel(ui, cmds, panel_h);
        } else if mode.is_atchat() {
            self.atchat_panel(ui, cmds, panel_h);
        } else if mode.is_wefax() {
            self.wefax_panel(ui, cmds, panel_h);
        } else if mode == Mode::Navtex {
            self.navtex_panel(ui, cmds, panel_h);
        } else if mode == Mode::Acars && !self.show_acars {
            self.acars_panel(ui, cmds, panel_h);
        } else if mode == Mode::Dsc {
            self.dsc_panel(ui, cmds, panel_h);
        } else if mode == Mode::UvPacket && !self.show_uvpacket {
            self.uvpacket_panel(ui, cmds, panel_h);
        } else if mode == Mode::Jtty && !self.show_jtty {
            self.jtty_panel(ui, cmds, panel_h);
        } else if mode == Mode::Ale && !self.show_ale {
            self.ale_panel(ui, cmds, panel_h);
        } else if mode.is_image() && !self.image_window_open(mode) {
            self.image_panel(ui, cmds, mode);
        } else if mode.is_rf_paint() && !self.show_rf_paint {
            self.rf_paint_panel(ui, cmds, panel_h);
        } else if mode.is_adsb() && !self.show_adsb {
            self.adsb_panel(ui, cmds, panel_h);
        } else if mode == Mode::Dab && !self.show_dab {
            self.dab_panel(ui, cmds, panel_h);
        } else if mode.is_vdl2() && !self.show_vdl2 {
            self.vdl2_panel(ui, cmds, panel_h);
        } else if mode.is_ais() && !self.show_ais {
            self.ais_panel(ui, cmds, panel_h);
        } else if mode.is_hfdl() && !self.show_hfdl {
            self.hfdl_panel(ui, cmds, panel_h);
        } else if mode.is_aprs() && !self.show_aprs {
            self.aprs_panel(ui, cmds, panel_h);
        } else if mode.is_packet() && !self.show_packet {
            self.packet_panel(ui, cmds, panel_h);
        } else if mode.is_fsq() && !self.show_fsq {
            self.fsq_panel(ui, cmds, panel_h);
        } else if mode.is_hell() && !self.show_hell {
            self.hell_panel(ui, cmds, panel_h);
        } else if is_text && !self.show_text {
            self.text_modem_panel(ui, cmds, panel_h);
        } else if mode.is_js8() && !self.show_js8 {
            self.js8_panel(ui, cmds, panel_h);
        } else if mode.is_wspr() && !self.show_wspr {
            self.wspr_panel(ui, cmds, panel_h);
        } else if mode.is_pi4() && !self.show_pi4 {
            self.pi4_panel(ui, cmds, panel_h);
        } else if matches!(
            mode,
            Mode::Jt65 | Mode::Jt9 | Mode::Fst4 | Mode::Msk144 | Mode::Q65 | Mode::Fsk441
        ) {
            self.jt_panel(ui, cmds);
        } else {
            self.digi_panel(ui, cmds);
        }
    }

    /// Send this client's screen settings to the server, when the operator has
    /// asked to keep them there. A no-op for a local engine (which has no
    /// server to tell) and when the scope is `Browser`. The profile sent is the
    /// one already in use, so a save lands on the set the client is reading
    /// rather than silently creating another.
    /// Save this screen's look against the profile, **on request only**.
    ///
    /// Not on every change, which is what it used to do: on a passwordless
    /// server every client shares the station default, so one operator moving
    /// the theme would move it for whoever signs in next. An explicit action
    /// says who decided it, and the row says afterwards what it did.
    /// Keep the signed-in profile's copy of the screen in step with what the
    /// operator is actually using, so the next session — on this device or
    /// another — comes back as they left it, **without a button being
    /// pressed**.
    ///
    /// **This is the fork's answer to "store everything on the server, not in
    /// the browser"** (fork discussion #4 and #9). The argument is Kevin's and
    /// it is the right one: browser storage cannot be depended on — `persist()`
    /// behaves differently in every browser (Chrome refuses it silently,
    /// Firefox prompts), so it works for one person and not the next — and a
    /// session kept there cannot be reset by anyone who is not standing in
    /// front of that device. On the server it can: it is a file, and the person
    /// who runs the station can put a good one back over SSH. That is what
    /// makes a station *administrable*, and it is why the screen belongs there.
    ///
    /// **Only with a real signed-in profile, and that restriction is the whole
    /// reason the button used to be manual.** A server with no password has one
    /// shared `default` bucket, so storing as we go would let one operator's
    /// theme become the next one's. A login is one person, so it is safe there
    /// and only there.
    fn auto_save_screen(&mut self, now: f64) {
        if !matches!(self.client_settings_from, Some(Some(_))) {
            return;
        }
        // Debounced: dragging the theme picker is one write, not sixty.
        if now - self.client_save_last < CLIENT_AUTO_SAVE_S {
            return;
        }
        let mut screen = sdroxide_types::ClientScreen::from_settings(&self.ui_settings);
        screen.center_on_vfo = self.view.center_on_vfo;
        screen.fft_size = self.view.fft_size;
        if self.client_settings_stored == Some(screen) {
            return;
        }
        self.client_save_last = now;
        let profile = self.client_settings_from.clone().flatten();
        self.ctrl.send_client_settings(profile, screen);
        self.client_settings_stored = Some(screen);
        // Deliberately no `client_settings_status`: this happens on its own, and
        // a row that announces a save every few seconds is a row nobody reads.
        // The manual buttons still say what they did.
    }

    pub(in crate::app) fn save_screen_to_profile(&mut self) {
        let profile = self.client_settings_from.clone().flatten();
        let mut screen = sdroxide_types::ClientScreen::from_settings(&self.ui_settings);
        // `from_settings` cannot see these — they are the panadapter's own, in
        // `ViewState`, which this crate owns and `sdroxide-types` does not. So
        // the client fills them here and puts them back on the way in.
        screen.center_on_vfo = self.view.center_on_vfo;
        screen.fft_size = self.view.fft_size;
        self.ctrl.send_client_settings(profile.clone(), screen);
        self.client_settings_stored = Some(screen);
        self.client_settings_status = Some(match &profile {
            Some(name) => tr!("frame-saved-to-profile", name = name),
            None => tr!("frame-saved-as-default"),
        });
    }

    /// Silent variant used by control chips: persists the screen (including the
    /// FFT size and CTR) without announcing it in the status row.
    pub(in crate::app) fn save_screen_to_profile_silent(&mut self) {
        let profile = self.client_settings_from.clone().flatten();
        let mut screen = sdroxide_types::ClientScreen::from_settings(&self.ui_settings);
        screen.center_on_vfo = self.view.center_on_vfo;
        screen.fft_size = self.view.fft_size;
        self.ctrl.send_client_settings(profile, screen.clone());
        self.client_settings_stored = Some(screen);
    }
}

impl SdroxideApp {
    /// This client's "do not ask me again" answers, as it holds them now.
    fn current_client_acks(&self) -> sdroxide_types::ClientAcks {
        let mut rx_only_dismissed = Vec::new();
        if self.rx_only_nudge_dismissed {
            rx_only_dismissed.push(self.radio_id);
        }
        sdroxide_types::ClientAcks {
            bindings: sdroxide_types::ClientAcks::bindings_from(
                self.ui_settings.client_share_bindings,
                self.ui_settings.client_bindings_declined,
            ),
            dismissed_advisories: self.ui_settings.dismissed_advisories,
            cb_tx_warning_ack: self.ui_settings.cb_tx_warning_ack,
            rx_only_dismissed,
        }
    }

    /// Take the server's answers. **The server is authoritative**: a question
    /// it holds an answer for is never asked here, whatever the browser did or
    /// did not keep.
    pub(in crate::app) fn apply_client_acks(&mut self, server: sdroxide_types::ClientAcks) {
        match server.bindings {
            Some(true) => {
                self.ui_settings.client_share_bindings = true;
                self.ui_settings.client_bindings_declined = false;
            }
            Some(false) => {
                self.ui_settings.client_share_bindings = false;
                self.ui_settings.client_bindings_declined = true;
                // Asked on this connection already? The answer is in.
                self.bindings_pending = None;
            }
            None => {}
        }
        self.ui_settings.dismissed_advisories |= server.dismissed_advisories;
        self.ui_settings.cb_tx_warning_ack |= server.cb_tx_warning_ack;
        if server.rx_only_dismissed.contains(&self.radio_id) {
            self.rx_only_nudge_dismissed = true;
        }
        self.acks_server = Some(server);
    }

    /// Send an answer the server does not hold yet. Nothing goes out until the
    /// server has said what it holds, so a fresh tab cannot report "nothing
    /// answered" over somebody's answer — and the server merges in any case.
    pub(in crate::app) fn sync_client_acks(&mut self) {
        let Some(server) = self.acks_server.as_ref() else { return };
        let mine = self.current_client_acks();
        let mut want = server.clone();
        want.merge(&mine);
        if want != *server {
            self.ctrl.send_client_acks(mine);
            // Assume it lands; the server's echo replaces this either way.
            self.acks_server = Some(want);
        }
    }
}

/// How often the automatic profile save may write, in seconds.
///
/// Long enough that dragging a slider through its values is one write, short
/// enough that closing a tab right after a change still has it stored.
const CLIENT_AUTO_SAVE_S: f64 = 2.0;

/// Control bindings the signed-in profile carries, offered rather than
/// dropped when this client has not adopted the opt-in.
///
/// The same shape as [`SdroxideApp::client_settings_stored`]: kept so the
/// question can be asked with the profile's name in it, and so accepting is
/// a single click rather than a round trip.
/// What to do with the control bindings a server just offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::app) enum BindingsAction {
    /// This client opted in: use them.
    Apply,
    /// Not opted in, but the profile has keys. **Ask** — do not drop them
    /// in silence, which is what made this look broken.
    Offer,
    /// Not opted in and there is nothing to offer anyway.
    Ignore,
    /// Already offered this session; a client that said no is not asked again.
    AlreadyAsked,
}

/// The decision, as a pure function so it can be pinned.
///
/// `opted_in` is `UiSettings::client_share_bindings`, which is *client-local*
/// and on the web lives in browser storage — so the same eviction that hid an
/// operator's screen settings can empty it. That is not a reason to stay
/// silent; it is exactly the case where asking again is right.
pub(in crate::app) fn bindings_action(opted_in: bool, keys: usize, asked: bool) -> BindingsAction {
    if opted_in {
        return BindingsAction::Apply;
    }
    if keys == 0 {
        // Nothing to carry. Offering an empty set would be a question with no
        // possible answer, and adopting it would clear the operator's keys.
        return BindingsAction::Ignore;
    }
    if asked {
        return BindingsAction::AlreadyAsked;
    }
    BindingsAction::Offer
}

#[derive(Clone)]
pub(in crate::app) struct BindingsOffer {
    /// Which profile they came from — a name, or `None` for the station
    /// default.
    pub profile: Option<String>,
    pub bindings: sdroxide_types::InputSettings,
}

/// Should this client be offered a profile's keyboard bindings?
///
/// Two different answers have to end the question for good, and only one used
/// to: the opt-in persists when the operator says *yes*, so saying *no*
/// persisted nothing and the offer came back on the next session and on every
/// other radio of the station — the fork's #18, "displays all the time, already
/// reported". Both are booleans now, so both answers stick.
///
/// Deliberately not a function of anything the *server* sends: this is a
/// decision made on a machine, like the opt-in beside it.
fn bindings_should_offer(already_asked_this_session: bool, declined: bool) -> bool {
    !already_asked_this_session && !declined
}

impl SdroxideApp {
    /// The profile carries control bindings and this client has not said it
    /// wants them. Ask, once, and act on the answer.
    ///
    /// Asking rather than applying is not a gesture at safety: adopting a
    /// profile's keys *is* rebinding a shared keyboard, which is why the
    /// opt-in exists and why it stays off by default. What was wrong before
    /// was not the caution, it was the silence — the operator stored keys,
    /// the server held them, and every session dropped them with nothing
    /// said anywhere. One question, with the same warning the settings row
    /// gives, and the answer is persisted so it is not asked twice.
    pub(in crate::app) fn bindings_offer_ui(&mut self, ctx: &egui::Context) {
        let Some(offer) = self.bindings_pending.clone() else { return };
        let mut adopt = false;
        let mut decline = false;
        egui::Modal::new(egui::Id::new("client-bindings-offer")).show(ctx, |ui| {
            ui.set_max_width(470.0);
            ui.heading(
                RichText::new(tr!("frame-bindings-offer-title"))
                    .color(crate::theme::ALERT()),
            );
            ui.add_space(6.0);
            ui.label(match &offer.profile {
                Some(name) => tr!("frame-bindings-offer-profile", name = name),
                None => tr!("frame-bindings-offer-default"),
            });
            ui.add_space(6.0);
            ui.label(tr!("frame-bindings-offer-choice"));
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                if ui
                    .button(
                        RichText::new(tr!("frame-bindings-use")).color(crate::theme::ALERT()),
                    )
                    .clicked()
                {
                    adopt = true;
                }
                if ui.button(tr!("frame-bindings-keep-mine")).clicked() {
                    decline = true;
                }
            });
        });
        if adopt {
            // The opt-in itself is persisted, so this is asked once and not
            // again after a restart or an eviction.
            self.ui_settings.client_share_bindings = true;
            crate::app::persist::persist_ui_settings(&self.ui_settings);
            self.input.cfg = offer.bindings;
            self.input.cfg.migrate();
            self.input.persist();
            self.client_settings_from = Some(offer.profile);
            self.client_settings_status =
                Some(tr!("frame-bindings-using-profile"));
            self.bindings_pending = None;
        } else if decline {
            // **"and stop asking" has to mean it.** Dropping the offer is not
            // an answer: the yes branch persists the opt-in, so the no branch
            // persisting nothing meant this offer came back on the next session
            // and on every other radio of the station — which is the report on
            // the fork's #18, "displays all the time, already reported".
            self.ui_settings.client_bindings_declined = true;
            crate::app::persist::persist_ui_settings(&self.ui_settings);
            self.client_settings_status =
                Some(tr!("frame-bindings-keeping-own"));
            self.bindings_pending = None;
        }
    }
}

impl SdroxideApp {
    /// Put the profile's stored look back, discarding local changes to it.
    pub(in crate::app) fn revert_screen_to_profile(&mut self) {
        match self.client_settings_stored.clone() {
            Some(stored) => {
                stored.apply_to(&mut self.ui_settings);
                self.view.center_on_vfo = stored.center_on_vfo;
                self.view.fft_size = stored.fft_size;
                crate::app::persist::persist_ui_settings(&self.ui_settings);
                self.client_settings_status =
                    Some(tr!("frame-look-back-to-profile"));
            }
            None => {
                self.client_settings_status =
                    Some(tr!("frame-look-none-stored"));
            }
        }
    }

    /// Send this client's control bindings to the server, when the operator has
    /// opted in. A no-op otherwise, so the default-off path can never put a
    /// binding on the wire — the one opt-in left in this area, and the only
    /// part of a profile that can disturb another operator.
    pub(in crate::app) fn push_client_bindings_if_server(&mut self) {
        if !self.ui_settings.client_share_bindings {
            return;
        }
        let profile = self.client_settings_from.clone().flatten();
        self.ctrl.send_client_bindings(profile, self.input.cfg.clone());
    }

    /// Dispatch keyboard and mouse-button bindings for this frame.
    ///
    /// The bindings themselves live in `input.json` (see
    /// [`crate::input::InputRuntime`]); the shipped defaults reproduce the
    /// shortcuts that used to be hardcoded here — ←/→ ±100 Hz (Shift: ±10),
    /// ↑/↓ ±1 kHz, PgUp/PgDn ±10 kHz, M mute, N noise blanker, F fit span.
    fn control_inputs(&mut self, ctx: &egui::Context, now: f64, cmds: &mut Vec<Command>) {
        let mut speech_acts: Vec<sdroxide_types::Action> = Vec::new();
        let mut open_workspaces = false;
        let mut flip_broadcast_labels = false;
        // Destructured rather than borrowed field-by-field: the runtime needs
        // `state` and the window flags mutably at the same time, and they are
        // disjoint parts of `self`.
        let SdroxideApp {
            input,
            state,
            view,
            help,
            show_settings,
            show_logbook,
            show_spots,
            show_memories,
            show_voice,
            caps,
            wide_frame,
            ui_settings: _,
            ..
        } = self;
        // Read before the borrow below, which takes `self` apart.
        let rig_squelch = caps.as_ref().is_some_and(|c| c.commands_squelch);
        // How far a bound zoom-out may go: the full-band lane where the front
        // end has one, else its passband. Read here for the same reason
        // `rig_squelch` is — after this the borrow has taken `self` apart.
        let stated = caps.as_ref().map_or(0.0, |c| c.wide_span_hz);
        let seen = wide_frame.as_ref().map(|f| (f.center_hz, f.span_hz));
        let zoom_out = match (seen, stated > state.sample_rate) {
            (Some((c, span)), _) if span.max(stated) > state.sample_rate => (c, span.max(stated)),
            (None, true) => (state.center_hz, stated),
            _ => (state.center_hz, state.sample_rate),
        };
        let mut sink = crate::input::UiSink {
            view,
            help: &mut help.open,
            settings: show_settings,
            logbook: show_logbook,
            spots: show_spots,
            memories: show_memories,
            voice: show_voice,
            workspaces: &mut open_workspaces,
            broadcast_labels: &mut flip_broadcast_labels,
            speech: &mut speech_acts,
            rig_squelch,
            zoom_out,
        };
        input.poll_pointer_and_keys(ctx, state, &mut sink, cmds);
        #[cfg(not(target_arch = "wasm32"))]
        input.poll_midi(ctx, state, &mut sink, cmds);
        drop(sink);
        if open_workspaces {
            // Open the main Settings dialog on the UI page, which is where the
            // saved window arrangements live.
            self.show_settings = true;
            self.settings_tab = crate::app::SettingsTab::Ui;
        }
        if flip_broadcast_labels {
            // The same write the SPOTS window's BC chip makes, and persisted the
            // same way — a bound key that forgot itself at the next start would
            // be a switch that works once.
            self.ui_settings.broadcast_labels_all = !self.ui_settings.broadcast_labels_all;
            crate::app::persist::persist_ui_settings(&self.ui_settings);
        }
        for act in speech_acts {
            self.apply_speech_action(act, now);
        }
    }

    /// Answer a speech hotkey.
    fn apply_speech_action(&mut self, act: sdroxide_types::Action, now: f64) {
        use sdroxide_types::Action::*;
        match act {
            SpeakStatus => self.speech.announcer.say_status(&self.state, self.meters.as_ref(), now),
            SpeakRepeat => self.speech.announcer.repeat_last(now),
            SpeechSilence => self.speech.announcer.silence(),
            SpeechToggle => {
                let mut cfg = self.speech.settings().clone();
                cfg.enabled = !cfg.enabled;
                let turning_on = cfg.enabled;
                self.speech.set_settings(cfg.clone());
                crate::app::persist::persist_speech_settings(&cfg);
                // Confirm out loud when switching on; switching off is its own
                // confirmation, and speaking after being told to stop would be
                // a poor joke.
                if turning_on {
                    self.speech.announcer.say_sample(now);
                }
            }
            // `apply_action` only ever collects the four above.
            _ => {}
        }
    }

    /// De-assert every held control. Closing the window while a footswitch or
    /// a bound key is down must not leave the transmitter keyed.
    pub(in crate::app) fn release_held_controls(&mut self, cmds: &mut Vec<Command>) {
        let SdroxideApp {
            input,
            state,
            view,
            help,
            show_settings,
            show_logbook,
            show_spots,
            show_memories,
            show_voice,
            caps,
            cw_key_down,
            ui_settings: _,
            ..
        } = self;
        // The keyboard straight key is held by the operator's hand rather than
        // by an input binding, so `release_all` knows nothing about it: without
        // this, switching away from the tab with the key down left the carrier
        // on until the operator came back (issue #322). The *mode* stays
        // engaged — only the key goes up.
        if *cw_key_down {
            *cw_key_down = false;
            cmds.push(Command::CwKey(false));
        }
        let rig_squelch = caps.as_ref().is_some_and(|c| c.commands_squelch);
        let mut sink = crate::input::UiSink {
            view,
            help: &mut help.open,
            settings: show_settings,
            logbook: show_logbook,
            spots: show_spots,
            memories: show_memories,
            voice: show_voice,
            // Releases only: nothing is pressed on this path, so no action can
            // ask for the workspaces. The sink's field is still filled so the
            // two constructions cannot drift apart.
            workspaces: &mut false,
            broadcast_labels: &mut false,
            speech: &mut Vec::new(),
            rig_squelch,
            // Releasing held keys never pans or zooms, so the passband will do.
            zoom_out: (state.center_hz, state.sample_rate),
        };
        input.release_all(state, &mut sink, cmds);
    }
}

#[cfg(test)]
mod tests {
    use eframe::egui;
    use sdroxide_types::{Band, Mode};

    use super::{
        BindingsAction, bindings_action, digi_split, panadapter_pill_rects, qsy_clears_decodes,
        waterfall_bw_text,
    };

    /// **The reported bug (#215): SSTV opened two windows, `SSTV` and `IMAGE`, on
    /// the same panel.**
    ///
    /// SSTV and SSTV-FM have a window of their own; `IMAGE` belongs to the image
    /// modes that have none. Every image mode therefore uses exactly one of them,
    /// and this pins both halves so an auto-open that matches `is_image()` again
    /// cannot bring the duplicate back.
    #[test]
    fn an_image_mode_opens_one_window_and_never_both() {
        use super::{ImageWindow, image_window_for};
        let image_modes: Vec<Mode> = Mode::ALL.into_iter().filter(|m| m.is_image()).collect();
        assert!(
            image_modes.len() >= 3,
            "the image modes went missing from Mode::ALL: {image_modes:?}"
        );
        for m in &image_modes {
            let want = if m.is_sstv() { ImageWindow::Sstv } else { ImageWindow::Image };
            assert_eq!(image_window_for(*m), want, "{m:?} would open the wrong window");
            // And never both: the choice is a single enum, so this is the whole
            // property — but assert the two modes that share a panel are on
            // opposite sides of it, which is what the report was about.
            assert!(
                image_window_for(*m) != image_window_for(Mode::Rifp) || !m.is_sstv(),
                "{m:?} shares RIFP's window as well as its own"
            );
        }
        assert_eq!(image_window_for(Mode::Sstv), ImageWindow::Sstv);
        assert_eq!(image_window_for(Mode::SstvFm), ImageWindow::Sstv);
        assert_eq!(image_window_for(Mode::Rifp), ImageWindow::Image);
    }

    /// **CTR centres the picture in the image modes and the dial elsewhere.**
    /// A weather chart lives 1900 Hz above the carrier; centring the carrier
    /// leaves the picture in one margin, which is what was reported on #239.
    #[test]
    fn ctr_centres_the_picture_in_the_image_modes() {
        use sdroxide_types::Mode;
        assert!(super::SdroxideApp::centres_on_picture(Mode::Sstv));
        assert!(super::SdroxideApp::centres_on_picture(Mode::Wefax));
        assert!(!super::SdroxideApp::centres_on_picture(Mode::Usb));
        assert!(!super::SdroxideApp::centres_on_picture(Mode::Ft8));
    }

    /// **The round trip, in the window grammar.** No state is a one-way door:
    /// the docked pane goes out to a float with its own chip, the float has a
    /// title bar with roll-up, out to a window and close (which docks), and the
    /// OS window — which on a tiling desktop has no title bar at all — carries
    /// one dock picture in its own content.
    ///
    /// The float is deliberately **not** a button on the OS window any more: that
    /// state was reached by a chip, then by a row of chips, and with the title bar
    /// it is reached from the pane's own chip or the window list. One glyph per
    /// window is the whole point of this pass.
    #[test]
    fn every_state_of_the_operating_pane_has_a_way_out_of_it() {
        let src = include_str!("frame.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap_or(src.len())];
        // The float's title bar carries all three: roll up, out, close.
        assert!(
            code.contains("frame-window-panel-hint"),
            "the floating panel has no way out to a real window"
        );
        assert!(
            code.contains("frame-roll-up-hint"),
            "the floating panel cannot be rolled up, so it costs the laptop its picture"
        );
        assert!(
            code.contains("frame-dock-panel-hint"),
            "the floating panel has no way back to the pane"
        );
        // Its bar is ours, because egui has no hook for a fourth title-bar button.
        assert!(
            code.contains(".title_bar(false)"),
            "the floating panel is back to egui's title bar, which cannot carry the out button"
        );
        // The OS window's one glyph, and the one state at a time.
        assert!(
            code.contains("set_detached(sdroxide_types::DetachableModule::Panel, false)"),
            "the panel's OS window cannot be brought back"
        );
        assert!(
            code.contains(
                "self.panel_floating = false;\n            self.detach_operating_panel();"
            ),
            "floating and windowed can both be true, so the pane is drawn twice"
        );
        // And the docked pane's own way out to a float still exists in the
        // panels that carry it.
        assert!(
            include_str!("panels/mod.rs").contains("panels-float-chip-hint"),
            "no panel can float any more"
        );
    }

    /// **A detached window answers the layout questions for itself.** No
    /// headless test can open a second OS window — the harness runs embedded
    /// viewports, where `detached_viewport` draws nothing at all — so the one
    /// line that scopes the answers is pinned by reading it. Drop it and every
    /// child window silently goes back to laying itself out from the main
    /// window's geometry: a small panel sized as though it were desktop-wide,
    /// its chips running past its right edge, and a split view drawing the
    /// last tab's radio salt in the focused radio's windows.
    #[test]
    fn a_detached_window_scopes_the_layout_answers_to_itself() {
        let src = include_str!("frame.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap_or(src.len())];
        assert!(
            code.contains("let _scope = child.scope(ui);"),
            "the child viewport publishes the main window's answers again"
        );
        // …and the floating panel, which is a window inside this one and is
        // where a desktop-wide chip row first ran past its own edge.
        assert!(
            code.contains("ChildWindow::panes(self).scope(ui)"),
            "the floating panel lays itself out from the main window again"
        );
        // A panel window takes the one-pane arrangement when it is too narrow
        // for two, and then *it* carries the chips that reach the other panes:
        // the main window's row is behind it, and its waterfall tab has nothing
        // to show in a panel window. Both the detached window and the floating
        // one draw the row with no waterfall tab.
        assert_eq!(
            code.matches("self.digi_tabs(ui, mode, false);").count(),
            2,
            "a narrow panel window lost the chips that reach its other panes"
        );
        assert_eq!(
            code.matches("self.digi_tabs(ui, mode, true);").count(),
            1,
            "the main window's row keeps the waterfall tab"
        );
        // Every module says which shape it is, and the type is what enforces
        // that: `detached_viewport` takes a `ChildWindow`, so a call cannot be
        // written without one. The two desks are the console, the band selector
        // and the rotator.
        assert_eq!(
            code.matches("ChildWindow::desk(self)").count(),
            3,
            "a desk tool stopped saying so and will wear a compact strip"
        );
    }

    /// **A narrow panel's header lays out as one wrapped row, not three zones.**
    ///
    /// The operator's photograph of a floating panel: *"look at the top right.. that
    /// looks messy"* — the FT8 header's three fixed thirds, with the centre group
    /// wider than a third of a 540 pt panel and spilling under the chips pinned to
    /// the right-hand third. Two things were wrong, and the icon was one of them: it
    /// was drawn inside the floating panel as well, so two arrows sat side by side
    /// saying the same thing.
    #[test]
    fn a_narrow_panel_header_wraps_instead_of_overlapping_its_zones() {
        let src = include_str!("panels/decodes.rs");
        assert!(
            src.contains("let roomy = row.width() >= Self::HEADER_ZONE_MIN_W;"),
            "the header is back to three fixed thirds at every width, so a narrow \
             panel overlaps itself again"
        );
        assert!(
            src.contains("ui.horizontal_wrapped(|ui| {"),
            "the narrow header has no wrapped row"
        );
        assert!(
            src.contains("if !me.panel_in_a_window(me.state.rx[0].mode) && super::window_chip(ui)"),
            "the way-out icon is drawn in a panel that has already been lifted out"
        );
        assert!(
            src.contains("const HEADER_ZONE_MIN_W: f32 = 520.0;"),
            "the zone threshold changed; re-measure it against the header's content"
        );
    }

    /// **The pane has its own way out, in the header that is crowded — and it
    /// floats, it does not open an OS window.**
    ///
    /// The operator's correction on the first version: *"you open a new window
    /// whereas it should be a floating panel in the first place."* So the icon
    /// calls `float_operating_panel`, which opens the mode's in-window tool window;
    /// `detach_operating_panel` stays for Settings → Detachable, which is for a
    /// panel that wants another monitor.
    ///
    /// He asked for the control at the top right of the panel rather than in
    /// Settings — *"it gets crowded in some modes in the top of that
    /// pane"* — and the two other shapes were tried and rejected: a row above the
    /// panel took a chip off WEFAX's header at 600 pt, and a 21 pt gutter did the
    /// same by wrapping it, both caught by
    /// `the_fax_header_row_does_not_draw_on_itself`. It goes where the space was
    /// made for it: the right-hand end of a panel's own header row, in place of the
    /// word that chip used to spell.
    #[test]
    fn the_pane_has_its_way_out_in_the_header_that_is_crowded() {
        let src = include_str!("frame.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap_or(src.len())];
        assert!(
            code.contains("fn float_operating_panel"),
            "there is no way to float the operating pane"
        );
        // **The floating panel is the pane itself**, not a per-mode tool window:
        // the first version opened the mode's own window, so a mode without one
        // (FT8) got a chip that did nothing.
        assert!(
            code.contains("fn floating_panel_window"),
            "the pane has no floating form, so the icon hides it and shows nothing"
        );
        assert!(
            code.contains("self.draw_operating_panel(ui, cmds, mode, h);"),
            "the floating panel does not draw the operating panel"
        );
        assert!(
            !code.contains("pub(in crate::app) fn float_operating_panel(&mut self, mode: Mode)"),
            "floating is per mode again, so a mode without a window silently loses its panel"
        );
        assert!(
            code.contains("fn detach_operating_panel"),
            "the OS-window path (Settings → Detachable) went missing"
        );
        assert!(
            code.contains("set_detached(sdroxide_types::DetachableModule::Panel, true)"),
            "the pane's way out does not detach the operating panel"
        );
        // And the panels that carry it, in their own header rows.
        for (name, panel) in [
            ("decodes.rs", include_str!("panels/decodes.rs")),
            ("js8.rs", include_str!("panels/js8.rs")),
            ("atchat.rs", include_str!("panels/atchat.rs")),
            ("packet.rs", include_str!("panels/packet.rs")),
            ("adsb.rs", include_str!("panels/adsb.rs")),
        ] {
            assert!(panel.contains("super::window_chip(ui)"), "{name} has no way out of its pane");
        }
        // **The shape that broke WEFAX must not come back**: the pane is drawn at
        // the height it was given, with nothing above it.
        assert!(
            code.contains("self.operating_panel(ui, cmds, mode, panel_h);"),
            "the operating panel is no longer drawn at its own height"
        );
    }

    /// And the other half of the crowding: **the setup chip is a gear, without the
    /// word.** A chip that is already a symbol does not need to spell itself out in
    /// a header the operator says is crowded, and every one of them keeps a hover
    /// that names it.
    #[test]
    fn the_setup_chip_is_a_gear_without_the_word() {
        for (name, src) in [
            ("decodes.rs", include_str!("panels/decodes.rs")),
            ("js8.rs", include_str!("panels/js8.rs")),
            ("packet.rs", include_str!("panels/packet.rs")),
            ("atchat.rs", include_str!("panels/atchat.rs")),
            ("adsb.rs", include_str!("panels/adsb.rs")),
        ] {
            assert!(!src.contains("\"⚙ SETUP\""), "{name}: the setup chip still spells out SETUP");
            assert!(!src.contains("\"SETUP\""), "{name}: a chip is still labelled SETUP");
        }
    }

    /// **#217 at the frame: entering a mode docks its panel, and opens nothing.**
    ///
    /// The report: *"When clicking a mode by default a window opens instead of
    /// opening in the pane below the waterfall what is and should be the
    /// default."* Every mode that used to pop a window is entered in turn and no
    /// window may open. The detachable windows all still exist — they are the
    /// operator's to open now.
    #[test]
    fn entering_a_mode_docks_its_panel_and_opens_no_window() {
        use crate::multi::frame_test_lock;
        use sdroxide_types::RadioController;
        let _guard = frame_test_lock();
        let dir = std::env::temp_dir().join(format!("sdroxide-dock-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };

        #[derive(Default)]
        struct Silent;
        impl RadioController for Silent {
            fn send(&mut self, _cmd: sdroxide_types::Command) {}
            fn poll_event(&mut self) -> Option<sdroxide_types::RadioEvent> {
                None
            }
        }

        let ctx = egui::Context::default();
        let mut app = super::SdroxideApp::new_tab(&ctx, None, None, Box::new(Silent), 0, true);
        // The modes that used to be opened on entry, and a few that never were, so
        // the sweep is not only the list it was written from.
        let modes = [
            Mode::Sstv,
            Mode::SstvFm,
            Mode::Rifp,
            Mode::Dab,
            Mode::Js8,
            Mode::Psk,
            Mode::Rtty,
            Mode::Olivia,
            Mode::Thor,
            Mode::Packet,
            Mode::Wspr,
            Mode::Pi4,
            Mode::Jtty,
            Mode::Ale,
            Mode::RfPaint,
            Mode::Adsb,
            Mode::Hfdl,
            Mode::Vdl2,
            Mode::Acars,
            Mode::Aprs,
            Mode::Fsq,
            Mode::Hell,
            Mode::Sstv,
        ];
        for mode in modes {
            app.state.rx[0].mode = mode;
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1280.0, 800.0),
                )),
                ..Default::default()
            };
            ctx.run_ui(input, |ui| {
                eframe::App::ui(&mut app, ui, &mut eframe::Frame::_new_kittest());
            })
            .drop_without_applying_deltas();
            let opened: Vec<&str> = [
                ("SSTV", app.show_sstv),
                ("IMAGE", app.show_image),
                ("DAB", app.show_dab),
                ("JS8", app.show_js8),
                ("ACARS", app.show_acars),
                ("APRS", app.show_aprs),
                ("HFDL", app.show_hfdl),
                ("VDL2", app.show_vdl2),
                ("AIS", app.show_ais),
                ("ADS-B", app.show_adsb),
                ("FSQ", app.show_fsq),
                ("HELL", app.show_hell),
                ("PACKET", app.show_packet),
                ("WSPR", app.show_wspr),
                ("PI4", app.show_pi4),
                ("TEXT", app.show_text),
                ("UVPACKET", app.show_uvpacket),
                ("JTTY", app.show_jtty),
                ("ALE", app.show_ale),
                ("RF PAINT", app.show_rf_paint),
            ]
            .into_iter()
            .filter(|(_, open)| *open)
            .map(|(name, _)| name)
            .collect();
            assert!(
                opened.is_empty(),
                "{mode:?} opened {opened:?} — a mode's panel belongs in the pane below \
                 the waterfall (#217)"
            );
        }
        assert_eq!(
            app.ui_settings.tool_windows_open & super::MODE_PANEL_WINDOWS,
            0,
            "entering modes wrote window bits into the settings again (#217)"
        );
    }

    /// The repair: the bits a previous run wrote are cleared **once**, and the
    /// version that makes it once is written — otherwise every restart would shut a
    /// window the operator had opened on purpose.
    #[test]
    fn the_mode_panel_bits_are_repaired_once_and_only_once() {
        use super::{MODE_PANEL_WINDOWS, TW_GRID, TW_SSTV, repair_mode_panel_windows};
        let mut ui = sdroxide_types::UiSettings::default();
        // What 2.1.0.x left behind: the panels it opened, plus GRID, which the
        // operator opened himself and which must survive.
        ui.tool_windows_open = MODE_PANEL_WINDOWS | TW_GRID;
        assert!(repair_mode_panel_windows(&mut ui), "the repair did nothing");
        assert_eq!(ui.tool_windows_repaired, 1);
        assert_eq!(
            ui.tool_windows_open & MODE_PANEL_WINDOWS,
            0,
            "a mode panel's window would still open on its own"
        );
        assert_eq!(
            ui.tool_windows_open & TW_GRID,
            TW_GRID,
            "a window the operator opened was closed by the repair"
        );
        // And never again: a window opened later stays open.
        ui.tool_windows_open |= TW_SSTV;
        assert!(!repair_mode_panel_windows(&mut ui), "the repair ran twice");
        assert_eq!(ui.tool_windows_open & TW_SSTV, TW_SSTV);
    }

    /// **Nothing opens a mode panel's window on the mode's behalf**, and the one
    /// that is drawn is the mode's own.
    ///
    /// This is the source half of #215 and #217 together: #215 was two windows
    /// because two auto-opens fired for one mode, and #217 is the auto-open itself
    /// — it put every panel in a window instead of the pane below the waterfall.
    /// With it gone the duplicate cannot happen, and `image_window_for` still
    /// decides which single window a mode would draw *if* one is open.
    #[test]
    fn nothing_opens_a_mode_panel_and_only_the_modes_own_image_window_draws() {
        let src = include_str!("frame.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap_or(src.len())];
        assert!(
            !code.contains("if mode.is_image() && self.show_image {"),
            "the IMAGE window is drawn on a bare is_image() again (#215)"
        );
        assert!(
            code.contains("image_window_for(mode) == ImageWindow::Image"),
            "the draw has to ask which window the mode uses"
        );
        // The docked panel asks the same decision, through the helper that reads
        // the mode's *own* window — the part that used to blank an SSTV panel when
        // RIFP's window had been left open.
        assert!(
            src.contains("fn image_window_open")
                && src.contains("ImageWindow::Sstv => self.show_sstv"),
            "the docked panel no longer asks which window the mode uses"
        );
        assert!(
            !code.contains("self.show_sstv = true; self.ui_settings.tool_windows_open |= TW_SSTV;"),
            "a window is opened on mode entry again (#217)"
        );
        assert!(
            !code.contains("if mode.is_js8() { self.show_js8 = true;"),
            "a window is opened on mode entry again (#217)"
        );
    }

    /// The reported bug, in one line: a profile's control bindings reached the
    /// server and were written to disk correctly, survived a restart, and were
    /// then **discarded at sign-in with nothing said** — because the opt-in that
    /// authorises using them is client-local, and in a browser it lives in
    /// storage the same eviction can empty.
    ///
    /// So the case that matters is not "opted in" and not "declined". It is
    /// "the flag is false because it was lost", and the only correct answer to
    /// that is to ask.
    #[test]
    fn stored_bindings_are_offered_rather_than_dropped() {
        // The lost flag: not opted in, but there are keys → offer.
        assert_eq!(bindings_action(false, 25, false), BindingsAction::Offer);
        // Deliberately declined, same session → do not nag.
        assert_eq!(bindings_action(false, 25, true), BindingsAction::AlreadyAsked);
        // A fresh session asks again, which is what makes "keep mine" mean
        // "this session" rather than "never, silently".
        assert_eq!(bindings_action(false, 25, false), BindingsAction::Offer);
    }

    /// Opted in — the path that always worked, and the one the report's author
    /// was told to check. Unchanged.
    #[test]
    fn an_opted_in_client_still_just_applies_them() {
        assert_eq!(bindings_action(true, 25, false), BindingsAction::Apply);
        assert_eq!(bindings_action(true, 0, true), BindingsAction::Apply);
    }

    /// An empty set is never offered: there would be no question with a possible
    /// answer, and adopting it would wipe the operator's own keys.
    #[test]
    fn an_empty_binding_set_is_not_offered() {
        assert_eq!(bindings_action(false, 0, false), BindingsAction::Ignore);
    }

    /// The reported bug: 20 m to 40 m left the previous band's decodes in the
    /// list, because the only thing that cleared it was a mode change and the
    /// digital-mode band buttons deliberately keep the mode.
    #[test]
    fn crossing_into_another_band_clears_the_decodes() {
        assert!(qsy_clears_decodes(Band::M20, Band::M40, Mode::Ft8));
        assert!(qsy_clears_decodes(Band::M40, Band::M20, Mode::Ft4));
        // Out of the amateur bands entirely is still somewhere else.
        assert!(qsy_clears_decodes(Band::M20, Band::Gen, Mode::Ft8));
    }

    /// Band-level, not a delta on the dial. Moving from the FT8 slot to the FT4
    /// slot, or into a DXpedition window, is the same band opening and the same
    /// stations — clearing there would empty the list every time the operator
    /// looked somewhere else in the segment.
    #[test]
    fn tuning_about_inside_a_band_clears_nothing() {
        assert!(!qsy_clears_decodes(Band::M20, Band::M20, Mode::Ft8));
        // …and the frequencies that motivates: both 20 m slots are one band.
        assert_eq!(Band::containing(14_074_000.0), Band::containing(14_080_000.0));
        // The pair that must *not* fold together, from the same reasoning.
        assert_ne!(Band::containing(14_074_000.0), Band::containing(7_074_000.0));
    }

    /// WSPR hops bands by itself once a slot when `wspr_hop` is set, and its
    /// spots each carry the frequency they were heard on, so there is nothing
    /// ambiguous to throw away. Clearing here would empty the list every two
    /// minutes.
    #[test]
    fn wspr_band_hopping_is_exempt() {
        assert!(!qsy_clears_decodes(Band::M20, Band::M40, Mode::Wspr));
        assert!(!qsy_clears_decodes(Band::M40, Band::M30, Mode::Wspr));
    }

    #[test]
    fn the_digi_split_always_fits_the_height_it_was_given() {
        // 277 pt is what the old independent clamps needed (190 panel + 80
        // waterfall + 7 handle); every height below it used to overflow, and a
        // phone in landscape has around 250 to give.
        //
        // The divider is the handle plus the gap either side of it — 7 + 2×5 on
        // a desktop, 7 + 2×7 on a touched layout. Both are checked: leaving the
        // gaps out is what hung the panel off the bottom of the window.
        for divider in [7.0f32, 17.0, 21.0] {
            for total in [120.0f32, 180.0, 200.0, 250.0, 277.0, 400.0, 900.0] {
                for fraction in [0.2f32, 0.5, 0.82] {
                    for row_h in [18.0f32, 34.0] {
                        let (wf, panel) = digi_split(total, divider, fraction, row_h);
                        assert!(wf >= 0.0 && panel >= 0.0, "negative split at {total}/{fraction}");
                        let used = wf + panel + divider;
                        assert!(
                            used <= total + 0.01,
                            "{total} pt tall, {fraction} panel, {divider} divider, {row_h} rows: \
                             asked for {used}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn a_roomy_window_still_honours_the_fraction_and_the_floors() {
        let (wf, panel) = digi_split(900.0, 7.0, 0.5, 18.0);
        assert!((panel - (893.0 * 0.5)).abs() < 0.01, "panel {panel} ignored the fraction");
        assert!(wf >= 80.0, "waterfall {wf} below its floor");
        // Dragged all the way down, the waterfall keeps its 80 pt minimum.
        let (wf, panel) = digi_split(900.0, 7.0, 0.99, 18.0);
        assert!(wf >= 80.0, "waterfall {wf} squeezed out");
        assert!(panel >= 242.0, "panel {panel} below its floor");
    }

    /// Issue #231: a 1366×768 screen is the *tablet* tier — touch metrics, rows
    /// half again as tall — and the panel's floor was a figure measured on a
    /// desktop. Fifty points short is all it takes: the transcript is handed
    /// less than its own scroll area will accept, overflows what it was given,
    /// and paints over the message row underneath.
    #[test]
    fn a_touched_layout_gets_a_taller_floor_under_the_panel() {
        // The window in the report: 1290×724, of which the digital area is 609
        // and the divider 21.
        let (_, desktop) = digi_split(609.0, 21.0, 0.46, 18.0);
        let (wf, touch) = digi_split(609.0, 21.0, 0.46, 34.0);
        assert!(touch > desktop, "the touched layout needs the taller floor");
        assert!(touch >= 295.0, "{touch} still squeezes the transcript out of its own minimum");
        assert!(wf >= 80.0, "waterfall {wf} squeezed out to pay for it");
        // A window with room to spare is untouched: the floor is a floor, not a
        // share.
        let (_, roomy) = digi_split(900.0, 21.0, 0.46, 34.0);
        assert!((roomy - (879.0 * 0.46)).abs() < 0.01, "the fraction still decides {roomy}");
        // And the floor can never take more than three fifths of the area,
        // however tall the rows: a waterfall is what the operator is watching.
        let (wf, panel) = digi_split(300.0, 21.0, 0.2, 34.0);
        assert!(panel <= 0.6 * 279.0 + 0.01, "floor {panel} ate the waterfall");
        assert!(wf > 0.0);
    }

    /// The detached window's overlay: MHz, trailing zeros trimmed, so
    /// 27.265 MHz reads as itself rather than as "27.265000 MHz".
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_detached_readout_trims_to_the_dial() {
        assert_eq!(super::readout_text(27_265_000.0), "27.265 MHz");
        assert_eq!(super::readout_text(144_800_000.0), "144.8 MHz");
        assert_eq!(super::readout_text(10_120_600.0), "10.1206 MHz");
    }

    /// SP1's toolbar is a header, not a share of the picture: enough room for
    /// the 21 pt readout, and small enough against the window it opens at that
    /// the waterfall still gets the window. This is the one arithmetic the bar
    /// has — a header that grows with the window is a header that eats it.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_sp1_toolbar_is_a_header_and_not_a_share_of_the_picture() {
        let h = super::SP1_TOOLBAR_H;
        assert!(h >= 28.0, "{h} pt will not hold the readout it carries");
        let window =
            super::detached_spec(sdroxide_types::DetachableModule::Panadapter).default_size[1];
        assert!(h < window * 0.1, "a {h} pt bar is a tenth of the {window} pt window it opens at");
    }

    /// The detached window opens at the operator's last geometry, or a wide
    /// default, and **comes back onto the monitor it is about to open on** —
    /// size and position both.
    ///
    /// The position half was the report behind issue #235: a window saved on a
    /// display that is no longer there (a laptop undocked from a second screen,
    /// a resolution or DPI change) opened exactly where nothing could be
    /// reached, and the operator was left with a settings window they could not
    /// access at all. Only the right and bottom edges are pulled in, so a
    /// monitor to the left of the primary — a negative x — still means what it
    /// says.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn the_detached_window_opens_fitting_the_monitor() {
        let (size, pos) = super::detached_geometry(None, None, [960.0, 540.0]);
        assert_eq!(size, eframe::egui::vec2(960.0, 540.0), "the wide default");
        assert!(pos.is_none(), "no stored position, none given");

        // A position past the right edge of a small monitor comes back inside it,
        // and the window still fits the screen it opened on.
        let stored =
            sdroxide_types::DetachedWindow { size: [960.0, 540.0], pos: Some([1920.0, 0.0]) };
        let (size, pos) = super::detached_geometry(
            Some(eframe::egui::vec2(800.0, 600.0)),
            Some(stored),
            [960.0, 540.0],
        );
        assert!(size.x <= 800.0 && size.y <= 600.0, "grew past the monitor: {size:?}");
        assert_eq!(pos, Some(eframe::egui::pos2(720.0, 0.0)), "still off the right edge");

        // A position that is already on the monitor is left exactly where the
        // operator put it — this is not a re-layout, it is a rescue.
        let stored =
            sdroxide_types::DetachedWindow { size: [640.0, 480.0], pos: Some([120.0, 90.0]) };
        let (_, pos) = super::detached_geometry(
            Some(eframe::egui::vec2(1920.0, 1080.0)),
            Some(stored),
            [640.0, 480.0],
        );
        assert_eq!(pos, Some(eframe::egui::pos2(120.0, 90.0)));

        // A monitor to the left of the primary is a real place, not a lost window.
        let stored =
            sdroxide_types::DetachedWindow { size: [640.0, 480.0], pos: Some([-1920.0, 40.0]) };
        let (_, pos) = super::detached_geometry(
            Some(eframe::egui::vec2(1920.0, 1080.0)),
            Some(stored),
            [640.0, 480.0],
        );
        assert!(pos.unwrap().x < 0.0, "a left-hand monitor was pulled to the primary");
    }

    /// The line under the clock names the width the ruler spans, in the units
    /// the ruler itself uses, so the picture and the number cannot disagree.
    #[test]
    fn the_bandwidth_line_reads_the_visible_span() {
        assert_eq!(waterfall_bw_text(148_000.0), "BW waterfall · 148.0 kHz");
        assert_eq!(waterfall_bw_text(2_400_000.0), "BW waterfall · 2.400 MHz");
        assert_eq!(waterfall_bw_text(300.0), "BW waterfall · 300 Hz");
    }

    /// Before the first spectrum frame there is no window yet, and a `--` is
    /// the one honest reading. A zero, a negative or a non-finite span must
    /// never print a bandwidth the picture does not have.
    #[test]
    fn an_unset_span_reads_as_no_bandwidth() {
        for span in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert_eq!(waterfall_bw_text(span), "BW waterfall · —", "{span}");
        }
    }

    /// The two pills are one stack: equal width, one right edge, the clock's
    /// margin above and the bandwidth line 4 pt beneath it.
    #[test]
    fn the_clock_pills_share_their_width_and_right_edge() {
        let area = egui::Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(800.0, 400.0));
        let (clock, bw) =
            panadapter_pill_rects(area, egui::vec2(221.0, 15.0), egui::vec2(222.0, 16.0));
        assert_eq!(clock.size(), bw.size(), "the stack must have one width");
        assert_eq!(clock.left(), bw.left());
        assert!(
            (clock.right() - (area.right() - 8.0)).abs() < 1e-3,
            "the clock's 8 pt right margin moved: {}",
            clock.right()
        );
        assert_eq!(clock.top(), area.top() + 8.0);
        assert_eq!(bw.top() - clock.bottom(), 4.0, "the line sits under the clock");
        // The wider text sets the width, plus the pill's 12 pt of padding.
        assert_eq!(clock.width(), 222.0 + 12.0);
    }

    /// BAND TX: same width and edge as the other two, twice their height,
    /// under the bandwidth line.
    #[test]
    fn the_band_tx_pill_is_double_height_under_the_bandwidth_line() {
        let area = egui::Rect::from_min_size(egui::pos2(10.0, 20.0), egui::vec2(800.0, 400.0));
        let (clock, bw, tx) =
            super::panadapter_pill_stack(area, egui::vec2(221.0, 15.0), egui::vec2(222.0, 16.0));
        assert_eq!(tx.width(), clock.width());
        assert_eq!(tx.right(), clock.right());
        assert_eq!(tx.height(), clock.height() * 2.0);
        assert_eq!(tx.top() - bw.bottom(), 4.0);
    }

    /// On UTC the clock drops its local half and is narrower than the
    /// bandwidth line; the pair takes the wider text's width rather than
    /// clipping the line or shrinking the pill.
    #[test]
    fn the_narrower_text_does_not_shrink_the_pills() {
        let area = egui::Rect::from_min_size(egui::pos2(0.0, 0.0), egui::vec2(800.0, 400.0));
        let (clock, bw) =
            panadapter_pill_rects(area, egui::vec2(120.0, 15.0), egui::vec2(210.0, 16.0));
        assert_eq!(clock.width(), 210.0 + 12.0);
        assert_eq!(bw.width(), clock.width());
        assert_eq!(clock.right(), bw.right());
    }
}

#[cfg(test)]
mod bindings_offer_tests {
    use super::bindings_should_offer;

    /// The button said "and stop asking", so a no has to end it — for the rest
    /// of the session *and* every session after, and on every radio of the
    /// station. Before this, only the yes persisted.
    #[test]
    fn saying_no_stops_the_asking_for_good() {
        assert!(bindings_should_offer(false, false), "a fresh client is asked once");
        // Asked this session: not again.
        assert!(!bindings_should_offer(true, false), "asked already this session");
        // Said no: not on any later session, which is the half that was missing.
        assert!(!bindings_should_offer(false, true), "declined, so never asked again");
        assert!(!bindings_should_offer(true, true), "and still not on this one");
    }

    /// Saying no is not saying yes. A declined client must keep its own keys,
    /// which is the whole reason the question exists, so the two must stay
    /// separate flags rather than one tri-state that is easy to read backwards.
    #[test]
    fn declining_is_not_opting_in() {
        let s = sdroxide_types::UiSettings::default();
        assert!(!s.client_share_bindings, "the opt-in is off by default");
        assert!(!s.client_bindings_declined, "and so is having been asked");
    }
}

impl SdroxideApp {
    pub(in crate::app) fn dsc_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open = self.tool_window(ctx, "dsc", "DSC", [640.0, 480.0], self.show_dsc, |me, ui| {
            let h = ui.available_height();
            me.dsc_panel(ui, cmds, h);
        });
        self.show_dsc = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_DSC;
        } else {
            self.ui_settings.tool_windows_open &= !TW_DSC;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    pub(in crate::app) fn navtex_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open = self.tool_window(
            ctx,
            "navtex",
            "NAVTEX",
            [900.0, 600.0],
            self.show_navtex,
            |me, ui| {
                let h = ui.available_height();
                me.navtex_panel(ui, cmds, h);
            },
        );
        self.show_navtex = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_NAVTEX;
        } else {
            self.ui_settings.tool_windows_open &= !TW_NAVTEX;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    pub(in crate::app) fn aprs_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open =
            self.tool_window(ctx, "aprs", "APRS", [760.0, 600.0], self.show_aprs, |me, ui| {
                let h = ui.available_height();
                me.aprs_panel(ui, cmds, h);
            });
        self.show_aprs = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_APRS;
        } else {
            self.ui_settings.tool_windows_open &= !TW_APRS;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    pub(in crate::app) fn wefax_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open =
            self.tool_window(ctx, "wefax", "WEFAX", [900.0, 600.0], self.show_wefax, |me, ui| {
                let h = ui.available_height();
                me.wefax_panel(ui, cmds, h);
            });
        self.show_wefax = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_WEFAX;
        } else {
            self.ui_settings.tool_windows_open &= !TW_WEFAX;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
}

impl SdroxideApp {
    pub(in crate::app) fn sstv_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open =
            self.tool_window(ctx, "sstv", "SSTV", [760.0, 600.0], self.show_sstv, |me, ui| {
                let h = ui.available_height();
                me.image_panel(ui, cmds, sdroxide_types::Mode::Sstv);
                let _ = h;
            });
        self.show_sstv = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_SSTV;
        } else {
            self.ui_settings.tool_windows_open &= !TW_SSTV;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    pub(in crate::app) fn dab_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open = self.tool_window(ctx, "dab", "DAB", [720.0, 560.0], self.show_dab, |me, ui| {
            let h = ui.available_height();
            me.dab_panel(ui, cmds, h);
        });
        self.show_dab = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_DAB;
        } else {
            self.ui_settings.tool_windows_open &= !TW_DAB;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    /// Radio Classic in a window: when the operator detached it (Settings →
    /// Detachable), or when it is switched on in a mode whose panel owns the
    /// slot under the waterfall, or on a phone, where there is no room to dock.
    pub(in crate::app) fn radio_classic_window(
        &mut self,
        ctx: &egui::Context,
        cmds: &mut Vec<Command>,
    ) {
        use crate::app::panels::radio_classic::{RadioClassicPlace, place};
        let phone = crate::layout::tier(ctx) == crate::layout::Tier::Phone;
        let mode = self.state.rx[0].mode;
        let wanted = place(self.ui_settings.radio_classic_on, self.show_radio_classic, mode, phone)
            == RadioClassicPlace::Window;
        let open = self.tool_window(
            ctx,
            "radio_classic",
            "RADIO CLASSIC",
            [1180.0, 440.0],
            wanted,
            |me, ui| {
                let h = ui.available_height();
                me.radio_classic_panel(ui, cmds, h);
            },
        );
        let mut changed = false;
        if wanted && !open {
            // Closed with its ✕. A detached panel goes back where it docks; one
            // that was only a window because the slot is taken is switched off,
            // or the ✕ would do nothing.
            if self.show_radio_classic {
                self.show_radio_classic = false;
            } else {
                self.ui_settings.radio_classic_on = false;
            }
            changed = true;
        }
        let bit = self.ui_settings.tool_windows_open & TW_RADIO_CLASSIC != 0;
        if bit != self.show_radio_classic {
            if self.show_radio_classic {
                self.ui_settings.tool_windows_open |= TW_RADIO_CLASSIC;
            } else {
                self.ui_settings.tool_windows_open &= !TW_RADIO_CLASSIC;
            }
            changed = true;
        }
        if changed {
            crate::app::persist::persist_ui_settings(&self.ui_settings);
        }
    }
    pub(in crate::app) fn js8_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open = self.tool_window(ctx, "js8", "JS8", [640.0, 480.0], self.show_js8, |me, ui| {
            let h = ui.available_height();
            me.js8_panel(ui, cmds, h);
        });
        self.show_js8 = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_JS8;
        } else {
            self.ui_settings.tool_windows_open &= !TW_JS8;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
}

impl SdroxideApp {
    pub(in crate::app) fn acars_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open =
            self.tool_window(ctx, "acars", "ACARS", [720.0, 560.0], self.show_acars, |me, ui| {
                let h = ui.available_height();
                me.acars_panel(ui, cmds, h);
            });
        self.show_acars = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_ACARS;
        } else {
            self.ui_settings.tool_windows_open &= !TW_ACARS;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    pub(in crate::app) fn hfdl_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open =
            self.tool_window(ctx, "hfdl", "HFDL", [720.0, 560.0], self.show_hfdl, |me, ui| {
                let h = ui.available_height();
                me.hfdl_panel(ui, cmds, h);
            });
        self.show_hfdl = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_HFDL;
        } else {
            self.ui_settings.tool_windows_open &= !TW_HFDL;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    pub(in crate::app) fn vdl2_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open =
            self.tool_window(ctx, "vdl2", "VDL2", [720.0, 560.0], self.show_vdl2, |me, ui| {
                let h = ui.available_height();
                me.vdl2_panel(ui, cmds, h);
            });
        self.show_vdl2 = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_VDL2;
        } else {
            self.ui_settings.tool_windows_open &= !TW_VDL2;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    pub(in crate::app) fn ais_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open = self.tool_window(ctx, "ais", "AIS", [720.0, 560.0], self.show_ais, |me, ui| {
            let h = ui.available_height();
            me.ais_panel(ui, cmds, h);
        });
        self.show_ais = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_AIS;
        } else {
            self.ui_settings.tool_windows_open &= !TW_AIS;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
}

impl SdroxideApp {
    pub(in crate::app) fn adsb_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open =
            self.tool_window(ctx, "adsb", "ADS-B", [720.0, 560.0], self.show_adsb, |me, ui| {
                let h = ui.available_height();
                me.adsb_panel(ui, cmds, h);
            });
        self.show_adsb = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_ADSB;
        } else {
            self.ui_settings.tool_windows_open &= !TW_ADSB;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    pub(in crate::app) fn fsq_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open = self.tool_window(ctx, "fsq", "FSQ", [720.0, 560.0], self.show_fsq, |me, ui| {
            let h = ui.available_height();
            me.fsq_panel(ui, cmds, h);
        });
        self.show_fsq = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_FSQ;
        } else {
            self.ui_settings.tool_windows_open &= !TW_FSQ;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    pub(in crate::app) fn hell_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open =
            self.tool_window(ctx, "hell", "HELL", [720.0, 560.0], self.show_hell, |me, ui| {
                let h = ui.available_height();
                me.hell_panel(ui, cmds, h);
            });
        self.show_hell = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_HELL;
        } else {
            self.ui_settings.tool_windows_open &= !TW_HELL;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    pub(in crate::app) fn packet_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open = self.tool_window(
            ctx,
            "packet",
            "PACKET",
            [720.0, 560.0],
            self.show_packet,
            |me, ui| {
                let h = ui.available_height();
                me.packet_panel(ui, cmds, h);
            },
        );
        self.show_packet = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_PACKET;
        } else {
            self.ui_settings.tool_windows_open &= !TW_PACKET;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
}

impl SdroxideApp {
    pub(in crate::app) fn wspr_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open =
            self.tool_window(ctx, "wspr", "WSPR", [640.0, 480.0], self.show_wspr, |me, ui| {
                let h = ui.available_height();
                me.wspr_panel(ui, cmds, h);
            });
        self.show_wspr = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_WSPR;
        } else {
            self.ui_settings.tool_windows_open &= !TW_WSPR;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    pub(in crate::app) fn pi4_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open = self.tool_window(ctx, "pi4", "PI4", [640.0, 480.0], self.show_pi4, |me, ui| {
            let h = ui.available_height();
            me.pi4_panel(ui, cmds, h);
        });
        self.show_pi4 = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_PI4;
        } else {
            self.ui_settings.tool_windows_open &= !TW_PI4;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    pub(in crate::app) fn text_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open =
            self.tool_window(ctx, "text", "TEXT", [640.0, 480.0], self.show_text, |me, ui| {
                let h = ui.available_height();
                me.text_modem_panel(ui, cmds, h);
            });
        self.show_text = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_TEXT;
        } else {
            self.ui_settings.tool_windows_open &= !TW_TEXT;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    pub(in crate::app) fn uvpacket_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open = self.tool_window(
            ctx,
            "uvpacket",
            "UVPACKET",
            [640.0, 480.0],
            self.show_uvpacket,
            |me, ui| {
                let h = ui.available_height();
                me.uvpacket_panel(ui, cmds, h);
            },
        );
        self.show_uvpacket = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_UVPACKET;
        } else {
            self.ui_settings.tool_windows_open &= !TW_UVPACKET;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
}

impl SdroxideApp {
    pub(in crate::app) fn jtty_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open =
            self.tool_window(ctx, "jtty", "JTTY", [640.0, 480.0], self.show_jtty, |me, ui| {
                let h = ui.available_height();
                me.jtty_panel(ui, cmds, h);
            });
        self.show_jtty = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_JTTY;
        } else {
            self.ui_settings.tool_windows_open &= !TW_JTTY;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    pub(in crate::app) fn ale_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open = self.tool_window(ctx, "ale", "ALE", [640.0, 480.0], self.show_ale, |me, ui| {
            let h = ui.available_height();
            me.ale_panel(ui, cmds, h);
        });
        self.show_ale = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_ALE;
        } else {
            self.ui_settings.tool_windows_open &= !TW_ALE;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    pub(in crate::app) fn rf_paint_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open = self.tool_window(
            ctx,
            "rfpaint",
            "RF PAINT",
            [640.0, 480.0],
            self.show_rf_paint,
            |me, ui| {
                let h = ui.available_height();
                me.rf_paint_panel(ui, cmds, h);
            },
        );
        self.show_rf_paint = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_RFPAINT;
        } else {
            self.ui_settings.tool_windows_open &= !TW_RFPAINT;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
    /// Is this mode's **own** image window open? The docked panel is drawn only
    /// when it is not — and only the window that mode actually uses counts, so a
    /// `IMAGE` left open from RIFP cannot blank SSTV's panel.
    fn image_window_open(&self, mode: Mode) -> bool {
        match image_window_for(mode) {
            ImageWindow::Sstv => self.show_sstv,
            ImageWindow::Image => self.show_image,
        }
    }

    pub(in crate::app) fn image_window(
        &mut self,
        ctx: &egui::Context,
        cmds: &mut Vec<Command>,
        mode: sdroxide_types::Mode,
    ) {
        let open =
            self.tool_window(ctx, "image", "IMAGE", [760.0, 600.0], self.show_image, |me, ui| {
                me.image_panel(ui, cmds, mode);
            });
        self.show_image = open;
        if open {
            self.ui_settings.tool_windows_open |= TW_IMAGE;
        } else {
            self.ui_settings.tool_windows_open &= !TW_IMAGE;
        }
        crate::app::persist::persist_ui_settings(&self.ui_settings);
    }
}
/// **Which of the two image windows a mode uses — never both.**
///
/// SSTV and SSTV-FM have had their own `SSTV` window since their panel was made
/// detachable. `IMAGE` arrived later, for the image modes that had no window of
/// their own (RIFP), and the auto-open that came with it matched **every** image
/// mode — so opening SSTV opened the same panel twice, once under each title
/// (#215, reported against 2.1.0.2.5). One decision, used by the auto-open, by the
/// draw and by the docked panel, is what keeps it to one.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(in crate::app) enum ImageWindow {
    /// The mode's own `SSTV` window (`TW_SSTV`).
    Sstv,
    /// The shared `IMAGE` window (`TW_IMAGE`), for an image mode without one.
    Image,
}

/// The window this image mode uses. See [`ImageWindow`]: exactly one, always.
pub(in crate::app) fn image_window_for(mode: Mode) -> ImageWindow {
    if mode.is_sstv() { ImageWindow::Sstv } else { ImageWindow::Image }
}

/// The bits the program itself used to set: every mode panel it opened on the
/// operator's behalf. See [`repair_mode_panel_windows`] and issue #217.
pub const MODE_PANEL_WINDOWS: u64 = TW_DSC
    | TW_NAVTEX
    | TW_WEFAX
    | TW_SSTV
    | TW_DAB
    | TW_JS8
    | TW_ACARS
    | TW_APRS
    | TW_HFDL
    | TW_VDL2
    | TW_AIS
    | TW_ADSB
    | TW_FSQ
    | TW_HELL
    | TW_PACKET
    | TW_WSPR
    | TW_PI4
    | TW_TEXT
    | TW_UVPACKET
    | TW_JTTY
    | TW_ALE
    | TW_RFPAINT
    | TW_IMAGE;

/// **Dock the mode panels again, once** (issue #217).
///
/// Until this repair, choosing a mode opened its panel as a window and wrote that
/// into [`sdroxide_types::UiSettings::tool_windows_open`], so the bits a previous
/// run left behind cannot be told apart from the operator's own. They are cleared
/// once — the version in [`sdroxide_types::UiSettings::tool_windows_repaired`]
/// makes it once — and from then on a panel window is open only because somebody
/// opened it. Returns true when it changed anything, so the caller can save.
pub(in crate::app) fn repair_mode_panel_windows(ui: &mut sdroxide_types::UiSettings) -> bool {
    if ui.tool_windows_repaired >= 1 {
        return false;
    }
    ui.tool_windows_open &= !MODE_PANEL_WINDOWS;
    ui.tool_windows_repaired = 1;
    true
}

// Tool window bit indices (persisted in UiSettings.tool_windows_open).
pub const TW_DSC: u64 = 1 << 0;
pub const TW_NAVTEX: u64 = 1 << 1;
pub const TW_APRS: u64 = 1 << 2;
pub const TW_WEFAX: u64 = 1 << 3;
pub const TW_SSTV: u64 = 1 << 4;
pub const TW_DAB: u64 = 1 << 5;
pub const TW_JS8: u64 = 1 << 6;
pub const TW_ACARS: u64 = 1 << 7;
pub const TW_HFDL: u64 = 1 << 8;
pub const TW_VDL2: u64 = 1 << 9;
pub const TW_AIS: u64 = 1 << 10;
pub const TW_ADSB: u64 = 1 << 11;
pub const TW_FSQ: u64 = 1 << 12;
pub const TW_HELL: u64 = 1 << 13;
pub const TW_PACKET: u64 = 1 << 14;
pub const TW_WSPR: u64 = 1 << 15;
pub const TW_PI4: u64 = 1 << 16;
pub const TW_TEXT: u64 = 1 << 17;
pub const TW_UVPACKET: u64 = 1 << 18;
pub const TW_JTTY: u64 = 1 << 19;
pub const TW_ALE: u64 = 1 << 20;
pub const TW_RFPAINT: u64 = 1 << 21;
pub const TW_IMAGE: u64 = 1 << 22;

pub const TW_GRID: u64 = 1 << 23;
pub const TW_MEMORIES: u64 = 1 << 24;
pub const TW_EXTRAS: u64 = 1 << 25;
pub const TW_BANDS: u64 = 1 << 26;
pub const TW_CONTEST: u64 = 1 << 27;
pub const TW_DRM: u64 = 1 << 28;
pub const TW_ENIGMA: u64 = 1 << 29;
pub const TW_HD_RADIO: u64 = 1 << 30;
pub const TW_ISM: u64 = 1 << 31;
pub const TW_MORSE: u64 = 1 << 32;
pub const TW_RDS: u64 = 1 << 33;
pub const TW_SAT: u64 = 1 << 34;
pub const TW_SCANNER: u64 = 1 << 35;
pub const TW_SIGNAL_ID: u64 = 1 << 36;
pub const TW_SPOTS: u64 = 1 << 37;
pub const TW_SWL_LOG: u64 = 1 << 38;
pub const TW_WEATHER: u64 = 1 << 39;
pub const TW_MAIL: u64 = 1 << 40;
pub const TW_AWARDS: u64 = 1 << 41;
pub const TW_RECORDINGS: u64 = 1 << 42;
pub const TW_LISTEN: u64 = 1 << 43;
pub const TW_LOGBOOK: u64 = 1 << 44;
pub const TW_SETTINGS: u64 = 1 << 45;
pub const TW_SCHEDULE: u64 = 1 << 46;
pub const TW_PLAYBACK: u64 = 1 << 47;
pub const TW_PUBLICSDR: u64 = 1 << 48;
pub const TW_AIRCRAFT: u64 = 1 << 49;
pub const TW_RADIO_CLASSIC: u64 = 1 << 50;

/// Charge a failed interface change to the public-SDR receiver just tried, so
/// its row reads "could not connect" instead of being offered again. The
/// engine's own reopen error begins with "Interface change failed". Returns
/// whether a receiver was marked.
fn charge_failed_public_sdr(
    notice: Option<&str>,
    last_try: &mut Option<String>,
    dead: &mut std::collections::HashSet<String>,
) -> bool {
    let failed = notice.is_some_and(|why| why.starts_with("Interface change failed"));
    if !failed {
        return false;
    }
    let Some(addr) = last_try.take() else {
        return false;
    };
    dead.insert(addr);
    true
}

#[cfg(test)]
mod public_sdr_dead_tests {
    use super::charge_failed_public_sdr;

    #[test]
    fn a_failed_interface_change_marks_the_receiver_just_tried_dead() {
        let mut last = Some("1.2.3.4:5555".to_string());
        let mut dead = std::collections::HashSet::new();
        assert!(charge_failed_public_sdr(
            Some("Interface change failed: connection timed out"),
            &mut last,
            &mut dead,
        ));
        assert!(dead.contains("1.2.3.4:5555"));
        assert!(last.is_none(), "the charge consumes the last-tried address");
    }

    #[test]
    fn an_ordinary_notice_does_not_mark_a_receiver_dead() {
        let mut last = Some("1.2.3.4:5555".to_string());
        let mut dead = std::collections::HashSet::new();
        assert!(!charge_failed_public_sdr(Some("some other notice"), &mut last, &mut dead));
        assert!(dead.is_empty());
        assert!(last.is_some());
    }

    #[test]
    fn a_failure_without_a_previous_pick_marks_nothing() {
        let mut last = None;
        let mut dead = std::collections::HashSet::new();
        assert!(!charge_failed_public_sdr(
            Some("Interface change failed: timeout"),
            &mut last,
            &mut dead
        ));
        assert!(dead.is_empty());
    }
}

#[cfg(test)]
mod detached_geometry_tests {
    use super::detached_geometry;
    use eframe::egui;

    /// **A window saved off the edge of the screen comes back onto it.**
    ///
    /// The size was fitted to the monitor and the position was not, so a window
    /// saved on a display that is no longer attached opened where nothing could be
    /// reached — reported as "I cannot access the settings at all" (issue #235).
    #[test]
    fn a_saved_position_off_the_screen_is_brought_back() {
        let monitor = Some(egui::vec2(1920.0, 1080.0));
        let saved = Some(sdroxide_types::DetachedWindow {
            size: [900.0, 700.0],
            pos: Some([5000.0, 3000.0]),
        });
        let (_, pos) = detached_geometry(monitor, saved, [800.0, 600.0]);
        let pos = pos.expect("a restored position");
        assert!(pos.x <= 1920.0 - 80.0, "x {:.0} is off the right edge", pos.x);
        assert!(pos.y <= 1080.0 - 60.0, "y {:.0} is off the bottom edge", pos.y);

        // A window on a monitor to the left, at a negative x, is left alone: that
        // is a real place, not a lost one.
        let left = Some(sdroxide_types::DetachedWindow {
            size: [900.0, 700.0],
            pos: Some([-1200.0, 40.0]),
        });
        let (_, pos) = detached_geometry(monitor, left, [800.0, 600.0]);
        let pos = pos.expect("a restored position");
        assert!(pos.x < 0.0, "a left-hand monitor was pulled to the primary");
        assert!((pos.y - 40.0).abs() < 0.5);
    }
}
pub const TW_SESSION_QSOS: u64 = 1 << 51;
