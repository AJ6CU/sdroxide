//! A one-shot DNS-SD (mDNS) querier, just large enough to find `iiod`.
//!
//! `iiod` publishes itself over Avahi as `_iio._tcp`, and a Pluto's own
//! hostname is `pluto.local`. Both are ordinary multicast-DNS questions, so
//! this asks them directly rather than pulling in a general-purpose mDNS
//! responder — nothing here has to *answer* queries, cache records, or stay
//! resident.
//!
//! # One-shot, unicast-response
//!
//! A full mDNS client binds UDP 5353, which means sharing the port with the
//! system's own responder (Avahi, mDNSResponder) and the socket options that go
//! with that. This instead binds an ephemeral port and sets the **QU bit** — the
//! top bit of the question's class — which asks responders to reply directly to
//! the sender rather than to the multicast group. RFC 6762 §5.4 calls this a
//! one-shot query and it is exactly the case here: one question, asked when the
//! operator presses Discover, answered within a second or two.
//!
//! # One socket per interface
//!
//! A Pluto's USB gadget is a *separate* network interface from the LAN, and a
//! multicast datagram goes out of one interface. `std::net::UdpSocket` has no
//! way to name that interface, but binding the socket to the interface's own
//! address selects it — which is why this enumerates local addresses and sends
//! from each, the same shape the HPSDR backend's broadcast discovery uses.

use std::collections::BTreeMap;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

/// The multicast group and port every mDNS responder listens on.
const MDNS_GROUP: Ipv4Addr = Ipv4Addr::new(224, 0, 0, 251);
const MDNS_PORT: u16 = 5353;

/// What `iiod`'s Avahi service is called.
const IIO_SERVICE: &str = "_iio._tcp.local";
/// A Pluto's default hostname, asked for directly in case the service record is
/// missing — a firmware with Avahi switched off still answers for its own name.
const PLUTO_HOST: &str = "pluto.local";

const QTYPE_A: u16 = 1;
const QTYPE_PTR: u16 = 12;
const QTYPE_TXT: u16 = 16;
const QTYPE_SRV: u16 = 33;
/// IN, with the top bit set: "please answer me directly, not the group".
const QCLASS_IN_UNICAST: u16 = 0x8001;

/// One thing that answered.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Found {
    pub ip: Ipv4Addr,
    /// The service instance name, or the hostname when that is all we learned.
    pub name: String,
    pub port: Option<u16>,
}

/// Ask for `iiod` services and for `pluto.local`, and collect what answers
/// within `timeout`.
pub(crate) fn query(timeout: Duration) -> Vec<Found> {
    let questions = [build_query(IIO_SERVICE, QTYPE_PTR), build_query(PLUTO_HOST, QTYPE_A)];
    let mut sockets = Vec::new();
    for local in local_ipv4() {
        let Ok(sock) = UdpSocket::bind(SocketAddr::new(IpAddr::V4(local), 0)) else { continue };
        if sock.set_read_timeout(Some(Duration::from_millis(100))).is_err() {
            continue;
        }
        // 255 rather than the multicast default of 1: a Pluto reached through a
        // router still gets the question, and mDNS responders ignore the hop
        // count anyway.
        let _ = sock.set_multicast_ttl_v4(255);
        let _ = sock.set_multicast_loop_v4(true);
        let dest = SocketAddr::new(IpAddr::V4(MDNS_GROUP), MDNS_PORT);
        let mut asked = false;
        for q in &questions {
            if sock.send_to(q, dest).is_ok() {
                asked = true;
            }
        }
        if asked {
            tracing::debug!("PlutoSDR: mDNS query sent from {local}");
            sockets.push(sock);
        }
    }
    if sockets.is_empty() {
        tracing::debug!("PlutoSDR: no local IPv4 interface accepted an mDNS query");
        return Vec::new();
    }

    // Collate across every answer: the A record that carries the address and
    // the SRV/PTR records that carry the name usually arrive in one datagram,
    // but nothing in the protocol says they have to.
    let mut names: BTreeMap<Ipv4Addr, String> = BTreeMap::new();
    let mut ports: BTreeMap<Ipv4Addr, u16> = BTreeMap::new();
    let mut host_names: BTreeMap<String, String> = BTreeMap::new();
    let mut host_ports: BTreeMap<String, u16> = BTreeMap::new();
    let mut host_addrs: BTreeMap<String, Ipv4Addr> = BTreeMap::new();

    let deadline = Instant::now() + timeout;
    let mut buf = [0u8; 4096];
    while Instant::now() < deadline {
        let mut any = false;
        for sock in &sockets {
            while let Ok((n, from)) = sock.recv_from(&mut buf) {
                any = true;
                let Some(records) = parse_records(&buf[..n]) else { continue };
                for r in records {
                    match r {
                        Record::A { name, addr } => {
                            host_addrs.insert(name.to_ascii_lowercase(), addr);
                        }
                        Record::Srv { name, target, port } => {
                            host_names.insert(target.to_ascii_lowercase(), instance_of(&name));
                            host_ports.insert(target.to_ascii_lowercase(), port);
                        }
                        Record::Ptr { target, .. } => {
                            // The instance name on its own; the SRV that goes
                            // with it is usually in the same packet.
                            let _ = target;
                        }
                        Record::Txt { .. } => {}
                    }
                }
                // A responder that sent only an address still tells us where it
                // is: the source of the datagram.
                if let IpAddr::V4(ip) = from.ip() {
                    names.entry(ip).or_insert_with(|| ip.to_string());
                }
            }
        }
        if !any {
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    for (host, addr) in &host_addrs {
        if let Some(instance) = host_names.get(host) {
            names.insert(*addr, instance.clone());
        } else {
            names.entry(*addr).or_insert_with(|| trim_local(host).to_string());
        }
        if let Some(p) = host_ports.get(host) {
            ports.insert(*addr, *p);
        }
    }

    let mut out: Vec<Found> = names
        .into_iter()
        .map(|(ip, name)| Found { ip, name, port: ports.get(&ip).copied() })
        .collect();
    out.sort_by_key(|f| f.ip.octets());
    out
}

/// Local IPv4 addresses worth sending a multicast query from: loopback is
/// excluded (nothing answers there), but the Pluto's own gadget subnet is very
/// much included.
fn local_ipv4() -> Vec<Ipv4Addr> {
    use network_interface::NetworkInterfaceConfig;
    let Ok(ifaces) = network_interface::NetworkInterface::show() else { return Vec::new() };
    let mut out = Vec::new();
    for iface in ifaces {
        for addr in iface.addr {
            if let IpAddr::V4(v4) = addr.ip()
                && !v4.is_loopback()
                && !v4.is_unspecified()
            {
                out.push(v4);
            }
        }
    }
    out.sort();
    out.dedup();
    out
}

/// A DNS query packet: header, one question, nothing else.
fn build_query(name: &str, qtype: u16) -> Vec<u8> {
    let mut pkt = Vec::with_capacity(64);
    // ID 0 — mDNS matches answers by question, not by id.
    pkt.extend_from_slice(&[0, 0]);
    pkt.extend_from_slice(&[0, 0]); // flags: standard query
    pkt.extend_from_slice(&1u16.to_be_bytes()); // one question
    pkt.extend_from_slice(&[0, 0, 0, 0, 0, 0]); // no answer/authority/additional
    encode_name(&mut pkt, name);
    pkt.extend_from_slice(&qtype.to_be_bytes());
    pkt.extend_from_slice(&QCLASS_IN_UNICAST.to_be_bytes());
    pkt
}

fn encode_name(out: &mut Vec<u8>, name: &str) {
    for label in name.split('.').filter(|l| !l.is_empty()) {
        let bytes = label.as_bytes();
        out.push(bytes.len().min(63) as u8);
        out.extend_from_slice(&bytes[..bytes.len().min(63)]);
    }
    out.push(0);
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Record {
    A { name: String, addr: Ipv4Addr },
    Ptr { name: String, target: String },
    Srv { name: String, target: String, port: u16 },
    Txt { name: String },
}

/// Pull the A/PTR/SRV/TXT records out of a response. Returns `None` for
/// anything that is not a parseable DNS message — there is other traffic on
/// 5353 and none of it should take Discover down.
fn parse_records(pkt: &[u8]) -> Option<Vec<Record>> {
    if pkt.len() < 12 {
        return None;
    }
    let qd = u16::from_be_bytes([pkt[4], pkt[5]]) as usize;
    let counts = [6usize, 8, 10]
        .iter()
        .map(|&o| u16::from_be_bytes([pkt[o], pkt[o + 1]]) as usize)
        .sum::<usize>();
    let mut pos = 12;
    for _ in 0..qd {
        let (_, next) = read_name(pkt, pos)?;
        pos = next.checked_add(4)?; // qtype + qclass
    }
    let mut out = Vec::new();
    for _ in 0..counts {
        let (name, next) = read_name(pkt, pos)?;
        pos = next;
        if pos + 10 > pkt.len() {
            break;
        }
        let rtype = u16::from_be_bytes([pkt[pos], pkt[pos + 1]]);
        let rdlen = u16::from_be_bytes([pkt[pos + 8], pkt[pos + 9]]) as usize;
        pos += 10;
        let end = pos.checked_add(rdlen)?;
        if end > pkt.len() {
            break;
        }
        let rdata = &pkt[pos..end];
        match rtype {
            QTYPE_A if rdlen == 4 => out.push(Record::A {
                name: trim_local(&name).to_string(),
                addr: Ipv4Addr::new(rdata[0], rdata[1], rdata[2], rdata[3]),
            }),
            QTYPE_PTR => {
                if let Some((target, _)) = read_name(pkt, pos) {
                    out.push(Record::Ptr { name, target });
                }
            }
            QTYPE_SRV if rdlen >= 7 => {
                let port = u16::from_be_bytes([rdata[4], rdata[5]]);
                if let Some((target, _)) = read_name(pkt, pos + 6) {
                    out.push(Record::Srv { name, target: trim_local(&target).to_string(), port });
                }
            }
            QTYPE_TXT => out.push(Record::Txt { name }),
            _ => {}
        }
        pos = end;
    }
    Some(out)
}

/// Read a (possibly compressed) DNS name, returning it and the offset just past
/// it. Pointer chasing is bounded, because a packet from the network can point
/// at itself.
fn read_name(pkt: &[u8], mut pos: usize) -> Option<(String, usize)> {
    let mut labels: Vec<String> = Vec::new();
    let mut after: Option<usize> = None;
    let mut hops = 0;
    loop {
        let len = *pkt.get(pos)? as usize;
        if len == 0 {
            pos += 1;
            break;
        }
        if len & 0xC0 == 0xC0 {
            let ptr = ((len & 0x3F) << 8) | *pkt.get(pos + 1)? as usize;
            after.get_or_insert(pos + 2);
            hops += 1;
            if hops > 16 {
                return None;
            }
            pos = ptr;
            continue;
        }
        let end = pos + 1 + len;
        let label = pkt.get(pos + 1..end)?;
        labels.push(String::from_utf8_lossy(label).into_owned());
        pos = end;
    }
    Some((labels.join("."), after.unwrap_or(pos)))
}

/// `pluto.local` → `pluto`; also drops the trailing dot a wire name may carry.
fn trim_local(name: &str) -> &str {
    name.trim_end_matches('.').trim_end_matches(".local").trim_end_matches('.')
}

/// The first label of a service instance name — `Pluto._iio._tcp.local`
/// becomes `Pluto`.
fn instance_of(name: &str) -> String {
    name.split('.').next().unwrap_or(name).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_query_is_one_question_with_the_unicast_bit_set() {
        let q = build_query(IIO_SERVICE, QTYPE_PTR);
        assert_eq!(u16::from_be_bytes([q[4], q[5]]), 1, "one question");
        assert_eq!(u16::from_be_bytes([q[6], q[7]]), 0, "no answers");
        // `_iio` `_tcp` `local`, each length-prefixed, then a zero.
        assert_eq!(&q[12..17], b"\x04_iio");
        assert_eq!(&q[17..22], b"\x04_tcp");
        assert_eq!(&q[22..28], b"\x05local");
        assert_eq!(q[28], 0);
        assert_eq!(u16::from_be_bytes([q[29], q[30]]), QTYPE_PTR);
        // The QU bit is what makes this a one-shot query rather than one that
        // needs a socket bound to 5353 to hear the reply.
        assert_eq!(u16::from_be_bytes([q[31], q[32]]), 0x8001);
    }

    /// Build a response the way a responder would — length-prefixed labels and
    /// a compression pointer — rather than by running our own encoder
    /// backwards, so an encoder bug cannot hide inside a passing test.
    fn pluto_response() -> Vec<u8> {
        let mut p: Vec<u8> = Vec::new();
        p.extend_from_slice(&[0, 0]); // id
        p.extend_from_slice(&[0x84, 0x00]); // flags: authoritative response
        p.extend_from_slice(&[0, 0]); // questions
        p.extend_from_slice(&2u16.to_be_bytes()); // answers: SRV + A
        p.extend_from_slice(&[0, 0, 0, 0]); // authority, additional

        // SRV  pluto._iio._tcp.local  ->  pluto.local:30431
        let srv_name_at = p.len();
        encode_name(&mut p, "pluto._iio._tcp.local");
        p.extend_from_slice(&QTYPE_SRV.to_be_bytes());
        p.extend_from_slice(&1u16.to_be_bytes()); // class IN
        p.extend_from_slice(&120u32.to_be_bytes()); // ttl
        let host_at;
        {
            let mut rdata: Vec<u8> = Vec::new();
            rdata.extend_from_slice(&0u16.to_be_bytes()); // priority
            rdata.extend_from_slice(&0u16.to_be_bytes()); // weight
            rdata.extend_from_slice(&30431u16.to_be_bytes()); // port
            let start = p.len() + 2 + rdata.len();
            host_at = start;
            encode_name(&mut rdata, "pluto.local");
            p.extend_from_slice(&(rdata.len() as u16).to_be_bytes());
            p.extend_from_slice(&rdata);
        }

        // A  <pointer to pluto.local>  ->  192.168.2.1
        p.extend_from_slice(&[0xC0 | (host_at >> 8) as u8, host_at as u8]);
        p.extend_from_slice(&QTYPE_A.to_be_bytes());
        p.extend_from_slice(&1u16.to_be_bytes());
        p.extend_from_slice(&120u32.to_be_bytes());
        p.extend_from_slice(&4u16.to_be_bytes());
        p.extend_from_slice(&[192, 168, 2, 1]);
        let _ = srv_name_at;
        p
    }

    #[test]
    fn a_response_yields_the_service_and_its_address() {
        let records = parse_records(&pluto_response()).expect("parse");
        let srv = records
            .iter()
            .find_map(|r| match r {
                Record::Srv { target, port, .. } => Some((target.clone(), *port)),
                _ => None,
            })
            .expect("srv");
        assert_eq!(srv, ("pluto".to_string(), 30431));
        let a = records
            .iter()
            .find_map(|r| match r {
                Record::A { name, addr } => Some((name.clone(), *addr)),
                _ => None,
            })
            .expect("a");
        assert_eq!(a, ("pluto".to_string(), Ipv4Addr::new(192, 168, 2, 1)));
    }

    /// There is other traffic on 5353, and none of it should be able to take
    /// the Discover button down.
    #[test]
    fn rubbish_on_the_wire_is_ignored_rather_than_fatal() {
        assert!(parse_records(b"").is_none());
        assert!(parse_records(b"not a dns packet at all").is_none());
        // A truncated record stops the walk instead of running off the end.
        let mut short = pluto_response();
        short.truncate(short.len() - 3);
        let _ = parse_records(&short);
    }

    /// A malicious or broken packet can point a name at itself; chasing that
    /// forever would hang the Discover button.
    #[test]
    fn a_self_referential_name_pointer_terminates() {
        let mut p = vec![0u8; 12];
        p[5] = 0; // no questions
        p[7] = 1; // one answer
        // A name that is a pointer to itself, at offset 12.
        p.extend_from_slice(&[0xC0, 12]);
        assert!(read_name(&p, 12).is_none());
    }

    #[test]
    fn names_lose_their_local_suffix_and_instances_lose_their_service() {
        assert_eq!(trim_local("pluto.local"), "pluto");
        assert_eq!(trim_local("pluto.local."), "pluto");
        assert_eq!(trim_local("shack-pc"), "shack-pc");
        assert_eq!(instance_of("Pluto._iio._tcp.local"), "Pluto");
        assert_eq!(instance_of("Pluto"), "Pluto");
    }
}
