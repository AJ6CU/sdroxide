// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM
//
// Minimal protobuf wire-format reader (and the few writers the transmit side
// needs). It reads what Meshtastic puts on the air without a code generator.

/// Wire types of the protobuf encoding.
pub const WIRE_VARINT: u32 = 0;
pub const WIRE_FIXED64: u32 = 1;
pub const WIRE_LENGTH: u32 = 2;
pub const WIRE_FIXED32: u32 = 5;

/// A forward cursor over a protobuf message.
#[derive(Debug, Clone, Copy)]
pub struct Reader<'a> {
    buf: &'a [u8],
}

impl<'a> Reader<'a> {
    pub fn new(buf: &'a [u8]) -> Self {
        Self { buf }
    }

    /// True once every byte has been consumed.
    pub fn is_empty(&self) -> bool {
        self.buf.is_empty()
    }

    pub fn varint(&mut self) -> Option<u64> {
        let mut v = 0u64;
        let mut shift = 0u32;
        for (i, &b) in self.buf.iter().enumerate() {
            v |= ((b & 0x7f) as u64) << shift;
            if b & 0x80 == 0 {
                self.buf = &self.buf[i + 1..];
                return Some(v);
            }
            shift += 7;
            if shift > 63 {
                return None;
            }
        }
        None
    }

    /// The next field key: `(field number, wire type)`.
    pub fn tag(&mut self) -> Option<(u32, u32)> {
        let t = self.varint()?;
        Some(((t >> 3) as u32, (t & 7) as u32))
    }

    pub fn fixed32(&mut self) -> Option<u32> {
        let b = self.buf.get(..4)?;
        let v = u32::from_le_bytes([b[0], b[1], b[2], b[3]]);
        self.buf = &self.buf[4..];
        Some(v)
    }

    pub fn fixed64(&mut self) -> Option<u64> {
        let b = self.buf.get(..8)?;
        let v = u64::from_le_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
        self.buf = &self.buf[8..];
        Some(v)
    }

    /// A length-delimited value (bytes, string or embedded message).
    pub fn bytes(&mut self) -> Option<&'a [u8]> {
        let len = self.varint()? as usize;
        let b = self.buf.get(..len)?;
        self.buf = &self.buf[len..];
        Some(b)
    }

    /// Skip a value of the given wire type.
    pub fn skip(&mut self, wire: u32) -> Option<()> {
        match wire {
            WIRE_VARINT => self.varint().map(|_| ()),
            WIRE_FIXED64 => self.fixed64().map(|_| ()),
            WIRE_LENGTH => self.bytes().map(|_| ()),
            WIRE_FIXED32 => self.fixed32().map(|_| ()),
            _ => None,
        }
    }
}

/// Zig-zag decode of a `sint32`/`sint64` varint.
pub fn zigzag(v: u64) -> i64 {
    ((v >> 1) as i64) ^ -((v & 1) as i64)
}

/// Append a varint.
pub fn put_varint(out: &mut Vec<u8>, mut v: u64) {
    loop {
        let b = (v & 0x7f) as u8;
        v >>= 7;
        if v == 0 {
            out.push(b);
            return;
        }
        out.push(b | 0x80);
    }
}

/// Append a field key.
pub fn put_tag(out: &mut Vec<u8>, field: u32, wire: u32) {
    put_varint(out, ((field as u64) << 3) | wire as u64);
}

/// Append a length-delimited field.
pub fn put_bytes(out: &mut Vec<u8>, field: u32, bytes: &[u8]) {
    put_tag(out, field, WIRE_LENGTH);
    put_varint(out, bytes.len() as u64);
    out.extend_from_slice(bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn varints_round_trip() {
        for v in [0u64, 1, 127, 128, 300, 0x1234_5678, u64::MAX] {
            let mut b = Vec::new();
            put_varint(&mut b, v);
            assert_eq!(Reader::new(&b).varint(), Some(v));
        }
    }

    #[test]
    fn a_truncated_varint_is_refused() {
        assert_eq!(Reader::new(&[0x80, 0x80]).varint(), None);
        assert_eq!(Reader::new(&[]).varint(), None);
        // Eleven continuation bytes overflow 64 bits.
        assert_eq!(Reader::new(&[0xff; 11]).varint(), None);
    }

    #[test]
    fn a_length_past_the_end_is_refused() {
        let mut r = Reader::new(&[5, 1, 2]);
        assert_eq!(r.bytes(), None);
    }

    #[test]
    fn zigzag_matches_the_spec() {
        assert_eq!(zigzag(0), 0);
        assert_eq!(zigzag(1), -1);
        assert_eq!(zigzag(2), 1);
        assert_eq!(zigzag(3), -2);
        assert_eq!(zigzag(0xffff_fffe), 0x7fff_ffff);
    }

    #[test]
    fn fields_are_walked_in_order() {
        let mut b = Vec::new();
        put_tag(&mut b, 1, WIRE_VARINT);
        put_varint(&mut b, 67);
        put_bytes(&mut b, 2, b"hi");
        let mut r = Reader::new(&b);
        assert_eq!(r.tag(), Some((1, WIRE_VARINT)));
        assert_eq!(r.varint(), Some(67));
        assert_eq!(r.tag(), Some((2, WIRE_LENGTH)));
        assert_eq!(r.bytes(), Some(&b"hi"[..]));
        assert!(r.is_empty());
    }
}
