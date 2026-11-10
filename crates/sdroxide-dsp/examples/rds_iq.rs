//! Write a synthetic FM broadcast station, RDS and all, as an IQ file the radio
//! can tune.
//!
//! For exercising the whole engine and the RDS window without a transmitter, and
//! for having something with a *known* station name to check a decode against —
//! on the air you only ever find out what a station calls itself by reading it
//! off this same decoder.
//!
//! ```text
//! cargo run -p sdroxide-dsp --example rds_iq -- station.iq [seconds]
//! cargo run --release -- --file station.iq --rate 240000 --freq 100000000 --mode WFM
//! ```
//!
//! Interleaved little-endian `complex64` (two f32s per sample), which is what
//! `FileSource` reads. The file loops, so a few seconds is enough to sit on.

use std::io::{BufWriter, Write};

const RATE: f64 = 240_000.0;
const BITRATE: f64 = 1_187.5;
/// Peak deviation the whole multiplex is scaled to.
const DEVIATION_HZ: f64 = 75_000.0;
/// Data subcarrier injection, as a fraction of peak deviation. Mid-range for a
/// real station.
const RDS_INJECTION: f64 = 0.03;
/// Programme level, chosen so pilot + audio + subcarrier stay inside the
/// deviation limit. Over-deviating does not fit the channel and the resulting
/// truncation distortion lands right across the composite.
const AUDIO_AMP: f64 = 0.45;

const PI_CODE: u16 = 0xD3C2;
const PTY: u16 = 10;
const PS: &[u8; 8] = b"SDROXIDE";
const RT: &str = "THE PILOTS - THIRD HARMONIC";
const PTYN: &[u8; 8] = b"CHARTS  ";
/// Where the artist and title sit inside `RT`, for the RadioText+ tags.
const ARTIST: (u16, u16) = (0, 10);
const TITLE: (u16, u16) = (13, 14);

// --- the block code -------------------------------------------------------

const POLY: u32 = 0b101_1011_1001;
const OFF_A: u16 = 0b00_1111_1100;
const OFF_B: u16 = 0b01_1001_1000;
const OFF_C: u16 = 0b01_0110_1000;
const OFF_CP: u16 = 0b11_0101_0000;
const OFF_D: u16 = 0b01_1011_0100;

fn check_word(info: u16) -> u16 {
    let mut reg = 0u32;
    let word = (info as u32) << 10;
    for i in (0..26).rev() {
        reg = (reg << 1) | ((word >> i) & 1);
        if reg & (1 << 10) != 0 {
            reg ^= POLY;
        }
    }
    (reg & 0x3ff) as u16
}

fn bits_for(groups: &[[u16; 4]]) -> Vec<u8> {
    let mut bits = Vec::with_capacity(groups.len() * 104);
    for g in groups {
        let third = if g[1] & 0x0800 != 0 { OFF_CP } else { OFF_C };
        for (i, &info) in g.iter().enumerate() {
            let off = [OFF_A, OFF_B, third, OFF_D][i];
            let word = ((info as u32) << 10) | (check_word(info) ^ off) as u32;
            for k in (0..26).rev() {
                bits.push(((word >> k) & 1) as u8);
            }
        }
    }
    bits
}

// --- the groups -----------------------------------------------------------

fn block_b(gtype: u16, rest: u16) -> u16 {
    (gtype << 12) | (1 << 10) | (PTY << 5) | rest
}

fn group_0a(seg: u16) -> [u16; 4] {
    let i = seg as usize * 2;
    [
        PI_CODE,
        block_b(0, (1 << 3) | seg),
        (1u16 << 8) | 204,
        ((PS[i] as u16) << 8) | PS[i + 1] as u16,
    ]
}

fn group_2a(seg: u16) -> [u16; 4] {
    let text: Vec<u8> = RT.bytes().chain(std::iter::once(0x0d)).collect();
    let at = |i: usize| *text.get(i).unwrap_or(&b' ') as u16;
    let base = seg as usize * 4;
    [PI_CODE, block_b(2, seg), (at(base) << 8) | at(base + 1), (at(base + 2) << 8) | at(base + 3)]
}

fn group_10a(seg: u16) -> [u16; 4] {
    let i = seg as usize * 4;
    let w = |a: usize, b: usize| ((PTYN[a] as u16) << 8) | PTYN[b] as u16;
    [PI_CODE, block_b(10, seg), w(i, i + 1), w(i + 2, i + 3)]
}

fn group_11a() -> [u16; 4] {
    let (ct1, start1, len1) = (4u16, ARTIST.0, ARTIST.1 - 1);
    let (ct2, start2, len2) = (1u16, TITLE.0, TITLE.1 - 1);
    [
        PI_CODE,
        block_b(11, (1 << 3) | (ct1 >> 3)),
        ((ct1 & 0x7) << 13) | (start1 << 7) | (len1 << 1) | (ct2 >> 5),
        ((ct2 & 0x1f) << 11) | (start2 << 5) | len2,
    ]
}

/// Group 4A, carrying a fixed clock: 2026-08-17 14:32 UTC, +2 h locally.
fn group_4a() -> [u16; 4] {
    const MJD: u32 = 61_269;
    let (hour, minute, offset) = (14u32, 32u16, 4u16);
    [
        PI_CODE,
        block_b(4, ((MJD >> 15) & 0x3) as u16),
        (((MJD & 0x7fff) << 1) as u16) | ((hour >> 4) & 1) as u16,
        (((hour & 0xf) as u16) << 12) | (minute << 6) | offset,
    ]
}

/// The group mix, in the proportions a station transmits: name and text often,
/// housekeeping rarely.
fn programme() -> Vec<[u16; 4]> {
    let mut g = Vec::new();
    for seg in 0..4 {
        g.push(group_0a(seg));
        g.push(group_2a(seg * 2));
        g.push(group_2a(seg * 2 + 1));
    }
    // Group 1A, variant 0: extended country code 0xE0, the European block.
    g.push([PI_CODE, block_b(1, 0), 0xE0, 0]);
    // Group 3A: RadioText+ rides on group 11A here.
    g.push([PI_CODE, block_b(3, 11 << 1), 0, 0x4BD7]);
    g.push(group_11a());
    g.push(group_4a());
    g.push(group_10a(0));
    g.push(group_10a(1));
    g
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let Some(path) = args.get(1) else {
        eprintln!("usage: rds_iq <out.iq> [seconds]");
        eprintln!("then:  sdroxide --file <out.iq> --rate 240000 --freq 100000000 --mode WFM");
        std::process::exit(2);
    };
    let secs: f64 = args.get(2).map_or(20.0, |s| s.parse().unwrap_or(20.0));
    let n = (RATE * secs) as usize;

    // Enough repeats of the group sequence to cover the whole file. The file
    // loops, and the bit stream restarting mid-group costs at most one group.
    let groups = programme();
    let per_group = 104.0 / BITRATE;
    let cycles = (secs / (per_group * groups.len() as f64)).ceil() as usize + 1;
    let repeated: Vec<[u16; 4]> =
        groups.iter().copied().cycle().take(groups.len() * cycles).collect();
    let bits = bits_for(&repeated);

    // Differential encoding, then the shaped biphase symbol.
    let mut state = 0u8;
    let symbols: Vec<f32> = bits
        .iter()
        .map(|&b| {
            state ^= b;
            if state == 1 { 1.0 } else { -1.0 }
        })
        .collect();

    let mut out = BufWriter::new(std::fs::File::create(path)?);
    let mut phase = 0.0f64;
    for i in 0..n {
        let t = i as f64 / RATE;

        let pos = t * BITRATE;
        let sym = symbols[(pos as usize).min(symbols.len() - 1)] as f64;
        let data = sym * (std::f64::consts::TAU * pos.fract()).sin();

        let l = AUDIO_AMP * (std::f64::consts::TAU * 440.0 * t).sin();
        let r = AUDIO_AMP * (std::f64::consts::TAU * 660.0 * t).sin();
        let (m, s) = ((l + r) / 2.0, (l - r) / 2.0);

        let mpx = 0.85 * (m + s * (std::f64::consts::TAU * 38_000.0 * t).sin())
            + 0.1 * (std::f64::consts::TAU * 19_000.0 * t).sin()
            + RDS_INJECTION * data * (std::f64::consts::TAU * 57_000.0 * t).sin();

        phase += std::f64::consts::TAU * DEVIATION_HZ * mpx / RATE;
        out.write_all(&(phase.cos() as f32).to_le_bytes())?;
        out.write_all(&(phase.sin() as f32).to_le_bytes())?;
    }
    out.flush()?;

    println!("{path}: {secs} s at {RATE} sps");
    println!("  station  {}", String::from_utf8_lossy(PS));
    println!("  identity {PI_CODE:04X}");
    println!("  text     {RT}");
    println!();
    println!("sdroxide --file {path} --rate 240000 --freq 100000000 --mode WFM");
    Ok(())
}
