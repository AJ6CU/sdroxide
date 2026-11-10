//! Wire bytes to baseband and back.
//!
//! The HackRF's samples are **signed** 8-bit, I then Q. The RTL-SDR's, in the
//! crate next door, are *unsigned* offset-binary with a measured 127.4 centre.
//! The two are one line apart in shape and nothing in a spectrum makes the
//! difference obvious — an offset-binary reading of signed bytes puts a large
//! constant on I and Q, which looks like a DC spike, which is exactly what
//! everybody expects to see on a zero-IF radio anyway. So the table is built
//! here from a signed cast and the test asserts its endpoints against
//! literals: a "fix" that copies the RTL-SDR's formula across has to fail.

use sdroxide_dsp::Complex32;

/// Conversion lookup: signed 8-bit to `f32`.
///
/// No 127.4 fudge and no offset — mid-scale really is zero on this radio.
/// 1/128 puts full scale just inside ±1.0, so a saturated sample cannot become
/// an out-of-range float downstream. A table rather than arithmetic because
/// this runs 40 million times a second at 20 Msps.
pub fn sample_lut() -> [f32; 256] {
    core::array::from_fn(|i| (i as u8 as i8) as f32 / 128.0)
}

/// Convert a transfer's signed 8-bit I/Q into complex samples, appending.
///
/// `carry` holds a leftover byte from a previous short completion so that I/Q
/// pairing survives a transfer that ended between an I and its Q. Without it a
/// single odd-length completion swaps I and Q for the rest of the session — a
/// mirrored spectrum that reads like a driver bug rather than the framing slip
/// it is.
pub fn to_iq(bytes: &[u8], lut: &[f32; 256], carry: &mut Option<u8>, out: &mut Vec<Complex32>) {
    out.reserve(bytes.len() / 2 + 1);

    let mut rest = bytes;
    if let Some(c) = carry.take() {
        match rest.split_first() {
            Some((first, tail)) => {
                out.push(Complex32::new(lut[c as usize], lut[*first as usize]));
                rest = tail;
            }
            // Nothing arrived to pair it with; keep holding it.
            None => {
                *carry = Some(c);
                return;
            }
        }
    }

    let pairs = rest.len() / 2;
    for chunk in rest[..pairs * 2].chunks_exact(2) {
        out.push(Complex32::new(lut[chunk[0] as usize], lut[chunk[1] as usize]));
    }
    if rest.len() % 2 == 1 {
        *carry = Some(rest[rest.len() - 1]);
    }
}

/// Full scale for the transmit path.
///
/// **127, not 128.** The DAC takes the same signed byte the ADC produces, so
/// −128 is representable — but using it makes the negative excursion one step
/// longer than the positive one, and an asymmetric clip is a second harmonic
/// that a symmetric one does not have. Giving up one part in 128 of output
/// power to keep the constellation symmetric is the right trade on a radio
/// whose harmonic suppression is already its weakest property.
const TX_FULL_SCALE: f32 = 127.0;

/// Convert complex baseband into the signed 8-bit pairs the transmit endpoint
/// wants, appending, and count how many components clipped.
///
/// The saturation count is not diagnostics for its own sake: this radio's drive
/// is applied digitally before it ever reaches here, so an operator who turns
/// drive up past unity gets intermodulation rather than power, with nothing on
/// screen to say so. The counter is what lets the stats line say "the modulator
/// is clipping" instead of leaving them to wonder about their IMD.
pub fn from_iq(samples: &[Complex32], out: &mut Vec<u8>, saturated: &mut u64) {
    out.reserve(samples.len() * 2);
    for s in samples {
        for v in [s.re, s.im] {
            let scaled = v * TX_FULL_SCALE;
            // Written as two comparisons rather than `!(-FS..=FS).contains(..)`,
            // which clippy suggests and which would be wrong: `contains` is
            // false for NaN, so negating it counts every NaN as a clip. NaN is
            // a bug upstream, not the modulator running out of headroom, and
            // conflating the two would make the clipping indicator cry wolf.
            // Here NaN is neither above nor below, so it falls through to the
            // cast — which yields 0, silence for one component.
            #[allow(clippy::manual_range_contains)]
            if scaled > TX_FULL_SCALE || scaled < -TX_FULL_SCALE {
                *saturated += 1;
            }
            let clamped = scaled.clamp(-TX_FULL_SCALE, TX_FULL_SCALE);
            out.push(clamped.round() as i8 as u8);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lut() -> [f32; 256] {
        sample_lut()
    }

    /// Asserted against literals, not against the formula, so that copying the
    /// RTL-SDR's offset-binary table across fails here rather than in the field.
    #[test]
    fn the_table_is_signed_and_centred_on_zero() {
        let l = lut();
        assert_eq!(l[0x00], 0.0, "mid-scale on this radio is byte 0, not byte 128");
        assert_eq!(l[0x80], -1.0, "0x80 is -128, the most negative sample");
        assert_eq!(l[0x7f], 127.0 / 128.0);
        assert_eq!(l[0xff], -1.0 / 128.0, "0xff is -1, not full positive scale");
        assert_eq!(l[0x01], 1.0 / 128.0);
        // Nothing exceeds unity, so a saturated sample cannot become an
        // out-of-range float downstream.
        for (i, v) in l.iter().enumerate() {
            assert!(v.abs() <= 1.0, "byte {i} converts to {v}");
        }
        // The unsigned reading would put mid-scale near byte 128; this one must
        // not. Stated as an assertion because it is the whole hazard.
        assert!(l[0x80] != 0.0, "a signed table cannot have 0x80 at DC");
    }

    /// The hazard this defends against: a bulk transfer completing on an odd
    /// byte count. Split the same stream into odd-length chunks and the output
    /// must be identical to processing it in one go — otherwise I and Q swap
    /// for the rest of the session.
    #[test]
    fn odd_length_transfers_do_not_swap_i_and_q() {
        let l = lut();
        let stream: Vec<u8> = (0..64u8).collect();

        let mut carry = None;
        let mut whole = Vec::new();
        to_iq(&stream, &l, &mut carry, &mut whole);
        assert_eq!(carry, None, "an even stream leaves nothing carried");

        let mut carry = None;
        let mut chunked = Vec::new();
        let mut pos = 0;
        for len in [3usize, 5, 7, 1, 9, 2, 11, 6, 13, 7] {
            let end = (pos + len).min(stream.len());
            if pos >= end {
                break;
            }
            to_iq(&stream[pos..end], &l, &mut carry, &mut chunked);
            pos = end;
        }
        if pos < stream.len() {
            to_iq(&stream[pos..], &l, &mut carry, &mut chunked);
        }

        assert_eq!(whole, chunked, "odd-length chunking must not shift the I/Q phase");
    }

    #[test]
    fn a_lone_byte_is_held_until_a_partner_arrives() {
        let l = lut();
        let mut carry = None;
        let mut out = Vec::new();

        to_iq(&[0x10], &l, &mut carry, &mut out);
        assert!(out.is_empty(), "a single byte cannot form a pair");
        assert_eq!(carry, Some(0x10));

        // An empty completion must not discard the carried byte.
        to_iq(&[], &l, &mut carry, &mut out);
        assert!(out.is_empty());
        assert_eq!(carry, Some(0x10), "an empty transfer must not drop the carry");

        to_iq(&[0x20], &l, &mut carry, &mut out);
        assert_eq!(out, vec![Complex32::new(l[0x10], l[0x20])]);
        assert_eq!(carry, None);
    }

    /// I is first on the wire. Getting this backwards mirrors the spectrum,
    /// which on a symmetric test signal looks like nothing at all being wrong.
    #[test]
    fn i_comes_before_q() {
        let l = lut();
        let mut carry = None;
        let mut out = Vec::new();
        // +1 quadrant: I strongly positive, Q slightly positive.
        to_iq(&[0x7f, 0x01], &l, &mut carry, &mut out);
        assert_eq!(out.len(), 1);
        assert!(out[0].re > 0.9, "the first byte is I: {:?}", out[0]);
        assert!(out[0].im > 0.0 && out[0].im < 0.1, "the second byte is Q: {:?}", out[0]);
    }

    /// A clip has to be symmetric, and it has to be counted.
    #[test]
    fn transmit_clipping_is_symmetric_and_counted() {
        let mut out = Vec::new();
        let mut sat = 0;
        from_iq(&[Complex32::new(2.0, -2.0)], &mut out, &mut sat);
        assert_eq!(out, vec![127u8, (-127i8) as u8], "the clip must be symmetric");
        assert_eq!(sat, 2, "both components clipped");

        // Exactly full scale is not a clip.
        out.clear();
        sat = 0;
        from_iq(&[Complex32::new(1.0, -1.0)], &mut out, &mut sat);
        assert_eq!(out, vec![127u8, (-127i8) as u8]);
        assert_eq!(sat, 0, "unity is the top of the range, not past it");

        // Zero is zero, and −128 is never emitted however hard it is pushed.
        out.clear();
        sat = 0;
        from_iq(&[Complex32::new(0.0, 0.0), Complex32::new(-1000.0, -1000.0)], &mut out, &mut sat);
        assert_eq!(out[0..2], [0u8, 0]);
        assert!(out[2..4].iter().all(|b| *b as i8 == -127), "-128 must never be emitted");
    }

    /// NaN reaching the modulator is a bug upstream, but it must come out as
    /// silence rather than as whatever the bit pattern casts to.
    #[test]
    fn a_nan_sample_becomes_silence() {
        let mut out = Vec::new();
        let mut sat = 0;
        from_iq(&[Complex32::new(f32::NAN, f32::INFINITY)], &mut out, &mut sat);
        assert_eq!(out[0], 0, "NaN is silence");
        assert_eq!(out[1] as i8, 127, "infinity clips to full scale");
        assert_eq!(sat, 1, "infinity saturated; NaN is not a saturation");
    }

    /// Round trip, and the exact size of the mismatch between the two scale
    /// factors.
    ///
    /// Transmit divides by 127 and receive multiplies by 1/128, so a sample
    /// sent out and read back comes home `1/128` low — a systematic 0.068 dB
    /// droop — plus up to half a step of quantisation. That is 1.5/128 in
    /// total, and it is the price of the symmetric clip [`TX_FULL_SCALE`] buys.
    /// Asserting the real bound rather than a round number means a later change
    /// to either scale factor shows up here as an arithmetic fact instead of
    /// hiding inside a loose tolerance.
    #[test]
    fn the_two_directions_agree_to_within_a_step_and_a_half() {
        let l = lut();
        let mut out = Vec::new();
        let mut sat = 0;
        let want: Vec<Complex32> =
            (-100..=100).map(|i| Complex32::new(i as f32 / 100.0, -i as f32 / 100.0)).collect();
        from_iq(&want, &mut out, &mut sat);
        assert_eq!(sat, 0, "nothing in ±1.0 should clip");

        let mut carry = None;
        let mut back = Vec::new();
        to_iq(&out, &l, &mut carry, &mut back);
        assert_eq!(back.len(), want.len());

        let bound = 1.5 / 128.0;
        let mut worst: f32 = 0.0;
        for (a, b) in want.iter().zip(&back) {
            for (x, y) in [(a.re, b.re), (a.im, b.im)] {
                let err = (x - y).abs();
                assert!(err <= bound, "{a:?} came back as {b:?}, off by {err}");
                worst = worst.max(err);
            }
        }
        // The droop is real, not a rounding artefact that a tighter table would
        // remove: at full scale it is a whole step on its own.
        assert!(worst > 1.0 / 128.0, "the 127-vs-128 droop should be visible: {worst}");
    }
}
