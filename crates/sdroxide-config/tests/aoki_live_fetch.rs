//! Live check on Aoki's **whole** download path: index → URL → zip → cache → parse.
//!
//! `#[ignore]`d because it needs the network. Run it when a source's URL, its
//! cache naming or its parser changes — it is the only thing that catches the
//! class this feature has hit twice, where every offline test passes and the
//! real download yields nothing.
//!
//! ```sh
//! SDROXIDE_CONFIG_DIR=/tmp/aoki-live \
//!   cargo test -p sdroxide-config --test aoki_live_fetch -- --ignored --nocapture
//! ```
#[test]
#[ignore = "needs the network"]
fn aoki_downloads_from_the_url_its_index_names() {
    let rows = sdroxide_config::fetch_broadcast_schedule_from(sdroxide_types::ScheduleSource::Aoki)
        .expect("Aoki must download");
    assert!(rows.len() > 1000, "got {} rows", rows.len());
    assert!(
        rows.iter().any(|r| r.source == sdroxide_types::ScheduleSource::Aoki),
        "rows are tagged with their source"
    );
    println!("AOKI ROWS: {}", rows.len());
}
