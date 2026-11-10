//! MIDI wire format: raw bytes ↔ [`MidiMsg`] + a 7-bit value.
//!
//! Pure functions with no device access, so the parts most likely to be wrong
//! are the parts that can be unit-tested.

use sdroxide_types::{MidiMsg, MidiMsgKind};

/// One control movement decoded off the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MidiControl {
    pub msg: MidiMsg,
    /// 7-bit value: CC value, note velocity, program number, or the high 7 bits
    /// of a pitch bend.
    pub value: u8,
    /// True for a note-off, so a button release is distinguishable from a
    /// press at velocity 0 without inspecting the status byte again.
    pub off: bool,
}

/// Parse one MIDI message. Returns `None` for anything that cannot carry a
/// binding: system-exclusive, realtime clock, aftertouch, malformed runts.
pub fn parse(bytes: &[u8]) -> Option<MidiControl> {
    let status = *bytes.first()?;
    // System messages (0xF0..0xFF) carry no channel and never bind.
    if !(0x80..0xF0).contains(&status) {
        return None;
    }
    let channel = status & 0x0F;
    let kind = status & 0xF0;
    let d1 = bytes.get(1).copied().unwrap_or(0) & 0x7F;
    let d2 = bytes.get(2).copied().unwrap_or(0) & 0x7F;
    let msg = |k: MidiMsgKind, data1: u8| MidiMsg { kind: k, channel, data1 };
    Some(match kind {
        0x80 => MidiControl { msg: msg(MidiMsgKind::Note, d1), value: 0, off: true },
        // A note-on at velocity 0 is the conventional note-off; controllers
        // that use it would otherwise leave a button latched down forever.
        0x90 => MidiControl { msg: msg(MidiMsgKind::Note, d1), value: d2, off: d2 == 0 },
        0xB0 => MidiControl { msg: msg(MidiMsgKind::Cc, d1), value: d2, off: false },
        0xC0 => MidiControl { msg: msg(MidiMsgKind::ProgramChange, d1), value: d1, off: false },
        0xE0 => {
            // 14-bit little-endian; bindings only need the coarse position.
            let bend = ((d2 as u16) << 7) | d1 as u16;
            MidiControl {
                msg: msg(MidiMsgKind::PitchBend, 0),
                value: (bend >> 7) as u8,
                off: false,
            }
        }
        // Polyphonic/channel aftertouch: real controls, but nothing binds them.
        _ => return None,
    })
}

/// Encode a value to send back to a controller, lighting an LED or driving a
/// motor fader. Omni bindings have no channel of their own, so they address
/// channel 1.
pub fn encode(msg: &MidiMsg, value: u8) -> Vec<u8> {
    let ch = if msg.channel == MidiMsg::ANY_CHANNEL { 0 } else { msg.channel & 0x0F };
    let v = value & 0x7F;
    match msg.kind {
        MidiMsgKind::Note => vec![0x90 | ch, msg.data1 & 0x7F, v],
        MidiMsgKind::Cc => vec![0xB0 | ch, msg.data1 & 0x7F, v],
        MidiMsgKind::ProgramChange => vec![0xC0 | ch, v],
        MidiMsgKind::PitchBend => {
            let bend = (v as u16) << 7;
            vec![0xE0 | ch, (bend & 0x7F) as u8, (bend >> 7) as u8]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_note_on_and_off() {
        let on = parse(&[0x90, 60, 127]).unwrap();
        assert_eq!(on.msg.kind, MidiMsgKind::Note);
        assert_eq!(on.msg.channel, 0);
        assert_eq!(on.msg.data1, 60);
        assert_eq!(on.value, 127);
        assert!(!on.off);

        let off = parse(&[0x82, 60, 0]).unwrap();
        assert!(off.off);
        assert_eq!(off.msg.channel, 2);
    }

    /// Controllers that never send 0x8n rely on this; without it a button
    /// bound to PTT would key the rig and never unkey it.
    #[test]
    fn note_on_at_zero_velocity_is_a_release() {
        assert!(parse(&[0x90, 60, 0]).unwrap().off);
    }

    #[test]
    fn parses_control_change() {
        let cc = parse(&[0xB0, 16, 65]).unwrap();
        assert_eq!(cc.msg.kind, MidiMsgKind::Cc);
        assert_eq!(cc.msg.data1, 16);
        assert_eq!(cc.value, 65);
    }

    #[test]
    fn parses_pitch_bend_to_its_coarse_position() {
        // Centre (8192) is the middle of the 0..127 coarse range.
        let pb = parse(&[0xE0, 0x00, 0x40]).unwrap();
        assert_eq!(pb.msg.kind, MidiMsgKind::PitchBend);
        assert_eq!(pb.value, 64);
        assert_eq!(parse(&[0xE0, 0x7F, 0x7F]).unwrap().value, 127);
        assert_eq!(parse(&[0xE0, 0x00, 0x00]).unwrap().value, 0);
    }

    #[test]
    fn ignores_system_and_aftertouch_traffic() {
        assert!(parse(&[0xF8]).is_none(), "MIDI clock must not bind");
        assert!(parse(&[0xF0, 0x7E, 0xF7]).is_none(), "sysex must not bind");
        assert!(parse(&[0xA0, 60, 40]).is_none(), "poly aftertouch");
        assert!(parse(&[0xD0, 40]).is_none(), "channel aftertouch");
        assert!(parse(&[]).is_none());
        assert!(parse(&[0x40, 1, 2]).is_none(), "data byte as status");
    }

    #[test]
    fn short_messages_do_not_panic() {
        assert!(parse(&[0xB0]).is_some());
        assert!(parse(&[0x90, 60]).is_some());
    }

    #[test]
    fn encode_round_trips_through_parse() {
        for msg in [
            MidiMsg { kind: MidiMsgKind::Cc, channel: 3, data1: 16 },
            MidiMsg { kind: MidiMsgKind::Note, channel: 0, data1: 60 },
        ] {
            let got = parse(&encode(&msg, 99)).unwrap();
            assert_eq!(got.msg, msg);
            assert_eq!(got.value, 99);
        }
    }

    #[test]
    fn omni_bindings_address_channel_one() {
        let omni = MidiMsg { kind: MidiMsgKind::Cc, channel: MidiMsg::ANY_CHANNEL, data1: 7 };
        assert_eq!(encode(&omni, 64), vec![0xB0, 7, 64]);
    }
}
