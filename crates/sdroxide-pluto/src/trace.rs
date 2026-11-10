//! A self-contained session trace, so a user with a Pluto can produce a report
//! that a developer without one can read.
//!
//! This backend was written without hardware to test against, which makes the
//! trace load-bearing rather than a nicety. Everything here also goes to
//! `tracing`, but `tracing` is the wrong shape for a bug report: it needs
//! `RUST_LOG` set before the run, and asking somebody to reproduce a connect
//! failure with the right filter set is asking them to reproduce it twice.
//! A [`Trace`] is always on, bounded, and dumps as one text block.
//!
//! # What it records
//!
//! Every IIOD command and its reply code, the context summary, the scan-element
//! formats the sample decoders were built from, a hex dump of the head of the
//! first `READBUF` response, and a periodic throughput summary. Buffer payloads
//! are *not* recorded — 2 Msps is 8 MB/s, and the framing is what diagnoses a
//! fault.
//!
//! The head of that first response is the single most valuable line in here:
//! the chunk framing (length line, then the channel mask, then the data) is the
//! part of the protocol most likely to be wrong in a from-scratch client, and
//! it cannot be checked from this side of the wire.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Debug, Default)]
struct Inner {
    lines: VecDeque<String>,
    dropped: u64,
    context: Option<String>,
}

/// A shared, bounded record of one Pluto session.
///
/// Cloning shares the same buffer — the control connection, the RX thread, the
/// TX thread and the `IqSource` all hold one.
#[derive(Debug, Clone)]
pub struct Trace {
    inner: Arc<Mutex<Inner>>,
    started: Instant,
}

impl Default for Trace {
    fn default() -> Self {
        Trace::new()
    }
}

impl Trace {
    /// How many lines are kept before the oldest are discarded. At roughly 80
    /// bytes a line this is a few hundred kilobytes — small enough to hold
    /// permanently, long enough to cover a connect plus several minutes of use.
    pub const CAPACITY: usize = 4000;

    pub fn new() -> Trace {
        Trace { inner: Arc::new(Mutex::new(Inner::default())), started: Instant::now() }
    }

    /// Record a free-form event.
    pub fn note(&self, what: impl AsRef<str>) {
        self.push(format!("{:>9.3}  {}", self.started.elapsed().as_secs_f64(), what.as_ref()));
    }

    /// Record a command we sent.
    pub fn cmd(&self, line: &str) {
        self.note(format!("→ {}", line.trim_end()));
        tracing::trace!(target: "pluto::iiod", cmd = %line.trim_end(), "sent");
    }

    /// Record the reply code (or the head of a text reply) a command produced.
    pub fn reply(&self, what: impl AsRef<str>) {
        self.note(format!("← {}", what.as_ref().trim_end()));
        tracing::trace!(target: "pluto::iiod", reply = %what.as_ref().trim_end(), "received");
    }

    /// Store the one-off context summary (devices, channels, scan formats).
    /// Kept out of the ring so a long session cannot age it out — it is the
    /// first thing anybody reading a report needs.
    pub fn set_context(&self, summary: impl Into<String>) {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).context = Some(summary.into());
    }

    /// Record the first bytes of a buffer response as hex, for the framing
    /// check described in this module's header.
    pub fn wire_head(&self, what: &str, bytes: &[u8]) {
        self.note(format!("▷ {what}: {}", hex_head(bytes, 64)));
        self.note(format!("▷ {what} (ascii): {}", ascii_head(bytes, 64)));
        tracing::debug!(
            target: "pluto::iiod",
            head = %hex_head(bytes, 64),
            ascii = %ascii_head(bytes, 64),
            "{what}"
        );
    }

    /// Render the whole trace as one text block, ready to paste into an issue.
    pub fn dump(&self) -> String {
        let g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let mut out = String::with_capacity(64 * 1024);
        out.push_str("=== sdroxide PlutoSDR session trace ===\n");
        out.push_str(&format!("elapsed: {:.1} s\n", self.started.elapsed().as_secs_f64()));
        if g.dropped > 0 {
            out.push_str(&format!(
                "note: {} earlier lines were discarded (ring holds {})\n",
                g.dropped,
                Trace::CAPACITY
            ));
        }
        out.push_str("\n--- context ---\n");
        match &g.context {
            Some(c) => {
                out.push_str(c);
                out.push('\n');
            }
            None => out.push_str("(the context description was never read)\n"),
        }
        out.push_str("\n--- iiod ---\n");
        for line in &g.lines {
            out.push_str(line);
            out.push('\n');
        }
        out
    }

    fn push(&self, line: String) {
        let mut g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        if g.lines.len() >= Trace::CAPACITY {
            g.lines.pop_front();
            g.dropped += 1;
        }
        g.lines.push_back(line);
    }
}

/// Format the first `n` bytes as spaced uppercase hex.
pub(crate) fn hex_head(bytes: &[u8], n: usize) -> String {
    bytes.iter().take(n).map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ")
}

/// The same bytes as printable ASCII, non-printables as `.`. IIOD framing is
/// text, so this is the line that actually reads: `1 0 2 4 . 0 0 0 0 0 0 0 3 .`
/// is a length, a mask and a newline, and anything else is a framing bug.
pub(crate) fn ascii_head(bytes: &[u8], n: usize) -> String {
    bytes.iter().take(n).map(|&b| if (0x20..0x7f).contains(&b) { b as char } else { '.' }).collect()
}

/// Where the always-on traces of the most recent sessions live, so the settings
/// UI can offer them as copyable text after the connection has already failed
/// and the handle has been dropped. Mirrors `sdroxide-smartsdr`'s diagnostics.
///
/// Open sessions and connection tests get separate slots. With one slot, a
/// "Test connection" click while a radio session was live (or after it died)
/// replaced the streaming trace with eight attribute reads — exactly the
/// evidence a streaming bug report needs, destroyed by the tester's most
/// natural next move. A field report was lost to this.
static LAST_OPEN: Mutex<Option<Trace>> = Mutex::new(None);
static LAST_PROBE: Mutex<Option<Trace>> = Mutex::new(None);

/// Remember an open (streaming) session's trace. The stored clone shares the
/// live buffer, so the slot keeps filling for as long as the session runs.
pub(crate) fn remember(trace: &Trace) {
    *LAST_OPEN.lock().unwrap_or_else(|e| e.into_inner()) = Some(trace.clone());
}

/// Remember a connection test's trace, in its own slot so it cannot displace
/// the evidence of an open session.
pub(crate) fn remember_probe(trace: &Trace) {
    *LAST_PROBE.lock().unwrap_or_else(|e| e.into_inner()) = Some(trace.clone());
}

/// The most recent session traces, for a bug report: the open session first —
/// it is the one with streaming evidence — then the most recent connection
/// test. `None` before the first attempt of either kind.
pub fn diagnostics() -> Option<String> {
    let open = LAST_OPEN.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let probe = LAST_PROBE.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if open.is_none() && probe.is_none() {
        return None;
    }
    let mut out = String::new();
    if let Some(t) = open {
        out.push_str("### radio session (open / stream)\n");
        out.push_str(&t.dump());
    }
    if let Some(t) = probe {
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str("### connection test\n");
        out.push_str(&t.dump());
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dump_reports_commands_replies_and_the_context() {
        let t = Trace::new();
        t.set_context("ad9361-phy, cf-ad9361-lpc [le:s12/16>>0]");
        t.cmd("READ ad9361-phy INPUT voltage0 hardwaregain\r\n");
        t.reply("4\n40.0");
        let d = t.dump();
        assert!(d.contains("→ READ ad9361-phy INPUT voltage0 hardwaregain"));
        assert!(d.contains("← 4"));
        assert!(d.contains("le:s12/16>>0"));
    }

    /// The context summary must survive a session long enough to overflow the
    /// ring — it is the first thing a reader needs and the ring would eat it.
    #[test]
    fn the_context_outlives_the_line_ring() {
        let t = Trace::new();
        t.set_context("ad9361-phy");
        for i in 0..(Trace::CAPACITY + 50) {
            t.note(format!("line {i}"));
        }
        let d = t.dump();
        assert!(d.contains("50 earlier lines were discarded"));
        assert!(d.contains("ad9361-phy"));
        assert!(d.contains(&format!("line {}", Trace::CAPACITY + 49)));
    }

    #[test]
    fn the_wire_head_is_readable_as_text_as_well_as_hex() {
        let t = Trace::new();
        t.wire_head("first READBUF", b"1024\n00000003\n\x01\x02");
        let d = t.dump();
        assert!(d.contains("31 30 32 34"), "{d}"); // "1024"
        assert!(d.contains("1024.00000003.."), "{d}");
    }

    #[test]
    fn an_empty_trace_still_dumps() {
        assert!(Trace::new().dump().contains("context description was never read"));
    }

    /// The mistake this guards against: a tester whose stream is misbehaving
    /// presses "Test connection" to check the radio is alive, and that click
    /// used to replace the streaming trace — the entire point of the report —
    /// with eight attribute reads.
    #[test]
    fn a_connection_test_does_not_displace_an_open_sessions_trace() {
        let open = Trace::new();
        open.note("streaming evidence");
        remember(&open);
        let probe = Trace::new();
        probe.note("eight attribute reads");
        remember_probe(&probe);
        let d = diagnostics().expect("both slots filled");
        let open_at = d.find("radio session").expect("open section present");
        let probe_at = d.find("connection test").expect("probe section present");
        assert!(open_at < probe_at, "the open session must come first:\n{d}");
        assert!(d.contains("streaming evidence"), "{d}");
        assert!(d.contains("eight attribute reads"), "{d}");
    }
}
