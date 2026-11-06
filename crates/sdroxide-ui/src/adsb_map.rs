//! The ADS-B radar picture: every aircraft being tracked, drawn the way a
//! surveillance display draws one.
//!
//! The projection, the pan/zoom and the dotted continents are the FT8 panel
//! map's ([`crate::widgets::worldmap`]) — the views have to agree about where
//! the world is, and one of them owning that is how they do. What is here is
//! everything an air picture needs and a station map does not:
//!
//! - a **square** per target, not an icon. Everything up there is an aeroplane,
//!   so a symbol that said which kind would be saying nothing; what the square
//!   does is stay the same size and shape at every zoom, so a scan across a
//!   busy sector reads as one class of thing.
//! - **history dots** behind it — where it has been, fading with age. A
//!   controller reads speed and turn off the spacing of those dots before
//!   reading any number on the screen.
//! - a **leader line** ahead of it, as long as the distance it covers in
//!   [`sdroxide_types::AdsbSettings::vector_minutes`]. The length *is* the
//!   speed, which is why it is drawn in degrees of latitude and longitude
//!   before being projected: at a glance two aircraft with the same leader are
//!   going the same speed whatever the zoom. A turning aircraft gets a curved
//!   one, bent by the turn rate the tracker derives.
//! - a **data block** beside it: callsign, altitude, speed. Three short lines,
//!   the order every radar display in the world puts them in.
//!
//! # Why a stale target is not drawn at all
//!
//! Past [`sdroxide_types::AdsbSettings::drop_map_s`] a target's position is old
//! news, and it comes off the map — symbol, dots and block together. It stays
//! on the list, greyed. Fading it instead was the obvious alternative and is
//! wrong: a faint square at a stale position is still a claim about where an
//! aeroplane is, drawn in the same ink as the ones that are true, and the map
//! is the one place with no room for a hedge.
use crate::i18n::tr;

use eframe::egui::{
    Align2, Color32, FontId, Painter, Pos2, Rect, Sense, Shape, Stroke, Ui, Vec2, pos2, vec2,
};
use sdroxide_types::{AdsbAircraft, AdsbSettings, Vfo};

use crate::theme;
use crate::widgets::map_labels::{self, MapLabel};
use crate::widgets::worldmap::{
    MapView, alpha, draw_painted_base, interact_with, paint_night, wrap180,
};

/// Below this height the map is not worth drawing.
pub const MIN_HEIGHT: f32 = 90.0;

/// How close a click has to land, in points.
const HIT_RADIUS: f32 = 16.0;

/// Half the side of a target square, in points.
const TARGET_R: f32 = 10.5;
/// Half the side of an airport symbol (large, medium) and of a VOR's, in pixels.
const AIRPORT_HALF_LARGE: f32 = 7.0;
const AIRPORT_HALF_MEDIUM: f32 = 5.25;
const VOR_HALF: f32 = 5.25;

/// Never auto-fit tighter than this longitudinal span. Aircraft are hundreds of
/// kilometres apart and one overhead must not blow the map up to street level.
const MIN_LON_SPAN: f64 = 0.5;

/// Extra margin left around the outermost target.
const PAD: f64 = 1.25;

/// The colour of the station's record of an aircraft: the card's *history*
/// badge, so the trail and the lines say they are the same thing.
const HISTORY_TRAIL: Color32 = Color32::from_rgb(190, 150, 255);

/// Per-frame ease toward the auto-fit.
const EASE: f64 = 0.06;

/// One nautical mile in degrees of latitude. A minute of arc, by definition —
/// which is what makes converting a speed in knots into map degrees exact
/// rather than approximate.
const NM_PER_DEG_LAT: f64 = 60.0;

/// The map's own state, owned by the panel so the view survives across frames.
#[derive(Default)]
pub struct AdsbMapState {
    pub view: MapView,
    /// The map's optional layers — the flight paths, and the airports and
    /// navigation aids underneath them. Chips on the map pane's own row.
    pub layers: MapLayers,
    /// The aircraft whose card is open, by ICAO address.
    pub selected: Option<u32>,
    /// The aircraft whose small card is pinned beside it, by ICAO address. Set
    /// by a click on an aircraft, cleared by the card's close button, Esc, a
    /// click on empty map, another aircraft's click (which moves it) or the
    /// aircraft leaving the map.
    pub pinned: Option<u32>,
    /// Where the pinned aircraft is on screen this frame, written by [`show`].
    pub pin_anchor: Option<Pos2>,
    /// The map's rectangle this frame, written by [`show`]; the card stays in it.
    pub pin_bounds: Option<Rect>,
    /// How big the pinned card was last frame, so it can be placed to fit.
    pub pin_size: Option<Vec2>,
    /// The map that owns the pinned card, and the frame it last drew. The AIRCRAFT
    /// window and the ADS-B panel share this state and can both be open: only the
    /// map that was clicked draws the card, or one aircraft shows two.
    pub pin_owner: Option<(eframe::egui::Id, u64)>,
    /// The airport whose box is open on the map, by index into
    /// [`crate::airdata::airports`]. Set by a click on an airport (an aircraft
    /// under the pointer wins), cleared by a click on empty map or Esc.
    pub airport_pin: Option<usize>,
    /// The Wikipedia picture of the open airport box, written by the app each
    /// frame from what the station delivered; drawn only while it is for the
    /// airport that is pinned.
    pub airport_photo: Option<AirportPhoto>,
    /// The local time and temperature of the open airport box, written by the
    /// app each frame like the picture; drawn only for the pinned airport.
    pub airport_weather: Option<AirportWeather>,
    /// The aircraft the operator pressed "Look up online" for: its thumbnail is
    /// loaded once the answer is in. `None` until that button is pressed.
    pub lookup: Option<u32>,
    /// The aircraft the search box lets through, by ICAO address; `None` when
    /// nothing is searched for. Written by the panel each frame. Whatever is not
    /// in it is neither drawn, nor hovered, nor counted in the framing — except
    /// the pinned and the selected aircraft: see [`map_visible`].
    pub visible: Option<std::collections::HashSet<u32>>,
    /// Where the aircraft whose history is on has been, as line segments, the
    /// newest the most opaque. Written by the panel each frame from what the
    /// station sent; empty when no history is on.
    pub trail: Vec<TrailSeg>,
    /// The map under the picture: land, rivers and borders as an image rather
    /// than the dot stipple the propagation maps use. Rebuilt only when the
    /// view moves by more than a texel.
    pub ground: crate::widgets::worldmap::GroundCache,
}

/// What the open airport box shows above its text.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CardPhoto {
    /// No picture: the airport has no article, the relay is off, or it failed.
    /// The box is text alone, as before.
    #[default]
    None,
    /// On its way: the room is kept, so the box does not jump when it arrives.
    Pending,
    Ready {
        texture: eframe::egui::TextureId,
        /// The picture's size in pixels.
        size: Vec2,
    },
}

/// What an open airport box knows about its airport's clock and air.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AirportClock {
    /// No station, the relay is off, or the station could not say: UTC only, and
    /// the box says so. A local time is never guessed.
    Offline,
    /// Asked, not yet answered.
    Asking,
    Known {
        /// The zone's offset from UTC now, summer time included, in seconds.
        utc_offset_s: i32,
        /// The air temperature in °C, when the service gave one.
        temp_c: Option<f32>,
    },
}

/// The clock and air of one airport's box, with the moment they are read at.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AirportWeather {
    /// Index into [`crate::airdata::airports`].
    pub airport: usize,
    pub clock: AirportClock,
    /// Now, in unix seconds: the local time is this moved by the zone's offset.
    pub now: i64,
}

/// The picture of one airport's box, with the article it comes from.
#[derive(Debug, Clone, PartialEq)]
pub struct AirportPhoto {
    /// Index into [`crate::airdata::airports`].
    pub airport: usize,
    pub photo: CardPhoto,
    /// The Wikipedia page, where the credit points.
    pub page_url: String,
}

/// One stretch of an aircraft's past track: two positions (latitude, longitude)
/// and how opaque it is drawn, 0 to 1.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TrailSeg {
    pub from: (f64, f64),
    pub to: (f64, f64),
    pub alpha: f32,
}

/// Which of the map's optional layers are drawn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MapLayers {
    /// **The flight's path**: where it has been and where it is going, as one
    /// line through it rather than a scatter of history dots.
    pub paths: bool,
    /// **Airports and radio navigation aids** — the ground truth under the
    /// picture, from the public-domain tables in [`crate::airdata`].
    pub nav: bool,
}

impl Default for MapLayers {
    fn default() -> Self {
        // Both on. They are what the picture is for, and each has a chip to
        // turn it off — a layer nobody can find is a layer that does not exist.
        Self { paths: true, nav: true }
    }
}

/// The view to ease toward: every target that can be drawn, plus us, framed.
fn target_view(home: Option<(f64, f64)>, pts: &[(f64, f64)], aspect: f64) -> (f64, f64, f64) {
    let all: Vec<(f64, f64)> = home.into_iter().chain(pts.iter().copied()).collect();
    if all.is_empty() {
        return (20.0, 0.0, 360.0);
    }
    let (clat, clon) = match home {
        Some(h) => h,
        None => {
            let n = all.len() as f64;
            let lon_ref = all[0].1;
            (
                all.iter().map(|p| p.0).sum::<f64>() / n,
                wrap180(lon_ref + all.iter().map(|p| wrap180(p.1 - lon_ref)).sum::<f64>() / n),
            )
        }
    };
    let mut max_dlat = 0.0f64;
    let mut max_dlon = 0.0f64;
    for &(lat, lon) in &all {
        max_dlat = max_dlat.max((lat - clat).abs());
        max_dlon = max_dlon.max(wrap180(lon - clon).abs());
    }
    let need_lon = 2.0 * max_dlon * PAD;
    let need_lat = 2.0 * max_dlat * PAD;
    let lon_span = need_lon.max(need_lat / aspect.max(1e-3)).clamp(MIN_LON_SPAN, 360.0);
    let lat_span = (lon_span * aspect).min(180.0);
    let clat = if lat_span >= 180.0 {
        0.0
    } else {
        clat.clamp(-90.0 + lat_span / 2.0, 90.0 - lat_span / 2.0)
    };
    (clat, clon, lon_span)
}

/// Where a target will be after `minutes`, as a list of (lat, lon) waypoints
/// including where it is now.
///
/// Straight when it is flying straight, and bent by its turn rate when it is
/// not, which is what makes the leader on a turning aircraft point at where it
/// is going rather than where it was going. Stepped rather than solved because
/// the turn rate is itself an estimate: five short chords are as much shape as
/// the data supports.
fn leader(ac: &AdsbAircraft, minutes: f32) -> Vec<(f64, f64)> {
    let (Some(lat), Some(lon)) = (ac.lat, ac.lon) else { return Vec::new() };
    let (Some(kt), Some(track)) = (ac.ground_speed_kt, ac.track_deg) else { return Vec::new() };
    if minutes <= 0.0 || kt <= 1.0 {
        return Vec::new();
    }
    const STEPS: usize = 5;
    let secs = f64::from(minutes) * 60.0 / STEPS as f64;
    // Knots are nautical miles per hour and a nautical mile is a minute of
    // latitude, so this needs no earth radius and no approximation.
    let step_deg = f64::from(kt) * (secs / 3600.0) / NM_PER_DEG_LAT;
    let turn = f64::from(ac.turn_rate_deg_s) * secs;

    let mut out = Vec::with_capacity(STEPS + 1);
    out.push((lat, lon));
    let (mut la, mut lo, mut hdg) = (lat, lon, f64::from(track));
    for _ in 0..STEPS {
        let r = hdg.to_radians();
        la += step_deg * r.cos();
        // A degree of longitude shrinks with latitude; without this a leader at
        // 60 N is drawn half as long east-west as it should be.
        lo += step_deg * r.sin() / la.to_radians().cos().abs().max(0.02);
        if !la.is_finite() || !lo.is_finite() || la.abs() > 89.0 {
            break;
        }
        out.push((la, wrap180(lo)));
        hdg += turn;
    }
    out
}

/// How hard an aircraft's data block fights for room on a busy map: lower wins.
///
/// An emergency first, then anything flying, then anything on the ground — an
/// apron of parked aircraft must not crowd the approach off the picture.
fn block_rank(a: &AdsbAircraft) -> u8 {
    if a.emergency.is_some() {
        0
    } else if !a.on_ground {
        1
    } else {
        2
    }
}

/// Airports and radio navigation aids, filtered by zoom.
///
/// **The filter is the feature.** The tables hold 11 008 navaids and 5 280
/// airports, and drawing them all at every zoom is a wall of idents with a map
/// somewhere behind it. So the view decides: large airports always, medium ones
/// once they are places rather than specks, navaids only once a station is
/// something you could point an aerial at — which is also the zoom at which the
/// **MF ones become useful to a listener**, because those are receivable.
fn draw_nav_layer(
    p: Option<&Painter>,
    map: &theme::MapPalette,
    layers: MapLayers,
    clat: f64,
    clon: f64,
    lon_span: f64,
    lat_span: f64,
    project: &impl Fn(f64, f64) -> Pos2,
    pointer: Option<Pos2>,
) -> Option<NavHover> {
    if !layers.nav || !crate::airdata::loaded() {
        return None;
    }
    let zoom = nav_zoom(lon_span);
    let (half_lat, half_lon) = (lat_span * 0.5, lon_span * 0.5);
    let in_view = |lat: f64, lon: f64| {
        (lat - clat).abs() <= half_lat && wrap180(lon - clon).abs() <= half_lon
    };
    let font = FontId::monospace(9.0);
    // Only what is *drawn* can be hovered — see the note on the filter above.
    // A name for something invisible would be a map that answers questions
    // about things that are not on it.
    let mut hit: Option<(f32, Pos2, String, Option<u32>, Option<usize>)> = None;

    for (ai, a) in crate::airdata::airports().iter().enumerate() {
        if (!a.large && !zoom.medium_airports) || !in_view(a.lat, a.lon) {
            continue;
        }
        let c = project(a.lat, a.lon);
        if let Some(p) = p {
            let (half, ink) = if a.large {
                (AIRPORT_HALF_LARGE, alpha(map.city_label, 240.0))
            } else {
                (AIRPORT_HALF_MEDIUM, alpha(map.city_label, 175.0))
            };
            crate::adsb_icons::airport(p, c, half, ink);
            if zoom.airport_labels && (a.large || zoom.medium_labels) {
                let name = if a.iata.is_empty() { a.ident.as_str() } else { a.iata.as_str() };
                p.text(
                    c + vec2(if a.large { 8.5 } else { 6.5 }, -4.0),
                    Align2::LEFT_TOP,
                    name,
                    font.clone(),
                    alpha(map.city_label, 190.0),
                );
            }
        }
        if let Some(m) = pointer {
            let d = c.distance(m);
            if d <= NAV_HIT_RADIUS && hit.as_ref().is_none_or(|(best, ..)| d < *best) {
                // An airport carries no frequency in the source table: what is
                // tuned is the *aid* — a VOR or an NDB — which is the row below.
                hit = Some((d, c, airport_hover_text(a), None, Some(ai)));
            }
        }
    }

    if !zoom.navaids {
        return hit.map(|(_, at, text, freq_khz, airport)| NavHover {
            at,
            text,
            freq_khz,
            airport,
        });
    }
    for (ni, n) in crate::airdata::navaids().iter().enumerate() {
        // An aid on an airport's field is listed on the airport's card and has
        // no symbol of its own: one place, one thing to click.
        if !in_view(n.lat, n.lon) || crate::airdata::navaid_airport(ni).is_some() {
            continue;
        }
        let c = project(n.lat, n.lon);
        // A listener's aid is a different colour from a pilot's: an NDB is on
        // medium wave and can be *heard*, and that is the half of this table
        // this fork cares most about.
        let ink = if n.kind.is_audible_on_mf() { map.home } else { map.river };
        if let Some(p) = p {
            if n.kind.is_audible_on_mf() {
                p.circle_filled(c, 3.0, alpha(ink, 210.0));
            } else {
                // The tower of a VOR family aid.
                crate::adsb_icons::vor(p, c, VOR_HALF, alpha(ink, 240.0));
            }
        }
        if let Some(m) = pointer {
            let d = c.distance(m);
            if d <= NAV_HIT_RADIUS && hit.as_ref().is_none_or(|(best, ..)| d < *best) {
                hit = Some((
                    d,
                    c,
                    navaid_hover_text(n),
                    (n.freq_khz != 0).then_some(n.freq_khz),
                    None,
                ));
            }
        }
        if let Some(p) = p
            && zoom.navaid_labels
            && !n.ident.is_empty()
        {
            p.text(
                c + vec2(7.0, -4.0),
                Align2::LEFT_TOP,
                &n.ident,
                font.clone(),
                alpha(map.city_label, 200.0),
            );
        }
    }
    hit.map(|(_, at, text, freq_khz, airport)| NavHover { at, text, freq_khz, airport })
}

/// Where a coordinate lands on the map, for a view and a rect. One function:
/// the hover pick, the drawing and the pre-pick that decides whether a
/// double-click is a tune or a reframe all have to agree about it.
fn project_at(
    rect: Rect,
    clat: f64,
    clon: f64,
    lon_span: f64,
    lat_span: f64,
    lat: f64,
    lon: f64,
) -> Pos2 {
    let dlon = wrap180(lon - clon);
    pos2(
        rect.left() + (0.5 + (dlon / lon_span) as f32) * rect.width(),
        rect.top() + (0.5 - ((lat - clat) / lat_span) as f32) * rect.height(),
    )
}

/// The name of whatever the pointer is over, drawn at it.
///
/// Drawn rather than handed to egui as a tooltip: the map is one widget, so a
/// tooltip could only say "somewhere on the map", and what the operator wants
/// is the name beside the symbol they are pointing at.
fn draw_hover_label(p: &Painter, rect: Rect, map: &theme::MapPalette, hover: &NavHover) {
    draw_hover_card(p, rect, map, hover, CardPhoto::None, None);
}

/// The widest a card's picture is drawn, and the tallest, in points. A
/// Wikipedia thumbnail is 320 px wide; half of that is plenty for a card and
/// keeps it a card.
const CARD_PHOTO_W: f32 = 168.0;
const CARD_PHOTO_MAX_H: f32 = 112.0;
/// The room kept for a picture on its way: a landscape 3:2.
const CARD_PHOTO_PENDING_H: f32 = 112.0;

/// Where a picture of `size` pixels is drawn in a card: as wide as the card's
/// picture may be and no taller than it may be, its proportions kept.
fn photo_fit(size: Vec2) -> Vec2 {
    if size.x < 1.0 || size.y < 1.0 {
        return Vec2::ZERO;
    }
    let k = (CARD_PHOTO_W / size.x).min(CARD_PHOTO_MAX_H / size.y);
    size * k
}

/// What a card took on the screen: the box, and the credit line when it has one.
struct CardDrawn {
    box_: Rect,
    credit: Option<Rect>,
}

/// [`draw_hover_label`] with the picture of the place above its text and, under
/// the picture, `credit` (the line that names where the picture is from).
fn draw_hover_card(
    p: &Painter,
    rect: Rect,
    map: &theme::MapPalette,
    hover: &NavHover,
    photo: CardPhoto,
    credit: Option<&str>,
) -> CardDrawn {
    // The gesture is *said*, where the station's name is read: a station that can
    // be tuned has to look like one, or the feature is invisible. Only the ones
    // that are on a frequency say it.
    let hint = hover.freq_khz.is_some().then(|| tr!("adsbmap-tune-hint"));
    let galley =
        p.layout_no_wrap(hover.text.clone(), FontId::monospace(10.0), alpha(map.city_label, 255.0));
    let hint_galley =
        hint.map(|h| p.layout_no_wrap(h, FontId::proportional(9.0), alpha(map.city_label, 190.0)));
    let pad = vec2(6.0, 4.0);
    let hint_h = hint_galley.as_ref().map_or(0.0, |g| g.size().y + 2.0);
    let pic = match photo {
        CardPhoto::None => Vec2::ZERO,
        CardPhoto::Pending => vec2(CARD_PHOTO_W, CARD_PHOTO_PENDING_H),
        CardPhoto::Ready { size, .. } => photo_fit(size),
    };
    // The credit goes with a picture that is drawn, not with room that is kept.
    let credit_galley = credit.filter(|_| matches!(photo, CardPhoto::Ready { .. })).map(|c| {
        p.layout_no_wrap(c.to_string(), FontId::proportional(9.0), alpha(map.city_label, 200.0))
    });
    let credit_h = credit_galley.as_ref().map_or(0.0, |g| g.size().y);
    let photo_h = if pic.y > 0.0 { pic.y + 2.0 + credit_h + 4.0 } else { 0.0 };
    let size = vec2(galley.size().x.max(pic.x), galley.size().y + photo_h + hint_h) + pad * 2.0;
    let mut at = hover.at + vec2(11.0, -size.y - 7.0);
    // Kept on the map: a name for something at the edge is the one that most
    // needs to move, because the auto-fit leaves the outermost targets close to
    // the edge.
    at.x = at.x.clamp(rect.left() + 2.0, (rect.right() - size.x - 2.0).max(rect.left() + 2.0));
    at.y = at.y.clamp(rect.top() + 2.0, (rect.bottom() - size.y - 2.0).max(rect.top() + 2.0));
    let box_ = Rect::from_min_size(at, size);
    p.rect_filled(box_, 3.0, alpha(map.sea, 240.0));
    p.rect_stroke(
        box_,
        3.0,
        Stroke::new(1.0, alpha(map.border, 210.0)),
        eframe::egui::StrokeKind::Inside,
    );
    let mut y = box_.min.y + pad.y;
    let mut credit_rect = None;
    if pic.y > 0.0 {
        let frame = Rect::from_min_size(pos2(box_.min.x + pad.x, y), pic);
        match photo {
            CardPhoto::Ready { texture, .. } => {
                p.image(
                    texture,
                    frame,
                    Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                    Color32::WHITE,
                );
                p.rect_stroke(
                    frame,
                    1.0,
                    Stroke::new(1.0, alpha(map.border, 160.0)),
                    eframe::egui::StrokeKind::Outside,
                );
            }
            _ => {
                p.rect_filled(frame, 2.0, alpha(map.border, 40.0));
                p.text(
                    frame.center(),
                    Align2::CENTER_CENTER,
                    "…",
                    FontId::proportional(14.0),
                    alpha(map.city_label, 150.0),
                );
            }
        }
        y += pic.y + 2.0;
        if let Some(g) = credit_galley {
            let r = Rect::from_min_size(pos2(box_.min.x + pad.x, y), g.size());
            p.galley(r.min, g, map.city_label);
            p.line_segment(
                [r.left_bottom(), r.right_bottom()],
                Stroke::new(0.8, alpha(map.city_label, 160.0)),
            );
            credit_rect = Some(r);
            y += r.height();
        }
        y += 4.0;
    }
    let text_h = galley.size().y;
    p.galley(pos2(box_.min.x + pad.x, y), galley, map.city_label);
    if let Some(g) = hint_galley {
        p.galley(pos2(box_.min.x + pad.x, y + text_h + 2.0), g, map.city_label);
    }
    // A ring on the symbol itself, so the name is attached to a mark.
    p.circle_stroke(hover.at, 4.5, Stroke::new(1.3, alpha(map.city_label, 220.0)));
    CardDrawn { box_, credit: credit_rect }
}

/// How far from an airport an aircraft counts as "near" it, km.
const AIRPORT_NEAR_KM: f64 = 100.0;

/// The most runways and frequencies the pinned card lists; the rest is a "+N".
const CARD_RUNWAYS: usize = 4;
const CARD_FREQS: usize = 6;
/// The most radio aids of the field the pinned card lists.
const CARD_NAVAIDS: usize = 6;
/// Frequencies per line, so a long list stays narrow.
const CARD_FREQS_PER_LINE: usize = 3;

fn surface_name(s: crate::airdata::Surface) -> Option<String> {
    use crate::airdata::Surface;
    Some(match s {
        Surface::Unknown => return None,
        Surface::Asphalt => tr!("adsbmap-surface-asphalt"),
        Surface::Concrete => tr!("adsbmap-surface-concrete"),
        Surface::Grass => tr!("adsbmap-surface-grass"),
        Surface::Gravel => tr!("adsbmap-surface-gravel"),
        Surface::Dirt => tr!("adsbmap-surface-dirt"),
        Surface::Water => tr!("adsbmap-surface-water"),
    })
}

/// `118.100` for 118 100 kHz.
fn mhz(khz: u32) -> String {
    format!("{}.{:03}", khz / 1000, khz % 1000)
}

/// `20261031` as `2026-10-31`.
fn iso_date(d: u32) -> String {
    format!("{:04}-{:02}-{:02}", d / 10_000, d / 100 % 100, d % 100)
}

/// The runway and frequency lines of the pinned card, from the embedded
/// OurAirports tables — nothing is looked up and nothing is invented: an
/// airport the source says nothing about gets no lines. Lengths follow the
/// chosen units (feet, or metres in the metric set).
fn airport_detail_lines(a: &crate::airdata::Airport) -> Vec<String> {
    let units = sdroxide_types::AirUnits::current();
    let mut out = Vec::new();
    let rwys = crate::airdata::runways_of(&a.ident);
    for r in rwys.iter().take(CARD_RUNWAYS) {
        let mut parts =
            vec![format!("{}/{}", r.low, r.high), units.altitude_dual(i32::from(r.length_ft))];
        parts.extend(surface_name(r.surface));
        if r.lit {
            parts.push(tr!("adsbmap-airport-runway-lit"));
        }
        out.push(parts.join(" · "));
    }
    if rwys.len() > CARD_RUNWAYS {
        out.push(tr!("adsbmap-airport-more", n = rwys.len() - CARD_RUNWAYS));
    }
    let freqs = crate::airdata::frequencies_of(&a.ident);
    if !freqs.is_empty() {
        out.push(tr!("adsbmap-airport-freq-head"));
        for line in freqs.iter().take(CARD_FREQS).collect::<Vec<_>>().chunks(CARD_FREQS_PER_LINE) {
            out.push(
                line.iter()
                    .map(|f| format!("{} {}", f.kind.label(), mhz(f.khz)))
                    .collect::<Vec<_>>()
                    .join(" · "),
            );
        }
        if freqs.len() > CARD_FREQS {
            out.push(tr!("adsbmap-airport-more", n = freqs.len() - CARD_FREQS));
        }
    }
    // The radio aids on the field — a VOR or an NDB is part of the airport,
    // not a second thing on the map — with the same one-line shape as the hover.
    let navaids = crate::airdata::airports()
        .iter()
        .position(|x| x.ident == a.ident)
        .map(crate::airdata::navaids_of)
        .unwrap_or_default();
    if freqs.is_empty() && (!out.is_empty() || !navaids.is_empty()) {
        out.push(tr!("adsbmap-airport-no-freq"));
    }
    if !navaids.is_empty() {
        out.push(tr!("adsbmap-airport-navaids-head"));
        for (n, _) in navaids.iter().take(CARD_NAVAIDS) {
            out.push(navaid_hover_text(n));
        }
        if navaids.len() > CARD_NAVAIDS {
            out.push(tr!("adsbmap-airport-more", n = navaids.len() - CARD_NAVAIDS));
        }
    }
    if !out.is_empty() {
        out.push(tr!("adsbmap-airport-data-date", date = iso_date(crate::airdata::extras_date())));
    }
    out
}

/// The airport's card: codes, name, the height of the field in the chosen
/// units, how far and which way it is from the station, and how many of the
/// aircraft on the map are near it. With `detail` (the pinned card, not the
/// hover) it adds the runways and the published frequencies.
fn airport_card_text(
    a: &crate::airdata::Airport,
    home: Option<(f64, f64)>,
    live: &[&AdsbAircraft],
    detail: bool,
) -> String {
    airport_card_text_with(a, home, live, detail, None)
}

/// The lines about an airport's clock and air, in the chosen units: the local
/// time with its zone and the temperature when they are known (and the credit
/// the service asks for), else UTC with the reason there is no local time.
fn airport_clock_lines(w: &AirportWeather) -> Vec<String> {
    let units = sdroxide_types::AirUnits::current();
    let at = |offset_s: i32| units.clock(sdroxide_types::local_secs_of_day(w.now, offset_s));
    match w.clock {
        AirportClock::Known { utc_offset_s, temp_c } => {
            let mut line = tr!(
                "adsbmap-airport-local-time",
                time = at(utc_offset_s),
                zone = sdroxide_types::utc_label(utc_offset_s)
            );
            if let Some(t) = temp_c {
                line.push_str(" · ");
                line.push_str(&tr!(
                    "adsbmap-airport-temperature",
                    temp = units.temperature_dual(t)
                ));
            }
            vec![line, tr!("adsbmap-airport-weather-credit")]
        }
        AirportClock::Asking => vec![tr!("adsbmap-airport-utc-wait", time = at(0))],
        AirportClock::Offline => vec![tr!("adsbmap-airport-utc-offline", time = at(0))],
    }
}

/// [`airport_card_text`] with the airport's clock and air, for the pinned box.
fn airport_card_text_with(
    a: &crate::airdata::Airport,
    home: Option<(f64, f64)>,
    live: &[&AdsbAircraft],
    detail: bool,
    weather: Option<&AirportWeather>,
) -> String {
    let units = sdroxide_types::AirUnits::current();
    let codes = match (a.iata.is_empty(), a.icao.is_empty() || a.icao == a.ident) {
        (true, true) => a.ident.clone(),
        (true, false) => format!("{} / {}", a.ident, a.icao),
        (false, true) => format!("{} / {}", a.iata, a.ident),
        (false, false) => format!("{} / {} / {}", a.iata, a.icao, a.ident),
    };
    let elev = units.altitude_dual(i32::from(a.elev_ft));
    let mut text = format!(
        "{codes}\n{}\n{}",
        a.name,
        if a.large {
            tr!("adsbmap-airport-large", elev = elev)
        } else {
            tr!("adsbmap-airport-medium", elev = elev)
        }
    );
    if let Some(h) = home {
        let km = sdroxide_types::distance_km(h, (a.lat, a.lon));
        let bear = sdroxide_types::bearing_deg(h, (a.lat, a.lon));
        text.push_str(&format!(
            "\n{}",
            tr!(
                "adsbmap-from-station",
                dist = units.distance_dual(km),
                bear = format!("{bear:.0}")
            )
        ));
    }
    let near = live
        .iter()
        .filter_map(|ac| ac.lat.zip(ac.lon))
        .filter(|&(la, lo)| {
            sdroxide_types::distance_km((a.lat, a.lon), (la, lo)) <= AIRPORT_NEAR_KM
        })
        .count();
    if near > 0 {
        text.push_str(&format!(
            "\n{}",
            tr!("adsbmap-aircraft-within", n = near, dist = units.distance_dual(AIRPORT_NEAR_KM))
        ));
    }
    if detail {
        for line in weather.map(airport_clock_lines).unwrap_or_default() {
            text.push('\n');
            text.push_str(&line);
        }
        for line in airport_detail_lines(a) {
            text.push('\n');
            text.push_str(&line);
        }
    }
    text
}

/// The airport pin after a click this frame: an aircraft under the pointer
/// leaves it alone (the click belongs to the aircraft), an airport takes it,
/// empty map closes it.
pub fn airport_pin_after_click(
    pinned: Option<usize>,
    aircraft_hit: bool,
    airport_hit: Option<usize>,
    clicked: bool,
) -> Option<usize> {
    if !clicked || aircraft_hit {
        return pinned;
    }
    airport_hit
}

/// The airport pin after a key press: Esc closes it, as it does the aircraft's.
pub fn airport_pin_after_key(pinned: Option<usize>, esc: bool) -> Option<usize> {
    if esc { None } else { pinned }
}

/// What a hovered airport says: the abbreviation first, because that is what
/// the operator asked for and what a chart writes, then the name.
fn airport_hover_text(a: &crate::airdata::Airport) -> String {
    if a.iata.is_empty() {
        format!("{} · {}", a.ident, a.name)
    } else {
        format!("{} · {}", a.iata, a.name)
    }
}

/// What a hovered navaid says: its ident, what kind of aid it is, and the
/// frequency on the unit it is published in — **kHz for the medium-wave ones,
/// because those are audible**, which is the half of this table a listener
/// cares about.
fn navaid_hover_text(n: &crate::airdata::Navaid) -> String {
    let freq = if n.kind.is_audible_on_mf() {
        tr!("adsbmap-navaid-mw", khz = n.freq_khz)
    } else {
        format!("{:.3} MHz", n.freq_khz as f64 / 1000.0)
    };
    if n.name.is_empty() {
        format!("{} · {} · {}", n.ident, n.kind.label(), freq)
    } else {
        format!("{} · {} · {} — {}", n.ident, n.kind.label(), freq, n.name)
    }
}

/// A feature the pointer is over, what to say about it, and — for the stations
/// that are on a radio frequency — where to tune to hear it.
struct NavHover {
    at: Pos2,
    text: String,
    freq_khz: Option<u32>,
    /// Set when the thing under the pointer is an airport.
    airport: Option<usize>,
}

/// **Where a double-clicked station goes: the *other* dial, always.**
///
/// Never the one in use, and that is a property of the mode rather than a
/// preference. This map is the ADS-B panel, and ADS-B is a channel: entering it
/// tunes the active VFO to 1090 MHz and switching back to that VFO puts it there
/// again — *"ADS-B is on 1090 MHz; tuning there"*, in the engine's own words. A
/// frequency written to that dial is overwritten the moment the VFO is recalled,
/// which is exactly the fault the operator reported before he traced it himself:
/// *"the mode is ADS-B so it wants to switch back to the ADS-B frequency… in that
/// mode you cannot set frequency A because the mode dictates"*. His conclusion,
/// and his rule for this gesture: *"if you want to click a frequency it always
/// has to go to B"*.
#[must_use]
pub fn other_dial(active: Vfo) -> Vfo {
    match active {
        Vfo::A => Vfo::B,
        Vfo::B => Vfo::A,
    }
}

/// The dial's frequency for a station's published one.
///
/// The tables are in **kHz** (`airdata::Navaid::freq_khz` — "the unit both kinds
/// are published in") and a dial is in Hz, which is the conversion this fork has
/// got wrong before in the other direction.
#[must_use]
pub fn tune_hz(freq_khz: u32) -> f64 {
    freq_khz as f64 * 1000.0
}

/// What a double-click on the map asked for: a station to listen to, on the dial
/// the current mode does not own (see [`other_dial`]).
pub struct MapTune {
    /// Where to tune, in Hz.
    pub hz: f64,
    /// The dial to put it on.
    pub vfo: Vfo,
}

/// How close the pointer has to be to a symbol to name it. Wider than the
/// aircraft's hit radius: these are small marks on a busy map, and the operator
/// is asking about a place rather than selecting a target.
const NAV_HIT_RADIUS: f32 = 12.0;

/// What the view's span allows the navigation layer to draw.
///
/// **The filter is the feature.** The tables hold 11 008 navaids and 5 280
/// airports; drawn all at once at every zoom they are a wall of idents with a
/// map somewhere behind. Each threshold opens as the view tightens and none
/// ever closes again going in — `nav_zoom` is a step function of the span, and
/// that monotonicity is the property the test pins, because a layer that
/// *loses* detail as you zoom in is the bug this shape exists to prevent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct NavZoom {
    medium_airports: bool,
    airport_labels: bool,
    /// Medium airports get their code written too, once the view is a region
    /// rather than a country.
    medium_labels: bool,
    navaids: bool,
    navaid_labels: bool,
}

fn nav_zoom(lon_span: f64) -> NavZoom {
    NavZoom {
        medium_airports: lon_span <= 40.0,
        airport_labels: lon_span <= 12.0,
        medium_labels: lon_span <= 4.0,
        // Below this a station is a place rather than a speck — and it is also
        // the zoom at which the medium-wave beacons become a listening list
        // rather than decoration.
        navaids: lon_span <= 20.0,
        navaid_labels: lon_span <= 6.0,
    }
}

/// What the plain tooltip over a target says. Kept as it was before the map had
/// a hover card, because the card is optional and this is what the map says
/// without one.
fn tooltip_text(a: &AdsbAircraft, home: Option<(f64, f64)>) -> String {
    let mut tip = a.label();
    if !a.callsign.is_empty() {
        tip.push_str(&format!("  {}", a.hex()));
    }
    if let Some(cat) = &a.category {
        tip.push_str(&format!("\n{cat}"));
    }
    let u = sdroxide_types::AirUnits::current();
    tip.push_str(&format!(
        "\n{}   {}",
        u.altitude(a.altitude_ft.unwrap_or(0)),
        a.ground_speed_kt.map(|k| u.speed(k)).unwrap_or_else(|| "--- kt".into())
    ));
    if let Some(vr) = a.vertical_rate_fpm.filter(|v| v.abs() > 100) {
        tip.push_str(&format!("\n{}", u.vertical_rate(vr)));
    }
    if let (Some(h), Some((lat, lon))) = (home, a.lat.zip(a.lon)) {
        let km = sdroxide_types::distance_km(h, (lat, lon));
        let bear = sdroxide_types::bearing_deg(h, (lat, lon));
        tip.push_str(&format!("\n{}   {bear:.0}°", u.distance(km)));
    }
    if let Some(e) = &a.emergency {
        tip.push_str(&format!("\n{e}"));
    }
    tip
}

/// Draw the map. Returns the aircraft clicked this frame, if any.
///
/// `hover_ui` draws the card shown over an aircraft; without it the plain text
/// of [`tooltip_text`] is shown.
pub fn show(
    ui: &mut Ui,
    state: &mut AdsbMapState,
    aircraft: &[AdsbAircraft],
    home: Option<(f64, f64)>,
    now: i64,
    cfg: AdsbSettings,
    night: Option<eframe::egui::TextureId>,
    style: sdroxide_types::MapStyle,
    max_h: f32,
    active_vfo: Vfo,
    hover_ui: Option<&dyn Fn(&mut Ui, &AdsbAircraft)>,
) -> Option<MapTune> {
    // The pinned card goes with its aircraft, and is only placed once this frame
    // has drawn the aircraft where it is.
    state.pinned = pin_keep(state.pinned, aircraft, now, cfg.drop_map_s);
    state.pin_anchor = None;
    state.pin_bounds = None;
    let frame = ui.ctx().cumulative_frame_nr();
    state.pin_owner = pin_owner_after(state.pin_owner, ui.id(), frame, false);
    let avail_w = ui.available_width();
    if avail_w < MIN_HEIGHT {
        return None;
    }
    let h = max_h.min(avail_w).max(MIN_HEIGHT);
    let (rect, resp) = ui.allocate_exact_size(vec2(avail_w, h), Sense::click_and_drag());
    if !ui.is_rect_visible(rect) {
        return None;
    }
    let map = &theme::map_styled(style);
    let p = ui.painter_at(rect);
    p.rect_filled(rect, 0.0, map.sea);

    // Only the targets with a position fresh enough to draw, and that the search
    // box lets through. Everything below works from this list, so a stale
    // aircraft cannot pull the view toward where it used to be either — and
    // neither can one the search has hidden.
    let live: Vec<&AdsbAircraft> = aircraft
        .iter()
        .filter(|a| !a.pos_stale(now, cfg.drop_map_s))
        .filter(|a| map_visible(state.visible.as_ref(), a.icao, state.pinned, state.selected))
        .collect();

    // ── the view ──
    let aspect = (rect.height() / rect.width()) as f64;
    let pts: Vec<(f64, f64)> = live.iter().filter_map(|a| a.lat.zip(a.lon)).collect();
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
    // **Is the double-click for a station, or for the view?** Asked from the
    // view as it stands *before* the interaction, because a double-click never
    // pans — the coordinates are the ones the pick will use a moment later.
    // Painting nothing: the same function, asked a question.
    let station_under_pointer = {
        let pre = |lat: f64, lon: f64| {
            project_at(rect, view.clat, view.clon, view.lon_span, view.lon_span * aspect, lat, lon)
        };
        draw_nav_layer(
            None,
            map,
            state.layers,
            view.clat,
            view.clon,
            view.lon_span,
            view.lon_span * aspect,
            &pre,
            resp.hover_pos(),
        )
        .and_then(|h| h.freq_khz)
        .is_some()
    };
    if interact_with(ui, view, &resp, aspect, !(resp.double_clicked() && station_under_pointer)) {
        crate::repaint::animate(ui.ctx());
    }
    let (clat, clon, lon_span) = (view.clat, view.clon, view.lon_span);
    let lat_span = lon_span * aspect;
    let manual = view.manual;

    // A painted map, not the dot stipple: an air picture is looked at, and a
    // stippled ground is the propagation panel's idiom (see `draw_painted_base`).
    let dot_r = draw_painted_base(
        ui.ctx(),
        &p,
        rect,
        clat,
        clon,
        lon_span,
        lat_span,
        map,
        style,
        &mut state.ground,
    );
    // Over the continents, so land darkens with the sea on the night side; the
    // marks drawn below stay lit.
    paint_night(&p, rect, clat, clon, lon_span, lat_span, night);

    let project = |lat: f64, lon: f64| project_at(rect, clat, clon, lon_span, lat_span, lat, lon);

    // Which target the pointer is over, decided before anything is drawn so the
    // hovered one can be drawn last and on top.
    let pointer = resp.hover_pos();
    let mut hover: Option<usize> = None;
    let mut best = HIT_RADIUS;
    for (i, a) in live.iter().enumerate() {
        let Some((lat, lon)) = a.lat.zip(a.lon) else { continue };
        let c = project(lat, lon);
        if !rect.contains(c) {
            continue;
        }
        if let Some(m) = pointer {
            let d = c.distance(m);
            if d <= best {
                best = d;
                hover = Some(i);
            }
        }
    }

    // ── airports and navaids, at the very bottom ──
    let mut nav_hover = draw_nav_layer(
        Some(&p),
        map,
        state.layers,
        clat,
        clon,
        lon_span,
        lat_span,
        &project,
        pointer,
    );

    // An airport says more than its name: the card text replaces the one line.
    if let Some(h) = nav_hover.as_mut()
        && let Some(a) = h.airport.and_then(|i| crate::airdata::airports().get(i))
    {
        h.text = airport_card_text(a, home, &live, false);
    }

    // **A double-click on a station tunes it.** The map is where the stations
    // are *seen*; without this the operator reads a frequency off the hover text
    // and types it into a box. Shift puts it on the other dial, which is the A/B
    // pair an operator listening to two stations at once is already using.
    let mut tune: Option<MapTune> = None;
    if resp.double_clicked()
        && let Some(khz) = nav_hover.as_ref().and_then(|h| h.freq_khz)
    {
        tune = Some(MapTune { hz: tune_hz(khz), vfo: other_dial(active_vfo) });
    }

    // ── the station's record of the one aircraft whose history is on ──
    // Under the live tracks, fainter the older it is. A stretch wholly off the
    // map is skipped.
    for t in &state.trail {
        let (from, to) = (project(t.from.0, t.from.1), project(t.to.0, t.to.1));
        if rect.contains(from) || rect.contains(to) {
            p.line_segment([from, to], Stroke::new(1.8, alpha(HISTORY_TRAIL, t.alpha * 255.0)));
        }
    }

    // ── history, under everything else ──
    for a in &live {
        let n = a.track.len();
        if state.layers.paths && n >= 2 {
            // **The flight's path**: the history joined into one line, and run
            // through the aircraft's position so it carries on into the leader
            // drawn below — where it has been and where it is going read as one
            // thing rather than as a scatter of dots and a stub.
            let mut pts: Vec<Pos2> =
                a.track.iter().map(|&(la, lo)| project(f64::from(la), f64::from(lo))).collect();
            if let Some((lat, lon)) = a.lat.zip(a.lon) {
                pts.push(project(lat, lon));
            }
            p.add(Shape::line(pts, Stroke::new(1.2, alpha(map.trail, 130.0))));
            continue;
        }
        for (k, &(plat, plon)) in a.track.iter().enumerate() {
            let c = project(f64::from(plat), f64::from(plon));
            if !rect.contains(c) {
                continue;
            }
            // Oldest faintest, so the trail reads as a direction without a
            // single arrowhead being drawn.
            let age = (k + 1) as f32 / n as f32;
            p.circle_filled(c, 1.3, alpha(map.trail, 40.0 + 150.0 * age));
        }
    }

    let font = FontId::monospace(9.0);
    let selected_icao = state.selected;
    let tint_for = |a: &AdsbAircraft, top: bool| -> Color32 {
        if a.emergency.is_some() || selected_icao == Some(a.icao) {
            map.dx
        } else if top {
            map.hover
        } else {
            map.station
        }
    };

    // ── targets, under the data blocks ──
    // The hovered one last, so it sits over its neighbours.
    let draw_target = |a: &AdsbAircraft, top: bool| {
        let Some((lat, lon)) = a.lat.zip(a.lon) else { return };
        let c = project(lat, lon);
        if !rect.contains(c) && !top {
            return;
        }
        let tint = tint_for(a, top);

        // The leader, under the symbol: where it is now matters more than where
        // it will be.
        let path = leader(a, cfg.vector_minutes);
        if path.len() > 1 {
            let pts: Vec<_> = path.iter().map(|&(la, lo)| project(la, lo)).collect();
            for w in pts.windows(2) {
                p.line_segment([w[0], w[1]], (1.2, alpha(tint, 190.0)));
            }
        }

        // The target: an aeroplane turned to its track, the same size at every zoom.
        let r = if top || selected_icao == Some(a.icao) { TARGET_R + 3.0 } else { TARGET_R };
        crate::adsb_icons::aircraft(&p, c, r, a.track_deg, tint, a.on_ground);
    };
    for (i, a) in live.iter().enumerate() {
        if hover != Some(i) {
            draw_target(a, false);
        }
    }
    if let Some(i) = hover {
        draw_target(live[i], true);
    }

    // ── the data blocks ──
    // The selected, the hovered and an emergency always; the rest by how much
    // they matter — what is flying, then what is on the ground — each only
    // where its block misses the symbols and the blocks already placed. A busy
    // sector then keeps the blocks it has room for, rather than losing every
    // one of them at once the moment a couple of dozen aircraft are in view,
    // which is how the AIS chart's names went missing (#408).
    let mut labels = Vec::new();
    for (i, a) in live.iter().enumerate() {
        let Some((lat, lon)) = a.lat.zip(a.lon) else { continue };
        let c = project(lat, lon);
        if !rect.contains(c) {
            continue;
        }
        let top = hover == Some(i);
        let selected = selected_icao == Some(a.icao);
        let tint = tint_for(a, top);
        labels.push(MapLabel {
            at: c,
            r: if top || selected { TARGET_R + 3.0 } else { TARGET_R },
            tick_from: 0.7,
            tick: alpha(tint, 110.0),
            lines: vec![
                (a.label(), alpha(tint, 240.0)),
                (a.fmt_altitude(), alpha(tint, 190.0)),
                (a.fmt_speed(), alpha(tint, 190.0)),
            ],
            must: top || selected || a.emergency.is_some(),
            rank: block_rank(a),
            key: a.icao,
        });
    }
    // Our own mark is drawn over the blocks, so they keep off it.
    let marks: Vec<Rect> = home
        .map(|(lat, lon)| Rect::from_center_size(project(lat, lon), vec2(16.0, 16.0)))
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

    // ── the name of whatever the pointer is over, on top of all of it ──
    // An aircraft wins: it carries its own data block, and a place name drawn
    // over it would be answering a different question than the one being asked.
    if hover.is_none() {
        if let Some(h) = &nav_hover {
            // Not over the airport whose box is already open: it would say the
            // same thing twice.
            if h.airport.is_none() || h.airport != state.airport_pin {
                draw_hover_label(&p, rect, map, h);
            }
        }
    }
    // The open airport box stays where its airport is, until a click elsewhere.
    let mut credit_clicked = false;
    if let Some(a) = state.airport_pin.and_then(|i| crate::airdata::airports().get(i)) {
        let at = project(a.lat, a.lon);
        if rect.contains(at) {
            let pinned = NavHover {
                at,
                text: format!(
                    "{}\n{}",
                    airport_card_text_with(
                        a,
                        home,
                        &live,
                        true,
                        state
                            .airport_weather
                            .as_ref()
                            .filter(|w| Some(w.airport) == state.airport_pin),
                    ),
                    tr!("adsbmap-click-to-close")
                ),
                freq_khz: None,
                airport: state.airport_pin,
            };
            let shown = state
                .airport_photo
                .as_ref()
                .filter(|f| Some(f.airport) == state.airport_pin)
                .map(|f| (f.photo, f.page_url.clone()));
            let (photo, page) = shown.unwrap_or((CardPhoto::None, String::new()));
            let drawn = draw_hover_card(
                &p,
                rect,
                map,
                &pinned,
                photo,
                Some(&tr!("adsbmap-airport-photo-credit")),
            );
            // The credit is a link to the article, where the picture's author and
            // licence are. A click on it follows the link and does not close the box.
            if let Some(r) = drawn.credit {
                let link = ui.interact(r, resp.id.with("airport-photo-credit"), Sense::click());
                if link.hovered() {
                    ui.ctx().set_cursor_icon(eframe::egui::CursorIcon::PointingHand);
                }
                if link.clicked() && !page.is_empty() {
                    ui.ctx().open_url(eframe::egui::OpenUrl::new_tab(page));
                    credit_clicked = true;
                }
            }
        }
    }

    // ── interaction ──
    if let Some(i) = hover {
        let a = live[i];
        // A card from the app when it has one; the plain text when it has not.
        // Not over the aircraft whose card is pinned: it would say it twice.
        if hover_wanted(state.pinned, a.icao) {
            match hover_ui {
                Some(card) => {
                    resp.clone().on_hover_ui(|ui| card(ui, a));
                }
                None => {
                    resp.clone().on_hover_text(tooltip_text(a, home));
                }
            }
        }
        if resp.clicked() {
            state.selected = Some(a.icao);
        }
    } else if resp.clicked() {
        state.selected = None;
    }
    // A click on an aircraft moves the pin there; a click on empty map closes it
    // (a beginner never looks for the ✕). The credit link is not empty map.
    state.pinned = pin_after_click(
        state.pinned,
        hover.map(|i| live[i].icao),
        resp.clicked() && !credit_clicked,
    );
    if hover.is_some() && resp.clicked() {
        state.pin_owner = pin_owner_after(state.pin_owner, ui.id(), frame, true);
    }
    // Esc closes the box while the pointer is over the map, so it does not fight
    // a text field elsewhere that is using the key.
    let esc = resp.hovered() && ui.input(|i| i.key_pressed(eframe::egui::Key::Escape));
    state.airport_pin = airport_pin_after_key(state.airport_pin, esc);
    state.airport_pin = airport_pin_after_click(
        state.airport_pin,
        hover.is_some(),
        nav_hover.as_ref().and_then(|h| h.airport),
        resp.clicked() && !credit_clicked,
    );
    state.pin_bounds = Some(rect);
    state.pin_anchor = state
        .pinned
        .and_then(|p| live.iter().find(|a| a.icao == p))
        .and_then(|a| a.lat.zip(a.lon))
        .map(|(lat, lon)| project(lat, lon))
        .filter(|c| rect.contains(*c));

    // Only where it is true: over a station the double-click tunes instead, and a
    // hint that promises the wrong gesture is worse than none.
    if manual && resp.hovered() && !station_under_pointer {
        p.text(
            rect.right_bottom() + vec2(-6.0, -4.0),
            Align2::RIGHT_BOTTOM,
            tr!("adsbmap-double-click-reframe"),
            FontId::proportional(9.0),
            alpha(Color32::WHITE, 110.0),
        );
    }
    // A **tune wins over the aircraft pick**: a double-click on a station is a
    // request for a frequency, and the click that selected an aircraft under it
    // is the lesser answer.
    tune
}

// ── the pinned card ───────────────────────────────────────────────────────

/// Gap between an aircraft's symbol and the card pinned beside it, points.
pub const PIN_GAP: f32 = 14.0;

/// The pin after a click this frame: an aircraft under the pointer takes it, a
/// click on empty map closes it, no click changes nothing.
pub fn pin_after_click(pinned: Option<u32>, hit: Option<u32>, clicked: bool) -> Option<u32> {
    match (clicked, hit) {
        (true, Some(icao)) => Some(icao),
        (true, None) => None,
        _ => pinned,
    }
}

/// Which map owns the pinned card. A click on an aircraft hands it to the map
/// clicked; otherwise the owner keeps it while it is still being drawn, and a
/// map takes it over once the owner has stopped (its window was closed).
pub fn pin_owner_after(
    owner: Option<(eframe::egui::Id, u64)>,
    me: eframe::egui::Id,
    frame: u64,
    clicked_aircraft: bool,
) -> Option<(eframe::egui::Id, u64)> {
    match owner {
        Some((id, _)) if id == me => Some((me, frame)),
        Some((_, seen)) if !clicked_aircraft && frame <= seen + 2 => owner,
        _ => Some((me, frame)),
    }
}

/// Whether this map draws the pinned card.
pub fn owns_pin(owner: Option<(eframe::egui::Id, u64)>, me: eframe::egui::Id) -> bool {
    owner.is_none_or(|(id, _)| id == me)
}

/// The pin after its close button.
pub fn pin_after_close(_pinned: Option<u32>) -> Option<u32> {
    None
}

/// The pin after a key press: Esc closes it unless a text field has the keyboard.
pub fn pin_after_key(pinned: Option<u32>, esc: bool, typing: bool) -> Option<u32> {
    if esc && !typing { None } else { pinned }
}

/// The pin once the aircraft list is known: it closes with its aircraft, which
/// is gone from the list or no longer has a fresh enough position to be drawn.
pub fn pin_keep(
    pinned: Option<u32>,
    aircraft: &[AdsbAircraft],
    now: i64,
    drop_map_s: u16,
) -> Option<u32> {
    let icao = pinned?;
    aircraft
        .iter()
        .find(|a| a.icao == icao)
        .filter(|a| !a.pos_stale(now, drop_map_s))
        .map(|a| a.icao)
}

/// Whether the map draws this aircraft: the search lets it through, or it is
/// the one whose card is pinned or open, which must not vanish from under its
/// own card because the box is narrower than it was.
pub fn map_visible(
    visible: Option<&std::collections::HashSet<u32>>,
    icao: u32,
    pinned: Option<u32>,
    selected: Option<u32>,
) -> bool {
    visible.is_none_or(|set| set.contains(&icao)) || pinned == Some(icao) || selected == Some(icao)
}

/// Whether the hover card is drawn over this aircraft: not over the one whose
/// card is already pinned, which would only say the same thing twice.
pub fn hover_wanted(pinned: Option<u32>, icao: u32) -> bool {
    pinned != Some(icao)
}

/// Top-left of a card of `size` pinned to an aircraft at `anchor`: below and to
/// the right of it, flipped to the other side where that would leave the map,
/// and kept inside `bounds` whatever is left.
pub fn pin_placement(anchor: Pos2, size: Vec2, bounds: Rect) -> Pos2 {
    let mut x = anchor.x + PIN_GAP;
    if x + size.x > bounds.right() {
        x = anchor.x - PIN_GAP - size.x;
    }
    let mut y = anchor.y + PIN_GAP;
    if y + size.y > bounds.bottom() {
        y = anchor.y - PIN_GAP - size.y;
    }
    // Last, and the minimum last of all: a card bigger than the map is pinned to
    // its top-left corner rather than asked to fit where it cannot.
    pos2(
        x.min(bounds.right() - size.x).max(bounds.left()),
        y.min(bounds.bottom() - size.y).max(bounds.top()),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_click_on_an_airport_opens_its_box_and_the_map_closes_it() {
        // An airport under the pointer takes the pin.
        assert_eq!(airport_pin_after_click(None, false, Some(7), true), Some(7));
        // Another airport moves it.
        assert_eq!(airport_pin_after_click(Some(7), false, Some(9), true), Some(9));
        // An aircraft under the pointer keeps the click for itself.
        assert_eq!(airport_pin_after_click(Some(7), true, Some(9), true), Some(7));
        // Empty map closes it; no click changes nothing.
        assert_eq!(airport_pin_after_click(Some(7), false, None, true), None);
        assert_eq!(airport_pin_after_click(Some(7), false, None, false), Some(7));
    }

    #[test]
    fn escape_closes_the_airport_box() {
        assert_eq!(airport_pin_after_key(Some(3), true), None);
        assert_eq!(airport_pin_after_key(Some(3), false), Some(3));
        assert_eq!(airport_pin_after_key(None, true), None);
    }

    #[test]
    fn the_airport_card_says_codes_name_field_distance_and_traffic() {
        let a = crate::airdata::Airport {
            // Not a real ident, so the table has no runways for it and the
            // card is only its first lines.
            ident: "ZZZZ".into(),
            name: "Amsterdam Airport Schiphol".into(),
            iata: "AMS".into(),
            icao: "ZZZZ".into(),
            large: true,
            lat: 52.31,
            lon: 4.76,
            elev_ft: -11,
        };
        let mut near = AdsbAircraft::new(1, 0);
        near.lat = Some(52.4);
        near.lon = Some(4.8);
        let mut far = AdsbAircraft::new(2, 0);
        far.lat = Some(48.0);
        far.lon = Some(2.0);
        sdroxide_types::AirUnits::Aviation.set_current();
        let t = airport_card_text(&a, Some((52.0, 5.0)), &[&near, &far], true);
        let lines: Vec<&str> = t.lines().collect();
        assert_eq!(lines[0], "AMS / ZZZZ");
        assert_eq!(lines[1], "Amsterdam Airport Schiphol");
        assert_eq!(lines[2], "large airport · field -11 ft / -3 m");
        assert!(lines[3].ends_with("° from your station"), "{t}");
        assert_eq!(lines[4], "1 aircraft within 100 km / 62 mi", "{t}");
        // No home, nobody near: only the first three lines.
        let bare = airport_card_text(&a, None, &[&far], true);
        assert_eq!(bare.lines().count(), 3, "{bare}");
        sdroxide_types::AirUnits::Metric.set_current();
        assert!(airport_card_text(&a, None, &[], true).contains("field -3 m"));
        sdroxide_types::AirUnits::Aviation.set_current();
    }

    fn real_airport(ident: &str) -> &'static crate::airdata::Airport {
        crate::airdata::airports().iter().find(|a| a.ident == ident).expect("in the table")
    }

    /// The pinned card lists runways and frequencies from the embedded tables;
    /// the hover card does not (it stays short).
    #[test]
    fn the_pinned_airport_card_lists_runways_and_frequencies() {
        let a = real_airport("EHAM");
        sdroxide_types::AirUnits::Aviation.set_current();
        let full = airport_card_text(a, None, &[], true);
        assert!(full.contains("18R/36L · 1"), "{full}");
        assert!(full.contains("asphalt") && full.contains("lit"), "{full}");
        assert!(full.contains("TWR 1"), "{full}");
        assert!(full.contains("Frequencies, MHz"), "{full}");
        assert!(
            full.lines().last().unwrap().starts_with("runway and frequency data of 20"),
            "{full}"
        );
        // More than four runways: the rest is a count, not a long list.
        let extra = crate::airdata::runways_of("EHAM").len() - CARD_RUNWAYS;
        assert!(full.contains(&format!("+{extra} more")), "{full}");
        let hover = airport_card_text(a, None, &[], false);
        assert!(!hover.contains("18R") && !hover.contains("TWR"), "{hover}");
    }

    /// Lengths are written in both systems, the chosen one first: feet then
    /// metres in the aviation and imperial sets, metres then feet in the metric one.
    #[test]
    fn runway_lengths_follow_the_unit_set() {
        let a = real_airport("EHAM");
        let ft = crate::airdata::runways_of("EHAM")[0].length_ft;
        let m = (f64::from(ft) * 0.3048).round() as i32;
        for (units, want) in [
            (sdroxide_types::AirUnits::Aviation, format!("{ft} ft / {m} m")),
            (sdroxide_types::AirUnits::Imperial, format!("{ft} ft / {m} m")),
            (sdroxide_types::AirUnits::Metric, format!("{m} m / {ft} ft")),
        ] {
            units.set_current();
            let t = airport_card_text(a, None, &[], true);
            assert!(t.contains(&want), "{units:?}: {t}");
        }
        sdroxide_types::AirUnits::Aviation.set_current();
    }

    /// 2026-10-31 13:05:00 UTC.
    const CLOCK_NOW: i64 = 1_793_451_900;

    fn clock_of(clock: AirportClock) -> AirportWeather {
        AirportWeather { airport: 0, clock, now: CLOCK_NOW }
    }

    fn pinned_text(clock: AirportClock) -> String {
        airport_card_text_with(real_airport("EHAM"), None, &[], true, Some(&clock_of(clock)))
    }

    /// One test per unit set: the same moment and the same air, written in the
    /// set's own clock and degrees.
    #[test]
    fn the_pinned_card_gives_the_local_time_and_temperature_in_each_unit_set() {
        let known = AirportClock::Known { utc_offset_s: 3600, temp_c: Some(11.6) };
        for (units, time, temp) in [
            (sdroxide_types::AirUnits::Aviation, "14:05", "12 °C / 53 °F"),
            (sdroxide_types::AirUnits::Metric, "14:05", "12 °C / 53 °F"),
            (sdroxide_types::AirUnits::Imperial, "2:05 PM", "53 °F / 12 °C"),
        ] {
            units.set_current();
            let t = pinned_text(known);
            assert!(
                t.contains(&format!("Local time {time} (UTC+1) · Temperature {temp}")),
                "{units:?}: {t}"
            );
            assert!(t.contains("Weather: Open-Meteo.com"), "{units:?}: {t}");
            assert!(!t.contains("offline"), "{units:?}: {t}");
        }
        sdroxide_types::AirUnits::Aviation.set_current();
    }

    #[test]
    fn without_a_temperature_the_local_time_stands_alone() {
        sdroxide_types::AirUnits::Aviation.set_current();
        let t = pinned_text(AirportClock::Known { utc_offset_s: 19_800, temp_c: None });
        assert!(t.contains("Local time 18:35 (UTC+5:30)"), "{t}");
        assert!(!t.contains("Temperature") && !t.contains(" · Temp"), "{t}");
    }

    /// Offline the card gives UTC and says why there is no local time. It never
    /// puts a local time in its place.
    #[test]
    fn offline_or_waiting_the_card_says_utc_and_never_invents_a_local_time() {
        sdroxide_types::AirUnits::Aviation.set_current();
        let off = pinned_text(AirportClock::Offline);
        assert!(off.contains("UTC 13:05 (local time: offline)"), "{off}");
        assert!(!off.contains("Local time") && !off.contains("Temperature"), "{off}");
        assert!(!off.contains("Open-Meteo"), "no credit for data it did not use: {off}");
        let wait = pinned_text(AirportClock::Asking);
        assert!(wait.contains("UTC 13:05 (local time: loading…)"), "{wait}");
        sdroxide_types::AirUnits::Imperial.set_current();
        assert!(pinned_text(AirportClock::Offline).contains("UTC 1:05 PM (local time: offline)"));
        sdroxide_types::AirUnits::Aviation.set_current();
    }

    /// The hover box asks nothing of the station, so it says nothing of the clock.
    #[test]
    fn only_the_pinned_card_carries_the_clock() {
        sdroxide_types::AirUnits::Aviation.set_current();
        let a = real_airport("EHAM");
        let w = clock_of(AirportClock::Known { utc_offset_s: 3600, temp_c: Some(11.6) });
        let hover = airport_card_text_with(a, None, &[], false, Some(&w));
        assert!(!hover.contains("Local time") && !hover.contains("UTC"), "{hover}");
        let none = airport_card_text(a, None, &[], true);
        assert!(!none.contains("Local time") && !none.contains("UTC 1"), "{none}");
    }

    /// The clock is read before the runways: it is what a person glances for.
    #[test]
    fn the_clock_comes_before_the_runways() {
        sdroxide_types::AirUnits::Aviation.set_current();
        let t = pinned_text(AirportClock::Known { utc_offset_s: 3600, temp_c: Some(11.6) });
        let (clock, runway) = (t.find("Local time").unwrap(), t.find("18R").unwrap());
        assert!(clock < runway, "{t}");
    }

    /// A zone behind UTC, and one that carries the date over midnight.
    #[test]
    fn the_local_time_follows_zones_behind_and_ahead_of_utc() {
        sdroxide_types::AirUnits::Aviation.set_current();
        let west = pinned_text(AirportClock::Known { utc_offset_s: -5 * 3600, temp_c: None });
        assert!(west.contains("Local time 08:05 (UTC-5)"), "{west}");
        let east = pinned_text(AirportClock::Known { utc_offset_s: 11 * 3600, temp_c: None });
        assert!(east.contains("Local time 00:05 (UTC+11)"), "{east}");
        let utc = pinned_text(AirportClock::Known { utc_offset_s: 0, temp_c: None });
        assert!(utc.contains("Local time 13:05 (UTC)"), "{utc}");
    }

    /// One airport, one card: the VOR of Épinal is listed on the card of
    /// Épinal-Mirecourt with its frequency, has no symbol of its own to steal
    /// the click, and a field with no published frequency says so.
    #[test]
    fn the_vor_on_the_field_is_on_the_airports_card() {
        sdroxide_types::AirUnits::Aviation.set_current();
        let a = real_airport("LFSG");
        let text = airport_card_text(a, None, &[], true);
        assert!(text.contains("Radio aids on the field"), "{text}");
        assert!(text.contains("EPL · VOR"), "{text}");
        assert!(text.contains("113.000 MHz"), "{text}");
        let hover = airport_card_text(a, None, &[], false);
        assert!(!hover.contains("EPL · VOR"), "{hover}");
        // The aid stands on the field, so the map keeps only the airport.
        let ni = crate::airdata::navaids()
            .iter()
            .position(|n| n.ident == "EPL" && n.country == "FR")
            .unwrap();
        assert!(crate::airdata::navaid_airport(ni).is_some());
    }

    /// An airport the tables know nothing about: no runway line, no frequency
    /// line, no "data of" line — nothing invented.
    #[test]
    fn an_airport_without_data_gets_no_detail_lines() {
        let mut a = real_airport("EHAM").clone();
        a.ident = "ZZZZ".into();
        assert!(airport_detail_lines(&a).is_empty());
    }

    /// "Near an airport" is 100 km: 80 km counts, 120 km does not.
    #[test]
    fn aircraft_within_100_km_of_an_airport_are_counted() {
        let a = real_airport("EHAM");
        let mut at = |dlat: f64| {
            let mut ac = AdsbAircraft::new(1, 0);
            ac.lat = Some(a.lat + dlat);
            ac.lon = Some(a.lon);
            ac
        };
        let (inside, outside) = (at(80.0 / 111.19), at(120.0 / 111.19));
        sdroxide_types::AirUnits::Aviation.set_current();
        assert!(
            airport_card_text(a, None, &[&inside], false)
                .contains("1 aircraft within 100 km / 62 mi")
        );
        assert!(!airport_card_text(a, None, &[&outside], false).contains("within"));
        sdroxide_types::AirUnits::Imperial.set_current();
        assert!(airport_card_text(a, None, &[&inside], false).contains("within 62 mi / 100 km"));
        sdroxide_types::AirUnits::Aviation.set_current();
    }

    /// The card as drawn on a blank 360×300 map, headless: what it took.
    fn drawn_card(text: &str, photo: CardPhoto, credit: Option<&str>) -> CardDrawn {
        use eframe::egui;
        let size = egui::vec2(360.0, 300.0);
        let ctx = egui::Context::default();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            time: Some(1.0),
            ..Default::default()
        };
        let mut got = None;
        let mut full = ctx.run_ui(input, |ui| {
            let h = NavHover {
                at: egui::pos2(10.0, size.y - 10.0),
                text: text.to_string(),
                freq_khz: None,
                airport: Some(0),
            };
            let rect = egui::Rect::from_min_size(egui::Pos2::ZERO, size);
            got = Some(draw_hover_card(ui.painter(), rect, theme::map(), &h, photo, credit));
        });
        full.textures_delta.clear();
        got.unwrap()
    }

    fn ready(w: f32, h: f32) -> CardPhoto {
        CardPhoto::Ready {
            texture: eframe::egui::TextureId::default(),
            size: eframe::egui::vec2(w, h),
        }
    }

    #[test]
    fn a_picture_is_fitted_into_the_card_with_its_proportions_kept() {
        let wide = photo_fit(vec2(320.0, 213.0));
        assert!((wide.x - CARD_PHOTO_W).abs() < 0.01, "{wide:?}");
        assert!((wide.x / wide.y - 320.0 / 213.0).abs() < 0.01);
        let tall = photo_fit(vec2(100.0, 400.0));
        assert!((tall.y - CARD_PHOTO_MAX_H).abs() < 0.01 && tall.x < CARD_PHOTO_W, "{tall:?}");
        assert_eq!(photo_fit(vec2(0.0, 10.0)), Vec2::ZERO, "an empty picture takes no room");
    }

    #[test]
    fn a_card_without_a_picture_is_as_it_was_and_a_picture_adds_its_own_room() {
        let text = "AMS / EHAM\nAmsterdam Airport Schiphol";
        let bare = drawn_card(text, CardPhoto::None, Some("Photo: Wikipedia"));
        assert!(bare.credit.is_none(), "no picture, no credit");
        let waiting = drawn_card(text, CardPhoto::Pending, Some("Photo: Wikipedia"));
        assert!(waiting.credit.is_none(), "the credit goes with a picture that is drawn");
        assert!(
            (waiting.box_.height() - bare.box_.height() - (CARD_PHOTO_PENDING_H + 2.0 + 4.0)).abs()
                < 0.5,
            "room is kept for what is on its way"
        );
        let full = drawn_card(text, ready(320.0, 213.0), Some("Photo: Wikipedia"));
        let credit = full.credit.expect("a credit under the picture");
        assert!(full.box_.contains_rect(credit), "{credit:?} in {:?}", full.box_);
        assert!(full.box_.height() > bare.box_.height() + 100.0);
        assert!(full.box_.width() >= CARD_PHOTO_W, "the card is as wide as its picture");
        // The credit sits between the picture and the text.
        assert!(credit.top() > full.box_.top() + 100.0);
    }

    #[test]
    fn a_card_with_a_picture_stays_on_the_map() {
        let full = drawn_card("AMS / EHAM", ready(320.0, 213.0), Some("Photo: Wikipedia"));
        let map = eframe::egui::Rect::from_min_size(Pos2::ZERO, vec2(360.0, 300.0));
        assert!(map.contains_rect(full.box_), "{:?}", full.box_);
    }

    /// A stand-in for a Wikipedia thumbnail (sky, field, runway), 320×213 as the
    /// real ones are. Not a photograph of anything.
    fn stand_in_picture() -> eframe::egui::ColorImage {
        let (w, h) = (320usize, 213usize);
        let mut px = Vec::with_capacity(w * h);
        for y in 0..h {
            for x in 0..w {
                let horizon = 90;
                let c = if y < horizon {
                    let t = y as f32 / horizon as f32;
                    Color32::from_rgb((90.0 + 90.0 * t) as u8, (140.0 + 70.0 * t) as u8, 235)
                } else if (x as i32 - 160).abs() < ((y - horizon) as i32 / 3 + 4) {
                    Color32::from_gray(70)
                } else {
                    Color32::from_rgb(70, 130, 60)
                };
                px.push(c);
            }
        }
        eframe::egui::ColorImage::new([w, h], px)
    }

    /// Not a check: writes PNGs of the real pinned airport card, with a stand-in
    /// picture, with the local time and temperature known, on their way and
    /// offline, for a person to look at, in the three unit sets.
    /// `cargo test -p sdroxide-ui --lib airport_card_preview -- --ignored`
    #[test]
    #[ignore]
    fn airport_card_preview() {
        use eframe::egui;
        let dir = std::env::var("PREVIEW_DIR").unwrap_or_else(|_| "../../target/previews".into());
        std::fs::create_dir_all(&dir).unwrap();
        let a = real_airport("EHAM");
        let known = AirportClock::Known { utc_offset_s: 3600, temp_c: Some(11.6) };
        let boards = [
            (sdroxide_types::AirUnits::Aviation, "aviation", "ready", known),
            (sdroxide_types::AirUnits::Metric, "metric", "ready", known),
            (sdroxide_types::AirUnits::Imperial, "imperial", "ready", known),
            (sdroxide_types::AirUnits::Aviation, "offline", "none", AirportClock::Offline),
            (sdroxide_types::AirUnits::Aviation, "asking", "pending", AirportClock::Asking),
        ];
        for (units, name, state, clock) in boards {
            units.set_current();
            let weather = AirportWeather { airport: 0, clock, now: CLOCK_NOW };
            let text = format!(
                "{}\n{}",
                airport_card_text_with(a, Some((52.0, 5.0)), &[], true, Some(&weather)),
                tr!("adsbmap-click-to-close")
            );
            let size = egui::vec2(380.0, 460.0);
            let ctx = egui::Context::default();
            let handle =
                ctx.load_texture("preview-airport", stand_in_picture(), Default::default());
            let photo = match state {
                "ready" => CardPhoto::Ready { texture: handle.id(), size: vec2(320.0, 213.0) },
                "pending" => CardPhoto::Pending,
                _ => CardPhoto::None,
            };
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                time: Some(1.0),
                ..Default::default()
            };
            let mut tex = crate::test_raster::Textures::default();
            let mut out = None;
            for _ in 0..3 {
                let mut full = ctx.run_ui(input.clone(), |ui| {
                    let p = ui.painter().clone();
                    let map = theme::map();
                    p.rect_filled(egui::Rect::from_min_size(egui::Pos2::ZERO, size), 0.0, map.land);
                    let h = NavHover {
                        at: egui::pos2(10.0, size.y - 10.0),
                        text: text.clone(),
                        freq_khz: None,
                        airport: Some(0),
                    };
                    draw_hover_card(
                        &p,
                        egui::Rect::from_min_size(egui::Pos2::ZERO, size),
                        map,
                        &h,
                        photo,
                        Some(&tr!("adsbmap-airport-photo-credit")),
                    );
                });
                tex.apply(&full.textures_delta);
                full.textures_delta.clear();
                out = Some(full);
            }
            let img =
                crate::test_raster::rasterize(&ctx, out.unwrap(), &tex, size, theme::BG_DEEP());
            img.save(format!("{dir}/pr4_airport_card_{name}.png")).unwrap();
        }
        sdroxide_types::AirUnits::Aviation.set_current();
    }

    /// One PNG of what `draw` paints on the map's land colour.
    fn icon_shot(
        dir: &str,
        name: &str,
        size: eframe::egui::Vec2,
        ppp: f32,
        draw: impl Fn(&Painter),
    ) {
        use eframe::egui;
        let ctx = egui::Context::default();
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
            time: Some(1.0),
            viewports: std::iter::once((
                egui::ViewportId::ROOT,
                egui::ViewportInfo { native_pixels_per_point: Some(ppp), ..Default::default() },
            ))
            .collect(),
            ..Default::default()
        };
        let mut tex = crate::test_raster::Textures::default();
        let mut out = None;
        for _ in 0..2 {
            let mut full = ctx.run_ui(input.clone(), |ui| {
                let p = ui.painter().clone();
                p.rect_filled(
                    egui::Rect::from_min_size(egui::Pos2::ZERO, size),
                    0.0,
                    theme::map().land,
                );
                draw(&p);
            });
            tex.apply(&full.textures_delta);
            full.textures_delta.clear();
            out = Some(full);
        }
        let img = crate::test_raster::rasterize(&ctx, out.unwrap(), &tex, size, theme::BG_DEEP());
        img.save(format!("{dir}/icons_{name}.png")).unwrap();
    }

    /// Not a check: writes the five PNGs for a person to look at — every symbol
    /// together, then the real map with the aircraft, the airports, the VORs and
    /// all three on it.
    /// `cargo test -p sdroxide-ui --lib icon_previews -- --ignored`
    #[test]
    #[ignore]
    fn icon_previews() {
        use crate::adsb_icons as icons;
        use eframe::egui::{pos2, vec2};
        let dir = std::env::var("PREVIEW_DIR").unwrap_or_else(|_| "../../target/previews".into());
        std::fs::create_dir_all(&dir).unwrap();
        let map = theme::map();
        let airport_ink = |large: bool| {
            if large { alpha(map.city_label, 240.0) } else { alpha(map.city_label, 175.0) }
        };

        // 1 — every symbol, at the size it has on the map.
        icon_shot(&dir, "1_all", vec2(330.0, 60.0), 4.0, |p| {
            icons::airport(p, pos2(25.0, 30.0), AIRPORT_HALF_LARGE, airport_ink(true));
            icons::airport(p, pos2(50.0, 30.0), AIRPORT_HALF_MEDIUM, airport_ink(false));
            icons::vor(p, pos2(75.0, 30.0), VOR_HALF, alpha(map.river, 240.0));
            p.circle_filled(pos2(97.0, 30.0), 3.0, alpha(map.home, 210.0));
            icons::aircraft(p, pos2(130.0, 30.0), TARGET_R, Some(45.0), map.station, false);
            icons::aircraft(p, pos2(168.0, 30.0), TARGET_R + 3.0, Some(45.0), map.hover, false);
            icons::aircraft(p, pos2(208.0, 30.0), TARGET_R + 3.0, Some(45.0), map.dx, false);
            icons::aircraft(p, pos2(246.0, 30.0), TARGET_R, Some(45.0), map.station, true);
            icons::aircraft(p, pos2(280.0, 30.0), TARGET_R, None, map.station, false);
        });

        // The real tables over Western Europe, with a few aircraft.
        let size = vec2(760.0, 480.0);
        let (clat, clon, lon_span) = (51.0, 6.0, 12.0);
        let lat_span = lon_span * f64::from(size.y) / f64::from(size.x);
        let project = move |lat: f64, lon: f64| {
            pos2(
                size.x * 0.5 + ((lon - clon) / lon_span) as f32 * size.x,
                size.y * 0.5 - ((lat - clat) / lat_span) as f32 * size.y,
            )
        };
        let in_view = move |lat: f64, lon: f64| {
            (lat - clat).abs() <= lat_span / 2.0 && (lon - clon).abs() <= lon_span / 2.0
        };
        let zoom = nav_zoom(lon_span);
        let airports = |p: &Painter| {
            for a in crate::airdata::airports() {
                if (!a.large && !zoom.medium_airports) || !in_view(a.lat, a.lon) {
                    continue;
                }
                let half = if a.large { AIRPORT_HALF_LARGE } else { AIRPORT_HALF_MEDIUM };
                icons::airport(p, project(a.lat, a.lon), half, airport_ink(a.large));
            }
        };
        let vors = |p: &Painter| {
            for (ni, n) in crate::airdata::navaids().iter().enumerate() {
                if !in_view(n.lat, n.lon) || crate::airdata::navaid_airport(ni).is_some() {
                    continue;
                }
                let c = project(n.lat, n.lon);
                if n.kind.is_audible_on_mf() {
                    p.circle_filled(c, 3.0, alpha(map.home, 210.0));
                } else {
                    icons::vor(p, c, VOR_HALF, alpha(map.river, 240.0));
                }
            }
        };
        // A scatter that is the same every time: position and track from a counter.
        let aircraft = |p: &Painter| {
            let mut k = 7_u32;
            let mut next = |m: u32| {
                k = k.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                (k >> 16) % m
            };
            for i in 0..26 {
                let c = pos2(30.0 + next(700) as f32, 30.0 + next(420) as f32);
                let track = next(360) as f32;
                let (col, r, ground) = match i {
                    3 => (map.hover, TARGET_R + 3.0, false),
                    8 => (map.dx, TARGET_R + 3.0, false),
                    14 => (map.station, TARGET_R, true),
                    _ => (map.station, TARGET_R, false),
                };
                icons::aircraft(p, c, r, Some(track), col, ground);
            }
        };
        icon_shot(&dir, "2_map_aircraft", size, 2.0, |p| aircraft(p));
        icon_shot(&dir, "3_map_airports", size, 2.0, |p| airports(p));
        icon_shot(&dir, "4_map_vor", size, 2.0, |p| vors(p));
        icon_shot(&dir, "5_map_all", size, 2.0, |p| {
            airports(p);
            vors(p);
            aircraft(p);
        });
    }

    #[test]
    fn medium_airports_are_named_only_once_the_view_is_a_region() {
        assert!(!nav_zoom(10.0).medium_labels && nav_zoom(10.0).airport_labels);
        assert!(nav_zoom(4.0).medium_labels);
        assert!(nav_zoom(1.0).medium_labels);
    }

    use super::*;

    // ── the pinned card ───────────────────────────────────────────────────

    fn on_map(icao: u32, pos_age: i64) -> AdsbAircraft {
        let mut a = AdsbAircraft::new(icao, 1_000);
        a.lat = Some(52.0);
        a.lon = Some(5.0);
        a.last_pos_at = 1_000 - pos_age;
        a
    }

    #[test]
    fn a_click_on_an_aircraft_pins_it() {
        assert_eq!(pin_after_click(None, Some(0x4840D6), true), Some(0x4840D6));
        assert_eq!(pin_after_click(None, Some(0x4840D6), false), None, "no click, no pin");
    }

    #[test]
    fn a_click_on_empty_map_closes_the_pin() {
        assert_eq!(pin_after_click(Some(0x4840D6), None, true), None);
        assert_eq!(pin_after_click(None, None, true), None);
    }

    #[test]
    fn a_click_on_another_aircraft_moves_the_pin() {
        assert_eq!(pin_after_click(Some(0x4840D6), Some(0x484006), true), Some(0x484006));
    }

    #[test]
    fn the_pin_survives_a_frame_without_a_click() {
        assert_eq!(pin_after_close(Some(0x4840D6)), None);
        assert_eq!(pin_after_click(Some(0x4840D6), None, false), Some(0x4840D6));
        assert_eq!(pin_after_click(Some(0x4840D6), Some(0x484006), false), Some(0x4840D6));
    }

    #[test]
    fn escape_closes_the_pin_unless_a_text_field_has_the_keyboard() {
        assert_eq!(pin_after_key(Some(7), true, false), None);
        assert_eq!(pin_after_key(Some(7), true, true), Some(7), "typing keeps its Esc");
        assert_eq!(pin_after_key(Some(7), false, false), Some(7));
        assert_eq!(pin_after_key(None, true, false), None);
    }

    #[test]
    fn a_pin_whose_aircraft_is_gone_closes() {
        let list = [on_map(1, 0), on_map(2, 0)];
        assert_eq!(pin_keep(Some(1), &list, 1_000, 60), Some(1));
        assert_eq!(pin_keep(Some(9), &list, 1_000, 60), None, "9 left the list");
        assert_eq!(pin_keep(None, &list, 1_000, 60), None);
    }

    #[test]
    fn a_pin_whose_position_expired_closes() {
        let list = [on_map(1, 61), on_map(2, 59)];
        assert_eq!(pin_keep(Some(1), &list, 1_000, 60), None, "off the map, so off the card");
        assert_eq!(pin_keep(Some(2), &list, 1_000, 60), Some(2));
    }

    #[test]
    fn hovering_the_pinned_aircraft_adds_no_second_card() {
        assert!(!hover_wanted(Some(5), 5));
        assert!(hover_wanted(Some(5), 6));
        assert!(hover_wanted(None, 5));
    }

    fn bounds() -> Rect {
        Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 600.0))
    }

    #[test]
    fn the_pinned_card_sits_beside_the_aircraft_and_stays_in_the_map() {
        let size = vec2(200.0, 100.0);
        let anchor = pos2(100.0, 100.0);
        let at = pin_placement(anchor, size, bounds());
        let r = Rect::from_min_size(at, size);
        assert!(!r.contains(anchor), "the card covers its own aircraft");
        assert!(r.min.x > anchor.x && r.min.y > anchor.y, "below and to the right: {r:?}");
        assert!(r.min.x - anchor.x <= 2.0 * PIN_GAP, "and close to it: {r:?}");

        // Near the right edge it goes to the left of the aircraft.
        let anchor = pos2(780.0, 100.0);
        let r = Rect::from_min_size(pin_placement(anchor, size, bounds()), size);
        assert!(r.max.x <= anchor.x, "{r:?}");
        assert!(bounds().contains_rect(r), "{r:?}");

        // Near the bottom it goes above.
        let anchor = pos2(100.0, 590.0);
        let r = Rect::from_min_size(pin_placement(anchor, size, bounds()), size);
        assert!(r.max.y <= anchor.y, "{r:?}");
        assert!(bounds().contains_rect(r), "{r:?}");

        // In the corner, both.
        let anchor = pos2(795.0, 595.0);
        let r = Rect::from_min_size(pin_placement(anchor, size, bounds()), size);
        assert!(r.max.x <= anchor.x && r.max.y <= anchor.y, "{r:?}");
        assert!(!r.contains(anchor));
    }

    #[test]
    fn a_card_bigger_than_the_map_is_pinned_to_its_corner_without_panicking() {
        let tiny = Rect::from_min_size(pos2(10.0, 20.0), vec2(120.0, 80.0));
        let at = pin_placement(pos2(60.0, 50.0), vec2(440.0, 300.0), tiny);
        assert_eq!(at, tiny.min);
    }

    /// **The plain tooltip is what it always was.** The hover card replaces it
    /// only where the app hands the map one; everywhere else, and for anything
    /// the card does not cover, this text is the answer.
    #[test]
    fn the_plain_tooltip_says_what_it_always_said() {
        let mut a = AdsbAircraft::new(0x4840D6, 100);
        a.callsign = "KLM1234".into();
        a.category = Some("heavy".into());
        a.lat = Some(52.3);
        a.lon = Some(4.8);
        a.altitude_ft = Some(35_000);
        a.ground_speed_kt = Some(450.0);
        a.vertical_rate_fpm = Some(1800);
        a.emergency = Some("general emergency".into());
        let home = (52.0, 5.0);
        let km = sdroxide_types::distance_km(home, (52.3, 4.8));
        let bear = sdroxide_types::bearing_deg(home, (52.3, 4.8));
        assert_eq!(
            tooltip_text(&a, Some(home)),
            format!(
                "KLM1234  4840D6\nheavy\n35000 ft   450 kt\n+1800 ft/min\n{km:.0} km   {bear:.0}°\ngeneral emergency"
            )
        );
        let mut b = AdsbAircraft::new(0x123456, 100);
        b.vertical_rate_fpm = Some(50);
        assert_eq!(tooltip_text(&b, Some(home)), "123456\n0 ft   --- kt");
    }

    /// **A hover names the thing under the pointer**, in the words a chart
    /// would use: the abbreviation first for an airport, and for a navaid its
    /// ident, what it is, and the frequency on the unit it is published in —
    /// kHz and "medium wave" for the ones a listener can actually hear.
    #[test]
    fn what_a_hovered_nav_feature_says() {
        let ams = crate::airdata::airports()
            .iter()
            .find(|a| a.iata == "AMS")
            .expect("Schiphol is in the table");
        // The name is the source's, and the field is wide enough to hold it
        // whole — the first cut gave names 20 bytes and the hover read
        // "Amsterdam Airport Sc". Asserted from the table rather than from
        // memory: this line has been wrong twice.
        assert_eq!(airport_hover_text(ams), "AMS · Amsterdam Airport Schiphol");

        // An airport with no IATA code falls back to its ident rather than
        // showing a gap where the abbreviation should be.
        let mut unnamed = ams.clone();
        unnamed.iata.clear();
        unnamed.ident = "EHAM".into();
        assert!(airport_hover_text(&unnamed).starts_with("EHAM · "));

        // A medium-wave beacon says kHz, because that is what a receiver dial
        // is marked in, and says that it is audible.
        let ndb = crate::airdata::navaids()
            .iter()
            .find(|n| n.kind == crate::airdata::NavaidKind::Ndb)
            .expect("there are NDBs");
        let text = navaid_hover_text(ndb);
        assert!(text.contains("NDB"), "{text}");
        assert!(text.contains("kHz (medium wave)"), "{text}");

        // And a VHF aid says MHz, to three places — the spacing the band uses.
        let vor = crate::airdata::navaids()
            .iter()
            .find(|n| n.kind == crate::airdata::NavaidKind::VorDme)
            .expect("there are VOR/DMEs");
        let text = navaid_hover_text(vor);
        assert!(text.contains("VOR/DME"), "{text}");
        assert!(text.contains("MHz"), "{text}");
        assert!(!text.contains("kHz"), "{text}");
    }

    /// The reported fault: the VOR of Épinal stood next to its airport, was
    /// the nearer of the two to the pointer and took the click, so the airport
    /// could not be pinned. With the aid attached to the field, the pointer on
    /// the VOR's own position finds the airport and nothing else.
    #[test]
    fn the_vor_beside_an_airport_no_longer_takes_the_click() {
        let ctx = eframe::egui::Context::default();
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(600.0, 400.0));
        let (clat, clon, lon_span) = (48.3, 6.07, 3.0);
        let lat_span = lon_span * 0.66;
        let project = |lat: f64, lon: f64| {
            pos2(
                rect.left() + (0.5 + (wrap180(lon - clon) / lon_span) as f32) * rect.width(),
                rect.top() + (0.5 - ((lat - clat) / lat_span) as f32) * rect.height(),
            )
        };
        let map = theme::map_styled(sdroxide_types::MapStyle::Radar);
        let airports = crate::airdata::airports();
        let ai = airports.iter().position(|a| a.ident == "LFSG").unwrap();
        let epl = crate::airdata::navaids()
            .iter()
            .find(|n| n.ident == "EPL" && n.country == "FR")
            .unwrap();
        let on_vor = project(epl.lat, epl.lon);
        let mut got = None;
        let input = eframe::egui::RawInput { screen_rect: Some(rect), ..Default::default() };
        ctx.run_ui(input, |ui| {
            let p = ui.painter().clone();
            got = draw_nav_layer(
                Some(&p),
                &map,
                MapLayers { paths: true, nav: true },
                clat,
                clon,
                lon_span,
                lat_span,
                &project,
                Some(on_vor),
            );
        })
        .drop_without_applying_deltas();
        let hit = got.expect("the airport is within reach of the pointer");
        assert_eq!(hit.airport, Some(ai), "{}", hit.text);
    }

    /// **The hover finds the airport under the pointer**, on the real tables
    /// and the real filter — not a re-implementation of either.
    ///
    /// `draw_nav_layer` takes a painter and numbers, so it can be driven without
    /// a window: a painter over a rect, a span, and a pointer placed where
    /// Schiphol projects to. That is the whole path the mouse takes.
    #[test]
    fn the_hover_finds_the_airport_under_the_pointer() {
        let ctx = eframe::egui::Context::default();
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(600.0, 400.0));
        // Schiphol, and a view tight enough that the layer is switched on.
        let (clat, clon, lon_span) = (52.3, 4.75, 6.0);
        let lat_span = lon_span * 0.66;
        let project = |lat: f64, lon: f64| {
            pos2(
                rect.left() + (0.5 + (wrap180(lon - clon) / lon_span) as f32) * rect.width(),
                rect.top() + (0.5 - ((lat - clat) / lat_span) as f32) * rect.height(),
            )
        };
        let map = theme::map_styled(sdroxide_types::MapStyle::Radar);
        let ams = crate::airdata::airports()
            .iter()
            .find(|a| a.iata == "AMS")
            .expect("Schiphol is in the table");
        let at = project(ams.lat, ams.lon);
        let sea = pos2(rect.left() + 3.0, rect.top() + 3.0);

        // Inside a frame: egui has no fonts until the first `run`, and the
        // layer lays text out.
        let mut asked = Vec::new();
        let input = eframe::egui::RawInput { screen_rect: Some(rect), ..Default::default() };
        let full = ctx.run_ui(input, |ui| {
            let p = ui.painter().clone();
            for (layers, pointer) in [
                (MapLayers { paths: true, nav: true }, Some(at)),
                (MapLayers { paths: true, nav: true }, Some(sea)),
                (MapLayers { paths: true, nav: false }, Some(at)),
            ] {
                asked.push(draw_nav_layer(
                    Some(&p),
                    &map,
                    layers,
                    clat,
                    clon,
                    lon_span,
                    lat_span,
                    &project,
                    pointer,
                ));
            }
        });
        full.drop_without_applying_deltas();

        let named = asked[0].as_ref().expect("the pointer is on the airport and nothing was named");
        assert!(named.text.starts_with("AMS · "), "named {:?}", named.text);
        assert!(
            asked[1].is_none(),
            "the sea named something: {:?}",
            asked[1].as_ref().map(|h| &h.text)
        );
        assert!(
            asked[2].is_none(),
            "the layer is off and the hover still answered — a name for a mark that is not drawn"
        );
    }

    /// **A station with a frequency says so, and one without does not.** The
    /// hover is what offers the gesture, and the frequency it carries is what a
    /// double-click tunes to — so this is the whole feature's data path, read
    /// from an aid the table actually has.
    #[test]
    fn the_hover_carries_the_frequency_a_station_can_be_tuned_to() {
        let ctx = eframe::egui::Context::default();
        let rect = Rect::from_min_size(pos2(0.0, 0.0), vec2(600.0, 400.0));
        let map = theme::map_styled(sdroxide_types::MapStyle::Radar);
        // An aid that is on a frequency, and the view tight enough for the
        // navaids to be drawn at all.
        let ndb = crate::airdata::navaids()
            .iter()
            .find(|n| n.freq_khz > 0)
            .expect("a navaid with a frequency");
        let (clat, clon) = (ndb.lat, ndb.lon);
        let lon_span = 0.5;
        let lat_span = lon_span * 0.66;
        let project =
            |lat: f64, lon: f64| project_at(rect, clat, clon, lon_span, lat_span, lat, lon);
        let at = project(ndb.lat, ndb.lon);
        let input = eframe::egui::RawInput { screen_rect: Some(rect), ..Default::default() };
        let mut hover = None;
        let full = ctx.run_ui(input, |ui| {
            let p = ui.painter().clone();
            hover = draw_nav_layer(
                Some(&p),
                &map,
                MapLayers { paths: false, nav: true },
                clat,
                clon,
                lon_span,
                lat_span,
                &project,
                Some(at),
            );
        });
        full.drop_without_applying_deltas();
        let hover = hover.expect("the pointer is on the aid and nothing was named");
        assert_eq!(
            hover.freq_khz,
            Some(ndb.freq_khz),
            "the hover did not carry the frequency its own text names"
        );
        assert!(hover.text.contains(&ndb.ident), "named the wrong thing: {:?}", hover.text);
        // And the conversion the dial is given: kHz in the table, Hz on the dial.
        assert!((tune_hz(108_400) - 108_400_000.0).abs() < 1.0, "108 400 kHz is 108.4 MHz");
    }

    /// **A station goes to the dial the mode is not using**, and that A/B pair is
    /// the operator's own reason for the gesture — the plane on one dial, the
    /// ground station on the other, switched between with the VFO key.
    #[test]
    fn a_tuned_station_goes_to_the_dial_the_mode_does_not_own() {
        assert_eq!(other_dial(Vfo::A), Vfo::B, "ADS-B holds A, so the station takes B");
        assert_eq!(other_dial(Vfo::B), Vfo::A);
    }

    /// **The filter is the feature, and it only opens.** Zooming in must never
    /// take something away: a layer that loses the medium airports when you get
    /// close, or the navaids when you get closer, reads as the map breaking
    /// rather than as a rule.
    #[test]
    fn the_navigation_layer_only_ever_opens_as_you_zoom_in() {
        // World scale down to the tightest this map goes.
        let spans = [360.0, 180.0, 60.0, 41.0, 40.0, 20.0, 12.0, 6.0, 3.0, 0.5];
        let steps: Vec<NavZoom> = spans.iter().map(|s| nav_zoom(*s)).collect();
        for pair in steps.windows(2) {
            let (a, b) = (pair[0], pair[1]);
            assert!(
                (!a.medium_airports || b.medium_airports)
                    && (!a.airport_labels || b.airport_labels)
                    && (!a.navaids || b.navaids)
                    && (!a.navaid_labels || b.navaid_labels),
                "zooming in from {:?} to {:?} took something away",
                a,
                b
            );
        }
        // And the ends are what they should be: a world view is large airports
        // only, and the closest view labels everything.
        assert!(!steps[0].navaids && !steps[0].medium_airports, "the world view is crowded");
        assert!(
            steps[steps.len() - 1].navaids && steps[steps.len() - 1].navaid_labels,
            "the closest view does not name the stations"
        );
    }

    fn flying(kt: f32, track: f32, turn: f32) -> AdsbAircraft {
        let mut a = AdsbAircraft::new(0x3C_6444, 0);
        a.lat = Some(48.0);
        a.lon = Some(16.0);
        a.ground_speed_kt = Some(kt);
        a.track_deg = Some(track);
        a.turn_rate_deg_s = turn;
        a
    }

    /// On a busy map the blocks that give way first are those of aircraft on
    /// the ground, and an emergency gives way to nothing.
    #[test]
    fn a_parked_aircraft_gives_way_to_one_flying() {
        let mut parked = flying(0.0, 0.0, 0.0);
        parked.on_ground = true;
        let airborne = flying(250.0, 0.0, 0.0);
        let mut mayday = flying(250.0, 0.0, 0.0);
        mayday.emergency = Some("7700 general emergency".into());
        assert!(block_rank(&mayday) < block_rank(&airborne));
        assert!(block_rank(&airborne) < block_rank(&parked));
    }

    /// The leader's length is the distance covered in the vector time, in
    /// degrees of latitude — which is what makes two aircraft with equal
    /// leaders equally fast at any zoom.
    #[test]
    fn the_leader_is_as_long_as_a_minute_of_flight() {
        // 480 knots due north for one minute is 8 NM, which is 8 minutes of
        // latitude: 0.1333 degrees.
        let path = leader(&flying(480.0, 0.0, 0.0), 1.0);
        let (lat, lon) = *path.last().expect("a leader");
        assert!((lat - 48.0 - 8.0 / 60.0).abs() < 1e-6, "reached {lat}");
        assert!((lon - 16.0).abs() < 1e-9, "due north should not move east: {lon}");

        // Half the speed, half the line.
        let half = leader(&flying(240.0, 0.0, 0.0), 1.0);
        let (hlat, _) = *half.last().expect("a leader");
        assert!(((hlat - 48.0) - (lat - 48.0) / 2.0).abs() < 1e-9);
    }

    /// A degree of longitude is shorter than a degree of latitude everywhere but
    /// the equator, so an eastbound leader has to be drawn wider in degrees or
    /// it comes out short on the map.
    #[test]
    fn an_eastbound_leader_is_stretched_for_the_latitude_it_is_at() {
        let path = leader(&flying(480.0, 90.0, 0.0), 1.0);
        let (lat, lon) = *path.last().expect("a leader");
        assert!((lat - 48.0).abs() < 1e-6, "due east should not move north: {lat}");
        let want = (8.0 / 60.0) / 48.0f64.to_radians().cos();
        assert!((lon - 16.0 - want).abs() < 1e-4, "reached {lon}, wanted {}", 16.0 + want);
    }

    /// A turning aircraft gets a curved leader — the whole reason the tracker
    /// derives a turn rate at all.
    #[test]
    fn a_turning_aircraft_gets_a_bent_leader() {
        let straight = leader(&flying(300.0, 0.0, 0.0), 1.0);
        let turning = leader(&flying(300.0, 0.0, 3.0), 1.0);
        assert_eq!(straight.len(), turning.len());
        let end_s = straight.last().unwrap();
        let end_t = turning.last().unwrap();
        assert!(
            (end_t.1 - end_s.1).abs() > 0.01,
            "a standard-rate turn should visibly bend the leader"
        );
        // Every leg of a straight leader is on the same meridian; a turning
        // one's are not.
        assert!(straight.iter().all(|p| (p.1 - 16.0).abs() < 1e-9));
    }

    /// A target with nothing to draw a leader from gets none, rather than a
    /// zero-length stub or a line pointing north by default.
    #[test]
    fn a_target_with_no_velocity_gets_no_leader() {
        let mut a = flying(400.0, 90.0, 0.0);
        a.ground_speed_kt = None;
        assert!(leader(&a, 1.0).is_empty());
        let mut a = flying(400.0, 90.0, 0.0);
        a.track_deg = None;
        assert!(leader(&a, 1.0).is_empty());
        // Parked on a stand.
        assert!(leader(&flying(0.0, 90.0, 0.0), 1.0).is_empty());
        // And the operator can turn leaders off altogether.
        assert!(leader(&flying(400.0, 90.0, 0.0), 0.0).is_empty());
    }

    /// One aircraft overhead must not zoom the map to street level, and an
    /// empty sky shows the world rather than the Gulf of Guinea.
    #[test]
    fn the_auto_fit_has_a_floor_and_an_empty_default() {
        let (_, _, span) = target_view(Some((48.2, 16.37)), &[(48.2005, 16.3705)], 0.5);
        assert!(span >= MIN_LON_SPAN);
        let (_, _, span) = target_view(None, &[], 0.5);
        assert_eq!(span, 360.0);
    }

    // ── the search box on the map ─────────────────────────────────────────

    #[test]
    fn the_map_draws_what_the_search_lets_through_and_the_aircraft_with_a_card() {
        use std::collections::HashSet;
        let only: HashSet<u32> = HashSet::from([1]);
        assert!(map_visible(None, 5, None, None), "no search, everything shows");
        assert!(map_visible(Some(&only), 1, None, None));
        assert!(!map_visible(Some(&only), 2, None, None), "not matched, not drawn");
        assert!(map_visible(Some(&only), 2, Some(2), None), "the pinned one stays");
        assert!(map_visible(Some(&only), 3, None, Some(3)), "so does the selected one");
        assert!(!map_visible(Some(&only), 4, Some(2), Some(3)), "and only those");
    }

    fn map_frame(
        ctx: &eframe::egui::Context,
        state: &mut AdsbMapState,
        list: &[AdsbAircraft],
        events: Vec<eframe::egui::Event>,
    ) {
        let mut raw = eframe::egui::RawInput::default();
        raw.screen_rect = Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 600.0)));
        raw.events = events;
        let _ = crate::test_support::run_frame(ctx, raw, |ui| {
            show(
                ui,
                state,
                list,
                None,
                1_000,
                AdsbSettings::default(),
                None,
                sdroxide_types::MapStyle::Radar,
                400.0,
                Vfo::A,
                None,
            );
        });
    }

    fn at(icao: u32, lat: f64, lon: f64) -> AdsbAircraft {
        let mut a = on_map(icao, 0);
        a.lat = Some(lat);
        a.lon = Some(lon);
        a
    }

    #[test]
    fn an_aircraft_the_search_hides_does_not_pull_the_view() {
        // One over Amsterdam, one over Singapore; only the first is searched for.
        let list = [at(1, 52.0, 5.0), at(2, 1.3, 103.8)];
        let mut all = AdsbMapState::default();
        map_frame(&eframe::egui::Context::default(), &mut all, &list, vec![]);
        assert!(
            all.view.clat < 40.0,
            "the control: both in view frames them both ({})",
            all.view.clat
        );

        let mut some = AdsbMapState::default();
        some.visible = Some(std::collections::HashSet::from([1]));
        map_frame(&eframe::egui::Context::default(), &mut some, &list, vec![]);
        assert!(
            (some.view.clat - 52.0).abs() < 1.0 && (some.view.clon - 5.0).abs() < 1.0,
            "framed on the one that shows: {} {}",
            some.view.clat,
            some.view.clon
        );
    }

    /// A click on the middle of the map, where two aircraft sit on top of each
    /// other: returns which one took it.
    fn click_the_middle(visible: Option<std::collections::HashSet<u32>>) -> Option<u32> {
        use eframe::egui::{Event, PointerButton};
        // The last one in the list wins a tie, so the hidden one is last.
        let list = [at(2, 52.0, 5.0), at(1, 52.0, 5.0)];
        let mut st = AdsbMapState::default();
        st.visible = visible;
        let ctx = eframe::egui::Context::default();
        map_frame(&ctx, &mut st, &list, vec![]);
        let c = st.pin_bounds.expect("the map was drawn").center();
        let button = |pressed| Event::PointerButton {
            pos: c,
            button: PointerButton::Primary,
            pressed,
            modifiers: eframe::egui::Modifiers::NONE,
        };
        map_frame(&ctx, &mut st, &list, vec![Event::PointerMoved(c)]);
        map_frame(&ctx, &mut st, &list, vec![Event::PointerMoved(c), button(true), button(false)]);
        st.pinned
    }

    #[test]
    fn a_hidden_aircraft_is_not_hovered_or_clicked() {
        assert_eq!(
            click_the_middle(None),
            Some(1),
            "the control: with no search the top one is hit"
        );
        assert_eq!(
            click_the_middle(Some(std::collections::HashSet::from([2]))),
            Some(2),
            "with the search on 2, the click lands on 2 and not on the one over it"
        );
    }

    /// Where a target sits on the map `map_frame` draws, once it is drawn.
    fn click_at(
        ctx: &eframe::egui::Context,
        st: &mut AdsbMapState,
        list: &[AdsbAircraft],
        c: Pos2,
    ) {
        use eframe::egui::{Event, PointerButton};
        let button = |pressed| Event::PointerButton {
            pos: c,
            button: PointerButton::Primary,
            pressed,
            modifiers: eframe::egui::Modifiers::NONE,
        };
        map_frame(ctx, st, list, vec![Event::PointerMoved(c)]);
        map_frame(ctx, st, list, vec![Event::PointerMoved(c), button(true), button(false)]);
        map_frame(ctx, st, list, vec![Event::PointerMoved(c)]);
    }

    #[test]
    fn a_second_click_on_another_aircraft_moves_the_pin() {
        let list = [at(1, 52.0, 4.0), at(2, 52.0, 6.0)];
        let mut st = AdsbMapState::default();
        let ctx = eframe::egui::Context::default();
        map_frame(&ctx, &mut st, &list, vec![]);
        let bounds = st.pin_bounds.expect("the map was drawn");
        let (clat, clon, span) = (st.view.clat, st.view.clon, st.view.lon_span);
        let aspect = (bounds.height() / bounds.width()) as f64;
        let at_pos = |a: &AdsbAircraft| {
            project_at(bounds, clat, clon, span, span * aspect, a.lat.unwrap(), a.lon.unwrap())
        };
        click_at(&ctx, &mut st, &list, at_pos(&list[0]));
        assert_eq!(st.pinned, Some(1), "the first click pins the first");
        click_at(&ctx, &mut st, &list, at_pos(&list[1]));
        assert_eq!(st.pinned, Some(2), "the second click pins the second, not the first");
    }

    #[test]
    fn the_click_that_pins_draws_no_hover_card_for_it() {
        let list = [at(1, 52.0, 4.0)];
        let mut st = AdsbMapState::default();
        let ctx = eframe::egui::Context::default();
        map_frame(&ctx, &mut st, &list, vec![]);
        let bounds = st.pin_bounds.expect("the map was drawn");
        let c = bounds.center();
        let cards = std::cell::Cell::new(0);
        let card = |_: &mut eframe::egui::Ui, _: &AdsbAircraft| cards.set(cards.get() + 1);
        use eframe::egui::{Event, PointerButton};
        let button = |pressed| Event::PointerButton {
            pos: c,
            button: PointerButton::Primary,
            pressed,
            modifiers: eframe::egui::Modifiers::NONE,
        };
        let mut frame = |events: Vec<Event>| {
            let mut raw = eframe::egui::RawInput::default();
            raw.screen_rect = Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 600.0)));
            raw.events = events;
            let _ = crate::test_support::run_frame(&ctx, raw, |ui| {
                show(
                    ui,
                    &mut st,
                    &list,
                    None,
                    1_000,
                    AdsbSettings::default(),
                    None,
                    sdroxide_types::MapStyle::Radar,
                    400.0,
                    Vfo::A,
                    Some(&card),
                );
            });
        };
        frame(vec![Event::PointerMoved(c)]);
        for _ in 0..3 {
            frame(vec![Event::PointerMoved(c)]);
        }
        let before = cards.get();
        frame(vec![Event::PointerMoved(c), button(true), button(false)]);
        assert_eq!(cards.get(), before, "the frame that pins it draws no tooltip for it");
    }

    #[test]
    fn two_maps_never_draw_the_same_pinned_card_twice() {
        use eframe::egui::Id;
        let (a, b) = (Id::new("window"), Id::new("panel"));
        // The window draws first and is clicked: it owns the card.
        let mut o = pin_owner_after(None, a, 10, true);
        // Both draw on the next frames; the panel does not take it.
        o = pin_owner_after(o, a, 11, false);
        o = pin_owner_after(o, b, 11, false);
        assert!(owns_pin(o, a) && !owns_pin(o, b));
        // A click in the panel hands it over.
        o = pin_owner_after(o, b, 12, true);
        assert!(owns_pin(o, b) && !owns_pin(o, a));
        // The owner's window closes: the other takes over after a few frames.
        let mut o = Some((a, 20));
        for f in 21..26 {
            o = pin_owner_after(o, b, f, false);
        }
        assert!(owns_pin(o, b), "the owner stopped drawing, so the card is not lost");
    }

    #[test]
    fn no_search_leaves_the_map_as_it_was() {
        let list = [at(1, 52.0, 5.0), at(2, 1.3, 103.8)];
        let mut a = AdsbMapState::default();
        let mut b = AdsbMapState::default();
        b.visible = Some(std::collections::HashSet::from([1, 2]));
        map_frame(&eframe::egui::Context::default(), &mut a, &list, vec![]);
        map_frame(&eframe::egui::Context::default(), &mut b, &list, vec![]);
        assert_eq!((a.view.clat, a.view.clon), (b.view.clat, b.view.clon));
    }

    // ── the trail of the aircraft whose history is on ─────────────────────

    /// How many shapes one frame of the map paints.
    fn shapes_in_a_frame(state: &mut AdsbMapState, list: &[AdsbAircraft]) -> usize {
        let mut raw = eframe::egui::RawInput::default();
        raw.screen_rect = Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 600.0)));
        let out = crate::test_support::run_frame(&eframe::egui::Context::default(), raw, |ui| {
            show(
                ui,
                state,
                list,
                None,
                1_000,
                AdsbSettings::default(),
                None,
                sdroxide_types::MapStyle::Radar,
                400.0,
                Vfo::A,
                None,
            );
        });
        out.shapes.len()
    }

    fn seg(from: (f64, f64), to: (f64, f64), alpha: f32) -> TrailSeg {
        TrailSeg { from, to, alpha }
    }

    #[test]
    fn the_trail_is_painted_under_the_aircraft_one_line_a_stretch() {
        let list = [at(1, 52.0, 5.0)];
        let mut bare = AdsbMapState::default();
        let without = shapes_in_a_frame(&mut bare, &list);
        let mut with = AdsbMapState::default();
        with.trail = vec![
            seg((52.0, 5.0), (52.01, 5.0), 0.3),
            seg((52.01, 5.0), (52.02, 5.0), 0.6),
            seg((52.02, 5.0), (52.03, 5.0), 0.9),
        ];
        let painted = shapes_in_a_frame(&mut with, &list);
        assert!(painted >= without + 3, "{painted} shapes with a trail, {without} without");
        // The control: an empty trail paints what no trail paints.
        let mut empty = AdsbMapState::default();
        assert_eq!(shapes_in_a_frame(&mut empty, &list), without);
    }

    #[test]
    fn a_stretch_of_trail_off_the_map_is_not_painted() {
        let list = [at(1, 52.0, 5.0)];
        let mut bare = AdsbMapState::default();
        let without = shapes_in_a_frame(&mut bare, &list);
        let mut far = AdsbMapState::default();
        far.trail = vec![seg((-40.0, 150.0), (-41.0, 151.0), 0.9)];
        assert_eq!(shapes_in_a_frame(&mut far, &list), without, "wholly out of view");
        let mut near = AdsbMapState::default();
        near.trail = vec![seg((52.0, 5.0), (52.01, 5.0), 0.9)];
        assert!(shapes_in_a_frame(&mut near, &list) > without, "the control: in view it is");
    }
}
