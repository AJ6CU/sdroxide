//! Callsigns, grid squares, and the letters they are made of.
//!
//! A callsign is the one token in an announcement that must not be misheard.
//! Everything else the operator can ask for again; a wrong callsign is a wrong
//! log entry. So the default is the ITU phonetic alphabet, at about two seconds
//! for a six-character call, which is the right trade for a message addressed
//! to you.

use sdroxide_types::CallsignStyle;

use super::numbers::digit;

/// ITU/NATO phonetics. `juliett` keeps the ITU spelling with two Ts, which is
/// also how [`crate::lexicon`] holds its pronunciation.
const NATO: [&str; 26] = [
    "alpha", "bravo", "charlie", "delta", "echo", "foxtrot", "golf", "hotel", "india", "juliett",
    "kilo", "lima", "mike", "november", "oscar", "papa", "quebec", "romeo", "sierra", "tango",
    "uniform", "victor", "whiskey", "xray", "yankee", "zulu",
];

/// Letter names, for [`CallsignStyle::Letters`]. Spelled the way they are said
/// rather than the way they are written, so the dictionary can pronounce them —
/// `aitch`, `doubleyou` and `eff` are in [`crate::lexicon`], the rest are
/// ordinary words.
const LETTERS: [&str; 26] = [
    "ay",
    "bee",
    "see",
    "dee",
    "ee",
    "eff",
    "gee",
    "aitch",
    "eye",
    "jay",
    "kay",
    "el",
    "em",
    "en",
    "oh",
    "pee",
    "cue",
    "are",
    "ess",
    "tee",
    "you",
    "vee",
    "doubleyou",
    "ex",
    "why",
    "zee",
];

/// One letter, in the requested style. Empty for anything not A–Z.
pub fn letter(ch: char, style: CallsignStyle) -> &'static str {
    let Some(i) = (ch.to_ascii_uppercase() as u8).checked_sub(b'A') else {
        return "";
    };
    let i = i as usize;
    if i >= 26 {
        return "";
    }
    match style {
        CallsignStyle::Phonetic => NATO[i],
        // `AsIs` never reaches here — the caller hands the raw text over
        // instead — but a letter name is a better fallback than nothing.
        CallsignStyle::Letters | CallsignStyle::AsIs => LETTERS[i],
    }
}

/// Spell a callsign out.
///
/// `/` becomes "stroke", the Region 1 reading, which is what the rest of this
/// program assumes throughout its band plan. Digits are ordinary digit words,
/// so `9A1AAA` opens "niner"-free with plain "nine" — the aviation forms buy
/// nothing here and cost familiarity.
pub fn callsign(call: &str, style: CallsignStyle) -> String {
    // `AsIs` hands the letters over as written and lets the voice do what it
    // will with them — but the digits still become words. Leaving them as
    // digits would not read them faster, it would drop them: the phonemizer
    // has no entry for `1`, so "K1ABC" would be spoken as "kabc".
    if style == CallsignStyle::AsIs {
        let mut out: Vec<String> = Vec::new();
        let mut run = String::new();
        for ch in call.chars() {
            match ch {
                'a'..='z' | 'A'..='Z' => run.push(ch.to_ascii_lowercase()),
                _ => {
                    if !run.is_empty() {
                        out.push(std::mem::take(&mut run));
                    }
                    match ch {
                        '0'..='9' => out.push(digit(ch as u8 - b'0').to_string()),
                        '/' => out.push("stroke".into()),
                        '-' => out.push("dash".into()),
                        _ => {}
                    }
                }
            }
        }
        if !run.is_empty() {
            out.push(run);
        }
        return out.join(" ");
    }

    let mut out: Vec<&str> = Vec::new();
    for ch in call.chars() {
        match ch {
            'a'..='z' | 'A'..='Z' => out.push(letter(ch, style)),
            '0'..='9' => out.push(digit(ch as u8 - b'0')),
            '/' => out.push("stroke"),
            '-' => out.push("dash"),
            _ => {}
        }
    }
    out.join(" ")
}

/// Read a Maidenhead locator: `JN47ab` becomes
/// "juliett november four seven alpha bravo".
///
/// The field and square letters go through the same phonetics as a callsign —
/// a grid is logged and a wrong one is a wrong distance.
pub fn grid(grid: &str, style: CallsignStyle) -> String {
    callsign(grid, style)
}

/// Whether a token looks like a callsign.
///
/// # Shape, not just ingredients
///
/// This used to ask only whether a token mixed letters with digits inside a
/// plausible length. On FT8 that is nearly always right, because almost nothing
/// else in a message looks like that. On a RTTY bulletin it is wrong several
/// times a line: `10100KHZ`, `1013HPA`, `24H` and `12UTC` are all letters and
/// digits in a plausible length, and none of them is a callsign.
///
/// A callsign is not an arbitrary mixture — the ITU gives it a shape, and the
/// shape is what is tested here:
///
/// - **amateur**: a prefix of one or two letters, or a digit and one or two
///   letters (`K`, `DL`, `9A`, `3DA`), then the region digits, then a suffix of
///   one to four letters. `K1ABC`, `9A1AAA`, `VP2E`, `2E0ABC`, `W100AW`.
/// - **station**: three letters and one or two digits, with no suffix — the
///   form the utility services use, and the reason `DDK2` and `DDH7` are
///   spelled out rather than read as words.
/// - either of those may carry up to two `/` parts (`DL/K1ABC/P`).
///
/// Getting this wrong in one direction spells a word out letter by letter,
/// which is merely slow; in the other it reads a callsign as a word, which is
/// useless. What the tightening gives up is the run of digits-and-letters that
/// is *not* a callsign — and the token rules in `super` read those by their
/// parts instead, so nothing is dropped either way.
pub fn looks_like_callsign(token: &str) -> bool {
    let core = token.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '/');
    if core.is_empty() || core.len() > 16 {
        return false;
    }
    let mut base = false;
    for (n, part) in core.split('/').enumerate() {
        if n >= 3 || part.is_empty() {
            return false;
        }
        if is_call_body(part) {
            base = true;
        } else if !is_call_affix(part) {
            return false;
        }
    }
    base
}

/// The callsign proper, without any `/` part.
fn is_call_body(part: &str) -> bool {
    let Some(runs) = runs(part) else {
        return false;
    };
    let alpha = |r: &(bool, usize), lo: usize, hi: usize| r.0 && (lo..=hi).contains(&r.1);
    let digit = |r: &(bool, usize), lo: usize, hi: usize| !r.0 && (lo..=hi).contains(&r.1);

    match runs.as_slice() {
        // Prefix, region digits, suffix: the amateur form. Three region digits
        // and a five-letter suffix are both special-event stretches of the
        // rules, but they are on the air and they are still callsigns.
        [p, d, s] if alpha(p, 1, 2) && digit(d, 1, 3) && alpha(s, 1, 5) => true,
        // The same, behind a numeric prefix: 9A1AAA, 3DA0RS.
        [n, p, d, s] if digit(n, 1, 1) && alpha(p, 1, 2) && digit(d, 1, 3) && alpha(s, 1, 5) => {
            true
        }
        // Utility and broadcast stations: DDK2, DDH7, DDK9, CFH6. Two letters
        // would take `RR73`, `TNX73` and `GUD73` with it, so this form asks
        // for three — and even then not one the shorthand table knows, which
        // is what keeps `CONDX` and a trailing number apart from a callsign.
        [p, d]
            if alpha(p, 3, 3)
                && digit(d, 1, 2)
                && super::abbrev::lookup(&part[..p.1]).is_none() =>
        {
            true
        }
        _ => false,
    }
}

/// A `/` part that is not itself a callsign: `/P`, `/MM`, `/QRP`, or the
/// country prefix in `DL/K1ABC`.
///
/// Loose on purpose — it only ever applies next to a part that *did* pass
/// [`is_call_body`], and no bare token reaches it.
fn is_call_affix(part: &str) -> bool {
    (1..=4).contains(&part.len()) && part.bytes().all(|b| b.is_ascii_alphanumeric())
}

/// The token as alternating letter and digit runs — `(is_letters, length)`.
/// `None` if anything else is in there.
fn runs(part: &str) -> Option<Vec<(bool, usize)>> {
    let mut out: Vec<(bool, usize)> = Vec::new();
    for b in part.bytes() {
        let alpha = match b {
            b'a'..=b'z' | b'A'..=b'Z' => true,
            b'0'..=b'9' => false,
            _ => return None,
        };
        match out.last_mut() {
            Some(last) if last.0 == alpha => last.1 += 1,
            _ => out.push((alpha, 1)),
        }
    }
    (!out.is_empty()).then_some(out)
}

/// Whether a token looks like a 4- or 6-character Maidenhead locator.
pub fn looks_like_grid(token: &str) -> bool {
    let b = token.as_bytes();
    if b.len() != 4 && b.len() != 6 {
        return false;
    }
    let field = |c: u8| c.is_ascii_alphabetic() && c.to_ascii_uppercase() <= b'R';
    let sub = |c: u8| c.is_ascii_alphabetic() && c.to_ascii_lowercase() <= b'x';
    field(b[0])
        && field(b[1])
        && b[2].is_ascii_digit()
        && b[3].is_ascii_digit()
        && (b.len() == 4 || (sub(b[4]) && sub(b[5])))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn callsigns_in_every_style() {
        assert_eq!(callsign("K1ABC", CallsignStyle::Phonetic), "kilo one alpha bravo charlie");
        assert_eq!(callsign("K1ABC", CallsignStyle::Letters), "kay one ay bee see");
        // Even "as written" spells the digit: a bare `1` reaches a
        // dictionary that has no entry for it and simply vanishes.
        assert_eq!(callsign("K1ABC", CallsignStyle::AsIs), "k one abc");
    }

    #[test]
    fn portable_and_prefixed_calls() {
        assert_eq!(
            callsign("DL/K1ABC", CallsignStyle::Phonetic),
            "delta lima stroke kilo one alpha bravo charlie"
        );
        assert_eq!(callsign("9A1AAA", CallsignStyle::Phonetic), "nine alpha one alpha alpha alpha");
    }

    #[test]
    fn grids() {
        assert_eq!(grid("JN47", CallsignStyle::Phonetic), "juliett november four seven");
        assert_eq!(
            grid("JN47ab", CallsignStyle::Phonetic),
            "juliett november four seven alpha bravo"
        );
    }

    #[test]
    fn callsign_detection() {
        for yes in [
            "K1ABC",
            "9A1AAA",
            "DL/K1ABC",
            "VP2E",
            "W1AW",
            "OE3ABC",
            "2E0ABC",
            "3DA0RS",
            "W100AW",
            "K1ABC/P",
            "DL/K1ABC/QRP",
            // The utility stations, which is what a RTTY bulletin is signed
            // with — DWD's own three.
            "DDK2",
            "DDH7",
            "DDK9",
        ] {
            assert!(looks_like_callsign(yes), "{yes} should look like a callsign");
        }
        for no in ["CQ", "the", "hello", "73", "ab"] {
            assert!(!looks_like_callsign(no), "{no} should not look like a callsign");
        }
    }

    /// What a weather bulletin is full of. Every one of these passed the old
    /// "has a letter and a digit" test and was read out letter by letter.
    #[test]
    fn bulletin_text_is_not_a_callsign() {
        for no in ["10100KHZ", "1013HPA", "24H", "12UTC", "5MIN", "21C", "H24", "1200Z"] {
            assert!(!looks_like_callsign(no), "{no} should not look like a callsign");
        }
        // Shorthand welded to a number is shorthand, not a station: the letter
        // run is in the abbreviation table, so the station form declines it.
        for no in ["RR73", "TNX73", "GUD73", "PSE73"] {
            assert!(!looks_like_callsign(no), "{no} should not look like a callsign");
        }
    }

    #[test]
    fn grid_detection() {
        for yes in ["JN47", "JN47ab", "FN31pr", "AA00"] {
            assert!(looks_like_grid(yes), "{yes} should look like a grid");
        }
        for no in ["K1ABC", "ZZ99", "JN4", "JN47abc", "1234"] {
            assert!(!looks_like_grid(no), "{no} should not look like a grid");
        }
    }
}
