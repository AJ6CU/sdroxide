//! The anti-touch lock: one padlock chip on the phone and tablet strips that
//! freezes the whole screen against a stray finger.
//!
//! A tablet held in two hands, or a phone in a pocket, touches the screen all
//! the time, and every touch on this program tunes something: the panadapter
//! retunes on a tap, the readout on a swipe, a chip changes the band. So the
//! lock does not grey controls out one by one — a list of what to grey would
//! always be one gesture short (the waterfall reads the pointer itself, not
//! through a button). It works one level down, on the input egui is handed:
//! while locked, [`filter`] drops every press, wheel, pinch, touch and key
//! before egui sees it, except a press that lands on the padlock itself. The
//! receiver, the audio and the link are untouched — the lock only stops input.
//!
//! Locking is a tap. Unlocking is a **3 s hold** on the padlock, with a ring
//! filling round it while the finger stays down; lifting early cancels it.
//! A brush of the screen cannot hold one spot for three seconds.
//!
//! After an unlock the padlock **rests for 10 s** before a tap locks again.
//! The finger that held it is still on the glass when the screen unlocks, and
//! a phone's browser can report it again as a fresh press (a long press is a
//! gesture of its own there); without the rest, that press was a tap on an
//! unlocked padlock and the screen locked again before the finger lifted.
//!
//! One exception, asked for by the operator: a control can declare itself
//! **usable while locked** ([`allow_while_locked`]) — the DAB panel's VU meter,
//! which is also its volume, so the sound can come down for the adverts
//! without unlocking. A press, a touch, a drag or the wheel over that control
//! gets through; anywhere else the screen stays locked.
//!
//! The state is per window and session-only: it lives in the context's
//! temporary memory, so every radio tab in the shell shares one lock and a
//! restart always starts unlocked. If the padlock stops being drawn — the
//! window turned into a desktop-sized one, which carries no padlock — the lock
//! lets go by itself, so the screen can never be left locked with no way out.

use crate::i18n::tr;

use eframe::egui::{
    self, Context, Event, Id, Pos2, Rect, Response, Sense, Shape, Stroke, TouchPhase, Ui,
};

/// How long the padlock must be held to unlock.
pub const UNLOCK_HOLD_S: f64 = 3.0;

/// How long the padlock rests after an unlock before a tap locks again.
pub const RELOCK_PAUSE_S: f64 = 10.0;

/// How long the "unlocked" flash, and the hint after a tap on a locked
/// padlock, stay on screen.
const FLASH_S: f64 = 0.8;
const HINT_S: f64 = 2.0;

#[derive(Clone, Default)]
struct State {
    locked: bool,
    /// Where the padlock was drawn, and on which pass. More than one in split
    /// view, where each radio's strip carries its own.
    rects: Vec<Rect>,
    rects_pass: u64,
    /// Controls that stay usable while locked, and the pass they were drawn
    /// on — gone with the control when it stops being drawn.
    allowed: Vec<Rect>,
    allowed_pass: u64,
    /// When the current hold on a locked padlock began.
    hold_since: Option<f64>,
    /// A hold that already unlocked: ignored until the finger lifts, so the
    /// same press cannot lock again.
    spent: bool,
    unlocked_at: Option<f64>,
    hint_at: Option<f64>,
}

fn id() -> Id {
    Id::new("sdroxide-touch-lock")
}

fn state(ctx: &Context) -> State {
    ctx.data(|d| d.get_temp::<State>(id())).unwrap_or_default()
}

fn store(ctx: &Context, s: State) {
    ctx.data_mut(|d| d.insert_temp(id(), s));
}

/// Whether the screen is locked.
#[cfg(test)]
pub fn is_locked(ctx: &Context) -> bool {
    state(ctx).locked
}

/// Whether the padlock was drawn recently enough to be the way out. A few
/// passes of slack: a discarded pass, or one the layout asked to redo, draws
/// nothing and must not let the lock go.
fn padlock_live(s: &State, pass: u64) -> bool {
    !s.rects.is_empty() && pass <= s.rects_pass + 3
}

/// Mark `rect` as a control that keeps working while the screen is locked.
/// Called every frame the control is drawn; it lapses when it stops being.
pub fn allow_while_locked(ctx: &Context, rect: Rect) {
    let mut s = state(ctx);
    let pass = ctx.cumulative_pass_nr();
    if s.allowed_pass != pass {
        s.allowed.clear();
        s.allowed_pass = pass;
    }
    s.allowed.push(rect);
    store(ctx, s);
}

/// Where a press may land on a locked screen at `pass`: the padlock, and the
/// controls allowed while locked if they were drawn recently — the same few
/// passes of slack the padlock gets.
fn open_rects(s: &State, pass: u64) -> Vec<Rect> {
    let mut open = s.rects.clone();
    if pass <= s.allowed_pass + 3 {
        open.extend(s.allowed.iter().copied());
    }
    open
}

/// Whether one input event reaches egui while the screen is locked.
///
/// Presses only on the padlock or a control allowed while locked; releases
/// always, so a button or key held when the lock fell is not left stuck down.
/// Movement is harmless and keeps hover alive. The wheel only over an allowed
/// control (`pointer` is where the pointer last was). Everything else that can
/// change a setting — a pinch, a touch elsewhere, a key, text, a paste — is
/// dropped.
fn keep_while_locked(event: &Event, open: &[Rect], pointer: Option<Pos2>) -> bool {
    let inside = |p: Pos2| open.iter().any(|r| r.contains(p));
    match event {
        Event::PointerButton { pos, pressed, .. } => !*pressed || inside(*pos),
        Event::Touch { phase, pos, .. } => {
            matches!(phase, TouchPhase::End | TouchPhase::Cancel) || inside(*pos)
        }
        Event::MouseWheel { .. } => pointer.is_some_and(inside),
        Event::Key { pressed, .. } => !*pressed,
        Event::PointerMoved(_)
        | Event::MouseMoved(_)
        | Event::PointerGone
        | Event::ModifiersChanged(_)
        | Event::WindowFocused(_)
        | Event::Screenshot { .. } => true,
        _ => false,
    }
}

/// Strip the input of everything a locked screen must not act on. Called from
/// the app's `raw_input_hook`, before egui turns the events into pointer and
/// keyboard state — removing them any later would be too late, since a
/// widget reads the state, not the events.
pub fn filter(ctx: &Context, raw: &mut egui::RawInput) {
    let mut s = state(ctx);
    if !s.locked {
        return;
    }
    if !padlock_live(&s, ctx.cumulative_pass_nr()) {
        // No padlock on screen, so no way to unlock: let go rather than trap.
        s.locked = false;
        s.hold_since = None;
        store(ctx, s);
        return;
    }
    let open = open_rects(&s, ctx.cumulative_pass_nr());
    let pointer = ctx.input(|i| i.pointer.latest_pos());
    raw.events.retain(|e| keep_while_locked(e, &open, pointer));
    raw.dropped_files.clear();
    raw.hovered_files.clear();
}

/// Whether the padlock is still resting after the unlock at `unlocked_at`.
fn relock_paused(unlocked_at: Option<f64>, now: f64) -> bool {
    unlocked_at.is_some_and(|t| now - t < RELOCK_PAUSE_S)
}

/// Where a hold stands: 0 at the press, 1 when it unlocks.
fn hold_progress(since: f64, now: f64) -> f32 {
    (((now - since) / UNLOCK_HOLD_S) as f32).clamp(0.0, 1.0)
}

/// The padlock chip, at an exact size, with its whole behaviour: a tap locks,
/// a 3 s hold unlocks.
pub fn padlock(ui: &mut Ui, size: egui::Vec2) -> Response {
    let ctx = ui.ctx().clone();
    let now = ctx.input(|i| i.time);
    let mut s = state(&ctx);
    let pass = ctx.cumulative_pass_nr();

    let progress = s.hold_since.map(|t| hold_progress(t, now));
    let flash = s.unlocked_at.map(|t| now - t).filter(|d| *d < FLASH_S);
    let resp = crate::chrome::chip_padlock(ui, s.locked, progress, flash.is_some(), size);

    if s.rects_pass != pass {
        s.rects.clear();
        s.rects_pass = pass;
    }
    s.rects.push(resp.rect);

    let down = resp.is_pointer_button_down_on();
    if !down {
        s.spent = false;
    }
    if s.locked {
        if down && !s.spent {
            let since = *s.hold_since.get_or_insert(now);
            if hold_progress(since, now) >= 1.0 {
                s.locked = false;
                s.hold_since = None;
                s.spent = true;
                s.unlocked_at = Some(now);
                s.hint_at = None;
            }
            ctx.request_repaint();
        } else if s.hold_since.take().is_some() {
            // Lifted early: stay locked, and say how to get out.
            s.hint_at = Some(now);
        }
        if resp.clicked() {
            s.hint_at = Some(now);
        }
    } else if resp.clicked() && relock_paused(s.unlocked_at, now) {
        // Resting after an unlock: the press is the finger that unlocked, or a
        // tap too soon after it. Say why nothing happened.
        s.hint_at = Some(now);
    } else if resp.clicked() && !s.spent {
        s.locked = true;
        s.hint_at = None;
        s.unlocked_at = None;
        // Whatever was open is closed with the lock: a menu left open would
        // be a menu nobody can close until the screen is unlocked.
        egui::Popup::close_all(&ctx);
        ctx.memory_mut(|m| m.stop_text_input());
    }

    let resp = resp.on_hover_text(if s.locked {
        tr!("touchlock-locked-hover")
    } else {
        tr!("touchlock-unlocked-hover")
    });

    // The caption under the padlock while something is happening.
    let caption = if let Some(p) = s.hold_since.map(|t| hold_progress(t, now)) {
        Some(tr!("touchlock-hold", s = format!("{:.0}", ((1.0 - p) as f64 * UNLOCK_HOLD_S).ceil().max(1.0))))
    } else if flash.is_some() || s.unlocked_at.is_some_and(|t| now - t < FLASH_S) {
        Some(tr!("touchlock-unlocked"))
    } else if s.locked && s.hint_at.is_some_and(|t| now - t < HINT_S) {
        Some(tr!("touchlock-locked-hold"))
    } else if !s.locked
        && relock_paused(s.unlocked_at, now)
        && s.hint_at.is_some_and(|t| now - t < HINT_S)
    {
        let left = s.unlocked_at.map_or(0.0, |t| RELOCK_PAUSE_S - (now - t));
        Some(tr!("touchlock-unlocked-relock", s = format!("{:.0}", left.ceil().max(1.0))))
    } else {
        None
    };
    if let Some(text) = caption {
        caption_under(&ctx, resp.rect, &text);
        ctx.request_repaint_after(std::time::Duration::from_millis(100));
    }

    store(&ctx, s);
    resp
}

/// A small label just under the padlock, on the tooltip layer so the strip's
/// own layout is not disturbed and nothing paints over it.
fn caption_under(ctx: &Context, at: Rect, text: &str) {
    let painter = ctx.layer_painter(egui::LayerId::new(egui::Order::Tooltip, id().with("caption")));
    let font = egui::FontId::proportional(12.0);
    let galley = painter.layout_no_wrap(text.to_owned(), font, crate::theme::INK_ON_CYAN());
    let pad = egui::vec2(6.0, 3.0);
    let size = galley.size() + 2.0 * pad;
    let screen = ctx.content_rect();
    let mut min = Pos2::new(at.center().x - size.x / 2.0, at.max.y + 4.0);
    min.x = min.x.clamp(screen.min.x + 2.0, (screen.max.x - size.x - 2.0).max(screen.min.x));
    let rect = Rect::from_min_size(min, size);
    painter.rect_filled(rect, 3.0, crate::theme::CYAN());
    painter.galley(rect.min + pad, galley, crate::theme::INK_ON_CYAN());
}

/// The padlock mark: a body and a shackle, the shackle lifted off its right
/// leg when open. `ink` is the chip's label colour.
pub(crate) fn paint_padlock(
    p: &egui::Painter,
    rect: Rect,
    closed: bool,
    ink: egui::Color32,
    hole: egui::Color32,
) {
    let h = rect.height() * 0.56;
    let c = rect.center();
    let body_w = h * 0.78;
    let body_h = h * 0.52;
    let body = Rect::from_center_size(
        Pos2::new(c.x, c.y + h * 0.5 - body_h / 2.0),
        egui::vec2(body_w, body_h),
    );
    let stroke = Stroke::new((h * 0.11).clamp(1.5, 2.6), ink);
    p.rect_filled(body, h * 0.08, ink);
    // The shackle: a half circle on two legs.
    let r = body_w * 0.32;
    let lift = if closed { 0.0 } else { h * 0.18 };
    let top = body.min.y - h * 0.22 - lift;
    let (lx, rx) = (c.x - r, c.x + r);
    const STEPS: usize = 16;
    let mut pts = vec![Pos2::new(lx, body.min.y)];
    for i in 0..=STEPS {
        let a = std::f32::consts::PI * (1.0 + i as f32 / STEPS as f32);
        pts.push(Pos2::new(c.x + r * a.cos(), top + r * a.sin()));
    }
    // Closed, the right leg runs into the body; open, it stops short.
    let right_end = if closed { body.min.y } else { top + r * 0.4 };
    pts.push(Pos2::new(rx, right_end));
    p.add(Shape::line(pts, stroke));
    // The keyhole, in the chip's own ground.
    let keyhole = Pos2::new(c.x, body.center().y);
    p.circle_filled(keyhole, (body_h * 0.14).max(1.2), hole);
}

/// The hold ring: an arc round the padlock from twelve o'clock, clockwise.
pub(crate) fn paint_ring(p: &egui::Painter, rect: Rect, progress: f32, ink: egui::Color32) {
    let r = rect.height() * 0.42;
    let c = rect.center();
    let stroke = Stroke::new(2.5, ink);
    p.circle_stroke(c, r, Stroke::new(1.0, ink.gamma_multiply(0.3)));
    if progress <= 0.0 {
        return;
    }
    const STEPS: usize = 48;
    let n = ((STEPS as f32 * progress).ceil() as usize).max(1);
    let start = -std::f32::consts::FRAC_PI_2;
    let sweep = std::f32::consts::TAU * progress;
    let pts: Vec<Pos2> = (0..=n)
        .map(|i| {
            let a = start + sweep * i as f32 / n as f32;
            Pos2::new(c.x + r * a.cos(), c.y + r * a.sin())
        })
        .collect();
    p.add(Shape::line(pts, stroke));
}

/// The sense the padlock chip takes: a hold has to stay its own even if the
/// finger drifts a little.
pub(crate) fn sense() -> Sense {
    Sense::click_and_drag()
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui::{Modifiers, PointerButton, TouchDeviceId, TouchId, pos2, vec2};

    fn pad() -> Vec<Rect> {
        vec![Rect::from_min_size(pos2(100.0, 10.0), vec2(60.0, 40.0))]
    }

    fn press(x: f32, y: f32, pressed: bool) -> Event {
        Event::PointerButton {
            pos: pos2(x, y),
            button: PointerButton::Primary,
            pressed,
            modifiers: Modifiers::NONE,
        }
    }

    #[test]
    fn a_locked_screen_takes_a_press_only_on_the_padlock() {
        assert!(keep_while_locked(&press(120.0, 30.0, true), &pad(), None));
        assert!(!keep_while_locked(&press(400.0, 300.0, true), &pad(), None));
        // A release anywhere still arrives, so nothing is left held down.
        assert!(keep_while_locked(&press(400.0, 300.0, false), &pad(), None));
    }

    #[test]
    fn a_locked_screen_drops_every_way_of_tuning() {
        let touch = |phase| Event::Touch {
            device_id: TouchDeviceId(0),
            id: TouchId(1),
            phase,
            pos: pos2(400.0, 300.0),
            force: None,
        };
        let wheel = Event::MouseWheel {
            unit: egui::MouseWheelUnit::Line,
            delta: vec2(0.0, 1.0),
            phase: TouchPhase::Move,
            modifiers: Modifiers::NONE,
        };
        let key = |pressed| Event::Key {
            key: egui::Key::ArrowUp,
            physical_key: None,
            pressed,
            repeat: false,
            modifiers: Modifiers::NONE,
        };
        for dropped in [
            touch(TouchPhase::Start),
            touch(TouchPhase::Move),
            wheel,
            Event::Zoom(1.2),
            Event::Rotate(0.1),
            key(true),
            Event::Text("7".into()),
            Event::Paste("7074".into()),
        ] {
            assert!(!keep_while_locked(&dropped, &pad(), None), "{dropped:?} got through");
        }
        for kept in [
            touch(TouchPhase::End),
            touch(TouchPhase::Cancel),
            key(false),
            Event::PointerMoved(pos2(400.0, 300.0)),
            Event::PointerGone,
        ] {
            assert!(keep_while_locked(&kept, &pad(), None), "{kept:?} was dropped");
        }
    }

    /// The operator's exception: on a locked screen the VU meter still takes
    /// a finger and the wheel — so the sound can come down for the adverts —
    /// while a press anywhere else stays dropped.
    #[test]
    fn a_control_allowed_while_locked_takes_a_finger_and_the_wheel() {
        let meter = Rect::from_min_size(pos2(20.0, 500.0), vec2(300.0, 46.0));
        let open = [pad(), vec![meter]].concat();
        let touch = |phase, x, y| Event::Touch {
            device_id: TouchDeviceId(0),
            id: TouchId(1),
            phase,
            pos: pos2(x, y),
            force: None,
        };
        let wheel = Event::MouseWheel {
            unit: egui::MouseWheelUnit::Line,
            delta: vec2(0.0, 1.0),
            phase: TouchPhase::Move,
            modifiers: Modifiers::NONE,
        };
        assert!(keep_while_locked(&touch(TouchPhase::Start, 100.0, 520.0), &open, None));
        assert!(keep_while_locked(&touch(TouchPhase::Move, 200.0, 520.0), &open, None));
        assert!(keep_while_locked(&press(150.0, 520.0, true), &open, None));
        assert!(keep_while_locked(&wheel, &open, Some(pos2(150.0, 520.0))));
        // Off the meter, the lock holds.
        assert!(!keep_while_locked(&touch(TouchPhase::Start, 400.0, 300.0), &open, None));
        assert!(!keep_while_locked(&press(400.0, 300.0, true), &open, None));
        assert!(!keep_while_locked(&wheel, &open, Some(pos2(400.0, 300.0))));
        assert!(!keep_while_locked(&wheel, &open, None));
    }

    /// The exception lapses with the control: a meter not drawn any more
    /// opens nothing.
    #[test]
    fn an_allowed_control_lapses_when_it_is_no_longer_drawn() {
        let meter = Rect::from_min_size(pos2(20.0, 500.0), vec2(300.0, 46.0));
        let s = State { rects: pad(), allowed: vec![meter], allowed_pass: 50, ..State::default() };
        assert!(open_rects(&s, 51).contains(&meter), "drawn just now: open");
        assert!(!open_rects(&s, 60).contains(&meter), "not drawn for ten passes: shut");
        assert!(open_rects(&s, 60).contains(&pad()[0]), "the padlock is always the way out");
    }

    #[test]
    fn the_hold_takes_three_seconds() {
        assert_eq!(hold_progress(10.0, 10.0), 0.0);
        assert!(hold_progress(10.0, 11.5) < 1.0);
        assert!(hold_progress(10.0, 12.99) < 1.0);
        assert_eq!(hold_progress(10.0, 13.0), 1.0);
    }

    #[test]
    fn the_padlock_rests_ten_seconds_after_an_unlock() {
        assert!(!relock_paused(None, 5.0), "never unlocked: nothing to rest from");
        assert!(relock_paused(Some(100.0), 100.0));
        assert!(relock_paused(Some(100.0), 109.9));
        assert!(!relock_paused(Some(100.0), 110.0));
    }

    #[test]
    fn a_lock_with_no_padlock_on_screen_lets_go() {
        let ctx = Context::default();
        store(&ctx, State { locked: true, ..State::default() });
        let mut raw =
            egui::RawInput { events: vec![press(400.0, 300.0, true)], ..Default::default() };
        filter(&ctx, &mut raw);
        assert!(!is_locked(&ctx), "locked with no way to unlock");
        assert_eq!(raw.events.len(), 1, "an unlocked screen must take the press");
    }

    #[test]
    fn a_lock_with_its_padlock_drops_the_press() {
        let ctx = Context::default();
        let pass = ctx.cumulative_pass_nr();
        store(&ctx, State { locked: true, rects: pad(), rects_pass: pass, ..State::default() });
        let mut raw = egui::RawInput {
            events: vec![press(400.0, 300.0, true), press(120.0, 30.0, true)],
            ..Default::default()
        };
        filter(&ctx, &mut raw);
        assert!(is_locked(&ctx));
        assert_eq!(raw.events.len(), 1);
    }
}

/// Where the padlock was drawn on the last pass — for the app-level tests,
/// which have to press it.
#[cfg(test)]
pub(crate) fn padlock_rects(ctx: &Context) -> Vec<Rect> {
    state(ctx).rects
}
