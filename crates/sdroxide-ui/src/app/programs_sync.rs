//! The PROGRAMS window's state, kept by the station.
//!
//! TOUT, the starred stations, the filters, the sort and the colours are the
//! station's: a client that is connected to one reads them from it at start and
//! writes every change back, so a browser keeps nothing of them and the same
//! window opens on every screen. A program running on the radio's own machine
//! has no station to ask and keeps using its config file.

use sdroxide_types::ProgramsState;

use super::SdroxideApp;

/// Where the state is read and written: the station this client is connected to.
pub(in crate::app) fn programs_url(peer_url: &str) -> Option<String> {
    let base = crate::login::http_base(&crate::login::station_key(peer_url))?;
    Some(format!("{base}/programs-state"))
}

/// How long to wait after a request that got no answer.
const RETRY_SECS: i64 = 10;

#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Phase {
    /// Nothing asked yet: the first thing sent is a read.
    #[default]
    Unasked,
    /// The read is out.
    Reading,
    /// The station's state is known; changes are written.
    Ready,
    /// The station has no `/programs-state` (404, 405 or 501 on the read):
    /// nothing more is sent this session.
    Absent,
}

/// What was last asked, to know what an answer is the answer to.
#[derive(Clone, Copy, PartialEq, Debug, Default)]
enum Asked {
    #[default]
    Nothing,
    Read,
    Write,
}

/// The bookkeeping of the exchange with the station. No I/O: the caller sends
/// what [`Self::next_request`] returns and hands the answer to [`Self::deliver`].
#[derive(Default)]
pub(in crate::app) struct ProgramsSync {
    phase: Phase,
    asked: Asked,
    in_flight: bool,
    retry_at: i64,
    /// The state the station is known to hold.
    synced: Option<ProgramsState>,
}

/// What [`ProgramsSync::deliver`] asks the app to do.
#[derive(Debug, PartialEq)]
pub(in crate::app) enum Outcome {
    /// Nothing: a write that went through, or an answer that was no use.
    Nothing,
    /// Show this state: it is what the station holds.
    Apply(ProgramsState),
}

impl ProgramsSync {
    /// The request due this frame: `Some(None)` is a read, `Some(Some(body))` a
    /// write of `current`. A write waits while a text box is being typed in, so
    /// a search is one write and not one per letter.
    pub(in crate::app) fn next_request(
        &mut self,
        has_station: bool,
        current: &ProgramsState,
        typing: bool,
        now: i64,
    ) -> Option<Option<String>> {
        if !has_station || self.in_flight || now < self.retry_at {
            return None;
        }
        match self.phase {
            Phase::Unasked => {
                self.phase = Phase::Reading;
                self.asked = Asked::Read;
                self.in_flight = true;
                Some(None)
            }
            Phase::Reading | Phase::Absent => None,
            Phase::Ready => {
                if typing || self.synced.as_ref() == Some(current) {
                    return None;
                }
                let body = serde_json::to_string(current).ok()?;
                self.asked = Asked::Write;
                self.in_flight = true;
                Some(Some(body))
            }
        }
    }

    /// The answer to the last request. `current` is what this client holds now:
    /// when the station has nothing yet (it answers the default) and this client
    /// does, the client's is kept and written, so an existing setup moves to the
    /// station once instead of being wiped by it.
    pub(in crate::app) fn deliver(
        &mut self,
        status: Option<u16>,
        body: &str,
        current: &ProgramsState,
        now: i64,
    ) -> Outcome {
        self.in_flight = false;
        let state = (status == Some(200))
            .then(|| serde_json::from_str::<ProgramsState>(body).ok())
            .flatten();
        let Some(state) = state else {
            if self.asked == Asked::Read && matches!(status, Some(404 | 405 | 501)) {
                // The station has no such route (an older binary): it will not
                // grow one during this session, so stop asking. The config
                // file keeps the state, as without a station.
                self.phase = Phase::Absent;
                return Outcome::Nothing;
            }
            // No answer, a refusal, or something that is not a state: try again.
            self.retry_at = now + RETRY_SECS;
            if self.asked == Asked::Read {
                self.phase = Phase::Unasked;
            }
            return Outcome::Nothing;
        };
        match self.asked {
            Asked::Read => {
                self.phase = Phase::Ready;
                if state == ProgramsState::default() && *current != ProgramsState::default() {
                    // The station is empty: leave `synced` as the empty state so
                    // the difference is written next frame.
                    self.synced = Some(state);
                    Outcome::Nothing
                } else {
                    self.synced = Some(state.clone());
                    Outcome::Apply(state)
                }
            }
            _ => {
                self.synced = Some(state);
                Outcome::Nothing
            }
        }
    }
}

impl SdroxideApp {
    /// What the window holds now, as the station keeps it.
    pub(in crate::app) fn programs_state(&self) -> ProgramsState {
        ProgramsState {
            filters: self.ui_settings.programs.clone(),
            favourites: self.broadcast_favs.clone(),
            colour_all: self.ui_settings.programs_colour_all,
            colours: self.ui_settings.programs_colours.clone(),
        }
    }

    /// Show the station's state: TOUT, the stars, the filters and the colours.
    pub(in crate::app) fn apply_programs_state(&mut self, s: ProgramsState) {
        self.broadcast_all = s.filters.all;
        self.broadcast_favs = s.favourites;
        self.schedule.apply_filters(&s.filters);
        self.ui_settings.programs = s.filters;
        self.ui_settings.programs_colour_all = s.colour_all;
        self.ui_settings.programs_colours = s.colours;
    }

    /// Once a frame: take what the station answered, and send the read or the
    /// write that is due. Without a station (the program on the radio's own
    /// machine) nothing happens and the config file keeps the state.
    pub(in crate::app) fn programs_sync_tick(&mut self, ctx: &eframe::egui::Context) {
        let peer = self.ctrl.peer_url();
        let now = crate::time::now_unix();
        for (status, body) in std::mem::take(&mut self.aircraft_online.programs_inbox) {
            let current = self.programs_state();
            if let Outcome::Apply(state) = self.programs_sync.deliver(status, &body, &current, now)
            {
                self.apply_programs_state(state);
            }
        }
        let current = self.programs_state();
        let typing = ctx.memory(|m| m.focused()).is_some();
        if let Some(post) = self.programs_sync.next_request(peer.is_some(), &current, typing, now) {
            self.aircraft_online.ask_programs(ctx, peer.as_deref(), post);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lit() -> ProgramsState {
        let mut s = ProgramsState::default();
        s.filters.all = true;
        s.favourites = vec!["BBC".into(), "RFI".into()];
        s.colours.insert("BBC".into(), [1, 2, 3]);
        s
    }

    fn json(s: &ProgramsState) -> String {
        serde_json::to_string(s).unwrap()
    }

    #[test]
    fn the_first_thing_sent_is_a_read_and_only_once() {
        let mut s = ProgramsSync::default();
        let d = ProgramsState::default();
        assert_eq!(s.next_request(false, &d, false, 0), None, "no station, nothing");
        assert_eq!(s.next_request(true, &d, false, 0), Some(None));
        assert_eq!(s.next_request(true, &d, false, 0), None, "one at a time");
    }

    #[test]
    fn what_the_station_holds_is_shown() {
        let mut s = ProgramsSync::default();
        s.next_request(true, &ProgramsState::default(), false, 0);
        let out = s.deliver(Some(200), &json(&lit()), &ProgramsState::default(), 0);
        assert_eq!(out, Outcome::Apply(lit()), "TOUT comes back from the station");
        assert_eq!(s.next_request(true, &lit(), false, 1), None, "and nothing is written back");
    }

    #[test]
    fn a_change_is_written_once_and_then_it_is_quiet() {
        let mut s = ProgramsSync::default();
        s.next_request(true, &ProgramsState::default(), false, 0);
        s.deliver(Some(200), &json(&ProgramsState::default()), &ProgramsState::default(), 0);
        let now = lit();
        let req = s.next_request(true, &now, false, 1).expect("TOUT lit is a write");
        assert_eq!(req, Some(json(&now)));
        assert_eq!(s.next_request(true, &now, false, 1), None, "while the write is out");
        assert_eq!(s.deliver(Some(200), &json(&now), &now, 1), Outcome::Nothing);
        assert_eq!(s.next_request(true, &now, false, 2), None, "stored, so quiet");
    }

    #[test]
    fn nothing_is_written_while_a_text_box_has_the_focus() {
        let mut s = ProgramsSync::default();
        s.next_request(true, &ProgramsState::default(), false, 0);
        s.deliver(Some(200), &json(&ProgramsState::default()), &ProgramsState::default(), 0);
        assert_eq!(s.next_request(true, &lit(), true, 1), None);
        assert!(s.next_request(true, &lit(), false, 1).is_some(), "typing done: written");
    }

    #[test]
    fn a_station_with_nothing_yet_takes_what_this_client_has() {
        let mut s = ProgramsSync::default();
        s.next_request(true, &lit(), false, 0);
        let out = s.deliver(Some(200), &json(&ProgramsState::default()), &lit(), 0);
        assert_eq!(out, Outcome::Nothing, "the client's setup is not wiped");
        assert_eq!(s.next_request(true, &lit(), false, 1), Some(Some(json(&lit()))));
    }

    #[test]
    fn a_read_that_failed_is_asked_again_after_a_pause() {
        let mut s = ProgramsSync::default();
        let d = ProgramsState::default();
        s.next_request(true, &d, false, 0);
        assert_eq!(s.deliver(Some(401), "", &d, 0), Outcome::Nothing);
        assert_eq!(s.next_request(true, &d, false, 5), None, "not at once");
        assert_eq!(s.next_request(true, &d, false, RETRY_SECS), Some(None));
        assert_eq!(s.deliver(None, "", &d, RETRY_SECS), Outcome::Nothing);
        assert_eq!(s.deliver(Some(200), "garbage", &d, RETRY_SECS), Outcome::Nothing);
    }

    #[test]
    fn a_station_without_the_route_is_asked_once() {
        for status in [404u16, 405, 501] {
            let mut s = ProgramsSync::default();
            let d = ProgramsState::default();
            assert_eq!(s.next_request(true, &d, false, 0), Some(None));
            assert_eq!(s.deliver(Some(status), "", &d, 0), Outcome::Nothing);
            for t in [RETRY_SECS, 1000, 100_000] {
                assert_eq!(s.next_request(true, &lit(), false, t), None, "{status} at {t}");
            }
        }
        // A server error or a refusal is not "no such route": keep trying.
        let mut s = ProgramsSync::default();
        let d = ProgramsState::default();
        s.next_request(true, &d, false, 0);
        s.deliver(Some(500), "", &d, 0);
        assert_eq!(s.next_request(true, &d, false, RETRY_SECS), Some(None));
    }

    #[test]
    fn a_write_that_failed_is_sent_again() {
        let mut s = ProgramsSync::default();
        let d = ProgramsState::default();
        s.next_request(true, &d, false, 0);
        s.deliver(Some(200), &json(&d), &d, 0);
        assert!(s.next_request(true, &lit(), false, 1).is_some());
        s.deliver(None, "", &lit(), 1);
        assert!(s.next_request(true, &lit(), false, 1 + RETRY_SECS).is_some());
    }

    #[test]
    fn the_address_is_the_stations() {
        assert_eq!(
            programs_url("wss://shack.example/ws").as_deref(),
            Some("https://shack.example/programs-state")
        );
        assert_eq!(programs_url("not an address"), None);
    }
}
