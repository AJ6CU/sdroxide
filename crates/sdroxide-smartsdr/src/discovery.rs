//! FlexRadio discovery: radios broadcast an ASCII key/value packet to UDP 4992
//! roughly once a second, forever, whether or not anybody is listening.
//!
//! There is no probe to send — [`discover`] simply binds the port and collects
//! what arrives, which is why it costs a full listen window rather than a round
//! trip.

use std::collections::BTreeMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

use crate::protocol::{RADIO_PORT, parse_kvs};

/// How long a default scan listens. Radios broadcast about once a second, so
/// this catches two or three announcements from each — enough that one dropped
/// packet doesn't hide a radio.
pub const DEFAULT_SCAN: Duration = Duration::from_millis(2500);

/// A radio seen on the network.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FlexRadioInfo {
    pub ip: String,
    pub port: u16,
    /// Model, e.g. `FLEX-6600`.
    pub model: String,
    pub serial: String,
    /// Operator-set radio name.
    pub nickname: String,
    pub callsign: String,
    /// Firmware version, e.g. `3.3.28.0`.
    pub version: String,
    /// `Available`, `In_Use`, …
    pub status: String,
    /// Whether multiFLEX (more than one GUI client at a time) is enabled. When
    /// it is off and somebody else is already connected, our GUI registration
    /// will be refused.
    pub multiflex: bool,
    /// Station names of GUI clients already connected.
    pub gui_clients: Vec<String>,
    /// Every key the broadcast carried, including ones this struct doesn't
    /// model. Kept so a diagnostic dump shows what the radio actually said —
    /// firmware adds fields faster than clients learn them.
    pub raw: BTreeMap<String, String>,
}

impl FlexRadioInfo {
    /// One-line label for the selection UI.
    pub fn label(&self) -> String {
        let name = if !self.nickname.is_empty() {
            self.nickname.clone()
        } else if !self.model.is_empty() {
            self.model.clone()
        } else {
            "FlexRadio".to_string()
        };
        let mut s = format!("{name}  {}", self.ip);
        if !self.model.is_empty() && self.model != name {
            s = format!("{name} ({})  {}", self.model, self.ip);
        }
        if !self.version.is_empty() {
            s.push_str(&format!("  v{}", self.version));
        }
        if !self.gui_clients.is_empty() {
            s.push_str(&format!("  [in use: {}]", self.gui_clients.join(", ")));
            if !self.multiflex {
                s.push_str(" — multiFLEX off");
            }
        }
        s
    }

    /// Whether connecting as a GUI client should succeed: nobody else has the
    /// radio, or multiFLEX allows sharing it.
    pub fn joinable(&self) -> bool {
        self.gui_clients.is_empty() || self.multiflex
    }
}

/// Parse a discovery broadcast payload.
///
/// Values are cleaned of control characters, and `0x7F` is decoded back to a
/// space in the list fields — the radio substitutes it there because the packet
/// itself is space-separated, so a station named "Shack Two" would otherwise
/// split into two tokens.
pub fn parse_broadcast(data: &[u8]) -> FlexRadioInfo {
    let text = String::from_utf8_lossy(data);
    let kvs = parse_kvs(text.trim());

    let clean = |s: &str| -> String {
        s.chars().filter(|c| !c.is_control() && *c != '\u{7f}').collect::<String>().trim().into()
    };
    let clean_list = |s: &str| -> Vec<String> {
        s.replace('\u{7f}', " ").split(',').map(&clean).filter(|t| !t.is_empty()).collect()
    };
    let get = |k: &str| kvs.get(k).map(|v| clean(v)).unwrap_or_default();

    FlexRadioInfo {
        ip: get("ip"),
        port: kvs.get("port").and_then(|p| p.trim().parse().ok()).unwrap_or(RADIO_PORT),
        model: get("model"),
        serial: get("serial"),
        nickname: get("nickname"),
        callsign: get("callsign"),
        version: get("version"),
        status: get("status"),
        // Absent means enabled: firmware old enough not to send the key predates
        // the single-client restriction being optional.
        multiflex: kvs.get("mf_enable").map(|v| v.trim() != "0").unwrap_or(true),
        gui_clients: kvs.get("gui_client_stations").map(|v| clean_list(v)).unwrap_or_default(),
        raw: kvs,
    }
}

/// Listen for discovery broadcasts for `window` and return the radios seen,
/// deduplicated by serial (falling back to IP for a radio that omits one).
///
/// Binding 4992 is deliberately shared: SmartSDR itself, or another sdroxide
/// window, may already hold it, and a scan that fails because a second listener
/// exists would be useless on exactly the machines most likely to run one.
pub fn discover(window: Duration) -> std::io::Result<Vec<FlexRadioInfo>> {
    let sock = bind_shared(RADIO_PORT)?;
    sock.set_read_timeout(Some(Duration::from_millis(250)))?;

    let mut found: Vec<FlexRadioInfo> = Vec::new();
    let deadline = Instant::now() + window;
    let mut buf = [0u8; 2048];

    while Instant::now() < deadline {
        let (n, from) = match sock.recv_from(&mut buf) {
            Ok(v) => v,
            Err(e)
                if matches!(
                    e.kind(),
                    std::io::ErrorKind::WouldBlock | std::io::ErrorKind::TimedOut
                ) =>
            {
                continue;
            }
            Err(e) => return Err(e),
        };
        let mut info = parse_broadcast(&buf[..n]);
        // Trust the sender address over the advertised one: a radio behind a
        // NAT or on a second interface can announce an IP we cannot reach.
        if info.ip.is_empty() {
            info.ip = from.ip().to_string();
        }
        let key = |r: &FlexRadioInfo| {
            if r.serial.is_empty() { r.ip.clone() } else { r.serial.clone() }
        };
        let k = key(&info);
        tracing::debug!(ip = %info.ip, model = %info.model, serial = %info.serial, "SmartSDR discovery packet");
        match found.iter_mut().find(|r| key(r) == k) {
            Some(existing) => *existing = info,
            None => found.push(info),
        }
    }

    found.sort_by(|a, b| a.ip.cmp(&b.ip));
    Ok(found)
}

/// A default-length scan.
pub fn discover_default() -> Vec<FlexRadioInfo> {
    match discover(DEFAULT_SCAN) {
        Ok(v) => v,
        Err(e) => {
            tracing::warn!("SmartSDR discovery failed: {e}");
            Vec::new()
        }
    }
}

/// Bind a UDP port with address reuse, so a scan works alongside SmartSDR — or
/// a second sdroxide window — already listening on it.
///
/// This is why the crate takes `socket2`: the reuse flags have to be set
/// *before* the bind, and `std::net::UdpSocket` gives no window in which to do
/// that. On the machines most likely to be running a Flex client already, a
/// scan that failed with "address in use" would be useless.
pub(crate) fn bind_shared(port: u16) -> std::io::Result<UdpSocket> {
    use socket2::{Domain, Protocol, Socket, Type};

    let sock = Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP))?;
    sock.set_reuse_address(true)?;
    // Not portable (absent on Windows), and not required there — Windows'
    // SO_REUSEADDR already has the semantics the other platforms need
    // SO_REUSEPORT for.
    #[cfg(all(
        unix,
        not(any(
            target_os = "solaris",
            target_os = "illumos",
            target_os = "cygwin",
            target_os = "nuttx",
        ))
    ))]
    sock.set_reuse_port(true)?;
    sock.set_broadcast(true)?;
    sock.bind(&SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port).into())?;
    Ok(sock.into())
}

/// Send one discovery broadcast — the simulator's side of [`discover`].
pub fn send_broadcast(sock: &UdpSocket, payload: &str, port: u16) -> std::io::Result<usize> {
    sock.send_to(payload.as_bytes(), SocketAddr::new(IpAddr::V4(Ipv4Addr::BROADCAST), port))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "discovery_protocol_version=3.0.0.1 model=FLEX-6600 \
         serial=1234-5678-6600-9999 version=3.3.28.0 nickname=Shack \
         callsign=OE1XYZ ip=192.168.1.50 port=4992 status=Available \
         max_licensed_version=v3 mf_enable=1";

    #[test]
    fn parses_a_discovery_broadcast() {
        let info = parse_broadcast(SAMPLE.as_bytes());
        assert_eq!(info.model, "FLEX-6600");
        assert_eq!(info.ip, "192.168.1.50");
        assert_eq!(info.port, 4992);
        assert_eq!(info.version, "3.3.28.0");
        assert_eq!(info.nickname, "Shack");
        assert!(info.multiflex);
        assert!(info.joinable());
        // Unmodelled keys still reach a diagnostic dump.
        assert_eq!(info.raw["max_licensed_version"], "v3");
    }

    #[test]
    fn station_names_decode_del_as_space() {
        // The packet is space-separated, so the radio substitutes 0x7F inside
        // list values; a station called "Shack Two" arrives as "Shack\x7fTwo".
        let pkt = format!("{SAMPLE} gui_client_stations=Shack\u{7f}Two,Mobile");
        let info = parse_broadcast(pkt.as_bytes());
        assert_eq!(info.gui_clients, vec!["Shack Two", "Mobile"]);
        assert!(info.joinable()); // multiFLEX is on
    }

    #[test]
    fn an_occupied_radio_without_multiflex_is_not_joinable() {
        let pkt = SAMPLE.replace("mf_enable=1", "mf_enable=0") + " gui_client_stations=SomeoneElse";
        let info = parse_broadcast(pkt.as_bytes());
        assert!(!info.multiflex);
        assert!(!info.joinable());
        assert!(info.label().contains("multiFLEX off"));
    }

    #[test]
    fn firmware_without_mf_enable_is_assumed_shareable() {
        let pkt = SAMPLE.replace(" mf_enable=1", "");
        assert!(parse_broadcast(pkt.as_bytes()).multiflex);
    }

    #[test]
    fn garbage_does_not_panic() {
        let info = parse_broadcast(&[0xFF, 0x00, 0x41, 0x3D]);
        assert_eq!(info.port, RADIO_PORT);
    }
}
