//! Fine Offset Electronics sensors on 868.3 MHz, and the many names they are
//! sold under — Ecowitt, Froggit, Ambient Weather, SwitchDoc, Misol.
//!
//! # Source
//!
//! The frame layouts are transcribed from the protocol documentation comments in
//! `rtl_433`'s `src/devices/fineoffset.c`, which describe the devices rather than
//! the decoder. Each one is reproduced here as a test vector using the sample
//! frame and the decoded values published beside it, so what is implemented is
//! checked against a capture from a real sensor and not against a reading of the
//! prose. No code was taken from there.
//!
//! # One family, several frame lengths
//!
//! These sensors share a physical layer with the LaCrosse units — 2-FSK at
//! 17.24 kbps behind an `0xaa` preamble and an `0x2dd4` sync word — and then a
//! common framing:
//!
//! ```text
//! FF <payload …> CC SS
//!  |              |  `- sum of every preceding byte, mod 256
//!  |              `---- CRC-8, poly 0x31, init 0, over every byte before it
//!  `------------------- family code, which says what the payload means
//! ```
//!
//! Two independent checks, which is what makes this family safe to decode at all:
//! a CRC-8 alone passes one noise frame in 256, and the burst gate on a busy band
//! offers plenty of those. With the sum as well it is one in 65 536, on top of the
//! preamble and sync-word constraints the slicer has already applied.
//!
//! The members differ in length and in *where* the two check bytes sit — the WS85
//! carries four more bytes after its sum — so a layout is identified by the family
//! code plus the position of its CRC, and [`LAYOUTS`] is the whole of what
//! distinguishes them.
//!
//! # How much of each layout is believed
//!
//! Every layout below was checked by computing both check bytes over the sample
//! frame published with it: nine bytes of WN34, nine of WH57, eighteen of WS80,
//! thirty-one of WS85 and thirty-two of WS90 all agree, which settles the framing
//! and the byte alignment beyond argument. The *fields* are on a weaker footing,
//! and the tests say which is which — a sample published with the readings it
//! produced (WH24, WH51) is proof, a sample published bare is only corroboration
//! that the numbers coming out are physically sensible.
//!
//! Two fields are deliberately missing as a result, both rain totals: see
//! [`ws85`] and [`ws90`].
//!
//! # Recognised but not read
//!
//! A frame whose family code is not one of the mapped layouts still gets reported
//! when both checks pass, with its identity and its bytes but no readings. That is
//! not a guess: the CRC and the sum agreeing means this really is a Fine Offset
//! sensor of *some* model, and saying "there is a WH45 here that I cannot read
//! yet" is more use than saying nothing.

use sdroxide_types::{IsmProtocol, IsmQuantity, IsmReading};

use super::Decoded;
use crate::crc::crc8;
use crate::slice::Phy;

/// Shared with the LaCrosse sensors; only the framing after the sync word differs.
pub const PHY: Phy = Phy {
    baud: 17_241.0,
    sync: 0x2DD4,
    sync_bits: 16,
    sync_errors: 1,
    preamble_bits: 16,
    // The shortest mapped layout is nine bytes (WN34, WH57) and the longest is
    // thirty-two (WS90).
    payload_min: 9,
    payload_max: 32,
    // Plain NRZ: one bit per symbol on the air.
    manchester: false,
};

pub const CHANNEL_HZ: f64 = 868_300_000.0;

const CRC_POLY: u8 = 0x31;

/// One frame layout: which family code announces it, where its CRC byte sits, what
/// to call it, and how to read it.
struct Layout {
    family: u8,
    /// Index of the CRC byte. It covers everything before it, and the byte sum
    /// immediately follows it covering everything up to and including it.
    ///
    /// A position rather than a frame length because the WS85 is not the end of
    /// its own frame — four bytes of trailer follow its sum.
    crc_at: usize,
    name: &'static str,
    /// `None` for a layout that is recognised but whose payload is not read.
    read: Option<fn(&[u8]) -> Option<Decoded>>,
}

/// Every layout this module knows, tried longest first.
///
/// A code only belongs here once there is a source saying which sensor sends it
/// *and* a sample frame whose two check bytes agree at the stated offset.
const LAYOUTS: &[Layout] = &[
    Layout { family: 0x90, crc_at: 30, name: "WS90", read: Some(ws90) },
    Layout { family: 0x85, crc_at: 26, name: "WS85", read: Some(ws85) },
    Layout { family: 0x80, crc_at: 16, name: "WS80", read: Some(ws80) },
    Layout { family: 0x24, crc_at: 15, name: "WH24/WH65", read: Some(wh24) },
    Layout { family: 0x51, crc_at: 12, name: "WH51 soil", read: Some(wh51) },
    Layout { family: 0x57, crc_at: 7, name: "WH57 lightning", read: Some(wh57) },
    Layout { family: 0x38, crc_at: 7, name: "WN38", read: Some(wn34) },
    Layout { family: 0x34, crc_at: 7, name: "WN34", read: Some(wn34) },
];

/// Whether the two check bytes at `crc_at` agree with the frame.
fn checks_pass(p: &[u8], crc_at: usize) -> bool {
    if p.len() < crc_at + 2 || crc_at == 0 {
        return false;
    }
    if crc8(CRC_POLY, 0x00, &p[..crc_at]) != p[crc_at] {
        return false;
    }
    // Mod-256 sum of everything up to and including the CRC. The source comment
    // calls this a "bitsum (XOR)", which it is not: the published sample only
    // agrees with an ordinary sum, and the test vectors below are what settle it.
    p[..=crc_at].iter().fold(0u8, |a, &b| a.wrapping_add(b)) == p[crc_at + 1]
}

pub fn parse(p: &[u8]) -> Option<Decoded> {
    let family = *p.first()?;

    // The family code says which layout this is, so only that one is tried — and
    // the CRC then has to agree at exactly the offset that layout puts it. Trying
    // every offset instead would be six more chances for noise to pass.
    for l in LAYOUTS {
        if l.family != family || !checks_pass(p, l.crc_at) {
            continue;
        }
        return match l.read {
            Some(read) => read(p),
            None => Some(identified(p, l.name)),
        };
    }

    // Not a mapped family, but if the checks agree at any known offset this is
    // still a real sensor of this family.
    for l in LAYOUTS {
        if checks_pass(p, l.crc_at) {
            return Some(identified(p, &format!("family {family:#04x}")));
        }
    }
    None
}

/// A frame that validated but whose payload this module does not read.
fn identified(p: &[u8], name: &str) -> Decoded {
    Decoded {
        protocol: IsmProtocol::FineOffset,
        model: Some(name.to_string()),
        // Every layout in this family puts the sensor id straight after the
        // family code; how many bytes of it there are is what varies, so the
        // widest safe reading is the one byte they all have.
        device: format!("{:02x}", p[1]),
        readings: Vec::new(),
        extra: vec![("payload".to_string(), "not decoded".to_string())],
        encrypted: false,
        raw_hex: String::new(),
    }
}

/// WH51 / WN31 soil moisture, and the SwitchDoc SM23.
///
/// ```text
/// FF II II II TB YY MM ZA AA XX XX XX CC SS
/// ```
///
/// `T` is a transmission-period boost flag, `B` the battery voltage in tenths,
/// `MM` the moisture percentage, and `AAA` a nine-bit raw ADC reading. `YY` and
/// the three `XX` bytes are fixed padding.
fn wh51(p: &[u8]) -> Option<Decoded> {
    let id = (u32::from(p[1]) << 16) | (u32::from(p[2]) << 8) | u32::from(p[3]);
    let battery_v = f64::from(p[4] & 0x1F) / 10.0;
    let moisture = p[6];
    if moisture > 100 {
        return None;
    }

    let mut readings = vec![IsmReading::new(IsmQuantity::SoilMoisturePct, f64::from(moisture))];
    // A plausible cell voltage only. Zero means the field was not populated, and
    // anything above 2 V is not a single AA cell, so neither is worth reporting as
    // a measurement.
    if (0.1..=2.0).contains(&battery_v) {
        readings.push(IsmReading::new(IsmQuantity::BatteryVolts, battery_v));
    }

    Some(Decoded {
        protocol: IsmProtocol::FineOffset,
        model: Some("WH51 soil".to_string()),
        device: format!("{id:06x}"),
        readings,
        extra: Vec::new(),
        encrypted: false,
        raw_hex: String::new(),
    })
}

/// WH24 / WH65 / WS69 / HP1000 outdoor arrays.
///
/// ```text
/// FF II DD VT TT HH WW GG RR RR UU UU LL LL LL CC BB
/// ```
///
/// # Why there is no wind speed here
///
/// The layout is confirmed by the published sample for the identity,
/// temperature, humidity, wind direction and rainfall — every one of those
/// reproduces the value stated beside the capture, and the test below asserts it.
/// The wind *speed* fields do not: the sample has `WW = 8` and `GG = 2` while the
/// readings quoted with it are 1.12 m/s average and 2.24 m/s gust, which no single
/// scale factor produces, and a gust below the average is not a reading a
/// weather station can take. Something in that example is wrong, and with no
/// capture of our own to settle it the honest thing is to leave the two fields
/// out rather than publish a number derived from an inconsistency.
fn wh24(p: &[u8]) -> Option<Decoded> {
    let id = p[1];
    let flags = p[3] >> 4;
    // Nine bits: the byte, plus one bit carried in the flags nibble.
    let wind_dir = u32::from(p[2]) | (u32::from(flags & 0x08) << 5);

    // Twelve bits spanning the low nibble of byte 3 and byte 4, of which the top
    // one is a low-battery flag.
    let raw = (u32::from(p[3] & 0x0F) << 8) | u32::from(p[4]);
    let low_battery = raw & 0x800 != 0;
    let temp_c = f64::from(raw & 0x7FF) / 10.0 - 40.0;
    if !(-40.0..=80.0).contains(&temp_c) {
        return None;
    }

    let humidity = p[5];
    let rain_counter = (u32::from(p[8]) << 8) | u32::from(p[9]);

    let mut readings = vec![IsmReading::new(IsmQuantity::TempC, temp_c)];
    if humidity <= 100 {
        readings.push(IsmReading::new(IsmQuantity::HumidityPct, f64::from(humidity)));
    }
    if wind_dir <= 360 {
        readings.push(IsmReading::new(IsmQuantity::WindDirDeg, f64::from(wind_dir)));
    }
    // 0.3 mm per tick, which the published sample confirms: 74 ticks is the
    // 22.2 mm quoted with it.
    readings.push(IsmReading::new(IsmQuantity::RainMm, f64::from(rain_counter) * 0.3));

    let mut extra = Vec::new();
    if low_battery {
        extra.push(("battery".to_string(), "low".to_string()));
    }

    Some(Decoded {
        protocol: IsmProtocol::FineOffset,
        model: Some("WH24/WH65".to_string()),
        device: format!("{id:02x}"),
        readings,
        extra,
        encrypted: false,
        raw_hex: String::new(),
    })
}

/// Plausible cell or supercap voltage. Zero means the field was not populated,
/// and these sensors run from one or two cells or a solar-charged supercap, so
/// anything outside this is a decode error rather than a measurement.
const BATTERY_RANGE_V: std::ops::RangeInclusive<f64> = 0.5..=4.0;

/// Highest UV index worth believing. The strongest ever recorded at the Earth's
/// surface is around 43, on an Andean volcano; a garden sensor reporting more
/// than 20 is reporting a byte that was not a UV reading.
const MAX_UV: f64 = 20.0;

/// The measurement block the WS80 and WS90 share.
///
/// Identical byte for byte in both layouts, which is worth more than either
/// sample on its own: two devices documented independently and agreeing is a
/// cross-check no single capture provides. Bytes 4–13, laid out
///
/// ```text
/// byte  4  5   6    7      8    9    10  11  12   13
/// field light   batt flags  temp hum  wind dir gust uv
/// ```
///
/// where `flags` carries the ninth bit of each nine-bit wind field and the top
/// two of the ten-bit temperature. A wind field reading `0x1ff` is the sensor
/// saying it has no value, not a 51 m/s gale.
fn ws80_block(p: &[u8], readings: &mut Vec<IsmReading>, extra: &mut Vec<(String, String)>) {
    let flags = p[7];

    let temp_c = f64::from((u32::from(flags & 0x03) << 8) | u32::from(p[8])) / 10.0 - 40.0;
    if (-40.0..=80.0).contains(&temp_c) {
        readings.push(IsmReading::new(IsmQuantity::TempC, temp_c));
    }
    if p[9] <= 100 {
        readings.push(IsmReading::new(IsmQuantity::HumidityPct, f64::from(p[9])));
    }

    let avg = (u32::from(flags & 0x10) << 4) | u32::from(p[10]);
    if avg != 0x1FF {
        readings.push(IsmReading::new(IsmQuantity::WindAvgMs, f64::from(avg) / 10.0));
    }
    let gust = (u32::from(flags & 0x40) << 2) | u32::from(p[12]);
    if gust != 0x1FF {
        readings.push(IsmReading::new(IsmQuantity::WindGustMs, f64::from(gust) / 10.0));
    }
    let dir = (u32::from(flags & 0x20) << 3) | u32::from(p[11]);
    if dir < 360 {
        readings.push(IsmReading::new(IsmQuantity::WindDirDeg, f64::from(dir)));
    }

    let uv = f64::from(p[13]) / 10.0;
    if uv <= MAX_UV {
        readings.push(IsmReading::new(IsmQuantity::UvIndex, uv));
    }
    readings.push(IsmReading::new(
        IsmQuantity::LuxLx,
        f64::from((u32::from(p[4]) << 8) | u32::from(p[5])) * 10.0,
    ));

    let battery_v = f64::from(p[6]) * 0.02;
    if BATTERY_RANGE_V.contains(&battery_v) {
        readings.push(IsmReading::new(IsmQuantity::BatteryVolts, battery_v));
    } else {
        extra.push(("battery".to_string(), "flat or absent".to_string()));
    }
}

/// WS80 outdoor array — temperature, humidity, wind, UV and light, no rain.
///
/// ```text
/// YY II II II LL LL BB FF TT HH WW DD GG VV UU UU AA XX
/// ```
fn ws80(p: &[u8]) -> Option<Decoded> {
    let (mut readings, mut extra) = (Vec::new(), Vec::new());
    ws80_block(p, &mut readings, &mut extra);
    Some(Decoded {
        protocol: IsmProtocol::FineOffset,
        model: Some("WS80".to_string()),
        device: format!("{:02x}{:02x}{:02x}", p[1], p[2], p[3]),
        readings,
        extra,
        encrypted: false,
        raw_hex: String::new(),
    })
}

/// WS90 "Wittboy" array — a WS80 with a haptic rain gauge and a supercap.
///
/// # Why there is no rainfall here
///
/// The layout puts five bytes aside for rain and the documentation names only two
/// of them as the total. The one published capture has both of those bytes at
/// zero, so it cannot tell a right guess from a wrong one — and there is a
/// non-zero byte sitting immediately before them that would read as 18.6 mm on
/// the same scale. A rain total is exactly the sort of number that is worse than
/// useless if it is read out of the wrong bytes, so it waits for a capture from a
/// sensor that is actually wet. Everything else here is the WS80 block, which the
/// two devices share.
fn ws90(p: &[u8]) -> Option<Decoded> {
    let (mut readings, mut extra) = (Vec::new(), Vec::new());
    ws80_block(p, &mut readings, &mut extra);

    // Solar-charged reservoir the sensor runs from at night, and the number that
    // says whether it is about to stop reporting at 4 a.m.
    let supercap_v = f64::from(p[21] & 0x3F) / 10.0;
    if supercap_v > 0.0 {
        extra.push(("supercap".to_string(), format!("{supercap_v:.1} V")));
    }
    extra.push(("rain".to_string(), "not decoded".to_string()));

    Some(Decoded {
        protocol: IsmProtocol::FineOffset,
        model: Some("WS90".to_string()),
        device: format!("{:02x}{:02x}{:02x}", p[1], p[2], p[3]),
        readings,
        extra,
        encrypted: false,
        raw_hex: String::new(),
    })
}

/// WS85 — wind and rain only, on a different byte order from the WS80 block.
///
/// ```text
/// YY II II II BB FF UU WW DD GG UU UU RS UU UU R1 R2 SS …
/// ```
///
/// Rain is left out for the same reason as on the [`ws90`]: the published capture
/// is dry, so nothing distinguishes the documented byte pair from its neighbours.
/// That is most of the point of a WS85, and it is said plainly in the report
/// rather than quietly omitted.
fn ws85(p: &[u8]) -> Option<Decoded> {
    let flags = p[5];
    let mut readings = Vec::new();

    let avg = (u32::from(flags & 0x10) << 4) | u32::from(p[7]);
    if avg != 0x1FF {
        readings.push(IsmReading::new(IsmQuantity::WindAvgMs, f64::from(avg) / 10.0));
    }
    let gust = (u32::from(flags & 0x40) << 2) | u32::from(p[9]);
    if gust != 0x1FF {
        readings.push(IsmReading::new(IsmQuantity::WindGustMs, f64::from(gust) / 10.0));
    }
    let dir = (u32::from(flags & 0x20) << 3) | u32::from(p[8]);
    if dir < 360 {
        readings.push(IsmReading::new(IsmQuantity::WindDirDeg, f64::from(dir)));
    }

    let mut extra = vec![("rain".to_string(), "not decoded".to_string())];
    let battery_v = f64::from(p[4]) * 0.02;
    if BATTERY_RANGE_V.contains(&battery_v) {
        readings.push(IsmReading::new(IsmQuantity::BatteryVolts, battery_v));
    }
    let supercap_v = f64::from(p[17] & 0x3F) / 10.0;
    if supercap_v > 0.0 {
        extra.push(("supercap".to_string(), format!("{supercap_v:.1} V")));
    }

    Some(Decoded {
        protocol: IsmProtocol::FineOffset,
        model: Some("WS85".to_string()),
        device: format!("{:02x}{:02x}{:02x}", p[1], p[2], p[3]),
        readings,
        extra,
        encrypted: false,
        raw_hex: String::new(),
    })
}

/// WN34 soil and water probes, and the WN38 — sold as Froggit DP150 / DP35 and
/// Ecowitt WN34S / WN34L / WN34D.
///
/// ```text
/// YY II II II ST TT BB XX AA
/// ```
///
/// `S` is a sub-type nibble and `TTT` a twelve-bit temperature in tenths.
///
/// # The offset that depends on the sub-type
///
/// Sub-type 4 reads its temperature straight; every other sub-type has 40 °C
/// subtracted. That is an odd rule to take on trust, so it is worth saying what
/// argues for it: the two published captures are a sub-type 0 and a sub-type 4,
/// and under this rule both come out at 24.5 °C — two probes on the same bench.
/// Under either single rule applied to both, one of them reads 64.5 °C or
/// −15.5 °C. That is not proof, but a coincidence at one decimal place is
/// unlikely enough to act on.
fn wn34(p: &[u8]) -> Option<Decoded> {
    let sub = p[4] >> 4;
    let raw = f64::from((u32::from(p[4] & 0x0F) << 8) | u32::from(p[5]));
    let temp_c = if sub == 4 { raw / 10.0 } else { raw / 10.0 - 40.0 };
    if !(-40.0..=80.0).contains(&temp_c) {
        return None;
    }

    let mut readings = vec![IsmReading::new(IsmQuantity::TempC, temp_c)];
    let battery_v = f64::from(p[6] & 0x7F) * 0.02;
    if BATTERY_RANGE_V.contains(&battery_v) {
        readings.push(IsmReading::new(IsmQuantity::BatteryVolts, battery_v));
    }

    Some(Decoded {
        protocol: IsmProtocol::FineOffset,
        model: Some(
            match (p[0], sub) {
                (0x38, _) => "WN38",
                (_, 0) | (_, 4) => "WN34 water",
                _ => "WN34 soil",
            }
            .to_string(),
        ),
        device: format!("{:02x}{:02x}{:02x}", p[1], p[2], p[3]),
        readings,
        extra: Vec::new(),
        encrypted: false,
        raw_hex: String::new(),
    })
}

/// WH57 lightning detector, sold as Ecowitt WH57, Froggit DP60 and Ambient
/// Weather WH31L. An AS3935 Franklin front end in a plastic egg.
///
/// ```text
/// YY SI II II FF KK CC XX AA
/// ```
///
/// `S` is what the sensor is reporting, `KK` the estimated storm distance in the
/// low six bits, and `CC` a running strike total.
///
/// # No battery here
///
/// The layout puts a battery level in two bits of the flags byte. Both published
/// captures read the same value there, so nothing says which end of the range is
/// flat, and a battery indicator that might be inverted is worse than none.
fn wh57(p: &[u8]) -> Option<Decoded> {
    let event = match p[1] >> 4 {
        0 => "startup",
        1 => "interference",
        4 => "noise",
        8 => "strike",
        // A state nothing documents. The frame passed both checks so the sensor
        // is real, but this module will not name what it is saying.
        _ => return None,
    };

    let mut readings = vec![IsmReading::new(IsmQuantity::StrikeCount, f64::from(p[6]))];
    // 63 is the sensor saying it has no estimate — at startup, and whenever it
    // counted a strike it could not range.
    let distance_km = p[5] & 0x3F;
    if distance_km != 63 {
        readings.push(IsmReading::new(IsmQuantity::LightningKm, f64::from(distance_km)));
    }

    Some(Decoded {
        protocol: IsmProtocol::FineOffset,
        model: Some("WH57 lightning".to_string()),
        device: format!(
            "{:05x}",
            (u32::from(p[1] & 0x0F) << 16) | (u32::from(p[2]) << 8) | u32::from(p[3])
        ),
        readings,
        extra: vec![("event".to_string(), event.to_string())],
        encrypted: false,
        raw_hex: String::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(hex: &str) -> Vec<u8> {
        hex.split_whitespace().map(|b| u8::from_str_radix(b, 16).unwrap()).collect()
    }

    /// The WH51 sample published with the layout, and the fields it states.
    ///
    /// Both checks passing on this byte alignment is strong evidence on its own:
    /// a CRC and an independent sum agreeing over thirteen bytes does not happen
    /// if the frame has been split up wrongly.
    #[test]
    fn the_published_wh51_capture_decodes() {
        // aa aa aa 2d d4 | 51 00 6b 58 6e 7f 24 f8 d2 ff ff ff 3c 28
        let f =
            [0x51, 0x00, 0x6B, 0x58, 0x6E, 0x7F, 0x24, 0xF8, 0xD2, 0xFF, 0xFF, 0xFF, 0x3C, 0x28];
        assert!(checks_pass(&f, 12), "the CRC and sum of the published sample must agree");

        let d = parse(&f).expect("the sample did not decode");
        assert_eq!(d.protocol, IsmProtocol::FineOffset);
        assert_eq!(d.model.as_deref(), Some("WH51 soil"));
        assert_eq!(d.device, "006b58");
        // Moisture is the documented 0x24 = 36 %.
        assert_eq!(d.readings[0].quantity, IsmQuantity::SoilMoisturePct);
        assert!((d.readings[0].value - 36.0).abs() < 1e-9);
        // Battery is the low five bits of 0x6e in tenths of a volt.
        assert_eq!(d.readings[1].quantity, IsmQuantity::BatteryVolts);
        assert!((d.readings[1].value - 1.4).abs() < 1e-9, "got {}", d.readings[1].value);
    }

    /// The WH24 sample, and the readings published beside it: id 191, 11.8 °C,
    /// 78 %RH, wind direction 266°, rainfall 22.2 mm.
    #[test]
    fn the_published_wh24_capture_decodes_to_its_stated_readings() {
        let f = [
            0x24, 0xBF, 0x0A, 0xE2, 0x06, 0x4E, 0x08, 0x02, 0x00, 0x4A, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x8F, 0x07,
        ];
        assert!(checks_pass(&f, 15), "the CRC and sum of the published sample must agree");

        let d = parse(&f).expect("the sample did not decode");
        assert_eq!(d.model.as_deref(), Some("WH24/WH65"));
        assert_eq!(d.device, "bf", "id 191 is 0xbf");

        let by = |q: IsmQuantity| d.readings.iter().find(|r| r.quantity == q).map(|r| r.value);
        assert!((by(IsmQuantity::TempC).unwrap() - 11.8).abs() < 1e-9);
        assert!((by(IsmQuantity::HumidityPct).unwrap() - 78.0).abs() < 1e-9);
        assert!((by(IsmQuantity::WindDirDeg).unwrap() - 266.0).abs() < 1e-9);
        assert!((by(IsmQuantity::RainMm).unwrap() - 22.2).abs() < 1e-6);
        // The two fields the published example contradicts itself about.
        assert!(by(IsmQuantity::WindAvgMs).is_none(), "wind speed must not be guessed");
        assert!(by(IsmQuantity::WindGustMs).is_none(), "gust must not be guessed");
    }

    /// Both checks have to reject, independently. A decoder that only really
    /// applied one of them would report noise as soil moisture once every 256
    /// bursts, and on this band that is often.
    #[test]
    fn each_check_rejects_on_its_own() {
        let good =
            [0x51, 0x00, 0x6B, 0x58, 0x6E, 0x7F, 0x24, 0xF8, 0xD2, 0xFF, 0xFF, 0xFF, 0x3C, 0x28];
        assert!(parse(&good).is_some());

        // Break the CRC, leaving the sum consistent with the broken CRC byte.
        let mut bad = good;
        bad[12] ^= 0x01;
        bad[13] = bad[..13].iter().fold(0u8, |a, &b| a.wrapping_add(b));
        assert!(parse(&bad).is_none(), "a wrong CRC was accepted");

        // Break only the sum.
        let mut bad = good;
        bad[13] ^= 0x01;
        assert!(parse(&bad).is_none(), "a wrong sum was accepted");

        // A payload value the sensor cannot produce.
        let mut bad = good;
        bad[6] = 200; // 200 % moisture
        bad[12] = crc8(CRC_POLY, 0x00, &bad[..12]);
        bad[13] = bad[..13].iter().fold(0u8, |a, &b| a.wrapping_add(b));
        assert!(parse(&bad).is_none(), "200 % soil moisture was accepted");

        for n in 0..14 {
            assert!(parse(&good[..n]).is_none(), "{n} bytes should not parse");
        }
    }

    /// An unmapped family whose checks pass is a real sensor and has to be
    /// reported — with no readings, because none are known.
    #[test]
    fn an_unmapped_family_is_reported_without_readings() {
        // A well-formed frame whose checks land at a known offset, with a family
        // code nothing maps.
        let mut f = [0u8; 14];
        f[0] = 0x9E;
        f[1] = 0xAB;
        for (i, b) in f.iter_mut().enumerate().take(12).skip(2) {
            *b = (i as u8) * 7;
        }
        f[12] = crc8(CRC_POLY, 0x00, &f[..12]);
        f[13] = f[..13].iter().fold(0u8, |a, &b| a.wrapping_add(b));

        let d = parse(&f).expect("a valid frame of an unknown layout must still be reported");
        assert_eq!(d.model.as_deref(), Some("family 0x9e"), "the code itself, not a guessed model");
        assert_eq!(d.device, "ab");
        assert!(d.readings.is_empty());
        assert_eq!(d.extra, vec![("payload".to_string(), "not decoded".to_string())]);
    }

    /// The five layouts added from published captures. None of these samples was
    /// published with its readings, so what is asserted is what the sample can
    /// actually prove: that both check bytes agree at the stated offset, which
    /// settles the framing, the length and the byte alignment together.
    #[test]
    fn every_published_sample_validates_at_its_layouts_offset() {
        for (hex, family, crc_at, model) in [
            ("80 0a 00 3b 00 00 88 8a 59 38 18 6d 1c 00 ff ff d8 df", 0x80, 16, "WS80"),
            (
                "90 00 34 2b 00 77 a4 82 62 39 00 3e 00 00 3f ff 20 00 ba 00 00 26 02 00 ff 9f \
                 f8 00 00 82 92 4f",
                0x90,
                30,
                "WS90",
            ),
            (
                "85 00 28 eb 87 82 6f 00 83 00 3f ff 00 00 00 00 00 0b 00 00 ff ef fd 00 00 6b \
                 dd 0f 00 00 00",
                0x85,
                26,
                "WS85",
            ),
            ("34 00 29 65 02 85 44 66 f3", 0x34, 7, "WN34 water"),
            ("38 00 28 85 40 f5 41 4c a7", 0x38, 7, "WN38"),
            ("57 01 05 c8 05 bf 00 dd c6", 0x57, 7, "WH57 lightning"),
            ("57 81 05 c8 05 81 01 df 0b", 0x57, 7, "WH57 lightning"),
        ] {
            let f = frame(hex);
            assert_eq!(f[0], family, "{hex}");
            assert!(checks_pass(&f, crc_at), "{hex}: CRC and sum must agree at byte {crc_at}");
            let d = parse(&f).unwrap_or_else(|| panic!("{hex} did not decode"));
            assert_eq!(d.model.as_deref(), Some(model), "{hex}");
        }
    }

    /// The WS80's one sample carries no stated readings, so what this asserts is
    /// coherence: every field has to land somewhere a real garden could be, and
    /// the gust has to be at least the average — which is the check that caught
    /// the broken WH24 wind example.
    #[test]
    fn the_ws80_sample_reads_as_a_coherent_set_of_measurements() {
        let f = frame("80 0a 00 3b 00 00 88 8a 59 38 18 6d 1c 00 ff ff d8 df");
        let d = parse(&f).expect("did not decode");
        assert_eq!(d.device, "0a003b");

        let by = |q: IsmQuantity| d.readings.iter().find(|r| r.quantity == q).map(|r| r.value);
        assert!((by(IsmQuantity::TempC).unwrap() - 20.1).abs() < 1e-9);
        assert!((by(IsmQuantity::HumidityPct).unwrap() - 56.0).abs() < 1e-9);
        assert!((by(IsmQuantity::WindAvgMs).unwrap() - 2.4).abs() < 1e-9);
        assert!((by(IsmQuantity::WindGustMs).unwrap() - 2.8).abs() < 1e-9);
        assert!(
            by(IsmQuantity::WindGustMs) >= by(IsmQuantity::WindAvgMs),
            "a gust below the average means the two fields are the wrong way round"
        );
        assert!((by(IsmQuantity::WindDirDeg).unwrap() - 109.0).abs() < 1e-9);
        assert!((by(IsmQuantity::BatteryVolts).unwrap() - 2.72).abs() < 1e-9);
        // Night: no light, no UV.
        assert!((by(IsmQuantity::LuxLx).unwrap()).abs() < 1e-9);
        assert!((by(IsmQuantity::UvIndex).unwrap()).abs() < 1e-9);
    }

    /// The WS90 shares the WS80's measurement block, so the same coherence
    /// argument applies — and the rain total must be absent and *said* to be
    /// absent, rather than quietly reading as zero.
    #[test]
    fn the_ws90_sample_is_coherent_and_declines_to_report_rain() {
        let f = frame(
            "90 00 34 2b 00 77 a4 82 62 39 00 3e 00 00 3f ff 20 00 ba 00 00 26 02 00 ff 9f f8 \
             00 00 82 92 4f",
        );
        let d = parse(&f).expect("did not decode");
        assert_eq!(d.device, "00342b");

        let by = |q: IsmQuantity| d.readings.iter().find(|r| r.quantity == q).map(|r| r.value);
        assert!((by(IsmQuantity::TempC).unwrap() - 21.0).abs() < 1e-9);
        assert!((by(IsmQuantity::HumidityPct).unwrap() - 57.0).abs() < 1e-9);
        assert!((by(IsmQuantity::LuxLx).unwrap() - 1190.0).abs() < 1e-9);
        assert!((by(IsmQuantity::BatteryVolts).unwrap() - 3.28).abs() < 1e-9);
        assert!(by(IsmQuantity::RainMm).is_none(), "the rain bytes are unverified");
        assert!(d.extra.contains(&("rain".to_string(), "not decoded".to_string())));
        assert!(d.extra.contains(&("supercap".to_string(), "3.8 V".to_string())));
    }

    /// The WS85's wind and battery, and the same refusal to invent a rain total —
    /// which on a rain gauge is worth stating out loud rather than leaving the
    /// row looking empty.
    #[test]
    fn the_ws85_sample_reports_wind_and_says_rain_is_missing() {
        let f = frame(
            "85 00 28 eb 87 82 6f 00 83 00 3f ff 00 00 00 00 00 0b 00 00 ff ef fd 00 00 6b dd \
             0f 00 00 00",
        );
        let d = parse(&f).expect("did not decode");
        assert_eq!(d.device, "0028eb");

        let by = |q: IsmQuantity| d.readings.iter().find(|r| r.quantity == q).map(|r| r.value);
        assert!((by(IsmQuantity::WindDirDeg).unwrap() - 131.0).abs() < 1e-9);
        assert!((by(IsmQuantity::WindAvgMs).unwrap()).abs() < 1e-9, "calm");
        assert!((by(IsmQuantity::BatteryVolts).unwrap() - 2.70).abs() < 1e-9);
        assert!(by(IsmQuantity::RainMm).is_none());
        assert!(d.extra.contains(&("rain".to_string(), "not decoded".to_string())));
    }

    /// The two soil/water probes. See the note on [`wn34`]: both published
    /// captures landing on the same temperature is the whole of the evidence for
    /// the sub-type rule, so it is what the test asserts.
    #[test]
    fn both_probe_captures_land_on_the_same_bench_temperature() {
        for hex in ["34 00 29 65 02 85 44 66 f3", "38 00 28 85 40 f5 41 4c a7"] {
            let d = parse(&frame(hex)).unwrap_or_else(|| panic!("{hex} did not decode"));
            let by = |q: IsmQuantity| d.readings.iter().find(|r| r.quantity == q).map(|r| r.value);
            assert!(
                (by(IsmQuantity::TempC).unwrap() - 24.5).abs() < 1e-9,
                "{hex} gave {:?}",
                by(IsmQuantity::TempC)
            );
            // Both are single-cell probes, and both read as one.
            let v = by(IsmQuantity::BatteryVolts).unwrap();
            assert!((1.0..=1.7).contains(&v), "{hex} battery {v} V is not an AA cell");
        }
    }

    /// The lightning detector's two captures are published with the state the
    /// sensor was in, which is the one thing about them that *is* stated.
    #[test]
    fn the_lightning_captures_reproduce_their_stated_states() {
        // Powering up: no strike, and no distance estimate to give.
        let d = parse(&frame("57 01 05 c8 05 bf 00 dd c6")).expect("did not decode");
        assert_eq!(d.device, "105c8");
        assert_eq!(d.extra, vec![("event".to_string(), "startup".to_string())]);
        let by = |d: &Decoded, q: IsmQuantity| {
            d.readings.iter().find(|r| r.quantity == q).map(|r| r.value)
        };
        assert!((by(&d, IsmQuantity::StrikeCount).unwrap()).abs() < 1e-9);
        assert!(by(&d, IsmQuantity::LightningKm).is_none(), "63 means no estimate");

        // And a strike, from the same sensor: one count, one kilometre.
        let d = parse(&frame("57 81 05 c8 05 81 01 df 0b")).expect("did not decode");
        assert_eq!(d.device, "105c8", "the same sensor as the startup frame");
        assert_eq!(d.extra, vec![("event".to_string(), "strike".to_string())]);
        assert!((by(&d, IsmQuantity::StrikeCount).unwrap() - 1.0).abs() < 1e-9);
        assert!((by(&d, IsmQuantity::LightningKm).unwrap() - 1.0).abs() < 1e-9);

        // A state nothing documents must be refused rather than named.
        let mut f = frame("57 81 05 c8 05 81 01 df 0b");
        f[1] = 0x21;
        f[7] = crc8(CRC_POLY, 0x00, &f[..7]);
        f[8] = f[..8].iter().fold(0u8, |a, &b| a.wrapping_add(b));
        assert!(parse(&f).is_none(), "an undocumented state was given a name");
    }

    /// Noise must not pass both checks. Two independent eight-bit checks put the
    /// odds at one in 65 536, so a few hundred thousand random frames is enough
    /// to notice a decoder that is only applying one of them.
    #[test]
    fn random_frames_essentially_never_pass_both_checks() {
        let mut seed = 0x2545_F491_4F6C_DD1Du64;
        let mut accepted = 0;
        for _ in 0..200_000 {
            let mut f = [0u8; 17];
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
        // Four lengths are tried, so the expectation is about 12 in 200 000.
        assert!(accepted < 40, "{accepted} random frames were accepted as sensors");
    }
}
