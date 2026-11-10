//! Stamps the identity of this build into the binary.
//!
//! Two things a build can be, and both are decided here because
//! `CARGO_PKG_VERSION` cannot tell them apart:
//!
//! - **A release**, cut from a tag. `CARGO_PKG_VERSION` stays plain semver
//!   (`2.2.0`) and this fork tags fourth-point releases off it
//!   (`v2.2.3.5_brown`), so the crate version is *not* what the operator
//!   installed. CI passes the tag's own version in `SDROXIDE_RELEASE` — that is
//!   issue #354, where Settings said `2.2.0` to someone running `2.2.3.5`.
//! - **A build that is not a release**, which says which build it is and which
//!   commit it came from: CI sets `SDROXIDE_BUILD` to `nightly.20261031.abc12`,
//!   and a bug report then names the build rather than the release it was cut
//!   from.
//!
//! Nothing here shells out to git. The -compat builds compile inside a
//! container that has no repository, and a source tarball has no `.git` at
//! all — so the inputs are environment variables, and a plain `cargo build`
//! anywhere produces the release string byte for byte.
//!
//! The rule that combines them lives in `src/compose.rs`, included here
//! because a build script cannot call into the crate it builds, and a module
//! there so `cargo test` can check it.

include!("src/compose.rs");

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/compose.rs");
    // Without these cargo has no reason to believe anything changed between two
    // builds of the same commit, and the second would keep the first's stamp.
    println!("cargo:rerun-if-env-changed=SDROXIDE_BUILD");
    println!("cargo:rerun-if-env-changed=SDROXIDE_RELEASE");

    let base = std::env::var("CARGO_PKG_VERSION").unwrap();
    // The values are interpolated into a `cargo:` directive, which is parsed a
    // line at a time, and from there into a window title and a crash report.
    // Keep them to what a version string is made of and drop the rest, rather
    // than let a stray newline rewrite this script's output.
    let clean = |raw: String| -> String {
        raw.chars().filter(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_')).collect()
    };
    let release = clean(std::env::var("SDROXIDE_RELEASE").unwrap_or_default());
    let stamp = clean(std::env::var("SDROXIDE_BUILD").unwrap_or_default());

    let version = compose_version(&base, &release, &stamp);
    println!("cargo:rustc-env=SDROXIDE_VERSION={version}");
}
