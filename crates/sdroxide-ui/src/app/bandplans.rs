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

/// A preset parsed into this build's plan, or `None` when it does not fit.
pub fn preset_plan(json: &str) -> Option<BandPlan> {
    serde_json::from_str::<BandPlan>(json).ok()
}

/// Whether `plan` is exactly the Germany (NRW) preset, so the chooser can name
/// what is loaded rather than guessing from "not the built-in one".
pub fn is_germany_nrw(plan: &BandPlan) -> bool {
    let Some(preset) = preset_plan(GERMANY_NRW) else { return false };
    match (serde_json::to_string(plan), serde_json::to_string(&preset)) {
        (Ok(a), Ok(b)) => a == b,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shipped preset must load, or the chooser would offer a plan the
    /// build cannot use.
    #[test]
    fn the_germany_preset_parses() {
        let plan = preset_plan(GERMANY_NRW).expect("the shipped preset must parse");
        assert!(!plan.is_default(), "the preset must differ from the built-in tables");
        assert!(plan.problems().is_empty(), "{:?}", plan.problems());
    }

    /// The 11 m stretch is the freeband/CB block, named as such — not a
    /// personal remark about pirates.
    #[test]
    fn the_eleven_metre_segment_is_labeled_cb() {
        let plan = preset_plan(GERMANY_NRW).expect("preset");
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
}
