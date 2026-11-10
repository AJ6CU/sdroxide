//! List the RTL-SDR dongles on the USB bus.
//!
//! The field-diagnosis tool for this backend: it answers "does the machine see
//! the dongle at all, and may this user have it?" without involving the rest of
//! sdroxide. Run it with `cargo run -p sdroxide-rtlsdr --example probe`, and
//! add `RUST_LOG=sdroxide_rtlsdr=debug` for the register traffic.

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sdroxide_rtlsdr=info".into()),
        )
        .init();

    let devices = sdroxide_rtlsdr::list();
    if devices.is_empty() {
        println!("No RTL-SDR dongles found on USB.");
        println!();
        println!("If one is plugged in, this is usually a permissions problem:");
        println!("  Linux   — install the udev rule, then re-plug the dongle");
        println!("  Windows — bind the device to WinUSB with Zadig");
        return;
    }

    println!("{} RTL-SDR dongle(s):", devices.len());
    for (i, d) in devices.iter().enumerate() {
        println!("  {i}: {}", d.label());
        println!("      usb {:04x}:{:04x}", d.vid, d.pid);
    }

    // Open the first one and stream briefly, so the probe answers "does it
    // actually work" and not just "is it plugged in".
    println!();
    let cfg = sdroxide_types::RtlSdrConfig {
        sample_rate_hz: 2_400_000.0,
        tuner_gain_db: 30.0,
        ..Default::default()
    };
    let center = std::env::var("PROBE_FREQ")
        .ok()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(100_000_000.0);

    println!("Opening at {:.3} MHz…", center / 1e6);
    let mut handle = match sdroxide_rtlsdr::RtlSdrHandle::open(&cfg, center) {
        Ok(h) => h,
        Err(e) => {
            println!("FAILED: {e}");
            std::process::exit(1);
        }
    };

    println!("  device : {}", handle.label);
    println!("  tuner  : {}", handle.tuner);
    println!("  rate   : {:.6} Msps", handle.sample_rate_hz / 1e6);
    println!("  HF     : {}", if handle.hf_capable { "yes" } else { "no" });
    println!("  gains  : {:?} dB", handle.gains_db);

    // Optionally dump raw interleaved f32 I/Q, so the capture can be compared
    // against `rtl_sdr` output at the same settings. That comparison is what
    // catches IF sign, spectrum inversion and I/Q swap — the failure modes
    // where every register is individually correct.
    let mut dump = std::env::var("PROBE_DUMP").ok().map(|p| {
        std::io::BufWriter::new(std::fs::File::create(p).expect("cannot create the dump file"))
    });

    let mut buf = vec![0f32; 1 << 16];
    let mut total = 0usize;
    let mut peak = 0f32;
    let mut sum_sq = 0f64;
    let start = std::time::Instant::now();
    while start.elapsed() < std::time::Duration::from_secs(3) {
        let n = handle.rx_read(&mut buf);
        if n == 0 {
            std::thread::sleep(std::time::Duration::from_millis(2));
            continue;
        }
        total += n / 2;
        for v in &buf[..n] {
            peak = peak.max(v.abs());
            sum_sq += (*v as f64) * (*v as f64);
        }
        if let Some(w) = dump.as_mut() {
            use std::io::Write;
            for v in &buf[..n] {
                let _ = w.write_all(&v.to_le_bytes());
            }
        }
    }
    // Retune stress: hammer the control channel the way dragging a panadapter
    // does, and check the sample stream survives it. This is the acceptance
    // test for the Ctrl coalescing in `stream.rs` — without it the thread
    // falls behind the dial and the stream starves.
    if std::env::var("PROBE_STRESS").is_ok() {
        println!();
        println!("Retune stress: 60 s of continuous tuning…");
        let t0 = std::time::Instant::now();
        let mut sent = 0u64;
        let mut got = 0usize;
        let mut worst_gap = std::time::Duration::ZERO;
        let mut last_sample = std::time::Instant::now();
        while t0.elapsed() < std::time::Duration::from_secs(60) {
            // Sweep like a mouse drag: many small steps, fast.
            let phase = t0.elapsed().as_secs_f64();
            let f = center + (phase * 3.0).sin() * 2_000_000.0;
            handle.set_center_hz(f);
            sent += 1;
            let n = handle.rx_read(&mut buf);
            if n > 0 {
                got += n / 2;
                last_sample = std::time::Instant::now();
            } else {
                worst_gap = worst_gap.max(last_sample.elapsed());
                std::thread::sleep(std::time::Duration::from_millis(1));
            }
        }
        let secs = t0.elapsed().as_secs_f64();
        println!("  sent {sent} retune requests ({:.0}/s)", sent as f64 / secs);
        println!(
            "  streamed {got} samples ({:.1} ksps, {:.1}% of nominal)",
            got as f64 / secs / 1e3,
            got as f64 / secs / handle.sample_rate_hz * 100.0
        );
        println!("  worst gap between samples: {:.0} ms", worst_gap.as_secs_f64() * 1000.0);
        if !handle.is_alive() {
            println!("  THREAD DIED under stress.");
            std::process::exit(1);
        }
    }

    let secs = start.elapsed().as_secs_f64();
    let rms = (sum_sq / (total.max(1) * 2) as f64).sqrt();
    println!();
    println!("  streamed {total} complex samples in {secs:.2} s");
    println!(
        "  measured {:.1} ksps (nominal {:.1})",
        total as f64 / secs / 1e3,
        handle.sample_rate_hz / 1e3
    );
    println!("  peak |sample| {peak:.4}, rms {rms:.4}");
    if total == 0 {
        println!("  NO SAMPLES — the device opened but never delivered anything.");
        std::process::exit(1);
    }
    if peak == 0.0 {
        println!("  ALL ZERO — samples are flowing but the ADC output is dead.");
        std::process::exit(1);
    }
}
