//! A KISS TNC on a socket: sdroxide's modem, offered to other people's software.
//!
//! sdroxide is the **TNC** here, not the host. Pat, an APRS client, `axcall`,
//! or the Linux kernel's AX.25 stack via `kissattach` connect to this port and
//! use the radio through it. That is the same posture as the TCI and rigctld
//! servers — "sdroxide is the server" — and it is what makes the packet work
//! useful to programs that will never know sdroxide exists.
//!
//! It also earns its keep before any of that: pointing the Linux AX.25 stack at
//! this port tests our framing, modem and connected-mode machine against an
//! implementation nobody here wrote.
//!
//! # Shape
//!
//! Modelled on `sdroxide-rigctld`: bind up front so a port clash surfaces to
//! the settings dialog rather than disappearing into a thread, an acceptor
//! thread, one reader per client, and an engine-facing handle that is polled
//! from the existing tick. Nothing here blocks the engine.
//!
//! # What it does not do
//!
//! No digipeating, no beaconing, no `KISS` hardware commands beyond noting
//! them. Those either belong to the radio (which the engine owns) or to the
//! host (which is the point of KISS).

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender, unbounded};
use sdroxide_ax25::kiss::{self, Command, Decoder};
use tracing::{debug, info, warn};

/// How often the acceptor wakes to take a connection or notice a stop.
const ACCEPT_POLL: Duration = Duration::from_millis(50);
/// Client read timeout, so an idle connection still notices a shutdown.
const READ_TIMEOUT: Duration = Duration::from_millis(250);

/// What a connected host asked for.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KissRequest {
    /// Transmit this AX.25 frame. Address through information, no FCS — the
    /// modem adds that, because on a real TNC the hardware does.
    Send(Vec<u8>),
    /// The host adjusted a TNC parameter. Reported rather than applied: TXDELAY
    /// and persistence are the operator's settings here, and a client silently
    /// overriding what the operator configured would be a surprise with no
    /// indication anywhere.
    Parameter(Command, u8),
    /// A client connected or disconnected.
    Clients(usize),
}

/// The engine's handle on a running server.
pub struct KissServer {
    stop: Arc<AtomicBool>,
    clients: Arc<AtomicUsize>,
    /// Every connected client's write half, for broadcasting received frames.
    sinks: Arc<Mutex<Vec<TcpStream>>>,
    req_rx: Receiver<KissRequest>,
    addr: String,
    acceptor: Option<JoinHandle<()>>,
}

impl KissServer {
    /// Bind and start serving.
    ///
    /// The listener is bound before the thread is spawned, so a port already in
    /// use — most often a real TNC daemon — is an `Err` here rather than a
    /// silent failure.
    pub fn start(addr: &str) -> Result<Self, String> {
        let listener = TcpListener::bind(addr).map_err(|e| format!("bind {addr}: {e}"))?;
        listener.set_nonblocking(true).map_err(|e| format!("{addr}: {e}"))?;
        // Report where we landed, not what was asked for: they differ whenever
        // the port is 0, and a status line should never lie.
        let bound = listener.local_addr().map(|a| a.to_string()).unwrap_or_else(|_| addr.into());

        let stop = Arc::new(AtomicBool::new(false));
        let clients = Arc::new(AtomicUsize::new(0));
        let sinks: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new()));
        let (req_tx, req_rx) = unbounded();

        let acceptor = {
            let (stop, clients, sinks, req_tx) =
                (Arc::clone(&stop), Arc::clone(&clients), Arc::clone(&sinks), req_tx.clone());
            std::thread::Builder::new()
                .name("sdroxide-kiss".into())
                .spawn(move || accept_loop(listener, stop, clients, sinks, req_tx))
                .map_err(|e| format!("starting the KISS server: {e}"))?
        };

        info!(addr = %bound, "KISS server listening");
        Ok(KissServer { stop, clients, sinks, req_rx, addr: bound, acceptor: Some(acceptor) })
    }

    /// Where it actually bound.
    #[must_use]
    pub fn addr(&self) -> &str {
        &self.addr
    }

    #[must_use]
    pub fn clients(&self) -> usize {
        self.clients.load(Ordering::Relaxed)
    }

    /// Take whatever the hosts have asked for. Never blocks.
    pub fn poll(&self) -> Vec<KissRequest> {
        self.req_rx.try_iter().collect()
    }

    /// Hand a frame heard on the air to every connected host.
    ///
    /// The FCS is already gone — the deframer checked and stripped it, which is
    /// what a TNC's firmware would have done.
    pub fn broadcast(&self, frame: &[u8]) {
        let wire = kiss::escape(frame);
        let mut sinks = self.sinks.lock().unwrap_or_else(|e| e.into_inner());
        sinks.retain_mut(|s| match s.write_all(&wire) {
            Ok(()) => true,
            Err(e) => {
                debug!("dropping a KISS client: {e}");
                false
            }
        });
    }
}

impl Drop for KissServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(t) = self.acceptor.take() {
            let _ = t.join();
        }
    }
}

fn accept_loop(
    listener: TcpListener,
    stop: Arc<AtomicBool>,
    clients: Arc<AtomicUsize>,
    sinks: Arc<Mutex<Vec<TcpStream>>>,
    req_tx: Sender<KissRequest>,
) {
    while !stop.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((sock, peer)) => {
                info!(%peer, "KISS client connected");
                let _ = sock.set_nodelay(true);
                let _ = sock.set_read_timeout(Some(READ_TIMEOUT));
                let Ok(reader) = sock.try_clone() else { continue };
                if let Ok(mut s) = sinks.lock() {
                    s.push(sock);
                }
                let n = clients.fetch_add(1, Ordering::SeqCst) + 1;
                let _ = req_tx.send(KissRequest::Clients(n));

                let (stop, clients, req_tx) =
                    (Arc::clone(&stop), Arc::clone(&clients), req_tx.clone());
                let spawned = std::thread::Builder::new()
                    .name("sdroxide-kiss-client".into())
                    .spawn(move || {
                        client_loop(reader, &stop, &req_tx);
                        let n = clients.fetch_sub(1, Ordering::SeqCst).saturating_sub(1);
                        let _ = req_tx.send(KissRequest::Clients(n));
                        info!("KISS client gone");
                    });
                if let Err(e) = spawned {
                    warn!("starting a KISS client thread: {e}");
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(ACCEPT_POLL);
            }
            Err(e) => {
                warn!("KISS accept: {e}");
                std::thread::sleep(ACCEPT_POLL);
            }
        }
    }
}

fn client_loop(mut sock: TcpStream, stop: &AtomicBool, req_tx: &Sender<KissRequest>) {
    let mut dec = Decoder::new();
    let mut buf = [0u8; 4096];
    let mut frames = Vec::new();
    while !stop.load(Ordering::SeqCst) {
        match sock.read(&mut buf) {
            Ok(0) => return,
            Ok(n) => {
                frames.clear();
                // A malformed frame is dropped and reported; the stream
                // resynchronises on the next delimiter. Never fatal — this is a
                // socket, and taking the radio down with the operator's
                // receiver on it is not a proportionate answer to a bad byte.
                if let Err(e) = dec.push(&buf[..n], &mut frames) {
                    debug!("KISS decode: {e}");
                }
                for (cmd, payload) in frames.drain(..) {
                    let req = match cmd {
                        Command::Data if !payload.is_empty() => KissRequest::Send(payload),
                        Command::Data => continue,
                        other => {
                            KissRequest::Parameter(other, payload.first().copied().unwrap_or(0))
                        }
                    };
                    if req_tx.send(req).is_err() {
                        return;
                    }
                }
            }
            Err(ref e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) => {}
            Err(e) => {
                debug!("KISS client read: {e}");
                return;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;

    fn connect(s: &KissServer) -> TcpStream {
        let c = TcpStream::connect(s.addr()).expect("connect");
        c.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        // Wait for the server to notice, so the test is not racing the accept.
        for _ in 0..200 {
            if s.clients() > 0 {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        c
    }

    fn wait_for<T>(mut f: impl FnMut() -> Option<T>) -> T {
        for _ in 0..300 {
            if let Some(v) = f() {
                return v;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        panic!("timed out waiting for the server");
    }

    /// A frame written by a host must reach the engine intact.
    #[test]
    fn a_host_frame_reaches_the_engine() {
        let s = KissServer::start("127.0.0.1:0").expect("bind");
        let mut c = connect(&s);

        // Deliberately contains both bytes KISS has to escape.
        let frame = vec![0x82, 0xA0, kiss::FEND, 0x03, kiss::FESC, 0xF0, b'h', b'i'];
        c.write_all(&kiss::escape(&frame)).unwrap();

        let got = wait_for(|| {
            s.poll().into_iter().find_map(|r| match r {
                KissRequest::Send(f) => Some(f),
                _ => None,
            })
        });
        assert_eq!(got, frame, "the frame did not survive the socket");
    }

    /// A frame heard on the air must reach the host.
    #[test]
    fn an_air_frame_reaches_the_host() {
        let s = KissServer::start("127.0.0.1:0").expect("bind");
        let c = connect(&s);

        let frame = vec![0x9E, kiss::FESC, 0x03, 0xF0, b'o', b'k'];
        s.broadcast(&frame);

        let mut dec = Decoder::new();
        let mut out = Vec::new();
        let mut reader = BufReader::new(c);
        let mut buf = [0u8; 256];
        for _ in 0..50 {
            let n = reader.get_mut().read(&mut buf).unwrap_or(0);
            if n > 0 {
                dec.push(&buf[..n], &mut out).unwrap();
            }
            if !out.is_empty() {
                break;
            }
        }
        assert_eq!(out, vec![(Command::Data, frame)], "the frame did not reach the host");
    }

    /// A host adjusting TXDELAY is reported, not silently applied. Those are
    /// the operator's settings, and a client overriding them with no indication
    /// anywhere would be a mystery to debug.
    #[test]
    fn a_parameter_command_is_reported() {
        let s = KissServer::start("127.0.0.1:0").expect("bind");
        let mut c = connect(&s);
        // Command 1 = TXDELAY, value 40 (units of 10 ms).
        c.write_all(&[kiss::FEND, 0x01, 40, kiss::FEND]).unwrap();

        let got = wait_for(|| {
            s.poll().into_iter().find_map(|r| match r {
                KissRequest::Parameter(cmd, v) => Some((cmd, v)),
                _ => None,
            })
        });
        assert_eq!(got, (Command::TxDelay, 40));
    }

    /// Garbage must not take the server down, and the stream must recover.
    #[test]
    fn a_malformed_frame_does_not_stop_the_next_one() {
        let s = KissServer::start("127.0.0.1:0").expect("bind");
        let mut c = connect(&s);

        let good = vec![0x82, 0xA0, 0x03, 0xF0, b'o', b'k'];
        c.write_all(&[kiss::FEND, 0x00, kiss::FESC, 0x99, kiss::FEND]).unwrap();
        c.write_all(&kiss::escape(&good)).unwrap();

        let got = wait_for(|| {
            s.poll().into_iter().find_map(|r| match r {
                KissRequest::Send(f) => Some(f),
                _ => None,
            })
        });
        assert_eq!(got, good, "a bad frame poisoned the stream");
    }

    /// The client count has to be right, or the UI lies about whether anything
    /// is attached to the radio.
    #[test]
    fn clients_are_counted() {
        let s = KissServer::start("127.0.0.1:0").expect("bind");
        assert_eq!(s.clients(), 0);
        let c = connect(&s);
        assert_eq!(s.clients(), 1);
        drop(c);
        let n = wait_for(|| (s.clients() == 0).then_some(0));
        assert_eq!(n, 0, "a departed client was still counted");
    }

    /// A port already in use is an error the settings dialog can show, not a
    /// thread that quietly does nothing.
    #[test]
    fn a_port_clash_is_an_error() {
        let first = KissServer::start("127.0.0.1:0").expect("bind");
        let addr = first.addr().to_string();
        assert!(KissServer::start(&addr).is_err(), "two servers bound the same port");
    }
}
