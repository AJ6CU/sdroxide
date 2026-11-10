//! A self-contained session trace, so a user with a HackRF can produce a report
//! that a developer reading it later can act on.
//!
//! Everything here also goes to `tracing`, but `tracing` is the wrong shape for
//! a bug report: it needs `RUST_LOG` set *before* the run, and asking somebody
//! to reproduce a transmit fault with the right filter set is asking them to
//! key up twice. A [`Trace`] is always on, bounded, and dumps as one text block.
//!
//! # Why this matters more here than on the receivers next door
//!
//! On a receive-only backend a trace answers "what did the radio say". On this
//! one the question is almost always **"in what order did we say it"**. The
//! four SoapyHackRF defects this driver exists to avoid are every one of them
//! an ordering bug — an amplifier programmed before a mode change instead of
//! after, a receive stream deactivated instead of closed — and none of them is
//! visible in any single transfer. [`Trace::direction`] therefore marks each
//! transition with a banner, so the requests that bracket a key-down can be
//! read as a sequence rather than reconstructed from timestamps.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use sdroxide_dsp::Complex32;

/// What the settings panel tells an operator, and what the manual repeats.
pub const FIELD_REPORT_HINT: &str = "If the radio misbehaves — especially around \
    transmit — please attach the session trace (Settings → Radio → Copy \
    diagnostic report) to a bug report. It contains every command exchanged \
    with the radio, in order, including the sequence around each key-down.";

#[derive(Debug, Default)]
struct Inner {
    lines: VecDeque<String>,
    dropped: u64,
    identity: Option<String>,
}

/// A shared, bounded record of one HackRF session.
///
/// Cloning shares the same buffer — the stream thread and the `IqSource` both
/// hold one.
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
    /// permanently, long enough to cover an open plus several minutes of use.
    ///
    /// Larger than the Airspy HF+'s ring for a reason: a direction change costs
    /// a dozen lines, and an FT8 session keys up every fifteen seconds. A ring
    /// that only covered a few minutes of that would age out the open sequence
    /// — which is what a transmit report needs to be read against.
    pub const CAPACITY: usize = 8000;

    pub fn new() -> Trace {
        Trace { inner: Arc::new(Mutex::new(Inner::default())), started: Instant::now() }
    }

    /// Record a free-form event.
    pub fn note(&self, what: impl AsRef<str>) {
        self.push(format!("{:>9.3}  {}", self.started.elapsed().as_secs_f64(), what.as_ref()));
    }

    /// Mark a change of direction, with a banner that survives skimming.
    ///
    /// The whole transmit design is a sequence — drain, close the endpoint,
    /// mode off, retune, mode on, *then* the gains — and the failure modes are
    /// steps happening in the wrong order or not at all. A reader has to be
    /// able to find where each over began without counting timestamps.
    pub fn direction(&self, what: &str) {
        self.note(format!("======== {what} ========"));
        tracing::debug!(target: "hackrf::direction", what);
    }

    /// Record a control transfer and what came of it.
    ///
    /// `got` is the reply length for an IN request and `None` for an OUT one.
    /// It is a separate column rather than part of the outcome text because
    /// "asked for 1, got 0" is how a refused gain looks, and it must be
    /// greppable.
    // Eight columns, and every one of them is a field of the USB setup packet
    // or its outcome. A struct here would be a struct with eight fields.
    #[allow(clippy::too_many_arguments)]
    pub fn ctrl(
        &self,
        request: u8,
        name: &str,
        value: u16,
        index: u16,
        want: usize,
        got: Option<usize>,
        outcome: &str,
    ) {
        let len = match got {
            Some(n) if n == want => format!("len {n}"),
            Some(n) => format!("len {n}/{want} SHORT"),
            None => format!("len {want}"),
        };
        self.note(format!(
            "req {request:>2} {name:<28} val 0x{value:04x} idx 0x{index:04x} {len:<16} {outcome}"
        ));
        tracing::debug!(target: "hackrf::ctrl", request, name, value, index, want, ?got, outcome);
    }

    /// Store the one-off identity summary. Kept out of the ring so a long
    /// session cannot age it out — it is the first thing anybody reading a
    /// report needs.
    pub fn set_identity(&self, summary: impl Into<String>) {
        self.inner.lock().unwrap_or_else(|e| e.into_inner()).identity = Some(summary.into());
    }

    /// Record raw bytes as hex.
    pub fn wire_head(&self, what: &str, bytes: &[u8], n: usize) {
        self.note(format!("▷ {what}: {}", hex_head(bytes, n)));
    }

    /// The head of the first bulk completion, hex *and* decoded.
    ///
    /// Point the radio at a carrier deliberately above the LO and these two
    /// lines together answer the question this driver cannot answer for itself:
    /// is I really first on the wire? A mirrored spectrum and a correct one
    /// look identical in hex, and on a symmetric test signal they look
    /// identical on screen too.
    pub fn first_samples(&self, bytes: &[u8], decoded: &[Complex32]) {
        self.note(format!("▷ first bulk bytes: {}", hex_head(bytes, 64)));
        let pairs: Vec<String> =
            decoded.iter().take(8).map(|s| format!("({:+.4},{:+.4})", s.re, s.im)).collect();
        self.note(format!("▷ decoded as (re,im): {}", pairs.join(" ")));
    }

    /// Render the whole trace as one text block, ready to paste into an issue.
    pub fn dump(&self) -> String {
        let g = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        let mut out = String::with_capacity(64 * 1024);
        out.push_str("=== sdroxide HackRF session trace ===\n");
        // The host, because the USB layer under this driver is a different
        // program on each one — usbfs on Linux, WinUSB on Windows, IOKit on
        // macOS — and they fail differently. A report that does not say which
        // one it came from has been read against the wrong one at least once.
        out.push_str(&format!("host: {} ({})\n", std::env::consts::OS, std::env::consts::ARCH));
        out.push_str(&format!("elapsed: {:.1} s\n", self.started.elapsed().as_secs_f64()));
        if g.dropped > 0 {
            out.push_str(&format!(
                "note: {} earlier lines were discarded (ring holds {})\n",
                g.dropped,
                Trace::CAPACITY
            ));
        }
        out.push_str("\n--- identity ---\n");
        match &g.identity {
            Some(c) => {
                out.push_str(c);
                out.push('\n');
            }
            None => out.push_str("(the radio was never identified)\n"),
        }
        out.push_str("\n--- session ---\n");
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
pub fn hex_head(bytes: &[u8], n: usize) -> String {
    bytes.iter().take(n).map(|b| format!("{b:02X}")).collect::<Vec<_>>().join(" ")
}

/// Where the always-on traces of the most recent sessions live, so the settings
/// UI can offer them as copyable text after the radio has already failed and
/// the handle has been dropped. Mirrors `sdroxide-airspyhf`'s diagnostics.
///
/// Open sessions and probes get separate slots. With one slot, a Rescan or a
/// `--probe` after a fault would replace the streaming trace — the entire point
/// of the report — with an enumeration.
static LAST_OPEN: Mutex<Option<Trace>> = Mutex::new(None);
static LAST_PROBE: Mutex<Option<Trace>> = Mutex::new(None);

/// Remember an open (streaming) session's trace. The stored clone shares the
/// live buffer, so the slot keeps filling for as long as the session runs.
pub fn remember(trace: &Trace) {
    *LAST_OPEN.lock().unwrap_or_else(|e| e.into_inner()) = Some(trace.clone());
}

/// Remember a probe's trace, in its own slot so it cannot displace the evidence
/// of an open session.
pub fn remember_probe(trace: &Trace) {
    *LAST_PROBE.lock().unwrap_or_else(|e| e.into_inner()) = Some(trace.clone());
}

/// The most recent session traces, for a bug report: the open session first —
/// it is the one with streaming and transmit evidence — then the most recent
/// probe. `None` before the first attempt of either kind.
pub fn diagnostics() -> Option<String> {
    let open = LAST_OPEN.lock().unwrap_or_else(|e| e.into_inner()).clone();
    let probe = LAST_PROBE.lock().unwrap_or_else(|e| e.into_inner()).clone();
    if open.is_none() && probe.is_none() {
        return None;
    }
    let mut out = String::new();
    if let Some(t) = open {
        out.push_str("### radio session (open / stream / transmit)\n");
        out.push_str(&t.dump());
    }
    if let Some(t) = probe {
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str("### probe\n");
        out.push_str(&t.dump());
    }
    Some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dump_reports_requests_and_the_identity() {
        let t = Trace::new();
        t.set_identity("HackRF One, firmware 2024.02.1");
        t.ctrl(14, "BoardIdRead", 0, 0, 1, Some(1), "HackRF One");
        let d = t.dump();
        assert!(d.contains("req 14 BoardIdRead"), "{d}");
        assert!(d.contains("2024.02.1"), "{d}");
    }

    /// A refused gain is an IN request that came back with nothing, which is
    /// what the length column exists to make greppable.
    #[test]
    fn a_short_reply_is_called_out_in_the_length_column() {
        let t = Trace::new();
        t.ctrl(19, "SetLnaGain", 0, 40, 1, Some(0), "FAILED: refused");
        let d = t.dump();
        assert!(d.contains("len 0/1 SHORT"), "{d}");
        // An exact reply is not flagged, or the flag would mean nothing.
        let t2 = Trace::new();
        t2.ctrl(19, "SetLnaGain", 0, 16, 1, Some(1), "ok");
        assert!(!t2.dump().contains("SHORT"));
        // An OUT request has no reply length to compare against.
        let t3 = Trace::new();
        t3.ctrl(16, "SetFreq", 0, 0, 8, None, "100.100 MHz");
        assert!(!t3.dump().contains("SHORT"));
    }

    /// The whole point of the direction banner: the requests bracketing an over
    /// have to be findable as a block, because every SoapyHackRF defect this
    /// driver avoids is an ordering bug rather than a bad transfer.
    #[test]
    fn a_direction_change_is_findable_as_a_block() {
        let t = Trace::new();
        t.ctrl(19, "SetLnaGain", 0, 16, 1, Some(1), "ok");
        t.direction("RX -> TX");
        t.ctrl(1, "SetTransceiverMode", 0, 0, 0, None, "Off");
        t.ctrl(16, "SetFreq", 0, 0, 8, None, "14.074 MHz");
        t.ctrl(1, "SetTransceiverMode", 2, 0, 0, None, "Transmit");
        t.ctrl(21, "SetTxvgaGain", 0, 20, 1, Some(1), "ok");
        t.direction("TX -> RX");
        let d = t.dump();
        let tx_at = d.find("RX -> TX").expect("the transition is marked");
        let back_at = d.find("TX -> RX").expect("and so is the way back");
        assert!(tx_at < back_at);
        // The gains must be visible *after* the mode change inside the block —
        // that ordering is the bug this driver exists to not have.
        let mode_at = d[tx_at..back_at].find("Transmit").expect("mode set inside the block");
        let gain_at = d[tx_at..back_at].find("SetTxvgaGain").expect("gain set inside the block");
        assert!(mode_at < gain_at, "the amp/gain writes must follow the mode change:\n{d}");
    }

    /// The identity must survive a session long enough to overflow the ring —
    /// it is the first thing a reader needs and the ring would eat it.
    #[test]
    fn the_identity_outlives_the_line_ring() {
        let t = Trace::new();
        t.set_identity("HackRF One");
        for i in 0..(Trace::CAPACITY + 50) {
            t.note(format!("line {i}"));
        }
        let d = t.dump();
        assert!(d.contains("50 earlier lines were discarded"));
        assert!(d.contains("HackRF One"));
        assert!(d.contains(&format!("line {}", Trace::CAPACITY + 49)));
    }

    /// Hex alone cannot tell a mirrored spectrum from a correct one; the
    /// decoded pairs beside it can.
    #[test]
    fn the_first_samples_are_shown_as_bytes_and_as_numbers() {
        let t = Trace::new();
        let bytes = [0x40u8, 0x20];
        t.first_samples(&bytes, &[Complex32::new(0.5, 0.25)]);
        let d = t.dump();
        assert!(d.contains("40 20"), "{d}");
        assert!(d.contains("(+0.5000,+0.2500)"), "{d}");
    }

    #[test]
    fn an_empty_trace_still_dumps() {
        assert!(Trace::new().dump().contains("never identified"));
    }

    /// The mistake this guards against: an operator whose transmit is
    /// misbehaving presses Rescan to check the radio is still there, and that
    /// click replaces the trace containing the key-down sequence — the entire
    /// point of the report — with an enumeration.
    #[test]
    fn a_probe_does_not_displace_an_open_sessions_trace() {
        let open = Trace::new();
        open.note("key-down evidence");
        remember(&open);
        let probe = Trace::new();
        probe.note("just an enumeration");
        remember_probe(&probe);
        let d = diagnostics().expect("both slots filled");
        let open_at = d.find("radio session").expect("open section present");
        let probe_at = d.find("### probe").expect("probe section present");
        assert!(open_at < probe_at, "the open session must come first:\n{d}");
        assert!(d.contains("key-down evidence"), "{d}");
        assert!(d.contains("just an enumeration"), "{d}");
    }
}
