//! A generic "poll an HTTP endpoint on an interval" worker thread, shared by
//! the POTA, SOTA and PSK Reporter feeds. Shutdown is immediate (a control
//! channel with `recv_timeout` used as the interval sleep).

use std::thread::JoinHandle;
use std::time::Duration;

use crossbeam_channel::{Receiver, Sender};
use sdroxide_types::{Spot, SpotKind};

use crate::event::{EventTx, FeedTx, NetEvent};

/// A running poller. Dropping it stops the thread promptly.
pub struct PollHandle {
    stop: Sender<()>,
    join: Option<JoinHandle<()>>,
}

impl Drop for PollHandle {
    fn drop(&mut self) {
        let _ = self.stop.send(());
        if let Some(j) = self.join.take() {
            let _ = j.join();
        }
    }
}

/// Spawn a poller that calls `fetch` every `interval`, sending the resulting
/// spots (full current set for `kind`) to `feed`. `fetch` returns
/// `Ok(spots)` or `Err(message)` (surfaced as a status line, throttled).
pub fn spawn<F>(
    name: &'static str,
    kind: SpotKind,
    interval: Duration,
    feed: FeedTx,
    events: EventTx,
    fetch: F,
) -> PollHandle
where
    F: Fn() -> Result<Vec<Spot>, String> + Send + 'static,
{
    let (stop_tx, stop_rx) = crossbeam_channel::bounded::<()>(1);
    let interval = interval.max(Duration::from_secs(15));
    let join = std::thread::Builder::new()
        .name(name.into())
        .spawn(move || run(kind, interval, feed, events, fetch, stop_rx))
        .ok();
    PollHandle { stop: stop_tx, join }
}

fn run<F>(
    kind: SpotKind,
    interval: Duration,
    feed: FeedTx,
    events: EventTx,
    fetch: F,
    stop: Receiver<()>,
) where
    F: Fn() -> Result<Vec<Spot>, String>,
{
    let mut last_err: Option<String> = None;
    loop {
        match fetch() {
            Ok(spots) => {
                if last_err.take().is_some() {
                    let _ = events.send(NetEvent::Status(None));
                }
                let _ = feed.send((kind, spots));
            }
            Err(e) => {
                if last_err.as_deref() != Some(e.as_str()) {
                    let _ =
                        events.send(NetEvent::Status(Some(format!("{} feed: {e}", kind.label()))));
                    last_err = Some(e);
                }
            }
        }
        // Sleep for `interval`, waking immediately on shutdown.
        match stop.recv_timeout(interval) {
            Ok(()) | Err(crossbeam_channel::RecvTimeoutError::Disconnected) => return,
            Err(crossbeam_channel::RecvTimeoutError::Timeout) => {}
        }
    }
}
