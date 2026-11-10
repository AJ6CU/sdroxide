//! The picture stores, driven through the real engine.
//!
//! What matters here is the command surface, because it is reachable from a
//! WebSocket that nothing authenticates: an upload has to be refused before it
//! costs anything, a file name has to be refused before it is joined to a
//! directory, and every request has to produce an answer so a gallery that is
//! waiting can stop. The store's own behaviour — scaling, fingerprinting,
//! ordering, path resolution — is covered by the unit tests in `image_store.rs`.
//!
//! Deliberately read-only against the operator's store: nothing here writes a
//! slot, so running the suite cannot disturb the pictures on the machine it is
//! run on.

use std::time::Duration;

use sdroxide_radio::{Complex32, EngineConfig, IqSource, Result, start_engine};
use sdroxide_types::{Command, DeviceCaps, ImageKind, RadioEvent};

struct MockSource {
    center: f64,
    rate: f64,
}

impl IqSource for MockSource {
    fn sample_rate(&self) -> f64 {
        self.rate
    }
    fn center_hz(&self) -> f64 {
        self.center
    }
    fn set_center_hz(&mut self, hz: f64) -> Result<()> {
        self.center = hz;
        Ok(())
    }
    fn read(&mut self, buf: &mut [Complex32]) -> Result<usize> {
        std::thread::sleep(Duration::from_millis(5));
        let n = buf.len().min(2048);
        buf[..n].fill(Complex32::new(0.0, 0.0));
        Ok(n)
    }
    fn describe(&self) -> String {
        "mock picture source".into()
    }
}

fn caps(rate: f64) -> DeviceCaps {
    DeviceCaps {
        driver: "mock".into(),
        label: "mock".into(),
        rx_channels: 1,
        sample_rates: vec![rate],
        freq_ranges_rx: vec![(0.0, 1_000_000_000.0)],
        ..DeviceCaps::default()
    }
}

/// Longest any of this is allowed to take. Generous because the whole file
/// runs six engines at once and the point of the deadline is to fail rather
/// than hang, not to time anything.
const LIMIT: Duration = Duration::from_secs(10);

/// Run `cmds` against a real engine and return everything it emitted, stopping
/// as soon as `done` is satisfied.
///
/// Waiting on events rather than on a fixed sleep: several of these run
/// concurrently, and a settle window long enough to be reliable on a loaded
/// machine would make the suite crawl on an idle one.
fn run(cmds: &[Command], done: impl Fn(&[RadioEvent]) -> bool) -> Vec<RadioEvent> {
    let rate = 2_400_000.0;
    let src = MockSource { center: 14_200_000.0, rate };
    let mut h = start_engine(Box::new(src), caps(rate), EngineConfig::default());
    let thread = h.thread.take();
    let mut events = Vec::new();

    // The startup announcements are the engine saying it is up. Sending before
    // them would race the command loop's first turn.
    let started = std::time::Instant::now();
    while started.elapsed() < LIMIT
        && !events.iter().any(|e| matches!(e, RadioEvent::ImagePresets(_)))
    {
        while let Ok(ev) = h.event_rx.try_recv() {
            events.push(ev);
        }
        std::thread::sleep(Duration::from_millis(5));
    }
    for c in cmds {
        h.cmd_tx.send(c.clone()).unwrap();
    }

    let deadline = std::time::Instant::now() + LIMIT;
    while std::time::Instant::now() < deadline && !done(&events) {
        while let Ok(ev) = h.event_rx.try_recv() {
            events.push(ev);
        }
        std::thread::sleep(Duration::from_millis(5));
    }

    drop(h.cmd_tx);
    if let Some(t) = thread {
        let _ = t.join();
    }
    while let Ok(ev) = h.event_rx.try_recv() {
        events.push(ev);
    }
    events
}

/// How many times the engine announced the presets.
fn announcements(events: &[RadioEvent]) -> usize {
    events.iter().filter(|e| matches!(e, RadioEvent::ImagePresets(_))).count()
}

/// Answers to `ImageGetSlot`, as `(slot, version, byte count)`.
fn slot_sources(events: &[RadioEvent]) -> Vec<(u8, u32, usize)> {
    events
        .iter()
        .filter_map(|e| match e {
            RadioEvent::ImageSlotSource { slot, version, png } => {
                Some((*slot, *version, png.len()))
            }
            _ => None,
        })
        .collect()
}

/// The presets are announced once, at startup, and the server caches that
/// announcement to replay on connect. A client that never sees it opens on five
/// empty slots with no second announcement to wait for.
#[test]
fn the_presets_are_announced_at_startup() {
    let events = run(&[], |_| true);
    let presets = events
        .iter()
        .find_map(|e| match e {
            RadioEvent::ImagePresets(p) => Some(p),
            _ => None,
        })
        .expect("the engine announces the transmit presets at startup");
    assert_eq!(presets.slots.len(), sdroxide_types::IMAGE_SLOTS);
}

/// The cap exists so an unauthenticated socket cannot make the engine allocate
/// on demand, which means it has to bite before the bytes reach a decoder — and
/// the operator has to be told, wherever they are sitting.
#[test]
fn an_oversized_upload_is_refused_with_a_notice() {
    let huge = vec![0u8; sdroxide_types::IMAGE_UPLOAD_MAX + 1];
    let events = run(&[Command::ImageSetSlot { slot: 0, bytes: huge }], |evs| {
        evs.iter().any(|e| matches!(e, RadioEvent::Notice(Some(_))))
    });

    let notice = events.iter().rev().find_map(|e| match e {
        RadioEvent::Notice(Some(n)) => Some(n.clone()),
        _ => None,
    });
    assert!(notice.is_some_and(|n| n.contains("refused")), "the operator must be told");
    // One announcement — the one at startup. The refusal changed nothing, so
    // there is nothing to announce.
    assert_eq!(announcements(&events), 1, "a refused upload must not change the presets");
}

/// A slot that does not exist is a bug in the client, not an operator
/// condition: it is dropped rather than announced, and nothing else stops.
#[test]
fn an_upload_to_a_slot_that_does_not_exist_changes_nothing() {
    // Nothing is emitted for the bad upload, so a harmless request follows it
    // as an ordering barrier: commands are applied in order, so its answer
    // arriving means the upload has already been dealt with.
    let events = run(
        &[Command::ImageSetSlot { slot: 200, bytes: vec![1, 2, 3] }, Command::ImageGetSlot(0)],
        |evs| !slot_sources(evs).is_empty(),
    );
    assert_eq!(announcements(&events), 1, "the presets must not change");
    assert!(!events.iter().any(|e| matches!(e, RadioEvent::Notice(Some(_)))));
}

/// Every request gets an answer, including the ones that fail. A gallery
/// waiting on a picture has to be able to stop waiting, so a name from outside
/// the store comes back empty rather than silently.
#[test]
fn a_name_from_outside_the_store_is_answered_empty() {
    let answered = |evs: &[RadioEvent]| {
        evs.iter().filter(|e| matches!(e, RadioEvent::ImageFile { .. })).count() >= 2
    };
    let events = run(
        &[
            Command::ImageGet { kind: ImageKind::Sstv, name: "../../etc/passwd".into() },
            Command::ImageGet { kind: ImageKind::Wefax, name: "a/b.png".into() },
        ],
        answered,
    );
    let files: Vec<_> = events
        .iter()
        .filter_map(|e| match e {
            RadioEvent::ImageFile { name, png, .. } => Some((name.clone(), png.len())),
            _ => None,
        })
        .collect();
    assert_eq!(files.len(), 2, "both requests must be answered: {files:?}");
    assert!(files.iter().all(|(_, len)| *len == 0), "nothing outside the store may be read");
}

/// A slot fetch is answered inline, and the version it reports is the one the
/// status carries — that pairing is what lets a client cache the pixels.
#[test]
fn a_slot_fetch_reports_the_version_the_status_carries() {
    let events = run(&[Command::ImageGetSlot(0), Command::ImageGetSlot(99)], |evs| {
        slot_sources(evs).len() >= 2
    });
    let presets = events
        .iter()
        .find_map(|e| match e {
            RadioEvent::ImagePresets(p) => Some(p.clone()),
            _ => None,
        })
        .expect("announced at startup");
    let sources = slot_sources(&events);
    assert_eq!(sources.len(), 2, "both fetches must be answered: {sources:?}");
    let (_, version, len) = sources[0];
    assert_eq!(version, presets.slot(0).version);
    assert_eq!(len > 0, presets.slot(0).has_picture(), "pixels iff the slot has a picture");
    // A slot that does not exist is empty rather than an error, for the same
    // reason: the client asked, so the client gets an answer.
    assert_eq!(sources[1], (99, 0, 0));
}

/// Listing a store always answers, even when there is nothing in it, and it
/// says which directory it looked in — a path on the *radio's* machine, which
/// is the only way a remote panel can name where its charts are going.
#[test]
fn listing_a_store_always_answers_and_names_its_directory() {
    let events = run(
        &[
            Command::ImageList { kind: ImageKind::Sstv, offset: 0, count: u32::MAX },
            Command::ImageList { kind: ImageKind::Wefax, offset: 0, count: 8 },
        ],
        |evs| evs.iter().filter(|e| matches!(e, RadioEvent::ImageListing(_))).count() >= 2,
    );
    let listings: Vec<_> = events
        .iter()
        .filter_map(|e| match e {
            RadioEvent::ImageListing(l) => Some(l.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(listings.len(), 2, "both listings must be answered");
    for l in &listings {
        assert!(!l.dir.is_empty(), "a gallery has to be able to say where the pictures are");
        assert!(l.entries.len() as u32 <= sdroxide_types::IMAGE_PAGE_MAX, "the page cap holds");
        assert!(l.entries.len() as u32 <= l.total, "a page is never bigger than the store");
    }
    // Matched by kind, never by position. Each `ImageList` is answered from its
    // own spawned thread, so two of them land in whatever order they finish —
    // asking for the wefax charts second does not make their listing arrive
    // second. This test read `listings[1]` as "the one asked for with count 8",
    // which is only true when the threads happen to agree, so it passed against
    // an empty store and failed against a real one: eleven received SSTV
    // pictures answered the *first* command, were the second listing to arrive,
    // and `11 <= 8` failed. The store's size is not something a test may
    // depend on, and the assertion is about the count we asked for.
    let wefax = listings.iter().find(|l| l.kind == ImageKind::Wefax).expect("the wefax listing");
    assert!(wefax.entries.len() <= 8, "an explicit count is honoured");
    let sstv = listings.iter().find(|l| l.kind == ImageKind::Sstv).expect("the sstv listing");
    // The two must be told apart, or "the wefax listing" above is the sstv one
    // and the whole test is checking the wrong store's arithmetic. This is the
    // assertion that would have caught the original fault on a store of any
    // size: the two listings are only interchangeable while both are short.
    assert_ne!(
        wefax.dir, sstv.dir,
        "the two listings name different stores, so matching by kind is what separates them"
    );
    // Deliberately NOT asserted here: that the sstv page honours `u32::MAX` as
    // "the page cap". That is `image_store::page`'s own arithmetic and it is
    // pinned there over a 200-picture store; asserting it here would only be
    // true while this operator's store is smaller than the cap, which is
    // exactly the mistake that made the original assertion meaningless.
}

/// A delete is answered whatever it names, and the answer is the gallery's cue
/// to drop a thumbnail — so a name that reaches nothing must still produce one,
/// or a picture the store has already lost sits on screen for ever.
///
/// Every name here is chosen so that a broken path check cannot destroy
/// anything: the traversal ones point at files that do not exist, and the plain
/// one is not a name either store has ever written.
#[test]
fn a_delete_is_always_answered_and_reaches_nothing_outside_the_store() {
    let names = [
        "../sdroxide-delete-guard-does-not-exist.png",
        "a/b-sdroxide-delete-guard.png",
        "zzz-sdroxide-delete-guard.png",
    ];
    let cmds: Vec<Command> = names
        .iter()
        .map(|n| Command::ImageDelete { kind: ImageKind::Sstv, name: (*n).into() })
        .collect();
    let events = run(&cmds, |evs| {
        evs.iter().filter(|e| matches!(e, RadioEvent::ImageDeleted { .. })).count() >= names.len()
    });

    let deleted: Vec<_> = events
        .iter()
        .filter_map(|e| match e {
            RadioEvent::ImageDeleted { name, .. } => Some(name.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(deleted.len(), names.len(), "every delete must be answered: {deleted:?}");
    for n in names {
        assert!(deleted.iter().any(|d| d == n), "{n} went unanswered");
    }
    // Nothing was really removed, so the presets never moved.
    assert_eq!(announcements(&events), 1);
}
