//! Radio Classic through a whole app: docked under the waterfall when it is
//! switched on in a voice mode, in a window when the Detachable page says so or
//! when the slot is taken, and not there at all when it is off.

use eframe::App;
use eframe::egui;
use sdroxide_types::{Mode, RadioController, RadioEvent};

use super::SdroxideApp;
use super::frame::TW_RADIO_CLASSIC;

struct NoStation;

impl RadioController for NoStation {
    fn send(&mut self, _cmd: sdroxide_types::Command) {}
    fn poll_event(&mut self) -> Option<RadioEvent> {
        None
    }
}

fn texts(shapes: &[egui::epaint::ClippedShape]) -> Vec<String> {
    fn walk(s: &egui::Shape, out: &mut Vec<String>) {
        match s {
            egui::Shape::Text(t) => out.push(t.galley.text().to_string()),
            egui::Shape::Vec(v) => v.iter().for_each(|s| walk(s, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    shapes.iter().for_each(|c| walk(&c.shape, &mut out));
    out
}

fn app(mode: Mode) -> (egui::Context, SdroxideApp) {
    let dir = std::env::temp_dir().join(format!("sdroxide-rc-frame-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };
    let ctx = egui::Context::default();
    let mut app = SdroxideApp::new_tab(&ctx, None, None, Box::new(NoStation), 0, true);
    app.state.rx[0].mode = mode;
    (ctx, app)
}

/// Run a few frames at 1600x900 and return the text drawn in the last one.
fn run(ctx: &egui::Context, app: &mut SdroxideApp) -> Vec<String> {
    let base = ctx.input(|i| i.time);
    let mut last = Vec::new();
    for i in 1..6 {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1600.0, 900.0),
            )),
            time: Some(base + f64::from(i) * 0.1),
            ..Default::default()
        };
        let mut out = ctx.run_ui(input, |ui| app.ui(ui, &mut eframe::Frame::_new_kittest()));
        last = texts(&out.shapes);
        out.textures_delta.clear();
    }
    last
}

fn count(text: &[String], what: &str) -> usize {
    text.iter().filter(|l| l.as_str() == what).count()
}

#[test]
fn off_it_is_not_drawn() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app) = app(Mode::Wfm);
    let text = run(&ctx, &mut app);
    assert_eq!(count(&text, "RADIO CLASSIC"), 0, "{text:?}");
}

#[test]
fn switched_on_in_a_voice_mode_it_docks_under_the_waterfall() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app) = app(Mode::Wfm);
    app.ui_settings.radio_classic_on = true;
    let text = run(&ctx, &mut app);
    // The panel's own heading, once: docked, so no window title beside it.
    assert_eq!(count(&text, "RADIO CLASSIC"), 1, "{text:?}");
    assert!(count(&text, "BROADCAST") == 1 && count(&text, "WFM") >= 1, "{text:?}");
    assert!(!app.show_radio_classic, "docking must not open the window");
}

#[test]
fn the_detachable_pages_switch_opens_it_as_a_window_and_keeps_its_bit() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app) = app(Mode::Wfm);
    app.ui_settings.radio_classic_on = true;
    app.ui_settings.tool_windows_open |= TW_RADIO_CLASSIC;
    let text = run(&ctx, &mut app);
    assert!(app.show_radio_classic, "the switch did not reach the window flag");
    assert_ne!(app.ui_settings.tool_windows_open & TW_RADIO_CLASSIC, 0, "the bit was lost");
    // Window title and panel heading, and not a second copy docked under the
    // waterfall.
    assert_eq!(count(&text, "RADIO CLASSIC"), 2, "{text:?}");
}

#[test]
fn where_a_panel_owns_the_slot_it_opens_as_a_window_instead_of_doing_nothing() {
    let _guard = crate::multi::frame_test_lock();
    // CW has a panel of its own under the waterfall. (The digital modes no
    // longer get here: RADIO yields to them, see `choosing_another_mode_…`.)
    let (ctx, mut app) = app(Mode::Cw);
    app.ui_settings.radio_classic_on = true;
    let text = run(&ctx, &mut app);
    assert_eq!(count(&text, "RADIO CLASSIC"), 2, "{text:?}");
    assert!(!app.show_radio_classic, "the window is forced, not detached");
}

struct Recorder(std::sync::Arc<std::sync::Mutex<Vec<sdroxide_types::Command>>>);

impl RadioController for Recorder {
    fn send(&mut self, cmd: sdroxide_types::Command) {
        self.0.lock().unwrap().push(cmd);
    }
    fn poll_event(&mut self) -> Option<RadioEvent> {
        None
    }
}

fn wants(sent: &std::sync::Mutex<Vec<sdroxide_types::Command>>) -> usize {
    let sent = sent.lock().unwrap();
    sent.iter().filter(|c| matches!(c, sdroxide_types::Command::WantAudioLevel)).count()
}

/// The station sends the level only to a screen that asked for it: closed, the
/// panel asks for nothing; open, it asks once and renews, not every frame.
#[test]
fn the_panel_asks_for_the_audio_level_only_while_it_is_open() {
    let _guard = crate::multi::frame_test_lock();
    let sent = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
    let (ctx, mut app) = app(Mode::Wfm);
    app.ctrl = Box::new(Recorder(sent.clone()));
    run(&ctx, &mut app);
    assert_eq!(wants(&sent), 0, "a closed panel must not ask for the stream");
    app.ui_settings.radio_classic_on = true;
    run(&ctx, &mut app);
    assert_eq!(wants(&sent), 1, "five frames in well under a second: one lease");
}

/// Where a label was drawn, as the middle of its text — the lowest one on
/// screen, which is the panel's and not the top bar's.
fn centre_of(shapes: &[egui::epaint::ClippedShape], what: &str) -> Option<egui::Pos2> {
    text_rects(shapes)
        .into_iter()
        .filter(|(t, _)| t == what)
        .map(|(_, r)| r.center())
        .max_by(|a, b| a.y.total_cmp(&b.y))
}

fn one_frame(
    ctx: &egui::Context,
    app: &mut SdroxideApp,
    time: f64,
    events: Vec<egui::Event>,
) -> Vec<egui::epaint::ClippedShape> {
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(1600.0, 900.0))),
        time: Some(time),
        events,
        ..Default::default()
    };
    let mut out = ctx.run_ui(input, |ui| app.ui(ui, &mut eframe::Frame::_new_kittest()));
    out.textures_delta.clear();
    out.shapes
}

/// Settle the layout, then click the middle of the first label that reads
/// `what`; returns false when no such label is on screen.
fn click(ctx: &egui::Context, app: &mut SdroxideApp, what: &str) -> bool {
    run(ctx, app);
    let base = ctx.input(|i| i.time);
    let shapes = one_frame(ctx, app, base + 1.0, vec![]);
    let Some(at) = centre_of(&shapes, what) else { return false };
    let button = egui::PointerButton::Primary;
    let mods = egui::Modifiers::NONE;
    one_frame(ctx, app, base + 1.1, vec![egui::Event::PointerMoved(at)]);
    one_frame(
        ctx,
        app,
        base + 1.2,
        vec![egui::Event::PointerButton { pos: at, button, pressed: true, modifiers: mods }],
    );
    one_frame(
        ctx,
        app,
        base + 1.3,
        vec![egui::Event::PointerButton { pos: at, button, pressed: false, modifiers: mods }],
    );
    one_frame(ctx, app, base + 1.4, vec![]);
    true
}

/// PROGRAMS opens the PROGRAMS window — the one the top bar opens — not a
/// little list of its own.
#[test]
fn the_programs_chip_opens_the_programs_window() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app) = app(Mode::Wfm);
    app.ui_settings.radio_classic_on = true;
    assert!(!app.schedule.show);
    assert!(click(&ctx, &mut app, "PROGRAMS"), "no PROGRAMS chip on the panel");
    assert!(app.schedule.show, "the click did not open the PROGRAMS window");
}

/// Every text shape on screen, as `(text, rect)`.
fn text_rects(shapes: &[egui::epaint::ClippedShape]) -> Vec<(String, egui::Rect)> {
    fn walk(s: &egui::Shape, out: &mut Vec<(String, egui::Rect)>) {
        match s {
            egui::Shape::Text(t) => out.push((
                t.galley.text().to_string(),
                egui::Rect::from_min_size(t.pos, t.galley.size()),
            )),
            egui::Shape::Vec(v) => v.iter().for_each(|s| walk(s, out)),
            _ => {}
        }
    }
    let mut out = Vec::new();
    shapes.iter().for_each(|c| walk(&c.shape, &mut out));
    out
}

/// The big frequency sits in the middle of its column: the hint line under it
/// is centred on the column, and the digits must share its centre.
#[test]
fn the_big_frequency_is_centred_in_its_column() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app) = app(Mode::Wfm);
    app.ui_settings.radio_classic_on = true;
    run(&ctx, &mut app);
    let shapes = one_frame(&ctx, &mut app, 2.0, vec![]);
    let texts = text_rects(&shapes);
    let hint = texts
        .iter()
        .find(|(t, _)| t.starts_with("wheel on a digit"))
        .map(|(_, r)| *r)
        .expect("no hint line under the frequency");
    let column = hint.center().x;
    let digits: Vec<egui::Rect> = texts
        .iter()
        .filter(|(t, r)| {
            (t.chars().count() == 1 && t.chars().all(|c| c.is_ascii_digit() || c == '.')
                || t.trim() == "Hz")
                && r.bottom() <= hint.top() + 2.0
                && r.top() > hint.top() - 90.0
                && (r.center().x - column).abs() < 450.0
                && r.height() > 14.0
        })
        .map(|(_, r)| *r)
        .collect();
    assert!(digits.len() >= 9, "found {} digits", digits.len());
    let span = digits.iter().fold(digits[0], |a, r| a.union(*r));
    let off = span.center().x - hint.center().x;
    assert!(off.abs() <= 1.5, "the frequency is {off} px off the column's centre {column}");
}

/// A station name longer than the column keeps its size on one line, and what it carries in
/// brackets goes under it in small type, centred on the column.
#[test]
fn a_long_station_name_keeps_its_size_and_the_bracket_goes_small_under_it() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app) = app(Mode::Wfm);
    app.ui_settings.radio_classic_on = true;
    app.state.vfo_a_hz = 89_500_000.0;
    app.memories = vec![sdroxide_types::MemoryChannel {
        id: 1040,
        name: "Belgien Klara (Sint-Pieters-Leeuw Nahe Brüssel)".into(),
        freq_hz: 89_500_000.0,
        mode: Mode::Wfm,
        filter_lo: -96_000.0,
        filter_hi: 96_000.0,
        folder: None,
        rtty: None,
        repeater: None,
        antenna: None,
        color: None,
    }];
    run(&ctx, &mut app);
    let shapes = one_frame(&ctx, &mut app, 2.0, vec![]);
    let hint = text_rects(&shapes)
        .into_iter()
        .find(|(t, _)| t.starts_with("wheel on a digit"))
        .map(|(_, r)| r)
        .expect("no hint line");
    let mut all = Vec::new();
    fn walk(s: &egui::Shape, out: &mut Vec<(String, egui::Rect)>) {
        match s {
            egui::Shape::Text(t) => {
                out.push((t.galley.text().to_string(), t.galley.rect.translate(t.pos.to_vec2())))
            }
            egui::Shape::Vec(v) => v.iter().for_each(|s| walk(s, out)),
            _ => {}
        }
    }
    shapes.iter().for_each(|c| walk(&c.shape, &mut all));
    let main = all
        .iter()
        .find(|(t, r)| t.starts_with("BELGIEN") && r.height() > 25.0)
        .expect("the big name is not drawn");
    let note = all
        .iter()
        .find(|(t, _)| t.starts_with("(Sint-Pieters"))
        .expect("the bracket is not drawn on its own");
    assert!(!main.0.contains("SINT"), "the bracket is still in the big name");
    // The header above the waterfall draws the name in capitals: without the bracket.
    assert!(
        !all.iter().any(|(t, _)| t.contains("SINT")),
        "the bracket is also in the header above the waterfall"
    );
    assert!(note.1.top() >= main.1.bottom() - 1.0, "the bracket is not under the name");
    assert!(note.1.height() < main.1.height() * 0.7, "the bracket is not in small type");
    for (what, r) in [("name", main.1), ("bracket", note.1)] {
        assert!((r.center().x - hint.center().x).abs() <= 1.5, "the {what} is off the centre");
    }
}

/// The Wideband row of the band menu, drawn on its own.
fn wideband_row(
    app: &mut SdroxideApp,
    ctx: &egui::Context,
    radio: &mut super::top_bar::RadioChip,
) -> Vec<String> {
    let mut show_ism = false;
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(800.0, 600.0))),
        ..Default::default()
    };
    let mut out = ctx.run_ui(input, |ui| {
        super::top_bar::wideband_chip_grid(
            ui,
            Mode::Wfm,
            sdroxide_types::Band::Gen,
            &app.state,
            false,
            &mut show_ism,
            radio,
            &mut Vec::new(),
        );
    });
    out.textures_delta.clear();
    texts(&out.shapes)
}

/// WIDEBAND has a RADIO chip to the right of ISM, and it is lit while RADIO
/// CLASSIC is on — the same state EXTRAS' chip reads.
#[test]
fn the_wideband_row_has_a_radio_chip_after_ism() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app) = app(Mode::Wfm);
    let mut radio = super::top_bar::RadioChip::new(app.radio_classic_active());
    let row = wideband_row(&mut app, &ctx, &mut radio);
    let ism = row.iter().position(|t| t == "ISM").expect("no ISM chip");
    assert_eq!(row.get(ism + 1).map(String::as_str), Some("RADIO"), "{row:?}");
}

/// EXTRAS' chip and the Wideband row's flip the same switch.
#[test]
fn both_radio_chips_flip_the_same_switch() {
    let _guard = crate::multi::frame_test_lock();
    let (_ctx, mut app) = app(Mode::Wfm);
    assert!(!app.radio_classic_active());
    app.toggle_radio_classic();
    assert!(app.radio_classic_active() && app.ui_settings.radio_classic_on);
    app.toggle_radio_classic();
    assert!(!app.radio_classic_active());
}

fn spectrum(seq: u32) -> std::sync::Arc<sdroxide_types::SpectrumFrame> {
    std::sync::Arc::new(sdroxide_types::SpectrumFrame {
        seq,
        center_hz: 100e6,
        span_hz: 2e6,
        db_floor: -120.0,
        db_ceil: 0.0,
        bins: (0..512).map(|i| (i % 256) as u8).collect(),
        rows: Vec::new(),
        rows_clocked: false,
    })
}

/// Feed the panel `n` spectrum frames, one per screen frame, and say how many
/// waterfall rows it kept.
fn rows_after(n: u32) -> usize {
    let (ctx, mut app) = app(Mode::Wfm);
    app.ui_settings.radio_classic_on = true;
    run(&ctx, &mut app);
    app.radio_classic.wf.clear();
    for i in 0..n {
        app.frame = Some(spectrum(1000 + i));
        one_frame(&ctx, &mut app, 10.0 + f64::from(i) * 0.05, vec![]);
    }
    app.radio_classic.wf.len()
}

/// The little waterfall scrolls one and a half times as fast as it did at one
/// row for two spectrum frames — three rows for every four — and keeps ten
/// times as many rows as it first did.
#[test]
fn the_scope_waterfall_runs_three_rows_in_four_and_keeps_four_hundred() {
    let _guard = crate::multi::frame_test_lock();
    assert_eq!(rows_after(20), 15, "twenty frames should leave fifteen rows");
    assert_eq!(rows_after(1200), 400, "the waterfall keeps 400 rows");
}

type Sent = std::sync::Arc<std::sync::Mutex<Vec<sdroxide_types::Command>>>;

/// A panel on, a recorder behind it, and the receiver set to `mode`.
fn recorded(mode: Mode) -> (egui::Context, SdroxideApp, Sent) {
    let sent: Sent = std::sync::Arc::default();
    let (ctx, mut app) = app(mode);
    app.ctrl = Box::new(Recorder(sent.clone()));
    app.ui_settings.radio_classic_on = true;
    (ctx, app, sent)
}

/// The four boxes under the S-meter act: BW takes the next filter of the mode.
#[test]
fn the_bw_box_takes_the_next_filter_of_the_mode() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app, sent) = recorded(Mode::Usb);
    app.state.rx[0].filter_lo = 200.0;
    app.state.rx[0].filter_hi = 2600.0; // the 2.4k preset
    assert!(click(&ctx, &mut app, "BW"), "no BW box");
    let sent = sent.lock().unwrap();
    let got = sent.iter().find_map(|c| match c {
        sdroxide_types::Command::SetFilter { rx: sdroxide_types::RxId::Main, lo, hi } => {
            Some((*lo, *hi))
        }
        _ => None,
    });
    assert_eq!(got, Some((150.0, 2850.0)), "{sent:?}");
}

/// STEP takes the next tuning step, the one the top bar's step chip takes.
#[test]
fn the_step_box_takes_the_next_tuning_step() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app, _sent) = recorded(Mode::Wfm);
    let before = app.ui_settings.tune_step_hz;
    let next = app.ui_settings.next_tune_step();
    assert_ne!(before, next);
    assert!(click(&ctx, &mut app, "STEP"), "no STEP box");
    assert_eq!(app.ui_settings.tune_step_hz, next);
}

/// AGC cycles Off → Slow → Med → Fast.
#[test]
fn the_agc_box_cycles_the_agc() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app, sent) = recorded(Mode::Wfm);
    app.state.rx[0].agc = sdroxide_types::AgcMode::Med;
    assert!(click(&ctx, &mut app, "AGC"), "no AGC box");
    let sent = sent.lock().unwrap();
    assert!(
        sent.iter().any(|c| matches!(
            c,
            sdroxide_types::Command::SetAgc {
                rx: sdroxide_types::RxId::Main,
                agc: sdroxide_types::AgcMode::Fast
            }
        )),
        "{sent:?}"
    );
}

/// SQL: a click closes the gate on the last threshold, the next one opens it.
#[test]
fn the_sql_box_toggles_the_squelch_between_open_and_the_last_threshold() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app, sent) = recorded(Mode::Wfm);
    app.state.rx[0].squelch_db = sdroxide_types::SQUELCH_OPEN_DB;
    assert!(click(&ctx, &mut app, "SQL"), "no SQL box");
    let db = |sent: &Sent| {
        sent.lock().unwrap().iter().rev().find_map(|c| match c {
            sdroxide_types::Command::SetSquelch { rx: sdroxide_types::RxId::Main, db } => Some(*db),
            _ => None,
        })
    };
    let set = db(&sent).expect("the click set no squelch");
    assert!(set > sdroxide_types::SQUELCH_OPEN_DB + 1.0, "still open: {set}");
    assert!(click(&ctx, &mut app, "SQL"));
    assert_eq!(db(&sent), Some(sdroxide_types::SQUELCH_OPEN_DB), "the second click did not open");
}

/// An app in `mode` with a recorder, RADIO still off, and a few frames run so
/// that switching it on afterwards is a real switch-on.
fn off_in(mode: Mode) -> (egui::Context, SdroxideApp, Sent) {
    let sent: Sent = std::sync::Arc::default();
    let (ctx, mut app) = app(mode);
    app.ctrl = Box::new(Recorder(sent.clone()));
    run(&ctx, &mut app);
    (ctx, app, sent)
}

fn vfo_sent(sent: &Sent) -> Option<f64> {
    sent.lock().unwrap().iter().rev().find_map(|c| match c {
        sdroxide_types::Command::SetVfo { hz, .. } => Some(*hz),
        _ => None,
    })
}

fn mode_sent(sent: &Sent) -> Option<Mode> {
    sent.lock().unwrap().iter().rev().find_map(|c| match c {
        sdroxide_types::Command::SetMode { rx: sdroxide_types::RxId::Main, mode } => Some(*mode),
        _ => None,
    })
}

/// The first time the panel is switched on it tunes 100 MHz in WFM.
#[test]
fn the_first_launch_tunes_100_mhz_in_wfm() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app, sent) = off_in(Mode::Usb);
    app.state.vfo_a_hz = 7_100_000.0;
    app.toggle_radio_classic();
    run(&ctx, &mut app);
    assert_eq!(vfo_sent(&sent), Some(100e6));
    assert_eq!(mode_sent(&sent), Some(Mode::Wfm));
}

/// After that it comes back to the last frequency it was listening on.
#[test]
fn the_next_launch_returns_to_the_last_frequency_listened() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app, sent) = off_in(Mode::Usb);
    app.ui_settings.radio_classic_last_hz = 1_215_000.0;
    app.ui_settings.radio_classic_last_mode = Some(Mode::Am);
    app.toggle_radio_classic();
    run(&ctx, &mut app);
    assert_eq!(vfo_sent(&sent), Some(1_215_000.0));
    assert_eq!(mode_sent(&sent), Some(Mode::Am));
}

/// While it is on, whatever the dial lands on (MEM, PROGRAMS, the wheel) is
/// what it will come back to.
#[test]
fn the_frequency_it_ends_on_is_the_one_it_remembers() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app, _sent) = off_in(Mode::Wfm);
    app.toggle_radio_classic();
    run(&ctx, &mut app);
    // The station answers; some time passes; the operator tunes elsewhere.
    for _ in 0..4 {
        run(&ctx, &mut app);
    }
    app.state.vfo_a_hz = 99_700_000.0;
    app.state.rx[0].mode = Mode::Wfm;
    for _ in 0..2 {
        run(&ctx, &mut app);
    }
    assert_eq!(app.ui_settings.radio_classic_last_hz, 99_700_000.0);
    assert_eq!(app.ui_settings.radio_classic_last_mode, Some(Mode::Wfm));
}

/// Radio Classic has the receiver to itself: from SSTV, the SSTV window is
/// closed, the second receiver and the scan and the ISM decoder are stopped,
/// and the dial goes straight to Radio Classic's frequency in a voice mode.
#[test]
fn switching_radio_on_from_sstv_takes_the_whole_receiver() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app, sent) = off_in(Mode::Sstv);
    app.show_sstv = true;
    app.show_ism = true;
    app.state.sub_rx_enabled = true;
    app.state.scan.running = true;
    app.state.ism.enabled = true;
    app.state.ism.families = 1;
    app.toggle_radio_classic();
    run(&ctx, &mut app);
    assert!(!app.show_sstv, "the SSTV window is still open");
    assert!(!app.show_ism, "the ISM window is still open");
    let sent = sent.lock().unwrap();
    let has = |f: &dyn Fn(&sdroxide_types::Command) -> bool| sent.iter().any(f);
    assert!(has(&|c| matches!(c, sdroxide_types::Command::SetSubRx(false))), "{sent:?}");
    assert!(has(&|c| matches!(c, sdroxide_types::Command::SetScanning(false))), "{sent:?}");
    assert!(
        has(&|c| matches!(c, sdroxide_types::Command::SetIsmConfig(s) if !s.any_enabled())),
        "{sent:?}"
    );
    assert!(has(&|c| matches!(c, sdroxide_types::Command::SetMode { mode: Mode::Wfm, .. })));
    assert!(has(&|c| matches!(c, sdroxide_types::Command::SetVfo { hz, .. } if *hz == 100e6)));
    drop(sent);
    assert!(app.radio_classic_active(), "it switched itself off again");
}

/// Choosing another mode afterwards hands the receiver back: RADIO goes off.
#[test]
fn choosing_another_mode_switches_radio_off() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app, _sent) = off_in(Mode::Wfm);
    app.toggle_radio_classic();
    for _ in 0..4 {
        run(&ctx, &mut app);
    }
    assert!(app.radio_classic_active());
    app.state.rx[0].mode = Mode::Ft8;
    for _ in 0..2 {
        run(&ctx, &mut app);
    }
    assert!(!app.radio_classic_active(), "RADIO stayed on over FT8");
}

fn fm_meters(stereo: bool) -> sdroxide_types::Meters {
    sdroxide_types::Meters {
        s_dbm: -67.0,
        pa_temp_c: None,
        adc_peak_dbfs: -30.0,
        adc_clip: 0.0,
        adc_overload: None,
        tx: None,
        stereo,
        tone: None,
        passband_dbfs: -40.0,
        puresignal: None,
    }
}

/// Two ears that read the same, over a spectrum with more in the odd bands.
fn level(db: f32) -> sdroxide_types::AudioLevel {
    sdroxide_types::AudioLevel {
        rms_db: [db, db],
        bands: (0..sdroxide_types::AUDIO_BANDS)
            .map(|i| if i % 2 == 0 { 200 } else { 120 })
            .collect(),
    }
}

/// Run frames with that level playing, and return the two needles.
fn needles(app: &mut SdroxideApp, ctx: &egui::Context) -> [f32; 2] {
    for i in 0..40 {
        app.audio_level = Some((crate::time::now_unix_f64(), level(-20.0)));
        one_frame(ctx, app, 30.0 + f64::from(i) * 0.033, vec![]);
    }
    app.radio_classic.vu.level
}

/// 64–108 MHz in WFM has a STEREO chip that is there whether or not the pilot
/// is locked, and says which.
#[test]
fn the_stereo_chip_is_on_the_fm_band_and_lit_only_with_a_pilot() {
    let _guard = crate::multi::frame_test_lock();
    for (stereo, hz, mode, shown) in [
        (false, 98e6, Mode::Wfm, true),
        (true, 98e6, Mode::Wfm, true),
        (true, 70e6, Mode::Wfm, true),
        (true, 150e6, Mode::Wfm, false),
        (true, 98e6, Mode::Am, false),
    ] {
        let (ctx, mut app, _sent) = recorded(mode);
        app.state.vfo_a_hz = hz;
        app.meters = Some(fm_meters(stereo));
        let text = run(&ctx, &mut app);
        assert_eq!(
            count(&text, "STEREO") == 1,
            shown,
            "{mode:?} {} MHz pilot={stereo}: {text:?}",
            hz / 1e6
        );
    }
}

/// Anything that is not a locked FM stereo broadcast is mono, and its two
/// needles are a false stereo — different, though the sound is the same.
#[test]
fn a_mono_signal_gets_a_false_stereo_vu() {
    let _guard = crate::multi::frame_test_lock();
    for (mode, hz, stereo) in [(Mode::Am, 7e6, false), (Mode::Wfm, 98e6, false)] {
        let (ctx, mut app, _sent) = recorded(mode);
        app.state.vfo_a_hz = hz;
        app.meters = Some(fm_meters(stereo));
        let [l, r] = needles(&mut app, &ctx);
        assert!((l - r).abs() > 0.5, "{mode:?} {hz}: the needles agree: {l} {r}");
    }
}

/// A real stereo broadcast is shown as it is: two equal ears stay equal.
#[test]
fn a_real_stereo_signal_is_not_touched() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app, _sent) = recorded(Mode::Wfm);
    app.state.vfo_a_hz = 98e6;
    app.meters = Some(fm_meters(true));
    let [l, r] = needles(&mut app, &ctx);
    assert!((l - r).abs() < 0.01, "a locked pilot was given a false stereo: {l} {r}");
}

/// Switching RADIO on in WFM puts the broadcast stereo decode back on if it
/// had been switched off.
#[test]
fn switching_radio_on_allows_the_wfm_stereo_decode() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app, sent) = off_in(Mode::Usb);
    app.state.rx[0].wfm_stereo = false;
    app.toggle_radio_classic();
    run(&ctx, &mut app);
    assert!(
        sent.lock().unwrap().iter().any(|c| matches!(
            c,
            sdroxide_types::Command::SetWfmStereo { rx: sdroxide_types::RxId::Main, on: true }
        )),
        "{:?}",
        sent.lock().unwrap()
    );
}

/// Renders the band menu's body, where the Wideband row now ends in RADIO, to a
/// PNG for looking at (`RC_WIDEBAND_OUT`, default `target/radio_wideband_preview.png`).
#[test]
#[ignore = "writes a picture to look at; not an assertion"]
fn wideband_preview() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app) = app(Mode::Wfm);
    app.ui_settings.radio_classic_on = true;
    let (w, h) = (620.0, 640.0);
    let mut tex = crate::test_raster::Textures::default();
    let mut last = None;
    for i in 0..6 {
        let input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(w, h))),
            time: Some(10.0 + f64::from(i) * 0.1),
            ..Default::default()
        };
        let mut full = ctx.run_ui(input, |ui| {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                app.band_menu_body(ui, &mut Vec::new());
            });
        });
        tex.apply(&full.textures_delta);
        full.textures_delta.clear();
        last = Some(full);
    }
    let img = crate::test_raster::rasterize(
        &ctx,
        last.unwrap(),
        &tex,
        egui::vec2(w, h),
        crate::theme::BG_DEEP(),
    );
    let out = std::env::var("RC_WIDEBAND_OUT").unwrap_or_else(|_| {
        format!("{}/../../target/radio_wideband_preview.png", env!("CARGO_MANIFEST_DIR"))
    });
    img.save(&out).expect("write the preview");
    println!("wrote {out}");
}

/// The arrow in the corner: docked it lifts the panel out, detached it docks it
/// back, and a panel that is a window only because the slot is taken wears none.
#[test]
fn the_corner_arrow_is_worn_only_where_it_has_somewhere_to_go() {
    use super::panels::radio_classic::{RadioClassicPlace as P, detach_icon};
    use crate::chrome::WindowIcon as I;
    assert_eq!(detach_icon(P::Docked, false, false), Some(I::Out));
    assert_eq!(detach_icon(P::Window, true, false), Some(I::Dock));
    assert_eq!(detach_icon(P::Window, false, false), None, "slot taken: nowhere to dock");
    assert_eq!(detach_icon(P::Docked, false, true), None, "no room to float on a phone");
    assert_eq!(detach_icon(P::Hidden, false, false), None);
}

/// With ST switched off the pilot still locks, but the sound is mono: the meter
/// is a false stereo and the lamp is dark — it was lit over a mono meter.
#[test]
fn st_off_with_a_pilot_is_mono_on_the_meter_and_dark_on_the_lamp() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app, _sent) = recorded(Mode::Wfm);
    app.state.vfo_a_hz = 98e6;
    app.state.rx[0].wfm_stereo = false;
    app.meters = Some(fm_meters(true));
    let [l, r] = needles(&mut app, &ctx);
    assert!((l - r).abs() > 0.5, "ST off but the meter reads stereo: {l} {r}");
}

/// The STEREO lamp is the ST switch: a click flips the decode.
#[test]
fn a_click_on_the_stereo_lamp_flips_the_decode() {
    let _guard = crate::multi::frame_test_lock();
    let (ctx, mut app, sent) = recorded(Mode::Wfm);
    app.state.vfo_a_hz = 98e6;
    app.state.rx[0].wfm_stereo = true;
    app.meters = Some(fm_meters(true));
    assert!(click(&ctx, &mut app, "STEREO"), "no STEREO lamp");
    assert!(sent.lock().unwrap().iter().any(|c| matches!(
        c,
        sdroxide_types::Command::SetWfmStereo { on: false, .. }
    )));
    assert!(!app.state.rx[0].wfm_stereo);
}

/// A PROGRAMS row tunes the VFO the receiver is on, and the dial shows it at
/// once — not VFO A whatever the receiver is on.
#[test]
fn a_programs_row_tunes_the_active_vfo() {
    let _guard = crate::multi::frame_test_lock();
    let (_ctx, mut app, _sent) = recorded(Mode::Wfm);
    app.state.active_vfo = sdroxide_types::Vfo::B;
    let station = sdroxide_types::BroadcastStation {
        name: "Radio X".into(),
        freq_khz: 6070.0,
        ..Default::default()
    };
    let mut cmds = Vec::new();
    app.tune_to_station(&station, &mut cmds);
    assert!(cmds.iter().any(|c| matches!(
        c,
        sdroxide_types::Command::SetVfo { vfo: sdroxide_types::Vfo::B, hz } if *hz == 6_070_000.0
    )));
    assert_eq!(app.state.vfo_b_hz, 6_070_000.0);
    assert_eq!(app.state.rx[0].mode, Mode::Am);
}
