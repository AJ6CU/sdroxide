//! International Morse (ITU-R M.1677-1): the letters and digits a VOR ident
//! can be made of, and the decoding of a keyed sequence into them.

/// Letters then digits, as dots and dashes.
pub const CODES: [(char, &str); 36] = [
    ('A', ".-"),
    ('B', "-..."),
    ('C', "-.-."),
    ('D', "-.."),
    ('E', "."),
    ('F', "..-."),
    ('G', "--."),
    ('H', "...."),
    ('I', ".."),
    ('J', ".---"),
    ('K', "-.-"),
    ('L', ".-.."),
    ('M', "--"),
    ('N', "-."),
    ('O', "---"),
    ('P', ".--."),
    ('Q', "--.-"),
    ('R', ".-."),
    ('S', "..."),
    ('T', "-"),
    ('U', "..-"),
    ('V', "...-"),
    ('W', ".--"),
    ('X', "-..-"),
    ('Y', "-.--"),
    ('Z', "--.."),
    ('0', "-----"),
    ('1', ".----"),
    ('2', "..---"),
    ('3', "...--"),
    ('4', "....-"),
    ('5', "....."),
    ('6', "-...."),
    ('7', "--..."),
    ('8', "---.."),
    ('9', "----."),
];

/// The character a pattern of dots and dashes stands for.
pub fn decode_char(pattern: &str) -> Option<char> {
    CODES.iter().find(|(_, p)| *p == pattern).map(|(c, _)| *c)
}

/// The pattern of a character, upper or lower case.
pub fn encode_char(c: char) -> Option<&'static str> {
    let c = c.to_ascii_uppercase();
    CODES.iter().find(|(k, _)| *k == c).map(|(_, p)| *p)
}

/// One key-down or key-up stretch, in the unit the caller counts in.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Run {
    pub on: bool,
    pub len: u32,
}

/// Slowest and fastest dot, in seconds: 5 to 20 words a minute. A VOR ident is
/// keyed at 7 to 10, so this leaves room on both sides and nothing else fits.
const DOT_RANGE_S: (f64, f64) = (0.06, 0.25);

/// Decode one ident: the runs between two long silences, `unit_s` seconds per
/// count. `None` unless every character is known and there are two to four (the ones beacons carry).
pub fn decode_runs(runs: &[Run], unit_s: f64) -> Option<String> {
    // The dot is the shortest thing keyed, whether a mark or an inner gap: both
    // are one unit long. A tone that is all dashes and word gaps would fool
    // this, and no ident is.
    let shortest = runs.iter().map(|r| r.len).min()?;
    let dot = shortest as f64 * unit_s;
    if !(DOT_RANGE_S.0..=DOT_RANGE_S.1).contains(&dot) {
        return None;
    }
    let dash_at = 2.0 * dot;
    let letter_at = 2.0 * dot;
    let word_at = 5.0 * dot;

    let mut out = String::new();
    let mut pattern = String::new();
    for run in runs {
        let secs = run.len as f64 * unit_s;
        if run.on {
            pattern.push(if secs > dash_at { '-' } else { '.' });
        } else if secs > letter_at {
            if !pattern.is_empty() {
                out.push(decode_char(&pattern)?);
                pattern.clear();
            }
            if secs > word_at {
                // A word gap inside an ident is not one we know.
                return None;
            }
        }
    }
    if !pattern.is_empty() {
        out.push(decode_char(&pattern)?);
    }
    (2..=4).contains(&out.len()).then_some(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_pattern_is_unique() {
        for (i, (_, a)) in CODES.iter().enumerate() {
            for (_, b) in &CODES[i + 1..] {
                assert_ne!(a, b);
            }
        }
    }

    #[test]
    fn encode_and_decode_agree() {
        for (c, p) in CODES {
            assert_eq!(encode_char(c), Some(p));
            assert_eq!(decode_char(p), Some(c));
        }
        assert_eq!(encode_char('q'), Some("--.-"));
        assert_eq!(decode_char("......."), None);
    }

    fn keyed(text: &str, dot: u32) -> Vec<Run> {
        let mut runs = Vec::new();
        for (i, ch) in text.chars().enumerate() {
            if i > 0 {
                runs.push(Run { on: false, len: 3 * dot });
            }
            for (j, sym) in encode_char(ch).unwrap().chars().enumerate() {
                if j > 0 {
                    runs.push(Run { on: false, len: dot });
                }
                runs.push(Run { on: true, len: if sym == '-' { 3 * dot } else { dot } });
            }
        }
        runs
    }

    #[test]
    fn decodes_a_keyed_ident() {
        // 6 counts of 20 ms is a 120 ms dot: 10 words a minute.
        assert_eq!(decode_runs(&keyed("EPL", 6), 0.02).as_deref(), Some("EPL"));
        assert_eq!(decode_runs(&keyed("TST", 8), 0.02).as_deref(), Some("TST"));
    }

    #[test]
    fn rejects_what_is_not_morse_speed() {
        // A 20 ms dot is 60 words a minute; a 400 ms one is 3.
        assert_eq!(decode_runs(&keyed("EPL", 1), 0.02), None);
        assert_eq!(decode_runs(&keyed("EPL", 20), 0.02), None);
    }

    #[test]
    fn rejects_an_unknown_pattern_and_a_wrong_length() {
        let seven_dots: Vec<Run> = (0..13)
            .map(|i| Run { on: i % 2 == 0, len: 6 })
            .collect();
        assert_eq!(decode_runs(&seven_dots, 0.02), None);
        assert_eq!(decode_runs(&keyed("EPLEPL", 6), 0.02), None);
        assert_eq!(decode_runs(&[], 0.02), None);
    }
}
