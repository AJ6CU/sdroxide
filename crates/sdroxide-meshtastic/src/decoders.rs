// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM
//
// Per-port protobuf decoders. Field numbers and types come from the
// Meshtastic .proto files (mesh.proto, telemetry.proto, atak.proto,
// storeforward.proto, ...). Every decoder is a tag switch over the payload:
// a field that arrives with a wire type other than the one the proto
// declares is skipped rather than misread, and unknown fields are skipped.

//! What a decrypted `Data` payload says, by port number.
//!
//! [`decode_payload`] picks the decoder for a port and returns a typed
//! [`Decoded`] value; [`Data::decode`] does the same for a parsed `Data`.
//! A payload that is truncated, or that carries nothing the decoder knows,
//! gives `None`: the caller still has the raw bytes.

use crate::packet::Data;
use crate::protobuf::{Reader, WIRE_FIXED32, WIRE_LENGTH, WIRE_VARINT, zigzag};
use crate::protocol::port;

fn float(v: u32) -> f32 {
    f32::from_bits(v)
}

/// An `sfixed32` latitude or longitude, in degrees (the wire unit is 1e-7).
fn degrees(v: u32) -> f64 {
    f64::from(v as i32) * 1e-7
}

fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

/// Read a repeated scalar in either encoding: one varint per tag, or a packed
/// length-delimited block. Senders may use either for the same field.
fn repeated(r: &mut Reader, wire: u32, out: &mut Vec<u64>) -> Option<()> {
    match wire {
        WIRE_VARINT => out.push(r.varint()?),
        WIRE_LENGTH => {
            let mut packed = Reader::new(r.bytes()?);
            while let Some(v) = packed.varint() {
                out.push(v);
            }
        }
        w => r.skip(w)?,
    }
    Some(())
}

// ---- POSITION_APP: meshtastic.Position ----

/// A position report. Latitude and longitude are degrees.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Position {
    pub lat_deg: Option<f64>,
    pub lon_deg: Option<f64>,
    /// Altitude above mean sea level, metres.
    pub altitude_m: Option<i32>,
    /// Sender wall clock, Unix seconds.
    pub time: Option<u32>,
    pub location_source: u32,
    pub altitude_source: u32,
    /// GPS fix time, Unix seconds.
    pub timestamp: Option<u32>,
    pub timestamp_millis_adjust: i32,
    pub altitude_hae_m: Option<i32>,
    pub altitude_geoidal_separation_m: Option<i32>,
    pub pdop_x100: u32,
    pub hdop_x100: u32,
    pub vdop_x100: u32,
    pub gps_accuracy_mm: u32,
    pub ground_speed_mps: Option<u32>,
    /// Course over ground in hundredths of a degree.
    pub ground_track_x100: Option<u32>,
    pub fix_quality: u32,
    pub fix_type: u32,
    pub sats_in_view: u32,
    pub sensor_id: u32,
    pub next_update_s: u32,
    pub seq_number: u32,
    /// How many bits of the coordinates the sender kept (privacy blur).
    pub precision_bits: u32,
}

impl Position {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let mut r = Reader::new(buf);
        let mut p = Position::default();
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_FIXED32) => p.lat_deg = Some(degrees(r.fixed32()?)),
                (2, WIRE_FIXED32) => p.lon_deg = Some(degrees(r.fixed32()?)),
                (3, WIRE_VARINT) => p.altitude_m = Some(r.varint()? as i32),
                (4, WIRE_FIXED32) => p.time = Some(r.fixed32()?),
                (5, WIRE_VARINT) => p.location_source = r.varint()? as u32,
                (6, WIRE_VARINT) => p.altitude_source = r.varint()? as u32,
                (7, WIRE_FIXED32) => p.timestamp = Some(r.fixed32()?),
                (8, WIRE_VARINT) => p.timestamp_millis_adjust = r.varint()? as i32,
                (9, WIRE_VARINT) => p.altitude_hae_m = Some(zigzag(r.varint()?) as i32),
                (10, WIRE_VARINT) => {
                    p.altitude_geoidal_separation_m = Some(zigzag(r.varint()?) as i32)
                }
                (11, WIRE_VARINT) => p.pdop_x100 = r.varint()? as u32,
                (12, WIRE_VARINT) => p.hdop_x100 = r.varint()? as u32,
                (13, WIRE_VARINT) => p.vdop_x100 = r.varint()? as u32,
                (14, WIRE_VARINT) => p.gps_accuracy_mm = r.varint()? as u32,
                (15, WIRE_VARINT) => p.ground_speed_mps = Some(r.varint()? as u32),
                (16, WIRE_VARINT) => p.ground_track_x100 = Some(r.varint()? as u32),
                (17, WIRE_VARINT) => p.fix_quality = r.varint()? as u32,
                (18, WIRE_VARINT) => p.fix_type = r.varint()? as u32,
                (19, WIRE_VARINT) => p.sats_in_view = r.varint()? as u32,
                (20, WIRE_VARINT) => p.sensor_id = r.varint()? as u32,
                (21, WIRE_VARINT) => p.next_update_s = r.varint()? as u32,
                (22, WIRE_VARINT) => p.seq_number = r.varint()? as u32,
                (23, WIRE_VARINT) => p.precision_bits = r.varint()? as u32,
                (_, w) => r.skip(w)?,
            }
        }
        let any =
            p.lat_deg.is_some() || p.lon_deg.is_some() || p.time.is_some() || p.timestamp.is_some();
        any.then_some(p)
    }
}

// ---- NODEINFO_APP: meshtastic.User ----

/// A node's identity card.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct User {
    pub id: String,
    pub long_name: String,
    pub short_name: String,
    pub macaddr: Option<[u8; 6]>,
    pub hw_model: u32,
    pub is_licensed: bool,
    pub role: u32,
    pub public_key: Vec<u8>,
    pub is_unmessagable: Option<bool>,
}

impl User {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let mut r = Reader::new(buf);
        let mut u = User::default();
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_LENGTH) => u.id = text(r.bytes()?),
                (2, WIRE_LENGTH) => u.long_name = text(r.bytes()?),
                (3, WIRE_LENGTH) => u.short_name = text(r.bytes()?),
                (4, WIRE_LENGTH) => u.macaddr = r.bytes()?.try_into().ok(),
                (5, WIRE_VARINT) => u.hw_model = r.varint()? as u32,
                (6, WIRE_VARINT) => u.is_licensed = r.varint()? != 0,
                (7, WIRE_VARINT) => u.role = r.varint()? as u32,
                (8, WIRE_LENGTH) => {
                    let key = r.bytes()?;
                    if key.len() <= 32 {
                        u.public_key = key.to_vec();
                    }
                }
                (9, WIRE_VARINT) => u.is_unmessagable = Some(r.varint()? != 0),
                (_, w) => r.skip(w)?,
            }
        }
        (!u.id.is_empty() || !u.long_name.is_empty() || !u.short_name.is_empty()).then_some(u)
    }
}

// ---- TELEMETRY_APP: meshtastic.Telemetry (a oneof) ----

#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeviceMetrics {
    pub battery_level: u32,
    pub voltage: f32,
    pub channel_utilization: f32,
    pub air_util_tx: f32,
    pub uptime_seconds: u32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct EnvironmentMetrics {
    pub temperature_c: f32,
    pub relative_humidity: f32,
    pub barometric_pressure_hpa: f32,
    pub gas_resistance: f32,
    pub voltage: f32,
    pub current: f32,
    /// Indoor air quality index (an integer on the wire, not a float).
    pub iaq: u32,
    pub distance_mm: f32,
    pub lux: f32,
    pub white_lux: f32,
    pub ir_lux: f32,
    pub uv_lux: f32,
    /// Degrees (an integer on the wire).
    pub wind_direction: u32,
    pub wind_speed: f32,
    pub weight: f32,
    pub wind_gust: f32,
    pub wind_lull: f32,
    pub radiation_usvh: f32,
    pub rainfall_1h_mm: f32,
    pub rainfall_24h_mm: f32,
    pub soil_moisture: u32,
    pub soil_temperature_c: f32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PowerChannel {
    pub voltage: f32,
    pub current: f32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PowerMetrics {
    pub channels: [PowerChannel; 8],
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AirQuality {
    pub pm10_standard: u32,
    pub pm25_standard: u32,
    pub pm100_standard: u32,
    pub pm10_env: u32,
    pub pm25_env: u32,
    pub pm100_env: u32,
    pub particles_03um: u32,
    pub particles_05um: u32,
    pub particles_10um: u32,
    pub particles_25um: u32,
    pub particles_50um: u32,
    pub particles_100um: u32,
    pub co2: u32,
    pub co2_temperature_c: f32,
    pub co2_humidity: f32,
    pub formaldehyde_ppb: f32,
    pub form_humidity: f32,
    pub form_temperature_c: f32,
    pub pm40_standard: u32,
    pub particles_40um: u32,
    pub pm_temperature_c: f32,
    pub pm_humidity: f32,
    pub pm_voc_idx: f32,
    pub pm_nox_idx: f32,
    pub particles_tps: f32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct LocalStats {
    pub uptime_s: u32,
    pub channel_utilization: f32,
    pub air_util_tx: f32,
    pub num_packets_tx: u32,
    pub num_packets_rx: u32,
    pub num_packets_rx_bad: u32,
    pub num_online_nodes: u32,
    pub num_total_nodes: u32,
    pub num_rx_dupe: u32,
    pub num_tx_relay: u32,
    pub num_tx_relay_canceled: u32,
    pub heap_total_bytes: u32,
    pub heap_free_bytes: u32,
    pub num_tx_dropped: u32,
    pub noise_floor_dbm: i32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Health {
    pub heart_bpm: u32,
    pub spo2: u32,
    pub temperature_c: f32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Host {
    pub uptime_s: u32,
    pub freemem_bytes: u64,
    pub diskfree_bytes: [u64; 3],
    pub load1_x100: u32,
    pub load5_x100: u32,
    pub load15_x100: u32,
    pub user_string: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct TrafficManagement {
    pub packets_inspected: u32,
    pub position_dedup_drops: u32,
    pub nodeinfo_cache_hits: u32,
    pub rate_limit_drops: u32,
    pub unknown_packet_drops: u32,
    pub hop_exhausted_packets: u32,
    pub router_hops_preserved: u32,
}

/// A telemetry report: the sender's clock and whichever metric groups it sent.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Telemetry {
    pub time_unix: u32,
    pub device: Option<DeviceMetrics>,
    pub environment: Option<EnvironmentMetrics>,
    pub air_quality: Option<AirQuality>,
    pub power: Option<PowerMetrics>,
    pub local_stats: Option<LocalStats>,
    pub health: Option<Health>,
    pub host: Option<Host>,
    pub traffic_management: Option<TrafficManagement>,
}

// A truncated group keeps what was read before the cut, as the group itself
// is still evidence that the sensor reported.
fn device_metrics(buf: &[u8]) -> DeviceMetrics {
    let mut m = DeviceMetrics::default();
    let mut r = Reader::new(buf);
    let _ = (|| -> Option<()> {
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_VARINT) => m.battery_level = r.varint()? as u32,
                (2, WIRE_FIXED32) => m.voltage = float(r.fixed32()?),
                (3, WIRE_FIXED32) => m.channel_utilization = float(r.fixed32()?),
                (4, WIRE_FIXED32) => m.air_util_tx = float(r.fixed32()?),
                (5, WIRE_VARINT) => m.uptime_seconds = r.varint()? as u32,
                (_, w) => r.skip(w)?,
            }
        }
        Some(())
    })();
    m
}

fn environment_metrics(buf: &[u8]) -> EnvironmentMetrics {
    let mut m = EnvironmentMetrics::default();
    let mut r = Reader::new(buf);
    let _ = (|| -> Option<()> {
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_FIXED32) => m.temperature_c = float(r.fixed32()?),
                (2, WIRE_FIXED32) => m.relative_humidity = float(r.fixed32()?),
                (3, WIRE_FIXED32) => m.barometric_pressure_hpa = float(r.fixed32()?),
                (4, WIRE_FIXED32) => m.gas_resistance = float(r.fixed32()?),
                (5, WIRE_FIXED32) => m.voltage = float(r.fixed32()?),
                (6, WIRE_FIXED32) => m.current = float(r.fixed32()?),
                (7, WIRE_VARINT) => m.iaq = r.varint()? as u32,
                (8, WIRE_FIXED32) => m.distance_mm = float(r.fixed32()?),
                (9, WIRE_FIXED32) => m.lux = float(r.fixed32()?),
                (10, WIRE_FIXED32) => m.white_lux = float(r.fixed32()?),
                (11, WIRE_FIXED32) => m.ir_lux = float(r.fixed32()?),
                (12, WIRE_FIXED32) => m.uv_lux = float(r.fixed32()?),
                (13, WIRE_VARINT) => m.wind_direction = r.varint()? as u32,
                (14, WIRE_FIXED32) => m.wind_speed = float(r.fixed32()?),
                (15, WIRE_FIXED32) => m.weight = float(r.fixed32()?),
                (16, WIRE_FIXED32) => m.wind_gust = float(r.fixed32()?),
                (17, WIRE_FIXED32) => m.wind_lull = float(r.fixed32()?),
                (18, WIRE_FIXED32) => m.radiation_usvh = float(r.fixed32()?),
                (19, WIRE_FIXED32) => m.rainfall_1h_mm = float(r.fixed32()?),
                (20, WIRE_FIXED32) => m.rainfall_24h_mm = float(r.fixed32()?),
                (21, WIRE_VARINT) => m.soil_moisture = r.varint()? as u32,
                (22, WIRE_FIXED32) => m.soil_temperature_c = float(r.fixed32()?),
                (_, w) => r.skip(w)?,
            }
        }
        Some(())
    })();
    m
}

fn power_metrics(buf: &[u8]) -> PowerMetrics {
    let mut m = PowerMetrics::default();
    let mut r = Reader::new(buf);
    let _ = (|| -> Option<()> {
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                // 1..=16: voltage and current of channels 1..=8, interleaved.
                (n @ 1..=16, WIRE_FIXED32) => {
                    let ch = &mut m.channels[(n as usize - 1) / 2];
                    let v = float(r.fixed32()?);
                    if n % 2 == 1 {
                        ch.voltage = v;
                    } else {
                        ch.current = v;
                    }
                }
                (_, w) => r.skip(w)?,
            }
        }
        Some(())
    })();
    m
}

fn air_quality(buf: &[u8]) -> AirQuality {
    let mut m = AirQuality::default();
    let mut r = Reader::new(buf);
    let _ = (|| -> Option<()> {
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_VARINT) => m.pm10_standard = r.varint()? as u32,
                (2, WIRE_VARINT) => m.pm25_standard = r.varint()? as u32,
                (3, WIRE_VARINT) => m.pm100_standard = r.varint()? as u32,
                (4, WIRE_VARINT) => m.pm10_env = r.varint()? as u32,
                (5, WIRE_VARINT) => m.pm25_env = r.varint()? as u32,
                (6, WIRE_VARINT) => m.pm100_env = r.varint()? as u32,
                (7, WIRE_VARINT) => m.particles_03um = r.varint()? as u32,
                (8, WIRE_VARINT) => m.particles_05um = r.varint()? as u32,
                (9, WIRE_VARINT) => m.particles_10um = r.varint()? as u32,
                (10, WIRE_VARINT) => m.particles_25um = r.varint()? as u32,
                (11, WIRE_VARINT) => m.particles_50um = r.varint()? as u32,
                (12, WIRE_VARINT) => m.particles_100um = r.varint()? as u32,
                (13, WIRE_VARINT) => m.co2 = r.varint()? as u32,
                (14, WIRE_FIXED32) => m.co2_temperature_c = float(r.fixed32()?),
                (15, WIRE_FIXED32) => m.co2_humidity = float(r.fixed32()?),
                (16, WIRE_FIXED32) => m.formaldehyde_ppb = float(r.fixed32()?),
                (17, WIRE_FIXED32) => m.form_humidity = float(r.fixed32()?),
                (18, WIRE_FIXED32) => m.form_temperature_c = float(r.fixed32()?),
                (19, WIRE_VARINT) => m.pm40_standard = r.varint()? as u32,
                (20, WIRE_VARINT) => m.particles_40um = r.varint()? as u32,
                (21, WIRE_FIXED32) => m.pm_temperature_c = float(r.fixed32()?),
                (22, WIRE_FIXED32) => m.pm_humidity = float(r.fixed32()?),
                (23, WIRE_FIXED32) => m.pm_voc_idx = float(r.fixed32()?),
                (24, WIRE_FIXED32) => m.pm_nox_idx = float(r.fixed32()?),
                (25, WIRE_FIXED32) => m.particles_tps = float(r.fixed32()?),
                (_, w) => r.skip(w)?,
            }
        }
        Some(())
    })();
    m
}

fn local_stats(buf: &[u8]) -> LocalStats {
    let mut m = LocalStats::default();
    let mut r = Reader::new(buf);
    let _ = (|| -> Option<()> {
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_VARINT) => m.uptime_s = r.varint()? as u32,
                (2, WIRE_FIXED32) => m.channel_utilization = float(r.fixed32()?),
                (3, WIRE_FIXED32) => m.air_util_tx = float(r.fixed32()?),
                (4, WIRE_VARINT) => m.num_packets_tx = r.varint()? as u32,
                (5, WIRE_VARINT) => m.num_packets_rx = r.varint()? as u32,
                (6, WIRE_VARINT) => m.num_packets_rx_bad = r.varint()? as u32,
                (7, WIRE_VARINT) => m.num_online_nodes = r.varint()? as u32,
                (8, WIRE_VARINT) => m.num_total_nodes = r.varint()? as u32,
                (9, WIRE_VARINT) => m.num_rx_dupe = r.varint()? as u32,
                (10, WIRE_VARINT) => m.num_tx_relay = r.varint()? as u32,
                (11, WIRE_VARINT) => m.num_tx_relay_canceled = r.varint()? as u32,
                (12, WIRE_VARINT) => m.heap_total_bytes = r.varint()? as u32,
                (13, WIRE_VARINT) => m.heap_free_bytes = r.varint()? as u32,
                (14, WIRE_VARINT) => m.num_tx_dropped = r.varint()? as u32,
                (15, WIRE_VARINT) => m.noise_floor_dbm = r.varint()? as i32,
                (_, w) => r.skip(w)?,
            }
        }
        Some(())
    })();
    m
}

fn health(buf: &[u8]) -> Health {
    let mut m = Health::default();
    let mut r = Reader::new(buf);
    let _ = (|| -> Option<()> {
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_VARINT) => m.heart_bpm = r.varint()? as u32,
                (2, WIRE_VARINT) => m.spo2 = r.varint()? as u32,
                (3, WIRE_FIXED32) => m.temperature_c = float(r.fixed32()?),
                (_, w) => r.skip(w)?,
            }
        }
        Some(())
    })();
    m
}

fn host(buf: &[u8]) -> Host {
    let mut m = Host::default();
    let mut r = Reader::new(buf);
    let _ = (|| -> Option<()> {
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_VARINT) => m.uptime_s = r.varint()? as u32,
                (2, WIRE_VARINT) => m.freemem_bytes = r.varint()?,
                (n @ 3..=5, WIRE_VARINT) => m.diskfree_bytes[n as usize - 3] = r.varint()?,
                (6, WIRE_VARINT) => m.load1_x100 = r.varint()? as u32,
                (7, WIRE_VARINT) => m.load5_x100 = r.varint()? as u32,
                (8, WIRE_VARINT) => m.load15_x100 = r.varint()? as u32,
                (9, WIRE_LENGTH) => m.user_string = text(r.bytes()?),
                (_, w) => r.skip(w)?,
            }
        }
        Some(())
    })();
    m
}

fn traffic_management(buf: &[u8]) -> TrafficManagement {
    let mut m = TrafficManagement::default();
    let mut r = Reader::new(buf);
    let _ = (|| -> Option<()> {
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_VARINT) => m.packets_inspected = r.varint()? as u32,
                (2, WIRE_VARINT) => m.position_dedup_drops = r.varint()? as u32,
                (3, WIRE_VARINT) => m.nodeinfo_cache_hits = r.varint()? as u32,
                (4, WIRE_VARINT) => m.rate_limit_drops = r.varint()? as u32,
                (5, WIRE_VARINT) => m.unknown_packet_drops = r.varint()? as u32,
                (6, WIRE_VARINT) => m.hop_exhausted_packets = r.varint()? as u32,
                (7, WIRE_VARINT) => m.router_hops_preserved = r.varint()? as u32,
                (_, w) => r.skip(w)?,
            }
        }
        Some(())
    })();
    m
}

impl Telemetry {
    /// `None` when nothing at all could be read. A message cut short keeps the
    /// groups that arrived whole.
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let mut r = Reader::new(buf);
        let mut t = Telemetry::default();
        let mut any = false;
        let _ = (|| -> Option<()> {
            while let Some((field, wire)) = r.tag() {
                match (field, wire) {
                    (1, WIRE_FIXED32) => t.time_unix = r.fixed32()?,
                    (2, WIRE_LENGTH) => t.device = Some(device_metrics(r.bytes()?)),
                    (3, WIRE_LENGTH) => t.environment = Some(environment_metrics(r.bytes()?)),
                    (4, WIRE_LENGTH) => t.air_quality = Some(air_quality(r.bytes()?)),
                    (5, WIRE_LENGTH) => t.power = Some(power_metrics(r.bytes()?)),
                    (6, WIRE_LENGTH) => t.local_stats = Some(local_stats(r.bytes()?)),
                    (7, WIRE_LENGTH) => t.health = Some(health(r.bytes()?)),
                    (8, WIRE_LENGTH) => t.host = Some(host(r.bytes()?)),
                    (9, WIRE_LENGTH) => t.traffic_management = Some(traffic_management(r.bytes()?)),
                    (_, w) => r.skip(w)?,
                }
                any = true;
            }
            Some(())
        })();
        any.then_some(t)
    }
}

// ---- ROUTING_APP / TRACEROUTE_APP: meshtastic.Routing, RouteDiscovery ----

/// A path of node numbers, with the signal-to-noise of each hop.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RouteDiscovery {
    pub route: Vec<u32>,
    /// SNR of each hop towards the destination, in quarter decibels.
    pub snr_towards: Vec<i8>,
    pub route_back: Vec<u32>,
    pub snr_back: Vec<i8>,
}

impl RouteDiscovery {
    fn fill(buf: &[u8]) -> Option<Self> {
        let mut r = Reader::new(buf);
        let (mut route, mut towards, mut back, mut snr_back) =
            (Vec::new(), Vec::new(), Vec::new(), Vec::new());
        while let Some((field, wire)) = r.tag() {
            match field {
                1 => repeated(&mut r, wire, &mut route)?,
                2 => repeated(&mut r, wire, &mut towards)?,
                3 => repeated(&mut r, wire, &mut back)?,
                4 => repeated(&mut r, wire, &mut snr_back)?,
                _ => r.skip(wire)?,
            }
        }
        let ids = |v: Vec<u64>| -> Vec<u32> { v.into_iter().map(|x| x as u32).collect() };
        let snr = |v: Vec<u64>| -> Vec<i8> { v.into_iter().map(|x| x as i32 as i8).collect() };
        Some(Self {
            route: ids(route),
            snr_towards: snr(towards),
            route_back: ids(back),
            snr_back: snr(snr_back),
        })
    }

    /// A traceroute payload. `None` unless it names at least one hop.
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let d = Self::fill(buf)?;
        (!d.route.is_empty() || !d.route_back.is_empty()).then_some(d)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingKind {
    Request,
    Reply,
    /// An acknowledgement (reason 0) or a delivery failure.
    Error,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Routing {
    pub kind: RoutingKind,
    pub discovery: RouteDiscovery,
    pub error_reason: Option<u32>,
}

impl Routing {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let mut r = Reader::new(buf);
        let mut out: Option<Routing> = None;
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1 | 2, WIRE_LENGTH) => {
                    let kind = if field == 1 { RoutingKind::Request } else { RoutingKind::Reply };
                    let discovery = RouteDiscovery::fill(r.bytes()?).unwrap_or_default();
                    out = Some(Routing { kind, discovery, error_reason: None });
                }
                (3, WIRE_VARINT) => {
                    out = Some(Routing {
                        kind: RoutingKind::Error,
                        discovery: RouteDiscovery::default(),
                        error_reason: Some(r.varint()? as u32),
                    });
                }
                (_, w) => r.skip(w)?,
            }
        }
        out
    }
}

/// The name of a `Routing.Error` value, for the ones the firmware defines.
pub fn routing_error_name(reason: u32) -> Option<&'static str> {
    Some(match reason {
        0 => "NONE",
        1 => "NO_ROUTE",
        2 => "GOT_NAK",
        3 => "TIMEOUT",
        4 => "NO_INTERFACE",
        5 => "MAX_RETRANSMIT",
        6 => "NO_CHANNEL",
        7 => "TOO_LARGE",
        8 => "NO_RESPONSE",
        9 => "DUTY_CYCLE_LIMIT",
        32 => "BAD_REQUEST",
        33 => "NOT_AUTHORIZED",
        34 => "PKI_FAILED",
        35 => "PKI_UNKNOWN_PUBKEY",
        36 => "ADMIN_BAD_SESSION_KEY",
        37 => "ADMIN_PUBLIC_KEY_UNAUTHORIZED",
        38 => "RATE_LIMIT_EXCEEDED",
        _ => return None,
    })
}

// ---- WAYPOINT_APP: meshtastic.Waypoint ----

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Waypoint {
    pub id: u32,
    pub lat_deg: Option<f64>,
    pub lon_deg: Option<f64>,
    /// Unix seconds, 0 for never.
    pub expire: u32,
    /// Node allowed to edit it, 0 for anyone.
    pub locked_to: u32,
    pub name: String,
    pub description: String,
    /// A Unicode code point.
    pub icon: u32,
}

impl Waypoint {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let mut r = Reader::new(buf);
        let mut w = Waypoint::default();
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_VARINT) => w.id = r.varint()? as u32,
                (2, WIRE_FIXED32) => w.lat_deg = Some(degrees(r.fixed32()?)),
                (3, WIRE_FIXED32) => w.lon_deg = Some(degrees(r.fixed32()?)),
                (4, WIRE_VARINT) => w.expire = r.varint()? as u32,
                (5, WIRE_VARINT) => w.locked_to = r.varint()? as u32,
                (6, WIRE_LENGTH) => w.name = text(r.bytes()?),
                (7, WIRE_LENGTH) => w.description = text(r.bytes()?),
                (8, WIRE_FIXED32) => w.icon = r.fixed32()?,
                (_, x) => r.skip(x)?,
            }
        }
        (w.lat_deg.is_some() || w.lon_deg.is_some() || !w.name.is_empty()).then_some(w)
    }
}

// ---- NEIGHBORINFO_APP: meshtastic.NeighborInfo ----

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Neighbor {
    pub node_id: u32,
    pub snr_db: f32,
    pub last_rx_time: u32,
    pub node_broadcast_interval_secs: u32,
}

/// The most neighbours kept from one report.
pub const MAX_NEIGHBORS: usize = 16;

#[derive(Debug, Clone, Default, PartialEq)]
pub struct NeighborInfo {
    pub node_id: u32,
    pub last_sent_by_id: u32,
    pub node_broadcast_interval_secs: u32,
    pub neighbors: Vec<Neighbor>,
}

fn neighbor(buf: &[u8]) -> Neighbor {
    let mut n = Neighbor::default();
    let mut r = Reader::new(buf);
    let _ = (|| -> Option<()> {
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_VARINT) => n.node_id = r.varint()? as u32,
                (2, WIRE_FIXED32) => n.snr_db = float(r.fixed32()?),
                (3, WIRE_FIXED32) => n.last_rx_time = r.fixed32()?,
                (4, WIRE_VARINT) => n.node_broadcast_interval_secs = r.varint()? as u32,
                (_, w) => r.skip(w)?,
            }
        }
        Some(())
    })();
    n
}

impl NeighborInfo {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let mut r = Reader::new(buf);
        let mut n = NeighborInfo::default();
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_VARINT) => n.node_id = r.varint()? as u32,
                (2, WIRE_VARINT) => n.last_sent_by_id = r.varint()? as u32,
                (3, WIRE_VARINT) => n.node_broadcast_interval_secs = r.varint()? as u32,
                (4, WIRE_LENGTH) => {
                    let b = r.bytes()?;
                    if n.neighbors.len() < MAX_NEIGHBORS {
                        n.neighbors.push(neighbor(b));
                    }
                }
                (_, w) => r.skip(w)?,
            }
        }
        (n.node_id != 0 || !n.neighbors.is_empty()).then_some(n)
    }
}

// ---- KEY_VERIFICATION_APP: meshtastic.KeyVerification (metadata only) ----

/// Only the nonce and the hash sizes. The hash bytes are not kept: they could
/// fingerprint a verification exchange.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct KeyVerification {
    pub nonce: u64,
    pub hash1_len: usize,
    pub hash2_len: usize,
}

impl KeyVerification {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let mut r = Reader::new(buf);
        let mut k = KeyVerification::default();
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_VARINT) => k.nonce = r.varint()?,
                (2, WIRE_LENGTH) => k.hash1_len = r.bytes()?.len(),
                (3, WIRE_LENGTH) => k.hash2_len = r.bytes()?.len(),
                (_, w) => r.skip(w)?,
            }
        }
        (k.nonce != 0 || k.hash1_len != 0 || k.hash2_len != 0).then_some(k)
    }
}

// ---- MAP_REPORT_APP: meshtastic.MapReport ----

#[derive(Debug, Clone, Default, PartialEq)]
pub struct MapReport {
    pub long_name: String,
    pub short_name: String,
    pub role: u32,
    pub hw_model: u32,
    pub firmware_version: String,
    pub region: u32,
    pub modem_preset: u32,
    pub has_default_channel: u32,
    pub lat_deg: Option<f64>,
    pub lon_deg: Option<f64>,
    pub altitude_m: i32,
    pub position_precision: u32,
    pub num_online_local_nodes: u32,
}

impl MapReport {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let mut r = Reader::new(buf);
        let mut m = MapReport::default();
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_LENGTH) => m.long_name = text(r.bytes()?),
                (2, WIRE_LENGTH) => m.short_name = text(r.bytes()?),
                (3, WIRE_VARINT) => m.role = r.varint()? as u32,
                (4, WIRE_VARINT) => m.hw_model = r.varint()? as u32,
                (5, WIRE_LENGTH) => m.firmware_version = text(r.bytes()?),
                (6, WIRE_VARINT) => m.region = r.varint()? as u32,
                (7, WIRE_VARINT) => m.modem_preset = r.varint()? as u32,
                (8, WIRE_VARINT) => m.has_default_channel = r.varint()? as u32,
                (9, WIRE_FIXED32) => m.lat_deg = Some(degrees(r.fixed32()?)),
                (10, WIRE_FIXED32) => m.lon_deg = Some(degrees(r.fixed32()?)),
                (11, WIRE_VARINT) => m.altitude_m = r.varint()? as i32,
                (12, WIRE_VARINT) => m.position_precision = r.varint()? as u32,
                (13, WIRE_VARINT) => m.num_online_local_nodes = r.varint()? as u32,
                (_, w) => r.skip(w)?,
            }
        }
        (!m.long_name.is_empty() || m.lat_deg.is_some()).then_some(m)
    }
}

// ---- ADMIN_APP: meshtastic.AdminMessage (which request, not its content) ----

/// `AdminMessage` is a oneof of some forty requests. Only which one was
/// present is kept: enough to see that an administrative call happened.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Admin {
    /// Field number of the first request field; 0 when only a passkey came.
    pub variant_field: u32,
    pub has_session_passkey: bool,
}

impl Admin {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let mut r = Reader::new(buf);
        let mut a = Admin::default();
        while let Some((field, wire)) = r.tag() {
            if field == 101 && wire == WIRE_LENGTH {
                a.has_session_passkey = !r.bytes()?.is_empty();
            } else {
                if a.variant_field == 0 && field != 0 {
                    a.variant_field = field;
                }
                r.skip(wire)?;
            }
        }
        (a.variant_field != 0 || a.has_session_passkey).then_some(a)
    }
}

// ---- ATAK_PLUGIN: meshtastic.atak.TAKPacket ----

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtakKind {
    /// Position and location information.
    Pli,
    Chat,
    /// A raw detail block (XML).
    Detail,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AtakChat {
    pub message: String,
    pub to: String,
    pub to_callsign: String,
    pub receipt_for_uid: String,
    pub receipt_type: u32,
    pub lang: String,
    pub room_id: String,
    pub has_voice_profile: bool,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Atak {
    pub kind: Option<AtakKind>,
    pub is_compressed: bool,
    pub callsign: String,
    pub device_callsign: String,
    pub role: u32,
    pub team: u32,
    pub battery: u32,
    pub lat_deg: Option<f64>,
    pub lon_deg: Option<f64>,
    pub altitude_hae_m: i32,
    pub speed_mps: u32,
    pub course_deg: u32,
    pub chat: Option<AtakChat>,
    pub detail_xml: Vec<u8>,
}

/// The name of an ATAK team colour.
pub fn atak_team_name(team: u32) -> &'static str {
    const NAMES: [&str; 15] = [
        "Unspecified",
        "White",
        "Yellow",
        "Orange",
        "Magenta",
        "Red",
        "Maroon",
        "Purple",
        "DarkBlue",
        "Blue",
        "Cyan",
        "Teal",
        "Green",
        "DarkGreen",
        "Brown",
    ];
    NAMES.get(team as usize).copied().unwrap_or("Unspecified")
}

/// The name of an ATAK member role.
pub fn atak_role_name(role: u32) -> &'static str {
    const NAMES: [&str; 9] = [
        "Unspecified",
        "TeamMember",
        "TeamLead",
        "HQ",
        "Sniper",
        "Medic",
        "ForwardObserver",
        "RTO",
        "K9",
    ];
    NAMES.get(role as usize).copied().unwrap_or("Unknown")
}

impl Atak {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let mut r = Reader::new(buf);
        let mut a = Atak::default();
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_VARINT) => a.is_compressed = r.varint()? != 0,
                (2, WIRE_LENGTH) => {
                    let mut c = Reader::new(r.bytes()?);
                    while let Some((f, w)) = c.tag() {
                        let ok = match (f, w) {
                            (1, WIRE_LENGTH) => c.bytes().map(|b| a.callsign = text(b)),
                            (2, WIRE_LENGTH) => c.bytes().map(|b| a.device_callsign = text(b)),
                            _ => c.skip(w),
                        };
                        if ok.is_none() {
                            break;
                        }
                    }
                }
                (3, WIRE_LENGTH) => {
                    let mut g = Reader::new(r.bytes()?);
                    while let Some((f, w)) = g.tag() {
                        let ok = match (f, w) {
                            (1, WIRE_VARINT) => g.varint().map(|v| a.role = v as u32),
                            (2, WIRE_VARINT) => g.varint().map(|v| a.team = v as u32),
                            _ => g.skip(w),
                        };
                        if ok.is_none() {
                            break;
                        }
                    }
                }
                (4, WIRE_LENGTH) => {
                    let mut s = Reader::new(r.bytes()?);
                    while let Some((f, w)) = s.tag() {
                        let ok = match (f, w) {
                            (1, WIRE_VARINT) => s.varint().map(|v| a.battery = v as u32),
                            _ => s.skip(w),
                        };
                        if ok.is_none() {
                            break;
                        }
                    }
                }
                (5, WIRE_LENGTH) => {
                    a.kind = Some(AtakKind::Pli);
                    let mut p = Reader::new(r.bytes()?);
                    while let Some((f, w)) = p.tag() {
                        let ok = match (f, w) {
                            (1, WIRE_FIXED32) => p.fixed32().map(|v| a.lat_deg = Some(degrees(v))),
                            (2, WIRE_FIXED32) => p.fixed32().map(|v| a.lon_deg = Some(degrees(v))),
                            (3, WIRE_VARINT) => p.varint().map(|v| a.altitude_hae_m = v as i32),
                            (4, WIRE_VARINT) => p.varint().map(|v| a.speed_mps = v as u32),
                            (5, WIRE_VARINT) => p.varint().map(|v| a.course_deg = v as u32),
                            _ => p.skip(w),
                        };
                        if ok.is_none() {
                            break;
                        }
                    }
                }
                (6, WIRE_LENGTH) => {
                    a.kind = Some(AtakKind::Chat);
                    let mut chat = AtakChat::default();
                    let mut c = Reader::new(r.bytes()?);
                    while let Some((f, w)) = c.tag() {
                        let ok = match (f, w) {
                            (1, WIRE_LENGTH) => c.bytes().map(|b| chat.message = text(b)),
                            (2, WIRE_LENGTH) => c.bytes().map(|b| chat.to = text(b)),
                            (3, WIRE_LENGTH) => c.bytes().map(|b| chat.to_callsign = text(b)),
                            (4, WIRE_LENGTH) => c.bytes().map(|b| chat.receipt_for_uid = text(b)),
                            (5, WIRE_VARINT) => c.varint().map(|v| chat.receipt_type = v as u32),
                            (6, WIRE_LENGTH) => c.bytes().map(|b| chat.lang = text(b)),
                            (7, WIRE_LENGTH) => c.bytes().map(|b| chat.room_id = text(b)),
                            (8, WIRE_LENGTH) => c.bytes().map(|_| chat.has_voice_profile = true),
                            _ => c.skip(w),
                        };
                        if ok.is_none() {
                            break;
                        }
                    }
                    a.chat = Some(chat);
                }
                (7, WIRE_LENGTH) => {
                    a.kind = Some(AtakKind::Detail);
                    a.detail_xml = r.bytes()?.to_vec();
                }
                (_, w) => r.skip(w)?,
            }
        }
        (a.kind.is_some() || !a.callsign.is_empty()).then_some(a)
    }
}

// ---- REMOTE_HARDWARE_APP: meshtastic.HardwareMessage ----

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RemoteHardware {
    /// 0 unset, 1 write GPIOs, 2 watch GPIOs, 3 GPIOs changed, 4 read GPIOs,
    /// 5 reply to a read.
    pub kind: u32,
    pub gpio_mask: u64,
    pub gpio_value: u64,
}

impl RemoteHardware {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let mut r = Reader::new(buf);
        let mut h = RemoteHardware::default();
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_VARINT) => h.kind = r.varint()? as u32,
                (2, WIRE_VARINT) => h.gpio_mask = r.varint()?,
                (3, WIRE_VARINT) => h.gpio_value = r.varint()?,
                (_, w) => r.skip(w)?,
            }
        }
        Some(h)
    }
}

// ---- STORE_FORWARD_APP: meshtastic.StoreAndForward ----

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StoreForwardStats {
    pub messages_total: u32,
    pub messages_saved: u32,
    pub messages_max: u32,
    pub up_time_s: u32,
    pub requests: u32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StoreForwardHistory {
    pub messages: u32,
    pub window_s: u32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StoreForward {
    /// The `RequestResponse` code; see [`store_forward_name`].
    pub rr: u32,
    pub stats: Option<StoreForwardStats>,
    pub history: Option<StoreForwardHistory>,
}

/// The name of a `StoreAndForward.RequestResponse` value.
pub fn store_forward_name(rr: u32) -> Option<&'static str> {
    Some(match rr {
        0 => "UNSET",
        1 => "ROUTER_ERROR",
        2 => "ROUTER_HEARTBEAT",
        3 => "ROUTER_PING",
        4 => "ROUTER_PONG",
        5 => "ROUTER_BUSY",
        6 => "ROUTER_HISTORY",
        7 => "ROUTER_STATS",
        8 => "ROUTER_TEXT_DIRECT",
        9 => "ROUTER_TEXT_BROADCAST",
        64 => "CLIENT_ERROR",
        65 => "CLIENT_HISTORY",
        66 => "CLIENT_STATS",
        67 => "CLIENT_PING",
        68 => "CLIENT_PONG",
        106 => "CLIENT_ABORT",
        _ => return None,
    })
}

fn store_forward_stats(buf: &[u8]) -> Option<StoreForwardStats> {
    let mut r = Reader::new(buf);
    let mut s = StoreForwardStats::default();
    while let Some((field, wire)) = r.tag() {
        match (field, wire) {
            (1, WIRE_VARINT) => s.messages_total = r.varint()? as u32,
            (2, WIRE_VARINT) => s.messages_saved = r.varint()? as u32,
            (3, WIRE_VARINT) => s.messages_max = r.varint()? as u32,
            (4, WIRE_VARINT) => s.up_time_s = r.varint()? as u32,
            (5, WIRE_VARINT) => s.requests = r.varint()? as u32,
            (_, w) => r.skip(w)?,
        }
    }
    Some(s)
}

fn store_forward_history(buf: &[u8]) -> Option<StoreForwardHistory> {
    let mut r = Reader::new(buf);
    let mut h = StoreForwardHistory::default();
    while let Some((field, wire)) = r.tag() {
        match (field, wire) {
            (1, WIRE_VARINT) => h.messages = r.varint()? as u32,
            (2, WIRE_VARINT) => h.window_s = r.varint()? as u32,
            (_, w) => r.skip(w)?,
        }
    }
    Some(h)
}

impl StoreForward {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let mut r = Reader::new(buf);
        let mut s = StoreForward::default();
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_VARINT) => s.rr = r.varint()? as u32,
                (2, WIRE_LENGTH) => s.stats = store_forward_stats(r.bytes()?),
                (3, WIRE_LENGTH) => s.history = store_forward_history(r.bytes()?),
                (_, w) => r.skip(w)?,
            }
        }
        Some(s)
    }
}

// ---- PAXCOUNTER_APP: meshtastic.Paxcount ----

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Paxcounter {
    pub wifi: u32,
    pub ble: u32,
    pub uptime_s: u32,
}

impl Paxcounter {
    pub fn parse(buf: &[u8]) -> Option<Self> {
        let mut r = Reader::new(buf);
        let mut p = Paxcounter::default();
        while let Some((field, wire)) = r.tag() {
            match (field, wire) {
                (1, WIRE_VARINT) => p.wifi = r.varint()? as u32,
                (2, WIRE_VARINT) => p.ble = r.varint()? as u32,
                (3, WIRE_VARINT) => p.uptime_s = r.varint()? as u32,
                (_, w) => r.skip(w)?,
            }
        }
        Some(p)
    }
}

// ---- dispatch ----

/// A decoded payload, one variant per supported port.
#[derive(Debug, Clone, PartialEq)]
pub enum Decoded {
    Text(String),
    Position(Position),
    NodeInfo(User),
    Telemetry(Telemetry),
    Routing(Routing),
    Waypoint(Waypoint),
    NeighborInfo(NeighborInfo),
    KeyVerification(KeyVerification),
    MapReport(MapReport),
    Admin(Admin),
    Atak(Atak),
    Traceroute(RouteDiscovery),
    RemoteHardware(RemoteHardware),
    /// The payload of a detection sensor is plain text, not a protobuf.
    Detection(String),
    StoreForward(StoreForward),
    Paxcounter(Paxcounter),
}

/// Decode the payload of a `Data` message by its port number. `None` for a
/// port with no decoder here, or a payload the decoder cannot read.
pub fn decode_payload(portnum: u32, payload: &[u8]) -> Option<Decoded> {
    Some(match portnum {
        port::TEXT_MESSAGE => Decoded::Text(text(payload)),
        port::POSITION => Decoded::Position(Position::parse(payload)?),
        port::NODEINFO => Decoded::NodeInfo(User::parse(payload)?),
        port::TELEMETRY => Decoded::Telemetry(Telemetry::parse(payload)?),
        port::ROUTING => Decoded::Routing(Routing::parse(payload)?),
        port::WAYPOINT => Decoded::Waypoint(Waypoint::parse(payload)?),
        port::NEIGHBORINFO => Decoded::NeighborInfo(NeighborInfo::parse(payload)?),
        port::KEY_VERIFICATION => Decoded::KeyVerification(KeyVerification::parse(payload)?),
        port::MAP_REPORT => Decoded::MapReport(MapReport::parse(payload)?),
        port::ADMIN => Decoded::Admin(Admin::parse(payload)?),
        port::ATAK_PLUGIN => Decoded::Atak(Atak::parse(payload)?),
        port::TRACEROUTE => Decoded::Traceroute(RouteDiscovery::parse(payload)?),
        port::REMOTE_HARDWARE => Decoded::RemoteHardware(RemoteHardware::parse(payload)?),
        port::DETECTION_SENSOR if !payload.is_empty() => Decoded::Detection(text(payload)),
        port::STORE_FORWARD => Decoded::StoreForward(StoreForward::parse(payload)?),
        port::PAXCOUNTER => Decoded::Paxcounter(Paxcounter::parse(payload)?),
        _ => return None,
    })
}

impl Data {
    /// What this message's payload says; see [`decode_payload`].
    pub fn decode(&self) -> Option<Decoded> {
        decode_payload(self.portnum, &self.payload)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protobuf::{put_bytes, put_tag, put_varint};

    fn varint(out: &mut Vec<u8>, field: u32, v: u64) {
        put_tag(out, field, WIRE_VARINT);
        put_varint(out, v);
    }

    fn fixed32(out: &mut Vec<u8>, field: u32, v: u32) {
        put_tag(out, field, WIRE_FIXED32);
        out.extend_from_slice(&v.to_le_bytes());
    }

    fn float32(out: &mut Vec<u8>, field: u32, v: f32) {
        fixed32(out, field, v.to_bits());
    }

    fn lat(deg: f64) -> u32 {
        (deg * 1e7).round() as i32 as u32
    }

    #[test]
    fn a_position_carries_signed_coordinates() {
        let mut b = Vec::new();
        fixed32(&mut b, 1, lat(-33.8688));
        fixed32(&mut b, 2, lat(151.2093));
        varint(&mut b, 3, 58);
        fixed32(&mut b, 4, 1_700_000_000);
        varint(&mut b, 9, 81); // zigzag: 81 is -41
        varint(&mut b, 15, 12);
        varint(&mut b, 23, 13);
        let p = Position::parse(&b).unwrap();
        assert!((p.lat_deg.unwrap() + 33.8688).abs() < 1e-6);
        assert!((p.lon_deg.unwrap() - 151.2093).abs() < 1e-6);
        assert_eq!(p.altitude_m, Some(58));
        assert_eq!(p.time, Some(1_700_000_000));
        assert_eq!(p.altitude_hae_m, Some(-41));
        assert_eq!(p.ground_speed_mps, Some(12));
        assert_eq!(p.precision_bits, 13);
    }

    #[test]
    fn a_position_field_with_the_wrong_wire_type_is_skipped() {
        // Latitude sent as a varint: ignored, not read as a coordinate.
        let mut b = Vec::new();
        varint(&mut b, 1, 12345);
        fixed32(&mut b, 2, lat(10.0));
        let p = Position::parse(&b).unwrap();
        assert_eq!(p.lat_deg, None);
        assert!((p.lon_deg.unwrap() - 10.0).abs() < 1e-6);
    }

    #[test]
    fn a_position_with_no_fix_is_refused() {
        let mut b = Vec::new();
        varint(&mut b, 19, 7);
        assert_eq!(Position::parse(&b), None);
        assert_eq!(Position::parse(&[]), None);
    }

    #[test]
    fn a_truncated_position_is_refused() {
        let mut b = Vec::new();
        fixed32(&mut b, 1, lat(1.0));
        b.pop();
        assert_eq!(Position::parse(&b), None);
    }

    #[test]
    fn a_user_reads_names_and_key() {
        let mut b = Vec::new();
        put_bytes(&mut b, 1, b"!deadbeef");
        put_bytes(&mut b, 2, b"Seb F4NRM");
        put_bytes(&mut b, 3, b"SEB");
        put_bytes(&mut b, 4, &[1, 2, 3, 4, 5, 6]);
        varint(&mut b, 5, 43);
        varint(&mut b, 6, 1);
        varint(&mut b, 7, 2);
        put_bytes(&mut b, 8, &[7; 32]);
        varint(&mut b, 9, 1);
        let u = User::parse(&b).unwrap();
        assert_eq!(u.id, "!deadbeef");
        assert_eq!(u.long_name, "Seb F4NRM");
        assert_eq!(u.short_name, "SEB");
        assert_eq!(u.macaddr, Some([1, 2, 3, 4, 5, 6]));
        assert_eq!((u.hw_model, u.role), (43, 2));
        assert!(u.is_licensed);
        assert_eq!(u.public_key.len(), 32);
        assert_eq!(u.is_unmessagable, Some(true));
        assert_eq!(User::parse(&[]), None);
    }

    #[test]
    fn device_telemetry_reads_floats_and_counters() {
        let mut dev = Vec::new();
        varint(&mut dev, 1, 87);
        float32(&mut dev, 2, 4.1);
        float32(&mut dev, 3, 12.5);
        float32(&mut dev, 4, 1.25);
        varint(&mut dev, 5, 3600);
        let mut b = Vec::new();
        fixed32(&mut b, 1, 1_700_000_000);
        put_bytes(&mut b, 2, &dev);
        let t = Telemetry::parse(&b).unwrap();
        assert_eq!(t.time_unix, 1_700_000_000);
        let d = t.device.unwrap();
        assert_eq!(d.battery_level, 87);
        assert_eq!(d.voltage, 4.1);
        assert_eq!(d.channel_utilization, 12.5);
        assert_eq!(d.air_util_tx, 1.25);
        assert_eq!(d.uptime_seconds, 3600);
        assert!(t.environment.is_none());
    }

    #[test]
    fn environment_telemetry_keeps_the_wind_fields_on_the_right_numbers() {
        let mut env = Vec::new();
        float32(&mut env, 1, 21.5);
        varint(&mut env, 7, 120); // iaq is an integer
        varint(&mut env, 13, 270); // wind direction is an integer
        float32(&mut env, 14, 5.5);
        float32(&mut env, 15, 80.0); // weight
        float32(&mut env, 16, 9.0); // gust
        float32(&mut env, 17, 2.0); // lull
        let mut b = Vec::new();
        put_bytes(&mut b, 3, &env);
        let e = Telemetry::parse(&b).unwrap().environment.unwrap();
        assert_eq!(e.temperature_c, 21.5);
        assert_eq!(e.iaq, 120);
        assert_eq!(e.wind_direction, 270);
        assert_eq!(e.wind_speed, 5.5);
        assert_eq!(e.weight, 80.0);
        assert_eq!(e.wind_gust, 9.0);
        assert_eq!(e.wind_lull, 2.0);
    }

    #[test]
    fn power_channels_pair_voltage_and_current() {
        let mut p = Vec::new();
        float32(&mut p, 1, 5.0);
        float32(&mut p, 2, 0.5);
        float32(&mut p, 15, 3.3);
        float32(&mut p, 16, 0.1);
        let mut b = Vec::new();
        put_bytes(&mut b, 5, &p);
        let m = Telemetry::parse(&b).unwrap().power.unwrap();
        assert_eq!(m.channels[0], PowerChannel { voltage: 5.0, current: 0.5 });
        assert_eq!(m.channels[7], PowerChannel { voltage: 3.3, current: 0.1 });
        assert_eq!(m.channels[3], PowerChannel::default());
    }

    #[test]
    fn local_stats_keep_a_negative_noise_floor() {
        let mut s = Vec::new();
        varint(&mut s, 1, 99);
        varint(&mut s, 5, 1234);
        varint(&mut s, 15, (-110i32) as i64 as u64);
        let mut b = Vec::new();
        put_bytes(&mut b, 6, &s);
        let m = Telemetry::parse(&b).unwrap().local_stats.unwrap();
        assert_eq!((m.uptime_s, m.num_packets_rx), (99, 1234));
        assert_eq!(m.noise_floor_dbm, -110);
    }

    #[test]
    fn a_telemetry_cut_short_keeps_the_groups_that_arrived() {
        let mut dev = Vec::new();
        varint(&mut dev, 1, 50);
        let mut b = Vec::new();
        put_bytes(&mut b, 2, &dev);
        put_tag(&mut b, 3, WIRE_LENGTH);
        put_varint(&mut b, 40); // promises 40 bytes, none follow
        let t = Telemetry::parse(&b).unwrap();
        assert_eq!(t.device.unwrap().battery_level, 50);
        assert!(t.environment.is_none());
        assert_eq!(Telemetry::parse(&[]), None);
    }

    #[test]
    fn routing_tells_an_ack_from_a_failure() {
        let mut b = Vec::new();
        varint(&mut b, 3, 0);
        let ack = Routing::parse(&b).unwrap();
        assert_eq!(ack.kind, RoutingKind::Error);
        assert_eq!(ack.error_reason, Some(0));
        assert_eq!(routing_error_name(0), Some("NONE"));

        let mut b = Vec::new();
        varint(&mut b, 3, 3);
        assert_eq!(
            routing_error_name(Routing::parse(&b).unwrap().error_reason.unwrap()),
            Some("TIMEOUT")
        );
        assert_eq!(routing_error_name(99), None);
        assert_eq!(Routing::parse(&[]), None);
    }

    #[test]
    fn a_route_reads_packed_and_unpacked_hops() {
        // Packed: one length-delimited block of varints.
        let mut packed = Vec::new();
        for n in [0x11u64, 0x22, 0x3300_0044] {
            put_varint(&mut packed, n);
        }
        let mut b = Vec::new();
        put_bytes(&mut b, 1, &packed);
        // Unpacked: one tag per hop, as the back route here.
        varint(&mut b, 3, 0x55);
        varint(&mut b, 3, 0x66);
        // SNRs, one negative (a negative int32 is a ten-byte varint).
        varint(&mut b, 2, 20);
        varint(&mut b, 2, (-8i32) as i64 as u64);
        let d = RouteDiscovery::parse(&b).unwrap();
        assert_eq!(d.route, [0x11, 0x22, 0x3300_0044]);
        assert_eq!(d.route_back, [0x55, 0x66]);
        assert_eq!(d.snr_towards, [20, -8]);

        let mut r = Vec::new();
        put_bytes(&mut r, 1, &b);
        let routing = Routing::parse(&r).unwrap();
        assert_eq!(routing.kind, RoutingKind::Request);
        assert_eq!(routing.discovery.route.len(), 3);
        assert_eq!(RouteDiscovery::parse(&[]), None);
    }

    #[test]
    fn a_waypoint_reads_its_place_and_label() {
        let mut b = Vec::new();
        varint(&mut b, 1, 77);
        fixed32(&mut b, 2, lat(48.8584));
        fixed32(&mut b, 3, lat(2.2945));
        varint(&mut b, 4, 1_800_000_000);
        put_bytes(&mut b, 6, "Tour".as_bytes());
        put_bytes(&mut b, 7, "Eiffel".as_bytes());
        fixed32(&mut b, 8, 0x1F5FC);
        let w = Waypoint::parse(&b).unwrap();
        assert_eq!((w.id, w.expire, w.icon), (77, 1_800_000_000, 0x1F5FC));
        assert_eq!((w.name.as_str(), w.description.as_str()), ("Tour", "Eiffel"));
        assert!((w.lat_deg.unwrap() - 48.8584).abs() < 1e-6);
    }

    #[test]
    fn neighbor_info_lists_the_neighbours_and_caps_them() {
        let mut one = Vec::new();
        varint(&mut one, 1, 0xAABB);
        float32(&mut one, 2, -7.5);
        fixed32(&mut one, 3, 1_700_000_001);
        varint(&mut one, 4, 900);
        let mut b = Vec::new();
        varint(&mut b, 1, 0x1234);
        varint(&mut b, 2, 0x5678);
        varint(&mut b, 3, 1800);
        for _ in 0..20 {
            put_bytes(&mut b, 4, &one);
        }
        let n = NeighborInfo::parse(&b).unwrap();
        assert_eq!((n.node_id, n.last_sent_by_id), (0x1234, 0x5678));
        assert_eq!(n.neighbors.len(), MAX_NEIGHBORS);
        assert_eq!(
            n.neighbors[0],
            Neighbor {
                node_id: 0xAABB,
                snr_db: -7.5,
                last_rx_time: 1_700_000_001,
                node_broadcast_interval_secs: 900
            }
        );
        assert_eq!(NeighborInfo::parse(&[]), None);
    }

    #[test]
    fn key_verification_keeps_sizes_not_hashes() {
        let mut b = Vec::new();
        varint(&mut b, 1, 0x1122_3344_5566);
        put_bytes(&mut b, 2, &[9; 32]);
        put_bytes(&mut b, 3, &[8; 8]);
        let k = KeyVerification::parse(&b).unwrap();
        assert_eq!((k.nonce, k.hash1_len, k.hash2_len), (0x1122_3344_5566, 32, 8));
    }

    #[test]
    fn a_map_report_reads_the_node_and_its_place() {
        let mut b = Vec::new();
        put_bytes(&mut b, 1, b"Relay");
        put_bytes(&mut b, 2, b"RLY");
        varint(&mut b, 3, 4);
        put_bytes(&mut b, 5, b"2.5.9");
        varint(&mut b, 6, 3);
        varint(&mut b, 7, 0);
        fixed32(&mut b, 9, lat(45.0));
        fixed32(&mut b, 10, lat(-1.5));
        varint(&mut b, 11, 300);
        varint(&mut b, 13, 6);
        let m = MapReport::parse(&b).unwrap();
        assert_eq!((m.long_name.as_str(), m.firmware_version.as_str()), ("Relay", "2.5.9"));
        assert_eq!((m.region, m.altitude_m, m.num_online_local_nodes), (3, 300, 6));
        assert!((m.lon_deg.unwrap() + 1.5).abs() < 1e-6);
    }

    #[test]
    fn admin_reports_which_request_it_was() {
        let mut b = Vec::new();
        varint(&mut b, 5, 1); // a get_*_request
        put_bytes(&mut b, 101, &[1, 2, 3, 4]);
        let a = Admin::parse(&b).unwrap();
        assert_eq!(a, Admin { variant_field: 5, has_session_passkey: true });
        assert_eq!(Admin::parse(&[]), None);
    }

    #[test]
    fn atak_reads_a_position_report() {
        let mut contact = Vec::new();
        put_bytes(&mut contact, 1, b"VIPER");
        put_bytes(&mut contact, 2, b"dev-1");
        let mut group = Vec::new();
        varint(&mut group, 1, 2);
        varint(&mut group, 2, 5);
        let mut status = Vec::new();
        varint(&mut status, 1, 73);
        let mut pli = Vec::new();
        fixed32(&mut pli, 1, lat(51.5));
        fixed32(&mut pli, 2, lat(-0.12));
        varint(&mut pli, 3, 35);
        varint(&mut pli, 4, 3);
        varint(&mut pli, 5, 180);
        let mut b = Vec::new();
        put_bytes(&mut b, 2, &contact);
        put_bytes(&mut b, 3, &group);
        put_bytes(&mut b, 4, &status);
        put_bytes(&mut b, 5, &pli);
        let a = Atak::parse(&b).unwrap();
        assert_eq!(a.kind, Some(AtakKind::Pli));
        assert_eq!((a.callsign.as_str(), a.device_callsign.as_str()), ("VIPER", "dev-1"));
        assert_eq!((atak_role_name(a.role), atak_team_name(a.team)), ("TeamLead", "Red"));
        assert_eq!((a.battery, a.altitude_hae_m, a.speed_mps, a.course_deg), (73, 35, 3, 180));
        assert!((a.lat_deg.unwrap() - 51.5).abs() < 1e-6);
        assert_eq!(atak_team_name(99), "Unspecified");
        assert_eq!(atak_role_name(99), "Unknown");
    }

    #[test]
    fn atak_reads_a_chat_and_a_detail_block() {
        let mut chat = Vec::new();
        put_bytes(&mut chat, 1, b"on my way");
        put_bytes(&mut chat, 2, b"All Chat Rooms");
        put_bytes(&mut chat, 8, &[1]);
        let mut b = Vec::new();
        put_bytes(&mut b, 6, &chat);
        let a = Atak::parse(&b).unwrap();
        assert_eq!(a.kind, Some(AtakKind::Chat));
        let c = a.chat.unwrap();
        assert_eq!((c.message.as_str(), c.to.as_str()), ("on my way", "All Chat Rooms"));
        assert!(c.has_voice_profile);

        let mut d = Vec::new();
        put_bytes(&mut d, 7, b"<detail/>");
        let a = Atak::parse(&d).unwrap();
        assert_eq!((a.kind, a.detail_xml.as_slice()), (Some(AtakKind::Detail), &b"<detail/>"[..]));
        assert_eq!(Atak::parse(&[]), None);
    }

    #[test]
    fn remote_hardware_reads_the_gpio_masks() {
        let mut b = Vec::new();
        varint(&mut b, 1, 1);
        varint(&mut b, 2, 0xFF00_0000_0000);
        varint(&mut b, 3, 0x0F00_0000_0000);
        let h = RemoteHardware::parse(&b).unwrap();
        assert_eq!((h.kind, h.gpio_mask, h.gpio_value), (1, 0xFF00_0000_0000, 0x0F00_0000_0000));
    }

    #[test]
    fn store_forward_reads_stats_and_history() {
        let mut stats = Vec::new();
        varint(&mut stats, 1, 500);
        varint(&mut stats, 2, 120);
        varint(&mut stats, 3, 600);
        varint(&mut stats, 4, 86_400);
        varint(&mut stats, 5, 9);
        let mut hist = Vec::new();
        varint(&mut hist, 1, 40);
        varint(&mut hist, 2, 3600);
        let mut b = Vec::new();
        varint(&mut b, 1, 7);
        put_bytes(&mut b, 2, &stats);
        put_bytes(&mut b, 3, &hist);
        let s = StoreForward::parse(&b).unwrap();
        assert_eq!(store_forward_name(s.rr), Some("ROUTER_STATS"));
        assert_eq!(s.stats.unwrap().messages_saved, 120);
        assert_eq!(s.history.unwrap(), StoreForwardHistory { messages: 40, window_s: 3600 });
        assert_eq!(store_forward_name(1000), None);
    }

    #[test]
    fn paxcounter_reads_both_counts() {
        let mut b = Vec::new();
        varint(&mut b, 1, 31);
        varint(&mut b, 2, 12);
        varint(&mut b, 3, 7200);
        assert_eq!(
            Paxcounter::parse(&b).unwrap(),
            Paxcounter { wifi: 31, ble: 12, uptime_s: 7200 }
        );
    }

    #[test]
    fn the_dispatcher_picks_the_decoder_by_port() {
        assert_eq!(decode_payload(port::TEXT_MESSAGE, b"73"), Some(Decoded::Text("73".into())));
        assert_eq!(
            decode_payload(port::DETECTION_SENSOR, b"Detection event"),
            Some(Decoded::Detection("Detection event".into()))
        );
        assert_eq!(decode_payload(port::DETECTION_SENSOR, b""), None);
        let mut b = Vec::new();
        varint(&mut b, 1, 5);
        assert!(matches!(decode_payload(port::PAXCOUNTER, &b), Some(Decoded::Paxcounter(_))));
        // No decoder for these ports.
        for p in [port::AUDIO, port::RANGE_TEST, port::PRIVATE, 9999] {
            assert_eq!(decode_payload(p, &b), None);
        }
    }

    #[test]
    fn garbage_never_panics() {
        let ports = [
            port::REMOTE_HARDWARE,
            port::POSITION,
            port::NODEINFO,
            port::ROUTING,
            port::ADMIN,
            port::WAYPOINT,
            port::KEY_VERIFICATION,
            port::PAXCOUNTER,
            port::STORE_FORWARD,
            port::TELEMETRY,
            port::TRACEROUTE,
            port::NEIGHBORINFO,
            port::ATAK_PLUGIN,
            port::MAP_REPORT,
        ];
        let mut x = 0x1234_5678u32;
        for len in 0..64usize {
            let junk: Vec<u8> = (0..len)
                .map(|_| {
                    x = x.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                    (x >> 24) as u8
                })
                .collect();
            for &p in &ports {
                let _ = decode_payload(p, &junk);
            }
        }
    }
}
