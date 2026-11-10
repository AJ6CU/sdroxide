//! The card pinned to an aircraft on the map, and the one button in it that
//! asks.
//!
//! A click on an aircraft pins its card ([`crate::adsb_map`] holds which); the
//! card is the hover card's ([`super::aircraft_hover`]) with a link that can be
//! followed, a close button and, where it makes sense, **Look up online**. The
//! click and the card ask for nothing. Only that button does, once per press,
//! for the pinned aircraft alone, through the same line the AIRCRAFT window's
//! card uses — one question at a time, the station's own ceiling, the same
//! back-off. After a press the card follows that one question every frame, as
//! the Cards view does: a question that never came back is given up on, a
//! failure that mends itself is asked again once its back-off is over (the
//! card says how long, and the button hurries it), and the thumbnail is
//! fetched whenever the answer holds a picture, so it is back when the card is
//! opened again.

// ==== IMPL ====
use eframe::egui;
use sdroxide_types::AdsbAircraft;

use super::SdroxideApp;
use super::aircraft_card::CardKey;
use super::aircraft_history;
use super::aircraft_hover::{self, PinAction};
use super::aircraft_online::{self, OnlineState, ThumbStatus, Want};
use super::aircraft_registry::AircraftCard;
use crate::adsb_map;

/// The question a pressed lookup sends: the pinned aircraft's.
pub(in crate::app) fn lookup_want(pinned: Option<u32>, want: Option<Want>) -> Option<Want> {
    let want = want?;
    pinned.is_some_and(|p| p & 0xff_ffff == want.icao).then_some(want)
}

/// The thumbnail to fetch now: whenever the pinned aircraft's answer carries a
/// picture Planespotters hands out. An answer is only there after a lookup, so
/// nothing is fetched for an aircraft nobody asked about; and it does not wait
/// for a new press, so the picture comes back when the card is opened again.
pub(in crate::app) fn thumb_wish(pinned: Option<u32>, state: &OnlineState) -> Option<String> {
    pinned?;
    aircraft_online::photo_view(state, ThumbStatus::NotTried)
        .filter(|p| p.show_image)
        .map(|p| p.thumb_url)
}

/// Whether a pressed lookup is still being followed: until its aircraft is no
/// longer the pinned one, or its question ended for good (the relay is off or
/// refuses us). A failure that mends itself is still followed.
pub(in crate::app) fn lookup_after(
    lookup: Option<u32>,
    pinned: Option<u32>,
    state: &OnlineState,
) -> Option<u32> {
    let followed = lookup.filter(|_| lookup == pinned);
    followed.filter(|_| match state {
        // Idle: pressed, and the question goes out on the next line.
        OnlineState::Idle | OnlineState::Loading | OnlineState::Ready(_) => true,
        OnlineState::Unavailable(r) => r.retry_after().is_some(),
    })
}

impl SdroxideApp {
    /// Give the open airport box what the station can tell it — the Wikipedia
    /// picture, and the local time and temperature — and keep it only while the
    /// box is open: the box asks every frame it is drawn, and what it holds goes
    /// (here and at the station) when it stops.
    pub(in crate::app) fn adsb_airport_online(&mut self, ctx: &egui::Context, now: i64) {
        let airport = self
            .adsb_map
            .airport_pin
            .and_then(|i| crate::airdata::airports().get(i).map(|a| (i, a)));
        let Some((i, a)) = airport else {
            self.adsb_map.airport_photo = None;
            self.adsb_map.airport_weather = None;
            return;
        };
        // What the line delivered since the last frame also arrives here, when
        // the box is the only thing open.
        self.aircraft_online.poll(ctx, now);
        let peer = self.ctrl.peer_url();
        let before = self.adsb_map.airport_photo.as_ref().map(|p| std::mem::discriminant(&p.photo));
        self.adsb_map.airport_photo = self
            .aircraft_online
            .airport_card_photo(ctx, peer.as_deref(), &a.ident)
            .map(|(photo, page_url)| adsb_map::AirportPhoto { airport: i, photo, page_url });
        // This runs after the map is drawn: a picture that arrived (or was
        // given up on) in the poll above is only seen by a frame drawn after
        // it, and nothing else asks for one — the card stayed on its "…" until
        // the pointer moved.
        let after = self.adsb_map.airport_photo.as_ref().map(|p| std::mem::discriminant(&p.photo));
        if before != after {
            ctx.request_repaint();
        }
        let clock = self.aircraft_online.airport_clock(ctx, peer.as_deref(), i, (a.lat, a.lon));
        self.adsb_map.airport_weather = Some(adsb_map::AirportWeather {
            airport: i,
            clock: match clock {
                aircraft_online::Clock::Offline => adsb_map::AirportClock::Offline,
                aircraft_online::Clock::Asking => adsb_map::AirportClock::Asking,
                aircraft_online::Clock::Known(w) => {
                    adsb_map::AirportClock::Known { utc_offset_s: w.utc_offset_s, temp_c: w.temp_c }
                }
            },
            now,
        });
    }

    /// Draw the pinned card over the map, and do what was pressed in it.
    pub(in crate::app) fn adsb_pinned_card(
        &mut self,
        ui: &mut egui::Ui,
        aircraft: &[AdsbAircraft],
        home: Option<(f64, f64)>,
        now: i64,
    ) {
        // The record turned on from the pin goes when the pin does.
        let (on, owner) = aircraft_history::after_pin(
            self.aircraft_online.history.on(),
            self.pin_history_owner,
            self.adsb_map.pinned,
        );
        self.aircraft_online.history.set_on(on);
        self.pin_history_owner = owner;
        let Some(pinned) = self.adsb_map.pinned else {
            self.adsb_map.lookup = None;
            return;
        };
        let Some(a) = aircraft.iter().find(|a| a.icao == pinned) else { return };
        // Another map (the ADS-B panel under the AIRCRAFT window) shares this
        // pin: only the map that was clicked draws its card.
        if !adsb_map::owns_pin(self.adsb_map.pin_owner, ui.id()) {
            return;
        }
        let ctx = ui.ctx().clone();

        // Esc closes it, unless a text field has the keyboard.
        let typing = ctx.egui_wants_keyboard_input();
        let esc =
            !typing && ctx.input_mut(|i| i.consume_key(egui::Modifiers::NONE, egui::Key::Escape));
        if adsb_map::pin_after_key(Some(pinned), esc, typing).is_none() {
            self.adsb_map.pinned = adsb_map::pin_after_close(self.adsb_map.pinned);
            self.adsb_map.lookup = None;
            return;
        }
        let (Some(anchor), Some(bounds)) = (self.adsb_map.pin_anchor, self.adsb_map.pin_bounds)
        else {
            return;
        };

        // What the line has delivered since the last frame; this is also what
        // brings a lookup's answer in while the Cards view is not on screen.
        self.aircraft_online.poll(&ctx, now);
        let peer = self.ctrl.peer_url();
        let has_station = peer.is_some();
        // The station's own record, fetched again every 30 s while it is on.
        self.aircraft_online.ask_history(&ctx, peer.as_deref(), now);
        let icao = a.icao & 0xff_ffff;
        let state = self.aircraft_online.store.state_for(icao, has_station).clone();
        self.adsb_map.lookup = lookup_after(self.adsb_map.lookup, Some(pinned), &state);
        // A pressed lookup is followed every frame, like the Cards view does:
        // the store answers `Some` once, then only when a back-off is over, and
        // it gives up on a question that never came back.
        if self.adsb_map.lookup.is_some() {
            let none = AircraftCard::default();
            let card = super::aircraft_card::lookup(&self.aircraft_registry, &CardKey::Icao(icao))
                .unwrap_or(&none);
            let want =
                lookup_want(Some(pinned), aircraft_online::want_for(&CardKey::Icao(icao), card));
            self.aircraft_online.ask(&ctx, peer.as_deref(), want.as_ref(), now);
            if !matches!(state, OnlineState::Ready(_)) {
                // The countdown moves, and a lost question is given up on.
                ctx.request_repaint_after(std::time::Duration::from_secs(1));
            }
        }
        if let Some(url) = thumb_wish(Some(pinned), &state) {
            self.aircraft_online.want_thumb(&ctx, peer.as_deref(), icao, &url);
        }
        let retry_in = self.aircraft_online.store.retry_in(icao, now);
        let offer = aircraft_hover::lookup_offer(
            &state,
            has_station,
            self.aircraft_online.store.line_busy(now),
            retry_in,
        );
        let history =
            aircraft_history::offer(self.aircraft_online.history.on(), Some(icao), has_station);

        let size = self
            .adsb_map
            .pin_size
            .unwrap_or_else(|| egui::vec2(aircraft_hover::HOVER_W + 16.0, 220.0));
        let at = adsb_map::pin_placement(anchor, size, bounds);
        let (card, tex) = aircraft_hover::card_view(
            a,
            &self.aircraft_registry,
            &self.aircraft_online,
            has_station,
            home,
            now,
        );
        let shown = egui::Area::new(egui::Id::new("adsb-pinned-card"))
            .order(egui::Order::Middle)
            .fixed_pos(at)
            .constrain_to(bounds)
            .show(&ctx, |ui| {
                egui::Frame::popup(ui.style())
                    .show(ui, |ui| aircraft_hover::draw_pinned(ui, &card, tex, offer, history))
                    .inner
            });
        self.adsb_map.pin_size = Some(shown.response.rect.size());

        for act in shown.inner.actions {
            match act {
                PinAction::Close => {
                    self.adsb_map.pinned = adsb_map::pin_after_close(self.adsb_map.pinned);
                    self.adsb_map.lookup = None;
                }
                PinAction::OpenLink(url) => {
                    if !crate::download::open_external(&url) {
                        tracing::warn!(%url, "no handler for this link");
                    }
                }
                PinAction::History => {
                    self.aircraft_online.history.toggle(icao);
                    // Turned on from here, it goes with this pin.
                    self.pin_history_owner =
                        (self.aircraft_online.history.on() == Some(icao)).then_some(icao);
                }
                PinAction::Lookup => {
                    // The question itself goes out on the next frame, from the
                    // line above; a press after a failure ends its back-off.
                    if offer.pressable() {
                        self.aircraft_online.store.retry_now(icao);
                        self.adsb_map.lookup = Some(pinned);
                        ctx.request_repaint();
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::aircraft_online::{Answer, OnlineStore, Reason};
    use super::*;
    use sdroxide_aircraft_online::{Info, Photo, Status};

    const NOW: i64 = 1_000_000;

    fn want(icao: u32) -> Want {
        Want { icao, callsign: Some("KLM1234".into()), registration: None }
    }

    fn info_with(thumb: &str) -> Info {
        let mut i = Info::bare(Status::Ok);
        i.photo = Some(Photo {
            thumb_url: thumb.into(),
            page_url: "https://www.planespotters.net/photo/1/x".into(),
            photographer: "A. Spotter".into(),
        });
        i
    }

    fn ready_with(thumb: &str) -> OnlineState {
        OnlineState::Ready(Box::new(info_with(thumb)))
    }

    const THUMB: &str = "https://t.plnspttrs.net/1/a_t.jpg";

    #[test]
    fn a_pressed_lookup_asks_only_the_pinned_aircraft() {
        // What a pressed lookup may send.
        assert_eq!(lookup_want(Some(5), Some(want(5))), Some(want(5)));
        assert_eq!(lookup_want(Some(5), Some(want(6))), None);
        assert_eq!(lookup_want(None, Some(want(5))), None);
        assert_eq!(lookup_want(Some(5), None), None);

        // What the line does with it: once, without holding another aircraft
        // up, and no second try before the back-off is over.
        let mut store = OnlineStore::default();
        let w = lookup_want(Some(5), Some(want(5))).unwrap();
        assert_eq!(store.next_request(Some(&w), true, NOW), Some(w.clone()));
        assert_eq!(store.next_request(Some(&w), true, NOW + 1), None, "asked every frame");
        assert!(store.next_request(Some(&want(6)), true, NOW + 1).is_some(), "not held up");
        store.deliver(5, Answer::Unavailable(Reason::Busy), NOW + 2);
        assert_eq!(store.next_request(Some(&w), true, NOW + 3), None, "back-off");
    }

    #[test]
    fn the_thumbnail_is_wanted_whenever_the_pinned_answer_has_one() {
        let st = ready_with(THUMB);
        assert_eq!(thumb_wish(Some(5), &st).as_deref(), Some(THUMB));
        assert_eq!(thumb_wish(None, &st), None);
        for st in
            [OnlineState::Idle, OnlineState::Loading, OnlineState::Unavailable(Reason::Offline)]
        {
            assert_eq!(thumb_wish(Some(5), &st), None, "{st:?}: nothing asked about");
        }
        let elsewhere = ready_with("https://evil.example/a.jpg");
        assert_eq!(thumb_wish(Some(5), &elsewhere), None, "only the picture host");
    }

    #[test]
    fn a_lookup_ends_with_its_aircraft_or_a_failure_that_does_not_mend() {
        let ready = ready_with(THUMB);
        assert_eq!(lookup_after(Some(5), Some(5), &OnlineState::Loading), Some(5));
        assert_eq!(lookup_after(Some(5), Some(5), &OnlineState::Idle), Some(5), "just pressed");
        assert_eq!(lookup_after(Some(5), Some(5), &ready), Some(5), "the picture is still coming");
        assert_eq!(lookup_after(Some(5), Some(6), &ready), None, "the pin moved on");
        assert_eq!(lookup_after(Some(5), None, &ready), None, "the card was closed");
        for r in [Reason::Offline, Reason::Busy, Reason::Unknown] {
            let st = OnlineState::Unavailable(r);
            assert_eq!(lookup_after(Some(5), Some(5), &st), Some(5), "{r:?} is asked again");
        }
        for r in [Reason::Disabled, Reason::Unauthorized, Reason::NoStation] {
            let st = OnlineState::Unavailable(r);
            assert_eq!(lookup_after(Some(5), Some(5), &st), None, "{r:?} is final");
        }
        assert_eq!(lookup_after(None, Some(5), &ready), None);
    }

    /// The reported run: look up A, close it, open B, open A again, look up
    /// again, with the questions that go missing in between.
    #[test]
    fn a_card_closed_and_opened_again_gets_its_picture_and_its_second_lookup() {
        let mut store = OnlineStore::default();
        let a = lookup_want(Some(5), Some(want(5))).unwrap();
        let b = lookup_want(Some(6), Some(want(6))).unwrap();

        // A is looked up and answers with a picture.
        assert!(store.next_request(Some(&a), true, NOW).is_some());
        store.deliver(5, Answer::Info(Box::new(info_with(THUMB))), NOW + 1);
        assert_eq!(thumb_wish(Some(5), store.state_for(5, true)).as_deref(), Some(THUMB));

        // The card is closed (the picture is released), B is looked up and the
        // answer to it never comes back.
        let mut lookup = lookup_after(Some(5), None, store.state_for(5, true));
        assert_eq!(lookup, None);
        assert!(store.next_request(Some(&b), true, NOW + 10).is_some());

        // A is opened again: its picture is wanted again without a new press.
        let st = store.state_for(5, true).clone();
        assert_eq!(thumb_wish(Some(5), &st).as_deref(), Some(THUMB), "picture on reopening");

        // B, pressed, is followed: the lost question is given up on and asked
        // again once its back-off is over.
        lookup = lookup_after(Some(6), Some(6), store.state_for(6, true));
        assert_eq!(lookup, Some(6));
        let lost = NOW + 10 + aircraft_online::FLIGHT_TIMEOUT_S;
        assert_eq!(store.next_request(Some(&b), true, lost), None, "given up on");
        assert_eq!(
            store.retry_in(6, lost),
            Some(aircraft_online::RETRY_OFFLINE_S),
            "the card can say how long"
        );
        assert_eq!(lookup_after(lookup, Some(6), store.state_for(6, true)), Some(6));
        assert_eq!(store.next_request(Some(&b), true, lost + 10), None);
        assert_eq!(store.retry_in(6, lost + 10), Some(aircraft_online::RETRY_OFFLINE_S - 10));
        let due = lost + aircraft_online::RETRY_OFFLINE_S;
        assert!(store.next_request(Some(&b), true, due).is_some(), "asked again by itself");
    }

    #[test]
    fn pressing_again_ends_the_back_off_of_a_failure_and_only_of_that() {
        let mut store = OnlineStore::default();
        let w = want(5);
        assert!(store.next_request(Some(&w), true, NOW).is_some());
        store.deliver(5, Answer::Unavailable(Reason::Offline), NOW + 1);
        assert_eq!(store.next_request(Some(&w), true, NOW + 2), None);
        store.retry_now(5);
        assert_eq!(store.retry_in(5, NOW + 2), Some(0));
        assert!(store.next_request(Some(&w), true, NOW + 2).is_some(), "one press, one question");
        assert_eq!(store.next_request(Some(&w), true, NOW + 3), None, "not twice");
        // A card that was never asked, or has its answer, has no back-off.
        store.retry_now(9);
        assert_eq!(store.retry_in(9, NOW), None);
        store.deliver(5, Answer::Info(Box::new(info_with(THUMB))), NOW + 4);
        store.retry_now(5);
        assert_eq!(store.retry_in(5, NOW + 4), None);
    }

    fn shipped(src: &str) -> &str {
        src.split("\n#[cfg(test)]").next().unwrap_or(src)
    }

    #[test]
    fn the_only_question_the_pinned_card_asks_is_the_lookup_buttons() {
        let code = shipped(include_str!("aircraft_pin.rs"));
        let ask = concat!(".as", "k(");
        let thumb = concat!("want_", "thumb(");
        assert_eq!(code.matches(ask).count(), 1, "one place may ask");
        assert_eq!(code.matches(thumb).count(), 1, "one place may fetch a picture");
        assert!(
            code.matches(concat!("next_", "request(")).count() == 0,
            "the line is the client's"
        );
    }

    #[test]
    fn the_pinned_card_keeps_nothing_in_the_browser_and_names_no_service() {
        let banned = [
            concat!("local", "Storage"),
            concat!("session", "Storage"),
            concat!("Indexed", "DB"),
            concat!("api.", "adsbdb"),
            concat!("api.", "planespotters"),
            concat!("api_", "key"),
            concat!("web_sys", "::"),
        ];
        for src in [include_str!("aircraft_pin.rs"), include_str!("aircraft_hover.rs")] {
            let code = shipped(src);
            for b in banned {
                assert!(!code.contains(b), "contains {b}");
            }
        }
    }
}
