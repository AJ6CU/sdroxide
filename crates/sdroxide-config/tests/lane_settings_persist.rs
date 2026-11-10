//! The two lanes that used to forget everything they were told.
//!
//! HFDL and QO-100 were the only wideband lanes whose settings were never
//! written to disk — deliberately, and the code said so (*"it is not persisted
//! — the same convention `qo100_cfg` follows"*). Every other lane (ADS-B, AIS,
//! VDL2, ISM, DAB, the scanner) has saved and restored its own since it was
//! added, and starts its decoder from the file at launch.
//!
//! The two being different produced the one thing an operator notices without
//! being able to name it: turn the decoder on, settle a channel, close the
//! program, open it, and that lane alone is off and on a default frequency.
//! r/ShortwavePlus called it *"a few noise-reduction features; they're nothing
//! special"* about a different feature and discussion #90 called it *"many
//! parameter not persistant"* about the program in general; this is the part of
//! that which is provably true.

/// Point the config store at a directory of its own, the way every other test
/// in this crate that touches a file does.
fn isolated(name: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join(format!("sdroxide-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).unwrap();
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };
    dir
}

/// One test, not two: `SDROXIDE_CONFIG_DIR` is process-global, so two tests in
/// one binary race each other for it — which is exactly how the first version
/// of this file failed, with the *other* lane's directory winning.
#[test]
fn the_two_session_only_lanes_remember_what_they_were_told() {
    let dir = isolated("hfdlcfg");
    // Nothing saved yet: the lane the program ships with, off, on the
    // reference channel. Off by default is a property of the *default*, and
    // restoring what the operator chose does not change it.
    let fresh = sdroxide_config::load_hfdl_config();
    assert!(!fresh.enabled, "a fresh configuration starts with HFDL off");
    assert_eq!(fresh.frequency_hz, sdroxide_types::HFDL_DEFAULT_HZ);

    let mine = sdroxide_types::HfdlSettings { enabled: true, frequency_hz: 5_451_000.0 };
    sdroxide_config::save_hfdl_config(&mine).unwrap();
    assert!(dir.join("hfdl.json").exists());
    assert_eq!(sdroxide_config::load_hfdl_config(), mine, "the lane forgot what it was told");

    let _ = std::fs::remove_dir_all(&dir);

    let dir = isolated("qo100cfg");
    assert!(
        !sdroxide_config::load_qo100_config().enabled,
        "a fresh configuration starts with the beacon tracker off"
    );

    let mine = sdroxide_types::Qo100Settings {
        enabled: true,
        decode_telemetry: true,
        park_lo_hz: 3_000.0,
        park_hi_hz: 9_000.0,
        ..sdroxide_types::Qo100Settings::default()
    };
    sdroxide_config::save_qo100_config(&mine).unwrap();
    assert!(dir.join("qo100.json").exists());
    assert_eq!(sdroxide_config::load_qo100_config(), mine, "the tracker forgot what it was told");

    let _ = std::fs::remove_dir_all(&dir);
    unsafe { std::env::remove_var("SDROXIDE_CONFIG_DIR") };
}
