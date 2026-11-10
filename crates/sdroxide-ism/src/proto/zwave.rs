//! Z-Wave on the European 868.42 MHz channel, at the 9.6 kbit/s "R1" rate.
//!
//! # Source
//!
//! Unlike every other module here, this one was not transcribed from anybody's
//! write-up. The layout below was **derived from a capture taken off the air**
//! with `ism_forensics`, and then confirmed the only way that means anything: the
//! frame's own checksum reproduces. See the test.
//!
//! The public description of the physical layer — 868.42 MHz, 9.6 kbit/s,
//! Manchester-coded FSK, a preamble of alternating bits and an `0xf0`
//! start-of-frame delimiter — is ITU-T G.9959, which is what Z-Wave's PHY and MAC
//! were standardised as.
//!
//! # Why the routing header is readable and the payload may not be
//!
//! Z-Wave encrypts at the application layer, not the link layer. A secured
//! network still puts the home identifier, both node numbers and the frame type
//! in clear on the air, because the mesh has to route the frame without opening
//! it. So a report here always says *which node spoke to which*, and says what
//! the message was only when the command class above it is not one of the
//! encrypted ones. That is the same bargain as an encrypted meter: the inventory
//! is available, the contents are not, and sdroxide holds no keys.
//!
//! # Frame
//!
//! ```text
//! f0 HH HH HH HH SS FF FF LL DD <payload …> CK
//!  |  |           |  |     |  |             `- XOR of every byte before it,
//!  |  |           |  |     |  |                starting from 0xff
//!  |  |           |  |     |  `--------------- destination node
//!  |  |           |  |     `------------------ length of everything from HH to CK
//!  |  |           |  `------------------------ frame control; the low nibble of
//!  |  |           |                            the second byte is the header type
//!  |  |           `--------------------------- source node
//!  |  `--------------------------------------- home identifier, 32 bits
//!  `------------------------------------------ start of frame
//! ```
//!
//! # Why an eight-bit sync word is enough here
//!
//! `0xf0` on its own would match one noise byte in 256, which everywhere else in
//! this crate would be far too thin. It is not carrying the frame alone: the
//! length field has to agree with the number of bytes the burst actually held,
//! the checksum is another eight bits, the node numbers have to be in range, and
//! the whole thing has to have survived Manchester decoding at one of two pair
//! alignments — a stream that is not Manchester almost never does. Together that
//! is a good deal more than the sixteen bits the weather sensors get by on.

use sdroxide_types::IsmProtocol;

use super::Decoded;
use crate::slice::Phy;

/// Manchester at 9.6 kbit/s, so 19 200 chips a second on the air.
///
/// `baud` is the chip rate because that is what the symbol-rate estimator
/// measures — a capture of this traffic reads 19 191, which is how the rate was
/// identified in the first place.
pub const PHY: Phy = Phy {
    baud: 19_200.0,
    sync: 0xF0,
    sync_bits: 8,
    // Zero: eight bits is already the thinnest sync word in the crate, and an
    // error budget on top would double the false-match rate for nothing. The
    // preamble here is long, so the delimiter arrives well after the slicer has
    // settled and does not need the slack the sensor protocols do.
    sync_errors: 0,
    preamble_bits: 16,
    // Ten bytes is the shortest frame the layout allows: four of home
    // identifier, source, two of frame control, length, destination, checksum.
    payload_min: 10,
    payload_max: 64,
    manchester: true,
};

pub const CHANNEL_HZ: f64 = 868_420_000.0;

/// Command classes worth naming.
///
/// These are the public Z-Wave command-class identifiers. Only the first was
/// confirmed against the capture — a node repeating `98 40` every thirty seconds
/// is a secured device asking the controller for a nonce, which is exactly what
/// that pair means — and the rest are transcribed. A wrong name here mislabels a
/// row; it cannot produce a wrong measurement, and anything not listed is
/// reported as its hex value rather than guessed at.
const CLASSES: &[(u8, &str)] = &[
    (0x98, "security"),
    (0x9F, "security 2"),
    (0x20, "basic"),
    (0x25, "switch"),
    (0x31, "sensor"),
    (0x32, "meter"),
    (0x71, "notification"),
    (0x80, "battery"),
    (0x84, "wake up"),
    (0x86, "version"),
];

/// Header types, from the low nibble of the second frame-control byte.
fn header_type(fc: u8) -> Option<&'static str> {
    match fc & 0x0F {
        1 => Some("singlecast"),
        2 => Some("multicast"),
        3 => Some("ack"),
        // A value the capture never showed and nothing here documents. Refused
        // rather than named — the same rule the WH57's state nibble follows.
        _ => None,
    }
}

pub fn parse(p: &[u8]) -> Option<Decoded> {
    if p.len() < 10 {
        return None;
    }
    let len = usize::from(p[7]);
    // The length field has to describe a frame this burst could actually have
    // held. This is the check that carries most of the weight — see the module
    // note on the eight-bit sync word.
    if !(10..=64).contains(&len) || len > p.len() {
        return None;
    }

    // Checksum: XOR of the frame from the home identifier up to but not
    // including the checksum byte, starting from 0xff.
    let want = p[..len - 1].iter().fold(0xFFu8, |a, &b| a ^ b);
    if want != p[len - 1] {
        return None;
    }

    let kind = header_type(p[6])?;
    let home = format!("{:02x}{:02x}{:02x}{:02x}", p[0], p[1], p[2], p[3]);
    let (src, dst) = (p[4], p[8]);
    // Node numbers are 1..232 in a Z-Wave network; 0 is not a node and 233 and
    // up are reserved. A frame naming one is a frame that was read wrongly.
    if !(1..=232).contains(&src) || !(1..=232).contains(&dst) {
        return None;
    }

    let mut extra = vec![
        ("route".to_string(), format!("node {src} → {dst}")),
        ("type".to_string(), kind.to_string()),
    ];

    // The command class, where there is a payload to hold one.
    if len >= 11 {
        let class = p[9];
        extra.push((
            "command".to_string(),
            match CLASSES.iter().find(|(c, _)| *c == class) {
                Some((_, name)) => (*name).to_string(),
                None => format!("class {class:#04x}"),
            },
        ));
        if matches!(class, 0x98 | 0x9F) {
            extra.push(("payload".to_string(), "encrypted".to_string()));
        }
    }

    Some(Decoded {
        protocol: IsmProtocol::ZWave,
        // The home identifier names the *network*; the source node is the
        // device. Both are needed to tell two networks' node 3 apart.
        model: Some(format!("node {src}")),
        device: home,
        // Nothing here is a physical quantity. A Z-Wave frame carries state and
        // routing, not measurements — the sensor readings live inside command
        // classes this module does not open.
        readings: Vec::new(),
        extra,
        encrypted: len >= 11 && matches!(p[9], 0x98 | 0x9F),
        raw_hex: String::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(hex: &str) -> Vec<u8> {
        hex.split_whitespace().map(|b| u8::from_str_radix(b, 16).unwrap()).collect()
    }

    /// The frame that identified this protocol, taken off the air at 868.42 MHz
    /// and Manchester-decoded by hand. Everything about it hangs together: the
    /// length field matches the bytes present, the checksum reproduces, the node
    /// numbers are in range, and the command class is the one a secured node
    /// repeats every thirty seconds.
    #[test]
    fn the_captured_frame_reproduces_its_own_checksum() {
        let f = frame("db b7 9d 8e 11 41 01 0c 01 98 40 04");
        assert_eq!(f.len(), 12, "the length byte says twelve");

        let d = parse(&f).expect("the captured frame did not decode");
        assert_eq!(d.protocol, IsmProtocol::ZWave);
        assert_eq!(d.device, "dbb79d8e", "the home identifier");
        assert_eq!(d.model.as_deref(), Some("node 17"));
        let has = |k: &str, v: &str| d.extra.iter().any(|(a, b)| a == k && b == v);
        assert!(has("route", "node 17 → 1"), "{:?}", d.extra);
        assert!(has("type", "singlecast"));
        assert!(has("command", "security"));
        assert!(d.encrypted, "a security command class means the payload is not readable");
    }

    /// The checksum is most of what stands behind an eight-bit sync word, so it
    /// has to bite on every single-byte change.
    #[test]
    fn every_corrupted_byte_is_rejected() {
        let good = frame("db b7 9d 8e 11 41 01 0c 01 98 40 04");
        assert!(parse(&good).is_some());
        for i in 0..good.len() {
            let mut bad = good.clone();
            bad[i] ^= 0x01;
            assert!(parse(&bad).is_none(), "a flipped bit in byte {i} was accepted");
        }
    }

    /// A length field that does not match the burst, and node numbers that
    /// cannot exist, both have to reject — they are doing as much work as the
    /// checksum here.
    #[test]
    fn an_impossible_length_or_node_number_is_refused() {
        let good = frame("db b7 9d 8e 11 41 01 0c 01 98 40 04");

        // Longer than the bytes that arrived.
        let mut bad = good.clone();
        bad[7] = 40;
        assert!(parse(&bad).is_none(), "a length past the end of the burst was accepted");

        // Node 0 does not exist. Repair the checksum so only the range check can
        // catch it.
        let mut bad = good.clone();
        bad[4] = 0;
        let n = bad.len();
        bad[n - 1] = bad[..n - 1].iter().fold(0xFFu8, |a, &b| a ^ b);
        assert!(parse(&bad).is_none(), "node 0 was accepted");

        // And an undocumented header type is refused rather than named.
        let mut bad = good.clone();
        bad[6] = 0x07;
        let n = bad.len();
        bad[n - 1] = bad[..n - 1].iter().fold(0xFFu8, |a, &b| a ^ b);
        assert!(parse(&bad).is_none(), "an unknown header type was given a name");

        for n in 0..good.len() {
            assert!(parse(&good[..n]).is_none(), "{n} bytes should not parse");
        }
    }

    /// Noise must not pass. Eight bits of checksum, a length that has to match
    /// the burst, two node ranges and a header type between them are a good deal
    /// more than the checksum alone.
    #[test]
    fn random_frames_essentially_never_pass() {
        let mut seed = 0x853C_49E6_748F_EA9Bu64;
        let mut accepted = 0;
        for _ in 0..200_000 {
            let mut f = [0u8; 24];
            for b in f.iter_mut() {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                *b = (seed >> 33) as u8;
            }
            if parse(&f).is_some() {
                accepted += 1;
            }
        }
        assert!(accepted < 20, "{accepted} random frames were accepted as Z-Wave");
    }
}
