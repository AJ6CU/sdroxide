//! A `detached` list that is the wrong length must cost that field, not the
//! whole `config.toml`.
//!
//! `UiSettings::detached` is one slot per [`DetachableModule`], so **adding a
//! module makes the array longer** — AUX SP is exactly that. Every config written
//! before it carries a shorter list, and the cost of getting this wrong is not a
//! lost window position: `Settings::load` *quarantines* a file it cannot parse
//! and answers `Settings::default()`, which would take the operator's theme,
//! fonts, layout and the rest of their station with it.
//!
//! One test in the binary, because `SDROXIDE_CONFIG_DIR` is process-global and
//! setting it from a `#[test]` sharing a binary with others would race them.

use std::fs;

use sdroxide_types::{DetachableModule, UiSettings};

fn scratch(tag: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sdroxide-detached-{tag}-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

/// Load a `config.toml` holding `detached` and report what came back: the frame
/// rate (a setting with nothing to do with the windows, so it proves the file
/// parsed), the slots that were undocked, and whether the file is still there.
fn load_with(dir: &std::path::Path, detached: &str) -> (UiSettings, Vec<bool>, bool) {
    // SAFETY: this is the only test in this binary; nothing races the setter.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", dir) };
    fs::write(
        dir.join("config.toml"),
        format!("[ui]\nframe_rate_fps = 37\ndetached = [{detached}]\n"),
    )
    .expect("write");
    let ui = sdroxide_config::load_ui_settings();
    let slots = DetachableModule::ALL.iter().map(|m| ui.detached[m.index()].detached).collect();
    let quarantined = !dir.join("config.toml").exists();
    (ui, slots, quarantined)
}

#[test]
fn a_detached_list_of_another_length_leaves_the_rest_of_the_config_alone() {
    // A config from before a module was added: a shorter list. The slots it
    // carries are kept, the module it does not reach starts docked, and above
    // all the file is still there.
    let dir = scratch("short");
    let (ui, slots, quarantined) = load_with(&dir, "{ detached = true }, { detached = true }");
    assert_eq!(ui.frame_rate_fps, 37, "the file parsed: the whole [ui] table is intact");
    assert!(
        !quarantined,
        "a shorter detached list quarantined config.toml — every other setting went with it"
    );
    // Spelled out rather than pinned to a count: the point is the *rule* — the
    // two slots the list carried are kept, and every module it did not reach
    // starts docked — so this stays true as more modules are added.
    let kept: Vec<usize> = (0..slots.len()).filter(|i| slots[*i]).collect();
    assert_eq!(kept, vec![0, 1], "the slots it carries, and no invented ones");

    // And the mirror: a config written by a build with more modules than this
    // one has heard of. Its extra slots are ignored rather than failing the file.
    let dir = scratch("long");
    let long: Vec<&str> =
        std::iter::repeat_n("{ detached = true }", DetachableModule::COUNT + 2).collect();
    let (ui, slots, quarantined) = load_with(&dir, &long.join(", "));
    assert_eq!(ui.frame_rate_fps, 37);
    assert!(!quarantined, "a longer detached list quarantined config.toml");
    assert_eq!(slots, vec![true; DetachableModule::COUNT]);
}
