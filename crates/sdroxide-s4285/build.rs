//! Compiles the vendored reference decoder.
//!
//! It is C++ and it stays C++: `vendor/s4285-ref` is upstream unmodified, and
//! the only thing built here is our own shim around it.

fn target_is_windows_gnu() -> bool {
    std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("gnu")
}

fn main() {
    println!("cargo:rerun-if-changed=src/shim.cc");
    println!("cargo:rerun-if-changed=../../vendor/s4285-ref/include");
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++17")
        .include("../../vendor/s4285-ref/include")
        .file("src/shim.cc")
        .warnings(false);
    // **`M_PI` on Windows.** The vendored decoder's headers use `M_PI`, which a
    // C++ standard library is not obliged to define: glibc's does, the MinGW
    // toolchain the Windows runner uses does not, and the whole crate failed to
    // build there — `error: 'M_PI' was not declared in this scope`, both Windows
    // jobs, on v2.1.1.1 and again on v2.1.2. `vendor/` stays untouched, so the
    // constant is given to the compiler instead. Windows only: elsewhere it is
    // already there and a second definition would be a redefinition warning.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        build.define("M_PI", "3.14159265358979323846");
    }
    if target_is_windows_gnu() {
        // Same as crates/sdroxide-drm/build.rs: MinGW links libstdc++/winpthread
        // as DLLs by default, which a plain Windows box does not have. Link
        // them statically so sdroxide.exe starts without MSYS2/PothosSDR.
        build.cpp_link_stdlib(None::<&str>);
    }
    build.compile("s4285ref");

    if target_is_windows_gnu() {
        println!("cargo:rustc-link-lib=static:-bundle=stdc++");
        println!("cargo:rustc-link-lib=static:-bundle=winpthread");
    }
}
