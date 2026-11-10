//! The fleet export through a whole app: that the AIRCRAFT window offers the two
//! formats, and that pressing one with no station gives a file made from the
//! cards held, with the reason. The file dialog is not opened: the poll gives
//! the file back and saving it is the caller's.

use eframe::App;
use eframe::egui;
use sdroxide_aircraft_history::export::Format;
use sdroxide_types::{AdsbAircraft, RadioController, RadioEvent};

use super::SdroxideApp;

struct NoStation;

impl RadioController for NoStation {
    fn send(&mut self, _cmd: sdroxide_types::Command) {}
    fn poll_event(&mut self) -> Option<RadioEvent> {
        None
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

fn app() -> (egui::Context, SdroxideApp) {
    let dir = std::env::temp_dir().join(format!("sdroxide-export-frame-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };
    let ctx = egui::Context::default();
    let app = SdroxideApp::new_tab(&ctx, None, None, Box::new(NoStation), 0, true);
    (ctx, app)
}

#[test]
fn the_window_offers_both_formats() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app) = app();
    app.show_aircraft = true;
    let mut text = String::new();
    for i in 1..5 {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600.0, 900.0),
            )),
            time: Some(f64::from(i) * 0.1),
            ..Default::default()
        };
        let mut out = ctx.run_ui(input, |ui| app.ui(ui, &mut eframe::Frame::_new_kittest()));
        text = texts(&out.shapes);
        out.textures_delta.clear();
    }
    for want in ["Export", "CSV", "JSON"] {
        assert!(text.lines().any(|l| l == want), "no `{want}` in the window:\n{text}");
    }
}

#[test]
fn pressed_with_no_station_the_file_is_made_from_the_cards_held_and_says_why() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app) = app();
    let t = crate::time::now_unix();
    let mut a = AdsbAircraft::new(0x4840D6, t);
    a.callsign = "KLM1234".into();
    a.lat = Some(52.3123);
    a.lon = Some(4.7634);
    a.altitude_ft = Some(35_000);
    a.last_at = t;
    a.last_pos_at = t;
    app.aircraft_registry.ingest_adsb(&a);

    assert!(app.aircraft_export_poll(&ctx).is_none(), "nothing pressed, nothing made");
    assert!(app.aircraft_online.export.start(Format::Csv));
    let done = app.aircraft_export_poll(&ctx).expect("a file");
    assert!(done.name.starts_with("aircraft-") && done.name.ends_with(".csv"), "{}", done.name);
    let csv = String::from_utf8(done.bytes).unwrap();
    assert!(csv.starts_with("icao,callsign,"), "{csv}");
    assert!(csv.contains("4840D6,KLM1234,"), "{csv}");
    for banned in ["52.31", "4.76", "35000", "altitude"] {
        assert!(!csv.contains(banned), "`{banned}` in {csv}");
    }
    assert!(done.note.starts_with("History is kept by the station"), "{}", done.note);
    assert!(done.note.contains("1 aircraft"), "{}", done.note);
    assert!(app.aircraft_export_poll(&ctx).is_none(), "one file per press");
    assert!(!app.aircraft_online.export.busy());

    assert!(app.aircraft_online.export.start(Format::Json));
    let done = app.aircraft_export_poll(&ctx).expect("a file");
    assert!(done.name.ends_with(".json"));
    let v: serde_json::Value = serde_json::from_slice(&done.bytes).unwrap();
    assert_eq!(v["aircraft"][0]["callsign"], "KLM1234");
}

fn frame(
    ctx: &egui::Context,
    app: &mut SdroxideApp,
    t: f64,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1600.0, 900.0))),
        time: Some(t),
        events,
        ..Default::default()
    };
    ctx.run_ui(input, |ui| app.ui(ui, &mut eframe::Frame::_new_kittest()))
}

fn find_text(shapes: &[egui::epaint::ClippedShape], text: &str) -> Option<egui::Pos2> {
    fn walk(s: &egui::Shape, text: &str) -> Option<egui::Pos2> {
        match s {
            egui::Shape::Text(t) if t.galley.text() == text => {
                Some(t.pos + t.galley.rect.center().to_vec2())
            }
            egui::Shape::Vec(v) => v.iter().find_map(|s| walk(s, text)),
            _ => None,
        }
    }
    shapes.iter().find_map(|c| walk(&c.shape, text))
}

/// A press on the CSV button starts an export. The file is not made in the same
/// frame, so the dialog that saving it would open is not reached.
#[test]
fn a_press_on_the_csv_button_starts_an_export() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app) = app();
    app.show_aircraft = true;
    let mut t = 0.0;
    let mut at = None;
    for _ in 0..4 {
        t += 0.1;
        let out = frame(&ctx, &mut app, t, Vec::new());
        at = find_text(&out.shapes, "CSV");
        out.drop_without_applying_deltas();
    }
    let at = at.expect("a CSV button");
    assert!(!app.aircraft_online.export.busy());
    let button = |pressed| egui::Event::PointerButton {
        pos: at,
        button: egui::PointerButton::Primary,
        pressed,
        modifiers: egui::Modifiers::NONE,
    };
    t += 0.05;
    frame(&ctx, &mut app, t, vec![egui::Event::PointerMoved(at), button(true)])
        .drop_without_applying_deltas();
    t += 0.05;
    frame(&ctx, &mut app, t, vec![button(false)]).drop_without_applying_deltas();
    assert!(app.aircraft_online.export.busy(), "the press started an export");
}

/// The relay's cards held this session are in the file made here.
#[test]
fn what_the_relay_said_is_in_the_file_made_here() {
    use super::aircraft_online::Answer;
    use sdroxide_aircraft_online::{AircraftInfo, Info, Status};
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app) = app();
    let info = Info {
        aircraft: Some(AircraftInfo {
            type_name: Some("747-8".into()),
            manufacturer: Some("Boeing".into()),
            operator: Some("KLM".into()),
            registration: Some("PH-BKA".into()),
        }),
        fetched_at: Some(crate::time::now_unix()),
        ..Info::bare(Status::Ok)
    };
    app.aircraft_online.store.deliver(
        0x4840D6,
        Answer::Info(Box::new(info)),
        crate::time::now_unix(),
    );
    app.aircraft_online.export.start(Format::Csv);
    let done = app.aircraft_export_poll(&ctx).expect("a file");
    let csv = String::from_utf8(done.bytes).unwrap();
    assert!(csv.contains("4840D6,,PH-BKA,,Boeing 747-8,KLM,"), "{csv}");
    assert!(csv.trim_end().ends_with(",online"), "{csv}");
}
