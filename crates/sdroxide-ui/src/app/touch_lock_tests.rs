//! The anti-touch lock driven through a whole app frame, the way a finger
//! drives it: the input goes through the app's own `raw_input_hook`, so what is
//! tested is the wiring as well as the filter.

use std::cell::RefCell;
use std::rc::Rc;

use eframe::App;
use eframe::egui;
use sdroxide_types::{Command, RadioController, RadioEvent};

use super::SdroxideApp;

/// Keeps every command the app sends.
struct Sent(Rc<RefCell<Vec<Command>>>);

impl RadioController for Sent {
    fn send(&mut self, cmd: Command) {
        self.0.borrow_mut().push(cmd);
    }
    fn poll_event(&mut self) -> Option<RadioEvent> {
        None
    }
}

struct Rig {
    ctx: egui::Context,
    app: SdroxideApp,
    sent: Rc<RefCell<Vec<Command>>>,
    t: f64,
    size: egui::Vec2,
}

impl Rig {
    fn frame(&mut self, events: Vec<egui::Event>, dt: f64) -> egui::FullOutput {
        self.t += dt;
        let mut input = egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, self.size)),
            time: Some(self.t),
            events,
            ..Default::default()
        };
        self.app.raw_input_hook(&self.ctx, &mut input);
        let app = &mut self.app;
        let out = self.ctx.run_ui(input, |ui| app.ui(ui, &mut eframe::Frame::_new_kittest()));
        out
    }

    fn button(&mut self, at: egui::Pos2, pressed: bool) -> egui::Event {
        egui::Event::PointerButton {
            pos: at,
            button: egui::PointerButton::Primary,
            pressed,
            modifiers: egui::Modifiers::NONE,
        }
    }

    /// A tap: down and up a frame apart, the way a touch screen delivers one.
    fn tap(&mut self, at: egui::Pos2) {
        let down = self.button(at, true);
        self.frame(vec![egui::Event::PointerMoved(at), down], 0.05).drop_without_applying_deltas();
        let up = self.button(at, false);
        self.frame(vec![up], 0.05).drop_without_applying_deltas();
        self.frame(vec![egui::Event::PointerGone], 0.05).drop_without_applying_deltas();
    }

    fn retunes(&self) -> usize {
        self.sent.borrow().iter().filter(|c| matches!(c, Command::SetVfo { .. })).count()
    }
}

/// Where a label is drawn, by its text.
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

/// The request: once locked, a finger on the screen must not move the dial —
/// and a 3 s hold on the padlock, and only that, gives the screen back.
#[test]
fn a_locked_screen_does_not_tune_and_a_three_second_hold_unlocks_it() {
    let _guard = crate::multi::frame_test_lock();
    let dir = std::env::temp_dir().join(format!("sdroxide-touch-lock-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };

    let sent = Rc::new(RefCell::new(Vec::new()));
    let ctx = egui::Context::default();
    let app = SdroxideApp::new_tab(&ctx, None, None, Box::new(Sent(sent.clone())), 0, true);
    // A tablet, the tier the padlock was asked for.
    let size = egui::vec2(1000.0, 900.0);
    assert_eq!(
        crate::layout::tier_for(size, sdroxide_types::LayoutMode::Auto),
        crate::layout::Tier::Tablet
    );
    let mut rig = Rig { ctx: ctx.clone(), app, sent, t: 0.0, size };

    // A few frames for the layout to settle: the first passes size things.
    for _ in 0..3 {
        rig.frame(Vec::new(), 0.05).drop_without_applying_deltas();
    }
    let out = rig.frame(Vec::new(), 0.05);
    let plus = find_text(&out.shapes, "+").expect("the tune-step row's + is on the strip");
    out.drop_without_applying_deltas();
    let lock = crate::touch_lock::padlock_rects(&ctx)[0].center();

    // Unlocked, + tunes: the control works, so the rest of the test means something.
    rig.tap(plus);
    assert_eq!(rig.retunes(), 1, "+ did not tune on an unlocked screen");

    rig.tap(lock);
    assert!(crate::touch_lock::is_locked(&ctx), "a tap on the padlock did not lock");
    rig.tap(plus);
    assert_eq!(rig.retunes(), 1, "+ tuned through the lock");

    // A hold that lifts short of 3 s leaves the screen locked.
    let down = rig.button(lock, true);
    rig.frame(vec![egui::Event::PointerMoved(lock), down], 0.05).drop_without_applying_deltas();
    for _ in 0..10 {
        rig.frame(Vec::new(), 0.2).drop_without_applying_deltas();
    }
    let up = rig.button(lock, false);
    rig.frame(vec![up], 0.05).drop_without_applying_deltas();
    assert!(crate::touch_lock::is_locked(&ctx), "a 2 s hold unlocked");

    // The whole 3 s does.
    let down = rig.button(lock, true);
    rig.frame(vec![egui::Event::PointerMoved(lock), down], 0.05).drop_without_applying_deltas();
    for _ in 0..16 {
        rig.frame(Vec::new(), 0.2).drop_without_applying_deltas();
    }
    assert!(!crate::touch_lock::is_locked(&ctx), "a 3 s hold did not unlock");
    // ...and the release that ends it does not lock again.
    let up = rig.button(lock, false);
    rig.frame(vec![up], 0.05).drop_without_applying_deltas();
    rig.frame(vec![egui::Event::PointerGone], 0.05).drop_without_applying_deltas();
    assert!(!crate::touch_lock::is_locked(&ctx), "lifting off the hold locked again");

    rig.tap(plus);
    assert_eq!(rig.retunes(), 2, "+ did not tune once unlocked");
}

/// The request beside the padlock: a touch operator recalls a memory without
/// digging through a menu. On every touch strip — the stacked tablet, the
/// short tablet's one row and the phone — MEM sits on the row just before the
/// padlock (after SYS where the strip has one), and a tap on it opens the
/// memory channels.
#[test]
fn mem_sits_before_the_padlock_and_opens_the_memories() {
    let _guard = crate::multi::frame_test_lock();
    let dir = std::env::temp_dir().join(format!("sdroxide-mem-chip-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };

    // Tablet (stacked), tablet (short: one row), phone.
    for (size, has_sys) in [
        (egui::vec2(1000.0, 900.0), true),
        (egui::vec2(1280.0, 720.0), true),
        (egui::vec2(412.0, 915.0), false),
    ] {
        // Each strip starts as a first run: an open window is remembered for the
        // next start, and the strip before this one left the memories open.
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let sent = Rc::new(RefCell::new(Vec::new()));
        let ctx = egui::Context::default();
        let app = SdroxideApp::new_tab(&ctx, None, None, Box::new(Sent(sent.clone())), 0, true);
        let mut rig = Rig { ctx: ctx.clone(), app, sent, t: 0.0, size };
        for _ in 0..3 {
            rig.frame(Vec::new(), 0.05).drop_without_applying_deltas();
        }
        let out = rig.frame(Vec::new(), 0.05);
        let mem =
            find_text(&out.shapes, "MEM").unwrap_or_else(|| panic!("no MEM chip at {size:?}"));
        let sys = find_text(&out.shapes, "SYS");
        out.drop_without_applying_deltas();
        let lock = crate::touch_lock::padlock_rects(&ctx)[0];

        assert!(
            (mem.y - lock.center().y).abs() < lock.height() / 2.0,
            "MEM is not on the padlock's row at {size:?}"
        );
        assert!(mem.x < lock.left(), "MEM is not before the padlock at {size:?}");
        if has_sys {
            let sys = sys.unwrap_or_else(|| panic!("no SYS chip at {size:?}"));
            assert!(sys.x < mem.x, "MEM is not after SYS at {size:?}");
        }

        assert!(!rig.app.show_memories);
        rig.tap(mem);
        assert!(rig.app.show_memories, "a tap on MEM did not open the memories at {size:?}");
    }
}

/// The report (Kevin, phone and tablet): a 3 s hold unlocks, and the padlock
/// then takes the same finger — still on the glass, or seen again by the
/// browser as a fresh press — as a tap and locks again before it can lift.
/// After an unlock the padlock rests for 10 s: a press then is not a lock.
#[test]
fn an_unlock_is_not_undone_by_the_finger_that_made_it() {
    let _guard = crate::multi::frame_test_lock();
    let dir = std::env::temp_dir().join(format!("sdroxide-touch-relock-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };

    let sent = Rc::new(RefCell::new(Vec::new()));
    let ctx = egui::Context::default();
    let app = SdroxideApp::new_tab(&ctx, None, None, Box::new(Sent(sent.clone())), 0, true);
    let mut rig = Rig { ctx: ctx.clone(), app, sent, t: 0.0, size: egui::vec2(1000.0, 900.0) };
    for _ in 0..4 {
        rig.frame(Vec::new(), 0.05).drop_without_applying_deltas();
    }
    let lock = crate::touch_lock::padlock_rects(&ctx)[0].center();

    rig.tap(lock);
    assert!(crate::touch_lock::is_locked(&ctx), "a tap on the padlock did not lock");

    // Hold for the 3 s: unlocked.
    let down = rig.button(lock, true);
    rig.frame(vec![egui::Event::PointerMoved(lock), down], 0.05).drop_without_applying_deltas();
    for _ in 0..16 {
        rig.frame(Vec::new(), 0.2).drop_without_applying_deltas();
    }
    assert!(!crate::touch_lock::is_locked(&ctx), "a 3 s hold did not unlock");
    let up = rig.button(lock, false);
    rig.frame(vec![up], 0.05).drop_without_applying_deltas();

    // The same finger, seen again as a press and a release on the padlock
    // within the pause: the screen stays unlocked.
    for _ in 0..3 {
        rig.tap(lock);
        assert!(!crate::touch_lock::is_locked(&ctx), "the padlock locked again under the finger");
        rig.frame(Vec::new(), 1.0).drop_without_applying_deltas();
    }

    // Past the 10 s pause (about 3.5 s have gone by above), a tap locks as it
    // always did.
    for _ in 0..8 {
        rig.frame(Vec::new(), 1.0).drop_without_applying_deltas();
    }
    rig.tap(lock);
    assert!(crate::touch_lock::is_locked(&ctx), "a tap after the pause did not lock");
}

/// A tap on MEM opens the memory channels and they stay open. The window's
/// open state lives in two places — the `show_memories` flag a chip flips and
/// the persisted `tool_windows_open` bit the Settings → Detachable menu flips —
/// and a frame that copied the bit over the flag undid the tap in the same frame
/// it was made. Nothing here sets a bit by hand: only the chip and the frames.
#[test]
fn a_tap_on_mem_leaves_the_memories_open_across_frames() {
    let _guard = crate::multi::frame_test_lock();
    let dir = std::env::temp_dir().join(format!("sdroxide-mem-stays-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };

    let size = egui::vec2(1000.0, 900.0);
    let sent = Rc::new(RefCell::new(Vec::new()));
    let ctx = egui::Context::default();
    let app = SdroxideApp::new_tab(&ctx, None, None, Box::new(Sent(sent.clone())), 0, true);
    let mut rig = Rig { ctx: ctx.clone(), app, sent, t: 0.0, size };
    for _ in 0..3 {
        rig.frame(Vec::new(), 0.05).drop_without_applying_deltas();
    }
    let out = rig.frame(Vec::new(), 0.05);
    let mem = find_text(&out.shapes, "MEM").expect("a MEM chip");
    out.drop_without_applying_deltas();
    assert!(find_text(&rig.frame(Vec::new(), 0.05).shapes, "Memories").is_none());

    rig.tap(mem);
    // Left alone for a second, the window is still there, drawn.
    let mut drawn = false;
    for _ in 0..20 {
        let out = rig.frame(Vec::new(), 0.05);
        drawn = find_text(&out.shapes, "Memories").is_some();
        out.drop_without_applying_deltas();
    }
    assert!(rig.app.show_memories, "the memories closed after a tap on MEM");
    assert!(drawn, "the Memories window is not on screen after a tap on MEM");
    assert!(
        rig.app.ui_settings.tool_windows_open & super::frame::TW_MEMORIES != 0,
        "the open state was not kept for the next start"
    );
}

/// EXTRAS then AIRCRAFT, by real taps: the window opens, is still there a second
/// later, and the open state is written for the next start. Same fault as MEM:
/// the mask sync put the flag back in the frame the chip flipped it. No bit is
/// set by hand.
#[test]
fn a_tap_on_aircraft_in_extras_leaves_the_window_open() {
    let _guard = crate::multi::frame_test_lock();
    let dir = std::env::temp_dir().join(format!("sdroxide-aircraft-stays-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };

    let size = egui::vec2(1600.0, 900.0);
    let sent = Rc::new(RefCell::new(Vec::new()));
    let ctx = egui::Context::default();
    let app = SdroxideApp::new_tab(&ctx, None, None, Box::new(Sent(sent.clone())), 0, true);
    let mut rig = Rig { ctx: ctx.clone(), app, sent, t: 0.0, size };
    for _ in 0..3 {
        rig.frame(Vec::new(), 0.05).drop_without_applying_deltas();
    }
    let out = rig.frame(Vec::new(), 0.05);
    let extras = find_text(&out.shapes, "EXTRAS").expect("an EXTRAS chip");
    out.drop_without_applying_deltas();
    rig.tap(extras);
    for _ in 0..3 {
        rig.frame(Vec::new(), 0.05).drop_without_applying_deltas();
    }
    assert!(rig.app.show_extras, "EXTRAS did not open");
    let out = rig.frame(Vec::new(), 0.05);
    let air = find_text(&out.shapes, "AIRCRAFT").expect("an AIRCRAFT chip in EXTRAS");
    out.drop_without_applying_deltas();

    rig.tap(air);
    for _ in 0..20 {
        rig.frame(Vec::new(), 0.05).drop_without_applying_deltas();
    }
    assert!(rig.app.show_aircraft, "the AIRCRAFT window closed after the tap");
    assert!(
        rig.app.ui_settings.tool_windows_open & super::frame::TW_AIRCRAFT != 0,
        "the open state was not kept for the next start"
    );
}

/// EXTRAS then ANTENNA, by real taps: the calculator window opens and renders
/// the shapes.
#[test]
fn a_tap_on_antenna_in_extras_opens_the_calculator() {
    let _guard = crate::multi::frame_test_lock();
    let dir =
        std::env::temp_dir().join(format!("sdroxide-antenna-opens-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };

    let size = egui::vec2(1600.0, 900.0);
    let sent = Rc::new(RefCell::new(Vec::new()));
    let ctx = egui::Context::default();
    let app = SdroxideApp::new_tab(&ctx, None, None, Box::new(Sent(sent.clone())), 0, true);
    let mut rig = Rig { ctx: ctx.clone(), app, sent, t: 0.0, size };
    for _ in 0..3 {
        rig.frame(Vec::new(), 0.05).drop_without_applying_deltas();
    }
    let out = rig.frame(Vec::new(), 0.05);
    let extras = find_text(&out.shapes, "EXTRAS").expect("an EXTRAS chip");
    out.drop_without_applying_deltas();
    rig.tap(extras);
    for _ in 0..3 {
        rig.frame(Vec::new(), 0.05).drop_without_applying_deltas();
    }
    let out = rig.frame(Vec::new(), 0.05);
    let ant = find_text(&out.shapes, "ANTENNA").expect("an ANTENNA chip in EXTRAS");
    out.drop_without_applying_deltas();
    rig.tap(ant);
    for _ in 0..3 {
        rig.frame(Vec::new(), 0.05).drop_without_applying_deltas();
    }
    assert!(rig.app.show_antenna, "the ANTENNA window did not open");
    let out = rig.frame(Vec::new(), 0.05);
    find_text(&out.shapes, "Half-wave dipole").expect("the dipole row rendered");
    out.drop_without_applying_deltas();
}
