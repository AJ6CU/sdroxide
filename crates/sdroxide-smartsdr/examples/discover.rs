//! Listen for FlexRadio discovery broadcasts and print what announces itself.
//!
//! Radios broadcast on UDP 4992 unprompted, so this only listens — there is
//! nothing to send and nothing on the network is disturbed.
//!
//! Run: `cargo run -p sdroxide-smartsdr --example discover [seconds]`

use std::time::Duration;

fn main() {
    let secs: u64 = std::env::args().nth(1).and_then(|s| s.parse().ok()).unwrap_or(4);
    let window = Duration::from_secs(secs);

    println!("listening for FlexRadio broadcasts on UDP 4992 for {secs} s ...");
    let radios = match sdroxide_smartsdr::discover(window) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("discovery failed: {e}");
            eprintln!(
                "\nUDP 4992 could not be opened. A firewall may be blocking it, \
                 or another program may hold it without allowing sharing."
            );
            std::process::exit(1);
        }
    };

    if radios.is_empty() {
        println!("no radios found.");
        println!(
            "\nA FlexRadio announces itself about once a second, so a quiet scan usually \
             means the radio is off, on another subnet, or behind a firewall that drops \
             broadcasts. Radios reached over a router or VPN never broadcast to you at \
             all — enter the address by hand in Settings → Radio."
        );
        return;
    }

    for r in &radios {
        println!("\n{}", r.label());
        println!("  address     : {}:{}", r.ip, r.port);
        println!("  model       : {}", r.model);
        println!("  serial      : {}", r.serial);
        println!("  firmware    : {}", r.version);
        println!("  status      : {}", r.status);
        println!("  multiFLEX   : {}", if r.multiflex { "enabled" } else { "disabled" });
        if !r.gui_clients.is_empty() {
            println!("  GUI clients : {}", r.gui_clients.join(", "));
        }
        println!("  joinable    : {}", if r.joinable() { "yes" } else { "no" });
        // Firmware adds keys faster than clients learn them, and an unmodelled
        // one is often exactly what a bug report needs.
        println!("  raw broadcast:");
        for (k, v) in &r.raw {
            println!("      {k} = {v}");
        }
    }
}
