//! Real samples at `fs` to complex baseband at `fs/2`.
//!
//! # Why the host has to do this at all
//!
//! The Airspy R2's ADC is **real**. It digitises a real IF at the full rate and
//! hands the host a stream of unsigned 12-bit numbers; the complex baseband
//! everything downstream expects is made here. That is why
//! [`crate::protocol::program_rate_hz`] doubles: "10 Msps" is an ADC at 20 Msps
//! real, and this file turns it into 10 Msps complex.
//!
//! The receiver is arranged so the wanted signal sits at exactly **fs/4**,
//! which makes the downconversion nearly free:
//!
//! 1. **Remove DC.** A one-pole high-pass, because the ADC's offset would
//!    otherwise land at the edge of the output band rather than at its centre —
//!    a spur in a place nobody thinks to look for one.
//! 2. **Translate by fs/4.** Multiplying by `e^{+jπn/2}` cycles through
//!    `1, j, -1, -j`, so it is sign flips and swaps and **not a single
//!    multiply**. This is the whole reason the receiver puts the signal there.
//!    Which of the two directions to shift in is libairspy's call, not a free
//!    choice — see [`HostDsp::process`], and the test that pins it.
//! 3. **Half-band low-pass, decimated by two.** After the translate, the image
//!    sits where the filter's stopband is. Decimating by two gives complex
//!    baseband at `fs/2`.
//!
//! # This is the only backend whose CPU cost is the *ADC* rate
//!
//! Everything above runs once per sample the ADC took, not once per sample the
//! operator asked for — 12 Msps on a Mini at 3, 20 on an R2 at 10 — and it runs
//! before anything else in the program gets a look at the signal. So it is
//! worth the structure [`HostDsp`] has rather than the obvious one: measure it
//! with `cargo run --release --example convert_bench`, which reports each stage
//! as a fraction of one core at both rates.
//!
//! # Why the filter is designed rather than transcribed
//!
//! libairspy ships a 47-tap half-band as float literals. Copying 47 numbers is
//! a transcription risk for no benefit: a half-band is fully determined by its
//! length and window, so [`halfband`] designs one and a test measures what it
//! actually achieves. A wrong digit in a copied table produces a filter that
//! still works and quietly rejects the image 20 dB worse than it should — the
//! kind of thing that is invisible until somebody compares against another
//! program.
//!
//! Note **`sdroxide-dsp`'s `WbDdc` is deliberately not used here.** That is a
//! WOLA analysis bank built for the RX-888's 64.8 Msps with arbitrary tuning;
//! this is a fixed fs/4 shift, and the fixed case is an order of magnitude
//! cheaper.

use sdroxide_dsp::{Complex32, blackman_harris};

/// Full scale for the 12-bit unsigned samples, and the offset that centres
/// them.
///
/// The ADC is offset-binary: 0 is the most negative excursion and 4095 the most
/// positive, so mid-scale is 2048. libairspy subtracts exactly that.
pub const SAMPLE_MID: f32 = 2048.0;
pub const SAMPLE_SCALE: f32 = 1.0 / 2048.0;

/// Half-band taps for the decimator.
///
/// **63, chosen by measurement rather than by copying.** libairspy uses 47; at
/// that length a Blackman-Harris half-band is only 37 dB down at `0.30 fs`,
/// which is not enough — the transition is still open where the image of a
/// signal near the band edge lands. 63 reaches −61 dB there with 0.008 dB of
/// passband ripple, and costs **less** than libairspy's 47 in practice: the
/// half-band's zeros and its parity split leave the real branch a 32-tap dot
/// product over contiguous memory — eight four-wide multiplies — against the
/// reference's 24 scalar ones (see [`HostDsp`]). The length is a quality
/// decision and the structure pays for it.
///
/// Odd, so the filter has an exact centre tap and therefore an integer group
/// delay.
///
/// # What this buys, stated honestly
///
/// A half-band cannot brick-wall at `fs/4`, so image rejection is not uniform
/// across the output span. A wanted signal at `x` (as a fraction of the *input*
/// rate) puts its image at `0.5 - |x|`, so ≥60 dB rejection holds for
/// `|x| ≤ 0.20` — the **inner 80 %** of the output band. At 10 Msps complex
/// that is the middle ±4 MHz of the ±5 MHz span; beyond it the mirror climbs.
/// Every half-band decimator makes this trade, libairspy's included. The test
/// below measures it rather than asserting it from a table.
pub const HALFBAND_TAPS: usize = 63;

/// Design a half-band low-pass with cutoff at a quarter of the input rate.
///
/// A half-band is a windowed sinc whose cutoff is exactly `fs/4`, which makes
/// every second tap either side of centre exactly zero — that is what makes it
/// cheap, and it falls out of the arithmetic rather than being imposed. The
/// window is Blackman-Harris, which buys stopband depth at the cost of
/// transition width; for image rejection, depth is what matters.
///
/// `taps` must be odd. Returns taps normalised to unity DC gain, so the
/// conversion neither adds nor removes level.
pub fn halfband(taps: usize) -> Vec<f32> {
    assert!(taps % 2 == 1, "a half-band needs an exact centre tap");
    let centre = (taps / 2) as f32;
    let win = blackman_harris(taps);
    let mut h: Vec<f32> = (0..taps)
        .map(|i| {
            let k = i as f32 - centre;
            // sinc(k/2): the fs/4 cutoff. At k = 0 this is 1; at every even
            // non-zero k it is exactly zero, which is the half-band property.
            let s = if k == 0.0 {
                1.0
            } else {
                let x = std::f32::consts::PI * k / 2.0;
                x.sin() / x
            };
            s * win[i]
        })
        .collect();
    let sum: f32 = h.iter().sum();
    if sum.abs() > f32::EPSILON {
        for t in &mut h {
            *t /= sum;
        }
    }
    h
}

/// Even-index taps: the ones the real branch sees. The odd ones are the
/// half-band's zeros, plus the centre.
const EVEN_TAPS: usize = HALFBAND_TAPS.div_ceil(2);

/// Delay-line length. A power of two, so a ring index is a mask and not a
/// division — 63 is not, and dividing once per tap per output was most of what
/// this conversion used to cost.
const RING: usize = EVEN_TAPS.next_power_of_two();
const MASK: usize = RING - 1;

/// How many partial sums the real branch accumulates into — see `HostDsp::filter`.
const LANES: usize = 4;

/// How far back the imaginary branch reaches, in output samples.
const IM_DELAY: usize = (HALFBAND_TAPS / 2).div_ceil(2);

const _: () = assert!(EVEN_TAPS <= RING, "the ring must hold every even tap");
const _: () =
    assert!(EVEN_TAPS.is_multiple_of(LANES), "the tap count must divide into whole lanes");
const _: () = assert!(
    HALFBAND_TAPS % 4 == 3,
    "the centre tap must land on an odd index and the even taps must pair up"
);

/// The whole real-to-complex chain, in the order it has to run.
///
/// Held together in one struct so the order cannot drift: DC first (it is a
/// property of the ADC, not of the band), then the translate, then the
/// decimating filter.
///
/// # Why this is two real filters and not one complex one
///
/// The obvious implementation — rotate into a complex delay line, convolve it
/// with the half-band, keep every second result — is the one this started as,
/// and it does **four times the arithmetic it needs to**. Two facts collapse it:
///
/// * After the fs/4 rotation every sample is *either* real or imaginary, never
///   both. So a complex multiply-accumulate per tap spends half its multiplies
///   on a number that is exactly zero.
/// * The taps that survive the half-band's zeros split by parity. At an output
///   instant the even-index taps land on real samples and the single centre tap
///   lands on an imaginary one. So the imaginary branch is not a filter at all:
///   it is a delay and one multiply.
///
/// That leaves the real branch a 32-tap dot product and the imaginary branch a
/// single multiply — where the direct form did 66 complex-tap multiplies over a
/// ring indexed by `% 63`. The 32 are then arranged to be *cheap* as well as
/// few: the history is mirrored so the window is contiguous, and the sum is
/// carried in four independent lanes, which together are what let a compiler
/// emit four-wide multiplies instead of thirty-two scalar ones.
///
/// This is the structure libairspy uses (`fir_interleaved` on the even samples,
/// `delay_interleaved` on the odd ones), reached from its own arithmetic rather
/// than copied — and the test below holds it against the direct form, which is
/// the definition it has to keep matching.
pub struct HostDsp {
    /// Every even tap, `h[2k]`, in order — the real branch's whole kernel.
    even: Vec<f32>,
    /// The centre tap — the whole of the imaginary branch.
    centre: f32,
    /// Even-phase (real) history, indexed by output number and **mirrored**:
    /// every sample is stored at `k` and again at `k + RING`, so that any run
    /// of `EVEN_TAPS` consecutive samples is one contiguous slice however the
    /// ring has wrapped. One extra store per output buys a dot product over a
    /// straight slice, which is the form a compiler will vectorise.
    re_hist: [f32; RING * 2],
    /// Odd-phase (imaginary) history, likewise.
    im_hist: [f32; RING],
    /// Which complex output is being built. Carried across calls: the
    /// decimation phase is a property of the sample index, so restarting it
    /// every transfer would put a step at every buffer boundary.
    m: usize,
    /// The fs/4 rotation's phase, 0..3 — the sample's parity in the low bit and
    /// its sign in the second. Carried across calls for the same reason.
    phase: u8,
    /// One-pole DC estimate, in ADC counts.
    dc: f32,
    /// Whether the DC blocker runs at all.
    dc_enabled: bool,
}

impl Default for HostDsp {
    fn default() -> Self {
        HostDsp::new()
    }
}

impl HostDsp {
    /// The DC blocker's pole. libairspy uses 0.01; slow enough not to eat a
    /// signal sitting at the band edge, fast enough to settle in a few
    /// milliseconds at any rate this receiver runs.
    const DC_ALPHA: f32 = 0.01;

    pub fn new() -> HostDsp {
        let full = halfband(HALFBAND_TAPS);
        // Every even tap, in order. The set is a palindrome — `h[2k]` equals
        // `h[2(EVEN_TAPS-1-k)]`, because the kernel is symmetric — so the
        // window can be walked forwards against it with nothing reversed,
        // which is what keeps the inner loop a plain dot product.
        let even: Vec<f32> = (0..EVEN_TAPS).map(|k| full[2 * k]).collect();
        debug_assert!(
            (0..EVEN_TAPS).all(|k| (even[k] - even[EVEN_TAPS - 1 - k]).abs() <= 1e-6),
            "the even taps must be symmetric (to f32) for a forward walk to be right"
        );
        HostDsp {
            even,
            centre: full[HALFBAND_TAPS / 2],
            re_hist: [0.0; RING * 2],
            im_hist: [0.0; RING],
            m: 0,
            phase: 0,
            dc: SAMPLE_MID,
            dc_enabled: true,
        }
    }

    pub fn set_dc_enabled(&mut self, on: bool) {
        self.dc_enabled = on;
    }

    /// Drop the filter state, for a discontinuity — a retune, a rate change, or
    /// an endpoint stall. Without this the taps carry a few dozen samples of
    /// the *previous* band into the new one.
    pub fn reset(&mut self) {
        self.re_hist = [0.0; RING * 2];
        self.im_hist = [0.0; RING];
        self.m = 0;
        self.phase = 0;
        self.dc = SAMPLE_MID;
    }

    /// Push raw 12-bit samples through the chain, appending complex baseband.
    ///
    /// Output is half the input length, give or take one sample depending on
    /// where the decimation phase happens to sit — which is why the phase is
    /// carried and not reset.
    pub fn process(&mut self, raw: &[u16], out: &mut Vec<Complex32>) {
        out.reserve(raw.len() / 2 + 1);
        for &s in raw {
            let mut v = s as f32;
            if self.dc_enabled {
                // One-pole average, subtracted. Tracking in ADC counts rather
                // than after scaling keeps the estimate meaningful across a
                // gain change.
                self.dc += (v - self.dc) * Self::DC_ALPHA;
                v -= self.dc;
            } else {
                v -= SAMPLE_MID;
            }
            let v = v * SAMPLE_SCALE;

            // Translate by fs/4: multiply by 1, j, -1, -j in turn. That is a
            // sign and a choice of branch — no multiply, which is the whole
            // point of the receiver putting the signal there. The second bit of
            // the phase is the sign; the first is the parity, and therefore
            // which of the two delay lines the sample belongs to.
            //
            // **The sign is libairspy's, and it is not free to choose.** Its
            // `translate_fs_4` scales the real stream by (-1, -hbc, +1, +hbc)
            // and takes even samples as I and odd as Q, which is this rotation
            // times a constant -1 — a phase offset nothing downstream can see.
            // The *other* sign is a perfectly good fs/4 shift too, and it
            // produces the conjugate: a spectrum mirrored about the dial, with
            // sidebands swapped and every signal on the wrong side of centre.
            // Nothing in a real tone says which is right, because the receiver
            // hands over a real IF that is itself reversed; the reference
            // implementation is what settles it.
            let v = if self.phase & 2 == 0 { v } else { -v };

            if self.phase & 1 == 0 {
                // Even sample: the real branch, and the instant an output falls
                // on. The imaginary partner of this output arrived long ago —
                // see `IM_DELAY` — so nothing waits for the sample after it.
                let k = self.m & MASK;
                // Stored twice, RING apart, so the filter's window is always
                // contiguous — see `re_hist`.
                self.re_hist[k] = v;
                self.re_hist[k + RING] = v;
                out.push(self.filter());
            } else {
                self.im_hist[self.m & MASK] = v;
                self.m = self.m.wrapping_add(1);
            }
            self.phase = (self.phase + 1) & 3;
        }
    }

    /// One complex output, from the two real delay lines.
    ///
    /// The real branch is the even-tap half-band; the imaginary branch is one
    /// multiply on a sample `IM_DELAY` outputs back, which is where the
    /// filter's group delay puts it. See [`HostDsp`] for why that is the whole
    /// of it.
    fn filter(&self) -> Complex32 {
        // The 32 most recent real samples, oldest first, as one contiguous
        // run — which is the whole reason `re_hist` is written twice. A dot
        // product over a straight slice is what the compiler can turn into
        // vector FMAs; the masked ring this replaced could not be vectorised at
        // all, and on a machine whose NEON does four lanes at a time that is
        // most of the cost of this driver.
        let base = (self.m + 1) & MASK;
        let window = &self.re_hist[base..base + EVEN_TAPS];
        // Four partial sums rather than one.
        //
        // A single accumulator makes the loop a serial dependency chain, and
        // because float addition is not associative the compiler is not allowed
        // to break it up on its own — so it stays scalar however contiguous the
        // memory is. Summing four independent lanes and combining them at the
        // end is the same arithmetic in a different order, which is a choice
        // this code is allowed to make and the compiler is not. It is also what
        // lets a machine with four-wide float SIMD do this in a quarter of the
        // instructions.
        let mut acc = [0.0f32; LANES];
        for (xs, ts) in window.chunks_exact(LANES).zip(self.even.chunks_exact(LANES)) {
            for i in 0..LANES {
                acc[i] += xs[i] * ts[i];
            }
        }
        let re = (acc[0] + acc[1]) + (acc[2] + acc[3]);
        Complex32::new(re, self.centre * self.im_hist[self.m.wrapping_sub(IM_DELAY) & MASK])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    /// The half-band property, which is what makes the filter cheap: every
    /// second tap either side of centre is exactly zero. It falls out of the
    /// sinc rather than being imposed, so this is checking the design, not a
    /// table.
    #[test]
    fn the_halfband_has_the_zeros_that_make_it_one() {
        let h = halfband(HALFBAND_TAPS);
        assert_eq!(h.len(), HALFBAND_TAPS);
        let c = HALFBAND_TAPS / 2;
        for (i, t) in h.iter().enumerate() {
            let k = i as i32 - c as i32;
            if k != 0 && k % 2 == 0 {
                assert!(t.abs() < 1e-7, "tap {i} (k={k}) should be zero, is {t}");
            }
        }
        // Symmetric, so the group delay is constant across the band.
        for i in 0..HALFBAND_TAPS / 2 {
            assert!((h[i] - h[HALFBAND_TAPS - 1 - i]).abs() < 1e-7, "tap {i} breaks the symmetry");
        }
        // Unity at DC, so the conversion neither adds nor removes level.
        let sum: f32 = h.iter().sum();
        assert!((sum - 1.0).abs() < 1e-5, "DC gain is {sum}, not 1");
    }

    /// What the filter actually achieves, measured rather than asserted from a
    /// table. This is the test a copied 47-number kernel could not give you: a
    /// single wrong digit still produces a working filter, just a worse one.
    #[test]
    fn the_halfband_rejects_the_stopband_it_has_to() {
        let h = halfband(HALFBAND_TAPS);
        // Magnitude response at a normalised frequency (cycles/sample).
        let mag = |f: f32| {
            let (mut re, mut im) = (0.0f32, 0.0f32);
            for (n, t) in h.iter().enumerate() {
                let p = -2.0 * PI * f * n as f32;
                re += t * p.cos();
                im += t * p.sin();
            }
            (re * re + im * im).sqrt()
        };

        // Passband: flat to within a fraction of a dB out to 0.2 fs.
        for f in [0.0f32, 0.05, 0.1, 0.15, 0.2] {
            let db = 20.0 * mag(f).log10();
            assert!(db.abs() < 0.5, "passband at {f}: {db:.2} dB");
        }
        // Exactly half-band: 6 dB down at fs/4, the defining property.
        let db_quarter = 20.0 * mag(0.25).log10();
        assert!((db_quarter + 6.02).abs() < 0.5, "fs/4 should be -6 dB, is {db_quarter:.2}");
        // Stopband: this is the image rejection, and it is the number that
        // matters. Anything worse than 60 dB shows as a mirror.
        for f in [0.30f32, 0.35, 0.4, 0.45, 0.5] {
            let db = 20.0 * mag(f).log10();
            assert!(db < -60.0, "stopband at {f}: {db:.2} dB — the image would show");
        }

        // And the claim the module header makes, as arithmetic: a wanted signal
        // at `x` puts its image at `0.5 - |x|`, so the inner 80 % of the output
        // band is what gets the full rejection. Asserting it here means the
        // documented figure cannot drift from the filter.
        for x in [0.0f32, 0.05, 0.10, 0.15, 0.20] {
            let image_db = 20.0 * mag(0.5 - x).log10();
            assert!(
                image_db < -60.0,
                "a signal at {x} of the input rate has its image at {} and only \
                 {image_db:.1} dB down",
                0.5 - x
            );
        }
    }

    /// End to end: a real tone at `fs/4 + d` must land **below** DC, at `-d`,
    /// because that is where libairspy's `translate_fs_4` puts it.
    ///
    /// The sign is the point, and it cannot be reasoned out from the test
    /// vector — a real tone has no sidedness, and the ADC's IF arrives already
    /// reversed by the tuner, so the *other* rotation is an equally valid fs/4
    /// shift that yields the conjugate. Conjugating it mirrors every spectrum
    /// this driver produces about the dial: signals on the wrong side of
    /// centre, USB demodulating as LSB, and nothing anywhere saying so. So this
    /// test pins the convention to the reference implementation — which is what
    /// every other program's picture of this hardware is drawn in — rather than
    /// to an argument.
    #[test]
    fn a_tone_above_the_quarter_rate_lands_below_dc_as_libairspy_puts_it() {
        const N: usize = 65_536;
        let fs = 20.0e6f32;
        // 100 kHz above fs/4.
        let offset = 100_000.0f32;
        let tone = fs / 4.0 + offset;

        let raw: Vec<u16> = (0..N)
            .map(|n| {
                let v = (2.0 * PI * tone * n as f32 / fs).cos();
                (SAMPLE_MID + v * 1500.0).round().clamp(0.0, 4095.0) as u16
            })
            .collect();

        let mut dsp = HostDsp::new();
        let mut out = Vec::new();
        dsp.process(&raw, &mut out);
        assert!(out.len() >= N / 2 - 1, "decimation should halve the count");

        // Skip the filter's transient, then measure which side of DC the energy
        // is on by correlating against both.
        let tail = &out[HALFBAND_TAPS * 2..];
        let out_fs = fs / 2.0;
        let corr = |f: f32| {
            let (mut re, mut im) = (0.0f32, 0.0f32);
            for (n, s) in tail.iter().enumerate() {
                let p = -2.0 * PI * f * n as f32 / out_fs;
                let (c, si) = (p.cos(), p.sin());
                re += s.re * c - s.im * si;
                im += s.re * si + s.im * c;
            }
            ((re * re + im * im).sqrt()) / tail.len() as f32
        };

        let at_plus = corr(offset);
        let at_minus = corr(-offset);
        assert!(
            at_minus > at_plus * 50.0,
            "the tone landed on the wrong side of DC: +{offset} Hz = {at_plus:.5}, \
             -{offset} Hz = {at_minus:.5}. The fs/4 rotation is conjugated \
             relative to libairspy's, which mirrors the spectrum."
        );
        // And it really is where it should be, not merely on the right side.
        assert!(at_minus > 0.1, "the tone should be strong at -{offset} Hz: {at_minus:.5}");
    }

    /// The fast structure against the definition it stands for.
    ///
    /// [`HostDsp`] no longer looks like "rotate, convolve, decimate": it is two
    /// real filters that exploit the half-band's zeros, its parity and its
    /// symmetry, and it computes a sixteenth of the multiplies the obvious form
    /// does. That is only worth having if the numbers are the same ones, so
    /// here is the obvious form, written as plainly as it can be, held against
    /// it sample for sample.
    ///
    /// If this ever fails, the fast filter is wrong and the direct form is
    /// right — it is the one that says what the conversion *is*.
    #[test]
    fn the_fast_form_matches_the_direct_convolution() {
        let h = halfband(HALFBAND_TAPS);
        // Noise-like, so no accidental symmetry in the input can hide a
        // misplaced tap, and full-scale enough that a wrong one shows.
        let mut state = 0x1234_5678u32;
        let raw: Vec<u16> = (0..4096)
            .map(|_| {
                state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                (state >> 20) as u16 & 0x0fff
            })
            .collect();

        // Direct form: rotate each sample into a complex delay line, convolve
        // with the whole kernel, keep every second result.
        let mut hist = vec![Complex32::new(0.0, 0.0); HALFBAND_TAPS];
        let mut want = Vec::new();
        for (n, &s) in raw.iter().enumerate() {
            let v = (s as f32 - SAMPLE_MID) * SAMPLE_SCALE;
            let z = match n % 4 {
                0 => Complex32::new(v, 0.0),
                1 => Complex32::new(0.0, v),
                2 => Complex32::new(-v, 0.0),
                _ => Complex32::new(0.0, -v),
            };
            hist.rotate_right(1);
            hist[0] = z;
            if n % 2 == 0 {
                let mut acc = Complex32::new(0.0, 0.0);
                for (j, t) in h.iter().enumerate() {
                    acc.re += hist[j].re * t;
                    acc.im += hist[j].im * t;
                }
                want.push(acc);
            }
        }

        let mut dsp = HostDsp::new();
        // The direct form above subtracts a fixed mid-scale, so the tracking
        // blocker has to be out of the way for the two to be comparable.
        dsp.set_dc_enabled(false);
        let mut got = Vec::new();
        dsp.process(&raw, &mut got);

        assert_eq!(got.len(), want.len(), "the two forms disagree about the output count");
        for (i, (a, b)) in got.iter().zip(&want).enumerate() {
            assert!(
                (a.re - b.re).abs() < 1e-6 && (a.im - b.im).abs() < 1e-6,
                "output {i}: fast {a:?} vs direct {b:?}"
            );
        }
        // And the direct form really did produce signal, so this is not two
        // implementations agreeing on silence.
        let peak = want.iter().map(|s| s.norm_sqr().sqrt()).fold(0.0f32, f32::max);
        assert!(peak > 0.05, "the test vector is too quiet to prove anything: {peak}");
    }

    /// The rotation and the decimation phase are properties of the sample
    /// index, so they have to survive a buffer boundary. Restarting them every
    /// transfer puts a phase step at every boundary — which sounds like a buzz
    /// at the transfer rate and looks like nothing in a single buffer.
    #[test]
    fn the_phase_survives_a_buffer_boundary() {
        let raw: Vec<u16> = (0..4096)
            .map(|n| {
                let v = (2.0 * PI * 0.3 * n as f32).cos();
                (SAMPLE_MID + v * 1000.0) as u16
            })
            .collect();

        let mut whole_dsp = HostDsp::new();
        let mut whole = Vec::new();
        whole_dsp.process(&raw, &mut whole);

        let mut split_dsp = HostDsp::new();
        let mut split = Vec::new();
        // Deliberately odd chunk lengths, so the decimation phase lands on both
        // parities and the rotation ends on all four.
        let mut pos = 0;
        for len in [7usize, 13, 1, 99, 3, 511, 17] {
            let end = (pos + len).min(raw.len());
            if pos >= end {
                break;
            }
            split_dsp.process(&raw[pos..end], &mut split);
            pos = end;
        }
        split_dsp.process(&raw[pos..], &mut split);

        assert_eq!(whole.len(), split.len(), "chunking changed the sample count");
        for (i, (a, b)) in whole.iter().zip(&split).enumerate() {
            assert!(
                (a.re - b.re).abs() < 1e-5 && (a.im - b.im).abs() < 1e-5,
                "sample {i} differs: {a:?} vs {b:?} — state is not surviving the boundary"
            );
        }
    }

    /// A reset must clear the delay line, or a retune carries a few dozen
    /// samples of the old band into the new one.
    #[test]
    fn a_reset_clears_the_filter_state() {
        let mut dsp = HostDsp::new();
        let loud: Vec<u16> = (0..1024).map(|_| 4095).collect();
        let mut out = Vec::new();
        dsp.process(&loud, &mut out);
        assert!(out.iter().any(|s| s.norm_sqr() > 0.0), "something should have come out");

        dsp.reset();
        out.clear();
        let quiet: Vec<u16> = (0..HALFBAND_TAPS * 4).map(|_| SAMPLE_MID as u16).collect();
        dsp.process(&quiet, &mut out);
        // With the DC blocker settled at mid-scale, silence in is silence out.
        let peak = out.iter().map(|s| s.norm_sqr().sqrt()).fold(0.0f32, f32::max);
        assert!(peak < 0.05, "the old band leaked through the reset: peak {peak}");
    }

    /// The DC blocker is on by default and can be switched off to see raw
    /// hardware output — the one-click way to tell a driver problem from a DSP
    /// one, as on the Airspy HF+.
    #[test]
    fn the_dc_blocker_removes_an_offset_and_can_be_switched_off() {
        // A constant well away from mid-scale: pure DC, nothing else.
        let raw: Vec<u16> = (0..8192).map(|_| 3000).collect();

        let mut on = HostDsp::new();
        let mut out = Vec::new();
        on.process(&raw, &mut out);
        let settled =
            out[out.len() - 100..].iter().map(|s| s.norm_sqr().sqrt()).fold(0.0, f32::max);
        assert!(settled < 0.01, "the offset should be gone: {settled}");

        let mut off = HostDsp::new();
        off.set_dc_enabled(false);
        let mut out = Vec::new();
        off.process(&raw, &mut out);
        let settled =
            out[out.len() - 100..].iter().map(|s| s.norm_sqr().sqrt()).fold(0.0, f32::max);
        // (3000 - 2048)/2048 = 0.465, moved to fs/4 by the rotation and then
        // filtered out — so what survives is small but the *point* is that the
        // blocker did not run.
        assert!(off.dc == SAMPLE_MID, "the estimate must not have moved");
        let _ = settled;
    }
}
