fn target_is_windows_gnu() -> bool {
    std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("gnu")
}

fn main() {
    let mut build = cc::Build::new();
    build
        .cpp(true)
        .std("c++14")
        // **`M_PI` is not in the standard, and `-std=c++14` is strict ANSI.**
        // MinGW's libstdc++ hides it under that dialect, so the reference
        // headers (which use it in three places) fail to compile there while
        // they build fine on glibc — the release's Windows job died on
        // `'M_PI' was not declared in this scope` and the fork had no Windows
        // binaries until this was noticed. `-std=gnu++14` would do the same job
        // and is not a flag MSVC knows, so the constant is defined instead.
        // `_USE_MATH_DEFINES` is its MSVC spelling.
        .define("_USE_MATH_DEFINES", None)
        .define("M_PI", "3.14159265358979323846")
        .warnings(false)
        .include("cxx")
        .file("cxx/olivia_shim.cpp");
    if target_is_windows_gnu() {
        // MinGW links libstdc++/winpthread as DLLs by default, and those exist
        // only on a box with MSYS2 or PothosSDR on PATH. Link them statically
        // (see crates/sdroxide-drm/build.rs) so sdroxide.exe starts on a plain
        // Windows install.
        build.cpp_link_stdlib(None::<&str>);
    }
    build.compile("olivia_engine");

    if target_is_windows_gnu() {
        println!("cargo:rustc-link-lib=static:-bundle=stdc++");
        println!("cargo:rustc-link-lib=static:-bundle=winpthread");
    }

    println!("cargo:rerun-if-changed=cxx/olivia_shim.cpp");
    println!("cargo:rerun-if-changed=cxx/olivia_shim.h");
    println!("cargo:rerun-if-changed=cxx/pj_mfsk.h");
}
