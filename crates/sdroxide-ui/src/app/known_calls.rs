//! The **KNOWN** window: who the digital decoder's hash table can currently
//! resolve.
//!
//! An FT8 message addresses a station by a one-way hash, so a `<...>` cannot be
//! turned back into a callsign by arithmetic — there is nothing inside it to
//! read. The only way to name a station is to have heard it spelled out, and
//! that set was previously invisible: the operator could see a message resolve
//! and had no way to see *why*, or to browse who they currently know.
//!
//! So this lists the set itself, newest first, and — because on 11 m that is the
//! question actually worth asking — **by country**. `19DC797` is a Dutch
//! station and `4CB04` an Argentine one, resolved through WSJT-CB's own
//! numbering ahead of the amateur table, and the flag machinery is the same one
//! the decode rows already draw.

use crate::i18n::tr;
use eframe::egui::{self, RichText};
use sdroxide_types::resolve_callsign;

/// The window's state, and the last answer the engine gave.
#[derive(Default)]
pub(in crate::app) struct KnownCallsState {
    pub(in crate::app) show: bool,
    /// Free-text filter over callsign, country and DXCC prefix.
    query: String,
    /// Newest first, capped by the engine at [`KNOWN_CALLS_REPLY_MAX`].
    calls: Vec<String>,
    /// How many the table holds, which is more than `calls` when the cap bit.
    total: usize,
    /// Asked for, and no answer yet. Distinguishes "not asked" from "asked and
    /// there is genuinely nobody", which are different things to say on screen.
    asked: bool,
    /// Asked, and the worker did not answer in time.
    no_answer: bool,
}

impl KnownCallsState {
    /// A reply from the engine, which arrives as a `RadioEvent`. `None` is a
    /// real answer — this mode keeps no callsign table — and is kept apart from
    /// an empty list, which means the band is quiet.
    pub(in crate::app) fn accept(&mut self, reply: Option<sdroxide_types::KnownCallsReply>) {
        self.asked = true;
        match reply {
            Some(r) => {
                self.calls = r.calls;
                self.total = r.total;
                self.no_answer = false;
            }
            None => {
                self.calls.clear();
                self.total = 0;
                self.no_answer = true;
            }
        }
    }

    /// The window was opened: ask again, so the list is never stale from the
    /// last time it was looked at.
    pub(in crate::app) fn request(&mut self) {
        self.asked = false;
        self.no_answer = false;
    }

    /// Why the list is empty, in the operator's terms. Every branch names which
    /// half is missing rather than saying "no results", so the window never
    /// reads as a fault when it is only a mode that does not keep a table.
    fn empty_reason(&self) -> String {
        if self.no_answer {
            return tr!("knowncalls-no-table");
        }
        if !self.asked {
            return tr!("knowncalls-not-asked");
        }
        tr!("knowncalls-nobody")
    }
}

impl super::SdroxideApp {
    /// Ask the engine which callsigns it can resolve, if the window has just
    /// been opened. Collected as an ordinary command like every other, and
    /// driven from the frame loop rather than the click so the answer arrives
    /// when the window is up.
    pub(in crate::app) fn poll_known_calls(&mut self, cmds: &mut Vec<sdroxide_types::Command>) {
        if self.known_calls.show && !self.known_calls.asked {
            self.known_calls.asked = true;
            cmds.push(sdroxide_types::Command::GetKnownCalls);
        }
    }

    pub(in crate::app) fn known_calls_window(&mut self, ctx: &egui::Context) {
        if !self.known_calls.show {
            return;
        }
        let was_open = self.known_calls.show;
        let show = self.tool_window(
            ctx,
            "known-calls",
            &tr!("knowncalls-title"),
            [560.0, 520.0],
            true,
            |me, ui| {
                me.known_calls_body(ui);
            },
        );
        self.known_calls.show = show;
        // Reopening asks afresh, so the list is never the one from last time.
        if show && !was_open {
            self.known_calls.request();
        }
    }

    fn known_calls_body(&mut self, ui: &mut egui::Ui) {
        ui.label(
            RichText::new(tr!("knowncalls-intro"))
            .size(11.0)
            .color(crate::theme::gray(160)),
        );
        ui.add_space(6.0);

        ui.horizontal(|ui| {
            ui.label(RichText::new(tr!("signalid-find")).size(11.0).color(crate::theme::gray(150)));
            crate::chrome::field(
                ui,
                egui::TextEdit::singleline(&mut self.known_calls.query)
                    .desired_width(200.0)
                    .hint_text(tr!("knowncalls-find-hint")),
            );
            if crate::chrome::chip(ui, false, tr!("decodes-clear")).clicked() {
                self.known_calls.query.clear();
            }
            if crate::chrome::chip(ui, false, tr!("knowncalls-refresh")).clicked() {
                self.known_calls.request();
            }
        });
        ui.add_space(4.0);
        ui.separator();

        // The cap, said out loud: a truncated list that does not admit it is a
        // list that reads as complete.
        let shown = self.known_calls.calls.len();
        let capped = shown < self.known_calls.total;
        ui.label(
            RichText::new(if capped {
tr!("knowncalls-capped", shown = shown, total = self.known_calls.total)
            } else {
                tr!("knowncalls-known", shown = shown)
            })
            .size(10.5)
            .color(crate::theme::gray(150)),
        );

        let query = self.known_calls.query.trim().to_uppercase();
        let rows: Vec<(String, Option<sdroxide_types::EntityInfo>)> = self
            .known_calls
            .calls
            .iter()
            .filter_map(|c| {
                let info = resolve_callsign(c);
                if !query.is_empty() {
                    let hit = c.contains(&query)
                        || info.as_ref().is_some_and(|i| {
                            i.name.to_uppercase().contains(&query)
                                || i.primary_prefix.to_uppercase().contains(&query)
                                || i.flag.eq_ignore_ascii_case(&query)
                        });
                    if !hit {
                        return None;
                    }
                }
                Some((c.clone(), info))
            })
            .collect();

        if !query.is_empty() {
            ui.label(
                RichText::new(tr!("knowncalls-matches", n = rows.len()))
                    .size(10.5)
                    .color(crate::theme::gray(150)),
            );
        }

        egui::ScrollArea::vertical().auto_shrink([false, false]).id_salt("known-calls-list").show(
            ui,
            |ui| {
                if rows.is_empty() {
                    ui.add_space(6.0);
                    let reason = if !self.known_calls.query.trim().is_empty() {
                        tr!("knowncalls-no-match")
                    } else {
                        self.known_calls.empty_reason()
                    };
                    ui.label(RichText::new(reason).color(crate::theme::gray(160)));
                    return;
                }
                for (call, info) in &rows {
                    known_row(ui, call, info.as_ref());
                }
            },
        );
    }
}

fn known_row(ui: &mut egui::Ui, call: &str, info: Option<&sdroxide_types::EntityInfo>) {
    egui::Frame::new()
        .fill(crate::theme::ROW_BG())
        .stroke(egui::Stroke::new(1.0, crate::theme::LINE()))
        .inner_margin(egui::Margin::symmetric(9, 6))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.horizontal_wrapped(|ui| {
                crate::callsign_card::on_hover(
                    ui.label(RichText::new(call).monospace().strong().color(crate::theme::CYAN())),
                    call,
                );
                match info {
                    // A CB country resolves ahead of the amateur table, so this
                    // is the 11 m numbering and not a DXCC prefix guess.
                    Some(i) => {
                        ui.label(RichText::new(i.name).size(11.0));
                        ui.label(
                            RichText::new(i.primary_prefix)
                                .size(10.5)
                                .color(crate::theme::gray(150)),
                        );
                        ui.label(
                            RichText::new(i.continent).size(10.5).color(crate::theme::gray(170)),
                        );
                    }
                    None => {
                        ui.label(
                            RichText::new(tr!("knowncalls-no-country"))
                                .size(10.5)
                                .color(crate::theme::gray(170)),
                        );
                    }
                }
            });
        });
    ui.add_space(3.0);
}

#[cfg(test)]
mod tests {
    use super::KnownCallsState;
    use sdroxide_types::KNOWN_CALLS_REPLY_MAX;

    fn reply(calls: &[&str], total: usize) -> sdroxide_types::KnownCallsReply {
        sdroxide_types::KnownCallsReply {
            calls: calls.iter().map(|s| s.to_string()).collect(),
            total,
        }
    }

    #[test]
    fn an_empty_list_says_which_half_is_missing() {
        let mut s = KnownCallsState::default();
        // Never asked, and asked-but-not-yet-answered, both read the same: the
        // list is genuinely unknown, not empty.
        assert!(s.empty_reason().contains("Not asked"));
        s.request();
        assert!(s.empty_reason().contains("Not asked"));

        // A `None` answer is its own case, and is checked FIRST: this mode
        // keeps no table, which is not the same as a band with nobody on it.
        s.accept(None);
        assert!(s.empty_reason().contains("keeps no callsign table"));
        assert!(s.calls.is_empty());

        // An answer that arrived empty is the real "nobody yet".
        s.accept(Some(reply(&[], 0)));
        assert!(s.empty_reason().contains("Nobody yet"));

        // A real answer clears the no-table case rather than leaving it latched.
        s.accept(Some(reply(&["19DC373"], 1)));
        assert!(!s.no_answer);
        assert_eq!(s.calls, vec!["19DC373".to_string()]);
    }

    #[test]
    fn a_reply_keeps_the_total_the_cap_would_hide() {
        let mut s = KnownCallsState::default();
        s.accept(Some(reply(&["19DC373", "4CB04"], 340)));
        assert_eq!(s.calls.len(), 2);
        // 340 total against 2 carried is what lets the window say "newest 2 of
        // 340" rather than implying those two are everything.
        assert_eq!(s.total, 340);
        assert!(s.total > s.calls.len());
    }

    #[test]
    fn the_cap_is_the_one_the_engine_applies() {
        // The window must not quote a cap of its own; the engine decides, and a
        // second number here could disagree with the list actually sent. This
        // pins the cap to a real value, so a change to it is a deliberate edit
        // rather than something the window silently stops matching.
        assert_eq!(KNOWN_CALLS_REPLY_MAX, 200);
    }
}
