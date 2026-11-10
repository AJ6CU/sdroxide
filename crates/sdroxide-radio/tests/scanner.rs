//! Scanning end to end through the engine: stopping on a signal, resuming when
//! it goes, and getting out of the operator's way.

use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, rtrb, start_engine};
use sdroxide_types::{
    Command, DeviceCaps, MemoryChannel, Mode, RadioEvent, RepeaterState, ScanKind, ScanResume,
    ScannerConfig, Shift, ToneMode, Vfo,
};

const RATE: f64 = 1_536_000.0;
const CENTER: f64 = 145_000_000.0;

/// Point the config directory at a scratch of our own, before any engine reads
/// it.
///
/// Unlike most of the suite this one cannot be read-only against the operator's
/// config: a scan is configured by a command the engine persists, and a memory
/// scan needs memories to scan. So the whole config directory is redirected —
/// this is a separate test binary, so nothing outside it is affected, and the
/// `Once` guarantees it happens before the first engine (and therefore the
/// first reader of the environment) exists.
fn isolate_config() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {
        let dir = std::env::temp_dir().join(format!("sdroxide-scan-test-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("scratch config directory");
        // SAFETY: no engine exists yet, so nothing else is reading these.
        unsafe {
            // Linux, macOS, Windows — whichever `directories` consults.
            std::env::set_var("XDG_CONFIG_HOME", &dir);
            std::env::set_var("HOME", &dir);
            std::env::set_var("APPDATA", &dir);
        }
    });
}

/// A front end that delivers a carrier at one absolute frequency, and records
/// every hardware centre it was sent to.
struct Bench {
    center_hz: f64,
    /// Where the (single) signal is, or `None` for a dead band.
    signal_hz: Arc<Mutex<Option<f64>>>,
    visited: Arc<Mutex<Vec<f64>>>,
    phase: f64,
    /// One second of modulating audio, or `None` for a bare carrier.
    ///
    /// A voice hunt is judged on the audio a channel carries, so proving it
    /// needs a station that carries some — and the difference between "a
    /// carrier" and "somebody talking" is exactly the thing under test.
    audio: Option<Vec<f32>>,
    /// Sample index since the source opened, for walking `audio`.
    n: usize,
}

impl Bench {
    fn new(signal_hz: Option<f64>) -> Self {
        Bench {
            center_hz: CENTER,
            signal_hz: Arc::new(Mutex::new(signal_hz)),
            visited: Arc::new(Mutex::new(vec![CENTER])),
            phase: 0.0,
            audio: None,
            n: 0,
        }
    }

    /// The same carrier, amplitude-modulated with a voice.
    fn speaking(signal_hz: Option<f64>) -> Self {
        Bench { audio: Some(voice_audio()), ..Bench::new(signal_hz) }
    }
}

/// One second of something a speech detector should recognise: a 120 Hz glottal
/// pulse train with two formants, gated into syllables at 4 Hz, looped forever
/// by the source so a scan can find it whatever the phase.
///
/// Generated at 48 kHz and held for thirty-two samples by the source, because
/// the point of the test is the detector and not the sample rate.
fn voice_audio() -> Vec<f32> {
    const AUDIO_RATE: f64 = 48_000.0;
    const SECS: f64 = 1.0;
    let n = (AUDIO_RATE * SECS) as usize;
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let t = i as f64 / AUDIO_RATE;
        // Syllables at 4 Hz, with the slight lift a talker gives the middle of
        // a phrase.
        let syll = 0.55 - 0.45 * (std::f64::consts::TAU * 4.0 * t).cos();
        let mut s = 0.0f64;
        for k in 1..=28 {
            let f = 120.0 * k as f64;
            let formant = 1.0
                + 2.5 * (-((f - 700.0) / 130.0f64).powi(2) / 2.0).exp()
                + 1.8 * (-((f - 1900.0) / 200.0f64).powi(2) / 2.0).exp();
            s += formant / k as f64 * (std::f64::consts::TAU * f * t).sin();
        }
        out.push((s * syll) as f32);
    }
    let peak = out.iter().fold(0.0f32, |m, s| m.max(s.abs())).max(1e-9);
    out.iter().map(|s| s * 0.7 / peak).collect()
}

impl IqSource for Bench {
    fn sample_rate(&self) -> f64 {
        RATE
    }
    fn center_hz(&self) -> f64 {
        self.center_hz
    }
    fn set_center_hz(&mut self, hz: f64) -> Result<()> {
        self.center_hz = hz;
        self.visited.lock().unwrap().push(hz);
        Ok(())
    }
    fn read(&mut self, buf: &mut [Complex32]) -> Result<usize> {
        let n = buf.len().min(16_384);
        // Real time, so the engine's dwells and grace periods mean what they say.
        std::thread::sleep(Duration::from_secs_f64(n as f64 / RATE));
        let signal = *self.signal_hz.lock().unwrap();
        match signal {
            Some(f) => {
                let step = std::f64::consts::TAU * (f - self.center_hz) / RATE;
                let audio = self.audio.as_deref();
                for s in buf[..n].iter_mut() {
                    self.phase += step;
                    let (si, co) = self.phase.sin_cos();
                    // Amplitude modulation: an envelope above the carrier is
                    // what an AM demodulator turns back into audio.
                    let env = match audio {
                        Some(a) if !a.is_empty() => 1.0 + a[(self.n / 32) % a.len()],
                        _ => 1.0,
                    };
                    self.n += 1;
                    let amp = 0.5 * env;
                    *s = Complex32::new(co as f32 * amp, si as f32 * amp);
                }
            }
            None => {
                self.n += n;
                buf[..n].fill(Complex32::new(0.0, 0.0));
            }
        }
        Ok(n)
    }
    fn describe(&self) -> String {
        "scanner bench".into()
    }
}

fn caps() -> DeviceCaps {
    DeviceCaps {
        driver: "test".into(),
        label: "test".into(),
        rx_channels: 1,
        sample_rates: vec![RATE],
        freq_ranges_rx: vec![(24_000_000.0, 1_766_000_000.0)],
        ..DeviceCaps::default()
    }
}

fn range_cfg() -> ScannerConfig {
    ScannerConfig {
        kind: ScanKind::Range,
        range_lo_hz: 144_000_000.0,
        range_hi_hz: 146_000_000.0,
        step_hz: 12_500.0,
        mode: Mode::Nfm,
        threshold_db: -60.0,
        follow_squelch: false,
        dwell_ms: 60,
        resume: ScanResume::Carrier,
        resume_ms: 300,
        skip: Vec::new(),
        ..ScannerConfig::default()
    }
}

/// The engine's state as this test last saw it.
///
/// Accumulated across calls rather than rebuilt each time, because the engine
/// only announces its state when something *changes*: a scan sitting on a
/// signal is silent, and a fresh snapshot would read that silence as "not
/// holding" and quietly invert the assertion.
#[derive(Default)]
struct Watch {
    dial: f64,
    running: bool,
    holding: bool,
    held_at: Option<f64>,
    /// The repeater setup the engine was in when it stopped — the shift and the
    /// tone that would go out if the operator answered the call.
    held_repeater: Option<sdroxide_types::RepeaterState>,
    notices: Vec<String>,
    /// The settings as the engine last persisted them — which is where a skip
    /// taken during a scan has to end up if it is to survive the next run.
    scanner: Option<ScannerConfig>,
    /// The memory store as the engine last announced it. `None` until it has
    /// said anything at all, which is how "no memories" is told apart from
    /// "not asked yet".
    memories: Option<Vec<MemoryChannel>>,
    /// The folders, announced the same way and read the same way.
    folders: Vec<sdroxide_types::MemoryFolder>,
    /// Every dial the engine has announced since the last `forget_seen`. What a
    /// scan *did not* tune to is only checkable against the whole trail.
    dials: Vec<f64>,
    /// The speech reading the engine last reported with its state.
    voice: Option<sdroxide_types::ScanVoice>,
}

struct Rig {
    h: sdroxide_radio::EngineHandles,
    thread: Option<std::thread::JoinHandle<()>>,
    visited: Arc<Mutex<Vec<f64>>>,
    signal: Arc<Mutex<Option<f64>>>,
    /// The speaker ring, drained and discarded. Its presence is the point: with
    /// it the engine builds a real receive chain, and the scanner reads the same
    /// level meter the squelch does — which is the arrangement every operator
    /// actually has.
    audio: rtrb::Consumer<f32>,
    w: Watch,
}

impl Rig {
    fn new(signal_hz: Option<f64>) -> Self {
        Rig::with(Bench::new(signal_hz))
    }

    /// A rig whose signal is somebody talking.
    fn speaking(signal_hz: Option<f64>) -> Self {
        Rig::with(Bench::speaking(signal_hz))
    }

    fn with(source: Bench) -> Self {
        isolate_config();
        let (visited, signal) = (Arc::clone(&source.visited), Arc::clone(&source.signal_hz));
        let (producer, audio) = rtrb::RingBuffer::new(48_000);
        let cfg = EngineConfig {
            audio: Some(sdroxide_radio::AudioParams { producer, out_rate: 48_000.0 }),
            ..EngineConfig::default()
        };
        let mut h = start_engine(Box::new(source), caps(), cfg);
        let thread = h.thread.take();
        Rig { h, thread, visited, signal, audio, w: Watch::default() }
    }

    fn send(&self, cmd: Command) {
        self.h.cmd_tx.send(cmd).unwrap();
    }

    /// Take in everything the engine has said since the last look. One place,
    /// so that a helper waiting on one kind of event does not drain the rest
    /// onto the floor.
    fn drain(&mut self) {
        while let Ok(ev) = self.h.event_rx.try_recv() {
            match ev {
                RadioEvent::State(s) => {
                    if self.w.dials.last() != Some(&s.active_freq_hz()) {
                        self.w.dials.push(s.active_freq_hz());
                    }
                    self.w.dial = s.active_freq_hz();
                    self.w.running = s.scan.running;
                    self.w.holding = s.scan.holding;
                    if s.scan.voice.is_some() {
                        self.w.voice = s.scan.voice;
                    }
                    if s.scan.holding && self.w.held_at.is_none() {
                        self.w.held_at = Some(s.active_freq_hz());
                        // Captured with the frequency rather than read off the
                        // last state seen: what matters is the setup in force on
                        // the channel the scan stopped on.
                        self.w.held_repeater = Some(s.repeater);
                    }
                }
                RadioEvent::Notice(Some(n)) => self.w.notices.push(n),
                RadioEvent::Scanner(c) => self.w.scanner = Some(c),
                RadioEvent::Memories(m) => self.w.memories = Some(m),
                RadioEvent::MemoryFolders(f) => self.w.folders = f,
                _ => {}
            }
        }
        // Keep the speaker ring moving so the engine is never blocked on it.
        while self.audio.pop().is_ok() {}
    }

    /// Run the engine for up to `secs`, returning early once it has stopped on
    /// something if `until_hold`.
    fn pump(&mut self, secs: f64, until_hold: bool) -> &Watch {
        let deadline = Instant::now() + Duration::from_secs_f64(secs);
        while Instant::now() < deadline {
            self.drain();
            if until_hold && self.w.held_at.is_some() {
                break;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        &self.w
    }

    /// Forget what has been *seen* so far, for an assertion about what happens
    /// next rather than what has happened at all. Nothing on the engine's side
    /// is disturbed — in particular the stored memories stay stored, which is
    /// what `clear_memories` is for.
    fn forget_seen(&mut self) {
        self.w.held_at = None;
        self.w.held_repeater = None;
        self.w.notices.clear();
        self.w.dials.clear();
        self.w.voice = None;
    }

    /// The stored channels, once the engine has at least `want` of them.
    /// Storing one is a command like any other, so the answer is only right
    /// once it has been acted on; `want` of 0 still waits for the engine's
    /// opening announcement, so an empty answer means empty.
    fn memories(&mut self, want: usize) -> Vec<MemoryChannel> {
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            self.drain();
            match &self.w.memories {
                Some(m) if m.len() >= want => return m.clone(),
                _ if Instant::now() >= deadline => {
                    return self.w.memories.clone().unwrap_or_default();
                }
                _ => std::thread::sleep(Duration::from_millis(10)),
            }
        }
    }

    /// Every dial the engine has been on since the last `forget_seen`.
    fn dials(&mut self) -> Vec<f64> {
        self.drain();
        self.w.dials.clone()
    }

    /// The folders, once the engine has at least `want` of them. Same bargain
    /// as [`Rig::memories`]: making one is a command, and the answer is only
    /// right once it has been acted on.
    fn folders(&mut self, want: usize) -> Vec<sdroxide_types::MemoryFolder> {
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            self.drain();
            if self.w.folders.len() >= want || Instant::now() >= deadline {
                return self.w.folders.clone();
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    /// Empty the memory store, for a test that asserts on what is in it.
    ///
    /// `isolate_config` redirects the config directory once per *process*, so
    /// every test in this binary shares one store and an engine starts with
    /// whatever the last test to save left in it. A test that cares how many
    /// memories there are has to make that true rather than assume it. Only
    /// this engine's own copy is at stake: these engines are built without a
    /// `StoreSync`, so nothing re-reads the file behind one's back and a store
    /// emptied here stays empty for the rest of the test, however many other
    /// tests are storing memories at the same moment.
    fn clear_memories(&mut self) {
        for m in self.memories(0) {
            self.send(Command::DeleteMemory(m.id));
        }
        let deadline = Instant::now() + Duration::from_secs(2);
        loop {
            self.drain();
            if self.w.memories.as_ref().is_some_and(|m| m.is_empty()) {
                return;
            }
            assert!(Instant::now() < deadline, "the engine never emptied its memory store");
            std::thread::sleep(Duration::from_millis(10));
        }
    }
}

impl Drop for Rig {
    fn drop(&mut self) {
        let (tx, _) = crossbeam_channel::unbounded();
        let dead = std::mem::replace(&mut self.h.cmd_tx, tx);
        drop(dead);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

/// The point of sweeping rather than stepping: a carrier anywhere in a 2 MHz
/// range is found, on the channel grid, without visiting a hundred and sixty
/// channels to get there.
#[test]
fn a_range_scan_stops_on_a_carrier() {
    const SIGNAL: f64 = 145_312_500.0;
    let mut rig = Rig::new(Some(SIGNAL));
    rig.send(Command::SetScannerConfig(range_cfg()));
    rig.send(Command::SetScanning(true));

    let held = rig.pump(6.0, true).held_at.expect("the scan should have stopped on the carrier");
    assert!(
        (held - SIGNAL).abs() <= 12_500.0,
        "stopped on {held} rather than within a channel of {SIGNAL}"
    );
}

/// A dead band is scanned round and round without ever stopping, and without
/// the dial wandering outside the range it was given.
#[test]
fn a_quiet_range_never_stops_and_stays_in_range() {
    let mut rig = Rig::new(None);
    rig.send(Command::SetScannerConfig(range_cfg()));
    rig.send(Command::SetScanning(true));

    let w = rig.pump(2.5, false);
    assert!(w.running, "the scan should still be going");
    assert_eq!(w.held_at, None, "nothing was transmitting, so nothing should have stopped it");
    for &f in rig.visited.lock().unwrap().iter().skip(1) {
        // Slices are centred inside the range, so every hardware centre has to
        // be one the range could have asked for.
        assert!(
            (143_000_000.0..=147_000_000.0).contains(&f),
            "the sweep tuned to {f}, outside the range it was given"
        );
    }
}

/// Carrier resume: the scan leaves once the signal goes, but only then — a gap
/// between overs is not the end of a conversation.
#[test]
fn a_carrier_resume_waits_for_the_signal_to_go() {
    const SIGNAL: f64 = 145_312_500.0;
    let mut rig = Rig::new(Some(SIGNAL));
    rig.send(Command::SetScannerConfig(range_cfg()));
    rig.send(Command::SetScanning(true));
    assert!(rig.pump(6.0, true).held_at.is_some(), "did not stop on the carrier to begin with");

    // Still holding well past the 300 ms grace, because the signal is still there.
    assert!(rig.pump(1.0, false).holding, "left a channel that was still busy");

    // Now take it away.
    *rig.signal.lock().unwrap() = None;
    let w = rig.pump(2.0, false);
    assert!(!w.holding, "stayed on a channel that had gone quiet");
    assert!(w.running, "resuming is not stopping");
}

/// Manual resume means manual: it sits there until it is told otherwise, and
/// NEXT is what tells it.
#[test]
fn manual_resume_stays_until_next() {
    const SIGNAL: f64 = 145_312_500.0;
    let mut rig = Rig::new(Some(SIGNAL));
    rig.send(Command::SetScannerConfig(ScannerConfig {
        resume: ScanResume::Manual,
        ..range_cfg()
    }));
    rig.send(Command::SetScanning(true));
    assert!(rig.pump(6.0, true).held_at.is_some(), "did not stop on the carrier");

    *rig.signal.lock().unwrap() = None;
    assert!(rig.pump(1.5, false).holding, "manual resume left on its own");

    rig.send(Command::ScanNext);
    let w = rig.pump(1.0, false);
    assert!(!w.holding, "NEXT did not move it on");
    assert!(w.running);
}

/// Touching the dial stops the scan. Every scanner works this way, and the
/// alternative is the engine and the operator fighting over the VFO.
#[test]
fn tuning_by_hand_stops_the_scan() {
    let mut rig = Rig::new(None);
    rig.send(Command::SetScannerConfig(range_cfg()));
    rig.send(Command::SetScanning(true));
    assert!(rig.pump(0.6, false).running, "the scan should have started");

    rig.send(Command::SetVfo { vfo: Vfo::A, hz: 145_500_000.0 });
    let w = rig.pump(0.6, false);
    assert!(!w.running, "the scan kept going after the operator tuned");
    assert!(
        (w.dial - 145_500_000.0).abs() < 1.0,
        "and it should have left the dial where they put it, not at {}",
        w.dial
    );
}

/// A memory scan visits the stored channels, and a skipped one is not among
/// them.
#[test]
fn a_memory_scan_passes_over_skipped_channels() {
    const SIGNAL: f64 = 145_600_000.0;
    let mut rig = Rig::new(Some(SIGNAL));
    // The store is shared with the rest of the binary, so start from none.
    rig.clear_memories();
    // Two channels: a quiet one, and the one the carrier is on.
    rig.send(Command::SetVfo { vfo: Vfo::A, hz: 145_200_000.0 });
    rig.send(Command::StoreMemory { name: "quiet".into() });
    rig.send(Command::SetVfo { vfo: Vfo::A, hz: SIGNAL });
    rig.send(Command::StoreMemory { name: "busy".into() });
    let mems = rig.memories(2);
    assert_eq!(mems.len(), 2, "both memories should have been stored");
    let busy = mems.iter().find(|m| m.name == "busy").expect("the busy memory").id;

    let cfg = ScannerConfig {
        kind: ScanKind::Memories,
        threshold_db: -60.0,
        dwell_ms: 60,
        resume: ScanResume::Manual,
        ..range_cfg()
    };
    rig.send(Command::SetScannerConfig(cfg.clone()));
    rig.send(Command::SetScanning(true));
    let held = rig.pump(5.0, true).held_at.expect("should have stopped on the busy memory");
    assert!((held - SIGNAL).abs() < 1.0, "stopped on {held} rather than {SIGNAL}");

    // Skip it, and now there is nothing left to stop on.
    rig.send(Command::SetScanning(false));
    rig.send(Command::SetScannerConfig(ScannerConfig { skip: vec![busy], ..cfg }));
    rig.forget_seen();
    rig.send(Command::SetScanning(true));
    assert_eq!(rig.pump(2.5, true).held_at, None, "it stopped on a channel it was told to skip");
}

/// A memory scan carries each channel's stored repeater setup with it, in both
/// directions: stopping on a repeater channel puts the shift and the tone on,
/// and stopping on a simplex one takes them off again (issue #264).
///
/// This matters more in a scan than in a recall. A recall is a channel the
/// operator picked; a scan hands them whichever channel called, and they answer
/// it by reaching straight for the PTT — so a shift left standing from the last
/// stop transmits 600 kHz away from the station that is calling them.
#[test]
fn a_memory_scan_carries_each_channels_repeater_setup() {
    const RPTR: f64 = 145_600_000.0;
    const SIMPLEX: f64 = 145_500_000.0;
    let mut rig = Rig::new(Some(RPTR));
    rig.clear_memories();

    // One repeater channel and one simplex one, each stored with the setup it
    // is worked on — which is what every memory stores nowadays.
    let shifted = RepeaterState {
        shift: Shift::Minus,
        offset_hz: 600_000,
        tone: ToneMode::Ctcss,
        ..RepeaterState::default()
    };
    rig.send(Command::SetRepeater(shifted));
    rig.send(Command::SetVfo { vfo: Vfo::A, hz: RPTR });
    rig.send(Command::StoreMemory { name: "repeater".into() });
    rig.send(Command::SetRepeater(RepeaterState::default()));
    rig.send(Command::SetVfo { vfo: Vfo::A, hz: SIMPLEX });
    rig.send(Command::StoreMemory { name: "simplex".into() });
    assert_eq!(rig.memories(2).len(), 2, "both memories should have been stored");

    let cfg = ScannerConfig {
        kind: ScanKind::Memories,
        threshold_db: -60.0,
        dwell_ms: 60,
        resume: ScanResume::Manual,
        ..range_cfg()
    };
    rig.send(Command::SetScannerConfig(cfg));
    rig.send(Command::SetScanning(true));
    let w = rig.pump(5.0, true);
    let held = w.held_at.expect("should have stopped on the repeater channel");
    assert!((held - RPTR).abs() < 1.0, "stopped on {held} rather than {RPTR}");
    let r = w.held_repeater.expect("a state came with the stop");
    assert_eq!(
        (r.shift, r.tone),
        (Shift::Minus, ToneMode::Ctcss),
        "the repeater channel was not put into its stored setup: {r:?}"
    );

    // And now the way round the bug was reported: the radio is sitting in that
    // repeater's shift with its tone on, and the channel that calls next is a
    // simplex one.
    rig.send(Command::SetScanning(false));
    *rig.signal.lock().unwrap() = Some(SIMPLEX);
    rig.forget_seen();
    rig.send(Command::SetScanning(true));
    let w = rig.pump(5.0, true);
    let held = w.held_at.expect("should have stopped on the simplex channel");
    assert!((held - SIMPLEX).abs() < 1.0, "stopped on {held} rather than {SIMPLEX}");
    let r = w.held_repeater.expect("a state came with the stop");
    assert_eq!(
        (r.shift, r.tone),
        (Shift::Simplex, ToneMode::Off),
        "the simplex channel was left in the last repeater's setup: {r:?}"
    );
}

/// A fast memory scan reads its channels off the wideband spectrum instead of
/// visiting each one (issue #228): a whole band's worth of memories is one tune
/// a lap, and only the channel something is on is ever listened to.
///
/// What is asserted is both halves — that it still finds the carrier, and that
/// it got there without visiting the quiet channels, which is the entire point.
#[test]
fn a_fast_memory_scan_finds_the_carrier_without_visiting_the_rest() {
    const SIGNAL: f64 = 145_400_000.0;
    let mut rig = Rig::new(Some(SIGNAL));
    rig.clear_memories();
    // Twenty channels 25 kHz apart, all inside one 1.536 MHz window, with the
    // carrier on one of them.
    let base = 145_000_000.0;
    let busy_at = ((SIGNAL - base) / 25_000.0).round() as usize;
    for i in 0..20 {
        rig.send(Command::SetVfo { vfo: Vfo::A, hz: base + i as f64 * 25_000.0 });
        rig.send(Command::StoreMemory { name: format!("ch{i}") });
    }
    assert_eq!(rig.memories(20).len(), 20, "all twenty should have been stored");

    let cfg = ScannerConfig {
        kind: ScanKind::Memories,
        mem_fast: true,
        threshold_db: -60.0,
        dwell_ms: 60,
        resume: ScanResume::Manual,
        ..range_cfg()
    };
    rig.send(Command::SetScannerConfig(cfg.clone()));
    // Off the list before the trail starts, so the dial the scan *inherits* —
    // the last channel stored — is not mistaken for one it went to.
    rig.send(Command::SetVfo { vfo: Vfo::A, hz: 144_500_000.0 });
    // Drained before the trail is cleared: the announcements of the stores
    // above are still in the channel, and they name the channels they stored.
    rig.pump(0.5, false);
    rig.forget_seen();
    rig.send(Command::SetScanning(true));
    let held = rig.pump(5.0, true).held_at.expect("should have stopped on the busy channel");
    assert!((held - SIGNAL).abs() < 1.0, "stopped on {held} rather than {SIGNAL}");
    rig.send(Command::SetScanning(false));

    // …and the nineteen quiet ones were never tuned to. The dial visits the
    // window's centre and then the one channel worth listening to; a scan that
    // walked the list would have been on every one of them.
    let dials = rig.dials();
    let quiet: Vec<f64> = (0..20)
        .filter(|&i| i != busy_at)
        .map(|i| base + i as f64 * 25_000.0)
        .filter(|hz| dials.iter().any(|d| (d - hz).abs() < 1.0))
        .collect();
    assert!(quiet.is_empty(), "the sweep tuned to quiet channels: {quiet:?}");
}

/// A memory scan can be pointed at chosen folders (issue #236). A station's
/// memories are filed by service — marine, airband, the local repeaters — and a
/// scan of all of them at once spends most of its time somewhere nobody is
/// listening.
///
/// No selection is still every folder, which is what every setting written
/// before this existed means and what a folder made tomorrow falls into.
#[test]
fn a_memory_scan_runs_over_the_folders_it_was_given() {
    const SIGNAL: f64 = 145_700_000.0;
    let mut rig = Rig::new(Some(SIGNAL));
    rig.clear_memories();
    // Two channels in two folders: the carrier is on the one in "busy folder".
    rig.send(Command::CreateMemoryFolder { name: "quiet folder".into() });
    rig.send(Command::CreateMemoryFolder { name: "busy folder".into() });
    let folders = rig.folders(2);
    let folder = |name: &str| folders.iter().find(|f| f.name == name).expect(name).id;

    rig.send(Command::SetVfo { vfo: Vfo::A, hz: 145_250_000.0 });
    rig.send(Command::StoreMemory { name: "quiet".into() });
    rig.send(Command::SetVfo { vfo: Vfo::A, hz: SIGNAL });
    rig.send(Command::StoreMemory { name: "busy".into() });
    let mems = rig.memories(2);
    let id = |name: &str| mems.iter().find(|m| m.name == name).expect(name).id;
    rig.send(Command::MoveMemoryToFolder { id: id("quiet"), folder: Some(folder("quiet folder")) });
    rig.send(Command::MoveMemoryToFolder { id: id("busy"), folder: Some(folder("busy folder")) });

    let base = ScannerConfig {
        kind: ScanKind::Memories,
        threshold_db: -60.0,
        dwell_ms: 60,
        resume: ScanResume::Manual,
        ..range_cfg()
    };

    // The quiet folder alone: the carrier is in the other one, so nothing to
    // stop on — even though the scan can plainly hear it, which is the whole
    // assertion.
    rig.send(Command::SetScannerConfig(ScannerConfig {
        folders: vec![Some(folder("quiet folder"))],
        ..base.clone()
    }));
    rig.forget_seen();
    rig.send(Command::SetScanning(true));
    assert_eq!(
        rig.pump(2.5, true).held_at,
        None,
        "it stopped on a channel filed under a folder it was not given"
    );

    // The busy one, and it finds it.
    rig.send(Command::SetScanning(false));
    rig.send(Command::SetScannerConfig(ScannerConfig {
        folders: vec![Some(folder("busy folder"))],
        ..base.clone()
    }));
    rig.forget_seen();
    rig.send(Command::SetScanning(true));
    let held = rig.pump(5.0, true).held_at.expect("the chosen folder's busy channel");
    assert!((held - SIGNAL).abs() < 1.0, "stopped on {held} rather than {SIGNAL}");

    // And an empty selection is every folder, as it always was.
    rig.send(Command::SetScanning(false));
    rig.send(Command::SetScannerConfig(base));
    rig.forget_seen();
    rig.send(Command::SetScanning(true));
    let held = rig.pump(5.0, true).held_at.expect("no selection should scan everything");
    assert!((held - SIGNAL).abs() < 1.0, "stopped on {held} rather than {SIGNAL}");
    rig.send(Command::SetScanning(false));
}

/// SKIP on a range scan means the same as SKIP on a memory scan: not this one,
/// not now and not next time round either. There is no channel to name, so the
/// frequency is what gets remembered — and it has to survive the scan being
/// stopped and started, or an operator dismissing a data channel would have to
/// dismiss it again after every pause.
#[test]
fn a_skipped_range_channel_stays_skipped_across_runs() {
    const SIGNAL: f64 = 145_312_500.0;
    let mut rig = Rig::new(Some(SIGNAL));
    rig.send(Command::SetScannerConfig(range_cfg()));
    rig.send(Command::SetScanning(true));
    let held = rig.pump(6.0, true).held_at.expect("did not stop on the carrier to begin with");
    assert!((held - SIGNAL).abs() <= 12_500.0, "stopped on {held}, not near {SIGNAL}");

    rig.send(Command::ScanSkip);
    rig.forget_seen();
    let w = rig.pump(3.0, true);
    assert!(w.running, "SKIP is not STOP");
    assert_eq!(w.held_at, None, "it stopped again on the channel it was told to skip");

    // Persisted, not merely held in the running scan's queue.
    let saved = w.scanner.clone().expect("the engine should have announced the saved settings");
    assert!(
        saved.skips_freq(SIGNAL),
        "the skipped channel is not in the saved settings: {:?}",
        saved.skip_freq_hz
    );

    // A new run of the same range honours it.
    rig.send(Command::SetScanning(false));
    rig.send(Command::SetScanning(true));
    rig.forget_seen();
    let w = rig.pump(3.0, true);
    assert!(w.running, "the second run should be going");
    assert_eq!(w.held_at, None, "a fresh run forgot the skip");
}

/// The skips belong to the range they were taken in. Scanning somewhere else
/// starts with a clean sheet — a channel dismissed on 2 m says nothing about
/// the same arithmetic on 70 cm, and a scanner silently refusing to stop
/// somewhere the operator does not remember dismissing is a bug they cannot see.
#[test]
fn retuning_the_range_forgets_its_skips() {
    const SIGNAL: f64 = 145_312_500.0;
    let mut rig = Rig::new(Some(SIGNAL));
    rig.send(Command::SetScannerConfig(range_cfg()));
    rig.send(Command::SetScanning(true));
    assert!(rig.pump(6.0, true).held_at.is_some(), "did not stop on the carrier");
    rig.send(Command::ScanSkip);
    rig.forget_seen();
    assert_eq!(rig.pump(2.0, true).held_at, None, "the skip did not take");

    // Move the range, then move it back: the skip taken in the old one is gone.
    rig.send(Command::SetScanning(false));
    rig.send(Command::SetScannerConfig(ScannerConfig {
        range_lo_hz: 430_000_000.0,
        range_hi_hz: 432_000_000.0,
        ..range_cfg()
    }));
    rig.send(Command::SetScannerConfig(range_cfg()));
    rig.send(Command::SetScanning(true));
    rig.forget_seen();
    let w = rig.pump(6.0, true);
    let held = w.held_at.expect("the skip should not have followed the range away and back");
    assert!((held - SIGNAL).abs() <= 12_500.0, "stopped on {held}, not near {SIGNAL}");
}

/// Asking for a scan that could never stop anywhere says so rather than
/// pretending to run.
#[test]
fn an_impossible_scan_is_refused_with_a_reason() {
    let mut rig = Rig::new(None);
    rig.send(Command::SetScannerConfig(ScannerConfig {
        range_lo_hz: 145_000_000.0,
        range_hi_hz: 145_000_000.0,
        ..range_cfg()
    }));
    rig.send(Command::SetScanning(true));
    let w = rig.pump(1.0, false);
    assert!(!w.running, "an empty range should not start a scan");
    assert!(
        w.notices.iter().any(|n| n.contains("range")),
        "the operator should be told why, got {:?}",
        w.notices
    );

    // And a memory scan with nothing to scan — which means emptying the store
    // first, because it is shared with whatever else in this binary is running
    // and another test stores two channels in it.
    rig.forget_seen();
    rig.clear_memories();
    rig.send(Command::SetScannerConfig(ScannerConfig { kind: ScanKind::Memories, ..range_cfg() }));
    rig.send(Command::SetScanning(true));
    let w = rig.pump(1.0, false);
    assert!(!w.running, "a memory scan with no memories should not start");
    assert!(
        w.notices.iter().any(|n| n.contains("memory")),
        "the operator should be told why, got {:?}",
        w.notices
    );
}

/// The whole point of the voice gate: a carrier strong enough to stop the scan
/// is not a person, and a voice hunt goes past it.
///
/// The first half is what makes the second half mean anything — the same
/// carrier, with the gate off, stops the scan exactly as it always has. Without
/// it this test would pass on a scan that had simply gone deaf.
#[test]
fn a_voice_hunt_goes_past_a_bare_carrier() {
    const SIGNAL: f64 = 145_312_500.0;
    let mut rig = Rig::new(Some(SIGNAL));

    // The level alone stops it.
    rig.send(Command::SetScannerConfig(ScannerConfig { mode: Mode::Am, ..range_cfg() }));
    rig.send(Command::SetScanning(true));
    assert!(
        rig.pump(6.0, true).held_at.is_some(),
        "the bench carrier did not stop the scan at all, so the rest of this proves nothing"
    );

    // Now the same carrier, judged as speech.
    rig.send(Command::SetScanning(false));
    // Let the stop land before forgetting it: the engine announces the end of a
    // scan like anything else, and a state still in flight would be counted as
    // the next half's first stop.
    rig.pump(0.4, false);
    rig.forget_seen();
    rig.send(Command::SetScannerConfig(ScannerConfig {
        mode: Mode::Am,
        voice_only: true,
        ..range_cfg()
    }));
    rig.send(Command::SetScanning(true));
    let w = rig.pump(4.0, true);
    assert_eq!(w.held_at, None, "an unmodulated carrier was taken for a voice");
    assert!(w.running, "the scan should still be hunting, not stopped");

    // And the reason is on the wire, which is the difference between a control
    // that did nothing and one that declined: the reading named the channel and
    // said it was not a voice.
    let v = w.voice.expect("the scan reported no speech reading at all");
    assert!(
        (v.freq_hz as f64 - SIGNAL).abs() <= 12_500.0,
        "the reading was about {} rather than the carrier at {SIGNAL}",
        v.freq_hz
    );
    assert!(v.score < 45, "a bare carrier scored {}", v.score);
}

/// And the other half: a channel carrying speech stops it.
#[test]
fn a_voice_hunt_stops_on_a_voice() {
    const SIGNAL: f64 = 145_312_500.0;
    let mut rig = Rig::speaking(Some(SIGNAL));
    rig.send(Command::SetScannerConfig(ScannerConfig {
        mode: Mode::Am,
        voice_only: true,
        // Long enough to hear a syllable, and a grace that rides out the gaps
        // between words rather than leaving in the middle of a sentence.
        dwell_ms: 800,
        resume_ms: 1_500,
        ..range_cfg()
    }));
    rig.send(Command::SetScanning(true));
    let w = rig.pump(10.0, true);
    let held = w.held_at.expect("the scan never stopped on a channel with a voice on it");
    assert!((held - SIGNAL).abs() <= 12_500.0, "stopped on {held} rather than near {SIGNAL}");
    let v = w.voice.expect("a voice stop with no reading to show for it");
    assert!(v.score >= 60, "the channel it stopped on scored {} for speech", v.score);
}

/// The gate is a promise, so a scan that cannot keep it refuses with a reason
/// rather than falling back to the level behind the operator's back.
#[test]
fn a_voice_hunt_that_cannot_be_had_says_so() {
    let mut rig = Rig::new(None);
    // A mode that carries no speech, in a range that is otherwise perfectly
    // scanable: the scan would run and stop nowhere.
    rig.send(Command::SetScannerConfig(ScannerConfig {
        mode: Mode::Ft8,
        voice_only: true,
        ..range_cfg()
    }));
    rig.send(Command::SetScanning(true));
    let w = rig.pump(1.0, false);
    assert!(!w.running, "a voice hunt over a mode with no voice should not start");
    assert!(
        w.notices.iter().any(|n| n.contains("speech")),
        "the operator should be told why, got {:?}",
        w.notices
    );
}
