// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM

//! Decode Meshtastic frames from a recorded IQ file.
//!
//! ```text
//! cargo run -p sdroxide-meshtastic --example decode_iq -- \
//!     capture.cf32 --rate 250000 --preset LongFast --keys default
//! ```
//!
//! The file is raw interleaved complex samples, `cf32` (little-endian `f32`
//! pairs, the default) or `cs8` (signed bytes). The sample rate must be a whole
//! multiple of the preset's bandwidth and the channel must be centred on it.

use std::io::Read;

use num_complex::Complex32;
use sdroxide_meshtastic::protocol::{DEFAULT_PRESET, lookup_preset, port, port_name};
use sdroxide_meshtastic::{Decoded, DecoderConfig, KeySet, LoraDecoder, decode_frame};

fn main() {
    let mut path = None;
    let mut rate = 250_000u32;
    let mut preset = DEFAULT_PRESET.name.to_string();
    let mut keys = String::from("default");
    let mut format = String::from("cf32");
    let mut center = 0.0f64;
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        let mut value = |name: &str| {
            args.next().unwrap_or_else(|| {
                eprintln!("{name} needs a value");
                std::process::exit(2);
            })
        };
        match a.as_str() {
            "--rate" => {
                rate = value("--rate").parse().expect("--rate is a number of samples per second")
            }
            "--preset" => preset = value("--preset"),
            "--keys" => keys = value("--keys"),
            "--format" => format = value("--format"),
            "--center" => {
                center = value("--center").parse().expect("--center is a frequency in hertz")
            }
            "-h" | "--help" => {
                eprintln!(
                    "usage: decode_iq FILE [--rate HZ] [--preset NAME] [--keys SPEC,..] [--format cf32|cs8] [--center HZ]"
                );
                return;
            }
            _ => path = Some(a),
        }
    }
    let Some(path) = path else {
        eprintln!(
            "usage: decode_iq FILE [--rate HZ] [--preset NAME] [--keys SPEC,..] [--format cf32|cs8] [--center HZ]"
        );
        std::process::exit(2);
    };

    let Some(p) = lookup_preset(&preset) else {
        eprintln!("unknown preset {preset:?}");
        std::process::exit(2);
    };
    if !rate.is_multiple_of(p.bw_hz) {
        eprintln!("the rate {rate} is not a multiple of the {} Hz bandwidth", p.bw_hz);
        std::process::exit(2);
    }
    let mut keyset = KeySet::new();
    let accepted = keyset.add_csv(&keys);
    eprintln!(
        "{} {}: SF{} CR4/{} BW{} Hz, {accepted} key(s)",
        p.name, p.channel_name, p.sf, p.cr, p.bw_hz
    );

    let mut dec = LoraDecoder::new(DecoderConfig {
        sf: p.sf,
        cr: p.cr,
        bw_hz: p.bw_hz,
        os_factor: (rate / p.bw_hz) as usize,
        center_freq_hz: center,
    })
    .expect("the preset is a valid LoRa configuration");

    let mut file = std::fs::File::open(&path).unwrap_or_else(|e| {
        eprintln!("{path}: {e}");
        std::process::exit(1);
    });
    let width = if format == "cs8" { 2 } else { 8 };
    let mut buf = vec![0u8; width * 65_536];
    let mut carry = 0usize;
    let mut frames = 0usize;
    loop {
        let n = file.read(&mut buf[carry..]).expect("read");
        if n == 0 {
            break;
        }
        let have = carry + n;
        let whole = have / width * width;
        let iq: Vec<Complex32> = buf[..whole]
            .chunks_exact(width)
            .map(|c| {
                if width == 2 {
                    Complex32::new(c[0] as i8 as f32 / 128.0, c[1] as i8 as f32 / 128.0)
                } else {
                    Complex32::new(
                        f32::from_le_bytes([c[0], c[1], c[2], c[3]]),
                        f32::from_le_bytes([c[4], c[5], c[6], c[7]]),
                    )
                }
            })
            .collect();
        buf.copy_within(whole..have, 0);
        carry = have - whole;
        for f in dec.feed(&iq) {
            frames += 1;
            show(frames, &f.payload, f.crc_ok, f.snr_db, f.cfo_bins, &keyset);
        }
    }
    let s = dec.stats();
    eprintln!(
        "{frames} frame(s); {} preamble(s), header {}/{} ok, crc {}/{} ok",
        s.preamble_locks,
        s.header_pass,
        s.header_pass + s.header_fail,
        s.crc_pass,
        s.crc_pass + s.crc_fail
    );
}

fn show(n: usize, frame: &[u8], crc_ok: bool, snr_db: f32, cfo_bins: f32, keys: &KeySet) {
    print!(
        "#{n} {} bytes, crc {}, snr {snr_db:.1} dB, cfo {cfo_bins:+.2} bins",
        frame.len(),
        if crc_ok { "ok" } else { "BAD" }
    );
    match decode_frame(frame, keys) {
        None => println!(": too short for a Meshtastic header"),
        Some(p) => {
            print!(
                ": !{:08x} -> {}",
                p.header.from,
                if p.header.to == 0xFFFF_FFFF {
                    "all".to_string()
                } else {
                    format!("!{:08x}", p.header.to)
                }
            );
            match p.data {
                Some(d) => {
                    print!(" [{}] {}", p.channel_name, port_name(d.portnum));
                    match d.decode() {
                        Some(Decoded::Text(t)) => print!(": {t:?}"),
                        Some(v) => print!(": {v:?}"),
                        None if d.portnum == port::TEXT_MESSAGE => {}
                        None => print!(" ({} payload bytes, not decoded)", d.payload.len()),
                    }
                    println!();
                }
                None if p.decrypted => println!(" (header only)"),
                None => println!(
                    " (encrypted, no matching key, channel hash 0x{:02x})",
                    p.header.channel
                ),
            }
        }
    }
}
