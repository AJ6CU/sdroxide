//! Open whatever LimeSDR is attached and print everything it reports.
//!
//! The first thing to build and the first thing to run against real hardware:
//! it settles the layout pins, the enumeration filter and every advertised
//! range before a single sample has to be right.
//!
//! With no board attached it still exercises the useful half — that LimeSuite
//! loads, that the symbols resolve, and whether this build has LimeRFE support.
//!
//! ```text
//! cargo run -p sdroxide-lime --example probe
//! ```

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sdroxide_lime=debug".into()),
        )
        .init();

    match sdroxide_lime::try_list() {
        Err(e) => {
            println!("LimeSuite unavailable: {e}");
            return;
        }
        Ok(found) => {
            println!("LimeSuite listed {} Lime board(s)", found.devices.len());
            for d in &found.devices {
                println!("  {}", d.label());
                println!("    device string: {}", d.info);
            }
            if !found.rejected.is_empty() {
                println!(
                    "\n{} device(s) LimeSuite listed that are not Lime boards, ignored:",
                    found.rejected.len()
                );
                for r in &found.rejected {
                    println!("  {r}");
                }
                println!(
                    "  (LimeSuite claims the bare Cypress FX3 id an unprogrammed RX-888 also \
                     presents; opening one would hand back a receiver that hears nothing)"
                );
            }
            if found.devices.is_empty() {
                println!(
                    "\nNothing to open. Is a board plugged in? Compare with `LimeUtil --find`."
                );
                return;
            }
        }
    }

    let cfg = sdroxide_types::LimeConfig::default();
    let mut handle = match sdroxide_lime::LimeHandle::open(&cfg, 145_500_000.0) {
        Ok(h) => h,
        Err(e) => {
            println!("\nopen failed: {e}");
            return;
        }
    };

    let info = handle.info().clone();
    println!("\n--- {} ---", handle.label());
    println!("firmware {}  hardware {}  gateware {}", info.firmware, info.hardware, info.gateware);
    println!("serial   {}", info.serial);
    println!("rate     {:.6} Msps", handle.sample_rate() / 1e6);
    println!("filter   {:.3} MHz", handle.analog_bw() / 1e6);
    if let Some(t) = handle.chip_temp_c() {
        println!("chip     {t:.1} C");
    }
    for (dir, tx) in [("rx", false), ("tx", true)] {
        match handle.lo_range(tx) {
            Ok(r) => {
                println!("{dir} LO   {:.3} – {:.3} MHz (step {})", r.min / 1e6, r.max / 1e6, r.step)
            }
            Err(e) => println!("{dir} LO   unavailable: {e}"),
        }
        match handle.rate_range(tx) {
            Ok(r) => println!("{dir} rate {:.3} – {:.3} Msps", r.min / 1e6, r.max / 1e6),
            Err(e) => println!("{dir} rate unavailable: {e}"),
        }
    }
    println!("rx ports {:?} (now {})", handle.antennas_rx(), handle.antenna_rx());
    println!("tx ports {:?} (now {})", handle.antennas_tx(), handle.antenna_tx());
    println!("rx gain  {} dB", handle.rx_gain_db());

    // A short read, to prove samples actually arrive and are not all zero.
    let mut buf = vec![num_complex::Complex32::new(0.0, 0.0); 8192];
    match handle.read_within(&mut buf, sdroxide_lime::handle::RX_TIMEOUT_MS) {
        Ok(0) => println!("\nread: timed out with no samples"),
        Ok(n) => {
            let peak = buf[..n].iter().map(|c| c.norm_sqr()).fold(0.0f32, f32::max).sqrt();
            println!("\nread: {n} samples, peak {peak:.5} full scale");
            if peak == 0.0 {
                println!("  (all zero — the stream is running but the front end is silent)");
            }
        }
        Err(e) => println!("\nread failed: {e}"),
    }
}
