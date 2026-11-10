//! Send on return ends the over with the line.
//!
//! The keyboard modes hold the carrier open by design — idle reversals between
//! sentences are how the other station hears that you are still there, and that
//! is what makes type-ahead work. Under `send_on_enter` the operator composes
//! *off* the air instead, so the same hold would put idle reversals out for as
//! long as they take to type the next line. The over has to end with the text.

use sdroxide_digi::DigiEngine;
use sdroxide_digi::text_modem::TextModemController;
use sdroxide_types::{DigiConfig, Mode};

const RATE: f64 = 8000.0;
const BLOCK: usize = 480;

/// Pump transmit audio until the modem says the over is finished, or give up.
/// Returns whether it finished.
fn drain(ctl: &mut TextModemController, blocks: usize) -> bool {
    let mut audio = [0.0f32; BLOCK];
    for _ in 0..blocks {
        if ctl.fill_tx_block(&mut audio) {
            return true;
        }
    }
    false
}

#[test]
fn a_committed_line_releases_transmit_when_it_has_gone_out() {
    for mode in [Mode::Psk, Mode::Rtty, Mode::Olivia, Mode::Thor] {
        let cfg = DigiConfig { send_on_enter: true, ..Default::default() };
        let mut ctl = TextModemController::new(mode, cfg, RATE);
        ctl.set_tx_text("TNX FER CALL\n".into());
        ctl.set_tx_active(true);
        assert!(drain(&mut ctl, 200_000), "{mode:?}: the carrier was held after the line");
        assert!(!ctl.status().tx_next, "{mode:?}: transmit was left switched on");
    }
}

/// The switch is still a switch: pressed over an empty box it holds, because an
/// over that never started is not an over that finished. Taking transmit back
/// on the press itself would read as a broken button.
#[test]
fn holding_transmit_over_an_empty_box_still_holds() {
    for mode in [Mode::Psk, Mode::Rtty, Mode::Olivia, Mode::Thor] {
        let cfg = DigiConfig { send_on_enter: true, ..Default::default() };
        let mut ctl = TextModemController::new(mode, cfg, RATE);
        ctl.set_tx_active(true);
        assert!(!drain(&mut ctl, 500), "{mode:?}: an empty box ended the over on its own");
        assert!(ctl.status().tx_next, "{mode:?}: transmit released itself");
    }
}

/// And with it off, nothing changes: the carrier stays up between sentences,
/// which is the whole point of type-ahead.
#[test]
fn type_ahead_still_holds_the_carrier_between_sentences() {
    let cfg = DigiConfig::default();
    assert!(!cfg.send_on_enter, "the default must stay type-ahead");
    let mut ctl = TextModemController::new(Mode::Psk, cfg, RATE);
    ctl.set_tx_text("TNX FER CALL".into());
    ctl.set_tx_active(true);
    assert!(!drain(&mut ctl, 200_000), "the carrier dropped when the text ran out");
    assert!(ctl.status().tx_next);
}
