//! Does this receiver's front end actually respond to the switches?
//!
//! ```text
//! cargo run -p sdroxide-rx888 --example gpio_check
//! ```
//!
//! The bias tee, dither, ADC range select and the randomizer all ride the same
//! `GPIOFX3` control word, and none of them can be read back on the firmware
//! this driver ships: a receiver whose front-end GPIO was never initialised
//! accepts every write and does nothing, which is indistinguishable from a
//! working one until an antenna fails to power up.
//!
//! So this asks the receiver what it thinks it is, and then proves — or
//! disproves — that a GPIO write reaches the hardware, using the *randomizer*,
//! which is the one bit in the word whose effect shows up in the samples: with
//! the ADC scrambling its output and the host un-scrambling it the level is
//! sane, and with the two disagreeing it is not. If the randomizer moves, the
//! bias tee is reaching the hardware too; if it does not, nothing in the word
//! is, and no amount of clicking the checkbox will help.
//!
//! Receive only: nothing here transmits, and the bias tee is left off.

use std::time::{Duration, Instant};

use nusb::transfer::{Bulk, In};

use sdroxide_rx888::device::{Device, Settings};
use sdroxide_rx888::protocol::Identity;
use sdroxide_rx888::{convert, protocol};

/// Bulk transfer size and depth. Small: this is a diagnostic, not a stream.
const XFER: usize = 256 * 1024;
const IN_FLIGHT: usize = 8;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sdroxide_rx888=info".into()),
        )
        .init();

    let settings = Settings::default();
    let mut dev = match Device::open(&settings, None) {
        Ok(d) => d,
        Err(e) => {
            eprintln!("open failed: {e}");
            std::process::exit(1);
        }
    };

    println!("Receiver: {}", dev.label());
    match dev.identity() {
        Some(Identity { hardware, version }) => {
            println!("  firmware   {version}");
            println!("  hardware   0x{hardware:02x}");
            if !dev.identity().is_some_and(|i| i.front_end_ready()) {
                println!(
                    "  ** the firmware did not recognise this receiver: it never configured the\n\
                     ** front-end control pins, so the bias tee, dither, ADC range and both gain\n\
                     ** stages do nothing. Reception still works. This is the fault to report."
                );
            }
        }
        None => println!("  (the receiver did not answer TESTFX3)"),
    }
    if let Some(w) = dev.warning() {
        println!("  note: {w}");
    }
    println!("  gpio word  0x{:08x}", dev.gpio_word());

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

    println!("\nGPIO writes on a streaming receiver (randomizer as the witness):");
    let mut verdicts = Vec::new();
    for on in [true, false, true] {
        if let Err(e) = dev.set_randomize(on) {
            println!("  set_randomize({on}) FAILED: {e}");
        }
        std::thread::sleep(Duration::from_millis(200));
        let (derand, raw) = level(&mut ep);
        // Whichever reading is the *lower* says what the ADC is really doing:
        // undoing a scrambling that is not there splits the DC offset in two
        // and inflates the spread.
        let scrambling = derand < raw;
        verdicts.push(scrambling == on);
        println!(
            "  randomizer {:3}  gpio 0x{:08x}  un-scrambled {:.0} counts, raw {:.0} counts  => \
             ADC is {}",
            if on { "on" } else { "off" },
            dev.gpio_word(),
            derand * 32768.0,
            raw * 32768.0,
            if scrambling { "scrambling" } else { "not scrambling" },
        );
    }
    let reaches = verdicts.iter().all(|ok| *ok);
    println!(
        "\n=> GPIO writes {} this receiver's front end.",
        if reaches { "REACH" } else { "DO NOT REACH" }
    );
    if reaches {
        println!("   The bias tee rides the same word, so enabling it does put DC on the HF port.");
        println!("   If an antenna still is not powered, the fault is past the SMA connector.");
    }

    // ...and the bias tee itself, which nothing here can measure: DC on the
    // coax is invisible to the ADC by design.
    println!("\nBias tee (write only — the DC itself cannot be seen from here):");
    for on in [true, false] {
        match dev.set_bias_tee(on) {
            Ok(()) => println!(
                "  set_bias_tee({on:5}) -> gpio 0x{:08x}  (BIAS_HF {})",
                dev.gpio_word(),
                if dev.gpio_word() & protocol::gpio::BIAS_HF != 0 { "set" } else { "clear" }
            ),
            Err(e) => println!("  set_bias_tee({on:5}) FAILED: {e}"),
        }
        std::thread::sleep(Duration::from_millis(200));
    }

    ep.cancel_all();
    let until = Instant::now() + Duration::from_millis(500);
    while ep.pending() > 0 && Instant::now() < until {
        if ep.wait_next_complete(Duration::from_millis(50)).is_none() {
            break;
        }
    }
    let _ = dev.stop();
    println!("\nDone.");
}

/// Standard deviation of the live stream read both ways — un-scrambled and raw
/// — in fractions of full scale.
fn level(ep: &mut nusb::Endpoint<Bulk, In>) -> (f64, f64) {
    // Everything already queued was captured before the caller's GPIO write;
    // measuring those would report the previous state, one step late.
    for _ in 0..(IN_FLIGHT * 3) {
        while ep.pending() < IN_FLIGHT {
            ep.submit(ep.allocate(XFER));
        }
        if let Some(c) = ep.wait_next_complete(Duration::from_millis(300)) {
            let mut buf = c.buffer;
            buf.clear();
            ep.submit(buf);
        }
    }

    let mut acc = [Vec::<f64>::new(), Vec::<f64>::new()];
    for _ in 0..8 {
        while ep.pending() < IN_FLIGHT {
            ep.submit(ep.allocate(XFER));
        }
        let Some(c) = ep.wait_next_complete(Duration::from_millis(300)) else { break };
        if c.status.is_ok() {
            for (slot, derandomize) in acc.iter_mut().zip([true, false]) {
                let mut out = Vec::new();
                convert::to_f32(&c.buffer[..], derandomize, &mut None, &mut out);
                if out.len() > 4096 {
                    // Standard deviation, not rms: the ADC's DC offset is large
                    // compared with a quiet band and would swamp the comparison.
                    let n = out.len() as f64;
                    let mean = out.iter().map(|v| *v as f64).sum::<f64>() / n;
                    let var = out.iter().map(|v| (*v as f64 - mean).powi(2)).sum::<f64>() / n;
                    slot.push(var.sqrt());
                }
            }
        }
        let mut buf = c.buffer;
        buf.clear();
        ep.submit(buf);
    }
    let mean =
        |v: &Vec<f64>| if v.is_empty() { 0.0 } else { v.iter().sum::<f64>() / v.len() as f64 };
    (mean(&acc[0]), mean(&acc[1]))
}
