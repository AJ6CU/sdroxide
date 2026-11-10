//! A self-contained session trace, so a user with a radio can produce a report
//! that a developer without one can read.
//!
//! Same reasoning as the SmartSDR and Pluto traces: `tracing` needs `RUST_LOG`
//! set before the run and interleaves with every other subsystem, so asking
//! somebody to reproduce a connect failure with the right filter is asking them
//! to reproduce it twice. A [`Trace`] is always on, bounded, and dumps as one
//! text block.
//!
//! # What it records
//!
//! Every handshake step, every CI-V frame in both directions (they are short
//! and they are the interesting part), and periodic counters for the audio and
//! scope streams. Audio payloads are *not* recorded — 48 kHz mono is 96 kB/s,
//! and the counters diagnose the fault the bytes would not.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Debug, Default, Clone, Copy)]
pub struct Counters {
    pub audio_packets: u64,
    pub audio_bytes: u64,
    pub civ_frames: u64,
    pub scope_sweeps: u64,
    pub sent: u64,
    pub retransmit_in: u64,
    pub retransmit_out: u64,
    pub missing: u64,
}

#[derive(Debug, Default)]
struct Inner {
    lines: VecDeque<String>,
    dropped: u64,
    counters: Counters,
}

/// A shared, bounded record of one radio session. Cloning shares the buffer —
/// the three stream threads and the `IqSource` all hold one.
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
    /// At roughly 80 bytes a line this is a few hundred kilobytes: small enough
    /// to hold permanently, long enough to cover a connect plus minutes of
    /// CI-V traffic.
    pub const CAPACITY: usize = 4000;

    pub fn new() -> Trace {
        Trace { inner: Arc::new(Mutex::new(Inner::default())), started: Instant::now() }
    }

    pub fn note(&self, what: impl AsRef<str>) {
        let line = format!("{:>9.3}  {}", self.started.elapsed().as_secs_f64(), what.as_ref());
        let mut g = self.lock();
        if g.lines.len() >= Trace::CAPACITY {
            g.lines.pop_front();
            g.dropped += 1;
        }
        g.lines.push_back(line);
    }

    /// A CI-V frame we sent. Long frames are elided — the only long frames are
    /// scope sweeps, and 475 bins of hex helps nobody.
    pub fn civ_tx(&self, frame: &[u8]) {
        self.note(format!("→ CI-V {}", hex_elided(frame)));
    }

    /// A CI-V frame the radio sent.
    pub fn civ_rx(&self, frame: &[u8]) {
        {
            let mut g = self.lock();
            g.counters.civ_frames += 1;
            // Counted here rather than at the point the sweep is assembled, so
            // the figure answers the question a report is filed to answer: is
            // the radio sending its scope at all? Zero sweeps against a healthy
            // CI-V count says the `27 10`/`27 11` writes did not take.
            if is_scope_sweep(frame) {
                g.counters.scope_sweeps += 1;
            }
        }
        self.note(format!("← CI-V {}", hex_elided(frame)));
    }

    pub fn audio_packet(&self, bytes: usize) {
        let mut g = self.lock();
        g.counters.audio_packets += 1;
        g.counters.audio_bytes += bytes as u64;
    }

    pub fn sent(&self) {
        self.lock().counters.sent += 1;
    }

    /// The peer asked us to resend something.
    pub fn retransmit_in(&self) {
        self.lock().counters.retransmit_in += 1;
    }

    /// We asked the peer to resend something.
    pub fn retransmit_out(&self, n: usize) {
        let mut g = self.lock();
        g.counters.retransmit_out += 1;
        g.counters.missing += n as u64;
    }

    pub fn counters(&self) -> Counters {
        self.lock().counters
    }

    /// Render the whole trace as one text block, ready to paste into an issue.
    pub fn dump(&self) -> String {
        let g = self.lock();
        let c = g.counters;
        let mut out = String::with_capacity(64 * 1024);
        out.push_str("=== sdroxide Icom LAN session trace ===\n");
        out.push_str(&format!("elapsed: {:.1} s\n", self.started.elapsed().as_secs_f64()));
        if g.dropped > 0 {
            out.push_str(&format!(
                "note: {} earlier lines were discarded (ring holds {})\n",
                g.dropped,
                Trace::CAPACITY
            ));
        }
        out.push_str("\n--- counters ---\n");
        out.push_str(&format!(
            "packets sent      {}\naudio packets     {} ({} bytes)\nCI-V frames in    {}\n\
             scope sweeps      {}\nresend asked of us {}\nresend we asked   {} ({} sequences)\n",
            c.sent,
            c.audio_packets,
            c.audio_bytes,
            c.civ_frames,
            c.scope_sweeps,
            c.retransmit_in,
            c.retransmit_out,
            c.missing,
        ));
        out.push_str("\n--- session ---\n");
        for line in &g.lines {
            out.push_str(line);
            out.push('\n');
        }
        out
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }
}

/// Whether an inbound frame is a `27 00` scope sweep: `FE FE <to> <from> 27 00`.
fn is_scope_sweep(frame: &[u8]) -> bool {
    frame.len() > 6 && frame[..2] == [0xfe, 0xfe] && frame[4] == 0x27 && frame[5] == 0x00
}

/// Hex, but a scope sweep does not get to fill the buffer on its own.
fn hex_elided(frame: &[u8]) -> String {
    const HEAD: usize = 12;
    let mut s = String::with_capacity(3 * HEAD + 24);
    for (i, b) in frame.iter().take(HEAD).enumerate() {
        if i > 0 {
            s.push(' ');
        }
        s.push_str(&format!("{b:02x}"));
    }
    if frame.len() > HEAD {
        s.push_str(&format!(" … ({} bytes)", frame.len()));
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dump_reports_both_directions_and_counters() {
        let t = Trace::new();
        t.civ_tx(&[0xfe, 0xfe, 0xb6, 0xe0, 0x03, 0xfd]);
        t.civ_rx(&[0xfe, 0xfe, 0xe0, 0xb6, 0x03, 0x00, 0x50, 0x45, 0x14, 0x00, 0xfd]);
        t.audio_packet(1920);

        let d = t.dump();
        assert!(d.contains("→ CI-V fe fe b6 e0 03 fd"));
        assert!(d.contains("← CI-V fe fe e0 b6"));
        assert!(d.contains("audio packets     1 (1920 bytes)"));
        // A frequency report is not a sweep.
        assert!(d.contains("scope sweeps      0"));
    }

    #[test]
    fn a_sweep_is_counted_as_one_and_a_report_with_none_says_so() {
        // The counter answers the one question a report about a missing
        // full-band waterfall is filed to answer. It used to be incremented
        // nowhere at all, so it always read zero — worse than absent, because
        // zero looked like an answer.
        let t = Trace::new();
        let mut sweep = vec![0xfe, 0xfe, 0xe0, 0xa4, 0x27, 0x00, 0x00, 0x01, 0x01];
        sweep.extend(std::iter::repeat_n(0x40u8, 487));
        sweep.push(0xfd);
        t.civ_rx(&sweep);
        t.civ_rx(&[0xfe, 0xfe, 0xe0, 0xa4, 0x03, 0x00, 0x50, 0x45, 0x14, 0x00, 0xfd]);
        assert_eq!(t.counters().scope_sweeps, 1);
        assert_eq!(t.counters().civ_frames, 2);
    }

    #[test]
    fn a_scope_sweep_does_not_fill_the_buffer() {
        let t = Trace::new();
        t.civ_rx(&vec![0x33u8; 490]);
        assert_eq!(t.counters().scope_sweeps, 0, "junk is not a sweep");
        let d = t.dump();
        assert!(d.contains("… (490 bytes)"));
        assert!(d.len() < 2000);
    }

    #[test]
    fn the_ring_bounds_a_long_session_and_says_so() {
        let t = Trace::new();
        for i in 0..(Trace::CAPACITY + 20) {
            t.note(format!("line {i}"));
        }
        let d = t.dump();
        assert!(d.contains("20 earlier lines were discarded"));
        assert!(d.contains(&format!("line {}", Trace::CAPACITY + 19)));
        assert!(!d.contains("line 0\n"));
    }

    #[test]
    fn an_empty_trace_still_dumps() {
        assert!(Trace::new().dump().contains("packets sent      0"));
    }
}
