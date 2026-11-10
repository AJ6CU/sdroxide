//! The WSJT-CB (11 m) callsign grammar.
//!
//! WSJT-CB ([`vash909/WSJT-CB`](https://github.com/vash909/WSJT-CB)) widened
//! the FT8 family to the citizens' band, where an identifier is
//! `N{1,3}L{1,2}N{1,3}` — a numeric country prefix, one or two letters, and a
//! numeric unit number — rather than the amateur `[prefix][digit][letter-suffix]`
//! shape. This is the grammar [`is_cb_callsign`] implements, and the fork's
//! decoder uses it to recognise those identifiers; on air they travel as free
//! text or a hashed non-standard call (see `sdroxide-digi`).
//!
//! [`is_cb_callsign_wide`] is an **experimental superset** for the shape the
//! growing 11 m community uses but WSJT-CB does not yet accept:
//! `N{1,3}L{1,3}N{1,4}`. It is opt-in and off by default — see that function.
//!
//! It began life inside the fork's `mfsk-core` pin, offered upstream as
//! `jl1nie/mfsk-core#373`. That maintainer's call (2026-09-20) was that a
//! *dialect's* grammar is application policy rather than something the codec
//! should carry, so it lives here now. The signature is deliberately the one
//! the field-based decode hook wants —
//! `DecodeRequest::also_accept(|m| m.callsigns().all(is_cb_callsign))`, their
//! #386 — so that when that lands, only the call site changes, not this. See
//! `AGENTS.md`.

/// The longest 11 m identifier the wire can carry, from `wsjt77::pack77_type4`:
/// a non-standard call travels as a 58-bit base-38 number, and `38^11` is under
/// `2^58`, so eleven characters is the ceiling and it is exact.
pub const CB_CALL_MAX_LEN: usize = 11;

/// Check whether a string is a CB-style callsign in the WSJT-CB sense.
///
/// WSJT-CB (vash909/WSJT-CB) widened the FT8 family to the 11 m CB world,
/// where an identifier is `N{1,3}L{1,2}N{1,3}` — a numeric country prefix,
/// one or two letters, and a numeric unit number — rather than the amateur
/// `[prefix][digit][letter-suffix]` shape. Two extensions ride on top:
///
/// * a **four-digit unit number is allowed only behind a one-digit prefix**
///   (`1TT1000` passes; `26AT1000` and `111TT1000` do not),
/// * a trailing **compound form** `N{1,3}L{1,2}/L{2}` (`999ZZ/ZZ`), whose base
///   carries no unit number of its own, and
/// * a trailing **modifier** — `/P`, `/MM`, `/QRP`, an event marker like `/F1`
///   — on any legal closed-form call (`19DC373/P`). This last one is WSJT-CB's
///   own behaviour: it *widens* the standard `is_callsign` rather than
///   replacing it, so portable-style suffixes ride on a CB base call.
///
/// Patterns with a three-letter middle part, no numeric prefix, no numeric
/// unit, a four-digit unit behind a multi-digit prefix, a second slash, or a
/// slash suffix that is neither `/LL` nor a modifier are rejected — mirroring
/// WSJT-CB's `Radio::is_cb_callsign` (25-case table in the tests).
pub fn is_cb_callsign(call: &str) -> bool {
    cb_shape(call, 2, true)
}

/// The **experimental** wider CB grammar: `N{1,3}L{1,3}N{1,4}` and the slash
/// form `N{1,3}L{1,3}/L{2}`.
///
/// WSJT-CB's own grammar is [`is_cb_callsign`] — `N{1,3}L{1,2}N{1,3}`, with a
/// four-digit unit only behind a one-digit prefix. The 11 m community has
/// outgrown it: three-letter middle groups and four-digit unit numbers are in
/// use, and a call of either shape is dropped by the codec's gate unless this
/// recognises it.
///
/// It is a **superset**: everything [`is_cb_callsign`] accepts is accepted here
/// too, and the four-digit unit is allowed with any prefix (the coupling is
/// WSJT-CB's, not the band's). It is experimental because **WSJT-CB itself does
/// not accept this shape** — a station running WSJT-CB will not resolve a call
/// that needs it, so it is a receive-side widening and an opt-in. The wire
/// format is unchanged: the longest call here is ten characters, inside the
/// eleven a Type-4 nonstandard callsign can carry (`pack77_type4`).
///
/// A **modifier suffix** (`19DC373/P`) is *not* part of this experiment. It is
/// plain WSJT-CB behaviour that both grammars accept, so switching this on
/// buys nothing on that front — see [`is_cb_callsign`].
pub fn is_cb_callsign_wide(call: &str) -> bool {
    cb_shape(call, 3, false)
}

/// The grammar the operator's switch selects: WSJT-CB's own
/// ([`is_cb_callsign`]) or the experimental wider one
/// ([`is_cb_callsign_wide`]).
///
/// One place that makes the choice, so the decode gate, the pack ladder and the
/// QSO machine cannot end up running different grammars.
pub fn is_cb_callsign_with(call: &str, wide: bool) -> bool {
    if wide { is_cb_callsign_wide(call) } else { is_cb_callsign(call) }
}

/// The shared body of the two grammars: `max_letters` is the widest middle
/// group, and `unit4_coupled` keeps WSJT-CB's "four-digit unit only behind a
/// one-digit prefix" rule (the strict grammar) rather than dropping it.
///
/// A trailing modifier is stripped first — see [`is_cb_callsign_with`]. The
/// compound `N{1,3}L{1,2}/L{2}` form is a *base* shape, not a modifier, so it
/// is matched by the slash branch below and never reaches the modifier test.
///
/// A trailing modifier is a *third* accepted form, on top of the closed shape
/// and the compound `N{1,3}L{1,2}/L{2}`: see [`is_cb_modifier`].
fn cb_shape(call: &str, max_letters: usize, unit4_coupled: bool) -> bool {
    let b = call.trim().as_bytes();
    // The wire bound, and the reason the modifier length is not arbitrary: a
    // non-standard call is carried as a 58-bit base-38 number, which is under
    // 2^58 for eleven characters (`wsjt77::pack77_type4`). A longer identifier
    // is not a call this wire can carry, whatever its letters look like.
    if b.is_empty() || b.len() > CB_CALL_MAX_LEN {
        return false;
    }
    let Some(sl) = b.iter().position(|&c| c == b'/') else {
        return cb_prefix_digits_letters_digits(b, max_letters, unit4_coupled);
    };
    // At most one slash: `1/AT100` is not a call.
    if b[sl + 1..].contains(&b'/') {
        return false;
    }
    let (stem, sfx) = (&b[..sl], &b[sl + 1..]);
    // The compound form, where the suffix *is* the call: `999ZZ/ZZ`.
    if sfx.len() == 2
        && sfx.iter().all(|c| c.is_ascii_uppercase())
        && cb_prefix_digits_letters(stem, max_letters)
    {
        return true;
    }
    // A portable/special-activity modifier on a legal closed-form call:
    // `19DC373/P`, `19DC373/MM`, `19DC373/QRP`, `19DC373/F1`.
    is_cb_modifier(sfx) && cb_prefix_digits_letters_digits(stem, max_letters, unit4_coupled)
}

/// Whether a slash suffix is a portable/special-activity modifier.
///
/// WSJT-CB does not *replace* the standard callsign rules with the CB pattern,
/// it **widens** them — the README says it "extended `Radio::is_callsign` so CB
/// calls are treated as valid callsigns". So the portable-style suffixes the
/// standard rules already know (`/P`, `/MM`, `/QRP`, `/F1`, an event marker)
/// ride on a CB base call unchanged, and WSJT-CB decodes them.
///
/// The fork's gate was built as a *union* of two narrow predicates instead
/// (`is_valid_callsign` || `is_cb_callsign`), so it rejected every one of
/// these: the fork could hear the station and then could not answer it. That is
/// the bug this form fixes.
///
/// The set is deliberately the general `1..=4` of `A-Z0-9` rather than a
/// transcription of the ham suffix list, which is open-ended for event
/// callouts and is not ours to pin. The real bound is the wire: the whole
/// identifier must still fit the 58-bit base-38 field — 11 characters — which
/// `pack77_type4` enforces on the encode side.
fn is_cb_modifier(sfx: &[u8]) -> bool {
    !sfx.is_empty()
        && sfx.len() <= 4
        && sfx.iter().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
}

/// The split-form base `N{1,3}L{1,max}`.
fn cb_prefix_digits_letters(b: &[u8], max_letters: usize) -> bool {
    let digits = b.iter().take_while(|&&c| c.is_ascii_digit()).count();
    let letters = b[digits..].iter().take_while(|&&c| c.is_ascii_uppercase()).count();
    (1..=3).contains(&digits) && (1..=max_letters).contains(&letters) && digits + letters == b.len()
}

/// Whether a string is meant to be an **11 m / citizens' band** identifier.
///
/// An 11 m call opens with the country number (`19…`, `26…`, `999…`) where an
/// amateur call opens with a letter, so "starts with a digit" is the tell the
/// settings field uses to decide whether the 11 m length rule applies. It is
/// deliberately loose: it is a *warning* trigger, not a gate, and a call this
/// rejects is still judged on its own merits everywhere that matters.
pub fn looks_like_cb_call(call: &str) -> bool {
    call.trim().as_bytes().first().is_some_and(u8::is_ascii_digit)
}

/// Why a typed 11 m callsign cannot go out, if it cannot.
///
/// The operator picks an **activation callsign** here, and the 58-bit
/// base-38 field FT8 uses for a non-standard call is **11 characters for the
/// whole identifier, suffix included** — `38^11 < 2^58`, so eleven is the exact
/// ceiling, and `pack77_type4` enforces it on the way out. Nothing in the
/// program can send a longer one; a station that announces `19TST1001/QRP` (13)
/// is announcing something that will not fit.
///
/// `None` when the call is empty, is not CB-shaped, or fits. The returned
/// length is the operator's own, so the message can quote it.
pub fn cb_call_length_problem(call: &str) -> Option<usize> {
    let c = call.trim();
    if !looks_like_cb_call(c) {
        return None;
    }
    (c.len() > CB_CALL_MAX_LEN).then_some(c.len())
}

/// The closed form `N{1,3}L{1,max}N{1,4}`, with the four-digit-unit caveat when
/// `unit4_coupled` is set.
fn cb_prefix_digits_letters_digits(b: &[u8], max_letters: usize, unit4_coupled: bool) -> bool {
    let digits = b.iter().take_while(|&&c| c.is_ascii_digit()).count();
    let letters = b[digits..].iter().take_while(|&&c| c.is_ascii_uppercase()).count();
    let units = b[digits + letters..].iter().take_while(|&&c| c.is_ascii_digit()).count();
    if !(1..=3).contains(&digits) || !(1..=max_letters).contains(&letters) {
        return false;
    }
    if digits + letters + units != b.len() {
        return false;
    }
    if !(1..=4).contains(&units) {
        return false;
    }
    // A four-digit unit number only behind a one-digit prefix (strict grammar).
    !unit4_coupled || units < 4 || digits == 1
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The WSJT-CB acceptance table, mirrored exactly from
    /// vash909/WSJT-CB's README so the two validators agree case for case.
    const WSJT_CB_TABLE: &[(&str, bool)] = &[
        // (callsign, accepted by WSJT-CB)
        ("1A1", true),        // 1-digit prefix, 1 letter, 1-digit suffix
        ("1TT1", true),       // 1-digit prefix, 2 letters, 1-digit suffix
        ("1TT01", true),      // 1-digit prefix, 2 letters, 2-digit suffix
        ("1TT001", true),     // 1-digit prefix, 2 letters, 3-digit suffix
        ("1TT1000", true),    // 4-digit suffix, allowed with a 1-digit prefix
        ("1AT1000", true),    // 4-digit suffix, allowed with a 1-digit prefix
        ("11TT1", true),      // 2-digit prefix, 2 letters, 1-digit suffix
        ("111TT11", true),    // 3-digit prefix, 2 letters, 2-digit suffix
        ("111TT999", true),   // 3-digit prefix, 2 letters, 3-digit suffix
        ("26AT715", true),    // 2-digit prefix, 2 letters, 3-digit suffix
        ("999ZZ/ZZ", true),   // slash form: 3-digit prefix, 2 letters, /ZZ
        ("26AT1000", false),  // 4-digit suffix is not allowed with a 2-digit prefix
        ("111TT1000", false), // 4-digit suffix is not allowed with a 3-digit prefix
        ("99Z9999", false),   // 4-digit suffix is not allowed with a 2-digit prefix
        ("1TT10000", false),  // suffix longer than 4 digits is not allowed
        ("1TT", false),       // missing numeric suffix
        ("1AT", false),       // missing numeric suffix
        ("AT1000", false),    // missing numeric prefix
        ("AAA", false),       // letters only; numeric prefix and suffix are both missing
        ("123", false),       // digits only; the alphabetic middle part is missing
        ("12A", false),       // missing numeric suffix
        ("12ABC1", false),    // 3-letter middle part is not allowed
        ("ABC123", false),    // missing numeric prefix
        ("1/AT100", false),   // slash is only allowed as a final /LL suffix
        ("999ZZ", false),     // closed form still needs its numeric unit
    ];

    #[test]
    fn cb_callsigns_match_wsjt_cb_rejection_table() {
        for &(call, accepted) in WSJT_CB_TABLE {
            assert_eq!(is_cb_callsign(call), accepted, "is_cb_callsign({call})");
        }
    }

    /// A modifier rides on a call the strict grammar already accepts, because
    /// WSJT-CB *widens* the standard `is_callsign` rather than replacing it.
    /// The fork's union-of-two gate rejected all of these, so it could decode a
    /// station and then not answer it.
    const MODIFIER_TABLE: &[(&str, bool)] = &[
        ("19DC373/P", true),     // portable — the common activation case
        ("19DC373/MM", true),    // maritime mobile
        ("19DC373/F1", true),    // event marker, letter + digit
        ("19DC373/QRP", true),   // low power, three characters
        ("26AT715/P", true),     // a shorter base takes one too
        ("1AT1000/QRP", true),   // …including the coupled four-digit unit form
        ("19DC373/AM", true),    // two letters are fine as a modifier
        ("999ZZ/ZZ", true),      // the compound form, not a modifier, still passes
        ("19DC373/Q", true),     // a single character is a modifier
        ("19DC373/QRPX", false), // five characters is not
        ("19DC373/p", false),    // lower case is not a suffix
        ("19DC373/-P", false),   // punctuation is not a suffix
        ("/P", false),           // no base call to attach it to
        ("19DC373/", false),     // empty suffix
        ("ABC373/P", false),     // the base still has to be a legal 11 m call
        ("999ZZ/P", false),      // …so the compound form's stem is not a base
        ("1/AT100/P", false),    // two slashes is not a call
    ];

    #[test]
    fn a_modifier_rides_on_a_legal_cb_call() {
        for &(call, accepted) in MODIFIER_TABLE {
            assert_eq!(is_cb_callsign(call), accepted, "is_cb_callsign({call})");
        }
    }

    /// The modifier is WSJT-CB's own behaviour, not the experimental widening,
    /// so it must not depend on the operator's toggle — a station that is
    /// reachable on the air cannot depend on an off-by-default switch.
    #[test]
    fn a_modifier_does_not_need_the_wide_grammar() {
        for &(call, accepted) in MODIFIER_TABLE {
            assert_eq!(is_cb_callsign_wide(call), accepted, "is_cb_callsign_wide({call})");
            assert_eq!(is_cb_callsign_with(call, false), accepted, "strict switch ({call})");
            assert_eq!(is_cb_callsign_with(call, true), accepted, "wide switch ({call})");
        }
    }

    /// The whole identifier still has to fit the 58-bit base-38 field that
    /// `pack77_type4` encodes into, so the longest legal modifier call is
    /// exactly the eleven characters that field allows.
    #[test]
    fn a_modified_call_still_fits_the_type4_field() {
        assert!(is_cb_callsign("19DC373/QRP"));
        assert_eq!("19DC373/QRP".len(), 11);
        // One more character is beyond the field, so it is not a call.
        assert!(!is_cb_callsign("19DC373/QRPP"));
    }

    #[test]
    fn surrounding_whitespace_is_ignored() {
        assert!(is_cb_callsign("  26AT715 "));
        assert!(is_cb_callsign("  19DC373/P "));
        assert!(!is_cb_callsign("   "));
        assert!(!is_cb_callsign(""));
    }

    /// The shape the wider grammar adds, and the edges it still refuses. The
    /// strict table above has the matching rejections (`12ABC1`, `26AT1000`,
    /// `111TT1000`) so the two grammars can be read as a pair.
    const WIDE_TABLE: &[(&str, bool)] = &[
        ("26ABC715", true),   // 3-letter middle, 2-digit prefix — the new shape
        ("1ABC1", true),      // 3-letter middle, 1-digit prefix and unit
        ("26ABC1000", true),  // 4-digit unit with a 2-digit prefix (coupling dropped)
        ("111ABC1000", true), // 4-digit unit with a 3-digit prefix
        ("26ABCD715", false), // 4-letter middle is past the grammar
        ("1ABC10000", false), // 5-digit unit is past the grammar
        ("12ABC", false),     // still needs a numeric unit
        ("ABC123", false),    // still needs a numeric prefix
        ("26ABC/ZZ", true),   // slash middle widened too
        ("26ABCD/ZZ", false), // …but not past three letters
        ("26ABC/ZZZ", false), // the slash suffix stays exactly two letters
    ];

    #[test]
    fn the_wide_grammar_adds_the_three_letter_shape() {
        for &(call, accepted) in WIDE_TABLE {
            assert_eq!(is_cb_callsign_wide(call), accepted, "is_cb_callsign_wide({call})");
        }
        // A superset: every strict call is a wide one, and the strict table's
        // own rejections that the wide shape now admits are exactly the new
        // cases above — never a call the strict grammar calls malformed.
        for &(call, accepted) in WSJT_CB_TABLE {
            if accepted {
                assert!(is_cb_callsign_wide(call), "wide must accept strict call {call}");
            }
        }
    }

    /// The toggle default: the strict grammar is what runs unless the operator
    /// opts in, so nothing here can be the only thing keeping a call out.
    #[test]
    fn the_strict_grammar_is_untouched_by_the_wide_one() {
        assert!(!is_cb_callsign("12ABC1"));
        assert!(is_cb_callsign_wide("12ABC1"));
        assert!(!is_cb_callsign("26AT1000"));
        assert!(is_cb_callsign_wide("26AT1000"));
    }

    /// The bench cases, as the operator meets them: an activation callsign is
    /// typed, and a `/zzz` suffix is **part of the 11 characters**, not extra.
    #[test]
    fn the_eleven_character_ceiling_counts_the_suffix() {
        // Fits — with and without a modifier.
        assert_eq!(cb_call_length_problem("19DC373"), None);
        assert_eq!(cb_call_length_problem("19DC373/P"), None);
        assert_eq!(cb_call_length_problem("19DC3733/P"), None);
        // Exactly at the ceiling, and only just.
        assert_eq!(cb_call_length_problem("19DC373/QRP"), None);
        assert_eq!("19DC373/QRP".len(), CB_CALL_MAX_LEN);
        // One character past it, which is the whole point of warning about it.
        assert_eq!(cb_call_length_problem("19DC373/QRPP"), Some(12));
        // The suffix is what tips a long base over: the 9-character base fits,
        // the same base with `/QRP` is 13 and cannot be sent at all.
        assert_eq!(cb_call_length_problem("19TST1001"), None);
        assert_eq!("19TST1001".len(), 9);
        assert_eq!(cb_call_length_problem("19TST1001/QRP"), Some(13));
    }

    /// The warning is CB-only, as it must be: an amateur callsign is not held
    /// to the 11 m field, and neither is an empty box.
    #[test]
    fn the_length_warning_is_citizens_band_only() {
        assert!(!looks_like_cb_call("K1ABC"));
        assert!(!looks_like_cb_call(""));
        assert!(!looks_like_cb_call("  "));
        assert!(looks_like_cb_call("19DC373"));
        assert!(looks_like_cb_call("19DC373/P"));
        // A long *amateur* call is not this warning's business.
        assert_eq!(cb_call_length_problem("DL1ABCDEFGH"), None);
        assert_eq!(cb_call_length_problem(""), None);
    }
}
