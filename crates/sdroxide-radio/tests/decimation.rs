//! Front-end decimation: the RX box's DEC chip, on any source that delivers IQ.
//!
//! The engine keeps the hardware streaming exactly as it was and thins the
//! stream itself, so what has to hold is that everything derived from "how much
//! bandwidth is there" follows the *decimated* rate — the published state, the
//! panadapter's span, and where a signal lands inside it — and that a factor
//! the device cannot carry never reaches the DSP.

use std::f64::consts::TAU;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use sdroxide_radio::{Complex32, EngineConfig, EngineHandles, IqSource, Result, start_engine};
use sdroxide_types::{Command, DeviceCaps, RadioEvent, RadioState};

const RATE: f64 = 768_000.0;
const CENTER: f64 = 14_100_000.0;
/// Where the test signal sits relative to the centre — well inside the ±48 kHz
/// that survives a /8, so the same signal is present in both spans.
const TONE_OFFSET_HZ: f64 = 30_000.0;

/// A front end streaming one tone, at whatever rate it was built for.
struct ToneSource {
    rate_hz: f64,
    center_hz: f64,
    phase: f64,
    audio_mode: bool,
    /// What a zero-IF front end asks for: its LO parked this far above the VFO.
    lo_offset_hz: f64,
    /// Where the engine last put the LO.
    landed: Arc<Mutex<f64>>,
}

impl ToneSource {
    fn new(rate_hz: f64) -> Self {
        ToneSource {
            rate_hz,
            center_hz: CENTER,
            phase: 0.0,
            audio_mode: false,
            lo_offset_hz: 0.0,
            landed: Arc::new(Mutex::new(CENTER)),
        }
    }
}

impl IqSource for ToneSource {
    fn sample_rate(&self) -> f64 {
        self.rate_hz
    }
    fn center_hz(&self) -> f64 {
        self.center_hz
    }
    fn lo_offset_hz(&self) -> f64 {
        self.lo_offset_hz
    }
    fn set_center_hz(&mut self, hz: f64) -> Result<()> {
        self.center_hz = hz;
        *self.landed.lock().unwrap() = hz;
        Ok(())
    }
    fn read(&mut self, buf: &mut [Complex32]) -> Result<usize> {
        // A block big enough that a decimated FFT still fills in a few reads,
        // paced so the engine's loop runs at a realistic rate.
        std::thread::sleep(Duration::from_millis(5));
        let n = buf.len().min(8192);
        let step = TAU * TONE_OFFSET_HZ / self.rate_hz;
        for s in &mut buf[..n] {
            *s = Complex32::new(self.phase.cos() as f32, self.phase.sin() as f32);
            self.phase = (self.phase + step) % TAU;
        }
        Ok(n)
    }
    fn describe(&self) -> String {
        "tone generator".into()
    }
}

fn caps(rate_hz: f64, audio_mode: bool) -> DeviceCaps {
    DeviceCaps {
        driver: "test".into(),
        label: "test".into(),
        rx_channels: 1,
        sample_rates: vec![rate_hz],
        freq_ranges_rx: vec![(1_000_000.0, 60_000_000.0)],
        audio_mode,
        ..DeviceCaps::default()
    }
}

/// The last state the engine published within `secs`.
fn settle(h: &mut EngineHandles, secs: f64) -> RadioState {
    let mut state = None;
    let deadline = Instant::now() + Duration::from_secs_f64(secs);
    while Instant::now() < deadline {
        while let Ok(ev) = h.event_rx.try_recv() {
            if let RadioEvent::State(s) = ev {
                state = Some(s);
            }
        }
        std::thread::sleep(Duration::from_millis(10));
    }
    state.expect("the engine publishes its state")
}

/// Ask for `factor` on a device streaming `rate_hz` and report what the engine
/// settled on: (decimation, receiver rate).
fn ask_for(factor: u32, rate_hz: f64, audio_mode: bool) -> (u32, f64) {
    let mut source = ToneSource::new(rate_hz);
    source.audio_mode = audio_mode;
    let mut h = start_engine(
        Box::new(source),
        caps(rate_hz, audio_mode),
        EngineConfig { remember_session: false, ..Default::default() },
    );
    let thread = h.thread.take();
    h.cmd_tx.send(Command::SetDecimation(factor)).unwrap();
    let state = settle(&mut h, 0.5);
    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    (state.decimation, state.sample_rate)
}

#[test]
fn decimating_divides_the_rate_the_receiver_runs_at() {
    assert_eq!(ask_for(4, RATE, false), (4, RATE / 4.0));
    // And back off again, which has to restore the device's own rate exactly.
    assert_eq!(ask_for(1, RATE, false), (1, RATE));
}

/// The chip only offers powers of two, but the command is on the wire protocol
/// and a client may send anything.
#[test]
fn a_factor_that_is_not_a_power_of_two_rounds_down() {
    assert_eq!(ask_for(6, RATE, false).0, 4);
    assert_eq!(ask_for(3, RATE, false).0, 2);
}

/// A request the device has no bandwidth for is clamped rather than refused —
/// what comes back is the deepest decimation it can carry. 768 kHz allows /16
/// (48 kHz left) and no more.
#[test]
fn a_factor_the_device_cannot_carry_comes_back_clamped() {
    assert_eq!(ask_for(64, RATE, false), (16, 48_000.0));
    // A device already streaming a narrow span has nothing to give up at all.
    assert_eq!(ask_for(8, 48_000.0, false), (1, 48_000.0));
}

/// A CAT rig on a sound card delivers demodulated audio, not IQ: there is no
/// span to trade and the state must say so rather than report a decimation the
/// receiver is not doing.
#[test]
fn an_audio_mode_source_has_no_iq_to_decimate() {
    assert_eq!(ask_for(4, RATE, true).0, 1);
}

/// A zero-IF front end parks its LO clear of the VFO — a quarter of its span
/// above it — and the DDC brings the signal back down. That offset is a
/// fraction of the span, so it has to shrink with the decimation: left at the
/// device's own figure it would sit outside the bandwidth the decimator keeps,
/// and the VFO would be tuned to spectrum that is no longer in the stream. On
/// screen that is a receiver that goes deaf when the DEC chip is clicked.
#[test]
fn a_zero_if_lo_offset_shrinks_with_the_span() {
    const WIDE: f64 = 2_000_000.0;
    let mut source = ToneSource::new(WIDE);
    source.lo_offset_hz = WIDE * 0.25;
    let landed = Arc::clone(&source.landed);
    let mut h = start_engine(
        Box::new(source),
        caps(WIDE, false),
        EngineConfig { remember_session: false, ..Default::default() },
    );
    let thread = h.thread.take();

    let state = settle(&mut h, 0.5);
    let vfo = state.active_freq_hz();
    assert!(
        (*landed.lock().unwrap() - (vfo + WIDE * 0.25)).abs() < 1.0,
        "undecimated, the LO should be a quarter of 2 Msps above the VFO"
    );

    h.cmd_tx.send(Command::SetDecimation(8)).unwrap();
    let state = settle(&mut h, 0.5);
    let lo = *landed.lock().unwrap();
    assert_eq!(state.decimation, 8);
    assert!(
        (lo - (vfo + WIDE / 8.0 * 0.25)).abs() < 1.0,
        "the LO should have moved to a quarter of the decimated span above the \
         VFO, not to {lo}"
    );
    // Which is the property that actually matters: the VFO is still inside the
    // bandwidth the receiver is being handed.
    assert!((vfo - lo).abs() < state.sample_rate / 2.0, "the VFO fell outside the decimated span");

    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
}

/// The end-to-end one: after decimating, the panadapter covers the narrower
/// span and the signal is still at the frequency it was on. A span that
/// followed the device rate instead would leave the peak at a quarter of the
/// offset it should be at, which on screen is a station that moves when the
/// operator changes a display setting.
#[test]
fn a_signal_keeps_its_frequency_across_a_decimation_change() {
    let mut h = start_engine(
        Box::new(ToneSource::new(RATE)),
        caps(RATE, false),
        EngineConfig { remember_session: false, ..Default::default() },
    );
    let thread = h.thread.take();

    // The frequency of the strongest bin in the latest frame, and the span it
    // was measured across. Both spans put the tone on a bin boundary, so the
    // reading is only ever as exact as a couple of display bins — which is the
    // point: two bins of a *decimated* frame is 94 Hz, and a span that had not
    // followed the decimation would be out by tens of kilohertz.
    let peak = |h: &mut EngineHandles, secs: f64| -> (f64, f64) {
        let deadline = Instant::now() + Duration::from_secs_f64(secs);
        let mut found = None;
        while Instant::now() < deadline {
            if h.spectrum_out.update() {
                let f = h.spectrum_out.output_buffer();
                if let Some((bin, _)) = f.bins.iter().enumerate().max_by_key(|(_, v)| **v) {
                    found = Some((f.freq_at_bin(bin), f.span_hz));
                }
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        found.expect("the engine publishes spectrum frames")
    };
    let on_the_tone = |hz: f64, span: f64| {
        let tol = span / 2048.0 * 2.0;
        assert!(
            (hz - (CENTER + TONE_OFFSET_HZ)).abs() < tol,
            "tone read as {hz} in a {span} Hz span, expected {} ± {tol:.0}",
            CENTER + TONE_OFFSET_HZ
        );
    };

    let (hz, span) = peak(&mut h, 1.5);
    assert!((span - RATE).abs() < 1.0, "undecimated span is {span}");
    on_the_tone(hz, span);

    h.cmd_tx.send(Command::SetDecimation(8)).unwrap();
    let (hz, span) = peak(&mut h, 1.5);
    assert!((span - RATE / 8.0).abs() < 1.0, "decimated span is {span}");
    on_the_tone(hz, span);

    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
}
