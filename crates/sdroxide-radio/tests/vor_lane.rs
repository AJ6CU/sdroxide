//! The VOR lane, from a mock front end to the read-out on the AIRCRAFT window.
//!
//! The decoder's own arithmetic is pinned in `sdroxide-vor`. What cannot be seen
//! from there is whether the engine carries a beacon to it: the dial deciding
//! that the lane runs, the down-converter placed on the dial, the envelope taken
//! with nothing filtered in between, the status that comes out. A synthetic
//! beacon — AM carrier, 30 Hz variable at a known radial, 9960 Hz subcarrier
//! frequency-modulated at 30 Hz, a keyed ident — goes in as I/Q, and the radial
//! and the ident have to come out.
//!
//! * **The lane follows the dial.** Inside 108–118 MHz it runs, whatever the
//!   mode; off it, it says nothing.
//! * **A front end that hands over audio is told so**, rather than showing a
//!   read-out that can never fill.

use std::f64::consts::TAU;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sdroxide_radio::{AudioParams, Complex32, EngineConfig, IqSource, Result, rtrb, start_engine};
use sdroxide_types::{Command, DeviceCaps, Mode, RadioEvent, RxId, Vfo, VorStatus, VorUnavailable};

/// The channel the beacon is on.
const VOR_HZ: f64 = 114_100_000.0;
/// The stream the stand-in front end delivers.
const RATE: f64 = 480_000.0;
/// How far the carrier sits from the hardware centre, Hz. A whole number of
/// cycles in the loop, so the loop has no seam.
const CARRIER_OFFSET_HZ: f64 = 2_000.0;
/// The loop: the ident repeats every six seconds.
const LOOP_S: f64 = 6.0;
const IDENT: &str = "BBG";
const DOT_S: f64 = 0.12;

/// Whether the ident key is down at `t` seconds.
fn key_down(t: f64) -> bool {
    let mut at = 0.0;
    let mut marks = Vec::new();
    for (i, ch) in IDENT.chars().enumerate() {
        if i > 0 {
            at += 3.0 * DOT_S;
        }
        for (j, sym) in sdroxide_vor::morse::encode_char(ch).unwrap().chars().enumerate() {
            if j > 0 {
                at += DOT_S;
            }
            let len = if sym == '-' { 3.0 * DOT_S } else { DOT_S };
            marks.push((at, at + len));
            at += len;
        }
    }
    let phase = t.rem_euclid(LOOP_S);
    marks.iter().any(|&(a, b)| phase >= a && phase < b)
}

/// One loop of a beacon at `radial_deg`, as complex baseband about a centre
/// [`CARRIER_OFFSET_HZ`] below the carrier.
fn beacon(radial_deg: f64, with_ident: bool) -> Vec<Complex32> {
    let n = (RATE * LOOP_S) as usize;
    let theta = radial_deg.to_radians();
    let mut noise = 0x5EED_u64;
    let mut rnd = move || {
        noise = noise.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        ((noise >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0
    };
    (0..n)
        .map(|i| {
            let t = i as f64 / RATE;
            let var = 0.30 * (TAU * 30.0 * t - theta).cos();
            let reference = 0.30 * (TAU * 9960.0 * t + 16.0 * (TAU * 30.0 * t).sin()).cos();
            let ident = if with_ident && key_down(t) { 0.10 * (TAU * 1020.0 * t).cos() } else { 0.0 };
            let env = 0.4 * (1.0 + var + reference + ident);
            let (s, c) = (TAU * CARRIER_OFFSET_HZ * t).sin_cos();
            Complex32::new((env * c + 0.002 * rnd()) as f32, (env * s + 0.002 * rnd()) as f32)
        })
        .collect()
}

/// A front end with one beacon on the air.
struct Band {
    center: Arc<Mutex<f64>>,
    samples: Vec<Complex32>,
    pos: usize,
}

impl IqSource for Band {
    fn sample_rate(&self) -> f64 {
        RATE
    }
    fn center_hz(&self) -> f64 {
        *self.center.lock().unwrap()
    }
    fn set_center_hz(&mut self, hz: f64) -> Result<()> {
        *self.center.lock().unwrap() = hz;
        Ok(())
    }
    fn read(&mut self, buf: &mut [Complex32]) -> Result<usize> {
        // Not real time, but not a flood either: about three times real time.
        std::thread::sleep(Duration::from_millis(5));
        let n = buf.len().min(8192);
        for z in buf[..n].iter_mut() {
            *z = self.samples[self.pos];
            self.pos = (self.pos + 1) % self.samples.len();
        }
        Ok(n)
    }
    fn describe(&self) -> String {
        "one VOR beacon".into()
    }
}

fn caps(audio_mode: bool) -> DeviceCaps {
    DeviceCaps {
        driver: "mock".into(),
        label: "mock".into(),
        rx_channels: 1,
        sample_rates: vec![RATE],
        freq_ranges_rx: vec![(1_000_000.0, 2_000_000_000.0)],
        audio_mode,
        ..DeviceCaps::default()
    }
}

fn isolate_config(name: &str) {
    let root = std::env::temp_dir().join(format!("sdroxide-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    // The variable is process-global; unset means the operator's live config.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };
}

fn engine(radial_deg: f64, with_ident: bool, audio_mode: bool) -> sdroxide_radio::EngineHandles {
    // The lanes are fed from the same pass as the speaker, so an engine with
    // nowhere to play audio never reaches them: a ring nothing reads.
    let (producer, _consumer) = rtrb::RingBuffer::<f32>::new(48_000);
    let center = Arc::new(Mutex::new(VOR_HZ - CARRIER_OFFSET_HZ));
    start_engine(
        Box::new(Band { center, samples: beacon(radial_deg, with_ident), pos: 0 }),
        caps(audio_mode),
        EngineConfig {
            remember_session: false,
            audio: Some(AudioParams { producer, out_rate: 48_000.0 }),
            ..Default::default()
        },
    )
}

/// Wait for a status that satisfies `f`, or say what the last one was.
fn status(
    h: &sdroxide_radio::EngineHandles,
    what: &str,
    secs: u64,
    mut f: impl FnMut(&VorStatus) -> bool,
) -> VorStatus {
    let deadline = Instant::now() + Duration::from_secs(secs);
    let mut last: Option<VorStatus> = None;
    while Instant::now() < deadline {
        while let Ok(ev) = h.event_rx.try_recv() {
            if let RadioEvent::VorStatus(s) = ev {
                if f(&s) {
                    return *s;
                }
                last = Some(*s);
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    panic!("the VOR lane never reported {what}; last status: {last:?}");
}

fn finish(mut h: sdroxide_radio::EngineHandles) {
    let thread = h.thread.take();
    drop(h);
    if let Some(t) = thread {
        let _ = t.join();
    }
}

fn err_deg(a: f32, b: f64) -> f64 {
    let d = (a as f64 - b).rem_euclid(360.0);
    d.min(360.0 - d)
}

#[test]
fn the_radial_of_a_beacon_reaches_the_read_out() {
    for radial in [0.0, 137.0, 215.0, 359.0] {
        isolate_config("vor-radial");
        let h = engine(radial, false, false);
        h.cmd_tx.send(Command::SetVfo { vfo: Vfo::A, hz: VOR_HZ }).unwrap();
        h.cmd_tx.send(Command::SetMode { rx: RxId::Main, mode: Mode::Am }).unwrap();

        let st = status(&h, &format!("a lock on the {radial}° radial"), 30, |s| {
            s.running && s.locked
        });
        assert!(st.unavailable.is_none(), "{radial}: {:?}", st.unavailable);
        assert!((st.freq_hz - VOR_HZ).abs() < 1.0, "{radial}: channel {}", st.freq_hz);
        assert!(
            err_deg(st.radial_deg, radial) <= 1.0,
            "{radial}: the read-out says {}°",
            st.radial_deg
        );
        assert!((st.var_depth - 0.30).abs() < 0.03, "{radial}: depth {}", st.var_depth);
        assert!((st.deviation_hz - 480.0).abs() < 30.0, "{radial}: deviation {}", st.deviation_hz);
        finish(h);
    }
}

#[test]
fn the_ident_reaches_the_read_out() {
    isolate_config("vor-ident");
    let h = engine(100.0, true, false);
    h.cmd_tx.send(Command::SetVfo { vfo: Vfo::A, hz: VOR_HZ }).unwrap();
    h.cmd_tx.send(Command::SetMode { rx: RxId::Main, mode: Mode::Am }).unwrap();

    let st = status(&h, "the ident", 60, |s| s.ident.is_some());
    assert_eq!(st.ident.as_deref(), Some(IDENT));
    assert!(st.running);
    finish(h);
}

#[test]
fn the_lane_stays_quiet_off_the_navigation_band() {
    isolate_config("vor-off-band");
    let h = engine(100.0, false, false);
    h.cmd_tx.send(Command::SetVfo { vfo: Vfo::A, hz: 14_200_000.0 }).unwrap();
    h.cmd_tx.send(Command::SetMode { rx: RxId::Main, mode: Mode::Usb }).unwrap();

    let until = Instant::now() + Duration::from_secs(4);
    while Instant::now() < until {
        while let Ok(ev) = h.event_rx.try_recv() {
            if let RadioEvent::VorStatus(s) = ev {
                assert!(!s.running, "the lane ran on 14.2 MHz: {s:?}");
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    finish(h);
}

#[test]
fn a_front_end_that_hands_over_audio_is_told_so() {
    isolate_config("vor-audio-front");
    let h = engine(100.0, false, true);
    h.cmd_tx.send(Command::SetVfo { vfo: Vfo::A, hz: VOR_HZ }).unwrap();
    h.cmd_tx.send(Command::SetMode { rx: RxId::Main, mode: Mode::Am }).unwrap();

    let st = status(&h, "the reason it cannot run", 15, |s| s.unavailable.is_some());
    assert_eq!(st.unavailable, Some(VorUnavailable::AudioFront));
    assert!(!st.running);
    finish(h);
}
