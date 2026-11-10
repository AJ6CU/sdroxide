//! End-to-end test over a real MIDI port.
//!
//! ALSA (and CoreMIDI) let a program create a *virtual* port, so the whole
//! path — enumeration, connect, the driver's callback thread, the queue, the
//! wake closure, hot-plug detection — can be exercised with no hardware and no
//! root. Everything below therefore runs against the real MIDI stack rather
//! than a stub.
//!
//! Skipped rather than failed where no MIDI stack exists: a headless CI
//! container has no `/dev/snd/seq`, and that is not a bug in this crate.

#![cfg(target_os = "linux")]

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use midir::os::unix::VirtualOutput;
use sdroxide_midi::{MidiConfig, MidiEvent};

/// Open a virtual source port other programs (including us) can connect to.
/// `None` when this machine has no ALSA sequencer.
///
/// Each test uses its own port name: they run in parallel, and two ports
/// sharing a name would have one test connect to the other's.
fn virtual_source(name: &str) -> Option<midir::MidiOutputConnection> {
    let out = midir::MidiOutput::new("sdroxide-test").ok()?;
    out.create_virtual(name).ok()
}

/// Wait for a freshly created virtual port to show up in enumeration.
fn find_port(name: &str) -> sdroxide_midi::PortInfo {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Some(p) = sdroxide_midi::input_ports().into_iter().find(|p| p.name.contains(name)) {
            return p;
        }
        assert!(Instant::now() < deadline, "virtual port {name} never appeared in input_ports()");
        std::thread::sleep(Duration::from_millis(50));
    }
}

/// Wait for the worker to produce an event satisfying `want`, or give up.
fn wait_for(
    h: &mut sdroxide_midi::MidiHandle,
    deadline: Duration,
    mut want: impl FnMut(&MidiEvent) -> bool,
) -> Option<MidiEvent> {
    let end = Instant::now() + deadline;
    while Instant::now() < end {
        for ev in h.poll() {
            if want(&ev) {
                return Some(ev);
            }
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    None
}

#[test]
fn a_real_port_round_trips_control_changes() {
    const PORT: &str = "sdroxide-test-roundtrip";
    let Some(mut src) = virtual_source(PORT) else {
        eprintln!("no ALSA sequencer; skipping");
        return;
    };
    let port = find_port(PORT);

    // The wake closure is what makes a knob feel immediate rather than waiting
    // out the UI's idle poll, so assert it actually fires.
    let wakes = Arc::new(AtomicUsize::new(0));
    let w = Arc::clone(&wakes);
    let cfg = MidiConfig {
        enabled: true,
        in_port_id: port.id.clone(),
        in_port_name: port.name.clone(),
        ..MidiConfig::default()
    };
    let mut h = sdroxide_midi::spawn(cfg, move || {
        w.fetch_add(1, Ordering::Relaxed);
    });

    assert!(
        wait_for(&mut h, Duration::from_secs(5), |e| matches!(e, MidiEvent::Connected(_)))
            .is_some(),
        "worker never connected to the virtual port"
    );
    assert!(h.status().connected);

    // CC 16 = 65 on channel 1 — a single detent from a 64-centred jog wheel.
    src.send(&[0xB0, 16, 65]).unwrap();
    let ev = wait_for(&mut h, Duration::from_secs(5), |e| matches!(e, MidiEvent::Control(_)))
        .expect("control change never arrived");
    let MidiEvent::Control(c) = ev else { unreachable!() };
    assert_eq!(c.msg.kind, sdroxide_types::MidiMsgKind::Cc);
    assert_eq!(c.msg.data1, 16);
    assert_eq!(c.value, 65);
    assert!(!c.off);
    assert_eq!(
        sdroxide_types::RelativeMode::BinaryOffset.decode(c.value),
        Some(1),
        "a 64-centred encoder's single clockwise detent"
    );
    assert!(wakes.load(Ordering::Relaxed) > 0, "the wake closure must fire on input");

    // A note-on then its release, the shape a PTT button takes.
    src.send(&[0x90, 60, 127]).unwrap();
    let on = wait_for(
        &mut h,
        Duration::from_secs(5),
        |e| matches!(e, MidiEvent::Control(c) if c.msg.kind == sdroxide_types::MidiMsgKind::Note),
    );
    assert!(matches!(on, Some(MidiEvent::Control(c)) if !c.off), "note-on should press");

    src.send(&[0x80, 60, 0]).unwrap();
    let off =
        wait_for(&mut h, Duration::from_secs(5), |e| matches!(e, MidiEvent::Control(c) if c.off));
    assert!(off.is_some(), "note-off must arrive, or PTT would never unkey");
}

/// Unplugging a controller mid-QSO must be noticed, because whatever it was
/// holding down can no longer be released by the controller itself.
#[test]
fn losing_the_port_reports_a_disconnect() {
    const PORT: &str = "sdroxide-test-unplug";
    let Some(src) = virtual_source(PORT) else {
        eprintln!("no ALSA sequencer; skipping");
        return;
    };
    let port = find_port(PORT);
    let cfg = MidiConfig {
        enabled: true,
        in_port_id: port.id.clone(),
        in_port_name: port.name.clone(),
        ..MidiConfig::default()
    };
    let mut h = sdroxide_midi::spawn(cfg, || {});
    assert!(
        wait_for(&mut h, Duration::from_secs(5), |e| matches!(e, MidiEvent::Connected(_)))
            .is_some(),
        "never connected"
    );

    // Closing the source is the software equivalent of pulling the cable.
    src.close();
    assert!(
        wait_for(&mut h, Duration::from_secs(10), |e| matches!(e, MidiEvent::Disconnected))
            .is_some(),
        "an unplugged controller must report a disconnect"
    );
    assert!(!h.status().connected);
}
