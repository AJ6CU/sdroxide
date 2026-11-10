//! Homematic BidCoS on 868.3 MHz — recognised, not read.
//!
//! # What this module does and does not claim
//!
//! It says "there is a Homematic device here" and nothing else. That is the
//! whole of it, and it is deliberate.
//!
//! The recognition is solid: BidCoS opens with a preamble of `0xaa` and then a
//! **thirty-two bit** sync word, `0xe9ca` sent twice, at 10 kbit/s — and that is
//! exactly what a capture off this band contains, byte-aligned, with the rate and
//! deviation to match. A 32-bit pattern within two bit errors matches one random
//! stream in eight million, which is more protection than several of the frames
//! this crate decodes in full. Presence is not in doubt.
//!
//! The *contents* are another matter. The bytes behind that sync word are
//! obfuscated by the protocol, and the one capture available here is a single
//! partial burst — nine bytes of a frame that should be longer. Its first byte is
//! not a plausible length in either bit sense, which is as far as the evidence
//! goes. Deriving a de-obfuscation from published prose and applying it to one
//! unverifiable frame would produce sender and receiver addresses that look
//! authoritative and might be nonsense, and a device address is precisely the
//! sort of field that is worse wrong than absent.
//!
//! So the frame is reported with its bytes as hex and no interpretation. If more
//! captures turn up — several frames from one device would show which bytes hold
//! still, the same argument `ism_forensics` is built on — the header can be read
//! then, on evidence.

use sdroxide_types::IsmProtocol;

use super::Decoded;
use crate::slice::Phy;

/// 2-FSK at 10 kbit/s, preamble `0xaa…`, then `0xe9ca` twice.
pub const PHY: Phy = Phy {
    baud: 10_000.0,
    sync: 0xE9CA_E9CA,
    sync_bits: 32,
    // None. A 32-bit pattern within two errors matches one *random* stream in
    // eight million, which sounded ample — and on real traffic it was not. This
    // sync word is `0xe9ca` transmitted twice, and the band is full of periodic
    // signals: LoRa chirps sliced as bits repeat with a period of their own, so a
    // pattern that is itself a repeat matches them far more often than
    // randomness predicts. Three of four hits on a five-minute capture were
    // wideband bursts that are plainly not Homematic.
    //
    // An error budget exists so that one slipped bit does not cost a frame, and
    // on a device that repeats itself the cost of dropping a frame is a wait. The
    // cost of the other mistake is a home-automation device reported in a house
    // that has none. The one real capture matched the sync word exactly, so
    // nothing is being given up here that was observed to be needed.
    sync_errors: 0,
    preamble_bits: 16,
    // A BidCoS frame is a length byte, a counter, flags, a type and two
    // three-byte addresses before any payload.
    payload_min: 9,
    payload_max: 32,
    manchester: false,
};

pub const CHANNEL_HZ: f64 = 868_300_000.0;

pub fn parse(p: &[u8]) -> Option<Decoded> {
    if p.len() < 9 {
        return None;
    }
    Some(Decoded {
        protocol: IsmProtocol::Homematic,
        model: Some("BidCoS".to_string()),
        // No device identity: the addresses are inside the obfuscated part. An
        // empty string rather than an invented one, so every Homematic frame
        // collapses into a single "there is Homematic traffic here" row instead
        // of inventing a new device per burst.
        device: String::new(),
        readings: Vec::new(),
        extra: vec![("payload".to_string(), "obfuscated, not decoded".to_string())],
        encrypted: false,
        raw_hex: String::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The burst that identified this protocol. What is asserted is what the
    /// capture actually establishes — that the sync word is there, byte-aligned,
    /// with a preamble in front of it — and not anything about the payload.
    #[test]
    fn the_captured_burst_carries_the_bidcos_sync_word() {
        // As sliced, from the start of the signal.
        let burst: [u8; 17] = [
            0xA8, 0x2A, 0xAA, 0xAA, 0xE9, 0xCA, 0xE9, 0xCA, 0xF9, 0xF4, 0x99, 0x29, 0xD1, 0xC8,
            0xA2, 0x8D, 0x18,
        ];
        assert_eq!(&burst[4..8], &[0xE9, 0xCA, 0xE9, 0xCA], "the sync word, byte-aligned");
        assert_eq!(PHY.sync, 0xE9CA_E9CA, "and the same one the slicer looks for");

        // Everything after it is what this module declines to interpret.
        let d = parse(&burst[8..]).expect("a frame behind that sync word must be reported");
        assert_eq!(d.protocol, IsmProtocol::Homematic);
        assert!(d.readings.is_empty());
        assert!(d.device.is_empty(), "no address is claimed");
        assert_eq!(d.extra, vec![("payload".to_string(), "obfuscated, not decoded".to_string())]);
    }

    /// The evidence for *not* reading the header: the first byte is not a
    /// plausible frame length in either bit sense, so nothing anchors an attempt.
    #[test]
    fn the_captured_frames_first_byte_is_not_a_plausible_length() {
        let first = 0xF9u8;
        assert!(usize::from(first) > 32, "0xf9 is far longer than any BidCoS frame");
        assert!(usize::from(!first) < 9, "and its complement is shorter than the header");
    }

    /// A burst too short to hold a header is not a frame.
    #[test]
    fn a_short_burst_is_not_reported() {
        for n in 0..9 {
            assert!(parse(&vec![0xAA; n]).is_none(), "{n} bytes should not report a device");
        }
    }
}
