//! The B2F (FBB binary forwarding, protocol level 2) wire format.
//!
//! A forwarding session is line-oriented ASCII with one binary interlude:
//!
//! 1. Both ends announce themselves with a **SID** — `[Name-Version-CODES]` —
//!    and the codes say which protocol levels they speak. We require `B2`.
//! 2. The sender offers a **proposal block**: one `FC` line per message, then
//!    `F>` with a checksum over the lines just sent.
//! 3. The receiver answers `FS` with one character per proposal, in order:
//!    `+` accept, `-` reject, `=` defer.
//! 4. Each accepted message is sent as **framed binary** — `SOH` header,
//!    `STX` data chunks, `EOT` and a checksum.
//! 5. `FF` (nothing more to send) or `FQ` (quit) ends the turn.
//!
//! Both checksums here are the same one-byte construction: sum the bytes, then
//! negate — so the receiver's running total over the data *and* the checksum
//! byte ends at zero. That is the property [`verify_frame_checksum`] tests, and
//! it is cheaper and harder to get subtly wrong than comparing two sums.

use std::fmt::Write as _;

/// Line/frame control bytes.
const CHR_NUL: u8 = 0;
const CHR_SOH: u8 = 1;
const CHR_STX: u8 = 2;
const CHR_EOT: u8 = 4;

/// Largest data chunk in one `STX` frame.
///
/// The protocol allows 255 and paclink-unix uses 250, but 125 keeps a frame
/// inside an AX.25 link running the default paclen of 128 — which costs
/// nothing on telnet and is the difference between working and not once a
/// packet transport arrives.
pub const MAX_CHUNK: usize = 125;

/// Most proposals to offer in one block.
pub const MAX_BLOCK: usize = 5;

/// The SID codes we claim: FBB compressed protocol v2, FBB basic ASCII,
/// hierarchical routing, and MID support. `$` closes the list and the protocol
/// requires it last.
pub const LOCAL_SID_CODES: &str = "B2FHM$";

/// A station identifier line, `[Name-Version-CODES]`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sid {
    pub name: String,
    pub version: String,
    pub codes: String,
}

impl Sid {
    /// Our own SID line, ready to send (without the trailing `\r`).
    pub fn local(name: &str, version: &str) -> String {
        format!("[{name}-{version}-{LOCAL_SID_CODES}]")
    }

    /// True for a line that looks like a SID at all.
    pub fn is_sid(line: &str) -> bool {
        let line = line.trim();
        line.starts_with('[') && line.ends_with(']')
    }

    pub fn parse(line: &str) -> Option<Sid> {
        let line = line.trim();
        let inner = line.strip_prefix('[')?.strip_suffix(']')?;
        // Split from the right: the application name may itself contain
        // hyphens, but the codes and version never do.
        let (rest, codes) = inner.rsplit_once('-')?;
        let (name, version) = rest.rsplit_once('-')?;
        Some(Sid { name: name.to_string(), version: version.to_string(), codes: codes.to_string() })
    }

    /// Whether the peer advertises a code. `B2` is the one that matters: a
    /// peer without it cannot do compressed v2 forwarding and there is no
    /// point continuing.
    pub fn has(&self, code: &str) -> bool {
        // `B2` must not match the `B` in a `B1`-only SID, so compare on the
        // token, not by substring — except that the codes run together with no
        // separator. Digits bind to the preceding letter.
        let mut tokens = Vec::new();
        let mut cur = String::new();
        for ch in self.codes.chars() {
            if ch.is_ascii_digit() {
                cur.push(ch);
            } else {
                if !cur.is_empty() {
                    tokens.push(std::mem::take(&mut cur));
                }
                cur.push(ch);
            }
        }
        if !cur.is_empty() {
            tokens.push(cur);
        }
        tokens.iter().any(|t| t.eq_ignore_ascii_case(code))
    }
}

/// How a peer answered one proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Answer {
    Accept,
    Reject,
    Defer,
}

impl Answer {
    pub fn from_byte(b: u8) -> Option<Answer> {
        match b {
            b'+' | b'Y' | b'y' => Some(Answer::Accept),
            b'-' | b'N' | b'n' => Some(Answer::Reject),
            b'=' | b'L' | b'l' => Some(Answer::Defer),
            _ => None,
        }
    }

    pub fn as_byte(self) -> u8 {
        match self {
            Answer::Accept => b'+',
            Answer::Reject => b'-',
            Answer::Defer => b'=',
        }
    }
}

/// One offered message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Proposal {
    /// `C` for LZHUF-compressed B2, `D` for gzip (a Pat/wl2k-go extension we
    /// accept but never offer).
    pub code: char,
    /// `EM` (encapsulated message) or `CM`.
    pub msg_type: String,
    pub mid: String,
    /// Uncompressed size.
    pub size: usize,
    /// Compressed size — what actually gets transferred.
    pub compressed_size: usize,
}

impl Proposal {
    /// The `FC` line for this proposal, without the trailing `\r`.
    pub fn line(&self) -> String {
        format!(
            "F{} {} {} {} {} 0",
            self.code, self.msg_type, self.mid, self.size, self.compressed_size
        )
    }

    /// Parse an `FC`/`FD` proposal line.
    pub fn parse(line: &str) -> Option<Proposal> {
        let line = line.trim_end_matches(['\r', '\n']);
        let bytes = line.as_bytes();
        if bytes.len() < 4 || bytes[0] != b'F' {
            return None;
        }
        let code = bytes[1] as char;
        if !matches!(code, 'C' | 'D') {
            // A, B and the basic proposals are v0/v1 and we do not speak them.
            return None;
        }

        let mut parts = line[3..].split_whitespace();
        let msg_type = parts.next()?.to_string();
        if msg_type != "EM" && msg_type != "CM" {
            return None;
        }
        let mid = parts.next()?.to_string();
        let size = parts.next()?.parse().ok()?;
        let compressed_size = parts.next()?.parse().ok()?;
        Some(Proposal { code, msg_type, mid, size, compressed_size })
    }
}

/// One byte such that the bytes summed plus this byte come to zero mod 256.
fn negated_sum(bytes: impl IntoIterator<Item = u8>) -> u8 {
    let sum = bytes.into_iter().fold(0u8, |acc, b| acc.wrapping_add(b));
    sum.wrapping_neg()
}

/// Build a whole proposal block: the `FC` lines, then the `F>` terminator
/// carrying a checksum over exactly those lines.
///
/// The checksum covers each line's bytes *including* its `\r`. Leaving the
/// carriage returns out yields a block the CMS rejects with no explanation.
pub fn proposal_block(proposals: &[Proposal]) -> Vec<u8> {
    let mut out = String::new();
    let mut sum: u8 = 0;
    for prop in proposals.iter().take(MAX_BLOCK) {
        let line = prop.line();
        for b in line.bytes().chain(std::iter::once(b'\r')) {
            sum = sum.wrapping_add(b);
        }
        let _ = write!(out, "{line}\r");
    }
    let _ = write!(out, "F> {:02X}\r", sum.wrapping_neg());
    out.into_bytes()
}

/// The `FS` answer line for a set of decisions.
pub fn answer_line(answers: &[Answer]) -> Vec<u8> {
    let mut out = b"FS ".to_vec();
    out.extend(answers.iter().map(|a| a.as_byte()));
    out.push(b'\r');
    out
}

/// Parse an `FS` line into one answer per proposal.
///
/// Offsets (`!123`, `A123`) are not supported; a proposal answered that way is
/// treated as rejected so the session stays in step rather than desynchronising
/// on a resume we cannot honour.
pub fn parse_answer_line(line: &str) -> Vec<Answer> {
    let body = line.trim().strip_prefix("FS").unwrap_or("").trim_start();
    body.bytes().filter_map(Answer::from_byte).collect()
}

/// Frame one message body for transmission: the `SOH` header naming it, the
/// `STX` data chunks, and the `EOT` checksum.
///
/// `title` must be ASCII — the field has no encoding — and `data` is the
/// already-compressed payload.
pub fn encode_frames(title: &str, offset: usize, data: &[u8]) -> Vec<u8> {
    let title: String = title.chars().filter(|c| c.is_ascii() && !c.is_control()).collect();
    let title = if title.is_empty() { "No title".to_string() } else { title };
    let offset_str = offset.to_string();

    let mut out = Vec::with_capacity(data.len() + data.len() / MAX_CHUNK * 2 + 64);
    // The length byte counts the title, the offset, and their two NULs.
    out.push(CHR_SOH);
    out.push((title.len() + offset_str.len() + 2) as u8);
    out.extend_from_slice(title.as_bytes());
    out.push(CHR_NUL);
    out.extend_from_slice(offset_str.as_bytes());
    out.push(CHR_NUL);

    for chunk in data.chunks(MAX_CHUNK) {
        out.push(CHR_STX);
        out.push(chunk.len() as u8);
        out.extend_from_slice(chunk);
    }

    out.push(CHR_EOT);
    // Over the data only — not the frame headers.
    out.push(negated_sum(data.iter().copied()));
    out
}

/// A message read off the wire.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IncomingFrame {
    pub title: String,
    pub offset: usize,
    /// Still compressed; feed to [`crate::decompress`].
    pub data: Vec<u8>,
}

/// What can go wrong reading framed data.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum FrameError {
    #[error("frame data ended early")]
    Incomplete,
    #[error("expected SOH to start a message, found {0:#04x}")]
    ExpectedSoh(u8),
    #[error("malformed SOH header")]
    BadHeader,
    #[error("unexpected frame byte {0:#04x}")]
    UnexpectedByte(u8),
    #[error("frame checksum mismatch")]
    Checksum,
}

/// Decode one framed message from `buf`, returning it and how many bytes were
/// consumed. Returns [`FrameError::Incomplete`] if `buf` does not yet hold a
/// whole message, so a caller reading from a socket can simply try again.
pub fn decode_frames(buf: &[u8]) -> Result<(IncomingFrame, usize), FrameError> {
    let mut i = 0usize;
    let first = *buf.first().ok_or(FrameError::Incomplete)?;
    if first != CHR_SOH {
        return Err(FrameError::ExpectedSoh(first));
    }
    i += 1;

    let hdr_len = *buf.get(i).ok_or(FrameError::Incomplete)? as usize;
    i += 1;
    let hdr = buf.get(i..i + hdr_len).ok_or(FrameError::Incomplete)?;
    i += hdr_len;

    // `title NUL offset NUL`
    let mut fields = hdr.split(|&b| b == CHR_NUL);
    let title = fields.next().ok_or(FrameError::BadHeader)?;
    let offset = fields.next().ok_or(FrameError::BadHeader)?;
    let title = String::from_utf8_lossy(title).into_owned();
    let offset: usize = String::from_utf8_lossy(offset).trim().parse().unwrap_or(0);

    let mut data = Vec::new();
    loop {
        match *buf.get(i).ok_or(FrameError::Incomplete)? {
            CHR_STX => {
                let len = *buf.get(i + 1).ok_or(FrameError::Incomplete)? as usize;
                let start = i + 2;
                let chunk = buf.get(start..start + len).ok_or(FrameError::Incomplete)?;
                data.extend_from_slice(chunk);
                i = start + len;
            }
            CHR_EOT => {
                let sum = *buf.get(i + 1).ok_or(FrameError::Incomplete)?;
                i += 2;
                if !verify_frame_checksum(&data, sum) {
                    return Err(FrameError::Checksum);
                }
                return Ok((IncomingFrame { title, offset, data }, i));
            }
            other => return Err(FrameError::UnexpectedByte(other)),
        }
    }
}

/// True when `data` and its trailing checksum byte sum to zero mod 256 — the
/// property the format is built around.
pub fn verify_frame_checksum(data: &[u8], checksum: u8) -> bool {
    data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b)).wrapping_add(checksum) == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_sid_is_well_formed() {
        let sid = Sid::local("sdroxide", "1.2.0");
        assert_eq!(sid, "[sdroxide-1.2.0-B2FHM$]");
        let parsed = Sid::parse(&sid).unwrap();
        assert_eq!(parsed.name, "sdroxide");
        assert_eq!(parsed.version, "1.2.0");
        assert!(parsed.has("B2"));
        // `$` must close the code list.
        assert!(sid.ends_with("$]"));
    }

    #[test]
    fn parses_a_real_cms_sid() {
        let sid = Sid::parse("[WL2K-5.0-B2FWIHJM$]").unwrap();
        assert_eq!(sid.name, "WL2K");
        assert_eq!(sid.version, "5.0");
        assert!(sid.has("B2"));
        assert!(sid.has("F"));
    }

    #[test]
    fn b2_does_not_match_a_b1_only_peer() {
        // The whole point of the token split: substring matching would see the
        // `B` of `B1` and happily start a v2 session the peer cannot follow.
        let sid = Sid::parse("[Old-1.0-B1FH$]").unwrap();
        assert!(!sid.has("B2"));
        assert!(sid.has("B1"));
    }

    fn sample_proposal() -> Proposal {
        Proposal {
            code: 'C',
            msg_type: "EM".into(),
            mid: "TJKYEIMMHSRB".into(),
            size: 527,
            compressed_size: 123,
        }
    }

    #[test]
    fn proposal_line_matches_the_documented_shape() {
        assert_eq!(sample_proposal().line(), "FC EM TJKYEIMMHSRB 527 123 0");
    }

    #[test]
    fn proposal_round_trips() {
        let prop = sample_proposal();
        assert_eq!(Proposal::parse(&prop.line()).unwrap(), prop);
        // And tolerates the trailing carriage return it arrives with.
        assert_eq!(Proposal::parse(&format!("{}\r", prop.line())).unwrap(), prop);
    }

    #[test]
    fn rejects_v0_and_v1_proposals() {
        assert!(Proposal::parse("FA EM ABC 1 1 0").is_none());
        assert!(Proposal::parse("FB EM ABC 1 1 0").is_none());
    }

    #[test]
    fn proposal_block_checksum_zeroes_out() {
        let props = [sample_proposal()];
        let block = proposal_block(&props);
        let text = String::from_utf8(block).unwrap();

        let (lines, terminator) = text.rsplit_once("F> ").unwrap();
        let declared = u8::from_str_radix(terminator.trim_end_matches('\r'), 16).unwrap();

        // The defining property: the lines' bytes plus the checksum come to
        // zero. This is what the CMS actually verifies.
        let sum = lines.bytes().fold(0u8, |a, b| a.wrapping_add(b));
        assert_eq!(sum.wrapping_add(declared), 0);
    }

    #[test]
    fn proposal_block_checksum_includes_carriage_returns() {
        // Pinned because omitting the `\r` is the natural mistake and produces
        // a block that is rejected with no diagnostic.
        let block = String::from_utf8(proposal_block(&[sample_proposal()])).unwrap();
        let declared =
            u8::from_str_radix(block.rsplit_once("F> ").unwrap().1.trim_end_matches('\r'), 16)
                .unwrap();
        let without_cr = sample_proposal().line().bytes().fold(0u8, |a, b| a.wrapping_add(b));
        assert_ne!(without_cr.wrapping_neg(), declared);
    }

    #[test]
    fn block_is_capped() {
        let props: Vec<_> = (0..10).map(|_| sample_proposal()).collect();
        let text = String::from_utf8(proposal_block(&props)).unwrap();
        assert_eq!(text.matches("FC EM").count(), MAX_BLOCK);
    }

    #[test]
    fn answers_round_trip() {
        let answers = [Answer::Accept, Answer::Reject, Answer::Defer];
        let line = String::from_utf8(answer_line(&answers)).unwrap();
        assert_eq!(line, "FS +-=\r");
        assert_eq!(parse_answer_line(&line), answers);
    }

    #[test]
    fn accepts_alternate_answer_letters() {
        // FBB v1 peers use letters rather than punctuation.
        assert_eq!(parse_answer_line("FS YNL\r"), [Answer::Accept, Answer::Reject, Answer::Defer]);
    }

    #[test]
    fn frames_round_trip() {
        let data: Vec<u8> = (0..1000u32).map(|i| (i % 251) as u8).collect();
        let framed = encode_frames("Test message", 0, &data);
        let (got, used) = decode_frames(&framed).unwrap();
        assert_eq!(used, framed.len());
        assert_eq!(got.title, "Test message");
        assert_eq!(got.offset, 0);
        assert_eq!(got.data, data);
    }

    #[test]
    fn frames_chunk_at_the_paclen_limit() {
        let data = vec![0u8; MAX_CHUNK * 2 + 7];
        let framed = encode_frames("t", 0, &data);
        assert_eq!(framed.iter().filter(|&&b| b == CHR_STX).count(), 3);
        assert_eq!(decode_frames(&framed).unwrap().0.data, data);
    }

    #[test]
    fn frame_checksum_property_holds() {
        let data = b"the quick brown fox";
        let framed = encode_frames("t", 0, data);
        let checksum = *framed.last().unwrap();
        assert!(verify_frame_checksum(data, checksum));
        assert!(!verify_frame_checksum(data, checksum.wrapping_add(1)));
    }

    #[test]
    fn detects_corrupt_frame_data() {
        let mut framed = encode_frames("t", 0, b"hello world");
        // Flip a payload byte, past the SOH header.
        let pos = framed.len() - 4;
        framed[pos] ^= 0xff;
        assert_eq!(decode_frames(&framed), Err(FrameError::Checksum));
    }

    #[test]
    fn partial_frames_are_incomplete_not_corrupt() {
        // A socket reader hands over whatever has arrived; every prefix must
        // say "not yet" rather than "broken", or messages get dropped on a slow
        // link.
        let framed = encode_frames("Test", 0, &vec![7u8; 300]);
        for cut in 1..framed.len() {
            assert_eq!(
                decode_frames(&framed[..cut]),
                Err(FrameError::Incomplete),
                "prefix of {cut} bytes should be incomplete"
            );
        }
        assert!(decode_frames(&framed).is_ok());
    }

    #[test]
    fn empty_title_becomes_a_placeholder() {
        let framed = encode_frames("", 0, b"x");
        assert_eq!(decode_frames(&framed).unwrap().0.title, "No title");
    }

    #[test]
    fn non_ascii_titles_are_stripped() {
        // The title field has no encoding, so anything outside ASCII would
        // corrupt the length byte's agreement with the bytes that follow.
        let framed = encode_frames("grüße 🛰", 0, b"x");
        let (got, _) = decode_frames(&framed).unwrap();
        assert!(got.title.is_ascii(), "title {:?} should be ASCII", got.title);
    }
}
