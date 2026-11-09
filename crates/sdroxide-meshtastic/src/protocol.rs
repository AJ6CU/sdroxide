// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM
//
// The protocol constants in this file (default PSK bytes, preset SF/CR/BW
// tables, port-number assignments, region/band edges) come from the upstream
// Meshtastic firmware at https://github.com/meshtastic/firmware
// (GPL-3.0-or-later).
//
// Protocol constant tables and tiny helpers.

/// Length of the radio header in front of every frame.
pub const HEADER_BYTES: usize = 16;
/// Destination of a broadcast.
pub const BROADCAST_NODE: u32 = 0xFFFF_FFFF;

pub const FLAG_HOP_LIMIT_MASK: u8 = 0x07;
pub const FLAG_WANT_ACK_MASK: u8 = 0x08;
pub const FLAG_VIA_MQTT_MASK: u8 = 0x10;
pub const FLAG_HOP_START_MASK: u8 = 0xE0;
pub const FLAG_HOP_START_SHIFT: u8 = 5;

/// The stock default-channel PSK (the key the firmware uses out of the box
/// for "LongFast"). `simpleN` keys replace the last byte with `N`; `simple0`
/// means no encryption.
pub const DEFAULT_PSK: [u8; 16] = [
    0xd4, 0xf1, 0xbb, 0x3a, 0x20, 0x29, 0x07, 0x59, 0xf0, 0xbc, 0xff, 0xab, 0xcf, 0x4e, 0x69, 0x01,
];

/// The `simpleN` key: the default key with its last byte replaced.
pub fn simple_psk(n: u8) -> [u8; 16] {
    let mut k = DEFAULT_PSK;
    k[15] = n;
    k
}

/// One modem preset of the firmware.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Preset {
    /// Firmware name, `LONG_FAST`.
    pub name: &'static str,
    /// Channel name used in the channel hash, `LongFast`.
    pub channel_name: &'static str,
    pub sf: u8,
    /// Coding-rate denominator, 5..=8.
    pub cr: u8,
    /// Bandwidth below 1 GHz.
    pub bw_hz: u32,
    /// Bandwidth in the 2.4 GHz region.
    pub bw_hz_wide: u32,
    /// Preamble symbols below 1 GHz.
    pub preamble: usize,
    /// Preamble symbols at 2.4 GHz.
    pub preamble_wide: usize,
}

pub const PRESETS: [Preset; 9] = [
    Preset {
        name: "SHORT_TURBO",
        channel_name: "ShortTurbo",
        sf: 7,
        cr: 5,
        bw_hz: 500_000,
        bw_hz_wide: 1_625_000,
        preamble: 16,
        preamble_wide: 12,
    },
    Preset {
        name: "SHORT_FAST",
        channel_name: "ShortFast",
        sf: 7,
        cr: 5,
        bw_hz: 250_000,
        bw_hz_wide: 812_000,
        preamble: 16,
        preamble_wide: 12,
    },
    Preset {
        name: "SHORT_SLOW",
        channel_name: "ShortSlow",
        sf: 8,
        cr: 5,
        bw_hz: 250_000,
        bw_hz_wide: 812_000,
        preamble: 16,
        preamble_wide: 12,
    },
    Preset {
        name: "MEDIUM_FAST",
        channel_name: "MediumFast",
        sf: 9,
        cr: 5,
        bw_hz: 250_000,
        bw_hz_wide: 812_000,
        preamble: 16,
        preamble_wide: 12,
    },
    Preset {
        name: "MEDIUM_SLOW",
        channel_name: "MediumSlow",
        sf: 10,
        cr: 5,
        bw_hz: 250_000,
        bw_hz_wide: 812_000,
        preamble: 16,
        preamble_wide: 12,
    },
    Preset {
        name: "LONG_FAST",
        channel_name: "LongFast",
        sf: 11,
        cr: 5,
        bw_hz: 250_000,
        bw_hz_wide: 812_000,
        preamble: 16,
        preamble_wide: 12,
    },
    Preset {
        name: "LONG_MODERATE",
        channel_name: "LongMod",
        sf: 11,
        cr: 8,
        bw_hz: 125_000,
        bw_hz_wide: 406_000,
        preamble: 16,
        preamble_wide: 12,
    },
    Preset {
        name: "LONG_SLOW",
        channel_name: "LongSlow",
        sf: 12,
        cr: 8,
        bw_hz: 125_000,
        bw_hz_wide: 406_000,
        preamble: 16,
        preamble_wide: 12,
    },
    Preset {
        name: "LONG_TURBO",
        channel_name: "LongTurbo",
        sf: 11,
        cr: 8,
        bw_hz: 500_000,
        bw_hz_wide: 1_625_000,
        preamble: 16,
        preamble_wide: 12,
    },
];

/// The default preset, `LONG_FAST`.
pub const DEFAULT_PRESET: &Preset = &PRESETS[5];

/// Lower-case name with `_`, `-` and spaces dropped, for tolerant matching.
fn squash(s: &str) -> String {
    s.chars().filter(|c| !matches!(c, '_' | '-' | ' ')).map(|c| c.to_ascii_lowercase()).collect()
}

/// Look a preset up by `LongFast`, `LONG_FAST`, `long-fast`, `longfast`...
pub fn lookup_preset(name: &str) -> Option<&'static Preset> {
    let want = squash(name);
    PRESETS.iter().find(|p| squash(p.name) == want || squash(p.channel_name) == want)
}

/// The preset matching a (SF, CR, bandwidth) triple, if any.
pub fn preset_for(sf: u8, cr: u8, bw_hz: u32) -> Option<&'static Preset> {
    PRESETS.iter().find(|p| p.sf == sf && p.cr == cr && (p.bw_hz == bw_hz || p.bw_hz_wide == bw_hz))
}

/// A regulatory region: the band the channel slots are cut from.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Region {
    pub name: &'static str,
    pub lo_mhz: f64,
    pub hi_mhz: f64,
    /// 2.4 GHz: use the wide bandwidths of the preset.
    pub wide: bool,
}

pub const REGIONS: [Region; 26] = [
    Region { name: "US", lo_mhz: 902.0, hi_mhz: 928.0, wide: false },
    Region { name: "EU_433", lo_mhz: 433.0, hi_mhz: 434.0, wide: false },
    Region { name: "EU_868", lo_mhz: 869.4, hi_mhz: 869.65, wide: false },
    Region { name: "CN", lo_mhz: 470.0, hi_mhz: 510.0, wide: false },
    Region { name: "JP", lo_mhz: 920.5, hi_mhz: 923.5, wide: false },
    Region { name: "ANZ", lo_mhz: 915.0, hi_mhz: 928.0, wide: false },
    Region { name: "ANZ_433", lo_mhz: 433.05, hi_mhz: 434.79, wide: false },
    Region { name: "RU", lo_mhz: 868.7, hi_mhz: 869.2, wide: false },
    Region { name: "KR", lo_mhz: 920.0, hi_mhz: 923.0, wide: false },
    Region { name: "TW", lo_mhz: 920.0, hi_mhz: 925.0, wide: false },
    Region { name: "IN", lo_mhz: 865.0, hi_mhz: 867.0, wide: false },
    Region { name: "NZ_865", lo_mhz: 864.0, hi_mhz: 868.0, wide: false },
    Region { name: "TH", lo_mhz: 920.0, hi_mhz: 925.0, wide: false },
    Region { name: "UA_433", lo_mhz: 433.0, hi_mhz: 434.7, wide: false },
    Region { name: "UA_868", lo_mhz: 868.0, hi_mhz: 868.6, wide: false },
    Region { name: "MY_433", lo_mhz: 433.0, hi_mhz: 435.0, wide: false },
    Region { name: "MY_919", lo_mhz: 919.0, hi_mhz: 924.0, wide: false },
    Region { name: "SG_923", lo_mhz: 917.0, hi_mhz: 925.0, wide: false },
    Region { name: "PH_433", lo_mhz: 433.0, hi_mhz: 434.7, wide: false },
    Region { name: "PH_868", lo_mhz: 868.0, hi_mhz: 869.4, wide: false },
    Region { name: "PH_915", lo_mhz: 915.0, hi_mhz: 918.0, wide: false },
    Region { name: "KZ_433", lo_mhz: 433.075, hi_mhz: 434.775, wide: false },
    Region { name: "KZ_863", lo_mhz: 863.0, hi_mhz: 868.0, wide: false },
    Region { name: "NP_865", lo_mhz: 865.0, hi_mhz: 868.0, wide: false },
    Region { name: "BR_902", lo_mhz: 902.0, hi_mhz: 907.5, wide: false },
    Region { name: "LORA_24", lo_mhz: 2400.0, hi_mhz: 2483.5, wide: true },
];

/// Look a region up by name, with the usual aliases (`EU868`, `EU433`,
/// `LORA24`, `LORA_2G4`).
pub fn lookup_region(name: &str) -> Option<&'static Region> {
    let aliased = match name.to_ascii_uppercase().as_str() {
        "EU868" => "EU_868".to_string(),
        "EU433" => "EU_433".to_string(),
        "LORA24" | "LORA_2G4" => "LORA_24".to_string(),
        other => other.to_string(),
    };
    REGIONS.iter().find(|r| r.name.eq_ignore_ascii_case(&aliased))
}

impl Region {
    /// Number of channel slots a preset of `bw_hz` cuts out of the band.
    pub fn channel_count(&self, bw_hz: u32) -> usize {
        if bw_hz == 0 {
            return 0;
        }
        let span = (self.hi_mhz - self.lo_mhz) * 1.0e6;
        (span / bw_hz as f64).max(0.0) as usize
    }

    /// Centre frequency in hertz of channel slot `index`: `lo + bw/2 + index * bw`.
    pub fn channel_freq_hz(&self, bw_hz: u32, index: usize) -> Option<u64> {
        if index >= self.channel_count(bw_hz) {
            return None;
        }
        let bw_mhz = bw_hz as f64 / 1.0e6;
        let f_mhz = self.lo_mhz + bw_mhz * 0.5 + bw_mhz * index as f64;
        Some((f_mhz * 1.0e6 + 0.5) as u64)
    }
}

/// Application port numbers (the `PortNum` enum of the firmware).
pub mod port {
    pub const UNKNOWN: u32 = 0;
    pub const TEXT_MESSAGE: u32 = 1;
    pub const REMOTE_HARDWARE: u32 = 2;
    pub const POSITION: u32 = 3;
    pub const NODEINFO: u32 = 4;
    pub const ROUTING: u32 = 5;
    pub const ADMIN: u32 = 6;
    pub const TEXT_MESSAGE_COMPRESSED: u32 = 7;
    pub const WAYPOINT: u32 = 8;
    pub const AUDIO: u32 = 9;
    pub const DETECTION_SENSOR: u32 = 10;
    pub const ALERT: u32 = 11;
    pub const KEY_VERIFICATION: u32 = 12;
    pub const REPLY: u32 = 32;
    pub const IP_TUNNEL: u32 = 33;
    pub const PAXCOUNTER: u32 = 34;
    pub const STORE_FORWARD_PLUSPLUS: u32 = 35;
    pub const NODE_STATUS: u32 = 36;
    pub const SERIAL: u32 = 64;
    pub const STORE_FORWARD: u32 = 65;
    pub const RANGE_TEST: u32 = 66;
    pub const TELEMETRY: u32 = 67;
    pub const ZPS: u32 = 68;
    pub const SIMULATOR: u32 = 69;
    pub const TRACEROUTE: u32 = 70;
    pub const NEIGHBORINFO: u32 = 71;
    pub const ATAK_PLUGIN: u32 = 72;
    pub const MAP_REPORT: u32 = 73;
    pub const POWERSTRESS: u32 = 74;
    pub const RETICULUM_TUNNEL: u32 = 76;
    pub const PRIVATE: u32 = 256;
    pub const ATAK_FORWARDER: u32 = 257;
}

/// Name of a port number as the firmware writes it; `UNKNOWN` otherwise.
pub fn port_name(p: u32) -> &'static str {
    match p {
        port::UNKNOWN => "UNKNOWN_APP",
        port::TEXT_MESSAGE => "TEXT_MESSAGE_APP",
        port::REMOTE_HARDWARE => "REMOTE_HARDWARE_APP",
        port::POSITION => "POSITION_APP",
        port::NODEINFO => "NODEINFO_APP",
        port::ROUTING => "ROUTING_APP",
        port::ADMIN => "ADMIN_APP",
        port::TEXT_MESSAGE_COMPRESSED => "TEXT_MESSAGE_COMPRESSED_APP",
        port::WAYPOINT => "WAYPOINT_APP",
        port::AUDIO => "AUDIO_APP",
        port::DETECTION_SENSOR => "DETECTION_SENSOR_APP",
        port::ALERT => "ALERT_APP",
        port::KEY_VERIFICATION => "KEY_VERIFICATION_APP",
        port::REPLY => "REPLY_APP",
        port::IP_TUNNEL => "IP_TUNNEL_APP",
        port::PAXCOUNTER => "PAXCOUNTER_APP",
        port::STORE_FORWARD_PLUSPLUS => "STORE_FORWARD_PLUSPLUS_APP",
        port::NODE_STATUS => "NODE_STATUS_APP",
        port::SERIAL => "SERIAL_APP",
        port::STORE_FORWARD => "STORE_FORWARD_APP",
        port::RANGE_TEST => "RANGE_TEST_APP",
        port::TELEMETRY => "TELEMETRY_APP",
        port::ZPS => "ZPS_APP",
        port::SIMULATOR => "SIMULATOR_APP",
        port::TRACEROUTE => "TRACEROUTE_APP",
        port::NEIGHBORINFO => "NEIGHBORINFO_APP",
        port::ATAK_PLUGIN => "ATAK_PLUGIN",
        port::MAP_REPORT => "MAP_REPORT_APP",
        port::POWERSTRESS => "POWERSTRESS_APP",
        port::RETICULUM_TUNNEL => "RETICULUM_TUNNEL_APP",
        port::PRIVATE => "PRIVATE_APP",
        port::ATAK_FORWARDER => "ATAK_FORWARDER",
        _ => "UNKNOWN",
    }
}

/// XOR of every byte.
fn xor_hash(bytes: &[u8]) -> u8 {
    bytes.iter().fold(0, |a, &b| a ^ b)
}

/// The channel hash that goes in the header: `xorHash(name) ^ xorHash(psk)`.
pub fn channel_hash(channel_name: &str, psk: &[u8]) -> u8 {
    xor_hash(channel_name.as_bytes()) ^ xor_hash(psk)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_channel_hashes_to_0x08() {
        // "LongFast" with the stock key is hash 8 on every default node.
        assert_eq!(channel_hash("LongFast", &DEFAULT_PSK), 0x08);
    }

    #[test]
    fn presets_are_found_by_any_spelling() {
        for s in ["LongFast", "LONG_FAST", "long-fast", "longfast"] {
            assert_eq!(lookup_preset(s).map(|p| p.name), Some("LONG_FAST"), "{s}");
        }
        assert_eq!(lookup_preset("LongMod").map(|p| p.name), Some("LONG_MODERATE"));
        assert!(lookup_preset("nonsense").is_none());
    }

    #[test]
    fn a_triple_names_its_preset() {
        assert_eq!(preset_for(11, 5, 250_000).map(|p| p.name), Some("LONG_FAST"));
        assert_eq!(preset_for(12, 8, 125_000).map(|p| p.name), Some("LONG_SLOW"));
        assert!(preset_for(9, 6, 250_000).is_none());
    }

    #[test]
    fn eu_868_has_one_long_fast_slot_at_869_525() {
        let r = lookup_region("EU868").unwrap();
        assert_eq!(r.channel_count(250_000), 1);
        assert_eq!(r.channel_freq_hz(250_000, 0), Some(869_525_000));
        assert_eq!(r.channel_freq_hz(250_000, 1), None);
    }

    #[test]
    fn us_long_fast_has_104_slots() {
        let r = lookup_region("US").unwrap();
        assert_eq!(r.channel_count(250_000), 104);
        assert_eq!(r.channel_freq_hz(250_000, 0), Some(902_125_000));
    }

    #[test]
    fn port_names_follow_the_firmware() {
        assert_eq!(port_name(port::TEXT_MESSAGE), "TEXT_MESSAGE_APP");
        assert_eq!(port_name(port::TELEMETRY), "TELEMETRY_APP");
        assert_eq!(port_name(9999), "UNKNOWN");
    }
}
