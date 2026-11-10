//! Each radio keeps its own DAB channel list.
//!
//! One test per file on purpose: `SDROXIDE_CONFIG_DIR` is process-global, and
//! setting it from a `#[test]` that shares a binary with others would race them.

use sdroxide_config::Store;
use sdroxide_types::DabSettings;

fn list(v: &[&str]) -> Vec<String> {
    v.iter().map(|s| s.to_string()).collect()
}

/// A Pluto and an RTL-SDR hear different blocks, and an × on one radio's
/// channel must not touch the other's. They shared one `dab.json`, so a channel
/// cleared on one radio was back as soon as the other saved its list.
#[test]
fn one_radios_dab_channels_are_not_anothers() {
    let root = std::env::temp_dir().join(format!("sdroxide-dab-radio-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("scratch dir");
    // SAFETY: this is the only test in this binary; nothing races the setter.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };

    let (first, second) = (Store::radio(0), Store::radio(2));

    // An installation from before the split: one shared list, and a radio that
    // has never saved its own starts from it rather than from nothing.
    let shared = DabSettings { found: list(&["8B", "11D", "5A"]), ..DabSettings::default() };
    first.save_dab_config(&shared).unwrap();
    assert_eq!(second.load_dab_config().found, list(&["8B", "11D", "5A"]));

    // The second radio clears 5A: gone there, still on the first.
    let mut cfg = second.load_dab_config();
    cfg.forget_channel("5A");
    second.save_dab_config(&cfg).unwrap();
    assert_eq!(second.load_dab_config().found, list(&["8B", "11D"]));
    assert_eq!(first.load_dab_config().found, list(&["8B", "11D", "5A"]));

    // And the first saving its own list leaves the second's alone.
    let mut cfg = first.load_dab_config();
    cfg.found.push("12C".into());
    first.save_dab_config(&cfg).unwrap();
    assert_eq!(second.load_dab_config().found, list(&["8B", "11D"]));

    let _ = std::fs::remove_dir_all(&root);
}
