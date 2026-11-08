// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM
//
// LoRa CSS demodulator. The DSP path is:
//   accumulate N = 2^SF samples per symbol
//     -> dechirp (multiply by the reference down-chirp)
//     -> N-point FFT
//     -> argmax bin = symbol value
//   IDLE        : look for a repeated bin (the preamble)
//   PREAMBLE_OK : count further matching up-chirps; the first different one is
//                 the sync word
//   HEADER      : two down-chirps (carrier offset), then 8 symbols
//                 -> Hamming(8,4) -> length, CR, CRC flag, checksum
//   PAYLOAD     : cw_per_block symbols -> deinterleave -> Hamming -> bytes
//   DELIVER     : dewhiten, CRC check, frame out
//
// Significant portions of the bit-level decode (hard-decode Hamming,
// deinterleave, gray, dewhiten, preamble-mode vote) are ported from gr-lora_sdr
// by Joachim Tapparel @ EPFL TCL Lab (GPL-3.0-or-later). CSS demodulation, the
// frame state machine and the FFT / CFO handling are original to the
// CEMAXECUTER LLC code this is written from.

use std::f64::consts::PI;
use std::sync::Arc;

use num_complex::{Complex32, Complex64};
use rustfft::{Fft, FftPlanner};

use crate::lora::{
    self, deinterleave, gray_decode, hamming_decode, header_checksum, payload_crc, whiten,
};

mod state;

/// Preamble symbols that must match before the bin is trusted.
const PREAMBLE_MIN: usize = 5;
/// Longest payload the decoder will assemble.
const MAX_PAYLOAD_BYTES: usize = 256;
const MAX_PAYLOAD_SYMBOLS: usize = 1024;
/// Preamble symbols kept for the fractional timing estimate.
const PREAMBLE_DECHIRPED_K: usize = 4;

/// What a decoder is tuned to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DecoderConfig {
    /// Spreading factor, 7..=12.
    pub sf: u8,
    /// Coding rate the channel is expected to use, denominator 5..=8. The
    /// frame header carries the real value, so this is informational.
    pub cr: u8,
    /// Chirp bandwidth in hertz.
    pub bw_hz: u32,
    /// Input samples per chirp sample (input rate = `os_factor * bw_hz`).
    /// 1 is the cheapest; 2 or more recovers the fractional symbol timing.
    pub os_factor: usize,
    /// RF centre frequency, used to turn the measured carrier offset into a
    /// sample-clock drift estimate. 0 leaves the drift correction off.
    pub center_freq_hz: f64,
}

impl DecoderConfig {
    /// A decoder at one sample per chirp sample and no drift correction.
    pub fn new(sf: u8, cr: u8, bw_hz: u32) -> Self {
        Self { sf, cr, bw_hz, os_factor: 1, center_freq_hz: 0.0 }
    }
}

/// One frame that came out of the decoder.
#[derive(Debug, Clone, PartialEq)]
pub struct LoraFrame {
    /// The payload bytes (the whole LoRa payload, without the CRC field).
    pub payload: Vec<u8>,
    pub sf: u8,
    /// Coding rate of the payload as the frame's header declared it (5..=8).
    pub cr: u8,
    pub bw_hz: u32,
    /// The header said a CRC follows the payload.
    pub has_crc: bool,
    /// The CRC matched (or there is none to check). A payload under two
    /// bytes cannot be checked and reads as not matching.
    pub crc_ok: bool,
    /// Mean peak-to-noise ratio over the header and payload symbols, in dB.
    pub snr_db: f32,
    /// Carrier offset measured on the preamble, in FFT bins.
    pub cfo_bins: f32,
}

/// Counters of how far candidates got, for the interface.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct DecoderStats {
    pub preamble_candidates: u64,
    pub preamble_locks: u64,
    pub header_pass: u64,
    pub header_fail: u64,
    pub crc_pass: u64,
    pub crc_fail: u64,
    pub frames: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum State {
    Idle,
    PreambleOk,
    Header,
    Payload,
}

/// A LoRa demodulator for one (SF, BW) channel.
pub struct LoraDecoder {
    cfg: DecoderConfig,
    n: usize,
    spsym: usize,
    sto_offset: usize,

    upchirp: Vec<Complex32>,
    downchirp: Vec<Complex32>,

    fft: Arc<dyn Fft<f32>>,
    fft2: Arc<dyn Fft<f32>>,
    /// Dechirped input of the last demodulated symbol.
    dechirped: Vec<Complex32>,
    spectrum: Vec<Complex32>,
    fft_scratch: Vec<Complex32>,
    fft2_scratch: Vec<Complex32>,
    scratch_sym: Vec<Complex32>,

    symbuf: Vec<Complex32>,

    state: State,
    preamble_count: usize,
    preamble_peak_sum: f64,
    preamble_peak_count: usize,
    preamble_locked_once: bool,
    preamble_bin: usize,
    preamble_bin_hist: Vec<usize>,
    preamble_fft_hist: Vec<Complex32>,
    sto_skip_remaining: usize,
    sfo_next_sym_shift: i32,
    header_syms: [u16; 8],
    header_idx: usize,

    cfo_int: i64,
    cfo_frac: f32,

    payload_len: usize,
    payload_cr: u8,
    payload_has_crc: bool,
    payload_ldro: bool,
    hdr_leftover: Vec<u8>,
    payload_syms: Vec<u16>,
    payload_sym_target: usize,

    sfo_cum: f64,
    sfo_hat: f64,
    sto_frac: f32,
    preamble_dechirped: Vec<Vec<Complex32>>,
    preamble_dechirped_next: usize,
    preamble_dechirped_count: usize,

    snr_db_sum: f64,
    snr_db_count: usize,

    stats: DecoderStats,
    out: Vec<LoraFrame>,
}

/// Reference up-chirp `exp(j 2 pi (n^2 / 2N - n / 2))` and its conjugate.
fn build_chirps(n: usize) -> (Vec<Complex32>, Vec<Complex32>) {
    let mut up = Vec::with_capacity(n);
    let mut down = Vec::with_capacity(n);
    for k in 0..n {
        let kf = k as f64;
        let phase = 2.0 * PI * (kf * kf / (2.0 * n as f64) - kf * 0.5);
        let (s, c) = phase.sin_cos();
        let u = Complex32::new(c as f32, s as f32);
        up.push(u);
        down.push(u.conj());
    }
    (up, down)
}

impl LoraDecoder {
    /// Build a decoder. `None` for a parameter outside what LoRa allows.
    pub fn new(cfg: DecoderConfig) -> Option<Self> {
        if !(7..=12).contains(&cfg.sf)
            || !(5..=8).contains(&cfg.cr)
            || cfg.bw_hz == 0
            || !(1..=16).contains(&cfg.os_factor)
        {
            return None;
        }
        let n = 1usize << cfg.sf;
        let spsym = n * cfg.os_factor;
        let mut planner = FftPlanner::<f32>::new();
        let fft = planner.plan_fft_forward(n);
        let fft2 = planner.plan_fft_forward(2 * n);
        let (upchirp, downchirp) = build_chirps(n);
        Some(Self {
            cfg,
            n,
            spsym,
            sto_offset: 0,
            upchirp,
            downchirp,
            fft_scratch: vec![Complex32::new(0.0, 0.0); fft.get_inplace_scratch_len()],
            fft2_scratch: vec![Complex32::new(0.0, 0.0); fft2.get_inplace_scratch_len()],
            fft,
            fft2,
            dechirped: vec![Complex32::new(0.0, 0.0); n],
            spectrum: vec![Complex32::new(0.0, 0.0); n],
            scratch_sym: vec![Complex32::new(0.0, 0.0); n],
            symbuf: Vec::with_capacity(spsym),
            state: State::Idle,
            preamble_count: 0,
            preamble_peak_sum: 0.0,
            preamble_peak_count: 0,
            preamble_locked_once: false,
            preamble_bin: 0,
            preamble_bin_hist: Vec::new(),
            preamble_fft_hist: Vec::new(),
            sto_skip_remaining: 0,
            sfo_next_sym_shift: 0,
            header_syms: [0; 8],
            header_idx: 0,
            cfo_int: 0,
            cfo_frac: 0.0,
            payload_len: 0,
            payload_cr: 5,
            payload_has_crc: false,
            payload_ldro: false,
            hdr_leftover: Vec::new(),
            payload_syms: Vec::new(),
            payload_sym_target: 0,
            sfo_cum: 0.0,
            sfo_hat: 0.0,
            sto_frac: 0.0,
            preamble_dechirped: vec![vec![Complex32::new(0.0, 0.0); n]; PREAMBLE_DECHIRPED_K],
            preamble_dechirped_next: 0,
            preamble_dechirped_count: 0,
            snr_db_sum: 0.0,
            snr_db_count: 0,
            stats: DecoderStats::default(),
            out: Vec::new(),
        })
    }

    /// The configuration this decoder was built with.
    pub fn config(&self) -> &DecoderConfig {
        &self.cfg
    }

    /// How far candidates have got so far.
    pub fn stats(&self) -> DecoderStats {
        self.stats
    }

    /// Feed input samples (at `os_factor * bw_hz`). Returns the frames that
    /// completed inside this block, in order.
    pub fn feed(&mut self, samples: &[Complex32]) -> Vec<LoraFrame> {
        for &s in samples {
            if self.sto_skip_remaining > 0 {
                self.sto_skip_remaining -= 1;
                continue;
            }
            self.symbuf.push(s);
            if self.symbuf.len() == self.spsym {
                self.state_tick();
                if self.sfo_next_sym_shift > 0 {
                    // Next window one sample earlier: reuse the last sample.
                    let last = self.symbuf[self.spsym - 1];
                    self.symbuf.clear();
                    self.symbuf.push(last);
                } else if self.sfo_next_sym_shift < 0 {
                    self.sto_skip_remaining += 1;
                    self.symbuf.clear();
                } else {
                    self.symbuf.clear();
                }
                self.sfo_next_sym_shift = 0;
            }
        }
        std::mem::take(&mut self.out)
    }

    /// Drop back to hunting for a preamble and forget everything per-frame.
    fn reset_to_idle(&mut self) {
        self.state = State::Idle;
        self.preamble_count = 0;
        self.preamble_locked_once = false;
        self.preamble_fft_hist.clear();
        self.preamble_bin_hist.clear();
        self.preamble_peak_sum = 0.0;
        self.preamble_peak_count = 0;
        self.header_idx = 0;
        self.sto_skip_remaining = 0;
        self.hdr_leftover.clear();
        self.payload_syms.clear();
        self.payload_sym_target = 0;
        self.payload_ldro = false;
        self.cfo_int = 0;
        self.cfo_frac = 0.0;
        self.snr_db_sum = 0.0;
        self.snr_db_count = 0;
        self.sfo_cum = 0.0;
        self.sfo_hat = 0.0;
        self.sto_frac = 0.0;
        self.sfo_next_sym_shift = 0;
        self.preamble_dechirped_next = 0;
        for v in &mut self.preamble_dechirped {
            v.iter_mut().for_each(|c| *c = Complex32::new(0.0, 0.0));
        }
        self.preamble_dechirped_count = 0;
    }
}
