//! End-to-end check of the full backend: USB thread, converter thread, and the
//! ring the engine will read from.
//!
//! ```text
//! cargo run --release -p sdroxide-rx888 --example stream_check -- 13.635
//! ```
//!
//! Reports the achieved complex rate against the nominal one. That ratio is the
//! whole point: it is the number that says whether the two threads keep up with
//! 129.6 MB/s in real time, which no amount of offline replay can tell you.

use std::f64::consts::TAU;
use std::time::{Duration, Instant};

use sdroxide_rx888::{Settings, spawn};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sdroxide_rx888=info".into()),
        )
        .init();

    let mhz: f64 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(13.635);
    let secs: f64 = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(5.0);

    let settings = Settings { vga_db: 18.0, ..Default::default() };
    let mut h = match spawn(&settings, mhz * 1e6) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("failed to start: {e}");
            std::process::exit(1);
        }
    };
    if let Some(w) = h.warning() {
        println!("WARNING: {w}");
    }
    println!(
        "{} (serial {})\n  ADC {:.3} Msps real -> {:.4} Msps complex, tuning grid {:.2} kHz",
        h.label(),
        h.serial().unwrap_or("none"),
        h.adc_rate_hz() / 1e6,
        h.out_rate_hz() / 1e6,
        h.bin_hz() / 1e3,
    );
    println!("  tuned to {mhz:.4} MHz, reading for {secs:.1} s\n");

    let mut buf = vec![0f32; 1 << 16];
    let mut total = 0usize;
    let mut sum_sq = 0f64;
    let mut peak = 0f32;
    // Keep a slice of samples for a frequency check.
    let mut keep: Vec<(f32, f32)> = Vec::with_capacity(1 << 16);

    let started = Instant::now();
    let deadline = started + Duration::from_secs_f64(secs);
    while Instant::now() < deadline {
        let n = h.read(&mut buf);
        if n == 0 {
            std::thread::sleep(Duration::from_millis(2));
            continue;
        }
        total += n / 2;
        for c in buf[..n].chunks_exact(2) {
            let (re, im) = (c[0], c[1]);
            let p = (re * re + im * im) as f64;
            sum_sq += p;
            peak = peak.max(p.sqrt() as f32);
            if keep.len() < keep.capacity() {
                keep.push((re, im));
            }
        }
    }
    let elapsed = started.elapsed().as_secs_f64();
    let achieved = total as f64 / elapsed;

    println!("read {total} complex samples in {elapsed:.2} s");
    println!(
        "  achieved {:.4} Msps = {:.1}% of nominal",
        achieved / 1e6,
        100.0 * achieved / h.out_rate_hz()
    );
    println!("  dropped by the ring: {}", h.dropped());
    let rms = (sum_sq / total.max(1) as f64).sqrt();
    println!(
        "  rms {:.1} dBFS, peak {:.1} dBFS",
        20.0 * rms.max(1e-12).log10(),
        20.0 * peak.max(1e-12).log10()
    );

    // Where the energy sits inside the channel — a carrier should sit at DC if
    // the tuning is right.
    if keep.len() > 4096 {
        let fs = h.out_rate_hz();
        let mut best = (0.0f64, 0.0f32);
        for s in -240i32..=240 {
            let f = s as f64 * fs / 2.0 / 240.0 * 0.75;
            let (mut re, mut im) = (0.0f32, 0.0f32);
            for (i, (a, b)) in keep.iter().enumerate() {
                let ph = -TAU * f * i as f64 / fs;
                let (c, d) = (ph.cos() as f32, ph.sin() as f32);
                re += a * c - b * d;
                im += a * d + b * c;
            }
            let mag = (re * re + im * im).sqrt() / keep.len() as f32;
            if mag > best.1 {
                best = (f, mag);
            }
        }
        println!(
            "  strongest component {:+.1} kHz from centre = {:.4} MHz at {:.1} dBFS",
            best.0 / 1e3,
            (mhz * 1e6 + best.0) / 1e6,
            20.0 * best.1.max(1e-12).log10()
        );
    }

    // The full-band lane: prove it produces frames, and that the strongest
    // things in them land where they physically are.
    //
    // The axis comes off the frame rather than from the handle, because above
    // the VHF crossover it is a slice of the tuner's IF mapped back to RF and
    // moves with the tuner — `wide_span_hz` only describes the HF case.
    let mut frames = 0usize;
    let mut last: Option<sdroxide_rx888::handle::WideFrame> = None;
    let until = Instant::now() + Duration::from_secs(2);
    while Instant::now() < until {
        if let Some(f) = h.take_wide_spectrum() {
            frames += 1;
            last = Some(f);
        }
        let _ = h.read(&mut buf);
    }
    println!("  {frames} frames in 2 s ({:.1} fps)", frames as f64 / 2.0);
    if let Some(frame) = last {
        let (wc, wsp) = (frame.center_hz, frame.span_hz);
        println!("\nfull-band lane: centre {:.3} MHz, span {:.3} MHz", wc / 1e6, wsp / 1e6);
        let f = frame.bins;
        let lo = wc - wsp / 2.0;
        let bin_hz = wsp / (f.len() - 1) as f64;
        let floor = {
            let mut v = f.clone();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v[v.len() / 2]
        };
        println!("  {} bins, {:.1} kHz each, median floor {floor:.1} dBFS", f.len(), bin_hz / 1e3);
        // What the engine's auto-ranger would choose for this frame. Mirrors
        // `auto_levels`: DC region skipped, floor at the 10th percentile less
        // 4 dB, ceiling at the strongest bin plus 6.
        {
            let skip = (f.len() / 512).max(1);
            let mut v: Vec<f32> = f[skip..].to_vec();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            let p10 = v[((v.len() - 1) as f64 * 0.10).round() as usize];
            let auto = (p10 - 4.0, v[v.len() - 1] + 6.0);
            println!(
                "  auto-range would show {:.1} … {:.1} dBFS ({:.0} dB window)",
                auto.0,
                auto.1,
                auto.1 - auto.0
            );
        }
        let mut idx: Vec<usize> = (1..f.len() - 1)
            .filter(|&i| f[i] > floor + 25.0 && f[i] >= f[i - 1] && f[i] >= f[i + 1])
            .collect();
        idx.sort_by(|a, b| f[*b].partial_cmp(&f[*a]).unwrap());
        println!("  strongest carriers:");
        for i in idx.into_iter().take(8) {
            println!("    {:9.4} MHz  {:6.1} dBFS", (lo + i as f64 * bin_hz) / 1e6, f[i]);
        }
    }

    h.release();
    println!("\nreleased cleanly, alive = {}", h.is_alive());
}
