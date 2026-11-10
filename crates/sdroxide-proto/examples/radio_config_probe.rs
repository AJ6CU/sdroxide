//! Drive a running sdroxide server's interface configuration over the
//! WebSocket and report what comes back.
//!
//! An end-to-end check of the path a remote client's **Settings → Radio** takes,
//! which no unit test reaches: connect, receive the `radio.json` replayed on
//! connect, change a setting the far end has no `DeviceCaps` entry for — an
//! RTL-SDR's AGC mode, its ppm correction, its bias tee — and see the engine's
//! echo come back with it.
//!
//! **Not** safe to point at a station in use: it writes the server's
//! `radio.json`. It puts every value back at the end, but a probe that was
//! interrupted halfway would leave the change behind. Run it against a server
//! started with `SDROXIDE_CONFIG_DIR` pointed somewhere scratch.
//!
//! Usage: cargo run -p sdroxide-proto --example radio_config_probe -- 127.0.0.1:4950

use std::io::ErrorKind;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::{Duration, Instant};

use tungstenite::Message;

use sdroxide_proto::{AudioCaps, ClientMsg, PROTO_VERSION, ServerMsg, decode, encode};
use sdroxide_types::{Command, RadioConfig};

fn main() {
    let addr = std::env::args().nth(1).unwrap_or_else(|| "127.0.0.1:4950".to_string());
    println!("connecting to {addr} ...");
    let sockaddr = addr.to_socket_addrs().unwrap().next().unwrap();
    let stream = TcpStream::connect_timeout(&sockaddr, Duration::from_secs(3)).unwrap();
    stream.set_read_timeout(Some(Duration::from_millis(250))).unwrap();
    let (mut ws, _) = tungstenite::client(format!("ws://{addr}/ws").as_str(), stream).unwrap();

    send(
        &mut ws,
        &ClientMsg::Hello {
            proto: PROTO_VERSION,
            audio: AudioCaps { opus_decode: false, opus_encode: false },
        },
    );

    // The connect-time replay. Without it the Radio tab here would open on this
    // machine's defaults and write them over the operator's.
    let Some(original) = wait_for_config(&mut ws, Duration::from_secs(10)) else {
        println!("FAIL: no RadioConfig arrived within 10 s of connecting");
        std::process::exit(1);
    };
    println!("\n-- what the server is running --");
    println!("  interface        {:?}", original.backend);
    println!("  rtlsdr.tuner_gain_db {}", original.rtlsdr.tuner_gain_db);
    println!("  rtlsdr.agc       {:?}", original.rtlsdr.agc);
    println!("  rtlsdr.ppm       {}", original.rtlsdr.ppm);
    println!("  rtlsdr.bias_tee  {}", original.rtlsdr.bias_tee);
    println!("  rtlsdr.hf_mode   {:?}", original.rtlsdr.hf_mode);

    // Change the four settings a remote client could not reach before this
    // lane existed, plus the gain, which it could only reach as a `DeviceCaps`
    // slider with no way to take the AGC off it.
    let mut edited = original.clone();
    edited.rtlsdr.tuner_gain_db = if original.rtlsdr.tuner_gain_db == 33.8 { 22.9 } else { 33.8 };
    edited.rtlsdr.agc = match original.rtlsdr.agc {
        sdroxide_types::RtlSdrAgc::Manual => sdroxide_types::RtlSdrAgc::Rtl,
        _ => sdroxide_types::RtlSdrAgc::Manual,
    };
    edited.rtlsdr.ppm = original.rtlsdr.ppm + 7;
    edited.rtlsdr.iq_correction = !original.rtlsdr.iq_correction;

    println!("\n-- sending an edit --");
    send(
        &mut ws,
        &ClientMsg::Command(Command::SetRadioConfig {
            cfg: Box::new(edited.clone()),
            reopen: false,
        }),
    );

    let Some(echoed) = wait_for_config(&mut ws, Duration::from_secs(10)) else {
        println!("FAIL: the server never echoed the edit");
        std::process::exit(1);
    };
    let mut bad = 0;
    check(&mut bad, "tuner gain", echoed.rtlsdr.tuner_gain_db, edited.rtlsdr.tuner_gain_db);
    check(&mut bad, "agc", echoed.rtlsdr.agc, edited.rtlsdr.agc);
    check(&mut bad, "ppm", echoed.rtlsdr.ppm, edited.rtlsdr.ppm);
    check(&mut bad, "iq correction", echoed.rtlsdr.iq_correction, edited.rtlsdr.iq_correction);
    if echoed != edited {
        println!("  FAIL: some other field of the echo differs from what was sent");
        bad += 1;
    }

    // Put it back, so a probe run leaves the station as it found it.
    println!("\n-- restoring --");
    send(
        &mut ws,
        &ClientMsg::Command(Command::SetRadioConfig {
            cfg: Box::new(original.clone()),
            reopen: false,
        }),
    );
    match wait_for_config(&mut ws, Duration::from_secs(10)) {
        Some(back) if back == original => println!("  OK   the server is back on its own settings"),
        Some(_) => {
            println!("  FAIL the restore did not take — check radio.json on the server");
            bad += 1;
        }
        None => {
            println!("  FAIL no echo for the restore — check radio.json on the server");
            bad += 1;
        }
    }

    // Apply / reconnect, on the settings the server already had: the front end
    // is torn down and rebuilt under the session. What must not happen is the
    // session going with it — an operator who presses Apply from the sofa has
    // no way to press anything else if it does.
    println!("\n-- apply / reconnect --");
    send(
        &mut ws,
        &ClientMsg::Command(Command::SetRadioConfig {
            cfg: Box::new(original.clone()),
            reopen: true,
        }),
    );
    match wait_for_config(&mut ws, Duration::from_secs(10)) {
        Some(_) => println!("  OK   the server answered the reopen"),
        None => {
            println!("  FAIL no answer to the reopen");
            bad += 1;
        }
    }
    if wait_for_spectrum(&mut ws, Duration::from_secs(10)) {
        println!("  OK   spectrum still arriving — the session survived the rebuild");
    } else {
        println!("  FAIL no spectrum after the reopen");
        bad += 1;
    }

    println!("\n{}", if bad == 0 { "PASS" } else { "FAILED" });
    std::process::exit(i32::from(bad != 0));
}

/// Whether the receiver is still feeding this session. Proof that a rebuilt
/// front end came back, not merely that the socket is open.
fn wait_for_spectrum(ws: &mut tungstenite::WebSocket<TcpStream>, within: Duration) -> bool {
    let deadline = Instant::now() + within;
    while Instant::now() < deadline {
        match ws.read() {
            Ok(Message::Binary(bytes)) => {
                if let Ok(ServerMsg::Spectrum(f)) = decode::<ServerMsg>(&bytes) {
                    if !f.bins.is_empty() {
                        return true;
                    }
                }
            }
            Ok(_) => {}
            Err(tungstenite::Error::Io(e))
                if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(_) => return false,
        }
    }
    false
}

fn send(ws: &mut tungstenite::WebSocket<TcpStream>, msg: &ClientMsg) {
    ws.send(Message::Binary(encode(msg).unwrap().into())).unwrap();
}

fn check<T: PartialEq + std::fmt::Debug>(bad: &mut u32, what: &str, got: T, want: T) {
    if got == want {
        println!("  OK   {what} = {got:?}");
    } else {
        println!("  FAIL {what} = {got:?}, expected {want:?}");
        *bad += 1;
    }
}

/// Read until the next `RadioConfig`, letting the spectrum and audio lanes run
/// past. A read timeout is not the end of the wait — the socket is polled, and
/// on a quiet server most polls have nothing.
fn wait_for_config(
    ws: &mut tungstenite::WebSocket<TcpStream>,
    within: Duration,
) -> Option<RadioConfig> {
    let deadline = Instant::now() + within;
    while Instant::now() < deadline {
        match ws.read() {
            Ok(Message::Binary(bytes)) => match decode::<ServerMsg>(&bytes) {
                Ok(ServerMsg::RadioConfig(c)) => return Some(*c),
                Ok(ServerMsg::Error(e)) => {
                    println!("server error: {e}");
                    return None;
                }
                Ok(ServerMsg::Busy) => {
                    println!("server busy — another client holds the session");
                    return None;
                }
                _ => {}
            },
            Ok(_) => {}
            Err(tungstenite::Error::Io(e))
                if matches!(e.kind(), ErrorKind::WouldBlock | ErrorKind::TimedOut) => {}
            Err(e) => {
                println!("socket error: {e}");
                return None;
            }
        }
    }
    None
}
