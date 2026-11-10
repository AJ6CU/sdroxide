//! Which frequency a replacement front end should be opened at.
//!
//! One decision, in one place, because three different paths open a radio —
//! the program starting up, the operator pressing the link button, and the
//! background reconnect — and the bug this answers for all three at once was
//! that only the first of them knew about it.
//!
//! The reported case (#72) is a start frequency the hardware cannot reach: an
//! I.F. offset saved as a frequency, so the tuner is asked for 6 GHz, refuses,
//! and the operator has to correct the number by hand before anything opens.
//! The one thing worth doing about that is to open at a frequency the radio has
//! already proved it can tune.

/// Whether `hz` is a frequency this program may hand to an open attempt: a
/// real, positive number of hertz. A zero, a negative and a damaged file's NaN
/// are all "no frequency was ever recorded", not a place to tune to.
fn is_frequency(hz: f64) -> bool {
    hz.is_finite() && hz > 0.0
}

/// The frequency to open at instead of `asked_hz`, or `None` to open where
/// asked.
///
/// `refused_hz` is the centre the last attempt at *this* radio failed on, and
/// `last_good_hz` a frequency the radio is known to have tuned. A substitution
/// happens only when the radio has refused this exact centre; what is offered
/// in its place is, in order:
///
/// - the recorded frequency, when it is a real one that is **not** the one being
///   refused — a fallback to the number that just failed is the retry that has
///   already failed, which is how a station ended up correcting the same
///   unreachable start frequency by hand every single morning; and failing
///   that,
/// - the first of `memories` that is a real frequency.
///
/// **The memories are the operator's own list, and they are the only honest
/// source left.** A record that is the refused number means the last run never
/// tuned anything — which is exactly what happens to a station whose start
/// frequency could not be reached, since the stand-in it runs on answers every
/// tune with "done". Guessing a band centre there would move a satellite
/// listener's dial to a frequency nobody chose; their own stored channels are
/// frequencies they put there on purpose.
///
/// The one-hertz tolerance is a rounding difference, not a difference: an
/// open at 145.0000004 MHz after a refusal at 145.0 MHz is the same request.
pub fn fallback_open_hz(
    asked_hz: f64,
    refused_hz: Option<f64>,
    last_good_hz: f64,
    memories: &[f64],
) -> Option<f64> {
    let refused_it = refused_hz.is_some_and(|r| is_frequency(r) && (r - asked_hz).abs() <= 1.0);
    if !refused_it {
        return None;
    }
    let recorded = is_frequency(last_good_hz) && (last_good_hz - asked_hz).abs() > 1.0;
    if recorded {
        return Some(last_good_hz);
    }
    memories.iter().copied().find(|&hz| is_frequency(hz))
}

/// Where the "last frequency that actually tuned" record starts out for a
/// front end that never opened.
///
/// `opened_at_hz` is the centre the source reports — which, for the stand-in the
/// program substitutes when the configured interface will not open, is the
/// frequency that was *asked for* and never reached anything. `remembered_hz` is
/// what the last session recorded.
///
/// A stand-in must not stamp its asked-for centre as a frequency the hardware
/// took. Doing so writes the impossible value into the record as "last good",
/// and from then on every start reads the fallback as the same wrong number it
/// was trying to escape (#72, reported back on 2.0.12). So a stand-in starts
/// from what was genuinely recorded, which may be nothing — and nothing is an
/// honest answer here, where a number would be a lie the operator tunes by.
pub fn seeded_last_good_hz(source_opened: bool, opened_at_hz: f64, remembered_hz: f64) -> f64 {
    if source_opened && is_frequency(opened_at_hz) {
        opened_at_hz
    } else if is_frequency(remembered_hz) {
        remembered_hz
    } else {
        0.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const IMPOSSIBLE: f64 = 6_000_000_000.0;
    const QO100: f64 = 144_150_000.0;

    #[test]
    fn a_refused_centre_falls_back_to_a_frequency_that_tuned() {
        assert_eq!(fallback_open_hz(IMPOSSIBLE, Some(IMPOSSIBLE), QO100, &[]), Some(QO100));
    }

    #[test]
    fn a_poisoned_record_falls_back_to_the_operators_own_channel_list() {
        // His session held the unreachable frequency as the record, so the
        // fallback had nothing but the number that had just failed. His QO-100
        // memories were untouched by all of this, and they are frequencies he
        // put there on purpose.
        assert_eq!(
            fallback_open_hz(IMPOSSIBLE, Some(IMPOSSIBLE), IMPOSSIBLE, &[QO100, 435_000_000.0]),
            Some(QO100),
            "with the record useless, the first real stored channel is the honest answer"
        );
        assert_eq!(
            fallback_open_hz(IMPOSSIBLE, Some(IMPOSSIBLE), 0.0, &[0.0, QO100]),
            Some(QO100),
            "an unusable channel is skipped rather than opened on"
        );
        assert_eq!(
            fallback_open_hz(IMPOSSIBLE, Some(IMPOSSIBLE), IMPOSSIBLE, &[]),
            None,
            "no record and no channels means declining, not guessing"
        );
    }

    #[test]
    fn an_ordinary_dial_is_left_where_it_is() {
        assert_eq!(fallback_open_hz(QO100, Some(IMPOSSIBLE), IMPOSSIBLE, &[]), None);
        assert_eq!(fallback_open_hz(QO100, None, IMPOSSIBLE, &[]), None);
    }

    #[test]
    fn there_is_no_fallback_to_a_frequency_that_was_never_tuned() {
        // The case the report actually hit: the record held the very frequency
        // that had been refused, so the "fallback" was the same doomed retry.
        assert_eq!(fallback_open_hz(IMPOSSIBLE, Some(IMPOSSIBLE), IMPOSSIBLE, &[]), None);
        for never in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert_eq!(
                fallback_open_hz(IMPOSSIBLE, Some(IMPOSSIBLE), never, &[]),
                None,
                "a record holding {never} is not a frequency to open at"
            );
        }
    }

    #[test]
    fn a_refusal_of_some_other_frequency_does_not_move_this_dial() {
        assert_eq!(fallback_open_hz(QO100, Some(435_000_000.0), IMPOSSIBLE, &[]), None);
    }

    #[test]
    fn a_rounding_difference_is_the_same_request() {
        // The dial the operator sees and the centre the radio refused are the
        // same request to within a rounding difference, so the fallback still
        // applies — otherwise a refracted dial would read as "nobody refused
        // this one" and the open would be attempted a second time unchanged.
        assert_eq!(fallback_open_hz(QO100 + 0.4, Some(QO100), IMPOSSIBLE, &[]), Some(IMPOSSIBLE));
        // …and a real difference is not the same request.
        assert_eq!(fallback_open_hz(QO100, Some(QO100 + 400.0), IMPOSSIBLE, &[]), None);
    }

    #[test]
    fn a_stand_in_does_not_stamp_the_frequency_it_was_asked_for() {
        assert_eq!(
            seeded_last_good_hz(false, IMPOSSIBLE, QO100),
            QO100,
            "the stand-in never reached {IMPOSSIBLE} MHz, so that is not a frequency that tuned"
        );
    }

    #[test]
    fn a_stand_in_with_nothing_recorded_says_so_rather_than_guessing() {
        assert_eq!(seeded_last_good_hz(false, IMPOSSIBLE, 0.0), 0.0);
    }

    #[test]
    fn a_source_that_opened_keeps_its_own_centre() {
        assert_eq!(seeded_last_good_hz(true, QO100, IMPOSSIBLE), QO100);
    }

    #[test]
    fn a_first_run_of_a_radio_that_opens_at_nothing_useful_is_not_recorded_as_good() {
        // A stand-in asked for a frequency its hardware cannot reach, with
        // nothing on record: the honest answer is "nothing has tuned yet", and
        // that is what makes the next attempt say so rather than point the dial
        // at a number that was never received.
        assert_eq!(seeded_last_good_hz(false, IMPOSSIBLE, 0.0), 0.0);
    }
}
