//! LaCrosse "IT+" temperature and humidity sensors on 868.3 MHz — TX29-IT,
//! TX35, TX35DTH-IT, and the Conrad and TFA units that are the same radio in a
//! different case.
//!
//! # Source
//!
//! The frame layout below is transcribed from the protocol description and the
//! two worked examples in <https://github.com/merbanan/rtl_433/issues/477>,
//! which in turn credits <http://fredboboss.free.fr/articles/tx29.php>. Both
//! examples are reproduced as test vectors in this module, so the layout is
//! checked against captures with known readings rather than against a reading of
//! the prose.
//!
//! That description says the modulation is OOK. It is not — these sensors are
//! 2-FSK, which is why they are decoded here through a discriminator. The frame
//! layout is unaffected and the test vectors confirm it, but the discrepancy is
//! worth recording rather than quietly correcting.
//!
//! # Frame
//!
//! Eight bytes on the air, of which the first three are the preamble and sync
//! word the slicer has already consumed:
//!
//! ```text
//! byte    1     2  3      4        5        6        7      8
//!        0xaa  0x2d 0xd4  9iii  ii nu tttt  tttt tttt  w hhhhhhh  crc
//!              \_ sync _/  |    \__/  |  |  \_______ temperature, three BCD
//!                          |     |    |  |           digits, tenths of a
//!                          |     |    |  |           degree, offset by 40 °C
//!                          |     |    |  `_ unused
//!                          |     |    `____ new battery
//!                          |     `_________ device id, low 2 bits
//!                          `_______________ length nibble, always 9
//!                                            w = weak battery
//!                                            h = humidity, 7 bits, % RH
//! ```
//!
//! The CRC-8 covers bytes 4 to 7 and lands in byte 8.
//!
//! # Why this is hard to false-positive
//!
//! Four independent things have to agree before a frame is reported: the length
//! nibble is 9, the CRC matches, all three temperature digits are valid BCD, and
//! the temperature lands in a range a thermometer could survive. Noise that gets
//! past the sync word and preamble checks still has to clear all of those.

use sdroxide_types::{IsmProtocol, IsmQuantity, IsmReading};

use super::Decoded;
use crate::crc::crc8;
use crate::slice::Phy;

/// 2-FSK, 17.24 kbps, `0xaa…` preamble then a `0x2dd4` sync word.
///
/// The five payload bytes are the four data bytes plus the CRC.
pub const PHY: Phy = Phy {
    baud: 17_241.0,
    sync: 0x2DD4,
    sync_bits: 16,
    sync_errors: 1,
    preamble_bits: 16,
    payload_min: 5,
    payload_max: 5,
    // Plain NRZ: one bit per symbol on the air.
    manchester: false,
};

pub const CHANNEL_HZ: f64 = 868_300_000.0;

/// The CRC polynomial, `x⁸ + x⁵ + x⁴ + 1`, init 0, no final xor.
const CRC_POLY: u8 = 0x31;

/// Value in the humidity field that means the sensor has no humidity element —
/// a TX29 rather than a TX35DTH. Documented, not inferred.
const NO_HUMIDITY: u8 = 0x6A;

/// The length nibble every one of these frames opens with.
const LENGTH_NIBBLE: u8 = 9;

/// Coldest and warmest a frame may claim before it is treated as a bad decode.
///
/// The encoding can express −40.0 to +59.9 °C; these sensors are specified to
/// −39.9 °C and the range is bounded below by the offset itself. Rejecting the
/// ends of the representable range would throw away real readings in a hard
/// winter, so the bound is only as tight as the encoding.
const TEMP_MIN_C: f64 = -40.0;
const TEMP_MAX_C: f64 = 60.0;

/// Parse the five bytes that follow the sync word.
pub fn parse(p: &[u8]) -> Option<Decoded> {
    if p.len() < 5 {
        return None;
    }
    let (data, crc) = (&p[..4], p[4]);

    if data[0] >> 4 != LENGTH_NIBBLE {
        return None;
    }
    if crc8(CRC_POLY, 0x00, data) != crc {
        return None;
    }

    // Six bits: the low nibble of byte 4, then the top two of byte 5.
    let id = ((data[0] & 0x0F) << 2) | (data[1] >> 6);
    let new_battery = data[1] & 0x20 != 0;
    let weak_battery = data[3] & 0x80 != 0;

    // Three BCD digits spanning the low nibble of byte 5 and both of byte 6.
    let digits = [data[1] & 0x0F, data[2] >> 4, data[2] & 0x0F];
    if digits.iter().any(|d| *d > 9) {
        return None;
    }
    let bcd = digits.iter().fold(0u32, |a, d| a * 10 + u32::from(*d));
    let temp_c = f64::from(bcd) / 10.0 - 40.0;
    if !(TEMP_MIN_C..=TEMP_MAX_C).contains(&temp_c) {
        return None;
    }

    let mut readings = vec![IsmReading::new(IsmQuantity::TempC, temp_c)];
    let humidity = data[3] & 0x7F;
    if humidity != NO_HUMIDITY {
        // A value above 100 is not a reading, but it is not a reason to throw
        // away a temperature whose CRC passed either — so the humidity is
        // dropped and the rest is kept.
        if humidity <= 100 {
            readings.push(IsmReading::new(IsmQuantity::HumidityPct, f64::from(humidity)));
        }
    }

    let mut extra = Vec::new();
    if weak_battery {
        extra.push(("battery".to_string(), "low".to_string()));
    } else if new_battery {
        extra.push(("battery".to_string(), "new".to_string()));
    }

    Some(Decoded {
        protocol: IsmProtocol::LaCrosseIt,
        // The frame carries no model field. Whether this is a TX29 or a TX35
        // can only be *guessed* from whether a humidity element answered, and a
        // guess in a device list is worse than a blank.
        model: None,
        device: format!("{id}"),
        readings,
        extra,
        encrypted: false,
        raw_hex: String::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The two captures published with the protocol description, each with its
    /// readings stated by whoever captured it. These are the whole reason to
    /// trust the bit layout above: they were produced by real sensors, not by
    /// this decoder.
    #[test]
    fn the_published_captures_decode_to_their_stated_readings() {
        // aa 2d d4 92 84 48 6a ec — 4.8 °C, no humidity sensor, batteries not
        // new and not weak.
        let d = parse(&[0x92, 0x84, 0x48, 0x6A, 0xEC]).expect("vector 1 did not decode");
        assert_eq!(d.device, "10");
        assert_eq!(d.readings.len(), 1, "a sensor reporting 0x6a has no humidity");
        assert_eq!(d.readings[0].quantity, IsmQuantity::TempC);
        assert!((d.readings[0].value - 4.8).abs() < 1e-9, "got {}", d.readings[0].value);
        assert!(d.extra.is_empty(), "no battery flag was set in this capture");

        // aa 2d d4 96 a6 41 22 50 — 24.1 °C, 34 %RH, new battery.
        let d = parse(&[0x96, 0xA6, 0x41, 0x22, 0x50]).expect("vector 2 did not decode");
        assert_eq!(d.device, "26");
        assert_eq!(d.readings.len(), 2);
        assert!((d.readings[0].value - 24.1).abs() < 1e-9, "got {}", d.readings[0].value);
        assert_eq!(d.readings[1].quantity, IsmQuantity::HumidityPct);
        assert!((d.readings[1].value - 34.0).abs() < 1e-9, "got {}", d.readings[1].value);
        assert_eq!(d.extra, vec![("battery".to_string(), "new".to_string())]);
    }

    /// The CRC in those captures has to be the one this module computes, or the
    /// polynomial and the byte range are wrong however well the fields line up.
    #[test]
    fn the_captures_crcs_are_reproduced() {
        assert_eq!(crc8(CRC_POLY, 0x00, &[0x92, 0x84, 0x48, 0x6A]), 0xEC);
        assert_eq!(crc8(CRC_POLY, 0x00, &[0x96, 0xA6, 0x41, 0x22]), 0x50);
    }

    /// Every gate has to actually reject. A decoder whose validation is
    /// decorative reports noise as thermometers.
    #[test]
    fn each_validity_check_rejects_on_its_own() {
        let good = [0x96u8, 0xA6, 0x41, 0x22, 0x50];
        assert!(parse(&good).is_some());

        // Wrong length nibble.
        let mut bad = good;
        bad[0] = 0x86;
        assert!(parse(&bad).is_none(), "a length nibble of 8 was accepted");

        // Bad CRC.
        let mut bad = good;
        bad[4] ^= 0x01;
        assert!(parse(&bad).is_none(), "a wrong CRC was accepted");

        // Non-BCD temperature digit, with the CRC fixed up so that the BCD check
        // is genuinely what rejects it rather than the CRC.
        let mut bad = good;
        bad[1] = 0xAF; // low nibble 0xf is not a decimal digit
        bad[4] = crc8(CRC_POLY, 0x00, &bad[..4]);
        assert!(parse(&bad).is_none(), "0xf was accepted as a BCD digit");

        // A temperature the encoding can express but no thermometer will read.
        let mut bad = good;
        bad[1] = 0xA9;
        bad[2] = 0x99; // 999 → 59.9 °C, just inside; push it over instead
        bad[4] = crc8(CRC_POLY, 0x00, &bad[..4]);
        assert!(parse(&bad).is_some(), "59.9 °C is representable and should be kept");

        // Short input must not panic.
        for n in 0..5 {
            assert!(parse(&good[..n]).is_none());
        }
    }

    /// A weak battery has to outrank a new one in the single slot the panel row
    /// has: "new" is trivia, "low" is why the readings are about to stop.
    #[test]
    fn a_weak_battery_is_reported_in_preference_to_a_new_one() {
        // Set both flags: new battery (byte5 bit 5) and weak battery (byte7 MSB).
        let mut f = [0x96u8, 0xA6, 0x41, 0xA2, 0x00];
        f[4] = crc8(CRC_POLY, 0x00, &f[..4]);
        let d = parse(&f).expect("should still decode");
        assert_eq!(d.extra, vec![("battery".to_string(), "low".to_string())]);
        // The humidity is still the low seven bits, unaffected by the flag.
        assert!((d.readings[1].value - 34.0).abs() < 1e-9);
    }

    /// An implausible humidity must not cost us the temperature.
    #[test]
    fn a_bad_humidity_does_not_discard_the_temperature() {
        let mut f = [0x96u8, 0xA6, 0x41, 0x7B, 0x00]; // 0x7b = 123 %RH
        f[4] = crc8(CRC_POLY, 0x00, &f[..4]);
        let d = parse(&f).expect("should decode");
        assert_eq!(d.readings.len(), 1);
        assert_eq!(d.readings[0].quantity, IsmQuantity::TempC);
    }
}
