//! Sub-audible squelch signalling on analogue FM: CTCSS tones and DCS.
//!
//! Both ride under the voice on the same carrier — CTCSS as a continuous tone
//! between 67 and 254 Hz, DCS as a continuous 134.4 bps data stream — and both
//! exist so a receiver can ignore everything not addressed to it. On a
//! monitoring receiver they are worth showing whether or not you gate on them:
//! the tone identifies which repeater or system you are hearing.

use serde::{Deserialize, Serialize};

/// The 50 standard CTCSS tones of EIA/TIA-603, in tenths of a Hz.
///
/// Tenths rather than a float because these are catalogue entries, not
/// measurements: the decoder reports which one it matched, and both a tone
/// squelch comparison and the wire format want that to be exact.
pub const CTCSS_TONES: [u16; 50] = [
    670, 693, 719, 744, 770, 797, 825, 854, 885, 915, 948, 974, 1000, 1035, 1072, 1109, 1148, 1188,
    1230, 1273, 1318, 1365, 1413, 1462, 1514, 1567, 1598, 1622, 1655, 1679, 1713, 1738, 1773, 1799,
    1835, 1862, 1899, 1928, 1966, 1995, 2035, 2065, 2107, 2181, 2257, 2291, 2336, 2418, 2503, 2541,
];

/// A sub-audible squelch signal recovered from an FM signal, or required of one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SubTone {
    /// A CTCSS tone in tenths of a Hz — always one of [`CTCSS_TONES`].
    Ctcss(u16),
    /// A DCS data stream, without the three-digit code it carries.
    ///
    /// Reading the code needs the exact wire convention — which end of the
    /// 23-bit word goes out first, what the three fixed bits are set to — and
    /// that could not be established here. It matters more than it sounds:
    /// the code is carried in a *cyclic* Golay codeword, so every one of the 23
    /// possible word boundaries decodes to a valid codeword, and picking the
    /// wrong boundary yields a different, equally plausible code. Under every
    /// convention that was tried against the standard code list, each signal had
    /// exactly two readings — the right one, and one other code at the opposite
    /// polarity. A code shown on a coin toss is worse than no code, so this says
    /// only what can be established: that the signal is DCS rather than CTCSS,
    /// which the 23-bit periodicity of the data proves on its own.
    Dcs,
}

impl SubTone {
    /// How radios and repeater directories write it: `88.5`, or `DCS`.
    pub fn label(self) -> String {
        match self {
            SubTone::Ctcss(tenths) => format!("{}.{}", tenths / 10, tenths % 10),
            SubTone::Dcs => "DCS".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tones_are_sorted_and_far_enough_apart_to_tell_apart() {
        let mut closest = u16::MAX;
        for w in CTCSS_TONES.windows(2) {
            assert!(w[1] > w[0], "the table must be ascending: {} then {}", w[0], w[1]);
            closest = closest.min(w[1] - w[0]);
        }
        // 67.0/69.3 Hz is the tightest pair in the standard set. The detector's
        // analysis window has to resolve 2.3 Hz, which is what sets its length.
        assert_eq!(closest, 23, "closest standard pair, in tenths of a Hz");
    }

    #[test]
    fn labels_read_the_way_radios_write_them() {
        assert_eq!(SubTone::Ctcss(885).label(), "88.5");
        assert_eq!(SubTone::Ctcss(670).label(), "67.0");
        assert_eq!(SubTone::Ctcss(2541).label(), "254.1");
        assert_eq!(SubTone::Ctcss(1000).label(), "100.0");
        assert_eq!(SubTone::Dcs.label(), "DCS");
    }
}
