//! Bring-up and field-diagnosis tool for the RX-888 backend.
//!
//! ```text
//! cargo run -p sdroxide-rx888 --example probe
//! RUST_LOG=sdroxide_rx888=debug cargo run -p sdroxide-rx888 --example probe
//! ```
//!
//! Enumerates, uploads firmware if the receiver is still in its boot ROM, then
//! reports what the device says about itself. Nothing here writes to anything
//! persistent: the firmware goes to RAM, so unplugging the receiver undoes
//! everything this program does.

use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::{Duration, Instant};

use nusb::transfer::{Bulk, In};

use sdroxide_rx888::protocol::{Cmd, Identity, STATS_LEN, Stats};
use sdroxide_rx888::usb::{self, speed_name, usable_bytes_per_sec};
use sdroxide_rx888::{convert, device, protocol};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sdroxide_rx888=info".into()),
        )
        .init();

    let found = usb::list();
    if found.is_empty() {
        eprintln!("No RX-888 found on USB.");
        eprintln!(
            "Expected 04b4:00f3 (boot ROM) or 04b4:00f1 (programmed). \
             Check the cable and, on Linux, the udev rule."
        );
        std::process::exit(1);
    }

    println!("Found {} receiver(s):", found.len());
    for (i, d) in found.iter().enumerate() {
        println!(
            "  {i}: {}  [{}]",
            d.label(),
            if d.superspeed { "SuperSpeed" } else { "not SuperSpeed (normal in the boot ROM)" }
        );
    }

    println!("\nOpening (uploading firmware if needed)...");
    let dev = match usb::open(None, None) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("\nFailed: {e}");
            std::process::exit(1);
        }
    };
    println!("Opened: {} (serial {})", dev.label(), dev.serial().unwrap_or("none"));

    // Only the *programmed* link speed means anything. The FX3 boot ROM always
    // comes up at USB 2.0 high speed even on a perfectly good SuperSpeed cable
    // and port; the application firmware is what re-enumerates at SuperSpeed.
    // Judging the cable by the bootloader's speed says a receiver is crippled
    // when it is fine.
    let speed = dev.speed();
    let budget = usable_bytes_per_sec(speed);
    println!("\nLink: {} — about {:.0} MB/s of bulk budget", speed_name(speed), budget / 1e6);
    println!("  that is roughly {:.1} Msps of 16-bit real samples,", budget / 2.0 / 1e6);
    println!("  i.e. about {:.1} MHz of HF spectrum.", budget / 2.0 / 2.0 / 1e6);
    if !matches!(speed, Some(nusb::Speed::Super | nusb::Speed::SuperPlus)) {
        println!(
            "  NOTE: this is not a SuperSpeed link. Use a USB 3 cable (the wide\n\
             \x20       double-connector micro-B) directly into a USB 3 port to get\n\
             \x20       the full 32 MHz."
        );
    }

    // TESTFX3 is the cheapest proof that the firmware is alive and speaking.
    match dev.vendor_in(Cmd::TestFx3, 0, 0, 4) {
        Ok(b) => {
            println!("\nTESTFX3 -> {b:02x?}");
            match Identity::parse(&b) {
                Some(id) => {
                    println!("  firmware version {}", id.version);
                    println!(
                        "  hardware id      0x{:02x} ({})",
                        id.hardware,
                        if id.front_end_ready() {
                            "RX-888 Mk2 — front-end controls initialised"
                        } else {
                            "NOT recognised — bias tee, dither, ADC range and \
                             both gain stages will do nothing"
                        }
                    );
                }
                None => println!("  (short reply)"),
            }
        }
        Err(e) => println!("\nTESTFX3 failed: {e}"),
    }

    match dev.vendor_in(Cmd::GetStats, 0, 0, STATS_LEN) {
        Ok(b) => {
            println!("\nGETSTATS -> {} bytes {b:02x?}", b.len());
            match Stats::parse(&b) {
                Some(s) => {
                    println!("  dma_count      {}", s.dma_count);
                    println!(
                        "  gpif_state     {} ({})",
                        s.gpif_state,
                        match s.gpif_state {
                            1 => "idle",
                            255 => "ERROR",
                            _ => "running",
                        }
                    );
                    println!("  pib_errors     {}", s.pib_errors);
                    println!("  i2c_errors     {}", s.i2c_errors);
                    println!("  stream_faults  {}", s.stream_faults);
                    println!("  boot_count     {}", s.boot_count);
                    println!("  si5351 status  0x{:02x}", s.si5351_status);
                    println!(
                        "  si5351 clk0    ctrl=0x{:02x} enabled={} -> {}",
                        s.si5351_clk0_control,
                        s.clk0_enabled,
                        if s.clock_running() { "RUNNING" } else { "stopped" }
                    );
                    match s.gpio {
                        Some(g) => println!("  gpio           0x{g:08x}"),
                        None => println!("  gpio           (not reported by this firmware)"),
                    }
                }
                None => println!("  (block too short to decode)"),
            }
        }
        Err(e) => println!("\nGETSTATS failed: {e}"),
    }

    // Deliberately *not* sending STOPFX3 here. Stopping a GPIF engine that was
    // never started leaves it in state 255, and it does not come back without a
    // reboot — an earlier version of this program did exactly that and then
    // spent a while blaming the host for the STARTFX3 that followed.
    if std::env::var_os("PROBE_RESET").is_some() {
        println!("\nPROBE_RESET: resetting to the boot ROM so firmware reloads clean");
        let _ = dev.vendor_out(Cmd::ResetFx3, 0, 0, &[]);
        drop(dev);
        std::thread::sleep(Duration::from_millis(1500));
    } else {
        drop(dev);
        std::thread::sleep(Duration::from_millis(100));
    }

    stream_test();
}

/// Bring the receiver fully up and pull samples for a few seconds.
///
/// `PROBE_SECS` sets the duration, `PROBE_RATE` the ADC clock in Msps,
/// `PROBE_NORAND` disables the ADC randomizer (so the two paths can be compared
/// against each other), and `PROBE_DUMP` writes the raw `i16` stream to a file
/// for offline analysis.
fn stream_test() {
    let secs: f64 = env_f64("PROBE_SECS", 2.0);
    let rate_msps = env_f64("PROBE_RATE", device::DEFAULT_ADC_HZ / 1e6);
    let randomize = std::env::var_os("PROBE_NORAND").is_none();

    let settings =
        device::Settings { adc_rate_hz: rate_msps * 1e6, randomize, ..Default::default() };

    println!(
        "\n--- streaming for {secs:.1} s at {rate_msps:.1} Msps (randomizer {}) ---",
        if randomize { "on" } else { "off" }
    );

    let mut dev = match device::Device::open(&settings, None) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("open failed: {e}");
            return;
        }
    };
    if let Some(w) = dev.warning() {
        println!("WARNING: {w}");
    }
    let adc_hz = dev.adc_rate_hz();
    println!(
        "ADC clock {:.3} Msps -> {:.1} MHz of spectrum, {:.1} MB/s expected",
        adc_hz / 1e6,
        adc_hz / 2e6,
        adc_hz * 2.0 / 1e6
    );

    match dev.stats() {
        Ok(s) => println!(
            "before start: gpif={} boot={} clk0 {} gpio 0x{:08x}",
            s.gpif_state,
            s.boot_count,
            if s.clock_running() { "RUNNING" } else { "stopped" },
            dev.gpio_word(),
        ),
        Err(e) => println!("stats failed: {e}"),
    }

    const IN_FLIGHT: usize = 16;
    const XFER: usize = 256 * 1024;

    // Order matters here, and not in the obvious way: see the note in
    // `Device::start`. STARTFX3 goes first, before the bulk endpoint exists.
    if let Err(e) = dev.start() {
        eprintln!("STARTFX3 failed: {e}");
        return;
    }

    let mut ep = match dev.usb().interface().endpoint::<Bulk, In>(protocol::BULK_EP) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("cannot open the bulk endpoint: {e}");
            return;
        }
    };
    for _ in 0..IN_FLIGHT {
        ep.submit(ep.allocate(XFER));
    }

    if std::env::var_os("PROBE_SWEEP").is_some() {
        sweep(&mut dev, &mut ep, randomize);
        ep.cancel_all();
        let until = Instant::now() + Duration::from_millis(500);
        while ep.pending() > 0 && Instant::now() < until {
            if ep.wait_next_complete(Duration::from_millis(50)).is_none() {
                break;
            }
        }
        let _ = dev.stop();
        println!("\nDone.");
        return;
    }

    let mut dump = std::env::var_os("PROBE_DUMP")
        .map(|p| BufWriter::new(File::create(p).expect("cannot create PROBE_DUMP file")));

    let mut bytes = 0usize;
    let mut samples = 0usize;
    let mut errors = 0usize;
    let mut carry = None;
    let mut scratch: Vec<f32> = Vec::with_capacity(XFER);
    let mut peak = 0f32;
    let mut sum_sq = 0f64;
    let mut sum = 0f64;

    let started = Instant::now();
    let deadline = started + Duration::from_secs_f64(secs);
    while Instant::now() < deadline {
        while ep.pending() < IN_FLIGHT {
            ep.submit(ep.allocate(XFER));
        }
        let Some(c) = ep.wait_next_complete(Duration::from_millis(200)) else {
            continue;
        };
        match c.status {
            Ok(()) => {
                let raw = &c.buffer[..];
                bytes += raw.len();
                if let Some(w) = dump.as_mut() {
                    let _ = w.write_all(raw);
                }
                convert::to_f32(raw, randomize, &mut carry, &mut scratch);
                samples += scratch.len();
                for v in &scratch {
                    peak = peak.max(v.abs());
                    sum += *v as f64;
                    sum_sq += (*v as f64) * (*v as f64);
                }
            }
            Err(e) => {
                errors += 1;
                if errors < 5 {
                    eprintln!("transfer error: {e}");
                }
            }
        }
        let mut buf = c.buffer;
        buf.clear();
        ep.submit(buf);
    }
    let elapsed = started.elapsed().as_secs_f64();

    ep.cancel_all();
    let until = Instant::now() + Duration::from_millis(500);
    while ep.pending() > 0 && Instant::now() < until {
        if ep.wait_next_complete(Duration::from_millis(50)).is_none() {
            break;
        }
    }
    let _ = dev.stop();

    let mb = bytes as f64 / 1e6;
    let rms = (sum_sq / samples.max(1) as f64).sqrt();
    let mean = sum / samples.max(1) as f64;
    println!("\nreceived {mb:.1} MB in {elapsed:.2} s = {:.1} MB/s", mb / elapsed);
    println!(
        "  {samples} samples = {:.2} Msps (expected {:.2})",
        samples as f64 / elapsed / 1e6,
        adc_hz / 1e6
    );
    println!("  achieved {:.1}% of the ADC rate", 100.0 * (samples as f64 / elapsed) / adc_hz);
    println!("  transfer errors: {errors}");
    println!(
        "  peak {peak:.4} ({:.1} dBFS), rms {rms:.5} ({:.1} dBFS), mean {mean:+.6}",
        20.0 * peak.max(1e-9).log10(),
        20.0 * rms.max(1e-9).log10()
    );
    if peak >= 0.999 {
        println!("  NOTE: clipping — reduce the VGA gain or add attenuation.");
    }
    if rms > 0.20 {
        println!("  NOTE: rms is very high for HF noise; if the randomizer setting is wrong");
        println!("        the stream looks like full-scale noise. Compare with PROBE_NORAND=1.");
    }

    match dev.stats() {
        Ok(s) => println!(
            "after stop: dma={} gpif={} pib_err={} i2c_err={} faults={} clk0 {}",
            s.dma_count,
            s.gpif_state,
            s.pib_errors,
            s.i2c_errors,
            s.stream_faults,
            if s.clock_running() { "RUNNING" } else { "stopped" }
        ),
        Err(e) => println!("stats failed: {e}"),
    }
    if let Some(mut w) = dump {
        let _ = w.flush();
        println!("raw i16 written to PROBE_DUMP");
    }
    println!("\nDone.");
}

fn env_f64(key: &str, default: f64) -> f64 {
    std::env::var(key).ok().and_then(|v| v.parse().ok()).unwrap_or(default)
}

/// Sweep the two analogue gain stages and report the resulting noise level.
///
/// This is the test that distinguishes "no antenna" from "the front end is not
/// in circuit". If the ADC is seeing the analogue chain at all, raising the VGA
/// must raise the noise floor roughly dB for dB; a level that does not move is a
/// front end that is switched out, muted, or unpowered — and no amount of
/// antenna will fix that.
fn sweep(dev: &mut device::Device, ep: &mut nusb::Endpoint<Bulk, In>, randomize: bool) {
    const XFER: usize = 256 * 1024;

    println!("\n--- gain sweep ---");
    println!("  {:>7} {:>7} {:>5} {:>10} {:>10}", "VGA dB", "ATT dB", "PGA", "rms dBFS", "std cnt");

    for (vga, att, pga) in [
        (12.0, -0.0, true),
        (12.0, -2.0, true),
        (12.0, -4.0, true),
        (12.0, -8.0, true),
        (12.0, -12.0, true),
        (12.0, -16.0, true),
        (12.0, -20.0, true),
        (12.0, -24.0, true),
        (12.0, -28.0, true),
        (12.0, -31.5, true),
    ] {
        if let Err(e) = dev.set_vga_db(vga) {
            println!("    set_vga_db({vga}) FAILED: {e}");
        }
        if let Err(e) = dev.set_attenuator_db(att) {
            println!("    set_attenuator_db({att}) FAILED: {e}");
        }
        if let Err(e) = dev.set_pga(pga) {
            println!("    set_pga({pga}) FAILED: {e}");
        }
        std::thread::sleep(Duration::from_millis(60));

        // Drain whatever is stale, then take a fresh transfer.
        for _ in 0..4 {
            ep.submit(ep.allocate(XFER));
        }
        let mut acc: Vec<f64> = Vec::new();
        for _ in 0..8 {
            let Some(c) = ep.wait_next_complete(Duration::from_millis(300)) else { break };
            if c.status.is_ok() {
                let mut out = Vec::new();
                convert::to_f32(&c.buffer[..], randomize, &mut None, &mut out);
                if out.len() > 4096 {
                    let n = out.len() as f64;
                    let mean = out.iter().map(|v| *v as f64).sum::<f64>() / n;
                    let var = out.iter().map(|v| (*v as f64 - mean).powi(2)).sum::<f64>() / n;
                    acc.push(var.sqrt());
                }
            }
            let mut buf = c.buffer;
            buf.clear();
            ep.submit(buf);
        }
        let stats = if acc.is_empty() {
            None
        } else {
            Some((acc.iter().sum::<f64>() / acc.len() as f64, 0.0))
        };
        match stats {
            // Standard deviation, not raw rms: the ADC's DC offset is large
            // compared with the signal here and would swamp the comparison.
            Some((sd, _)) => println!(
                "  {vga:>7.1} {att:>7.1} {:>5} {:>10.1} {:>10.1}",
                if pga { "on" } else { "off" },
                20.0 * sd.max(1e-12).log10(),
                sd * 32768.0,
            ),
            None => println!(
                "  {vga:>7.1} {att:>7.1} {:>5}   (no data)",
                if pga { "on" } else { "off" }
            ),
        }
    }
}
