//! Live end-to-end check of the spot manager against the real POTA feed.
//! Ignored by default (needs network); run with:
//!   cargo test -p sdroxide-net --test live_feeds -- --ignored --nocapture

use std::time::{Duration, Instant};

use sdroxide_net::{NetEvent, SpotManager};
use sdroxide_types::{FeedConfig, NetworkConfig};

#[test]
#[ignore = "hits the live POTA API"]
fn pota_feed_delivers_spots() {
    let mut mgr = SpotManager::new();
    let cfg = NetworkConfig {
        pota: FeedConfig { enabled: true, interval_secs: 60 },
        ..Default::default()
    };
    mgr.set_config(cfg);

    let deadline = Instant::now() + Duration::from_secs(30);
    let mut count = 0usize;
    let mut status = String::new();
    while Instant::now() < deadline && count == 0 {
        for ev in mgr.poll() {
            match ev {
                NetEvent::Spots(s) => count = s.len(),
                NetEvent::Status(Some(msg)) => status = msg,
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(250));
    }
    println!("POTA spots received: {count}; last status: {status:?}");
    assert!(count > 0, "expected POTA spots from the live feed (status: {status:?})");
}
