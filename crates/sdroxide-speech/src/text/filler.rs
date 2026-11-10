//! Test patterns and idle runs — the parts of a transmission with no words in
//! them.
//!
//! A teleprinter link is kept awake by sending something continuously, and what
//! it sends is not text. `RYRYRYRYRYRYRYRYRY…` is the classic: R and Y are
//! alternating tones in Baudot, so a run of them exercises the whole shift and
//! makes a printer's mistuning obvious. DWD sends a line of it between
//! bulletins; other services send `RY` for minutes at a time before a schedule.
//! Read aloud, sixty-four characters of it is over a minute of "romeo yankee"
//! during which nothing else can be spoken — and the operator has already
//! missed the start of the bulletin it was announcing.
//!
//! The same shape covers the rest of the family: a stuck key, a Baudot diddle
//! printing as `EEEEEEEE`, a tuning run of `MMMMMM`.
//!
//! Digits are deliberately not part of this. `0000` is a repetition too, and it
//! is also midnight.

/// Whether a token is filler to be dropped rather than read.
///
/// True for a run of four or more letters built by repeating a one- or
/// two-letter unit — `RYRYRY`, `EEEEEE`, `ABABAB` — and for a bare `RY`, which
/// is the same idle sent in groups rather than as one run.
///
/// The final unit may be partial (`RYRYR`), because where the decoder cut the
/// run says nothing about what it was.
pub fn is_filler(token: &str) -> bool {
    let b = token.as_bytes();
    if b.is_empty() || !b.iter().all(|c| c.is_ascii_alphabetic()) {
        return false;
    }
    if token.eq_ignore_ascii_case("ry") {
        return true;
    }
    if b.len() < 4 {
        return false;
    }
    [1, 2].iter().any(|&unit| {
        // A repeat of a single letter is also a repeat of two, so `AAAA` is
        // caught by the first pass and `ABAB` by the second.
        b.iter().enumerate().all(|(i, c)| c.eq_ignore_ascii_case(&b[i % unit]))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The telex control markers fall out of the same test, and should: they
    /// bracket a bulletin, they are not part of it, and nobody reads them.
    #[test]
    fn the_message_markers_go_too() {
        assert!(is_filler("ZCZC"), "start of message");
        assert!(is_filler("NNNN"), "end of message");
    }

    #[test]
    fn the_teleprinter_idle() {
        assert!(is_filler("RYRYRYRYRYRYRYRYRYRYRYRYRYRYRYRY"));
        assert!(is_filler("RYRY"));
        // Cut mid-unit by the decoder, or by our own chunking.
        assert!(is_filler("RYRYR"));
        assert!(is_filler("ry"));
        assert!(is_filler("EEEEEEEE"));
        assert!(is_filler("MMMM"));
    }

    #[test]
    fn real_text_survives() {
        for word in ["GOOD", "SEA", "CQ", "DDK2", "K1ABC", "0000", "1010", "TEST", "NOON"] {
            assert!(!is_filler(word), "{word} is not filler");
        }
    }
}
