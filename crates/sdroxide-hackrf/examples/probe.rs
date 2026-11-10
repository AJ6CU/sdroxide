//! Open a HackRF, receive for a few seconds, and print everything a bug report
//! needs. **Receive only — this never keys the transmitter.**
//!
//! ```text
//! cargo run -p sdroxide-hackrf --example probe -- [MHz] [Msps]
//! ```
//!
//! The line worth the trouble is `decoded as (re,im)`. Whether I really comes
//! before Q on the wire cannot be checked from this side of the USB cable: hex
//! looks identical either way, and so does a symmetric test signal on a
//! spectrum. Point the radio at a carrier deliberately *above* the tuned
//! frequency and the peak has to land on the positive side of DC. If it lands
//! below, the two are swapped.

use std::time::{Duration, Instant};

use sdroxide_hackrf::HackRfHandle;
use sdroxide_hackrf::convert::{sample_lut, to_iq};
use sdroxide_types::HackRfConfig;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sdroxide_hackrf=debug".into()),
        )
        .init();

    let mut args = std::env::args().skip(1);
    let mhz: f64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(93.2);
    let msps: f64 = args.next().and_then(|s| s.parse().ok()).unwrap_or(2.0);

    println!("=== devices on the bus ===");
    let devices = sdroxide_hackrf::list();
    if devices.is_empty() {
        println!("  none — check the udev rule and the cable");
        return;
    }
    for (i, d) in devices.iter().enumerate() {
        println!("  {i}: {}  (usb pid {:04x})", d.label(), d.pid);
    }

    let cfg = HackRfConfig {
        sample_rate_hz: msps * 1e6,
        // Whatever else this example does, it does not transmit.
        tx_enabled: false,
        txvga_db: 0.0,
        tx_amp: false,
        ..Default::default()
    };

    println!("\n=== opening at {mhz} MHz, {msps} Msps ===");
    let mut h = match HackRfHandle::open(&cfg, mhz * 1e6) {
        Ok(h) => h,
        Err(e) => {
            println!("open failed: {e}");
            if let Some(t) = sdroxide_hackrf::diagnostics() {
                println!("\n{t}");
            }
            return;
        }
    };

    println!("  board       : {} ({:?})", h.label, h.kind);
    println!("  firmware    : {:?}", h.firmware);
    println!("  serial      : {}", h.serial.as_deref().unwrap_or("none"));
    println!("  rate        : {:.3} Msps", h.sample_rate_hz / 1e6);
    println!("  filter      : {:.2} MHz", h.filter_bw_hz as f64 / 1e6);
    println!("  tuning      : {:.1}–{:.1} MHz", h.freq_range.0 / 1e6, h.freq_range.1 / 1e6);
    println!("  bias tee    : {}", if h.has_bias_tee { "present" } else { "not on this board" });
    if let Some(w) = &h.link_warning {
        println!("  LINK        : {w}");
    }
    if let Some(from) = h.snapped_from {
        println!("  rate snapped: asked {:.3} Msps", from / 1e6);
    }

    println!("\n=== receiving for 3 s ===");
    let lut = sample_lut();
    let mut carry = None;
    let mut bytes = vec![0u8; 1 << 16];
    let mut iq = Vec::new();
    let mut total = 0usize;
    let mut peak: f32 = 0.0;
    let mut sum_sq = 0.0f64;
    let mut first: Option<Vec<sdroxide_dsp::Complex32>> = None;

    let until = Instant::now() + Duration::from_secs(3);
    while Instant::now() < until {
        let n = h.rx_read(&mut bytes);
        if n == 0 {
            std::thread::sleep(Duration::from_millis(2));
            continue;
        }
        iq.clear();
        to_iq(&bytes[..n], &lut, &mut carry, &mut iq);
        if first.is_none() && !iq.is_empty() {
            first = Some(iq.iter().take(8).copied().collect());
        }
        for s in &iq {
            peak = peak.max(s.re.abs()).max(s.im.abs());
            sum_sq += (s.re * s.re + s.im * s.im) as f64;
        }
        total += iq.len();
    }

    let rms = (sum_sq / total.max(1) as f64).sqrt();
    println!("  samples     : {total} ({:.3} Msps measured)", total as f64 / 3.0 / 1e6);
    println!("  peak        : {peak:.4} of full scale");
    println!("  rms         : {rms:.4}");
    if peak >= 0.99 {
        println!("  NOTE: the front end is saturating — turn the LNA or the amp down.");
    }
    if total == 0 {
        println!("  NOTE: nothing arrived at all. The trace below is the whole story.");
    } else if peak < 0.01 {
        println!("  NOTE: nearly silent — check the antenna, the gains, and the frequency.");
    }
    if let Some(f) = &first {
        let pairs: Vec<String> = f.iter().map(|s| format!("({:+.4},{:+.4})", s.re, s.im)).collect();
        println!("  first (re,im): {}", pairs.join(" "));
        println!(
            "  ^ point the radio at a carrier ABOVE the tuned frequency: the peak must\n\
             \x20   land on the positive side of DC, or I and Q are swapped."
        );
    }

    h.release();
    println!("\n=== session trace ===");
    match sdroxide_hackrf::diagnostics() {
        Some(t) => println!("{t}"),
        None => println!("(none)"),
    }
}
