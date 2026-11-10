//! The grid tracker: the worked Maidenhead squares — or DXCC countries — on a map.
//!
//! The awards dashboard already tallies both grids and countries, and the 3D
//! view places entities on a globe, but neither says *where* a worked square is
//! on a flat map. This draws the log's 4-character squares as filled cells,
//! green where a QSL has come back and amber where it has not, so a gap in a
//! continent reads at a glance.
//!
//! A CB log carries no squares at all: an 11 m exchange is a country call and a
//! report, and the country is the leading digits of the callsign. So the map has
//! a **COUNTRY** mode as well, marking each worked DXCC entity at its nominal
//! centre — the same resolver the awards tally uses turns a CB call into its
//! country, so a CB operator and a ham see the same map in the form their band
//! uses. Which one it opens on is decided from the log.
//!
//! It is the listener's tool as much as a ham's, which is why the chip is not
//! hidden in SWL mode. A listener is not working anyone, so the live decode list
//! is offered as a second layer — heard rather than worked — behind a HEARD
//! toggle. The heard layer is drawn under the worked one so a square or country
//! that is both reads as worked.

use crate::i18n::tr;
use eframe::egui::{self, Rect, Sense, Ui, pos2, vec2};
use std::collections::HashSet;

use crate::theme;
use crate::widgets::worldmap::{MapView, alpha, draw_base, interact, wrap180};

use crate::app::SdroxideApp;

/// Below this size the map is not worth drawing and the grid unreadable.
pub const MIN_HEIGHT: f32 = 200.0;

/// A heard mark is drawn this alpha; the worked layers are solid enough to
/// out-read it, so a heard-and-worked square never looks merely heard.
const HEARD_ALPHA: f32 = 88.0;
/// Worked and confirmed marks are opaque. A frame of "not worked" would be the
/// whole world, and drawing that is drawing the sea.
const WORKED_ALPHA: f32 = 205.0;
/// A country has no extent on this map — only a nominal centre — so it is a
/// marker, not a filled cell. Held at a fixed size rather than scaled with the
/// zoom, because the centre is coarse and a marker that grows implies an
/// accuracy the position does not have.
const COUNTRY_R: f32 = 5.0;

/// Which shape the tracker draws.
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum TrackerMode {
    /// 4-character Maidenhead squares, which is what a ham log carries.
    #[default]
    Grid,
    /// DXCC countries, which is what a CB log carries — an 11 m exchange is a
    /// country call and a report, with no locator in it at all.
    Country,
}

/// The tracker's own state, owned by the app so the pan/zoom survives a close.
#[derive(Default)]
pub struct GridTracker {
    pub view: MapView,
    /// Shade the marks heard but not worked, from the live decode list.
    pub show_heard: bool,
    /// Squares or countries.
    pub mode: TrackerMode,
    /// Whether the opening default has been chosen. Set once, so a later log
    /// change never drags the operator off the mode they picked.
    picked: bool,
}

/// Everything the map draws, whichever shape is selected. Built by the caller
/// so the drawing stays a free function.
pub struct TrackerData<'a> {
    /// `(grid, confirmed)`.
    pub worked_grids: &'a [(String, bool)],
    /// 4-character squares heard on the live decode list.
    pub heard_grids: &'a HashSet<String>,
    /// `(lat, lon, confirmed, country name)`.
    pub worked_countries: &'a [(f64, f64, bool, &'static str)],
    /// `(lat, lon, country name)` heard on the live decode list.
    pub heard_countries: &'a [(f64, f64, &'static str)],
}

/// Which mode to open on.
///
/// A log with countries but no squares is a CB log, and showing it a grid map
/// is showing it nothing — the silent-empty-window failure this fork treats as
/// a bug. Any square at all keeps the grid view, because a log with squares is
/// a ham's even if it also has CB contacts.
pub fn default_mode(worked_grids: usize, worked_countries: usize) -> TrackerMode {
    if worked_grids == 0 && worked_countries > 0 { TrackerMode::Country } else { TrackerMode::Grid }
}

/// Draw the map. Returns the square or country under the pointer, for a hover
/// label.
pub fn draw(
    ui: &mut Ui,
    state: &mut GridTracker,
    data: &TrackerData,
    home: Option<(f64, f64)>,
    max_h: f32,
) -> Option<String> {
    let avail_w = ui.available_width();
    if avail_w < MIN_HEIGHT {
        return None;
    }
    let h = max_h.min(avail_w).max(MIN_HEIGHT);
    let (rect, resp) = ui.allocate_exact_size(vec2(avail_w, h), Sense::click_and_drag());
    if !ui.is_rect_visible(rect) {
        return None;
    }
    let p = ui.painter_at(rect);
    let map = theme::map();
    p.rect_filled(rect, 0.0, map.sea);

    let aspect = (rect.height() / rect.width()) as f64;
    // A tracker has no natural autofit — the marks span continents — so it
    // opens on the whole world and stays where the operator puts it.
    if !state.view.initialized {
        state.view.clat = 0.0;
        state.view.clon = 0.0;
        state.view.lon_span = 360.0;
        state.view.initialized = true;
    }
    state.view.clamp(aspect);
    if interact(ui, &mut state.view, &resp, aspect) {
        crate::repaint::animate(ui.ctx());
    }
    let (clat, clon, lon_span) = (state.view.clat, state.view.clon, state.view.lon_span);
    let lat_span = lon_span * aspect;

    let dot_r = draw_base(&p, rect, clat, clon, lon_span, lat_span, map);

    let project = |lat: f64, lon: f64| {
        let dlon = wrap180(lon - clon);
        pos2(
            rect.left() + (0.5 + (dlon / lon_span) as f32) * rect.width(),
            rect.top() + (0.5 - ((lat - clat) / lat_span) as f32) * rect.height(),
        )
    };

    let heard_col = alpha(theme::CYAN(), HEARD_ALPHA);
    let worked_col = alpha(theme::YELLOW(), WORKED_ALPHA);
    let confirmed_col = alpha(theme::GREEN(), WORKED_ALPHA);

    // The mark under the pointer, as (label, lat, lon). Decided after drawing
    // so the label paints on top; worked marks win over heard ones.
    let mut hovered: Option<(String, f64, f64)> = None;
    let pointer = resp.hover_pos();

    match state.mode {
        TrackerMode::Grid => {
            let cell = |lat: f64, lon: f64| -> Rect {
                cell_rect(rect, clat, clon, lon_span, lat_span, lat, lon)
            };
            // Heard first, under the worked squares: a square in both lists is
            // worked, and must not read as merely heard.
            for g in data.heard_grids {
                if let Some((lat, lon)) = sdroxide_types::grid_to_latlon(g) {
                    let r = cell(lat, lon);
                    if rect.intersects(r) {
                        p.rect_filled(r, 0.0, heard_col);
                    }
                }
            }
            for (g, confirmed) in data.worked_grids {
                if let Some((lat, lon)) = sdroxide_types::grid_to_latlon(g) {
                    let r = cell(lat, lon);
                    if rect.intersects(r) {
                        p.rect_filled(r, 0.0, if *confirmed { confirmed_col } else { worked_col });
                    }
                }
            }
            if let Some(m) = pointer {
                let mut best = f32::MAX;
                for g in data.worked_grids.iter().map(|(g, _)| g).chain(data.heard_grids.iter()) {
                    let Some((lat, lon)) = sdroxide_types::grid_to_latlon(g) else { continue };
                    let r = cell(lat, lon);
                    if r.contains(m) {
                        let d = r.center().distance(m);
                        if d <= best {
                            best = d;
                            hovered = Some((g.clone(), lat, lon));
                        }
                    }
                }
            }
        }
        TrackerMode::Country => {
            // Heard as open rings, under the worked markers.
            for (lat, lon, _) in data.heard_countries {
                let c = project(*lat, *lon);
                if rect.contains(c) {
                    p.circle_stroke(c, COUNTRY_R, (1.4, heard_col));
                }
            }
            for (lat, lon, confirmed, _) in data.worked_countries {
                let c = project(*lat, *lon);
                if rect.contains(c) {
                    // A sea-coloured halo, so a marker over the stippled land
                    // stays readable.
                    p.circle_filled(c, COUNTRY_R + 1.5, alpha(map.sea, 180.0));
                    p.circle_filled(
                        c,
                        COUNTRY_R,
                        if *confirmed { confirmed_col } else { worked_col },
                    );
                }
            }
            if let Some(m) = pointer {
                let mut best = COUNTRY_R * 1.8;
                for (lat, lon, _, name) in data.worked_countries {
                    let d = project(*lat, *lon).distance(m);
                    if d <= best {
                        best = d;
                        hovered = Some((name.to_string(), *lat, *lon));
                    }
                }
            }
        }
    }

    // Home last, over the marks, so the operator can find themselves.
    if let Some((lat, lon)) = home {
        let c = project(lat, lon);
        if rect.contains(c) {
            p.circle_filled(c, dot_r.max(3.0) + 1.0, alpha(map.home, 90.0));
            p.circle_filled(c, dot_r.max(2.5), map.home);
        }
    }

    if let Some((label, lat, lon)) = &hovered {
        let c = project(*lat, *lon);
        p.text(
            c + vec2(0.0, -COUNTRY_R - 3.0),
            egui::Align2::CENTER_BOTTOM,
            label,
            egui::FontId::monospace(11.0),
            alpha(map.hover, 235.0),
        );
    }
    hovered.map(|(label, _, _)| label)
}

/// The screen rectangle a 4-character square covers, centred on its
/// `grid_to_latlon` point.
///
/// A square is 2° of longitude by 1° of latitude, so every cell is the same
/// size at a given zoom and the projection stays linear in both axes. The
/// minimum of one point keeps a cell visible on a fully zoomed-out view, where
/// 2° would otherwise round to nothing.
fn cell_rect(
    rect: Rect,
    clat: f64,
    clon: f64,
    lon_span: f64,
    lat_span: f64,
    lat: f64,
    lon: f64,
) -> Rect {
    let dlon = wrap180(lon - clon);
    let c = pos2(
        rect.left() + (0.5 + (dlon / lon_span) as f32) * rect.width(),
        rect.top() + (0.5 - ((lat - clat) / lat_span) as f32) * rect.height(),
    );
    let w = (2.0 / lon_span * f64::from(rect.width())) as f32;
    let h = (1.0 / lat_span * f64::from(rect.height())) as f32;
    Rect::from_center_size(c, vec2(w.max(1.0), h.max(1.0)))
}

impl SdroxideApp {
    /// The worked squares on the map, cached by log length.
    ///
    /// Computed with **no band filter**, unlike `ensure_awards`. The AWARDS
    /// window has a band picker and this map has none, so reading the filtered
    /// tally would silently hide every square outside the band the operator
    /// happened to leave selected there — with nothing on this window to say
    /// why. Upstream fixed the same thing on its side of #613 (3348b389); this
    /// is the fork's copy of that fix, on a cache of its own because the
    /// countries below need the same unfiltered confirmation column.
    fn ensure_grid_squares(&mut self) {
        let len = self.qso_log.len();
        if self.grid_squares.as_ref().map(|(l, _)| *l) == Some(len) {
            return;
        }
        let awards = sdroxide_types::compute_awards(&self.qso_log, None, None);
        let squares = awards.grids.iter().map(|(g, s)| (g.clone(), s.confirmed)).collect();
        self.grid_squares = Some((len, squares));
    }

    /// The worked countries placed on the map, cached by log length.
    ///
    /// Resolved from the log's calls rather than from the awards tally's names,
    /// because the tally is keyed by name and a CB call's name does not always
    /// match the country file's — the resolver places both, so this does too.
    /// Confirmation comes from the same tally so the two cannot disagree.
    fn ensure_grid_countries(&mut self) {
        let len = self.qso_log.len();
        if self.grid_countries.as_ref().map(|(l, _)| *l) == Some(len) {
            return;
        }
        // The confirmation column comes from an **unfiltered** tally, for the same
        // reason the squares do: a mark that follows the AWARDS window's band
        // filter would disagree with the squares drawn beside it.
        let awards = sdroxide_types::compute_awards(&self.qso_log, None, None);
        let conf: std::collections::HashMap<&str, bool> =
            awards.dxcc.iter().map(|(n, s)| (n.as_str(), s.confirmed)).collect();
        let mut seen = HashSet::new();
        let mut pts = Vec::new();
        for q in &self.qso_log {
            if q.call.trim().is_empty() {
                continue;
            }
            if let Some(place) = sdroxide_types::resolve_place(&q.call)
                && seen.insert(place.name)
            {
                pts.push((
                    place.lat,
                    place.lon,
                    conf.get(place.name).copied().unwrap_or(false),
                    place.name,
                ));
            }
        }
        self.grid_countries = Some((len, pts));
    }

    pub(in crate::app) fn grid_tracker_window(&mut self, ctx: &egui::Context) {
        if !self.show_grid {
            return;
        }
        self.ensure_grid_countries();
        self.ensure_grid_squares();

        let worked_grids: Vec<(String, bool)> =
            self.grid_squares.as_ref().map(|(_, w)| w.clone()).unwrap_or_default();
        let worked_countries: Vec<(f64, f64, bool, &'static str)> =
            self.grid_countries.as_ref().map(|(_, v)| v.clone()).unwrap_or_default();

        // Choose the opening mode once, from what the log actually holds.
        if !self.grid_tracker.picked {
            self.grid_tracker.mode = default_mode(worked_grids.len(), worked_countries.len());
            self.grid_tracker.picked = true;
        }
        let mode = self.grid_tracker.mode;
        let show_heard = self.grid_tracker.show_heard;

        let heard_grids: HashSet<String> = if mode == TrackerMode::Grid && show_heard {
            self.digi_decodes
                .iter()
                .filter_map(|d| d.grid.as_deref().and_then(sdroxide_types::grid4))
                .collect()
        } else {
            HashSet::new()
        };
        let heard_countries: Vec<(f64, f64, &'static str)> =
            if mode == TrackerMode::Country && show_heard {
                let mut seen = HashSet::new();
                self.digi_decodes
                    .iter()
                    .filter_map(|d| d.from.as_deref())
                    .filter_map(sdroxide_types::resolve_place)
                    .filter(|p| seen.insert(p.name))
                    .map(|p| (p.lat, p.lon, p.name))
                    .collect()
            } else {
                Vec::new()
            };

        let home = {
            let g = self.my_grid();
            sdroxide_types::grid_to_latlon(&g)
        };

        let data = TrackerData {
            worked_grids: &worked_grids,
            heard_grids: &heard_grids,
            worked_countries: &worked_countries,
            heard_countries: &heard_countries,
        };
        let (worked_n, confirmed, heard_n) = match mode {
            TrackerMode::Grid => (
                worked_grids.len(),
                worked_grids.iter().filter(|(_, c)| *c).count(),
                heard_grids.len(),
            ),
            TrackerMode::Country => (
                worked_countries.len(),
                worked_countries.iter().filter(|(_, _, c, _)| *c).count(),
                heard_countries.len(),
            ),
        };

        // The shell's window: an egui window or its own OS one, with the ⇱
        // WINDOW chip — one of the station's windows rather than a dialog.
        //
        // The tracker's own state is borrowed out for the draw because the map
        // is painted from it while the shell holds the app, and the body is a
        // free function over that borrow rather than a method.
        let mut tracker = std::mem::take(&mut self.grid_tracker);
        let open = self.tool_window(
            ctx,
            "grid-tracker",
            &tr!("gridtracker-grid-tracker"),
            [760.0, 560.0],
            self.show_grid,
            |_me, ui| {
                grid_tracker_body(ui, &mut tracker, worked_n, confirmed, heard_n, &data, home)
            },
        );
        self.grid_tracker = tracker;
        self.show_grid = open;
    }
}

/// The grid tracker's body: the worked-squares map and the countries column.
///
/// A free function over the tracker's own state rather than an app method: the
/// shell borrows the app to draw the window while the tracker is borrowed out of
/// it for the paint, so the body cannot be reached through the app.
fn grid_tracker_body<'a>(
    ui: &mut egui::Ui,
    tracker: &mut GridTracker,
    worked_n: usize,
    confirmed: usize,
    heard_n: usize,
    data: &'a TrackerData<'a>,
    home: Option<(f64, f64)>,
) {
    crate::chrome::window_body_bg(ui);
    ui.horizontal(|ui| {
        // Which shape the log is read in. A CB operator wants
        // countries, a ham wants squares, and a mixed log can look
        // at either.
        if ui
            .selectable_label(tracker.mode == TrackerMode::Grid, tr!("gridtracker-grid-ham"))
            .on_hover_text(
                tr!("gridtracker-maidenhead-squares-what-a-ham-log"),
            )
            .clicked()
        {
            tracker.mode = TrackerMode::Grid;
        }
        if ui
            .selectable_label(tracker.mode == TrackerMode::Country, tr!("gridtracker-country-cb"))
            .on_hover_text(
                tr!("gridtracker-dxcc-countries-at-their-nominal"),
            )
            .clicked()
        {
            tracker.mode = TrackerMode::Country;
        }
        ui.separator();
        ui.label(
            egui::RichText::new(tr!("gridtracker-worked", worked = worked_n)).color(theme::YELLOW()).monospace(),
        );
        ui.label(
            egui::RichText::new(tr!("gridtracker-confirmed", confirmed = confirmed)).color(theme::GREEN()).monospace(),
        );
        if ui
            .selectable_label(
                tracker.show_heard,
                egui::RichText::new(tr!("gridtracker-heard", heard = heard_n)).color(theme::CYAN()).monospace(),
            )
            .on_hover_text(
                tr!("gridtracker-shade-squares-heard-live-decode"),
            )
            .clicked()
        {
            tracker.show_heard = !tracker.show_heard;
        }
    });
    ui.separator();
    draw(ui, tracker, &data, home, ui.available_height());
    ui.separator();
    ui.label(
        egui::RichText::new(
            tr!("gridtracker-drag-pan-wheel-zoom-amber"),
        )
        .size(10.0)
        .color(theme::gray(150)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_grid_cell_covers_its_own_two_by_one_square() {
        // World view sized 1 pt per degree, so the arithmetic is readable.
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(360.0, 180.0));
        let a = cell_rect(rect, 0.0, 0.0, 360.0, 180.0, 0.0, 0.0);
        assert!((a.center().x - 180.0).abs() < 0.01, "{}", a.center().x);
        assert!((a.center().y - 90.0).abs() < 0.01, "{}", a.center().y);
        assert!((a.width() - 2.0).abs() < 0.01, "{}", a.width());
        assert!((a.height() - 1.0).abs() < 0.01, "{}", a.height());
        // A square one degree east is one point east and no taller or wider.
        let b = cell_rect(rect, 0.0, 0.0, 360.0, 180.0, 0.0, 1.0);
        assert!((b.center().x - a.center().x - 1.0).abs() < 0.01);
        assert!((b.width() - a.width()).abs() < 0.01);
    }

    #[test]
    fn a_cell_across_the_antimeridian_wraps_the_short_way() {
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(360.0, 180.0));
        // View centred on +179: a square at -179 is 2° east of it, not 358°
        // west, which is what stops a pan across the seam scattering cells.
        let a = cell_rect(rect, 0.0, 179.0, 360.0, 180.0, 0.0, 179.0);
        let b = cell_rect(rect, 0.0, 179.0, 360.0, 180.0, 0.0, -179.0);
        assert!((b.center().x - a.center().x - 2.0).abs() < 0.01);
    }

    #[test]
    fn a_cb_log_opens_on_countries() {
        // The point of the mode: a CB log has no squares, and a grid map of it
        // is an empty window with no explanation.
        assert_eq!(default_mode(0, 0), TrackerMode::Grid);
        assert_eq!(default_mode(0, 12), TrackerMode::Country);
        // Any square at all is a ham's log, even with CB contacts beside it.
        assert_eq!(default_mode(1, 12), TrackerMode::Grid);
        assert_eq!(default_mode(40, 0), TrackerMode::Grid);
    }
}
