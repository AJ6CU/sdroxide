use crate::{Band, Mode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SignalFamily {
    Amateur,
    Broadcast,
    Utility,
    Marine,
    Aviation,
    Digital,
    Satellite,
    Radar,
    Time,
    Numbers,
    Other,
}

impl SignalFamily {
    pub fn label(self) -> &'static str {
        match self {
            SignalFamily::Amateur => "Amateur",
            SignalFamily::Broadcast => "Broadcast",
            SignalFamily::Utility => "Utility",
            SignalFamily::Marine => "Marine",
            SignalFamily::Aviation => "Aviation",
            SignalFamily::Digital => "Digital",
            SignalFamily::Satellite => "Satellite",
            SignalFamily::Radar => "Radar",
            SignalFamily::Time => "Time",
            SignalFamily::Numbers => "Numbers",
            SignalFamily::Other => "Other",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SignalProfile {
    pub name: &'static str,
    pub family: SignalFamily,
    pub modulation: &'static str,
    pub bandwidth_hz: (f64, f64),
    pub bands: &'static [Band],
    pub frequencies_hz: &'static [f64],
    pub modes: &'static [Mode],
    pub summary: &'static str,
    pub sigidwiki: Option<&'static str>,
}

impl SignalProfile {
    pub fn bandwidth_text(&self) -> String {
        let (lo, hi) = self.bandwidth_hz;
        if hi <= 0.0 {
            "unknown".into()
        } else if lo <= 0.0 {
            format!("up to {}", span_text(hi))
        } else if (hi - lo) < lo * 0.05 {
            span_text(lo)
        } else {
            format!("{}–{}", span_text(lo), span_text(hi))
        }
    }

    pub fn matches(&self, needle: &str) -> bool {
        let needle = needle.trim().to_lowercase();
        needle.is_empty()
            || self.name.to_lowercase().contains(&needle)
            || self.summary.to_lowercase().contains(&needle)
            || self.modulation.to_lowercase().contains(&needle)
    }

    pub fn wiki_url(&self) -> Option<String> {
        self.sigidwiki.map(|slug| format!("{SIGIDWIKI_BASE}{slug}"))
    }
}

pub const SIGIDWIKI_BASE: &str = "https://www.sigidwiki.com/wiki/";

fn span_text(hz: f64) -> String {
    if hz >= 1e9 {
        format!("{:.3} GHz", hz / 1e9)
    } else if hz >= 1e6 {
        format!("{:.3} MHz", hz / 1e6)
    } else if hz >= 1e3 {
        format!("{:.3} kHz", hz / 1e3)
    } else {
        format!("{hz:.0} Hz")
    }
}

const HF: &[Band] = &[
    Band::M160,
    Band::M80,
    Band::M60,
    Band::M40,
    Band::M30,
    Band::M20,
    Band::M17,
    Band::M15,
    Band::M12,
    Band::M11,
    Band::M10,
];
const VHF_UHF: &[Band] = &[Band::M6, Band::M4, Band::M2, Band::M125, Band::M70];
const HF_VHF: &[Band] = &[
    Band::M160,
    Band::M80,
    Band::M60,
    Band::M40,
    Band::M30,
    Band::M20,
    Band::M17,
    Band::M15,
    Band::M12,
    Band::M11,
    Band::M10,
    Band::M6,
    Band::M4,
    Band::M2,
];
const SW: &[Band] = &[Band::Sw];

pub static PROFILES: &[SignalProfile] = &[
    SignalProfile {
        name: "FT8",
        family: SignalFamily::Digital,
        modulation: "8-GFSK, 6.25 baud",
        bandwidth_hz: (0.0, 50.0),
        bands: HF_VHF,
        frequencies_hz: &[
            1_840_000.0,
            3_573_000.0,
            7_074_000.0,
            10_136_000.0,
            14_074_000.0,
            18_100_000.0,
            21_074_000.0,
            24_915_000.0,
            27_245_000.0,
            28_074_000.0,
            50_313_000.0,
            144_174_000.0,
        ],
        modes: &[Mode::Ft8],
        summary: "Weak-signal QSO mode: 15 s slots, 13 characters, decodes far below the noise.",
        sigidwiki: Some("FT8"),
    },
    SignalProfile {
        name: "FT4",
        family: SignalFamily::Digital,
        modulation: "4-GFSK, 20.83 baud",
        bandwidth_hz: (0.0, 90.0),
        bands: HF_VHF,
        frequencies_hz: &[
            3_575_000.0,
            7_047_500.0,
            10_140_000.0,
            14_080_000.0,
            18_104_000.0,
            21_140_000.0,
            24_919_000.0,
            28_180_000.0,
            50_318_000.0,
        ],
        modes: &[Mode::Ft4],
        summary: "FT8's faster sibling: 7.5 s slots, contest and fast-QSO work.",
        sigidwiki: Some("FT4"),
    },
    SignalProfile {
        name: "FT2",
        family: SignalFamily::Digital,
        modulation: "4-GFSK, 41.67 baud",
        bandwidth_hz: (0.0, 170.0),
        bands: HF_VHF,
        frequencies_hz: &[
            3_578_000.0,
            7_062_000.0,
            10_144_000.0,
            14_084_000.0,
            18_108_000.0,
            21_144_000.0,
            24_923_000.0,
            28_184_000.0,
            50_316_000.0,
            144_177_000.0,
        ],
        modes: &[Mode::Ft2],
        summary: "FT4 with the symbol rate doubled: a 2.5 s burst in a 3.75 s slot.",
        sigidwiki: None,
    },
    SignalProfile {
        name: "JS8Call",
        family: SignalFamily::Digital,
        modulation: "8-FSK, 6.25–31 baud",
        bandwidth_hz: (0.0, 160.0),
        bands: HF,
        frequencies_hz: &[
            1_842_000.0,
            3_578_000.0,
            7_078_000.0,
            10_130_000.0,
            14_078_000.0,
            18_104_000.0,
            21_078_000.0,
            24_922_000.0,
            27_245_000.0,
            28_078_000.0,
        ],
        modes: &[Mode::Js8],
        summary: "Keyboard messaging on FT8's waveform: free text, directed commands, heartbeats.",
        sigidwiki: Some("JS8"),
    },
    SignalProfile {
        name: "WSPR",
        family: SignalFamily::Digital,
        modulation: "4-FSK, 1.46 baud",
        bandwidth_hz: (0.0, 6.0),
        bands: HF,
        frequencies_hz: &[
            1_836_600.0,
            3_568_600.0,
            7_038_600.0,
            10_138_700.0,
            14_095_600.0,
            18_104_600.0,
            21_094_600.0,
            24_924_600.0,
            28_124_600.0,
        ],
        modes: &[Mode::Wspr],
        summary: "Propagation beacon: callsign, grid and power, 110 s in a two-minute slot.",
        sigidwiki: Some("WSPR"),
    },
    SignalProfile {
        name: "PI4",
        family: SignalFamily::Digital,
        modulation: "4-FSK, 6 baud",
        bandwidth_hz: (0.0, 30.0),
        bands: HF_VHF,
        frequencies_hz: &[
            3_592_000.0,
            7_038_000.0,
            10_140_000.0,
            14_099_000.0,
            18_110_000.0,
            21_149_000.0,
            24_929_000.0,
            28_199_000.0,
            50_472_000.0,
            144_492_000.0,
        ],
        modes: &[Mode::Pi4],
        summary: "IARU next-generation propagation beacon: a callsign in 24 s on a one-minute cycle.",
        sigidwiki: Some("PI4"),
    },
    SignalProfile {
        name: "PSK31",
        family: SignalFamily::Digital,
        modulation: "BPSK/QPSK, 31.25 baud",
        bandwidth_hz: (20.0, 100.0),
        bands: HF,
        frequencies_hz: &[
            1_838_000.0,
            3_580_000.0,
            7_040_000.0,
            7_070_000.0,
            10_142_000.0,
            14_070_000.0,
            18_097_000.0,
            21_070_000.0,
            24_920_000.0,
            28_120_000.0,
        ],
        modes: &[Mode::Psk],
        summary: "Narrow keyboard mode: varicode over phase-shift keying, typed character by character.",
        sigidwiki: Some("Phase_Shift_Keying_(PSK)"),
    },
    SignalProfile {
        name: "RTTY",
        family: SignalFamily::Digital,
        modulation: "FSK, 45.45 baud, 170 Hz shift",
        bandwidth_hz: (200.0, 500.0),
        bands: HF,
        frequencies_hz: &[
            3_580_000.0,
            3_590_000.0,
            7_040_000.0,
            7_080_000.0,
            10_142_000.0,
            14_080_000.0,
            14_083_000.0,
            18_105_000.0,
            21_080_000.0,
            24_925_000.0,
            28_080_000.0,
        ],
        modes: &[Mode::Rtty, Mode::RttyFm],
        summary: "Baudot teleprinter: two tones, mark and space, a mechanical chatter on the waterfall.",
        sigidwiki: Some("RTTY"),
    },
    SignalProfile {
        name: "Olivia",
        family: SignalFamily::Digital,
        modulation: "MFSK with FEC",
        bandwidth_hz: (250.0, 2_000.0),
        bands: HF,
        frequencies_hz: &[14_072_000.0],
        modes: &[Mode::Olivia],
        summary: "Multi-tone keyboard mode that copies through heavy fading and interference.",
        sigidwiki: Some("Olivia"),
    },
    SignalProfile {
        name: "THOR",
        family: SignalFamily::Digital,
        modulation: "MFSK, 18 tones with FEC",
        bandwidth_hz: (200.0, 2_000.0),
        bands: HF,
        frequencies_hz: &[14_075_000.0],
        modes: &[Mode::Thor],
        summary: "DominoEX-family multi-tone keyboard mode with forward error correction.",
        sigidwiki: Some("THOR"),
    },
    SignalProfile {
        name: "FSQ",
        family: SignalFamily::Digital,
        modulation: "IFK, 33 tones",
        bandwidth_hz: (1_500.0, 2_200.0),
        bands: HF,
        frequencies_hz: &[
            1_842_000.0,
            3_588_000.0,
            7_105_000.0,
            10_144_000.0,
            14_105_000.0,
            18_106_000.0,
            21_105_000.0,
            24_925_000.0,
            28_105_000.0,
        ],
        modes: &[Mode::Fsq],
        summary: "Fast Simple QSO: directed messages, contacts and images on a multi-tone keyboard mode.",
        sigidwiki: Some("FSQ"),
    },
    SignalProfile {
        name: "Hellschreiber",
        family: SignalFamily::Digital,
        modulation: "OOK/facsimile",
        bandwidth_hz: (100.0, 500.0),
        bands: HF,
        frequencies_hz: &[14_063_000.0],
        modes: &[Mode::Hell],
        summary: "Facsimile text painted dot by dot; the eye reads the raster with no decoding.",
        sigidwiki: Some("Hellschreiber"),
    },
    SignalProfile {
        name: "FreeDV / RADE",
        family: SignalFamily::Digital,
        modulation: "OFDM/QAM neural codec",
        bandwidth_hz: (1_000.0, 1_900.0),
        bands: HF,
        frequencies_hz: &[14_236_000.0, 7_177_000.0, 21_236_000.0],
        modes: &[Mode::Rade],
        summary: "Digital voice over HF: a machine-learned codec inside an OFDM waveform.",
        sigidwiki: Some("FreeDV_COHPSK"),
    },
    SignalProfile {
        name: "SSTV",
        family: SignalFamily::Digital,
        modulation: "FM-analogue image subcarrier",
        bandwidth_hz: (1_500.0, 3_000.0),
        bands: HF,
        frequencies_hz: &[
            3_730_000.0,
            3_845_000.0,
            7_165_000.0,
            7_171_000.0,
            14_230_000.0,
            21_340_000.0,
            27_700_000.0,
            28_680_000.0,
        ],
        modes: &[Mode::Sstv],
        summary: "Slow-scan television: a picture in about two minutes, heard as a warbling chirp.",
        sigidwiki: Some("SSTV"),
    },
    SignalProfile {
        name: "SSTV (FM)",
        family: SignalFamily::Digital,
        modulation: "FM image subcarrier",
        bandwidth_hz: (1_500.0, 3_000.0),
        bands: VHF_UHF,
        frequencies_hz: &[50_510_000.0, 144_500_000.0, 433_400_000.0],
        modes: &[Mode::SstvFm],
        summary: "Slow-scan television keyed onto an FM carrier, the VHF and UHF way.",
        sigidwiki: Some("SSTV"),
    },
    SignalProfile {
        name: "RIFP",
        family: SignalFamily::Digital,
        modulation: "CPFSK, 4800 baud",
        bandwidth_hz: (20_000.0, 25_000.0),
        bands: &[Band::M70, Band::M2],
        frequencies_hz: &[433_920_000.0],
        modes: &[Mode::Rifp],
        summary: "Radio Image Framing Protocol: packetised images at 4800 baud on 70 cm and 2 m.",
        sigidwiki: None,
    },
    SignalProfile {
        name: "AX.25 packet (VHF/UHF)",
        family: SignalFamily::Digital,
        modulation: "FM, 1200/9600 baud AFSK/G3RUH",
        bandwidth_hz: (12_000.0, 20_000.0),
        bands: VHF_UHF,
        frequencies_hz: &[144_900_000.0, 145_010_000.0, 430_500_000.0],
        modes: &[Mode::Packet],
        summary: "Amateur packet radio on FM: connected links, bulletin boards, Winlink.",
        sigidwiki: Some("PACKET"),
    },
    SignalProfile {
        name: "AX.25 packet (HF)",
        family: SignalFamily::Digital,
        modulation: "FSK, 300 baud, 200 Hz shift",
        bandwidth_hz: (300.0, 1_000.0),
        bands: HF,
        frequencies_hz: &[3_600_000.0, 7_100_000.0, 14_100_000.0],
        modes: &[Mode::PacketHf],
        summary: "Packet radio on the HF bands: 300 baud AFSK on a sideband.",
        sigidwiki: Some("PACKET"),
    },
    SignalProfile {
        name: "APRS",
        family: SignalFamily::Digital,
        modulation: "FM, 1200 baud AFSK",
        bandwidth_hz: (12_000.0, 20_000.0),
        bands: VHF_UHF,
        frequencies_hz: &[
            144_390_000.0,
            144_640_000.0,
            144_800_000.0,
            145_175_000.0,
            432_500_000.0,
            445_925_000.0,
        ],
        modes: &[Mode::Aprs],
        summary: "Automatic Packet Reporting System: positions, weather and messages on one FM channel.",
        sigidwiki: Some("APRS"),
    },
    SignalProfile {
        name: "AtChat NET",
        family: SignalFamily::Digital,
        modulation: "COFDM, dynamic master",
        bandwidth_hz: (2_400.0, 3_000.0),
        bands: HF,
        frequencies_hz: &[3_604_000.0, 7_100_000.0, 14_100_000.0],
        modes: &[Mode::AtChat],
        summary: "Multi-station chat and ARQ file transfer over a COFDM keyboard mode.",
        sigidwiki: None,
    },
    SignalProfile {
        name: "UVPacket",
        family: SignalFamily::Digital,
        modulation: "pi/4-DQPSK burst",
        bandwidth_hz: (10_000.0, 25_000.0),
        bands: VHF_UHF,
        frequencies_hz: &[],
        modes: &[Mode::UvPacket],
        summary: "Private VHF/UHF packet: a short burst with a header and a byte payload.",
        sigidwiki: None,
    },
    SignalProfile {
        name: "MSK144",
        family: SignalFamily::Digital,
        modulation: "MSK, 2000 baud",
        bandwidth_hz: (300.0, 1_000.0),
        bands: &[Band::M6, Band::M2],
        frequencies_hz: &[50_280_000.0, 144_150_000.0],
        modes: &[Mode::Msk144],
        summary: "Meteor scatter on 6 m and 2 m: continuous 72 ms frames in a 15 s period.",
        sigidwiki: Some("MSK144"),
    },
    SignalProfile {
        name: "JT65",
        family: SignalFamily::Digital,
        modulation: "65-FSK, 2.69 baud",
        bandwidth_hz: (150.0, 400.0),
        bands: HF_VHF,
        frequencies_hz: &[3_576_000.0, 7_076_000.0, 14_076_000.0, 50_276_000.0, 144_489_000.0],
        modes: &[Mode::Jt65],
        summary: "The classic 60-second weak-signal mode: 65 tones, RS(63,12), a two-character report.",
        sigidwiki: Some("JT65"),
    },
    SignalProfile {
        name: "JT9",
        family: SignalFamily::Digital,
        modulation: "9-FSK, 1.74 baud",
        bandwidth_hz: (10.0, 30.0),
        bands: HF,
        frequencies_hz: &[3_578_000.0, 7_078_000.0, 10_140_000.0, 14_078_000.0, 18_104_000.0],
        modes: &[Mode::Jt9],
        summary: "JT65's very narrow sibling: a 16 Hz signal for the weakest HF paths.",
        sigidwiki: Some("JT9"),
    },
    SignalProfile {
        name: "FST4",
        family: SignalFamily::Digital,
        modulation: "160-symbol GFSK",
        bandwidth_hz: (10.0, 100.0),
        bands: HF_VHF,
        frequencies_hz: &[1_840_000.0, 3_570_000.0, 7_060_000.0, 14_080_000.0],
        modes: &[Mode::Fst4],
        summary: "Slow weak-signal mode for EME and LF/MF: T/R periods of 15 s to 300 s.",
        sigidwiki: Some("FST4"),
    },
    SignalProfile {
        name: "Q65",
        family: SignalFamily::Digital,
        modulation: "65-tone FSK with Q-ary LDPC",
        bandwidth_hz: (100.0, 800.0),
        bands: VHF_UHF,
        frequencies_hz: &[50_275_000.0, 144_116_000.0, 432_065_000.0],
        modes: &[Mode::Q65],
        summary: "Modern EME and scatter mode: Doppler-tolerant tones in a chosen sub-mode.",
        sigidwiki: Some("Q65"),
    },
    SignalProfile {
        name: "FSK441",
        family: SignalFamily::Digital,
        modulation: "4-FSK, 441 baud",
        bandwidth_hz: (200.0, 1_500.0),
        bands: &[Band::M6, Band::M2],
        frequencies_hz: &[50_260_000.0, 144_200_000.0],
        modes: &[Mode::Fsk441],
        summary: "The original high-speed meteor scatter mode: 30 s periods of short ionised-trail pings.",
        sigidwiki: Some("FSK441"),
    },
    SignalProfile {
        name: "ADS-B / Mode S",
        family: SignalFamily::Aviation,
        modulation: "PPM, 1 Mbit/s",
        bandwidth_hz: (1_000_000.0, 4_000_000.0),
        bands: &[],
        frequencies_hz: &[1_090_000_000.0],
        modes: &[Mode::Adsb],
        summary: "Aircraft surveillance downlink on 1090 MHz: identity, altitude, velocity, position.",
        sigidwiki: Some("Automatic_Dependent_Surveillance-Broadcast_(ADS-B)"),
    },
    SignalProfile {
        name: "VDL Mode 2",
        family: SignalFamily::Aviation,
        modulation: "D8PSK, 10 500 baud",
        bandwidth_hz: (12_000.0, 20_000.0),
        bands: &[Band::Air],
        frequencies_hz: &[
            136_650_000.0,
            136_675_000.0,
            136_700_000.0,
            136_725_000.0,
            136_750_000.0,
            136_775_000.0,
            136_800_000.0,
            136_825_000.0,
            136_850_000.0,
            136_875_000.0,
            136_900_000.0,
            136_925_000.0,
            136_950_000.0,
            136_975_000.0,
        ],
        modes: &[Mode::Vdl2],
        summary: "The VHF airliner datalink: fourteen 25 kHz channels around 136.8 MHz.",
        sigidwiki: Some("VHF_Data_Link_-_Mode_2_(VDL-M2)"),
    },
    SignalProfile {
        name: "ACARS",
        family: SignalFamily::Aviation,
        modulation: "AM, 2400 baud MSK",
        bandwidth_hz: (2_000.0, 5_000.0),
        bands: &[Band::Air],
        frequencies_hz: &[
            131_525_000.0,
            131_725_000.0,
            136_700_000.0,
            136_750_000.0,
            136_800_000.0,
            136_850_000.0,
            136_900_000.0,
        ],
        modes: &[Mode::Acars],
        summary: "Aircraft messages and position reports on the airband, character by character.",
        sigidwiki: Some("Aircraft_Communications_Addressing_and_Reporting_System_(ACARS)"),
    },
    SignalProfile {
        name: "AIS",
        family: SignalFamily::Marine,
        modulation: "GMSK, 9600 bit/s",
        bandwidth_hz: (10_000.0, 20_000.0),
        bands: &[],
        frequencies_hz: &[161_975_000.0, 162_025_000.0],
        modes: &[Mode::Ais],
        summary: "Ship reporting on two 25 kHz channels: identity, position, course, destination.",
        sigidwiki: Some("Automatic_Identification_System_(AIS)"),
    },
    SignalProfile {
        name: "DSC",
        family: SignalFamily::Marine,
        modulation: "FFSK, 1200 baud, 1300/2100 Hz",
        bandwidth_hz: (500.0, 1_500.0),
        bands: &[Band::M2, Band::Sw],
        frequencies_hz: &[
            2_187_500.0,
            4_207_500.0,
            6_312_000.0,
            8_414_500.0,
            12_577_000.0,
            16_804_500.0,
            156_525_000.0,
        ],
        modes: &[Mode::Dsc],
        summary: "Digital Selective Calling: distress alerts and routine calls on the marine MF/HF and VHF channels.",
        sigidwiki: Some("GMDSS_Digital_Selective_Calling"),
    },
    SignalProfile {
        name: "NAVTEX",
        family: SignalFamily::Marine,
        modulation: "SITOR-B, 100 baud, 170 Hz shift",
        bandwidth_hz: (300.0, 600.0),
        bands: &[],
        frequencies_hz: &[518_000.0, 490_000.0, 4_209_500.0],
        modes: &[Mode::Navtex],
        summary: "Maritime safety and weather broadcasts on 518 kHz and its two companions.",
        sigidwiki: Some("SITOR-B"),
    },
    SignalProfile {
        name: "WEFAX",
        family: SignalFamily::Marine,
        modulation: "FM subcarrier, 120/240 lpm",
        bandwidth_hz: (1_500.0, 3_000.0),
        bands: &[Band::Sw],
        frequencies_hz: &[
            2_618_500.0,
            3_855_000.0,
            4_235_000.0,
            4_610_000.0,
            6_340_500.0,
            7_880_000.0,
            8_040_000.0,
            9_110_000.0,
            12_750_000.0,
            13_882_500.0,
        ],
        modes: &[Mode::Wefax],
        summary: "Weather charts by radiofax: a continuous raster on a 1900 Hz subcarrier.",
        sigidwiki: Some("WEFAX"),
    },
    SignalProfile {
        name: "HFDL (ARINC 635)",
        family: SignalFamily::Aviation,
        modulation: "M-PSK, multiple rates",
        bandwidth_hz: (1_500.0, 3_000.0),
        bands: &[Band::Sw],
        frequencies_hz: &[
            2_851_000.0,
            3_401_000.0,
            4_652_000.0,
            5_481_000.0,
            6_532_000.0,
            8_825_000.0,
            10_081_000.0,
            11_384_000.0,
            13_276_000.0,
            17_907_000.0,
            21_931_000.0,
        ],
        modes: &[Mode::Hfdl],
        summary: "The HF aircraft datalink: ground stations across the shortwave band talking to oceanic aircraft.",
        sigidwiki: Some("High_Frequency_Data_Link_(HFDL)"),
    },
    SignalProfile {
        name: "IBP beacons (NCDXF/IARU)",
        family: SignalFamily::Amateur,
        modulation: "CW carrier, 100 W–100 mW",
        bandwidth_hz: (50.0, 500.0),
        bands: HF,
        frequencies_hz: &[14_100_000.0, 18_110_000.0, 21_150_000.0, 24_930_000.0, 28_200_000.0],
        modes: &[Mode::Cw],
        summary: "Eighteen propagation beacons, one per minute per band, stepping power as they send.",
        sigidwiki: None,
    },
    SignalProfile {
        name: "Time signals (WWV, CHU)",
        family: SignalFamily::Time,
        modulation: "AM, 1000 Hz ticks",
        bandwidth_hz: (3_000.0, 8_000.0),
        bands: &[Band::Sw],
        frequencies_hz: &[
            2_500_000.0,
            3_330_000.0,
            5_000_000.0,
            7_850_000.0,
            10_000_000.0,
            14_670_000.0,
            15_000_000.0,
            20_000_000.0,
        ],
        modes: &[Mode::Am, Mode::Usb, Mode::Cw],
        summary: "Standard-frequency and time stations: voice announcements and a once-a-second tick.",
        sigidwiki: Some("CHU"),
    },
    SignalProfile {
        name: "CB (11 m)",
        family: SignalFamily::Other,
        modulation: "AM / FM / SSB",
        bandwidth_hz: (2_000.0, 8_000.0),
        bands: &[Band::M11],
        frequencies_hz: &[27_065_000.0, 27_185_000.0, 27_245_000.0, 27_700_000.0],
        modes: &[Mode::Am, Mode::Nfm, Mode::Usb, Mode::Lsb],
        summary: "The citizens' band: 40 shared channels from 26.965 MHz. Channel 9 is emergency, 19 is calling.",
        sigidwiki: None,
    },
    SignalProfile {
        name: "PMR446",
        family: SignalFamily::Other,
        modulation: "NFM / dPMR",
        bandwidth_hz: (6_000.0, 12_500.0),
        bands: &[],
        frequencies_hz: &[446_006_250.0, 446_031_250.0],
        modes: &[Mode::Nfm],
        summary: "Licence-free handhelds on the 446 MHz band: 16 analogue or 32 digital channels.",
        sigidwiki: None,
    },
    SignalProfile {
        name: "DRM",
        family: SignalFamily::Broadcast,
        modulation: "OFDM, QAM",
        bandwidth_hz: (9_000.0, 12_000.0),
        bands: &[Band::Mw, Band::Sw],
        frequencies_hz: &[],
        modes: &[Mode::Drm],
        summary: "Digital Radio Mondiale: a digital broadcast filling 9 or 10 kHz on MF and HF.",
        sigidwiki: Some("Digital_Radio_Mondiale_(DRM)"),
    },
    SignalProfile {
        name: "HD Radio (NRSC-5)",
        family: SignalFamily::Broadcast,
        modulation: "OFDM sidebands alongside FM",
        bandwidth_hz: (200_000.0, 400_000.0),
        bands: &[Band::Fm, Band::Mw],
        frequencies_hz: &[],
        modes: &[Mode::HdRadio],
        summary: "North American digital broadcast: CD-quality audio and text beside the analogue carrier.",
        sigidwiki: Some("HD_Radio_(FM)"),
    },
    SignalProfile {
        name: "C-QUAM AM stereo",
        family: SignalFamily::Broadcast,
        modulation: "AM with phase-modulated difference",
        bandwidth_hz: (4_000.0, 10_000.0),
        bands: &[Band::Mw],
        frequencies_hz: &[],
        modes: &[Mode::Cquam],
        summary: "Motorola's AM stereo: mono-compatible sum, stereo difference on carrier phase.",
        sigidwiki: None,
    },
    SignalProfile {
        name: "DAB / DAB+",
        family: SignalFamily::Broadcast,
        modulation: "OFDM, QPSK to 64-QAM",
        bandwidth_hz: (1_400_000.0, 1_700_000.0),
        bands: &[],
        frequencies_hz: &[],
        modes: &[],
        summary: "Digital Audio Broadcasting: a 1.536 MHz ensemble of stations in Band III or L-band.",
        sigidwiki: Some("Digital_Audio_Broadcasting_(DAB)"),
    },
    SignalProfile {
        name: "FM broadcast",
        family: SignalFamily::Broadcast,
        modulation: "WFM with 19 kHz stereo pilot and RDS",
        bandwidth_hz: (150_000.0, 250_000.0),
        bands: &[Band::Fm],
        frequencies_hz: &[],
        modes: &[Mode::Wfm],
        summary: "VHF FM radio: wide, quieting, with a stereo pilot and an RDS data subcarrier.",
        sigidwiki: Some("FM_Broadcast_Radio"),
    },
    SignalProfile {
        name: "AM broadcast (medium wave)",
        family: SignalFamily::Broadcast,
        modulation: "AM, 9/10 kHz channels",
        bandwidth_hz: (4_000.0, 10_000.0),
        bands: &[Band::Mw],
        frequencies_hz: &[],
        modes: &[Mode::Am, Mode::Sam],
        summary: "Long-distance medium-wave broadcasting, strongest after dark.",
        sigidwiki: Some("Amplitude_Modulation_(AM)"),
    },
    SignalProfile {
        name: "LW broadcast and NDBs",
        family: SignalFamily::Broadcast,
        modulation: "AM / CW",
        bandwidth_hz: (1_000.0, 6_000.0),
        bands: &[Band::Lw],
        frequencies_hz: &[],
        modes: &[Mode::Am, Mode::Sam, Mode::Cw],
        summary: "Longwave broadcast and aeronautical non-directional beacons in the 150–280 kHz range.",
        sigidwiki: None,
    },
    SignalProfile {
        name: "Shortwave broadcast",
        family: SignalFamily::Broadcast,
        modulation: "AM",
        bandwidth_hz: (4_000.0, 10_000.0),
        bands: SW,
        frequencies_hz: &[],
        modes: &[Mode::Am, Mode::Sam],
        summary: "International AM broadcasting between 2.3 and 26.1 MHz, fading with the ionosphere.",
        sigidwiki: None,
    },
    SignalProfile {
        name: "Airband voice and VOLMET",
        family: SignalFamily::Aviation,
        modulation: "AM, 8.33/25 kHz",
        bandwidth_hz: (5_000.0, 9_000.0),
        bands: &[Band::Air],
        frequencies_hz: &[121_500_000.0],
        modes: &[Mode::Am],
        summary: "Civil aviation voice: tower, approach and ground, plus continuous VOLMET weather.",
        sigidwiki: None,
    },
    SignalProfile {
        name: "VOR and ILS navigation",
        family: SignalFamily::Aviation,
        modulation: "AM, navigation subcarriers",
        bandwidth_hz: (10_000.0, 30_000.0),
        bands: &[Band::Air],
        frequencies_hz: &[],
        modes: &[Mode::Am],
        summary: "Airport navigation aids in 108–118 MHz: VOR bearing and ILS localiser/glideslope.",
        sigidwiki: Some("VHF_Omnidirectional_Range_(VOR)"),
    },
    SignalProfile {
        name: "Military airband",
        family: SignalFamily::Aviation,
        modulation: "AM",
        bandwidth_hz: (5_000.0, 9_000.0),
        bands: &[Band::Mil],
        frequencies_hz: &[243_000_000.0],
        modes: &[Mode::Am],
        summary: "NATO UHF air traffic control and tactical voice in 225–400 MHz.",
        sigidwiki: None,
    },
    SignalProfile {
        name: "NFM voice and repeaters",
        family: SignalFamily::Amateur,
        modulation: "FM, 12.5/25 kHz",
        bandwidth_hz: (6_000.0, 16_000.0),
        bands: &[Band::M10, Band::M6, Band::M4, Band::M2, Band::M125, Band::M70],
        frequencies_hz: &[145_500_000.0, 146_520_000.0, 433_500_000.0, 446_000_000.0],
        modes: &[Mode::Nfm],
        summary: "Narrow FM voice on the VHF and UHF bands: simplex calling and repeater channels.",
        sigidwiki: Some("NFM_Voice"),
    },
    SignalProfile {
        name: "Digital voice (DMR, D-Star, C4FM, NXDN)",
        family: SignalFamily::Digital,
        modulation: "4FSK/C4FM, 4800–9600 baud",
        bandwidth_hz: (6_000.0, 12_500.0),
        bands: &[Band::M2, Band::M70],
        frequencies_hz: &[145_237_500.0, 438_800_000.0, 439_012_500.0],
        modes: &[Mode::Nfm],
        summary: "Amateur digital voice on FM channels: a buzzing packet stream rather than speech on the carrier.",
        sigidwiki: Some("Digital_Mobile_Radio_(DMR)"),
    },
    SignalProfile {
        name: "SSB voice",
        family: SignalFamily::Amateur,
        modulation: "SSB, suppressed carrier",
        bandwidth_hz: (1_800.0, 3_000.0),
        bands: &[
            Band::M160,
            Band::M80,
            Band::M40,
            Band::M20,
            Band::M17,
            Band::M15,
            Band::M12,
            Band::M10,
            Band::M6,
        ],
        frequencies_hz: &[],
        modes: &[Mode::Usb, Mode::Lsb, Mode::Digu, Mode::Digl, Mode::Dsb],
        summary: "Single-sideband voice: the ordinary HF phone mode, a clean band of speech on the waterfall.",
        sigidwiki: Some("Single_Sideband_Voice"),
    },
    SignalProfile {
        name: "AM voice (HF)",
        family: SignalFamily::Amateur,
        modulation: "AM",
        bandwidth_hz: (4_000.0, 9_000.0),
        bands: &[Band::M160, Band::M80, Band::M40, Band::M20, Band::M17, Band::M15, Band::M10],
        frequencies_hz: &[29_000_000.0],
        modes: &[Mode::Am, Mode::Sam],
        summary: "Amplitude-modulated voice on the amateur bands, with a carrier and both sidebands.",
        sigidwiki: Some("Amplitude_Modulation_(AM)"),
    },
    SignalProfile {
        name: "CW (Morse)",
        family: SignalFamily::Amateur,
        modulation: "On-off keyed carrier",
        bandwidth_hz: (50.0, 500.0),
        bands: HF_VHF,
        frequencies_hz: &[],
        modes: &[Mode::Cw],
        summary: "Morse on a keyed carrier: dots and dashes, readable far below the noise.",
        sigidwiki: Some("Morse_Code_(CW)"),
    },
    SignalProfile {
        name: "ISB (independent sideband)",
        family: SignalFamily::Utility,
        modulation: "Two sidebands, independent content",
        bandwidth_hz: (2_000.0, 6_000.0),
        bands: SW,
        frequencies_hz: &[],
        modes: &[Mode::Isb],
        summary: "One carrier, two different signals: a language on one sideband and data on the other.",
        sigidwiki: None,
    },
    SignalProfile {
        name: "QO-100 satellite",
        family: SignalFamily::Satellite,
        modulation: "SSB/CW/data, narrowband transponder",
        bandwidth_hz: (1_000.0, 3_000.0),
        bands: &[],
        frequencies_hz: &[10_489_750_000.0],
        modes: &[Mode::Usb, Mode::Lsb, Mode::Cw],
        summary: "The geostationary amateur satellite: a 10.489 GHz downlink over Europe, Africa and the Middle East.",
        sigidwiki: Some("QO-100-modem"),
    },
    SignalProfile {
        name: "ISM band devices",
        family: SignalFamily::Utility,
        modulation: "OOK/FSK bursts",
        bandwidth_hz: (10_000.0, 100_000.0),
        bands: &[],
        frequencies_hz: &[315_000_000.0, 433_920_000.0, 868_000_000.0, 915_000_000.0],
        modes: &[],
        summary: "Short bursts from weather stations, remotes, meters and alarms on the licence-free ISM bands.",
        sigidwiki: Some("ISM_Band_device"),
    },
];

fn score(p: &SignalProfile, freq_hz: f64, band: Option<Band>, bw_hz: Option<f64>) -> i32 {
    let mut s = 0;

    let mut freq_best = 0;
    for &f in p.frequencies_hz {
        let d = (f - freq_hz).abs();
        if d <= 500.0 {
            freq_best = freq_best.max(50);
        } else if d <= 5_000.0 {
            freq_best = freq_best.max(20);
        }
    }
    s += freq_best;

    if let Some(b) = band
        && p.bands.contains(&b)
    {
        s += 15;
    }

    if let Some(bw) = bw_hz
        && bw > 0.0
        && p.bandwidth_hz.1 > 0.0
    {
        let (lo, hi) = p.bandwidth_hz;
        let lo = lo.max(1.0);
        s += if bw >= lo && bw <= hi {
            10
        } else if bw < lo {
            (10.0 * bw / lo).round() as i32
        } else {
            (10.0 * hi / bw).round() as i32
        };
    }

    if !p.frequencies_hz.is_empty() {
        s += 5;
    }

    s
}

pub fn identify(
    freq_hz: f64,
    band: Option<Band>,
    mode: Mode,
    bw_hz: Option<f64>,
) -> Vec<&'static SignalProfile> {
    let mut scored: Vec<(bool, i32, &'static SignalProfile)> = PROFILES
        .iter()
        .map(|p| (p.modes.contains(&mode), score(p, freq_hz, band, bw_hz), p))
        .filter(|(mode_match, s, _)| *mode_match || *s > 0)
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| b.1.cmp(&a.1)));
    scored.into_iter().map(|(_, _, p)| p).collect()
}

pub fn search_profiles(query: &str) -> Vec<&'static SignalProfile> {
    PROFILES.iter().filter(|p| p.matches(query)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn top(freq_hz: f64, band: Option<Band>, mode: Mode, bw_hz: Option<f64>) -> &'static str {
        identify(freq_hz, band, mode, bw_hz).first().map(|p| p.name).unwrap_or("")
    }

    #[test]
    fn the_known_channels_identify() {
        assert_eq!(top(14_074_000.0, Some(Band::M20), Mode::Ft8, Some(2_500.0)), "FT8");
        assert_eq!(top(518_000.0, None, Mode::Navtex, Some(500.0)), "NAVTEX");
        assert_eq!(top(27_185_000.0, Some(Band::M11), Mode::Am, Some(6_000.0)), "CB (11 m)");
        assert_eq!(top(161_975_000.0, None, Mode::Ais, None), "AIS");
    }

    #[test]
    fn a_mode_match_beats_a_bare_band_hit() {
        let hits = identify(7_074_000.0, Some(Band::M40), Mode::Ft8, Some(2_500.0));
        assert_eq!(hits.first().map(|p| p.name), Some("FT8"));
        let hits = identify(7_074_000.0, Some(Band::M40), Mode::Lsb, Some(2_400.0));
        assert_eq!(hits.first().map(|p| p.name), Some("SSB voice"));
        assert_ne!(hits.first().map(|p| p.name), Some("FT8"));
    }

    #[test]
    fn the_search_box_finds_a_profile_by_name_and_by_what_it_says() {
        assert!(search_profiles("ft8").iter().any(|p| p.name == "FT8"));
        assert!(search_profiles("meteor").iter().any(|p| p.family == SignalFamily::Digital));
        assert!(search_profiles("zzzz").is_empty());
    }

    #[test]
    fn every_profile_is_well_formed_and_has_a_summary() {
        for p in PROFILES {
            assert!(!p.name.is_empty(), "a profile has no name");
            assert!(!p.modulation.is_empty(), "{} has no modulation", p.name);
            assert!(!p.summary.is_empty(), "{} has no summary", p.name);
            let (lo, hi) = p.bandwidth_hz;
            assert!(hi > 0.0, "{} has no bandwidth", p.name);
            assert!(lo <= hi, "{} has inverted bandwidth", p.name);
            if let Some(slug) = p.sigidwiki {
                assert!(!slug.is_empty(), "{} has an empty sigidwiki slug", p.name);
            }
        }
    }

    #[test]
    fn names_are_unique() {
        let mut seen: Vec<&str> = Vec::new();
        for p in PROFILES {
            assert!(!seen.contains(&p.name), "{} appears twice", p.name);
            seen.push(p.name);
        }
    }
}
