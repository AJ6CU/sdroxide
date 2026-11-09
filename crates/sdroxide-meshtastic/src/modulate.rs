// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM
//
// LoRa transmit chain: payload bytes to chirp symbols to baseband IQ. It is
// the exact inverse of what `demod` receives, so a frame made here decodes
// there. The bit stages (whitening, header, CRC, Hamming, interleaver, Gray)
// are checked against stage-by-stage reference vectors in the tests.

use std::f64::consts::PI;

use num_complex::Complex32;

use crate::lora::{
    self, gray_encode, hamming_encode, header_checksum, interleave, ldro, payload_crc, whiten,
};

/// Parameters of one transmitted LoRa frame.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TxParams {
    /// Spreading factor, 7..=12.
    pub sf: u8,
    /// Coding-rate denominator, 5..=8 (4/5..4/8).
    pub cr: u8,
    /// Bandwidth in hertz.
    pub bw_hz: u32,
    /// Preamble length in upchirps (Meshtastic uses 16).
    pub preamble: usize,
    /// Sync word as the SX127x register value (Meshtastic uses 0x2B).
    pub sync_word: u8,
    /// Append the payload CRC.
    pub has_crc: bool,
    /// Output samples per chirp sample (sample rate = `os * bw_hz`).
    pub os: usize,
}

impl TxParams {
    /// A Meshtastic frame: 16-symbol preamble, sync word 0x2B, CRC on.
    pub fn meshtastic(sf: u8, cr: u8, bw_hz: u32) -> Self {
        Self { sf, cr, bw_hz, preamble: 16, sync_word: 0x2B, has_crc: true, os: 1 }
    }

    /// Sample rate of the IQ this produces.
    pub fn sample_rate(&self) -> u32 {
        self.bw_hz * self.os as u32
    }
}

/// The nibble stream of a frame: five header nibbles, the whitened payload
/// (low nibble of each byte first) and the CRC nibbles.
pub fn frame_nibbles(payload: &[u8], cr: u8, has_crc: bool) -> Vec<u8> {
    let len = payload.len().min(255) as u8;
    let n0 = len >> 4;
    let n1 = len & 0x0f;
    let n2 = (((cr - 4) & 0x07) << 1) | has_crc as u8;
    let chk = header_checksum(&[n0, n1, n2]);
    let mut out = vec![n0, n1, n2, (chk >> 4) & 1, chk & 0x0f];

    let mut white = payload[..len as usize].to_vec();
    whiten(&mut white);
    for b in &white {
        out.push(b & 0x0f);
        out.push(b >> 4);
    }
    if has_crc {
        let crc = payload_crc(&payload[..len as usize]).unwrap_or(0);
        for b in [crc as u8, (crc >> 8) as u8] {
            out.push(b & 0x0f);
            out.push(b >> 4);
        }
    }
    out
}

/// The interleaver output before the Gray stage: the eight header symbols
/// (`sf - 2` bits each), then the payload symbols (`sf` bits, or `sf - 2`
/// with the low-data-rate optimisation).
pub fn interleaved_symbols(nibbles: &[u8], p: &TxParams) -> Vec<u16> {
    let sf = p.sf as usize;
    let sf_app_h = (sf - 2).max(5);
    let mut padded = nibbles.to_vec();
    if padded.len() < sf_app_h {
        padded.resize(sf_app_h, 0);
    }
    // Header block: always 4/8, sf - 2 rows.
    let cw: Vec<u8> = padded[..sf_app_h].iter().map(|&x| hamming_encode(x, 8)).collect();
    let mut out = interleave(&cw, sf_app_h, 8);

    // Payload blocks at the frame's coding rate, zero-padded to a full block.
    let sf_app = if ldro(p.sf, p.bw_hz) { sf - 2 } else { sf };
    for block in padded[sf_app_h..].chunks(sf_app) {
        let mut b = block.to_vec();
        b.resize(sf_app, 0);
        let cw: Vec<u8> = b.iter().map(|&x| hamming_encode(x, p.cr)).collect();
        out.extend(interleave(&cw, sf_app, p.cr as usize));
    }
    out
}

/// Chirp symbol values for a payload: the header block first, then the
/// payload blocks, already Gray-mapped for the modulator.
pub fn encode_symbols(payload: &[u8], p: &TxParams) -> Vec<u16> {
    let n = 1u32 << p.sf;
    let reduced = ldro(p.sf, p.bw_hz);
    let nibbles = frame_nibbles(payload, p.cr, p.has_crc);
    interleaved_symbols(&nibbles, p)
        .into_iter()
        .enumerate()
        .map(|(i, v)| {
            let g = gray_encode(v);
            if i < 8 || reduced { 4 * g + 1 } else { ((g as u32 + 1) % n) as u16 }
        })
        .collect()
}

/// One chirp of `n_total = (1 << sf) * os` samples. `id` shifts the start
/// frequency (a data symbol); `down` conjugates it.
fn chirp(sf: u8, os: usize, id: u32, down: bool, out: &mut Vec<Complex32>) {
    chirp_part(sf, os, id, down, 1 << sf, out);
}

/// The first `len_chirp` chirp-samples of a chirp (a full chirp is `1 << sf`).
fn chirp_part(sf: u8, os: usize, id: u32, down: bool, len_chirp: u32, out: &mut Vec<Complex32>) {
    let n = (1u32 << sf) as f64;
    let id = id as f64;
    let total = len_chirp as usize * os;
    for k in 0..total {
        let t = k as f64 / os as f64;
        let phase = if t < n - id {
            2.0 * PI * (t * t / (2.0 * n) + (id / n - 0.5) * t)
        } else {
            2.0 * PI * (t * t / (2.0 * n) + (id / n - 1.5) * t)
        };
        let (s, c) = phase.sin_cos();
        let v = Complex32::new(c as f32, if down { -(s as f32) } else { s as f32 });
        out.push(v);
    }
}

/// Baseband IQ of one frame at `p.sample_rate()`: preamble, sync word,
/// 2.25 down-chirps, then the data symbols. No leading or trailing silence.
pub fn modulate(payload: &[u8], p: &TxParams) -> Vec<Complex32> {
    let symbols = encode_symbols(payload, p);
    let spsym = (1usize << p.sf) * p.os;
    let mut iq = Vec::with_capacity((p.preamble + 5 + symbols.len()) * spsym);
    for _ in 0..p.preamble {
        chirp(p.sf, p.os, 0, false, &mut iq);
    }
    chirp(p.sf, p.os, ((p.sync_word >> 4) as u32) << 3, false, &mut iq);
    chirp(p.sf, p.os, ((p.sync_word & 0x0f) as u32) << 3, false, &mut iq);
    chirp(p.sf, p.os, 0, true, &mut iq);
    chirp(p.sf, p.os, 0, true, &mut iq);
    chirp_part(p.sf, p.os, 0, true, (1u32 << p.sf) / 4, &mut iq);
    for &s in &symbols {
        chirp(p.sf, p.os, s as u32, false, &mut iq);
    }
    iq
}

/// How many samples a frame of `payload_len` bytes lasts.
pub fn frame_samples(payload_len: usize, p: &TxParams) -> usize {
    let reduced = ldro(p.sf, p.bw_hz);
    let data = 8 + lora::payload_symbol_count(payload_len, p.sf, p.cr, p.has_crc, reduced);
    let spsym = (1usize << p.sf) * p.os;
    // preamble + 2 sync + 2 down-chirps, a quarter down-chirp, then the data.
    (p.preamble + 4 + data) * spsym + spsym / 4
}
