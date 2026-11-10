//! The KISS server against Direwolf's own client.
//!
//! `kissutil` ships with Direwolf and describes itself as a "Utility for
//! testing a KISS TNC". It speaks TNC2 monitor format on its stdin and stdout
//! and KISS on the socket, so pointing it at our server exercises our framing,
//! our escaping and our command handling against an implementation nobody here
//! wrote — the same argument that makes `atest` the real test of the modem.
//!
//! # What this does *not* cover
//!
//! `kissutil` sends and receives frames; it has no connected-mode state
//! machine. So this proves the KISS server and the AX.25 frame encoding, and
//! says nothing about SABM/UA/I/RR sequencing. The proper check for that is the
//! Linux kernel's AX.25 stack via `kissattach` + `axcall`, which **cannot run on
//! this machine**: kernel 7.1.8-arch1 ships no `ax25` module anywhere in
//! `/lib/modules`, `modules.dep` has no reference to it, and there is no
//! `/proc/net/ax25`, so `AF_AX25` sockets do not exist. Installing `ax25-tools`
//! would not help. Until a kernel with AX.25 is available, connected mode is
//! covered only by the vendored upstream tests and our own two-station
//! loopback — both written in this tree, which is exactly the weakness this
//! file exists to avoid elsewhere.

use std::io::{BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use sdroxide_kiss::{KissRequest, KissServer};

fn have_kissutil() -> bool {
    Command::new("kissutil").arg("-p").output().is_ok()
}

/// Wait for something, or give up.
fn wait_for<T>(mut f: impl FnMut() -> Option<T>) -> Option<T> {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if let Some(v) = f() {
            return Some(v);
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    None
}

/// A frame Direwolf built, arriving through our server.
#[test]
fn direwolfs_client_can_send_us_a_frame() {
    if !have_kissutil() {
        eprintln!("kissutil not installed — skipping the Direwolf KISS check");
        return;
    }
    let server = KissServer::start("127.0.0.1:0").expect("bind");
    let port = server.addr().rsplit(':').next().unwrap().to_string();

    let mut child = Command::new("kissutil")
        .args(["-h", "127.0.0.1", "-p", &port])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawning kissutil");

    // Let it connect before writing.
    wait_for(|| (server.clients() > 0).then_some(())).expect("kissutil never connected");

    let mut stdin = child.stdin.take().expect("stdin");
    writeln!(stdin, "OE3JJS-10>APDW12:hello from direwolf").unwrap();
    stdin.flush().unwrap();

    let frame = wait_for(|| {
        server.poll().into_iter().find_map(|r| match r {
            KissRequest::Send(f) => Some(f),
            _ => None,
        })
    });
    let _ = child.kill();
    let frame = frame.expect("no frame arrived from kissutil");

    // Decode far enough to prove it is the frame Direwolf meant to send: the
    // addresses are shifted left one bit, and the payload follows the control
    // and PID bytes.
    let call = |b: &[u8]| -> String {
        b[..6].iter().map(|&c| (c >> 1) as char).collect::<String>().trim_end().to_string()
    };
    assert!(frame.len() > 16, "runt frame: {frame:?}");
    assert_eq!(call(&frame[0..7]), "APDW12", "destination");
    assert_eq!(call(&frame[7..14]), "OE3JJS", "source");
    assert_eq!((frame[13] >> 1) & 0x0F, 10, "source SSID");
    assert_eq!(frame[14], 0x03, "UI control byte");
    assert_eq!(frame[15], 0xF0, "no layer 3");
    assert_eq!(
        String::from_utf8_lossy(&frame[16..]),
        "hello from direwolf",
        "payload did not survive"
    );
}

/// A frame heard on the air, arriving at Direwolf's client.
#[test]
fn direwolfs_client_can_read_a_frame_from_us() {
    if !have_kissutil() {
        eprintln!("kissutil not installed — skipping the Direwolf KISS check");
        return;
    }
    let server = KissServer::start("127.0.0.1:0").expect("bind");
    let port = server.addr().rsplit(':').next().unwrap().to_string();

    let mut child = Command::new("kissutil")
        .args(["-h", "127.0.0.1", "-p", &port])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawning kissutil");
    wait_for(|| (server.clients() > 0).then_some(())).expect("kissutil never connected");

    // Build the UI frame by hand, as the deframer would hand it over: shifted
    // callsigns, end-of-address on the source, control 0x03, PID 0xF0.
    let enc = |call: &str, ssid: u8, last: bool, hi: bool| -> Vec<u8> {
        let mut v: Vec<u8> =
            call.bytes().chain(std::iter::repeat(b' ')).take(6).map(|c| c << 1).collect();
        v.push((ssid << 1) | 0b0110_0000 | u8::from(last) | if hi { 0x80 } else { 0 });
        v
    };
    let mut frame = enc("APRS", 0, false, true);
    frame.extend(enc("OE3JJS", 1, true, false));
    frame.push(0x03);
    frame.push(0xF0);
    frame.extend_from_slice(b"heard on the air");

    // The client may still be settling; repeat until it prints or we give up.
    let mut out = BufReader::new(child.stdout.take().expect("stdout"));
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        let mut line = String::new();
        while out.read_line(&mut line).unwrap_or(0) > 0 {
            if tx.send(line.clone()).is_err() {
                return;
            }
            line.clear();
        }
    });

    let mut seen = None;
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline && seen.is_none() {
        server.broadcast(&frame);
        while let Ok(line) = rx.recv_timeout(Duration::from_millis(200)) {
            if line.contains("OE3JJS-1") && line.contains("heard on the air") {
                seen = Some(line);
                break;
            }
        }
    }
    let _ = child.kill();
    let line = seen.expect("Direwolf's client never printed our frame");
    assert!(
        line.contains("OE3JJS-1>APRS"),
        "Direwolf read the addresses differently than we wrote them: {line}"
    );
}
