//! VOR — the VHF omnidirectional range, 108.0 to 117.95 MHz.
//!
//! A VOR beacon amplitude-modulates its carrier with three things at once:
//!
//! * a **30 Hz variable signal**, whose phase is the bearing of the receiver
//!   from the beacon (the antenna pattern rotates 30 times a second);
//! * a **9960 Hz subcarrier**, frequency-modulated at 30 Hz with a 480 Hz
//!   deviation: the **reference**, in phase with the variable signal at
//!   magnetic north;
//! * the **ident**, a 1020 Hz tone keyed in Morse every ten seconds or so.
//!
//! So the radial is the lag of the 30 Hz AM component behind the 30 Hz of the
//! FM subcarrier. [`VorDecoder`] takes the output of an AM detector — the
//! composite, before any audio filtering — and returns that lag, how strong each
//! of the two signals was, and the ident.
//!
//! # The audio must be the plain envelope
//!
//! Both phases are measured against the same clock, so anything between the
//! detector and this decoder that delays the 30 Hz (a high-pass for DC, a
//! de-emphasis, an audio low-pass) moves the radial by the same angle.
//! [`VorDecoder::set_phase_offset_deg`] corrects a fixed delay of that kind.
//!
//! # Sources
//!
//! ICAO Annex 10 Volume I, 3.3.5 (the signal in space), and the Morse code of
//! ITU-R M.1677-1. Nothing is taken from another decoder.
//!
//! # Not a navigation instrument
//!
//! The radial is where the receiver is from the beacon, **magnetic** north at
//! the beacon's own declination, measured through whatever the signal met on
//! the way. Fine for a map and for curiosity; not for flying.

pub mod morse;

use std::f64::consts::TAU;

use morse::Run;

/// The lowest audio rate that carries the subcarrier and its sidebands.
pub const MIN_SAMPLE_RATE: u32 = 24_000;

/// The rotating-pattern frequency, Hz.
pub const VAR_HZ: f64 = 30.0;
/// The reference subcarrier, Hz.
pub const SUBCARRIER_HZ: f64 = 9960.0;
/// The reference's peak frequency deviation, Hz.
pub const NOMINAL_DEVIATION_HZ: f64 = 480.0;
/// The ident tone, Hz.
pub const IDENT_HZ: f64 = 1020.0;

/// Taps of the filter that isolates the subcarrier; odd, so it has a whole
/// number of samples of delay.
const REF_TAPS: usize = 301;
/// Its cut-off: past the 480 Hz deviation and the first sidebands, well short
/// of the nearest voice component, which lands 6960 Hz away.
const REF_CUTOFF_HZ: f64 = 1000.0;
/// The rate the FM discriminator runs at, about.
const REF_WORK_RATE_HZ: f64 = 4000.0;
/// Blocks per second; the radial is refreshed once a block.
const BLOCKS_PER_S: usize = 10;
/// Seconds of signal one radial is measured over. Thirty whole cycles of the
/// 30 Hz, so the two correlators have nothing to leak.
const WINDOW_BLOCKS: usize = BLOCKS_PER_S;
/// Ident detector blocks per second.
const IDENT_BLOCKS_PER_S: usize = 50;
/// Silence that ends one ident, in ident blocks (0.9 s). The longest gap inside
/// one is a word gap at the slowest speed, 0.5 s.
const IDENT_END_BLOCKS: u32 = 45;
/// A tone keyed down longer than this is not Morse (1.0 s; a dash at 5 wpm is
/// 0.72 s).
const IDENT_MAX_ON_BLOCKS: u32 = 50;
/// Runs in one ident, beyond which it is noise: five characters of five marks.
const IDENT_MAX_RUNS: usize = 64;

/// What the 30 Hz signals measured over the last second say.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Reading {
    /// Bearing of the receiver from the beacon, degrees clockwise from the
    /// beacon's north, `0.0 <= r < 360.0`.
    pub radial_deg: f32,
    /// Depth of the 30 Hz amplitude modulation, as a fraction of the carrier.
    /// Nominally 0.30.
    pub var_depth: f32,
    /// Measured peak deviation of the subcarrier, Hz. Nominally 480.
    pub ref_deviation_hz: f32,
    /// Depth of the 9960 Hz subcarrier, as a fraction of the carrier. Nominally
    /// 0.30.
    pub ref_depth: f32,
    /// Whether both are strong enough for `radial_deg` to mean anything.
    pub locked: bool,
}

/// What one [`VorDecoder::push`] produced.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Update {
    /// The newest reading, if a tenth of a second has passed since the last.
    pub reading: Option<Reading>,
    /// An ident confirmed by this call and different from the one before.
    pub new_ident: Option<String>,
}

/// Why a decoder could not be built.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VorError {
    /// The audio rate does not reach the subcarrier.
    SampleRateTooLow { got: u32 },
}

impl std::fmt::Display for VorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VorError::SampleRateTooLow { got } => {
                write!(f, "sample rate {got} Hz is below the {MIN_SAMPLE_RATE} Hz a VOR needs")
            }
        }
    }
}

impl std::error::Error for VorError {}

/// Sums of one block, kept so the window can slide a block at a time.
#[derive(Clone, Copy, Default)]
struct Block {
    var_re: f64,
    var_im: f64,
    ref_re: f64,
    ref_im: f64,
    sum_x: f64,
    sum_ref_mag: f64,
    n_in: u32,
    n_ref: u32,
}

/// The ident's three Goertzel bins: the tone and a bin either side for the
/// noise it is measured against.
struct IdentBins {
    coeff: [f64; IDENT_BINS],
    s1: [f64; IDENT_BINS],
    s2: [f64; IDENT_BINS],
}

/// Offsets from the ident tone of the Goertzel bins; the tone itself is the
/// middle one, the other four are the noise it is measured against.
const IDENT_OFFSETS_HZ: [f64; IDENT_BINS] = [-200.0, -120.0, 0.0, 120.0, 200.0];
const IDENT_BINS: usize = 5;
/// Shortest mark or space believed, in ident blocks (60 ms): a dot at 20 words a
/// minute. Anything shorter is the threshold being crossed by noise.
const IDENT_MIN_RUN: u32 = 3;

/// The decoder. One per receiver; feed it the AM detector's output.
pub struct VorDecoder {
    fs: f64,
    phase_offset_deg: f64,
    ident_snr: f64,

    // Counting.
    n: u64,
    block_len: u32,
    in_block: u32,

    // 30 Hz variable path.
    blocks: [Block; WINDOW_BLOCKS],
    block_at: usize,
    cur: Block,
    blocks_seen: usize,

    // Reference path.
    taps: Vec<f64>,
    ring_re: Vec<f64>,
    ring_im: Vec<f64>,
    ring_at: usize,
    mixer_phase: f64,
    mixer_step: f64,
    decim: u32,
    since_out: u32,
    work_rate: f64,
    prev: Option<(f64, f64)>,

    // Ident path.
    ident_len: usize,
    ident_at: usize,
    hann: Vec<f64>,
    bins: IdentBins,
    dc_x1: f64,
    dc_y1: f64,
    carrier: f64,
    key_on: bool,
    run_len: u32,
    runs: Vec<Run>,
    candidate: Option<String>,
    confirmed: Option<String>,

    latest: Option<Reading>,
}

impl VorDecoder {
    /// A decoder for audio at `sample_rate_hz`.
    pub fn new(sample_rate_hz: u32) -> Result<Self, VorError> {
        if sample_rate_hz < MIN_SAMPLE_RATE {
            return Err(VorError::SampleRateTooLow { got: sample_rate_hz });
        }
        let fs = sample_rate_hz as f64;
        let decim = ((fs / REF_WORK_RATE_HZ).floor() as u32).max(1);
        let ident_len = (sample_rate_hz as usize / IDENT_BLOCKS_PER_S).max(8);
        let bin = |hz: f64| 2.0 * (TAU * hz / fs).cos();
        Ok(Self {
            fs,
            phase_offset_deg: 0.0,
            ident_snr: 6.0,
            n: 0,
            block_len: (sample_rate_hz / BLOCKS_PER_S as u32).max(1),
            in_block: 0,
            blocks: [Block::default(); WINDOW_BLOCKS],
            block_at: 0,
            cur: Block::default(),
            blocks_seen: 0,
            taps: lowpass_taps(REF_TAPS, REF_CUTOFF_HZ, fs),
            ring_re: vec![0.0; 2 * REF_TAPS],
            ring_im: vec![0.0; 2 * REF_TAPS],
            ring_at: 0,
            mixer_phase: 0.0,
            mixer_step: TAU * SUBCARRIER_HZ / fs,
            decim,
            since_out: 0,
            work_rate: fs / decim as f64,
            prev: None,
            ident_len,
            ident_at: 0,
            hann: (0..ident_len)
                .map(|i| 0.5 - 0.5 * (TAU * (i as f64 + 0.5) / ident_len as f64).cos())
                .collect(),
            dc_x1: 0.0,
            dc_y1: 0.0,
            carrier: 0.0,
            bins: IdentBins {
                coeff: IDENT_OFFSETS_HZ.map(|o| bin(IDENT_HZ + o)),
                s1: [0.0; IDENT_BINS],
                s2: [0.0; IDENT_BINS],
            },
            key_on: false,
            run_len: 0,
            runs: Vec::new(),
            candidate: None,
            confirmed: None,
            latest: None,
        })
    }

    /// The audio rate this decoder was built for.
    pub fn sample_rate_hz(&self) -> u32 {
        self.fs as u32
    }

    /// Add `deg` to every radial from now on: the lag a filter between the
    /// detector and here put on the 30 Hz. Positive when the variable signal
    /// arrives late.
    pub fn set_phase_offset_deg(&mut self, deg: f32) {
        self.phase_offset_deg = deg as f64;
    }

    /// How far above the noise beside it the 1020 Hz tone must stand for a
    /// mark, as a power ratio. Lower hears weaker idents and invents more.
    /// Default 6 (7.8 dB).
    pub fn set_ident_snr(&mut self, ratio: f32) {
        self.ident_snr = (ratio as f64).max(1.5);
    }

    /// The newest reading, if one has been made.
    pub fn reading(&self) -> Option<Reading> {
        self.latest
    }

    /// The last ident confirmed (heard twice running).
    pub fn ident(&self) -> Option<&str> {
        self.confirmed.as_deref()
    }

    /// Forget everything: a retune, or a new beacon.
    pub fn reset(&mut self) {
        let offset = self.phase_offset_deg;
        let snr = self.ident_snr;
        *self = Self::new(self.fs as u32).expect("the rate was accepted once");
        self.phase_offset_deg = offset;
        self.ident_snr = snr;
    }

    /// Feed audio. Returns what became known during it; the newest reading and
    /// any newly confirmed ident.
    pub fn push(&mut self, audio: &[f32]) -> Update {
        let mut update = Update::default();
        for &x in audio {
            let x = x as f64;
            self.sample(x);
            if let Some(r) = self.advance_block() {
                update.reading = Some(r);
            }
            if let Some(id) = self.advance_ident(x) {
                update.new_ident = Some(id);
            }
            self.n += 1;
        }
        update
    }

    fn sample(&mut self, x: f64) {
        let t = self.n as f64;
        // Variable: x correlated with e^{-j 2π 30 t}.
        let (s, c) = (TAU * (VAR_HZ * t / self.fs).fract()).sin_cos();
        self.cur.var_re += x * c;
        self.cur.var_im -= x * s;
        self.cur.sum_x += x;
        self.cur.n_in += 1;

        // Reference: down to baseband, then into the filter's ring.
        let (ms, mc) = self.mixer_phase.sin_cos();
        self.mixer_phase += self.mixer_step;
        if self.mixer_phase >= TAU {
            self.mixer_phase -= TAU;
        }
        let at = self.ring_at;
        self.ring_re[at] = x * mc;
        self.ring_im[at] = -x * ms;
        self.ring_re[at + REF_TAPS] = x * mc;
        self.ring_im[at + REF_TAPS] = -x * ms;
        self.ring_at = (at + 1) % REF_TAPS;

        self.since_out += 1;
        if self.since_out == self.decim {
            self.since_out = 0;
            self.reference_output();
        }
    }

    /// One output of the subcarrier filter, then the discriminator step.
    fn reference_output(&mut self) {
        // The oldest sample of the window sits at `ring_at`; the taps are
        // symmetric, so which way round they run does not matter.
        let w = self.ring_at;
        let (mut re, mut im) = (0.0, 0.0);
        for (k, &tap) in self.taps.iter().enumerate() {
            re += tap * self.ring_re[w + k];
            im += tap * self.ring_im[w + k];
        }
        if let Some((pr, pi)) = self.prev {
            // arg(conj(prev) * cur): the phase advance, radians per output.
            let phi = (pr * im - pi * re).atan2(pr * re + pi * im);
            // This step describes the instant halfway between two outputs, and
            // an output describes the input `REF_TAPS/2` samples before the
            // newest one it used. Timing it so is what keeps the two phases on
            // the same clock.
            let when = self.n as f64 - (REF_TAPS / 2) as f64 - self.decim as f64 / 2.0;
            let (s, c) = (TAU * (VAR_HZ * when / self.fs).rem_euclid(1.0)).sin_cos();
            self.cur.ref_re += phi * c;
            self.cur.ref_im -= phi * s;
            self.cur.n_ref += 1;
        }
        self.cur.sum_ref_mag += re.hypot(im);
        self.prev = Some((re, im));
    }

    fn advance_block(&mut self) -> Option<Reading> {
        self.in_block += 1;
        if self.in_block < self.block_len {
            return None;
        }
        self.in_block = 0;
        self.blocks[self.block_at] = std::mem::take(&mut self.cur);
        self.block_at = (self.block_at + 1) % WINDOW_BLOCKS;
        self.blocks_seen += 1;
        // Until the window is full the ring holds zeros and the filter is still
        // settling; say nothing.
        if self.blocks_seen < WINDOW_BLOCKS + 1 {
            return None;
        }
        let mut total = Block::default();
        for b in &self.blocks {
            total.var_re += b.var_re;
            total.var_im += b.var_im;
            total.ref_re += b.ref_re;
            total.ref_im += b.ref_im;
            total.sum_x += b.sum_x;
            total.sum_ref_mag += b.sum_ref_mag;
            total.n_in += b.n_in;
            total.n_ref += b.n_ref;
        }
        if total.n_in == 0 || total.n_ref == 0 {
            return None;
        }
        let mean = total.sum_x / total.n_in as f64;
        // x = A + a cos(ωt − φ) correlates to (a/2) e^{-jφ} per sample, so the
        // lag of the variable behind the reference is arg(ref · conj(var)).
        let lag = (total.ref_im * total.var_re - total.ref_re * total.var_im)
            .atan2(total.ref_re * total.var_re + total.ref_im * total.var_im);
        let radial = (lag.to_degrees() + self.phase_offset_deg).rem_euclid(360.0);
        let var_mag = (total.var_re.hypot(total.var_im)) * 2.0 / total.n_in as f64;
        let depth = if mean.abs() > 1e-12 { var_mag / mean.abs() } else { 0.0 };
        let ref_mag = total.ref_re.hypot(total.ref_im) * 2.0 / total.n_ref as f64;
        let deviation = ref_mag * self.work_rate / TAU;
        // The subcarrier's own level: half its depth lands in the baseband.
        let sub_depth = if mean.abs() > 1e-12 {
            2.0 * total.sum_ref_mag / total.n_ref as f64 / mean.abs()
        } else {
            0.0
        };
        let locked = depth >= 0.08
            && sub_depth >= 0.10
            && (0.35 * NOMINAL_DEVIATION_HZ..=1.5 * NOMINAL_DEVIATION_HZ).contains(&deviation);
        let r = Reading {
            radial_deg: radial as f32,
            var_depth: depth as f32,
            ref_deviation_hz: deviation as f32,
            ref_depth: sub_depth as f32,
            locked,
        };
        self.latest = Some(r);
        Some(r)
    }

    fn advance_ident(&mut self, x: f64) -> Option<String> {
        // The carrier is a million times the ident and leaks through any
        // window; take it out first. One-pole DC blocker, corner near 80 Hz.
        let y = x - self.dc_x1 + 0.99 * self.dc_y1;
        self.dc_x1 = x;
        self.dc_y1 = y;
        self.carrier += (x.abs() - self.carrier) * 1e-3;
        let w = y * self.hann[self.ident_at];
        for i in 0..IDENT_BINS {
            let s0 = w + self.bins.coeff[i] * self.bins.s1[i] - self.bins.s2[i];
            self.bins.s2[i] = self.bins.s1[i];
            self.bins.s1[i] = s0;
        }
        self.ident_at += 1;
        if self.ident_at < self.ident_len {
            return None;
        }
        self.ident_at = 0;
        let mut power = [0.0; IDENT_BINS];
        for (i, p) in power.iter_mut().enumerate() {
            *p = self.bins.s1[i].powi(2) + self.bins.s2[i].powi(2)
                - self.bins.coeff[i] * self.bins.s1[i] * self.bins.s2[i];
            self.bins.s1[i] = 0.0;
            self.bins.s2[i] = 0.0;
        }
        let noise = (power.iter().sum::<f64>() - power[2]) / (IDENT_BINS - 1) as f64;
        // Below a tone a thousandth of the carrier there is nothing to key.
        let floor = (1e-3 * self.carrier * self.ident_len as f64 / 4.0).powi(2);
        let ratio = power[2] / (noise + floor + 1e-30);
        // Hysteresis: a mark needs the full ratio to start and half of it to go
        // on, so a tone riding the threshold does not chatter into dots.
        let on = if self.key_on { ratio > self.ident_snr * 0.5 } else { ratio > self.ident_snr };
        self.keyed(on)
    }

    fn keyed(&mut self, on: bool) -> Option<String> {
        if on == self.key_on {
            self.run_len = self.run_len.saturating_add(1);
        } else if self.run_len < IDENT_MIN_RUN && !self.runs.is_empty() {
            // A blip: fold it, and the run it interrupted, back into one.
            let before = self.runs.pop().expect("checked non-empty");
            self.key_on = on;
            self.run_len = before.len + self.run_len + 1;
        } else {
            if self.run_len > 0 && !(self.runs.is_empty() && !self.key_on) {
                self.runs.push(Run { on: self.key_on, len: self.run_len });
            }
            self.key_on = on;
            self.run_len = 1;
        }
        if (self.key_on && self.run_len > IDENT_MAX_ON_BLOCKS) || self.runs.len() > IDENT_MAX_RUNS {
            self.runs.clear();
            return None;
        }
        if !self.key_on && self.run_len == IDENT_END_BLOCKS {
            return self.finish_ident();
        }
        None
    }

    fn finish_ident(&mut self) -> Option<String> {
        let runs = std::mem::take(&mut self.runs);
        let seconds_per_block = 1.0 / IDENT_BLOCKS_PER_S as f64;
        let text = morse::decode_runs(&runs, seconds_per_block)?;
        // Heard once is a candidate, heard twice running is an ident: a beacon
        // repeats it, noise does not.
        let agreed = self.candidate.as_deref() == Some(text.as_str());
        self.candidate = Some(text.clone());
        if agreed && self.confirmed.as_deref() != Some(text.as_str()) {
            self.confirmed = Some(text.clone());
            return Some(text);
        }
        None
    }
}

/// Windowed-sinc low-pass, Blackman window, unity gain at DC.
fn lowpass_taps(n: usize, cutoff_hz: f64, fs: f64) -> Vec<f64> {
    let mid = (n - 1) as f64 / 2.0;
    let fc = cutoff_hz / fs;
    let mut taps: Vec<f64> = (0..n)
        .map(|i| {
            let k = i as f64 - mid;
            let sinc = if k == 0.0 { 2.0 * fc } else { (TAU * fc * k).sin() / (std::f64::consts::PI * k) };
            let w = 0.42 - 0.5 * (TAU * i as f64 / (n - 1) as f64).cos()
                + 0.08 * (2.0 * TAU * i as f64 / (n - 1) as f64).cos();
            sinc * w
        })
        .collect();
    let sum: f64 = taps.iter().sum();
    taps.iter_mut().for_each(|t| *t /= sum);
    taps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn refuses_a_rate_that_misses_the_subcarrier() {
        assert!(matches!(VorDecoder::new(16_000), Err(VorError::SampleRateTooLow { got: 16_000 })));
        assert!(VorDecoder::new(24_000).is_ok());
    }

    #[test]
    fn the_filter_has_unity_gain_at_dc_and_kills_the_voice_band() {
        let taps = lowpass_taps(REF_TAPS, REF_CUTOFF_HZ, 48_000.0);
        let gain = |hz: f64| {
            let (mut re, mut im) = (0.0, 0.0);
            for (k, t) in taps.iter().enumerate() {
                let a = TAU * hz * k as f64 / 48_000.0;
                re += t * a.cos();
                im += t * a.sin();
            }
            re.hypot(im)
        };
        assert!((gain(0.0) - 1.0).abs() < 1e-9);
        assert!(gain(480.0) > 0.99);
        assert!(gain(6960.0) < 1e-3);
        assert!(gain(9960.0) < 1e-3);
    }

    #[test]
    fn silence_gives_no_lock() {
        let mut d = VorDecoder::new(48_000).unwrap();
        let up = d.push(&vec![0.0; 48_000 * 3]);
        assert!(up.new_ident.is_none());
        assert!(d.reading().is_none_or(|r| !r.locked));
    }
}
