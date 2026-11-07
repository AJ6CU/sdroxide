//! The process-wide plan counts how often it becomes a *different* plan, which
//! is what lets each radio's engine withdraw its own band-list confirmation
//! when the plan changes and leave it alone when the same plan is announced
//! again.
//!
//! Its own test binary: the plan is a process-wide global, and the unit tests
//! in the crate rely on nothing having installed one.

use sdroxide_types::{BandPlan, PlanMeta, band_plan, band_plan_revision, set_band_plan};

fn narrowed() -> BandPlan {
    serde_json::from_str(
        r#"{"region1":{"bands":[{"band":"M2","lo_mhz":144.0,"hi_mhz":144.4}]},
            "region2":{"bands":[{"band":"M2","lo_mhz":144.0,"hi_mhz":148.0}]},
            "region3":{"bands":[{"band":"M2","lo_mhz":144.0,"hi_mhz":148.0}]}}"#,
    )
    .expect("a valid plan")
}

#[test]
fn the_counter_moves_only_when_the_plan_does() {
    let start = band_plan_revision();
    assert!(band_plan().is_default());

    // The plan already in force, announced again: nothing happened.
    set_band_plan(BandPlan::default());
    assert_eq!(band_plan_revision(), start);

    // A different plan: one change.
    set_band_plan(narrowed());
    assert_eq!(band_plan_revision(), start + 1);
    assert!(!band_plan().is_default());

    // The same again, even under another name: still the same plan.
    set_band_plan(narrowed());
    set_band_plan(narrowed().with_meta(PlanMeta { name: "Renamed".into(), ..PlanMeta::default() }));
    assert_eq!(band_plan_revision(), start + 1);
    assert_eq!(band_plan().meta().name, "Renamed", "but the new name is installed");

    // And back to the built-in tables is a change too, however many times the
    // operator has been there before.
    set_band_plan(BandPlan::default());
    assert_eq!(band_plan_revision(), start + 2);
    set_band_plan(narrowed());
    assert_eq!(band_plan_revision(), start + 3);
}
