//! The aircraft the operator asked to be told about.
//!
//! A short list of callsigns and registrations, kept by the station so it is the
//! same from every screen that connects to it, and served on `/aircraft-watch`.
//! This is only the list: whether a watched aircraft is on the air is decided by
//! the client, from the cards it already holds, and nothing here reaches
//! anybody. Pure: no clock, no file, no socket.

use serde::{Deserialize, Serialize};

use crate::{FORMAT_VERSION, Status};

/// Most entries kept. A list is for a handful of aircraft, not for a fleet.
pub const MAX_ENTRIES: usize = 100;
/// Shortest and longest an entry may be once spelled plainly (`PHBXA`, `KLM1234`).
pub const MIN_LEN: usize = 3;
pub const MAX_LEN: usize = 10;

/// An entry as it is kept and compared: upper case, letters and digits only, so
/// `ph-bxa`, `PH BXA` and `PHBXA` are one aircraft. Nothing for a scrap too short
/// or too long to be a callsign or a registration.
pub fn normalize(raw: &str) -> Option<String> {
    let plain: String =
        raw.chars().filter(char::is_ascii_alphanumeric).map(|c| c.to_ascii_uppercase()).collect();
    (MIN_LEN..=MAX_LEN).contains(&plain.len()).then_some(plain)
}

/// What an edit came to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Outcome {
    Added,
    /// Already on the list: nothing changed.
    Already,
    Removed,
    /// Not on the list: nothing changed.
    Absent,
    /// Not something that can be a callsign or a registration.
    Invalid,
    /// The list is at [`MAX_ENTRIES`].
    Full,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Watchlist {
    entries: Vec<String>,
}

impl Watchlist {
    pub fn entries(&self) -> &[String] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn contains(&self, raw: &str) -> bool {
        normalize(raw).is_some_and(|e| self.entries.binary_search(&e).is_ok())
    }

    pub fn add(&mut self, raw: &str) -> Outcome {
        let Some(entry) = normalize(raw) else { return Outcome::Invalid };
        match self.entries.binary_search(&entry) {
            Ok(_) => Outcome::Already,
            Err(_) if self.entries.len() >= MAX_ENTRIES => Outcome::Full,
            Err(at) => {
                self.entries.insert(at, entry);
                Outcome::Added
            }
        }
    }

    pub fn remove(&mut self, raw: &str) -> Outcome {
        let Some(entry) = normalize(raw) else { return Outcome::Invalid };
        match self.entries.binary_search(&entry) {
            Ok(at) => {
                self.entries.remove(at);
                Outcome::Removed
            }
            Err(_) => Outcome::Absent,
        }
    }

    /// The entries an aircraft answers to, by its callsign or its registration.
    pub fn matching(&self, callsign: Option<&str>, registration: Option<&str>) -> Vec<&str> {
        let wanted: Vec<String> =
            [callsign, registration].into_iter().flatten().filter_map(normalize).collect();
        self.entries.iter().filter(|e| wanted.contains(e)).map(String::as_str).collect()
    }

    /// The file's text.
    pub fn to_json(&self) -> String {
        serde_json::to_string(&Disk { v: FORMAT_VERSION, entries: self.entries.clone() })
            .unwrap_or_default()
    }

    /// Read a file's text. Anything that is not a file of this version gives an
    /// empty list; an entry that is not valid is dropped, a repeat is kept once,
    /// and the list is cut to [`MAX_ENTRIES`].
    pub fn from_json(text: &str) -> Watchlist {
        let Ok(disk) = serde_json::from_str::<Disk>(text) else { return Watchlist::default() };
        if disk.v != FORMAT_VERSION {
            return Watchlist::default();
        }
        let mut list = Watchlist::default();
        for e in disk.entries {
            list.add(&e);
        }
        list
    }
}

#[derive(Serialize, Deserialize)]
struct Disk {
    v: u32,
    entries: Vec<String>,
}

/// What the client sends to change the list: one thing at a time.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct WatchEdit {
    #[serde(default)]
    pub add: Option<String>,
    #[serde(default)]
    pub remove: Option<String>,
}

/// The answer to `GET` and `POST /aircraft-watch`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WatchReply {
    pub v: u32,
    pub status: Status,
    /// What the edit came to; none for a plain `GET`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outcome: Option<Outcome>,
    pub entries: Vec<String>,
}

impl WatchReply {
    pub fn bare(status: Status) -> WatchReply {
        WatchReply { v: FORMAT_VERSION, status, outcome: None, entries: Vec::new() }
    }

    pub fn of(list: &Watchlist, outcome: Option<Outcome>) -> WatchReply {
        WatchReply {
            v: FORMAT_VERSION,
            status: Status::Ok,
            outcome,
            entries: list.entries().to_vec(),
        }
    }
}

impl WatchEdit {
    /// Apply the edit to `list`. Exactly one of the two must be given: both or
    /// neither is [`Outcome::Invalid`], and the list is left alone.
    pub fn apply(&self, list: &mut Watchlist) -> Outcome {
        match (&self.add, &self.remove) {
            (Some(e), None) => list.add(e),
            (None, Some(e)) => list.remove(e),
            _ => Outcome::Invalid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list(of: &[&str]) -> Watchlist {
        let mut l = Watchlist::default();
        for e in of {
            assert_eq!(l.add(e), Outcome::Added, "{e}");
        }
        l
    }

    // ── what an entry is ──────────────────────────────────────────────────

    #[test]
    fn an_entry_is_upper_case_letters_and_digits_only() {
        assert_eq!(normalize("ph-bxa").as_deref(), Some("PHBXA"));
        assert_eq!(normalize(" KLM 1234 ").as_deref(), Some("KLM1234"));
        assert_eq!(normalize("d.aima").as_deref(), Some("DAIMA"));
        assert_eq!(normalize("N123AB").as_deref(), Some("N123AB"));
    }

    #[test]
    fn a_scrap_is_not_an_entry() {
        for s in ["", "  ", "--", "ab", "A", "ABCDEFGHIJK", "KLM/1234!", "é-é-é"] {
            // "KLM/1234!" has letters and digits in it; the punctuation goes and
            // what is left is a callsign, so it is the only one of these that is kept.
            let want = (s == "KLM/1234!").then(|| "KLM1234".to_string());
            assert_eq!(normalize(s), want, "{s:?}");
        }
        assert_eq!(normalize(&"A".repeat(MAX_LEN)).map(|s| s.len()), Some(MAX_LEN));
        assert_eq!(normalize(&"A".repeat(MIN_LEN)).map(|s| s.len()), Some(MIN_LEN));
    }

    // ── adding and removing ───────────────────────────────────────────────

    #[test]
    fn adding_keeps_it_once_and_spelled_the_same_way() {
        let mut l = Watchlist::default();
        assert_eq!(l.add("ph-bxa"), Outcome::Added);
        assert_eq!(l.add("PHBXA"), Outcome::Already);
        assert_eq!(l.add("ph bxa"), Outcome::Already);
        assert_eq!(l.entries(), ["PHBXA"]);
        assert!(l.contains("ph-bxa") && !l.contains("KLM1"));
    }

    #[test]
    fn a_scrap_is_refused_and_changes_nothing() {
        let mut l = list(&["KLM1234"]);
        assert_eq!(l.add("x"), Outcome::Invalid);
        assert_eq!(l.entries(), ["KLM1234"]);
    }

    #[test]
    fn the_list_is_ordered_whatever_the_order_it_was_built_in() {
        assert_eq!(list(&["KLM1", "BAW9", "AFR2"]).entries(), ["AFR2", "BAW9", "KLM1"]);
    }

    #[test]
    fn the_list_stops_at_its_ceiling_and_a_repeat_at_the_ceiling_is_not_full() {
        let mut l = Watchlist::default();
        for i in 0..MAX_ENTRIES {
            assert_eq!(l.add(&format!("KLM{i:03}")), Outcome::Added);
        }
        assert_eq!(l.add("BAW999"), Outcome::Full);
        assert_eq!(l.len(), MAX_ENTRIES);
        assert_eq!(l.add("KLM000"), Outcome::Already, "known, so not a reason to refuse");
        assert_eq!(l.remove("KLM000"), Outcome::Removed);
        assert_eq!(l.add("BAW999"), Outcome::Added, "room again");
    }

    #[test]
    fn removing_takes_it_off_whatever_the_spelling() {
        let mut l = list(&["PHBXA", "KLM1"]);
        assert_eq!(l.remove("ph-bxa"), Outcome::Removed);
        assert_eq!(l.entries(), ["KLM1"]);
        assert_eq!(l.remove("PHBXA"), Outcome::Absent);
        assert_eq!(l.remove("?"), Outcome::Invalid);
        assert_eq!(l.entries(), ["KLM1"]);
    }

    // ── matching ──────────────────────────────────────────────────────────

    #[test]
    fn an_aircraft_answers_to_its_callsign_or_its_registration() {
        let l = list(&["KLM1234", "PHBXA", "BAW9"]);
        assert_eq!(l.matching(Some("KLM1234"), None), ["KLM1234"]);
        assert_eq!(l.matching(None, Some("PH-BXA")), ["PHBXA"]);
        assert_eq!(l.matching(Some(" klm1234 "), Some("PH-BXA")), ["KLM1234", "PHBXA"]);
        assert!(l.matching(Some("KLM12"), Some("PH-BXB")).is_empty(), "whole, not a prefix");
        assert!(l.matching(None, None).is_empty());
        assert!(Watchlist::default().matching(Some("KLM1234"), None).is_empty());
    }

    // ── the file ──────────────────────────────────────────────────────────

    #[test]
    fn the_list_goes_to_text_and_back() {
        let l = list(&["KLM1", "PHBXA"]);
        assert_eq!(Watchlist::from_json(&l.to_json()), l);
        assert_eq!(Watchlist::from_json(&Watchlist::default().to_json()), Watchlist::default());
    }

    #[test]
    fn a_file_that_is_not_ours_is_an_empty_list() {
        for t in ["", "garbage", "[]", r#"{"v":99,"entries":["KLM1"]}"#, r#"{"entries":["KLM1"]}"#]
        {
            assert!(Watchlist::from_json(t).is_empty(), "{t:?}");
        }
    }

    #[test]
    fn a_file_with_bad_entries_loses_those_and_keeps_the_rest_once() {
        let t =
            format!(r#"{{"v":{FORMAT_VERSION},"entries":["klm1","x","KLM1","ph-bxa",7,"BAW9"]}}"#);
        // The `7` is not text: the whole file is not ours, rather than guessed at.
        assert!(Watchlist::from_json(&t).is_empty());
        let t =
            format!(r#"{{"v":{FORMAT_VERSION},"entries":["klm1","x","KLM1","ph-bxa","BAW9"]}}"#);
        assert_eq!(Watchlist::from_json(&t).entries(), ["BAW9", "KLM1", "PHBXA"]);
    }

    #[test]
    fn a_file_longer_than_the_ceiling_is_cut() {
        let many: Vec<String> = (0..MAX_ENTRIES + 20).map(|i| format!("KLM{i:03}")).collect();
        let t = serde_json::json!({"v": FORMAT_VERSION, "entries": many}).to_string();
        assert_eq!(Watchlist::from_json(&t).len(), MAX_ENTRIES);
    }

    // ── what is sent and answered ─────────────────────────────────────────

    #[test]
    fn an_edit_is_one_add_or_one_remove() {
        let mut l = list(&["KLM1"]);
        let add = WatchEdit { add: Some("baw9".into()), remove: None };
        assert_eq!(add.apply(&mut l), Outcome::Added);
        let rem = WatchEdit { add: None, remove: Some("KLM1".into()) };
        assert_eq!(rem.apply(&mut l), Outcome::Removed);
        assert_eq!(l.entries(), ["BAW9"]);
        let both = WatchEdit { add: Some("AFR2".into()), remove: Some("BAW9".into()) };
        assert_eq!(both.apply(&mut l), Outcome::Invalid);
        assert_eq!(WatchEdit::default().apply(&mut l), Outcome::Invalid);
        assert_eq!(l.entries(), ["BAW9"], "neither was applied");
    }

    #[test]
    fn an_edit_reads_from_json_with_either_field_missing() {
        let e: WatchEdit = serde_json::from_str(r#"{"add":"KLM1"}"#).unwrap();
        assert_eq!((e.add.as_deref(), e.remove), (Some("KLM1"), None));
        let e: WatchEdit = serde_json::from_str("{}").unwrap();
        assert_eq!(e, WatchEdit::default());
    }

    #[test]
    fn the_reply_carries_the_list_and_what_the_edit_came_to() {
        let l = list(&["KLM1"]);
        let r = WatchReply::of(&l, Some(Outcome::Added));
        let back: WatchReply = serde_json::from_str(&serde_json::to_string(&r).unwrap()).unwrap();
        assert_eq!(back, r);
        assert_eq!(
            (back.v, back.status, back.entries.as_slice()),
            (FORMAT_VERSION, Status::Ok, ["KLM1".to_string()].as_slice())
        );
        let plain = serde_json::to_string(&WatchReply::of(&l, None)).unwrap();
        assert!(!plain.contains("outcome"), "{plain}");
        assert_eq!(WatchReply::bare(Status::Unauthorized).entries.len(), 0);
    }

    #[test]
    fn this_module_touches_no_file_and_no_socket() {
        let src = include_str!("watch.rs");
        let code = &src[..src.find("#[cfg(test)]").unwrap()];
        for banned in ["std::fs", "File", "TcpStream", "ureq", "SystemTime"] {
            assert!(!code.contains(banned), "watch.rs must not touch `{banned}`");
        }
        assert!(code.contains("Watchlist"), "the control: the scan sees what is there");
    }
}
