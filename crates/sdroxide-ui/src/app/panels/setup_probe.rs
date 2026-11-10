//! Render the digimode setup window to a PNG, so the layout can be looked at.
//!
//! Issue #357 is a *drawn* fault: in a browser the FT8 setup window's controls
//! were pushed off the right-hand edge, the depth chips clipped and the fields
//! squeezed to a few points wide. No assertion about a rectangle says whether
//! the operator can read the dialog; a picture does. So this writes the real
//! window at the widths it is reported at, in the language it was reported in,
//! and leaves the files in `target/digi-setup-probe-*.png` to open.
//!
//! ```sh
//! SDROXIDE_PROBE=1 cargo test -p sdroxide-ui --lib digi_setup_probe -- --nocapture
//! ```

#[cfg(test)]
mod probe {
    use crate::app::SdroxideApp;
    use sdroxide_types::UiLanguage;

    /// A station with no radio behind it: the dialog is drawn from the app's own
    /// state, and a receiver would only add a spectrum nothing here reads.
    struct Bare;

    impl sdroxide_types::RadioController for Bare {
        fn send(&mut self, _cmd: sdroxide_types::Command) {}
        fn poll_event(&mut self) -> Option<sdroxide_types::RadioEvent> {
            None
        }
    }

    /// One width, one language, one picture.
    fn shot(name: &str, language: UiLanguage, size: eframe::egui::Vec2) {
        use eframe::egui;
        if std::env::var("SDROXIDE_PROBE").is_err() {
            return;
        }
        let dir = std::env::temp_dir()
            .join(format!("sdroxide-digi-setup-probe-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };

        let ctx = egui::Context::default();
        let mut app = SdroxideApp::new_tab(&ctx, None, None, Box::new(Bare), 0, true);
        // On the app rather than on the loader: every frame re-applies the
        // client's own choice (`set_language(self.applied_language)`), so a call
        // to the loader alone is overwritten on the first frame.
        app.ui_settings.language = language;
        crate::i18n::set_language(language);
        app.state.rx[0].mode = sdroxide_types::Mode::Ft8;
        app.show_digi_settings = true;
        app.digi_cfg_edit.my_call = "F4NRM".into();
        app.digi_cfg_edit.my_grid = "JN38ES".into();

        // Textures accumulate across every frame: the font atlas is handed out
        // once, in the first frame, and a mesh whose texture is unknown is
        // skipped — so keeping only the last frame rasterises the text to
        // nothing.
        let mut textures = crate::test_raster::Textures::default();
        let mut full = None;
        for _ in 0..4 {
            let input = egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
                ..Default::default()
            };
            let mut out = ctx.run_ui(input, |ui| {
                eframe::App::ui(&mut app, ui, &mut eframe::Frame::_new_kittest())
            });
            textures.apply(&out.textures_delta);
            // A test has no renderer, and `epaint` asserts in `Drop` that
            // somebody took the deltas. Applied above and thrown away here, the
            // same way `test_support::run_frame` does it.
            out.textures_delta.clear();
            full = Some(out);
        }
        let full = full.expect("at least one frame");
        let img = crate::test_raster::rasterize(
            &ctx,
            full,
            &textures,
            size,
            ctx.style_of(egui::Theme::Dark).visuals.window_fill(),
        );
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target");
        let _ = std::fs::create_dir_all(&dir);
        let path = dir.join(format!("digi-setup-probe-{name}.png"));
        img.save(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        println!("wrote {}", path.display());
    }

    /// The reporter's own setup: French — the language the screenshot was taken
    /// in — on a browser at laptop width.
    #[test]
    fn digi_setup_probe_web_fr() {
        use crate::multi::frame_test_lock;
        let _guard = frame_test_lock();
        shot("web-fr", UiLanguage::from_tag("fr").unwrap(), eframe::egui::vec2(1400.0, 900.0));
    }

    /// The dialog at its own designed width, which is what a phone gets.
    #[test]
    fn digi_setup_probe_narrow_fr() {
        use crate::multi::frame_test_lock;
        let _guard = frame_test_lock();
        shot("narrow-fr", UiLanguage::from_tag("fr").unwrap(), eframe::egui::vec2(420.0, 760.0));
    }

    /// English, for the comparison: the fault was a *message* drawn as a row
    /// caption, so it showed in every language and not only the long ones.
    #[test]
    fn digi_setup_probe_web_en() {
        use crate::multi::frame_test_lock;
        let _guard = frame_test_lock();
        shot("web-en", UiLanguage::from_tag("en-US").unwrap(), eframe::egui::vec2(1400.0, 900.0));
    }
}
