//! Run the wire-level FlexRadio simulator, so sdroxide can be pointed at
//! "a radio" with none in the room.
//!
//! Run: `cargo run -p sdroxide-smartsdr --example sim [-- --port 4992 --announce]`
//!
//! Then set Settings → Radio → SmartSDR to the address it prints. With
//! `--announce` it also broadcasts discovery packets, so the Discover button
//! finds it — that puts a phantom radio on the LAN, so it is off by default.

use std::net::{IpAddr, Ipv4Addr};
use std::time::Duration;

use sdroxide_smartsdr::sim::{SimConfig, SimRadio};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()),
        )
        .init();

    let mut cfg = SimConfig { port: 4992, ..SimConfig::default() };
    let mut args = std::env::args().skip(1);
    while let Some(a) = args.next() {
        match a.as_str() {
            "--port" => cfg.port = args.next().and_then(|v| v.parse().ok()).unwrap_or(4992),
            "--announce" => cfg.announce = true,
            // Answering on the LAN means somebody else's SmartSDR can find it,
            // so it takes an explicit flag.
            "--lan" => cfg.bind = IpAddr::V4(Ipv4Addr::UNSPECIFIED),
            "--refuse-gui" => cfg.refuse_gui = true,
            "--freq" => {
                if let Some(hz) = args.next().and_then(|v| v.parse::<f64>().ok()) {
                    cfg.start_freq_hz = hz;
                    cfg.carriers = vec![(hz, 0.20), (hz + 6_000.0, 0.08), (hz + 26_000.0, 0.30)];
                }
            }
            "--help" | "-h" => {
                println!(
                    "usage: sim [--port N] [--announce] [--lan] [--refuse-gui] [--freq HZ]\n\
                     \n\
                     --port N       TCP/UDP port to serve on (default 4992)\n\
                     --announce     broadcast discovery packets on UDP 4992\n\
                     --lan          bind all interfaces instead of loopback\n\
                     --refuse-gui   reject GUI registration, to exercise the error path\n\
                     --freq HZ      centre the synthetic scene here (default 14074000)"
                );
                return;
            }
            other => eprintln!("ignoring unknown argument {other:?}"),
        }
    }

    let announce = cfg.announce;
    let freq = cfg.start_freq_hz;
    let sim = match SimRadio::start(cfg) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("could not start the simulator: {e}");
            if e.kind() == std::io::ErrorKind::AddrInUse {
                eprintln!(
                    "Port 4992 is taken — SmartSDR or another simulator may be running. \
                     Try --port 5992."
                );
            }
            std::process::exit(1);
        }
    };

    println!("simulated FlexRadio listening on {}", sim.address());
    println!("  scene centred at {:.6} MHz", freq / 1e6);
    if announce {
        println!("  broadcasting discovery on UDP 4992");
    } else {
        println!("  not broadcasting — enter the address by hand, or pass --announce");
    }
    println!("\nPoint Settings → Radio → SmartSDR at that address. Ctrl-C to stop.\n");

    // Report what the client is doing, so a session can be watched without
    // attaching a debugger to either side.
    let mut last = (0u64, 0u64);
    loop {
        std::thread::sleep(Duration::from_secs(2));
        use std::sync::atomic::Ordering;
        let iq = sim.stats.iq_packets.load(Ordering::Relaxed);
        let tx = sim.stats.tx_frames.load(Ordering::Relaxed);
        if (iq, tx) != last {
            println!(
                "IQ packets {iq}  TX frames {tx}  peak {:.3}  slice {:.6} MHz  {}",
                sim.stats.tx_peak(),
                sim.stats.slice_freq_hz() / 1e6,
                if sim.stats.keyed.load(Ordering::Relaxed) { "KEYED" } else { "rx" },
            );
            last = (iq, tx);
        }
    }
}
