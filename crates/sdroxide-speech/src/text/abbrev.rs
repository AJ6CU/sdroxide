//! On-air shorthand, read the way an operator would say it.
//!
//! Applied to **free text only** — the body of an FT8 message, a JS8 or FSQ
//! message, a CW or RTTY tail. Never to phrases this crate generates itself,
//! which are already words.
//!
//! Without this pass, `73` reaches the phonemizer as a token no dictionary has
//! and disappears; `CQ` is read as a word; `RR73` becomes nothing at all. The
//! table is what turns a decoded line into something an operator recognises
//! hearing.

/// Multi-token entries, matched first and longest-first.
///
/// Only where the pair means something the words separately do not.
const PHRASES: &[(&str, &str)] =
    // `HW` alone is "how copy", so `HW CPY` would otherwise be "how copy copy".
    &[("hi hi", "hi hi"), ("cq dx", "C Q D X"), ("hw cpy", "how copy"), ("de", "from")];

/// Single tokens. Uppercase keys; lookup is case-insensitive.
///
/// Spaced capitals ("Q S L") are the way to make the phonemizer spell
/// something out: each letter goes through the dictionary as its own word.
const WORDS: &[(&str, &str)] = &[
    // Sign-off numbers. These are the ones that vanish without the table.
    ("73", "seventy three"),
    ("72", "seventy two"),
    ("88", "eighty eight"),
    ("RR73", "roger roger seventy three"),
    ("RRR", "roger roger roger"),
    ("RR", "roger roger"),
    // Q-codes and procedural signals: spelled, because that is how they are said.
    ("CQ", "C Q"),
    ("QSL", "Q S L"),
    ("QRZ", "Q R Z"),
    ("QSO", "Q S O"),
    ("QRM", "Q R M"),
    ("QRN", "Q R N"),
    ("QRT", "Q R T"),
    ("QRV", "Q R V"),
    ("QRX", "Q R X"),
    ("QSY", "Q S Y"),
    ("QTH", "Q T H"),
    ("QRP", "Q R P"),
    ("QRO", "Q R O"),
    ("QSB", "Q S B"),
    ("QRL", "Q R L"),
    ("RST", "R S T"),
    ("SNR", "S N R"),
    ("DX", "D X"),
    ("SK", "S K"),
    ("AR", "A R"),
    ("KN", "K N"),
    ("BK", "break"),
    // Ordinary shorthand.
    ("TU", "thank you"),
    ("TNX", "thanks"),
    ("TKS", "thanks"),
    ("PSE", "please"),
    ("UR", "your"),
    ("ES", "and"),
    ("AGN", "again"),
    ("ABT", "about"),
    ("HW", "how copy"),
    ("GM", "good morning"),
    ("GA", "good afternoon"),
    ("GE", "good evening"),
    ("GN", "good night"),
    ("GL", "good luck"),
    ("GUD", "good"),
    ("FB", "fine business"),
    ("SRI", "sorry"),
    ("CFM", "confirm"),
    ("RPT", "repeat"),
    ("WX", "weather"),
    ("PWR", "power"),
    ("ANT", "antenna"),
    ("RIG", "rig"),
    ("HR", "here"),
    ("NW", "now"),
    ("VY", "very"),
    ("WID", "with"),
    ("OM", "old man"),
    ("YL", "young lady"),
    ("XYL", "X Y L"),
    ("OP", "operator"),
    ("NAME", "name"),
    ("CPY", "copy"),
    ("MSG", "message"),
    ("GD", "good"),
    ("MNI", "many"),
    ("DR", "dear"),
    ("STN", "station"),
    ("FREQ", "frequency"),
    ("NR", "number"),
    ("CONDX", "conditions"),
    ("SIGS", "signals"),
    ("XMTR", "transmitter"),
    ("RCVR", "receiver"),
    ("BCNU", "be seeing you"),
    ("CUL", "see you later"),
    ("CU", "see you"),
    ("BTU", "back to you"),
    ("WKD", "worked"),
    ("OK", "okay"),
    ("MR", "mister"),
    // Award and activity programmes.
    ("POTA", "pota"),
    ("SOTA", "sota"),
    ("IOTA", "iota"),
    ("WWFF", "W W F F"),
    ("SES", "special event station"),
    // ── units ───────────────────────────────────────────────────────────
    // A unit is a word, not an initialism: an operator hears "ten thousand
    // one hundred kilohertz", never "one zero one zero zero K H Z". Without
    // these the letter rules below would spell every one of them, because
    // `KHZ` and `HPA` are shaped exactly like initialisms.
    ("HZ", "hertz"),
    ("KHZ", "kilohertz"),
    ("MHZ", "megahertz"),
    ("GHZ", "gigahertz"),
    ("KW", "kilowatts"),
    ("DB", "decibels"),
    ("WPM", "words per minute"),
    ("DEG", "degrees"),
    ("KM", "kilometers"),
    // Marine and aviation weather, which is most of what the RTTY bands
    // actually carry: DWD, Northwood, the coast stations.
    ("UTC", "U T C"),
    ("GMT", "G M T"),
    ("HPA", "hectopascals"),
    ("KT", "knots"),
    ("KTS", "knots"),
    // Compass points. "S W" is a reading nobody wants in a wind direction,
    // and a bulletin is where these overwhelmingly appear. `NW` is not here:
    // it is already "now" above, which is what it means in a QSO, and a QSO
    // is the older claim on it.
    ("NE", "northeast"),
    ("SE", "southeast"),
    ("SW", "southwest"),
];

/// Read as letters even though they are pronounceable.
///
/// The general rule below — an all-capitals token with no vowel is an
/// initialism — cannot see these, because a vowel is exactly what they have.
/// Every entry is something an operator says letter by letter anyway.
///
/// `AM` and `IF` are deliberately absent. Both are ordinary English words that
/// turn up in the middle of a weather bulletin far more often than they turn up
/// as a mode or an intermediate frequency, and the mode announcement has its
/// own path through [`super::Speaker::mode`] that never reaches here.
const LETTER_TOKENS: &[&str] = &[
    "USB", "AGC", "ALC", "AFSK", "APRS", "ADIF", "ARRL", "IARU", "ITU", "EME", "ERP", "EIRP",
    "UHF", "EHF", "ISS", "USA", "UK",
];

/// Vowels, for the "is this pronounceable" test. `Y` counts: without it `TRY`,
/// `DRY` and `SKY` are spelled out, and they are words.
const VOWELS: [char; 6] = ['A', 'E', 'I', 'O', 'U', 'Y'];

/// The replacement for one token, if the table has one.
///
/// `K` is deliberately absent: on its own it means "over", but it is also a
/// callsign letter and a mode name, and the caller decides from position.
pub fn lookup(token: &str) -> Option<&'static str> {
    WORDS.iter().find(|(k, _)| k.eq_ignore_ascii_case(token)).map(|(_, v)| *v)
}

/// A multi-token replacement starting at `words[i]`, and how many it consumed.
///
/// Trailing sentence punctuation does not break a match: `HW CPY?` is the
/// phrase with a question mark on it, and the caller puts the punctuation back.
pub fn lookup_phrase(words: &[&str], i: usize) -> Option<(&'static str, usize)> {
    for (key, val) in PHRASES {
        let n = key.split(' ').count();
        if i + n <= words.len() {
            let matches = key.split(' ').zip(&words[i..i + n]).all(|(k, w)| {
                k.eq_ignore_ascii_case(w.trim_end_matches(['.', ',', '!', '?', ':', ';']))
            });
            if matches {
                return Some((val, n));
            }
        }
    }
    None
}

/// Whether a token is an initialism to spell rather than a word to say.
///
/// # Why capitalisation is not the test
///
/// It used to be: one to five capitals meant "spell it". That works for FT8,
/// where a message is a handful of tokens from a fixed vocabulary. It falls
/// apart on RTTY, because **Baudot has no lower case** — every character a DWD
/// weather bulletin sends arrives capitalised, so the rule spelled `G O O D`,
/// `K H Z` and `S E A` right through the forecast.
///
/// So the question asked here is not "was it capitalised" but "can it be said":
///
/// - a lone capital is a letter — a bare `R` or `N` in a decoded line;
/// - `Q` plus two letters is a Q-code, the whole family of them, said as
///   letters whether or not the table above happens to list that one;
/// - a token with no vowel in it cannot be pronounced, so it is an initialism
///   by construction: `SSB`, `PSK`, `DWD`, `MSLP`;
/// - the handful of pronounceable ones that are still read as letters are
///   listed in `LETTER_TOKENS`.
///
/// Everything else is handed on as a word. That is the important half of the
/// change: what this function declines, the dictionary decides — and a word it
/// does not know is spelled out downstream anyway (see
/// `synth::phonemes`), so an unlisted initialism still reads correctly
/// while `GOOD` is simply said.
pub fn reads_as_letters(token: &str) -> bool {
    let n = token.chars().count();
    if n == 0 || n > 5 || !token.chars().all(|c| c.is_ascii_uppercase()) {
        return false;
    }
    if n == 1 {
        return true;
    }
    if n == 3 && token.starts_with('Q') {
        return true;
    }
    LETTER_TOKENS.contains(&token) || !token.contains(VOWELS)
}

/// Whether a token reads as an RST report — `599`, `5NN`, `579`.
///
/// Cut numbers are expanded before the shape is checked, so `5NN` and `599`
/// take the same path. The digit ranges are what makes this safe to run ahead
/// of the plain-integer rule: an ordinary `250` or `100` fails, because no RST
/// has a zero in it.
pub fn looks_like_rst(token: &str) -> bool {
    let b: Vec<char> = token.chars().map(expand_cut).collect();
    b.len() == 3
        && ('1'..='5').contains(&b[0])
        && ('1'..='9').contains(&b[1])
        && ('1'..='9').contains(&b[2])
}

/// A cut number as the digit it stands for. Anything else passes through.
pub fn expand_cut(c: char) -> char {
    match c.to_ascii_uppercase() {
        'N' => '9',
        'T' => '0',
        'A' => '1',
        'E' => '5',
        other => other,
    }
}

/// Space out a token's letters so each is spoken as its own word.
pub fn spaced(token: &str) -> String {
    token
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| c.to_ascii_uppercase().to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_sign_off_numbers_are_covered() {
        // These are the tokens that disappear entirely without the table:
        // no dictionary has an entry for "73".
        assert_eq!(lookup("73"), Some("seventy three"));
        assert_eq!(lookup("RR73"), Some("roger roger seventy three"));
        assert_eq!(lookup("rr73"), Some("roger roger seventy three"));
    }

    #[test]
    fn q_codes_are_spelled() {
        assert_eq!(lookup("QSL"), Some("Q S L"));
        assert_eq!(lookup("qrz"), Some("Q R Z"));
        assert_eq!(lookup("CQ"), Some("C Q"));
    }

    #[test]
    fn phrases_win_over_words() {
        let words = ["CQ", "DX", "K1ABC"];
        assert_eq!(lookup_phrase(&words, 0), Some(("C Q D X", 2)));
        assert_eq!(lookup_phrase(&words, 2), None);
    }

    #[test]
    fn initialisms() {
        assert!(reads_as_letters("RST"));
        assert!(reads_as_letters("SSB"));
        // A lone capital is a letter to spell, not a word.
        assert!(reads_as_letters("K"));
        assert!(!reads_as_letters("hello"));
        assert!(!reads_as_letters("K1ABC"));
        assert_eq!(spaced("RST"), "R S T");
    }

    /// The RTTY case: Baudot is upper case only, so capitalisation says
    /// nothing and every one of these used to be spelled out.
    #[test]
    fn capitals_alone_do_not_mean_spell_it() {
        for word in ["GOOD", "SEA", "WIND", "GALE", "NORTH", "LATER", "MAY", "SKY", "DRY"] {
            assert!(!reads_as_letters(word), "{word} is a word, not an initialism");
        }
        // ...while the things that genuinely are initialisms still are.
        for init in ["SSB", "PSK", "DWD", "MSLP", "TX", "USB", "QRK", "QSA", "N"] {
            assert!(reads_as_letters(init), "{init} should be spelled out");
        }
    }

    /// The vowel test cannot see a unit — `KHZ` is shaped exactly like an
    /// initialism — so the table has to.
    #[test]
    fn units_are_words() {
        assert_eq!(lookup("KHZ"), Some("kilohertz"));
        assert_eq!(lookup("MHz"), Some("megahertz"));
        assert_eq!(lookup("hpa"), Some("hectopascals"));
        assert_eq!(lookup("PSE"), Some("please"));
    }

    #[test]
    fn no_duplicate_keys() {
        for (k, _) in WORDS {
            assert_eq!(
                WORDS.iter().filter(|(o, _)| o.eq_ignore_ascii_case(k)).count(),
                1,
                "duplicate abbreviation: {k}"
            );
        }
    }
}
