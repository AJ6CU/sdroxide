//! Queue policy: coalescing, the backlog budget, eviction, expiry, barge-in
//! and the transmit gag.
//!
//! No clock anywhere — `now` is a plain `f64`, so every case is exact.

use sdroxide_speech::queue::{Gag, Key, Priority, Push, QueueLimits, SpeechQueue, Utterance};

fn u(text: &str, p: Priority, now: f64) -> Utterance {
    Utterance::new(text, p, now)
}

#[test]
fn a_newer_frequency_replaces_an_older_unspoken_one() {
    let mut q = SpeechQueue::default();
    assert_eq!(
        q.push(u("fourteen point zero seven four", Priority::Routine, 0.0).key(Key::Freq)),
        Push::Queued
    );
    assert_eq!(
        q.push(u("fourteen point zero eight zero", Priority::Routine, 0.1).key(Key::Freq)),
        Push::Coalesced
    );
    assert_eq!(q.len(), 1);
    assert_eq!(q.pop(0.2, false).unwrap().text, "fourteen point zero eight zero");
}

#[test]
fn coalescing_keeps_the_queue_position() {
    let mut q = SpeechQueue::default();
    // Same priority throughout, so order is purely arrival order.
    q.push(u("first", Priority::Routine, 0.0).key(Key::Freq));
    q.push(u("second", Priority::Routine, 0.0).key(Key::Drive));
    q.push(u("third", Priority::Routine, 0.0).key(Key::MicGain));
    // The frequency changes again: it must not jump ahead of the two behind it.
    q.push(u("first again", Priority::Routine, 0.1).key(Key::Freq));

    assert_eq!(q.pop(0.2, false).unwrap().text, "first again");
    assert_eq!(q.pop(0.2, false).unwrap().text, "second");
    assert_eq!(q.pop(0.2, false).unwrap().text, "third");
}

#[test]
fn keyless_utterances_never_coalesce() {
    let mut q = SpeechQueue::default();
    // Two FT8 messages addressed to you are two facts, not two drafts.
    q.push(u("kilo one alpha bravo charlie calling you", Priority::Notable, 0.0));
    q.push(u("whiskey three x ray yankee calling you", Priority::Notable, 0.0));
    assert_eq!(q.len(), 2);
}

#[test]
fn a_burst_of_decodes_cannot_back_up_more_than_the_budget() {
    let mut q = SpeechQueue::default();
    for i in 0..200 {
        q.push(u(&format!("station number {i} is calling you right now"), Priority::Notable, 0.0));
    }
    assert!(q.len() <= 12, "item cap breached: {}", q.len());
    assert!(q.pending_speech_s() <= 20.0, "speech budget breached: {} s", q.pending_speech_s());
    // The survivors are the newest, not the oldest.
    let first = q.pop(0.0, false).unwrap();
    assert!(first.text.contains("19"), "kept the wrong end of the burst: {first:?}");
}

#[test]
fn eviction_takes_the_oldest_lowest_priority_item() {
    let mut q = SpeechQueue::new(QueueLimits { max_items: 3, ..Default::default() });
    q.push(u("old chatter", Priority::Chatter, 0.0));
    q.push(u("routine", Priority::Routine, 0.0));
    q.push(u("notable", Priority::Notable, 0.0));
    // Full. The next Notable must displace the Chatter, not the Routine.
    q.push(u("another notable", Priority::Notable, 0.1));

    let mut texts: Vec<String> = Vec::new();
    while let Some(x) = q.pop(0.2, false) {
        texts.push(x.text);
    }
    assert!(!texts.iter().any(|t| t == "old chatter"), "kept the chatter: {texts:?}");
    assert!(texts.iter().any(|t| t == "routine"), "dropped the routine: {texts:?}");
}

#[test]
fn an_incoming_item_below_everything_pending_is_refused() {
    let mut q = SpeechQueue::new(QueueLimits { max_items: 2, ..Default::default() });
    q.push(u("alert one", Priority::Alert, 0.0));
    q.push(u("alert two", Priority::Alert, 0.0));
    // Nothing here is worth evicting for a piece of read-along text.
    assert_eq!(q.push(u("chatter", Priority::Chatter, 0.1)), Push::Dropped);
    assert_eq!(q.len(), 2);
}

#[test]
fn expired_routine_items_are_never_spoken() {
    let mut q = SpeechQueue::default();
    // Routine gets a 4 s default TTL.
    q.push(u("drive thirty five percent", Priority::Routine, 0.0));
    assert!(q.pop(3.9, false).is_some());

    q.push(u("drive forty percent", Priority::Routine, 0.0));
    assert!(q.pop(4.1, false).is_none(), "spoke a stale level");
    assert_eq!(q.stats().dropped_expired, 1);
}

#[test]
fn notable_and_alert_do_not_expire_on_their_own() {
    let mut q = SpeechQueue::default();
    q.push(u("high S W R", Priority::Alert, 0.0));
    q.push(u("split on", Priority::Notable, 0.0));
    assert!(q.pop(600.0, false).is_some());
    assert!(q.pop(600.0, false).is_some());
}

#[test]
fn an_alert_interrupts_and_clears_below_notable() {
    let mut q = SpeechQueue::default();
    let speaking = u("fourteen point zero seven four", Priority::Routine, 0.0).key(Key::Freq);
    q.begin(&speaking);
    q.push(u("chatter", Priority::Chatter, 0.0));
    q.push(u("drive thirty percent", Priority::Routine, 0.0).key(Key::Drive));

    let r = q.push(u("outside the band", Priority::Alert, 0.1).interrupting());
    assert_eq!(r, Push::InterruptNow);

    q.clear_below(Priority::Notable);
    assert_eq!(q.len(), 1);
    assert_eq!(q.pop(0.2, false).unwrap().text, "outside the band");
}

#[test]
fn a_same_key_interrupt_replaces_what_is_speaking() {
    let mut q = SpeechQueue::default();
    // The operator has already spun past the frequency being read out.
    let speaking = u("fourteen point zero seven four", Priority::Routine, 0.0).key(Key::Freq);
    q.begin(&speaking);
    let r = q.push(
        u("fourteen point one zero zero", Priority::Routine, 0.5).key(Key::Freq).interrupting(),
    );
    assert_eq!(r, Push::InterruptNow);
}

#[test]
fn an_ordinary_change_does_not_interrupt() {
    let mut q = SpeechQueue::default();
    let speaking = u("twenty meters", Priority::Routine, 0.0).key(Key::Band);
    q.begin(&speaking);
    // Interrupting mid-word should be rare enough that it means something.
    assert_eq!(q.push(u("A G C slow", Priority::Routine, 0.1).key(Key::Agc)), Push::Queued);
}

#[test]
fn the_ptt_gag_refuses_at_push_time_not_merely_defers() {
    let mut q = SpeechQueue::default();
    q.set_gag(Gag::Transmitting);
    assert_eq!(q.push(u("drive thirty percent", Priority::Routine, 0.0)), Push::Dropped);
    assert_eq!(q.push(u("kilo one alpha calling you", Priority::Notable, 0.0)), Push::Dropped);
    assert_eq!(q.len(), 0);

    // Unkeying must not unload a backlog of stale announcements.
    q.set_gag(Gag::None);
    assert_eq!(q.len(), 0);
    assert!(q.pop(1.0, false).is_none());
}

#[test]
fn the_ptt_gag_lets_an_alert_through() {
    let mut q = SpeechQueue::default();
    q.set_gag(Gag::Transmitting);
    // A runaway SWR mid-transmission is exactly why the tier exists.
    assert_eq!(q.push(u("high S W R, four point two to one", Priority::Alert, 0.0)), Push::Queued);
    assert_eq!(q.pop(0.1, false).unwrap().text, "high S W R, four point two to one");
}

#[test]
fn switching_speech_off_refuses_everything() {
    let mut q = SpeechQueue::default();
    q.push(u("split on", Priority::Notable, 0.0));
    q.set_gag(Gag::Off);
    assert_eq!(q.push(u("high S W R", Priority::Alert, 0.0)), Push::Dropped);
    assert_eq!(q.len(), 0);
}

#[test]
fn nothing_is_returned_while_the_sink_is_busy() {
    let mut q = SpeechQueue::default();
    q.push(u("split on", Priority::Notable, 0.0));
    assert!(q.pop(0.1, true).is_none());
    assert!(q.pop(0.1, false).is_some());
}

#[test]
fn higher_priority_is_spoken_first() {
    let mut q = SpeechQueue::default();
    q.push(u("chatter", Priority::Chatter, 0.0));
    q.push(u("routine", Priority::Routine, 0.0));
    q.push(u("alert", Priority::Alert, 0.0));
    q.push(u("notable", Priority::Notable, 0.0));
    let order: Vec<String> = std::iter::from_fn(|| q.pop(0.1, false)).map(|x| x.text).collect();
    assert_eq!(order, ["alert", "notable", "routine", "chatter"]);
}

#[test]
fn an_utterance_longer_than_the_whole_budget_still_gets_said() {
    let mut q = SpeechQueue::new(QueueLimits { max_pending_s: 2.0, ..Default::default() });
    let long = "a".repeat(500);
    // Refusing it would mean never saying anything long, which is worse than
    // briefly exceeding a budget that exists to bound a backlog.
    assert_eq!(q.push(u(&long, Priority::Notable, 0.0)), Push::Queued);
    assert!(q.pop(0.1, false).is_some());
}

#[test]
fn empty_text_is_never_queued() {
    let mut q = SpeechQueue::default();
    assert_eq!(q.push(u("", Priority::Alert, 0.0)), Push::Dropped);
    assert_eq!(q.push(u("   ", Priority::Alert, 0.0)), Push::Dropped);
    assert!(q.is_empty());
}

#[test]
fn the_deadline_is_the_earliest_expiry() {
    let mut q = SpeechQueue::default();
    q.push(u("later", Priority::Chatter, 0.0)); // +8 s
    q.push(u("sooner", Priority::Routine, 0.0)); // +4 s
    assert_eq!(q.next_deadline(), Some(4.0));
    q.push(u("never", Priority::Alert, 0.0));
    assert_eq!(q.next_deadline(), Some(4.0));
}
