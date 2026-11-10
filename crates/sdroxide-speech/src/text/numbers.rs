//! Numbers as words.
//!
//! Nothing downstream can pronounce a digit. Piper's front end is a dictionary
//! lookup, and a dictionary has no entry for `120`, `-12` or `14.074`; an
//! unknown token is dropped, so a stray digit is not a mispronunciation but a
//! **missing word**. Every number therefore becomes words here, before it goes
//! anywhere near the phonemizer.

const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];

const TENS: [&str; 10] =
    ["", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];

/// A single digit, as a word.
///
/// Always "zero", never "oh". A misheard frequency is a wrong frequency, and
/// "oh" and "four" are far closer over a speaker than "zero" and "four" are.
pub fn digit(d: u8) -> &'static str {
    ONES[(d % 10) as usize]
}

/// A cardinal number: `-1234` becomes "minus one thousand two hundred thirty four".
///
/// American style, without the "and" of "two hundred and thirty four" — one
/// word fewer in something the operator hears constantly, and unambiguous
/// either way.
pub fn cardinal(n: i64) -> String {
    if n < 0 {
        return format!("minus {}", cardinal(n.unsigned_abs() as i64));
    }
    let mut out = String::new();
    write_cardinal(n as u64, &mut out);
    out
}

fn write_cardinal(n: u64, out: &mut String) {
    const SCALES: [(u64, &str); 4] = [
        (1_000_000_000_000, "trillion"),
        (1_000_000_000, "billion"),
        (1_000_000, "million"),
        (1_000, "thousand"),
    ];

    if n < 20 {
        push(out, ONES[n as usize]);
        return;
    }
    if n < 100 {
        push(out, TENS[(n / 10) as usize]);
        if n % 10 != 0 {
            push(out, ONES[(n % 10) as usize]);
        }
        return;
    }
    if n < 1_000 {
        push(out, ONES[(n / 100) as usize]);
        push(out, "hundred");
        if n % 100 != 0 {
            write_cardinal(n % 100, out);
        }
        return;
    }
    for (scale, name) in SCALES {
        if n >= scale {
            write_cardinal(n / scale, out);
            push(out, name);
            if n % scale != 0 {
                write_cardinal(n % scale, out);
            }
            return;
        }
    }
    // Beyond a trillion nothing this program measures can reach; read the
    // digits rather than inventing scale names.
    push(out, &digits(&n.to_string()));
}

fn push(out: &mut String, word: &str) {
    if !out.is_empty() && !word.is_empty() {
        out.push(' ');
    }
    out.push_str(word);
}

/// Each digit of `s` separately: `"074"` becomes "zero seven four".
///
/// Anything that is not an ASCII digit is skipped, so a caller may hand this a
/// slice straight out of a formatted number.
pub fn digits(s: &str) -> String {
    let mut out = String::new();
    for ch in s.chars() {
        if let Some(d) = ch.to_digit(10) {
            push(&mut out, ONES[d as usize]);
        }
    }
    out
}

/// A number with one decimal place: `1.4` becomes "one point four".
///
/// A whole value loses the decimal entirely — "two to one", not "two point
/// zero to one".
pub fn decimal1(v: f32) -> String {
    decimals(v, 1)
}

/// A number with up to `places` decimals, trailing zeros trimmed.
///
/// The fractional digits are read individually — "one point zero five", not
/// "one point five" — because that is how a measurement is read aloud and
/// because it removes any question of where the decimal point was.
pub fn decimals(v: f32, places: usize) -> String {
    if !v.is_finite() {
        return "unknown".into();
    }
    let neg = v < 0.0;
    let v = v.abs();
    let scale = 10f64.powi(places as i32);
    let rounded = (v as f64 * scale).round() / scale;
    let whole = rounded.trunc() as i64;
    let frac = format!("{:.*}", places, rounded.fract());
    // "0.140" -> "14", i.e. drop the leading "0." and any trailing zeros.
    let frac = frac.split_once('.').map(|(_, f)| f).unwrap_or("");
    let frac = frac.trim_end_matches('0');

    let mut out = String::new();
    if neg {
        push(&mut out, "minus");
    }
    push(&mut out, &cardinal(whole));
    if !frac.is_empty() {
        push(&mut out, "point");
        push(&mut out, &digits(frac));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cardinals() {
        for (n, want) in [
            (0i64, "zero"),
            (1, "one"),
            (13, "thirteen"),
            (20, "twenty"),
            (21, "twenty one"),
            (99, "ninety nine"),
            (100, "one hundred"),
            (110, "one hundred ten"),
            (120, "one hundred twenty"),
            (145, "one hundred forty five"),
            (250, "two hundred fifty"),
            (999, "nine hundred ninety nine"),
            (1_000, "one thousand"),
            (1_500, "one thousand five hundred"),
            (14_074, "fourteen thousand seventy four"),
            (1_000_000, "one million"),
            (-6, "minus six"),
            (-120, "minus one hundred twenty"),
        ] {
            assert_eq!(cardinal(n), want, "cardinal({n})");
        }
    }

    #[test]
    fn individual_digits() {
        assert_eq!(digits("074"), "zero seven four");
        assert_eq!(digits("599"), "five nine nine");
        assert_eq!(digits(""), "");
        // Never "oh": over a speaker it is too close to "four".
        assert!(!digits("0").contains("oh"));
        assert_eq!(digits("0"), "zero");
    }

    #[test]
    fn one_decimal_place() {
        assert_eq!(decimal1(1.0), "one");
        assert_eq!(decimal1(1.4), "one point four");
        assert_eq!(decimal1(2.95), "three");
        assert_eq!(decimal1(9.9), "nine point nine");
        assert_eq!(decimal1(-6.0), "minus six");
        assert_eq!(decimal1(0.5), "zero point five");
    }

    #[test]
    fn more_decimal_places() {
        assert_eq!(decimals(1.05, 2), "one point zero five");
        assert_eq!(decimals(1.50, 2), "one point five");
        assert_eq!(decimals(14.074, 3), "fourteen point zero seven four");
        assert_eq!(decimals(f32::NAN, 2), "unknown");
    }
}
