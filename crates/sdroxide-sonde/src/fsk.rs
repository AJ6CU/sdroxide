//! The front end every FSK radiosonde model shares: mix the channel to
//! baseband, FM discriminator, slicer, and a bit clock that integrates over
//! each symbol and is nudged by the zero crossings.
//!
//! Models differ in baud rate, line coding and frame; they all start from the
//! raw symbol stream this produces.

use sdroxide_dsp::Complex32;

/// How the slicing threshold follows the signal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Threshold {
    /// Slow mean of the symbols: right for a DC-balanced code (Manchester).
    Mean,
    /// Midway between the recent highest and lowest level: right when the
    /// stream is not balanced, or sits at one level between bursts.
    Midrange,
}

pub struct FskSlicer {
    spb: f64,
    osc: f64,
    osc_step: f64,
    prev: Complex32,
    y: f32,
    mode: Threshold,
    dc: f32,
    hi: f32,
    lo: f32,
    phase: f64,
    acc: f32,
    cnt: u32,
    last_above: bool,
    level: f32,
}

impl FskSlicer {
    /// `baud` raw symbols per second; the channel centre is `freq_offset_hz`
    /// from the middle of the input.
    pub fn new(input_rate: f64, freq_offset_hz: f64, baud: f64, mode: Threshold) -> Result<Self, String> {
        if input_rate < 4.0 * baud {
            return Err(format!("input rate {input_rate} too low for {baud} baud"));
        }
        Ok(Self {
            spb: input_rate / baud,
            osc: 0.0,
            osc_step: -2.0 * std::f64::consts::PI * freq_offset_hz / input_rate,
            prev: Complex32::new(1.0, 0.0),
            y: 0.0,
            mode,
            dc: 0.0,
            hi: 0.0,
            lo: 0.0,
            phase: 0.0,
            acc: 0.0,
            cnt: 0,
            last_above: false,
            level: -120.0,
        })
    }

    /// Feed I/Q; `bit` is called with every raw symbol (0 or 1).
    pub fn process(&mut self, input: &[Complex32], mut bit: impl FnMut(u8)) {
        for &x in input {
            let (s, c) = self.osc.sin_cos();
            self.osc += self.osc_step;
            if self.osc > std::f64::consts::TAU {
                self.osc -= std::f64::consts::TAU;
            } else if self.osc < -std::f64::consts::TAU {
                self.osc += std::f64::consts::TAU;
            }
            let z = x * Complex32::new(c as f32, s as f32);
            self.level += 0.0005 * (10.0 * (z.norm_sqr() + 1e-12).log10() - self.level);
            let d = (z * self.prev.conj()).arg();
            self.prev = z;
            self.y += 0.5 * (d - self.y);

            if self.mode == Threshold::Midrange {
                if self.y > self.hi {
                    self.hi = self.y;
                } else {
                    self.hi += 0.001 * (self.y - self.hi);
                }
                if self.y < self.lo {
                    self.lo = self.y;
                } else {
                    self.lo += 0.001 * (self.y - self.lo);
                }
            }
            let thr = self.threshold();

            self.acc += self.y;
            self.cnt += 1;
            self.phase += 1.0;
            let above = self.y > thr;
            if above != self.last_above {
                let err = if self.phase < self.spb / 2.0 { self.phase } else { self.phase - self.spb };
                self.phase -= 0.08 * err;
            }
            self.last_above = above;
            if self.phase >= self.spb {
                self.phase -= self.spb;
                let mean = self.acc / self.cnt.max(1) as f32;
                self.acc = 0.0;
                self.cnt = 0;
                let b = u8::from(mean > thr);
                if self.mode == Threshold::Mean {
                    self.dc += 0.02 * (mean - self.dc);
                }
                bit(b);
            }
        }
    }

    fn threshold(&self) -> f32 {
        match self.mode {
            Threshold::Mean => self.dc,
            Threshold::Midrange => 0.5 * (self.hi + self.lo),
        }
    }

    /// Smoothed channel power, dBFS.
    pub fn level_dbfs(&self) -> f32 {
        self.level
    }
}
