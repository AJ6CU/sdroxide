//! CTCSS and DCS decoding, driven through the real NFM demodulator.
//!
//! Every signal here is genuine frequency modulation at the deviations a
//! transmitter actually uses — a sub-audible tone at about 15 % of peak
//! deviation, under speech-band programme at the rest — and goes in as IQ at the
//! channel rate through `make_demod(Mode::Nfm, …)`. A detector fed a bare audio
//! tone would pass tests it has no business passing: that is exactly how the WFM
//! stereo blend came to be miscalibrated in this codebase.

use num_complex::Complex;
use sdroxide_dsp::make_demod;
use sdroxide_types::{Mode, SubTone};

type C32 = Complex<f32>;

const RATE: f64 = 48_000.0;
/// What a transmitter puts on the sub-audible signalling: about 15 % of the
/// 5 kHz peak deviation an NFM system uses.
const TONE_DEV: f64 = 750.0;
/// And what is left for the voice.
const VOICE_DEV: f64 = 3_500.0;

/// Deterministic noise, the same splitmix64 the other DSP tests use.
struct Rng(u64);

impl Rng {
    fn next_f32(&mut self) -> f32 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        ((z ^ (z >> 31)) >> 40) as f32 / (1 << 23) as f32 - 1.0
    }
}

/// A busy speech band: enough partials, each breathing at its own rate, that no
/// single one of them could be mistaken for a sub-audible carrier. This is the
/// part that catches a detector which only works on a clean signal.
///
/// Nothing below 300 Hz, because nothing is there on the air either: an FM
/// transmitter high-passes its microphone at about 300 Hz precisely so the
/// sub-audible band underneath belongs to the tone.
fn programme(t: f64) -> f64 {
    const PARTIALS: [(f64, f64); 6] =
        [(330.0, 0.9), (520.0, 1.0), (830.0, 0.8), (1270.0, 0.7), (1900.0, 0.5), (2600.0, 0.35)];
    let sum: f64 = PARTIALS
        .iter()
        .enumerate()
        .map(|(i, (f, a))| {
            // Amplitude, not frequency: a wobble on the phase argument would
            // sweep each partial across hundreds of hertz, which is a chirp
            // generator rather than a voice.
            let breath = 1.0 + 0.35 * (std::f64::consts::TAU * (0.7 + i as f64 * 0.31) * t).sin();
            a * breath * (std::f64::consts::TAU * f * t).sin()
        })
        .sum();
    let norm: f64 = PARTIALS.iter().map(|(_, a)| a).sum();
    sum / (norm * 1.35)
}

/// FM-modulate a baseband deviation function into IQ at the channel rate.
fn modulate(secs: f64, noise_amp: f32, dev_at: impl Fn(f64) -> f64) -> Vec<C32> {
    let n = (RATE * secs) as usize;
    let mut rng = Rng(0x5DEE_CE66_D125_1337);
    let mut phase = 0.0f64;
    (0..n)
        .map(|i| {
            let t = i as f64 / RATE;
            phase += std::f64::consts::TAU * dev_at(t) / RATE;
            let (s, c) = phase.sin_cos();
            C32::new(
                c as f32 * 0.5 + noise_amp * rng.next_f32(),
                s as f32 * 0.5 + noise_amp * rng.next_f32(),
            )
        })
        .collect()
}

/// A repeater sending a CTCSS tone under speech.
fn ctcss_signal(tone_hz: f64, secs: f64, noise_amp: f32) -> Vec<C32> {
    modulate(secs, noise_amp, |t| {
        TONE_DEV * (std::f64::consts::TAU * tone_hz * t).sin() + VOICE_DEV * programme(t)
    })
}

/// Run a signal through the real NFM demodulator and report what the
/// sub-audible decoder made of it, plus the audio it produced.
fn demodulate(iq: &[C32]) -> (Option<SubTone>, Vec<f32>) {
    demodulate_in_blocks(iq, 2048)
}

/// As [`demodulate`], with the size of the blocks the caller feeds spelled out.
///
/// 2048 is a poor default to test only at: three of them are exactly one
/// detector hop, so a decoder that quietly assumed its caller's block boundaries
/// were its own would pass anyway. See
/// [`the_tone_is_found_whatever_block_size_the_caller_uses`].
fn demodulate_in_blocks(iq: &[C32], block: usize) -> (Option<SubTone>, Vec<f32>) {
    let mut demod = make_demod(Mode::Nfm, RATE).expect("NFM has a demodulator");
    let mut audio = Vec::new();
    for chunk in iq.chunks(block) {
        demod.process(chunk, &mut audio);
    }
    (demod.sub_tone(), audio)
}

/// Goertzel power of a real signal at one frequency, normalised so a full-scale
/// sinusoid reads 1.0.
fn power_at(x: &[f32], freq: f64) -> f64 {
    let w = std::f64::consts::TAU * freq / RATE;
    let coeff = 2.0 * w.cos();
    let (mut s1, mut s2) = (0.0f64, 0.0f64);
    for &v in x {
        let s0 = v as f64 + coeff * s1 - s2;
        s2 = s1;
        s1 = s0;
    }
    (s1 * s1 + s2 * s2 - coeff * s1 * s2) / (x.len() as f64 * x.len() as f64 / 4.0)
}

/// The everyday case: a repeater keys up with its tone under a voice.
#[test]
fn a_repeater_tone_is_identified_under_dense_speech() {
    let (tone, _) = demodulate(&ctcss_signal(88.5, 2.0, 0.0));
    assert_eq!(tone, Some(SubTone::Ctcss(885)));
}

/// The tightest pairs in the standard table are 2.3 and 2.4 Hz apart, which is
/// what fixes the length of the analysis window. Each has to come back as
/// itself, not as its neighbour.
#[test]
fn adjacent_standard_tones_are_not_confused() {
    for (hz, tenths) in [(67.0, 670u16), (69.3, 693), (159.8, 1598), (162.2, 1622), (254.1, 2541)] {
        let (tone, _) = demodulate(&ctcss_signal(hz, 2.0, 0.0));
        assert_eq!(tone, Some(SubTone::Ctcss(tenths)), "{hz} Hz");
    }
}

/// A tone that is not in the table is not a CTCSS tone. 105 Hz sits between
/// 103.5 and 107.2 and would be reported as one of them by anything that simply
/// picks the largest bin.
#[test]
fn a_tone_that_is_not_in_the_table_is_refused() {
    for hz in [105.0, 140.0, 60.0] {
        let (tone, _) = demodulate(&ctcss_signal(hz, 2.0, 0.0));
        assert_eq!(tone, None, "{hz} Hz is not a standard tone");
    }
}

/// Speech with no sub-audible tone at all, and noise with no signal at all,
/// both have to come back empty — a readout that invents a tone on an open
/// channel is worse than no readout.
#[test]
fn nothing_is_reported_without_a_tone() {
    let speech_only = modulate(2.0, 0.0, |t| VOICE_DEV * programme(t));
    assert_eq!(demodulate(&speech_only).0, None, "speech alone");

    let mut rng = Rng(0xC0FF_EE12_3456_789A);
    let noise: Vec<C32> = (0..(RATE * 2.0) as usize)
        .map(|_| C32::new(rng.next_f32() * 0.3, rng.next_f32() * 0.3))
        .collect();
    assert_eq!(demodulate(&noise).0, None, "noise alone");
}

/// Tuning onto a transmission already in progress must lock just the same.
///
/// The equivalent trap cost the keyboard-modem squelch a release: it seeded its
/// noise floor from whatever was present when it started, so arriving mid-over
/// latched it shut for good. This detector holds no such reference — it is a
/// ratio test inside each window — and this is the test that says so.
#[test]
fn a_transmission_already_in_progress_is_picked_up() {
    // No lead-in silence: the very first sample is already modulated.
    let (tone, _) = demodulate(&ctcss_signal(103.5, 1.6, 0.0));
    assert_eq!(tone, Some(SubTone::Ctcss(1035)));
}

/// Nothing about the block size the caller happens to use may reach the answer.
///
/// It did. The detector accumulated samples and then evaluated whatever the
/// history held once the block was drained, so consecutive windows were a block
/// apart rather than the hop apart its frequency check assumes — and that check
/// compares angles, so a wrong spacing wraps into a plausible-looking offset
/// instead of an obviously wrong one. The tone then passed or failed by
/// coincidence. A live radio reads a SoapySDR transfer of a size nobody here
/// chose, which is why this was reported from the air and not from this file:
/// every size here was a multiple of 512, and 512, 1024, 2048 and 6144 samples
/// all divide into the hop exactly.
///
/// So the sizes below are the awkward ones. 4096 straddles the hop, 16384
/// carries several of them, and the rest are simply not round.
#[test]
fn the_tone_is_found_whatever_block_size_the_caller_uses() {
    let iq = ctcss_signal(88.5, 2.5, 0.0);
    for block in [512usize, 1024, 1500, 1920, 2048, 3000, 4096, 6144, 8192, 16384] {
        let (tone, _) = demodulate_in_blocks(&iq, block);
        assert_eq!(tone, Some(SubTone::Ctcss(885)), "{block}-sample blocks");
    }
}

/// And every tone in the table has to survive an awkward block size, not just
/// the one that happened to be tried. The old failure was a function of the
/// tone's own frequency: some entries wrapped back inside tolerance and passed.
#[test]
fn an_awkward_block_size_costs_no_tone_in_the_table() {
    for (hz, tenths) in
        [(67.0, 670u16), (88.5, 885), (100.0, 1000), (103.5, 1035), (156.7, 1567), (254.1, 2541)]
    {
        let (tone, _) = demodulate_in_blocks(&ctcss_signal(hz, 2.5, 0.0), 16384);
        assert_eq!(tone, Some(SubTone::Ctcss(tenths)), "{hz} Hz");
    }
}

/// Once a repeater's tone has been identified it has to stay identified, block
/// after block, for as long as it is transmitting. The squelch gate is driven
/// straight off this, so a single window's worth of doubt is a hole in the
/// audio.
#[test]
fn a_held_tone_is_reported_without_interruption() {
    let iq = ctcss_signal(94.8, 4.0, 0.04);
    let mut demod = make_demod(Mode::Nfm, RATE).expect("NFM has a demodulator");
    let mut audio = Vec::new();
    let mut acquired = None;
    for (i, chunk) in iq.chunks(4096).enumerate() {
        demod.process(chunk, &mut audio);
        match (acquired, demod.sub_tone()) {
            (None, t) => acquired = t.map(|t| (i, t)),
            (Some((at, want)), got) => {
                assert_eq!(got, Some(want), "block {i}: dropped a tone held since block {at}")
            }
        }
    }
    let (at, tone) = acquired.expect("the tone is found at all");
    assert_eq!(tone, SubTone::Ctcss(948));
    // 1.024 s of window plus two agreeing hops, in 4096-sample blocks.
    assert!(at <= 16, "took {at} blocks to identify the tone");
}

/// A weak signal still gets its tone. Noise here is comparable to the carrier,
/// which on FM is around the point where the audio becomes hard to listen to.
#[test]
fn a_noisy_signal_still_yields_its_tone() {
    let (tone, _) = demodulate(&ctcss_signal(114.8, 2.5, 0.10));
    assert_eq!(tone, Some(SubTone::Ctcss(1148)));
}

/// The tone is meant to be decoded, not heard. The listener band-pass takes it
/// out; the voice above it is untouched.
#[test]
fn the_tone_does_not_reach_the_speaker() {
    let (tone, audio) = demodulate(&ctcss_signal(88.5, 2.0, 0.0));
    assert_eq!(tone, Some(SubTone::Ctcss(885)));
    // Skip the filter's start-up transient.
    let settled = &audio[audio.len() / 2..];
    let sub = power_at(settled, 88.5);
    let voice = power_at(settled, 830.0);
    assert!(
        sub < voice / 1000.0,
        "the 88.5 Hz tone is only {:.1} dB below the voice",
        10.0 * (voice / sub).log10()
    );
}

// --- DCS ------------------------------------------------------------------

/// A 23-bit Golay codeword, streamed out LSB first over and over the way a DCS
/// transmitter does. The code the word carries is deliberately not the point —
/// see `SubTone::Dcs` — so this just needs to be a real codeword of a real
/// word's weight.
fn dcs_bits(info: u32, inverted: bool, words: usize) -> Vec<bool> {
    let word = sdroxide_dsp::golay23_encode(info);
    let word = if inverted { !word & 0x7F_FFFF } else { word };
    (0..words * 23).map(|i| word & (1 << (i % 23)) != 0).collect()
}

/// 134.4 bps of DCS data at the deviation a transmitter uses for it.
fn dcs_signal(info: u32, inverted: bool, secs: f64, with_voice: bool) -> Vec<C32> {
    let bits = dcs_bits(info, inverted, (secs * 134.4 / 23.0).ceil() as usize + 2);
    modulate(secs, 0.0, |t| {
        let idx = (t * 134.4) as usize;
        let level = if bits[idx.min(bits.len() - 1)] { 1.0 } else { -1.0 };
        let voice = if with_voice { VOICE_DEV * programme(t) } else { 0.0 };
        TONE_DEV * level + voice
    })
}

/// A DCS stream is recognised as one, in either polarity — the complement of a
/// Golay codeword is a Golay codeword, and both repeat every 23 bits.
#[test]
fn a_dcs_stream_is_recognised_in_both_polarities() {
    for info in [0x213u32, 0x0AC, 0x1F5] {
        for inverted in [false, true] {
            let (tone, _) = demodulate(&dcs_signal(info, inverted, 2.0, false));
            assert_eq!(tone, Some(SubTone::Dcs), "info={info:#05x} inverted={inverted}");
        }
    }
}

#[test]
fn a_dcs_stream_survives_speech_on_top_of_it() {
    let (tone, _) = demodulate(&dcs_signal(0x213, false, 2.5, true));
    assert_eq!(tone, Some(SubTone::Dcs));
}

/// A DCS stream must not be mistaken for a CTCSS tone: its spectrum has plenty
/// of low-frequency energy, and a naive peak-picker would name whichever
/// standard tone happened to catch most of it.
#[test]
fn a_dcs_stream_is_never_reported_as_a_ctcss_tone() {
    for info in [0x213u32, 0x0AC] {
        let (tone, _) = demodulate(&dcs_signal(info, false, 2.5, true));
        assert_eq!(tone, Some(SubTone::Dcs), "info={info:#05x} came back as {tone:?}");
    }
}

/// An unmodulated carrier is perfectly periodic at every period there is, so it
/// would satisfy a naive repetition test. It is not DCS, and neither is it a
/// tone.
#[test]
fn an_unmodulated_carrier_reports_nothing() {
    let (tone, _) = demodulate(&modulate(2.0, 0.0, |_| 0.0));
    assert_eq!(tone, None);
}
