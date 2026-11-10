//! Turning the bulk endpoint's bytes into real samples.
//!
//! The device sends 16-bit signed little-endian *real* samples — one channel,
//! no I/Q — optionally passed through the LTC2208's output randomizer.

/// Undo the LTC2208 output randomizer.
///
/// The ADC scrambles its own parallel output so the digital bus stops radiating
/// into the analogue front end sitting a few millimetres away: every bit except
/// the least significant is XORed with the least significant one, which is left
/// alone to say whether the rest were inverted. The operation is its own
/// inverse, so this function both applies and removes it.
///
/// With the randomizer enabled and this step missing, the result is not subtly
/// wrong — it is full-scale noise, because the top bit flips on half of all
/// samples.
///
/// Written branchlessly: bit 0 is broadcast to a full-width mask, with bit 0
/// itself cleared from it so the flag survives the XOR. A conditional here
/// would read more plainly, but this runs once per sample at the ADC's full
/// 32 Msps and a per-sample branch is what stops the conversion loop below
/// from vectorising.
#[inline]
pub fn derandomize(sample: i16) -> i16 {
    sample ^ ((sample & 1).wrapping_neg() & !1i16)
}

/// Full scale for a 16-bit signed sample.
const SCALE: f32 = 1.0 / 32768.0;

/// Convert a buffer of little-endian `i16` into `f32` in `-1.0..1.0`.
///
/// A bulk transfer can complete on an *odd* byte count, which would put every
/// subsequent sample one byte out of phase — a far worse outcome than losing a
/// byte, since it corrupts the rest of the session rather than one sample. The
/// straggler is therefore carried into the next call, exactly as the RTL-SDR
/// backend carries a lone I or Q byte.
///
/// `out` is resized to hold exactly the samples produced and every element is
/// overwritten, so the caller can reuse one buffer. Sizing rather than clearing
/// and pushing is deliberate: a steady stream of equal-sized bulk transfers
/// leaves the length untouched from call to call, so the buffer is written once
/// instead of being zeroed and then filled.
pub fn to_f32(bytes: &[u8], randomized: bool, carry: &mut Option<u8>, out: &mut Vec<f32>) {
    let mut rest = bytes;
    // A carried byte pairs with the first byte here, producing one sample that
    // sits ahead of everything the bulk loop below converts.
    let mut lead = None;
    if let Some(lo) = carry.take() {
        if let Some((hi, tail)) = rest.split_first() {
            lead = Some(sample(lo, *hi, randomized));
            rest = tail;
        } else {
            // Nothing arrived to pair with it; keep holding on.
            *carry = Some(lo);
            out.clear();
            return;
        }
    }

    let (pairs, odd) = rest.as_chunks::<2>();
    out.resize(pairs.len() + usize::from(lead.is_some()), 0.0);
    let (head, body) = out.split_at_mut(usize::from(lead.is_some()));
    if let Some(v) = lead {
        head[0] = v;
    }
    convert(body, pairs, randomized);
    if let Some(&lo) = odd.first() {
        *carry = Some(lo);
    }
}

/// The bulk of the work: whole little-endian pairs to scaled `f32`.
///
/// `randomized` is tested once for the whole block rather than once per sample
/// so that each loop is a straight-line stride over a slice of known length —
/// which is what lets the compiler widen it into vector loads and converts.
#[inline]
fn convert(dst: &mut [f32], src: &[[u8; 2]], randomized: bool) {
    debug_assert_eq!(dst.len(), src.len());
    if randomized {
        for (d, p) in dst.iter_mut().zip(src) {
            *d = derandomize(i16::from_le_bytes(*p)) as f32 * SCALE;
        }
    } else {
        for (d, p) in dst.iter_mut().zip(src) {
            *d = i16::from_le_bytes(*p) as f32 * SCALE;
        }
    }
}

#[inline]
fn sample(lo: u8, hi: u8, randomized: bool) -> f32 {
    let raw = i16::from_le_bytes([lo, hi]);
    let s = if randomized { derandomize(raw) } else { raw };
    s as f32 * SCALE
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The randomizer as the datasheet describes it, written independently of
    /// the implementation so the test is not the implementation restated.
    fn spec_randomize(sample: i16) -> i16 {
        let u = sample as u16;
        if u & 1 != 0 {
            // Invert bits 15..1, leave bit 0 alone.
            (u ^ 0xFFFE) as i16
        } else {
            sample
        }
    }

    #[test]
    fn derandomize_inverts_the_datasheet_operation() {
        for raw in i16::MIN..=i16::MAX {
            assert_eq!(derandomize(spec_randomize(raw)), raw, "raw {raw}");
        }
    }

    #[test]
    fn derandomize_is_its_own_inverse() {
        for raw in i16::MIN..=i16::MAX {
            assert_eq!(derandomize(derandomize(raw)), raw);
        }
    }

    #[test]
    fn even_samples_pass_through_untouched() {
        // Bit 0 clear means the ADC did not invert anything.
        for raw in [0i16, 2, -2, 0x7ffe, -0x8000] {
            assert_eq!(derandomize(raw), raw);
        }
    }

    #[test]
    fn odd_length_transfers_do_not_shift_every_later_sample() {
        // Split a run of samples at an odd byte boundary and confirm the
        // reassembled stream matches the unsplit one.
        let vals: Vec<i16> = (0..64).map(|i| (i * 517) as i16).collect();
        let bytes: Vec<u8> = vals.iter().flat_map(|v| v.to_le_bytes()).collect();

        let mut whole = Vec::new();
        to_f32(&bytes, false, &mut None, &mut whole);

        let mut carry = None;
        let mut a = Vec::new();
        let mut b = Vec::new();
        to_f32(&bytes[..31], false, &mut carry, &mut a);
        assert!(carry.is_some(), "the 31st byte must be held back");
        to_f32(&bytes[31..], false, &mut carry, &mut b);
        assert_eq!(carry, None);

        a.extend_from_slice(&b);
        assert_eq!(a, whole);
    }

    #[test]
    fn a_lone_byte_is_held_until_a_partner_arrives() {
        let mut carry = None;
        let mut out = Vec::new();
        to_f32(&[0x34], false, &mut carry, &mut out);
        assert!(out.is_empty());
        assert_eq!(carry, Some(0x34));

        // An empty transfer must not drop it.
        to_f32(&[], false, &mut carry, &mut out);
        assert!(out.is_empty());
        assert_eq!(carry, Some(0x34));

        to_f32(&[0x12], false, &mut carry, &mut out);
        assert_eq!(carry, None);
        assert_eq!(out, vec![0x1234 as f32 * SCALE]);
    }

    #[test]
    fn full_scale_maps_just_inside_unity() {
        let mut out = Vec::new();
        to_f32(&i16::MAX.to_le_bytes(), false, &mut None, &mut out);
        assert!((out[0] - 0.99997).abs() < 1e-4, "{}", out[0]);
        to_f32(&i16::MIN.to_le_bytes(), false, &mut None, &mut out);
        assert_eq!(out[0], -1.0);
    }

    /// What the unpack costs per second of ADC output.
    ///
    /// The RX-888 clocks 32 Msps of 16-bit real samples, so this loop sees
    /// 64 MB/s no matter what the receiver downstream is doing — it is a fixed
    /// tax on every session. The figure to watch is the last column: anything
    /// much above a few percent of a core means the conversion stopped
    /// vectorising and went back to one sample at a time.
    #[test]
    #[ignore = "a measurement, not an assertion; needs --release to mean anything"]
    fn unpack_cost_per_second_of_adc() {
        use std::time::Instant;

        const SPS: usize = 32_400_000;
        // One bulk transfer's worth, sized as the streamer sizes them.
        let bytes: Vec<u8> = (0..1 << 20).map(|i| (i * 7) as u8).collect();
        let samples_per_call = bytes.len() / 2;
        let calls = SPS / samples_per_call;

        println!("{:>12}{:>12}{:>16}", "randomized", "ms/s of RF", "% of one core");
        for randomized in [false, true] {
            let mut out = Vec::new();
            // Warm the buffer to its steady-state length first: the point of
            // the measurement is the loop, not the one-off allocation.
            to_f32(&bytes, randomized, &mut None, &mut out);

            let t = Instant::now();
            for _ in 0..calls {
                to_f32(&bytes, randomized, &mut None, &mut out);
            }
            let ms = t.elapsed().as_secs_f64() * 1e3;
            println!("{randomized:>12}{ms:>12.2}{:>16.2}", ms / 10.0);
        }
    }

    #[test]
    fn randomized_bytes_recover_the_original_signal() {
        // A ramp, scrambled the way the ADC would, must come back exactly.
        let vals: Vec<i16> = (-32768..32768).step_by(97).map(|v| v as i16).collect();
        let scrambled: Vec<u8> =
            vals.iter().flat_map(|v| spec_randomize(*v).to_le_bytes()).collect();

        let mut out = Vec::new();
        to_f32(&scrambled, true, &mut None, &mut out);

        let want: Vec<f32> = vals.iter().map(|v| *v as f32 * SCALE).collect();
        assert_eq!(out, want);
    }
}
