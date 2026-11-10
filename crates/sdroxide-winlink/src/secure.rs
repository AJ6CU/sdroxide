//! The Winlink secure-login challenge, mandatory on every account since
//! 15 April 2016. There is no way to reach the CMS without it.
//!
//! The exchange is two lines inside an otherwise ordinary B2F session: the
//! server offers `;PQ: <challenge>` and the client answers `;PR: <response>`,
//! where the response is eight decimal digits derived from the challenge, the
//! account password, and a fixed 64-byte salt.
//!
//! # Not to be confused with the other Winlink challenge
//!
//! Winlink's keyboard/telnet mode and its APRS gateway use a *positional*
//! scheme — the server sends three digit positions such as `425` and the
//! operator replies with the password characters at those positions plus three
//! arbitrary ones. That is a human-interactive mechanism for a different
//! purpose. B2F uses only what is implemented here.

use md5::{Digest, Md5};

/// The fixed salt, as found in paclink-unix and used by every implementation
/// that talks to the CMS. It is not secret and not negotiable; the response is
/// simply wrong without it.
#[rustfmt::skip]
const SECURE_SALT: [u8; 64] = [
    77, 197, 101, 206, 190, 249,
    93, 200, 51, 243, 93, 237,
    71, 94, 239, 138, 68, 108,
    70, 185, 225, 137, 217, 16,
    51, 122, 193, 48, 194, 195,
    198, 175, 172, 169, 70, 84,
    61, 62, 104, 186, 114, 52,
    61, 168, 66, 129, 192, 208,
    187, 249, 232, 193, 41, 113,
    41, 45, 240, 16, 29, 228,
    208, 228, 61, 20,
];

/// Answer a `;PQ:` challenge.
///
/// **The password is used exactly as given.** The response is case-sensitive —
/// the same challenge under `FOOBAR` and `FooBar` produces different answers,
/// which the tests pin — so nothing here folds case. Winlink issues passwords
/// in upper case and the operator is expected to enter them that way; silently
/// normalising would be a guess that fails closed at the CMS with nothing but
/// a rejected login to explain it.
pub fn login_response(challenge: &str, password: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(challenge.as_bytes());
    hasher.update(password.as_bytes());
    hasher.update(SECURE_SALT);
    let sum = hasher.finalize();

    // The low 30 bits of the digest's first four bytes, little-endian, read as
    // a decimal number. Masking byte 3 to 0x3f keeps it positive and bounds it
    // at 1_073_741_823 — ten digits, hence the trailing-eight slice below.
    let mut pr = (sum[3] & 0x3f) as u32;
    for i in (0..3).rev() {
        pr = (pr << 8) | sum[i] as u32;
    }

    let digits = format!("{pr:08}");
    digits[digits.len() - 8..].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Published vectors from `la5nta/wl2k-go`'s `secure_test.go`. These are
    /// the only external check available short of a live CMS login, so they are
    /// the thing standing between a subtle byte-order slip and an account that
    /// simply cannot connect.
    #[test]
    fn matches_published_vectors() {
        assert_eq!(login_response("23753528", "FOOBAR"), "72768415");
        assert_eq!(login_response("23753528", "FooBar"), "95074758");
    }

    #[test]
    fn is_case_sensitive() {
        // Pinned deliberately: it is tempting to "helpfully" upper-case the
        // password on the way in, and these vectors say that changes the answer.
        assert_ne!(login_response("23753528", "FOOBAR"), login_response("23753528", "foobar"));
    }

    #[test]
    fn is_always_eight_digits() {
        for challenge in ["0", "23753528", "99999999", ""] {
            for password in ["", "A", "FOOBAR", "a much longer passphrase"] {
                let got = login_response(challenge, password);
                assert_eq!(got.len(), 8, "{challenge:?}/{password:?} gave {got:?}");
                assert!(got.bytes().all(|b| b.is_ascii_digit()), "non-digit in {got:?}");
            }
        }
    }

    #[test]
    fn salt_is_the_expected_length() {
        assert_eq!(SECURE_SALT.len(), 64);
    }
}
