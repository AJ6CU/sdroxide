//! The network cockpit's outbound half: callsign lookup and QSO upload.
//!
//! Lookups and uploads are queued here and drained into [`Command`]s by the
//! frame loop, so nothing in the UI ever blocks on the network. What comes
//! back is applied to the in-progress QSO and appended to the rolling log the
//! SPOTS window shows.

use crate::i18n::tr;
use std::collections::HashMap;
use std::panic::{AssertUnwindSafe, catch_unwind};

use sdroxide_types::{
    CallsignInfo, Command, LookupProvider, NetworkConfig, QsoRecord, UploadResult, UploadTarget,
};

use crate::app::SdroxideApp;
use crate::app::persist::persist_qso_log;

/// Wasm-safe wall clock — `SystemTime::now()` panics in the browser (#262/#297).
fn now_unix() -> i64 {
    crate::time::now_unix()
}

/// Least time between two locator lookups driven by a heard list.
const GRID_LOOKUP_INTERVAL_S: f64 = 1.5;

/// Fill `dst` from `src` only when `dst` is blank and `src` is non-empty;
/// returns whether it changed anything.
fn fill_blank(dst: &mut String, src: Option<&str>) -> bool {
    if dst.trim().is_empty() {
        if let Some(v) = src.map(str::trim) {
            if !v.is_empty() {
                *dst = v.to_string();
                return true;
            }
        }
    }
    false
}

/// Which upload targets are enabled for auto-upload in `cfg`.
fn auto_upload_targets(cfg: &NetworkConfig) -> Vec<UploadTarget> {
    let mut t = Vec::new();
    if cfg.auto_upload_eqsl {
        t.push(UploadTarget::Eqsl);
    }
    if cfg.auto_upload_qrz {
        t.push(UploadTarget::QrzLogbook);
    }
    if cfg.auto_upload_hamqth {
        t.push(UploadTarget::HamQth);
    }
    if cfg.auto_upload_clublog {
        t.push(UploadTarget::ClubLog);
    }
    if cfg.auto_upload_wrl {
        t.push(UploadTarget::Wrl);
    }
    if cfg.auto_upload_log11dx {
        t.push(UploadTarget::Log11Dx);
    }
    if cfg.wavelog.auto_upload && cfg.wavelog.is_configured() && cfg.wavelog.station_id != 0 {
        t.push(UploadTarget::Wavelog);
    }
    t
}

/// Upload targets that have credentials configured (for the manual per-QSO
/// upload button).
pub(in crate::app) fn configured_upload_targets(cfg: &NetworkConfig) -> Vec<UploadTarget> {
    let mut t = Vec::new();
    if !cfg.eqsl.user.trim().is_empty() {
        t.push(UploadTarget::Eqsl);
    }
    if !cfg.qrz_logbook_key.trim().is_empty() {
        t.push(UploadTarget::QrzLogbook);
    }
    // The same account as the callsign lookup — a HamQTH user who filled in the
    // lookup half has already configured the upload half.
    if !cfg.hamqth.user.trim().is_empty() && !cfg.hamqth.password.trim().is_empty() {
        t.push(UploadTarget::HamQth);
    }
    if !cfg.clublog.user.trim().is_empty() && !cfg.clublog_api_key.trim().is_empty() {
        t.push(UploadTarget::ClubLog);
    }
    if !cfg.wrl_api_key.trim().is_empty() {
        t.push(UploadTarget::Wrl);
    }
    if !cfg.log11dx_api_token.trim().is_empty() {
        t.push(UploadTarget::Log11Dx);
    }
    if cfg.wavelog.is_configured() {
        t.push(UploadTarget::Wavelog);
    }
    t
}

/// Single-record ADIF + targets for auto-upload of a freshly logged QSO, or
/// `None` when auto-upload is off or no target is enabled.
pub(in crate::app) fn auto_upload_adif(
    cfg: &NetworkConfig,
    rec: &QsoRecord,
) -> Option<(u64, String, Vec<UploadTarget>)> {
    if !cfg.auto_upload {
        return None;
    }
    let targets = auto_upload_targets(cfg);
    if targets.is_empty() {
        return None;
    }
    Some((rec.id, sdroxide_types::qso_log_to_adif(std::slice::from_ref(rec)), targets))
}

impl SdroxideApp {
    /// Queue an auto-lookup if a provider + auto-lookup are configured.
    ///
    /// Returns whether one was actually queued, so a caller that rations
    /// lookups can tell "asked" from "lookup is switched off" and not burn its
    /// one-per-interval budget on a request that never left.
    pub(in crate::app) fn queue_lookup(&mut self, call: String) -> bool {
        let call = call.trim().to_string();
        if call.is_empty()
            || !self.net_cfg_edit.auto_lookup
            || self.net_cfg_edit.lookup_provider == LookupProvider::None
        {
            return false;
        }
        self.pending_lookups.push(call);
        true
    }

    /// Whether another heard-list locator lookup may go out yet.
    ///
    /// Rationed across every mode that places stations by callsign rather than
    /// per panel: they share one cache and one network, and a busy band would
    /// otherwise put fifty requests on fifty threads the moment a panel opens.
    pub(in crate::app) fn grid_lookup_due(&self, now_t: f64) -> bool {
        now_t - self.grid_lookup_at >= GRID_LOOKUP_INTERVAL_S
    }

    /// Ask where a heard station is, and remember having asked.
    pub(in crate::app) fn queue_grid_lookup(&mut self, call: Option<String>, now_t: f64) {
        let Some(call) = call else { return };
        // Only spend the interval on a request that actually left: with no
        // provider configured this must stay ready for the moment one is.
        if self.queue_lookup(call.clone()) {
            self.grid_looked_up.insert(call);
            self.grid_lookup_at = now_t;
        }
    }

    /// Merge a callsign-lookup result into the open log entry and, if none
    /// matches, into the most recent logged QSO for that call — filling only
    /// blank fields.
    pub(in crate::app) fn apply_callsign(&mut self, info: CallsignInfo) {
        let mut summary = info.call.clone();
        if let Some(n) = &info.name {
            summary.push_str(&format!(" — {n}"));
        }
        if let Some(q) = &info.qth {
            summary.push_str(&format!(", {q}"));
        }
        if let Some(c) = &info.country {
            summary.push_str(&format!(" ({c})"));
        }
        self.push_net_log(summary);
        // Keep the whole record: the JS8 map places stations by the grid in it,
        // and that station may never have a log entry to merge into.
        self.callsign_cache.insert(info.call.to_ascii_uppercase(), info.clone());

        // 1) The open entry form, if it's for this call.
        if let Some(f) = self.log_edit.as_mut() {
            if f.call.trim().eq_ignore_ascii_case(&info.call) {
                fill_blank(&mut f.name, info.name.as_deref());
                fill_blank(&mut f.qth, info.qth.as_deref());
                fill_blank(&mut f.grid, info.grid.as_deref());
                fill_blank(&mut f.state, info.state.as_deref());
                fill_blank(&mut f.country, info.country.as_deref());
                return;
            }
        }
        // 2) Otherwise, enrich the most recent logged QSO with that call.
        if let Some(rec) = self
            .qso_log
            .iter_mut()
            .filter(|q| q.call.eq_ignore_ascii_case(&info.call))
            .max_by_key(|q| q.start_utc)
        {
            let mut changed = false;
            changed |= fill_blank(&mut rec.name, info.name.as_deref());
            changed |= fill_blank(&mut rec.qth, info.qth.as_deref());
            changed |= fill_blank(&mut rec.state, info.state.as_deref());
            changed |= fill_blank(&mut rec.country, info.country.as_deref());
            if rec.grid.as_deref().unwrap_or("").is_empty() {
                if let Some(g) = &info.grid {
                    rec.grid = Some(g.clone());
                    changed = true;
                }
            }
            if rec.dxcc.is_none() && info.dxcc.is_some() {
                rec.dxcc = info.dxcc;
                changed = true;
            }
            if rec.cq_zone.is_none() && info.cq_zone.is_some() {
                rec.cq_zone = info.cq_zone;
                changed = true;
            }
            if rec.itu_zone.is_none() && info.itu_zone.is_some() {
                rec.itu_zone = info.itu_zone;
                changed = true;
            }
            if changed {
                persist_qso_log(&self.qso_log);
                self.log_content_changed();
            }
        }
    }

    /// Record an upload result and mark the QSO's sent flag on success.
    pub(in crate::app) fn on_upload_result(&mut self, r: UploadResult) {
        let status = if r.ok { "OK" } else { &tr!("net-fail") };
        self.push_net_log(format!("{} → {}: {}", r.target.label(), status, r.message));
        // Keep the outcome for the panel to show beside a completed QSO: this
        // is how an operator learns a logged contact reached LOG11DX and not
        // only the local log. Latest outcome per target wins, so a retry's
        // result replaces the failed first try.
        record_upload_outcome(&mut self.qso_upload_status, r.qso_id, r.target, r.ok);
        if r.ok {
            if let Some(rec) = self.qso_log.iter_mut().find(|q| q.id == r.qso_id) {
                match r.target {
                    UploadTarget::Eqsl => rec.eqsl_sent = true,
                    UploadTarget::QrzLogbook => rec.qrz_sent = true,
                    UploadTarget::ClubLog => rec.clublog_sent = true,
                    UploadTarget::HamQth => rec.hamqth_sent = true,
                    UploadTarget::Wrl => rec.wrl_sent = true,
                    UploadTarget::Log11Dx => rec.log11dx_sent = true,
                    UploadTarget::Wavelog => rec.wavelog_sent = true,
                }
                persist_qso_log(&self.qso_log);
            }
        }
    }

    /// One page of QSOs downloaded from Wavelog.
    ///
    /// Merged as it arrives rather than collected and applied at the end: a
    /// first download can be a whole log, and a run that is interrupted half
    /// way has then already brought in half the contacts and moved the cursor
    /// with them — so the next sync resumes instead of starting again.
    pub(in crate::app) fn on_wavelog_batch(&mut self, b: sdroxide_types::WavelogBatch) {
        if b.done {
            self.wavelog_sync_running = false;
            if let Some(e) = b.error {
                self.wavelog_sync_status = Some(tr!("net-wavelog-stopped", error = e));
                self.push_net_log(tr!("net-wavelog-download-stopped", error = e));
            }
            return;
        }
        let next_id = self.qso_log.iter().map(|q| q.id).max().unwrap_or(0) + 1;
        let merged = sdroxide_types::merge_from_wavelog(&mut self.qso_log, b.records, next_id);
        self.wavelog_sync.advance(b.station_id, b.last_id);
        self.wavelog_sync.last_sync_utc = now_unix();
        crate::app::persist::persist_wavelog_sync(&self.wavelog_sync);
        if merged.changed {
            persist_qso_log(&self.qso_log);
        }
        self.wavelog_sync_status =
            Some(tr!("net-wavelog-new-already-here", added = merged.added, matched = merged.matched));
    }

    /// The network settings as the settings page has them, for a Test button
    /// that must check what the operator has just typed rather than what was
    /// last applied.
    pub(in crate::app) fn net_edit_snapshot(&self) -> NetworkConfig {
        self.net_cfg_edit.clone()
    }

    /// Start a Wavelog download if the settings tab (or the logbook) asked for
    /// one this frame.
    ///
    /// The stations and the cursors come from what the screen holds, which is
    /// why this is here rather than in the engine: the logbook and the sync
    /// position live with the client, and the engine is told only where to
    /// start.
    pub(in crate::app) fn wavelog_tick(&mut self, cmds: &mut Vec<Command>) {
        if !self.wavelog_sync_request.take() || self.wavelog_sync_running {
            return;
        }
        let cfg = &self.net_cfg_edit.wavelog;
        if !cfg.is_configured() {
            self.wavelog_sync_status =
                Some(tr!("net-wavelog-fill-in-first"));
            return;
        }
        let stations = if cfg.download_station_ids.is_empty() {
            // Nothing ticked means the one uploads go to: an operator who set a
            // location up to log to expects it to be the one read back, and
            // asking them to tick the same number twice is a trap.
            vec![cfg.station_id]
        } else {
            cfg.download_station_ids.clone()
        }
        .into_iter()
        .filter(|s| *s != 0)
        .collect::<Vec<_>>();
        if stations.is_empty() {
            self.wavelog_sync_status =
                Some(tr!("net-wavelog-no-station-location"));
            return;
        }
        let starts = self.wavelog_sync.starts(&cfg.url, &stations);
        self.wavelog_sync_running = true;
        self.wavelog_sync_status = Some(tr!("net-downloading"));
        cmds.push(Command::WavelogSync { stations: starts, full: false });
    }

    pub(in crate::app) fn push_net_log(&mut self, line: String) {
        self.net_log.insert(0, line);
        self.net_log.truncate(50);
    }

    /// Drain a pending ADIF import: parse, lightly de-dup against the current
    /// log (same call+band within 2 minutes), append with fresh ids, persist.
    ///
    /// The parse runs under [`catch_unwind`] because this is the frame loop
    /// handing an operator-supplied file to a parser: a file it cannot make
    /// sense of has to cost the import, not the application. Nothing touches
    /// the log until the parse has returned, so a failed one leaves it as it
    /// was.
    ///
    /// That backstop is native only. The browser build aborts on a panic, so
    /// there is nothing for `catch_unwind` to catch and a parser panic takes
    /// the page down with it — which is why the parser's own tests feed it
    /// damaged documents (`adif_import_survives_a_truncated_document` and
    /// `adif_import_survives_tags_that_are_not_tags`).
    pub(in crate::app) fn poll_adif_import(&mut self) {
        let loaded = self.adif_import_inbox.lock().ok().and_then(|mut g| g.take());
        let Some(loaded) = loaded else { return };
        // Whatever went wrong is said here rather than on stderr: the operator
        // who clicked Import is looking at the window, and on Windows there is
        // no console behind it to print to.
        let loaded = match loaded {
            Ok(loaded) => loaded,
            Err(e) => {
                self.push_net_log(tr!("net-adif-import-failed", error = e.to_string()));
                return;
            }
        };
        let text = loaded.text;
        let parsed =
            catch_unwind(AssertUnwindSafe(|| sdroxide_types::adif_to_qso_log_counting_swl(&text)));
        let Ok((records, swl)) = parsed else {
            self.push_net_log(tr!("net-adif-import-failed-file-could").to_string());
            return;
        };
        let (fresh, skipped) = new_import_records(&self.qso_log, records);
        let mut next_id = self.next_log_id();
        let added = fresh.len();
        for mut r in fresh {
            r.id = next_id;
            next_id += 1;
            self.qso_log.push(r);
        }
        if added > 0 {
            persist_qso_log(&self.qso_log);
        }
        // Naming the guessed code page is the point of carrying it this far: a
        // name that came out as nonsense is then a code page to report, not a
        // decoder to doubt.
        let assumed = match loaded.assumed {
            Some(enc) => tr!("net-not-unicode-read", enc = enc),
            None => String::new(),
        };
        // A file of received reports — the decode list's own ADIF export —
        // adds nothing, and saying why beats reporting an empty import.
        let reports = match swl {
            0 => String::new(),
            n => tr!("net-received-reports-swl-left-out", n = n),
        };
        self.push_net_log(tr!(
            "net-adif-import-added-duplicates-skipped",
            added = added,
            skipped = skipped,
            reports = reports,
            assumed = assumed
        ));
    }

    /// Drain a pending channel-list import: parse the CHIRP CSV and hand the
    /// channels to the engine, which owns the memory list and the numbering in
    /// it — including the duplicate check, because only it knows what is
    /// already stored (issue #234).
    ///
    /// Under [`catch_unwind`] for the reason the ADIF import is: this is the
    /// frame loop handing an operator-supplied file to a parser, and a file it
    /// cannot make sense of has to cost the import rather than the application.
    /// Native only, as there: in the browser the parser must simply not panic
    /// (`a_damaged_file_never_panics`).
    pub(in crate::app) fn poll_chirp_import(&mut self, cmds: &mut Vec<Command>) {
        let loaded = self.chirp_import_inbox.lock().ok().and_then(|mut g| g.take());
        let Some(loaded) = loaded else { return };
        let loaded = match loaded {
            Ok(loaded) => loaded,
            Err(e) => {
                self.push_net_log(tr!("net-channel-import-failed", error = e.to_string()));
                return;
            }
        };
        let text = loaded.text;
        let parsed =
            catch_unwind(AssertUnwindSafe(|| sdroxide_types::chirp_csv_to_memories(&text)));
        let Ok((channels, skipped)) = parsed else {
            self.push_net_log(tr!("net-channel-import-failed-file-could").to_string());
            return;
        };
        if channels.is_empty() {
            // The commonest way to get here is a file that is not a CHIRP
            // export at all, so say what was expected rather than "0 imported".
            self.push_net_log(tr!("net-channel-import-nothing-read-chirp").to_string());
            return;
        }
        let n = channels.len();
        cmds.push(Command::ImportMemories(channels));
        let skipped = match skipped {
            0 => String::new(),
            n => tr!("net-line-s-skipped", n = n),
        };
        let assumed = match loaded.assumed {
            Some(enc) => tr!("net-not-unicode-read-2", enc = enc),
            None => String::new(),
        };
        // "read", not "added": the engine drops the channels already stored,
        // and the memory list itself is what shows the result.
        self.push_net_log(tr!(
            "net-channel-import-channel-s-read",
            n = n,
            skipped = skipped,
            assumed = assumed
        ));
    }
}

/// Remember one upload outcome against its QSO and target, replacing any earlier
/// outcome for that pair — a retry's result must supersede the failed first try
/// rather than sit beside it. Kept as a free function so its contract is tested
/// without a running app or a disk.
fn record_upload_outcome(
    status: &mut std::collections::HashMap<u64, Vec<(UploadTarget, bool)>>,
    qso_id: u64,
    target: UploadTarget,
    ok: bool,
) {
    let outcomes = status.entry(qso_id).or_default();
    match outcomes.iter_mut().find(|(t, _)| *t == target) {
        Some(slot) => slot.1 = ok,
        None => outcomes.push((target, ok)),
    }
    // The map is keyed by log id and only ever read for a QSO the panel is
    // still showing, so pruning is only about a long session's memory — and
    // clearing only once past the cap keeps it from churning every frame.
    if status.len() > 64 {
        status.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn qso(call: &str, band: &str, start_utc: i64) -> sdroxide_types::QsoRecord {
        sdroxide_types::QsoRecord {
            call: call.into(),
            band: band.into(),
            mode: "FT8".into(),
            start_utc,
            ..Default::default()
        }
    }

    /// **A contact already in the log is not imported a second time, and the
    /// band is not part of how that is decided.**
    ///
    /// The sequencer logs a QSO with the frequency and an empty band; the same
    /// contact re-imported from an ADIF carries the ADIF's band. Keying on both
    /// let a whole file back in as duplicates of itself — 65 pairs of the same
    /// call at the same second, one row with a band and one without (reported
    /// from the bench, 2026-11-02).
    #[test]
    fn an_import_does_not_readd_what_the_log_already_has() {
        let existing = vec![qso("13WF01", "", 1_789_281_763)];
        let parsed = vec![
            // The same contact, as the ADIF spells it: a band, not an empty one.
            qso("13WF01", "11m", 1_789_281_763),
            // A different station, and the same station five minutes later: both
            // are contacts the log does not have.
            qso("19AT168", "11m", 1_789_281_763),
            qso("13WF01", "11m", 1_789_281_763 + 300),
        ];
        let (fresh, skipped) = new_import_records(&existing, parsed);
        assert_eq!(skipped, 1, "the bandless copy in the log is the same contact");
        let calls: Vec<&str> = fresh.iter().map(|q| q.call.as_str()).collect();
        assert_eq!(calls, vec!["19AT168", "13WF01"]);
    }

    /// The same file twice in a row adds nothing the second time — the bug above
    /// is the import disagreeing with *itself*, so this is the plain case.
    #[test]
    fn importing_the_same_file_twice_adds_nothing() {
        let file = vec![qso("19JV75", "11m", 1_789_279_546), qso("13WF01", "11m", 1_789_281_763)];
        let (first, skipped) = new_import_records(&[], file);
        assert_eq!((first.len(), skipped), (2, 0));
        let (second, skipped) = new_import_records(&first, vec![
            qso("19JV75", "11m", 1_789_279_546),
            qso("13WF01", "11m", 1_789_281_763),
        ]);
        assert_eq!((second.len(), skipped), (0, 2));
    }

    /// A retry's outcome replaces the first, and each target is tracked on its
    /// own — so a QSO that reached LOG11DX but failed eQSL reads as exactly
    /// that, not as one blended "uploaded".
    #[test]
    fn an_upload_outcome_replaces_the_earlier_one_for_its_target() {
        let mut status = std::collections::HashMap::new();
        record_upload_outcome(&mut status, 7, UploadTarget::Log11Dx, false);
        record_upload_outcome(&mut status, 7, UploadTarget::Eqsl, true);
        assert_eq!(status[&7], vec![(UploadTarget::Log11Dx, false), (UploadTarget::Eqsl, true)]);
        // The retry succeeds: the failure goes, it does not accumulate.
        record_upload_outcome(&mut status, 7, UploadTarget::Log11Dx, true);
        assert_eq!(status[&7], vec![(UploadTarget::Log11Dx, true), (UploadTarget::Eqsl, true)]);
    }
}

/// Which of an imported ADIF's contacts this log does not already have, and how
/// many it does.
///
/// **The match is call and time, never band.** The log is not consistent about
/// band: a contact logged by a digital sequencer carries the frequency and an
/// empty band, while the same contact re-imported from an ADIF carries the
/// ADIF's own band — so keying on band let a whole file back in as duplicates
/// of itself, and the operator was left with 65 pairs of the same call at the
/// same second, one row with a band and one without (reported 2026-11-02).
///
/// Two minutes is the window, and it is call-only because nothing is worked
/// twice on two different bands inside two minutes. The existing log is indexed
/// once by call so the check stays linear: this runs inside a single frame and a
/// full logbook is tens of thousands of records.
fn new_import_records(
    existing: &[sdroxide_types::QsoRecord],
    parsed: Vec<sdroxide_types::QsoRecord>,
) -> (Vec<sdroxide_types::QsoRecord>, usize) {
    let mut starts_by_call: HashMap<String, Vec<i64>> = HashMap::new();
    for q in existing {
        starts_by_call.entry(q.call.trim().to_ascii_uppercase()).or_default().push(q.start_utc);
    }
    let mut fresh = Vec::with_capacity(parsed.len());
    let mut skipped = 0usize;
    for r in parsed {
        let call = r.call.trim().to_ascii_uppercase();
        if call.is_empty() {
            continue;
        }
        let starts = starts_by_call.entry(call).or_default();
        if starts.iter().any(|&t| (t - r.start_utc).abs() < 120) {
            skipped += 1;
            continue;
        }
        starts.push(r.start_utc);
        fresh.push(r);
    }
    (fresh, skipped)
}
