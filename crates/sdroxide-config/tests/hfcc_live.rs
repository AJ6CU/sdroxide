//! Live check on HFCC: the package downloads and resolves **names, sites and
//! positions** for nearly every row.
//!
//! `#[ignore]`d because it needs the network. It is the only thing that catches
//! a *silent* parse failure — this source has produced three in a row, every one
//! of which loaded the file successfully and returned plausible-looking rows.
//!
//! ```sh
//! SDROXIDE_CONFIG_DIR=/tmp/hfcc-live \
//!   cargo test -p sdroxide-config --test hfcc_live -- --ignored --nocapture
//! ```
#[test]
#[ignore = "needs the network"]
fn hfcc_resolves_names_sites_and_positions() {
    let rows = sdroxide_config::fetch_broadcast_schedule_from(sdroxide_types::ScheduleSource::Hfcc)
        .expect("HFCC must download and resolve");
    let named = rows.iter().filter(|r| r.name.len() > 3).count();
    let placed = rows.iter().filter(|r| r.lat.is_some() && r.lon.is_some()).count();
    assert_eq!(rows.len(), 3367, "the A26 schedule's rows");
    assert!(named * 100 >= rows.len() * 95, "only {named} of {} resolved a name", rows.len());
    assert!(placed * 100 >= rows.len() * 95, "only {placed} of {} resolved a position", rows.len());
}
