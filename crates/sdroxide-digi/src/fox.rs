//! The Fox half of FT8 DXpedition mode: running a pile-up instead of a single
//! contact.
//!
//! A Fox transmits several signals at once (up to five, 60 Hz apart) in one
//! period, and works a queue of Hounds rather than one station. The layout that
//! makes this pay is the DXpedition message — `K1ABC RR73; W9XYZ <DX1FOX> +03`
//! closes one contact *and* opens the next in a single signal, so a slot spent
//! saying goodbye is not a slot lost.
//!
//! The state machine is pure: [`Fox::plan`] computes a transmission from the
//! current queue without changing anything (so the operator can be shown what
//! is about to go out), and [`Fox::apply`] commits that same plan once the
//! burst has actually left the transmitter.

use std::collections::{HashSet, VecDeque};

use sdroxide_types::{
    Decode, DigiConfig, FOX_MAX_SLOTS, FoxCaller, Mode, QsoRecord, fmt_report, resolve_callsign,
};

use crate::qso::{Payload, classify_payload};

/// Spacing between the Fox's simultaneous signals, as WSJT-X lays them out.
pub const SLOT_SPACING_HZ: f32 = 60.0;

/// FT2's spacing. An FT2 signal is 167 Hz wide against FT8's 50, so 60 Hz
/// slots would overlap outright; the reference client uses 500 Hz for the
/// same reason (`foxgenft2.f90`: "we use fstep=500 Hz between slots for 3x
/// isolation margin (was 200 Hz)").
pub const FT2_SLOT_SPACING_HZ: f32 = 500.0;

/// Spacing between a Fox's simultaneous signals in `mode`.
pub fn slot_spacing_hz(mode: Mode) -> f32 {
    match mode {
        Mode::Ft2 => FT2_SLOT_SPACING_HZ,
        _ => SLOT_SPACING_HZ,
    }
}

/// A Hound in the pile-up.
#[derive(Debug, Clone)]
struct Hound {
    call: String,
    grid: Option<String>,
    /// Their signal at us — the report we send (or sent) them.
    snr: i16,
    /// The report they sent back, once they roger ours.
    rpt_rcvd: Option<i16>,
    /// When we first started working them (their contact's start time).
    started_utc: i64,
    /// Reports transmitted to them with nothing back yet.
    calls_sent: u32,
}

impl Hound {
    fn caller(&self, working: bool) -> FoxCaller {
        FoxCaller { call: self.call.clone(), grid: self.grid.clone(), snr_db: self.snr, working }
    }
}

/// One transmission's worth of Fox traffic: the messages, and the state changes
/// that sending them implies. Produced by [`Fox::plan`], committed by
/// [`Fox::apply`] — never by whoever merely looked at it.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct FoxPlan {
    /// Messages to transmit simultaneously, one per signal.
    pub messages: Vec<String>,
    /// Callers this transmission promotes from the queue into a contact.
    start: Vec<String>,
    /// Contacts this transmission repeats an unanswered report to.
    repeat: Vec<String>,
    /// Contacts this transmission closes with RR73 (and so logs).
    close: Vec<String>,
}

/// The Fox's pile-up.
pub struct Fox {
    /// Hounds that have called us and are not being worked yet.
    queue: Vec<Hound>,
    /// Hounds we have reported to, waiting for their R+report.
    working: Vec<Hound>,
    /// Hounds that rogered our report and are owed an RR73.
    owed_rr73: Vec<Hound>,
    /// Contacts completed and waiting to be logged.
    completed: VecDeque<QsoRecord>,
    /// Calls worked this session. Its own copy rather than the QSO machine's:
    /// the Fox both fills it and reads it back when ranking the queue, and
    /// nothing outside the pile-up needs to see it.
    worked: HashSet<String>,
}

impl Fox {
    pub fn new() -> Self {
        Fox {
            queue: Vec::new(),
            working: Vec::new(),
            owed_rr73: Vec::new(),
            completed: VecDeque::new(),
            worked: HashSet::new(),
        }
    }

    /// True while there is nobody to work — the plan will be a bare CQ.
    pub fn is_idle(&self) -> bool {
        self.queue.is_empty() && self.working.is_empty() && self.owed_rr73.is_empty()
    }

    /// The pile-up as the operator sees it: contacts in progress first, then
    /// the queue in the order they will be taken.
    pub fn status(&self) -> Vec<FoxCaller> {
        let mut out: Vec<FoxCaller> =
            self.owed_rr73.iter().chain(self.working.iter()).map(|h| h.caller(true)).collect();
        out.extend(self.ranked_queue().into_iter().rev().map(|h| h.caller(false)));
        out
    }

    pub fn take_completed(&mut self) -> Option<QsoRecord> {
        self.completed.pop_front()
    }

    /// Fold in a slot's decodes. Returns true if the pile-up changed.
    ///
    /// Only messages addressed to us matter: a Hound calling (`FOX HOUND GRID`)
    /// joins the queue, and a Hound rogering our report (`FOX HOUND R-12`) earns
    /// an RR73. Everything else on the band is somebody else's business.
    pub fn on_rx(&mut self, decodes: &[Decode], my_call: &str, now_utc: i64) -> bool {
        let mut changed = false;
        for d in decodes {
            if d.to.as_deref() != Some(my_call) {
                continue;
            }
            let Some(from) = d.from.as_deref().filter(|f| !f.is_empty()) else { continue };
            match classify_payload(&d.message) {
                // Somebody signing off, not somebody asking to be worked. A
                // Hound we have just logged commonly sends a 73 afterwards, and
                // reading that as a fresh call put them straight back into the
                // pile-up to be worked all over again.
                Payload::B73 | Payload::Rrr | Payload::Rr73 => continue,
                // They rogered our report: their contact is complete bar the
                // RR73 we owe them.
                Payload::RReport(r) => {
                    if let Some(pos) = self.working.iter().position(|h| h.call == from) {
                        let mut h = self.working.remove(pos);
                        h.rpt_rcvd = Some(r);
                        self.owed_rr73.push(h);
                        changed = true;
                    }
                }
                // A call, with or without a grid. Already in the pile-up: just
                // refresh what we heard, since the report we send is their
                // signal *now*.
                payload => {
                    let grid = match &payload {
                        Payload::Grid(g) => Some(g.clone()),
                        _ => d.grid.clone(),
                    };
                    let known = self
                        .working
                        .iter_mut()
                        .chain(self.owed_rr73.iter_mut())
                        .chain(self.queue.iter_mut())
                        .find(|h| h.call == from);
                    match known {
                        Some(h) => {
                            h.snr = d.snr_db;
                            if h.grid.is_none() {
                                h.grid = grid;
                            }
                        }
                        None => {
                            self.queue.push(Hound {
                                call: from.to_string(),
                                grid,
                                snr: d.snr_db,
                                rpt_rcvd: None,
                                started_utc: now_utc,
                                calls_sent: 0,
                            });
                            changed = true;
                        }
                    }
                }
            }
        }
        changed
    }

    /// Compute this transmission without committing it.
    ///
    /// Closing contacts come first — they are the ones that pay double, since
    /// each takes a fresh caller along with it — then repeats to Hounds that
    /// have not rogered, then new contacts in whatever slots are left.
    pub fn plan(&self, cfg: &DigiConfig) -> FoxPlan {
        let slots = cfg.fox_slots.clamp(1, FOX_MAX_SLOTS) as usize;
        let my_call = cfg.my_call.trim();
        let mut fresh = self.ranked_queue();
        let mut plan = FoxPlan::default();

        for h in &self.owed_rr73 {
            if plan.messages.len() == slots {
                break;
            }
            match fresh.pop() {
                // One signal, two contacts: goodbye to one Hound and a report
                // to the next.
                Some(n) => {
                    plan.messages.push(format!(
                        "{} RR73; {} {} {}",
                        h.call,
                        n.call,
                        my_call,
                        fmt_report(n.snr)
                    ));
                    plan.start.push(n.call.clone());
                }
                None => plan.messages.push(format!("{} {} RR73", h.call, my_call)),
            }
            plan.close.push(h.call.clone());
        }
        for h in &self.working {
            if plan.messages.len() == slots {
                break;
            }
            plan.messages.push(format!("{} {} {}", h.call, my_call, fmt_report(h.snr)));
            plan.repeat.push(h.call.clone());
        }
        while plan.messages.len() < slots {
            let Some(n) = fresh.pop() else { break };
            plan.messages.push(format!("{} {} {}", n.call, my_call, fmt_report(n.snr)));
            plan.start.push(n.call.clone());
        }
        // Nobody in the pile-up: say we're here.
        if plan.messages.is_empty() {
            let grid: String = cfg.my_grid.chars().take(4).collect();
            plan.messages.push(DigiConfig::fill(&cfg.msg_cq, my_call, &grid, "", None));
        }
        plan
    }

    /// Commit a plan that has gone out: log the closed contacts, promote the
    /// new ones, and give up on Hounds that never roger.
    pub fn apply(&mut self, plan: &FoxPlan, cfg: &DigiConfig, mode: Mode, now_utc: i64) {
        for call in &plan.close {
            if let Some(pos) = self.owed_rr73.iter().position(|h| &h.call == call) {
                let h = self.owed_rr73.remove(pos);
                self.log(&h, cfg, mode, now_utc);
            }
        }
        for call in &plan.repeat {
            if let Some(h) = self.working.iter_mut().find(|h| &h.call == call) {
                h.calls_sent += 1;
            }
        }
        // A Hound that has been sent its report `max_tx_repeats` times without
        // rogering is not coming back — drop it rather than hold a slot open
        // for the rest of the operation.
        let limit = cfg.max_tx_repeats;
        if limit > 0 {
            self.working.retain(|h| h.calls_sent < limit);
        }
        for call in &plan.start {
            if let Some(pos) = self.queue.iter().position(|h| &h.call == call) {
                let mut h = self.queue.remove(pos);
                h.started_utc = now_utc;
                h.calls_sent = 1;
                self.working.push(h);
            }
        }
    }

    fn log(&mut self, h: &Hound, cfg: &DigiConfig, mode: Mode, now_utc: i64) {
        self.worked.insert(h.call.to_ascii_uppercase());
        self.completed.push_back(QsoRecord {
            call: h.call.clone(),
            grid: h.grid.clone(),
            rst_sent: Some(h.snr),
            rst_rcvd: h.rpt_rcvd,
            mode: mode.label().to_string(),
            start_utc: h.started_utc,
            end_utc: now_utc,
            my_call: cfg.my_call.clone(),
            my_grid: cfg.my_grid.clone(),
            // freq_hz / band are filled by the controller, which knows the dial.
            ..Default::default()
        });
    }

    /// The queue in the order callers will be taken — *worst first*, so `pop`
    /// yields the best. A station already worked this session goes last whatever
    /// its signal; beyond that a new DXCC entity beats a repeat one and the
    /// strongest signal beats the rest, because it is the one that will complete.
    fn ranked_queue(&self) -> Vec<Hound> {
        let mut q = self.queue.clone();
        let entities: HashSet<&str> =
            self.worked.iter().filter_map(|c| resolve_callsign(c)).map(|e| e.name).collect();
        q.sort_by_key(|h| {
            let fresh = !self.worked.contains(&h.call.to_ascii_uppercase());
            let new_entity = resolve_callsign(&h.call).is_some_and(|e| !entities.contains(e.name));
            (fresh, h.snr.div_euclid(6), new_entity, h.snr)
        });
        q
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg(slots: u8) -> DigiConfig {
        DigiConfig {
            my_call: "DX1FOX".into(),
            my_grid: "AA00".into(),
            fox_slots: slots,
            ..Default::default()
        }
    }

    /// A message addressed to the fox by `from`.
    fn to_fox(from: &str, payload: &str, snr: i16) -> Decode {
        Decode {
            slot_utc: 0,
            snr_db: snr,
            dt: 0.1,
            audio_hz: 1500.0,
            message: format!("DX1FOX {from} {payload}"),
            to: Some("DX1FOX".into()),
            from: Some(from.into()),
            grid: None,
            is_cq: false,
            cq_to: None,
            free_text: false,
            rr73_to: None,
        }
    }

    #[test]
    fn an_empty_pile_up_calls_cq() {
        let fox = Fox::new();
        assert!(fox.is_idle());
        assert_eq!(fox.plan(&cfg(3)).messages, ["CQ DX1FOX AA00"]);
    }

    #[test]
    fn callers_are_reported_to_in_parallel_up_to_the_slot_count() {
        let mut fox = Fox::new();
        fox.on_rx(
            &[
                to_fox("K1ABC", "FN42", -5),
                to_fox("W9XYZ", "EM48", -12),
                to_fox("G4ABC", "IO91", -8),
            ],
            "DX1FOX",
            100,
        );
        // Two slots: the two best signals go out, the third waits its turn.
        let plan = fox.plan(&cfg(2));
        assert_eq!(plan.messages, ["K1ABC DX1FOX -05", "G4ABC DX1FOX -08"]);
        fox.apply(&plan, &cfg(2), Mode::Ft8, 100);
        assert_eq!(fox.status().iter().filter(|c| c.working).count(), 2);
        // Next transmission repeats to both and still has nobody spare.
        assert_eq!(fox.plan(&cfg(2)).messages, ["K1ABC DX1FOX -05", "G4ABC DX1FOX -08"]);
    }

    #[test]
    fn a_rogered_report_is_closed_and_the_next_caller_rides_along() {
        let mut fox = Fox::new();
        let c = cfg(1);
        fox.on_rx(&[to_fox("K1ABC", "FN42", -5)], "DX1FOX", 100);
        let plan = fox.plan(&c);
        assert_eq!(plan.messages, ["K1ABC DX1FOX -05"]);
        fox.apply(&plan, &c, Mode::Ft8, 100);

        // K1ABC rogers; W9XYZ calls in the same slot.
        assert!(fox.on_rx(
            &[to_fox("K1ABC", "R-11", -5), to_fox("W9XYZ", "EM48", -12)],
            "DX1FOX",
            115
        ));
        // One signal closes K1ABC and starts W9XYZ.
        let plan = fox.plan(&c);
        assert_eq!(plan.messages, ["K1ABC RR73; W9XYZ DX1FOX -12"]);
        fox.apply(&plan, &c, Mode::Ft8, 130);

        let rec = fox.take_completed().expect("K1ABC logged when the RR73 went out");
        assert_eq!(rec.call, "K1ABC");
        assert_eq!(rec.rst_sent, Some(-5));
        assert_eq!(rec.rst_rcvd, Some(-11));
        assert_eq!(rec.my_call, "DX1FOX");
        assert_eq!(rec.end_utc, 130);
        // W9XYZ is now the one being worked.
        assert_eq!(fox.plan(&c).messages, ["W9XYZ DX1FOX -12"]);
    }

    #[test]
    fn a_lone_closing_contact_still_gets_its_rr73() {
        let mut fox = Fox::new();
        let c = cfg(3);
        fox.on_rx(&[to_fox("K1ABC", "FN42", -5)], "DX1FOX", 100);
        let plan = fox.plan(&c);
        fox.apply(&plan, &c, Mode::Ft8, 100);
        fox.on_rx(&[to_fox("K1ABC", "R-11", -5)], "DX1FOX", 115);
        // Nobody waiting, so the RR73 travels as an ordinary message.
        assert_eq!(fox.plan(&c).messages, ["K1ABC DX1FOX RR73"]);
    }

    #[test]
    fn a_hound_that_never_rogers_is_dropped() {
        let c = DigiConfig { max_tx_repeats: 2, ..cfg(1) };
        let mut fox = Fox::new();
        fox.on_rx(&[to_fox("K1ABC", "FN42", -5)], "DX1FOX", 100);
        for _ in 0..2 {
            let plan = fox.plan(&c);
            fox.apply(&plan, &c, Mode::Ft8, 100);
        }
        assert!(fox.is_idle(), "gave up after two unanswered reports");
        assert_eq!(fox.plan(&c).messages, ["CQ DX1FOX AA00"]);
    }

    #[test]
    fn a_repeat_caller_waits_behind_a_station_not_yet_worked() {
        let mut fox = Fox::new();
        let c = cfg(1);
        // Work K1ABC through to a log entry.
        fox.on_rx(&[to_fox("K1ABC", "FN42", -2)], "DX1FOX", 100);
        let plan = fox.plan(&c);
        fox.apply(&plan, &c, Mode::Ft8, 100);
        fox.on_rx(&[to_fox("K1ABC", "R-11", -2)], "DX1FOX", 115);
        let plan = fox.plan(&c);
        fox.apply(&plan, &c, Mode::Ft8, 130);
        assert!(fox.take_completed().is_some());

        // They call again alongside a much weaker station we've never worked.
        fox.on_rx(&[to_fox("K1ABC", "FN42", -2), to_fox("W9XYZ", "EM48", -20)], "DX1FOX", 145);
        assert_eq!(fox.plan(&c).messages, ["W9XYZ DX1FOX -20"]);
    }
}
