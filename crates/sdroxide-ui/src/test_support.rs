//! Test-only helpers, shared across the crate.
//!
//! One thing, for now, and it is here because a dozen tests had each discovered
//! it separately: `egui`'s output carries a [`egui::TexturesDelta`] that a
//! renderer is expected to apply, and `epaint` **asserts in `Drop`** that
//! somebody did (`debug_assert!`, so it fires under `cargo test` and not under
//! `cargo test --release` — which is why it went unseen for so long).
//!
//! A test has no renderer, so every frame it runs has to take the deltas and
//! throw them away. Dropping a frame's output unread panics *inside a
//! destructor*, and a panic in a destructor aborts the whole process — so one
//! careless test took the other 949 down with it and the suite reported nothing
//! at all. Every test that draws a frame goes through [`run_frame`] now, and a
//! test added tomorrow should too.

use eframe::egui;

/// Run one frame and discard the texture deltas a renderer would have applied.
///
/// Everything else about the output — shapes, platform commands, repaint
/// requests — is returned as `egui` produced it.
pub(crate) fn run_frame(
    ctx: &egui::Context,
    input: egui::RawInput,
    ui: impl FnMut(&mut egui::Ui),
) -> egui::FullOutput {
    let mut out = ctx.run_ui(input, ui);
    out.textures_delta.clear();
    out
}
