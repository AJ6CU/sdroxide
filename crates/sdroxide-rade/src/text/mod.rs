//! The RADE V1 End-of-Over text channel: a station's callsign, carried by the
//! last frame of an over.
//!
//! This is the *only* way one FreeDV station identifies another on the air, and
//! it is what FreeDV Reporter `rx_report`s are built from — so it has to be
//! bit-exact with FreeDV GUI or it is worthless. Ported from `rade_text.cpp` in
//! <https://github.com/tmiw/freedv-integrations> / `freedv-backend`
//! (BSD-2-Clause, Mooneer Salem):
//!
//! ```text
//! callsign ─► 6-bit OTA alphabet (≤8 chars) ─► CRC-8 ─► 56 info bits
//!          ─► LDPC(112,56) ─► ×37 interleaver ─► QPSK ─► EOO floats
//! ```
//!
//! The array handed to and from the modem is [`crate::Rade::n_eoo_bits`] floats
//! long (180 for V1) holding 90 complex symbols as interleaved I/Q pairs. The
//! first 56 symbols carry the codeword; the rest is a known sequence the
//! receiver uses to estimate noise variance.
//!
//! Several details here look like bugs and are faithfully reproduced, because
//! FreeDV GUI has them too and interoperating means matching them exactly. They
//! are called out where they occur.

// Bit positions are the subject of the packing loops below, not an incidental
// way of walking an array, and they mirror the reference's arithmetic exactly.
#![allow(clippy::needless_range_loop)]

mod hra;
mod ldpc;

use hra::{K, N};
use ldpc::Sym;

/// Maximum callsign length an End-of-Over frame can carry.
pub const MAX_CHARS: usize = 8;

/// Floats occupied by the LDPC codeword at the head of the EOO array.
pub const CODEWORD_FLOATS: usize = N;

/// Interleaver stride. `(37 · i) mod 56` is a bijection because 37 is coprime
/// with 56, and it spreads adjacent codeword symbols far apart so a fade takes
/// out scattered bits rather than a run of them.
const INTERLEAVE_STRIDE: usize = 37;

// ─────────────────────────── 6-bit OTA alphabet ───────────────────────────
//
//   0        NUL (padding)
//   1..=9    ASCII 38..=46  ('&' … '.')
//   10..=19  '0'..='9'
//   20..=45  'A'..='Z'
//   46       '/'
//   47       SPACE — see `from_ota`

/// Map a callsign to OTA values, dropping anything unrepresentable.
///
/// Only the first [`MAX_CHARS`] *input* characters are considered (matching the
/// reference's `maxLength` bound), so dropped characters shorten the result
/// rather than letting later ones through.
///
/// SPACE is deliberately not mapped. The reference reserves OTA value 47 for
/// it but has no branch that produces one, so a space is silently dropped —
/// reproduced here, because encoding one would emit a value FreeDV GUI cannot
/// decode.
fn to_ota(callsign: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(MAX_CHARS);
    for c in callsign.chars().take(MAX_CHARS) {
        // The reference walks bytes and stops at NUL; a multi-byte character
        // cannot be represented anyway, so skipping it is the same outcome.
        let c = c.to_ascii_uppercase();
        let v = match c {
            // Note the overlap: '.' is ASCII 46 and lands here as 9, while '/'
            // is ASCII 47 and is handled by its own branch below.
            '&'..='.' => c as u8 - 37,
            '0'..='9' => c as u8 - b'0' + 10,
            'A'..='Z' => c as u8 - b'A' + 20,
            '/' => 46,
            _ => continue,
        };
        out.push(v);
    }
    out
}

/// Map OTA values back to text, stopping at the first NUL.
///
/// Value 47 (SPACE) has no case here — the reference drops it, and a decoder
/// that accepted it would disagree with FreeDV GUI about what was sent.
fn from_ota(vals: &[u8]) -> String {
    let mut out = String::with_capacity(vals.len());
    for &v in vals {
        match v {
            0 => break,
            1..=9 => out.push((v + 37) as char),
            10..=19 => out.push((v - 10 + b'0') as char),
            20..=45 => out.push((v - 20 + b'A') as char),
            46 => out.push('/'),
            _ => {}
        }
    }
    out
}

/// CRC-8 with polynomial 0x1D, initial value 0, MSB-first.
///
/// Stops at the first NUL, so the padding beyond a short callsign never
/// contributes — which is why the transmitter (which passes the true length)
/// and the receiver (which always passes [`MAX_CHARS`]) agree.
fn crc8(data: &[u8]) -> u8 {
    let mut crc: u8 = 0;
    for &b in data {
        if b == 0 {
            break;
        }
        crc ^= b;
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 { (crc << 1) ^ 0x1D } else { crc << 1 };
        }
    }
    crc
}

/// Pack a CRC byte and up to [`MAX_CHARS`] OTA values into 56 information bits:
/// the CRC LSB-first, then six bits per character, also LSB-first.
fn pack_info(crc: u8, ota: &[u8; MAX_CHARS]) -> [u8; K] {
    let mut bits = [0u8; K];
    for (i, b) in bits.iter_mut().take(8).enumerate() {
        *b = (crc >> i) & 1;
    }
    for i in 8..K {
        let since_crc = i - 8;
        bits[i] = (ota[since_crc / 6] >> (since_crc % 6)) & 1;
    }
    bits
}

/// Inverse of [`pack_info`].
fn unpack_info(bits: &[u8; N]) -> (u8, [u8; MAX_CHARS]) {
    let mut crc = 0u8;
    for i in 0..8 {
        crc |= (bits[i] & 1) << i;
    }
    let mut ota = [0u8; MAX_CHARS];
    for i in 8..K {
        let since_crc = i - 8;
        ota[since_crc / 6] |= (bits[i] & 1) << (since_crc % 6);
    }
    (crc, ota)
}

// ─────────────────────────── public API ───────────────────────────

/// Fill `out` with the End-of-Over soft-decision floats carrying `callsign`.
///
/// `out` must be at least [`CODEWORD_FLOATS`] long, and in practice is exactly
/// [`crate::Rade::n_eoo_bits`]. Floats `0..112` carry the interleaved
/// LDPC(112,56) codeword as QPSK I/Q pairs; everything beyond is set to the
/// known sequence — every tail symbol is `(1, 0)` — from which the receiver
/// estimates noise variance.
///
/// Characters outside the 6-bit alphabet (including SPACE) are dropped,
/// lowercase is upper-cased, and only the first [`MAX_CHARS`] input characters
/// are considered. An empty callsign is valid and produces a well-formed frame
/// carrying no text, which is still worth transmitting: the tail sequence is
/// what lets the far end measure its own noise floor.
pub fn encode(callsign: &str, out: &mut [f32]) {
    assert!(
        out.len() >= CODEWORD_FLOATS,
        "EOO buffer holds {} floats, need at least {CODEWORD_FLOATS}",
        out.len()
    );

    let ota_vec = to_ota(callsign);
    let mut ota = [0u8; MAX_CHARS];
    ota[..ota_vec.len()].copy_from_slice(&ota_vec);

    let info = pack_info(crc8(&ota), &ota);
    let code = ldpc::encode(&info);

    // Interleave whole symbols (bit pairs), then map to QPSK.
    for i in 0..K {
        let dst = (INTERLEAVE_STRIDE * i) % K;
        let (re, im) = match (code[2 * i], code[2 * i + 1]) {
            (0, 0) => (1.0, 0.0),
            (0, 1) => (0.0, 1.0),
            (1, 0) => (0.0, -1.0),
            _ => (-1.0, 0.0),
        };
        out[2 * dst] = re;
        out[2 * dst + 1] = im;
    }
    // The known tail: alternating 1.0/0.0 by *float* index, i.e. symbol (1, 0).
    for (i, v) in out.iter_mut().enumerate().skip(CODEWORD_FLOATS) {
        *v = if i % 2 == 0 { 1.0 } else { 0.0 };
    }
}

/// Recover a callsign from a received End-of-Over frame.
///
/// `eoo` is the buffer [`crate::Rade::rx`] fills when it reports an
/// End-of-Over; it holds `eoo.len() / 2` complex symbols.
///
/// Returns `None` unless the LDPC decoder converged *and* the recomputed CRC
/// matches the transmitted one — and also when the frame decoded cleanly but
/// carried no callsign, since there is then nothing to report. The CRC is what
/// keeps a false positive off the air: a converged-but-wrong codeword would
/// otherwise put a fabricated callsign into a reception report.
pub fn decode(eoo: &[f32]) -> Option<String> {
    if eoo.len() < CODEWORD_FLOATS {
        return None;
    }
    let n_syms = eoo.len() / 2;

    // Deinterleave, inverting `encode`'s placement.
    let mut syms = [Sym::default(); K];
    for (i, s) in syms.iter_mut().enumerate() {
        let src = (INTERLEAVE_STRIDE * i) % K;
        *s = Sym { re: eoo[2 * src], im: eoo[2 * src + 1] };
    }

    // Noise variance from the known tail, whose every symbol should be (1, 0).
    // Read from the raw array: the tail is outside the interleaved region.
    let mut ss = 0.0f32;
    let mut ss_count = 0i32;
    for i in K..n_syms {
        let (re, im) = (eoo[2 * i], eoo[2 * i + 1]);
        let amp = (re * re + im * im).sqrt();
        if amp > 0.0 {
            let dr = 1.0 - re / amp;
            let di = -im / amp;
            ss += dr * dr + di * di;
            ss_count += 2;
        }
    }
    // The reference divides by (n - 1) and clamps the result; with no usable
    // tail symbols that is a division by -1, giving -0.0, which the clamp then
    // lifts to the floor. Reproduced rather than "corrected".
    let noise_var = (ss / (ss_count - 1) as f32).max(1e-6);

    // Normalise to unit magnitude, keeping the magnitudes as per-symbol
    // amplitudes. A zero-magnitude symbol would divide by zero in the
    // reference; treat it as a null observation instead of poisoning the
    // decoder with NaN.
    let mut amps = [0.0f32; K];
    for (k, s) in syms.iter_mut().enumerate() {
        let amp = (s.re * s.re + s.im * s.im).sqrt();
        if amp > 0.0 {
            s.re /= amp;
            s.im /= amp;
        } else {
            *s = Sym { re: 0.0, im: 0.0 };
        }
        amps[k] = amp;
    }

    let d = ldpc::decode(&syms, &amps, noise_var);
    if !d.converged {
        return None;
    }
    let (rx_crc, ota) = unpack_info(&d.bits);
    if rx_crc != crc8(&ota) {
        return None;
    }
    let call = from_ota(&ota);
    (!call.is_empty()).then_some(call)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A full-length EOO buffer, as the V1 modem uses.
    const EOO_FLOATS: usize = 180;

    fn round_trip(call: &str) -> Option<String> {
        let mut buf = vec![0.0f32; EOO_FLOATS];
        encode(call, &mut buf);
        decode(&buf)
    }

    #[test]
    fn ota_alphabet_maps_both_ways() {
        for (text, vals) in [
            ("K1ABC", vec![30, 11, 20, 21, 22]),
            ("W1AW", vec![42, 11, 20, 42]),
            ("0", vec![10]),
            ("9", vec![19]),
            ("A", vec![20]),
            ("Z", vec![45]),
            ("/", vec![46]),
            (".", vec![9]),
            ("&", vec![1]),
        ] {
            assert_eq!(to_ota(text), vals, "encoding {text:?}");
            assert_eq!(from_ota(&vals), text, "decoding {text:?}");
        }
    }

    #[test]
    fn lowercase_is_upcased() {
        assert_eq!(to_ota("k1abc"), to_ota("K1ABC"));
        assert_eq!(round_trip("oe1tkj").as_deref(), Some("OE1TKJ"));
    }

    #[test]
    fn space_is_dropped_and_ota_47_is_ignored() {
        // The reference reserves 47 for SPACE but never emits one, and its
        // decoder has no case for it. Both halves reproduced.
        assert_eq!(to_ota("K1 ABC"), to_ota("K1ABC"));
        assert_eq!(from_ota(&[30, 47, 11]), "K1", "value 47 is skipped, not rendered");
    }

    #[test]
    fn the_punctuation_block_is_representable() {
        // ASCII 38..=46 all map, so '-' (45) is a legal callsign character
        // even though it is not one a real callsign uses.
        assert_eq!(to_ota("-"), vec![8]);
        assert_eq!(from_ota(&[8]), "-");
        assert_eq!(round_trip("K1-ABC").as_deref(), Some("K1-ABC"));
    }

    #[test]
    fn unrepresentable_characters_are_dropped() {
        assert_eq!(to_ota("K1_ABC"), to_ota("K1ABC"), "'_' (95) is above the alphabet");
        assert_eq!(to_ota("K1!ABC"), to_ota("K1ABC"), "'!' (33) is below it");
        assert_eq!(to_ota("Kü1"), to_ota("K1"), "non-ASCII is dropped");
    }

    #[test]
    fn only_the_first_eight_input_characters_are_considered() {
        assert_eq!(to_ota("ABCDEFGHIJK"), to_ota("ABCDEFGH"));
        assert_eq!(round_trip("ABCDEFGHIJK").as_deref(), Some("ABCDEFGH"));
        // The bound is on *input*, so dropped characters shorten the result
        // rather than pulling later ones in.
        assert_eq!(to_ota("A B C D E"), to_ota("ABCD"));
    }

    #[test]
    fn crc8_is_stable_and_length_independent_past_the_nul() {
        // Padding beyond the callsign must not change the CRC — this is what
        // lets the transmitter pass the true length and the receiver pass 8.
        for call in ["K1ABC", "OE1TKJ", "VK7XYZ/P", "W1AW", "A", ""] {
            let v = to_ota(call);
            let mut padded = [0u8; MAX_CHARS];
            padded[..v.len()].copy_from_slice(&v);
            assert_eq!(crc8(&v), crc8(&padded), "padding changed the CRC for {call:?}");
        }
        assert_eq!(crc8(&[]), 0);
        assert_eq!(crc8(&[0, 30, 11]), 0, "a leading NUL stops the CRC immediately");
    }

    #[test]
    fn info_bits_round_trip() {
        let ota = [30u8, 11, 20, 21, 22, 0, 0, 0];
        let crc = crc8(&ota);
        let info = pack_info(crc, &ota);
        // unpack_info reads the first K of a full codeword.
        let mut code = [0u8; N];
        code[..K].copy_from_slice(&info);
        assert_eq!(unpack_info(&code), (crc, ota));
    }

    #[test]
    fn the_interleaver_is_a_bijection() {
        let mut seen = [false; K];
        for i in 0..K {
            let dst = (INTERLEAVE_STRIDE * i) % K;
            assert!(!seen[dst], "index {dst} produced twice");
            seen[dst] = true;
        }
        assert!(seen.iter().all(|&s| s));
    }

    #[test]
    fn the_tail_is_the_known_sequence() {
        let mut buf = vec![0.0f32; EOO_FLOATS];
        encode("K1ABC", &mut buf);
        for i in CODEWORD_FLOATS..EOO_FLOATS {
            let want = if i % 2 == 0 { 1.0 } else { 0.0 };
            assert_eq!(buf[i], want, "tail float {i}");
        }
    }

    #[test]
    fn encoded_symbols_are_exactly_representable() {
        // Only -1, 0 and +1 come out, which is what makes byte-for-byte
        // comparison against the C++ reference legitimate.
        let mut buf = vec![0.0f32; EOO_FLOATS];
        encode("VK7XYZ/P", &mut buf);
        for (i, v) in buf.iter().enumerate() {
            assert!(*v == -1.0 || *v == 0.0 || *v == 1.0, "float {i} is {v}");
        }
        // Every codeword symbol sits on the constellation (unit magnitude).
        for k in 0..K {
            let (re, im) = (buf[2 * k], buf[2 * k + 1]);
            assert_eq!(re * re + im * im, 1.0, "symbol {k} is off the constellation");
        }
    }

    #[test]
    fn noiseless_round_trip() {
        for call in ["K1ABC", "OE1TKJ", "VK7XYZ/P", "W1AW", "2E0ABC", "A", "AA1ZZZZZ"] {
            assert_eq!(round_trip(call).as_deref(), Some(call), "round trip of {call:?}");
        }
    }

    #[test]
    fn an_empty_callsign_encodes_but_reports_nothing() {
        // Still a well-formed frame — the tail is what the far end measures its
        // noise floor from — but there is no callsign to report.
        let mut buf = vec![0.0f32; EOO_FLOATS];
        encode("", &mut buf);
        assert_eq!(buf[..CODEWORD_FLOATS].iter().filter(|v| **v != 0.0).count(), K);
        assert_eq!(decode(&buf), None);
    }

    /// xorshift64*, so any failure is reproducible.
    struct Rng(u64);
    impl Rng {
        fn next_u64(&mut self) -> u64 {
            let mut x = self.0;
            x ^= x >> 12;
            x ^= x << 25;
            x ^= x >> 27;
            self.0 = x;
            x.wrapping_mul(0x2545_F491_4F6C_DD1D)
        }
        /// Uniform in [0, 1).
        fn unit(&mut self) -> f32 {
            (self.next_u64() >> 40) as f32 / (1u32 << 24) as f32
        }
        /// Standard normal, Box–Muller.
        fn normal(&mut self) -> f32 {
            let u1 = self.unit().max(1e-7);
            let u2 = self.unit();
            (-2.0 * u1.ln()).sqrt() * (std::f32::consts::TAU * u2).cos()
        }
    }

    fn add_noise(buf: &mut [f32], sigma: f32, rng: &mut Rng) {
        for v in buf.iter_mut() {
            *v += sigma * rng.normal();
        }
    }

    #[test]
    fn survives_awgn() {
        // σ = 0.4 is comfortably inside the code's working range; the waterfall
        // falls off steeply above ~0.6, so this threshold is not marginal.
        let mut rng = Rng(0x5eed_1234);
        let mut ok = 0;
        const TRIALS: usize = 50;
        for _ in 0..TRIALS {
            let mut buf = vec![0.0f32; EOO_FLOATS];
            encode("OE1TKJ", &mut buf);
            add_noise(&mut buf, 0.4, &mut rng);
            if decode(&buf).as_deref() == Some("OE1TKJ") {
                ok += 1;
            }
        }
        assert!(ok >= 48, "only {ok}/{TRIALS} recovered at sigma 0.4");
    }

    #[test]
    fn the_crc_rejects_a_valid_codeword_carrying_the_wrong_checksum() {
        // Construct a syndrome-valid codeword whose CRC field is deliberately
        // wrong: the decoder will converge, and only the CRC stands between
        // that and a fabricated callsign on the public reporter.
        let ota = {
            let v = to_ota("K1ABC");
            let mut a = [0u8; MAX_CHARS];
            a[..v.len()].copy_from_slice(&v);
            a
        };
        let good = crc8(&ota);
        let info = pack_info(good ^ 0x01, &ota);
        let code = ldpc::encode(&info);

        let mut buf = vec![0.0f32; EOO_FLOATS];
        for i in 0..K {
            let dst = (INTERLEAVE_STRIDE * i) % K;
            let (re, im) = match (code[2 * i], code[2 * i + 1]) {
                (0, 0) => (1.0, 0.0),
                (0, 1) => (0.0, 1.0),
                (1, 0) => (0.0, -1.0),
                _ => (-1.0, 0.0),
            };
            buf[2 * dst] = re;
            buf[2 * dst + 1] = im;
        }
        for (i, v) in buf.iter_mut().enumerate().skip(CODEWORD_FLOATS) {
            *v = if i % 2 == 0 { 1.0 } else { 0.0 };
        }
        assert_eq!(decode(&buf), None, "a bad CRC must not yield a callsign");
    }

    #[test]
    fn garbage_never_decodes() {
        // A false positive here would put a station that does not exist onto
        // qso.freedv.org, so this is the test that matters most.
        let mut rng = Rng(0xfeed_face);
        for _ in 0..2000 {
            let buf: Vec<f32> = (0..EOO_FLOATS).map(|_| rng.unit() * 4.0 - 2.0).collect();
            assert_eq!(decode(&buf), None, "random noise decoded to a callsign");
        }
    }

    #[test]
    fn degenerate_input_is_handled() {
        assert_eq!(decode(&[]), None);
        assert_eq!(decode(&[0.0; 64]), None, "too short for a codeword");
        // All zeros: every amplitude is zero, so both the noise estimate and
        // the normalisation hit their guards.
        assert_eq!(decode(&[0.0; EOO_FLOATS]), None);
        assert_eq!(decode(&[f32::NAN; EOO_FLOATS]), None);
        assert_eq!(decode(&[f32::INFINITY; EOO_FLOATS]), None);
        // Exactly the codeword region, with no tail to measure noise from.
        assert_eq!(decode(&[0.5; CODEWORD_FLOATS]), None);
    }

    #[test]
    #[should_panic(expected = "EOO buffer")]
    fn encoding_into_a_short_buffer_is_a_programming_error() {
        let mut buf = [0.0f32; 100];
        encode("K1ABC", &mut buf);
    }
}
