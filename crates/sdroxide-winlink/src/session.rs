//! A B2F forwarding session, from the client side.
//!
//! One session is one complete exchange: shake hands, take whatever the peer
//! offers, offer whatever we have, hang up. It is deliberately synchronous and
//! transport-agnostic — it drives anything that is `Read + Write`, which is
//! what lets the tests run a whole forwarding exchange against an in-memory
//! fake CMS with no socket involved.
//!
//! The client is the *slave* in B2F terms, which splits the session in two:
//! the CMS opens the handshake and we answer it, but once that is done the
//! turn order inverts and **we** propose first. Getting that second half
//! backwards deadlocks against the real server while every self-consistent
//! test fixture still passes — see [`run_into`].

use std::io::{Read, Write};

use crate::b2f::{self, Answer, Proposal, Sid};
use crate::message::Message;
use crate::secure;

/// Who we are and how to authenticate.
pub struct SessionConfig {
    pub callsign: String,
    /// The Winlink *account* password, for the `;PQ:` challenge. Not the
    /// gateway password that opened the socket.
    pub password: String,
    /// Maidenhead locator, reported in the greeting line. Cosmetic.
    pub locator: String,
    pub app_name: String,
    pub app_version: String,
    /// Who we are forwarding with, for the greeting line.
    ///
    /// `wl2k` for the CMS, the gateway's callsign over the air. Carried here
    /// rather than read from [`crate::transport::Transport::target_call`] so
    /// that [`run_into`] stays generic over `Read + Write` — the in-memory
    /// `FakeCms` the tests drive is not a `Transport` and should not have to
    /// become one to exercise a greeting line.
    pub target_call: String,
}

impl Default for SessionConfig {
    fn default() -> Self {
        SessionConfig {
            callsign: String::new(),
            password: String::new(),
            locator: String::new(),
            app_name: String::new(),
            app_version: String::new(),
            target_call: crate::transport::CMS_TARGET_CALL.to_string(),
        }
    }
}

/// What a session did.
#[derive(Debug, Default)]
pub struct SessionOutcome {
    pub received: Vec<Message>,
    /// MIDs the peer accepted from us.
    pub sent: Vec<String>,
    /// MIDs the peer declined — already held, usually.
    pub rejected: Vec<String>,
    /// The protocol transcript, for the operator to look at when something
    /// goes wrong. Contains no secrets: the only credential-derived value on
    /// the wire is the one-time challenge response.
    pub log: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum SessionError {
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("peer sent no SID, so it is not speaking B2F")]
    NoSid,
    #[error("peer does not support B2 compressed forwarding (SID codes {0:?})")]
    NoB2(String),
    #[error("peer sent a proposal block with a bad checksum")]
    BadProposalChecksum,
    #[error("peer disconnected mid-session")]
    Disconnected,
    #[error("decompressing {mid}: {source}")]
    Decompress {
        mid: String,
        #[source]
        source: crate::LzhufError,
    },
    #[error("parsing {mid}: {source}")]
    Parse {
        mid: String,
        #[source]
        source: crate::message::MessageError,
    },
}

/// A buffered reader that can switch between `\r`-terminated lines and the raw
/// binary frames the same stream carries.
///
/// A plain `BufReader` cannot do this: once it has buffered past the end of a
/// line the frame bytes are inside it, so the buffer has to be ours.
struct Wire<'a, T: Read + Write> {
    io: &'a mut T,
    buf: Vec<u8>,
}

impl<'a, T: Read + Write> Wire<'a, T> {
    fn new(io: &'a mut T) -> Self {
        Wire { io, buf: Vec::with_capacity(4096) }
    }

    /// Pull more bytes in. `false` at clean end of stream.
    fn fill(&mut self) -> Result<bool, SessionError> {
        let mut chunk = [0u8; 4096];
        let n = self.io.read(&mut chunk)?;
        if n == 0 {
            return Ok(false);
        }
        self.buf.extend_from_slice(&chunk[..n]);
        Ok(true)
    }

    /// Read one line, without its terminator. Blank lines are skipped: the CMS
    /// pads with them and they carry no meaning.
    fn read_line(&mut self) -> Result<String, SessionError> {
        loop {
            if let Some(pos) = self.buf.iter().position(|&b| b == b'\r' || b == b'\n') {
                let line: Vec<u8> = self.buf.drain(..=pos).collect();
                let line = String::from_utf8_lossy(&line[..pos]).trim().to_string();
                if line.is_empty() {
                    continue;
                }
                return Ok(line);
            }
            if !self.fill()? {
                return Err(SessionError::Disconnected);
            }
        }
    }

    /// Read one framed message, filling from the transport until it is whole.
    fn read_frame(&mut self) -> Result<b2f::IncomingFrame, SessionError> {
        loop {
            match b2f::decode_frames(&self.buf) {
                Ok((frame, used)) => {
                    self.buf.drain(..used);
                    return Ok(frame);
                }
                Err(b2f::FrameError::Incomplete) => {
                    if !self.fill()? {
                        return Err(SessionError::Disconnected);
                    }
                }
                Err(e) => {
                    return Err(SessionError::Io(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        e.to_string(),
                    )));
                }
            }
        }
    }

    fn send(&mut self, bytes: &[u8]) -> Result<(), SessionError> {
        self.io.write_all(bytes)?;
        self.io.flush()?;
        Ok(())
    }
}

/// Run one complete forwarding exchange, as [`run_into`], allocating the
/// outcome. Convenient when a failure's transcript is not wanted.
pub fn run<T: Read + Write>(
    io: &mut T,
    cfg: &SessionConfig,
    outbound: &[Message],
) -> Result<SessionOutcome, SessionError> {
    let mut out = SessionOutcome::default();
    run_into(io, cfg, outbound, &mut out)?;
    Ok(out)
}

/// Run one complete forwarding exchange, recording progress into `out`.
///
/// `outbound` is offered to the peer; whatever it offers us lands in
/// [`SessionOutcome::received`]. The caller decides what to do with either —
/// this function owns no storage.
///
/// The outcome is filled as the session proceeds rather than returned at the
/// end, so that **a failed session still leaves its transcript behind**. A
/// forwarding session that dies halfway is exactly when the operator needs to
/// see what was said, and returning it only on success would throw it away at
/// the one moment it matters.
pub fn run_into<T: Read + Write>(
    io: &mut T,
    cfg: &SessionConfig,
    outbound: &[Message],
    out: &mut SessionOutcome,
) -> Result<(), SessionError> {
    let mut wire = Wire::new(io);

    let challenge = handshake(&mut wire, cfg, out)?;
    send_our_handshake(&mut wire, cfg, challenge.as_deref(), out)?;

    // Compress once, up front: the proposal has to declare the compressed size
    // before the peer will accept, so it cannot be deferred to send time.
    let mut pending: Vec<(Message, Vec<u8>)> =
        outbound.iter().map(|m| (m.clone(), crate::compress(&m.encode()))).collect();

    // **The client speaks first.** We are the slave in B2F terms, and the turn
    // alternates from `!master` — so after the handshake it is our move, not
    // the CMS's. Waiting for the peer here deadlocks until the read timeout,
    // with a transcript that looks like a healthy handshake and then nothing.
    let mut my_turn = true;
    let mut quit_sent = false;
    let mut quit_received = false;
    // Set once the peer says FF. It is what turns our next empty turn into FQ
    // rather than a second FF, which would loop forever.
    let mut remote_no_msgs = false;

    while !quit_sent && !quit_received {
        if my_turn {
            quit_sent = handle_outbound(&mut wire, &mut pending, remote_no_msgs, out)?;
        } else {
            match handle_inbound(&mut wire, out) {
                Ok(Inbound::NoMore) => remote_no_msgs = true,
                // They had mail for us; they have not said they are done, so
                // the session goes round again rather than hanging up on them.
                Ok(Inbound::Received) => remote_no_msgs = false,
                Ok(Inbound::Quit) => quit_received = true,
                // The CMS routinely drops the socket rather than saying FQ.
                // That is a normal ending, not a failure.
                Err(SessionError::Disconnected) => break,
                Err(e) => return Err(e),
            }
        }
        my_turn = !my_turn;
    }

    Ok(())
}

/// Our turn: offer what we have, or declare that we have nothing.
///
/// Returns whether we sent `FQ`.
fn handle_outbound<T: Read + Write>(
    wire: &mut Wire<'_, T>,
    pending: &mut Vec<(Message, Vec<u8>)>,
    remote_no_msgs: bool,
    out: &mut SessionOutcome,
) -> Result<bool, SessionError> {
    if pending.is_empty() {
        // Nothing to offer. If the peer has also finished, close the session;
        // otherwise say so and let it have another turn.
        let resp = if remote_no_msgs { "FQ" } else { "FF" };
        send_line(wire, resp, out)?;
        return Ok(remote_no_msgs);
    }

    send_outbound(wire, pending, out)?;
    Ok(false)
}

/// What ended the peer's turn.
enum Inbound {
    /// `FF`, or an empty proposal block — nothing more from them this session.
    NoMore,
    /// `FQ` — they are hanging up.
    Quit,
    /// They proposed messages and we took them. Their turn is over, but they
    /// have not said they are finished, so the session continues.
    Received,
}

/// Their turn: accumulate a proposal block, accept it, and read the messages.
///
/// **A proposal block ends the peer's turn.** After the `FS` answer and the
/// messages it accepted, the peer says nothing more — no `FF` follows. Waiting
/// for one blocks until the read timeout and reports a failed session even
/// though the mail arrived, which is precisely what the live CMS does and what
/// a fixture scripted with a trailing `FF` will never show.
fn handle_inbound<T: Read + Write>(
    wire: &mut Wire<'_, T>,
    out: &mut SessionOutcome,
) -> Result<Inbound, SessionError> {
    let mut proposals: Vec<Proposal> = Vec::new();
    let mut checksum_input: Vec<u8> = Vec::new();

    loop {
        let line = wire.read_line()?;
        out.log.push(format!("< {line}"));

        if line.starts_with("FQ") {
            return Ok(Inbound::Quit);
        }
        if line.starts_with("FF") {
            return Ok(Inbound::NoMore);
        }
        if let Some(prop) = Proposal::parse(&line) {
            // The checksum covers each proposal line's bytes and its `\r`.
            checksum_input.extend_from_slice(line.as_bytes());
            checksum_input.push(b'\r');
            proposals.push(prop);
            continue;
        }
        if let Some(rest) = line.strip_prefix("F>") {
            let declared = u8::from_str_radix(rest.trim(), 16).unwrap_or(0);
            let sum = checksum_input.iter().fold(0u8, |a, &b| a.wrapping_add(b));
            if sum.wrapping_add(declared) != 0 {
                return Err(SessionError::BadProposalChecksum);
            }
            // An empty block is how a peer says it has nothing, in place of FF.
            if proposals.is_empty() {
                return Ok(Inbound::NoMore);
            }
            receive_messages(wire, &proposals, out)?;
            return Ok(Inbound::Received);
        }
        // `;PM`, MOTD and anything else is informational; the log has it.
    }
}

/// Read the peer's opening: SID, optional secure challenge, prompt.
fn handshake<T: Read + Write>(
    wire: &mut Wire<'_, T>,
    _cfg: &SessionConfig,
    out: &mut SessionOutcome,
) -> Result<Option<String>, SessionError> {
    let mut challenge = None;
    let mut sid: Option<Sid> = None;

    loop {
        let line = wire.read_line()?;
        out.log.push(format!("< {line}"));

        if Sid::is_sid(&line) {
            let parsed = Sid::parse(&line).ok_or(SessionError::NoSid)?;
            if !parsed.has("B2") {
                return Err(SessionError::NoB2(parsed.codes));
            }
            sid = Some(parsed);
        } else if let Some(rest) = line.strip_prefix(";PQ:") {
            challenge = Some(rest.trim().to_string());
        } else if line.ends_with('>') {
            // The prompt ends the peer's opening turn.
            break;
        }
    }

    if sid.is_none() {
        return Err(SessionError::NoSid);
    }
    Ok(challenge)
}

fn send_our_handshake<T: Read + Write>(
    wire: &mut Wire<'_, T>,
    cfg: &SessionConfig,
    challenge: Option<&str>,
    out: &mut SessionOutcome,
) -> Result<(), SessionError> {
    send_line(wire, &format!(";FW: {}", cfg.callsign), out)?;
    send_line(wire, &Sid::local(&cfg.app_name, &cfg.app_version), out)?;

    if let Some(challenge) = challenge {
        let response = secure::login_response(challenge, &cfg.password);
        send_line(wire, &format!(";PR: {response}"), out)?;
    }

    send_line(wire, &format!("; {} DE {} ({})", cfg.target_call, cfg.callsign, cfg.locator), out)?;
    Ok(())
}

/// Accept a parsed proposal block and read the messages that follow.
fn receive_messages<T: Read + Write>(
    wire: &mut Wire<'_, T>,
    proposals: &[Proposal],
    out: &mut SessionOutcome,
) -> Result<(), SessionError> {
    // Accept everything. A client with a full mailbox might defer; ours has
    // nowhere to put a deferral, and taking the message is what the operator
    // connected for.
    let answers: Vec<Answer> = proposals.iter().map(|_| Answer::Accept).collect();
    let line = String::from_utf8_lossy(&b2f::answer_line(&answers)).trim_end().to_string();
    send_line(wire, &line, out)?;

    for prop in proposals {
        let frame = wire.read_frame()?;
        out.log.push(format!("< [{} bytes] {}", frame.data.len(), frame.title));

        let raw = crate::decompress(&frame.data)
            .map_err(|source| SessionError::Decompress { mid: prop.mid.clone(), source })?;
        let msg = Message::decode(&raw)
            .map_err(|source| SessionError::Parse { mid: prop.mid.clone(), source })?;
        out.received.push(msg);
    }

    Ok(())
}

/// Offer one block of messages and send the ones accepted, draining them from
/// `pending` so the next turn either offers the remainder or declares us done.
///
/// No `FF` is sent here: the block *is* the turn, and the peer answers next.
fn send_outbound<T: Read + Write>(
    wire: &mut Wire<'_, T>,
    pending: &mut Vec<(Message, Vec<u8>)>,
    out: &mut SessionOutcome,
) -> Result<(), SessionError> {
    let take = pending.len().min(b2f::MAX_BLOCK);
    let batch: Vec<(Message, Vec<u8>)> = pending.drain(..take).collect();
    let proposals: Vec<Proposal> = batch
        .iter()
        .map(|(msg, packed)| Proposal {
            code: 'C',
            msg_type: "EM".into(),
            mid: msg.mid.clone(),
            size: msg.encode().len(),
            compressed_size: packed.len(),
        })
        .collect();

    let block = b2f::proposal_block(&proposals);
    for line in String::from_utf8_lossy(&block).split('\r').filter(|l| !l.is_empty()) {
        out.log.push(format!("> {line}"));
    }
    wire.send(&block)?;

    let reply = wire.read_line()?;
    out.log.push(format!("< {reply}"));
    let answers = b2f::parse_answer_line(&reply);

    for (i, (msg, packed)) in batch.iter().enumerate() {
        match answers.get(i) {
            Some(Answer::Accept) => {
                wire.send(&b2f::encode_frames(&msg.title(), 0, packed))?;
                out.log.push(format!("> [{} bytes] {}", packed.len(), msg.title()));
                out.sent.push(msg.mid.clone());
            }
            _ => out.rejected.push(msg.mid.clone()),
        }
    }

    Ok(())
}

fn send_line<T: Read + Write>(
    wire: &mut Wire<'_, T>,
    line: &str,
    out: &mut SessionOutcome,
) -> Result<(), SessionError> {
    out.log.push(format!("> {line}"));
    wire.send(format!("{line}\r").as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Attachment;

    /// A scripted peer: a canned byte stream to read, and a record of what we
    /// wrote. Enough to run a whole forwarding exchange with no socket.
    struct FakeCms {
        to_client: std::io::Cursor<Vec<u8>>,
        from_client: Vec<u8>,
    }

    impl Read for FakeCms {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            self.to_client.read(buf)
        }
    }
    impl Write for FakeCms {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.from_client.extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    fn config() -> SessionConfig {
        SessionConfig {
            callsign: "OE3JJS".into(),
            password: "TESTPASS".into(),
            locator: "JN88".into(),
            app_name: "sdroxide".into(),
            app_version: "1.2.0".into(),
            ..Default::default()
        }
    }

    fn inbound_message() -> Message {
        Message {
            mid: "ABCDEFGHIJKL".into(),
            date: "2026/08/14 09:30".into(),
            msg_type: "Private".into(),
            from: "OE1XYZ".into(),
            to: vec!["OE3JJS".into()],
            cc: vec![],
            subject: "Hello from the CMS".into(),
            mbo: "OE1XYZ".into(),
            body: "Body of the test message.\r\n".into(),
            attachments: vec![Attachment { name: "note.txt".into(), data: b"attached".to_vec() }],
        }
    }

    /// Build what a CMS sends across a whole session.
    ///
    /// **Turn order matters here.** The client speaks first after the
    /// handshake, so everything after the prompt is the CMS's *second* turn —
    /// it arrives only once we have sent our `FF` or our proposals. An earlier
    /// version of this script had the CMS proposing immediately, which matched
    /// the implementation's mistake and hid a deadlock that only showed up
    /// against the live server.
    fn scripted_cms(msgs: &[Message]) -> Vec<u8> {
        let mut s = Vec::new();
        s.extend_from_slice(b"[WL2K-5.0-B2FWIHJM$]\r");
        s.extend_from_slice(b";PQ: 23753528\r");
        s.extend_from_slice(b"Winlink CMS >\r");

        if msgs.is_empty() {
            s.extend_from_slice(b"FF\r");
        } else {
            let packed: Vec<Vec<u8>> = msgs.iter().map(|m| crate::compress(&m.encode())).collect();
            let proposals: Vec<Proposal> = msgs
                .iter()
                .zip(&packed)
                .map(|(m, p)| Proposal {
                    code: 'C',
                    msg_type: "EM".into(),
                    mid: m.mid.clone(),
                    size: m.encode().len(),
                    compressed_size: p.len(),
                })
                .collect();
            s.extend_from_slice(&b2f::proposal_block(&proposals));
            for (m, p) in msgs.iter().zip(&packed) {
                s.extend_from_slice(&b2f::encode_frames(&m.title(), 0, p));
            }
            // **No FF here.** A proposal block plus its messages is the whole
            // of the peer's turn; the real CMS says nothing more. An earlier
            // version of this script appended one, which let the client sit
            // waiting for it and still pass — the live session then failed on
            // a read timeout after the mail had already arrived. The FF below
            // belongs to the peer's *next* turn.
            s.extend_from_slice(b"FF\r");
        }
        s
    }

    #[test]
    fn receives_a_message_end_to_end() {
        let want = inbound_message();
        let mut cms = FakeCms {
            to_client: std::io::Cursor::new(scripted_cms(std::slice::from_ref(&want))),
            from_client: Vec::new(),
        };

        let outcome = run(&mut cms, &config(), &[]).expect("session should complete");

        assert_eq!(outcome.received.len(), 1);
        assert_eq!(outcome.received[0], want);
        assert_eq!(outcome.received[0].attachments[0].data, b"attached");

        let sent = String::from_utf8_lossy(&cms.from_client);
        assert!(sent.contains(";FW: OE3JJS\r"), "must request mail for the callsign");
        assert!(sent.contains("[sdroxide-1.2.0-B2FHM$]\r"), "must send our SID");

        // The exact turn sequence, not just "an FS appeared somewhere": we
        // offer nothing, take their block, offer nothing again, and quit once
        // they say they are done. Pinned because waiting for a phantom FF
        // after the messages is invisible to any looser assertion.
        let tail = &sent[sent.find("(JN88)").expect("greeting") + 6..];
        assert_eq!(tail, "\rFF\rFS +\rFF\rFQ\r", "unexpected turn sequence: {tail:?}");
    }

    #[test]
    fn answers_the_secure_challenge() {
        let mut cms =
            FakeCms { to_client: std::io::Cursor::new(scripted_cms(&[])), from_client: Vec::new() };
        run(&mut cms, &config(), &[]).unwrap();

        let sent = String::from_utf8_lossy(&cms.from_client);
        // The challenge in the script is the one from the published vectors, so
        // this pins the whole path from challenge line to response line.
        let want = secure::login_response("23753528", "TESTPASS");
        assert!(sent.contains(&format!(";PR: {want}\r")), "sent: {sent:?}");
    }

    #[test]
    fn never_writes_the_password_to_the_wire_or_log() {
        let mut cms =
            FakeCms { to_client: std::io::Cursor::new(scripted_cms(&[])), from_client: Vec::new() };
        let outcome = run(&mut cms, &config(), &[]).unwrap();

        let sent = String::from_utf8_lossy(&cms.from_client);
        assert!(!sent.contains("TESTPASS"), "password must never reach the wire");
        for line in &outcome.log {
            assert!(!line.contains("TESTPASS"), "password leaked into the log: {line}");
        }
    }

    #[test]
    fn client_speaks_first() {
        // With nothing to send and a CMS with nothing to offer, the exchange is
        // exactly: we say FF, they say FF, we say FQ. Pinned because getting
        // this backwards deadlocks against the real server while every
        // self-consistent fixture still passes.
        let mut cms =
            FakeCms { to_client: std::io::Cursor::new(scripted_cms(&[])), from_client: Vec::new() };
        run(&mut cms, &config(), &[]).unwrap();

        let sent = String::from_utf8_lossy(&cms.from_client);
        let tail = &sent[sent.find("(JN88)").expect("greeting") + 6..];
        assert_eq!(tail, "\rFF\rFQ\r", "unexpected turn order: {tail:?}");
    }

    #[test]
    fn sends_an_outbound_message() {
        // We propose first; the CMS accepts, then reports nothing of its own.
        let mut script = Vec::new();
        script.extend_from_slice(b"[WL2K-5.0-B2FWIHJM$]\r");
        script.extend_from_slice(b"Winlink CMS >\r");
        script.extend_from_slice(b"FS +\r");
        script.extend_from_slice(b"FF\r");

        let mut cms = FakeCms { to_client: std::io::Cursor::new(script), from_client: Vec::new() };

        let mut msg = inbound_message();
        msg.from = "OE3JJS".into();
        msg.to = vec!["SMTP:someone@example.org".into()];

        let outcome = run(&mut cms, &config(), std::slice::from_ref(&msg)).unwrap();
        assert_eq!(outcome.sent, vec![msg.mid.clone()]);

        // The proposal must declare the compressed size, and the bytes that
        // follow must decompress back to the message we handed in.
        let sent = cms.from_client.clone();
        let text = String::from_utf8_lossy(&sent);
        assert!(text.contains(&format!("FC EM {} ", msg.mid)), "sent: {text:?}");

        let soh = sent.iter().position(|&b| b == 1).expect("a framed message");
        let (frame, _) = b2f::decode_frames(&sent[soh..]).expect("frame should decode");
        let raw = crate::decompress(&frame.data).expect("payload should decompress");
        assert_eq!(Message::decode(&raw).unwrap(), msg);
    }

    #[test]
    fn rejects_a_peer_without_b2() {
        let mut cms = FakeCms {
            to_client: std::io::Cursor::new(b"[Old-1.0-B1FH$]\rprompt >\r".to_vec()),
            from_client: Vec::new(),
        };
        assert!(matches!(run(&mut cms, &config(), &[]), Err(SessionError::NoB2(_))));
    }

    #[test]
    fn rejects_a_corrupt_proposal_block() {
        let mut script = Vec::new();
        script.extend_from_slice(b"[WL2K-5.0-B2FWIHJM$]\r");
        script.extend_from_slice(b"Winlink CMS >\r");
        script.extend_from_slice(b"FC EM ABCDEFGHIJKL 100 50 0\r");
        script.extend_from_slice(b"F> 00\r"); // deliberately wrong
        let mut cms = FakeCms { to_client: std::io::Cursor::new(script), from_client: Vec::new() };
        assert!(matches!(run(&mut cms, &config(), &[]), Err(SessionError::BadProposalChecksum)));
    }

    #[test]
    fn survives_an_abrupt_disconnect_after_quit() {
        // The CMS routinely drops the socket right after FQ. That is a normal
        // ending, not an error.
        let mut script = scripted_cms(&[]);
        script.extend_from_slice(b"FQ\r");
        let mut cms = FakeCms { to_client: std::io::Cursor::new(script), from_client: Vec::new() };
        assert!(run(&mut cms, &config(), &[]).is_ok());
    }
}
