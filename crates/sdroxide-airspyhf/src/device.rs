//! The receiver as a whole, and — more to the point — the order its parts have
//! to be touched in.
//!
//! Three orderings here are not obvious and are the reason this is one file
//! rather than a set of free functions:
//!
//! * A **rate change** re-tunes. The architecture (zero-IF or low-IF) is a
//!   property of the *rate*, and the tuning arithmetic depends on it, so
//!   changing rate without re-tuning leaves the synthesiser 5 kHz off and the
//!   host oscillator correcting for an offset that is no longer there.
//! * A **tune** reads the synthesiser's residual error back afterwards and
//!   folds it into the host's shift. The read is only valid for the LO just
//!   programmed.
//! * The **attenuator** is only obeyed with the AGC off, so both are applied
//!   together and in that order.
//!
//! All of this runs on the stream thread; see the invariant in [`crate::usb`].

use sdroxide_types::{AirspyHfConfig, AirspyHfModel};

use crate::error::{Error, Result};
use crate::protocol::{
    self, DEFAULT_IF_SHIFT_HZ, FLASH_CONFIG_BYTES, FlashConfig, MIN_LOW_IF_LO_KHZ,
    MIN_ZERO_IF_LO_KHZ, Request,
};
use crate::trace::Trace;
use crate::usb::UsbDev;

/// Where the synthesiser is put, and what the host has to do about it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Tuning {
    /// What goes out in `SET_FREQ`, in whole kHz.
    pub lo_khz: u32,
    /// How far the wanted signal sits **above** the LO, in Hz. The host
    /// oscillator runs at minus this.
    pub shift_hz: f64,
    /// The dial frequency after the calibration correction.
    pub adjusted_hz: f64,
}

/// Work out where to put the synthesiser and what the host must make up for.
///
/// Pure, and separated from the I/O precisely so it can be tested: six numbers
/// go in, two come out, and every tuning bug in this driver is a wrong value
/// among them.
///
/// * `cal_ppb` is parts per **billion** — the receiver's own unit.
/// * On a zero-IF rate with the host DSP running, the LO is deliberately parked
///   [`DEFAULT_IF_SHIFT_HZ`] above the signal so the synthesiser's own leakage
///   lands clear of it. With the DSP off there is nothing to shift the signal
///   back, so there is no offset either.
/// * Below the synthesiser's floor the LO clamps and `shift_hz` grows to cover
///   the difference. That is not a degraded mode — it is how this receiver
///   tunes VLF at all.
pub fn tuning(hz: f64, cal_ppb: i32, low_if: bool, dsp: bool, freq_delta_hz: f64) -> Tuning {
    let if_shift = if dsp && !low_if { DEFAULT_IF_SHIFT_HZ } else { 0.0 };
    let adjusted_hz = hz * (1.0e9 + cal_ppb as f64) * 1.0e-9;
    let floor_khz = if low_if { MIN_LOW_IF_LO_KHZ } else { MIN_ZERO_IF_LO_KHZ };
    let khz = ((adjusted_hz + if_shift) * 1e-3).round().clamp(0.0, u32::MAX as f64) as u32;
    let lo_khz = khz.max(floor_khz);
    Tuning { lo_khz, shift_hz: adjusted_hz - lo_khz as f64 * 1e3 + freq_delta_hz, adjusted_hz }
}

/// The index of the available rate nearest `want`.
///
/// Nearest rather than exact on purpose. Which rates exist depends on the model
/// *and* the firmware, so a `radio.json` written against a Discovery names rates
/// a Dual does not have — and a receiver that refuses to open because of a
/// number in a config file is the wrong failure. The caller says what it did.
pub fn snap_rate(want: f64, rates: &[f64]) -> Option<usize> {
    rates
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| (*a - want).abs().total_cmp(&(*b - want).abs()))
        .map(|(i, _)| i)
}

/// The attenuator step nearest to, and not less than, `att_db` of attenuation.
///
/// Matches upstream: the first step that reaches the requested attenuation, so
/// a request always lands on at least as much as was asked for. Returns the
/// index to send and the dB it actually corresponds to.
pub fn snap_attenuator(att_db: f64, steps_db: &[f64]) -> (u16, f64) {
    for (i, s) in steps_db.iter().enumerate() {
        if *s >= att_db {
            return (i as u16, *s);
        }
    }
    // Asked for more than the hardware has: the last step is the most it can do.
    match steps_db.len() {
        0 => (0, 0.0),
        n => ((n - 1) as u16, steps_db[n - 1]),
    }
}

/// An open Airspy HF+, configured and ready to stream.
pub struct Device {
    usb: UsbDev,

    pub model: AirspyHfModel,
    /// The 64-bit serial the board reports, which is the same number as the
    /// descriptor's hex digits.
    pub board_serial: u64,
    pub firmware: String,

    /// Rates the receiver offers, in the order it lists them.
    pub rates_hz: Vec<f64>,
    /// Which of those are low-IF, index-parallel to [`Self::rates_hz`].
    pub low_if: Vec<bool>,
    /// Attenuator steps the receiver offers, in dB of attenuation.
    pub att_steps_db: Vec<f64>,
    /// Index into [`Self::rates_hz`] currently in effect.
    pub rate_index: usize,
    /// Set when the configured rate was not on offer and had to be snapped.
    /// Surfaced to the operator rather than logged and forgotten.
    pub snapped_from: Option<f64>,
    /// Whether the receiver answered the bias-tee query at all. Older firmware
    /// does not have one.
    pub bias_tee_supported: bool,

    flash: FlashConfig,
    cal_ppb: i32,
    dsp_enabled: bool,

    freq_hz: f64,
    lo_khz: u32,
    freq_delta_hz: f64,
    shift_hz: f64,
    filter_gain: f32,
    streaming: bool,
}

impl Device {
    /// Open, identify and configure a receiver, leaving it ready but not
    /// streaming.
    pub fn open(cfg: &AirspyHfConfig, center_hz: f64, trace: &Trace) -> Result<Device> {
        let usb = UsbDev::open(&cfg.serial, trace)?;

        // Identity first, so that a failure anywhere after this point arrives
        // with the receiver already named in the report.
        let (part_id, board_serial) = usb
            .control_in_exact(Request::SerialBoardId, 0, 0, 20)
            .ok()
            .and_then(|b| protocol::parse_board_id(&b))
            .unwrap_or((0, 0));
        let model = AirspyHfModel::from_part_id(part_id);
        let firmware = usb
            .control_in(Request::GetVersionString, 0, 0, 63)
            .map(|b| protocol::parse_ascii(&b))
            .unwrap_or_default();

        let mut dev = Device {
            usb,
            model,
            board_serial,
            firmware,
            rates_hz: Vec::new(),
            low_if: Vec::new(),
            att_steps_db: Vec::new(),
            rate_index: 0,
            snapped_from: None,
            bias_tee_supported: false,
            flash: FlashConfig::default(),
            cal_ppb: 0,
            dsp_enabled: cfg.lib_dsp,
            freq_hz: center_hz,
            // Not a real LO value: it forces the first `set_center_hz` to
            // actually program the synthesiser rather than skipping it as
            // unchanged.
            lo_khz: u32::MAX,
            freq_delta_hz: 0.0,
            shift_hz: 0.0,
            filter_gain: 1.0,
            streaming: false,
        };

        dev.usb.trace().set_identity(format!(
            "{} (part id 0x{part_id:08x}), serial {:016X}, firmware {:?}, {}",
            dev.model.label(),
            dev.board_serial,
            dev.firmware,
            dev.usb.speed_name(),
        ));

        dev.read_rate_table();
        dev.read_att_steps();
        dev.read_flash(cfg);

        // The operator's override beats the receiver's flash, and `None` means
        // "use whatever the receiver says", which is the only value that cannot
        // be wrong on hardware nobody here has seen.
        if let Some(ppb) = cfg.calibration_ppb {
            dev.cal_ppb = ppb;
        }

        let want = cfg.sample_rate_hz;
        let idx = snap_rate(want, &dev.rates_hz).unwrap_or(0);
        if (dev.rates_hz[idx] - want).abs() > 1.0 {
            dev.snapped_from = Some(want);
        }
        dev.set_rate_index(idx)?;

        // Gains after the rate, because the rate change re-tunes and a re-tune
        // is the expensive part; doing it once at the end of the open costs one
        // round trip instead of several.
        dev.apply_gains(cfg)?;
        dev.probe_bias_tee();
        dev.set_bias_tee(cfg.bias_tee);

        Ok(dev)
    }

    // ---- identity and state ---------------------------------------------

    pub fn label(&self) -> &str {
        self.usb.label()
    }

    pub fn usb(&self) -> &UsbDev {
        &self.usb
    }

    pub fn trace(&self) -> &Trace {
        self.usb.trace()
    }

    pub fn sample_rate_hz(&self) -> f64 {
        self.rates_hz[self.rate_index]
    }

    pub fn is_low_if(&self) -> bool {
        self.low_if.get(self.rate_index).copied().unwrap_or(false)
    }

    /// How far the wanted signal sits above the LO. The host oscillator runs at
    /// minus this; see [`crate::convert::HostDsp::set_shift`].
    pub fn shift_hz(&self) -> f64 {
        self.shift_hz
    }

    pub fn filter_gain(&self) -> f32 {
        self.filter_gain
    }

    pub fn center_hz(&self) -> f64 {
        self.freq_hz
    }

    pub fn calibration_ppb(&self) -> i32 {
        self.cal_ppb
    }

    pub fn flash(&self) -> FlashConfig {
        self.flash
    }

    /// Which of the receiver's own attenuator steps are on offer, as *gains*
    /// (0 down to −max), for the settings panel.
    pub fn attenuator_range_db(&self) -> (f64, f64) {
        let max = self.att_steps_db.last().copied().unwrap_or(AirspyHfConfig::ATT_MAX_DB);
        let step = match self.att_steps_db.len() {
            0 | 1 => AirspyHfConfig::ATT_STEP_DB,
            n => (max / (n - 1) as f64).max(0.1),
        };
        (max, step)
    }

    // ---- open-time queries -----------------------------------------------

    /// Ask the receiver which rates it has and which of them are low-IF.
    ///
    /// Two round trips each: a count, then the list. A firmware that answers
    /// neither is old enough to predate the query, and every one of those runs
    /// 768 kSPS zero-IF, which is what upstream falls back to as well.
    fn read_rate_table(&mut self) {
        let rates = self
            .usb
            .control_in(Request::GetSampleRates, 0, 0, 4)
            .ok()
            .and_then(|b| protocol::parse_count(&b))
            .filter(|n| *n > 0 && *n <= 64)
            .and_then(|n| {
                self.usb.control_in(Request::GetSampleRates, 0, n as u16, (n * 4) as u16).ok()
            })
            .map(|b| protocol::parse_rates(&b))
            .filter(|r| !r.is_empty());

        self.rates_hz = match rates {
            Some(r) => r,
            None => {
                self.trace().note(
                    "the receiver did not report its rate table; assuming a single \
                     768 kSPS zero-IF rate, which is what every firmware old \
                     enough to lack the query actually runs",
                );
                vec![768_000.0]
            }
        };

        // The architecture reply is one byte per rate even though the request
        // asks for four, so a short answer here is normal, not a fault.
        let n = self.rates_hz.len();
        let arch =
            self.usb.optional_in(Request::GetSampleRateArchitectures, 0, n as u16, (n * 4) as u16);
        self.low_if = match &arch {
            Some(b) => protocol::parse_architectures(b, n),
            None => vec![false; n],
        };
        self.trace().note(format!(
            "rates: [{}]",
            self.rates_hz
                .iter()
                .zip(&self.low_if)
                .map(|(r, lo)| format!(
                    "{:.0}k {}",
                    r / 1000.0,
                    if *lo { "low-IF" } else { "zero-IF" }
                ))
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    fn read_att_steps(&mut self) {
        let steps = self
            .usb
            .optional_in(Request::GetAttSteps, 0, 0, 4)
            .and_then(|b| protocol::parse_count(&b))
            .filter(|n| *n > 0 && *n <= 64)
            .and_then(|n| self.usb.optional_in(Request::GetAttSteps, 0, n as u16, (n * 4) as u16))
            .map(|b| protocol::parse_att_steps(&b))
            .filter(|s| !s.is_empty());

        self.att_steps_db = match steps {
            Some(s) => s,
            None => {
                // The Discovery's table, which is what upstream assumes too.
                self.trace().note("no attenuator table; assuming nine 6 dB steps");
                (0..9).map(|i| i as f64 * AirspyHfConfig::ATT_STEP_DB).collect()
            }
        };
    }

    /// Read the receiver's stored calibration and push the parts of it that
    /// live in volatile registers back to the hardware.
    ///
    /// The VCTCXO trim and the front-end options are not persisted *in* the
    /// receiver's running state — the vendor tool writes them to flash and the
    /// host is expected to replay them at every open. Skipping that leaves a
    /// calibrated receiver running uncalibrated.
    ///
    /// **Nothing here ever writes flash.** The page also carries the VCTCXO
    /// trim, and a wrong write would cost the receiver its factory calibration
    /// with no way back short of the vendor tool.
    fn read_flash(&mut self, cfg: &AirspyHfConfig) {
        let page = match self.usb.control_in(Request::ConfigRead, 0, 0, FLASH_CONFIG_BYTES as u16) {
            Ok(p) => p,
            Err(e) => {
                self.trace().note(format!("could not read the calibration page ({e})"));
                return;
            }
        };
        self.trace().wire_head("flash config page", &page, 32);
        self.flash = FlashConfig::parse(&page);
        if !self.flash.valid {
            self.trace().note(
                "the calibration page has no magic — this receiver has never been calibrated",
            );
            return;
        }
        self.cal_ppb = self.flash.calibration_ppb;
        self.trace().note(format!(
            "stored calibration: {} ppb, vctcxo {}, frontend options 0x{:08x}{}",
            self.flash.calibration_ppb,
            self.flash.calibration_vctcxo,
            self.flash.frontend_options,
            match cfg.calibration_ppb {
                Some(p) => format!(" (overridden by the operator: {p} ppb)"),
                None => String::new(),
            }
        ));
        let vctcxo = self.flash.calibration_vctcxo as u16;
        let _ = self.usb.control_out(Request::SetVctcxoCalibration, vctcxo, 0, &[]);
        let flags = self.flash.frontend_options;
        let _ = self.usb.control_out(
            Request::SetFrontendOptions,
            (flags & 0xffff) as u16,
            (flags >> 16) as u16,
            &[],
        );
    }

    /// Whether this receiver has a bias tee at all. Older firmware stalls the
    /// query, in which case the switch is simply not offered.
    fn probe_bias_tee(&mut self) {
        let count = self
            .usb
            .optional_in(Request::GetBiasTeeCount, 0, 0, 4)
            .and_then(|b| protocol::parse_count(&b))
            .unwrap_or(0);
        self.bias_tee_supported = count > 0;
        if self.bias_tee_supported {
            let names: Vec<String> = (0..count.min(4))
                .filter_map(|i| self.usb.optional_in(Request::GetBiasTeeName, 0, i as u16, 63))
                .map(|b| protocol::parse_ascii(&b))
                .collect();
            self.trace().note(format!("bias tee outputs: {count} [{}]", names.join(", ")));
        }
    }

    // ---- control ---------------------------------------------------------

    /// Program the synthesiser and work out what the host has to make up for.
    ///
    /// The LO is only reprogrammed when the whole-kHz value actually changes —
    /// a dial dragged across a few hundred Hz costs nothing — but the host's
    /// shift is recomputed every time, because that is where the sub-kHz part
    /// of the dial lives.
    pub fn set_center_hz(&mut self, hz: f64) -> Result<()> {
        let t = tuning(hz, self.cal_ppb, self.is_low_if(), self.dsp_enabled, self.freq_delta_hz);
        if t.lo_khz != self.lo_khz {
            self.usb.control_out(Request::SetFreq, 0, 0, &protocol::encode_freq_khz(t.lo_khz))?;
            self.lo_khz = t.lo_khz;
            // The synthesiser cannot land exactly on every kHz; ask what it
            // actually did. Only meaningful for the LO just programmed, which
            // is why this read lives inside the branch.
            self.freq_delta_hz = self
                .usb
                .optional_in(Request::GetFreqDelta, 0, 0, 4)
                .and_then(|b| protocol::decode_freq_delta(&b))
                .unwrap_or(0.0);
        }
        self.freq_hz = hz;
        // Recomputed with the delta now known, which the first pass could not
        // have used.
        self.shift_hz =
            tuning(hz, self.cal_ppb, self.is_low_if(), self.dsp_enabled, self.freq_delta_hz)
                .shift_hz;
        tracing::debug!(
            "tune {hz:.0} Hz: LO {} kHz, delta {:+.1} Hz, host shift {:+.1} Hz",
            self.lo_khz,
            self.freq_delta_hz,
            self.shift_hz
        );
        Ok(())
    }

    /// Switch to one of the receiver's rates.
    ///
    /// Does *not* stop the stream — upstream does not either, and the endpoint
    /// is cleared instead. The zero-IF floor is raised first when needed,
    /// because a receiver sitting below 180 kHz cannot be handed a zero-IF rate
    /// while it is there.
    pub fn set_rate_index(&mut self, index: usize) -> Result<()> {
        let index = index.min(self.rates_hz.len() - 1);
        let low_if = self.low_if.get(index).copied().unwrap_or(false);

        if !low_if && self.lo_khz < MIN_ZERO_IF_LO_KHZ {
            self.usb.control_out(
                Request::SetFreq,
                0,
                0,
                &protocol::encode_freq_khz(MIN_ZERO_IF_LO_KHZ),
            )?;
            self.lo_khz = MIN_ZERO_IF_LO_KHZ;
        }

        self.usb.control_out(Request::SetSampleRate, 0, index as u16, &[])?;
        self.rate_index = index;

        // The firmware's own processing gain changes with the rate, and the
        // conversion undoes it. Reading it here rather than at open is what
        // upstream does, and it is the only place it can be right.
        self.filter_gain = self
            .usb
            .optional_in(Request::GetFilterGain, 0, 0, 1)
            .map(|b| protocol::filter_gain_from_db(&b))
            .unwrap_or(1.0);

        // The architecture just changed, so the tuning arithmetic did too.
        let hz = self.freq_hz;
        self.lo_khz = u32::MAX;
        self.set_center_hz(hz)?;

        tracing::info!(
            "sample rate {:.0} kSPS ({}), filter gain {:.4}",
            self.sample_rate_hz() / 1000.0,
            if self.is_low_if() { "low-IF" } else { "zero-IF" },
            self.filter_gain
        );
        Ok(())
    }

    /// Apply every gain-side setting in the order the hardware wants them.
    pub fn apply_gains(&mut self, cfg: &AirspyHfConfig) -> Result<()> {
        self.set_agc(cfg.agc)?;
        self.set_agc_threshold(cfg.agc_threshold_high)?;
        // Only obeyed with the AGC off, so it goes after it.
        self.set_attenuator_db(cfg.attenuator_db)?;
        self.set_lna(cfg.lna)
    }

    pub fn set_agc(&mut self, on: bool) -> Result<()> {
        self.usb.control_out(Request::SetAgc, u16::from(on), 0, &[])
    }

    pub fn set_agc_threshold(&mut self, high: bool) -> Result<()> {
        self.usb.control_out(Request::SetAgcThreshold, u16::from(high), 0, &[])
    }

    /// `db` is a *gain*: 0 is no attenuation, −48 is all of it.
    pub fn set_attenuator_db(&mut self, db: f64) -> Result<()> {
        let (index, actual) = snap_attenuator(-db, &self.att_steps_db);
        self.usb.control_out(Request::SetAtt, index, 0, &[])?;
        tracing::debug!("attenuator: asked {db:.1} dB, set step {index} = -{actual:.1} dB");
        Ok(())
    }

    pub fn set_lna(&mut self, on: bool) -> Result<()> {
        self.usb.control_out(Request::SetLna, u16::from(on), 0, &[])
    }

    /// Bias tee, when the receiver has one. Never fatal: a receiver without one
    /// is not broken, and neither is a firmware that predates the request.
    pub fn set_bias_tee(&mut self, on: bool) {
        if !self.bias_tee_supported && !on {
            return;
        }
        // Upstream sends an `int8_t` widened into `wValue`, so a negative value
        // arrives as 0xFF__. Only 0 and 1 are used here; the sign extension is
        // reproduced anyway so the encoding cannot drift if that changes.
        let value = protocol::bias_tee_wvalue(i8::from(on));
        if !self.usb.optional_out(Request::SetBiasTee, value, 0) && on {
            tracing::warn!("{}: the bias tee could not be switched on", self.label());
        }
    }

    /// Set the calibration and re-tune, because every frequency now means
    /// something slightly different.
    pub fn set_calibration_ppb(&mut self, ppb: i32) -> Result<()> {
        self.cal_ppb = ppb;
        let hz = self.freq_hz;
        self.set_center_hz(hz)
    }

    /// Turn the host DSP on or off. Changes the tuning arithmetic, because the
    /// 5 kHz zero-IF offset only exists when something is going to shift the
    /// signal back.
    pub fn set_lib_dsp(&mut self, on: bool) -> Result<()> {
        if self.dsp_enabled == on {
            return Ok(());
        }
        self.dsp_enabled = on;
        let hz = self.freq_hz;
        self.set_center_hz(hz)
    }

    pub fn lib_dsp(&self) -> bool {
        self.dsp_enabled
    }

    // ---- streaming -------------------------------------------------------

    /// Start the sample stream.
    ///
    /// The off-then-on is deliberate and comes from upstream: a receiver that
    /// was left running by a program that died still has data in its FIFO, and
    /// starting without clearing it delivers stale samples first.
    pub fn start(&mut self) -> Result<()> {
        self.usb.control_out(Request::ReceiverMode, 0, 0, &[])?;
        self.usb.control_out(Request::ReceiverMode, 1, 0, &[])?;
        self.streaming = true;
        Ok(())
    }

    pub fn stop(&mut self) {
        if self.streaming {
            let _ = self.usb.control_out(Request::ReceiverMode, 0, 0, &[]);
            self.streaming = false;
        }
    }

    /// Leave the receiver in a state the next program can open.
    ///
    /// The bias tee is switched off explicitly: it is a persisted setting, and
    /// leaving DC on somebody's feedline after the program has exited is not
    /// something to do by omission.
    pub fn shutdown(&mut self) {
        self.stop();
        if self.bias_tee_supported {
            let _ = self.usb.optional_out(Request::SetBiasTee, 0, 0);
        }
    }

    /// A one-line summary for logs and the UI.
    pub fn describe(&self) -> String {
        let mut s = format!("{} {:.0} kSPS", self.model.label(), self.sample_rate_hz() / 1000.0);
        if self.is_low_if() {
            s.push_str(" low-IF");
        }
        if !self.firmware.is_empty() {
            s.push_str(&format!(" ({})", self.firmware));
        }
        s
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        self.shutdown();
    }
}

impl Device {
    /// A receiver that reported no rates at all cannot be worked with. The
    /// fallback in [`Device::read_rate_table`] guarantees at least one, so this
    /// firing means that fallback was bypassed — but the stream indexes into
    /// the table, so it checks rather than trusting.
    pub(crate) fn assert_has_rates(&self) -> Result<()> {
        if self.rates_hz.is_empty() {
            return Err(Error::Unsupported("the receiver reported no sample rates".into()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The synthesiser is parked 5 kHz above the dial so its own leakage stays
    /// off the operator's signal, and the host brings the signal back.
    #[test]
    fn zero_if_parks_the_lo_five_khz_above_the_dial() {
        let t = tuning(14_074_000.0, 0, false, true, 0.0);
        assert_eq!(t.lo_khz, 14_079);
        assert!((t.shift_hz + 5_000.0).abs() < 1e-6, "shift was {}", t.shift_hz);
        assert_eq!(t.adjusted_hz, 14_074_000.0);
    }

    /// A low-IF rate has the firmware doing the offset, so the host must not
    /// add a second one — and the synthesiser reaches lower.
    #[test]
    fn low_if_uses_no_offset_and_the_lower_floor() {
        let t = tuning(14_074_000.0, 0, true, true, 0.0);
        assert_eq!(t.lo_khz, 14_074);
        assert!(t.shift_hz.abs() < 1e-6);
        // 100 kHz is under the zero-IF floor but over the low-IF one.
        assert_eq!(tuning(100_000.0, 0, true, true, 0.0).lo_khz, 100);
        assert_eq!(tuning(50_000.0, 0, true, true, 0.0).lo_khz, MIN_LOW_IF_LO_KHZ);
    }

    /// With no host oscillator there is nothing to undo the offset, so the
    /// offset must not be applied in the first place.
    #[test]
    fn turning_the_host_dsp_off_removes_the_zero_if_offset() {
        let t = tuning(14_074_000.0, 0, false, false, 0.0);
        assert_eq!(t.lo_khz, 14_074);
        assert!(t.shift_hz.abs() < 1e-6);
    }

    /// How this receiver tunes VLF at all. A "clamp to the floor" that forgot
    /// the oscillator would silently receive 180 kHz while the dial read 60.
    #[test]
    fn below_the_lo_floor_the_oscillator_does_all_the_work() {
        let t = tuning(60_000.0, 0, false, true, 0.0);
        assert_eq!(t.lo_khz, MIN_ZERO_IF_LO_KHZ);
        assert!((t.shift_hz + 120_000.0).abs() < 1e-6, "shift was {}", t.shift_hz);
        // And the shift stays inside the passband, or it could not work: at
        // 768 kSPS the window is ±384 kHz.
        assert!(t.shift_hz.abs() < 768_000.0 / 2.0);
        // 500 Hz — the Discovery's bottom end — is still reachable.
        let vlf = tuning(500.0, 0, false, true, 0.0);
        assert_eq!(vlf.lo_khz, MIN_ZERO_IF_LO_KHZ);
        assert!(vlf.shift_hz.abs() < 768_000.0 / 2.0);
    }

    /// Parts per *billion*, not million. A value copied out of an RTL-SDR's ppm
    /// field would be a thousand times too small, and the other way round is a
    /// thousand times too big.
    #[test]
    fn calibration_is_parts_per_billion() {
        // One part per million is a *thousand* ppb, and at 10 MHz it is 10 Hz.
        let t = tuning(10_000_000.0, 1_000, true, true, 0.0);
        assert!((t.adjusted_hz - 10_000_010.0).abs() < 1e-3, "{}", t.adjusted_hz);
        // And it is signed.
        let n = tuning(10_000_000.0, -1_000, true, true, 0.0);
        assert!((n.adjusted_hz - 9_999_990.0).abs() < 1e-3, "{}", n.adjusted_hz);
        assert_eq!(tuning(10_000_000.0, 0, true, true, 0.0).adjusted_hz, 10_000_000.0);
        // The unit is what makes this worth a test: the same number read as ppm
        // would move a 10 MHz dial by 10 kHz instead of 10 Hz.
        let as_ppm = tuning(10_000_000.0, 1_000_000, true, true, 0.0);
        assert!((as_ppm.adjusted_hz - 10_010_000.0).abs() < 1e-3, "{}", as_ppm.adjusted_hz);
    }

    /// The sub-kHz part of the dial and the synthesiser's own error both end up
    /// in the host's shift; there is nowhere else for them to go.
    #[test]
    fn the_shift_absorbs_the_rounding_and_the_synthesiser_error() {
        // 14.074_321 MHz rounds to a 14 074 kHz LO on a low-IF rate, leaving
        // 321 Hz for the oscillator.
        let t = tuning(14_074_321.0, 0, true, true, 0.0);
        assert_eq!(t.lo_khz, 14_074);
        assert!((t.shift_hz - 321.0).abs() < 1e-6, "{}", t.shift_hz);
        // A synthesiser that landed 12 Hz high leaves 12 Hz more to make up.
        let d = tuning(14_074_321.0, 0, true, true, 12.0);
        assert!((d.shift_hz - 333.0).abs() < 1e-6, "{}", d.shift_hz);
    }

    /// A `radio.json` naming a rate this receiver does not have must not stop
    /// it opening.
    #[test]
    fn a_rate_the_receiver_lacks_snaps_to_the_nearest_one() {
        let rates = [192_000.0, 384_000.0, 768_000.0];
        assert_eq!(snap_rate(912_000.0, &rates), Some(2));
        assert_eq!(snap_rate(768_000.0, &rates), Some(2));
        assert_eq!(snap_rate(200_000.0, &rates), Some(0));
        assert_eq!(snap_rate(0.0, &rates), Some(0));
        assert_eq!(snap_rate(1e9, &rates), Some(2));
        assert_eq!(snap_rate(768_000.0, &[]), None);
    }

    /// A request always lands on at least as much attenuation as was asked for,
    /// so "turn it down" never turns it up.
    #[test]
    fn the_attenuator_snaps_up_to_the_next_step() {
        let steps: Vec<f64> = (0..9).map(|i| i as f64 * 6.0).collect();
        assert_eq!(snap_attenuator(0.0, &steps), (0, 0.0));
        assert_eq!(snap_attenuator(1.0, &steps), (1, 6.0));
        assert_eq!(snap_attenuator(6.0, &steps), (1, 6.0));
        assert_eq!(snap_attenuator(47.0, &steps), (8, 48.0));
        // More than the hardware has: as much as it can do, not a wrap-around.
        assert_eq!(snap_attenuator(100.0, &steps), (8, 48.0));
        assert_eq!(snap_attenuator(0.0, &[]), (0, 0.0));
    }
}
