//! Compiles the faad2 shim. The faad2 headers and symbols come from
//! `sdroxide-faad2`, which builds the one stock faad2 in the binary — see its
//! `build.rs`. This only adds our small C wrapper on top.

fn main() {
    let include = std::env::var("DEP_FAAD2_INCLUDE").expect("sdroxide-faad2 exports its headers");
    cc::Build::new()
        .file("src/aac_shim.c")
        .include(&include)
        .warnings(true)
        .compile("sdroxide_dab_aac");
    // The shim calls `NeAACDec*`, which live in the archive `sdroxide-faad2`
    // builds. A static archive is searched once, in order, and by the time the
    // linker reads ours it has already passed faad2's — so the symbols go
    // undefined on some targets (a `--example`, the lib tests). Re-naming the
    // archive after ours, inside a group, resolves both directions whatever
    // order the two land in. `sdroxide-faad2` declares `links = "faad2"`, so
    // this is still the one faad2 in the binary.
    println!("cargo:rustc-link-lib=static=sdroxide_faad2");
    println!("cargo:rustc-link-arg=-Wl,--start-group");
    println!("cargo:rustc-link-arg=-lsdroxide_dab_aac");
    println!("cargo:rustc-link-arg=-lsdroxide_faad2");
    println!("cargo:rustc-link-arg=-Wl,--end-group");
    println!("cargo:rerun-if-changed=src/aac_shim.c");
}
