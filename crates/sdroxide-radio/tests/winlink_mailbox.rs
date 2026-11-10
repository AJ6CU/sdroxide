//! The Winlink mailbox through the engine's command path.
//!
//! Everything the mail window does goes through `Command`/`RadioEvent`, so
//! this drives that seam rather than the crate underneath it: compose, list,
//! fetch, move, delete. It is the only test that says the engine wiring — the
//! command arms, the manager, the mailbox root under the config directory —
//! actually joins up, and it runs with no network and no radio.

use std::time::{Duration, Instant};

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DeviceCaps, MailDraft, MailFolder, RadioEvent};

const RATE: f64 = 2_400_000.0;

/// Silence, at a leisurely rate. The mailbox does not care what the radio is
/// doing, and this keeps the test cheap.
struct Silence {
    center: f64,
}

impl IqSource for Silence {
    fn sample_rate(&self) -> f64 {
        RATE
    }
    fn center_hz(&self) -> f64 {
        self.center
    }
    fn set_center_hz(&mut self, hz: f64) -> Result<()> {
        self.center = hz;
        Ok(())
    }
    fn read(&mut self, buf: &mut [Complex32]) -> Result<usize> {
        std::thread::sleep(Duration::from_millis(10));
        let n = buf.len().min(2048);
        buf[..n].fill(Complex32::new(0.0, 0.0));
        Ok(n)
    }
    fn describe(&self) -> String {
        "silence".into()
    }
}

fn caps() -> DeviceCaps {
    DeviceCaps {
        driver: "mock".into(),
        label: "mock".into(),
        rx_channels: 1,
        sample_rates: vec![RATE],
        freq_ranges_rx: vec![(0.0, 1_000_000_000.0)],
        ..DeviceCaps::default()
    }
}

struct Harness {
    h: sdroxide_radio::EngineHandles,
    thread: Option<std::thread::JoinHandle<()>>,
}

impl Harness {
    fn send(&self, cmd: Command) {
        self.h.cmd_tx.send(cmd).unwrap();
    }

    fn wait_for<T>(&self, what: &str, mut pred: impl FnMut(&RadioEvent) -> Option<T>) -> T {
        let deadline = Instant::now() + Duration::from_secs(5);
        while Instant::now() < deadline {
            while let Ok(ev) = self.h.event_rx.try_recv() {
                if let Some(t) = pred(&ev) {
                    return t;
                }
            }
            std::thread::sleep(Duration::from_millis(10));
        }
        panic!("timed out waiting for {what}");
    }

    fn list(&self, folder: MailFolder) -> sdroxide_types::MailListing {
        self.send(Command::MailList { folder, offset: 0, count: 50 });
        self.wait_for("a mail listing", |ev| match ev {
            RadioEvent::MailListing(l) if l.folder == folder => Some(l.clone()),
            _ => None,
        })
    }

    fn stop(mut self) {
        drop(self.h.cmd_tx);
        if let Some(t) = self.thread.take() {
            let _ = t.join();
        }
    }
}

/// One test, because it redirects the config directory through the environment,
/// which is process-global.
#[test]
fn mail_round_trips_through_the_engine() {
    let root = std::env::temp_dir().join(format!("sdroxide-winlink-test-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).expect("scratch dir");
    // SAFETY: this is the only test in this binary; nothing races the setter.
    unsafe { std::env::set_var("SDROXIDE_CONFIG_DIR", &root) };

    // Do not bind the TCI server port: a real sdroxide may be running.
    let tci_off = sdroxide_types::TciServerConfig { enabled: false, ..Default::default() };
    sdroxide_config::Store::radio(0).save_tci_server_config(&tci_off).unwrap();

    let mut h = start_engine(
        Box::new(Silence { center: 14_200_000.0 }),
        caps(),
        EngineConfig { store: sdroxide_config::Store::radio(0), ..Default::default() },
    );
    let thread = h.thread.take();
    let eng = Harness { h, thread };

    eng.wait_for("the first state", |ev| matches!(ev, RadioEvent::State(_)).then_some(()));

    // The account has to be set before anything can be filed: the engine mints
    // the From address from it.
    let mut net = sdroxide_types::NetworkConfig::default();
    net.winlink.callsign = "OE3JJS".into();
    net.winlink.password = "not-used-offline".into();
    eng.send(Command::SetNetworkConfig(net));
    eng.wait_for("winlink status after configuring", |ev| {
        matches!(ev, RadioEvent::WinlinkStatus(_)).then_some(())
    });

    // Compose → the outbox.
    eng.send(Command::MailCompose(Box::new(MailDraft {
        to: vec!["OE1XYZ".into()],
        cc: vec![],
        subject: "engine round trip".into(),
        body: "hello from the test\r\n".into(),
        attachments: vec![sdroxide_types::MailAttachment {
            name: "reading.bin".into(),
            data: vec![0, 1, 254, 255],
        }],
    })));
    let mid = eng.wait_for("the composed message id", |ev| match ev {
        RadioEvent::MailSaved(mid) => Some(mid.clone()),
        _ => None,
    });
    assert!(!mid.is_empty(), "the engine mints the MID, not the client");

    // It lists.
    let listing = eng.list(MailFolder::Outbox);
    assert_eq!(listing.total, 1);
    assert_eq!(listing.entries[0].mid, mid);
    assert_eq!(listing.entries[0].subject, "engine round trip");
    assert_eq!(listing.entries[0].attachments, 1);

    // It fetches whole, attachment bytes intact — the part that has to survive
    // being written to disk and read back through the wire types.
    eng.send(Command::MailGet { folder: MailFolder::Outbox, mid: mid.clone() });
    let msg = eng.wait_for("the fetched message", |ev| match ev {
        RadioEvent::MailMessage(m) if m.mid == mid => Some(m.clone()),
        _ => None,
    });
    assert_eq!(msg.from, "OE3JJS");
    assert_eq!(msg.to, ["OE1XYZ"]);
    assert_eq!(msg.body, "hello from the test\r\n");
    assert_eq!(msg.attachments[0].name, "reading.bin");
    assert_eq!(msg.attachments[0].data, vec![0, 1, 254, 255]);

    // Move to the archive.
    eng.send(Command::MailMove {
        from: MailFolder::Outbox,
        to: MailFolder::Archive,
        mid: mid.clone(),
    });
    eng.wait_for("the move to be announced", |ev| {
        matches!(ev, RadioEvent::MailDeleted { folder, .. } if *folder == MailFolder::Outbox)
            .then_some(())
    });
    assert_eq!(eng.list(MailFolder::Outbox).total, 0);
    assert_eq!(eng.list(MailFolder::Archive).total, 1);

    // Delete.
    eng.send(Command::MailDelete { folder: MailFolder::Archive, mid: mid.clone() });
    eng.wait_for("the delete to be announced", |ev| {
        matches!(ev, RadioEvent::MailDeleted { folder, .. } if *folder == MailFolder::Archive)
            .then_some(())
    });
    assert_eq!(eng.list(MailFolder::Archive).total, 0);

    // A message id that would escape the mailbox directory is refused rather
    // than resolved. It arrives over a socket nothing authenticates and ends at
    // `remove_file`, so this is the rail that matters most here.
    eng.send(Command::MailDelete { folder: MailFolder::Inbox, mid: "../../../etc/passwd".into() });
    let notice = eng.wait_for("a refusal", |ev| match ev {
        RadioEvent::Notice(Some(n)) => Some(n.clone()),
        _ => None,
    });
    assert!(notice.contains("not a usable message id"), "unexpected notice: {notice}");

    // Connecting with no password is refused before any network I/O.
    let mut net = sdroxide_types::NetworkConfig::default();
    net.winlink.callsign = "OE3JJS".into();
    eng.send(Command::SetNetworkConfig(net));
    eng.wait_for("winlink status after clearing the password", |ev| {
        matches!(ev, RadioEvent::WinlinkStatus(_)).then_some(())
    });
    eng.send(Command::WinlinkConnect);
    let notice = eng.wait_for("a refusal to connect", |ev| match ev {
        RadioEvent::Notice(Some(n)) if n.contains("Winlink") => Some(n.clone()),
        _ => None,
    });
    assert!(notice.contains("callsign and password"), "unexpected notice: {notice}");

    // Put the full account back, so what the restart below reloads is a
    // configured station rather than the half-cleared one used just above.
    let mut net = sdroxide_types::NetworkConfig::default();
    net.winlink.callsign = "OE3JJS".into();
    net.winlink.password = "not-used-offline".into();
    eng.send(Command::SetNetworkConfig(net));
    eng.wait_for("winlink status after restoring the account", |ev| {
        matches!(ev, RadioEvent::WinlinkStatus(_)).then_some(())
    });

    eng.stop();

    // Settings must survive a restart. The engine loads `net.json` at startup,
    // but the mailbox manager is built before that happens, so it has to be
    // handed the account explicitly — otherwise every session refuses with
    // "set a callsign and password first" and it reads as the settings having
    // been lost.
    let mut h = start_engine(
        Box::new(Silence { center: 14_200_000.0 }),
        caps(),
        EngineConfig { store: sdroxide_config::Store::radio(0), ..Default::default() },
    );
    let thread = h.thread.take();
    let restarted = Harness { h, thread };
    restarted.wait_for("the first state", |ev| matches!(ev, RadioEvent::State(_)).then_some(()));

    // The callsign persisted, so composing works without configuring anything.
    restarted.send(Command::MailCompose(Box::new(MailDraft {
        to: vec!["OE1XYZ".into()],
        subject: "after restart".into(),
        body: "still configured\r\n".into(),
        ..Default::default()
    })));
    let mid = restarted.wait_for("a composed message after restart", |ev| match ev {
        RadioEvent::MailSaved(mid) => Some(mid.clone()),
        _ => None,
    });
    restarted.send(Command::MailGet { folder: MailFolder::Outbox, mid: mid.clone() });
    let msg = restarted.wait_for("the message after restart", |ev| match ev {
        RadioEvent::MailMessage(m) if m.mid == mid => Some(m.clone()),
        _ => None,
    });
    assert_eq!(msg.from, "OE3JJS", "the persisted callsign should have been reloaded");

    restarted.stop();
    let _ = std::fs::remove_dir_all(&root);
}
