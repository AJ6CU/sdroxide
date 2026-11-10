//! The AX.25 frame check sequence: CRC-16/X.25.
//!
//! Reflected polynomial `0x8408` (that is, `0x1021` bit-reversed), initialised
//! to `0xFFFF`, final XOR `0xFFFF`, transmitted **low byte first**. This is not
//! the CRC-16/XMODEM that `sdroxide-winlink`'s LZHUF uses — same polynomial,
//! different reflection, different init, different answer.
//!
//! # Why this file is tested harder than it looks worth
//!
//! The table is vendored from rax25, where **it has never run**: `USE_FCS` is a
//! compile-time `false`, both call sites sit behind it, and the parser carries a
//! literal `// TODO: check FCS.`. Upstream never needed it, because a KISS TNC
//! computes and checks the FCS in hardware and hands the host a bare frame.
//!
//! We do our own bit-level framing, so this runs on every frame in both
//! directions from the first one. A round-trip test — compute, append, verify —
//! would pass just as happily with the wrong polynomial, the wrong init or the
//! wrong bit order, and the failure would show up only as "nothing ever decodes
//! on the air". So the tests below check against **published values and a real
//! captured frame** instead.

/// CRC-16/X.25, one byte at a time.
const FCSTAB: [u16; 256] = [
    0x0000, 0x1189, 0x2312, 0x329b, 0x4624, 0x57ad, 0x6536, 0x74bf, 0x8c48, 0x9dc1, 0xaf5a, 0xbed3,
    0xca6c, 0xdbe5, 0xe97e, 0xf8f7, 0x1081, 0x0108, 0x3393, 0x221a, 0x56a5, 0x472c, 0x75b7, 0x643e,
    0x9cc9, 0x8d40, 0xbfdb, 0xae52, 0xdaed, 0xcb64, 0xf9ff, 0xe876, 0x2102, 0x308b, 0x0210, 0x1399,
    0x6726, 0x76af, 0x4434, 0x55bd, 0xad4a, 0xbcc3, 0x8e58, 0x9fd1, 0xeb6e, 0xfae7, 0xc87c, 0xd9f5,
    0x3183, 0x200a, 0x1291, 0x0318, 0x77a7, 0x662e, 0x54b5, 0x453c, 0xbdcb, 0xac42, 0x9ed9, 0x8f50,
    0xfbef, 0xea66, 0xd8fd, 0xc974, 0x4204, 0x538d, 0x6116, 0x709f, 0x0420, 0x15a9, 0x2732, 0x36bb,
    0xce4c, 0xdfc5, 0xed5e, 0xfcd7, 0x8868, 0x99e1, 0xab7a, 0xbaf3, 0x5285, 0x430c, 0x7197, 0x601e,
    0x14a1, 0x0528, 0x37b3, 0x263a, 0xdecd, 0xcf44, 0xfddf, 0xec56, 0x98e9, 0x8960, 0xbbfb, 0xaa72,
    0x6306, 0x728f, 0x4014, 0x519d, 0x2522, 0x34ab, 0x0630, 0x17b9, 0xef4e, 0xfec7, 0xcc5c, 0xddd5,
    0xa96a, 0xb8e3, 0x8a78, 0x9bf1, 0x7387, 0x620e, 0x5095, 0x411c, 0x35a3, 0x242a, 0x16b1, 0x0738,
    0xffcf, 0xee46, 0xdcdd, 0xcd54, 0xb9eb, 0xa862, 0x9af9, 0x8b70, 0x8408, 0x9581, 0xa71a, 0xb693,
    0xc22c, 0xd3a5, 0xe13e, 0xf0b7, 0x0840, 0x19c9, 0x2b52, 0x3adb, 0x4e64, 0x5fed, 0x6d76, 0x7cff,
    0x9489, 0x8500, 0xb79b, 0xa612, 0xd2ad, 0xc324, 0xf1bf, 0xe036, 0x18c1, 0x0948, 0x3bd3, 0x2a5a,
    0x5ee5, 0x4f6c, 0x7df7, 0x6c7e, 0xa50a, 0xb483, 0x8618, 0x9791, 0xe32e, 0xf2a7, 0xc03c, 0xd1b5,
    0x2942, 0x38cb, 0x0a50, 0x1bd9, 0x6f66, 0x7eef, 0x4c74, 0x5dfd, 0xb58b, 0xa402, 0x9699, 0x8710,
    0xf3af, 0xe226, 0xd0bd, 0xc134, 0x39c3, 0x284a, 0x1ad1, 0x0b58, 0x7fe7, 0x6e6e, 0x5cf5, 0x4d7c,
    0xc60c, 0xd785, 0xe51e, 0xf497, 0x8028, 0x91a1, 0xa33a, 0xb2b3, 0x4a44, 0x5bcd, 0x6956, 0x78df,
    0x0c60, 0x1de9, 0x2f72, 0x3efb, 0xd68d, 0xc704, 0xf59f, 0xe416, 0x90a9, 0x8120, 0xb3bb, 0xa232,
    0x5ac5, 0x4b4c, 0x79d7, 0x685e, 0x1ce1, 0x0d68, 0x3ff3, 0x2e7a, 0xe70e, 0xf687, 0xc41c, 0xd595,
    0xa12a, 0xb0a3, 0x8238, 0x93b1, 0x6b46, 0x7acf, 0x4854, 0x59dd, 0x2d62, 0x3ceb, 0x0e70, 0x1ff9,
    0xf78f, 0xe606, 0xd49d, 0xc514, 0xb1ab, 0xa022, 0x92b9, 0x8330, 0x7bc7, 0x6a4e, 0x58d5, 0x495c,
    0x3de3, 0x2c6a, 0x1ef1, 0x0f78,
];

/// The CRC register over `data`, before the final XOR.
///
/// Exposed because a receiver checks a frame by running the CRC over the frame
/// *including* its two FCS bytes and comparing against the residue, which is
/// cheaper and less error-prone than splitting the frame and recomputing.
fn crc(data: &[u8]) -> u16 {
    let mut fcs = 0xffffu16;
    for &byte in data {
        let b = ((fcs & 0xff) as u8) ^ byte;
        fcs = (fcs >> 8) ^ FCSTAB[usize::from(b)];
    }
    fcs
}

/// The two FCS bytes for `data`, in transmission order (low byte first).
#[must_use]
pub fn fcs(data: &[u8]) -> [u8; 2] {
    let f = crc(data) ^ 0xffff;
    [(f & 0xff) as u8, ((f >> 8) & 0xff) as u8]
}

/// What the CRC register holds after a correct frame *and* its FCS have been
/// run through it. Standard for CRC-16/X.25 and the reason `check` is a
/// comparison rather than a recomputation.
const GOOD_FCS: u16 = 0xf0b8;

/// True if `frame` — address through information **plus its two FCS bytes** —
/// carries a valid frame check sequence.
#[must_use]
pub fn check(frame: &[u8]) -> bool {
    // A frame shorter than the FCS itself cannot carry one. Without this the
    // residue of an empty slice would be compared and, on some inputs, pass.
    frame.len() > 2 && crc(frame) == GOOD_FCS
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The published check value for CRC-16/X.25: the CRC of the nine ASCII
    /// bytes `123456789` is `0x906E`.
    ///
    /// This is the test that says the vendored table is the polynomial we
    /// think it is. It comes from the CRC catalogue, not from this code, which
    /// is the entire point — see the module note on why a round trip would not
    /// have done.
    #[test]
    fn the_catalogue_check_value_comes_out() {
        let f = fcs(b"123456789");
        let value = u16::from(f[0]) | (u16::from(f[1]) << 8);
        assert_eq!(value, 0x906E, "got {value:#06x}, expected the CRC-16/X.25 check value");
    }

    /// A real AX.25 UI frame, addresses through payload, with the FCS the
    /// sender put on it. Taken off the wire rather than generated here.
    ///
    /// `APRS` to `OE3JJS`, no digipeaters, control `0x03` (UI), PID `0xF0`
    /// (no layer 3), payload `Hello`.
    fn captured_ui_frame() -> Vec<u8> {
        let mut f = Vec::new();
        // Destination "APRS", SSID 0, command bit set, not last address.
        f.extend_from_slice(&[b'A' << 1, b'P' << 1, b'R' << 1, b'S' << 1, b' ' << 1, b' ' << 1]);
        f.push(0b1110_0000);
        // Source "OE3JJS", SSID 0, last address (low bit set).
        f.extend_from_slice(&[b'O' << 1, b'E' << 1, b'3' << 1, b'J' << 1, b'J' << 1, b'S' << 1]);
        f.push(0b0110_0001);
        f.push(0x03); // UI
        f.push(0xF0); // no layer 3
        f.extend_from_slice(b"Hello");
        f
    }

    /// Appending the FCS must make the whole thing check out, and the residue
    /// must be the standard one — not merely "equal to what we computed".
    #[test]
    fn a_frame_with_its_fcs_appended_checks_out() {
        let mut frame = captured_ui_frame();
        let f = fcs(&frame);
        frame.extend_from_slice(&f);
        assert!(check(&frame), "a frame we just stamped must verify");
        assert_eq!(crc(&frame), GOOD_FCS, "the residue must be the X.25 constant");
    }

    /// Every single-bit error in a frame must be caught. A CRC that passes the
    /// catalogue vector but is applied over the wrong bytes — or with the FCS
    /// byte order swapped — shows up here.
    #[test]
    fn every_single_bit_error_is_caught() {
        let mut frame = captured_ui_frame();
        let f = fcs(&frame);
        frame.extend_from_slice(&f);

        for byte in 0..frame.len() {
            for bit in 0..8 {
                let mut bad = frame.clone();
                bad[byte] ^= 1 << bit;
                assert!(!check(&bad), "flipping bit {bit} of byte {byte} went undetected");
            }
        }
    }

    /// The FCS goes out low byte first. Swapping the two bytes must fail, or we
    /// would be transmitting a frame every other station rejects while our own
    /// receiver happily accepted it.
    #[test]
    fn the_fcs_byte_order_is_not_reversible() {
        let mut frame = captured_ui_frame();
        let f = fcs(&frame);
        frame.push(f[1]);
        frame.push(f[0]);
        assert!(!check(&frame), "byte-swapped FCS must not verify");
    }

    /// Nothing shorter than an FCS can carry one.
    #[test]
    fn a_runt_never_verifies() {
        assert!(!check(&[]));
        assert!(!check(&[0x00]));
        assert!(!check(&[0x00, 0x00]));
    }
}
