//! One card per aircraft, merged from every receiver that heard it.
//!
//! ADS-B, VDL2, HFDL and the ACARS audio decoder each report a different slice
//! of the same aeroplane. The registry keys them by the 24-bit ICAO address and
//! keeps, for every field, where it came from and when. UI side only: nothing
//! here touches the wire, and nothing here reaches the network.
//!
//! The replacement rule is deliberately short:
//! - a field is replaced when the new reading is more recent;
//! - at the same age, the higher-priority source wins;
//! - so HFDL, the lowest priority, only ever fills a field nothing as recent
//!   has set.
//!
//! The decoders re-send whole tables, so every ingest is idempotent: hearing the
//! same message twice changes nothing.
//!
//! Read by the AIRCRAFT window's card list and by the map's hover card
//! ([`super::aircraft_hover`]), which only looks a card up by its address and
//! never feeds the registry.

use std::collections::HashMap;

use sdroxide_types::{
    AcarsMessage, AdsbAircraft, HfdlDecode, Vdl2AddrKind, Vdl2Message, Vdl2Payload,
};

/// Seconds without any new datum before a card is dropped.
pub(in crate::app) const CARD_TTL_S: i64 = 600;
/// Most cards kept, ICAO and fallback tables each. The oldest go first.
pub(in crate::app) const MAX_CARDS: usize = 5000;

/// Where a reading came from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::app) enum Source {
    Adsb,
    Vdl2,
    Hfdl,
    AcarsAudio,
}

/// A reading with its origin and its time (Unix seconds).
#[derive(Debug, Clone, PartialEq)]
pub(in crate::app) struct Stamped<T> {
    pub value: T,
    pub source: Source,
    pub at: i64,
}

/// What is known about one aircraft.
#[derive(Debug, Clone, Default, PartialEq)]
pub(in crate::app) struct AircraftCard {
    pub position: Option<Stamped<(f64, f64)>>,
    pub altitude_ft: Option<Stamped<i32>>,
    pub ground_speed_kt: Option<Stamped<f32>>,
    pub track_deg: Option<Stamped<f32>>,
    /// Flight number or callsign.
    pub flight: Option<Stamped<String>>,
    pub registration: Option<Stamped<String>>,
    /// The last ACARS message, as its one-line summary.
    pub last_message: Option<Stamped<String>>,
    // What the aircraft says about its own flight, from its ADS-B messages;
    // each is absent until one has been heard.
    pub selected_altitude_ft: Option<Stamped<i32>>,
    pub selected_heading_deg: Option<Stamped<f32>>,
    pub qnh_hpa: Option<Stamped<f32>>,
    /// Indicated and true airspeed, knots.
    pub airspeed: Option<Stamped<(Option<f32>, Option<f32>)>>,
    pub heading_deg: Option<Stamped<f32>>,
    /// The `NAV_*` bits of the autopilot modes engaged.
    pub nav_modes: Option<Stamped<u8>>,
    pub adsb_version: Option<Stamped<u8>>,
    /// Position accuracy, velocity accuracy and source integrity categories.
    pub accuracy: Option<Stamped<(Option<u8>, Option<u8>, Option<u8>)>>,
    /// Latest datum of any kind; what expiry looks at.
    pub last_seen: i64,
}

/// The identity of an HFDL aircraft that never gave its ICAO address.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(in crate::app) enum FallbackKey {
    AircraftId(u32),
    Flight(String),
}

#[derive(Debug, Default)]
pub(in crate::app) struct AircraftRegistry {
    cards: HashMap<u32, AircraftCard>,
    /// Registration to ICAO address, learned from VDL2 only.
    by_reg: HashMap<String, u32>,
    /// HFDL aircraft with no ICAO address. Never merged into `cards`.
    fallback: HashMap<FallbackKey, AircraftCard>,
}

/// Source priority at equal age. Position and motion trust ADS-B first; the
/// identity of the flight trusts VDL2 first (it carries the registration and
/// the airline's own flight number).
#[derive(Clone, Copy)]
enum Field {
    Motion,
    Identity,
}

fn priority(field: Field, s: Source) -> u8 {
    match (field, s) {
        (Field::Motion, Source::Adsb) | (Field::Identity, Source::Vdl2) => 3,
        (Field::Motion, Source::Vdl2) | (Field::Identity, Source::Adsb) => 2,
        (_, Source::Hfdl) => 1,
        (_, Source::AcarsAudio) => 0,
    }
}

/// The replacement rule, in one place: newer wins, then source priority.
fn put<T>(slot: &mut Option<Stamped<T>>, field: Field, value: T, source: Source, at: i64) {
    let replace = match slot {
        None => true,
        Some(cur) => {
            at > cur.at || (at == cur.at && priority(field, source) > priority(field, cur.source))
        }
    };
    if replace {
        *slot = Some(Stamped { value, source, at });
    }
}

/// `.PH-BXA` and `ph-bxa ` are the same aeroplane.
fn normalize_reg(raw: &str) -> Option<String> {
    let r = raw.trim().trim_start_matches('.').trim().to_uppercase();
    (!r.is_empty()).then_some(r)
}

fn nonempty(raw: &str) -> Option<String> {
    let t = raw.trim();
    (!t.is_empty()).then(|| t.to_string())
}

impl AircraftCard {
    fn touch(&mut self, at: i64) {
        self.last_seen = self.last_seen.max(at);
    }

    fn put_position(&mut self, lat: f64, lon: f64, source: Source, at: i64) {
        put(&mut self.position, Field::Motion, (lat, lon), source, at);
        self.touch(at);
    }

    fn put_flight(&mut self, flight: &str, source: Source, at: i64) {
        if let Some(f) = nonempty(flight) {
            put(&mut self.flight, Field::Identity, f, source, at);
        }
        self.touch(at);
    }
}

/// Drop the oldest entries until `map` fits in [`MAX_CARDS`].
fn trim<K: Clone + Eq + std::hash::Hash>(map: &mut HashMap<K, AircraftCard>) {
    if map.len() <= MAX_CARDS {
        return;
    }
    let mut by_age: Vec<(i64, K)> = map.iter().map(|(k, c)| (c.last_seen, k.clone())).collect();
    by_age.sort_by_key(|(t, _)| *t);
    for (_, k) in by_age.into_iter().take(map.len() - MAX_CARDS) {
        map.remove(&k);
    }
}

impl AircraftRegistry {
    pub fn ingest_adsb(&mut self, a: &AdsbAircraft) {
        let c = self.cards.entry(a.icao & 0xff_ffff).or_default();
        c.put_flight(&a.callsign, Source::Adsb, a.last_at);
        if let (Some(lat), Some(lon)) = (a.lat, a.lon) {
            if a.last_pos_at > 0 {
                c.put_position(lat, lon, Source::Adsb, a.last_pos_at);
            }
        }
        // Altitude, speed and track ride the same squitters as the rest.
        if let Some(v) = a.altitude_ft {
            put(&mut c.altitude_ft, Field::Motion, v, Source::Adsb, a.last_at);
        }
        if let Some(v) = a.ground_speed_kt {
            put(&mut c.ground_speed_kt, Field::Motion, v, Source::Adsb, a.last_at);
        }
        if let Some(v) = a.track_deg {
            put(&mut c.track_deg, Field::Motion, v, Source::Adsb, a.last_at);
        }
        if let Some(v) = a.selected_altitude_ft {
            put(&mut c.selected_altitude_ft, Field::Motion, v, Source::Adsb, a.last_at);
        }
        if let Some(v) = a.selected_heading_deg {
            put(&mut c.selected_heading_deg, Field::Motion, v, Source::Adsb, a.last_at);
        }
        if let Some(v) = a.qnh_hpa {
            put(&mut c.qnh_hpa, Field::Motion, v, Source::Adsb, a.last_at);
        }
        if a.ias_kt.is_some() || a.tas_kt.is_some() {
            put(&mut c.airspeed, Field::Motion, (a.ias_kt, a.tas_kt), Source::Adsb, a.last_at);
        }
        if let Some(v) = a.heading_deg {
            put(&mut c.heading_deg, Field::Motion, v, Source::Adsb, a.last_at);
        }
        if let Some(v) = a.nav_modes {
            put(&mut c.nav_modes, Field::Motion, v, Source::Adsb, a.last_at);
        }
        if let Some(v) = a.adsb_version {
            put(&mut c.adsb_version, Field::Motion, v, Source::Adsb, a.last_at);
        }
        if a.nac_p.is_some() || a.nac_v.is_some() || a.sil.is_some() {
            put(
                &mut c.accuracy,
                Field::Motion,
                (a.nac_p, a.nac_v, a.sil),
                Source::Adsb,
                a.last_at,
            );
        }
        c.touch(a.last_at);
        trim(&mut self.cards);
    }

    pub fn ingest_vdl2(&mut self, m: &Vdl2Message) {
        // The aircraft end of the frame: the sender, else the addressee.
        let icao = if m.src_kind == Vdl2AddrKind::Aircraft {
            m.src
        } else if m.dst_kind == Vdl2AddrKind::Aircraft {
            m.dst
        } else {
            return;
        } & 0xff_ffff;
        let from_aircraft = m.src_kind == Vdl2AddrKind::Aircraft;
        let c = self.cards.entry(icao).or_default();
        c.touch(m.at);
        match &m.payload {
            Vdl2Payload::Acars(ac) => {
                if let Some(reg) = normalize_reg(&ac.registration) {
                    put(&mut c.registration, Field::Identity, reg.clone(), Source::Vdl2, m.at);
                    self.by_reg.insert(reg, icao);
                }
                c.put_flight(&ac.flight, Source::Vdl2, m.at);
                put(&mut c.last_message, Field::Identity, ac.summary(), Source::Vdl2, m.at);
            }
            Vdl2Payload::Xid(x) if from_aircraft => {
                if let (Some(lat), Some(lon)) = (x.lat, x.lon) {
                    c.put_position(lat, lon, Source::Vdl2, m.at);
                }
            }
            _ => {}
        }
        trim(&mut self.cards);
    }

    pub fn ingest_hfdl(&mut self, d: &HfdlDecode) {
        let Some(fix) = &d.position else { return };
        let icao = fix
            .icao
            .as_deref()
            .map(|s| s.trim().trim_start_matches("0x"))
            .and_then(|s| u32::from_str_radix(s, 16).ok())
            .map(|v| v & 0xff_ffff);
        let flight = fix.flight.as_deref().and_then(nonempty);
        let card = match (icao, fix.aircraft_id, &flight) {
            (Some(icao), _, _) => self.cards.entry(icao).or_default(),
            (None, Some(id), _) => self.fallback.entry(FallbackKey::AircraftId(id)).or_default(),
            (None, None, Some(f)) => {
                self.fallback.entry(FallbackKey::Flight(f.clone())).or_default()
            }
            // No identity at all: nothing to attach the fix to.
            (None, None, None) => return,
        };
        card.put_position(fix.lat, fix.lon, Source::Hfdl, d.unix);
        if let Some(f) = &flight {
            card.put_flight(f, Source::Hfdl, d.unix);
        }
        trim(&mut self.cards);
        trim(&mut self.fallback);
    }

    /// Enriches a card that already exists; never creates one.
    pub fn ingest_acars(&mut self, m: &AcarsMessage) {
        let Some(reg) = normalize_reg(&m.address) else { return };
        let Some(icao) = self.by_reg.get(&reg) else { return };
        let Some(c) = self.cards.get_mut(icao) else { return };
        let text = m.text.trim();
        let line = if text.is_empty() {
            format!("[{}]", m.label)
        } else {
            format!("[{}] {text}", m.label)
        };
        put(&mut c.last_message, Field::Identity, line, Source::AcarsAudio, m.at);
        c.touch(m.at);
    }

    /// Drops what has been silent for more than [`CARD_TTL_S`], then any link
    /// whose card is gone. A card with a fresh datum is never touched.
    pub fn expire(&mut self, now: i64) {
        let stale = |c: &AircraftCard| now - c.last_seen > CARD_TTL_S;
        self.cards.retain(|_, c| !stale(c));
        self.fallback.retain(|_, c| !stale(c));
        let cards = &self.cards;
        self.by_reg.retain(|_, icao| cards.contains_key(icao));
        trim(&mut self.cards);
        trim(&mut self.fallback);
    }

    pub fn card(&self, icao: u32) -> Option<&AircraftCard> {
        self.cards.get(&(icao & 0xff_ffff))
    }
    pub fn fallback_card(&self, key: &FallbackKey) -> Option<&AircraftCard> {
        self.fallback.get(key)
    }
    /// Every ICAO card, in no particular order.
    pub fn entries(&self) -> impl Iterator<Item = (u32, &AircraftCard)> {
        self.cards.iter().map(|(k, c)| (*k, c))
    }
    /// Every HFDL card that never gave an address, in no particular order.
    pub fn fallback_entries(&self) -> impl Iterator<Item = (&FallbackKey, &AircraftCard)> {
        self.fallback.iter()
    }
    pub fn len(&self) -> usize {
        self.cards.len()
    }
    pub fn fallback_len(&self) -> usize {
        self.fallback.len()
    }
    #[cfg(test)]
    pub fn linked_registrations(&self) -> usize {
        self.by_reg.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdroxide_types::{HfdlFix, Vdl2Acars, Vdl2Frame};

    const ICAO_A: u32 = 0x4CA1FA;
    const ICAO_B: u32 = 0x3C6444;

    fn adsb(icao: u32, call: &str, alt: i32, at: i64) -> AdsbAircraft {
        let mut a = AdsbAircraft::new(icao, at);
        a.callsign = call.to_string();
        a.lat = Some(52.0);
        a.lon = Some(4.0);
        a.altitude_ft = Some(alt);
        a.ground_speed_kt = Some(430.0);
        a.track_deg = Some(90.0);
        a.last_at = at;
        a.last_pos_at = at;
        a
    }

    fn vdl2(src: u32, reg: &str, flight: &str, at: i64) -> Vdl2Message {
        Vdl2Message {
            at,
            freq_hz: 136_975_000.0,
            src,
            src_kind: Vdl2AddrKind::Aircraft,
            dst: 0x123456,
            dst_kind: Vdl2AddrKind::GroundAdmin,
            command: true,
            frame: Vdl2Frame::I { ns: 0, nr: 0, p: false },
            payload: Vdl2Payload::Acars(Vdl2Acars {
                registration: reg.to_string(),
                flight: flight.to_string(),
                label: "H1".to_string(),
                text: "HELLO".to_string(),
                ..Default::default()
            }),
            snr_db: 10.0,
            rssi_dbfs: -30.0,
            evm_deg: 5.0,
            freq_err_hz: 0.0,
            rs_corrected: 0,
            raw_hex: String::new(),
        }
    }

    fn hfdl(
        icao: Option<&str>,
        id: Option<u32>,
        flight: Option<&str>,
        lat: f64,
        at: i64,
    ) -> HfdlDecode {
        HfdlDecode {
            unix: at,
            kind: "performance-data".to_string(),
            gs: None,
            freq_khz: 8_912,
            snr_db: None,
            freq_skew_hz: None,
            fec_corrected: None,
            details: String::new(),
            position: Some(HfdlFix {
                lat,
                lon: -30.0,
                aircraft_id: id,
                icao: icao.map(str::to_string),
                flight: flight.map(str::to_string),
            }),
        }
    }

    fn acars(addr: &str, at: i64) -> AcarsMessage {
        AcarsMessage {
            address: addr.to_string(),
            label: "H1".to_string(),
            text: "OVER AUDIO".to_string(),
            crc_ok: true,
            at,
            ..Default::default()
        }
    }

    #[test]
    fn adsb_then_vdl2_then_hfdl_merge_on_one_icao() {
        let mut r = AircraftRegistry::default();
        r.ingest_adsb(&adsb(ICAO_A, "KLM123", 35_000, 1_000));
        r.ingest_vdl2(&vdl2(ICAO_A, ".PH-BXA", "KL0123", 1_001));
        r.ingest_hfdl(&hfdl(Some("4CA1FA"), Some(7), Some("KLM9"), 10.0, 1_000));
        assert_eq!(r.len(), 1);
        let c = r.card(ICAO_A).expect("card");
        // HFDL as old as the ADS-B position does not displace it.
        assert_eq!(c.position.as_ref().unwrap().value, (52.0, 4.0));
        assert_eq!(c.position.as_ref().unwrap().source, Source::Adsb);
        assert_eq!(c.altitude_ft.as_ref().unwrap().value, 35_000);
        assert_eq!(c.registration.as_ref().unwrap().value, "PH-BXA");
        assert_eq!(c.registration.as_ref().unwrap().source, Source::Vdl2);
        // The VDL2 flight is the newest reading, so it stands.
        assert_eq!(c.flight.as_ref().unwrap().value, "KL0123");
        assert_eq!(c.flight.as_ref().unwrap().source, Source::Vdl2);
    }

    #[test]
    fn a_newer_hfdl_position_replaces_an_older_adsb_one() {
        let mut r = AircraftRegistry::default();
        r.ingest_adsb(&adsb(ICAO_A, "KLM123", 35_000, 1_000));
        r.ingest_hfdl(&hfdl(Some("4CA1FA"), None, None, 10.0, 1_500));
        let p = r.card(ICAO_A).unwrap().position.as_ref().unwrap();
        assert_eq!((p.value, p.source), ((10.0, -30.0), Source::Hfdl));
    }

    #[test]
    fn equal_age_goes_to_the_higher_priority_source() {
        let mut r = AircraftRegistry::default();
        r.ingest_hfdl(&hfdl(Some("4CA1FA"), None, Some("HFX1"), 10.0, 1_000));
        r.ingest_adsb(&adsb(ICAO_A, "KLM123", 35_000, 1_000));
        let c = r.card(ICAO_A).unwrap();
        assert_eq!(c.position.as_ref().unwrap().source, Source::Adsb);
        assert_eq!(c.flight.as_ref().unwrap().source, Source::Adsb);
        // VDL2 beats ADS-B for the flight at equal age.
        r.ingest_vdl2(&vdl2(ICAO_A, "PH-BXA", "KL0123", 1_000));
        assert_eq!(r.card(ICAO_A).unwrap().flight.as_ref().unwrap().source, Source::Vdl2);
        // And the reverse order cannot undo it.
        r.ingest_adsb(&adsb(ICAO_A, "KLM123", 35_000, 1_000));
        assert_eq!(r.card(ICAO_A).unwrap().flight.as_ref().unwrap().value, "KL0123");
    }

    #[test]
    fn older_hfdl_does_not_replace_a_newer_adsb_position() {
        let mut r = AircraftRegistry::default();
        r.ingest_adsb(&adsb(ICAO_A, "KLM123", 35_000, 2_000));
        r.ingest_hfdl(&hfdl(Some("4CA1FA"), None, None, 10.0, 1_500));
        assert_eq!(r.card(ICAO_A).unwrap().position.as_ref().unwrap().value, (52.0, 4.0));
    }

    #[test]
    fn re_sent_snapshots_change_nothing() {
        let mut r = AircraftRegistry::default();
        let a = adsb(ICAO_A, "KLM123", 35_000, 1_000);
        r.ingest_adsb(&a);
        let once = r.card(ICAO_A).unwrap().clone();
        r.ingest_adsb(&a);
        assert_eq!(r.card(ICAO_A).unwrap(), &once);
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn acars_attaches_by_registration_learned_from_vdl2() {
        let mut r = AircraftRegistry::default();
        r.ingest_vdl2(&vdl2(ICAO_A, "PH-BXA", "KL0123", 1_000));
        r.ingest_acars(&acars("PH-BXA", 1_010));
        let c = r.card(ICAO_A).unwrap();
        let m = c.last_message.as_ref().unwrap();
        assert_eq!(m.source, Source::AcarsAudio);
        assert_eq!(m.at, 1_010);
        assert!(m.value.contains("OVER AUDIO"));
        assert_eq!(r.len(), 1);
    }

    #[test]
    fn acars_with_unknown_registration_creates_nothing() {
        let mut r = AircraftRegistry::default();
        r.ingest_adsb(&adsb(ICAO_A, "PHBXA", 35_000, 1_000));
        r.ingest_acars(&acars("PH-ZZZ", 1_010));
        assert_eq!(r.len(), 1);
        assert_eq!(r.fallback_len(), 0);
        assert!(r.card(ICAO_A).unwrap().last_message.is_none());
    }

    #[test]
    fn adsb_callsign_never_links_a_registration() {
        let mut r = AircraftRegistry::default();
        // A callsign that looks exactly like the registration must not link.
        r.ingest_adsb(&adsb(ICAO_A, "PH-BXA", 35_000, 1_000));
        r.ingest_acars(&acars("PH-BXA", 1_010));
        assert_eq!(r.linked_registrations(), 0);
        assert!(r.card(ICAO_A).unwrap().last_message.is_none());
    }

    #[test]
    fn acars_link_to_an_expired_card_is_ignored() {
        let mut r = AircraftRegistry::default();
        r.ingest_vdl2(&vdl2(ICAO_A, "PH-BXA", "KL0123", 1_000));
        r.expire(1_000 + CARD_TTL_S + 1);
        r.ingest_acars(&acars("PH-BXA", 1_000 + CARD_TTL_S + 2));
        assert_eq!(r.len(), 0);
        assert_eq!(r.fallback_len(), 0);
    }

    #[test]
    fn two_aircraft_stay_apart() {
        let mut r = AircraftRegistry::default();
        r.ingest_adsb(&adsb(ICAO_A, "KLM123", 35_000, 1_000));
        r.ingest_adsb(&adsb(ICAO_B, "BAW9", 12_000, 1_000));
        r.ingest_vdl2(&vdl2(ICAO_B, "G-EUUU", "BA0009", 1_001));
        r.ingest_acars(&acars("G-EUUU", 1_002));
        assert_eq!(r.len(), 2);
        let a = r.card(ICAO_A).unwrap();
        let b = r.card(ICAO_B).unwrap();
        assert_eq!(a.altitude_ft.as_ref().unwrap().value, 35_000);
        assert_eq!(b.altitude_ft.as_ref().unwrap().value, 12_000);
        assert!(a.registration.is_none() && a.last_message.is_none());
        assert_eq!(b.registration.as_ref().unwrap().value, "G-EUUU");
        assert!(b.last_message.is_some());
    }

    #[test]
    fn cards_expire_after_the_ttl() {
        let mut r = AircraftRegistry::default();
        r.ingest_adsb(&adsb(ICAO_A, "KLM123", 35_000, 1_000));
        r.expire(1_000 + CARD_TTL_S);
        assert_eq!(r.len(), 1, "exactly at the TTL is still fresh");
        r.expire(1_000 + CARD_TTL_S + 1);
        assert_eq!(r.len(), 0);
    }

    #[test]
    fn expire_purges_orphans_and_fallback_but_keeps_fresh_icao_cards() {
        let mut r = AircraftRegistry::default();
        r.ingest_vdl2(&vdl2(ICAO_A, "PH-BXA", "KL0123", 1_000));
        r.ingest_hfdl(&hfdl(None, Some(7), None, 10.0, 1_000));
        // B stays fresh.
        r.ingest_adsb(&adsb(ICAO_B, "BAW9", 12_000, 1_500));
        r.expire(1_000 + CARD_TTL_S + 1);
        assert!(r.card(ICAO_A).is_none());
        assert_eq!(r.fallback_len(), 0);
        assert_eq!(r.linked_registrations(), 0, "orphan link purged");
        assert!(r.card(ICAO_B).is_some(), "fresh ICAO card survives");
    }

    #[test]
    fn hfdl_without_icao_goes_to_the_fallback_only() {
        let mut r = AircraftRegistry::default();
        r.ingest_hfdl(&hfdl(None, Some(7), None, 10.0, 1_000));
        r.ingest_hfdl(&hfdl(None, None, Some("FLT1"), 11.0, 1_000));
        assert_eq!(r.len(), 0);
        assert_eq!(r.fallback_len(), 2);
        let c = r.fallback_card(&FallbackKey::AircraftId(7)).unwrap();
        assert_eq!(c.position.as_ref().unwrap().value, (10.0, -30.0));
        assert_eq!(c.position.as_ref().unwrap().source, Source::Hfdl);
        assert!(r.fallback_card(&FallbackKey::Flight("FLT1".to_string())).is_some());
    }

    #[test]
    fn hfdl_with_no_identity_at_all_is_dropped() {
        let mut r = AircraftRegistry::default();
        r.ingest_hfdl(&hfdl(None, None, None, 10.0, 1_000));
        assert_eq!(r.len() + r.fallback_len(), 0);
    }

    #[test]
    fn the_cap_evicts_the_oldest() {
        let mut r = AircraftRegistry::default();
        for i in 0..(MAX_CARDS as u32 + 3) {
            r.ingest_adsb(&adsb(0x100000 + i, "X", 1_000, 1_000 + i as i64));
        }
        assert_eq!(r.len(), MAX_CARDS);
        // The three oldest are gone, the newest is there.
        for i in 0..3 {
            assert!(r.card(0x100000 + i).is_none(), "oldest {i} evicted");
        }
        assert!(r.card(0x100000 + MAX_CARDS as u32 + 2).is_some());
    }

    #[test]
    fn the_cap_also_bounds_the_fallback_table() {
        let mut r = AircraftRegistry::default();
        for i in 0..(MAX_CARDS as u32 + 2) {
            r.ingest_hfdl(&hfdl(None, Some(i), None, 10.0, 1_000 + i as i64));
        }
        assert_eq!(r.fallback_len(), MAX_CARDS);
        assert!(r.fallback_card(&FallbackKey::AircraftId(0)).is_none());
    }
}
