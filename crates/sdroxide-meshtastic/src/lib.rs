// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM

//! Meshtastic over LoRa.
//!
//! * [`lora`]: the bit-level LoRa coding (whitening, Gray, Hamming,
//!   interleaver, CRC, header checksum).
//! * [`demod`]: a chirp-spread-spectrum demodulator that turns baseband IQ at
//!   a multiple of the bandwidth into LoRa frames.
//! * [`modulate`]: the transmit chain, payload bytes to IQ.
//! * [`packet`]: the 16-byte radio header, AES-CTR channel encryption and the
//!   `Data` envelope.
//! * [`keys`]: channel keys and the channel-hash dispatch.
//! * [`decoders`]: what a decrypted payload says, one decoder per port.
//! * [`protocol`]: presets, regions, port numbers and the default key.
//! * [`protobuf`]: a small protobuf reader and writer.
//!
//! Nothing here touches a radio: it takes and gives samples and bytes.

pub mod decoders;
pub mod demod;
pub mod keys;
pub mod lora;
pub mod modulate;
pub mod packet;
pub mod protobuf;
pub mod protocol;

pub use decoders::{Decoded, decode_payload};
pub use demod::{DecoderConfig, DecoderStats, LoraDecoder, LoraFrame};
pub use keys::{KeyEntry, KeyError, KeySet};
pub use modulate::{TxParams, modulate};
pub use packet::{Data, Header, Packet, decode_frame};
