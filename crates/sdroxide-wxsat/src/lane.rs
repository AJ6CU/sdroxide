//! The weather-satellite lane the engine runs: a downconverter on the channel,
//! a worker thread, and a status with the picture lines new since the last one.
//!
//! The realtime engine thread hands I/Q blocks to the worker over a bounded
//! channel (a dropped block costs a fraction of a line) and drains statuses
//! non-blocking, like `sdroxide_sonde::SondeController`.

use std::thread::JoinHandle;

use crossbeam_channel::{Receiver, Sender, bounded, select, unbounded};
use sdroxide_dsp::{ComplexResampler, Ddc, Complex32};
use sdroxide_types::{WXSAT_MAX_NEW_LINES, WxsatSettings, WxsatStatus};

use crate::WXSAT_LANE_RATE_HZ;
use crate::apt::{AptDecoder, AptDemod, AptLine};

/// Bounded I/Q queue depth: a couple of seconds of 48 kS/s lane at a typical
/// device read.
const IQ_QUEUE_DEPTH: usize = 256;

/// A status goes out at least once every N blocks even when no line was
/// decoded, so the level meter keeps moving.
const IDLE_TICK: u64 = 250;

enum Ctl {
    Enabled(bool),
    Stop,
}

/// The decoder worker, fed lane-rate I/Q.
pub struct WxsatController {
    iq_tx: Sender<Vec<Complex32>>,
    ctl_tx: Sender<Ctl>,
    res_rx: Receiver<WxsatStatus>,
    worker: Option<JoinHandle<()>>,
}

impl WxsatController {
    /// `pass` is the number the statuses carry, so the client can tell a new
    /// picture from the continuation of the old one.
    pub fn new(pass: u32) -> Self {
        let (iq_tx, iq_rx) = bounded::<Vec<Complex32>>(IQ_QUEUE_DEPTH);
        let (ctl_tx, ctl_rx) = unbounded::<Ctl>();
        let (res_tx, res_rx) = unbounded::<WxsatStatus>();
        let worker = std::thread::Builder::new()
            .name("sdroxide-wxsat".into())
            .spawn(move || {
                let mut demod = AptDemod::new(WXSAT_LANE_RATE_HZ);
                let mut dec = AptDecoder::new();
                let (mut px, mut lines): (Vec<f32>, Vec<AptLine>) = (Vec::new(), Vec::new());
                let mut enabled = true;
                let mut total: u32 = 0;
                let mut tick = 0u64;
                let mut quality = 0.0f32;
                loop {
                    select! {
                        recv(ctl_rx) -> msg => match msg {
                            Ok(Ctl::Enabled(on)) => enabled = on,
                            Ok(Ctl::Stop) | Err(_) => break,
                        },
                        recv(iq_rx) -> msg => match msg {
                            Ok(iq) => {
                                tick += 1;
                                px.clear();
                                lines.clear();
                                if enabled {
                                    demod.process(&iq, &mut px);
                                    dec.process(&px, &mut lines);
                                }
                                if !lines.is_empty() || tick % IDLE_TICK == 0 || tick == 1 {
                                    // A client that fell far behind loses the oldest.
                                    let skip = lines.len().saturating_sub(WXSAT_MAX_NEW_LINES);
                                    let first = total + skip as u32;
                                    if let Some(l) = lines.last() {
                                        quality = l.quality;
                                    }
                                    let new_lines: Vec<Vec<u8>> =
                                        lines.drain(..).skip(skip).map(|l| l.pixels).collect();
                                    total = first + new_lines.len() as u32;
                                    let _ = res_tx.send(WxsatStatus {
                                        running: enabled,
                                        level_dbfs: demod.level_dbfs(),
                                        locked: dec.locked(),
                                        quality,
                                        pass,
                                        total_lines: total,
                                        first_line: first,
                                        new_lines,
                                    });
                                }
                            }
                            Err(_) => break,
                        },
                    }
                }
            })
            .expect("spawn wxsat worker");
        WxsatController { iq_tx, ctl_tx, res_rx, worker: Some(worker) }
    }

    /// Realtime path: hand a block of lane-rate I/Q to the worker. Non-blocking.
    pub fn on_rx_iq(&self, iq: &[Complex32]) {
        let _ = self.iq_tx.try_send(iq.to_vec());
    }

    pub fn set_enabled(&self, on: bool) {
        let _ = self.ctl_tx.send(Ctl::Enabled(on));
    }

    /// Drain what the worker has said, oldest first. Every status is kept: each
    /// carries lines nobody else has, so they cannot be collapsed to the newest.
    pub fn poll(&self) -> Vec<WxsatStatus> {
        let mut out = Vec::new();
        while let Ok(s) = self.res_rx.try_recv() {
            out.push(s);
        }
        out
    }
}

impl Drop for WxsatController {
    fn drop(&mut self) {
        let _ = self.ctl_tx.send(Ctl::Stop);
        if let Some(h) = self.worker.take() {
            let _ = h.join();
        }
    }
}

/// The channel mixed out of the wide window, decoded.
pub struct WxsatLane {
    in_rate: f64,
    ddc: Ddc,
    rs: Option<ComplexResampler>,
    buf: Vec<Complex32>,
    res_buf: Vec<Complex32>,
    freq_hz: f64,
    ctl: WxsatController,
    pass: u32,
}

impl WxsatLane {
    pub fn new(in_rate: f64, center_hz: f64, cfg: WxsatSettings, pass: u32) -> Self {
        let mut ddc = Ddc::new(in_rate, WXSAT_LANE_RATE_HZ);
        ddc.set_offset_hz(cfg.frequency_hz - center_hz);
        let rs = ComplexResampler::new(ddc.out_rate(), WXSAT_LANE_RATE_HZ);
        Self {
            in_rate,
            ddc,
            rs,
            buf: Vec::new(),
            res_buf: Vec::new(),
            freq_hz: cfg.frequency_hz,
            ctl: WxsatController::new(pass),
            pass,
        }
    }

    /// The sample rate this lane was built for; a different one needs a new lane.
    pub fn in_rate(&self) -> f64 {
        self.in_rate
    }

    pub fn pass(&self) -> u32 {
        self.pass
    }

    pub fn frequency_hz(&self) -> f64 {
        self.freq_hz
    }

    /// Re-seat the mixer after a retune of the window or of the channel.
    pub fn set_tuning(&mut self, center_hz: f64, freq_hz: f64) {
        self.freq_hz = freq_hz;
        self.ddc.set_offset_hz(freq_hz - center_hz);
    }

    /// Hand the engine's wide I/Q to the lane.
    pub fn on_wide_iq(&mut self, iq: &[Complex32]) {
        self.buf.clear();
        self.ddc.process(iq, &mut self.buf);
        let lane = match self.rs.as_mut() {
            Some(rs) => {
                self.res_buf.clear();
                rs.push(&self.buf, &mut self.res_buf);
                &self.res_buf
            }
            None => &self.buf,
        };
        self.ctl.on_rx_iq(lane);
    }

    pub fn poll(&self) -> Vec<WxsatStatus> {
        self.ctl.poll()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::apt::{IMAGE_A_START, IMAGE_WIDTH, synth};

    fn wait_lines(c: &WxsatController, want: u32) -> Vec<WxsatStatus> {
        let mut all = Vec::new();
        for _ in 0..500 {
            all.extend(c.poll());
            if all.last().is_some_and(|s| s.total_lines >= want) {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        all
    }

    #[test]
    fn the_worker_reports_the_lines_once_each_with_their_numbers() {
        let a: Vec<u8> = (0..IMAGE_WIDTH).map(|i| (i * 255 / (IMAGE_WIDTH - 1)) as u8).collect();
        let lines: Vec<Vec<u8>> = (0..12).map(|_| synth::line(&a, &a)).collect();
        let iq = synth::modulate(&lines, WXSAT_LANE_RATE_HZ, 1.0);
        let c = WxsatController::new(5);
        for chunk in iq.chunks(2048) {
            c.on_rx_iq(chunk);
        }
        let all = wait_lines(&c, 9);
        let total = all.last().expect("a status").total_lines;
        assert!(total >= 9, "{total} lines");
        // Every line once, numbered in order.
        let mut next = 0u32;
        for s in &all {
            assert_eq!(s.pass, 5);
            assert_eq!(s.first_line, next);
            next += s.new_lines.len() as u32;
        }
        assert_eq!(next, total);
        let first = all.iter().find_map(|s| s.new_lines.first()).unwrap();
        assert_eq!(first.len(), sdroxide_types::WXSAT_LINE_PIXELS);
        let mid = first[IMAGE_A_START + IMAGE_WIDTH / 2];
        assert!((i32::from(mid) - 127).abs() < 30, "mid grey {mid}");
    }

    #[test]
    fn a_lane_on_a_wide_window_finds_a_channel_off_centre() {
        let a: Vec<u8> = vec![128; IMAGE_WIDTH];
        let lines: Vec<Vec<u8>> = (0..14).map(|_| synth::line(&a, &a)).collect();
        let narrow = synth::modulate(&lines, WXSAT_LANE_RATE_HZ, 1.0);
        // Put the signal 100 kHz off the centre of a 480 kS/s window: upsample
        // by 10 (hold) and mix up.
        let wide_rate = 480_000.0;
        let mut wide = Vec::with_capacity(narrow.len() * 10);
        let mut ph = 0.0f64;
        let w = std::f64::consts::TAU * 100_000.0 / wide_rate;
        for x in &narrow {
            for _ in 0..10 {
                wide.push(*x * Complex32::new(ph.cos() as f32, ph.sin() as f32));
                ph += w;
            }
        }
        let cfg = WxsatSettings { enabled: true, frequency_hz: 137_100_000.0 };
        let mut lane = WxsatLane::new(wide_rate, 137_000_000.0, cfg, 1);
        for chunk in wide.chunks(16_384) {
            lane.on_wide_iq(chunk);
        }
        let mut total = 0;
        for _ in 0..500 {
            if let Some(s) = lane.poll().last() {
                total = total.max(s.total_lines);
            }
            if total >= 6 {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        assert!(total >= 6, "{total} lines through the downconverter");
    }
}
