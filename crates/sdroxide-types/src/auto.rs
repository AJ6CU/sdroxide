//! Auto mode — the policy for the unattended 11 m sequencer.
//!
//! This is pure: given the decodes just heard and the operator's log, it
//! decides whether auto mode may run and, when it may, which station to answer
//! or that nobody is worth answering and a CQ should go out instead. The UI
//! owns the log and the input clock, so the decision lives here beside them
//! while the actual sequencing stays in the engine's QSO machine.
//!
//! Auto mode is deliberately narrow. It is for the 11 m citizens' band only —
//! the fork's WSJT-CB operation, where an unattended run is the point — and it
//! only ever answers a station calling CQ whose callsign is *new*: not one
//! already in the log. Calling a station mid-exchange, or working a dupe, is
//! left to the operator.

use std::collections::HashSet;

use crate::awards::LogIndex;
use crate::digi::{cq_is_for_us, Decode, QsoStep, TranscriptLine};
use crate::{Band, Mode};

/// How long the app may go without a single input event before auto mode
/// disarms, when the operator has not chosen a figure. The backstop for an
/// unattended transmitter: a window nobody has touched for this long is a
/// window nobody is watching.
pub const AUTO_IDLE_STOP_S: f64 = 20.0 * 60.0;

/// The longest the inactivity stop may be set to, in minutes.
///
/// Deliberately a ceiling rather than a free field: auto mode is for a bathroom
/// break, not for leaving a station to work a contest unattended. The operator
/// chooses within it, never past it.
pub const AUTO_IDLE_STOP_MAX_MIN: u32 = 45;

/// The stored value that means "never stop for inactivity" — the **DISABLE**
/// entry. Only honoured on a radio whose backend
/// [`has_long_idle_stop`]; anywhere else it clamps to the ceiling like any
/// other figure past it.
pub const AUTO_IDLE_STOP_NEVER: u32 = u32::MAX;

/// The choices offered on a radio with the long inactivity stop, in the order
/// the list shows them: the default first, then the short ones, the long ones
/// and **DISABLE** last.
pub const AUTO_IDLE_STOP_LONG_CHOICES: [u32; 8] =
    [20, 30, 45, 60, 120, 240, 360, AUTO_IDLE_STOP_NEVER];

/// Whether this radio may set the inactivity stop past
/// [`AUTO_IDLE_STOP_MAX_MIN`], up to six hours, or switch it off.
///
/// Only the PlutoSDR for now: it is the radio this was asked for and tested
/// against, and every other backend keeps the 45-minute ceiling until there is
/// a reason to widen it.
pub fn has_long_idle_stop(backend: crate::Backend) -> bool {
    backend == crate::Backend::Pluto
}

/// The inactivity stop in seconds, from the operator's choice in minutes, or
/// `None` when it is switched off.
///
/// Zero is the default (twenty minutes), which a config written before this
/// existed loads as. With `long` (see [`has_long_idle_stop`]) the figure is
/// taken as it is and [`AUTO_IDLE_STOP_NEVER`] means no stop; without it the
/// figure is clamped to [`AUTO_IDLE_STOP_MAX_MIN`], `NEVER` included, so a
/// setting carried over from a Pluto never switches the stop off elsewhere.
pub fn auto_idle_stop_s(minutes: u32, long: bool) -> Option<f64> {
    let m = if minutes == 0 { (AUTO_IDLE_STOP_S / 60.0) as u32 } else { minutes };
    if long {
        if m == AUTO_IDLE_STOP_NEVER {
            return None;
        }
        return Some(f64::from(m) * 60.0);
    }
    Some(f64::from(m.min(AUTO_IDLE_STOP_MAX_MIN)) * 60.0)
}

/// How long after issuing a command the tick waits before issuing another.
///
/// Only to cover the round trip to the engine: the command is queued and the
/// status that reflects it arrives a frame or two later. Without it, the idle
/// status still showing would each frame queue another CQ.
pub const AUTO_COOLDOWN_S: f64 = 3.0;

/// **Which unattended role the station is running.**
///
/// One toggle used to do both: answer a CQ when one was heard and call CQ when
/// none was. That is a sensible default and a poor instrument — the operator
/// could not say "call CQ and work whoever answers, and do not go chasing
/// somebody else's CQ", nor "listen only and never key a CQ" — which is what
/// WSJT-Z's Auto CQ and Auto Rx are, and what issue #236 asked for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoRole {
    /// Answer stations calling CQ. **Never** calls CQ itself.
    Rx,
    /// Call CQ and work whoever answers it. Does not answer other stations'
    /// CQs — that is [`AutoRole::Rx`]'s job, and the two are switched rather
    /// than blended.
    Cq,
    /// **Continuous operation**: the two roles in sequence, without a click
    /// between contacts — finish the QSO, work whoever called us, then answer a
    /// station calling CQ, and call CQ ourselves only when neither exists. Runs
    /// until the operator stops it or a safety limit is reached.
    ///
    /// A third role rather than a change to the other two, because the operator
    /// asked in as many words that their meaning not be altered quietly.
    Continuous,
}

impl AutoRole {
    /// The word on the toggle.
    pub fn label(self) -> &'static str {
        match self {
            AutoRole::Rx => "AUTO RX",
            AutoRole::Cq => "AUTO CQ",
            AutoRole::Continuous => "AUTO CONT",
        }
    }

    /// The role one line up from this one, for a toggle that cycles.
    pub fn next(self) -> AutoRole {
        match self {
            AutoRole::Rx => AutoRole::Cq,
            AutoRole::Cq => AutoRole::Continuous,
            AutoRole::Continuous => AutoRole::Rx,
        }
    }

    /// Every role, in the order the chips are drawn.
    pub const ALL: [AutoRole; 3] = [AutoRole::Rx, AutoRole::Cq, AutoRole::Continuous];
}

/// Counts the CQ calls of one run, from the engine's own transcript.
///
/// The engine knows when a burst goes out; this reads the record of it rather
/// than counting commands, because the sequencer repeats the CQ by itself and
/// one `DigiCallCq` can become many transmissions. Nothing here is persisted:
/// it describes a run, and a run does not survive the process.
#[derive(Debug, Clone, Default)]
pub struct CqCounter {
    seen: usize,
    sent: u32,
}

impl CqCounter {
    /// Fold in the transcript's latest state and answer how many CQ calls this
    /// run has made.
    ///
    /// The count restarts whenever the sequencer is not calling CQ: a step of
    /// `TxGrid` or beyond means somebody answered, and a run that produced a
    /// contact has done its job. It also survives the transcript being cleared
    /// (`call_cq` starts a new exchange log), which is the one thing an
    /// index-into-the-list counter gets wrong.
    pub fn observe(&mut self, step: QsoStep, transcript: &[TranscriptLine]) -> u32 {
        if step != QsoStep::CallingCq {
            *self = CqCounter::default();
            return 0;
        }
        if transcript.len() < self.seen {
            // The log was cleared under us: start again from the top of it.
            self.seen = 0;
        }
        for line in &transcript[self.seen..] {
            if line.tx && line.text.trim_start().starts_with("CQ ") {
                self.sent += 1;
            }
        }
        self.seen = transcript.len();
        self.sent
    }

    pub fn sent(&self) -> u32 {
        self.sent
    }
}

/// What an Auto CQ run does when its CQ calls reach the operator's limit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CqLimit {
    /// Keep calling; the limit has not been reached (or is switched off).
    Calling,
    /// Switch to [`AutoRole::Rx`] — call no more, but keep listening and answer.
    Listen,
    /// Disarm auto mode entirely.
    Stop,
}

impl CqLimit {
    pub fn what(self) -> &'static str {
        match self {
            CqLimit::Calling => "calling CQ",
            CqLimit::Listen => "listening",
            CqLimit::Stop => "stopped",
        }
    }
}

/// **The bound on an Auto CQ run.** `repeats` is the number of CQ calls one run
/// may make; 0 means no bound of its own, and the transmit watchdog and the
/// inactivity stop still apply. `then_listen` decides which way it ends.
///
/// A CQ run that nobody answers is the expected state, so this is the one
/// figure that can end it deliberately rather than by the watchdog's clock —
/// which is exactly what WSJT-Z's repetition count is for.
pub fn cq_limit_action(sent: u32, repeats: u16, then_listen: bool) -> CqLimit {
    if repeats == 0 || sent < u32::from(repeats) {
        CqLimit::Calling
    } else if then_listen {
        CqLimit::Listen
    } else {
        CqLimit::Stop
    }
}

/// **What the unattended run does next.** One decision, taken from the facts,
/// so the scheduler itself has no branches a test cannot reach.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AutoStep {
    /// Work the station that is calling our callsign.
    AnswerCaller,
    /// Answer a station calling CQ (Rx and continuous only).
    AnswerCq,
    /// Call CQ.
    CallCq,
    /// Stay quiet and keep listening — the CQ run has used its calls and the
    /// operator asked it to listen rather than stop.
    Listen,
    /// Disarm, with the reason.
    Stop(StopReason),
}

/// Why a continuous run ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    /// The CQ run reached its call limit and the operator asked it to stop.
    CqLimit,
    /// The overall session limit the operator set.
    Session,
}

impl StopReason {
    pub fn sentence(self, limit: u32) -> String {
        match self {
            StopReason::CqLimit => format!("no answer after {limit} CQ calls"),
            StopReason::Session => format!("the {limit} minute session ended"),
        }
    }
}

/// The facts the decision is taken from — everything the scheduler knows, so the
/// nine behaviours the operator listed can be tested as decisions rather than
/// argued about from a log.
#[derive(Debug, Clone, Copy, Default)]
pub struct AutoFacts {
    /// Somebody is calling our callsign and is eligible.
    pub caller: bool,
    /// A station calling CQ passed the operator's filters and the log's novelty
    /// test.
    pub cq_candidate: bool,
    /// CQ calls this run has made.
    pub cq_calls: u32,
    /// The run's call limit, 0 for none of its own.
    pub repeats: u16,
    /// After the limit, listen rather than stop.
    pub then_listen: bool,
    /// The overall session limit in minutes, 0 for none, and how long the run
    /// has been armed.
    pub session_min: u16,
    pub session_s: f64,
}

/// **The scheduler's whole policy.** Given the role and the facts, what happens
/// next:
///
/// 1. A station calling us is worked — it is the contact already half made, and
///    it takes precedence over everything, in every role.
/// 2. Otherwise, in the roles that answer other people's CQs, an eligible CQ is
///    answered.
/// 3. Otherwise CQ is called, subject to the run's limit: listening or stopping
///    when it is reached, as the operator chose.
///
/// The session limit is checked first, because a safety limit that can be
/// overtaken by a contact is not a limit.
pub fn plan(role: AutoRole, f: &AutoFacts) -> AutoStep {
    let cap = f.session_min.min(AUTO_SESSION_MAX_MIN);
    if cap > 0 && f.session_s >= f64::from(cap) * 60.0 {
        return AutoStep::Stop(StopReason::Session);
    }
    if f.caller {
        return AutoStep::AnswerCaller;
    }
    let answers_others = matches!(role, AutoRole::Rx | AutoRole::Continuous);
    if answers_others && f.cq_candidate {
        return AutoStep::AnswerCq;
    }
    // **`Auto Rx` never calls CQ** — that is the whole of what the role means,
    // so a CQ limit has nothing to bound there and listening is the only step
    // left. Its bounds are the session limit and the inactivity stop.
    if role == AutoRole::Rx {
        return AutoStep::Listen;
    }
    match cq_limit_action(f.cq_calls, f.repeats, f.then_listen) {
        CqLimit::Calling => AutoStep::CallCq,
        CqLimit::Listen => AutoStep::Listen,
        CqLimit::Stop => AutoStep::Stop(StopReason::CqLimit),
    }
}

/// **The longest an unattended run may be set to**, in minutes.
///
/// Twelve hours, and it is a hard ceiling rather than a default: the maintainer
/// set it (2026-10-26) after discussion on the PR, and it is clamped here rather
/// than trusted in the control, so a config that arrives with more — from an
/// older build, from another client — stops at twelve.
pub const AUTO_SESSION_MAX_MIN: u16 = 720;

/// How long a station that used up its retries is left alone before it becomes
/// eligible again, in seconds.
///
/// **A quota that never expires is a blacklist.** A station that did not answer
/// twice at noon may be the only one on the band at half past: after this long the
/// attempt count is cleared and it is a candidate again.
pub const AUTO_RETRY_DELAY_S: f64 = 600.0;

/// A station's record in the run's retry book: **(calls made, deferred until)**,
/// where a zero `until` means "not deferred".
pub type RetryRecord = (u8, f64);

/// Whether a station's quota still holds it back.
///
/// Deferred means "used up the budget *and* the delay has not passed". Once the
/// delay is over the station is a candidate again, which is what makes this a
/// retry policy rather than a blacklist.
pub fn retry_holds(record: RetryRecord, budget: u8, now: f64) -> bool {
    let (calls, until) = record;
    calls >= budget && until > 0.0 && now < until
}

/// The record after calling this station once.
///
/// A record whose delay has expired starts again from this call, so the count is
/// never carried across the delay — a station that was silent ten minutes ago is
/// not on its last chance now.
pub fn after_call(record: RetryRecord, budget: u8, now: f64) -> RetryRecord {
    let (calls, until) = record;
    if until > 0.0 && now >= until {
        return (1, 0.0);
    }
    let calls = calls.saturating_add(1);
    if calls >= budget {
        (calls, now + AUTO_RETRY_DELAY_S)
    } else {
        (calls, until)
    }
}

/// The station auto mode has chosen to answer.
#[derive(Debug, Clone, PartialEq)]
pub struct AutoTarget {
    pub call: String,
    /// Their grid, when the CQ carried one. 11 m CQs usually do not.
    pub grid: Option<String>,
    /// Their signal at us — the report we will send.
    pub snr_db: i16,
    /// The tone offset their CQ was heard at, to answer on.
    pub audio_hz: f32,
}

/// Whether auto mode may run: a slotted FT mode in a band this station may
/// transmit on, with a non-zero transmit watchdog to bound an unattended run.
///
/// `tx_permitted` is the engine's own answer for the current dial — the same
/// question its key-down gate asks — so auto mode never arms somewhere it could
/// not actually key: outside an amateur allocation with `tx_ham_only` set, on a
/// broadcast band, or on a receive-only radio. It is passed in rather than
/// worked out here because the licence gate and the band plan live in the
/// engine, not in this crate.
///
/// The watchdog cannot be zero: it is the only thing that stops a station
/// calling CQ into an empty band forever, and auto mode is exactly the case
/// where nobody is watching to notice.
pub fn auto_ready(mode: Mode, watchdog_min: u32, tx_permitted: bool) -> bool {
    matches!(mode, Mode::Ft8 | Mode::Ft4 | Mode::Ft2) && tx_permitted && watchdog_min > 0
}

/// Why auto mode may not run, as a sentence for the toggle's tooltip, or
/// `None` when it may. The same tests as [`auto_ready`], in the order the
/// operator can do something about them.
pub fn auto_block_reason(
    mode: Mode,
    watchdog_min: u32,
    tx_permitted: bool,
) -> Option<&'static str> {
    if !matches!(mode, Mode::Ft8 | Mode::Ft4 | Mode::Ft2) {
        return Some("Auto mode runs FT8, FT4 and FT2 only.");
    }
    if !tx_permitted {
        return Some(
            "This radio may not transmit on the current band — tune to an amateur band \
             (or enable 11 m transmit) to arm it.",
        );
    }
    if watchdog_min == 0 {
        return Some(
            "Set a transmit watchdog first (the digi setup): auto mode needs it to pace itself.",
        );
    }
    None
}

/// Pick the CQ auto mode should answer, or `None` when calling CQ is the move.
///
/// A candidate is a CQ that passes [`cq_is_for_us`], names a callsign, is not
/// already in the log, and has not already been tried this session. The
/// strongest is chosen — the one most likely to complete a contact — with the
/// newest slot breaking ties.
///
/// `skip` is asked about every row before anything else, so a caller can rule
/// a station out here instead of handing over a filtered copy. The decode list
/// is borrowed for the whole call and holds up to 200 rows, and this runs every
/// frame: building a copy of it to drop a handful of rows would cost far more
/// than the handful of comparisons it saves.
pub fn pick_cq(
    decodes: &[Decode],
    log: &LogIndex,
    my_call: &str,
    my_grid: &str,
    tried: &HashSet<String>,
    skip: impl Fn(&Decode) -> bool,
) -> Option<AutoTarget> {
    let mut best: Option<&Decode> = None;
    for d in decodes {
        if skip(d) {
            continue;
        }
        let Some(from) = d.from.as_deref().filter(|c| !c.is_empty()) else { continue };
        if !d.is_cq || !cq_is_for_us(d, my_call, my_grid) {
            continue;
        }
        let key = from.to_ascii_uppercase();
        if tried.contains(&key) {
            continue;
        }
        // The empty band: 11 m has no ADIF enumeration and no per-band log
        // judgement, so this is the all-time "never worked this call" test.
        if !log.novelty(&key, d.grid.as_deref(), "").new_call {
            continue;
        }
        match best {
            Some(b) if (d.snr_db, d.slot_utc) <= (b.snr_db, b.slot_utc) => {}
            _ => best = Some(d),
        }
    }
    best.map(|d| AutoTarget {
        call: d.from.clone().unwrap_or_default(),
        grid: d.grid.clone(),
        snr_db: d.snr_db,
        audio_hz: d.audio_hz,
    })
}

/// **A station calling *us*** — the pending-caller queue's source.
///
/// Distinct from [`pick_cq`], and the distinction is the whole of Auto CQ: that
/// one answers a station calling CQ, which is Auto Rx's job, while this is
/// somebody addressing our callsign — the message that arrives while we are
/// already working somebody else, which the panel shows as "also calling: …"
/// and which used to need the operator's hand on REPLY.
///
/// Kept pure for the same reason as the rest of this file: the choice of who to
/// work next is a decision, and the engine's sequencing is the side effect.
pub fn pick_caller(
    decodes: &[Decode],
    my_call: &str,
    log: &LogIndex,
    tried: &HashSet<String>,
    skip: impl Fn(&Decode) -> bool,
) -> Option<AutoTarget> {
    let mine = my_call.trim().to_ascii_uppercase();
    if mine.is_empty() {
        return None;
    }
    let mut best: Option<&Decode> = None;
    for d in decodes {
        if skip(d) {
            continue;
        }
        let Some(to) = d.to.as_deref() else { continue };
        if !to.eq_ignore_ascii_case(&mine) {
            continue;
        }
        // A message *from* ourselves can only be our own transmission echoed
        // back by a receiver, or a station that has our callsign in the `from`
        // field by mistake. Either way there is nobody to work.
        let Some(from) = d.from.as_deref().filter(|c| !c.is_empty()) else { continue };
        if from.eq_ignore_ascii_case(&mine) {
            continue;
        }
        let key = from.to_ascii_uppercase();
        if tried.contains(&key) {
            continue;
        }
        // Nobody is worked twice in a session, the rule Auto Rx already keeps.
        if !log.novelty(&key, d.grid.as_deref(), "").new_call {
            continue;
        }
        match best {
            Some(b) if (d.snr_db, d.slot_utc) <= (b.snr_db, b.slot_utc) => {}
            _ => best = Some(d),
        }
    }
    best.map(|d| AutoTarget {
        call: d.from.clone().unwrap_or_default(),
        grid: d.grid.clone(),
        snr_db: d.snr_db,
        audio_hz: d.audio_hz,
    })
}

/// How long a station that looks busy is left alone, in seconds.
///
/// Long enough for the exchange it is in to finish or move on — an FT8 contact
/// is under two minutes — and short enough that the caller is not forgotten.
pub const BUSY_DEFER_S: f64 = 150.0;

/// When the decodes say a station is **already working somebody else**, and when
/// they say nothing of the kind.
///
/// The distinction issue #236 asks for: a station that called us ten minutes ago
/// is not a caller any more, and one that has since addressed another station is
/// mid-exchange — calling it now would talk over a contact in progress, which is
/// the fastest way to be the station everybody filters out.
///
/// `now` and `window_s` are **both in seconds** — `Decode::slot_utc` is Unix
/// seconds and so is the caller's clock — and the last decode from `call` decides,
/// because a station's latest message is where its exchange stands.
pub fn looks_busy(decodes: &[Decode], call: &str, my_call: &str, now: i64, window_s: i64) -> bool {
    let call = call.trim();
    if call.is_empty() {
        return false;
    }
    let newest = decodes
        .iter()
        .filter(|d| d.from.as_deref().is_some_and(|f| f.eq_ignore_ascii_case(call)))
        .max_by_key(|d| d.slot_utc);
    let Some(last) = newest else { return false };
    // Old traffic says nothing about where the station is now.
    // **Seconds, like both clocks.** This read `* 1000` once — a window of 150
    // "seconds" that was really 150 000 of them, so no traffic was ever stale
    // and every station that had worked somebody stayed permanently busy.
    if now.saturating_sub(last.slot_utc) > window_s.max(0) {
        return false;
    }
    // Addressing somebody other than us is the exchange; addressing us is our
    // own call being answered.
    let mine = my_call.trim();
    match last.to.as_deref() {
        Some(to) => {
            !to.eq_ignore_ascii_case("CQ") && (mine.is_empty() || !to.eq_ignore_ascii_case(mine))
        }
        None => false,
    }
}

/// Whether the licence gate lets this band be keyed, mirroring the engine's
/// own key-down test: every amateur allocation always, and 11 m when the
/// operator has opened it. The broadcast services and general coverage never.
///
/// The engine is the authority and checks more than this (the device's range,
/// the offset against the band plan); this is only the band-level question auto
/// mode needs to decide whether arming is even meaningful.
pub fn band_may_transmit(band: Band, ham_only: bool, cb_tx_allowed: bool) -> bool {
    !ham_only || band.is_amateur() || (band == Band::M11 && cb_tx_allowed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::QsoRecord;

    fn cq(from: &str, snr: i16, slot: i64) -> Decode {
        Decode {
            slot_utc: slot,
            snr_db: snr,
            dt: 0.0,
            audio_hz: 1000.0,
            message: format!("CQ {from}"),
            to: None,
            from: Some(from.to_string()),
            grid: None,
            is_cq: true,
            cq_to: None,
            free_text: false,
            rr73_to: None,
        }
    }

    fn log_with(calls: &[&str]) -> LogIndex {
        let log: Vec<QsoRecord> = calls
            .iter()
            .map(|c| QsoRecord { call: (*c).to_string(), mode: "FT8".into(), ..Default::default() })
            .collect();
        LogIndex::build(&log)
    }

    #[test]
    fn picks_the_strongest_new_cq() {
        let d = [cq("19LR121", -5, 1), cq("14AT276", 12, 2), cq("30AO010", 3, 3)];
        let got =
            pick_cq(&d, &log_with(&[]), "26AT715", "", &HashSet::new(), |_| false).expect("a CQ");
        assert_eq!(got.call, "14AT276");
        assert_eq!(got.snr_db, 12);
    }

    #[test]
    fn never_answers_a_call_already_in_the_log() {
        let d = [cq("19LR121", 20, 1)];
        assert!(pick_cq(&d, &log_with(&["19LR121"]), "26AT715", "", &HashSet::new(), |_| false)
            .is_none());
    }

    #[test]
    fn skips_a_station_already_tried_this_session() {
        let d = [cq("19LR121", 20, 1)];
        let tried: HashSet<String> = ["19LR121".to_string()].into_iter().collect();
        assert!(pick_cq(&d, &log_with(&[]), "26AT715", "", &tried, |_| false).is_none());
    }

    #[test]
    fn a_skipped_station_is_never_chosen_even_when_it_is_the_strongest() {
        // The session ignore list arrives here as a predicate rather than as a
        // filtered copy of the decode list, so this is the case that says the
        // rule is actually consulted: the muted station is the best CQ on the
        // band, and it must still lose to a weaker one.
        let d = [cq("19LR121", -5, 1), cq("14AT276", 20, 2), cq("30AO010", 3, 3)];
        let muted = ["14AT276".to_string()];
        let got = pick_cq(&d, &log_with(&[]), "26AT715", "", &HashSet::new(), |d| {
            d.from.as_deref().is_some_and(|c| muted.iter().any(|m| m == c))
        })
        .expect("a CQ");
        assert_eq!(got.call, "30AO010", "the muted station is the strongest and must not win");
    }

    #[test]
    fn ignores_a_cq_aimed_elsewhere() {
        let mut d = cq("JA1ABC", 20, 1);
        d.cq_to = Some("JA".to_string());
        // Not a Japanese station: "CQ JA" is not ours to answer.
        assert!(pick_cq(&[d], &log_with(&[]), "26AT715", "", &HashSet::new(), |_| false).is_none());
    }

    #[test]
    fn ignores_non_cq_traffic() {
        let mut d = cq("19LR121", 20, 1);
        d.is_cq = false;
        assert!(pick_cq(&[d], &log_with(&[]), "26AT715", "", &HashSet::new(), |_| false).is_none());
    }

    #[test]
    fn readiness_needs_an_ft_mode_where_tx_is_permitted_and_a_watchdog() {
        assert!(auto_ready(Mode::Ft8, 6, true));
        assert!(auto_ready(Mode::Ft4, 6, true));
        assert!(auto_ready(Mode::Ft2, 1, true));
        // Wrong mode, no permission on this band, no watchdog.
        assert!(!auto_ready(Mode::Js8, 6, true));
        assert!(!auto_ready(Mode::Ft8, 6, false));
        assert!(!auto_ready(Mode::Ft8, 0, true));
    }

    #[test]
    fn the_licence_gate_opens_amateur_bands_and_11m_once_opted_in() {
        // An amateur band is always keyable under the gate.
        assert!(band_may_transmit(Band::M20, true, false));
        // 11 m only once opened.
        assert!(!band_may_transmit(Band::M11, true, false));
        assert!(band_may_transmit(Band::M11, true, true));
        // Broadcast and general coverage stay locked either way.
        assert!(!band_may_transmit(Band::Sw, true, true));
        assert!(!band_may_transmit(Band::Gen, true, true));
        // The gate off opens everything, as `tx_ham_only = false` means.
        assert!(band_may_transmit(Band::Gen, false, false));
    }

    #[test]
    fn the_inactivity_stop_is_clamped_to_the_ceiling() {
        assert_eq!(auto_idle_stop_s(0, false), Some(AUTO_IDLE_STOP_S), "zero means the default");
        assert_eq!(auto_idle_stop_s(20, false), Some(20.0 * 60.0));
        assert_eq!(auto_idle_stop_s(45, false), Some(45.0 * 60.0));
        assert_eq!(auto_idle_stop_s(120, false), Some(45.0 * 60.0), "never past the ceiling");
        assert_eq!(
            auto_idle_stop_s(AUTO_IDLE_STOP_NEVER, false),
            Some(45.0 * 60.0),
            "DISABLE carried to another radio still stops at the ceiling"
        );
    }

    #[test]
    fn a_pluto_may_run_up_to_six_hours_or_never_stop() {
        assert!(has_long_idle_stop(crate::Backend::Pluto));
        assert!(!has_long_idle_stop(crate::Backend::SdrPlay));
        assert!(!has_long_idle_stop(crate::Backend::RtlSdr));
        assert_eq!(auto_idle_stop_s(0, true), Some(AUTO_IDLE_STOP_S), "zero means the default");
        for m in [60, 120, 240, 360] {
            assert_eq!(auto_idle_stop_s(m, true), Some(f64::from(m) * 60.0), "{m} min on a Pluto");
        }
        assert_eq!(auto_idle_stop_s(AUTO_IDLE_STOP_NEVER, true), None, "DISABLE never stops");
        assert_eq!(AUTO_IDLE_STOP_LONG_CHOICES[0], (AUTO_IDLE_STOP_S / 60.0) as u32);
    }

    // ── the Auto CQ run's own bound (issue #236) ──

    fn sent_line(text: &str) -> TranscriptLine {
        TranscriptLine::sent(text)
    }

    /// **A CQ run is counted from what actually went out**, not from the
    /// commands: the sequencer repeats the CQ by itself, so one `DigiCallCq` is
    /// many transmissions and a count of presses would never reach any limit.
    #[test]
    fn a_cq_run_is_counted_from_the_transcript() {
        let mut c = CqCounter::default();
        let log = vec![sent_line("CQ OE1TEST JN88")];
        assert_eq!(c.observe(QsoStep::CallingCq, &log), 1);
        let log = vec![
            sent_line("CQ OE1TEST JN88"),
            sent_line("CQ OE1TEST JN88"),
            sent_line("CQ OE1TEST JN88"),
        ];
        assert_eq!(c.observe(QsoStep::CallingCq, &log), 3);
        // Nothing new, nothing counted.
        assert_eq!(c.observe(QsoStep::CallingCq, &log), 3);
        // A line that is not a CQ is not a CQ call — the report to a station
        // that answered, for instance.
        let log = vec![
            sent_line("CQ OE1TEST JN88"),
            sent_line("CQ OE1TEST JN88"),
            sent_line("CQ OE1TEST JN88"),
            sent_line("OE1TEST DL1ABC -12"),
        ];
        assert_eq!(c.observe(QsoStep::CallingCq, &log), 3);
    }

    /// Somebody answering ends the run: the next CQ run starts from zero, or a
    /// station that works a dozen contacts would be cut off at the dozenth.
    #[test]
    fn a_contact_restarts_the_count_and_a_cleared_transcript_does_not_confuse_it() {
        let mut c = CqCounter::default();
        let log = vec![sent_line("CQ OE1TEST JN88")];
        assert_eq!(c.observe(QsoStep::CallingCq, &log), 1);
        // They came back: the sequencer is past CallingCq.
        assert_eq!(c.observe(QsoStep::TxReport, &log), 0);
        // The engine clears the exchange log when a run starts (`call_cq`), so
        // the index has to survive a shorter transcript than it last saw.
        let fresh = vec![sent_line("CQ OE1TEST JN88")];
        assert_eq!(c.observe(QsoStep::CallingCq, &fresh), 1);
        let fresh = vec![sent_line("CQ OE1TEST JN88"), sent_line("CQ OE1TEST JN88")];
        assert_eq!(c.observe(QsoStep::CallingCq, &fresh), 2);
    }

    /// **The limit either ends the run or turns it into listening.** Zero is no
    /// limit of its own — the watchdog and the inactivity stop still bound it.
    #[test]
    fn the_cq_limit_ends_the_run_or_turns_it_into_listening() {
        assert_eq!(cq_limit_action(9, 0, true), CqLimit::Calling, "0 is no limit");
        assert_eq!(cq_limit_action(2, 3, false), CqLimit::Calling);
        assert_eq!(cq_limit_action(3, 3, false), CqLimit::Stop);
        assert_eq!(cq_limit_action(4, 3, false), CqLimit::Stop);
        assert_eq!(cq_limit_action(3, 3, true), CqLimit::Listen);
        // A limit of one is honoured: one call, then whatever the operator asked.
        assert_eq!(cq_limit_action(1, 1, true), CqLimit::Listen);
        assert_eq!(cq_limit_action(0, 1, false), CqLimit::Calling);
    }

    /// The two roles are named so the toggle can say which one is running.
    #[test]
    fn the_roles_say_which_one_is_armed() {
        assert_eq!(AutoRole::Rx.label(), "AUTO RX");
        assert_eq!(AutoRole::Cq.label(), "AUTO CQ");
        assert_ne!(AutoRole::Rx, AutoRole::Cq);
    }

    // ── the pending caller (Auto CQ's queue) ──

    fn caller(from: &str, to: &str, snr: i16, slot: i64) -> Decode {
        Decode {
            slot_utc: slot,
            snr_db: snr,
            dt: 0.0,
            audio_hz: 1500.0,
            message: format!("{to} {from} -{snr}"),
            to: Some(to.into()),
            from: Some(from.into()),
            grid: None,
            is_cq: false,
            cq_to: None,
            free_text: false,
            rr73_to: None,
        }
    }

    /// **A station calling *us*** is the one the queue is for, and nobody else
    /// is: an unrelated CQ belongs to Auto Rx, and our own echo belongs to
    /// nobody.
    #[test]
    fn only_a_station_addressing_our_callsign_is_a_pending_caller() {
        let empty = LogIndex::build(&[]);
        let none = HashSet::new();
        let d = [
            caller("IU5HIU", "19DCG373", -8, 10),
            // Somebody calling CQ in general — not ours to take in Auto CQ.
            Decode { is_cq: true, ..caller("SP3JHZ", "CQ", -3, 12) },
            // Our own transmission, echoed back by a receiver.
            caller("19DCG373", "SP3JHZ", -1, 14),
            // Addressed to another station.
            caller("DL1ABC", "F4NRM", -2, 16),
        ];
        let t = pick_caller(&d, "19DCG373", &empty, &none, |_| false).expect("IU5HIU is calling");
        assert_eq!(t.call, "IU5HIU");
    }

    /// The strongest caller wins, so the queue starts with the contact most
    /// likely to complete; and a station already tried this session is not in
    /// it again.
    #[test]
    fn the_queue_takes_the_strongest_caller_once() {
        let empty = LogIndex::build(&[]);
        let d = [caller("WEAK", "19DCG373", -18, 10), caller("LOUD", "19DCG373", -2, 12)];
        let t = pick_caller(&d, "19DCG373", &empty, &HashSet::new(), |_| false).unwrap();
        assert_eq!(t.call, "LOUD");
        let mut tried = HashSet::new();
        tried.insert("LOUD".to_string());
        let t = pick_caller(&d, "19DCG373", &empty, &tried, |_| false).unwrap();
        assert_eq!(t.call, "WEAK", "a station already tried is passed over");
    }

    /// An empty callsign is a station that has not been configured yet, and it
    /// must not turn every decode into a caller.
    #[test]
    fn an_unconfigured_callsign_has_no_callers() {
        let empty = LogIndex::build(&[]);
        let d = [caller("IU5HIU", "19DCG373", -8, 10)];
        assert!(pick_caller(&d, "", &empty, &HashSet::new(), |_| false).is_none());
        assert!(pick_caller(&d, "   ", &empty, &HashSet::new(), |_| false).is_none());
    }

    // ── a station that is already busy ──

    fn traffic(from: &str, to: &str, slot: i64) -> Decode {
        Decode {
            slot_utc: slot,
            snr_db: -10,
            dt: 0.0,
            audio_hz: 1500.0,
            message: format!("{to} {from} -10"),
            to: Some(to.into()),
            from: Some(from.into()),
            grid: None,
            is_cq: to == "CQ",
            cq_to: None,
            free_text: false,
            rr73_to: None,
        }
    }

    /// **A station addressing somebody else is mid-exchange**; one addressing us
    /// is answering, and one whose last message is old tells us nothing.
    #[test]
    fn a_station_is_busy_when_its_latest_message_is_to_somebody_else() {
        let now = 1_000_000;
        // Fresh (five seconds ago), and addressed to another station: busy.
        let d = [traffic("IU5HIU", "SP3JHZ", now - 5)];
        assert!(looks_busy(&d, "IU5HIU", "19DCG373", now, 300));

        // Addressed to us: that is our call being answered, not an exchange.
        let d = [traffic("IU5HIU", "19DCG373", now - 5)];
        assert!(!looks_busy(&d, "IU5HIU", "19DCG373", now, 300));

        // Calling CQ: free to be answered.
        let d = [traffic("IU5HIU", "CQ", now - 5)];
        assert!(!looks_busy(&d, "IU5HIU", "19DCG373", now, 300));

        // Old traffic says nothing about now.
        let d = [traffic("IU5HIU", "SP3JHZ", now - 600)];
        assert!(!looks_busy(&d, "IU5HIU", "19DCG373", now, 300));

        // The *latest* message decides: it called us, then went back to a
        // contact with somebody else.
        let d = [traffic("IU5HIU", "19DCG373", now - 20), traffic("IU5HIU", "SP3JHZ", now - 5)];
        assert!(looks_busy(&d, "IU5HIU", "19DCG373", now, 300));

        // A station we have never heard is not busy — it may be calling us for
        // the first time.
        assert!(!looks_busy(&[], "IU5HIU", "19DCG373", now, 300));
        assert!(!looks_busy(&[], "", "19DCG373", now, 300), "an empty call is not a station");
    }

    // ── the scheduler's policy, as the operator numbered it ──

    fn facts() -> AutoFacts {
        AutoFacts { repeats: 0, then_listen: true, ..Default::default() }
    }

    /// **1. A caller is worked in every role**, and it beats a CQ on the band and
    /// the CQ run's own limit — the contact is already half made.
    #[test]
    fn a_caller_is_worked_before_anything_else_in_every_role() {
        for role in AutoRole::ALL {
            let f = AutoFacts { caller: true, cq_candidate: true, ..facts() };
            assert_eq!(plan(role, &f), AutoStep::AnswerCaller, "{role:?}");
            // Even at the CQ limit: the limit bounds the calling, not the
            // answering.
            let f =
                AutoFacts { caller: true, cq_calls: 9, repeats: 3, then_listen: false, ..facts() };
            assert_eq!(plan(role, &f), AutoStep::AnswerCaller, "{role:?}");
        }
    }

    /// **2. AUTO RX answers others' CQs; AUTO CQ does not.** That separation is
    /// the point of the two roles and must not be blurred by the third.
    #[test]
    fn only_the_roles_that_answer_others_take_a_cq() {
        let f = AutoFacts { cq_candidate: true, ..facts() };
        assert_eq!(plan(AutoRole::Rx, &f), AutoStep::AnswerCq);
        assert_eq!(plan(AutoRole::Continuous, &f), AutoStep::AnswerCq);
        assert_eq!(plan(AutoRole::Cq, &f), AutoStep::CallCq, "AUTO CQ took somebody else's CQ");
    }

    /// **3. With nobody to work, each role does its own thing**: RX listens,
    /// CQ calls, continuous calls.
    #[test]
    fn an_empty_band_leaves_rx_listening_and_the_others_calling() {
        let f = facts();
        assert_eq!(plan(AutoRole::Rx, &f), AutoStep::Listen);
        assert_eq!(plan(AutoRole::Cq, &f), AutoStep::CallCq);
        assert_eq!(plan(AutoRole::Continuous, &f), AutoStep::CallCq);
    }

    /// **4. The CQ limit ends the run the way the operator chose**, in every
    /// role, including continuous — a safety limit that continuous mode could
    /// ignore would not be one.
    #[test]
    fn the_cq_limit_ends_or_quietens_every_role() {
        let listen = AutoFacts { cq_calls: 5, repeats: 5, then_listen: true, ..facts() };
        let stop = AutoFacts { cq_calls: 5, repeats: 5, then_listen: false, ..facts() };
        for role in [AutoRole::Cq, AutoRole::Continuous] {
            assert_eq!(plan(role, &listen), AutoStep::Listen, "{role:?}");
            assert_eq!(plan(role, &stop), AutoStep::Stop(StopReason::CqLimit), "{role:?}");
        }
        // `Auto Rx` calls no CQ, so a call limit has nothing to bound there: it
        // listens, and the session limit is what ends it.
        assert_eq!(plan(AutoRole::Rx, &listen), AutoStep::Listen);
        assert_eq!(plan(AutoRole::Rx, &stop), AutoStep::Listen);
    }

    /// **5. The session limit is a limit, not a hint**: it ends the run in every
    /// role, and it is checked before a caller, so nothing can overtake it.
    #[test]
    fn the_session_limit_ends_the_run_before_a_caller_is_taken() {
        let f = AutoFacts { caller: true, session_min: 30, session_s: 30.0 * 60.0, ..facts() };
        for role in AutoRole::ALL {
            assert_eq!(plan(role, &f), AutoStep::Stop(StopReason::Session), "{role:?}");
        }
        // One second short, and the caller is taken as usual.
        let f =
            AutoFacts { caller: true, session_min: 30, session_s: 30.0 * 60.0 - 1.0, ..facts() };
        assert_eq!(plan(AutoRole::Continuous, &f), AutoStep::AnswerCaller);
        // Zero means no limit.
        let f = AutoFacts { session_min: 0, session_s: 9_999.0, ..facts() };
        assert_eq!(plan(AutoRole::Continuous, &f), AutoStep::CallCq);
    }

    /// **Twelve hours is the ceiling, and it is enforced here**, not merely
    /// offered by the control: a config from an older build that asks for more
    /// stops at twelve.
    #[test]
    fn an_unattended_run_is_capped_at_twelve_hours() {
        let over = AutoFacts { session_min: 1440, session_s: 13.0 * 3600.0, ..facts() };
        assert_eq!(
            plan(AutoRole::Continuous, &over),
            AutoStep::Stop(StopReason::Session),
            "a twenty-four-hour config ran past the twelve-hour ceiling"
        );
        // Two hours itself still runs to the last second.
        let within = AutoFacts {
            session_min: AUTO_SESSION_MAX_MIN,
            session_s: f64::from(AUTO_SESSION_MAX_MIN) * 60.0 - 1.0,
            ..facts()
        };
        assert_eq!(plan(AutoRole::Continuous, &within), AutoStep::CallCq);
    }

    /// The three roles are named, cycle in one order, and are all of `ALL`.
    #[test]
    fn the_roles_are_named_and_cycle() {
        assert_eq!(AutoRole::ALL.map(|r| r.label()), ["AUTO RX", "AUTO CQ", "AUTO CONT"]);
        assert_eq!(AutoRole::Rx.next(), AutoRole::Cq);
        assert_eq!(AutoRole::Cq.next(), AutoRole::Continuous);
        assert_eq!(AutoRole::Continuous.next(), AutoRole::Rx);
    }

    /// **Eleven metres runs the same auto mode.** The band's FT8 is the CB
    /// dialect — free-text calls, no locator, a different dictionary — but it is
    /// the same mode and the same sequencer, so the only gate is the licence
    /// switch that opens 11 m transmit. Nothing here excludes the band, and this
    /// says so before somebody adds a restriction that reads sensible and costs
    /// the fork the band it is built for.
    #[test]
    fn auto_mode_runs_on_eleven_metres_once_its_transmitter_is_opened() {
        // An opened 11 m: a watchdog and the licence switch, and nothing else.
        assert!(auto_ready(Mode::Ft8, 6, band_may_transmit(Band::M11, true, true)));
        // Not opened: the same refusal every other receive-only band gets, and it
        // names the switch rather than saying "no".
        let why = auto_block_reason(Mode::Ft8, 6, band_may_transmit(Band::M11, true, false))
            .expect("an unopened 11 m is refused");
        assert!(why.contains("11 m"), "{why}");
        // And with the switch open, no block reason at all.
        assert_eq!(auto_block_reason(Mode::Ft8, 6, true), None);

        // A CB caller with no locator is a caller like any other: the queue on
        // 11 m works off callsigns, which is all the band carries.
        let empty = LogIndex::build(&[]);
        let d = [Decode {
            slot_utc: 10,
            snr_db: -9,
            dt: 0.0,
            audio_hz: 1500.0,
            message: "19DCG373 26AT715 -09".into(),
            to: Some("19DCG373".into()),
            from: Some("26AT715".into()),
            grid: None,
            is_cq: false,
            cq_to: None,
            free_text: false,
            rr73_to: None,
        }];
        let t = pick_caller(&d, "19DCG373", &empty, &HashSet::new(), |_| false)
            .expect("a CB station calling us is a caller");
        assert_eq!(t.call, "26AT715");
        assert!(t.grid.is_none(), "11 m carries no locator, and nothing should invent one");
    }

    /// **A used-up quota expires.** Two calls with a budget of two put the
    /// station aside for the delay; after it, it is a candidate again with a
    /// fresh count — a quota that never expires would be a blacklist.
    #[test]
    fn a_used_up_retry_quota_expires_after_the_delay() {
        let now = 1_000.0;
        let mut r: RetryRecord = (0, 0.0);
        assert!(!retry_holds(r, 2, now), "a station never called is never held back");
        r = after_call(r, 2, now);
        assert_eq!(r, (1, 0.0), "one call, no delay yet");
        assert!(!retry_holds(r, 2, now));
        r = after_call(r, 2, now + 30.0);
        assert_eq!(r.0, 2, "two calls");
        assert!(r.1 > now, "the delay was set");
        assert!(retry_holds(r, 2, now + 60.0), "held back during the delay");
        let after = r.1 + 1.0;
        assert!(!retry_holds(r, 2, after), "eligible again after it");

        // The next call after the delay starts a fresh count, not the old one.
        let again = after_call(r, 2, after);
        assert_eq!(again, (1, 0.0), "the count did not restart");
    }
}
