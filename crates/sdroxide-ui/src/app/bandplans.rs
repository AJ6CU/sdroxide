//! Band plans shipped with the program, and the chooser that loads one.
//!
//! The built-in IARU tables stay the default and the fallback. A preset is a
//! complete `bandplan.json` a listener can select instead; choosing one writes
//! it to the station's config directory and re-reads it, exactly as a
//! hand-edited file would be. The file stays the source of truth, so an
//! operator can still open it and change any row afterwards.
//!
//! Every preset must parse with this build's [`sdroxide_types::BandPlan`]. A
//! band a later version adds, but the preset does not name, is filled in from
//! the built-in tables by the loader, so an older contribution does not lose
//! new bands.

use sdroxide_types::BandPlan;

/// Germany (NRW), a hand-made plan from the SWL users group (#523): 630 m, a
/// detailed 2 m and 70 cm (repeaters, satellites, simplex, NL repeaters),
/// 6 m, QO-100, Freenet, PMR 446, the 11 m CB/freeband stretch, radiosondes,
/// sea radio, NOAA/Meteor weather satellites, DVB-T2, DAB+ and the
/// 694–2200 MHz utility bands.
pub const GERMANY_NRW: &str = include_str!("../../assets/bandplans/bandplan-de-nrw.json");

/// A band plan parsed from the text of a JSON file — a shipped preset or one
/// the operator imported — or `None` when it does not fit this build's plan.
pub fn parse_plan(json: &str) -> Option<BandPlan> {
    let plan: BandPlan = serde_json::from_str(json).ok()?;
    if !plan.problems().is_empty() {
        return None;
    }
    Some(plan)
}

/// Whether `plan` is exactly the Germany (NRW) preset, so the chooser can name
/// what is loaded rather than guessing from "not the built-in one".
pub fn is_germany_nrw(plan: &BandPlan) -> bool {
    let Some(preset) = parse_plan(GERMANY_NRW) else { return false };
    match (serde_json::to_string(plan), serde_json::to_string(&preset)) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

/// The selectable entries in the chooser, in the order the menu shows them.
///
/// The built-in IARU tables come first because they are what a fresh install
/// runs on and what everything falls back to; a shipped plan is an alternative
/// to them, never the opening position. "Station's own" is not an entry — it
/// is what the chooser says when the operator holds a hand-edited or imported
/// plan, and there is nothing to write for it.
pub const CHOICES: usize = 2;

/// Which chooser entry `plan` is on: 0 built-in, 1 Germany (NRW), 2 the
/// station's own.
pub fn choice_of(plan: &BandPlan) -> usize {
    if plan.is_default() {
        0
    } else if is_germany_nrw(plan) {
        1
    } else {
        2
    }
}

/// The plan a selectable entry writes, or `None` for an entry that is not one
/// of the shipped plans ("Station's own" has nothing to write).
pub fn plan_for_choice(choice: usize) -> Option<BandPlan> {
    match choice {
        0 => Some(BandPlan::default()),
        1 => parse_plan(GERMANY_NRW),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shipped preset must load, or the chooser would offer a plan the
    /// build cannot use.
    #[test]
    fn the_germany_preset_parses() {
        let plan = parse_plan(GERMANY_NRW).expect("the shipped preset must parse");
        assert!(!plan.is_default(), "the preset must differ from the built-in tables");
        assert!(plan.problems().is_empty(), "{:?}", plan.problems());
    }

    /// The chooser opens on the built-in tables and lists the German plan
    /// after them. The order is the contract: an entry added above the
    /// built-in one puts a national plan on a fresh install.
    #[test]
    fn the_built_in_plan_is_the_first_choice() {
        assert_eq!(CHOICES, 2, "only the built-in and the shipped plans are entries");
        assert_eq!(choice_of(&BandPlan::default()), 0, "a fresh install reads built-in");
        assert!(
            plan_for_choice(0).is_some_and(|p| p.is_default()),
            "the first entry must write the built-in tables"
        );
        let german = parse_plan(GERMANY_NRW).expect("preset");
        assert_eq!(choice_of(&german), 1, "the German plan is second, after the IARU tables");
        assert_eq!(plan_for_choice(1), Some(german));
    }

    /// A plan the station edited is named as its own and writes nothing: the
    /// chooser must not offer to replace the operator's file with a preset.
    #[test]
    fn an_edited_plan_is_the_station_s_own() {
        let text = BandPlan::default()
            .to_json_document()
            .replace(r#""band": "M20", "lo_mhz": 14.0"#, r#""band": "M20", "lo_mhz": 14.01"#);
        let own: BandPlan = serde_json::from_str(&text).expect("an edited plan still parses");
        assert!(!own.is_default(), "the edit has to land for this test to mean anything");
        assert_eq!(choice_of(&own), 2);
        assert_eq!(plan_for_choice(2), None);
    }

    /// The 11 m stretch is the freeband/CB block, named as such — not a
    /// personal remark about pirates.
    #[test]
    fn the_eleven_metre_segment_is_labeled_cb() {
        let plan = parse_plan(GERMANY_NRW).expect("preset");
        let labeled: Vec<_> = plan
            .region(sdroxide_types::Region::R1)
            .segments()
            .iter()
            .filter(|s| s.label.is_some_and(|l| l.contains("CB/Freeband")))
            .collect();
        assert_eq!(labeled.len(), 1, "the 11 m segment must be the CB/Freeband one");
        let s = labeled[0];
        assert!(s.lo < 27_500_000.0 && s.hi > 27_400_000.0);
        assert!(
            plan.region(sdroxide_types::Region::R1)
                .segments()
                .iter()
                .all(|s| !s.label.is_some_and(|l| l.to_lowercase().contains("pirate"))),
            "the personal remark must be gone"
        );
    }

    /// The shipped NRW plan keeps medium wave where it belongs (#546).
    #[test]
    fn the_shipped_nrw_plan_keeps_medium_wave() {
        let plan = parse_plan(GERMANY_NRW).expect("the shipped preset must parse");
        assert_eq!(
            plan.region(sdroxide_types::Region::R1).containing(1_000_000.0),
            sdroxide_types::Band::Mw,
            "MW listening must survive the shipped plan"
        );
    }
}
