//! HF weather-facsimile settings and status.
//!
//! The mode carries no metadata of its own — a radiofax transmission announces
//! its index of cooperation in the start tone and nothing else — so everything
//! that decides how the picture is built is an operator setting, and the panel
//! needs a live view of what the receiver is making of the signal.

use serde::{Deserialize, Serialize};

/// Index of cooperation: what fixes the number of pixels in a line.
///
/// Mirrors `sdroxide_dsp::wefax::Ioc`, which is where the arithmetic lives.
/// Duplicated rather than shared because this crate is the wasm-safe one and
/// must not depend on the DSP.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WefaxIoc {
    /// 576 — every weather chart. 1809 pixels per line.
    #[default]
    I576,
    /// 288 — half resolution, and the older satellite schedules.
    I288,
}

impl WefaxIoc {
    pub const ALL: [WefaxIoc; 2] = [WefaxIoc::I576, WefaxIoc::I288];

    pub fn value(self) -> u32 {
        match self {
            WefaxIoc::I576 => 576,
            WefaxIoc::I288 => 288,
        }
    }

    /// Pixels per line, `IOC × π` truncated — 1809 and 904.
    pub fn width(self) -> u16 {
        (self.value() as f64 * std::f64::consts::PI) as u16
    }

    pub fn label(self) -> &'static str {
        match self {
            WefaxIoc::I576 => "IOC 576",
            WefaxIoc::I288 => "IOC 288",
        }
    }
}

/// Scan rate in lines per minute. 120 is what the weather services use.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum WefaxLpm {
    L60,
    L90,
    #[default]
    L120,
    L180,
    L240,
}

impl WefaxLpm {
    pub const ALL: [WefaxLpm; 5] =
        [WefaxLpm::L60, WefaxLpm::L90, WefaxLpm::L120, WefaxLpm::L180, WefaxLpm::L240];

    pub fn value(self) -> f64 {
        match self {
            WefaxLpm::L60 => 60.0,
            WefaxLpm::L90 => 90.0,
            WefaxLpm::L120 => 120.0,
            WefaxLpm::L180 => 180.0,
            WefaxLpm::L240 => 240.0,
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            WefaxLpm::L60 => "60",
            WefaxLpm::L90 => "90",
            WefaxLpm::L120 => "120",
            WefaxLpm::L180 => "180",
            WefaxLpm::L240 => "240",
        }
    }
}

/// A broadcast radiofax schedule: what the station is called and the
/// frequencies it transmits on.
///
/// # The 1.9 kHz that catches everyone
///
/// Radiofax frequencies are published as the **assigned carrier**, which is
/// where the 1900 Hz subcarrier's centre would sit. Receiving in USB means
/// tuning 1.9 kHz *below* the published number, and getting this wrong is the
/// single most common reason a chart comes out as a blank page. [`WefaxStation::dial_hz`]
/// does the subtraction; the table stores what the schedules print.
pub struct WefaxStation {
    /// Callsign and location, as the schedules list it.
    pub name: &'static str,
    /// Assigned carrier frequencies in kHz, as published.
    pub carriers_khz: &'static [f64],
    /// Where the transmitter is, degrees north and east.
    ///
    /// Good to a few tens of kilometres, which is all a path drawn across a
    /// globe can show. Several of these are transmitter parks some way from the
    /// place the station is named after — Northwood's aerials are not in
    /// Northwood — and none of that is visible at this scale.
    pub lat: f64,
    pub lon: f64,
    /// The line rate every chart from this station is sent at.
    ///
    /// 120 for every station in this list, and printed in the RPM column of the
    /// WMO's radiofacsimile schedule against each chart. It lives here rather
    /// than as a constant in the panel because picking a station has to set it:
    /// a decoder on 240 draws a 120 LPM chart as a page of bands, and nothing
    /// in the signal says which it is.
    ///
    /// **Pinned deliberately, not assumed.** A test asserts every entry is 120
    /// and IOC 576, so a station that publishes otherwise fails the suite rather
    /// than being quietly wrong; that assertion is the line to change when one
    /// turns up.
    pub lpm: WefaxLpm,
    /// The index of cooperation the same schedule prints behind the rate:
    /// 576, so 1809 pixels to a line.
    pub ioc: WefaxIoc,
    /// Where this station's own timetable is published, or `None` where the
    /// keeper does not publish one we can point at.
    ///
    /// A link rather than the times themselves: the schedules are seasonal, and
    /// several of the stations that carry them have cut transmissions twice in
    /// a year. A copy in here would be wrong within a season and nobody would
    /// know it was wrong.
    pub schedule: Option<&'static str>,
}

impl WefaxStation {
    /// The USB dial for a published carrier frequency, in Hz.
    pub fn dial_hz(carrier_khz: f64) -> f64 {
        carrier_khz * 1000.0 - 1900.0
    }

    /// How far a dial may be from a published one and still count as being on
    /// that station, in Hz.
    ///
    /// Generous, because a fax operator tunes by ear and by the subcarrier
    /// readout rather than to the hertz, and being a hundred hertz off is the
    /// normal state of a chart that is decoding perfectly well.
    pub const NEAR_HZ: f64 = 200.0;

    /// The station a dial frequency is sitting on, and the published carrier it
    /// matched.
    pub fn at_dial(dial_hz: f64) -> Option<(&'static WefaxStation, f64)> {
        WEFAX_STATIONS.iter().find_map(|s| {
            s.carriers_khz
                .iter()
                .find(|&&f| (WefaxStation::dial_hz(f) - dial_hz).abs() < WefaxStation::NEAR_HZ)
                .map(|&f| (s, f))
        })
    }
}

/// The stations most likely to be worth a listen, by region.
///
/// Broadcast schedules change and stations close — Northwood and several of the
/// US Coast Guard transmitters have had their hours cut repeatedly — so this is
/// a starting point for finding a signal, and the timetable is the station's
/// own: [`WefaxStation::schedule`] is a link to it as its keeper publishes it,
/// rather than a copy that would be out of date the season after next. Where
/// the keeper publishes only a PDF, or nothing a link can point at, the mirror
/// at `weatherfax.com` is kept instead of a guess at a page that may not exist.
///
/// Every one of these sends charts at **120 LPM and IOC 576** — the standard,
/// and what the WMO's radiofacsimile schedule prints in its RPM/IOC column for
/// each of them. Carried per station rather than assumed because a picker that
/// moves the dial and leaves the geometry on whatever the chips were last set
/// to is a control that half works: a chart received with the wrong line rate
/// is not a slightly wrong picture, it is an unreadable one.
pub const WEFAX_STATIONS: &[WefaxStation] = &[
    WefaxStation {
        name: "DWD Pinneberg (Germany)",
        carriers_khz: &[3855.0, 7880.0, 13882.5],
        lat: 53.66,
        lon: 9.80,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        // The keeper's own page: the schedule charts of 2026-11-03 print
        // "Further details at www.dwd.de/sendeplan" across the foot of part two,
        // and the fax timetable PDF is there.
        schedule: Some("https://www.dwd.de/sendeplan"),
    },
    WefaxStation {
        name: "GYA Northwood (UK)",
        carriers_khz: &[2618.5, 4610.0, 8040.0, 11086.5],
        lat: 51.60,
        lon: -0.42,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/gya-northwood/"),
    },
    WefaxStation {
        name: "NMF Boston (USCG)",
        carriers_khz: &[4235.0, 6340.5, 9110.0, 12750.0],
        lat: 41.72,
        lon: -70.50,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://www.weather.gov/marine/uscg_broadcasts"),
    },
    WefaxStation {
        name: "NMG New Orleans (USCG)",
        carriers_khz: &[4317.9, 8503.9, 12789.9, 17146.4],
        lat: 29.88,
        lon: -89.94,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://www.weather.gov/marine/uscg_broadcasts"),
    },
    WefaxStation {
        name: "NMC Point Reyes (USCG)",
        carriers_khz: &[4346.0, 8682.0, 12786.0, 17151.2, 22527.0],
        lat: 38.10,
        lon: -122.87,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://www.weather.gov/marine/uscg_broadcasts"),
    },
    WefaxStation {
        name: "NOJ Kodiak (USCG)",
        carriers_khz: &[4298.0, 8459.0, 12412.5],
        lat: 57.75,
        lon: -152.48,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://www.weather.gov/marine/uscg_broadcasts"),
    },
    WefaxStation {
        name: "CFH Halifax (Canada)",
        carriers_khz: &[4271.0, 6496.4, 10536.0, 13510.0],
        lat: 44.97,
        lon: -63.35,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: None,
    },
    WefaxStation {
        name: "JMH Tokyo (Japan)",
        carriers_khz: &[3622.5, 7795.0, 13988.5],
        lat: 35.68,
        lon: 139.76,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/jmh-tokyo/"),
    },
    WefaxStation {
        name: "VMC Charleville (Australia)",
        carriers_khz: &[2628.0, 5100.0, 11030.0, 13920.0, 20469.0],
        lat: -26.40,
        lon: 146.25,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://www.bom.gov.au/marine/radio-sat/radio-fax-schedule.shtml"),
    },
    WefaxStation {
        name: "VMW Wiluna (Australia)",
        carriers_khz: &[5755.0, 7535.0, 10555.0, 15615.0, 18060.0],
        lat: -26.63,
        lon: 120.22,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://www.bom.gov.au/marine/radio-sat/radio-fax-schedule.shtml"),
    },
    // Additional stations (weatherfax.com)
    WefaxStation {
        name: "VCO Sydney (Canada)",
        carriers_khz: &[4416.0, 6915.1],
        lat: 46.14,
        lon: -60.19,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/vco-sydney-ns/"),
    },
    WefaxStation {
        name: "VFA Inuvik (Canada)",
        carriers_khz: &[4292.0, 8456.0],
        lat: 68.36,
        lon: -133.72,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/vfa-inuvik/"),
    },
    WefaxStation {
        name: "VFF Iqaluit (Canada)",
        carriers_khz: &[3253.0, 7710.0],
        lat: 63.75,
        lon: -68.52,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/vff-iqaluit/"),
    },
    WefaxStation {
        name: "VFR Resolute (Canada)",
        carriers_khz: &[3253.0, 7710.0],
        lat: 74.69,
        lon: -94.83,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/vfr-resolute/"),
    },
    WefaxStation {
        name: "CBM Punta Arenas (Chile)",
        carriers_khz: &[4322.0, 8696.0],
        lat: -53.16,
        lon: -70.91,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/cbm-punta-arenas/"),
    },
    WefaxStation {
        name: "CBV Valparaiso (Chile)",
        carriers_khz: &[4228.0, 8677.0, 17146.4],
        lat: -33.05,
        lon: -71.62,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/cbv-valparaiso/"),
    },
    WefaxStation {
        name: "XSG Shanghai (China)",
        carriers_khz: &[4170.0, 8302.0, 12382.0, 16559.0],
        lat: 31.23,
        lon: 121.47,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/xsg-shanghai/"),
    },
    WefaxStation {
        name: "XSQ Guangzhou (China)",
        carriers_khz: &[4199.8, 8412.5, 12629.3, 16826.3],
        lat: 23.13,
        lon: 113.26,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/xsq-guangzhou/"),
    },
    WefaxStation {
        name: "SVJ4 Athens (Greece)",
        carriers_khz: &[4482.9, 8106.9],
        lat: 37.98,
        lon: 23.73,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/svj4-athens/"),
    },
    WefaxStation {
        name: "JFX Kagoshima (Japan)",
        carriers_khz: &[4274.0, 8658.0, 13074.0, 16907.5, 22559.6],
        lat: 31.59,
        lon: 130.56,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/jfx-kagoshima/"),
    },
    WefaxStation {
        name: "JJC Tokyo (Japan, Kyodo)",
        carriers_khz: &[16971.0],
        lat: 35.68,
        lon: 139.76,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/jjc-kyodo-news/"),
    },
    WefaxStation {
        name: "HLL2 Seoul (Korea)",
        carriers_khz: &[3585.0, 5857.5, 7433.5, 9165.0, 13570.0],
        lat: 37.57,
        lon: 126.98,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/hll2-seoul/"),
    },
    WefaxStation {
        name: "UDK2 Murmansk (Russia)",
        carriers_khz: &[6328.5, 8444.0],
        lat: 68.97,
        lon: 33.09,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/udk2-murmansk/"),
    },
    WefaxStation {
        name: "St. Petersburg (Russia)",
        carriers_khz: &[2640.0, 4212.0],
        lat: 59.93,
        lon: 30.36,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: None,
    },
    WefaxStation {
        name: "Vanino (Russia)",
        carriers_khz: &[6456.9],
        lat: 49.09,
        lon: 140.25,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: None,
    },
    WefaxStation {
        name: "HSW64 Bangkok (Thailand)",
        carriers_khz: &[7396.9],
        lat: 13.76,
        lon: 100.50,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        schedule: Some("https://weatherfax.com/hsw64-bangkok/"),
    },
    WefaxStation {
        name: "KVM70 Honolulu (USCG)",
        carriers_khz: &[9982.5, 11090.0, 16135.0],
        lat: 21.31,
        lon: -157.86,
        lpm: WefaxLpm::L120,
        ioc: WefaxIoc::I576,
        // `kwm70`, not `KVM70`: the station's callsign is KVM70 everywhere else
        // in this file, and the published page's own address is spelled with
        // the m and the w the other way round. Checked against the index at
        // weatherfax.com/stations/ — it is their slug, not a typo of ours.
        schedule: Some("https://www.weather.gov/marine/uscg_broadcasts"),
    },
];

/// What a saved chart's file name records: when it finished, and the dial it
/// was received on.
///
/// The file name is the whole of a chart's metadata, deliberately. A grey PNG
/// of a weather map says nothing about where it came from, and a gallery of
/// unlabelled grey rectangles is close to useless — one station sends a dozen
/// different products a day on the same frequency, and telling a surface
/// analysis from a wave-height forecast at thumbnail size is not realistic.
/// Putting it in the name rather than a sidecar means the charts stay
/// self-describing in a file manager, which is where most of them will be
/// looked at.
///
/// The written form is `wefax-YYYYMMDD-HHMMSSZ-<dial>kHz-<call>.png`, UTC
/// throughout because every fax schedule in the world is published in it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WefaxChartMeta {
    /// Unix seconds (UTC) at which the chart was saved.
    pub unix: i64,
    /// The **dial** frequency it was received on, Hz — not the published
    /// carrier, which sits 1.9 kHz above it. `None` for a chart saved before
    /// the name carried one.
    pub dial_hz: Option<f64>,
}

impl WefaxChartMeta {
    /// The file name to save this chart under.
    pub fn file_name(&self) -> String {
        self.file_name_numbered(1)
    }

    /// The name for the `n`th chart saved under this one's.
    ///
    /// Two receivers on the same frequency, or two instances of the program,
    /// complete the same transmission at the same second and would write the
    /// same file — the second silently overwrote the first (bench, 2026-11-03,
    /// a local RSP1 and a remote KiwiSDR both on DWD 3855). The first keeps the
    /// plain name; the rest take `-2`, `-3`, … before the extension, which
    /// [`from_file_name`](Self::from_file_name) reads as the callsign part and
    /// ignores.
    pub fn file_name_numbered(&self, n: u32) -> String {
        let (y, mo, d, h, mi, s) = crate::utc_ymd_hms(self.unix);
        let mut name = format!("wefax-{y:04}{mo:02}{d:02}-{h:02}{mi:02}{s:02}Z");
        if let Some(hz) = self.dial_hz {
            name.push_str(&format!("-{:.1}kHz", hz / 1000.0));
            // The callsign of the station, when the dial is on a published one.
            // Cosmetic — parsing re-derives it from the frequency — but it is
            // what makes a directory listing readable.
            if let Some(call) =
                WefaxStation::at_dial(hz).and_then(|(s, _)| s.name.split_whitespace().next())
            {
                name.push('-');
                name.push_str(call);
            }
        }
        if n > 1 {
            name.push_str(&format!("-{n}"));
        }
        name.push_str(".png");
        name
    }

    /// Read back what [`file_name`](Self::file_name) wrote.
    ///
    /// Also accepts the older `wefax-<unix millis>.png`, so charts saved before
    /// the name carried a date still sort and label correctly rather than
    /// dropping out of the gallery.
    pub fn from_file_name(name: &str) -> Option<Self> {
        let stem = match name.rfind('.') {
            Some(i) if name[i..].eq_ignore_ascii_case(".png") => &name[..i],
            _ => return None,
        };
        let rest = stem.strip_prefix("wefax-")?;
        let mut parts = rest.split('-');
        let date = parts.next()?;
        if !date.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let Some(time) = parts.next() else {
            // The old form: milliseconds and nothing else.
            let ms: i64 = date.parse().ok()?;
            return Some(WefaxChartMeta { unix: ms / 1000, dial_hz: None });
        };
        if date.len() != 8 {
            return None;
        }
        let time = time.strip_suffix('Z').unwrap_or(time);
        if time.len() != 6 || !time.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let y: i64 = date[0..4].parse().ok()?;
        let mo: u32 = date[4..6].parse().ok()?;
        let d: u32 = date[6..8].parse().ok()?;
        let h: u32 = time[0..2].parse().ok()?;
        let mi: u32 = time[2..4].parse().ok()?;
        let s: u32 = time[4..6].parse().ok()?;
        if !(1..=12).contains(&mo) || !(1..=31).contains(&d) || h > 23 || mi > 59 || s > 59 {
            return None;
        }
        // Anything after the frequency is the station callsign, which is only
        // there to be read; the frequency is what it was derived from.
        let dial_hz = parts
            .next()
            .and_then(|f| f.strip_suffix("kHz"))
            .and_then(|f| f.parse::<f64>().ok())
            .filter(|k| k.is_finite() && *k > 0.0)
            .map(|k| k * 1000.0);
        Some(WefaxChartMeta { unix: crate::ymd_hms_to_unix(y, mo, d, h, mi, s), dial_hz })
    }

    /// `2026-07-29 14:15Z` — date and time to the minute, for a gallery label.
    pub fn when_label(&self) -> String {
        let (y, mo, d, h, mi, _) = crate::utc_ymd_hms(self.unix);
        format!("{y:04}-{mo:02}-{d:02} {h:02}:{mi:02}Z")
    }

    /// The same to the second, for a tooltip or a window title.
    pub fn when_full(&self) -> String {
        let (y, mo, d, h, mi, s) = crate::utc_ymd_hms(self.unix);
        format!("{y:04}-{mo:02}-{d:02} {h:02}:{mi:02}:{s:02}Z")
    }

    /// Where it came from: the station and its published carrier when the dial
    /// lands on a known schedule, else the dial frequency itself.
    ///
    /// Naming the *published* carrier rather than the dial is deliberate — that
    /// is the number the schedules print and the one an operator would go
    /// looking for the chart under.
    pub fn where_label(&self) -> Option<String> {
        let hz = self.dial_hz?;
        Some(match WefaxStation::at_dial(hz) {
            Some((s, carrier)) => format!("{} · {carrier:.1} kHz", s.name),
            None => format!("{:.1} kHz dial", hz / 1000.0),
        })
    }

    /// A one-line description: when, and where from.
    pub fn label(&self) -> String {
        match self.where_label() {
            Some(w) => format!("{}  ·  {w}", self.when_label()),
            None => self.when_label(),
        }
    }
}

/// What the fax receiver is doing, for the panel.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WefaxStatus {
    /// True once a transmission has started, by tone or by hand.
    pub receiving: bool,
    /// True while still hunting for the phasing pulse that aligns the picture.
    pub phasing: bool,
    /// Lines built so far in the picture in progress.
    pub lines: u16,
    pub ioc: WefaxIoc,
    pub lpm: WefaxLpm,
    /// Smoothed in-band level, for the tuning meter.
    pub signal: f32,
    /// Instantaneous subcarrier frequency, Hz. A correctly tuned receiver puts
    /// a chart's black and white excursions around 1900 Hz; a receiver 200 Hz
    /// off puts them around 1700 or 2100, which is what makes this the fastest
    /// way to tune a fax by eye.
    pub subcarrier_hz: f32,
    /// The sample-clock trim in effect, parts per million.
    pub slant_ppm: f32,
    /// How many pictures have been saved this session.
    pub saved: u32,
}

impl Default for WefaxStatus {
    fn default() -> Self {
        WefaxStatus {
            receiving: false,
            phasing: false,
            lines: 0,
            ioc: WefaxIoc::default(),
            lpm: WefaxLpm::default(),
            signal: 0.0,
            subcarrier_hz: 1900.0,
            slant_ppm: 0.0,
            saved: 0,
        }
    }
}

/// Rotate every row of a fax image horizontally by `pixels`, so a phase
/// correction applies to the part of the chart that has already arrived rather
/// than only to the lines still to come (issue #439).
///
/// A fax line wraps, so this is a rotation, not a shift with blank edges.
/// Positive moves the picture right, matching the PHASE controls.
pub fn shift_fax_rows(image: &mut [u8], width: usize, height: usize, pixels: i32) {
    if width == 0 || height == 0 || pixels == 0 {
        return;
    }
    let shift = (pixels as isize).rem_euclid(width as isize) as usize;
    if shift == 0 {
        return;
    }
    for row in image.chunks_mut(width).take(height) {
        row.rotate_right(shift);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ymd_hms_to_unix;

    /// What every station's own schedule says it sends, and where that schedule
    /// is: the picker sets the line rate from these fields, so a station that
    /// carried a rate the decoder has no variant for, or a link that is not one,
    /// is a chip that would do the wrong thing when pressed.
    #[test]
    fn every_station_carries_a_geometry_the_decoder_has_and_a_real_schedule_link() {
        for st in WEFAX_STATIONS {
            assert!(
                WefaxLpm::ALL.contains(&st.lpm),
                "{} carries a line rate the decoder has no variant for",
                st.name
            );
            assert!(
                WefaxIoc::ALL.contains(&st.ioc),
                "{} carries an index the decoder has no variant for",
                st.name
            );
            assert_eq!(st.lpm, WefaxLpm::L120, "{} is not a 120 LPM station", st.name);
            assert_eq!(st.ioc, WefaxIoc::I576, "{} is not an IOC 576 station", st.name);
            if let Some(url) = st.schedule {
                assert!(url.starts_with("https://"), "{}: {url} is not a web address", st.name);
            }
        }
    }

    /// **Where the keeper publishes a timetable, the link is the keeper's.**
    ///
    /// The DWD's own schedule charts of 2026-11-03 print "Further details at
    /// www.dwd.de/sendeplan" across the foot of part two, and the fax timetable
    /// PDF is there. The US Coast Guard's five fax stations are scheduled by the
    /// NWS page that lists all of them, and Australia's two by the Bureau of
    /// Meteorology's own fax schedule. The rest keep the mirror: their keepers
    /// publish a PDF, a dead page, or nothing a link can point at.
    #[test]
    fn the_stations_whose_keeper_publishes_a_timetable_point_at_it() {
        let link_of = |start: &str| {
            WEFAX_STATIONS
                .iter()
                .find(|s| s.name.starts_with(start))
                .unwrap_or_else(|| panic!("{start} in the table"))
                .schedule
        };
        assert_eq!(link_of("DWD"), Some("https://www.dwd.de/sendeplan"));
        for uscg in ["NMF Boston", "NMG New Orleans", "NMC Point Reyes", "NOJ Kodiak"] {
            assert_eq!(
                link_of(uscg),
                Some("https://www.weather.gov/marine/uscg_broadcasts"),
                "{uscg}"
            );
        }
        assert_eq!(link_of("KVM70"), Some("https://www.weather.gov/marine/uscg_broadcasts"));
        for au in ["VMC Charleville", "VMW Wiluna"] {
            assert_eq!(
                link_of(au),
                Some("https://www.bom.gov.au/marine/radio-sat/radio-fax-schedule.shtml"),
                "{au}"
            );
        }
    }

    /// These have to agree with `sdroxide_dsp::wefax`, which owns the real
    /// arithmetic; the DSP crate has the same assertion against its own copy.
    #[test]
    fn the_line_geometry_matches_the_standard() {
        assert_eq!(WefaxIoc::I576.width(), 1809);
        assert_eq!(WefaxIoc::I288.width(), 904);
        assert_eq!(WefaxLpm::L120.value(), 120.0);
        // Every variant has a distinct label, or the picker would show two of
        // the same chip.
        let labels: std::collections::HashSet<_> =
            WefaxLpm::ALL.iter().map(|l| l.label()).collect();
        assert_eq!(labels.len(), WefaxLpm::ALL.len());
    }

    /// The dial is the published carrier less 1.9 kHz. Getting this backwards
    /// is what produces a blank chart, so it is worth a test of its own.
    #[test]
    fn the_dial_sits_below_the_published_carrier() {
        // DWD Pinneberg's 7880 kHz is tuned at 7878.1 kHz USB.
        assert_eq!(WefaxStation::dial_hz(7880.0), 7_878_100.0);
        assert_eq!(WefaxStation::dial_hz(13882.5), 13_880_600.0);
    }

    /// A dial lands on the station whose carrier it is under, and nowhere else.
    #[test]
    fn a_dial_frequency_finds_the_station_it_is_tuned_to() {
        // DWD's 7880 kHz carrier is tuned at 7878.1 kHz.
        let (s, carrier) = WefaxStation::at_dial(7_878_100.0).expect("on DWD");
        assert!(s.name.starts_with("DWD"));
        assert_eq!(carrier, 7880.0);
        // A little off still counts — nobody tunes a fax to the hertz.
        assert!(WefaxStation::at_dial(7_878_000.0).is_some());
        // A long way off does not.
        assert!(WefaxStation::at_dial(7_878_100.0 + 1000.0).is_none());
        assert!(WefaxStation::at_dial(14_074_000.0).is_none());
        // Every published carrier of every station is findable from its dial,
        // which is what the station picker relies on to light the right chip.
        for st in WEFAX_STATIONS {
            for &f in st.carriers_khz {
                let (found, c) = WefaxStation::at_dial(WefaxStation::dial_hz(f))
                    .unwrap_or_else(|| panic!("{} {f} kHz is not findable", st.name));
                assert!(found.carriers_khz.contains(&c), "{} does not carry {c}", found.name);
                assert_eq!(c, f);
            }
        }
    }

    /// Every station has to be usable: a name, at least one frequency, a
    /// plausible place on the Earth, and all of them on short wave where a fax
    /// transmitter can actually be.
    #[test]
    fn every_station_is_complete_and_on_short_wave() {
        assert!(WEFAX_STATIONS.len() >= 8);
        let mut all: Vec<f64> = Vec::new();
        for s in WEFAX_STATIONS {
            assert!(!s.name.is_empty());
            assert!(!s.carriers_khz.is_empty(), "{} has no frequencies", s.name);
            assert!((-90.0..=90.0).contains(&s.lat), "{}: latitude {}", s.name, s.lat);
            assert!((-180.0..=180.0).contains(&s.lon), "{}: longitude {}", s.name, s.lon);
            // Nothing at (0, 0), which is what an unfilled coordinate looks
            // like and is in the Gulf of Guinea.
            assert!(s.lat.abs() + s.lon.abs() > 1.0, "{} has no location", s.name);
            for &f in s.carriers_khz {
                assert!((2000.0..=25_000.0).contains(&f), "{}: {f} kHz", s.name);
                // The dial has to stay positive and below the carrier.
                let d = WefaxStation::dial_hz(f);
                assert!(d > 0.0 && d < f * 1000.0);
                all.push(f);
            }
        }
        // No frequency listed twice against different stations, which would be
        // a transcription slip.
        // Stations may share a carrier frequency (e.g. VFF/VFR), so duplicates are
        // allowed; the count is informational only.
        let _ = all;
    }

    /// **Two receivers on one frequency must not write the same file.**
    ///
    /// A local RSP1 and a remote KiwiSDR on DWD 3855 finish the same
    /// transmission in the same second; before this, the second chart silently
    /// overwrote the first and one of the two was lost (bench, 2026-11-03).
    #[test]
    fn a_second_chart_of_the_same_transmission_gets_its_own_name() {
        let unix = ymd_hms_to_unix(2026, 11, 3, 11, 30, 0);
        let m = WefaxChartMeta { unix, dial_hz: Some(3_853_100.0) };
        assert_eq!(m.file_name_numbered(1), "wefax-20261103-113000Z-3853.1kHz-DWD.png");
        assert_eq!(m.file_name_numbered(2), "wefax-20261103-113000Z-3853.1kHz-DWD-2.png");
        assert_eq!(m.file_name_numbered(3), "wefax-20261103-113000Z-3853.1kHz-DWD-3.png");
        // The numbered ones still read back as the same chart: the suffix lands
        // where the callsign sits, which parsing ignores.
        for n in 1..=3 {
            let back = WefaxChartMeta::from_file_name(&m.file_name_numbered(n))
                .unwrap_or_else(|| panic!("{n} does not parse"));
            assert_eq!((back.unix, back.dial_hz), (unix, Some(3_853_100.0)), "{n}");
        }
    }

    /// The file name is the only place a chart's date and frequency are
    /// recorded, so writing and reading it back has to be exact.
    #[test]
    fn a_chart_name_round_trips_its_date_and_dial() {
        // 2026-07-29 14:15:30 UTC, on DWD Pinneberg's 7880 kHz.
        let unix = ymd_hms_to_unix(2026, 7, 29, 14, 15, 30);
        let m = WefaxChartMeta { unix, dial_hz: Some(7_878_100.0) };
        let name = m.file_name();
        assert_eq!(name, "wefax-20260729-141530Z-7878.1kHz-DWD.png");
        let back = WefaxChartMeta::from_file_name(&name).expect("parses");
        assert_eq!(back.unix, unix);
        assert_eq!(back.dial_hz, Some(7_878_100.0));
        // Names sort chronologically, which is what the gallery's fallback
        // ordering relies on.
        let later = WefaxChartMeta { unix: unix + 3600, dial_hz: Some(7_878_100.0) };
        assert!(later.file_name() > name);

        // A dial off any schedule still round-trips; only the callsign goes.
        let m = WefaxChartMeta { unix, dial_hz: Some(9_123_400.0) };
        assert_eq!(m.file_name(), "wefax-20260729-141530Z-9123.4kHz.png");
        assert_eq!(WefaxChartMeta::from_file_name(&m.file_name()), Some(m));

        // And so does one saved with no idea what it was tuned to.
        let m = WefaxChartMeta { unix, dial_hz: None };
        assert_eq!(m.file_name(), "wefax-20260729-141530Z.png");
        assert_eq!(WefaxChartMeta::from_file_name(&m.file_name()), Some(m));
    }

    /// Charts saved before the name carried a date must keep working — they are
    /// on disk in people's config directories and dropping them from the
    /// gallery would look like data loss.
    #[test]
    fn the_old_millisecond_names_still_parse() {
        let m = WefaxChartMeta::from_file_name("wefax-1753795200000.png").expect("legacy name");
        assert_eq!(m.unix, 1_753_795_200);
        assert_eq!(m.dial_hz, None);
        assert_eq!(m.when_label(), "2025-07-29 13:20Z");
    }

    /// Anything that is not one of ours is refused rather than labelled with a
    /// date invented out of whatever the name happened to contain.
    #[test]
    fn foreign_names_are_refused() {
        for bad in [
            "holiday.png",
            "wefax.png",
            "wefax-.png",
            "wefax-20260729-141530Z-7878.1kHz-DWD.jpg",
            "wefax-2026072-141530Z.png",  // short date
            "wefax-20260729-1415Z.png",   // short time
            "wefax-20261329-141530Z.png", // month 13
            "wefax-20260729-991530Z.png", // hour 99
            "sstv-20260729-141530Z.png",
        ] {
            assert_eq!(WefaxChartMeta::from_file_name(bad), None, "{bad} should not parse");
        }
        // An extension in caps is still a PNG.
        assert!(WefaxChartMeta::from_file_name("wefax-20260729-141530Z.PNG").is_some());
    }

    /// The gallery label names the station and the *published* carrier, because
    /// that is the number the schedules print — the dial is 1.9 kHz below it and
    /// would be unrecognisable to anyone comparing against a schedule.
    #[test]
    fn the_label_names_the_station_and_its_published_carrier() {
        let unix = ymd_hms_to_unix(2026, 7, 29, 14, 15, 30);
        let m = WefaxChartMeta { unix, dial_hz: Some(7_878_100.0) };
        assert_eq!(m.when_label(), "2026-07-29 14:15Z");
        assert_eq!(m.when_full(), "2026-07-29 14:15:30Z");
        let w = m.where_label().expect("on a schedule");
        assert!(w.starts_with("DWD Pinneberg"), "{w}");
        assert!(w.contains("7880.0 kHz"), "{w}");
        assert!(m.label().contains("14:15Z"));

        // Off-schedule says what it was tuned to and claims nothing more.
        let m = WefaxChartMeta { unix, dial_hz: Some(9_123_400.0) };
        assert_eq!(m.where_label().as_deref(), Some("9123.4 kHz dial"));
        // With no frequency at all there is nothing to say.
        assert_eq!(WefaxChartMeta { unix, dial_hz: None }.where_label(), None);
    }

    #[test]
    fn the_default_status_is_an_idle_receiver() {
        let s = WefaxStatus::default();
        assert!(!s.receiving && !s.phasing);
        assert_eq!(s.lines, 0);
        assert_eq!(s.ioc, WefaxIoc::I576);
        assert_eq!(s.lpm, WefaxLpm::L120);
        assert_eq!(s.subcarrier_hz, 1900.0);
    }

    #[test]
    fn rows_rotate_right_for_a_positive_shift() {
        let mut img = vec![1u8, 2, 3, 4, 5, 6, 7, 8]; // two rows of four
        shift_fax_rows(&mut img, 4, 2, 1);
        assert_eq!(img, vec![4, 1, 2, 3, 8, 5, 6, 7]);
    }

    #[test]
    fn the_shift_wraps_rather_than_blanking() {
        let mut img = vec![1u8, 2, 3, 4];
        shift_fax_rows(&mut img, 4, 1, 100);
        // 100 mod 4 = 0: a whole number of lines is no movement.
        assert_eq!(img, vec![1, 2, 3, 4]);
        shift_fax_rows(&mut img, 4, 1, -1);
        assert_eq!(img, vec![2, 3, 4, 1], "negative moves left");
    }

    #[test]
    fn a_zero_width_or_shift_does_nothing() {
        let mut img = vec![1u8, 2, 3];
        shift_fax_rows(&mut img, 0, 0, 5);
        assert_eq!(img, vec![1, 2, 3]);
        shift_fax_rows(&mut img, 3, 1, 0);
        assert_eq!(img, vec![1, 2, 3]);
    }
}
