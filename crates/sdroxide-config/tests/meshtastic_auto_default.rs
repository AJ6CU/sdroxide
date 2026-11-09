//! The Meshtastic preset is "Auto" by default, because not every operator knows
//! which preset a network runs. A `meshtastic.json` written while the default
//! was LongFast is moved to Auto once; a later deliberate LongFast is kept.
//!
//! One test in the binary: `SDROXIDE_CONFIG_DIR` is process-global.

use std::fs;

#[test]
fn an_old_long_fast_default_becomes_auto_once() {
    let dir = std::env::temp_dir().join(format!("sdroxide-mesh-auto-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch dir");
    // SAFETY: the only test in this binary; nothing races the setter.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };

    // A file from before: LongFast, never chosen by anybody.
    fs::write(
        dir.join("meshtastic.json"),
        r#"{"enabled":true,"region":"EU_868","preset":"LongFast","frequency_hz":869525000.0,"keys":"","restore":null}"#,
    )
    .unwrap();
    let first = sdroxide_config::load_meshtastic_config();
    assert_eq!(first.preset, "Auto");
    assert!(first.enabled, "nothing else changes");
    let stored = fs::read_to_string(dir.join("meshtastic.json")).unwrap();
    assert!(stored.contains("\"Auto\""), "and the file says so: {stored}");

    // The operator picks LongFast on purpose: it stays.
    let mut chosen = first.clone();
    chosen.preset = "LongFast".to_string();
    sdroxide_config::save_meshtastic_config(&chosen).unwrap();
    assert_eq!(sdroxide_config::load_meshtastic_config().preset, "LongFast");

    // No file at all: the default.
    fs::remove_file(dir.join("meshtastic.json")).unwrap();
    assert_eq!(sdroxide_config::load_meshtastic_config().preset, "Auto");
    let _ = fs::remove_dir_all(&dir);
}
