//! Does changing settings actually take effect without restarting?
//!
//! ```text
//! cargo run --release -p sdroxide-rx888 --example reopen_check
//! ```
//!
//! Opens the receiver, streams, releases it, and opens it again at a *different*
//! ADC clock — the sequence the engine performs when the operator changes a
//! setting that needs the DSP chain rebuilt. Then does it a few more times, so a
//! leak or a device left in a bad state shows up rather than passing once.

use std::time::{Duration, Instant};

use sdroxide_rx888::{Settings, spawn};

fn drain(h: &mut sdroxide_rx888::Rx888Handle, secs: f64) -> (usize, f64) {
    let mut buf = vec![0f32; 1 << 16];
    let mut total = 0usize;
    let started = Instant::now();
    let deadline = started + Duration::from_secs_f64(secs);
    while Instant::now() < deadline {
        let n = h.read(&mut buf);
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
                .unwrap_or_else(|_| "sdroxide_rx888=warn".into()),
        )
        .init();

    // Alternate rates so each cycle really does reprogram the Si5351 and resize
    // the downconverter, rather than reopening at the settings it already had.
    let rates = [64.8e6, 32.4e6, 129.6e6, 64.8e6, 16.2e6];
    let mut ok = true;

    for (i, rate) in rates.iter().enumerate() {
        let settings = Settings { adc_rate_hz: *rate, vga_db: 18.0, ..Default::default() };
        let t0 = Instant::now();
        let mut h = match spawn(&settings, 13.635e6) {
            Ok(h) => h,
            Err(e) => {
                println!("cycle {i}: open at {:.1} Msps FAILED: {e}", rate / 1e6);
                ok = false;
                continue;
            }
        };
        let open_ms = t0.elapsed().as_secs_f64() * 1e3;
        let (n, secs) = drain(&mut h, 1.5);
        let achieved = n as f64 / secs;
        let nominal = h.out_rate_hz();
        let pct = 100.0 * achieved / nominal;
        let wide = h.take_wide_spectrum().map(|f| f.bins.len()).unwrap_or(0);

        println!(
            "cycle {i}: ADC {:>6.1} Msps -> out {:.4} Msps | open {open_ms:5.0} ms | \
             got {pct:5.1}% | wide bins {wide}",
            h.adc_rate_hz() / 1e6,
            nominal / 1e6,
        );
        if let Some(w) = h.warning() {
            println!("          warning: {w}");
        }
        // Below 60 % something is wrong: the clamp for a slow link aside, a
        // reopened receiver should stream as well as a freshly opened one.
        if pct < 60.0 {
            ok = false;
            println!("          ^ too few samples after reopen");
        }
        h.release();
        // No sleep between cycles on purpose: the engine's reopen path does not
        // pause either, and a device that needs a breather would be a bug.
    }

    println!("\n{}", if ok { "all cycles OK" } else { "FAILURES ABOVE" });
    if !ok {
        std::process::exit(1);
    }
}
