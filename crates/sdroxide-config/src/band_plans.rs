//! The station's library of band plans: one `<name>.json` per plan in the
//! `bandplans` folder of the engine's config directory, and which of them the
//! station runs.
//!
//! The folder is the engine's, never a client's. A browser has no folder of the
//! station's to write to and a remote screen would write its own laptop's copy,
//! so a screen sends the *text* of a plan over the wire
//! ([`sdroxide_types::Command::ImportBandPlan`]) and the engine, which owns the
//! disk, checks it and writes it. Native, remote and web all take that road.
//!
//! Everything here takes the config directory as a parameter (`*_in`), which is
//! what lets the tests run without redirecting the process-wide one; the
//! functions without the suffix are the same thing on the real directory.
//!
//! ## What "the plan the station runs" means
//!
//! `config.toml` holds the name of the chosen plan ([`Settings::band_plan`]),
//! `None` for the built-in IARU tables — the default, and where "back to the
//! default" lands in one click. A name whose file has gone, or will not parse,
//! runs the built-in tables for that session with an alert and is left as it
//! is: it is somebody's document, and moving it aside would take it away at
//! the moment they need to fix it.

use std::fs;
use std::path::{Path, PathBuf};

use sdroxide_types::{BandPlan, PlanInfo};
use tracing::{info, warn};

use crate::{ConfigError, Settings, config_dir, push_load_alert, write_atomic};

/// The folder's name in the config directory.
pub const BAND_PLANS_DIR: &str = "bandplans";

/// The largest plan file taken in. The real ones are 20–30 kB; this is to stop
/// a wrong file (a log, a picture) from being read whole off the wire and
/// stored, not to limit anybody's plan.
pub const BAND_PLAN_MAX_BYTES: usize = 512 * 1024;

/// The longest key a plan can have.
const KEY_MAX: usize = 48;

/// The file the station's band plan lived in before there was a library. Read
/// once, to be carried into the library, and never written again.
const LEGACY_FILE: &str = "bandplan.json";

/// The key the carried-over legacy plan is stored under.
const LEGACY_KEY: &str = "own";

/// Plans that come with the program, copied into a new library the first time
/// it is made: `(key, text)`. Never overwritten afterwards — a shipped plan the
/// operator edited stays as they left it.
pub const SHIPPED: &[(&str, &str)] =
    &[("bandplan-de-nrw", include_str!("../assets/bandplans/bandplan-de-nrw.json"))];

/// Whether `key` names a plan that comes with the program.
pub fn is_shipped(key: &str) -> bool {
    SHIPPED.iter().any(|(k, _)| *k == key)
}

/// The key a plan is stored under for the name somebody typed or a file bore:
/// lower case, letters, digits and single hyphens.
///
/// This is also what keeps a name from being a path. `../x` is `x`, `a/b` is
/// `ab`, and a dot cannot survive, so no key can end in `.bak` and be mistaken
/// for the backup of another plan.
pub fn plan_key(name: &str) -> Result<String, String> {
    let name = name.trim();
    let name = name
        .strip_suffix(".json")
        .or_else(|| name.strip_suffix(".JSON"))
        .unwrap_or(name);
    let mut key = String::new();
    for c in name.chars() {
        if c.is_ascii_alphanumeric() {
            key.push(c.to_ascii_lowercase());
        } else if matches!(c, ' ' | '-' | '_') && !key.ends_with('-') && !key.is_empty() {
            key.push('-');
        }
    }
    let key = key.trim_matches('-');
    if key.is_empty() {
        return Err("A band plan needs a name made of letters or digits.".into());
    }
    Ok(key.chars().take(KEY_MAX).collect::<String>().trim_end_matches('-').to_string())
}

/// `dir`'s band-plan folder.
pub fn library_in(dir: &Path) -> PathBuf {
    dir.join(BAND_PLANS_DIR)
}

fn file_in(dir: &Path, key: &str) -> PathBuf {
    library_in(dir).join(format!("{key}.json"))
}

/// Make the library if it is not there yet: the shipped plans go in, and a
/// band plan the station ran before there was a library is carried over.
///
/// Returns the key of the carried-over plan when there was one that is not the
/// built-in tables (the caller makes it the station's choice, once).
pub fn ensure_library_in(dir: &Path) -> Result<Option<String>, ConfigError> {
    let lib = library_in(dir);
    if lib.is_dir() {
        return Ok(None);
    }
    fs::create_dir_all(&lib)?;
    for (key, text) in SHIPPED {
        write_atomic(&lib, &format!("{key}.json"), text)?;
    }
    // The legacy file is left exactly where it is: it was the operator's own
    // document, and a copy in the library is all that is wanted of it.
    let Ok(text) = fs::read_to_string(dir.join(LEGACY_FILE)) else { return Ok(None) };
    match serde_json::from_str::<BandPlan>(&text) {
        Ok(plan) if !plan.is_default() => {
            write_atomic(&lib, &format!("{LEGACY_KEY}.json"), &text)?;
            info!("carried the station's {LEGACY_FILE} into the library as {LEGACY_KEY}");
            Ok(Some(LEGACY_KEY.to_string()))
        }
        // The built-in tables written out by an older start: nothing to carry.
        Ok(_) => Ok(None),
        Err(e) => {
            warn!("{LEGACY_FILE} could not be carried into the library: {e}");
            Ok(None)
        }
    }
}

/// The plans in `dir`'s library, by name. A file that is not a plan is left out
/// (and said so in the log) rather than listed as something that cannot run.
pub fn list_in(dir: &Path) -> Vec<PlanInfo> {
    let Ok(rd) = fs::read_dir(library_in(dir)) else { return Vec::new() };
    let mut out = Vec::new();
    for entry in rd.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("json") {
            continue;
        }
        let Some(key) = path.file_stem().and_then(|s| s.to_str()).map(str::to_owned) else {
            continue;
        };
        let Ok(text) = fs::read_to_string(&path) else { continue };
        let plan = match serde_json::from_str::<BandPlan>(&text) {
            Ok(p) => p,
            Err(e) => {
                warn!("{}: not a band plan ({e}); not listed", path.display());
                continue;
            }
        };
        let meta = plan.meta();
        let md = entry.metadata().ok();
        out.push(PlanInfo {
            name: if meta.name.trim().is_empty() { key.clone() } else { meta.name.clone() },
            country: meta.country.clone(),
            author: meta.author.clone(),
            bytes: u32::try_from(text.len()).unwrap_or(u32::MAX),
            modified: md
                .and_then(|m| m.modified().ok())
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map_or(0, |d| d.as_secs()),
            shipped: is_shipped(&key),
            file: key,
        });
    }
    out.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()).then(a.file.cmp(&b.file)));
    out
}

/// The plan called `key` in `dir`'s library.
pub fn read_in(dir: &Path, key: &str) -> Result<BandPlan, String> {
    let path = file_in(dir, key);
    let text = fs::read_to_string(&path)
        .map_err(|e| format!("The band plan “{key}” could not be read ({e})."))?;
    serde_json::from_str::<BandPlan>(&text)
        .map_err(|e| format!("The band plan “{key}” is not a valid plan ({e})."))
}

/// What a plan file must be for the station to take it in: small enough, a plan
/// that parses, and with nothing in it that has to be dropped or repaired.
///
/// Stricter than loading, on purpose. A file somebody has already been running
/// loses its odd row with a notice; a file arriving *now* is refused whole, so
/// the operator is told while they are looking at the file that made it.
pub fn check_plan_text(text: &str) -> Result<BandPlan, String> {
    if text.len() > BAND_PLAN_MAX_BYTES {
        return Err(format!(
            "That file is {} kB; a band plan is a few tens of kB at most.",
            text.len() / 1024
        ));
    }
    let plan = serde_json::from_str::<BandPlan>(text.trim_start_matches('\u{feff}'))
        .map_err(|e| format!("That file is not a band plan: {e}"))?;
    if !plan.problems().is_empty() {
        let shown: Vec<&str> = plan.problems().iter().take(3).map(String::as_str).collect();
        let more = plan.problems().len().saturating_sub(shown.len());
        return Err(format!(
            "That band plan has errors: {}{}",
            shown.join(" "),
            if more > 0 { format!(" (and {more} more)") } else { String::new() }
        ));
    }
    Ok(plan)
}

/// Add the plan `text` to `dir`'s library under `name`, returning its key.
///
/// A key already in use is replaced; the file it had is kept once, as
/// `<key>.json.bak` — one backup, overwritten by the next replacement, not a
/// numbered series.
pub fn import_in(dir: &Path, name: &str, text: &str) -> Result<String, String> {
    let key = plan_key(name)?;
    check_plan_text(text)?;
    ensure_library_in(dir).map_err(|e| e.to_string())?;
    let lib = library_in(dir);
    let path = file_in(dir, &key);
    if path.exists() {
        let bak = lib.join(format!("{key}.json.bak"));
        fs::copy(&path, &bak).map_err(|e| format!("The old “{key}” could not be backed up: {e}"))?;
    }
    write_atomic(&lib, &format!("{key}.json"), text.trim_start_matches('\u{feff}'))
        .map_err(|e| format!("The band plan could not be saved: {e}"))?;
    Ok(key)
}

/// Delete the plan called `key` from `dir`'s library. Its `.bak`, if any, stays.
pub fn delete_in(dir: &Path, key: &str) -> Result<(), String> {
    let key = plan_key(key)?;
    fs::remove_file(file_in(dir, &key))
        .map_err(|e| format!("The band plan “{key}” could not be deleted ({e})."))
}

/// Keep `plan` — the plan the station runs, or the built-in tables — in `dir`'s
/// library under `name`, returning its key. The name is also what the list shows
/// (the plan's own country and author, if it has them, are kept). A key already
/// in use is replaced, the old file kept once as `<key>.json.bak`.
pub fn save_current_in(dir: &Path, name: &str, plan: &BandPlan) -> Result<String, String> {
    let key = plan_key(name)?;
    let mut meta = plan.meta().clone();
    meta.name = name.trim().chars().take(80).collect();
    let text = plan.clone().with_meta(meta).to_json_document();
    import_in(dir, &key, &text)
}

/// The text of the plan called `name` in `dir`'s library, with its key.
pub fn export_text_in(dir: &Path, name: &str) -> Result<(String, String), String> {
    let key = plan_key(name)?;
    let text = fs::read_to_string(file_in(dir, &key))
        .map_err(|e| format!("The band plan “{key}” could not be read ({e})."))?;
    Ok((key, text))
}

/// Put the shipped plans back in `dir`'s library: one that is missing is written
/// again, one that differs from what shipped is replaced (the file it had kept
/// once as `.bak`). Returns the keys that were written.
pub fn restore_shipped_in(dir: &Path) -> Result<Vec<String>, String> {
    ensure_library_in(dir).map_err(|e| e.to_string())?;
    let mut written = Vec::new();
    for (key, text) in SHIPPED {
        let now = fs::read_to_string(file_in(dir, key)).ok();
        if now.as_deref() != Some(*text) {
            import_in(dir, key, text)?;
            written.push((*key).to_string());
        }
    }
    Ok(written)
}

/// The plans some radio runs on its own: the `band_plan` of the station's own
/// `radio.json` and of every `radio-<n>/radio.json` in `dir`. A file that is
/// missing or will not parse names nothing.
pub fn plans_used_by_radios_in(dir: &Path) -> Vec<String> {
    let mut files = vec![dir.join("radio.json")];
    if let Ok(rd) = fs::read_dir(dir) {
        for e in rd.flatten() {
            let name = e.file_name();
            if name.to_string_lossy().starts_with("radio-") && e.path().is_dir() {
                files.push(e.path().join("radio.json"));
            }
        }
    }
    let mut used: Vec<String> = files
        .iter()
        .filter_map(|f| fs::read_to_string(f).ok())
        .filter_map(|t| serde_json::from_str::<serde_json::Value>(&t).ok())
        .filter_map(|v| v.get("band_plan")?.as_str().map(str::to_owned))
        .filter_map(|n| plan_key(&n).ok())
        .collect();
    used.sort();
    used.dedup();
    used
}

// ── The same on the real config directory ───────────────────────────────────

/// Where the library is, for the settings dialog to show the operator.
pub fn band_plans_dir() -> Option<PathBuf> {
    config_dir().ok().map(|d| library_in(&d))
}

/// The library, made first if it is the first time. A plan carried over from a
/// station that predates the library becomes its choice here, once.
fn ensure_library() -> Result<PathBuf, String> {
    let dir = config_dir().map_err(|e| e.to_string())?;
    if let Some(key) = ensure_library_in(&dir).map_err(|e| e.to_string())? {
        let mut s = Settings::load();
        if s.band_plan.is_none() {
            s.band_plan = Some(key);
            if let Err(e) = s.save() {
                warn!("could not record the carried-over band plan as the station's: {e}");
            }
        }
    }
    Ok(dir)
}

/// The plans in the station's library.
pub fn list_band_plans() -> Vec<PlanInfo> {
    ensure_library().map(|d| list_in(&d)).unwrap_or_default()
}

/// Which plan the station runs, or `None` for the built-in IARU tables.
pub fn band_plan_selection() -> Option<String> {
    Settings::load().band_plan
}

/// Add a plan to the station's library under `name`. See [`import_in`].
pub fn import_band_plan(name: &str, text: &str) -> Result<String, String> {
    import_in(&ensure_library()?, name, text)
}

/// Keep `plan` in the station's library under `name`. See [`save_current_in`].
pub fn save_current_band_plan(name: &str, plan: &BandPlan) -> Result<String, String> {
    save_current_in(&ensure_library()?, name, plan)
}

/// The file of a plan in the station's library, with its key.
pub fn export_band_plan_text(name: &str) -> Result<(String, String), String> {
    export_text_in(&ensure_library()?, name)
}

/// Put the shipped plans back in the station's library. See [`restore_shipped_in`].
pub fn restore_shipped_band_plans() -> Result<Vec<String>, String> {
    restore_shipped_in(&ensure_library()?)
}

/// The plan called `name` in the station's library.
pub fn read_band_plan(name: &str) -> Result<BandPlan, String> {
    let dir = ensure_library()?;
    read_in(&dir, &plan_key(name)?)
}

/// Delete a plan from the station's library. Refused for the plan the station
/// runs and for one any radio runs on its own: it would leave `config.toml` or a
/// `radio.json` naming a file that is not there.
pub fn delete_band_plan(name: &str) -> Result<(), String> {
    let dir = ensure_library()?;
    let key = plan_key(name)?;
    if band_plan_selection().as_deref() == Some(key.as_str()) {
        return Err(format!(
            "The band plan “{key}” is the one running. Choose another plan first."
        ));
    }
    if plans_used_by_radios_in(&dir).contains(&key) {
        return Err(format!(
            "The band plan “{key}” is run by a radio of its own. Choose another plan for that \
             radio first."
        ));
    }
    delete_in(&dir, &key)
}

/// Make `name` the plan the station runs (`None`: the built-in IARU tables),
/// remember it in `config.toml`, and return it.
///
/// The plan is read *before* anything is written, so a name that is not in the
/// library, or a file that will not parse, leaves the choice as it was.
pub fn select_band_plan(name: Option<&str>) -> Result<BandPlan, String> {
    let dir = ensure_library()?;
    let (key, plan) = match name {
        None => (None, BandPlan::default()),
        Some(n) => {
            let key = plan_key(n)?;
            let plan = read_in(&dir, &key)?;
            (Some(key), plan)
        }
    };
    let mut s = Settings::load();
    s.band_plan = key;
    s.save().map_err(|e| format!("The choice could not be saved: {e}"))?;
    Ok(plan)
}

/// The band plan the station runs: the one `config.toml` names, read from the
/// library, or the built-in IARU tables when it names none.
///
/// A name whose file is missing or will not parse runs the built-in tables for
/// this session, says so through the load alerts, and leaves both the name and
/// the file alone — see the module notes. A plan that loads with rows to drop
/// reports each one the same way.
pub fn load_band_plan() -> BandPlan {
    let dir = match ensure_library() {
        Ok(d) => d,
        Err(e) => {
            warn!("band plan library: {e}");
            return BandPlan::default();
        }
    };
    let Some(key) = band_plan_selection() else { return BandPlan::default() };
    let path = file_in(&dir, &key);
    let text = match fs::read_to_string(&path) {
        Ok(t) => t,
        Err(e) => {
            warn!("could not read {}: {e}; using the built-in band plan", path.display());
            push_load_alert(format!(
                "The band plan “{key}” could not be read ({e}) — running on the built-in IARU \
                 band plan. Choose another plan, or go back to the built-in one."
            ));
            return BandPlan::default();
        }
    };
    match serde_json::from_str::<BandPlan>(&text) {
        Ok(plan) => {
            for p in plan.problems() {
                warn!("{key}.json: {p}");
                push_load_alert(format!("{key}.json: {p}"));
            }
            plan
        }
        Err(e) => {
            warn!("failed to parse {key}.json: {e}; using the built-in band plan");
            push_load_alert(format!(
                "The band plan “{key}” could not be read ({e}) — running on the built-in IARU \
                 band plan. The file has been left alone."
            ));
            BandPlan::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sdroxide_types::{Band, Region};
    use std::sync::atomic::{AtomicU32, Ordering};

    /// A fresh directory per test, outside the process-wide config directory.
    fn scratch() -> PathBuf {
        static N: AtomicU32 = AtomicU32::new(0);
        let d = std::env::temp_dir().join(format!(
            "sdroxide-bandplans-{}-{}",
            std::process::id(),
            N.fetch_add(1, Ordering::SeqCst)
        ));
        let _ = fs::remove_dir_all(&d);
        fs::create_dir_all(&d).unwrap();
        d
    }

    /// A plan that is not the built-in one: 2 m narrowed in Region 1.
    fn narrow_2m() -> String {
        r#"{"meta":{"name":"Narrow 2 m","country":"Nowhere","author":"Me"},
            "region1":{"bands":[{"band":"M2","lo_mhz":144.0,"hi_mhz":144.4}]},
            "region2":{"bands":[{"band":"M2","lo_mhz":144.0,"hi_mhz":148.0}]},
            "region3":{"bands":[{"band":"M2","lo_mhz":144.0,"hi_mhz":148.0}]}}"#
            .to_string()
    }

    #[test]
    fn a_name_is_made_into_a_key_that_cannot_be_a_path() {
        assert_eq!(plan_key("Germany (NRW).json").unwrap(), "germany-nrw");
        assert_eq!(plan_key("  My  plan_1 ").unwrap(), "my-plan-1");
        assert_eq!(plan_key("../../etc/passwd").unwrap(), "etcpasswd");
        assert_eq!(plan_key("a/b\\c").unwrap(), "abc");
        // A dot cannot survive, so no key can end in .bak or .tmp.
        assert_eq!(plan_key("x.json.bak").unwrap(), "xjsonbak");
        assert!(plan_key("...").is_err());
        assert!(plan_key("   ").is_err());
        assert!(plan_key("日本語").is_err(), "no letters or digits left");
        assert!(plan_key(&"a".repeat(200)).unwrap().len() <= KEY_MAX);
    }

    #[test]
    fn a_new_library_holds_the_shipped_plans_and_is_not_made_twice() {
        let dir = scratch();
        assert_eq!(ensure_library_in(&dir).unwrap(), None);
        let names: Vec<_> = list_in(&dir).into_iter().map(|p| (p.file, p.name, p.shipped)).collect();
        assert_eq!(
            names,
            vec![("bandplan-de-nrw".into(), "Germany (NRW)".into(), true)],
            "{names:?}"
        );
        // A shipped plan the operator deleted is not brought back by a start.
        delete_in(&dir, "bandplan-de-nrw").unwrap();
        ensure_library_in(&dir).unwrap();
        assert!(list_in(&dir).is_empty());
    }

    #[test]
    fn every_shipped_plan_loads_clean_and_says_who_it_is() {
        for (key, text) in SHIPPED {
            let plan = check_plan_text(text).unwrap_or_else(|e| panic!("{key}: {e}"));
            assert!(!plan.meta().name.is_empty(), "{key} has no name");
            assert!(!plan.meta().author.is_empty(), "{key} has no author");
            assert_eq!(plan_key(key).unwrap(), *key, "{key} is not a key");
        }
    }

    #[test]
    fn the_legacy_file_is_carried_into_the_library_once() {
        let dir = scratch();
        fs::write(dir.join(LEGACY_FILE), narrow_2m()).unwrap();
        assert_eq!(ensure_library_in(&dir).unwrap().as_deref(), Some("own"));
        let own = read_in(&dir, "own").unwrap();
        assert_eq!(
            own.region(Region::R1).edges(Band::M2),
            Some((144_000_000.0, 144_400_000.0))
        );
        assert!(dir.join(LEGACY_FILE).exists(), "the operator's file is left alone");
        // Already carried: a second start does nothing.
        assert_eq!(ensure_library_in(&dir).unwrap(), None);

        // A legacy file that is only the built-in tables has nothing to carry.
        let dir = scratch();
        fs::write(dir.join(LEGACY_FILE), BandPlan::default().to_json_document()).unwrap();
        assert_eq!(ensure_library_in(&dir).unwrap(), None);
        assert!(read_in(&dir, "own").is_err());

        // And one that will not parse is not carried, nor does it stop the library.
        let dir = scratch();
        fs::write(dir.join(LEGACY_FILE), "{ \"region1\": ").unwrap();
        assert_eq!(ensure_library_in(&dir).unwrap(), None);
        assert!(library_in(&dir).is_dir());
    }

    #[test]
    fn an_imported_plan_is_listed_with_its_meta_and_read_back() {
        let dir = scratch();
        let key = import_in(&dir, "Narrow 2m.json", &narrow_2m()).unwrap();
        assert_eq!(key, "narrow-2m");
        let info = list_in(&dir).into_iter().find(|p| p.file == key).unwrap();
        assert_eq!(info.name, "Narrow 2 m");
        assert_eq!(info.country, "Nowhere");
        assert_eq!(info.author, "Me");
        assert!(!info.shipped);
        assert!(info.bytes > 100);
        let plan = read_in(&dir, &key).unwrap();
        assert!(!plan.is_default());
        assert_eq!(plan.meta().author, "Me");
    }

    #[test]
    fn a_plan_without_meta_is_listed_under_its_file_name() {
        let dir = scratch();
        let text = r#"{"region1":{"bands":[{"band":"M2","lo_mhz":144.0,"hi_mhz":146.0}]},
            "region2":{"bands":[{"band":"M2","lo_mhz":144.0,"hi_mhz":148.0}]},
            "region3":{"bands":[{"band":"M2","lo_mhz":144.0,"hi_mhz":148.0}]}}"#;
        let key = import_in(&dir, "plain", text).unwrap();
        let info = list_in(&dir).into_iter().find(|p| p.file == key).unwrap();
        assert_eq!(info.name, "plain");
        assert_eq!(info.author, "");
    }

    #[test]
    fn replacing_a_plan_keeps_one_backup_not_a_series() {
        let dir = scratch();
        import_in(&dir, "mine", &narrow_2m()).unwrap();
        let second = narrow_2m().replace("144.4", "145.0");
        import_in(&dir, "mine", &second).unwrap();
        let bak = library_in(&dir).join("mine.json.bak");
        assert_eq!(fs::read_to_string(&bak).unwrap(), narrow_2m(), "the first version");
        let third = narrow_2m().replace("144.4", "145.5");
        import_in(&dir, "mine", &third).unwrap();
        assert_eq!(fs::read_to_string(&bak).unwrap(), second, "overwritten, not numbered");
        let backups = fs::read_dir(library_in(&dir))
            .unwrap()
            .flatten()
            .filter(|e| e.file_name().to_string_lossy().contains(".bak"))
            .count();
        assert_eq!(backups, 1);
        assert!(fs::read_to_string(file_in(&dir, "mine")).unwrap().contains("145.5"));
    }

    #[test]
    fn a_refused_file_writes_nothing_and_touches_nothing() {
        let dir = scratch();
        import_in(&dir, "mine", &narrow_2m()).unwrap();
        let before = fs::read_to_string(file_in(&dir, "mine")).unwrap();

        // Not JSON, not a plan, a region with no bands, a row that is dropped
        // on load, and a file far too big.
        let refused = [
            "not json at all".to_string(),
            "[1, 2, 3]".to_string(),
            r#"{"region1":{"bands":[]},"region2":{"bands":[]},"region3":{"bands":[]}}"#.to_string(),
            narrow_2m().replace(r#""lo_mhz":144.0,"hi_mhz":144.4"#, r#""lo_mhz":148.0,"hi_mhz":144.0"#),
            format!(r#"{{"readme":["{}"]}}"#, "x".repeat(BAND_PLAN_MAX_BYTES)),
        ];
        for text in &refused {
            let e = import_in(&dir, "mine", text).unwrap_err();
            assert!(!e.is_empty());
            assert_eq!(fs::read_to_string(file_in(&dir, "mine")).unwrap(), before, "{e}");
            assert!(!library_in(&dir).join("mine.json.bak").exists(), "no backup for a refusal");
        }
        assert_eq!(list_in(&dir).iter().filter(|p| p.file == "mine").count(), 1);
    }

    #[test]
    fn a_name_that_is_a_path_stays_inside_the_library() {
        let dir = scratch();
        let key = import_in(&dir, "../../outside", &narrow_2m()).unwrap();
        assert_eq!(key, "outside");
        assert!(file_in(&dir, &key).starts_with(library_in(&dir)));
        assert!(!dir.join("outside.json").exists());
        assert!(!dir.parent().unwrap().join("outside.json").exists());
        assert!(import_in(&dir, "///", &narrow_2m()).is_err());
    }

    #[test]
    fn a_plan_can_be_deleted_and_a_missing_one_says_so() {
        let dir = scratch();
        import_in(&dir, "gone", &narrow_2m()).unwrap();
        delete_in(&dir, "gone").unwrap();
        assert!(read_in(&dir, "gone").is_err());
        assert!(delete_in(&dir, "gone").is_err());
    }

    #[test]
    fn the_running_plan_can_be_kept_under_a_name_and_read_back() {
        let dir = scratch();
        // The built-in tables become a file that can be chosen and downloaded.
        let key = save_current_in(&dir, "My IARU copy", &BandPlan::default()).unwrap();
        assert_eq!(key, "my-iaru-copy");
        let info = list_in(&dir).into_iter().find(|p| p.file == key).unwrap();
        assert_eq!(info.name, "My IARU copy", "the name given is what the list shows");
        assert!(read_in(&dir, &key).unwrap().is_default(), "same edges as what was running");

        // An edited plan keeps its country and author but takes the new name.
        let narrow = serde_json::from_str::<BandPlan>(&narrow_2m()).unwrap();
        let key = save_current_in(&dir, "Mine", &narrow).unwrap();
        let info = list_in(&dir).into_iter().find(|p| p.file == key).unwrap();
        assert_eq!((info.name.as_str(), info.country.as_str(), info.author.as_str()),
                   ("Mine", "Nowhere", "Me"));
        assert_eq!(read_in(&dir, &key).unwrap(), narrow);

        // Saving over a name keeps one backup of what was there.
        save_current_in(&dir, "Mine", &BandPlan::default()).unwrap();
        assert!(library_in(&dir).join("mine.json.bak").exists());
        assert!(read_in(&dir, "mine").unwrap().is_default());
        assert!(save_current_in(&dir, "///", &BandPlan::default()).is_err());
    }

    #[test]
    fn a_plan_is_exported_as_the_text_of_its_file() {
        let dir = scratch();
        import_in(&dir, "Mine", &narrow_2m()).unwrap();
        let (key, text) = export_text_in(&dir, "Mine.json").unwrap();
        assert_eq!(key, "mine");
        assert_eq!(text, narrow_2m(), "byte for byte what was kept");
        assert!(export_text_in(&dir, "nothing").is_err());
        // What was exported imports again under another name.
        assert!(import_in(&dir, "copy", &text).is_ok());
    }

    #[test]
    fn restoring_puts_back_a_deleted_plan_and_an_edited_one_with_a_backup() {
        let dir = scratch();
        ensure_library_in(&dir).unwrap();
        assert_eq!(restore_shipped_in(&dir).unwrap(), Vec::<String>::new(), "nothing to restore");

        delete_in(&dir, "bandplan-de-nrw").unwrap();
        assert_eq!(restore_shipped_in(&dir).unwrap(), vec!["bandplan-de-nrw".to_string()]);
        assert!(read_in(&dir, "bandplan-de-nrw").is_ok());

        // Edited by the operator: replaced, and their version is kept once.
        let edited = SHIPPED[0].1.replace("Germany (NRW)", "My own NRW");
        fs::write(file_in(&dir, "bandplan-de-nrw"), &edited).unwrap();
        assert_eq!(restore_shipped_in(&dir).unwrap(), vec!["bandplan-de-nrw".to_string()]);
        assert_eq!(fs::read_to_string(file_in(&dir, "bandplan-de-nrw")).unwrap(), SHIPPED[0].1);
        assert_eq!(
            fs::read_to_string(library_in(&dir).join("bandplan-de-nrw.json.bak")).unwrap(),
            edited
        );
        // The operator's own plans are not touched.
        import_in(&dir, "mine", &narrow_2m()).unwrap();
        restore_shipped_in(&dir).unwrap();
        assert!(read_in(&dir, "mine").is_ok());
    }

    #[test]
    fn the_plans_radios_run_on_their_own_are_found_in_every_radio_scope() {
        let dir = scratch();
        assert!(plans_used_by_radios_in(&dir).is_empty());
        fs::write(dir.join("radio.json"), r#"{"band_plan": "Mine"}"#).unwrap();
        fs::create_dir_all(dir.join("radio-2")).unwrap();
        fs::write(dir.join("radio-2").join("radio.json"), r#"{"band_plan": "other", "x": 1}"#).unwrap();
        fs::create_dir_all(dir.join("radio-3")).unwrap();
        fs::write(dir.join("radio-3").join("radio.json"), r#"{"band_plan": null}"#).unwrap();
        fs::create_dir_all(dir.join("radio-4")).unwrap();
        fs::write(dir.join("radio-4").join("radio.json"), "not json").unwrap();
        fs::create_dir_all(dir.join("recordings")).unwrap();
        fs::write(dir.join("recordings").join("radio.json"), r#"{"band_plan": "no"}"#).unwrap();
        assert_eq!(plans_used_by_radios_in(&dir), vec!["mine".to_string(), "other".to_string()]);
    }

    #[test]
    fn a_file_in_the_folder_that_is_not_a_plan_is_not_listed() {
        let dir = scratch();
        ensure_library_in(&dir).unwrap();
        fs::write(library_in(&dir).join("junk.json"), "oops").unwrap();
        fs::write(library_in(&dir).join("notes.txt"), "hello").unwrap();
        let files: Vec<_> = list_in(&dir).into_iter().map(|p| p.file).collect();
        assert_eq!(files, vec!["bandplan-de-nrw".to_string()]);
    }
}
