//! What a `radio.json` that cannot be read costs the operator.
//!
//! One test per file on purpose: `SDROXIDE_CONFIG_DIR` is process-global, and
//! setting it from a `#[test]` that shares a binary with others would race them.

use sdroxide_config::Store;
use sdroxide_types::Backend;

/// A configuration that has just been taken away from the operator must not be
/// replaced by one that opens hardware.
///
/// Field-reported, and the chain was long enough that nothing on screen
/// connected its ends. A renamed enum variant made one value in `radio.json`
/// unreadable; serde fails a `RadioConfig` whole, so the loader quarantined the
/// file and returned the defaults; the default backend is a device backend, so
/// the program went looking for hardware and opened the first thing a SoapySDR
/// module offered — which claimed the operator's receiver by its bare USB
/// controller id, flooded the console with transfer errors, and never started.
/// The operator had selected a native driver for that same receiver.
///
/// The fix is not to guess better. It is to open nothing and say so.
#[test]
fn a_quarantined_radio_config_selects_no_interface_at_all() {
    let root =
        std::env::temp_dir().join(format!("sdroxide-quarantine-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("scratch dir");
    // SAFETY: this is the only test in this binary; nothing races the setter.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };

    // A first run has no file at all. That is not a fault and the defaults are
    // the right answer, so this path must keep working exactly as before.
    let fresh = Store::radio(0).load_radio_config();
    assert_eq!(fresh.backend, Backend::default());
    assert_ne!(Backend::default(), Backend::None, "the default must still open something");

    // Now a real configuration, with a value this build cannot read. Everything
    // else in it is valid — which is the whole point: the operator chose an
    // interface, and one unrelated setting is about to lose it for them.
    let text = r#"{
        "backend": "Rx888",
        "rx888": { "serial": "0000000004BE" },
        "cat": { "kenwood_send": "SomeVariantThisBuildDoesNotHave" }
    }"#;
    std::fs::write(root.join("radio.json"), text).expect("write");

    let cfg = Store::radio(0).load_radio_config();
    assert_eq!(
        cfg.backend,
        Backend::None,
        "a reset config must open nothing, not go looking for hardware"
    );

    // The evidence of what was lost survives, so the operator can see what the
    // file said and put it back.
    assert!(root.join("radio.json.bak").exists(), "the unreadable file was not kept");
    let kept = std::fs::read_to_string(root.join("radio.json.bak")).expect("read .bak");
    assert!(kept.contains("0000000004BE"), "the kept copy is not the original");

    // And it has to hold across a restart, which is the half that actually
    // bit. Quarantine *renames* the bad file away, so a fallback that lives
    // only in memory leaves the next start looking at an empty directory —
    // indistinguishable from a first run, and straight back to the default
    // backend. The operator hit that on every launch after the first, which is
    // why nothing they did made it stop.
    assert!(root.join("radio.json").exists(), "the safe fallback was not recorded on disk");
    for restart in 1..=3 {
        assert_eq!(
            Store::radio(0).load_radio_config().backend,
            Backend::None,
            "start {restart} after the reset went looking for hardware again"
        );
    }

    // Choosing an interface sticks, so the state is a prompt and not a trap.
    let chosen = sdroxide_types::RadioConfig { backend: Backend::Rx888, ..Default::default() };
    Store::radio(0).save_radio_config(&chosen).expect("save");
    assert_eq!(Store::radio(0).load_radio_config().backend, Backend::Rx888);

    unsafe { std::env::remove_var("SDROXIDE_CONFIG_DIR") };
    let _ = std::fs::remove_dir_all(&root);
}
