//! The operator's transmit band list, on screen.
//!
//! The list itself is `RadioConfig::tx_band_enable`, and the engine refuses to
//! key on a band that is not in it (see `Engine::tx_band_refusal`). This is the
//! half the operator sees: the notice that asks them to check and confirm the
//! list once per radio, the PTT and TUNE that stay dark where TX is off, and the
//! notice shown when the dial is on a band that is off. The dial is never held.
//!
//! None of it applies on a listening screen (SWL): there is nothing to key.

use sdroxide_types::Band;

use super::SdroxideApp;
use crate::i18n::tr;

/// Where this radio stands with its transmit band list.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(in crate::app) enum TxBands {
    /// A listening screen, a receive-only radio, or no configuration yet.
    NotApplicable,
    /// Not confirmed. `carried_over` is an upgrade whose bands were filled in
    /// from the previous version; otherwise a new radio with every band off.
    Unconfirmed {
        carried_over: bool,
    },
    Confirmed,
}

/// How many times per session and radio tab the refused-band notice shows.
pub(in crate::app) const TX_BAND_NOTICES_MAX: u8 = 3;

/// A band's name as the operator reads it. The CB band is "11M-CB" here only:
/// `Band::label()` stays "11M" for logs, contests and the config.
pub(in crate::app) fn tx_band_label(b: Band) -> String {
    match b {
        Band::M11 => "11M-CB".to_string(),
        b => b.label().to_string(),
    }
}

/// The name of the band `hz` is in, as the operator sees it.
fn band_name(hz: f64) -> String {
    match Band::containing(hz) {
        Band::Gen => tr!("settings-other"),
        b => tx_band_label(b),
    }
}

/// The red refusal text for `why` at `hz`, in the operator's language.
fn tx_block_text(why: Option<sdroxide_types::TxBlock>, hz: f64) -> String {
    match why {
        Some(sdroxide_types::TxBlock::BandOff(_)) => {
            tr!("tx-off-band", freq = format!("{:.3}", hz / 1e6), band = band_name(hz))
        }
        Some(sdroxide_types::TxBlock::CbOff) => tr!("tx-off-cb"),
        _ => tr!("tx-off-unconfirmed"),
    }
}

/// What the BAND TX pill shows.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(in crate::app) enum BandTxPill {
    /// The dial is on a band the operator enabled and confirmed: receiving.
    Allowed,
    /// The dial is on a band that is off: transmit is cut.
    Refused,
    /// The radio is transmitting (PTT or TUNE) on an allowed band.
    Transmitting,
}

impl SdroxideApp {
    /// The pill, or `None` where it is hidden: a listening screen (SWL, listen
    /// mode), a receive-only radio or mode, a playback, a list nobody confirmed,
    /// or no band ticked at all.
    pub(in crate::app) fn band_tx_pill(&self) -> Option<BandTxPill> {
        let cfg = self.radio_cfg.as_ref()?;
        let any_band = cfg.tx_band_enable.as_ref().is_some_and(|b| !b.is_empty());
        if !self.tx_chrome_ok()
            || self.state.playback.is_some()
            || !cfg.tx_bands_confirmed
            || !any_band
        {
            return None;
        }
        Some(if !cfg.tx_dial_allowed(self.state.tx_freq_hz()) {
            BandTxPill::Refused
        } else if self.state.tx.ptt || self.state.tx.tune {
            BandTxPill::Transmitting
        } else {
            BandTxPill::Allowed
        })
    }

    pub(in crate::app) fn tx_bands_status(&self) -> TxBands {
        if !self.tx_capable() {
            return TxBands::NotApplicable;
        }
        let Some(cfg) = self.radio_cfg.as_ref() else { return TxBands::NotApplicable };
        if cfg.tx_bands_confirmed {
            TxBands::Confirmed
        } else {
            TxBands::Unconfirmed {
                carried_over: cfg.tx_band_enable.as_ref().is_some_and(|b| !b.is_empty()),
            }
        }
    }

    /// Whether PTT and TUNE may be offered at the dial this radio would
    /// transmit on: the dial is in a band that is ticked and validated.
    ///
    /// `true` where nothing is offered anyway (a listening screen, a radio that
    /// is known not to transmit). **Fail-safe**: while the radio's capabilities
    /// are not known yet, a screen that is not a listening one is NOT keyable
    /// until the band list says so.
    pub(in crate::app) fn tx_keyable_here(&self) -> bool {
        if self.swl_mode() || self.caps.as_ref().is_some_and(|c| !c.is_transmit_capable()) {
            return true;
        }
        self.tx_block().is_none()
    }

    /// Why the dial may not be keyed, from the one rule (`tx_block_reason`).
    /// No configuration yet counts as blocked.
    fn tx_block(&self) -> Option<sdroxide_types::TxBlock> {
        match self.radio_cfg.as_ref() {
            Some(c) => c.tx_block_reason(self.state.tx_freq_hz()),
            None => Some(sdroxide_types::TxBlock::Unconfirmed),
        }
    }

    /// The big red message shown on hover and on every press attempt of a PTT
    /// or TUNE that is off, or `None` when keying is allowed here. Never capped:
    /// the three-times limit is the yellow banner's alone.
    pub(in crate::app) fn tx_off_text(&self) -> Option<String> {
        if self.tx_keyable_here() {
            return None;
        }
        Some(tx_block_text(self.tx_block(), self.state.tx_freq_hz()))
    }

    /// Put over a PTT or TUNE that is greyed out: the big red message on hover,
    /// and every press attempt (mouse or touch) raises the red banner.
    pub(in crate::app) fn tx_off_overlay(
        &mut self,
        ui: &mut eframe::egui::Ui,
        chip: &eframe::egui::Response,
    ) {
        if chip.enabled() {
            return;
        }
        let Some(text) = self.tx_off_text() else { return };
        let r = ui.interact(chip.rect, chip.id.with("tx-off"), eframe::egui::Sense::click());
        r.clone().on_hover_ui(|ui| {
            ui.label(
                eframe::egui::RichText::new(text).size(20.0).strong().color(crate::theme::ALERT()),
            );
        });
        if r.hovered() && ui.input(|i| i.pointer.primary_pressed()) {
            self.tx_engine_refusal = None;
            self.tx_press_refused_at = Some(crate::time::now_unix_f64());
        }
    }

    /// The engine refused a key-down: raise the red banner with its reason, at
    /// the frequency it names.
    pub(in crate::app) fn on_tx_refused(&mut self, why: sdroxide_types::TxBlock, hz: f64) {
        self.tx_engine_refusal = Some((why, hz));
        self.tx_press_refused_at = Some(crate::time::now_unix_f64());
    }

    /// The red banner's text while a refused press is recent (5 s, renewed by
    /// every new attempt), or `None`.
    pub(in crate::app) fn tx_press_banner(&self, now: f64) -> Option<String> {
        let at = self.tx_press_refused_at?;
        if !(0.0..5.0).contains(&(now - at)) {
            return None;
        }
        match self.tx_engine_refusal {
            Some((why, hz)) => Some(tx_block_text(Some(why), hz)),
            None => self.tx_off_text(),
        }
    }

    /// The notice for an unconfirmed list, or `None` when there is nothing to say.
    ///
    /// One banner for both lists: the transmit text, then the receive one when
    /// the receive list is not confirmed either.
    pub(in crate::app) fn tx_bands_notice(&self) -> Option<String> {
        let tx = match self.tx_bands_status() {
            TxBands::Unconfirmed { carried_over: false } => Some(tr!("tx-bands-notice-new")),
            TxBands::Unconfirmed { carried_over: true } => Some(tr!("tx-bands-notice-carried")),
            _ => None,
        };
        let rx = self.rx_bands_unconfirmed().then(|| tr!("rx-bands-notice"));
        match (tx, rx) {
            (Some(t), Some(r)) => Some(format!("{t} {r}")),
            (t, r) => t.or(r),
        }
    }

    /// Whether this radio's receive band list still waits for the operator.
    /// Every radio has one, a receive-only one and a listening screen too.
    pub(in crate::app) fn rx_bands_unconfirmed(&self) -> bool {
        self.radio_cfg.as_ref().is_some_and(|c| !c.rx_bands_confirmed)
    }

    /// Warn, without ever touching the dial, when the frequency is outside the
    /// bands the operator enabled and confirmed. Transmit is already off there
    /// (the engine refuses to key); this is only the notice.
    ///
    /// The dial is free: nothing is put back and the screen is not changed.
    /// Each move onto a refused band counts as one showing, at most
    /// [`TX_BAND_NOTICES_MAX`] per session and per radio tab — the count is
    /// not saved, so it starts again at the next launch.
    pub(in crate::app) fn tx_band_guard(&mut self) {
        if self.tx_bands_status() != TxBands::Confirmed || self.state.playback.is_some() {
            self.tx_band_msg = None;
            self.tx_band_refused_in = None;
            return;
        }
        let Some(cfg) = self.radio_cfg.as_ref() else { return };
        let hz = self.state.tx_freq_hz();
        if cfg.tx_dial_allowed(hz) {
            self.tx_band_msg = None;
            self.tx_band_refused_in = None;
            return;
        }
        let band = Band::containing(hz);
        if self.tx_band_refused_in == Some(band) {
            return;
        }
        self.tx_band_refused_in = Some(band);
        if self.tx_band_shown < TX_BAND_NOTICES_MAX {
            self.tx_band_shown += 1;
            self.tx_band_msg = Some(match cfg.tx_block_reason(hz) {
                Some(sdroxide_types::TxBlock::CbOff) => tr!("tx-off-cb"),
                _ => tr!("tx-bands-off", band = band_name(hz), max = TX_BAND_NOTICES_MAX),
            });
        } else {
            self.tx_band_msg = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use sdroxide_types::{Band, RadioConfig};

    fn confirmed(bands: &[Band]) -> RadioConfig {
        RadioConfig {
            tx_band_enable: Some(bands.to_vec()),
            tx_bands_confirmed: true,
            ..RadioConfig::default()
        }
    }

    #[test]
    fn a_band_is_allowed_only_when_listed_and_confirmed() {
        let cfg = confirmed(&[Band::M40]);
        assert!(cfg.tx_dial_allowed(7_074_000.0));
        assert!(!cfg.tx_dial_allowed(3_573_000.0), "80 m is not listed");
        let unconfirmed = RadioConfig { tx_bands_confirmed: false, ..cfg };
        assert!(
            !unconfirmed.tx_dial_allowed(7_074_000.0),
            "a list nobody confirmed allows nothing"
        );
        assert!(!RadioConfig::default().tx_dial_allowed(7_074_000.0), "no list, no band");
    }

    /// ALL is a selector, not a permission: the check is on the real frequency,
    /// and outside every band nothing is ever keyed, whatever the list holds.
    #[test]
    fn a_frequency_outside_every_band_is_never_keyed() {
        assert_eq!(Band::containing(35_000_000.0), Band::Gen);
        let cfg = confirmed(&[Band::M40, Band::M20]);
        assert!(!cfg.tx_dial_allowed(35_000_000.0));
        let cfg = confirmed(&[Band::Gen]);
        assert!(!cfg.tx_dial_allowed(35_000_000.0), "an old list naming Other grants nothing");
        assert!(!cfg.tx_dial_allowed(7_074_000.0));
    }

    #[test]
    fn an_upgrade_is_filled_in_unconfirmed_and_only_once() {
        let mut cfg = RadioConfig::default();
        assert!(cfg.tx_band_enable.is_none());
        assert!(cfg.migrate_tx_bands(|hz| hz < 30e6));
        let bands = cfg.tx_band_enable.clone().unwrap();
        assert!(bands.contains(&Band::M40) && bands.contains(&Band::M10));
        assert!(!bands.contains(&Band::M2), "out of the radio's range");
        assert!(!bands.contains(&Band::M11), "CB is not an amateur band");
        assert!(!bands.contains(&Band::Gen));
        assert!(!cfg.tx_bands_confirmed, "carried over, never confirmed");
        assert!(!cfg.migrate_tx_bands(|_| true), "an existing list is left alone");
    }
}
