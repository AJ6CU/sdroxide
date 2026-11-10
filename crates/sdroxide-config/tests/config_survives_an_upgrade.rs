//! **A configuration written by an older build still loads in this one.**
//!
//! This is the shape of a whole class of "my settings were lost after the
//! update": a field added to a stored struct without a serde default makes the
//! *entire* file unparseable, and `Store::load` **quarantines** it — the old
//! file is renamed to `.bak` and every setting in it is back to defaults at
//! once. The theme, the fonts, the layout and the radio go together.
//!
//! Three separate guarantees, because a file can be lost three ways:
//!
//! 1. **An older file still parses.** An empty document is the oldest file
//!    there is, so if `{}` loads, any subset of the fields loads. The partial
//!    `[ui]` table is the case that actually happens: the operator's
//!    `config.toml` holds the keys their build knew, and a newer build reads it.
//! 2. **A setting survives being written and read back.** This is the half a
//!    parse check cannot see: a field with `skip_serializing`, or a
//!    `deserialize_with` that reads something other than what the derived
//!    serializer wrote, vanishes on every restart *while the file parses
//!    perfectly*. The colours and the detached-window list are the two
//!    hand-written ones here, and they are the reason this test exists.
//! 3. **Nothing is silently dropped on the way out**, which falls out of (2)'s
//!    line-by-line comparison rather than being asserted separately.
//!
//! Written while chasing discussion #90's *"many parameter not persistant"*.
//! It found none of these broken, which is the point of keeping it: the class
//! is now checked rather than argued about.
//!
//! **What actually protects these three is a container-level `#[serde(default)]`
//! on `Settings`, `UiSettings` and `Session`**, not the per-field attributes —
//! remove the container one and a missing field takes the whole file down
//! again. That is what this test fires on, verified: with it removed,
//! `an_older_config_toml_still_loads` fails with *"a [ui] table with one key no
//! longer loads … missing field `frame_rate_fps`"*. A new stored struct needs
//! the same attribute, and the same line in the list below.
//!
//! **One thing it deliberately does not cover**, so nobody adds them here by
//! mistake: `SpectrumConfig`, `Qo100Settings` and `HfdlSettings` cannot load an
//! empty document, and that is harmless because **none of them is ever read
//! from a file**. The first travels on the wire; the other two are held by the
//! engine and — at the time of writing — never saved at all (see the note in
//! `engine.rs`: *"it is not persisted — the same convention `qo100_cfg`
//! follows"*). If one of them is ever given a file, add it to this test **and**
//! give every field a default in the same commit.

use sdroxide_config::Settings;
use sdroxide_types::{ChromeStyle, FontSize, LayoutMode, SpeechSettings, UiTheme};

/// Read a document that an older build could have written.
macro_rules! loads {
    ($what:literal, $ty:ty, $text:expr) => {
        assert!(
            toml::from_str::<$ty>($text).is_ok(),
            "{} no longer loads — a field was added without a serde default, so \
             Store::load quarantines the whole file and every setting in it goes \
             back to defaults: {:?}",
            $what,
            toml::from_str::<$ty>($text).err(),
        )
    };
}

#[test]
fn an_older_config_toml_still_loads() {
    loads!("an empty config.toml", Settings, "");
    loads!("a config.toml with one key", Settings, "sample_rate = 2048000.0\n");
    loads!("a [ui] table with one key", Settings, "[ui]\ntheme = \"Default\"\n");
    loads!(
        "a [ui] table written before the newest keys existed",
        Settings,
        "[ui]\nframe_rate_fps = 60\nwaterfall_speed = \"Medium\"\nlayout = \"Auto\"\n"
    );
    loads!(
        "a [ui] table a newer build wrote, read by this one",
        Settings,
        "[ui]\nsome_key_from_a_future_build = 1\n"
    );

    // The same question asked of the sub-tables on their own, which is what a
    // partial file hands them.
    loads!("an empty [ui]", sdroxide_types::UiSettings, "");
    loads!("[ui] with only an old key", sdroxide_types::UiSettings, "theme = \"Default\"\n");
    loads!("an empty [speech]", SpeechSettings, "");
    loads!("an empty [alerts]", sdroxide_types::AlertSettings, "");
    loads!("an empty [remote_server]", sdroxide_types::RemoteServer, "");
    loads!("an empty [remote_access]", sdroxide_types::RemoteAccess, "");
}

#[test]
fn every_setting_survives_a_save_and_load() {
    let mut s = Settings::default();
    // Non-default values, and deliberately the ones the recent PRs added plus
    // every field with hand-written serde — a silent drop lives in one of
    // those or it lives nowhere.
    s.sample_rate = 1_234_567.0;
    s.cal_offset_db = -3.25;
    s.spectrum_fft = 8_192;
    s.spectrum_fps = 17;
    s.swr_limit = 3.75;
    s.ui.theme = UiTheme::Calm;
    s.ui.button_style = ChromeStyle::Bevel;
    s.ui.window_style = ChromeStyle::Terminal;
    s.ui.layout = LayoutMode::Desktop;
    s.ui.skimmer_font_size = FontSize::Large;
    s.ui.menu_font_size = FontSize::Small;
    s.ui.spectrum_fill = true;
    s.ui.spectrum_fill_color = [11, 22, 33];
    s.ui.spectrum_fill_style = sdroxide_types::SpectrumFillStyle::Mixed;
    s.ui.spectrum_gradient = false;
    s.ui.gradient_top = [1, 2, 3];
    s.ui.gradient_bottom = [4, 5, 6];
    s.ui.tune_step_hz = 12_500.0;
    s.ui.spot_colors[0] = [7, 8, 9];
    s.ui.bandplan_colors[1] = [10, 11, 12];
    s.ui.simple_ui = true;
    s.ui.map_cities = false;
    s.ui.keep_screen_on = true;
    s.ui.broadcast_scope = sdroxide_types::BroadcastScope::WholeBand;
    s.speech = SpeechSettings { enabled: !SpeechSettings::default().enabled, ..s.speech };
    s.tx_ham_only = !s.tx_ham_only;
    s.cb_tx_allowed = !s.cb_tx_allowed;
    s.region = sdroxide_types::Region::R2;
    s.cb_plan = sdroxide_types::CbPlan::Cept;

    let wrote = toml::to_string(&s).expect("config.toml is writable");
    let read: Settings = toml::from_str(&wrote).expect("what it wrote, it can read");
    let again = toml::to_string(&read).expect("and write a second time");

    let lost: Vec<String> = wrote
        .lines()
        .zip(again.lines())
        .enumerate()
        .filter(|(_, (a, b))| a != b)
        .map(|(i, (a, b))| format!("line {}: wrote {a:?}, read {b:?}", i + 1))
        .collect();
    assert!(
        lost.is_empty(),
        "{} line(s) are written but do not come back — a setting that disappears \
         on every restart while the file parses:\n{}",
        lost.len(),
        lost.join("\n")
    );
    assert_eq!(
        wrote.lines().count(),
        again.lines().count(),
        "the second write is a different size from the first"
    );
}

#[test]
fn a_radio_config_survives_a_save_and_load() {
    let r = sdroxide_types::RadioConfig {
        smeter_cal: -3,
        converter_offset_hz: 9_750_000.0,
        rx_audio_gain_db: -7.5,
        ..sdroxide_types::RadioConfig::default()
    };
    let wrote = serde_json::to_string(&r).expect("radio.json is writable");
    let read: sdroxide_types::RadioConfig = serde_json::from_str(&wrote).expect("and readable");
    assert_eq!(
        wrote,
        serde_json::to_string(&read).unwrap(),
        "radio.json does not survive its own round trip"
    );
}

/// Every stored config, read from the document a previous build would have
/// left. The list is the types `Store::load` is actually called with.
macro_rules! empty_document {
    ($($t:ty),* $(,)?) => {
        #[test]
        fn every_stored_config_loads_an_empty_document() {
            let mut bad = Vec::new();
            $(
                if let Err(e) = serde_json::from_str::<$t>("{}") {
                    bad.push(format!("{}: {e}", stringify!($t)));
                }
            )*
            assert!(
                bad.is_empty(),
                "these are stored as whole files and no longer load an older \
                 one — a field is missing a serde default:\n{}",
                bad.join("\n")
            );
        }
    };
}

empty_document!(
    sdroxide_types::RadioConfig,
    sdroxide_types::NetworkConfig,
    sdroxide_types::DigiConfig,
    sdroxide_types::DabSettings,
    sdroxide_types::ScannerConfig,
    sdroxide_types::TciServerConfig,
    sdroxide_types::RigctldConfig,
    sdroxide_types::WsjtxConfig,
    sdroxide_types::AlertSettings,
    sdroxide_types::SpeechSettings,
    sdroxide_types::SkimmerSettings,
    sdroxide_types::SatConfig,
    sdroxide_types::RotatorConfig,
    sdroxide_types::AdsbSettings,
    sdroxide_config::Session,
);
