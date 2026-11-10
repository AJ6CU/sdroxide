//! Wire bytes to interleaved floats.
//!
//! The DDC delivers little-endian signed I,Q pairs, either 32-bit or 16-bit
//! depending on the rate ([`crate::protocol::component_bytes`]). Both are
//! divided by their own full scale and multiplied by the device's calibration,
//! so the two paths hand the engine the same numbers for the same signal.
//!
//! # The carry
//!
//! A bulk transfer is not guaranteed to end on a sample boundary — a stall, a
//! short packet or a resubmit can leave part of one behind. Those bytes are
//! kept and prefixed to the next block. Dropping them instead would shift the
//! stream by one component and swap I with Q for the rest of the session, which
//! shows up as a mirrored spectrum and reads like a driver bug rather than the
//! one lost sample it is.

use crate::protocol::component_bytes;

/// Turns wire bytes into interleaved `f32`, carrying any partial sample across
/// blocks.
pub struct Deconstructor {
    bytes_per_component: usize,
    scale: f32,
    /// Bytes of a component the last block ran out in the middle of. Always
    /// fewer than one component's worth: the moment it fills, it is decoded.
    carry: Vec<u8>,
    /// A decoded component with no partner yet — an I whose Q has not arrived.
    ///
    /// Held rather than emitted, because a consumer that reads pairs must never
    /// see a Q where an I should be: one unpaired float handed on would swap the
    /// two for the rest of the session, which presents as a mirrored spectrum
    /// and reads like a driver bug rather than the one ragged transfer it is.
    odd: Option<f32>,
}

impl Deconstructor {
    pub fn new(rate_hz: u32) -> Deconstructor {
        Deconstructor {
            bytes_per_component: component_bytes(rate_hz),
            scale: 1.0,
            carry: Vec::with_capacity(8),
            odd: None,
        }
    }

    /// The device's calibrated full-scale factor. Cheap to re-set, so the
    /// stream thread does it whenever a front-end switch moves.
    pub fn set_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    /// Forget the partial sample.
    ///
    /// Only after a stall: the stream resumes on a packet boundary there, so
    /// bytes carried from before it belong to nothing and prefixing them would
    /// misalign everything that follows.
    pub fn reset(&mut self) {
        self.carry.clear();
        self.odd = None;
    }

    /// Append the floats in `bytes` to `out`, keeping I and Q paired.
    ///
    /// `out` is not cleared: the caller reuses one buffer.
    pub fn push(&mut self, bytes: &[u8], out: &mut Vec<f32>) {
        let n = self.bytes_per_component;
        let mut src: &[u8] = bytes;

        // Finish the component the last block ran out in the middle of.
        if !self.carry.is_empty() {
            let need = n - self.carry.len();
            let take = need.min(src.len());
            self.carry.extend_from_slice(&src[..take]);
            src = &src[take..];
            if self.carry.len() < n {
                return;
            }
            let v = self.component(&self.carry);
            self.carry.clear();
            self.emit(v, out);
        }

        let whole = src.len() / n * n;
        for chunk in src[..whole].chunks_exact(n) {
            let v = self.component(chunk);
            self.emit(v, out);
        }
        self.carry.extend_from_slice(&src[whole..]);
    }

    /// Pair `v` with the component before it, or hold it until the next one.
    fn emit(&mut self, v: f32, out: &mut Vec<f32>) {
        match self.odd.take() {
            Some(i) => {
                out.push(i);
                out.push(v);
            }
            None => self.odd = Some(v),
        }
    }

    fn component(&self, bytes: &[u8]) -> f32 {
        match self.bytes_per_component {
            2 => {
                let v = i16::from_le_bytes([bytes[0], bytes[1]]);
                v as f32 / 32_768.0 * self.scale
            }
            _ => {
                let v = i32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                v as f32 / 2_147_483_648.0 * self.scale
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn i32_le(v: i32) -> [u8; 4] {
        v.to_le_bytes()
    }

    #[test]
    fn thirty_two_bit_samples_are_scaled_by_their_own_full_scale() {
        let mut d = Deconstructor::new(192_000);
        let mut out = Vec::new();
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&i32_le(i32::MAX));
        bytes.extend_from_slice(&i32_le(i32::MIN));
        d.push(&bytes, &mut out);
        assert_eq!(out.len(), 2);
        assert!((out[0] - 1.0).abs() < 1e-6, "{}", out[0]);
        assert!((out[1] + 1.0).abs() < 1e-6, "{}", out[1]);
    }

    #[test]
    fn sixteen_bit_samples_land_on_the_same_scale() {
        let mut d = Deconstructor::new(6_144_000);
        let mut out = Vec::new();
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&i16::MAX.to_le_bytes());
        bytes.extend_from_slice(&i16::MIN.to_le_bytes());
        d.push(&bytes, &mut out);
        assert_eq!(out.len(), 2);
        // The same signal at either width has to read the same, or the
        // panadapter's scale would jump when the rate setting changed.
        assert!((out[0] - 1.0).abs() < 1e-3, "{}", out[0]);
        assert!((out[1] + 1.0).abs() < 1e-6, "{}", out[1]);
    }

    #[test]
    fn the_calibration_multiplies_every_sample() {
        let mut d = Deconstructor::new(192_000);
        d.set_scale(0.5);
        let mut out = Vec::new();
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&i32_le(i32::MAX));
        bytes.extend_from_slice(&i32_le(0));
        d.push(&bytes, &mut out);
        assert!((out[0] - 0.5).abs() < 1e-6, "{}", out[0]);
        assert_eq!(out[1], 0.0);
    }

    /// A block that ends mid-component must not shift the stream: the bytes are
    /// held and prefixed to the next one.
    #[test]
    fn a_component_split_across_two_blocks_is_rejoined() {
        let mut d = Deconstructor::new(192_000);
        let mut out = Vec::new();
        let mut whole = Vec::new();
        whole.extend_from_slice(&i32_le(i32::MAX)); // I
        whole.extend_from_slice(&i32_le(i32::MIN)); // Q
        // Cut anywhere and the result has to be identical.
        for cut in 1..whole.len() {
            let mut d2 = Deconstructor::new(192_000);
            let mut split = Vec::new();
            d2.push(&whole[..cut], &mut split);
            d2.push(&whole[cut..], &mut split);
            assert_eq!(split.len(), 2, "cut at {cut}");
            assert!((split[0] - 1.0).abs() < 1e-6, "cut at {cut}");
            assert!((split[1] + 1.0).abs() < 1e-6, "cut at {cut}");
        }
        d.push(&whole, &mut out);
        assert_eq!(out.len(), 2);
    }

    /// An odd number of components is half a sample. Handing it on would swap I
    /// with Q for the rest of the session.
    #[test]
    fn an_odd_component_is_held_back_rather_than_handed_on() {
        let mut d = Deconstructor::new(192_000);
        let mut out = Vec::new();
        // One I with no Q behind it.
        d.push(&i32_le(1_000_000), &mut out);
        assert!(out.is_empty(), "half a sample was emitted: {out:?}");
        // The Q arrives next block and the pair completes, in order.
        d.push(&i32_le(-1_000_000), &mut out);
        assert_eq!(out.len(), 2);
        assert!(out[0] > 0.0 && out[1] < 0.0, "{out:?}");
    }

    /// After a stall the stream restarts on a packet boundary, so anything
    /// carried from before it belongs to nothing.
    #[test]
    fn a_reset_drops_the_carry_instead_of_misaligning_the_stream() {
        let mut d = Deconstructor::new(192_000);
        let mut out = Vec::new();
        d.push(&[0x01, 0x02], &mut out); // half a component
        d.reset();
        let mut bytes = Vec::new();
        bytes.extend_from_slice(&i32_le(i32::MAX));
        bytes.extend_from_slice(&i32_le(0));
        d.push(&bytes, &mut out);
        assert_eq!(out.len(), 2);
        assert!((out[0] - 1.0).abs() < 1e-6, "the carry was not dropped: {out:?}");
    }

    /// A long run of ragged blocks must not lose or gain a component, or the
    /// stream slowly rotates I into Q.
    #[test]
    fn ragged_blocks_preserve_every_sample_in_order() {
        let mut d = Deconstructor::new(192_000);
        let mut wire = Vec::new();
        for i in 0..200i32 {
            wire.extend_from_slice(&i32_le(i * 1_000_000));
        }
        let mut out = Vec::new();
        let mut at = 0;
        for step in [1usize, 3, 7, 4, 13, 2, 9, 32].into_iter().cycle() {
            if at >= wire.len() {
                break;
            }
            let end = (at + step).min(wire.len());
            d.push(&wire[at..end], &mut out);
            at = end;
        }
        assert_eq!(out.len(), 200);
        for (i, v) in out.iter().enumerate() {
            let want = (i as i32 * 1_000_000) as f32 / 2_147_483_648.0;
            assert!((v - want).abs() < 1e-6, "component {i}: {v} != {want}");
        }
    }
}
