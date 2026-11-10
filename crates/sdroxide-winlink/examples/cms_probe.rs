//! Connect to the live Winlink CMS and run one forwarding session.
//!
//! A development harness, in the spirit of the other crates' `probe` examples:
//! it exercises the whole client stack — telnet login, B2F handshake, secure
//! login, proposals, framing, LZHUF — against the real thing, and prints the
//! protocol transcript so a mismatch is visible rather than merely fatal.
//!
//! Credentials come from the environment so they stay out of the repository
//! and out of shell history:
//!
//! ```text
//! WINLINK_CALLSIGN=OE3JJS WINLINK_PASSWORD=... \
//!     cargo run -p sdroxide-winlink --example cms_probe
//! ```
//!
//! With `--send` it also posts a short message. With `--mailbox <dir>` it runs
//! the session through [`WinlinkManager`] instead of driving the session layer
//! directly, so the whole stack is exercised — receive, file to disk, list —
//! which matters because the CMS delivers a message once and there is no
//! second chance to test the filing half.

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use sdroxide_types::MailFolder;
use sdroxide_winlink::WinlinkManager;
use sdroxide_winlink::message::{Message, format_date, generate_mid};
use sdroxide_winlink::session::{self, SessionConfig};
use sdroxide_winlink::transport::{CMS_ADDRESS, TelnetTransport};

fn main() {
    let callsign =
        std::env::var("WINLINK_CALLSIGN").expect("set WINLINK_CALLSIGN").trim().to_uppercase();
    let password = std::env::var("WINLINK_PASSWORD").expect("set WINLINK_PASSWORD");
    let locator = std::env::var("WINLINK_LOCATOR").unwrap_or_else(|_| "JN88".into());
    let send = std::env::args().any(|a| a == "--send");
    // Production refuses client names it does not know, and says so, naming
    // cms-z.winlink.org as the host for everyone else. Override to develop
    // against that until "sdroxide" is a registered client type.
    let address = std::env::var("WINLINK_CMS").unwrap_or_else(|_| CMS_ADDRESS.into());
    // Some implementations announce themselves as a known client to get in.
    // Kept as an explicit, off-by-default knob rather than a silent default:
    // claiming to be somebody else's client is not ours to do quietly.
    let app_name = std::env::var("WINLINK_APP_NAME").unwrap_or_else(|_| "sdroxide".into());

    // `--mailbox <dir>`: drive the manager, which is what the engine uses.
    let args: Vec<String> = std::env::args().collect();
    if let Some(i) = args.iter().position(|a| a == "--mailbox") {
        let dir = args.get(i + 1).expect("--mailbox needs a directory").clone();
        run_via_manager(&dir, &callsign, &password, &locator, &address, &app_name, send);
        return;
    }

    println!("connecting to {address} as {callsign} (client name {app_name})…");
    let mut transport = match TelnetTransport::dial(&address, &callsign, Duration::from_secs(30)) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("dial failed: {e}");
            std::process::exit(1);
        }
    };
    println!("logged in to the gateway, starting B2F…\n");

    let cfg = SessionConfig {
        callsign: callsign.clone(),
        password,
        locator,
        app_name,
        app_version: env!("CARGO_PKG_VERSION").into(),
        ..Default::default()
    };

    let outbound = if send { vec![test_message(&callsign)] } else { Vec::new() };
    if let Some(msg) = outbound.first() {
        println!("offering a test message to {} (mid {})\n", msg.to[0], msg.mid);
    }

    let mut outcome = session::SessionOutcome::default();
    outcome.log.extend(transport.login_log().iter().cloned());
    let result = session::run_into(&mut transport, &cfg, &outbound, &mut outcome);

    // Print the transcript first and unconditionally: on a failure it is the
    // whole diagnosis.
    println!("--- transcript ---");
    for line in &outcome.log {
        println!("{line}");
    }

    if let Err(e) = result {
        eprintln!("\nsession failed: {e}");
        std::process::exit(2);
    }

    println!("\n--- result ---");
    println!("received {} message(s), sent {}", outcome.received.len(), outcome.sent.len());
    for msg in &outcome.received {
        println!(
            "\n  from {}  to {:?}\n  date {}  mid {}\n  subject: {}\n  body ({} bytes):\n{}",
            msg.from,
            msg.to,
            msg.date,
            msg.mid,
            msg.subject,
            msg.body.len(),
            indent(&msg.body)
        );
        for att in &msg.attachments {
            println!("  attachment: {} ({} bytes)", att.name, att.data.len());
        }
    }
    if !outcome.rejected.is_empty() {
        println!("rejected by the CMS: {:?}", outcome.rejected);
    }
}

fn test_message(callsign: &str) -> Message {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    Message {
        mid: generate_mid(callsign, now.as_nanos()),
        date: format_date(now.as_secs() as i64),
        msg_type: "Private".into(),
        from: callsign.to_string(),
        to: vec![callsign.to_string()],
        cc: vec![],
        subject: "sdroxide test".into(),
        mbo: callsign.to_string(),
        body: "Sent by sdroxide's Winlink client over the CMS telnet gateway.\r\n".into(),
        attachments: vec![],
    }
}

fn indent(text: &str) -> String {
    text.lines().map(|l| format!("    {l}")).collect::<Vec<_>>().join("\n")
}

/// Run one session through [`WinlinkManager`] and report what landed in the
/// mailbox, so the filing half is covered by the same single delivery.
#[allow(clippy::too_many_arguments)]
fn run_via_manager(
    dir: &str,
    callsign: &str,
    password: &str,
    locator: &str,
    address: &str,
    app_name: &str,
    send: bool,
) {
    let cfg = sdroxide_types::WinlinkConfig {
        callsign: callsign.to_string(),
        password: password.to_string(),
        locator: locator.to_string(),
        cms_address: address.to_string(),
        app_name: app_name.to_string(),
        ..Default::default()
    };
    let mut mgr = WinlinkManager::new(dir, cfg).expect("opening the mailbox");
    println!("mailbox at {dir}");

    if send {
        let draft = sdroxide_types::MailDraft {
            to: vec![callsign.to_string()],
            subject: "sdroxide test".into(),
            body: "Sent by sdroxide's Winlink client.\r\n".into(),
            ..Default::default()
        };
        println!("filed {} in the outbox", mgr.compose(&draft).expect("composing"));
    }

    println!("connecting to {address} as {callsign} (client name {app_name})…");
    mgr.connect(sdroxide_winlink::WinlinkRoute::Telnet { address: address.to_string() })
        .expect("starting the session");
    while !mgr.poll() {
        std::thread::sleep(Duration::from_millis(200));
    }

    let status = mgr.status().clone();
    println!("\n--- transcript ---");
    for line in &status.log {
        println!("{line}");
    }

    println!("\n--- result ---");
    if let Some(err) = &status.last_error {
        println!("session failed: {err}");
    }
    println!("received {}, sent {}", status.last_received, status.last_sent);
    for (i, folder) in MailFolder::ALL.into_iter().enumerate() {
        println!("  {:<8} {}", folder.dir_name(), status.counts[i]);
    }

    // The point of this mode: what actually reached the disk.
    for folder in [MailFolder::Inbox, MailFolder::Sent] {
        let listing = mgr.list(folder, 0, 20);
        for entry in &listing.entries {
            println!("\n[{}] {}  {}", folder.dir_name(), entry.date, entry.mid);
            println!("  from {}  to {}", entry.from, entry.to);
            println!("  subject: {}", entry.subject);
            if let Some(msg) = mgr.get(folder, &entry.mid) {
                println!("  body ({} bytes):", msg.body.len());
                for line in msg.body.lines() {
                    println!("    {line}");
                }
                for att in &msg.attachments {
                    println!("  attachment: {} ({} bytes)", att.name, att.data.len());
                }
            }
        }
    }
}
