// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM
//
// Modulate, damage the signal the way a radio would, demodulate. The
// modulator is checked against reference vectors in `stages.rs`, so a frame
// made here is a real LoRa frame; what these tests add is the handling of
// timing, carrier offset, oversampling and noise on the receive side.

use std::f64::consts::PI;

use num_complex::Complex32;
use sdroxide_meshtastic::packet::{Data, Header, build_frame};
use sdroxide_meshtastic::protocol::{BROADCAST_NODE, DEFAULT_PSK, port};
use sdroxide_meshtastic::{
    DecoderConfig, KeySet, LoraDecoder, LoraFrame, TxParams, decode_frame, modulate,
};

/// Deterministic noise: xorshift64* into Box-Muller.
struct Noise(u64);

impl Noise {
    fn uniform(&mut self) -> f64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        let v = self.0.wrapping_mul(0x2545_F491_4F6C_DD1D);
        ((v >> 11) as f64 + 0.5) / (1u64 << 53) as f64
    }

    fn gauss(&mut self) -> f64 {
        let (u1, u2) = (self.uniform(), self.uniform());
        (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos()
    }
}

struct Channel {
    /// Samples of silence before the frame, deliberately not a symbol multiple.
    lead: usize,
    /// Carrier offset in hertz.
    cfo_hz: f64,
    /// Per-sample SNR in dB (signal power is 1); `None` for a clean channel.
    snr_db: Option<f64>,
}

impl Channel {
    fn clean() -> Self {
        Self { lead: 5000, cfo_hz: 0.0, snr_db: None }
    }
}

fn through(frame: &[Complex32], p: &TxParams, ch: &Channel) -> Vec<Complex32> {
    let fs = p.sample_rate() as f64;
    let tail = 25 * (1usize << p.sf) * p.os;
    let mut iq = vec![Complex32::new(0.0, 0.0); ch.lead];
    iq.extend_from_slice(frame);
    iq.extend(std::iter::repeat_n(Complex32::new(0.0, 0.0), tail));
    if ch.cfo_hz != 0.0 {
        for (n, s) in iq.iter_mut().enumerate() {
            let (sn, cs) = (2.0 * PI * ch.cfo_hz * n as f64 / fs).sin_cos();
            *s *= Complex32::new(cs as f32, sn as f32);
        }
    }
    if let Some(snr) = ch.snr_db {
        let sigma = (10f64.powf(-snr / 10.0) / 2.0).sqrt();
        let mut noise = Noise(0x9E37_79B9_7F4A_7C15);
        for s in iq.iter_mut() {
            *s += Complex32::new((noise.gauss() * sigma) as f32, (noise.gauss() * sigma) as f32);
        }
    }
    iq
}

fn decode(iq: &[Complex32], p: &TxParams) -> Vec<LoraFrame> {
    let mut d = LoraDecoder::new(DecoderConfig {
        sf: p.sf,
        cr: p.cr,
        bw_hz: p.bw_hz,
        os_factor: p.os,
        center_freq_hz: 0.0,
    })
    .unwrap();
    // Feed in odd-sized blocks: the decoder must not care where they end.
    iq.chunks(3001).flat_map(|c| d.feed(c)).collect()
}

fn payload(len: usize) -> Vec<u8> {
    (0..len).map(|i| (i as u8).wrapping_mul(31).wrapping_add(7)).collect()
}

fn assert_one_frame(got: &[LoraFrame], want: &[u8]) {
    assert_eq!(got.len(), 1, "frames: {got:?}");
    assert_eq!(got[0].payload, want);
    assert!(got[0].has_crc && got[0].crc_ok);
}

#[test]
fn a_clean_long_fast_frame_decodes() {
    let p = TxParams::meshtastic(11, 5, 250_000);
    let want = payload(21);
    let got = decode(&through(&modulate(&want, &p), &p, &Channel::clean()), &p);
    assert_one_frame(&got, &want);
    assert_eq!(got[0].cr, 5);
    assert_eq!(got[0].sf, 11);
}

#[test]
fn every_spreading_factor_and_coding_rate_decodes() {
    for sf in 7..=12u8 {
        for cr in 5..=8u8 {
            let bw = if sf == 12 { 125_000 } else { 250_000 };
            let p = TxParams::meshtastic(sf, cr, bw);
            for len in [2usize, 13, 60] {
                let want = payload(len);
                let got = decode(&through(&modulate(&want, &p), &p, &Channel::clean()), &p);
                assert_eq!(got.len(), 1, "sf {sf} cr {cr} len {len}: {got:?}");
                assert_eq!(got[0].payload, want, "sf {sf} cr {cr} len {len}");
                assert!(got[0].crc_ok, "sf {sf} cr {cr} len {len}");
                assert_eq!(got[0].cr, cr, "the header carries the coding rate");
            }
        }
    }
}

#[test]
fn the_low_data_rate_optimisation_decodes() {
    // SF12 at 125 kHz lasts 32.8 ms a symbol: the payload runs at sf - 2 bits.
    let p = TxParams::meshtastic(12, 8, 125_000);
    let want = payload(40);
    let got = decode(&through(&modulate(&want, &p), &p, &Channel::clean()), &p);
    assert_one_frame(&got, &want);
}

#[test]
fn a_frame_without_crc_decodes() {
    let mut p = TxParams::meshtastic(9, 5, 250_000);
    p.has_crc = false;
    let want = payload(17);
    let got = decode(&through(&modulate(&want, &p), &p, &Channel::clean()), &p);
    assert_eq!(got.len(), 1);
    assert_eq!(got[0].payload, want);
    assert!(!got[0].has_crc && got[0].crc_ok);
}

#[test]
fn every_start_offset_decodes() {
    // The frame lands anywhere against the receiver's symbol grid. A window
    // that straddles the end of the preamble by more than a third of a symbol
    // used to lock one symbol early and lose the header: 41 of 256 offsets at
    // SF8. Every offset over two symbols, at three spreading factors.
    for sf in [7u8, 8, 9] {
        let p = TxParams::meshtastic(sf, 5, 250_000);
        let want = payload(24);
        let frame = modulate(&want, &p);
        for lead in 0..2 * (1usize << sf) {
            let ch = Channel { lead, ..Channel::clean() };
            let got = decode(&through(&frame, &p, &ch), &p);
            assert_eq!(got.len(), 1, "sf {sf} lead {lead}: {got:?}");
            assert_eq!(got[0].payload, want, "sf {sf} lead {lead}");
        }
    }
}

#[test]
fn a_carrier_offset_is_absorbed() {
    let p = TxParams::meshtastic(11, 5, 250_000);
    let bin_hz = 250_000.0 / 2048.0;
    let want = payload(30);
    let frame = modulate(&want, &p);
    // Whole and fractional bins, both signs (an offset of exactly half a bin
    // splits the peak between two bins and is left out).
    for bins in [-8.0, -4.0, -2.4, -1.3, -0.4, 0.4, 1.3, 2.4, 4.0, 8.0] {
        let ch = Channel { cfo_hz: bins * bin_hz, ..Channel::clean() };
        let got = decode(&through(&frame, &p, &ch), &p);
        assert_eq!(got.len(), 1, "cfo {bins} bins: {got:?}");
        assert_eq!(got[0].payload, want, "cfo {bins} bins");
        assert!(
            (got[0].cfo_bins as f64 - bins).abs() < 1.0,
            "cfo {bins}: measured {}",
            got[0].cfo_bins
        );
    }
}

#[test]
fn oversampled_input_decodes() {
    for os in [2usize, 4] {
        let mut p = TxParams::meshtastic(9, 5, 250_000);
        p.os = os;
        let want = payload(33);
        let frame = modulate(&want, &p);
        // Every offset over a symbol, in steps that are not a divisor of it.
        for lead in (0..512 * os).step_by(7) {
            let ch = Channel { lead, ..Channel::clean() };
            let got = decode(&through(&frame, &p, &ch), &p);
            assert_eq!(got.len(), 1, "os {os} lead {lead}: {got:?}");
            assert_eq!(got[0].payload, want, "os {os} lead {lead}");
        }
    }
}

#[test]
fn noise_well_below_the_signal_is_survived() {
    // The processing gain of 2^SF puts these far under the 0 dB line.
    for (sf, snr) in [(7u8, -2.0), (9, -9.0), (11, -15.0)] {
        let p = TxParams::meshtastic(sf, 5, 250_000);
        let want = payload(20);
        let ch = Channel { snr_db: Some(snr), ..Channel::clean() };
        let got = decode(&through(&modulate(&want, &p), &p, &ch), &p);
        assert_eq!(got.len(), 1, "sf {sf} at {snr} dB: {got:?}");
        assert_eq!(got[0].payload, want, "sf {sf} at {snr} dB");
    }
}

#[test]
fn two_frames_in_a_row_both_decode() {
    let p = TxParams::meshtastic(8, 6, 250_000);
    let (a, b) = (payload(10), payload(37));
    let mut iq = through(&modulate(&a, &p), &p, &Channel::clean());
    iq.extend(through(&modulate(&b, &p), &p, &Channel { lead: 900, ..Channel::clean() }));
    let got = decode(&iq, &p);
    assert_eq!(got.len(), 2, "{got:?}");
    assert_eq!(got[0].payload, a);
    assert_eq!(got[1].payload, b);
}

#[test]
fn silence_and_noise_decode_nothing() {
    let p = TxParams::meshtastic(10, 5, 250_000);
    let silence = vec![Complex32::new(0.0, 0.0); 200_000];
    assert!(decode(&silence, &p).is_empty());

    let mut noise = Noise(42);
    let hiss: Vec<Complex32> =
        (0..400_000).map(|_| Complex32::new(noise.gauss() as f32, noise.gauss() as f32)).collect();
    assert!(decode(&hiss, &p).is_empty());
}

#[test]
fn a_decoder_tuned_to_another_spreading_factor_hears_nothing() {
    let sent = TxParams::meshtastic(11, 5, 250_000);
    let iq = through(&modulate(&payload(20), &sent), &sent, &Channel::clean());
    let heard = TxParams::meshtastic(9, 5, 250_000);
    assert!(decode(&iq, &heard).is_empty());
}

#[test]
fn a_frame_cut_short_is_not_delivered() {
    let p = TxParams::meshtastic(9, 5, 250_000);
    let frame = modulate(&payload(40), &p);
    let cut = &frame[..frame.len() - 6 * 512];
    let mut iq = vec![Complex32::new(0.0, 0.0); 3000];
    iq.extend_from_slice(cut);
    iq.extend(vec![Complex32::new(0.0, 0.0); 3000]);
    assert!(decode(&iq, &p).is_empty());
}

#[test]
fn damaged_payload_symbols_fail_the_crc() {
    let p = TxParams::meshtastic(9, 5, 250_000);
    let want = payload(30);
    let mut frame = modulate(&want, &p);
    // Replace the last two payload symbols by another frame's chirps.
    let spsym = 512;
    let n = frame.len();
    let other = modulate(&payload(1), &p);
    let junk = &other[other.len() - 2 * spsym..];
    frame[n - 2 * spsym..].copy_from_slice(junk);
    let got = decode(&through(&frame, &p, &Channel::clean()), &p);
    assert!(got.iter().all(|f| !(f.crc_ok && f.payload == want)), "{got:?}");
}

#[test]
fn a_meshtastic_text_goes_from_the_air_to_the_screen() {
    let p = TxParams::meshtastic(11, 5, 250_000);
    let header = Header {
        to: BROADCAST_NODE,
        from: 0xDEAD_BEEF,
        packet_id: 0x1234_5678,
        flags: 0x07,
        ..Header::default()
    };
    let data = Data { portnum: port::TEXT_MESSAGE, payload: b"Hello".to_vec(), ..Data::default() };
    let frame = build_frame(header, "LongFast", &DEFAULT_PSK, &data).unwrap();

    let ch = Channel { cfo_hz: 150.0, snr_db: Some(-8.0), ..Channel::clean() };
    let got = decode(&through(&modulate(&frame, &p), &p, &ch), &p);
    assert_one_frame(&got, &frame);

    let pkt = decode_frame(&got[0].payload, &KeySet::with_defaults()).unwrap();
    assert!(pkt.decrypted);
    assert_eq!(pkt.header.from, 0xDEAD_BEEF);
    assert_eq!(pkt.channel_name, "LongFast");
    let d = pkt.data.unwrap();
    assert_eq!(d.portnum, port::TEXT_MESSAGE);
    assert_eq!(d.payload, b"Hello");
}

#[test]
fn a_position_report_goes_from_the_air_to_typed_values() {
    use sdroxide_meshtastic::Decoded;
    use sdroxide_meshtastic::protobuf::{WIRE_FIXED32, WIRE_VARINT, put_tag, put_varint};

    // Latitude 48.8584, longitude 2.2945, altitude 35 m, as a node sends them.
    let mut pos = Vec::new();
    for (field, deg) in [(1, 48.8584f64), (2, 2.2945)] {
        put_tag(&mut pos, field, WIRE_FIXED32);
        pos.extend_from_slice(&((deg * 1e7).round() as i32).to_le_bytes());
    }
    put_tag(&mut pos, 3, WIRE_VARINT);
    put_varint(&mut pos, 35);

    let p = TxParams::meshtastic(11, 5, 250_000);
    let header =
        Header { to: BROADCAST_NODE, from: 0xCAFE_F00D, packet_id: 9, flags: 0x07, ..Header::default() };
    let data = Data { portnum: port::POSITION, payload: pos, ..Data::default() };
    let frame = build_frame(header, "LongFast", &DEFAULT_PSK, &data).unwrap();

    let ch = Channel { snr_db: Some(-6.0), ..Channel::clean() };
    let got = decode(&through(&modulate(&frame, &p), &p, &ch), &p);
    assert_one_frame(&got, &frame);

    let pkt = decode_frame(&got[0].payload, &KeySet::with_defaults()).unwrap();
    let Some(Decoded::Position(fix)) = pkt.data.unwrap().decode() else {
        panic!("a position was expected");
    };
    assert!((fix.lat_deg.unwrap() - 48.8584).abs() < 1e-6);
    assert!((fix.lon_deg.unwrap() - 2.2945).abs() < 1e-6);
    assert_eq!(fix.altitude_m, Some(35));
}

#[test]
fn the_decoder_counts_what_it_saw() {
    let p = TxParams::meshtastic(8, 5, 250_000);
    let iq = through(&modulate(&payload(12), &p), &p, &Channel::clean());
    let mut d = LoraDecoder::new(DecoderConfig::new(8, 5, 250_000)).unwrap();
    let frames = d.feed(&iq);
    assert_eq!(frames.len(), 1);
    let s = d.stats();
    assert_eq!((s.header_pass, s.crc_pass, s.crc_fail, s.frames), (1, 1, 0, 1));
    assert!(s.preamble_locks >= 1);
}

#[test]
fn bad_parameters_are_refused() {
    assert!(LoraDecoder::new(DecoderConfig::new(6, 5, 250_000)).is_none());
    assert!(LoraDecoder::new(DecoderConfig::new(13, 5, 250_000)).is_none());
    assert!(LoraDecoder::new(DecoderConfig::new(9, 4, 250_000)).is_none());
    assert!(LoraDecoder::new(DecoderConfig::new(9, 5, 0)).is_none());
    assert!(
        LoraDecoder::new(DecoderConfig { os_factor: 0, ..DecoderConfig::new(9, 5, 250_000) })
            .is_none()
    );
}
