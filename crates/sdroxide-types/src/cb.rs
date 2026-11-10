//! Licence-free channel plans: the citizens' band, per country, and PMR446.
//!
//! 11 m is not one band everywhere. The channel *set* is country law: the
//! CEPT/FCC 40 channels that most of the world shares, Germany's 80-channel
//! plan that adds a high band, and the UK's 27/81 channels, which sit at
//! different frequencies from anyone else's. An operator on 11 m thinks in
//! channel numbers, so the fork carries the plans and shows the channel.
//!
//! The same table also carries the **PMR446** service at 446 MHz — 16 narrow-FM
//! channels, or 32 dPMR446 FDMA ones — because it is the other licence-free
//! service a CB or SWL operator tunes by channel number, and it wants the same
//! treatment: a channel plan the dial reads in and a `CH nn` tag on the tuning
//! line, without claiming a band of its own (`CbPlan::is_uhf` tells the two
//! apart).
//!
//! ## What a plan does here
//!
//! Only the channels: which frequencies are channels, what the band opens
//! on, and (for the UI) the channel number the dial is on. The band's edges are
//! left wide — 26.965–27.860, covering every plan and the freeband between
//! them — so switching plans never moves the operator off the air or changes
//! what transmits. That is deliberate: a narrower plan is about labelling and
//! the default channel, not about locking the receiver.
//!
//! ## The 40-channel table is not a grid
//!
//! Channels 23–25 are the historical oddity (27.255, 27.235, 27.245), and the
//! spacing elsewhere is three 10 kHz steps then a 20 kHz skip. The table is
//! therefore written out rather than generated from a spacing.

use core::sync::atomic::{AtomicBool, AtomicU8, Ordering};

use serde::{Deserialize, Serialize};

use crate::Mode;

/// The shared 40-channel 27 MHz table, channel 1 first, in hertz.
pub const CH_40: [u32; 40] = [
    26_965_000, 26_975_000, 26_985_000, 27_005_000, 27_015_000, 27_025_000, 27_035_000, 27_055_000,
    27_065_000, 27_075_000, 27_085_000, 27_105_000, 27_115_000, 27_125_000, 27_135_000, 27_155_000,
    27_165_000, 27_175_000, 27_185_000, 27_205_000, 27_215_000, 27_225_000, 27_255_000, 27_235_000,
    27_245_000, 27_265_000, 27_275_000, 27_285_000, 27_295_000, 27_305_000, 27_315_000, 27_325_000,
    27_335_000, 27_345_000, 27_355_000, 27_365_000, 27_375_000, 27_385_000, 27_395_000, 27_405_000,
];

/// Germany's high band is the same table shifted up by this much, channels
/// 41–80 at 27.415–27.855.
const HIGH_OFFSET_HZ: u32 = 450_000;

/// The UK 27/81 channels are a plain 10 kHz grid, 27.60125 upward.
const UK_FIRST_HZ: u32 = 27_601_250;
const CHANNEL_SPACING_HZ: u32 = 10_000;

/// PMR446: the 16 analogue/FM (and DMR TDMA) channels of 446.0–446.2 MHz, a
/// 12.5 kHz grid from 446.00625, as ECC Decision (15)05 doubled the original
/// eight. Channel 8 is the analogue calling and distress channel, channel 9
/// the digital one; both are conventions, not rules, so neither is privileged
/// here.
pub const PMR446_CH_16: [u32; 16] = [
    446_006_250,
    446_018_750,
    446_031_250,
    446_043_750,
    446_056_250,
    446_068_750,
    446_081_250,
    446_093_750,
    446_106_250,
    446_118_750,
    446_131_250,
    446_143_750,
    446_156_250,
    446_168_750,
    446_181_250,
    446_193_750,
];

/// dPMR446: the 32 FDMA channels of the same band, a 6.25 kHz grid from
/// 446.003125. A refinement of the same service rather than a separate one, so
/// it is offered as a plan an operator switches to, not alongside the analogue
/// channels. Channel 19 is the dPMR calling channel by convention.
pub const DPMR446_CH_32: [u32; 32] = [
    446_003_125,
    446_009_375,
    446_015_625,
    446_021_875,
    446_028_125,
    446_034_375,
    446_040_625,
    446_046_875,
    446_053_125,
    446_059_375,
    446_065_625,
    446_071_875,
    446_078_125,
    446_084_375,
    446_090_625,
    446_096_875,
    446_103_125,
    446_109_375,
    446_115_625,
    446_121_875,
    446_128_125,
    446_134_375,
    446_140_625,
    446_146_875,
    446_153_125,
    446_159_375,
    446_165_625,
    446_171_875,
    446_178_125,
    446_184_375,
    446_190_625,
    446_196_875,
];

/// How near a channel the dial must be to be *on* it, for the channel readout.
/// Half a channel, so the number flips at the midpoint between two.
///
/// The 27 MHz plans are 10 kHz apart and PMR446's analogue channels 12.5, so
/// one tolerance does for both; the dPMR446 grid is 6.25 kHz and gets its own
/// below.
const ON_CHANNEL_HZ: f64 = 5_000.0;

/// The same, for the 6.25 kHz dPMR446 grid: half a channel, 3.125 kHz.
const ON_CHANNEL_HZ_DPMR: f64 = 3_125.0;

/// Which country's CB channels the station works.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum CbPlan {
    /// The 40 shared channels plus the high band (41–80): the freeband span,
    /// permissive about mode. The default, and what the 11 m band has always
    /// covered.
    #[default]
    World,
    /// CEPT / EU: the 40 shared channels, FM.
    Cept,
    /// Germany: 80 channels, FM.
    Germany80,
    /// The United Kingdom: 27/81, 40 channels at their own frequencies, FM.
    Uk27_81,
    /// The United States: the 40 shared channels, AM and SSB.
    Us,
    /// Australia: the 40 shared channels, AM and SSB.
    Australia,
    /// PMR446: the licence-exempt European UHF personal radio service,
    /// 446.0–446.2 MHz — 16 narrow FM channels, NFM.
    Pmr446,
    /// dPMR446: the same band's 32 FDMA channels, a 6.25 kHz grid. NFM too —
    /// to a receiver the digital waveform is a signal inside the channel.
    Dpmr446,
}

impl CbPlan {
    pub const ALL: [CbPlan; 8] = [
        CbPlan::World,
        CbPlan::Cept,
        CbPlan::Germany80,
        CbPlan::Uk27_81,
        CbPlan::Us,
        CbPlan::Australia,
        CbPlan::Pmr446,
        CbPlan::Dpmr446,
    ];

    fn index(self) -> u8 {
        match self {
            CbPlan::World => 0,
            CbPlan::Cept => 1,
            CbPlan::Germany80 => 2,
            CbPlan::Uk27_81 => 3,
            CbPlan::Us => 4,
            CbPlan::Australia => 5,
            CbPlan::Pmr446 => 6,
            CbPlan::Dpmr446 => 7,
        }
    }

    /// The chip label.
    pub fn short(self) -> &'static str {
        match self {
            CbPlan::World => "WORLD",
            CbPlan::Cept => "EU",
            CbPlan::Germany80 => "DE",
            CbPlan::Uk27_81 => "UK",
            CbPlan::Us => "US",
            CbPlan::Australia => "AU",
            CbPlan::Pmr446 => "PMR",
            CbPlan::Dpmr446 => "dPMR",
        }
    }

    /// The full name, for the settings dropdown and tooltips.
    pub fn label(self) -> &'static str {
        match self {
            CbPlan::World => "World / freeband — 80 channels",
            CbPlan::Cept => "CEPT / EU — 40 channels",
            CbPlan::Germany80 => "Germany — 80 channels",
            CbPlan::Uk27_81 => "United Kingdom — 27/81, 40 channels",
            CbPlan::Us => "United States — 40 channels",
            CbPlan::Australia => "Australia — 40 channels",
            CbPlan::Pmr446 => "PMR446 — 16 channels (446 MHz)",
            CbPlan::Dpmr446 => "dPMR446 — 32 channels (446 MHz)",
        }
    }

    /// The modes the plan is worked in, for a tooltip.
    pub fn modes(self) -> &'static str {
        match self {
            CbPlan::World => "AM · FM · SSB",
            CbPlan::Cept | CbPlan::Germany80 | CbPlan::Uk27_81 => "FM",
            CbPlan::Us | CbPlan::Australia => "AM · SSB",
            CbPlan::Pmr446 => "FM (narrow)",
            CbPlan::Dpmr446 => "FM carrier · dPMR / DMR data",
        }
    }

    /// Whether this is a 446 MHz PMR plan rather than an 11 m one. The band
    /// selector and the channel readout use this to know which band the plan
    /// belongs to.
    pub fn is_uhf(self) -> bool {
        matches!(self, CbPlan::Pmr446 | CbPlan::Dpmr446)
    }

    /// Every channel of the plan, in order, in hertz.
    pub fn channels(self) -> Vec<u32> {
        match self {
            CbPlan::Uk27_81 => (0..40).map(|i| UK_FIRST_HZ + i * CHANNEL_SPACING_HZ).collect(),
            CbPlan::Cept | CbPlan::Us | CbPlan::Australia => CH_40.to_vec(),
            CbPlan::World | CbPlan::Germany80 => {
                let mut out = CH_40.to_vec();
                out.extend(CH_40.iter().map(|hz| hz + HIGH_OFFSET_HZ));
                out
            }
            CbPlan::Pmr446 => PMR446_CH_16.to_vec(),
            CbPlan::Dpmr446 => DPMR446_CH_32.to_vec(),
        }
    }

    /// The channel the band opens on: channel 25 (27.245), the agreed 11 m
    /// digital calling channel, where the plan has it — the UK's own channels do
    /// not include 27.245, so UK opens on channel 19 (27.78125) instead. The
    /// PMR plans open on their calling channels, channel 8 (446.09375) and
    /// channel 19 (446.115625) respectively, on narrow FM like everything else
    /// there.
    pub fn default_entry(self) -> (f64, Mode) {
        match self {
            CbPlan::Uk27_81 => (f64::from(UK_FIRST_HZ + 18 * CHANNEL_SPACING_HZ), Mode::Usb),
            CbPlan::Pmr446 => (f64::from(PMR446_CH_16[7]), Mode::Nfm),
            CbPlan::Dpmr446 => (f64::from(DPMR446_CH_32[18]), Mode::Nfm),
            _ => (27_245_000.0, Mode::Usb),
        }
    }

    /// The channel the dial is on, as `(number, hz)`, when it is within half a
    /// channel of one.
    pub fn on_channel(self, hz: f64) -> Option<(usize, u32)> {
        let tolerance = if self == CbPlan::Dpmr446 { ON_CHANNEL_HZ_DPMR } else { ON_CHANNEL_HZ };
        self.channels()
            .into_iter()
            .enumerate()
            .map(|(i, c)| (i + 1, c))
            .find(|(_, c)| (f64::from(*c) - hz).abs() < tolerance)
    }
}

/// The station's CB plan, as a discriminant index into [`CbPlan::ALL`].
static CURRENT: AtomicU8 = AtomicU8::new(0);

/// The CB channel plan every channel lookup uses.
pub fn cb_plan() -> CbPlan {
    match CURRENT.load(Ordering::Relaxed) {
        1 => CbPlan::Cept,
        2 => CbPlan::Germany80,
        3 => CbPlan::Uk27_81,
        4 => CbPlan::Us,
        5 => CbPlan::Australia,
        6 => CbPlan::Pmr446,
        7 => CbPlan::Dpmr446,
        _ => CbPlan::World,
    }
}

/// Adopt `p` as the station's CB plan. Called at startup from the config, by the
/// engine when the operator changes it, and on a remote client when the station
/// announces its own.
pub fn set_cb_plan(p: CbPlan) {
    CURRENT.store(p.index(), Ordering::Relaxed);
}

/// Whether the 11 m band may be keyed even though it is not an amateur
/// allocation.
///
/// Off by default, and that default is the whole point: the transmit lockout
/// ([`crate::Band::is_amateur`]) refuses every non-amateur band so a licensed
/// operator cannot key outside their allocation by accident, and 11 m is a
/// separate radio service with its own rules and its own type-approved
/// equipment — not a free-for-all. Turning this on is a deliberate act by an
/// operator who knows that, which is why the interface makes them acknowledge
/// it once.
///
/// The broadcast services (Lw/Mw/Sw/Fm) and general coverage stay locked either
/// way: they are receive-only everywhere. This opens 11 m and nothing else.
static CB_TX_ALLOWED: AtomicBool = AtomicBool::new(false);

/// Whether transmit is currently allowed on the 11 m citizens' band.
pub fn cb_tx_allowed() -> bool {
    CB_TX_ALLOWED.load(Ordering::Relaxed)
}

/// Adopt `v` as the station's 11 m transmit permission. Called at startup from
/// the config, by the engine when the operator changes it, and on a remote
/// client when the station announces its own.
pub fn set_cb_tx_allowed(v: bool) {
    CB_TX_ALLOWED.store(v, Ordering::Relaxed);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_shared_table_is_the_real_cb_channel_list() {
        assert_eq!(CH_40.len(), 40);
        assert_eq!(CH_40[0], 26_965_000, "channel 1");
        assert_eq!(CH_40[24], 27_245_000, "channel 25, the digital calling channel");
        assert_eq!(CH_40[39], 27_405_000, "channel 40");
        // The 23/24/25 quirk: 27.255, 27.235, 27.245 — not a rising grid.
        assert_eq!((CH_40[22], CH_40[23], CH_40[24]), (27_255_000, 27_235_000, 27_245_000));
        // And the 20 kHz skip between 27.035 and 27.055.
        assert_eq!((CH_40[6], CH_40[7]), (27_035_000, 27_055_000));
    }

    #[test]
    fn world_and_germany_are_the_eighty_frequency_plan() {
        let chans = CbPlan::World.channels();
        assert_eq!(chans.len(), 80);
        assert_eq!(chans[40], 27_415_000, "channel 41 is channel 1 plus 450 kHz");
        assert_eq!(chans[79], 27_855_000, "channel 80");
        assert_eq!(CbPlan::Germany80.channels(), chans);
    }

    #[test]
    fn the_uk_plan_is_its_own_grid() {
        let chans = CbPlan::Uk27_81.channels();
        assert_eq!(chans.len(), 40);
        assert_eq!(chans[0], 27_601_250);
        assert_eq!(chans[39], 27_991_250);
        assert_eq!(CbPlan::Uk27_81.default_entry(), (27_781_250.0, Mode::Usb));
    }

    #[test]
    fn the_pmr446_table_is_the_real_channel_list() {
        assert_eq!(PMR446_CH_16.len(), 16);
        assert_eq!(PMR446_CH_16[0], 446_006_250, "channel 1");
        assert_eq!(PMR446_CH_16[7], 446_093_750, "channel 8, the calling channel");
        assert_eq!(PMR446_CH_16[15], 446_193_750, "channel 16");
        // A 12.5 kHz grid throughout, so consecutive channels are 12.5 kHz.
        for w in PMR446_CH_16.windows(2) {
            assert_eq!(w[1] - w[0], 12_500);
        }
        assert_eq!(DPMR446_CH_32.len(), 32);
        assert_eq!(DPMR446_CH_32[0], 446_003_125);
        assert_eq!(DPMR446_CH_32[18], 446_115_625, "channel 19, the dPMR calling channel");
        for w in DPMR446_CH_32.windows(2) {
            assert_eq!(w[1] - w[0], 6_250);
        }
        // Every channel is inside the PMR446 allocation, 446.0–446.2 MHz.
        for c in PMR446_CH_16.iter().chain(DPMR446_CH_32.iter()) {
            assert!((446_000_000..=446_200_000).contains(c), "{c} is outside 446.0–446.2");
        }
    }

    #[test]
    fn the_pmr_plans_open_on_their_calling_channels() {
        assert_eq!(CbPlan::Pmr446.default_entry(), (446_093_750.0, Mode::Nfm));
        assert_eq!(CbPlan::Dpmr446.default_entry(), (446_115_625.0, Mode::Nfm));
        assert!(CbPlan::Pmr446.is_uhf());
        assert!(CbPlan::Dpmr446.is_uhf());
        assert!(!CbPlan::Cept.is_uhf());
    }

    /// The dPMR446 grid is half the width of the others, so its channel
    /// tolerance is half as well: a dial between two channels reads as neither.
    #[test]
    fn the_dpmr_grid_reads_back_at_its_own_tolerance() {
        assert_eq!(CbPlan::Dpmr446.on_channel(446_115_625.0), Some((19, 446_115_625)));
        // Within the half-channel of 19 it reads as 19...
        assert_eq!(CbPlan::Dpmr446.on_channel(446_118_000.0), Some((19, 446_115_625)));
        // ...and past the midpoint it is the next channel's, 20, which is the
        // whole point of halving the tolerance for the 6.25 kHz grid.
        assert_eq!(CbPlan::Dpmr446.on_channel(446_119_000.0), Some((20, 446_121_875)));
        // Dead between two channels reads as neither.
        assert!(CbPlan::Dpmr446.on_channel(446_118_750.0).is_none());
        // The 12.5 kHz analogue grid, by contrast, reads 446.11875 as its
        // channel 10 (446.11875 exactly), so the two plans genuinely differ.
        assert_eq!(CbPlan::Pmr446.on_channel(446_118_750.0), Some((10, 446_118_750)));
    }

    #[test]
    fn a_dial_on_a_channel_reads_back() {
        assert_eq!(CbPlan::Cept.on_channel(27_245_000.0), Some((25, 27_245_000)));
        // Just off it still reads; past the midpoint it does not.
        assert_eq!(CbPlan::Cept.on_channel(27_249_000.0), Some((25, 27_245_000)));
        assert!(CbPlan::Cept.on_channel(27_250_000.0).is_none());
        // 27.245 is not a UK channel, so the UK plan has nothing there.
        assert!(CbPlan::Uk27_81.on_channel(27_245_000.0).is_none());
    }

    #[test]
    fn the_default_entries_are_on_channel() {
        for p in CbPlan::ALL {
            let (hz, _) = p.default_entry();
            assert!(
                p.on_channel(hz).is_some(),
                "{} default {hz} Hz is not on one of its channels",
                p.label()
            );
        }
    }

    #[test]
    fn the_global_round_trips_and_restores() {
        // The only test that touches the global; it restores the default.
        for p in CbPlan::ALL {
            set_cb_plan(p);
            assert_eq!(cb_plan(), p);
        }
        set_cb_plan(CbPlan::default());
        assert_eq!(cb_plan(), CbPlan::World);
    }
}
