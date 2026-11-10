//! The protocol registry: what a decoded frame means, and which decoders to try
//! on a burst.
//!
//! A protocol is a channel, a physical layer, and a function from validated
//! bytes to readings. Adding one is adding a row to [`PROTOCOLS`] and a module
//! beside this one; nothing else in the crate has to know it exists.

use sdroxide_types::{IsmProtocol, IsmReading};

use crate::gate::Burst;
use crate::slice::Phy;

pub mod bresser;
pub mod fineoffset;
pub mod homematic;
pub mod lacrosse;
pub mod zwave;

/// What a protocol module makes of a frame.
pub struct Decoded {
    pub protocol: IsmProtocol,
    /// Model or variant, when the frame actually says. `None` rather than a
    /// guess — see the note in [`lacrosse::parse`].
    pub model: Option<String>,
    pub device: String,
    pub readings: Vec<IsmReading>,
    pub extra: Vec<(String, String)>,
    /// The frame validated but its payload is encrypted. Reached by the metering
    /// protocols; no sensor uses it.
    pub encrypted: bool,
    /// The validated frame as hex. Filled in by [`decode`], which is the layer
    /// that still has the bytes — a protocol module leaves it empty.
    pub raw_hex: String,
}

/// One entry in the registry.
pub struct Protocol {
    pub id: IsmProtocol,
    /// The channel this protocol lives on, matched against the burst's.
    pub channel_hz: f64,
    pub phy: Phy,
    /// Bytes after the sync word to readings, or `None` if they are not a frame
    /// of this protocol. Must validate — the slicer's candidates include noise.
    pub parse: fn(&[u8]) -> Option<Decoded>,
}

/// Every protocol, in the order they are tried.
pub const PROTOCOLS: &[Protocol] = &[
    Protocol {
        id: IsmProtocol::LaCrosseIt,
        channel_hz: lacrosse::CHANNEL_HZ,
        phy: lacrosse::PHY,
        parse: lacrosse::parse,
    },
    Protocol {
        id: IsmProtocol::FineOffset,
        channel_hz: fineoffset::CHANNEL_HZ,
        phy: fineoffset::PHY,
        parse: fineoffset::parse,
    },
    Protocol {
        id: IsmProtocol::Bresser,
        channel_hz: bresser::CHANNEL_HZ,
        phy: bresser::PHY,
        parse: bresser::parse,
    },
    Protocol {
        id: IsmProtocol::Homematic,
        channel_hz: homematic::CHANNEL_HZ,
        phy: homematic::PHY,
        parse: homematic::parse,
    },
    Protocol {
        id: IsmProtocol::ZWave,
        channel_hz: zwave::CHANNEL_HZ,
        phy: zwave::PHY,
        parse: zwave::parse,
    },
];

/// What a burst turned out to be, alongside the measurements taken from it.
pub struct Outcome {
    pub decoded: Vec<Decoded>,
    /// Absolute RF frequency the burst was actually on: the channel centre plus
    /// the carrier offset measured from the signal. Reporting the channel centre
    /// instead would put every device in the band on the same four frequencies.
    pub freq_hz: f64,
    pub snr_db: f32,
    /// Absolute peak power, for comparing two receivers' views of one burst.
    pub peak_dbfs: f32,
    /// Measured deviation, symbol rate and envelope swing, for the diagnostic
    /// dump. Not used to decide anything.
    pub dev_hz: f32,
    pub baud: f64,
    pub env_depth: f32,
    /// What kind of signal it is, decided from the burst alone.
    pub class: sdroxide_types::IsmBurstClass,
    /// The evidence behind [`Outcome::class`]: how much of the signal sat at a
    /// tone, and how far its quiet part stood above the channel's noise floor.
    pub bimodal: f32,
    pub floor_margin_db: f32,
    /// Candidate frames the slicer produced, whether or not any parsed. A burst
    /// with candidates but no decode is an unsupported device; one with no
    /// candidates at all is not this modulation.
    pub candidates: usize,
    /// Symbol rate measured from the burst itself with no protocol to guess from,
    /// and how well the measurement fitted. The number that says what to write
    /// next when nothing decoded.
    pub measured_baud: Option<(f64, f32)>,
    /// The burst sliced at its own measured rate, both bit senses, as hex.
    ///
    /// Computed always rather than behind a flag: it is one slicing pass against
    /// the thirty-two each protocol already does, and a burst nothing could read is
    /// exactly when somebody wants to look at the bits.
    pub bits_hex: String,
}

/// Furthest a burst's carrier may sit from its channel and still be treated as
/// that channel's traffic, in Hz.
///
/// The channels in the plan are as close as 120 kHz apart while their baseband
/// windows are two to five hundred kilohertz wide, so a transmission on one
/// arrives in its neighbour's downconverter as well. Without this guard the same
/// device would be attributed to two channels — and once a protocol exists on
/// each of them, decoded and reported twice.
///
/// 60 kHz at 868 MHz is 69 ppm, which no crystal-referenced transmitter is out
/// by: these are cheap references, but they are an order of magnitude better than
/// that. So the bound rejects a neighbour without ever rejecting a device that is
/// merely off frequency.
const MAX_CARRIER_OFFSET_HZ: f32 = 60_000.0;

/// Space-separated lowercase hex, which is how every write-up and capture file
/// for these protocols prints a frame — so a report can be pasted straight into a
/// search or a bug report.
pub fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ")
}

/// How well a burst's own symbol-rate measurement must have verified itself
/// before the rate is worth *printing*.
///
/// Higher than the 0.8 the slicers accept, and for a different reason: a slicer
/// that takes a poor rate estimate just fails to find a sync word and costs
/// nothing, whereas a poor estimate here becomes a figure in the operator's panel
/// claiming a symbol rate that was never really measured. Below this the row
/// simply does not quote one.
const UNIDENTIFIED_MIN_FIT: f32 = 0.9;

/// Alternating bits that must precede the derived sync word.
///
/// Three bytes of preamble. The point is to be sure this really is a framed
/// transmission before describing it as one: noise produces short alternating
/// runs constantly and a two-byte threshold would list a great deal of nothing.
const UNIDENTIFIED_PREAMBLE_BITS: usize = 24;

/// How finely an unidentified burst's frequency is rounded when it becomes that
/// burst's identity, in Hz.
///
/// Only used for bursts with no sync word to be known by. Coarse enough that one
/// transmitter's drift and the estimator's own scatter stay in one row, fine
/// enough that two devices a channel apart do not merge.
const UNIDENTIFIED_FREQ_BUCKET_HZ: f64 = 25_000.0;

/// Describe a burst that no decoder claimed.
///
/// The operator's question about an unread burst is "what is it, and is it always
/// there". Four things answer that without knowing the protocol: what kind of
/// signal it is, how wide, how fast, and — when it is framed — the bytes
/// immediately after its preamble, which for every protocol in this band are the
/// sync word and therefore identify the *protocol* even when nothing can read it.
///
/// # What a row is keyed on
///
/// The sync word where there is one, so repeats collapse into a single row with a
/// count rather than filling the panel. Two devices sharing a protocol merge,
/// which is the right answer at this level of knowledge: what is known is "this
/// protocol is here, this often".
///
/// A chirp or a bare carrier has no preamble and no sync word to be known by, so
/// those are keyed on their rounded frequency instead. That is a weaker identity
/// and it is why the class is named in the row: "chirp at 868.35" is a true and
/// useful thing to say, and it is all that can be said.
fn unidentified(
    class: sdroxide_types::IsmBurstClass,
    sync: Option<[u8; 2]>,
    stats: &crate::demod::Stats,
    measured_baud: Option<(f64, f32)>,
    burst: &Burst,
) -> Decoded {
    let freq = burst.center_hz + f64::from(stats.center_hz);
    let mut extra = vec![("dev".to_string(), format!("{:.1} kHz", stats.dev_hz / 1e3))];
    match measured_baud.filter(|(_, fit)| *fit >= UNIDENTIFIED_MIN_FIT) {
        Some((b, _)) => extra.push(("rate".to_string(), format!("{:.1} kbaud", b / 1000.0))),
        // Said out loud rather than left blank: for a chirp there is no symbol
        // rate to find, and "none" is a finding rather than a gap.
        None => extra.push(("rate".to_string(), "not measurable".to_string())),
    }

    Decoded {
        protocol: IsmProtocol::Unidentified,
        model: Some(class.label().to_string()),
        device: match sync {
            Some(s) => format!("sync {:02x}{:02x}", s[0], s[1]),
            None => format!(
                "{:.3} MHz",
                (freq / UNIDENTIFIED_FREQ_BUCKET_HZ).round() * UNIDENTIFIED_FREQ_BUCKET_HZ / 1e6
            ),
        },
        readings: Vec::new(),
        extra,
        encrypted: false,
        raw_hex: String::new(),
    }
}

/// The two bytes after a burst's preamble, when it has one.
fn signature(bits: &[u8]) -> Option<[u8; 2]> {
    // The end of the first run of alternating bits long enough to be a preamble.
    let mut i = 0usize;
    let (start, mut end) = loop {
        let mut j = i + 1;
        while j < bits.len() && bits[j] != bits[j - 1] {
            j += 1;
        }
        if j - i >= UNIDENTIFIED_PREAMBLE_BITS {
            break (i, j);
        }
        if j >= bits.len() {
            return None;
        }
        i = j;
    };

    // Where the preamble stops is ambiguous by exactly one bit, and the ambiguity
    // is not academic — it is the difference between reading a sync word as
    // `d391` and as `a723`.
    //
    // The run ends at the first bit equal to its predecessor. If the sync word's
    // own first bit happens to continue the alternation, that pair is (sync bit
    // 0, sync bit 1) and the sync started one bit earlier; if it breaks the
    // alternation, the pair is (last preamble bit, sync bit 0) and it starts
    // here. One burst cannot tell those apart.
    //
    // What settles it is that these preambles are a whole number of bytes: the
    // offset that lands on a byte boundary measured from the start of the run is
    // the transmitter's. Both real signatures in this band — Homematic's `e9ca`
    // and the `2c4c` emitter — come out right under that rule and wrong without
    // it.
    if end > start && (end - start) % 8 != 0 && (end - 1 - start) % 8 == 0 {
        end -= 1;
    }
    if end + 16 > bits.len() {
        return None;
    }
    let sync = crate::slice::pack_bits(&bits[end..end + 16]);
    Some([sync[0], sync[1]])
}

/// Scratch buffers, so a channel's decoder does not allocate per burst.
#[derive(Default)]
pub struct Scratch {
    disc: Vec<f32>,
    pick: Vec<f32>,
}

/// Try every enabled protocol on a burst.
///
/// `enabled` decides which registry entries run; the caller maps the operator's
/// family switches onto it.
///
/// `require_channel` asks whether a protocol may only be tried on bursts from its
/// own channel. True for the engine, where a burst arriving on 868.95 MHz cannot
/// have come from a sensor that only transmits on 868.30 and trying anyway is
/// wasted work. False for a survey, whose receivers sit on a grid of its own
/// choosing rather than on the plan — there, "it decoded as a LaCrosse" is the most
/// useful identification a burst can be given, and refusing to try because the
/// tile centre is 54 kHz off would throw it away.
pub fn decode(
    burst: &Burst,
    scratch: &mut Scratch,
    enabled: &dyn Fn(IsmProtocol) -> bool,
    require_channel: bool,
) -> Outcome {
    crate::demod::discriminate(&burst.iq, burst.rate_hz, &mut scratch.disc);
    let stats = crate::demod::stats(&burst.iq, &scratch.disc, &mut scratch.pick);

    // Only the part of the burst that is signal; see the note at the slicers below.
    let body = &scratch.disc[stats.signal.clone()];
    let measured_baud = crate::demod::estimate_baud_free(body, stats.center_hz, burst.rate_hz);
    // Kept rather than formatted and dropped: the same slice is what an
    // unidentified burst is described by, further down.
    let sliced = measured_baud
        .map(|(b, _)| crate::slice::slice_at(body, stats.center_hz, burst.rate_hz / b));
    let bits_hex = match &sliced {
        Some(bits) => {
            // Enough for a whole burst rather than just its header. The longest
            // frame this crate decodes is a Bresser 7-in-1 at twenty-five payload
            // bytes, but the dump exists for the bursts nothing decodes — and a
            // 30 ms transmission at 20 kbaud is seventy-five bytes, so a cap set
            // by what already decodes would truncate exactly the traffic it is
            // there to identify.
            let take = bits.len().min(96 * 8);
            let bytes = crate::slice::pack_bits(&bits[..take]);
            let inverted: Vec<u8> = bytes.iter().map(|x| !*x).collect();
            format!("bits {}\n      inv  {}", hex(&bytes), hex(&inverted))
        }
        None => String::new(),
    };

    let mut decoded = Vec::new();
    let mut candidates = 0usize;
    let mut baud = 0.0f64;

    // A burst sitting far off the channel centre is a neighbouring channel's
    // transmission leaking through this one's filter, not this one's traffic.
    let on_channel = stats.center_hz.abs() <= MAX_CARRIER_OFFSET_HZ;

    for p in PROTOCOLS {
        if !on_channel {
            break;
        }
        if !enabled(p.id) {
            continue;
        }
        // The channel a burst arrived on decides which protocols can possibly
        // have sent it. Comparing to within a tenth of the channel spacing
        // rather than exactly keeps this robust to a plan edited in Hz.
        if require_channel && (p.channel_hz - burst.center_hz).abs() > 10_000.0 {
            continue;
        }

        // The slicing level is the *measured* carrier, not zero: these
        // transmitters are tens of kilohertz off channel often enough that
        // slicing about zero would read every bit the same.
        // The signal region only: the padding either side is noise, and over noise
        // the discriminator wanders across the whole channel — enough to defeat the
        // symbol-period measurement, which looks at the run-up and would otherwise
        // be reading the pre-trigger ring.
        let frames = crate::slice::frames(body, stats.center_hz, burst.rate_hz, &p.phy);
        candidates += frames.len();

        for f in &frames {
            if let Some(mut d) = (p.parse)(&f.bytes) {
                d.raw_hex = hex(&f.bytes);
                baud = burst.rate_hz / f.sps;
                decoded.push(d);
                // One device per protocol per burst. Later candidates are other
                // slicing phases reading the same transmission.
                break;
            }
        }
    }

    // Nothing read it. If the operator asked to see those, say what could be
    // measured about it rather than dropping it — see [`unidentified`].
    if decoded.is_empty() && on_channel && enabled(IsmProtocol::Unidentified) {
        let class = crate::class::classify(burst, &stats, measured_baud);
        // A keyed burst's rate lives in its envelope, not in its frequency — the
        // discriminator estimator reads noise over the silent half. See
        // [`crate::demod::estimate_ook_baud`].
        let rate = if class == sdroxide_types::IsmBurstClass::Ook {
            let mag: Vec<f32> = burst.iq[stats.signal.clone()].iter().map(|z| z.norm()).collect();
            crate::demod::estimate_ook_baud(&mag, burst.rate_hz)
        } else {
            measured_baud
        };
        let sync = sliced.as_deref().and_then(signature);
        let mut d = unidentified(class, sync, &stats, rate, burst);
        if let Some(bits) = &sliced {
            d.raw_hex = hex(&crate::slice::pack_bits(&bits[..bits.len().min(32 * 8)]));
        }
        decoded.push(d);
    }

    Outcome {
        decoded,
        freq_hz: burst.center_hz + f64::from(stats.center_hz),
        snr_db: burst.snr_db,
        peak_dbfs: burst.peak_dbfs,
        dev_hz: stats.dev_hz,
        baud,
        env_depth: stats.env_depth,
        class: crate::class::classify(burst, &stats, measured_baud),
        bimodal: stats.bimodal,
        floor_margin_db: stats.env_low_dbfs - (burst.peak_dbfs - burst.snr_db),
        candidates,
        measured_baud,
        bits_hex,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every registry entry has to sit on a channel the plan actually opens, or
    /// its bursts are never delivered to it and it silently never runs.
    #[test]
    fn every_protocol_is_on_a_channel_in_the_plan() {
        for p in PROTOCOLS {
            let ch = crate::plan::CHANNELS
                .iter()
                .find(|c| (c.center_hz - p.channel_hz).abs() < 10_000.0)
                .unwrap_or_else(|| {
                    panic!("{:?} is on {} Hz, which is not in the plan", p.id, p.channel_hz)
                });
            // And that channel has to be wide enough to hold the signal.
            assert!(
                ch.rate_hz >= p.phy.baud * 4.0,
                "{:?} needs more than {} Hz to be demodulated",
                p.id,
                ch.rate_hz
            );
        }
    }

    /// A protocol whose family is switched off must not be tried at all.
    #[test]
    fn a_disabled_protocol_is_not_tried() {
        let burst = Burst {
            iq: vec![sdroxide_dsp::Complex32::new(0.1, 0.0); 4096],
            rate_hz: 250_000.0,
            center_hz: 868_300_000.0,
            snr_db: 20.0,
            peak_dbfs: -20.0,
        };
        let mut scratch = Scratch::default();
        let out = decode(&burst, &mut scratch, &|_| false, true);
        assert!(out.decoded.is_empty());
        assert_eq!(out.candidates, 0, "a disabled protocol still ran its slicer");
    }

    /// A burst from the wrong channel must not be offered to a protocol that
    /// does not live there.
    #[test]
    fn a_burst_from_another_channel_is_not_offered() {
        let burst = Burst {
            iq: vec![sdroxide_dsp::Complex32::new(0.1, 0.0); 4096],
            rate_hz: 500_000.0,
            center_hz: 868_950_000.0,
            snr_db: 20.0,
            peak_dbfs: -20.0,
        };
        let mut scratch = Scratch::default();
        let out = decode(&burst, &mut scratch, &|_| true, true);
        assert_eq!(out.candidates, 0);
    }
}
