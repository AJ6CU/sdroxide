//! A real band list, converted: the French one, a long list in hertz with every
//! habit the lists have — services filed under `amateur`, rows that overlap,
//! a band listed twice.

use sdroxide_types::{Band, BandPlan, Region, convert_band_list, looks_like_band_list};

const FRANCE: &str = include_str!("fixtures/band-list-france.json");

#[test]
fn the_french_list_becomes_a_plan_for_region_1_and_leaves_the_lockout_alone() {
    assert!(looks_like_band_list(FRANCE));
    let c = convert_band_list(FRANCE, "F4NRM").expect("the list converts");
    assert_eq!(c.regions, vec![Region::R1]);
    assert!(c.dropped.is_empty(), "every row of this list is a range: {:?}", c.dropped);

    let plan: BandPlan = serde_json::from_str(&c.text).expect("the result is a band plan file");
    assert!(plan.problems().is_empty(), "no overlap, no bad row: {:?}", plan.problems());
    assert_eq!(plan.meta().name, "France");
    assert_eq!(plan.meta().country, "France");
    assert_eq!(plan.meta().author, "F4NRM");

    // The lockout is the built-in one, in every region.
    let stock = BandPlan::default();
    for region in Region::ALL {
        for band in Band::ALL {
            assert_eq!(plan.region(region).edges(band), stock.region(region).edges(band));
        }
    }
    // The other regions are the built-in tables whole.
    assert_eq!(plan.region(Region::R2), stock.region(Region::R2));
    assert_eq!(plan.region(Region::R3), stock.region(Region::R3));

    // Region 1 draws the list: the 40 m amateur row is there under its name,
    // and nothing overlaps another.
    let segs = plan.region(Region::R1).segments();
    assert!(segs.len() > 60, "{} segments", segs.len());
    assert!(
        segs.iter().any(|s| s.lo == 7_000_000.0 && s.hi == 7_200_000.0
            && s.label == Some("40m - Radioamateur")),
        "the 40 m row"
    );
    for w in segs.windows(2) {
        assert!(w[0].hi <= w[1].lo, "overlap {:?} {:?}", w[0], w[1]);
    }
    // CB is drawn as what it is called, and the list's "5,650GHz" duplicate of
    // the 6 cm row did not become a second row.
    assert!(segs.iter().any(|s| s.label == Some("11m - CB")));
    assert_eq!(segs.iter().filter(|s| s.lo == 5_650e6).count(), 1);

    // The text is a file of our own: reading it and writing it back changes nothing.
    let again = plan.to_json_document();
    let back: BandPlan = serde_json::from_str(&again).unwrap();
    assert_eq!(back, plan);
    assert_eq!(back.fingerprint(), plan.fingerprint());
}

#[test]
fn a_list_that_is_a_plan_file_is_left_alone() {
    let own = BandPlan::default().to_json_document();
    assert!(!looks_like_band_list(&own), "a plan of our own is not converted");
}
