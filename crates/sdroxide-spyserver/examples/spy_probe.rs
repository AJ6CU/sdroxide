//! Connect to a real SpyServer and report what both lanes actually deliver.
//!
//! The fake server in `tests/` proves this client against the protocol *as
//! transcribed*; it cannot prove the transcription. This does — point it at a
//! real server and it prints what the far end says about itself, what the two
//! streams arrive at, and where the strongest signal in the full-band FFT
//! sits. Tune the server to a known broadcast station and that last figure is
//! a direct check that the FFT's axis is right: a span taken from the wrong
//! field, or bins read in the wrong order, puts the carrier somewhere it
//! demonstrably is not.
//!
//! ```text
//! cargo run -p sdroxide-spyserver --example spy_probe -- 127.0.0.1:5555 93.2e6 [vfo]
//! ```

use std::time::{Duration, Instant};

use sdroxide_spyserver::SpyServerHandle;
use sdroxide_types::SpyServerConfig;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let mut args = std::env::args().skip(1);
    let address = args.next().unwrap_or_else(|| "127.0.0.1:5555".into());
    let center: f64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(93_200_000.0);
    let vfo = args.next().is_some_and(|s| s.eq_ignore_ascii_case("vfo"));

    match sdroxide_spyserver::test_connection(&address, Duration::from_secs(5)) {
        Ok(line) => println!("probe : {line}"),
        Err(e) => {
            eprintln!("probe : {e}");
            std::process::exit(1);
        }
    }

    // A narrowed FFT so the band view has slack to hold *within*. At stage 0
    // it covers the whole receiver and is pinned to the device centre, which
    // would make the hysteresis below untestable rather than working.
    let cfg = SpyServerConfig { address, fft_decimation: 2, ..SpyServerConfig::default() };
    let mut h = match if vfo {
        SpyServerHandle::connect_vfo(&cfg, center)
    } else {
        SpyServerHandle::connect_wideband(&cfg, center)
    } {
        Ok(h) => h,
        Err(e) => {
            eprintln!("connect: {e}");
            std::process::exit(1);
        }
    };

    let info = h.info;
    println!("device: {}", info.describe());
    println!(
        "ladder: {}",
        info.iq_rates(vfo)
            .iter()
            .map(|(i, r)| format!("{i}:{:.1}k", r / 1e3))
            .collect::<Vec<_>>()
            .join(" ")
    );
    // The two figures the FFT stage reports are *different*, and a client that
    // uses the first for the axis draws every signal at the wrong frequency.
    if let Some(&(i, label, span)) = info.fft_stages().first() {
        println!(
            "fft[{i}]: labelled {:.3} MHz, delivers {:.3} MHz  (they differ by {:.1}%)",
            label / 1e6,
            span / 1e6,
            (label - span) / label * 100.0,
        );
    }
    println!(
        "iq    : {:.3} ksps, stage {}, control {}",
        h.sample_rate_hz / 1e3,
        h.iq_decimation,
        h.can_control()
    );
    println!("fft   : {:.3} MHz span", h.fft_span_hz / 1e6);

    let start = Instant::now();
    let mut buf = vec![0f32; 1 << 16];
    let mut samples = 0u64;
    let mut frames = 0u64;
    let mut best: Option<(f64, f32, usize)> = None;

    while start.elapsed() < Duration::from_secs(6) {
        samples += h.rx_read(&mut buf) as u64 / 2;
        if let Some(f) = h.take_fft() {
            frames += 1;
            let (peak_i, peak_db) = f
                .bins
                .iter()
                .enumerate()
                .fold((0usize, f32::MIN), |a, (i, &v)| if v > a.1 { (i, v) } else { a });
            let n = f.bins.len().max(1) as f64;
            let hz = f.center_hz - f.span_hz / 2.0 + (peak_i as f64 + 0.5) / n * f.span_hz;
            best = Some((hz, peak_db, f.bins.len()));
        }
        std::thread::sleep(Duration::from_millis(2));
    }

    let secs = start.elapsed().as_secs_f64();
    println!(
        "\nrx    : {samples} samples in {secs:.1}s = {:.1} ksps (nominal {:.1})",
        samples as f64 / secs / 1e3,
        h.sample_rate_hz / 1e3,
    );
    println!("frames: {frames} FFT frames = {:.1}/s", frames as f64 / secs);
    match best {
        Some((hz, db, bins)) => println!(
            "peak  : {:.4} MHz at {db:.1} dB, {bins} bins  ({:+.1} kHz from the dial)",
            hz / 1e6,
            (hz - center) / 1e3,
        ),
        None => println!("peak  : no FFT frames arrived"),
    }

    // The servo, against a real server: walk the dial and watch the narrow
    // window follow while the band view holds. Both halves matter — a window
    // that does not follow is a receiver that stops hearing, and a band view
    // that follows every step is one nobody can steer by.
    println!("\ntuning: walking the dial {:+} kHz in 9 steps", 9 * 40);
    let mut fft_moves = 0;
    let mut last_fft = h.take_fft().map(|f| f.center_hz);
    for step in 1..=9 {
        let want = center + f64::from(step) * 40_000.0;
        h.set_center_hz(want);
        let deadline = Instant::now() + Duration::from_millis(400);
        while Instant::now() < deadline {
            let _ = h.rx_read(&mut buf);
            if let Some(f) = h.take_fft() {
                if last_fft.is_some_and(|c| (c - f.center_hz).abs() > 1.0) {
                    fft_moves += 1;
                    println!(
                        "        the band view re-centred on {:.4} MHz at step {step}",
                        f.center_hz / 1e6
                    );
                }
                last_fft = Some(f.center_hz);
            }
            std::thread::sleep(Duration::from_millis(5));
        }
    }
    let want = center + 360_000.0;
    println!(
        "iq    : asked for {:.4} MHz, the server reports its window on {:.4} MHz ({:+.1} kHz)",
        want / 1e6,
        h.iq_center_hz() / 1e6,
        (h.iq_center_hz() - want) / 1e3,
    );
    println!("fft   : the band view moved {fft_moves} time(s) over that walk");
    h.release();
}
