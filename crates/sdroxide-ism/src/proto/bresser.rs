//! Bresser Weather Center sensors on 868.3 MHz — the 5-in-1, 6-in-1 and 7-in-1
//! families and the water-leakage sensor, all behind one physical layer.
//!
//! # Source
//!
//! Layout transcribed from the protocol documentation comment in `rtl_433`'s
//! `src/devices/bresser_5in1.c`, which describes the device and publishes
//! seventeen captured frames — several annotated with the readings the sensor was
//! showing. Five of those are reproduced as test vectors below, and the ones with
//! stated readings (37.4 m/s gust, 27.5 m/s average, −7.0 °C, battery low)
//! reproduce exactly. That comment in turn credits
//! <https://github.com/andreafabrizi/BresserWeatherCenter>. No code was taken
//! from either.
//!
//! # Why this one is the safest frame in the band to accept
//!
//! Twenty-six payload bytes, of which the **first thirteen are the bitwise
//! complement of the last thirteen**. That is 104 bits of pure redundancy, on top
//! of a checksum byte holding the population count of the twelve data bytes. A
//! frame passing both is not a coincidence in any quantity of noise this band can
//! produce — which is worth having, because unlike the Fine Offset family there is
//! no CRC here at all.
//!
//! The complement is checked *loosely* — eleven of thirteen bytes rather than all
//! thirteen. One published capture differs in a single bit, and a single bit error
//! is exactly what a 13 dB burst produces; demanding all 104 bits agree would
//! throw away most real frames while the check remains overwhelming at eleven.
//!
//! # Frame
//!
//! ```text
//! byte  0 … 12   13   14   15    16  17    18  19    20  21   22   23 24   25
//!      ~13 … ~25 ck   id   s|typ gust dir|g wind w   temp t    hum  rain    B|t
//! ```
//!
//! Wind gust is plain binary in tenths of a metre per second, nine bits with the
//! top one in the low nibble of byte 17. Wind average and temperature are **BCD**,
//! two digits in one byte plus a hundreds digit in the low nibble of the next —
//! that structure is confirmed by the wind field, whose published 27.5 m/s only
//! comes out of the low nibble, and the temperature field follows it.
//!
//! Rainfall is deliberately not decoded: it is two BCD bytes whose digit order no
//! published capture states a value for, and a rain total is exactly the sort of
//! number that is useless if the digits are the wrong way round.
//!
//! # The 6-in-1 family
//!
//! A different frame behind the same preamble, sync word and symbol rate:
//! eighteen bytes carrying an LFSR-16 digest and an add-to-`0xff` checksum. Both
//! were verified against the four published captures annotated with readings, and
//! both reproduce exactly — which matters, because the add-checksum alone is eight
//! bits and would pass one noise frame in 256.
//!
//! ```text
//! byte 0 1   2 … 5   6      7 … 11   12 13    14   15 16   17
//!      digest  id    type   wind     temp  flags  hum  uv/rain  checksum
//! ```
//!
//! These sensors alternate message types: one carries temperature, humidity and
//! rain, the next carries only wind. The wind-only variant is a *different length*
//! and neither published check validates it at this one, so it is not decoded —
//! and, being rejected by the checks rather than misread, it costs nothing. Wind
//! from the temperature message is left alone too: its nibbles are interleaved and
//! partly inverted, and no capture annotated with a wind speed also validates
//! here, so there is nothing to check an implementation against.
//!
//! Byte 6 says which member of the family sent the frame — a weather station, a
//! thermo-hygro unit, a pool thermometer or a soil probe — and the last two
//! suppress the wind and UV fields rather than sending zeros.
//!
//! # The 7-in-1 family
//!
//! Twenty-five bytes behind the same sync word, whitened by XOR with `0xaa` and
//! checked by an LFSR-16 digest with a different key. Every field is BCD, and the
//! documentation for it is unusually good: seven captures are published with the
//! light level and UV index the sensor was showing, and all seven reproduce
//! exactly — see the tests. That is what makes this one safe to read fields from
//! rather than merely identify.
//!
//! It is also the **weakest frame in this module**, and worth knowing as such: a
//! sixteen-bit digest and nothing else, where the 5-in-1 carries around a hundred
//! bits of redundancy. One random frame in 65 536 passes it. Everything the format
//! offers on top of that is taken — the sensor-type nibble must be one that is
//! documented, every BCD field must be wholly decimal or wholly `0xf`, and a value
//! outside the range its sensor can produce fails the whole frame rather than
//! being dropped on its own — which together get it to roughly one in 200 000. A
//! device reported once is still worth treating as provisional; that is what the
//! heard-count in the panel is for.
//!
//! # The leakage sensor
//!
//! Eight bytes, and the only member of the family with a real CRC — CRC-16/XMODEM
//! over five of its bytes. Fifteen captures are published with the channel, alarm,
//! battery and startup state the sensor was in, and all fifteen validate.
//!
//! The flags byte is *outside* the CRC, which is worth knowing: the alarm bit
//! itself is unprotected. It is not unguarded, though — the same byte carries the
//! bit's own complement and a nibble that is always zero, so five bits of it are
//! checkable, and this module checks them.

use sdroxide_types::{IsmProtocol, IsmQuantity, IsmReading};

use super::Decoded;
use crate::crc::crc16;
use crate::slice::Phy;

/// 2-FSK, 8.22 kbaud, `0xaa…` preamble then `0x2dd4`.
///
/// The rate is what makes this family invisible to a decoder that assumes the
/// 17.24 kbaud of the Fine Offset sensors it shares the channel and the sync word
/// with — see the note on symbol-rate candidates in [`crate::slice::frames`].
///
/// The length range spans the whole family: eight bytes for a leakage sensor,
/// eighteen for a 6-in-1, twenty-five for a 7-in-1 and twenty-six for a 5-in-1.
/// [`parse`] tries each at the length it expects.
pub const PHY: Phy = Phy {
    baud: 8_220.0,
    sync: 0x2DD4,
    sync_bits: 16,
    sync_errors: 1,
    preamble_bits: 16,
    payload_min: 8,
    payload_max: 26,
    // Plain NRZ: one bit per symbol on the air.
    manchester: false,
};

pub const CHANNEL_HZ: f64 = 868_300_000.0;

/// Complement bytes that must agree, of thirteen. See the module note.
const MIN_COMPLEMENT: usize = 11;

/// Sensor types that are a rain gauge rather than the 5-in-1 array, and so carry
/// a payload this module does not read.
const RAIN_GAUGE_TYPES: [u8; 3] = [0x9, 0xA, 0xB];

/// Two BCD digits, or `None` if either nibble is not a decimal digit.
fn bcd(b: u8) -> Option<u32> {
    let (hi, lo) = (b >> 4, b & 0x0F);
    (hi <= 9 && lo <= 9).then_some(u32::from(hi) * 10 + u32::from(lo))
}

/// A whole BCD field, from its nibbles in transmission order.
///
/// Three outcomes, and the third is what keeps the 7-in-1 honest:
///
/// - `Some(Some(v))` — every nibble is a decimal digit, and this is the value.
/// - `Some(None)` — every nibble is `0xf`: the sensor saying it has no such
///   measurement, which is how a 3-in-1 reports having no anemometer.
/// - `None` — a mix of the two, which is not something these sensors transmit.
///
/// That last case is worth refusing the whole frame over. The 7-in-1's only check
/// is a sixteen-bit digest, which lets one noise burst in 65 536 through, and this
/// band produces noise bursts by the thousand. Across the thirteen published
/// captures every one of their six BCD fields is wholly decimal or wholly `0xf`,
/// never mixed — so requiring it costs nothing on real traffic and adds around
/// sixteen bits of rejection on random bytes.
fn field(nibbles: &[u32]) -> Option<Option<u32>> {
    if nibbles.iter().all(|&n| n <= 9) {
        Some(Some(nibbles.iter().fold(0, |a, &n| a * 10 + n)))
    } else if nibbles.iter().all(|&n| n == 0x0F) {
        Some(None)
    } else {
        None
    }
}

pub fn parse(p: &[u8]) -> Option<Decoded> {
    // Longest first: a shorter frame's checks cannot accidentally pass on the head
    // of a longer one as easily as the reverse.
    if p.len() >= 26 {
        if let Some(d) = five_in_one(p) {
            return Some(d);
        }
    }
    if p.len() >= 25 {
        if let Some(d) = seven_in_one(p) {
            return Some(d);
        }
    }
    if p.len() >= 18 {
        if let Some(d) = six_in_one(p) {
            return Some(d);
        }
    }
    leakage(p)
}

/// LFSR-16 digest, generator `0x8810` (`poly` here — `gen` is a reserved word in
/// edition 2024), as the 6-in-1 frames carry.
///
/// One pass over the data, most-significant bit first: each set bit folds the
/// running key into the sum, and the key advances by a Galois shift every bit
/// whether or not it was set. Verified against four published captures.
fn lfsr_digest16(data: &[u8], poly: u16, key: u16) -> u16 {
    let mut sum = 0u16;
    let mut k = key;
    for &b in data {
        for bit in (0..8).rev() {
            if (b >> bit) & 1 != 0 {
                sum ^= k;
            }
            k = if k & 1 != 0 { (k >> 1) ^ poly } else { k >> 1 };
        }
    }
    sum
}

/// Bresser 6-in-1: eighteen bytes, digest plus add-checksum.
fn six_in_one(p: &[u8]) -> Option<Decoded> {
    if p.len() < 18 {
        return None;
    }
    let body = &p[2..17];

    // Add to 0xff. Cheap, so it goes first.
    if body.iter().fold(0u8, |a, &b| a.wrapping_add(b)).wrapping_add(p[17]) != 0xFF {
        return None;
    }
    // And the digest, which is what makes accepting the frame safe.
    if lfsr_digest16(body, 0x8810, 0x5412) != (u16::from(p[0]) << 8 | u16::from(p[1])) {
        return None;
    }

    // Temperature is three BCD digits: two in byte 12, the tenth in the high
    // nibble of byte 13, with the sign and the battery flag in its low nibble.
    let tens = bcd(p[12])?;
    let tenth = u32::from(p[13] >> 4);
    if tenth > 9 {
        return None;
    }
    let magnitude = i64::from(tens * 10 + tenth);
    let temp_c = if p[13] & 0x08 != 0 { -magnitude } else { magnitude } as f64 / 10.0;
    if !(-40.0..=80.0).contains(&temp_c) {
        return None;
    }

    // Which member of the family sent this. A thermo-hygro unit and a soil probe
    // have no anemometer, and a soil probe reports its moisture through the
    // humidity byte instead of a percentage.
    let s_type = p[6] >> 4;
    let mut readings = vec![IsmReading::new(IsmQuantity::TempC, temp_c)];

    match (s_type, bcd(p[14])) {
        // Soil probe: the humidity byte is an index into a sixteen-step scale,
        // not a percentage. Out of range means the probe sent no reading.
        (4, Some(h @ 1..=16)) => {
            let pct = MOISTURE_STEPS[h as usize - 1];
            readings.push(IsmReading::new(IsmQuantity::SoilMoisturePct, f64::from(pct)));
        }
        (4, _) => {}
        (_, Some(h)) if h <= 100 => {
            readings.push(IsmReading::new(IsmQuantity::HumidityPct, f64::from(h)));
        }
        _ => {}
    }

    let mut extra = Vec::new();
    // Bit 1 set means the battery is *good*. Inverting this is an easy mistake and
    // an invisible one — it only shows on a sensor that is actually flat — so the
    // two captures published with their battery state are what pins it, and the
    // test below asserts against both.
    if p[13] & 0x02 == 0 {
        extra.push(("battery".to_string(), "low".to_string()));
    }

    Some(Decoded {
        protocol: IsmProtocol::Bresser,
        model: Some(
            match s_type {
                1 => "6-in-1",
                2 => "thermo-hygro",
                3 => "pool thermometer",
                4 => "soil probe",
                _ => "6-in-1",
            }
            .to_string(),
        ),
        // Four bytes of it, which is what distinguishes two of these in one garden.
        device: format!("{:02x}{:02x}{:02x}{:02x}", p[2], p[3], p[4], p[5]),
        readings,
        extra,
        encrypted: false,
        raw_hex: String::new(),
    })
}

/// The soil probe's sixteen-step moisture scale, as percentages.
///
/// The probe measures a dielectric constant and reports which of sixteen bands it
/// fell in; these are the percentages the manufacturer's own display shows for
/// each. Not a linear scale, so it has to be a table.
const MOISTURE_STEPS: [u8; 16] = [0, 7, 13, 20, 27, 33, 40, 47, 53, 60, 67, 73, 80, 87, 93, 99];

/// Bresser 5-in-1: twenty-six bytes, complement copy plus population count.
fn five_in_one(p: &[u8]) -> Option<Decoded> {
    if p.len() < 26 {
        return None;
    }

    // The checksum: the population count of the twelve data bytes.
    let ck: u32 = p[14..26].iter().map(|b| b.count_ones()).sum();
    if ck != u32::from(p[13]) {
        return None;
    }

    // And the complement copy, loosely — see the module note.
    let agree = (0..13).filter(|&i| p[i] == !p[i + 13]).count();
    if agree < MIN_COMPLEMENT {
        return None;
    }

    let id = p[14];
    let sensor_type = p[15] & 0x0F;
    let battery_low = (p[25] >> 4) & 0x08 != 0;

    let mut extra = Vec::new();
    if battery_low {
        extra.push(("battery".to_string(), "low".to_string()));
    }

    if RAIN_GAUGE_TYPES.contains(&sensor_type) {
        // A real, validated Bresser frame whose payload means something else.
        // Reported rather than dropped: knowing a rain gauge is there is worth
        // having, and inventing a rainfall from an unverified digit order is not.
        extra.push(("payload".to_string(), "rain gauge, not decoded".to_string()));
        return Some(Decoded {
            protocol: IsmProtocol::Bresser,
            model: Some("rain gauge".to_string()),
            device: format!("{id:02x}"),
            readings: Vec::new(),
            extra,
            encrypted: false,
            raw_hex: String::new(),
        });
    }

    let mut readings = Vec::new();

    // Temperature: two BCD digits plus a hundreds digit, sign in the low nibble
    // of the last byte.
    if let Some(tens) = bcd(p[20]) {
        let hundreds = u32::from(p[21] & 0x0F);
        if hundreds <= 9 {
            let tenths = i64::from(hundreds * 100 + tens);
            let signed = if p[25] & 0x0F != 0 { -tenths } else { tenths };
            let temp_c = signed as f64 / 10.0;
            if (-40.0..=80.0).contains(&temp_c) {
                readings.push(IsmReading::new(IsmQuantity::TempC, temp_c));
            }
        }
    }

    if let Some(h) = bcd(p[22]) {
        if h <= 100 {
            readings.push(IsmReading::new(IsmQuantity::HumidityPct, f64::from(h)));
        }
    }

    // Wind average, BCD, same two-digits-plus-hundreds shape as the temperature.
    if let Some(tens) = bcd(p[18]) {
        let hundreds = u32::from(p[19] & 0x0F);
        if hundreds <= 9 {
            let ms = f64::from(hundreds * 100 + tens) / 10.0;
            if ms <= MAX_WIND_MS {
                readings.push(IsmReading::new(IsmQuantity::WindAvgMs, ms));
            }
        }
    }

    // Gust, plain binary in tenths, nine bits.
    let gust = f64::from((u32::from(p[17] & 0x0F) << 8) | u32::from(p[16])) / 10.0;
    if gust <= MAX_WIND_MS {
        readings.push(IsmReading::new(IsmQuantity::WindGustMs, gust));
    }

    // Sixteen compass points.
    readings.push(IsmReading::new(IsmQuantity::WindDirDeg, f64::from(p[17] >> 4) * 22.5));

    // A frame that validated but yielded nothing readable is a decode failure, not
    // a device with no measurements.
    if readings.len() < 2 {
        return None;
    }

    Some(Decoded {
        protocol: IsmProtocol::Bresser,
        model: Some("5-in-1".to_string()),
        device: format!("{id:02x}"),
        readings,
        extra,
        encrypted: false,
        raw_hex: String::new(),
    })
}

/// Bresser 7-in-1: twenty-five whitened bytes behind an LFSR-16 digest.
///
/// ```text
/// byte  0 1  2 3   4  5   6  7  8   9   10 11 12  13  14 15  16  17 18 19  20 21  22 … 24
///      digest  id  wdir  ?  gust  wavg    rain     ?   temp|f hum   light    uv    trailer
/// ```
///
/// Byte 6 is *not* whitened — it carries the sensor type, a startup flag and the
/// channel, and it is the byte that says whether the rest of the frame is weather
/// at all. The air-quality members of this family put a CO₂ or particulate
/// reading where the wind goes, so they are identified and left unread rather
/// than reported as a very still, very cold garden.
///
/// Everything else is BCD, and an absent field arrives as `0xf` nibbles — which is
/// how a 3-in-1 with no anemometer says so.
fn seven_in_one(p: &[u8]) -> Option<Decoded> {
    if p.len() < 25 {
        return None;
    }
    let m: Vec<u8> = p[..25].iter().map(|b| b ^ 0xAA).collect();

    // The digest is over the de-whitened frame and is compared against the
    // de-whitened check bytes; the constant absorbs the whitening either way.
    let chk = (u16::from(m[0]) << 8) | u16::from(m[1]);
    if lfsr_digest16(&m[2..25], 0x8810, 0xBA95) ^ chk != 0x6DF1 {
        return None;
    }

    // From the *raw* byte, which this family leaves unwhitened.
    let s_type = p[6] >> 4;
    let device = format!("{:02x}{:02x}", m[2], m[3]);
    let named = |name: &str, extra: Vec<(String, String)>| Decoded {
        protocol: IsmProtocol::Bresser,
        model: Some(name.to_string()),
        device: device.clone(),
        readings: Vec::new(),
        extra,
        encrypted: false,
        raw_hex: String::new(),
    };

    let model = match s_type {
        1 => "7-in-1",
        12 => "3-in-1",
        13 => "8-in-1",
        // Real frames of this family whose payload is something else entirely.
        // Worth reporting as present; the readings themselves are air quality
        // rather than weather, and out of this decoder's scope.
        //
        // Their value field still has to be valid BCD before the frame is
        // believed. Identifying a device costs no field checks of its own, and on
        // a frame whose only check is sixteen bits that leaves nothing but the
        // digest between this branch and a noise burst.
        10 | 11 => {
            bcd(m[4])?;
            bcd(m[5])?;
            let what = if s_type == 10 { ("CO2", "CO2 ppm") } else { ("HCHO/VOC", "HCHO ppb") };
            return Some(named(
                what.0,
                vec![("payload".to_string(), format!("{}, not decoded", what.1))],
            ));
        }
        // An undocumented type, on a frame whose only check is sixteen bits. Both
        // halves of that matter: nothing says what this payload means, and
        // accepting it would be the widest door in the module for a noise burst
        // that happened to hit the digest.
        _ => return None,
    };

    // Each field, as its nibbles in transmission order.
    let n = |b: u8, high: bool| -> u32 { u32::from(if high { b >> 4 } else { b & 0x0F }) };
    // The direction's third nibble shares a byte with a field this module does not
    // read, and on a sensor with no vane that byte reads zero while the two
    // nibbles the direction owns read `0xf`. So absence is marked by the byte the
    // field owns, not by all three of its nibbles.
    let wdir = if m[4] == 0xFF {
        None
    } else {
        Some(field(&[n(m[4], true), n(m[4], false), n(m[5], true)])??)
    };
    let gust = field(&[n(m[7], true), n(m[7], false), n(m[8], true)])?;
    let wavg = field(&[n(m[8], false), n(m[9], true), n(m[9], false)])?;
    let rain = field(&[
        n(m[10], true),
        n(m[10], false),
        n(m[11], true),
        n(m[11], false),
        n(m[12], true),
        n(m[12], false),
    ])?;
    let lux = field(&[
        n(m[17], true),
        n(m[17], false),
        n(m[18], true),
        n(m[18], false),
        n(m[19], true),
        n(m[19], false),
    ])?;
    let uv = field(&[n(m[20], true), n(m[20], false), n(m[21], true)])?;

    let mut readings = Vec::new();

    // Temperature wraps rather than carrying a sign bit: three BCD digits cover
    // 0.0–99.9, and anything above 60.0 is that range folded round to negative.
    // A weather frame always carries one, so an absent field is a decode error.
    let raw = field(&[n(m[14], true), n(m[14], false), n(m[15], true)])??;
    let temp_c =
        if raw > 600 { (i64::from(raw) - 1000) as f64 / 10.0 } else { f64::from(raw) / 10.0 };
    if !(-40.0..=80.0).contains(&temp_c) {
        return None;
    }
    readings.push(IsmReading::new(IsmQuantity::TempC, temp_c));

    // And a humidity, likewise.
    let h = bcd(m[16]).filter(|h| *h <= 100)?;
    readings.push(IsmReading::new(IsmQuantity::HumidityPct, f64::from(h)));

    // A field that is valid BCD but outside the range its sensor can produce means
    // the frame was misread, not that this one measurement is odd — so it takes
    // the whole frame with it rather than being quietly dropped. That is the same
    // argument the 5-in-1 makes below, and on a check this thin it is also most of
    // what stands between the parser and a lucky noise burst.
    if let Some(d) = wdir {
        if d >= 360 {
            return None;
        }
        readings.push(IsmReading::new(IsmQuantity::WindDirDeg, f64::from(d)));
    }
    for (v, q) in [(gust, IsmQuantity::WindGustMs), (wavg, IsmQuantity::WindAvgMs)] {
        if let Some(v) = v {
            let ms = f64::from(v) / 10.0;
            if ms > MAX_WIND_MS {
                return None;
            }
            readings.push(IsmReading::new(q, ms));
        }
    }
    if let Some(r) = rain {
        readings.push(IsmReading::new(IsmQuantity::RainMm, f64::from(r) / 10.0));
    }
    if let Some(l) = lux {
        // The sensor saturates at 200 000 lx — the value the brightest published
        // capture reports, and about twice the brightest daylight on Earth.
        if l > 200_000 {
            return None;
        }
        readings.push(IsmReading::new(IsmQuantity::LuxLx, f64::from(l)));
    }
    if let Some(u) = uv {
        if u > 200 {
            return None;
        }
        readings.push(IsmReading::new(IsmQuantity::UvIndex, f64::from(u) / 10.0));
    }

    let mut extra = Vec::new();
    // Both flag bits together, not either one — a single bit here is something
    // else, and the published captures all have exactly one of them set.
    if m[15] & 0x06 == 0x06 {
        extra.push(("battery".to_string(), "low".to_string()));
    }

    Some(Decoded {
        protocol: IsmProtocol::Bresser,
        model: Some(model.to_string()),
        device,
        readings,
        extra,
        encrypted: false,
        raw_hex: String::new(),
    })
}

/// Bresser water-leakage sensor.
///
/// ```text
/// byte  0  1   2 … 5    6            7
///      crc-16   id      type|q|chan   alarm|~alarm|batt|0000
/// ```
///
/// The CRC covers bytes 2–6 only, so the flags byte carrying the alarm is outside
/// it. Five bits of that byte are checkable anyway — the alarm bit is repeated
/// inverted, and the low nibble is always zero — and all five are checked here,
/// because "your floor is wet" is not a message to deliver on an unvalidated bit.
fn leakage(p: &[u8]) -> Option<Decoded> {
    if p.len() < 8 {
        return None;
    }
    if crc16(0x1021, 0x0000, &p[2..7]) != (u16::from(p[0]) << 8) | u16::from(p[1]) {
        return None;
    }
    // This is the only sensor type published for this frame, and requiring it adds
    // four bits to a check that has to carry an unprotected alarm bit.
    if p[6] >> 4 != 5 {
        return None;
    }

    let flags = p[7];
    if flags & 0x0F != 0 {
        return None;
    }
    let alarm = flags & 0x80 != 0;
    let no_alarm = flags & 0x40 != 0;
    if alarm == no_alarm {
        return None;
    }

    let mut extra = vec![("leak".to_string(), if alarm { "ALARM" } else { "dry" }.to_string())];
    if flags & 0x30 == 0 {
        extra.push(("battery".to_string(), "low".to_string()));
    }
    extra.push(("channel".to_string(), (p[6] & 0x07).to_string()));

    Some(Decoded {
        protocol: IsmProtocol::Bresser,
        model: Some("leakage".to_string()),
        device: format!("{:02x}{:02x}{:02x}{:02x}", p[2], p[3], p[4], p[5]),
        // No quantity: this sensor measures nothing, it only says wet or dry.
        readings: Vec::new(),
        extra,
        encrypted: false,
        raw_hex: String::new(),
    })
}

/// Fastest wind worth believing, m/s. The encoding reaches 99.9; the strongest
/// surface gust ever recorded is near 113, and a domestic anemometer reporting
/// anything close is reporting a decode error.
const MAX_WIND_MS: f64 = 80.0;

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(hex: &str) -> Vec<u8> {
        hex.split_whitespace().map(|b| u8::from_str_radix(b, 16).unwrap()).collect()
    }

    fn get(d: &Decoded, q: IsmQuantity) -> Option<f64> {
        d.readings.iter().find(|r| r.quantity == q).map(|r| r.value)
    }

    /// The published capture annotated "Large Wind Values, Gust=37.4m/s
    /// Avg=27.5m/s". Both come out exactly, which is what pins the BCD
    /// hundreds-digit nibble — and with it the temperature field, which has the
    /// same shape.
    #[test]
    fn the_published_wind_capture_reproduces_its_stated_speeds() {
        let f =
            frame("e3 fd 7f 89 7e 8a ed 68 fe af 9b fd ff 1c 02 80 76 81 75 12 97 01 50 64 02 00");
        let d = parse(&f).expect("the sample did not decode");
        assert_eq!(d.protocol, IsmProtocol::Bresser);
        assert_eq!(d.model.as_deref(), Some("5-in-1"));
        assert_eq!(d.device, "02");
        assert!((get(&d, IsmQuantity::WindGustMs).unwrap() - 37.4).abs() < 1e-9);
        assert!((get(&d, IsmQuantity::WindAvgMs).unwrap() - 27.5).abs() < 1e-9);
        assert!((get(&d, IsmQuantity::WindDirDeg).unwrap() - 180.0).abs() < 1e-9);
        assert!((get(&d, IsmQuantity::HumidityPct).unwrap() - 50.0).abs() < 1e-9);
        assert!(d.extra.is_empty(), "battery was fine in this capture");
    }

    /// The captures annotated with a battery state and a temperature sign.
    #[test]
    fn the_published_temperature_captures_reproduce_their_signs_and_batteries() {
        // "low batt -ve temp -7.0C"
        let f =
            frame("ed a1 ff ff 1f ff ef 8f ff d6 df ff 77 12 5e 00 00 e0 00 10 70 00 29 20 00 88");
        let d = parse(&f).expect("did not decode");
        assert!((get(&d, IsmQuantity::TempC).unwrap() + 7.0).abs() < 1e-9);
        assert!((get(&d, IsmQuantity::HumidityPct).unwrap() - 29.0).abs() < 1e-9);
        assert_eq!(d.extra, vec![("battery".to_string(), "low".to_string())]);

        // "low batt +ve temp"
        let f =
            frame("ef a1 ff ff 1f ff ef dc ff de df ff 7f 10 5e 00 00 e0 00 10 23 00 21 20 00 80");
        let d = parse(&f).expect("did not decode");
        assert!(get(&d, IsmQuantity::TempC).unwrap() > 0.0, "sign bit clear means positive");
        assert!((get(&d, IsmQuantity::TempC).unwrap() - 2.3).abs() < 1e-9);
        assert_eq!(d.extra, vec![("battery".to_string(), "low".to_string())]);

        // "good batt -ve temp" — and this is the one whose complement copy differs
        // by a single bit, so it is also the test that the loose check exists for.
        let f =
            frame("ec 91 ff ff 3f fb ef e7 fe ad ed ff f7 13 6e 00 00 e0 04 10 18 01 52 12 00 08");
        let agree = (0..13).filter(|&i| f[i] == !f[i + 13]).count();
        assert_eq!(agree, 12, "this capture is one byte out; that is the point of it");
        let d = parse(&f).expect("a one-bit slip must not lose the frame");
        assert!(get(&d, IsmQuantity::TempC).unwrap() < 0.0);
        assert!(d.extra.is_empty(), "battery was fine here");
    }

    /// A rain gauge is a different payload behind the same framing. It has to be
    /// reported as present without inventing a rainfall.
    #[test]
    fn a_rain_gauge_is_reported_but_not_read() {
        let f =
            frame("ed ee 46 ff ff ff ef 9f ff 8b 7d eb ff 12 11 b9 00 00 00 10 60 00 74 82 14 00");
        let d = parse(&f).expect("did not decode");
        assert_eq!(d.model.as_deref(), Some("rain gauge"));
        assert_eq!(d.device, "11");
        assert!(d.readings.is_empty(), "the rain digit order is unverified");
        assert!(d.extra.iter().any(|(k, _)| k == "payload"));
    }

    /// Both checks have to bite, independently.
    #[test]
    fn each_check_rejects_on_its_own() {
        let good =
            frame("e3 fd 7f 89 7e 8a ed 68 fe af 9b fd ff 1c 02 80 76 81 75 12 97 01 50 64 02 00");
        assert!(parse(&good).is_some());

        // Wrong checksum only.
        let mut bad = good.clone();
        bad[13] ^= 0x01;
        assert!(parse(&bad).is_none(), "a wrong population count was accepted");

        // Complement broken in three bytes, with the checksum left valid — the
        // data half is untouched, so only the redundancy check can reject it.
        let mut bad = good.clone();
        for i in 0..3 {
            bad[i] ^= 0xFF;
        }
        assert!(parse(&bad).is_none(), "a broken complement copy was accepted");

        for n in 0..26 {
            assert!(parse(&good[..n]).is_none(), "{n} bytes should not parse");
        }
    }

    /// The four published 6-in-1 captures annotated with readings. Both of that
    /// frame's checks — the LFSR-16 digest and the add-to-0xff checksum — have to
    /// reproduce, and the temperature and humidity have to come out as stated.
    #[test]
    fn the_published_6in1_captures_reproduce_their_readings() {
        for hex in [
            "5e aa 18 80 02 c3 18 fa 8f fb 27 68 11 84 81 ff f0 72 00",
            "f8 2e 18 80 02 c3 18 fc c6 fd 26 38 11 84 81 ff f0 68 00",
            "21 e8 18 80 02 c3 18 fb 9c fc 33 08 11 84 81 ff f0 b7 f8",
            "5c e4 18 80 02 c3 18 fb ba fc 26 98 11 84 81 ff f0 16 00",
        ] {
            let f = frame(hex);
            let d = parse(&f).unwrap_or_else(|| panic!("{hex} did not decode"));
            assert_eq!(d.protocol, IsmProtocol::Bresser);
            assert_eq!(d.model.as_deref(), Some("6-in-1"));
            assert_eq!(d.device, "188002c3", "the four-byte id");
            assert!(
                (get(&d, IsmQuantity::TempC).unwrap() - 11.8).abs() < 1e-9,
                "temp was {:?}",
                get(&d, IsmQuantity::TempC)
            );
            assert!((get(&d, IsmQuantity::HumidityPct).unwrap() - 81.0).abs() < 1e-9);
        }
    }

    /// The digest is the check that makes an 8-bit add-checksum safe to trust, so
    /// it has to actually reject.
    #[test]
    fn the_6in1_digest_rejects_a_tampered_frame() {
        let good = frame("5e aa 18 80 02 c3 18 fa 8f fb 27 68 11 84 81 ff f0 72 00");
        assert!(parse(&good).is_some());

        // Move a data byte and repair the add-checksum, so only the digest can
        // catch it. This is the case an add-checksum alone lets through.
        let mut bad = good.clone();
        bad[12] = 0x22;
        let body: u8 = bad[2..17].iter().fold(0u8, |a, &b| a.wrapping_add(b));
        bad[17] = 0xFFu8.wrapping_sub(body);
        assert_eq!(
            bad[2..17].iter().fold(0u8, |a, &b| a.wrapping_add(b)).wrapping_add(bad[17]),
            0xFF,
            "the add-checksum must be valid, or this tests the wrong thing"
        );
        assert!(parse(&bad).is_none(), "the digest failed to catch a tampered payload");

        // And the add-checksum alone must also bite.
        let mut bad = good.clone();
        bad[17] ^= 0x01;
        assert!(parse(&bad).is_none());
    }

    /// The wind-only alternating message is a different length and validates at
    /// neither check here. It must be rejected, not misread as a temperature.
    #[test]
    fn the_wind_only_message_is_rejected_rather_than_misread() {
        for hex in [
            "ae d1 18 80 02 c3 18 fa 8d fb 26 78 ff ff ff fe 02 db f0",
            "c4 7d 18 80 02 c3 18 fc 78 fd 29 28 ff ff ff fe 03 97 f0",
            "28 1e 18 80 02 c3 18 fb b7 fc 26 58 ff ff ff fe 02 c3 f0",
        ] {
            assert!(parse(&frame(hex)).is_none(), "{hex} should not have decoded");
        }
    }

    /// The seven published 7-in-1 captures. Four state the light level the sensor
    /// was showing and three also state its UV index, and every one of them has to
    /// come out exactly — that is what makes the BCD digit order here proven
    /// rather than assumed, unlike the 5-in-1 rain field.
    #[test]
    fn the_published_7in1_captures_reproduce_their_stated_light_and_uv() {
        for (hex, lux, uv) in [
            ("10b8b4a5a3ca10aaaaaaaaaaaaaa8bcacbaaaa2aaaaaaaaaaa", 80.0, Some(0.0)),
            ("543bb4a5a3ca10aaaaaaaaaaaaaa8bcacbaaaa28aaaaaaaaaa", 82.0, Some(0.0)),
            ("2492b4a5a3ca10aaaaaaaaaaaaaa8bdacbaaaa2daaaaaaaaaa", 87.0, Some(0.0)),
            ("9a59b4a5a3da10aaaaaaaaaaaaaa8bdac8afea28a8caaaaaaa", 54_082.0, Some(2.6)),
            ("fe15b4a5a3da10aaaaaaaaaaaaaa8bdacbba382aacdaaaaaaa", 109_280.0, Some(6.7)),
            ("2544b4a5a32a10aaaaaaaaaaaaaa8bdac88aaaaabeaaaaaaaa", 200_000.0, Some(14.0)),
            ("631d05c09e9a18abaabaaaaaaaaa8adacbacff9cafcaaaaaaa", 65_536.0, Some(5.6)),
        ] {
            let f: Vec<u8> = (0..hex.len() / 2)
                .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap())
                .collect();
            let d = parse(&f).unwrap_or_else(|| panic!("{hex} did not decode"));
            assert_eq!(d.protocol, IsmProtocol::Bresser);
            assert_eq!(d.model.as_deref(), Some("7-in-1"), "{hex}");
            assert!(
                (get(&d, IsmQuantity::LuxLx).unwrap() - lux).abs() < 1e-6,
                "{hex}: light was {:?}, expected {lux}",
                get(&d, IsmQuantity::LuxLx)
            );
            if let Some(uv) = uv {
                assert!(
                    (get(&d, IsmQuantity::UvIndex).unwrap() - uv).abs() < 1e-9,
                    "{hex}: uv was {:?}, expected {uv}",
                    get(&d, IsmQuantity::UvIndex)
                );
            }
            // Six of the seven are one sensor logged over a few minutes indoors.
            let t = get(&d, IsmQuantity::TempC).unwrap();
            assert!((0.0..=30.0).contains(&t), "{hex}: {t} °C");
        }
    }

    /// A 3-in-1 has no anemometer and no light sensor, and says so by filling
    /// those fields with `0xf` nibbles. They have to come back absent — a decoder
    /// that read the digits anyway would report a 166 m/s gale.
    #[test]
    fn a_7in1_frame_with_absent_fields_omits_them_rather_than_reading_ffff() {
        let hex = "a6b2bf4b55aac8555555aa3e8f3eabca3355555555555555aa";
        let f: Vec<u8> = (0..hex.len() / 2)
            .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap())
            .collect();
        let d = parse(&f).expect("did not decode");
        assert_eq!(d.model.as_deref(), Some("3-in-1"), "byte 6 is read unwhitened");
        assert!((get(&d, IsmQuantity::TempC).unwrap() - 1.6).abs() < 1e-9);
        assert!((get(&d, IsmQuantity::HumidityPct).unwrap() - 99.0).abs() < 1e-9);
        for absent in [
            IsmQuantity::WindGustMs,
            IsmQuantity::WindAvgMs,
            IsmQuantity::WindDirDeg,
            IsmQuantity::LuxLx,
            IsmQuantity::UvIndex,
        ] {
            assert!(get(&d, absent).is_none(), "{absent:?} was invented from 0xf nibbles");
        }
    }

    /// The 7-in-1 digest is the only thing standing between this length and noise,
    /// so it has to reject a single changed byte.
    #[test]
    fn the_7in1_digest_rejects_a_tampered_frame() {
        let hex = "9a59b4a5a3da10aaaaaaaaaaaaaa8bdac8afea28a8caaaaaaa";
        let good: Vec<u8> = (0..hex.len() / 2)
            .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap())
            .collect();
        assert!(parse(&good).is_some());
        for i in 2..25 {
            let mut bad = good.clone();
            bad[i] ^= 0x10;
            assert!(parse(&bad).is_none(), "a changed byte {i} survived the digest");
        }
    }

    /// The two 6-in-1 soil captures published with their battery state. These are
    /// the vectors that pin the battery bit's *polarity*, which was inverted here
    /// until they were checked — a set bit means good, not low.
    #[test]
    fn the_soil_captures_reproduce_their_stated_battery_state() {
        for (hex, low) in [
            ("e3ae1870079341ffffff0000221201fff279", false),
            ("3d2c1870079341ffffff0000219001fff2fc", true),
        ] {
            let f: Vec<u8> = (0..hex.len() / 2)
                .map(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap())
                .collect();
            let d = parse(&f).unwrap_or_else(|| panic!("{hex} did not decode"));
            assert_eq!(d.model.as_deref(), Some("soil probe"), "sensor type 4");
            assert_eq!(d.device, "18700793");
            assert_eq!(
                d.extra.iter().any(|(k, v)| k == "battery" && v == "low"),
                low,
                "{hex}: battery polarity is the wrong way round"
            );
            // Moisture, not humidity: index 1 of the sixteen-step scale.
            assert!(get(&d, IsmQuantity::SoilMoisturePct).is_some());
            assert!(get(&d, IsmQuantity::HumidityPct).is_none(), "a probe has no humidity");
            assert!((get(&d, IsmQuantity::SoilMoisturePct).unwrap()).abs() < 1e-9);
        }
        // And the temperatures, which differ between the two captures.
        let f: Vec<u8> = (0..18)
            .map(|i| {
                u8::from_str_radix(&"e3ae1870079341ffffff0000221201fff279"[i * 2..i * 2 + 2], 16)
                    .unwrap()
            })
            .collect();
        assert!((get(&parse(&f).unwrap(), IsmQuantity::TempC).unwrap() - 22.1).abs() < 1e-9);
    }

    /// All fifteen published leakage captures, each with the state the sensor was
    /// in. The alarm bit is outside the CRC, so the point of this test is that the
    /// unprotected byte is nonetheless read correctly in every combination of
    /// alarm, battery and startup that was published.
    #[test]
    fn every_published_leakage_capture_reproduces_its_stated_state() {
        for (hex, ch, alarm, batt_low) in [
            ("C7 70 35 97 04 08 57 70", 7, false, false),
            ("DF 7D 36 49 27 09 56 70", 6, false, false),
            ("9E 30 79 84 33 06 55 70", 5, false, false),
            ("E2 C8 68 27 91 24 54 70", 4, false, false),
            ("B3 DA 55 57 17 40 53 70", 3, false, false),
            ("37 FA 84 73 03 02 52 70", 2, false, false),
            ("27 F3 80 02 52 88 51 70", 1, false, false),
            ("A6 FB 80 02 52 88 59 70", 1, false, false),
            ("A6 FB 80 02 52 88 59 B0", 1, true, false),
            ("C0 10 36 79 37 09 51 70", 1, false, false),
            ("C0 10 36 79 37 09 51 B0", 1, true, false),
            ("71 9C 54 81 72 09 51 40", 1, false, true),
            ("71 9C 54 81 72 09 51 80", 1, true, true),
            ("F0 94 54 81 72 09 59 40", 1, false, true),
            ("F0 94 54 81 72 09 59 80", 1, true, true),
        ] {
            let f = frame(hex);
            let d = parse(&f).unwrap_or_else(|| panic!("{hex} did not decode"));
            assert_eq!(d.model.as_deref(), Some("leakage"), "{hex}");
            assert!(d.readings.is_empty(), "a leak sensor measures nothing");
            let has = |k: &str, v: &str| d.extra.iter().any(|(a, b)| a == k && b == v);
            assert!(has("leak", if alarm { "ALARM" } else { "dry" }), "{hex}: wrong alarm state");
            assert_eq!(
                d.extra.iter().any(|(k, v)| k == "battery" && v == "low"),
                batt_low,
                "{hex}: wrong battery state"
            );
            assert!(has("channel", &ch.to_string()), "{hex}: wrong channel");
        }
    }

    /// Each of the leakage frame's checks has to bite on its own, including the
    /// three that guard the byte the CRC does not cover.
    #[test]
    fn the_leakage_checks_each_reject_on_their_own() {
        let good = frame("27 F3 80 02 52 88 51 70");
        assert!(parse(&good).is_some());

        // A changed id, with the flags byte untouched: only the CRC can catch it.
        let mut bad = good.clone();
        bad[4] ^= 0x01;
        assert!(parse(&bad).is_none(), "a wrong CRC was accepted");

        // The alarm bit set without clearing its complement.
        let mut bad = good.clone();
        bad[7] |= 0x80;
        assert!(parse(&bad).is_none(), "alarm and no-alarm both set was accepted");

        // The nibble that is always zero.
        let mut bad = good.clone();
        bad[7] |= 0x01;
        assert!(parse(&bad).is_none(), "a non-zero flags nibble was accepted");

        // A sensor type this frame is not published for.
        let mut bad = good.clone();
        bad[6] = 0x21;
        let repaired = crate::crc::crc16(0x1021, 0, &bad[2..7]).to_be_bytes();
        bad[0..2].copy_from_slice(&repaired);
        assert!(parse(&bad).is_none(), "an unpublished sensor type was accepted");
    }

    /// Two hundred thousand random frames through each layout, counted separately
    /// — because the four do not offer remotely the same protection and a single
    /// combined number would hide that.
    ///
    /// The 5-in-1 has around a hundred bits of redundancy, the 6-in-1 twenty-four
    /// and the leakage sensor twenty-five, so all three should be exactly zero.
    /// The 7-in-1 has a sixteen-bit digest and nothing else, which is one frame in
    /// 65 536 before any field check — so zero is not a promise its format can
    /// keep, and asserting it would only mean the test was tuned until it passed.
    /// What the field checks buy is measured here instead.
    #[test]
    fn random_frames_are_rejected_at_the_rate_each_layout_can_promise() {
        let mut seed = 0x9E37_79B9_7F4A_7C15u64;
        let (mut five, mut seven, mut six, mut leak) = (0, 0, 0, 0);
        for _ in 0..200_000 {
            let mut f = [0u8; 26];
            for b in f.iter_mut() {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                *b = (seed >> 33) as u8;
            }
            five += usize::from(five_in_one(&f).is_some());
            seven += usize::from(seven_in_one(&f).is_some());
            six += usize::from(six_in_one(&f).is_some());
            leak += usize::from(leakage(&f).is_some());
        }
        assert_eq!(five, 0, "{five} random frames passed the 5-in-1's complement copy");
        assert_eq!(six, 0, "{six} random frames passed the 6-in-1's digest and checksum");
        assert_eq!(leak, 0, "{leak} random frames passed the leakage sensor's CRC-16");
        // Three would be the bare digest. The BCD structure checks take it under
        // one, and a device reported once is what [`IsmReport::count`] is for.
        assert!(seven <= 1, "{seven} random frames passed the 7-in-1; the field checks slipped");
    }

    /// And the same for the 6-in-1 length, whose protection is a 16-bit digest plus
    /// an 8-bit checksum — one in sixteen million, so a few hundred thousand tries
    /// should find none.
    #[test]
    fn random_6in1_length_frames_never_pass() {
        let mut seed = 0x1357_9BDF_0246_8ACEu64;
        let mut accepted = 0;
        for _ in 0..300_000 {
            let mut f = [0u8; 18];
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
        assert_eq!(accepted, 0, "{accepted} random 18-byte frames were accepted");
    }
}
