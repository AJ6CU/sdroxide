//! The WSJT-X datagram formats, as pure byte builders.
//!
//! Every message opens with the same header — magic, schema, type, and the
//! sending station's id — and continues with that type's fields in Qt
//! `QDataStream` encoding, big-endian throughout:
//!
//! | Qt type      | on the wire                                              |
//! |--------------|----------------------------------------------------------|
//! | `QString`    | `u32` byte count then UTF-8; `0xFFFFFFFF` means null      |
//! | `bool`       | one byte                                                  |
//! | `QTime`      | `u32` milliseconds since midnight                         |
//! | `QDateTime`  | `i64` Julian day, `u32` ms, `u8` time spec (1 = UTC)      |
//! | `double`     | 8 bytes IEEE-754 (Qt streams floats at double precision)  |

use sdroxide_types::QsoRecord;

/// Marks a datagram as this protocol.
const MAGIC: u32 = 0xadbc_cbda;
/// Schema WSJT-X 2.x speaks, and the one every current client expects.
const SCHEMA: u32 = 3;

/// Julian day number of 1970-01-01, for `QDate`.
const UNIX_EPOCH_JD: i64 = 2_440_588;

// Message type numbers, from `NetworkMessage.hpp`.
const T_HEARTBEAT: u32 = 0;
const T_STATUS: u32 = 1;
const T_DECODE: u32 = 2;
const T_CLEAR: u32 = 3;
const T_QSO_LOGGED: u32 = 5;
const T_CLOSE: u32 = 6;
const T_LOGGED_ADIF: u32 = 12;

/// One decoded message, as the clients' decode window shows it.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DecodeInfo {
    /// False replays an old decode (clients may re-sort rather than alert).
    pub new: bool,
    /// Unix seconds of the slot the message was decoded from.
    pub slot_utc: i64,
    pub snr_db: i32,
    /// Time offset from the slot start, in seconds.
    pub dt: f64,
    /// Audio offset within the passband, in Hz.
    pub audio_hz: u32,
    pub mode: String,
    pub message: String,
}

/// Where this station is and what it's doing.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StatusInfo {
    pub dial_hz: u64,
    pub mode: String,
    pub dx_call: String,
    /// The report we're sending them, as a bare number ("-13").
    pub report: String,
    pub tx_enabled: bool,
    pub transmitting: bool,
    pub decoding: bool,
    pub rx_df_hz: u32,
    pub tx_df_hz: u32,
    pub de_call: String,
    pub de_grid: String,
    pub dx_grid: String,
    pub tx_watchdog: bool,
    /// Transmit/receive period in seconds (FT8 15, FT4 7.5 → 7).
    pub tr_period_s: u32,
    /// The message queued for the next transmission.
    pub tx_message: String,
}

// ── QDataStream primitives ──────────────────────────────────────────────────

fn u8_(out: &mut Vec<u8>, v: u8) {
    out.push(v);
}

fn u32_(out: &mut Vec<u8>, v: u32) {
    out.extend_from_slice(&v.to_be_bytes());
}

fn i32_(out: &mut Vec<u8>, v: i32) {
    out.extend_from_slice(&v.to_be_bytes());
}

fn u64_(out: &mut Vec<u8>, v: u64) {
    out.extend_from_slice(&v.to_be_bytes());
}

fn i64_(out: &mut Vec<u8>, v: i64) {
    out.extend_from_slice(&v.to_be_bytes());
}

fn f64_(out: &mut Vec<u8>, v: f64) {
    out.extend_from_slice(&v.to_be_bytes());
}

fn bool_(out: &mut Vec<u8>, v: bool) {
    out.push(u8::from(v));
}

/// A `QString`. An empty string is sent as a *null* string (`0xFFFFFFFF`),
/// which is how WSJT-X sends its own unset fields — clients test for null.
fn str_(out: &mut Vec<u8>, s: &str) {
    if s.is_empty() {
        u32_(out, 0xFFFF_FFFF);
        return;
    }
    u32_(out, s.len() as u32);
    out.extend_from_slice(s.as_bytes());
}

/// A `QTime`: milliseconds since midnight UTC of the given instant.
fn time_(out: &mut Vec<u8>, unix: i64) {
    u32_(out, (unix.rem_euclid(86_400) * 1000) as u32);
}

/// A `QDateTime` in UTC.
fn datetime_(out: &mut Vec<u8>, unix: i64) {
    i64_(out, unix.div_euclid(86_400) + UNIX_EPOCH_JD);
    u32_(out, (unix.rem_euclid(86_400) * 1000) as u32);
    u8_(out, 1); // Qt::UTC
}

/// Magic, schema, message type, and the sending station's id.
fn header(out: &mut Vec<u8>, kind: u32, id: &str) {
    u32_(out, MAGIC);
    u32_(out, SCHEMA);
    u32_(out, kind);
    str_(out, id);
}

// ── Messages ────────────────────────────────────────────────────────────────

pub fn heartbeat(id: &str, version: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(64);
    header(&mut out, T_HEARTBEAT, id);
    u32_(&mut out, SCHEMA); // maximum schema we understand
    str_(&mut out, version);
    str_(&mut out, ""); // revision
    out
}

pub fn decode(id: &str, d: &DecodeInfo) -> Vec<u8> {
    let mut out = Vec::with_capacity(96);
    header(&mut out, T_DECODE, id);
    bool_(&mut out, d.new);
    time_(&mut out, d.slot_utc);
    i32_(&mut out, d.snr_db);
    f64_(&mut out, d.dt);
    u32_(&mut out, d.audio_hz);
    str_(&mut out, &d.mode);
    str_(&mut out, &d.message);
    bool_(&mut out, false); // low confidence
    bool_(&mut out, false); // off air
    out
}

pub fn status(id: &str, s: &StatusInfo) -> Vec<u8> {
    let mut out = Vec::with_capacity(160);
    header(&mut out, T_STATUS, id);
    u64_(&mut out, s.dial_hz);
    str_(&mut out, &s.mode);
    str_(&mut out, &s.dx_call);
    str_(&mut out, &s.report);
    str_(&mut out, &s.mode); // tx mode (never differs from the rx mode here)
    bool_(&mut out, s.tx_enabled);
    bool_(&mut out, s.transmitting);
    bool_(&mut out, s.decoding);
    u32_(&mut out, s.rx_df_hz);
    u32_(&mut out, s.tx_df_hz);
    str_(&mut out, &s.de_call);
    str_(&mut out, &s.de_grid);
    str_(&mut out, &s.dx_grid);
    bool_(&mut out, s.tx_watchdog);
    str_(&mut out, ""); // sub-mode (JT65 only)
    bool_(&mut out, false); // fast mode
    u8_(&mut out, 0); // special operation mode: none
    u32_(&mut out, 0); // frequency tolerance: not applicable
    u32_(&mut out, s.tr_period_s);
    str_(&mut out, "Default"); // configuration name
    str_(&mut out, &s.tx_message);
    out
}

pub fn clear(id: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(32);
    header(&mut out, T_CLEAR, id);
    out
}

pub fn close(id: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(32);
    header(&mut out, T_CLOSE, id);
    out
}

pub fn qso_logged(id: &str, q: &QsoRecord) -> Vec<u8> {
    let mut out = Vec::with_capacity(224);
    header(&mut out, T_QSO_LOGGED, id);
    datetime_(&mut out, q.end_utc);
    str_(&mut out, &q.call);
    str_(&mut out, q.grid.as_deref().unwrap_or(""));
    u64_(&mut out, q.freq_hz.max(0.0) as u64);
    str_(&mut out, &q.mode);
    str_(&mut out, &report(q.rst_sent));
    str_(&mut out, &report(q.rst_rcvd));
    str_(&mut out, &q.tx_pwr.map(|w| format!("{w:.0}")).unwrap_or_default());
    str_(&mut out, &q.comment);
    str_(&mut out, &q.name);
    datetime_(&mut out, q.start_utc);
    str_(&mut out, &q.operator);
    str_(&mut out, &q.my_call);
    str_(&mut out, &q.my_grid);
    str_(&mut out, &q.stx_string);
    str_(&mut out, &q.srx_string);
    str_(&mut out, ""); // ADIF propagation mode
    out
}

pub fn logged_adif(id: &str, adif: &str) -> Vec<u8> {
    let mut out = Vec::with_capacity(adif.len() + 32);
    header(&mut out, T_LOGGED_ADIF, id);
    str_(&mut out, adif);
    out
}

/// A signal report as the protocol carries it: a bare signed number for the
/// digital modes, empty when there is none.
fn report(db: Option<i16>) -> String {
    db.map(|v| v.to_string()).unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Read fields back the way a client does, so the tests describe the wire
    /// rather than restating the writer.
    struct Reader<'a> {
        b: &'a [u8],
        at: usize,
    }

    impl<'a> Reader<'a> {
        fn new(b: &'a [u8]) -> Self {
            Reader { b, at: 0 }
        }
        fn u8(&mut self) -> u8 {
            self.at += 1;
            self.b[self.at - 1]
        }
        fn bool(&mut self) -> bool {
            self.u8() != 0
        }
        fn u32(&mut self) -> u32 {
            let v = u32::from_be_bytes(self.b[self.at..self.at + 4].try_into().unwrap());
            self.at += 4;
            v
        }
        fn i32(&mut self) -> i32 {
            self.u32() as i32
        }
        fn u64(&mut self) -> u64 {
            let v = u64::from_be_bytes(self.b[self.at..self.at + 8].try_into().unwrap());
            self.at += 8;
            v
        }
        fn i64(&mut self) -> i64 {
            self.u64() as i64
        }
        fn f64(&mut self) -> f64 {
            f64::from_be_bytes({
                let v = self.b[self.at..self.at + 8].try_into().unwrap();
                self.at += 8;
                v
            })
        }
        fn str(&mut self) -> Option<String> {
            let n = self.u32();
            if n == 0xFFFF_FFFF {
                return None;
            }
            let s = String::from_utf8(self.b[self.at..self.at + n as usize].to_vec()).unwrap();
            self.at += n as usize;
            Some(s)
        }
        fn header(&mut self) -> (u32, u32, Option<String>) {
            assert_eq!(self.u32(), MAGIC, "magic");
            let schema = self.u32();
            let kind = self.u32();
            (schema, kind, self.str())
        }
        fn at_end(&self) -> bool {
            self.at == self.b.len()
        }
    }

    #[test]
    fn every_message_opens_with_the_same_header() {
        for (packet, kind) in [
            (heartbeat("sdroxide", "0.6.0"), T_HEARTBEAT),
            (clear("sdroxide"), T_CLEAR),
            (close("sdroxide"), T_CLOSE),
            (logged_adif("sdroxide", "<CALL:5>W9XYZ<EOR>"), T_LOGGED_ADIF),
        ] {
            let mut r = Reader::new(&packet);
            let (schema, got, id) = r.header();
            assert_eq!(schema, SCHEMA);
            assert_eq!(got, kind);
            assert_eq!(id.as_deref(), Some("sdroxide"));
        }
    }

    #[test]
    fn a_decode_reads_back_field_for_field() {
        let d = DecodeInfo {
            new: true,
            // 2023-11-14 22:13:20 UTC → 22:13:20 into the day.
            slot_utc: 1_700_000_000,
            snr_db: -13,
            dt: 0.25,
            audio_hz: 1234,
            mode: "FT8".into(),
            message: "CQ W9XYZ EM48".into(),
        };
        let packet = decode("WSJT-X", &d);
        let mut r = Reader::new(&packet);
        assert_eq!(r.header().1, T_DECODE);
        assert!(r.bool(), "new");
        assert_eq!(r.u32(), ((22 * 3600 + 13 * 60 + 20) * 1000) as u32, "ms since midnight");
        assert_eq!(r.i32(), -13);
        assert_eq!(r.f64(), 0.25);
        assert_eq!(r.u32(), 1234);
        assert_eq!(r.str().as_deref(), Some("FT8"));
        assert_eq!(r.str().as_deref(), Some("CQ W9XYZ EM48"));
        assert!(!r.bool(), "low confidence");
        assert!(!r.bool(), "off air");
        assert!(r.at_end(), "trailing bytes would shift a client's next field");
    }

    #[test]
    fn status_carries_the_full_schema_3_field_list() {
        // A client reads these positionally: a missing or mistyped field
        // silently corrupts everything after it, so count them all off.
        let s = StatusInfo {
            dial_hz: 14_074_000,
            mode: "FT8".into(),
            dx_call: "W9XYZ".into(),
            report: "-13".into(),
            tx_enabled: true,
            transmitting: false,
            decoding: true,
            rx_df_hz: 1500,
            tx_df_hz: 1500,
            de_call: "AB1CD".into(),
            de_grid: "FN42".into(),
            dx_grid: "EM48".into(),
            tx_watchdog: false,
            tr_period_s: 15,
            tx_message: "W9XYZ AB1CD FN42".into(),
        };
        let packet = status("WSJT-X", &s);
        let mut r = Reader::new(&packet);
        assert_eq!(r.header().1, T_STATUS);
        assert_eq!(r.u64(), 14_074_000);
        assert_eq!(r.str().as_deref(), Some("FT8"));
        assert_eq!(r.str().as_deref(), Some("W9XYZ"));
        assert_eq!(r.str().as_deref(), Some("-13"));
        assert_eq!(r.str().as_deref(), Some("FT8"), "tx mode");
        assert!(r.bool(), "tx enabled");
        assert!(!r.bool(), "transmitting");
        assert!(r.bool(), "decoding");
        assert_eq!(r.u32(), 1500);
        assert_eq!(r.u32(), 1500);
        assert_eq!(r.str().as_deref(), Some("AB1CD"));
        assert_eq!(r.str().as_deref(), Some("FN42"));
        assert_eq!(r.str().as_deref(), Some("EM48"));
        assert!(!r.bool(), "tx watchdog");
        assert_eq!(r.str(), None, "sub-mode is null, not empty");
        assert!(!r.bool(), "fast mode");
        assert_eq!(r.u8(), 0, "special operation mode");
        assert_eq!(r.u32(), 0, "frequency tolerance");
        assert_eq!(r.u32(), 15, "T/R period");
        assert_eq!(r.str().as_deref(), Some("Default"));
        assert_eq!(r.str().as_deref(), Some("W9XYZ AB1CD FN42"));
        assert!(r.at_end());
    }

    #[test]
    fn a_logged_qso_reads_back_as_the_logger_expects() {
        let q = QsoRecord {
            call: "W9XYZ".into(),
            grid: Some("EM48".into()),
            rst_sent: Some(-9),
            rst_rcvd: Some(-12),
            freq_hz: 14_075_500.0,
            mode: "FT8".into(),
            band: "20m".into(),
            start_utc: 1_700_000_000,
            end_utc: 1_700_000_060,
            my_call: "AB1CD".into(),
            my_grid: "FN42".into(),
            ..Default::default()
        };
        let packet = qso_logged("WSJT-X", &q);
        let mut r = Reader::new(&packet);
        assert_eq!(r.header().1, T_QSO_LOGGED);
        // QDateTime off: Julian day then ms, in UTC.
        assert_eq!(r.i64(), 1_700_000_060 / 86_400 + UNIX_EPOCH_JD);
        assert_eq!(r.u32(), ((1_700_000_060 % 86_400) * 1000) as u32);
        assert_eq!(r.u8(), 1, "Qt::UTC");
        assert_eq!(r.str().as_deref(), Some("W9XYZ"));
        assert_eq!(r.str().as_deref(), Some("EM48"));
        assert_eq!(r.u64(), 14_075_500);
        assert_eq!(r.str().as_deref(), Some("FT8"));
        assert_eq!(r.str().as_deref(), Some("-9"));
        assert_eq!(r.str().as_deref(), Some("-12"));
        assert_eq!(r.str(), None, "tx power unset");
        assert_eq!(r.str(), None, "comment");
        assert_eq!(r.str(), None, "name");
        assert_eq!(r.i64(), 1_700_000_000 / 86_400 + UNIX_EPOCH_JD, "date on");
        assert_eq!(r.u32(), ((1_700_000_000 % 86_400) * 1000) as u32);
        assert_eq!(r.u8(), 1);
        assert_eq!(r.str(), None, "operator");
        assert_eq!(r.str().as_deref(), Some("AB1CD"));
        assert_eq!(r.str().as_deref(), Some("FN42"));
        assert_eq!(r.str(), None, "exchange sent");
        assert_eq!(r.str(), None, "exchange received");
        assert_eq!(r.str(), None, "propagation mode");
        assert!(r.at_end());
    }
}
