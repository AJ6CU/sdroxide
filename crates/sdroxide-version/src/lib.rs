//! The version string the program gives when an operator asks what it is.
//!
//! [`VERSION`] is, in order of what decides it:
//!
//! - the **release** a tag names — `2.2.3.5` for `v2.2.3.5_brown`. This fork
//!   keeps `CARGO_PKG_VERSION` at its plain semver (`2.2.0`) and cuts
//!   fourth-point releases off it, so the crate version is not what an operator
//!   installed; the tag's own version rides in `SDROXIDE_RELEASE` and is what a
//!   release reports (issue #354);
//! - the crate version plus, on a build CI did not cut from a tag, what kind of
//!   build it is and the commit it came from —
//!   `1.6.8-nightly.20260922.abc1234`;
//! - the bare crate version for a local `cargo build`, unchanged from what
//!   `CARGO_PKG_VERSION` gave before.
//!
//! It is deliberately **not** a drop-in replacement for `CARGO_PKG_VERSION`.
//! Use it for what a person reads — `--version`, the About box, a crash report,
//! the window title — and leave the plain crate version everywhere something
//! other than a person parses it:
//!
//! - `sdroxide_config::version_is_newer` splits on `.` and compares the parts
//!   as numbers, so a stamped version reads as `1.6.0` and the update banner
//!   would never switch off again;
//! - rigctld's `Info` reply, the WSJT-X and Winlink identifiers, and the
//!   WSPRnet / PSK Reporter / FreeDV Reporter software fields ride wire
//!   protocols other people's software parses;
//! - the SSTV ID goes out over the air, where the extra characters do not fit.

/// The rule that decides [`VERSION`], compiled into the build script as well as
/// here so there is one implementation. Test-only in the crate: the value it
/// produces is a `const`, so nothing at run time calls it — the tests are what
/// keep it honest.
#[cfg(test)]
mod compose;

/// What this build calls itself.
pub const VERSION: &str = env!("SDROXIDE_VERSION");

/// What this build calls itself on screen: **SDR Oxide Brown**.
///
/// It began as "SDR Oxide CB/SWL", after what this fork adds. That worked as a
/// way to tell two installed builds apart, but several operators run it as an
/// ordinary amateur station and did not want "CB/SWL" in a screenshot or a
/// screen share — it reads as a different program than the one they are using.
/// The name is now **Brown**, the same idea as SDR++ Brown: a variant name that
/// says "this build, not upstream" without saying what is in it. One string, so
/// the window title, the General tab's header and `--version` all name it the
/// same way and cannot drift apart.
pub const FLAVOR: &str = "SDR Oxide Brown";

/// The **release** suffix the tags carry: `v1.9.2_brown`. The crate version
/// stays plain semver (`1.9.2`), because `1.9.2_brown` is not valid semver and
/// cargo refuses it — the suffix is for people and for a tag, never for
/// anything a machine parses.
pub const RELEASE_SUFFIX: &str = "_brown";

/// The version as a person reads it, name and all: what `--version` prints, and
/// what [`FLAVOR`] is for the places that compose their own line. The suffix is
/// added here rather than in `Cargo.toml`, so a stamped nightly reads
/// `…-nightly.…_brown` and a release reads `1.9.2_brown`.
pub const LONG_VERSION: &str = concat!("SDR Oxide Brown ", env!("SDROXIDE_VERSION"), "_brown");

/// The numeric release a **tag or a version string** names, as
/// `(major, minor, patch)`, or `None` if it names none.
///
/// A tag is `v2.0.3_brown`; the crate version is `2.0.3`; a stamped nightly is
/// `2.0.3-nightly.20261008…`. All three reduce to their leading three numbers,
/// which is the only part the update check compares — a suffix is not a newer
/// version, it is the *same* one packaged differently.
pub fn release_number(s: &str) -> Option<(u32, u32, u32)> {
    let t = s.trim().trim_start_matches('v');
    let t = t.split(RELEASE_SUFFIX).next().unwrap_or(t);
    let mut parts = t.split(|c: char| !c.is_ascii_digit()).filter(|p| !p.is_empty());
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next().unwrap_or("0").parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;
    Some((major, minor, patch))
}

/// Whether `latest` (a release tag) is a **newer release** than `current` (the
/// running version). A string that names no release is never newer, so a build
/// with a version this cannot parse is never nagged.
pub fn is_newer_release(latest: &str, current: &str) -> bool {
    match (release_number(latest), release_number(current)) {
        (Some(l), Some(c)) => l > c,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::compose::compose_version;

    /// An unstamped build must stay byte-identical to what the call sites
    /// printed before the stamp existed, because a release is built that way.
    #[test]
    fn unstamped_build_is_the_plain_crate_version() {
        if option_env!("SDROXIDE_BUILD").unwrap_or("").is_empty()
            && option_env!("SDROXIDE_RELEASE").unwrap_or("").is_empty()
        {
            assert_eq!(super::VERSION, env!("CARGO_PKG_VERSION"));
        }
    }

    /// A stamped one keeps the crate version as its prefix, so anything that
    /// eyeballs the leading digits still reads the release it came from.
    ///
    /// Not so for a **release** build: the tag's version replaces the crate
    /// version outright, because this fork tags `v2.2.3.5_brown` off a crate
    /// version of `2.2.0` and the tag is the truth (issue #354).
    #[test]
    fn stamped_build_keeps_the_crate_version_as_its_prefix() {
        if !option_env!("SDROXIDE_RELEASE").unwrap_or("").is_empty() {
            assert_eq!(super::VERSION, option_env!("SDROXIDE_RELEASE").unwrap());
            return;
        }
        assert!(super::VERSION.starts_with(env!("CARGO_PKG_VERSION")));
    }

    /// The three cases the build script decides between, checked here because
    /// the build script cannot be tested: it has no test harness, and the rule
    /// it runs is this one.
    #[test]
    fn the_release_tag_wins_over_the_crate_version() {
        // A release tag: v2.2.3.5_brown is built off crate version 2.2.0, and
        // `2.2.0` is what Settings used to say to someone running 2.2.3.5.
        assert_eq!(compose_version("2.2.0", "2.2.3.5", ""), "2.2.3.5");
        // A release is never stamped, but the release still decides if both
        // arrive: the tag is what was installed.
        assert_eq!(compose_version("2.2.0", "2.2.3.5", "nightly.20261031.abc12"), "2.2.3.5");
        // Not a release: the stamp says which build it is.
        assert_eq!(
            compose_version("2.2.0", "", "nightly.20261031.abc12"),
            "2.2.0-nightly.20261031.abc12"
        );
        // A plain `cargo build` anywhere: the crate version, byte for byte.
        assert_eq!(compose_version("2.2.0", "", ""), "2.2.0");
    }

    /// The long form names the build, so `--version` says which of two
    /// installed builds is which.
    #[test]
    fn the_long_version_names_the_build() {
        assert!(super::LONG_VERSION.starts_with(super::FLAVOR));
        assert!(super::LONG_VERSION.contains(super::VERSION));
    }

    /// The update check: a tag, a crate version and a stamped nightly all read
    /// as their leading three numbers, and only a genuinely higher release is
    /// "newer" — a nightly of the same release, or a downgrade, never is.
    #[test]
    fn only_a_higher_release_is_newer() {
        assert_eq!(super::release_number("v2.0.3_brown"), Some((2, 0, 3)));
        assert_eq!(super::release_number("2.0.3"), Some((2, 0, 3)));
        assert_eq!(super::release_number("2.0.3-nightly.20261008"), Some((2, 0, 3)));
        assert!(super::is_newer_release("v2.0.3_brown", "2.0.2"));
        assert!(super::is_newer_release("v2.1.0_brown", "2.0.9"));
        // The same release packaged as a nightly is not an update.
        assert!(!super::is_newer_release("v2.0.3_brown", "2.0.3-nightly.1"));
        // Nor is an older one, nor nonsense.
        assert!(!super::is_newer_release("v2.0.2_brown", "2.0.3"));
        assert!(!super::is_newer_release("nightly", "2.0.3"));
        assert!(!super::is_newer_release("v2.0.3_brown", "garbage"));
    }

    /// The release suffix is display-only and never touches the crate version,
    /// so anything that parses `CARGO_PKG_VERSION` or a wire field still sees
    /// plain semver.
    #[test]
    fn the_release_suffix_is_not_in_the_crate_version() {
        assert_eq!(super::RELEASE_SUFFIX, "_brown");
        assert!(!env!("CARGO_PKG_VERSION").contains("brown"));
        assert!(super::LONG_VERSION.ends_with("_brown"));
    }
}
