//! The seam between deciding what to say and saying it.
//!
//! The announcer talks to a `SpeechSink` and nothing else. That keeps it free
//! of audio, of threads and of the model, so it can be driven from a test with
//! a synthetic clock — and it is also what lets sdroxide-ui depend on this
//! crate on wasm32, where [`NullSink`] stands in and nothing is synthesized.

/// Somewhere for an utterance to go.
///
/// Implementations must not block: every method is called from the UI thread,
/// inside a frame.
pub trait SpeechSink {
    /// Hand over one utterance. `seq` identifies it, so that a later
    /// [`SpeechSink::done_seq`] can say which one finished.
    fn speak(&mut self, text: &str, seq: u64);

    /// Abandon whatever is in flight and drop any audio already queued for it.
    /// Idempotent — barge-in calls this whether or not anything is speaking.
    fn stop(&mut self);

    /// Whether audio for a `speak` is still being generated or played.
    fn busy(&self) -> bool;

    /// Generation of the last utterance that finished, or was stopped.
    fn done_seq(&self) -> u64;
}

/// Discards everything. Used on wasm32, and whenever speech is switched off.
///
/// It reports each utterance as finished the moment it arrives, so an
/// announcer driving it still advances its queue instead of stalling on
/// [`SpeechSink::busy`] forever.
#[derive(Debug, Default)]
pub struct NullSink {
    done: u64,
}

impl SpeechSink for NullSink {
    fn speak(&mut self, _text: &str, seq: u64) {
        self.done = seq;
    }
    fn stop(&mut self) {}
    fn busy(&self) -> bool {
        false
    }
    fn done_seq(&self) -> u64 {
        self.done
    }
}

/// A view of what a [`RecordingSink`] has been told to say.
///
/// Shared, because the sink itself disappears into a `Box<dyn SpeechSink>` the
/// moment it is handed to an announcer, and a test still needs to read back
/// what the operator would have heard.
#[derive(Debug, Clone, Default)]
pub struct SpeechLog(std::sync::Arc<std::sync::Mutex<Vec<String>>>);

impl SpeechLog {
    /// Everything said so far, oldest first.
    pub fn all(&self) -> Vec<String> {
        self.0.lock().unwrap().clone()
    }

    /// Everything said since the last call, and clear.
    pub fn take(&self) -> Vec<String> {
        std::mem::take(&mut *self.0.lock().unwrap())
    }

    pub fn len(&self) -> usize {
        self.0.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Whether anything said so far contains `needle`.
    pub fn any(&self, needle: &str) -> bool {
        self.0.lock().unwrap().iter().any(|s| s.contains(needle))
    }
}

/// Records what was said, for tests.
///
/// Like [`NullSink`] it completes instantly, so a test can push a burst of
/// state changes and then read back exactly what the operator would have heard,
/// in order.
#[derive(Debug, Default)]
pub struct RecordingSink {
    log: SpeechLog,
    done: u64,
    speaking: bool,
}

impl RecordingSink {
    /// The sink, and a handle to read it through once it has been boxed.
    pub fn new() -> (Self, SpeechLog) {
        let s = RecordingSink::default();
        let log = s.log.clone();
        (s, log)
    }
}

impl SpeechSink for RecordingSink {
    fn speak(&mut self, text: &str, seq: u64) {
        self.log.0.lock().unwrap().push(text.to_string());
        self.done = seq;
        self.speaking = false;
    }
    fn stop(&mut self) {
        self.speaking = false;
    }
    fn busy(&self) -> bool {
        self.speaking
    }
    fn done_seq(&self) -> u64 {
        self.done
    }
}
