//! A CW paddle read straight from the kernel's input layer (Linux, native).
//!
//! Some USB keyer boxes have no keyer in them: they report the two paddle
//! contacts and nothing else, often as the buttons of an otherwise useless
//! "mouse". This opens one interface's input devices, takes them exclusively
//! (`EVIOCGRAB`, so the contacts cannot also land as mouse clicks in whatever
//! window has focus), and publishes **the two contacts** for whoever is
//! transmitting.
//!
//! **It publishes contacts, not edges, and runs no keyer of its own** (issue
//! #569). A keyer on this side of the wire would make every element from
//! whatever this thread happened to sample — at 20 wpm a dit is 60 ms, inside
//! one 50 ms transmit block — and could not reach a rig that keys itself at
//! all, because on that route the sidetone is never transmitted. The iambic
//! timing belongs next to the transmitter, so the contacts go up the wire and
//! [`sdroxide_dsp::CwKeyer`] runs on the engine side. What is left here is the
//! device, and a monitor tone so the operator can hear their own paddle.
//!
//! A composite HID keyer often registers two nodes for the one physical
//! interface — a keyboard node and a mouse node — and which of them carries the
//! contacts depends on the firmware. Both are opened and grabbed, and the
//! dropdown lists one entry per interface rather than one per node.
//!
//! Native and Linux only: it is raw evdev, and the browser and the other
//! platforms have no equivalent. The keyer itself is portable and lives in the
//! DSP crate; this is only the part that touches a device.

use crate::i18n::tr;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::os::fd::AsRawFd;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use sdroxide_audio::start_output;

/// `_IOW('E', 0x90, int)` — take the device exclusively.
const EVIOCGRAB: libc::c_ulong = 0x4004_4590;
const EV_KEY: u16 = 1;
const BTN_LEFT: u16 = 0x110;
const BTN_RIGHT: u16 = 0x111;
const BTN_MIDDLE: u16 = 0x112;

/// One entry per keyer interface, preferring its mouse node (where the contacts
/// usually are) and falling back to its keyboard node.
pub fn devices() -> Vec<PathBuf> {
    let mut bases: BTreeMap<String, (Option<PathBuf>, Option<PathBuf>)> = BTreeMap::new();
    if let Ok(rd) = std::fs::read_dir("/dev/input/by-id") {
        for e in rd.flatten() {
            let name = e.file_name().to_string_lossy().into_owned();
            let (base, keyboard) = if let Some(b) = name.strip_suffix("-event-mouse") {
                (b.to_string(), false)
            } else if let Some(b) = name.strip_suffix("-event-kbd") {
                (b.to_string(), true)
            } else {
                continue;
            };
            let slot = bases.entry(base).or_default();
            if keyboard {
                slot.1 = Some(e.path());
            } else {
                slot.0 = Some(e.path());
            }
        }
    }
    bases.into_values().filter_map(|(mouse, kbd)| mouse.or(kbd)).collect()
}

/// The device to open by default: the one whose name says "key", so a real
/// mouse is never grabbed by accident. `None` leaves it to the operator.
pub fn default_device() -> Option<PathBuf> {
    let all = devices();
    all.iter()
        .find(|p| name_says_key(p) && is_mouse(p))
        .or_else(|| all.iter().find(|p| name_says_key(p)))
        .cloned()
}

fn name_says_key(p: &Path) -> bool {
    p.file_name().map(|n| n.to_string_lossy().to_lowercase().contains("key")).unwrap_or(false)
}

fn is_mouse(p: &Path) -> bool {
    p.file_name().map(|n| n.to_string_lossy().ends_with("-event-mouse")).unwrap_or(false)
}

/// The two nodes one interface may register. Both are opened, because which one
/// carries the paddle contacts is the firmware's choice.
fn siblings(path: &Path) -> Vec<PathBuf> {
    let name = path.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    let Some(base) = name.strip_suffix("-event-mouse").or_else(|| name.strip_suffix("-event-kbd"))
    else {
        return vec![path.to_path_buf()];
    };
    let dir = path.parent().map(Path::to_path_buf).unwrap_or_default();
    let mut out = Vec::new();
    for suffix in ["-event-mouse", "-event-kbd"] {
        let p = dir.join(format!("{base}{suffix}"));
        if p.exists() {
            out.push(p);
        }
    }
    if out.is_empty() {
        out.push(path.to_path_buf());
    }
    out
}

/// How the two contacts map onto dit and dah, and what the monitor tone is.
///
/// **No keyer mode here, deliberately** (#569). Straight and iambic differ in
/// who decides the envelope, and this half no longer decides anything: it reads
/// the contacts, maps them onto dit and dah, and publishes them. The straight
/// jack is still reported, because a straight key plugged into a paddle box is
/// the middle contact and it is the one contact there is.
#[derive(Clone, Copy)]
pub struct KeySetup {
    pub pitch_hz: f32,
    pub reverse: bool,
    /// Play the sidetone here. Off when the caller already hears its own tone
    /// (the CW panel's sidetone while the transmitter is keyed), so the two do
    /// not double.
    pub monitor: bool,
}

impl Default for KeySetup {
    fn default() -> Self {
        KeySetup { pitch_hz: 700.0, reverse: false, monitor: true }
    }
}

struct Shared {
    /// The mapped contacts. `AtomicU8` bit 0 is dit, bit 1 is dah, bit 2 the
    /// straight (middle) contact, so a caller reads one word and does not have
    /// to reconcile three loads taken a moment apart.
    contacts: AtomicU8,
    /// The monitor tone as it was played, for a caller that wants to decode
    /// what the operator sent. Bounded: it is a read-back tap, not a recorder,
    /// and a caller that stops draining it loses old audio rather than memory.
    tone: Mutex<Vec<f32>>,
    error: Mutex<Option<String>>,
}

const CONTACT_DIT: u8 = 1;
const CONTACT_DAH: u8 = 2;
const CONTACT_MIDDLE: u8 = 4;

/// Ceiling on the undrained monitor tap, in samples — about eight seconds at
/// 48 kHz. Past it the oldest audio is dropped rather than the buffer grown.
const TONE_TAP_MAX: usize = 48_000 * 8;

/// A running paddle source: a reader thread, the contacts it saw, and a tone.
pub struct CwKeySource {
    shared: Arc<Shared>,
    stop: Arc<AtomicBool>,
    thread: Option<JoinHandle<()>>,
}

impl CwKeySource {
    pub fn start(path: &Path, setup: KeySetup) -> Result<Self, String> {
        let shared = Arc::new(Shared {
            contacts: AtomicU8::new(0),
            tone: Mutex::new(Vec::new()),
            error: Mutex::new(None),
        });
        let stop = Arc::new(AtomicBool::new(false));
        let (shared2, stop2) = (Arc::clone(&shared), Arc::clone(&stop));
        let paths = siblings(path);
        let thread = std::thread::Builder::new()
            .name("cw-key".into())
            .spawn(move || run(paths, setup, &shared2, &stop2))
            .map_err(|e| tr!("cwkey-thread-failed", error = e.to_string()))?;
        Ok(CwKeySource { shared, stop, thread: Some(thread) })
    }

    /// The two **paddle contacts** as of this instant, already mapped through
    /// the reverse switch. This is the whole output of the thread: the caller
    /// sends them as `Command::CwContacts` and the engine makes the elements.
    pub fn contacts(&self) -> (bool, bool) {
        let c = self.shared.contacts.load(Ordering::Relaxed);
        (c & CONTACT_DIT != 0, c & CONTACT_DAH != 0)
    }

    /// The contact a **straight key** uses in this box: the middle jack if the
    /// box has one, else the dit contact.
    ///
    /// Both, deliberately, and it is not a guess: a paddle box with a straight
    /// key in it reports the key on whichever single contact it wired it to —
    /// the middle one where there is a middle one, and the dit contact where
    /// there is not. Reading only the middle would leave every straight key in a
    /// plain two-contact box dead, silently, which is the failure this whole
    /// change is about not having. Whether that contact is a straight key or a
    /// paddle is the caller's decision, which is why this is reported and not
    /// interpreted.
    pub fn straight_contact(&self) -> bool {
        let c = self.shared.contacts.load(Ordering::Relaxed);
        c & (CONTACT_MIDDLE | CONTACT_DIT) != 0
    }

    /// The monitor tone since the last call, at 48 kHz, for a caller that wants
    /// to decode what the operator sent. Empty when the monitor is off — which
    /// is the CW panel's own arrangement, since it hears the engine's sidetone
    /// instead and reads that back through `CwSelfRx`.
    pub fn take_tone(&self) -> Vec<f32> {
        std::mem::take(&mut *self.shared.tone.lock().unwrap())
    }

    /// A failure the thread hit after starting (device open, audio), said in the
    /// pane rather than swallowed.
    pub fn error(&self) -> Option<String> {
        self.shared.error.lock().unwrap().clone()
    }
}

impl Drop for CwKeySource {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

fn set_error(shared: &Shared, msg: String) {
    *shared.error.lock().unwrap() = Some(msg);
}

fn run(paths: Vec<PathBuf>, setup: KeySetup, shared: &Shared, stop: &AtomicBool) {
    let mut files = Vec::new();
    for path in &paths {
        match File::open(path) {
            Ok(f) => files.push((path.clone(), f)),
            Err(e) => {
                set_error(shared, format!("cannot open {}: {e}", path.display()));
                return;
            }
        }
    }
    let fds: Vec<libc::c_int> = files.iter().map(|(_, f)| f.as_raw_fd()).collect();
    let mut any_ungrabbed = false;
    for fd in &fds {
        if unsafe { libc::ioctl(*fd, EVIOCGRAB, 1) } != 0 {
            any_ungrabbed = true;
        }
    }
    if any_ungrabbed {
        // Not fatal: the contacts are still readable, they just also arrive as
        // clicks. Say so instead of pretending the grab worked.
        set_error(
            shared,
            tr!("cwkey-grab-failed"),
        );
    }
    for fd in &fds {
        unsafe { libc::fcntl(*fd, libc::F_SETFL, libc::O_NONBLOCK) };
    }

    // The sidetone is optional: a caller that keys a transmitter already hears
    // its own tone.
    let audio = if setup.monitor {
        match start_output(None, 48_000, sdroxide_types::AudioRouting::Both) {
            Ok(v) => Some(v),
            Err(e) => {
                set_error(shared, format!("no audio output for the sidetone: {e}"));
                for fd in &fds {
                    unsafe { libc::ioctl(*fd, EVIOCGRAB, 0) };
                }
                return;
            }
        }
    } else {
        None
    };
    let (out, mut ring) = match audio {
        Some((out, ring)) => (Some(out), Some(ring)),
        None => (None, None),
    };
    let rate = out.as_ref().map(|o| o.sample_rate).unwrap_or(48_000.0);
    let capacity = rate as usize * 2;
    let inc = std::f64::consts::TAU * setup.pitch_hz as f64 / rate;

    let (mut dit, mut dah, mut middle) = (false, false, false);
    let mut phase = 0.0f64;
    let start = Instant::now();
    let mut generated: u64 = 0;
    let mut buf = [0u8; 24 * 256];
    let ev_size = std::mem::size_of::<libc::input_event>();
    // Staged outside the lock, drained each pass: the tap must never be held
    // across the sleep below, or the caller's read-back stalls with it.
    let mut tap: Vec<f32> = Vec::new();

    while !stop.load(Ordering::Relaxed) {
        for (_, file) in &mut files {
            loop {
                match file.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        for chunk in buf[..n].chunks_exact(ev_size) {
                            let ev = unsafe {
                                std::ptr::read_unaligned(chunk.as_ptr() as *const libc::input_event)
                            };
                            if ev.type_ != EV_KEY {
                                continue;
                            }
                            let down = ev.value != 0;
                            match ev.code {
                                BTN_LEFT => dit = down,
                                BTN_RIGHT => dah = down,
                                BTN_MIDDLE => middle = down,
                                _ => {}
                            }
                        }
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                    Err(_) => break,
                }
            }
        }

        let now = start.elapsed().as_secs_f64();
        // The one thing this thread decides: which physical contact is dit and
        // which is dah. Nothing more — the elements are the engine's to make,
        // because they have to be made next to the transmitter that carries
        // them. The middle contact is published alongside, since a straight key
        // in a paddle box is the middle one.
        let (d, a) = if setup.reverse { (dah, dit) } else { (dit, dah) };
        let mut word = 0u8;
        if d {
            word |= CONTACT_DIT;
        }
        if a {
            word |= CONTACT_DAH;
        }
        if middle {
            word |= CONTACT_MIDDLE;
        }
        // Only on a change: the caller sends contacts, and a poll that changed
        // nothing is not a message.
        let prev = shared.contacts.swap(word, Ordering::Relaxed);
        let changed = prev != word;
        let down = d || a || middle;

        // Fill the device one millisecond at a time, from the wall clock so it
        // cannot drift; a full ring is skipped rather than blocked on.
        let want = (now * rate) as u64;
        let mut to_gen = want.saturating_sub(generated);
        let cap = (rate / 50.0) as u64; // never chase more than 20 ms
        if to_gen > cap {
            to_gen = cap;
            generated = want - cap;
        }
        let Some(ring) = ring.as_mut() else {
            std::thread::sleep(Duration::from_millis(1));
            continue;
        };
        while to_gen > 0 {
            if ring.slots() + 2 > capacity {
                break;
            }
            // The monitor follows the *contact*, not a keyer: what the operator
            // hears is their own hand, and the iambic timing they are listening
            // for is the engine's, arriving on the engine's own sidetone.
            let s = if down { (phase.sin() * 0.3) as f32 } else { 0.0 };
            phase += inc;
            if phase > std::f64::consts::TAU {
                phase -= std::f64::consts::TAU;
            }
            let _ = ring.push(s);
            let _ = ring.push(s);
            // The same tone, for a caller that decodes what was sent. Bounded,
            // and dropped rather than allowed to grow: a caller that stops
            // draining should lose old audio, not memory.
            if changed {
                tap.push(s);
                tap.push(s);
            }
            to_gen -= 1;
            generated += 1;
        }
        let _ = &out;
        if !tap.is_empty() {
            let mut guard = shared.tone.lock().unwrap();
            if guard.len() < TONE_TAP_MAX {
                guard.append(&mut tap);
            }
            tap.clear();
        }

        std::thread::sleep(Duration::from_millis(1));
    }
    for fd in &fds {
        unsafe { libc::ioctl(*fd, EVIOCGRAB, 0) };
    }
}
