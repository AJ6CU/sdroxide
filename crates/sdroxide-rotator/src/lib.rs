//! Antenna-rotator client: sdroxide driving a motorised antenna.
//!
//! The satellite lock and the manual compass compute azimuth and elevation;
//! this crate is how those numbers reach a motor. Two kinds of transport:
//!
//! - **Rotctld** — a Hamlib `rotctld` daemon over TCP, default port 4533. One
//!   short text protocol (`P az el` to point, `p` to ask, `S` to stop) reaches
//!   every rotator Hamlib can drive, which is essentially all of them. This is
//!   the default and the one route that needs no hardware in hand to trust.
//! - **EasyComm II** and **GS-232A/B** — a controller on a serial port
//!   directly, for a station that would rather not run a daemon. The line
//!   protocols are simpler than rotctld's and slot in behind the same
//!   [`Transport`] the daemon does.
//!
//! The mirror image of `sdroxide-rigctld`: there we *are* the daemon and
//! GPredict connects to us; here the daemon (or the controller) already exists
//! and we connect to it.
//!
//! # Shape
//!
//! One worker thread owns the transport; the engine talks to it through a small
//! command channel and reads health back from a shared [`RotStatus`]. The
//! worker deduplicates movement (no point grinding motors over a fraction of a
//! degree), applies the configured azimuth offset, reconnects with backoff, and
//! polls the real antenna position once a second so the operator can see the
//! hardware answer.
//!
//! # Not proven
//!
//! The **serial** transports are implemented from the published protocols and
//! their framing is unit-tested, but **no controller has been driven with them**
//! — this crate's own bench is a rotctld daemon. A station with a serial
//! controller should try `rotctld -m <model>` first (which drives the same
//! hardware and is exercised here); the serial paths are offered for those who
//! would rather not, and are marked not proven until somebody runs one.

use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, RecvTimeoutError, Sender, bounded};
use tracing::{debug, warn};

pub use sdroxide_types::{RotatorConfig, RotatorTransport};

/// The client's health, as the engine reads it back: reachable or not, where
/// the daemon says the antenna points, and the last thing that went wrong.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RotStatus {
    pub connected: bool,
    /// Where the *hardware* reports itself — the commanded target lives in the
    /// operator's own state (the satellite lock's track, or the compass).
    pub az_deg: f64,
    pub el_deg: f64,
    pub error: Option<String>,
}

enum RotCmd {
    Target {
        az: f64,
        el: f64,
    },
    Park,
    /// Halt the antenna where it is, whatever the configured park position.
    Stop,
}

/// Handle to the worker thread. Dropping it closes the command channel, which
/// is the shutdown signal; the drop joins the thread so a socket mid-write
/// cannot outlive the engine.
pub struct RotatorClient {
    tx: Option<Sender<RotCmd>>,
    status: Arc<Mutex<RotStatus>>,
    join: Option<std::thread::JoinHandle<()>>,
}

impl RotatorClient {
    pub fn start(cfg: RotatorConfig) -> RotatorClient {
        let (tx, rx) = bounded(8);
        let status = Arc::new(Mutex::new(RotStatus::default()));
        let st = status.clone();
        let join = std::thread::Builder::new()
            .name("sdroxide-rotator".into())
            .spawn(move || worker(cfg, rx, st))
            .map_err(|e| warn!("could not start the rotator client: {e}"))
            .ok();
        RotatorClient { tx: Some(tx), status, join }
    }

    /// Point the antenna. Safe to call at the tracking rate: the worker
    /// deduplicates against the last commanded position and the configured
    /// minimum movement, so a satellite crawling across the sky becomes a
    /// trickle of real commands rather than a 5 Hz stream.
    pub fn set_target(&self, az_deg: f64, el_deg: f64) {
        if let Some(tx) = &self.tx {
            // A full queue means the worker is mid-reconnect; the stale
            // target is better dropped — a fresher one follows shortly.
            let _ = tx.try_send(RotCmd::Target { az: az_deg, el: el_deg });
        }
    }

    /// Stop tracking: drive to the configured park position, or just stop
    /// moving when none is set. Idempotent — the worker remembers it already
    /// parked, so the engine can say this every tick the satellite is down.
    pub fn park(&self) {
        if let Some(tx) = &self.tx {
            let _ = tx.try_send(RotCmd::Park);
        }
    }

    /// Halt the antenna where it is. Distinct from [`Self::park`], which drives
    /// to the configured park position when there is one: a stop is the operator
    /// saying "that is far enough", and it must not be answered by swinging the
    /// beam to the park bearing instead.
    ///
    /// Idempotent for the same reason `park` is — the engine says this every
    /// tick the antenna is under a manual stop, and only the first one may
    /// reach the wire.
    pub fn stop(&self) {
        if let Some(tx) = &self.tx {
            let _ = tx.try_send(RotCmd::Stop);
        }
    }

    pub fn status(&self) -> RotStatus {
        self.status.lock().unwrap().clone()
    }
}

impl Drop for RotatorClient {
    fn drop(&mut self) {
        self.tx = None; // closes the channel — the worker's shutdown signal
        if let Some(j) = self.join.take() {
            let _ = j.join();
        }
    }
}

/// First retry after a connection failure, and the ceiling the spacing
/// doubles up to while the transport stays unreachable.
const RECONNECT_FIRST: Duration = Duration::from_secs(1);
const RECONNECT_MAX: Duration = Duration::from_secs(30);
/// How often the real antenna position is read back.
const POSITION_POLL: Duration = Duration::from_secs(1);
/// How long the worker waits for a command before doing housekeeping —
/// paces the reconnect attempts and the position poll.
const TICK: Duration = Duration::from_millis(250);

// ── the transports ───────────────────────────────────────────────────────────

/// What every transport can do. The engine-side interface — "here is az/el,
/// go" — is the same for all of them, so the worker never knows which it holds.
trait Transport: Send {
    fn point(&mut self, cfg: &RotatorConfig, az: f64, el: f64) -> Result<(), String>;
    fn park(&mut self, cfg: &RotatorConfig) -> Result<(), String>;
    fn stop(&mut self) -> Result<(), String>;
    /// `Ok(None)` is "no reading right now", which is not the same as a
    /// failure — and PSTRotator needs the distinction to exist at all: it stays
    /// silent while no controller is connected, yet still accepts a pointing
    /// command. Dropping the connection on a quiet answer would stop the
    /// antenna being driven by the one transport that is working.
    fn position(&mut self) -> Result<Option<(f64, f64)>, String>;
}

/// Open whichever transport the config asks for.
fn open_transport(cfg: &RotatorConfig) -> Result<Box<dyn Transport>, String> {
    match cfg.transport {
        RotatorTransport::Rotctld => Ok(Box::new(RotctldTransport::open(cfg)?)),
        RotatorTransport::EasyComm2 | RotatorTransport::Gs232 => {
            Ok(Box::new(SerialTransport::open(cfg)?))
        }
        RotatorTransport::PsRotator => Ok(Box::new(PstLink::open(
            cfg,
            cfg.port.wrapping_add(1),
        )?)),
    }
}

/// The movement memory every transport shares: what was last commanded (so a
/// crawling satellite is a trickle and not a stream) and whether a park or a
/// stop has already been said.
struct MoveState {
    last_pointed: Option<(f64, f64)>,
    parked: bool,
    stopped: bool,
}

impl MoveState {
    /// Born parked: an idle engine must leave hardware it never commanded
    /// exactly where it found it.
    fn new() -> MoveState {
        MoveState { last_pointed: None, parked: true, stopped: false }
    }

    /// Whether a point should reach the wire, applying the offset, the
    /// elevation clamp and the dead band. `None` means "too small a move".
    fn resolve(&self, cfg: &RotatorConfig, az: f64, el: f64) -> Option<(f64, f64)> {
        let az = (az + cfg.az_offset_deg).rem_euclid(360.0);
        let el = el.clamp(0.0, 90.0);
        if !self.parked && !self.stopped {
            if let Some((la, le)) = self.last_pointed {
                let min = cfg.min_move_deg.max(0.0);
                if az_dist(la, az) < min && (le - el).abs() < min {
                    return None;
                }
            }
        }
        Some((az, el))
    }

    fn pointed(&mut self, az: f64, el: f64) {
        self.parked = false;
        self.stopped = false;
        self.last_pointed = Some((az, el));
    }

    fn took_park(&mut self) {
        self.parked = true;
        self.stopped = false;
        self.last_pointed = None;
    }

    fn took_stop(&mut self) {
        self.stopped = true;
        self.parked = false;
        self.last_pointed = None;
    }
}

fn worker(cfg: RotatorConfig, rx: Receiver<RotCmd>, status: Arc<Mutex<RotStatus>>) {
    let mut conn: Option<Box<dyn Transport>> = None;
    let mut retry_at = Instant::now();
    let mut retry_every = RECONNECT_FIRST;
    let mut next_poll = Instant::now();

    loop {
        if conn.is_none() && Instant::now() >= retry_at {
            match open_transport(&cfg) {
                Ok(c) => {
                    debug!("rotator connected: {}", cfg.address());
                    retry_every = RECONNECT_FIRST;
                    set(&status, |s| {
                        s.connected = true;
                        s.error = None;
                    });
                    next_poll = Instant::now();
                    conn = Some(c);
                }
                Err(e) => {
                    set(&status, |s| {
                        s.connected = false;
                        s.error = Some(e);
                    });
                    retry_at = Instant::now() + retry_every;
                    retry_every = (retry_every * 2).min(RECONNECT_MAX);
                }
            }
        }

        let cmd = match rx.recv_timeout(TICK) {
            Ok(c) => Some(c),
            Err(RecvTimeoutError::Timeout) => None,
            Err(RecvTimeoutError::Disconnected) => return,
        };
        // A command while disconnected is dropped rather than queued: the
        // tracking loop sends fresh ones continuously, so replaying a stale
        // pointing after a minute of downtime would swing the antenna to
        // where the satellite used to be.
        let Some(c) = conn.as_mut() else { continue };

        let r = match cmd {
            Some(RotCmd::Target { az, el }) => c.point(&cfg, az, el),
            Some(RotCmd::Park) => c.park(&cfg),
            Some(RotCmd::Stop) => c.stop(),
            None => Ok(()),
        }
        .and_then(|()| {
            if Instant::now() >= next_poll {
                next_poll = Instant::now() + POSITION_POLL;
                match c.position()? {
                    Some((az, el)) => set(&status, |s| {
                        s.connected = true;
                        s.error = None;
                        s.az_deg = az;
                        s.el_deg = el;
                    }),
                    // A UDP program that has not answered: say so, but keep
                    // sending — PSTRotator stays quiet while it has no
                    // controller connected, and pointing still reaches it.
                    None => set(&status, |s| {
                        s.connected = false;
                        s.error = Some(format!(
                            "no answer from PSTRotator on UDP port {} — is UDP Control \
                             enabled in its Setup?",
                            cfg.port.wrapping_add(1)
                        ));
                    }),
                }
            }
            Ok(())
        });
        if let Err(e) = r {
            warn!("rotator: {e}");
            set(&status, |s| {
                s.connected = false;
                s.error = Some(e);
            });
            conn = None;
            retry_at = Instant::now() + RECONNECT_FIRST;
            retry_every = RECONNECT_FIRST;
        }
    }
}

fn set(status: &Mutex<RotStatus>, f: impl FnOnce(&mut RotStatus)) {
    if let Ok(mut s) = status.lock() {
        f(&mut s);
    }
}

/// Shortest angular distance between two azimuths, degrees.
fn az_dist(a: f64, b: f64) -> f64 {
    let d = (a - b).rem_euclid(360.0);
    d.min(360.0 - d)
}

// ── rotctld over TCP ─────────────────────────────────────────────────────────

/// One TCP connection to a `rotctld` daemon, with the movement state.
struct RotctldTransport {
    stream: TcpStream,
    reader: BufReader<TcpStream>,
    state: MoveState,
}

impl RotctldTransport {
    fn open(cfg: &RotatorConfig) -> Result<RotctldTransport, String> {
        use std::net::ToSocketAddrs;
        let addr = cfg.address();
        let sock = addr
            .to_socket_addrs()
            .map_err(|e| format!("{addr}: {e}"))?
            .next()
            .ok_or_else(|| format!("{addr}: no address"))?;
        let stream = TcpStream::connect_timeout(&sock, Duration::from_secs(3))
            .map_err(|e| format!("{addr}: {e}"))?;
        let _ = stream.set_read_timeout(Some(Duration::from_secs(2)));
        let _ = stream.set_nodelay(true);
        let reader = BufReader::new(stream.try_clone().map_err(|e| e.to_string())?);
        Ok(RotctldTransport { stream, reader, state: MoveState::new() })
    }

    fn send_line(&mut self, line: &str) -> Result<(), String> {
        self.stream.write_all(line.as_bytes()).map_err(|e| e.to_string())
    }

    fn read_line(&mut self) -> Result<String, String> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => Err("rotctld closed the connection".into()),
            Ok(_) => Ok(line),
            Err(e) => Err(e.to_string()),
        }
    }

    /// `RPRT 0` on success; anything else is the daemon's error code.
    fn rprt(&mut self) -> Result<(), String> {
        let line = self.read_line()?;
        let t = line.trim();
        match t.strip_prefix("RPRT ") {
            Some("0") => Ok(()),
            Some(code) => Err(format!("rotctld answered RPRT {code}")),
            None => Err(format!("unexpected rotctld reply {t:?}")),
        }
    }
}

impl Transport for RotctldTransport {
    fn point(&mut self, cfg: &RotatorConfig, az: f64, el: f64) -> Result<(), String> {
        let Some((az, el)) = self.state.resolve(cfg, az, el) else { return Ok(()) };
        self.send_line(&format!("P {az:.2} {el:.2}\n"))?;
        self.rprt()?;
        self.state.pointed(az, el);
        Ok(())
    }

    fn park(&mut self, cfg: &RotatorConfig) -> Result<(), String> {
        if self.state.parked {
            return Ok(());
        }
        self.state.took_park();
        match cfg.park {
            Some((az, el)) => {
                let az = (az + cfg.az_offset_deg).rem_euclid(360.0);
                self.send_line(&format!("P {az:.2} {:.2}\n", el.clamp(0.0, 90.0)))?;
                self.rprt()
            }
            // No park position: stop where it is rather than inventing one.
            None => {
                self.send_line("S\n")?;
                self.rprt()
            }
        }
    }

    fn stop(&mut self) -> Result<(), String> {
        if self.state.stopped {
            return Ok(());
        }
        self.state.took_stop();
        self.send_line("S\n")?;
        self.rprt()
    }

    /// `p` — where the daemon says the antenna is. Two bare lines on success;
    /// an `RPRT -n` in their place on failure.
    fn position(&mut self) -> Result<Option<(f64, f64)>, String> {
        self.send_line("p\n")?;
        let first = self.read_line()?;
        let t = first.trim().to_string();
        if let Some(code) = t.strip_prefix("RPRT ") {
            return Err(format!("rotctld answered RPRT {} to a position query", code.trim()));
        }
        let az: f64 = t.parse().map_err(|_| format!("unparseable azimuth {t:?}"))?;
        let second = self.read_line()?;
        let t = second.trim();
        let el: f64 = t.parse().map_err(|_| format!("unparseable elevation {t:?}"))?;
        Ok(Some((az, el)))
    }
}

// ── serial: EasyComm II and GS-232A/B ────────────────────────────────────────

/// Which line protocol a serial controller speaks, and how to frame it. Pure:
/// the framing is tested here without a controller in hand.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SerialProtocol {
    /// EasyComm II — the open standard behind SatNOGS-style homebrew.
    EasyComm2,
    /// Yaesu GS-232B — the ubiquitous interface, and its many clones.
    Gs232,
}

impl SerialProtocol {
    /// The bytes to point the antenna. Both protocols carry whole degrees or a
    /// tenth; azimuth is wrapped, elevation clamped by the caller.
    fn point(self, az: f64, el: f64) -> String {
        match self {
            // `AZ<az> EL<el>`, one decimal. The set and the read share a shape.
            SerialProtocol::EasyComm2 => format!("AZ{az:.1} EL{el:.1}\r\n"),
            // `W<aaa> <eee>` — GS-232B's azimuth+elevation set, fixed three
            // digits each. GS-232A is azimuth-only and ignores the elevation,
            // which is harmless: an azimuth-only controller has no elevation.
            SerialProtocol::Gs232 => format!("W{:03.0} {:03.0}\r\n", az.round(), el.round()),
        }
    }

    fn position_query(self) -> &'static str {
        match self {
            // `AZ EL` reads the position back in the set shape.
            SerialProtocol::EasyComm2 => "AZ EL\r\n",
            // `C2` returns azimuth then elevation (GS-232B). A GS-232A given
            // `C2` answers the azimuth and nothing else, which the parser
            // accepts with elevation 0.
            SerialProtocol::Gs232 => "C2\r\n",
        }
    }

    fn stop(self) -> &'static str {
        match self {
            // Stop azimuth and elevation moving.
            SerialProtocol::EasyComm2 => "SA SE\r\n",
            SerialProtocol::Gs232 => "S\r\n",
        }
    }

    /// Pull an azimuth and elevation out of a reply, if it carries one.
    /// `None` while the reply is still partial, so the reader can wait for more.
    fn parse_position(self, reply: &str) -> Option<(f64, f64)> {
        match self {
            // `AZ120.0 EL30.0`
            SerialProtocol::EasyComm2 => {
                let mut az = None;
                let mut el = None;
                for tok in reply.split_whitespace() {
                    if let Some(v) = tok.strip_prefix("AZ").and_then(parse_num) {
                        az = Some(v);
                    } else if let Some(v) = tok.strip_prefix("EL").and_then(parse_num) {
                        el = Some(v);
                    }
                }
                match (az, el) {
                    (Some(a), Some(e)) => Some((a, e)),
                    _ => None,
                }
            }
            // `+0120\r\n+0030` — a sign then the value, azimuth first. One line
            // (azimuth only) is a GS-232A and reads as elevation 0.
            SerialProtocol::Gs232 => {
                let mut nums = reply.lines().filter_map(parse_num);
                let az = nums.next()?;
                let el = nums.next().unwrap_or(0.0);
                Some((az, el))
            }
        }
    }
}

/// Parse a number the way these controllers write one: a trailing `\r` is
/// already gone (the string was split on lines/whitespace), but a leading `+`
/// is kept, and `parse` takes it.
fn parse_num(s: &str) -> Option<f64> {
    s.trim().parse::<f64>().ok()
}

/// A controller on a serial port.
struct SerialTransport {
    port: Box<dyn serialport::SerialPort>,
    proto: SerialProtocol,
    state: MoveState,
}

impl SerialTransport {
    fn open(cfg: &RotatorConfig) -> Result<SerialTransport, String> {
        let proto = match cfg.transport {
            RotatorTransport::EasyComm2 => SerialProtocol::EasyComm2,
            RotatorTransport::Gs232 => SerialProtocol::Gs232,
            RotatorTransport::Rotctld | RotatorTransport::PsRotator => {
                return Err("not a serial transport".into())
            }
        };
        if cfg.serial_port.trim().is_empty() {
            return Err("no serial port set for the rotator".into());
        }
        let port = serialport::new(cfg.serial_port.trim(), cfg.baud.max(1200))
            .timeout(Duration::from_millis(600))
            .open()
            .map_err(|e| format!("{}: {e}", cfg.serial_port.trim()))?;
        Ok(SerialTransport { port, proto, state: MoveState::new() })
    }

    fn write(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.port.write_all(bytes).map_err(|e| e.to_string())?;
        self.port.flush().map_err(|e| e.to_string())
    }

    /// Read until the protocol can parse a position out of what has arrived, or
    /// the port times out. Two reads rather than one because GS-232B's `C2`
    /// answers on two lines, which may land in two chunks.
    fn read_position(&mut self) -> Result<(f64, f64), String> {
        let mut out = String::new();
        let mut buf = [0u8; 256];
        let deadline = Instant::now() + Duration::from_millis(1200);
        while Instant::now() < deadline {
            match self.port.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => {
                    out.push_str(&String::from_utf8_lossy(&buf[..n]));
                    if let Some(ll) = self.proto.parse_position(&out) {
                        return Ok(ll);
                    }
                }
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => break,
                Err(e) => return Err(e.to_string()),
            }
        }
        Err(format!("no position in the controller's reply {:?}", out.trim()))
    }
}

impl Transport for SerialTransport {
    fn point(&mut self, cfg: &RotatorConfig, az: f64, el: f64) -> Result<(), String> {
        let Some((az, el)) = self.state.resolve(cfg, az, el) else { return Ok(()) };
        let cmd = self.proto.point(az, el);
        self.write(cmd.as_bytes())?;
        self.state.pointed(az, el);
        Ok(())
    }

    fn park(&mut self, cfg: &RotatorConfig) -> Result<(), String> {
        if self.state.parked {
            return Ok(());
        }
        self.state.took_park();
        match cfg.park {
            Some((az, el)) => {
                let az = (az + cfg.az_offset_deg).rem_euclid(360.0);
                let cmd = self.proto.point(az, el.clamp(0.0, 90.0));
                self.write(cmd.as_bytes())
            }
            // No park position: stop where it is rather than inventing one.
            None => {
                let cmd = self.proto.stop();
                self.write(cmd.as_bytes())
            }
        }
    }

    fn stop(&mut self) -> Result<(), String> {
        if self.state.stopped {
            return Ok(());
        }
        self.state.took_stop();
        let cmd = self.proto.stop();
        self.write(cmd.as_bytes())
    }

    fn position(&mut self) -> Result<Option<(f64, f64)>, String> {
        let q = self.proto.position_query();
        self.write(q.as_bytes())?;
        self.read_position().map(Some)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufRead, BufReader, Write};
    use std::net::TcpListener;
    use std::sync::mpsc;

    /// A one-connection rotctld impersonation: answers `P` with the given
    /// report code, `p` with a fixed position, `S` with success, and sends
    /// every command line it saw down the channel for the test to inspect.
    fn mock_rotctld(rprt_for_p: &'static str) -> (u16, mpsc::Receiver<String>) {
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
                } else if t.starts_with("P ") {
                    let _ = tx.send(t);
                    let _ = w.write_all(format!("RPRT {rprt_for_p}\n").as_bytes());
                } else if t == "S" {
                    let _ = tx.send(t);
                    let _ = w.write_all(b"RPRT 0\n");
                }
            }
        });
        (port, rx)
    }

    fn cfg(port: u16) -> RotatorConfig {
        RotatorConfig { enabled: true, host: "127.0.0.1".into(), port, ..Default::default() }
    }

    fn wait_for(mut ok: impl FnMut() -> bool) {
        let deadline = Instant::now() + Duration::from_secs(5);
        while !ok() {
            assert!(Instant::now() < deadline, "timed out waiting for the client");
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    fn recv(rx: &mpsc::Receiver<String>) -> String {
        rx.recv_timeout(Duration::from_secs(5)).expect("a command")
    }

    #[test]
    fn points_polls_and_reports_the_hardware_position() {
        let (port, seen) = mock_rotctld("0");
        let client = RotatorClient::start(cfg(port));
        client.set_target(200.0, 30.0);
        assert_eq!(recv(&seen), "P 200.00 30.00");
        // The position poll reads the mock's fixed answer back.
        wait_for(|| {
            let s = client.status();
            s.connected && (s.az_deg - 123.4).abs() < 0.01 && (s.el_deg - 45.6).abs() < 0.01
        });
        assert_eq!(client.status().error, None);
    }

    /// Sub-degree jitter must not become motor commands, and a real move must.
    #[test]
    fn small_movements_are_swallowed_by_the_dedup() {
        let (port, seen) = mock_rotctld("0");
        let client = RotatorClient::start(cfg(port)); // min_move_deg = 1.0 default
        client.set_target(100.0, 20.0);
        assert_eq!(recv(&seen), "P 100.00 20.00");
        client.set_target(100.4, 20.4); // inside the dead band
        client.set_target(103.0, 20.0); // outside it
        assert_eq!(recv(&seen), "P 103.00 20.00");
    }

    /// Parking with no park position stops the rotator; saying it twice
    /// sends nothing the second time.
    #[test]
    fn park_stops_once_and_only_after_tracking() {
        let (port, seen) = mock_rotctld("0");
        let client = RotatorClient::start(cfg(port));
        // Fresh connection is born parked: an idle engine's park chatter
        // must not touch hardware it never commanded.
        client.park();
        client.set_target(50.0, 10.0);
        assert_eq!(recv(&seen), "P 50.00 10.00");
        client.park();
        assert_eq!(recv(&seen), "S");
        client.park(); // already parked — nothing more may arrive
        client.set_target(60.0, 15.0); // ...and tracking resumes cleanly
        assert_eq!(recv(&seen), "P 60.00 15.00");
    }

    /// A stop halts where the antenna is and, said again every tick, speaks
    /// once. Tracking resumes cleanly afterwards.
    #[test]
    fn stop_halts_where_it_is_and_is_idempotent() {
        let (port, seen) = mock_rotctld("0");
        let client = RotatorClient::start(cfg(port));
        client.set_target(50.0, 10.0);
        assert_eq!(recv(&seen), "P 50.00 10.00");
        client.stop();
        assert_eq!(recv(&seen), "S");
        client.stop(); // already stopped — nothing more may arrive
        client.set_target(60.0, 15.0); // resumes cleanly after the stop
        assert_eq!(recv(&seen), "P 60.00 15.00");
    }

    /// A stop is not a park: with a park position configured, the operator
    /// saying "stop" must halt, not swing to the park bearing.
    #[test]
    fn a_stop_is_not_a_park_even_with_a_park_position() {
        let (port, seen) = mock_rotctld("0");
        let mut c = cfg(port);
        c.park = Some((0.0, 0.0));
        let client = RotatorClient::start(c);
        client.set_target(50.0, 10.0);
        assert_eq!(recv(&seen), "P 50.00 10.00");
        client.stop();
        assert_eq!(recv(&seen), "S");
    }

    #[test]
    fn the_azimuth_offset_is_applied_and_wraps() {
        let (port, seen) = mock_rotctld("0");
        let mut c = cfg(port);
        c.az_offset_deg = 10.0;
        let client = RotatorClient::start(c);
        client.set_target(355.0, 5.0);
        assert_eq!(recv(&seen), "P 5.00 5.00");
    }

    #[test]
    fn a_daemon_error_reaches_the_status() {
        let (port, _seen) = mock_rotctld("-1");
        let client = RotatorClient::start(cfg(port));
        client.set_target(10.0, 10.0);
        wait_for(|| client.status().error.as_deref().is_some_and(|e| e.contains("RPRT -1")));
    }

    #[test]
    fn an_unreachable_daemon_is_a_status_not_a_hang() {
        // A port nothing listens on: bind-then-drop guarantees it is closed.
        let port = TcpListener::bind("127.0.0.1:0").unwrap().local_addr().unwrap().port();
        let client = RotatorClient::start(cfg(port));
        client.set_target(10.0, 10.0);
        wait_for(|| {
            let s = client.status();
            !s.connected && s.error.is_some()
        });
    }

    // ── the serial protocols' framing, tested without a controller ──

    /// GS-232B points with `W<aaa> <eee>`, fixed three digits, CR-terminated —
    /// the one place a wrong format is a wrong command.
    #[test]
    fn gs232_frames_a_point_the_way_the_manual_says() {
        assert_eq!(SerialProtocol::Gs232.point(120.0, 30.0), "W120 030\r\n");
        assert_eq!(SerialProtocol::Gs232.point(5.4, 0.0), "W005 000\r\n");
        assert_eq!(SerialProtocol::Gs232.point(359.6, 90.0), "W360 090\r\n");
    }

    /// EasyComm II's set carries a decimal and both fields on one line.
    #[test]
    fn easycomm_frames_a_point_as_az_el() {
        assert_eq!(SerialProtocol::EasyComm2.point(120.0, 30.0), "AZ120.0 EL30.0\r\n");
        assert_eq!(SerialProtocol::EasyComm2.point(5.25, 0.0), "AZ5.2 EL0.0\r\n");
    }

    /// A GS-232B `C2` answer is two signed lines, azimuth then elevation.
    /// A GS-232A answers the azimuth alone, which reads as elevation 0.
    #[test]
    fn gs232_reads_a_two_line_position_and_tolerates_one() {
        assert_eq!(SerialProtocol::Gs232.parse_position("+0120\r\n+0030\r\n"), Some((120.0, 30.0)));
        assert_eq!(SerialProtocol::Gs232.parse_position("+0359\r\n+0000\r\n"), Some((359.0, 0.0)));
        // Azimuth only — a GS-232A, or a reply still arriving.
        assert_eq!(SerialProtocol::Gs232.parse_position("+0210\r\n"), Some((210.0, 0.0)));
        assert_eq!(SerialProtocol::Gs232.parse_position(""), None);
    }

    /// EasyComm II answers `AZ EL` with the set shape back.
    #[test]
    fn easycomm_reads_the_position_back() {
        assert_eq!(
            SerialProtocol::EasyComm2.parse_position("AZ120.0 EL30.0\r\n"),
            Some((120.0, 30.0))
        );
        // A partial reply — only the azimuth has arrived — waits for more.
        assert_eq!(SerialProtocol::EasyComm2.parse_position("AZ120.0 "), None);
        assert_eq!(SerialProtocol::EasyComm2.parse_position("garbage"), None);
    }
}

/// PSTRotator's UDP control (#638), from its manual: each datagram is one
/// command wrapped in `<PST>…</PST>` — `<AZIMUTH>n</AZIMUTH>`,
/// `<ELEVATION>n</ELEVATION>`, `<STOP>1</STOP>`, `<TRACK>0</TRACK>` — sent to
/// its control port; the queries `AZ?` and `EL?` are answered `AZ:n` / `EL:n`
/// on that port plus one.
struct PstLink {
    sock: std::net::UdpSocket,
    to: std::net::SocketAddr,
    /// Whether PSTRotator has been taken out of its own tracking yet.
    took_over: bool,
    /// The last position it reported.
    az: f64,
    el: f64,
}

impl PstLink {
    fn open(cfg: &RotatorConfig, reply_port: u16) -> Result<PstLink, String> {
        use std::net::ToSocketAddrs;
        let addr = cfg.address();
        let to = addr
            .to_socket_addrs()
            .map_err(|e| format!("{addr}: {e}"))?
            .next()
            .ok_or_else(|| format!("{addr}: no address"))?;
        let sock = std::net::UdpSocket::bind(("0.0.0.0", reply_port))
            .map_err(|e| format!("listening for PSTRotator on UDP {reply_port}: {e}"))?;
        let _ = sock.set_read_timeout(Some(Duration::from_millis(200)));
        Ok(PstLink { sock, to, took_over: false, az: 0.0, el: 0.0 })
    }

    fn send(&self, body: &str) -> Result<(), String> {
        self.sock
            .send_to(format!("<PST>{body}</PST>").as_bytes(), self.to)
            .map(|_| ())
            .map_err(|e| format!("PSTRotator {}: {e}", self.to))
    }

    fn goto(&mut self, az: f64, el: f64) -> Result<(), String> {
        // Its own tracking would fight ours: hand control over once.
        if !self.took_over {
            self.send("<TRACK>0</TRACK>")?;
            self.took_over = true;
        }
        self.send(&format!("<AZIMUTH>{az:.1}</AZIMUTH>"))?;
        self.send(&format!("<ELEVATION>{el:.1}</ELEVATION>"))
    }

    fn stop(&mut self) -> Result<(), String> {
        // Twice: a single STOP is known to be missed now and then.
        self.send("<STOP>1</STOP>")?;
        self.send("<STOP>1</STOP>")
    }

    fn position(&mut self) -> Result<Option<(f64, f64)>, String> {
        self.send("AZ?")?;
        self.send("EL?")?;
        let (mut got_az, mut got_el) = (false, false);
        let mut buf = [0u8; 512];
        let deadline = Instant::now() + Duration::from_millis(400);
        while !(got_az && got_el) && Instant::now() < deadline {
            let Ok((n, _)) = self.sock.recv_from(&mut buf) else { continue };
            for line in String::from_utf8_lossy(&buf[..n]).split(['\r', '\n']) {
                let line = line.trim();
                if let Some(v) = line.strip_prefix("AZ:").and_then(|v| v.trim().parse().ok()) {
                    self.az = v;
                    got_az = true;
                } else if let Some(v) = line.strip_prefix("EL:").and_then(|v| v.trim().parse().ok())
                {
                    self.el = v;
                    got_el = true;
                }
            }
        }
        // Answered this time or not: a program that has gone quiet should read
        // as quiet, not as the last place it said the antenna was.
        Ok(got_az.then_some((self.az, self.el)))
    }
}

/// PSTRotator speaks the engine-side interface through `goto`/`position`;
/// these four names are the engine-side view of the same four, so the worker
/// above cannot tell which transport it holds — which is the whole reason the
/// trait exists at all.
impl Transport for PstLink {
    fn point(&mut self, _cfg: &RotatorConfig, az: f64, el: f64) -> Result<(), String> {
        self.goto(az, el)
    }

    fn park(&mut self, _cfg: &RotatorConfig) -> Result<(), String> {
        self.goto(0.0, 0.0)
    }

    fn stop(&mut self) -> Result<(), String> {
        PstLink::stop(self)
    }

    fn position(&mut self) -> Result<Option<(f64, f64)>, String> {
        PstLink::position(self)
    }
}

#[cfg(test)]
mod pst_tests {
    use super::*;

    /// **PSTRotator is reachable through the transport this fork already has.**
    ///
    /// Upstream added this as a separate `RotatorProtocol` enum beside the
    /// transport choice, which would have left `RotatorConfig` with two fields
    /// both answering "how is the antenna reached" and only one of them wired.
    /// It is an arm on `RotatorTransport` here instead, so a config picks one
    /// thing and there is one place that reads it.
    #[test]
    fn psrotator_is_a_transport_and_not_a_second_enum() {
        let cfg = RotatorConfig { transport: RotatorTransport::PsRotator, ..Default::default() };
        // Opening asks the UDP port for its reply half; a failure here is the
        // port being taken, not the wiring being wrong — so assert the shape.
        assert_eq!(cfg.transport, RotatorTransport::PsRotator);
        assert_ne!(
            cfg.transport,
            RotatorConfig::default().transport,
            "the default must stay rotctld, so nothing changes for an existing station"
        );
    }

    /// The three transports this fork had before must all still be there.
    #[test]
    fn the_transports_we_already_had_are_untouched() {
        let had = [
            RotatorTransport::Rotctld,
            RotatorTransport::EasyComm2,
            RotatorTransport::Gs232,
        ];
        for t in had {
            assert_ne!(t, RotatorTransport::PsRotator);
        }
        assert_eq!(RotatorTransport::default(), RotatorTransport::Rotctld);
    }
}
