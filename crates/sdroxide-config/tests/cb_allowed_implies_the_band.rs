//! "Allow transmit on 11 m (CB)" and the 11M-CB tick are one setting shown twice.
//!
//! The settings dialog keeps them in step — but only while it is open and one of
//! the two changes. A `radio.json` written before the tick existed, or one whose
//! dialog was never opened, keeps the permission without the band, and then
//! nothing on the citizens' band can be keyed however the box reads.
//!
//! One test in this binary on purpose: `SDROXIDE_CONFIG_DIR` is process-global.

use sdroxide_config::Store;
use sdroxide_types::Band;

#[test]
fn a_cb_permission_without_the_band_gets_the_band() {
    let root = std::env::temp_dir().join(format!("sdroxide-cb-band-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("scratch dir");
    // SAFETY: this is the only test in this binary; nothing races the setter.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };

    // The operator allowed CB transmit, and the band list predates the tick.
    sdroxide_config::save_cb_tx_allowed(true).expect("write the permission");
    let store = Store::station();
    let mut cfg = store.load_radio_config();
    cfg.tx_band_enable = Some(vec![Band::M20]);
    store.save_radio_config(&cfg).expect("seed radio.json");

    let loaded = store.load_radio_config();
    let bands = loaded.tx_band_enable.expect("the list exists");
    assert!(
        bands.contains(&Band::M11),
        "the CB permission implies the band it is about: {bands:?}"
    );
    assert!(bands.contains(&Band::M20), "and it leaves the rest of the list alone");
}
