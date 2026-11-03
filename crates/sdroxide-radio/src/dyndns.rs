//! **Dynamic DNS**: keep one hostname pointed at this station's address.
//!
//! Every provider — DuckDNS, dynv6, No-IP, freedns, a station's own script —
//! reduces to the same thing: an **update URL** that is fetched, with the token
//! in it. So this is one URL and an interval, not a list of providers and not a
//! single key of ours: whatever the operator was issued is what is asked for,
//! and a provider we have never heard of works the same way.
//!
//! **Empty is off, and empty is the default.** Nothing is started, no thread
//! exists, no request is made: a station that does not want this never meets
//! it, and the program runs offline exactly as it did before (AGENTS: no paid
//! service, no API key of ours, no account — the address is the operator's own).
//!
//! **It lives here, in the engine, and not in the screen.** The name has to
//! point at the address of the machine that is *heard*, and the screen is not
//! always on it: `sdroxide --server` on a Pi runs with no UI open at all, and a
//! UI on a laptop driving a remote station would otherwise send the laptop's
//! address to the provider — the name would resolve to the operator's desk
//! instead of his aerial. A browser client has no address to keep and never
//! reaches this file.
//!
//! The URL carries the token, so it is a secret: it is never written to a log
//! and never repeated in a message. What travels back to the screen is the
//! provider's **answer** (`good`, `nochg`, `KO`, an HTTP status), which is the
//! only part that is not a secret.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, TryRecvError};
use std::sync::Arc;

/// The provider's answer, as the operator reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DyndnsAnswer {
    /// When the update was answered, unix seconds.
    pub at_unix: i64,
    /// The provider's own word, trimmed and shortened. Never the address.
    pub text: String,
    /// Whether the provider accepted it.
    pub ok: bool,
}

/// Seconds before one retry after a failed update.
///
/// A station whose network is not up yet at boot would otherwise wait a whole
/// interval — five minutes by default — before its name was right. One retry
/// covers that without hammering a provider that is refusing the token.
const RETRY_S: u64 = 60;

/// The worker, in step with the network settings.
#[derive(Default)]
pub struct Dyndns {
    /// The configuration the running thread was started for, so a change of
    /// address or interval stops it and starts another.
    running: Option<(String, u32)>,
    stop: Option<Arc<AtomicBool>>,
    rx: Option<Receiver<DyndnsAnswer>>,
}

impl Dyndns {
    /// Keep the worker in step with the settings.
    ///
    /// Called whenever the network configuration is applied, and at boot, so
    /// the thread exists exactly when an address is set.
    pub fn set(&mut self, url: &str, minutes: u32) {
        let wanted = url.trim().to_string();
        let minutes = minutes.clamp(1, 1440);
        let wanted = if wanted.is_empty() { None } else { Some((wanted, minutes)) };
        if self.running != wanted {
            self.stop_worker();
            self.running = wanted.clone();
            if let Some((url, minutes)) = wanted {
                self.start_worker(url, minutes);
            }
        }
    }

    /// The newest answer, if one has arrived since the last call.
    pub fn poll(&mut self) -> Option<DyndnsAnswer> {
        let rx = self.rx.take()?;
        let mut newest = None;
        loop {
            match rx.try_recv() {
                Ok(answer) => newest = Some(answer),
                Err(TryRecvError::Empty) => {
                    self.rx = Some(rx);
                    return newest;
                }
                Err(TryRecvError::Disconnected) => return newest,
            }
        }
    }

    fn start_worker(&mut self, url: String, minutes: u32) {
        let (tx, rx) = std::sync::mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let stopped = stop.clone();
        std::thread::Builder::new()
            .name("sdroxide-dyndns".into())
            .spawn(move || {
                // One short retry after a failure, then the normal interval:
                // a station whose network is not up yet at boot gets its name
                // right a minute later instead of a whole interval later.
                let mut retried = false;
                while !stopped.load(Ordering::Relaxed) {
                    let answer = update_once(&url);
                    let ok = answer.ok;
                    if tx.send(answer).is_err() {
                        return;
                    }
                    let wait = if !ok && !retried {
                        retried = true;
                        RETRY_S.min(u64::from(minutes) * 60)
                    } else {
                        retried = false;
                        u64::from(minutes) * 60
                    };
                    // Wake once a second so a settings change stops it promptly
                    // instead of after a whole interval.
                    let until = std::time::Instant::now() + std::time::Duration::from_secs(wait);
                    while std::time::Instant::now() < until {
                        if stopped.load(Ordering::Relaxed) {
                            return;
                        }
                        std::thread::sleep(std::time::Duration::from_secs(1));
                    }
                }
            })
            .ok();
        self.stop = Some(stop);
        self.rx = Some(rx);
    }

    fn stop_worker(&mut self) {
        if let Some(stop) = self.stop.take() {
            stop.store(true, Ordering::Relaxed);
        }
        self.rx = None;
    }
}

/// One update, as the provider answers it.
///
/// The address is not on any error path: a failure is reported by what went
/// wrong, not by repeating the secret.
fn update_once(url: &str) -> DyndnsAnswer {
    let at = now_unix();
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(std::time::Duration::from_secs(20)))
        .build()
        .new_agent();
    match agent.get(url).call() {
        Ok(mut resp) => {
            let status = resp.status().as_u16();
            let body: String = resp.body_mut().read_to_string().unwrap_or_default();
            let text: String = body.trim().chars().take(60).collect();
            let text = if text.is_empty() { status.to_string() } else { text };
            DyndnsAnswer { at_unix: at, text, ok: answer_is_ok(status, &body) }
        }
        Err(e) => {
            // Only the status or the kind of failure — never the address.
            let text = match e {
                ureq::Error::StatusCode(code) => format!("HTTP {code}"),
                ureq::Error::Timeout(_) => "timeout".to_string(),
                ureq::Error::HostNotFound => "host not found".to_string(),
                ureq::Error::ConnectionFailed => "connection failed".to_string(),
                _ => "request failed".to_string(),
            };
            DyndnsAnswer { at_unix: at, text, ok: false }
        }
    }
}

/// Whether the provider's answer reads as success.
///
/// The words differ by provider — `good` and `nochg` at dynv6, `OK` at DuckDNS,
/// `nochg` at No-IP — so the test is on the refusals rather than on a list of
/// acceptances: anything whose **first word** is a refusal, or any non-2xx
/// status, is a failure, and the operator is shown the word itself either way.
///
/// The first word and not the whole body: a substring test read `koala.example`
/// as `KO` and showed a good update in red.
fn answer_is_ok(status: u16, body: &str) -> bool {
    if !(200..300).contains(&status) {
        return false;
    }
    let low = body.trim().to_ascii_lowercase();
    let first = low.split([' ', '\n', '\r', '\t']).next().unwrap_or("");
    !["ko", "error", "badauth", "nohost", "abuse", "notfqdn", "badagent"]
        .iter()
        .any(|bad| first.starts_with(bad))
}

/// Unix seconds now, without pulling a date library in for it.
fn now_unix() -> i64 {
    crate::engine::unix_now()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The provider's word decides, and the refusals are the ones that
    /// count** — the acceptances differ from provider to provider.
    #[test]
    fn a_refusal_is_recognised_whatever_the_provider_calls_it() {
        for (status, body) in [(200u16, "good"), (200, "nochg"), (200, "OK")] {
            assert!(answer_is_ok(status, body), "{body} at {status} is an acceptance");
        }
        for (status, body) in [
            (200u16, "KO"),
            (401, "badauth"),
            (200, "nohost"),
            (200, "abuse"),
            (200, "notfqdn"),
            (500, "good"),
        ] {
            assert!(!answer_is_ok(status, body), "{body} at {status} is a refusal");
        }
    }

    /// **A good answer is not a refusal for containing the letters.** DuckDNS
    /// answers `good`; a hostname in a body may well read `koala`.
    #[test]
    fn a_body_that_merely_contains_a_refusal_word_is_still_good() {
        assert!(answer_is_ok(200, "good koala.example.org"));
        assert!(answer_is_ok(200, "nochg"));
        assert!(!answer_is_ok(200, "KO 1234567890"), "a refusal as the first word");
    }

    /// **Empty is off**: no thread is started and nothing is ever asked for.
    /// This is the promise that lets the program stay an offline program.
    #[test]
    fn an_empty_address_starts_nothing_at_all() {
        let mut d = Dyndns::default();
        d.set("", 5);
        assert!(d.running.is_none(), "nothing to run");
        assert!(d.stop.is_none(), "no worker was started");
        d.set("   ", 5);
        assert!(d.running.is_none(), "whitespace is empty too");
        assert!(d.poll().is_none(), "and there is no answer to report");
    }

    /// The interval is clamped to something a provider will not refuse: no
    /// address is sent more than once a minute, and never less than daily.
    #[test]
    fn the_interval_is_kept_sane() {
        let mut d = Dyndns::default();
        d.set("https://example.org/update?token=x", 0);
        assert_eq!(d.running.as_ref().map(|(_, m)| *m), Some(1));
        d.set("https://example.org/update?token=x", 99999);
        assert_eq!(d.running.as_ref().map(|(_, m)| *m), Some(1440), "a day at most");
    }
}
