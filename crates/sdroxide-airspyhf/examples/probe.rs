//! Identify an Airspy HF+ and print everything a first bug report needs.
//!
//! ```text
//! cargo run -p sdroxide-airspyhf --example probe
//! PROBE_FREQ=9750000 cargo run -p sdroxide-airspyhf --example probe
//! ```
//!
//! This backend was written from the reference implementation rather than on a
//! bench, so this tool exists to be *pasted into an issue*. It lists the bus,
//! opens the first receiver, streams briefly, and dumps the whole session trace
//! — every vendor request with its reply length, the flash page, the tuning
//! arithmetic, and the first bulk bytes decoded as `(re, im)` pairs.
//!
//! Point the receiver at a known strong carrier before running it (`PROBE_FREQ`,
//! in Hz — a local broadcast station is ideal). The decoded sample line is what
//! settles whether the samples really do arrive Q before I, which is the one
//! thing this driver cannot check for itself.

use std::time::{Duration, Instant};

use sdroxide_types::AirspyHfConfig;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sdroxide_airspyhf=info".into()),
        )
        .init();

    let devices = sdroxide_airspyhf::list();
    if devices.is_empty() {
        println!("No Airspy HF+ receivers found on USB.");
        println!();
        println!("If one is plugged in, this is usually a permissions problem:");
        println!("  Linux   — install 60-sdroxide-airspyhf.rules, then re-plug it");
        println!("  Windows — bind the device to WinUSB with Zadig");
        println!("  macOS   — nothing to install; quit any other SDR software first");
        return;
    }

    println!("{} Airspy HF+ receiver(s):", devices.len());
    for (i, d) in devices.iter().enumerate() {
        println!("  {i}: {}", d.label());
    }

    let center =
        std::env::var("PROBE_FREQ").ok().and_then(|s| s.parse::<f64>().ok()).unwrap_or(9_750_000.0);

    println!();
    println!("Opening at {:.3} kHz…", center / 1e3);
    let cfg = AirspyHfConfig::default();
    let mut handle = match sdroxide_airspyhf::AirspyHfHandle::open(&cfg, center) {
        Ok(h) => h,
        Err(e) => {
            println!("FAILED: {e}");
            if let Some(d) = sdroxide_airspyhf::diagnostics() {
                println!("\n{d}");
            }
            std::process::exit(1);
        }
    };

    println!("  device   : {}", handle.label);
    println!("  model    : {}", handle.model.label());
    println!("  firmware : {:?}", handle.firmware);
    println!("  serial   : {:016X}", handle.board_serial);
    println!("  rate     : {:.0} kSPS", handle.sample_rate_hz / 1e3);
    if let Some(from) = handle.snapped_from {
        println!("             (snapped from the configured {:.0} kSPS)", from / 1e3);
    }
    println!(
        "  rates    : {}",
        handle
            .rates_hz
            .iter()
            .zip(&handle.low_if)
            .map(|(r, lo)| format!("{:.0}k{}", r / 1e3, if *lo { " low-IF" } else { "" }))
            .collect::<Vec<_>>()
            .join(", ")
    );
    println!(
        "  atten.   : 0 to -{:.0} dB in {:.1} dB steps",
        handle.att_max_db, handle.att_step_db
    );
    println!("  bias tee : {}", if handle.bias_tee_supported { "present" } else { "not offered" });
    println!(
        "  cal      : {} ppb ({})",
        handle.calibration_ppb,
        if handle.calibration_from_flash { "from the receiver's flash" } else { "not calibrated" }
    );

    // Stream long enough for the image balancer to have something to say.
    let secs = 12.0;
    println!();
    println!("Streaming for {secs:.0} s…");
    let mut buf = vec![0f32; 1 << 16];
    let mut total = 0usize;
    let mut peak = 0f32;
    let started = Instant::now();
    while started.elapsed().as_secs_f64() < secs {
        let n = handle.rx_read(&mut buf);
        if n == 0 {
            std::thread::sleep(Duration::from_millis(2));
            continue;
        }
        total += n / 2;
        for v in &buf[..n] {
            peak = peak.max(v.abs());
        }
    }
    let dt = started.elapsed().as_secs_f64();
    println!("  {total} samples in {dt:.2} s = {:.1} kSPS", total as f64 / dt / 1e3);
    println!("  nominal {:.1} kSPS", handle.sample_rate_hz / 1e3);
    println!("  peak |sample| {peak:.4} (1.0 is full scale)");
    match handle.balance_state() {
        Some((p, a, n)) => {
            println!("  image balancer: phase {p:+.6}, amplitude {a:+.6}, {n} updates")
        }
        None => println!("  image balancer: no update yet (low-IF rate, or too quiet / too short)"),
    }
    if total == 0 {
        println!();
        println!("NO SAMPLES ARRIVED. This is the interesting case — please report the trace.");
    }

    handle.release();

    println!();
    println!("--- paste everything below into the bug report ---");
    println!();
    match sdroxide_airspyhf::diagnostics() {
        Some(d) => println!("{d}"),
        None => println!("(no trace was recorded, which should not happen)"),
    }
}
