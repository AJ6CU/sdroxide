//! The radiosonde picture: every sonde the decoder has heard, drawn with the
//! track it has flown over the same dotted continents the ADS-B, AIS and HFDL
//! maps use ([`crate::widgets::worldmap`]).
//!
//! A sonde reports its own GPS position once a second, so unlike HFDL the map
//! has a real track to draw: a line from where it was first heard to where it
//! is now, a mark at the head, and beside it the serial, the altitude and the
//! climb rate. The table is the decoder's own ([`sdroxide_types::SondeStatus`]);
//! this map keeps only the view and the selection.

use crate::i18n::tr;
use eframe::egui::{Align2, Color32, FontId, Sense, Stroke, Ui, pos2, vec2};
use crate::sonde_online::OnlineSonde;
use sdroxide_types::SondeTrack;

use crate::hfdl_map::target_view;
use crate::theme;
use crate::widgets::map_labels::{self, MapLabel};
use crate::widgets::worldmap::{MapView, alpha, draw_base, interact, paint_night, wrap180};

/// Below this height the map is not worth drawing.
pub const MIN_HEIGHT: f32 = 90.0;

/// How close a click has to land, in points.
const HIT_RADIUS: f32 = 14.0;

/// Per-frame ease toward the auto-fit.
const EASE: f64 = 0.06;

/// A sonde not heard for this long is shown as lost rather than flying: after
/// landing it is silent, and its last position is where it came down.
pub const LOST_AFTER_S: i64 = 60;

/// The map's state: the view and the selected sonde, kept by the app so they
/// survive the window being closed.
#[derive(Default)]
pub struct SondeMapState {
    pub view: MapView,
    /// The serial of the sonde whose details are shown.
    pub selected: Option<String>,
}

/// What a sonde is doing, from its latest frame and how long ago it was heard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase {
    Ascending,
    Descending,
    /// No frame for [`LOST_AFTER_S`]: on the ground, or out of range.
    Silent,
}

pub fn phase(s: &SondeTrack, now: i64) -> Phase {
    if now - s.last_unix > LOST_AFTER_S {
        Phase::Silent
    } else if s.climb_ms.is_some_and(|c| c < -1.0) {
        Phase::Descending
    } else {
        Phase::Ascending
    }
}

fn phase_color(p: Phase, map: &theme::MapPalette) -> Color32 {
    match p {
        Phase::Ascending => map.station,
        Phase::Descending => map.dx,
        Phase::Silent => alpha(map.station, 110.0),
    }
}

/// One line of data under a sonde's name: altitude and climb.
pub fn summary(s: &SondeTrack) -> String {
    let alt = s.alt_m.map(|a| format!("{a:.0} m")).unwrap_or_else(|| "—".into());
    match s.climb_ms {
        Some(c) => format!("{alt}  {c:+.1} m/s"),
        None => alt,
    }
}

/// Draw the map. Returns the serial clicked this frame, if any.
pub fn show(
    ui: &mut Ui,
    state: &mut SondeMapState,
    sondes: &[SondeTrack],
    online: &[OnlineSonde],
    home: Option<(f64, f64)>,
    now: i64,
    night: Option<eframe::egui::TextureId>,
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
    let map = theme::map();
    let p = ui.painter_at(rect);
    p.rect_filled(rect, 0.0, map.sea);

    // Only sondes with a place can be drawn.
    let placed: Vec<(&SondeTrack, f64, f64)> =
        sondes.iter().filter_map(|s| Some((s, s.lat?, s.lon?))).collect();

    // ── the view ──
    let aspect = (rect.height() / rect.width()) as f64;
    let mut pts: Vec<(f64, f64)> = Vec::new();
    for (s, lat, lon) in &placed {
        pts.push((*lat, *lon));
        // The whole track frames, not just the head, so a long flight fits.
        pts.extend(s.track.iter().step_by(30).map(|q| (q.lat, q.lon)));
    }
    let (t_clat, t_clon, t_span) = target_view(home, &pts, aspect);
    let view = &mut state.view;
    if !view.initialized {
        view.clat = t_clat;
        view.clon = t_clon;
        view.lon_span = t_span;
        view.initialized = true;
    } else if view.manual {
        view.clamp(aspect);
    } else {
        view.clat += (t_clat - view.clat) * EASE;
        view.clon = wrap180(view.clon + wrap180(t_clon - view.clon) * EASE);
        view.lon_span += (t_span - view.lon_span) * EASE;
        let settled = (view.clat - t_clat).abs() < 0.02
            && wrap180(t_clon - view.clon).abs() < 0.02
            && (view.lon_span - t_span).abs() < 0.02;
        if !settled {
            crate::repaint::after_ms(ui.ctx(), 16);
        }
    }
    if interact(ui, view, &resp, aspect) {
        crate::repaint::animate(ui.ctx());
    }
    let (clat, clon, lon_span) = (view.clat, view.clon, view.lon_span);
    let lat_span = lon_span * aspect;

    let dot_r = draw_base(&p, rect, clat, clon, lon_span, lat_span, map);
    paint_night(&p, rect, clat, clon, lon_span, lat_span, night);

    if placed.is_empty() {
        p.text(
            rect.center(),
            Align2::CENTER_CENTER,
            tr!("sondemap-no-sonde-yet"),
            FontId::proportional(11.0),
            alpha(map.station, 160.0),
        );
    }

    let project = |lat: f64, lon: f64| {
        let dlon = wrap180(lon - clon);
        pos2(
            rect.left() + (0.5 + (dlon / lon_span) as f32) * rect.width(),
            rect.top() + (0.5 - ((lat - clat) / lat_span) as f32) * rect.height(),
        )
    };

    // The sonde under the pointer, decided before drawing.
    let pointer = resp.hover_pos();
    let mut hover: Option<usize> = None;
    let mut best = HIT_RADIUS;
    if let Some(m) = pointer {
        for (i, (_, lat, lon)) in placed.iter().enumerate() {
            let d = project(*lat, *lon).distance(m);
            if d <= best {
                best = d;
                hover = Some(i);
            }
        }
    }

    let font = FontId::monospace(9.0);
    let selected = state.selected.clone();

    // ── tracks, under the marks ──
    for (i, (s, _, _)) in placed.iter().enumerate() {
        let is_sel = selected.as_deref() == Some(s.serial.as_str());
        let tint = phase_color(phase(s, now), map);
        let w = if hover == Some(i) || is_sel { 2.2 } else { 1.4 };
        let line: Vec<_> = s.track.iter().map(|q| project(q.lat, q.lon)).collect();
        // Split where the line wraps the antimeridian, rather than drawing a
        // stroke across the whole map.
        for seg in line.windows(2) {
            if (seg[0].x - seg[1].x).abs() < rect.width() * 0.5 {
                p.line_segment([seg[0], seg[1]], Stroke::new(w, alpha(tint, 190.0)));
            }
        }
    }

    // ── the sondes other stations hear (SondeHub), hollow diamonds ──
    let online_ink = alpha(Color32::from_rgb(110, 165, 255), 230.0);
    for o in online.iter().filter(|o| !sondes.iter().any(|s| s.serial == o.serial)) {
        let c = project(o.lat, o.lon);
        if !rect.contains(c) {
            continue;
        }
        let r = 4.0;
        let d = vec![pos2(c.x, c.y - r), pos2(c.x + r, c.y), pos2(c.x, c.y + r), pos2(c.x - r, c.y)];
        p.add(eframe::egui::Shape::closed_line(d, Stroke::new(1.4, online_ink)));
        let alt = o.alt_m.map(|a| format!(" {a:.0} m")).unwrap_or_default();
        p.text(
            c + vec2(7.0, 0.0),
            Align2::LEFT_CENTER,
            format!("{}{alt}", o.serial),
            FontId::monospace(9.0),
            alpha(online_ink, 190.0),
        );
        if let Some(m) = pointer.filter(|m| m.distance(c) <= HIT_RADIUS) {
            let _ = m;
            let mut tip = format!("SondeHub  {} {}", o.model, o.serial);
            tip.push_str(&format!("
{:.4}, {:.4}", o.lat, o.lon));
            if let Some(a) = o.alt_m {
                tip.push_str(&format!("
{a:.0} m"));
            }
            if let Some(c) = o.climb_ms {
                tip.push_str(&format!("  {c:+.1} m/s"));
            }
            if let Some(f) = o.freq_mhz {
                tip.push_str(&format!("
{f:.3} MHz"));
            }
            resp.clone().on_hover_text(tip);
        }
    }

    // ── marks and labels ──
    let mut labels = Vec::new();
    for (i, (s, lat, lon)) in placed.iter().enumerate() {
        let c = project(*lat, *lon);
        let top = hover == Some(i);
        let is_sel = selected.as_deref() == Some(s.serial.as_str());
        let ph = phase(s, now);
        let tint = if is_sel {
            map.dx
        } else if top {
            map.hover
        } else {
            phase_color(ph, map)
        };
        let r = if top || is_sel { 5.0 } else { 4.0 };
        p.circle_filled(c, r + 2.0, alpha(map.sea, 170.0));
        match ph {
            // A balloon going up: a disc. Coming down under the parachute: a
            // triangle. Silent: a cross, where it was last heard.
            Phase::Ascending => {
                p.circle_stroke(c, r, (1.6, tint));
            }
            Phase::Descending => {
                let pts = vec![
                    pos2(c.x, c.y - r),
                    pos2(c.x - r, c.y + r * 0.8),
                    pos2(c.x + r, c.y + r * 0.8),
                ];
                p.add(eframe::egui::Shape::closed_line(pts, Stroke::new(1.6, tint)));
            }
            Phase::Silent => {
                p.line_segment([c + vec2(-r, -r), c + vec2(r, r)], (1.6, tint));
                p.line_segment([c + vec2(-r, r), c + vec2(r, -r)], (1.6, tint));
            }
        }
        if !rect.contains(c) && !top {
            continue;
        }
        labels.push(MapLabel {
            at: c,
            r,
            tick_from: 0.7,
            tick: alpha(tint, 110.0),
            lines: vec![
                (s.serial.clone(), alpha(tint, 240.0)),
                (summary(s), alpha(tint, 190.0)),
            ],
            must: top || is_sel,
            rank: if top || is_sel { 0 } else { 1 },
            key: i as u32,
        });
    }
    let marks: Vec<eframe::egui::Rect> = home
        .map(|(lat, lon)| eframe::egui::Rect::from_center_size(project(lat, lon), vec2(16.0, 16.0)))
        .into_iter()
        .collect();
    map_labels::draw(&p, rect, &font, labels, &marks);

    // ── us ──
    if let Some((lat, lon)) = home {
        let c = project(lat, lon);
        p.circle_filled(c, dot_r + 5.0, alpha(map.home, 55.0));
        p.circle_filled(c, 3.4, map.home);
        p.circle_stroke(c, 6.5, (1.2, alpha(map.home, 170.0)));
    }

    // ── interaction ──
    let mut clicked = None;
    if let Some(i) = hover {
        let (s, lat, lon) = &placed[i];
        let mut tip = format!("{}  {}\n{lat:.4}, {lon:.4}", s.model, s.serial);
        tip.push_str(&format!("\n{}", summary(s)));
        if let Some(v) = s.speed_ms {
            tip.push_str(&format!("\n{:.0} km/h", v * 3.6));
        }
        tip.push_str(&format!("
{:.3} MHz", s.freq_hz / 1e6));
        if let Some(v) = s.battery_v {
            tip.push_str(&format!("\n{v:.1} V"));
        }
        if let Some((hlat, hlon)) = home {
            let km = sdroxide_types::distance_km((hlat, hlon), (*lat, *lon));
            let bear = sdroxide_types::bearing_deg((hlat, hlon), (*lat, *lon));
            tip.push_str(&format!(
                "\n{}   {bear:.0}°",
                sdroxide_types::AirUnits::current().distance(km)
            ));
        }
        resp.clone().on_hover_text(tip);
        if resp.clicked() {
            state.selected = Some(s.serial.clone());
            clicked = Some(s.serial.clone());
        }
    } else if resp.clicked() {
        state.selected = None;
    }
    clicked
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sonde(climb: Option<f32>, last: i64) -> SondeTrack {
        SondeTrack {
            model: "RS41".into(),
            serial: "N1".into(),
            freq_hz: 403e6,
            frame: 1,
            lat: Some(48.0),
            lon: Some(2.0),
            alt_m: Some(1500.0),
            speed_ms: Some(5.0),
            course_deg: None,
            climb_ms: climb,
            num_sv: Some(9),
            battery_v: Some(2.7),
            last_unix: last,
            frames: 1,
            track: vec![],
        }
    }

    #[test]
    fn the_phase_follows_the_climb_and_the_silence() {
        assert_eq!(phase(&sonde(Some(5.0), 100), 110), Phase::Ascending);
        assert_eq!(phase(&sonde(Some(-8.0), 100), 110), Phase::Descending);
        assert_eq!(phase(&sonde(Some(-0.2), 100), 110), Phase::Ascending, "float, not a fall");
        assert_eq!(phase(&sonde(Some(5.0), 100), 100 + LOST_AFTER_S + 1), Phase::Silent);
    }

    #[test]
    fn the_summary_names_altitude_and_climb() {
        assert_eq!(summary(&sonde(Some(5.04), 0)), "1500 m  +5.0 m/s");
        assert_eq!(summary(&sonde(None, 0)), "1500 m");
    }
}
