//! Talk to a LimeRFE over its own USB port and report what happens, one
//! transaction at a time.
//!
//! This exists because the serial protocol cannot be verified without the
//! board, and "it does not work" is a hard thing to act on. Each step prints
//! the bytes it sent and what came back, so a report from it says exactly which
//! exchange failed.
//!
//! ```text
//! cargo run -p sdroxide-limerfe --example rfe -- /dev/ttyUSB0
//! ```
//!
//! Nothing here transmits or switches an amplifier: it says hello, reads the
//! board's identity and its current state, and stops. Add `--configure` to also
//! set a receive-only wideband configuration, which is the gentlest thing the
//! board can be asked to do.

use std::time::Duration;

use sdroxide_limerfe::{RfeTransport, SerialTransport};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "sdroxide_limerfe=debug".into()),
        )
        .init();

    let args: Vec<String> = std::env::args().skip(1).collect();
    let configure = args.iter().any(|a| a == "--configure");
    let path = args.iter().find(|a| !a.starts_with("--")).cloned().unwrap_or_else(|| {
        eprintln!("usage: rfe <serial port> [--configure]");
        eprintln!("\nSerial ports on this machine:");
        for p in serialport::available_ports().unwrap_or_default() {
            eprintln!("  {}", p.port_name);
        }
        std::process::exit(2);
    });

    println!("Opening {path} at 9600 8N1 and saying hello...");
    println!("  (hello is a single 0x00 byte, answered with the same byte — not a 16-byte frame)");
    let mut rfe = match SerialTransport::open(&path) {
        Ok(t) => {
            println!("  OK — {}", t.describe());
            t
        }
        Err(e) => {
            println!("  FAILED: {e}");
            println!(
                "\nThings worth checking, in order:\n  \
                 - that this is the LimeRFE's own micro-USB port, not the LimeSDR's\n  \
                 - that LimeSuiteGUI, SDRangel or another copy of sdroxide does not have it open\n  \
                 - that you can read the port at all: `sudo chmod a+rw {path}` as a one-off test,\n    \
                   or install the udev rule (see the README) for a permanent fix"
            );
            std::process::exit(1);
        }
    };

    println!("\nBoard identity (GET_INFO, 16 bytes each way):");
    match rfe.info() {
        Ok(i) => println!("  firmware {}  hardware {}", i.firmware, i.hardware),
        Err(e) => println!("  FAILED: {e}"),
    }

    println!("\nRound trip this transport reports: {:?}", rfe.round_trip());

    if configure {
        use sdroxide_limerfe::RfeState;
        use sdroxide_types::{RfeChannel, RfeMode, RfePort};
        let state = RfeState {
            channel_rx: RfeChannel::Wb1000,
            channel_tx: RfeChannel::Wb1000,
            port_rx: RfePort::J3,
            port_tx: RfePort::J4,
            // Receive only. Nothing here should key an amplifier.
            mode: RfeMode::Rx,
            notch: false,
            atten_steps: 0,
            swr_enable: false,
            swr_source_cell: false,
        };
        println!("\nConfiguring: wideband, receive only, J3/J4 (CONFIG, 16 bytes each way)");
        match rfe.configure(state) {
            Ok(()) => println!("  OK — the relays should have clicked"),
            Err(e) => println!("  FAILED: {e}"),
        }

        println!("\nMode to receive (MODE, *2* bytes each way — not 16)");
        match rfe.set_mode(RfeMode::Rx) {
            Ok(()) => println!("  OK"),
            Err(e) => println!("  FAILED: {e}"),
        }

        std::thread::sleep(Duration::from_millis(200));
        println!("\nReading the state back (GET_CONFIG):");
        match rfe.info() {
            Ok(i) => println!("  still answering — firmware {}", i.firmware),
            Err(e) => println!("  FAILED: {e}"),
        }
    } else {
        println!("\n(Pass --configure to also set a receive-only wideband configuration.)");
    }
}
