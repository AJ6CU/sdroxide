//! Engine.IO v4 / Socket.IO v4 text-frame codec — parsing and emitting only,
//! with no I/O at all. That split is deliberate: the whole grammar is testable
//! offline, without a socket or a live server.
//!
//! Written from the public Engine.IO / Socket.IO v4 protocol specification and
//! from frames captured against `qso.freedv.org`; it is not a transcription of
//! any existing client implementation.
//!
//! Only the subset FreeDV Reporter actually uses is modelled. The server runs
//! a single default namespace, sends no binary attachments and never uses
//! acknowledgement ids, so packet types outside the table below are reported as
//! [`Frame::Ignored`] rather than given variants of their own.
//!
//! ```text
//! 0{json}          engine.io OPEN     — session parameters incl. ping timings
//! 1                engine.io CLOSE
//! 2                engine.io PING     — the peer must answer with `3`
//! 3                engine.io PONG
//! 4<packet>        engine.io MESSAGE  — carries one socket.io packet:
//!     40{json}         socket.io CONNECT / CONNECT ack (auth out, sid back)
//!     42["ev",args]    socket.io EVENT
//!     44{json}         socket.io CONNECT_ERROR
//! ```

use serde_json::Value;

/// The engine.io PONG payload, sent in reply to every [`Frame::Ping`].
pub const PONG: &str = "3";

/// One decoded inbound text frame.
#[derive(Debug, Clone, PartialEq)]
pub enum Frame {
    /// Engine.io OPEN. `ping_interval_ms + ping_timeout_ms` is how long we may
    /// go without hearing from the server before treating it as dead.
    Open { sid: String, ping_interval_ms: u64, ping_timeout_ms: u64 },
    /// Engine.io CLOSE — the server is hanging up.
    Close,
    /// Engine.io PING. The caller must answer with [`PONG`].
    Ping,
    /// Engine.io PONG. We never send `2`, so this is informational.
    Pong,
    /// Socket.io CONNECT ack. The `sid` identifies *us* in the server's roster,
    /// which is how our own station gets filtered out of the spot list.
    Connected { sid: String },
    /// Socket.io CONNECT_ERROR — the namespace refused us; reconnect.
    ConnectError,
    /// Socket.io EVENT. `args` is [`Value::Null`] when the event carried none.
    Event { name: String, args: Value },
    /// A frame we knowingly ignore (transport-upgrade traffic, socket.io
    /// packet types this client has no use for).
    Ignored,
    /// A frame that is not engine.io at all. Treat the peer as broken.
    Invalid,
}

/// Decode one inbound WebSocket text frame.
pub fn parse(text: &str) -> Frame {
    let mut chars = text.chars();
    let Some(kind) = chars.next() else {
        // An empty frame is not a valid engine.io packet.
        return Frame::Invalid;
    };
    let rest = &text[kind.len_utf8()..];
    match kind {
        '0' => match serde_json::from_str::<Value>(rest) {
            Ok(v) => Frame::Open {
                sid: v.get("sid").and_then(Value::as_str).unwrap_or_default().to_string(),
                // Absent timings would make the watchdog fire immediately, so
                // fall back to the protocol's own defaults rather than zero.
                ping_interval_ms: v.get("pingInterval").and_then(Value::as_u64).unwrap_or(25_000),
                ping_timeout_ms: v.get("pingTimeout").and_then(Value::as_u64).unwrap_or(20_000),
            },
            Err(_) => Frame::Invalid,
        },
        '1' => Frame::Close,
        '2' => Frame::Ping,
        '3' => Frame::Pong,
        '4' => parse_socketio(rest),
        // Other digits are transport-upgrade packets (`5` UPGRADE, `6` NOOP).
        d if d.is_ascii_digit() => Frame::Ignored,
        _ => Frame::Invalid,
    }
}

/// Decode the socket.io packet carried inside an engine.io MESSAGE.
fn parse_socketio(text: &str) -> Frame {
    let mut chars = text.chars();
    let Some(kind) = chars.next() else {
        return Frame::Ignored;
    };
    let rest = &text[kind.len_utf8()..];
    match kind {
        '0' => Frame::Connected {
            sid: serde_json::from_str::<Value>(rest)
                .ok()
                .and_then(|v| v.get("sid").and_then(Value::as_str).map(str::to_string))
                .unwrap_or_default(),
        },
        '2' => match serde_json::from_str::<Value>(rest) {
            // `["name"]` and `["name", args]` are both legal; anything whose
            // first element is not a string is not an event we can dispatch.
            Ok(Value::Array(mut a)) if !a.is_empty() => match a.first().and_then(Value::as_str) {
                Some(name) => {
                    let name = name.to_string();
                    let args = if a.len() > 1 { a.swap_remove(1) } else { Value::Null };
                    Frame::Event { name, args }
                }
                None => Frame::Ignored,
            },
            _ => Frame::Ignored,
        },
        '4' => Frame::ConnectError,
        // DISCONNECT, ACK, and the binary packet types go unused by this server.
        _ => Frame::Ignored,
    }
}

/// Build the socket.io CONNECT frame that opens the default namespace, carrying
/// the auth object the server authenticates and identifies us with.
pub fn connect(auth: &Value) -> String {
    format!("40{}", compact(auth))
}

/// Build a socket.io EVENT frame. Pass `None` for an event with no arguments.
pub fn emit(event: &str, args: Option<&Value>) -> String {
    let payload = match args {
        Some(a) => Value::Array(vec![Value::String(event.to_string()), a.clone()]),
        None => Value::Array(vec![Value::String(event.to_string())]),
    };
    format!("42{}", compact(&payload))
}

/// Serialise compactly, with no whitespace between tokens.
///
/// `serde_json`'s `Map` is a `BTreeMap` here (the `preserve_order` feature is
/// not enabled anywhere in the workspace), so object keys come out sorted. JSON
/// objects are unordered and the server does not care, but it does make the
/// emitted frames deterministic, which is what the tests below rely on.
fn compact(v: &Value) -> String {
    serde_json::to_string(v).unwrap_or_else(|_| "{}".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn parses_the_live_open_frame() {
        // Captured verbatim from qso.freedv.org:80.
        let f = parse(
            r#"0{"sid":"i8xoMcn7yFfTIpacX-pS","upgrades":["websocket"],"pingTimeout":5000,"pingInterval":5000,"maxPayload":1000000}"#,
        );
        assert_eq!(
            f,
            Frame::Open {
                sid: "i8xoMcn7yFfTIpacX-pS".into(),
                ping_interval_ms: 5000,
                ping_timeout_ms: 5000,
            }
        );
    }

    #[test]
    fn open_without_timings_falls_back_to_protocol_defaults() {
        let f = parse(r#"0{"sid":"x"}"#);
        assert_eq!(
            f,
            Frame::Open { sid: "x".into(), ping_interval_ms: 25_000, ping_timeout_ms: 20_000 }
        );
    }

    #[test]
    fn ping_pong_and_close() {
        assert_eq!(parse("2"), Frame::Ping);
        assert_eq!(parse("3"), Frame::Pong);
        assert_eq!(parse("1"), Frame::Close);
    }

    #[test]
    fn connect_ack_carries_our_sid() {
        assert_eq!(parse(r#"40{"sid":"abc123"}"#), Frame::Connected { sid: "abc123".into() });
        // Some servers ack with no body at all.
        assert_eq!(parse("40"), Frame::Connected { sid: String::new() });
    }

    #[test]
    fn parses_an_event_with_args() {
        let f = parse(r#"42["freq_change",{"sid":"a","freq":14236000}]"#);
        let Frame::Event { name, args } = f else { panic!("not an event") };
        assert_eq!(name, "freq_change");
        assert_eq!(args["freq"], json!(14_236_000u64));
    }

    #[test]
    fn parses_a_bare_event_with_no_args() {
        assert_eq!(
            parse(r#"42["connection_successful"]"#),
            Frame::Event { name: "connection_successful".into(), args: Value::Null }
        );
    }

    #[test]
    fn parses_bulk_update_pairs() {
        let f = parse(
            r#"42["bulk_update",[["freq_change",{"freq":1}],["tx_report",{"transmitting":true}]]]"#,
        );
        let Frame::Event { name, args } = f else { panic!("not an event") };
        assert_eq!(name, "bulk_update");
        let items = args.as_array().expect("array");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0][0], json!("freq_change"));
        assert_eq!(items[1][1]["transmitting"], json!(true));
    }

    #[test]
    fn namespace_error_is_a_disconnect() {
        assert_eq!(parse(r#"44{"message":"Not authorized"}"#), Frame::ConnectError);
    }

    #[test]
    fn unused_packet_types_are_ignored_not_fatal() {
        assert_eq!(parse("41"), Frame::Ignored); // socket.io DISCONNECT
        assert_eq!(parse("43[]"), Frame::Ignored); // socket.io ACK
        assert_eq!(parse("6"), Frame::Ignored); // engine.io NOOP
        assert_eq!(parse("4"), Frame::Ignored); // MESSAGE with no packet
        assert_eq!(parse(r#"42[{"not":"a name"}]"#), Frame::Ignored);
        assert_eq!(parse("42[]"), Frame::Ignored);
        assert_eq!(parse("42not-json"), Frame::Ignored);
    }

    #[test]
    fn garbage_is_invalid() {
        assert_eq!(parse(""), Frame::Invalid);
        assert_eq!(parse("x"), Frame::Invalid);
        assert_eq!(parse("hello"), Frame::Invalid);
        assert_eq!(parse("0not-json"), Frame::Invalid);
    }

    #[test]
    fn multibyte_lead_does_not_panic() {
        // Byte-slicing the tail would panic here if we used `text[1..]`.
        assert_eq!(parse("€vent"), Frame::Invalid);
    }

    #[test]
    fn emit_is_compact_json() {
        assert_eq!(
            emit("freq_change", Some(&json!({ "freq": 14_236_000u64 }))),
            r#"42["freq_change",{"freq":14236000}]"#
        );
        assert_eq!(emit("hide_self", None), r#"42["hide_self"]"#);
        assert_eq!(
            emit("rx_report", Some(&json!({ "callsign": "K1ABC", "mode": "RADEV1", "snr": -3 }))),
            r#"42["rx_report",{"callsign":"K1ABC","mode":"RADEV1","snr":-3}]"#
        );
    }

    #[test]
    fn connect_frame_carries_the_auth_object() {
        // Keys come out sorted (see `compact`), not in `json!` order.
        assert_eq!(
            connect(&json!({ "role": "view", "protocol_version": 2 })),
            r#"40{"protocol_version":2,"role":"view"}"#
        );
    }

    #[test]
    fn emitted_frames_round_trip_through_parse() {
        let out = emit("tx_report", Some(&json!({ "mode": "RADEV1", "transmitting": false })));
        assert_eq!(
            parse(&out),
            Frame::Event {
                name: "tx_report".into(),
                args: json!({ "mode": "RADEV1", "transmitting": false }),
            }
        );
    }
}
