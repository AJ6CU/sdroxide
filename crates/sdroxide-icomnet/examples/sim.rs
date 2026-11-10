//! Run the wire-level Icom simulator, so sdroxide can be pointed at "a radio"
//! with none in the room.
//!
//! Run: `cargo run -p sdroxide-icomnet --example sim [-- --port 50001]`
//!
//! Then set Settings → Radio to **Icom LAN (network)**, address `127.0.0.1`,
//! and the user name and password it prints. There is no discovery to fake — an
//! Icom never announces itself — so pointing a client at it is all there is.

use std::net::Ipv4Addr;

use sdroxide_icomnet::sim::{Sim, SimOptions};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let mut opts = SimOptions { port: 50_001, tone_hz: Some(1_000.0), ..SimOptions::default() };
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--port" => opts.port = args.next().and_then(|v| v.parse().ok()).unwrap_or(50_001),
            // Answering on the LAN means somebody else's client can find it, so
            // it takes an explicit flag.
            "--lan" => opts.bind = Ipv4Addr::UNSPECIFIED,
            "--civ" => {
                if let Some(v) = args.next() {
                    opts.civ_address =
                        u8::from_str_radix(v.trim_start_matches("0x"), 16).unwrap_or(0xB6);
                }
            }
            "--freq" => {
                if let Some(hz) = args.next().and_then(|v| v.parse::<f64>().ok()) {
                    opts.freq_hz = hz;
                }
            }
            "--tone" => opts.tone_hz = args.next().and_then(|v| v.parse::<f64>().ok()),
            "--drop" => opts.drop_every = args.next().and_then(|v| v.parse().ok()).unwrap_or(0),
            "--no-scope" => opts.scope = false,
            "--rx-only" => opts.can_transmit = false,
            "--help" | "-h" => {
                println!(
                    "usage: sim [--port N] [--lan] [--civ HEX] [--freq HZ] [--tone HZ]\n\
                     \x20          [--drop N] [--no-scope] [--rx-only]\n\
                     \n\
                     --port N     control port (default 50001)\n\
                     --lan        bind all interfaces instead of loopback\n\
                     --civ HEX    CI-V address (default b6, an IC-7300MK2)\n\
                     --freq HZ    dial the radio reports (default 14074000)\n\
                     --tone HZ    audio tone to emit\n\
                     --drop N     drop every Nth packet, to exercise retransmit\n\
                     --no-scope   do not stream spectrum sweeps\n\
                     --rx-only    offer no transmit stream, like an IC-R8600\n"
                );
                return;
            }
            other => eprintln!("ignoring unknown argument {other}"),
        }
    }

    let user = opts.username.clone();
    let pass = opts.password.clone();
    let civ = opts.civ_address;
    let name = opts.radio_name.clone();
    let sim = match Sim::start(opts) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("cannot start the simulator: {e}");
            std::process::exit(1);
        }
    };
    println!("Icom simulator: {name}, CI-V {civ:02X}h, control port {}", sim.port());
    println!("  user \"{user}\", password \"{pass}\"");
    println!("Ctrl-C to stop.");
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
