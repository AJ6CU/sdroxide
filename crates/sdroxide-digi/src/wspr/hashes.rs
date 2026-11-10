//! The callsigns WSPR does not send, and why this station cannot recover them.
//!
//! A Type-3 message spends its callsign field on a 15-bit hash so it can afford
//! a six-character grid instead of four. There is no way to invert the hash; the
//! sender is betting the receiver has heard them recently in a message that
//! *did* carry the callsign, and has kept a table.
//!
//! **We do not keep that table, because we cannot compute the hash.** WSPR's is
//! Bob Jenkins's `lookup3` (`nhash(call, len, 146)`, low fifteen bits), which is
//! a different function from the base-38 multiplicative hash the FT8 family uses
//! — and `mfsk-core`, which supplies the decoder, implements only the latter.
//! Its `ihashcall` is not a WSPR hash at any bit width.
//!
//! So a Type-3 message is reported as its hash. That is a true statement about
//! what arrived; guessing a callsign from a different hash function would be a
//! false one, and it would be uploaded to WSPRnet as though it were a station.
//!
//! What survives is the part that matters most here: the six-character grid is
//! in the clear, so the path is real, the map places it, and the propagation
//! field counts it. Only the name is missing.
//!
//! Adding real resolution means porting `lookup3` and validating it against
//! published vectors — not against itself. Until then this module exists to say
//! so where somebody would look.

/// How a hashed callsign is shown: the fifteen-bit hash, in the `<#…>` form
/// WSJT-X uses for a callsign it has not resolved.
pub fn hashed_label(hash: u32) -> String {
    format!("<#{:05x}>", hash & 0x7fff)
}

/// Whether a callsign is one this station actually knows.
///
/// False for the placeholder [`hashed_label`] produces. Callers use it to keep a
/// spot off WSPRnet: a reception report naming `<#0a3f7>` is not a report about
/// any station, and posting one would put a fiction in a shared database.
pub fn is_resolved(call: &str) -> bool {
    !call.starts_with("<#")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unresolved_hash_shows_as_a_hash_rather_than_as_a_guess() {
        assert_eq!(hashed_label(0x1234), "<#01234>");
        // Only fifteen bits are the hash; the message has nowhere to put more.
        assert_eq!(hashed_label(0xffff_ffff), "<#07fff>");
    }

    #[test]
    fn a_placeholder_is_not_mistaken_for_a_callsign() {
        assert!(!is_resolved(&hashed_label(0x0a3f)));
        assert!(is_resolved("K1ABC"));
        assert!(is_resolved("PJ4/K1ABC"));
    }
}
