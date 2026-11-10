//! Spoken announcements, for operating the radio without seeing it.
//!
//! Three layers, split by what they depend on:
//!
//! - the **pure core** — text normalisation ([`text`]), the priority queue
//!   ([`queue`]) and the announcement state machine (`announce`). No I/O, no
//!   clock of its own, no audio. Every entry point takes `now: f64` (egui's
//!   monotonic frame time), which is what makes the whole thing testable
//!   without hardware and deterministic under test.
//! - the **[`sink`]** seam — one trait between deciding *what* to say and
//!   actually saying it, so the announcer runs identically against a real
//!   synthesizer, a recording sink in tests, and a null sink on wasm32.
//! - the **[`synth`]** backend — grapheme-to-phoneme, Piper VITS inference
//!   through `rten`, and its own audio output stream. Behind the `synth`
//!   feature, because sdroxide-ui depends on this crate unconditionally and
//!   also compiles for wasm32.
//!
//! Speech deliberately does *not* go through the engine's mixer. Anything
//! summed in there reaches the MP3 recorder and every remote listener of the
//! station, and an announcement is for the operator sitting at this screen.

pub mod announce;
pub mod lexicon;
pub mod queue;
pub mod sink;
pub mod text;

#[cfg(feature = "synth")]
pub mod synth;

pub use announce::Announcer;
pub use queue::{Priority, SpeechQueue, Utterance};
pub use sink::{NullSink, RecordingSink, SpeechSink};
pub use text::Speaker;
