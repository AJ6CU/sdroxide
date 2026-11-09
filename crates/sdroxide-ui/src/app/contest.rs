//! The contest logger window.
//!
//! Mode-agnostic: the operator types the exchange by hand for CW, SSB or
//! anything else, and the FT8 side auto-fills the same entry when it drives one
//! of the contests that has an FT8 layout. A session is the operator's own
//! state (see [`sdroxide_types::ContestSession`]); the QSOs it logs go into the
//! ordinary logbook, tagged with the contest, so scoring and the Cabrillo
//! export read the same rows everything else does.

use eframe::egui;
use sdroxide_types::{Command, ContestId, ContestSession, Exchange, QsoRecord};

use super::SdroxideApp;
use crate::i18n::tr;

/// The in-progress entry: what the operator is typing for the station being
/// worked. Session-only, cleared after every logged QSO.
#[derive(Default)]
pub(in crate::app) struct ContestEntry {
    pub call: String,
    /// The report we send. Defaults from the mode — `599` on CW, `59` on phone
    /// — and stays editable, because the operator may have sent something else.
    pub rst_sent: String,
    /// The report they gave us.
    pub rst_rcvd: String,
    /// **One box per received exchange element** beyond the report — the serial
    /// and the locator for EU VHF, the zone for CQ WW, the text for a CB
    /// activity. A single box kept whichever element was typed last and threw
    /// the others away.
    pub fields: Vec<String>,
}

impl SdroxideApp {
    pub(in crate::app) fn contest_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        if !self.show_contest {
            return;
        }
        let show = self.tool_window(ctx, "contest", &tr!("contest-title"), [660.0, 540.0], true, |me, ui| {
            me.contest_body(ui, cmds);
        });
        self.show_contest = show;
    }

    fn contest_body(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        if self.contest.is_none() {
            self.contest_setup(ui, cmds);
        } else {
            self.contest_run(ui, cmds);
        }
    }

    /// The pre-session setup: pick the contest and enter our own exchange.
    fn contest_setup(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        ui.label(tr!("contest-intro"));
        ui.add_space(8.0);
        let mut picked = self.contest_pick;
        ui.horizontal_wrapped(|ui| {
            for c in ContestId::CHOICES {
                if crate::chrome::chip(ui, picked == c, c.label()).clicked() {
                    picked = c;
                }
            }
        });
        self.contest_pick = picked;
        let spec = picked.spec();
        ui.add_space(8.0);
        ui.label(egui::RichText::new(spec.name).strong());
        ui.label(tr!("contest-you-send", exchange = exchange_hint(spec.sent)));
        ui.label(tr!("contest-they-send", exchange = exchange_hint(spec.rcvd)));
        if picked == ContestId::CbActivity {
            ui.label(
                egui::RichText::new(tr!("contest-cb-note"))
                .size(11.0),
            );
        }
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label(tr!("contest-your-exchange"));
            crate::chrome::field(ui, egui::TextEdit::singleline(&mut self.contest_my_exchange));
        });
        ui.add_space(8.0);
        if crate::chrome::chip(ui, true, format!(" {} ", tr!("contest-start"))).clicked() {
            let now = crate::time::now_unix_f64() as i64;
            // Seeded from the log, so a session stopped and restarted — or the
            // program itself restarted — carries on from the last serial
            // instead of sending 001 again.
            self.contest = Some(ContestSession::seeded(
                picked,
                self.contest_my_exchange.trim().into(),
                now,
                &self.qso_log,
            ));
            self.contest_entry = Default::default();
            self.reset_entry_reports();
            // Tell the digi engine which FT8 contest layout to send, when this
            // contest has one **and the band carries it**. A hand-typed CW/SSB
            // contest sets nothing, and neither does any contest on 11 m — see
            // `digi_contest_for`. We remember what the engine held so STOP can
            // put it back rather than leaving the contest's calling message in
            // force after the session ends.
            //
            // Narrowly, on purpose: this panel is not the digi panel and its
            // `digi_cfg_edit` copy is not authoritative. Pushing a whole
            // `DigiConfig` from here would roll back whatever the engine holds
            // that this copy is stale on, and would clear an FT8 contest layout
            // the operator had already set.
            match digi_contest_for(picked, self.state.band) {
                Some(mode) => {
                    self.contest_prev_digi = Some(self.digi_cfg_edit.contest);
                    cmds.push(sdroxide_types::Command::SetDigiContest(mode));
                }
                None => self.contest_prev_digi = None,
            }
        }
    }

    /// Fill the report boxes from the mode in force, and size the exchange
    /// boxes to the contest's exchange.
    ///
    /// Called when a session starts so the boxes match it, and when a QSO is
    /// logged so the next one starts fresh without losing the report default.
    fn reset_entry_reports(&mut self) {
        let mode = self.state.rx[0].mode.label();
        let rst = default_report(mode).to_string();
        self.contest_entry.rst_sent = rst.clone();
        self.contest_entry.rst_rcvd = rst;
        let n = self.contest.as_ref().map(|s| s.contest.received_fields().len()).unwrap_or(0);
        self.contest_entry.fields = vec![String::new(); n];
    }

    /// The running session: entry, dupes, score and the session's log.
    fn contest_run(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        let Some(contest) = self.contest.as_ref().map(|s| s.contest) else { return };
        let spec = contest.spec();
        let mine = self.session_qsos();
        let score = sdroxide_types::score(&mine, contest);
        let now = crate::time::now_unix_f64() as i64;
        let r10 = sdroxide_types::rate(&mine, now, 600);
        let r60 = sdroxide_types::rate(&mine, now, 3600);

        let mut stop = false;
        ui.horizontal_wrapped(|ui| {
            ui.label(egui::RichText::new(contest.label()).strong());
            ui.label(tr!(
                "contest-score-line",
                qsos = score.qsos,
                points = score.points,
                mults = score.mults,
                total = score.total
            ));
            ui.label(tr!("contest-rate", r10 = r10, r60 = r60));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if crate::chrome::chip(ui, false, format!(" {} ", tr!("contest-stop"))).clicked() {
                    stop = true;
                }
            });
        });
        if stop {
            self.contest = None;
            // Put the digi engine back the way the session found it. Without
            // this the contest's layout stays in force after the session has
            // ended — the CQ keeps calling in the contest's format and the digi
            // panel shows a setting the operator never chose.
            if let Some(prev) = self.contest_prev_digi.take() {
                cmds.push(sdroxide_types::Command::SetDigiContest(prev));
            }
            return;
        }
        if spec.multiplier != sdroxide_types::Multiplier::None {
            ui.label(
                egui::RichText::new(tr!("contest-estimate"))
                    .size(10.0)
                    .color(crate::theme::gray(140)),
            );
        }
        ui.separator();

        // ── The entry form ────────────────────────────────────────────────
        // The dupe check looks at the **session's** QSOs, not the whole
        // logbook. Running it over everything made any pre-contest QSO with
        // the station — one from last month, on the same band — light up as a
        // dupe before the contest had even started.
        let dupe = {
            let call = self.contest_entry.call.trim();
            !call.is_empty()
                && sdroxide_types::worked_before(&mine, call, self.state.band.label(), "", 0)
        };
        if self.contest_entry.fields.len() != contest.received_fields().len() {
            self.contest_entry.fields = vec![String::new(); contest.received_fields().len()];
        }
        ui.horizontal_wrapped(|ui| {
            ui.label(tr!("contest-call"));
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut self.contest_entry.call).desired_width(100.0),
            );
            if dupe {
                ui.label(egui::RichText::new(tr!("contest-dupe")).strong().color(crate::theme::ALERT()));
            }
            ui.label(tr!("contest-sent"));
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut self.contest_entry.rst_sent).desired_width(46.0),
            );
            ui.label(tr!("contest-rcvd"));
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut self.contest_entry.rst_rcvd).desired_width(46.0),
            );
            // One box per received element, labelled from the contest's own
            // exchange — so EU VHF asks for its serial *and* its locator.
            for (i, ex) in contest.received_fields().iter().enumerate() {
                ui.label(ex.label());
                crate::chrome::field(
                    ui,
                    egui::TextEdit::singleline(&mut self.contest_entry.fields[i])
                        .desired_width(80.0),
                );
            }
        });
        ui.add_space(4.0);
        let serial = self.contest.as_ref().map(|s| s.next_serial).unwrap_or(1);
        ui.horizontal(|ui| {
            let stx = contest.sends_serial().then_some(serial);
            let ours = self.contest.as_ref().map(|s| s.sent_exchange(stx)).unwrap_or_default();
            let rst = self.contest_entry.rst_sent.trim();
            if ours.is_empty() {
                ui.label(tr!("contest-sending", rst = rst));
            } else {
                ui.label(tr!("contest-sending-ours", rst = rst, ours = ours));
            }
            ui.label(format!("· {} {}", self.state.band.label(), self.state.rx[0].mode.label()));
            if crate::chrome::chip(ui, true, format!(" {} ", tr!("contest-log"))).clicked() {
                self.log_contest_qso(cmds);
            }
            if crate::chrome::chip(ui, false, " CABRILLO ").clicked() {
                let cab = sdroxide_types::to_cabrillo(
                    contest,
                    &self.my_call(),
                    &self.contest_my_exchange,
                    &mine,
                    &format!("sdroxide {}", sdroxide_version::VERSION),
                );
                crate::download::save("sdroxide.cab", cab.as_bytes());
            }
        });
        ui.separator();

        // ── The session's log ─────────────────────────────────────────────
        egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
            egui::Grid::new("contest-log-grid")
                .num_columns(5)
                .spacing([12.0, 2.0])
                .striped(true)
                .show(ui, |ui| {
                    for q in mine.iter().rev().take(200) {
                        let (_, _, _, h, mi, _) = sdroxide_types::utc_ymd_hms(q.start_utc);
                        ui.label(format!("{h:02}{mi:02}"));
                        crate::callsign_card::on_hover(ui.label(&q.call), &q.call);
                        ui.label(&q.band);
                        ui.label(&q.mode);
                        ui.label(&q.srx_string);
                        ui.end_row();
                    }
                });
        });
    }

    /// The current session's QSOs: the log's rows tagged with this contest and
    /// logged since the session started.
    fn session_qsos(&self) -> Vec<QsoRecord> {
        let Some(s) = self.contest.as_ref() else { return Vec::new() };
        let id = s.contest.log_id();
        self.qso_log
            .iter()
            .filter(|q| q.contest_id == id && q.start_utc >= s.started_utc)
            .cloned()
            .collect()
    }

    /// Log the typed entry and advance the serial.
    pub(in crate::app) fn log_contest_qso(&mut self, cmds: &mut Vec<Command>) {
        let Some(session) = self.contest.as_ref() else { return };
        let call = self.contest_entry.call.trim().to_ascii_uppercase();
        if call.is_empty() {
            return;
        }
        let contest = session.contest;
        let serial = contest.sends_serial().then_some(session.next_serial);
        // Our side: the serial and our own exchange on one line. The report is
        // what the operator typed, or the mode's default if they cleared it.
        let stx_string = session.sent_exchange(serial);
        let my_exchange = session.my_exchange.clone();
        let now = crate::time::now_unix_f64() as i64;

        // Their side: each received element has its own box, so nothing is
        // overwritten by the element next to it.
        let values: Vec<String> =
            self.contest_entry.fields.iter().map(|s| s.trim().to_string()).collect();
        let mut srx = None;
        let mut cq_zone = None;
        let mut grid = None;
        let mut state = String::new();
        for (ex, v) in contest.received_fields().iter().zip(values.iter()) {
            match ex {
                Exchange::Serial => srx = v.parse().ok(),
                Exchange::CqZone => cq_zone = v.parse().ok(),
                Exchange::Grid => {
                    if !v.is_empty() {
                        grid = Some(v.to_ascii_uppercase());
                    }
                }
                Exchange::State => state = v.to_ascii_uppercase(),
                _ => {}
            }
        }
        let mode = self.state.rx[0].mode.label().to_string();
        let mut rec = QsoRecord {
            call,
            grid,
            rst_sent: parse_rst(&self.contest_entry.rst_sent)
                .or_else(|| Some(default_report(&mode))),
            rst_rcvd: parse_rst(&self.contest_entry.rst_rcvd)
                .or_else(|| Some(default_report(&mode))),
            cq_zone,
            state,
            freq_hz: self.on_air_freq_hz(),
            mode,
            band: self.state.band.label().to_string(),
            start_utc: now,
            end_utc: now,
            my_call: self.my_call(),
            // The sponsor's id, not the UI label — an ADIF `CONTEST_ID` read by
            // anyone else's logger has to say what the sponsor calls it.
            contest_id: contest.log_id().to_string(),
            stx: serial,
            srx,
            stx_string: if stx_string.is_empty() { my_exchange } else { stx_string },
            srx_string: values.join(" "),
            ..Default::default()
        };
        // The engine's `LogQso` only fans the contact out to WSJT-X and N1MM;
        // it does not own the logbook. The UI does, so a hand-typed contest
        // contact has to be written here as well, or it reaches nobody — not
        // the session's own list, not the score, not the Cabrillo export, not
        // the logbook. This is the same pairing the LOGBOOK window's own entry
        // uses, for the same reason (issue #341).
        rec.id = self.next_log_id();
        cmds.push(Command::LogQso(Box::new(rec.clone())));
        if let Some((qso_id, adif, targets)) =
            crate::app::net::auto_upload_adif(&self.net_cfg_edit, &rec)
        {
            self.pending_uploads.push((qso_id, adif, targets));
        }
        let call = rec.call.clone();
        self.last_logged_qso_id = Some(rec.id);
        self.session_qso_ids.insert(rec.id);
        self.qso_log.push(rec);
        self.session_qsos += 1;
        crate::app::persist::persist_qso_log(&self.qso_log);
        self.queue_lookup(call);
        if let Some(s) = self.contest.as_mut()
            && s.contest.sends_serial()
        {
            s.next_serial = sdroxide_types::next_contest_serial(s.next_serial);
        }
        self.contest_entry = Default::default();
        // Start the next entry with the report default and the right number of
        // exchange boxes, rather than blank ones.
        self.reset_entry_reports();
    }
}

/// "RST + SERIAL + GRID" — the exchange spelled out for the setup screen.
fn exchange_hint(fields: &[Exchange]) -> String {
    if fields.is_empty() {
        return "—".to_string();
    }
    fields.iter().map(|f| f.label()).collect::<Vec<_>>().join(" + ")
}

/// The FT8 message layout a contest session should put the digi engine into,
/// or `None` when it should leave it alone.
///
/// **11 m is never given an amateur layout.** The RTTY Roundup's calling
/// message is literally `CQ RU <call>`, which on the citizens' band is longer
/// than a Type-4 call can carry — the whole identifier is capped at eleven
/// characters — and is an exchange nobody on the band sends. The CB activity
/// is the 11 m contest format and it is typed by hand, so on 11 m every contest
/// leaves the engine as it found it. (The engine refuses the layout on 11 m as
/// well, so a setting persisted from another band cannot reach the air either;
/// this is the half that stops it being written in the first place.)
///
/// `None` and `ContestMode::None` are deliberately different answers: the first
/// is "change nothing", the second is "clear it". Starting a hand-typed CW/SSB
/// contest must not clear an FT8 layout the operator set from the digi panel.
fn digi_contest_for(
    c: ContestId,
    band: sdroxide_types::Band,
) -> Option<sdroxide_types::ContestMode> {
    if band == sdroxide_types::Band::M11 {
        return None;
    }
    match c {
        ContestId::EuVhf => Some(sdroxide_types::ContestMode::EuVhf),
        ContestId::CqWpx | ContestId::Generic => Some(sdroxide_types::ContestMode::RttyRoundup),
        _ => None,
    }
}

fn parse_rst(s: &str) -> Option<i16> {
    s.trim().parse().ok()
}

/// The report we send when the operator has not typed one: `599` on CW, and
/// `59` on everything else.
///
/// Copying the *received* report into the sent one — as the original did — told
/// the other station we heard them exactly as well as they heard us, which is
/// not a claim the operator made.
fn default_report(mode: &str) -> i16 {
    if sdroxide_types::cabrillo_mode(mode) == "CW" { 599 } else { 59 }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdroxide_types::Band;

    /// 11 m is never handed an amateur FT8 contest layout.
    ///
    /// The RTTY Roundup's calling message is `CQ RU <call>`, which cannot fit
    /// a Type-4 CB call (the whole identifier is capped at eleven characters),
    /// so on 11 m the window must leave the digi engine exactly as it found it.
    /// `None` here means "send nothing", which is what keeps the operator's own
    /// digi setting intact — as opposed to `ContestMode::None`, which would
    /// clear it.
    #[test]
    fn eleven_metres_is_never_given_an_amateur_ft8_layout() {
        for c in ContestId::CHOICES {
            assert_eq!(digi_contest_for(c, Band::M11), None, "{c:?} must not put a layout on 11 m");
        }
    }

    /// Off 11 m the layouts are still chosen, so the rule is the band's and not
    /// a blanket refusal.
    #[test]
    fn amateur_bands_still_get_their_ft8_layouts() {
        assert_eq!(
            digi_contest_for(ContestId::CqWpx, Band::M20),
            Some(sdroxide_types::ContestMode::RttyRoundup)
        );
        assert_eq!(
            digi_contest_for(ContestId::EuVhf, Band::M20),
            Some(sdroxide_types::ContestMode::EuVhf)
        );
        // A contest with no FT8 layout leaves the engine alone rather than
        // clearing whatever the operator set.
        assert_eq!(digi_contest_for(ContestId::CqWw, Band::M20), None);
        assert_eq!(digi_contest_for(ContestId::CbActivity, Band::M20), None);
    }
}
