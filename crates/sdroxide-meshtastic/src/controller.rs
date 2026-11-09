// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM

//! The receiver as the engine runs it: a worker thread that takes baseband IQ
//! from a fixed-rate lane, finds LoRa frames in it, opens them with the
//! operator's keys and keeps the table of nodes and the packet log.
//!
//! The realtime engine thread ships IQ blocks to the worker over a bounded
//! channel and drains status snapshots without blocking, the same shape as the
//! other decoder lanes. All the DSP runs on the worker. A dropped block costs
//! at most the frame it fell in, so the queue is the simple kind: `try_send`
//! and drop the whole block.

use std::collections::VecDeque;
use std::thread::JoinHandle;

use crossbeam_channel::{Receiver, Sender, bounded, select, unbounded};
use num_complex::Complex32;
use sdroxide_types::{
    MESH_LOG_DEPTH, MESH_MAX_NODES, MeshEntry, MeshNode, MeshtasticSettings, MeshtasticStatus,
};

use crate::decoders::Decoded;
use crate::demod::{DecoderConfig, LoraDecoder};
use crate::keys::KeySet;
use crate::packet::decode_frame;
use crate::protocol::{DEFAULT_PRESET, PRESETS, Preset, port};

/// The lane runs at this multiple of the preset's bandwidth. Two keeps the
/// channel edges clear of the decimation filter's roll-off.
pub const LANE_OVERSAMPLE: u32 = 2;

/// The preset name that means "listen for all of them at once".
pub const AUTO_PRESET: &str = "Auto";

/// The widest bandwidth of any preset: the automatic lane is cut for it, and
/// the narrower presets are decimated from it.
pub const AUTO_LANE_BW_HZ: u32 = 500_000;

/// Whether the settings ask for every preset to be tried at once.
pub fn is_auto(settings: &MeshtasticSettings) -> bool {
    settings.preset.eq_ignore_ascii_case(AUTO_PRESET)
}

/// The preset a setting names, the default one when the name is unknown. In
/// automatic mode this is the default preset too: it is what the channel slots
/// and a message sent from here are worked out from.
pub fn preset_of(settings: &MeshtasticSettings) -> &'static Preset {
    crate::protocol::lookup_preset(&settings.preset).unwrap_or(DEFAULT_PRESET)
}

/// The sample rate of the lane these settings need.
pub fn lane_rate_for(settings: &MeshtasticSettings) -> f64 {
    if is_auto(settings) {
        f64::from(AUTO_LANE_BW_HZ * LANE_OVERSAMPLE)
    } else {
        lane_rate_hz(preset_of(settings))
    }
}

/// The bandwidth the lane has to hold whole.
pub fn lane_bw_hz(settings: &MeshtasticSettings) -> u32 {
    if is_auto(settings) { AUTO_LANE_BW_HZ } else { preset_of(settings).bw_hz }
}

/// A name for the lane's shape: it changes exactly when the lane has to be
/// rebuilt because of the preset.
pub fn lane_name(settings: &MeshtasticSettings) -> &'static str {
    if is_auto(settings) { "AUTO" } else { preset_of(settings).name }
}

/// A low-pass and decimating filter for one narrower channel cut out of the
/// automatic lane. Polyphase in effect: it only computes the outputs it keeps.
struct Decimator {
    factor: usize,
    taps: Vec<f32>,
    buf: Vec<Complex32>,
    pos: usize,
}

impl Decimator {
    /// `cutoff` is the pass-band edge in cycles per input sample.
    fn new(factor: usize, cutoff: f64) -> Self {
        let n = 32 * factor + 1;
        let mid = (n - 1) as f64 / 2.0;
        let mut taps: Vec<f32> = (0..n)
            .map(|i| {
                let x = i as f64 - mid;
                let sinc = if x == 0.0 {
                    2.0 * cutoff
                } else {
                    (2.0 * std::f64::consts::PI * cutoff * x).sin() / (std::f64::consts::PI * x)
                };
                let w = 0.54 - 0.46 * (2.0 * std::f64::consts::PI * i as f64 / (n - 1) as f64).cos();
                (sinc * w) as f32
            })
            .collect();
        let sum: f32 = taps.iter().sum();
        taps.iter_mut().for_each(|t| *t /= sum);
        Decimator { factor, taps, buf: Vec::new(), pos: n - 1 }
    }

    fn process(&mut self, input: &[Complex32], out: &mut Vec<Complex32>) {
        self.buf.extend_from_slice(input);
        let n = self.taps.len();
        while self.pos < self.buf.len() {
            let mut acc = Complex32::new(0.0, 0.0);
            for (k, t) in self.taps.iter().enumerate() {
                acc += self.buf[self.pos - k] * *t;
            }
            out.push(acc);
            self.pos += self.factor;
        }
        let drop = self.pos - (n - 1);
        if drop <= self.buf.len() {
            self.buf.drain(..drop);
            self.pos = n - 1;
        }
    }
}

/// The sample rate of the lane a preset needs.
pub fn lane_rate_hz(preset: &Preset) -> f64 {
    f64::from(preset.bw_hz * LANE_OVERSAMPLE)
}

/// Realtime data, dropped on backpressure.
struct Iq {
    samples: Vec<Complex32>,
}

/// Control traffic, never dropped.
enum Ctl {
    Config(Box<MeshtasticSettings>),
    Stop,
}

/// Bounded IQ queue depth: a couple of seconds of a 500 kS/s lane at a typical
/// device read.
const IQ_QUEUE_DEPTH: usize = 256;

/// How often a status goes out when nothing has decoded: once every N blocks,
/// so the level meter keeps moving without flooding the event channel.
const IDLE_TICK: u64 = 100;

pub struct MeshtasticController {
    iq_tx: Sender<Iq>,
    ctl_tx: Sender<Ctl>,
    res_rx: Receiver<MeshtasticStatus>,
    worker: Option<JoinHandle<()>>,
}

impl MeshtasticController {
    pub fn new(cfg: MeshtasticSettings) -> Self {
        let (iq_tx, iq_rx) = bounded::<Iq>(IQ_QUEUE_DEPTH);
        let (ctl_tx, ctl_rx) = unbounded::<Ctl>();
        let (res_tx, res_rx) = unbounded::<MeshtasticStatus>();
        let worker = std::thread::Builder::new()
            .name("sdroxide-meshtastic".into())
            .spawn(move || run(cfg, iq_rx, ctl_rx, res_tx))
            .expect("spawn meshtastic worker");
        MeshtasticController { iq_tx, ctl_tx, res_rx, worker: Some(worker) }
    }

    /// Realtime path: hand a block of lane-rate IQ to the worker. Non-blocking.
    pub fn on_rx_iq(&self, iq: &[Complex32]) {
        let _ = self.iq_tx.try_send(Iq { samples: iq.to_vec() });
    }

    /// Apply new settings to the running worker. A new preset rebuilds the
    /// demodulator, so the lane must be re-seated at [`lane_rate_hz`] of it.
    pub fn set_config(&self, cfg: &MeshtasticSettings) {
        let _ = self.ctl_tx.send(Ctl::Config(Box::new(cfg.clone())));
    }

    /// Drain the latest status, non-blocking. Only the newest matters.
    pub fn poll(&self) -> Option<MeshtasticStatus> {
        let mut out = None;
        while let Ok(s) = self.res_rx.try_recv() {
            out = Some(s);
        }
        out
    }
}

impl Drop for MeshtasticController {
    fn drop(&mut self) {
        let _ = self.ctl_tx.send(Ctl::Stop);
        if let Some(h) = self.worker.take() {
            let _ = h.join();
        }
    }
}

fn now_unix() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

fn keys_of(cfg: &MeshtasticSettings) -> KeySet {
    let mut keys = KeySet::with_defaults();
    keys.add_csv(&cfg.keys);
    keys
}

fn decoder_for(preset: &Preset) -> Option<LoraDecoder> {
    LoraDecoder::new(DecoderConfig {
        sf: preset.sf,
        cr: preset.cr,
        bw_hz: preset.bw_hz,
        os_factor: LANE_OVERSAMPLE as usize,
        center_freq_hz: 0.0,
    })
}

/// What listens on the lane: one demodulator for the chosen preset, or, in
/// automatic mode, one for every (spreading factor, bandwidth) pair of the
/// presets, each fed from the same wide lane through its own decimator.
struct Rx {
    name: &'static str,
    chains: Vec<(LoraDecoder, Option<Decimator>)>,
    scratch: Vec<Complex32>,
}

impl Rx {
    fn new(cfg: &MeshtasticSettings) -> Option<Rx> {
        let name = lane_name(cfg);
        let mut chains = Vec::new();
        if is_auto(cfg) {
            let mut seen: Vec<(u8, u32)> = Vec::new();
            for p in &PRESETS {
                if seen.contains(&(p.sf, p.bw_hz)) {
                    continue;
                }
                seen.push((p.sf, p.bw_hz));
                let factor = (AUTO_LANE_BW_HZ / p.bw_hz).max(1) as usize;
                let decim = (factor > 1).then(|| {
                    // The half-power point sits at three quarters of the
                    // bandwidth: the chirp's edges (half the bandwidth either
                    // side of the carrier) stay in the flat part, and the
                    // stop band starts about where the new Nyquist is.
                    let cutoff = f64::from(p.bw_hz) * 0.75 / f64::from(AUTO_LANE_BW_HZ * LANE_OVERSAMPLE);
                    Decimator::new(factor, cutoff)
                });
                chains.push((decoder_for(p)?, decim));
            }
        } else {
            chains.push((decoder_for(preset_of(cfg))?, None));
        }
        Some(Rx { name, chains, scratch: Vec::new() })
    }

    fn feed(&mut self, block: &[Complex32]) -> Vec<crate::demod::LoraFrame> {
        let mut frames = Vec::new();
        for (decoder, decim) in &mut self.chains {
            match decim {
                Some(d) => {
                    self.scratch.clear();
                    d.process(block, &mut self.scratch);
                    frames.extend(decoder.feed(&self.scratch));
                }
                None => frames.extend(decoder.feed(block)),
            }
        }
        frames
    }

    fn preamble_locks(&self) -> u64 {
        self.chains.iter().map(|(d, _)| d.stats().preamble_locks).sum()
    }
}

/// The nodes heard and the packet log, and what a packet changes in them.
#[derive(Default)]
struct Book {
    nodes: Vec<MeshNode>,
    log: VecDeque<MeshEntry>,
}

impl Book {
    fn node(&mut self, id: u32) -> &mut MeshNode {
        if let Some(i) = self.nodes.iter().position(|n| n.id == id) {
            return &mut self.nodes[i];
        }
        if self.nodes.len() >= MESH_MAX_NODES {
            // Make room by dropping the node heard longest ago.
            if let Some(oldest) =
                self.nodes.iter().enumerate().min_by_key(|(_, n)| n.last_heard).map(|(i, _)| i)
            {
                self.nodes.swap_remove(oldest);
            }
        }
        self.nodes.push(MeshNode { id, ..Default::default() });
        self.nodes.last_mut().expect("just pushed")
    }

    fn hear(&mut self, entry: MeshEntry, decoded: Option<Decoded>) {
        let n = self.node(entry.from);
        n.last_heard = entry.time;
        n.snr_db = entry.snr_db;
        n.packets = n.packets.saturating_add(1);
        n.hops = entry.hops;
        match decoded {
            Some(Decoded::NodeInfo(u)) => {
                n.long_name = u.long_name;
                n.short_name = u.short_name;
                n.hw_model = u.hw_model;
                n.role = u.role;
            }
            Some(Decoded::Position(p)) => {
                n.lat_deg = p.lat_deg;
                n.lon_deg = p.lon_deg;
                n.altitude_m = p.altitude_m;
            }
            Some(Decoded::Telemetry(t)) => {
                if let Some(d) = t.device {
                    n.battery_pct = Some(d.battery_level);
                    n.voltage = Some(d.voltage);
                }
            }
            Some(Decoded::MapReport(m)) => {
                n.long_name = m.long_name;
                n.short_name = m.short_name;
                n.hw_model = m.hw_model;
                n.role = m.role;
                n.lat_deg = m.lat_deg;
                n.lon_deg = m.lon_deg;
                n.altitude_m = Some(m.altitude_m);
            }
            _ => {}
        }
        if self.log.len() >= MESH_LOG_DEPTH {
            self.log.pop_back();
        }
        self.log.push_front(entry);
    }

    fn snapshot(&self) -> (Vec<MeshNode>, Vec<MeshEntry>) {
        let mut nodes = self.nodes.clone();
        nodes.sort_by_key(|n| std::cmp::Reverse(n.last_heard));
        (nodes, self.log.iter().cloned().collect())
    }
}

fn level_dbfs(block: &[Complex32]) -> f32 {
    if block.is_empty() {
        return f32::MIN;
    }
    let p = block.iter().map(|s| s.norm_sqr()).sum::<f32>() / block.len() as f32;
    10.0 * p.max(1e-12).log10()
}

fn run(
    cfg: MeshtasticSettings,
    iq_rx: Receiver<Iq>,
    ctl_rx: Receiver<Ctl>,
    res_tx: Sender<MeshtasticStatus>,
) {
    let mut cfg = cfg;
    let Some(mut rx) = Rx::new(&cfg) else {
        tracing::error!(preset = lane_name(&cfg), "meshtastic demodulator could not start");
        return;
    };
    let mut keys = keys_of(&cfg);
    let mut book = Book::default();
    let (mut frames, mut crc_bad, mut tick) = (0u64, 0u64, 0u64);
    let mut level = f32::MIN;
    // The preamble locks of demodulators built before the present one.
    let mut locks_before = 0u64;

    loop {
        select! {
            recv(ctl_rx) -> msg => match msg {
                Ok(Ctl::Config(next)) => {
                    if lane_name(&next) != rx.name {
                        if let Some(new_rx) = Rx::new(&next) {
                            locks_before += rx.preamble_locks();
                            rx = new_rx;
                        }
                    }
                    if next.keys != cfg.keys {
                        keys = keys_of(&next);
                    }
                    cfg = *next;
                }
                Ok(Ctl::Stop) | Err(_) => break,
            },
            recv(iq_rx) -> msg => match msg {
                Ok(Iq { samples }) => {
                    tick += 1;
                    let block_level = level_dbfs(&samples);
                    level = if level == f32::MIN { block_level } else { 0.8 * level + 0.2 * block_level };
                    let mut heard = false;
                    if cfg.enabled {
                        for f in rx.feed(&samples) {
                            if !f.crc_ok {
                                crc_bad += 1;
                                heard = true;
                                continue;
                            }
                            frames += 1;
                            heard = true;
                            let Some(pkt) = decode_frame(&f.payload, &keys) else { continue };
                            let data = pkt.data.as_ref();
                            let decoded = data.and_then(|d| d.decode());
                            let hops = (pkt.header.hop_start() > 0)
                                .then(|| pkt.header.hop_start().saturating_sub(pkt.header.hop_limit()));
                            let text = match &decoded {
                                Some(Decoded::Text(t)) => Some(t.clone()),
                                _ => None,
                            };
                            let entry = MeshEntry {
                                time: now_unix(),
                                from: pkt.header.from,
                                to: pkt.header.to,
                                packet_id: pkt.header.packet_id,
                                port: data.map_or(port::UNKNOWN, |d| d.portnum),
                                channel: pkt.channel_name.clone(),
                                snr_db: f.snr_db,
                                hops,
                                encrypted: !pkt.decrypted,
                                via_mqtt: pkt.header.via_mqtt(),
                                text,
                            };
                            book.hear(entry, decoded);
                        }
                    }
                    if heard || tick % IDLE_TICK == 0 || tick == 1 {
                        let (nodes, log) = book.snapshot();
                        let _ = res_tx.send(MeshtasticStatus {
                            running: cfg.enabled,
                            // The engine knows the window and the band list;
                            // it fills these two in before the status leaves.
                            in_window: true,
                            refused: None,
                            level_dbfs: level,
                            preambles: locks_before + rx.preamble_locks(),
                            frames,
                            crc_bad,
                            nodes,
                            log,
                            // The transmit half is the engine's: it fills it in.
                            ..MeshtasticStatus::default()
                        });
                    }
                }
                Err(_) => break,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modulate::{TxParams, modulate};
    use crate::packet::{Data, Header, build_frame};
    use crate::protobuf::{WIRE_FIXED32, WIRE_VARINT, put_bytes, put_tag, put_varint};
    use crate::protocol::{BROADCAST_NODE, DEFAULT_PSK};

    fn on() -> MeshtasticSettings {
        MeshtasticSettings { enabled: true, preset: "LongFast".into(), ..Default::default() }
    }

    fn wait_for(
        c: &MeshtasticController,
        pred: impl Fn(&MeshtasticStatus) -> bool,
    ) -> Option<MeshtasticStatus> {
        let mut latest = None;
        for _ in 0..500 {
            if let Some(s) = c.poll() {
                let hit = pred(&s);
                latest = Some(s);
                if hit {
                    return latest;
                }
            }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        latest
    }

    /// One frame on the default channel, as lane-rate IQ with silence around it.
    fn air(from: u32, id: u32, portnum: u32, payload: Vec<u8>) -> Vec<Complex32> {
        let p = preset_of(&on());
        let header =
            Header { to: BROADCAST_NODE, from, packet_id: id, flags: 0x62, ..Default::default() };
        let data = Data { portnum, payload, ..Default::default() };
        let frame = build_frame(header, p.channel_name, &DEFAULT_PSK, &data).unwrap();
        let mut tx = TxParams::meshtastic(p.sf, p.cr, p.bw_hz);
        tx.os = LANE_OVERSAMPLE as usize;
        let pad = vec![Complex32::new(0.0, 0.0); 8 * (1usize << p.sf) * tx.os];
        let mut iq = pad.clone();
        iq.extend(modulate(&frame, &tx));
        iq.extend(pad);
        iq
    }

    fn feed(c: &MeshtasticController, iq: &[Complex32]) {
        for chunk in iq.chunks(4096) {
            c.on_rx_iq(chunk);
        }
    }

    #[test]
    fn the_default_settings_name_a_real_preset() {
        let p = preset_of(&MeshtasticSettings::default());
        assert_eq!(p.channel_name, "LongFast");
        assert_eq!(lane_rate_hz(p), 500_000.0);
        assert!(crate::protocol::lookup_region(sdroxide_types::MESH_DEFAULT_REGION).is_some());
    }

    #[test]
    fn a_text_on_the_lane_reaches_the_log_and_the_node_table() {
        let c = MeshtasticController::new(on());
        feed(&c, &air(0xCAFE_F00D, 1, port::TEXT_MESSAGE, b"73 de F4NRM".to_vec()));
        let s = wait_for(&c, |s| s.frames >= 1).expect("a status arrives");
        assert!(s.running);
        assert_eq!(s.frames, 1);
        assert_eq!(s.crc_bad, 0);
        assert_eq!(s.log.len(), 1);
        let e = &s.log[0];
        assert_eq!((e.from, e.to, e.port), (0xCAFE_F00D, BROADCAST_NODE, port::TEXT_MESSAGE));
        assert_eq!(e.text.as_deref(), Some("73 de F4NRM"));
        assert_eq!(e.channel, "LongFast");
        assert!(!e.encrypted);
        // The header says three hops started and two are left: one relay.
        assert_eq!(e.hops, Some(1));
        assert_eq!(s.nodes.len(), 1);
        assert_eq!(s.nodes[0].id, 0xCAFE_F00D);
        assert_eq!(s.nodes[0].packets, 1);
    }

    #[test]
    fn node_info_and_a_position_fill_in_the_node() {
        let mut user = Vec::new();
        put_bytes(&mut user, 1, b"!cafef00d");
        put_bytes(&mut user, 2, b"Seb F4NRM");
        put_bytes(&mut user, 3, b"SEB");
        let mut pos = Vec::new();
        put_tag(&mut pos, 1, WIRE_FIXED32);
        pos.extend_from_slice(&((48.8584f64 * 1e7).round() as i32).to_le_bytes());
        put_tag(&mut pos, 2, WIRE_FIXED32);
        pos.extend_from_slice(&((2.2945f64 * 1e7).round() as i32).to_le_bytes());
        put_tag(&mut pos, 3, WIRE_VARINT);
        put_varint(&mut pos, 35);

        let c = MeshtasticController::new(on());
        feed(&c, &air(0xCAFE_F00D, 2, port::NODEINFO, user));
        feed(&c, &air(0xCAFE_F00D, 3, port::POSITION, pos));
        let s = wait_for(&c, |s| s.frames >= 2).expect("a status arrives");
        assert_eq!(s.nodes.len(), 1);
        let n = &s.nodes[0];
        assert_eq!((n.long_name.as_str(), n.short_name.as_str()), ("Seb F4NRM", "SEB"));
        assert_eq!(n.packets, 2);
        assert!((n.lat_deg.unwrap() - 48.8584).abs() < 1e-6);
        assert_eq!(n.altitude_m, Some(35));
        // Newest first.
        assert_eq!(s.log[0].port, port::POSITION);
        assert_eq!(s.log[1].port, port::NODEINFO);
    }

    #[test]
    fn a_frame_no_key_opens_is_logged_as_encrypted() {
        let c = MeshtasticController::new(on());
        // The same frame, on a channel whose key we do not hold.
        let p = preset_of(&on());
        let header =
            Header { to: BROADCAST_NODE, from: 7, packet_id: 9, flags: 0x07, ..Default::default() };
        let data =
            Data { portnum: port::TEXT_MESSAGE, payload: b"secret".to_vec(), ..Default::default() };
        let frame = build_frame(header, "Private", &[0x5A; 16], &data).unwrap();
        let mut tx = TxParams::meshtastic(p.sf, p.cr, p.bw_hz);
        tx.os = LANE_OVERSAMPLE as usize;
        let pad = vec![Complex32::new(0.0, 0.0); 8 * (1usize << p.sf) * tx.os];
        let mut iq = pad.clone();
        iq.extend(modulate(&frame, &tx));
        iq.extend(pad);
        feed(&c, &iq);
        let s = wait_for(&c, |s| s.frames >= 1).expect("a status arrives");
        assert_eq!(s.log.len(), 1);
        assert!(s.log[0].encrypted);
        assert_eq!(s.log[0].text, None);
        assert_eq!(s.log[0].from, 7);
        // With the key added, the next frame opens.
        c.set_config(&MeshtasticSettings {
            keys: format!("Private=hex:{}", "5a".repeat(16)),
            ..on()
        });
        std::thread::sleep(std::time::Duration::from_millis(100));
        feed(&c, &iq);
        let s = wait_for(&c, |s| s.frames >= 2).expect("a second status");
        assert!(!s.log[0].encrypted);
        assert_eq!(s.log[0].text.as_deref(), Some("secret"));
    }

    #[test]
    fn a_disabled_lane_decodes_nothing() {
        let c = MeshtasticController::new(MeshtasticSettings::default());
        feed(&c, &air(1, 1, port::TEXT_MESSAGE, b"hi".to_vec()));
        let s = wait_for(&c, |s| s.level_dbfs > f32::MIN).expect("an idle snapshot arrives");
        assert!(!s.running);
        assert_eq!((s.frames, s.log.len(), s.nodes.len()), (0, 0, 0));
    }

    #[test]
    fn a_new_preset_rebuilds_the_demodulator() {
        let c = MeshtasticController::new(on());
        // ShortFast: SF7 at 250 kHz, a different symbol length from LongFast.
        c.set_config(&MeshtasticSettings { preset: "ShortFast".into(), ..on() });
        let p = crate::protocol::lookup_preset("ShortFast").unwrap();
        let header =
            Header { to: BROADCAST_NODE, from: 5, packet_id: 4, flags: 0x07, ..Default::default() };
        let data =
            Data { portnum: port::TEXT_MESSAGE, payload: b"fast".to_vec(), ..Default::default() };
        let frame = build_frame(header, p.channel_name, &DEFAULT_PSK, &data).unwrap();
        let mut tx = TxParams::meshtastic(p.sf, p.cr, p.bw_hz);
        tx.os = LANE_OVERSAMPLE as usize;
        let pad = vec![Complex32::new(0.0, 0.0); 8 * (1usize << p.sf) * tx.os];
        let mut iq = pad.clone();
        iq.extend(modulate(&frame, &tx));
        iq.extend(pad);
        std::thread::sleep(std::time::Duration::from_millis(100));
        feed(&c, &iq);
        let s = wait_for(&c, |s| s.frames >= 1).expect("a status arrives");
        assert_eq!(s.log[0].text.as_deref(), Some("fast"));
    }

    /// A text of `preset`, as IQ at the automatic lane's 1 MS/s.
    fn air_on_the_auto_lane(preset: &str, from: u32, text: &str) -> Vec<Complex32> {
        let p = crate::protocol::lookup_preset(preset).unwrap();
        let header =
            Header { to: BROADCAST_NODE, from, packet_id: from, flags: 0x62, ..Default::default() };
        let data = Data {
            portnum: port::TEXT_MESSAGE,
            payload: text.as_bytes().to_vec(),
            ..Default::default()
        };
        let frame = build_frame(header, p.channel_name, &DEFAULT_PSK, &data).unwrap();
        let mut tx = TxParams::meshtastic(p.sf, p.cr, p.bw_hz);
        tx.os = (AUTO_LANE_BW_HZ * LANE_OVERSAMPLE / p.bw_hz) as usize;
        let pad = vec![Complex32::new(0.0, 0.0); 8 * (1usize << p.sf) * tx.os];
        let mut iq = pad.clone();
        iq.extend(modulate(&frame, &tx));
        iq.extend(pad);
        iq
    }

    #[test]
    fn the_automatic_mode_decodes_several_presets_on_one_wide_lane() {
        let auto = MeshtasticSettings { preset: "Auto".into(), ..on() };
        assert!(is_auto(&auto));
        assert_eq!(lane_rate_for(&auto), 1_000_000.0);
        assert_eq!(lane_rate_for(&on()), 500_000.0);
        let c = MeshtasticController::new(auto);
        let sent = [("LongFast", 11, "long fast"), ("ShortFast", 12, "short fast")];
        for (preset, from, text) in sent {
            // Paced: a debug build cannot take a 1 MS/s lane at full speed, and
            // a full queue drops blocks.
            for chunk in air_on_the_auto_lane(preset, from, text).chunks(4096) {
                c.on_rx_iq(chunk);
                std::thread::sleep(std::time::Duration::from_millis(2));
            }
        }
        let s = wait_for(&c, |s| s.frames >= 2).expect("a status arrives");
        assert_eq!((s.frames, s.crc_bad), (2, 0));
        let mut texts: Vec<_> = s.log.iter().filter_map(|e| e.text.clone()).collect();
        texts.sort();
        assert_eq!(texts, ["long fast", "short fast"]);
    }

    /// The slowest preset, at a quarter of the lane's bandwidth, and the
    /// fastest, at all of it, through the same bank.
    #[test]
    fn the_bank_decodes_the_narrowest_and_the_widest_preset() {
        let auto = MeshtasticSettings { preset: "Auto".into(), ..on() };
        for preset in ["ShortTurbo", "LongSlow"] {
            let mut rx = Rx::new(&auto).unwrap();
            let mut got = 0;
            for chunk in air_on_the_auto_lane(preset, 5, "x").chunks(4096) {
                got += rx.feed(chunk).iter().filter(|f| f.crc_ok).count();
            }
            assert_eq!(got, 1, "{preset}");
        }
    }

    #[test]
    fn the_decimator_keeps_a_tone_in_the_channel_and_drops_one_outside() {
        let (n, f) = (20_000usize, 4usize);
        let tone = |hz: f64| -> Vec<Complex32> {
            (0..n)
                .map(|i| {
                    let ph = 2.0 * std::f64::consts::PI * hz * i as f64 / 1e6;
                    Complex32::new(ph.cos() as f32, ph.sin() as f32)
                })
                .collect()
        };
        let power = |v: &[Complex32]| v[v.len() / 2..].iter().map(|c| c.norm_sqr()).sum::<f32>() / (v.len() / 2) as f32;
        // 125 kHz channel: its edge is at 62.5 kHz, the new Nyquist at 125 kHz.
        let mut d = Decimator::new(f, 125_000.0 * 0.75 / 1e6);
        let mut inside = Vec::new();
        d.process(&tone(60_000.0), &mut inside);
        let mut d = Decimator::new(f, 125_000.0 * 0.75 / 1e6);
        let mut outside = Vec::new();
        d.process(&tone(300_000.0), &mut outside);
        assert!(power(&inside) > 0.8, "in-channel {}", power(&inside));
        assert!(power(&outside) < 0.01, "out-of-channel {}", power(&outside));
        assert_eq!(inside.len(), (n - 32 * f) / f);
    }

    #[test]
    fn noise_never_decodes() {
        let c = MeshtasticController::new(on());
        let noise: Vec<Complex32> = (0..500_000)
            .map(|i| Complex32::new((i as f32 * 0.7).sin(), (i as f32 * 1.9 + 1.0).sin()) * 0.1)
            .collect();
        feed(&c, &noise);
        let s = wait_for(&c, |s| s.running).expect("a snapshot");
        assert_eq!(s.frames, 0);
        assert!(s.log.is_empty());
    }
}
