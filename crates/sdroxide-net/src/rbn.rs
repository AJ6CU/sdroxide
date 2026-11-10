//! Reverse Beacon Network reader: the world's skimmers, turned into paths.
//!
//! Structurally a twin of [`crate::cluster`] — a blocking `TcpStream` with a
//! short read timeout, a `crossbeam` control channel, a `Drop` that stops the
//! thread — and deliberately not the same client, because what arrives is not
//! the same thing.
//!
//! A DX cluster carries invitations: somebody thinks this station is worth
//! working. RBN carries measurements: a receiver in one place decoded a
//! transmitter in another, at a known frequency, with a signal-to-noise figure
//! attached. Thousands a minute, most of them about stations nobody wants to
//! call. They go to the propagation map and never to the spot list, which is
//! why this reader emits [`PropPath`]s rather than `Spot`s.
//!
//! ## What this reader cannot know
//!
//! **Neither end has a locator.** An RBN line carries two callsigns and no
//! grids, and RBN publishes no machine-readable skimmer list, so both ends are
//! placed at their DXCC entity's nominal centre via
//! [`sdroxide_types::resolve_place`]. For Liechtenstein that is a good
//! position; for the United States it can be two thousand kilometres out.
//! [`sdroxide_types::PropSource::Rbn`] exists as its own switchable source
//! precisely so this evidence stays separable from the kind that comes with a
//! grid attached.

use std::io::Read;
use std::net::TcpStream;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use crossbeam_channel::{Receiver, Sender};
use sdroxide_types::{PropPath, RbnConfig, resolve_place};

use tracing::{debug, warn};

use crate::cluster::{parse_dx_line, send_login_to, wants_login};
use crate::event::{EventTx, NetEvent};

/// How often a batch of paths is handed on.
///
/// The map decays over tens of minutes, so nothing is gained by delivering
/// each line the instant it lands — and a channel send per line, at RBN's
/// rate, would be the only busy thing in an otherwise idle program.
const FLUSH: Duration = Duration::from_secs(5);

/// Most paths carried in one batch.
///
/// A backstop, not a policy: the feed runs at a few hundred paths per flush.
/// If a node ever dumps its history at us, this bounds the damage — and what
/// it drops is counted and reported rather than silently discarded, because a
/// truncated batch that says nothing reads as a complete one.
const MAX_BATCH: usize = 4000;

enum Ctrl {
    Shutdown,
}

/// A live RBN connection. Dropping it stops the thread.
pub struct RbnHandle {
    ctrl: Sender<Ctrl>,
    join: Option<JoinHandle<()>>,
}

impl RbnHandle {
    /// Connect and spawn the reader thread. `login` is the callsign sent at the
    /// prompt; RBN wants a real one.
    pub fn connect(
        cfg: RbnConfig,
        login: String,
        now_utc: impl Fn() -> i64 + Send + 'static,
        events: EventTx,
    ) -> RbnHandle {
        let (ctrl_tx, ctrl_rx) = crossbeam_channel::unbounded();
        let join = std::thread::Builder::new()
            .name("sdroxide-rbn".into())
            .spawn(move || run(cfg, login, now_utc, events, ctrl_rx))
            .ok();
        RbnHandle { ctrl: ctrl_tx, join }
    }
}

impl Drop for RbnHandle {
    fn drop(&mut self) {
        let _ = self.ctrl.send(Ctrl::Shutdown);
        if let Some(j) = self.join.take() {
            let _ = j.join();
        }
    }
}

fn run(
    cfg: RbnConfig,
    login: String,
    now_utc: impl Fn() -> i64,
    events: EventTx,
    ctrl: Receiver<Ctrl>,
) {
    let addr = format!("{}:{}", cfg.host.trim(), cfg.port);
    let _ = events.send(NetEvent::Status(Some(format!("RBN: connecting to {addr}…"))));
    let stream = match TcpStream::connect(&addr) {
        Ok(s) => s,
        Err(e) => {
            let _ = events.send(NetEvent::Status(Some(format!("RBN: connect failed: {e}"))));
            warn!("rbn connect {addr}: {e}");
            return;
        }
    };
    if stream.set_read_timeout(Some(Duration::from_millis(200))).is_err() {
        return;
    }
    let mut stream = stream;

    let mut buf = String::new();
    let mut raw = [0u8; 8192];
    let mut logged_in = false;
    let mut batch: Vec<PropPath> = Vec::new();
    let mut dropped = 0usize;
    // Lines that parsed as spots but whose callsigns the country file could not
    // place. Reported with the rest so a prefix table going stale shows up as a
    // number rather than as a map that is quietly missing paths.
    let mut unplaced = 0usize;
    let connected_at = Instant::now();
    let mut last_flush = Instant::now();
    let _ = events.send(NetEvent::Status(Some(format!("RBN: connected to {addr}"))));

    loop {
        if matches!(ctrl.try_recv(), Ok(Ctrl::Shutdown)) {
            let _ = events.send(NetEvent::Status(None));
            return;
        }

        match stream.read(&mut raw) {
            Ok(0) => {
                let _ = events.send(NetEvent::Status(Some("RBN: disconnected".into())));
                return;
            }
            Ok(n) => buf.push_str(&String::from_utf8_lossy(&raw[..n])),
            Err(e)
                if e.kind() == std::io::ErrorKind::WouldBlock
                    || e.kind() == std::io::ErrorKind::TimedOut => {}
            Err(e) => {
                let _ = events.send(NetEvent::Status(Some(format!("RBN: read error: {e}"))));
                return;
            }
        }

        while let Some(pos) = buf.find('\n') {
            let line: String = buf.drain(..=pos).collect();
            let line = line.trim_end_matches(['\r', '\n']);
            if !logged_in && wants_login(line) {
                send_login_to(&mut stream, &login, &cfg.commands, "rbn");
                logged_in = true;
                continue;
            }
            match parse_rbn_line(line, now_utc()) {
                Some(p) => {
                    if batch.len() < MAX_BATCH {
                        batch.push(p);
                    } else {
                        dropped += 1;
                    }
                }
                None if is_spot_line(line) => unplaced += 1,
                None => {}
            }
        }

        if !logged_in && wants_login(&buf) {
            send_login_to(&mut stream, &login, &cfg.commands, "rbn");
            logged_in = true;
            buf.clear();
        }
        if !logged_in && connected_at.elapsed() > Duration::from_secs(2) {
            send_login_to(&mut stream, &login, &cfg.commands, "rbn");
            logged_in = true;
        }

        if last_flush.elapsed() >= FLUSH {
            last_flush = Instant::now();
            if dropped > 0 || unplaced > 0 {
                debug!("rbn: {dropped} paths over the batch cap, {unplaced} unplaceable");
                let _ = events.send(NetEvent::Status(Some(format!(
                    "RBN: {} paths, {dropped} over cap, {unplaced} unplaceable",
                    batch.len()
                ))));
                dropped = 0;
                unplaced = 0;
            }
            if !batch.is_empty()
                && events.send(NetEvent::PropPaths(std::mem::take(&mut batch))).is_err()
            {
                return; // the manager went away
            }
        }
    }
}

/// Does this look like a spot line we should have been able to use?
///
/// Only asked about lines [`parse_rbn_line`] rejected, to tell "that was the
/// node's welcome banner" from "that was a spot we could not place".
fn is_spot_line(line: &str) -> bool {
    let t = line.trim_start();
    t.starts_with("DX de ") || t.starts_with("Dx de ")
}

/// Turn one RBN telnet line into a path, or `None` if it is not a spot or
/// either end cannot be placed.
fn parse_rbn_line(line: &str, now: i64) -> Option<PropPath> {
    let spot = parse_dx_line(line, now)?;
    // Skimmer callsigns are published with a `-#` (or `-2`, for a second
    // receiver at one site) marker. It is not part of the callsign and the
    // country file does not know it.
    let spotter = spot.spotter.split('-').next().unwrap_or(&spot.spotter);
    let rx = resolve_place(spotter)?;
    let tx = resolve_place(&spot.call)?;
    Some(PropPath {
        tx: (tx.lat, tx.lon),
        rx: (rx.lat, rx.lon),
        freq_hz: spot.freq_hz,
        at_utc: spot.when_utc,
        snr_db: snr_from_comment(&spot.comment),
        key: format!("{spotter}>{}", spot.call),
    })
}

/// Pull the `19 dB` figure out of a skimmer comment.
///
/// The token before a bare `dB`, which is how every RBN line writes it. A
/// missing or unreadable figure costs the margin and keeps the path: that a
/// signal got from one place to another is the more important half of what
/// this line says.
fn snr_from_comment(comment: &str) -> Option<f32> {
    let toks: Vec<&str> = comment.split_whitespace().collect();
    let i = toks.iter().position(|t| t.eq_ignore_ascii_case("db"))?;
    toks.get(i.checked_sub(1)?)?.parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: i64 = 1_800_000_000;

    /// A real line, as RBN sends it.
    const LINE: &str =
        "DX de EA5WU-#:    7018.3  RW1M           CW    19 dB  18 WPM  CQ      2259Z";

    #[test]
    fn a_skimmer_line_becomes_a_path_between_two_countries() {
        let p = parse_rbn_line(LINE, NOW).expect("a well-formed RBN line");
        assert_eq!(p.freq_hz, 7_018_300.0);
        assert_eq!(p.snr_db, Some(19.0));
        // Spain heard Russia: the receiver is the skimmer, the transmitter the
        // station it decoded. Getting these the wrong way round would mirror
        // every path on the map about its own midpoint.
        assert!(p.rx.0 > 35.0 && p.rx.0 < 45.0, "receiver not in Spain: {:?}", p.rx);
        assert!(p.tx.0 > 45.0, "transmitter not in Russia: {:?}", p.tx);
        // The `-#` marker is not part of the callsign.
        assert_eq!(p.key, "EA5WU>RW1M");
    }

    /// The dedup key is what stops a firehose counting one path a hundred
    /// times, so it has to be the same string every time that path appears.
    #[test]
    fn the_same_path_gets_the_same_key_whatever_the_report_says() {
        let a = parse_rbn_line(LINE, NOW).unwrap();
        let b = parse_rbn_line(
            "DX de EA5WU-#:    7018.4  RW1M           CW    7 dB  19 WPM  CQ      2301Z",
            NOW + 120,
        )
        .unwrap();
        assert_eq!(a.key, b.key);
        assert_eq!(b.snr_db, Some(7.0));
    }

    /// A second receiver at one site is the same operator and the same place.
    #[test]
    fn a_numbered_second_skimmer_resolves_like_the_first() {
        let p = parse_rbn_line(
            "DX de EA5WU-2:    7018.3  RW1M           CW    19 dB  18 WPM  CQ      2259Z",
            NOW,
        )
        .unwrap();
        assert_eq!(p.key, "EA5WU>RW1M");
    }

    #[test]
    fn banners_and_prompts_are_not_paths() {
        assert!(parse_rbn_line("Welcome to the Reverse Beacon Network", NOW).is_none());
        assert!(parse_rbn_line("please enter your call: ", NOW).is_none());
        assert!(!is_spot_line("Welcome to the Reverse Beacon Network"));
        assert!(is_spot_line("DX de EA5WU-#: 7018.3 RW1M CW 19 dB 18 WPM CQ 2259Z"));
    }

    /// A skimmer spot with no dB figure is still a demonstrated path.
    #[test]
    fn a_line_without_a_report_keeps_its_path() {
        let p = parse_rbn_line("DX de EA5WU-#:  7018.3  RW1M  CW  CQ  2259Z", NOW).unwrap();
        assert_eq!(p.snr_db, None);
        assert_eq!(p.key, "EA5WU>RW1M");
    }

    #[test]
    fn a_callsign_the_country_file_cannot_place_is_dropped_rather_than_guessed() {
        assert!(
            parse_rbn_line("DX de 11111-#:  7018.3  22222  CW  19 dB  18 WPM  CQ  2259Z", NOW)
                .is_none()
        );
    }

    #[test]
    fn the_snr_token_is_the_one_before_the_unit() {
        assert_eq!(snr_from_comment("CW 19 dB 18 WPM CQ"), Some(19.0));
        assert_eq!(snr_from_comment("RTTY 6 dB 45 BPS CQ"), Some(6.0));
        assert_eq!(snr_from_comment("CW CQ"), None);
        // "dB" with nothing usable in front of it must not become a zero.
        assert_eq!(snr_from_comment("dB"), None);
        assert_eq!(snr_from_comment("CW loud dB"), None);
    }
}
