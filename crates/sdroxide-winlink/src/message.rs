//! The Winlink message format, and the message IDs that name one.
//!
//! It looks like email and is not. Headers are RFC-822-*style* ASCII lines,
//! but the body is not delimited by anything — a `Body:` header declares its
//! exact byte count, and each attachment declares its own with a `File:` line.
//! Everything after the blank line is positional: `Body` bytes of text, then
//! each file's bytes back to back, in the order the `File:` headers appeared.
//! There is no MIME, no boundary marker, and no base64; attachments ride as
//! raw 8-bit bytes.
//!
//! That design is why a byte-count slip is unrecoverable rather than merely
//! wrong: everything after it is read at the wrong offset.

use std::fmt::Write as _;

use md5::{Digest, Md5};

/// Winlink caps a message ID at 12 characters.
pub const MAX_MID_LEN: usize = 12;

/// Header names, spelled as Winlink spells them. Header *matching* is
/// case-insensitive on the way in — implementations in the wild disagree on
/// capitalisation — but these are what we write.
mod header {
    pub const MID: &str = "Mid";
    pub const DATE: &str = "Date";
    pub const TYPE: &str = "Type";
    pub const FROM: &str = "From";
    pub const TO: &str = "To";
    pub const CC: &str = "Cc";
    pub const SUBJECT: &str = "Subject";
    pub const MBO: &str = "Mbo";
    pub const BODY: &str = "Body";
    pub const FILE: &str = "File";
    pub const CONTENT_TYPE: &str = "Content-Type";
    pub const CONTENT_TRANSFER_ENCODING: &str = "Content-Transfer-Encoding";
}

/// Winlink's default body charset. Latin-1, not UTF-8 — the format predates
/// any expectation otherwise.
pub const DEFAULT_CHARSET: &str = "ISO-8859-1";
/// Bodies ride as raw bytes; nothing is base64'd or quoted-printable.
pub const DEFAULT_TRANSFER_ENCODING: &str = "8bit";

/// One attachment: a name and its bytes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attachment {
    pub name: String,
    pub data: Vec<u8>,
}

/// A Winlink message, in the form that rides the wire inside a proposal.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Message {
    pub mid: String,
    /// `YYYY/MM/DD HH:MM`, UTC. See [`format_date`].
    pub date: String,
    /// `Private` for ordinary mail; also seen: `Service`, `Inquiry`, `Position`.
    pub msg_type: String,
    pub from: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub subject: String,
    /// The originating message box — the sender's own callsign, normally.
    pub mbo: String,
    pub body: String,
    pub attachments: Vec<Attachment>,
}

/// What can go wrong reading a message off the wire.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum MessageError {
    #[error("message has no header/body separator")]
    NoSeparator,
    #[error("message is missing the required {0} header")]
    MissingHeader(&'static str),
    #[error("malformed {0} header: {1:?}")]
    BadHeader(&'static str, String),
    #[error("message declares {want} bytes of {what} but only {got} remain")]
    Truncated { what: &'static str, want: usize, got: usize },
}

/// Format a Unix timestamp the way the `Date` header wants it: `YYYY/MM/DD
/// HH:MM`, always UTC, minute resolution.
pub fn format_date(unix: i64) -> String {
    let (y, mo, d, h, mi, _s) = sdroxide_types::utc_ymd_hms(unix);
    format!("{y:04}/{mo:02}/{d:02} {h:02}:{mi:02}")
}

/// Mint a message ID: base32 of an MD5 over the callsign and the moment,
/// truncated to twelve characters.
///
/// Uniqueness is what matters — the CMS rejects a duplicate MID — so the
/// timestamp is taken at whole-nanosecond resolution and the callsign mixed in
/// to separate two stations minting one in the same instant.
pub fn generate_mid(callsign: &str, unix_nanos: u128) -> String {
    let mut hasher = Md5::new();
    hasher.update(unix_nanos.to_string().as_bytes());
    hasher.update(b"-");
    hasher.update(callsign.to_uppercase().as_bytes());
    let sum = hasher.finalize();

    // RFC 4648 base32, upper case, no padding needed — 16 bytes of digest is
    // far more than the twelve characters we keep.
    const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut out = String::with_capacity(MAX_MID_LEN);
    let mut acc = 0u16;
    let mut bits = 0u8;
    for &byte in sum.iter() {
        acc = (acc << 8) | byte as u16;
        bits += 8;
        while bits >= 5 {
            let idx = ((acc >> (bits - 5)) & 0x1f) as usize;
            out.push(ALPHABET[idx] as char);
            bits -= 5;
            if out.len() == MAX_MID_LEN {
                return out;
            }
        }
    }
    out
}

impl Message {
    /// Serialise to the on-the-wire form. This is what gets LZHUF-compressed
    /// and proposed; the `Body`/`File` counts are computed here so they cannot
    /// disagree with what follows them.
    pub fn encode(&self) -> Vec<u8> {
        let body = self.body.as_bytes();

        // Everything except Mid, which is defined to come first. The rest go
        // out sorted by name — the reference implementations do this for
        // reproducibility, and matching them removes one variable when a
        // message the CMS accepted turns out not to be delivered.
        let mut fields: Vec<(&str, String)> = vec![
            (header::DATE, self.date.clone()),
            (header::TYPE, self.msg_type.clone()),
            (header::FROM, self.from.clone()),
            (header::SUBJECT, self.subject.clone()),
            (header::BODY, body.len().to_string()),
            // Winlink messages carry these explicitly. They are not optional
            // decoration: a message without them is accepted by the CMS and
            // then quietly fails to arrive.
            (header::CONTENT_TRANSFER_ENCODING, DEFAULT_TRANSFER_ENCODING.into()),
            (header::CONTENT_TYPE, format!("text/plain; charset={DEFAULT_CHARSET}")),
        ];
        fields.extend(self.to.iter().map(|to| (header::TO, to.clone())));
        fields.extend(self.cc.iter().map(|cc| (header::CC, cc.clone())));
        if !self.mbo.is_empty() {
            fields.push((header::MBO, self.mbo.clone()));
        }
        for att in &self.attachments {
            fields.push((header::FILE, format!("{} {}", att.data.len(), att.name)));
        }

        // Stable sort, so repeated keys — To, Cc and especially File — keep the
        // order they were added in. File order is load-bearing: it is what
        // pairs each header with its bytes further down.
        fields.sort_by(|a, b| a.0.cmp(b.0));

        let mut head = String::new();
        let _ = writeln!(head, "{}: {}\r", header::MID, self.mid);
        for (name, value) in &fields {
            let _ = writeln!(head, "{name}: {value}\r");
        }
        head.push_str("\r\n");

        let mut out = head.into_bytes();
        out.extend_from_slice(body);
        for att in &self.attachments {
            out.extend_from_slice(&att.data);
        }
        out
    }

    /// Parse the on-the-wire form.
    pub fn decode(raw: &[u8]) -> Result<Message, MessageError> {
        let split = find_separator(raw).ok_or(MessageError::NoSeparator)?;
        let (head, rest) = raw.split_at(split.0);
        let rest = &rest[split.1..];

        let mut msg = Message::default();
        let mut body_len: Option<usize> = None;
        // (declared length, name), in header order — the order is what pairs a
        // File header with its bytes further down.
        let mut files: Vec<(usize, String)> = Vec::new();

        for line in String::from_utf8_lossy(head).lines() {
            let line = line.trim_end_matches('\r');
            if line.is_empty() {
                continue;
            }
            let Some((name, value)) = line.split_once(':') else { continue };
            let value = value.trim();
            // Case-insensitive: implementations in the wild vary.
            match name.trim().to_ascii_lowercase().as_str() {
                "mid" => msg.mid = value.to_string(),
                "date" => msg.date = value.to_string(),
                "type" => msg.msg_type = value.to_string(),
                "from" => msg.from = value.to_string(),
                "to" => msg.to.push(value.to_string()),
                "cc" => msg.cc.push(value.to_string()),
                "subject" => msg.subject = value.to_string(),
                "mbo" => msg.mbo = value.to_string(),
                "body" => {
                    body_len = Some(
                        value
                            .parse()
                            .map_err(|_| MessageError::BadHeader(header::BODY, value.into()))?,
                    );
                }
                "file" => {
                    // `File: <bytes> <name>`; the name may contain spaces.
                    let (size, name) = value
                        .split_once(' ')
                        .ok_or_else(|| MessageError::BadHeader(header::FILE, value.into()))?;
                    let size = size
                        .parse()
                        .map_err(|_| MessageError::BadHeader(header::FILE, value.into()))?;
                    files.push((size, name.trim().to_string()));
                }
                _ => {}
            }
        }

        let body_len = body_len.ok_or(MessageError::MissingHeader(header::BODY))?;
        if rest.len() < body_len {
            return Err(MessageError::Truncated { what: "body", want: body_len, got: rest.len() });
        }
        let (body, mut tail) = rest.split_at(body_len);
        msg.body = String::from_utf8_lossy(body).into_owned();

        for (size, name) in files {
            if tail.len() < size {
                return Err(MessageError::Truncated {
                    what: "attachment",
                    want: size,
                    got: tail.len(),
                });
            }
            let (data, next) = tail.split_at(size);
            msg.attachments.push(Attachment { name, data: data.to_vec() });
            tail = next;
        }

        Ok(msg)
    }

    /// The proposal title for this message — what the peer sees before it
    /// decides whether to accept.
    pub fn title(&self) -> String {
        if self.subject.is_empty() { "No title".into() } else { self.subject.clone() }
    }
}

/// Locate the blank line ending the headers, returning `(header end, separator
/// length)`. Accepts `\r\n\r\n` and bare `\n\n`, because both appear in the
/// wild and rejecting the latter would drop real mail.
fn find_separator(raw: &[u8]) -> Option<(usize, usize)> {
    for i in 0..raw.len() {
        if raw[i..].starts_with(b"\r\n\r\n") {
            return Some((i, 4));
        }
        if raw[i..].starts_with(b"\n\n") {
            return Some((i, 2));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Message {
        Message {
            mid: "TJKYEIMMHSRB".into(),
            date: "2026/08/14 09:30".into(),
            msg_type: "Private".into(),
            from: "OE1XYZ".into(),
            to: vec!["OE3ABC".into(), "SMTP:someone@example.org".into()],
            cc: vec![],
            subject: "Test message".into(),
            mbo: "OE1XYZ".into(),
            body: "Hello over the air.\r\n".into(),
            attachments: vec![],
        }
    }

    #[test]
    fn round_trips() {
        let msg = sample();
        assert_eq!(Message::decode(&msg.encode()).unwrap(), msg);
    }

    #[test]
    fn round_trips_with_attachments() {
        let mut msg = sample();
        msg.attachments = vec![
            Attachment { name: "reading.csv".into(), data: b"1,2,3\r\n4,5,6\r\n".to_vec() },
            // Deliberately not text, and containing bytes that would break any
            // line-oriented reader.
            Attachment { name: "photo.jpg".into(), data: vec![0xff, 0xd8, 0x00, 0x0a, 0x0d, 0x1a] },
        ];
        let decoded = Message::decode(&msg.encode()).unwrap();
        assert_eq!(decoded, msg);
        assert_eq!(decoded.attachments[1].data, vec![0xff, 0xd8, 0x00, 0x0a, 0x0d, 0x1a]);
    }

    #[test]
    fn body_length_is_bytes_not_chars() {
        // The count is bytes. A multi-byte character in the body is exactly how
        // an implementation that counts `chars()` desynchronises every
        // attachment after it.
        let mut msg = sample();
        msg.body = "grüße… 🛰".into();
        msg.attachments = vec![Attachment { name: "a.bin".into(), data: vec![7, 7, 7] }];

        let encoded = msg.encode();
        let head = String::from_utf8_lossy(&encoded);
        assert!(head.contains(&format!("Body: {}\r", msg.body.len())));
        assert!(msg.body.len() > msg.body.chars().count());

        let decoded = Message::decode(&encoded).unwrap();
        assert_eq!(decoded.body, msg.body);
        assert_eq!(decoded.attachments[0].data, vec![7, 7, 7]);
    }

    #[test]
    fn accepts_bare_lf_separator() {
        let raw = b"Mid: ABC\nBody: 5\n\nhello".to_vec();
        let msg = Message::decode(&raw).unwrap();
        assert_eq!(msg.mid, "ABC");
        assert_eq!(msg.body, "hello");
    }

    #[test]
    fn header_matching_is_case_insensitive() {
        let raw = b"MID: ABC\r\nsubject: Hi\r\nBODY: 2\r\n\r\nhi".to_vec();
        let msg = Message::decode(&raw).unwrap();
        assert_eq!(msg.mid, "ABC");
        assert_eq!(msg.subject, "Hi");
    }

    #[test]
    fn rejects_truncated_body() {
        let raw = b"Mid: ABC\r\nBody: 99\r\n\r\nshort".to_vec();
        assert!(matches!(Message::decode(&raw), Err(MessageError::Truncated { what: "body", .. })));
    }

    #[test]
    fn rejects_missing_body_header() {
        let raw = b"Mid: ABC\r\nSubject: Hi\r\n\r\n".to_vec();
        assert_eq!(Message::decode(&raw), Err(MessageError::MissingHeader("Body")));
    }

    #[test]
    fn file_names_may_contain_spaces() {
        let mut msg = sample();
        msg.attachments =
            vec![Attachment { name: "field report.txt".into(), data: b"ok".to_vec() }];
        let decoded = Message::decode(&msg.encode()).unwrap();
        assert_eq!(decoded.attachments[0].name, "field report.txt");
    }

    #[test]
    fn mid_is_twelve_base32_chars() {
        let mid = generate_mid("OE1XYZ", 1_755_160_000_000_000_000);
        assert_eq!(mid.len(), MAX_MID_LEN);
        assert!(mid.bytes().all(|b| b.is_ascii_uppercase() || (b'2'..=b'7').contains(&b)));
    }

    #[test]
    fn mid_varies_with_time_and_callsign() {
        let a = generate_mid("OE1XYZ", 1);
        assert_ne!(a, generate_mid("OE1XYZ", 2));
        assert_ne!(a, generate_mid("OE3ABC", 1));
    }

    #[test]
    fn date_format_is_winlinks() {
        // 2026-08-14 09:30:45 UTC. Seconds are dropped, not rounded.
        assert_eq!(format_date(1_786_699_845), "2026/08/14 09:30");
        // A pre-2000 date, to catch a two-digit year slip.
        assert_eq!(format_date(0), "1970/01/01 00:00");
    }
}
