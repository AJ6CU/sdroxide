//! The watch list through a whole app frame: that an aircraft coming on the air
//! reaches the banner, the AIRCRAFT chip and the Watch view, and that looking at
//! it is what makes it seen. The station is a loopback address that nothing
//! listens on, and the list is already "read" and fresh, so no request is due.

use eframe::App;
use eframe::egui;
use sdroxide_aircraft_history::watch::{WatchReply, Watchlist};
use sdroxide_types::{AdsbAircraft, RadioController, RadioEvent};

use super::SdroxideApp;
use super::aircraft_watch::{Answer, WatchState};

struct Station;

impl RadioController for Station {
    fn send(&mut self, _cmd: sdroxide_types::Command) {}
    fn poll_event(&mut self) -> Option<RadioEvent> {
        None
    }
    fn peer_url(&self) -> Option<String> {
        Some("wss://127.0.0.1:9/ws".into())
    }
}

fn texts(shapes: &[egui::epaint::ClippedShape]) -> String {
    fn walk(s: &egui::Shape, out: &mut String) {
        match s {
            egui::Shape::Text(t) => {
                out.push_str(t.galley.text());
                out.push('\n');
            }
            egui::Shape::Vec(v) => v.iter().for_each(|s| walk(s, out)),
            _ => {}
        }
    }
    let mut out = String::new();
    shapes.iter().for_each(|c| walk(&c.shape, &mut out));
    out
}

fn frame(ctx: &egui::Context, app: &mut SdroxideApp, t: f64) -> String {
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1600.0, 900.0))),
        time: Some(t),
        ..Default::default()
    };
    let mut out = ctx.run_ui(input, |ui| app.ui(ui, &mut eframe::Frame::_new_kittest()));
    let text = texts(&out.shapes);
    out.textures_delta.clear();
    text
}

fn now() -> i64 {
    crate::time::now_unix()
}

#[test]
fn a_watched_aircraft_coming_on_the_air_shows_on_the_banner_the_chip_and_the_view() {
    let _guard = crate::multi::frame_test_lock();
    let dir = std::env::temp_dir().join(format!("sdroxide-watch-frame-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };

    let ctx = egui::Context::default();
    let mut app = SdroxideApp::new_tab(&ctx, None, None, Box::new(Station), 0, true);
    app.show_aircraft = true;
    app.show_extras = true;

    // The station has said what the list is, a moment ago.
    let mut list = Watchlist::default();
    list.add("KLM1234");
    app.aircraft_online.watch.next_request(true, now());
    app.aircraft_online.watch.deliver(Answer::Reply(WatchReply::of(&list, None)), now());
    assert_eq!(*app.aircraft_online.watch.state(), WatchState::Ready);
    // The tracker has been looking since long enough to announce.
    app.aircraft_tracker.observe(now() - 100, &list, &[]);

    let mut t = 0.0;
    let mut go = |app: &mut SdroxideApp| {
        t += 0.1;
        frame(&ctx, app, t)
    };
    // A few frames for the windows to open and size themselves.
    let mut quiet = String::new();
    for _ in 0..4 {
        quiet = go(&mut app);
    }
    assert!(!quiet.contains("is on the air"), "nobody is on the air yet:\n{quiet}");
    assert!(!quiet.contains('\u{25cf}'), "no count on the chip");
    assert!(quiet.contains("Watch 1"), "the Watch button counts the list:\n{quiet}");

    // The aircraft is heard.
    let mut a = AdsbAircraft::new(0x4840D6, now());
    a.callsign = "KLM1234".into();
    a.last_at = now();
    app.aircraft_registry.ingest_adsb(&a);
    // The cards are looked at once a second.
    std::thread::sleep(std::time::Duration::from_millis(1100));
    let mut shown = String::new();
    for _ in 0..5 {
        shown = go(&mut app);
        if shown.contains("is on the air") {
            break;
        }
    }
    assert!(shown.contains("KLM1234 is on the air"), "the banner:\n{shown}");
    // The chip is drawn before the cards are looked at, so it wears the count
    // from the next frame on.
    let shown = go(&mut app);
    assert!(shown.contains("AIRCRAFT \u{25cf}1"), "the chip wears a count:\n{shown}");
    assert_eq!(app.aircraft_tracker.unseen(), 1);

    // Opening the view shows it, and looking is what makes it seen.
    app.aircraft_watch_view = true;
    let view = go(&mut app);
    assert!(view.contains("on the air now"), "the entry is marked:\n{view}");
    assert!(view.contains("Alerts"), "{view}");
    assert!(view.contains("KLM1234 is on the air"), "the alert is listed:\n{view}");
    assert_eq!(app.aircraft_tracker.unseen(), 0, "seen");
    let after = go(&mut app);
    assert!(!after.contains("AIRCRAFT \u{25cf}"), "the count is gone:\n{after}");
    assert!(after.contains("KLM1234 is on the air"), "but the alert is kept in the list");
}

#[test]
fn without_a_station_the_view_says_where_the_list_is_kept() {
    struct NoStation;
    impl RadioController for NoStation {
        fn send(&mut self, _cmd: sdroxide_types::Command) {}
        fn poll_event(&mut self) -> Option<RadioEvent> {
            None
        }
    }
    let _guard = crate::multi::frame_test_lock();
    let dir = std::env::temp_dir().join(format!("sdroxide-watch-nostation-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };
    let ctx = egui::Context::default();
    let mut app = SdroxideApp::new_tab(&ctx, None, None, Box::new(NoStation), 0, true);
    app.show_aircraft = true;
    app.aircraft_watch_view = true;
    let mut text = String::new();
    for i in 1..4 {
        text = frame(&ctx, &mut app, f64::from(i) * 0.1);
    }
    assert!(text.contains("kept by the station you are connected to"), "{text}");
    assert!(!text.contains("Add"), "no box to type in when there is nobody to keep it:\n{text}");
}
