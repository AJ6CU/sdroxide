//! The LISTEN window — the schedule on a map, and a comment on what is on it.
//!
//! The operator's description, which is the spec: *"a listen panel with a map on
//! in the right part and station info in the left part where they can
//! immediately add a comment and hit log."*
//!
//! **`docs/internal/SWL-HANDOVER.md` §12** has the design and what already exists. The short
//! version: the reception log (`swl_log.rs`) already has the comment and the
//! schedule pre-fill, and no window anywhere in the tree is map-driven — so this
//! is a map-first way to fill the same log, not a new log.

// ── the pure half of the LISTEN window ──────────────────────────────────────

/// What the LISTEN map is showing, and what it chose to show.
///
/// Split out from the drawing because this is the part with decisions in it —
/// which stations are worth a dot, and what happens when two of them land on the
/// same few pixels — and decisions are what need tests. The drawing only reads
/// this.
#[derive(Debug, Clone, Default)]
pub(in crate::app) struct ListenSet {
    /// The schedule stations in the span around the dial, as map marks.
    pub stations: Vec<crate::digi_map::DigiStation>,
    /// Previously logged stations, as map marks — **only** when the history
    /// toggle is on.
    pub logged: Vec<(f64, f64, (u8, u8, u8))>,
    /// How many schedule stations were considered and dropped, by reason.
    ///
    /// Shown on the window, because a listener looking at a map that is nearly
    /// empty is entitled to know whether the band is quiet or the picture is
    /// lying to them. An operator who cannot tell those apart stops trusting the
    /// map, which is worse than having no map.
    pub on_band: usize,
    pub dropped_no_position: usize,
    pub dropped_off_band: usize,
    /// Names of the stations nearest the dial, for the side list.
    pub near: Vec<String>,
}

/// Stations worth plotting, and the ones dropped, from a schedule and a dial.
///
/// **The span is the panadapter's**, not the band plan's: a listener with 10 kHz
/// of span wants the stations **in that window**, and a whole band would put a
/// thousand dots on screen for a fraction of them audible. Half a span either
/// side is drawn so a station just off the edge is a scroll away rather than
/// absent.
pub(in crate::app) fn listen_set(
    schedule: &[sdroxide_types::BroadcastStation],
    dial_hz: f64,
    span_hz: f64,
    home: Option<(f64, f64)>,
    history: bool,
    logged: &[sdroxide_types::SwlEntry],
    scope: sdroxide_types::BroadcastScope,
) -> ListenSet {
    use sdroxide_types::BroadcastStation;
    let mut out = ListenSet::default();
    // A panadapter showing nothing has not settled yet; a floor keeps the map
    // useful in the moment after start-up rather than blank.
    //
    // **The window is the operator's choice, not ours** (#80). The span is right
    // for a waterfall label, where a station off the edge of a 10 kHz window is
    // not something to label on it — and wrong for a listener looking at a
    // whole-band map, who is told there are four stations in the world because
    // the dial happens to be at 6190. `WholeBand` is what they asked for.
    let half = match scope {
        sdroxide_types::BroadcastScope::InView => (span_hz / 2.0).max(4_000.0),
        sdroxide_types::BroadcastScope::WholeBand => f64::INFINITY,
    };

    let mut considered = 0usize;
    for s in schedule {
        if (s.freq_hz() - dial_hz).abs() > half {
            out.dropped_off_band += 1;
            continue;
        }
        considered += 1;
        let (Some(lat), Some(lon)) = (s.lat, s.lon) else {
            // No position means no dot. This is the schedule's own gap: a
            // transmitter given as a town and not coordinates, which Aoki's
            // file fills for 99.9% of rows and EiBi's for rather fewer.
            out.dropped_no_position += 1;
            continue;
        };
        // A decode of our own signal sits on our own locator and has no path to
        // draw, so it is not worth a mark either.
        if let Some(h) = home
            && (lat - h.0).abs() < 1e-9
            && (lon - h.1).abs() < 1e-9
        {
            continue;
        }
        out.stations.push(crate::digi_map::DigiStation {
            lat,
            lon,
            fade: 1.0,
            call: Some(std::sync::Arc::from(s.name.as_str())),
            band: None,
        });
    }
    out.on_band = considered;

    // The nearest few by frequency, for a side list the operator can click
    // without hunting on the map.
    let mut near: Vec<&BroadcastStation> = schedule
        .iter()
        .filter(|s| (s.freq_hz() - dial_hz).abs() <= half && s.lat.is_some())
        .collect();
    near.sort_by(|a, b| (a.freq_hz() - dial_hz).abs().total_cmp(&(b.freq_hz() - dial_hz).abs()));
    out.near = near.into_iter().take(12).map(|s| s.name.clone()).collect();

    if history {
        for e in logged {
            let (Some(lat), Some(lon)) = (e.lat, e.lon) else { continue };
            out.logged.push((lat, lon, logged_tint(e.pirate)));
        }
    }
    out
}

/// A logged station's colour: a pirate in the alert red, an ordinary reception in
/// the log's own ink.
///
/// Two populations on one map have to be told apart at a glance, and the tint is
/// the mechanism the map already has — a logged station is passed as a *spot*,
/// which the map draws as a diamond rather than a dot. Shape says "logged", hue
/// says "pirate", and neither has to be read.
fn logged_tint(pirate: bool) -> (u8, u8, u8) {
    if pirate { (232, 64, 64) } else { (96, 176, 224) }
}

/// The station nearest a clicked map position, within `tol_deg`.
///
/// Clicking a map gives **coordinates, not a station** — and on a world map a
/// pixel is a degree or two, so "what did I click" is a nearest-neighbour
/// question with a tolerance. No match within the tolerance is `None` rather than
/// the closest one regardless of distance: clicking open ocean should select
/// nothing, not something 8,000 km away.
pub(in crate::app) fn station_near(
    schedule: &[sdroxide_types::BroadcastStation],
    at: (f64, f64),
    tol_deg: f64,
) -> Option<&sdroxide_types::BroadcastStation> {
    let mut best: Option<(f64, &sdroxide_types::BroadcastStation)> = None;
    for s in schedule {
        let (Some(lat), Some(lon)) = (s.lat, s.lon) else { continue };
        let d = ((lat - at.0).powi(2) + (lon - at.1).powi(2)).sqrt();
        if d <= tol_deg && best.is_none_or(|(bd, _)| d < bd) {
            best = Some((d, s));
        }
    }
    best.map(|(_, s)| s)
}

#[cfg(test)]
mod tests {
    use super::{listen_set, station_near};
    use sdroxide_types::BroadcastScope;
    use sdroxide_types::BroadcastStation;
    use sdroxide_types::SwlEntry;

    /// **The span is not the whole band, when the operator says so** (#80).
    ///
    /// drdatapi's report: broadcast names gone from the waterfall, and the SPOTS
    /// list empty, on 2.0.11 and 2.0.12 — while a click on a signal still found
    /// the stations, so the data was never missing. The window is the
    /// panadapter's own span, which is right for a label on that waterfall and
    /// wrong for a listener looking at a whole-band map.
    ///
    /// So the two are a choice, and this pins both ends of it.
    #[test]
    fn the_broadcast_window_is_the_span_or_the_band() {
        let stations: Vec<BroadcastStation> = [
            ("Local", 6_190.0, Some(52.0), Some(5.0)),
            ("Far away", 6_950.0, Some(-33.0), Some(151.0)),
            ("Further", 7_200.0, Some(40.0), Some(-74.0)),
        ]
        .iter()
        .map(|&(n, k, la, lo)| station(n, k, la, lo))
        .collect();

        // 20 kHz of span at 6190 kHz reaches none of the others.
        let inview =
            listen_set(&stations, 6_190_000.0, 20_000.0, None, false, &[], BroadcastScope::InView);
        assert_eq!(inview.near.len(), 1, "the span should hold only what it holds");
        assert_eq!(inview.on_band, 1);

        // The whole band is all of them, which is what a listener asked for.
        let whole = listen_set(
            &stations,
            6_190_000.0,
            20_000.0,
            None,
            false,
            &[],
            BroadcastScope::WholeBand,
        );
        assert_eq!(whole.near.len(), 3, "the whole band should hold every station");
        assert_eq!(whole.on_band, 3);
    }

    fn station(name: &str, khz: f64, lat: Option<f64>, lon: Option<f64>) -> BroadcastStation {
        BroadcastStation {
            name: name.into(),
            freq_khz: khz,
            site: String::new(),
            country: String::new(),
            lat,
            lon,
            ..Default::default()
        }
    }

    /// The span is the **panadapter's**, so a 10 kHz view with 40 stations in
    /// the band puts on screen the few inside the window — not all forty.
    #[test]
    fn only_the_span_around_the_dial_is_plotted() {
        let s = vec![
            station("in span", 6_190.0, Some(50.0), Some(5.0)),
            station("just outside", 6_196.0, Some(50.0), Some(6.0)),
            station("far away", 7_200.0, Some(50.0), Some(7.0)),
        ];
        let set = listen_set(&s, 6_190_000.0, 10_000.0, None, false, &[], BroadcastScope::InView);
        let names: Vec<&str> = set.stations.iter().filter_map(|d| d.call.as_deref()).collect();
        assert_eq!(names, vec!["in span"], "only the span: {names:?}");
        assert_eq!(set.dropped_off_band, 2);
        assert_eq!(set.on_band, 1);
    }

    /// A station with no coordinates cannot have a dot, and the count of them
    /// must be **reported** — a map that is empty because the schedule had no
    /// positions looks identical to a map that is empty because the band is
    /// quiet, and the operator cannot tell those apart without being told.
    #[test]
    fn a_station_without_a_position_is_dropped_and_counted() {
        let s = vec![
            station("placed", 6_190.0, Some(50.0), Some(5.0)),
            station("no position", 6_191.0, None, None),
            station("half placed", 6_192.0, Some(50.0), None),
        ];
        let set = listen_set(&s, 6_190_000.0, 20_000.0, None, false, &[], BroadcastScope::InView);
        assert_eq!(set.stations.len(), 1, "only the placed one is drawn");
        assert_eq!(
            set.dropped_no_position, 2,
            "a half-placed station is as unplottable as an unplaced one"
        );
        assert_eq!(set.on_band, 3, "all three were on the band");
    }

    /// **History is a toggle, off by default** — the operator's reason being
    /// that an experienced listening station does not want its history
    /// cluttering the band. This is that decision, pinned.
    #[test]
    fn the_history_is_off_unless_it_is_asked_for() {
        let logged = SwlEntry {
            station: "old catch".into(),
            lat: Some(48.0),
            lon: Some(11.0),
            ..Default::default()
        };
        let s = vec![station("on band", 6_190.0, Some(50.0), Some(5.0))];

        let off = listen_set(
            &s,
            6_190_000.0,
            20_000.0,
            None,
            false,
            &[logged.clone()],
            BroadcastScope::InView,
        );
        assert!(off.logged.is_empty(), "history off must draw nothing of it");

        let on =
            listen_set(&s, 6_190_000.0, 20_000.0, None, true, &[logged], BroadcastScope::InView);
        assert_eq!(on.logged.len(), 1, "history on draws what was logged");
        assert_eq!(on.stations.len(), 1, "and does not disturb what is on now");
    }

    /// A logged station with no coordinates is skipped rather than drawn at
    /// 0,0 — which is the Gulf of Guinea, and reads as a real reception there.
    #[test]
    fn a_logged_station_without_coordinates_is_not_drawn_at_zero() {
        let logged = SwlEntry { station: "typed by hand".into(), ..Default::default() };
        let set =
            listen_set(&[], 6_190_000.0, 20_000.0, None, true, &[logged], BroadcastScope::InView);
        assert!(set.logged.is_empty(), "absent coordinates must not become 0,0");
    }

    /// A pirate reads differently from an ordinary reception, because "is this
    /// one a pirate" is the question a listener is asking when they open their
    /// own history.
    #[test]
    fn a_pirate_is_tinted_apart_from_an_ordinary_reception() {
        let mut pirate = SwlEntry {
            station: "pirate".into(),
            lat: Some(48.0),
            lon: Some(11.0),
            ..Default::default()
        };
        pirate.pirate = true;
        let ordinary = SwlEntry {
            station: "legal".into(),
            lat: Some(49.0),
            lon: Some(12.0),
            ..Default::default()
        };
        let set = listen_set(
            &[],
            6_190_000.0,
            20_000.0,
            None,
            true,
            &[pirate, ordinary],
            BroadcastScope::InView,
        );
        assert_eq!(set.logged.len(), 2);
        assert_ne!(
            set.logged[0].2, set.logged[1].2,
            "a pirate and a legal station must not wear the same colour"
        );
    }

    /// Clicking the map gives coordinates, and a pixel on a world map is a
    /// degree or two — so this is nearest-neighbour **within a tolerance**.
    #[test]
    fn a_click_picks_the_station_under_it() {
        let s = vec![
            station("near the click", 6_190.0, Some(50.0), Some(5.0)),
            station("elsewhere", 6_190.0, Some(-33.0), Some(151.0)),
        ];
        let hit = station_near(&s, (50.05, 5.05), 2.0).expect("a click 0.07° away must hit");
        assert_eq!(hit.name, "near the click");
        // And the far one is never preferred just because it is in the list.
        let other = station_near(&s, (-33.05, 151.05), 2.0).expect("the other must hit too");
        assert_eq!(other.name, "elsewhere");
    }

    /// **A station's frequency is what a click resolves to**, so the map can
    /// name the station *and* the number the dial has to go to.
    ///
    /// The window's first version selected a station and left the dial alone,
    /// which is the one behaviour every other map here does not have — the
    /// SCHEDULE window's rows tune, the decodes map points the beam. The
    /// operator's report was the whole diagnosis: *"clicking a station is not
    /// changing the frequency"*. This pins the half that is pure arithmetic,
    /// which is the half that can be wrong without anyone noticing: a click
    /// yields **coordinates**, so the frequency has to come back by name, and a
    /// wrong lookup would tune somewhere plausible.
    #[test]
    fn a_clicked_station_carries_the_frequency_to_tune_to() {
        use sdroxide_types::ScheduleSource as S;
        let s = vec![
            station("Voice of Nigeria", 6_350.0, Some(9.08), Some(7.26)),
            station("Radio Romania", 6_040.0, Some(45.28), Some(28.20)),
        ];
        // Clicking near the first station picks it, and its frequency is what a
        // click on it means.
        let hit = station_near(&s, (9.1, 7.3), 6.0).expect("a click 0.06 deg away must hit");
        assert_eq!(hit.name, "Voice of Nigeria");
        assert_eq!(
            hit.freq_hz(),
            6_350_000.0,
            "the dial has to be sent the station's own frequency, in hertz"
        );
        // The same for the other, so it is not an off-by-one on the list order.
        let other = station_near(&s, (45.3, 28.2), 6.0).expect("the other must hit too");
        assert_eq!(other.freq_hz(), 6_040_000.0);
        // And a schedule row carries the **mode** it is meant to be heard on, so
        // a click can put the radio in the right mode as well as the right place
        // — which is the difference between tuning *to* a station and merely
        // landing on its frequency. Absent on this row, so the fallback is AM.
        assert_eq!(other.mode(), sdroxide_types::Mode::Am);
        let _ = S::Eibi;
    }

    /// **Clicking open ocean selects nothing.** Without the tolerance, "the
    /// closest station" would happily return one 8,000 km away, and the side
    /// panel would fill in with a station the operator did not click.
    #[test]
    fn a_click_in_open_water_selects_nothing() {
        let s = vec![station("Berlin", 6_190.0, Some(52.5), Some(13.4))];
        // Mid-Atlantic.
        assert!(
            station_near(&s, (0.0, -30.0), 2.0).is_none(),
            "nothing is near 0,-30 and something was selected anyway"
        );
        // And a station with no position is never "near" anything.
        let s2 = vec![station("unplaced", 6_190.0, None, None)];
        assert!(station_near(&s2, (0.0, 0.0), 180.0).is_none());
    }

    /// Two stations on the same few pixels: the **closest** wins, deterministically.
    #[test]
    fn the_closest_of_two_nearby_stations_wins() {
        let s = vec![
            station("further", 6_190.0, Some(50.0), Some(5.0)),
            station("closer", 6_190.0, Some(50.05), Some(5.05)),
        ];
        let hit = station_near(&s, (50.06, 5.06), 2.0).unwrap();
        assert_eq!(hit.name, "closer");
        // Order of the schedule must not change the answer.
        let mut reversed = s.clone();
        reversed.reverse();
        assert_eq!(station_near(&reversed, (50.06, 5.06), 2.0).unwrap().name, "closer");
    }

    /// Our own locator gets no dot and no path — a reception of a local
    /// transmitter is a real thing to log, but as a map mark it sits on top of
    /// the home marker and hides it.
    #[test]
    fn a_station_on_our_own_locator_is_not_plotted_over_home() {
        let home = (52.0, 5.0);
        let s = vec![
            station("our own local tx", 6_190.0, Some(52.0), Some(5.0)),
            station("a real one", 6_191.0, Some(50.0), Some(6.0)),
        ];
        let set =
            listen_set(&s, 6_190_000.0, 20_000.0, Some(home), false, &[], BroadcastScope::InView);
        assert_eq!(set.stations.len(), 1, "the local one must not cover home");
    }
}

// ── the window ──────────────────────────────────────────────────────────────

/// A frequency the way the log writes one: kHz first, MHz beside it, because a
/// listener's dial is in kHz and a schedule's row is too, while the reception
/// report wants MHz.
fn khz_text(khz: f64) -> String {
    format!("{khz:.0} kHz ({:.3} MHz)", khz / 1e3)
}

use eframe::egui::{self, RichText};
use sdroxide_types::Command;

use crate::app::SdroxideApp;
use crate::app::swl_log::SwlEditForm;
use crate::time::now_unix;

/// The window's app-id. A tool window, so it gets the `⇱` chip and a remembered
/// geometry like every other one.
pub(in crate::app) const LISTEN_APP_ID: &str = "swl-listen";

impl SdroxideApp {
    /// Open LISTEN. A **window**, not a docked panel — `docs/internal/SWL-HANDOVER.md` §12 for
    /// why: the docked column caps near 280 pt and a map beside station info
    /// wants about 700, so docked it squeezes whatever is below it.
    pub(in crate::app) fn listen_window(&mut self, ctx: &egui::Context, cmds: &mut Vec<Command>) {
        let open = self.tool_window(
            ctx,
            LISTEN_APP_ID,
            "LISTEN",
            [940.0, 620.0],
            self.show_listen,
            |me, ui| {
                me.listen_body(ui, cmds);
            },
        );
        self.show_listen = open;
    }

    /// Station info and comment on the left, the map on the right.
    fn listen_body(&mut self, ui: &mut egui::Ui, cmds: &mut Vec<Command>) {
        let dial = self.state.rx_freq_hz();
        let span = self.view.span();
        let home = sdroxide_types::grid_to_latlon(&self.my_grid());

        // ── the header: what is on, and the history toggle ──
        ui.horizontal(|ui| {
            // No second "LISTEN" here — the window's own title bar already says
            // it, two lines up. What this line has to add is the thing that is
            // not otherwise visible: **what span the map is showing**, which is
            // what makes the station list below it make sense.
            ui.label(
                RichText::new(format!("What is on around {}", khz_text(dial / 1e3)))
                    .size(11.5)
                    .color(crate::theme::gray(150)),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // **The history toggle.** Off by default: an experienced
                // listening station does not want its own history cluttering
                // the band, and on it is what makes the window worth a
                // screenshot.
                let hist = crate::chrome::chip(ui, self.listen_history, "HISTORY");
                if hist
                    .on_hover_text(
                        "Show every station you have logged as well as what is on now. \
                     Off, the map is only the band; on, it is also your own history",
                    )
                    .clicked()
                {
                    self.listen_history = !self.listen_history;
                }
            });
        });
        ui.add_space(4.0);

        let set = listen_set(
            &self.broadcast,
            dial,
            span,
            home,
            self.listen_history,
            &self.swl_log,
            self.ui_settings.broadcast_scope,
        );
        // Say plainly what the map is *not* showing, or an empty map is
        // indistinguishable from a quiet band.
        if set.stations.is_empty() {
            let why = if set.on_band == 0 {
                "nothing scheduled inside this span".to_string()
            } else if set.dropped_no_position == set.on_band {
                format!("{} scheduled here, none with a position on the map", set.on_band)
            } else {
                "nothing to plot here".to_string()
            };
            ui.colored_label(crate::theme::gray(150), why);
        }

        let avail = ui.available_size();
        // Roughly 380 of the window for the column and the rest for the map —
        // the map is the point, so it takes whatever is left and the column
        // takes what it needs.
        let left_w = (avail.x * 0.38).clamp(300.0, 400.0);
        ui.horizontal_top(|ui| {
            let mut log_now = false;
            ui.vertical(|ui| {
                ui.set_width(left_w);
                self.listen_side(ui, &set, home, &mut log_now, cmds);
            });
            ui.vertical(|ui| {
                self.listen_map(ui, &set, home, avail.y, cmds);
            });
            if log_now {
                self.listen_log_now(cmds);
            }
        });
    }

    /// The left column: what is selected, a comment box, and LOG.
    fn listen_side(
        &mut self,
        ui: &mut egui::Ui,
        set: &ListenSet,
        home: Option<(f64, f64)>,
        log_now: &mut bool,
        cmds: &mut Vec<Command>,
    ) {
        // The nearest stations by frequency, clickable — selecting on a world
        // map means finding a dot, and on a busy evening the list is faster.
        ui.label(RichText::new("ON THIS SPAN").size(10.0).color(crate::theme::gray(150)));
        egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
            if set.near.is_empty() {
                ui.colored_label(crate::theme::gray(150), "none scheduled here");
            }
            for name in &set.near {
                let picked = self.listen_sel.as_ref().is_some_and(|s| &s.name == name);
                if ui
                    .selectable_label(picked, name)
                    .on_hover_text("Pick this station and tune to it")
                    .clicked()
                {
                    let hz = self.broadcast.iter().find(|s| &s.name == name).map(|s| s.freq_hz());
                    self.listen_select(name);
                    if let Some(hz) = hz {
                        cmds.push(Command::SetVfo { vfo: self.state.active_vfo, hz });
                    }
                }
            }
        });
        ui.add_space(6.0);
        ui.separator();
        ui.add_space(4.0);

        // Half the span, the same measure the map's own filter uses, so "on
        // this span" means one thing throughout the window.
        let span = (self.view.span() / 2.0).max(4_000.0);
        let Some(s) = self.listen_sel.clone() else {
            ui.colored_label(
                crate::theme::gray(150),
                "Pick a station on the map or from the list, then write what you heard.",
            );
            return;
        };

        // The station itself.
        ui.label(RichText::new(&s.name).size(13.0).strong().color(crate::theme::CYAN()));
        let mut line = khz_text(s.freq_khz);
        if !s.country.is_empty() {
            line.push_str(&format!("  ·  {}", s.country));
        }
        if !s.lang.is_empty() {
            line.push_str(&format!("  ·  {}", s.lang));
        }
        ui.label(RichText::new(line).size(11.0).color(crate::theme::gray(150)));
        if !s.site.is_empty() {
            ui.label(RichText::new(&s.site).size(11.0).color(crate::theme::gray(150)));
        }
        // **Say when the selected station is not what the dial is on.** Clicking a
        // station tunes to it, so this only happens if the operator retuned
        // afterwards — and with no word for it the window reads as broken: the
        // header says one frequency, the list says another, and the station in
        // the middle belongs to neither. A beginner cannot tell that from a
        // fault, and a house rule says never leave one guessing.
        if (s.freq_hz() - self.state.rx_freq_hz()).abs() > span {
            ui.label(
                RichText::new("Not on this dial — click it on the map to tune there")
                    .size(10.5)
                    .color(crate::theme::YELLOW()),
            );
        }

        // Geometry and the dark share — the number this window exists to make
        // useful. Grey-line absorption is why a long path is open or not, and
        // this is it as a figure rather than something read off the terminator.
        if let (Some(h), Some(lat), Some(lon)) = (home, s.lat, s.lon) {
            let km = sdroxide_types::distance_km(h, (lat, lon));
            let brg = sdroxide_types::bearing_deg(h, (lat, lon));
            let share = sdroxide_solar::ephem::dark_share(h, (lat, lon), now_unix(), 9);
            ui.label(
                RichText::new(format!(
                    "{:.0} km  ·  {:03.0}°  ·  {:.0}% of this path in night",
                    km,
                    brg,
                    share * 100.0
                ))
                .size(11.0)
                .color(crate::theme::gray(150)),
            );
        } else if s.lat.is_none() {
            ui.label(
                RichText::new("no position in the schedule, so no path to measure")
                    .size(11.0)
                    .color(crate::theme::gray(150)),
            );
        }

        ui.add_space(6.0);
        ui.label(RichText::new("What you heard").size(10.0).color(crate::theme::gray(150)));
        crate::chrome::field(
            ui,
            egui::TextEdit::multiline(&mut self.listen_comment)
                .desired_rows(3)
                .hint_text("Interval signal, programme notes, the language…"),
        );

        ui.add_space(4.0);
        if ui
            .add_sized(
                [ui.available_width(), 26.0],
                egui::Button::new(RichText::new("LOG").size(12.0).strong()),
            )
            .on_hover_text("Open the reception log with this station and your note ready to save")
            .clicked()
        {
            *log_now = true;
        }
    }

    /// The map. `worldmap::show` is untouched — the two station kinds already
    /// have a slot in it, and it already hands back the clicked position.
    fn listen_map(
        &mut self,
        ui: &mut egui::Ui,
        set: &ListenSet,
        home: Option<(f64, f64)>,
        max_h: f32,
        cmds: &mut Vec<Command>,
    ) {
        let heat = self.prop_texture(ui.ctx(), self.state.rx_freq_hz());
        let night = self.night_texture(ui.ctx());
        let reporters: Vec<sdroxide_types::Spot> = Vec::new();
        if let Some((lat, lon)) = crate::widgets::worldmap::show(
            ui,
            &mut self.listen_map_view,
            home,
            None,
            None,
            None,
            &set.stations,
            &set.logged,
            &reporters,
            heat,
            night,
            false,
            max_h,
            false,
            true,
        ) {
            // A click is coordinates, so this picks the station under it — or
            // nothing, if the click was open ocean.
            // A click is coordinates, so this picks the station under it — or
            // nothing, if the click was open ocean.
            //
            // **And it tunes**, which is what a click on a map means everywhere
            // else in this program: the SCHEDULE window's rows tune, and the
            // decodes map steers the beam at the spot it was clicked. This one
            // selected only, and the operator found it in a minute — *"the map
            // is not clickable, clicking a station is not changing the
            // frequency"*. That is the house rule about a control that silently
            // does less than it looks like it does, caught by the only person who
            // can see the whole picture; the hover says what it will do, and now
            // the dial moves.
            match station_near(&self.broadcast, (lat, lon), 6.0) {
                Some(s) => {
                    let (name, hz) = (s.name.clone(), s.freq_hz());
                    self.listen_select(&name);
                    cmds.push(Command::SetVfo { vfo: self.state.active_vfo, hz });
                }
                None => {
                    self.listen_sel = None;
                    self.listen_comment.clear();
                }
            }
        }
    }

    /// Select by name, re-finding the station so a reload of the schedule does
    /// not leave a stale copy behind.
    pub(in crate::app) fn listen_select(&mut self, name: &str) {
        let found = self.broadcast.iter().find(|s| &s.name == name).cloned();
        if found.as_ref().is_some_and(|s| &s.name != name) {
            return;
        }
        if self.listen_sel.as_ref().is_none_or(|s| &s.name != name) {
            // Switching station clears the note: carrying one station's comment
            // onto another is how a log acquires a lie.
            self.listen_comment.clear();
        }
        self.listen_sel = found;
    }

    /// Hand the selection to the reception log's own entry form, with the note
    /// already in it.
    ///
    /// **It opens the form rather than saving**, and that is the point: SINPO, a
    /// reception report, a pirate flag and a QSL letter all live there, and a
    /// second way to save an entry is a second way to get one of those wrong.
    fn listen_log_now(&mut self, cmds: &mut Vec<Command>) {
        let Some(s) = self.listen_sel.clone() else { return };
        let smeter = self.meters.map(|m| m.s_dbm);
        let grid = self.my_grid();
        let antenna = self.swl_antenna.clone();
        let mut form = SwlEditForm::from_station(
            s.freq_hz(),
            self.state.rx[0].mode,
            &s.name,
            &s.lang,
            &s.site,
            &s.email,
            &s.address,
            s.lat,
            s.lon,
            smeter,
            grid,
            antenna,
        );
        form.set_notes(&self.listen_comment);
        // The reception log's own form, which already knows how to save an
        // entry, print a report and carry the QSL loop.
        self.swl_edit = Some(form);
        self.show_swl = true;
        self.listen_comment.clear();
        let _ = cmds;
    }
}

#[cfg(test)]
mod window_tests {
    use super::khz_text;

    /// The frequency is written the way the reception log writes one, because
    /// LISTEN fills that form: a listener's dial and a schedule's row are in kHz
    /// and the report wants MHz, so both appear.
    #[test]
    fn a_frequency_reads_the_way_the_log_writes_it() {
        assert_eq!(khz_text(6185.0), "6185 kHz (6.185 MHz)");
        assert_eq!(khz_text(3955.0), "3955 kHz (3.955 MHz)");
        // And a whole number does not pick up a decimal it does not need.
        assert_eq!(khz_text(6000.0), "6000 kHz (6.000 MHz)");
    }
}
