//! CLK2 of the Si5351 — the R828D's reference clock.
//!
//! The bundled firmware carries a full Si5351 driver but only ever calls it for
//! CLK0, the ADC sample clock, because it has no tuner driver to need anything
//! else. So the host programs CLK2 itself, over the I2C passthrough. Ported from
//! that same firmware's `si5351aSetFrequencyB`, so both outputs are configured
//! by the same arithmetic.
//!
//! # What must not be touched
//!
//! CLK0, on PLLA, is the LTC2208's sample clock — the only reason this receiver
//! produces samples at all. Everything here confines itself to PLLB, MultiSynth
//! 2 and CLK2:
//!
//! * the PLL reset at register 177 is written `0x80`, PLLB alone, never `0x20`
//!   or `0xA0` — resetting PLLA restarts the ADC clock, which at best gaps the
//!   stream and at worst leaves the FX3's GPIF state machine out of step;
//! * registers 16 (`CLK0_CONTROL`), 26–33 (PLLA) and 42–49 (MultiSynth 0) are
//!   never addressed;
//! * register 3, output enable, is left alone entirely. Its bit 0 is CLK0's,
//!   and the firmware already clears the whole register during its own start-up,
//!   so there is nothing to gain here and an ADC clock to lose.
//!
//! # The register allowlist
//!
//! The firmware permits host writes only to registers 0–3, 9, 15–92, 149–170,
//! 177, 183 and 187. The part fitted is usually an MS5351M rather than a genuine
//! Si5351A, and on that clone a write to a *reserved* register can wedge the I2C
//! slave until the receiver is power-cycled. Every register this module touches
//! — 18, 34–41, 58–65, 177 — is inside the allowlist, and [`ALLOWED_REGS`]
//! exists so that is asserted rather than trusted.
//!
//! # Verified on hardware
//!
//! On an RX888mk2 running firmware 2.3: CLK2 comes up at 16 MHz, the tuner's
//! PLL locks against it, and `GETSTATS` still reports CLK0 running with its
//! control register unchanged — so PLLA and the ADC clock are undisturbed.
//! Tuning is accurate to a few kHz at 96 MHz, which also confirms the 27 MHz
//! reference below to about the same precision.

use crate::error::Result;
use crate::usb::UsbDev;

/// I2C address of the clock generator: 0x60, shifted up into the 8-bit form the
/// passthrough wants.
pub const SI5351_ADDR: u8 = 0xC0;

/// The board's reference oscillator.
///
/// Taken from the firmware, and nothing on the chip reports it back — but a
/// wrong value would put every VHF frequency out by the ratio, and a bench run
/// resolved a broadcast station to within 4 kHz at 96 MHz, so it is right to
/// better than 50 ppm.
pub const REF_HZ: f64 = 27_000_000.0;

/// What the R828D wants on its clock pin. Upstream passes this same figure to
/// `TUNERINIT`, and the tuner driver must be told the same number or its PLL
/// arithmetic is off by the ratio.
pub const TUNER_REF_HZ: f64 = 16_000_000.0;

const REG_CLK2_CONTROL: u8 = 18;
const REG_SYNTH_PLL_B: u8 = 34;
const REG_SYNTH_MS_2: u8 = 58;
const REG_PLL_RESET: u8 = 177;

/// PLLB only. `0x20` would be PLLA, and PLLA is the ADC clock.
const PLL_RESET_B: u8 = 0x80;
/// CLK2 powered up, MultiSynth 2 driving it, sourced from PLLB. Upstream writes
/// this as `0x4C | SI_CLK_SRC_PLL_B`.
const CLK2_ON: u8 = 0x4C | 0x20;
/// Bit 7 of a `CLKn_CONTROL` register powers that output stage down.
const CLK_POWER_DOWN: u8 = 0x80;

/// The denominator of the PLL's fractional multiplier: 20 bits.
const DENOM: u32 = 1_048_575;
/// The synthesiser's internal ceiling.
const PLL_MAX_HZ: f64 = 900_000_000.0;

/// Every register this module is allowed to write, for the test that proves it.
pub const ALLOWED_REGS: &[u8] = &[
    REG_CLK2_CONTROL,
    34,
    35,
    36,
    37,
    38,
    39,
    40,
    41, // PLLB
    58,
    59,
    60,
    61,
    62,
    63,
    64,
    65, // MultiSynth 2
    REG_PLL_RESET,
];

/// The divider chain for an output frequency.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plan {
    /// Integer part of the PLL multiplier.
    pub mult: u32,
    /// Fractional numerator, over [`DENOM`].
    pub num: u32,
    /// MultiSynth output divider — always even, so the divider is integer.
    pub divider: u32,
    /// R-divider bits, for outputs below 1 MHz.
    pub rdiv: u8,
}

/// Pick the divider chain for `hz`, exactly as the firmware does.
///
/// The largest even output divider that puts the PLL under 900 MHz, then the
/// PLL ratio as a 20-bit fraction of the reference.
pub fn plan_for(hz: f64, ref_hz: f64) -> Plan {
    let mut frequency = hz;
    let mut rdiv = 0u8;
    // Outputs below 1 MHz need the final R stage; each step is one bit up in
    // the register's divider field.
    while frequency <= 1_000_000.0 && rdiv < 0x70 {
        frequency *= 2.0;
        rdiv += 0x10;
    }

    let mut divider = (PLL_MAX_HZ / frequency) as u32;
    if divider % 2 == 1 {
        divider -= 1;
    }

    let pll_freq = divider as f64 * frequency;
    let mult = (pll_freq / ref_hz) as u32;
    let rem = pll_freq - mult as f64 * ref_hz;
    let num = (rem * DENOM as f64 / ref_hz) as u32;

    Plan { mult, num, divider, rdiv }
}

/// What the chip actually produces for a plan.
///
/// Derived rather than echoed, so the figure that reaches the log is the one the
/// hardware will make and not the one that was asked for.
pub fn achieved_hz(p: &Plan, ref_hz: f64) -> f64 {
    let pll = ref_hz * (p.mult as f64 + p.num as f64 / DENOM as f64);
    let r = 1u32 << (p.rdiv >> 4);
    pll / p.divider as f64 / r as f64
}

/// The eight bytes of a fractional PLL setup.
fn pll_regs(p: &Plan) -> [u8; 8] {
    let p1 = 128 * p.mult + (128 * p.num) / DENOM - 512;
    let p2 = 128 * p.num - DENOM * ((128 * p.num) / DENOM);
    let p3 = DENOM;
    pack(p1, p2, p3, 0)
}

/// The same for an integer MultiSynth: `P2 = 0`, `P3 = 1` forces integer mode.
fn ms_regs(divider: u32, rdiv: u8) -> [u8; 8] {
    pack(128 * divider - 512, 0, 1, rdiv)
}

/// The shared P1/P2/P3 register packing, straight from AN619.
fn pack(p1: u32, p2: u32, p3: u32, rdiv: u8) -> [u8; 8] {
    [
        ((p3 & 0x0000_FF00) >> 8) as u8,
        (p3 & 0x0000_00FF) as u8,
        (((p1 & 0x0003_0000) >> 16) as u8) | rdiv,
        ((p1 & 0x0000_FF00) >> 8) as u8,
        (p1 & 0x0000_00FF) as u8,
        (((p3 & 0x000F_0000) >> 12) | ((p2 & 0x000F_0000) >> 16)) as u8,
        ((p2 & 0x0000_FF00) >> 8) as u8,
        (p2 & 0x0000_00FF) as u8,
    ]
}

/// Program CLK2 to `hz` and switch it on. Returns what the chip will produce.
pub fn set_clk2_hz(usb: &UsbDev, hz: f64) -> Result<f64> {
    let p = plan_for(hz, REF_HZ);
    usb.i2c_write(SI5351_ADDR, REG_SYNTH_PLL_B, &pll_regs(&p))?;
    usb.i2c_write(SI5351_ADDR, REG_SYNTH_MS_2, &ms_regs(p.divider, p.rdiv))?;
    usb.i2c_write(SI5351_ADDR, REG_PLL_RESET, &[PLL_RESET_B])?;
    usb.i2c_write(SI5351_ADDR, REG_CLK2_CONTROL, &[CLK2_ON])?;

    let achieved = achieved_hz(&p, REF_HZ);
    tracing::info!("RX-888: tuner reference CLK2 {:.6} MHz", achieved / 1e6);
    Ok(achieved)
}

/// Power CLK2 down.
///
/// Deliberately only the output stage: PLLB is left programmed, and register 3
/// is not touched, because its neighbouring bit is the ADC clock's. Upstream's
/// `si5351aSetFrequencyB(0)` does exactly this.
pub fn clk2_off(usb: &UsbDev) -> Result<()> {
    usb.i2c_write(SI5351_ADDR, REG_CLK2_CONTROL, &[CLK_POWER_DOWN])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_tuner_reference_lands_on_sixteen_megahertz() {
        let p = plan_for(TUNER_REF_HZ, REF_HZ);
        let got = achieved_hz(&p, REF_HZ);
        assert!(
            (got - TUNER_REF_HZ).abs() < 1.0,
            "CLK2 planned for {TUNER_REF_HZ} came out at {got}"
        );
    }

    /// The synthesiser has a hard internal range, and an even divider is what
    /// keeps the MultiSynth in integer mode.
    ///
    /// The ceiling is inclusive: a frequency that divides 900 MHz exactly — 5,
    /// 6, 10, 15, 18 MHz — lands the PLL precisely on 900 MHz by construction,
    /// which is the top of the datasheet's range and what the firmware's own
    /// `900000000 / frequency` produces too. The tolerance is for the float
    /// reconstruction of the ratio, not slack in the limit.
    #[test]
    fn the_pll_stays_in_range_with_an_even_divider() {
        for mhz in 4..=40 {
            let hz = f64::from(mhz) * 1e6;
            let p = plan_for(hz, REF_HZ);
            let pll = REF_HZ * (p.mult as f64 + p.num as f64 / DENOM as f64);
            assert!(pll >= 600e6, "{mhz} MHz put the PLL at {pll}, below the VCO floor");
            assert!(pll <= PLL_MAX_HZ + 1.0, "{mhz} MHz put the PLL at {pll}, over the ceiling");
            assert_eq!(p.divider % 2, 0, "{mhz} MHz chose an odd divider");

            // The multiplier's fraction is 20 bits, so one step at the PLL is
            // `REF_HZ / DENOM` ≈ 25.7 Hz, and at the output that step is
            // divided down. Anything inside it is the part's resolution rather
            // than an error in the plan.
            let step_hz = REF_HZ / DENOM as f64 / p.divider as f64;
            let err = (achieved_hz(&p, REF_HZ) - hz).abs();
            assert!(err <= step_hz, "{mhz} MHz was out by {err} Hz, more than the {step_hz} step");
        }
    }

    /// The exact bytes for the 16 MHz case, so a change to the packing has to
    /// be deliberate.
    #[test]
    fn the_register_blocks_match_an619s_packing() {
        let p = plan_for(TUNER_REF_HZ, REF_HZ);
        // 900 MHz / 16 MHz = 56 (even), PLL = 896 MHz = 27 MHz × 33.185…
        assert_eq!(p.divider, 56);
        assert_eq!(p.mult, 33);

        // P3 = 1048575 = 0x0FFFFF, so the low byte pair is 0xFF 0xFF and the
        // high nibble lands in byte 5.
        let pll = pll_regs(&p);
        assert_eq!(pll[0], 0xFF);
        assert_eq!(pll[1], 0xFF);

        // Integer MultiSynth: P1 = 128 × 56 − 512 = 6656 = 0x1A00, P2 = 0,
        // P3 = 1.
        let ms = ms_regs(p.divider, p.rdiv);
        assert_eq!(ms, [0x00, 0x01, 0x00, 0x1A, 0x00, 0x00, 0x00, 0x00]);
    }

    /// The safety net for the whole VHF feature: nothing here may reach a
    /// register the firmware would refuse, because on an MS5351M that can cost
    /// a power cycle.
    #[test]
    fn nothing_outside_the_firmwares_allowlist_is_written() {
        let allowed = |r: u8| {
            r <= 3
                || r == 9
                || (15..=92).contains(&r)
                || (149..=170).contains(&r)
                || r == 177
                || r == 183
                || r == 187
        };
        for &r in ALLOWED_REGS {
            assert!(allowed(r), "register {r} is reserved on this part");
        }
    }

    /// The one mistake here that would take the *HF* path down with it.
    #[test]
    fn the_adc_clock_is_never_addressed() {
        // CLK0_CONTROL, PLLA and MultiSynth 0.
        let adc: Vec<u8> = std::iter::once(16u8).chain(26..=33).chain(42..=49).collect();
        for &r in ALLOWED_REGS {
            assert!(!adc.contains(&r), "register {r} belongs to the ADC clock");
        }
        // And the reset must name PLLB alone.
        assert_eq!(PLL_RESET_B & 0x20, 0, "the PLL reset would also reset PLLA");
        assert_eq!(PLL_RESET_B, 0x80);
    }

    /// The write blocks are 8 bytes and start at the addresses in the
    /// allowlist, so a full PLL or MultiSynth write cannot run off the end of
    /// its own block.
    #[test]
    fn the_eight_byte_blocks_stay_inside_the_allowlist() {
        for start in [REG_SYNTH_PLL_B, REG_SYNTH_MS_2] {
            for r in start..start + 8 {
                assert!(ALLOWED_REGS.contains(&r), "register {r} is written but not allowed");
            }
        }
    }
}
