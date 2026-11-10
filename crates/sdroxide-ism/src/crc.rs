//! Frame checks for the ISM protocols.
//!
//! Not borrowed from elsewhere in the tree, and the reason is the same one the
//! other CRC modules here give: the parameters are what make a CRC, and these
//! are not the parameters anyone else uses. `sdroxide-ax25` has CRC-16/X.25,
//! reflected; `sdroxide-winlink` has an augmented CRC-16/CCITT; `sdroxide-dsp`
//! has CRC-32/ISO-HDLC. The cheap sensors on 868 MHz overwhelmingly use an
//! **8-bit, non-reflected** CRC — it is what a CC110x-class radio's reference
//! code shipped with — and none of those three can produce it.
//!
//! It is parameterised rather than table-driven. A burst decoder runs it over a
//! few dozen bytes a few times a second; a 256-entry table per polynomial would
//! cost more to maintain than it saves.

/// Bitwise CRC-8, MSB-first, no input or output reflection and no final xor.
///
/// `poly` is the polynomial in its normal (non-reversed) form: `0x31` for
/// `x⁸+x⁵+x⁴+1`, which is what the Fine Offset and LaCrosse sensors use, and
/// `0x07` for `x⁸+x²+x+1`.
pub fn crc8(poly: u8, init: u8, data: &[u8]) -> u8 {
    let mut crc = init;
    for &b in data {
        crc ^= b;
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 { (crc << 1) ^ poly } else { crc << 1 };
        }
    }
    crc
}

/// Bitwise CRC-16, MSB-first, no reflection and no final xor.
///
/// With `poly = 0x1021, init = 0x0000` this is CRC-16/XMODEM, which is what the
/// Bresser leakage sensor carries. None of the CRC-16s elsewhere in the tree is
/// this one: `sdroxide-ax25` reflects both directions and xors the output,
/// `sdroxide-winlink` augments the message.
pub fn crc16(poly: u16, init: u16, data: &[u8]) -> u16 {
    let mut crc = init;
    for &b in data {
        crc ^= u16::from(b) << 8;
        for _ in 0..8 {
            crc = if crc & 0x8000 != 0 { (crc << 1) ^ poly } else { crc << 1 };
        }
    }
    crc
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The anchor: both polynomials have a *published* check value — the CRC of
    /// the ASCII string `"123456789"` — in the catalogue of CRC parameters. A
    /// test that only compared this implementation against another version of
    /// itself would agree just as happily with a wrong polynomial.
    #[test]
    fn the_polynomials_reproduce_their_published_check_values() {
        let msg = b"123456789";
        // CRC-8/NRSC-5: poly 0x31, init 0xFF, no reflection, no xorout.
        assert_eq!(crc8(0x31, 0xFF, msg), 0xF7);
        // CRC-8/SMBUS: poly 0x07, init 0x00, no reflection, no xorout.
        assert_eq!(crc8(0x07, 0x00, msg), 0xF4);
        // CRC-16/XMODEM: poly 0x1021, init 0x0000, no reflection, no xorout.
        assert_eq!(crc16(0x1021, 0x0000, msg), 0x31C3);
        // And CRC-16/CCITT-FALSE, the same polynomial with init 0xFFFF, which
        // pins the init parameter as well as the polynomial.
        assert_eq!(crc16(0x1021, 0xFFFF, msg), 0x29B1);
    }

    /// The same append-and-check property, for the sixteen-bit width.
    #[test]
    fn a_message_with_its_crc16_appended_checks_to_zero() {
        for msg in [b"\x00".as_slice(), b"\xff\x00\xa5", b"hello ism", &[0u8; 32]] {
            let c = crc16(0x1021, 0x0000, msg);
            let mut with = msg.to_vec();
            with.extend_from_slice(&c.to_be_bytes());
            assert_eq!(crc16(0x1021, 0x0000, &with), 0, "over {msg:?}");
        }
    }

    /// A CRC appended to its own message leaves zero. This is the property the
    /// frame decoders actually rely on, so it is worth asserting directly
    /// rather than inferring from the check values.
    #[test]
    fn a_message_with_its_crc_appended_checks_to_zero() {
        for poly in [0x31u8, 0x07] {
            for msg in [b"\x00".as_slice(), b"\xff\x00\xa5", b"hello ism", &[0u8; 32]] {
                let c = crc8(poly, 0x00, msg);
                let mut with = msg.to_vec();
                with.push(c);
                assert_eq!(crc8(poly, 0x00, &with), 0, "poly {poly:#04x} over {msg:?}");
            }
        }
    }

    /// Every single-bit error has to be caught, or the frame gate is decorative.
    #[test]
    fn every_single_bit_flip_changes_the_crc() {
        let msg = [0x2Du8, 0xD4, 0x51, 0x12, 0x34, 0x56, 0x7A];
        let good = crc8(0x31, 0x00, &msg);
        for byte in 0..msg.len() {
            for bit in 0..8 {
                let mut bad = msg;
                bad[byte] ^= 1 << bit;
                assert_ne!(crc8(0x31, 0x00, &bad), good, "byte {byte} bit {bit} slipped through");
            }
        }
    }
}
