// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM
//
// Bit-level LoRa coding: whitening, Gray mapping, Hamming, diagonal
// interleaver, CRC-16 and the explicit header checksum. Both directions are
// here: the receive side feeds `demod`, the transmit side feeds `modulate`.
//
// Significant portions of the bit-level decode (Hamming, deinterleave, gray,
// dewhiten) are original work Copyright 2022 Tapparel Joachim @EPFL,TCL,
// modifications Copyright 2026 CEMAXECUTER LLC. SPDX-License-Identifier:
// GPL-3.0-or-later.

/// The LoRa whitening sequence (256 bytes).
pub const WHITEN: [u8; 256] = [
    0xff, 0xfe, 0xfc, 0xf8, 0xf0, 0xe1, 0xc2, 0x85, 0x0b, 0x17, 0x2f, 0x5e, 0xbc, 0x78, 0xf1, 0xe3,
    0xc6, 0x8d, 0x1a, 0x34, 0x68, 0xd0, 0xa0, 0x40, 0x80, 0x01, 0x02, 0x04, 0x08, 0x11, 0x23, 0x47,
    0x8e, 0x1c, 0x38, 0x71, 0xe2, 0xc4, 0x89, 0x12, 0x25, 0x4b, 0x97, 0x2e, 0x5c, 0xb8, 0x70, 0xe0,
    0xc0, 0x81, 0x03, 0x06, 0x0c, 0x19, 0x32, 0x64, 0xc9, 0x92, 0x24, 0x49, 0x93, 0x26, 0x4d, 0x9b,
    0x37, 0x6e, 0xdc, 0xb9, 0x72, 0xe4, 0xc8, 0x90, 0x20, 0x41, 0x82, 0x05, 0x0a, 0x15, 0x2b, 0x56,
    0xad, 0x5b, 0xb6, 0x6d, 0xda, 0xb5, 0x6b, 0xd6, 0xac, 0x59, 0xb2, 0x65, 0xcb, 0x96, 0x2c, 0x58,
    0xb0, 0x61, 0xc3, 0x87, 0x0f, 0x1f, 0x3e, 0x7d, 0xfb, 0xf6, 0xed, 0xdb, 0xb7, 0x6f, 0xde, 0xbd,
    0x7a, 0xf5, 0xeb, 0xd7, 0xae, 0x5d, 0xba, 0x74, 0xe8, 0xd1, 0xa2, 0x44, 0x88, 0x10, 0x21, 0x43,
    0x86, 0x0d, 0x1b, 0x36, 0x6c, 0xd8, 0xb1, 0x63, 0xc7, 0x8f, 0x1e, 0x3c, 0x79, 0xf3, 0xe7, 0xce,
    0x9c, 0x39, 0x73, 0xe6, 0xcc, 0x98, 0x31, 0x62, 0xc5, 0x8b, 0x16, 0x2d, 0x5a, 0xb4, 0x69, 0xd2,
    0xa4, 0x48, 0x91, 0x22, 0x45, 0x8a, 0x14, 0x29, 0x52, 0xa5, 0x4a, 0x95, 0x2a, 0x54, 0xa9, 0x53,
    0xa7, 0x4e, 0x9d, 0x3b, 0x77, 0xee, 0xdd, 0xbb, 0x76, 0xec, 0xd9, 0xb3, 0x67, 0xcf, 0x9e, 0x3d,
    0x7b, 0xf7, 0xef, 0xdf, 0xbf, 0x7e, 0xfd, 0xfa, 0xf4, 0xe9, 0xd3, 0xa6, 0x4c, 0x99, 0x33, 0x66,
    0xcd, 0x9a, 0x35, 0x6a, 0xd4, 0xa8, 0x51, 0xa3, 0x46, 0x8c, 0x18, 0x30, 0x60, 0xc1, 0x83, 0x07,
    0x0e, 0x1d, 0x3a, 0x75, 0xea, 0xd5, 0xaa, 0x55, 0xab, 0x57, 0xaf, 0x5f, 0xbe, 0x7c, 0xf9, 0xf2,
    0xe5, 0xca, 0x94, 0x28, 0x50, 0xa1, 0x42, 0x84, 0x09, 0x13, 0x27, 0x4f, 0x9f, 0x3f, 0x7f, 0x00,
];

/// XOR `data` with the whitening sequence, starting at its first byte.
/// Whitening is its own inverse. At most 256 bytes are touched.
pub fn whiten(data: &mut [u8]) {
    for (b, w) in data.iter_mut().zip(WHITEN.iter()) {
        *b ^= *w;
    }
}

/// Gray decode: `s ^ (s >> 1)`.
pub fn gray_decode(s: u16) -> u16 {
    s ^ (s >> 1)
}

/// Inverse of [`gray_decode`] (prefix XOR of the bits).
pub fn gray_encode(s: u16) -> u16 {
    let mut b = s;
    let mut s = s >> 1;
    while s != 0 {
        b ^= s;
        s >>= 1;
    }
    b
}

/// Hard Hamming decode of one codeword. `cr` is the coding-rate denominator
/// (5..=8 for 4/5..4/8). The codeword is `[d0 d1 d2 d3 | p0 p1 p2 p3]`
/// MSB-first and the returned nibble has `d0` as its least significant bit.
/// 4/7 and 4/8 correct one bit; 4/5 and 4/6 only detect, so they return the
/// data bits as received.
pub fn hamming_decode(cw: u8, cr: u8) -> u8 {
    let cr_app = cr.saturating_sub(4).clamp(1, 4);
    let cw_len = (cr_app + 4) as usize;
    let mut b = [0u8; 8];
    for (i, bit) in b.iter_mut().enumerate().take(cw_len) {
        *bit = (cw >> (cw_len - 1 - i)) & 1;
    }
    let (mut d0, mut d1, mut d2, mut d3) = (b[0], b[1], b[2], b[3]);
    let correct = match cr_app {
        // 4/8: only correct when the overall parity is odd.
        4 => b.iter().take(cw_len).map(|&x| x as u32).sum::<u32>() % 2 == 1,
        3 => true,
        _ => false,
    };
    if correct {
        let s0 = b[0] ^ b[1] ^ b[2] ^ b[4];
        let s1 = b[1] ^ b[2] ^ b[3] ^ b[5];
        let s2 = b[0] ^ b[1] ^ b[3] ^ b[6];
        // A single error in data bit b_i trips these checks: b0 -> 5, b1 -> 7,
        // b2 -> 3, b3 -> 6 (a parity bit errs alone and the nibble is fine).
        match s0 | (s1 << 1) | (s2 << 2) {
            5 => d0 ^= 1,
            7 => d1 ^= 1,
            3 => d2 ^= 1,
            6 => d3 ^= 1,
            _ => {}
        }
    }
    (d3 << 3) | (d2 << 2) | (d1 << 1) | d0
}

/// Hamming-encode one nibble at coding rate `cr` (5..=8). Inverse of
/// [`hamming_decode`]; the codeword is returned right-aligned in `cr` bits.
pub fn hamming_encode(nibble: u8, cr: u8) -> u8 {
    let cr_app = cr.saturating_sub(4).clamp(1, 4);
    let n = nibble & 0x0f;
    let d0 = n & 1;
    let d1 = (n >> 1) & 1;
    let d2 = (n >> 2) & 1;
    let d3 = (n >> 3) & 1;
    let p0 = d0 ^ d1 ^ d2;
    let p1 = d1 ^ d2 ^ d3;
    let p2 = d0 ^ d1 ^ d3;
    let p3 = d0 ^ d2 ^ d3;
    let data = (d0 << 3) | (d1 << 2) | (d2 << 1) | d3;
    match cr_app {
        1 => (data << 1) | (d0 ^ d1 ^ d2 ^ d3),
        2 => (data << 2) | (p0 << 1) | p1,
        3 => (data << 3) | (p0 << 2) | (p1 << 1) | p2,
        _ => (data << 4) | (p0 << 3) | (p1 << 2) | (p2 << 1) | p3,
    }
}

/// CRC-16/CCITT (poly 0x1021, init 0).
pub fn crc16(data: &[u8]) -> u16 {
    let mut crc = 0u16;
    for &b in data {
        crc ^= (b as u16) << 8;
        for _ in 0..8 {
            crc = if crc & 0x8000 != 0 { (crc << 1) ^ 0x1021 } else { crc << 1 };
        }
    }
    crc
}

/// The LoRa payload CRC: CRC-16 over all but the last two payload bytes,
/// XORed with those two bytes. `payload` is the plain (de-whitened) payload
/// without the CRC field. `None` when it is shorter than two bytes.
pub fn payload_crc(payload: &[u8]) -> Option<u16> {
    let n = payload.len();
    if n < 2 {
        return None;
    }
    Some(crc16(&payload[..n - 2]) ^ payload[n - 1] as u16 ^ ((payload[n - 2] as u16) << 8))
}

/// Diagonal deinterleave: `cr_use` symbols of `sf_app` bits become `sf_app`
/// codewords of `cr_use` bits. `deinter[(i - j - 1) mod sf_app][i] = inter[i][j]`.
pub fn deinterleave(symbols: &[u16], sf_app: usize, cr_use: usize) -> Vec<u8> {
    let mut out = vec![0u8; sf_app];
    if cr_use == 0 || sf_app == 0 {
        return out;
    }
    let mut deinter = vec![vec![0u8; cr_use]; sf_app];
    for i in 0..cr_use {
        let v = symbols.get(i).copied().unwrap_or(0);
        for j in 0..sf_app {
            let bit = ((v >> (sf_app - 1 - j)) & 1) as u8;
            let row = (i as isize - j as isize - 1).rem_euclid(sf_app as isize) as usize;
            if let Some(cell) = deinter.get_mut(row).and_then(|r| r.get_mut(i)) {
                *cell = bit;
            }
        }
    }
    for (r, cw) in out.iter_mut().enumerate() {
        *cw = deinter[r].iter().fold(0u8, |a, &b| (a << 1) | b);
    }
    out
}

/// Inverse of [`deinterleave`]: `sf_app` codewords of `cr_use` bits become
/// `cr_use` symbols of `sf_app` bits.
pub fn interleave(codewords: &[u8], sf_app: usize, cr_use: usize) -> Vec<u16> {
    let mut out = vec![0u16; cr_use];
    for (i, sym) in out.iter_mut().enumerate() {
        let mut v = 0u16;
        for j in 0..sf_app {
            let row = (i as isize - j as isize - 1).rem_euclid(sf_app as isize) as usize;
            let cw = codewords.get(row).copied().unwrap_or(0);
            let bit = (cw >> (cr_use - 1 - i)) & 1;
            v = (v << 1) | bit as u16;
        }
        *sym = v;
    }
    out
}

/// The five-bit checksum of the explicit header over its first three nibbles.
pub fn header_checksum(n: &[u8; 3]) -> u8 {
    let b = |v: u8, k: u8| (v >> k) & 1;
    let c4 = b(n[0], 3) ^ b(n[0], 2) ^ b(n[0], 1) ^ b(n[0], 0);
    let c3 = b(n[0], 3) ^ b(n[1], 3) ^ b(n[1], 2) ^ b(n[1], 1) ^ b(n[2], 0);
    let c2 = b(n[0], 2) ^ b(n[1], 3) ^ b(n[1], 0) ^ b(n[2], 3) ^ b(n[2], 1);
    let c1 = b(n[0], 1) ^ b(n[1], 2) ^ b(n[1], 0) ^ b(n[2], 2) ^ b(n[2], 1) ^ b(n[2], 0);
    let c0 = b(n[0], 0) ^ b(n[1], 1) ^ b(n[2], 3) ^ b(n[2], 2) ^ b(n[2], 1) ^ b(n[2], 0);
    (c4 << 4) | (c3 << 3) | (c2 << 2) | (c1 << 1) | c0
}

/// Low-data-rate optimisation applies when the symbol lasts over 16 ms.
pub fn ldro(sf: u8, bw_hz: u32) -> bool {
    (1u64 << sf) as f64 * 1000.0 / bw_hz as f64 > 16.0
}

/// Number of payload symbols that follow the header block.
pub fn payload_symbol_count(
    payload_len: usize,
    sf: u8,
    cr: u8,
    has_crc: bool,
    ldro: bool,
) -> usize {
    let crc_bits = if has_crc { 4 } else { 0 };
    let denom = sf as i64 - if ldro { 2 } else { 0 };
    let numer = (2 * payload_len as i64 - sf as i64 + 2 + 5 + crc_bits).max(0);
    let blocks = (numer + denom - 1) / denom;
    (blocks * cr as i64) as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gray_is_its_own_inverse_pair() {
        for s in 0..4096u16 {
            assert_eq!(gray_decode(gray_encode(s)), s);
            assert_eq!(gray_encode(gray_decode(s)), s);
        }
    }

    #[test]
    fn hamming_round_trips_every_nibble_at_every_rate() {
        for cr in 5..=8u8 {
            for n in 0..16u8 {
                assert_eq!(hamming_decode(hamming_encode(n, cr), cr), n, "cr {cr} nibble {n}");
            }
        }
    }

    #[test]
    fn hamming_corrects_one_bit_at_4_7_and_4_8_only() {
        for cr in [7u8, 8] {
            for n in 0..16u8 {
                let cw = hamming_encode(n, cr);
                for bit in 0..cr {
                    assert_eq!(
                        hamming_decode(cw ^ (1 << bit), cr),
                        n,
                        "cr {cr} nibble {n} bit {bit}"
                    );
                }
            }
        }
        // 4/5 detects only: a flipped data bit comes out wrong.
        assert_ne!(hamming_decode(hamming_encode(5, 5) ^ 0b10000, 5), 5);
    }

    #[test]
    fn interleave_undoes_deinterleave() {
        let cws: Vec<u8> = (0..9u8).map(|i| i.wrapping_mul(37)).collect();
        let syms = interleave(&cws, 9, 8);
        assert_eq!(deinterleave(&syms, 9, 8), cws);
    }

    #[test]
    fn whitening_is_an_involution() {
        let mut d: Vec<u8> = (0..40u8).collect();
        let orig = d.clone();
        whiten(&mut d);
        assert_ne!(d, orig);
        whiten(&mut d);
        assert_eq!(d, orig);
    }

    #[test]
    fn the_symbol_count_matches_the_reference_frame() {
        // 3 payload bytes with CRC at SF11, CR 4/5: 5 symbols after the header
        // block (the reference capture has 13 symbols, 8 of them header).
        assert_eq!(payload_symbol_count(3, 11, 5, true, false), 5);
    }
}
