// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM
//
// The transmit chain against stage-by-stage reference vectors: the frame
// "ABC" at SF11, CR 4/5, 250 kHz, captured after every stage of the
// reference modulator (whitening, header, CRC, Hamming, interleaver, Gray).
// A mistake in any stage changes the bytes here.

use sdroxide_meshtastic::lora::{hamming_encode, whiten};
use sdroxide_meshtastic::modulate::{TxParams, encode_symbols, frame_nibbles, interleaved_symbols};

const INPUT: &[u8] = include_bytes!("../testdata/lora_stages/00_input.bin");
const WHITENING: &[u8] = include_bytes!("../testdata/lora_stages/01_whitening.bin");
const HEADER: &[u8] = include_bytes!("../testdata/lora_stages/02_header.bin");
const ADD_CRC: &[u8] = include_bytes!("../testdata/lora_stages/03_add_crc.bin");
const HAMMING: &[u8] = include_bytes!("../testdata/lora_stages/04_hamming.bin");
const INTERLEAVER: &[u8] = include_bytes!("../testdata/lora_stages/05_interleaver.bin");
const GRAY: &[u8] = include_bytes!("../testdata/lora_stages/06_gray.bin");

fn params() -> TxParams {
    TxParams::meshtastic(11, 5, 250_000)
}

fn u32s(b: &[u8]) -> Vec<u32> {
    b.chunks(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect()
}

#[test]
fn the_input_is_abc() {
    assert_eq!(INPUT, b"ABC");
}

#[test]
fn whitening_gives_the_reference_nibbles() {
    let mut w = INPUT.to_vec();
    whiten(&mut w);
    let nibbles: Vec<u8> = w.iter().flat_map(|b| [b & 0x0f, b >> 4]).collect();
    assert_eq!(nibbles, WHITENING);
}

#[test]
fn the_header_and_crc_stages_match() {
    let nibbles = frame_nibbles(INPUT, 5, true);
    // Header stage: five header nibbles then the whitened payload.
    assert_eq!(&nibbles[..HEADER.len()], HEADER);
    // CRC stage: the whole stream.
    assert_eq!(nibbles, ADD_CRC);
}

#[test]
fn the_hamming_stage_matches() {
    let nibbles = frame_nibbles(INPUT, 5, true);
    // The header block (sf - 2 = 9 nibbles) is 4/8, the rest 4/5.
    let cw: Vec<u8> = nibbles
        .iter()
        .enumerate()
        .map(|(i, &n)| hamming_encode(n, if i < 9 { 8 } else { 5 }))
        .collect();
    assert_eq!(cw, HAMMING);
}

#[test]
fn the_interleaver_stage_matches() {
    let nibbles = frame_nibbles(INPUT, 5, true);
    let got = interleaved_symbols(&nibbles, &params());
    let want = u32s(INTERLEAVER);
    assert_eq!(got.len(), want.len());
    for (i, (&g, &w)) in got.iter().zip(&want).enumerate() {
        // The reference keeps the header symbols shifted up two bits.
        let w = if i < 8 { w >> 2 } else { w };
        assert_eq!(g as u32, w, "symbol {i}");
    }
}

#[test]
fn the_chirp_symbols_match() {
    let got: Vec<u32> = encode_symbols(INPUT, &params()).iter().map(|&s| s as u32).collect();
    assert_eq!(got, u32s(GRAY));
}
