//! Who may move the antenna, and what happens when two things want to.
//!
//! The rotator has one motor and several would-be drivers: the satellite lock,
//! which recomputes geometry every 200 ms, and the operator, who points by hand.
//! Someone has to own it, and the distinction is what keeps a point the operator
//! made from being undone by the next tracking tick — a control that silently
//! does nothing, which this fork treats as a bug in its own right rather than a
//! layout detail.
//!
//! The daemon is a mock `rotctld` that records every `P` / `S` it is sent, so
//! the claim under test is what actually reached the wire.
//!
//! # One test, not two
//!
//! Both scenarios would set `SDROXIDE_CONFIG_DIR`, which is process-global, so
//! two tests in one binary race each other's engine. The second scenario uses
//! the directory the first left behind, which is fine — its point is that it
//! configures nothing.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::time::{Duration, Instant};

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{
    Command, DeviceCaps, RadioEvent, RotatorAuthority, RotatorConfig, SatLockConfig, SatUplink, Vfo,
};

const DIAL: f64 = 14_074_000.0;

/// AO-7, the same set `tr_switch` locks — a real satellite with a published
/// orbit, so the engine computes a real track from it. Whether it happens to be
/// above the observer's horizon at the moment the test runs does not matter:
/// the lock's branch is walked either way, and *both* of its answers put
/// something on the wire, which is what makes the hold below observable.
fn ao7_tle() -> (String, String) {
    (
        "1 07530U 74089B   26205.50898980 -.00000033  00000+0  81693-4 0  9992".into(),
        "2 07530 101.9909 219.1448 0012602  61.1135  93.5622 12.53698681365193".into(),
    )
}

/// A one-connection rotctld impersonation: answers `P` and `S` with success,
/// `p` with a fixed position, and sends every command line it saw down the
/// channel for the test to inspect.
fn mock_rotctld(rprt: &'static str) -> (u16, mpsc::Receiver<String>) {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let port = listener.local_addr().unwrap().port();
    let (tx, rx) = mpsc::channel();
    std::thread::spawn(move || {
        let (stream, _) = match listener.accept() {
            Ok(s) => s,
            Err(_) => return,
        };
        let mut w = stream.try_clone().unwrap();
        let r = BufReader::new(stream);
        for line in r.lines() {
            let Ok(line) = line else { return };
            let t = line.trim().to_string();
            if t == "p" {
                let _ = w.write_all(b"123.40\n45.60\n");
            } else if t.starts_with("P ") || t == "S" {
                let _ = tx.send(t);
                let _ = w.write_all(format!("RPRT {rprt}\n").as_bytes());
            }
        }
    });
    (port, rx)
}

/// A source that produces silence. The rotator is the subject here, so the
/// radio itself never has to do anything.
struct Silent;
impl IqSource for Silent {
    fn sample_rate(&self) -> f64 {
        48_000.0
    }
    fn center_hz(&self) -> f64 {
        DIAL
    }
    fn set_center_hz(&mut self, _hz: f64) -> Result<()> {
        Ok(())
    }
    fn describe(&self) -> String {
        "silent".into()
    }
    fn read(&mut self, out: &mut [Complex32]) -> Result<usize> {
        for c in out.iter_mut() {
            *c = Complex32::new(0.0, 0.0);
        }
        Ok(out.len())
    }
}

/// Bring an engine up with a rotator wired to `port` and a live AO-7 lock that
/// is allowed to steer it. Returns the command sender and the mock's channel.
fn engine_with_lock(
    port: u16,
    seen: mpsc::Receiver<String>,
) -> (crossbeam_channel::Sender<Command>, mpsc::Receiver<String>) {
    let dir = std::env::temp_dir().join(format!("sdroxide-rotator-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("scratch dir");
    // SAFETY: this is the only test in this binary; nothing races the setter.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &dir) };
    let h = start_engine(
        Box::new(Silent),
        DeviceCaps::default(),
        EngineConfig { ..Default::default() },
    );
    h.cmd_tx.send(Command::SetVfo { vfo: Vfo::A, hz: DIAL }).unwrap();
    h.cmd_tx
        .send(Command::SetRotatorConfig(RotatorConfig {
            enabled: true,
            host: "127.0.0.1".into(),
            port,
            ..Default::default()
        }))
        .unwrap();
    h.cmd_tx
        .send(Command::SetSatLock(Some(Box::new(SatLockConfig {
            norad_id: 7530,
            name: "OSCAR 7 (AO-7)".into(),
            tle: Some(ao7_tle()),
            observer: Some((48.2, 16.4)),
            downlink_hz: 145_950_000.0,
            uplink: Some(SatUplink {
                up_lo_hz: 432_125_000.0,
                up_hi_hz: 432_175_000.0,
                down_lo_hz: 145_925_000.0,
                down_hi_hz: 145_975_000.0,
                inverting: true,
            }),
            doppler: false,
            ctcss_hz: None,
            rotator: true,
        }))))
        .unwrap();
    (h.cmd_tx, seen)
}

fn wait_for(mut ok: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(8);
    while !ok() {
        assert!(Instant::now() < deadline, "timed out waiting for the engine");
        std::thread::sleep(Duration::from_millis(10));
    }
}

fn recv(rx: &mpsc::Receiver<String>) -> String {
    rx.recv_timeout(Duration::from_secs(8)).expect("a command")
}

#[test]
fn the_rotator_has_one_owner_and_refuses_when_absent() {
    // ── a live satellite lock must not drag the antenna off a manual point ──
    {
        let (port, seen) = mock_rotctld("0");
        let (cmd_tx, seen) = engine_with_lock(port, seen);

        // The operator points by hand. This is the point under test.
        cmd_tx.send(Command::PointRotator { az: 217.0, el: 43.0 }).unwrap();
        assert_eq!(recv(&seen), "P 217.00 43.00");

        // And the live lock must not drag the antenna off it. `drive_rotator`
        // runs every 200 ms and, with the lock active, its branch always puts
        // something on the wire — the satellite's own target when the bird is
        // up, a park when it is down. So a second and a half of silence is the
        // claim: without an authority model the very next tick would answer the
        // operator's point with one of those.
        //
        // Which of the two the old code would have sent is deliberately not
        // asserted — it depends on where AO-7 is over Central Europe at the
        // moment, and a test that fails on a Tuesday is not a test.
        let mut overrides = Vec::new();
        let deadline = Instant::now() + Duration::from_millis(1500);
        while let Ok(c) = seen.recv_timeout(deadline.saturating_duration_since(Instant::now())) {
            overrides.push(c);
        }
        assert!(
            overrides.is_empty(),
            "a live satellite lock overrode a manual point on the next tracking tick: {overrides:?}"
        );

        // Handing ownership back ends the manual hold. The lock's answer — a
        // target or a park — proves the operator's 217/43 is no longer what the
        // engine is sending. Either is acceptable, for the same orbital reason
        // as above.
        cmd_tx.send(Command::SetRotatorAuthority(RotatorAuthority::Auto)).unwrap();
        let resumed = recv(&seen);
        assert!(
            resumed.starts_with('P') || resumed == "S",
            "handing control back did nothing — the manual hold is a dead end, got {resumed:?}"
        );
        assert_ne!(
            resumed, "P 217.00 43.00",
            "the engine kept sending the operator's target after the hold was released"
        );
    }

    // ── a point with no rotator configured is refused, not silently ignored ──
    {
        let h = start_engine(
            Box::new(Silent),
            DeviceCaps::default(),
            EngineConfig { ..Default::default() },
        );
        h.cmd_tx.send(Command::PointRotator { az: 90.0, el: 10.0 }).unwrap();
        h.cmd_tx.send(Command::SetVfo { vfo: Vfo::A, hz: DIAL }).unwrap();
        // The engine is still answering commands, so the point did not wedge it.
        wait_for(|| h.cmd_tx.send(Command::SetVfo { vfo: Vfo::A, hz: DIAL + 1.0 }).is_ok());
        // And no rotator status ever claims a connection that does not exist.
        while let Ok(ev) = h.event_rx.try_recv() {
            if let RadioEvent::RotatorStatus { connected, .. } = ev {
                assert!(
                    !connected,
                    "a rotator that was never configured reported itself connected"
                );
            }
        }
    }
}
