//! Loading and saving the app's own on-disk state.
//!
//! Everything here exists twice, once per target. Natively it goes through
//! [`sdroxide_config`], which writes real files next to the rest of the
//! configuration; in the browser there is no filesystem, so the same state
//! lives in eframe's storage (or, where it is bundled data, nowhere at all).

use sdroxide_types::{QsoRecord, RecordingJob, SwlEntry, SwlReportPrefs};

// ── Logbook persistence (native: config-dir JSON; wasm: eframe storage) ──────
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn load_qso_log(_storage: Option<&dyn eframe::Storage>) -> Vec<QsoRecord> {
    sdroxide_config::load_qso_log()
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn load_qso_log(storage: Option<&dyn eframe::Storage>) -> Vec<QsoRecord> {
    storage.and_then(|s| eframe::get_value(s, "qso_log")).unwrap_or_default()
}

/// Re-read the logbook before a QSO that arrived from the engine is appended,
/// or `None` when there is **no store to re-read**.
///
/// The difference between "the store says nothing" and "there is no store" is
/// the whole point. On this machine the logbook is a JSON file beside the rest
/// of the configuration, so it can always be re-read — that is what keeps two
/// radio tabs from writing each other's contacts away. In the browser the
/// logbook lives in eframe's storage, and the call site in the event loop has
/// no `Storage` to offer it: asking anyway answered "empty", which wiped the
/// log down to the contact just logged. A three-radio station in a browser
/// exported a logbook of exactly one QSO (issue #236) — and the export itself
/// was innocent, which is why the same button on a native single-radio station
/// exported all of them.
pub(in crate::app) fn reload_qso_log(
    #[allow(unused_variables)] storage: Option<&dyn eframe::Storage>,
) -> Option<Vec<QsoRecord>> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        Some(sdroxide_config::load_qso_log())
    }
    #[cfg(target_arch = "wasm32")]
    {
        storage.and_then(|s| eframe::get_value(s, "qso_log"))
    }
}

/// **The logbook after a shared-log reload**: what the store gave back, or the
/// log in hand when there was no store to ask.
///
/// Taken as a list rather than a bool so the rule can be tested without a
/// filesystem, a browser or an event loop — the fault being fixed here was
/// invisible to every one of them until a second radio tab existed.
pub(in crate::app) fn after_shared_log(
    current: Vec<QsoRecord>,
    reloaded: Option<Vec<QsoRecord>>,
) -> Vec<QsoRecord> {
    reloaded.unwrap_or(current)
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn persist_qso_log(log: &[QsoRecord]) {
    if let Err(e) = sdroxide_config::save_qso_log(log) {
        eprintln!("failed to save logbook: {e}");
    }
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn persist_qso_log(_log: &[QsoRecord]) {
    // Written by eframe's periodic `save()` into localStorage.
}

// ── Wavelog download cursors (native only) ───────────────────────────────────
//
// Beside the logbook they describe. Downloading needs the engine in this
// process, and a browser client never has it, so there is no browser half: it
// starts from nothing and never writes.

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn load_wavelog_sync() -> sdroxide_types::WavelogSyncState {
    sdroxide_config::load_wavelog_sync()
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn load_wavelog_sync() -> sdroxide_types::WavelogSyncState {
    sdroxide_types::WavelogSyncState::default()
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn persist_wavelog_sync(state: &sdroxide_types::WavelogSyncState) {
    if let Err(e) = sdroxide_config::save_wavelog_sync(state) {
        eprintln!("failed to save the Wavelog sync position: {e}");
    }
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn persist_wavelog_sync(_state: &sdroxide_types::WavelogSyncState) {}

// ── Reception log persistence (native: config-dir JSON; wasm: eframe storage) ─
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn load_swl_log(_storage: Option<&dyn eframe::Storage>) -> Vec<SwlEntry> {
    sdroxide_config::load_swl_log()
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn load_swl_log(storage: Option<&dyn eframe::Storage>) -> Vec<SwlEntry> {
    storage.and_then(|s| eframe::get_value(s, "swl_log")).unwrap_or_default()
}

/// The report preferences — **native only, and that is the whole of the split**.
///
/// `picture` is a path on this machine's disk, and a browser tab has no disk to
/// name. So in the browser the report carries no picture and the Settings row
/// says so, rather than storing a string that could never resolve to a file.
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn load_swl_report() -> SwlReportPrefs {
    sdroxide_config::load_swl_report()
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn load_swl_report() -> SwlReportPrefs {
    SwlReportPrefs::default()
}

/// Persist the report preferences; see [`load_swl_report`] for the native-only
/// split and why.
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn persist_swl_report(prefs: &SwlReportPrefs) {
    if let Err(e) = sdroxide_config::save_swl_report(prefs) {
        eprintln!("failed to save reception-report settings: {e}");
    }
}

/// Nothing to persist in the browser — see [`load_swl_report`].
#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn persist_swl_report(_prefs: &SwlReportPrefs) {}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn persist_swl_log(log: &[SwlEntry]) {
    if let Err(e) = sdroxide_config::save_swl_log(log) {
        eprintln!("failed to save reception log: {e}");
    }
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn persist_swl_log(_log: &[SwlEntry]) {
    // Written by eframe's periodic `save()` into localStorage.
}

// ── Recordings directory (native: config.toml; wasm: no engine, no path) ─────
/// Where recordings go, for the General tab to show. Resolved but **not
/// created** — the engine makes the directory when a recording actually starts,
/// so opening Settings does not litter the operator's music folder. Empty in
/// the browser, which has no engine to record with.
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn recordings_dir_for_display() -> std::path::PathBuf {
    sdroxide_config::recordings_dir_path().unwrap_or_default()
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn recordings_dir_for_display() -> std::path::PathBuf {
    std::path::PathBuf::new()
}

// ── Broadcast favourites (native: config-dir JSON; wasm: eframe storage) ─────
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn load_broadcast_favourites(
    _storage: Option<&dyn eframe::Storage>,
) -> Vec<String> {
    sdroxide_config::load_broadcast_favourites()
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn load_broadcast_favourites(
    storage: Option<&dyn eframe::Storage>,
) -> Vec<String> {
    storage.and_then(|s| eframe::get_value(s, "broadcast_favourites")).unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn persist_broadcast_favourites(names: &[String]) {
    if let Err(e) = sdroxide_config::save_broadcast_favourites(names) {
        eprintln!("failed to save broadcast favourites: {e}");
    }
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn persist_broadcast_favourites(_names: &[String]) {
    // Written by eframe's periodic `save()` into localStorage.
}

// ── HFDL custom channels (native: config-dir JSON; wasm: eframe storage) ─────
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn load_hfdl_channels(_storage: Option<&dyn eframe::Storage>) -> Vec<u32> {
    sdroxide_config::load_hfdl_channels()
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn load_hfdl_channels(storage: Option<&dyn eframe::Storage>) -> Vec<u32> {
    storage.and_then(|s| eframe::get_value(s, "hfdl_channels")).unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn persist_hfdl_channels(khz: &[u32]) {
    if let Err(e) = sdroxide_config::save_hfdl_channels(khz) {
        eprintln!("failed to save HFDL channels: {e}");
    }
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn persist_hfdl_channels(_khz: &[u32]) {
    // HFDL is a native wideband lane — the browser client never reaches this.
}

// ── Recording jobs (native: config-dir JSON; wasm: eframe storage) ───────────
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn load_recording_jobs(
    _storage: Option<&dyn eframe::Storage>,
) -> Vec<RecordingJob> {
    sdroxide_config::load_recording_jobs()
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn load_recording_jobs(
    storage: Option<&dyn eframe::Storage>,
) -> Vec<RecordingJob> {
    storage.and_then(|s| eframe::get_value(s, "recording_jobs")).unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn persist_recording_jobs(jobs: &[RecordingJob]) {
    if let Err(e) = sdroxide_config::save_recording_jobs(jobs) {
        eprintln!("failed to save recording jobs: {e}");
    }
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn persist_recording_jobs(_jobs: &[RecordingJob]) {
    // Written by eframe's periodic `save()` into localStorage.
}

// ── UI/display preferences (native: config.toml [ui]; wasm: eframe storage) ──
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn load_ui_settings(
    _storage: Option<&dyn eframe::Storage>,
) -> sdroxide_types::UiSettings {
    sdroxide_config::load_ui_settings()
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn load_ui_settings(
    storage: Option<&dyn eframe::Storage>,
) -> sdroxide_types::UiSettings {
    storage.and_then(|s| eframe::get_value(s, "ui_settings")).unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn persist_ui_settings(ui: &sdroxide_types::UiSettings) {
    if let Err(e) = sdroxide_config::save_ui_settings(ui) {
        eprintln!("failed to save UI settings: {e}");
    }
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn persist_ui_settings(_ui: &sdroxide_types::UiSettings) {
    // Written by eframe's periodic `save()` into localStorage.
}

// ── Spoken announcements (native: config.toml [speech]) ─────────────────────
//
// A client-side preference like `[ui]`, so it has a browser half too — the
// wasm build carries the announcer with a null sink, and remembering the
// settings there means a later browser backend inherits them rather than
// starting from defaults.

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn load_speech_settings(
    _storage: Option<&dyn eframe::Storage>,
) -> sdroxide_types::SpeechSettings {
    sdroxide_config::load_speech_settings()
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn load_speech_settings(
    storage: Option<&dyn eframe::Storage>,
) -> sdroxide_types::SpeechSettings {
    storage.and_then(|s| eframe::get_value(s, "speech_settings")).unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn persist_speech_settings(cfg: &sdroxide_types::SpeechSettings) {
    if let Err(e) = sdroxide_config::save_speech_settings(cfg) {
        eprintln!("failed to save speech settings: {e}");
    }
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn persist_speech_settings(_cfg: &sdroxide_types::SpeechSettings) {
    // Written by eframe's periodic `save()` into localStorage.
}

// ── Audible alerts (native: config.toml [alerts]) ────────────────────────────
//
// A client-side preference like `[speech]` and `[ui]`, with the same browser
// half: the wasm build has no alarm sink yet, but remembering the settings
// there means a later browser backend inherits them rather than starting from
// defaults.

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn load_alerts_settings(
    _storage: Option<&dyn eframe::Storage>,
) -> sdroxide_types::AlertSettings {
    sdroxide_config::load_alerts_settings()
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn load_alerts_settings(
    storage: Option<&dyn eframe::Storage>,
) -> sdroxide_types::AlertSettings {
    storage.and_then(|s| eframe::get_value(s, "alerts_settings")).unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn persist_alerts_settings(cfg: &sdroxide_types::AlertSettings) {
    if let Err(e) = sdroxide_config::save_alerts_settings(cfg) {
        eprintln!("failed to save alerts settings: {e}");
    }
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn persist_alerts_settings(_cfg: &sdroxide_types::AlertSettings) {
    // Written by eframe's periodic `save()` into localStorage.
}

// ── Morse trainer progress ────────────────────────────────────────────────────

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn load_morse_progress(
    _storage: Option<&dyn eframe::Storage>,
) -> sdroxide_types::MorseProgress {
    sdroxide_config::load_morse_progress()
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn load_morse_progress(
    storage: Option<&dyn eframe::Storage>,
) -> sdroxide_types::MorseProgress {
    storage.and_then(|s| eframe::get_value(s, "morse_progress")).unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn persist_morse_progress(progress: &sdroxide_types::MorseProgress) {
    if let Err(e) = sdroxide_config::save_morse_progress(progress) {
        eprintln!("failed to save Morse progress: {e}");
    }
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn persist_morse_progress(_progress: &sdroxide_types::MorseProgress) {
    // Written by eframe's periodic `save()` into localStorage.
}

// ── Remote-access credentials (native: config.toml [remote_access]) ──────────
//
// Who may connect to *this* machine's server. There is no browser half: these
// are a file on the machine the radio is attached to, and a browser client is
// by definition not it — the General tab hides the editor rather than offering
// one that writes nowhere.

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn load_remote_access() -> sdroxide_types::RemoteAccess {
    sdroxide_config::load_remote_access()
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn load_remote_access() -> sdroxide_types::RemoteAccess {
    sdroxide_types::RemoteAccess::default()
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn persist_remote_access(access: &sdroxide_types::RemoteAccess) {
    if let Err(e) = sdroxide_config::save_remote_access(access) {
        eprintln!("failed to save the remote-access credentials: {e}");
    }
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn persist_remote_access(_access: &sdroxide_types::RemoteAccess) {}

// ── Remote server address (native: config.toml [remote_server]) ──────────────
//
// The other direction: which station *this* screen dials, from Settings →
// Remote. No browser half at all, unlike everything above — a browser client is
// already attached to the server that served it and has nowhere to put a second
// connection, so that tab does not exist there and there is nothing to remember.

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn load_remote_server() -> sdroxide_types::RemoteServer {
    sdroxide_config::load_remote_server()
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn persist_remote_server(server: &sdroxide_types::RemoteServer) {
    if let Err(e) = sdroxide_config::save_remote_server(server) {
        eprintln!("failed to save the remote server address: {e}");
    }
}

// ── Broadcast stations ───────────────────────────────────────────────────────
//
// Native: the cached season schedule (or the compiled-in one until a download
// lands), plus the operator's own entries. Wasm: the compiled-in schedule, since
// the browser tab has nowhere to cache a download and no config file to overlay.

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn load_broadcast_stations() -> Vec<sdroxide_types::BroadcastStation> {
    sdroxide_config::load_broadcast_master()
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn load_broadcast_stations() -> Vec<sdroxide_types::BroadcastStation> {
    sdroxide_types::broadcast::builtin().to_vec()
}

// ── Custom filter presets (native: config-dir JSON; wasm: eframe storage) ──

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn load_filter_presets(
    _storage: Option<&dyn eframe::Storage>,
) -> Vec<sdroxide_types::CustomFilterPreset> {
    sdroxide_config::load_filter_presets()
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn load_filter_presets(
    storage: Option<&dyn eframe::Storage>,
) -> Vec<sdroxide_types::CustomFilterPreset> {
    storage.and_then(|s| eframe::get_value(s, "filter_presets")).unwrap_or_default()
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn persist_filter_presets(presets: &[sdroxide_types::CustomFilterPreset]) {
    if let Err(e) = sdroxide_config::save_filter_presets(presets) {
        eprintln!("failed to save filter presets: {e}");
    }
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn persist_filter_presets(_presets: &[sdroxide_types::CustomFilterPreset]) {
    // Written with the rest of the browser's storage in `App::save`.
}

// ── Band conditions ──────────────────────────────────────────────────────────
//
// Fetched here rather than by the 3D view, and this is the one product of the
// solar feed that works that way. The verdicts colour the band menu, which is
// on screen for the whole session — waiting for a window most operators never
// open would have left the feature blank for them.
//
// It is still one request an hour: this shares the solar disk cache and its
// validators with the feed's own copy, so with the 3D view open the second of
// the two is a conditional GET that comes back 304. The publisher asks for
// hourly at most and that is what is honoured.

/// Fetch (or reuse) the published band conditions on a worker thread.
///
/// Off the UI thread because it is a network round trip; the app picks the
/// result up from the receiver on a later frame. `None` means the thread could
/// not be spawned at all — an expired cache and an unreachable server both
/// arrive as `Ok(None)` on the channel instead, and leave the last known
/// verdicts on screen with their age.
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn spawn_band_conditions_fetch()
-> Option<std::sync::mpsc::Receiver<Option<sdroxide_solar::BandConditions>>> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name("band-conditions".into())
        .spawn(move || {
            let _ = tx.send(sdroxide_solar::band_conditions_cached());
        })
        .ok()?;
    Some(rx)
}

/// The browser has no disk cache and no HTTP client of its own; a viewer's
/// verdicts arrive over the solar relay instead.
#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn spawn_band_conditions_fetch()
-> Option<std::sync::mpsc::Receiver<Option<sdroxide_solar::BandConditions>>> {
    None
}

/// Fetch (or reuse) the global WSPR activity on a worker thread.
///
/// The same shape as [`spawn_band_conditions_fetch`], and off the UI thread for
/// the same reason. `None` means the thread could not be spawned; an
/// unreachable database and an expired cache both arrive as `Ok(None)` on the
/// channel instead.
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn spawn_band_activity_fetch()
-> Option<std::sync::mpsc::Receiver<Option<sdroxide_solar::BandActivityTable>>> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name("band-activity".into())
        .spawn(move || {
            let _ = tx.send(sdroxide_solar::band_activity_cached());
        })
        .ok()?;
    Some(rx)
}

/// The browser gets no disk cache and no HTTP client of its own here; the
/// measured column stays empty there rather than guessed.
#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn spawn_band_activity_fetch()
-> Option<std::sync::mpsc::Receiver<Option<sdroxide_solar::BandActivityTable>>> {
    None
}

/// Fetch (or reuse) the global PSK Reporter activity on a worker thread. The
/// activity-mode sibling of [`spawn_band_activity_fetch`].
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn spawn_psk_activity_fetch()
-> Option<std::sync::mpsc::Receiver<Option<sdroxide_solar::BandActivityTable>>> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name("psk-activity".into())
        .spawn(move || {
            let _ = tx.send(sdroxide_solar::psk_activity_cached());
        })
        .ok()?;
    Some(rx)
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn spawn_psk_activity_fetch()
-> Option<std::sync::mpsc::Receiver<Option<sdroxide_solar::BandActivityTable>>> {
    None
}

// ── Schedule download ───────────────────────────────────────────────────────

/// The result of a background schedule download.
pub(in crate::app) type ScheduleFetch = Result<Vec<sdroxide_types::BroadcastStation>, String>;

/// Download the current season's schedule on a worker thread.
///
/// Off the UI thread because it is a megabyte over a link that may not be there;
/// the app picks the result up from the receiver on a later frame. Returns `None`
/// when nothing needs fetching, which after a first run is every start until the
/// season turns over.
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn spawn_schedule_fetch(
    force: bool,
) -> Option<std::sync::mpsc::Receiver<ScheduleFetch>> {
    if !force && !sdroxide_config::broadcast_schedule_due() {
        return None;
    }
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name("broadcast-schedule".into())
        .spawn(move || {
            // **Every** source, not EiBi alone: the master store is the point,
            // and a source that fails keeps its cache rather than cancelling the
            // others.
            let _ = tx.send(sdroxide_config::fetch_broadcast_master());
        })
        .ok()?;
    Some(rx)
}

/// Fetch **one** source, for the Settings page's per-source button.
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn spawn_one_schedule_fetch(
    source: sdroxide_types::ScheduleSource,
) -> Option<std::sync::mpsc::Receiver<ScheduleFetch>> {
    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::Builder::new()
        .name("broadcast-one".into())
        .spawn(move || {
            let _ = tx.send(sdroxide_config::fetch_broadcast_schedule_from(source));
        })
        .ok()?;
    Some(rx)
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn spawn_one_schedule_fetch(
    _source: sdroxide_types::ScheduleSource,
) -> Option<std::sync::mpsc::Receiver<ScheduleFetch>> {
    None
}

/// The browser client has no cache to fill, so there is nothing to fetch.
#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn spawn_schedule_fetch(
    _force: bool,
) -> Option<std::sync::mpsc::Receiver<ScheduleFetch>> {
    None
}

/// Drop the cached schedule so the next fetch downloads it again.
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn clear_broadcast_cache() {
    if let Err(e) = sdroxide_config::clear_broadcast_cache() {
        eprintln!("failed to clear the broadcast schedule cache: {e}");
    }
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn clear_broadcast_cache() {}

// ── The band-plan file's path (shown on the General tab) ─────────────────────

/// Where `bandplan.json` is, so the settings dialog can name the file the
/// operator has to edit. `None` when there is no config directory to point at.
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn band_plan_path() -> Option<std::path::PathBuf> {
    sdroxide_config::band_plan_path()
}

/// The browser client has no filesystem, and the file it would want to name is
/// on the engine's machine in any case.
#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn band_plan_path() -> Option<std::path::PathBuf> {
    None
}

// ── Saved window arrangements (SDRuno's workspaces) ──────────────────────────

/// The saved window arrangements. Natively a shared `workspaces.json` beside the
/// rest of the configuration; the browser has no windows to arrange, so its list
/// is empty and the settings section that edits it is not drawn.
#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn load_workspaces() -> Vec<sdroxide_types::Workspace> {
    sdroxide_config::load_workspaces()
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn load_workspaces() -> Vec<sdroxide_types::Workspace> {
    Vec::new()
}

#[cfg(not(target_arch = "wasm32"))]
pub(in crate::app) fn save_workspaces(workspaces: &Vec<sdroxide_types::Workspace>) {
    if let Err(e) = sdroxide_config::save_workspaces(workspaces) {
        eprintln!("failed to save workspaces: {e}");
    }
}

#[cfg(target_arch = "wasm32")]
pub(in crate::app) fn save_workspaces(_workspaces: &Vec<sdroxide_types::Workspace>) {}

#[cfg(test)]
mod tests {
    use super::*;

    fn qso(call: &str) -> QsoRecord {
        QsoRecord { call: call.into(), ..QsoRecord::default() }
    }

    /// **A reload with no store to read must leave the logbook in hand alone.**
    ///
    /// This is the shape of the fault behind issue #236. A station with more
    /// than one radio shares one logbook between the radio tabs, so a contact
    /// logged by the engine is appended only after the log has been re-read —
    /// which keeps one tab from writing another's contacts away. In a browser
    /// there is no file to re-read, and the question "what does the store say?"
    /// was answered "nothing": the log was replaced by an empty list and the one
    /// contact just logged was pushed onto it. An export of a session then held
    /// exactly one QSO, while the same button on a native single-radio station
    /// held every one of them.
    #[test]
    fn a_logbook_is_not_wiped_by_a_reload_that_had_no_store_to_read() {
        let log = vec![qso("ON6CQ"), qso("DL1ABC"), qso("F4NRM")];
        assert_eq!(after_shared_log(log.clone(), None), log, "no store is not an empty logbook");
        // A store that *does* answer still replaces it: that is the point of the
        // reload, so a contact logged on another tab is not written away.
        let from_store = vec![qso("ON6CQ"), qso("DL1ABC"), qso("F4NRM"), qso("G4XYZ")];
        assert_eq!(after_shared_log(log, Some(from_store.clone())), from_store);
        // A first run really is an empty logbook, and that still works.
        assert!(after_shared_log(Vec::new(), Some(Vec::new())).is_empty());
    }
}
