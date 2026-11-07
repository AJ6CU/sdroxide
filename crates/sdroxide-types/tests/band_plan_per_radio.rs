//! A radio can run a band plan of its own without any other radio noticing.
//!
//! The plan every band edge, lockout and waterfall strip asks for depends on the
//! thread that asks: a thread that works for a radio with a plan of its own
//! gets that plan, every other thread gets the station's. Its own test binary,
//! because the station's plan is a process-wide global.

use sdroxide_types::{
    Band, BandPlan, ThreadPlanGuard, band_plan, band_plan_revision, leak_band_plan, set_band_plan,
    set_thread_band_plan, station_band_plan,
};

/// The built-in tables without 20 m: on a radio running this, 14.074 MHz is in
/// the shortwave broadcast band, not the amateur one.
fn without_20m() -> BandPlan {
    let doc = BandPlan::default().to_json_document();
    let row = r#"      {"band": "M20", "lo_mhz": 14.0, "hi_mhz": 14.35},"#;
    assert!(doc.contains(row), "the built-in 20 m row moved");
    serde_json::from_str(&doc.replace(&format!("{row}\n"), "")).expect("a valid plan")
}

const FT8_20M: f64 = 14_074_000.0;

/// The station's plan is one global shared by every test in this binary, and one
/// of them changes it: they take turns.
static TURN: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn turn() -> std::sync::MutexGuard<'static, ()> {
    TURN.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

#[test]
fn a_radios_own_plan_is_seen_by_its_thread_and_nobody_else() {
    let _turn = turn();
    let own = leak_band_plan(without_20m());
    assert_eq!(Band::containing(FT8_20M), Band::M20, "the station's plan has 20 m");
    let revision = band_plan_revision();

    // Another radio's thread (its engine) declares a plan of its own.
    let other = std::thread::spawn(move || {
        assert_eq!(Band::containing(FT8_20M), Band::M20, "before it declares: the station's");
        set_thread_band_plan(Some(own));
        let seen = (Band::containing(FT8_20M), band_plan().is_default());
        // And the station's plan is still the station's from inside that thread.
        (seen, station_band_plan().is_default())
    })
    .join()
    .unwrap();
    assert_eq!(other, ((Band::Sw, false), true), "that thread has no 20 m: it is in shortwave broadcast");

    // This thread never noticed, and nothing moved the station's counter.
    assert_eq!(Band::containing(FT8_20M), Band::M20);
    assert!(band_plan().is_default());
    assert_eq!(band_plan_revision(), revision, "a radio's own plan is not a change of the station's");
}

#[test]
fn a_screen_draws_each_radio_with_its_own_plan_and_puts_the_last_one_back() {
    let _turn = turn();
    let own = leak_band_plan(without_20m());
    assert_eq!(Band::containing(FT8_20M), Band::M20);
    {
        let _radio_a = ThreadPlanGuard::enter(Some(own));
        assert_eq!(Band::containing(FT8_20M), Band::Sw, "radio A, on its own plan");
        {
            let _radio_b = ThreadPlanGuard::enter(None);
            assert_eq!(Band::containing(FT8_20M), Band::M20, "radio B follows the station");
        }
        assert_eq!(Band::containing(FT8_20M), Band::Sw, "back to radio A");
    }
    assert_eq!(Band::containing(FT8_20M), Band::M20, "nobody's plan left behind");
}

#[test]
fn the_station_counter_compares_with_the_stations_plan_not_the_radios() {
    let _turn = turn();
    let own = leak_band_plan(without_20m());
    let start = band_plan_revision();
    let _radio = ThreadPlanGuard::enter(Some(own));

    // The station announcing the plan it already has — while this thread runs a
    // plan of its own — is no change: it is compared with the station's.
    set_band_plan(BandPlan::default());
    assert_eq!(band_plan_revision(), start);
    // A different station plan is, whatever this radio runs.
    set_band_plan(without_20m());
    assert_eq!(band_plan_revision(), start + 1);
    assert!(!station_band_plan().is_default());
    assert!(!band_plan().is_default(), "and this thread still has its own");
    drop(_radio);
    set_band_plan(BandPlan::default());
}
