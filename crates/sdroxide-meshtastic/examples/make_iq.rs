// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM

//! Write a synthetic Meshtastic capture: one text frame on the default
//! channel, as `cf32`, for trying `decode_iq` without a radio.
//!
//! ```text
//! cargo run -p sdroxide-meshtastic --example make_iq -- out.cf32 "Hello"
//! cargo run -p sdroxide-meshtastic --example decode_iq -- out.cf32
//! ```
//!
//! This is generated, not received: it proves the chain, not an antenna.

use std::io::Write;

use sdroxide_meshtastic::packet::{Data, Header, build_frame};
use sdroxide_meshtastic::protocol::{BROADCAST_NODE, DEFAULT_PRESET, DEFAULT_PSK, port};
use sdroxide_meshtastic::{TxParams, modulate};

fn main() {
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("usage: make_iq OUT.cf32 [TEXT]");
        std::process::exit(2);
    };
    let text = args.next().unwrap_or_else(|| "Hello from sdroxide".to_string());

    let p = DEFAULT_PRESET;
    let header = Header {
        to: BROADCAST_NODE,
        from: 0xDEAD_BEEF,
        packet_id: 0x1234_5678,
        flags: 0x07,
        ..Header::default()
    };
    let data = Data { portnum: port::TEXT_MESSAGE, payload: text.into_bytes(), ..Data::default() };
    let frame = build_frame(header, p.channel_name, &DEFAULT_PSK, &data).expect("a 16-byte key");

    let tx = TxParams::meshtastic(p.sf, p.cr, p.bw_hz);
    // A little silence before the frame and a symbol-train's worth after it,
    // so the receiver sees the frame start and drains.
    let pad = 8 * (1usize << p.sf);
    let silence = vec![num_complex::Complex32::new(0.0, 0.0); pad];
    let mut out = std::fs::File::create(&path).expect("create the output file");
    let samples =
        silence.iter().cloned().chain(modulate(&frame, &tx)).chain(silence.iter().cloned());
    for s in samples {
        out.write_all(&s.re.to_le_bytes())
            .and_then(|_| out.write_all(&s.im.to_le_bytes()))
            .expect("write");
    }
    eprintln!("{} bytes of frame, {} Hz, cf32, written to {path}", frame.len(), tx.sample_rate());
}
