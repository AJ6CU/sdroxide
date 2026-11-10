//! Live end-to-end check of the RBN reader against the real skimmer feed.
//! Ignored by default (needs network *and* a real callsign); run with:
//!   SDROXIDE_TEST_CALL=OE1XXX \
//!     cargo test -p sdroxide-net --test live_rbn -- --ignored --nocapture
//!
//! The callsign comes from the environment rather than a constant on purpose:
//! RBN wants a real one, and checking somebody's in is not something a test
//! file should do on its behalf.

use std::time::{Duration, Instant};

use sdroxide_net::{NetEvent, SpotManager};
use sdroxide_types::{NetworkConfig, RbnConfig};

#[test]
#[ignore = "connects to the live Reverse Beacon Network and logs in with a real callsign"]
fn rbn_delivers_paths() {
    let Ok(call) = std::env::var("SDROXIDE_TEST_CALL") else {
        eprintln!("set SDROXIDE_TEST_CALL to your callsign to run this");
        return;
    };

    let mut mgr = SpotManager::new();
    mgr.set_operator(&call, "");
    mgr.set_config(NetworkConfig {
        rbn: RbnConfig { enabled: true, ..Default::default() },
        ..Default::default()
    });

    // One flush interval is five seconds; allow for the login handshake too.
    let deadline = Instant::now() + Duration::from_secs(45);
    let mut paths = Vec::new();
    let mut status = String::new();
    while Instant::now() < deadline && paths.is_empty() {
        for ev in mgr.poll() {
            match ev {
                NetEvent::PropPaths(p) => paths = p,
                NetEvent::Status(Some(msg)) => status = msg,
                _ => {}
            }
        }
        std::thread::sleep(Duration::from_millis(250));
    }

    println!("RBN paths in the first batch: {}; last status: {status:?}", paths.len());
    for p in paths.iter().take(5) {
        println!(
            "  {} — {:.1} MHz, {:?} dB, tx {:.1},{:.1} → rx {:.1},{:.1}",
            p.key,
            p.freq_hz / 1e6,
            p.snr_db,
            p.tx.0,
            p.tx.1,
            p.rx.0,
            p.rx.1
        );
    }
    assert!(!paths.is_empty(), "expected paths from the live RBN feed (status: {status:?})");
    // Every path must be usable: a band the map has a plane for, and two
    // positions that are not the null island a failed lookup would give.
    for p in &paths {
        assert_ne!(
            sdroxide_types::Band::containing(p.freq_hz),
            sdroxide_types::Band::Gen,
            "{} landed outside every amateur band at {} Hz",
            p.key,
            p.freq_hz
        );
        assert!(p.tx != (0.0, 0.0) && p.rx != (0.0, 0.0), "{} was not placed", p.key);
    }
}
