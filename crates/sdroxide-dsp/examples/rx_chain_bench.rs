//! Where the receive path spends a core, per device rate and decimation.
//!
//! ```text
//! cargo run --release -p sdroxide-dsp --example rx_chain_bench
//! ```
//!
//! The engine's receive loop is three stages over every block it reads
//! (`engine.rs`, "Blanking comes first"): decimate at the **device** rate, then
//! the analyzer and the channel DDC at the **decimated** rate. Only the first
//! scales with the front end; the other two scale with what the operator left
//! after decimation — which is why a fixed decimation factor makes a faster
//! device cost more everywhere, not just in its own driver.
//!
//! The rows are the configurations worth comparing when a wide receiver is
//! expensive: the same device rate at two factors, and the same *decimated*
//! rate reached from two device rates.

use std::time::Instant;

use sdroxide_dsp::{Complex32, Ddc, Decimator, SpectrumAnalyzer};

/// The engine's own read buffer size, so per-call overhead is charged the way
/// the engine actually pays it.
const BLOCK: usize = 16_384;
const ROUNDS: usize = 40;

/// `SpectrumConfig::default()`.
const FFT_SIZE: usize = 4096;
/// `channel_target` for everything except WFM.
const CHANNEL_HZ: f64 = 48_000.0;

fn main() {
    let mut state = 0x2545_f491_4f6c_dd1du64;
    let mut rng = move || {
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;
        (state >> 40) as i32 as f32 / 8_388_608.0
    };
    let input: Vec<Complex32> = (0..BLOCK).map(|_| Complex32::new(rng(), rng())).collect();

    println!(
        "{:>9} {:>6} {:>10} {:>10} {:>10} {:>10} {:>9}",
        "device", "decim", "decimate", "analyzer", "DDC", "chain", "of a core"
    );
    println!("{}", "-".repeat(72));

    for (device_hz, factor) in [(3.0e6, 16u32), (6.0e6, 16), (6.0e6, 32), (3.0e6, 8), (6.0e6, 64)] {
        let out_hz = device_hz / factor as f64;

        // Stage 1: at the device rate.
        let mut d = Decimator::new(factor);
        let mut dec_out = Vec::with_capacity(BLOCK);
        d.process(&input, &mut dec_out);
        let t = Instant::now();
        for _ in 0..ROUNDS {
            dec_out.clear();
            d.process(&input, &mut dec_out);
        }
        let decim_s = t.elapsed().as_secs_f64() / (ROUNDS * BLOCK) as f64 * device_hz;

        // Stages 2 and 3: at the decimated rate. Fed a block of the size the
        // decimator would hand over, so the FFT hop lands as it really does.
        let dec_block: Vec<Complex32> =
            input.iter().take(BLOCK / factor as usize).copied().collect();
        let per_call = dec_block.len();

        let mut an = SpectrumAnalyzer::new(FFT_SIZE, out_hz, 0.10);
        let t = Instant::now();
        for _ in 0..ROUNDS {
            an.process(&dec_block);
        }
        let an_s = t.elapsed().as_secs_f64() / (ROUNDS * per_call) as f64 * out_hz;

        let mut ddc = Ddc::new(out_hz, CHANNEL_HZ);
        ddc.set_offset_hz(out_hz / 8.0);
        let mut ch = Vec::with_capacity(per_call);
        let t = Instant::now();
        for _ in 0..ROUNDS {
            ch.clear();
            ddc.process(&dec_block, &mut ch);
        }
        let ddc_s = t.elapsed().as_secs_f64() / (ROUNDS * per_call) as f64 * out_hz;

        let total = decim_s + an_s + ddc_s;
        println!(
            "{:>7.1}M {:>6} {:>9.1}% {:>9.1}% {:>9.1}% {:>9.1}% {:>8}",
            device_hz / 1e6,
            factor,
            decim_s * 100.0,
            an_s * 100.0,
            ddc_s * 100.0,
            total * 100.0,
            format!("{:.0} kHz out", out_hz / 1e3),
        );
    }

    println!(
        "\nEach column is the fraction of one core that stage needs to keep up in\n\
         real time. The audio chain past the DDC runs at {:.0} kHz whatever the\n\
         device does, so it is not in this table.",
        CHANNEL_HZ / 1e3
    );
}
