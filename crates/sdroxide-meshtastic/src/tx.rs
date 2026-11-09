// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM
//
//! What a text message needs before it can go on the air: the frame (header,
//! encrypted envelope), the modem parameters of the preset, how long the frame
//! lasts, and the duty-cycle bookkeeping a licence-free band asks for.
//!
//! Pure and clock-free: the caller gives the time. Nothing here keys a radio.

use std::collections::VecDeque;

use crate::keys::KeySet;
use crate::modulate::{TxParams, frame_samples};
use crate::packet::{Data, Header, build_frame};
use crate::protocol::{BROADCAST_NODE, DEFAULT_PSK, Preset, Region, port};

/// The longest text sent in one frame, in bytes. The radio frame holds 255
/// bytes; the header, the envelope and the encryption take the rest, and a
/// message that long is already a lot of air time.
pub const MAX_TEXT_BYTES: usize = 200;

/// Hops a sent message may take: the firmware's default.
pub const HOP_LIMIT: u8 = 3;

/// The length of the window a duty cycle is counted over, in seconds.
pub const DUTY_WINDOW_S: f64 = 3600.0;

/// The share of an hour a station may transmit in `region`, in percent, or
/// `None` where this program sets no limit of its own.
///
/// EU 868 is held to 1 % (the strictest sub-band rule that applies across the
/// 868 MHz band), EU 433 to 10 %. Elsewhere the rule is the operator's licence
/// and the country's, which this table does not know.
pub fn duty_cycle_percent(region: &str) -> Option<f32> {
    match crate::protocol::lookup_region(region).map(|r| r.name) {
        Some("EU_868") => Some(1.0),
        Some("EU_433") => Some(10.0),
        _ => None,
    }
}

/// The modem parameters a frame of `preset` is sent with in `region`.
pub fn tx_params(preset: &Preset, region: &Region) -> TxParams {
    let (bw, preamble) = if region.wide {
        (preset.bw_hz_wide, preset.preamble_wide)
    } else {
        (preset.bw_hz, preset.preamble)
    };
    TxParams { preamble, ..TxParams::meshtastic(preset.sf, preset.cr, bw) }
}

/// The key a frame on `preset`'s channel is encrypted with: the one the key
/// list names for that channel, else the default key.
pub fn channel_key(preset: &Preset, keys_csv: &str) -> Vec<u8> {
    let mut set = KeySet::new();
    set.add_csv(keys_csv);
    set.entries()
        .iter()
        .find(|e| e.channel_name == preset.channel_name)
        .map_or_else(|| DEFAULT_PSK.to_vec(), |e| e.psk.clone())
}

/// A text message as the frame that goes on the air. `None` when the text is
/// empty or longer than [`MAX_TEXT_BYTES`], or the key cannot encrypt.
pub fn build_text_frame(
    from: u32,
    to: u32,
    packet_id: u32,
    text: &str,
    preset: &Preset,
    psk: &[u8],
) -> Option<Vec<u8>> {
    if text.is_empty() || text.len() > MAX_TEXT_BYTES {
        return None;
    }
    let header = Header {
        to,
        from,
        packet_id,
        // Hop limit and hop start the same: the message has not been relayed.
        flags: HOP_LIMIT | (HOP_LIMIT << crate::protocol::FLAG_HOP_START_SHIFT),
        ..Header::default()
    };
    let data =
        Data { portnum: port::TEXT_MESSAGE, payload: text.as_bytes().to_vec(), ..Data::default() };
    build_frame(header, preset.channel_name, psk, &data)
}

/// Whether `to` names everyone.
pub fn is_broadcast(to: u32) -> bool {
    to == BROADCAST_NODE
}

/// How long a frame of `frame_len` bytes lasts on the air, in seconds.
pub fn airtime_s(frame_len: usize, p: &TxParams) -> f64 {
    frame_samples(frame_len, p) as f64 / f64::from(p.sample_rate())
}

/// The transmissions of the last hour, for the duty cycle.
#[derive(Debug, Clone, Default)]
pub struct DutyTracker {
    /// `(end time, air time)` in seconds, oldest first.
    sent: VecDeque<(f64, f64)>,
}

/// Why a transmission may not start yet.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DutyRefusal {
    /// Wait this many seconds, and it fits.
    Wait(f64),
    /// It would never fit: one frame is longer than the whole budget.
    TooLong,
}

impl DutyTracker {
    fn expire(&mut self, now: f64) {
        while self.sent.front().is_some_and(|(end, _)| end + DUTY_WINDOW_S <= now) {
            self.sent.pop_front();
        }
    }

    /// Seconds of air time used in the last hour.
    pub fn used_s(&mut self, now: f64) -> f64 {
        self.expire(now);
        self.sent.iter().map(|(_, a)| a).sum()
    }

    /// Whether `airtime_s` more may be sent at `now` under `percent` of an
    /// hour (`None`: no limit).
    pub fn check(
        &mut self,
        now: f64,
        airtime_s: f64,
        percent: Option<f32>,
    ) -> Result<(), DutyRefusal> {
        let Some(pct) = percent else { return Ok(()) };
        let budget = f64::from(pct) / 100.0 * DUTY_WINDOW_S;
        if airtime_s > budget {
            return Err(DutyRefusal::TooLong);
        }
        let mut used = self.used_s(now);
        if used + airtime_s <= budget {
            return Ok(());
        }
        // The oldest transmissions leave the hour first: the wait is until
        // enough of them have.
        for (end, air) in &self.sent {
            used -= air;
            if used + airtime_s <= budget {
                return Err(DutyRefusal::Wait((end + DUTY_WINDOW_S - now).max(0.0)));
            }
        }
        Err(DutyRefusal::TooLong)
    }

    /// Note a transmission that ended at `now`.
    pub fn record(&mut self, now: f64, airtime_s: f64) {
        self.expire(now);
        self.sent.push_back((now, airtime_s));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::keys::KeySet;
    use crate::packet::decode_frame;
    use crate::protocol::{lookup_preset, lookup_region};

    #[test]
    fn eu_868_is_held_to_one_percent() {
        assert_eq!(duty_cycle_percent("EU_868"), Some(1.0));
        assert_eq!(duty_cycle_percent("EU868"), Some(1.0));
        assert_eq!(duty_cycle_percent("EU_433"), Some(10.0));
        assert_eq!(duty_cycle_percent("US"), None);
        assert_eq!(duty_cycle_percent("nowhere"), None);
    }

    #[test]
    fn a_sent_text_decodes_with_the_default_key() {
        let p = lookup_preset("LongFast").unwrap();
        let f =
            build_text_frame(0x1234_5678, BROADCAST_NODE, 0xCAFE_F00D, "bonjour", p, &DEFAULT_PSK)
                .unwrap();
        let pkt = decode_frame(&f, &KeySet::with_defaults()).unwrap();
        assert!(pkt.decrypted);
        assert_eq!(pkt.header.from, 0x1234_5678);
        assert_eq!(pkt.header.packet_id, 0xCAFE_F00D);
        assert_eq!(pkt.header.hop_limit(), HOP_LIMIT);
        assert_eq!(pkt.header.hop_start(), HOP_LIMIT);
        let d = pkt.data.unwrap();
        assert_eq!(d.portnum, port::TEXT_MESSAGE);
        assert_eq!(d.payload, b"bonjour");
    }

    #[test]
    fn an_empty_or_huge_text_is_not_a_frame() {
        let p = lookup_preset("LongFast").unwrap();
        assert!(build_text_frame(1, 2, 3, "", p, &DEFAULT_PSK).is_none());
        let long = "x".repeat(MAX_TEXT_BYTES + 1);
        assert!(build_text_frame(1, 2, 3, &long, p, &DEFAULT_PSK).is_none());
        let max = "x".repeat(MAX_TEXT_BYTES);
        assert!(build_text_frame(1, 2, 3, &max, p, &DEFAULT_PSK).is_some());
    }

    #[test]
    fn the_named_key_wins_over_the_default() {
        let p = lookup_preset("LongFast").unwrap();
        assert_eq!(channel_key(p, ""), DEFAULT_PSK.to_vec());
        assert_eq!(channel_key(p, "Other=simple2"), DEFAULT_PSK.to_vec());
        let k = channel_key(p, "LongFast=hex:00112233445566778899aabbccddeeff");
        assert_eq!(k.len(), 16);
        assert_ne!(k, DEFAULT_PSK.to_vec());
    }

    #[test]
    fn long_fast_takes_about_half_a_second_for_a_short_text() {
        let p = lookup_preset("LongFast").unwrap();
        let r = lookup_region("EU_868").unwrap();
        let params = tx_params(p, r);
        assert_eq!((params.sf, params.cr, params.bw_hz), (11, 5, 250_000));
        let f = build_text_frame(1, BROADCAST_NODE, 2, "hello", p, &DEFAULT_PSK).unwrap();
        let t = airtime_s(f.len(), &params);
        assert!((0.3..1.2).contains(&t), "{t} s");
    }

    #[test]
    fn the_duty_cycle_counts_the_last_hour_only() {
        let mut d = DutyTracker::default();
        // 1 % of an hour is 36 s: three frames of 10 s fit, the fourth waits
        // until the first has left the hour.
        for i in 0..3 {
            assert_eq!(d.check(i as f64 * 20.0, 10.0, Some(1.0)), Ok(()));
            d.record(i as f64 * 20.0 + 10.0, 10.0);
        }
        // Used 30 s of 36 s at t = 60.
        assert_eq!(d.check(60.0, 10.0, Some(1.0)), Err(DutyRefusal::Wait(10.0 + 3600.0 - 60.0)));
        // An hour after the first ended, it fits again.
        assert_eq!(d.check(10.0 + 3600.0, 10.0, Some(1.0)), Ok(()));
        // No limit, no refusal.
        assert_eq!(d.check(60.0, 10_000.0, None), Ok(()));
        // A frame longer than the whole budget never fits.
        assert_eq!(d.check(60.0, 40.0, Some(1.0)), Err(DutyRefusal::TooLong));
    }
}
