//! **A newest-first list must not slide out from under the operator.**
//!
//! FT8's decode list is drawn newest-first, and since the finder began
//! reporting each decode as it lands rather than in one batch at the end of the
//! slot, a decode is **prepended** several times inside a slot. Every row on
//! screen is pushed down by the height of what arrived, so an operator reading
//! row 5 finds themselves at row 4 — and again. Discussion #90 reports it as
//! *"la barre de défilement FT8 n'est plus synchronisée avec le décodage"*.
//!
//! This drives the real helper, `chrome::anchored_list_scroll`: it scrolls the
//! area with a wheel event, lets the scroll **settle** (egui animates it over
//! several frames — measuring during that reads the animation, not the anchor,
//! which is how the first version of this test fooled itself), then prepends a
//! row exactly as the `Decodes` arm does and asks which row is at the top of
//! the viewport. **The row is the assertion** — an index is what shifts; the
//! row is what the operator loses.

use eframe::egui;
use sdroxide_ui::chrome::anchored_list_scroll;

/// One frame of the list; `wheel` is an optional one-off wheel event. Reports
/// the label at the top of the viewport.
fn frame(ctx: &egui::Context, items: &[String], wheel: Option<f32>) -> String {
    let mut events = vec![egui::Event::PointerMoved(egui::pos2(60.0, 60.0))];
    if let Some(dy) = wheel {
        events.push(egui::Event::MouseWheel {
            unit: egui::MouseWheelUnit::Point,
            delta: egui::vec2(0.0, dy),
            phase: egui::TouchPhase::Move,
            modifiers: Default::default(),
        });
    }
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(300.0, 200.0))),
        events,
        ..Default::default()
    };
    let mut top = String::new();
    let full = ctx.run_ui(input, |ui| {
        top = anchored_list_scroll(ui, "test-list", |ui| {
            let viewport_top = ui.clip_rect().top();
            let mut at_top = String::new();
            for label in items {
                let r = ui.label(label);
                if at_top.is_empty() && r.rect.bottom() > viewport_top {
                    at_top = label.clone();
                }
            }
            at_top
        });
    });
    full.drop_without_applying_deltas();
    top
}

/// Frames with nothing happening, until the wheel's inertia has died out, and
/// where the view ended. A fixed count rather than "until two frames agree":
/// egui animates a wheel over many frames and two consecutive frames can match
/// while it is still creeping, which made the first version of this test read
/// the animation instead of the anchor.
fn settle(ctx: &egui::Context, items: &[String]) -> String {
    let mut last = String::new();
    for _ in 0..40 {
        last = frame(ctx, items, None);
    }
    last
}

#[test]
fn a_newest_first_list_does_not_slide_under_the_reader() {
    let ctx = egui::Context::default();
    let plain: Vec<String> = (0..40).map(|i| format!("row {i}")).collect();

    // Scroll, then let it settle. Part way down only: at the *bottom* the
    // offset is the maximum and egui holds it there by clamping, so a test
    // parked at the end would pass for a reason that has nothing to do with
    // anchoring.
    let _ = frame(&ctx, &plain, Some(-120.0));
    let scrolled = settle(&ctx, &plain);
    assert_ne!(
        scrolled, "row 0",
        "the wheel did not scroll the list — this test would prove nothing"
    );
    assert_ne!(scrolled, "row 39", "the list scrolled to its end, where the clamp holds the view");

    // A decode streams in: prepended, as the newest-first list does.
    let mut streamed = plain.clone();
    streamed.insert(0, "row NEW".to_string());
    let after = settle(&ctx, &streamed);
    assert_eq!(
        after, scrolled,
        "a decode arrived above the reader and the list slid: they were reading {scrolled:?} and \
         are now looking at {after:?}"
    );

    // And the newest is *not* held back from someone watching the top: there
    // the list is meant to move, because the newest row is the point of it.
    let ctx_top = egui::Context::default();
    assert_eq!(settle(&ctx_top, &plain), "row 0", "a fresh list starts at its newest row");
    assert_eq!(
        settle(&ctx_top, &streamed),
        "row NEW",
        "at the top the newest decode is what the operator is watching for, and it must appear"
    );
}
