//! DAB / DAB+ reception for this program, wrapping the [`dabradio`] decoder.
//!
//! # Status: NOT SHIPPED — retained for future work
//!
//! **DAB is deliberately hidden from the user in this build.** Nothing in the UI
//! offers the mode, and [`DAB_ENABLED`] is `false`, which stops the engine
//! starting the lane even if a stored session still names `Mode::Dab`. The code
//! is kept so the work is not thrown away, not because it is ready.
//!
//! What it does do, proven off air: the OFDM front end syncs, the FIC decodes,
//! and the ensemble and **service list** come up — including a multiplex that
//! advertises no ensemble label, which some do. What it long did **not** do was
//! play a station cleanly: faad2 used to refuse most Access Units on **every**
//! service while the reference `dabradio` decoder (fdk-aac) took them cleanly,
//! which chopped the audio. The cause was ours: faad2 is built with
//! `DRM_SUPPORT` for the DRM receiver, and in that mode it rejects any frame
//! whose fill element carries something other than SBR data (error 30) — which
//! DAB+ encoders send routinely. `sdroxide-faad2` now patches that refusal at
//! build time; on an off-air capture (8B, BFM BUSINESS) the decoded audio went
//! from 6.1 s to 9.1 s of a 10 s recording, the rest being sync acquisition.
//!
//! ## The licence constraint that shapes any future fix
//!
//! `dabradio` reaches its AAC stage through **fdk-aac**, which is optional
//! there. The FDK licence grants **no patent licence** and forbids a copyright
//! licence fee — restrictions the GPL cannot carry — so a binary with fdk-aac
//! **linked in cannot be distributed** (it is why Debian ships it `non-free`).
//! This crate therefore runs the pipeline with fdk-aac **off** and hands the
//! Reed–Solomon-corrected Access Units to the **stock faad2** the binary already
//! carries for DRM ([`AacDecoder`]). Any future fix must stay on this side of
//! the line: a GPL-compatible decoder, or fdk-aac loaded **at run time** (the
//! trick `vendor/dream` already uses) rather than linked. DAB (not DAB+) carries
//! MP2, which is pure Rust in `dabradio` and needs no swap.
//!
//! DAB Mode I is a **wideband** service — about 1.536 MHz of occupied spectrum
//! in Band III, demodulated by an OFDM front end that wants its input at
//! 2.048 Msps — so, like ADS-B, it cannot ride the 12 kHz `on_rx_iq` tap the
//! narrow modes share. It is fed raw I/Q from a lane of its own.

use std::collections::{HashMap, VecDeque};

use num_complex::Complex32;
use thiserror::Error;
use tracing::warn;

pub mod aac;

pub use aac::AacDecoder;

/// Whether DAB ships in this build. **`false` — see the crate docs.**
///
/// A single switch, so the whole feature is one flag rather than a scattering
/// of `cfg`s. The UI does not offer the mode; the engine consults this so a
/// stored session that once selected `Mode::Dab` cannot start a lane the build
/// does not support. Flip it to `true` (and restore the mode to the band menu's
/// chip lists) to bring DAB back once the audio path is fixed.
pub const DAB_ENABLED: bool = true;

/// Re-exported from [`sdroxide_types`], where the UI can reach them too: the
/// window planning that decides whether a front end can serve DAB is arithmetic
/// the browser client runs (`sdroxide_dsp::dab_window_rate`), and this decoder
/// is native-only, so it cannot be their home.
pub use sdroxide_types::{DAB_BANDWIDTH_HZ, DAB_GOOD_RATE_HZ, DAB_SAMPLE_RATE};

#[derive(Debug, Error)]
pub enum DabError {
    #[error(
        "the front end's sample rate is below the {DAB_BANDWIDTH_HZ:.0} Hz a DAB ensemble needs"
    )]
    LaneTooNarrow,
    #[error("could not build a resampler from {0} Hz to {DAB_SAMPLE_RATE} Hz")]
    Resample(f64),
}

/// One service in the ensemble, as the FIC describes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DabService {
    /// The service identifier, as the FIC carries it (`0xF201`).
    pub service_id: String,
    /// The service label (programme name).
    pub label: String,
    /// The sub-channel this service's data rides in, when known.
    pub subchannel: Option<u8>,
    /// Bitrate, in kbps.
    pub bitrate: Option<u16>,
    /// The protection profile (`EEP 3-A` and so on).
    pub protection: Option<String>,
    /// The programme type the station signals (FIG 0/17): "News", "Pop
    /// Music"… `None` when the ensemble has not said.
    pub pty: Option<String>,
}

/// A picture the playing station sent in its slideshow (MOT SlideShow, in
/// the programme-associated data of its audio): the bytes as broadcast — a
/// JPEG or a PNG, at whatever size the station chose.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DabSlide {
    /// `image/jpeg` or `image/png`.
    pub content_type: String,
    /// The name the station gave the object, when it gave one.
    pub name: Option<String>,
    pub data: Vec<u8>,
}

/// The programme type names for the international code (EN 300 401 / IEC
/// 62106, the RDS table), by code 0–31. Code 0 is "no programme type" and is
/// not shown.
const PTY_NAMES: [&str; 32] = [
    "",
    "News",
    "Current Affairs",
    "Information",
    "Sport",
    "Education",
    "Drama",
    "Culture",
    "Science",
    "Varied",
    "Pop Music",
    "Rock Music",
    "Easy Listening",
    "Light Classical",
    "Serious Classical",
    "Other Music",
    "Weather",
    "Finance",
    "Children's",
    "Social Affairs",
    "Religion",
    "Phone In",
    "Travel",
    "Leisure",
    "Jazz Music",
    "Country Music",
    "National Music",
    "Oldies Music",
    "Folk Music",
    "Documentary",
    "Alarm Test",
    "Alarm",
];

/// The name for a programme-type code, or `None` for 0 and anything out of range.
pub fn pty_name(code: u8) -> Option<&'static str> {
    PTY_NAMES.get(code as usize).copied().filter(|n| !n.is_empty())
}

/// Read every FIG 0/17 (programme type) entry in one FIB into `out`, by
/// service id. `dabradio` does not parse this FIG, so it is read here from the
/// same FIB bytes, CRC already checked.
///
/// Layout (EN 300 401 §8.1.5): per service, a 16-bit SId; a flags byte whose
/// third and fourth bits are the language and complementary-code flags (both
/// zero in the current edition, set by older encoders); a language byte when
/// flagged; the international code in the low five bits of the next byte; and
/// a complementary-code byte when flagged. Only this ensemble's own services
/// (OE = 0) with 16-bit ids (P/D = 0) — programme services — are read.
fn parse_fig0_17(fib: &[u8], out: &mut HashMap<u32, u8>) {
    let data = &fib[..fib.len().min(30)];
    let mut pos = 0;
    while pos < data.len() {
        let fig_type = (data[pos] >> 5) & 0x07;
        let len = (data[pos] & 0x1F) as usize;
        if fig_type == 7 || len == 0 || pos + 1 + len > data.len() {
            break;
        }
        let fig = &data[pos + 1..pos + 1 + len];
        pos += 1 + len;
        if fig_type != 0 || fig.is_empty() {
            continue;
        }
        let oe = (fig[0] >> 6) & 1;
        let pd = (fig[0] >> 5) & 1;
        if fig[0] & 0x1F != 17 || oe != 0 || pd != 0 {
            continue;
        }
        let body = &fig[1..];
        let mut i = 0;
        while i + 4 <= body.len() {
            let sid = u32::from(body[i]) << 8 | u32::from(body[i + 1]);
            let flags = body[i + 2];
            let lang = (flags >> 5) & 1 == 1;
            let cc = (flags >> 4) & 1 == 1;
            let mut j = i + 3;
            if lang {
                j += 1;
            }
            let Some(&code) = body.get(j) else { break };
            out.insert(sid, code & 0x1F);
            j += 1;
            if cc {
                j += 1;
            }
            i = j;
        }
    }
}

/// The ensemble a lane is tuned to, and what it carries.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DabEnsemble {
    /// The ensemble label ("Métropolitain 2").
    pub label: Option<String>,
    pub services: Vec<DabService>,
}

/// What a service's audio is coded with — the choice that decides which decoder
/// runs, and so whether our faad2 or `dabradio`'s own MP2 does the work.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DabCoding {
    /// Legacy DAB, MPEG-1/2 Layer II — pure Rust, no faad2.
    MpegLayer2,
    /// DAB+, HE-AAC — decoded through our [`AacDecoder`] (faad2).
    DabPlus,
    /// A component the FIC named that neither decoder handles.
    Unsupported,
}

/// The audio pipeline for one service: MSC frames in, interleaved PCM out.
struct AudioPipe {
    coding: DabCoding,
    /// Extracts sub-channel frames from the MSC soft bits.
    msc: dabradio::msc::MscHandler,
    /// DAB+: Reed–Solomon super-frames → Access Units.
    superframe: Option<dabradio::audio::SuperframeDecoder>,
    /// DAB+: the AAC decoder (ours, over faad2).
    aac: Option<AacDecoder>,
    /// DAB (MP2): `dabradio`'s own decoder.
    mp2: Option<dabradio::audio::mp2::Mp2Decoder>,
    /// Logical frames waiting to be handed to the super-frame assembler.
    pending: VecDeque<Vec<u8>>,
    /// DAB+: the station's text (DLS) and slideshow (MOT), carried in the
    /// programme-associated data of each Access Unit.
    pad: dabradio::pad::PadExtractor,
    /// Whether the AAC decoder has been configured from the super-frame's ASC.
    aac_configured: bool,
}

/// One DAB receiver: raw I/Q in at whatever the front end gives, ensemble,
/// services and — once a service is picked — PCM audio out.
///
/// The front end opens at many rates (a 2.5 Msps capture, a 2.4 Msps dongle, a
/// 6 Msps HF+); the OFDM processor wants one. [`Self::new`] builds the bridge.
pub struct DabReceiver {
    ofdm: dabradio::ofdm::processor::OfdmProcessor,
    ensemble: dabradio::fic::fib::EnsembleInfo,
    /// Resamples the front end's rate down to [`DAB_SAMPLE_RATE`].
    resampler: Option<sdroxide_dsp::ComplexResampler>,
    resample_out: Vec<Complex32>,
    /// The service being decoded, and its pipeline. `None` until one is picked.
    audio: Option<AudioPipe>,
    /// The service to decode, by id or label, held until the FIC names it.
    wanted: Option<String>,
    /// OFDM frames and FIBs read since the receiver started.
    frames: u64,
    fibs: u64,
    /// Programme types by service id, from FIG 0/17.
    ptys: HashMap<u32, u8>,
    /// The playing station's latest text (DLS).
    dls: Option<String>,
    /// A slideshow picture not yet handed out.
    slide: Option<DabSlide>,
}

impl DabReceiver {
    /// Build a receiver for a front end running at `input_rate_hz`.
    ///
    /// Errors when the rate is too low to hold an ensemble, or when no
    /// resampler can bridge it — both are the operator's problem to see, not to
    /// discover as silence.
    pub fn new(input_rate_hz: f64) -> Result<Self, DabError> {
        if input_rate_hz < DAB_BANDWIDTH_HZ {
            return Err(DabError::LaneTooNarrow);
        }
        // A resampler is only needed — and only *constructible* — when the
        // rates differ: `ComplexResampler::new` answers `None` for equal rates,
        // which is not a failure. Treating it as one rejected exactly 2.048
        // Msps, the one rate that needs no bridge at all.
        let resampler = if (input_rate_hz - DAB_SAMPLE_RATE as f64).abs() < 0.01 {
            None
        } else {
            Some(
                sdroxide_dsp::ComplexResampler::new(input_rate_hz, DAB_SAMPLE_RATE as f64)
                    .ok_or(DabError::Resample(input_rate_hz))?,
            )
        };
        Ok(DabReceiver {
            ofdm: dabradio::ofdm::processor::OfdmProcessor::new(),
            ensemble: dabradio::fic::fib::EnsembleInfo::new(),
            resampler,
            resample_out: Vec::new(),
            audio: None,
            wanted: None,
            frames: 0,
            fibs: 0,
            ptys: HashMap::new(),
            dls: None,
            slide: None,
        })
    }

    /// Feed a block of the front end's I/Q. FIBs fold into the ensemble, and —
    /// once a service is selected and named — its audio is decoded.
    pub fn feed(&mut self, iq: &[Complex32]) {
        // Resample into an owned buffer when the front end's rate differs, so
        // the borrow ends before `self.ofdm` is used.
        let baseband: Vec<Complex32> = match self.resampler.as_mut() {
            Some(rs) => {
                self.resample_out.clear();
                rs.push(iq, &mut self.resample_out);
                std::mem::take(&mut self.resample_out)
            }
            None => iq.to_vec(),
        };
        // A selected service can only be started once the FIC has resolved it.
        if self.audio.is_none()
            && let Some(want) = self.wanted.clone()
        {
            self.ensemble.resolve_services();
            self.start_audio(&want);
        }
        for frame in self.ofdm.process(&baseband) {
            self.frames += 1;
            let soft_bits = dabradio::ofdm::decoder::dqpsk_decode(&frame.symbols);
            // The FIC rides the first three symbols of every CIF.
            if soft_bits.len() >= 3 {
                let fibs = dabradio::fic::handler::process_fic(&soft_bits[..3]);
                // Feed the decode ratio back to the OFDM processor. This is not
                // a statistic: it is how the front end knows it has locked and
                // stops applying coarse-frequency corrections. Without it the
                // loop runs open — frames sync, symbols are mis-rotated, and
                // not one FIB decodes (4 sub-blocks × 3 FIBs = 12 expected a
                // frame, the ratio dradio's own receiver computes).
                let ratio = ((fibs.len() as f32 / 12.0) * 100.0).round().min(100.0) as u8;
                self.ofdm.set_fic_decode_ratio(ratio);
                for fib in fibs {
                    parse_fig0_17(&fib, &mut self.ptys);
                    self.ensemble.parse_fib(&fib);
                    self.fibs += 1;
                }
            }
            // The MSC rides the rest, once a service is being decoded.
            const FIC_SYMBOLS: usize = 3;
            if let Some(pipe) = self.audio.as_mut() {
                let end = soft_bits.len().min(FIC_SYMBOLS + 72);
                for sym in &soft_bits[FIC_SYMBOLS..end] {
                    if let Some(logical) = pipe.msc.feed_symbol(sym) {
                        pipe.pending.push_back(logical);
                    }
                }
            }
        }
    }

    /// The ensemble as it stands, services resolved by label.
    pub fn ensemble(&self) -> DabEnsemble {
        let mut e = self.ensemble.clone();
        e.resolve_services();
        let out = e.to_output();
        DabEnsemble {
            label: out.ensemble_label,
            services: out
                .services
                .into_iter()
                .map(|s| DabService {
                    pty: parse_service_id(&s.service_id)
                        .and_then(|sid| self.ptys.get(&sid))
                        .and_then(|c| pty_name(*c))
                        .map(str::to_string),
                    service_id: s.service_id,
                    label: s.label.unwrap_or_default(),
                    subchannel: s.subchannel_id,
                    bitrate: s.bitrate,
                    protection: s.protection,
                })
                .collect(),
        }
    }

    /// Whether enough of the FIC has arrived to name the services.
    pub fn has_services(&self) -> bool {
        self.ensemble.has_services()
    }

    /// Frames and FIBs read since the receiver started, for the panel's header.
    pub fn counters(&self) -> (u64, u64) {
        (self.frames, self.fibs)
    }

    /// The service being decoded, by id or label, if any.
    pub fn playing(&self) -> Option<&str> {
        self.audio.as_ref().map(|_| self.wanted.as_deref().unwrap_or(""))
    }

    /// Select a service to decode, by service id (`0xF201`) or label.
    ///
    /// The pipeline is built as soon as the FIC names the service; until then
    /// the request is held, so a selection made the moment the list appears is
    /// not lost to a race with the FIC.
    pub fn select_service(&mut self, which: &str) {
        if self.wanted.as_deref() == Some(which) && self.audio.is_some() {
            return;
        }
        self.wanted = Some(which.to_string());
        self.audio = None;
        // Another station: the last one's text and picture are not this one's.
        self.dls = None;
        self.slide = None;
        self.ensemble.resolve_services();
        self.start_audio(which);
    }

    /// Take the PCM decoded so far. Interleaved, at the codec's own rate.
    pub fn take_pcm(&mut self) -> Vec<f32> {
        let Some(pipe) = self.audio.as_mut() else { return Vec::new() };
        let mut pcm = Vec::new();
        let mut pad = Vec::new();
        while let Some(frame) = pipe.pending.pop_front() {
            match pipe.coding {
                DabCoding::DabPlus => {
                    let Some(sf) = pipe.superframe.as_mut() else { continue };
                    let decoded = sf.feed_frame(&frame);
                    // faad2 is configured once, from the AudioSpecificConfig the
                    // super-frame carries — where `DabPlusDecoder` configures
                    // fdk-aac.
                    if !pipe.aac_configured
                        && let Some(fmt) = sf.format
                    {
                        if let Some(mut a) = AacDecoder::new() {
                            let asc = fmt.audio_specific_config();
                            if a.configure(&asc) {
                                pipe.aac = Some(a);
                                pipe.aac_configured = true;
                            } else {
                                warn!("DAB+: faad2 refused the AudioSpecificConfig");
                            }
                        } else {
                            warn!("DAB+: faad2 could not open an AAC decoder");
                        }
                    }
                    if let Some(aus) = decoded
                        && let Some(dec) = pipe.aac.as_mut()
                    {
                        for au in aus {
                            pad.extend(pipe.pad.extract_all_from_au(&au));
                            pcm.extend(dec.decode_au(&au));
                        }
                    }
                }
                DabCoding::MpegLayer2 => {
                    if let Some(mp2) = pipe.mp2.as_mut() {
                        pcm.extend(mp2.feed_frame(&frame).pcm);
                    }
                }
                DabCoding::Unsupported => {}
            }
        }
        for item in pad {
            match item {
                dabradio::pad::PadData::Dls(d) => {
                    let t = d.text.trim();
                    self.dls = (!t.is_empty()).then(|| t.to_string());
                }
                dabradio::pad::PadData::Mot(m) if !m.data.is_empty() => {
                    self.slide = Some(DabSlide {
                        content_type: m.content_type,
                        name: m.content_name,
                        data: m.data,
                    });
                }
                dabradio::pad::PadData::Mot(_) => {}
            }
        }
        pcm
    }

    /// The playing station's latest text (DLS), if it has sent one.
    pub fn dls(&self) -> Option<&str> {
        self.dls.as_deref()
    }

    /// The slideshow picture the station sent since the last call, if any.
    pub fn take_slide(&mut self) -> Option<DabSlide> {
        self.slide.take()
    }

    /// The shape of the PCM [`Self::take_pcm`] hands out: `(sample_rate_hz,
    /// channels)`, interleaved.
    ///
    /// A consumer has to know this to de-interleave, and getting it wrong is
    /// not subtle: treating stereo as mono plays the channels as consecutive
    /// samples, doubling the rate and garbling both. DAB+ AAC learns both from
    /// the AudioSpecificConfig, so before the first frame decodes there is
    /// nothing to report and 48 kHz stereo is the honest default (SBR doubled
    /// to 48 kHz is what DAB+ always ends at). MP2 in DAB is 48 kHz stereo.
    pub fn pcm_format(&self) -> (u32, usize) {
        match self.audio.as_ref() {
            Some(pipe) => match pipe.coding {
                DabCoding::DabPlus => pipe
                    .aac
                    .as_ref()
                    .map(|a| (a.sample_rate(), a.channels().max(1)))
                    .unwrap_or((48_000, 2)),
                DabCoding::MpegLayer2 => (48_000, 2),
                DabCoding::Unsupported => (48_000, 2),
            },
            None => (48_000, 2),
        }
    }

    /// Build the audio pipeline for `which`, if the FIC names it.
    fn start_audio(&mut self, which: &str) {
        use dabradio::fic::fib::AudioCoding;
        let target = parse_service_id(which);
        let service = match target {
            Some(sid) => self.ensemble.services.get(&sid),
            None => {
                let want = which.trim().to_lowercase();
                self.ensemble
                    .services
                    .values()
                    .find(|s| s.label.as_ref().is_some_and(|l| l.trim().to_lowercase() == want))
            }
        };
        let Some(service) = service else { return };
        let Some(subch_id) = service.subchannel_id else { return };
        let Some(subch) = self.ensemble.subchannels.get(&subch_id) else { return };
        let Some(coding) = service.audio_coding else { return };
        let Some(msc) = dabradio::msc::MscHandler::new(subch) else { return };
        let bitrate = subch.bitrate;
        let (coding, superframe, mp2) = match coding {
            AudioCoding::DabPlus => {
                (DabCoding::DabPlus, Some(dabradio::audio::SuperframeDecoder::new(bitrate)), None)
            }
            AudioCoding::MpegLayer2 => {
                (DabCoding::MpegLayer2, None, Some(dabradio::audio::mp2::Mp2Decoder::new()))
            }
            AudioCoding::Other(_) => (DabCoding::Unsupported, None, None),
        };
        self.audio = Some(AudioPipe {
            coding,
            msc,
            superframe,
            aac: None,
            mp2,
            pending: VecDeque::new(),
            pad: dabradio::pad::PadExtractor::new(),
            aac_configured: false,
        });
    }
}

/// Parse `0xF201` / `F201` into a service id number, or `None` for a label.
fn parse_service_id(s: &str) -> Option<u32> {
    let t = s.trim();
    let hex = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).unwrap_or(t);
    if hex.len() == 4 && hex.chars().all(|c| c.is_ascii_hexdigit()) {
        u32::from_str_radix(hex, 16).ok()
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// One FIB carrying a FIG 0/17 for two services — one plain, one with the
    /// language and complementary-code flags an older encoder sets — padded,
    /// as the FIC sends it.
    fn fib_with_pty() -> [u8; 32] {
        let body: [u8; 10] = [
            0xF2, 0x01, 0x00, 0x01, // F201: News
            0xF2, 0x03, 0x30, 0x0F, 0x18, 0x00, // F203: L+CC flags, lang 0x0F, Jazz, CC
        ];
        let mut fib = [0u8; 32];
        fib[0] = (1 + body.len()) as u8; // FIG type 0, length
        fib[1] = 17; // C/N 0, OE 0, P/D 0, extension 17
        fib[2..2 + body.len()].copy_from_slice(&body);
        fib[2 + body.len()] = 0xFF; // end marker
        fib
    }

    #[test]
    fn the_programme_type_is_read_from_fig_0_17() {
        let mut ptys = HashMap::new();
        parse_fig0_17(&fib_with_pty(), &mut ptys);
        assert_eq!(ptys.get(&0xF201).and_then(|c| pty_name(*c)), Some("News"));
        assert_eq!(ptys.get(&0xF203).and_then(|c| pty_name(*c)), Some("Jazz Music"));
        assert_eq!(ptys.len(), 2);
    }

    /// Another ensemble's services (OE = 1) are not this list's, and code 0
    /// names nothing.
    #[test]
    fn other_ensembles_and_code_zero_name_nothing() {
        let mut fib = fib_with_pty();
        fib[1] |= 0x40;
        let mut ptys = HashMap::new();
        parse_fig0_17(&fib, &mut ptys);
        assert!(ptys.is_empty());
        assert_eq!(pty_name(0), None);
        assert_eq!(pty_name(10), Some("Pop Music"));
    }

    /// The whole chain against a real capture, when one is on hand: sync, FIC,
    /// the service list, then DAB+ audio through our faad2 — the path this crate
    /// exists for. Ignored, because it needs a multi-hundred-megabyte off-air
    /// file; point `SDROXIDE_DAB_SAMPLE` at a raw cs16 capture and run with
    /// `--ignored`.
    #[test]
    #[ignore]
    fn an_off_air_capture_decodes_services_and_audio() {
        let (Ok(path), Ok(rate)) =
            (std::env::var("SDROXIDE_DAB_SAMPLE"), std::env::var("SDROXIDE_DAB_SAMPLE_RATE"))
        else {
            return;
        };
        let rate: f64 = rate.parse().expect("rate");
        let raw = std::fs::read(&path).expect("read capture");
        let iq: Vec<Complex32> = raw
            .chunks_exact(4)
            .map(|b| {
                Complex32::new(
                    i16::from_le_bytes([b[0], b[1]]) as f32 / 32768.0,
                    i16::from_le_bytes([b[2], b[3]]) as f32 / 32768.0,
                )
            })
            .collect();
        let mut rx = DabReceiver::new(rate).expect("receiver");
        let mut selected = false;
        let mut pcm = 0usize;
        for block in iq.chunks(32768) {
            rx.feed(block);
            if !selected && rx.has_services() {
                let e = rx.ensemble();
                assert!(!e.services.is_empty(), "the FIC named no services");
                // The first DAB+ service, so the faad2 path is exercised.
                rx.select_service(&e.services[0].service_id);
                selected = true;
            }
            pcm += rx.take_pcm().len();
        }
        assert!(selected, "no ensemble was decoded");
        eprintln!("decoded {pcm} PCM samples ({:.2}s)", pcm as f64 / 48000.0 / 2.0);
        assert!(pcm > 0, "the ensemble decoded but no service produced audio");
    }
}
