//! A small pool of decoders, for callers with many signals at once.
//!
//! The skimmer watches a whole band and may have a dozen stations going at the
//! same time. Each one needs its own window through the model, and at roughly
//! 25 ms apiece that is real work — enough to stall the skimmer's own thread if
//! it were done inline, and enough to be worth spreading across cores.
//!
//! Jobs are keyed so results can be matched back to whatever the caller is
//! tracking, and they come back in whatever order they finish. Submission never
//! blocks: when the pool is saturated the job is refused and the caller decides
//! what that means — for a skimmer it means this station waits for the next
//! round, which is exactly right.

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use crossbeam_channel::{Receiver, Sender, bounded};

use crate::ctc::Decoded;
use crate::model::{Decoder, Error, Window};

/// A window to decode, tagged with whatever the caller calls it.
pub struct Job {
    pub key: u64,
    pub window: Window,
}

/// Worker threads sharing the one loaded model.
pub struct Pool {
    jobs: Sender<Job>,
    results: Receiver<(u64, Result<Decoded, Error>)>,
    in_flight: Arc<AtomicUsize>,
    _threads: Vec<std::thread::JoinHandle<()>>,
}

impl Pool {
    /// Spawn `threads` decoders, sharing one inference pool of the same size.
    /// Clamped to at least one.
    ///
    /// The two numbers are deliberately the same. `threads` decides how many
    /// windows may be *in* the model at once; the inference pool decides how many
    /// cores they may use between them. Left to itself rten gives every decode
    /// the whole machine, so without the second bound the first one multiplies
    /// rather than caps — see [`Decoder::with_thread_pool`].
    pub fn new(threads: usize) -> Result<Self, Error> {
        crate::model::preload()?;
        let threads = threads.max(1);
        let infer = Arc::new(crate::ThreadPool::with_num_threads(threads));

        // Room for a round of work and a little slack, so a caller submitting a
        // batch is not refused halfway through it.
        let (job_tx, job_rx) = bounded::<Job>(threads * 4);
        let (result_tx, result_rx) = bounded(threads * 4 + 16);
        let in_flight = Arc::new(AtomicUsize::new(0));

        let mut handles = Vec::with_capacity(threads);
        for n in 0..threads {
            let jobs: Receiver<Job> = job_rx.clone();
            let results = result_tx.clone();
            let counter = Arc::clone(&in_flight);
            let infer = Arc::clone(&infer);
            handles.push(
                std::thread::Builder::new()
                    .name(format!("sdroxide-deepcw-{n}"))
                    .spawn(move || {
                        let mut decoder = match Decoder::with_thread_pool(infer) {
                            Ok(d) => d,
                            Err(e) => {
                                tracing::error!("DeepCW pool thread: {e}");
                                return;
                            }
                        };
                        while let Ok(job) = jobs.recv() {
                            let decoded = decoder.decode_window(&job.window);
                            counter.fetch_sub(1, Ordering::Release);
                            if results.send((job.key, decoded)).is_err() {
                                return;
                            }
                        }
                    })
                    .map_err(|e| Error::Load(e.to_string()))?,
            );
        }

        Ok(Pool { jobs: job_tx, results: result_rx, in_flight, _threads: handles })
    }

    /// Queue a window. `false` means the pool is saturated and the job was not
    /// taken — try again next round rather than waiting on it.
    pub fn submit(&self, key: u64, window: Window) -> bool {
        self.in_flight.fetch_add(1, Ordering::Release);
        if self.jobs.try_send(Job { key, window }).is_err() {
            self.in_flight.fetch_sub(1, Ordering::Release);
            return false;
        }
        true
    }

    /// Everything finished since the last call, in completion order.
    pub fn poll(&self) -> Vec<(u64, Result<Decoded, Error>)> {
        self.results.try_iter().collect()
    }

    /// Jobs queued or being decoded right now.
    pub fn in_flight(&self) -> usize {
        self.in_flight.load(Ordering::Acquire)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_signal;

    fn drain(pool: &Pool, want: usize) -> Vec<(u64, Decoded)> {
        let mut out = Vec::new();
        for _ in 0..600 {
            for (key, decoded) in pool.poll() {
                out.push((key, decoded.expect("decode")));
            }
            if out.len() >= want {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        out
    }

    #[test]
    fn decodes_several_signals_and_keeps_them_apart() {
        let pool = Pool::new(3).expect("pool");
        let messages = ["CQ TEST DE W1AW K", "599 TU DE K6XX", "QRZ DE JA1ZZZ"];
        for (i, msg) in messages.iter().enumerate() {
            let audio = test_signal::render(msg, 22.0 + i as f32 * 4.0, 800.0, 25.0, i as u64 + 1);
            assert!(pool.submit(i as u64, Window::Audio(audio)));
        }

        let mut got = drain(&pool, messages.len());
        got.sort_by_key(|(key, _)| *key);
        assert_eq!(got.len(), messages.len(), "not every job came back");
        for (key, decoded) in got {
            let want = messages[key as usize];
            assert_eq!(decoded.normalized(), want, "job {key}");
        }
        assert_eq!(pool.in_flight(), 0);
    }

    #[test]
    fn a_saturated_pool_refuses_rather_than_blocking() {
        let pool = Pool::new(1).expect("pool");
        let audio = test_signal::render("CQ DE W1AW K", 20.0, 800.0, 25.0, 9);
        // Far more than the queue holds; some must be refused, and the call must
        // return either way.
        let refused = (0..64).filter(|i| !pool.submit(*i, Window::Audio(audio.clone()))).count();
        assert!(refused > 0, "queue is supposed to be bounded");
        let done = drain(&pool, 64 - refused);
        assert!(!done.is_empty());
    }
}
