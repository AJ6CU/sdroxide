//! Per-burst diagnostics, for characterising a band rather than decoding it.
//!
//! The device table the engine reports is the right answer for an operator and
//! the wrong one for finding out why nothing is being decoded. This is the other
//! view: every burst the gate opens on, with what was measured from it and
//! whether anything could read it — including the bursts nothing could.
//!
//! It exists because the honest answer to "does the decoder work on your band"
//! is a capture from that band, and because the first thing anyone will want to
//! know is what the *other* transmissions in the band are. A burst reported here
//! with a plausible symbol rate, a sane deviation and a handful of candidate
//! frames is a device worth writing a protocol module for; one with no candidates
//! at all is a modulation this chain does not handle.
//!
//! Unlike the engine's decoder this opens **every** channel in the plan whether
//! or not a protocol is implemented on it, because seeing that 868.95 MHz is busy
//! is exactly the point.

use sdroxide_dsp::{Complex32, Ddc};
use sdroxide_types::{IsmProtocol, IsmReading};

use crate::gate::{Burst, Gate};
use crate::plan;
use crate::proto::{self, Scratch};

/// What one burst looked like.
pub struct BurstReport {
    /// Channel the burst arrived on.
    pub channel_hz: f64,
    /// Measured RF frequency: the channel plus the carrier offset in the signal.
    pub freq_hz: f64,
    pub snr_db: f32,
    /// Absolute peak power, dBFS — see [`crate::Burst::peak_dbfs`].
    pub peak_dbfs: f32,
    /// Length in milliseconds.
    pub len_ms: f64,
    /// Measured FSK deviation, Hz. Meaningless for a burst that is not FSK, which
    /// is itself informative — a wild figure here says "not two-tone".
    pub dev_hz: f32,
    /// Symbol rate recovered from the run-up, or 0 when no protocol read it.
    pub baud: f64,
    /// Fractional envelope swing; see [`crate::demod::Stats::env_depth`].
    pub env_depth: f32,
    /// What kind of signal the burst is; see [`crate::class`].
    pub class: sdroxide_types::IsmBurstClass,
    /// The two numbers the class was decided on, so a survey line shows the
    /// evidence and not just the verdict.
    pub bimodal: f32,
    /// How far the quiet part of the envelope sits above the channel's floor, dB.
    pub floor_margin_db: f32,
    /// Frames the slicers pulled out, whether or not any validated.
    pub candidates: usize,
    /// Symbol rate measured from the burst itself, with no protocol to guess from,
    /// and the fraction of crossing intervals that agreed with it. `None` when the
    /// burst does not hold still enough to measure — see
    /// [`crate::demod::estimate_baud_free`].
    pub measured_baud: Option<(f64, f32)>,
    /// The burst sliced at its measured rate, as hex, so the framing can be read
    /// by eye. Empty unless a bit dump was asked for.
    pub bits_hex: String,
    /// Anything that decoded: protocol, model, device, readings, the non-numeric
    /// extras (battery state and the like), and the raw frame.
    ///
    /// The model is carried because for an unidentified burst it is the *class* —
    /// "2-FSK", "chirp" — which is the whole of what such a report says, and a
    /// diagnostic tool that dropped it would print every unread burst as the same
    /// anonymous line.
    pub decoded:
        Vec<(IsmProtocol, Option<String>, String, Vec<IsmReading>, Vec<(String, String)>, String)>,
}

impl BurstReport {
    /// One line, for a replay tool's log.
    pub fn line(&self) -> String {
        // The channel as well as the measured frequency: one transmission
        // arrives in more than one channel's downconverter when the two are
        // closer together than their baseband windows are wide, and without the
        // channel printed the duplicate reads as a second device.
        let mut s = format!(
            "ch {:7.3}  {:9.4} MHz  {:5.1} dB  {:6.2} ms  dev {:6.1} kHz  env {:4.2}  cand {:2}",
            self.channel_hz / 1e6,
            self.freq_hz / 1e6,
            self.snr_db,
            self.len_ms,
            self.dev_hz / 1e3,
            self.env_depth,
            self.candidates,
        );
        s.push_str(&format!(
            "  {:>12}  bim {:4.2}  floor +{:5.1} dB",
            self.class.label(),
            self.bimodal,
            self.floor_margin_db
        ));
        if let Some((baud, fit)) = self.measured_baud {
            s.push_str(&format!("  sym {baud:6.0} ({fit:.2})"));
        } else {
            s.push_str("  sym      ?      ");
        }
        if self.baud > 0.0 {
            s.push_str(&format!("  baud {:6.0}", self.baud));
        }
        if !self.bits_hex.is_empty() {
            s.push_str(&format!("\n      {}", self.bits_hex));
        }
        for (p, model, dev, readings, extra, raw) in &self.decoded {
            let mut vals: Vec<String> = readings.iter().map(|r| r.fmt_labelled()).collect();
            // The extras carry the battery state, which is the one thing an
            // operator wants from a sensor besides its reading.
            vals.extend(extra.iter().map(|(k, v)| format!("{k} {v}")));
            let kind = match model {
                Some(m) => format!("{} {m}", p.label()),
                None => p.label().to_string(),
            };
            s.push_str(&format!("\n    {kind} id {dev}  {}  [{raw}]", vals.join("  ")));
        }
        s
    }
}

/// One channel of the probe.
struct Chan {
    ddc: Ddc,
    gate: Gate,
    channel_hz: f64,
    buf: Vec<Complex32>,
}

/// Runs the whole chain over IQ and reports every burst.
pub struct Probe {
    chans: Vec<Chan>,
    scratch: Scratch,
    bursts: Vec<Burst>,
}

impl Probe {
    /// `window_center_hz` and `window_rate_hz` describe the IQ that will be fed.
    /// Channels outside that window are skipped, as they are in the engine.
    pub fn new(window_center_hz: f64, window_rate_hz: f64, threshold_db: f32) -> Probe {
        let mut chans = Vec::new();
        for ch in plan::CHANNELS {
            if !plan::fits(ch, window_center_hz, window_rate_hz) {
                continue;
            }
            let mut ddc = Ddc::new(window_rate_hz, ch.rate_hz);
            ddc.set_offset_hz(ch.center_hz - window_center_hz);
            let rate = ddc.out_rate();
            chans.push(Chan {
                ddc,
                gate: Gate::new(rate, ch.center_hz, threshold_db),
                channel_hz: ch.center_hz,
                buf: Vec::new(),
            });
        }
        Probe { chans, scratch: Scratch::default(), bursts: Vec::new() }
    }

    /// Channels this probe opened, for a tool to print before it starts.
    pub fn channels(&self) -> Vec<f64> {
        self.chans.iter().map(|c| c.channel_hz).collect()
    }

    /// Noise floor each channel has settled on, dBFS.
    pub fn floors_dbfs(&self) -> Vec<(f64, f32)> {
        self.chans.iter().map(|c| (c.channel_hz, c.gate.floor_dbfs())).collect()
    }

    /// Feed window-rate IQ; append a report for every completed burst.
    pub fn push(&mut self, iq: &[Complex32], out: &mut Vec<BurstReport>) {
        let mut bursts = std::mem::take(&mut self.bursts);
        for i in 0..self.chans.len() {
            bursts.clear();
            {
                let c = &mut self.chans[i];
                c.buf.clear();
                c.ddc.process(iq, &mut c.buf);
                let Chan { gate, buf, .. } = c;
                gate.push(buf, &mut bursts);
            }
            let channel_hz = self.chans[i].channel_hz;
            for b in &bursts {
                // Every protocol, so a burst is measured against everything this
                // build knows how to try.
                let o = proto::decode(b, &mut self.scratch, &|_| true, true);
                out.push(BurstReport {
                    channel_hz,
                    freq_hz: o.freq_hz,
                    snr_db: o.snr_db,
                    peak_dbfs: o.peak_dbfs,
                    len_ms: b.iq.len() as f64 / b.rate_hz * 1e3,
                    dev_hz: o.dev_hz,
                    baud: o.baud,
                    env_depth: o.env_depth,
                    class: o.class,
                    bimodal: o.bimodal,
                    floor_margin_db: o.floor_margin_db,
                    candidates: o.candidates,
                    measured_baud: o.measured_baud,
                    bits_hex: o.bits_hex.clone(),
                    decoded: o
                        .decoded
                        .into_iter()
                        .map(|d| (d.protocol, d.model, d.device, d.readings, d.extra, d.raw_hex))
                        .collect(),
                });
            }
        }
        self.bursts = bursts;
    }
}

/// Tile spacing for [`Survey`], in Hz.
///
/// The tiles have to overlap enough that no burst falls between two of them: a
/// signal more than half a spacing from the nearest tile centre is measured
/// through the skirt of that tile's filter, and its deviation and symbol rate
/// come back wrong. 100 kHz spacing puts every burst within 50 kHz of a centre.
const TILE_SPACING_HZ: f64 = 100_000.0;

/// Default bandwidth of each survey tile. Wide enough for the widest thing in the
/// band — wireless M-Bus mode T runs ±50 kHz at 100 kcps — so a burst is measured
/// whole rather than clipped.
///
/// Worth *narrowing* when hunting a narrow signal, though, and the reason is the
/// noise: a 13 kHz-deviation sensor occupying maybe 40 kHz, received in a 250 kHz
/// tile, is competing with six times the noise it needs to. Dropping the tile to
/// 50 kHz buys about 7 dB, which is the difference between a symbol rate that can
/// be measured and one that cannot. The cost is a wideband signal — LoRa, M-Bus
/// mode T — being clipped and mis-measured instead, so neither width sees
/// everything and both are worth a pass.
pub const TILE_RATE_DEFAULT_HZ: f64 = 250_000.0;

/// Where the transmissions in a band actually are, as against where the channel
/// plan expects them.
///
/// The channel decoders answer "is one of these four protocols talking". This
/// answers the question that comes first and that the plan cannot: *what is on
/// this band at all*. It tiles the whole window with overlapping receivers and
/// reports every burst any of them sees, so a neighbourhood whose traffic sits at
/// 866.5 MHz — the RFID part of the band, which no sensor protocol uses — shows up
/// as exactly that rather than as silence.
pub struct Survey {
    chans: Vec<Chan>,
    scratch: Scratch,
    bursts: Vec<Burst>,
    /// How far apart two tiles can be and still be hearing one transmission.
    ///
    /// Keyed on tile centres rather than on the reported frequencies, which is the
    /// whole point: a tile that hears a signal through its filter skirt measures the
    /// carrier offset badly — tens of kilohertz out — so the frequencies reported by
    /// the several tiles that heard one burst scatter by more than any sane
    /// threshold, and deduping on them keeps all of them.
    ///
    /// Two and a half spacings, because a tile's anti-alias filter is not a brick
    /// wall and a signal stays audible a good way past the nominal edge. The cost is
    /// that two genuinely simultaneous transmissions closer than that are reported as
    /// one; for a survey, whose job is to say where the traffic *is*, that beats
    /// reporting one transmission twice at two different frequencies, which would be
    /// a lie about the band.
    dedupe_hz: f64,
}

impl Survey {
    pub fn new(window_center_hz: f64, window_rate_hz: f64, threshold_db: f32) -> Survey {
        Survey::with_tile(window_center_hz, window_rate_hz, threshold_db, TILE_RATE_DEFAULT_HZ)
    }

    /// A survey at a chosen tile bandwidth — see [`TILE_RATE_DEFAULT_HZ`] for why
    /// that is a knob and not a constant.
    ///
    /// The tile spacing follows the bandwidth so the tiles keep overlapping: a
    /// narrow tile grid has to be finer, or bursts fall between the tiles and are
    /// measured through a filter skirt.
    pub fn with_tile(
        window_center_hz: f64,
        window_rate_hz: f64,
        threshold_db: f32,
        tile_rate_hz: f64,
    ) -> Survey {
        let spacing = (tile_rate_hz * TILE_SPACING_HZ / TILE_RATE_DEFAULT_HZ).max(10_000.0);
        let usable = window_rate_hz * plan::USABLE_FRACTION;
        let half = (usable - tile_rate_hz).max(0.0) / 2.0;
        let n = ((half * 2.0 / spacing).floor() as i64).max(0);

        let mut chans = Vec::new();
        for k in 0..=n {
            let center = window_center_hz - half + k as f64 * spacing;
            let mut ddc = Ddc::new(window_rate_hz, tile_rate_hz);
            ddc.set_offset_hz(center - window_center_hz);
            let rate = ddc.out_rate();
            chans.push(Chan {
                ddc,
                gate: Gate::new(rate, center, threshold_db),
                channel_hz: center,
                buf: Vec::new(),
            });
        }
        Survey { chans, scratch: Scratch::default(), bursts: Vec::new(), dedupe_hz: spacing * 2.5 }
    }

    /// Tile centres, for a tool to print before it starts.
    pub fn tiles(&self) -> Vec<f64> {
        self.chans.iter().map(|c| c.channel_hz).collect()
    }

    /// Feed window-rate IQ; append a report per distinct transmission.
    pub fn push(&mut self, iq: &[Complex32], out: &mut Vec<BurstReport>) {
        let first = out.len();
        let mut bursts = std::mem::take(&mut self.bursts);
        for i in 0..self.chans.len() {
            bursts.clear();
            {
                let c = &mut self.chans[i];
                c.buf.clear();
                c.ddc.process(iq, &mut c.buf);
                let Chan { gate, buf, .. } = c;
                gate.push(buf, &mut bursts);
            }
            let channel_hz = self.chans[i].channel_hz;
            for b in &bursts {
                // Off the plan's grid by construction, so the channel match is
                // not applied — see [`proto::decode`].
                let o = proto::decode(b, &mut self.scratch, &|_| true, false);
                let rep = BurstReport {
                    channel_hz,
                    freq_hz: o.freq_hz,
                    snr_db: o.snr_db,
                    peak_dbfs: o.peak_dbfs,
                    len_ms: b.iq.len() as f64 / b.rate_hz * 1e3,
                    dev_hz: o.dev_hz,
                    baud: o.baud,
                    env_depth: o.env_depth,
                    class: o.class,
                    bimodal: o.bimodal,
                    floor_margin_db: o.floor_margin_db,
                    candidates: o.candidates,
                    measured_baud: o.measured_baud,
                    bits_hex: o.bits_hex.clone(),
                    decoded: o
                        .decoded
                        .into_iter()
                        .map(|d| (d.protocol, d.model, d.device, d.readings, d.extra, d.raw_hex))
                        .collect(),
                };
                // The same transmission reaches several tiles; keep the one that
                // heard it best, by **SNR**.
                //
                // Not by which tile reports the signal closest to its own centre,
                // which is the obvious rule and is circular: the reported offset
                // is a measurement, and a tile hearing the signal through its
                // filter skirt measures it badly — often as near zero. That rule
                // therefore selects precisely the tile whose measurement is wrong.
                // And by *absolute* power, not SNR: each tile learns its own
                // noise floor, so their SNRs are referred to different things and
                // the higher one need not belong to the tile that heard the signal
                // better. Absolute peak power is the same quantity in every tile,
                // and the tile whose filter passes the signal most fully has the
                // most of it.
                match out[first..]
                    .iter()
                    .position(|p| (p.channel_hz - rep.channel_hz).abs() <= self.dedupe_hz)
                {
                    Some(k) => {
                        if rep.peak_dbfs > out[first + k].peak_dbfs {
                            out[first + k] = rep;
                        }
                    }
                    None => out.push(rep),
                }
            }
        }
        self.bursts = bursts;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The probe has to see a burst that the engine's decoder would see, on a
    /// channel the engine would not even open — that is the point of it.
    #[test]
    fn the_probe_opens_every_channel_in_the_window() {
        let p = Probe::new(plan::ideal_center_hz(), 2_025_000.0, 12.0);
        assert_eq!(
            p.channels().len(),
            plan::CHANNELS.len(),
            "the probe should not care which protocols are implemented"
        );
    }

    #[test]
    fn a_window_off_the_band_opens_nothing() {
        let p = Probe::new(866_000_000.0, 2_025_000.0, 12.0);
        assert!(p.channels().is_empty());
    }

    /// The survey has to tile the *whole* usable window, wherever it is tuned —
    /// including a window nowhere near the channel plan, which is the case it
    /// exists for.
    #[test]
    fn the_survey_tiles_the_window_wherever_it_is_pointed() {
        for center in [866_000_000.0, plan::ideal_center_hz()] {
            let s = Survey::new(center, 2_025_000.0, 12.0);
            let tiles = s.tiles();
            assert!(tiles.len() > 8, "only {} tiles for a 1.5 MHz window", tiles.len());

            // Every tile inside the window, and the outermost ones reaching its
            // edges to within a spacing.
            let half = 2_025_000.0 * plan::USABLE_FRACTION / 2.0;
            for t in &tiles {
                assert!((t - center).abs() <= half, "tile {t} is outside the window");
            }
            let lo = tiles.first().unwrap();
            let hi = tiles.last().unwrap();
            assert!(hi - lo > 2.0 * (half - TILE_RATE_DEFAULT_HZ) - TILE_SPACING_HZ);

            // And no gap a burst could hide in.
            for w in tiles.windows(2) {
                assert!((w[1] - w[0] - TILE_SPACING_HZ).abs() < 1.0);
            }
        }
    }

    /// One transmission must be reported once, not once per tile that heard it.
    #[test]
    fn the_survey_reports_a_transmission_once() {
        const RATE: f64 = 2_025_000.0;
        let center = 866_500_000.0;
        let mut sv = Survey::new(center, RATE, 12.0);

        let mut seed = 0x1234_5678_9ABC_DEF0u64;
        let mut noise = |sigma: f32| {
            let mut u = || {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                ((seed >> 11) as f64 / (1u64 << 53) as f64) as f32
            };
            let (u1, u2) = (u().max(1e-9), u());
            let r = (-2.0 * u1.ln()).sqrt() * sigma;
            Complex32::new(
                r * (std::f32::consts::TAU * u2).cos(),
                r * (std::f32::consts::TAU * u2).sin(),
            )
        };

        let mut out = Vec::new();
        let mut quiet = vec![Complex32::default(); 16_384];
        for _ in 0..16 {
            for z in quiet.iter_mut() {
                *z = noise(0.002);
            }
            sv.push(&quiet, &mut out);
        }
        assert!(out.is_empty(), "noise produced {} bursts", out.len());

        // A 4 ms tone 300 kHz above the window centre, deliberately not on any
        // channel in the plan.
        let want = center + 300_000.0;
        let len = (0.004 * RATE) as usize;
        let mut burst = vec![Complex32::default(); 40_000];
        let mut phase = 0.0f64;
        for (i, z) in burst.iter_mut().enumerate() {
            *z = noise(0.002);
            if i >= 4_000 && i < 4_000 + len {
                phase += std::f64::consts::TAU * 300_000.0 / RATE;
                *z += Complex32::new(0.05 * phase.cos() as f32, 0.05 * phase.sin() as f32);
            }
        }
        sv.push(&burst, &mut out);
        for _ in 0..4 {
            for z in quiet.iter_mut() {
                *z = noise(0.002);
            }
            sv.push(&quiet, &mut out);
        }

        assert_eq!(out.len(), 1, "one transmission became {} reports", out.len());
        assert!(
            (out[0].freq_hz - want).abs() < 20_000.0,
            "reported {:.4} MHz for a tone on {:.4}",
            out[0].freq_hz / 1e6,
            want / 1e6
        );
        // Nothing *read* it — a bare tone carries no frame. It is still reported,
        // classified for what it is, which is the whole point of the
        // unidentified path: a tone is a finding, not a gap.
        assert_eq!(out[0].decoded.len(), 1, "a tone should be reported, unread");
        let (proto, model, ..) = &out[0].decoded[0];
        assert_eq!(*proto, IsmProtocol::Unidentified);
        assert_eq!(model.as_deref(), Some("carrier"), "an unmodulated tone is a carrier");
    }
}
