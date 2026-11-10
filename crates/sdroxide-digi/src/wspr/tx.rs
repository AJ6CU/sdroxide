//! WSPR transmitter: 50 information bits → 162 tones → shaped audio.
//!
//! `mfsk_core::wspr::tx` exists and is not used here, for one reason it states
//! about itself: it applies no shaping at all. It steps the frequency between
//! symbols and it starts and stops the burst at full amplitude. On the air that
//! last part is a key click at each end of a 110-second transmission, and a
//! click is broadband — in a 200 Hz window shared with a hundred other beacons,
//! all of them decoding signals ten dB under the noise.
//!
//! So the synthesis is done here, with:
//!
//! * a **raised-cosine envelope** over the first and last [`RAMP_S`] seconds,
//!   which is what removes the clicks, and
//! * a **raised-cosine frequency transition** across each symbol boundary,
//!   which pulls the modulation sidelobes down without moving any symbol's
//!   steady-state tone — all the receiver's per-symbol integration looks at.
//!
//! Phase is continuous throughout, as WSPR requires.

use mfsk_core::wspr;

/// Symbol period in seconds: 8192 samples at 12 kHz.
const SYMBOL_DT: f64 = 8192.0 / 12_000.0;
/// Tone spacing in Hz, which for WSPR equals the symbol rate.
const TONE_SPACING_HZ: f64 = 12_000.0 / 8192.0;
/// Symbols in a transmission.
const N_SYMBOLS: usize = 162;

/// Raised-cosine envelope ramp at each end of the burst, in seconds.
///
/// Long enough that the click is gone, short enough to cost no symbol: 20 ms is
/// three per cent of one 683 ms symbol.
const RAMP_S: f64 = 0.020;

/// Width of the frequency crossfade at a symbol boundary, as a fraction of a
/// symbol, split evenly either side of it.
///
/// A quarter of a symbol — 171 ms — measured against the alternative in
/// `shaping_pulls_the_sidelobes_down_without_breaking_the_decode`. Wider would
/// trim the sidelobes further and start eating into the time each tone actually
/// spends at its nominal frequency, which is the receiver's entire signal.
const TRANSITION_SYMBOLS: f64 = 0.25;

/// Samples one transmission occupies at `sample_rate`.
pub fn burst_len(sample_rate: u32) -> usize {
    nsps(sample_rate) * N_SYMBOLS
}

fn nsps(sample_rate: u32) -> usize {
    (sample_rate as f64 * SYMBOL_DT).round() as usize
}

/// Pack, encode and synthesise in one step.
///
/// `None` when the message will not fit the Type-1 layout — a compound
/// callsign, or a grid that is not four characters. `mfsk-core` exposes no
/// packer for the other two layouts, so this station transmits Type 1 only;
/// it decodes all three.
pub fn synthesize(
    callsign: &str,
    grid: &str,
    power_dbm: i32,
    sample_rate: u32,
    base_freq_hz: f64,
    amplitude: f32,
) -> Option<Vec<f32>> {
    let symbols = symbols_for(callsign, grid, power_dbm)?;
    Some(synthesize_symbols(&symbols, sample_rate, base_freq_hz, amplitude))
}

/// A transmission, generated as it goes out.
///
/// Streaming rather than a buffer, because the buffer would be a big one: 110.6
/// seconds at 48 kHz is 5.3 million samples, twenty-one megabytes allocated,
/// filled and dropped for every transmission. The synthesis is a per-sample
/// loop either way, so metering it out costs nothing but this struct.
pub struct BurstGen {
    /// Tone frequency per symbol, resolved once at construction.
    tone: Vec<f64>,
    rate: f64,
    nsps: usize,
    total: usize,
    ramp: usize,
    amplitude: f32,
    pos: usize,
    phase: f64,
}

impl BurstGen {
    /// `base_freq_hz` is tone 0; the four tones run up from it in
    /// [`TONE_SPACING_HZ`] steps, so the signal's centre — what gets reported
    /// as the frequency — is 1.5 spacings above it.
    pub fn new(
        symbols: &[u8; N_SYMBOLS],
        sample_rate: u32,
        base_freq_hz: f64,
        amplitude: f32,
    ) -> Self {
        let nsps = nsps(sample_rate);
        let total = nsps * N_SYMBOLS;
        let rate = sample_rate as f64;
        BurstGen {
            tone: symbols
                .iter()
                .map(|&s| base_freq_hz + (s % 4) as f64 * TONE_SPACING_HZ)
                .collect(),
            rate,
            nsps,
            total,
            ramp: ((RAMP_S * rate).round() as usize).min(total / 2),
            amplitude,
            pos: 0,
            phase: 0.0,
        }
    }

    /// Samples still to come.
    pub fn remaining(&self) -> usize {
        self.total.saturating_sub(self.pos)
    }

    /// Fill `out` with the next block, zero-padding past the end. Returns true
    /// once the whole transmission has been written.
    pub fn fill(&mut self, out: &mut [f32]) -> bool {
        for slot in out.iter_mut() {
            if self.pos >= self.total {
                *slot = 0.0;
                continue;
            }
            *slot = self.next_sample();
        }
        self.pos >= self.total
    }

    fn next_sample(&mut self) -> f32 {
        let n = self.pos;
        // Position along the symbol axis, in symbols. The nearest integer is
        // the boundary this sample is closest to.
        let s = n as f64 / self.nsps as f64;
        let b = s.round();
        let d = s - b;
        let half = TRANSITION_SYMBOLS / 2.0;
        let f = if d.abs() >= half || b <= 0.0 || b >= N_SYMBOLS as f64 {
            // Away from a boundary — or at the very start or end, where there
            // is no neighbouring symbol to cross-fade with.
            self.tone[(s as usize).min(N_SYMBOLS - 1)]
        } else {
            let left = self.tone[b as usize - 1];
            let right = self.tone[b as usize];
            // Raised cosine: 0 at −half, 1 at +half, flat-tangent at both.
            let a = 0.5 * (1.0 + (std::f64::consts::PI * d / (2.0 * half)).sin());
            left * (1.0 - a) + right * a
        };

        let env = if n < self.ramp {
            0.5 * (1.0 - (std::f64::consts::PI * n as f64 / self.ramp as f64).cos())
        } else if n >= self.total - self.ramp {
            0.5 * (1.0
                - (std::f64::consts::PI * (self.total - 1 - n) as f64 / self.ramp as f64).cos())
        } else {
            1.0
        };

        let v = (self.phase.cos() * env) as f32 * self.amplitude;
        self.phase += std::f64::consts::TAU * f / self.rate;
        if self.phase > std::f64::consts::TAU {
            self.phase -= std::f64::consts::TAU;
        }
        self.pos += 1;
        v
    }
}

/// Synthesise a channel-symbol sequence as mono audio, all at once.
///
/// The tests and the loopback path use this; the transmitter uses [`BurstGen`]
/// directly so it never holds the whole burst.
pub fn synthesize_symbols(
    symbols: &[u8; N_SYMBOLS],
    sample_rate: u32,
    base_freq_hz: f64,
    amplitude: f32,
) -> Vec<f32> {
    let mut g = BurstGen::new(symbols, sample_rate, base_freq_hz, amplitude);
    let mut out = vec![0f32; g.total];
    g.fill(&mut out);
    out
}

/// Pack and encode a Type-1 message into channel symbols.
///
/// The locator is shortened to the four characters the message carries, so a
/// station that set a six-character one transmits rather than being refused —
/// see [`sdroxide_types::wspr_grid4`].
///
/// `None` when it still will not fit the layout: a compound callsign, or a
/// locator that is not one.
pub fn symbols_for(callsign: &str, grid: &str, power_dbm: i32) -> Option<[u8; N_SYMBOLS]> {
    Some(wspr::encode_channel_symbols(&pack(callsign, grid, power_dbm)?))
}

/// The fifty information bits, or `None` if this identity cannot be sent.
///
/// Both halves are upper-cased: the message packs them from an alphabet that
/// has no lower case, so a callsign typed in lower case would be refused for
/// looking like an invalid one rather than for being one.
fn pack(callsign: &str, grid: &str, power_dbm: i32) -> Option<[u8; 50]> {
    let g4 = sdroxide_types::wspr_grid4(grid)?;
    let call = callsign.trim().to_uppercase();
    if !packable_call(&call) {
        return None;
    }
    mfsk_core::msg::wspr::pack_type1(&call, &g4, power_dbm)
}

/// Whether `mfsk-core`'s callsign packer can be handed this string at all.
///
/// It right-aligns into six slots so the digit lands in slot 2, shifting the
/// call one place right when the digit is in slot 1 instead — and it does that
/// shift without checking there is room, so a six-character callsign with its
/// digit second writes past the end of the buffer and panics. `M0ABCD` is
/// enough to do it.
///
/// That matters more than a malformed callsign usually would, because nothing
/// here is a transmit path: [`why_not_sendable`] asks the packer this same
/// question every time the engine builds its status, so the panic is reachable
/// from the settings field by typing, and it would take down the engine thread
/// of a station that was only listening. Refused as unsendable instead, which
/// is what it is.
fn packable_call(call: &str) -> bool {
    let b = call.as_bytes();
    (3..=6).contains(&b.len())
        && b.iter().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        && (b[2].is_ascii_digit() || (b[1].is_ascii_digit() && b.len() < 6))
}

/// Why this identity cannot be transmitted, or `None` if it can.
///
/// The authority on the question, because it asks the packer rather than
/// re-deriving its rules: anything that returns `None` here is exactly what the
/// transmitter would refuse.
pub fn why_not_sendable(callsign: &str, grid: &str, power_dbm: i32) -> Option<String> {
    let call = callsign.trim();
    let grid = grid.trim();
    if call.is_empty() || grid.is_empty() {
        return Some("Set your callsign and grid before transmitting.".into());
    }
    if pack(call, grid, power_dbm).is_some() {
        return None;
    }
    if sdroxide_types::wspr_grid4(grid).is_none() {
        return Some(format!(
            "{grid} is not a Maidenhead locator, so there is nothing to transmit as. Two \
             letters and two digits — JN47 — is what the message carries."
        ));
    }
    // The grid packs, so it is the callsign the message cannot hold.
    Some(format!(
        "{call} cannot be sent: WSPR's message carries a plain callsign, and a compound one \
         needs a layout this station cannot encode. Receiving is unaffected."
    ))
}

#[cfg(test)]
mod tests {
    /// A callsign that cannot be packed has to come back as "cannot be sent",
    /// not as a panic — and this one used to panic, inside the status the
    /// engine rebuilds every poll.
    #[test]
    fn a_callsign_the_packer_cannot_take_is_refused_rather_than_crashing() {
        // Six characters with the digit second: the case that overflowed.
        assert!(super::why_not_sendable("M0ABCD", "JN47", 23).is_some());
        assert!(super::symbols_for("M0ABCD", "JN47", 23).is_none());
        // Neither slot holds a digit.
        assert!(super::why_not_sendable("ABCDEF", "JN47", 23).is_some());
        assert!(super::why_not_sendable("AB", "JN47", 23).is_some());
        // And the shapes that are sendable still are.
        for call in ["K1ABC", "OK1UNL", "G0XYZ", "2E0ABC", "VK7ZZ"] {
            assert!(super::why_not_sendable(call, "JN47", 23).is_none(), "{call} was refused");
        }
    }

    use super::*;

    /// Power spectrum of `audio` around `center_hz`, at the offsets in `offsets`
    /// (Hz), returned in dB relative to the strongest of them.
    ///
    /// Mixes to complex baseband and box-decimates by 120 (12 kHz → 100 Hz)
    /// before the DFT, so a 1.3 M-sample burst costs a few million operations
    /// instead of a few billion. The box filter's first null is at 100 Hz and it
    /// droops less than a dB inside ±20 Hz, which is the whole span measured.
    fn spectrum_db(audio: &[f32], rate: f64, center_hz: f64, offsets: &[f64]) -> Vec<f64> {
        const DECIM: usize = 120;
        let mut bb: Vec<(f64, f64)> = Vec::with_capacity(audio.len() / DECIM);
        let (mut acc_re, mut acc_im) = (0.0f64, 0.0f64);
        for (n, &x) in audio.iter().enumerate() {
            let w = -std::f64::consts::TAU * center_hz * n as f64 / rate;
            acc_re += x as f64 * w.cos();
            acc_im += x as f64 * w.sin();
            if (n + 1) % DECIM == 0 {
                bb.push((acc_re / DECIM as f64, acc_im / DECIM as f64));
                acc_re = 0.0;
                acc_im = 0.0;
            }
        }
        let bb_rate = rate / DECIM as f64;
        let raw: Vec<f64> = offsets
            .iter()
            .map(|&off| {
                let w = -std::f64::consts::TAU * off / bb_rate;
                let (mut re, mut im) = (0.0f64, 0.0f64);
                for (n, &(br, bi)) in bb.iter().enumerate() {
                    let (c, s) = (w * n as f64).cos_sin();
                    re += br * c - bi * s;
                    im += br * s + bi * c;
                }
                re * re + im * im
            })
            .collect();
        let peak = raw.iter().cloned().fold(f64::MIN_POSITIVE, f64::max);
        raw.iter().map(|p| 10.0 * (p / peak).max(1e-30).log10()).collect()
    }

    /// The unshaped burst, for comparison: what `mfsk_core::wspr::tx` produces
    /// and what this module exists to improve on.
    fn unshaped(symbols: &[u8; N_SYMBOLS], rate: u32, base: f64, amp: f32) -> Vec<f32> {
        let nsps = nsps(rate);
        let mut out = Vec::with_capacity(nsps * N_SYMBOLS);
        let mut phase = 0.0f64;
        for &s in symbols.iter() {
            let f = base + (s % 4) as f64 * TONE_SPACING_HZ;
            let dphi = std::f64::consts::TAU * f / rate as f64;
            for _ in 0..nsps {
                out.push(phase.cos() as f32 * amp);
                phase += dphi;
            }
        }
        out
    }

    fn symbols() -> [u8; N_SYMBOLS] {
        let info = mfsk_core::msg::wspr::pack_type1("K1ABC", "FN42", 37).expect("valid");
        wspr::encode_channel_symbols(&info)
    }

    #[test]
    fn a_burst_is_162_symbols_long_at_any_rate() {
        assert_eq!(burst_len(12_000), 8192 * 162);
        assert_eq!(synthesize_symbols(&symbols(), 12_000, 1500.0, 0.5).len(), burst_len(12_000));
        assert_eq!(synthesize_symbols(&symbols(), 48_000, 1500.0, 0.5).len(), burst_len(48_000));
        // 110.6 s, the figure every description of WSPR quotes.
        assert!((burst_len(12_000) as f64 / 12_000.0 - 110.6).abs() < 0.05);
    }

    /// The click test. An unramped burst starts at full amplitude on sample
    /// zero; this one has to start at silence and get there smoothly.
    #[test]
    fn the_burst_fades_in_and_out_rather_than_starting_at_full_amplitude() {
        let a = synthesize_symbols(&symbols(), 12_000, 1500.0, 0.5);
        assert!(a[0].abs() < 1e-6, "burst opens at {}", a[0]);
        assert!(a[a.len() - 1].abs() < 1e-3, "burst closes at {}", a[a.len() - 1]);
        // A fifth of a second in, it is at full level: the ramp is short.
        let mid_peak = a[2400..12_000].iter().fold(0.0f32, |m, x| m.max(x.abs()));
        assert!(mid_peak > 0.49, "never reached full amplitude ({mid_peak})");
        // And the ramp is monotonic enough that no sample early on exceeds the
        // final level — an overshoot would be its own click.
        assert!(a.iter().all(|x| x.abs() <= 0.5 + 1e-6));
    }

    /// The reason for the frequency crossfade, measured rather than asserted by
    /// authority: shaped sidelobes must be materially below unshaped ones.
    ///
    /// Only the near skirts are measured (10–20 Hz off centre). Further out the
    /// decimating box filter's own response starts to matter, and closer in is
    /// the signal itself.
    #[test]
    fn shaping_pulls_the_sidelobes_down() {
        let sym = symbols();
        let center = 1500.0 + 1.5 * TONE_SPACING_HZ;
        let offsets: Vec<f64> = (0..=40).map(|i| i as f64 * 0.5).collect(); // 0..20 Hz
        let skirt = |a: &[f32]| -> f64 {
            let db = spectrum_db(a, 12_000.0, center, &offsets);
            // Worst (loudest) point in the 10–20 Hz skirt.
            db.iter()
                .zip(&offsets)
                .filter(|(_, o)| **o >= 10.0)
                .map(|(d, _)| *d)
                .fold(f64::MIN, f64::max)
        };
        let shaped = skirt(&synthesize_symbols(&sym, 12_000, 1500.0, 0.5));
        let plain = skirt(&unshaped(&sym, 12_000, 1500.0, 0.5));
        assert!(
            shaped < plain - 6.0,
            "shaping bought only {:.1} dB (shaped {shaped:.1}, plain {plain:.1})",
            plain - shaped
        );
        // And in absolute terms the skirt is well down on the signal.
        assert!(shaped < -30.0, "skirt is only {shaped:.1} dB down");
    }

    #[test]
    fn a_message_that_does_not_fit_type_1_is_refused_rather_than_mangled() {
        // Compound callsigns need the Type-2 layout, which has no packer.
        assert!(synthesize("PJ4/K1ABC", "FN42", 37, 12_000, 1500.0, 0.5).is_none());
        assert!(synthesize("K1ABC", "FN42", 37, 12_000, 1500.0, 0.5).is_some());
    }

    /// A six-character locator is *not* a reason to refuse: its first four are
    /// the four-character one, which is what the message carries.
    #[test]
    fn a_six_character_locator_transmits_as_its_first_four() {
        let long = synthesize("K1ABC", "FN42aa", 37, 12_000, 1500.0, 0.5)
            .expect("a longer locator is shortened, not refused");
        let short = synthesize("K1ABC", "FN42", 37, 12_000, 1500.0, 0.5).expect("valid");
        assert_eq!(long, short, "FN42aa and FN42 must go out as the same message");
        // However it was typed.
        let lower = synthesize("k1abc", " fn42cb ", 37, 12_000, 1500.0, 0.5).expect("valid");
        assert_eq!(lower, short);
    }

    /// The reason given has to name the thing that is actually wrong, or it
    /// sends the operator to fix the wrong field — which is what it did.
    #[test]
    fn the_refusal_says_which_half_of_the_identity_is_the_problem() {
        assert!(why_not_sendable("K1ABC", "FN42", 37).is_none());
        assert!(why_not_sendable("K1ABC", "FN42aa", 37).is_none(), "a long locator is fine");

        let compound = why_not_sendable("PJ4/K1ABC", "FN42", 37).expect("cannot be sent");
        assert!(compound.contains("PJ4/K1ABC"), "{compound}");
        assert!(!compound.contains("locator"), "blamed the locator: {compound}");

        let bad_grid = why_not_sendable("K1ABC", "somewhere", 37).expect("cannot be sent");
        assert!(bad_grid.contains("locator"), "{bad_grid}");

        let empty = why_not_sendable("", "", 37).expect("nothing to send as");
        assert!(empty.contains("callsign"), "{empty}");
    }
}

#[cfg(test)]
trait CosSin {
    fn cos_sin(self) -> (f64, f64);
}

#[cfg(test)]
impl CosSin for f64 {
    fn cos_sin(self) -> (f64, f64) {
        (self.cos(), self.sin())
    }
}
