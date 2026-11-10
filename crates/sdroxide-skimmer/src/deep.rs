//! The DeepCW front end for the skimmer.
//!
//! Every tracked station needs its own window through the model, and the model
//! wants that window as a 65-bin spectrogram on a fixed grid: 12.5 Hz bins from
//! an 80 ms window, with the carrier in the middle. The obvious way to get there
//! is a filter chain per station — mix, decimate, resample — and it costs a
//! filter chain per station.
//!
//! One transform gets it for all of them at once. A CW skimmer is already
//! looking at a wide complex baseband window; taken at the model's own
//! resolution, every station's 65 bins are already sitting in it, and lifting
//! one out is a strided copy. Whether the band holds two signals or forty, the
//! transform is the same transform.
//!
//! What that costs is exactness in frequency: a station is centred to the
//! nearest whole bin of the detector's coarser grid rather than to the hertz.
//! The model's band is 800 Hz wide and it was trained across all of it, so being
//! a few tens of hertz off centre is not something it notices. Keeping the
//! centre *fixed* does matter, and an integer bin is fixed — an interpolated one
//! would wander as the estimate refined and smear the spectrogram it is meant to
//! be building.

use std::collections::HashMap;
use std::collections::VecDeque;
use std::sync::Arc;

use rustfft::{Fft, FftPlanner};
use sdroxide_deepcw::{BINS, CENTER_BIN, Window};
use sdroxide_dsp::Complex32 as C32;

/// Rolling spectrogram kept per station.
///
/// The model has no time subsampling, so what this costs is linear in its length
/// with the attention term growing on top: measured, one decode is 65 ms of CPU
/// at 12 s, 50 ms at 10 s and 39 ms at 8 s.
///
/// What it buys is not accuracy. Scored against the same signal from +20 dB down
/// to −9 dB, copy is flat from five seconds to twenty — the model reads what it
/// is given and does not need a run-up. What it buys is *how much of a
/// transmission the spot can show*, because this is also the most the operator
/// will ever see of one station: a contest CQ at 18 WPM runs the better part of
/// twenty seconds, and the window is a rolling one, so every second cut off it
/// is a second that has already rolled away by the time the transmission ends
/// and the decode that reads it is taken. Shortening this to ten seconds
/// measurably ate callsigns off the front of an ordinary CQ, and to eight it ate
/// more. The cost is real — 65 ms of CPU a decode against 50 at ten seconds —
/// but it is the wrong place to take it from.
const WINDOW_S: f64 = 12.0;
/// Don't decode a station until it has been heard this long. The model's own
/// documentation puts its floor at five seconds.
const MIN_S: f64 = 5.0;
/// How often each station is re-decoded.
///
/// The model re-reads the station's whole window every time, so this and
/// [`WINDOW_S`] together set the bill: the ratio between them is how many times
/// over every second of audio is put through the model. At 12 s and 2 s that was
/// six, and the first eight of those twelve seconds had already been read twice
/// before. Four seconds still leaves consecutive windows overlapping by eight, so
/// nothing goes unseen and no context is lost — the only cost is that new text
/// reaches the spot list up to two seconds later, which for a band-wide read-out
/// nobody is watching character by character is not a cost at all.
const INTERVAL_S: f64 = 4.0;
/// How long a station must have been off the air to count as having stopped.
///
/// The caller's `active` flag is barely debounced, so this is where the decision
/// is actually made. Long enough to sit through a word space at the slowest speed
/// the skimmer reports — seven units at 10 WPM is 840 ms — so an ordinary gap
/// between words does not read as the end of a transmission.
const QUIET_MS: f64 = 1_000.0;
/// Silence left on the end of a window when a station has stopped sending.
///
/// Everything past this is trimmed off, up to [`MAX_TRIM_MS`]. The model reads a
/// whole window at once and its answer replaces the station's text outright, so
/// what follows the signal is not free context: a transmission with seconds of
/// nothing stuck on the end reads *worse* than the same transmission alone, and
/// the worse answer overwrites the better one. Enough to make the ending
/// unambiguous is all it needs.
///
/// Trimming the end is safe in a way that not *recording* the silence is not —
/// what is left is still one continuous stretch of the band, where skipping the
/// quiet stretches would splice two transmissions together.
const TRAIL_KEEP_MS: f64 = 500.0;
/// The most that may ever be trimmed: exactly the silence this front end waited
/// through to decide the station had stopped.
///
/// Detection on a marginal signal is patchy, so `last_active_ms` can lag the
/// truth by more than it looks — and every millisecond of that lag is a
/// millisecond of real signal the trim would otherwise eat. Bounding it to the
/// wait means the worst case is losing the silence we already knew about.
const MAX_TRIM_MS: f64 = QUIET_MS + TAIL_MS;
/// How soon after a station stops to take the decode that captures its tail.
///
/// Short, and deliberately not [`INTERVAL_S`]: the last thing a station sends is
/// its callsign, and waiting a full interval for it means a burst shorter than
/// the interval is only ever read half-finished.
const TAIL_MS: f64 = 500.0;
// How many stations are buffered and decoded at once is the operator's, and
// arrives through `SkimmerSettings::cw_slots`. The detector will happily track
// far more than any of those figures on a busy band; the ones past the cap keep
// their spot marker and their frequency and simply carry no text, which is a
// real limit and is logged when it bites.

struct Buffered {
    /// Centre bin in the wide transform. Fixed for the life of the track.
    bin: i64,
    /// Position in the caller's strongest-first list, refreshed every `sync`.
    rank: usize,
    /// `[frames][BINS]` row-major, oldest first.
    rows: VecDeque<f32>,
    last_submit_ms: f64,
    /// When this station was last heard keying, on *this* struct's clock.
    ///
    /// It has to be stamped here rather than passed in as a timestamp. The
    /// detector and this front end each keep their own `now_ms`, advanced by
    /// their own hop over the samples they have consumed, and [`Self::process`]
    /// returns early without advancing at all when there is nothing to buffer for
    /// — so on a quiet band the two clocks drift apart without bound. Comparing a
    /// detector timestamp against `last_submit_ms` would be arithmetic between
    /// two different timebases. The caller says only whether the station is
    /// keying *now*; the time that happened is read off the local clock.
    last_active_ms: f64,
    /// Whether the decode that reads the end of a transmission has been taken.
    ///
    /// Set whenever a window is submitted and cleared whenever the station is
    /// heard keying, which is what makes the tail decode fire exactly once. A
    /// comparison between `last_active_ms` and `last_submit_ms` cannot do the job:
    /// `sync` stamps the first before [`DeepFront::process`] advances the clock
    /// and `mark_submitted` stamps the second after, so for a station that is
    /// sending steadily the two land in whichever order the block boundaries
    /// happen to fall.
    tail_taken: bool,
    /// Set once the track has been submitted, so the first decode waits for
    /// `MIN_S` but later ones only wait for `INTERVAL_S`.
    submitted: bool,
    /// A window for this station is with the pool right now.
    ///
    /// At most one may be, and that is the point. The pool runs several threads
    /// and finishes jobs in whatever order they happen to finish, so two windows
    /// of the same station in flight together can land backwards — and since each
    /// result replaces the station's text wholesale, an older, shorter window
    /// would overwrite a newer one. How often that happens depends on how loaded
    /// the machine is, which is the worst way for a decoder to be wrong.
    pending: bool,
}

pub struct DeepFront {
    fft: Arc<dyn Fft<f32>>,
    size: usize,
    hop: usize,
    /// Wide-transform bins per bin of the detector's coarser grid.
    bins_per_coarse: f64,
    /// Maps the wide transform's magnitudes onto the scale the model was trained
    /// on. Both a carrier and the noise around it scale with the number of
    /// samples in the window, and the window is the same 80 ms either way, so
    /// one factor puts both right — see the note on `Window::Spectrogram`.
    scale: f32,
    window: Vec<f32>,
    inbuf: Vec<C32>,
    read_pos: usize,
    scratch: Vec<C32>,
    tracks: HashMap<u64, Buffered>,
    max_tracks: usize,
    max_rows: usize,
    now_ms: f64,
    frame_ms: f64,
    /// Logged once per skimmer rather than once per frame.
    warned_full: bool,
    /// Spectrograms materialised, for the cost harness. `Cell` because `due`
    /// takes `&self`.
    #[cfg(test)]
    windows_built: std::cell::Cell<u64>,
}

impl DeepFront {
    /// `skim_rate` is the wide window's sample rate; `coarse_bins` is the size
    /// of the detector's own transform, whose bin numbering the caller uses.
    pub fn new(skim_rate: f64, coarse_bins: usize, max_tracks: usize) -> Self {
        let size = (skim_rate / sdroxide_deepcw::BIN_HZ).round().max(64.0) as usize;
        let hop = (skim_rate * sdroxide_deepcw::FRAME_S).round().max(1.0) as usize;
        let fft = FftPlanner::<f32>::new().plan_fft_forward(size);
        // Periodic Hann, matching the model's own front end.
        let window: Vec<f32> = (0..size)
            .map(|i| 0.5 * (1.0 - (std::f64::consts::TAU * i as f64 / size as f64).cos()) as f32)
            .collect();
        let frame_ms = hop as f64 / skim_rate * 1000.0;
        DeepFront {
            fft,
            size,
            hop,
            bins_per_coarse: size as f64 / coarse_bins as f64,
            scale: (sdroxide_deepcw::SAMPLE_RATE / skim_rate) as f32,
            window,
            inbuf: Vec::new(),
            read_pos: 0,
            scratch: vec![C32::default(); size],
            tracks: HashMap::new(),
            max_tracks: max_tracks.max(1),
            max_rows: (WINDOW_S / sdroxide_deepcw::FRAME_S) as usize,
            now_ms: 0.0,
            frame_ms,
            warned_full: false,
            #[cfg(test)]
            windows_built: std::cell::Cell::new(0),
        }
    }

    /// Spectrograms materialised since construction, for the cost harness.
    #[cfg(test)]
    pub fn windows_built(&self) -> u64 {
        self.windows_built.get()
    }

    pub fn reset(&mut self) {
        self.tracks.clear();
        self.inbuf.clear();
        self.read_pos = 0;
    }

    /// Change how many stations are read at once.
    ///
    /// Lowering it drops the stations with the least buffered, which are the ones
    /// with the least to lose. [`Self::sync`] deliberately never evicts — a
    /// station part-way through a window would lose everything it had — but this
    /// is the operator asking, not a stronger signal arriving.
    pub fn set_max_tracks(&mut self, n: usize) {
        let n = n.max(1);
        if n == self.max_tracks {
            return;
        }
        self.max_tracks = n;
        // A raise is worth reporting again if it later bites at the new figure.
        self.warned_full = false;
        while self.tracks.len() > n {
            let Some(&drop) =
                self.tracks.iter().min_by_key(|(id, t)| (t.rows.len(), **id)).map(|(id, _)| id)
            else {
                break;
            };
            self.tracks.remove(&drop);
        }
    }

    /// Tell the front end which stations exist, strongest first, and which of
    /// them are keying right now.
    ///
    /// Stations that have gone away lose their buffer; new ones get one while
    /// there is room. Existing buffers are never evicted for a stronger arrival —
    /// a station part-way through a window would lose everything it had, and
    /// churning the strongest few would mean nobody ever accumulates enough to
    /// decode.
    pub fn sync(&mut self, live: &[(u64, i64, bool)]) {
        self.tracks.retain(|id, _| live.iter().any(|(live_id, _, _)| live_id == id));
        for (rank, &(id, coarse_bin, active)) in live.iter().enumerate() {
            if let Some(t) = self.tracks.get_mut(&id) {
                t.rank = rank;
                if active {
                    t.last_active_ms = self.now_ms;
                    t.tail_taken = false;
                }
                continue;
            }
            if self.tracks.len() >= self.max_tracks {
                if !self.warned_full {
                    self.warned_full = true;
                    tracing::info!(
                        max = self.max_tracks,
                        "more CW signals than DeepCW slots; the weakest carry no text"
                    );
                }
                break;
            }
            self.tracks.insert(
                id,
                Buffered {
                    bin: (coarse_bin as f64 * self.bins_per_coarse).round() as i64,
                    rank,
                    rows: VecDeque::with_capacity(self.max_rows * BINS),
                    last_submit_ms: self.now_ms,
                    last_active_ms: self.now_ms,
                    tail_taken: false,
                    submitted: false,
                    pending: false,
                },
            );
        }
    }

    /// Feed the same IQ the detector sees.
    pub fn process(&mut self, iq: &[C32]) {
        if self.tracks.is_empty() {
            // Nothing to buffer for; keep the cursor from growing without bound.
            self.inbuf.clear();
            self.read_pos = 0;
            return;
        }
        self.inbuf.extend_from_slice(iq);
        while self.read_pos + self.size <= self.inbuf.len() {
            for i in 0..self.size {
                self.scratch[i] = self.inbuf[self.read_pos + i] * self.window[i];
            }
            self.fft.process(&mut self.scratch);
            self.append_rows();
            self.read_pos += self.hop;
            self.now_ms += self.frame_ms;
        }
        if self.read_pos >= self.size {
            self.inbuf.drain(..self.read_pos);
            self.read_pos = 0;
        }
    }

    /// Lift each station's 65 bins out of this frame.
    fn append_rows(&mut self) {
        let n = self.size as i64;
        for track in self.tracks.values_mut() {
            // Every frame, for every station, whether it is sending or not.
            //
            // Skipping the quiet ones looks like an obvious saving — the buffer
            // stops rolling, so the decode taken after a transmission still has
            // the start of it — and on a signal that never stops it even tests
            // well. On the air it destroys the decode: leaving the silence out
            // splices the *next* transmission onto the back of the previous one,
            // so a gap of half a minute reaches the model as a gap of one second.
            // This model reads Morse timing rather than fitting it, and a window
            // whose timing is a lie decodes to fragments — measured on 40 m, a
            // station keyed for fifty seconds came back as "I UR". The buffer has
            // to be a continuous stretch of the band, silences and all.
            for d in 0..BINS {
                let src = track.bin + d as i64 - CENTER_BIN as i64;
                // Outside the wide window there is nothing to copy; a station
                // that close to the edge sees silence beside it.
                // log1p of the scaled magnitude — the model's normalisation —
                // taken here rather than over the whole spectrum first, because
                // thirty-two stations need two thousand of these fifteen thousand
                // bins and the rest were being transformed to be thrown away.
                let v = if src > -n / 2 && src < n / 2 {
                    let z = self.scratch[src.rem_euclid(n) as usize];
                    (z.norm() * self.scale).ln_1p()
                } else {
                    0.0
                };
                track.rows.push_back(v);
            }
            while track.rows.len() > self.max_rows * BINS {
                track.rows.drain(..BINS);
            }
        }
    }

    /// The window for one station, ready for the model.
    ///
    /// Separate from [`Self::due`] because building it is a 130 kB copy and the
    /// pool may refuse the job: `due` used to return the windows themselves, so
    /// every station past the one the pool turned down had its spectrogram built
    /// and dropped on the floor — and then built again on the next block, a
    /// hundred-odd times a second, for as long as the pool stayed behind. The
    /// waste arrived exactly when the machine could least afford it.
    pub fn window(&self, id: u64) -> Option<Window> {
        let t = self.tracks.get(&id)?;
        #[cfg(test)]
        self.windows_built.set(self.windows_built.get() + 1);
        // Cut the silence since this station stopped, so the window ends near the
        // end of its transmission rather than wherever the clock happens to be.
        // Zero while it is still sending, and never below the model's own floor.
        let quiet = (self.now_ms - t.last_active_ms - TRAIL_KEEP_MS).clamp(0.0, MAX_TRIM_MS);
        let drop = (quiet / self.frame_ms) as usize * BINS;
        let floor = (MIN_S / sdroxide_deepcw::FRAME_S) as usize * BINS;
        let keep = t.rows.len().saturating_sub(drop).max(floor.min(t.rows.len()));
        Some(Window::Spectrogram(t.rows.iter().take(keep).copied().collect()))
    }

    /// Stations with enough audio buffered that they are worth decoding again,
    /// in the order they should be offered to the pool.
    ///
    /// A station that has stopped keying is not one of them. The model re-reads
    /// the whole window and its result *replaces* the station's text, so putting
    /// a window of the same characters followed by more silence through it again
    /// cannot add anything — it can only lose the oldest characters as they roll
    /// off the front, which is how a finished transmission used to erode from the
    /// spot list over the seconds before its track was pruned.
    ///
    /// So each station is read on the interval while it is sending, once more
    /// shortly after it stops — that last one is what catches the callsign at the
    /// end of a burst — and then not again until it is heard keying afresh. It is
    /// [`Self::sync`]'s `active` flag that makes this terminate: once the tail
    /// decode is taken, `last_submit_ms` sits ahead of `last_active_ms` and stays
    /// there for as long as the station is quiet.
    pub fn due(&self) -> Vec<u64> {
        let mut out: Vec<(bool, usize, u64)> = self
            .tracks
            .iter()
            .filter(|(_, t)| {
                let seconds = t.rows.len() as f64 / BINS as f64 * sdroxide_deepcw::FRAME_S;
                if t.pending || seconds < MIN_S {
                    return false;
                }
                let since_submit = self.now_ms - t.last_submit_ms;
                if !t.submitted {
                    return since_submit >= MIN_S * 1000.0;
                }
                if self.now_ms - t.last_active_ms < QUIET_MS {
                    return since_submit >= INTERVAL_S * 1000.0;
                }
                !t.tail_taken && since_submit >= TAIL_MS
            })
            .map(|(id, t)| (t.submitted, t.rank, *id))
            .collect();
        // Stations that have never been read come first, then the strongest.
        // Without this the order is the `HashMap`'s, which throws away the
        // strongest-first ordering the caller went to the trouble of computing —
        // so when there are more stations due than the pool will take, which ones
        // get read is decided by hash order.
        out.sort_unstable();
        out.into_iter().map(|(_, _, id)| id).collect()
    }

    /// Note that `id` was taken by the pool. A refused job is simply not marked,
    /// so it comes up again next round.
    pub fn mark_submitted(&mut self, id: u64) {
        if let Some(t) = self.tracks.get_mut(&id) {
            t.last_submit_ms = self.now_ms;
            t.submitted = true;
            t.pending = true;
            // Only a decode taken after the station went quiet has read its tail.
            // One taken while it was still sending has not, and must not stop the
            // tail decode from happening when it does stop.
            t.tail_taken = self.now_ms - t.last_active_ms >= QUIET_MS;
        }
    }

    /// A station's window came back, so it may be given another.
    pub fn complete(&mut self, id: u64) {
        if let Some(t) = self.tracks.get_mut(&id) {
            t.pending = false;
        }
    }

    /// Seconds of spectrogram buffered for a station, for tests and logging.
    #[cfg(test)]
    pub fn buffered_s(&self, id: u64) -> f64 {
        self.tracks
            .get(&id)
            .map_or(0.0, |t| t.rows.len() as f64 / BINS as f64 * sdroxide_deepcw::FRAME_S)
    }

    #[cfg(test)]
    pub fn advance_for_test(&mut self, ms: f64) {
        self.now_ms += ms;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The cap the tests run at, standing in for whatever the operator set.
    const CAP: usize = 32;

    fn front() -> DeepFront {
        DeepFront::new(192_000.0, 4096, CAP)
    }

    #[test]
    fn the_transform_lands_on_the_models_grid() {
        let f = front();
        // 12.5 Hz bins from an 80 ms window is the whole requirement.
        assert_eq!(f.size, 15_360);
        assert_eq!(192_000.0 / f.size as f64, sdroxide_deepcw::BIN_HZ);
        assert_eq!(f.size as f64 / 192_000.0, sdroxide_deepcw::WINDOW_S);
        assert_eq!(f.hop, 2_880);
        assert_eq!(f.hop as f64 / 192_000.0, sdroxide_deepcw::FRAME_S);
    }

    /// Advance the clock the way the caller does — a `sync` every block — so a
    /// station that is still sending goes on saying so. Jumping the clock without
    /// syncing would look like a station that stopped at the start of the jump.
    fn run_for(f: &mut DeepFront, ms: f64, active: bool) {
        let mut left = ms;
        while left > 0.0 {
            let step = 50.0f64.min(left);
            f.advance_for_test(step);
            f.sync(&[(1, 0, active)]);
            left -= step;
        }
    }

    /// Build a station's buffer the same way, feeding real blocks so the rows
    /// fill as the clock moves rather than in one jump.
    fn fill(f: &mut DeepFront, hops: usize, active: bool) {
        let block = vec![C32::new(0.01, 0.0); f.hop * 10];
        for _ in 0..hops / 10 {
            f.sync(&[(1, 0, active)]);
            f.process(&block);
        }
    }

    #[test]
    fn tracks_are_added_and_dropped_with_the_detector() {
        let mut f = front();
        f.sync(&[(1, 100, true), (2, -200, true)]);
        assert_eq!(f.tracks.len(), 2);
        f.sync(&[(2, -200, true)]);
        assert_eq!(f.tracks.len(), 1);
        assert!(f.tracks.contains_key(&2));
    }

    #[test]
    fn buffering_is_capped_and_stations_past_it_are_refused() {
        let mut f = front();
        let live: Vec<(u64, i64, bool)> =
            (0..CAP as u64 + 8).map(|i| (i, i as i64 * 10, true)).collect();
        f.sync(&live);
        assert_eq!(f.tracks.len(), CAP);
    }

    #[test]
    fn lowering_the_cap_drops_the_stations_with_least_to_lose() {
        let mut f = front();
        // Give track 1 a real buffer and the others nothing.
        fill(&mut f, 400, true);
        f.sync(&[(1, 0, true), (2, 100, true), (3, 200, true)]);
        assert_eq!(f.tracks.len(), 3);

        f.set_max_tracks(1);
        assert_eq!(f.tracks.len(), 1);
        assert!(f.tracks.contains_key(&1), "dropped the one that had a window built up");
    }

    #[test]
    fn a_buffer_does_not_grow_past_its_window() {
        let mut f = front();
        fill(&mut f, 1600, true);
        assert!(f.buffered_s(1) <= WINDOW_S + 0.1, "buffered {:.1}s", f.buffered_s(1));
        assert!(f.buffered_s(1) >= WINDOW_S - 0.1, "buffered only {:.1}s", f.buffered_s(1));
    }

    #[test]
    fn nothing_is_due_before_the_minimum_window() {
        let mut f = front();
        fill(&mut f, 100, true); // ~1.5 s
        assert!(f.due().is_empty());
        fill(&mut f, 300, true); // past 5 s
        assert_eq!(f.due().len(), 1);
    }

    #[test]
    fn the_window_stays_a_continuous_stretch_of_the_band() {
        // Silence goes into the buffer like everything else. Leaving it out to
        // stop the window rolling looks like a saving, and against a signal that
        // never pauses it even tests well — but it splices the next transmission
        // onto the back of the previous one, so a gap of half a minute reaches
        // the model as a gap of one second. This model reads Morse timing rather
        // than fitting it, and on 40 m that turned a station keyed for fifty
        // seconds into the text "I UR".
        let mut f = front();
        fill(&mut f, 200, true);
        let sending = f.buffered_s(1);
        assert!(sending > 2.0, "nothing buffered while the station was sending");

        // Off the air for a good while: the buffer must go on filling.
        fill(&mut f, 200, false);
        let quiet = f.buffered_s(1);
        assert!(
            quiet > sending + 2.0 || quiet >= WINDOW_S - 0.1,
            "silence was skipped: {sending:.1}s became {quiet:.1}s"
        );
    }

    #[test]
    fn a_submitted_station_waits_its_interval() {
        let mut f = front();
        fill(&mut f, 400, true);
        assert_eq!(f.due().len(), 1);
        f.mark_submitted(1);
        assert!(f.due().is_empty(), "resubmitted immediately");
        run_for(&mut f, INTERVAL_S * 1000.0 + 1.0, true);
        assert!(f.due().is_empty(), "resubmitted while its window was still out");
        f.complete(1);
        assert_eq!(f.due().len(), 1);
    }

    #[test]
    fn only_one_window_per_station_is_ever_out() {
        // Two in flight together can finish backwards, and the older one would
        // overwrite the newer station text.
        let mut f = front();
        fill(&mut f, 400, true);
        assert_eq!(f.due().len(), 1);
        f.mark_submitted(1);
        for _ in 0..5 {
            run_for(&mut f, INTERVAL_S * 1000.0 + 1.0, true);
            assert!(f.due().is_empty());
        }
        f.complete(1);
        assert_eq!(f.due().len(), 1);
    }

    #[test]
    fn asking_what_is_due_does_not_build_anything() {
        // `due` runs on every IQ block — a hundred-odd times a second — and the
        // pool takes a job only now and then. Building a spectrogram to answer
        // the question would be the most expensive thing the skimmer does.
        let mut f = front();
        fill(&mut f, 400, true);
        for _ in 0..50 {
            assert_eq!(f.due().len(), 1);
        }
        assert_eq!(f.windows_built(), 0, "due() built a window it was not asked for");
        f.window(1).expect("a window on request");
        assert_eq!(f.windows_built(), 1);
    }

    #[test]
    fn unread_stations_are_offered_before_re_reads_and_then_the_strongest_first() {
        let mut f = front();
        // The caller hands them over strongest first; ranks follow that order.
        let live = |active: bool| vec![(10, 0, active), (20, 100, active), (30, 200, active)];
        let block = vec![C32::new(0.01, 0.0); f.hop * 10];
        for _ in 0..40 {
            f.sync(&live(true));
            f.process(&block);
        }
        assert_eq!(f.due(), vec![10, 20, 30], "not offered strongest first");

        // The strongest has been read; the two that have not come ahead of it.
        f.mark_submitted(10);
        f.complete(10);
        assert_eq!(f.due(), vec![20, 30], "the one just submitted came straight back");
        // Past its interval, so it is due again — but behind the two that have
        // still never been read.
        for _ in 0..40 {
            f.sync(&live(true));
            f.process(&block);
        }
        assert_eq!(f.due(), vec![20, 30, 10], "a re-read cut ahead of an unread station");
    }

    #[test]
    fn a_station_that_stops_is_read_once_more_and_then_left_alone() {
        // The terminating property. A finished transmission has to be read one
        // last time — that is where the callsign at the end of a burst comes
        // from — but only once, however long its track lingers afterwards.
        let mut f = front();
        fill(&mut f, 400, true);
        assert_eq!(f.due().len(), 1, "the first look");
        f.mark_submitted(1);
        f.complete(1);

        // It carries on sending for another interval and is read again.
        run_for(&mut f, INTERVAL_S * 1000.0 + 1.0, true);
        assert_eq!(f.due().len(), 1, "still sending, so still on the interval");
        f.mark_submitted(1);
        f.complete(1);

        // Now it stops. One more decode, and it must not have to wait a whole
        // interval for it.
        run_for(&mut f, QUIET_MS + TAIL_MS, false);
        assert_eq!(f.due().len(), 1, "no decode captured the end of the transmission");
        f.mark_submitted(1);
        f.complete(1);

        // And then nothing, for as long as it stays quiet.
        for _ in 0..20 {
            run_for(&mut f, INTERVAL_S * 1000.0 + 1.0, false);
            assert!(f.due().is_empty(), "a silent station was decoded again");
        }
    }

    #[test]
    fn a_station_that_keys_again_is_read_again() {
        let mut f = front();
        fill(&mut f, 400, true);
        f.due();
        f.mark_submitted(1);
        f.complete(1);
        // Off the air long enough to take its tail decode and settle.
        run_for(&mut f, QUIET_MS + TAIL_MS, false);
        assert_eq!(f.due().len(), 1);
        f.mark_submitted(1);
        f.complete(1);
        run_for(&mut f, 30_000.0, false);
        assert!(f.due().is_empty());

        // Back on the air: read on the interval again, with no extra delay for
        // having been quiet.
        run_for(&mut f, INTERVAL_S * 1000.0 + 1.0, true);
        assert_eq!(f.due().len(), 1, "a station that came back was not picked up");
    }

    #[test]
    fn the_two_clocks_are_never_compared() {
        // `process` does not advance `now_ms` while there is nothing to buffer
        // for, so this clock and the detector's drift apart without bound on a
        // quiet band. Activity therefore arrives as a flag and is stamped here.
        // If anyone ever swaps that for a detector timestamp, this fails.
        let mut f = front();
        let idle = vec![C32::new(0.01, 0.0); f.hop * 100];
        f.process(&idle); // no tracks: the clock must not move
        assert_eq!(f.now_ms, 0.0, "the deep clock ran while nothing was buffered");

        fill(&mut f, 400, true);
        assert_eq!(f.due().len(), 1, "a track added after a long quiet spell never came due");
    }

    #[test]
    fn a_carrier_lands_in_the_middle_of_the_row() {
        // A tone at a known offset must come out at the band centre, which is
        // the whole point of the strided copy.
        let mut f = front();
        let coarse_bin = 100i64;
        f.sync(&[(7, coarse_bin, true)]);
        let hz = coarse_bin as f64 * 192_000.0 / 4096.0;
        let n = f.hop * 400;
        let iq: Vec<C32> = (0..n)
            .map(|i| {
                let ph = std::f64::consts::TAU * hz * i as f64 / 192_000.0;
                C32::new(ph.cos() as f32, ph.sin() as f32)
            })
            .collect();
        // A sync per block, as the caller does, so the carrier counts as keying
        // for the whole of it.
        for block in iq.chunks(f.hop * 10) {
            f.sync(&[(7, coarse_bin, true)]);
            f.process(block);
        }

        let id = f.due().into_iter().next().expect("due");
        let Some(Window::Spectrogram(rows)) = f.window(id) else {
            panic!("expected a spectrogram");
        };
        let frames = rows.len() / BINS;
        let row = &rows[(frames / 2) * BINS..][..BINS];
        let peak = row.iter().enumerate().max_by(|a, b| a.1.total_cmp(b.1)).unwrap().0;
        assert_eq!(peak, CENTER_BIN, "carrier landed at bin {peak}");
    }
}
