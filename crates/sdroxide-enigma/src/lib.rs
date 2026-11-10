//! A working Enigma I / M4 — and a solver that breaks a copied ciphertext.
//!
//! A decryption toy for the fork's listeners (fork discussion, and
//! `MORNING.md` §3): the real machine, its real weakness, and a brute-force
//! breaker for the kind of ciphertext a shortwave listener might copy off a
//! museum broadcast or a web puzzle.
//!
//! - [`machine`] is the machine itself: historical wheels and reflectors, the
//!   double-step, the plugboard, and the flaw that a letter never enciphers to
//!   itself.
//! - [`solve`] is the breaker: it recovers rotor order, ring settings, start
//!   positions and the plugboard by scoring candidate plaintext with the Index
//!   of Coincidence, using the no-self-encipher property to prune.
//!
//! Everything is pure integer work with no dependencies, so the same code
//! runs in the native panel and in the browser build.

pub mod machine;
pub mod solve;

pub use machine::{
    Enigma, Plugboard, REFLECTORS, ROTORS, Reflector, Rotor, Variant, Wheel, group_fives, letter,
};
pub use solve::{Solution, SolveParams, solve};
