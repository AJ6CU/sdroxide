//! MIDI control surfaces — the de-facto way to put a real VFO knob, a PTT
//! button and a row of faders in front of an SDR (Behringer CMD PL-1 /
//! X-Touch, Korg nanoKONTROL2, CTR2-MIDI, home-built encoder boxes).
//!
//! NATIVE ONLY — links `midir`, which binds the host MIDI stack (ALSA
//! sequencer, WinMM, CoreMIDI). It must never become a dependency of any
//! wasm-targeted crate. The rest of the app sees only the opaque
//! [`MidiHandle`]; no `midir` type escapes this crate.
//!
//! The worker owns the connection and re-enumerates once a second, because
//! MIDI has no unplug notification and ALSA renumbers ports across a replug —
//! a controller that comes back must reconnect without the operator noticing.

mod message;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender, bounded, unbounded};
use midir::MidiIO;
use sdroxide_types::MidiMsg;
use tracing::{debug, info, warn};

pub use message::MidiControl;

/// Client name this app registers under, as other MIDI software will see it.
const CLIENT: &str = "sdroxide";
/// How often the worker re-scans for the configured port.
const RESCAN: Duration = Duration::from_secs(1);
/// Worker tick. Inbound messages do not wait on this — they go straight from
/// the driver callback into the channel — so it only paces reconnection.
const TICK: Duration = Duration::from_millis(100);
/// Depth of the inbound queue. A stalled UI drops messages rather than
/// back-pressuring the MIDI driver's callback thread.
const QUEUE: usize = 256;
/// After receiving a value for a control, don't echo the same value back to it
/// for this long: a motor fader would move, re-send, and oscillate.
const ECHO_GUARD: Duration = Duration::from_millis(250);

/// One MIDI port as offered to the settings UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortInfo {
    /// Stable identifier used to reconnect. Survives a replug better than the
    /// port index, which ALSA reassigns.
    pub id: String,
    pub name: String,
}

/// Which ports to use. Mirrors the persisted `MidiSettings`, minus the
/// bindings — matching a message to an action is the client's job.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MidiConfig {
    pub enabled: bool,
    pub in_port_id: String,
    pub in_port_name: String,
    pub out_port_id: String,
    pub out_port_name: String,
}

/// Something the controller did, or something that happened to it.
#[derive(Debug, Clone, PartialEq)]
pub enum MidiEvent {
    Control(MidiControl),
    Connected(String),
    /// The port went away (unplugged, or the sending program quit). Anything
    /// the controller was holding down must be released.
    Disconnected,
}

/// A value to send back to the controller (LED, motor fader, ring display).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Feedback {
    pub msg: MidiMsg,
    pub value: u8,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MidiStatus {
    pub connected: bool,
    /// Name of the connected input port, when there is one.
    pub port: String,
    pub out_connected: bool,
    /// Last connection failure, cleared once a connection succeeds.
    pub error: Option<String>,
}

enum Ctl {
    Config(MidiConfig),
    Feedback(Vec<Feedback>),
    Stop,
}

/// The app's view of a MIDI surface: a worker thread plus two channels.
pub struct MidiHandle {
    ctl: Sender<Ctl>,
    events: Receiver<MidiEvent>,
    status: Arc<Mutex<MidiStatus>>,
    thread: Option<JoinHandle<()>>,
}

impl MidiHandle {
    pub fn set_config(&self, cfg: MidiConfig) {
        let _ = self.ctl.send(Ctl::Config(cfg));
    }

    /// Drain everything that has arrived since the last call. Non-blocking.
    pub fn poll(&mut self) -> Vec<MidiEvent> {
        let mut out = Vec::new();
        while let Ok(e) = self.events.try_recv() {
            out.push(e);
        }
        out
    }

    pub fn feedback(&self, msgs: &[Feedback]) {
        if !msgs.is_empty() {
            let _ = self.ctl.send(Ctl::Feedback(msgs.to_vec()));
        }
    }

    pub fn status(&self) -> MidiStatus {
        self.status.lock().map(|s| s.clone()).unwrap_or_default()
    }
}

impl Drop for MidiHandle {
    fn drop(&mut self) {
        let _ = self.ctl.send(Ctl::Stop);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

/// Available MIDI input ports, in the driver's own order.
pub fn input_ports() -> Vec<PortInfo> {
    let Ok(input) = midir::MidiInput::new(CLIENT) else { return Vec::new() };
    input
        .ports()
        .iter()
        .map(|p| PortInfo {
            id: p.id(),
            name: input.port_name(p).unwrap_or_else(|_| "(unknown)".into()),
        })
        .collect()
}

pub fn output_ports() -> Vec<PortInfo> {
    let Ok(output) = midir::MidiOutput::new(CLIENT) else { return Vec::new() };
    output
        .ports()
        .iter()
        .map(|p| PortInfo {
            id: p.id(),
            name: output.port_name(p).unwrap_or_else(|_| "(unknown)".into()),
        })
        .collect()
}

/// Start the MIDI worker. `wake` is called from the driver's callback thread
/// whenever a message arrives, so the UI can repaint immediately instead of
/// waiting out its idle poll — a knob that lags a quarter second feels broken.
pub fn spawn(cfg: MidiConfig, wake: impl Fn() + Send + Sync + 'static) -> MidiHandle {
    let (ctl_tx, ctl_rx) = unbounded();
    let (ev_tx, ev_rx) = bounded(QUEUE);
    let status = Arc::new(Mutex::new(MidiStatus::default()));
    let st = Arc::clone(&status);
    let wake: Arc<dyn Fn() + Send + Sync> = Arc::new(wake);
    let thread = std::thread::Builder::new()
        .name("sdroxide-midi".into())
        .spawn(move || worker(cfg, ctl_rx, ev_tx, st, wake))
        .ok();
    MidiHandle { ctl: ctl_tx, events: ev_rx, status, thread }
}

/// The one bit of `MidiInput`/`MidiOutput` this crate needs generically:
/// midir exposes `find_port_by_id` as an inherent method on each, not on its
/// `MidiIO` trait, so input and output would otherwise need duplicate lookups.
trait PortLookup: MidiIO {
    fn by_id(&self, id: &str) -> Option<Self::Port>;
}

impl PortLookup for midir::MidiInput {
    fn by_id(&self, id: &str) -> Option<Self::Port> {
        self.find_port_by_id(id)
    }
}

impl PortLookup for midir::MidiOutput {
    fn by_id(&self, id: &str) -> Option<Self::Port> {
        self.find_port_by_id(id)
    }
}

/// Find a port by its stable id, falling back to its name.
///
/// The id is preferred because two identical controllers share a name; the
/// name is the fallback because some backends mint a fresh id on every replug,
/// and losing the binding on a cable wiggle is worse than picking the wrong
/// one of two identical surfaces.
fn find_port<T: PortLookup>(io: &T, id: &str, name: &str) -> Option<T::Port> {
    if !id.is_empty()
        && let Some(p) = io.by_id(id)
    {
        return Some(p);
    }
    if name.is_empty() {
        return None;
    }
    io.ports().into_iter().find(|p| io.port_name(p).as_deref() == Ok(name))
}

/// Feedback bookkeeping per control, shared with the driver callback thread
/// (which records inbound values) and the worker (which sends outbound ones).
type EchoMap = Arc<Mutex<HashMap<(u8, u8, u8), Echo>>>;

#[derive(Clone, Copy)]
struct Echo {
    sent: Option<u8>,
    received: Option<(u8, Instant)>,
}

fn key(msg: &MidiMsg) -> (u8, u8, u8) {
    (msg.kind as u8, msg.channel, msg.data1)
}

struct Worker {
    cfg: MidiConfig,
    ev: Sender<MidiEvent>,
    status: Arc<Mutex<MidiStatus>>,
    wake: Arc<dyn Fn() + Send + Sync>,
    input: Option<midir::MidiInputConnection<()>>,
    output: Option<midir::MidiOutputConnection>,
    /// Last value sent to each control and when it last spoke. Keyed by the
    /// control, not the action, because two bindings can share one control.
    echo: EchoMap,
    next_scan: Instant,
}

fn worker(
    cfg: MidiConfig,
    ctl: Receiver<Ctl>,
    ev: Sender<MidiEvent>,
    status: Arc<Mutex<MidiStatus>>,
    wake: Arc<dyn Fn() + Send + Sync>,
) {
    let mut w = Worker {
        cfg,
        ev,
        status,
        wake,
        input: None,
        output: None,
        echo: Arc::new(Mutex::new(HashMap::new())),
        next_scan: Instant::now(),
    };
    loop {
        match ctl.recv_timeout(TICK) {
            Ok(Ctl::Config(c)) => {
                if c.in_port_id != w.cfg.in_port_id
                    || c.in_port_name != w.cfg.in_port_name
                    || c.enabled != w.cfg.enabled
                {
                    w.disconnect_input();
                }
                if c.out_port_id != w.cfg.out_port_id || c.out_port_name != w.cfg.out_port_name {
                    w.output = None;
                }
                w.cfg = c;
                w.next_scan = Instant::now();
            }
            Ok(Ctl::Feedback(msgs)) => w.send_feedback(&msgs),
            Ok(Ctl::Stop) => break,
            Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
            Err(crossbeam_channel::RecvTimeoutError::Disconnected) => break,
        }
        if Instant::now() >= w.next_scan {
            w.next_scan = Instant::now() + RESCAN;
            w.reconcile();
        }
    }
}

impl Worker {
    fn set_status(&self, f: impl FnOnce(&mut MidiStatus)) {
        if let Ok(mut s) = self.status.lock() {
            f(&mut s);
        }
    }

    fn disconnect_input(&mut self) {
        if self.input.take().is_some() {
            // Whatever the surface was holding down can no longer be released
            // by the surface itself, so tell the client to let go of it.
            let _ = self.ev.try_send(MidiEvent::Disconnected);
            (self.wake)();
            self.set_status(|s| {
                s.connected = false;
                s.port.clear();
            });
        }
    }

    /// Bring the live connections in line with the config, and notice a device
    /// that has vanished.
    fn reconcile(&mut self) {
        if !self.cfg.enabled {
            self.disconnect_input();
            self.output = None;
            return;
        }
        if self.input.is_some() {
            // midir gives no unplug callback, so poll the port list instead.
            // A failure to enumerate is not evidence the port is gone.
            let gone = match midir::MidiInput::new(CLIENT) {
                Ok(io) => find_port(&io, &self.cfg.in_port_id, &self.cfg.in_port_name).is_none(),
                Err(_) => false,
            };
            if gone {
                info!("MIDI input {} disappeared", self.cfg.in_port_name);
                self.disconnect_input();
            }
        }
        if self.input.is_none() {
            self.connect_input();
        }
        if self.output.is_none() {
            self.connect_output();
        }
    }

    fn connect_input(&mut self) {
        if self.cfg.in_port_id.is_empty() && self.cfg.in_port_name.is_empty() {
            return;
        }
        // `connect` consumes the `MidiInput`, so a fresh one is built per
        // attempt rather than kept around.
        let mut io = match midir::MidiInput::new(CLIENT) {
            Ok(io) => io,
            Err(e) => {
                self.set_status(|s| s.error = Some(format!("MIDI unavailable: {e}")));
                return;
            }
        };
        // Sysex, timing clock and active sensing carry no bindings and would
        // only burn queue slots.
        io.ignore(midir::Ignore::All);
        let Some(port) = find_port(&io, &self.cfg.in_port_id, &self.cfg.in_port_name) else {
            return;
        };
        let name = io.port_name(&port).unwrap_or_else(|_| self.cfg.in_port_name.clone());
        let ev = self.ev.clone();
        let wake = Arc::clone(&self.wake);
        let echo = Arc::clone(&self.echo);
        // Runs on the driver's own thread: parse, remember, hand off, wake.
        // Nothing here may block, and a full queue drops rather than stalling
        // the MIDI stack.
        let cb = move |_stamp: u64, bytes: &[u8], _: &mut ()| {
            let Some(c) = message::parse(bytes) else { return };
            if let Ok(mut e) = echo.lock() {
                e.entry(key(&c.msg)).or_insert(Echo { sent: None, received: None }).received =
                    Some((c.value, Instant::now()));
            }
            if ev.try_send(MidiEvent::Control(c)).is_ok() {
                wake();
            }
        };
        match io.connect(&port, "sdroxide-in", cb, ()) {
            Ok(conn) => {
                info!("MIDI input connected: {name}");
                self.input = Some(conn);
                let n = name.clone();
                self.set_status(move |s| {
                    s.connected = true;
                    s.port = n;
                    s.error = None;
                });
                let _ = self.ev.try_send(MidiEvent::Connected(name));
                (self.wake)();
            }
            Err(e) => {
                warn!("MIDI input {name}: {e}");
                self.set_status(|s| s.error = Some(format!("{e}")));
            }
        }
    }

    fn connect_output(&mut self) {
        if self.cfg.out_port_id.is_empty() && self.cfg.out_port_name.is_empty() {
            return;
        }
        let Ok(io) = midir::MidiOutput::new(CLIENT) else { return };
        let Some(port) = find_port(&io, &self.cfg.out_port_id, &self.cfg.out_port_name) else {
            return;
        };
        match io.connect(&port, "sdroxide-out") {
            Ok(conn) => {
                self.output = Some(conn);
                self.set_status(|s| s.out_connected = true);
            }
            Err(e) => {
                debug!("MIDI output: {e}");
                self.set_status(|s| s.out_connected = false);
            }
        }
    }

    /// Echo values back to the controller, suppressing anything that would
    /// start a loop.
    ///
    /// Two guards: never re-send a value the control already shows, and never
    /// bounce a value straight back at the control that just sent it. Without
    /// the second, a motor fader moves to the position it just reported,
    /// reports the new position, and the pair oscillates.
    fn send_feedback(&mut self, msgs: &[Feedback]) {
        let Some(out) = self.output.as_mut() else { return };
        let Ok(mut echo) = self.echo.lock() else { return };
        let now = Instant::now();
        for f in msgs {
            let e = echo.entry(key(&f.msg)).or_insert(Echo { sent: None, received: None });
            if e.sent == Some(f.value) {
                continue;
            }
            if let Some((v, at)) = e.received
                && v == f.value
                && now.duration_since(at) < ECHO_GUARD
            {
                continue;
            }
            if out.send(&message::encode(&f.msg, f.value)).is_ok() {
                e.sent = Some(f.value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdroxide_types::MidiMsgKind;

    /// Enumeration must not panic or hang where there is no MIDI stack at all
    /// (headless CI containers have no ALSA sequencer).
    #[test]
    fn enumeration_is_safe_without_a_midi_stack() {
        let _ = input_ports();
        let _ = output_ports();
    }

    #[test]
    fn a_disabled_worker_starts_and_stops_cleanly() {
        let h = spawn(MidiConfig::default(), || {});
        assert!(!h.status().connected);
        drop(h);
    }

    /// CC 16 and note 16 are different physical controls; sharing an echo slot
    /// would let one suppress the other's feedback.
    #[test]
    fn echo_key_separates_controls_that_share_a_number() {
        let cc = MidiMsg { kind: MidiMsgKind::Cc, channel: 0, data1: 16 };
        let note = MidiMsg { kind: MidiMsgKind::Note, channel: 0, data1: 16 };
        assert_ne!(key(&cc), key(&note));
    }
}
