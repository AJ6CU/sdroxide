//! Write a synthetic 868 MHz band as an IQ file the radio can tune: several
//! LaCrosse IT+ sensors on 868.3 MHz, each repeating with a known reading.
//!
//! For exercising the ISM window, the waterfall labels and the whole engine path
//! without waiting for a real sensor to wake up — and, more to the point, for
//! having devices whose temperature and humidity are *known*, since on the air
//! the only way to find out what a sensor is reading is to read it off this same
//! decoder.
//!
//! ```text
//! cargo run --release -p sdroxide-ism --example ism_iq -- band.iq [seconds]
//! cargo run --release -- --file band.iq --rate 2025000 --freq 868880000 --mode NFM
//! ```
//!
//! Then switch the ISM window on. The rate and centre above are the RX-888's VHF
//! numbers, so the synthetic band behaves like the real one.
//!
//! Interleaved little-endian `complex64` (two `f32`s per sample), which is what
//! sdroxide's file source reads. The file loops, so a few seconds is plenty.

use std::io::{BufWriter, Write};

/// The RX-888's VHF window: what its wideband downconverter delivers.
const RATE: f64 = 2_025_000.0;
/// The centre that fits the whole channel plan inside that window's flat part.
const CENTER_HZ: f64 = 868_880_000.0;
const CHANNEL_HZ: f64 = 868_300_000.0;

const BAUD: f64 = 17_241.0;
const DEVIATION_HZ: f64 = 50_000.0;
/// Burst amplitude. Well clear of the noise below, so the capture exercises the
/// decoder rather than its sensitivity limit — `ism_replay --threshold` is the
/// place to explore that.
const AMP: f32 = 0.05;
const NOISE: f32 = 0.002;

/// The sensors in the synthetic band: (id, tenths of a degree + 400, humidity).
///
/// The temperature field is three BCD digits of `(°C + 40) × 10`, so 24.1 °C is
/// 641 — the encoding is built here from the specification rather than borrowed
/// from the decoder, which is the point of a test signal.
const SENSORS: &[(u8, u32, u8, bool)] = &[
    // id, bcd, humidity %, new-battery flag
    (26, 641, 34, true),    // 24.1 °C — the published capture, for cross-checking
    (10, 448, 0x6A, false), // 4.8 °C, no humidity element (a TX29)
    (7, 213, 88, false),    // −18.7 °C, a freezer sensor
    (41, 795, 41, false),   // 39.5 °C
];

/// Fine Offset sensors, as raw frames.
///
/// These are the captures published with the protocol documentation, replayed
/// byte for byte: a WH51 soil probe reading 36 % and a WH24 outdoor array reading
/// 11.8 °C, 78 %RH, wind from 266° and 22.2 mm of rain. Using real frames rather
/// than synthesised ones means the generator cannot quietly agree with a mistake
/// in the decoder's idea of the layout.
const FINEOFFSET: &[(&str, &[u8])] = &[
    (
        "WH51 soil, 36 %, id 006b58",
        &[0x51, 0x00, 0x6B, 0x58, 0x6E, 0x7F, 0x24, 0xF8, 0xD2, 0xFF, 0xFF, 0xFF, 0x3C, 0x28],
    ),
    (
        "WH24 array, 11.8 C, 78 %RH, 266 deg, 22.2 mm, id bf",
        &[
            0x24, 0xBF, 0x0A, 0xE2, 0x06, 0x4E, 0x08, 0x02, 0x00, 0x4A, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x8F, 0x07,
        ],
    ),
];

/// How often each sensor speaks, in seconds. Real IT+ sensors are on a 48-second
/// cycle; this is faster so a few seconds of file shows something.
const PERIOD_S: f64 = 2.0;

fn main() {
    let mut args = std::env::args().skip(1);
    let path = args.next().unwrap_or_else(|| {
        eprintln!("usage: ism_iq <band.iq> [seconds]");
        std::process::exit(2);
    });
    let secs: f64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(10.0);

    let total = (secs * RATE) as usize;
    let mut buf: Vec<(f32, f32)> = vec![(0.0, 0.0); total];

    let mut seed = 0x2545_F491_4F6C_DD1Du64;
    let mut rng = move || {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        ((seed >> 11) as f64 / (1u64 << 53) as f64) as f32
    };
    for s in buf.iter_mut() {
        // Box-Muller, so the floor really is Gaussian noise and the gate's
        // estimator sees the distribution it was built for.
        let (u1, u2) = (rng().max(1e-9), rng());
        let r = (-2.0 * (u1 as f64).ln()).sqrt() as f32 * NOISE;
        *s = (r * (std::f32::consts::TAU * u2).cos(), r * (std::f32::consts::TAU * u2).sin());
    }

    // Every transmitter, as the bytes that go on the air. The Fine Offset
    // sensors share this channel and this physical layer, so they ride the same
    // cycle as the LaCrosse ones.
    let mut frames: Vec<Vec<u8>> =
        SENSORS.iter().map(|&(id, bcd, hum, nb)| frame_bytes(id, bcd, hum, nb).to_vec()).collect();
    frames.extend(FINEOFFSET.iter().map(|(_, f)| f.to_vec()));

    let offset = CHANNEL_HZ - CENTER_HZ;
    let mut placed = 0usize;
    for (k, frame) in frames.iter().enumerate() {
        // One slot each, out of the whole cycle. Two transmissions on top of each
        // other is a real situation on this band — and they destroy each other,
        // which is why the only test signal must not be built that way.
        let slot = PERIOD_S * k as f64 / frames.len() as f64;
        let mut t = slot;
        while t < secs {
            if write_burst(&mut buf, (t * RATE) as usize, frame, offset) {
                placed += 1;
            }
            t += PERIOD_S;
        }
    }

    let f = std::fs::File::create(&path).unwrap_or_else(|e| panic!("cannot write {path}: {e}"));
    let mut w = BufWriter::new(f);
    for (re, im) in &buf {
        w.write_all(&re.to_le_bytes()).expect("write");
        w.write_all(&im.to_le_bytes()).expect("write");
    }
    w.flush().expect("flush");

    println!("{path}: {secs} s at {:.3} Msps centred on {:.4} MHz", RATE / 1e6, CENTER_HZ / 1e6);
    println!(
        "{} sensors, {placed} transmissions on {:.3} MHz",
        SENSORS.len() + FINEOFFSET.len(),
        CHANNEL_HZ / 1e6
    );
    for &(id, bcd, humidity, _) in SENSORS {
        let t = bcd as f64 / 10.0 - 40.0;
        if humidity == 0x6A {
            println!("  LaCrosse id {id:2}  {t:5.1} °C  (no humidity element)");
        } else {
            println!("  LaCrosse id {id:2}  {t:5.1} °C  {humidity} %RH");
        }
    }
    for (what, _) in FINEOFFSET {
        println!("  {what}");
    }
}

/// Build the five bytes after the sync word, per the TX29/TX35 frame layout.
fn frame_bytes(id: u8, bcd: u32, humidity: u8, new_batt: bool) -> [u8; 5] {
    let d = [(bcd / 100) as u8, ((bcd / 10) % 10) as u8, (bcd % 10) as u8];
    // Length nibble 9, then the top four bits of the six-bit id.
    let b0 = 0x90 | (id >> 2);
    // The id's low two bits, the new-battery flag, one unused bit, then the first
    // temperature digit.
    let b1 = ((id & 0x03) << 6) | (u8::from(new_batt) << 5) | d[0];
    let b2 = (d[1] << 4) | d[2];
    let b3 = humidity & 0x7F;
    let mut out = [b0, b1, b2, b3, 0];
    out[4] = crc8(&out[..4]);
    out
}

/// CRC-8, poly 0x31, init 0, no final xor — stated here independently of the
/// decoder's own copy so a test signal cannot inherit its bugs.
fn crc8(data: &[u8]) -> u8 {
    let mut crc = 0u8;
    for &b in data {
        crc ^= b;
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 { (crc << 1) ^ 0x31 } else { crc << 1 };
        }
    }
    crc
}

/// Add one 2-FSK transmission into `buf` at sample `at`. Returns false if it
/// would not fit.
fn write_burst(buf: &mut [(f32, f32)], at: usize, payload: &[u8], offset_hz: f64) -> bool {
    let mut bits: Vec<u8> = (0..48).map(|i| (i % 2) as u8).collect();
    for i in (0..16).rev() {
        bits.push(((0x2DD4u32 >> i) & 1) as u8);
    }
    for &b in payload {
        for i in (0..8).rev() {
            bits.push((b >> i) & 1);
        }
    }

    let sps = RATE / BAUD;
    let len = (bits.len() as f64 * sps).ceil() as usize;
    if at + len >= buf.len() {
        return false;
    }

    let mut phase = 0.0f64;
    let mut n = at;
    for (i, &b) in bits.iter().enumerate() {
        let f = offset_hz + if b != 0 { DEVIATION_HZ } else { -DEVIATION_HZ };
        let count = (((i + 1) as f64 * sps).round() - (i as f64 * sps).round()) as usize;
        for _ in 0..count {
            phase += std::f64::consts::TAU * f / RATE;
            buf[n].0 += AMP * phase.cos() as f32;
            buf[n].1 += AMP * phase.sin() as f32;
            n += 1;
        }
    }
    true
}
