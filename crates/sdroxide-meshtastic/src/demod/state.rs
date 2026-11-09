// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (c) 2026 CEMAXECUTER LLC
// Rust port Copyright (c) 2026 F4NRM
//
// The symbol-level state machine of the demodulator: dechirp + FFT, preamble
// lock, carrier and timing recovery, header and payload assembly.

use super::*;

/// Circular distance between two FFT bins.
fn bin_distance(a: usize, b: usize, n: usize) -> usize {
    let d = a.abs_diff(b);
    if d > n / 2 { n - d } else { d }
}

impl LoraDecoder {
    /// Pick `N` samples out of the oversampled symbol window into
    /// `scratch_sym`, with the sub-sample timing corrections applied: the
    /// integer phase found while hunting, the fractional estimate made at
    /// preamble lock, and the drift accumulated since the down-chirps.
    fn downsample_symbol(&mut self, phase: usize) {
        let os = self.cfg.os_factor;
        if os == 1 {
            self.scratch_sym.copy_from_slice(&self.symbuf[..self.n]);
            return;
        }
        let half = (os / 2) as i64;
        let phase = phase.min(os - 1) as i64;
        let fine = ((self.sto_frac as f64 + self.sfo_cum) * os as f64).round_ties_even() as i64;
        let last = (self.spsym - 1) as i64;
        for k in 0..self.n {
            let idx = (half + (os * k) as i64 - phase - fine).clamp(0, last);
            self.scratch_sym[k] = self.symbuf[idx as usize];
        }
    }

    /// Dechirp and FFT the symbol in `scratch_sym`. Returns
    /// `(bin, peak, noise, value at capture_bin)`. The dechirped samples stay
    /// in `self.dechirped` for the fractional timing estimate.
    fn demod_one_symbol(&mut self, capture_bin: Option<usize>) -> (usize, f32, f32, Complex32) {
        let n = self.n;
        let cfo_bins = self.cfo_int as f64 + self.cfo_frac as f64;
        if cfo_bins != 0.0 {
            // Rotate the input by exp(-j 2 pi cfo n / N), then dechirp with the
            // canonical reference.
            let step = -2.0 * PI * cfo_bins / n as f64;
            let (ss, sc) = step.sin_cos();
            let rot_step = Complex64::new(sc, ss);
            let mut rot = Complex64::new(1.0, 0.0);
            for k in 0..n {
                let r = Complex32::new(rot.re as f32, rot.im as f32);
                self.dechirped[k] = self.scratch_sym[k] * r * self.downchirp[k];
                rot *= rot_step;
            }
        } else {
            for k in 0..n {
                self.dechirped[k] = self.scratch_sym[k] * self.downchirp[k];
            }
        }
        self.spectrum.copy_from_slice(&self.dechirped);
        self.fft.process_with_scratch(&mut self.spectrum, &mut self.fft_scratch);

        let mut best = 0.0f32;
        let mut best_bin = 0usize;
        let mut sum = 0.0f64;
        for (k, v) in self.spectrum.iter().enumerate() {
            let p = v.re * v.re + v.im * v.im;
            sum += p as f64;
            if p > best {
                best = p;
                best_bin = k;
            }
        }
        let avg = (sum - best as f64) / (n - 1) as f64;
        let noise = avg.max(0.0).sqrt() as f32;
        let captured = capture_bin.filter(|&b| b < n).map(|b| self.spectrum[b]).unwrap_or_default();
        (best_bin, best.sqrt(), noise, captured)
    }

    /// Dechirp `scratch_sym` with the up-chirp (a down-chirp symbol) and
    /// return the argmax bin, after removing the fractional carrier offset
    /// already estimated from the preamble.
    fn demod_downchirp_argmax(&mut self) -> usize {
        let n = self.n;
        let inv_n = 1.0 / n as f64;
        for k in 0..n {
            let ph = -2.0 * PI * self.cfo_frac as f64 * k as f64 * inv_n;
            let (s, c) = ph.sin_cos();
            let tw = Complex32::new(c as f32, s as f32);
            self.spectrum[k] = self.scratch_sym[k] * tw * self.upchirp[k];
        }
        self.fft.process_with_scratch(&mut self.spectrum, &mut self.fft_scratch);
        let mut best = 0.0f32;
        let mut best_bin = 0usize;
        for (k, v) in self.spectrum.iter().enumerate() {
            let p = v.re * v.re + v.im * v.im;
            if p > best {
                best = p;
                best_bin = k;
            }
        }
        best_bin
    }

    /// Fractional symbol-timing estimate (rational combined three spectral
    /// lines): zero-pad the saved dechirped preamble symbols to 2N, sum their
    /// power spectra, and interpolate round the peak. Result in [-0.5, 0.5].
    fn compute_sto_frac(&mut self) {
        let n = self.n;
        let m2 = 2 * n;
        if self.preamble_dechirped_count == 0 {
            self.sto_frac = 0.0;
            return;
        }
        let mut mag_sq = vec![0.0f64; m2];
        let mut buf = vec![Complex32::new(0.0, 0.0); m2];
        for s in 0..self.preamble_dechirped_count {
            buf[..n].copy_from_slice(&self.preamble_dechirped[s]);
            buf[n..].iter_mut().for_each(|c| *c = Complex32::new(0.0, 0.0));
            self.fft2.process_with_scratch(&mut buf, &mut self.fft2_scratch);
            for (j, v) in buf.iter().enumerate() {
                mag_sq[j] += (v.re * v.re + v.im * v.im) as f64;
            }
        }
        let mut k0 = 0usize;
        let mut peak = mag_sq[0];
        for (j, &v) in mag_sq.iter().enumerate().skip(1) {
            if v > peak {
                peak = v;
                k0 = j;
            }
        }
        let y_1 = mag_sq[(k0 + m2 - 1) % m2];
        let y0 = mag_sq[k0];
        let y1 = mag_sq[(k0 + 1) % m2];
        let u = 64.0 * n as f64 / 406.5506497;
        let v = u * 2.4674;
        let denom = u * (y1 + y_1) + v * y0;
        let wa = if denom != 0.0 { (y1 - y_1) / denom } else { 0.0 };
        let ka = wa * n as f64 / PI;
        let mut k_res = ((k0 as f64 + ka) / 2.0) % 1.0;
        if k_res < 0.0 {
            k_res += 1.0;
        }
        self.sto_frac = (k_res - if k_res > 0.5 { 1.0 } else { 0.0 }) as f32;
    }

    /// Run the state machine for one accumulated symbol.
    pub(super) fn state_tick(&mut self) {
        let os = self.cfg.os_factor;
        // While hunting, scan every sub-sample phase and keep the strongest.
        if os > 1
            && (self.state == State::Idle
                || (self.state == State::PreambleOk && !self.preamble_locked_once))
        {
            let mut best_peak = -1.0f32;
            let mut best_phase = 0usize;
            for ph in 0..os {
                self.downsample_symbol(ph);
                let (_, pk, _, _) = self.demod_one_symbol(None);
                if pk > best_peak {
                    best_peak = pk;
                    best_phase = ph;
                }
            }
            self.sto_offset = best_phase;
        }
        self.downsample_symbol(self.sto_offset);
        let capture = (self.state == State::PreambleOk).then_some(self.preamble_bin);
        let (sym, peak, noise, bin_val) = self.demod_one_symbol(capture);

        if matches!(self.state, State::Header | State::Payload) && peak > 0.0 && noise > 0.0 {
            self.snr_db_sum += 20.0 * (peak as f64 / noise as f64).log10();
            self.snr_db_count += 1;
        }

        // Silence has every bin at zero and would fake a preamble: a real one
        // has a peak well above the noise.
        let above_floor = peak > 0.0 && peak > 2.0 * if noise > 0.0 { noise } else { 1.0 };
        self.run_state(sym, peak, above_floor, bin_val);

        // Per-symbol sample-clock drift, after the down-chirps.
        let apply_drift =
            (self.state == State::Header && self.header_idx >= 2) || self.state == State::Payload;
        if apply_drift && self.sfo_hat != 0.0 {
            self.sfo_cum += self.sfo_hat;
            // A threshold of one sample: lower fires the carry-back at drift
            // the decoder already tolerates, and the carry-back itself
            // contaminates the next FFT with a phase discontinuity.
            let adj = 1.0 / os as f64;
            if self.sfo_cum > 1.0 {
                self.sfo_next_sym_shift = 1;
                self.sfo_cum -= adj;
            } else if self.sfo_cum < -1.0 {
                self.sfo_next_sym_shift = -1;
                self.sfo_cum += adj;
            }
        }
    }

    fn run_state(&mut self, sym: usize, peak: f32, above_floor: bool, bin_val: Complex32) {
        let n = self.n;
        match self.state {
            State::Idle => {
                if !above_floor {
                    return;
                }
                self.preamble_bin = sym;
                self.preamble_count = 1;
                self.preamble_fft_hist.clear();
                self.preamble_bin_hist.clear();
                self.preamble_bin_hist.push(sym);
                self.preamble_peak_sum = peak as f64;
                self.preamble_peak_count = 1;
                self.cfo_int = 0;
                self.cfo_frac = 0.0;
                self.state = State::PreambleOk;
                self.preamble_locked_once = false;
                self.stats.preamble_candidates += 1;
            }
            State::PreambleOk => {
                if !above_floor {
                    self.reset_to_idle();
                    return;
                }
                let diff = bin_distance(sym, self.preamble_bin, n);
                // A window straddling the preamble/sync boundary can still peak
                // on the preamble bin but at about half the height; that tick
                // already is the sync word, and counting it as preamble would
                // start the header one symbol late.
                let mean = if self.preamble_peak_count > 0 {
                    (self.preamble_peak_sum / self.preamble_peak_count as f64) as f32
                } else {
                    0.0
                };
                let straddle =
                    self.preamble_count >= PREAMBLE_MIN && mean > 0.0 && peak < 0.7 * mean;
                // Plus or minus two bins: window leakage and a residual carrier
                // offset make the peak wobble between neighbours.
                if diff <= 2 && !straddle {
                    if self.preamble_count < PREAMBLE_MIN + 4 {
                        self.preamble_count += 1;
                    }
                    self.preamble_peak_sum += peak as f64;
                    self.preamble_peak_count += 1;
                    if self.preamble_count >= PREAMBLE_MIN && !self.preamble_locked_once {
                        self.preamble_locked_once = true;
                        self.stats.preamble_locks += 1;
                    }
                    if self.preamble_fft_hist.len() < 8 {
                        self.preamble_fft_hist.push(bin_val);
                    }
                    if self.preamble_bin_hist.len() < 16 {
                        self.preamble_bin_hist.push(sym);
                    }
                    let slot = self.preamble_dechirped_next;
                    self.preamble_dechirped[slot].copy_from_slice(&self.dechirped);
                    self.preamble_dechirped_next = (slot + 1) % PREAMBLE_DECHIRPED_K;
                    if self.preamble_dechirped_count < PREAMBLE_DECHIRPED_K {
                        self.preamble_dechirped_count += 1;
                    }
                } else if self.preamble_count >= PREAMBLE_MIN {
                    // Caught on the peak drop with the bin still on the
                    // preamble: the window began more than half a symbol
                    // before the preamble's end, a whole symbol earlier than
                    // when the bin itself moves.
                    self.lock_on_sync(diff <= 2);
                } else {
                    self.reset_to_idle();
                }
            }
            State::Header => self.header_step(sym),
            State::Payload => self.payload_step(sym),
        }
    }

    /// The first sync-word symbol was seen: derive the symbol-grid offset and
    /// the fractional carrier offset, then queue the skip to the header.
    fn lock_on_sync(&mut self, early_by_a_symbol: bool) {
        let n = self.n;
        let os = self.cfg.os_factor;
        self.header_idx = 0;
        self.state = State::Header;
        self.compute_sto_frac();

        // The preamble bin is the symbol-grid offset plus the integer carrier
        // offset; take the most frequent one seen.
        let mut k_hat = self.preamble_bin;
        if self.preamble_bin_hist.len() >= 2 {
            let mut best_bin = self.preamble_bin_hist[0];
            let mut best_count = 1;
            for &b in &self.preamble_bin_hist {
                let c = self.preamble_bin_hist.iter().filter(|&&x| x == b).count();
                if c > best_count {
                    best_count = c;
                    best_bin = b;
                }
            }
            k_hat = best_bin;
        }
        if k_hat >= n {
            k_hat = 0;
        }
        // Signed in [-N/2, N/2): a negative offset wraps to near N, and the
        // skip must not slip by a whole symbol.
        // Caught early, the window began between 0.3 and 0.5 of a symbol short
        // of the grid, so the offset is positive whatever a carrier offset
        // adds to it; this keeps the half-symbol case on one side.
        let k_signed =
            if early_by_a_symbol || k_hat < n / 2 { k_hat as i64 } else { k_hat as i64 - n as i64 };
        self.sto_skip_remaining = ((n as i64 - k_signed) * os as i64).max(0) as usize;
        if early_by_a_symbol {
            self.sto_skip_remaining += n * os;
        }

        // Fractional carrier offset from the phase step between successive
        // preamble FFTs at the locked bin.
        let mut four_cum = Complex64::new(0.0, 0.0);
        for w in self.preamble_fft_hist.windows(2) {
            let a = Complex64::new(w[0].re as f64, w[0].im as f64);
            let b = Complex64::new(w[1].re as f64, w[1].im as f64);
            four_cum += a * b.conj();
        }
        self.cfo_frac = if four_cum.norm() > 0.0 {
            (-four_cum.im.atan2(four_cum.re) / (2.0 * PI)) as f32
        } else {
            0.0
        };
        // After the skip the symbol grid restarts at bin 0.
        self.preamble_bin = 0;
        self.preamble_bin_hist.clear();
    }

    fn header_step(&mut self, sym: usize) {
        let n = self.n;
        let os = self.cfg.os_factor;
        if self.header_idx < 2 {
            // Tick 0 is the first down-chirp, tick 1 the second: the integer
            // carrier offset is read off the second.
            if self.header_idx == 1 {
                let down_val = self.demod_downchirp_argmax();
                let signed_down =
                    if down_val < n / 2 { down_val as i64 } else { down_val as i64 - n as i64 };
                // The peak sits at round(2 (cfo_int + cfo_frac)); the fractional
                // part picks the rounding so both half-bin readings agree.
                self.cfo_int =
                    (signed_down as f64 / 2.0 - self.cfo_frac as f64).round_ties_even() as i64;
                // Same crystal for carrier and sample clock: the carrier offset
                // in bins scales to the drift in samples per symbol.
                self.sfo_hat = if self.cfg.center_freq_hz > 0.0 {
                    (self.cfo_int as f64 + self.cfo_frac as f64) * self.cfg.bw_hz as f64
                        / self.cfg.center_freq_hz
                } else {
                    0.0
                };
                self.sfo_cum = 0.0;
            }
            self.header_idx += 1;
            if self.header_idx == 2 {
                // A quarter down-chirp is still to come, plus the time shift
                // the carrier offset caused.
                let trim_out = n as f64 / 4.0 + self.cfo_int as f64 + self.cfo_frac as f64;
                let trim_input = (trim_out * os as f64).round_ties_even().max(0.0) as usize;
                self.sto_skip_remaining += trim_input;
            }
            return;
        }
        let hi = self.header_idx - 2;
        if hi < 8 {
            // (bin - 1) undoes the +1 of the transmitter's Gray stage; the
            // header carries sf - 2 bits per symbol.
            let corr = (sym as i64 - self.preamble_bin as i64 - 1).rem_euclid(n as i64) as u16;
            let demod_out = corr / 4;
            self.header_syms[hi] = demod_out ^ (demod_out >> 1);
            self.header_idx += 1;
        }
        if self.header_idx != 2 + 8 {
            return;
        }

        let sf = self.cfg.sf as usize;
        let sf_app = (sf - 2).max(5);
        let cw = deinterleave(&self.header_syms, sf_app, 8);
        let nib: Vec<u8> = cw.iter().map(|&c| hamming_decode(c, 8)).collect();

        self.payload_len = ((nib[0] as usize) << 4) | nib[1] as usize;
        self.payload_has_crc = nib[2] & 1 != 0;
        let hdr_cr_app = (nib[2] >> 1) & 7;
        self.payload_cr = if (1..=4).contains(&hdr_cr_app) { hdr_cr_app + 4 } else { 5 };
        let header_chk = ((nib[3] & 1) << 4) | (nib[4] & 0x0f);
        let computed = header_checksum(&[nib[0], nib[1], nib[2]]);
        if computed != header_chk || self.payload_len == 0 {
            self.stats.header_fail += 1;
            self.reset_to_idle();
            return;
        }
        self.stats.header_pass += 1;

        // The nibbles after the five header ones are the first payload ones.
        self.hdr_leftover = nib[5..sf_app].iter().map(|&x| x & 0x0f).collect();
        self.payload_ldro = lora::ldro(self.cfg.sf, self.cfg.bw_hz);
        let count = lora::payload_symbol_count(
            self.payload_len,
            self.cfg.sf,
            self.payload_cr,
            self.payload_has_crc,
            self.payload_ldro,
        );
        self.payload_sym_target = count.min(MAX_PAYLOAD_SYMBOLS);
        self.payload_syms.clear();
        self.state = State::Payload;
        if self.payload_sym_target == 0 {
            self.deliver(Vec::new(), !self.payload_has_crc);
        }
    }

    fn mean_snr(&self) -> f32 {
        if self.snr_db_count > 0 {
            (self.snr_db_sum / self.snr_db_count as f64) as f32
        } else {
            0.0
        }
    }

    /// Queue a finished frame and go back to hunting.
    fn deliver(&mut self, payload: Vec<u8>, crc_ok: bool) {
        self.out.push(LoraFrame {
            payload,
            sf: self.cfg.sf,
            cr: self.payload_cr,
            bw_hz: self.cfg.bw_hz,
            has_crc: self.payload_has_crc,
            crc_ok,
            snr_db: self.mean_snr(),
            cfo_bins: self.cfo_int as f32 + self.cfo_frac,
        });
        self.stats.frames += 1;
        self.reset_to_idle();
    }

    fn payload_step(&mut self, sym: usize) {
        let n = self.n as i64;
        let corr = (sym as i64 - self.preamble_bin as i64 - 1).rem_euclid(n) as u16;
        let demod_out = if self.payload_ldro { corr / 4 } else { corr };
        if self.payload_syms.len() < MAX_PAYLOAD_SYMBOLS {
            self.payload_syms.push(gray_decode(demod_out));
        }
        if self.payload_syms.len() < self.payload_sym_target {
            return;
        }

        let sf_p = if self.payload_ldro { self.cfg.sf as usize - 2 } else { self.cfg.sf as usize };
        let cr_p = self.payload_cr as usize;
        let mut bytes: Vec<u8> = Vec::with_capacity(MAX_PAYLOAD_BYTES);
        // Low nibble first; an odd count rolls into the next block.
        let mut pending: Option<u8> = None;
        let mut push = |nib: u8, bytes: &mut Vec<u8>| match pending.take() {
            None => pending = Some(nib),
            Some(lo) => bytes.push((nib << 4) | (lo & 0x0f)),
        };
        for &nib in &self.hdr_leftover {
            if bytes.len() >= MAX_PAYLOAD_BYTES {
                break;
            }
            push(nib, &mut bytes);
        }
        let mut blk = 0;
        while blk + cr_p <= self.payload_syms.len() {
            let cw = deinterleave(&self.payload_syms[blk..blk + cr_p], sf_p, cr_p);
            for &c in cw.iter().take(sf_p) {
                if bytes.len() >= MAX_PAYLOAD_BYTES {
                    break;
                }
                push(hamming_decode(c, self.payload_cr), &mut bytes);
            }
            blk += cr_p;
        }

        let total = self.payload_len + if self.payload_has_crc { 2 } else { 0 };
        bytes.truncate(total);
        let byte_count = bytes.len();
        // The CRC field is not whitened; only the payload is.
        let pay_only = self.payload_len.min(byte_count);
        whiten(&mut bytes[..pay_only]);

        let crc_ok = if self.payload_has_crc && byte_count >= 4 {
            let pay_len = byte_count - 2;
            let got = bytes[byte_count - 2] as u16 | ((bytes[byte_count - 1] as u16) << 8);
            let ok = payload_crc(&bytes[..pay_len]) == Some(got);
            if ok {
                self.stats.crc_pass += 1;
            } else {
                self.stats.crc_fail += 1;
            }
            ok
        } else {
            !self.payload_has_crc
        };

        bytes.truncate(pay_only);
        self.deliver(bytes, crc_ok);
    }
}
