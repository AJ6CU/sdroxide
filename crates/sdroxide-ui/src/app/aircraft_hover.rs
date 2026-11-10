//! The compact card shown when the pointer rests on an aircraft on the map.
//!
//! It reads what the AIRCRAFT window already holds — the registry's merged card
//! and, when the open card has already asked the station's relay, the answer —
//! and **asks for nothing**. A hover is a pointer passing over a map: it must
//! not cost a request, a download or a frame, so the online lines and the
//! picture are there only when the card has fetched them before. An aircraft
//! never opened in the card simply has no online lines, and is no less usable
//! for it.
//!
//! The radio's lines come first, in the card's own order; the online ones
//! follow and never replace them. Every value is clipped and the number of
//! lines is capped, so the card cannot grow with what a registry says.

// ==== IMPL ====
use crate::i18n::tr;
use eframe::egui::{self, RichText};
use sdroxide_types::AdsbAircraft;

use super::aircraft::{badge_chip, provenance_chip};
use super::aircraft_card::{self, CardKey, icao_hex, source_label};
use super::aircraft_history::HistoryOffer;
use super::aircraft_online::{self, Badge, OnlineState, Provenance, Reason, Row, ThumbStatus};
use super::aircraft_online_io::OnlineClient;
use super::aircraft_registry::{AircraftCard, AircraftRegistry, Source};

/// Width of the card, points.
pub(in crate::app) const HOVER_W: f32 = 440.0;
/// Tallest the card may be, points. A picture is the last thing to fit in.
pub(in crate::app) const HOVER_MAX_H: f32 = 420.0;
/// Longest value or note shown, characters. The rest is an ellipsis.
pub(in crate::app) const MAX_CHARS: usize = 28;
/// Most lines the card carries.
pub(in crate::app) const MAX_LINES: usize = 14;
/// Tallest the picture is drawn, points.
const PHOTO_H: f32 = 106.0;
/// Column widths of a line, points: label, badge, value. The note takes the rest.
const COL_LABEL: f32 = 62.0;
const COL_BADGE: f32 = 44.0;
const COL_VALUE: f32 = 180.0;
const LINE_H: f32 = 14.0;

/// The lines only the online half can add, in the order they follow the radio's.
const ONLINE_LABELS: [&str; 4] = ["Type", "Operator", "Origin", "Destination"];

/// The photo block: the credit and the link always, the picture only when the
/// open card has already loaded it.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) struct HoverPhoto {
    /// Set only when the thumbnail is already in memory.
    pub thumb_url: Option<String>,
    pub credit: String,
    pub page_url: Option<String>,
}

/// Everything the card says, as plain values.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) struct HoverCard {
    pub title: String,
    pub provenance: Provenance,
    pub lines: Vec<Row>,
    /// The note most lines shared ("ADS-B, 4s ago"), said once in the header
    /// and cleared from the lines that carried it. A line whose own note
    /// differs (an older position, a figure from the history) keeps it.
    pub common_note: Option<String>,
    pub photo: Option<HoverPhoto>,
    /// One grey line, only when the operator could do something about the
    /// missing online half.
    pub footer: Option<String>,
}

/// Lift the note that two or more lines share into one, and clear it from
/// them: the same "ADS-B, 4s ago" down every row said nothing after the first.
pub(in crate::app) fn lift_common_note(lines: &mut [Row]) -> Option<String> {
    let mut best: Option<(&str, usize)> = None;
    for r in lines.iter().filter(|r| !r.note.is_empty()) {
        let n = lines.iter().filter(|o| o.note == r.note).count();
        if best.is_none_or(|(_, b)| n > b) {
            best = Some((&r.note, n));
        }
    }
    let common = best.filter(|&(_, n)| n >= 2)?.0.to_string();
    for r in lines.iter_mut().filter(|r| r.note == common) {
        r.note.clear();
    }
    Some(common)
}

/// `s` cut to `max` characters, with an ellipsis where it was cut.
pub(in crate::app) fn clip(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    if max == 0 {
        return String::new();
    }
    let mut out: String = s.chars().take(max - 1).collect();
    out.push('…');
    out
}

/// The picture block for this state, from what is already in memory.
pub(in crate::app) fn hover_photo(state: &OnlineState, thumb: ThumbStatus) -> Option<HoverPhoto> {
    let p = aircraft_online::photo_view(state, thumb)?;
    Some(HoverPhoto {
        thumb_url: (thumb == ThumbStatus::Ready && p.show_image).then(|| p.thumb_url.clone()),
        credit: clip(&p.credit, 2 * MAX_CHARS),
        page_url: p.page_url,
    })
}

/// The one-line note under the card for an online half that is not there. Only
/// the reasons the operator can do something about; a failure that mends itself
/// is not said.
pub(in crate::app) fn footer(state: &OnlineState) -> Option<String> {
    match state {
        OnlineState::Unavailable(Reason::Disabled) => Some(tr!("aircraft-footer-disabled")),
        OnlineState::Unavailable(Reason::NoStation) => Some(tr!("aircraft-footer-no-station")),
        OnlineState::Unavailable(Reason::Unauthorized) => Some(tr!("aircraft-footer-unauthorized")),
        _ => None,
    }
}

fn extra(label: &'static str, value: String, note: String) -> Row {
    Row { label, value, badge: Badge::Radio, note }
}

/// The card for the aircraft under the pointer.
///
/// `registry` is the merged card when the registry has one; a target the map
/// has seen a frame before the registry gets a card from its own fields.
pub(in crate::app) fn hover_card(
    live: &AdsbAircraft,
    registry: Option<&AircraftCard>,
    state: &OnlineState,
    thumb: ThumbStatus,
    home: Option<(f64, f64)>,
    now: i64,
) -> HoverCard {
    let icao = live.icao & 0xff_ffff;
    let own;
    let card = match registry {
        Some(c) => c,
        None => {
            let mut reg = AircraftRegistry::default();
            reg.ingest_adsb(live);
            own = reg.card(icao).cloned().unwrap_or_default();
            &own
        }
    };
    let age = tr!(
        "aircraft-provenance",
        source = source_label(Source::Adsb),
        age = super::util::fmt_age(now - live.last_at)
    );

    let all: Vec<Row> = aircraft_online::rows(card, state, now)
        .into_iter()
        .filter(|r| r.label != "Last message" && r.label != "Online")
        .collect();
    let split = all.iter().position(|r| ONLINE_LABELS.contains(&r.label)).unwrap_or(all.len());
    let (radio, online) = all.split_at(split);

    let mut lines = vec![extra("ICAO", icao_hex(icao), age.clone())];
    lines.extend(radio.iter().cloned());
    if let (Some(h), Some((lat, lon))) = (home, live.lat.zip(live.lon)) {
        let km = sdroxide_types::distance_km(h, (lat, lon));
        let bear = sdroxide_types::bearing_deg(h, (lat, lon));
        let range = format!("{}  {bear:.0}°", sdroxide_types::AirUnits::current().distance(km));
        lines.push(extra("Range", range, tr!("aircraft-from-your-station")));
    }
    if let Some(vr) = live.vertical_rate_fpm.filter(|v| v.abs() > 100) {
        lines.push(extra(
            "Vertical",
            sdroxide_types::AirUnits::current().vertical_rate(vr),
            age.clone(),
        ));
    }
    if let Some(e) = &live.emergency {
        lines.push(extra("Emergency", e.clone(), age));
    }
    lines.extend(online.iter().cloned());
    lines.truncate(MAX_LINES);
    for r in &mut lines {
        r.value = clip(&r.value, MAX_CHARS);
        r.note = clip(&r.note, MAX_CHARS);
    }

    let common_note = lift_common_note(&mut lines);
    HoverCard {
        common_note,
        title: clip(&aircraft_card::title(&CardKey::Icao(icao), card), MAX_CHARS),
        provenance: aircraft_online::provenance(card, state),
        lines,
        photo: hover_photo(state, thumb),
        footer: footer(state),
    }
}

/// One fixed-width cell of a line. Truncated, never wrapped: the card keeps its
/// width whatever a registry writes, and a long value cannot print over the
/// note beside it. Returns the area the text took.
fn cell(ui: &mut egui::Ui, w: f32, text: RichText) -> egui::Rect {
    ui.allocate_ui_with_layout(
        egui::vec2(w, LINE_H),
        egui::Layout::left_to_right(egui::Align::Center),
        |ui| ui.add(egui::Label::new(text).truncate()).rect,
    )
    .inner
}

/// Draw the card. Returns the area it took. `texture` is the thumbnail when the
/// open card has loaded it; it is never fetched from here.
pub(in crate::app) fn draw(
    ui: &mut egui::Ui,
    card: &HoverCard,
    texture: Option<&egui::TextureHandle>,
) -> egui::Rect {
    draw_inner(ui, card, texture, None, LookupOffer::None, HistoryOffer::Hidden)
}

/// [`draw`] for the tooltip, or for the pinned card when `pin` is given: there
/// the link can be followed, the close button is drawn and the lookup is offered.
fn draw_inner(
    ui: &mut egui::Ui,
    card: &HoverCard,
    texture: Option<&egui::TextureHandle>,
    mut pin: Option<&mut PinnedOut>,
    offer: LookupOffer,
    history: HistoryOffer,
) -> egui::Rect {
    let gray = crate::theme::gray(160);
    ui.scope(|ui| {
        ui.set_max_width(HOVER_W);
        ui.set_max_height(HOVER_MAX_H);
        ui.spacing_mut().item_spacing = egui::vec2(4.0, 2.0);
        ui.horizontal(|ui| {
            ui.label(RichText::new(&card.title).strong().size(13.0));
            provenance_chip(ui, card.provenance);
            if let Some(n) = &card.common_note {
                ui.label(RichText::new(n).size(9.5).color(crate::theme::gray(130)));
            }
            if let Some(out) = pin.as_deref_mut() {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let r = ui.small_button("✕").on_hover_text(tr!("aircraft-hover-close"));
                    if r.clicked() {
                        out.actions.push(PinAction::Close);
                    }
                    out.close = Some(r.rect);
                });
            }
        });
        for r in &card.lines {
            ui.horizontal(|ui| {
                cell(ui, COL_LABEL, RichText::new(aircraft_card::field_label(r.label)).size(10.5).color(gray));
                ui.allocate_ui_with_layout(
                    egui::vec2(COL_BADGE, LINE_H),
                    egui::Layout::left_to_right(egui::Align::Center),
                    |ui| badge_chip(ui, r.badge),
                );
                let value = match r.badge {
                    Badge::Missing => RichText::new(&r.value).size(11.0).weak(),
                    _ if r.label == "Emergency" => {
                        RichText::new(&r.value).size(11.0).color(crate::theme::YELLOW())
                    }
                    _ => RichText::new(&r.value).size(11.0),
                };
                cell(ui, COL_VALUE, value);
                cell(
                    ui,
                    HOVER_W - COL_LABEL - COL_BADGE - COL_VALUE - 12.0,
                    RichText::new(&r.note).size(9.5).color(crate::theme::gray(130)),
                );
            });
        }
        if let Some(p) = &card.photo {
            ui.add_space(4.0);
            if let Some(tex) = p.thumb_url.as_ref().and(texture) {
                ui.add(egui::Image::new(tex).max_height(PHOTO_H).corner_radius(3.0));
            }
            ui.horizontal(|ui| {
                ui.add(
                    egui::Label::new(
                        RichText::new(&p.credit).size(10.0).color(crate::theme::gray(150)),
                    )
                    .truncate(),
                );
                // A tooltip goes with the pointer, so the link cannot be
                // followed from there: it is named, and clickable in the
                // pinned card.
                match (&p.page_url, pin.as_deref_mut()) {
                    (Some(url), Some(out)) => {
                        let r = ui.link(RichText::new("Planespotters").size(10.0));
                        if r.clicked() {
                            out.actions.push(PinAction::OpenLink(url.clone()));
                        }
                        out.link = Some(r.rect);
                    }
                    (Some(_), None) => {
                        ui.label(
                            RichText::new("Planespotters")
                                .size(10.0)
                                .color(ui.visuals().hyperlink_color),
                        );
                    }
                    (None, _) => {}
                }
            });
        }
        if let Some(f) = &card.footer {
            ui.label(RichText::new(f).size(10.0).color(crate::theme::gray(130)));
        }
        if let Some(out) = pin.as_deref_mut() {
            match offer {
                LookupOffer::Button => {
                    ui.add_space(4.0);
                    let r = ui.small_button(tr!("aircraft-hover-lookup"))
                        .on_hover_text(tr!("aircraft-hover-lookup-hint"));
                    if r.clicked() {
                        out.actions.push(PinAction::Lookup);
                    }
                    out.lookup = Some(r.rect);
                }
                LookupOffer::Retry(secs) => {
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(tr!("aircraft-hover-lookup-retry", s = secs))
                            .size(10.0)
                            .weak(),
                    );
                    let r = ui.small_button(tr!("aircraft-hover-lookup"))
                        .on_hover_text(tr!("aircraft-hover-lookup-hint"));
                    if r.clicked() {
                        out.actions.push(PinAction::Lookup);
                    }
                    out.lookup = Some(r.rect);
                }
                LookupOffer::Asking => {
                    ui.add_space(4.0);
                    ui.label(RichText::new(tr!("aircraft-asking-station-dots")).size(10.0).weak());
                }
                LookupOffer::Waiting => {
                    ui.add_space(4.0);
                    ui.label(
                        RichText::new(tr!("aircraft-hover-lookup-waiting"))
                            .size(10.0)
                            .weak(),
                    );
                }
                LookupOffer::None => {}
            }
            if history != HistoryOffer::Hidden {
                ui.add_space(4.0);
                let r = ui.small_button(history.label()).on_hover_text(tr!("aircraft-hover-history-hint"));
                if r.clicked() {
                    out.actions.push(PinAction::History);
                }
                out.history = Some(r.rect);
            }
        }
    })
    .response
    .rect
}

// ── the pinned card ───────────────────────────────────────────────────────

/// What the pinned card offers about the online half.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::app) enum LookupOffer {
    /// Nothing to press: there is no one to ask, or it has been asked already.
    None,
    /// "Look up online" can be pressed.
    Button,
    /// The question for this aircraft is out.
    Asking,
    /// Another aircraft's question is out; one at a time.
    Waiting,
    /// The question failed and will be asked again by itself in this many
    /// seconds; the button asks now.
    Retry(i64),
}

impl LookupOffer {
    /// Whether a press on the button is taken: the first question, or a retry.
    pub(in crate::app) fn pressable(self) -> bool {
        matches!(self, LookupOffer::Button | LookupOffer::Retry(_))
    }
}

/// Whether the pinned card offers the lookup: for an aircraft nobody has asked
/// about, on a station that will answer, and again after a failure that mends
/// itself (`retry_in` is the seconds left of its back-off). Once asked, the
/// card follows the question on its own; the button only hurries a retry.
pub(in crate::app) fn lookup_offer(
    state: &OnlineState,
    has_station: bool,
    line_busy: bool,
    retry_in: Option<i64>,
) -> LookupOffer {
    match state {
        OnlineState::Idle if has_station && line_busy => LookupOffer::Waiting,
        OnlineState::Idle if has_station => LookupOffer::Button,
        OnlineState::Loading => LookupOffer::Asking,
        OnlineState::Unavailable(_) if has_station => {
            retry_in.map_or(LookupOffer::None, LookupOffer::Retry)
        }
        _ => LookupOffer::None,
    }
}

/// What the pinned card asks of its owner when it is pressed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::app) enum PinAction {
    Close,
    OpenLink(String),
    Lookup,
    /// Show or hide where this aircraft has been, from the station's record.
    History,
}

/// What drawing the pinned card produced: what was pressed, and where the
/// things that can be pressed are.
#[derive(Debug, Clone, Default)]
pub(in crate::app) struct PinnedOut {
    pub area: Option<egui::Rect>,
    pub actions: Vec<PinAction>,
    pub link: Option<egui::Rect>,
    pub close: Option<egui::Rect>,
    pub lookup: Option<egui::Rect>,
    pub history: Option<egui::Rect>,
}

/// The card for an aircraft, from what the app already holds: the registry, the
/// station's answer, the thumbnail if it is in memory. The hover and the pinned
/// card both come from here, so they cannot disagree. It only reads.
pub(in crate::app) fn card_view<'a>(
    live: &AdsbAircraft,
    registry: &AircraftRegistry,
    online: &'a OnlineClient,
    has_station: bool,
    home: Option<(f64, f64)>,
    now: i64,
) -> (HoverCard, Option<&'a egui::TextureHandle>) {
    let icao = live.icao & 0xff_ffff;
    let state = online.store.state_for(icao, has_station);
    let thumb_url = match state {
        OnlineState::Ready(info) => info.photo.as_ref().map(|p| p.thumb_url.as_str()),
        _ => None,
    };
    let thumb = thumb_url.map_or(ThumbStatus::NotTried, |u| online.thumb_status(u));
    let card = hover_card(live, registry.card(icao), state, thumb, home, now);
    let tex = card
        .photo
        .as_ref()
        .and_then(|p| p.thumb_url.as_deref())
        .and_then(|u| online.thumb_texture(u));
    (card, tex)
}

/// Draw the pinned card: the same lines as the hover, with the link that can be
/// followed, a close button and, where offered, the lookup button.
pub(in crate::app) fn draw_pinned(
    ui: &mut egui::Ui,
    card: &HoverCard,
    texture: Option<&egui::TextureHandle>,
    offer: LookupOffer,
    history: HistoryOffer,
) -> PinnedOut {
    let mut out = PinnedOut::default();
    out.area = Some(draw_inner(ui, card, texture, Some(&mut out), offer, history));
    out
}

#[cfg(test)]
mod tests {
    use super::super::aircraft_registry::{Source, Stamped};
    use super::*;
    use sdroxide_aircraft_online::{AircraftInfo, Airport, Info, Photo, Route, Status};

    const NOW: i64 = 1_000_000;
    const HOME: (f64, f64) = (52.0, 5.0);

    fn stamped<T>(value: T, source: Source, age: i64) -> Option<Stamped<T>> {
        Some(Stamped { value, source, at: NOW - age })
    }

    fn live() -> AdsbAircraft {
        let mut a = AdsbAircraft::new(0x4840D6, NOW - 12);
        a.callsign = "KLM1234".into();
        a.lat = Some(52.3);
        a.lon = Some(4.8);
        a.altitude_ft = Some(35_000);
        a.ground_speed_kt = Some(450.0);
        a.track_deg = Some(90.0);
        a.vertical_rate_fpm = Some(1800);
        a.last_at = NOW - 12;
        a.last_pos_at = NOW - 12;
        a
    }

    fn heard() -> AircraftCard {
        AircraftCard {
            flight: stamped("KLM1234".to_string(), Source::Adsb, 12),
            registration: stamped("PH-BXA".to_string(), Source::Vdl2, 40),
            position: stamped((52.3, 4.8), Source::Adsb, 12),
            altitude_ft: stamped(35_000, Source::Adsb, 12),
            ground_speed_kt: stamped(450.0, Source::Adsb, 12),
            track_deg: stamped(90.0, Source::Adsb, 12),
            last_seen: NOW - 12,
            ..AircraftCard::default()
        }
    }

    fn info() -> Info {
        let mut i = Info::bare(Status::Ok);
        i.aircraft = Some(AircraftInfo {
            type_name: Some("737-800".into()),
            manufacturer: Some("Boeing".into()),
            operator: Some("KLM".into()),
            registration: Some("PH-ONLINE".into()),
        });
        i.route = Some(Route {
            origin: Some(Airport { code: "EHAM".into(), name: "Amsterdam Schiphol".into() }),
            destination: Some(Airport { code: "EGLL".into(), name: "London Heathrow".into() }),
        });
        i.photo = Some(Photo {
            thumb_url: "https://t.plnspttrs.net/12345/abc_t.jpg".into(),
            page_url: "https://www.planespotters.net/photo/12345/x".into(),
            photographer: "A. Spotter".into(),
        });
        i
    }

    fn ready(i: Info) -> OnlineState {
        OnlineState::Ready(Box::new(i))
    }

    fn card_with(
        registry: Option<&AircraftCard>,
        state: &OnlineState,
        thumb: ThumbStatus,
    ) -> HoverCard {
        hover_card(&live(), registry, state, thumb, Some(HOME), NOW)
    }

    fn labels(c: &HoverCard) -> Vec<&'static str> {
        c.lines.iter().map(|r| r.label).collect()
    }

    fn row<'a>(c: &'a HoverCard, label: &str) -> &'a Row {
        c.lines.iter().find(|r| r.label == label).unwrap_or_else(|| panic!("no {label} line"))
    }

    // ── what the card says ────────────────────────────────────────────────

    #[test]
    fn the_radio_lines_come_first_and_the_online_ones_after_in_a_fixed_order() {
        let c = card_with(Some(&heard()), &ready(info()), ThumbStatus::NotTried);
        assert_eq!(
            labels(&c),
            [
                "ICAO",
                "Flight",
                "Registration",
                "Position",
                "Altitude",
                "Speed",
                "Track",
                "Range",
                "Vertical",
                "Type",
                "Operator",
                "Origin",
                "Destination"
            ]
        );
    }

    #[test]
    fn what_the_radio_heard_wears_the_green_badge_with_its_source_and_age() {
        let c = card_with(Some(&heard()), &ready(info()), ThumbStatus::NotTried);
        for l in ["ICAO", "Flight", "Registration", "Position", "Altitude", "Speed", "Track"] {
            assert_eq!(row(&c, l).badge, Badge::Radio, "{l}");
        }
        assert_eq!(row(&c, "ICAO").value, "4840D6");
        assert_eq!(row(&c, "Altitude").value, "35000 ft");
        // The ADS-B source and age are said once, in the header, not per line.
        let common = c.common_note.as_deref().unwrap_or_default();
        assert!(common.starts_with("ADS-B") && common.contains("12"), "{common}");
        assert_eq!(row(&c, "Altitude").note, "");
        assert!(row(&c, "Registration").note.starts_with("VDL2"), "{:?}", row(&c, "Registration"));
    }

    #[test]
    fn online_fields_appear_only_when_the_state_is_ready() {
        for st in [
            OnlineState::Idle,
            OnlineState::Loading,
            OnlineState::Unavailable(Reason::Offline),
            OnlineState::Unavailable(Reason::Disabled),
            OnlineState::Unavailable(Reason::NoStation),
        ] {
            let c = card_with(Some(&heard()), &st, ThumbStatus::NotTried);
            assert!(c.lines.iter().all(|r| r.badge != Badge::Online), "{st:?}");
            for gone in ["Type", "Operator", "Origin", "Destination", "Online"] {
                assert!(c.lines.iter().all(|r| r.label != gone), "{st:?} {gone}");
            }
        }
    }

    #[test]
    fn online_fields_wear_the_blue_badge_and_an_absent_one_adds_no_line() {
        let c = card_with(Some(&heard()), &ready(info()), ThumbStatus::NotTried);
        assert_eq!(row(&c, "Type").value, "Boeing 737-800");
        for l in ["Type", "Operator", "Origin", "Destination"] {
            assert_eq!(row(&c, l).badge, Badge::Online, "{l}");
        }
        let mut i = Info::bare(Status::Ok);
        i.aircraft = Some(AircraftInfo { type_name: Some("A320".into()), ..Default::default() });
        let c = card_with(Some(&heard()), &ready(i), ThumbStatus::NotTried);
        assert_eq!(row(&c, "Type").value, "A320");
        for gone in ["Operator", "Origin", "Destination"] {
            assert!(c.lines.iter().all(|r| r.label != gone), "{gone}");
        }
    }

    #[test]
    fn the_global_badge_says_radio_or_radio_plus_online() {
        let radio = card_with(Some(&heard()), &OnlineState::Idle, ThumbStatus::NotTried);
        assert_eq!(radio.provenance, Provenance::Radio);
        assert_eq!(radio.provenance.label(), "radio");
        let both = card_with(Some(&heard()), &ready(info()), ThumbStatus::NotTried);
        assert_eq!(both.provenance, Provenance::RadioOnline);
        assert_eq!(both.provenance.label(), "radio + online");
    }

    #[test]
    fn a_missing_radio_field_is_a_grey_dash_with_its_reason() {
        let mut card = heard();
        card.registration = None;
        let c = card_with(Some(&card), &OnlineState::Idle, ThumbStatus::NotTried);
        let r = row(&c, "Registration");
        assert_eq!((r.value.as_str(), r.badge), ("—", Badge::Missing));
        assert!(r.note.starts_with("not heard yet"), "{:?}", r.note);
    }

    #[test]
    fn the_registration_the_radio_heard_wins_and_an_online_one_fills_a_gap() {
        let c = card_with(Some(&heard()), &ready(info()), ThumbStatus::NotTried);
        let r = row(&c, "Registration");
        assert_eq!((r.value.as_str(), r.badge), ("PH-BXA", Badge::Radio));
        let mut card = heard();
        card.registration = None;
        let c = card_with(Some(&card), &ready(info()), ThumbStatus::NotTried);
        let r = row(&c, "Registration");
        assert_eq!((r.value.as_str(), r.badge), ("PH-ONLINE", Badge::Online));
        assert_eq!(c.lines.iter().filter(|r| r.label == "Registration").count(), 1);
    }

    #[test]
    fn an_aircraft_the_registry_has_not_got_yet_still_gets_a_usable_card() {
        let c = card_with(None, &OnlineState::Idle, ThumbStatus::NotTried);
        assert_eq!(c.title, "KLM1234");
        assert_eq!(row(&c, "ICAO").value, "4840D6");
        assert_eq!(row(&c, "Altitude").value, "35000 ft");
        assert_eq!(row(&c, "Speed").value, "450 kt");
        assert_eq!(c.provenance, Provenance::Radio);
    }

    #[test]
    fn a_card_without_online_data_reads_as_the_radio_card_does() {
        let radio = card_with(Some(&heard()), &OnlineState::Idle, ThumbStatus::NotTried);
        for st in [
            OnlineState::Loading,
            OnlineState::Unavailable(Reason::Offline),
            OnlineState::Unavailable(Reason::Busy),
            OnlineState::Unavailable(Reason::Unknown),
        ] {
            let c = card_with(Some(&heard()), &st, ThumbStatus::NotTried);
            assert_eq!(c.lines, radio.lines, "{st:?}");
            assert_eq!(c.footer, None, "{st:?}");
        }
        assert!(!radio.lines.is_empty());
    }

    #[test]
    fn a_note_every_line_repeats_is_said_once_in_the_header() {
        let c = hover_card(
            &live(),
            Some(&heard()),
            &OnlineState::Idle,
            ThumbStatus::NotTried,
            Some(HOME),
            NOW,
        );
        let common = c.common_note.clone().expect("lines share the age note");
        assert!(common.starts_with("ADS-B"), "{common}");
        assert!(c.lines.iter().all(|r| r.note != common), "still repeated: {:?}", c.lines);
        // A line with its own note keeps it.
        assert_eq!(row(&c, "Range").note, "from your station");
    }

    #[test]
    fn two_lines_with_different_notes_share_nothing() {
        let mut rows = vec![extra("A", "1".into(), "x".into()), extra("B", "2".into(), "y".into())];
        assert_eq!(lift_common_note(&mut rows), None);
        assert_eq!(rows[0].note, "x");
    }

    #[test]
    fn the_card_writes_range_and_climb_in_the_chosen_units() {
        let km = sdroxide_types::distance_km(HOME, (52.3, 4.8));
        let bear = sdroxide_types::bearing_deg(HOME, (52.3, 4.8));
        let card = || {
            hover_card(
                &live(),
                Some(&heard()),
                &OnlineState::Idle,
                ThumbStatus::NotTried,
                Some(HOME),
                NOW,
            )
        };
        sdroxide_types::AirUnits::Metric.set_current();
        let c = card();
        assert_eq!(row(&c, "Range").value, format!("{km:.0} km  {bear:.0}°"));
        assert_eq!(row(&c, "Vertical").value, "+9.1 m/s");
        sdroxide_types::AirUnits::Imperial.set_current();
        let c = card();
        assert_eq!(row(&c, "Range").value, format!("{:.0} mi  {bear:.0}°", km * 0.621_371));
        assert_eq!(row(&c, "Vertical").value, "+1800 ft/min");
        sdroxide_types::AirUnits::Aviation.set_current();
    }

    #[test]
    fn the_distance_the_bearing_and_the_emergency_of_the_old_tooltip_are_kept() {
        let mut a = live();
        a.emergency = Some("general emergency".into());
        let c = hover_card(
            &a,
            Some(&heard()),
            &OnlineState::Idle,
            ThumbStatus::NotTried,
            Some(HOME),
            NOW,
        );
        let km = sdroxide_types::distance_km(HOME, (52.3, 4.8));
        let bear = sdroxide_types::bearing_deg(HOME, (52.3, 4.8));
        assert_eq!(row(&c, "Range").value, format!("{km:.0} km  {bear:.0}°"));
        assert_eq!(row(&c, "Range").badge, Badge::Radio);
        assert_eq!(row(&c, "Emergency").value, "general emergency");
        assert_eq!(row(&c, "Vertical").value, "+1800 ft/min");
        let none = hover_card(
            &live(),
            Some(&heard()),
            &OnlineState::Idle,
            ThumbStatus::NotTried,
            None,
            NOW,
        );
        assert!(none.lines.iter().all(|r| r.label != "Range" && r.label != "Emergency"));
    }

    // ── the picture ───────────────────────────────────────────────────────

    #[test]
    fn the_picture_is_drawn_only_when_it_is_already_loaded() {
        let st = ready(info());
        let url = "https://t.plnspttrs.net/12345/abc_t.jpg";
        let loaded = hover_photo(&st, ThumbStatus::Ready).expect("photo");
        assert_eq!(loaded.thumb_url.as_deref(), Some(url));
        for t in [ThumbStatus::NotTried, ThumbStatus::Loading, ThumbStatus::Failed] {
            let p = hover_photo(&st, t).expect("credit and link stay");
            assert_eq!(p.thumb_url, None, "{t:?}");
            assert!(p.credit.contains("A. Spotter"), "{t:?}");
            assert!(p.page_url.is_some(), "{t:?}");
        }
        assert_eq!(hover_photo(&OnlineState::Idle, ThumbStatus::Ready), None);
        assert_eq!(hover_photo(&OnlineState::Loading, ThumbStatus::Ready), None);
    }

    #[test]
    fn the_credit_and_the_page_link_are_kept_and_only_planespotters_pages_are_linked() {
        let p = hover_photo(&ready(info()), ThumbStatus::NotTried).unwrap();
        assert_eq!(p.credit, "Photo: A. Spotter / Planespotters");
        assert_eq!(p.page_url.as_deref(), Some("https://www.planespotters.net/photo/12345/x"));
        let mut i = info();
        i.photo.as_mut().unwrap().page_url = "https://evil.example/x".into();
        let p = hover_photo(&ready(i), ThumbStatus::NotTried).unwrap();
        assert_eq!(p.page_url, None);
        let mut i = info();
        i.photo.as_mut().unwrap().thumb_url = "https://evil.example/t.jpg".into();
        let p = hover_photo(&ready(i), ThumbStatus::Ready).unwrap();
        assert_eq!(p.thumb_url, None, "a host Planespotters does not use is never drawn");
    }

    #[test]
    fn the_card_carries_the_photo_block_of_its_state() {
        let c = card_with(Some(&heard()), &ready(info()), ThumbStatus::Ready);
        assert!(c.photo.as_ref().is_some_and(|p| p.thumb_url.is_some()));
        let c = card_with(Some(&heard()), &OnlineState::Idle, ThumbStatus::NotTried);
        assert_eq!(c.photo, None);
    }

    // ── what is said when the online half is off ─────────────────────────

    #[test]
    fn only_a_switched_off_relay_or_a_missing_station_says_so_in_the_footer() {
        for r in [Reason::Disabled, Reason::NoStation, Reason::Unauthorized] {
            let c = card_with(Some(&heard()), &OnlineState::Unavailable(r), ThumbStatus::NotTried);
            assert!(c.footer.as_ref().is_some_and(|f| f.contains("online unavailable")), "{r:?}");
            assert!(c.footer.as_ref().is_some_and(|f| f.chars().count() <= 60), "{r:?}");
        }
        for st in [
            OnlineState::Idle,
            OnlineState::Loading,
            ready(info()),
            OnlineState::Unavailable(Reason::Offline),
            OnlineState::Unavailable(Reason::Busy),
            OnlineState::Unavailable(Reason::Unknown),
        ] {
            let c = card_with(Some(&heard()), &st, ThumbStatus::NotTried);
            assert_eq!(c.footer, None, "{st:?}");
        }
    }

    // ── the card stays small ──────────────────────────────────────────────

    #[test]
    fn clipping_keeps_short_text_and_ends_a_long_one_with_an_ellipsis() {
        assert_eq!(clip("abc", 5), "abc");
        assert_eq!(clip("abcde", 5), "abcde");
        let s = clip("abcdefgh", 5);
        assert_eq!(s, "abcd…");
        assert_eq!(s.chars().count(), 5);
        assert_eq!(clip("éééééé", 3), "éé…");
        assert_eq!(clip("", 3), "");
    }

    #[test]
    fn however_long_the_registry_writes_the_card_is_bounded() {
        let long = "X".repeat(400);
        let mut i = info();
        i.aircraft = Some(AircraftInfo {
            type_name: Some(long.clone()),
            manufacturer: Some(long.clone()),
            operator: Some(long.clone()),
            registration: None,
        });
        i.route = Some(Route {
            origin: Some(Airport { code: long.clone(), name: long.clone() }),
            destination: Some(Airport { code: long.clone(), name: long.clone() }),
        });
        i.photo.as_mut().unwrap().photographer = long.clone();
        let mut card = heard();
        card.flight = stamped(long.clone(), Source::Adsb, 1);
        let c = card_with(Some(&card), &ready(i), ThumbStatus::NotTried);
        assert!(c.lines.len() <= MAX_LINES);
        for r in &c.lines {
            assert!(r.value.chars().count() <= MAX_CHARS, "{}: {}", r.label, r.value);
            assert!(r.note.chars().count() <= MAX_CHARS, "{}: {}", r.label, r.note);
        }
        assert!(c.title.chars().count() <= MAX_CHARS, "{}", c.title);
        let p = c.photo.expect("photo");
        assert!(p.credit.chars().count() <= 2 * MAX_CHARS, "{}", p.credit);
    }

    #[test]
    fn the_drawn_card_never_outgrows_its_width_or_its_height() {
        let long = "X".repeat(400);
        let mut i = info();
        i.aircraft = Some(AircraftInfo {
            type_name: Some(long.clone()),
            operator: Some(long.clone()),
            ..Default::default()
        });
        i.photo.as_mut().unwrap().photographer = long;
        for st in [ready(i), OnlineState::Idle, OnlineState::Unavailable(Reason::NoStation)] {
            let c = card_with(Some(&heard()), &st, ThumbStatus::Ready);
            let ctx = egui::Context::default();
            let mut rect = egui::Rect::NOTHING;
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
                rect = draw(ui, &c, None);
            });
            out.textures_delta.clear();
            assert!(rect.width() > 40.0, "nothing was drawn: {rect:?}");
            assert!(rect.width() <= HOVER_W + 1.0, "too wide: {rect:?}");
            assert!(rect.height() <= HOVER_MAX_H, "too tall: {rect:?}");
        }
    }

    /// Height the card gains when the thumbnail is drawn, measured on the
    /// real card rather than read from the constant.
    fn thumbnail_gain(thumb_w: usize, thumb_h: usize) -> f32 {
        let c = card_with(Some(&heard()), &ready(info()), ThumbStatus::Ready);
        let ctx = egui::Context::default();
        let tex = ctx.load_texture(
            "thumb",
            egui::ColorImage::new([thumb_w, thumb_h], vec![egui::Color32::GRAY; thumb_w * thumb_h]),
            Default::default(),
        );
        let mut heights = [0.0_f32; 2];
        for (i, t) in [None, Some(&tex)].into_iter().enumerate() {
            let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
                heights[i] = draw(ui, &c, t).height();
            });
            out.textures_delta.clear();
        }
        heights[1] - heights[0]
    }

    #[test]
    fn the_thumbnail_is_drawn_a_quarter_larger_than_it_was() {
        // It was 84 px high and the operator found it too small. A Planespotters
        // thumbnail is 280 x 187 (3:2), so the proportions must survive.
        let gain = thumbnail_gain(280, 187);
        assert!(gain >= PHOTO_H, "the picture is not drawn at its reserved height: {gain}");
        assert!(PHOTO_H >= 104.0 && PHOTO_H <= 112.0, "about a quarter larger than 84: {PHOTO_H}");
        assert!(gain < PHOTO_H + 8.0, "{gain} is more than the picture and its gap");
    }

    #[test]
    fn the_larger_thumbnail_keeps_the_card_inside_its_bounds() {
        let c = card_with(Some(&heard()), &ready(info()), ThumbStatus::Ready);
        let ctx = egui::Context::default();
        let tex = ctx.load_texture(
            "thumb",
            egui::ColorImage::new([280, 187], vec![egui::Color32::GRAY; 280 * 187]),
            Default::default(),
        );
        let mut rect = egui::Rect::NOTHING;
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            rect = draw(ui, &c, Some(&tex));
        });
        out.textures_delta.clear();
        assert!(rect.width() <= HOVER_W + 1.0 && rect.height() <= HOVER_MAX_H, "{rect:?}");
    }

    /// Writes the card with a stand-in picture (a gradient, 3:2 like a
    /// Planespotters thumbnail), next to the old 84 px picture, to look at.
    #[test]
    #[ignore = "writes a picture to look at; not an assertion"]
    fn thumbnail_size_preview() {
        let (w, h) = (700.0_f32, 470.0_f32);
        let ctx = egui::Context::default();
        let px: Vec<egui::Color32> = (0..280 * 187)
            .map(|i| {
                let (x, y) = ((i % 280) as u8, (i / 280) as u8);
                egui::Color32::from_rgb(x, y, 200 - y / 2)
            })
            .collect();
        let tex = ctx.load_texture(
            "thumb",
            egui::ColorImage::new([280, 187], px),
            Default::default(),
        );
        let c = card_with(Some(&heard()), &ready(info()), ThumbStatus::Ready);
        let mut tx = crate::test_raster::Textures::default();
        let mut last = None;
        for i in 0..3 {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(w, h))),
                time: Some(1.0 + i as f64 / 30.0),
                ..Default::default()
            };
            let mut full = ctx.run_ui(input, |ui| {
                ui.horizontal_top(|ui| {
                    ui.vertical(|ui| {
                        ui.label("before: 84 px");
                        ui.add(egui::Image::new(&tex).max_height(84.0).corner_radius(3.0));
                    });
                    ui.vertical(|ui| {
                        ui.label(format!("after: {PHOTO_H} px, in the card"));
                        draw(ui, &c, Some(&tex));
                    });
                });
            });
            tx.apply(&full.textures_delta);
            full.textures_delta.clear();
            last = Some(full);
        }
        let img = crate::test_raster::rasterize(
            &ctx,
            last.unwrap(),
            &tx,
            egui::vec2(w, h),
            crate::theme::BG_DEEP(),
        );
        let out = std::env::var("THUMB_PREVIEW").unwrap_or_else(|_| {
            format!("{}/../../target/thumb_preview.png", env!("CARGO_MANIFEST_DIR"))
        });
        img.save(&out).expect("write the preview");
        println!("wrote {out}");
    }

    #[test]
    fn a_cell_never_prints_past_its_own_width() {
        let ctx = egui::Context::default();
        let mut widths = Vec::new();
        let mut out = ctx.run_ui(egui::RawInput::default(), |ui| {
            for w in [COL_LABEL, COL_VALUE] {
                let long = RichText::new("W".repeat(120)).size(11.0);
                widths.push((w, cell(ui, w, long).width()));
            }
        });
        out.textures_delta.clear();
        for (cell_w, text_w) in widths {
            assert!(text_w <= cell_w + 0.5, "text {text_w} in a cell of {cell_w}");
            assert!(text_w > cell_w * 0.5, "the text was not drawn: {text_w}");
        }
    }

    // ── the hover asks for nothing ────────────────────────────────────────

    // ── the pinned card ───────────────────────────────────────────────────

    fn registry_with_live() -> AircraftRegistry {
        let mut r = AircraftRegistry::default();
        r.ingest_adsb(&live());
        r
    }

    #[test]
    fn the_pinned_card_is_the_hover_card() {
        use super::super::aircraft_online::Answer;
        let reg = registry_with_live();
        let mut online = OnlineClient::default();
        online.store.deliver(0x4840D6, Answer::Info(Box::new(info())), NOW);
        let (pinned, tex) = card_view(&live(), &reg, &online, true, Some(HOME), NOW);
        let state = online.store.state_for(0x4840D6, true).clone();
        let hover =
            hover_card(&live(), reg.card(0x4840D6), &state, ThumbStatus::NotTried, Some(HOME), NOW);
        assert_eq!(pinned, hover);
        assert!(!pinned.lines.is_empty());
        assert!(tex.is_none(), "no thumbnail has been loaded");
    }

    #[test]
    fn the_pinned_link_is_only_a_planespotters_page() {
        use super::super::aircraft_online::Answer;
        let reg = registry_with_live();
        let mut online = OnlineClient::default();
        online.store.deliver(0x4840D6, Answer::Info(Box::new(info())), NOW);
        let (c, _) = card_view(&live(), &reg, &online, true, None, NOW);
        assert_eq!(
            c.photo.as_ref().and_then(|p| p.page_url.as_deref()),
            Some("https://www.planespotters.net/photo/12345/x")
        );
        let mut other = info();
        other.photo.as_mut().unwrap().page_url = "https://evil.example/photo/1".into();
        online.store.deliver(0x4840D6, Answer::Info(Box::new(other)), NOW);
        let (c, _) = card_view(&live(), &reg, &online, true, None, NOW);
        let p = c.photo.expect("the credit is still there");
        assert_eq!(p.page_url, None, "a page that is not Planespotters' is not linked");
        assert!(p.credit.contains("A. Spotter"));
    }

    #[test]
    fn the_pinned_card_shows_the_thumbnail_only_when_already_loaded() {
        use super::super::aircraft_online::Answer;
        let reg = registry_with_live();
        let mut online = OnlineClient::default();
        online.store.deliver(0x4840D6, Answer::Info(Box::new(info())), NOW);
        let (c, tex) = card_view(&live(), &reg, &online, true, None, NOW);
        let p = c.photo.expect("credit and link are there");
        assert_eq!(p.thumb_url, None, "never loaded, so no picture");
        assert!(tex.is_none());
        assert!(p.credit.contains("Planespotters") && p.page_url.is_some());
    }

    #[test]
    fn pinning_asks_for_nothing() {
        use super::super::aircraft_online::{Answer, Want};
        let reg = registry_with_live();
        let mut online = OnlineClient::default();
        online.store.deliver(0x4840D6, Answer::Info(Box::new(info())), NOW);
        let before = online.store.len();
        let url = "https://t.plnspttrs.net/12345/abc_t.jpg";
        let (c, _) = card_view(&live(), &reg, &online, true, Some(HOME), NOW);
        assert!(c.photo.is_some(), "the card is built from the answer already held");
        let _ = card_view(&live(), &reg, &online, false, Some(HOME), NOW);
        assert_eq!(online.store.len(), before);
        assert_eq!(online.thumb_status(url), ThumbStatus::NotTried, "a pin started a download");
        // An aircraft only pinned is still one nobody has asked about.
        let want = Want { icao: 0x123456, callsign: None, registration: None };
        assert_eq!(online.store.next_request(Some(&want), true, NOW), Some(want));
    }

    #[test]
    fn the_lookup_button_is_offered_for_an_aircraft_nobody_asked_and_after_a_failure_that_mends() {
        use LookupOffer::*;
        assert_eq!(lookup_offer(&OnlineState::Idle, true, false, Option::None), Button);
        assert_eq!(lookup_offer(&OnlineState::Idle, true, true, Option::None), Waiting);
        assert_eq!(lookup_offer(&OnlineState::Loading, true, true, Option::None), Asking);
        assert_eq!(lookup_offer(&ready(info()), true, false, Option::None), None);
        for r in [Reason::Offline, Reason::Busy, Reason::Unknown] {
            let st = OnlineState::Unavailable(r);
            assert_eq!(lookup_offer(&st, true, false, Some(12)), Retry(12), "{r:?}");
            assert!(Retry(12).pressable());
            assert_eq!(lookup_offer(&st, true, false, Option::None), None, "{r:?}: no retry");
        }
        assert!(Button.pressable());
        for o in [None, Asking, Waiting] {
            assert!(!o.pressable(), "{o:?}");
        }
        assert_eq!(lookup_offer(&OnlineState::Idle, false, false, Option::None), None, "no station");
        let off = OnlineState::Unavailable(Reason::NoStation);
        assert_eq!(lookup_offer(&off, false, false, Some(3)), None, "no station");
    }

    #[test]
    fn no_lookup_button_when_the_relay_is_off() {
        use super::super::aircraft_online::Answer;
        use LookupOffer::*;
        // Control: the same aircraft on a station that has not said no is offered it.
        let fresh = super::super::aircraft_online::OnlineStore::default();
        assert_eq!(lookup_offer(fresh.state_for(1, true), true, false, Option::None), Button);
        let mut store = super::super::aircraft_online::OnlineStore::default();
        // The station says it will not answer: every aircraft's state follows.
        store.deliver(1, Answer::Unavailable(Reason::Disabled), NOW);
        for icao in [1_u32, 2, 3] {
            let st = store.state_for(icao, true);
            assert_eq!(lookup_offer(st, true, false, Option::None), None, "{icao}: {st:?}");
        }
        let mut store = super::super::aircraft_online::OnlineStore::default();
        store.deliver(1, Answer::Unavailable(Reason::Unauthorized), NOW);
        assert_eq!(lookup_offer(store.state_for(2, true), true, false, Option::None), None);
        let store = super::super::aircraft_online::OnlineStore::default();
        assert_eq!(lookup_offer(store.state_for(2, false), false, false, Option::None), None);
    }

    // ── pressing the pinned card ──────────────────────────────────────────

    /// One frame of `draw_pinned` in a window of its own, with `events` fed in.
    fn pinned_frame(
        ctx: &egui::Context,
        events: Vec<egui::Event>,
        card: &HoverCard,
        offer: LookupOffer,
    ) -> PinnedOut {
        pinned_frame_h(ctx, events, card, offer, HistoryOffer::Hidden)
    }

    fn pinned_frame_h(
        ctx: &egui::Context,
        events: Vec<egui::Event>,
        card: &HoverCard,
        offer: LookupOffer,
        history: HistoryOffer,
    ) -> PinnedOut {
        let mut raw = egui::RawInput::default();
        raw.screen_rect =
            Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0)));
        raw.events = events;
        let mut out = PinnedOut::default();
        let mut full = ctx.run_ui(raw, |ui| {
            out = draw_pinned(ui, card, None, offer, history);
        });
        full.textures_delta.clear();
        out
    }

    /// The card, laid out once, then a click at `pick(layout)` in the next frame.
    fn press(offer: LookupOffer, pick: impl Fn(&PinnedOut) -> Option<egui::Rect>) -> PinnedOut {
        press_h(offer, HistoryOffer::Hidden, pick)
    }

    fn press_h(
        offer: LookupOffer,
        history: HistoryOffer,
        pick: impl Fn(&PinnedOut) -> Option<egui::Rect>,
    ) -> PinnedOut {
        use super::super::aircraft_online::Answer;
        let reg = registry_with_live();
        let mut online = OnlineClient::default();
        online.store.deliver(0x4840D6, Answer::Info(Box::new(info())), NOW);
        let (card, _) = card_view(&live(), &reg, &online, true, Some(HOME), NOW);
        let ctx = egui::Context::default();
        let laid = pinned_frame_h(&ctx, vec![], &card, offer, history);
        let _ = pinned_frame_h(&ctx, vec![], &card, offer, history);
        let at = pick(&laid).expect("the thing to press was not drawn").center();
        let button = |pressed| egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        };
        pinned_frame_h(
            &ctx,
            vec![egui::Event::PointerMoved(at), button(true), button(false)],
            &card,
            offer,
            history,
        )
    }

    #[test]
    fn pressing_the_link_hands_back_the_page() {
        let out = press(LookupOffer::None, |o| o.link);
        assert_eq!(
            out.actions,
            [PinAction::OpenLink("https://www.planespotters.net/photo/12345/x".into())]
        );
    }

    #[test]
    fn pressing_the_cross_closes_the_card() {
        let out = press(LookupOffer::None, |o| o.close);
        assert_eq!(out.actions, [PinAction::Close]);
    }

    #[test]
    fn pressing_the_lookup_button_asks_to_look_up() {
        let out = press(LookupOffer::Button, |o| o.lookup);
        assert_eq!(out.actions, [PinAction::Lookup]);
    }

    #[test]
    fn the_lookup_button_is_drawn_only_when_offered_and_never_pressable_while_asking() {
        use super::super::aircraft_online::Answer;
        let reg = registry_with_live();
        let online = OnlineClient::default();
        let _ = Answer::Unavailable(Reason::Offline);
        let (card, _) = card_view(&live(), &reg, &online, true, Some(HOME), NOW);
        let ctx = egui::Context::default();
        for (offer, drawn) in [
            (LookupOffer::None, false),
            (LookupOffer::Button, true),
            (LookupOffer::Asking, false),
            (LookupOffer::Waiting, false),
            (LookupOffer::Retry(7), true),
        ] {
            let out = pinned_frame(&ctx, vec![], &card, offer);
            assert_eq!(out.lookup.is_some(), drawn, "{offer:?}");
            assert!(out.actions.is_empty());
        }
    }

    fn shipped(src: &str) -> &str {
        src.split("\n#[cfg(test)]").next().unwrap_or(src)
    }

    #[test]
    fn the_hover_code_asks_for_nothing() {
        let banned = [
            concat!(".as", "k("),
            concat!("want_", "thumb("),
            concat!(".po", "ll("),
            concat!("next_", "request("),
            concat!("deli", "ver("),
            concat!("send_", "info"),
            concat!("send_", "thumb"),
        ];
        let code = shipped(include_str!("aircraft_hover.rs"));
        for b in banned {
            assert!(!code.contains(b), "the hover contains {b}");
        }
    }

    #[test]
    fn building_a_card_leaves_the_store_as_it_found_it() {
        use super::super::aircraft_online::{Answer, OnlineStore, Want};
        let mut store = OnlineStore::default();
        store.deliver(0x4840D6, Answer::Info(Box::new(info())), NOW);
        let before = store.len();
        for icao in [0x4840D6_u32, 0x123456] {
            let state = store.state_for(icao, true).clone();
            let _ = hover_card(&live(), Some(&heard()), &state, ThumbStatus::NotTried, None, NOW);
        }
        assert_eq!(store.len(), before, "a hover created an entry");
        // An aircraft only hovered is still one nobody has asked about.
        let want = Want { icao: 0x123456, callsign: None, registration: None };
        assert_eq!(store.next_request(Some(&want), true, NOW), Some(want));
    }

    #[test]
    fn the_hover_keeps_nothing_in_the_browser_and_names_no_service() {
        let banned = [
            concat!("local", "Storage"),
            concat!("session", "Storage"),
            concat!("Indexed", "DB"),
            concat!("api.", "adsbdb"),
            concat!("api.", "planespotters"),
            concat!("api_", "key"),
            concat!("api", "key"),
            concat!("x-api-", "key"),
            concat!("Bear", "er "),
            concat!("web_sys", "::"),
        ];
        let code = shipped(include_str!("aircraft_hover.rs"));
        for b in banned {
            assert!(!code.contains(b), "the hover contains {b}");
        }
    }

    // ── the history button ────────────────────────────────────────────────

    #[test]
    fn pressing_the_history_button_asks_to_toggle_it() {
        for h in [HistoryOffer::Show, HistoryOffer::On] {
            let out = press_h(LookupOffer::None, h, |o| o.history);
            assert_eq!(out.actions, [PinAction::History], "{h:?}");
        }
    }

    #[test]
    fn the_history_button_is_drawn_only_when_offered() {
        let reg = registry_with_live();
        let online = OnlineClient::default();
        let (card, _) = card_view(&live(), &reg, &online, true, Some(HOME), NOW);
        let ctx = egui::Context::default();
        let hidden = pinned_frame_h(&ctx, vec![], &card, LookupOffer::None, HistoryOffer::Hidden);
        assert!(hidden.history.is_none(), "{hidden:?}");
        let ctx = egui::Context::default();
        let shown = pinned_frame_h(&ctx, vec![], &card, LookupOffer::None, HistoryOffer::Show);
        assert!(shown.history.is_some(), "the control: offered, so drawn");
        // The hover card is not the pinned card: it has no buttons at all.
        let ctx = egui::Context::default();
        let mut area = None;
        let mut full = ctx.run_ui(egui::RawInput::default(), |ui| {
            area = Some(draw(ui, &card, None));
        });
        full.textures_delta.clear();
        assert!(area.is_some());
    }
}
