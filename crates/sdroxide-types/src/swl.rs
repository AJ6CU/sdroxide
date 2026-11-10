//! The shortwave listener's log: what was **heard**, not what was worked.
//!
//! Deliberately not the QSO log. A reception has no callsign, no RST and no
//! grid to exchange — it has a *station*, a frequency, a time, and a listener's
//! judgement of how well it came through. Folding those into [`crate::QsoRecord`]
//! would give both records fields that are always empty for one of them.
//!
//! The judgement is a **SINPO** report — Strength, Interference, Noise,
//! Propagation, Overall, each 1–5 — or its older three-figure **SIO** form
//! (Strength, Interference, Overall). Both are in use and this fork keeps
//! whichever the listener wrote.

use serde::{Deserialize, Serialize};

/// A five-figure SINPO report. Each figure is 1 (worst) to 5 (best).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sinpo {
    /// Signal strength.
    pub s: u8,
    /// Interference from other stations.
    pub i: u8,
    /// Noise (atmospheric, man-made).
    pub n: u8,
    /// Propagation: fading and distortion.
    pub p: u8,
    /// Overall merit.
    pub o: u8,
}

impl Default for Sinpo {
    fn default() -> Self {
        // A neutral middle for every figure, so a half-filled report reads as
        // "average" rather than "worst".
        Sinpo { s: 3, i: 3, n: 3, p: 3, o: 3 }
    }
}

/// A three-figure SIO report. The older form, still common.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct Sio {
    pub s: u8,
    pub i: u8,
    pub o: u8,
}

impl Default for Sio {
    fn default() -> Self {
        Sio { s: 3, i: 3, o: 3 }
    }
}

/// How a reception was scored: SINPO or SIO, whichever the listener used.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SignalReport {
    Sinpo(Sinpo),
    Sio(Sio),
}

impl SignalReport {
    /// The tag a listener writes ahead of the figures.
    pub fn label(self) -> &'static str {
        match self {
            SignalReport::Sinpo(_) => "SINPO",
            SignalReport::Sio(_) => "SIO",
        }
    }

    /// The figures as a listener spaces them: `"4 3 3 4 4"`.
    pub fn digits(self) -> String {
        match self {
            SignalReport::Sinpo(r) => format!("{} {} {} {} {}", r.s, r.i, r.n, r.p, r.o),
            SignalReport::Sio(r) => format!("{} {} {}", r.s, r.i, r.o),
        }
    }
}

/// One reception in the listener's log.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SwlEntry {
    /// Stable log id (0 = unassigned; the UI assigns on first store).
    pub id: u64,
    /// When it was heard, as Unix seconds UTC. The log sorts and groups by it.
    pub heard_at_unix: u64,
    /// The station, free text or as the schedule names it.
    pub station: String,
    /// The frequency heard, in Hz (dial + any offset).
    pub freq_hz: f64,
    pub mode: crate::Mode,
    /// Programme language, free text ("English", "Dutch").
    pub language: String,
    /// SINPO or SIO, as given. `None` = not judged.
    pub report: Option<SignalReport>,
    /// S-meter reading in dBm when it was heard, if the radio reports one.
    pub smeter_dbm: Option<f32>,
    /// Transmitter site, when the schedule supplied it.
    pub site: String,
    /// The receiving station's Maidenhead locator when it was heard — *where
    /// the reception was made*, which is not the transmitter's [`Self::site`].
    /// A listener who moves the aerial (or the receiver) wants to know which
    /// spot heard it, and a propagation comparison between two evenings needs
    /// it. Pre-filled from the screen's own grid, editable per entry.
    pub recv_grid: String,
    /// The antenna in use when it was heard, in the listener's own words,
    /// captured from the SWL LOG window's session field at the moment of
    /// logging. It goes on the reception report's **Antenna:** line, and it is
    /// kept per entry so a report of an older reception names the aerial that
    /// actually heard it.
    pub antenna: String,
    /// Programme notes — what was on, what was said.
    pub notes: String,
    /// Where the **transmitter** stands, when the entry was logged off the
    /// schedule and the schedule knew it.
    ///
    /// The entry already carried [`Self::site`] as a *string* ("Solec Kujawski"),
    /// which is no use to a map: this is the same fact as coordinates. Kept
    /// beside it rather than instead of it, because the site's name is what goes
    /// on the reception report and what the operator recognises.
    ///
    /// `None` for an entry typed by hand, and for an old log file written before
    /// these fields existed — which is why both are `Option` and `serde(default)`
    /// and why the file loads unchanged. **No `PROTO_VERSION` bump:** `SwlEntry`
    /// is local `swl.json` and appears in no wire message.
    #[serde(default)]
    pub lat: Option<f64>,
    /// Longitude of the transmitter, paired with [`Self::lat`]. West negative.
    #[serde(default)]
    pub lon: Option<f64>,
    /// The broadcaster's reception-report email, copied from the schedule when
    /// the entry was logged from it. Free text; empty when the schedule carried
    /// no contact. It goes on the report's **Send to:** block, and the log
    /// offers it to copy.
    pub email: String,
    /// The broadcaster's postal address for a reception report, from the
    /// schedule when it carried one. The fallback destination when there is no
    /// email.
    pub address: String,
    /// When a reception report was sent for this hearing, Unix seconds UTC, or
    /// `None` while it has not been. The `REPORT` button stamps it, and the log
    /// shows a **sent** mark — the second step of the SWL's loop, *hear →
    /// report → await QSL*, which the log otherwise forgot the moment the file
    /// was saved.
    pub report_sent_unix: Option<u64>,
    /// When a QSL (or any verification) came back for it, Unix seconds UTC, or
    /// `None` while it is still awaited — the last step of the loop. The log
    /// shows a **QSL** mark for the ones that completed.
    pub qsl_received_unix: Option<u64>,
    /// The listener marked this as an **unlicensed ("pirate") broadcast** — a
    /// station transmitting outside any allocation, which is a thing shortwave
    /// listeners deliberately hunt. Its own flag rather than a word in the
    /// notes, so the log can show it and a future filter can use it. Off by
    /// default, and nothing infers it: it is the listener's judgement.
    pub pirate: bool,
}

impl Default for SwlEntry {
    fn default() -> Self {
        SwlEntry {
            id: 0,
            heard_at_unix: 0,
            station: String::new(),
            freq_hz: 0.0,
            mode: crate::Mode::Am,
            language: String::new(),
            report: None,
            smeter_dbm: None,
            site: String::new(),
            recv_grid: String::new(),
            antenna: String::new(),
            notes: String::new(),
            lat: None,
            lon: None,
            email: String::new(),
            address: String::new(),
            report_sent_unix: None,
            qsl_received_unix: None,
            pirate: false,
        }
    }
}

impl SwlEntry {
    /// The frequency as a listener says it: `6185 kHz (6.185 MHz)`.
    pub fn frequency_text(&self) -> String {
        format!("{:.0} kHz ({:.3} MHz)", self.freq_hz / 1e3, self.freq_hz / 1e6)
    }

    /// UTC as the log shows it, e.g. `2026-09-16 19:42 UTC`.
    pub fn utc_text(&self) -> String {
        let (y, mo, d, h, mi, _s) = crate::utc_ymd_hms(self.heard_at_unix as i64);
        format!("{y:04}-{mo:02}-{d:02} {h:02}:{mi:02} UTC")
    }

    /// A reception report, ready to paste into an email or a station's web
    /// form. `listener`, `grid`, `receiver` and `antenna` describe the
    /// *listening* station: `listener` comes from the screen and `receiver` is
    /// the program's own name, while `grid` and `antenna` are usually this
    /// entry's own [`Self::recv_grid`] and [`Self::antenna`], captured when it
    /// was logged — pass them from the entry so a report names the place and
    /// the aerial that actually made the reception. `listener` is the
    /// listener's own identity — an SWL number, a club number, a name — kept
    /// apart from the transmitting callsign so that reporting a broadcast never
    /// keys a CB transmitter with it.
    ///
    /// Empty lines are left out rather than shown blank, and an unjudged
    /// reception says so instead of printing a row of zeroes.
    pub fn report_text(&self, listener: &str, grid: &str, receiver: &str, antenna: &str) -> String {
        let mut out = String::from("Reception report\n\n");
        out.push_str(&format!("Station:    {}\n", self.station.trim()));
        out.push_str(&format!("Frequency:  {}, {}\n", self.frequency_text(), self.mode.label()));
        out.push_str(&format!("Heard:      {}\n", self.utc_text()));
        let line = |out: &mut String, name: &str, value: &str| {
            if !value.trim().is_empty() {
                out.push_str(&format!("{name:<12}{}\n", value.trim()));
            }
        };
        line(&mut out, "Location:", grid);
        line(&mut out, "Receiver:", receiver);
        line(&mut out, "Antenna:", antenna);
        match self.report {
            Some(r) => out.push_str(&format!("{}:  {}\n", r.label(), r.digits())),
            None => out.push_str("Signal report: not judged\n"),
        }
        line(&mut out, "Notes:", &self.notes);
        line(&mut out, "Reported by:", listener);
        // Where to send it, when the schedule carried a contact: the point of
        // the report is a QSL card, and hunting for the address afterwards is
        // the step that gets skipped.
        if !self.email.trim().is_empty() || !self.address.trim().is_empty() {
            out.push_str("\nSend to:\n");
            line(&mut out, "Email:", &self.email);
            line(&mut out, "Address:", &self.address);
        }
        out
    }
}

/// A Unix instant as `YYYY-MM-DD HH:MM:SS`, UTC — the shape both exports print.
fn export_utc(unix: u64) -> String {
    let (y, mo, d, h, mi, s) = crate::utc_ymd_hms(unix as i64);
    format!("{y:04}-{mo:02}-{d:02} {h:02}:{mi:02}:{s:02}")
}

/// The whole reception log as CSV: one row per hearing, for a spreadsheet or a
/// quick look in a text editor.
///
/// This is the log's own export, where [`SwlEntry::report_text`] (the `REPORT`
/// button) writes a *single* entry to send to a broadcaster. The signal report
/// is split into its kind (`SINPO` / `SIO`) and its figures so a spreadsheet can
/// sort on either, and both reporting dates are carried out so the loop's state
/// travels with the log.
pub fn swl_log_to_csv(entries: &[SwlEntry]) -> String {
    let mut out = String::from(
        "utc,station,freq_khz,freq_mhz,band,mode,language,site,report_kind,report,\
         smeter_dbm,recv_grid,antenna,report_sent,qsl_received,pirate,notes\r\n",
    );
    let csv = crate::digi::csv_field;
    for e in entries {
        out.push_str(&format!(
            "{},{},{:.3},{:.6},{},{},{},{},{},{},{},{},{},{},{},{},{}\r\n",
            export_utc(e.heard_at_unix),
            csv(&e.station),
            e.freq_hz / 1e3,
            e.freq_hz / 1e6,
            crate::Band::containing(e.freq_hz).label(),
            e.mode.label(),
            csv(&e.language),
            csv(&e.site),
            e.report.map(|r| r.label()).unwrap_or(""),
            csv(&e.report.map(|r| r.digits()).unwrap_or_default()),
            e.smeter_dbm.map(|d| format!("{d:.1}")).unwrap_or_default(),
            csv(&e.recv_grid),
            csv(&e.antenna),
            e.report_sent_unix.map(export_utc).unwrap_or_default(),
            e.qsl_received_unix.map(export_utc).unwrap_or_default(),
            if e.pirate { "yes" } else { "" },
            csv(&e.notes),
        ));
    }
    out
}

/// One reception as a bare ADIF record ending in `<EOR>` — or `None` for a
/// hearing that names no station, which a logger cannot keep.
///
/// A reception, not a contact: nothing was worked, so the record says `SWL=Y`
/// and hangs the two reporting dates on the fields ADIF defines for them —
/// `QSL_SENT`/`QSLSDATE` for the report sent, `QSL_RCVD`/`QSLRDATE` for the
/// verification back. `CALL` carries the station name: a broadcast has no
/// callsign and ADIF has no field for a broadcaster, so the name is what a
/// logger needs to keep the record rather than drop it, and the language and
/// transmitter site ride in `APP_` fields ADIF reserves for a program's own
/// data. The receiving locator and aerial are ADIF's own `MY_` pair.
pub fn swl_entry_to_adif_record(e: &SwlEntry) -> Option<String> {
    let station = e.station.trim();
    if station.is_empty() {
        return None;
    }
    let field = crate::digi::adif_field;
    let (date, time) = crate::digi::adif_date_time(e.heard_at_unix as i64);
    let mut out = String::new();
    out.push_str(&field("CALL", station));
    out.push_str(&field("SWL", "Y"));
    out.push_str(&field("QSO_DATE", &date));
    out.push_str(&field("TIME_ON", &time));
    out.push_str(&field("BAND", crate::digi::adif_band(e.freq_hz)));
    out.push_str(&field("MODE", e.mode.label()));
    out.push_str(&field("FREQ", &format!("{:.6}", e.freq_hz / 1e6)));
    if !e.recv_grid.is_empty() {
        out.push_str(&field("MY_GRIDSQUARE", &e.recv_grid));
    }
    if !e.antenna.is_empty() {
        out.push_str(&field("MY_ANTENNA", &e.antenna));
    }
    if let Some(r) = e.report {
        out.push_str(&field("APP_SDROXIDE_REPORT", &format!("{} {}", r.label(), r.digits())));
    }
    if let Some(d) = e.smeter_dbm {
        out.push_str(&field("APP_SDROXIDE_SMETER_DBM", &format!("{d:.1}")));
    }
    if !e.site.is_empty() {
        out.push_str(&field("APP_SDROXIDE_SITE", &e.site));
    }
    if !e.language.is_empty() {
        out.push_str(&field("APP_SDROXIDE_LANGUAGE", &e.language));
    }
    if e.pirate {
        out.push_str(&field("APP_SDROXIDE_PIRATE", "Y"));
    }
    if let Some(t) = e.report_sent_unix {
        out.push_str(&field("QSL_SENT", "Y"));
        out.push_str(&field("QSLSDATE", &crate::digi::adif_date_time(t as i64).0));
    }
    if let Some(t) = e.qsl_received_unix {
        out.push_str(&field("QSL_RCVD", "Y"));
        out.push_str(&field("QSLRDATE", &crate::digi::adif_date_time(t as i64).0));
    }
    if !e.notes.trim().is_empty() {
        out.push_str(&field("NOTES", e.notes.trim()));
    }
    out.push_str("<EOR>");
    Some(out)
}

/// The whole reception log as an ADIF file, one record per entry (see
/// [`swl_entry_to_adif_record`]).
pub fn swl_log_to_adif(entries: &[SwlEntry]) -> String {
    let mut out = String::from(
        "ADIF export from sdroxide — reception log (SWL)\r\n\
         <ADIF_VER:5>3.1.4\r\n<PROGRAMID:8>sdroxide\r\n<EOH>\r\n",
    );
    for record in entries.iter().filter_map(swl_entry_to_adif_record) {
        out.push_str(&record);
        out.push_str("\r\n");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// **An existing log file must load unchanged.** Every listener's `swl.json`
    /// was written before these fields existed, and a load that cannot parse
    /// quarantines the file — so if these ever stopped being **optional**, the
    /// first thing an operator would notice is their whole reception history
    /// gone, with the cause in a `.quarantined` file.
    ///
    /// Worth being precise about *what* makes that safe, because the obvious
    /// guess is wrong: not the `#[serde(default)]`, which turned out to be
    /// redundant — **serde already reads a missing `Option` field as `None`**,
    /// so dropping the attribute changed nothing and this test still passed. It
    /// is the `Option` itself. Changing `lat` to a bare `f64` fails this test,
    /// and that is the mutation it is here to catch.
    #[test]
    fn a_log_written_before_coordinates_still_loads() {
        let old = r#"{
            "id": 3,
            "heard_at_unix": 1789587720,
            "station": "Radio Prague",
            "freq_hz": 9550000.0,
            "mode": "Am",
            "language": "English",
            "site": "Prague",
            "recv_grid": "JN79",
            "antenna": "80m dipole",
            "notes": "Interval signal, then news",
            "pirate": false
        }"#;
        let e: SwlEntry = serde_json::from_str(old).expect("an old log file must still load");
        assert_eq!(e.station, "Radio Prague", "and it must be intact");
        assert_eq!(e.notes, "Interval signal, then news");
        // The new fields are absent, so they read as "unknown" rather than 0,0
        // off the coast of Ghana.
        assert_eq!(e.lat, None, "absent means unknown, not zero");
        assert_eq!(e.lon, None);
        // And a round trip does not invent them either.
        let back: SwlEntry = serde_json::from_str(&serde_json::to_string(&e).unwrap()).unwrap();
        assert_eq!(back.lat, None);
        assert_eq!(back.station, "Radio Prague");
    }

    /// And a new entry with coordinates keeps them, so a station logged off the
    /// schedule can be plotted on the LISTEN map afterwards.
    #[test]
    fn coordinates_survive_the_round_trip() {
        let e = entry();
        let back: SwlEntry =
            serde_json::from_str(&serde_json::to_string(&e).unwrap()).expect("round trip");
        assert_eq!(back.lat, e.lat);
        assert_eq!(back.lon, e.lon);
        assert_eq!(back.station, e.station);
    }

    fn entry() -> SwlEntry {
        SwlEntry {
            id: 7,
            // 2026-09-16 19:42:00 UTC
            heard_at_unix: 1_789_587_720,
            station: "Radio Taiwan International".into(),
            lat: Some(24.79),
            lon: Some(121.54),
            freq_hz: 6_185_000.0,
            mode: crate::Mode::Am,
            language: "English".into(),
            report: Some(SignalReport::Sinpo(Sinpo { s: 4, i: 3, n: 3, p: 4, o: 4 })),
            smeter_dbm: Some(-73.0),
            site: "Tamsui".into(),
            recv_grid: "JO22".into(),
            antenna: "Longwire 20 m".into(),
            notes: "News, then music".into(),
            // Contact left empty so the exact-text report test covers the
            // no-contact case; `the_report_says_where_to_send_it` sets them.
            email: String::new(),
            address: String::new(),
            // Both stamped, so the serde round-trip and the pre-field-load
            // test cover them.
            report_sent_unix: Some(1_789_588_000),
            qsl_received_unix: Some(1_790_000_000),
            pirate: true,
        }
    }

    #[test]
    fn an_entry_round_trips_through_json() {
        let e = entry();
        let text = serde_json::to_string(&e).unwrap();
        let back: SwlEntry = serde_json::from_str(&text).unwrap();
        assert_eq!(e, back);
    }

    /// A log written before the flag existed has no `pirate` field; it must
    /// load as "not a pirate" rather than fail to parse — the whole reason the
    /// field is `#[serde(default)]`-covered.
    #[test]
    fn an_entry_without_the_flag_loads_as_not_a_pirate() {
        let mut v = serde_json::to_value(entry()).unwrap();
        v.as_object_mut().unwrap().remove("pirate");
        let back: SwlEntry = serde_json::from_value(v).unwrap();
        assert!(!back.pirate);
    }

    /// An entry written by an older build, before a field existed, must load:
    /// every field is `#[serde(default)]`.
    #[test]
    fn an_older_entry_still_loads() {
        let json = r#"{"station":"BBC","freq_hz":9410000.0,"report":{"sinpo":{"s":5,"i":4,"n":4,"p":4,"o":5}}}"#;
        let e: SwlEntry = serde_json::from_str(json).unwrap();
        assert_eq!(e.station, "BBC");
        assert_eq!(e.report, Some(SignalReport::Sinpo(Sinpo { s: 5, i: 4, n: 4, p: 4, o: 5 })));
        assert_eq!(e.mode, crate::Mode::Am, "a missing mode defaults to AM");
        assert!(e.notes.is_empty());
        assert!(e.report_sent_unix.is_none(), "old entry: no report sent yet");
        assert!(e.qsl_received_unix.is_none(), "old entry: no QSL yet");
    }

    #[test]
    fn reports_print_the_way_a_listener_writes_them() {
        assert_eq!(
            SignalReport::Sinpo(Sinpo { s: 4, i: 3, n: 3, p: 4, o: 4 }).digits(),
            "4 3 3 4 4"
        );
        assert_eq!(SignalReport::Sio(Sio { s: 4, i: 3, o: 4 }).digits(), "4 3 4");
        assert_eq!(SignalReport::Sio(Sio::default()).label(), "SIO");
    }

    #[test]
    fn the_frequency_reads_in_khz_and_mhz() {
        assert_eq!(entry().frequency_text(), "6185 kHz (6.185 MHz)");
    }

    /// The whole log's export — the CSV and the reception ADIF, as distinct
    /// from the single-entry `REPORT` text. The ADIF says a listener *heard*
    /// the station, and carries both reporting dates the loop is about.
    #[test]
    fn the_log_exports_as_csv_and_reception_adif() {
        let e = entry();
        let csv = swl_log_to_csv(std::slice::from_ref(&e));
        let mut lines = csv.lines();
        assert!(lines.next().unwrap().starts_with("utc,station,freq_khz"));
        let row = lines.next().unwrap();
        assert!(row.contains("2026-09-16 19:42:00"), "{row}");
        assert!(row.contains("6185.000") && row.contains("6.185000"), "{row}");
        assert!(row.contains("AM,"), "mode: {row}");
        assert!(row.contains("SINPO,4 3 3 4 4"), "report split: {row}");
        assert!(row.contains(",yes,"), "pirate flag: {row}");

        let adif = swl_log_to_adif(std::slice::from_ref(&e));
        assert!(adif.contains("<CALL:26>Radio Taiwan International"), "{adif}");
        assert!(adif.contains("<SWL:1>Y"), "{adif}");
        assert!(adif.contains("<QSL_SENT:1>Y") && adif.contains("<QSLSDATE:8>2026"), "{adif}");
        assert!(adif.contains("<QSL_RCVD:1>Y") && adif.contains("<QSLRDATE:8>2026"), "{adif}");
        assert!(adif.contains("<MY_GRIDSQUARE:4>JO22"), "{adif}");
        assert_eq!(adif.matches("<EOR>").count(), 1);
    }

    /// A station name with a comma or a quote must not break the CSV, and a
    /// hearing that names no station cannot be an ADIF record at all — a logger
    /// would drop it or import a blank call.
    #[test]
    fn the_export_quotes_csv_oddities_and_skips_a_nameless_record() {
        let mut e = entry();
        e.station = "Voice of, \"Hope\"".into();
        let csv = swl_log_to_csv(std::slice::from_ref(&e));
        assert!(csv.lines().nth(1).unwrap().contains("\"Voice of, \"\"Hope\"\"\""), "{csv}");

        e.station = "   ".into();
        assert!(swl_entry_to_adif_record(&e).is_none());
        assert!(!swl_log_to_adif(std::slice::from_ref(&e)).contains("<CALL"));
    }

    #[test]
    fn the_report_is_the_expected_text() {
        let text = entry().report_text("19DCG373", "JO22aa", "RTL-SDR + sdroxide", "long wire");
        let want = "Reception report\n\n\
                    Station:    Radio Taiwan International\n\
                    Frequency:  6185 kHz (6.185 MHz), AM\n\
                    Heard:      2026-09-16 19:42 UTC\n\
                    Location:   JO22aa\n\
                    Receiver:   RTL-SDR + sdroxide\n\
                    Antenna:    long wire\n\
                    SINPO:  4 3 3 4 4\n\
                    Notes:      News, then music\n\
                    Reported by:19DCG373\n";
        assert_eq!(text, want);
    }

    /// The report names where to send it when the schedule gave a contact, and
    /// leaves the block out entirely when it did not.
    #[test]
    fn the_report_says_where_to_send_it() {
        let mut e = entry();
        e.email = "rti@rti.org.tw".into();
        e.address = "55 Beian Road, Taipei".into();
        let text = e.report_text("", "", "", "");
        assert!(text.contains("Send to:"), "{text}");
        assert!(text.contains("rti@rti.org.tw"), "{text}");
        assert!(text.contains("55 Beian Road, Taipei"), "{text}");

        assert!(!entry().report_text("", "", "", "").contains("Send to:"));
    }

    /// The missing pieces say so, rather than printing blanks or zeroes.
    #[test]
    fn an_unjudged_report_says_so() {
        let mut e = entry();
        e.report = None;
        e.notes.clear();
        let text = e.report_text("", "", "", "");
        assert!(text.contains("Signal report: not judged"), "{text}");
        assert!(!text.contains("Location:"), "{text}");
        assert!(!text.contains("Notes:"), "{text}");
        assert!(!text.contains("Reported by:"), "{text}");
    }
}

/// The listener's own preferences for mailing a reception report to a
/// broadcaster — what the report carries besides the reception itself.
///
/// **Local to this machine and not on the wire.** `picture` is a path to a file
/// on the operator's disk: their own card, a photograph, whatever they want a
/// station to see. That is nobody else's to relay to a browser tab or a second
/// station, so it lives beside the reception log in `swl_report.json` rather
/// than in a config that travels (`NetworkConfig::swl_id` is the *identity*,
/// and it is on the wire; a file path is not an identity).
///
/// Both default to empty, and both are genuinely optional: a reception report is
/// a courtesy, and a plain-text one is a perfectly good report. Nothing here is
/// required for the report to be sent.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SwlReportPrefs {
    /// Absolute path to the picture a report carries. Empty = none, and the
    /// report goes as text.
    pub picture: String,
    /// The listener's standing note, pre-filled into every report — who they
    /// are, what they are listening on, anything a station asks for once rather
    /// than per letter. A convenience, never a secret: this is a plain file.
    pub message: String,
}
