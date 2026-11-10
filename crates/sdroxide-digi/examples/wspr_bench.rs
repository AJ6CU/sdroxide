//! Synthesise a WSPR band of known stations, decode it, and say what was
//! missed and what was invented.
//!
//! This is where the tuned constants in `wspr::decode` come from, and the WAV
//! it writes is the point of it: WSJT-X's own `wsprd` reads the same file, so
//! the two decoders can be compared on identical audio rather than on each
//! other's claims. A decoder measured only against a signal generator written
//! by the same hand agrees with itself by construction.
//!
//! ```text
//! cargo run --release -p sdroxide-digi --example wspr_bench -- /tmp/band.wav
//! wsprd -f 14.0956 -w /tmp/band.wav        # the reference, same file
//! ```
//!
//! `WSPR_N` sets how many beacons to put in the 200 Hz window (default 10),
//! spread evenly and running from −6 dB down to −32; `WSPR_DRIFT` gives each
//! one that many hertz of drift across the transmission, alternating sign;
//! `WSPR_SEED` picks the noise realisation, so a claim can be checked over
//! several rather than tuned to one.

use sdroxide_digi::wspr::decode_slot;

const RATE: u32 = 12_000;
const SLOT: usize = 120 * RATE as usize;

/// xorshift64* — deterministic, no `rand` in this tree.
struct Rng(u64);
impl Rng {
    fn next_u32(&mut self) -> u32 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        (self.0.wrapping_mul(0x2545_F491_4F6C_DD1D) >> 32) as u32
    }
    fn unit(&mut self) -> f64 {
        (self.next_u32() as f64 + 0.5) / 4_294_967_296.0
    }
    fn gaussian(&mut self) -> f64 {
        let (u1, u2) = (self.unit(), self.unit());
        (-2.0 * u1.ln()).sqrt() * (std::f64::consts::TAU * u2).cos()
    }
}

/// Build a station list of `n` beacons spread across the window, with SNRs
/// running from strong down to below the floor. `drift` gives each one a
/// linear drift in Hz across the transmission, alternating sign.
fn beacons(n: usize, drift: f64) -> Vec<(String, String, i32, f64, f64, f64, f64)> {
    const GRIDS: &[&str] = &[
        "FN42", "IO91", "QE38", "PM95", "JO70", "FM19", "JN48", "IM98", "GF05", "RE68", "EM12",
        "KO85", "OF88", "DM04", "BP51",
    ];
    // 1400..1600 is the 200 Hz window; keep 10 Hz clear of each edge.
    let (lo, hi) = (1408.0, 1592.0);
    (0..n)
        .map(|i| {
            // K + digit + three letters: the Type-1 layout's own shape, so the
            // packer takes every one of them.
            let call = format!(
                "K{}A{}{}",
                i % 10,
                (b'A' + (i / 10) as u8) as char,
                (b'A' + (i % 26) as u8) as char
            );
            let grid = GRIDS[i % GRIDS.len()].to_string();
            let shift: f64 =
                std::env::var("WSPR_FSHIFT").ok().and_then(|v| v.parse().ok()).unwrap_or(0.0);
            let f0 = lo + (hi - lo) * (i as f64 / (n.max(2) - 1) as f64) + shift;
            // −6 dB at the top down to −32 at the bottom, so a run always
            // covers the whole range the operator cares about.
            let snr = match std::env::var("WSPR_SNR").ok().and_then(|v| v.parse::<f64>().ok()) {
                Some(fixed) => fixed,
                None => -6.0 - 26.0 * (i as f64 / (n.max(2) - 1) as f64),
            };
            let dt = ((i % 7) as f64 - 3.0) * 0.3;
            let d = if i % 2 == 0 { drift } else { -drift };
            (call, grid, 23, f0, snr, dt, d)
        })
        .collect()
}

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "/tmp/wspr_bench.wav".into());
    let n: usize = std::env::var("WSPR_N").ok().and_then(|v| v.parse().ok()).unwrap_or(10);
    let drift: f64 = std::env::var("WSPR_DRIFT").ok().and_then(|v| v.parse().ok()).unwrap_or(0.0);
    let list = beacons(n, drift);
    // Noise first: every beacon's amplitude is then derived from it, so the
    // reported SNRs are all referenced to the same floor.
    let sigma = 0.02f64;
    let seed: u64 =
        std::env::var("WSPR_SEED").ok().and_then(|v| v.parse().ok()).unwrap_or(0x5EED_1234);
    let mut rng = Rng(seed);
    let mut slot = vec![0f32; SLOT];
    for s in slot.iter_mut() {
        *s = (rng.gaussian() * sigma) as f32;
    }

    for (call, grid, pwr, f0, snr_db, dt, drift) in &list {
        // Noise power inside WSPR's 2500 Hz reference bandwidth, given real
        // audio spanning 0..6000 Hz.
        let n_ref = sigma * sigma * 2500.0 / 6000.0;
        let amp = (2.0 * n_ref * 10f64.powf(snr_db / 10.0)).sqrt() as f32;
        let symbols = sdroxide_digi::wspr::tx::symbols_for(call, grid, *pwr)
            .unwrap_or_else(|| panic!("{call} {grid} {pwr} is not sendable"));
        // Synthesised here rather than through `tx::synthesize` so the tone can
        // creep across the burst: a real beacon's oscillator does.
        let nsps = (RATE as f64 * 0.682_666_667).round() as usize;
        let mut phase = 0.0f64;
        let start = ((1.0 + dt) * RATE as f64) as usize;
        for (k, &sym) in symbols.iter().enumerate() {
            for j in 0..nsps {
                let frac = (k * nsps + j) as f64 / (symbols.len() * nsps) as f64;
                let f = f0 + (sym % 4) as f64 * 1.464_843_75 + drift * (frac - 0.5);
                phase += std::f64::consts::TAU * f / RATE as f64;
                let idx = start + k * nsps + j;
                if idx < SLOT {
                    slot[idx] += amp * phase.sin() as f32;
                }
            }
        }
    }

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: RATE,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut w = hound::WavWriter::create(&out, spec).expect("create wav");
    for &s in &slot {
        w.write_sample((s.clamp(-1.0, 1.0) * 32767.0) as i16).expect("write");
    }
    w.finalize().expect("finalize");
    if std::env::var("WSPR_QUIET").is_err() {
        eprintln!("wrote {out}");
    }

    let t0 = std::time::Instant::now();
    let decodes = decode_slot(&slot, RATE);
    let dur = t0.elapsed();

    let quiet = std::env::var("WSPR_QUIET").is_ok();
    let mut found = Vec::new();
    for d in &decodes {
        let call = d.message.to_string().split_whitespace().next().unwrap_or("").to_string();
        let real = list.iter().any(|b| b.0 == call);
        if !quiet {
            eprintln!(
                "{} {:>8.1} Hz  dt {:+5.2}  snr {:+4.0}  drift {:+3.0}  fit {:+.3}  {}",
                if real { " " } else { "!" },
                d.carrier_hz(),
                d.dt_sec,
                d.snr_db,
                d.drift_hz,
                d.fit,
                d.message
            );
        }
        found.push(call);
    }
    let mut hits = 0;
    for (call, _, _, _, snr, _, _) in &list {
        let ok = found.iter().any(|c| c == call);
        if ok {
            hits += 1;
        }
        if !quiet {
            eprintln!("  {:>4} {call:<8} @ {snr:+.0} dB", if ok { "ok" } else { "MISS" });
        }
    }
    let false_pos = found.iter().filter(|c| !list.iter().any(|b| &&b.0 == c)).count();
    eprintln!(
        "--- sdroxide: recall {hits}/{}  false {false_pos}  in {:.1} s",
        list.len(),
        dur.as_secs_f64()
    );
}
