// What a build calls itself, decided from the three things a build knows.
//
// Compiled in twice, deliberately: `VERSION` is a `const` set by the build
// script, so the rule that decides it has to run *in* the build script — and a
// build script cannot call into the crate it builds. This file is `include!`d
// there and a module here, so there is one implementation and `cargo test` gets
// to check it (see `the_release_tag_wins_over_the_crate_version` in `lib.rs`).
//
// Regular comments above rather than `//!`, and that is not a style choice:
// `include!` lands this text after the build script's own header, where an
// inner doc comment is a compile error.
//
// The three inputs, in the order that decides it:
//
// - `release` — the version a **tag** names, `2.2.3.5` for `v2.2.3.5_brown`.
//   A tag is a release, so that is exactly what it calls itself.
// - `stamp` — what kind of build this is and which commit, for a nightly or a
//   CI build that is not a release: `nightly.20261031.abc1234`.
// - `base` — the crate version, `2.2.0`.
//
// **Why `release` is not just `base`.** This fork keeps `CARGO_PKG_VERSION` at
// its plain semver and cuts its releases as *fourth-point tags* —
// `v2.2.3.5_brown` off a crate version of `2.2.0`. Taking the crate version as
// the release version is therefore a lie that costs support calls: Settings
// says `2.2.0` to an operator who installed `2.2.3.5`, and the first question
// of every bug report becomes "which build is that" (issue #354). The crate
// version stays plain because machines parse it; the *release* is what a
// person reads, so the tag's own version rides in on its own variable.

/// The version this build reports, from the release, the stamp and the crate.
pub fn compose_version(base: &str, release: &str, stamp: &str) -> String {
    if !release.is_empty() {
        return release.to_string();
    }
    if stamp.is_empty() {
        return base.to_string();
    }
    format!("{base}-{stamp}")
}
