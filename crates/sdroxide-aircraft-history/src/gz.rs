//! The history file's bytes: plain JSON while it is small, gzip once it is not.
//!
//! One file name either way. [`decode`] tells the two apart by the gzip magic,
//! so there is never a second file to disagree with the first, and a file from
//! before this existed is read as it always was. Pure: bytes in, bytes out.

use std::io::{Read, Write};

use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;

/// Above this many bytes of JSON the file is written gzipped. A day of busy sky
/// is repetitive text that gzip takes to a tenth, so the active file stays
/// small to read at start-up and light on an SD card.
pub const COMPRESS_ABOVE_BYTES: usize = 5 * 1024 * 1024;

const MAGIC: [u8; 2] = [0x1f, 0x8b];

/// The bytes to write for `text`: gzip when it is longer than `above`, the text
/// itself otherwise.
pub fn encode(text: &str, above: usize) -> Vec<u8> {
    if text.len() <= above {
        return text.as_bytes().to_vec();
    }
    let mut gz = GzEncoder::new(Vec::new(), Compression::default());
    // Into memory: neither step can fail, and if one did the plain text is the
    // safe answer.
    match gz.write_all(text.as_bytes()).and_then(|()| gz.finish()) {
        Ok(bytes) => bytes,
        Err(_) => text.as_bytes().to_vec(),
    }
}

/// The JSON text in a file's bytes, gzipped or plain. Nothing when it is
/// neither: a damaged file starts an empty history, as any unreadable one does.
pub fn decode(bytes: &[u8]) -> Option<String> {
    if bytes.starts_with(&MAGIC) {
        let mut text = String::new();
        GzDecoder::new(bytes).read_to_string(&mut text).ok()?;
        Some(text)
    } else {
        String::from_utf8(bytes.to_vec()).ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Fix, History};

    const NOW: i64 = 1_000_000;

    /// A history of `n` aircraft, the way a busy day writes it.
    fn big_json(n: u32) -> String {
        let mut h = History::default();
        for i in 0..n {
            let fix = Fix {
                at: NOW,
                lat: 51.0 + f64::from(i) * 1e-3,
                lon: 0.5,
                altitude_ft: Some(30_000),
                on_ground: false,
            };
            h.note_heard(NOW, 0x400000 + i, Some(&format!("BAW{i}")), None, Some(fix));
        }
        h.to_json()
    }

    #[test]
    fn a_small_file_stays_plain_text() {
        let t = "{\"v\":1,\"journals\":[]}";
        assert_eq!(encode(t, COMPRESS_ABOVE_BYTES), t.as_bytes());
    }

    #[test]
    fn a_file_over_the_threshold_is_gzipped_and_much_smaller() {
        let text = big_json(2000);
        let bytes = encode(&text, 1000);
        assert_eq!(bytes[..2], MAGIC, "gzip magic");
        assert!(bytes.len() * 3 < text.len(), "{} -> {}", text.len(), bytes.len());
    }

    #[test]
    fn the_threshold_is_exclusive() {
        let text = "x".repeat(100);
        assert_eq!(encode(&text, 100), text.as_bytes(), "exactly at it: plain");
        assert_eq!(encode(&text, 99)[..2], MAGIC, "one over: gzip");
    }

    #[test]
    fn gzipped_and_plain_both_decode_to_the_same_text() {
        let text = big_json(300);
        assert_eq!(decode(&encode(&text, 10)).as_deref(), Some(text.as_str()));
        assert_eq!(decode(&encode(&text, usize::MAX)).as_deref(), Some(text.as_str()));
    }

    #[test]
    fn a_big_history_comes_back_whole_through_the_file_bytes() {
        let text = big_json(2000);
        let back = decode(&encode(&text, 1000)).unwrap();
        let h = History::from_json(&back, NOW + 10);
        assert_eq!(h.len(), 2000);
        assert_eq!(h.journal(0x400000).unwrap().callsign(), Some("BAW0"));
    }

    #[test]
    fn a_damaged_gzip_or_binary_junk_is_nothing_and_not_a_panic() {
        let mut bytes = encode(&big_json(300), 10);
        bytes.truncate(bytes.len() / 2);
        assert_eq!(decode(&bytes), None, "truncated gzip");
        assert_eq!(decode(&[0xff, 0xfe, 0x00, 0x80]), None, "not text");
        assert_eq!(decode(&MAGIC), None, "magic and nothing else");
        assert_eq!(decode(&[]), Some(String::new()), "an empty file is an empty text");
    }

    #[test]
    fn the_threshold_is_five_megabytes() {
        assert_eq!(COMPRESS_ABOVE_BYTES, 5 * 1024 * 1024);
    }

    #[test]
    fn this_module_touches_no_file_and_no_socket() {
        let src = include_str!("gz.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap()];
        for banned in ["std::fs", "File", "TcpStream", "ureq", "SystemTime"] {
            assert!(!code.contains(banned), "gz.rs must not touch `{banned}`");
        }
        assert!(code.contains("GzEncoder"), "the control: the scan sees what is there");
    }
}
