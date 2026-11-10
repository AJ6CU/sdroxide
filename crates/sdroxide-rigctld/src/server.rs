//! Sockets: the acceptor thread, one reader thread per client, and the
//! engine-facing handle.
//!
//! Nothing here inspects a command — [`crate::proto`] owns every protocol
//! decision, so this file stays small enough to be obviously correct.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, RwLock};
use std::thread::JoinHandle;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender, unbounded};
use sdroxide_types::RigctldConfig;
use tracing::{debug, info, warn};

use crate::ServerRequest;
use crate::proto::{self, Reply, Session};
use crate::state::RigState;

/// How often the acceptor wakes to take a new connection or notice a stop.
const ACCEPT_POLL: Duration = Duration::from_millis(50);
/// Client socket read timeout, so an idle connection still notices a shutdown.
const READ_TIMEOUT: Duration = Duration::from_millis(250);
/// Longest command line we will buffer. Real commands are a few dozen bytes;
/// anything past this is a peer that is not speaking rigctld.
const MAX_LINE: u64 = 4096;

/// State shared between the engine, the acceptor and every client thread.
struct Shared {
    state: RwLock<RigState>,
    cfg: RwLock<RigctldConfig>,
    clients: AtomicUsize,
    stop: AtomicBool,
}

/// The engine's handle on the running server.
pub struct RigctldController {
    shared: Arc<Shared>,
    req_rx: Receiver<ServerRequest>,
    addr: String,
    acceptor: Option<JoinHandle<()>>,
}

impl RigctldController {
    /// Bind and start serving.
    ///
    /// The listener is bound **before** the acceptor thread is spawned, so a
    /// bind failure — most often a real `rigctld` already holding 4532 —
    /// surfaces here as an `Err` the settings dialog can show, instead of
    /// disappearing into a thread.
    pub fn start(cfg: &RigctldConfig, state: RigState) -> Result<Self, String> {
        let want = cfg.addr();
        let listener = TcpListener::bind(&want).map_err(|e| format!("bind {want}: {e}"))?;
        listener.set_nonblocking(true).map_err(|e| format!("{want}: {e}"))?;
        // Report where we actually landed, not what was asked for — they differ
        // whenever the port is 0, and the status line should never lie.
        let addr = listener.local_addr().map(|a| a.to_string()).unwrap_or(want);

        let (req_tx, req_rx) = unbounded();
        let shared = Arc::new(Shared {
            state: RwLock::new(state),
            cfg: RwLock::new(cfg.clone()),
            clients: AtomicUsize::new(0),
            stop: AtomicBool::new(false),
        });
        let acceptor = {
            let shared = Arc::clone(&shared);
            std::thread::Builder::new()
                .name("sdroxide-rigctld".into())
                .spawn(move || accept_loop(listener, shared, req_tx))
                .map_err(|e| format!("spawn rigctld thread: {e}"))?
        };
        info!("rigctld server listening on {addr}");
        Ok(RigctldController { shared, req_rx, addr, acceptor: Some(acceptor) })
    }

    /// The `host:port` we are bound to.
    pub fn addr(&self) -> &str {
        &self.addr
    }

    pub fn clients(&self) -> usize {
        self.shared.clients.load(Ordering::Relaxed)
    }

    /// Apply a changed configuration that doesn't move the socket (transmit
    /// permission, client limit, reported name). Rebinding is the caller's job.
    pub fn set_config(&self, cfg: RigctldConfig) {
        if let Ok(mut c) = self.shared.cfg.write() {
            *c = cfg;
        }
    }

    /// Publish the current radio state. Clients read it on their next command;
    /// rigctld never pushes, so there is nothing to broadcast.
    pub fn publish_state(&self, snap: RigState) {
        if let Ok(mut s) = self.shared.state.write() {
            *s = snap;
        }
    }

    /// Drain everything clients have asked for since the last call. Non-blocking.
    pub fn poll(&self) -> Vec<ServerRequest> {
        self.req_rx.try_iter().collect()
    }
}

impl Drop for RigctldController {
    fn drop(&mut self) {
        self.shared.stop.store(true, Ordering::Relaxed);
        if let Some(h) = self.acceptor.take() {
            let _ = h.join();
        }
    }
}

fn accept_loop(listener: TcpListener, shared: Arc<Shared>, req_tx: Sender<ServerRequest>) {
    let mut clients: Vec<JoinHandle<()>> = Vec::new();
    while !shared.stop.load(Ordering::Relaxed) {
        match listener.accept() {
            Ok((stream, peer)) => {
                // Reap finished threads before checking the limit, so a client
                // that just left frees its slot immediately.
                clients.retain(|h| !h.is_finished());
                let max = shared.cfg.read().map(|c| c.max_clients).unwrap_or(4).max(1);
                if clients.len() >= max {
                    debug!("rigctld: refusing {peer}, {max} clients already connected");
                    continue;
                }
                let shared = Arc::clone(&shared);
                let req_tx = req_tx.clone();
                if let Ok(h) = std::thread::Builder::new()
                    .name("sdroxide-rigctld-cli".into())
                    .spawn(move || {
                        shared.clients.fetch_add(1, Ordering::Relaxed);
                        let _ = req_tx
                            .send(ServerRequest::Clients(shared.clients.load(Ordering::Relaxed)));
                        client(stream, &shared, &req_tx);
                        let left = shared.clients.fetch_sub(1, Ordering::Relaxed) - 1;
                        let _ = req_tx.send(ServerRequest::Clients(left));
                    })
                {
                    clients.push(h);
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(ACCEPT_POLL);
            }
            Err(e) => {
                warn!("rigctld accept: {e}");
                std::thread::sleep(ACCEPT_POLL);
            }
        }
    }
    // Dropping the listener stops new connections; existing clients notice the
    // stop flag on their next read timeout.
    drop(listener);
    for h in clients {
        let _ = h.join();
    }
}

fn client(stream: TcpStream, shared: &Shared, req_tx: &Sender<ServerRequest>) {
    if stream.set_nodelay(true).is_err() {
        debug!("rigctld client: could not disable Nagle");
    }
    if stream.set_read_timeout(Some(READ_TIMEOUT)).is_err() {
        warn!("rigctld client: could not set read timeout");
        return;
    }
    let Ok(mut writer) = stream.try_clone() else { return };
    let mut reader = BufReader::new(stream);
    let mut sess = Session::default();
    let mut line = String::new();

    while !shared.stop.load(Ordering::Relaxed) {
        line.clear();
        // A bounded take: a peer that never sends a newline can't grow this
        // buffer without limit.
        match (&mut reader).take(MAX_LINE).read_line(&mut line) {
            Ok(0) => return, // peer closed
            Ok(_) => {}
            Err(ref e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) =>
            {
                continue;
            }
            Err(e) => {
                debug!("rigctld client read: {e}");
                return;
            }
        }

        let mut cmds = Vec::new();
        let reply = {
            // Snapshot the state and config, then drop both locks before
            // touching the socket: a slow write must never stall the engine's
            // next state publish.
            let st = match shared.state.read() {
                Ok(s) => s.clone(),
                Err(_) => return,
            };
            let cfg = match shared.cfg.read() {
                Ok(c) => c.clone(),
                Err(_) => return,
            };
            proto::handle(&line, &st, &cfg, &mut sess, &mut cmds)
        };

        for c in cmds {
            let _ = req_tx.send(ServerRequest::Cmd(c));
        }
        match reply {
            Reply::Close => return,
            Reply::Send(text) if text.is_empty() => {}
            Reply::Send(text) => {
                // Hamlib reads a line at a time, so a partial write would
                // desynchronise the whole session.
                if writer.write_all(text.as_bytes()).is_err() || writer.flush().is_err() {
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn loopback_cfg() -> RigctldConfig {
        // Port 0 asks the OS for a free one, so tests never collide with a
        // real rigctld or with each other.
        RigctldConfig { enabled: true, port: 0, ..RigctldConfig::default() }
    }

    #[test]
    fn binds_and_reports_its_address() {
        let c = RigctldController::start(&loopback_cfg(), RigState::default()).unwrap();
        assert!(c.addr().starts_with("127.0.0.1:"));
        assert_eq!(c.clients(), 0);
    }

    /// A second bind to the same port must fail loudly, so the settings dialog
    /// can say "a real rigctld may already own 4532" instead of silently doing
    /// nothing.
    #[test]
    fn a_taken_port_reports_an_error() {
        let first = RigctldController::start(&loopback_cfg(), RigState::default()).unwrap();
        let port: u16 = first.addr().rsplit(':').next().unwrap().parse().unwrap();
        let cfg = RigctldConfig { enabled: true, port, ..RigctldConfig::default() };
        let err = match RigctldController::start(&cfg, RigState::default()) {
            Err(e) => e,
            Ok(_) => panic!("a second bind to the same port must fail"),
        };
        assert!(err.contains("bind"), "{err}");
    }

    #[test]
    fn answers_a_real_client() {
        let state = RigState { vfo_a_hz: 14_074_000.0, can_tx: true, ..RigState::default() };
        let ctrl = RigctldController::start(&loopback_cfg(), state).unwrap();
        let mut sock = TcpStream::connect(ctrl.addr()).unwrap();
        sock.set_read_timeout(Some(Duration::from_secs(5))).unwrap();

        sock.write_all(b"f\n").unwrap();
        let mut buf = [0u8; 64];
        let n = sock.read(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"14074000\n");

        // A set must reach the engine as a Command.
        sock.write_all(b"F 7100000\n").unwrap();
        let n = sock.read(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"RPRT 0\n");
        let reqs = wait_for_cmd(&ctrl);
        assert!(
            reqs.iter()
                .any(|r| matches!(r, ServerRequest::Cmd(sdroxide_types::Command::SetVfo { .. })))
        );
    }

    /// The exchange every Hamlib client opens with. If `\dump_state` is wrong
    /// the connection dies here rather than at the first real command.
    #[test]
    fn serves_the_hamlib_handshake() {
        let ctrl = RigctldController::start(&loopback_cfg(), RigState::default()).unwrap();
        let mut sock = TcpStream::connect(ctrl.addr()).unwrap();
        sock.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        sock.write_all(b"\\chk_vfo\n").unwrap();
        let mut buf = [0u8; 16];
        let n = sock.read(&mut buf).unwrap();
        assert_eq!(&buf[..n], b"0\n");

        sock.write_all(b"\\dump_state\n").unwrap();
        let mut got = String::new();
        let mut buf = [0u8; 4096];
        while !got.contains("done\n") {
            let n = sock.read(&mut buf).unwrap();
            assert!(n > 0, "socket closed mid-dump_state");
            got.push_str(&String::from_utf8_lossy(&buf[..n]));
        }
        assert!(got.starts_with("1\n"));
    }

    #[test]
    fn quit_closes_the_connection() {
        let ctrl = RigctldController::start(&loopback_cfg(), RigState::default()).unwrap();
        let mut sock = TcpStream::connect(ctrl.addr()).unwrap();
        sock.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        sock.write_all(b"q\n").unwrap();
        let mut buf = [0u8; 16];
        assert_eq!(sock.read(&mut buf).unwrap(), 0, "server should close after q");
    }

    /// Poll until the engine-facing queue produces a command, so the test does
    /// not race the client thread.
    fn wait_for_cmd(ctrl: &RigctldController) -> Vec<ServerRequest> {
        let mut all = Vec::new();
        for _ in 0..200 {
            all.extend(ctrl.poll());
            if all.iter().any(|r| matches!(r, ServerRequest::Cmd(_))) {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        all
    }
}
