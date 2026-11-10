//! Pronunciations we do not trust a dictionary to get right.
//!
//! Consulted before the grapheme-to-phoneme dictionary. Two kinds of entry
//! earn a place here and nothing else does:
//!
//! - words CMUdict does not have, which would otherwise fall through to letter
//!   rules — the NATO alphabet, `wefax`, `sdroxide`;
//! - words CMUdict has but pronounces as something else — `juliett` is not
//!   "Juliet", and an operator says "ritty", not "R T T Y".
//!
//! The values are IPA in the eSpeak inventory the voice was trained on, using
//! `ˈ` for primary stress and `ˌ` for secondary. Keep this table short: past
//! about sixty entries it stops being a list of exceptions and becomes a second
//! dictionary that nobody maintains.

/// IPA for `word`, which must already be lowercase and letters-only.
pub fn lookup(word: &str) -> Option<&'static str> {
    let lower = word.to_ascii_lowercase();
    LEXICON.iter().find(|(w, _)| *w == lower).map(|(_, ipa)| *ipa)
}

/// Whether the lexicon has an entry, without building the lowercase copy.
pub fn has(word: &str) -> bool {
    LEXICON.iter().any(|(w, _)| w.eq_ignore_ascii_case(word))
}

/// The table. Alphabetical, so a duplicate is easy to spot.
pub const LEXICON: &[(&str, &str)] = &[
    // ── NATO phonetic alphabet ──────────────────────────────────────────
    // CMUdict has some of these as ordinary words with the wrong sense
    // ("charlie", "victor", "mike") and lacks others entirely. They are the
    // single most important thing on this list to get right: a callsign is
    // the one token a misheard letter makes useless.
    ("alpha", "ˈalfɐ"),
    ("bravo", "bɹˈɑːvoʊ"),
    ("charlie", "tʃˈɑːlɪ"),
    ("delta", "dˈɛltɐ"),
    ("echo", "ˈɛkoʊ"),
    ("foxtrot", "fˈɒkstɹɒt"),
    ("golf", "ɡˈɒlf"),
    ("hotel", "hoʊtˈɛl"),
    ("india", "ˈɪndiə"),
    ("juliett", "dʒˈuːliːˌɛt"),
    ("kilo", "kˈiːloʊ"),
    ("lima", "lˈiːmɐ"),
    ("mike", "mˈaɪk"),
    ("november", "noʊvˈɛmbɚ"),
    ("oscar", "ˈɒskɐ"),
    ("papa", "pəpˈɑː"),
    ("quebec", "kəbˈɛk"),
    ("romeo", "ɹˈoʊmiˌoʊ"),
    ("sierra", "siˈɛɹɐ"),
    ("tango", "tˈaŋɡoʊ"),
    ("uniform", "jˈuːnɪfɔːɹm"),
    ("victor", "vˈɪktɚ"),
    ("whiskey", "wˈɪski"),
    ("xray", "ˈɛksɹeɪ"),
    ("yankee", "jˈaŋki"),
    ("zulu", "zˈuːluː"),
    // ── letter names ────────────────────────────────────────────────────
    // Used when a word has to be spelled out. CMUdict has the other
    // twenty-three as ordinary words; these three it does not have at all,
    // and a letter read as a gap is what makes a spelled callsign useless.
    ("aitch", "ˈeɪtʃ"),
    ("doubleyou", "dˈʌbəljuː"),
    ("eff", "ˈɛf"),
    // ── units and measurements ──────────────────────────────────────────
    ("hertz", "hˈɜːts"),
    ("kilohertz", "kˈɪloʊhɜːts"),
    ("megahertz", "mˈɛɡɐhɜːts"),
    ("gigahertz", "ɡˈɪɡɐhɜːts"),
    ("decibels", "dˈɛsɪbɛlz"),
    ("kilowatts", "kˈɪlowɒts"),
    // ── modes and jargon ────────────────────────────────────────────────
    // "ritty" is how RTTY is said; nobody spells it out.
    ("ritty", "ɹˈɪti"),
    ("hellschreiber", "hˈɛlʃɹaɪbɚ"),
    ("olivia", "əlˈɪviə"),
    ("wefax", "wˈiːfaks"),
    ("sstv", "ˌɛsɛstiːvˈiː"),
    ("pota", "pˈoʊtɐ"),
    ("sota", "sˈoʊtɐ"),
    ("iota", "aɪˈoʊtɐ"),
    ("qso", "kjˌuːɛsˈoʊ"),
    ("qsl", "kjˌuːɛsˈɛl"),
    ("simplex", "sˈɪmplɛks"),
    ("duplex", "dˈuːplɛks"),
    ("sideband", "sˈaɪdband"),
    ("passband", "pˈasband"),
    ("waterfall", "wˈɔːtɚfɔːl"),
    ("sdroxide", "ˌɛsdiːˈɑːɹɒksaɪd"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entries_are_unique_and_sane() {
        for (i, (w, ipa)) in LEXICON.iter().enumerate() {
            assert!(!w.is_empty() && !ipa.is_empty(), "empty entry at {i}");
            assert!(
                w.chars().all(|c| c.is_ascii_lowercase()),
                "lexicon keys must be lowercase ascii: {w:?}"
            );
            assert!(
                LEXICON.iter().filter(|(o, _)| o == w).count() == 1,
                "duplicate lexicon entry: {w:?}"
            );
        }
    }

    #[test]
    fn lookup_is_case_insensitive() {
        assert!(lookup("ALPHA").is_some());
        assert!(lookup("Alpha").is_some());
        assert!(has("BrAvO"));
        assert!(lookup("nosuchword").is_none());
    }
}
