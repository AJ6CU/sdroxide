//! Connect to a radio, stream IQ for a few seconds, then print the session
//! trace — the diagnostic a user with hardware can send back.
//!
//! Run: `cargo run -p sdroxide-smartsdr --example probe -- 192.168.1.50`
//!
//! Unlike `test_connection`, this *does* register as a GUI client and create
//! streams, so it will contend for the radio with a running SmartSDR unless
//! multiFLEX is enabled. It tears everything down on the way out.

use std::time::{Duration, Instant};

use sdroxide_smartsdr::FlexHandle;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,smartsdr=debug".into()),
        )
        .init();

    let addr = std::env::args().nth(1).unwrap_or_else(|| "127.0.0.1:4992".to_string());
    let rate: f64 = std::env::args().nth(2).and_then(|s| s.parse().ok()).unwrap_or(192_000.0);

    println!("connecting to {addr} at {} kHz IQ ...", rate / 1000.0);
    let mut h = match FlexHandle::connect(&addr, rate, 1, "sdroxide-probe") {
        Ok(h) => h,
        Err(e) => {
            eprintln!("\nconnect failed: {e}");
            eprintln!("\n{}", sdroxide_smartsdr::FIELD_REPORT_HINT);
            std::process::exit(1);
        }
    };

    println!("connected: {} v{}", h.ident.model, h.ident.version);
    println!("handle 0x{:08X}", h.ident.handle);

    let mut buf = vec![0f32; 16384];
    let mut samples = 0usize;
    let mut power = 0f64;
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        let n = h.rx_read(&mut buf);
        if n == 0 {
            std::thread::sleep(Duration::from_millis(5));
            continue;
        }
        samples += n;
        power += buf[..n].iter().map(|&v| (v as f64) * (v as f64)).sum::<f64>();
        for u in h.poll_updates() {
            println!("radio reported: {u:?}");
        }
    }

    let secs = 5.0;
    println!("\n--- receive summary ---");
    println!("IQ samples      : {samples} ({} pairs)", samples / 2);
    println!("effective rate  : {:.1} kHz", (samples / 2) as f64 / secs / 1000.0);
    println!("requested rate  : {:.1} kHz", h.requested_rate_hz / 1000.0);
    println!("radio is sending: {:.1} kHz", h.actual_rate_hz() / 1000.0);
    if samples > 0 {
        let mean = power / samples as f64;
        println!("mean power      : {mean:.3e}  ({:.1} dBFS)", 10.0 * mean.log10());
    } else {
        println!("NO IQ RECEIVED — the radio accepted the session but sent nothing.");
        println!("That usually means UDP from the radio is being dropped by a firewall.");
    }

    println!("\n{}", h.trace.dump());
    println!("{}", sdroxide_smartsdr::FIELD_REPORT_HINT);
}
