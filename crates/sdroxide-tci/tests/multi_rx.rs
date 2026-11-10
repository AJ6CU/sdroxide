//! Two receivers on one TCI connection — the SunSDR2DX arrangement.
//!
//! A scripted rig announces `trx_count:2` and streams distinct IQ for each
//! receiver; the client must route every frame to the ring of the receiver it
//! names, refuse receivers the rig never announced (and double-attachments),
//! and detach one stream without so much as pausing the other.
//!
//! Ports are in the 50091.. range (loopback holds 50051.., handshake 50081..);
//! a test whose port is unavailable skips rather than fails.

use std::net::TcpListener;
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender};
use sdroxide_tci::TciDevice;
use sdroxide_tci::protocol as p;
use tungstenite::Message;

const IQ_RATE: f64 = 96_000.0;

/// Fill values that mark which stream (and which act) a frame belongs to.
const RX0_FILL: f32 = 0.25;
const RX1_FILL: f32 = 0.75;
const RX0_AFTER_DETACH_FILL: f32 = 0.5;

fn bind(port: u16) -> Option<TcpListener> {
    match TcpListener::bind(("127.0.0.1", port)) {
        Ok(l) => Some(l),
        Err(e) => {
            eprintln!("skip: cannot bind 127.0.0.1:{port} ({e})");
            None
        }
    }
}

/// A scripted two-receiver rig. Every text command the client sends lands on
/// `cmds` for the test to assert on; an `iq_start` is answered with a burst of
/// frames for that receiver, and `iq_stop:1` with one more receiver-0 frame —
/// the proof that detaching one stream leaves the other flowing.
fn serve_two_rx(listener: TcpListener, cmds: Sender<String>) {
    let (stream, _) = listener.accept().expect("accept");
    let mut ws = tungstenite::accept(stream).expect("server handshake");
    ws.send(Message::Text(
        "device:TestDuo;protocol:ExpertSDR3,1.9;trx_count:2;drive:40;ready;".into(),
    ))
    .expect("send burst");
    ws.get_ref().set_read_timeout(Some(Duration::from_millis(20))).ok();

    let frame = |rx: u32, fill: f32| {
        let iq = vec![fill; 128];
        Message::Binary(p::build_iq(IQ_RATE as u32, rx, &iq).into())
    };
    loop {
        match ws.read() {
            Ok(Message::Text(t)) => {
                for cmd in t.as_str().split(';').filter(|c| !c.trim().is_empty()) {
                    let cmd = cmd.trim().to_string();
                    // A send failing means the client hung up (the test is
                    // tearing down); a real rig stops streaming, so does this
                    // one — the commands seen so far are already on `cmds`.
                    let sent = match cmd.as_str() {
                        "iq_start:0" => (0..3).all(|_| ws.send(frame(0, RX0_FILL)).is_ok()),
                        "iq_start:1" => (0..3).all(|_| ws.send(frame(1, RX1_FILL)).is_ok()),
                        "iq_stop:1" => ws.send(frame(0, RX0_AFTER_DETACH_FILL)).is_ok(),
                        _ => true,
                    };
                    let _ = cmds.send(cmd);
                    if !sent {
                        return;
                    }
                }
            }
            Ok(Message::Close(_)) => return,
            Ok(_) => {}
            Err(tungstenite::Error::Io(e))
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut => {}
            Err(_) => return,
        }
    }
}

/// Wait until the rig has seen `wanted`, or fail with what it did see.
fn expect_cmd(cmds: &Receiver<String>, wanted: &str) {
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut seen = Vec::new();
    while Instant::now() < deadline {
        while let Ok(c) = cmds.try_recv() {
            if c == wanted {
                return;
            }
            seen.push(c);
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    panic!("rig never received {wanted:?}; it saw {seen:?}");
}

/// Wait until `rx` yields a sample equal to `fill`.
fn expect_fill(rx: &mut sdroxide_tci::TciRx, fill: f32, what: &str) {
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut buf = [0.0f32; 256];
    let mut seen = Vec::new();
    while Instant::now() < deadline {
        let n = rx.rx_read(&mut buf);
        for &v in &buf[..n] {
            if (v - fill).abs() < 1e-6 {
                return;
            }
            if seen.len() < 8 && !seen.contains(&v) {
                seen.push(v);
            }
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    panic!("{what}: never saw fill {fill}; distinct values seen: {seen:?}");
}

/// Everything of the arrangement in one scripted session: the count is
/// honoured, the frames route by receiver, and streams come and go
/// independently. One test rather than several because the scripted rig is a
/// single sequential conversation.
#[test]
fn two_receivers_share_one_connection_and_detach_independently() {
    let Some(listener) = bind(50091) else { return };
    let (cmd_tx, cmds) = crossbeam_channel::unbounded();
    let server = std::thread::spawn(move || serve_two_rx(listener, cmd_tx));

    let dev = TciDevice::connect("127.0.0.1:50091", IQ_RATE).expect("connect");
    assert_eq!(dev.device(), "TestDuo");
    assert_eq!(dev.trx_count(), 2);

    // A receiver the rig never announced is refused with the rig's count.
    let err = dev.rx(2).expect_err("rx 2 of 2 must be refused").to_string();
    assert!(err.contains("2 receiver"), "the refusal should carry the count: {err}");

    let mut r0 = dev.rx(0).expect("attach rx 0");
    let mut r1 = dev.rx(1).expect("attach rx 1");
    expect_cmd(&cmds, "rx_enable:0,true");
    expect_cmd(&cmds, "iq_start:0");
    expect_cmd(&cmds, "rx_enable:1,true");
    expect_cmd(&cmds, "iq_start:1");

    // A receiver cannot be attached twice — two engines draining one ring
    // would each get half the samples.
    let err = dev.rx(1).expect_err("duplicate rx 1 must be refused").to_string();
    assert!(err.contains("already"), "{err}");

    // Each receiver's frames land in its own ring, not the other's.
    expect_fill(&mut r0, RX0_FILL, "receiver 0");
    expect_fill(&mut r1, RX1_FILL, "receiver 1");

    // Tuning is per receiver: retuning RX2 names receiver 1 on the wire.
    r1.set_center(7_100_000.0);
    expect_cmd(&cmds, "dds:1,7100000");

    // Detaching RX2 stops *its* stream — and only the stream: the rig's
    // receiver is never disabled (toggling a SunSDR2DX's second receiver off
    // under its operator is what left streams stalled in the field).
    drop(r1);
    expect_cmd(&cmds, "iq_stop:1");
    std::thread::sleep(Duration::from_millis(200));
    while let Ok(c) = cmds.try_recv() {
        assert_ne!(c, "rx_enable:1,false", "a detach must not switch the rig's receiver off");
    }
    // Receiver 0 streams on: the frame the rig sent after the detach
    // arrives, and the receiver can be attached again.
    expect_fill(&mut r0, RX0_AFTER_DETACH_FILL, "receiver 0 after rx 1 detached");
    assert!(r0.is_alive());
    let r1_again = dev.rx(1).expect("re-attach rx 1 after detach");
    drop(r1_again);

    drop(r0);
    drop(dev);
    server.join().expect("server thread");
}

/// A rig that quietly stops a stream while the WebSocket stays healthy. This
/// scripted one never answers the first `iq_start:0` with data; the client's
/// watchdog must notice the silence and ask again — and the second ask is the
/// one this rig honours.
fn serve_stalled_rx(listener: TcpListener, cmds: Sender<String>) {
    let (stream, _) = listener.accept().expect("accept");
    let mut ws = tungstenite::accept(stream).expect("server handshake");
    ws.send(Message::Text("device:TestSolo;protocol:ExpertSDR3,1.9;trx_count:1;ready;".into()))
        .expect("send burst");
    ws.get_ref().set_read_timeout(Some(Duration::from_millis(20))).ok();

    let mut starts = 0u32;
    loop {
        match ws.read() {
            Ok(Message::Text(t)) => {
                for cmd in t.as_str().split(';').filter(|c| !c.trim().is_empty()) {
                    let cmd = cmd.trim().to_string();
                    if cmd == "iq_start:0" {
                        starts += 1;
                        if starts >= 2 {
                            let iq = vec![RX0_FILL; 128];
                            let frame = Message::Binary(p::build_iq(IQ_RATE as u32, 0, &iq).into());
                            if ws.send(frame).is_err() {
                                return;
                            }
                        }
                    }
                    let _ = cmds.send(cmd);
                }
            }
            Ok(Message::Close(_)) => return,
            Ok(_) => {}
            Err(tungstenite::Error::Io(e))
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut => {}
            Err(_) => return,
        }
    }
}

#[test]
fn a_stalled_stream_is_kicked_until_it_flows() {
    let Some(listener) = bind(50092) else { return };
    let (cmd_tx, cmds) = crossbeam_channel::unbounded();
    let server = std::thread::spawn(move || serve_stalled_rx(listener, cmd_tx));

    let dev = TciDevice::connect("127.0.0.1:50092", IQ_RATE).expect("connect");
    let mut r0 = dev.rx(0).expect("attach rx 0");
    // The attach asks once; the watchdog, seeing only silence, must ask
    // again (~2 s later — `expect_cmd` waits up to 5).
    expect_cmd(&cmds, "iq_start:0");
    expect_cmd(&cmds, "iq_start:0");
    // The rig honours the second ask, and the stream flows.
    expect_fill(&mut r0, RX0_FILL, "receiver 0 after the watchdog kick");

    drop(r0);
    drop(dev);
    server.join().expect("server thread");
}
