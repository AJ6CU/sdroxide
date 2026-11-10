//! Bit recovery from a discriminator trace: symbol timing by brute force, then
//! a sync-word search that also settles which way round the tones are.
//!
//! # Why brute force
//!
//! A tracking loop — Gardner, early-late, anything with a bandwidth — has to
//! acquire, and a frame that is over in five milliseconds does not give it long
//! enough. `RifpRx` in `sdroxide-dsp` solves the same problem the same way and
//! its module note puts it best: on a burst that arrives without warning it costs
//! nothing to be right. Here that means slicing the burst at every one of
//! [`PHASES`] sub-symbol offsets and keeping whichever one finds the sync word
//! with the fewest bit errors. A few thousand comparisons per burst.
//!
//! # Why the tone sense is searched rather than declared
//!
//! Whether the higher of the two tones means one or zero depends on the device,
//! and then on whether anything in the receive chain mirrored the spectrum. A
//! known sync word settles it for free: look for the pattern *and* its
//! complement, and whichever matches tells you the sense for the rest of the
//! frame. Declaring it instead means a whole family of sensors silently fails to
//! decode on a front end with high-side injection.

/// Sub-symbol offsets tried. One sixteenth of a symbol is closer to optimal
/// timing than the crystal tolerance of these transmitters, so more would be
/// measuring nothing.
const PHASES: usize = 16;

/// Fraction of each symbol integrated, centred in the symbol.
///
/// Integrating the whole symbol is the matched filter for unshaped NRZ and has
/// the best noise performance, but it also straddles both transitions, so any
/// residual timing error mixes the neighbours in. Trimming an eighth off each
/// end costs almost nothing in SNR and makes the slicer far less sensitive to
/// being half a sample out.
const INTEGRATE_FRAC: f64 = 0.75;

/// What a protocol's physical layer looks like, as far as the slicer cares.
#[derive(Debug, Clone, Copy)]
pub struct Phy {
    /// Nominal symbol rate. Only a starting point — the real one is measured
    /// from the preamble, see [`crate::demod::estimate_sps`].
    pub baud: f64,
    /// Sync word, right-aligned in `sync_bits` bits, MSB first on the air.
    pub sync: u32,
    pub sync_bits: u32,
    /// Bit errors tolerated in the sync match. Zero is tempting and wrong: the
    /// sync word arrives at the weakest point of the burst, before any level
    /// estimate has settled, and one slipped bit there throws away a frame whose
    /// CRC would have passed.
    pub sync_errors: u32,
    /// Alternating bits that must immediately precede the sync word, or 0 not to
    /// check.
    ///
    /// This is what keeps noise out. A 16-bit sync word matched within one error
    /// accepts 17 of 65 536 patterns, so a long noise burst sliced at sixteen
    /// phases in both senses throws up a handful of candidates every time, and
    /// an 8-bit CRC then passes one in 256 of those. Requiring a run of
    /// alternating bits in front — which every protocol here transmits, because
    /// its receiver needs it for exactly the same reason ours does — costs
    /// nothing and removes almost all of them.
    pub preamble_bits: u32,
    /// Fewest payload bytes a frame of this protocol can have. A burst too short
    /// to hold this many is not one.
    pub payload_min: usize,
    /// Most payload bytes to take. A family whose members differ in length — the
    /// Fine Offset sensors run from fourteen bytes to seventeen — sets this to the
    /// longest and lets the parser check its own CRC at whichever length it
    /// expects. Taking a fixed count instead would reject every shorter member.
    pub payload_max: usize,
    /// Whether each pair of transmitted chips carries one data bit.
    ///
    /// Z-Wave's slowest rate is Manchester, and it is the reason this field
    /// exists: at 9.6 kbit/s it puts 19 200 chips a second on the air, so a
    /// decoder that sliced at the *data* rate would read every second edge and
    /// find nothing. [`baud`](Self::baud) is always the chip rate — what the
    /// symbol-rate estimator actually measures off the signal — and the pair
    /// decoding happens after slicing, before the sync search.
    pub manchester: bool,
}

/// One candidate frame lifted out of a burst.
#[derive(Debug, Clone, PartialEq)]
pub struct Frame {
    pub bytes: Vec<u8>,
    /// Bit errors in the sync match that found it.
    pub sync_errors: u32,
    /// True when the frame was found by matching the complement of the sync
    /// word, i.e. the high tone means zero on this device.
    pub inverted: bool,
    /// Samples per symbol actually used, for diagnostics.
    pub sps: f64,
}

/// Recover every distinct frame a burst can be read as.
///
/// `disc` is the discriminator trace, `level` the slicing level — the carrier,
/// from [`crate::demod::stats`]. Candidates come back in increasing order of
/// sync error, so a caller that CRC-checks them in order tries the most likely
/// reading first.
pub fn frames(disc: &[f32], level: f32, rate_hz: f64, phy: &Phy) -> Vec<Frame> {
    let nominal = rate_hz / phy.baud;

    // Every symbol period worth trying, not just one.
    //
    // Refining the protocol's declared rate is bounded to ±20 % of it, which is
    // right for a transmitter that is merely off frequency and wrong for a *family*
    // whose members run at different rates. A Bresser sensor at 8.2 kbaud and a
    // Fine Offset one at 17.24 share a sync word and a channel, and bounding the
    // search around either one makes the other invisible — the sync word is sitting
    // in the trace and nothing ever slices at a rate that could see it.
    //
    // So the burst's own measured rate goes in the list as well. Trying both costs
    // one more pass over a few thousand samples, and what identifies a protocol is
    // its sync word and its CRC, not a rate written down in advance.
    let mut rates: Vec<f64> = Vec::with_capacity(3);
    if let Some((baud, fit)) = crate::demod::estimate_baud_free(disc, level, rate_hz) {
        // Only a measurement that verified itself; see `estimate_baud_free`.
        if fit >= 0.8 {
            rates.push(rate_hz / baud);
        }
    }
    rates.push(crate::demod::estimate_sps(disc, level, nominal).unwrap_or(nominal));
    // Near-duplicates are the same slicing done twice.
    rates.dedup_by(|a, b| (*a - *b).abs() < *b * 0.02);

    let want_bits = phy.sync_bits as usize + phy.payload_min * 8;
    let mut out: Vec<Frame> = Vec::new();
    let mut bits: Vec<u8> = Vec::new();

    for sps in rates {
        let symbols = (disc.len() as f64 / sps) as usize;
        if symbols < want_bits {
            continue;
        }
        frames_at(disc, level, rate_hz, phy, sps, &mut bits, &mut out);
    }

    out.sort_by_key(|f| f.sync_errors);
    out
}

/// Every frame the burst reads as at one symbol period.
fn frames_at(
    disc: &[f32],
    level: f32,
    _rate_hz: f64,
    phy: &Phy,
    sps: f64,
    bits: &mut Vec<u8>,
    out: &mut Vec<Frame>,
) {
    // Holds the pair-decoded stream when the protocol is Manchester. The raw
    // slice stays in `bits`, because the second pair alignment has to be decoded
    // from the chips again rather than from the first alignment's output.
    let mut decoded: Vec<u8> = Vec::new();
    for p in 0..PHASES {
        let phase = sps * p as f64 / PHASES as f64;
        slice_bits(disc, level, sps, phase, bits);

        // A Manchester stream has two possible pair alignments and only one of
        // them is the transmitter's. Rather than pick by counting coding
        // violations — which would need a threshold to justify — both are offered
        // to the sync search and the sync word settles it for free. The wrong
        // alignment reads as noise and matches nothing.
        for pair in 0..if phy.manchester { 2 } else { 1 } {
            let view: &[u8] = if phy.manchester {
                manchester_decode(bits, pair, &mut decoded);
                &decoded
            } else {
                bits
            };

            for inverted in [false, true] {
                let target = if inverted {
                    // Complement, kept inside the sync word's own width.
                    (!phy.sync) & mask(phy.sync_bits)
                } else {
                    phy.sync & mask(phy.sync_bits)
                };
                // Every match, not just the best one: a false match earlier in the
                // burst can tie on sync errors with the real one, and taking only
                // the first would then lose the frame to the preamble check below.
                for (at, errs) in find_sync(view, target, phy.sync_bits, phy.sync_errors) {
                    if !preamble_ok(view, at, phy.preamble_bits) {
                        continue;
                    }
                    let start = at + phy.sync_bits as usize;
                    if start + phy.payload_min * 8 > view.len() {
                        continue;
                    }
                    // As much as the burst holds, up to the longest frame the
                    // protocol can send.
                    let take = phy.payload_max.min((view.len() - start) / 8);
                    let mut bytes = pack(&view[start..start + take * 8]);
                    if inverted {
                        for b in bytes.iter_mut() {
                            *b = !*b;
                        }
                    }
                    // Two adjacent phases usually read the same frame. Keep the
                    // better match rather than handing the protocol layer
                    // duplicates to CRC-check.
                    match out.iter_mut().find(|f| f.bytes == bytes) {
                        Some(f) if f.sync_errors > errs => {
                            f.sync_errors = errs;
                            f.inverted = inverted;
                            f.sps = sps;
                        }
                        Some(_) => {}
                        None => out.push(Frame { bytes, sync_errors: errs, inverted, sps }),
                    }
                }
            }
        }
    }
}

/// Fold Manchester chip pairs into data bits, starting at chip `phase`.
///
/// `10` is a one and `01` a zero. A pair of equal chips is a coding violation —
/// noise, or the wrong pair alignment — and is emitted as the first chip of the
/// pair rather than dropped, so the bit positions after it stay put: a violation
/// mid-frame should cost that one bit and let the frame's own checksum decide,
/// not shift everything after it and destroy the whole frame.
fn manchester_decode(chips: &[u8], phase: usize, out: &mut Vec<u8>) {
    out.clear();
    out.reserve(chips.len() / 2);
    let mut i = phase;
    while i + 1 < chips.len() {
        out.push(chips[i]);
        i += 2;
    }
}

fn mask(bits: u32) -> u32 {
    if bits >= 32 { u32::MAX } else { (1u32 << bits) - 1 }
}

/// Slice a whole trace at one symbol rate, sampling mid-symbol.
///
/// For the diagnostic dump, where there is no sync word to pick a phase by: a
/// preamble and a sync word are recognisable in hex at any phase, which is the
/// point of showing them.
pub fn slice_at(disc: &[f32], level: f32, sps: f64) -> Vec<u8> {
    let mut out = Vec::new();
    slice_bits(disc, level, sps, sps / 2.0, &mut out);
    out
}

/// Bits to bytes, MSB first. Public for the diagnostic dump.
pub fn pack_bits(bits: &[u8]) -> Vec<u8> {
    pack(bits)
}

/// Integrate-and-dump each symbol and slice it against `level`.
fn slice_bits(disc: &[f32], level: f32, sps: f64, phase: f64, out: &mut Vec<u8>) {
    out.clear();
    let skip = sps * (1.0 - INTEGRATE_FRAC) / 2.0;
    let mut k = 0usize;
    loop {
        let a = phase + k as f64 * sps + skip;
        let b = phase + (k + 1) as f64 * sps - skip;
        let (i0, i1) = (a.round() as usize, b.round() as usize);
        if i1 > disc.len() || i0 >= i1 {
            break;
        }
        let sum: f32 = disc[i0..i1].iter().sum();
        out.push(if sum / (i1 - i0) as f32 > level { 1 } else { 0 });
        k += 1;
    }
}

/// Every position where `target` matches within `max_errors` bits, best first.
fn find_sync(bits: &[u8], target: u32, width: u32, max_errors: u32) -> Vec<(usize, u32)> {
    let w = width as usize;
    let mut hits = Vec::new();
    if bits.len() < w {
        return hits;
    }
    for at in 0..=bits.len() - w {
        let mut errs = 0u32;
        for i in 0..w {
            let want = (target >> (w - 1 - i)) & 1;
            if u32::from(bits[at + i]) != want {
                errs += 1;
                if errs > max_errors {
                    break;
                }
            }
        }
        if errs <= max_errors {
            hits.push((at, errs));
        }
    }
    hits.sort_by_key(|&(_, e)| e);
    hits
}

/// Whether the `n` bits ending at `at` alternate, as a preamble does.
///
/// The phase is not checked, only that consecutive bits differ: a preamble is
/// `1010…` or `0101…` depending on where the slicer happened to start, and
/// inverting the tone sense swaps the two, so requiring one of them would reject
/// half of all real frames.
///
/// The budget is **two** equal-adjacent pairs, not one, because one flipped bit
/// in the middle of an alternating run breaks the pair on each side of it. A
/// budget of one would therefore tolerate a slip only at the very ends of the
/// run, which is not what "allow one bit error" means. Two pairs out of fifteen
/// still happens by chance in only about one random sequence in 270, so the
/// filter loses almost none of its strength.
fn preamble_ok(bits: &[u8], at: usize, n: u32) -> bool {
    if n == 0 {
        return true;
    }
    let n = n as usize;
    if at < n {
        return false;
    }
    let seg = &bits[at - n..at];
    let same = seg.windows(2).filter(|w| w[0] == w[1]).count();
    same <= 2
}

/// Bits to bytes, MSB first — the order every protocol here puts on the air.
fn pack(bits: &[u8]) -> Vec<u8> {
    bits.chunks(8).map(|c| c.iter().fold(0u8, |acc, &b| (acc << 1) | (b & 1))).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const RATE: f64 = 250_000.0;
    const BAUD: f64 = 17_241.0;

    fn phy() -> Phy {
        Phy {
            baud: BAUD,
            sync: 0x2DD4,
            sync_bits: 16,
            sync_errors: 1,
            preamble_bits: 16,
            payload_min: 5,
            payload_max: 5,
            manchester: false,
        }
    }

    /// Build the discriminator trace a 2-FSK transmitter would produce, from the
    /// bit sequence up: preamble, sync word, payload. Deliberately *not* built
    /// with any of the functions under test.
    fn trace(payload: &[u8], sps: f64, dev: f32, invert: bool, noise: f32) -> Vec<f32> {
        let mut bits: Vec<u8> = (0..48).map(|i| (i % 2) as u8).collect();
        for i in (0..16).rev() {
            bits.push(((0x2DD4u32 >> i) & 1) as u8);
        }
        for &b in payload {
            for i in (0..8).rev() {
                bits.push((b >> i) & 1);
            }
        }

        let mut seed = 0xA5A5_1234_DEAD_BEEFu64;
        let mut rng = move || {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            ((seed >> 11) as f64 / (1u64 << 53) as f64) as f32 * 2.0 - 1.0
        };

        let mut out = Vec::new();
        for (i, &b) in bits.iter().enumerate() {
            let hi = if invert { b == 0 } else { b != 0 };
            let f = if hi { dev } else { -dev };
            let n = (((i + 1) as f64 * sps).round() - (i as f64 * sps).round()) as usize;
            for _ in 0..n {
                out.push(f + rng() * noise);
            }
        }
        out
    }

    #[test]
    fn a_clean_frame_comes_back_exactly() {
        let payload = [0x9A, 0x1B, 0x2C, 0x3D, 0x4E];
        let d = trace(&payload, RATE / BAUD, 50_000.0, false, 0.0);
        let f = frames(&d, 0.0, RATE, &phy());
        assert!(!f.is_empty(), "no frame found in a noiseless burst");
        assert_eq!(f[0].bytes, payload);
        assert_eq!(f[0].sync_errors, 0);
        assert!(!f[0].inverted);
    }

    /// The inversion case: a device — or a front end — with the tones the other
    /// way round has to decode identically.
    #[test]
    fn an_inverted_burst_decodes_to_the_same_payload() {
        let payload = [0x9A, 0x1B, 0x2C, 0x3D, 0x4E];
        let d = trace(&payload, RATE / BAUD, 50_000.0, true, 0.0);
        let f = frames(&d, 0.0, RATE, &phy());
        assert!(!f.is_empty(), "an inverted burst found nothing");
        assert_eq!(f[0].bytes, payload);
        assert!(f[0].inverted, "the inversion was not reported");
    }

    /// A transmitter running off nominal must still decode — this is the reason
    /// the symbol period is measured rather than taken from `Phy::baud`.
    #[test]
    fn a_transmitter_off_frequency_rate_still_decodes() {
        let payload = [0x51, 0x00, 0x12, 0x34, 0xAB];
        for err in [-0.005f64, -0.002, 0.002, 0.005] {
            let sps = RATE / (BAUD * (1.0 + err));
            let d = trace(&payload, sps, 50_000.0, false, 0.0);
            let f = frames(&d, 0.0, RATE, &phy());
            assert!(!f.is_empty(), "rate error {err} decoded nothing");
            assert_eq!(f[0].bytes, payload, "rate error {err}");
        }
    }

    /// Noise on the discriminator is the normal case, not the exception.
    #[test]
    fn a_noisy_frame_still_decodes() {
        let payload = [0x9A, 0xFF, 0x00, 0x55, 0xAA];
        // Noise amplitude a third of the deviation.
        let d = trace(&payload, RATE / BAUD, 50_000.0, false, 16_000.0);
        let f = frames(&d, 0.0, RATE, &phy());
        assert!(!f.is_empty(), "a noisy burst decoded nothing");
        assert!(f.iter().any(|x| x.bytes == payload), "payload not among {} candidates", f.len());
    }

    /// Noise alone must not manufacture frames. This is the test that justifies
    /// `Phy::preamble_bits`: without the alternating-run requirement the same
    /// input yields around ten candidates, and an 8-bit CRC lets one in 256 of
    /// those through.
    #[test]
    fn noise_produces_few_candidates() {
        let mut seed = 0x1357_9BDF_0246_8ACEu64;
        let d: Vec<f32> = (0..20_000)
            .map(|_| {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                ((seed >> 11) as f64 / (1u64 << 53) as f64) as f32 * 100_000.0 - 50_000.0
            })
            .collect();
        let f = frames(&d, 0.0, RATE, &phy());
        assert!(f.len() < 8, "noise produced {} candidate frames", f.len());
    }

    /// A burst too short to hold the frame must produce nothing rather than
    /// reading off the end of the buffer.
    #[test]
    fn a_burst_shorter_than_the_frame_is_rejected() {
        let d = trace(&[0x11, 0x22, 0x33, 0x44, 0x55], RATE / BAUD, 50_000.0, false, 0.0);
        for take in [0usize, 1, 10, 200] {
            let f = frames(&d[..take.min(d.len())], 0.0, RATE, &phy());
            assert!(f.is_empty(), "{take} samples produced {} frames", f.len());
        }
    }

    #[test]
    fn bits_pack_msb_first() {
        assert_eq!(pack(&[1, 0, 1, 0, 1, 1, 0, 0]), vec![0xAC]);
        assert_eq!(pack(&[0, 0, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0, 1, 0, 0]), vec![0x2D, 0xD4]);
    }

    #[test]
    fn the_sync_search_reports_where_and_how_badly_it_matched() {
        let mut bits = vec![0u8; 20];
        for i in 0..16 {
            bits.push(((0x2DD4u32 >> (15 - i)) & 1) as u8);
        }
        bits.extend([1, 1, 1, 1]);
        assert_eq!(find_sync(&bits, 0x2DD4, 16, 0), vec![(20, 0)]);

        // One bit flipped: found, with the error counted.
        bits[24] ^= 1;
        assert_eq!(find_sync(&bits, 0x2DD4, 16, 1), vec![(20, 1)]);
        assert!(find_sync(&bits, 0x2DD4, 16, 0).is_empty());
    }

    /// The preamble check must accept either phase of an alternating run — the
    /// slicer's starting point and the tone sense both decide which one arrives,
    /// and rejecting one of them would lose half of all real frames.
    #[test]
    fn the_preamble_check_accepts_both_phases_and_one_slip() {
        let a: Vec<u8> = (0..16).map(|i| (i % 2) as u8).collect();
        let b: Vec<u8> = (0..16).map(|i| ((i + 1) % 2) as u8).collect();
        assert!(preamble_ok(&a, 16, 16));
        assert!(preamble_ok(&b, 16, 16));

        // One slipped bit in the middle breaks the pair on each side of it, so
        // this is the case the budget of two exists for.
        let mut one_slip = a.clone();
        one_slip[7] ^= 1;
        assert!(preamble_ok(&one_slip, 16, 16), "one slipped bit should be tolerated");

        let mut two_slips = a.clone();
        two_slips[4] ^= 1;
        two_slips[11] ^= 1;
        assert!(!preamble_ok(&two_slips, 16, 16), "two slips is not a preamble");

        // And a run that is not alternating at all is rejected outright.
        assert!(!preamble_ok(&[0u8; 16], 16, 16));
        assert!(!preamble_ok(&[1u8; 16], 16, 16));

        assert!(!preamble_ok(&a, 8, 16), "there are not 16 bits before position 8");
        assert!(preamble_ok(&a, 0, 0), "a protocol that declares no preamble skips the check");
    }
}
