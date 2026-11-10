//! Builds faad2 for the DRM receiver: knik0's 2.11.2 from `vendor/faad2`, as it
//! is, with `DRM_SUPPORT`.
//!
//! Stock sources with one patch, applied here at build time so `vendor/` stays
//! untouched: see [`DRM_FILL_PATCH`]. The HD Radio decoder needs faad2's HDC
//! variant, which upstream does not carry, and it gets it from the `libnrsc5`
//! it loads at run time, which keeps its own patched copy private (issue #488).
//! So the one faad2 compiled into sdroxide is the one DRM needs — and DAB+
//! decodes through it too.
//!
//! Dependents find the headers through `DEP_FAAD2_INCLUDE`.

use std::path::{Path, PathBuf};

fn main() {
    let manifest = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
    let faad2 = manifest.join("../../vendor/faad2");

    if !faad2.join("include/neaacdec.h").exists() {
        panic!(
            "vendored faad2 is missing at {}\nrun: git submodule update --init --recursive",
            faad2.display()
        );
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", faad2.join("include").display());
    println!("cargo:rerun-if-changed={}", faad2.join("libfaad").display());

    build(&faad2);
    println!("cargo:include={}", faad2.join("include").display());
}

/// `DRM_SUPPORT` makes faad2's `fill_element` refuse a whole frame (error 30,
/// "No standard extension payload allowed in DRM") when a fill element carries
/// anything but SBR data. That is DRM's rule, not AAC's — and DAB+ encoders pad
/// their Access Units with ordinary fill elements, so in a DAB+ broadcast most
/// frames were thrown away and the audio came out chopped. The patch skips the
/// payload's bytes, exactly as the non-DRM build does with `extension_payload`.
/// A conforming DRM stream carries no such payload, so DRM is unaffected.
const DRM_FILL_PATCH: (&str, &str) = (
    "            (void)drc;\n            return 30;\n",
    "            (void)drc;\n            while (count > 0) {\n                faad_getbits(ld, LEN_BYTE);\n                count--;\n            }\n",
);

/// `syntax.c` with [`DRM_FILL_PATCH`] applied, written to `OUT_DIR`. Panics if
/// the anchor is not found exactly once, so a faad2 update cannot silently
/// drop the patch.
fn patched_syntax(faad2: &Path) -> PathBuf {
    let src = faad2.join("libfaad/syntax.c");
    let text = std::fs::read_to_string(&src).expect("read faad2 syntax.c");
    // Normalise line endings **before** matching. A Windows checkout of the
    // `vendor/faad2` submodule comes out CRLF (git's `core.autocrlf`), and the
    // anchor below is written with `\n` — so on Windows it matched nothing and
    // the build stopped with "the DRM fill-element anchor moved" while Linux and
    // macOS, which check out LF, built fine. The patch and the written file are
    // LF either way, which every C compiler takes.
    let text = text.replace("\r\n", "\n");
    let (from, to) = DRM_FILL_PATCH;
    assert_eq!(text.matches(from).count(), 1, "faad2 syntax.c: the DRM fill-element anchor moved");
    let out = PathBuf::from(std::env::var("OUT_DIR").unwrap()).join("syntax.c");
    std::fs::write(&out, text.replacen(from, to, 1)).expect("write patched syntax.c");
    out
}

/// The stock sources (one patched) with `DRM_SUPPORT`, which brings in `NeAACDecInitDRM`.
/// Upstream ships that as a second library, `libfaad_drm`, because the plain
/// one cannot decode DRM at all.
fn build(faad2: &Path) {
    let mut build = cc::Build::new();
    build
        .include(faad2.join("libfaad"))
        .include(faad2.join("include"))
        .define("HAVE_INTTYPES_H", "1")
        .define("HAVE_MEMCPY", "1")
        .define("HAVE_STRING_H", "1")
        .define("HAVE_STRINGS_H", "1")
        .define("HAVE_SYS_STAT_H", "1")
        .define("HAVE_SYS_TYPES_H", "1")
        .define("PACKAGE_VERSION", "\"2.11.2\"")
        .define("APPLY_DRC", None)
        .define("DRM_SUPPORT", None)
        .opt_level(2)
        .warnings(false);
    // The target's C library, not the host's: a build script runs on the host.
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() != Ok("msvc") {
        build.define("HAVE_LRINTF", "1");
    }
    let mut sources: Vec<PathBuf> = std::fs::read_dir(faad2.join("libfaad"))
        .expect("read faad2/libfaad")
        .map(|entry| entry.expect("faad2 dir entry").path())
        .filter(|path| path.extension().is_some_and(|e| e == "c"))
        .filter(|path| path.file_name().is_none_or(|n| n != "syntax.c"))
        .collect();
    sources.push(patched_syntax(faad2));
    sources.sort();
    build.files(sources);
    build.compile("sdroxide_faad2");
}
