//! What the host conversion costs, per stage, as a fraction of one core.
//!
//! ```text
//! cargo run --release -p sdroxide-airspy --example convert_bench
//! ```
//!
//! This receiver's whole DSP burden is on the host — unpacking 12-bit samples
//! and turning a real stream into complex baseband — and it is the only backend
//! whose cost scales with the *ADC* rate rather than the rate an operator
//! picked. A Mini at 3 Msps complex is 6 Msps of arithmetic; an R2 at 10 is 20.
//!
//! The figure that matters is the last column: what fraction of one core each
//! stage needs to keep up in real time. Everything downstream of this — the
//! spectrum, the demodulator, the audio — has to fit in what is left.

use std::time::Instant;

use sdroxide_airspy::convert::HostDsp;
use sdroxide_airspy::protocol::unpack12_bytes;
use sdroxide_dsp::Complex32;

/// Long enough to swamp the timer and to run warm, short enough to stay in a
/// cache a stream would also be using.
const RAW_SAMPLES: usize = 1 << 20;
const ROUNDS: usize = 20;

fn main() {
    // A noise-like stream at mid-scale, which is what a real one looks like to
    // this arithmetic: no branch here depends on the values, but a degenerate
    // input can flatter a benchmark through the denormal path.
    let mut state = 0x2545_f491_4f6c_dd1du64;
    let mut rng = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        state
    };
    let raw: Vec<u16> =
        (0..RAW_SAMPLES).map(|_| 2048u16.wrapping_add((rng() % 512) as u16) & 0x0fff).collect();
    let packed: Vec<u8> = (0..RAW_SAMPLES * 3 / 2).map(|_| rng() as u8).collect();

    println!("{:<28} {:>12} {:>12} {:>16}", "stage", "ns/sample", "Msample/s", "core @ 6/12 Msps");
    println!("{}", "-".repeat(72));

    // --- unpacking -------------------------------------------------------
    let mut out = Vec::with_capacity(RAW_SAMPLES * 2);
    let t = Instant::now();
    for _ in 0..ROUNDS {
        out.clear();
        unpack12_bytes(&packed, &mut out);
    }
    let per = t.elapsed().as_secs_f64() / (ROUNDS * out.len()) as f64;
    report("unpack12 (packed wire)", per);

    // --- the real-to-complex chain ---------------------------------------
    let mut dsp = HostDsp::new();
    let mut iq: Vec<Complex32> = Vec::with_capacity(RAW_SAMPLES);
    let t = Instant::now();
    for _ in 0..ROUNDS {
        iq.clear();
        dsp.process(&raw, &mut iq);
    }
    let per = t.elapsed().as_secs_f64() / (ROUNDS * raw.len()) as f64;
    report("HostDsp::process", per);

    // --- the interleave the stream does before the ring -------------------
    let mut inter: Vec<f32> = Vec::with_capacity(RAW_SAMPLES);
    let t = Instant::now();
    for _ in 0..ROUNDS {
        inter.clear();
        for s in &iq {
            inter.push(s.re);
            inter.push(s.im);
        }
    }
    // Charged per *input* sample, so the column compares like with like.
    let per = t.elapsed().as_secs_f64() / (ROUNDS * iq.len() * 2) as f64;
    report("interleave for the ring", per);

    println!(
        "\n(a checksum so nothing above is optimised away: {:.3}, {}, {:.3})",
        iq.iter().map(|s| s.re).sum::<f32>(),
        out.len(),
        inter.iter().sum::<f32>(),
    );
}

/// One row: cost per *input* (ADC) sample, and what that is as a fraction of
/// one core at the two rates that matter — a Mini at 3 Msps complex and an R2
/// at 6, both expressed as the real rate the ADC actually runs at.
fn report(stage: &str, per_sample_s: f64) {
    println!(
        "{stage:<28} {:>12.3} {:>12.1} {:>15}",
        per_sample_s * 1e9,
        1e-6 / per_sample_s,
        format!("{:.0}% / {:.0}%", per_sample_s * 6.0e6 * 100.0, per_sample_s * 12.0e6 * 100.0),
    );
}
