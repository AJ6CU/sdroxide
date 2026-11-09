// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM
//
// The wire format and AES-CTR nonce layout described below are the Meshtastic
// over-the-air protocol, derived from the upstream firmware at
// https://github.com/meshtastic/firmware (GPL-3.0-or-later). Only the
// on-the-air constants come from the firmware.
//
// Wire format of a Meshtastic LoRa frame (after CSS demodulation and CRC):
//
//   bytes  0..3:   to          (u32 LE)
//   bytes  4..7:   from        (u32 LE)
//   bytes  8..11:  packet id   (u32 LE), low 32 bits of the nonce's id
//   byte   12:     flags       hop limit (3) | want ack (1) | via MQTT (1) | hop start (3)
//   byte   13:     channel     xorHash(name) ^ xorHash(psk)
//   byte   14:     next hop
//   byte   15:     relay node
//   bytes 16..:    AES-CTR of the `Data` protobuf
//
// Nonce (16 bytes): packet id as u64 LE, sender as u32 LE, block counter as
// u32 big-endian starting at 0.

use aes::{Aes128, Aes256};
use ctr::cipher::{KeyIvInit, StreamCipher};

use crate::keys::KeySet;
use crate::protobuf::{Reader, WIRE_FIXED32, WIRE_VARINT, put_bytes, put_tag, put_varint};
use crate::protocol::*;

type Aes128Ctr = ctr::Ctr128BE<Aes128>;
type Aes256Ctr = ctr::Ctr128BE<Aes256>;

/// The 16-byte radio header.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Header {
    /// Destination; `BROADCAST_NODE` for everyone.
    pub to: u32,
    /// Sender node id.
    pub from: u32,
    /// Per-sender packet id (in the AES nonce).
    pub packet_id: u32,
    pub flags: u8,
    pub channel: u8,
    pub next_hop: u8,
    pub relay_node: u8,
}

impl Header {
    pub fn hop_limit(&self) -> u8 {
        self.flags & FLAG_HOP_LIMIT_MASK
    }

    pub fn hop_start(&self) -> u8 {
        (self.flags & FLAG_HOP_START_MASK) >> FLAG_HOP_START_SHIFT
    }

    pub fn want_ack(&self) -> bool {
        self.flags & FLAG_WANT_ACK_MASK != 0
    }

    pub fn via_mqtt(&self) -> bool {
        self.flags & FLAG_VIA_MQTT_MASK != 0
    }

    /// Read the header off the front of a frame.
    pub fn parse(frame: &[u8]) -> Option<Self> {
        let f = frame.get(..HEADER_BYTES)?;
        let le = |i: usize| u32::from_le_bytes([f[i], f[i + 1], f[i + 2], f[i + 3]]);
        Some(Self {
            to: le(0),
            from: le(4),
            packet_id: le(8),
            flags: f[12],
            channel: f[13],
            next_hop: f[14],
            relay_node: f[15],
        })
    }

    pub fn to_bytes(&self) -> [u8; HEADER_BYTES] {
        let mut b = [0u8; HEADER_BYTES];
        b[0..4].copy_from_slice(&self.to.to_le_bytes());
        b[4..8].copy_from_slice(&self.from.to_le_bytes());
        b[8..12].copy_from_slice(&self.packet_id.to_le_bytes());
        b[12] = self.flags;
        b[13] = self.channel;
        b[14] = self.next_hop;
        b[15] = self.relay_node;
        b
    }
}

/// The 16-byte AES-CTR counter block of a packet.
pub fn nonce(packet_id: u32, from: u32) -> [u8; 16] {
    let mut n = [0u8; 16];
    n[0..4].copy_from_slice(&packet_id.to_le_bytes());
    // n[4..8]: upper half of the id, zero on the air.
    n[8..12].copy_from_slice(&from.to_le_bytes());
    // n[12..16]: block counter, big-endian, starts at 0.
    n
}

/// Apply the AES-CTR keystream (encrypt and decrypt are the same). `key` is
/// 16 or 32 bytes; `None` for any other length.
pub fn aes_ctr(key: &[u8], nonce: &[u8; 16], data: &mut [u8]) -> Option<()> {
    match key.len() {
        16 => Aes128Ctr::new(key.into(), nonce.into()).apply_keystream(data),
        32 => Aes256Ctr::new(key.into(), nonce.into()).apply_keystream(data),
        _ => return None,
    }
    Some(())
}

/// What a decrypted frame carries: the `Data` protobuf.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Data {
    pub portnum: u32,
    pub payload: Vec<u8>,
    pub want_response: bool,
    pub request_id: u32,
    pub reply_id: u32,
}

impl Data {
    /// Parse the `Data` envelope. `None` when it is malformed or the port
    /// number is outside the plausible range (a wrong key decrypts to noise).
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let mut r = Reader::new(buf);
        let mut d = Data::default();
        while !r.is_empty() {
            let (field, wire) = r.tag()?;
            match field {
                1 => d.portnum = r.varint()? as u32,
                2 => d.payload = r.bytes()?.to_vec(),
                3 => d.want_response = r.varint()? != 0,
                6 => d.request_id = r.fixed32()?,
                7 => d.reply_id = r.fixed32()?,
                _ => r.skip(wire)?,
            }
        }
        (d.portnum <= 1024).then_some(d)
    }

    /// Encode the envelope: port, payload, and the optional fields.
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::new();
        put_tag(&mut out, 1, WIRE_VARINT);
        put_varint(&mut out, self.portnum as u64);
        put_bytes(&mut out, 2, &self.payload);
        if self.want_response {
            put_tag(&mut out, 3, WIRE_VARINT);
            put_varint(&mut out, 1);
        }
        if self.request_id != 0 {
            put_tag(&mut out, 6, WIRE_FIXED32);
            out.extend_from_slice(&self.request_id.to_le_bytes());
        }
        if self.reply_id != 0 {
            put_tag(&mut out, 7, WIRE_FIXED32);
            out.extend_from_slice(&self.reply_id.to_le_bytes());
        }
        out
    }
}

/// A received frame, header parsed and, when a key fit, decrypted.
#[derive(Debug, Clone, PartialEq)]
pub struct Packet {
    pub header: Header,
    /// A key decrypted the payload (or there was nothing to decrypt).
    pub decrypted: bool,
    /// Name of the key that matched; empty when none did.
    pub channel_name: String,
    /// The decrypted envelope, when `decrypted` and there was a payload.
    pub data: Option<Data>,
    /// The bytes after the header, as received.
    pub ciphertext: Vec<u8>,
}

/// Parse and, if a key fits, decrypt one frame. `None` when the frame is
/// shorter than the header.
pub fn decode_frame(frame: &[u8], keys: &KeySet) -> Option<Packet> {
    let header = Header::parse(frame)?;
    let cipher = &frame[HEADER_BYTES..];
    if cipher.is_empty() {
        // A header-only frame (a bare acknowledgement) has nothing to decrypt.
        return Some(Packet {
            header,
            decrypted: true,
            channel_name: String::new(),
            data: None,
            ciphertext: Vec::new(),
        });
    }
    let nonce = nonce(header.packet_id, header.from);
    for entry in keys.candidates(header.channel) {
        let mut plain = cipher.to_vec();
        if !entry.psk.is_empty() && aes_ctr(&entry.psk, &nonce, &mut plain).is_none() {
            continue;
        }
        if let Some(data) = Data::parse(&plain) {
            return Some(Packet {
                header,
                decrypted: true,
                channel_name: entry.channel_name.clone(),
                data: Some(data),
                ciphertext: cipher.to_vec(),
            });
        }
    }
    Some(Packet {
        header,
        decrypted: false,
        channel_name: String::new(),
        data: None,
        ciphertext: cipher.to_vec(),
    })
}

/// Build a frame: header, then the envelope encrypted with `psk` (empty for
/// an unencrypted channel). The header's `channel` byte is set from the
/// channel name and key.
pub fn build_frame(
    mut header: Header,
    channel_name: &str,
    psk: &[u8],
    data: &Data,
) -> Option<Vec<u8>> {
    header.channel = channel_hash(channel_name, psk);
    let mut body = data.encode();
    if !psk.is_empty() {
        aes_ctr(psk, &nonce(header.packet_id, header.from), &mut body)?;
    }
    let mut frame = header.to_bytes().to_vec();
    frame.extend_from_slice(&body);
    Some(frame)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn hello(psk: &[u8]) -> Vec<u8> {
        let h = Header {
            to: BROADCAST_NODE,
            from: 0xDEAD_BEEF,
            packet_id: 0x1234_5678,
            flags: 0x07,
            ..Header::default()
        };
        let d = Data { portnum: port::TEXT_MESSAGE, payload: b"Hello".to_vec(), ..Data::default() };
        build_frame(h, "LongFast", psk, &d).unwrap()
    }

    #[test]
    fn the_nonce_layout_is_id_sender_counter() {
        let n = nonce(0x1234_5678, 0xDEAD_BEEF);
        assert_eq!(n, [0x78, 0x56, 0x34, 0x12, 0, 0, 0, 0, 0xEF, 0xBE, 0xAD, 0xDE, 0, 0, 0, 0]);
    }

    #[test]
    fn the_header_parses_and_serialises() {
        let f = hello(&DEFAULT_PSK);
        let h = Header::parse(&f).unwrap();
        assert_eq!(h.to, BROADCAST_NODE);
        assert_eq!(h.from, 0xDEAD_BEEF);
        assert_eq!(h.packet_id, 0x1234_5678);
        assert_eq!(h.channel, 0x08);
        assert_eq!(h.hop_limit(), 7);
        assert_eq!(h.hop_start(), 0);
        assert!(!h.want_ack() && !h.via_mqtt());
        assert_eq!(&h.to_bytes()[..], &f[..HEADER_BYTES]);
    }

    #[test]
    fn flags_unpack() {
        let h = Header { flags: 0b1011_1011, ..Header::default() };
        assert_eq!(h.hop_limit(), 3);
        assert!(h.want_ack());
        assert!(h.via_mqtt());
        assert_eq!(h.hop_start(), 5);
    }

    #[test]
    fn a_default_channel_frame_decrypts_to_the_text() {
        let f = hello(&DEFAULT_PSK);
        let p = decode_frame(&f, &KeySet::with_defaults()).unwrap();
        assert!(p.decrypted);
        assert_eq!(p.channel_name, "LongFast");
        let d = p.data.unwrap();
        assert_eq!(d.portnum, port::TEXT_MESSAGE);
        assert_eq!(d.payload, b"Hello");
    }

    #[test]
    fn the_ciphertext_is_not_the_plaintext() {
        let f = hello(&DEFAULT_PSK);
        assert!(!f[HEADER_BYTES..].windows(5).any(|w| w == b"Hello"));
    }

    #[test]
    fn the_wrong_key_leaves_the_frame_encrypted() {
        let f = hello(&DEFAULT_PSK);
        let mut keys = KeySet::new();
        // Same channel hash, different key: the dispatch picks it, the
        // envelope check rejects the noise.
        let mut other = DEFAULT_PSK;
        other[0] ^= 0x01;
        other[1] ^= 0x01; // keeps the xor-hash, so the bucket still matches
        keys.add("LongFast", &other).unwrap();
        let p = decode_frame(&f, &keys).unwrap();
        assert!(!p.decrypted);
        assert!(p.data.is_none());
        assert_eq!(p.ciphertext, f[HEADER_BYTES..]);
    }

    #[test]
    fn aes_256_keys_work_too() {
        let key: Vec<u8> = (0..32u8).collect();
        let f = hello(&key);
        let mut keys = KeySet::new();
        keys.add("LongFast", &key).unwrap();
        let p = decode_frame(&f, &keys).unwrap();
        assert_eq!(p.data.unwrap().payload, b"Hello");
    }

    #[test]
    fn an_unencrypted_channel_needs_no_key_bytes() {
        let f = hello(&[]);
        let mut keys = KeySet::new();
        keys.add("LongFast", &[]).unwrap();
        assert_eq!(decode_frame(&f, &keys).unwrap().data.unwrap().payload, b"Hello");
    }

    #[test]
    fn a_header_only_frame_is_reported_as_such() {
        let h = Header { from: 1, to: 2, ..Header::default() };
        let p = decode_frame(&h.to_bytes(), &KeySet::new()).unwrap();
        assert!(p.decrypted && p.data.is_none());
    }

    #[test]
    fn a_short_frame_is_refused() {
        assert!(decode_frame(&[0u8; 15], &KeySet::new()).is_none());
    }

    #[test]
    fn the_envelope_round_trips_with_its_optional_fields() {
        let d = Data {
            portnum: port::ROUTING,
            payload: vec![1, 2, 3],
            want_response: true,
            request_id: 0xAABB_CCDD,
            reply_id: 7,
        };
        assert_eq!(Data::parse(&d.encode()), Some(d));
    }

    #[test]
    fn an_implausible_port_is_not_an_envelope() {
        let d = Data { portnum: 5000, payload: vec![], ..Data::default() };
        assert_eq!(Data::parse(&d.encode()), None);
    }
}
