//! The ATS Mini (ESP32-S3 + Si4732) "ad hoc" remote protocol — host side.
//!
//! The firmware exposes a single-character command protocol over TCP, USB
//! serial or BLE. `t` turns on a 500 ms telemetry monitor on the same link;
//! multi-character commands (`F`, `#`) take a trailing CR. This module is the
//! pure codec for both directions — command bytes out, telemetry in — so it can
//! be unit-tested against captured lines without any I/O. The socket, the
//! sound-card audio and the `IqSource` wrapper live in the binary crate.
//!
//! Upstream reference: `esp32-si4732/ats-mini`, `Remote.cpp` (`remoteDoCommand`,
//! `remotePrintStatus`) and `docs/source/remote.md`. The protocol is from the
//! firmware's own source, reimplemented here; see
//! See AGENTS.md, "The ATS Mini", for the bench notes.

/// Default TCP port of the firmware's ad hoc control server.
pub const DEFAULT_PORT: u16 = 60000;

/// Default mDNS host. The bench box has no mDNS resolver, so an IP works too.
pub const DEFAULT_HOST: &str = "atsmini.local";

/// The monitor cadence the firmware emits (`remoteTickTime`, 500 ms).
pub const MONITOR_PERIOD_MS: u32 = 500;

/// The modulation the *radio* is running, as reported in the telemetry `mode`
/// field. This is the hardware demod; sdroxide's own receive mode only decides
/// what it does with the resulting audio.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FirmwareMode {
    Am,
    Lsb,
    Usb,
    Fm,
}

impl FirmwareMode {
    /// Parse the telemetry spelling.
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "AM" => Some(Self::Am),
            "LSB" => Some(Self::Lsb),
            "USB" => Some(Self::Usb),
            "FM" => Some(Self::Fm),
            _ => None,
        }
    }

    /// The sdroxide receive mode that corresponds — the Si4732's FM is wide
    /// broadcast FM.
    pub fn as_rx_mode(self) -> crate::Mode {
        match self {
            Self::Am => crate::Mode::Am,
            Self::Lsb => crate::Mode::Lsb,
            Self::Usb => crate::Mode::Usb,
            Self::Fm => crate::Mode::Wfm,
        }
    }

    /// The telemetry spelling.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Am => "AM",
            Self::Lsb => "LSB",
            Self::Usb => "USB",
            Self::Fm => "FM",
        }
    }
}

/// The four demodulators the Si4732 has, as sdroxide receive modes.
///
/// This is the ATS Mini popup's mode row, in full and in one place: the radio
/// is receive-only, so every band offers every one of them and no band rule may
/// grey one out. In particular every band the firmware defaults to `AM` also
/// accepts `LSB` and `USB` — the mode cycle is `LSB → USB → AM`, so the
/// sidebands are reachable wherever AM is, and most broadcast DX is on a
/// sideband. `every_band_offers_every_demodulator` pins it.
pub const DEMOD_MODES: [crate::Mode; 4] =
    [crate::Mode::Am, crate::Mode::Lsb, crate::Mode::Usb, crate::Mode::Wfm];

/// One line of the 500 ms monitor, parsed.
///
/// Field order from `remotePrintStatus`:
/// `version, freq, bfo, bandCal, band, mode, step, bw, agc, volume, rssi, snr,
/// tuningCap, voltage, seq`.
#[derive(Clone, Debug, PartialEq)]
pub struct Telemetry {
    /// Firmware version ×100 (`240` = v2.40).
    pub version: u16,
    /// Frequency in the firmware's units: 10 kHz for FM, kHz for AM/SSB.
    pub freq_raw: u32,
    /// BFO in Hz (SSB only; 0 in the other modes).
    pub bfo_hz: i32,
    /// Per-band SSB calibration in Hz (SSB only).
    pub band_cal_hz: i32,
    /// Band name, e.g. `VHF`, `ALL`, `CB` (see the handover doc for the table).
    pub band: String,
    pub mode: FirmwareMode,
    /// Step description, e.g. `100k`, `1k`.
    pub step: String,
    /// Bandwidth description, e.g. `Auto`, `3.0k`.
    pub bandwidth: String,
    pub agc: u16,
    /// 0–63, 0 = mute.
    pub volume: u16,
    /// 0–127 dBµV.
    pub rssi_dbuv: u16,
    /// 0–127 dB.
    pub snr_db: u16,
    /// Antenna tuning capacitor, 0–6143.
    pub tuning_cap: u16,
    /// Battery/supply voltage in volts (already scaled by the firmware).
    pub voltage: f32,
    /// 0–255, wrapping.
    pub seq: u16,
}

impl Telemetry {
    /// Parse one telemetry line. `None` for anything else — a command echo or
    /// an `Error:` reply shares the socket and must not be mistaken for state.
    pub fn parse(line: &str) -> Option<Self> {
        let line = line.trim_end_matches(['\r', '\n']);
        let f: Vec<&str> = line.split(',').collect();
        if f.len() != 15 {
            return None;
        }
        let mode = FirmwareMode::parse(f[5])?;
        Some(Telemetry {
            version: f[0].parse().ok()?,
            freq_raw: f[1].parse().ok()?,
            bfo_hz: f[2].parse().ok()?,
            band_cal_hz: f[3].parse().ok()?,
            band: f[4].to_string(),
            mode,
            step: f[6].to_string(),
            bandwidth: f[7].to_string(),
            agc: f[8].parse().ok()?,
            volume: f[9].parse().ok()?,
            rssi_dbuv: f[10].parse().ok()?,
            snr_db: f[11].parse().ok()?,
            tuning_cap: f[12].parse().ok()?,
            voltage: f[13].parse().ok()?,
            seq: f[14].parse().ok()?,
        })
    }

    /// The displayed receive frequency in Hz.
    ///
    /// FM reports `freq_raw` in 10 kHz units; AM/SSB in kHz, and in SSB the BFO
    /// is added (`remotePrintStatus`'s "display frequency" rule).
    pub fn dial_hz(&self) -> f64 {
        match self.mode {
            FirmwareMode::Fm => f64::from(self.freq_raw) * 10_000.0,
            _ => f64::from(self.freq_raw) * 1000.0 + f64::from(self.bfo_hz),
        }
    }

    /// Firmware version as a float (v2.40 → 2.40).
    pub fn fw_version(&self) -> f32 {
        f32::from(self.version) / 100.0
    }
}

/// One of the receiver's bands, in the order `B` steps through them.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BandInfo {
    pub name: &'static str,
    /// Where the band was parked when the table was captured. Used to tell the
    /// two bands named `15M` apart, and as a tooltip.
    pub default_hz: f64,
    pub mode: FirmwareMode,
    /// The band's tuning range, from the firmware's manual. The bands overlap
    /// (`ALL` covers everything, `CB` overlaps `10M`), so "does this band hold
    /// `hz`" is a question with more than one right answer; this is only used
    /// to *stay* in the band the operator is already on.
    pub min_hz: f64,
    pub max_hz: f64,
}

impl BandInfo {
    pub fn contains(&self, hz: f64) -> bool {
        (self.min_hz..=self.max_hz).contains(&hz)
    }
}

/// The band to put a tuning target in: stay where we are if its range holds the
/// frequency, else `ALL` for HF (150 kHz–30 MHz, general coverage) or `VHF` for
/// 64–108 MHz. Using the general-coverage band avoids the overlap the metre
/// bands have, so a tune lands every time.
pub fn band_for_tuning(hz: f64, current: Option<usize>) -> Option<usize> {
    if (64_000_000.0..=108_000_000.0).contains(&hz) {
        return (current != Some(0)).then_some(0);
    }
    if (150_000.0..=30_000_000.0).contains(&hz) {
        if current.is_some_and(|c| BANDS[c].contains(hz)) {
            return None; // already in a band that holds it: do not move
        }
        return Some(1); // ALL
    }
    None
}

/// The firmware's bands (v2.40 bench radio), in `B` cycle order, with the
/// default frequency and mode each was found on.
///
/// **Not a promise.** The firmware lets the operator edit these, so this is
/// "what the radio offers" at its defaults, used to draw a band list and to
/// work out which band the radio is on. The `11M` here is the 25.6–26.1 MHz
/// *broadcast* band and `CB` is the 27 MHz citizens' band.
#[rustfmt::skip]
pub const BANDS: [BandInfo; 28] = [
    BandInfo { name: "VHF",  default_hz: 103_000_000.0, mode: FirmwareMode::Fm,  min_hz: 64_000_000.0, max_hz: 108_000_000.0 },
    BandInfo { name: "ALL",  default_hz: 15_000_000.0,  mode: FirmwareMode::Am,  min_hz: 150_000.0,    max_hz: 30_000_000.0 },
    BandInfo { name: "11M",  default_hz: 25_850_000.0,  mode: FirmwareMode::Am,  min_hz: 25_600_000.0, max_hz: 26_100_000.0 },
    BandInfo { name: "13M",  default_hz: 21_650_000.0,  mode: FirmwareMode::Am,  min_hz: 21_500_000.0, max_hz: 21_900_000.0 },
    BandInfo { name: "15M",  default_hz: 18_950_000.0,  mode: FirmwareMode::Am,  min_hz: 18_900_000.0, max_hz: 19_100_000.0 },
    BandInfo { name: "16M",  default_hz: 17_650_000.0,  mode: FirmwareMode::Am,  min_hz: 17_400_000.0, max_hz: 18_100_000.0 },
    BandInfo { name: "19M",  default_hz: 15_450_000.0,  mode: FirmwareMode::Am,  min_hz: 15_100_000.0, max_hz: 15_900_000.0 },
    BandInfo { name: "22M",  default_hz: 13_650_000.0,  mode: FirmwareMode::Am,  min_hz: 13_500_000.0, max_hz: 13_900_000.0 },
    BandInfo { name: "25M",  default_hz: 11_850_000.0,  mode: FirmwareMode::Am,  min_hz: 11_000_000.0, max_hz: 13_000_000.0 },
    BandInfo { name: "31M",  default_hz: 9_650_000.0,   mode: FirmwareMode::Am,  min_hz: 9_000_000.0,  max_hz: 11_000_000.0 },
    BandInfo { name: "41M",  default_hz: 7_300_000.0,   mode: FirmwareMode::Am,  min_hz: 7_000_000.0,  max_hz: 9_000_000.0 },
    BandInfo { name: "49M",  default_hz: 6_000_000.0,   mode: FirmwareMode::Am,  min_hz: 5_000_000.0,  max_hz: 7_000_000.0 },
    BandInfo { name: "60M",  default_hz: 4_950_000.0,   mode: FirmwareMode::Am,  min_hz: 4_000_000.0,  max_hz: 5_100_000.0 },
    BandInfo { name: "75M",  default_hz: 3_950_000.0,   mode: FirmwareMode::Am,  min_hz: 3_500_000.0,  max_hz: 4_000_000.0 },
    BandInfo { name: "90M",  default_hz: 3_300_000.0,   mode: FirmwareMode::Am,  min_hz: 3_000_000.0,  max_hz: 3_500_000.0 },
    BandInfo { name: "MW3",  default_hz: 2_500_000.0,   mode: FirmwareMode::Am,  min_hz: 1_700_000.0,  max_hz: 3_500_000.0 },
    BandInfo { name: "MW2",  default_hz: 783_000.0,     mode: FirmwareMode::Am,  min_hz: 495_000.0,    max_hz: 1_701_000.0 },
    BandInfo { name: "MW1",  default_hz: 810_000.0,     mode: FirmwareMode::Am,  min_hz: 150_000.0,    max_hz: 1_800_000.0 },
    BandInfo { name: "160M", default_hz: 1_900_000.0,   mode: FirmwareMode::Lsb, min_hz: 1_800_000.0,  max_hz: 2_000_000.0 },
    BandInfo { name: "80M",  default_hz: 3_800_000.0,   mode: FirmwareMode::Lsb, min_hz: 3_500_000.0,  max_hz: 4_000_000.0 },
    BandInfo { name: "40M",  default_hz: 7_150_000.0,   mode: FirmwareMode::Lsb, min_hz: 7_000_000.0,  max_hz: 7_300_000.0 },
    BandInfo { name: "30M",  default_hz: 10_125_000.0,  mode: FirmwareMode::Lsb, min_hz: 10_000_000.0, max_hz: 10_200_000.0 },
    BandInfo { name: "20M",  default_hz: 14_100_000.0,  mode: FirmwareMode::Usb, min_hz: 14_000_000.0, max_hz: 14_400_000.0 },
    BandInfo { name: "17M",  default_hz: 18_115_000.0,  mode: FirmwareMode::Usb, min_hz: 18_000_000.0, max_hz: 18_200_000.0 },
    BandInfo { name: "15M",  default_hz: 21_225_000.0,  mode: FirmwareMode::Usb, min_hz: 21_000_000.0, max_hz: 21_500_000.0 },
    BandInfo { name: "12M",  default_hz: 24_940_000.0,  mode: FirmwareMode::Usb, min_hz: 24_800_000.0, max_hz: 25_000_000.0 },
    BandInfo { name: "10M",  default_hz: 28_500_000.0,  mode: FirmwareMode::Usb, min_hz: 28_000_000.0, max_hz: 29_700_000.0 },
    BandInfo { name: "CB",   default_hz: 27_135_000.0,  mode: FirmwareMode::Am,  min_hz: 25_000_000.0, max_hz: 28_000_000.0 },
];

/// Which band the radio is on, from the telemetry band name and dial.
///
/// The name alone is not enough: the firmware has two bands called `15M` (the
/// broadcast band and the amateur one), so the nearest default breaks the tie.
pub fn band_index(name: &str, dial_hz: f64) -> Option<usize> {
    let mut best: Option<(usize, f64)> = None;
    for (i, b) in BANDS.iter().enumerate() {
        if b.name != name {
            continue;
        }
        let d = (b.default_hz - dial_hz).abs();
        if best.is_none_or(|(_, bd)| d < bd) {
            best = Some((i, d));
        }
    }
    best.map(|(i, _)| i)
}

/// One of the receiver's memory slots, as the `$` dump reports it and the `#`
/// command takes it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtsMiniMemory {
    /// 1..=99, as numbered on the radio.
    pub slot: u8,
    /// The firmware band name (`VHF`, `ALL`, `CB`, …). A memory names its band,
    /// and the firmware refuses a frequency outside the band it is in.
    pub band: String,
    pub freq_hz: u64,
    pub mode: FirmwareMode,
}

impl AtsMiniMemory {
    /// Parse a `#NN,BAND,HZ,MODE` line — the `$` dump writes exactly the form
    /// the `#` command reads, so this parses both.
    pub fn parse(line: &str) -> Option<Self> {
        let line = line.trim();
        let rest = line.strip_prefix('#').unwrap_or(line);
        let mut parts = rest.split(',');
        let slot: u8 = parts.next()?.trim().parse().ok()?;
        let band = parts.next()?.trim().to_string();
        let freq_hz: u64 = parts.next()?.trim().parse().ok()?;
        let mode = FirmwareMode::parse(parts.next()?.trim())?;
        (1..=99).contains(&slot).then_some(Self { slot, band, freq_hz, mode })
    }

    /// The `#NN,BAND,HZ,MODE\r` the firmware's `#` command takes.
    pub fn command(&self) -> String {
        format!("#{:02},{},{},{}\r", self.slot, self.band, self.freq_hz, self.mode.as_str())
    }
}

/// Toggle the 500 ms telemetry monitor (`t`).
pub fn monitor_toggle() -> char {
    't'
}

/// Set the frequency (`F<Hz>\r`). The firmware rejects it unless it falls
/// inside the *current* band, so the caller cycles bands first.
pub fn set_frequency(hz: u64) -> String {
    format!("F{hz}\r")
}

/// Band up/down (`B`/`b`). There is no direct band-select.
pub fn band_step(up: bool) -> char {
    if up { 'B' } else { 'b' }
}

/// Mode up/down (`M`/`m`): cycles `LSB → USB → AM → LSB`, FM only on VHF.
pub fn mode_step(up: bool) -> char {
    if up { 'M' } else { 'm' }
}

/// Volume up/down (`V`/`v`).
pub fn volume_step(up: bool) -> char {
    if up { 'V' } else { 'v' }
}

/// Bandwidth up/down (`W`/`w`).
pub fn bandwidth_step(up: bool) -> char {
    if up { 'W' } else { 'w' }
}

/// AGC/attenuator up/down (`A`/`a`).
pub fn agc_step(up: bool) -> char {
    if up { 'A' } else { 'a' }
}

/// Tuning-step up/down (`S`/`s`).
pub fn tuning_step(up: bool) -> char {
    if up { 'S' } else { 's' }
}

/// Dump the 99 memory slots (`$`).
pub fn dump_memories() -> char {
    '$'
}

/// Set a memory slot (`#nn,band,hz,mode\r`). `hz` in Hz; 0 clears the slot.
pub fn set_memory(slot: u8, band: &str, hz: u64, mode: FirmwareMode) -> String {
    format!("#{slot:02},{band},{hz},{}\r", mode.as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Lines captured from the bench radio, firmware v2.40.

    #[test]
    fn parses_an_fm_telemetry_line() {
        let t =
            Telemetry::parse("240,10390,0,0,VHF,FM,100k,Auto,0,35,18,0,1,4.46,1").expect("parses");
        assert_eq!(t.version, 240);
        assert_eq!(t.fw_version(), 2.40);
        assert_eq!(t.band, "VHF");
        assert_eq!(t.mode, FirmwareMode::Fm);
        assert_eq!(t.volume, 35);
        assert_eq!(t.rssi_dbuv, 18);
        assert_eq!(t.snr_db, 0);
        assert!((t.voltage - 4.46).abs() < 1e-6);
        // FM: freq_raw is 10 kHz units.
        assert_eq!(t.dial_hz(), 103_900_000.0);
    }

    #[test]
    fn parses_an_am_telemetry_line() {
        let t =
            Telemetry::parse("240,27185,0,0,ALL,AM,1k,3.0k,0,35,19,0,1,4.47,2").expect("parses");
        assert_eq!(t.mode, FirmwareMode::Am);
        assert_eq!(t.band, "ALL");
        // AM: freq_raw is kHz, no BFO.
        assert_eq!(t.dial_hz(), 27_185_000.0);
    }

    #[test]
    fn parses_an_ssb_telemetry_line_and_adds_the_bfo() {
        let t = Telemetry::parse("240,14100,-1500,0,20M,USB,10,3.0k,0,35,33,0,1,4.47,5")
            .expect("parses");
        assert_eq!(t.mode, FirmwareMode::Usb);
        // Displayed = freq_kHz*1000 + bfo.
        assert_eq!(t.dial_hz(), 14_098_500.0);
    }

    #[test]
    fn rejects_command_echoes_and_errors() {
        // The socket carries the echo and any error alongside the monitor.
        for line in [
            "F27185000",
            "Error: Frequency is out of range for the current band",
            "",
            "240,10390,0,0,VHF,FM,100k,Auto,0,35,18,0,1,4.46",
            "a,b,c,d,e,f,g,h,i,j,k,l,m,n,o",
        ] {
            assert!(Telemetry::parse(line).is_none(), "should reject {line:?}");
        }
    }

    #[test]
    fn the_band_index_tells_the_two_15m_bands_apart() {
        // The firmware has two bands named 15M — 18.95 broadcast and 21.225
        // amateur — so the dial, not the name, decides.
        assert_eq!(band_index("15M", 18_950_000.0), Some(4));
        assert_eq!(band_index("15M", 21_225_000.0), Some(24));
        assert_eq!(band_index("VHF", 103_400_000.0), Some(0));
        assert_eq!(band_index("CB", 27_135_000.0), Some(27));
        assert_eq!(band_index("ALL", 27_265_000.0), Some(1));
        assert_eq!(band_index("NOPE", 1_000_000.0), None);
    }

    #[test]
    fn tuning_lands_in_a_band_that_holds_the_frequency() {
        // Stay put when the current band's range holds it (49M is 5–7 MHz).
        assert_eq!(band_for_tuning(6_070_000.0, Some(11)), None);
        // Otherwise general coverage, which holds every HF frequency.
        assert_eq!(band_for_tuning(6_070_000.0, Some(20)), Some(1)); // from 40M
        assert_eq!(band_for_tuning(6_070_000.0, None), Some(1));
        // FM goes to VHF, and stays there.
        assert_eq!(band_for_tuning(100_400_000.0, Some(11)), Some(0));
        assert_eq!(band_for_tuning(100_400_000.0, Some(0)), None);
        // The CB band is kept for a CB frequency.
        assert_eq!(band_for_tuning(27_265_000.0, Some(27)), None);
        // The 30–64 MHz gap is in no band; leave it to `F` to refuse.
        assert_eq!(band_for_tuning(50_000_000.0, Some(1)), None);
    }

    #[test]
    fn firmware_modes_map_back_to_rx_modes() {
        assert_eq!(FirmwareMode::Am.as_rx_mode(), crate::Mode::Am);
        assert_eq!(FirmwareMode::Lsb.as_rx_mode(), crate::Mode::Lsb);
        assert_eq!(FirmwareMode::Usb.as_rx_mode(), crate::Mode::Usb);
        assert_eq!(FirmwareMode::Fm.as_rx_mode(), crate::Mode::Wfm);
    }

    /// The ATS Mini is receive-only, so its mode row is the Si4732's four
    /// demodulators on every band, and no band rule may drop one. In particular
    /// every band the firmware defaults to AM also accepts both sidebands — its
    /// mode cycle is `LSB → USB → AM`, so the sidebands are reachable wherever
    /// AM is, and most broadcast DX is on a sideband. The popup draws this row
    /// from [`DEMOD_MODES`], so pinning the constant is pinning the UI.
    #[test]
    fn every_band_offers_every_demodulator() {
        assert_eq!(DEMOD_MODES.len(), 4, "the Si4732 has four demodulators");
        for b in BANDS {
            assert!(
                DEMOD_MODES.contains(&b.mode.as_rx_mode()),
                "{} defaults to {:?}, which the mode row does not offer",
                b.name,
                b.mode
            );
            if b.mode == FirmwareMode::Am {
                for m in [crate::Mode::Lsb, crate::Mode::Usb] {
                    assert!(
                        DEMOD_MODES.contains(&m),
                        "{} defaults to AM, so {m:?} must stay selectable",
                        b.name
                    );
                }
            }
        }
    }

    #[test]
    fn a_memory_line_parses_and_round_trips() {
        let m = AtsMiniMemory::parse("#01,VHF,107900000,FM").expect("parses");
        assert_eq!(m.slot, 1);
        assert_eq!(m.band, "VHF");
        assert_eq!(m.freq_hz, 107_900_000);
        assert_eq!(m.mode, FirmwareMode::Fm);
        // The dump form and the set form are the same, so it round-trips.
        assert_eq!(m.command(), "#01,VHF,107900000,FM\r");
        assert_eq!(AtsMiniMemory::parse(m.command().trim()), Some(m));

        // Junk, an empty slot and an unknown mode are all refused.
        assert!(AtsMiniMemory::parse("").is_none());
        assert!(AtsMiniMemory::parse("#00,VHF,107900000,FM").is_none());
        assert!(AtsMiniMemory::parse("#05,VHF,107900000,XX").is_none());
    }

    #[test]
    fn command_builders_match_the_firmware_wire_format() {
        assert_eq!(monitor_toggle(), 't');
        assert_eq!(set_frequency(27_185_000), "F27185000\r");
        assert_eq!(band_step(true), 'B');
        assert_eq!(band_step(false), 'b');
        assert_eq!(mode_step(false), 'm');
        assert_eq!(volume_step(true), 'V');
        assert_eq!(bandwidth_step(false), 'w');
        assert_eq!(agc_step(true), 'A');
        assert_eq!(tuning_step(false), 's');
        assert_eq!(dump_memories(), '$');
        assert_eq!(set_memory(1, "VHF", 107_900_000, FirmwareMode::Fm), "#01,VHF,107900000,FM\r");
    }
}
