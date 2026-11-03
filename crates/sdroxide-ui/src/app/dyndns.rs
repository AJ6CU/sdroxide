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
//! The URL carries the token, so it is a secret: it is never written to a log
//! and never shown back in a message. What the screen shows is the provider's
//! **answer** (`good`, `nochg`, `KO`, an HTTP status), which is the only part
//! that is not a secret.
//!
//! Native only — a browser build has no such address to keep.

#[cfg(not(target_arch = "wasm32"))]
use std::sync::atomic::{AtomicBool, Ordering};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::mpsc::{Receiver, TryRecvError};
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;

/// The provider's answer, as the operator reads it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::app) struct DyndnsAnswer {
    /// When the update was answered, unix seconds.
    pub at_unix: i64,
    /// The provider's own word, trimmed and shortened. Never the address.
    pub text: String,
    /// Whether the provider accepted it.
    pub ok: bool,
}

/// The address as it may be shown: scheme and host, the rest hidden.
///
/// Every provider puts its token in the path or the query, so neither is ever
/// printed. What is left is enough for the operator to recognise *which*
/// address is in use without the secret being on a screenshot.
pub(in crate::app) fn mask_url(url: &str) -> String {
    let url = url.trim();
    if url.is_empty() {
        return String::new();
    }
    match url.split_once("://") {
        Some((scheme, rest)) => {
            let host = rest.split(['/', '?']).next().unwrap_or(rest);
            format!("{scheme}://{host}/…")
        }
        // No scheme: keep nothing but the host, if it looks like one.
        None => {
            let host = url.split(['/', '?']).next().unwrap_or(url);
            format!("{host}/…")
        }
    }
}

/// Whether the provider's answer reads as success.
///
/// The words differ by provider — `good` and `nochg` at dynv6, `OK` at DuckDNS,
/// `nochg` at No-IP — so the test is on the refusals rather than on a list of
/// acceptances: anything that says it failed, or any non-2xx status, is a
/// failure and the operator is shown the word itself either way.
fn answer_is_ok(status: u16, body: &str) -> bool {
    if !(200..300).contains(&status) {
        return false;
    }
    let low = body.to_ascii_lowercase();
    !["ko", "error", "badauth", "nohost", "abuse", "notfqdn", "badagent"]
        .iter()
        .any(|bad| low.trim().starts_with(bad) || low.contains(bad))
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
pub(in crate::app) struct Dyndns {
    /// The configuration the running thread was started for, so a change of
    /// address or interval stops it and starts another.
    running: Option<(String, u32)>,
    stop: Option<Arc<AtomicBool>>,
    rx: Option<Receiver<DyndnsAnswer>>,
    last: Option<DyndnsAnswer>,
}

#[cfg(not(target_arch = "wasm32"))]
impl Dyndns {
    /// Keep the worker in step with the settings, and pick up its last answer.
    ///
    /// Cheap every frame: the common case is one comparison of two strings.
    pub(in crate::app) fn poll(&mut self, url: &str, minutes: u32) {
        let wanted = url.trim().to_string();
        let wanted = if wanted.is_empty() { None } else { Some((wanted, minutes.max(1))) };
        if self.running != wanted {
            self.stop_worker();
            self.running = wanted.clone();
            if let Some((url, minutes)) = wanted {
                self.start_worker(url, minutes);
            }
        }
        let Some(rx) = self.rx.take() else { return };
        loop {
            match rx.try_recv() {
                Ok(answer) => self.last = Some(answer),
                Err(TryRecvError::Empty) => {
                    self.rx = Some(rx);
                    return;
                }
                Err(TryRecvError::Disconnected) => return,
            }
        }
    }

    /// The last answer, for the settings row.
    pub(in crate::app) fn last(&self) -> Option<&DyndnsAnswer> {
        self.last.as_ref()
    }

    fn start_worker(&mut self, url: String, minutes: u32) {
        let (tx, rx) = std::sync::mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let stopped = stop.clone();
        std::thread::Builder::new()
            .name("sdroxide-dyndns".into())
            .spawn(move || {
                while !stopped.load(Ordering::Relaxed) {
                    let answer = update_once(&url);
                    if tx.send(answer).is_err() {
                        return;
                    }
                    // Wake once a second so a settings change stops it promptly
                    // instead of after a whole interval.
                    let until = std::time::Instant::now()
                        + std::time::Duration::from_secs(u64::from(minutes) * 60);
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
#[cfg(not(target_arch = "wasm32"))]
fn update_once(url: &str) -> DyndnsAnswer {
    let at = crate::time::now_unix();
    let agent = ureq::Agent::config_builder()
        .timeout_global(Some(std::time::Duration::from_secs(20)))
        .build()
        .new_agent();
    match agent.get(url).call() {
        Ok(mut resp) => {
            let status = resp.status().as_u16();
            let body: String = resp.body_mut().read_to_string().unwrap_or_default();
            let text: String = body.trim().chars().take(60).collect();
            DyndnsAnswer { at_unix: at, text: if text.is_empty() { status.to_string() } else { text }, ok: answer_is_ok(status, &body) }
        }
        Err(e) => {
            // Only the status or the kind of failure — never the address.
            let text = match e {
                ureq::Error::StatusCode(code) => format!("HTTP {code}"),
                other => format!("{}", match other {
                    ureq::Error::Timeout(_) => "timeout".to_string(),
                    ureq::Error::HostNotFound => "host not found".to_string(),
                    ureq::Error::ConnectionFailed => "connection failed".to_string(),
                    _ => "request failed".to_string(),
                }),
            };
            DyndnsAnswer { at_unix: at, text, ok: false }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **The address never appears in what may be shown.** Every provider puts
    /// its token after the host, in the path or the query, so both go.
    #[test]
    fn the_update_address_is_masked_down_to_its_host() {
        assert_eq!(
            mask_url("https://www.duckdns.org/update?domains=x&token=SECRET"),
            "https://www.duckdns.org/…"
        );
        assert_eq!(
            mask_url("https://dynv6.com/api/update?hostname=x&token=SECRET"),
            "https://dynv6.com/…"
        );
        assert_eq!(mask_url("  "), "", "nothing to mask, nothing shown");
        assert_eq!(mask_url("example.org/nic/update?host=x"), "example.org/…");
    }

    /// The provider's word decides, and the refusals are the ones that count —
    /// the acceptances differ from provider to provider.
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
            (500, "good"),
        ] {
            assert!(!answer_is_ok(status, body), "{body} at {status} is a refusal");
        }
    }

    /// **Empty is off**: no thread is started and nothing is ever asked for.
    /// This is the promise that lets the program stay an offline program.
    #[test]
    fn an_empty_address_starts_nothing_at_all() {
        let mut d = Dyndns::default();
        d.poll("", 5);
        assert!(d.running.is_none(), "nothing to run");
        assert!(d.stop.is_none(), "no worker was started");
        d.poll("   ", 5);
        assert!(d.running.is_none(), "whitespace is empty too");
    }
}
