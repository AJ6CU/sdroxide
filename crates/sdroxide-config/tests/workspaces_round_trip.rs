//! Workspaces survive a round trip through `workspaces.json`, and a workspace
//! this build cannot read costs only itself.
//!
//! One test in the binary, because `SDROXIDE_CONFIG_DIR` is process-global.

use std::fs;

use sdroxide_types::{DetachableModule, DetachedState, UiSettings, Workspace};

#[test]
fn workspaces_round_trip_and_a_bad_row_costs_only_itself() {
    let dir = std::env::temp_dir().join(format!("sdroxide-workspaces-{}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("scratch dir");
    // SAFETY: this is the only test in this binary; nothing races the setter.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };

    let mut ui = UiSettings::default();
    ui.set_detached(DetachableModule::Panadapter, true);
    ui.set_detached(DetachableModule::AuxPanadapter, true);
    let saved = Workspace::capture("two spectra", &ui);

    sdroxide_config::save_workspaces(&vec![saved.clone()]).expect("saves");
    let back = sdroxide_config::load_workspaces();
    assert_eq!(back, vec![saved.clone()], "a workspace comes back exactly as it went out");

    // A row this build cannot read is dropped, and the rest of the file stays:
    // a list of independent arrangements, not one document.
    let mut raw: Vec<serde_json::Value> =
        serde_json::from_str(&fs::read_to_string(dir.join("workspaces.json")).unwrap()).unwrap();
    raw.push(serde_json::json!({ "name": "broken", "slots": "not a list" }));
    fs::write(dir.join("workspaces.json"), serde_json::to_string(&raw).unwrap()).unwrap();

    let survivors = sdroxide_config::load_workspaces();
    assert_eq!(survivors, vec![saved], "the good row survived the bad one");

    // And applying what came back restores the arrangement, module for module.
    let mut applied = UiSettings::default();
    survivors[0].apply(&mut applied);
    assert!(applied.is_detached(DetachableModule::Panadapter));
    assert!(applied.is_detached(DetachableModule::AuxPanadapter));
    assert!(!applied.is_detached(DetachableModule::Controls));

    // A workspace with no slots (a row that never got an arrangement) leaves
    // everything docked rather than half-restored.
    let empty = Workspace { name: "empty".into(), slots: Vec::<DetachedState>::new() };
    let mut from_empty = UiSettings::default();
    from_empty.set_detached(DetachableModule::Controls, true);
    empty.apply(&mut from_empty);
    assert!(!from_empty.is_detached(DetachableModule::Controls));
}
