//! Does changing settings actually take effect without restarting?
//!
//! ```text
//! cargo run --release -p sdroxide-airspyhf --example reopen_check
//! ```
//!
//! Opens the receiver, streams, releases it, and opens it again at a *different*
//! sample rate — the sequence the engine performs when the operator presses
//! Apply. Then does it a few more times, so a leaked endpoint, an interface that
//! was never let go, or a receiver left with its `RECEIVER_MODE` on shows up
//! rather than passing once.
//!
//! Alternating between a zero-IF and a low-IF rate is the point: that is the
//! change that reprograms the synthesiser floor, re-reads the filter gain and
//! flips whether the host image balancer runs at all.

use std::time::{Duration, Instant};

use sdroxide_types::AirspyHfConfig;

fn drain(h: &mut sdroxide_airspyhf::AirspyHfHandle, secs: f64) -> (usize, f64) {
    let mut buf = vec![0f32; 1 << 16];
    let mut total = 0usize;
    let started = Instant::now();
    let deadline = started + Duration::from_secs_f64(secs);
    while Instant::now() < deadline {
        let n = h.rx_read(&mut buf);
        if n == 0 {
            std::thread::sleep(Duration::from_millis(2));
            continue;
        }
        total += n / 2;
    }
    (total, started.elapsed().as_secs_f64())
}

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sdroxide_airspyhf=warn".into()),
        )
        .init();

    // 768 k is zero-IF on every model; 384 k and 192 k are low-IF. Anything the
    // receiver does not have is snapped, which the run reports.
    let rates = [768_000.0, 384_000.0, 192_000.0, 768_000.0, 256_000.0];
    let center =
        std::env::var("PROBE_FREQ").ok().and_then(|s| s.parse::<f64>().ok()).unwrap_or(9_750_000.0);
    let mut ok = true;

    for (i, rate) in rates.iter().enumerate() {
        let cfg = AirspyHfConfig { sample_rate_hz: *rate, ..Default::default() };
        let t0 = Instant::now();
        let mut h = match sdroxide_airspyhf::AirspyHfHandle::open(&cfg, center) {
            Ok(h) => h,
            Err(e) => {
                println!(
                    "cycle {i}: open at {:.0} kSPS FAILED after {:.2} s: {e}",
                    rate / 1e3,
                    t0.elapsed().as_secs_f64()
                );
                ok = false;
                break;
            }
        };
        let open_ms = t0.elapsed().as_secs_f64() * 1000.0;
        let (n, dt) = drain(&mut h, 3.0);
        let measured = n as f64 / dt;
        let err = (measured / h.sample_rate_hz - 1.0) * 100.0;
        println!(
            "cycle {i}: {:.0} kSPS{} — opened in {open_ms:.0} ms, {n} samples in {dt:.2} s \
             ({measured:.0} sps, {err:+.2} %)",
            h.sample_rate_hz / 1e3,
            if h.snapped_from.is_some() { " (snapped)" } else { "" },
        );
        if n == 0 {
            println!("        NO SAMPLES — this is the failure this tool is for");
            ok = false;
        }
        // A rate that is out by more than a percent is not a clock error, it is
        // the wrong rate: the table index and the rate it stands for disagree.
        if n > 0 && err.abs() > 1.0 {
            println!("        the achieved rate does not match the nominal one");
            ok = false;
        }
        let t1 = Instant::now();
        h.release();
        println!("        released in {:.0} ms", t1.elapsed().as_secs_f64() * 1000.0);
    }

    println!();
    if ok {
        println!("All cycles reopened cleanly.");
    } else {
        println!("A cycle failed. Please attach the trace below to a bug report.");
        if let Some(d) = sdroxide_airspyhf::diagnostics() {
            println!();
            println!("{d}");
        }
        std::process::exit(1);
    }
}
