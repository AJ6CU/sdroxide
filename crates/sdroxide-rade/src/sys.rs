//! Raw bindings to `rade_api.h` and `shim.h`.
//!
//! Private by design: nothing here is re-exported. Every caller goes through
//! the safe wrappers in [`crate`], which own the lifetime rules and turn the C
//! return conventions into `Result`.

#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case, dead_code)]

/// The RADE context, treated as an opaque handle.
///
/// `struct rade` is fully defined in `rade_api.h` — it embeds the V1 and V2
/// transmit and receive state inline — but we only ever hold a pointer handed
/// back by `rade_open()`. Declaring it opaque here means nothing in Rust can
/// depend on that layout, so a change to the C struct can't silently
/// misinterpret its fields.
#[repr(C)]
pub struct rade {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
