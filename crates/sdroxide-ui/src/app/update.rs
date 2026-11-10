//! The **update check**: ask the fork's GitHub Releases for the latest full
//! release and, if it is newer than this build, say so once with a banner.
//!
//! Only **releases** count. The endpoint is `/releases/latest`, which GitHub
//! answers with the newest non-draft, non-pre-release — so the nightly tag,
//! which is published as a pre-release, is never offered as an update, and a
//! nightly of the same release is not "older" either. The comparison itself is
//! [`sdroxide_version::is_newer_release`], which reduces both sides to their
//! leading three numbers.
//!
//! The request runs on its own thread: it is a blocking HTTP call, and it must
//! never stall the frame that started it. Off unless the operator has it on
//! (`UiSettings::check_for_updates`), so a build that is not asked does not
//! phone home.
//!
//! Native only — the browser has no updater to point at.

#[cfg(not(target_arch = "wasm32"))]
use std::sync::mpsc::{Receiver, TryRecvError};

/// Where a build with an update available sends the operator to download it.
pub const RELEASES_PAGE: &str = "https://github.com/madmedicnl/sdroxide-brown/releases/latest";

/// The result of the startup check. `latest` is the newer release's tag once one
/// has been found, and stays `None` otherwise — nothing to show is the common
/// case, and a network failure is not something to bother the operator with.
#[cfg(not(target_arch = "wasm32"))]
#[derive(Default)]
pub(in crate::app) struct UpdateCheck {
    rx: Option<Receiver<Result<String, String>>>,
    pub(in crate::app) latest: Option<String>,
}

#[cfg(not(target_arch = "wasm32"))]
impl UpdateCheck {
    /// Start the check on a worker thread. Called once, at startup, where the
    /// operator has asked for it.
    pub(in crate::app) fn start() -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let _ = tx.send(sdroxide_config::latest_release_tag());
        });
        Self { rx: Some(rx), latest: None }
    }

    /// Pick up the answer when it arrives. Cheap every frame; the channel is
    /// dropped the moment it has been read, so there is no busy work after.
    pub(in crate::app) fn poll(&mut self) {
        let Some(rx) = self.rx.take() else { return };
        match rx.try_recv() {
            Ok(Ok(tag)) => {
                if sdroxide_version::is_newer_release(&tag, sdroxide_version::VERSION) {
                    self.latest = Some(tag);
                }
            }
            // A failed check is silent: the operator asked whether there is a
            // new version, not to be told the network was down.
            Ok(Err(_)) => {}
            Err(TryRecvError::Empty) => self.rx = Some(rx),
            Err(TryRecvError::Disconnected) => {}
        }
    }
}
