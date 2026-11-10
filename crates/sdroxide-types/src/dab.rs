//! DAB / DAB+ — Digital Audio Broadcasting.
//!
//! The OFDM digital radio band, in **Band III** (174–240 MHz) and L-band. A
//! **wideband** service like ADS-B: a Mode I ensemble is about 1.536 MHz of
//! occupied spectrum, so it is decoded from its own lane of raw I/Q rather than
//! the 12 kHz tap the narrow modes share, and the panel is a service list
//! rather than a waterfall of audio.
//!
//! This module holds only the wire/persisted shape — the band table, the
//! settings and the status the panel draws. The decoder itself is the
//! `sdroxide-dab` crate, which wraps the MIT `dabradio` library and decodes
//! DAB+ audio through the stock faad2 the binary already carries.

use serde::{Deserialize, Serialize};

/// The Band III channels, by name and centre frequency, in the order the band
/// plan lists them — the plan an operator tunes by, not an arithmetic sequence
/// (the block centres are not evenly spaced).
pub const DAB_BAND_III: &[(&str, f64)] = &[
    ("5A", 174_928_000.0),
    ("5B", 176_640_000.0),
    ("5C", 178_352_000.0),
    ("5D", 180_064_000.0),
    ("6A", 181_936_000.0),
    ("6B", 183_648_000.0),
    ("6C", 185_360_000.0),
    ("6D", 187_072_000.0),
    ("7A", 188_928_000.0),
    ("7B", 190_640_000.0),
    ("7C", 192_352_000.0),
    ("7D", 194_064_000.0),
    ("8A", 195_936_000.0),
    ("8B", 197_648_000.0),
    ("8C", 199_360_000.0),
    ("8D", 201_072_000.0),
    ("9A", 202_928_000.0),
    ("9B", 204_640_000.0),
    ("9C", 206_352_000.0),
    ("9D", 208_064_000.0),
    ("10A", 209_936_000.0),
    ("10B", 211_648_000.0),
    ("10C", 213_360_000.0),
    ("10D", 215_072_000.0),
    ("11A", 216_928_000.0),
    ("11B", 218_640_000.0),
    ("11C", 220_352_000.0),
    ("11D", 222_064_000.0),
    ("12A", 223_936_000.0),
    ("12B", 225_648_000.0),
    ("12C", 227_360_000.0),
    ("12D", 229_072_000.0),
    ("13A", 230_784_000.0),
    ("13B", 232_496_000.0),
    ("13C", 234_208_000.0),
    ("13D", 235_776_000.0),
    ("13E", 237_488_000.0),
    ("13F", 239_200_000.0),
];

/// The rate the DAB Mode I OFDM front end is defined at, in samples/s.
///
/// Kept here, not in the decoder, because the *window planning* that decides
/// whether a front end can serve DAB is arithmetic the UI runs too — see
/// `sdroxide_dsp::dab_window_rate` — and `sdroxide-dab` is a native-only
/// decoder the browser client cannot depend on. `sdroxide-dab` re-exports
/// these, so the engine, the decoder and the UI all read one value.
pub const DAB_SAMPLE_RATE: u32 = 2_048_000;

/// How wide a lane a DAB ensemble occupies, in Hz — Mode I is ~1.536 MHz of
/// occupied spectrum, so a front end opened narrower loses the outer carriers.
pub const DAB_BANDWIDTH_HZ: f64 = 1_536_000.0;

/// The reference rate for a DAB lane: **2.5 Msps**.
///
/// At the bare [`DAB_BANDWIDTH_HZ`] the front end is right at its ADC floor,
/// where the SDRplay API in particular drops samples — and a DAB decode needs a
/// *continuous* OFDM stream, so a splice stops sync. 2.5 Msps is the rate DAB+
/// was confirmed decoding cleanly at, end to end, on a PlutoSDR (Kevin,
/// 2.0.7.3, 2026-10-11: no dropouts). It replaced the earlier 3.072 Msps
/// "twice the occupied width", which was a guess made before anybody had
/// decoded on the air and which no front end's ladder could reach. Not a
/// refusal below this: a narrower-but-clean stream still decodes.
pub const DAB_GOOD_RATE_HZ: f64 = 2_500_000.0;

/// The channel whose centre is nearest `hz`, as `(name, centre)`.
pub fn dab_channel_at(hz: f64) -> Option<(&'static str, f64)> {
    DAB_BAND_III
        .iter()
        .min_by(|a, b| {
            (a.1 - hz).abs().partial_cmp(&(b.1 - hz).abs()).unwrap_or(std::cmp::Ordering::Equal)
        })
        .copied()
}

/// One service the ensemble carries.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DabService {
    /// The service identifier as the FIC carries it (`0xF201`).
    pub service_id: String,
    /// The service label (programme name).
    pub label: String,
    /// The sub-channel the service's data rides in, when known.
    #[serde(default)]
    pub subchannel: Option<u8>,
    /// Bitrate in kbps.
    #[serde(default)]
    pub bitrate: Option<u16>,
    /// The protection profile ("EEP 3-A" and so on).
    #[serde(default)]
    pub protection: Option<String>,
    /// The programme type the FIC names for it ("News", "Pop Music"), when it
    /// does. Appended last.
    #[serde(default)]
    pub pty: Option<String>,
}

/// How the DAB receiver behaves. Owned by the engine (it lives in
/// [`crate::RadioState`]) and edited from the panel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct DabSettings {
    /// The decoder runs. Follows the mode — selecting DAB switches it on — but
    /// kept as a field so a front end that cannot feed it can switch it off and
    /// say so.
    pub enabled: bool,
    /// The channel to tune, by name from [`DAB_BAND_III`] ("8B"). The
    /// ensemble's own centre; the dial follows it, as ADS-B's 1090 MHz does.
    pub channel: String,
    /// The service currently playing, by service id. Empty until one is picked.
    pub service_id: String,
    /// Volume for DAB audio, on the same 0..=1 scale the receiver's own uses.
    pub volume: f32,
    /// Channels a scan has found an ensemble on, remembered across restarts.
    ///
    /// Which blocks carry a transmission is a fact about where the operator
    /// is, not about the band: the same 12C is a multiplex in one country and
    /// silence in the next. A scan fills this, and the panel offers it, so the
    /// operator picks from what is actually on the air rather than a
    /// hardcoded list that only fits one country.
    #[serde(default)]
    pub found: Vec<String>,
    /// The stations each found channel carries, remembered with the channel,
    /// so a channel picked from the list shows its stations at once rather
    /// than after the FIC has been read again — and so they survive a restart.
    ///
    /// Appended last: `DabSettings` rides the wire whole inside `RadioState`.
    #[serde(default)]
    pub stations: Vec<DabKnownStation>,
    /// The operator's presets: stations starred to be one press away, in the
    /// order they were starred. Appended last.
    #[serde(default)]
    pub presets: Vec<DabKnownStation>,
}

/// A station the receiver has heard named, and the channel it rides on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DabKnownStation {
    /// The block it was heard on ("8B").
    pub channel: String,
    /// The service id, which is what a pick sends.
    pub service_id: String,
    /// The programme name as the FIC gave it; may be empty.
    pub label: String,
}

impl DabSettings {
    /// The remembered stations on `channel`, in the order they were heard.
    pub fn stations_on<'a>(
        &'a self,
        channel: &'a str,
    ) -> impl Iterator<Item = &'a DabKnownStation> + 'a {
        self.stations.iter().filter(move |s| s.channel == channel)
    }

    /// Fold what the ensemble on `channel` names into the memory. Returns
    /// whether anything changed, so a caller sends the settings only then.
    ///
    /// A station already known keeps its place and takes a newer label (the
    /// FIC often names a service a moment after it lists it); a new one is
    /// added after the others on that channel.
    pub fn remember_stations(&mut self, channel: &str, services: &[DabService]) -> bool {
        if channel.is_empty() {
            return false;
        }
        let mut changed = false;
        for svc in services {
            match self
                .stations
                .iter_mut()
                .find(|k| k.channel == channel && k.service_id == svc.service_id)
            {
                Some(k) => {
                    if !svc.label.trim().is_empty() && k.label != svc.label {
                        k.label = svc.label.clone();
                        changed = true;
                    }
                }
                None => {
                    self.stations.push(DabKnownStation {
                        channel: channel.to_string(),
                        service_id: svc.service_id.clone(),
                        label: svc.label.clone(),
                    });
                    changed = true;
                }
            }
        }
        changed
    }

    /// Forget a whole channel: drop it from the found list and every station
    /// remembered on it. A SCAN finds it again.
    pub fn forget_channel(&mut self, channel: &str) {
        self.found.retain(|c| c != channel);
        self.stations.retain(|s| s.channel != channel);
    }

    /// Take a sweep's answer: it walked [`DAB_BAND_III`] up to block `at`
    /// (the whole band when `at` is past its end) and decoded stations on the
    /// blocks in `found`.
    ///
    /// Only blocks that named stations are kept — a block with nothing on it
    /// is not a channel to offer. A block the sweep never reached (it was
    /// stopped early) keeps what was known of it, so stopping is never a loss;
    /// a block it did walk and found silent is dropped with its stations. The
    /// presets are the operator's and are left alone. Band order.
    pub fn apply_scan(&mut self, at: usize, found: &[String]) {
        let old = std::mem::take(&mut self.found);
        self.found = DAB_BAND_III
            .iter()
            .enumerate()
            .filter(|(i, (name, _))| {
                found.iter().any(|f| f == name) || (*i >= at && old.iter().any(|o| o == name))
            })
            .map(|(_, (name, _))| name.to_string())
            .collect();
        let kept = &self.found;
        self.stations.retain(|s| kept.contains(&s.channel));
    }

    /// Forget one station on one channel. It comes back the next time the
    /// ensemble is read — a SCAN, or tuning the channel again.
    pub fn forget_station(&mut self, channel: &str, service_id: &str) {
        self.stations.retain(|s| !(s.channel == channel && s.service_id == service_id));
    }
}

impl Default for DabSettings {
    fn default() -> Self {
        DabSettings {
            enabled: true,
            channel: "12B".to_string(),
            service_id: String::new(),
            volume: 1.0,
            found: Vec::new(),
            stations: Vec::new(),
            presets: Vec::new(),
        }
    }
}

impl DabSettings {
    /// Nothing running: the decoder is off and the panel says so.
    pub const OFF: DabSettings = DabSettings {
        enabled: false,
        channel: String::new(),
        service_id: String::new(),
        volume: 1.0,
        found: Vec::new(),
        stations: Vec::new(),
        presets: Vec::new(),
    };
}

/// What the DAB panel draws.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DabStatus {
    /// The ensemble's label once the FIC has named it ("Métropolitain 2").
    pub ensemble: Option<String>,
    /// The services the ensemble carries.
    pub services: Vec<DabService>,
    /// Why nothing is running, when nothing is running — a receiver that cannot
    /// reach Band III, or cannot deliver enough bandwidth, produces an empty
    /// panel either way; only this distinguishes that from a quiet channel.
    pub unavailable: Option<String>,
    /// Where the decoder's own window is, and how wide, in Hz — the same
    /// "your receiver is not looking there" the ADS-B window reports.
    pub window_center_hz: f64,
    pub window_rate_hz: f64,
    /// OFDM frames the decoder has seen, and how many FIBs it read out of them.
    /// A frame count that climbs with no services is a channel with something
    /// else on it, worth showing rather than leaving the panel looking broken.
    pub frames: u64,
    pub fibs: u64,
    /// Whether audio is being decoded, and for which service.
    pub playing: Option<String>,
    /// A receiver too narrow to hold an ensemble decodes nothing and says so.
    #[serde(default)]
    pub degraded: Option<String>,
    /// The decoded audio's level, left and right, in dBFS RMS over the block
    /// last sent to the speakers — measured before the volume control, so the
    /// meter shows the programme and not the knob. `None` while nothing plays.
    ///
    /// Appended last: `DabStatus` rides the wire whole.
    #[serde(default)]
    pub audio_rms_db: Option<[f32; 2]>,
    /// The playing service's Dynamic Label (DLS) — the line of text a station
    /// sends with its audio: the song, the programme. `None` until it sends
    /// one. Appended last.
    #[serde(default)]
    pub dls: Option<String>,
}

/// A picture the playing DAB station broadcast in its slideshow (MOT
/// SlideShow), as the station sent it: a JPEG or PNG at whatever size and
/// shape it chose — 320x240 is the standard's, square is common.
///
/// Its own message rather than a field on [`DabStatus`]: the status goes out
/// several times a second and a picture is tens of kilobytes, sent once when a
/// new one arrives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DabSlide {
    /// The service that sent it, so a picture that arrives just after a
    /// change of station is not shown on the next one.
    pub service_id: String,
    /// `image/jpeg` or `image/png`.
    pub content_type: String,
    /// The name the station gave the object, when it gave one.
    pub name: Option<String>,
    pub data: Vec<u8>,
}

/// The rate a digital down-converter's decimation ladder settles on, without
/// building one — the pure arithmetic behind `sdroxide_dsp::Ddc::rate_for`,
/// which calls this so the two are one.
///
/// It lives here, in a wasm-safe crate, because deciding whether a front end
/// can serve DAB is arithmetic the **browser client** runs, and the DSP crate
/// is native-only. `sdroxide-dsp` depends on this crate, so `Ddc` uses it too.
pub fn ddc_rate_for(in_rate: f64, target_rate: f64) -> f64 {
    let mut rate = in_rate;
    while rate > target_rate * 8.0 {
        rate /= 2.0;
    }
    rate / (rate / target_rate).round().max(1.0)
}

/// The rate a DAB lane's ladder actually lands on, for a receiver running at
/// `device_rate`.
///
/// The window target is capped at [`DAB_SAMPLE_RATE`] — the width an ensemble
/// needs — so what decides the lane is the ladder, not the device rate: a front
/// end can be opened wider than the target and still hand the lane no more than
/// [`ddc_rate_for`] picks.
pub fn dab_window_rate(device_rate: f64) -> f64 {
    let target = f64::from(DAB_SAMPLE_RATE).min(device_rate);
    ddc_rate_for(device_rate, target)
}

/// Would raising the receiver's rate give the DAB lane the margin it wants?
///
/// Asks what the lane would land on at each trial rate rather than assuming:
/// [`ddc_rate_for`] picks the rung *nearest* the target, so a wider rate does
/// not always widen the lane (8 Msps lands it on 2.0). 2.5 and 5 Msps reach
/// the [`DAB_GOOD_RATE_HZ`] reference; a receiver already there gets `false`.
pub fn dab_widening_helps(device_rate: f64) -> bool {
    const TRIALS: &[f64] = &[
        2_500_000.0,
        4_000_000.0,
        5_000_000.0,
        6_000_000.0,
        8_000_000.0,
        10_000_000.0,
        16_000_000.0,
    ];
    dab_window_rate(device_rate) < DAB_GOOD_RATE_HZ
        && TRIALS.iter().any(|&c| dab_window_rate(c) >= DAB_GOOD_RATE_HZ)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn known(channel: &str, id: &str) -> DabKnownStation {
        DabKnownStation { channel: channel.into(), service_id: id.into(), label: id.into() }
    }

    fn idx(name: &str) -> usize {
        DAB_BAND_III.iter().position(|(n, _)| *n == name).unwrap()
    }

    /// A whole sweep keeps exactly the blocks that named stations, in band
    /// order, and drops a block it walked and found silent with its stations.
    /// The presets are not the sweep's to touch.
    #[test]
    fn a_full_sweep_keeps_only_the_blocks_that_named_stations() {
        let mut d = DabSettings {
            found: vec!["12C".into(), "5A".into()],
            stations: vec![known("5A", "A"), known("12C", "C"), known("8B", "B")],
            presets: vec![known("5A", "A")],
            ..DabSettings::default()
        };
        d.apply_scan(DAB_BAND_III.len(), &["12C".into(), "8B".into()]);
        assert_eq!(d.found, vec!["8B".to_string(), "12C".to_string()]);
        assert_eq!(d.stations, vec![known("12C", "C"), known("8B", "B")]);
        assert_eq!(d.presets, vec![known("5A", "A")], "a sweep cleared a preset");
    }

    /// Stopping a sweep half way loses nothing it never looked at.
    #[test]
    fn a_stopped_sweep_keeps_what_it_never_reached() {
        let mut d = DabSettings {
            found: vec!["5A".into(), "12C".into()],
            stations: vec![known("5A", "A"), known("12C", "C")],
            ..DabSettings::default()
        };
        d.apply_scan(idx("8B"), &["7D".into()]);
        assert_eq!(d.found, vec!["7D".to_string(), "12C".to_string()]);
        assert_eq!(d.stations, vec![known("12C", "C")]);
    }

    /// 2.5 Msps is the reference (confirmed on a Pluto), and it is reachable:
    /// a 2.5 or 5 Msps receiver lands the lane on it, so nothing is advised.
    /// A receiver opened narrower is told widening helps, because it does.
    #[test]
    fn two_and_a_half_msps_is_the_dab_reference() {
        assert_eq!(DAB_GOOD_RATE_HZ, 2_500_000.0);
        for rate in [2_500_000.0, 5_000_000.0] {
            assert!(
                dab_window_rate(rate) >= DAB_GOOD_RATE_HZ,
                "{rate:.0} sps: below the reference"
            );
        }
        for rate in [1_000_000.0, 2_000_000.0, 2_048_000.0] {
            assert!(dab_window_rate(rate) < DAB_GOOD_RATE_HZ, "{rate:.0} sps");
            assert!(dab_widening_helps(rate), "{rate:.0} sps: 2.5 Msps would widen it");
        }
    }

    #[test]
    fn the_channel_table_covers_band_iii() {
        assert_eq!(DAB_BAND_III.first().unwrap().0, "5A");
        assert_eq!(DAB_BAND_III.last().unwrap().0, "13F");
        // Ordered ascending by centre, as the plan reads.
        assert!(DAB_BAND_III.windows(2).all(|w| w[0].1 < w[1].1));
    }

    #[test]
    fn a_frequency_resolves_to_the_nearest_channel() {
        assert_eq!(dab_channel_at(197_648_000.0).map(|c| c.0), Some("8B"));
        assert_eq!(dab_channel_at(227_400_000.0).map(|c| c.0), Some("12C"));
        // A few tens of kHz off still lands on the channel.
        assert_eq!(dab_channel_at(197_700_000.0).map(|c| c.0), Some("8B"));
    }
}
